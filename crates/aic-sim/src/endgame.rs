//! 154: the end-game probe, opt in (`"endgame"` in a spec). For a game lost to
//! the round cap (155: any loss, with `all_losses`) it records what the purge
//! still needed at the end, what the team held toward it a few rounds before
//! (the inputs of a feasibility bound,
//! read by `experiments/results/154_endgame_r66/endgame154.py`), and, with an
//! agent named, plays the last rounds again from that round's start with that
//! agent (the real game continued: same state, same dice) to see whether a
//! stronger player wins it. It never touches the game being played.

use aic_core::content::{Op, Resource, Side};
use aic_core::modules::{adversary, alignment, cards, common};
use aic_core::op::{Piece, Removable};
use aic_core::{GameState, Rules};
use serde::{Deserialize, Serialize};

use crate::agents::AgentSpec;

/// The spec's switch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct EndgameProbe {
    /// Rounds before the end the snapshot is taken at: 3 is the start of the
    /// third-last round (round 9 of 11).
    pub back: u32,
    /// The replay agent; empty: no replay, the end state and bound only.
    pub agent: String,
    /// Replays per game (agent seeds differ; the game's own dice are the
    /// state's, so the adversary plays the same cards).
    pub rollouts: usize,
    /// 155: every loss, not only the round cap's (`reason` says which).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub all_losses: bool,
}

impl Default for EndgameProbe {
    fn default() -> Self {
        EndgameProbe { back: 3, agent: String::new(), rollouts: 1, all_losses: false }
    }
}

/// The adversary pieces on the board, by kind, and what clearing them costs in
/// ambassadors at the printed rates.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Pieces {
    pub cities: i64,
    pub botnets: i64,
    pub troops: i64,
    /// Adversary ambassadors anywhere, and of them on corrupted cities.
    pub ambassadors: i64,
    pub ambassadors_on_corrupted: i64,
    /// Corrupted cities in some seat's reach (a liberation needs reach).
    pub cities_in_reach: i64,
    /// Ambassadors the Liberate action takes for every corrupted city
    /// (`liberate_at` plus those on it).
    pub liberate_cost: i64,
    /// Remove actions (one ambassador each) to clear the adversary
    /// ambassadors off every hex that is not a corrupted city, `remove_count`
    /// a use from any hexes (r75). (178: no agent lands on a liberated city, so `remove_after_liberate` and
    /// `free_leaves` are gone.)
    pub remove_actions: i64,
    /// Corruption markers (Clear sways 2 Unity for 2 credits each), and of them in some seat's reach.
    pub markers: i64,
    pub markers_in_reach: i64,
    /// The purge's count at the state's own level.
    pub purge: i64,
}

/// What the team held toward the purge at the snapshot.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub ambassadors_supply: i64,
    pub ambassadors_board: i64,
    pub credits: i64,
    /// Cards in hand and in play whose text removes a botnet, liberates a
    /// city, removes troops, removes adversary ambassadors (by op).
    pub botnet_cards: i64,
    pub liberate_cards: i64,
    pub troop_cards: i64,
    pub ambassador_cards: i64,
    /// Unity cards still in the deck whose text (any tier) removes a botnet, or
    /// adversary ambassadors (by op, as the player cards; 165: was 0/1 on two
    /// pre-156 card ids).
    pub unity_botnet_left: i64,
    pub unity_ambassador_left: i64,
    pub unity_drawn: i64,
    pub unity_track: i64,
    pub unity_capacity: i64,
}

/// One round-cap loss (or, `all_losses`, any loss), probed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EndgameRead {
    /// The end reason (155; empty in older streams: the round cap).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    pub end_round: u32,
    pub level: i64,
    pub unity_drawn: i64,
    pub discord_left: i64,
    pub end: Pieces,
    pub snap_round: u32,
    pub snap_level: i64,
    pub snap: Pieces,
    pub team: Team,
    /// The replays: how many, how many won, each one's end reason and purge count.
    pub replays: usize,
    pub replay_wins: usize,
    pub replay_reasons: Vec<String>,
    pub replay_purge: Vec<i64>,
}

pub fn pieces(rules: &Rules, g: &GameState) -> Pieces {
    let w = rules.world();
    let level = alignment::bar_level(&w, g);
    let lib_at = w.int("board.liberate_at");
    let per_use = w.int("board.remove_count").max(1);
    let mut p = Pieces::default();
    let mut loose = 0;
    for h in 0..g.hexes.len() {
        let amb = g.tokens(h, common::ADV_AMBASSADOR);
        p.botnets += g.tokens(h, common::BOTNET);
        p.troops += g.tokens(h, common::TROOP);
        p.ambassadors += amb;
        let m = g.tokens(h, common::MARKER);
        p.markers += m;
        if m > 0 && (0..g.seats.len()).any(|s| common::in_reach(&w, g, s, h)) {
            p.markers_in_reach += m;
        }
        if common::corrupted_city(&w, g, h) {
            p.cities += 1;
            p.ambassadors_on_corrupted += amb;
            p.liberate_cost += lib_at + amb;
            if (0..g.seats.len()).any(|s| common::in_reach(&w, g, s, h)) {
                p.cities_in_reach += 1;
            }
        } else if amb > 0 {
            loose += amb;
        }
    }
    // r75: one Remove takes up to `remove_count` agents from any hexes in reach.
    p.remove_actions = (loose + per_use - 1) / per_use;
    p.purge = adversary::purge_count(&w, g, level);
    p
}

/// An op that removes a botnet.
fn removes_botnet(o: &Op) -> bool {
    // 169: r71's Unity cards remove by board condition (`remove_where`); without it the probe counted none.
    matches!(o, Op::RemoveBotnet | Op::RemoveBotnets { .. } | Op::CutBotnet | Op::RemoveWhere { pieces: Removable::Botnets, .. })
}

/// An op that removes adversary ambassadors (agents).
fn removes_ambassadors(o: &Op) -> bool {
    matches!(o, Op::RemoveAmbassadorsCountry { .. } | Op::Pushback { .. } | Op::RemoveWhere { pieces: Removable::Agents, .. })
        || matches!(o, Op::Gather { strike: true, pieces, .. } if pieces.contains(&Piece::Agents))
}

/// An op that removes adversary troops (169: a strike gather too).
fn removes_troops(o: &Op) -> bool {
    matches!(o, Op::RemoveTroops { .. }) || matches!(o, Op::Gather { strike: true, pieces, .. } if pieces.contains(&Piece::Troops))
}

/// `ops` and every op nested under them.
fn with_nested<'a>(ops: impl IntoIterator<Item = &'a Op>, out: &mut Vec<&'a Op>) {
    for op in ops {
        out.push(op);
        with_nested(op.nested(), out);
    }
}

fn team(rules: &Rules, g: &GameState) -> Team {
    let w = rules.world();
    let mut t = Team::default();
    for s in 0..g.seats.len() {
        t.ambassadors_supply += g.resource(s, Resource::Ambassadors);
        t.ambassadors_board += (0..g.hexes.len()).map(|h| g.tokens(h, common::ambassador(s))).sum::<i64>();
        t.credits += g.resource(s, Resource::Credits);
        let mut ids: Vec<String> = g.pile(&cards::hand(s)).iter().map(|k| k.as_str().to_string()).collect();
        ids.extend(cards::in_play(g, s).iter().map(|k| k.as_str().to_string()));
        for id in ids {
            let ops = w.content.card(&id).all_ops();
            let has = |f: fn(&Op) -> bool| ops.iter().any(|o| f(o)) as i64;
            t.botnet_cards += has(removes_botnet);
            t.liberate_cards += has(|o| matches!(o, Op::Liberate { .. } | Op::ClearCity { .. } | Op::Cleanse { .. }));
            t.troop_cards += has(removes_troops);
            t.ambassador_cards += has(removes_ambassadors);
        }
    }
    for id in g.pile(&alignment::deck(Side::Unity)) {
        let mut ops = Vec::new();
        with_nested(w.content.alignment_card(id.as_str()).all_effects(), &mut ops);
        t.unity_botnet_left += ops.iter().any(|o| removes_botnet(o)) as i64;
        t.unity_ambassador_left += ops.iter().any(|o| removes_ambassadors(o)) as i64;
    }
    t.unity_drawn = alignment::unity_drawn(g);
    t.unity_track = g.var(alignment::track(Side::Unity));
    t.unity_capacity = alignment::capacity(&w, g, Side::Unity);
    t
}

/// Probes a finished game; `starts` holds the state at the start of each
/// round played (round 1 first). None unless it was lost to the round cap
/// (`deck_empty`), or lost at all with `all_losses`.
pub fn probe(rules: &Rules, probe: &EndgameProbe, g: &GameState, starts: &[GameState], max_actions: usize) -> Option<EndgameRead> {
    if g.outcome.as_ref().is_none_or(|o| o.won || (o.reason != "deck_empty" && !probe.all_losses)) {
        return None;
    }
    let w = rules.world();
    let snap_round = g.round.saturating_sub(probe.back.saturating_sub(1)).max(1);
    let snap = starts.iter().rev().find(|s| s.round <= snap_round)?;
    let mut r = EndgameRead {
        reason: g.outcome.as_ref().map_or(String::new(), |o| o.reason.clone()),
        end_round: g.round,
        level: alignment::bar_level(&w, g).steps(),
        unity_drawn: alignment::unity_drawn(g),
        discord_left: alignment::cards_left(g, Side::Discord) as i64,
        end: pieces(rules, g),
        snap_round: snap.round,
        snap_level: alignment::bar_level(&w, snap).steps(),
        snap: pieces(rules, snap),
        team: team(rules, snap),
        ..Default::default()
    };
    if probe.agent.is_empty() {
        return Some(r);
    }
    let spec = AgentSpec::parse(&probe.agent).expect("checked in cells()");
    let mut legal = Vec::new();
    for k in 0..probe.rollouts {
        let mut s = snap.clone();
        let mut a = spec.build(rules, g.seed.wrapping_add(0x0e9d_9a3e).wrapping_add(k as u64 * 0x9e37_79b9));
        let mut n = 0;
        while n < max_actions {
            rules.legal_actions_into(&s, &mut legal);
            if legal.is_empty() {
                break;
            }
            let i = a.choose(rules, &s, &legal);
            if rules.apply_mut(&mut s, &legal[i], None).is_err() {
                break;
            }
            n += 1;
        }
        r.replays += 1;
        let won = s.outcome.as_ref().is_some_and(|o| o.won);
        r.replay_wins += won as usize;
        r.replay_reasons.push(s.outcome.as_ref().map_or("timeout".into(), |o| o.reason.clone()));
        let lvl = alignment::bar_level(&w, &s);
        r.replay_purge.push(if won { 0 } else { adversary::purge_count(&w, &s, lvl) });
    }
    Some(r)
}

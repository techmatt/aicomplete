//! 154: the end-game probe, opt in (`"endgame"` in a spec). For a game lost to
//! the round cap it records what the purge still needed at the end, what the
//! team held toward it a few rounds before (the inputs of a feasibility bound,
//! read by `experiments/results/154_endgame_r66/endgame154.py`), and, with an
//! agent named, plays the last rounds again from that round's start with that
//! agent (the real game continued: same state, same dice) to see whether a
//! stronger player wins it. It never touches the game being played.

use aic_core::content::{Resource, Side};
use aic_core::modules::{adversary, alignment, cards, common};
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
}

impl Default for EndgameProbe {
    fn default() -> Self {
        EndgameProbe { back: 3, agent: String::new(), rollouts: 1 }
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
    /// a hex a use; and those that land back on the liberated cities
    /// (`free_leaves` each).
    pub remove_actions: i64,
    pub remove_after_liberate: i64,
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
    /// Unity cards still in the deck that remove pieces (Decommission: a
    /// botnet; Civil Defense: an adversary ambassador a country).
    pub unity_botnet_left: i64,
    pub unity_ambassador_left: i64,
    pub unity_drawn: i64,
    pub unity_track: i64,
    pub unity_capacity: i64,
}

/// One round-cap loss, probed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EndgameRead {
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

fn has_op(json: &str, ops: &[&str]) -> bool {
    ops.iter().any(|o| json.contains(&format!("\"op\":\"{o}\"")))
}

pub fn pieces(rules: &Rules, g: &GameState) -> Pieces {
    let w = rules.world();
    let level = alignment::bar_level(&w, g);
    let lib_at = w.int("board.liberate_at");
    let per_use = w.int("board.remove_count").max(1);
    let leaves = w.int("board.free_leaves");
    let mut p = Pieces::default();
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
            p.remove_after_liberate += (leaves + per_use - 1) / per_use;
            if (0..g.seats.len()).any(|s| common::in_reach(&w, g, s, h)) {
                p.cities_in_reach += 1;
            }
        } else if amb > 0 {
            p.remove_actions += (amb + per_use - 1) / per_use;
        }
    }
    p.purge = adversary::purge_count(&w, g, level);
    p
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
            let json = serde_json::to_string(w.content.card(&id)).unwrap_or_default();
            t.botnet_cards += has_op(&json, &["remove_botnet", "cut_botnet"]) as i64;
            t.liberate_cards += has_op(&json, &["liberate", "clear_city", "cleanse"]) as i64;
            t.troop_cards += has_op(&json, &["remove_troop", "remove_troops"]) as i64;
            t.ambassador_cards += has_op(&json, &["remove_ambassadors", "remove_ambassadors_country"]) as i64;
        }
    }
    let drawn: Vec<String> = g.pile(&alignment::resolved(Side::Unity)).iter().map(|k| k.as_str().to_string()).collect();
    t.unity_botnet_left = (!drawn.iter().any(|c| c == "decommission")) as i64;
    t.unity_ambassador_left = (!drawn.iter().any(|c| c == "civil_defense")) as i64;
    t.unity_drawn = alignment::unity_drawn(g);
    t.unity_track = g.var(alignment::track(Side::Unity));
    t.unity_capacity = alignment::capacity(&w, g, Side::Unity);
    t
}

/// Probes a finished game; `starts` holds the state at the start of each
/// round played (round 1 first). None unless it was lost to the round cap.
pub fn probe(rules: &Rules, probe: &EndgameProbe, g: &GameState, starts: &[GameState], max_actions: usize) -> Option<EndgameRead> {
    if g.outcome.as_ref().is_none_or(|o| o.won || o.reason != "deck_empty") {
        return None;
    }
    let w = rules.world();
    let snap_round = g.round.saturating_sub(probe.back.saturating_sub(1)).max(1);
    let snap = starts.iter().rev().find(|s| s.round <= snap_round)?;
    let mut r = EndgameRead {
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

//! Alignment and the end of the game: the two Alignment tracks (Unity and
//! Discord, each fed with its own Sway and tipping at its own threshold),
//! the Unity and Discord decks, and the win and loss
//! conditions, checked after every action and every automatic phase.
//!
//! Replaces human opposition (rules revision 5). Since 017 every card is
//! one-shot; 017–018's Discord-tip troop advance became 019's march
//! (module `board`). r55 (136): the Undecided pool and the tug of war are
//! gone; Unity swayed always goes onto the Unity track (r139; it could hold Discord off).
//! r67 (156): the purge is the only win (no Unity-deck win); every card
//! prints three tiers, picked by its side's tips (`tier_at`); an empty Unity
//! deck reshuffles, the Discord deck never does.

use crate::module::Hot;
use crate::content::{AlignmentCard, Op, Resource, Side};
use crate::key::{Key, KeyBuf};
use crate::module::{
    Build, Cx, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{adversary, cards, effects};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::int("tip_per_player", 1, 50, "the Unity track tips when it holds this many Sway per player"),
    ParamSpec::int("discord_tip_per_player", 1, 50, "the Discord track tips when it holds this many Sway per player"),
    ParamSpec::int("carry", 0, 2, "Sway past a tip stays toward the next tip: 2 both sides (r75, Matt's ruling 7), 1 Unity's only (r48), 0 neither (a tip empties its track)"),
    ParamSpec::int("momentum_every", 0, 10, "Momentum: a side's track holds 1 fewer Sway per this many of its tips, down to 1 per player (0: off)"),
    ParamSpec::int("troop", 0, 10, "Sway toward Unity for removing an adversary troop"),
    ParamSpec::int("free_city", 0, 10, "Sway toward Unity for freeing a corrupted city"),
    ParamSpec::int("free_facility", 0, 10, "Sway toward Unity for liberating an adversary station or factory"),
    ParamSpec::int("botnet", 0, 10, "Sway toward Unity for removing a botnet"),
    ParamSpec::list("tier_at", 1, 50, "the tip (its side's count, this tip included) from which its cards resolve strength II, then strength III"),
    ParamSpec::int("node_loss", 0, 1, "1: a player with no nodes on the board loses the game for everyone"),
    ParamSpec::int("clear_win_stage", 1, 3, "the win (no corrupted city) counts only once this round's adversary card is of this stage (1: any time)"),
];

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto(
        "sway",
        "the round's Sway: the adversary board's printed drip toward Discord (the Militarist prints none) and each Fallout token's",
    ),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "alignment",
    summary: "The Alignment tracks (Unity and Discord Sway, each tipping into its own cards), and winning and losing",
    requires: &["adversary", "ai", "board"],
    variants: &[
        VariantDef {
            name: "tracks",
            summary: "Two independent tracks: each side's Sway fills its own track and tips at its own threshold (136)",
            params: PARAMS,
            phases: PHASES,
            rng: &["alignment"],
            uses_cards: false,
            build,
        },
        VariantDef {
            name: "off",
            summary: "No Sway moves and nothing tips (for measuring Alignment); winning and losing stay",
            params: PARAMS,
            phases: PHASES,
            rng: &["alignment"],
            uses_cards: false,
            build,
        },
    ],
};

/// Every variant is one struct: `sway` and the action read the variant.
fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    let t = b.list("tier_at")?;
    if t.len() != 2 || t.iter().any(|&x| x < 1) || t[0] > t[1] {
        return Err("alignment.tier_at: give two tip counts of 1 or more, in order (strength II, strength III)".into());
    }
    Ok(Box::new(Alignment))
}

#[derive(Debug)]
struct Alignment;

/// Unity swayed during this action or phase, not yet on a track: once it
/// resolves, the players choose how much of it holds Discord off (136).
const UNITY_NEW: &str = "alignment.unity_new";
/// Credit income every player has from Unity cards.
const INCOME_BONUS: &str = "alignment.income_bonus";
/// Free keeps left (Unity cards).
const FREE_LEFT: &str = "alignment.free_purchases";
/// Insight every player gains each round from Unity cards.
const INSIGHT_BONUS: &str = "alignment.insight_bonus";

/// Credit income every player has from Unity cards (economy's upkeep).
pub fn income_bonus(g: &GameState) -> i64 {
    g.var(INCOME_BONUS)
}

/// Insight every player gains each round from Unity cards.
pub fn insight_bonus(g: &GameState) -> i64 {
    g.var(INSIGHT_BONUS)
}

/// Free keeps left from Unity cards.
pub fn free_purchases(g: &GameState) -> i64 {
    g.var(FREE_LEFT)
}

/// Uses up one free keep, if any is left; returns whether it did.
pub fn take_free_purchase(g: &mut GameState, cx: &mut Cx) -> bool {
    let free = g.var(FREE_LEFT);
    if free <= 0 {
        return false;
    }
    cx.set_key(g, FREE_LEFT, free - 1);
    true
}

/// Sway on a side's track.
pub fn track(side: Side) -> &'static str {
    match side {
        Side::Unity => "alignment.unity",
        Side::Discord => "alignment.discord",
    }
}

/// A side's tip count (only counts up).
pub fn tips_key(side: Side) -> &'static str {
    match side {
        Side::Unity => "alignment.unity_tips",
        Side::Discord => "alignment.discord_tips",
    }
}

/// A side's face-down deck (156: one a side; every card prints three tiers).
pub fn deck(side: Side) -> String {
    format!("alignment.deck.{}", side.name())
}

/// A side's resolved one-shot cards (top: the latest).
pub fn resolved(side: Side) -> String {
    format!("alignment.resolved.{}", side.name())
}

/// Cards left in a side's deck: the Discord deck is a clock (027); the
/// Unity deck reshuffles its resolved pile when empty (156).
pub fn cards_left(g: &GameState, side: Side) -> usize {
    g.pile(&deck(side)).len()
}

/// Unity tips so far, each one card drawn: they set its cards' strength (156: counted as tips, since the
/// reshuffle empties the resolved pile; r141: the win has no steps to read off them).
pub fn unity_drawn(g: &GameState) -> i64 {
    g.var(tips_key(Side::Unity))
}

/// 156: the tier (1-3) a card of `side` resolves at on its next tip; 157 (r68,
/// Matt 2026-11-09): that tip's own count against `tier_at` (tier I at tips 1-2,
/// II at 3-5, III from the 6th, at `tier_at` 3, 6).
pub fn next_tier(w: &World, g: &GameState, side: Side) -> usize {
    tier_of(w, g.var(tips_key(side)) + 1)
}

/// 157: the tier a side's `tip`th tip (1-based) resolves at.
fn tier_of(w: &World, tip: i64) -> usize {
    1 + reached(w.list("alignment.tier_at"), tip) as usize
}

/// Unity swayed this action or phase and still to be placed (136).
pub fn unity_pending(g: &GameState) -> i64 {
    g.var(UNITY_NEW)
}

fn card<'w>(w: &World<'w>, id: &str) -> &'w AlignmentCard {
    w.content.alignment_card(id)
}

/// Sway a side's track holds before it tips (Momentum applied).
pub fn capacity(w: &World, g: &GameState, side: Side) -> i64 {
    let seats = g.seats.len().max(1) as i64;
    let per = match side {
        Side::Unity => w.hot(Hot::TipPerPlayer),
        Side::Discord => w.hot(Hot::DiscordTipPerPlayer),
    };
    let full = per * seats;
    let every = w.hot(Hot::MomentumEvery);
    if every <= 0 {
        return full;
    }
    (full - g.var(tips_key(side)) / every).max(seats)
}

/// How many of `thresholds` a tip count has reached.
fn reached(thresholds: &[i64], tips: i64) -> i64 {
    thresholds.iter().filter(|&&t| tips >= t).count() as i64
}

/// Sways `n` toward `side`. Discord goes straight onto the Discord track;
/// Unity waits until the action or phase has resolved, then goes onto the
/// Unity track ([`place_unity`]). A track that fills tips then.
pub fn sway(g: &mut GameState, cx: &mut Cx, side: Side, n: i64) {
    if n <= 0 || cx.world.hot.alignment_off {
        return;
    }
    let key = match side {
        Side::Unity => UNITY_NEW,
        Side::Discord => track(Side::Discord),
    };
    cx.add_key(g, key, n);
    cx.tally(g, &KeyBuf::new("alignment.sway_").text(side.name()), n);
}

/// The Unity swayed this action or phase goes onto the Unity track (292: all of it, always; it could move Discord
/// off instead, 1 for 1). The Unity of one action is placed as it resolves: tips are checked after it.
fn place_unity(g: &mut GameState, cx: &mut Cx) -> Res {
    let n = g.var(UNITY_NEW);
    if n > 0 {
        cx.set_key(g, UNITY_NEW, 0);
        cx.add_key(g, track(Side::Unity), n);
    }
    Ok(())
}

/// [`sway`] toward Discord, tallied by where it came from (053).
pub fn discord_from(g: &mut GameState, cx: &mut Cx, from: &str, n: i64) {
    if n > 0 {
        cx.tally(g, &KeyBuf::new("alignment.discord_from.").text(from), n);
    }
    sway(g, cx, Side::Discord, n);
}

/// [`sway`] toward Unity, tallied by where it came from (136).
pub fn unity_from(g: &mut GameState, cx: &mut Cx, from: &str, n: i64) {
    if n > 0 && !cx.world.hot.alignment_off {
        cx.tally(g, &KeyBuf::new("alignment.unity_from.").text(from), n);
    }
    sway(g, cx, Side::Unity, n);
}

/// Sway toward Unity for a removed adversary troop (plus the remover's
/// Unity bonus).
pub fn for_troop_removed(g: &mut GameState, cx: &mut Cx, seat: Option<usize>) {
    let w = cx.world;
    let base = w.int("alignment.troop");
    let bonus: i64 = seat.map_or(0, |s| {
        cards::passives(&w, g, s)
            .iter()
            .map(|p| match p {
                Op::UnityBonus { amount } => *amount,
                _ => 0,
            })
            .sum()
    });
    unity_from(g, cx, "troop", base + bonus);
}

/// Sway toward Unity for a freed corrupted city.
pub fn for_city_freed(g: &mut GameState, cx: &mut Cx) {
    let n = cx.world.int("alignment.free_city");
    unity_from(g, cx, "city", n);
}

/// Sway toward Unity for a removed botnet.
pub fn for_botnet_removed(g: &mut GameState, cx: &mut Cx) {
    let n = cx.world.int("alignment.botnet");
    unity_from(g, cx, "botnet", n);
}

/// The win (r141): no corrupted city on the board, from an adversary card of stage `clear_win_stage` on (1: from
/// round 1; the stages are stacked and public).
fn liberated_won(w: &World, g: &GameState) -> bool {
    g.var(adversary::STAGE) >= w.hot(Hot::ClearWinStage) && adversary::no_corrupted_city(w, g)
}

/// Every player's `resource` income rises by `n` for good (credits or
/// insight: `Content::check` holds it to those).
pub fn add_team_income(g: &mut GameState, cx: &mut Cx, resource: Resource, n: i64) {
    let key = if resource == Resource::Insight { INSIGHT_BONUS } else { INCOME_BONUS };
    cx.add_key(g, key, n);
}

/// The next `n` keeps from the draft are free.
pub fn add_free_purchases(g: &mut GameState, cx: &mut Cx, n: i64) {
    cx.add_key(g, FREE_LEFT, n);
}

/// Up to `n` Sway comes off the Unity track (027; 136: it used to return to
/// the Undecided).
pub fn pull_unity(g: &mut GameState, cx: &mut Cx, n: i64) {
    let k = n.min(g.var(track(Side::Unity))).max(0);
    if k > 0 {
        cx.add_key(g, track(Side::Unity), -k);
        cx.tally(g, "alignment.unity_pulled", k);
    }
}

/// 297 (Matt, 2026-11-19): up to `n` Sway comes off the Discord track: a Unity card's printed line, the only
/// thing that takes Discord off (Unity swayed never cancels it).
pub fn pull_discord(g: &mut GameState, cx: &mut Cx, n: i64) {
    let k = n.min(g.var(track(Side::Discord))).max(0);
    if k > 0 {
        cx.add_key(g, track(Side::Discord), -k);
        cx.tally(g, "alignment.discord_pulled", k);
        cx.note(None, || format!("{k} Discord come{} off the Discord track", if k == 1 { "s" } else { "" }));
    }
}


/// Draws the next card of `side`. The Discord deck never reshuffles (027):
/// it is the game's clock. 156: an empty Unity deck shuffles its resolved
/// pile back in first (there is no Unity-deck win).
fn draw(g: &mut GameState, cx: &mut Cx, side: Side) -> Option<Key> {
    if side == Side::Unity && cards_left(g, side) == 0 {
        let mut ids: Vec<Key> = g.pile(&resolved(side)).to_vec();
        if !ids.is_empty() {
            shuffle(&mut ids, cx.rng(g, "alignment"));
            cx.set_pile(g, &resolved(side), Vec::new());
            cx.set_pile(g, &deck(side), ids);
            cx.tally(g, "alignment.unity_reshuffles", 1);
            cx.note(None, || "The Unity deck is empty: its resolved cards are shuffled back in".to_string());
        }
    }
    g.pile(&deck(side)).first().cloned()
}

/// A track tips: one card of its side resolves (at the tier this tip's own
/// count picks, 157), the side's tip count goes up by
/// one, and the track's Sway comes off (136: no pool to return to).
fn tip(g: &mut GameState, cx: &mut Cx, side: Side) -> Res {
    let w = cx.world;
    let round = g.round;
    cx.note(None, || format!("The {} track tips", side.title()));
    let before = g.var(tips_key(side));
    // r48 (Matt, 2026-11-05): a tip takes one tip's worth of Sway off; the
    // overflow stays on the track toward the next tip.
    let carry = w.int("alignment.carry");
    let carries = carry >= 2 || (carry == 1 && side == Side::Unity);
    let held = g.var(track(side));
    let back = if carries { capacity(&w, g, side).min(held) } else { held };
    cx.add_key(g, track(side), -back);
    cx.set_key(g, tips_key(side), before + 1);
    cx.tally(g, &KeyBuf::new("alignment.tips_").text(side.name()), 1);
    cx.tally(g, &format!("alignment.tip_{}.r{round:02}", side.name()), 1);
    let Some(id) = draw(g, cx, side) else {
        // Discord tips with no card left: the players lose (locked, 027).
        if side == Side::Discord {
            cx.end(g, false, "discord_deck", "a Discord tip found the Discord deck empty".into());
        }
        return Ok(());
    };
    let c = card(&w, &id);
    cx.move_card(g, &id, &deck(side), &resolved(side), true).unwrap();
    // 157: this tip's own count picks the tier (156 counted the tips before it).
    let tier = tier_of(&w, before + 1);
    let printed = &c.tiers[tier - 1];
    cx.note(None, || {
        format!("{} ({}, strength {}) resolves: {}", c.name, side.title(), crate::content::TIER_NUMERALS[tier - 1], printed.text)
    });
    let source = if side == Side::Discord { "discord" } else { "unity" };
    let fx0 = fx_snapshot(g);
    effects::run(g, cx, &printed.effects, &mut effects::Ctx::team().named(source).why(&c.name))?;
    // 167: what the card did, for the per-effect statistics (`scripts/cards/effect_stats.py`).
    let after = fx_snapshot(g);
    cx.tally(g, &format!("fx.{id}.t{tier}"), 1);
    for (i, name) in FX_MEASURES.iter().enumerate() {
        if after[i] != fx0[i] {
            cx.tally(g, &format!("fx.{id}.{name}"), after[i] - fx0[i]);
        }
    }
    cx.tally(g, &KeyBuf::new("alignment.card.").text(&id), 1);
    cx.tally(g, &format!("alignment.tier_{}.{tier}", side.name()), 1);
    Ok(())
}

/// 167: the board and the team, measured around a Unity or Discord card (deltas tallied as `fx.<id>.<measure>`).
const FX_MEASURES: [&str; 18] = [
    "agents", "troops", "botnets", "land", "corrupted", "threats", "credits", "energy", "insight", "ambassadors",
    "free_keeps", "income_credits", "income_insight", "unity", "unity_pulled", "discord", "discord_pulled", "research",
];

fn fx_snapshot(g: &GameState) -> [i64; 18] {
    let board = |kind: &str| (0..g.hexes.len()).map(|h| g.tokens(h, kind)).sum::<i64>();
    let seats = |r: Resource| (0..g.seats.len()).map(|s| g.resource(s, r)).sum::<i64>();
    let counter = |k: &str| g.stats.counters.get(k).copied().unwrap_or(0);
    [
        board(ADV_AMBASSADOR),
        board(TROOP),
        board(BOTNET),
        board(FALLOUT),
        (0..g.hexes.len()).filter(|&h| corrupted(g, h)).count() as i64,
        super::threats::active(g).len() as i64,
        seats(Resource::Credits),
        seats(Resource::Energy),
        seats(Resource::Insight),
        seats(Resource::Ambassadors),
        g.var(FREE_LEFT),
        g.var(INCOME_BONUS),
        g.var(INSIGHT_BONUS),
        counter("alignment.sway_unity"),
        counter("alignment.unity_pulled"),
        counter("alignment.sway_discord"),
        // 297: what the four Unity removers take off, and what Captured Labs feeds.
        counter("alignment.discord_pulled"),
        counter("adversary.research_made"),
    ]
}

/// "Discord tips so far; ..." with the tier steps as configured (156).
fn tier_words(side: &str, w: &World) -> String {
    let t = w.list("alignment.tier_at");
    format!(
        "{side} tips so far; a {side} card resolves strength I on tips 1-{}, II from the tip numbered {}, III from tip {}",
        t[0] - 1, t[0], t[1]
    )
}

impl Module for Alignment {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for side in [Side::Unity, Side::Discord] {
            cx.set_key(g, track(side), 0);
            cx.set_key(g, tips_key(side), 0);
            let mut ids: Vec<Key> = w
                .content
                .alignment
                .iter()
                .filter(|c| c.side == side)
                .map(|c| Key::new(&c.id))
                .collect();
            shuffle(&mut ids, cx.rng(g, "alignment"));
            // Difficulty 4: the Discord deck, the clock, is shorter.
            if side == Side::Discord && adversary::level(&w, 4) {
                let cut = w.int("adversary.discord_removed") as usize;
                ids.truncate(ids.len().saturating_sub(cut));
            }
            // 136: every printed card is dealt; 156: one deck a side.
            cx.set_pile(g, &deck(side), ids);
        }
        // Difficulty 4: the removed Discord cards count as tips made, so the
        // tiers come a tip sooner before the deck ends (034).
        if adversary::level(&w, 4) {
            cx.set_key(g, tips_key(Side::Discord), w.int("adversary.discord_removed"));
        }
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        // 136: the board's printed drip (it was `adversary_discord`, 0).
        let drip = cx.world.content.adversary_board.discord_drip;
        discord_from(g, cx, "drip", drip);
        // 138: each Fallout token drips 1 Discord (r98: none lies face down).
        let blight = adversary::blight_drip(&cx.world, g);
        discord_from(g, cx, "blight", blight);
        // 235: say where the round's Discord came from (the log showed a bare var change).
        if drip + blight > 0 && !cx.world.hot.alignment_off {
            cx.note(None, || format!("The round's Discord drip: {drip} printed, {blight} from Fallout"));
        }
        // 210 (stats only): Fallout tokens on the board as the round's drip reads them.
        let land: i64 = (0..g.hexes.len()).map(|h| g.tokens(h, FALLOUT)).sum();
        cx.per_round(g, "adversary.land_tokens", land);
        Ok(())
    }

    fn after(&self, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        // The Unity just swayed is placed (or holds Discord off), then full
        // tracks tip, Discord first; a tip's card can sway again.
        let tips = if w.hot.alignment_off {
            0
        } else {
            64
        };
        'tips: for _ in 0..tips {
            place_unity(g, cx)?;
            for side in [Side::Discord, Side::Unity] {
                if g.var(track(side)) >= capacity(&w, g, side) {
                    // 145: a player with no node loses before a Unity tip is
                    // drawn, whether or not a Discord tip came first (028's rule
                    // held only after a tip).
                    if side == Side::Unity && w.hot(Hot::NodeLoss) > 0 && adversary::loss(g).is_some() {
                        break;
                    }
                    tip(g, cx, side)?;
                    if g.is_over() {
                        return Ok(());
                    }
                    // 028: a card that takes a last node loses here, before
                    // a Unity tip can win a game with no player on the board.
                    if w.hot(Hot::NodeLoss) > 0 && adversary::loss(g).is_some() {
                        break;
                    }
                    continue 'tips;
                }
            }
            break;
        }
        // Losses first.
        if w.hot(Hot::NodeLoss) > 0 {
            if let Some((reason, detail)) = adversary::loss(g) {
                cx.end(g, false, reason, detail);
                return Ok(());
            }
        }
        if liberated_won(&w, g) {
            cx.end(g, true, "liberated", "no city is corrupted".to_string());
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        out.global(
            "unity",
            "Unity track",
            g.var(track(Side::Unity)),
            Some(capacity(w, g, Side::Unity)),
            "Sway toward the good AIs; tips into a Unity card when full",
        );
        out.global(
            "discord",
            "Discord track",
            g.var(track(Side::Discord)),
            Some(capacity(w, g, Side::Discord)),
            "Sway toward the evil AI; tips into a Discord card when full",
        );
        let next = |list: &[i64], n: i64| list.iter().copied().find(|&t| t > n);
        let ut = g.var(tips_key(Side::Unity));
        let dt = g.var(tips_key(Side::Discord));
        out.global(
            "unity_tips",
            "Unity tips",
            ut,
            next(w.list("alignment.tier_at"), ut),
            &tier_words("Unity", w),
        );
        out.global(
            "discord_tips",
            "Discord tips",
            dt,
            next(w.list("alignment.tier_at"), dt),
            &tier_words("Discord", w),
        );
        let cities = (0..g.hexes.len()).filter(|&h| is_city(w, h)).count() as i64;
        let bad = (0..g.hexes.len())
            .filter(|&h| corrupted_city(w, g, h))
            .count() as i64;
        out.global(
            "corrupted_cities",
            "corrupted cities",
            bad,
            Some(cities),
            // 285: no agent income since r130: a corrupted city's harm each round is its energy as Discord.
            "the win: no corrupted city on the board; each makes Discord every round unless a player ambassador stands on it, and none is liberated with a troop on or next to it",
        );
        // 135: a negative team income (Capital Flight, a project until r85) takes it below 0.
        if g.var(INCOME_BONUS) != 0 {
            out.global(
                "income_bonus",
                "team income bonus",
                g.var(INCOME_BONUS),
                None,
                "credit income every player has from Unity cards (less from adversary projects)",
            );
        }
        if g.var(INSIGHT_BONUS) > 0 {
            out.global(
                "insight_bonus",
                "Unity insight bonus",
                g.var(INSIGHT_BONUS),
                None,
                "insight every player gains each round from Unity cards",
            );
        }
        if g.var(FREE_LEFT) > 0 {
            out.global(
                "free_purchases",
                "free keeps",
                g.var(FREE_LEFT),
                None,
                "draft keeps that are free",
            );
        }
        let show = |id: &str, status: &str| {
            let c = card(w, id);
            CardView {
                id: id.to_string(),
                name: c.name.clone(),
                text: format!("{}: {}", c.side.title(), c.text()),
                status: status.to_string(),
                cost: None,
            }
        };
        for side in [Side::Unity, Side::Discord] {
            let n = cards_left(g, side);
            let label = match side {
                Side::Unity => "Unity deck (reshuffles its resolved cards when empty)",
                Side::Discord => "Discord deck (a tip with no card left loses)",
            };
            out.pile(&format!("deck.{}", side.name()), label, None, n, Vec::new());
            let done = g.pile(&resolved(side));
            let label = match side {
                Side::Unity => "Unity cards resolved",
                Side::Discord => "Discord cards resolved",
            };
            out.pile(
                &format!("resolved.{}", side.name()),
                label,
                None,
                done.len(),
                done.iter().map(|id| show(id, "resolved")).collect(),
            );
        }
        // 136: Unity waiting to be placed lives only within one action's check.
        out.bookkeeping(&["alignment.unity_new"]);
    }
}

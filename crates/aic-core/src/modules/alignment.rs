//! Alignment and the end of the game: the two Alignment tracks (Unity and
//! Discord, each fed with its own Sway and tipping at its own threshold),
//! the Unity and Discord decks, the antimatter action, and the win and loss
//! conditions, checked after every action and every automatic phase.
//!
//! Replaces human opposition (rules revision 5). Since 017 every card is
//! one-shot; 017–018's Discord-tip troop advance became 019's march
//! (module `board`). r55 (136): the Undecided pool and the tug of war are
//! gone; Unity swayed may instead hold Discord off, 1 for 1 (`hold_off`).
//! r67 (156): the purge is the only win (no Unity-deck win); every card
//! prints three tiers, picked by its side's tips (`tier_at`); an empty Unity
//! deck reshuffles, the Discord deck never does.

use crate::module::Hot;
use crate::action::Action;
use crate::content::{AlignmentCard, Op, Resource, Side};
use crate::key::{Key, KeyBuf};
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{adversary, board, cards, effects};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::int("tip_per_player", 1, 50, "the Unity track tips when it holds this many Sway per player"),
    ParamSpec::int("discord_tip_per_player", 1, 50, "the Discord track tips when it holds this many Sway per player"),
    ParamSpec::int("carry", 0, 2, "Sway past a tip stays toward the next tip: 1 Unity's only (Matt, r48), 2 both sides (an adversary lever), 0 neither (a tip empties its track)"),
    ParamSpec::int("hold_off", 0, 1, "1: each Unity swayed may instead move 1 Discord off the Discord track, the players' choice (136)"),
    ParamSpec::int("momentum_every", 0, 10, "Momentum: a side's track holds 1 fewer Sway per this many of its tips, down to 1 per player (0: off)"),
    ParamSpec::int("troop", 0, 10, "Sway toward Unity for removing an adversary troop"),
    ParamSpec::int("free_city", 0, 10, "Sway toward Unity for freeing a corrupted city"),
    ParamSpec::int("free_facility", 0, 10, "Sway toward Unity for liberating an adversary station or factory (137)"),
    ParamSpec::int("botnet", 0, 10, "Sway toward Unity for removing a botnet"),
    ParamSpec::int("antimatter_discord", 0, 5, "the antimatter action: Sway toward Discord to produce 1 antimatter (0: no such action)"),
    ParamSpec::int("antimatter_uses", 0, 5, "times each player may take the antimatter action per round"),
    ParamSpec::list("purge_at", 1, 50, "Unity tips at which the purge asks less: ambassadors, then troops, then botnets stop counting (136; 156: tips, not cards drawn, since the Unity deck reshuffles)"),
    ParamSpec::list("tier_at", 1, 50, "156: a side's tip count (before this tip) from which its cards resolve tier II, then tier III"),
    ParamSpec::int("node_loss", 0, 1, "1: a player with no nodes on the board loses the game for everyone"),
    ParamSpec::int("clear_win_stage", 1, 3, "the bar-0 win counts only once this round's adversary card is of this stage (1: any time)"),
];

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto(
        "sway",
        "the round's Sway: the adversary board's printed drip toward Discord (the Militarist prints none) and each land-corruption token's",
    ),
    PhaseSpec::decision(
        "act",
        "the antimatter action: sway toward Discord to produce 1 antimatter",
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
    let v = b.list("purge_at")?;
    if v.is_empty() || v.len() > 3 || v.iter().any(|&x| x < 1) || v.windows(2).any(|p| p[0] > p[1]) {
        return Err("alignment.purge_at: give up to three Unity-card counts of 1 or more, in order".into());
    }
    let t = b.list("tier_at")?;
    if t.len() != 2 || t.iter().any(|&x| x < 1) || t[0] > t[1] {
        return Err("alignment.tier_at: give two tip counts of 1 or more, in order (tier II, tier III)".into());
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

/// Unity tips so far, each one card drawn: the purge's steps (136; 156:
/// counted as tips, since the reshuffle empties the resolved pile).
pub fn unity_drawn(g: &GameState) -> i64 {
    g.var(tips_key(Side::Unity))
}

/// 156: the tier (1-3) a card of `side` resolves at on its next tip: the
/// side's tips so far against `tier_at`.
pub fn next_tier(w: &World, g: &GameState, side: Side) -> usize {
    1 + reached(w.list("alignment.tier_at"), g.var(tips_key(side))) as usize
}

/// Unity swayed this action or phase and still to be placed (136).
pub fn unity_pending(g: &GameState) -> i64 {
    g.var(UNITY_NEW)
}

fn used_key(seat: usize) -> String {
    format!("alignment.used.{seat}")
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

/// The clear-board bar level, from Unity cards drawn (136; see
/// `adversary::under_bar`).
pub fn bar_level(w: &World, g: &GameState) -> adversary::PurgeLevel {
    adversary::PurgeLevel::from_steps(reached(w.list("alignment.purge_at"), unity_drawn(g)))
}

/// Sways `n` toward `side`. Discord goes straight onto the Discord track;
/// Unity waits until the action or phase has resolved, when the players
/// choose how much of it holds Discord off instead ([`place_unity`]). A
/// track that fills tips then.
pub fn sway(g: &mut GameState, cx: &mut Cx, side: Side, n: i64) {
    if n <= 0 || cx.world.variant("alignment") == Some("off") {
        return;
    }
    let key = match side {
        Side::Unity => UNITY_NEW,
        Side::Discord => track(Side::Discord),
    };
    cx.add_key(g, key, n);
    cx.tally(g, &KeyBuf::new("alignment.sway_").text(side.name()), n);
}

/// Moves up to `n` Discord off the Discord track (Envoy, or Unity held
/// back); returns how many moved.
pub fn hold_off(g: &mut GameState, cx: &mut Cx, n: i64) -> i64 {
    let k = n.min(g.var(track(Side::Discord))).max(0);
    if k > 0 {
        cx.add_key(g, track(Side::Discord), -k);
        cx.tally(g, "alignment.discord_held_off", k);
    }
    k
}

/// The Unity swayed this action or phase goes on the Unity track, or, as
/// the players choose (seat 0 asks for the team), up to all of it moves
/// Discord off the Discord track 1 for 1 (136, `hold_off`).
fn place_unity(g: &mut GameState, cx: &mut Cx) -> Res {
    let n = g.var(UNITY_NEW);
    if n <= 0 {
        return Ok(());
    }
    let w = cx.world;
    let discord = g.var(track(Side::Discord));
    let most = if w.int("alignment.hold_off") > 0 { n.min(discord) } else { 0 };
    // 145: a hold-off that leaves the Discord track full still tips, and the
    // tip empties the track (unless Discord carries), so the Unity spent on it
    // is lost: only amounts that stop the tip are offered.
    let cap = capacity(&w, g, Side::Discord);
    let empties = w.int("alignment.carry") < 2;
    let ks: Vec<i64> = (0..=most).filter(|&k| k == 0 || !empties || discord - k < cap).collect();
    let k = if ks.len() > 1 {
        let opts = ks
            .iter()
            // 145: no "all 1" and no ", 0 onto Unity" on the buttons.
            .map(|&k| {
                opt(match k {
                    0 if n == 1 => "onto the Unity track".to_string(),
                    0 => format!("all {n} onto the Unity track"),
                    k if k == n => format!("{k} off the Discord track"),
                    k => format!("{k} off the Discord track, {} onto Unity", n - k),
                })
            })
            .collect();
        let prompt = || format!("{n} Unity swayed: how much holds Discord off instead?");
        ks[cx.choose(0, prompt, opts)?.unwrap_or(0)]
    } else {
        0
    };
    cx.set_key(g, UNITY_NEW, 0);
    if k > 0 {
        hold_off(g, cx, k);
        cx.tally(g, "alignment.unity_held_off", k);
    }
    cx.add_key(g, track(Side::Unity), n - k);
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
    if n > 0 && cx.world.variant("alignment") != Some("off") {
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

/// Whether the adversary's pieces are under the bar for `level`. The
/// full bar (no adversary pieces) counts only once this round's adversary
/// card is of stage `clear_win_stage` (the stages are stacked and public).
fn purge_won(w: &World, g: &GameState, level: adversary::PurgeLevel) -> bool {
    if level == adversary::PurgeLevel::All && g.var(adversary::STAGE) < w.hot(Hot::ClearWinStage) {
        return false;
    }
    adversary::under_bar(w, g, level)
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

/// A track tips: one card of its side resolves (at the tier its side's
/// tip count before this tip picks, 156), the side's tip count goes up by
/// one, and the track's Sway comes off (136: no pool to return to).
fn tip(g: &mut GameState, cx: &mut Cx, side: Side) -> Res {
    let w = cx.world;
    let round = g.round;
    cx.note(None, || format!("The {} track tips", side.name()));
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
    // 137: on a Discord tip each neutral factory makes 1 corp troop.
    if side == Side::Discord {
        board::tip_corp_troops(g, cx);
    }
    let Some(id) = draw(g, cx, side) else {
        // Discord tips with no card left: the players lose (locked, 027).
        if side == Side::Discord {
            cx.end(g, false, "discord_deck", "a Discord tip found the Discord deck empty".into());
        }
        return Ok(());
    };
    let c = card(&w, &id);
    cx.move_card(g, &id, &deck(side), &resolved(side), true).unwrap();
    // 156: the tip count before this tip picks the tier.
    let tier = 1 + reached(w.list("alignment.tier_at"), before) as usize;
    let printed = &c.tiers[tier - 1];
    cx.note(None, || {
        format!("{} ({}, tier {}) resolves: {}", c.name, side.name(), crate::content::TIER_NUMERALS[tier - 1], printed.text)
    });
    let source = if side == Side::Discord { "discord" } else { "unity" };
    effects::run(g, cx, &printed.effects, &mut effects::Ctx::team().named(source).why(&c.name))?;
    cx.tally(g, &KeyBuf::new("alignment.card.").text(&id), 1);
    cx.tally(g, &format!("alignment.tier_{}.{tier}", side.name()), 1);
    Ok(())
}

/// "Discord tips so far; ..." with the tier steps as configured (156).
fn tier_words(side: &str, w: &World) -> String {
    let t = w.list("alignment.tier_at");
    format!(
        "{side} tips so far; the next {side} card resolves tier I below {} earlier tips, II from {}, III from {}",
        t[0], t[0], t[1]
    )
}

/// Whether the antimatter action exists in this game (uses aside).
fn antimatter_on(w: &World) -> bool {
    w.int("alignment.antimatter_discord") > 0 && w.variant("alignment") != Some("off")
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

    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            if g.var(&used_key(s)) != 0 {
                cx.set_key(g, &used_key(s), 0);
            }
        }
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        // 136: the board's printed drip (it was `adversary_discord`, 0).
        let drip = cx.world.content.adversary_board.discord_drip;
        discord_from(g, cx, "drip", drip);
        // 138: each land-corruption token drips 1 Discord.
        let blight: i64 = (0..g.hexes.len()).map(|h| g.tokens(h, LAND_CORRUPTION)).sum();
        discord_from(g, cx, "blight", blight * cx.world.int("adversary.blight_discord"));
        Ok(())
    }

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        if antimatter_on(w) && g.var(&used_key(seat as usize)) < w.int("alignment.antimatter_uses") {
            out.push(Action::Antimatter { seat });
        }
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Antimatter { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let s = a.seat() as usize;
        let n = w.int("alignment.antimatter_discord");
        if !antimatter_on(&w) || g.var(&used_key(s)) >= w.int("alignment.antimatter_uses") {
            return Err(Halt::Illegal("no antimatter action left this round".into()));
        }
        cx.add_key(g, &used_key(s), 1);
        cx.add_resource(g, s, Resource::Antimatter, 1);
        discord_from(g, cx, "antimatter", n);
        cx.count(g, "antimatter_actions", 1);
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
        let level = bar_level(&w, g);
        if purge_won(&w, g, level) {
            cx.end(
                g,
                true,
                "purge",
                format!(
                    "the adversary fell under the bar for {} Unity tips",
                    unity_drawn(g)
                ),
            );
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
            "Sway toward the evil AI; tips into a Discord card when full (Unity swayed may move it off instead, as Envoy does)",
        );
        let next = |list: &[i64], n: i64| list.iter().copied().find(|&t| t > n);
        let ut = g.var(tips_key(Side::Unity));
        let dt = g.var(tips_key(Side::Discord));
        out.global(
            "unity_tips",
            "Unity tips",
            ut,
            next(w.list("alignment.purge_at"), ut),
            "Unity tips so far; the purge asks less at each step, and the next Unity card resolves at the tier they reach",
        );
        out.global(
            "discord_tips",
            "Discord tips",
            dt,
            next(w.list("alignment.tier_at"), dt),
            &tier_words("Discord", w),
        );
        let level = bar_level(w, g);
        let mut text = level.win_text().to_string();
        let stage = w.int("alignment.clear_win_stage");
        if level == adversary::PurgeLevel::All && g.var(adversary::STAGE) < stage {
            text += &format!(" (from adversary stage {stage})");
        }
        out.global(
            "bar",
            "clear-board bar",
            level.steps(),
            Some(w.list("alignment.purge_at").len() as i64),
            &text,
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
            "corrupted cities always count against the purge; each one builds adversary ambassadors",
        );
        // 135: Capital Flight (a project) takes it below 0.
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
                text: format!("{}: {}", c.side.name(), c.text()),
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
        out.bookkeeping(&["alignment.used.", "alignment.unity_new"]);
        let uses = w.int("alignment.antimatter_uses");
        if antimatter_on(w) {
            for s in 0..g.seats.len() {
                out.seat(
                    s,
                    "antimatter_left",
                    "antimatter actions left",
                    (uses - g.var(&used_key(s))).max(0),
                    Some(uses),
                    "sway toward Discord to produce 1 antimatter",
                );
            }
        }
    }
}

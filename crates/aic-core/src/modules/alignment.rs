//! Alignment and the end of the game: the two Alignment tracks (Unity and
//! Discord, each fed with its own Sway and tipping at its own threshold),
//! the Unity and Discord decks, the antimatter action, and the win and loss
//! conditions, checked after every action and every automatic phase.
//!
//! Replaces human opposition (rules revision 5). Since 017 every card is
//! one-shot, a Discord card's strength steps up with the Discord tip count
//! (printed steps); 017–018's Discord-tip troop advance became 019's march
//! (module `board`). r55 (136): the Undecided pool and the tug of war are
//! gone; Unity swayed may instead hold Discord off, 1 for 1 (`hold_off`).

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
    ParamSpec::int("threat_discord", 0, 10, "Sway toward Discord when a threat executes uncountered"),
    ParamSpec::int("antimatter_discord", 0, 5, "the antimatter action: Sway toward Discord to produce 1 antimatter (0: no such action)"),
    ParamSpec::int("antimatter_uses", 0, 5, "times each player may take the antimatter action per round"),
    ParamSpec::list("purge_at", 1, 50, "Unity cards drawn at which the purge asks less: ambassadors, then troops, then botnets stop counting (136)"),
    ParamSpec::int("unity_tier2", 0, 50, "Unity cards drawn from which a Unity tip draws the stronger tier"),
    ParamSpec::int("discord_tier2", 1, 50, "Discord tip count from which a Discord tip draws the stronger tier"),
    ParamSpec::int("node_loss", 0, 1, "1: a player with no nodes on the board loses the game for everyone"),
    ParamSpec::int("clear_win_stage", 1, 3, "the bar-0 win counts only once this round's adversary card is of this stage (1: any time)"),
];

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto(
        "sway",
        "the round's Sway: the adversary board's printed drip toward Discord (the Militarist prints none)",
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

fn deck(side: Side, tier: u8) -> KeyBuf {
    KeyBuf::new("alignment.deck.").text(side.name()).text(".").num(tier as usize)
}

/// A side's resolved one-shot cards (top: the latest).
pub fn resolved(side: Side) -> String {
    format!("alignment.resolved.{}", side.name())
}

/// Cards left in a side's decks: its clock (027).
pub fn cards_left(g: &GameState, side: Side) -> usize {
    g.pile(&deck(side, 1)).len() + g.pile(&deck(side, 2)).len()
}

/// Unity cards drawn so far: the purge's steps (136).
pub fn unity_drawn(g: &GameState) -> i64 {
    g.pile(&resolved(Side::Unity)).len() as i64
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
    let most = if w.int("alignment.hold_off") > 0 { n.min(g.var(track(Side::Discord))) } else { 0 };
    let k = if most > 0 {
        let opts = (0..=most)
            .map(|k| opt(if k == 0 { format!("all {n} onto the Unity track") } else { format!("{k} off the Discord track, {} onto Unity", n - k) }))
            .collect();
        let prompt = || format!("{n} Unity swayed: how much holds Discord off instead?");
        cx.choose(0, prompt, opts)?.unwrap_or(0) as i64
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


/// Draws the next card of `side`: the stronger tier once the side's tip
/// count has reached `unity_tier2` / `discord_tier2`, else tier 1; the other
/// tier if that deck is empty. Neither deck ever reshuffles (027): they are
/// the game's clocks.
fn draw(g: &GameState, w: &World, side: Side, tips: i64) -> Option<(Key, String)> {
    let first = match side {
        Side::Unity => w.int("alignment.unity_tier2"),
        Side::Discord => w.int("alignment.discord_tier2"),
    };
    let want: u8 = if tips >= first { 2 } else { 1 };
    for tier in [want, 3 - want] {
        if let Some(id) = g.pile(&deck(side, tier)).first().cloned() {
            return Some((id, deck(side, tier).to_string()));
        }
    }
    None
}

/// A track tips: one card of its side resolves (at the strength step
/// its side's tip count has reached), the side's tip count goes up by
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
    let Some((id, from)) = draw(g, &w, side, before) else {
        // Discord tips with no card left: the players lose (locked, 027).
        if side == Side::Discord {
            cx.end(g, false, "discord_deck", "a Discord tip found the Discord deck empty".into());
        }
        return Ok(());
    };
    let c = card(&w, &id);
    cx.move_card(g, &id, &from, &resolved(side), true).unwrap();
    // Drawing the last Unity card wins outright (027); it needn't act.
    if side == Side::Unity && cards_left(g, side) == 0 {
        cx.end(g, true, "unity_deck", "the last Unity card was drawn".into());
        return Ok(());
    }
    cx.note(None, || {
        format!("{} ({}) resolves: {}", c.name, side.name(), c.text)
    });
    let effects = c
        .steps
        .iter()
        .rev()
        .find(|st| before >= st.at)
        .map_or(&c.effects, |st| &st.effects);
    let source = if side == Side::Discord { "discord" } else { "unity" };
    effects::run(g, cx, effects, &mut effects::Ctx::team().named(source).why(&c.name))?;
    cx.tally(g, &KeyBuf::new("alignment.card.").text(&id), 1);
    Ok(())
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
            for tier in [1u8, 2] {
                let mut ids: Vec<Key> = w
                    .content
                    .alignment
                    .iter()
                    .filter(|c| c.side == side && c.tier == tier)
                    .map(|c| Key::new(&c.id))
                    .collect();
                shuffle(&mut ids, cx.rng(g, "alignment"));
                // Difficulty 4: the Discord deck, the clock, is shorter.
                if side == Side::Discord && tier == 1 && adversary::level(&w, 4) {
                    let cut = w.int("adversary.discord_removed") as usize;
                    ids.truncate(ids.len().saturating_sub(cut));
                }
                // 136: every printed Unity card is dealt (028-135 dealt
                // `unity_cards` of them); drawing the last one wins.
                cx.set_pile(g, &deck(side, tier), ids);
            }
        }
        // Difficulty 4: the removed Discord cards count as tips made, so every
        // printed step is still reached before the deck ends (034).
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
                    "the adversary fell under the bar for {} Unity cards drawn",
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
            next(w.list("alignment.purge_at"), unity_drawn(g)),
            "Unity tips so far; the purge asks less at each step of Unity cards drawn",
        );
        let tier2 = w.int("alignment.discord_tier2");
        out.global(
            "discord_tips",
            "Discord tips",
            dt,
            (dt < tier2).then_some(tier2),
            "Discord tips so far; a Discord card's stronger steps count the tips before its own: \"3+ earlier\" first fires on the 4th tip",
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
                text: format!("{} {}: {}", c.side.name(), c.tier, c.text),
                status: status.to_string(),
                cost: None,
            }
        };
        for side in [Side::Unity, Side::Discord] {
            let n = cards_left(g, side);
            let label = match side {
                Side::Unity => "Unity deck (drawing the last card wins)",
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

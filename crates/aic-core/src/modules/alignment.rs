//! Alignment and the end of the game: the Alignment track (the Undecided
//! and the Unity and Discord buckets, fed with Sway), tipping and the Unity
//! and Discord decks, the antimatter action, and the win and loss
//! conditions, checked after every action and every automatic phase.
//!
//! Replaces human opposition (rules revision 5). Since 017 every card is
//! one-shot, a Discord card's strength steps up with the Discord tip count
//! (printed steps); 017–018's Discord-tip troop advance became 019's march
//! (module `board`).

use crate::action::Action;
use crate::content::{AlignmentCard, Op, Resource, Side};
use crate::key::Key;
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ViewOut};

use super::common::*;
use super::{adversary, board, cards, effects};

const PARAMS: [ParamSpec; 16] = [
    ParamSpec::int("undecided_per_player", 1, 50, "Sway in the Undecided at the start, per player"),
    ParamSpec::int("tip_per_player", 1, 50, "the Unity bucket tips when it holds this many Sway per player"),
    ParamSpec::int("discord_tip_per_player", 1, 50, "the Discord bucket tips when it holds this many Sway per player"),
    ParamSpec::int("momentum_every", 0, 10, "Momentum: a side's bucket holds 1 fewer Sway per this many of its tips, down to 1 per player (0: off)"),
    ParamSpec::int("troop", 0, 10, "Sway toward Unity for removing an adversary troop"),
    ParamSpec::int("free_city", 0, 10, "Sway toward Unity for freeing a corrupted city"),
    ParamSpec::int("node", 0, 10, "Sway toward Unity for removing a node"),
    ParamSpec::int("threat_discord", 0, 10, "Sway toward Discord when a threat executes uncountered"),
    ParamSpec::int("adversary_discord", 0, 10, "Sway toward Discord the adversary adds at the start of every round"),
    ParamSpec::int("antimatter_discord", 0, 5, "the antimatter action: Sway toward Discord to produce 1 antimatter (0: no such action)"),
    ParamSpec::int("antimatter_uses", 0, 5, "times each player may take the antimatter action per round"),
    ParamSpec::list("unity_ease", 0, 50, "Unity tip counts at which the clear-board bar drops a level: ambassadors, then troops, then nodes stop counting (the first also starts the stronger Unity cards)"),
    ParamSpec::list("unity_cards", 1, 6, "Unity cards a game deals from the printed deck, tier 1 then tier 2 (drawing the last one wins)"),
    ParamSpec::int("discord_tier2", 1, 50, "Discord tip count from which a Discord tip draws the stronger tier"),
    ParamSpec::int("presence_loss", 0, 1, "1: a player with no presence on the board loses the game for everyone"),
    ParamSpec::int("clear_win_stage", 1, 3, "the bar-0 win counts only once this round's adversary card is of this stage (1: any time)"),
];

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto(
        "sway",
        "the round's Sway: the adversary toward Discord",
    ),
    PhaseSpec::decision(
        "act",
        "the antimatter action: sway toward Discord to produce 1 antimatter",
    ),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "alignment",
    summary: "The Alignment track (Sway, the Undecided, Unity and Discord buckets that tip into cards), and winning and losing",
    requires: &["adversary", "ai", "board"],
    variants: &[
        VariantDef {
            name: "tug",
            summary: "Tug of war: Sway with the Undecided empty comes from the opposite bucket",
            params: &PARAMS,
            phases: PHASES,
            rng: &["alignment"],
            uses_cards: false,
            build,
        },
        VariantDef {
            name: "still",
            summary: "Sway with the Undecided empty does nothing",
            params: &PARAMS,
            phases: PHASES,
            rng: &["alignment"],
            uses_cards: false,
            build,
        },
        VariantDef {
            name: "off",
            summary: "No Sway moves and nothing tips (for measuring Alignment); winning and losing stay",
            params: &PARAMS,
            phases: PHASES,
            rng: &["alignment"],
            uses_cards: false,
            build,
        },
    ],
};

/// Every variant is one struct: `sway` and the action read the variant.
fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    let v = b.list("unity_ease")?;
    if v.is_empty() || v.iter().any(|&x| x < 1) || v.windows(2).any(|p| p[0] > p[1]) {
        return Err("alignment.unity_ease: give tip counts of 1 or more, in order".into());
    }
    if b.list("unity_cards")?.len() != 2 {
        return Err("alignment.unity_cards: give two counts, tier 1 and tier 2".into());
    }
    // Sway only moves between the Undecided and the buckets: with less than a
    // tip in all, nothing could ever tip (020).
    if b.int("undecided_per_player")? < b.int("tip_per_player")?.max(b.int("discord_tip_per_player")?) {
        return Err("alignment.undecided_per_player: below a bucket's tip size, it could never tip (the Undecided must hold at least one tip)".into());
    }
    Ok(Box::new(Alignment))
}

#[derive(Debug)]
struct Alignment;

/// Sway in the Undecided.
const UNDECIDED: &str = "alignment.undecided";
/// Credit income every player has from Unity cards.
const INCOME_BONUS: &str = "alignment.income_bonus";
/// Free card purchases left (Unity cards).
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

/// Free card purchases left from Unity cards.
pub fn free_purchases(g: &GameState) -> i64 {
    g.var(FREE_LEFT)
}

/// Uses up one free card purchase, if any is left; returns whether it did.
pub fn take_free_purchase(g: &mut GameState, cx: &mut Cx) -> bool {
    let free = g.var(FREE_LEFT);
    if free <= 0 {
        return false;
    }
    cx.set_key(g, FREE_LEFT, free - 1);
    true
}

/// Sway in a side's bucket.
pub fn bucket(side: Side) -> &'static str {
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

fn deck(side: Side, tier: u8) -> String {
    format!("alignment.deck.{}.{tier}", side.name())
}

/// A side's resolved one-shot cards (top: the latest).
pub fn resolved(side: Side) -> String {
    format!("alignment.resolved.{}", side.name())
}

/// Cards left in a side's decks: its clock (027).
pub fn cards_left(g: &GameState, side: Side) -> usize {
    g.pile(&deck(side, 1)).len() + g.pile(&deck(side, 2)).len()
}

fn used_key(seat: usize) -> String {
    format!("alignment.used.{seat}")
}

fn card<'w>(w: &World<'w>, id: &str) -> &'w AlignmentCard {
    &w.content.alignment[w
        .content
        .alignment_index(id)
        .expect("alignment card in content")]
}

/// Sway a side's bucket holds before it tips (Momentum applied).
pub fn capacity(w: &World, g: &GameState, side: Side) -> i64 {
    let seats = g.seats.len().max(1) as i64;
    let per = match side {
        Side::Unity => w.int("alignment.tip_per_player"),
        Side::Discord => w.int("alignment.discord_tip_per_player"),
    };
    let full = per * seats;
    let every = w.int("alignment.momentum_every");
    if every <= 0 {
        return full;
    }
    (full - g.var(tips_key(side)) / every).max(seats)
}

/// How many of `thresholds` a tip count has reached.
fn reached(thresholds: &[i64], tips: i64) -> i64 {
    thresholds.iter().filter(|&&t| tips >= t).count() as i64
}

/// The clear-board bar level, from Unity tips (0: no adversary pieces
/// left; see `adversary::under_bar`).
pub fn bar_level(w: &World, g: &GameState) -> i64 {
    reached(w.list("alignment.unity_ease"), g.var(tips_key(Side::Unity)))
}

/// Moves `n` Sway toward `side`, one at a time: from the Undecided, or with
/// it empty (variant `tug`) from the opposite bucket. A bucket that fills
/// tips once the action or phase has resolved.
pub fn sway(g: &mut GameState, cx: &mut Cx, side: Side, n: i64) {
    if n <= 0 {
        return;
    }
    let variant = cx.world.variant("alignment");
    if variant == Some("off") {
        return;
    }
    let tug = variant == Some("tug");
    let mut moved = 0;
    for _ in 0..n {
        if g.var(UNDECIDED) > 0 {
            cx.add_key(g, UNDECIDED, -1);
        } else if tug && g.var(bucket(side.other())) > 0 {
            cx.add_key(g, bucket(side.other()), -1);
        } else {
            continue;
        }
        cx.add_key(g, bucket(side), 1);
        moved += 1;
    }
    if moved > 0 {
        cx.tally(g, &format!("alignment.sway_{}", side.name()), moved);
    }
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
    sway(g, cx, Side::Unity, base + bonus);
}

/// Sway toward Unity for a freed corrupted city.
pub fn for_city_freed(g: &mut GameState, cx: &mut Cx) {
    let n = cx.world.int("alignment.free_city");
    sway(g, cx, Side::Unity, n);
}

/// Sway toward Unity for a removed node.
pub fn for_node_removed(g: &mut GameState, cx: &mut Cx) {
    let n = cx.world.int("alignment.node");
    sway(g, cx, Side::Unity, n);
}

/// Whether the adversary's presence is under the bar for `level`. The
/// level-0 bar (no adversary pieces) counts only once a card of stage
/// `clear_win_stage` has been revealed (the stages are stacked and public).
fn purge_won(w: &World, g: &GameState, level: i64) -> bool {
    if level == 0 && g.var(adversary::STAGE) < w.int("alignment.clear_win_stage") {
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

/// The next `n` card purchases are free.
pub fn add_free_purchases(g: &mut GameState, cx: &mut Cx, n: i64) {
    cx.add_key(g, FREE_LEFT, n);
}

/// Up to `n` Sway leaves the Unity bucket for the Undecided (027).
pub fn pull_unity(g: &mut GameState, cx: &mut Cx, n: i64) {
    let k = n.min(g.var(bucket(Side::Unity))).max(0);
    if k > 0 {
        cx.add_key(g, bucket(Side::Unity), -k);
        cx.add_key(g, UNDECIDED, k);
        cx.tally(g, "alignment.unity_pulled", k);
    }
}

/// Seat `s` sacrifices one of their ambassadors (from their supply or a
/// hex) or one of their presence tokens, their choice; with none, nothing.
/// A last presence is never offered (027); a sacrificed presence leaves the
/// game, as a destroyed one does ([`board::lose_presence`]).
pub fn sacrifice(g: &mut GameState, cx: &mut Cx, s: usize) -> Res {
    let w = cx.world;
    let k = ambassador(s);
    let ambs = ambassador_hexes(g, s);
    // The hexes holding the seat's presence (020: this listed `presence_hexes`, the
    // free hexes where it could place one, so a sacrificed presence removed nothing).
    let pres: Vec<usize> = presence_of(g, s);
    let pres = if pres.len() < 2 { Vec::new() } else { pres };
    let labelled = |what: &str, h: usize| {
        let mut o = hex_opt(&w, h);
        o.option = format!("{what} at {}", o.option);
        o
    };
    let mut opts = Vec::new();
    if g.resource(s, Resource::Ambassadors) > 0 {
        opts.push(opt("an ambassador from your supply"));
    }
    let off = opts.len();
    opts.extend(ambs.iter().map(|&h| labelled("ambassador", h)));
    opts.extend(pres.iter().map(|&h| labelled("presence", h)));
    let prompt = format!("Seat {s}: sacrifice which ambassador or presence?");
    let Some(i) = cx.choose(s, &prompt, opts)? else {
        return Ok(());
    };
    if i < off {
        cx.add_resource(g, s, Resource::Ambassadors, -1);
        cx.tally(g, "alignment.sacrificed_ambassadors", 1);
    } else if i - off < ambs.len() {
        cx.add_tokens(g, ambs[i - off], &k, -1);
        cx.tally(g, "alignment.sacrificed_ambassadors", 1);
    } else {
        board::lose_presence(g, cx, pres[i - off - ambs.len()], s, "project");
        cx.tally(g, "alignment.sacrificed_presence", 1);
    }
    Ok(())
}

impl Alignment {
    /// Draws the next card of `side`: the stronger tier once the side's tip
    /// count has reached its first threshold, else tier 1; the other tier
    /// if that deck is empty. Neither deck ever reshuffles (027): they are
    /// the game's clocks.
    fn draw(&self, g: &GameState, w: &World, side: Side, tips: i64) -> Option<(Key, String)> {
        let first = match side {
            Side::Unity => w.list("alignment.unity_ease")[0],
            Side::Discord => w.int("alignment.discord_tier2"),
        };
        let want: u8 = if tips >= first { 2 } else { 1 };
        for tier in [want, 3 - want] {
            if let Some(id) = g.pile(&deck(side, tier)).first().cloned() {
                return Some((id, deck(side, tier)));
            }
        }
        None
    }

    /// A bucket tips: one card of its side resolves (at the strength step
    /// its side's tip count has reached), the side's tip count goes up by
    /// one, and the bucket's Sway returns to the Undecided.
    fn tip(&self, g: &mut GameState, cx: &mut Cx, side: Side) -> Res {
        let w = cx.world;
        let round = g.round;
        cx.note(None, || format!("The {} bucket tips", side.name()));
        let before = g.var(tips_key(side));
        let back = g.var(bucket(side));
        cx.set_key(g, bucket(side), 0);
        cx.add_key(g, UNDECIDED, back);
        cx.set_key(g, tips_key(side), before + 1);
        cx.tally(g, &format!("alignment.tips_{}", side.name()), 1);
        cx.tally(g, &format!("alignment.tip_{}.r{round:02}", side.name()), 1);
        let Some((id, from)) = self.draw(g, &w, side, before) else {
            // Discord tips with no card left: the players lose (locked, 027).
            if side == Side::Discord {
                cx.end(g, false, "discord_deck", "a Discord tip found the Discord deck empty".into());
            }
            return Ok(());
        };
        {
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
            effects::run(g, cx, effects, &mut effects::Ctx::team().named(source))?;
            cx.tally(g, &format!("alignment.card.{id}"), 1);
        }
        Ok(())
    }
}

/// Whether the antimatter action exists in this game (uses aside).
fn antimatter_on(w: &World) -> bool {
    w.int("alignment.antimatter_discord") > 0 && w.variant("alignment") != Some("off")
}

impl Module for Alignment {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        let start = w.int("alignment.undecided_per_player") * g.seats.len() as i64;
        cx.set_key(g, UNDECIDED, start);
        for side in [Side::Unity, Side::Discord] {
            cx.set_key(g, bucket(side), 0);
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
                // 028: the game deals a few of the printed Unity cards, so the
                // last one can be drawn.
                if side == Side::Unity {
                    let dealt = w.list("alignment.unity_cards");
                    ids.truncate(dealt[tier as usize - 1] as usize);
                }
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
        let w = cx.world;
        sway(g, cx, Side::Discord, w.int("alignment.adversary_discord"));
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
        sway(g, cx, Side::Discord, n);
        cx.count(g, "antimatter_actions", 1);
        Ok(())
    }

    fn after(&self, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        // Full buckets tip, Discord first; a tip's card can fill a bucket again.
        let tips = if w.variant("alignment") == Some("off") {
            0
        } else {
            64
        };
        'tips: for _ in 0..tips {
            for side in [Side::Discord, Side::Unity] {
                if g.var(bucket(side)) >= capacity(&w, g, side) {
                    self.tip(g, cx, side)?;
                    if g.is_over() {
                        return Ok(());
                    }
                    // 028: a card that takes a last presence loses here, before
                    // a Unity tip can win a game with no player on the board.
                    if w.int("alignment.presence_loss") > 0 && adversary::loss(g).is_some() {
                        break;
                    }
                    continue 'tips;
                }
            }
            break;
        }
        // Losses first.
        if w.int("alignment.presence_loss") > 0 {
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
                    g.var(tips_key(Side::Unity))
                ),
            );
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        let seats = g.seats.len() as i64;
        let total = w.int("alignment.undecided_per_player") * seats;
        out.global(
            "undecided",
            "the Undecided",
            g.var(UNDECIDED),
            Some(total),
            "Sway not yet in either bucket",
        );
        out.global(
            "unity",
            "Unity bucket",
            g.var(bucket(Side::Unity)),
            Some(capacity(w, g, Side::Unity)),
            "Sway toward the good AIs; tips into a Unity card when full",
        );
        out.global(
            "discord",
            "Discord bucket",
            g.var(bucket(Side::Discord)),
            Some(capacity(w, g, Side::Discord)),
            "Sway toward the evil AI; tips into a Discord card when full",
        );
        let next = |list: &[i64], tips: i64| list.iter().copied().find(|&t| t > tips);
        let ut = g.var(tips_key(Side::Unity));
        let dt = g.var(tips_key(Side::Discord));
        out.global(
            "unity_tips",
            "Unity tips",
            ut,
            next(w.list("alignment.unity_ease"), ut),
            "Unity tips so far; the clear-board bar drops at each Unity threshold",
        );
        let tier2 = w.int("alignment.discord_tier2");
        out.global(
            "discord_tips",
            "Discord tips",
            dt,
            (dt < tier2).then_some(tier2),
            "Discord tips so far; a Discord card's stronger steps count the tips before its own: \"after 3\" first fires on the 4th tip",
        );
        let level = bar_level(w, g);
        let mut text = adversary::win_text(level);
        let stage = w.int("alignment.clear_win_stage");
        if level == 0 && g.var(adversary::STAGE) < stage {
            text += &format!(" (from adversary stage {stage})");
        }
        out.global(
            "bar",
            "clear-board bar",
            level,
            Some(w.list("alignment.unity_ease").len() as i64),
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
        if g.var(INCOME_BONUS) > 0 {
            out.global(
                "income_bonus",
                "Unity income bonus",
                g.var(INCOME_BONUS),
                None,
                "credit income every player has from Unity cards",
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
                "free purchases",
                g.var(FREE_LEFT),
                None,
                "card purchases that are free",
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
        out.bookkeeping(&["alignment.used."]);
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

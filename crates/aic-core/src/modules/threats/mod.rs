//! Threats (135): dealt by the current adversary event card's pulls, one a
//! pull a country, each with location tokens on the board; countered by the
//! players near it (one option of its counter menu), executed in
//! `threats.execute` if still uncountered.
//! Variants: `countries` (each threat sits on a country) and `off` (no threats).

use crate::action::Action;
use crate::content::{
    CounterItem, Location, Op, Resource, TagOrAny, ThreatDef, ThreatSource, ThreatTag,
};
use crate::key::{Key, KeyBuf};
use crate::module::{Build, Cx, Halt, Module, ModuleDef, PhaseSpec, Res, VariantDef, World};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ThreatSourceView, ViewOut};

use super::common::*;
use super::{adversary, board, cards, effects};

mod counter;
mod near;
mod ops;

pub use counter::{
    cheapest_resource_need, need_and_share, need_and_share_with, paid_share, requirement, resource_only, standing_counters,
    CounterSteps, UNPAYABLE,
};
use counter::*;
pub use near::{counterable, first_token, here, hit_seat_of, near, near_hexes, near_seats, nearest_nodes, node_distance, token_of, tokens};
use near::*;
#[cfg(test)]
pub use near::mask_matches_reach;
pub use ops::*;

/// The auto phases by index in [`PHASES`] (198: named, `run` reads them; the counter phase, 1, is a decision).
const DEAL: usize = 0;
const EXECUTE: usize = 2;

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto(
        "deal",
        "deal the current adversary card's pulls: one threat a pull in each country, its tokens on the board",
    ),
    PhaseSpec::decision(
        "counter",
        "players near a threat pay one option of its counter menu (resources pooled; withdraw until the phase ends)",
    ),
    PhaseSpec::auto(
        "execute",
        "refund partial payments, then uncountered threats execute in order (one dealt after the counter phase waits a round)",
    ),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "threats",
    summary: "Threats: dealt by the adversary's event cards, countered by the players near them, executed if not",
    requires: &["adversary", "alignment"],
    variants: &[
        VariantDef {
            name: "countries",
            summary: "Each threat sits on one country with location tokens; only players near a token may counter it",
            params: &[],
            phases: PHASES,
            rng: &["threats"],
            uses_cards: false,
            build: build_on,
        },
        VariantDef {
            name: "off",
            summary: "No threats: nothing is dealt, countered or executed (cards that act on threats have no target)",
            params: &[],
            phases: PHASES,
            rng: &[],
            uses_cards: false,
            build: build_off,
        },
    ],
};

/// Variant `off`: the threat phases stay in the round and do nothing.
#[derive(Debug)]
struct NoThreats;

impl Module for NoThreats {}

fn build_off(_: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(NoThreats))
}

/// The tiers threat decks come in (content checks every printed tier against them).
pub use crate::content::THREAT_TIERS as TIERS;

fn deck(tier: u8) -> KeyBuf {
    KeyBuf::new("threats.deck.").num(tier as usize)
}

fn discard(tier: u8) -> KeyBuf {
    KeyBuf::new("threats.discard.").num(tier as usize)
}

fn on(country: usize) -> KeyBuf {
    KeyBuf::new("threats.on.").num(country)
}

/// Prefix of every payment toward `threat` (by any seat, in any resource).
pub fn paid_prefix(threat: &str) -> KeyBuf {
    KeyBuf::new("threats.paid.").text(threat).text(".")
}

/// What seat `seat` has paid toward `threat` in `resource`.
pub fn paid_key(threat: &str, seat: usize, resource: Resource) -> KeyBuf {
    paid_prefix(threat).num(seat).text(".").text(resource.name())
}

fn jam_key(threat: &str) -> KeyBuf {
    KeyBuf::new("threats.jam.").text(threat)
}

/// The round a threat was dealt in, if it came after that round's counter
/// phase (it waits and executes next round, 022).
fn late_key(threat: &str) -> KeyBuf {
    KeyBuf::new("threats.late.").text(threat)
}

/// Whether the counter phase has passed this round.
fn after_counter(g: &GameState) -> bool {
    g.phase_index("threats.counter").is_some_and(|i| g.phase > i)
}

/// Whether `threat` was dealt this round after the counter phase: it skips
/// this round's execution.
pub fn waits(g: &GameState, threat: &str) -> bool {
    g.var(&late_key(threat)) == g.round as i64
}

/// `threat` waits: it skips this round's execution and stays up, to be
/// countered next round (041).
pub fn delay(g: &mut GameState, cx: &mut Cx, threat: &str) {
    cx.set_key(g, &late_key(threat), g.round as i64);
    cx.tally(g, "threats.delayed", 1);
}

fn def<'w>(w: &World<'w>, id: &str) -> &'w ThreatDef {
    w.content.threat(id)
}

/// Active threats in execution order (country by country, each in the order
/// dealt), each with its country.
pub fn active(g: &GameState) -> Vec<(Key, usize)> {
    let mut on: Vec<(usize, &std::sync::Arc<Vec<Key>>)> = g
        .piles
        .from_key("threats.on.")
        .map_while(|(k, v)| Some((k.strip_prefix("threats.on.")?, v)))
        .filter_map(|(k, v)| Some((k.parse().ok()?, v)))
        .collect();
    on.sort_by_key(|(c, _)| *c);
    on.into_iter().flat_map(|(c, pile)| pile.iter().map(move |t| (*t, c))).collect()
}

/// The country active `threat` sits on, `None` if it isn't face up.
fn scope_of(g: &GameState, threat: &str) -> Option<usize> {
    g.piles
        .from_key("threats.on.")
        .map_while(|(k, v)| Some((k.strip_prefix("threats.on.")?, v)))
        .find(|(_, v)| v.iter().any(|t| t == threat))
        .and_then(|(k, _)| k.parse().ok())
}

/// Whether threats are in play (not variant `off`).
fn in_play(w: &World) -> bool {
    w.variant("threats") == Some("countries")
}

/// Refills an empty `tier` deck from its shuffled discard. False if both are empty.
fn refill(g: &mut GameState, cx: &mut Cx, tier: u8) -> bool {
    if !g.pile(&deck(tier)).is_empty() {
        return true;
    }
    let mut cards = g.pile(&discard(tier)).to_vec();
    if cards.is_empty() {
        return false;
    }
    shuffle(&mut cards, cx.threat_rng(g));
    cx.set_pile(g, &discard(tier), Vec::new());
    cx.set_pile(g, &deck(tier), cards);
    true
}

/// One pull into country `c` (292: no card is ever rejected): the top card of `tier`'s deck is dealt, an empty deck
/// reshuffling its discard first. Its tokens go on the board (`at`, 139: its first token on that hex instead); with
/// no site of its types left in `c`, its one token goes on the country's fallback city (`near::place_fallback`). One
/// dealt after the counter phase waits a round.
fn pull(g: &mut GameState, cx: &mut Cx, tier: u8, c: usize, at: Option<usize>) {
    let w = cx.world;
    if !refill(g, cx, tier) {
        cx.tally(g, "threats.short", 1);
        return;
    }
    let id = g.pile(&deck(tier))[0];
    // Every card turned up is dealt: a read's rejected cards are `threats.turned` less `threats.dealt`.
    cx.tally(g, "threats.turned", 1);
    cx.move_card(g, &id, &deck(tier), &on(c), false).unwrap();
    place_tokens(g, cx, &id, c);
    if at.is_none() && near::first_token(g, &id).is_none() {
        near::place_fallback(g, cx, &id, c);
        cx.tally(g, "threats.dealt_fallback", 1);
    }
    if let Some(h) = at {
        near::place_first_at(g, cx, &id, h);
        cx.tally(g, "threats.dealt_at_node", 1);
    }
    cx.tally(g, "threats.dealt", 1);
    cx.tally(g, &format!("threats.dealt.t{}", def(&w, &id).tier), 1);
    // 286: which threat, so a read can see how often each is dealt beside how often it corrupts.
    cx.tally(g, &format!("threats.dealt.id.{}", def(&w, &id).id), 1);
    // 285: the turn panel read a deal as "cards changed": name the threat and where it faces.
    let anchor = near::first_token(g, &id);
    cx.note(anchor, || {
        format!("Threat {} is dealt to {}{}", def(&w, &id).name, country_name(&w, c), anchor.map(|h| format!(", at {}", hex_name(&w, h))).unwrap_or_default())
    });
    if after_counter(g) {
        cx.set_key(g, &late_key(&id), g.round as i64);
    }
    // 118: a threat a standing counter cuts to nothing is countered as it lands.
    try_complete(g, cx, &id);
}

/// One pull of `tier` threats into `country` now (a threat dealing a threat; it waits a round if the counter phase
/// has passed). Variant `off`: nothing.
pub fn deal_tier(g: &mut GameState, cx: &mut Cx, tier: u8, country: usize) -> Res {
    if in_play(&cx.world) && TIERS.contains(&tier) {
        pull(g, cx, tier, country, None);
    }
    Ok(())
}

/// 139 (Offensive): one pull of `tier` threats into hex `h`'s country now, its first location token on
/// `h` (a node hex, whatever its site). Variant `off`: nothing.
pub fn deal_at(g: &mut GameState, cx: &mut Cx, tier: u8, h: usize) -> Res {
    let w = cx.world;
    if in_play(&w) && TIERS.contains(&tier) {
        pull(g, cx, tier, w.map.hexes[h].country as usize, Some(h));
    }
    Ok(())
}

/// 178 (Matt's ruling #8): what every threat card prints above its text: who may pay toward its counter, and who
/// its harm hits (292: the player of the country it was dealt in).
pub const THREAT_FRAME: &str =
    "Payers: any player with a Node or Ambassador on or next to a token. Target: this country's player.";

/// The current adversary event card's first pull's tier.
fn first_pull(w: &World, g: &GameState) -> Option<u8> {
    // 145: this round's card, the face-up one before the flip (a Discord tip in
    // the Sway phase read last round's card).
    let id = g.pile(adversary::round_pile(g)).first().copied()?;
    Some(w.content.adversary_card(&id).pulls.first()?.tier)
}

/// Deals `n` extra threats now (old-form `DealThreats`, Discord cards): pulls
/// of the current event card's first pull's tier, one per country in
/// turn from country 0. Variant `off`: nothing.
pub fn deal_extra(g: &mut GameState, cx: &mut Cx, n: i64) {
    let w = cx.world;
    if !in_play(&w) {
        return;
    }
    let tier = first_pull(&w, g).unwrap_or_else(|| adversary::threat_tier(g));
    let countries = w.map.countries.len().max(1);
    for k in 0..n.max(0) {
        pull(g, cx, tier, k as usize % countries, None);
    }
}

/// 118: counters every face-up threat that now needs nothing (a standing counter
/// came into play).
pub fn complete_paid(g: &mut GameState, cx: &mut Cx) {
    for (t, _) in active(g) {
        try_complete(g, cx, &t);
    }
}

/// `threat` leaves the board: its tokens off, to its tier's discard.
fn clear(g: &mut GameState, cx: &mut Cx, threat: &str) {
    let w = cx.world;
    let Some(c) = scope_of(g, threat) else {
        return;
    };
    cx.set_key(g, &jam_key(threat), 0);
    cx.set_key(g, &late_key(threat), 0);
    let key = Key::new(threat);
    clear_tokens(g, cx, &key);
    cx.move_card(g, threat, &on(c), &discard(def(&w, threat).tier), true).expect("active threat");
}

/// Counters `threat` without payment (a card or a reward): partial
/// payments are refunded and it is discarded.
pub fn counter(g: &mut GameState, cx: &mut Cx, threat: &str) {
    if scope_of(g, threat).is_none() {
        return;
    }
    refund(g, cx, threat);
    tally_countered(g, cx, threat, "card");
    clear(g, cx, threat);
}

/// Jam Signals: `threat` needs `n` less of each resource.
pub fn jam(g: &mut GameState, cx: &mut Cx, threat: &str, n: i64) {
    cx.add_key(g, &jam_key(threat), n);
    try_complete(g, cx, threat);
}

/// The tally label of a tier's threats (what they destroy is counted under it).
fn label(tier: u8) -> &'static str {
    // 146: no tier 3.
    if tier == 1 { "threat1" } else { "threat2" }
}

#[derive(Debug)]
struct Threats;

fn build_on(_: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Threats))
}

/// Phase `deal`: the current adversary card's pulls, one threat a pull in each country.
fn deal_round(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let Some(id) = g.pile(adversary::CURRENT).first().copied() else {
        return;
    };
    let card = w.content.adversary_card(&id);
    let n = w.map.countries.len();
    for p in &card.pulls {
        for c in 0..n {
            pull(g, cx, p.tier, c, None);
        }
    }
}

impl Module for Threats {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        // 292 (Matt, 2026-11-19): the decks are built once, here, so a round's dealt card is simply the next one. The
        // adversary board's lean (`threat_deck`): every threat of the tier carrying its tag (Universal counts) and
        // its printed `others` of the rest for each of those, taken at random; what is left stays in the box (in no
        // pile).
        let board = w.content.adversary_board.threat_deck.as_ref();
        let lean = board.map(|d| d.tag);
        for tier in TIERS {
            let of_tier = || w.content.threats.iter().filter(|t| t.tier == tier);
            let leans = |t: &&ThreatDef| lean.is_none_or(|tag| t.tags.iter().any(|&x| x == tag || x == ThreatTag::Universal));
            let mut ids: Vec<Key> = of_tier().filter(leans).map(|t| Key::new(&t.id)).collect();
            let mut others: Vec<Key> = of_tier().filter(|t| !leans(t)).map(|t| Key::new(&t.id)).collect();
            shuffle(&mut others, cx.rng(g, "threats"));
            others.truncate(ids.len() * board.map_or(0, |d| d.others));
            ids.extend(others);
            shuffle(&mut ids, cx.rng(g, "threats"));
            cx.set_pile(g, &deck(tier), ids);
        }
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        match phase {
            DEAL => {
                deal_round(g, cx);
                return Ok(());
            }
            EXECUTE => {}
            _ => unreachable!("threats: no auto phase {phase}"),
        }
        let w = cx.world;
        // Execute: anything fully paid is countered, partial payments come
        // back, then each threat in order. A threat dealt after this round's
        // counter phase waits a round (022).
        for (t, _) in active(g) {
            if !try_complete(g, cx, &t) {
                refund(g, cx, &t);
            }
        }
        let mut executed = 0;
        for (t, scope) in active(g) {
            if waits(g, &t) {
                // 145: a waiting threat is still uncountered: not a clean round.
                executed += 1;
                continue;
            }
            executed += 1;
            let d = def(&w, &t);
            cx.note(None, || format!("Threat {} executes in {}", d.name, country_name(&w, scope)));
            cx.count(g, "executed", 1);
            cx.tally(g, "threats.uncountered", 1);
            cx.per_round(g, "threats.uncountered", 1);
            // 292: its harm hits one player, the one whose home is the country it was dealt in (146-r138: the seat
            // whose node was nearest a token, at any distance, the players picking on a tie).
            let hit = hit_seat_of(&w, g, scope);
            if let Some(s) = hit {
                cx.tally(g, &format!("threats.hit.s{s}"), 1);
            }
            let mut ctx = effects::Ctx::threat(Some(scope)).named(label(d.tier)).why(&d.name).with_threat(t).hitting(hit);
            cx.tally(g, &format!("threats.uncountered.t{}", d.tier), 1);
            // 147: by stage too (see `tally_countered`).
            let stage = g.var(adversary::STAGE);
            cx.tally(g, &format!("threats.uncountered.s{stage}.t{}", d.tier), 1);
            // 146: which threat, so a read can see each harm type land.
            cx.tally(g, &format!("threats.uncountered.id.{}", d.id), 1);
            // 286 (D130): sites the threat's own execution corrupts (its lines and its anchor agent; a node loss to
            // agents is no site).
            let sites = |g: &GameState| {
                let c = |k: &str| g.stats.counters.get(k).copied().unwrap_or(0);
                c("adversary.corruptions") - c("adversary.corruptions.node")
            };
            let before = sites(g);
            effects::run(g, cx, &d.effects, &mut ctx)?;
            // 275: an uncountered threat also lands `threat_agents` agents on its anchor (its first location token),
            // an arrival like any other (checked there); a cordoned country takes none.
            let n = w.int("adversary.threat_agents");
            let cordoned = board::flagged(g, crate::content::RoundFlag::Cordon, scope);
            if let (Some(&h), true, false, false) = (near::tokens(g, &t).first(), n > 0, g.is_over(), cordoned) {
                // 285: named in the log (the turn panel showed "+1 agent" with no hex).
                let name = &d.name;
                adversary::place_noted(g, cx, h, n, &|| format!("{name}, uncountered, at its anchor"))?;
                cx.tally(g, "adversary.threat_anchor", n);
            }
            let took = sites(g) - before;
            if took > 0 {
                cx.tally(g, &format!("threats.corrupted.id.{}", d.id), took);
            }
            // 146: the adversary board's printed compute per uncountered threat (its
            // difficulty line's from `board_difficulty_at`).
            let k = adversary::threat_compute(&w);
            if k > 0 {
                adversary::gain_compute(g, cx, k)?;
            }
            clear(g, cx, &t);
        }
        // 070: the records' read of whether a board can stay clean.
        if executed == 0 {
            cx.tally(g, "threats.rounds_all_countered", 1);
        }
        Ok(())
    }

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        let s = seat as usize;
        for (t, _) in active(g) {
            // 044: a seat no longer near can't pay, but can take back.
            let can_pay = near(w, g, s, &t);
            let req = requirement(w, g, &t);
            for r in menu_resources(w, &t) {
                let most = if can_pay { g.resource(s, r).min(room(&req, g, &t, r)) } else { 0 };
                let amounts: &[i64] = match most {
                    ..=0 => &[],
                    1 => &[1],
                    _ => &[1, most],
                };
                for &amount in amounts {
                    out.push(Action::Pay { seat, threat: t, resource: r, amount });
                }
                if g.var(&paid_key(&t, s, r)) > 0 {
                    out.push(Action::Withdraw { seat, threat: t, resource: r });
                }
            }
            if can_pay {
                for (k, option) in req.iter().enumerate() {
                    if !resource_only(option) && can_pay_whole(w, g, s, &t, option) {
                        out.push(Action::CounterWith { seat, threat: t, option: k as u8 });
                    }
                }
            }
        }
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Pay { .. } | Action::Withdraw { .. } | Action::CounterWith { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let s = a.seat() as usize;
        match a {
            Action::Pay { threat, resource, amount, .. } => {
                let Some(c) = scope_of(g, threat) else {
                    return Err(Halt::Illegal(format!("no active threat {threat}")));
                };
                if !near(&w, g, s, threat) {
                    return Err("only a player near the threat can pay".into());
                }
                let req = requirement(&w, g, threat);
                if !menu_resources(&w, threat).contains(resource) {
                    return Err("that resource is not part of its counter".into());
                }
                if *amount < 1 || *amount > g.resource(s, *resource) || *amount > room(&req, g, threat, *resource) {
                    return Err("can't pay that much".into());
                }
                if c != home_country(&w, s) {
                    cx.tally(g, "threats.paid_abroad", 1);
                }
                cx.add_resource(g, s, *resource, -amount);
                cx.add_key(g, &paid_key(threat, s, *resource), *amount);
                try_complete(g, cx, threat);
                Ok(())
            }
            Action::Withdraw { threat, resource, .. } => {
                let k = paid_key(threat, s, *resource);
                let n = g.var(&k);
                if n == 0 {
                    return Err("nothing to withdraw".into());
                }
                cx.set_key(g, &k, 0);
                cx.add_resource(g, s, *resource, n);
                Ok(())
            }
            Action::CounterWith { threat, option, .. } => counter_with(g, cx, s, threat, *option as usize),
            _ => Err(Halt::Illegal(format!("not a threat action: {a:?}"))),
        }
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        // Payments, jams and tokens belong to a threat card: shown in its text
        // and its source view.
        out.bookkeeping(&["threats.paid.", "threats.jam.", "threats.late.", "threats.loc."]);
        let show = |t: &Key| {
            let d = def(w, t);
            let options: Vec<String> = requirement(w, g, t)
                .iter()
                .map(|o| o.iter().map(|i| item_words(g, t, i)).collect::<Vec<_>>().join(" + "))
                .collect();
            let need = options.join(" or ");
            let mut extra = String::new();
            let payers: Vec<String> = (0..g.seats.len())
                .flat_map(|s| {
                    menu_resources(w, t).into_iter().filter_map(move |r| {
                        let n = g.var(&paid_key(t, s, r));
                        (n > 0).then(|| format!("seat {s} {}", r.words(n)))
                    })
                })
                .collect();
            if !payers.is_empty() {
                extra.push_str(&format!(" Paid: {}.", payers.join(", ")));
            }
            let jam = g.var(&jam_key(t));
            if jam > 0 {
                extra.push_str(&format!(" Jammed: needs {jam} less of each resource."));
            }
            if waits(g, t) {
                extra.push_str(" Waits token: executes next round.");
            }
            CardView {
                id: t.to_string(),
                name: d.name.clone(),
                // 178 (Matt's ruling #8): the card names its payers and its target apart.
                text: format!("{THREAT_FRAME} {} Counter: {need}.{extra}", d.text),
                status: need,
                cost: None,
            }
        };
        for c in 0..w.map.countries.len() {
            let cards: Vec<CardView> = g.pile(&on(c)).iter().map(show).collect();
            if !cards.is_empty() {
                out.country_cards(c, "on", "threat", cards);
            }
            for t in g.pile(&on(c)).iter() {
                let tags: Vec<&str> = def(w, t).tags.iter().map(|x| x.name()).collect();
                out.threat_source(ThreatSourceView {
                    threat: t.to_string(),
                    source: tags.join("+"),
                    hexes: tokens(g, t),
                    target: None,
                    seats: near_seats(w, g, t),
                });
            }
        }
        out.global(
            "tier",
            "threat tier",
            // 145: as `deal_extra` picks it (it said the card's stage).
            first_pull(w, g).unwrap_or_else(|| adversary::threat_tier(g)) as i64,
            // 184: two tiers since 146 ("threat tier 2/3" pointed at a third).
            Some(2),
            "the tier a Discord card's extra threat comes from: this round's adversary card's first pull",
        );
    }
}

/// One counter item in words, with what is pooled on it so far.
fn item_words(g: &GameState, t: &str, i: &CounterItem) -> String {
    match *i {
        CounterItem::Pay { resource, amount } => format!("{}/{}", counter::paid(g, t, resource), resource.words(amount)),
        CounterItem::Sacrifice { sacrifice, amount } => format!("sacrifice {amount} {sacrifice} tags"),
        CounterItem::NodesNear { nodes_near } => format!("remove {nodes_near} of your nodes near it"),
    }
}

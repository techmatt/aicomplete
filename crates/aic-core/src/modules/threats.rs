//! Threats: dealt each round, countered by pooled payments, executed in
//! `threats.execute`, after production and upkeep, if still uncountered.
//! Variants: `countries`
//! (each threat sits on a country), `shared_row` (one row for every
//! country) and `off` (no threats).

use crate::action::Action;
use crate::content::{Op, Resource, Spot, ThreatDef, ThreatSource};
use crate::key::{Key, KeyBuf};
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::rng::shuffle;
use crate::state::GameState;
use crate::view::{CardView, ThreatSourceView, ViewOut};

use super::common::*;
use super::{adversary, alignment, board, cards, effects};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::int("per_country", 0, 5, "threats dealt each round per country"),
    ParamSpec::int(
        "uncreated",
        0,
        4,
        "a screen's switch: this source type makes nothing when uncountered (1 influence, 2 cyber, 3 economic, 4 military; 0 none)",
    ),
];

const PHASES: &[PhaseSpec] = &[
    PhaseSpec::auto(
        "deal",
        "deal threats face up (tier 1 until stage II begins, then tier 2)",
    ),
    PhaseSpec::decision(
        "counter",
        "pay toward threat counters (pooled; withdraw until the phase ends)",
    ),
    PhaseSpec::auto(
        "execute",
        "refund partial payments, then uncountered threats execute in order (one dealt after the counter phase waits a round)",
    ),
];

pub static DEF: ModuleDef = ModuleDef {
    id: "threats",
    summary: "Threats: dealt each round, countered by pooled payments, executed if not",
    requires: &["adversary", "alignment"],
    variants: &[
        VariantDef {
            name: "countries",
            summary: "Each threat sits on one country; only players with one of its sources in reach, or an ambassador on one, may pay",
            params: PARAMS,
            phases: PHASES,
            rng: &["threats"],
            uses_cards: false,
            build: build_countries,
        },
        VariantDef {
            name: "shared_row",
            summary: "Threats sit in one row, apply in every country, and anyone may pay",
            params: PARAMS,
            phases: PHASES,
            rng: &["threats"],
            uses_cards: false,
            build: build_shared,
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

const ROW: &str = "threats.row";

fn deck(tier: u8) -> String {
    format!("threats.deck.{tier}")
}

fn discard(tier: u8) -> String {
    format!("threats.discard.{tier}")
}

fn on(country: usize) -> String {
    format!("threats.on.{country}")
}

/// Prefix of every payment toward `threat` (by any seat, in any resource).
pub fn paid_prefix(threat: &str) -> KeyBuf {
    KeyBuf::new("threats.paid.").text(threat).text(".")
}

fn paid_key(threat: &str, seat: usize, resource: Resource) -> KeyBuf {
    KeyBuf::new("threats.paid.").text(threat).text(".").num(seat).text(".").text(resource.name())
}

/// The hex plus 1 a threat dealt with no source named as its target (070).
fn target_key(threat: &str) -> KeyBuf {
    KeyBuf::new("threats.target.").text(threat)
}

/// The last round this country's threats made a source (one a round).
fn made_key(c: usize) -> KeyBuf {
    KeyBuf::new("threats.made.").num(c)
}

fn jam_key(threat: &str) -> KeyBuf {
    KeyBuf::new("threats.jam.").text(threat)
}

/// The round a threat was dealt in, if it came after that round's counter
/// phase (it waits beside the row and executes next round, 022).
fn late_key(threat: &str) -> KeyBuf {
    KeyBuf::new("threats.late.").text(threat)
}

/// Whether the counter phase has passed this round (the current phase is
/// after the round's `threats.counter`).
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
    &w.content.threats[w.content.threat_index(id).expect("threat in content")]
}

/// Active threats in execution order, each with its scope (its country, or
/// `None` for a shared-row threat).
pub fn active(g: &GameState) -> Vec<(Key, Option<usize>)> {
    let mut out: Vec<(Key, Option<usize>)> =
        g.pile(ROW).iter().map(|t| (t.clone(), None)).collect();
    let mut on: Vec<(usize, &std::sync::Arc<Vec<Key>>)> = g
        .piles
        .from_key("threats.on.")
        .map_while(|(k, v)| Some((k.strip_prefix("threats.on.")?, v)))
        .filter_map(|(k, v)| Some((k.parse().ok()?, v)))
        .collect();
    on.sort_by_key(|(c, _)| *c);
    for (c, pile) in on {
        out.extend(pile.iter().map(|t| (t.clone(), Some(c))));
    }
    out
}

fn pile_of(scope: Option<usize>) -> String {
    scope.map_or(ROW.to_string(), on)
}

/// Where a threat dealt for country `c` goes: the shared row, or on `c`.
fn deal_pile(shared: bool, c: usize) -> String {
    if shared {
        ROW.to_string()
    } else {
        on(c)
    }
}

/// Whether hex `h` holds an adversary piece of type `source` (070).
pub fn is_source(w: &World, g: &GameState, h: usize, source: ThreatSource) -> bool {
    match source {
        ThreatSource::Influence => corrupted_city(w, g, h),
        ThreatSource::Cyber => g.tokens(h, NODE) > 0 || (g.tokens(h, STATION) > 0 && corrupted(g, h)),
        ThreatSource::Economic => g.tokens(h, FACTORY) > 0 && corrupted(g, h),
        ThreatSource::Military => g.tokens(h, TROOP) > 0,
    }
}

/// The sources of type `source` in country `c`, in hex order.
pub fn sources(w: &World, g: &GameState, c: usize, source: ThreatSource) -> Vec<usize> {
    w.map.countries[c].hexes.iter().copied().filter(|&h| is_source(w, g, h, source)).collect()
}

/// The source type `threat` prints.
pub fn source_of(w: &World, threat: &str) -> ThreatSource {
    def(w, threat).source
}

/// The hex `threat` named as its target when dealt with no source, if any.
pub fn target(g: &GameState, threat: &str) -> Option<usize> {
    let v = g.var(&target_key(threat));
    (v > 0).then(|| v as usize - 1)
}

/// What `threat` in country `c` counts as its sources: the adversary's pieces
/// of its type there, and the target it named.
pub fn threat_sources(w: &World, g: &GameState, threat: &str, c: usize) -> Vec<usize> {
    let mut hexes = sources(w, g, c, source_of(w, threat));
    if let Some(h) = target(g, threat) {
        if !hexes.contains(&h) {
            hexes.push(h);
        }
    }
    hexes
}

/// Whether seat `s` may counter `threat` in `scope`: one of its sources in
/// reach, or its ambassador on one (070). A shared-row threat is anyone's.
pub fn may_counter(w: &World, g: &GameState, s: usize, threat: &str, scope: Option<usize>) -> bool {
    let Some(c) = scope else {
        return true;
    };
    let k = ambassador(s);
    let open = |h: usize| g.tokens(h, &k) > 0 || in_reach(w, g, s, h);
    let source = source_of(w, threat);
    target(g, threat).is_some_and(open)
        || w.map.countries[c].hexes.iter().any(|&h| is_source(w, g, h, source) && open(h))
}

/// The active threats seat `s` may counter, of type `source` if given.
pub fn counterable(w: &World, g: &GameState, s: usize, source: Option<ThreatSource>) -> Vec<Key> {
    active(g)
        .into_iter()
        .filter(|(t, scope)| {
            source.is_none_or(|x| x == source_of(w, t)) && may_counter(w, g, s, t, *scope)
        })
        .map(|(t, _)| t)
        .collect()
}

/// An empty `tier` deck is refilled from its shuffled discard (070: extra
/// deals out of turn too: that is what made `threats.short`, not the deck's size).
fn refill(g: &mut GameState, cx: &mut Cx, tier: u8) {
    if g.pile(&deck(tier)).is_empty() {
        let mut cards = g.pile(&discard(tier)).to_vec();
        if cards.is_empty() {
            return;
        }
        shuffle(&mut cards, cx.threat_rng(g));
        cx.set_pile(g, &discard(tier), Vec::new());
        cx.set_pile(g, &deck(tier), cards);
    }
}

/// Whether a face-up threat of `source` in `h`'s country holds the adversary's
/// pieces of that type there (070: they can't be destroyed or freed until it
/// is countered). A shared-row threat has no country and holds nothing.
pub fn shielded(w: &World, g: &GameState, h: usize, source: ThreatSource) -> bool {
    g.pile(&on(w.map.hexes[h].country as usize)).iter().any(|t| def(w, t).source == source)
}

/// What a threat with no source in country `c` names (070): the lowest clean
/// piece of its type, neutral before player-owned; for a factory or station
/// with none to take, the lowest site one would be built on; for troops the
/// lowest base a Wall leaves open, else the lowest base. `None`: nothing to name.
fn pick_target(w: &World, g: &GameState, c: usize, source: ThreatSource) -> Option<usize> {
    let hexes = &w.map.countries[c].hexes;
    let lowest = |f: &dyn Fn(usize) -> bool| hexes.iter().copied().filter(|&h| f(h)).min();
    let clean = |kind: &str| {
        let on = |h: usize| g.tokens(h, kind) > 0 && !corrupted(g, h);
        lowest(&|h| on(h) && cube_owner(g, h).is_none()).or_else(|| lowest(&on))
    };
    match source {
        ThreatSource::Influence => lowest(&|h| clean_city(w, g, h)).or_else(|| lowest(&|h| collecting_city(w, g, h))),
        ThreatSource::Cyber => clean(STATION).or_else(|| lowest(&|h| buildable(w, g, h) && !w.map.hexes[h].industrial)),
        ThreatSource::Economic => clean(FACTORY).or_else(|| lowest(&|h| w.map.hexes[h].industrial && !has_building(g, h))),
        ThreatSource::Military => lowest(&|h| is_base(w, h) && !board::walled(w, g, h)).or_else(|| lowest(&|h| is_base(w, h))),
    }
}

/// An uncountered threat's target becomes its source, before its own effect:
/// the city or piece is corrupted, built first if it must be, or the troop
/// lands. One a country a round; a Wall stops the landing.
fn make_source(g: &mut GameState, cx: &mut Cx, c: usize, source: ThreatSource, h: usize) {
    let w = cx.world;
    let switched_off = match w.int("threats.uncreated") {
        1 => source == ThreatSource::Influence,
        2 => source == ThreatSource::Cyber,
        3 => source == ThreatSource::Economic,
        4 => source == ThreatSource::Military,
        _ => false,
    };
    if switched_off || g.var(&made_key(c)) == g.round as i64 {
        return;
    }
    match source {
        ThreatSource::Influence => {
            if corrupted(g, h) {
                return;
            }
            adversary::corrupt_city(g, cx, h);
        }
        ThreatSource::Cyber => {
            if g.tokens(h, STATION) == 0 {
                if !buildable(&w, g, h) {
                    return;
                }
                cx.add_tokens(g, h, STATION, 1);
            }
            if corrupted(g, h) {
                return;
            }
            adversary::corrupt_station(g, cx, h);
        }
        ThreatSource::Economic => {
            if g.tokens(h, FACTORY) == 0 {
                if has_building(g, h) {
                    return;
                }
                cx.add_tokens(g, h, FACTORY, 1);
            }
            if corrupted(g, h) {
                return;
            }
            adversary::corrupt_factory(g, cx, h);
        }
        ThreatSource::Military => {
            if board::walled(&w, g, h) {
                return;
            }
            adversary::place_troop(g, cx, h);
        }
    }
    cx.set_key(g, &made_key(c), g.round as i64);
    cx.tally(g, "threats.sources_made", 1);
    cx.note(None, || format!("{} source appears in {}", source.a_word(), country_name(&w, c)));
}

/// Deals the top threat of the `tier` deck onto `to` (`country`: the country
/// it needs a source in, `None` on the shared row), counted under
/// `threats.dealt` whichever module is running. False if the deck is empty
/// (no reshuffle here: the caller refills first if it may, and counts what
/// it could not deal under `threats.short`). A threat with no source in its
/// country names a target (070): its source while it is up. A threat dealt after the counter phase is stamped with the round:
/// it executes next round, not this one.
fn deal(g: &mut GameState, cx: &mut Cx, tier: u8, to: &str, country: Option<usize>) -> bool {
    let Some(id) = g.pile(&deck(tier)).first().cloned() else {
        return false;
    };
    let w = cx.world;
    if let Some(c) = country {
        let source = def(&w, &id).source;
        if sources(&w, g, c, source).is_empty() {
            if let Some(h) = pick_target(&w, g, c, source) {
                cx.set_key(g, &target_key(&id), h as i64 + 1);
            }
        }
    }
    cx.move_card(g, &id, &deck(tier), to, false).unwrap();
    cx.tally(g, "threats.dealt", 1);
    if after_counter(g) {
        cx.set_key(g, &late_key(&id), g.round as i64);
    }
    true
}

/// Whether the threats variant in play deals to the shared row (`None`:
/// no threats). For rules code outside the module, which has no `Threats`.
fn shared_row(w: &World) -> Option<bool> {
    match w.variant("threats") {
        Some("countries") => Some(false),
        Some("shared_row") => Some(true),
        _ => None,
    }
}

/// What countering `threat` asks for now, resource by resource (difficulty
/// and Jam Signals applied).
pub fn requirement(w: &World, g: &GameState, threat: &str) -> Vec<(Resource, i64)> {
    let jam = g.var(&jam_key(threat)) + standing(w, g, threat);
    let (bonus, all) = if adversary::level(w, 1) {
        (
            w.int("adversary.counter_bonus"),
            w.int("adversary.counter_bonus_all"),
        )
    } else {
        (0, 0)
    };
    def(w, threat)
        .counter
        .iter()
        .enumerate()
        .map(|(k, (r, n))| {
            let n = n + if k == 0 { bonus } else { 0 } + all - jam;
            (*r, n.max(0))
        })
        .collect()
}

/// Paid so far toward `threat` in `resource`, by all seats.
fn paid(g: &GameState, threat: &str, resource: Resource) -> i64 {
    (0..g.seats.len())
        .map(|s| g.var(&paid_key(threat, s, resource)))
        .sum()
}

/// Returns every contribution on `threat` to its payers.
fn refund(g: &mut GameState, cx: &mut Cx, threat: &str) {
    let w = cx.world;
    for (r, _) in &def(&w, threat).counter {
        for s in 0..g.seats.len() {
            let k = paid_key(threat, s, *r);
            let n = g.var(&k);
            if n > 0 {
                cx.add_resource(g, s, *r, n);
                cx.set_key(g, &k, 0);
            }
        }
    }
}

fn clear(g: &mut GameState, cx: &mut Cx, threat: &str, scope: Option<usize>) {
    let w = cx.world;
    let t = def(&w, threat);
    cx.set_key(g, &jam_key(threat), 0);
    cx.set_key(g, &late_key(threat), 0);
    cx.set_key(g, &target_key(threat), 0);
    cx.move_card(g, threat, &pile_of(scope), &discard(t.tier), true)
        .expect("active threat");
}

/// Counters `threat` without payment (a card or a reward): partial
/// payments are refunded and it is discarded.
pub fn counter(g: &mut GameState, cx: &mut Cx, threat: &str) {
    let Some((_, scope)) = active(g).into_iter().find(|(t, _)| t == threat) else {
        return;
    };
    refund(g, cx, threat);
    clear(g, cx, threat, scope);
    tally_countered(g, cx, threat);
}

/// Counted in all and by source type (070: the records read who counters what).
fn tally_countered(g: &mut GameState, cx: &mut Cx, threat: &str) {
    let w = cx.world;
    cx.tally(g, "threats.countered", 1);
    let name = match def(&w, threat).source {
        ThreatSource::Influence => "threats.countered.influence",
        ThreatSource::Cyber => "threats.countered.cyber",
        ThreatSource::Economic => "threats.countered.economic",
        ThreatSource::Military => "threats.countered.military",
    };
    cx.tally(g, name, 1);
}

/// Deals `n` extra threats of the current tier now, one per country in turn
/// from country 0 (Discord cards). An empty deck reshuffles as the round's
/// deal does (070). Variant `off`: nothing.
pub fn deal_extra(g: &mut GameState, cx: &mut Cx, n: i64) {
    let w = cx.world;
    let Some(shared) = shared_row(&w) else {
        return;
    };
    let tier = adversary::threat_tier(&w, g);
    let countries = w.map.countries.len().max(1);
    let n = n.max(0);
    for k in 0..n {
        let c = k as usize % countries;
        refill(g, cx, tier);
        if !deal(g, cx, tier, &deal_pile(shared, c), (!shared).then_some(c)) {
            cx.tally(g, "threats.short", n - k);
            return;
        }
    }
}

/// What standing counters take off `threat` (070): the Auto Counters of its
/// type held by the seat whose home country it sits on. Read, not stored, so
/// a delayed threat isn't cut twice and an extra deal is cut too.
fn standing(w: &World, g: &GameState, threat: &str) -> i64 {
    let source = source_of(w, threat);
    (0..g.seats.len())
        .filter(|&s| g.pile(&on(home_country(w, s))).iter().any(|t| t == threat))
        .flat_map(|s| cards::passives(w, g, s))
        .map(|p| match p {
            Op::AutoCounter { source: x, amount } if *x == source => *amount,
            _ => 0,
        })
        .sum()
}

/// Jam Signals: `threat` needs `n` less of each resource.
pub fn jam(g: &mut GameState, cx: &mut Cx, threat: &str, n: i64) {
    cx.add_key(g, &jam_key(threat), n);
    try_complete(g, cx, threat);
}

/// Counters `threat` if what is paid covers what it needs now: the
/// payments are spent (any excess, after Jam Signals, goes back to the
/// payers, later seats first), payers' Logistics AI pays out, and the
/// threat is discarded. Returns whether it was countered.
fn try_complete(g: &mut GameState, cx: &mut Cx, threat: &str) -> bool {
    let w = cx.world;
    let Some((_, scope)) = active(g).into_iter().find(|(t, _)| t == threat) else {
        return false;
    };
    let req = requirement(&w, g, threat);
    if !req.iter().all(|(r, n)| paid(g, threat, *r) >= *n) {
        return false;
    }
    let payers: Vec<usize> = (0..g.seats.len())
        .filter(|&p| req.iter().any(|(r, _)| g.var(&paid_key(threat, p, *r)) > 0))
        .collect();
    for (r, need) in &req {
        let mut excess = paid(g, threat, *r) - need;
        for p in (0..g.seats.len()).rev() {
            let k = paid_key(threat, p, *r);
            let back = excess.min(g.var(&k));
            if back > 0 {
                cx.add_resource(g, p, *r, back);
                excess -= back;
            }
            cx.set_key(g, &k, 0);
        }
    }
    let auto = standing(&w, g, threat);
    if auto > 0 {
        cx.tally(g, "threats.auto_countered", auto);
    }
    clear(g, cx, threat, scope);
    tally_countered(g, cx, threat);
    cx.note(None, || format!("{} is countered", def(&w, threat).name));
    for p in payers {
        for pa in cards::passives(&w, g, p) {
            if let Op::Logistics { resource, amount } = pa {
                cx.add_resource(g, p, *resource, *amount);
            }
        }
    }
    true
}

#[derive(Debug)]
struct Threats {
    shared: bool,
    per_country: i64,
}

fn build_countries(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Threats {
        shared: false,
        per_country: b.int("per_country")?,
    }))
}

fn build_shared(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Threats {
        shared: true,
        per_country: b.int("per_country")?,
    }))
}

impl Threats {
    fn can_pay(&self, w: &World, g: &GameState, seat: usize, threat: &str, scope: Option<usize>) -> bool {
        may_counter(w, g, seat, threat, scope)
    }

}

impl Module for Threats {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for tier in [1, 2] {
            let mut ids: Vec<Key> = w
                .content
                .threats
                .iter()
                .filter(|t| t.tier == tier)
                .map(|t| Key::new(&t.id))
                .collect();
            shuffle(&mut ids, cx.rng(g, "threats"));
            cx.set_pile(g, &deck(tier), ids);
        }
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        if phase == 0 {
            let tier = adversary::threat_tier(&w, g);
            // Difficulty 1: `extra_threats` more a round in all, one per
            // country in turn (country index 0 first in round 1, then 1, ...).
            let n = w.map.countries.len();
            let extra = if adversary::level(&w, 1) {
                w.int("adversary.extra_threats").max(0) as usize
            } else {
                0
            };
            let first = (g.round.max(1) as usize - 1) % n.max(1);
            for c in 0..n {
                let more = (0..extra).filter(|k| (first + k) % n == c).count() as i64;
                let want = self.per_country + more;
                for k in 0..want {
                    refill(g, cx, tier);
                    if !deal(g, cx, tier, &deal_pile(self.shared, c), (!self.shared).then_some(c)) {
                        // 022: a short deal is counted, not silent.
                        cx.tally(g, "threats.short", want - k);
                        break;
                    }
                }
            }
            return Ok(());
        }
        // Execute: anything fully paid is countered, partial payments come
        // back, then each threat in order. A threat dealt after this round's
        // counter phase waits a round (022: it executed at once, uncounterable).
        for (t, _) in active(g) {
            if !try_complete(g, cx, &t) {
                refund(g, cx, &t);
            }
        }
        let mut executed = 0;
        for (t, scope) in active(g) {
            if waits(g, &t) {
                continue;
            }
            executed += 1;
            let d = def(&w, &t);
            cx.note(None, || match scope {
                Some(c) => format!("Threat {} executes in {}", d.name, country_name(&w, c)),
                None => format!("Threat {} executes everywhere", d.name),
            });
            cx.count(g, "executed", 1);
            if let (Some(c), Some(h)) = (scope, target(g, &t)) {
                make_source(g, cx, c, d.source, h);
            }
            let source = if d.tier == 1 { "threat1" } else { "threat2" };
            effects::run(g, cx, &d.effects, &mut effects::Ctx::threat(scope).named(source))?;
            alignment::discord_from(g, cx, "threat", w.int("alignment.threat_discord"));
            clear(g, cx, &t, scope);
        }
        // 070: the records' read of whether a board can stay clean.
        if executed == 0 {
            cx.tally(g, "threats.rounds_all_countered", 1);
        }
        Ok(())
    }

    fn legal(
        &self,
        _phase: usize,
        g: &GameState,
        w: &World,
        seat: u8,
        out: &mut Vec<crate::Action>,
    ) {
        let s = seat as usize;
        for (t, scope) in active(g) {
            // 044: a seat that lost its presence there can't pay, but can take back.
            let can_pay = self.can_pay(w, g, s, &t, scope);
            for (r, need) in requirement(w, g, &t) {
                let have = g.resource(s, r);
                let left = need - paid(g, &t, r);
                let most = if can_pay { have.min(left) } else { 0 };
                let amounts: &[i64] = match most {
                    ..=0 => &[],
                    1 => &[1],
                    _ => &[1, most],
                };
                for &amount in amounts {
                    out.push(Action::Pay {
                        seat,
                        threat: t.clone(),
                        resource: r.clone(),
                        amount,
                    });
                }
                if g.var(&paid_key(&t, s, r)) > 0 {
                    out.push(Action::Withdraw {
                        seat,
                        threat: t.clone(),
                        resource: r.clone(),
                    });
                }
            }
        }
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Pay { .. } | Action::Withdraw { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let s = a.seat() as usize;
        match a {
            Action::Pay {
                threat,
                resource,
                amount,
                ..
            } => {
                let Some((_, scope)) = active(g).into_iter().find(|(t, _)| t == threat) else {
                    return Err(Halt::Illegal(format!("no active threat {threat}")));
                };
                if !self.can_pay(&w, g, s, threat, scope) {
                    return Err("only a player with one of its sources in reach, or an ambassador on one, can pay".into());
                }
                let req = requirement(&w, g, threat);
                let need = req.iter().find(|(r, _)| r == resource).map(|(_, n)| *n);
                let Some(need) = need else {
                    return Err("that resource is not part of its counter".into());
                };
                if *amount < 1
                    || *amount > g.resource(s, *resource)
                    || *amount > need - paid(g, threat, *resource)
                {
                    return Err("can't pay that much".into());
                }
                // 070: for the records: a payer who reaches it only by ambassador.
                if let Some(c) = scope {
                    if !threat_sources(&w, g, threat, c).iter().any(|&h| in_reach(&w, g, s, h)) {
                        cx.tally(g, "threats.paid_by_ambassador", 1);
                    }
                    if c != home_country(&w, s) {
                        cx.tally(g, "threats.paid_abroad", 1);
                    }
                }
                cx.add_resource(g, s, *resource, -amount);
                cx.add_key(g, &paid_key(threat, s, *resource), *amount);
                try_complete(g, cx, threat);
                Ok(())
            }
            Action::Withdraw {
                threat, resource, ..
            } => {
                let k = paid_key(threat, s, *resource);
                let n = g.var(&k);
                if n == 0 {
                    return Err("nothing to withdraw".into());
                }
                cx.set_key(g, &k, 0);
                cx.add_resource(g, s, *resource, n);
                Ok(())
            }
            _ => Err(Halt::Illegal(format!("not a threat action: {a:?}"))),
        }
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        // Payments and jams belong to a threat card: shown in its text.
        out.bookkeeping(&["threats.paid.", "threats.jam.", "threats.late.", "threats.target.", "threats.made."]);
        let show = |t: &str| {
            let d = def(w, t);
            let req = requirement(w, g, t);
            let progress: Vec<String> = req
                .iter()
                .map(|(r, n)| format!("{}/{}", paid(g, t, *r), r.words(*n)))
                .collect();
            // Who paid what so far (returned if they withdraw), and Jam Signals.
            let mut extra = String::new();
            let payers: Vec<String> = (0..g.seats.len())
                .flat_map(|s| {
                    req.iter().filter_map(move |(r, _)| {
                        let n = g.var(&paid_key(t, s, *r));
                        (n > 0).then(|| format!("seat {s} {}", r.words(n)))
                    })
                })
                .collect();
            if !payers.is_empty() {
                extra.push_str(&format!(" Paid: {}.", payers.join(", ")));
            }
            let jam = g.var(&jam_key(t));
            if jam > 0 {
                extra.push_str(&format!(" Jammed: needs {jam} less of each."));
            }
            if waits(g, t) {
                extra.push_str(" Waits: executes next round.");
            }
            CardView {
                id: t.to_string(),
                name: d.name.clone(),
                text: format!("{} Counter: {}.{extra}", d.text, progress.join(", ")),
                status: progress.join(", "),
                cost: None,
            }
        };
        if self.shared {
            let row: Vec<CardView> = g.pile(ROW).iter().map(|t| show(t)).collect();
            out.pile("row", "Threats (every country)", None, row.len(), row);
        } else {
            for c in 0..w.map.countries.len() {
                let cards: Vec<CardView> = g.pile(&on(c)).iter().map(|t| show(t)).collect();
                if !cards.is_empty() {
                    out.country_cards(c, "on", "threat", cards);
                }
                for t in g.pile(&on(c)).iter() {
                    out.threat_source(ThreatSourceView {
                        threat: t.to_string(),
                        source: source_of(w, t).to_string(),
                        hexes: threat_sources(w, g, t, c),
                        target: target(g, t),
                        seats: (0..g.seats.len()).filter(|&s| may_counter(w, g, s, t, Some(c))).collect(),
                    });
                }
            }
        }
        out.global(
            "tier",
            "threat tier",
            adversary::threat_tier(w, g) as i64,
            Some(2),
            // 020: it was labeled "dealt next round start", one round late at
            // the switch to stage II.
            "the tier of threats dealt this round: 2 once this round's adversary card is stage II (at difficulty 4, sooner)",
        );
    }
}

/// `count` adversary ambassadors from the supply, each onto a piece of kind
/// `at` in `scope` (players choose each).
pub fn place_ambassadors(g: &mut GameState, cx: &mut Cx, scope: Option<usize>, at: Spot, count: i64) -> Res {
    let w = cx.world;
    let hexes = scope_hexes(&w, scope);
    for _ in 0..count {
        let spots: Vec<usize> = hexes
            .iter()
            .copied()
            .filter(|&h| match at {
                Spot::PlayerPieces => player_owned(g, h),
                Spot::CleanCities => clean_city(&w, g, h),
                Spot::Factories => g.tokens(h, FACTORY) > 0,
                Spot::Bases => is_base(&w, h),
            })
            .collect();
        let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
        let Some(i) = cx.choose_loss(0, "Threat: an adversary ambassador goes on which piece?", opts)? else {
            break;
        };
        adversary::place(g, cx, spots[i], 1);
    }
    Ok(())
}

/// Each player in `scope` loses `amount` tokens off their cards, their choice.
pub fn lose_tokens(g: &mut GameState, cx: &mut Cx, scope: Option<usize>, amount: i64) -> Res {
    let w = cx.world;
    for s in seats_in_scope(&w, g, scope) {
        for _ in 0..amount {
            let cards: Vec<Key> = cards::token_cards(&w, g, s, None)
                .into_iter()
                .filter(|c| g.var(&cards::tokens_key(s, c)) > 0)
                .collect();
            let opts = cards.iter().map(|c| opt(c.clone())).collect();
            let prompt = format!("Seat {s}: lose a token from which card?");
            let Some(i) = cx.choose_loss(s, &prompt, opts)? else {
                break;
            };
            cards::add_tokens(g, cx, s, &cards[i], -1);
            cx.tally(g, "threats.tokens_lost", 1);
        }
    }
    Ok(())
}

/// Each player in `scope` discards `n` cards or pays `amount` `resource`
/// (their choice); with neither possible, what they have and their hand.
pub fn requisition(
    g: &mut GameState,
    cx: &mut Cx,
    scope: Option<usize>,
    resource: Resource,
    amount: i64,
    n: i64,
) -> Res {
    let w = cx.world;
    for s in seats_in_scope(&w, g, scope) {
        let mut opts = Vec::new();
        let can_pay = g.resource(s, resource) >= amount;
        if can_pay {
            opts.push(opt(format!("pay {}", resource.amount(amount))));
        }
        if g.pile(&cards::hand(s)).len() as i64 >= n {
            opts.push(opt(format!("discard {n} cards")));
        }
        let prompt = format!("Seat {s}: pay {} or discard {n} cards?", resource.amount(amount));
        match cx.choose(s, &prompt, opts)? {
            Some(0) if can_pay => {
                cx.add_resource(g, s, resource, -amount);
            }
            Some(_) => {
                for k in 0..n {
                    let hand = g.pile(&cards::hand(s)).to_vec();
                    let opts = hand.iter().map(|c| opt(c.clone())).collect();
                    let prompt = format!("Seat {s}: discard which card ({} of {n})?", k + 1);
                    if let Some(j) = cx.choose(s, &prompt, opts)? {
                        cards::discard(g, cx, &hand[j], &cards::hand(s));
                    }
                }
            }
            None => {
                // Neither possible: pay what you have and discard your hand.
                let k = g.resource(s, resource);
                cx.add_resource(g, s, resource, -k);
                for c in g.pile(&cards::hand(s)).to_vec() {
                    cards::discard(g, cx, &c, &cards::hand(s));
                }
            }
        }
    }
    Ok(())
}

/// Each player in `scope` loses a player ambassador: from their supply, else
/// one of theirs on the board (their choice).
pub fn lose_ambassador(g: &mut GameState, cx: &mut Cx, scope: Option<usize>) -> Res {
    let w = cx.world;
    for s in seats_in_scope(&w, g, scope) {
        if g.resource(s, Resource::Ambassadors) > 0 {
            cx.add_resource(g, s, Resource::Ambassadors, -1);
            continue;
        }
        let k = ambassador(s);
        let on = ambassador_hexes(g, s);
        let opts = on.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = format!("Seat {s}: lose the ambassador on which hex?");
        if let Some(i) = cx.choose_loss(s, &prompt, opts)? {
            cx.add_tokens(g, on[i], &k, -1);
        }
    }
    Ok(())
}

/// Each player in `scope` loses a presence in `scope` or pays `amount`
/// `resource` (their choice). Losing one needs 2 or more on the board (027:
/// this never takes a last presence); with neither possible, what they have.
pub fn lose_presence_or(
    g: &mut GameState,
    cx: &mut Cx,
    scope: Option<usize>,
    resource: Resource,
    amount: i64,
    source: &'static str,
) -> Res {
    let w = cx.world;
    let hexes = scope_hexes(&w, scope);
    for s in seats_in_scope(&w, g, scope) {
        let mine: Vec<usize> = if presence_count(g, s) >= 2 {
            hexes.iter().copied().filter(|&h| g.tokens(h, &presence(s)) > 0).collect()
        } else {
            Vec::new()
        };
        let mut opts = Vec::new();
        let can_pay = g.resource(s, resource) >= amount;
        if can_pay {
            opts.push(opt(format!("pay {amount} {resource}")));
        }
        let off = opts.len();
        opts.extend(mine.iter().map(|&h| {
            let mut o = hex_opt(&w, h);
            o.option = format!("lose presence at {}", o.option);
            o
        }));
        let prompt = format!("Seat {s}: pay {amount} {resource} or lose a presence?");
        match cx.choose_loss(s, &prompt, opts)? {
            Some(i) if i < off => cx.add_resource(g, s, resource, -amount),
            Some(i) => {
                board::lose_presence(g, cx, mine[i - off], s, source);
            }
            None => {
                let k = g.resource(s, resource);
                cx.add_resource(g, s, resource, -k);
            }
        }
    }
    Ok(())
}

/// Each player in `scope` discards their dearest card in hand (their choice
/// on a tie).
pub fn discard_most_expensive(g: &mut GameState, cx: &mut Cx, scope: Option<usize>) -> Res {
    let w = cx.world;
    for s in seats_in_scope(&w, g, scope) {
        let hand = g.pile(&cards::hand(s)).to_vec();
        let cost = |id: &str| cards::list_cost(&w, id);
        let Some(top) = hand.iter().map(|c| cost(c)).max() else {
            continue;
        };
        let tied: Vec<Key> = hand.into_iter().filter(|c| cost(c) == top).collect();
        let opts = tied.iter().map(|c| opt(c.as_str())).collect();
        let prompt = format!("Purge Protocols: seat {s} discards which card?");
        if let Some(i) = cx.choose(s, &prompt, opts)? {
            cards::discard(g, cx, &tied[i], &cards::hand(s));
        }
    }
    Ok(())
}

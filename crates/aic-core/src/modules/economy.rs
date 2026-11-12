//! Economy: insight each round, income in upkeep (AI credit production,
//! AI tracks, buildings, Unity cards) and building upkeep. Insight passes
//! between players only through cards that print it (r48: no give action).
//! What the board produces (energy, chips, megaproject
//! cubes) is the board's production phase.

use crate::content::{Op, PerResource, Resource, RoundFlag, RESOURCES, TAGS};
use crate::key::KeyBuf;
use crate::module::{
    Build, Cx, Module, ModuleDef, PhaseSpec, Res, VariantDef, World,
};
use crate::state::GameState;
use crate::view::ViewOut;

use super::common::{has_node_in, seat_opt};
use super::{ai, alignment, board, cards};

pub static DEF: ModuleDef = ModuleDef {
    id: "economy",
    summary: "Insight, income and upkeep",
    requires: &["ai"],
    variants: &[VariantDef {
        name: "standard",
        summary: "Income in upkeep, insight at the start of each round",
        params: &[],
        phases: &[
            PhaseSpec::auto(
                "upkeep",
                "collect income (credit production, tracks, buildings); pay building upkeep",
            ),
        ],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

/// Seat `s`'s printed production of `resource` raised by one-time permanent
/// gains (021).
pub fn production_key(seat: usize, resource: Resource) -> KeyBuf {
    KeyBuf::new("economy.production.").num(seat).text(".").text(resource.name())
}

/// Raises seat `s`'s production of `r` by `n` (a program, Copy). No cap (127:
/// a runaway is fixed at the cards that raise it).
pub fn raise_production(g: &mut GameState, cx: &mut Cx, seat: usize, r: Resource, n: i64) {
    if n != 0 {
        cx.add_key(g, &production_key(seat, r), n);
    }
}

/// Income of `r` seat `s` loses in its next upkeep (135: Credit Crunch), down to none; read once, then cleared.
pub fn income_debt(seat: usize, r: Resource) -> KeyBuf {
    KeyBuf::new("economy.income_debt.").num(seat).text(".").text(r.name())
}

/// Insight seat `s` gains next round from another seat's capstone (075).
pub fn insight_gift(seat: usize) -> String {
    format!("economy.insight_gift.{seat}")
}

/// Insight seat `s` carried into this round by Patience (075).
pub fn insight_carried(seat: usize) -> String {
    format!("economy.insight_carried.{seat}")
}

/// Whether seat `s`'s unspent insight carries over (075: Patience): its AI
/// board's innate, the only place content may print it.
pub fn patient(w: &World, seat: usize) -> bool {
    ai::board(w, seat).innate.passives.iter().any(|p| matches!(p, Op::Patience))
}

#[derive(Debug)]
struct Economy;

fn build(_b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Economy))
}

/// What the next upkeep pays seat `s`: its `income`, with no energy for a
/// seat with a node in a country sabotaged then. Once this round's upkeep
/// has run, a flag counts only if it outlasts this round's cleanup (2+).
fn upkeep_income(w: &World, g: &GameState, seat: usize) -> PerResource {
    let mut inc = income(w, g, seat);
    let upkeep_done = g.phase_index("economy.upkeep").is_some_and(|i| g.phase > i);
    let at = if upkeep_done { 2 } else { 1 };
    if (0..w.map.countries.len()).any(|c| {
        board::flag_level(g, RoundFlag::Sabotage, c) >= at && has_node_in(w, g, seat, c)
    }) {
        inc[Resource::Energy] = 0;
    }
    // 135: income a threat took from this upkeep.
    for r in RESOURCES {
        let debt = g.var(&income_debt(seat, r));
        if debt > 0 && r != Resource::Insight {
            inc[r] = (inc[r] - debt).max(0);
        }
    }
    inc
}

/// Ambassadors seat `s` gains a round (068): `Op::Income` prints them, though
/// they sit in no income array, being pieces rather than a player resource.
pub fn ambassador_income(w: &World, g: &GameState, seat: usize) -> i64 {
    cards::passives(w, g, seat)
        .iter()
        .map(|p| match p {
            Op::Income { resource: Resource::Ambassadors, amount } => *amount,
            _ => 0,
        })
        .sum()
}

/// Seat `s`'s income per resource from its AI board, tracks, buildings and
/// Unity cards (not the board's production).
pub fn income(w: &World, g: &GameState, seat: usize) -> PerResource {
    income_with(w, g, seat, &cards::tag_counts(w, g, seat))
}

/// [`income`], with the seat's [`cards::tag_counts`] already counted.
pub fn income_with(w: &World, g: &GameState, seat: usize, counts: &[i64; TAGS.len()]) -> PerResource {
    let mut out = PerResource::default();
    let mut add = |r: Resource, n: i64| out.add(r, n);
    add(Resource::Credits, ai::board(w, seat).credits);
    ai::each_track_income(w, g, seat, &mut add);
    cards::each_passive(w, g, seat, |p| match p {
        Op::Income { resource, amount } => add(*resource, *amount),
        Op::IncomePerTag {
            resource,
            tag,
            per,
            amount,
        } => add(*resource, amount * (counts[*tag as usize] / per)),
        _ => {}
    });
    // A walk of the seat's keys: most seats hold none of them.
    let prefix = KeyBuf::new("economy.production.").num(seat).text(".");
    for (k, n) in g.vars.from_key(prefix.as_str()).map_while(|(k, n)| Some((k.strip_prefix(prefix.as_str())?, *n))) {
        if let Some(r) = RESOURCES.iter().find(|r| r.name() == k) {
            add(*r, n);
        }
    }
    add(Resource::Credits, alignment::income_bonus(g));
    add(Resource::Insight, alignment::insight_bonus(g));
    out
}

/// Seat `s`'s capstone (075): `amount` insight from the supply for the other
/// players next round, one at a time, split as `s` chooses. Its own bank pays
/// nothing, and what lands is the receiver's next insight.
fn supply_insight(g: &mut GameState, cx: &mut Cx, s: usize) -> Res {
    let w = cx.world;
    // 118: alone, "other players" means you (Matt's ruling 7, r44): it gave nothing.
    let others: Vec<usize> = if g.seats.len() == 1 { vec![s] } else { (0..g.seats.len()).filter(|&o| o != s).collect() };
    let amount: i64 = cards::passives(&w, g, s)
        .iter()
        .map(|p| match p {
            Op::SupplyInsight { amount } => *amount,
            _ => 0,
        })
        .sum();
    if others.is_empty() {
        return Ok(());
    }
    for k in 0..amount {
        let opts = others.iter().map(|&o| seat_opt(o)).collect();
        let prompt = || format!("Seat {s}: insight {} of {amount} from the supply goes to whom?", k + 1);
        if let Some(i) = cx.choose(s, prompt, opts)? {
            cx.add_key(g, &insight_gift(others[i]), 1);
            cx.tally(g, "economy.insight_supplied", 1);
        }
    }
    Ok(())
}

impl Module for Economy {
    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for s in 0..g.seats.len() {
            // Unspent insight is lost, but under Patience (075), which keeps all
            // the seat holds (r82: insight others gave it too). No adversary harm
            // takes insight (r82), so nothing is owed against the reset.
            let have = g.resource(s, Resource::Insight);
            let gift = g.var(&insight_gift(s));
            let carried = if patient(&w, s) { have.max(0) } else { 0 };
            let n = income(&w, g, s)[Resource::Insight] + gift + carried;
            cx.add_resource(g, s, Resource::Insight, n - have);
            if gift > 0 {
                cx.set_key(g, &insight_gift(s), 0);
            }
            if patient(&w, s) {
                cx.set_key(g, &insight_carried(s), carried);
                cx.tally(g, "economy.insight_carried", carried);
                cx.per_round(g, "economy.insight_carried", carried);
            }
        }
    }

    fn run(&self, _phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        let w = cx.world;
        for s in 0..g.seats.len() {
            // What the action phase left unspent (021 screen): insight expires.
            let (cr, ins) = (g.resource(s, Resource::Credits), g.resource(s, Resource::Insight));
            cx.per_round(g, "economy.credits_unspent", cr);
            cx.per_round(g, "economy.insight_unspent", ins);
            let inc = upkeep_income(&w, g, s);
            for (r, n) in inc.iter() {
                if r != Resource::Insight {
                    cx.add_resource(g, s, r, n);
                }
            }
            cx.add_resource(g, s, Resource::Ambassadors, ambassador_income(&w, g, s));
            cx.count(g, "credits_income", inc[Resource::Credits]);
            for r in RESOURCES {
                let k = income_debt(s, r);
                if r != Resource::Insight && g.var(&k) > 0 {
                    cx.tally(g, "economy.income_lost", g.var(&k));
                    cx.set_key(g, &k, 0);
                }
            }
            cards::upkeep(g, cx, s);
            supply_insight(g, cx, s)?;
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        for s in 0..g.seats.len() {
            // What upkeep will pay, Sabotage included (020: the view showed the
            // energy a sabotaged seat would not get).
            let inc = upkeep_income(w, g, s);
            let parts = [
                (
                    "credits",
                    inc[Resource::Credits],
                    "credits in upkeep: credit production, tracks, buildings",
                ),
                (
                    "chips",
                    inc[Resource::Chips],
                    "chips in upkeep: Industry cards (player-owned factories make them in production)",
                ),
                (
                    "energy",
                    inc[Resource::Energy],
                    "energy in upkeep: the AI board, buildings (cities make energy in production)",
                ),
                (
                    "antimatter",
                    inc[Resource::Antimatter],
                    "antimatter in upkeep: buildings",
                ),
                (
                    "insight",
                    inc[Resource::Insight],
                    if patient(w, s) {
                        "insight at the start of each round (Patience: unspent insight carries over)"
                    } else {
                        "insight at the start of each round (unspent insight is lost)"
                    },
                ),
                (
                    "research_cubes",
                    inc[Resource::ResearchCubes],
                    "research cubes in upkeep: a capstone, buildings (stations make theirs in production)",
                ),
            ];
            for (r, n, doc) in parts {
                out.seat(s, &format!("income.{r}"), &format!("+{}", r.replace('_', " ")), n, None, doc);
            }
            if patient(w, s) {
                out.seat(
                    s,
                    "insight_carried",
                    "insight carried",
                    g.var(&insight_carried(s)),
                    None,
                    "Patience: unspent insight this seat carried into this round; all it holds now carries to the next",
                );
            }
            let gift = g.var(&insight_gift(s));
            if gift > 0 {
                out.seat(s, "insight_gift", "insight next round", gift, None, "this much more insight next round, from another seat's capstone");
            }
        }
        out.bookkeeping(&[
            "economy.production.",
            "economy.income_debt.",
            "economy.insight_gift.",
            "economy.insight_carried.",
        ]);
    }
}

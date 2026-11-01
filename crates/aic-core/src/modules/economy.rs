//! Economy: insight each round, income in upkeep (AI credit production,
//! AI tracks, buildings, Unity cards), building upkeep, and giving insight
//! to another player. What the board produces (energy, chips, megaproject
//! cubes) is the board's production phase.

use crate::action::Action;
use crate::content::{Op, PerResource, Resource, RoundFlag, Tag, RESOURCES};
use crate::key::KeyBuf;
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::state::GameState;
use crate::view::ViewOut;

use super::common::has_presence_in;
use super::{ai, alignment, board, cards};

pub static DEF: ModuleDef = ModuleDef {
    id: "economy",
    summary: "Insight, income and upkeep; giving insight",
    requires: &["ai"],
    variants: &[VariantDef {
        name: "standard",
        summary: "Income in upkeep, insight at the start of each round",
        params: &[ParamSpec::int(
            "give_insight",
            0,
            10,
            "insight the receiver gains for 1 given",
        )],
        phases: &[
            PhaseSpec::decision(
                "give",
                "give 1 insight to another player (combine with the action phase)",
            ),
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

/// Insight seat `s` loses next round (021: an adversary ambassador on its presence).
pub fn insight_debt(seat: usize) -> String {
    format!("economy.insight_debt.{seat}")
}

#[derive(Debug)]
struct Economy {
    give_insight: i64,
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Economy {
        give_insight: b.int("give_insight")?,
    }))
}

/// What the next upkeep pays seat `s`: its `income`, with no energy for a
/// seat with presence in a country sabotaged then. Once this round's upkeep
/// has run, a flag counts only if it outlasts this round's cleanup (2+).
fn upkeep_income(w: &World, g: &GameState, seat: usize) -> PerResource {
    let mut inc = income(w, g, seat);
    let upkeep_done = g.phase_index("economy.upkeep").is_some_and(|i| g.phase > i);
    let at = if upkeep_done { 2 } else { 1 };
    if (0..w.map.countries.len()).any(|c| {
        board::flag_level(g, RoundFlag::Sabotage, c) >= at && has_presence_in(w, g, seat, c)
    }) {
        inc[Resource::Energy] = 0;
    }
    inc
}

/// Seat `s`'s income per resource from its AI board, tracks, buildings and
/// Unity cards (not the board's production).
pub fn income(w: &World, g: &GameState, seat: usize) -> PerResource {
    let mut out = PerResource::default();
    let mut add = |r: Resource, n: i64| out.add(r, n);
    add(Resource::Credits, ai::board(w, seat).credits);
    for (r, n) in ai::track_income(w, g, seat) {
        add(r, n);
    }
    // Counted at the first passive that asks: most of a seat's pay by tag.
    let mut counts = None;
    let mut tags = |tag: Tag| counts.get_or_insert_with(|| cards::tag_counts(w, g, seat))[tag as usize];
    for p in cards::passives(w, g, seat) {
        match p {
            Op::Income { resource, amount } => add(*resource, *amount),
            Op::IncomePerTag {
                resource,
                tag,
                per,
            } => add(*resource, tags(*tag) / per),
            _ => {}
        }
    }
    for r in RESOURCES {
        add(r, g.var(&production_key(seat, r)));
    }
    add(Resource::Credits, alignment::income_bonus(g));
    add(Resource::Insight, alignment::insight_bonus(g));
    out
}

impl Economy {
    /// Whether seat `s` can give insight now. `give_insight` 0 switches giving
    /// off (rules.md §9; 020: it was still offered and cost the giver an
    /// insight for nothing).
    fn can_give(&self, g: &GameState, seat: usize) -> bool {
        g.resource(seat, Resource::Insight) >= 1 && self.give_insight > 0
    }
}

impl Module for Economy {
    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for s in 0..g.seats.len() {
            // Unspent insight is lost.
            let have = g.resource(s, Resource::Insight);
            let debt = g.var(&insight_debt(s));
            let n = (income(&w, g, s)[Resource::Insight] - debt).max(0);
            cx.add_resource(g, s, Resource::Insight, n - have);
            if debt > 0 {
                cx.set_key(g, &insight_debt(s), 0);
            }
        }
    }

    fn legal(&self, _phase: usize, g: &GameState, _w: &World, seat: u8, out: &mut Vec<Action>) {
        if !self.can_give(g, seat as usize) {
            return;
        }
        for to in 0..g.seats.len() as u8 {
            if to != seat {
                out.push(Action::Give { seat, to });
            }
        }
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Give { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let Action::Give { seat, to } = *a else {
            return Err(Halt::Illegal("not a give".into()));
        };
        let (s, t) = (seat as usize, to as usize);
        if s == t || t >= g.seats.len() || !self.can_give(g, s) {
            return Err("can't give insight there".into());
        }
        cx.add_resource(g, s, Resource::Insight, -1);
        cx.add_resource(g, t, Resource::Insight, self.give_insight);
        for p in cards::passives(&w, g, s) {
            if let Op::GiveBonus { resource, amount } = p {
                cx.add_resource(g, t, *resource, *amount);
            }
        }
        cx.count(g, "insight_given", 1);
        Ok(())
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
            cx.count(g, "credits_income", inc[Resource::Credits]);
            cards::upkeep(g, cx, s);
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
                    "insight at the start of each round (unspent insight is lost)",
                ),
            ];
            for (r, n, doc) in parts {
                out.seat(s, &format!("income.{r}"), &format!("+{r}"), n, None, doc);
            }
            let debt = g.var(&insight_debt(s));
            if debt > 0 {
                out.seat(
                    s,
                    "insight_debt",
                    "insight lost next round",
                    debt,
                    None,
                    "this much less insight next round: from adversary ambassadors on your presence, or a threat",
                );
            }
        }
        out.bookkeeping(&["economy.production.", "economy.insight_debt."]);
    }
}

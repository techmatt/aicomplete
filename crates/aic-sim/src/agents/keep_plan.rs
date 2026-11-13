//! The keep plan (022): the sketch of a seat's next rounds that
//! [`super::policy::Policy`] keeps drafted cards by ([`plan_keeps`]).
//!
//! The plan is simple and deterministic, and reads only the public state
//! and the seat's own hand: projected credit, insight,
//! chips, energy and antimatter income; the cards it would play (hand plus the
//! candidates) in value-per-cost order, filled into each round's budget.

use aic_core::content::{CardDef, Op, Resource, TAGS};
use aic_core::modules::{adversary, cards, common, economy, effects, threats};
use aic_core::module::World;
use aic_core::{GameState, Key};

use super::cards::{is_capstone, is_liberate};
use super::policy::Forced;
use super::{credit_cost, credit_equivalent};
use super::strategy::{self, Mults};
use super::value::{self, CardValue, Ctx};

/// A play's value falls by this share for each round it waits.
const WAIT: f64 = 0.15;

/// One round's budget in the plan.
#[derive(Clone, Copy)]
struct Budget {
    credits: i64,
    insight: i64,
    chips: i64,
    energy: i64,
    antimatter: i64,
    /// 092 (`plan_cubes`): research cubes.
    research_cubes: i64,
}

/// What the keep plan values cards by (082): the value table or the printed
/// cost, and the seat's strategy.
#[derive(Clone, Copy)]
pub struct PlanCtx<'a> {
    pub table: bool,
    pub strat: Option<u16>,
    pub mults: Option<&'a Mults>,
    /// The value table the plan prices by (102, D23: repriced at the heuristic's rates).
    pub t: &'a value::Table,
    /// 102 (D34): what the heuristic charges for an adversary project
    /// wholly filled, in table credits (0: no situational project value).
    pub project: f64,
    /// 102: the credits the heuristic counts in full and an idle
    /// credit's share of one beyond them; a net price pays from the idle ones
    /// first (`(f64::MAX, 1.0)`: every credit in full).
    pub idle: (f64, f64),
    /// 124 (`keep_spare`): insight beyond this is spare (0: off).
    pub spare_at: f64,
    /// 128: the seat's `Ctx::energy_rate` at the keep.
    pub energy_rate: f64,
    /// 150 (`plan_spare`): the credits an insight income a round is worth to a
    /// Patience seat whose insight is spare (`None`: the table's rate).
    pub insight_idle: Option<f64>,
}

/// What a plan's values read once (082): rounds left by the adversary deck, the
/// seat's nodes, whether a city is corrupted; 102: the visible projects' shares
/// filled, fullest first.
struct PlanTable {
    left: f64,
    node: f64,
    corrupted: bool,
    projects: Vec<(f64, i64)>,
    /// 107 (`rates.own_mix`): the share of the seat's cards carrying each tag.
    share: [f64; TAGS.len()],
    /// 124 (`plan_team`): the teammates' share (`None`: the seat's own).
    team: Option<[f64; TAGS.len()]>,
    /// 128: `Ctx::cubes_open` for the seat.
    cubes_open: bool,
}

impl PlanTable {
    fn read(t: &value::Table, w: &World, g: &GameState, s: usize, f: &Forced) -> PlanTable {
        PlanTable {
            team: (f.plan_team && g.seats.len() > 1).then(|| {
                let others = (0..g.seats.len())
                    .filter(|&o| o != s)
                    .map(|o| super::eval::own_mix_counts(w, g, o, &cards::tag_counts(w, g, o)));
                super::eval::team_share(t, others)
            }),
            left: super::eval::Eval::cards_left(g) as f64,
            node: common::node_count(g, s) as f64,
            corrupted: (0..g.hexes.len()).any(|h| common::corrupted_city(w, g, h)),
            cubes_open: !f.plan_cube_target || !aic_core::modules::effects::cube_projects(w, g, s, false).is_empty(),
            share: {
                let tags = cards::tag_counts(w, g, s);
                super::eval::own_mix(t, w, g, s, &tags, Ctx::at(t, 0.0, 0.0, tags, false)).share
            },
            projects: {
                // (share filled, compute on it), fullest first.
                let mut p: Vec<(f64, i64)> = adversary::projects(g)
                    .iter()
                    .map(|id| {
                        let on = g.var(&adversary::project_key(id));
                        (on as f64 / adversary::project_cost(w, g, id).max(1) as f64, on)
                    })
                    .collect();
                p.sort_by(|a, b| b.0.total_cmp(&a.0));
                p
            },
        }
    }
}

/// 102 (D34): what a card's project ops add over their table rows on this table:
/// a discard takes the fullest project, a drain the compute it can reach, each
/// at what the heuristic charges for it (`PlanCtx::project`). A one-use worth,
/// once (the project is the table as it stands).
fn situational(d: &CardDef, pc: &PlanCtx, pt: &PlanTable) -> f64 {
    if pc.project <= 0.0 {
        return 0.0;
    }
    let t = pc.t;
    let action = d.ability.iter().flat_map(|a| a.branches()).flat_map(|b| &b.effects);
    d.effects
        .iter()
        .chain(action)
        .map(|e| match e {
            Op::DiscardProject => match pt.projects.first() {
                Some((share, _)) => (pc.project * share - t.op("discard_project")).max(0.0),
                None => 0.0,
            },
            Op::DrainProject { amount } => match pt.projects.first() {
                Some((share, on)) if *on > 0 => {
                    let cost = *on as f64 / share.max(1e-9);
                    let drained = (*amount).min(*on) as f64;
                    (pc.project * drained / cost - t.op("drain_project") * *amount as f64).max(0.0)
                }
                _ => 0.0,
            },
            _ => 0.0,
        })
        .fold(0.0, f64::max)
}

/// The drafted cards seat `s` keeps: each, best value per cost first, if
/// the plan with it is worth more than the plan without it.
pub fn plan_keeps(w: &World, g: &GameState, s: usize, f: &Forced, pc: &PlanCtx) -> Vec<Key> {
    let pt = PlanTable::read(pc.t, w, g, s, f);
    let mut drafted = g.pile(&cards::drafted(s)).to_vec();
    let tags = cards::tag_counts(w, g, s);
    by_ratio(w, g, s, f, pc, &pt, &tags, &mut drafted);
    let mut kept: Vec<Key> = Vec::new();
    let mut best = plan_value(w, g, s, &kept, f, pc, &pt);
    for c in drafted {
        let mut with = kept.clone();
        with.push(c);
        let Some(v) = plan_value_checked(w, g, s, &with, &c, f, pc, &pt) else {
            continue;
        };
        if v > best + 1e-9 {
            best = v;
            kept = with;
        }
    }
    // 124 (`keep_spare`): insight beyond the heuristic's cap buys the picks the
    // plan can't fit yet: each worth its price at the table, best first, while
    // the spare insight pays its whole keep ("with insight to burn, keep every
    // card worth having").
    if pc.spare_at > 0.0 {
        let mut spare = g.resource(s, Resource::Insight) as f64 - pc.spare_at;
        for id in &kept {
            spare -= cards::keep_need(w, g, s, cards::def(w, id).tier) as f64;
        }
        let mut rest: Vec<Key> = g.pile(&cards::drafted(s)).iter().filter(|c| !kept.contains(c)).cloned().collect();
        by_ratio(w, g, s, f, pc, &pt, &tags, &mut rest);
        for c in rest {
            let d = cards::def(w, &c);
            let ins = cards::keep_need(w, g, s, d.tier);
            if ins as f64 > spare {
                continue;
            }
            let worth = if pc.table {
                let typical = Ctx::typical_for(pc.t, d.tier);
                CardValue::of(pc.t, d).total(pc.t, &typical) - pc.t.cost(d, d.cost, d.insight, typical.horizon)
            } else {
                1.0
            };
            if worth <= 0.0 {
                continue;
            }
            spare -= ins as f64;
            kept.push(c);
        }
    }
    kept
}

/// 203 (`kept_rule`): the hand cards the plan plays this round, from what the seat holds now, in the
/// plan's order: the ones worth their price at the plan's rates and affordable together.
pub fn plan_now(w: &World, g: &GameState, s: usize, f: &Forced, pc: &PlanCtx) -> Vec<Key> {
    let pt = PlanTable::read(pc.t, w, g, s, f);
    let one = Forced { plan_rounds: 1, plan_chain: false, ..f.clone() };
    plan(w, g, s, &[], &one, pc, &pt).map(|(_, p)| p).unwrap_or_default()
}

/// [`plan_value`] with `kept`, or `None` if the keeps can't be paid or the
/// new card `c` is not played in the plan.
#[allow(clippy::too_many_arguments)]
fn plan_value_checked(
    w: &World,
    g: &GameState,
    s: usize,
    kept: &[Key],
    c: &str,
    f: &Forced,
    pc: &PlanCtx,
    pt: &PlanTable,
) -> Option<f64> {
    let (v, planned) = plan(w, g, s, kept, f, pc, pt)?;
    planned.iter().any(|p| p == c).then_some(v)
}

fn plan_value(w: &World, g: &GameState, s: usize, kept: &[Key], f: &Forced, pc: &PlanCtx, pt: &PlanTable) -> f64 {
    plan(w, g, s, kept, f, pc, pt).map_or(f64::NEG_INFINITY, |(v, _)| v)
}

/// The jobs `d` would do for seat `s` on this table: markers in reach to
/// clear, threats to delay, nodes with no token to shield.
fn niche_jobs(w: &World, g: &GameState, s: usize, d: &CardDef) -> i64 {
    d.effects
        .iter()
        .map(|e| match e {
            Op::ClearMarkers { count } => {
                let open = |h: &usize| g.tokens(*h, common::MARKER) > 0 && common::in_reach(w, g, s, *h);
                (*count).min((0..g.hexes.len()).filter(open).count() as i64)
            }
            // 118: only threats the seat may counter, as the engine delays (F136).
            Op::DelayThreat { count } => {
                (*count).min(threats::counterable(w, g, s, None).iter().filter(|t| !threats::waits(g, t)).count() as i64)
            }
            Op::ShieldNode { count } => {
                (*count).min((0..g.hexes.len()).filter(|&h| effects::shieldable(g, s, h)).count() as i64)
            }
            // 070: a typed counter with a threat up it can reach (threats are dealt before keeps).
            Op::Jam { amount, source: Some(t) } => {
                if threats::counterable(w, g, s, Some(*t)).is_empty() { 0 } else { (*amount + 1) / 2 }
            }
            _ => 0,
        })
        .sum::<i64>()
}

/// 070: what a capstone's standing effect is worth to the keep plan, in
/// `plan_niche` jobs: its printed price undersells a card that pays every round.
/// 082: only on printed costs; the table values a standing effect by its rounds.
fn capstone_jobs(d: &CardDef) -> i64 {
    if !is_capstone(d) {
        return 0;
    }
    d.passives
        .iter()
        .map(|p| match p {
            Op::AutoCounter { amount, .. } => *amount,
            Op::ProjectDrain { amount } => 3 * *amount,
            Op::IncomePerTag { .. } => 3,
            _ => 0,
        })
        .sum()
}

/// The card's value to the plan, played `round` rounds from now with `tags` on
/// the seat's cards. On printed costs (before 082): its printed cost plus 1, plus
/// the jobs it would do (046) and a capstone's (070). By the table (082): its
/// ops' value at the rounds then left, times the seat's strategy multiplier, plus
/// the jobs it would do now.
#[allow(clippy::too_many_arguments)]
fn value(
    w: &World,
    g: &GameState,
    s: usize,
    d: &CardDef,
    f: &Forced,
    pc: &PlanCtx,
    pt: &PlanTable,
    round: usize,
    tags: &[i64; TAGS.len()],
    planned: &[Key],
) -> f64 {
    let jobs = f.plan_niche * niche_jobs(w, g, s, d) as f64;
    if !pc.table {
        return 1.0 + credit_cost(d) as f64 + jobs + f.plan_niche * capstone_jobs(d) as f64;
    }
    let t = pc.t;
    let ctx = Ctx {
        share: pt.share,
        team: pt.team,
        energy_rate: pc.energy_rate,
        cubes_open: pt.cubes_open,
        ..Ctx::at(t, (pt.left - round as f64).max(0.0), pt.node, *tags, pt.corrupted)
    };
    let m = match (pc.strat, pc.mults) {
        (Some(k), Some(mults)) => {
            let i = w.content.card_index(&d.id).unwrap_or(0);
            mults.get(k, i, strategy::phase(g.round + round as u32))
        }
        _ => 1.0,
    };
    // A building that must go over one of the seat's adds only what it beats the
    // one it covers by (082: kept and never played, it was worth less than that).
    let covered = match (f.plan_cover, &d.replaces) {
        (true, Some(tag)) => {
            let built = || g.pile(&cards::built(s)).iter().map(|id| cards::def(w, id)).filter(|b| b.tags.contains(tag));
            // 107 (`plan_under`): with none built, the planned building it goes over.
            let under: Vec<&CardDef> = if built().next().is_none() && f.plan_under {
                planned.iter().map(|id| cards::def(w, id)).filter(|b| b.is_building() && b.tags.contains(tag)).collect()
            } else {
                built().collect()
            };
            under
                .into_iter()
                // 102: what it would still earn; what playing it did is spent.
                .map(|b| buried_value(t, b, &ctx))
                .fold(f64::INFINITY, f64::min)
        }
        _ => f64::INFINITY,
    };
    let covered = if covered.is_finite() { covered.max(0.0) } else { 0.0 };
    // 150 (`plan_spare`): an insight income onto a spare Patience pile, at the idle rate.
    let spare = pc.insight_idle.map_or(0.0, |idle| {
        let per_round: i64 = d
            .passives
            .iter()
            .map(|p| match p {
                Op::Income { resource: Resource::Insight, amount } => *amount,
                _ => 0,
            })
            .sum();
        ctx.horizon * per_round as f64 * t.op("income") * (t.res(Resource::Insight) - idle).max(0.0)
    });
    m * (CardValue::of(t, d).total(t, &ctx) - spare) - covered + jobs + situational(d, pc, pt)
}

/// 092: what building `d`, buried now, would still have earned: its standing
/// effects and a share of its Action, each round left. Its tags stay, and what
/// playing it did (production included) is spent.
fn buried_value(t: &value::Table, d: &CardDef, ctx: &Ctx) -> f64 {
    let standing = value::passives_worth(t, &d.passives);
    // An Action at the value table's own rates, as `CardValue::of` reads it.
    let action = d.ability.as_ref().map(|a| value::ability_worth(t.own(), a)).unwrap_or_default();
    let use_ = (action.instant(t, ctx) + action.per_round(ctx)).max(0.0);
    // 099: a one-shot Action (Popular Uprising) buried unused loses its one use,
    // not a share of it every round left.
    if d.ability.as_ref().is_some_and(|a| a.once && a.or.is_none()) {
        return (ctx.horizon * standing.per_round(ctx)).max(0.0) + use_;
    }
    let uses = if d.ability.is_some() { t.rate("uses_share") } else { 0.0 };
    let round = standing.per_round(ctx) + uses * use_;
    (ctx.horizon * round).max(0.0)
}

/// Whether a building that must go over one of the seat's can (024: the
/// plan kept them with nothing to go over, and they were never played).
fn replaceable(w: &World, g: &GameState, s: usize, d: &CardDef) -> bool {
    d.replaces.as_ref().is_none_or(|t| {
        g.pile(&cards::built(s)).iter().any(|id| cards::def(w, id).tags.contains(t))
    })
}

/// 107 (`plan_under`): whether a building that must go over one of the seat's
/// tag may go over a building the plan plays before it.
fn under_planned(w: &World, d: &CardDef, planned: &[Key]) -> bool {
    d.replaces.as_ref().is_some_and(|t| {
        planned.iter().map(|id| cards::def(w, id)).any(|b| b.is_building() && b.replaces.is_none() && b.tags.contains(t))
    })
}

/// What seat `s` pays to play `d`: credits after its building discount, its
/// other costs at fixed rates (before 082) or the table's (082).
fn spend(w: &World, g: &GameState, s: usize, d: &CardDef, pc: &PlanCtx, pt: &PlanTable) -> f64 {
    if pc.table {
        pc.t.cost(d, cards::play_cost(w, g, s, d), cards::play_insight(w, g, s, d), pt.left)
    } else {
        credit_equivalent(d, cards::play_cost(w, g, s, d)) as f64
    }
}

#[allow(clippy::too_many_arguments)]
fn ratio(w: &World, g: &GameState, s: usize, id: &str, f: &Forced, pc: &PlanCtx, pt: &PlanTable, tags: &[i64; TAGS.len()]) -> f64 {
    let d = cards::def(w, id);
    value(w, g, s, d, f, pc, pt, 0, tags, &[]) / (1.0 + spend(w, g, s, d, pc, pt))
}

/// `ids`, best value per cost first.
#[allow(clippy::too_many_arguments)]
fn by_ratio(w: &World, g: &GameState, s: usize, f: &Forced, pc: &PlanCtx, pt: &PlanTable, tags: &[i64; TAGS.len()], ids: &mut [Key]) {
    let mut keyed: Vec<(f64, Key)> = ids.iter().map(|id| (ratio(w, g, s, id, f, pc, pt, tags), *id)).collect();
    keyed.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    for (slot, (_, id)) in ids.iter_mut().zip(keyed) {
        *slot = id;
    }
}

/// The plan with `kept` bought now: its value, and the cards it plays.
/// `None` if the keeps can't be paid. By the table (082), a card's `needs` may be
/// met by the tags of the cards the plan plays before it, and a card with no
/// target now may be played in a later round, at the table's target share.
fn plan(
    w: &World,
    g: &GameState,
    s: usize,
    kept: &[Key],
    f: &Forced,
    pc: &PlanCtx,
    pt: &PlanTable,
) -> Option<(f64, Vec<Key>)> {
    let rounds = f.plan_rounds;
    let res = |r: Resource| g.resource(s, r).max(0);
    let income = economy::income(w, g, s);
    let inc = |r: Resource| income[r];
    let mut now = Budget {
        credits: res(Resource::Credits),
        insight: res(Resource::Insight),
        chips: res(Resource::Chips),
        energy: res(Resource::Energy),
        antimatter: res(Resource::Antimatter),
        research_cubes: res(Resource::ResearchCubes),
    };
    // Pay the keeps in insight (r82: no credits buy a shortfall).
    for id in kept {
        now.insight -= cards::keep_need(w, g, s, cards::def(w, id).tier);
        if now.insight < 0 {
            return None;
        }
    }
    let mut tags = cards::tag_counts(w, g, s);
    let mut pool: Vec<Key> = g.pile(&cards::hand(s)).to_vec();
    pool.extend(kept.iter().cloned());
    by_ratio(w, g, s, f, pc, pt, &tags, &mut pool);
    let mut planned: Vec<Key> = Vec::new();
    let mut total = 0.0;
    let mut b = now;
    let mut needs = cards::tag_counts(w, g, s);
    let needs_now = needs;
    // 107 (`plan_reach`): the tags the hand and the keeps would add, a card's own aside.
    let mut reach = needs_now;
    if f.plan_reach && pc.table {
        for id in &pool {
            let d = cards::def(w, id);
            if !d.is_event() {
                for t in &d.tags {
                    reach[*t as usize] += 1;
                }
            }
        }
    }
    // 107: a seat builds toward a chain card only on its strategy's tags ("commit
    // to the tags your strategy plays"); greedy, with no strategy, never does.
    let committed = |d: &CardDef| {
        pc.strat.is_some_and(|k| {
            let spec = &strategy::strategies().strategies[k as usize];
            d.needs.iter().all(|(t, _)| spec.tag_weight(*t) > 0.0)
        })
    };
    let near = |d: &CardDef| {
        let by_hand = f.plan_reach && committed(d);
        let own = |t: usize| if by_hand && !d.is_event() { d.tags.iter().filter(|x| **x as usize == t).count() as i64 } else { 0 };
        let have = |t: usize| if by_hand { reach[t] - own(t) } else { needs_now[t] };
        !pc.table || d.needs.iter().map(|(t, n)| (n - have(*t as usize)).max(0)).sum::<i64>() <= f.plan_short as i64
    };
    let patient = economy::patient(w, s);
    let share = pc.t.rate("target_share");
    // 092 (`plan_bury`): the slots still open, and the tops a building may bury.
    let mut tops: Vec<&CardDef> = g.pile(&cards::built(s)).iter().map(|id| cards::def(w, id)).collect();
    let mut open = cards::capacity(w, g, s) - tops.len() as i64;
    for round in 0..rounds.max(1) {
        if round > 0 {
            // Credits, chips, energy and antimatter carry over; insight resets,
            // but under Patience (075).
            b = Budget {
                credits: b.credits + inc(Resource::Credits),
                insight: inc(Resource::Insight).max(0) + if patient { b.insight } else { 0 },
                chips: b.chips + inc(Resource::Chips),
                energy: b.energy + inc(Resource::Energy),
                antimatter: b.antimatter + inc(Resource::Antimatter),
                research_cubes: b.research_cubes + inc(Resource::ResearchCubes),
            };
        }
        for id in &pool {
            if planned.contains(id) {
                continue;
            }
            let d = cards::def(w, id);
            let credits = cards::play_cost(w, g, s, d);
            // 118: as the engine judges a play (a program counts its own tags: F183).
            let target = d.is_building() || effects::feasible(w, g, s, &effects::own_counted(d));
            let later = if pc.table {
                round > 0 && pile_ok(w, g, s, d, &planned) && (!f.plan_copy || copy_ok(w, g, s, d, &planned))
            } else {
                liberate_later(w, g, d)
            };
            let cubes = if f.plan_cubes { b.research_cubes } else { g.resource(s, Resource::ResearchCubes) };
            let fits = b.credits >= credits
                && b.insight >= cards::play_insight(w, g, s, d)
                && b.chips >= d.chips
                && b.energy >= d.energy
                && b.antimatter >= d.antimatter
                && cubes >= d.research_cubes
                && cards::lowers_ok(w, g, s, d)
                && (replaceable(w, g, s, d) || (f.plan_under && pc.table && under_planned(w, d, &planned)))
                && (target || later)
                && near(d)
                && d.needs.iter().all(|(t, n)| needs[*t as usize] >= *n);
            // 102 (D34): at the table a card adds its value less its price, and one
            // worth no more than it costs is no plan: the play check nets the price,
            // so spare credits kept cards it never played (099).
            let (v, price) = if fits {
                let v = value(w, g, s, d, f, pc, pt, round, &tags, &planned);
                let price = if pc.table {
                    // 102: credits over the heuristic's cap are worth its
                    // idle rate to it, so a price paid from them costs that much.
                    let (cap, share) = pc.idle;
                    let idle_paid = (credits as f64).min((b.credits as f64 - cap).max(0.0));
                    // 128: its energy at the seat's rate (spare energy is cheap).
                    let spare = d.energy as f64 * pc.t.res(Resource::Energy) * (1.0 - pc.energy_rate);
                    spend(w, g, s, d, pc, pt) - idle_paid * (1.0 - share) - spare
                } else {
                    0.0
                };
                (v, price)
            } else {
                (0.0, 0.0)
            };
            if fits && pc.table && v <= price {
                continue;
            }
            if fits {
                b.credits -= credits;
                b.insight -= cards::play_insight(w, g, s, d);
                b.chips -= d.chips;
                b.energy -= d.energy;
                b.antimatter -= d.antimatter;
                if f.plan_cubes {
                    b.research_cubes -= d.research_cubes;
                    b.research_cubes += d
                        .effects
                        .iter()
                        .map(|e| match e {
                            Op::Gain { resource: Resource::ResearchCubes, amount } => *amount,
                            _ => 0,
                        })
                        .sum::<i64>();
                }
                let found = if target || !pc.table { 1.0 } else { share };
                // 092: with every slot full, a plain building buries the top that
                // would earn least over the rounds left.
                let buried = if f.plan_bury && pc.table && d.is_building() && d.replaces.is_none() {
                    if open > 0 {
                        open -= 1;
                        0.0
                    } else {
                        let ctx = Ctx { share: pt.share, ..Ctx::at(pc.t, (pt.left - round as f64).max(0.0), pt.node, tags, pt.corrupted) };
                        let worst = (0..tops.len()).min_by(|&i, &j| {
                            buried_value(pc.t, tops[i], &ctx).total_cmp(&buried_value(pc.t, tops[j], &ctx))
                        });
                        match worst {
                            Some(i) => buried_value(pc.t, tops.swap_remove(i), &ctx),
                            None => 0.0,
                        }
                    }
                } else {
                    0.0
                };
                // 104 (Matt, 2026-11-04): a card worth no more than its price is
                // dropped above, and one that is adds its gross value (099's form).
                total += found * (v - buried) * (1.0 - WAIT * round as f64).max(0.1);
                planned.push(*id);
                if pc.table && !d.is_event() {
                    // 082: what it leaves in play counts toward later cards' needs.
                    for t in &d.tags {
                        needs[*t as usize] += 1;
                        tags[*t as usize] += 1;
                    }
                }
            }
        }
    }
    // 107 (`plan_chain`): the chain cards the plan builds toward but can't play
    // within its rounds count their value after it, times their share reached.
    if f.plan_chain && pc.table {
        let last = rounds.max(1);
        for id in &pool {
            let d = cards::def(w, id);
            if planned.contains(id) || d.needs.is_empty() || !committed(d) {
                continue;
            }
            let short: i64 = d.needs.iter().map(|(t, n)| (n - needs[*t as usize]).max(0)).sum();
            if short == 0 || short > f.plan_short as i64 {
                continue;
            }
            let want: i64 = d.needs.iter().map(|(_, n)| *n).sum();
            let share = (want - short) as f64 / want.max(1) as f64;
            let v = value(w, g, s, d, f, pc, pt, last, &tags, &planned) - spend(w, g, s, d, pc, pt);
            if v > 0.0 {
                total += share * share * v * (1.0 - WAIT * last as f64).max(0.1);
                planned.push(*id);
            }
        }
    }
    Some((total, planned))
}

/// 082: whether a card that works on a token pile of the seat's own (a feeder,
/// a cash-out, a count) has one to work on: a token card of its kind standing or
/// planned before it. A pile can't arrive from the board, as a target can.
fn pile_ok(w: &World, g: &GameState, s: usize, d: &CardDef, planned: &[Key]) -> bool {
    let kinds: Vec<aic_core::content::TokenKind> = d
        .effects
        .iter()
        .filter_map(|e| match e {
            Op::AddTokens { kind, here: false, .. } | Op::CashOut { kind, here: false, .. } => Some(*kind),
            Op::GainPerTokens { kind, here: false, .. } => Some(*kind),
            _ => None,
        })
        .collect();
    kinds.iter().all(|k| {
        cards::has_token_card(w, g, s, Some(*k))
            || planned.iter().any(|id| cards::def(w, id).tokens.as_ref().is_some_and(|t| t.kind == *k))
    })
}


/// 107 (`plan_copy`): whether a card that copies a building's income has one to
/// copy, built or planned before it.
fn copy_ok(w: &World, g: &GameState, s: usize, d: &CardDef, planned: &[Key]) -> bool {
    if !d.effects.iter().any(|e| matches!(e, Op::CopyIncome)) {
        return true;
    }
    let earns = |b: &CardDef| {
        b.is_building() && b.passives.iter().any(|p| matches!(p, Op::Income { .. } | Op::IncomePerTag { .. }))
    };
    g.pile(&cards::built(s)).iter().chain(planned).any(|id| earns(cards::def(w, id)))
}

/// 080: a liberate card with no corrupted city in reach now still plans to
/// play while a city is corrupted: a node can get there.
fn liberate_later(w: &World, g: &GameState, d: &CardDef) -> bool {
    is_liberate(d) && (0..g.hexes.len()).any(|h| common::corrupted_city(w, g, h))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn a_shield_counts_only_the_node_it_can_cover() {
        let r = crate::agents::tests::rules(0);
        let mut g = r.new_game(1);
        let w = r.world();
        let shield = cards::def(&w, "benchmark_suite");
        let open = (0..g.hexes.len()).filter(|&h| effects::shieldable(&g, 0, h)).count() as i64;
        assert!(open > 0);
        assert_eq!(niche_jobs(&w, &g, 0, shield), open.min(2));
        for h in Arc::make_mut(&mut g.hexes).iter_mut() {
            Arc::make_mut(h).tokens.clear();
        }
        assert_eq!(niche_jobs(&w, &g, 0, shield), 0);
    }
}

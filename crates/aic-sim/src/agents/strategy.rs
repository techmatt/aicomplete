//! 082: strategies. Each seat commits at its round-1 keep, from its board and its
//! opening 10, to one of its board's hand-written strategies
//! (`configs/agents/strategies.json`). A strategy is a weight vector over what a
//! card is (its tags, the resources and tokens it pays in or out, its kind, its
//! categories) and a phase schedule (early: income and engine pieces; mid: chain
//! pieces; late: capstones, liberation and purge). A card's multiplier for a seat
//! is 1 plus the weights it matches, times the schedule's for its categories.
//!
//! The choice is a pure function of the table at the round-1 keep, so a game
//! replays from its seed and actions, and a planner's rollouts are handed the
//! choice ([`super::planner::Planner`]).

use std::collections::BTreeMap;
use std::sync::OnceLock;

use aic_core::content::{CardDef, CardKind, Op, Power, Resource, TagOrAny, TokenKind};
use aic_core::module::World;
use aic_core::modules::{ai, cards};
use aic_core::GameState;
use serde::Deserialize;

/// The categories a strategy or the schedule can weight.
pub const CATEGORIES: [&str; 10] =
    ["income", "engine", "chain", "capstone", "liberate", "purge", "counter", "defense", "draw", "team"];

/// Each category's slot in a card's [`categories`], in [`CATEGORIES`] order.
mod cat {
    pub const INCOME: usize = 0;
    pub const ENGINE: usize = 1;
    pub const CHAIN: usize = 2;
    pub const CAPSTONE: usize = 3;
    pub const LIBERATE: usize = 4;
    pub const PURGE: usize = 5;
    pub const COUNTER: usize = 6;
    pub const DEFENSE: usize = 7;
    pub const DRAW: usize = 8;
    pub const TEAM: usize = 9;
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    /// `aic-sim strategies` prints `id`; docs/agents.md prints `name` and `says`
    /// (`docs/blocks.rs`); play reads the weights.
    pub id: String,
    pub board: String,
    pub name: String,
    pub says: String,
    #[serde(default)]
    tags: BTreeMap<String, f64>,
    #[serde(default)]
    resources: BTreeMap<String, f64>,
    #[serde(default)]
    tokens: BTreeMap<String, f64>,
    #[serde(default)]
    kinds: BTreeMap<String, f64>,
    #[serde(default)]
    categories: BTreeMap<String, f64>,
}

impl Spec {
    /// 107: the weight this strategy puts on tag `t` (0: none).
    pub fn tag_weight(&self, t: aic_core::content::Tag) -> f64 {
        self.tags.get(t.name()).copied().unwrap_or(0.0)
    }
}

#[derive(Debug, Deserialize)]
struct Schedule {
    early_through: u32,
    late_from: u32,
    early: BTreeMap<String, f64>,
    mid: BTreeMap<String, f64>,
    late: BTreeMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub struct Strategies {
    #[allow(dead_code)]
    note: String,
    schedule: Schedule,
    pub strategies: Vec<Spec>,
}

const SOURCE: &str = include_str!("../../../../configs/agents/strategies.json");

pub fn strategies() -> &'static Strategies {
    static S: OnceLock<Strategies> = OnceLock::new();
    S.get_or_init(|| serde_json::from_str(SOURCE).expect("configs/agents/strategies.json parses"))
}

/// A seat's strategy: its index in [`Strategies::strategies`].
pub type Choice = [Option<u16>; 4];

/// The phase of round `round`: 0 early, 1 mid, 2 late.
pub fn phase(round: u32) -> usize {
    let s = &strategies().schedule;
    if round <= s.early_through {
        0
    } else if round >= s.late_from {
        2
    } else {
        1
    }
}

/// 124 (D47, `w.strat_contents`): `ops`, and with `contents` the ops a
/// trigger or a threshold among them resolves ("a trigger that pays credits is a
/// credits card").
fn with_contents<'a>(ops: impl Iterator<Item = &'a Op>, contents: bool) -> Vec<&'a Op> {
    let mut out = Vec::new();
    for o in ops {
        out.push(o);
        if contents {
            if let Op::When { then, .. } | Op::Threshold { then, .. } = o {
                out.extend(then.iter());
            }
        }
        // 158: a "with N X tags, also" rider is the card's own effect, read always.
        if let Op::IfTags { then, .. } = o {
            out.extend(then.iter());
        }
    }
    out
}

/// Which of [`CATEGORIES`] card `d` falls in (124: with `contents`, its
/// triggers' and thresholds' ops read too).
pub fn categories(d: &CardDef, contents: bool) -> [bool; CATEGORIES.len()] {
    let mut c = [false; CATEGORIES.len()];
    let ability: Vec<&Op> = d.ability.iter().flat_map(|a| a.branches()).flat_map(|b| &b.effects).collect();
    let all: Vec<&Op> = with_contents(d.effects.iter().chain(&d.passives).chain(ability.iter().copied()), contents);
    let any = |f: &dyn Fn(&Op) -> bool| all.iter().any(|o| f(o));
    op_categories(&all, &mut c);
    c[cat::ENGINE] = d.is_building() && (d.ability.is_some() || d.tokens.is_some() || any(&|o| matches!(o, Op::ConvertUpkeep { .. } | Op::Slots { .. })));
    c[cat::CHAIN] = !d.needs.is_empty();
    c[cat::CAPSTONE] = super::policy::is_capstone(d) || (d.tier == 2 && d.antimatter > 0);
    c[cat::LIBERATE] = super::policy::is_liberate(d);
    c
}

/// The categories ops alone decide (income, purge, counter, defense, draw,
/// team), for a card or (092) a board power.
fn op_categories(all: &[&Op], c: &mut [bool; CATEGORIES.len()]) {
    let any = |f: &dyn Fn(&Op) -> bool| all.iter().any(|o| f(o));
    c[cat::INCOME] = any(&|o| matches!(o, Op::Income { .. } | Op::IncomePerTag { .. } | Op::Production { .. } | Op::ProductionPerTag { .. } | Op::BuildingDiscount { .. }));
    c[cat::PURGE] = any(&|o| {
        matches!(
            o,
            Op::Damage { .. }
                | Op::DamageCountry { .. }
                | Op::DestroyAll
                | Op::RemoveBotnet
                | Op::RemoveBotnets { .. }
                | Op::RemoveAmbassadorsCountry { .. }
                | Op::RemoveTroops { .. }
                | Op::ClearCity { .. }
                | Op::Pushback { .. }
                | Op::RemoveWhere { .. }
                | Op::Convert { .. }
                | Op::Gather { .. }
                | Op::Push { .. }
        )
    });
    c[cat::COUNTER] = any(&|o| matches!(o, Op::Jam { .. } | Op::CounterThreat | Op::AutoCounter { .. } | Op::DelayThreat { .. }));
    c[cat::DEFENSE] = any(&|o| {
        matches!(
            o,
            Op::Shelter
                | Op::Cordon
                | Op::Firewall
                | Op::Prohibit { .. }
                | Op::Harden { .. }
                | Op::HardenAction { .. }
                | Op::ShieldNode { .. }
                | Op::CivicShelter
                | Op::CorruptionShield
        )
    });
    c[cat::DRAW] = any(&|o| matches!(o, Op::Draw { .. }));
    c[cat::TEAM] = any(&|o| matches!(o, Op::OtherGain { .. } | Op::SupplyInsight { .. } | Op::Sway { .. } | Op::TeamIncome { .. }));
}

/// The resources card `d` pays out or (beyond credits and insight) costs.
fn resources(d: &CardDef, contents: bool) -> Vec<Resource> {
    let ability: Vec<&Op> = d.ability.iter().flat_map(|a| a.branches()).flat_map(|b| &b.effects).collect();
    let mut out =
        op_resources(with_contents(d.effects.iter().chain(&d.passives).chain(ability.iter().copied()), contents).into_iter());
    for (r, n) in [
        (Resource::Energy, d.energy),
        (Resource::Chips, d.chips),
        (Resource::Antimatter, d.antimatter),
        (Resource::ResearchCubes, d.research_cubes),
    ] {
        if n > 0 {
            out.push(r);
        }
    }
    out.sort();
    out.dedup();
    out
}

/// The resources ops pay out.
fn op_resources<'a>(ops: impl Iterator<Item = &'a Op>) -> Vec<Resource> {
    ops
        .filter_map(|o| match o {
            Op::Gain { resource, .. }
            | Op::GainPerNode { resource }
            | Op::Production { resource, .. }
            | Op::ProductionPerTag { resource, .. }
            | Op::Income { resource, .. }
            | Op::IncomePerTag { resource, .. }
            | Op::GainPerTokens { resource, .. }
            | Op::CashOut { resource, .. }
            | Op::GainPerTag { resource, .. } => Some(*resource),
            Op::GainAmbassadors { .. } => Some(Resource::Ambassadors),
            Op::ProjectCube { .. } | Op::BuildStation => Some(Resource::ResearchCubes),
            _ => None,
        })
        .collect()
}

/// The token kinds card `d` holds or feeds.
fn token_kinds(d: &CardDef, contents: bool) -> Vec<TokenKind> {
    let mut out: Vec<TokenKind> = d.tokens.iter().map(|t| t.kind).collect();
    let ops = if contents { with_contents(d.effects.iter().chain(&d.passives), true) } else { d.effects.iter().collect() };
    out.extend(ops.into_iter().filter_map(|o| match o {
        Op::AddTokens { kind, .. } => Some(*kind),
        _ => None,
    }));
    out.sort();
    out.dedup();
    out
}

fn kind_name(k: CardKind) -> &'static str {
    match k {
        CardKind::Building => "building",
        CardKind::Program => "program",
        CardKind::Event => "event",
    }
}

impl Spec {
    /// 1 plus the weights card `d` matches (124: `contents`, as [`categories`]).
    pub fn fit(&self, d: &CardDef, contents: bool) -> f64 {
        let get = |m: &BTreeMap<String, f64>, k: &str| m.get(k).copied().unwrap_or(0.0);
        let mut f = 1.0;
        f += d.tags.iter().map(|t| get(&self.tags, t.name())).sum::<f64>();
        f += resources(d, contents).iter().map(|r| get(&self.resources, r.name())).sum::<f64>();
        f += token_kinds(d, contents).iter().map(|k| get(&self.tokens, k.name())).sum::<f64>();
        f += get(&self.kinds, kind_name(d.kind));
        let cats = categories(d, contents);
        f += CATEGORIES.iter().zip(cats).filter(|(_, on)| *on).map(|(c, _)| get(&self.categories, c)).sum::<f64>();
        f
    }

    /// 092: 1 plus the weights board power `p` matches, read as a card would be:
    /// the tags it sacrifices, the resources it pays out or costs, and the
    /// categories of its effects.
    pub fn fit_power(&self, p: &Power) -> f64 {
        let get = |m: &BTreeMap<String, f64>, k: &str| m.get(k).copied().unwrap_or(0.0);
        let mut f = 1.0;
        f += p
            .tags
            .iter()
            .filter_map(|(t, _)| match t {
                TagOrAny::Is(t) => Some(get(&self.tags, t.name())),
                TagOrAny::Any => None,
            })
            .sum::<f64>();
        let mut res = op_resources(p.effects.iter());
        res.extend(p.cost.iter().map(|(r, _)| *r).filter(|r| !matches!(r, Resource::Credits | Resource::Insight | Resource::Energy)));
        res.sort();
        res.dedup();
        f += res.iter().map(|r| get(&self.resources, r.name())).sum::<f64>();
        let ops: Vec<&Op> = p.effects.iter().collect();
        let mut cats = [false; CATEGORIES.len()];
        op_categories(&ops, &mut cats);
        // A power is no building, chain card, capstone or liberate card.
        f += CATEGORIES
            .iter()
            .zip(cats)
            .enumerate()
            .filter(|(i, (_, on))| *on && !(1..=4).contains(i))
            .map(|(_, (c, _))| get(&self.categories, c))
            .sum::<f64>();
        f
    }

    /// Whether every weight names a tag, resource, token kind, kind or category.
    #[cfg(test)]
    pub fn check(&self) -> Result<(), String> {
        let tags: Vec<&str> = aic_core::content::TAGS.iter().map(|t| t.name()).collect();
        let res = ["credits", "chips", "energy", "antimatter", "insight", "research_cubes", "ambassadors"];
        let toks = ["datasets", "followers", "capital", "agents"];
        let kinds = ["building", "program", "event"];
        for (m, ok) in [
            (&self.tags, &tags[..]),
            (&self.resources, &res[..]),
            (&self.tokens, &toks[..]),
            (&self.kinds, &kinds[..]),
            (&self.categories, &CATEGORIES[..]),
        ] {
            if let Some(k) = m.keys().find(|k| !ok.contains(&k.as_str())) {
                return Err(format!("strategy {}: unknown weight {k:?}", self.id));
            }
        }
        Ok(())
    }
}

/// The schedule's multiplier for card `d` in phase `p`: the largest of its
/// categories' (1 with none).
pub fn schedule(d: &CardDef, p: usize, contents: bool) -> f64 {
    let s = &strategies().schedule;
    let m = [&s.early, &s.mid, &s.late][p];
    let cats = categories(d, contents);
    let hits: Vec<f64> = CATEGORIES.iter().zip(cats).filter(|(_, on)| *on).filter_map(|(c, _)| m.get(*c).copied()).collect();
    if hits.is_empty() {
        1.0
    } else {
        hits.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    }
}

/// Each strategy's multiplier for each card in each phase, worked out once for a
/// game's content: `[strategy][card][phase]`.
/// 116: `scale` (the heuristic's `strat_scale`, 1 by default) stretches each
/// multiplier's distance from 1: `1 + scale × (m − 1)`; at 0 every multiplier is
/// one. Only the multipliers read it: the choice itself (`choose`) and the keep
/// plan's `plan_reach` and `plan_chain` (policy.rs, `committed`) still follow
/// the chosen strategy's tags, so 0 is not quite `strategy=0`.
#[derive(Debug)]
pub struct Mults(Vec<Vec<[f64; 3]>>);

impl Mults {
    pub fn new(w: &World, scale: f64, contents: bool) -> Mults {
        let list = &strategies().strategies;
        // At 1 the multiplier as it was, bit for bit (1 + (m − 1) can round).
        let scaled = |m: f64| if scale == 1.0 { m } else { 1.0 + scale * (m - 1.0) };
        Mults(
            list.iter()
                .map(|s| {
                    w.content
                        .cards
                        .iter()
                        .map(|d| {
                            let f = s.fit(d, contents);
                            [0, 1, 2].map(|p| scaled(f * schedule(d, p, contents)))
                        })
                        .collect()
                })
                .collect(),
        )
    }

    /// Strategy `s`'s multiplier for the card at content index `card` in `phase`.
    pub fn get(&self, s: u16, card: usize, phase: usize) -> f64 {
        self.0[s as usize][card][phase]
    }
}

/// The strategies open to seat `s`: its board's, in file order.
pub fn menu(w: &World, s: usize) -> Vec<u16> {
    let board = &ai::board(w, s).id;
    strategies().strategies.iter().enumerate().filter(|(_, x)| &x.board == board).map(|(i, _)| i as u16).collect()
}

/// 092: a board power's worth to the strategy choice: one use's worth, and the
/// uses to count (`None`: powers aren't read).
pub type PowerWorth<'a> = Option<&'a dyn Fn(&Power) -> (f64, f64)>;

/// Seat `s`'s strategy from its board and the cards in front of it (the opening
/// deal, its hand): the one whose multipliers add most to what the cards are worth.
/// `fixed` (1-based in the board's menu) overrides the choice.
/// 092 (`power`, with `strat_powers`): each board power adds its use's worth
/// times its fit, times the uses counted; a power that sacrifices a named tag
/// is used at most once per card seen carrying it.
pub fn choose(
    w: &World,
    g: &GameState,
    s: usize,
    fixed: usize,
    value: &dyn Fn(&CardDef) -> f64,
    power: PowerWorth,
    contents: bool,
) -> Option<u16> {
    let menu = menu(w, s);
    if fixed > 0 {
        return menu.get(fixed - 1).copied().or(menu.first().copied());
    }
    let list = &strategies().strategies;
    let seen: Vec<&CardDef> = g
        .pile(&cards::drafted(s))
        .iter()
        .chain(g.pile(&cards::hand(s)).iter())
        .map(|id| cards::def(w, id))
        .collect();
    let mut best: Option<(f64, u16)> = None;
    for &k in &menu {
        let spec = &list[k as usize];
        let mut score: f64 = seen.iter().map(|d| value(d).max(0.0) * (spec.fit(d, contents) * schedule(d, phase(g.round), contents) - 1.0)).sum();
        if let Some(power) = power {
            for p in &ai::board(w, s).powers {
                let (worth, rounds) = power(p);
                let named: Vec<_> = p.tags.iter().filter_map(|(t, _)| match t {
                    TagOrAny::Is(t) => Some(*t),
                    TagOrAny::Any => None,
                }).collect();
                let uses = if named.is_empty() {
                    rounds
                } else {
                    rounds.min(seen.iter().filter(|d| d.tags.iter().any(|t| named.contains(t))).count() as f64)
                };
                score += worth.max(0.0) * uses * (spec.fit_power(p) - 1.0);
            }
        }
        if best.is_none_or(|(b, _)| score > b + 1e-9) {
            best = Some((score, k));
        }
    }
    best.map(|(_, k)| k)
}

/// The strategy named `id`.
#[cfg(test)]
pub fn by_id(id: &str) -> Option<u16> {
    strategies().strategies.iter().position(|s| s.id == id).map(|i| i as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The named slots match [`CATEGORIES`].
    #[test]
    fn category_slots_match_their_names() {
        let slots = [
            (cat::INCOME, "income"),
            (cat::ENGINE, "engine"),
            (cat::CHAIN, "chain"),
            (cat::CAPSTONE, "capstone"),
            (cat::LIBERATE, "liberate"),
            (cat::PURGE, "purge"),
            (cat::COUNTER, "counter"),
            (cat::DEFENSE, "defense"),
            (cat::DRAW, "draw"),
            (cat::TEAM, "team"),
        ];
        assert_eq!(slots.len(), CATEGORIES.len());
        for (i, name) in slots {
            assert_eq!(CATEGORIES[i], name);
        }
    }

    /// Every strategy reads, names a board and weights only what exists; every
    /// board has a menu.
    #[test]
    fn strategies_read() {
        let r = crate::agents::tests::rules(2);
        let w = r.world();
        for s in &strategies().strategies {
            s.check().unwrap();
            assert!(w.content.ais.iter().any(|b| b.id == s.board), "{}: no board {}", s.id, s.board);
            assert!(!s.says.is_empty());
        }
        for s in 0..3 {
            assert!(menu(&w, s).len() >= 3, "seat {s}");
        }
        // docs/agents.md's lines are `aic-sim docs`'s `strategies` block (119), checked by `docs --check`.
        let m = Mults::new(&w, 1.0, false);
        let payroll = w.content.card_index("payroll_automation").unwrap();
        let k = by_id("industry_income").unwrap();
        assert!(m.get(k, payroll, 0) > m.get(by_id("power_grid").unwrap(), payroll, 0));
    }

    /// 092: a board power reads as a card would: Shelter fits Civic shelter,
    /// Grant and Peer Review (paid in cubes) fit Cubes and stations, and a
    /// strategy weights nothing a power doesn't do.
    #[test]
    fn powers_fit_their_strategies() {
        let r = crate::agents::tests::rules(3);
        let w = r.world();
        let list = &strategies().strategies;
        let power = |board: &str, name: &str| {
            let b = w.content.ais.iter().find(|b| b.id == board).unwrap();
            b.powers.iter().find(|p| p.name == name).unwrap().clone()
        };
        let fit = |id: &str, p: &Power| list[by_id(id).unwrap() as usize].fit_power(p);
        let shelter = power("protector", "Shelter");
        assert!(fit("civic_shelter", &shelter) > fit("diplomacy_liberation", &shelter));
        for name in ["Grant", "Peer Review"] {
            let p = power("researcher", name);
            assert!(fit("cubes_stations", &p) > 1.0, "{name}");
            assert!(fit("cubes_stations", &p) > fit("science_chain", &p), "{name}");
        }
    }
}

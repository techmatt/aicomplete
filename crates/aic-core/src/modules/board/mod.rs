//! The board (014, 016): the printed country pieces (cities, open land,
//! ocean, industrial zones; military bases on the fronts), player presence
//! (owned here; AI boards read the track through `track_taken`), the setup,
//! the production/war clock, production (factories grow from cities and
//! make corp units and chips, front battles on war steps, adversary
//! troops, cities, stations, megaprojects), player ambassadors and site
//! claims, the standard actions and the round-end cleanup.

use crate::action::Action;
use crate::content::{Op, RoundFlag, Side};
use crate::key::KeyBuf;
use crate::module::{
    scheduled, Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::state::GameState;
use crate::view::ViewOut;

use super::common::*;
use super::{adversary, ai, alignment, cards, economy};

mod ambassadors;
mod destroy;
mod presence;
mod production;
mod standard;
mod troops;
mod view;

pub use ambassadors::*;
pub use destroy::*;
pub use presence::*;
use production::*;
pub use standard::*;
pub use troops::*;
use view::*;

pub static DEF: ModuleDef = ModuleDef {
    id: "board",
    summary: "Country pieces, cities, buildings, bases and fronts; production, war, ambassadors, standard actions",
    requires: &["adversary", "ai", "alignment"],
    variants: &[VariantDef {
        name: "corps",
        summary: "Each country is a printed piece with cities, a research station, factories and a corp's bases",
        params: &[
            ParamSpec::int("start_corrupted", 0, 3, "cities per country that start corrupted"),
            ParamSpec::int("start_ambassadors", 0, 10, "player ambassadors each player starts with"),
            ParamSpec::int("unit_cap", 0, 20, "corp units a base holds at most"),
            ParamSpec::int("factory_units", 0, 5, "corp units each factory makes on a production step at each of its corp's bases"),
            ParamSpec::list("clock", 0, 3, "the printed round track, per round: 1 production, 2 war, 3 both (the last entry repeats)"),
            ParamSpec::list("corp_energy", 0, 10, "energy cubes each city of country i makes a round (a megacorp parameter; the list repeats)"),
            ParamSpec::int("city_energy_credits", 0, 20, "credits for a neutral city's energy cube, bought by a player with presence next to it"),
            ParamSpec::int("aligned_bonus", 0, 10, "extra energy an aligned city gives its owner"),
            ParamSpec::int("station_cubes", 0, 5, "megaproject cubes a research station makes a round"),
            ParamSpec::int("project_credits", 0, 50, "credits each player cube pays when a megaproject completes"),
            ParamSpec::int("project_discord_per", 1, 10,"adversary cubes per Discord when a megaproject completes"),
            ParamSpec::int("divert_discord", 0, 5, "Discord per unit of a corrupted city's energy diverted to the adversary"),
            ParamSpec::int("ambassador_discord", 0, 5, "Discord per unit of energy an adversary ambassador on a clean or aligned city diverts"),
            ParamSpec::int("base_discord", 0, 10, "Discord an adversary ambassador on a producing base sways"),
            ParamSpec::int("base_unity", 0, 10, "Unity a player ambassador on a producing base sways"),
            ParamSpec::int("factory_unity", 0, 10, "Unity a player ambassador on a producing factory sways"),
            ParamSpec::int("hold_base", 0, 10, "a corrupted city's hold before adversary ambassadors: 1 more for each on or next to it"),
            ParamSpec::int("last_safe_stage", 0, 3, "while this round's adversary card is below this stage, nothing destroys a player's last presence (0: never safe)"),
            ParamSpec::int("hold_min", 1, 10, "player ambassadors that free a corrupted city, at the least"),
            ParamSpec::int("free_leaves", 0, 5, "adversary ambassadors left on a city as it frees (the piece ladder)"),
            ParamSpec::int("destruction_discord", 0, 5, "Discord each building destroyed in war sways"),
            ParamSpec::int("troop_spent", 0, 1, "1: an adversary troop that strikes a target is used up"),
            ParamSpec::int("troop_steps", 0, 5, "steps each adversary troop takes in the march: destroy a target on or next to it, else move 1 hex toward the nearest"),
            ParamSpec::int("troop_presence", 0, 4, "adversary troops strike presence from this stage of adversary card on (0: always, 4: never)"),
            ParamSpec::int("presence_marker", 0, 1, "1: a destroyed presence leaves a corruption marker on its hex"),
            ParamSpec::int("corruption_reach", 0, 1, "a city that corrupts destroys presence within this many hexes of it"),
            ParamSpec::int("war_reach", 0, 1, "war that destroys a factory destroys presence within this many hexes of it"),
            ParamSpec::int("presence_energy", 0, 20, "standard action: energy to place a presence, on top of the track step's cost"),
            ParamSpec::int("presence_action_cap", 0, 5, "standard action: placements a seat may make with it a round (0: no limit; the rich action has its own rise)"),
            ParamSpec::int("build_energy", 0, 20, "standard action: energy to build a research station"),
            ParamSpec::int("station_chips", 0, 20, "standard action: chips to build a research station"),
            ParamSpec::int("factory_chips", 0, 5, "chips a player-owned factory makes its owner on a production step"),
            ParamSpec::int("chips_credits", 0, 50, "standard action: credits for selling 1 chip"),
            ParamSpec::int("remove_energy", 0, 20, "standard action: energy to remove an adversary ambassador within reach"),
            ParamSpec::int("clear_energy", 0, 20, "standard action: energy to clear a corruption marker within reach"),
            ParamSpec::int("clear_unity", 0, 5, "Sway toward Unity for clearing a corruption marker"),
            ParamSpec::int("clear_credits", 0, 5, "credits back for clearing a corruption marker"),
            ParamSpec::int("move_energy", 0, 20, "standard action: energy to move a player ambassador"),
            ParamSpec::int("ambassador_credits", 1, 50, "standard action: credits to gain a player ambassador (at least 1: a free untargeted action has no limit)"),
            ParamSpec::int("harden_energy", 1, 20, "standard action (Harden passive): energy a hardening token (at least 1: free, it has no limit)"),
            ParamSpec::int("sell_credits", 0, 20, "standard action: credits for selling a card from hand"),
            ParamSpec::int("align_energy", 0, 20, "standard action: energy (and 1 ambassador) to align a clean city"),
            ParamSpec::int("presence_credits", 1, 50, "the rich action: credits to place a presence, before its rise and the step's cost"),
            ParamSpec::int("presence_rise", 0, 20, "the rich action: credits more for each earlier use this round"),
            ParamSpec::int("presence_stage", 1, 3, "the rich action: from this stage of adversary card on"),
            ParamSpec::int("harden_credits", 1, 50, "standard project: credits for a hardening token"),
            ParamSpec::int("energy_credits", 1, 50, "standard project: credits for 1 energy"),
            ParamSpec::int("drain_credits", 1, 50, "standard project: credits to drain compute from an adversary project"),
            ParamSpec::int("drain_amount", 1, 20, "standard project: compute drained"),
        ],
        phases: &[
            PhaseSpec::decision("act", "standard actions and projects: presence, rich presence, station, remove, clear, move, ambassador, align, harden, chips, sell, buy energy, drain"),
            PhaseSpec::auto("bases", "on a production step: bases and factories make corp units and chips, cities grow factories"),
            PhaseSpec::auto("war", "on a war step, the corp units on either side of each front fight"),
            PhaseSpec::auto("march", "each adversary troop takes its steps: destroy a target next to it, else move toward the nearest"),
            PhaseSpec::auto("produce", "cities and research stations produce; ambassadors divert them, and one on a lone presence drains its owner's insight into adversary compute"),
            PhaseSpec::auto("cleanup", "remove the round's country flags"),
        ],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

/// Country flags: up while their var is above 0, counted down in cleanup (1 lasts to
/// the end of the round; a threat sets 2, to the end of the next).
pub const FLAGS: [RoundFlag; 4] =
    [RoundFlag::Blackout, RoundFlag::Sabotage, RoundFlag::Shelter, RoundFlag::Embargo];

/// The var key of a country flag.
pub fn flag(f: RoundFlag, country: usize) -> String {
    format!("board.{}.{country}", f.name())
}

/// A country flag's count: above 0 while it is up.
pub fn flag_level(g: &GameState, f: RoundFlag, country: usize) -> i64 {
    g.var(&flag(f, country))
}

/// Whether a country flag is up.
pub fn flagged(g: &GameState, f: RoundFlag, country: usize) -> bool {
    g.var(&flag(f, country)) > 0
}

/// Shelters `country`'s buildings and presence for the rest of the round.
pub fn shelter(g: &mut GameState, cx: &mut Cx, country: usize) {
    cx.set_key(g, &flag(RoundFlag::Shelter, country), 1);
}

/// Cubes on country `c`'s megaproject: `None` for the adversary's.
pub fn project_key(c: usize, owner: Option<usize>) -> KeyBuf {
    let k = KeyBuf::new("board.project.").num(c).text(".");
    match owner {
        Some(s) => k.num(s),
        None => k.text("adversary"),
    }
}

/// Set once country `c`'s megaproject has completed.
pub fn project_done(c: usize) -> KeyBuf {
    KeyBuf::new("board.project_done.").num(c)
}

/// The clock's step for round `round`: 1 production, 2 war, 3 both.
fn clock_step(w: &World, round: u32) -> i64 {
    scheduled(w.list("board.clock"), round)
}

/// Whether this round has a production step (factories, troops, chips).
pub fn production_round(w: &World, g: &GameState) -> bool {
    clock_step(w, g.round) & 1 != 0
}

/// Whether this round is a war step (the clock, or this round's adversary card).
pub fn war_round(w: &World, g: &GameState) -> bool {
    clock_step(w, g.round) & 2 != 0 || adversary::war_twist(w, g)
}

#[derive(Debug)]
struct Board;

fn build(_b: &Build) -> Result<Box<dyn Module>, String> {
    Ok(Box::new(Board))
}

impl Module for Board {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        let mut corrupt = w.int("board.start_corrupted");
        if adversary::level(&w, 4) {
            corrupt += w.int("adversary.extra_corrupted");
        }
        for c in 0..w.map.countries.len() {
            let piece = &w.content.map.countries[c];
            let at = |n: u8| w.map.numbered(c, n);
            for &n in piece.corrupted.iter().take(corrupt as usize) {
                cx.add_tokens(g, at(n), ADV_CUBE, 1);
            }
            for &n in &piece.stations {
                cx.add_tokens(g, at(n), STATION, 1);
                cx.add_tokens(g, at(n), ADV_CUBE, 1);
            }
            for &n in &piece.factories {
                cx.add_tokens(g, at(n), FACTORY, 1);
            }
        }
        for s in 0..g.seats.len() {
            let c = home_country(&w, s);
            let n = w.content.map.countries[c].presence;
            cx.add_tokens(g, w.map.numbered(c, n), &presence(s), 1);
            cx.set_key(g, &track_key(s), 1);
            cx.add_resource(g, s, Resource::Ambassadors, w.int("board.start_ambassadors"));
        }
    }

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        let s = seat as usize;
        for act in STANDARD {
            if !seat_price(w, g, s, act).affordable(g, s) {
                continue;
            }
            if !std_targets(w, g, s, act).is_empty() {
                out.push(Action::Standard {
                    seat,
                    act: act.name().to_string(),
                });
            }
        }
        for id in g.pile(&cards::hand(s)) {
            out.push(Action::Sell {
                seat,
                card: id.clone(),
            });
        }
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Standard { .. } | Action::Sell { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        match a {
            Action::Sell { seat, card } => {
                let s = *seat as usize;
                let hand = cards::hand(s);
                if !g.pile(&hand).contains(card) {
                    return Err(Halt::Illegal("not in your hand".into()));
                }
                cards::discard(g, cx, card, &hand);
                cx.add_resource(g, s, Resource::Credits, w.int("board.sell_credits"));
                cx.tally(g, "board.std.sell", 1);
                Ok(())
            }
            Action::Standard { seat, act } => {
                let s = *seat as usize;
                let Some(act) = StandardAction::parse(act) else {
                    return Err(Halt::Illegal("no such standard action".into()));
                };
                let targets = std_targets(&w, g, s, act);
                let price = seat_price(&w, g, s, act);
                if !price.affordable(g, s) || targets.is_empty() {
                    return Err("can't take that standard action now".into());
                }
                if !act.pays_on_placing() {
                    price.pay(g, cx, s);
                }
                cx.tally(g, &format!("board.std.{}", act.name()), 1);
                cx.tally(g, "board.std_actions", 1);
                std_resolve(g, cx, s, act, &targets)
            }
            _ => Err(Halt::Illegal("not a standard action".into())),
        }
    }

    fn after(&self, g: &mut GameState, cx: &mut Cx) -> Res {
        free_cities(g, cx);
        Ok(())
    }

    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            cx.set_key(g, &rich_key(s), 0);
            cx.set_key(g, &placed_key(s), 0);
        }
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        match phase {
            1 => produce_units(g, cx)?,
            2 => battle(g, cx),
            3 => march(g, cx)?,
            4 => produce(g, cx)?,
            _ => cleanup(g, cx),
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        board_view(g, w, out);
    }
}

/// The round-end cleanup: flags count down, then the end-of-round tallies.
fn cleanup(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    // A flag counts down: 1 (set this round) clears, 2 (a threat
    // executed after this round's production) lasts the next round.
    for c in 0..w.map.countries.len() {
        for f in FLAGS {
            let k = flag(f, c);
            let v = g.var(&k);
            if v > 0 {
                cx.set_key(g, &k, v - 1);
            }
        }
    }
    // Rounds that end with only nodes between the players and the
    // purge (019: nodes are hard to remove).
    let level = alignment::bar_level(&w, g);
    let nodes = adversary::node_count(g, None);
    cx.peak(g, "adversary.nodes_peak", nodes);
    if level < 3 && nodes > 0 && adversary::purge_count(&w, g, level) == nodes {
        cx.tally(g, "alignment.purge_blocked_by_nodes", 1);
    }
    // Factories per side, as the game ends (the last round's level).
    let mut sides = [0i64; 3];
    for h in 0..g.hexes.len() {
        if g.tokens(h, FACTORY) > 0 {
            let i = if corrupted(g, h) {
                1
            } else if cube_owner(g, h).is_some() {
                0
            } else {
                2
            };
            sides[i] += 1;
        }
    }
    for (i, side) in ["player", "adversary", "neutral"].iter().enumerate() {
        cx.level(g, &format!("board.factories_end.{side}"), sides[i]);
    }
}

//! The board (014, 016): the printed country pieces (cities, open land,
//! ocean, industrial zones; military bases on the fronts), player node
//! (owned here; AI boards read the track through `track_taken`), the setup,
//! production every round (factories grow from cities and make chips or
//! adversary troops, cities, stations, megaprojects), war at the end of every round, player ambassadors and the one site
//! model (claim, liberate, block, armor), the standard actions and the round-end cleanup.

use crate::action::Action;
use crate::content::{Op, RoundFlag, Side};
use crate::key::KeyBuf;
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::state::GameState;
use crate::view::ViewOut;

use super::common::*;
use super::{adversary, ai, alignment, cards, economy, effects};

mod ambassadors;
mod destroy;
mod node;
mod production;
mod standard;
mod troops;
mod view;

pub use ambassadors::*;
pub use destroy::*;
pub use node::*;
pub use production::{corp_bases, corp_troops_at, tip_corp_troops, LEFT};
use production::*;
pub use standard::*;
pub use troops::*;
use view::*;

pub static DEF: ModuleDef = ModuleDef {
    id: "board",
    summary: "Country pieces, cities, facilities, bases and fronts; production, war, ambassadors, standard actions",
    requires: &["adversary", "ai", "alignment"],
    variants: &[VariantDef {
        name: "corps",
        summary: "Each country is a printed piece with cities, a research station, factories and a corp's bases",
        params: &[
            ParamSpec::int("start_corrupted", 0, 3, "cities per country that start corrupted"),
            ParamSpec::int("start_ambassadors", 0, 10, "player ambassadors each player starts with"),
            ParamSpec::int("unit_cap", 0, 20, "corp units a base holds at most"),
            ParamSpec::list("corp_energy", 0, 10, "energy cubes each city of country i makes a round (a megacorp parameter; the list repeats)"),
            ParamSpec::int("city_energy_credits", 0, 20, "credits for a neutral city's energy cube, bought by a player with a node next to it"),
            ParamSpec::int("aligned_bonus", 0, 10, "extra energy an aligned city gives its owner"),
            ParamSpec::int("station_cubes", 0, 5, "research cubes a neutral station makes a round (an adversary one: compute)"),
            ParamSpec::int("owned_station_cubes", 0, 5, "research cubes a player-owned station makes its owner a round (142)"),
            ParamSpec::int("divert_discord", 0, 5, "Discord per unit of a corrupted city's energy diverted to the adversary"),
            ParamSpec::int("ambassador_discord", 0, 5, "Discord per unit of energy, each round, of an aligned city an agent darkens"),
            ParamSpec::int("liberate_at", 1, 10, "the Liberate action: ambassadors that liberate an adversary city"),
            ParamSpec::int("liberate_facility_at", 1, 10,"the Liberate action: ambassadors that liberate an adversary station or factory"),
            ParamSpec::int("claim_city", 1, 10, "the Claim action: ambassadors that claim a neutral city (the adversary's corrupt_at)"),
            ParamSpec::int("claim_station", 1, 10, "the Claim action: ambassadors that claim a neutral research station"),
            ParamSpec::int("claim_factory", 1, 10, "the Claim action: ambassadors that claim a neutral factory"),
            ParamSpec::int("last_safe_stage", 0, 3, "while this round's adversary card is below this stage, nothing destroys a player's last node (0: never safe)"),
            ParamSpec::int("free_leaves", 0, 5, "agents left on a city as it frees (the piece ladder)"),
            ParamSpec::int("destruction_discord", 0, 5, "Discord each facility destroyed in war sways"),
            ParamSpec::int("troop_spent", 0, 1, "1: an adversary troop that strikes a target is used up"),
            ParamSpec::int("troop_steps", 0, 5, "steps each adversary troop takes in the march: destroy a target on or next to it, else move 1 hex toward the nearest"),
            ParamSpec::int("troop_node", 0, 4, "adversary troops strike nodes from this stage of adversary card on (0: always, 4: never)"),
            ParamSpec::int("node_marker", 0, 1, "1: a destroyed node leaves a corruption marker on its hex"),
            ParamSpec::int("war_reach", 0, 1, "war that destroys a factory destroys nodes within this many hexes of it"),
            ParamSpec::int("station_chips", 0, 20, "standard action: chips to build a research station"),
            ParamSpec::int("factory_chips", 0, 5, "chips a player-owned factory makes its owner each round"),
            ParamSpec::int("remove_ambassadors", 1, 5, "standard action: your ambassadors from the supply that go onto a hex to evict its agents, and stay there"),
            ParamSpec::int("remove_count", 1, 3, "standard action: agents one removal takes off its hex, at most"),
            ParamSpec::int("clear_cost", 0, 20, "standard action: credits to clear a corruption marker within reach"),
            ParamSpec::int("clear_unity", 0, 5, "Sway toward Unity for clearing a corruption marker"),
            ParamSpec::int("clear_credits", 0, 5, "credits back for clearing a corruption marker"),
            ParamSpec::int("envoy_ambassadors", 0, 5, "standard action Envoy: your ambassadors from the supply, used up, to move 1 Discord off the Discord track (0: no such action)"),
            ParamSpec::int("ambassador_credits", 1, 50, "standard action: credits to gain a player ambassador (at least 1: a free untargeted action has no limit)"),
            ParamSpec::int("sell_credits", 0, 20, "standard action: credits for selling a card from hand"),
            ParamSpec::int("harden_credits", 1, 50, "standard project: credits for a hardening token"),
            ParamSpec::int("drain_credits", 1, 50, "standard project: credits to drain compute from an adversary project"),
            ParamSpec::int("drain_amount", 1, 20, "standard project: compute drained"),
            ParamSpec::int("cube_insight", 0, 5, "standard action, no limit a round: insight for 1 research cube (0: no such action)"),
        ],
        phases: &[
            PhaseSpec::decision("act", "standard actions and projects: station, remove, clear, move (from the round's Movement), ambassador, claim, liberate, harden, sell, drain; completed megaprojects' converters"),
            PhaseSpec::auto("bases", "production: factories make chips or adversary troops, extra corp troops, cities grow factories"),
            PhaseSpec::auto("war", "at the end of the round, the corp troops on either side of each front fight"),
            PhaseSpec::auto("march", "each adversary troop takes its steps: destroy a target next to it, else move toward the nearest"),
            PhaseSpec::auto("produce", "cities make energy (an agent on one stays and darkens it) and research stations produce (ambassadors divert them); one on a lone node drains its owner's insight into adversary compute; full megaprojects complete"),
            PhaseSpec::auto("cleanup", "count the country flags down"),
        ],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

/// Country flags: up while their var is above 0, counted down in cleanup (1 lasts to
/// the end of the round; a threat sets 2, to the end of the next).
pub const FLAGS: [RoundFlag; 5] =
    [RoundFlag::Blackout, RoundFlag::Sabotage, RoundFlag::Shelter, RoundFlag::Embargo, RoundFlag::Cordon];

/// The var key of a country flag.
pub fn flag(f: RoundFlag, country: usize) -> KeyBuf {
    KeyBuf::new("board.").text(f.name()).text(".").num(country)
}

/// Prefix of the hex flags (135: a threat's flag on the hexes near it).
pub const HEX_FLAGS: &str = "board.hflag.";

/// The var key of flag `f` on hex `h` (135), counted down with the country flags.
pub fn hex_flag(f: RoundFlag, h: usize) -> KeyBuf {
    KeyBuf::new(HEX_FLAGS).text(f.name()).text(".").num(h)
}

/// Whether flag `f` is up on hex `h`: on the hex, or on its country.
pub fn hex_flagged(w: &World, g: &GameState, f: RoundFlag, h: usize) -> bool {
    g.var(&hex_flag(f, h)) > 0 || flagged(g, f, w.map.hexes[h].country as usize)
}

/// Standing troops on hex `h` (135): they never march or strike. A removal
/// takes them down with the hex's troops (cleanup trues the count).
pub fn stand_key(h: usize) -> KeyBuf {
    KeyBuf::new(STAND).num(h)
}

const STAND: &str = "board.stand.";
const OCCUPIED: &str = "board.occupied.";
const EXTRA_WAR: &str = "board.extra_war.";
const NEW_FACTORY: &str = "board.new_factory.";
const DESTROYED_AT: &str = "board.destroyed_at.";
const LANDED_AT: &str = "board.landed_at.";
const LANDED_N: &str = "board.landed_n.";

/// The marks a round's end clears outright ([`expire_marks`]).
const ROUND_MARKS: [&str; 5] = [NEW_FACTORY, DESTROYED_AT, LANDED_AT, LANDED_N, MOVED];

/// 135: every per-round mark [`expire_marks`] counts down, trues or clears
/// (the view lists them as bookkeeping).
pub(super) const MARKS: [&str; 10] =
    [HEX_FLAGS, STAND, OCCUPIED, CORP_EXTRA, EXTRA_WAR, NEW_FACTORY, DESTROYED_AT, LANDED_AT, LANDED_N, MOVED];

/// How many of hex `h`'s adversary troops stand.
pub fn standing(g: &GameState, h: usize) -> i64 {
    g.var(&stand_key(h)).min(g.tokens(h, TROOP)).max(0)
}

/// Makes `n` more of hex `h`'s troops stand.
pub fn add_standing(g: &mut GameState, cx: &mut Cx, h: usize, n: i64) {
    let v = (standing(g, h) + n).min(g.tokens(h, TROOP));
    cx.set_key(g, &stand_key(h), v);
}

/// Set while a troop occupies the factory on hex `h` (135, Factory Occupation).
pub fn occupied_key(h: usize) -> KeyBuf {
    KeyBuf::new(OCCUPIED).num(h)
}

/// Whether the factory on hex `h` is occupied: it makes nothing.
pub fn occupied(g: &GameState, h: usize) -> bool {
    g.var(&occupied_key(h)) > 0 && g.tokens(h, TROOP) > 0
}

/// Extra corp troops base `base` makes each production step through round
/// `until` (135, Conscription): the var holds how many.
pub fn corp_extra_key(base: usize, until: u32) -> KeyBuf {
    KeyBuf::new(CORP_EXTRA).num(base).text(".").num(until as usize)
}

const CORP_EXTRA: &str = "board.corp_extra.";

/// Set to the round in which front `i` fights a second war step (135: Casus Belli).
pub fn extra_war_key(front: usize) -> KeyBuf {
    KeyBuf::new(EXTRA_WAR).num(front)
}

/// The factory a threat just built (135), plus 1: what "on the new factory" names.
pub fn new_factory_key(threat: &str) -> KeyBuf {
    KeyBuf::new(NEW_FACTORY).text(threat)
}

/// Set (hex plus 1) when a threat's facility destruction emptied its zone
/// (135): "on an empty zone ... instead" reads the zone as it was.
pub fn destroyed_key(threat: &str) -> KeyBuf {
    KeyBuf::new(DESTROYED_AT).text(threat)
}

/// The hex (plus 1) a threat's troops just landed on, and how many (135):
/// what "they march at once" moves.
pub fn landed_key(threat: &str) -> KeyBuf {
    KeyBuf::new(LANDED_AT).text(threat)
}

/// How many troops a threat just landed (see [`landed_key`]).
pub fn landed_count_key(threat: &str) -> KeyBuf {
    KeyBuf::new(LANDED_N).text(threat)
}

/// Var keys under `prefix` with their values, in key order.
fn keys_under(g: &GameState, prefix: &str) -> Vec<(String, i64)> {
    g.vars
        .from_key(prefix)
        .map_while(|(k, v)| k.starts_with(prefix).then(|| (k.as_str().to_string(), *v)))
        .collect()
}

/// The registered extra corp troops (135): (base, until, count).
pub(super) fn corp_extras(g: &GameState) -> Vec<(usize, u32, i64)> {
    keys_under(g, CORP_EXTRA)
        .into_iter()
        .filter_map(|(k, n)| {
            let rest = k.strip_prefix(CORP_EXTRA)?;
            let (b, u) = rest.split_once('.')?;
            Some((b.parse().ok()?, u.parse().ok()?, n))
        })
        .collect()
}

/// A country flag's count: above 0 while it is up.
pub fn flag_level(g: &GameState, f: RoundFlag, country: usize) -> i64 {
    g.var(&flag(f, country))
}

/// Whether a country flag is up.
pub fn flagged(g: &GameState, f: RoundFlag, country: usize) -> bool {
    g.var(&flag(f, country)) > 0
}

/// Shelters `country`'s facilities and nodes for the rest of the round.
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

/// Neutral cubes on country `c`'s megaproject (074).
pub fn project_neutral(c: usize) -> KeyBuf {
    KeyBuf::new("board.project.").num(c).text(".neutral")
}

/// Set once country `c`'s megaproject has completed.
pub fn project_done(c: usize) -> KeyBuf {
    KeyBuf::new("board.project_done.").num(c)
}

/// What country `c`'s completed megaproject paid, cubes by color: `player`,
/// `adversary`, `neutral`. Only the card shows it (074).
pub fn project_paid(c: usize, color: &str) -> KeyBuf {
    KeyBuf::new("board.project_paid.").num(c).text(".").text(color)
}

/// The last round seat `s` used country `c`'s converter.
fn converter_round(c: usize, s: usize) -> KeyBuf {
    KeyBuf::new("board.converter_round.").num(c).text(".").num(s)
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
            cx.add_tokens(g, at(piece.neutral_station), STATION, 1);
            for &n in &piece.factories {
                cx.add_tokens(g, at(n), FACTORY, 1);
            }
        }
        for s in 0..g.seats.len() {
            let c = home_country(&w, s);
            let n = w.content.map.countries[c].node;
            let start = w.map.numbered(c, n);
            cx.add_tokens(g, start, node(s), 1);
            cx.set_key(g, &track_key(s), 1);
            cx.add_resource(g, s, Resource::Ambassadors, w.int("board.start_ambassadors"));
            if let Some(h) = bound_city(&w, g, c, start) {
                cx.add_tokens(g, h, cube(s), 1);
            }
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
        // 118: at `sell_credits` 0 a sale pays nothing: not offered.
        let sells = w.int("board.sell_credits") > 0;
        for id in g.pile(&cards::hand(s)).iter().filter(|_| sells) {
            out.push(Action::Sell {
                seat,
                card: *id,
            });
        }
        for c in 0..w.map.countries.len() {
            if converter_blocked(w, g, s, c).is_none() {
                out.push(Action::Convert { seat, country: c as u8 });
            }
        }
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Standard { .. } | Action::Sell { .. } | Action::Convert { .. })
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
                // 118: refused here too, not only hidden (a record or client could send it).
                if w.int("board.sell_credits") <= 0 {
                    return Err(Halt::Illegal("selling pays nothing here".into()));
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
                price.pay(g, cx, s);
                cx.tally(g, &KeyBuf::new("board.std.").text(act.name()), 1);
                cx.tally(g, "board.std_actions", 1);
                // 105: a removal here fires the seat's fight triggers.
                cards::fighting(g, cx, s, |g, cx| std_resolve(g, cx, s, act, &targets))
            }
            Action::Convert { seat, country } => {
                let (s, c) = (*seat as usize, *country as usize);
                // An answer replays this from a table that used it.
                if !cx.replaying() {
                    if c >= w.map.countries.len() {
                        return Err(Halt::Illegal("no such country".into()));
                    }
                    if let Some(why) = converter_blocked(&w, g, s, c) {
                        return Err(Halt::Illegal(format!("can't use that converter: {why}")));
                    }
                }
                convert(g, cx, s, c)
            }
            _ => Err(Halt::Illegal("not a standard action".into())),
        }
    }

    fn run(&self, phase: usize, g: &mut GameState, cx: &mut Cx) -> Res {
        match phase {
            1 => produce_units(g, cx)?,
            2 => battle(g, cx),
            3 => march(g, cx)?,
            4 => produce(g, cx)?,
            5 => cleanup(g, cx),
            _ => unreachable!("board: no auto phase {phase}"),
        }
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        board_view(g, w, out);
    }
}

/// The converter's block when the seat has no node in a city of the country.
pub const NO_CITY_NODE: &str = "no node in a city here";
/// The converter's block when the seat can't pay its cost.
pub const CANT_PAY: &str = "can't pay it";

/// Why seat `s` can't use country `c`'s converter now, if anything stops it.
pub fn converter_blocked(w: &World, g: &GameState, s: usize, c: usize) -> Option<&'static str> {
    if g.var(&project_done(c)) == 0 {
        return Some("the megaproject is not complete");
    }
    // r37 (Matt): a node in a city of the country, any city; 075 took any node there.
    if !has_city_node_in(w, g, s, c) {
        return Some(NO_CITY_NODE);
    }
    let conv = &w.content.map.countries[c].megaproject.converter;
    // 105: only a converter printed once a round keeps the limit.
    if conv.once && g.var(&converter_round(c, s)) == g.round as i64 {
        return Some("used this round");
    }
    let cost = &conv.cost;
    if cost.iter().any(|(r, n)| g.resource(s, *r) < *n) {
        return Some(CANT_PAY);
    }
    // 118: a converter that does nothing here (Biomia's Sway with alignment off) isn't offered.
    if !effects::feasible(w, g, s, &conv.effects) {
        return Some("does nothing here");
    }
    None
}

/// Seat `s` uses country `c`'s converter: pays, then its printed effects.
fn convert(g: &mut GameState, cx: &mut Cx, s: usize, c: usize) -> Res {
    let w = cx.world;
    let m = &w.content.map.countries[c].megaproject;
    for (r, n) in &m.converter.cost {
        cx.add_resource(g, s, *r, -n);
        if *r == Resource::ResearchCubes {
            cx.tally(g, "board.cubes_converted", *n);
        }
    }
    if m.converter.once {
        cx.set_key(g, &converter_round(c, s), g.round as i64);
    }
    cx.tally(g, &format!("board.converter.{c}"), 1);
    cards::fighting(g, cx, s, |g, cx| effects::run(g, cx, &m.converter.effects, &mut effects::Ctx::seat(s)))
}

/// 135: the hex flags count down as the country flags do; standing
/// troops and occupations follow the troops left on their hex; registrations
/// past their round, and a threat's notes to its own later ops, go.
fn expire_marks(g: &mut GameState, cx: &mut Cx) {
    let round = g.round as i64;
    for (k, v) in keys_under(g, HEX_FLAGS) {
        if v > 0 {
            cx.set_key(g, &k, v - 1);
        }
    }
    for (k, v) in keys_under(g, STAND) {
        let Some(h) = k.rsplit('.').next().and_then(|h| h.parse::<usize>().ok()) else { continue };
        let left = v.min(g.tokens(h, TROOP)).max(0);
        if left != v {
            cx.set_key(g, &k, left);
        }
    }
    for (k, _) in keys_under(g, OCCUPIED) {
        let Some(h) = k.rsplit('.').next().and_then(|h| h.parse::<usize>().ok()) else { continue };
        if g.tokens(h, TROOP) <= 0 {
            cx.set_key(g, &k, 0);
        }
    }
    for (b, until, _) in corp_extras(g) {
        if until as i64 <= round {
            cx.set_key(g, &corp_extra_key(b, until), 0);
        }
    }
    for (k, v) in keys_under(g, EXTRA_WAR) {
        if v <= round {
            cx.set_key(g, &k, 0);
        }
    }
    // 149: Movement unused this round is lost (tallied, for the movement read).
    for s in 0..g.seats.len() {
        let lost = moves_left(&cx.world, g, s);
        if lost > 0 {
            cx.tally(g, &KeyBuf::new("board.moves_lost.").num(s), lost);
        }
    }
    for prefix in ROUND_MARKS {
        for (k, _) in keys_under(g, prefix) {
            cx.set_key(g, &k, 0);
        }
    }
    cards::expire_disabled(g, cx);
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
    expire_marks(g, cx);
    // Rounds that end with only botnets between the players and the
    // purge (019: botnets are hard to remove).
    let level = alignment::bar_level(&w, g);
    let botnets = adversary::botnet_count(g, None);
    cx.peak(g, "adversary.botnets_peak", botnets);
    if level.counts_botnets() && botnets > 0 && adversary::purge_count(&w, g, level) == botnets {
        cx.tally(g, "alignment.purge_blocked_by_botnets", 1);
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

//! The heuristic value of a state, from the team's point of view. It reads
//! only what the table shows: pieces on the board, tracks, resources, open
//! hands and buildings, active threats and what is paid on them, deck
//! sizes and the face-up adversary card. Units are roughly "credits": a
//! credit in hand is worth `credits` (0.6) because credits buy cards and
//! standard actions that do more.
//!
//! The board (014): cities are clean, aligned (a player cube) or corrupted
//! (the adversary's cube). Adversary ambassadors
//! gather on clean cities until `adversary.corrupt_at` corrupts them, divert
//! the production of the pieces they sit on (a factory builds a troop),
//! and come out of a supply that caps them.
//!
//! The weights are agent parameters, not rules: they live in [`Weights`]
//! and an experiment can override any of them (`greedy:w.corrupted=40`).
//!
//! **Horizon mode** (`w.horizon` > 0; the `engine` agent's default): income
//! and buildings are valued by what they will pay over the rounds the game
//! is expected to last ([`Eval::rounds_left`]: the adversary deck's clock,
//! cut short by how soon the team is on course to win and by the danger it
//! is in), not by a flat weight; a card in hand is valued by whether it can
//! be paid for before the game ends; and resources lose their value as the
//! end nears (nothing is left to buy).

use aic_core::content::{CardDef, Op, Resource, Side, TagOrAny, RESOURCES, TAGS};
use aic_core::modules::common::{
    self, ADV_AMBASSADOR, ADV_CUBE, FACTORY, HARDEN, STATION, TROOP,
};
use aic_core::modules::{adversary, ai, alignment, board, cards, economy, plan, threats};
use aic_core::op::CorruptSite;
use aic_core::key::Fnv;
use aic_core::{GameState, Key, Rules};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::sync::Arc;

use super::credit_cost;
use super::guide;
use super::strategy::{self, Choice, Mults};
use super::value::{self, CardValue, Ctx, Worth};
use super::INSIGHT_COST;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weights {
    /// A finished game: won or lost.
    pub win: f64,
    /// Added per adversary card left (deck and face-up) on a win (a wider margin).
    pub win_margin: f64,
    /// Each corrupted city (it builds ambassadors and diverts its energy to
    /// Discord).
    pub corrupted: f64,
    /// A seat down to its last node (027: a player with none loses);
    /// a share of that with two, three or four.
    pub node_danger: f64,
    /// Each aligned city (a player cube: extra energy for its owner).
    pub aligned: f64,
    /// Each adversary ambassador on the board (the bar-0 win needs none).
    pub adv_ambassador: f64,
    /// Ambassadors on a city not yet corrupted: this share of `corrupted`
    /// times (ambassadors / the next corrupt threshold)², capped at 1.
    pub pressure: f64,
    /// Each adversary ambassador on a player piece, base or factory (it
    /// diverts the piece's production).
    pub on_piece: f64,
    /// Extra per factory holding any adversary ambassador, however many (it
    /// makes one troop a production step).
    pub on_factory: f64,
    /// Each adversary troop (it destroys a building every round).
    pub troop: f64,
    /// Per Discord card short of 3 left in the deck (027: the tip that finds
    /// it empty loses).
    pub discord_danger: f64,
    /// Each adversary piece that keeps the current clear-board bar from
    /// being met (the win condition).
    pub blocker: f64,
    /// Per Unity card short of 3 left in the deck (028: drawing the last wins).
    pub unity_deck: f64,
    /// Each Unity tip made; Sway on the Unity track counts as its share of
    /// the next tip (so a tip about to happen is worth nearly a tip).
    pub unity_tip: f64,
    /// Each Discord tip made (a cost); Sway on the Discord track costs its
    /// share of the next one.
    pub discord_tip: f64,
    /// Each level the clear-board bar has dropped (Unity cards drawn, 136).
    pub level: f64,
    /// Each visible adversary project's fill, as a share of its cost (a
    /// cost: it will resolve; 017).
    pub project: f64,
    /// Each player node on the board (reach, site claims, a share of
    /// city energy; its track income is counted as income).
    /// Was `data_center` (still accepted, see [`RENAMED`]).
    pub node: f64,
    /// Each botnet on the board (a cost: adversary compute, contests nodes,
    /// blocks the purge).
    pub botnet: f64,
    /// Each corruption marker (041: a hex lost to placement).
    pub marker: f64,
    /// A hardening token on a node hex, on top of `harden` (041: it
    /// keeps a token that can't be replaced).
    pub shield: f64,
    /// A threat that waits this round counts this share (041): it comes back,
    /// but with a counter phase first.
    pub threat_waits: f64,
    /// 070: an unpaid threat no seat may counter yet counts this much more: the
    /// pull to put a node or an ambassador on a source.
    pub threat_stranded: f64,
    /// 146: how far an unpaid threat's weight follows what it would do uncountered: its printed harm at the
    /// value table over `threat_harm_ref` (0: every threat alike, as before 146; 1: by its harm), so a cheap harm
    /// may be let through and a dear one is paid.
    pub threat_harm: f64,
    /// 146: the harm (credits at the value table) that weighs as one `threat`.
    pub threat_harm_ref: f64,
    /// Each player-owned factory (chips each production step); an
    /// adversary-owned one costs as much.
    pub owned_factory: f64,
    /// Each chip a seat holds.
    pub chips: f64,
    /// Each city, base or station hex inside some seat's reach.
    pub reach: f64,
    /// Each research station a player owns; a corrupted one costs as much.
    pub station: f64,
    /// Each player cube on a megaproject; an adversary cube costs half, and a
    /// neutral one nothing (074).
    pub project_cube: f64,
    /// Each research cube a seat holds (074): low on purpose, under what every
    /// converter nets by these weights, so a cube held never beats using one.
    pub research_cube: f64,
    /// r37: a neutral city in some seat's reach that an adversary ambassador
    /// darkens (no energy sale while it stands there).
    pub city_dark: f64,
    /// r38: an aligned city an adversary ambassador darkens (its owner gets no
    /// energy while it stands there).
    pub city_dark_aligned: f64,
    /// r37: a seat holding research cubes with a node in a city of a country
    /// whose megaproject is done (the converter's gate), once per such country.
    pub converter_city: f64,
    /// A research cube held by a seat whose board spends cubes on a power (075:
    /// the Researcher's second node takes 3): in place of `research_cube`.
    pub research_cube_power: f64,
    /// 092, a strategy-guide heuristic (0: off): a country sheltered while a
    /// threat or the war step showing would destroy a node or factory there is
    /// worth what the shelter saves (`agents/guide.rs`).
    pub shelter_read: f64,
    /// 092: a node a shelter saves, beyond `node` and the seat's `node_danger`
    /// step (a lost node leaves the game, and its hex takes a marker).
    pub shelter_node: f64,
    /// 092 (1: on): a shelter counts no threat the team can still counter from
    /// what it holds (a player counters first and shelters what it can't).
    pub shelter_counter: f64,
    /// Horizon mode: per round left, each of a seat's first
    /// [`STATIONS_USEFUL`] owned stations (074).
    pub station_round: f64,
    /// An owned station past a seat's first [`STATIONS_USEFUL`] (074), in place
    /// of `station`: a converter takes a cube or two a round, so more stations
    /// fill a card sooner and then only stack cubes.
    pub station_spare: f64,
    /// r35: a corrupted station costs this on top of `station` (a flip: a cube
    /// a round both ways), and freeing it earns it back.
    pub station_flip: f64,
    /// 137: a seat in reach saving ambassadors toward liberating an adversary
    /// station: this share of `station` + `station_flip` at the full cost,
    /// growing with its square.
    pub station_freeing: f64,
    pub factory: f64,
    /// Each hardening token.
    pub harden: f64,
    /// Each player ambassador in a seat's supply.
    pub ambassador: f64,
    /// Each player ambassador on the board (shields a building, cancels an
    /// arriving adversary ambassador, unites a base).
    pub ambassador_board: f64,
    /// Player ambassadors toward a corrupted city's hold (027), on it or in
    /// the supply of a seat with it in reach: this share of `corrupted`.
    pub freeing: f64,
    /// 080: each city liberated this game (by a card or the fallback), on top of
    /// the corrupted city it no longer is.
    pub liberated: f64,
    /// 080: a clean or aligned city one adversary ambassador from corrupting,
    /// on top of `pressure`.
    pub at_risk: f64,
    /// 080: a liberate building's tokens toward its liberation (Popular
    /// Uprising), at this much for a full pile, while a city is corrupted.
    pub liberate_progress: f64,
    /// Each point of per-round income (AI tracks).
    pub income: f64,
    /// A track's capstone (068): the square of the share of the track
    /// uncovered, so the last steps count most.
    pub track_capstone: f64,
    pub credits: f64,
    /// 024: credits a seat holds beyond `credit_cap` are worth `credit_idle`
    /// each: a stockpile buys nothing this round, so spending wins.
    pub credit_cap: f64,
    pub credit_idle: f64,
    /// A program in a seat's play area (021: its tags pay AI-board powers).
    pub program: f64,
    /// A token on one of a seat's cards (021).
    pub token: f64,
    /// Cards in hand (and drafted) that count, best first (021); 0: all.
    /// At 8 the 021 screen fell to 38% at d0 (agents hoarded their best
    /// cards and stopped playing), so it is off by default.
    pub hand_cap: f64,
    pub energy: f64,
    pub antimatter: f64,
    pub insight: f64,
    /// An insight held by a seat with Patience (075): it carries over, so it
    /// keeps its worth at the end of a round, where anyone else's goes.
    pub patience: f64,
    /// A player-owned facility under a Wall (075): no troop can strike it. Twice
    /// this with a troop within `WALL_NEAR` hexes.
    pub wall: f64,
    /// A card in hand: `hand + hand_cost * cost`, cost in credits (see
    /// [`super::credit_equivalent`]).
    pub hand: f64,
    pub hand_cost: f64,
    /// A drafted card not yet kept, as a share of its hand value.
    pub drafted: f64,
    /// 023: a card dealt into the draft packet counts as drafted, less its
    /// keep price (flat mode: insight at `INSIGHT_COST` credits; table mode: the
    /// table's `keep_tier1` / `keep_tier2` rate) times this; so the
    /// tier pick is a choice (before 023 a tier pick scored nothing, and
    /// tier 2 was drafted only once the tier-1 deck ran dry).
    pub packet_keep: f64,
    /// 046: above 0, drafted and packet cards count only as far as this
    /// round's insight keeps them, best first. Priced one at a time, every
    /// tier-2 pick looked worth a keep and agents drafted tier 2 three picks
    /// in four, more than a seat could keep, and tier-1 Science went unseen.
    pub packet_budget: f64,
    /// A building in play: `built + built_cost * cost`.
    pub built: f64,
    pub built_cost: f64,
    /// Each active threat, scaled by the share still unpaid.
    pub threat: f64,
    /// Horizon mode, when above 0: each unit of income (AI tracks,
    /// buildings, rewards) is worth `horizon` × the resource's own weight ×
    /// the rounds left, replacing `income`. 0 = the flat weights above.
    pub horizon: f64,
    /// Horizon mode: Sway toward Unity the team is assumed to gain per round
    /// before it has a record of its own (round 1), for the win estimate.
    /// Was `opp_rate` (still accepted, see [`RENAMED`]).
    pub unity_rate_start: f64,
    /// Horizon mode: rounds a win needs after the Unity target is reached.
    pub win_lag: f64,
    /// Horizon mode: chance of losing per round per unit of danger (an
    /// unpaid threat per country, a last node, the Discord deck at its edge);
    /// the rounds left are the expected survival over the clock.
    pub risk: f64,
    /// Horizon mode: per round left, a building that pays no income (an
    /// ability, a discount, protection), on top of `built`.
    pub ability: f64,
    /// Horizon mode: resources keep their full value only while at least
    /// this many rounds are left, then fall toward 0 (nothing left to buy).
    pub spend_horizon: f64,
    /// Horizon mode: board production (energy of aligned cities not
    /// diverted) counts this share of income over the rounds left, on top
    /// of the flat piece weights.
    pub production: f64,
    /// 082, horizon mode, when above 0: cards are valued by the value table
    /// (`agents/value.rs`), at this many heuristic points a table credit. A card in
    /// hand, drafted or in the packet counts its net worth (its value by its ops,
    /// times its seat's strategy multiplier, less what it costs to play), a
    /// building in play what it earns a round that income doesn't count, a token
    /// what its card's spend pays per token, an ambassador in supply its table row;
    /// an action is credited what it does that the board terms can't see. 0: the
    /// hand-written terms before 082 (`hand`, `hand_cost`, `ability`, `token`,
    /// `ambassador`, `liberated`, `liberate_progress`).
    pub table: f64,
    /// 082: the share of a card's net worth it counts in hand (table mode). Under
    /// 1, so playing a card (its board terms, which see about its whole value)
    /// beats holding it.
    pub hand_share: f64,
    /// 082, a probe: a supply ambassador in table mode is the table's row times this.
    pub table_ambassador: f64,
    /// 082, a probe: above 0, antimatter in table mode is the table's row (9
    /// credits) times this, in place of `antimatter`. At 1 the strategist held its
    /// antimatter and lost 59 points (31000, 144 games): off.
    pub table_antimatter: f64,
    /// 093: above 0, energy in table mode, held, earned as income or made by a
    /// city, is the table's rate (`resources.energy`, 4.5 credits) times this,
    /// in place of `energy`; income at `rates.energy_income_share` of it, as the
    /// card prices count it (value.rs `income_rate`). Off: at 1 greedy d0 lost
    /// 15.3 ±4.0 (it hoards energy) and the strategist d2 2.1 ±5.0; at 0.3, which
    /// is about `energy` 0.8 at the table's credit weight, d2 −4.9 ±2.7 and d0
    /// +2.1 ±2.1 (31600, 144 games each).
    pub table_energy: f64,
    /// 082, a probe: above 0, income (tracks, buildings, rewards) in table mode is
    /// valued at the table's rates times this, in place of the held resources'.
    pub table_income: f64,
    /// 082: the share of its play cost a card still to be drafted or kept counts
    /// (table mode). The keep plan judges what a seat can pay; charged in full, a
    /// tier-2 card (priced at about its value less its keep) reads as worth no
    /// more than its keep, and the engine drafted tier 1 only.
    pub pick_cost: f64,
    /// 082, the progress term: a card whose `needs` aren't met counts this share,
    /// plus the rest times the square of the share of the tags it has.
    pub needs_floor: f64,
    /// 082, the progress term: a board power that sacrifices two tags or more
    /// counts this share of one use, times the square of the share of the tags
    /// the seat has toward it.
    /// (102's Phase A switches, folded in by 124 and always on: the dear-power
    /// term reads `ai::power_is_dear` (D33); a token on a card that cashes its own
    /// pile out is worth its cash-out rate (D24); `converter_city` counts a city
    /// node whatever the cubes held (D27); `station_flip` is scaled by the share
    /// of its country's megaproject still open (D28); the liberation push needs
    /// the city's own adversary ambassadors on top of `liberate_at` (D35).)
    pub power_progress: f64,
    /// The deck clock (011), any mode: per round the team's projected win
    /// (Unity still needed at its rate so far, plus `win_lag`) comes
    /// after the adversary deck runs out. 0 = off.
    pub clock: f64,
    /// 116: how far a strategy's multipliers reach from 1 (`agents/strategy.rs`,
    /// `Mults`): 1 as written in `strategies.json`, 0 every multiplier 1 (the
    /// strategy is still chosen, and `plan_reach` still reads its tags). A sweep
    /// knob (scripts/tune/agent_sweep.py).
    pub strat_scale: f64,
    /// 124 (D47), above 0: a strategy reads the ops a card's triggers and
    /// thresholds resolve ("a trigger that pays credits is a credits card").
    pub strat_contents: f64,
    /// 124 (D47), above 0 (table mode): a trigger on a teammate's plays of a tag
    /// fires at the teammates' share of cards with it, not the owner's. The keep
    /// plan's twin is the forced row `plan_team` (policy.rs).
    pub team_share: f64,
    /// 124 (D47), above 0: a token on a card with a threshold (At N) is worth
    /// what the threshold pays per token, when that beats the held rate.
    pub threshold_tokens: f64,
    /// 124 (D53), above 0: an insight a round of income is worth this, in place
    /// of the held `insight` weight (horizon mode).
    pub insight_income: f64,
    /// 124, above 0: insight a seat holds beyond this is spare: worth
    /// `insight_idle` each held (and, under Patience, as income onto the pile),
    /// and a keep paid from it costs that ("insight you can't spend is cheap:
    /// keep with it").
    pub insight_cap: f64,
    pub insight_idle: f64,
    /// 128, table mode: a per-tag op counts the seat's tags in play plus this
    /// share of the tags on the cards in its hand ("you'll play most of these
    /// first": a per-tag card is worth what it pays at the count it's played at).
    pub tag_hand: f64,
    /// 128, above 0: energy a seat holds beyond this is spare: worth
    /// `energy_idle` each held, and an Action or card that spends it costs the
    /// seat that share of the table's energy price ("energy piling up is cheap:
    /// draft and use what spends it").
    pub energy_cap: f64,
    pub energy_idle: f64,
    /// 128, above 0: an Action that places project cubes counts nothing once no
    /// megaproject is open to the seat (`Ctx::cubes_open`). The keep plan's twin
    /// is the forced row `plan_cube_target` (policy.rs).
    pub cube_target: f64,
    /// 136 (the death audit), above 0: a node on or next to a clean city that
    /// is filling toward corruption counts as this share of lost, by
    /// ((its ambassadors + marker + 1) / the corrupt threshold)², capped at 1: it
    /// is worth less `node`, and the seat's `node_danger` reads its nodes less
    /// the exposed ones (a corrupting city takes its whole cluster).
    pub node_exposure: f64,
    /// 138: a player's research station or factory filling toward its
    /// Corrupt number (what it holds, plus next door when a gathering line is
    /// ahead) costs this share of `pressure` (0: off; cities always count).
    pub site_pressure: f64,
    /// 137: each adversary city, station or factory a player ambassador stands
    /// on, blocking its output (a city's diversion, a station's compute, a
    /// factory's troop): about a round's output.
    pub block: f64,
    /// 137: each hex where a seat's own ambassador stands on its node or owned
    /// facility: armor, used up instead of the piece.
    pub armor: f64,
}

/// Weights renamed since experiments named them: (former name, name).
/// 086 renamed `presence` to `node` and the old `node` (the adversary's) to
/// `botnet`; that `node` can't be aliased, see [`PRE_086`].
const RENAMED: [(&str, &str); 6] = [
    ("opp_rate", "unity_rate_start"),
    ("hardware", "chips"),
    ("data_center", "node"),
    ("capstone", "track_capstone"),
    ("presence", "node"),
    ("presence_danger", "node_danger"),
];

/// Weight names only a pre-086 spec uses. Such a spec's `node` meant the
/// adversary's (now `botnet`), so [`check_names`] refuses it there.
const PRE_086: [&str; 3] = ["presence", "presence_danger", "data_center"];

/// Refuses a spec's weight names that would silently mean another weight:
/// a bare `node` beside a pre-086 name. A pre-086 spec naming `node` alone
/// can't be told apart and sets the player-node weight.
pub fn check_names<'a>(names: impl IntoIterator<Item = &'a str>) -> Result<(), String> {
    let names: Vec<&str> = names.into_iter().collect();
    if let Some(old) = names.iter().find(|n| PRE_086.contains(n)) {
        if names.contains(&"node") {
            return Err(format!(
                "weight `node` beside pre-086 `{old}`: there `node` was the adversary's, now `botnet`; \
                 write `botnet`, or `node` with the other names current"
            ));
        }
    }
    Ok(())
}

/// A drafted card's keep price in credits (its tier's insight at
/// `INSIGHT_COST`), for valuing the draft packet (023).
fn keep_insight(world: &aic_core::module::World, tier: u8) -> f64 {
    let key = if tier == 2 { "cards.keep_tier2" } else { "cards.keep_tier1" };
    (INSIGHT_COST * world.int(key)) as f64
}

/// 102 (D28): the share of hex `h`'s country's megaproject still to fill, 0 once
/// it has completed (a station's cubes then go to their owner, none to it).
fn open_share(world: &aic_core::module::World, g: &GameState, h: usize) -> f64 {
    let c = world.map.hexes[h].country as usize;
    if c >= world.map.countries.len() || g.var(&board::project_done(c)) > 0 {
        return 0.0;
    }
    let need = world.content.map.countries[c].megaproject.cubes.max(1);
    let filled = g.var(&board::project_key(c, None))
        + g.var(&board::project_neutral(c))
        + (0..g.seats.len()).map(|s| g.var(&board::project_key(c, Some(s)))).sum::<i64>();
    (1.0 - filled as f64 / need as f64).clamp(0.0, 1.0)
}

/// The share of `node_danger` a seat with `count` nodes is charged.
fn danger_share(count: i64) -> f64 {
    match count {
        ..=1 => 1.0,
        2 => 0.5,
        3 => 0.25,
        4 => 0.1,
        _ => 0.0,
    }
}

/// How many of a seat's owned stations the eval counts in full (074).
const STATIONS_USEFUL: i64 = 2;
/// How far off a troop makes a Wall's facility worth more: two marches.
const WALL_NEAR: u32 = 4;
/// Flat mode: a point of track income is worth this many of the resource's
/// own points for antimatter and insight (1 for the rest), times `income`.
const ANTIMATTER_INCOME_UNIT: f64 = 3.0;
const INSIGHT_INCOME_UNIT: f64 = 1.5;

impl Default for Weights {
    fn default() -> Self {
        Weights {
            win: 1000.0,
            win_margin: 5.0,
            // 014: chosen by hand for the new board, not yet swept.
            corrupted: 30.0,
            node_danger: 60.0,
            aligned: 8.0,
            adv_ambassador: 3.0,
            pressure: 0.7,
            on_piece: 2.0,
            on_factory: 6.0,
            troop: 12.0,
            discord_danger: 40.0,
            unity_deck: 40.0,
            blocker: 2.0,
            // 013: a Unity tip replaces about 4 opposition points (24) and
            // brings a card; a Discord tip brings one of the adversary's.
            unity_tip: 24.0,
            discord_tip: 16.0,
            level: 10.0,
            // 017: chosen by hand, not swept.
            project: 30.0,
            node: 4.0,
            // 022 (020's D9): a botnet is permanent since 019; at 6 agents made
            // botnets answering the spread when an equal free option existed.
            botnet: 25.0,
            marker: 1.0,
            shield: 3.0,
            threat_waits: 0.5,
            threat_stranded: 1.25,
            threat_harm: 1.0,
            threat_harm_ref: 15.0,
            owned_factory: 4.0,
            chips: 2.0,
            reach: 0.5,
            station: 5.0,
            project_cube: 2.0,
            research_cube: 0.3,
            // r37: about one converter use's net over holding the cubes.
            converter_city: 2.0,
            // r37: a round's sale is near break-even at the table (5 credits for
            // energy worth 4.5), but energy is scarce: about one energy's worth.
            city_dark: 4.0,
            // r38: the owner's free energy each round (about 4.5 credits), lost
            // for sure, where a neutral sale is near break-even: half again.
            city_dark_aligned: 6.0,
            research_cube_power: 1.2,
            shelter_read: 1.0,
            shelter_node: 12.0,
            shelter_counter: 1.0,
            station_round: 1.0,
            station_spare: 1.0,
            station_flip: 20.0,
            station_freeing: 0.25,
            factory: 0.5,
            harden: 1.0,
            // 018: was 3, below the 6 credits (3.6) an ambassador costs, so no
            // agent ever bought one and corrupted cities were never freed.
            ambassador: 5.0,
            ambassador_board: 2.5,
            freeing: 2.0,
            liberated: 10.0,
            at_risk: 4.0,
            liberate_progress: 20.0,
            income: 3.0,
            track_capstone: 16.0,
            credits: 0.6,
            credit_cap: 12.0,
            credit_idle: 0.15,
            program: 1.0,
            token: 0.5,
            hand_cap: 0.0,
            energy: 0.8,
            antimatter: 2.0,
            insight: 0.5,
            patience: 1.0,
            wall: 3.0,
            hand: 3.0,
            hand_cost: 0.4,
            drafted: 0.3,
            packet_keep: 1.0,
            packet_budget: 1.0,
            built: 4.0,
            built_cost: 0.5,
            threat: 6.0,
            horizon: 0.0,
            unity_rate_start: 2.0,
            win_lag: 1.0,
            risk: 0.1,
            ability: 0.5,
            spend_horizon: 2.0,
            // Horizon mode only; the engine turns it on (see `engine`).
            production: 0.0,
            // 082: off in flat mode; the engine turns it on at the credit's weight.
            table: 0.0,
            hand_share: 0.5,
            table_ambassador: 1.0,
            table_antimatter: 0.0,
            table_energy: 0.0,
            table_income: 0.0,
            pick_cost: 0.0,
            needs_floor: 0.25,
            power_progress: 0.5,
            clock: 0.0,
            strat_scale: 1.0,
            // 124: the strategy and insight-income levers are the engine's (see
            // `engine`; greedy has no strategy and no horizon).
            strat_contents: 0.0,
            // 124: off; on, neither moved the screen (team_share −0.9 ±1.9 alone and
            // −2.1 on the candidate; threshold_tokens 0.0 ±0.9 and −1.4).
            team_share: 0.0,
            threshold_tokens: 0.0,
            // 124: every agent keeps with spare insight (with the plan's
            // `keep_spare`): what a new player would do (Matt, 2026-11-05, message
            // 5216). 127: the limit rules that came with it are gone with the limits.
            insight_income: 0.0,
            insight_cap: 10.0,
            insight_idle: 0.1,
            tag_hand: 0.0,
            energy_cap: 0.0,
            energy_idle: 0.2,
            cube_target: 1.0,
            node_exposure: 0.0,
            site_pressure: 1.0,
            block: 4.0,
            armor: 1.0,
        }
    }
}

impl Weights {
    /// The `engine` agent's defaults: horizon mode, with hand and building
    /// weights that no longer stand in for the future (008 sweep).
    pub fn engine() -> Weights {
        Weights {
            horizon: 1.0,
            hand: 1.0,
            hand_cost: 0.2,
            built: 1.0,
            built_cost: 0.0,
            // 011: see `clock`; chosen on 009's cheap-tier-2 patch, where
            // without it the engine ran out the deck.
            clock: 30.0,
            // 014 (d0, 2 countries, 48 seeds, the checkpoint's map): 1 won
            // 64.6%, 0 won ~37% (32 seeds); 2 won 66.7%. On the map as since
            // edited, level within noise at d1-d3 (32 seeds). Energy and
            // compute now come from the board.
            production: 1.0,
            // 082: cards by the value table, a table credit at a credit's weight.
            table: 0.6,
            // 082: half the draft's keep price: at the full price the strategist with
            // none (w.packet_keep=0) led it by +4.9 ±4.0 (30700) and +12.5 ±5.9
            // (31000), and 0.5 by +13.9 ±5.3 (31000).
            packet_keep: 0.5,
            // 124 (dev seeds 34756-34899, the strategist, 432 games a cell): the
            // strategy reads trigger contents (alone +2.5 ±1.2); an insight a round
            // at its table rate in points, 2.5 × `table` 0.6 (D53: Hyperscale Botnet
            // played 0.009 → 0.14 a game; alone −2.8 ±2.2, with `insight_cap` +2.3
            // ±2.3). With every agent's spare insight and the r49 limit
            // rules (gone at r51): +3.7 ±2.5.
            strat_contents: 1.0,
            insight_income: 1.5,
            // 128: per-tag ops at the count they're played at; spare energy cheap.
            tag_hand: 0.5,
            energy_cap: 4.0,
            ..Weights::default()
        }
    }

    /// Sets one weight by name (or a former name, see [`RENAMED`]); false
    /// if there is no such weight.
    pub fn set(&mut self, name: &str, v: f64) -> bool {
        let name = RENAMED
            .iter()
            .find(|(old, _)| *old == name)
            .map_or(name, |(_, new)| *new);
        let mut map = match serde_json::to_value(&*self) {
            Ok(serde_json::Value::Object(m)) => m,
            _ => return false,
        };
        if !map.contains_key(name) {
            return false;
        }
        map.insert(name.to_string(), serde_json::json!(v));
        match serde_json::from_value(serde_json::Value::Object(map)) {
            Ok(w) => *self = w,
            Err(_) => return false,
        }
        true
    }
}

/// A [`Weights`] bound to one game's rules, with the parameters it reads
/// cached. Cheap to clone (030): each of a planner's rollouts has its own.
#[derive(Clone)]
pub struct Eval {
    pub w: Weights,
    /// For the seat-terms store's key (see [`tableau_parts`]): clones share it.
    id: u64,
    seats: usize,
    /// Unity tip count the team aims for: the deck's last card.
    target_tips: i64,
    city_energy: i64,
    aligned_bonus: i64,
    tip_per_player: i64,
    /// The seat with Wall, if a seat has it.
    wall_seat: Option<usize>,
    /// Each seat's Patience (075).
    patient: [bool; 4],
    /// Each seat whose board has a power paid in research cubes (075).
    cube_power: [bool; 4],
    /// 146: each threat's harm uncountered, at the value table (credits), by id.
    threat_harms: Arc<Vec<(Key, f64)>>,
    keys: TokenKeys,
    /// Each card by its index in the content (found by `Content::card_index`,
    /// by its interned id's address): its [`Eval::cost`] and its table value. A
    /// position asks for a hand's worth of each: an id hashed as text was a tenth
    /// of the heuristic (069), and a map of its own by address a tenth again (141).
    facts: Arc<Vec<Fact>>,
    /// 107 (`rates.own_mix`): whether a one-tag trigger fires at the seat's own share.
    mix: bool,
    /// 131: the value table this agent reads (`t.<key>=v` in its spec; else the table).
    pub tbl: &'static value::Table,
    /// 082: each strategy's multiplier for each card and phase.
    mults: Arc<Mults>,
    /// 082: each seat's strategy, once chosen (none: every multiplier 1).
    strat: Choice,
    /// 082: each seat's board powers: what a use does that the board terms can't
    /// see, its worth for the progress term, and the tags it sacrifices.
    powers: Arc<Vec<Vec<PowerFact>>>,
    /// Each seat's "took a card from this draft hand" var (082: a packet a seat has
    /// picked from goes to the next seat, so it counts for that seat).
    picked: [String; 4],
}

/// One card, read once (see [`Eval::facts`]).
#[derive(Clone, Copy)]
struct Fact {
    index: usize,
    cost: f64,
    value: CardValue,
    /// What a token on it is worth.
    token: Worth,
    /// The token kind of a pile of the seat's own it works on, when played (a
    /// feeder, a cash-out, a count).
    pile: Option<aic_core::content::TokenKind>,
}

/// The token kind of a pile of the seat's own card `d` works on when played.
fn pile_kind(d: &CardDef) -> Option<aic_core::content::TokenKind> {
    d.effects.iter().find_map(|e| match e {
        Op::AddTokens { kind, here: false, .. }
        | Op::CashOut { kind, here: false, .. }
        | Op::GainPerTokens { kind, here: false, .. } => Some(*kind),
        _ => None,
    })
}

/// One board power, read once.
#[derive(Clone)]
struct PowerFact {
    /// `ai::power_is_dear` (102, D33).
    dear: bool,
    unseen: f64,
    worth: f64,
    tags: Vec<(TagOrAny, i64)>,
}

/// The tokens [`Eval::value`] reads on one hex, read once: asking the state
/// for each by name was some 800 scans a position (069).
#[derive(Clone, Copy, Default)]
struct HexTokens {
    adv_ambassador: i64,
    adv_cube: i64,
    marker: i64,
    factory: i64,
    station: i64,
    harden: i64,
    troop: i64,
    botnet: i64,
    node: [i64; 4],
    cube: [i64; 4],
    ambassador: [i64; 4],
}

/// The interned names of [`HexTokens`]' tokens, compared by pointer.
#[derive(Clone)]
struct TokenKeys {
    adv_ambassador: Key,
    adv_cube: Key,
    marker: Key,
    factory: Key,
    station: Key,
    harden: Key,
    troop: Key,
    botnet: Key,
    node: [Key; 4],
    cube: [Key; 4],
    ambassador: [Key; 4],
}

impl TokenKeys {
    fn new() -> TokenKeys {
        let k = Key::new;
        TokenKeys {
            adv_ambassador: k(ADV_AMBASSADOR),
            adv_cube: k(ADV_CUBE),
            marker: k(common::MARKER),
            factory: k(FACTORY),
            station: k(STATION),
            harden: k(HARDEN),
            troop: k(TROOP),
            botnet: k(common::BOTNET),
            node: std::array::from_fn(|s| k(common::node(s))),
            cube: std::array::from_fn(|s| k(common::cube(s))),
            ambassador: std::array::from_fn(|s| k(common::ambassador(s))),
        }
    }

    fn read(&self, g: &GameState) -> Vec<HexTokens> {
        let at = |keys: &[Key; 4], k: Key| keys.iter().position(|x| *x == k);
        g.hexes
            .iter()
            .map(|hex| {
                let mut t = HexTokens::default();
                for (k, n) in hex.tokens.iter() {
                    let (k, n) = (*k, *n);
                    if let Some(s) = at(&self.node, k) {
                        t.node[s] = n;
                    } else if let Some(s) = at(&self.cube, k) {
                        t.cube[s] = n;
                    } else if let Some(s) = at(&self.ambassador, k) {
                        t.ambassador[s] = n;
                    } else if k == self.adv_ambassador {
                        t.adv_ambassador = n;
                    } else if k == self.adv_cube {
                        t.adv_cube = n;
                    } else if k == self.marker {
                        t.marker = n;
                    } else if k == self.factory {
                        t.factory = n;
                    } else if k == self.station {
                        t.station = n;
                    } else if k == self.harden {
                        t.harden = n;
                    } else if k == self.troop {
                        t.troop = n;
                    } else if k == self.botnet {
                        t.botnet = n;
                    }
                }
                t
            })
            .collect()
    }
}

impl Eval {
    pub fn new(rules: &Rules, w: Weights) -> Eval {
        Eval::with_table(rules, w, value::table())
    }

    /// 131: an eval reading `tbl` for the value table (see [`value::table_with`]).
    pub fn with_table(rules: &Rules, w: Weights, tbl: &'static value::Table) -> Eval {
        let world = rules.world();
        let seats = rules.config.countries as usize;
        // 028: the Unity deck's last card wins; 136: every printed one is dealt.
        let target_tips = world.content.alignment.iter().filter(|c| c.side == Side::Unity).count() as i64;
        let strat_scale = w.strat_scale;
        let strat_contents = w.strat_contents > 0.0;
        let threshold_tokens = w.threshold_tokens > 0.0;
        Eval {
            w,
            id: next_eval_id(),
            seats,
            target_tips,
            city_energy: world.list("board.corp_energy").first().copied().unwrap_or(1),
            aligned_bonus: world.int("board.aligned_bonus"),
            tip_per_player: world.int("alignment.tip_per_player"),
            wall_seat: (0..seats).find(|&s| ai::board(&world, s).innate.passives.iter().any(|p| matches!(p, Op::Wall))),
            patient: std::array::from_fn(|s| s < seats && economy::patient(&world, s)),
            cube_power: std::array::from_fn(|s| {
                s < seats && ai::board(&world, s).powers.iter().any(|p| p.cost.iter().any(|(r, _)| *r == Resource::ResearchCubes))
            }),
            keys: TokenKeys::new(),
            threat_harms: Arc::new(
                world
                    .content
                    .threats
                    .iter()
                    .map(|t| (Key::new(&t.id), -t.effects.iter().map(|e| value::op_worth(tbl, e).now).sum::<f64>()))
                    .collect(),
            ),
            mix: tbl.rate("own_mix") > 0.0,
            tbl,
            facts: Arc::new(
                world
                    .content
                    .cards
                    .iter()
                    .enumerate()
                    .map(|(i, d)| Fact {
                            index: i,
                            cost: Self::cost(d),
                            value: CardValue::of(tbl, d),
                            token: {
                                // 102 (D24): a token on a card that cashes its own pile out is worth that.
                                let base = value::token_worth_cash(tbl, d);
                                if threshold_tokens {
                                    value::token_worth_threshold(tbl, d, base)
                                } else {
                                    base
                                }
                            },
                            pile: if d.is_building() { None } else { pile_kind(d) },
                    })
                    .collect(),
            ),
            mults: Arc::new(Mults::new(&world, strat_scale, strat_contents)),
            strat: [None; 4],
            powers: Arc::new(
                (0..seats)
                    .map(|s| {
                        ai::board(&world, s)
                            .powers
                            .iter()
                            .map(|p| {
                                let wv = value::power_worth(tbl, p);
                                PowerFact {
                                    dear: ai::power_is_dear(p),
                                    unseen: wv.unseen,
                                    worth: wv.instant(tbl, &Ctx::typical(tbl)),
                                    tags: p.tags.clone(),
                                }
                            })
                            .collect()
                    })
                    .collect(),
            ),
            picked: std::array::from_fn(aic_core::modules::cards::picked_key),
        }
    }

    /// 082: each seat's strategy (see `agents/strategy.rs`).
    pub fn set_strategies(&mut self, c: Choice) {
        self.strat = c;
    }

    pub fn strategies(&self) -> Choice {
        self.strat
    }

    /// Seat `s`'s strategy multiplier for the card at content index `i` in round
    /// `round` (1 with no strategy).
    fn mult(&self, s: usize, i: usize, round: u32) -> f64 {
        match self.strat.get(s).copied().flatten() {
            Some(k) => self.mults.get(k, i, strategy::phase(round)),
            None => 1.0,
        }
    }

    /// 082: what action `a` does that the board terms can't see after it (a
    /// shelter, a cordon, a peek), in heuristic points: credited when it is taken.
    /// State-free by design: a flat worth per kind of action. A worth that depends
    /// on the table now (what a shelter saves this round) is a term of `value` on
    /// the state the action leaves, as 092's shelter read is (`guide.rs`).
    pub fn bonus(&self, world: &aic_core::module::World, a: &aic_core::Action) -> f64 {
        if self.w.table <= 0.0 {
            return 0.0;
        }
        let unseen = match a {
            aic_core::Action::Play { card, .. } => self.fact(world, card).1.value.play.unseen,
            aic_core::Action::Use { card, .. } => self.fact(world, card).1.value.use_.unseen,
            aic_core::Action::Power { seat, power } => {
                self.powers.get(*seat as usize).and_then(|p| p.get(*power as usize)).map_or(0.0, |p| p.unseen)
            }
            _ => 0.0,
        };
        self.w.table * unseen
    }

    /// A card in a pile, and what is read once about it.
    /// By reference: a `Fact` is several hundred bytes, and copying one out on
    /// every lookup was a tenth of the strategist's time (141).
    fn fact<'w, 's>(&'s self, world: &aic_core::module::World<'w>, id: &Key) -> (&'w CardDef, &'s Fact) {
        let f = &self.facts[world.content.card_index(id.as_str()).expect("card in content")];
        (&world.content.cards[f.index], f)
    }

    /// A supply ambassador's worth: the table's row in table mode (082).
    fn ambassador_unit(&self) -> f64 {
        if self.w.table > 0.0 {
            self.w.table * self.w.table_ambassador * self.tbl.res(Resource::Ambassadors)
        } else {
            self.w.ambassador
        }
    }

    fn cost(d: &CardDef) -> f64 {
        credit_cost(d).max(0) as f64
    }

    /// A card in hand, before any chance of paying for it. The packet's
    /// formula stays apart: its keep price sits inside the bracket, and moving
    /// it changes the last bits of a score and so the order of ties.
    fn card_value(w: &Weights, d: &CardDef) -> f64 {
        w.hand + w.hand_cost * Self::cost(d)
    }

    /// Adversary cards still to come: the face-up one and the deck.
    pub(crate) fn cards_left(g: &GameState) -> usize {
        g.pile(adversary::DECK).len() + g.pile(adversary::NEXT).len()
    }

    /// What a research cube seat `s` holds is worth (075).
    fn cube_unit(&self, s: usize) -> f64 {
        if self.cube_power[s] {
            self.w.research_cube_power
        } else {
            self.w.research_cube
        }
    }

    /// What an insight seat `s` holds is worth (075).
    fn insight_unit(&self, s: usize) -> f64 {
        if self.patient[s] {
            self.w.patience
        } else {
            self.w.insight
        }
    }

    /// An energy held (093): the table's rate in table mode with `table_energy`
    /// on, else the flat `energy` weight.
    fn energy_unit(&self) -> f64 {
        let w = &self.w;
        if w.table > 0.0 && w.table_energy > 0.0 {
            w.table * w.table_energy * self.tbl.res(Resource::Energy)
        } else {
            w.energy
        }
    }

    /// What `n` energy held is worth: its unit each, and (128, `energy_cap`)
    /// `energy_idle` each beyond the cap.
    fn energy_value(&self, n: f64) -> f64 {
        let w = &self.w;
        if w.energy_cap > 0.0 && n > w.energy_cap {
            self.energy_unit() * w.energy_cap + w.energy_idle * (n - w.energy_cap)
        } else {
            self.energy_unit() * n
        }
    }

    /// 128 (`energy_cap`): the share of the table's energy price an energy costs
    /// seat `s` (`Ctx::energy_rate`): an idle one's share once its pile passes the cap.
    pub fn energy_rate(&self, g: &GameState, s: usize) -> f64 {
        let w = &self.w;
        if w.energy_cap > 0.0 && g.resource(s, Resource::Energy) as f64 > w.energy_cap {
            (w.energy_idle / self.energy_unit().max(1e-9)).min(1.0)
        } else {
            1.0
        }
    }

    /// The strategy multipliers this heuristic was built with (`strat_scale`,
    /// `strat_contents`), which the keep plan shares.
    pub fn mults(&self) -> Arc<Mults> {
        Arc::clone(&self.mults)
    }

    /// 102: for the keep plan, in table credits as this heuristic prices them
    /// (its points over a table credit's): an energy held and an antimatter held
    /// (D23), and an adversary project wholly filled (D34: `project`, charged
    /// for each project's share filled); the credits it counts in full
    /// (`credit_cap`) and an idle credit's share of one beyond them.
    pub fn plan_rates(&self) -> PlanRates {
        let w = &self.w;
        let credit = if w.table > 0.0 { w.table } else { w.credits };
        let antimatter = if w.table > 0.0 && w.table_antimatter > 0.0 {
            w.table * w.table_antimatter * self.tbl.res(Resource::Antimatter)
        } else {
            w.antimatter
        };
        let cap = if w.credit_cap > 0.0 { w.credit_cap } else { f64::MAX };
        let idle = if w.credits > 0.0 { w.credit_idle / w.credits } else { 1.0 };
        PlanRates {
            energy: self.energy_unit() / credit,
            antimatter: antimatter / credit,
            project: w.project / credit,
            cap,
            idle,
            insight_cap: w.insight_cap,
            energy_cap: w.energy_cap,
            energy_share: (w.energy_idle / self.energy_unit().max(1e-9)).min(1.0),
        }
    }

    /// An energy a round to come (income, a city's production; 093): as held,
    /// less `rates.energy_income_share` in table mode, as the card prices count it.
    fn energy_income_unit(&self) -> f64 {
        let w = &self.w;
        if w.table > 0.0 && w.table_energy > 0.0 {
            self.energy_unit() * self.tbl.rate("energy_income_share")
        } else {
            w.energy
        }
    }

    /// What a seat's credits are worth: `credits` each up to `credit_cap`,
    /// `credit_idle` each beyond it (024).
    fn credit_value(&self, n: f64) -> f64 {
        let w = &self.w;
        let cap = if w.credit_cap > 0.0 { w.credit_cap } else { f64::MAX };
        w.credits * n.min(cap) + w.credit_idle * (n - cap).max(0.0)
    }

    /// What site `h` would count toward corrupting: its own adversary
    /// ambassadors (and a city's markers), plus, with a gathering line ahead,
    /// those next to it in its country (138).
    fn toward(rules: &Rules, t: &[HexTokens], h: usize, gather: bool) -> i64 {
        let world = rules.world();
        let mut n = t[h].adv_ambassador + if common::is_city(&world, h) { t[h].marker } else { 0 };
        if gather {
            let c = world.map.hexes[h].country;
            n += world.map.hexes[h].neighbors.iter().filter(|&&x| world.map.hexes[x].country == c).map(|&x| t[x].adv_ambassador).sum::<i64>();
        }
        n
    }

    /// Each seat's reach as a bit per hex (a board has at most 4 × 16 hexes),
    /// worked out once a position: `in_reach` per hex was a tenth of the
    /// search's time (028).
    fn reach_masks(&self, rules: &Rules, t: &[HexTokens]) -> [u64; 4] {
        let world = rules.world();
        let mut near = [0u64; 4];
        for (h, hex) in world.map.hexes.iter().enumerate() {
            for (s, mask) in near.iter_mut().enumerate().take(self.seats) {
                if t[h].node[s] > 0 {
                    *mask |= 1 << h;
                    for &n in &hex.neighbors {
                        *mask |= 1 << n;
                    }
                }
            }
        }
        near
    }

    /// The value of `g`. Finished games score `±win` plus a margin bonus.
    pub fn value(&self, rules: &Rules, g: &GameState) -> f64 {
        let w = &self.w;
        if let Some(o) = &g.outcome {
            return if o.won {
                w.win + w.win_margin * Self::cards_left(g) as f64
            } else {
                -w.win
            };
        }
        let world = rules.world();
        // The next corrupt step's number for each site, read off the card ahead
        // (public), and whether a Corrupt line there gathers from next door (138).
        let (corrupt_at, gather) = plan::next_corrupt_n(&world, g, CorruptSite::City);
        // 138: a player's research station or factory corrupts too, by its own number.
        let (station_at, station_gather) = plan::next_corrupt_n(&world, g, CorruptSite::Station);
        let (factory_at, factory_gather) = plan::next_corrupt_n(&world, g, CorruptSite::Factory);
        let mut v = 0.0;
        let mut corrupted = 0i64;
        let mut stations = [0i64; 4];
        let mut reach = 0i64;
        let tokens = self.keys.read(g);
        let near = self.reach_masks(rules, &tokens);
        let seats = self.seats;
        let mut pieces = 0i64;
        let level = alignment::bar_level(&world, g);
        // 136: clean cities filling toward corruption, with their risk.
        let mut risky: Vec<(usize, f64)> = Vec::new();
        for h in 0..g.hexes.len() {
            let t = &tokens[h];
            let city = common::is_city(&world, h);
            let base = common::is_base(&world, h);
            let adv = t.adv_ambassador;
            let bad = t.adv_cube > 0;
            let owned = t.cube[..seats].iter().any(|&n| n > 0);
            if city {
                if bad {
                    corrupted += 1;
                    // 137: the Liberate action pays one seat's supply at once (its
                    // adversary ambassadors add to the cost): gathering toward it, by
                    // the best-placed seat in reach, counts a little, so a seat saves up.
                    if let Some(need) = board::liberate_cost(&world, g, h) {
                        let held = (0..seats)
                            .filter(|&s| near[s] >> h & 1 != 0)
                            .map(|s| g.resource(s, Resource::Ambassadors))
                            .max()
                            .unwrap_or(0) as f64;
                        v += w.corrupted * w.freeing * 0.25 * (held / need.max(1) as f64).min(1.0);
                    }
                    // 137: a player ambassador standing on it blocks its diversion.
                    if t.ambassador[..seats].iter().any(|&n| n > 0) {
                        v += w.block;
                    }
                } else {
                    if owned {
                        v += w.aligned;
                        if adv > 0 {
                            // r38: an adversary ambassador darkens an aligned city too.
                            v -= w.city_dark_aligned;
                        }
                    } else if adv > 0 && (0..seats).any(|s| near[s] >> h & 1 != 0) {
                        // r37: an adversary ambassador darkens a neutral city: no sale
                        // to the seats that could buy its energy, until it is cleared.
                        v -= w.city_dark;
                    }
                    // 034: a corruption marker counts toward corrupting; 138: so do
                    // the ambassadors next door when a gathering Corrupt line is ahead.
                    let toward = Self::toward(rules, &tokens, h, gather);
                    if w.node_exposure > 0.0 && toward > 0 {
                        let r = ((toward + 1) as f64 / corrupt_at.max(1) as f64).min(1.0);
                        risky.push((h, r * r));
                    }
                    let p = (toward as f64 / corrupt_at as f64).min(1.0);
                    v -= w.corrupted * w.pressure * p * p;
                    if toward + 1 >= corrupt_at {
                        v -= w.at_risk;
                    }
                }
            }
            let held = t.node[..seats].iter().any(|&n| n > 0);
            v -= w.adv_ambassador * adv as f64;
            if adv > 0 {
                let piece = held || owned || base || t.factory > 0 || t.station > 0;
                if piece {
                    v -= w.on_piece * adv as f64;
                }
                if t.factory > 0 {
                    v -= w.on_factory;
                }
                // 137: it blocks a player's station or factory while it stands.
                if owned && !bad && (t.station > 0 || t.factory > 0) {
                    v -= w.block;
                }
            }
            v -= w.troop * t.troop as f64;
            if t.station > 0 {
                if bad {
                    // 102 (D28): the flip counts what its cubes still change: none
                    // once its country's megaproject has completed.
                    let flip = w.station_flip * open_share(&world, g, h);
                    v -= w.station + flip;
                    // 137: saving up for the Liberate action, by the best-placed
                    // seat in reach (as on a city); an ambassador standing on it blocks its compute.
                    if let Some(need) = board::liberate_cost(&world, g, h) {
                        let held = (0..seats)
                            .filter(|&s| near[s] >> h & 1 != 0)
                            .map(|s| g.resource(s, Resource::Ambassadors))
                            .max()
                            .unwrap_or(0) as f64;
                        let share = (held / need.max(1) as f64).min(1.0);
                        v += (w.station + flip) * w.station_freeing * share * share;
                    }
                    if t.ambassador[..seats].iter().any(|&n| n > 0) {
                        v += w.block;
                    }
                } else if owned {
                    let s = (0..self.seats).find(|&s| t.cube[s] > 0).unwrap_or(0);
                    stations[s] += 1;
                    v += if stations[s] <= STATIONS_USEFUL { w.station } else { w.station_spare };
                    // 138: a player's station filling toward its Corrupt number.
                    let p = (Self::toward(rules, &tokens, h, station_gather) as f64 / station_at.max(1) as f64).min(1.0);
                    v -= w.site_pressure * (w.station + w.station_flip) * w.pressure * p * p;
                }
            }
            v += w.factory * t.factory as f64 + w.harden * t.harden as f64;
            if t.factory > 0 {
                if bad {
                    v -= w.owned_factory;
                    // 137: a player ambassador on an adversary factory blocks its troop.
                    if t.ambassador[..seats].iter().any(|&n| n > 0) {
                        v += w.block;
                    }
                } else if owned {
                    v += w.owned_factory;
                    // 138: a player's factory filling toward its Corrupt number.
                    let p = (Self::toward(rules, &tokens, h, factory_gather) as f64 / factory_at.max(1) as f64).min(1.0);
                    v -= w.site_pressure * 2.0 * w.owned_factory * w.pressure * p * p;
                }
            }
            // 137: your own ambassador on your own piece's hex is armor.
            if w.armor > 0.0 && (0..seats).any(|s| t.ambassador[s] > 0 && (t.node[s] > 0 || (t.cube[s] > 0 && t.station + t.factory > 0))) {
                v += w.armor;
            }
            v -= w.botnet * t.botnet as f64;
            v -= w.marker * t.marker as f64;
            if held && t.harden > 0 {
                v += w.shield;
            }
            // 075: a Wall's facility can't be struck.
            if owned && !bad && (t.station > 0 || t.factory > 0) && self.wall_seat.is_some_and(|s| t.node[s] > 0) {
                let near = (0..g.hexes.len()).any(|x| tokens[x].troop > 0 && world.map.distance(h, x) <= WALL_NEAR as i32);
                v += w.wall * if near { 2.0 } else { 1.0 };
            }
            if held {
                v += w.node;
            }
            for s in 0..self.seats {
                v += w.ambassador_board * t.ambassador[s] as f64;
            }
            let counts = city || base || t.station > 0;
            if counts && (0..self.seats).any(|s| near[s] >> h & 1 != 0) {
                reach += 1;
            }
            pieces += adversary::against_bar(city && bad, t.botnet, adv, t.troop, level);
        }
        v -= w.corrupted * corrupted as f64;
        // 082: in table mode a liberation counts as the city it frees.
        if w.table <= 0.0 {
            v += w.liberated * g.stats.counters.get("board.cities_freed").copied().unwrap_or(0) as f64;
        }
        let mut thin = i64::MAX;
        // 138: corruption destroys only a node on the corrupted hex itself.
        let blast = 0;
        for s in 0..self.seats {
            let mut count = tokens.iter().filter(|t| t.node[s] > 0).count() as i64;
            if !risky.is_empty() {
                // 136: a node in a filling city's blast is that share of lost.
                let exposed: f64 = (0..g.hexes.len())
                    .filter(|&h| tokens[h].node[s] > 0)
                    .map(|h| risky.iter().filter(|&&(c, _)| world.map.distance(h, c) <= blast).map(|&(_, r)| r).fold(0.0, f64::max))
                    .sum();
                v -= w.node_exposure * w.node * exposed;
                count -= exposed.round() as i64;
            }
            thin = thin.min(count);
            match count {
                ..=1 => v -= w.node_danger,
                2 => v -= 0.5 * w.node_danger,
                // 034: a token lost stays lost and a hit takes 2, so the danger
                // starts sooner.
                3 => v -= 0.25 * w.node_danger,
                4 => v -= 0.1 * w.node_danger,
                _ => {}
            }
        }
        // 092: a shelter is worth the destruction showing that it stops.
        if w.shelter_read > 0.0 {
            for saves in guide::sheltered_saves(&world, g, w.shelter_counter > 0.0) {
                for s in 0..self.seats.min(4) {
                    let k = saves.nodes[s];
                    if k <= 0.0 {
                        continue;
                    }
                    let count = tokens.iter().filter(|t| t.node[s] > 0).count() as i64;
                    let step = danger_share(count - 1) - danger_share(count);
                    v += w.shelter_read * k * (w.node + w.shelter_node + step * w.node_danger);
                }
                v += w.shelter_read * saves.factories * (w.owned_factory + w.factory);
            }
        }
        v += w.reach * reach as f64;
        // The Discord deck: the tip that finds it empty loses.
        let discord_left = alignment::cards_left(g, Side::Discord) as i64;
        v -= w.discord_danger * (3 - discord_left).max(0) as f64;
        v += w.unity_deck * (3 - alignment::cards_left(g, Side::Unity) as i64).max(0) as f64;
        // Megaprojects: each player cube pays when the project completes.
        for c in 0..world.map.countries.len() {
            if g.var(&board::project_done(c)) > 0 {
                // r37: the converter needs a node in a city of its country.
                if w.converter_city != 0.0 {
                    for s in 0..self.seats {
                        // 102 (D27): whatever the cubes, so the last cube is worth spending there.
                        if common::has_city_node_in(&world, g, s, c) {
                            v += w.converter_city;
                        }
                    }
                }
                continue;
            }
            let mine: i64 = (0..self.seats)
                .map(|s| g.var(&board::project_key(c, Some(s))))
                .sum();
            let theirs = g.var(&board::project_key(c, None));
            v += w.project_cube * (mine as f64 - 0.5 * theirs as f64);
        }
        for s in 0..self.seats {
            v += self.ambassador_unit() * g.resource(s, Resource::Ambassadors) as f64;
            v += w.chips * g.resource(s, Resource::Chips) as f64;
        }
        // The win bar: what still stands between the team and a clear board.
        v -= w.blocker * pieces as f64;
        // Alignment: tips made, and each bucket's share of its next tip.
        for (side, weight) in [(Side::Unity, w.unity_tip), (Side::Discord, -w.discord_tip)] {
            let cap = alignment::capacity(&world, g, side).max(1) as f64;
            // 145: Unity swayed and waiting on the hold-off ask counts as on its
            // track ("all onto the Unity track" is always offered); unread, a sway
            // whose ask fell past the search's choices was worth nothing.
            let pending = if side == Side::Unity { alignment::unity_pending(g) } else { 0 };
            let fill = ((g.var(alignment::track(side)) + pending) as f64 / cap).min(1.0);
            v += weight * (g.var(alignment::tips_key(side)) as f64 + fill);
        }
        v += w.level * level.steps() as f64;
        // A resolved (or ongoing) project counts in full, so letting one
        // fill never scores better than holding it back.
        for id in adversary::projects(g) {
            let cost = adversary::project_cost(&world, g, &id).max(1) as f64;
            v -= w.project * g.var(&adversary::project_key(&id)) as f64 / cost;
        }
        let resolved = g.stats.counters.get("adversary.projects_resolved");
        v -= w.project * resolved.copied().unwrap_or(0) as f64;
        // Added in place, term by term, so the flat agents' values (and so
        // their games) do not depend on horizon mode.
        let unpaid = self.unpaid_threats(rules, g);
        if w.horizon > 0.0 {
            v += self.horizon_value(rules, g, &tokens, &unpaid, thin, discord_left);
        } else {
            self.flat_value(rules, g, &mut v);
        }
        for share in unpaid {
            v -= w.threat * share;
        }
        v -= self.clock_penalty(g);
        v
    }

    /// Each active threat's share still unpaid.
    fn unpaid_threats(&self, rules: &Rules, g: &GameState) -> Vec<f64> {
        let world = rules.world();
        let w = &self.w;
        let mut out = Vec::new();
        for (t, scope) in threats::active(g) {
            // 135: the cheapest resource option of the menu (a tag or node option
            // is a whole payment, priced by what it removes, not pooled).
            // 145: the share paid in each option's own currencies (summed units across
            // currencies read 1 credit toward 15 as a third of a 3-energy counter).
            let (need, share) = threats::need_and_share(&world, g, &t);
            if need <= 0 {
                continue;
            }
            let waits = if threats::waits(g, &t) { w.threat_waits } else { 1.0 };
            let open = (0..self.seats).any(|s| threats::may_counter(&world, g, s, &t, Some(scope)));
            let stranded = if open { 1.0 } else { w.threat_stranded };
            // 146: weighed by what it would do uncountered, against a tier-1 counter's worth.
            let harm = if w.threat_harm > 0.0 {
                let h = self.threat_harms.iter().find(|(k, _)| *k == t).map_or(w.threat_harm_ref, |(_, h)| *h);
                (1.0 - w.threat_harm + w.threat_harm * h / w.threat_harm_ref.max(1.0)).max(0.2)
            } else {
                1.0
            };
            out.push(waits * stranded * harm * (1.0 - share.min(1.0)));
        }
        out
    }

    /// The rounds of income still to come, as the table reads: the adversary
    /// deck's clock (each card left is one more round), cut short by when
    /// the team is on course to win (Unity still needed at its rate so far,
    /// plus `win_lag`), then the expected rounds survived at a per-round
    /// loss chance of `risk` per unit of danger (unpaid threats per
    /// country, a seat's last node, the Discord deck at its edge).
    fn rounds_left(
        &self,
        rules: &Rules,
        g: &GameState,
        unpaid: &[f64],
        thin: i64,
        discord_left: i64,
    ) -> f64 {
        let w = &self.w;
        let clock = Self::cards_left(g) as f64;
        let (points, bar) = self.unity_progress(g);
        let to_win = ((bar - points).max(0.0) / self.unity_rate(g)).ceil() + w.win_lag;
        let rounds = clock.min(to_win).max(0.0);
        let world = rules.world();
        let countries = world.map.countries.len().max(1) as f64;
        let mut danger = unpaid.iter().sum::<f64>() / countries;
        if discord_left <= 1 {
            danger += 0.5;
        }
        if thin <= 1 {
            danger += 0.5;
        }
        let p = (1.0 - w.risk * danger).clamp(0.0, 1.0);
        // Sum of p^k for k = 1..=rounds, with a fractional last round.
        let mut total = 0.0;
        let mut surv = 1.0;
        let mut k = 0.0;
        while k < rounds {
            surv *= p;
            total += surv * (rounds - k).min(1.0);
            k += 1.0;
        }
        total
    }

    /// Unity so far, in Sway (each tip counts a full bucket at the base
    /// size), and the Sway the target (the outright Unity win) needs.
    fn unity_progress(&self, g: &GameState) -> (f64, f64) {
        let per = (self.tip_per_player * self.seats as i64) as f64;
        let tips = g.var(alignment::tips_key(Side::Unity)) as f64;
        let points = tips * per + (g.var(alignment::track(Side::Unity)) + alignment::unity_pending(g)) as f64;
        (points, self.target_tips as f64 * per)
    }

    /// Unity gained per round so far, in Sway (`unity_rate_start` before the
    /// team has a record of its own). Sway held may include this round's, so
    /// this round counts as played.
    fn unity_rate(&self, g: &GameState) -> f64 {
        let w = &self.w;
        let points = self.unity_progress(g).0;
        if g.round > 1 {
            (points / g.round as f64).max(w.unity_rate_start * 0.25)
        } else {
            w.unity_rate_start
        }
        .max(0.1)
    }

    /// The deck-clock penalty: `clock` per round the team, at its Unity
    /// rate so far, would still be short of its Unity target when the deck
    /// runs out. With `k` adversary cards left (the face-up one included),
    /// `k` action phases come after this one (the build step after the last
    /// loses); this round's points count as they are scored, not as a round
    /// still to come, so in the last round every point short costs
    /// `clock` / rate.
    fn clock_penalty(&self, g: &GameState) -> f64 {
        if self.w.clock == 0.0 {
            return 0.0;
        }
        let clock = Self::cards_left(g) as f64;
        let (points, top) = self.unity_progress(g);
        let need = (top - points).max(0.0) / self.unity_rate(g);
        self.w.clock * (need - clock).max(0.0)
    }

    /// Per-seat resources, income, hands and buildings, by flat weights.
    fn flat_value(&self, rules: &Rules, g: &GameState, v: &mut f64) {
        let w = &self.w;
        let world = rules.world();
        for s in 0..self.seats {
            let res = |k: Resource| g.resource(s, k) as f64;
            *v += self.credit_value(res(Resource::Credits))
                + self.energy_value(res(Resource::Energy))
                + w.antimatter * res(Resource::Antimatter)
                + self.insight_value(s, res(Resource::Insight))
                + self.cube_unit(s) * res(Resource::ResearchCubes);
            for (r, n) in ai::track_income(&world, g, s) {
                let unit = match r {
                    Resource::Antimatter => ANTIMATTER_INCOME_UNIT,
                    Resource::Insight => INSIGHT_INCOME_UNIT,
                    _ => 1.0,
                };
                *v += w.income * unit * n as f64;
            }
            let mut cards: Vec<f64> = g
                .pile(&cards::hand(s))
                .iter()
                .map(|id| Self::card_value(w, cards::def(&world, id)))
                .collect();
            let drafted = g.pile(&cards::drafted(s)).iter().map(|id| {
                let d = cards::def(&world, id);
                (w.drafted * Self::card_value(w, d), d.tier)
            });
            let packet = g.pile(&cards::packet(s)).iter().map(|id| {
                let d = cards::def(&world, id);
                let keep = self.keep_price(&world, g, s, d.tier, keep_insight(&world, d.tier));
                let v = w.hand + w.hand_cost * (Self::cost(d) - w.packet_keep * keep);
                (w.drafted * v.max(0.0), d.tier)
            });
            let picks: Vec<(f64, u8)> = drafted.chain(packet).collect();
            self.keepable(&world, g, s, picks, &mut cards);
            *v += Self::capped(&mut cards, w.hand_cap);
            for id in g.pile(&cards::built(s)) {
                *v += w.built + w.built_cost * Self::cost(cards::def(&world, id));
            }
            self.in_play_value(rules, g, s, None, v);
        }
    }

    /// Adds to `cards` the value of each of seat `s`'s picks (value, tier) that
    /// its insight this round keeps, best first (046; `packet_budget` 0: all).
    fn keepable(
        &self,
        world: &aic_core::module::World,
        g: &GameState,
        s: usize,
        mut picks: Vec<(f64, u8)>,
        cards: &mut Vec<f64>,
    ) {
        if self.w.packet_budget <= 0.0 {
            cards.extend(picks.iter().map(|p| p.0));
            return;
        }
        picks.sort_by(|a, b| b.0.total_cmp(&a.0));
        let mut left = g.resource(s, Resource::Insight);
        let mut kept = 0;
        for (v, tier) in picks {
            let (need, _) = cards::keep_need(world, g, s, tier, kept);
            if need <= left {
                left -= need;
                kept += cards::counts_as_kept(g, tier) as i64;
                cards.push(v);
            }
        }
    }

    /// The best `cap` of a seat's card values (021: hands outgrew what a
    /// seat can play, so cards beyond a playable hand are worth nothing and
    /// agents stop keeping every card they draft). `cap` 0: no cap.
    fn capped(cards: &mut [f64], cap: f64) -> f64 {
        if cap <= 0.0 {
            return cards.iter().sum();
        }
        cards.sort_by(|a, b| b.total_cmp(a));
        cards.iter().take(cap as usize).sum()
    }

    /// 021: programs in the play area (their tags pay powers) and tokens on
    /// cards, at flat weights; with `table` (082), a token at what its card's
    /// spend pays per token.
    fn in_play_value(&self, rules: &Rules, g: &GameState, s: usize, table: Option<&Ctx>, v: &mut f64) {
        let world = rules.world();
        *v += self.w.program * g.pile(&cards::played(s)).len() as f64;
        let b = ai::board(&world, s);
        let share = |taken: usize, len: usize| if len == 0 { 0.0 } else { (taken as f64 / len as f64).powi(2) };
        *v += self.w.track_capstone
            * (share(board::track_taken(g, s), b.economy_track.len()) + share(board::track2_taken(g, s), b.play_track.len()));
        if let Some(c) = table {
            for id in cards::token_cards(&world, g, s, None) {
                let k = g.var(&cards::tokens_key(s, &id));
                let (_, f) = self.fact(&world, &id);
                *v += self.w.table * k as f64 * f.token.instant(self.tbl, c).max(0.0);
            }
            return;
        }
        let corrupted = (0..g.hexes.len()).any(|h| common::corrupted_city(&world, g, h));
        for id in cards::token_cards(&world, g, s, None) {
            let k = g.var(&cards::tokens_key(s, &id));
            *v += self.w.token * k as f64;
            // 080: tokens toward a liberation count for it. 099: either half (r37's
            // Popular Uprising spends them in its only one; reading `or` alone left
            // this dead).
            let liberating = cards::def(&world, &id).ability.as_ref().and_then(|a| {
                a.branches().into_iter().find(|b| {
                    b.tokens > 0 && b.effects.iter().any(|e| matches!(e, aic_core::content::Op::Liberate { .. }))
                })
            });
            if let Some(need) = liberating {
                if corrupted {
                    *v += self.w.liberate_progress * (k as f64 / need.tokens as f64).min(1.0);
                }
            }
        }
    }

    /// What the board makes each seat a round if production came now: the
    /// energy of its aligned cities, less what adversary ambassadors divert.
    fn board_production(&self, rules: &Rules, tokens: &[HexTokens]) -> Vec<i64> {
        let world = rules.world();
        let mut out = vec![0; self.seats];
        for (h, t) in tokens.iter().enumerate() {
            if t.adv_ambassador > 0 || t.adv_cube > 0 {
                continue;
            }
            if common::is_city(&world, h) {
                if let Some(s) = (0..self.seats).find(|&s| t.cube[s] > 0) {
                    out[s] += self.city_energy + self.aligned_bonus;
                }
            }
        }
        out
    }

    /// 107: seat `s`'s cards in play and in hand counted by tag, and how many.
    fn mix_counts(
        &self,
        world: &aic_core::module::World,
        g: &GameState,
        s: usize,
        tags: [i64; TAGS.len()],
        n: i64,
    ) -> ([i64; TAGS.len()], i64) {
        let (mut counts, mut cards_n) = (tags, n);
        for id in g.pile(&cards::hand(s)) {
            cards_n += 1;
            for t in &self.fact(world, id).0.tags {
                counts[*t as usize] += 1;
            }
        }
        (counts, cards_n)
    }

    /// 128 (`tag_hand`): seat `s`'s tags in play plus `tag_hand` of those on the
    /// cards in its hand that stay in play (events leave none), rounded.
    fn hand_tags(&self, world: &aic_core::module::World, g: &GameState, s: usize, tags: [i64; TAGS.len()]) -> [i64; TAGS.len()] {
        let mut held = [0i64; TAGS.len()];
        for id in g.pile(&cards::hand(s)) {
            let d = self.fact(world, id).0;
            if d.is_event() {
                continue;
            }
            for t in &d.tags {
                held[*t as usize] += 1;
            }
        }
        let mut out = tags;
        for (o, h) in out.iter_mut().zip(held) {
            *o += (self.w.tag_hand * h as f64).round() as i64;
        }
        out
    }

    /// 124 (`insight_cap`): the insight seat `s` holds beyond the cap (0: off).
    fn spare_insight(&self, g: &GameState, s: usize) -> f64 {
        if self.w.insight_cap <= 0.0 {
            return 0.0;
        }
        (g.resource(s, Resource::Insight) as f64 - self.w.insight_cap).max(0.0)
    }

    /// What seat `s`'s `held` insight is worth: its unit each, and (124,
    /// `insight_cap`) `insight_idle` each beyond the cap.
    fn insight_value(&self, s: usize, held: f64) -> f64 {
        let w = &self.w;
        if w.insight_cap > 0.0 && held > w.insight_cap {
            self.insight_unit(s) * w.insight_cap + w.insight_idle * (held - w.insight_cap)
        } else {
            self.insight_unit(s) * held
        }
    }

    /// 124 (`insight_cap`): what a keep of `tier` costs seat `s`, `keep` at full:
    /// the share of its insight its spare insight pays costs the idle rate.
    fn keep_price(&self, world: &aic_core::module::World, g: &GameState, s: usize, tier: u8, keep: f64) -> f64 {
        let spare = self.spare_insight(g, s);
        if spare <= 0.0 {
            return keep;
        }
        let need = keep_insight(world, tier).max(1.0);
        let covered = (spare / need).min(1.0);
        let unit = self.insight_unit(s).max(1e-9);
        keep * (1.0 - covered * (1.0 - (self.w.insight_idle / unit).min(1.0)))
    }

    /// Per-seat resources, income, hands and buildings in horizon mode.
    fn horizon_value(
        &self,
        rules: &Rules,
        g: &GameState,
        tokens: &[HexTokens],
        unpaid: &[f64],
        thin: i64,
        discord_left: i64,
    ) -> f64 {
        let w = &self.w;
        let world = rules.world();
        let left = self.rounds_left(rules, g, unpaid, thin, discord_left);
        let prod = if w.production != 0.0 {
            self.board_production(rules, tokens)
        } else {
            vec![0; self.seats]
        };
        // Resources fade once fewer than `spend_horizon` rounds are left.
        let fade = if w.spend_horizon > 0.0 {
            ((left + 1.0) / w.spend_horizon).min(1.0)
        } else {
            1.0
        };
        // 021: chips sell 1:1 for credits (what a seat holds counts in `chips`).
        // 082, a probe: antimatter held and earned at the table's row.
        let antimatter = if w.table > 0.0 && w.table_antimatter > 0.0 {
            w.table * w.table_antimatter * self.tbl.res(Resource::Antimatter)
        } else {
            w.antimatter
        };
        let unit = [w.credits, w.credits, self.energy_income_unit(), antimatter, w.insight, w.research_cube];
        let mut v = 0.0;
        // 074: an owned station makes a cube a round for the rest of the game.
        if w.production != 0.0 {
            let mut owned = [0i64; 4];
            for t in tokens.iter().filter(|t| t.station > 0 && t.adv_cube == 0) {
                if let Some(s) = (0..self.seats).find(|&s| t.cube[s] > 0) {
                    owned[s] += 1;
                }
            }
            let useful: i64 = owned.iter().map(|&n| n.min(STATIONS_USEFUL)).sum();
            v += w.horizon * w.production * left * w.station_round * useful as f64;
        }
        let tbl = self.tbl;
        let table = w.table > 0.0;
        let corrupted = table && (0..g.hexes.len()).any(|h| common::corrupted_city(&world, g, h));
        // What each seat's cards are judged by, worked out once: the packet a seat
        // has picked from counts for the seat it goes to (082).
        let seats: Vec<SeatData> = (0..self.seats)
            .map(|s| {
                let terms = || {
                    let tags = cards::tag_counts(&world, g, s);
                    let mut required = tags;
                    for t in ai::board_tags(&world, g, s) {
                        required[*t as usize] += 1;
                    }
                    let income = economy::income_with(&world, g, s, &tags);
                    (tags, required, cards::prices_with(&world, g, s, &tags), income, in_play(g, s))
                };
                let (tags, required, prices, income, n) = seat_terms_cached(self.id, g, s, terms);
                let node = tokens.iter().filter(|t| t.node[s] > 0).count() as f64;
                let mut ctx = Ctx::at(tbl, left, node, tags, corrupted);
                if table && self.mix {
                    // 107: the share of the seat's cards, in play and in hand, by tag.
                    let (counts, cards_n) = self.mix_counts(&world, g, s, tags, n);
                    ctx = ctx.with_mix(tbl, &counts, cards_n);
                }
                if table && w.tag_hand > 0.0 {
                    // 128: per-tag ops at the count the seat can expect: its tags in
                    // play and a share of those on the cards it holds to play.
                    ctx.tags = self.hand_tags(&world, g, s, tags);
                }
                ctx.energy_rate = self.energy_rate(g, s);
                ctx.cubes_open =
                    w.cube_target <= 0.0 || !aic_core::modules::effects::cube_projects(&world, g, s, false).is_empty();
                SeatData { s, tags, required, prices, income, ctx }
            })
            .collect();
        // 124 (D47, `team_share`): a trigger on a teammate's plays fires at the
        // other seats' share of their cards with the tag.
        let mut seats = seats;
        if table && self.mix && w.team_share > 0.0 && self.seats > 1 {
            let mixes: Vec<([i64; TAGS.len()], i64)> =
                seats.iter().map(|sd| self.mix_counts(&world, g, sd.s, sd.tags, in_play(g, sd.s))).collect();
            for sd in seats.iter_mut() {
                let others = mixes.iter().enumerate().filter(|&(o, _)| o != sd.s).map(|(_, m)| *m);
                sd.ctx.team = Some(team_share(tbl, others));
            }
        }
        let mut packets: Vec<Vec<(f64, u8)>> = vec![Vec::new(); self.seats];
        let step = if g.round % 2 == 1 { 1 } else { self.seats - 1 };
        for s in 0..self.seats {
            let to = if table && g.var(&self.picked[s]) > 0 { (s + step) % self.seats } else { s };
            for id in g.pile(&cards::packet(s)) {
                let tier = cards::def(&world, id).tier;
                let keep = if table { w.table * tbl.keep(tier) } else { w.hand_cost * keep_insight(&world, tier) };
                let keep = self.keep_price(&world, g, to, tier, keep);
                let worth = self.hand_worth(&world, g, &seats[to], left, id, true);
                packets[to].push((w.drafted * (worth - w.packet_keep * keep).max(0.0), tier));
            }
        }
        for (s, made) in prod.iter().enumerate() {
            let sd = &seats[s];
            let res = |k: Resource| g.resource(s, k);
            // Insight is reset each round, so what is held now only counts this round.
            for (i, r) in RESOURCES.iter().enumerate() {
                if *r == Resource::Credits {
                    v += fade * self.credit_value(res(*r) as f64);
                } else if *r == Resource::Insight {
                    v += fade * self.insight_value(s, res(*r) as f64);
                } else if *r == Resource::ResearchCubes {
                    v += fade * self.cube_unit(s) * res(*r) as f64;
                } else if *r == Resource::Energy {
                    v += fade * self.energy_value(res(*r) as f64);
                } else if *r != Resource::Chips {
                    v += fade * unit[i] * res(*r) as f64;
                }
            }
            self.in_play_value(rules, g, s, table.then_some(&sd.ctx), &mut v);
            // Income (tracks, buildings with income passives, rewards) over
            // the rounds left; board production counts `production` of its own.
            let income = sd.income;
            for (i, (r, n)) in income.iter().enumerate() {
                let u = if table && w.table_income > 0.0 {
                    // 082, a probe: income at the table's rates.
                    w.table * w.table_income * tbl.res(r)
                } else if r == Resource::ResearchCubes {
                    self.cube_unit(s)
                } else if r == Resource::Insight && self.patient[s] && self.spare_insight(g, s) > 0.0 {
                    // 124 (`insight_cap`): under Patience the pile carries, so more
                    // insight on a spare pile is spare too (without it the pile resets).
                    w.insight_idle
                } else if r == Resource::Insight && w.insight_income > 0.0 {
                    // 124 (D53): an insight a round at its own weight.
                    w.insight_income
                } else {
                    unit[i]
                };
                v += w.horizon * left * u * n as f64;
            }
            v += w.horizon * w.production * left * self.energy_income_unit() * *made as f64;
            for id in g.pile(&cards::built(s)) {
                let (d, f) = self.fact(&world, id);
                v += w.built + w.built_cost * f.cost;
                if table {
                    // 082: what it earns a round that income doesn't count.
                    v += w.table * left * f.value.built_round(tbl, &sd.ctx);
                    continue;
                }
                if !d.pays_income() {
                    v += w.ability * left;
                }
                // 070: a standing counter or drain saves credits every round.
                for p in &d.passives {
                    let saved = match p {
                        Op::AutoCounter { amount, .. } => 0.5 * *amount as f64,
                        Op::ProjectDrain { amount } => 2.0 * *amount as f64,
                        _ => 0.0,
                    };
                    v += w.horizon * left * w.credits * saved;
                }
            }
            if table {
                v += self.power_progress(sd);
            }
            let mut cards: Vec<f64> =
                g.pile(&cards::hand(s)).iter().map(|id| self.hand_worth(&world, g, sd, left, id, false)).collect();
            let mut picks = Vec::new();
            for id in g.pile(&cards::drafted(s)) {
                picks.push((w.drafted * self.hand_worth(&world, g, sd, left, id, true), cards::def(&world, id).tier));
            }
            picks.append(&mut packets[s]);
            self.keepable(&world, g, s, picks, &mut cards);
            v += Self::capped(&mut cards, w.hand_cap);
        }
        v
    }

    /// A card in seat `sd`'s hand: its worth times the chance of paying for it
    /// before the game ends, from what the seat has and earns. In table mode
    /// (082) its worth is its net value by the table, less its cost to play, times
    /// the progress term for its `needs`; a card still to be drafted or kept
    /// (`pick`) also counts the seat's strategy multiplier. Not one in hand: what
    /// playing it brings is the board's terms, which no strategy multiplies, so a
    /// multiplied hand would make holding a strategy's cards beat playing them.
    /// Else the flat `hand` + `hand_cost` × its cost.
    fn hand_worth(&self, world: &aic_core::module::World, g: &GameState, sd: &SeatData, left: f64, id: &Key, pick: bool) -> f64 {
        let w = &self.w;
        let s = sd.s;
        let (d, f) = self.fact(world, id);
        let res = |k: Resource| g.resource(s, k);
        let mut p = 1.0;
        for (have, need, earn) in [
            (res(Resource::Credits), sd.prices.credits(d), sd.income[Resource::Credits]),
            (res(Resource::Antimatter), d.antimatter, sd.income[Resource::Antimatter]),
            (res(Resource::ResearchCubes), d.research_cubes, 1),
        ] {
            let short = need - have;
            if short > 0 {
                if earn <= 0 {
                    return 0.0;
                }
                let rounds = (short as f64 / earn as f64).ceil();
                p = f64::min(p, (1.0 - rounds / (left + 1.0)).max(0.0));
            }
        }
        if w.table <= 0.0 {
            return p * (w.hand + w.hand_cost * f.cost);
        }
        let tbl = self.tbl;
        let (m, share) = if pick { (self.mult(s, f.index, g.round), w.pick_cost) } else { (1.0, 1.0) };
        // 128: its energy at the seat's rate (spare energy is cheap).
        let spare = d.energy as f64 * tbl.res(Resource::Energy) * (1.0 - sd.ctx.energy_rate);
        let net = m * f.value.total(tbl, &sd.ctx) - share * (tbl.cost(d, sd.prices.credits(d), sd.prices.insight(d), left) - spare);
        // A card that works on a pile of the seat's own with none to work on counts
        // as a `needs` card with none of its tags.
        let pile = match f.pile {
            Some(k) if !cards::has_token_card(world, g, s, Some(k)) => w.needs_floor,
            _ => 1.0,
        };
        p * w.table * w.hand_share * net.max(0.0) * self.needs_factor(d, &sd.required) * pile
    }

    /// The progress term for a card's `needs` (082): 1 once met, else
    /// `needs_floor` plus the rest times the square of the share of the tags held.
    fn needs_factor(&self, d: &CardDef, required: &[i64; TAGS.len()]) -> f64 {
        if d.needs.is_empty() {
            return 1.0;
        }
        let (mut have, mut need) = (0i64, 0i64);
        for (t, n) in &d.needs {
            have += required[*t as usize].min(*n);
            need += *n;
        }
        if have >= need {
            return 1.0;
        }
        let share = have as f64 / need.max(1) as f64;
        self.w.needs_floor + (1.0 - self.w.needs_floor) * share * share
    }

    /// The progress term for seat `sd`'s dear powers (082): each power that
    /// sacrifices two tags or more counts `power_progress` of a use, times the
    /// square of the share of the tags the seat holds toward it.
    fn power_progress(&self, sd: &SeatData) -> f64 {
        let Some(powers) = self.powers.get(sd.s) else { return 0.0 };
        let all: i64 = sd.tags.iter().sum();
        let mut v = 0.0;
        for p in powers {
            let need: i64 = p.tags.iter().map(|(_, n)| n).sum();
            // 102 (D33): the rules' own test, which since r35 is what makes a power dear.
            let dear = p.dear && need > 0;
            if !dear || p.worth <= 0.0 {
                continue;
            }
            let have: i64 = p
                .tags
                .iter()
                .map(|(t, n)| match t {
                    TagOrAny::Is(t) => sd.tags[*t as usize].min(*n),
                    TagOrAny::Any => all.min(*n),
                })
                .sum();
            let share = have as f64 / need as f64;
            v += self.w.table * self.w.power_progress * p.worth * share * share;
        }
        v
    }
}

/// What one seat's cards are judged by in a position (082).
/// A seat's tags, `needs` tags, prices and income: what [`Eval::value`] reads
/// off its tableau, kept a decision long (103: worked out for every position, they
/// were a sixth of the strategist's time, and a search's positions mostly share them).
type SeatTerms = ([i64; TAGS.len()], [i64; TAGS.len()], cards::Prices, aic_core::content::PerResource, i64);

thread_local! {
    /// The seat terms worked out this decision, by a hash of [`tableau_parts`],
    /// with the parts to confirm a hit by.
    static SEAT_TERMS: std::cell::RefCell<SeatStore> = std::cell::RefCell::new(SeatStore::default());
}

/// Tableaus the store holds before it starts over (a decision reaches a few dozen).
const SEAT_STORE_CAP: usize = 4096;

#[derive(Default)]
struct SeatStore {
    terms: HashMap<u64, (Vec<u64>, SeatTerms), BuildHasherDefault<Fnv>>,
    parts: Vec<u64>,
}

/// Empties the seat-terms store: called as each decision's scoring starts, so
/// it holds one decision's tableaus.
pub fn new_decision() {
    SEAT_TERMS.with(|c| c.borrow_mut().terms.clear());
}

/// Gives each [`Eval`] (and its clones, a game's) an id for the store's key.
static EVAL_IDS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn next_eval_id() -> u64 {
    EVAL_IDS.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// Everything seat `s`'s terms are read from besides the rules (the [`Eval`]'s
/// `id` stands for those): its tracks, the cards of its piles in play by their
/// interned names (whose addresses never move), its production vars and the
/// team's income bonuses. Equal parts, equal terms: `tag_counts`, `board_tags`,
/// `income_with` and `prices_with` read nothing else, and a debug build checks
/// each hit against a fresh reading.
fn tableau_parts(id: u64, g: &GameState, s: usize, out: &mut Vec<u64>) {
    out.clear();
    out.extend([id, s as u64, board::track_taken(g, s) as u64, board::track2_taken(g, s) as u64]);
    out.extend(g.pile(&cards::built(s)).iter().map(|c| c.as_str().as_ptr() as u64));
    out.push(1);
    // Every pile under one of the seat's cards, by name, in one walk (104: a
    // lookup a top was most of this): the tops' own are among them.
    let under = aic_core::key::KeyBuf::new("cards.under.").num(s).text(".");
    for (k, pile) in g.piles.from_key(under.as_str()).take_while(|(k, _)| k.starts_with(under.as_str())) {
        out.push(k.as_str().as_ptr() as u64);
        out.extend(pile.iter().map(|c| c.as_str().as_ptr() as u64));
        out.push(1);
    }
    out.push(2);
    out.extend(g.pile(&cards::played(s)).iter().map(|c| c.as_str().as_ptr() as u64));
    out.push(3);
    out.extend([alignment::income_bonus(g) as u64, alignment::insight_bonus(g) as u64]);
    let prefix = aic_core::key::KeyBuf::new("economy.production.").num(s).text(".");
    for (k, n) in g.vars.from_key(prefix.as_str()).take_while(|(k, _)| k.starts_with(prefix.as_str())) {
        out.extend([k.as_str().as_ptr() as u64, *n as u64]);
    }
    // 145: which buildings are face down (135): their passives drop out of
    // income and prices, and two answers to "turn which face down?" differed
    // only here (the store gave every option the first one's terms).
    if cards::has_disabled(g, s) {
        out.push(4);
        out.extend(g.pile(&cards::built(s)).iter().map(|c| cards::disabled(g, s, c.as_str()) as u64));
    }
}

fn seat_terms_cached(id: u64, g: &GameState, s: usize, terms: impl Fn() -> SeatTerms) -> SeatTerms {
    SEAT_TERMS.with(|c| {
        let store = &mut *c.borrow_mut();
        tableau_parts(id, g, s, &mut store.parts);
        let key = store.parts.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, x| (h ^ x).wrapping_mul(0x0000_0100_0000_01b3));
        if let Some((parts, t)) = store.terms.get(&key) {
            if *parts == store.parts {
                debug_assert!(same_terms(t, &terms()), "seat terms read state tableau_parts leaves out");
                return *t;
            }
        }
        let t = terms();
        // A beam or a rollout never calls `new_decision`: a cap keeps the store small.
        if store.terms.len() >= SEAT_STORE_CAP {
            store.terms.clear();
        }
        store.terms.insert(key, (store.parts.clone(), t));
        t
    })
}

#[cfg(debug_assertions)]
fn same_terms(a: &SeatTerms, b: &SeatTerms) -> bool {
    a == b
}

#[cfg(not(debug_assertions))]
fn same_terms(_: &SeatTerms, _: &SeatTerms) -> bool {
    true
}

/// 107: `ctx` with each tag's share read from seat `s`'s cards, in play (its
/// `tags`, one a card) and in hand.
pub(crate) fn own_mix(t: &value::Table, w: &aic_core::module::World, g: &GameState, s: usize, tags: &[i64; TAGS.len()], ctx: Ctx) -> Ctx {
    let (counts, n) = own_mix_counts(w, g, s, tags);
    ctx.with_mix(t, &counts, n)
}

/// 107: seat `s`'s cards in play and in hand counted by tag, and how many.
pub(crate) fn own_mix_counts(w: &aic_core::module::World, g: &GameState, s: usize, tags: &[i64; TAGS.len()]) -> ([i64; TAGS.len()], i64) {
    let mut counts = *tags;
    let mut n = in_play(g, s);
    for id in g.pile(&cards::hand(s)) {
        n += 1;
        for t in &cards::def(w, id).tags {
            counts[*t as usize] += 1;
        }
    }
    (counts, n)
}

/// 124 (D47, `team_share` / `plan_team`): each tag's share of the other seats'
/// cards together, from their `own_mix_counts`.
pub(crate) fn team_share(t: &value::Table, others: impl Iterator<Item = ([i64; TAGS.len()], i64)>) -> [f64; TAGS.len()] {
    let (mut counts, mut n) = ([0i64; TAGS.len()], 0i64);
    for (c, k) in others {
        for (sum, m) in counts.iter_mut().zip(c) {
            *sum += m;
        }
        n += k;
    }
    Ctx::at(t, 0.0, 0.0, counts, false).with_mix(t, &counts, n).share
}

/// 107: seat `s`'s cards in play: its tops, the cards under them and its programs.
fn in_play(g: &GameState, s: usize) -> i64 {
    let tops = g.pile(&cards::built(s));
    let mut n = (tops.len() + g.pile(&cards::played(s)).len()) as i64;
    for top in tops {
        n += g.pile(&cards::under(s, top)).len() as i64;
    }
    n
}

struct SeatData {
    s: usize,
    tags: [i64; TAGS.len()],
    /// Tags for `needs`: the cards' and the board's.
    required: [i64; TAGS.len()],
    prices: cards::Prices,
    income: aic_core::content::PerResource,
    ctx: Ctx,
}


/// 102 (D23, D34): the heuristic's prices the keep plan shares (`Eval::plan_rates`), in
/// table credits: an energy and an antimatter held, an adversary project wholly filled,
/// the credits counted in full (`cap`) and an idle credit's share of one beyond them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlanRates {
    pub energy: f64,
    pub antimatter: f64,
    pub project: f64,
    pub cap: f64,
    pub idle: f64,
    /// 124: the heuristic's `insight_cap` (0: none), for `keep_spare`.
    pub insight_cap: f64,
    /// 128: the heuristic's `energy_cap` (0: none) and the share of the table's
    /// energy price an energy beyond it costs (`Ctx::energy_rate`).
    pub energy_cap: f64,
    pub energy_share: f64,
}

impl PlanRates {
    /// The value table's own rates, for an agent with no heuristic (random):
    /// no project charge, no credit cap, no insight or energy cap.
    pub fn table() -> PlanRates {
        let t = value::table();
        PlanRates {
            energy: t.res(Resource::Energy),
            antimatter: t.res(Resource::Antimatter),
            project: 0.0,
            cap: f64::MAX,
            idle: 1.0,
            insight_cap: 0.0,
            energy_cap: 0.0,
            energy_share: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_count_only_as_far_as_insight_keeps_them() {
        let r = crate::agents::tests::rules(0);
        let w = r.world();
        let mut g = r.new_game(1);
        g.round = 3;
        let keep = |tier| cards::keep_need(&w, &g, 0, tier, 0).0;
        let (one, two) = (keep(1), keep(2));
        g.seats[0].resources.insert(Resource::Insight, two + one);
        let picks = vec![(1.0, 2), (3.0, 2), (2.0, 1)];
        let run = |budget: f64| {
            let ev = Eval::new(&r, Weights { packet_budget: budget, ..Weights::default() });
            let mut cards = Vec::new();
            ev.keepable(&w, &g, 0, picks.clone(), &mut cards);
            cards
        };
        assert_eq!(run(1.0), [3.0, 2.0], "the second tier-2 card is past the budget");
        assert_eq!(run(0.0), [1.0, 3.0, 2.0]);
    }

    /// The one read of a hex's tokens (069) says what asking by name says, on
    /// positions from a played game, and adds up to the core's purge count.
    #[test]
    fn hex_tokens_read_as_the_state_does() {
        let r = crate::agents::tests::rules(2);
        let w = r.world();
        let keys = TokenKeys::new();
        let mut g = r.new_game(7);
        let mut legal = Vec::new();
        for step in 0..600 {
            if g.is_over() {
                break;
            }
            let tokens = keys.read(&g);
            let level = alignment::bar_level(&w, &g);
            let mut pieces = 0;
            for (h, t) in tokens.iter().enumerate() {
                assert_eq!(t.adv_ambassador, g.tokens(h, ADV_AMBASSADOR));
                assert_eq!(t.adv_cube, g.tokens(h, ADV_CUBE));
                assert_eq!(t.marker, g.tokens(h, common::MARKER));
                assert_eq!(t.factory, g.tokens(h, FACTORY));
                assert_eq!(t.station, g.tokens(h, STATION));
                assert_eq!(t.harden, g.tokens(h, HARDEN));
                assert_eq!(t.troop, g.tokens(h, TROOP));
                assert_eq!(t.botnet, g.tokens(h, common::BOTNET));
                for s in 0..g.seats.len() {
                    assert_eq!(t.node[s], g.tokens(h, common::node(s)));
                    assert_eq!(t.cube[s], g.tokens(h, common::cube(s)));
                    assert_eq!(t.ambassador[s], g.tokens(h, common::ambassador(s)));
                }
                let city = common::is_city(&w, h) && t.adv_cube > 0;
                pieces += adversary::against_bar(city, t.botnet, t.adv_ambassador, t.troop, level);
            }
            assert_eq!(pieces, adversary::purge_count(&w, &g, level));
            r.legal_actions_into(&g, &mut legal);
            let Some(a) = legal.get(step % legal.len().max(1)) else { break };
            g = r.apply_to(&g, a, None).expect("a legal action applies");
        }
    }

    #[test]
    fn renamed_weight_keeps_its_old_name() {
        let mut w = Weights::default();
        assert!(w.set("opp_rate", 1.5));
        assert_eq!(w.unity_rate_start, 1.5);
        assert!(w.set("unity_rate_start", 0.5));
        assert_eq!(w.unity_rate_start, 0.5);
        assert!(w.set("data_center", 2.5));
        assert_eq!(w.node, 2.5);
        // 086: presence -> node, presence_danger -> node_danger.
        assert!(w.set("presence", 1.25));
        assert_eq!(w.node, 1.25);
        assert!(w.set("presence_danger", 3.0));
        assert_eq!(w.node_danger, 3.0);
        assert!(check_names(["presence", "botnet"]).is_ok());
        assert!(check_names(["node", "botnet"]).is_ok());
        assert!(check_names(["presence", "node"]).is_err());
    }
}

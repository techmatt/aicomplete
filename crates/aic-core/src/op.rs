//! The one effect schema (015's call 2, done in 025): everything content
//! does is an [`Op`], whichever card, threat or board prints it.
//!
//! An op means the same wherever it is printed; what differs is the context
//! it resolves in (`effects::Ctx`):
//!
//! - **who chooses**: the acting seat on a player's card; seat 0 for the team
//!   on threats, Alignment cards and projects;
//! - **who it hits**: the acting seat; the seats in a threat's scope; every seat;
//! - **where**: a threat's scope (its country, or the whole board on the
//!   shared row); the whole board otherwise.
//!
//! Not every op makes sense everywhere, and the compiler no longer says so:
//! [`Op::slots`] does, and `Content::check` enforces it when content loads.
//! Instant ops resolve in `modules/effects/`, the one place an op's meaning
//! is dispatched. Standing ops and the twist-only ops never resolve: they are
//! read by the rule they change, while their card is in force.

use serde::{Deserialize, Serialize};

use crate::vocab::{Resource, Side, Tag, TagOrAny, ThreatSource, TokenKind};

/// The kinds of content list an op can sit in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    /// A player's effects: card, ability, AI-board power.
    Player,
    /// A threat's effects, in the threat's scope.
    Threat,
    /// A Unity or Discord card or an adversary project: the whole team.
    Team,
    /// A building's standing effects or an AI's innate rule, read while in force.
    Standing,
    /// What an adversary card changes about its round, read while it is current.
    Twist,
}

/// Pieces a threat places adversary ambassadors on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Spot {
    /// Player-owned pieces: presence, aligned cities, owned stations and factories.
    PlayerPieces,
    CleanCities,
    Factories,
    Bases,
}

/// A country marker (module `board`), up while its var is above 0: set by
/// threats and cards, counted down in the board's cleanup, so a threat's lasts
/// to the end of the next round.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundFlag {
    /// Cities in the country make no energy.
    Blackout,
    /// Players with presence in the country make no energy in upkeep (021: was compute).
    Sabotage,
    /// Buildings and presence in the country can't be destroyed.
    Shelter,
    /// Sites in the country can't be claimed (027).
    Embargo,
    /// Arrivals don't land in the country in the next build (068).
    Cordon,
}

/// A site type an adversary card's arrivals land on (027).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Site {
    /// Cities marked coastal on the map: next to ocean, or printed `coast`.
    Coastal,
    /// Printed industrial zones, built on or not.
    Industrial,
    /// Research stations, whoever owns them.
    Stations,
    /// Each country's first military base, or every base: `base_sites_per_country`.
    /// A threat's [`Spot::Bases`] is every base in its scope.
    #[serde(rename = "bases")]
    BaseSites,
}

impl RoundFlag {
    pub fn name(self) -> &'static str {
        match self {
            RoundFlag::Blackout => "blackout",
            RoundFlag::Sabotage => "sabotage",
            RoundFlag::Shelter => "shelter",
            RoundFlag::Embargo => "embargo",
            RoundFlag::Cordon => "cordon",
        }
    }
}

/// One thing content does. See the module docs for how context shapes it;
/// [`Op::slots`] says which lists may print it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Op {
    // ---------- instant: player effects ----------
    /// You gain `amount` of `resource`; on a team card, every player does.
    Gain {
        resource: Resource,
        amount: i64,
    },
    /// 1 of `resource` per presence you have on the board.
    GainPerPresence {
        resource: Resource,
    },
    /// Choose another player for the `other_gain` / `transfer` that follow.
    ChoosePlayer,
    /// The chosen player gains `amount` of `resource`.
    OtherGain {
        resource: Resource,
        amount: i64,
    },
    /// Give up to `max` of `resource` to the chosen player.
    Transfer {
        resource: Resource,
        max: i64,
    },
    /// Place `count` presence off your AI board's tracks, paying each step's
    /// cost unless `free` (041).
    PlacePresence {
        count: i64,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        free: bool,
    },
    /// Clear up to `count` corruption markers in your reach, for each one's
    /// reward (041).
    ClearMarkers {
        count: i64,
    },
    /// A hardening token on up to `count` hexes of your presence, in your
    /// reach (041): it absorbs a destruction there.
    ShieldPresence {
        count: i64,
    },
    /// Up to `count` active threats wait: they don't execute this round, and
    /// can be countered next round (041).
    DelayThreat {
        count: i64,
    },
    /// In one hex in your reach, remove `ambassadors` adversary ambassadors or
    /// `troops` troops: one kind, your choice (059; a 0 prints no such choice).
    Damage {
        #[serde(default)]
        ambassadors: i64,
        #[serde(default)]
        troops: i64,
    },
    /// The same in each hex of one country holding adversary pieces.
    DamageCountry {
        #[serde(default)]
        ambassadors: i64,
        #[serde(default)]
        troops: i64,
    },
    /// Remove every adversary ambassador and troop in one hex.
    DestroyAll,
    /// Remove one node within your reach (019: the tier-2 way to remove a node).
    RemoveNode,
    /// Remove every adversary ambassador in one country; sway 1 toward Unity per `unity_per` removed.
    RemoveAmbassadorsCountry {
        unity_per: i64,
    },
    /// Build a factory on a free industrial zone in your reach.
    BuildFactory,
    /// Take the adversary cube off up to `amount` corrupted stations or
    /// factories, one at a time (`one_country`: all in the first one's
    /// country). `cities` (024): corrupted cities too, outright; only a tier-2
    /// ultimate prints it.
    Cleanse {
        amount: i64,
        one_country: bool,
        #[serde(default)]
        cities: bool,
    },
    /// Remove up to `max` adversary ambassadors (0: all) on or next to one
    /// corrupted city in your reach, the ones next to it first (024).
    ClearCity {
        max: i64,
    },
    /// Put `count` hardening tokens on different facilities in reach.
    Harden {
        count: i64,
    },
    /// Move up to `max` of your ambassadors (from your supply or the board)
    /// onto pieces within your reach, one at a time.
    MoveAmbassadors {
        max: i64,
    },
    /// Gain `count` player ambassadors into your supply.
    GainAmbassadors {
        count: i64,
    },
    /// In one country, facilities and presence can't be destroyed this round.
    Shelter,
    /// In one country, no arrivals land in the next build (068: a Security
    /// card does an ambassador's job).
    Cordon,
    /// Play a building from your hand paying no credits; its other costs and
    /// its requirements stand (068: the Builder's expensive power).
    FreeBuilding,
    /// The next `count` adversary cards are shown.
    Peek {
        count: i64,
    },
    /// One threat the player may counter needs `amount` less of each resource;
    /// with `source`, one of that type (070: the typed counters).
    Jam {
        amount: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<ThreatSource>,
    },
    /// Counter one active threat (the chooser picks): on a player's card, one
    /// they may counter.
    CounterThreat,
    /// Remove up to `amount` compute from one visible adversary project
    /// (017; the Security cards' primitive).
    DrainProject {
        amount: i64,
    },
    /// Discard one visible adversary project, with its compute (017).
    DiscardProject,
    /// A one-time permanent gain (021): your printed production of
    /// `resource` rises by `amount` for the rest of the game.
    Production {
        resource: Resource,
        amount: i64,
    },
    /// Add `amount` tokens to one of your active buildings holding `kind` (021).
    AddTokens {
        kind: TokenKind,
        amount: i64,
    },
    /// A hack (021): a node within your reach makes no compute in the next build.
    CutNode,
    /// Movement add-on (021): move one of your ambassadors on the board up to
    /// `range` hexes onto a piece (or not at all).
    MoveOne {
        range: i64,
    },
    /// Spread (021, rare): each of your ambassadors on the board may move 1
    /// hex onto an adjacent piece.
    SpreadAmbassadors,
    /// Consolidate (021): move up to `max` of your ambassadors within your
    /// reach onto one piece in your reach.
    Consolidate {
        max: i64,
    },
    /// Draw `count` tier-1 cards into your hand, no keep paid (023: Science's
    /// card draw).
    Draw {
        count: i64,
    },
    /// Remove up to `amount` compute from every visible adversary project (023).
    DrainAll {
        amount: i64,
    },
    /// "Count as power" (023): 1 of `resource` per `per` tokens of `kind` on
    /// your active buildings (the tokens stay).
    GainPerTokens {
        resource: Resource,
        kind: TokenKind,
        per: i64,
    },
    /// Cash out (023, 059): remove all your `kind` tokens; gain `rate` of
    /// `resource` for each.
    CashOut {
        kind: TokenKind,
        resource: Resource,
        rate: i64,
    },

    // ---------- instant: players, threats, the team, twists ----------
    /// Sway `amount` toward `side` (module `alignment`).
    Sway {
        side: Side,
        amount: i64,
    },

    // ---------- instant: threats and the team ----------
    /// `count` adversary troops, each onto a factory in scope (players
    /// choose), or with no factory onto a city. With `each_country` (team
    /// cards), `count` in each country instead of `count` in all.
    Troops {
        count: i64,
        #[serde(default)]
        each_country: bool,
        /// Each troop also destroys the presence on the hex it lands on (027).
        #[serde(default)]
        destroy: bool,
    },
    /// Every presence on the most threatened hex in scope (the presence hex
    /// with the most adversary ambassadors and troops; ties: the first in
    /// board order) is destroyed, unless absorbed (027).
    DestroyPresence,
    /// Each player in scope destroys `count` of their presence in scope, their
    /// choice, unless absorbed; a last presence is safe in stage I (028).
    EachDestroyPresence { count: i64 },
    /// Up to `amount` Sway leaves the Unity bucket for the Undecided (027).
    PullUnity {
        amount: i64,
    },
    /// Each player in scope gets `amount` less insight next round (027).
    LoseInsight {
        amount: i64,
    },
    /// One research station in scope that is not corrupted becomes corrupted
    /// (players choose). With `each_country` (team cards), one per country.
    FlipStation {
        #[serde(default)]
        each_country: bool,
    },
    /// In each country in scope, the city not corrupted with the most
    /// adversary ambassadors (ties: lower hex number) becomes corrupted.
    CorruptCity,
    /// Each player in scope loses `amount` of `resource` (as much as they have).
    PlayersLose {
        resource: Resource,
        amount: i64,
    },
    /// The adversary builds now, in scope.
    AdversaryBuild,

    // ---------- instant: threats ----------
    /// `count` adversary ambassadors from the supply, each onto a piece of
    /// kind `at` in scope (players choose each).
    Ambassadors {
        at: Spot,
        count: i64,
    },
    /// Each player in scope loses a presence in scope, or pays `amount`
    /// `resource` (their choice). Losing it needs 2 or more presence on the
    /// board, so this never takes a last presence (027).
    LosePresenceOr {
        resource: Resource,
        amount: i64,
    },
    /// A round marker on each country in scope, read by the rule it names.
    Flag {
        name: RoundFlag,
    },
    /// Each player in scope discards `cards` cards or pays `amount`
    /// `resource` (their choice).
    Requisition {
        resource: Resource,
        amount: i64,
        #[serde(default = "one")]
        cards: i64,
    },
    /// Each player in scope loses a player ambassador (from their supply,
    /// else one of theirs on the board).
    LoseAmbassador,
    /// Each player in scope discards their dearest card in hand (their
    /// choice on a tie).
    DiscardMostExpensive,
    /// The adversary gains `amount` compute (onto its projects, 017).
    Compute {
        amount: i64,
    },
    /// The adversary gains `amount` compute on every visible project at once
    /// (074: Cintamani's adversary cubes), counted as compute it made.
    ComputeEvery {
        amount: i64,
    },
    /// Each player in scope loses `amount` tokens from their cards (021;
    /// their choice of card).
    LoseTokens {
        amount: i64,
    },

    // ---------- instant: the team (Alignment cards, projects) ----------
    /// Every player's `resource` income (credits or insight) rises by
    /// `amount` for the rest of the game.
    TeamIncome {
        resource: Resource,
        amount: i64,
    },
    /// The next `count` keeps from the draft are free.
    FreePurchases {
        count: i64,
    },
    /// Counter every active threat.
    CounterAll,
    /// In each country, remove `ambassadors` adversary ambassadors (players choose).
    Pushback {
        ambassadors: i64,
    },
    /// Deal `count` extra threats now, one per country in turn.
    DealThreats {
        count: i64,
    },
    /// In each country, the clean city with the most adversary ambassadors
    /// (ties: lower hex number) gains `amount` more.
    Infiltrate {
        amount: i64,
    },
    /// The adversary spreads now.
    AdversarySpread,
    /// Every player sacrifices one of their ambassadors (supply or board)
    /// or one of their presence tokens (their choice).
    Sacrifice,
    /// Every node makes `amount` adversary ambassadors on its hex.
    NodeAmbassadors {
        amount: i64,
    },
    /// Every clean city gains `amount` adversary ambassadors.
    CleanCityAmbassadors {
        amount: i64,
    },
    /// Remove `count` nodes anywhere on the board, one at a time (players choose).
    RemoveNodes {
        count: i64,
    },

    // ---------- standing: read while in force ----------
    /// `amount` of `resource` a round.
    Income {
        resource: Resource,
        amount: i64,
    },
    /// `1` per `per` of your `tag` tags.
    IncomePerTag {
        resource: Resource,
        tag: Tag,
        per: i64,
    },
    /// Your buildings cost `amount` credits less.
    BuildingDiscount {
        amount: i64,
    },
    /// You may use the Harden standard action, paid at `harden_chips`.
    HardenAction,
    /// `amount` more building slots.
    Slots {
        amount: i64,
    },
    /// Gain this when a threat you paid toward is countered.
    Logistics {
        resource: Resource,
        amount: i64,
    },
    /// In upkeep, `amount` compute comes off each adversary project (070).
    ProjectDrain {
        amount: i64,
    },
    /// Each threat of `source` in your home country needs `amount` less of each
    /// resource (070).
    AutoCounter {
        source: ThreatSource,
        amount: i64,
    },
    /// Extra Sway toward Unity when you remove an adversary troop.
    UnityBonus {
        amount: i64,
    },
    /// When you give insight, the receiver also gains this.
    GiveBonus {
        resource: Resource,
        amount: i64,
    },
    /// The adversary's spread places no ambassador on hexes holding your presence.
    SpreadShield,
    /// Once a round, prevent the first facility or presence destruction in a country
    /// where you have a presence.
    CivicShelter,
    /// Adversary troops can't enter a hex holding your presence (068: Wall).
    /// An AI board's innate only: `board::walled` reads nothing else, and
    /// `Content::check` refuses it on a card or a capstone.
    Wall,
    /// While you have this building, cards discarded from hand that carry
    /// `tag` (or any tag, `"any"`) count as those tags for AI-board powers (021).
    TagsFromHand {
        tag: TagOrAny,
    },
    /// In upkeep, convert `amount` `from` into `gain` `to` (if you have it).
    ConvertUpkeep {
        from: Resource,
        amount: i64,
        to: Resource,
        gain: i64,
    },

    // ---------- twists: read while the adversary card is current ----------
    /// Each corrupted city builds `amount` more.
    CityBuild {
        amount: i64,
    },
    /// Factories next to clean cities build `amount` too.
    CleanFactories {
        amount: i64,
    },
    /// Each corrupted research station builds `amount`.
    StationBuild {
        amount: i64,
    },
    /// Spread reaches `amount` hexes further.
    SpreadReach {
        amount: i64,
    },
    /// The round's arrivals (027): every matching site in every country gains
    /// `adversary.arrivals` adversary ambassadors in the build, whatever the
    /// board holds.
    Arrivals {
        site: Site,
    },
    /// Cities corrupt at `amount` fewer ambassadors.
    CorruptSooner {
        amount: i64,
    },
    /// This round is a war round.
    War,
}

fn one() -> i64 {
    1
}

impl Op {
    /// The lists that may print this op: the home of "what can go where".
    pub fn slots(&self) -> &'static [Slot] {
        use Slot::*;
        match self {
            Op::Sway { .. } => &[Player, Threat, Team, Twist],
            Op::PullUnity { .. } => &[Threat, Team, Twist],
            Op::EachDestroyPresence { .. } => &[Threat, Team, Twist],
            Op::Gain { .. } | Op::CounterThreat => &[Player, Team],
            Op::Troops { .. }
            | Op::FlipStation { .. }
            | Op::CorruptCity
            | Op::PlayersLose { .. }
            | Op::DestroyPresence
            | Op::LoseInsight { .. }
            | Op::AdversaryBuild => &[Threat, Team],
            Op::Ambassadors { .. }
            | Op::Flag { .. }
            | Op::Requisition { .. }
            | Op::LoseAmbassador
            | Op::LosePresenceOr { .. }
            | Op::DiscardMostExpensive
            | Op::LoseTokens { .. } => &[Threat],
            Op::Compute { .. } => &[Threat, Team],
            Op::TeamIncome { .. }
            | Op::FreePurchases { .. }
            | Op::CounterAll
            | Op::Pushback { .. }
            | Op::DealThreats { .. }
            | Op::Infiltrate { .. }
            | Op::AdversarySpread
            | Op::Sacrifice
            | Op::NodeAmbassadors { .. }
            | Op::CleanCityAmbassadors { .. }
            | Op::RemoveNodes { .. }
            | Op::ComputeEvery { .. } => &[Team],
            Op::Income { .. }
            | Op::IncomePerTag { .. }
            | Op::BuildingDiscount { .. }
            | Op::HardenAction
            | Op::Slots { .. }
            | Op::Logistics { .. }
            | Op::ProjectDrain { .. }
            | Op::AutoCounter { .. }
            | Op::UnityBonus { .. }
            | Op::GiveBonus { .. }
            | Op::SpreadShield
            | Op::CivicShelter
            | Op::Wall
            | Op::TagsFromHand { .. }
            | Op::ConvertUpkeep { .. } => &[Standing],
            Op::CityBuild { .. }
            | Op::CleanFactories { .. }
            | Op::StationBuild { .. }
            | Op::SpreadReach { .. }
            | Op::CorruptSooner { .. }
            | Op::Arrivals { .. }
            | Op::War => &[Twist],
            Op::GainPerPresence { .. }
            | Op::ChoosePlayer
            | Op::OtherGain { .. }
            | Op::Transfer { .. }
            | Op::PlacePresence { .. }
            | Op::ClearMarkers { .. }
            | Op::ShieldPresence { .. }
            | Op::DelayThreat { .. }
            | Op::Damage { .. }
            | Op::DamageCountry { .. }
            | Op::DestroyAll
            | Op::RemoveNode
            | Op::RemoveAmbassadorsCountry { .. }
            | Op::BuildFactory
            | Op::Cleanse { .. }
            | Op::ClearCity { .. }
            | Op::Harden { .. }
            | Op::MoveAmbassadors { .. }
            | Op::GainAmbassadors { .. }
            | Op::Shelter
            | Op::Cordon
            | Op::FreeBuilding
            | Op::Peek { .. }
            | Op::Jam { .. }
            | Op::DrainProject { .. }
            | Op::DiscardProject
            | Op::Production { .. }
            | Op::AddTokens { .. }
            | Op::CutNode
            | Op::MoveOne { .. }
            | Op::SpreadAmbassadors
            | Op::Consolidate { .. }
            | Op::Draw { .. }
            | Op::DrainAll { .. }
            | Op::GainPerTokens { .. }
            | Op::CashOut { .. } => &[Player],
        }
    }
}

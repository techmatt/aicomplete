//! The one effect schema (015's call 2, done in 025): everything content
//! does is an [`Op`], whichever card, threat or board prints it.
//!
//! An op means the same wherever it is printed; what differs is the context
//! it resolves in (`effects::Ctx`):
//!
//! - **who chooses**: the acting seat on a player's card; seat 0 for the team
//!   on threats, Alignment cards and projects;
//! - **who it hits**: the acting seat; the seats in a threat's scope; every seat;
//! - **where**: a threat's scope (its country); the whole board otherwise.
//!
//! Not every op makes sense everywhere, and the compiler no longer says so:
//! [`Op::slots`] does, and `Content::check` enforces it when content loads.
//! Instant ops resolve in `modules/effects/`, the one place an op's meaning
//! is dispatched. Standing ops and the twist-only ops never resolve: they are
//! read by the rule they change, while their card is in force.

use serde::{Deserialize, Serialize};

use crate::vocab::{Location, Resource, Side, Tag, TagOrAny, ThreatSource, TokenKind};

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
    /// Player-owned pieces: nodes, aligned cities, owned stations and factories.
    PlayerPieces,
    CleanCities,
    Factories,
    Bases,
    /// The threat's own location token (135).
    Location,
    /// The threat's city token (135: a threat with a base and a city).
    City,
    /// Every city on or next to the threat's tokens (135).
    CitiesNear,
}

/// Where a threat's flag or extra deal reaches (135).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// On or next to the threat's tokens.
    Near,
    /// The threat's country.
    Country,
}

/// Where a threat's troops land (135).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TroopAt {
    /// On the threat's location token.
    Location,
    /// On the factory this threat just built.
    NewFactory,
}

/// A condition on a threat's troops (135).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TroopIf {
    /// Only when the token's zone holds no factory.
    EmptyZone,
}

/// Where an adversary factory goes when not at the threat's token (135).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactoryWhere {
    /// In the country with the most free industrial zones.
    MostFreeZones,
}

/// Which bases make extra corp troops (135).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorpAt {
    /// The threat's base token.
    Location,
    /// Every military base.
    EveryBase,
}

/// A condition on a base making extra corp troops (135).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorpNear {
    /// An adversary troop on or next to the base.
    AdversaryTroop,
}

/// When extra corp troops come (135).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorpWhen {
    Now,
    EachProduction,
}

/// Which front fights twice (135).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrontPick {
    /// The front with the most corp troops (ties: the first).
    MostCorpTroops,
}

/// Who a theft hits (135).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TheftWho {
    /// The player near it with the most credits (ties: the lowest seat).
    RichestNear,
}

/// What a facility destruction hits (135).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestroyAt {
    Factory,
}

/// How long a disabled building stays face down (135).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Until {
    EndOfNextRound,
}

fn is_false(b: &bool) -> bool {
    !*b
}

fn is_zero(n: &i64) -> bool {
    *n == 0
}

/// A country marker (module `board`), up while its var is above 0: set by
/// threats and cards, counted down in the board's cleanup, so a threat's lasts
/// to the end of the next round.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundFlag {
    /// Cities in the country make no energy.
    Blackout,
    /// Players with a node in the country make no energy in upkeep (021: was compute).
    Sabotage,
    /// Buildings and nodes in the country can't be destroyed.
    Shelter,
    /// Sites in the country can't be claimed (027).
    Embargo,
    /// Arrivals don't land in the country in the next build (068).
    Cordon,
    /// Players near it can't use Actions that cost energy (135).
    PowerDown,
    /// The research stations of players near it make no research cubes (135).
    StationOutage,
    /// The next adversary build on or next to it happens twice (135).
    DoubleBuild,
    /// Research stations on or next to it produce nothing (135).
    StationDark,
    /// Cities and stations on or next to it produce nothing (135).
    GridDown,
}

/// 138: the sites a creation line ("create N per board on the lowest-numbered
/// X") lands on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateAt {
    /// Cities, whoever owns them.
    City,
    /// Cities marked coastal on the map: next to ocean, or printed `coast`.
    CoastalCity,
    /// Research stations, whoever owns them.
    Station,
    /// Factories, whoever owns them.
    Factory,
    /// Hexes holding a botnet.
    Botnet,
    /// The threat's own location token (a threat only).
    Location,
}

/// 138: the condition a creation line's site must meet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateIf {
    #[default]
    Any,
    /// Holding no adversary ambassador.
    NoAmbassadors,
    /// Holding at least one adversary ambassador.
    Ambassadors,
    /// Not the adversary's (a clean or aligned city, a neutral or player site).
    Clean,
    /// The adversary's (a corrupted city, an adversary station or factory).
    Adversary,
    /// A player's (an aligned city, a player's station or factory).
    Player,
    /// On or next to the threat's tokens (a threat only).
    Near,
}

impl CreateIf {
    pub fn is_any(&self) -> bool {
        *self == CreateIf::Any
    }
}

/// 138: what a Corrupt X line corrupts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorruptSite {
    City,
    Station,
    Factory,
    /// An empty land hex (no piece, not ocean): it takes the land-corruption token.
    Land,
}

impl CorruptSite {
    pub const ALL: [CorruptSite; 4] = [CorruptSite::City, CorruptSite::Station, CorruptSite::Factory, CorruptSite::Land];
    pub fn name(self) -> &'static str {
        match self {
            CorruptSite::City => "city",
            CorruptSite::Station => "station",
            CorruptSite::Factory => "factory",
            CorruptSite::Land => "land",
        }
    }
}

/// A facility kind a cleanse may be limited to (r35).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Facility {
    Station,
    Factory,
}

/// What a standing trigger (`Op::When`) waits for (105). Each fires as often as
/// it happens: there is no per-round limit on a trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case", deny_unknown_fields)]
pub enum Event {
    /// You play a card carrying `tag` (no tag: any card). A building counts
    /// itself: it is in play when the trigger is read.
    Play {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tag: Option<Tag>,
    },
    /// You play a card: once for each `tag` tag on it (the per-tag form);
    /// with no tag, once for each tag of any kind on it (106: the tier-2 form).
    PlayTag {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tag: Option<Tag>,
    },
    /// Another player plays a card carrying `tag` (no tag: any card); with
    /// `both`, that player gains the same (the trigger's ops must be gains).
    TeamPlay {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tag: Option<Tag>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        both: bool,
    },
    /// You play a card whose printed credit cost is at least `at` (a rebate).
    PlayCost { at: i64 },
    /// You remove an adversary troop: once a troop.
    RemoveTroop,
    /// You remove compute from adversary projects: once a card, Action, power
    /// or standard action that removes any.
    RemoveCompute,
    /// You remove adversary ambassadors (canceling included): once a card,
    /// Action, power or standard action that removes any.
    RemoveAmbassadors,
    /// You place a token of `kind` on one of your cards: once a token.
    PlaceTokens { kind: TokenKind },
    /// Your Wall stops a troop in the march (an AI board's innate: Vigil):
    /// once a troop a march.
    WallStops,
}

impl RoundFlag {
    pub fn name(self) -> &'static str {
        match self {
            RoundFlag::Blackout => "blackout",
            RoundFlag::Sabotage => "sabotage",
            RoundFlag::Shelter => "shelter",
            RoundFlag::Embargo => "embargo",
            RoundFlag::Cordon => "cordon",
            RoundFlag::PowerDown => "power_down",
            RoundFlag::StationOutage => "station_outage",
            RoundFlag::DoubleBuild => "double_build",
            RoundFlag::StationDark => "station_dark",
            RoundFlag::GridDown => "grid_down",
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
    /// 1 of `resource` per node you have on the board.
    GainPerNode {
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
    /// Place `count` nodes off your AI board's tracks, paying each step's
    /// cost unless `free` (041).
    PlaceNode {
        count: i64,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        free: bool,
    },
    /// Clear up to `count` corruption markers in your reach, for each one's
    /// reward (041).
    ClearMarkers {
        count: i64,
    },
    /// A hardening token on up to `count` hexes of your nodes, in your
    /// reach (041): it absorbs a destruction there.
    ShieldNode {
        count: i64,
    },
    /// Up to `count` active threats the seat may counter wait (118): they
    /// don't execute this round, and can be countered next round (041).
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
    /// Remove one botnet within your reach (019: the tier-2 way to remove a botnet).
    RemoveBotnet,
    /// Remove up to `max` adversary ambassadors in one country (0: every one);
    /// sway 1 toward Unity per `unity_per` removed (0: none). 076: a number, not "every".
    /// `reach` (111, Matt's ruling 3): in reach instead, any hexes within reach
    /// of your nodes, across countries.
    RemoveAmbassadorsCountry {
        #[serde(default)]
        unity_per: i64,
        #[serde(default)]
        max: i64,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        reach: bool,
    },
    /// Remove up to `count` adversary troops in one country, any country
    /// (076, Matt's ruling 6: a printed number, never "per hex"); with `reach`
    /// (111), in reach instead.
    RemoveTroops {
        count: i64,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        reach: bool,
    },
    /// Build a factory on a free industrial zone in your reach.
    BuildFactory,
    /// Take the adversary cube off up to `amount` corrupted stations or
    /// factories, one at a time (`one_country`: all in the first one's
    /// country). `cities` (024): corrupted cities too, each liberated as `Liberate`
    /// does (`board::liberate`); only a tier-2 ultimate prints it. `only` (r35):
    /// stations alone or factories alone, priced apart (a station is a flip
    /// worth several cubes both ways, a factory stays cheap).
    Cleanse {
        amount: i64,
        one_country: bool,
        #[serde(default)]
        cities: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        only: Option<Facility>,
    },
    /// Liberate one corrupted city in your reach (080, r34: the liberate
    /// capstones): its adversary cube comes off, so it is clean; with `align`
    /// your cube goes on it. Each engine's twist (r35): `ambassadors`, you gain
    /// that many; `troops`, up to that many adversary troops in the city's
    /// country come off; `botnet`, a botnet on or next to the city comes off.
    Liberate {
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        align: bool,
        #[serde(default, skip_serializing_if = "is_zero")]
        ambassadors: i64,
        #[serde(default, skip_serializing_if = "is_zero")]
        troops: i64,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        botnet: bool,
    },
    /// Remove up to `max` adversary ambassadors (0: all) on or next to one
    /// corrupted city in your reach (024), your choice which while more stand
    /// than it takes (059).
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
    /// In one country, facilities and nodes can't be destroyed this round.
    Shelter,
    /// In one country, no arrivals land through the next round, its build and any
    /// build a project brings (118; 068: a Security card does an ambassador's job).
    Cordon,
    /// Play a building from your hand paying no credits; its other costs and
    /// its requirements stand (068: the Builder's expensive power).
    FreeBuilding,
    /// Build a research station in your reach, paying no chips (075).
    BuildStation,
    /// Claim a neutral research station in your reach with no adversary ambassador on it, paying no
    /// ambassadors (142: the card's energy is the price).
    ClaimStation,
    /// `count` research cubes in your color onto one open megaproject
    /// (075): with `station`, in a country where you own a station. They pay at
    /// completion like any cube.
    ProjectCube {
        count: i64,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        station: bool,
    },
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
    /// A one-time permanent gain per tag (106, a program's): your printed
    /// production of `resource` rises by 1 per `per` `tag` tags on your cards
    /// in play, counted once when it resolves (a program counts itself).
    ProductionPerTag {
        resource: Resource,
        tag: Tag,
        per: i64,
    },
    /// Add `amount` tokens to one of your active buildings holding `kind` (021);
    /// with `here`, to the building whose Action this is (076).
    AddTokens {
        kind: TokenKind,
        amount: i64,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        here: bool,
    },
    /// A hack (021): a botnet within your reach makes no compute in the next build.
    CutBotnet,
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
    /// "Count as power" (023): 1 of `resource` per `per` tokens of `kind`
    /// (the tokens stay). 076 (Matt's ruling 12, no recounting across cards):
    /// only the tokens on one card, this building's with `here`, else one of
    /// yours the player picks.
    GainPerTokens {
        resource: Resource,
        kind: TokenKind,
        per: i64,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        here: bool,
    },
    /// Cash out (023, 059): remove every `kind` token from one card (076: one
    /// card, never all yours; this building's with `here`, else one of yours the
    /// player picks); gain `rate` of `resource` for each.
    CashOut {
        kind: TokenKind,
        resource: Resource,
        rate: i64,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        here: bool,
    },
    /// One-off (076, Matt's ruling 11): 1 of `resource` per `per` `tag` tags on
    /// your cards in play.
    GainPerTag {
        resource: Resource,
        tag: Tag,
        per: i64,
    },
    /// Copy (105, a program's): raise your income by one of your active
    /// buildings' Each round output, as it pays at the time of play (a
    /// per-tag income at its count then).
    CopyIncome,
    /// Reuse (105, a building's Action): use another of your buildings'
    /// Actions again this round. Not one whose own Action is a reuse.
    Reuse,
    /// Discard a card from your hand and gain `amount` of `resource` (105: the
    /// Researcher's Synthesis; one card a use, which keeps the agents' search
    /// to one pick).
    DiscardHand {
        resource: Resource,
        amount: i64,
    },

    // ---------- instant: players, threats, the team, twists ----------
    /// Sway `amount` toward `side` (module `alignment`).
    Sway {
        side: Side,
        amount: i64,
    },

    // ---------- instant: threats and the team ----------
    /// `count` adversary troops onto factories in scope (players choose), else cities; `each_country`: in each;
    /// `destroy`: each takes the node there; `at`, `location`: elsewhere; `stand`: they never march.
    Troops {
        count: i64,
        #[serde(default)]
        each_country: bool,
        /// Each troop also destroys the node on the hex it lands on (027).
        #[serde(default)]
        destroy: bool,
        /// Where they land instead of a factory (135).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        at: Option<TroopAt>,
        /// They stand: never march or strike, until removed (135).
        #[serde(default, skip_serializing_if = "is_false")]
        stand: bool,
        /// Which of the threat's tokens, when it has two (135).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location: Option<Location>,
        /// Only when this holds (135).
        #[serde(default, rename = "if", skip_serializing_if = "Option::is_none")]
        if_: Option<TroopIf>,
    },
    /// 139 (Offensive): a `tier` threat is dealt, any tag, by the draw rule, into the country of the most
    /// threatened node hex in scope (the node hex with the most adversary ambassadors and troops; ties: the first
    /// in board order), its first location token on that hex; it is countered or executes like any other.
    ThreatAtNode { tier: u8 },
    /// Each player in scope destroys `count` of their nodes in scope (their choice), unless absorbed;
    /// never a last node from a threat, nor in stage I. 139: a threat's or a twist's only (Border War); the team
    /// slot (Discord cards, projects) destroys no nodes. `bases` (139, Border War): only nodes on or next to a
    /// front's military base, so a player with none there loses none.
    EachDestroyNode {
        count: i64,
        #[serde(default, skip_serializing_if = "is_false")]
        bases: bool,
    },
    /// Up to `amount` Sway comes off the Unity track (027; 136: no Undecided).
    PullUnity {
        amount: i64,
    },
    /// Each player in scope gets `amount` less insight next round (027).
    LoseInsight {
        amount: i64,
    },
    /// A research station in scope is corrupted (a player's holds unless `adversary.station_flip_at` adversary
    /// and no player ambassadors are on it); `each_country`: one a country; `force`: a player's too.
    FlipStation {
        #[serde(default)]
        each_country: bool,
        /// Even a player's station flips (135).
        #[serde(default, skip_serializing_if = "is_false")]
        force: bool,
    },
    /// In each country in scope the uncorrupted city with the most adversary ambassadors is corrupted (`at`: the
    /// token's; `at: location` also destroys the nodes on and next to it).
    CorruptCity {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        at: Option<Spot>,
    },
    /// Each player in scope loses `amount` of `resource`, as much as they have (`all`: all of it).
    PlayersLose {
        resource: Resource,
        #[serde(default)]
        amount: i64,
        #[serde(default, skip_serializing_if = "is_false")]
        all: bool,
    },
    /// The adversary builds now, in scope.
    AdversaryBuild,

    // ---------- instant: threats ----------
    /// Each player in scope loses `count` nodes in scope or pays `amount` `resource` (their choice); never a last
    /// node.
    LoseNodeOr {
        resource: Resource,
        amount: i64,
        /// Nodes lost instead of paying (135; default 1).
        #[serde(default = "one", skip_serializing_if = "is_one")]
        count: i64,
    },
    /// A round marker on each country in scope, read by the rule it names (`scope` near: the hexes or players
    /// near the threat; `rounds`; `everywhere`).
    Flag {
        name: RoundFlag,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<Scope>,
        #[serde(default = "one", skip_serializing_if = "is_one")]
        rounds: i64,
        #[serde(default, skip_serializing_if = "is_false")]
        everywhere: bool,
    },
    /// Each player in scope discards `cards` cards or pays `amount` `resource` (their choice; `next_round`: off
    /// next round's).
    Requisition {
        resource: Resource,
        amount: i64,
        #[serde(default = "one")]
        cards: i64,
        /// The payment comes off next round's `resource` (insight) instead (135).
        #[serde(default, skip_serializing_if = "is_false")]
        next_round: bool,
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
    /// The adversary builds `count` factories by the token (`where`: in the country with most free zones); none
    /// free: `fallback_ambassadors` on a factory.
    AdversaryFactory {
        #[serde(default = "one", skip_serializing_if = "is_one")]
        count: i64,
        #[serde(default, rename = "where", skip_serializing_if = "Option::is_none")]
        where_: Option<FactoryWhere>,
        #[serde(default, skip_serializing_if = "is_zero")]
        fallback_ambassadors: i64,
    },
    /// The adversary gains `amount` compute per player near it with no `resource`.
    ComputePerMissing {
        resource: Resource,
        amount: i64,
    },
    /// Bases make `count` extra corp troops, `now` or each production (`rounds`, `permanent`), `past_cap`
    /// (`fill`: to the cap; `near`: by a troop).
    CorpTroops {
        #[serde(default, skip_serializing_if = "is_zero")]
        count: i64,
        at: CorpAt,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        near: Option<CorpNear>,
        when: CorpWhen,
        #[serde(default, skip_serializing_if = "is_false")]
        past_cap: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        permanent: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        fill: bool,
        #[serde(default, skip_serializing_if = "is_zero")]
        rounds: i64,
    },
    /// The factory under the token is destroyed with the nodes on its hex (never a last node).
    DestroyFacility {
        at: DestroyAt,
    },
    /// Each player near it turns `count` buildings face down until `until` (tags still count).
    DisableBuilding {
        count: i64,
        until: Until,
    },
    /// Each player near it discards `count` cards (their choice).
    Discard {
        count: i64,
    },
    /// The fronts next to the threat's base fight twice next round; `ongoing_use`: `front` fights twice every war
    /// step.
    ExtraWar {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        front: Option<FrontPick>,
        #[serde(default, skip_serializing_if = "is_false")]
        ongoing_use: bool,
    },
    /// Each player near it pays `amount` `resource` or loses `count` ambassadors.
    LoseAmbassadorsOr {
        count: i64,
        resource: Resource,
        amount: i64,
    },
    /// Each player in scope gets `amount` less `resource` income next round.
    LoseIncome {
        resource: Resource,
        amount: i64,
    },
    /// Each player in scope takes one of `options` (their choice).
    LoseOr {
        options: Vec<Op>,
    },
    /// The troops this threat landed march now, `steps` steps (default the march's).
    MarchNow {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        steps: Option<i64>,
    },
    /// A troop occupies the factory under the token (it makes nothing while the troop stands); on a free zone,
    /// `else_compute` compute.
    OccupyFactory {
        else_compute: i64,
    },
    /// `count` botnets on the token's station, else the nearest piece without one.
    PlaceBotnet {
        #[serde(default = "one", skip_serializing_if = "is_one")]
        count: i64,
    },
    /// The adversary seizes the factory under the token, even a player's; a free zone gets a new one.
    SeizeFactory,
    /// The player near it with the most credits loses `amount` `resource`; the adversary gains `compute`.
    Theft {
        who: TheftWho,
        resource: Resource,
        amount: i64,
        #[serde(default)]
        compute: i64,
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
    /// Deal extra threats now; they execute next round. No field set: `count` in all, countries in
    /// turn, at the tier of the card's first pull. Any field set: `count` per country (the scope's
    /// with `scope` country, else every one), at `tier` or the round's threat tier, any tag.
    DealThreats {
        count: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tier: Option<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<Scope>,
        #[serde(default, skip_serializing_if = "is_false")]
        each_country: bool,
    },
    /// 138, the one creation shape: "create `count` per board on the
    /// lowest-numbered `at` [`if`]": in each country in scope, the `sites`
    /// lowest-numbered matching sites each gain `count` adversary ambassadors.
    Create {
        count: i64,
        at: CreateAt,
        #[serde(default, rename = "if", skip_serializing_if = "CreateIf::is_any")]
        if_: CreateIf,
        #[serde(default = "one", skip_serializing_if = "is_one")]
        sites: i64,
    },
    /// 138, site creation: in each country in scope, a neutral `kind` on its
    /// lowest-numbered empty station site (a station) or free industrial zone
    /// (a factory).
    CreateSite {
        kind: Facility,
    },
    /// 138, Corrupt X: in each country in scope, every `site` that can gather
    /// its corrupt number (`adversary.corrupt_*`, less `cut`) of adversary
    /// ambassadors with one move each (on it and next to it, in its country)
    /// pulls them in, lowest-numbered hexes first, and corrupts.
    Corrupt {
        site: CorruptSite,
        #[serde(default, skip_serializing_if = "is_zero")]
        cut: i64,
    },
    /// Remove `count` botnets anywhere on the board, one at a time (players choose).
    RemoveBotnets {
        count: i64,
    },
    /// The board's difficulty line: each standing line landing troops lands `amount` more a country.
    StandingTroopsBonus {
        amount: i64,
    },

    // ---------- standing: read while in force ----------
    /// `amount` of `resource` a round.
    Income {
        resource: Resource,
        amount: i64,
    },
    /// `amount` (default 1; 106) per `per` of your `tag` tags.
    IncomePerTag {
        resource: Resource,
        tag: Tag,
        per: i64,
        #[serde(default = "one", skip_serializing_if = "is_one")]
        amount: i64,
    },
    /// Your buildings cost `amount` credits less.
    BuildingDiscount {
        amount: i64,
    },
    /// Your cards carrying `tag` (no tag: every card) cost `amount` less of
    /// `resource`, credits or insight, to play (105), down to `cards.discount_floor`.
    CardDiscount {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tag: Option<Tag>,
        resource: Resource,
        amount: i64,
    },
    /// A trigger (105): when `on` happens, resolve `then` (instant player ops;
    /// "here" is this card). On a card, an Effect: `:`.
    When {
        on: Event,
        then: Vec<Op>,
    },
    /// A threshold counter (105): whenever its tokens here reach `at`,
    /// whatever put them there (a feeder card's too, 118), remove `at` of
    /// them and resolve `then`.
    Threshold {
        at: i64,
        then: Vec<Op>,
    },
    /// You may use the Harden standard action, paid `chips` chips (080: the
    /// price is printed on the card; r33 read `board.harden_chips`).
    HardenAction {
        chips: i64,
    },
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
    /// Each threat of `source` you can pay toward (you may counter it) needs
    /// `amount` less of each resource (070; r48: any country, or the row).
    AutoCounter {
        source: ThreatSource,
        amount: i64,
    },
    /// Extra Sway toward Unity when you remove an adversary troop.
    UnityBonus {
        amount: i64,
    },
    /// 138: corruption never destroys your nodes (Sanctuary; was the spread shield).
    CorruptionShield,
    /// Once a round, prevent the first facility or node destruction in a country
    /// where you have a node.
    CivicShelter,
    /// Adversary troops can't enter or strike a hex holding your nodes
    /// (068: Wall; 075: strikes). An AI board's innate only: `board::walled`
    /// reads nothing else, and `Content::check` refuses it on a card or a capstone.
    Wall,
    /// Your own unspent insight carries over to the next round, not insight given you this round (075: Patience). An
    /// AI board's innate only: `economy` reads nothing else.
    Patience,
    /// Each round in upkeep, `amount` insight from the supply goes to other
    /// players for the next round, split as you choose (075).
    SupplyInsight {
        amount: i64,
    },
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
    /// Every adversary troop takes `amount` more steps this round.
    TroopSteps {
        amount: i64,
    },
}

fn one() -> i64 {
    1
}

fn is_one(n: &i64) -> bool {
    *n == 1
}

impl Op {
    /// The lists that may print this op: the home of "what can go where".
    pub fn slots(&self) -> &'static [Slot] {
        use Slot::*;
        match self {
            Op::Sway { .. } => &[Player, Threat, Team, Twist],
            Op::PullUnity { .. } => &[Threat, Team, Twist],
            Op::EachDestroyNode { .. } => &[Threat, Twist],
            Op::Gain { .. } | Op::CounterThreat => &[Player, Team],
            Op::Troops { .. }
            | Op::FlipStation { .. }
            | Op::CorruptCity { .. }
            | Op::PlayersLose { .. }
            | Op::LoseInsight { .. }
            | Op::AdversaryBuild
            | Op::AdversaryFactory { .. }
            | Op::CorpTroops { .. }
            | Op::ExtraWar { .. }
            | Op::LoseIncome { .. }
            | Op::Flag { .. }
            | Op::LoseAmbassador
            | Op::LoseTokens { .. }
            | Op::DealThreats { .. } => &[Threat, Team],
            Op::Requisition { .. }
            | Op::DiscardMostExpensive
            | Op::LoseNodeOr { .. }
            | Op::ComputePerMissing { .. }
            | Op::DestroyFacility { .. }
            | Op::DisableBuilding { .. }
            | Op::Discard { .. }
            | Op::LoseAmbassadorsOr { .. }
            | Op::LoseOr { .. }
            | Op::MarchNow { .. }
            | Op::OccupyFactory { .. }
            | Op::PlaceBotnet { .. }
            | Op::SeizeFactory
            | Op::Theft { .. } => &[Threat],
            Op::Compute { .. } => &[Threat, Team],
            Op::TeamIncome { .. }
            | Op::FreePurchases { .. }
            | Op::CounterAll
            | Op::Pushback { .. }
            | Op::ThreatAtNode { .. }
            | Op::RemoveBotnets { .. }
            | Op::StandingTroopsBonus { .. } => &[Team],
            Op::ComputeEvery { .. } => &[Threat, Team],
            Op::Create { .. } => &[Threat, Team],
            Op::Corrupt { .. } => &[Threat, Team, Twist],
            Op::CreateSite { .. } => &[Player, Threat, Team],
            Op::Income { .. }
            | Op::IncomePerTag { .. }
            | Op::BuildingDiscount { .. }
            | Op::CardDiscount { .. }
            | Op::When { .. }
            | Op::Threshold { .. }
            | Op::HardenAction { .. }
            | Op::Slots { .. }
            | Op::Logistics { .. }
            | Op::ProjectDrain { .. }
            | Op::AutoCounter { .. }
            | Op::UnityBonus { .. }
            | Op::CorruptionShield
            | Op::CivicShelter
            | Op::Wall
            | Op::Patience
            | Op::SupplyInsight { .. }
            | Op::TagsFromHand { .. }
            | Op::ConvertUpkeep { .. } => &[Standing],
            Op::TroopSteps { .. } => &[Twist],
            Op::GainPerNode { .. }
            | Op::ChoosePlayer
            | Op::OtherGain { .. }
            | Op::Transfer { .. }
            | Op::PlaceNode { .. }
            | Op::ClearMarkers { .. }
            | Op::ShieldNode { .. }
            | Op::DelayThreat { .. }
            | Op::Damage { .. }
            | Op::DamageCountry { .. }
            | Op::DestroyAll
            | Op::RemoveBotnet
            | Op::RemoveAmbassadorsCountry { .. }
            | Op::RemoveTroops { .. }
            | Op::GainPerTag { .. }
            | Op::BuildFactory
            | Op::Cleanse { .. }
            | Op::Liberate { .. }
            | Op::ClearCity { .. }
            | Op::Harden { .. }
            | Op::MoveAmbassadors { .. }
            | Op::GainAmbassadors { .. }
            | Op::Shelter
            | Op::Cordon
            | Op::FreeBuilding
            | Op::BuildStation
            | Op::ClaimStation
            | Op::ProjectCube { .. }
            | Op::Peek { .. }
            | Op::Jam { .. }
            | Op::DrainProject { .. }
            | Op::DiscardProject
            | Op::Production { .. }
            | Op::ProductionPerTag { .. }
            | Op::AddTokens { .. }
            | Op::CutBotnet
            | Op::MoveOne { .. }
            | Op::SpreadAmbassadors
            | Op::Consolidate { .. }
            | Op::Draw { .. }
            | Op::DrainAll { .. }
            | Op::GainPerTokens { .. }
            | Op::CashOut { .. }
            | Op::CopyIncome
            | Op::Reuse
            | Op::DiscardHand { .. } => &[Player],
        }
    }
}

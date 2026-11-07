//! Game content as data, embedded from `content/` at compile time (no I/O
//! at runtime): the map (piece shape, layouts, the printed country pieces),
//! player cards, threats, the adversary deck and its project deck, the AI
//! boards and the Alignment decks.
//!
//! What content does is one schema, [`Op`] (`op.rs`). A new op is a code
//! change; a new card, threat, adversary card, AI board or Alignment card
//! that only combines existing ops is a data change.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};

use serde::{Deserialize, Serialize};

use crate::hash::fnv1a64;
use crate::hex::{Dir, Hex};
use crate::key::FastMap;
pub use crate::op::{Event, Facility, Op, RoundFlag, Slot, Spot};
pub use crate::vocab::{
    Location, PerResource, PullTag, Resource, Side, Tag, TagOrAny, ThreatSource, ThreatTag, TokenKind,
    RESOURCES, TAGS,
};

const MAP_JSON: &str = include_str!("../../../content/map.json");
const CARDS_JSON: &str = include_str!("../../../content/cards.json");
const THREATS_JSON: &str = include_str!("../../../content/threats.json");
const ADVERSARY_JSON: &str = include_str!("../../../content/adversary.json");
const AIS_JSON: &str = include_str!("../../../content/ais.json");
const ALIGNMENT_JSON: &str = include_str!("../../../content/alignment.json");
const PROJECTS_JSON: &str = include_str!("../../../content/projects.json");

/// Every content file, in hashing order.
const FILES: [&str; 7] = [
    MAP_JSON,
    CARDS_JSON,
    THREATS_JSON,
    ADVERSARY_JSON,
    AIS_JSON,
    ALIGNMENT_JSON,
    PROJECTS_JSON,
];

/// Built only by [`Content::parse`], which fills the indexes: it serializes
/// for clients but is never read back whole.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Content {
    pub map: MapContent,
    pub cards: Vec<CardDef>,
    pub threats: Vec<ThreatDef>,
    /// The adversary's event deck (135).
    pub adversary: Vec<AdversaryCard>,
    /// The adversary's board (135).
    pub adversary_board: AdversaryBoard,
    pub ais: Vec<AiBoard>,
    /// The Unity and Discord decks (module `alignment`).
    pub alignment: Vec<AlignmentCard>,
    /// The adversary's project deck (module `adversary`, 017).
    pub projects: Vec<ProjectCard>,
    /// Id -> index into each list, for [`Content::card_index`] and the like
    /// (rules and agents look things up by id constantly). Built by `parse`.
    #[serde(skip)]
    card_ix: FastMap<usize>,
    /// A card id's index by where its interned [`Key`](crate::key::Key) lives
    /// (address and length): piles hold keys, and hashing the address beats
    /// hashing and comparing the text (103). Any other `&str` misses and falls
    /// to `card_ix`.
    #[serde(skip)]
    card_at: AddressMap,
    #[serde(skip)]
    threat_ix: FastMap<usize>,
    #[serde(skip)]
    project_ix: FastMap<usize>,
    #[serde(skip)]
    alignment_ix: FastMap<usize>,
    #[serde(skip)]
    adversary_ix: FastMap<usize>,
}

/// Each id's first place in a list.
fn index_of<T>(list: &[T], id: impl Fn(&T) -> &str) -> FastMap<usize> {
    list.iter().enumerate().rev().map(|(i, x)| (id(x).to_string(), i)).collect()
}

/// Index by (address, length), open addressing with a multiply for a hash
/// (141: `card_index` is called for every card a tag count, a price or a
/// heuristic term reads, and a `HashMap` with a byte hash was a sixth of a
/// long game's time). A slot's address 0 is empty: no `&str` lives there.
#[derive(Debug, Clone, PartialEq, Default)]
struct AddressMap {
    slots: Vec<(usize, usize, usize)>,
    shift: u32,
}

impl AddressMap {
    #[inline]
    fn slot(&self, ptr: usize) -> usize {
        (ptr as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15).wrapping_shr(self.shift) as usize
    }

    /// Room for `n` at under a quarter full.
    fn with_room(n: usize) -> Self {
        let bits = (n.max(1) * 4).next_power_of_two().trailing_zeros().max(1);
        AddressMap { slots: vec![(0, 0, 0); 1 << bits], shift: 64 - bits }
    }

    fn insert(&mut self, (ptr, len): (usize, usize), i: usize) {
        let mask = self.slots.len() - 1;
        let mut k = self.slot(ptr);
        while self.slots[k].0 != 0 && (self.slots[k].0, self.slots[k].1) != (ptr, len) {
            k = (k + 1) & mask;
        }
        self.slots[k] = (ptr, len, i);
    }

    #[inline]
    fn get(&self, &(ptr, len): &(usize, usize)) -> Option<&usize> {
        if self.slots.is_empty() {
            return None;
        }
        let mask = self.slots.len() - 1;
        let mut k = self.slot(ptr);
        loop {
            let s = &self.slots[k];
            if s.0 == ptr && s.1 == len {
                return Some(&s.2);
            }
            if s.0 == 0 {
                return None;
            }
            k = (k + 1) & mask;
        }
    }
}

/// [`index_of`] by each id's interned key's address and length: the first of
/// a repeated id wins, as in `index_of`.
fn index_at<T>(list: &[T], id: impl Fn(&T) -> &str) -> AddressMap {
    let at = |s: &'static str| (s.as_ptr() as usize, s.len());
    let mut m = AddressMap::with_room(list.len());
    for (i, x) in list.iter().enumerate().rev() {
        m.insert(at(crate::key::Key::new(id(x)).as_str()), i);
    }
    m
}

// ---------- map ----------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapContent {
    pub piece: PieceDef,
    pub layouts: Vec<LayoutDef>,
    /// The printed country pieces: country `i` of every layout uses piece `i`.
    pub countries: Vec<CountryPiece>,
}

/// The country piece: a `size` x `size` rhombus of hexes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PieceDef {
    pub size: u8,
}

/// Where the pieces go for a given number of countries. Piece `i` is country `i`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutDef {
    pub countries: u8,
    pub pieces: Vec<PiecePlacement>,
    /// The fronts, printed on the tiles: pairs of countries sharing a long
    /// edge. Pieces touching only at the middle of the map are not fronts.
    #[serde(default)]
    pub fronts: Vec<[u8; 2]>,
}

/// A placed piece covers `anchor + i*axes[0] + j*axes[1]` for `i, j` in
/// `0..size`. The two axes must be 60 or 120 degrees apart.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PiecePlacement {
    pub anchor: Hex,
    pub axes: [Dir; 2],
    /// Display color from the reference images; the core never reads it.
    #[serde(default)]
    pub color: String,
}

/// What a hex is. Cities, open land and ocean are printed on the piece; a
/// military base goes on an open hex of each land border (see [`crate::Map`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HexKind {
    City,
    Base,
    Open,
    Ocean,
}

impl HexKind {
    pub fn name(self) -> &'static str {
        match self {
            HexKind::City => "city",
            HexKind::Base => "base",
            HexKind::Open => "open",
            HexKind::Ocean => "ocean",
        }
    }
}

/// A printed country piece. Hexes are numbered 1-16 in piece order
/// (`i * size + j + 1` for piece position `(i, j)`); the numbers are
/// printed and break ties. The lists are hex numbers: printed cities and
/// ocean, and where the setup pieces go. Every other hex is open land.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CountryPiece {
    /// The corp's name: country = corp, one piece (074).
    pub name: String,
    /// What the corp does, in a few words.
    pub flavor: String,
    pub cities: Vec<u8>,
    /// The order cities start corrupted in (an adversary cube): setup
    /// corrupts the first `board.start_corrupted`, difficulty 4
    /// `adversary.extra_corrupted` more.
    pub corrupted: Vec<u8>,
    pub ocean: Vec<u8>,
    /// Cities printed coastal though no ocean touches them (a river or lake:
    /// a piece without ocean would get no coastal arrivals, 027).
    #[serde(default)]
    pub coast: Vec<u8>,
    /// Research stations at setup (they start corrupted).
    pub stations: Vec<u8>,
    /// The country's neutral research station at setup (074): open land, no
    /// industrial zone, no base.
    pub neutral_station: u8,
    /// Printed industrial zones: open hexes next to a city where factories go.
    pub industry: Vec<u8>,
    /// 137: printed empty station sites, 1-2 a country (open land, no setup
    /// piece, industrial zone or base), where content may create a neutral station.
    #[serde(default)]
    pub station_sites: Vec<u8>,
    /// Industrial zones holding a factory at setup (neutral).
    pub factories: Vec<u8>,
    /// Where the starting node of the player who starts here goes.
    pub node: u8,
    /// The corp's megaproject card.
    #[serde(rename = "project")]
    pub megaproject: Megaproject,
}

/// A megaproject card (074), the corp's defining card: research stations fill
/// it with cubes; at the end of production, if it holds `cubes` or more, every
/// player cube pays `player` to its owner and every adversary cube `adversary`.
/// After that a seat with a node in a city of the country has the `converter`
/// (r37; 075: any node there; 074: an aligned city). An adversary cube made after completion is lost, as a
/// neutral one is (075).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Megaproject {
    pub name: String,
    pub cubes: i64,
    #[serde(default)]
    pub text: String,
    pub player: Vec<Op>,
    pub adversary: Vec<Op>,
    pub converter: Converter,
}

/// What a completed megaproject prints for players (074): pay `cost` for
/// `effects`, as often as you can pay (105), or once a round with `once`
/// (Onyx's, whose chips build the stations that make its cubes).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Converter {
    pub cost: Vec<Cost>,
    pub effects: Vec<Op>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub once: bool,
}

impl Converter {
    /// In the card's words: "1 research cube + 2 energy → 1 antimatter".
    pub fn words(&self) -> String {
        let cost: Vec<String> = self.cost.iter().map(|(r, n)| r.words(*n)).collect();
        let once = if self.once { " (once a round)" } else { "" };
        format!("{} → {}{once}", cost.join(" + "), Megaproject::words(&self.effects))
    }
}

impl Megaproject {
    /// What a list of its ops does, in the card's words.
    pub fn words(ops: &[Op]) -> String {
        let word = |op: &Op| match op {
            Op::Gain { resource, amount } => resource.words(*amount),
            Op::Draw { count: 1 } => "1 drawn card".to_string(),
            Op::Sway { side: Side::Unity, amount } => format!("{amount} Sway to Unity"),
            Op::Sway { side: Side::Discord, amount } => format!("{amount} Sway to Discord"),
            Op::Compute { amount } => format!("{amount} adversary compute"),
            Op::ComputeEvery { amount } => format!("{amount} adversary compute on every visible project"),
            other => format!("{other:?}"),
        };
        ops.iter().map(word).collect::<Vec<_>>().join(", ")
    }
}

impl CountryPiece {
    /// The printed kind of hex number `n` (bases are placed, not printed).
    pub fn kind_at(&self, n: u8) -> HexKind {
        if self.cities.contains(&n) {
            HexKind::City
        } else if self.ocean.contains(&n) {
            HexKind::Ocean
        } else {
            HexKind::Open
        }
    }
}

// ---------- player cards ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardKind {
    /// Goes into a building slot; building over one splays it down (021).
    Building,
    /// Resolves and stays in your play area: its tags count (021).
    Program,
    /// Resolves and is discarded: its tags don't stay (021).
    Event,
}

/// A resource and an amount, written `["energy", 2]`.
pub type Cost = (Resource, i64);

/// A tag and a count, written `["power", 2]`.
pub type TagCount = (Tag, i64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardDef {
    pub id: String,
    pub name: String,
    /// 1 or 2: the draft deck it is in.
    pub tier: u8,
    pub kind: CardKind,
    /// Credits.
    pub cost: i64,
    #[serde(default)]
    pub antimatter: i64,
    /// Energy to play it (Power cards, 021).
    #[serde(default)]
    pub energy: i64,
    /// Chips a building needs (021: a requirement in the cost).
    #[serde(default)]
    pub chips: i64,
    /// Research cubes to play it (080: the research-cube engine's capstone).
    #[serde(default)]
    pub research_cubes: i64,
    /// Insight to play it (022: 0 for small programs and events, 1 by
    /// default, 2 for big effects; shared with keeps).
    #[serde(default)]
    pub insight: i64,
    /// Income it lowers for the rest of the game when played (022: a cost,
    /// as in Terraforming Mars): your production of the resource falls by
    /// the amount. Unplayable if that income is lower than the amount.
    #[serde(default)]
    pub lowers: Vec<Cost>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Keywords (021): `hack`, `token`, ... Not printed: faces come from ops (033).
    #[serde(default)]
    pub keywords: Vec<String>,
    /// A building that must be built over one of yours whose top card
    /// carries this tag (021: tier-2 replacement).
    #[serde(default)]
    pub replaces: Option<Tag>,
    /// The token type this card holds (021: token engines).
    #[serde(default)]
    pub tokens: Option<TokenSpec>,
    /// Tags you must have (on your cards in play) to play it.
    #[serde(default)]
    pub needs: Vec<TagCount>,
    /// Filled in at load from [`crate::face`]: never written in content (033).
    #[serde(default)]
    pub text: String,
    /// On play (a program), or when built (a building).
    #[serde(default)]
    pub effects: Vec<Op>,
    /// A building's standing effects.
    #[serde(default)]
    pub passives: Vec<Op>,
    /// A building's once-a-round ability.
    #[serde(default)]
    pub ability: Option<Ability>,
}

/// A card's token type. No card gains tokens by itself (Matt's ruling 12):
/// its effects and ability add them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenSpec {
    pub kind: TokenKind,
}

/// A once-a-round ability: `cost` from the seat, `tokens` from the card's own
/// tokens (021: spend at a rate).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ability {
    #[serde(default)]
    pub cost: Vec<Cost>,
    #[serde(default)]
    pub tokens: i64,
    pub effects: Vec<Op>,
    /// The other branch of an "A, OR B" Action (076, Matt's ruling 12: a
    /// progressive token card adds a token here or spends what is here). One
    /// use a round covers both: the player picks a branch it can pay.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub or: Option<Box<Ability>>,
    /// After this branch resolves the card leaves play, with the stack under
    /// it (r35: a one-shot Action).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub once: bool,
}

impl Ability {
    /// This ability and its OR branch, in print order.
    pub fn branches(&self) -> Vec<&Ability> {
        let mut out = vec![self];
        if let Some(b) = &self.or {
            out.extend(b.branches());
        }
        out
    }
}

// ---------- threats ----------

/// A threat card (135): one deck a tier, dealt by an event card's pulls;
/// its location tokens say where it lands and who is near it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThreatDef {
    pub id: String,
    pub name: String,
    /// 1-3: its deck.
    pub tier: u8,
    /// What pulls it fits (Universal: every one).
    pub tags: Vec<ThreatTag>,
    /// 1 or 2 location types: a token goes on a matching site when it is dealt.
    pub locations: Vec<Location>,
    #[serde(default)]
    pub text: String,
    /// The counter menu: the players near it pay one option in full (pooled).
    pub counters: Vec<Vec<CounterItem>>,
    pub effects: Vec<Op>,
    /// The harms ledger rows it prints (docs/harms.md).
    #[serde(default)]
    pub harms: Vec<String>,
}

/// One item of a threat's counter option (135).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CounterItem {
    /// Pay `amount` of `resource` (pooled across the players near it).
    Pay { resource: Resource, amount: i64 },
    /// Sacrifice cards in play carrying `amount` tags of this kind, as a power's cost.
    Sacrifice { sacrifice: Tag, amount: i64 },
    /// Remove `nodes_near` of your nodes on or next to a token (they leave the game).
    NodesNear { nodes_near: i64 },
}

// ---------- adversary ----------

/// The adversary's content file (135): its board and its event deck.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdversaryFile {
    pub board: AdversaryBoard,
    pub events: Vec<AdversaryCard>,
}

/// An adversary board (135): its project slots, its persistent rules and its
/// difficulty line. Its build numbers live in the config (`adversary.*`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdversaryBoard {
    pub id: String,
    pub name: String,
    pub project_slots: Vec<ProjectSlot>,
    #[serde(default)]
    pub rules: Vec<BoardRule>,
    pub difficulty: BoardRule,
    #[serde(default)]
    pub harms: Vec<String>,
    /// 136: Discord the board sways at the start of every round (the
    /// Militarist prints none).
    #[serde(default)]
    pub discord_drip: i64,
    /// 138: the board's own creation lines, run in every build (they replaced
    /// `city_build` and `factory_build`).
    #[serde(default)]
    pub build: Vec<Op>,
}

/// A project slot on the adversary board: a project must carry `tag` (or be
/// Universal) to fill it; none: any project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSlot {
    #[serde(default)]
    pub tag: Option<ThreatTag>,
}

/// A rule printed on the adversary board. `effects` hold the ops it reads
/// (empty for a rule the config's numbers already carry, such as Wall troops).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoardRule {
    #[serde(default)]
    pub name: String,
    pub text: String,
    #[serde(default)]
    pub effects: Vec<Op>,
}

/// One threat pull of an event card (135): in each country, a threat of
/// `tier` carrying one of `tags` (`any`: every card fits).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pull {
    pub tier: u8,
    pub tags: Vec<PullTag>,
}

/// An adversary event card (135): the clock and the stager. Top to bottom:
/// its pulls, its standing lines (compute, troops and its creation lines,
/// 138) and its twists (read while it is current; its Corrupt X lines among them).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdversaryCard {
    pub id: String,
    pub name: String,
    pub stage: u8,
    #[serde(default)]
    pub pulls: Vec<Pull>,
    #[serde(default)]
    pub standing: Vec<Op>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub twists: Vec<Op>,
    #[serde(default)]
    pub harms: Vec<String>,
}

// ---------- alignment ----------

/// A Unity or Discord card, resolved when its side's bucket tips.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AlignmentCard {
    pub id: String,
    pub name: String,
    pub side: Side,
    /// 1, or 2 for the stronger tier.
    pub tier: u8,
    pub text: String,
    pub effects: Vec<Op>,
    /// Printed strength steps (017): from `at` tips of the card's side
    /// (counted before this tip), `effects` replace the card's effects; the
    /// last step reached applies.
    #[serde(default)]
    pub steps: Vec<Step>,
}

/// A strength step printed on an Alignment card.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub at: i64,
    pub effects: Vec<Op>,
}

/// An adversary project (017): one shared deck, staged (stage 1 cheap,
/// stage 2 big). Adversary compute fills the cheapest visible project; a
/// full one resolves its effects once, or, `ongoing`, every round from then
/// on. Team choices are asked of seat 0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectCard {
    pub id: String,
    pub name: String,
    /// What slots it fits (135; Universal: every slot).
    pub tags: Vec<ThreatTag>,
    /// Compute to fill it, per player.
    pub cost: i64,
    #[serde(default)]
    pub ongoing: bool,
    pub text: String,
    /// A one-shot project's effect; an ongoing one's acts each round (its
    /// standing ops, `ongoing_use` and `each_production`, are read where they act).
    pub effects: Vec<Op>,
    /// What an ongoing project does once, when it resolves (135: Declaration of
    /// War's troops).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resolve: Vec<Op>,
    #[serde(default)]
    pub harms: Vec<String>,
}

// ---------- AI boards ----------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiBoard {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub text: String,
    pub innate: Innate,
    /// Credit production every round, before cards raise it.
    pub credits: i64,
    /// Income every round before the node track adds to it.
    #[serde(default)]
    pub income: Vec<Cost>,
    /// The node track: one space per node. The first token
    /// starts on the board, so the first space is uncovered from setup.
    pub economy_track: Vec<Uncover>,
    /// A second node track (021): placing nodes takes the next token
    /// from the track its player chooses. Its first space is covered at setup.
    #[serde(default)]
    pub play_track: Vec<Uncover>,
    /// AI-board powers (locked: tag costs are sacrifices), each usable once a round.
    #[serde(default)]
    pub powers: Vec<Power>,
    /// Tags printed on the board (068): they count for what a card requires,
    /// not for discounts or sacrifices.
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Innate {
    pub name: String,
    #[serde(default)]
    pub text: String,
    pub passives: Vec<Op>,
}

/// A node-track space: what placing its token uncovers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Uncover {
    /// Income every round once uncovered (insight raises the round's insight).
    #[serde(default)]
    pub income: Vec<Cost>,
    /// A board upgrade (021): keeping a tier-2 card costs
    /// `cards.keep_tier2_upgraded` insight instead of `cards.keep_tier2`.
    #[serde(default)]
    pub keep2: bool,
    /// Energy every placement pays for this step (041; a card can say free).
    #[serde(default)]
    pub cost: i64,
    /// Tags this space reveals (068): as a board's printed tags, they count
    /// for what a card requires.
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// The track's end (068): a standing rule once uncovered.
    #[serde(default)]
    pub capstone: Option<Innate>,
    /// What uncovering this space does, once (075).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Op>,
}

/// An AI-board power (021). `tags` is the tag cost (`"any"` for any tag), paid
/// by discarding played cards that carry them (locked); `cost` other resources;
/// then `effects`. Once a round.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Power {
    pub name: String,
    #[serde(default)]
    pub text: String,
    pub tags: Vec<(TagOrAny, i64)>,
    #[serde(default)]
    pub cost: Vec<Cost>,
    pub effects: Vec<Op>,
}

impl CardDef {
    pub fn is_event(&self) -> bool {
        self.kind == CardKind::Event
    }

    pub fn is_building(&self) -> bool {
        self.kind == CardKind::Building
    }

    /// Prints standing income: a passive that pays a resource every round
    /// while the card stands, flat or per tag.
    pub fn pays_income(&self) -> bool {
        self.passives.iter().any(|o| matches!(o, Op::Income { .. } | Op::IncomePerTag { .. }))
    }
}

impl Content {
    pub fn parse(files: [&str; FILES.len()]) -> Result<Content, String> {
        let [map, cards, threats, adversary, ais, alignment, projects] = files;
        fn p<T: for<'de> Deserialize<'de>>(name: &str, s: &str) -> Result<T, String> {
            serde_json::from_str(s).map_err(|e| format!("content/{name}: {e}"))
        }
        let AdversaryFile { board: adversary_board, events } = p("adversary.json", adversary)?;
        let mut c = Content {
            map: p("map.json", map)?,
            cards: p("cards.json", cards)?,
            threats: p("threats.json", threats)?,
            adversary: events,
            adversary_board,
            ais: p("ais.json", ais)?,
            alignment: p("alignment.json", alignment)?,
            projects: p("projects.json", projects)?,
            card_ix: FastMap::default(),
            card_at: AddressMap::default(),
            threat_ix: FastMap::default(),
            project_ix: FastMap::default(),
            alignment_ix: FastMap::default(),
            adversary_ix: FastMap::default(),
        };
        c.card_ix = index_of(&c.cards, |x| &x.id);
        c.card_at = index_at(&c.cards, |x| &x.id);
        c.threat_ix = index_of(&c.threats, |x| &x.id);
        c.project_ix = index_of(&c.projects, |x| &x.id);
        c.alignment_ix = index_of(&c.alignment, |x| &x.id);
        c.adversary_ix = index_of(&c.adversary, |x| &x.id);
        c.check()?;
        for card in &mut c.cards {
            card.text = crate::face::card_face(card).full_text();
        }
        Ok(c)
    }

    pub(crate) fn check(&self) -> Result<(), String> {
        self.check_ids()?;
        for c in &self.cards {
            check_card(c)?;
        }
        for t in &self.threats {
            check_threat(t)?;
        }
        for a in &self.adversary {
            check_adversary_card(a)?;
        }
        check_adversary_board(&self.adversary_board)?;
        for a in &self.alignment {
            check_alignment(a)?;
        }
        for p in &self.projects {
            check_project(p)?;
        }
        for b in &self.ais {
            self.check_ai_board(b)?;
        }
        let size = self.map.piece.size;
        let n = size as usize * size as usize;
        for (i, p) in self.map.countries.iter().enumerate() {
            check_country_piece(i, p, n)?;
        }
        for l in &self.map.layouts {
            self.check_layout(l)?;
        }
        Ok(())
    }

    fn check_ids(&self) -> Result<(), String> {
        let mut ids = BTreeSet::new();
        let all = self
            .cards
            .iter()
            .map(|c| &c.id)
            .chain(self.threats.iter().map(|t| &t.id))
            .chain(self.adversary.iter().map(|a| &a.id))
            .chain(self.ais.iter().map(|a| &a.id))
            .chain(self.alignment.iter().map(|a| &a.id))
            .chain(self.projects.iter().map(|a| &a.id));
        for id in all {
            if !ids.insert(id.as_str()) {
                return Err(format!(
                    "content: duplicate id {id:?} (ids are unique across all content)"
                ));
            }
        }
        Ok(())
    }

    fn check_ai_board(&self, b: &AiBoard) -> Result<(), String> {
        let what = format!("AI board {:?}", b.id);
        if b.economy_track.is_empty() {
            return Err(format!("{what}: needs a node track"));
        }
        for (r, _) in b
            .income
            .iter()
            .chain(b.economy_track.iter().chain(&b.play_track).flat_map(|u| u.income.iter()))
        {
            resource(r, &what)?;
        }
        // 068: among cards marker clearing stays Science's, but the Protector's
        // cheap power prints it too.
        let not_clearing = |op: &&Op| !matches!(op, Op::ClearMarkers { .. });
        if b.powers.iter().flat_map(|p| &p.effects).filter(not_clearing).any(science_niche) {
            return Err(format!("{what}: shields and delays are Science's"));
        }
        for p in &b.powers {
            check_ops(&p.effects, Slot::Player, &what)?;
            for (r, _) in &p.cost {
                resource(r, &what)?;
            }
            for (_, n) in &p.tags {
                if *n < 1 {
                    return Err(format!("{what}: power tag counts are 1 or more"));
                }
            }
            // Locked (Matt, 075): N of one tag or N of any tag, never a mix.
            if p.tags.len() > 1 {
                return Err(format!("{what}: a power's cost names one tag or `any`, never a mix"));
            }
        }
        check_ops(&b.innate.passives, Slot::Standing, &what)?;
        // 118: a capstone is read only on a track's last space; elsewhere it was shown as
        // uncovered and counted, and never applied.
        for t in [&b.economy_track, &b.play_track] {
            if t.iter().rev().skip(1).any(|u| u.capstone.is_some()) {
                return Err(format!("{what}: a capstone goes on a track's last space"));
            }
        }
        for u in b.economy_track.iter().chain(&b.play_track) {
            if let Some(c) = &u.capstone {
                check_ops(&c.passives, Slot::Standing, &what)?;
                innate_only(&c.passives, &what)?;
            }
            check_ops(&u.effects, Slot::Player, &what)?;
        }
        Ok(())
    }

    fn check_layout(&self, l: &LayoutDef) -> Result<(), String> {
        if l.pieces.len() != l.countries as usize {
            return Err(format!(
                "layout for {} countries has {} pieces",
                l.countries,
                l.pieces.len()
            ));
        }
        if l.countries as usize > self.map.countries.len() {
            return Err(format!(
                "layout for {} countries: not enough country pieces",
                l.countries
            ));
        }
        for p in &l.pieces {
            let d = (p.axes[0].index() + 6 - p.axes[1].index()) % 6;
            if d == 0 || d == 3 {
                return Err(format!(
                    "piece at {:?}: axes {:?} do not span a rhombus",
                    p.anchor, p.axes
                ));
            }
        }
        Ok(())
    }

    pub fn card_index(&self, id: &str) -> Option<usize> {
        match self.card_at.get(&(id.as_ptr() as usize, id.len())) {
            Some(&i) => Some(i),
            None => self.card_ix.get(id).copied(),
        }
    }

    pub fn threat_index(&self, id: &str) -> Option<usize> {
        self.threat_ix.get(id).copied()
    }

    pub fn project_index(&self, id: &str) -> Option<usize> {
        self.project_ix.get(id).copied()
    }

    pub fn alignment_index(&self, id: &str) -> Option<usize> {
        self.alignment_ix.get(id).copied()
    }

    pub fn adversary_index(&self, id: &str) -> Option<usize> {
        self.adversary_ix.get(id).copied()
    }

    /// The card `id`, which must be in content; the same for the four below.
    pub fn card(&self, id: &str) -> &CardDef {
        &self.cards[self.card_index(id).expect("card in content")]
    }

    pub fn threat(&self, id: &str) -> &ThreatDef {
        &self.threats[self.threat_index(id).expect("threat in content")]
    }

    pub fn project(&self, id: &str) -> &ProjectCard {
        &self.projects[self.project_index(id).expect("project in content")]
    }

    pub fn alignment_card(&self, id: &str) -> &AlignmentCard {
        &self.alignment[self.alignment_index(id).expect("alignment card in content")]
    }

    pub fn adversary_card(&self, id: &str) -> &AdversaryCard {
        &self.adversary[self.adversary_index(id).expect("adversary card in content")]
    }

    pub fn layout(&self, countries: u8) -> Option<&LayoutDef> {
        self.map.layouts.iter().find(|l| l.countries == countries)
    }
}

/// A printed cost or a track's income must name a player resource (only
/// `Op::Income` may also print ambassadors, through [`seat_resource`]).
fn resource(r: &Resource, what: &str) -> Result<(), String> {
    if r.is_player() {
        Ok(())
    } else {
        Err(format!("{what}: unknown resource {r:?}"))
    }
}

/// A resource an effect may give or move: a player resource, or the seat's
/// ambassador supply, which effects reach through the same `add_resource`.
fn seat_resource(r: &Resource, what: &str) -> Result<(), String> {
    if *r == Resource::Ambassadors {
        Ok(())
    } else {
        resource(r, what)
    }
}

/// 068: only an innate may hold `Wall`: nothing else is read for it, so
/// anywhere else it would print and do nothing.
fn innate_only(passives: &[Op], what: &str) -> Result<(), String> {
    if passives.iter().any(|o| matches!(o, Op::Wall | Op::Patience)) {
        return Err(format!("{what}: Wall and Patience go on an AI board's innate only"));
    }
    Ok(())
}

/// Whether `op` does a job only Science cards print (046): a shield, a threat
/// delay or marker clearing.
fn science_niche(op: &Op) -> bool {
    matches!(op, Op::ShieldNode { .. } | Op::DelayThreat { .. } | Op::ClearMarkers { .. })
}

fn check_card(c: &CardDef) -> Result<(), String> {
    let what = format!("card {:?}", c.id);
    check_ops(&c.effects, Slot::Player, &what)?;
    check_ops(&c.passives, Slot::Standing, &what)?;
    innate_only(&c.passives, &what)?;
    let branches: Vec<&Ability> = c.ability.iter().flat_map(|a| a.branches()).collect();
    for a in &branches {
        check_ops(&a.effects, Slot::Player, &what)?;
    }
    if c.tags.is_empty() || c.tags.len() > 3 {
        return Err(format!("{what}: 1-3 tags"));
    }
    // 076 (ruling 12): "here" names this building, so only an Action prints it;
    // a count of tokens is one card's, never "you have" across cards.
    let here_op = |o: &Op| {
        matches!(o, Op::AddTokens { here: true, .. } | Op::GainPerTokens { here: true, .. } | Op::CashOut { here: true, .. })
    };
    if c.effects.iter().any(here_op) && c.tokens.is_none() {
        return Err(format!("{what}: \"here\" names a building holding tokens: a program or event has no here"));
    }
    if branches.iter().flat_map(|a| &a.effects).any(here_op) && c.tokens.is_none() {
        return Err(format!("{what}: an Action counts tokens here but the card holds none"));
    }
    // 105: a trigger's or threshold's ops resolve on this card: "here" is it.
    let triggered = c.passives.iter().flat_map(|p| match p {
        Op::When { then, .. } | Op::Threshold { then, .. } => then.as_slice(),
        _ => &[],
    });
    if triggered.clone().any(here_op) && c.tokens.is_none() {
        return Err(format!("{what}: a trigger adds tokens here but the card holds none"));
    }
    if c.passives.iter().any(|p| matches!(p, Op::Threshold { .. })) && c.tokens.is_none() {
        return Err(format!("{what}: a threshold counts tokens here but the card holds none"));
    }
    // 118: an "add N here" of another kind printed one token, counted toward the card's
    // own and fired no trigger of either.
    if let Some(t) = &c.tokens {
        let all = c.effects.iter().chain(branches.iter().flat_map(|a| &a.effects)).chain(triggered.clone());
        if all.clone().any(|o| matches!(o, Op::AddTokens { here: true, kind, .. } if *kind != t.kind)) {
            return Err(format!("{what}: adds tokens here of a kind the card doesn't hold"));
        }
    }
    if c.effects.iter().chain(triggered).any(|o| matches!(o, Op::Reuse)) {
        return Err(format!("{what}: reuse is an Action's (a building's ongoing use, never a one-off)"));
    }
    if c.effects.iter().chain(branches.iter().flat_map(|a| &a.effects)).any(|o| matches!(o, Op::GainPerTokens { here: false, .. })) {
        return Err(format!("{what}: gain_per_tokens counts the tokens here (076, ruling 12)"));
    }
    let niche = c.effects.iter().chain(branches.iter().flat_map(|a| &a.effects)).any(science_niche);
    if niche && !c.tags.contains(&Tag::Science) {
        return Err(format!("{what}: shields, delays and marker clearing are Science's"));
    }
    if niche && c.needs.iter().any(|(t, _)| *t == Tag::Science) {
        return Err(format!("{what}: a Science niche card needs no Science tag, to stay open to a seat without them"));
    }
    if !c.is_building() && !c.passives.is_empty() {
        return Err(format!("{what}: only buildings have passives"));
    }
    if !c.text.is_empty() {
        return Err(format!("{what}: card text is generated from its ops, not written"));
    }
    if !c.is_building() && (c.ability.is_some() || c.tokens.is_some()) {
        return Err(format!(
            "{what}: only a building has an Action or gains tokens each round"
        ));
    }
    if c.replaces.is_some() && !c.is_building() {
        return Err(format!("{what}: only a building replaces one"));
    }
    if !(1..=2).contains(&c.tier) {
        return Err(format!("{what}: tier 1 or 2"));
    }
    for (r, _) in &c.lowers {
        resource(r, &what)?;
    }
    for a in &branches {
        for (r, _) in &a.cost {
            resource(r, &what)?;
        }
        if a.tokens != 0 && c.tokens.is_none() {
            return Err(format!("{what}: spends tokens but holds none"));
        }
    }
    Ok(())
}

fn check_threat(t: &ThreatDef) -> Result<(), String> {
    if !(1..=3).contains(&t.tier) || t.counters.is_empty() || t.counters.iter().any(|o| o.is_empty()) {
        return Err(format!("threat {:?}: tier 1-3 and a counter menu", t.id));
    }
    if t.tags.is_empty() || !(1..=2).contains(&t.locations.len()) {
        return Err(format!("threat {:?}: a tag and 1 or 2 locations", t.id));
    }
    let what = format!("threat {:?}", t.id);
    for item in t.counters.iter().flatten() {
        match item {
            CounterItem::Pay { resource: r, amount } => {
                if *r != Resource::Ambassadors {
                    resource(r, &what)?;
                }
                if *amount < 1 {
                    return Err(format!("{what}: a counter item of 1 or more"));
                }
            }
            CounterItem::Sacrifice { amount, .. } | CounterItem::NodesNear { nodes_near: amount } => {
                if *amount < 1 {
                    return Err(format!("{what}: a counter item of 1 or more"));
                }
            }
        }
    }
    check_ops(&t.effects, Slot::Threat, &what)?;
    // 142: a counter never pays in a currency its uncountered harm takes ("pay 7 credits or lose 8").
    let harm = harm_currencies(&t.effects);
    for item in t.counters.iter().flatten() {
        let paid = match item {
            CounterItem::Pay { resource, .. } => Currency::Resource(*resource),
            CounterItem::NodesNear { .. } => Currency::Nodes,
            CounterItem::Sacrifice { .. } => continue,
        };
        if harm.contains(&paid) {
            return Err(format!("{what}: a counter and its uncountered harm share {paid:?}"));
        }
    }
    Ok(())
}

/// What a threat's harm or counter takes from a player (142's shared-currency check; check.py's twin is
/// `harm_currencies`). Discarding from the hand is not a counter currency, so it isn't listed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Currency {
    Resource(Resource),
    Nodes,
}

/// The currencies `effects` take from the players near a threat, `LoseOr`'s options included.
pub fn harm_currencies(effects: &[Op]) -> Vec<Currency> {
    let mut out = Vec::new();
    for op in effects {
        let (r, nodes) = match op {
            Op::PlayersLose { resource, .. }
            | Op::LoseIncome { resource, .. }
            | Op::Theft { resource, .. }
            | Op::Requisition { resource, .. } => (Some(*resource), false),
            Op::LoseNodeOr { resource, .. } => (Some(*resource), true),
            Op::LoseAmbassadorsOr { resource, .. } => {
                out.push(Currency::Resource(Resource::Ambassadors));
                (Some(*resource), false)
            }
            Op::LoseInsight { .. } => (Some(Resource::Insight), false),
            Op::LoseAmbassador => (Some(Resource::Ambassadors), false),
            Op::EachDestroyNode { .. } | Op::DestroyFacility { .. } => (None, true),
            Op::Troops { destroy, .. } => (None, *destroy),
            Op::CorruptCity { at } => (None, *at == Some(Spot::Location)),
            Op::LoseOr { options } => {
                out.extend(harm_currencies(options));
                (None, false)
            }
            _ => (None, false),
        };
        out.extend(r.map(Currency::Resource));
        if nodes {
            out.push(Currency::Nodes);
        }
    }
    out
}

fn check_adversary_card(a: &AdversaryCard) -> Result<(), String> {
    if !(1..=3).contains(&a.stage) {
        return Err(format!("adversary card {:?}: stage 1-3", a.id));
    }
    if a.pulls.iter().any(|p| !(1..=3).contains(&p.tier) || p.tags.is_empty()) {
        return Err(format!("adversary card {:?}: each pull a tier 1-3 and a tag", a.id));
    }
    check_ops(&a.twists, Slot::Twist, &format!("adversary card {:?}", a.id))?;
    check_ops(&a.standing, Slot::Team, &format!("adversary card {:?} standing", a.id))?;
    Ok(())
}

fn check_adversary_board(b: &AdversaryBoard) -> Result<(), String> {
    let what = format!("adversary board {:?}", b.id);
    if b.project_slots.is_empty() {
        return Err(format!("{what}: needs a project slot"));
    }
    for r in b.rules.iter().chain([&b.difficulty]) {
        check_ops(&r.effects, Slot::Team, &what)?;
    }
    // 138: the board's build is creation lines only.
    if b.build.iter().any(|e| !matches!(e, Op::Create { at, if_, .. } if *at != crate::op::CreateAt::Location && *if_ != crate::op::CreateIf::Near)) {
        return Err(format!("{what}: its build prints creation lines only (no threat location)"));
    }
    Ok(())
}

fn check_alignment(a: &AlignmentCard) -> Result<(), String> {
    if !(1..=2).contains(&a.tier) || a.effects.is_empty() {
        return Err(format!(
            "alignment card {:?}: tier 1 or 2 and an effect",
            a.id
        ));
    }
    if a.steps.windows(2).any(|p| p[0].at >= p[1].at) || a.steps.iter().any(|s| s.at < 1) {
        return Err(format!(
            "alignment card {:?}: steps at 1 or more tips, in rising order",
            a.id
        ));
    }
    let what = format!("alignment card {:?}", a.id);
    check_ops(&a.effects, Slot::Team, &what)?;
    for st in &a.steps {
        check_ops(&st.effects, Slot::Team, &what)?;
    }
    Ok(())
}

fn check_project(p: &ProjectCard) -> Result<(), String> {
    if p.tags.is_empty() || p.cost < 1 || p.effects.is_empty() {
        return Err(format!(
            "project {:?}: a tag, a cost of 1 or more and an effect",
            p.id
        ));
    }
    check_ops(&p.effects, Slot::Team, &format!("project {:?}", p.id))?;
    check_ops(&p.resolve, Slot::Team, &format!("project {:?}", p.id))?;
    if !p.ongoing && !p.resolve.is_empty() {
        return Err(format!("project {:?}: only an ongoing project prints a resolve line", p.id));
    }
    Ok(())
}

fn check_country_piece(i: usize, p: &CountryPiece, n: usize) -> Result<(), String> {
    let start = vec![p.node];
    let neutral = vec![p.neutral_station];
    let lists = [
        &p.cities,
        &p.corrupted,
        &p.ocean,
        &p.coast,
        &p.stations,
        &neutral,
        &p.industry,
        &start,
    ];
    if lists
        .iter()
        .flat_map(|l| l.iter())
        .any(|&h| h == 0 || h as usize > n)
    {
        return Err(format!("country piece {i}: hex numbers are 1-{n}"));
    }
    if p.coast.iter().any(|h| !p.cities.contains(h)) {
        return Err(format!("country piece {i}: a coast entry is not a city"));
    }
    if p.corrupted.iter().any(|c| !p.cities.contains(c)) {
        return Err(format!(
            "country piece {i}: corrupted cities must be cities"
        ));
    }
    if p.factories.iter().any(|f| !p.industry.contains(f)) {
        return Err(format!(
            "country piece {i}: setup factories go on industrial zones"
        ));
    }
    let built: Vec<u8> = [&p.stations, &neutral, &p.industry, &start]
        .iter()
        .flat_map(|l| l.iter().copied())
        .collect();
    for (k, &h) in built.iter().enumerate() {
        if p.kind_at(h) != HexKind::Open || built[..k].contains(&h) {
            return Err(format!(
                "country piece {i}: buildings go on open hexes, one per hex (#{h})"
            ));
        }
    }
    let what = format!("project of country piece {i}");
    let m = &p.megaproject;
    if !(6..=12).contains(&m.cubes) || m.player.is_empty() || m.adversary.is_empty() {
        return Err(format!("{what}: 6-12 cubes, and a payout for each color"));
    }
    check_ops(&m.player, Slot::Player, &what)?;
    check_ops(&m.adversary, Slot::Team, &what)?;
    check_ops(&m.converter.effects, Slot::Player, &what)?;
    if m.converter.cost.is_empty() || m.converter.effects.is_empty() {
        return Err(format!("{what}: a converter has a cost and an effect"));
    }
    for (r, _) in &m.converter.cost {
        resource(r, &what)?;
    }
    Ok(())
}

/// Checks every op of a content list: it may sit in `slot`, and the
/// resources it names are ones it can use.
pub(crate) fn check_ops(ops: &[Op], slot: Slot, what: &str) -> Result<(), String> {
    for op in ops {
        if !op.slots().contains(&slot) {
            return Err(format!("{what}: op {op:?} can't go in a {slot:?} list"));
        }
        match op {
            Op::Gain { resource: r, .. }
            | Op::GainPerNode { resource: r, .. }
            | Op::OtherGain { resource: r, .. }
            | Op::Transfer { resource: r, .. }
            | Op::CashOut { resource: r, .. } => seat_resource(r, what)?,
            Op::Damage { ambassadors, troops } | Op::DamageCountry { ambassadors, troops }
                if *ambassadors < 0 || *troops < 0 || ambassadors + troops < 1 =>
            {
                return Err(format!("{what}: a removal names at least one piece"));
            }
            Op::RemoveTroops { count, .. } if *count < 1 => {
                return Err(format!("{what}: remove_troops needs count >= 1"));
            }
            // 142 (P1): the no-location form (the most infiltrated city) is gone.
            Op::CorruptCity { at: None } => {
                return Err(format!("{what}: corrupt_city needs `at` (the threat's city); Corrupt cities is `corrupt`"));
            }
            Op::GainPerTag { resource: r, per, .. } => {
                seat_resource(r, what)?;
                if *per < 1 {
                    return Err(format!("{what}: gain_per_tag needs per >= 1"));
                }
            }
            Op::ProductionPerTag { resource: r, per, .. } => {
                resource(r, what)?;
                if *per < 1 {
                    return Err(format!("{what}: production_per_tag needs per >= 1"));
                }
            }
            Op::IncomePerTag { resource: r, per, amount, .. } => {
                resource(r, what)?;
                if *per < 1 || *amount < 1 {
                    return Err(format!("{what}: income_per_tag needs per >= 1 and amount >= 1"));
                }
            }
            Op::GainPerTokens { resource: r, per, .. } => {
                seat_resource(r, what)?;
                if *per < 1 {
                    return Err(format!("{what}: gain_per_tokens needs per >= 1"));
                }
            }
            Op::ConvertUpkeep { from, to, .. } => {
                resource(from, what)?;
                resource(to, what)?;
            }
            Op::Production { resource: r, .. }
            | Op::PlayersLose { resource: r, .. }
            | Op::LoseNodeOr { resource: r, .. }
            | Op::Requisition { resource: r, .. }
            | Op::Logistics { resource: r, .. } => resource(r, what)?,
            // 068: income can print ambassadors, as effects do.
            Op::Income { resource: r, .. } => seat_resource(r, what)?,
            Op::TeamIncome { resource: r, .. } => {
                if !matches!(r, Resource::Credits | Resource::Insight) {
                    return Err(format!("{what}: team_income is credits or insight"));
                }
            }
            Op::Troops { each_country: true, .. } | Op::FlipStation { each_country: true, .. }
                if slot != Slot::Team =>
            {
                return Err(format!("{what}: each_country is for team cards"));
            }
            Op::LoseOr { options } => {
                if options.len() < 2 {
                    return Err(format!("{what}: lose_or offers at least two options"));
                }
                check_ops(options, slot, what)?;
            }
            // 105: a trigger resolves player ops; "both" hands the same gains to
            // the player who played, so it prints gains only.
            Op::When { on, then } => {
                check_ops(then, Slot::Player, what)?;
                if then.is_empty() {
                    return Err(format!("{what}: a trigger resolves something"));
                }
                if matches!(on, Event::TeamPlay { both: true, .. }) && !then.iter().all(|o| matches!(o, Op::Gain { .. })) {
                    return Err(format!("{what}: a trigger both players gain from prints gains only"));
                }
                if matches!(on, Event::PlayCost { at } if *at < 1) {
                    return Err(format!("{what}: a rebate's cost is at least 1"));
                }
                // A token placed fires it; a token it placed would fire it again, forever.
                if matches!(on, Event::PlaceTokens { .. }) && then.iter().any(|o| matches!(o, Op::AddTokens { .. })) {
                    return Err(format!("{what}: a place-token trigger places no tokens (it would loop)"));
                }
            }
            Op::Threshold { at, then } => {
                check_ops(then, Slot::Player, what)?;
                if *at < 1 || then.is_empty() {
                    return Err(format!("{what}: a threshold counts to at least 1 and resolves something"));
                }
                // Its tokens come off and the rest resolves while N stand here: tokens it
                // placed would count it again, forever.
                if then.iter().any(|o| matches!(o, Op::AddTokens { .. })) {
                    return Err(format!("{what}: a threshold places no tokens (it would loop)"));
                }
            }
            Op::CardDiscount { resource: r, amount, .. } => {
                if !matches!(r, Resource::Credits | Resource::Insight) || *amount < 1 {
                    return Err(format!("{what}: a card discount takes 1 or more credits or insight off"));
                }
            }
            Op::DiscardHand { resource: r, .. } => seat_resource(r, what)?,
            _ => {}
        }
    }
    Ok(())
}

/// The embedded content, parsed once.
pub fn content() -> &'static Content {
    static C: OnceLock<Content> = OnceLock::new();
    C.get_or_init(|| Content::parse(FILES).expect("embedded content is valid"))
}

/// The tables an overlay reaches, each with its index in [`FILES`]. Of the
/// map only `corps` (075): its country pieces, by lowercase name
/// (`corps.horizon.project.cubes=8`); a layout is not a number to sweep.
const OVERLAY_TABLES: [(&str, usize); 7] = [
    ("corps", 0),
    ("cards", 1),
    ("threats", 2),
    ("adversary", 3),
    ("ais", 4),
    ("alignment", 5),
    ("projects", 6),
];

/// `<table>.<id>.<path>`: a content overlay key, not a `<module>.<param>`.
pub fn is_overlay_key(key: &str) -> bool {
    let mut parts = key.split('.');
    let table = parts.next().unwrap_or("");
    OVERLAY_TABLES.iter().any(|(t, _)| *t == table) && parts.count() >= 2
}

/// One overlay entry applied to the parsed content files. `path` walks
/// object fields and list indexes (`effects.0.amount`, `economy_track.4.cost`);
/// the last step may add a field the JSON left at its default, and parsing
/// rejects one the schema doesn't have. `id` may be `*`: every row of the
/// table. The value is JSON, or a bare word taken as a string.
fn apply_entry(files: &mut [serde_json::Value], entry: &str) -> Result<(), String> {
    use serde_json::Value;
    let fail = |why: String| format!("content overlay {entry:?}: {why}");
    let shape = || fail("expected <table>.<id>.<path>=<value>".into());
    let (key, value) = entry.split_once('=').ok_or_else(shape)?;
    if !is_overlay_key(key.trim()) {
        return Err(shape());
    }
    let mut parts = key.trim().split('.');
    let (table, id) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
    let path: Vec<&str> = parts.collect();
    if path[0] == "id" {
        return Err(fail("an id can't be overlaid".into()));
    }
    let value = value.trim();
    let value: Value =
        serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.to_string()));
    let ix = OVERLAY_TABLES.iter().find(|(t, _)| *t == table).expect("is_overlay_key: a table").1;
    let corps = table == "corps";
    // 135: the adversary file is its board and its event list; an overlay reaches the events.
    let rows = match table {
        "corps" => files[ix].get_mut("countries"),
        "adversary" => files[ix].get_mut("events"),
        _ => Some(&mut files[ix]),
    };
    let rows = rows.and_then(Value::as_array_mut).ok_or_else(|| fail("not a list".into()))?;
    let mut hit = false;
    for row in rows {
        let name = if corps {
            row.get("name").and_then(Value::as_str).map(str::to_lowercase)
        } else {
            row.get("id").and_then(Value::as_str).map(str::to_string)
        };
        if id != "*" && name.as_deref() != Some(id) {
            continue;
        }
        hit = true;
        let (last, walk) = path.split_last().expect("is_overlay_key: a path");
        let mut at = &mut *row;
        for step in walk {
            at = match at {
                Value::Array(a) => step.parse::<usize>().ok().and_then(|i| a.get_mut(i)),
                Value::Object(o) => o.get_mut(*step),
                _ => None,
            }
            .ok_or_else(|| fail(format!("no {step:?} on the way")))?;
        }
        match at {
            Value::Array(a) => {
                let slot = last.parse::<usize>().ok().and_then(|i| a.get_mut(i));
                *slot.ok_or_else(|| fail(format!("no list entry {last:?}")))? = value.clone();
            }
            Value::Object(o) => {
                o.insert(last.to_string(), value.clone());
            }
            _ => return Err(fail(format!("{last:?} is inside a plain value"))),
        }
    }
    if !hit {
        return Err(fail(format!("no id {id:?} in {table}")));
    }
    Ok(())
}

/// The content with `entries` applied, and its hash. Parsed through
/// [`Content::parse`], so an overlay meets every check content does and card
/// text follows the new ops. `Rules` hold content for the life of the
/// process, so each distinct overlay is built once and kept: bounded by the
/// experiment, and invisible to the rules (same entries, same content).
pub fn overlaid(entries: &[String]) -> Result<(&'static Content, &'static str), String> {
    static BASE_HASH: OnceLock<String> = OnceLock::new();
    type Built = (&'static Content, &'static str);
    static BUILT: Mutex<BTreeMap<Vec<String>, Built>> = Mutex::new(BTreeMap::new());
    let base = BASE_HASH.get_or_init(content_hash).as_str();
    if entries.is_empty() {
        return Ok((content(), base));
    }
    if let Some(b) = BUILT.lock().expect("overlay lock").get(entries) {
        return Ok(*b);
    }
    let mut files: Vec<serde_json::Value> = FILES
        .iter()
        .map(|f| serde_json::from_str(f).expect("embedded content is JSON"))
        .collect();
    for e in entries {
        apply_entry(&mut files, e)?;
    }
    let text: Vec<String> = files.iter().map(ToString::to_string).collect();
    let refs: [&str; FILES.len()] = std::array::from_fn(|i| text[i].as_str());
    let c = Content::parse(refs).map_err(|e| format!("content overlay {entries:?}: {e}"))?;
    let hash = format!("{:016x}", fnv1a64(format!("{base}\0{}", entries.join("\0")).as_bytes()));
    let b: Built = (Box::leak(Box::new(c)), Box::leak(hash.into_boxed_str()));
    // Two threads may build one overlay; both copies are the same content.
    Ok(*BUILT.lock().expect("overlay lock").entry(entries.to_vec()).or_insert(b))
}

/// Hash of the embedded content files, recorded with every result so a
/// content edit is never mistaken for the same experiment. A config with an
/// overlay plays other content: `Rules::content_hash` names it.
pub fn content_hash() -> String {
    let mut bytes = Vec::new();
    for f in FILES {
        bytes.extend_from_slice(f.as_bytes());
        bytes.push(0);
    }
    format!("{:016x}", fnv1a64(&bytes))
}

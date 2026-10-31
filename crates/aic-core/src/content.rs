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
pub use crate::op::{Cond, CountryRule, DamageAt, Op, RoundFlag, Slot, Spot};
pub use crate::vocab::{
    DiscountOn, PerResource, Resource, Side, Tag, TagOrAny, TokenKind, RESOURCES, TAGS,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    pub map: MapContent,
    pub cards: Vec<CardDef>,
    pub threats: Vec<ThreatDef>,
    pub adversary: Vec<AdversaryCard>,
    pub ais: Vec<AiBoard>,
    /// The Unity and Discord decks (module `alignment`).
    pub alignment: Vec<AlignmentCard>,
    /// The adversary's project deck (module `adversary`, 017).
    pub projects: Vec<ProjectCard>,
    /// Id -> index into each list, for [`Content::card_index`] and the like
    /// (rules and agents look things up by id constantly). Built by `parse`.
    #[serde(skip)]
    card_ix: FastMap<usize>,
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
    /// Display colour from the reference images; the core never reads it.
    #[serde(default)]
    pub colour: String,
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
    pub name: String,
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
    /// Printed industrial zones: open hexes next to a city where factories go.
    pub industry: Vec<u8>,
    /// Industrial zones holding a factory at setup (neutral).
    pub factories: Vec<u8>,
    /// Where the starting presence of the player who starts here goes.
    pub presence: u8,
    /// The corp's megaproject card.
    #[serde(rename = "project")]
    pub megaproject: Megaproject,
}

/// A megaproject card: research stations fill it with cubes; when it holds
/// `cubes`, it completes and every cube pays its owner. After that, each
/// owned station in the country produces `benefit` for its owner every round.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Megaproject {
    pub name: String,
    pub cubes: i64,
    #[serde(default)]
    pub text: String,
    pub benefit: Vec<Cost>,
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
    /// 1 or 2 for the draft decks; 0 for an AI's unique cards.
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
    /// The token type this card holds, and how many it gains a round while
    /// an active building (021: token engines).
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
    /// A building that takes no building slot.
    #[serde(default)]
    pub slotless: bool,
    /// The AI board this card is unique to.
    #[serde(default)]
    pub unique: Option<String>,
}

/// A card's token type and its gain a round (in upkeep, while the card is
/// an active building).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenSpec {
    pub kind: TokenKind,
    #[serde(default)]
    pub per_round: i64,
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
}

// ---------- threats ----------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThreatDef {
    pub id: String,
    pub name: String,
    pub tier: u8,
    #[serde(default)]
    pub text: String,
    /// What countering it costs, in order (difficulty 1 raises the first).
    pub counter: Vec<Cost>,
    pub effects: Vec<Op>,
}

// ---------- adversary ----------

/// An adversary deck card: the clock and the stager. Its twists change
/// the round it is the current card of.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdversaryCard {
    pub id: String,
    pub name: String,
    pub stage: u8,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub twists: Vec<Op>,
    /// The Militarist's escalation: each military base gains an adversary
    /// ambassador when this card's round builds.
    #[serde(default)]
    pub escalation: bool,
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
    pub stage: u8,
    /// Compute to fill it, per player.
    pub cost: i64,
    #[serde(default)]
    pub ongoing: bool,
    pub text: String,
    pub effects: Vec<Op>,
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
    /// Resources this AI may spend as credits on some cards.
    #[serde(default)]
    pub discounts: Vec<Discount>,
    /// Income every round before the presence track adds to it.
    #[serde(default)]
    pub income: Vec<Cost>,
    /// The presence track: one space per presence token. The first token
    /// starts on the board, so the first space is uncovered from setup.
    pub presence: Vec<Uncover>,
    /// A second presence track (021): placing presence takes the next token
    /// from the track its player chooses. Its first space is covered at setup.
    #[serde(default)]
    pub presence2: Vec<Uncover>,
    pub growth: Vec<Growth>,
    /// Unique cards, in hand at the start.
    pub cards: Vec<String>,
    /// AI-board powers (021, locked: paid by sacrificing tags), each usable once a round.
    #[serde(default)]
    pub powers: Vec<Power>,
    /// A building the board starts with in a slot, carrying its built-in tag (021).
    #[serde(default)]
    pub start: Option<String>,
}

/// Spend `resource` for `rate` credits each off a card's credit cost: on
/// cards tagged `tag`, or on any building for `tag` "building".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Discount {
    pub resource: Resource,
    pub tag: DiscountOn,
    pub rate: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Innate {
    pub name: String,
    #[serde(default)]
    pub text: String,
    pub passives: Vec<Op>,
}

/// A presence-track space: what placing its token uncovers.
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
    /// The action adds `board.presence_energy`; the rich action pays it in
    /// credits.
    #[serde(default)]
    pub cost: i64,
}

/// An AI-board power (021, locked: paid by sacrificing tags). `tags` is the
/// tag cost (`"any"` for any tag), paid by discarding played cards that carry
/// them; `cost` other resources; then `effects`. Once a round.
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Growth {
    pub name: String,
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
    /// while the card stands, flat, per tag or if-tags.
    pub fn pays_income(&self) -> bool {
        self.passives.iter().any(|o| {
            matches!(
                o,
                Op::Income { .. } | Op::IncomePerTag { .. } | Op::IncomeIfTags { .. }
            )
        })
    }
}

impl Content {
    pub fn parse(files: [&str; 7]) -> Result<Content, String> {
        let [map, cards, threats, adversary, ais, alignment, projects] = files;
        fn p<T: for<'de> Deserialize<'de>>(name: &str, s: &str) -> Result<T, String> {
            serde_json::from_str(s).map_err(|e| format!("content/{name}: {e}"))
        }
        let mut c = Content {
            map: p("map.json", map)?,
            cards: p("cards.json", cards)?,
            threats: p("threats.json", threats)?,
            adversary: p("adversary.json", adversary)?,
            ais: p("ais.json", ais)?,
            alignment: p("alignment.json", alignment)?,
            projects: p("projects.json", projects)?,
            card_ix: FastMap::default(),
            threat_ix: FastMap::default(),
            project_ix: FastMap::default(),
            alignment_ix: FastMap::default(),
            adversary_ix: FastMap::default(),
        };
        c.card_ix = index_of(&c.cards, |x| &x.id);
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
        if b.presence.is_empty() {
            return Err(format!("AI board {:?}: a presence track", b.id));
        }
        for c in &b.cards {
            match self.card_index(c) {
                Some(i) if self.cards[i].unique.as_deref() == Some(b.id.as_str()) => {}
                _ => {
                    return Err(format!(
                        "AI board {:?}: {c:?} is not a card unique to it",
                        b.id
                    ))
                }
            }
        }
        let what = format!("AI board {:?}", b.id);
        for (r, _) in b
            .income
            .iter()
            .chain(b.presence.iter().chain(&b.presence2).flat_map(|u| u.income.iter()))
        {
            resource(r, &what)?;
        }
        for d in &b.discounts {
            resource(&d.resource, &what)?;
        }
        for gr in &b.growth {
            check_ops(&gr.effects, Slot::Player, &what)?;
        }
        if b.growth.iter().map(|g| &g.effects).chain(b.powers.iter().map(|p| &p.effects)).flatten().any(science_niche) {
            return Err(format!("{what}: shields, delays and marker clearing are Science's"));
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
        }
        if let Some(id) = &b.start {
            match self.card_index(id) {
                Some(i) if self.cards[i].is_building() => {}
                _ => return Err(format!("{what}: start building {id:?} is not a building")),
            }
        }
        check_ops(&b.innate.passives, Slot::Standing, &what)?;
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
        self.card_ix.get(id).copied()
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

    pub fn layout(&self, countries: u8) -> Option<&LayoutDef> {
        self.map.layouts.iter().find(|l| l.countries == countries)
    }

}

/// A cost or income must name a player resource.
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

/// Whether `op` does a job only Science cards print (046): a shield, a threat
/// delay or marker clearing. `if` branches included.
fn science_niche(op: &Op) -> bool {
    match op {
        Op::ShieldPresence { .. } | Op::DelayThreat { .. } | Op::ClearMarkers { .. } => true,
        Op::If { then, otherwise, .. } => then.iter().chain(otherwise).any(science_niche),
        _ => false,
    }
}

fn check_card(c: &CardDef) -> Result<(), String> {
    let what = format!("card {:?}", c.id);
    check_ops(&c.effects, Slot::Player, &what)?;
    check_ops(&c.passives, Slot::Standing, &what)?;
    if let Some(a) = &c.ability {
        check_ops(&a.effects, Slot::Player, &what)?;
    }
    if c.tags.is_empty() || c.tags.len() > 3 {
        return Err(format!("card {:?}: 1-3 tags", c.id));
    }
    let printed = c.effects.iter().chain(c.ability.iter().flat_map(|a| &a.effects));
    if !c.tags.contains(&Tag::Science) && printed.into_iter().any(science_niche) {
        return Err(format!("{what}: shields, delays and marker clearing are Science's"));
    }
    if !c.is_building()
        && (!c.passives.is_empty() || c.slotless)
    {
        return Err(format!(
            "card {:?}: only buildings have passives",
            c.id
        ));
    }
    if c.slotless && !crate::face::prints_an_effect(c) {
        return Err(format!("{what}: a slotless card needs an effect passive"));
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
        return Err(format!("card {:?}: only a building replaces one", c.id));
    }
    if c.tier > 2 || (c.tier == 0) != c.unique.is_some() {
        return Err(format!(
            "card {:?}: tier 0 exactly for unique cards, else 1 or 2",
            c.id
        ));
    }
    for (r, _) in &c.lowers {
        resource(r, &what)?;
    }
    if let Some(a) = &c.ability {
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
    if !(1..=2).contains(&t.tier) || t.counter.is_empty() {
        return Err(format!("threat {:?}: tier 1 or 2 and a counter cost", t.id));
    }
    let what = format!("threat {:?}", t.id);
    for (r, _) in &t.counter {
        resource(r, &what)?;
    }
    check_ops(&t.effects, Slot::Threat, &what)?;
    Ok(())
}

fn check_adversary_card(a: &AdversaryCard) -> Result<(), String> {
    if !(1..=3).contains(&a.stage) {
        return Err(format!("adversary card {:?}: stage 1-3", a.id));
    }
    check_ops(&a.twists, Slot::Twist, &format!("adversary card {:?}", a.id))?;
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
    if !(1..=2).contains(&p.stage) || p.cost < 1 || p.effects.is_empty() {
        return Err(format!(
            "project {:?}: stage 1 or 2, a cost of 1 or more and an effect",
            p.id
        ));
    }
    check_ops(&p.effects, Slot::Team, &format!("project {:?}", p.id))?;
    Ok(())
}

fn check_country_piece(i: usize, p: &CountryPiece, n: usize) -> Result<(), String> {
    let start = vec![p.presence];
    let lists = [
        &p.cities,
        &p.corrupted,
        &p.ocean,
        &p.coast,
        &p.stations,
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
    let built: Vec<u8> = [&p.stations, &p.industry, &start]
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
    for (r, _) in &p.megaproject.benefit {
        resource(r, &format!("project of country piece {i}"))?;
    }
    Ok(())
}

/// Checks every op of a content list: it may sit in `slot`, and the
/// resources it names are ones it can use. `if` branches included.
pub(crate) fn check_ops(ops: &[Op], slot: Slot, what: &str) -> Result<(), String> {
    for op in ops {
        if !op.slots().contains(&slot) {
            return Err(format!("{what}: op {op:?} can't go in a {slot:?} list"));
        }
        match op {
            Op::Gain { resource: r, .. }
            | Op::GainPerPresence { resource: r, .. }
            | Op::OtherGain { resource: r, .. }
            | Op::Transfer { resource: r, .. }
            | Op::CashOut { resource: r, .. } => seat_resource(r, what)?,
            Op::GainPerTokens { resource: r, per, .. } => {
                seat_resource(r, what)?;
                if *per < 1 {
                    return Err(format!("{what}: gain_per_tokens needs per >= 1"));
                }
            }
            Op::Convert { from, to, .. } | Op::ConvertUpkeep { from, to, .. } => {
                resource(from, what)?;
                resource(to, what)?;
            }
            Op::Production { resource: r, .. }
            | Op::PlayersLose { resource: r, .. }
            | Op::LosePresenceOr { resource: r, .. }
            | Op::Requisition { resource: r, .. }
            | Op::Income { resource: r, .. }
            | Op::Logistics { resource: r, .. }
            | Op::GiveBonus { resource: r, .. }
            | Op::IncomePerTag { resource: r, .. }
            | Op::IncomeIfTags { resource: r, .. } => resource(r, what)?,
            Op::TeamIncome { resource: r, .. } => {
                if !matches!(r, Resource::Credits | Resource::Insight) {
                    return Err(format!("{what}: team_income is credits or insight"));
                }
            }
            Op::Troops { each_country: true, .. } | Op::FlipStation { each_country: true }
                if slot != Slot::Team =>
            {
                return Err(format!("{what}: each_country is for team cards"));
            }
            Op::If { then, otherwise, .. } => {
                check_ops(then, slot, what)?;
                check_ops(otherwise, slot, what)?;
            }
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

/// The tables an overlay reaches, each with its index in [`FILES`]. The map
/// stays out: its pieces have no ids, and a layout is not a number to sweep.
const OVERLAY_TABLES: [(&str, usize); 6] = [
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
/// object fields and list indexes (`effects.0.amount`, `presence.4.cost`);
/// the last step may add a field the JSON left at its default, and parsing
/// rejects one the schema doesn't have. `id` may be `*`: every row of the
/// table. The value is JSON, or a bare word taken as a string.
fn apply_entry(files: &mut [serde_json::Value], entry: &str) -> Result<(), String> {
    use serde_json::Value;
    let fail = |why: String| format!("content overlay {entry:?}: {why}");
    let (key, value) = entry
        .split_once('=')
        .ok_or_else(|| fail("expected <table>.<id>.<path>=<value>".into()))?;
    if !is_overlay_key(key.trim()) {
        return Err(fail("expected <table>.<id>.<path>=<value>".into()));
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
    let ix = OVERLAY_TABLES.iter().find(|(t, _)| *t == table).map_or(0, |t| t.1);
    let rows = files[ix].as_array_mut().ok_or_else(|| fail("not a list".into()))?;
    let mut hit = false;
    for row in rows {
        if id != "*" && row.get("id").and_then(Value::as_str) != Some(id) {
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
    let refs: [&str; 7] = std::array::from_fn(|i| text[i].as_str());
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

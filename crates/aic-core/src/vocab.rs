//! The game's closed word lists as Rust enums: resources, tags, token types
//! and the Alignment sides. Content, actions and the state name them in JSON
//! by their snake_case word; everything else uses the enum, so a misspelt
//! resource or tag can't compile (or load).

use std::fmt;

use serde::{Deserialize, Serialize};

/// Everything a seat holds as a count (`Seat::resources`).
///
/// **Declared alphabetically on purpose:** the derived `Ord` then matches the
/// order of the words, which `Seat::resources` serializes in. Reordering these
/// changes state fingerprints. Play doesn't read the order: nothing sorts
/// actions, and legal actions and agents walk [`RESOURCES`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Resource {
    /// Player ambassadors in the seat's supply (effects give them like a
    /// resource; never a cost or an income).
    Ambassadors,
    Antimatter,
    Chips,
    Credits,
    Energy,
    Insight,
}

/// The player resources, in display order (the order of income arrays, see
/// [`Resource::index`]). 021: players have no compute; compute is the
/// adversary's, `adversary.compute`.
pub const RESOURCES: [Resource; 5] = [
    Resource::Credits,
    Resource::Chips,
    Resource::Energy,
    Resource::Antimatter,
    Resource::Insight,
];

impl Resource {
    pub fn name(self) -> &'static str {
        match self {
            Resource::Ambassadors => "ambassadors",
            Resource::Antimatter => "antimatter",
            Resource::Credits => "credits",
            Resource::Energy => "energy",
            Resource::Chips => "chips",
            Resource::Insight => "insight",
        }
    }

    /// `n` of this resource in words: "2 chips", "1 chip". Only chips take a
    /// singular so far; the other words print as they are.
    ///
    /// An option string is a record key, so what goes into one stays this
    /// (`1 credits`); text only people read takes [`Resource::words`].
    pub fn amount(self, n: i64) -> String {
        match (self, n) {
            (Resource::Chips, 1) => "1 chip".to_string(),
            _ => format!("{n} {self}"),
        }
    }

    /// The same for a label or a doc: `1 credit`.
    pub fn words(self, n: i64) -> String {
        match (self, n) {
            (Resource::Credits, 1) => "1 credit".to_string(),
            _ => self.amount(n),
        }
    }

    /// Position in [`RESOURCES`]; `None` for what is not a player resource.
    pub fn index(self) -> Option<usize> {
        RESOURCES.iter().position(|r| *r == self)
    }

    /// One of the five [`RESOURCES`]: what costs and incomes may name.
    pub fn is_player(self) -> bool {
        self.index().is_some()
    }
}

/// Card tags (021). A card carries 1-3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tag {
    Science,
    Industry,
    Cyber,
    Power,
    Diplomacy,
    Security,
}

/// Every tag, in printed order.
pub const TAGS: [Tag; 6] = [
    Tag::Science,
    Tag::Industry,
    Tag::Cyber,
    Tag::Power,
    Tag::Diplomacy,
    Tag::Security,
];

impl Tag {
    pub fn name(self) -> &'static str {
        match self {
            Tag::Science => "science",
            Tag::Industry => "industry",
            Tag::Cyber => "cyber",
            Tag::Power => "power",
            Tag::Diplomacy => "diplomacy",
            Tag::Security => "security",
        }
    }
}

/// Token types (021): tokens live only on cards that print their type.
/// Datasets (Science), Followers (Diplomacy), Capital (Industry), Agents
/// (Security); a Cyber type is still to come.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenKind {
    Datasets,
    Followers,
    Capital,
    Agents,
}

impl TokenKind {
    pub fn name(self) -> &'static str {
        match self {
            TokenKind::Datasets => "datasets",
            TokenKind::Followers => "followers",
            TokenKind::Capital => "capital",
            TokenKind::Agents => "agents",
        }
    }
}

/// A side of the Alignment track: Unity (the good AIs) or Discord (the evil AI).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Unity,
    Discord,
}

impl Side {
    pub fn other(self) -> Side {
        match self {
            Side::Unity => Side::Discord,
            Side::Discord => Side::Unity,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Side::Unity => "unity",
            Side::Discord => "discord",
        }
    }
}

macro_rules! display_by_name {
    ($($t:ty),*) => {$(
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.name())
            }
        }
    )*};
}
display_by_name!(Resource, Tag, TokenKind, Side);

/// A tag cost on a power, or a `tags_from_hand` filter: a tag, or `"any"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum TagOrAny {
    Any,
    Is(Tag),
}

impl TagOrAny {
    /// Whether a card tag `t` answers this.
    pub fn accepts(self, t: Tag) -> bool {
        self == TagOrAny::Any || self == TagOrAny::Is(t)
    }
}

impl fmt::Display for TagOrAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagOrAny::Any => f.write_str("any"),
            TagOrAny::Is(t) => t.fmt(f),
        }
    }
}

impl TryFrom<String> for TagOrAny {
    type Error = String;
    fn try_from(s: String) -> Result<TagOrAny, String> {
        if s == "any" {
            return Ok(TagOrAny::Any);
        }
        TAGS.iter()
            .find(|t| t.name() == s)
            .map(|t| TagOrAny::Is(*t))
            .ok_or_else(|| format!("unknown tag {s:?}"))
    }
}

impl From<TagOrAny> for String {
    fn from(t: TagOrAny) -> String {
        t.to_string()
    }
}

/// What an AI board's resource discount applies to: cards with a tag, or
/// any building (`"building"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum DiscountOn {
    Building,
    Tag(Tag),
}

impl fmt::Display for DiscountOn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscountOn::Building => f.write_str("building"),
            DiscountOn::Tag(t) => t.fmt(f),
        }
    }
}

impl TryFrom<String> for DiscountOn {
    type Error = String;
    fn try_from(s: String) -> Result<DiscountOn, String> {
        if s == "building" {
            return Ok(DiscountOn::Building);
        }
        TAGS.iter()
            .find(|t| t.name() == s)
            .map(|t| DiscountOn::Tag(*t))
            .ok_or_else(|| format!("unknown discount target {s:?}"))
    }
}

impl From<DiscountOn> for String {
    fn from(t: DiscountOn) -> String {
        t.to_string()
    }
}

/// One amount per player resource (incomes), indexed by [`Resource`]. A
/// resource that is not a player resource reads 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PerResource([i64; RESOURCES.len()]);

impl PerResource {
    /// Adds to a player resource (anything else has no entry and is ignored).
    pub fn add(&mut self, r: Resource, n: i64) {
        if let Some(i) = r.index() {
            self.0[i] += n;
        }
    }

    /// `(resource, amount)` in display order.
    pub fn iter(&self) -> impl Iterator<Item = (Resource, i64)> + '_ {
        RESOURCES.iter().copied().zip(self.0.iter().copied())
    }
}

impl std::ops::Index<Resource> for PerResource {
    type Output = i64;
    fn index(&self, r: Resource) -> &i64 {
        r.index().map_or(&0, |i| &self.0[i])
    }
}

/// Setting an entry; panics for what is not a player resource.
impl std::ops::IndexMut<Resource> for PerResource {
    fn index_mut(&mut self, r: Resource) -> &mut i64 {
        &mut self.0[r.index().expect("a player resource")]
    }
}

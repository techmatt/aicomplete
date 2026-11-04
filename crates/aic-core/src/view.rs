//! The generic view: what a client draws, for any set of modules.
//!
//! A client should not need code for each module. [`Rules::view`] turns a
//! state into a fixed set of shapes (hexes with tokens, per-seat and
//! per-country values, card groups, card piles, global values)
//! and every module may add to them through [`Module::view`]. The engine
//! fills in the generic parts itself, so a new module's tokens, resources
//! and tracks show up without any view code; a module adds a `view` hook
//! only to show something derived (a threshold, a projection, card status),
//! a player board ([`ViewOut::board`]), or to keep its per-seat and
//! per-card bookkeeping `vars` out of the game-wide globals
//! ([`ViewOut::bookkeeping`]).
//!
//! The view is display data only: nothing here is read back by the rules,
//! and it is not part of the state, its hash or its replay.
//!
//! [`Module::view`]: crate::module::Module::view

use serde::Serialize;

use crate::content::Content;
use crate::hex::Hex;
use crate::key::Key;
use crate::state::{GameState, Outcome};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct View {
    pub round: u32,
    /// The phase waiting for decisions, or `None` once the game is over.
    pub phase: Option<PhaseView>,
    /// The round's phase sequence (`config.round`), with each entry's kind.
    pub round_order: Vec<PhaseInfo>,
    pub outcome: Option<Outcome>,
    /// Every hex, aligned with `Map::hexes` and `GameState::hexes`.
    pub hexes: Vec<HexView>,
    pub countries: Vec<CountryView>,
    pub seats: Vec<SeatView>,
    /// Values that belong to the whole game: every `vars` track that no
    /// module declares bookkeeping, plus what modules add.
    pub globals: Vec<Value>,
    /// Ordered card piles (decks, discards, queues) modules show.
    pub piles: Vec<PileView>,
    /// The standard actions modules publish, with their costs, so clients
    /// label them without knowing the parameters.
    pub standard: Vec<StandardView>,
    /// The open choice, if the rules are waiting for one.
    pub choice: Option<ChoiceView>,
    /// Where pieces are heading (019: each adversary troop's next target),
    /// so clients draw it without knowing the rules.
    pub arrows: Vec<ArrowView>,
    /// Each face-up threat's source type and pieces (070), for the map.
    pub threat_sources: Vec<ThreatSourceView>,
}

/// What feeds a face-up threat, and who may counter it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ThreatSourceView {
    pub threat: String,
    /// `influence`, `cyber`, `economic` or `military`.
    pub source: String,
    /// The hexes of its sources.
    pub hexes: Vec<usize>,
    /// The hex it named with no source: it becomes one if it goes uncountered.
    pub target: Option<usize>,
    /// The seats that may pay toward it.
    pub seats: Vec<usize>,
}

/// A piece's heading: from its hex to the hex it targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArrowView {
    /// What is heading there (`troop`).
    pub kind: String,
    pub from: usize,
    pub to: usize,
    /// How many pieces on `from` share it.
    pub count: i64,
    /// The rounds until it arrives (1: this coming one).
    pub rounds: i64,
    pub doc: String,
}

/// A standard action (`Action::Standard { act }`) and its cost.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StandardView {
    pub act: String,
    pub energy: i64,
    pub credits: i64,
    pub chips: i64,
    pub ambassadors: i64,
    /// What it does and costs, in words.
    pub doc: String,
    /// 099: chips each seat pays where its cards set the price (the Harden
    /// action a card grants), by seat; empty when the printed price stands.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub seat_chips: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChoiceView {
    pub seat: u8,
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PhaseInfo {
    /// The round entry, e.g. `"board.act"` or `"cards.play+board.act"`.
    pub name: String,
    /// `"auto"` or `"decision"`.
    pub kind: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PhaseView {
    /// Index into `round_order`.
    pub index: usize,
    pub name: String,
    pub kind: &'static str,
    /// Seats that have not passed yet.
    pub waiting: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HexView {
    pub index: usize,
    pub hex: Hex,
    pub country: u8,
    pub piece_pos: [u8; 2],
    /// Indices of on-board neighbors.
    pub neighbors: Vec<usize>,
    /// The printed hex number (1-16 on its piece), for tie-breaks.
    pub number: u8,
    /// What the hex is: `city`, `base`, `open` or `ocean`.
    pub kind: &'static str,
    /// For a military base, the country across its front.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facing: Option<u8>,
    /// A printed industrial zone (where cities build factories).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub industrial: bool,
    /// Non-zero token stacks, by kind (sorted).
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Token {
    pub kind: String,
    pub count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CountryView {
    pub index: u8,
    /// The printed country piece's name (`content/map.json` `countries`;
    /// country `c` is piece `c`).
    pub name: String,
    /// The seat that plays this country (one seat per country).
    pub seat: u8,
    /// Display color from `content/map.json`.
    pub color: String,
    pub hexes: Vec<usize>,
    /// Token totals over the country's hexes (`tokens.<kind>`), then what
    /// modules add.
    pub values: Vec<Value>,
    /// Card groups modules show on this country (threats, ...).
    pub cards: Vec<CardGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SeatView {
    pub index: u8,
    pub country: u8,
    /// Passed in the current decision phase.
    pub passed: bool,
    /// Every resource (`resource.<name>`), then what modules add.
    pub values: Vec<Value>,
    /// Card groups modules show for this seat. Exhausted cards that no
    /// module shows land in a generic `engine.exhausted` group.
    pub cards: Vec<CardGroup>,
    /// The player board this seat plays, if a module shows one.
    pub board: Option<BoardView>,
}

/// A player board: a printed sheet with a name, rules text, tracks and
/// slots. Which board and what its tracks mean is the module's business;
/// the shape is the same for any board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BoardView {
    /// `"<module>.<key>"`.
    pub key: String,
    pub source: String,
    /// Content id of the board.
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
    /// A standing rule printed on the board, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub innate: Option<InnateView>,
    /// Tags printed on the board: they count for what a card requires (068).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub tracks: Vec<TrackView>,
    /// Slots printed on the board (what they hold is the module's), if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<i64>,
    /// The board's powers as cards, with their status this round.
    pub cards: Vec<CardView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InnateView {
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
}

/// A printed track: a row of steps, the leftmost `uncovered` of them
/// showing, the rest still covered by pieces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TrackView {
    pub label: String,
    pub steps: Vec<TrackStep>,
    /// How many steps, from the left, are uncovered.
    pub uncovered: usize,
    /// Index of the step in effect.
    pub current: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TrackStep {
    pub value: i64,
    /// Extra things printed on the step. Clients may style known keys.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub marks: Vec<TrackMark>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TrackMark {
    /// A short stable word chosen by the module (`"tier2"`, `"antimatter"`).
    pub key: String,
    /// What to print (`"T2"`, `"+1 AM"`).
    pub label: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub doc: String,
}

/// One number worth showing, with where it came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Value {
    /// Stable id: `resource.<name>`, `tokens.<kind>`, a `vars` key, or
    /// `"<module>.<name>"` for a module's own value.
    pub key: String,
    pub label: String,
    pub value: i64,
    /// A limit the value is measured against (a threshold, a round count).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    /// `"engine"` for the generic parts, otherwise the module id.
    pub source: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub doc: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CardGroup {
    pub key: String,
    pub label: String,
    pub source: String,
    pub cards: Vec<CardView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CardView {
    /// Content card id.
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
    /// A short word for its state, chosen by the module (`"ready"`,
    /// `"used"`, ...). Clients may style known words and show any other.
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PileView {
    pub key: String,
    pub label: String,
    pub source: String,
    /// The seat it belongs to, or `None` for a shared pile.
    pub owner: Option<u8>,
    pub count: usize,
    /// Cards a player can see, top first (all of a face-up pile, none of a
    /// face-down one).
    pub visible: Vec<CardView>,
}

/// What one module adds to the view. Keys are namespaced
/// `"<module>.<key>"` automatically.
pub struct ViewOut<'a> {
    pub(crate) module: &'static str,
    pub(crate) view: &'a mut View,
}

impl ViewOut<'_> {
    fn group(&self, key: &str, label: &str, cards: Vec<CardView>) -> CardGroup {
        CardGroup {
            key: format!("{}.{key}", self.module),
            label: label.to_string(),
            source: self.module.to_string(),
            cards,
        }
    }

    fn value(&self, key: &str, label: &str, value: i64, max: Option<i64>, doc: &str) -> Value {
        Value {
            key: format!("{}.{key}", self.module),
            label: label.to_string(),
            value,
            max,
            source: self.module.to_string(),
            doc: doc.to_string(),
        }
    }

    /// A face-up threat's sources, for the map (070).
    pub fn threat_source(&mut self, t: ThreatSourceView) {
        self.view.threat_sources.push(t);
    }

    /// Publishes a piece's heading (an arrow from hex `from` to hex `to`).
    pub fn arrow(&mut self, kind: &str, from: usize, to: usize, count: i64, rounds: i64, doc: &str) {
        self.view.arrows.push(ArrowView {
            kind: kind.to_string(),
            from,
            to,
            count,
            rounds,
            doc: doc.to_string(),
        });
    }

    /// Publishes a standard action and its cost.
    pub fn standard(&mut self, act: &str, energy: i64, credits: i64, chips: i64, ambassadors: i64, doc: &str) {
        self.view.standard.push(StandardView {
            act: act.to_string(),
            energy,
            credits,
            chips,
            ambassadors,
            doc: doc.to_string(),
            seat_chips: Vec::new(),
        });
    }

    /// 099: the chips each seat pays for the last standard action added.
    pub fn standard_seat_chips(&mut self, chips: Vec<i64>) {
        if let Some(s) = self.view.standard.last_mut() {
            s.seat_chips = chips;
        }
    }

    /// A value of the whole game. One with the key of a `vars` track
    /// replaces that track's generic entry.
    pub fn global(&mut self, key: &str, label: &str, value: i64, max: Option<i64>, doc: &str) {
        let v = self.value(key, label, value, max, doc);
        upsert(&mut self.view.globals, v);
    }

    pub fn country(
        &mut self,
        country: usize,
        key: &str,
        label: &str,
        value: i64,
        max: Option<i64>,
        doc: &str,
    ) {
        let v = self.value(key, label, value, max, doc);
        upsert(&mut self.view.countries[country].values, v);
    }

    pub fn seat(
        &mut self,
        seat: usize,
        key: &str,
        label: &str,
        value: i64,
        max: Option<i64>,
        doc: &str,
    ) {
        let v = self.value(key, label, value, max, doc);
        upsert(&mut self.view.seats[seat].values, v);
    }

    /// A card group on a country (say, the threat on it).
    pub fn country_cards(&mut self, country: usize, key: &str, label: &str, cards: Vec<CardView>) {
        let group = self.group(key, label, cards);
        self.view.countries[country].cards.push(group);
    }

    pub fn cards(&mut self, seat: usize, key: &str, label: &str, cards: Vec<CardView>) {
        let group = self.group(key, label, cards);
        self.view.seats[seat].cards.push(group);
    }

    /// The player board seat `seat` plays. `board.key` is namespaced and
    /// `board.source` set here.
    pub fn board(&mut self, seat: usize, key: &str, mut board: BoardView) {
        board.key = format!("{}.{key}", self.module);
        board.source = self.module.to_string();
        self.view.seats[seat].board = Some(board);
    }

    /// Declares `vars` keys that are the module's bookkeeping rather than
    /// tracks of the whole game: per-seat or per-card markers, flags
    /// already shown elsewhere. A prefix ending in `.` covers every key
    /// under it; any other entry is one exact key. Their generic `globals`
    /// entries are dropped; the module shows what a player needs of them
    /// where it belongs (a seat value, a card's text). A value the module
    /// itself adds under such a key stays.
    pub fn bookkeeping(&mut self, keys: &[&str]) {
        let claimed = |k: &str| {
            keys.iter().any(|p| {
                if p.ends_with('.') {
                    k.starts_with(p)
                } else {
                    k == *p
                }
            })
        };
        self.view
            .globals
            .retain(|v| v.source != "engine" || !claimed(&v.key));
    }

    pub fn pile(
        &mut self,
        key: &str,
        label: &str,
        owner: Option<u8>,
        count: usize,
        visible: Vec<CardView>,
    ) {
        self.view.piles.push(PileView {
            key: format!("{}.{key}", self.module),
            label: label.to_string(),
            source: self.module.to_string(),
            owner,
            count,
            visible,
        });
    }
}

/// A token kind in words, for a label: `node:0` is "seat 0's node",
/// `ambassador:adversary` "adversary ambassador" (020: labels were raw keys).
fn token_words(kind: &str) -> String {
    match kind.split_once(':') {
        Some((what, "adversary")) => format!("adversary {}", what.replace('_', " ")),
        Some((what, seat)) => format!("seat {seat}'s {}", what.replace('_', " ")),
        None => kind.replace('_', " "),
    }
}

fn upsert(values: &mut Vec<Value>, v: Value) {
    match values.iter_mut().find(|x| x.key == v.key) {
        Some(x) => *x = v,
        None => values.push(v),
    }
}

/// The parts of the view the engine fills in for every config.
pub(crate) fn generic(rules: &crate::Rules, g: &GameState) -> View {
    let content = rules.world().content;
    let engine = |key: String, label: &str, value: i64, doc: &str| Value {
        key,
        label: label.to_string(),
        value,
        max: None,
        source: "engine".to_string(),
        doc: doc.to_string(),
    };
    let hexes: Vec<HexView> = rules
        .map
        .hexes
        .iter()
        .zip(g.hexes.iter())
        .enumerate()
        .map(|(i, (cell, h))| HexView {
            index: i,
            hex: cell.hex,
            country: cell.country,
            piece_pos: cell.piece_pos,
            neighbors: cell.neighbors.clone(),
            number: cell.number,
            kind: cell.kind.name(),
            facing: cell.facing,
            industrial: cell.industrial,
            tokens: h
                .tokens
                .iter()
                .map(|(k, &n)| Token {
                    kind: k.to_string(),
                    count: n,
                })
                .collect(),
        })
        .collect();
    let kinds: std::collections::BTreeSet<&str> = g
        .hexes
        .iter()
        .flat_map(|h| h.tokens.keys().map(Key::as_str))
        .collect();
    let countries = rules
        .map
        .countries
        .iter()
        .enumerate()
        .map(|(c, geo)| {
            // Every kind on the board, so a clean country shows 0 rather than nothing.
            let mut totals: std::collections::BTreeMap<&str, i64> =
                kinds.iter().map(|&k| (k, 0)).collect();
            for &h in &geo.hexes {
                for (k, &n) in &g.hexes[h].tokens {
                    *totals.entry(k.as_str()).or_insert(0) += n;
                }
            }
            CountryView {
                index: c as u8,
                name: content.map.countries[c].name.clone(),
                seat: c as u8,
                color: geo.color.clone(),
                hexes: geo.hexes.clone(),
                values: totals
                    .into_iter()
                    .map(|(k, n)| {
                        engine(
                            format!("tokens.{k}"),
                            &token_words(k),
                            n,
                            "tokens on this country's hexes",
                        )
                    })
                    .collect(),
                cards: Vec::new(),
            }
        })
        .collect();
    let seats = g
        .seats
        .iter()
        .enumerate()
        .map(|(s, seat)| SeatView {
            index: s as u8,
            country: s as u8,
            passed: seat.passed,
            values: seat
                .resources
                .iter()
                .map(|(k, &n)| {
                    engine(
                        format!("resource.{k}"),
                        &k.name().replace('_', " "),
                        n,
                        "on this seat's board",
                    )
                })
                .collect(),
            cards: Vec::new(),
            board: None,
        })
        .collect();
    let globals = g
        .vars
        .iter()
        .map(|(k, &n)| engine(k.to_string(), k, n, "a module track (vars)"))
        .collect();
    let phase = rules.phase(g).map(|p| PhaseView {
        index: g.phase,
        name: p.name.clone(),
        kind: p.kind.name(),
        waiting: g
            .seats
            .iter()
            .enumerate()
            .filter(|(_, s)| !s.passed)
            .map(|(i, _)| i as u8)
            .collect(),
    });
    View {
        round: g.round,
        phase,
        round_order: rules
            .round
            .iter()
            .map(|p| PhaseInfo {
                name: p.name.clone(),
                kind: p.kind.name(),
            })
            .collect(),
        outcome: g.outcome.clone(),
        hexes,
        countries,
        seats,
        globals,
        piles: Vec::new(),
        standard: Vec::new(),
        arrows: Vec::new(),
        threat_sources: Vec::new(),
        choice: g.pending.as_ref().map(|p| ChoiceView {
            seat: p.ask.seat,
            prompt: p.ask.prompt.clone(),
        }),
    }
}

/// After the modules: exhausted cards that no module's card group shows go
/// into a generic group, so nothing in the state goes unshown.
pub(crate) fn finish(view: &mut View, g: &GameState, content: &Content) {
    for (s, seat) in g.seats.iter().enumerate() {
        let sv = &mut view.seats[s];
        let shown = |id: &str| {
            sv.cards
                .iter()
                .any(|grp| grp.cards.iter().any(|c| c.id == id))
        };
        let rest: Vec<CardView> = seat
            .exhausted
            .iter()
            .filter(|id| !shown(id))
            .map(|id| CardView {
                id: id.to_string(),
                name: content.card_index(id).map_or(id.as_str(), |i| &content.cards[i].name).to_string(),
                text: String::new(),
                status: "used".to_string(),
                cost: None,
            })
            .collect();
        if !rest.is_empty() {
            sv.cards.push(CardGroup {
                key: "engine.exhausted".to_string(),
                label: "exhausted".to_string(),
                source: "engine".to_string(),
                cards: rest,
            });
        }
    }
}

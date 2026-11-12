//! A player card's printed face, generated from its ops (033): the icon lines
//! and the rules text come out of one walk, so the text mirrors the icons in
//! order and a card can't say one thing and play another. House wording and
//! what no face may print are in docs/card-guidelines.md.
//!
//! **Where it sits is what it is.** Above the art (`top`, buildings only): what
//! lasts while the building is active. Below the art (`effect`): what happens
//! once, when played, extra costs first. The grammar is in
//! docs/rules/card-grammar.md; `web/card/card.js` draws the parts, and `web/card/faces.json`
//! is this module's output for it, kept current by `aic-sim docs`.
//!
//! A new one-shot op fails to compile here until it has its face: that match
//! has no wildcard. `standing` has one, so a new standing op panics at content
//! load until it has a face. Ops no card can print ([`Op::slots`], and `Wall`,
//! which only an innate may hold) panic: `Content::check` rejects them first.

use serde::Serialize;

use crate::content::{Ability, CardDef, Event, Facility, Op, Resource, Tag, TokenKind};

/// One part of an icon line: an icon or a connector (`→ / : OR ALL ACT`), a
/// tag (`#science`), a number, or a box of parts.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Part {
    Word(String),
    Income { income: Vec<Part> },
    Adversary { adversary: Vec<Part> },
    /// Dashed: another player's.
    Other { other: Vec<Part> },
    /// Thin solid: counted across your cards.
    Yours { yours: Vec<Part> },
}

/// What a card prints: icon lines, rules text, above and below the art.
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Face {
    pub top: Vec<Vec<Part>>,
    pub top_text: String,
    pub effect: Vec<Vec<Part>>,
    pub text: String,
}

impl Face {
    /// All of the card's rules text, in print order.
    pub fn full_text(&self) -> String {
        [self.top_text.as_str(), self.text.as_str()]
            .iter()
            .filter(|s| !s.is_empty())
            .copied()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// What a clause gives, when the next clause can share its sentence.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Share {
    /// A plain gain of yours.
    Mine,
    /// A gain of the chosen player's.
    Theirs,
    No,
}

/// One clause: its icon line and its words, without the capital or the stop.
#[derive(Debug, Clone)]
struct Clause {
    icons: Vec<Part>,
    say: String,
    share: Share,
}

fn w(s: &str) -> Part {
    Part::Word(s.to_string())
}

fn clause(icons: Vec<Part>, say: impl Into<String>) -> Clause {
    Clause { icons, say: say.into(), share: Share::No }
}

fn adversary(parts: Vec<Part>) -> Vec<Part> {
    vec![Part::Adversary { adversary: parts }]
}

/// A number goes before its icon; no number means 1.
fn counted(n: i64, icon: &str) -> Vec<Part> {
    if n == 1 {
        vec![w(icon)]
    } else {
        vec![Part::Word(n.to_string()), w(icon)]
    }
}

/// A negative always has its minus sign, 1 too. 179 (Matt, friction #34): the minus is pay or lose, and only
/// that: a cost paid anywhere (below the art, left of an Action's arrow) prints it, a discount never does.
fn minus(n: i64, icon: &str) -> Vec<Part> {
    vec![Part::Word(format!("\u{2212}{n}")), w(icon)]
}

/// 179 (friction #34): "less", a discount or a jam: its own mark (`less:N`), never the minus.
fn less(n: i64, icon: &str) -> Vec<Part> {
    vec![Part::Word(format!("less:{n}")), w(icon)]
}

/// 179 (friction #38): At N, when the tokens here reach N: the number in a ring (`at:N`), then the disc.
fn at(n: i64, icon: &str) -> Vec<Part> {
    vec![Part::Word(format!("at:{n}")), w(icon)]
}

/// 179 (friction #36): below the art, the brown box moves an income track once: a track arrow, up or down,
/// before the amount (no sign: the minus is pay or lose).
fn track_move(n: i64, rest: Vec<Part>) -> Vec<Part> {
    let mut p = vec![w(if n < 0 { "lower" } else { "raise" })];
    if n.abs() != 1 {
        p.push(Part::Word(n.abs().to_string()));
    }
    p.extend(rest);
    vec![Part::Income { income: p }]
}

/// 166: a Gather's or Push's pieces as icons: ambassadors bare, agents and troops in the adversary's frame.
/// 179 (Matt, friction #33): an agent prints its own figure, the map's.
fn piece_icons(pieces: &[crate::op::Piece]) -> Vec<Part> {
    use crate::op::Piece;
    let mut out = Vec::new();
    let mut adv = Vec::new();
    for p in pieces {
        match p {
            Piece::Ambassadors => out.push(w("ambassador")),
            Piece::Agents => adv.push(w("agent")),
            Piece::Troops => adv.push(w("troop")),
        }
    }
    if !adv.is_empty() {
        out.extend(adversary(adv));
    }
    out
}

/// 166: a Gather's or Push's pieces in words, in the icons' order: "agents or troops".
fn piece_words(pieces: &[crate::op::Piece]) -> String {
    pieces.iter().map(|p| p.word()).collect::<Vec<_>>().join(" or ")
}

fn icon(r: Resource) -> &'static str {
    match r {
        Resource::Ambassadors => "ambassador",
        Resource::Antimatter => "antimatter",
        Resource::Chips => "chips",
        Resource::Credits => "credits",
        Resource::Energy => "energy",
        Resource::Insight => "insight",
        Resource::ResearchCubes => "research_cubes",
    }
}

/// `r` as a noun for `n`, without the number.
fn noun(r: Resource, n: i64) -> &'static str {
    match (r, n) {
        (Resource::Credits, 1) => "credit",
        (Resource::Chips, 1) => "chip",
        (Resource::Ambassadors, 1) => "ambassador",
        (Resource::Credits, _) => "credits",
        (Resource::Chips, _) => "chips",
        (Resource::Ambassadors, _) => "ambassadors",
        (Resource::Antimatter, _) => "antimatter",
        (Resource::Energy, _) => "energy",
        (Resource::Insight, _) => "insight",
        (Resource::ResearchCubes, 1) => "research cube",
        (Resource::ResearchCubes, _) => "research cubes",
    }
}

fn amount(r: Resource, n: i64) -> String {
    format!("{n} {}", noun(r, n))
}

/// The word for a track: "credit income", "energy income".
fn track(r: Resource) -> &'static str {
    noun(r, 1)
}

fn token(kind: TokenKind, n: i64) -> &'static str {
    match (kind, n) {
        (TokenKind::Datasets, 1) => "Dataset",
        (TokenKind::Followers, 1) => "Follower",
        // 166 (Matt, 2026-11-09): printed Operatives, so "agents" are only the adversary's pieces (code name stays).
        (TokenKind::Agents, 1) => "Operative",
        (TokenKind::Datasets, _) => "Datasets",
        (TokenKind::Followers, _) => "Followers",
        (TokenKind::Agents, _) => "Operatives",
        (TokenKind::Capital, _) => "Capital",
    }
}

/// A lowercase word with its capital.
fn title(word: &str) -> String {
    word[..1].to_uppercase() + &word[1..]
}

fn tag(t: Tag) -> String {
    title(t.name())
}

fn hash(t: Tag) -> Part {
    Part::Word(format!("#{}", t.name()))
}

/// "per 2 Agents" and "per Agent": no number means 1, in words too.
fn per_tokens(kind: TokenKind, per: i64) -> String {
    if per == 1 {
        format!("per {}", token(kind, 1))
    } else {
        format!("per {per} {}", token(kind, per))
    }
}

fn hexes(n: i64) -> String {
    if n == 1 {
        "1 hex".to_string()
    } else {
        format!("{n} hexes")
    }
}

fn plural(n: i64, one: &str) -> String {
    format!("{n} {one}{}", if n == 1 { "" } else { "s" })
}

/// A removal's pieces: "2 agents or 1 troop".
fn pieces(ambassadors: i64, troops: i64) -> String {
    match (ambassadors, troops) {
        (0, t) => plural(t, "troop"),
        (a, 0) => plural(a, "agent"),
        (a, t) => format!("{} or {}", plural(a, "agent"), plural(t, "troop")),
    }
}

/// A removal's icons: the choice of pieces, in the dark frame.
fn removal_icons(ambassadors: i64, troops: i64) -> Vec<Part> {
    let mut parts = Vec::new();
    if ambassadors > 0 {
        parts.extend(minus(ambassadors, "agent"));
    }
    if ambassadors > 0 && troops > 0 {
        parts.push(w("OR"));
    }
    if troops > 0 {
        parts.extend(minus(troops, "troop"));
    }
    parts
}

/// The clauses of an op list. `gain` is the verb of a plain gain: "gain" below
/// the art, "get" in an Action (call 2, the arrow's right side).
fn clauses(ops: &[Op], gain: &str) -> Vec<Clause> {
    let mut out: Vec<Clause> = Vec::new();
    for op in ops {
        for c in once(op, gain) {
            // Gains in a row are one sentence, as they are one row of icons.
            match out.last_mut() {
                Some(last) if last.share == c.share && c.share != Share::No => {
                    last.icons.extend(c.icons);
                    let word = if c.share == Share::Theirs { "another player gains" } else { gain };
                    let tail = c.say.trim_start_matches(word).trim_start();
                    // "a and b", then "a, b and c".
                    let head = last.say.replace(" and ", ", ");
                    last.say = format!("{head} and {tail}");
                }
                _ => out.push(c),
            }
        }
    }
    // A box of a merged run: the dashed outline holds all of the other's gain.
    for c in &mut out {
        if c.share == Share::Theirs {
            let parts = std::mem::take(&mut c.icons);
            c.icons = vec![Part::Other { other: parts }];
        }
    }
    out
}

/// The clauses one op prints when it resolves.
fn once(op: &Op, gain: &str) -> Vec<Clause> {
    let one = |c: Clause| vec![c];
    match op {
        Op::Gain { resource, amount: n } => one(Clause {
            icons: counted(*n, icon(*resource)),
            say: format!("{gain} {}", amount(*resource, *n)),
            share: Share::Mine,
        }),
        Op::GainAmbassadors { count } => one(Clause {
            icons: counted(*count, "ambassador"),
            say: format!("{gain} {}", amount(Resource::Ambassadors, *count)),
            share: Share::Mine,
        }),
        Op::GainPerNode { resource } => one(clause(
            vec![w(icon(*resource)), w("/"), w("node")],
            format!("{gain} 1 {} per node you have on the board", noun(*resource, 1)),
        )),
        // It only names who the `other_gain` or `transfer` after it reaches.
        Op::ChoosePlayer => Vec::new(),
        Op::OtherGain { resource, amount: n } => one(Clause {
            icons: counted(*n, icon(*resource)),
            say: format!("another player gains {}", amount(*resource, *n)),
            share: Share::Theirs,
        }),
        Op::Transfer { resource, max } => one(clause(
            {
                // ≤: the giver chooses how much, up to the number, on both sides.
                let given = vec![Part::Word(format!("\u{2264}{max}")), w(icon(*resource))];
                let mut p = vec![Part::Word(format!("\u{2212}\u{2264}{max}")), w(icon(*resource))];
                p.push(Part::Other { other: given });
                p
            },
            format!("give another player up to {}", amount(*resource, *max)),
        )),
        Op::PlaceNode { count } => one(clause(
            counted(*count, "node"),
            format!("place {}", plural(*count, "node")),
        )),
        Op::ClearMarkers { count } => one(clause(
            adversary(minus(*count, "marker")),
            format!("clear {} in reach", plural(*count, "corruption marker")),
        )),
        Op::ShieldNode { count } => one(clause(
            { let mut p = counted(*count, "harden"); p.push(w("node")); p },
            format!("harden {count} of your nodes"),
        )),
        Op::DelayThreat { count } => one(clause(
            adversary(counted(*count, "clock")),
            format!("delay {} one round", plural(*count, "threat")),
        )),
        Op::Damage { ambassadors, troops } => {
            let mut parts = removal_icons(*ambassadors, *troops);
            parts.push(w("hex"));
            // One piece names its hex; a choice of pieces all come from one.
            let place = if ambassadors + troops == 1 { "in reach" } else { "from a hex in reach" };
            one(clause(adversary(parts), format!("remove {} {place}", pieces(*ambassadors, *troops))))
        }
        Op::DamageCountry { ambassadors, troops } => {
            let mut parts = removal_icons(*ambassadors, *troops);
            parts.extend([w("/"), w("hex")]);
            let mut icons = adversary(parts);
            icons.push(w("country"));
            one(clause(icons, format!("remove {} per hex of a country", pieces(*ambassadors, *troops))))
        }
        Op::DestroyAll => one(clause(
            adversary(vec![w("ALL"), w("agent"), w("troop"), w("hex")]),
            "remove every agent and troop in one hex",
        )),
        Op::RemoveBotnet => one(clause(adversary(minus(1, "botnet")), "remove a botnet in reach")),
        Op::RemoveAmbassadorsCountry { unity_per, max, reach } => {
            // 111: "in reach" prints no place, as a botnet or a marker in reach.
            let (mut icons, what) = if *max > 0 && *reach {
                (adversary(minus(*max, "agent")), format!("remove {} in reach", plural(*max, "agent")))
            } else if *max > 0 {
                let mut p = minus(*max, "agent");
                p.push(w("country"));
                (adversary(p), format!("remove {} in one country", plural(*max, "agent")))
            } else {
                (
                    adversary(vec![w("ALL"), w("agent"), w("country")]),
                    "remove every agent in one country".to_string(),
                )
            };
            if *unity_per == 0 {
                return one(clause(icons, what));
            }
            icons.extend([w("sway"), w("/")]);
            icons.extend(adversary(vec![Part::Word(unity_per.to_string()), w("agent")]));
            one(clause(icons, format!("{what}, and sway 1 toward Unity per {unity_per} removed")))
        }
        Op::RemoveTroops { count, reach } => {
            let mut parts = minus(*count, "troop");
            if *reach {
                return one(clause(adversary(parts), format!("remove {} in reach", plural(*count, "troop"))));
            }
            parts.push(w("country"));
            one(clause(adversary(parts), format!("remove {} in one country", plural(*count, "troop"))))
        }
        Op::GainPerTag { resource, tag: t, per } => {
            let mut icons = vec![w(icon(*resource)), w("/")];
            if *per != 1 {
                icons.push(Part::Word(per.to_string()));
            }
            icons.push(hash(*t));
            let tags = if *per == 1 { format!("{} tag", tag(*t)) } else { format!("{per} {} tags", tag(*t)) };
            one(clause(icons, format!("{gain} 1 {} per {tags} on your cards", noun(*resource, 1))))
        }
        Op::BuildFactory => one(clause(vec![w("factory")], "build a factory in reach")),
        Op::Cleanse { amount: n, one_country, cities, only } => {
            let what = match (*cities, only, *n) {
                (true, _, _) => "corrupted cities, stations or factories",
                (false, Some(Facility::Station), 1) => "corrupted station",
                (false, Some(Facility::Station), _) => "corrupted stations",
                (false, Some(Facility::Factory), 1) => "corrupted factory",
                (false, Some(Facility::Factory), _) => "corrupted factories",
                (false, None, 1) => "corrupted station or factory",
                (false, None, _) => "corrupted stations or factories",
            };
            let place = if *one_country { "in reach, in one country" } else { "in reach" };
            // 191 (184's D80): a city is liberated, a station or factory freed, so the city form prints both.
            let verb = if *cities { "liberate or free" } else { "free" };
            let say = format!("{verb} {n} {what} {place}");
            let mut parts = minus(*n, "cube");
            // D37: a narrowed cube names its facility, so the bare cube keeps one meaning.
            match (*cities, only) {
                (false, Some(Facility::Station)) => parts.push(w("station")),
                (false, Some(Facility::Factory)) => parts.push(w("factory")),
                // D48 (r48): a cleanse that frees cities shows the city it frees (Great Liberation).
                (true, _) => parts.push(w("city")),
                _ => {}
            }
            if *one_country {
                parts.push(w("country"));
            }
            one(clause(adversary(parts), say))
        }
        Op::Liberate { ambassadors, troops, botnet } => {
            let mut parts = minus(1, "cube");
            parts.push(w("city"));
            // 178 (#29): every liberation is the Liberate action's, so the house term carries it (the Agents on
            // it come off, your cube goes on); the align mark (D48) would say it on every card, so none prints it.
            let say = "liberate a corrupted city in reach".to_string();
            let mut out = vec![clause(adversary(parts), say)];
            if *ambassadors > 0 {
                let n = *ambassadors;
                let noun = if n == 1 { "ambassador" } else { "ambassadors" };
                out.push(clause(counted(n, "ambassador"), format!("gain {n} {noun}")));
            }
            if *troops > 0 {
                let mut p = minus(*troops, "troop");
                p.push(w("country"));
                out.push(clause(adversary(p), format!("remove up to {troops} troops in its country")));
            }
            if *botnet {
                // D37: on or next to the city, as Clear a city's city glyph says.
                let mut p = minus(1, "botnet");
                p.push(w("city"));
                out.push(clause(adversary(p), "remove a botnet on or next to it".to_string()));
            }
            out
        }
        Op::ClearCity { max } => {
            let (parts, what) = if *max > 0 {
                let mut p = minus(*max, "agent");
                p.push(w("city"));
                (p, format!("{max} agents"))
            } else {
                (vec![w("ALL"), w("agent"), w("city")], "every agent".to_string())
            };
            one(clause(adversary(parts), format!("remove {what} on or next to one corrupted city in reach")))
        }
        Op::Harden { count } => {
            let icons = counted(*count, "harden");
            let what = if *count == 1 { "1 facility".to_string() } else { format!("{count} facilities") };
            one(clause(icons, format!("harden {what} in reach")))
        }
        Op::MoveAmbassadors { max } => one(clause(
            vec![Part::Word(max.to_string()), w("move")],
            format!("move up to {max} ambassadors onto pieces in reach"),
        )),
        Op::Shelter => one(clause(
            vec![w("shelter"), w("country")],
            // 178 (Matt's ruling #24): any country, no reach needed.
            "shelter any country this round",
        )),
        // 178 (Matt's ruling #30): the cordon's span is the firewall's. 179 (friction #35): the wall mark (the
        // clock is a delay only).
        Op::Cordon => one(clause(
            adversary(vec![w("agent"), w("country"), w("wall")]),
            "cordon a country until the end of the next adversary turn",
        )),
        Op::Peek { count } => {
            let say = if *count == 1 {
                "look at the next card of the adversary deck".to_string()
            } else {
                format!("look at the next {count} cards of the adversary deck")
            };
            one(clause(counted(*count, "peek"), say))
        }
        // 179 (friction #40): the jam prints outside the dark frame, its number the less mark.
        Op::Jam { amount: n, source: None } => one(clause(
            less(*n, "jam"),
            format!("one threat needs {n} less of each resource"),
        )),
        Op::Jam { amount: n, source: Some(s) } => one(clause(
            { let mut p = less(*n, "jam"); p.push(w(&format!("#{}", s.name()))); p },
            format!("counts as {n} toward {} threat", s.a_word()),
        )),
        Op::CounterThreat => one(clause(adversary(vec![w("counter")]), "counter one threat")),
        Op::DrainProject { amount: n } => {
            let mut parts = minus(*n, "compute");
            parts.push(w("project"));
            one(clause(adversary(parts), format!("remove {n} compute from an adversary project")))
        }
        Op::DrainAll { amount: n } => {
            let mut parts = minus(*n, "compute");
            parts.extend([w("/"), w("project")]);
            one(clause(adversary(parts), format!("remove {n} compute from each adversary project")))
        }
        Op::DiscardProject => one(clause(adversary(minus(1, "project")), "discard an adversary project")),
        // 106: a program's per-tag income raise, counted once.
        Op::ProductionPerTag { resource, tag: t, per } => {
            let mut icons = vec![w(icon(*resource)), w("/")];
            if *per != 1 {
                icons.push(Part::Word(per.to_string()));
            }
            icons.push(hash(*t));
            let tags = if *per == 1 { format!("{} tag", tag(*t)) } else { format!("{per} {} tags", tag(*t)) };
            one(clause(
                track_move(1, icons),
                format!("raise your {} income by 1 per {tags} on your cards", track(*resource)),
            ))
        }
        Op::Production { resource, amount: n } => {
            let verb = if *n < 0 { "lower" } else { "raise" };
            one(clause(
                track_move(*n, vec![w(icon(*resource))]),
                format!("{verb} your {} income by {}", track(*resource), n.abs()),
            ))
        }
        // 076: a disc alone is on this card; boxed, it goes to one of yours.
        Op::AddTokens { kind, amount: n, here: true } => one(clause(
            { let mut p = vec![Part::Word(format!("+{n}"))]; p.push(w(kind.name())); p },
            format!("add {n} {} here", token(*kind, *n)),
        )),
        Op::AddTokens { kind, amount: n, here: false } => one(clause(
            vec![Part::Yours { yours: counted(*n, kind.name()) }],
            format!("add {n} {} to one of your {} buildings", token(*kind, *n), token(*kind, 1)),
        )),
        Op::CutBotnet => one(clause(
            adversary(vec![w("botnet")]),
            "a botnet in reach makes no compute next build",
        )),
        Op::MoveOne { range } => one(clause(
            vec![w("move"), Part::Word(range.to_string()), w("hex")],
            format!("move 1 ambassador on the board up to {}", hexes(*range)),
        )),
        Op::SpreadAmbassadors => one(clause(
            vec![w("ALL"), w("move"), w("hex")],
            "each of your ambassadors on the board may move 1 hex onto an adjacent piece",
        )),
        // 179 (friction #39): onto one piece, the destination mark (Liaison Corps' plain move is onto pieces).
        Op::Consolidate { max } => one(clause(
            vec![Part::Word(max.to_string()), w("move"), w("onto")],
            format!("move up to {max} ambassadors in reach onto one piece"),
        )),
        // 158: convert, gather and the firewall (Matt, 2026-11-09).
        Op::Convert { max } => {
            let mut icons = adversary({
                let mut p = minus(*max, "agent");
                p.push(w("city"));
                p
            });
            // 184: each becomes your ambassador where it stood (r77, 178 #32): no node.
            icons.extend(counted(*max, "ambassador"));
            let what = if *max == 1 { "1 agent".to_string() } else { format!("{max} agents") };
            one(clause(icons, format!("convert {what} around a city in reach")))
        }
        // 166 (Matt, 2026-11-09, *Moving pieces*): bare silhouettes, a number before its icon, "up to"
        // printed (≤N); the dark frame only around the adversary's pieces.
        Op::Gather { max, pieces, strike: false } => {
            let mut icons = vec![Part::Word(format!("\u{2264}{max}"))];
            icons.extend(piece_icons(pieces));
            icons.extend([w("move"), w("hex")]);
            let say = format!("gather up to {max} {} into a hex in reach, each from a hex next to it", piece_words(pieces));
            one(clause(icons, say))
        }
        // The strike (178, Matt's ruling #27): a removal around a hex, the hex included, the player's choice (the
        // number carries its minus); no move icon, as nothing moves.
        Op::Gather { max, pieces, strike: true } => {
            let mut icons = vec![Part::Word(format!("\u{2212}\u{2264}{max}"))];
            icons.extend(piece_icons(pieces));
            icons.push(w("hex"));
            let say = format!("remove up to {max} {} on or next to a hex in reach, your choice", piece_words(pieces));
            one(clause(icons, say))
        }
        Op::Push { max, pieces } => {
            let mut icons = vec![w("hex"), w("move"), Part::Word(format!("\u{2264}{max}"))];
            icons.extend(piece_icons(pieces));
            let say = format!("push up to {max} {} from a hex in reach, each to a hex next to it", piece_words(pieces));
            one(clause(icons, say))
        }
        Op::Firewall => one(clause(
            adversary(vec![w("agent"), w("hex"), w("wall")]),
            // 178: its span ("until the end of the next adversary turn") prints on the token and in the house
            // term; Air Gap's box has no room for it twice.
            "firewall a hex in reach",
        )),
        Op::IfTags { tag: t, count, then } => {
            let inner = clauses(then, gain);
            let mut icons = vec![Part::Word(count.to_string()), hash(*t)];
            let mut says = Vec::new();
            for c in inner {
                icons.extend(c.icons);
                says.push(c.say);
            }
            one(clause(icons, format!("with {count} {} tags, also {}", tag(*t), says.join(" and "))))
        }
        Op::Draw { count } => one(clause(
            counted(*count, "cards"),
            format!("draw {}", plural(*count, "card")),
        )),
        // 076 (ruling 12): one card's tokens, never "you have". Content::check
        // refuses gain_per_tokens without `here`.
        Op::GainPerTokens { resource, kind, per, here } => {
            let mut icons = vec![w(icon(*resource)), w("/")];
            let counted_here = counted(*per, kind.name());
            let place = if *here { "here" } else { "on one of your cards" };
            if *here {
                icons.extend(counted_here);
            } else {
                icons.push(Part::Yours { yours: counted_here });
            }
            one(clause(icons, format!("{gain} 1 {} {} {place}", noun(*resource, 1), per_tokens(*kind, *per))))
        }
        Op::CashOut { kind, resource, rate, here } => {
            let all = vec![w("ALL"), w(kind.name())];
            let mut icons = if *here { all } else { vec![Part::Yours { yours: all }] };
            icons.push(w("/"));
            icons.extend(counted(*rate, icon(*resource)));
            let from = if *here { "here" } else { "from one of your cards" };
            one(clause(icons, format!("remove the {} {from} for {} each", token(*kind, 2), amount(*resource, *rate))))
        }
        Op::Sway { side, amount: n } => one(clause(
            counted(*n, "sway"),
            format!("sway {n} toward {}", title(side.name())),
        )),
        Op::FreeBuilding => one(clause(
            vec![w("building"), w("free")],
            "play a building from your hand, paying no credits",
        )),
        Op::BuildStation => one(clause(
            vec![w("station"), w("free")],
            "build a research station in reach, paying no chips",
        )),
        Op::ClaimStation => one(clause(
            vec![w("station")],
            "claim a neutral research station in reach",
        )),
        Op::ProjectCube { count, station } => {
            let mut icons = counted(*count, "research_cubes");
            icons.push(w("megaproject"));
            // D48 (r48): the station limit shows (Field Grant).
            if *station {
                icons.extend([w("/"), w("station")]);
            }
            let what = amount(Resource::ResearchCubes, *count);
            // 118: both kinds need an incomplete megaproject (Field Grant's said nothing of it).
            // 130: "an open megaproject" (a house term: one that isn't complete).
            let place = if *station { ", in a country where you own a station" } else { "" };
            one(clause(icons, format!("add {what} in your color to an open megaproject{place}")))
        }
        // 105: a brown box below the art moves the track; the solid outline is
        // one of your cards.
        Op::CopyIncome => one(clause(
            track_move(1, vec![Part::Yours { yours: vec![w("building")] }]),
            "raise your income by one of your buildings' Each round output",
        )),
        Op::Reuse => one(clause(
            vec![w("ACT"), Part::Yours { yours: vec![w("building")] }],
            "use another of your buildings' Actions again this round",
        )),
        Op::DiscardHand { resource, amount: n } => one(clause(
            { let mut p = counted(*n, icon(*resource)); p.extend([w("/"), w("cards")]); p },
            format!("discard a card from your hand and {gain} {}", amount(*resource, *n)),
        )),
        Op::Troops { .. }
        | Op::FlipStation { .. }
        | Op::CorruptCity { .. }
        | Op::PlayersLose { .. }
        | Op::ThreatAtNode { .. }
        | Op::EachDestroyNode { .. }
        | Op::DestroyNode { .. }
        | Op::PullUnity { .. }
        | Op::AdversaryBuild
        | Op::Create { .. }
        | Op::CreateSite { .. }
        | Op::Corrupt { .. }
        | Op::Flag { .. }
        | Op::Requisition { .. }
        | Op::LoseAmbassador
        | Op::LoseNodeOr { .. }
        | Op::DiscardMostExpensive
        | Op::Compute { .. }
        | Op::ComputeEvery { .. }
        | Op::LoseTokens { .. }
        | Op::TeamIncome { .. }
        | Op::FreePurchases { .. }
        | Op::CounterAll
        | Op::Pushback { .. }
        | Op::DealThreats { .. }
        | Op::RemoveBotnets { .. }
        | Op::AdversaryCubes { .. }
        | Op::BotnetCubes { .. }
        | Op::OnePlayer { .. }
        | Op::PerSite { .. }
        | Op::RemoveWhere { .. }
        | Op::Prohibit { .. }
        | Op::Rally { .. }
        | Op::IncreaseIncome { .. }
        | Op::Income { .. }
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
        | Op::ConvertUpkeep { .. }
        | Op::AdversaryFactory { .. }
        | Op::ComputePerMissing { .. }
        | Op::CorpTroops { .. }
        | Op::DestroyFacility { .. }
        | Op::DisableBuilding { .. }
        | Op::Discard { .. }
        | Op::ExtraWar { .. }
        | Op::LoseAmbassadorsOr { .. }
        | Op::LoseIncome { .. }
        | Op::LoseOr { .. }
        | Op::MarchNow { .. }
        | Op::OccupyFactory { .. }
        | Op::PlaceBotnet { .. }
        | Op::SeizeFactory
        | Op::StandingTroopsBonus { .. }
        | Op::ThreatCompute { .. }
        | Op::Theft { .. }
        | Op::TroopSteps { .. } => panic!("{op:?} is not an instant player op: Content::check rejects it"),
    }
}

/// A standing op: `Income` is the brown box's, `Effect` the rest.
enum Standing {
    Income(Vec<Part>, String),
    Effect(Vec<Part>, String),
}

/// 105: a trigger's left side, before its `:`, and its words ("when ...").
fn event_face(on: &Event) -> (Vec<Part>, String) {
    let a_card = |t: Option<Tag>| match t {
        // "an Industry card" (106: the article was always "a").
        Some(t) if tag(t).starts_with(['A', 'E', 'I', 'O', 'U']) => format!("an {} card", tag(t)),
        Some(t) => format!("a {} card", tag(t)),
        None => "a card".to_string(),
    };
    let played = |t: Option<Tag>| {
        let mut p = vec![w("play")];
        p.extend(t.map(hash));
        p
    };
    match *on {
        Event::Play { tag: t } => (played(t), format!("when you play {}", a_card(t))),
        Event::PlayTag { .. } => (played(None), "when you play a card".to_string()),
        Event::TeamPlay { tag: t, .. } => {
            (vec![Part::Other { other: played(t) }], format!("when another player plays {}", a_card(t)))
        }
        Event::PlayCost { at } => (
            vec![w("play"), Part::Word(format!("\u{2265}{at}")), w("credits")],
            format!("when you play a card printed at {at} credits or more"),
        ),
        // 184: once an action (r77), however many come off: no count, as the agent trigger.
        Event::RemoveTroop => (adversary(vec![w("troop")]), "when you remove a troop".to_string()),
        Event::RemoveCompute => (
            adversary(vec![w("compute"), w("project")]),
            // 106: compute is only ever on adversary projects; the long form didn't fit 7.5 pt.
            "when you remove adversary compute".to_string(),
        ),
        Event::RemoveAmbassadors => (adversary(vec![w("agent")]), "when you remove agents".to_string()),
        Event::PlaceTokens { kind } => (
            vec![Part::Yours { yours: vec![w(kind.name())] }],
            format!("when you place a {} on one of your cards", token(kind, 1)),
        ),
        Event::MarchEndsNear => {
            (vec![w("troop"), w("node")], "when a Troop ends its march next to your Node".to_string())
        }
    }
}

/// What one standing op prints; `kind` is the card's token type.
fn standing(op: &Op, kind: Option<TokenKind>) -> Standing {
    let income = |parts: Vec<Part>, say: String| Standing::Income(parts, say);
    let effect = |parts: Vec<Part>, say: String| Standing::Effect(parts, say);
    match op {
        Op::CardDiscount { tag: t, resource, amount: n } => {
            let mut parts = match t {
                Some(t) => vec![hash(*t)],
                None => vec![w("play")],
            };
            parts.push(w(":"));
            parts.extend(less(*n, icon(*resource)));
            let which = t.map_or("your cards".to_string(), |t| format!("your {} cards", tag(t)));
            effect(parts, format!("{which} cost {} less to play", amount(*resource, *n)))
        }
        Op::When { on, then } => {
            let (mut parts, when) = event_face(on);
            parts.push(w(":"));
            let cls = clauses(then, "gain");
            let gains: Vec<Part> = cls.iter().flat_map(|c| c.icons.clone()).collect();
            parts.extend(gains.clone());
            let mut say = cls.into_iter().map(|c| c.say).collect::<Vec<_>>().join(", ");
            match on {
                Event::PlayTag { tag: Some(t) } => {
                    parts.extend([w("/"), hash(*t)]);
                    say.push_str(&format!(" per {} tag on it", tag(*t)));
                }
                // 106: every tag on the played card, the empty ring.
                Event::PlayTag { tag: None } => {
                    parts.extend([w("/"), Part::Word("#any".to_string())]);
                    say.push_str(" per tag on it");
                }
                Event::TeamPlay { both: true, .. } => {
                    parts.push(Part::Other { other: gains });
                    say = format!("you both {say}");
                }
                _ => {}
            }
            effect(parts, format!("{when}, {say}"))
        }
        Op::Threshold { at, then } => {
            let kind = kind.expect("Content::check: a threshold counts the card's own tokens");
            let mut parts = self::at(*at, kind.name());
            parts.push(w(":"));
            let cls = clauses(then, "gain");
            parts.extend(cls.iter().flat_map(|c| c.icons.clone()));
            let say = cls.into_iter().map(|c| c.say).collect::<Vec<_>>().join(", ");
            // "At N" is a house term (they come off, then the rest): it fits 7.5 pt.
            effect(parts, format!("at {at}, {say}"))
        }
        Op::Income { resource, amount: n } => income(counted(*n, icon(*resource)), amount(*resource, *n)),
        Op::IncomePerTag { resource, tag: t, per, amount: n } => {
            let mut parts = counted(*n, icon(*resource));
            parts.push(w("/"));
            if *per != 1 {
                parts.push(Part::Word(per.to_string()));
            }
            parts.push(hash(*t));
            let tags = if *per == 1 { format!("{} tag", tag(*t)) } else { format!("{per} {} tags", tag(*t)) };
            income(parts, format!("{n} {} per {tags} on your cards", noun(*resource, *n)))
        }
        Op::ConvertUpkeep { from, amount: n, to, gain } => {
            let mut parts = minus(*n, icon(*from));
            parts.push(w(":"));
            parts.extend(counted(*gain, icon(*to)));
            income(parts, format!("if you can, pay {} and get {}", self::amount(*from, *n), self::amount(*to, *gain)))
        }
        Op::BuildingDiscount { amount: n } => {
            let mut parts = vec![w("building"), w(":")];
            parts.extend(less(*n, "credits"));
            effect(parts, format!("your buildings cost {} less", amount(Resource::Credits, *n)))
        }
        Op::HardenAction { chips: n } => {
            let mut parts = vec![w("ACT"), w("harden")];
            parts.extend(minus(*n, "chips"));
            effect(parts, format!("you gain the Harden action, paid {}", amount(Resource::Chips, *n)))
        }
        Op::Slots { amount: n } => effect(
            vec![Part::Word(format!("+{n}")), w("slot")],
            format!("you have {}", plural(*n, "more building slot")),
        ),
        Op::Logistics { resource, amount: n } => effect(
            { let mut p = vec![w("threat"), w(":")]; p.extend(counted(*n, icon(*resource))); p },
            format!("when a threat you paid toward is countered, gain {}", amount(*resource, *n)),
        ),
        Op::ProjectDrain { amount: n } => effect(
            { let mut p = adversary(minus(*n, "compute")); p.extend([w("/"), w("project")]); p },
            format!("each round, remove {n} compute from each adversary project"),
        ),
        Op::AutoCounter { source, amount: n } => effect(
            { let mut p = less(*n, "jam"); p.push(w(&format!("#{}", source.name()))); p },
            format!("{source} threats need {n} less to counter"),
        ),
        Op::UnityBonus { amount: n } => effect(
            { let mut p = adversary(minus(1, "troop")); p.push(w(":")); p.extend(counted(*n, "sway")); p },
            format!("when you remove an adversary troop, sway {n} more toward Unity"),
        ),
        // 179 (Matt, friction #37; Aegis by amendment 8227): the shield is one stop on one piece (a token); the roof
        // covers many pieces (Shelter: a whole country for the round; with a node, all of your pieces: Aegis keeps
        // any one of them once a round, Sanctuary every node against corruption).
        Op::CorruptionShield => effect(
            vec![w("corrupt"), w(":"), w("shelter"), w("node")],
            "corruption never destroys your nodes".to_string(),
        ),
        Op::CivicShelter => effect(
            vec![w("shelter"), w("node")],
            "once a round, the first time one of your nodes or facilities would be destroyed, it is sheltered".to_string(),
        ),
        Op::TagsFromHand { tag: t } => effect(
            vec![w("cards"), w(":"), w("ACT")],
            format!("when you pay a power's tags, cards you discard from hand count for their {t} tags"),
        ),
        other => panic!("{other:?} has no standing face: Content::check rejects it"),
    }
}

/// The left side of an Action: what it pays, tokens from this card last. 179 (Matt, friction #34): a cost paid
/// prints its minus here too, one rule for costs.
fn action_cost(a: &Ability, kind: Option<TokenKind>) -> (Vec<Part>, Vec<String>) {
    let mut parts = Vec::new();
    let mut said = Vec::new();
    for (r, n) in &a.cost {
        parts.extend(minus(*n, icon(*r)));
        said.push(amount(*r, *n));
    }
    if a.tokens != 0 {
        let kind = kind.expect("Content::check: an ability's tokens are the card's own");
        parts.extend(minus(a.tokens, kind.name()));
        said.push(format!("{} {}", a.tokens, token(kind, a.tokens)));
    }
    (parts, said)
}

fn listed(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// Incomes in a list: a per-tag one after a flat one reads as its own, not as
/// "1 energy and 1 credit per tag", which could count the energy too.
fn incomes(items: &[String]) -> String {
    match items {
        [rest @ .., last] if !rest.is_empty() && last.contains(" per ") => format!("{}, plus {last}", listed(rest)),
        _ => listed(items),
    }
}

fn sentence(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => format!("{}{}.", c.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

/// The face of `c`, generated from its ops.
pub fn card_face(c: &CardDef) -> Face {
    let mut f = Face::default();
    above_art(c, &mut f);
    below_art(c, &mut f);
    including_this(c, &mut f);
    f.top_text = pieces_capitalized(&f.top_text);
    f.text = pieces_capitalized(&f.text);
    f
}

/// 178 (amendment 8044, Matt: as Spirit Island prints its Invaders): on cards, the pieces' names print
/// capitalized (Ambassador, Agent, Node, Botnet, Troop; the Drone is a proper name already); the rulebook's prose
/// keeps them lowercase.
pub fn pieces_capitalized(text: &str) -> String {
    const PIECES: [&str; 5] = ["ambassador", "agent", "node", "botnet", "troop"];
    let mut out = String::with_capacity(text.len());
    let mut word = String::new();
    let flush = |word: &mut String, out: &mut String| {
        let stem = word.strip_suffix('s').unwrap_or(word);
        if PIECES.contains(&stem) {
            out.push_str(&title(word));
        } else {
            out.push_str(word);
        }
        word.clear();
    };
    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            word.push(ch);
        } else {
            flush(&mut word, &mut out);
            out.push(ch);
        }
    }
    flush(&mut word, &mut out);
    out
}

/// 178 (Matt, friction #21): a per-tag line counts the card's own tags, and says so. An event's tags never count
/// (`effects::own_counted`), so its lines stay as they are; nor does a card's own tag count toward its `needs`.
fn including_this(c: &CardDef, f: &mut Face) {
    if c.is_event() {
        return;
    }
    let mut tags: Vec<String> = c.tags.iter().map(|t| tag(*t)).collect();
    tags.dedup();
    for t in tags {
        for text in [&mut f.top_text, &mut f.text] {
            for form in [format!("{t} tag on your cards"), format!("{t} tags on your cards")] {
                if text.contains(&form) {
                    *text = text.replace(&form, &format!("{form} (including this)"));
                }
            }
        }
    }
}

/// Above the art: income first, in one brown box and one "Each round, ...".
fn above_art(c: &CardDef, f: &mut Face) {
    let mut top_said: Vec<String> = Vec::new();

    let mut boxes: Vec<Vec<Part>> = Vec::new();
    let mut each_round: Vec<String> = Vec::new();
    let mut effects: Vec<(Vec<Part>, String)> = Vec::new();
    let mut plain = false;
    for op in &c.passives {
        match standing(op, c.tokens.as_ref().map(|t| t.kind)) {
            Standing::Income(parts, say) => {
                let this_plain = matches!(op, Op::Income { .. });
                // 118: in a box of two incomes a lone icon ("1 chip") keeps its 1:
                // Fabrication Megaplex read "7 credits chips".
                let one = |mut p: Vec<Part>| {
                    if p.len() == 1 {
                        p.insert(0, Part::Word("1".to_string()));
                    }
                    p
                };
                match boxes.last_mut() {
                    Some(b) if plain && this_plain => {
                        let first = std::mem::take(b);
                        *b = one(first);
                        b.extend(one(parts));
                    }
                    _ => boxes.push(parts),
                }
                plain = this_plain;
                each_round.push(say);
            }
            Standing::Effect(parts, say) => effects.push((parts, say)),
        }
    }
    for b in boxes {
        f.top.push(vec![Part::Income { income: b }]);
    }
    // A conversion is a sentence of its own: "1 chip and if you can ..." reads wrong.
    let (ifs, plain_round): (Vec<String>, Vec<String>) =
        each_round.into_iter().partition(|s| s.starts_with("if you can"));
    if !plain_round.is_empty() {
        top_said.push(sentence(&format!("each round, {}", incomes(&plain_round))));
    }
    for s in ifs {
        top_said.push(sentence(&format!("each round, {s}")));
    }
    if !effects.is_empty() {
        let says: Vec<&str> = effects.iter().map(|(_, say)| say.as_str()).collect();
        top_said.push(sentence(&format!("effect: {}", says.join("; "))));
    }
    for (parts, _) in effects {
        f.top.push(parts);
    }
    if let Some(a) = &c.ability {
        // 076: an OR Action prints a line a branch, the second opening with OR.
        let mut halves: Vec<String> = Vec::new();
        // 130: halves that all pay the same say it once, "pay 2 energy: A, OR B" (Particle
        // Accelerator's two halves overran its top box).
        let branches = a.branches();
        let costs: Vec<_> = branches.iter().map(|b| action_cost(b, c.tokens.as_ref().map(|t| t.kind)).1).collect();
        let shared = branches.len() > 1 && !costs[0].is_empty() && costs.iter().all(|p| *p == costs[0]);
        for (i, b) in branches.into_iter().enumerate() {
            let (cost, paid) = action_cost(b, c.tokens.as_ref().map(|t| t.kind));
            let mut left = if i > 0 { vec![w("OR")] } else { Vec::new() };
            // r35: a gate on Unity tips and a one-shot half are said in words only
            // (the card art has no icon for a tip or a discard).
            left.extend(cost);
            left.push(w("\u{2192}"));
            let right = clauses(&b.effects, "get");
            left.extend(right.iter().flat_map(|c| c.icons.clone()));
            let mut says: Vec<String> = right.into_iter().map(|c| c.say).collect();
            if b.once {
                // D31: the stack under it goes too, in the plates' term. The words are paid
                // for in the same top box at 7.5 pt: liberating already means a corrupted
                // city, and the list order says "then".
                for s in &mut says {
                    *s = s.replace("liberate a corrupted city", "liberate a city");
                }
                says.push("discard it and what it's built over".to_string());
            }
            // Call 2: a bare arrow is a free action; every Action is once a round.
            let open = if paid.is_empty() || shared { String::new() } else { format!("pay {}, ", listed(&paid)) };
            match f.top.last_mut() {
                // 130: a shared cost prints once, the later halves on the first half's line.
                Some(line) if shared && i > 0 => {
                    line.push(w("OR"));
                    line.extend(left.into_iter().skip_while(|p| *p != w("\u{2192}")).skip(1));
                }
                _ => f.top.push(left),
            }
            let mut said = format!("{open}{}", says.join(", "));
            // 130: a later half that gets what the first half placed says "get them"
            // (Particle Accelerator: "add 3 research cubes ... OR get them").
            let again = shared
                && i > 0
                && said
                    .strip_prefix("get ")
                    .is_some_and(|what| halves.first().is_some_and(|h| h.starts_with(&format!("add {what} "))));
            if again {
                said = "get them".to_string();
            }
            halves.push(said);
        }
        let pay = if shared { format!("pay {}: ", listed(&costs[0])) } else { String::new() };
        top_said.push(sentence(&format!("action: {pay}{}", halves.join(", OR "))));
    }
    f.top_text = top_said.join(" ");
}

/// Below the art: what it costs beyond credits, then what it does.
fn below_art(c: &CardDef, f: &mut Face) {
    let mut said: Vec<String> = Vec::new();
    let pays: Vec<(Resource, i64)> = [
        (Resource::Chips, c.chips),
        (Resource::Energy, c.energy),
        (Resource::Antimatter, c.antimatter),
        (Resource::ResearchCubes, c.research_cubes),
    ]
    .into_iter()
    .filter(|(_, n)| *n != 0)
    .collect();
    if !pays.is_empty() {
        f.effect.push(pays.iter().flat_map(|(r, n)| minus(*n, icon(*r))).collect());
        let list: Vec<String> = pays.iter().map(|(r, n)| amount(*r, *n)).collect();
        said.push(sentence(&format!("pay {}", listed(&list))));
    }
    for (r, n) in &c.lowers {
        f.effect.push(track_move(-n, vec![w(icon(*r))]));
        said.push(sentence(&format!("lower your {} income by {n}", track(*r))));
    }
    for cl in clauses(&c.effects, "gain") {
        f.effect.push(cl.icons);
        said.push(sentence(&cl.say));
    }
    f.text = said.join(" ");
}

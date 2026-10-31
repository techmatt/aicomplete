//! A player card's printed face, generated from its ops (033): the icon lines
//! and the rules text come out of one walk, so the text mirrors the icons in
//! order and a card can't say one thing and play another.
//!
//! **Where it sits is what it is.** Above the art (`top`, buildings only): what
//! lasts while the building is active. Below the art (`effect`): what happens
//! once, when played, extra costs first. The grammar is in docs/rules.md (*The
//! card grammar*); `web/card/card.js` draws the parts, and `web/card/faces.json`
//! is this module's output for it, kept current by `aic-sim docs`.
//!
//! A new one-shot op fails to compile here until it has its face: that match
//! has no wildcard. `standing` has one, so a new standing op panics at content
//! load until it has a face. Ops no card can print ([`Op::slots`]) panic:
//! `Content::check` rejects them first.

use serde::Serialize;

use crate::content::{Ability, CardDef, Cond, CountryRule, DamageAt, Op, Resource, Tag, TokenKind};

/// One part of an icon line: an icon or a connector (`→ / : * OR ALL ELSE ACT`), a
/// tag (`#science`), a number, or a box of parts.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Part {
    Word(String),
    Income { income: Vec<Part> },
    Adversary { adversary: Vec<Part> },
    Other { other: Vec<Part> },
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

/// A negative always has its minus sign, 1 too.
fn minus(n: i64, icon: &str) -> Vec<Part> {
    vec![Part::Word(format!("\u{2212}{n}")), w(icon)]
}

fn icon(r: Resource) -> &'static str {
    match r {
        Resource::Ambassadors => "ambassador",
        Resource::Antimatter => "antimatter",
        Resource::Chips => "chips",
        Resource::Credits => "credits",
        Resource::Energy => "energy",
        Resource::Insight => "insight",
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
        (TokenKind::Agents, 1) => "Agent",
        (TokenKind::Datasets, _) => "Datasets",
        (TokenKind::Followers, _) => "Followers",
        (TokenKind::Agents, _) => "Agents",
        (TokenKind::Capital, _) => "Capital",
    }
}

fn tag(t: Tag) -> String {
    let name = t.name();
    name[..1].to_uppercase() + &name[1..]
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

fn country_for(rule: CountryRule) -> &'static str {
    match rule {
        CountryRule::Any => "in your reach, in one country",
        CountryRule::PresenceCountry => "in your reach* (*one country with your presence)",
    }
}

fn hexes(n: i64) -> String {
    if n == 1 {
        "1 hex".to_string()
    } else {
        format!("{n} hexes")
    }
}

fn holds(cond: Cond) -> &'static str {
    match cond {
        Cond::RemovedTroop => "if you removed an adversary troop this round",
        Cond::CityWithAmbassador => "if a clean city holds adversary ambassadors",
    }
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
        Op::GainPerPresence { resource, max } => one(clause(
            vec![w(icon(*resource)), w("/"), w("presence"), w("*")],
            format!("{gain} 1 {} per presence you have on the board (*max {max})", noun(*resource, 1)),
        )),
        // It only names who the `other_gain` or `transfer` after it reaches.
        Op::ChoosePlayer => Vec::new(),
        Op::OtherGain { resource, amount: n } => one(Clause {
            icons: counted(*n, icon(*resource)),
            say: format!("another player gains {}", amount(*resource, *n)),
            share: Share::Theirs,
        }),
        Op::Transfer { resource, max, bonus } => {
            let mut parts = counted(*max, icon(*resource));
            let mut say = format!("give another player up to {}", amount(*resource, *max));
            if *bonus > 0 {
                parts.push(w("*"));
                say.push_str(&format!("* (*if you gave any, they gain {bonus} more)"));
            }
            one(clause(vec![Part::Other { other: parts }], say))
        }
        Op::Convert { from, to, max, rate } => {
            let mut icons = counted(*rate, icon(*to));
            icons.push(w("/"));
            icons.extend(minus(1, icon(*from)));
            icons.push(w("*"));
            one(clause(
                icons,
                format!(
                    "{gain} {} per {} you pay (*at most {})",
                    amount(*to, *rate),
                    noun(*from, 1),
                    amount(*from, *max)
                ),
            ))
        }
        Op::PlacePresence { count, free } => one(clause(
            counted(*count, "presence"),
            format!("place {count} presence{}", if *free { " free" } else { "" }),
        )),
        Op::ClearMarkers { count } => one(clause(
            adversary(minus(*count, "marker")),
            format!("clear up to {count} corruption marker{} in your reach", if *count == 1 { "" } else { "s" }),
        )),
        Op::ShieldPresence { count } => one(clause(
            counted(*count, "harden"),
            format!("harden up to {count} hex{} of your presence", if *count == 1 { "" } else { "es" }),
        )),
        Op::DelayThreat { count } => one(clause(
            adversary(counted(*count, "clock")),
            if *count == 1 {
                "one threat waits: not executed this round".to_string()
            } else {
                format!("up to {count} threats wait: not executed this round")
            },
        )),
        Op::Damage { amount: n, at } => {
            let mut parts = minus(*n, "ambassador");
            parts.extend([w("troop"), w("hex")]);
            let place = match at {
                DamageAt::Reach => "one hex in your reach",
                DamageAt::PresenceCountry => {
                    parts.push(w("*"));
                    "one hex in your reach* (*in a country with your presence)"
                }
            };
            one(clause(adversary(parts), format!("remove {n} worth of adversary pieces in {place}")))
        }
        Op::DamageCountry { amount: n } => {
            let mut parts = minus(*n, "ambassador");
            parts.extend([w("troop"), w("/"), w("hex")]);
            let mut icons = adversary(parts);
            icons.push(w("country"));
            one(clause(icons, format!("remove {n} worth of adversary pieces in each hex of one country")))
        }
        Op::DestroyAll => one(clause(
            adversary(vec![w("ALL"), w("ambassador"), w("troop"), w("hex")]),
            "remove every adversary ambassador and troop in one hex",
        )),
        Op::RemoveNode => one(clause(adversary(minus(1, "node")), "remove a node in your reach")),
        Op::RemoveAmbassadorsCountry { unity_per } => {
            let mut icons = adversary(vec![w("ALL"), w("ambassador"), w("country")]);
            icons.extend([w("sway"), w("/")]);
            icons.extend(adversary(vec![Part::Word(unity_per.to_string()), w("ambassador")]));
            one(clause(
                icons,
                format!("remove every adversary ambassador in one country, and sway 1 toward Unity per {unity_per} removed"),
            ))
        }
        Op::BuildFactory => one(clause(vec![w("factory")], "build a factory on a free industrial zone in your reach")),
        Op::Cleanse { amount: n, one_country, cities } => {
            let what = match (*cities, *n) {
                (true, _) => "corrupted cities, stations or factories",
                (false, 1) => "corrupted research station or factory",
                (false, _) => "corrupted research stations or factories",
            };
            let place = if *one_country { "in your reach, all in one country" } else { "in your reach" };
            let say = if *cities {
                format!("free up to {n} {what} {place}")
            } else {
                let most = if *n == 1 { "1".to_string() } else { format!("up to {n}") };
                format!("take the adversary cube off {most} {what} {place}")
            };
            let mut parts = minus(*n, "cube");
            if *one_country {
                parts.push(w("country"));
            }
            one(clause(adversary(parts), say))
        }
        Op::ClearCity { max } => {
            let (parts, what) = if *max > 0 {
                let mut p = minus(*max, "ambassador");
                p.push(w("city"));
                (p, format!("{max} adversary ambassadors, or all if fewer,"))
            } else {
                (vec![w("ALL"), w("ambassador"), w("city")], "every adversary ambassador".to_string())
            };
            one(clause(adversary(parts), format!("remove {what} next to, then on, one corrupted city in your reach")))
        }
        Op::Harden { count, in_country } => {
            let mut icons = counted(*count, "harden");
            icons.push(w("country"));
            if *in_country == CountryRule::PresenceCountry {
                icons.push(w("*"));
            }
            let what = if *count == 1 { "1 building".to_string() } else { format!("{count} buildings") };
            one(clause(icons, format!("harden {what} {}", country_for(*in_country))))
        }
        Op::MoveAmbassadors { max } => one(clause(
            vec![Part::Word(max.to_string()), w("move")],
            format!("move up to {max} of your ambassadors onto pieces in your reach"),
        )),
        Op::Shelter => one(clause(
            vec![w("shelter"), w("country")],
            "shelter one country this round",
        )),
        Op::Peek { count } => {
            let say = if *count == 1 {
                "look at the top card of the adversary deck".to_string()
            } else {
                format!("look at the top {count} cards of the adversary deck")
            };
            one(clause(counted(*count, "peek"), say))
        }
        Op::Jam { amount: n } => one(clause(
            adversary(minus(*n, "threat")),
            format!("one threat needs {n} less of each resource"),
        )),
        Op::CounterThreat => one(clause(adversary(vec![w("counter")]), "counter one active threat")),
        Op::DrainProject { amount: n } => {
            let mut parts = minus(*n, "compute");
            parts.push(w("project"));
            one(clause(adversary(parts), format!("remove up to {n} compute from one visible adversary project")))
        }
        Op::DrainAll { amount: n } => {
            let mut parts = minus(*n, "compute");
            parts.extend([w("/"), w("project")]);
            one(clause(adversary(parts), format!("remove up to {n} compute from every visible adversary project")))
        }
        Op::DiscardProject => one(clause(
            adversary(minus(1, "project")),
            "discard one visible adversary project, with its compute",
        )),
        Op::NextRoundInsight { amount: n } => {
            let mut parts = vec![w("ALL")];
            parts.extend(counted(*n, "insight"));
            parts.push(w("*"));
            one(clause(parts, format!("every player gains {n} insight* (*next round)")))
        }
        Op::NextBuildingDiscount { amount: n } => {
            let mut icons = vec![w("building"), w(":")];
            icons.extend(minus(*n, "credits"));
            one(clause(icons, format!("your next building this round costs {n} less")))
        }
        Op::Production { resource, amount: n } => {
            let sign = if *n < 0 { format!("\u{2212}{}", -n) } else { format!("+{n}") };
            let verb = if *n < 0 { "lower" } else { "raise" };
            one(clause(
                vec![Part::Income { income: vec![Part::Word(sign), w(icon(*resource))] }],
                format!("{verb} your {} income by {}", track(*resource), n.abs()),
            ))
        }
        Op::AddTokens { kind, amount: n } => one(clause(
            vec![Part::Other { other: counted(*n, kind.name()) }],
            format!("add {n} {} to one of your active buildings that holds them", token(*kind, *n)),
        )),
        Op::CutNode => one(clause(
            adversary(vec![w("node"), w("*")]),
            "cut off a node in your reach* (*no compute from it in the next build)",
        )),
        Op::MoveOne { range } => one(clause(
            vec![w("move"), Part::Word(range.to_string()), w("hex")],
            format!("you may move one of your ambassadors on the board up to {}, onto a piece", hexes(*range)),
        )),
        Op::SpreadAmbassadors => one(clause(
            vec![w("ALL"), w("move"), w("hex")],
            "each of your ambassadors on the board may move 1 hex onto an adjacent piece",
        )),
        Op::Consolidate { max } => one(clause(
            vec![Part::Word(max.to_string()), w("move"), w("*")],
            format!("move up to {max} of your ambassadors in your reach* (*all onto one piece in your reach)"),
        )),
        Op::Draw { count } => {
            let cards = if *count == 1 { "card" } else { "cards" };
            one(clause(counted(*count, "cards"), format!("draw {count} tier-1 {cards}")))
        }
        Op::GainPerTokens { resource, kind, per, max } => {
            let mut icons = vec![w(icon(*resource)), w("/")];
            icons.push(Part::Other { other: counted(*per, kind.name()) });
            icons.push(w("*"));
            one(clause(
                icons,
                format!(
                    "{gain} 1 {} {} on your active buildings (*max {max}; tokens stay)",
                    noun(*resource, 1),
                    per_tokens(*kind, *per)
                ),
            ))
        }
        Op::CashOut { kind, resource, max } => {
            let what = if *max > 0 {
                format!("{max} {} from your active buildings, or all if fewer", token(*kind, 2))
            } else {
                format!("all {} from your active buildings", token(*kind, 2))
            };
            one(clause(
                vec![Part::Other { other: vec![w("\u{2212}n"), w(kind.name())] }, w(icon(*resource)), w("*")],
                format!("remove {what}, {gain} {}* (*1+2+...+n for the n removed)", noun(*resource, 2)),
            ))
        }
        Op::If { cond, then, otherwise } => {
            let yes = clauses(then, gain);
            let no = clauses(otherwise, gain);
            let mut icons: Vec<Part> = yes.iter().flat_map(|c| c.icons.clone()).collect();
            icons.push(w("*"));
            let said = |list: &[Clause]| list.iter().map(|c| c.say.clone()).collect::<Vec<_>>().join(", ");
            let mut say = format!("{}* (*{})", said(&yes), holds(*cond));
            if !no.is_empty() {
                icons.push(w("ELSE"));
                icons.extend(no.iter().flat_map(|c| c.icons.clone()));
                say.push_str(&format!(", otherwise {}", said(&no)));
            }
            one(clause(icons, say))
        }
        Op::Sway { side, amount: n } => {
            let name = side.name();
            one(clause(
                counted(*n, "sway"),
                format!("sway {n} toward {}{}", name[..1].to_uppercase(), &name[1..]),
            ))
        }
        Op::Troops { .. }
        | Op::FlipStation { .. }
        | Op::CorruptCity
        | Op::PlayersLose { .. }
        | Op::DestroyPresence
        | Op::EachDestroyPresence { .. }
        | Op::PullUnity { .. }
        | Op::LoseInsight { .. }
        | Op::AdversaryBuild
        | Op::Ambassadors { .. }
        | Op::Flag { .. }
        | Op::Requisition { .. }
        | Op::LoseAmbassador
        | Op::LosePresenceOr { .. }
        | Op::DiscardMostExpensive
        | Op::Compute { .. }
        | Op::LoseTokens { .. }
        | Op::TeamIncome { .. }
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
        | Op::Income { .. }
        | Op::IncomePerTag { .. }
        | Op::IncomeIfTags { .. }
        | Op::BuildingDiscount { .. }
        | Op::HardenAction
        | Op::Slots { .. }
        | Op::Logistics { .. }
        | Op::UnityBonus { .. }
        | Op::GiveBonus { .. }
        | Op::SpreadShield
        | Op::CivicShelter
        | Op::TagsFromHand { .. }
        | Op::ConvertUpkeep { .. }
        | Op::CityBuild { .. }
        | Op::CleanFactories { .. }
        | Op::StationBuild { .. }
        | Op::SpreadReach { .. }
        | Op::CorruptSooner { .. }
        | Op::Arrivals { .. }
        | Op::War => panic!("{op:?} is not an instant player op: Content::check rejects it"),
    }
}

/// A standing op: `Income` is the brown box's, `Effect` the rest.
enum Standing {
    Income(Vec<Part>, String),
    Effect(Vec<Part>, String),
}

/// A card with a passive the face prints as an effect: what a slotless
/// card's "takes no slot" note hangs on (`Content::check`).
pub(crate) fn prints_an_effect(c: &CardDef) -> bool {
    c.passives.iter().any(|op| matches!(standing(op), Standing::Effect(..)))
}

/// What one standing op prints.
fn standing(op: &Op) -> Standing {
    let income = |parts: Vec<Part>, say: String| Standing::Income(parts, say);
    let effect = |parts: Vec<Part>, say: String| Standing::Effect(parts, say);
    match op {
        Op::Income { resource, amount: n } => income(counted(*n, icon(*resource)), amount(*resource, *n)),
        Op::IncomePerTag { resource, tag: t, per, max } => {
            let mut parts = vec![w(icon(*resource)), w("/")];
            if *per != 1 {
                parts.push(Part::Word(per.to_string()));
            }
            parts.extend([hash(*t), w("*")]);
            let tags = if *per == 1 { format!("{} tag", tag(*t)) } else { format!("{per} {} tags", tag(*t)) };
            income(parts, format!("1 {} per {tags} you have (*max {max})", noun(*resource, 1)))
        }
        Op::IncomeIfTags { resource, tag: t, count, amount: n } => {
            let mut parts = counted(*n, icon(*resource));
            parts.push(w("*"));
            income(parts, format!("{}* (*with {count}+ {} tags)", amount(*resource, *n), tag(*t)))
        }
        Op::ConvertUpkeep { from, amount: n, to, gain } => {
            let mut parts = minus(*n, icon(*from));
            parts.push(w(":"));
            parts.extend(counted(*gain, icon(*to)));
            income(parts, format!("if you can, pay {} and get {}", self::amount(*from, *n), self::amount(*to, *gain)))
        }
        Op::BuildingDiscount { amount: n } => {
            let mut parts = vec![w("building"), w(":")];
            parts.extend(minus(*n, "credits"));
            effect(parts, format!("when you play a building, pay {} less", amount(Resource::Credits, *n)))
        }
        Op::HardenAction => effect(
            vec![w("ACT"), w("harden")],
            "you may Harden as a standard action".to_string(),
        ),
        Op::Slots { amount: n } => effect(
            vec![Part::Word(format!("+{n}")), w("slot")],
            format!("you have {n} more building slot{}", if *n == 1 { "" } else { "s" }),
        ),
        Op::Logistics { resource, amount: n } => effect(
            { let mut p = vec![w("threat"), w(":")]; p.extend(counted(*n, icon(*resource))); p },
            format!("when a threat you paid toward is countered, gain {}", amount(*resource, *n)),
        ),
        Op::UnityBonus { amount: n } => effect(
            { let mut p = adversary(minus(1, "troop")); p.push(w(":")); p.extend(counted(*n, "sway")); p },
            format!("when you remove an adversary troop, sway {n} more toward Unity"),
        ),
        Op::GiveBonus { resource, amount: n } => effect(
            { let mut p = vec![Part::Other { other: vec![w("insight")] }, w(":")]; p.extend(counted(*n, icon(*resource))); p },
            format!("when you give insight, the receiver also gains {}", amount(*resource, *n)),
        ),
        Op::SpreadShield => effect(
            { let mut p = adversary(vec![w("move")]); p.extend([w(":"), w("shelter"), w("presence")]); p },
            "when the adversary spreads, it skips your presence".to_string(),
        ),
        Op::CivicShelter => effect(
            vec![w("building"), w(":"), w("shelter"), w("*")],
            "when a building or presence in a country with your presence would be destroyed, save it* (*once a round)".to_string(),
        ),
        Op::TagsFromHand { tag: t } => effect(
            vec![w("cards"), w(":"), w("ACT")],
            format!("when you pay a power's tags, cards you discard from hand count for their {t} tags"),
        ),
        other => panic!("{other:?} is not a standing op: Content::check rejects it"),
    }
}

/// The left side of an Action: what it pays, tokens from this card last.
fn action_cost(a: &Ability, kind: Option<TokenKind>) -> (Vec<Part>, Vec<String>) {
    let mut parts = Vec::new();
    let mut said = Vec::new();
    for (r, n) in &a.cost {
        parts.extend(counted(*n, icon(*r)));
        said.push(amount(*r, *n));
    }
    if a.tokens != 0 {
        let kind = kind.expect("Content::check: an ability's tokens are the card's own");
        parts.extend(counted(a.tokens, kind.name()));
        said.push(format!("{} {} from this card", a.tokens, token(kind, a.tokens)));
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
    f
}

/// Above the art: income first, in one brown box and one "Each round, ...".
fn above_art(c: &CardDef, f: &mut Face) {
    let mut top_said: Vec<String> = Vec::new();

    let mut boxes: Vec<Vec<Part>> = Vec::new();
    let mut each_round: Vec<String> = Vec::new();
    let mut effects: Vec<(Vec<Part>, String)> = Vec::new();
    let mut plain = false;
    for op in &c.passives {
        match standing(op) {
            Standing::Income(parts, say) => {
                let this_plain = matches!(op, Op::Income { .. });
                match boxes.last_mut() {
                    Some(b) if plain && this_plain => b.extend(parts),
                    _ => boxes.push(parts),
                }
                plain = this_plain;
                each_round.push(say);
            }
            Standing::Effect(parts, say) => effects.push((parts, say)),
        }
    }
    if let Some(t) = &c.tokens {
        boxes.push(counted(t.per_round, t.kind.name()));
        each_round.push(format!("{} {} on this card", t.per_round, token(t.kind, t.per_round)));
    }
    for b in boxes {
        f.top.push(vec![Part::Income { income: b }]);
    }
    // A conversion is a sentence of its own: "1 chip and if you can ..." reads wrong.
    let (ifs, plain_round): (Vec<String>, Vec<String>) =
        each_round.into_iter().partition(|s| s.starts_with("if you can"));
    if !plain_round.is_empty() {
        top_said.push(sentence(&format!("each round, {}", listed(&plain_round))));
    }
    for s in ifs {
        top_said.push(sentence(&format!("each round, {s}")));
    }
    if c.slotless {
        // It costs no slot: the exception to "+1 slot" a reader needs, not a line of its own.
        let (parts, say) = effects.last_mut().expect("Content::check: a slotless card has an effect passive");
        parts.push(w("*"));
        say.push_str("* (*this card takes no slot)");
    }
    if !effects.is_empty() {
        let says: Vec<&str> = effects.iter().map(|(_, say)| say.as_str()).collect();
        top_said.push(sentence(&format!("effect: {}", says.join("; "))));
    }
    for (parts, _) in effects {
        f.top.push(parts);
    }
    if let Some(a) = &c.ability {
        let (mut left, paid) = action_cost(a, c.tokens.as_ref().map(|t| t.kind));
        left.push(w("\u{2192}"));
        let right = clauses(&a.effects, "get");
        left.extend(right.iter().flat_map(|c| c.icons.clone()));
        let says: Vec<String> = right.into_iter().map(|c| c.say).collect();
        // Call 2: a bare arrow is a free once-a-round action.
        let open = if paid.is_empty() { "once a round".to_string() } else { format!("pay {}", listed(&paid)) };
        f.top.push(left);
        top_said.push(sentence(&format!("action: {open}, {}", says.join(", "))));
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
        f.effect.push(vec![Part::Income { income: minus(*n, icon(*r)) }]);
        said.push(sentence(&format!("lower your {} income by {n}", track(*r))));
    }
    for cl in clauses(&c.effects, "gain") {
        f.effect.push(cl.icons);
        said.push(sentence(&cl.say));
    }
    f.text = said.join(" ");
}

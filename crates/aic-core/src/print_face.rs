//! 327: the printed faces beyond player cards (Matt, Nov 25: show the printed component, rules text on hover):
//! the AI boards, the adversary's event cards, projects, threats and research row, as icon lines generated from
//! their ops. `aic-sim docs` writes them to `web/card/print-faces.json`; `web/card/print.js` draws them.
//!
//! A player op's icons come from [`crate::face`] (one home); the adversary's ops print here, each one line in the
//! adversary's dark frame. Words stay the content's own hand-written `text` (CLAUDE.md, *Text beside ops*), carried
//! for the hover. A number is the solo game's resolved value (Matt, Nov 25): event cards print no Infiltrate count,
//! since solo's is 1.

use serde::Serialize;

use crate::content::{AiBoard, Content, CounterItem, Op, Tag};
use crate::face::{op_lines, resource_icon, Part};
use crate::op::{CreateAt, Facility, FalloutAt, RoundFlag, SiteType};
use crate::vocab::{Location, Side, ThreatTag};

fn w(s: &str) -> Part {
    Part::Word(s.to_string())
}

/// A number before its icon; no number means 1.
fn counted(n: i64, icon: &str) -> Vec<Part> {
    if n == 1 { vec![w(icon)] } else { vec![Part::Word(n.to_string()), w(icon)] }
}

/// Pay or lose: the minus, 1 too.
fn minus(n: i64, icon: &str) -> Vec<Part> {
    vec![Part::Word(format!("\u{2212}{n}")), w(icon)]
}

fn adv(parts: Vec<Part>) -> Vec<Part> {
    vec![Part::Adversary { adversary: parts }]
}

fn site_type(s: SiteType) -> &'static str {
    match s {
        SiteType::City => "city",
        SiteType::CoastalCity => "coastal",
        SiteType::Station => "station",
        SiteType::Factory => "factory",
        SiteType::Botnet => "botnet",
    }
}

fn facility(f: Facility) -> &'static str {
    match f {
        Facility::Station => "station",
        Facility::Factory => "factory",
    }
}

fn location(l: Location) -> &'static str {
    match l {
        Location::City => "city",
        Location::CoastalCity => "coastal",
        Location::Station => "station",
        Location::Factory => "factory",
    }
}

fn threat_tag(t: ThreatTag) -> Part {
    match t {
        // Universal is any tag: the empty ring.
        ThreatTag::Universal => w("#any"),
        t => Part::Word(format!("#{}", t.name())),
    }
}

fn tag(t: Tag) -> Part {
    Part::Word(format!("#{}", t.name()))
}

/// One adversary op as one icon line, in the dark frame (what the adversary does or the players lose). A shared
/// gain (a neutral site opening) prints outside it. An op with no printed line here returns nothing: the fast-lane
/// test `every_printed_face_has_its_lines` refuses content that reaches one.
pub fn adversary_line(op: &Op) -> Vec<Part> {
    match op {
        Op::Infiltrate { site } => adv(vec![w("agent"), w("onto"), w(site_type(*site))]),
        Op::Escalate { .. } => adv(vec![w("escalate")]),
        Op::Strike => adv(vec![w("troop"), w("strike")]),
        Op::Redeploy => adv(vec![w("troop"), w("redeploy")]),
        Op::CreateSite { kind } => vec![Part::Word("+1".into()), w(facility(*kind))],
        // The adversary's own income track: the raise arrow in its dark frame (the brown box is a player's income).
        Op::IncreaseIncome { compute } => {
            let mut p = vec![w("raise")];
            if *compute != 1 {
                p.push(Part::Word(compute.to_string()));
            }
            p.push(w("compute"));
            adv(p)
        }
        Op::SpreadBotnets { count } => adv(counted(*count, "botnet").into_iter().chain([w("spread")]).collect()),
        Op::Flag { name, .. } => adv(vec![w(match name {
            RoundFlag::Blackout => "blackout",
            RoundFlag::Sabotage => "sabotage",
            RoundFlag::Embargo => "embargo",
            RoundFlag::Cordon => "wall",
        })]),
        Op::Fallout { count, at } => {
            let mut p = counted(*count, "fallout");
            match at {
                FalloutAt::Location => {}
                FalloutAt::CorruptedCity => p.extend([w("near"), w("corrupt"), w("city")]),
                FalloutAt::CorruptedFactory => p.extend([w("near"), w("corrupt"), w("factory")]),
                FalloutAt::CorruptedStation => p.extend([w("near"), w("corrupt"), w("station")]),
            }
            adv(p)
        }
        Op::Compute { amount } | Op::ComputeEvery { amount } | Op::ThreatCompute { amount } => adv(counted(*amount, "compute")),
        Op::AdversaryResearch { amount, .. } => adv(counted(*amount, "research_track")),
        Op::Create { count, at, sites, .. } => {
            let mut p = counted(*count, "agent");
            p.push(w("onto"));
            if *sites > 1 {
                p.push(Part::Word(sites.to_string()));
            }
            p.push(w(match at {
                CreateAt::City => "city",
                CreateAt::CoastalCity => "coastal",
                CreateAt::Station => "station",
                CreateAt::Factory => "factory",
                CreateAt::Botnet => "botnet",
                CreateAt::Location => "hex",
                #[allow(unreachable_patterns)]
                _ => "site",
            }));
            adv(p)
        }
        Op::Requisition { resource, amount, cards, .. } | Op::Theft { resource, amount, cards, .. } => {
            let mut p = minus(*amount, resource_icon(*resource));
            p.push(w("OR"));
            p.extend(minus(*cards, "cards"));
            let mut line = adv(p);
            // A theft's compute comes either way: its own box beside the payment ("/" is "per").
            if let Op::Theft { compute, .. } = op {
                if *compute > 0 {
                    line.extend(adv(counted(*compute, "compute")));
                }
            }
            line
        }
        Op::DealThreats { count, .. } => adv(counted(*count, "threat")),
        Op::PlaceBotnet { count } => adv(counted(*count, "botnet")),
        Op::DestroyNode { count } => adv(minus(*count, "node")),
        // The factory under the token, with the nodes on its hex (no hidden harm).
        Op::DestroyFacility { .. } => adv([minus(1, "factory"), vec![w("ALL"), w("node")]].concat()),
        Op::DisableBuilding { count, .. } => adv(counted(*count, "facedown")),
        Op::Discard { count } => adv(minus(*count, "cards")),
        Op::DiscardMostExpensive => adv(minus(1, "cards")),
        Op::LoseTokens { amount } => adv(minus(*amount, "token")),
        Op::PullUnity { amount } => adv(minus(*amount, "sway")),
        Op::Sway { side: Side::Discord, amount, .. } => adv(counted(*amount, "discord")),
        Op::Sway { side: Side::Unity, amount, .. } => counted(*amount, "sway"),
        Op::FlipStation { .. } => adv(vec![w("corrupt"), w("station")]),
        Op::CorruptCity { .. } => adv(vec![w("corrupt"), w("city")]),
        Op::CorruptFactory => adv(vec![w("corrupt"), w("factory")]),
        Op::TakeSite { site } => adv(vec![w("corrupt"), w(facility(*site))]),
        Op::AdversaryFactory { count, .. } => adv(counted(*count, "factory")),
        // "Builds now": this round's Infiltrate lines land again.
        Op::AdversaryBuild => adv(vec![w("agent"), w("onto"), w("again")]),
        Op::AgentsOnNodes { count } => adv(vec![w("agent"), w("onto"), Part::Word(count.to_string()), w("node")]),
        // With at least `least` botnets: `count` of them come off, each `troops` troops.
        Op::BotnetTroops { count, troops, least } => {
            let mut p = Vec::new();
            if *least > 1 {
                p.extend([Part::Word(format!("\u{2265}{least}")), w("botnet"), w(":")]);
            }
            p.extend(minus(*count, "botnet"));
            p.extend(counted(*troops, "troop"));
            adv(p)
        }
        Op::FactoryTroops { count } => adv(counted(*count, "troop").into_iter().chain([w("/"), w("factory")]).collect()),
        Op::Troops { count, .. } => adv(counted(*count, "troop")),
        Op::LoseAmbassador => adv(minus(1, "ambassador")),
        Op::RecallAmbassadors => adv(vec![w("ambassador"), w("recall")]),
        Op::StripCubes { count } => adv(minus(*count, "research_cubes").into_iter().chain([w("megaproject")]).collect()),
        // One facility a country loses its hardening tokens: all of them there.
        Op::StripHarden => adv(vec![w("station"), w("/"), w("factory"), w(":"), w("ALL"), w("harden")]),
        Op::Clock { count } => adv(minus(*count, "deck")),
        Op::PlayersLose { resource, amount, .. } => adv(minus(*amount, resource_icon(*resource))),
        Op::LoseIncome { resource, amount, .. } => {
            vec![Part::Income { income: vec![w("lower"), Part::Word(amount.to_string()), w(resource_icon(*resource))] }]
        }
        _ => Vec::new(),
    }
}

fn lines(ops: &[Op]) -> Vec<Vec<Part>> {
    ops.iter().map(adversary_line).filter(|l| !l.is_empty()).collect()
}

/// A threat's ways to pay, a line each (OR between them on the face).
fn counter_line(items: &[CounterItem]) -> Vec<Part> {
    let mut out = Vec::new();
    for it in items {
        match *it {
            CounterItem::Pay { resource, amount } => out.extend(minus(amount, resource_icon(resource))),
            CounterItem::Sacrifice { sacrifice, amount } => {
                out.push(Part::Word(format!("\u{2212}{amount}")));
                out.push(tag(sacrifice));
            }
            CounterItem::NodesNear { nodes_near } => out.extend([Part::Word(format!("\u{2265}{nodes_near}")), w("node"), w("near")]),
        }
    }
    out
}

#[derive(Debug, Serialize)]
pub struct ThreatFace {
    pub name: String,
    pub tier: u8,
    pub tags: Vec<Part>,
    pub sites: Vec<Part>,
    /// Ways to pay it, a line each.
    pub pay: Vec<Vec<Part>>,
    /// What it does when nobody pays.
    pub unpaid: Vec<Vec<Part>>,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectFace {
    pub name: String,
    pub cost: i64,
    pub tags: Vec<Part>,
    pub lines: Vec<Vec<Part>>,
    pub text: String,
}

/// An event card's lines, each led by the beat it belongs to ("card", "infiltrate", "escalate", "strike", ...).
#[derive(Debug, Serialize)]
pub struct EventLine {
    pub beat: &'static str,
    pub icons: Vec<Part>,
}

#[derive(Debug, Serialize)]
pub struct EventFace {
    pub name: String,
    pub stage: u8,
    pub lines: Vec<EventLine>,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct Space {
    /// What covering it costs (energy), a line.
    pub pay: Vec<Part>,
    /// What it prints once uncovered: income, insight, tags, the capstone's name.
    pub get: Vec<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capstone: Option<(String, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PowerFace {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub line: Vec<Part>,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct BoardFace {
    pub name: String,
    pub credits: i64,
    pub income: Vec<Part>,
    pub tags: Vec<Part>,
    pub insight: i64,
    pub rule: (String, String),
    pub rule_line: Vec<Vec<Part>>,
    pub economy: Vec<Space>,
    pub play: Vec<Space>,
    pub powers: Vec<PowerFace>,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct ResearchSlot {
    pub id: String,
    pub name: String,
    pub icons: Vec<Part>,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct PrintFaces {
    pub boards: Vec<(String, BoardFace)>,
    pub events: Vec<(String, EventFace)>,
    pub projects: Vec<(String, ProjectFace)>,
    pub threats: Vec<(String, ThreatFace)>,
    /// The research row's slots in order, the loss slot last.
    pub research: Vec<ResearchSlot>,
}

fn space(u: &crate::content::Uncover) -> Space {
    let mut get = Vec::new();
    for (r, n) in &u.income {
        // An income step: its +N in the brown box, 1 too (the track's spaces read as a row of raises).
        get.push(Part::Income { income: vec![Part::Word(format!("+{n}")), w(resource_icon(*r))] });
    }
    get.extend(u.tags.iter().map(|t| tag(*t)));
    if u.keep2 {
        get.extend([w("less:1"), w("insight")]);
    }
    for (icons, _) in op_lines(&u.effects, "gain") {
        get.extend(icons);
    }
    Space {
        pay: if u.cost > 0 { minus(u.cost, "energy") } else { Vec::new() },
        get,
        capstone: u.capstone.as_ref().map(|c| (c.name.clone(), c.text.clone())),
        insight: u.insight,
    }
}

/// The special rule as one icon line: its passives through the building faces' standing lines; the Wall and
/// Patience, which only an innate holds, by hand; a rule with no ops (Breadth) as its building slots.
/// One icon group a passive (the Protector's Wall and Vigil side by side, no "/": that mark is "per").
fn rule_line(b: &AiBoard, slots: Option<i64>) -> Vec<Vec<Part>> {
    let mut out: Vec<Vec<Part>> = b
        .innate
        .passives
        .iter()
        .map(|op| match op {
            Op::Wall => vec![w("troop"), w("wall"), w("node")],
            Op::Patience => vec![w("insight"), w("carry")],
            op => crate::face::standing_icons(op),
        })
        .collect();
    if out.is_empty() {
        if let Some(n) = slots {
            out.push(counted(n, "slot"));
        }
    }
    out
}

fn board_face(b: &AiBoard, slots: Option<i64>) -> BoardFace {
    let powers = b
        .powers
        .iter()
        .map(|p| {
            let mut line = Vec::new();
            for (r, n) in &p.cost {
                line.extend(minus(*n, resource_icon(*r)));
            }
            for (t, n) in &p.tags {
                line.push(Part::Word(format!("\u{2212}{n}")));
                line.push(match t {
                    crate::vocab::TagOrAny::Any => w("#any"),
                    crate::vocab::TagOrAny::Is(t) => tag(*t),
                });
            }
            line.push(w("\u{2192}"));
            for (icons, _) in op_lines(&p.effects, "get") {
                line.extend(icons);
            }
            PowerFace { name: p.name.clone(), group: p.group.clone(), line, text: p.text.clone() }
        })
        .collect();
    let mut income = Vec::new();
    for (r, n) in &b.income {
        income.extend(counted(*n, resource_icon(*r)));
    }
    BoardFace {
        name: b.name.clone(),
        credits: b.credits,
        income: vec![Part::Income { income }],
        tags: b.tags.iter().map(|t| tag(*t)).collect(),
        insight: b.insight,
        rule: (b.innate.name.clone(), b.innate.text.clone()),
        rule_line: rule_line(b, slots),
        economy: b.economy_track.iter().map(space).collect(),
        play: b.play_track.iter().map(space).collect(),
        powers,
        text: b.text.clone(),
    }
}

/// A hover's words for solo (level 1, Matt, Nov 25: the resolved value, never "from level X"): a "(level 1+)" or
/// "(level 0+)" gate holds there, so it goes.
fn solo(text: &str) -> String {
    text.replace(" (level 1+)", "").replace(" (level 0+)", "")
}

/// Every printed face content holds. `slots`: each board's building slots, in board order (`board.slots`).
pub fn print_faces(c: &Content, slots: &[i64]) -> PrintFaces {
    let events = c
        .adversary
        .iter()
        .map(|e| {
            let mut out = Vec::new();
            // The Card beat: its threats, a medallion a pull, by tier.
            let mut tiers: Vec<u8> = Vec::new();
            for p in &e.pulls {
                if !tiers.contains(&p.tier) {
                    tiers.push(p.tier);
                }
            }
            for t in tiers {
                let n = e.pulls.iter().filter(|p| p.tier == t).count() as i64;
                let mut icons = counted(n, "threat");
                icons.push(Part::Word(format!("tier:{t}")));
                out.push(EventLine { beat: "card", icons: adv(icons) });
            }
            for op in e.standing.iter().chain(&e.twists) {
                let beat = match op {
                    Op::Infiltrate { .. } => "infiltrate",
                    Op::Strike => "strike",
                    Op::Redeploy => "redeploy",
                    _ => "card",
                };
                let icons = adversary_line(op);
                if !icons.is_empty() {
                    out.push(EventLine { beat, icons });
                }
            }
            // The spine (rulings, *Adversary*): every event card escalates last round's sites and factories.
            out.push(EventLine { beat: "escalate", icons: adv(vec![w("escalate")]) });
            (e.id.clone(), EventFace { name: e.name.clone(), stage: e.stage, lines: out, text: solo(&e.text) })
        })
        .collect();
    let projects = c
        .projects
        .iter()
        .map(|p| {
            (p.id.clone(), ProjectFace { name: p.name.clone(), cost: p.cost, tags: p.tags.iter().map(|t| threat_tag(*t)).collect(), lines: lines(&p.effects), text: solo(&p.text) })
        })
        .collect();
    let threats = c
        .threats
        .iter()
        .map(|t| {
            (
                t.id.clone(),
                ThreatFace {
                    name: t.name.clone(),
                    tier: t.tier,
                    tags: t.tags.iter().map(|g| threat_tag(*g)).collect(),
                    sites: t.locations.iter().map(|l| w(location(*l))).collect(),
                    pay: t.counters.iter().map(|c| counter_line(c)).collect(),
                    unpaid: lines(&t.effects),
                    text: solo(&t.text),
                },
            )
        })
        .collect();
    let board = &c.adversary_board;
    let mut research: Vec<ResearchSlot> = board
        .research
        .iter()
        .flat_map(|r| r.slots.iter())
        .filter_map(|id| board.icons.iter().find(|i| &i.id == id))
        .map(|i| ResearchSlot { id: i.id.clone(), name: i.name.clone(), icons: lines(&i.effects).concat(), text: solo(&i.text) })
        .collect();
    // The last slot: the players lose (the row's own text speaks of K and level lines, which solo resolves).
    research.push(ResearchSlot {
        id: "lose".into(),
        name: "The players lose".into(),
        icons: adv(vec![w("lose")]),
        text: format!("When the marker reaches slot {}, the players lose.", research.len() + 1),
    });
    PrintFaces { boards: c.ais.iter().enumerate().map(|(i, b)| (b.id.clone(), board_face(b, slots.get(i).copied()))).collect(), events, projects, threats, research }
}

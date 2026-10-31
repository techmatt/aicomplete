//! `aic-sim docs [--check]`: the parts of the rules docs generated from
//! content, configs and the op schema, so a number or a card has one home.
//!
//! `docs/rules.md` holds blocks between `<!-- generated: NAME -->` and
//! `<!-- /generated -->`; `docs/ops.md` and `docs/harms.md` are generated
//! whole (the ledger's checks fail the run); `web/card/faces.json`, every
//! card's face from its ops (`aic_core::face`), too. `--check` changes
//! nothing and exits 1 if anything is stale, or if `rules.md` backticks a
//! `module.param` or bare `snake_case` name that isn't a tunable. The fast
//! test lane runs the check. A block is a pure function of `Rules` or
//! `Content`, so a test can show it moves with its source.

use std::collections::BTreeMap;
use std::fs;
use std::process::exit;

use aic_core::config::ParamValue;
use aic_core::content::{content, AiBoard, Content, Cost, Op, Resource, Uncover};
use aic_core::face::card_face;
use aic_core::modules::adversary;
use aic_core::rules::Tunable;
use aic_core::vocab::DiscountOn;
use aic_core::Rules;

use crate::cli::{die, load, rules, Options};

const BASELINE: &str = "configs/baseline.json";
const RULES_MD: &str = "docs/rules.md";
const OPS_MD: &str = "docs/ops.md";
const OP_SOURCE: &str = "crates/aic-core/src/op.rs";
const HARMS_MD: &str = "docs/harms.md";
const FACES_JSON: &str = "web/card/faces.json";

/// The harms of the ledger, in its row order.
pub const HARMS: [&str; 14] = [
    "presence destroyed",
    "credits or income lost",
    "cards discarded",
    "insight lost",
    "ambassadors lost",
    "tokens stripped from cards",
    "buildings, stations or factories destroyed or seized",
    "production blacked out",
    "adversary pieces placed",
    "projects advanced (compute)",
    "extra threats",
    "Discord sway",
    "Unity pulled back",
    "actions or claims blocked",
];

/// The sources of the ledger, in its column order. The first five read
/// content; `RULE_HARMS` says what the rest do.
pub const SOURCES: [&str; 10] = [
    "tier-1 threats",
    "tier-2 threats",
    "Discord cards",
    "adversary twists",
    "adversary projects",
    "arrivals",
    "build and spread",
    "corruption",
    "war",
    "march",
];

/// What the board rules (the sources content doesn't print) do: source, harm,
/// and the rule, as the ledger's note. Update this when an adversary rule changes.
pub const RULE_HARMS: [(&str, &str, &str); 16] = [
    ("arrivals", "adversary pieces placed", "every matching site gains an ambassador"),
    ("arrivals", "ambassadors lost", "an arrival cancels a player ambassador there"),
    ("arrivals", "Discord sway", "an ambassador on a base sways Discord when it produces"),
    ("arrivals", "production blacked out", "an ambassador on a city or station diverts it"),
    ("build and spread", "adversary pieces placed", "cities build; surplus spreads; two meeting make a node"),
    ("build and spread", "projects advanced (compute)", "each node makes compute"),
    ("build and spread", "actions or claims blocked", "a node blocks site claims next to it"),
    ("build and spread", "insight lost", "an ambassador on a lone presence drains insight"),
    ("build and spread", "ambassadors lost", "a build or spread cancels a player ambassador there"),
    ("corruption", "presence destroyed", "presence on or next to a city that corrupts is destroyed"),
    ("corruption", "ambassadors lost", "player ambassadors on it are lost"),
    ("corruption", "production blacked out", "its energy goes to the adversary"),
    ("corruption", "Discord sway", "its energy sways Discord each round"),
    ("war", "presence destroyed", "a destroyed factory takes the presence on its hex"),
    ("war", "buildings, stations or factories destroyed or seized", "surplus troops destroy factories"),
    ("march", "buildings, stations or factories destroyed or seized", "a troop strikes player-owned buildings"),
];

/// Rows of [`HARMS`], for [`op_harms`].
const PRESENCE_DESTROYED: usize = 0;
const CREDITS_LOST: usize = 1;
const CARDS_DISCARDED: usize = 2;
const INSIGHT_LOST: usize = 3;
const AMBASSADORS_LOST: usize = 4;
const TOKENS_STRIPPED: usize = 5;
const BUILDINGS_DESTROYED: usize = 6;
const BLACKED_OUT: usize = 7;
const PIECES_PLACED: usize = 8;
const PROJECTS_ADVANCED: usize = 9;
const EXTRA_THREATS: usize = 10;
const DISCORD_SWAY: usize = 11;
const UNITY_PULLED: usize = 12;
const BLOCKED: usize = 13;

/// The harms one op does, as rows of [`HARMS`].
fn op_harms(op: &Op) -> Vec<usize> {
    match op {
        Op::DestroyPresence | Op::EachDestroyPresence { .. } | Op::LosePresenceOr { .. } => vec![PRESENCE_DESTROYED],
        Op::Sacrifice => vec![PRESENCE_DESTROYED, AMBASSADORS_LOST],
        Op::PlayersLose { resource: Resource::Credits, .. } => vec![CREDITS_LOST],
        Op::Requisition { .. } | Op::DiscardMostExpensive => vec![CARDS_DISCARDED],
        Op::LoseInsight { .. } => vec![INSIGHT_LOST],
        Op::LoseAmbassador => vec![AMBASSADORS_LOST],
        Op::LoseTokens { .. } => vec![TOKENS_STRIPPED],
        Op::FlipStation { .. } => vec![BUILDINGS_DESTROYED],
        Op::Flag { name } => match name.name() {
            "embargo" => vec![BLOCKED],
            _ => vec![BLACKED_OUT],
        },
        Op::Ambassadors { .. }
        | Op::Infiltrate { .. }
        | Op::NodeAmbassadors { .. }
        | Op::CleanCityAmbassadors { .. }
        | Op::AdversaryBuild
        | Op::AdversarySpread
        | Op::CityBuild { .. }
        | Op::CleanFactories { .. }
        | Op::StationBuild { .. } => vec![PIECES_PLACED],
        // 044: a corruption takes the presence and ambassadors around the city too.
        Op::CorruptCity => vec![PIECES_PLACED, PRESENCE_DESTROYED, AMBASSADORS_LOST],
        Op::Troops { destroy, .. } => {
            if *destroy {
                vec![PIECES_PLACED, PRESENCE_DESTROYED]
            } else {
                vec![PIECES_PLACED]
            }
        }
        Op::Compute { .. } => vec![PROJECTS_ADVANCED],
        Op::DealThreats { .. } => vec![EXTRA_THREATS],
        Op::Sway { side, .. } if side.name() == "discord" => vec![DISCORD_SWAY],
        Op::PullUnity { .. } => vec![UNITY_PULLED],
        _ => Vec::new(),
    }
}

/// One cell of the ledger.
#[derive(Clone, Copy)]
enum Mark {
    /// How many printed cards do it.
    Count(usize),
    /// A rule of the board does it.
    Rule,
}

impl Mark {
    /// What the checks count it as.
    fn weight(self) -> usize {
        match self {
            Mark::Count(n) => n,
            Mark::Rule => 1,
        }
    }

    /// What the ledger prints.
    fn text(self) -> String {
        match self {
            Mark::Count(n) => n.to_string(),
            Mark::Rule => "yes".to_string(),
        }
    }
}

/// `(harm, source)` marks.
fn harm_marks() -> BTreeMap<(usize, usize), Mark> {
    let c = content();
    let mut counts: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    let mut add = |src: usize, ops: &[Op]| {
        let mut seen = Vec::new();
        for o in ops {
            for h in op_harms(o) {
                if !seen.contains(&h) {
                    seen.push(h);
                    *counts.entry((h, src)).or_default() += 1;
                }
            }
        }
    };
    for t in &c.threats {
        add(t.tier as usize - 1, &t.effects);
    }
    for a in c.alignment.iter().filter(|a| a.side.name() == "discord") {
        let mut all = a.effects.clone();
        for s in &a.steps {
            all.extend(s.effects.iter().cloned());
        }
        add(2, &all);
    }
    for a in &c.adversary {
        add(3, &a.twists);
    }
    for p in &c.projects {
        add(4, &p.effects);
    }
    let mut out: BTreeMap<(usize, usize), Mark> =
        counts.into_iter().map(|(k, n)| (k, Mark::Count(n))).collect();
    for (src, harm, _) in RULE_HARMS {
        let s = SOURCES.iter().position(|x| *x == src).expect("a source");
        let h = HARMS.iter().position(|x| *x == harm).expect("a harm");
        out.insert((h, s), Mark::Rule);
    }
    out
}

/// What fails the ledger's distribution checks.
pub fn harm_failures() -> Vec<String> {
    let marks = harm_marks();
    let mut bad = Vec::new();
    for (h, name) in HARMS.iter().enumerate() {
        let n = (0..SOURCES.len()).filter(|&s| marks.contains_key(&(h, s))).count();
        if n < 2 {
            bad.push(format!("{name}: in {n} kind(s) of source, needs 2"));
        }
    }
    for s in 0..2 {
        let n = (0..HARMS.len()).filter(|&h| marks.contains_key(&(h, s))).count();
        if n < 5 {
            bad.push(format!("{}: {n} kinds of harm, needs 5", SOURCES[s]));
        }
    }
    let all: usize = (0..SOURCES.len())
        .filter_map(|s| marks.get(&(PRESENCE_DESTROYED, s)))
        .map(|m| m.weight())
        .sum();
    for s in 0..SOURCES.len() {
        if let Some(m) = marks.get(&(PRESENCE_DESTROYED, s)) {
            let n = m.weight();
            if 2 * n > all + 1 {
                bad.push(format!("{}: {n} of {all} printed presence destructions", SOURCES[s]));
            }
        }
    }
    bad
}

/// `docs/harms.md`: the ledger, from content and `RULE_HARMS`.
fn harms_md() -> String {
    let marks = harm_marks();
    let mut head = vec!["Harm"];
    head.extend(SOURCES);
    let rows = HARMS
        .iter()
        .enumerate()
        .map(|(h, name)| {
            let mut r = vec![name.to_string()];
            r.extend((0..SOURCES.len()).map(|s| marks.get(&(h, s)).map_or(String::new(), |m| m.text())));
            r
        })
        .collect();
    let notes: String = RULE_HARMS
        .iter()
        .map(|(s, h, why)| format!("- **{s}, {h}:** {why}.\n"))
        .collect();
    format!(
        "# The harms ledger\n\nGenerated by `aic-sim docs` from `content/` and `RULE_HARMS` in \
         `crates/aic-sim/src/docs.rs`: edit those, not this file. Keep it current: a change to \
         threats, Discord cards or adversary rules that adds or moves a harm updates \
         `RULE_HARMS` or `op_harms`.\n\nOne row per harm, one column per kind of source. A number \
         is how many printed cards do it (a threat, Discord card, twist or project counts once); \
         `yes` is a rule of the board. `Discord cards` covers both tiers; `march` is the adversary \
         troops' strikes, which take buildings only at `board.troop_presence` 4. The `yes` cells \
         are true at the baseline's numbers (`board.destruction_discord` and \
         `board.ambassador_discord` are 0, so neither war nor an ambassador on a clean city sways).\n\n{}\n\
         **Checks** (`aic-sim docs --check`, and a fast test): every harm in at least 2 kinds of \
         source; each threat tier at least 5 kinds of harm; no kind of source more than about half \
         of the printed presence destruction. Measured shares are in the 034 screen (`experiments/results/screen-034-clocks/clocks.md`).\n\n\
         ## The board's rules\n\n{notes}",
        table(&head, rows)
    )
}

/// `3 credits, 1 energy`.
fn costs(c: &[Cost]) -> String {
    let parts: Vec<String> = c.iter().map(|(r, n)| r.words(*n)).collect();
    parts.join(", ")
}

/// Escapes a table cell.
fn cell(s: &str) -> String {
    s.replace('|', "\\|")
}

fn table(head: &[&str], rows: Vec<Vec<String>>) -> String {
    let mut out = format!("| {} |\n|{}\n", head.join(" | "), "---|".repeat(head.len()));
    for r in rows {
        let r: Vec<String> = r.iter().map(|c| cell(c)).collect();
        out.push_str(&format!("| {} |\n", r.join(" | ")));
    }
    out
}

fn threats_block() -> String {
    let rows = content()
        .threats
        .iter()
        .map(|t| vec![t.name.clone(), t.tier.to_string(), t.text.clone(), costs(&t.counter)])
        .collect();
    table(&["Threat", "Tier", "If not countered", "Counter"], rows)
}

fn adversary_block() -> String {
    let rows = content()
        .adversary
        .iter()
        .map(|a| {
            vec![
                a.name.clone(),
                a.stage.to_string(),
                a.text.clone(),
                if a.escalation { "yes" } else { "" }.to_string(),
            ]
        })
        .collect();
    table(&["Card", "Stage", "Twist", "Escalation"], rows)
}

fn alignment_block() -> String {
    let rows = content()
        .alignment
        .iter()
        .map(|a| {
            let steps: Vec<String> = a.steps.iter().map(|s| s.at.to_string()).collect();
            vec![
                a.name.clone(),
                a.side.to_string(),
                a.tier.to_string(),
                a.text.clone(),
                steps.join(", "),
            ]
        })
        .collect();
    table(&["Card", "Side", "Tier", "Text", "Stronger after tips"], rows)
}

fn projects_block() -> String {
    let rows = content()
        .projects
        .iter()
        .map(|p| {
            vec![
                p.name.clone(),
                p.stage.to_string(),
                p.cost.to_string(),
                if p.ongoing { "ongoing" } else { "once" }.to_string(),
                p.text.clone(),
            ]
        })
        .collect();
    table(&["Project", "Stage", "Compute per player", "Resolves", "Text"], rows)
}

fn value(v: &ParamValue) -> String {
    match v {
        ParamValue::Int(n) => n.to_string(),
        ParamValue::List(l) => {
            let l: Vec<String> = l.iter().map(|n| n.to_string()).collect();
            l.join(", ")
        }
    }
}

fn tunables_block(r: &Rules) -> String {
    let rows = r
        .tunables()
        .iter()
        .map(|t| {
            vec![
                format!("`{}`", t.key),
                value(&t.value),
                format!("{}..{}", t.min, t.max),
                t.doc.to_string(),
            ]
        })
        .collect();
    table(&["Parameter", "Baseline", "Range", "What it does"], rows)
}

/// The baseline's tunable `key` (`module.param`).
fn tunable(r: &Rules, key: &str) -> Tunable {
    r.tunables()
        .into_iter()
        .find(|t| t.key == key)
        .unwrap_or_else(|| die(&format!("docs: no tunable {key}")))
}

fn int(r: &Rules, key: &str) -> i64 {
    match tunable(r, key).value {
        ParamValue::Int(n) => n,
        ParamValue::List(_) => die(&format!("docs: {key} is a list")),
    }
}

fn list(r: &Rules, key: &str) -> Vec<i64> {
    match tunable(r, key).value {
        ParamValue::Int(n) => vec![n],
        ParamValue::List(l) => l,
    }
}

/// A tunable as the tables print it: `` `hold_base` 3 ``, or with its module
/// where the bare name says little (`` `alignment.node` 2 ``).
fn tick(r: &Rules, key: &str) -> String {
    let name = key.split_once('.').map_or(key, |(_, p)| p);
    let shown = if name.contains('_') && name != "free_city" { name } else { key };
    format!("`{shown}` {}", value(&tunable(r, key).value))
}

/// `+1 credit, +2 chips`. `Resource::amount` keeps "credits" plural, as content
/// text does; a track space reads better singular.
fn gains(c: &[Cost]) -> String {
    let parts: Vec<String> = c.iter().map(|(r, n)| format!("+{}", r.words(*n))).collect();
    parts.join(", ")
}

/// `Industry`, for a tag's lower-case name.
fn title(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or(String::new(), |f| f.to_uppercase().chain(c).collect())
}

fn numbers(l: &[u8]) -> String {
    if l.is_empty() {
        return "—".to_string();
    }
    let l: Vec<String> = l.iter().map(|n| n.to_string()).collect();
    l.join(", ")
}

/// The printed country pieces.
fn countries_block(c: &Content) -> String {
    let rows = c
        .map
        .countries
        .iter()
        .enumerate()
        .map(|(i, p)| {
            vec![
                i.to_string(),
                p.name.clone(),
                numbers(&p.cities),
                numbers(&p.ocean),
                numbers(&p.stations),
                format!("{} ({})", numbers(&p.industry), numbers(&p.factories)),
                p.presence.to_string(),
                format!("{}, {} cubes: {}", p.megaproject.name, p.megaproject.cubes, costs(&p.megaproject.benefit)),
            ]
        })
        .collect();
    table(
        &[
            "#",
            "Name",
            "Cities (corrupted first)",
            "Ocean",
            "Station (node)",
            "Industrial zones (setup factories)",
            "Starting presence",
            "Megaproject (benefit per owned station)",
        ],
        rows,
    )
}

/// Cards of each stage a game dealt at `difficulty` holds: the engine's own
/// deal, read back, so this can't drift from the rule (the rounds a game can
/// last are the cards dealt).
fn dealt(r: &Rules, difficulty: i64) -> [usize; 3] {
    let mut config = (*r.config).clone();
    config
        .apply_override(&format!("adversary.difficulty={difficulty}"))
        .unwrap_or_else(|e| die(&format!("docs: {e}")));
    let level = Rules::new(&config).unwrap_or_else(|e| die(&format!("docs: difficulty {difficulty}: {e}")));
    let g = level.new_game(1);
    let mut out = [0; 3];
    for pile in [adversary::DECK, adversary::NEXT, adversary::CURRENT, adversary::DISCARD] {
        for id in g.pile(pile) {
            let card = content().adversary.iter().find(|c| c.id == id.as_ref());
            let stage = card.unwrap_or_else(|| die("docs: an adversary card not in content")).stage;
            out[stage as usize - 1] += 1;
        }
    }
    out
}

/// The round cap and the adversary deck's shape at each difficulty.
fn round_cap_block(r: &Rules) -> String {
    let d = tunable(r, "adversary.difficulty");
    let rows = (d.min..=d.max)
        .map(|level| {
            let n = dealt(r, level);
            vec![
                level.to_string(),
                n[0].to_string(),
                n[1].to_string(),
                n[2].to_string(),
                (n[0] + 1).to_string(),
                n.iter().sum::<usize>().to_string(),
            ]
        })
        .collect();
    table(
        &["Difficulty", "Stage I cards", "Stage II", "Stage III", "Stage II from round", "Round cap"],
        rows,
    )
}

/// What each difficulty level adds.
fn difficulty_block(r: &Rules) -> String {
    let t = |keys: &[&str]| {
        let k: Vec<String> = keys.iter().map(|k| tick(r, k)).collect();
        k.join(", ")
    };
    let rows: [(&str, String); 5] = [
        ("Base rules", String::new()),
        (
            "Extra threats each round in all, countries in turn",
            t(&["adversary.extra_threats", "adversary.counter_bonus", "adversary.counter_bonus_all"]),
        ),
        (
            "A stage-I card swapped for a stage-III card: stage II (escalation, tier-2 threats) sooner and a harder end, at the same round cap (the table above)",
            t(&["adversary.stage1_removed", "adversary.stage3_added"]),
        ),
        (
            "Larger builds: each corrupted city gains more, and a stage-II card removed (a shorter round cap)",
            t(&["adversary.build_bonus", "adversary.stage2_removed"]),
        ),
        (
            "A still shorter Discord deck (027: was a smaller supply, which does nothing now it is a cap): the removed cards count as tips already made, so every printed step still acts (034); tier-2 threats sooner; optionally more corrupted cities per country at setup",
            t(&["adversary.discord_removed", "adversary.tier2_early", "adversary.extra_corrupted"]),
        ),
    ];
    let rows = rows
        .into_iter()
        .enumerate()
        .map(|(i, (adds, keys))| vec![i.to_string(), adds.to_string(), keys])
        .collect();
    table(&["Level", "Adds", "Tunable"], rows)
}

/// `(board, its slots)`: slots are per seat in `ai.boards` order; a board no
/// seat plays has none.
fn board_slots(r: &Rules, board: usize) -> String {
    let slots = list(r, "ai.slots");
    list(r, "ai.boards")
        .iter()
        .position(|b| *b == board as i64)
        .map_or(String::new(), |seat| slots[seat % slots.len()].to_string())
}

/// `Foundry Core (Industry): +1 credit`.
fn start_building(c: &Content, b: &AiBoard) -> String {
    let Some(id) = &b.start else {
        return String::new();
    };
    let Some(card) = c.cards.iter().find(|d| d.id == *id) else {
        die(&format!("docs: no card {id}"))
    };
    let tags: Vec<String> = card.tags.iter().map(|t| title(t.name())).collect();
    let income: Vec<Cost> = card
        .passives
        .iter()
        .filter_map(|p| match p {
            Op::Income { resource, amount } => Some((*resource, *amount)),
            _ => None,
        })
        .collect();
    format!("{} ({}): {}", card.name, tags.join(", "), gains(&income))
}

/// What each AI board prints before its tracks.
fn boards_block(r: &Rules, c: &Content) -> String {
    let rows = c
        .ais
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let mut printed = vec![(Resource::Credits, b.credits)];
            printed.extend(b.income.iter().copied());
            vec![
                b.name.clone(),
                costs(&printed),
                start_building(c, b),
                board_slots(r, i),
                b.presence.len().to_string(),
                b.presence2.len().to_string(),
                (b.presence.len() + b.presence2.len()).to_string(),
            ]
        })
        .collect();
    table(
        &[
            "Board",
            "Printed income",
            "Starting building",
            "Slots",
            "Track tokens",
            "Second track",
            "Tokens in all",
        ],
        rows,
    )
}

/// Each board's innate, discount, growth and unique cards.
fn board_details_block(c: &Content) -> String {
    let name = |id: &String| {
        c.cards
            .iter()
            .find(|d| d.id == *id)
            .map_or_else(|| die(&format!("docs: no card {id}")), |d| d.name.clone())
    };
    let rows = c
        .ais
        .iter()
        .map(|b| {
            let discounts: Vec<String> = b
                .discounts
                .iter()
                .map(|d| match d.tag {
                    DiscountOn::Building => format!("{} on buildings {}:1", d.resource, d.rate),
                    DiscountOn::Tag(t) => format!("{} on {} cards {}:1", d.resource, title(t.name()), d.rate),
                })
                .collect();
            let growth: Vec<String> = b.growth.iter().map(|g| g.name.clone()).collect();
            let cards: Vec<String> = b.cards.iter().map(name).collect();
            vec![
                b.name.clone(),
                format!("*{}*: {}", b.innate.name, b.innate.text),
                discounts.join(", "),
                growth.join("; "),
                cards.join(", "),
            ]
        })
        .collect();
    table(&["Board", "Innate", "Discount", "Growth options", "Unique cards"], rows)
}

/// One space of a presence track.
fn space(u: &Uncover) -> String {
    let mut parts = vec![gains(&u.income)];
    if u.keep2 {
        parts.push("**keep upgrade**".to_string());
    }
    let mut out = parts.join(", ");
    if u.cost != 0 {
        out.push_str(&format!(" (cost {})", u.cost));
    }
    out
}

/// A presence track of every board, space by space. `first` labels space 1.
fn track_block(boards: &[AiBoard], track: fn(&AiBoard) -> &[Uncover], first: &str) -> String {
    let spaces = boards.iter().map(|b| track(b).len()).max().unwrap_or(0);
    let rows = (0..spaces)
        .map(|i| {
            let label = if i == 0 { first.to_string() } else { (i + 1).to_string() };
            let mut row = vec![label];
            row.extend(boards.iter().map(|b| track(b).get(i).map_or(String::new(), space)));
            row
        })
        .collect();
    let mut head = vec!["Space"];
    head.extend(boards.iter().map(|b| b.name.as_str()));
    table(&head, rows)
}

fn presence_track_block(c: &Content) -> String {
    track_block(&c.ais, |b| &b.presence, "1 (on the board at setup)")
}

fn presence_track2_block(c: &Content) -> String {
    track_block(&c.ais, |b| &b.presence2, "1")
}

fn powers_block(c: &Content) -> String {
    let mut rows = Vec::new();
    for b in &c.ais {
        for p in &b.powers {
            let mut cost: Vec<String> = p.tags.iter().map(|(t, n)| format!("{n} {}", title(&t.to_string()).replace("Any", "any"))).collect();
            cost.extend(p.cost.iter().map(|(r, n)| r.words(*n)));
            rows.push(vec![b.name.clone(), p.name.clone(), cost.join(" + "), p.text.clone()]);
        }
    }
    table(&["Board", "Power", "Cost", "Effect"], rows)
}

/// The standard actions and projects, with their prices.
fn standard_actions_block(r: &Rules) -> String {
    let t = |key: &str| tick(r, key);
    let rows: Vec<[String; 3]> = vec![
        [
            "Place presence".to_string(),
            format!("{}, plus the step's printed cost (§7)", t("board.presence_energy")),
            format!("{} a round (046): the next token off your track (uncovering its space) on a land hex next to your presence, with no corruption marker, 1 presence per hex (**locked**)", t("board.presence_action_cap")),
        ],
        [
            "Build a research station".to_string(),
            format!("{} + {} chips", t("board.build_energy"), t("board.station_chips")),
            "on an open hex in reach that is not an industrial zone; it is yours (your cube)".to_string(),
        ],
        [
            "Remove an adversary ambassador".to_string(),
            t("board.remove_energy"),
            "in reach".to_string(),
        ],
        [
            "Clear a corruption marker".to_string(),
            t("board.clear_energy"),
            format!(
                "in reach (041): sway {} toward Unity, gain {} credits",
                t("board.clear_unity"),
                t("board.clear_credits")
            ),
        ],
        [
            "Move a player ambassador".to_string(),
            t("board.move_energy"),
            "from your supply or the board onto a piece in reach (not the hex it comes from, 020)".to_string(),
        ],
        [
            "Gain a player ambassador".to_string(),
            t("board.ambassador_credits"),
            "into your supply; a standard project too".to_string(),
        ],
        [
            "Align a clean city".to_string(),
            format!("{} + 1 ambassador", t("board.align_energy")),
            "in reach, with a site claim: your cube on it".to_string(),
        ],
        [
            "Harden a building (the Protector; a card can grant it)".to_string(),
            format!("{} energy a token", t("board.harden_energy")),
            "a player-owned station or factory in reach that is not corrupted, one a troop can strike (022: any building, the adversary's too)".to_string(),
        ],
        [
            "Sell chips".to_string(),
            "1 chip".to_string(),
            format!("gain {} (021: replaces buying chips)", t("board.chips_credits")),
        ],
        [
            "Sell a card".to_string(),
            format!("gain {}", t("board.sell_credits")),
            "from your hand".to_string(),
        ],
        [
            "The rich action (`presence_buy`, 034)".to_string(),
            format!(
                "{} credits, + {} for each earlier use by you this round, + {} an energy of the step's printed cost",
                t("board.presence_credits"),
                t("board.presence_rise"),
                t("board.energy_credits")
            ),
            format!(
                "from stage {} (`presence_stage`): as Place presence, no energy. Worse per token than Place presence, worth it only with spare credits (024: a cheap project from round 1)",
                int(r, "board.presence_stage")
            ),
        ],
        [
            "Project: harden (`harden_buy`)".to_string(),
            format!("{} credits", t("board.harden_credits")),
            "any player: a hardening token on a player-owned station or factory in reach".to_string(),
        ],
        [
            "Project: buy energy (`energy`)".to_string(),
            format!("{} credits", t("board.energy_credits")),
            "gain 1 energy".to_string(),
        ],
        [
            "Project: drain (`drain`)".to_string(),
            format!("{} credits", t("board.drain_credits")),
            format!("remove up to {} compute from one visible adversary project", t("board.drain_amount")),
        ],
    ];
    table(&["Action", "Cost", "What"], rows.into_iter().map(|r| r.to_vec()).collect())
}

/// Where Sway comes from, with the amounts.
fn sway_sources_block(r: &Rules) -> String {
    let t = |key: &str| tick(r, key);
    let rows: Vec<[String; 3]> = vec![
        [
            "Unity".to_string(),
            "a player ambassador on a producing base / factory".to_string(),
            format!("{} / {}", t("board.base_unity"), t("board.factory_unity")),
        ],
        ["Unity".to_string(), "removing an adversary troop".to_string(), t("alignment.troop")],
        [
            "Unity".to_string(),
            "freeing a corrupted city (§2; or a tier-2 ultimate)".to_string(),
            t("alignment.free_city"),
        ],
        ["Unity".to_string(), "removing a node".to_string(), t("alignment.node")],
        [
            "Unity".to_string(),
            "cards (Peace Dividend, Planetary Referendum, Martial Cordon), AI boards, Unity cards".to_string(),
            "card text".to_string(),
        ],
        [
            "Discord".to_string(),
            "energy diverted (a corrupted city, an ambassador on a city)".to_string(),
            format!(
                "{} per unit of a corrupted city's, {} of an ambassador's",
                t("board.divert_discord"),
                t("board.ambassador_discord")
            ),
        ],
        ["Discord".to_string(), "an adversary ambassador on a producing base".to_string(), t("board.base_discord")],
        [
            "Discord".to_string(),
            "a building destroyed in war (not by an adversary troop; 018)".to_string(),
            t("board.destruction_discord"),
        ],
        [
            "Discord".to_string(),
            "adversary cubes on a completed megaproject".to_string(),
            format!("1 per {}", t("board.project_discord_per")),
        ],
        [
            "Discord".to_string(),
            "an uncountered threat, the antimatter action, twists, Discord cards, adversary projects".to_string(),
            format!("{}, {}, card text", t("alignment.threat_discord"), t("alignment.antimatter_discord")),
        ],
        [
            "Discord".to_string(),
            "the adversary at the start of every round".to_string(),
            t("alignment.adversary_discord"),
        ],
    ];
    table(&["Toward", "From", "Amount"], rows.into_iter().map(|r| r.to_vec()).collect())
}

/// What stops counting against the purge at each `unity_ease` threshold.
fn purge_ease_block(r: &Rules) -> String {
    const STEPS: [(&str, &str); 3] = [
        ("first", "adversary ambassadors stop counting"),
        ("second", "adversary troops stop counting"),
        ("third", "**nodes** stop counting (019, candidate: with node removal rare, Unity buys the win)"),
    ];
    let mut out = String::new();
    for (tips, (which, what)) in list(r, "alignment.unity_ease").iter().zip(STEPS) {
        out.push_str(&format!("- {which} threshold ({tips} Unity tips): {what};\n"));
    }
    out.push_str("- corrupted cities always count.\n");
    out
}

/// The opening paragraph for new players.
fn new_players_block(r: &Rules) -> String {
    let c = content();
    let n = |key: &str| value(&tunable(r, key).value);
    let slots = list(r, "ai.slots");
    let range = |l: &[i64]| {
        let (lo, hi) = (l.iter().min().copied().unwrap_or(0), l.iter().max().copied().unwrap_or(0));
        if lo == hi {
            lo.to_string()
        } else {
            format!("{lo}-{hi}")
        }
    };
    let insight: Vec<i64> = c.cards.iter().map(|d| d.insight).collect();
    format!(
        "**For new players.** In the first round you are dealt {} cards and keep {} free (more \
         at {} credits each). From then on, each round you build a hand of {} cards one at a time, \
         choosing for each whether it comes from the cheaper tier-1 deck or the stronger tier-2 \
         deck; you take one card and pass the rest to your neighbour until the hands are gone, then \
         pay insight to keep what you drafted ({} for tier 1, {} for tier 2). Insight is this round's \
         attention: what you don't spend is lost. Playing a card costs its credits and {} insight, \
         from the same insight as keeps. Buildings go into your AI board's {} slots; once they are \
         full, a new building goes on top of an old one, which stops working but keeps its tags. \
         Programs stay in front of you; events are used and discarded. Every card in front of you \
         shows tags (Science, Industry, Cyber, Power, Diplomacy, Security): they unlock some cards, \
         and once a round you can sacrifice cards with the right tags to use your AI board's power.\n",
        n("cards.opening_deal"),
        n("cards.opening_free"),
        n("cards.opening_keep_credits"),
        n("cards.deal"),
        n("cards.keep_tier1"),
        n("cards.keep_tier2"),
        range(&insight),
        range(&slots),
    )
}

/// How the game ends, for new players.
fn game_end_block(r: &Rules) -> String {
    let ease = list(r, "alignment.unity_ease");
    let easing = match ease.as_slice() {
        [_, a, b, ..] => format!(
            "{a} Unity tips and the purge asks only for no nodes and no corrupted cities; {b} and it \
             asks only for no corrupted cities."
        ),
        [.., a] => format!("{a} Unity tips and the purge asks only for no nodes and no corrupted cities."),
        [] => String::new(),
    };
    format!(
        "**How the game ends, for new players.** You win by purging the board or by drawing the last \
         Unity card. Pressure comes every round: the face-up adversary card names a kind of site, \
         and every such site gains an adversary ambassador, so you always know where the next wave \
         lands. Ambassadors pile up, pairs become nodes, {} on a city corrupt it, and corrupted \
         cities and uncountered threats push the Alignment track toward Discord. Each time the track \
         tips, a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, \
         you lose. Pushing back works the same way in your favour. Mass your ambassadors on a \
         corrupted city to free it, which sways toward Unity, and each Unity tip brings a good card. \
         The Unity deck is small too: a game deals only {}, and drawing the last wins at once. \
         {easing} You also lose if any player has no presence left on the board. \
         Threats, war, corruption and late adversary and Discord cards destroy it, a token at a time, and \
         a lost token leaves the game and marks its hex, so you can't act from there any more; its income \
         stays. In stage I your last one is safe. The adversary deck is a round cap too, {} rounds at most, \
         though games rarely reach it. So there are three different pressures: the Discord deck (politics), \
         your presence (the board) and the cap.\n",
        int(r, "adversary.corrupt_at"),
        list(r, "alignment.unity_cards").iter().sum::<i64>(),
        dealt(r, 0).iter().sum::<usize>(),
    )
}

fn snake(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// One op's source: its doc comment and name, read from the `Op` enum.
struct OpDoc {
    section: String,
    name: String,
    doc: String,
}

/// Reads `enum Op` from `op.rs`: section comments, doc comments, variants.
fn op_docs(src: &str) -> Vec<OpDoc> {
    let body = src
        .split_once("pub enum Op {")
        .expect("enum Op in op.rs")
        .1
        .split_once("\n}\n")
        .expect("enum Op ends")
        .0;
    let (mut out, mut doc, mut section) = (Vec::new(), Vec::<String>::new(), String::new());
    for line in body.lines() {
        let Some(l) = line.strip_prefix("    ") else {
            continue;
        };
        if let Some(s) = l.strip_prefix("// ---------- ") {
            section = s.trim_end_matches(" ----------").to_string();
        } else if let Some(d) = l.strip_prefix("/// ") {
            doc.push(d.to_string());
        } else if l.starts_with(|c: char| c.is_uppercase()) {
            let name: String = l.chars().take_while(|c| c.is_alphanumeric()).collect();
            out.push(OpDoc {
                section: section.clone(),
                name: snake(&name),
                doc: doc.join(" "),
            });
            doc.clear();
        }
    }
    out
}

/// How many times each op is printed in content.
fn op_uses() -> BTreeMap<String, usize> {
    fn walk(v: &serde_json::Value, out: &mut BTreeMap<String, usize>) {
        match v {
            serde_json::Value::Object(o) => {
                if let Some(n) = o.get("op").and_then(|n| n.as_str()) {
                    *out.entry(n.to_string()).or_default() += 1;
                }
                o.values().for_each(|x| walk(x, out));
            }
            serde_json::Value::Array(a) => a.iter().for_each(|x| walk(x, out)),
            _ => {}
        }
    }
    let c = content();
    let mut out = BTreeMap::new();
    fn add<T: serde::Serialize>(ops: &[T], out: &mut BTreeMap<String, usize>) {
        walk(&serde_json::to_value(ops).expect("ops serialize"), out);
    }
    for d in &c.cards {
        add(&d.effects, &mut out);
        add(&d.passives, &mut out);
        if let Some(a) = &d.ability {
            add(&a.effects, &mut out);
        }
    }
    for t in &c.threats {
        add(&t.effects, &mut out);
    }
    for a in &c.adversary {
        add(&a.twists, &mut out);
    }
    for a in &c.alignment {
        add(&a.effects, &mut out);
        for s in &a.steps {
            add(&s.effects, &mut out);
        }
    }
    for p in &c.projects {
        add(&p.effects, &mut out);
    }
    for b in &c.ais {
        add(&b.innate.passives, &mut out);
        for g in &b.growth {
            add(&g.effects, &mut out);
        }
        for p in &b.powers {
            add(&p.effects, &mut out);
        }
    }
    out
}

fn ops_md(src: &str) -> String {
    let uses = op_uses();
    let mut out = String::from(
        "# Ops\n\nGenerated by `aic-sim docs` from `crates/aic-core/src/op.rs` and `content/`; \
         don't edit. An op is one thing content can print (`{\"op\": \"gain\", ...}`). \
         **Printed** counts it across all content: 0 means rules code with nothing using it yet.\n",
    );
    let mut section = String::new();
    let mut rows = Vec::new();
    let mut flush = |section: &str, rows: &mut Vec<Vec<String>>| {
        if !rows.is_empty() {
            out.push_str(&format!("\n## {section}\n\n"));
            out.push_str(&table(&["Op", "Printed", "What it does"], std::mem::take(rows)));
        }
    };
    for o in op_docs(src) {
        if o.section != section {
            flush(&section, &mut rows);
            section = o.section.clone();
        }
        rows.push(vec![
            format!("`{}`", o.name),
            uses.get(&o.name).copied().unwrap_or(0).to_string(),
            o.doc,
        ]);
    }
    flush(&section, &mut rows);
    out
}

/// Every card's face, one card a line, so a change shows as that card's diff.
fn faces_json() -> String {
    let rows: Vec<String> = content()
        .cards
        .iter()
        .map(|c| {
            let face = serde_json::to_string(&card_face(c)).expect("a face is JSON");
            format!(" {}: {face}", serde_json::to_string(&c.id).expect("an id is JSON"))
        })
        .collect();
    format!("{{
{}
}}
", rows.join(",
"))
}

const OPEN: &str = "<!-- generated: ";
const CLOSE: &str = "<!-- /generated -->";

/// `text` with every generated block replaced by what `block` says; panics
/// (dies) on an unknown name.
fn fill(text: &str, block: &dyn Fn(&str) -> Option<String>) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(i) = rest.find(OPEN) {
        let after = &rest[i + OPEN.len()..];
        let Some(end) = after.find(" -->") else {
            die("docs: unclosed generated marker")
        };
        let name = &after[..end];
        let Some(body) = block(name) else {
            die(&format!("docs: unknown generated block {name:?}"))
        };
        let Some(close) = after.find(CLOSE) else {
            die(&format!("docs: block {name:?} has no {CLOSE}"))
        };
        out.push_str(&rest[..i]);
        out.push_str(&format!("{OPEN}{name} -->\n{body}{CLOSE}"));
        rest = &after[close + CLOSE.len()..];
    }
    out.push_str(rest);
    out
}

/// Dotted names rules.md may print that are counters, not tunables or phases.
const COUNTERS: [&str; 1] = ["threats.short"];

/// Bare `snake_case` names rules.md may print that are no tunable: op and
/// action names, content fields, and retired parameters in its history.
const NOT_TUNABLES: [&str; 16] = [
    "cash_out",
    "clear_city",
    "free_city_at",
    "free_keep",
    "gain_per_tokens",
    "harden_buy",
    "legal_actions",
    "move_ambassadors",
    "move_one",
    "node_energy",
    "per_round",
    "presence_buy",
    "purge_round",
    "shared_row",
    "spread_ambassadors",
    "unity_win",
];

/// Backticked `module.name` words in `text` that are no tunable, phase or
/// counter of the baseline, and bare `snake_case` words that are no tunable.
fn unknown_params(text: &str, r: &Rules) -> Vec<String> {
    let mut known: Vec<String> = r.tunables().into_iter().map(|t| t.key).collect();
    let modules: Vec<String> = known
        .iter()
        .filter_map(|k| k.split('.').next().map(str::to_string))
        .collect();
    for m in aic_core::describe_modules().as_array().expect("modules") {
        for v in m["variants"].as_array().expect("variants") {
            for p in v["phases"].as_array().expect("phases") {
                known.push(format!("{}.{}", m["id"].as_str().unwrap(), p["name"].as_str().unwrap()));
            }
        }
    }
    known.extend(COUNTERS.iter().map(|c| c.to_string()));
    let mut out = Vec::new();
    for tick in text.split('`').skip(1).step_by(2) {
        if let Some((m, p)) = tick.split_once('.') {
            let word = !p.is_empty() && p.chars().all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit());
            if modules.iter().any(|x| x == m) && word && !known.iter().any(|k| k == tick) {
                out.push(tick.to_string());
            }
        } else if tick.contains('_')
            && tick.chars().all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit())
            && !NOT_TUNABLES.contains(&tick)
            && !known.iter().any(|k| k.split_once('.').is_some_and(|(_, p)| p == tick))
        {
            out.push(tick.to_string());
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Regenerates the docs, or with `--check` reports what is stale.
pub fn run(o: &Options) {
    let r = rules(BASELINE, &load(BASELINE, &[]));
    let src = fs::read_to_string(OP_SOURCE).unwrap_or_else(|e| die(&format!("{OP_SOURCE}: {e}")));
    let block = |name: &str| match name {
        "threats" => Some(threats_block()),
        "adversary" => Some(adversary_block()),
        "alignment" => Some(alignment_block()),
        "projects" => Some(projects_block()),
        "tunables" => Some(tunables_block(&r)),
        "countries" => Some(countries_block(content())),
        "round_cap" => Some(round_cap_block(&r)),
        "difficulty" => Some(difficulty_block(&r)),
        "boards" => Some(boards_block(&r, content())),
        "board_details" => Some(board_details_block(content())),
        "presence_track" => Some(presence_track_block(content())),
        "presence_track2" => Some(presence_track2_block(content())),
        "powers" => Some(powers_block(content())),
        "standard_actions" => Some(standard_actions_block(&r)),
        "sway_sources" => Some(sway_sources_block(&r)),
        "purge_ease" => Some(purge_ease_block(&r)),
        "new_players" => Some(new_players_block(&r)),
        "game_end" => Some(game_end_block(&r)),
        _ => None,
    };
    let old = fs::read_to_string(RULES_MD).unwrap_or_else(|e| die(&format!("{RULES_MD}: {e}")));
    let on_disk = |path: &str| fs::read_to_string(path).unwrap_or_default();
    // Path, what it should hold, what it holds.
    let files = [
        (FACES_JSON, faces_json(), on_disk(FACES_JSON)),
        (RULES_MD, fill(&old, &block), old.clone()),
        (OPS_MD, ops_md(&src), on_disk(OPS_MD)),
        (HARMS_MD, harms_md(), on_disk(HARMS_MD)),
    ];
    let stale: Vec<&(&str, String, String)> = files.iter().filter(|(_, new, old)| new != old).collect();
    let failures = harm_failures();
    for f in &failures {
        eprintln!("{HARMS_MD}: {f}");
    }
    if o.check {
        let bad = unknown_params(&old, &r);
        for b in &bad {
            eprintln!("{RULES_MD}: `{b}` is not a tunable");
        }
        for (path, ..) in &stale {
            eprintln!("{path} is stale: run aic-sim docs");
        }
        if !stale.is_empty() || !bad.is_empty() || !failures.is_empty() {
            exit(1);
        }
        return;
    }
    for (path, new, _) in stale {
        fs::write(path, new).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aic_core::Config;

    const BASELINE_JSON: &str = include_str!("../../../configs/baseline.json");

    fn baseline(overrides: &[&str]) -> Rules {
        let mut c = Config::from_json(BASELINE_JSON).unwrap();
        for o in overrides {
            c.apply_override(o).unwrap();
        }
        Rules::new(&c).unwrap()
    }

    /// A row of `block` whose first cell is `first`, as its cells.
    fn row(block: &str, first: &str) -> Vec<String> {
        let line = block
            .lines()
            .find(|l| l.starts_with(&format!("| {first} |")))
            .unwrap_or_else(|| panic!("no row {first} in {block}"));
        line.trim_matches('|').split(" | ").map(|c| c.trim().to_string()).collect()
    }

    #[test]
    fn the_round_cap_comes_from_the_engines_deal() {
        let base = round_cap_block(&baseline(&[]));
        // At the baseline: 4, 5, 2 cards; difficulty 2 swaps a stage-I card for a stage-III.
        assert_eq!(row(&base, "0"), ["0", "4", "5", "2", "5", "11"]);
        assert_eq!(row(&base, "2"), ["2", "3", "5", "3", "4", "11"]);
        let shorter = round_cap_block(&baseline(&["adversary.stage_cards=4,4,2"]));
        assert_eq!(row(&shorter, "0")[5], "10");
        let harsher = round_cap_block(&baseline(&["adversary.stage1_removed=2"]));
        assert_eq!(row(&base, "1")[5], row(&harsher, "1")[5], "difficulty 1 removes nothing");
        assert_eq!(row(&harsher, "2")[5], "10");
    }

    /// Every block that reads a config moves with it.
    #[test]
    fn a_config_change_changes_its_block() {
        let base = baseline(&[]);
        let blocks: [(&str, fn(&Rules) -> String); 8] = [
            ("adversary.build_bonus=2", difficulty_block),
            ("board.remove_energy=3", standard_actions_block),
            ("board.base_unity=3", sway_sources_block),
            ("alignment.unity_ease=1,3,5", purge_ease_block),
            ("cards.keep_tier2=5", new_players_block),
            ("adversary.corrupt_at=7", game_end_block),
            ("adversary.stage_cards=3,5,2", round_cap_block),
            ("ai.slots=5,3", |r| boards_block(r, content())),
        ];
        for (o, block) in blocks {
            assert_ne!(block(&base), block(&baseline(&[o])), "{o}");
        }
    }

    /// Every block that reads content moves with it.
    #[test]
    fn a_content_change_changes_its_block() {
        let base = content();
        let mut c = base.clone();
        c.ais[0].presence[1].income[0].1 += 1;
        c.ais[1].presence2[2].cost += 1;
        c.ais[1].powers[0].tags[0].1 += 1;
        c.ais[0].growth[0].name = "Something else".to_string();
        c.map.countries[0].megaproject.cubes += 1;
        assert_ne!(presence_track_block(base), presence_track_block(&c));
        assert_ne!(presence_track2_block(base), presence_track2_block(&c));
        assert_ne!(powers_block(base), powers_block(&c));
        assert_ne!(board_details_block(base), board_details_block(&c));
        assert_ne!(countries_block(base), countries_block(&c));
    }

    #[test]
    fn check_flags_a_bare_name_that_is_no_tunable() {
        let r = baseline(&[]);
        assert!(unknown_params("`hold_base`, `presence_buy`", &r).is_empty());
        assert_eq!(unknown_params("`hold_bass`", &r), ["hold_bass"]);
    }

    #[test]
    fn a_track_space_prints_income_upgrade_and_cost() {
        let u = Uncover {
            income: vec![(Resource::Credits, 2), (Resource::Chips, 1)],
            keep2: true,
            cost: 3,
        };
        assert_eq!(space(&u), "+2 credits, +1 chip, **keep upgrade** (cost 3)");
    }
}

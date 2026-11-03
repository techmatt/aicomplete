//! The harms ledger: what each source does to the players, and its checks.

use std::collections::BTreeMap;

use aic_core::content::{content, Op, Resource};

use super::table;

/// The harms of the ledger, in its row order.
pub const HARMS: [&str; 14] = [
    "node destroyed",
    "credits or income lost",
    "cards discarded",
    "insight lost",
    "ambassadors lost",
    "tokens stripped from cards",
    "facilities destroyed or seized",
    "production blacked out",
    "adversary pieces placed",
    "projects advanced (compute)",
    "extra threats",
    "Discord sway",
    "Unity pulled back",
    "actions or claims blocked",
];

/// The sources of the ledger, in its column order. The first six read
/// content; `RULE_HARMS` says what the rest do.
pub const SOURCES: [&str; 12] = [
    "tier-1 threats",
    "tier-2 threats",
    "Discord cards",
    "adversary twists",
    "adversary projects",
    "megaprojects",
    "arrivals",
    "build and spread",
    "corruption",
    "uncountered threats",
    "war",
    "march",
];

/// What the board rules (the sources content doesn't print) do: source, harm,
/// and the rule, as the ledger's note. Update this when an adversary rule changes.
pub const RULE_HARMS: [(&str, &str, &str); 20] = [
    ("arrivals", "adversary pieces placed", "every matching site gains an ambassador"),
    ("arrivals", "ambassadors lost", "an arrival cancels a player ambassador there"),
    ("arrivals", "Discord sway", "an ambassador on a base sways Discord when it produces"),
    ("arrivals", "production blacked out", "an ambassador on a city or station diverts it"),
    ("build and spread", "adversary pieces placed", "cities build; surplus spreads; two meeting make a botnet"),
    ("build and spread", "projects advanced (compute)", "each botnet makes compute"),
    ("build and spread", "actions or claims blocked", "a botnet blocks site claims next to it"),
    ("build and spread", "insight lost", "an ambassador on a lone node drains insight every round until removed"),
    ("build and spread", "ambassadors lost", "a build or spread cancels a player ambassador there"),
    ("corruption", "node destroyed", "a node on or next to a city that corrupts is destroyed"),
    ("corruption", "ambassadors lost", "player ambassadors on it are lost"),
    ("corruption", "production blacked out", "its energy goes to the adversary"),
    ("corruption", "Discord sway", "its energy sways Discord each round"),
    ("uncountered threats", "adversary pieces placed", "a threat with no source that goes uncountered makes its target one: a city, station or factory goes over, or a troop lands at a base; one a country a round"),
    ("uncountered threats", "node destroyed", "a city it corrupts takes the nodes on and next to it"),
    ("uncountered threats", "ambassadors lost", "player ambassadors on a city it corrupts are lost"),
    ("uncountered threats", "facilities destroyed or seized", "with no neutral one left, its target is a player's factory or station"),
    ("war", "node destroyed", "a destroyed factory takes the node on its hex"),
    ("war", "facilities destroyed or seized", "surplus troops destroy factories"),
    ("march", "facilities destroyed or seized", "a troop strikes player-owned facilities; it can't enter or strike a hex with the Protector's node"),
];

/// Columns of [`SOURCES`] that read content, for [`harm_marks`].
const DISCORD_CARDS: usize = 2;
const ADVERSARY_TWISTS: usize = 3;
const ADVERSARY_PROJECTS: usize = 4;
const MEGAPROJECTS: usize = 5;
/// The first columns: the threats, by tier.
const THREAT_TIERS: usize = 2;

/// Rows of [`HARMS`], for [`op_harms`].
const NODE_DESTROYED: usize = 0;
const CREDITS_LOST: usize = 1;
const CARDS_DISCARDED: usize = 2;
const INSIGHT_LOST: usize = 3;
const AMBASSADORS_LOST: usize = 4;
const TOKENS_STRIPPED: usize = 5;
const FACILITIES_DESTROYED: usize = 6;
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
        Op::DestroyNode | Op::EachDestroyNode { .. } | Op::LoseNodeOr { .. } => vec![NODE_DESTROYED],
        Op::Sacrifice => vec![NODE_DESTROYED, AMBASSADORS_LOST],
        Op::PlayersLose { resource: Resource::Credits, .. } => vec![CREDITS_LOST],
        Op::Requisition { .. } | Op::DiscardMostExpensive => vec![CARDS_DISCARDED],
        Op::LoseInsight { .. } => vec![INSIGHT_LOST],
        Op::LoseAmbassador => vec![AMBASSADORS_LOST],
        Op::LoseTokens { .. } => vec![TOKENS_STRIPPED],
        Op::FlipStation { .. } => vec![FACILITIES_DESTROYED],
        Op::Flag { name } => match name.name() {
            "embargo" => vec![BLOCKED],
            _ => vec![BLACKED_OUT],
        },
        Op::Ambassadors { .. }
        | Op::Infiltrate { .. }
        | Op::BotnetAmbassadors { .. }
        | Op::CleanCityAmbassadors { .. }
        | Op::AdversaryBuild
        | Op::AdversarySpread
        | Op::CityBuild { .. }
        | Op::CleanFactories { .. }
        | Op::StationBuild { .. } => vec![PIECES_PLACED],
        // 044: a corruption takes the node and ambassadors around the city too.
        Op::CorruptCity => vec![PIECES_PLACED, NODE_DESTROYED, AMBASSADORS_LOST],
        Op::Troops { destroy, .. } => {
            if *destroy {
                vec![PIECES_PLACED, NODE_DESTROYED]
            } else {
                vec![PIECES_PLACED]
            }
        }
        Op::Compute { .. } | Op::ComputeEvery { .. } => vec![PROJECTS_ADVANCED],
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
        add(DISCORD_CARDS, &all);
    }
    for a in &c.adversary {
        add(ADVERSARY_TWISTS, &a.twists);
    }
    for p in &c.projects {
        add(ADVERSARY_PROJECTS, &p.effects);
    }
    for p in &c.map.countries {
        add(MEGAPROJECTS, &[&p.megaproject.adversary[..], &p.megaproject.after[..]].concat());
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
    for (s, source) in SOURCES.iter().enumerate().take(THREAT_TIERS) {
        let n = (0..HARMS.len()).filter(|&h| marks.contains_key(&(h, s))).count();
        if n < 5 {
            bad.push(format!("{source}: {n} kinds of harm, needs 5"));
        }
    }
    let all: usize = (0..SOURCES.len())
        .filter_map(|s| marks.get(&(NODE_DESTROYED, s)))
        .map(|m| m.weight())
        .sum();
    for (s, source) in SOURCES.iter().enumerate() {
        if let Some(m) = marks.get(&(NODE_DESTROYED, s)) {
            let n = m.weight();
            if 2 * n > all + 1 {
                bad.push(format!("{source}: {n} of {all} printed node destructions"));
            }
        }
    }
    bad
}

/// `docs/harms.md`: the ledger, from content and `RULE_HARMS`.
pub(super) fn harms_md() -> String {
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
         `crates/aic-sim/src/docs/harms.rs`: edit those, not this file. Keep it current: a change to \
         threats, Discord cards or adversary rules that adds or moves a harm updates \
         `RULE_HARMS` or `op_harms`.\n\nOne row per harm, one column per kind of source. A number \
         is how many printed cards do it (a threat, Discord card, twist or project counts once); \
         `yes` is a rule of the board. `Discord cards` covers both tiers; `march` is the adversary \
         troops' strikes, which take facilities only at `board.troop_node` 4. The `yes` cells \
         are true at the baseline's numbers (`board.destruction_discord` and \
         `board.ambassador_discord` are 0, so neither war nor an ambassador on a clean city sways).\n\n{}\n\
         **Checks** (`aic-sim docs --check`, and a fast test): every harm in at least 2 kinds of \
         source; each threat tier at least 5 kinds of harm; no kind of source more than about half \
         of the printed node destruction. Measured shares are in the 034 screen (`experiments/results/screen-034-clocks/clocks.md`).\n\n\
         ## The board's rules\n\n{notes}",
        table(&head, rows)
    )
}

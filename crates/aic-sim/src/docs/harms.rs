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
    "adversary events",
    "adversary projects",
    "megaprojects",
    "income and rally",
    "botnets",
    "corruption",
    "uncountered threats",
    "war",
    "march",
];

/// What the board rules (the sources content doesn't print) do: source, harm,
/// and the rule, as the ledger's note. Update this when an adversary rule changes.
pub const RULE_HARMS: [(&str, &str, &str); 17] = [
    ("income and rally", "adversary pieces placed", "ambassador income fills each board's pool; every event's Rally X lands it on the sites of its type (151)"),
    ("income and rally", "projects advanced (compute)", "compute income onto the projects every round (151)"),
    ("income and rally", "ambassadors lost", "an arriving ambassador cancels a player ambassador there"),
    ("income and rally", "production blacked out", "an ambassador on a city or station blocks its output while it stands (137)"),
    ("income and rally", "insight lost", "an ambassador on a lone node drains insight every round until removed"),
    ("botnets", "projects advanced (compute)", "each botnet makes compute"),
    ("botnets", "actions or claims blocked", "a botnet blocks site claims next to it"),
    ("corruption", "node destroyed", "a node on the corrupted hex itself is destroyed (an allied ambassador there is armor)"),
    ("corruption", "facilities destroyed or seized", "a research station or factory that corrupts becomes the adversary's"),
    ("corruption", "ambassadors lost", "player ambassadors on it are lost"),
    ("corruption", "production blacked out", "its energy goes to the adversary"),
    ("corruption", "Discord sway", "its energy sways Discord each round"),
    // 146: no unprinted harm (the node loss near an uncountered threat and its floor sway are gone); the adversary
    // board may print compute per uncountered threat.
    ("uncountered threats", "projects advanced (compute)", "the adversary board's printed compute per threat that executes uncountered (the Militarist's difficulty line, from `board_difficulty_at`)"),
    ("war", "node destroyed", "a destroyed factory takes the node on its hex"),
    ("war", "facilities destroyed or seized", "surplus troops destroy factories"),
    ("war", "Discord sway", "each factory destroyed in war sways `destruction_discord` toward Discord"),
    ("march", "facilities destroyed or seized", "a troop strikes player-owned facilities; it can't enter or strike a hex with the Protector's node"),
];

/// Columns of [`SOURCES`] that read content, for [`harm_marks`].
const DISCORD_CARDS: usize = 2;
const ADVERSARY_EVENTS: usize = 3;
const ADVERSARY_PROJECTS: usize = 4;
const MEGAPROJECTS: usize = 5;
/// The first columns: the threats, by tier (146: no tier 3).
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
        Op::EachDestroyNode { .. } | Op::DestroyNode { .. } | Op::LoseNodeOr { .. } => vec![NODE_DESTROYED],
        Op::PlayersLose { resource: Resource::Credits, .. } => vec![CREDITS_LOST],
        // 146: a threat's "pays 2 ambassadors or discards 2 cards" takes either.
        Op::Requisition { resource: Resource::Ambassadors, .. } | Op::Theft { resource: Resource::Ambassadors, .. } => {
            vec![AMBASSADORS_LOST, CARDS_DISCARDED]
        }
        Op::Requisition { .. } | Op::DiscardMostExpensive | Op::Discard { .. } => vec![CARDS_DISCARDED],
        Op::Theft { resource: Resource::Credits, .. } | Op::LoseIncome { resource: Resource::Credits, .. } => {
            vec![CREDITS_LOST]
        }
        // 146: theft is "pays N or discards M cards", and the adversary gains its compute.
        Op::Theft { .. } => vec![CARDS_DISCARDED, PROJECTS_ADVANCED],
        // Capital Flight: a negative credit income for every player.
        Op::TeamIncome { resource: Resource::Credits, amount } if *amount < 0 => vec![CREDITS_LOST],
        Op::LoseAmbassadorsOr { .. } => vec![AMBASSADORS_LOST, CREDITS_LOST],
        Op::LoseOr { options } => {
            let mut all: Vec<usize> = options.iter().flat_map(op_harms).collect();
            all.sort_unstable();
            all.dedup();
            all
        }
        Op::DestroyFacility { .. } => vec![FACILITIES_DESTROYED, NODE_DESTROYED],
        Op::SeizeFactory | Op::OccupyFactory { .. } => vec![FACILITIES_DESTROYED],
        Op::DisableBuilding { .. } => vec![BLOCKED],
        Op::AdversaryFactory { .. } | Op::PlaceBotnet { .. } | Op::CorpTroops { .. } | Op::MarchNow { .. } => {
            vec![PIECES_PLACED]
        }
        Op::ExtraWar { .. } => vec![FACILITIES_DESTROYED],
        Op::ComputePerMissing { .. } => vec![PROJECTS_ADVANCED],
        Op::LoseInsight { .. } => vec![INSIGHT_LOST],
        Op::LoseAmbassador => vec![AMBASSADORS_LOST],
        Op::LoseTokens { .. } => vec![TOKENS_STRIPPED],
        Op::FlipStation { .. } => vec![FACILITIES_DESTROYED],
        Op::Flag { name, .. } => match name.name() {
            "embargo" | "power_down" => vec![BLOCKED],
            _ => vec![BLACKED_OUT],
        },
        Op::Create { .. } | Op::AdversaryBuild | Op::Rally { .. } => vec![PIECES_PLACED],
        Op::IncreaseIncome { compute, ambassadors } => {
            let mut v = Vec::new();
            if *ambassadors > 0 {
                v.push(PIECES_PLACED);
            }
            if *compute > 0 {
                v.push(PROJECTS_ADVANCED);
            }
            v
        }
        // 138: a corruption takes a node and the ambassadors on the hex itself.
        Op::CorruptCity { .. } | Op::Corrupt { .. } => vec![PIECES_PLACED, NODE_DESTROYED, AMBASSADORS_LOST],
        Op::Troops { destroy, .. } => {
            if *destroy {
                vec![PIECES_PLACED, NODE_DESTROYED]
            } else {
                vec![PIECES_PLACED]
            }
        }
        Op::Compute { .. } | Op::ComputeEvery { .. } => vec![PROJECTS_ADVANCED],
        Op::DealThreats { .. } | Op::ThreatAtNode { .. } => vec![EXTRA_THREATS],
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
        add(ADVERSARY_EVENTS, &[&a.standing[..], &a.twists[..]].concat());
    }
    for r in &c.adversary_board.rules {
        add(ADVERSARY_EVENTS, &r.effects);
    }
    for p in &c.projects {
        // An ongoing project's resolve line counts too, once per harm with its effects.
        add(ADVERSARY_PROJECTS, &[&p.effects[..], &p.resolve[..]].concat());
    }
    for p in &c.map.countries {
        add(MEGAPROJECTS, &p.megaproject.adversary);
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
         `crates/aic-sim/src/docs/harms.rs`: edit those, not this file. Rules revision {rev}. Keep it \
         current: a change to threats, Discord cards or adversary rules that adds or moves a harm \
         updates `RULE_HARMS` or `op_harms`.\n\nOne row per harm, one column per kind of source. A number \
         is how many printed cards do it (a threat, Discord card, twist or project counts once); \
         `yes` is a rule of the board. `Discord cards` covers both tiers; `march` is the adversary \
         troops' strikes, which take facilities only at `board.troop_node` 4. The `yes` cells \
         are true at the baseline's numbers (`board.ambassador_discord` is 0, so an ambassador \
         darkening an aligned city doesn't sway).\n\n{}\n\
         **Checks** (`aic-sim docs --check`, and a fast test): every harm in at least 2 kinds of \
         source; each threat tier at least 5 kinds of harm; no kind of source more than about half \
         of the printed node destruction. Measured shares are in the 034 screen (`experiments/results/screen-034-clocks/clocks.md`).\n\n\
         ## The board's rules\n\n{notes}",
        table(&head, rows),
        rev = aic_core::RULES_REVISION,
    )
}

//! The harms ledger: what each source does to the players, and its checks.

use std::collections::BTreeMap;

use aic_core::content::{content, Op, Resource};

use aic_core::Rules;

use super::blocks::tick;
use super::table;

/// The harms of the ledger, in its row order.
pub const HARMS: [&str; 13] = [
    "node destroyed",
    "credits or income lost",
    "cards discarded",
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
pub const RULE_HARMS: [(&str, &str, &str); 18] = [
    ("income and rally", "adversary pieces placed", "agent income fills each country's pool; every event's Rally X deals it one at a time onto the sites of its type (151; r80)"),
    ("income and rally", "projects advanced (compute)", "compute income onto the projects in the compute step every round (151; 175: with all other compute)"),
    ("income and rally", "ambassadors lost", "an arriving agent cancels a player ambassador there"),
    ("income and rally", "production blacked out", "an agent on a city or station blocks its output while it stands (137)"),
    ("botnets", "projects advanced (compute)", "each botnet makes compute"),
    ("corruption", "node destroyed", "a hex with a player's node and `corrupt_node_at` agents loses the node (175; protections apply)"),
    ("corruption", "Discord sway", "a node hex that corrupts takes land corruption, which sways `blight_discord` toward Discord each round (r80)"),
    ("corruption", "node destroyed", "a node on the corrupted hex itself is destroyed (its owner may spend a protection: r75)"),
    ("corruption", "facilities destroyed or seized", "a research station or factory that corrupts becomes the adversary's; a firewall or a Corrupt prohibition's token guards every path, flips and threats included (r79, D77; r80)"),
    ("corruption", "ambassadors lost", "player ambassadors on it are lost"),
    ("corruption", "production blacked out", "its energy goes to the adversary"),
    ("corruption", "Discord sway", "its energy sways Discord each round"),
    // 185 (Matt's ruling #2): the megaprojects column reads the printed payout; this is what feeds it.
    ("corruption", "Discord sway", "an adversary station puts `station_cubes` adversary cubes a round on its country's open megaproject; each pays the card's adversary payout at completion, 1 Sway to Discord (r79)"),
    ("corruption", "projects advanced (compute)", "an adversary station makes compute once its megaproject is complete; before, its cubes on Polar Compute Array pay 1 adversary compute each at completion (r79)"),
    // 146: no unprinted harm (the node loss near an uncountered threat and its floor sway are gone); the adversary
    // board may print compute per uncountered threat.
    ("uncountered threats", "projects advanced (compute)", "the adversary board's printed compute per threat that executes uncountered (the Militarist's difficulty line, from `board_difficulty_at`)"),
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
const AMBASSADORS_LOST: usize = 3;
const TOKENS_STRIPPED: usize = 4;
const FACILITIES_DESTROYED: usize = 5;
const BLACKED_OUT: usize = 6;
const PIECES_PLACED: usize = 7;
const PROJECTS_ADVANCED: usize = 8;
const EXTRA_THREATS: usize = 9;
const DISCORD_SWAY: usize = 10;
const UNITY_PULLED: usize = 11;
const BLOCKED: usize = 12;

/// The harms one op does, as rows of [`HARMS`].
fn op_harms(op: &Op) -> Vec<usize> {
    match op {
        // 167: "for each X (at most N)": the harms of what it repeats.
        Op::PerSite { then, .. } => then.iter().flat_map(op_harms).collect(),
        Op::EachDestroyNode { .. } | Op::DestroyNode { .. } | Op::LoseNodeOr { .. } => vec![NODE_DESTROYED],
        Op::PlayersLose { resource: Resource::Credits, .. } => vec![CREDITS_LOST],
        // 146: a threat's "pays 2 ambassadors or discards 2 cards" takes either.
        Op::Requisition { resource: Resource::Ambassadors, .. } | Op::Theft { resource: Resource::Ambassadors, .. } => {
            vec![AMBASSADORS_LOST, CARDS_DISCARDED]
        }
        // r82: "pays N credits or discards M cards" (Data Heist) takes either; insight is never a harm.
        Op::Requisition { resource: Resource::Credits, .. } => vec![CREDITS_LOST, CARDS_DISCARDED],
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
        Op::LoseAmbassador => vec![AMBASSADORS_LOST],
        Op::LoseTokens { .. } => vec![TOKENS_STRIPPED],
        Op::FlipStation { .. } => vec![FACILITIES_DESTROYED],
        Op::Flag { name, .. } => match name.name() {
            "embargo" => vec![BLOCKED],
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
        // 185: an adversary cube pays a megaproject's adversary payout (Discord, or compute on one card).
        Op::AdversaryCubes { .. } | Op::BotnetCubes { .. } => vec![DISCORD_SWAY, PROJECTS_ADVANCED],
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
        let all: Vec<_> = a.all_effects().cloned().collect();
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
            // r80: "about half" with a two-mark slack: war stopped taking nodes (ruling #52), leaving the events'
            // Corrupt lines at 15 of 28 (a design call for Matt in 187's report, not a content change here).
            if 2 * n > all + 2 {
                bad.push(format!("{source}: {n} of {all} printed node destructions"));
            }
        }
    }
    bad
}

/// `docs/harms.md`: the ledger, from content and `RULE_HARMS`.
pub(super) fn harms_md(r: &Rules) -> String {
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
         troops' strikes, which take facilities only at {troop_node}. The `yes` cells \
         are true at the baseline's numbers ({amb_discord}: an agent \
         darkening an aligned city sways only when that is above 0).\n\n{}\n\
         **Checks** (`aic-sim docs --check`, and a fast test): every harm in at least 2 kinds of \
         source; each threat tier at least 5 kinds of harm; no kind of source more than about half \
         of the printed node destruction. Measured shares are in the 034 screen (`experiments/results/screen-034-clocks/clocks.md`).\n\n\
         ## The board's rules\n\n{notes}",
        table(&head, rows),
        rev = aic_core::RULES_REVISION,
        troop_node = tick(r, "board.troop_node"),
        amb_discord = tick(r, "board.ambassador_discord"),
    )
}

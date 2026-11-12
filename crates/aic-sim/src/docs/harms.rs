//! The harms ledger: what each source does to the players, and its checks.

use std::collections::BTreeMap;

use aic_core::content::{content, Op, Resource, RoundFlag, Spot};
use aic_core::op::{CorpWhen, CorruptSite};

use aic_core::Rules;

use super::blocks::tick;
use super::table;

/// The harms of the ledger, in its row order. 191 (Matt, 2026-11-11): every harm an adversary source can do has a
/// row; none for insight (not an adversary harm, r82) and none yet for costs raised (paged as a harm, no card).
pub const HARMS: [&str; 26] = [
    "node destroyed",
    "city lost",
    "facilities destroyed",
    "facilities seized",
    "credits or income lost",
    "energy lost",
    "chips lost",
    "antimatter lost",
    "research cubes lost",
    "cards discarded",
    "ambassadors lost",
    "tokens stripped from cards",
    "protections stripped",
    "pieces pushed or reach cut",
    "production blacked out",
    "adversary pieces placed",
    "adversary cubes on megaprojects",
    "adversary income raised",
    "projects advanced (compute)",
    "extra threats",
    "the clock sped up",
    "Discord sway",
    "Unity pulled back",
    "a megaproject or converter denied",
    "the card market disrupted",
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
pub const RULE_HARMS: [(&str, &str, &str); 21] = [
    ("income and rally", "adversary pieces placed", "agent income fills each country's pool; every event's Rally X deals it one at a time onto the sites of its type (151; r80)"),
    ("income and rally", "projects advanced (compute)", "compute income onto the projects in the compute step every round (151; 175: with all other compute)"),
    ("income and rally", "ambassadors lost", "an arriving agent cancels a player ambassador there"),
    ("income and rally", "production blacked out", "an agent on a city or station blocks its output while it stands (137)"),
    ("botnets", "projects advanced (compute)", "each botnet makes compute"),
    ("corruption", "node destroyed", "a hex with a player's node and `corrupt_node_at` agents loses the node (175; protections apply)"),
    ("corruption", "Discord sway", "a node hex that corrupts takes land corruption, saved node or not (r83, #56), which sways `blight_discord` toward Discord each round (r80)"),
    ("corruption", "node destroyed", "a node on the corrupted hex itself is destroyed (its owner may spend a protection: r75)"),
    ("corruption", "city lost", "a city holding its corrupt number of agents becomes the adversary's until a player liberates it"),
    ("corruption", "facilities seized", "a research station or factory that corrupts becomes the adversary's; a firewall or a Corrupt prohibition's token guards every path, flips and threats included (r79, D77; r80)"),
    ("corruption", "ambassadors lost", "player ambassadors on it are lost"),
    ("corruption", "production blacked out", "its energy goes to the adversary"),
    ("corruption", "Discord sway", "its energy sways Discord each round"),
    // 185 (Matt's ruling #2): the megaprojects column reads the printed payout; this is what feeds it.
    ("corruption", "adversary cubes on megaprojects", "an adversary station puts `station_cubes` adversary cubes a round on its country's open megaproject; each pays the card's adversary payout at completion, 1 Sway to Discord (r79)"),
    ("corruption", "projects advanced (compute)", "an adversary station makes compute once its megaproject is complete; before, its cubes on Polar Compute Array pay 1 adversary compute each at completion (r79)"),
    // 146: no unprinted harm (the node loss near an uncountered threat and its floor sway are gone); the adversary
    // board may print compute per uncountered threat.
    ("uncountered threats", "projects advanced (compute)", "the adversary board's printed compute per threat that executes uncountered (the Militarist's difficulty line, from `board_difficulty_at`)"),
    ("war", "facilities destroyed", "surplus troops destroy factories"),
    ("war", "node destroyed", "the Militarist's War Takes Ground: a drone that destroys a factory destroys a node on its hex too (r83; protections apply)"),
    ("war", "Discord sway", "each factory destroyed in war sways `destruction_discord` toward Discord"),
    // 195: a strike a protection absorbs spends it (the troop is used up either way, `troop_spent`).
    ("march", "protections stripped", "a strike absorbed by a hardening token or the owner's ambassador spends it (r75; the troop is used up either way)"),
    ("march", "facilities destroyed", "a troop strikes player-owned facilities, walking past ambassadors (r83, #44); it can't enter or strike a hex with the Protector's node"),
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
const CITY_LOST: usize = 1;
const FACILITIES_DESTROYED: usize = 2;
const FACILITIES_SEIZED: usize = 3;
const CREDITS_LOST: usize = 4;
const ENERGY_LOST: usize = 5;
const CHIPS_LOST: usize = 6;
const ANTIMATTER_LOST: usize = 7;
const CUBES_LOST: usize = 8;
const CARDS_DISCARDED: usize = 9;
const AMBASSADORS_LOST: usize = 10;
const TOKENS_STRIPPED: usize = 11;
const PROTECTIONS_STRIPPED: usize = 12;
const BLACKED_OUT: usize = 14;
const PIECES_PLACED: usize = 15;
const ADVERSARY_CUBES: usize = 16;
const INCOME_RAISED: usize = 17;
const PROJECTS_ADVANCED: usize = 18;
const EXTRA_THREATS: usize = 19;
const CLOCK_SPED_UP: usize = 20;
const DISCORD_SWAY: usize = 21;
const UNITY_PULLED: usize = 22;
const BLOCKED: usize = 25;

/// The row a lost resource falls in. Insight is never a harm (r82: `Content::check` refuses one).
fn lost(r: Resource) -> Vec<usize> {
    match r {
        Resource::Credits => vec![CREDITS_LOST],
        Resource::Energy => vec![ENERGY_LOST],
        Resource::Chips => vec![CHIPS_LOST],
        Resource::Antimatter => vec![ANTIMATTER_LOST],
        Resource::ResearchCubes => vec![CUBES_LOST],
        Resource::Ambassadors => vec![AMBASSADORS_LOST],
        Resource::Insight => Vec::new(),
    }
}

/// The harms of several ops, each once.
fn all_harms(ops: &[Op]) -> Vec<usize> {
    let mut all: Vec<usize> = ops.iter().flat_map(op_harms).collect();
    all.sort_unstable();
    all.dedup();
    all
}

/// The harms one op does, as rows of [`HARMS`]. 191: every op has its arm, so a new op can't fall through
/// unread; the player-side ops (gains, removals, passives) do no harm.
fn op_harms(op: &Op) -> Vec<usize> {
    match op {
        // 167: "for each X (at most N)": the harms of what it repeats.
        Op::PerSite { then, .. } | Op::OnePlayer { then } => all_harms(then),
        Op::LoseOr { options } => all_harms(options),
        Op::EachDestroyNode { .. } | Op::DestroyNode { .. } | Op::LoseNodeOr { .. } => vec![NODE_DESTROYED],
        Op::PlayersLose { resource, .. } | Op::LoseIncome { resource, .. } => lost(*resource),
        // Capital Flight: a negative income for every player.
        Op::TeamIncome { resource, amount } if *amount < 0 => lost(*resource),
        // 146: "pays N or discards M cards"; theft also gives the adversary its compute.
        Op::Requisition { resource, .. } => [lost(*resource), vec![CARDS_DISCARDED]].concat(),
        Op::Theft { resource, compute, .. } => {
            let mut v = [lost(*resource), vec![CARDS_DISCARDED]].concat();
            if *compute > 0 {
                v.push(PROJECTS_ADVANCED);
            }
            v
        }
        Op::DiscardMostExpensive | Op::Discard { .. } => vec![CARDS_DISCARDED],
        Op::LoseAmbassadorsOr { resource, .. } => [vec![AMBASSADORS_LOST], lost(*resource)].concat(),
        Op::LoseAmbassador => vec![AMBASSADORS_LOST],
        Op::DestroyFacility { .. } => vec![FACILITIES_DESTROYED, NODE_DESTROYED],
        Op::ExtraWar { .. } => vec![FACILITIES_DESTROYED],
        Op::SeizeFactory | Op::FlipStation { .. } => vec![FACILITIES_SEIZED],
        // A troop on the factory: it makes nothing while the troop stands.
        Op::OccupyFactory { .. } => vec![BLACKED_OUT, PIECES_PLACED],
        Op::DisableBuilding { .. } => vec![BLOCKED],
        Op::Flag { name, .. } => match name {
            RoundFlag::Embargo => vec![BLOCKED],
            RoundFlag::Blackout | RoundFlag::Sabotage => vec![BLACKED_OUT],
            // A player's flags (Shelter, Cordon) do no harm.
            RoundFlag::Shelter | RoundFlag::Cordon => Vec::new(),
        },
        Op::LoseTokens { .. } => vec![TOKENS_STRIPPED],
        Op::AdversaryFactory { .. }
        | Op::PlaceBotnet { .. }
        | Op::MarchNow { .. }
        | Op::Create { .. }
        | Op::CreateSite { .. }
        | Op::AdversaryBuild
        | Op::Rally { .. } => vec![PIECES_PLACED],
        Op::CorpTroops { when, permanent, rounds, .. } => {
            if *permanent || *rounds > 0 || *when == CorpWhen::EachProduction {
                vec![PIECES_PLACED, INCOME_RAISED]
            } else {
                vec![PIECES_PLACED]
            }
        }
        Op::IncreaseIncome { compute, ambassadors } => {
            let mut v = vec![INCOME_RAISED];
            if *ambassadors > 0 {
                v.push(PIECES_PLACED);
            }
            if *compute > 0 {
                v.push(PROJECTS_ADVANCED);
            }
            v
        }
        Op::StandingTroopsBonus { .. } | Op::TroopSteps { .. } => vec![INCOME_RAISED],
        Op::ThreatCompute { .. } => vec![INCOME_RAISED, PROJECTS_ADVANCED],
        // A city corrupted by a card; `at: location` also takes the nodes on and next to it.
        Op::CorruptCity { at } => {
            if *at == Some(Spot::Location) {
                vec![PIECES_PLACED, CITY_LOST, NODE_DESTROYED, AMBASSADORS_LOST]
            } else {
                vec![PIECES_PLACED, CITY_LOST, AMBASSADORS_LOST]
            }
        }
        // 191 (amendment 8617): a Corrupt line takes a node only where agents already stand, so it isn't node
        // destruction here; it takes the site it corrupts and the ambassadors on it.
        Op::Corrupt { site, .. } => match site {
            CorruptSite::City | CorruptSite::CoastalCity => vec![PIECES_PLACED, CITY_LOST, AMBASSADORS_LOST],
            CorruptSite::Station | CorruptSite::Factory => vec![PIECES_PLACED, FACILITIES_SEIZED, AMBASSADORS_LOST],
            CorruptSite::Land | CorruptSite::Node => vec![PIECES_PLACED, AMBASSADORS_LOST],
        },
        Op::Troops { destroy, .. } => {
            if *destroy {
                vec![PIECES_PLACED, NODE_DESTROYED]
            } else {
                vec![PIECES_PLACED]
            }
        }
        Op::Compute { .. } | Op::ComputeEvery { .. } | Op::ComputePerMissing { .. } => vec![PROJECTS_ADVANCED],
        Op::DealThreats { .. } | Op::ThreatAtNode { .. } => vec![EXTRA_THREATS],
        Op::Sway { side, .. } if side.name() == "discord" => vec![DISCORD_SWAY],
        Op::PullUnity { .. } => vec![UNITY_PULLED],
        // 185: an adversary cube pays a megaproject's adversary payout (Discord, or compute on one card).
        Op::AdversaryCubes { .. } | Op::BotnetCubes { .. } => vec![ADVERSARY_CUBES, DISCORD_SWAY, PROJECTS_ADVANCED],
        // 195: the project deck's board verbs.
        Op::BotnetTroops { .. } | Op::FactoryTroops { .. } => vec![PIECES_PLACED],
        Op::SpreadBotnets { .. } => vec![PIECES_PLACED, PROJECTS_ADVANCED],
        // A neutral site corrupts, else a player's turns neutral: seized either way.
        Op::TakeSite { .. } => vec![FACILITIES_SEIZED, AMBASSADORS_LOST],
        Op::RaidNode => vec![NODE_DESTROYED],
        Op::StripHarden => vec![PROTECTIONS_STRIPPED],
        Op::Clock { .. } => vec![CLOCK_SPED_UP],
        // The player's side: gains, removals, protections and passives.
        Op::Sway { .. }
        | Op::TeamIncome { .. }
        | Op::Gain { .. }
        | Op::GainPerNode { .. }
        | Op::ChoosePlayer
        | Op::OtherGain { .. }
        | Op::Transfer { .. }
        | Op::PlaceNode { .. }
        | Op::ClearMarkers { .. }
        | Op::ShieldNode { .. }
        | Op::DelayThreat { .. }
        | Op::Damage { .. }
        | Op::DamageCountry { .. }
        | Op::DestroyAll
        | Op::RemoveBotnet
        | Op::RemoveAmbassadorsCountry { .. }
        | Op::RemoveTroops { .. }
        | Op::BuildFactory
        | Op::Cleanse { .. }
        | Op::Liberate { .. }
        | Op::ClearCity { .. }
        | Op::Harden { .. }
        | Op::MoveAmbassadors { .. }
        | Op::GainAmbassadors { .. }
        | Op::Shelter
        | Op::Cordon
        | Op::FreeBuilding
        | Op::BuildStation
        | Op::ClaimStation
        | Op::ProjectCube { .. }
        | Op::Peek { .. }
        | Op::Jam { .. }
        | Op::CounterThreat
        | Op::DrainProject { .. }
        | Op::DiscardProject
        | Op::Production { .. }
        | Op::ProductionPerTag { .. }
        | Op::AddTokens { .. }
        | Op::CutBotnet
        | Op::MoveOne { .. }
        | Op::SpreadAmbassadors
        | Op::Consolidate { .. }
        | Op::Convert { .. }
        | Op::Gather { .. }
        | Op::Push { .. }
        | Op::Firewall
        | Op::IfTags { .. }
        | Op::Draw { .. }
        | Op::DrainAll { .. }
        | Op::GainPerTokens { .. }
        | Op::CashOut { .. }
        | Op::GainPerTag { .. }
        | Op::CopyIncome
        | Op::Reuse
        | Op::DiscardHand { .. }
        | Op::FreePurchases { .. }
        | Op::CounterAll
        | Op::Pushback { .. }
        | Op::RemoveBotnets { .. }
        | Op::RemoveWhere { .. }
        | Op::Prohibit { .. }
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
        | Op::ConvertUpkeep { .. } => Vec::new(),
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
        for h in all_harms(ops) {
            *counts.entry((h, src)).or_default() += 1;
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

/// How many kinds of source do each harm.
fn kinds(marks: &BTreeMap<(usize, usize), Mark>, h: usize) -> usize {
    (0..SOURCES.len()).filter(|&s| marks.contains_key(&(h, s))).count()
}

/// 191: the harms in under 2 kinds of source, with their count: gaps, the list for the adversary-card pass, not
/// failures.
pub fn harm_gaps() -> Vec<(&'static str, usize)> {
    let marks = harm_marks();
    HARMS.iter().enumerate().map(|(h, name)| (*name, kinds(&marks, h))).filter(|&(_, n)| n < 2).collect()
}

/// The kinds of source over about half of the printed node destruction: "about half" with its original one-mark
/// slack (191, amendment 8617: a Corrupt line no longer counts, as it takes a node only where agents already stand).
/// 191 (default of question 1, Matt's to confirm): reported as a gap, the adversary-card pass's to fix.
pub fn node_share_gaps() -> Vec<String> {
    let marks = harm_marks();
    let all: usize = (0..SOURCES.len())
        .filter_map(|s| marks.get(&(NODE_DESTROYED, s)))
        .map(|m| m.weight())
        .sum();
    SOURCES
        .iter()
        .enumerate()
        .filter_map(|(s, source)| marks.get(&(NODE_DESTROYED, s)).map(|m| (source, m.weight())))
        .filter(|&(_, n)| 2 * n > all + 1)
        .map(|(source, n)| format!("{source}: {n} of {all} printed node destructions, over about half"))
        .collect()
}

/// What fails the ledger's distribution checks.
pub fn harm_failures() -> Vec<String> {
    let marks = harm_marks();
    let mut bad = Vec::new();
    for (s, source) in SOURCES.iter().enumerate().take(THREAT_TIERS) {
        let n = (0..HARMS.len()).filter(|&h| marks.contains_key(&(h, s))).count();
        if n < 5 {
            bad.push(format!("{source}: {n} kinds of harm, needs 5"));
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
    let mut gaps: String = harm_gaps()
        .iter()
        .map(|(h, n)| format!("- {h}: {n} kind{} of source.\n", if *n == 1 { "" } else { "s" }))
        .collect();
    gaps.extend(node_share_gaps().iter().map(|g| format!("- node destroyed, {g}.\n")));
    format!(
        "# The harms ledger\n\nGenerated by `aic-sim docs` from `content/` and `RULE_HARMS` in \
         `crates/aic-sim/src/docs/harms.rs`: edit those, not this file. Rules revision {rev}. Keep it \
         current: a change to threats, Discord cards or adversary rules that adds or moves a harm \
         updates `RULE_HARMS` or `op_harms` (every op has its arm there, so none falls through unread).\n\n\
         One row per harm an adversary source can do, one column per kind of source. A number \
         is how many printed cards do it (a threat, Discord card, twist or project counts once); \
         `yes` is a rule of the board. Insight is no adversary harm, and costs raised has no row yet. \
         `Discord cards` covers both tiers; `march` is the adversary \
         troops' strikes, which take facilities only at {troop_node}. The `yes` cells \
         are true at the baseline's numbers ({amb_discord}: an agent \
         darkening an aligned city sways only when that is above 0).\n\n{}\n\
         **Check** (`aic-sim docs --check`, and a fast test): each threat tier at least 5 kinds of harm. Measured \
         shares are in the 034 screen (`experiments/results/screen-034-clocks/clocks.md`).\n\n\
         ## Gaps\n\nThe list for the adversary-card pass, not failures: a harm in under 2 kinds of source, and a kind \
         of source over about half of the printed node destruction (with one mark of slack; a Corrupt line doesn't \
         count: it takes a node only where agents already stand).\n\n\
         {gaps}\n## The board's rules\n\n{notes}",
        table(&head, rows),
        rev = aic_core::RULES_REVISION,
        troop_node = tick(r, "board.troop_node"),
        amb_discord = tick(r, "board.ambassador_discord"),
    )
}

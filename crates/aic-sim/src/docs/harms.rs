//! The harms ledger: what each source does to the players, and its checks.

use std::collections::BTreeMap;

use aic_core::content::{content, Location, Op, Resource, RoundFlag, Spot};
use aic_core::op::CreateAt;
use aic_core::op::SiteType;

use aic_core::Rules;

use super::blocks::{int, list, tick};
use super::table;

/// The harms of the ledger, in its row order. 191 (Matt, 2026-11-11): every harm an adversary source can do has a
/// row; none for insight (not an adversary harm, r82) and none yet for costs raised (paged as a harm, no card).
pub const HARMS: &[&str] = &[
    "node destroyed",
    "city lost",
    "facilities destroyed",
    "facilities corrupted",
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
    "adversary income raised",
    "projects advanced (compute)",
    "extra threats",
    "the clock sped up",
    "Discord sway",
    "Unity pulled back",
    "a megaproject or converter denied",
    "the card market disrupted",
    "actions or claims blocked",
    // 227 (Matt, 2026-11-13): the land scar, a common harm on all four adversity sources.
    "Fallout placed",
    // 259 (Matt, 2026-11-16): cubes toward the adversary board's research rule.
    "adversary research advanced",
];

/// The sources of the ledger, in its column order. The first five read
/// content; `RULE_HARMS` says what the rest do.
pub const SOURCES: &[&str] = &[
    "tier-1 threats",
    "tier-2 threats",
    "Discord cards",
    "adversary events",
    "adversary projects",
    "income and Infiltrate",
    "botnets",
    "corruption",
    "uncountered threats",
    "troops",
    // 259: the adversary board's research rule firing.
    "adversary research",
];

/// What the board rules (the sources content doesn't print) do: source, harm,
/// and the rule, as the ledger's note. Update this when an adversary rule changes.
pub const RULE_HARMS: &[(&str, &str, &str)] = &[
    ("income and Infiltrate", "adversary pieces placed", "every event's Infiltrate X lands `infiltrate_agents` (1; 2 at level 4) on each site of its type, at most `infiltrate_facility_cap` (1) a site from a station, factory or botnet line, not yet the adversary's (275: it replaced Rally and the agent pools; 282: the cap)"),
    ("income and Infiltrate", "projects advanced (compute)", "compute income onto the projects in the compute step every round (151; 175: with all other compute)"),
    ("income and Infiltrate", "ambassadors lost", "an arriving agent cancels a player ambassador there"),
    ("income and Infiltrate", "production blacked out", "an agent on a city or station blocks its output while it stands (137)"),
    ("botnets", "projects advanced (compute)", "each botnet makes compute"),
    ("uncountered threats", "adversary pieces placed", "a threat that executes uncountered lands `threat_agents` agent on its anchor, its first location token (275)"),
    ("corruption", "node destroyed", "`corrupt_node_at` agents on a hex with a player's node destroy the node and leave no mark (175, r98; protections apply)"),
    ("corruption", "node destroyed", "a node on the corrupted hex itself is destroyed (its owner may spend a protection: r75)"),
    ("corruption", "city lost", "a city its corrupt number of agents lands on (an arrival, 275: only that hex is checked) becomes the adversary's until a player liberates it"),
    // 267: the ladder's level-1 rule, Sleeper Cells: a city falls sooner (the one corrupt number a level lowers).
    ("corruption", "city lost", "Sleeper Cells (level 1, from `corrupt_cut_at`): an agent landing on a city checks it at its printed number less `corrupt_cut`'s first entry"),
    ("corruption", "facilities corrupted", "a research station or factory that corrupts becomes the adversary's; a firewall or a Corrupt prohibition's token guards every path, flips and threats included (r79, D77; r80)"),
    ("corruption", "ambassadors lost", "player ambassadors on it are lost"),
    ("corruption", "production blacked out", "its energy goes to the adversary"),
    ("corruption", "Discord sway", "a corrupted city makes 1 Discord each round, its one output (293)"),
    // 293 (Matt, 2026-11-19): one output a corrupted site.
    ("corruption", "adversary research advanced", "a corrupted station puts `station_cubes` research cube a round on the adversary's research track, from round 1 (293: megaprojects take no adversary cube)"),
    ("corruption", "adversary pieces placed", "a corrupted factory makes 1 agent on itself and pushes it along its zone's arrow onto its city the next round (293: no troop); one whose city is the adversary's keeps its one agent"),
    ("adversary research", "the clock sped up", "the research track advances a slot for every `research_at` cubes a country, each slot's icon firing once; slot 7 is a loss (293)"),
    ("income and Infiltrate", "adversary pieces placed", "each site of last round's Infiltrate type that still holds an agent takes 1 more (293: it replaced Spill and Grow); the Militarist's Commandeered Plants does the same to factories every round"),
    // 146: no unprinted harm (the node loss near an uncountered threat and its floor sway are gone); the adversary
    // board may print compute per uncountered threat.
    ("uncountered threats", "projects advanced (compute)", "the adversary board's printed compute per threat that executes uncountered (the Militarist's level-2 rule, War Footing, from `board_difficulty_at`)"),
    // 195: a strike a protection absorbs spends it (the troop stays, r141).
    ("troops", "protections stripped", "a strike absorbed by a hardening token or the owner's ambassador spends it (r75; the troop stays and strikes again on the next icon)"),
    ("troops", "facilities destroyed", "on a Strike icon each troop destroys one player-owned facility or node on or next to it (r141); none strikes a hex with the Protector's node"),
    // 201 (r89, Matt 2026-11-12: troops are few and deadly): troops strike nodes from the start (`troop_node` 0).
    ("troops", "node destroyed", "on a Strike icon a troop destroys a player's node on or next to its hex (`troop_node` 0; protections apply)"),
    // r141 (Matt, 2026-11-19): the guarding rule.
    ("troops", "actions or claims blocked", "a corrupted site with a troop on or next to it can't be liberated, by the action or by a card (r141)"),
];

/// 304: an entry's index in [`HARMS`] or [`SOURCES`], by name, at compile time: a renamed or removed entry is a
/// compile error at the constant naming it, never a silent shift.
const fn at(list: &[&str], name: &str) -> usize {
    let mut i = 0;
    while i < list.len() {
        let (a, b) = (list[i].as_bytes(), name.as_bytes());
        if a.len() == b.len() {
            let mut j = 0;
            while j < a.len() && a[j] == b[j] {
                j += 1;
            }
            if j == a.len() {
                return i;
            }
        }
        i += 1;
    }
    panic!("no such harms-ledger entry")
}

/// Columns of [`SOURCES`] that read content, for [`harm_marks`].
const DISCORD_CARDS: usize = at(SOURCES, "Discord cards");
const ADVERSARY_EVENTS: usize = at(SOURCES, "adversary events");
const ADVERSARY_PROJECTS: usize = at(SOURCES, "adversary projects");
/// The first columns: the threats, by tier (146: no tier 3).
const THREAT_TIERS: usize = at(SOURCES, "tier-2 threats") + 1;
const _: () = assert!(at(SOURCES, "tier-1 threats") == 0, "the threat tiers are the first columns");

/// Rows of [`HARMS`], for [`op_harms`].
const NODE_DESTROYED: usize = at(HARMS, "node destroyed");
const CITY_LOST: usize = at(HARMS, "city lost");
const FACILITIES_DESTROYED: usize = at(HARMS, "facilities destroyed");
const FACILITIES_CORRUPTED: usize = at(HARMS, "facilities corrupted");
const CREDITS_LOST: usize = at(HARMS, "credits or income lost");
const ENERGY_LOST: usize = at(HARMS, "energy lost");
const CHIPS_LOST: usize = at(HARMS, "chips lost");
const ANTIMATTER_LOST: usize = at(HARMS, "antimatter lost");
const CUBES_LOST: usize = at(HARMS, "research cubes lost");
const CARDS_DISCARDED: usize = at(HARMS, "cards discarded");
const AMBASSADORS_LOST: usize = at(HARMS, "ambassadors lost");
const TOKENS_STRIPPED: usize = at(HARMS, "tokens stripped from cards");
const PROTECTIONS_STRIPPED: usize = at(HARMS, "protections stripped");
const PIECES_PUSHED: usize = at(HARMS, "pieces pushed or reach cut");
const BLACKED_OUT: usize = at(HARMS, "production blacked out");
const PIECES_PLACED: usize = at(HARMS, "adversary pieces placed");
const INCOME_RAISED: usize = at(HARMS, "adversary income raised");
const PROJECTS_ADVANCED: usize = at(HARMS, "projects advanced (compute)");
const EXTRA_THREATS: usize = at(HARMS, "extra threats");
const CLOCK_SPED_UP: usize = at(HARMS, "the clock sped up");
const DISCORD_SWAY: usize = at(HARMS, "Discord sway");
const UNITY_PULLED: usize = at(HARMS, "Unity pulled back");
const MEGAPROJECT_DENIED: usize = at(HARMS, "a megaproject or converter denied");
const BLOCKED: usize = at(HARMS, "actions or claims blocked");
const FALLOUT_PLACED: usize = at(HARMS, "Fallout placed");
const RESEARCH_ADVANCED: usize = at(HARMS, "adversary research advanced");

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
        // A corrupted factory or flipped station takes its hex: the node and ambassadors on it go too (197).
        Op::CorruptFactory | Op::FlipStation { .. } => vec![FACILITIES_CORRUPTED, NODE_DESTROYED, AMBASSADORS_LOST],
        // A troop on the factory: it makes nothing while the troop stands.
        Op::OccupyFactory { .. } => vec![BLACKED_OUT, PIECES_PLACED],
        Op::DisableBuilding { .. } => vec![BLOCKED],
        Op::Flag { name, .. } => match name {
            RoundFlag::Embargo => vec![BLOCKED],
            RoundFlag::Blackout | RoundFlag::Sabotage => vec![BLACKED_OUT],
            // A player's flags (Shelter, Cordon) do no harm.
            RoundFlag::Cordon => Vec::new(),
        },
        Op::LoseTokens { .. } => vec![TOKENS_STRIPPED],
        Op::AdversaryFactory { .. }
        | Op::PlaceBotnet { .. }
        | Op::Create { .. }
        | Op::CreateSite { .. }
        | Op::AdversaryBuild
        | Op::Infiltrate { .. } => vec![PIECES_PLACED],
        // 293: an Escalate's arrival corrupts the site it lands on at its number.
        Op::Escalate { site } => match site {
            SiteType::City | SiteType::CoastalCity => vec![PIECES_PLACED, CITY_LOST, AMBASSADORS_LOST],
            _ => vec![PIECES_PLACED, FACILITIES_CORRUPTED, AMBASSADORS_LOST],
        },
        // 255: each production, its one form.
        Op::IncreaseIncome { compute } => {
            let mut v = vec![INCOME_RAISED];
            if *compute > 0 {
                v.push(PROJECTS_ADVANCED);
            }
            v
        }
        Op::StandingTroopsBonus { .. } => vec![INCOME_RAISED],
        // r141: a Strike destroys a player's node or facility beside each troop; a Redeploy puts a guard back on a
        // corrupted city (no liberation there).
        Op::Strike => vec![FACILITIES_DESTROYED, NODE_DESTROYED],
        Op::Redeploy => vec![BLOCKED],
        Op::ThreatCompute { .. } => vec![INCOME_RAISED, PROJECTS_ADVANCED],
        // A city corrupted by a card; `at: location` also takes the nodes on and next to it.
        Op::CorruptCity { at } => {
            if *at == Some(Spot::Location) {
                vec![PIECES_PLACED, CITY_LOST, NODE_DESTROYED, AMBASSADORS_LOST]
            } else {
                vec![PIECES_PLACED, CITY_LOST, AMBASSADORS_LOST]
            }
        }
        Op::Fallout { .. } => vec![FALLOUT_PLACED],
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
        // 259: cubes toward the adversary board's research rule.
        Op::AdversaryResearch { .. } => vec![RESEARCH_ADVANCED],
        // 195: the project deck's board verbs.
        Op::BotnetTroops { .. } | Op::FactoryTroops { .. } => vec![PIECES_PLACED],
        Op::SpreadBotnets { .. } => vec![PIECES_PLACED, PROJECTS_ADVANCED],
        // A neutral site corrupts (a node on its hex goes with it, 197), else a player's turns neutral: a facility
        // lost either way.
        Op::TakeSite { .. } => vec![FACILITIES_CORRUPTED, NODE_DESTROYED, AMBASSADORS_LOST],
        // 231: Night Raid's agents on node hexes destroy a node by the 2-agent rule in the Corrupt beat.
        Op::AgentsOnNodes { .. } => vec![PIECES_PLACED, NODE_DESTROYED],
        Op::RecallAmbassadors => vec![PIECES_PUSHED],
        Op::StripCubes { .. } => vec![MEGAPROJECT_DENIED],
        Op::StripHarden => vec![PROTECTIONS_STRIPPED],
        Op::Clock { .. } => vec![CLOCK_SPED_UP],
        // The player's side: gains, removals, protections and passives.
        Op::Sway { .. }
        | Op::TeamIncome { .. }
        | Op::PullDiscord { .. }
        | Op::Gain { .. }
        | Op::GainPerNode { .. }
        | Op::ChoosePlayer
        | Op::OtherGain { .. }
        | Op::Transfer { .. }
        | Op::PlaceNode { .. }
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
        | Op::PlaceAmbassadors { .. }
        | Op::Target { .. }
        | Op::Shelter
        | Op::Cordon
        | Op::FreeBuilding
        | Op::BuildStation
        | Op::ClaimSite { .. }
        | Op::ProjectCube { .. }
        | Op::Peek { .. }
        | Op::Jam { .. }
        | Op::CounterThreat
        | Op::DrainProject { .. }
        | Op::RemoveResearch { .. }
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
        | Op::IfNodeNext { .. }
        | Op::AmbassadorToSpot
        | Op::LiftFallout { .. }
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

/// 286 (D130): a threat whose own creation, with the uncountered anchor agent (`threat_agents`), reaches its anchor
/// site's corrupt number at some level (an empty site): the capture is its effect, counted as the corruption's
/// harms. (name, tier, agents created, the site's lowest number, the site.)
pub struct Capture {
    pub name: String,
    pub tier: u8,
    pub made: i64,
    pub number: i64,
    pub site: &'static str,
}

/// The [`Capture`]s at `r`'s numbers (the lowest corrupt number over levels 0-4).
pub fn captures(r: &Rules) -> Vec<Capture> {
    let anchor = int(r, "adversary.threat_agents");
    // The levels' cut applies from `corrupt_cut_at` (5: never; the levels are 0-4).
    let cut = if int(r, "adversary.corrupt_cut_at") <= 4 { list(r, "adversary.corrupt_cut") } else { Vec::new() };
    // 263: from `city_cut_at` (5: never; the levels are 0-4) a city corrupts with one agent fewer again.
    let reach = i64::from(int(r, "adversary.city_cut_at") <= 4);
    let n = |key: &str, i: usize| int(r, key) - cut.get(i).copied().unwrap_or(0);
    content()
        .threats
        .iter()
        .filter_map(|t| {
            let (number, site) = match t.locations.first()? {
                Location::City | Location::CoastalCity => (n("adversary.corrupt_at", 0) - reach, "city"),
                Location::Station => (n("adversary.corrupt_station_at", 1), "research station"),
                Location::Factory => (n("adversary.corrupt_factory_at", 2), "factory"),
            };
            let made: i64 = t
                .effects
                .iter()
                .filter_map(|e| match e {
                    Op::Create { count, at: CreateAt::Location, .. } => Some(*count),
                    _ => None,
                })
                .sum();
            let number = number.max(1);
            (made > 0 && made + anchor >= number).then(|| Capture { name: t.name.clone(), tier: t.tier, made, number, site })
        })
        .collect()
}

/// The harms a [`Capture`] adds: the corruption's (a node on the hex and the ambassadors there go with it).
fn capture_harms(c: &Capture) -> [usize; 3] {
    let lost = if c.site == "city" { CITY_LOST } else { FACILITIES_CORRUPTED };
    [lost, NODE_DESTROYED, AMBASSADORS_LOST]
}

/// `(harm, source)` marks; with `r`, a threat's capture ([`captures`]) counts too.
fn harm_marks(r: Option<&Rules>) -> BTreeMap<(usize, usize), Mark> {
    let c = content();
    let mut counts: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    let taken: Vec<Capture> = r.map(captures).unwrap_or_default();
    for t in &c.threats {
        let mut hs = all_harms(&t.effects);
        if let Some(cap) = taken.iter().find(|x| x.name == t.name) {
            hs.extend(capture_harms(cap));
        }
        hs.sort_unstable();
        hs.dedup();
        for h in hs {
            *counts.entry((h, t.tier as usize - 1)).or_default() += 1;
        }
    }
    let mut add = |src: usize, ops: &[Op]| {
        for h in all_harms(ops) {
            *counts.entry((h, src)).or_default() += 1;
        }
    };
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
    // 293: the research row's icons, each slot once, under the research source (a row edit moves the ledger).
    if let Some(row) = &c.adversary_board.research {
        let research = SOURCES.iter().position(|x| *x == "adversary research").expect("the research source");
        for id in &row.slots {
            if let Some(icon) = c.adversary_board.icons.iter().find(|i| &i.id == id) {
                add(research, &icon.effects);
            }
        }
    }
    for p in &c.projects {
        add(ADVERSARY_PROJECTS, &p.effects);
    }
    let mut out: BTreeMap<(usize, usize), Mark> =
        counts.into_iter().map(|(k, n)| (k, Mark::Count(n))).collect();
    for &(src, harm, _) in RULE_HARMS {
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
pub fn harm_gaps(r: &Rules) -> Vec<(&'static str, usize)> {
    let marks = harm_marks(Some(r));
    HARMS.iter().enumerate().map(|(h, name)| (*name, kinds(&marks, h))).filter(|&(_, n)| n < 2).collect()
}

/// The kinds of source over about half of the printed node destruction: "about half" with its original one-mark
/// slack (191, amendment 8617: a Corrupt line no longer counts, as it takes a node only where agents already stand).
/// 191 (default of question 1, Matt's to confirm): reported as a gap, the adversary-card pass's to fix.
pub fn node_share_gaps(r: &Rules) -> Vec<String> {
    let marks = harm_marks(Some(r));
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

/// What fails the ledger's distribution checks; 286 (D130): a tier-1 threat that takes its own site (the tier
/// standard: a tier-1 harm is never permanent).
pub fn harm_failures(r: &Rules) -> Vec<String> {
    let marks = harm_marks(Some(r));
    let mut bad: Vec<String> = captures(r)
        .iter()
        .filter(|c| c.tier == 1)
        .map(|c| format!("tier-1 {}: creates {} on its {}, which falls at {} with the anchor agent", c.name, c.made, c.site, c.number))
        .collect();
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
    let marks = harm_marks(Some(r));
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
    let mut gaps: String = harm_gaps(r)
        .iter()
        .map(|(h, n)| format!("- {h}: {n} kind{} of source.\n", if *n == 1 { "" } else { "s" }))
        .collect();
    gaps.extend(node_share_gaps(r).iter().map(|g| format!("- node destroyed, {g}.\n")));
    format!(
        "# The harms ledger\n\nGenerated by `aic-sim docs` from `content/` and `RULE_HARMS` in \
         `crates/aic-sim/src/docs/harms.rs`: edit those, not this file. Rules revision {rev}. Keep it \
         current: a change to threats, Discord cards or adversary rules that adds or moves a harm \
         updates `RULE_HARMS` or `op_harms` (every op has its arm there, so none falls through unread).\n\n\
         One row per harm an adversary source can do, one column per kind of source. A number \
         is how many printed cards do it (a threat, Discord card, twist or project counts once); \
         `yes` is a rule of the board. Insight is no adversary harm, and costs raised has no row yet. \
         `Discord cards` covers both tiers; `troops` is the adversary \
         troops' guard and their strikes on a Strike icon, which take nodes from adversary stage {troop_node} on (0: always; 4: never). The `yes` cells \
         are true at the baseline's numbers ({amb_discord}: an agent \
         darkening an aligned city sways only when that is above 0). A printed \"Botnets spread\" counts at every \
         level, though below {spread_at} it does nothing.\n\n{}\n\
         **Check** (`aic-sim docs --check`, and a fast test): each threat tier at least 5 kinds of harm. Measured \
         shares are in the 034 screen (`experiments/results/screen-034-clocks/clocks.md`).\n\n\
         ## Gaps\n\nThe list for the adversary-card pass, not failures: a harm in under 2 kinds of source, and a kind \
         of source over about half of the printed node destruction (with one mark of slack).\n\n\
         {gaps}\n## Threats that take their own site\n\n{taken}\n## The board's rules\n\n{notes}\n\
         ## Agents in and out, by level\n\n{flow}",
        table(&head, rows),
        rev = aic_core::RULES_REVISION,
        troop_node = tick(r, "board.troop_node"),
        amb_discord = tick(r, "board.ambassador_discord"),
        // 263: the d0 step that skips every "Botnets spread" line.
        spread_at = tick(r, "adversary.botnet_spread_at"),
        flow = agent_flow(),
        taken = taken_md(r),
    )
}

/// 286 (D130): one row per threat whose own creation and anchor agent reach its site's number ([`captures`]).
fn taken_md(r: &Rules) -> String {
    let rows: String = captures(r)
        .iter()
        .map(|c| {
            format!(
                "- **{}** (tier {}): creates {} on its {}; with the anchor agent ({}) it reaches the {}'s number ({}), so it takes \
                 the site uncountered: counted above as the corruption's harms.\n",
                c.name, c.tier, c.made, c.site, tick(r, "adversary.threat_agents"), c.site, c.number
            )
        })
        .collect();
    format!(
        "An uncountered threat's creation and its anchor agent land on one site: where they reach its corrupt number \
         (its lowest, at any level, on an empty site), the capture is the threat's effect. Tier 1 may not (the threat \
         card standard: never permanent; `aic-sim docs --check` refuses one); tier 2 may (286, D130).\n\n{}",
        if rows.is_empty() { "None.\n".to_string() } else { rows }
    )
}

/// One row of [`AGENT_FLOW`].
pub type FlowRow = (u8, f64, f64, f64, f64, f64, f64, f64);

/// 275: agents put on the board and taken off, a game-round, by level, the judge on the read named in
/// [`AGENT_FLOW_READ`]: (level, arrivals made, of them by Infiltrate, canceled on arrival, removed by players,
/// returned by corruptions, cities corrupted by round 4, by the game's end). Measured, not computed: refresh it from
/// a read after a change to the adversary (`experiments/results/275_adversary_redesign/flow.py`).
pub const AGENT_FLOW: [FlowRow; 5] = [
    (0, 7.1, 4.4, 0.5, 2.2, 2.1, 0.2, 0.5),
    (1, 8.6, 3.4, 0.6, 1.5, 4.5, 1.7, 3.7),
    (2, 9.0, 2.9, 0.5, 1.4, 5.3, 2.2, 4.9),
    (3, 10.1, 2.6, 0.4, 1.1, 6.5, 3.0, 5.2),
    (4, 10.9, 4.2, 0.4, 1.0, 7.4, 3.1, 4.2),
];
/// The read [`AGENT_FLOW`] comes from.
pub const AGENT_FLOW_READ: &str = "f275-read, r130, seeds 49884-49931, 144 games a level";

fn agent_flow() -> String {
    let rows = AGENT_FLOW
        .iter()
        .map(|&(d, made, inf, cancel, removed, corrupt, r4, end)| {
            [d.to_string()]
                .into_iter()
                .chain([made, inf, cancel, removed, corrupt, r4, end].map(|x| format!("{x:.1}")))
                .collect()
        })
        .collect();
    format!(
        "The sources and sinks of agents a game-round (the judge, {AGENT_FLOW_READ}). Arrivals are every agent the \
         adversary puts on the board from its supply (when the table was read: Infiltrate, Surge, Grow, botnet seeds, \
         threats' anchors and creation lines; since r140: Infiltrate, Escalate, a factory's own agent, threats' \
         anchors and creation lines, a factory's push not counted); a canceled one meets a player ambassador and both come off; removed is the \
         players' removals; corruption returns a corrupted site's agents to the supply.\n\n{}",
        table(
            &["Level", "Arrivals", "Of them Infiltrate", "Canceled", "Removed by players", "Back by corruption",
              "Cities corrupted by round 4 (a game)", "By the end (a game)"],
            rows
        )
    )
}

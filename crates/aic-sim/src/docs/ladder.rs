//! 267 (Matt's ruling, modeled on Spirit Island's adversary card): the difficulty ladder as players
//! track it. Each level is named in content (`AdversaryBoard::levels`) and adds **setup** (markers,
//! the deck, read here from the config, `adversary.*`) and at most one **triggered mid-game rule**
//! (the level's `rule`). One reader for the adversary.md table, rulebook §21, the reference card
//! (`web/card/adversary.json`) and `scripts/design_check.py`'s ladder budget.

use aic_core::content::content;
use aic_core::op::Op;
use serde::Serialize;

use super::blocks::{dealt, int, list};
use super::table;
use crate::cli::fail;
use aic_core::Rules;

/// One level of the ladder, in players' words.
#[derive(Debug, Clone, Serialize)]
pub(super) struct Level {
    pub level: i64,
    pub name: String,
    pub flavor: String,
    /// What setup this level changes from the one below (level 0: the whole setup).
    pub setup: Vec<String>,
    /// Its named mid-game rule ("" for setup only).
    pub rule: String,
    /// Named rules this level adds (0 or 1: content prints one a level) and those active at it.
    pub new_rules: usize,
    pub active_rules: usize,
    /// 267 (second review): the mid-game changes behind them, one a mid-game dial starting at this level (a named
    /// rule may merge two), and those active; `design_check.py`'s ladder budget reads these.
    pub new_changes: usize,
    pub active_changes: usize,
}

/// The engine dials that change play mid-game, and the level each starts at: each must sit at a level
/// whose content names a rule, so no mid-game change goes unprinted. 270 (Matt, amendment 12344):
/// `botnet_spread_at` is not one: each "Botnets spread" card prints its "(level N+)" (checked below).
const MID_GAME_DIALS: [&str; 4] = [
    "adversary.corrupt_cut_at",
    "adversary.city_cut_at",
    "adversary.board_difficulty_at",
    "adversary.research_strong_at",
];

fn entry(l: &[i64], level: i64) -> i64 {
    l[(level as usize).min(l.len() - 1)]
}

/// The setup a level prints, whole (`changed` against the level below picks what it adds).
fn setup_items(r: &Rules, level: i64) -> Vec<(&'static str, String)> {
    let n = dealt(r, level);
    let mut out = vec![
        ("compute", format!("Compute income {}", entry(&list(r, "adversary.compute_income"), level))),
        ("agents", format!("Agent income +{}", entry(&list(r, "adversary.ambassador_income"), level))),
        ("research", format!("Research threshold {}", entry(&list(r, "adversary.research_at"), level))),
        ("deck", format!("Deck {} / {} / {} ({} rounds)", n[0], n[1], n[2], n.iter().sum::<usize>())),
        // Printed on the map's sites: the same at every level (a level's rule may say a city falls sooner).
        ("sites", format!(
            "Sites print their corrupt numbers: city {}, station {}, factory {}, land {}",
            int(r, "adversary.corrupt_at"),
            int(r, "adversary.corrupt_station_at"),
            int(r, "adversary.corrupt_factory_at"),
            int(r, "adversary.corrupt_land_at")
        )),
    ];
    if level >= 4 {
        let disc = int(r, "adversary.discord_removed");
        if disc > 0 {
            out.push(("discord", format!("{disc} Discord card{} removed", if disc == 1 { "" } else { "s" })));
        }
        let extra = int(r, "adversary.extra_corrupted");
        if extra > 0 {
            out.push(("extra", format!("{extra} more corrupted cit{} a country", if extra == 1 { "y" } else { "ies" })));
        }
    }
    out
}

/// The ladder, level 0 (the board as printed) to the top.
pub(super) fn ladder(r: &Rules) -> Vec<Level> {
    let board = &content().adversary_board;
    let top = r.tunables().into_iter().find(|t| t.key == "adversary.difficulty").map_or(4, |t| t.max);
    for dial in MID_GAME_DIALS {
        let at = int(r, dial);
        if at <= 0 || at > top {
            continue;
        }
        let named = board.levels.iter().any(|v| v.level == at && !v.rule.is_empty());
        if !named {
            fail(&format!("docs: {dial} {at} changes play mid-game at a level whose content names no rule (267)"));
        }
    }
    // 267: a level lowering a station's, factory's or land's number is mid-game arithmetic: the ladder prints
    // only the city's (Sleeper Cells), so any other entry fails.
    if list(r, "adversary.corrupt_cut").iter().skip(1).any(|&c| c > 0) {
        fail("docs: corrupt_cut lowers a station, factory or land number: print it on the sites instead (267)");
    }
    // The city's number the level's rule prints is the config's (one home: `corrupt_at` less `corrupt_cut`).
    let cut_at = int(r, "adversary.corrupt_cut_at");
    let printed = int(r, "adversary.corrupt_at");
    let city = (printed - list(r, "adversary.corrupt_cut").first().copied().unwrap_or(0)).max(1);
    if let Some(v) = board.levels.iter().find(|v| v.level == cut_at) {
        let says = v.rule.contains(&format!("falls at {city} agents, not {printed}"));
        if (city != printed) != says {
            fail(&format!("docs: level {cut_at} ({}): its rule and \"falls at {city} agents, not {printed}\" (corrupt_at less corrupt_cut) disagree", v.name));
        }
    }
    // 270 (amendment 12344): a spread is card text, "(level N+)" at `botnet_spread_at` on every card that spreads
    // (the events' lines and Botnet Swarm), and no level's rule names it.
    let spread_at = int(r, "adversary.botnet_spread_at");
    for v in &board.levels {
        if v.rule.contains("Botnets spread") {
            fail(&format!("docs: level {} ({}): \"Botnets spread\" is the cards' \"(level {spread_at}+)\", not a level's rule", v.level, v.name));
        }
    }
    let mark = format!("(level {spread_at}+)");
    let spreads = |ops: &[Op]| ops.iter().any(|o| matches!(o, Op::SpreadBotnets { .. }));
    let c = content();
    let cards = c.adversary.iter().filter(|e| spreads(&e.standing)).map(|e| (&e.name, &e.text));
    let projects = c.projects.iter().filter(|p| spreads(&p.effects)).map(|p| (&p.name, &p.text));
    for (name, text) in cards.chain(projects) {
        if (spread_at > 0) != text.contains(&mark) {
            fail(&format!("docs: {name}: a Botnets spread prints \"{mark}\" exactly when `botnet_spread_at` is above 0"));
        }
    }
    // The other numbers a level's rule prints are content's: War Footing's compute and Arms Race's Muster.
    let says = |at: &str, words: String| {
        let at = int(r, at);
        if let Some(v) = board.levels.iter().find(|v| v.level == at) {
            if !v.rule.contains(&words) {
                fail(&format!("docs: level {at} ({}) must print {words:?}, as its effect does", v.name));
            }
        }
    };
    let compute: i64 = board.difficulty.effects.iter().map(|e| if let Op::ThreatCompute { amount } = e { *amount } else { 0 }).sum();
    if compute > 0 {
        says("adversary.board_difficulty_at", format!("{compute} compute"));
    }
    // War Footing's words are the board's `difficulty` line's, one text.
    let at = int(r, "adversary.board_difficulty_at");
    if let Some(v) = board.levels.iter().find(|v| v.level == at) {
        if v.rule != board.difficulty.text {
            fail(&format!("docs: level {at} ({})'s rule and the board's difficulty line differ: make them one text", v.name));
        }
    }
    let base_muster = board.research.as_ref().and_then(|x| x.effects.first());
    if let (Some(Op::Troops { count, .. }), Some(Op::Troops { count: base, .. })) =
        (board.research.as_ref().and_then(|x| x.strong.effects.first()), base_muster)
    {
        says("adversary.research_strong_at", format!("musters {count}, not {base}"));
    }
    let mut out: Vec<Level> = Vec::new();
    let mut active = 0;
    let mut changes = 0;
    let mut below: Vec<(&str, String)> = Vec::new();
    for level in 0..=top {
        let items = setup_items(r, level);
        let setup: Vec<String> = if level == 0 {
            items.iter().map(|(_, t)| t.clone()).collect()
        } else {
            items.iter().filter(|it| !below.contains(it)).map(|(_, t)| t.clone()).collect()
        };
        below = items;
        let named = board.levels.iter().find(|v| v.level == level);
        let rule = named.map_or(String::new(), |v| v.rule.clone());
        let new_rules = usize::from(!rule.is_empty());
        active += new_rules;
        let new_changes = MID_GAME_DIALS.iter().filter(|d| int(r, d) == level && level > 0).count();
        changes += new_changes;
        out.push(Level {
            level,
            name: named.map_or_else(|| board.name.clone(), |v| v.name.clone()),
            flavor: named.map_or(String::new(), |v| v.flavor.clone()),
            setup,
            rule,
            new_rules,
            active_rules: active,
            new_changes,
            active_changes: changes,
        });
    }
    out
}

fn or_dash(s: String) -> String {
    if s.is_empty() { "—".into() } else { s }
}

/// rules/adversary.md's *Difficulty* table: each level's setup and named rule, the rules active, and the dials.
pub(super) fn difficulty_block(r: &Rules) -> String {
    let keys: [&[&str]; 5] = [
        &["adversary.compute_income", "adversary.ambassador_income", "adversary.research_at", "adversary.corrupt_at"],
        &["adversary.botnet_spread_at", "adversary.corrupt_cut", "adversary.corrupt_cut_at"],
        &["adversary.board_difficulty_at"],
        &["adversary.stage2_removed", "adversary.stage1_added", "adversary.research_strong_at"],
        &["adversary.stage1_removed", "adversary.stage2_added", "adversary.stage3_added", "adversary.stage_swap_at"],
    ];
    let rows = ladder(r)
        .into_iter()
        .map(|v| {
            // Names only: the setup column prints the values (tunables.md has every one).
            let k = keys.get(v.level as usize).map_or(String::new(), |k| {
                k.iter().map(|k| format!("`{}`", k.trim_start_matches("adversary."))).collect::<Vec<_>>().join(", ")
            });
            vec![
                v.level.to_string(),
                format!("**{}**", v.name),
                or_dash(v.setup.join("; ")),
                or_dash(v.rule),
                v.active_rules.to_string(),
                k,
            ]
        })
        .collect();
    table(&["Level", "Name", "Setup (incomes and threshold a country)", "Mid-game rule (new)", "Rules active", "Tunable"], rows)
}

/// 229, 267: rulebook §21's table, the same ladder in players' words (no tunable names).
pub(super) fn rulebook_difficulty_block(r: &Rules) -> String {
    let rows = ladder(r)
        .into_iter()
        .map(|v| {
            let name = if v.flavor.is_empty() { format!("**{}**", v.name) } else { format!("**{}**: *{}*", v.name, v.flavor) };
            vec![v.level.to_string(), name, or_dash(v.setup.join("; ")), or_dash(v.rule), v.active_rules.to_string()]
        })
        .collect();
    table(&["Level", "Name", "Setup (set once; incomes and threshold a country)", "Mid-game rule it adds", "Rules active"], rows)
}

/// The adversary's escalation as the event deck carries it, a line a stage: each stage's threats a country
/// (the most cards print) and whether its cards Muster, read from content.
fn escalation() -> Vec<String> {
    let events = &content().adversary;
    (1..=3u8)
        .filter_map(|stage| {
            let cards: Vec<_> = events.iter().filter(|c| c.stage == stage).collect();
            if cards.is_empty() {
                return None;
            }
            let lo = cards.iter().map(|c| c.pulls.len()).min().unwrap_or(1);
            let hi = cards.iter().map(|c| c.pulls.len()).max().unwrap_or(1);
            let tiers: std::collections::BTreeSet<u8> = cards.iter().flat_map(|c| c.pulls.iter().map(|p| p.tier)).collect();
            let tiers: Vec<String> = tiers.iter().map(|t| t.to_string()).collect();
            let muster = cards.iter().all(|c| c.text.contains("Muster"));
            let numeral = ["I", "II", "III"][stage as usize - 1];
            let n = if lo == hi { lo.to_string() } else { format!("{lo}-{hi}") };
            let s = if hi == 1 { "" } else { "s" };
            let t = if tiers.len() == 1 { "tier" } else { "tiers" };
            Some(format!("Stage {numeral}: {n} threat{s} a country, {t} {}{}", tiers.join(" and "), if muster { "; Muster" } else { "" }))
        })
        .collect()
}

/// `web/card/adversary.json`: the reference card's words (the board's rules, its research rule and the ladder),
/// for `web/scripts/cards-export.mjs`.
pub(super) fn adversary_json(r: &Rules) -> String {
    let b = &content().adversary_board;
    #[derive(Serialize)]
    struct Card<'a> {
        id: &'a str,
        name: &'a str,
        rules: Vec<(&'a str, &'a str)>,
        research: Option<(&'a str, &'a str)>,
        escalation: Vec<String>,
        levels: Vec<Level>,
    }
    let card = Card {
        id: &b.id,
        name: &b.name,
        rules: b.rules.iter().map(|x| (x.name.as_str(), x.text.as_str())).collect(),
        // The stronger effect is a level's named rule (Arms Race), so the card prints it there.
        research: b.research.as_ref().map(|x| (x.name.as_str(), x.text.as_str())),
        escalation: escalation(),
        levels: ladder(r),
    };
    serde_json::to_string_pretty(&card).expect("the card is JSON") + "\n"
}

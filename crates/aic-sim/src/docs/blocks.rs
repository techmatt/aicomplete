//! The blocks of `docs/rules/`, each a pure function of `Rules` or `Content`,
//! and the house terms.

use aic_core::config::ParamValue;
use aic_core::content::{content, AiBoard, Content, Cost, Megaproject, Resource, Uncover};
use aic_core::modules::adversary;
use aic_core::rules::Tunable;
use aic_core::Rules;

use super::table;
use crate::cli::fail;

/// 119: `docs/agents.md`'s strategies as a player says them, each board's in turn: the one
/// home of the words is `configs/agents/strategies.json` (`says`).
pub fn strategies_block() -> String {
    let mut out = String::new();
    let mut board = "";
    for s in &crate::agents::strategy::strategies().strategies {
        if s.board != board {
            let name = content().ais.iter().find(|b| b.id == s.board).map_or(s.board.as_str(), |b| b.name.as_str());
            out.push_str(&format!("{}**The {name}**\n\n", if board.is_empty() { "" } else { "\n" }));
            board = &s.board;
        }
        out.push_str(&format!("- *{}.* {}\n", s.name, s.says));
    }
    out
}

/// The words cards lean on, once: `docs/rules/README.md`'s *Terms* and the player aid print these.
pub const HOUSE_TERMS: [(&str, &str); 22] = [
    ("Facility", "a research station or factory on the board. A building is always a card. Harden goes on a facility and nothing else"),
    ("Ambassador", "a player's piece, any player's: a card never says whose"),
    ("Agent", "the adversary's piece on the board (the Security token is an Operative)"),
    ("Troop", "an adversary troop; a megacorp's is a Drone, its own piece, waiting at its military bases"),
    ("In reach", "on a hex within 1 of your nodes"),
    ("A country you're in", "a country where you have a node"),
    ("Your home country", "the country you start in"),
    ("A threat", "on your card: one you may counter, near it (your node or ambassador on or next to one of its tokens)"),
    ("Near it", "on a threat: on or next to one of its location tokens"),
    ("Here", "on a building: on this card, where its tokens sit"),
    ("Remove N", "as many as there are, up to N"),
    ("At N", "on a building's Effect: when its tokens here reach N, N of them come off and the rest happens"),
    ("Draw", "from the tier-1 deck, with no keep to pay; if it is empty, reshuffle its discard first"),
    ("On your cards", "on tags: on your cards in play, covered buildings included, and your board's tags, printed or revealed by its tracks. Tokens count only on the card that holds them"),
    ("Action", "once a round, always; \"A, OR B\" is one use, either half"),
    ("Open megaproject", "one that isn't complete; research cubes in your color go only onto one"),
    ("Liberate", "a corrupted site: the agents on it come off, the adversary's cube comes off and yours goes on (the action pays for it in ambassadors; a card's is unpaid)"),
    ("Convert", "an agent on or next to the city leaves its stack and becomes your ambassador where it stood"),
    ("Gather up to N", "pieces (ambassadors, agents, troops; botnets and markers never move) into a hex in your reach, each from a hex next to it; you may stop at any piece (a strike, \"remove up to N on or next to a hex\", moves nothing)"),
    ("Push up to N", "pieces from a hex in your reach, each to a hex next to it that you choose; you may stop at any piece"),
    ("Firewall", "a token on a hex: no agent lands or moves there (a Rally's share for it stays in the supply) and nothing corrupts or flips its site; it comes off at the end of the next adversary turn"),
    ("In the next adversary turn, no X", "a Unity card's prohibition: a token on each hex where the condition holds when the card resolves; that step skips a hex with its token (a Rally's share for it stays in the supply), and the tokens come off with the firewalls"),
];

/// `3 credits, 1 energy`.
fn costs(c: &[Cost]) -> String {
    let parts: Vec<String> = c.iter().map(|(r, n)| r.words(*n)).collect();
    parts.join(", ")
}

pub(super) fn house_terms() -> String {
    HOUSE_TERMS.iter().map(|(term, says)| format!("- **{term}**: {says}.\n")).collect()
}

/// 135: the threat decks by tier and tag (every card, with its text and menu,
/// is in docs/adversary-cards.md: 64 threats would overrun this page's budget).
pub(super) fn threats_block() -> String {
    use aic_core::content::ThreatTag;
    const TAGS: [ThreatTag; 6] = [
        ThreatTag::Security,
        ThreatTag::Diplomacy,
        ThreatTag::Cyber,
        ThreatTag::Industry,
        ThreatTag::Power,
        ThreatTag::Universal,
    ];
    let c = content();
    let rows = (1..=3u8)
        .map(|tier| {
            let deck: Vec<_> = c.threats.iter().filter(|t| t.tier == tier).collect();
            let mut row = vec![tier.to_string(), deck.len().to_string()];
            row.extend(TAGS.iter().map(|tag| deck.iter().filter(|t| t.tags.contains(tag)).count().to_string()));
            row
        })
        .collect();
    table(&["Tier", "Cards", "Security", "Diplomacy", "Cyber", "Industry", "Power", "Universal"], rows)
}

fn pull_words(p: &aic_core::content::Pull) -> String {
    let tags: Vec<String> = p.tags.iter().map(|t| String::from(*t)).collect();
    format!("tier {} ({})", p.tier, tags.join(" or "))
}

/// 138: an event card's Corrupt X lines ("cities, land (1 fewer)").
fn corrupt_words(a: &aic_core::content::AdversaryCard) -> String {
    a.twists
        .iter()
        .filter_map(|t| match t {
            aic_core::content::Op::Corrupt { site, cut } => {
                let name = site.plural();
                Some(if *cut > 0 { format!("{name} ({cut} fewer)") } else { name.to_string() })
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub(super) fn adversary_block() -> String {
    let rows = content()
        .adversary
        .iter()
        .map(|a| {
            let pulls: Vec<String> = a.pulls.iter().map(pull_words).collect();
            let rally = aic_core::modules::adversary::rally_site(a).map(|r| r.plural()).unwrap_or("");
            let deploy: i64 = a
                .standing
                .iter()
                .map(|e| match e {
                    aic_core::content::Op::Troops { count, each_country: true, .. } => *count,
                    _ => 0,
                })
                .sum();
            vec![
                a.name.clone(),
                a.stage.to_string(),
                pulls.join("; "),
                rally.to_string(),
                deploy.to_string(),
                corrupt_words(a),
            ]
        })
        .collect();
    table(&["Card", "Stage", "Threat (each country)", "Rally", "Muster", "Corrupt"], rows)
}

pub(super) fn alignment_block() -> String {
    let rows = content()
        .alignment
        .iter()
        .collect::<Vec<_>>();
    // 156: the three tiers' text is in docs/cards.md (20+ cards would overrun this page's budget).
    let mut out = String::new();
    for side in [aic_core::content::Side::Unity, aic_core::content::Side::Discord] {
        let names: Vec<String> = rows
            .iter()
            .filter(|a| a.side == side)
            .map(|a| if a.draft { format!("{} (draft)", a.name) } else { a.name.clone() })
            .collect();
        out.push_str(&format!("- **{}** ({}): {}.\n", side.name(), names.len(), names.join(", ")));
    }
    out.push_str("- Each card's three strengths: [cards.md](../cards.md#unity-and-discord-cards).\n");
    out
}

pub(super) fn projects_block() -> String {
    let rows = content()
        .projects
        .iter()
        .map(|p| {
            let tags: Vec<&str> = p.tags.iter().map(|t| t.name()).collect();
            // 175: printed per player count (the cost is per player: 2, 3 and 4 players read off the card;
            // 184: and the solo game's, 1 player).
            vec![
                p.name.clone(),
                tags.join(" + "),
                p.cost.to_string(),
                (2 * p.cost).to_string(),
                (3 * p.cost).to_string(),
                (4 * p.cost).to_string(),
                if p.ongoing { "ongoing" } else { "once" }.to_string(),
            ]
        })
        .collect();
    table(&["Project", "Tags", "1 player", "2 players", "3 players", "4 players", "Resolves"], rows)
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

pub(super) fn tunables_block(r: &Rules) -> String {
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
        .unwrap_or_else(|| fail(&format!("docs: no tunable {key}")))
}

fn int(r: &Rules, key: &str) -> i64 {
    match tunable(r, key).value {
        ParamValue::Int(n) => n,
        ParamValue::List(_) => fail(&format!("docs: {key} is a list")),
    }
}

fn list(r: &Rules, key: &str) -> Vec<i64> {
    match tunable(r, key).value {
        ParamValue::Int(n) => vec![n],
        ParamValue::List(l) => l,
    }
}

/// A tunable as the tables print it: `` `botnet_compute` 1 ``, or with its module
/// where the bare name says little (`` `alignment.botnet` 2 ``).
pub(super) fn tick(r: &Rules, key: &str) -> String {
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
pub(super) fn countries_block(c: &Content) -> String {
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
                numbers(&[&[p.neutral_station][..], &p.more_stations].concat()),
                numbers(&p.station_sites),
                format!("{} ({})", numbers(&p.industry), numbers(&p.factories)),
                p.node.to_string(),
            ]
        })
        .collect();
    table(
        &[
            "#",
            "Name",
            "Cities (corrupted first)",
            "Ocean",
            "Station (botnet)",
            "Neutral stations",
            "Empty station sites",
            "Industrial zones (setup factories)",
            "Starting node",
        ],
        rows,
    )
}

/// The corps: each piece's megaproject, what it pays and what it prints after.
pub(super) fn corps_block(c: &Content) -> String {
    let rows = c
        .map
        .countries
        .iter()
        .map(|p| {
            let m = &p.megaproject;
            vec![
                format!("{} ({})", p.name, p.flavor),
                m.name.clone(),
                m.cubes.to_string(),
                format!("{} / {}", Megaproject::words(&m.player), Megaproject::words(&m.adversary)),
                m.converter.words(),
                // 199: the chip converter, where the card prints one.
                m.chip_converter.as_ref().map_or_else(|| "none".to_string(), |c| c.words()),
            ]
        })
        .collect();
    table(
        &[
            "Corp",
            "Megaproject",
            "Cubes",
            "At completion: player cube / adversary cube",
            "Converter (as often as you can pay, unless marked)",
            "Chip converter",
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
        .unwrap_or_else(|e| fail(&format!("docs: {e}")));
    let level = Rules::new(&config).unwrap_or_else(|e| fail(&format!("docs: difficulty {difficulty}: {e}")));
    let g = level.new_game(1);
    let mut out = [0; 3];
    for pile in [adversary::DECK, adversary::NEXT, adversary::CURRENT, adversary::DISCARD] {
        for id in g.pile(pile) {
            let card = content().adversary.iter().find(|c| c.id == id.as_ref());
            let stage = card.unwrap_or_else(|| fail("docs: an adversary card not in content")).stage;
            out[stage as usize - 1] += 1;
        }
    }
    out
}

/// The round cap and the adversary deck's shape at each difficulty.
pub(super) fn round_cap_block(r: &Rules) -> String {
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

/// Level 4's row: a clause for each of its three dials that does something (177, 182: a dial at 0 prints none).
fn level4_text(r: &Rules) -> String {
    let mut clauses: Vec<String> = Vec::new();
    if int(r, "adversary.discord_removed") > 0 {
        clauses.push("A still shorter Discord deck: the removed cards count as tips already made, so the Discord strengths come a tip sooner".into());
    }
    if int(r, "adversary.tier2_early") > 0 {
        clauses.push("tier-2 threats sooner".into());
    }
    let extra = int(r, "adversary.extra_corrupted");
    if extra > 0 {
        clauses.push(format!("{extra} more corrupted {} per country at setup", if extra == 1 { "city" } else { "cities" }));
    }
    let mut text = clauses.join("; ");
    if let Some(c) = text.get(0..1) {
        text = c.to_uppercase() + &text[1..];
    }
    if int(r, "adversary.discord_removed") == 0 {
        let note = "the Discord deck as at level 3: `discord_removed` 0";
        text = if text.is_empty() { format!("Nothing more: {note}") } else { format!("{text} ({note})") };
    }
    text
}

/// What each difficulty level adds.
pub(super) fn difficulty_block(r: &Rules) -> String {
    let t = |keys: &[&str]| {
        let k: Vec<String> = keys.iter().map(|k| tick(r, k)).collect();
        k.join(", ")
    };
    // Each of `steps`' six dials (`corrupt_cut_at` .. `stage_swap_at`) sits at whatever level its `_at` value names.
    let mut rows: Vec<(String, Vec<&str>)> = vec![
        // The incomes are a list with an entry a level, not a step: every level reads its own entry.
        (
            "Base rules; the incomes, an entry a level".into(),
            vec!["adversary.compute_income", "adversary.ambassador_income"],
        ),
        (String::new(), vec![]),
        (String::new(), vec![]),
        (
            "Stage-II cards removed, stage-I cards added back".into(),
            vec!["adversary.stage2_removed", "adversary.stage1_added"],
        ),
        (level4_text(r), vec!["adversary.discord_removed", "adversary.tier2_early", "adversary.extra_corrupted"]),
    ];
    // 182: the sites `corrupt_cut` lowers, read from the list (city, station, factory, land), not printed whole.
    let cut_sites: Vec<&str> = ["city", "station", "factory", "land"]
        .into_iter()
        .zip(list(r, "adversary.corrupt_cut"))
        .filter(|&(_, cut)| cut > 0)
        .map(|(site, _)| site)
        .collect();
    let cut_text = format!(
        "{} corrupt with fewer agents (from `corrupt_cut_at`)",
        if cut_sites.is_empty() { "no site".to_string() } else { cut_sites.join(", ") }
    );
    let steps: [(&str, &str, &[&str]); 6] = [
        ("adversary.corrupt_cut_at", &cut_text,
            &["adversary.corrupt_cut", "adversary.corrupt_cut_at"]),
        ("adversary.counter_bonus_at", "counters cost more: every resource item, a credits item by its own step, the first option's more still (from `counter_bonus_at`)",
            &["adversary.counter_bonus_all", "adversary.counter_bonus_credits", "adversary.counter_bonus", "adversary.counter_bonus_at"]),
        ("adversary.counter_more_at", "counters cost more again (from `counter_more_at`)",
            &["adversary.counter_more_all", "adversary.counter_more_credits", "adversary.counter_more_at"]),
        ("adversary.board_difficulty_at", "the adversary board's difficulty line (from `board_difficulty_at`)",
            &["adversary.board_difficulty_at"]),
        ("adversary.extra_threats_at", "extra threats each round in all, countries in turn",
            &["adversary.extra_threats", "adversary.extra_threats_at"]),
        ("adversary.stage_swap_at", "a stage-I card swapped for a stage-III card (the table above; from `stage_swap_at`)",
            &["adversary.stage1_removed", "adversary.stage3_added", "adversary.stage_swap_at"]),
    ];
    for (dial, text, keys) in steps {
        // A dial past the top level (5) never applies: it is no level's step.
        let at = int(r, dial).max(0) as usize;
        if at >= rows.len() {
            continue;
        }
        rows[at].0 = if rows[at].0.is_empty() {
            let mut c = text.chars();
            c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
        } else {
            format!("{}; {text}", rows[at].0)
        };
        rows[at].1.extend(keys.iter().copied());
    }
    // A level nothing moved onto says so.
    for row in rows.iter_mut().filter(|r| r.0.is_empty()) {
        row.0 = "Nothing more".into();
    }
    let rows: Vec<(String, String)> = rows.into_iter().map(|(text, keys)| (text, t(&keys))).collect();
    let rows = rows
        .into_iter()
        .enumerate()
        .map(|(i, (adds, keys))| vec![i.to_string(), adds.to_string(), keys])
        .collect();
    table(&["Level", "Adds", "Tunable"], rows)
}

/// What `key`, a list by board (093; one value sets every board), gives `board`.
fn per_board(r: &Rules, key: &str, board: usize) -> String {
    let values = list(r, key);
    values[board % values.len()].to_string()
}

/// What each AI board prints before its tracks.
pub(super) fn boards_block(r: &Rules, c: &Content) -> String {
    let rows: Vec<Vec<String>> = c
        .ais
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let mut printed = vec![(Resource::Credits, b.credits)];
            // r82 (amendment 8615): insight prints only on the card-play track (its start and totals).
            printed.extend(b.income.iter().copied());
            vec![
                b.name.clone(),
                costs(&printed),
                per_board(r, "ai.start_credits", i),
                per_board(r, "ai.slots", i),
                b.tags.iter().map(|t| title(t.name())).collect::<Vec<_>>().join(", "),
                // 149: printed Dispatches, and the most its spaces raise it to.
                format!(
                    "{} (up to {})",
                    b.moves,
                    b.moves + b.economy_track.iter().chain(&b.play_track).map(|u| u.moves).sum::<i64>()
                ),
                b.economy_track.len().to_string(),
                b.play_track.len().to_string(),
                (b.economy_track.len() + b.play_track.len()).to_string(),
            ]
        })
        .collect();
    let head = [
        "Board",
        "Printed income",
        "Starting credits",
        "Slots",
        "Printed tags",
        "Dispatches",
        "Economy track",
        "Card-play track",
        "Tokens in all",
    ];
    table(&head, rows)
}

/// Each board's special rule (innate).
pub(super) fn board_details_block(c: &Content) -> String {
    let rows = c
        .ais
        .iter()
        .map(|b| vec![b.name.clone(), format!("*{}*: {}", b.innate.name, b.innate.text)])
        .collect();
    table(&["Board", "Special rule"], rows)
}

/// One space of a node track.
pub(super) fn space(u: &Uncover) -> String {
    // r82 (amendment 8615): insight prints as the new total.
    let mut parts: Vec<String> = u.insight.map(|n| format!("**insight {n}**")).into_iter().collect();
    parts.push(gains(&u.income));
    if u.keep2 {
        parts.push("**keep upgrade**".to_string());
    }
    parts.extend(u.tags.iter().map(|t| format!("{} tag", title(t.name()))));
    if u.moves > 0 {
        parts.push(format!("**Dispatch +{}**", u.moves));
    }
    if aic_core::modules::ai::builds_station(u) {
        parts.push("builds a free research station in reach".to_string());
    }
    if let Some(c) = &u.capstone {
        parts.push(format!("**{}** (capstone): {}", c.name, c.text));
    }
    let mut parts: Vec<String> = parts.into_iter().filter(|p| !p.is_empty()).collect();
    if parts.is_empty() {
        parts.push("nothing".to_string());
    }
    // 179 (Matt, friction #16): the space's cost prints as a cost, first: what you pay to uncover it.
    let out = parts.join(", ");
    if u.cost != 0 {
        format!("pay {} energy: {out}", u.cost)
    } else {
        out
    }
}

/// A node track of every board, space by space. `first` labels space 1.
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

pub(super) fn node_track_block(c: &Content) -> String {
    track_block(&c.ais, |b| &b.economy_track, "1 (on the board at setup)")
}

pub(super) fn node_track2_block(c: &Content) -> String {
    let start = c.ais.iter().map(|b| format!("insight {}", b.insight)).collect::<Vec<_>>();
    let mut out = track_block(&c.ais, |b| &b.play_track, "1");
    // r82 (amendment 8615): the printed start, left of space 1 and never covered.
    let row = format!("| printed start | {} |\n", start.join(" | "));
    let at = out.find("\n|---").and_then(|i| out[i + 1..].find('\n').map(|j| i + 1 + j + 1)).unwrap_or(out.len());
    out.insert_str(at, &row);
    out
}

pub(super) fn powers_block(c: &Content) -> String {
    let mut rows = Vec::new();
    for b in &c.ais {
        for p in &b.powers {
            let mut cost: Vec<String> = p.tags.iter().map(|(t, n)| format!("{n} {}", title(&t.to_string()).replace("Any", "any"))).collect();
            cost.extend(p.cost.iter().map(|(r, n)| r.words(*n)));
            // 105: Synthesis pays with the card it discards, nothing else.
            let cost = if cost.is_empty() { "nothing".to_string() } else { cost.join(" + ") };
            rows.push(vec![b.name.clone(), p.name.clone(), cost, p.text.clone()]);
        }
    }
    table(&["Board", "Power", "Cost", "Effect"], rows)
}

/// The standard actions and projects, with their prices: each action's words
/// are `std_rule_text`'s, beside the view's `std_text` in standard.rs (121).
pub(super) fn standard_actions_block(r: &Rules) -> String {
    use aic_core::modules::board::{std_offered, std_rule_text, StandardAction as A};
    let t = |key: &str| tick(r, key);
    // 206: a cut action (its price param 0) prints no row.
    let w = r.world();
    let on = |a: &&A| std_offered(&w, **a);
    // The rule book's order; selling a card is no standard action, but sits by the actions
    // (149: chips can't be sold).
    let order = [
        A::Node, A::Station, A::Factory, A::Remove, A::Troop, A::Lift, A::Move, A::Ambassador, A::Claim, A::Liberate, A::Botnet,
        A::BuyEnergy, A::HardenChips,
    ];
    let mut rows: Vec<[String; 3]> = order.iter().filter(on).map(|&a| std_rule_text(a, &t)).collect();
    rows.push(["Sell a card".to_string(), format!("gain {}", t("board.sell_credits")), "from your hand".to_string()]);
    rows.extend(
        [A::Insight, A::Envoy, A::Antimatter, A::Breakthrough].iter().filter(on).map(|&a| std_rule_text(a, &t)),
    );
    table(&["Action", "Cost", "What"], rows.into_iter().map(|r| r.to_vec()).collect())
}

/// Where Sway comes from, with the amounts.
pub(super) fn sway_sources_block(r: &Rules) -> String {
    let t = |key: &str| tick(r, key);
    // 200: Biomia's converter read from content (it was a typed string, stale after a content change).
    let biomia = r.world().content.map.countries.iter().find(|p| p.name == "Biomia")
        .map_or_else(|| "none".to_string(), |p| p.megaproject.converter.words());
    let rows: Vec<[String; 3]> = vec![
        ["Unity".to_string(), "removing an adversary troop".to_string(), t("alignment.troop")],
        ["Unity".to_string(), "Biomia's converter".to_string(), biomia],
        [
            "Unity".to_string(),
            "liberating an adversary city (ambassadors.md: a liberate card or the Liberate action)".to_string(),
            t("alignment.free_city"),
        ],
        [
            "Unity".to_string(),
            "liberating an adversary station or factory (the Liberate action)".to_string(),
            t("alignment.free_facility"),
        ],
        ["Unity".to_string(), "removing a botnet".to_string(), t("alignment.botnet")],
        // 099: two Unity sources the table left out.
        [
            "Unity".to_string(),
            "clearing a corruption marker (Recovery, cards)".to_string(),
            t("board.clear_unity"),
        ],
        [
            "Unity".to_string(),
            "player cubes on a megaproject as it completes, as its corp's card prints (Delta Seawall)".to_string(),
            "1 each".to_string(),
        ],
        [
            "Unity".to_string(),
            "cards (Peace Conference, Coalition Charter, Open Assembly), Unity cards".to_string(),
            "card text".to_string(),
        ],
        [
            "Discord".to_string(),
            "energy diverted (a corrupted city) or darkened (an agent on an aligned city)".to_string(),
            format!(
                "{} per unit of a corrupted city's, {} per unit, each round, of a dark aligned city's",
                t("board.divert_discord"),
                t("board.ambassador_discord")
            ),
        ],
        [
            "Discord".to_string(),
            "a factory destroyed in war (not by an adversary troop)".to_string(),
            t("board.destruction_discord"),
        ],
        [
            "Discord".to_string(),
            "adversary cubes on a megaproject as it completes, as its corp's card prints".to_string(),
            "1 each".to_string(),
        ],
        [
            "Discord".to_string(),
            "twists, threats, Discord cards, adversary projects".to_string(),
            "card text".to_string(),
        ],
        [
            "Discord".to_string(),
            "the adversary board's drip, at the start of every round".to_string(),
            format!("as printed ({}: {})", content().adversary_board.name, content().adversary_board.discord_drip),
        ],
        // 145: the land-corruption drip (138) the table left out.
        [
            "Discord".to_string(),
            "each land-corruption token (Blitzkrieg, Naval Blockade), at the start of every round".to_string(),
            t("adversary.blight_discord"),
        ],
    ];
    // 204: the standard antimatter spend; 213 (r94): gone at `breakthrough_unity` 0, no row.
    let mut rows = rows;
    if r.world().int("board.breakthrough_unity") > 0 {
        rows.push(["Unity".to_string(), "Breakthrough (1 antimatter, once a round)".to_string(), t("board.breakthrough_unity")]);
    }
    table(&["Toward", "From", "Amount"], rows.into_iter().map(|r| r.to_vec()).collect())
}

/// What the purge asks at each step of `purge_at` (Unity cards drawn, 136).
pub(super) fn purge_ease_block(r: &Rules) -> String {
    const PIECES: [&str; 3] = ["agents", "adversary troops", "botnets"];
    let at = list(r, "alignment.purge_at");
    // Each step's lower bound and the pieces still counting from it.
    let mut steps: Vec<(i64, usize)> = vec![(0, 0)];
    for (i, &n) in at.iter().enumerate() {
        match steps.last_mut() {
            Some(last) if last.0 == n => last.1 = i + 1,
            _ => steps.push((n, i + 1)),
        }
    }
    let mut out = String::new();
    for (k, &(from, gone)) in steps.iter().enumerate() {
        let range = match steps.get(k + 1) {
            Some(&(next, _)) if next - 1 > from => format!("{from}-{}", next - 1),
            Some(_) => from.to_string(),
            None => format!("{from}+"),
        };
        let left: Vec<&str> = PIECES[gone..].iter().rev().copied().collect();
        let what = if left.is_empty() {
            "only the corrupted cities".to_string()
        } else {
            format!("the corrupted cities, {}", left.join(", "))
        };
        out.push_str(&format!("- {range} Unity tips: remove {what};\n"));
    }
    out.push_str("- corrupted cities always count.\n");
    out
}

/// The draft set's counts (the baseline's `cards` module list) that
/// player-cards.md quotes: insight to play and energy costs.
pub(super) fn draft_set_block(r: &Rules, c: &Content) -> String {
    let ids = r.config.modules.iter().find(|m| m.module == "cards").map_or(&[][..], |m| &m.cards[..]);
    let deck: Vec<_> = ids.iter().map(|id| c.card(id)).collect();
    let count = |f: &dyn Fn(&aic_core::content::CardDef) -> bool| deck.iter().filter(|d| f(d)).count();
    let tier2 = count(&|d| d.tier == 2);
    let at = |i: i64| count(&|d| d.insight == i);
    let building = |d: &aic_core::content::CardDef| d.kind == aic_core::content::CardKind::Building;
    format!(
        "**The draft set in numbers.** {} cards ({} tier 1, {tier2} tier 2). Insight to play: {} / {} / {} cards \
         at 0 / 1 / 2, {} of the {tier2} tier-2 cards at 2. An energy cost: {} cards, {} of the {} buildings.\n",
        deck.len(),
        deck.len() - tier2,
        at(0),
        at(1),
        at(2),
        count(&|d| d.tier == 2 && d.insight == 2),
        count(&|d| d.energy > 0),
        count(&|d| building(d) && d.energy > 0),
        count(&building),
    )
}

/// The opening paragraph for new players.
pub(super) fn new_players_block(r: &Rules) -> String {
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
        "**For new players.** You start with {} tier-1 and {} tier-2 cards in your hand, free: the \
         tier-2 cards are something to plan toward, to play once you meet their needs. Then every round, \
         the first included, you build a hand of {} cards one at a time, \
         choosing for each whether it comes from the cheaper tier-1 deck or the stronger tier-2 \
         deck; you take one card and pass the rest to the next player in seat order until the hands are gone, then \
         pay insight to keep what you drafted ({} for tier 1, {} for tier 2; nothing else buys a keep). \
         Insight is this round's attention: your card-play track shows it, the number on the rightmost \
         space you've uncovered ({} before any), and what you don't spend is lost. Playing a card costs its credits and {} insight, \
         from the same insight as keeps. Buildings go into your AI board's {} slots; once they are \
         full, a new building goes on top of an old one, which stops working but keeps its tags. \
         Programs stay in front of you and are where lasting income comes from (\"raise your credit \
         income by 1 per 2 Science tags on your cards\"); buildings are your engine: Actions you use once a \
         round and Effects that pay each time their trigger happens; events are used once and set aside, never to return. Every card in front of you \
         shows tags (Science, Industry, Cyber, Power, Diplomacy, Security): they unlock some cards, \
         and you can sacrifice cards with the right tags to use your AI board's powers. There is no \
         limit on what you hold or on how far cards raise your income; energy you don't spend stays \
         energy, so drafting cards that spend it is up to you.\n\n\
         **Nodes, for new players.** Nodes don't grow on their own. In the action phase you place \
         nodes with your board's node power, once a round, which costs a tag or the board's own currency and does a \
         little more besides, or with the standard action Place a node, which any player may take twice a round for \
         credits; cards can add more. Every placement, by power, action or card, pays the energy printed on the track space you uncover. You \
         start with {} energy and one city bound to you, which makes energy every round.\n\n\
         **Dispatches, tags and sales, for new players.** A Dispatch puts one of your ambassadors, \
         from your supply or anywhere on the map, onto a piece or botnet in your reach and costs nothing, but you \
         have only your board's Dispatches each round ({} printed, more as Dispatch spaces are \
         uncovered), and what you don't use is lost, like insight. **One tag rule:** \
         the tag printed on your AI board, and any a late, costly track space reveals, count like the \
         tags on your cards, for everything a tag does; the only difference is that a board tag can't \
         be sacrificed. You may sell a card from your hand for {} credit; chips can't be sold.\n",
        n("cards.opening_deal"),
        n("cards.opening_deal_tier2"),
        n("cards.deal"),
        n("cards.keep_tier1"),
        n("cards.keep_tier2"),
        c.ais.first().map_or(0, |b| b.insight),
        range(&insight),
        range(&slots),
        n("ai.start_energy"),
        range(&c.ais.iter().map(|b| b.moves).collect::<Vec<_>>()),
        n("board.sell_credits"),
    )
}

/// How the game ends, for new players.
pub(super) fn game_end_block(r: &Rules) -> String {
    let at = list(r, "alignment.purge_at");
    let easing = match at.as_slice() {
        [a, _, b] => format!(
            "At 0 Unity tips you must purge almost everything: corrupted cities, botnets, adversary troops and \
             agents. After {a} Unity tips the purge no longer asks you to clear \
             agents; after {b}, it asks only that the adversary owns no city, and you win the moment it doesn't."
        ),
        [a, ..] => format!("After {a} Unity tips the purge no longer asks you to clear agents."),
        [] => String::new(),
    };
    let tiers = list(r, "alignment.tier_at");
    let (t2, t3) = (tiers.first().copied().unwrap_or(0), tiers.get(1).copied().unwrap_or(0));
    let unity = content().alignment.iter().filter(|c| c.side == aic_core::content::Side::Unity).count();
    // 175: no stage protects a last node; `board.start_nodes` may give a spare.
    let safe = match int(r, "board.start_nodes") {
        1 => "No stage protects your last node: keep a second one on the board. ".to_string(),
        n => format!("No stage protects your last node; you start with {n}. "),
    };
    format!(
        "**How the game ends, for new players.** The only way to win is the purge, and Unity eases it, \
         step by step, like terror levels. {easing} Pressure comes every round: the face-up adversary event card deals threats \
         by your sites and says where its new agents land (dealt one at a time onto every site of a kind), \
         so you always know where the next wave lands. Agents pile up, a card's Corrupt line \
         moves every agent next to a city onto it, a city holding {} corrupts, a hex with your node and {} \
         loses the node, and corrupted \
         cities and uncountered threats fill the Discord track. Each time it \
         fills, it tips: a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, \
         you lose. Discord cards never destroy your nodes directly, only through the threats and corruptions \
         they bring: they deal more or stronger threats (Offensive \
         aims one at the node hex most crowded with adversary pieces), create agents, run Corrupt \
         lines, and take credits, more of each the more sites are corrupted. Unity has its own track, which nothing on the \
         Discord side empties: free a corrupted \
         city, clear corruption or remove a botnet and you sway Unity, and each Unity tip brings a good card. \
         Like fear cards, each acts by what is on the board, never by your reach: for each site a player owns or \
         each corrupted one, on each hex on or next to your pieces or a corrupted city, or each player from one such \
         hex; some forbid the adversary's next \
         Rally, Muster or Corrupt where your pieces stand, so place them before its turn. \
         Every Unity and Discord card prints three strengths, and the tip's own count on its side picks which \
         one resolves (strength I on tips below {t2}, II from tip {t2}, III from tip {t3}): a later strength counts more \
         or reaches wider, and Discord's late cards hit harder. The {unity} Unity cards reshuffle when the deck runs out. \
         Each Unity you sway may instead take 1 Discord off its track, and Envoy trades one of your \
         ambassadors for the same, so you choose between racing toward Unity and holding off the next \
         Discord card. You also lose if any player has no nodes left on the board. \
         Nodes are lost only where the board is overrun: an uncountered threat near them, a corruption of \
         their own hex (or agents on it), war taking a factory on their hex, a project's raid (the node \
         hex with the most agents), and Border War (nodes by a front's base). A lost token leaves the game and marks its hex, so you can't act from there any more; its \
         income stays. {safe}The adversary deck is a round cap too, {} rounds at \
         most (fewer at the harder levels), and reaching it loses. So there are three different \
         pressures: the Discord deck (politics), your nodes (the board) and the cap.\n",
        int(r, "adversary.corrupt_at"),
        int(r, "adversary.corrupt_node_at"),
        dealt(r, 0).iter().sum::<usize>(),
    )
}

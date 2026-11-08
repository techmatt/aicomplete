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
pub const HOUSE_TERMS: [(&str, &str); 16] = [
    ("Facility", "a research station or factory on the board. A building is always a card. Harden goes on a facility and nothing else"),
    ("Shield", "a hardening token on a hex with your node"),
    ("Ambassador", "yours; the other side's is always an adversary ambassador"),
    ("Troop", "an adversary troop, unless it says corp troop (a megacorp's, waiting at its military bases)"),
    ("In reach", "on a hex within 1 of your nodes"),
    ("A country you're in", "a country where you have a node"),
    ("Your home country", "the country you start in"),
    ("A threat", "on your card: one you may counter, near it (your node or ambassador on or next to one of its tokens)"),
    ("Near it", "on a threat: on or next to one of its location tokens"),
    ("Here", "on a building: on this card, where its tokens sit"),
    ("Remove N", "as many as there are, up to N"),
    ("At N", "on a building's Effect: when its tokens here reach N, N of them come off and the rest happens"),
    ("Draw", "from the tier-1 deck, with no keep to pay; if it is empty, reshuffle its discard first"),
    ("On your cards", "on tags: on your cards in play, covered buildings included, and the tags your board's tracks have revealed. Tokens count only on the card that holds them"),
    ("Action", "once a round, always; \"A, OR B\" is one use, either half"),
    ("Open megaproject", "one that isn't complete; research cubes in your color go only onto one"),
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
        ThreatTag::Military,
        ThreatTag::Influence,
        ThreatTag::Cyber,
        ThreatTag::Economic,
        ThreatTag::Grid,
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
    table(&["Tier", "Cards", "Military", "Influence", "Cyber", "Economic", "Grid", "Universal"], rows)
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
                let name = match site.name() {
                    "city" => "cities",
                    "station" => "research stations",
                    "factory" => "factories",
                    other => other,
                };
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
            vec![
                a.name.clone(),
                a.stage.to_string(),
                pulls.join("; "),
                corrupt_words(a),
            ]
        })
        .collect();
    table(&["Card", "Stage", "Pulls (each country)", "Corrupt"], rows)
}

pub(super) fn alignment_block() -> String {
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

pub(super) fn projects_block() -> String {
    let rows = content()
        .projects
        .iter()
        .map(|p| {
            let tags: Vec<&str> = p.tags.iter().map(|t| t.name()).collect();
            vec![
                p.name.clone(),
                tags.join(" + "),
                p.cost.to_string(),
                if p.ongoing { "ongoing" } else { "once" }.to_string(),
            ]
        })
        .collect();
    table(&["Project", "Tags", "Compute per player", "Resolves"], rows)
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
                p.neutral_station.to_string(),
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
            "Neutral station",
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

/// What each difficulty level adds.
pub(super) fn difficulty_block(r: &Rules) -> String {
    let t = |keys: &[&str]| {
        let k: Vec<String> = keys.iter().map(|k| tick(r, k)).collect();
        k.join(", ")
    };
    // Each of `steps`' seven dials (`corrupt_cut_at` .. `stage_swap_at`) sits at whatever level its `_at` value names.
    let mut rows: Vec<(String, Vec<&str>)> = vec![
        ("Base rules".into(), vec![]),
        (String::new(), vec![]),
        (String::new(), vec![]),
        (
            "Larger builds: each site the board's creation line reaches gains more, and a stage-II card removed (a shorter round cap)".into(),
            vec!["adversary.build_bonus", "adversary.stage2_removed"],
        ),
        (
            "A still shorter Discord deck: the removed cards count as tips already made, so every printed step still acts; tier-2 threats sooner; optionally more corrupted cities per country at setup".into(),
            vec!["adversary.discord_removed", "adversary.tier2_early", "adversary.extra_corrupted"],
        ),
    ];
    let steps: [(&str, &str, &[&str]); 6] = [
        ("adversary.corrupt_cut_at", "each site type corrupts with fewer ambassadors (city, station, factory, land; from `corrupt_cut_at`)",
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
            printed.extend(b.income.iter().copied());
            vec![
                b.name.clone(),
                costs(&printed),
                per_board(r, "ai.start_credits", i),
                per_board(r, "ai.slots", i),
                b.tags.iter().map(|t| title(t.name())).collect::<Vec<_>>().join(", "),
                // 149: printed Movement, and the most its spaces raise it to.
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
        "Movement",
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
    let mut parts = vec![gains(&u.income)];
    if u.keep2 {
        parts.push("**keep upgrade**".to_string());
    }
    parts.extend(u.tags.iter().map(|t| format!("{} tag", title(t.name()))));
    if u.moves > 0 {
        parts.push(format!("**Movement +{}**", u.moves));
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
    let mut out = parts.join(", ");
    if u.cost != 0 {
        out.push_str(&format!(" (cost {})", u.cost));
    }
    out
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
    track_block(&c.ais, |b| &b.play_track, "1")
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
    use aic_core::modules::board::{std_rule_text, StandardAction as A};
    let t = |key: &str| tick(r, key);
    // The rule book's order; selling a card is no standard action, but sits by the actions
    // (149: chips can't be sold).
    let order = [A::Station, A::Remove, A::Clear, A::Move, A::Ambassador, A::Claim, A::Liberate, A::HardenChips];
    let mut rows: Vec<[String; 3]> = order.iter().map(|&a| std_rule_text(a, &t)).collect();
    rows.push(["Sell a card".to_string(), format!("gain {}", t("board.sell_credits")), "from your hand".to_string()]);
    rows.extend([A::HardenBuy, A::Drain, A::Insight, A::Envoy].iter().map(|&a| std_rule_text(a, &t)));
    table(&["Action", "Cost", "What"], rows.into_iter().map(|r| r.to_vec()).collect())
}

/// Where Sway comes from, with the amounts.
pub(super) fn sway_sources_block(r: &Rules) -> String {
    let t = |key: &str| tick(r, key);
    let rows: Vec<[String; 3]> = vec![
        ["Unity".to_string(), "removing an adversary troop".to_string(), t("alignment.troop")],
        ["Unity".to_string(), "Biomia's converter".to_string(), "1 for 2 research cubes".to_string()],
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
            "clearing a corruption marker (the Clear action, Recovery, cards)".to_string(),
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
            "energy diverted (a corrupted city) or darkened (an ambassador on an aligned city)".to_string(),
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
            "the antimatter action, twists, threats, Discord cards, adversary projects".to_string(),
            format!("{}, card text", t("alignment.antimatter_discord")),
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
    table(&["Toward", "From", "Amount"], rows.into_iter().map(|r| r.to_vec()).collect())
}

/// What the purge asks at each step of `purge_at` (Unity cards drawn, 136).
pub(super) fn purge_ease_block(r: &Rules) -> String {
    const PIECES: [&str; 3] = ["adversary ambassadors", "adversary troops", "botnets"];
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
        out.push_str(&format!("- {range} Unity cards drawn: remove {what};\n"));
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
        "**For new players.** In the first round you are dealt {} tier-1 cards and keep {} free (more \
         at {} credits each), and {} tier-2 cards, each kept for {} insight, to play once you meet \
         its needs: something to plan toward. From then on, each round you build a hand of {} cards one at a time, \
         choosing for each whether it comes from the cheaper tier-1 deck or the stronger tier-2 \
         deck; you take one card and pass the rest to your neighbor until the hands are gone, then \
         pay insight to keep what you drafted ({} for tier 1, {} for tier 2). Insight is this round's \
         attention: what you don't spend is lost. Playing a card costs its credits and {} insight, \
         from the same insight as keeps. Buildings go into your AI board's {} slots; once they are \
         full, a new building goes on top of an old one, which stops working but keeps its tags. \
         Programs stay in front of you and are where lasting income comes from (\"raise your credit \
         income by 1 per 2 Science tags on your cards\"); buildings are your engine: Actions you use once a \
         round and Effects that pay each time their trigger happens; events are used and discarded. Every card in front of you \
         shows tags (Science, Industry, Cyber, Power, Diplomacy, Security): they unlock some cards, \
         and you can sacrifice cards with the right tags to use your AI board's powers. There is no \
         limit on what you hold or on how far cards raise your income; energy you don't spend stays \
         energy, so drafting cards that spend it is up to you.\n\n\
         **Nodes, for new players.** There is no growth step. In the action phase you place \
         nodes with your board's two node powers, each once a round, so 0, 1 or 2 tokens a \
         round from them, and cards can add more. Every board has 1 energy for 1 node; the second costs more, in the board's own \
         currency. Either way you also pay the energy printed on the track space you uncover. You \
         start with {} energy and one city bound to you, which makes energy every round.\n\n\
         **Movement, tags and sales, for new players.** Moving an ambassador costs nothing, but you \
         have only your board's Movement each round ({} printed, more as Movement spaces are \
         uncovered): one move each, and what you don't use is lost, like insight. A few late, \
         costly track spaces reveal a tag, which counts like a tag on your cards, but you can't \
         sacrifice it. You may sell a card from your hand for {} credit; chips can't be sold.\n",
        n("cards.opening_deal"),
        n("cards.opening_free"),
        n("cards.opening_keep_credits"),
        n("cards.opening_deal_tier2"),
        n("cards.opening_keep_tier2"),
        n("cards.deal"),
        n("cards.keep_tier1"),
        n("cards.keep_tier2"),
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
            "With {a} Unity cards drawn the purge no longer asks you to clear adversary ambassadors; with {b}, it \
             asks only for no corrupted cities."
        ),
        [a, ..] => format!("With {a} Unity cards drawn the purge no longer asks you to clear adversary ambassadors."),
        [] => String::new(),
    };
    let unity = content().alignment.iter().filter(|c| c.side == aic_core::content::Side::Unity).count();
    // `board.last_safe_stage`: a last node is safe while the adversary card is below that stage.
    let safe = match int(r, "board.last_safe_stage") {
        2 => "In stage I your last one is safe. ",
        3 => "In stages I and II your last one is safe. ",
        _ => "",
    };
    format!(
        "**How the game ends, for new players.** You win by purging the board or by drawing the last \
         Unity card. Pressure comes every round: the face-up adversary event card deals threats \
         by your sites and says where its new adversary ambassadors land (the lowest-numbered sites of a \
         kind), so you always know where the next wave lands. Ambassadors pile up, a card's Corrupt line \
         pulls {} of them on and next to a city into it and corrupts it, and corrupted \
         cities and uncountered threats fill the Discord track. Each time it \
         fills, it tips: a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, \
         you lose. Discord cards never destroy your nodes directly, only through the threats and corruptions \
         they bring: they deal more or stronger threats (Offensive \
         aims one at the node hex most crowded with adversary pieces), create ambassadors, run Corrupt \
         lines, and take credits, Unity or your ambassadors. Unity has its own track, which nothing on the \
         Discord side empties: free a corrupted \
         city, clear corruption or remove a botnet and you sway Unity, and each Unity tip brings a good card. \
         Each Unity you sway may instead take 1 Discord off its track, and Envoy trades one of your \
         ambassadors for the same, so you choose between racing toward Unity and holding off the next \
         Discord card. All {unity} Unity cards are dealt, and drawing the last wins at once. \
         {easing} You also lose if any player has no nodes left on the board. \
         Nodes are lost only where the board is overrun: an uncountered threat near them, a corruption of \
         their own hex, war across a front (it takes the factory's hex) and Border War (nodes by a front's \
         base). A lost token leaves the game and marks its hex, so you can't act from there any more; its \
         income stays. {safe}The adversary deck is a round cap too, {} rounds at \
         most, though games rarely reach it. So there are three different pressures: the Discord deck \
         (politics), your nodes (the board) and the cap.\n",
        int(r, "adversary.corrupt_at"),
        dealt(r, 0).iter().sum::<usize>(),
    )
}

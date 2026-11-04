//! The blocks of `docs/rules.md`, each a pure function of `Rules` or `Content`,
//! and the house terms.

use aic_core::config::ParamValue;
use aic_core::content::{content, AiBoard, Content, Cost, Megaproject, Resource, Uncover};
use aic_core::modules::adversary;
use aic_core::rules::Tunable;
use aic_core::Rules;

use super::table;
use crate::cli::fail;

/// The words cards lean on, once: rules.md's *Terms* and the player aid print these.
pub const HOUSE_TERMS: [(&str, &str); 13] = [
    ("Facility", "a research station or factory on the board. A building is always a card. Harden goes on a facility and nothing else"),
    ("Shield", "a hardening token on a hex with your node"),
    ("Ambassador", "yours; the other side's is always an adversary ambassador"),
    ("Troop", "always the adversary's"),
    ("In reach", "on a hex within 1 of your nodes"),
    ("A country you're in", "a country where you have a node"),
    ("Your home country", "the country you start in"),
    ("A threat", "on your card: one you may counter, with one of its sources in reach or your ambassador on one"),
    ("Here", "on a building: on this card, where its tokens sit"),
    ("Remove N", "as many as there are, up to N"),
    ("Draw", "from the tier-1 deck, with no keep to pay"),
    ("On your cards", "on tags: on your cards in play, covered buildings included. Tokens count only on the card that holds them"),
    ("Action", "once a round, always; \"A, OR B\" is one use, either half"),
];

/// `3 credits, 1 energy`.
fn costs(c: &[Cost]) -> String {
    let parts: Vec<String> = c.iter().map(|(r, n)| r.words(*n)).collect();
    parts.join(", ")
}

pub(super) fn house_terms() -> String {
    HOUSE_TERMS.iter().map(|(term, says)| format!("- **{term}**: {says}.\n")).collect()
}

pub(super) fn threats_block() -> String {
    let rows = content()
        .threats
        .iter()
        .map(|t| vec![t.name.clone(), t.tier.to_string(), t.source.to_string(), t.text.clone(), costs(&t.counter)])
        .collect();
    table(&["Threat", "Tier", "Source", "If not countered", "Counter"], rows)
}

pub(super) fn adversary_block() -> String {
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
                if m.after.is_empty() { "lost".to_string() } else { Megaproject::words(&m.after) },
            ]
        })
        .collect();
    table(
        &[
            "Corp",
            "Megaproject",
            "Cubes",
            "At completion: player cube / adversary cube",
            "Converter (once a round)",
            "Each adversary cube after",
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
            "A still shorter Discord deck: the removed cards count as tips already made, so every printed step still acts; tier-2 threats sooner; optionally more corrupted cities per country at setup",
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
            rows.push(vec![b.name.clone(), p.name.clone(), cost.join(" + "), p.text.clone()]);
        }
    }
    table(&["Board", "Power", "Cost", "Effect"], rows)
}

/// The standard actions and projects, with their prices.
pub(super) fn standard_actions_block(r: &Rules) -> String {
    let t = |key: &str| tick(r, key);
    let rows: Vec<[String; 3]> = vec![
        [
            "Build a research station".to_string(),
            format!("{} chips", t("board.station_chips")),
            "on an open hex in reach that is not an industrial zone; it is yours (your cube)".to_string(),
        ],
        [
            "Remove adversary ambassadors".to_string(),
            format!("{} of your ambassadors, from your supply", t("board.remove_ambassadors")),
            format!("it goes onto one hex in reach, removes up to {} there and stays", t("board.remove_count")),
        ],
        [
            "Clear a corruption marker".to_string(),
            format!("{} credits", t("board.clear_cost")),
            format!(
                "in reach: sway {} toward Unity, gain {} credits",
                t("board.clear_unity"),
                t("board.clear_credits")
            ),
        ],
        [
            "Move a player ambassador".to_string(),
            format!("{} credit", t("board.move_credits")),
            "from your supply or the board onto a piece in reach (not the hex it comes from)".to_string(),
        ],
        [
            "Gain a player ambassador".to_string(),
            t("board.ambassador_credits"),
            "into your supply; a standard project too".to_string(),
        ],
        [
            "Align a clean city".to_string(),
            "1 ambassador".to_string(),
            "in reach, with a site claim: your cube on it".to_string(),
        ],
        [
            "Harden a facility (a card grants it: Hardened Bunker Complex)".to_string(),
            "the chips the card prints a token".to_string(),
            "a player-owned station or factory in reach that is not corrupted, one a troop can strike".to_string(),
        ],
        [
            "Sell chips".to_string(),
            "1 chip".to_string(),
            format!("gain {}", t("board.chips_credits")),
        ],
        [
            "Sell a card".to_string(),
            format!("gain {}", t("board.sell_credits")),
            "from your hand".to_string(),
        ],
        [
            "Project: harden (`harden_buy`)".to_string(),
            format!("{} credits", t("board.harden_credits")),
            "any player: a hardening token on a player-owned station or factory in reach".to_string(),
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
pub(super) fn sway_sources_block(r: &Rules) -> String {
    let t = |key: &str| tick(r, key);
    let rows: Vec<[String; 3]> = vec![
        [
            "Unity".to_string(),
            "a player ambassador on a producing base / factory".to_string(),
            format!("{} / {}", t("board.base_unity"), t("board.factory_unity")),
        ],
        ["Unity".to_string(), "removing an adversary troop".to_string(), t("alignment.troop")],
        ["Unity".to_string(), "Biomia's converter".to_string(), "1 for 2 research cubes".to_string()],
        [
            "Unity".to_string(),
            "liberating a corrupted city (§2: a liberate card or the fallback)".to_string(),
            t("alignment.free_city"),
        ],
        ["Unity".to_string(), "removing a botnet".to_string(), t("alignment.botnet")],
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
            "a facility destroyed in war (not by an adversary troop)".to_string(),
            t("board.destruction_discord"),
        ],
        [
            "Discord".to_string(),
            "adversary cubes on a megaproject as it completes, as its corp's card prints".to_string(),
            "1 each".to_string(),
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
pub(super) fn purge_ease_block(r: &Rules) -> String {
    const STEPS: [(&str, &str); 3] = [
        ("first", "adversary ambassadors stop counting"),
        ("second", "adversary troops stop counting"),
        ("third", "**botnets** stop counting (candidate: with botnet removal rare, Unity buys the win)"),
    ];
    let mut out = String::new();
    for (tips, (which, what)) in list(r, "alignment.unity_ease").iter().zip(STEPS) {
        out.push_str(&format!("- {which} threshold ({tips} Unity tips): {what};\n"));
    }
    out.push_str("- corrupted cities always count.\n");
    out
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
        "**For new players.** In the first round you are dealt {} cards and keep {} free (more \
         at {} credits each). From then on, each round you build a hand of {} cards one at a time, \
         choosing for each whether it comes from the cheaper tier-1 deck or the stronger tier-2 \
         deck; you take one card and pass the rest to your neighbor until the hands are gone, then \
         pay insight to keep what you drafted ({} for tier 1, {} for tier 2). Insight is this round's \
         attention: what you don't spend is lost. Playing a card costs its credits and {} insight, \
         from the same insight as keeps. Buildings go into your AI board's {} slots; once they are \
         full, a new building goes on top of an old one, which stops working but keeps its tags. \
         Programs stay in front of you; events are used and discarded. Every card in front of you \
         shows tags (Science, Industry, Cyber, Power, Diplomacy, Security): they unlock some cards, \
         and you can sacrifice cards with the right tags to use your AI board's powers.\n\n\
         **Nodes, for new players.** There is no growth step. In the action phase you place \
         nodes with your board's two node powers, each once a round, so 0, 1 or 2 tokens a \
         round from them, and cards can add more. Every board has 1 energy for 1 node; the second costs more, in the board's own \
         currency. Either way you also pay the energy printed on the track space you uncover. You \
         start with {} energy and one city bound to you, which makes energy every round.\n",
        n("cards.opening_deal"),
        n("cards.opening_free"),
        n("cards.opening_keep_credits"),
        n("cards.deal"),
        n("cards.keep_tier1"),
        n("cards.keep_tier2"),
        range(&insight),
        range(&slots),
        n("ai.start_energy"),
    )
}

/// How the game ends, for new players.
pub(super) fn game_end_block(r: &Rules) -> String {
    let ease = list(r, "alignment.unity_ease");
    let easing = match ease.as_slice() {
        [_, a, b, ..] => format!(
            "{a} Unity tips and the purge asks only for no botnets and no corrupted cities; {b} and it \
             asks only for no corrupted cities."
        ),
        [.., a] => format!("{a} Unity tips and the purge asks only for no botnets and no corrupted cities."),
        [] => String::new(),
    };
    format!(
        "**How the game ends, for new players.** You win by purging the board or by drawing the last \
         Unity card. Pressure comes every round: the face-up adversary card names a kind of site, \
         and every such site gains an adversary ambassador, so you always know where the next wave \
         lands. Ambassadors pile up, pairs become botnets, {} on a city corrupt it, and corrupted \
         cities and uncountered threats push the Alignment track toward Discord. Each time the track \
         tips, a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, \
         you lose. Pushing back works the same way in your favor. Mass your ambassadors on a \
         corrupted city to free it, which sways toward Unity, and each Unity tip brings a good card. \
         The Unity deck is small too: a game deals only {}, and drawing the last wins at once. \
         {easing} You also lose if any player has no nodes left on the board. \
         Threats, war, corruption and late adversary and Discord cards destroy it, a token at a time, and \
         a lost token leaves the game and marks its hex, so you can't act from there any more; its income \
         stays. In stage I your last one is safe. The adversary deck is a round cap too, {} rounds at most, \
         though games rarely reach it. So there are three different pressures: the Discord deck (politics), \
         your nodes (the board) and the cap.\n",
        int(r, "adversary.corrupt_at"),
        list(r, "alignment.unity_cards").iter().sum::<i64>(),
        dealt(r, 0).iter().sum::<usize>(),
    )
}

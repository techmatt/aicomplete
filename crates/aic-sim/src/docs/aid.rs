//! 268: `docs/player-aid.md`, the table's reference sheet: the round, what is once a round, the tip strips,
//! the troops (r141: garrison, guard, Strike, Redeploy), the tracks and markers (274), what spends an ambassador and city growth, then the house terms. Every number comes from the rules or content,
//! so the sheet moves with them; the words are the rulebook's, shortened (it stays the rules' one full statement).

use aic_core::content::content;
use aic_core::vocab::Side;
use aic_core::Rules;

use super::blocks::{house_terms, int, list};

const ROUND: &str = "\
| | Step | Do |
|---|---|---|
| | Round start | Set your insight. Clear your used markers (below). |
| A | Sway | Each Fallout: 1 Discord. |
| B | Adversary | **Card** (threats; Escalate: each site of last round's type holding an agent takes 1 more; the board's rule; Infiltrate, Strike, Redeploy, other lines; each arrival checks only its own hex, once all its agents there are on), **Income** (next card up; each corrupted site's one output: a city 1 Discord, a station 1 research cube, a factory its agent pushed along its arrow then 1 new agent, a botnet 1 compute; then all compute onto the leftmost project at once; the research track advances a slot for every K cubes; firewalls, cordons and no-X tokens off). |
| C | Cards | Build a hand of 4, draft, keep (1 insight a tier-1; 3 a tier-2, 2 with the keep upgrade, less 1 per 2 Science tags you have). |
| D | Actions | All at once, until everyone passes. |
| E | Production | Read from the tracks, less each blocked chip's site: factories, cities (1 energy to every player with a node on or next to one), stations, megaprojects, income. |
| F | Threats | Each threat not paid off takes effect (and lands 1 agent on its first token), one that waited from last round included. |
| G | Cleanup | This round's effects end (Shelter, round flags). |

After every action and step: Unity just gained goes on its track; tip any full track, Discord first;
then check the losses, then the win (no city corrupted).
";

/// The once-a-round list: what a player must otherwise remember with nothing on the table to show it.
fn once_a_round(r: &Rules) -> String {
    // 276: a group (Growth's options, a power's tiers) is one use and one marker: named once.
    let uses = |b: &aic_core::content::AiBoard| {
        let mut names: Vec<&str> = Vec::new();
        for p in &b.powers {
            let n = p.use_key();
            if !names.contains(&n) {
                names.push(n);
            }
        }
        names.join(", ")
    };
    let boards: Vec<String> = content().ais.iter().map(|b| format!("{} ({})", b.name, uses(b))).collect();
    // 0 is "no limit" for these two (standard.rs).
    let times = |n: i64| match n {
        0 => "any number of times".to_string(),
        1 => "once".to_string(),
        n => format!("{n} times"),
    };
    format!(
        "Each of these is used at most once a round. **Put a used marker on its used space when you use it** (each \
         power, building slot and converter prints one), and clear them all at round \
         start.\n\n\
         - **Your board's Growth and powers:** each once: one Growth option, one tier of each power. {}.\n\
         - **Each building's Action** (`→`): once; \"A, OR B\" is one use.\n\
         - **Standard actions:** Remove a troop {} a round, Turn a cube into insight {} a round, each player.\n\
         - **Each megacorp's converter and chip converter:** once a round each, each player.\n\
         - **Vigil** (Protector): only the first {} troops a round pay.\n\
         - **Aegis Doctrine** (Protector capstone): the first save a round.\n\
         - **A research slot's icon** happens once, as the marker arrives.\n\n\
         **Lasting effects, with their token:** a firewall, a cordon and a no-Infiltrate, no-Strike or no-Corrupt token \
         come off in the next adversary turn's Income beat; Shelter ends at cleanup; a round-flag marker runs through \
         the next round. Insight resets at \
         round start (Patience keeps it); Dispatches refresh.\n",
        boards.join("; "),
        times(int(r, "board.troop_per_round")),
        times(int(r, "board.insight_per_round")),
        int(r, "board.vigil_cap"),
    )
}

/// One tip strip a side: each tip's card strength, and the Discord cards left (r141: the purge's steps are gone,
/// so Unity's strip shows no "what the win asks" column).
fn tip_strips(r: &Rules) -> String {
    let tier = list(r, "alignment.tier_at");
    let discord_deck = content().alignment.iter().filter(|c| c.side == Side::Discord).count() as i64;
    let last = discord_deck + 1;
    let strength = |t: i64| {
        if t < tier[0] {
            "I"
        } else if t < tier[1] {
            "II"
        } else {
            "III"
        }
    };
    let mut out = "Keep one tip-count marker a side on a strip like this, and read off its row the strength of the \
                   card drawn at that tip and the Discord cards left. You win the moment no city is corrupted, \
                   at any tip count.\n\n| Tip count | Card drawn at this tip | Discord cards left |\n|---|---|---|\n"
        .to_string();
    for t in 0..=last {
        let count = if t == last { format!("{t}+") } else { t.to_string() };
        let card = if t == 0 { "—".to_string() } else { format!("strength {}", strength(t)) };
        let left = match t {
            0 => discord_deck.to_string(),
            t if t <= discord_deck => (discord_deck - t).to_string(),
            _ => "**you lose**".to_string(),
        };
        out += &format!("| {count} | {card} | {left} |\n");
    }
    let (u, d) = (int(r, "alignment.tip_per_player"), int(r, "alignment.discord_tip_per_player"));
    let at = |k: i64| (1..=4).map(|n| (k * n).to_string()).collect::<Vec<_>>().join(" | ");
    out += &format!(
        "\nA track tips when it holds this much:\n\n| Players | 1 | 2 | 3 | 4 |\n|---|---|---|---|---|\n\
         | Unity | {} |\n| Discord | {} |\n",
        at(u),
        at(d)
    );
    out
}

/// Corrupt numbers at each level, the printed ones with the level's cuts applied, consecutive equal levels merged:
/// the number a player uses, never the map's printed one and a modifier.
fn corrupt_numbers(r: &Rules) -> String {
    let base = [
        int(r, "adversary.corrupt_at"),
        int(r, "adversary.corrupt_station_at"),
        int(r, "adversary.corrupt_factory_at"),
    ];
    let (cut, cut_at, city_at) =
        (list(r, "adversary.corrupt_cut"), int(r, "adversary.corrupt_cut_at"), int(r, "adversary.city_cut_at"));
    let node = int(r, "adversary.corrupt_node_at");
    let at = |level: i64| -> Vec<i64> {
        let mut v: Vec<i64> = base
            .iter()
            .enumerate()
            .map(|(i, &n)| n - if level >= cut_at { cut.get(i).copied().unwrap_or(0) } else { 0 })
            .collect();
        if level >= city_at {
            v[0] -= 1;
        }
        v.push(node);
        v
    };
    let mut rows: Vec<(i64, i64, Vec<i64>)> = Vec::new();
    for level in 0..=4 {
        let v = at(level);
        match rows.last_mut() {
            Some(last) if last.2 == v => last.1 = level,
            _ => rows.push((level, level, v)),
        }
    }
    let mut out = "| Level | City | Station | Factory | Your node |\n|---|---|---|---|---|\n".to_string();
    for (lo, hi, v) in rows {
        let name = if lo == hi { lo.to_string() } else { format!("{lo}-{hi}") };
        out += &format!("| {name} | {} |\n", v.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" | "));
    }
    out + "\nChecked only when the adversary puts agents on a hex (an arrival), and only that hex: a site holding its \
           number corrupts; else a hex with your node and that many loses the node. Each placement (a line, \
           an Escalate, a factory's push, a threat's or card's agents) puts all its agents for a hex on, then checks that \
           hex once. A Push or Gather is no arrival.\n"
}

const TROOPS: &str = "\
Troops don't move or act on their own. They come late: from the research row, some projects and Discord cards.

1. **Garrison.** A new troop goes on its country's lowest-numbered corrupted factory; with none, its \
lowest-numbered corrupted city; with none, its lowest-numbered city (never the Protector's node hex: it stays in the \
supply). It stays there.
2. **Guard.** A corrupted site with a troop on or next to it can't be liberated. Remove the troop or push it away \
first.
3. **Strike** (an icon on the face-up event card, a round ahead, or a research slot): each troop, country by \
country in hex order, destroys one player node, or player's station or factory, on or next to it (the players \
choose; never anything on the Protector's node hex): destroyed unless protected (§13). The troop stays. Vigil: 1 \
insight for each troop on or next to the Protector's nodes.
4. **Redeploy** (an icon, rare): each troop not on or next to a corrupted city is picked up and put on the nearest \
corrupted city by land (ties: the lower country, then hex number).
";

/// 274 (table time): every running number the table keeps sits on a track or a marker, read, never re-added.
const TRACKS: &str = "\
- **Your income tracks** (your board): one marker a resource. Move it when a track space, program or building \
raises an income, or (your sites' output) when you gain or lose a station or factory or its megaproject \
completes; in production, collect what \
the markers show, less each blocked site's output (until its megaproject completes, a station's cube goes onto it).
- **Blocked chips:** on a producing site while an opposing piece stands there (an agent on yours or a neutral \
station, your ambassador on the adversary's) and on a city a blackout darkens; lift \
it when the last one leaves.
- **Your tag dials** (your board): one a tag. Turn it when a card with that tag comes into play or is sacrificed.
- **Used markers:** on each power, building slot and converter's used space; a bought chip on a city (above).
- **Unity and Discord tip tracks:** one space a tip; each space prints its card strength.
- **The adversary board:** the compute income track, the research row (six icon slots, a marker, and the cubes \
toward its next slot), the last-round space (the card whose sites escalate), a compute track beside each \
project slot (a marker, not cubes), and its corrupted sites' outputs.
- **Zone arrows** (the map): each industrial zone prints an arrow to its city; a corrupted factory there pushes \
its agent along it.
- **Waits token:** on a threat dealt after the counter phase or delayed; it takes effect next round.
";

fn ambassadors(r: &Rules) -> String {
    format!(
        "- **An agent meets one** (it lands, or your ambassador arrives): one for one, both off.\n\
         - **Corruption** removes every ambassador on the hex.\n\
         - **Liberate:** city {}, station {}, factory {} (yours on the site pay first, then your supply).\n\
         - **Remove agents:** {} from your supply.\n\
         - **Protecting** your node or facility on its hex (§13): one, instead of the piece.\n\
         - **Threats** that print ambassadors as a cost: from your supply.\n\
         - **Blocking** spends none: an ambassador on a corrupted site stops its output while it stands. A \
         **Dispatch** moves one; a troop never touches one.\n\
         - **Gain one:** {} credits, Accord, cards.\n",
        int(r, "board.liberate_at"),
        int(r, "board.claim_station"),
        int(r, "board.claim_factory"),
        int(r, "board.remove_ambassadors"),
        int(r, "board.ambassador_credits"),
    )
}

/// 293: the adversary's outputs and research row (they took the Grow section's place).
fn outputs(r: &Rules) -> String {
    let b = &content().adversary_board;
    let k = list(r, "adversary.research_at").first().copied().unwrap_or(0);
    let every = if k == 1 { "each cube a country".to_string() } else { format!("every {k} cubes a country") };
    let row = b.research.as_ref().map_or(String::new(), |row| {
        let slots: Vec<String> = row
            .slots
            .iter()
            .enumerate()
            .map(|(i, id)| {
                let icon = b.icons.iter().find(|x| &x.id == id).expect("a row's icon");
                format!("{}. **{}**: {}", i + 1, icon.name, icon.text)
            })
            .collect();
        format!(
            "\n**{}** ({every}: one slot; its icon happens once, in each country):\n\n{}\n7. **The players lose.**\n",
            row.name,
            slots.join("\n")
        )
    });
    format!(
        "Each corrupted site makes one thing in the Income beat (a player ambassador on it stops it):\n\n\
         | Site | Output |\n|---|---|\n\
         | City | {} Discord |\n\
         | Research station | {} research cube onto the adversary's track |\n\
         | Factory | its agent moves along its zone's arrow onto its city (not onto one already the adversary's); \
         then, holding none, it makes 1 agent |\n\
         | Botnet | {} compute |\n{row}",
        int(r, "board.divert_discord"),
        int(r, "board.station_cubes"),
        int(r, "adversary.botnet_compute"),
    )
}

pub(super) fn player_aid_md(r: &Rules) -> String {
    format!(
        "# Player aid\n\nGenerated by `aic-sim docs` from `crates/aic-sim/src/docs/aid.rs` (its numbers from the \
         rules) and `HOUSE_TERMS` in `crates/aic-sim/src/docs/blocks.rs`: edit them there. The table's reference \
         sheet; the full rules are [rulebook.md](rulebook.md), the marks [card-guidelines.md](card-guidelines.md).\n\n\
         ## The round (rulebook §4)\n\n{ROUND}\n## Once a round (§5, §8, §20)\n\n{}\n\
         ## Tip strips (§12, §14)\n\n{}\n## Corrupt numbers (§6, §21)\n\n{}\n## Troops\n\n{TROOPS}\n\
         ## Tracks and markers (§2)\n\n{TRACKS}\n## What spends an ambassador (§8)\n\n{}\n\
         ## The adversary's outputs and research row\n\n{}\n## Words on cards\n\n{}",
        once_a_round(r),
        tip_strips(r),
        corrupt_numbers(r),
        ambassadors(r),
        outputs(r),
        house_terms()
    )
}

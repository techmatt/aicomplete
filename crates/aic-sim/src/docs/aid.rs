//! 268: `docs/player-aid.md`, the table's reference sheet: the round, what is once a round, the tip strips,
//! the march, the tracks and markers (274), what spends an ambassador and city growth, then the house terms. Every number comes from the rules or content,
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
| B | Adversary | **Card** (threats, Infiltrate, Muster, other lines, Spill, Grow; each arrival checks only its own hex), **March** (each stack 1 step), **Income** (next card up; each botnet seeds an agent; compute onto the leftmost project; research track; firewalls, cordons and no-X tokens off). |
| C | Cards | Build a hand of 4, draft, keep (1 insight a tier-1, 3 a tier-2, 2 with the keep upgrade). |
| D | Actions | All at once, until everyone passes. |
| E | Production | Factories, cities, stations, megaprojects, income; then extra drones. |
| F | Threats | Each threat not paid off takes effect (and lands 1 agent on its first token), one that waited from last round included. |
| G | War | Each front fights. |
| H | Cleanup | This round's effects end (Shelter, round flags). |

After every action and step: Unity just gained goes on its track or against Discord; tip any full track, Discord first;
then check the losses, then the purge.
";

/// The once-a-round list: what a player must otherwise remember with nothing on the table to show it.
fn once_a_round(r: &Rules) -> String {
    // 276: a group (Growth's options, a power's tiers) is one use and one marker: named once.
    let uses = |b: &aic_core::content::AiBoard| {
        let mut names: Vec<&str> = Vec::new();
        for p in &b.powers {
            let n = p.group.as_deref().unwrap_or(&p.name);
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
         power, building slot and converter prints one; a city takes a bought chip), and clear them all at round \
         start.\n\n\
         - **Your board's Growth and powers:** each once: one Growth option, one tier of each power. {}.\n\
         - **Each building's Action** (`→`): once; \"A, OR B\" is one use.\n\
         - **Standard actions:** Remove a troop {} a round, Turn a cube into insight {} a round, each player; **Buy \
         energy** once a round a city, for all players together.\n\
         - **Each megacorp's converter and chip converter:** once a round each, each player.\n\
         - **Vigil** (Protector): only the first {} troops a round pay.\n\
         - **Aegis Doctrine** (Protector capstone): the first save a round.\n\
         - **The adversary's research rule** fires at most once a round.\n\n\
         **Lasting effects, with their token:** a firewall, a cordon and a no-Infiltrate, no-Muster or no-Corrupt token \
         come off in the next adversary turn's Income beat; Shelter ends at cleanup; a round-flag marker runs through \
         the next round. Insight resets at \
         round start (Patience keeps it); Dispatches refresh.\n",
        boards.join("; "),
        times(int(r, "board.troop_per_round")),
        times(int(r, "board.insight_per_round")),
        int(r, "board.vigil_cap"),
    )
}

/// One tip strip a side: each tip's card strength, and on Unity's what the purge still asks for. Merges the
/// rulebook's two tables (§12's strengths, §14's purge) into one line read off one count.
fn tip_strips(r: &Rules) -> String {
    let tier = list(r, "alignment.tier_at");
    let purge = list(r, "alignment.purge_at");
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
    let asks = |t: i64| {
        let mut s = vec!["cities"];
        if t < purge[2] {
            s.push("botnets");
        }
        if t < purge[1] {
            s.push("troops");
        }
        if t < purge[0] {
            s.push("agents");
        }
        s.join(", ")
    };
    let mut out = "Keep one tip-count marker a side on a strip like this, and read everything off its row: the \
                   strength of the card drawn at that tip, what the purge still needs gone (Unity), and the Discord \
                   cards left (Discord).\n\n| Tip count | Card drawn at this tip | Unity: you win when the map holds \
                   none of | Discord cards left |\n|---|---|---|---|\n"
        .to_string();
    for t in 0..=last {
        let count = if t == last { format!("{t}+") } else { t.to_string() };
        let card = if t == 0 { "—".to_string() } else { format!("strength {}", strength(t)) };
        let left = match t {
            0 => discord_deck.to_string(),
            t if t <= discord_deck => (discord_deck - t).to_string(),
            _ => "**you lose**".to_string(),
        };
        out += &format!("| {count} | {card} | {} | {left} |\n", asks(t));
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
           number corrupts; else a hex with your node and that many loses the node. A Push or Gather is no arrival.\n"
}

const MARCH: &str = "\
The troops on each hex march as one stack, stack by stack (country by country, hex order), 1 step; troops on a hex \
a stack enters march later, on their own:

1. **Targets on or next to it?** (players' nodes and players' stations and factories; never anything on the \
Protector's node hex.) One troop strikes each, while troops last (the players choose): destroyed unless protected \
(§13). A troop that strikes leaves the map either way.
2. **The rest** move together 1 hex toward the nearest target by land, never onto the Protector's node hex; ties: \
the players choose, once for the stack. No way to any target: they stay.
3. Troops ignore ambassadors. Vigil: a troop ended its march (moved or not) next to the Protector's node.
";

/// 274 (table time): every running number the table keeps sits on a track or a marker, read, never re-added.
const TRACKS: &str = "\
- **Your income tracks** (your board): one marker a resource. Move it when a track space, program or building \
raises an income; in production, collect what the markers show.
- **Your tag dials** (your board): one a tag. Turn it when a card with that tag comes into play or is sacrificed.
- **Used markers:** on each power, building slot and converter's used space; a bought chip on a city (above).
- **Unity and Discord tip tracks:** one space a tip; each space prints its card strength, and Unity's the purge's.
- **The adversary board:** the compute income track, the research track, and a compute track beside each \
project slot (a marker, not cubes).
- **Waits token:** on a threat dealt after the counter phase or delayed; it takes effect next round.
";

fn ambassadors(r: &Rules) -> String {
    format!(
        "- **An agent meets one** (it lands, or your ambassador arrives): one for one, both off.\n\
         - **Corruption** removes every ambassador on the hex.\n\
         - **Liberate:** city {}, station {}, factory {} (yours on the site pay first, then your supply).\n\
         - **Remove agents:** {} from your supply. **Envoy:** {}, for 1 Discord off.\n\
         - **A drone** landing on a base where one stands: one a drone.\n\
         - **Protecting** your node or facility on its hex (§13): one, instead of the piece.\n\
         - **Threats** that print ambassadors as a cost: from your supply.\n\
         - **Blocking** spends none: an ambassador on a corrupted site stops its output while it stands. A \
         **Dispatch** moves one; a troop never touches one.\n\
         - **Gain one:** {} credits, Accord, cards.\n",
        int(r, "board.liberate_at"),
        int(r, "board.claim_station"),
        int(r, "board.claim_factory"),
        int(r, "board.remove_ambassadors"),
        int(r, "board.envoy_ambassadors"),
        int(r, "board.ambassador_credits"),
    )
}

/// 275: the event card's Grow line (it replaced city growth and its chips).
fn growth(r: &Rules) -> String {
    let cap = int(r, "adversary.grow_cap");
    let infest = int(r, "adversary.grow_infest_at").max(1);
    format!(
        "**Grow: X** (the event card's last line), in each country:\n\n\
         1. **Which:** the X holding the most agents (tie: the lowest number) with a free hex next to it{}.\n\
         2. **What:** a neutral facility of the kind it has **fewer of next to it** (tie: a factory), else the other \
         kind: a factory on a free industrial zone, a station on other open land (no facility, Fallout or base).\n\
         3. **Agent:** if that X holds {} or is the adversary's, put 1 agent on the new facility (an arrival).\n",
        if cap > 0 { format!(" and fewer than {cap} facilities next to it") } else { String::new() },
        if infest == 1 { "an agent".to_string() } else { format!("{infest} agents") },
    )
}

pub(super) fn player_aid_md(r: &Rules) -> String {
    format!(
        "# Player aid\n\nGenerated by `aic-sim docs` from `crates/aic-sim/src/docs/aid.rs` (its numbers from the \
         rules) and `HOUSE_TERMS` in `crates/aic-sim/src/docs/blocks.rs`: edit them there. The table's reference \
         sheet; the full rules are [rulebook.md](rulebook.md), the marks [card-guidelines.md](card-guidelines.md).\n\n\
         ## The round (rulebook §4)\n\n{ROUND}\n## Once a round (§5, §8, §20)\n\n{}\n\
         ## Tip strips (§12, §14)\n\n{}\n## Corrupt numbers (§6, §21)\n\n{}\n## The march (§6)\n\n{MARCH}\n\
         ## Tracks and markers (§2)\n\n{TRACKS}\n## What spends an ambassador (§8)\n\n{}\n\
         ## Grow (§6)\n\n{}\n## Words on cards\n\n{}",
        once_a_round(r),
        tip_strips(r),
        corrupt_numbers(r),
        ambassadors(r),
        growth(r),
        house_terms()
    )
}

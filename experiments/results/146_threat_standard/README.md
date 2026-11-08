# 146: the threat card standard (r61)

Rules revision 61, bumped from the constant: 145's hunt fixes took r60, so the prompt's "r60" is r61 here, and the
baseline it called r59 is r60 (`main` b0615cb; 145's 16-seed smoke played every game as r59). Rules: Matt's threat card
standard (design doc *Adversary system*, 2026-11-07, locked), amendment 6227 (145's D60, D62, D63), message 6254 (the
Discord cards). Every threat, with its give / get: [cards.md](cards.md) (written by [threat_cards.py](threat_cards.py),
which also writes `drafts/adversary/threats_t1.json` and `threats_t2.json`). Scripts: [specs.py](specs.py) and
[levers.json](levers.json) (screens and reads), [read146.py](read146.py) (win, nodes lost, threats by tier, `--vs`
paired, `--harms` each threat's uncountered executions). Checks: [smoke.md](smoke.md).

## The rules

- **No hidden harm.** `threat_nodes` (1 / 2 / 3 nodes of every player near an uncountered threat), its d3 step
  `harm_bonus` and the floor sway `threat_discord` are gone, code and configs.
- **The nearest player.** An uncountered threat's harm hits one seat: the one whose node is closest (hex distance)
  to one of its tokens, wherever that node is (`threats::nearest_seats`; `Ctx::hit`). A tie is a loss choice
  (`choose_loss`) of the lowest tied seat, so agents pick the seat that loses least; ties 0.2-0.8 a game (d0-d3).
  Paying is unchanged (near: a node or ambassador on or next to a token).
- **New op `destroy_node`**: the hit player destroys their node(s) closest to the threat, never the last.
  `each_destroy_node` is a twist's only (Border War).
- **Theft** (`who: nearest`) hits the nearest player as a payment ("pays 5 chips or discards 2 cards"), and the
  adversary gains its compute either way.
- **D62:** every resource harm is `requisition` ("the nearest player pays 3 energy or discards 3 cards"), so holding
  none dodges nothing; no `lose_or` or `players_lose` is left on a threat. `requisition` may print ambassadors.
- **D60:** a station flip (forced or not) and a factory seizure go through corruption's cleanup (`corrupt_hex`: the
  node on it or its armor, the ambassadors). **D63:** a threat's flip takes only its own station token, nothing once
  that one is corrupted; Regime Change is cut.
- **No tier 3.** The 11 tier-3 cards: 4 re-cut as tier 2 (Last Ultimatum, Iron Curtain, War Economy, Cascade Failure:
  ids lose their `t3_` prefix), 7 cut. Event pull lines to the ladder: stage I one tier-1, stage II one tier-2 (Supply
  Lines: tier 1 and tier 2), stage III two tier-2 (`check.py` refuses another shape). **The Discord cards** Incursion,
  Offensive and Two Fronts dealt tier 3: each tier-3 threat is now two tier-2s (Matt, 6254): Incursion 5+ and Two
  Fronts 3+ / 4+ deal two tier-2 threats in each country; Offensive 3+ deals a second tier-2 threat, 4+ builds first,
  Corrupt cities, then the two. The load check refuses a tier-3 pull anywhere.
- **The checks.** The load check (`content::check_threat`, `threat_rule_break`) and `check.py` (`standard_breaks`)
  refuse a tier other than 1-2, more than two counter options, more than two consequences, a shared currency, an
  each-player harm (a hex or country flag, an each-country line, `each_destroy_node`, `corrupt_city` at location) and
  a special piece (a standing or occupying troop, corp troops past their cap).
- **Cards.** 57 threats (25 tier 1, 32 tier 2): one consequence or two (the second most often compute), two counter
  options (a scarce currency or a tag sacrifice, or credits). Printed counters: tier 1 3 energy / 3 ambassadors / 6
  insight / 3 cubes / 6 chips / 2 tags / 12 credits; tier 2 4 / 4 / 7 / 5 / 7 / 3 tags / 17 (the screens' 85%
  folded in). Give / get at `values.json`: tier 1 median 1.20 (1.00-1.50), tier 2 median 1.25 (0.75-2.18; Total War
  2.18 prices its march as a strike a round early, Power Rationing 0.75 because a 3-card discard caps it).
- **The ladder.** d1: Corrupt X one fewer and every counter item +1 (+3 credits); d2: +1 more (+3); d3: the bigger
  build, a stage-II card out, and **the Militarist's difficulty line: 4 compute per uncountered threat** (new board op
  `threat_compute`, from `board_difficulty_at`), the replacement for `harm_bonus`; d4 as before.
- **Agents.** The eval weighs each unpaid threat by its printed harm at the value table over 15 credits
  (`w.threat_harm` 1, `w.threat_harm_ref` 15): a cheap harm may be let through, a dear one is paid. Smoke (d2, dev
  seeds, 96 games each): on 65.6, off 67.7 (within noise); with it on, the tier-2 counter rate went 68 to 74% and tier 1
  67 to 58%. The shelter forecast (`guide.rs`) reads `destroy_node` on the nearest seat.

## Why the board's compute line for d3

Screened against the as-built base's d3 42.7 on the dev seeds (96 games, paired): **+2 compute per uncountered threat**
-4.2 ±5.9; a bigger d3 counter step -3.1 ±5.8; an extra threat a round from d3 -24.0 ±6.2. The compute line prints on
the board (one line), is the standard's own suggestion ("an adversary board may add a flat amount per uncountered
threat, especially at higher difficulty") and keeps 142's ruling that below d4 the ladder raises intensity, not count;
the extra threat adds count and overshoots; a bigger printed harm at one level can't print on a card. It went from 2
to 4 once the Discord cards' two tier-2s and the smaller d2 step were in (dev d3 -3.1 for the extra 2).

## The reads (strategist, the three pairs pooled, d0-d3)

| Build | Seeds | d0 | d1 | d2 | d3 |
|---|---|---|---|---|---|
| as built (no d3 step), dev | 36800-36831, 96 a level | 88.5 | 77.1 | 65.6 | 42.7 |
| r61 frozen (2 compute at d3), first read | 36952-37047, 288 a level | 84.0 ±2.2 | 79.9 ±2.4 | 63.5 ±2.8 | 40.6 ±2.9 |
| + L1 (counters 85%, d1 +1/+3, d2 +2/+5), confirming read | 37048-37143 | 93.4 ±1.5 | 80.9 ±2.3 | 58.0 ±2.9 | 48.6 ±2.9 |
| + Matt's Discord ruling (6254), same seeds | 37048-37143 | 92.7 ±1.5 | 80.6 ±2.3 | 51.7 ±2.9 | 44.8 ±2.9 |
| **final** (d2 +1/+3, d3 4 compute), final read | 37144-37239 | **91.0 ±1.7** | **74.0 ±2.6** | **62.5 ±2.9** | **42.4 ±2.9** |
| final, set a / set b | 144 each | 91.0 / 91.0 | 73.6 / 74.3 | 67.4 / 57.6 | 47.2 / 37.5 |
| r60 on the final read's seeds | 37144-37239 | 89.9 ±1.8 | 86.1 ±2.0 | 56.2 ±2.9 | 45.8 ±2.9 |

Each read was spent by the change it led to and the next change confirmed on the next free seeds only. The first read
missed d0 by 6 (2.7 SE): L1 was picked on the dev seeds from P90+G3, P85+G3 and A1+G3 (A1, fewer created ambassadors,
took d2 -14.6). Matt's Discord ruling then took d2 under 55; the smaller d2 step (M1: d2 +12.5 ±6.0, d3 +6.2 ±6.1 on
the dev seeds) and d3's 4 compute were screened on the dev seeds and read once on 37144-37239. **Every band is met** on
that final read; d2's and d3's two sets differ by about 10 points (seed sets move a cell as much as a lever does).
The 85% fold replays L1's overlay cell exactly (72 dev games).

**What changed in play** (final read's seeds, r61 / r60): nodes lost to threats 0.19 / 1.93 (d0), 0.66 / 4.78 (d2),
0.85 / 7.69 (d3); to corruption 3.02 / 2.62, 3.80 / 2.43, 4.51 / 2.25 (threats create ambassadors on their sites and
games run longer: 9.3-9.6 rounds against 7.9-9.2). Ends at d2: Unity deck 174 / 158, Discord deck 99 / 108, node 0 /
17, round cap 9 / 0; at d3 Discord 136 / 92, node 0 / 64, round cap 30 / 0. A seat now loses through the Discord deck
and the clock, almost never its last node. Threats dealt a game 26-32 (r60 23-24: stage III deals two tier-2s a
country).

**Counter rate by tier at d2** (final read's seeds, countered over countered plus uncountered): r61 tier 1 66%, tier 2
72%; r60 tier 1 70%, tier 2 70%, tier 3 62%. **Every harm type lands uncountered**: all 57 threats executed
uncountered at least once in the final read's 1,152 games (the fewest: Propaganda Blitz 15, Insurrection 27, Backdoor
28).

**Forced pulls**: `threats.pull_forced` 0.01-0.06 a game (most at d3): a tier-2 deck runs dry when threats that deal
threats (Last Ultimatum, Full Spectrum Assault) and the Discord cards' two tier-2s add pulls to stage III's two a
country; `check.py`'s simulation doesn't model those.

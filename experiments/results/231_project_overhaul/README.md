# 231: project overhaul (r102)

`RULES_REVISION` 101 → 102, bumped from the constant. The prompt said to start from r99 (227). By launch, 228
(the balance pass) and 230 (antimatter follow-ups) had moved main to r101, so every comparison here is against r101.
Every number is a candidate. Matt's rulings (2026-11-14) are in the prompt. The rules text is
[docs/rules/adversary.md](../../../docs/rules/adversary.md).

## What changed

- **Per-board compute.** `compute_income` is now per country: at 2 players it doubles, since 2 players seat 2 countries
  (one seat a country, `state.rs`). Every printed "+N compute a round" is also N a country (`rally::increase_income`).
  Signal Jamming and Supply Lines print +1 a country, which was +2 flat: the same at 2 players. Botnets and uncountered
  threats were already per board. A project prints one per-board cost; `project_cost` is that × the countries.
  Agents value a compute rise per country too (`value.rs` `IncreaseIncome`; review finding, fixed before the confirm).
- **One-shot only.** `Content::check` refuses `ongoing`. Removed from the engine: `PROJECTS_ONGOING`, `ongoing_acting`,
  `standing_project_op` and `ProjectCard.resolve`. The viewer's ongoing group is gone too.
- **Every board.** War Plants builds a factory in each country (`FactoryWhere::EachCountry`); it used to build in the
  country with the most free zones. Shadow Factories is folded into it.
- **Night Raid** (new op `agents_on_nodes`). In each country, 1 agent lands on each of two different node hexes:
  most agents first, ties to the lowest-numbered hex. A firewall skips its hex, and an arrival cancels a player
  ambassador, as a Rally's does. The 2-agent rule destroys the node in the next Corrupt beat. Mob Rule and Sabotage
  Teams lost their raids, and `raid_node` is gone.
- **Bundles by cost.** A small project has one effect and a big one two; a third is allowed from `THIRD_EFFECT_COST`
  18 (it was 14 on the old scale).
- **Ledger gaps printed: 3 of 5.**
  - antimatter lost: Containment Breach (`requisition` in antimatter).
  - pieces pushed: Crackdown (new op `recall_ambassadors`).
  - a megaproject denied: Lab Purge (new op `strip_cubes`).
  The clock already had one source and stays on Accelerated Timetable. The card market has no source yet.

## The deck: 22 → 28 (17 small, 11 big)

All 22 projects were reviewed, and 25 were cut, reworked, replaced or added:

- **Cut (3):** War Footing, Factory Garrisons, Shadow Factories.
- **New (9):** Containment Breach, Crackdown, Lab Purge, Purge Lists, Dirty Bomb, Lab Seizure, Slander Campaign,
  Coastal Raids, Plant Seizure.
- **Reworked (13):** Night Raid, Mob Rule, Sabotage Teams, War Plants, Recruitment Drive, Propaganda Network (one-shot),
  Infiltration Drive (+Fallout), Accelerated Timetable (+a tier-2 threat a country), Grand Offensive (+Corrupt
  factories), Disinformation (Sway 8), Backdoor Research (4 cubes), Botnet Swarm (2 a country), Botnet Conscripts
  (3 troops).
- **Kept, repriced per board:** Rolling Blackouts, False Flag, Asset Seizure, Data Heist, Hostile Takeover (Rally 3),
  Station Coup.

Per-board costs are 5-22. The card list, each card's give / get at `configs/agents/values.json` and the generator are
in [`projects_231.py`](projects_231.py); `--write` rewrites `drafts/adversary/projects.json`. Raising the potency of the
first deck (screen 2's overlay P) is folded into the printed numbers.

## Calibration (screens on spent seeds, then one confirm)

| screen | seeds | what | strategist d0 / d1 / d2 / d3 | d2 resolutions (small / big) |
|---|---|---|---|---|
| s231a | 47008-47039 | deck v1 (22), dial 0,3,5,7 | 83.3 / 81.2 / 68.8 / 56.2 | 10.4 (5.1 / 5.3) |
| s231b | same | P (potency) × D (0,4,7,10) | P: 84.4 / 78.1 / 63.5 / 51.0 | P: 10.9 |
| s231c | same | deck v2 (26), d3 7 / 9 | 80.2 / 75.0 / 63.5 / 47.9 (d3 9) | 10.4 (6.1 / 4.3) |
| f231 | 47040-47087 (fresh, then spent) | v2, 0,3,5,9 | 88.2 / 76.4 / 68.8 / 60.4 | 9.2 (5.3 / 3.9) |
| s231d | 47008-47087 | agents fix; d2 5 / 6, d3 9 / 12 / 13 | d2 66.7 / 65.4; d3 55.4 / 51.7 / 49.2 | 9.7 / 10.8 |

What the screens showed:

- The dial barely moves d2. Going from 5 to 6 a country cost 1.3 points and pushed throughput past target, so it
  stayed at 5.
- d3 went to 13 a country, and d4 to 18. d4 was never screened: its number is scaled from d3.
- Adding Coastal Raids and Plant Seizure gave a d3 game 28 cards, because s231d at d3 13 used all 26.
- The fresh read f231 counts as spent: the agents' valuation fix and the d3 dial changed the build after it.

## The confirm read (fresh 47088-47135, paired against r101)

Two players; pairs B+P, B+R and P+R; 48 seeds a cell; d0-d3. Each row below is 144 games (the three pairs pooled).

| agent | d | r102 win % | r101 win % | gap ±SE | band | rounds | resolutions (small / big) | r101 resolutions |
|---|---|---|---|---|---|---|---|---|
| strategist | 0 | 85.4 | 85.4 | +0.0 ±3.9 | 85+ met | 10.6 | 2.9 (2.0 / 0.9) | 4.0 |
| strategist | 1 | 75.7 | 65.3 | +10.4 ±4.8 | 65-80 met | 10.5 | 6.2 (4.3 / 1.9) | 11.3 |
| strategist | 2 | 72.2 | 56.9 | +15.3 ±5.2 | 50-65 **unmet** | 9.8 | 10.5 (7.0 / 3.5) | 14.7 |
| strategist | 3 | 42.4 | 43.1 | −0.7 ±5.6 | 35-50 met | 8.6 | 15.4 (10.0 / 5.5) | 15.7 |
| greedy | 0 | 85.4 | 84.0 | +1.4 ±3.8 | | 10.5 | 2.7 | |
| greedy | 1 | 84.7 | 72.9 | +11.8 ±4.4 | | 9.6 | 5.3 | |
| greedy | 2 | 64.6 | 49.3 | +15.3 ±5.3 | | 9.7 | 10.2 (6.8 / 3.4) | |
| greedy | 3 | 47.2 | 41.0 | +6.2 ±5.0 | | 8.7 | 15.6 | |

- **Throughput.** d2 resolves 10.5 projects a game, about 1 a round: 7.0 small (target 5-6) and 3.5 big (target 3-4).
  d0 resolves 2.9 and d1 6.2, fewer as asked. r101 resolved 4.0, 11.3, 14.7 and 15.7 per level.
- **The d2 band misses by 7** (SE about 4). The dial can't close it within throughput (s231d), so the miss is
  reported, not chased (the prompt's rule and message 10322). The candidate lever is the potency of the big projects
  at the same costs. Screen 5's overlay was stopped by message 10322 and stays unread: Mob Rule Sway 5; Data Heist 4
  tokens, 10 credits or 3 cards; Hostile Takeover and Grand Offensive corrupt with 1 fewer; Recruitment Drive Rally 3;
  Propaganda Network Sway 8. d2 resolves about 4 times as many big projects as d0, so this lever leaves d0 nearly
  alone.
- **The deck never reshuffled.** The most draws in a game were 28 (d3, one game) and 23 at d2. That is right at the
  28-card deck's edge, so a 30-card deck is a candidate (one small card, one big).
- **Every project completes in d2-d3 games.** The per-project table (revealed, completes, completion share, per
  difficulty, both agents pooled) is [projects.md](projects.md). Its "ongoing" columns are 195's and read 0.
- **Resource use:** [resource-use.md](resource-use.md). Games run about a round longer at d1-d2, so a seat spends
  about 10% more of everything. Run-out shares are within 1 point of r101.
- **Design check:** [design-check.md](design-check.md) (its first line is the report header's warnings).

## Speed

`speed.py check` gives no verdict across the revision change. On `per_decision.py` (r101 against r102, d2, c2):

| agent | ms per decision | options per decision | games |
|---|---|---|---|
| greedy | 1.56× | 1.44× | 6, pre-confirm build |
| greedy | 0.85× | 0.93× | 12, final build |
| strategist | 1.07× | | 4, pre-confirm build |

The 6-game greedy number came from the game mix: on 12 games the options per decision fell back. The baseline was
updated (`--update`) on the final build.

## Recipes

- **Farm.** `farm-seeds F N OUT SPEC_NAME` takes the spec's name, not `experiment SPEC`. The broker went away mid-job
  twice in this item, so screens 2-5 and both reads ran natively: about 10 min for 2,304 games on 6 threads.
- **Readers:**
  - [`arms.py`](arms.py): arms against a base, paired, with small and big resolutions.
  - [`compute_stats.py`](compute_stats.py): compute by source.
  - [`deck_use.py`](deck_use.py): draws a game (`adversary.projects_revealed` counts every draw).
  - [`specs.py`](specs.py): writes the screen specs.

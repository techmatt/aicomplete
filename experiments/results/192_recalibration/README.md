# 192: recalibration after r80-r83 and 191 (r84)

`RULES_REVISION` 83 → 84, bumped from the constant (191 was content-only, no bump). Built on main c599c75 (191).
Every number is a candidate.

## Where it started

191's card pass left every band unmet (its read, seeds 43759-43902: 86.8 / 60.6 / 45.1 / 35.9 / 34.7). The base
here is that build (`out/base`, a copy of c599c75's binary and configs), read on this item's fresh seeds
43903-44046: **81.9 / 68.3 / 44.4 / 38.9 / 35.9** (strategist, 432 games a level), greedy 81.9 / 69.4 / 41.0 /
34.0 / 27.1 (144). Bands need about +8 at d0, +2 at d1, +11 at d2, +1 at d3.

## What changed (r84)

| lever | r83 | r84 | read |
|---|---|---|---|
| Vigil's cap (`board.vigil_cap`) | 1 | **2** | screen 1: +4 / +3 / +6 / +4 / +1 at d0-d4 (72 games a level, SE ~3.5); the Protector board's text |
| Land corruption a node hex takes (`adversary.blight_node_discord`, new) | drips 1 | **face down: no drip** | see *The drip* |
| `compute_income` d0-d4 | 1, 4, 22, 24, 24 | **1, 4, 19, 24, 24** | 22 → 14 at d2: +15; 16: +22; 18: +8; 17 read fresh 67.8; 19 confirmed below |
| `ambassador_income` d0-d4 | 0, 3, 3, 3, 3 | **0, 2, 2, 3, 3** | 2 at d3 overshot (+12 at d3); 3 kept there |
| The Protector's Muster skipping players' factories (`adversary.wall_landing`, new) | anyone's | **unchanged (0)** | inert: players own 0.01-0.34 factories a game since r83 (no neutral claims), so 714 of 720 games were identical |
| The eval's adversary cube (`w.adversary_cube`, new Weights field) | 0.5 | **0.5** | inert: 0, 0.5 and 1 play identical games (16 seeds, d0-d4, three pairs): no action moves an adversary cube inside the eval's one step |

**The drip.** Land corruption's drip is small at r83: 0.62 Discord a game in all (its node-hex share less). On the
dev seeds face-down land corruption read +0 to +3 at d0-d1. On the fresh seeds it read the other way at d0: the r84
build with only the drip put back plays r83's 144 br d0 games exactly, and r84 loses 9 of them more (br d0 87.5 →
78.5, −9.0 ± 2.6; every d0 loss is the round cap). Put back on the spent fresh seeds (screen 4, 72 seeds, 216
games a level), the drip lifts d0 about 6 and drops d3 about 4, below its band. Kept face down: d0 misses either way,
d3 meets only with it. Why less Discord makes d0 slower is not diagnosed (Needs a decision, below).

## Ladder

Strategist, three pairs, paired against r83 as 191 left it. d0, d1, d3, d4 from the first package on 43903-44046
(those levels' games are unchanged by the later d2 dial); d2 on the next free seeds 44047-44190 with compute 19 (the
first package read d2 67.8 at compute 17, over its band by 2.8).

| d | r84 strategist | r83 (191) | gap ± SE | band | r84 greedy | r83 greedy |
|---|---|---|---|---|---|---|
| d0 | 78.9 | 81.9 | −3.0 ± 1.9 | 90-100 **unmet by 11.1** | 85.4 | 81.9 |
| d1 | 78.0 | 68.3 | +9.7 ± 2.7 | 70-80 met | 72.2 | 69.4 |
| d2 | **60.2** (44047-44190, compute 19) | 41.2 | +19.0 ± 2.9 | 55-65 met | 68.8 (compute 17) | 41.0 |
| d3 | 42.4 | 38.9 | +3.5 ± 2.0 | 40-50 met | 37.5 | 34.0 |
| d4 | 38.2 | 35.9 | +2.3 ± 2.0 | - (below d3) | 31.9 | 27.1 |

d2 at compute 17 on 43903-44046 read 67.8 (+23.4 ± 2.7), over its band; the screen-4 estimate for 19 was about
−1.5 to −3 on top of that, and the next free seeds read 60.2. Greedy (48 seeds a pair) ran with the first package
only. Pairs (`pairs-strategist.md`): d0 bp −6.9 ± 3.6, br −9.0 ± 2.6, pr +6.9 ± 3.5; d2 (confirm) bp +22.2, br
+13.9, pr +20.8.

Tables: [`ladder-strategist.md`](ladder-strategist.md) (with the standing measures), [`ladder-greedy.md`](ladder-greedy.md),
[`pairs-strategist.md`](pairs-strategist.md), [`confirm-d2.md`](confirm-d2.md).

**d0 resisted.** Every d0 lever was tried: Vigil 2 (+4) and 3 (no more), compute 0 and 2 at d0 (both read about 7
worse than 1 on the dev seeds), the drip both ways (above). The best d0 read anywhere is about 85 (screen 4, drip
back); the band floor is 90. The d0 losses are all the round cap (deck_empty): the players are too slow to purge,
which no adversary dial at d0 (already the lowest income) speeds up. The next lever is the players' side or the
round cap at d0 (a design question, not a dial here).

## Screens (dev seeds 36800-36831, strategist, three pairs, d0-d4)

`screen_spec.py` / `screen2_spec.py` (specs `experiments/farm/f192-screen*.json`), read by `ladder.py --screen`
(screen 1's first 8 seeds farmed, the rest native: the farm's broker dropped and its local fallback ran one game at
a time). Gaps against base on the same seeds (72 games a level unless said):

| cell | d0 | d1 | d2 | d3 | d4 |
|---|---|---|---|---|---|
| Vigil 2 | +4.2 | +2.8 | +5.6 | +4.2 | +1.4 |
| face-down drip | +0.0 | +6.9 | +4.2 | +0.0 | +1.4 |
| Muster landing | +1.4 | 0 | 0 | 0 | 0 |
| the three | +6.9 | −2.8 | +5.6 | +4.2 | +4.2 |
| three + agents 0/2/2/2/3 | +6.9 | +5.6 | +8.3 | +12.5 | +4.2 |
| three + compute 1/2/14/16/24 | +6.9 | −9.7 | +15.3 | +4.2 | +4.2 |
| p16 (compute 0/4/16, agents 2/2/2) | +0.0 | +4.1 | +22.2 | +12.5 | +4.2 |
| q (compute 1/4/17, agents 0/2/2/3/3; frozen) | +7.0 | +4.1 | +16.7 | +4.1 | +4.2 |
| q with the drip | +4.2 | +1.4 | +16.7 | +4.1 | +1.4 |

## Matt's watch list

The first package's fresh read (43903-44046, 2,160 games a side; d2 there at compute 17), r84 against r83. Tables:
[`watch192.md`](watch192.md), [`watch189.md`](watch189.md), [`plays.md`](plays.md),
[`resource-use.md`](resource-use.md), [`map-by-round.md`](map-by-round.md) (its column labels say r83 / r82 from
190's script: read them as r84 / r83).

| per game (both seats unless said) | r84 | r83 |
|---|---|---|
| blocked Rally shares (agents a firewall or no-Rally token kept in the supply) | 2.26 (games with any 44%) | 2.28 (41%) |
| of them by a no-Rally token | 1.21 | 1.23 |
| insight binding: a drafted card unkept for want of insight, a seat | 0.10 | 0.09 |
| insight binding: cards left in hand short of insight, a seat (credits: 91.5) | 2.84 | 2.79 |
| insight ran out (share of seat-rounds) | 2% | 2% |
| plays by insight cost, tier 1 at 0 / 1 / 2 | 12.1 / 12.2 / 3.8 | 11.0 / 11.3 / 3.5 |
| plays by insight cost, tier 2 at 0 / 1 / 2 | 4.1 / 4.3 / 2.2 | 3.9 / 4.0 / 2.0 |
| the opening's tier-2s (4 dealt, free): played | 1.17 (29%) | 1.12 (28%) |
| Patience: insight carried a Researcher game / rounds carrying any | 86.1 / 83% | 82.5 / 83% |
| **Focus** (Builder, 5 credits → 1 insight) uses a Builder game | **0.00** | **0.00** |
| neutral factory claims / neutral station claims (by round 10) | 0.00 / 0.03 | 0.00 / 0.04 |
| city builds (factory + station, summed over rounds 1-10, per game still running) | 2.6 + 3.4 | 2.3 + 3.4 |
| repriced node cards' plays: National Grid / Beamed Power / Relay Mast / Aegis Grid | 0.11 / 0.02 / 0.76 / 0.39 | 0.10 / 0.02 / 0.71 / 0.37 |
| adversary cubes on megaprojects: landed / paid | 5.91 / 5.84 | 6.63 / 6.57 |
| Vigil insight a Protector game | 12.9 | 6.6 |
| Discord from land corruption / land corruption cleared | 0.13 / 0.18 | 0.62 / 1.51 |

(189's script prints the r83 side's opening kept as 0: its kept rule predates the free deal; both sides keep all 4.)
Focus is never used at either revision: insight runs out in 2% of seat-rounds and credits are what stop plays, so
paying 5 credits for an insight never pays. Neutral factories are still never claimed by the agents (190's open
question: the eval's `owned_factory` 4 against 3 ambassadors). Face down, node-hex land corruption is no longer worth
clearing to the agents (1.51 → 0.18 cleared a game).

## Speed

The levers' code, at r83's numbers, plays r83's games exactly (the farm fingerprint and 144 br d0 games identical),
so any speed change is the games. `per_decision.py out/base out/new84` (8 games, seed 1000, d2, one thread, one pass,
quiet box): greedy 1.15x a decision (1.00x the options, 1.17x the rounds: d2's games run longer at the lower compute),
strategist 0.87x. Within the noise of 8 games. `speed.py check --update` set the baseline at r84.

## Checks

Fast lane and `cargo test-all` green, clippy clean, `aic-sim docs --check` current, `regen.py` run. The view
test caught the face-down count as a global: it is bookkeeping now (`adversary/mod.rs`). Not run: `npm run
cards:fit` (no card face changed; the Protector board's text did, one line), `loop_smoke.py` (no card or op
changed), `bench/same_results.py` (no core rules file changed after the fresh reads but the view's bookkeeping list,
which plays nothing). docs/rules-history.md was over budget: rows r73-r74 left it (git keeps them).

## Files

`screen_spec.py`, `screen2_spec.py` (screens 1-3), `watch192.py` (Focus, Vigil, adversary cubes, the drip, Muster
landings); the earlier readers ran as is (189 `watch.py`, `resource_use.py`; 190 `map_read.py`; 191 `plays.py`;
187 `by_pair.py`). Seeds 43903-44190 spent (ledger next free 44191).

## For new players (r84's changes)

The Protector's Vigil now pays for the first two troops each round that end their march next to its nodes, not
just the first. When a node hex corrupts, its land corruption token goes on face down: it still blocks building, but
only face-up land corruption (open land's) adds Discord each round. At levels 1 and 2 the adversary gains 2 agents a
country a round instead of 3, and at level 2 its compute income is 19 instead of 22. **What resisted the summary:**
nothing; the face-down token is one more thing to remember for a Clear (clearing one now gains nothing but space).

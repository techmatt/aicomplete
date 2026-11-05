# 118 bug hunt: the reads (r46 → r47)

The hunt's sim reads, for what its fixes did to play. All on `experiments/use-090.json` (2 players, the three board
pairs, d2, the engine and the strategist), 48 seeds a cell, 288 games a run, 3 threads on a shared box (no timing
read is a verdict; see Speed).

## Same seeds, r46 against r47 (34272-34319)

| run | wins | rate | note |
|---|---|---|---|
| r46 (`710aecc`) | 165 / 288 | 57.3% ± 2.9 | |
| r47, batch 1 | 161 / 288 | 55.9% | |
| r47, final (`fc5f043`) | 150 / 288 | 52.1% ± 2.9 | 69 games changed outcome: 42 to a loss, 27 to a win |

Not a factorial: the fixes landed together. What moved: threats delayed 1.59 → 0.61 a game and threats executed 2.00 →
2.27 (F136: a delay reaches only threats you may counter), Autonomous Defense Grid's counters 0.33 → 1.00 (F156),
triggers 58 → 61 and At N payouts 5.8 → 6.2 a game (F133, F134). The likeliest cause of the drop is F136, the one fix
that takes something from the players; it follows the house term "a threat", so whether it stays is **Matt's call**.

## Fresh seeds, r47 final (34500-34547)

152 / 288, 52.8% ± 2.9: the same level as the same-seed read. `runaway_read.py`: no counter in the hundreds over a
handful mean; 9 games bank over 150 insight (D43). `play_read.py`: Moonshot Grant kept, never played (117's known
read); Hyperscale Botnet kept in 46% of strategist games and played in none (D53; at r46 2%).

## Plays of the cards the fixes touched (same seeds, played a game, engine / strategist)

| card | r46 | r47 | fix |
|---|---|---|---|
| Summit Caucus | 0.34 / 0.38 | 0.56 / 0.58 | F135, F186 |
| Liaison Corps | 0.56 / 0.62 | 0.74 / 0.77 | F135 |
| Early Warning Drill | 0.41 / 0.42 | 0.65 / 0.55 | F136 (0.16 / 0.12 before its veto was lifted) |
| Predictive Modeling | 0.62 / 0.58 | 0.64 / 0.62 | F136 |
| Consular Pouch | 57 / 48 plays | 51 / 56 plays | F183 |

## 1-4 countries, r47 batch 1 (34320-34351, 32 seeds a cell)

1 country: the engine 28%, the strategist 31%, rounds 5.7, most losses a seat's last node (balance, not a bug; D55 is
the missing military base). 3 countries: 88% / 72%; 4 countries: 72% / 84%.

## The viewer sweep

r46 (13 cases × 2 seeds, before the crash its browser's cleanup by a play session caused): no hard failures.
r47 (13 cases × 2 seeds, 3024 captures, 4260 s): no hard failure, no step over 150 ms, heap growth 5-25 MB a game;
925 "overlap" reports were the checker's (F178). r47 final (3 cases): only W16's truncations.

## Speed

Per decision on one thread (`bench/per_decision.py`, engine, 6 games): r47 batch 1 0.94× r46 at 3 countries; r47 final
0.89× at 2 countries and 1.16× at 3, options a decision 1.06×. Game-level ratios move with game length (the fixes change
games), and the box was shared all night: no verdict, no `--update` of the baseline. One self-made cost was found and
fixed on the way (F188's helper cloned every hand card's ops on each legal pass; it borrows now).

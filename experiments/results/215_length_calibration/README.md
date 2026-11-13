# 215: game length and calibration (r96)

`RULES_REVISION` 95 → 96, bumped from the constant (the prompt named none). Built on main eb5f625 (214, r95). Every
lever is a config overlay on one binary, so r95 is put back by overlay (`r95` in [spec.py](spec.py)) on the same seeds
as r96. Reads: [rounds_read.py](rounds_read.py) (rounds by level and end), [screen_read.py](screen_read.py) (a screen's
levers against a base, win and rounds paired), [node_read.py](node_read.py) (node losses by source),
[split_read.py](split_read.py) (a farmed screen into per-lever read folders for `design_check.py` and `ladder.py`).

## What changed

- **The deck length:** `adversary.stage_cards` 5, 5, 3 (was 4, 4, 3). The round cap is 13 at d0-d2 and 11 at d3-d4
  (11 and 9), and stage II starts in round 6 (5). There are only 5 stage-I cards, so 6, 5, 3 is refused.
- **The Discord tip size:** `alignment.discord_tip_per_player` 4 (was 3). Unity's `tip_per_player` stays at 3. The prompt
  didn't name this lever. It shipped as a candidate by the default of ask 1.
- **The purge's last step at 7 Unity tips:** `purge_at` 3, 7, 7 (was 3, 6, 6), also a candidate lever.
- **Bands** loosened and made soft (Matt, 2026-11-13): 85+ / 65-80 / 50-65 / 35-50, in `principles.md` and
  `bench/ladder.py`'s default `--bands`.

## Where the length went (r95, 214's fresh read 46432-46527)

Games ran 8.81 / 8.24 / 7.59 / 7.12 / 6.00 rounds at d0-d4. Wins came at 8.1 / 7.5 / 7.0 rounds. Node losses (40% of
d2 games, 35% at d3, 66% at d4) came at 5.5-7.2 rounds. Round-cap losses counted 11.1-11.8 (the cap plus one, as
`docs/experiments.md` says). So d2's length is set by its early node losses and its early wins.

## Reachability check (spent fresh 46432-46479, 48 seeds × 3 pairs: 144 games a cell and level, d0-d2)

| lever | Δwin d0 / d1 / d2 | rounds d0 / d1 / d2 (base 8.93 / 8.20 / 7.61) |
|---|---|---|
| `u4` Unity tip 4 | −40.3 / −22.2 / −7.6 | 10.81 / 9.67 / 8.29 |
| `u5` Unity tip 5 | −68.1 / −33.3 / −18.8 | 11.47 / 9.92 / 8.59 |
| `dc4` Discord tip 4 | −2.8 / +2.8 / +13.2 | 9.22 / 8.67 / 7.85 |
| `s5` deck 5, 4, 3 | +4.9 / +12.5 / +5.6 | 9.27 / 8.15 / 8.02 |
| `s55` deck 5, 5, 3 | +5.6 / +5.6 / +10.4 | 9.61 / 8.12 / 8.00 |
| `u4s5` | −30.6 / −18.8 / −3.5 | 11.38 / 9.58 / 8.68 |

The Unity tip size buys length by turning d0 wins into round-cap losses (u4: 60% of d0 games), so it is out. The deck
length and the Discord tip add rounds and wins together.

## Screens (spent fresh 46240-46431, 192 seeds × 3 pairs: 576 games a cell and level, d0-d3, paired with r95)

| cell | win d0 / d1 / d2 / d3 | rounds d0 / d1 / d2 / d3 |
|---|---|---|
| r95 (base) | 75.9 / 56.1 / 43.1 / 30.0 | 8.88 / 8.11 / 7.24 / 6.91 |
| `dc4` | +3.1 / +4.5 / −0.3 / +4.7 | +0.09 / +0.00 / +0.33 / +0.10 |
| `s5` | +10.2 / +5.4 / +2.8 / +8.0 | +0.08 / +0.12 / +0.22 / +0.13 |
| `s5dc4` | +5.7 / +9.9 / +3.0 / +11.5 | +0.24 / +0.26 / +0.47 / +0.35 |
| `s55dc4` | +13.5 / +9.0 / +7.3 / +15.1 | +0.31 / +0.42 / +0.55 / +0.48 |
| screen 2, one change each on `s55dc4`: | | |
| `s56dc4` (a sixth stage-II card) | +13.4 / +11.3 / +7.6 / +14.6 | +0.59 / +0.36 / +0.43 / +0.59 |
| `s55dc5` (Discord tip 5) | +13.9 / +11.5 / +8.7 / +14.8 | +0.26 / +0.34 / +0.53 / +0.55 |
| **`s55dc4p7`** (purge's last step at 7) | +8.0 / +7.6 / +6.8 / +13.5 | +1.07 / +0.79 / +0.68 / +0.70 |
| `s55dc4u4` (Unity tip 4) | −4.7 / −3.5 / −6.1 / −1.4 | +2.26 / +1.43 / +1.17 / +1.15 |

SE about ±2.1 on win and ±0.10 on rounds. Taken: **`s55dc4p7`**, at 9.95 / 8.89 / 7.92 rounds and 83.9 / 63.7 / 49.8 /
43.6 win. It was the only cell with d0 under 10 and d2 within 0.1 of 8. `s55dc4u4` is the only cell with d2 over 8 on
these seeds, but it puts d0 at 11.1 rounds and costs 1-6 win points. Matt's #9772 ruled out a third screen.

## Fresh read (new seeds 46528-46623, 96 seeds × 3 pairs, d0-d4; r95 by overlay on the same seeds)

| d | r96 strategist | r95 | Δ (paired) | band | rounds r96 (win / loss) | rounds r95 | r96 greedy (rounds) |
|---|---|---|---|---|---|---|---|
| d0 | 85.4 | 79.5 | +5.9 ± 2.9 | ≥ 85 met | **10.12** (9.57 / 13.38) | 8.77 | 87.2 (9.70) |
| d1 | 68.8 | 58.0 | +10.8 ± 3.5 | 65-80 met | 9.28 (8.33 / 11.37) | 8.35 | 67.4 (8.69) |
| d2 | 54.9 | 43.4 | +11.5 ± 3.1 | 50-65 met | 8.07 (8.03 / 8.12) | 7.19 | 54.2 (7.99) |
| d3 | 46.5 | 37.8 | +8.7 ± 3.0 | 35-50 met | 7.66 | 6.85 | 43.1 (7.48) |
| d4 | 20.1 | 17.4 | +2.8 ± 2.5 | - | 6.44 | 5.74 | 26.4 (6.46) |

All four bands are met. Rounds are paired with r95 at +1.35 / +0.93 / +0.88 / +0.81 / +0.69 (SE 0.12-0.17). d1 and d2
are in 8-10. **d0 runs 10.12, 0.12 over (about 1 SE)**: its 14% round-cap losses count 14 rounds by the measure, 13
played, so the rounds played average 9.98. The spread between d0 and d2 is set by d0's round-cap losses: about 2.05
rounds here and 2.03 on the screen, so a 2-round band holds both levels only at its edges. Greedy reads within 3.5 of
the strategist at every level (d4 +6.3).

**Screen 3, after the fresh read** (spent 46240-46431, 576 a cell, d0-d2; r96 reproduced screen 2's `s55dc4p7` game
for game). A 12-card deck (`s552`: 5, 5, 2) moves rounds −0.15 / −0.20 / −0.10 ± 0.08 and win −6.1 / +0.0 / −2.6 ± 1.9.
It brings d0's rounds in by about the overshoot but costs d0 six points (its band) and d2 a tenth of a round, so it
was not taken. d0's overshoot stands.

Node losses (d2, a game): 9.84 nodes lost (r95 9.40). Node-loss games end at 6.76 rounds (r95 5.89). Fewer threat
losses (0.52 against 0.75), and more from projects (1.54 against 1.44).

## Resource use (fresh, strategist, a seat-game, gained / spent)

| | credits | insight | energy | chips | antimatter | research cubes | card tokens | ambassadors |
|---|---|---|---|---|---|---|---|---|
| r96 | 259.0 / 274.9 | 51.7 / 43.7 | 38.3 / 26.6 | 9.3 / 5.9 | 4.0 / 1.6 | 17.9 / 12.0 | 2.4 / 2.0 | 21.6 / 16.4 |
| r95 | 210.4 / 228.0 | 44.2 / 37.6 | 32.0 / 23.2 | 7.0 / 4.9 | 2.8 / 1.2 | 13.5 / 8.9 | 1.5 / 1.3 | 16.9 / 13.7 |

A seat-game is about 13% longer (8.31 rounds against 7.38, levels pooled). Per round, credits rise about 10% and
antimatter about 27%. The spent shares hold: antimatter 40% (r95 43%), energy 69% (72%).

## Checks

- Design check: [design-fresh.txt](design-fresh.txt) (r96 fresh against r95 on its seeds) and
  [design-screen.txt](design-screen.txt). New at r96: `rounds d0 10.1`. Cleared: `rounds d2`. `rounds d3 7.7` and
  `d4 6.4` stand, which the prompt allows. `kept-not-played` is now Field Grant (kept 0.19, played 0.01 a game); 214's
  Deep State Ledger has cleared. The other checks stand as at r95.
- Speed (`per_decision.py`, r95's configs against r96's on the same binary, 6 games at d2, one thread, on a loaded box,
  so ratios only): the strategist 1.17× a decision (with the same decisions a game), greedy 0.95×. The engine code is
  r95's, so the content alone moves these.
- No cards changed, so `play_read.py`, `loop_smoke.py` and `cards:fit` were not run.

## Seeds and runs

Spent fresh 46432-46479 (reach check, 3,024 games) and 46240-46431 (screens 1-3: 11,520 + 9,216 + 3,456 games). Fresh
46528-46623: 2,880 strategist games (r96 and r95) and 1,440 greedy. Ledger next free 46624. Kept: `out/keep/f215-base`,
`out/keep/f215-r95` and `out/keep/f215g`, with the screens' farmed files as `out/keep/s215-*.jsonl`.

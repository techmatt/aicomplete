# 092: test-agent heuristics at r35

Rules revision 35, unchanged. Four strategy-guide switches (docs/agents.md, *Strategy-guide
heuristics*), all on by default: `w.shelter_read` (with `w.shelter_counter`), `plan_bury`,
`plan_cubes`, `strat_powers` (with `strat_power_uses`). With all four off the strategist plays
090's games exactly (24 games on 31100, every column but timing identical).

Specs: `experiments/screen-092.json` (d2: all on, all off, each off alone; 864 games, about
2 min on 6 threads) and `screen-092-d0.json` (d0: engine and strategist, on and all off; 576
games, about 2 min). 2 players, the three pairs, 48 seeds a cell.

## Seeds

- **31400**, the first build (shelter credited every unpaid destroying threat; powers counted
  over 4 uses). Read once, then two heuristics changed, so it picked them: not a confirmation.
- **31100** (090's seeds) for the two fixes' development.
- **31500**, the final build, fresh: the read below.

## Final read, d2 strategist, seeds 31500

| pair | all on | all off (090's agent) | shelter off | bury off | cubes off | powers off |
|---|---|---|---|---|---|---|
| bp | 72.9 ±6.4 | 75.0 ±6.2 | 64.6 ±6.9 | 75.0 ±6.2 | 72.9 ±6.4 | 70.8 ±6.6 |
| br | 72.9 ±6.4 | 60.4 ±7.1 | 68.8 ±6.7 | 68.8 ±6.7 | 72.9 ±6.4 | 66.7 ±6.8 |
| pr | 58.3 ±7.1 | 68.8 ±6.7 | 60.4 ±7.1 | 62.5 ±7.0 | 58.3 ±7.1 | 66.7 ±6.8 |

Off minus on, paired seed by seed (`scripts/bench/paired.py`):

| switch off | bp | br | pr | pooled, 144 seeds |
|---|---|---|---|---|
| all four | +2.1 ±6.3 | −12.5 ±6.4 | +10.4 ±7.4 | +0.0 ±3.9 |
| `w.shelter_read` | −8.3 ±5.8 | −4.2 ±4.2 | +2.1 ±4.7 | −3.5 ±2.9 |
| `plan_bury` | +2.1 ±5.6 | −4.2 ±4.2 | +4.2 ±5.1 | +0.7 ±2.9 |
| `plan_cubes` | 0 | 0 | 0 | 0 (no game changed) |
| `strat_powers` | −2.1 ±3.6 | −6.2 ±6.9 | +8.3 ±7.2 | +0.0 ±3.6 |

Beside 090 (strategist d2 50 / 65 / 69 on 31100, target 45-85): 72.9 / 72.9 / 58.3 on, and
090's own agent reads 75.0 / 60.4 / 68.8 on these seeds, so seed sets alone move a pair 10-25
points at 48 seeds.

**Shelter** (the Protector's power), a game: 0.33 bp, 0.46 pr on; 0.00 off (090: 0-0.02).
Lost nodes a game, on / all off: bp 7.5 / 6.9, br 5.6 / 5.9, pr 7.2 / 6.1.

## d0, seeds 31500

| pair | engine on | engine off | strategist on | strategist off |
|---|---|---|---|---|
| bp | 77.1 | 85.4 | 89.6 | 85.4 |
| br | 85.4 | 85.4 | 87.5 | 87.5 |
| pr | 93.8 | 89.6 | 91.7 | 91.7 |

Engine off minus on, pooled: +1.4 ±2.6; strategist −1.4 ±2.4. Beside 090's greedy d0 77 / 81 /
90 (target 70-85): 77 / 85 / 94 on, pr above the target as 090's was near it.

## Build-overs and slot fill (d2, 31500; full tables `slots-d2.md`, `slots-d0.md`)

Build-overs a game per seat, plan_bury on / off: bp Builder 0.73 / 0.60, Protector 0.88 /
0.79; br Builder 0.79 / 0.96, Researcher 1.15 / 1.15; pr Protector 0.71 / 0.65, Researcher
0.94 / 0.90. Slot fill as rounds 2, 4, 6, 8 start (on): Builder (5 slots) 1.7, 2.8, 3.7, 4.4;
Protector (3) 1.8, 2.1, 2.5, 2.8 (bp) and 1.7, 2.2, 2.5, 2.7 (pr); Researcher (2) full from
round 5. The one-step eval already drops a buried building's income (horizon mode reads
income from tops only), so the keep plan's charge changes few games.

## The first build, seeds 31400 (picked the fixes)

Off minus on, pooled: shelter +7.6 ±4.0 (pr +14.6 ±6.7), powers +6.2 ±4.4, bury +0.7 ±2.1,
cubes 0, all four +3.5 ±4.7. Shelter was used 1.1-1.2 a game: it credited every destroying
threat still unpaid, though the team counters most threats, so seats spent
a Security card and an energy on a country they then also paid for. Powers over 4 uses set
every Builder's strategy to Power grid and every Researcher's to Patience burst.

## Dev, seeds 31100 (090's)

Off minus on, pooled: shelter −2.1 ±2.7 (Shelter 0.33 a game), counter-first off −1.4 ±4.3
(Shelter 1.2 a game), powers off −5.6 ±2.7, powers at 4 uses −0.7 ±3.2. With one use, Builders
still pick Power grid 37 of 48 (Overcharge pays energy; Mint chips).

## Not added

- **Bulwark, Hardened Bunker Complex** (a hardening token): no rule. The token stays until it
  absorbs, and the eval already counts it (`w.harden`); a "harden what a destruction showing
  will hit" rule would follow `guide.rs`'s shelter read, unscreened here.
- **Aegis Dome Array, Aegis Doctrine** (automatic): no decision to guide.

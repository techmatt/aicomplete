# One guess or sampled deals (061, no rules change, r25)

Fresh seeds, never used to pick anything: band `--seed 8000` (`experiments/band-061.json`), gate `--seed 13000`.
The r23 tuning numbers in `screen-054-deals/deals.md` are history: all of this is r25.

## Verdict: one guess is the default

`planner` is now `deals` 1, `lead` 0, `prune` 12. `planner:deals=4,lead=60` is the 054 planner.

| 48 seeds a cell, d2 | c2 | c3 | c2 s/game | c3 s/game |
|---|---|---|---|---|
| one guess | **67** | **75** | 16.2 | 48.6 |
| 054 (`deals=4,lead=60`) | 67 | 69 | 20.6 | 60.9 |
| engine | 67 | 44 | 1.5 | 4.0 |
| one guess minus 054, paired | +0 ±8.4 | +6 ±7.5 | | |
| 054 minus engine, paired | +0 ±9.4 | +25 ±8.7 | | |

One guess is at least as strong, within the noise, and about 0.8 × the time. Both sizes are in 60-75, so
nothing is re-centred and no rule changed. 75 is the band's top edge, ±6: the next rules item that adds
push has room at c3 and little at c2. The prompt's 1.76 × a game was not reproduced on 48 seeds: the two
differ by 1.25-1.28 × (see *Games a minute*).

**At c2 neither planner beats the engine** (+0 ±9.4). Search is worth 31 points at c3 and nothing
measurable at c2 on these seeds.

The default replays the spec exactly: 96 of 96 games on seeds 8000 are action for action the same as
`planner:deals=1,lead=0`'s.

## Games a minute

| 8 threads, same seeds, a-b-b-a (`speed.py specs`) | c2, 16 games | c3, 8 games | gate mix |
|---|---|---|---|
| one guess | 29.6 | 8.8 | 19 (388 games, 061) |
| 054 | 24.1 | 5.4 | 15 (055, r24; another seed set) |
| time, 054 over one guess | 1.23 × | 1.62 × | 1.27 × |

The c3 column is 8 seeds, and the same two agents read 2.05 × on one game: small samples swing with the
seeds. **Take 1.25-1.28 ×, the 48-seed read**, which the gate mix agrees with. At 1.27 × 055's 54-minute
price sweep is about 43 min, so a four-hour price pass gets four sweeps, not three.

## Where the two first differ

All 96 games differ, early: at about action 21 at c2 and 31 at c3, 4-5% of the way in (`bench/first_divergence.py
--cells DIR A B`). The first different action is a `pay` on both sides in 32 of 48 at c2 and 45 of 48 at c3, and a `pay`
on one side in 9 more at c2. So it is the first action phase's threat payment, the first place an
alternative leads the line. From the code, not traced game by game: both play the same first guess, so the
difference there is the bar. One guess takes any lead; 054 needs over 60, on the first deal and on the mean.

## The gate

`experiments/gate.json`, 8 seeds, `--seed 13000`: 388 of 512 games, 20.5 min on 8 threads, 0 panics. **Not
passed by the tool's reading**: five checks level, and the planner itself read 88 / 88 on those 8 seeds, so
level there is a high bar. Those five, and three at +12, went to 48 seeds on the band's seeds
(`experiments/gate-close-061-c2.json`, `-c3.json`). Gap = planner minus check, ± one SE of the paired
difference.

| check | c2 | c3 | 059 (c2 / c3) |
|---|---|---|---|
| tier-2-rush | **-10 ±8.5** | (+25 at 8 seeds) | +8 / -2 |
| always-Consolidate | (+33 at 6 seeds) | **-8 ±7.8** | +21 / -6 |
| ignore-Industry | -4 ±9.4 | (+38 at 8 seeds) | +6 / |
| no-powers | -4 ±8.9 | +15 ±8.4 | +2 / -4 |
| ignore-Science | (+50 at 6 seeds) | +8 ±8.3 | +8 / +2 |
| always-Expand | (+33 at 6 seeds) | +12 ±7.7 | |
| science-rush | (+67 at 6 seeds) | +35 ±9.2 | / +21 |

**No check is ahead beyond 2 SE, and two are ahead beyond 1 SE:** tier-2-rush at c2 and always-Consolidate
at c3. 059 had none beyond 1 SE. That is on other seeds and another planner, and the gaps sit within noise of
059's, so this is not evidence that one guess made the gate worse. It does not clear it either.

**The structural cause** (inferred from the reports, not tested by a lever). Most games are a race for the
Unity deck, and both leading checks reach its end sooner on less play. Tier-2-rush at c2: 8.42 rounds against
8.92, 20.7 cards a game. Always-Consolidate at c3: 8.94 rounds against 9.42, 37.9 cards against 42.2. A fixed
rule that keeps tempo gives up little to a two-round search, and at c2 that search adds nothing measurable
over the engine, so every simple check starts level there. It is 053's and 054's standing finding, tier 2
simply better. The fix is price, not agent.

## Commands

    aic-sim experiment experiments/band-061.json --seed 8000 --threads 8
    aic-sim experiment experiments/gate.json --seed 13000 --threads 8
    aic-sim experiment experiments/gate-close-061-c2.json --seed 8000 --threads 8
    aic-sim experiment experiments/gate-close-061-c3.json --seed 8000 --threads 8

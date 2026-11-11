# 185: Vigil cap and adversary research cubes (r79)

Built on main 69c082a (r78, 184). `RULES_REVISION` 78 → 79, bumped from the constant (the prompt also said r79).

## What changed

- **Vigil capped (Matt's ruling #1, locked: some cap).** The first troop each round that ends its march next to a
  Protector node pays 1 insight; later ones that round pay nothing (`board.vigil_cap` 1, candidate; tallied
  `board.vigil_capped`). The once-a-round marker is two board keys a seat (`board.vigil_round.S`, `board.vigil_paid.S`).
- **Adversary research cubes (ruling #2, locked: the payout stays).** Until its country's megaproject completes, an
  adversary station puts `station_cubes` (1) adversary cubes on it in production (`board.adversary_cubes`), and makes
  no compute; after completion it makes compute as before. A player ambassador on it blocks either. Completion pays
  each adversary cube the printed adversary payout (1 Sway to Discord; Polar Compute Array 1 adversary compute).
- **Ops (ruling #3), printed on no card:** `adversary_cubes {count}` (count cubes on each open megaproject) and
  `botnet_cubes {count}` (up to count botnets, players choose, each come off and become an adversary cube on their
  country's open megaproject; no Unity for the removal). Values: `adversary_cubes` −3 a cube a country,
  `botnet_cubes` +9 (the botnet off at `remove_botnets` 12, the cube on at −3; the compute price is the card's).
- **D77 (amendment 8456, candidate):** a firewall or Corrupt prohibition guards Station Coup's flip
  (`corrupt_station`, and the flip op's choice), Lab Takeover and Station Seizure (`force_flip`) and Overrun / Regime
  Change (`corrupt_city`): `adversary::guard_holds`, tallied `adversary.guard_held.{flip,force_flip,corrupt_city}`.
  A guarded station is left out of a flip's choice (the flip takes another open one if any). Rule of Law's three
  faces say "nothing corrupts or flips a site"; the rulebook's firewall sentence is back to "nothing on it corrupts".
- **182's sim-player fixes** (a helper, in a copy, then applied): the already-lost return for every agent; converge,
  energy, move and strike rules filter `ruled_out`; behaviours out of `Forced` (`Behaviors`, always wrapping);
  `strike_rule` its own switch; `guide.rs` counts protections from `board::guards` (Aegis now in the forecast; a
  base's guard still never fires there: the forecast models no march).

## Give / get

No card added or repriced. The two ops are priced in `values.json` above; neither is printed. Rule of Law's faces
changed words only.

## Smoke (dev seeds 36800-36815, d0-d4, three pairs, 240 games a level a side; before the 182 fixes)

`m185.py out/smoke79s out/smoke78s` (strategist), the same for greedy. A game, mean over the three pairs:

| | Vigil insight r78 → r79 | Vigil capped (r79) | adv cubes landed (r79) | adv cubes paid (r79) |
|---|---|---|---|---|
| strategist d0-d4 | 25.3 / 47.7 / 68.3 / 70.8 / 79.5 → 4.4 / 4.3 / 4.0 / 3.9 / 4.0 | 21-62 | 4.6 / 7.2 / 8.6 / 8.3 / 8.7 | 4.6 / 7.0 / 8.4 / 7.9 / 8.7 |
| greedy d0-d4 | 23.4 / 54.6 / 91.9 / 82.5 / 79.6 → 4.9 / 4.4 / 4.4 / 4.2 / 4.4 | 21-76 | 3.7 / 6.8 / 8.0 / 7.8 / 7.8 | 3.7 / 6.5 / 7.8 / 7.5 / 7.7 |

r78 landed and paid 0 adversary cubes. The D77 guards fired at most 0.08 a game. 182's fixes, smoked by the helper
(HEAD r78 against HEAD + fixes, d2, 48 games an agent): greedy 66.7 → 64.6, strategist 62.5 → 62.5, 47 of 48 games
identical; `greedy:plan_keep=0` 60.4 → 52.1 (now wrapped by the behaviours; 5 of 48 identical). Per decision
1.00-1.02×. `loop_smoke.py` on the r79 smoke: no timeouts, no card at the play flag (most: 9).

## The fresh ladder

[fresh-ladder.md](fresh-ladder.md): the strategist on 144 fresh seeds (43039-43182: 432 games a level), greedy on 48
(144 a level), d0-d4, the three pairs, r79 (the final build, 3f2d5ca) against r78 on the same seeds, paired.

| d | strategist r78 → r79 | gap ± SE | band | greedy r78 → r79 | gap ± SE |
|---|---|---|---|---|---|
| 0 | 92.4 → 93.5 | +1.2 ± 1.7 | 90-100, met | 94.4 → 89.6 | −4.9 ± 3.2 |
| 1 | 79.6 → 75.5 | −4.2 ± 2.5 | 70-80, met | 78.5 → 82.6 | +4.2 ± 4.6 |
| 2 | 68.8 → 56.2 | −12.5 ± 2.6 | 55-65, met | 67.4 → 61.8 | −5.6 ± 4.8 |
| 3 | 60.6 → 50.7 | −10.0 ± 2.6 | 40-50, **over by 0.7** | 66.7 → 49.3 | −17.4 ± 4.9 |
| 4 | 56.0 → 46.3 | −9.7 ± 2.7 | below d3, met | 58.3 → 47.2 | −11.1 ± 4.7 |

**The pairs.** d3's 0.7 over (well inside its SE of 2.4) is carried by pr 53.5 and br 51.4; bp reads 47.2. The drop
is the Protector's: strategist bp 74.3 → 48.6 and pr 74.3 → 59.0 at d2, against br 57.6 → 61.1. At d2 the bp pair now
reads under br (48.6 against 61.1), the reverse of r78: the cap at 1 more than takes back what 178's ruled Vigil gave.

**Measures (strategist, a game, d0-d4):** Vigil insight 21.9 / 40.4 / 73.9 / 69.5 / 74.8 → 4.2 / 4.4 / 4.3 / 4.1 / 4.1
(bp 7, pr 5.5 a game; capped firings 20-72 a game); adversary cubes landed 4.6 / 7.8 / 8.5 / 8.4 / 8.4 and paid
4.5-8.4 (r78: 0); station compute 9-20 → 4-11 a game; megaprojects completed 1.2-1.8 → 1.9-1.96 a game (the adversary
cubes fill them sooner). **D77 guards** (amendment 8456): Station Coup's flip held 0.00-0.05 a game, a forced flip
0.00-0.01, Overrun 0 (no Overrun met a firewalled or prohibited city in 2,160 games).

**The cap as a lever** (`f185-vigil`, `vig185.py`: caps 2 and 10 on r79, the ladder's first 48 seeds, spent seeds,
so a screen, not a confirmation). br is identical at every cap (Vigil is the only change). The Protector pairs, d0-d4:

| cap | bp+pr win % | Vigil insight a game |
|---|---|---|
| 1 (r79) | 91.7 / 72.9 / 49.0 / 43.8 / 39.6 | 6.1-6.7 |
| 2 | 95.8 / 80.2 / 64.6 / 45.8 / 40.6 | 12.6-12.9 |
| 10 | 89.6 / 75.0 / 62.5 / 58.3 / 53.1 | 36-51 |

Pooled with br (56.2 / 54.2 at d2 / d3 on these seeds), cap 2 reads about 95 / 78 / 62 / 49 / 40: every band met, and
the Protector pairs no longer under br at d2. Cap 10 still caps (r78 paid ~100 a game in these pairs); even so its
Protector pairs read 6-12 under r78's at d2-d3, so part of the drop is not Vigil's (the adversary cubes, D77 or the
agent fixes; not separated). Recalibrating is a separate item: no dial moved here.

**Two levers in one row.** The r79 build carries the three rulings, D77 and 182's agent fixes; only the fixes were
isolated (the helper's smoke: 47 of 48 games identical for greedy and the strategist). The pair pattern (only the
Protector pairs moved) puts the drop on Vigil, but the adversary-cube harm (about 7 Discord or compute a game at d2-d4)
is not separated from it.

## Eval

The eval already charged an adversary cube on an open megaproject at half a player cube's credit
(`eval.rs`, `w.project_cube`, `0.5 * theirs`), inert until r79; it now reads real cubes, so no new term was added
and none screened. Vigil's keep value (`values.json` `rates.fires_wall` 0.5 a round) now sits near the measured
~0.6-0.8 firings a round in a Protector game (r78's was ~10).

## Checks

Fast lane and `cargo test-all` green, clippy clean, `aic-sim docs --check` current, `npm run cards:fit` passes (no
player card's face changed), `loop_smoke.py` (the r79 dev smoke) no timeouts. Speed, `per_decision.py` r78 against
r79, 16 games a side, one thread, with a farm running beside (ratios only): ms a decision 0.86× greedy, 0.96× the
strategist (the cap trims Vigil's trigger work). `speed.py check` gives no verdict across a revision.

## Files

`m185.py` (the measures, old against new on the same seeds). Specs `experiments/farm/f185-strategist.json`,
`f185-greedy.json`. Seeds 43039-43182 spent (ledger next free 43183).

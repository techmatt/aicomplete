# 090: the r35 balance pass

Rules revision 35, bumped from the constant (34 to 35; the prompt also said 35). A ballpark read under the
*Simulation scale* ruling: 2 players, the pairs B+P, B+R and P+R, d2, the strategist (`engine:strategy=1`) as judge,
48 seeds a cell, target band 45-85, no gate.

- **Seeds:** 31100 the fresh read (and the r34 binary on the same seeds), 31200 the 16-seed engine smoke, 31300 the
  on/off and liberate-price screens.
- **Specs:** `experiments/screen-090.json`, `screen-090-d0.json`, `use-090.json`, `screen-090-onoff.json`,
  `screen-090-lib.json`.
- **Read with** `scripts/cards/energy_read.py` and `scripts/bench/paired.py`.
- **Full tables:** [d2-fresh.md](d2-fresh.md), [d0.md](d0.md), [onoff.md](onoff.md).
- **Build:** every number below is from the final build, except the liberate-price lever, which ran one build earlier
  (before four card-fit edits: Substation Hub and Treaty Grid Exchange lost their credit income, Tidal Barrage lost its
  gift, The Nuclear Option lost its energy cost).

## The calibration, reasoned before building

**Energy is rare.**

- **Rate:** 2 → 4.5 credits an energy (`values.json`).
- **Pricing anchor:** +1 energy of income on a tier-1 card costs Matt's 8-10 credits; +2 is tier 2; no flat income
  above +2.
- **Incomes cut:**
  - Dyson Swarm Tranche 5 → 2, Continental Supergrid 4 → 2, Positron Storage Ring 3 → 1.
  - Baseload Contract and Peaker Turbines 2 → 1, Hardened Bunker Complex 2 → 1.
  - Tidal Barrage: an energy per Power tag → per 2 Power tags.
  - Gifts of energy: Interconnector Treaty 6 → 3, Brownout Relief 3 → 2, Joint Security Compact 4 → 2.
- **Where it matters:** energy has a use every round, since node powers and track steps spend it. The sinks below add
  uses that don't compete with nodes.

**Energy counters.**

- Blackout, Sabotage and Commandeer (the grid's own harms) now take 1 credit + 1 energy, where they took 4, 5 and 4
  credits: about the same weight at 4.5 an energy.
- Threats land every round, so the sink matters all game.
- Loop: none. Energy is spent for good. Cap: the threats dealt.

**Energy-cost cards.** Kinetic Interdiction and Orbital Mass Driver cost 2 energy: one-shot payments, so no loop.

**The energy gift.**

- Substation Hub gives another player 2 chips, Treaty Grid Exchange 1 insight; each pays 1 energy, once a round.
- Loop: credits → neutral city energy (2 credits) → gift. The 2 chips sell for 2 and the insight expires with the
  round, so the loop never gains, and nothing turns the gift back into energy.
- Cap: once a round a building, and the neutral cities next to a seat's nodes.
- How the agents read it: the table puts a use at about +0 (`other_gain` at 0.75); the search puts it at about +3
  (the team's view counts the other seat in full).

**Stations.**

- **The action:** *Free a station*. Stand 2 of your ambassadors on a corrupted station with no adversary ambassador
  there, and pay 6 credits. Both ambassadors are used up and the station becomes neutral.
- **Worth against cost:** a freed station is a flip, a cube a round both ways over the 4-5 rounds left at a typical
  round 5-7, about 15 (`cleanse_station`). It costs about 2 × 4 + 6 = 14.
- **The defense:** a player's station flips only once 2 adversary ambassadors and no player ambassador stand on it.
- **Loop:** none; the ambassadors are consumed.
- **Agents:** the search now charges a corrupted station `station + station_flip` (5 + 20), and ambassadors gathering
  on it earn a share that grows as they arrive. At the default weights, freeing nets about +2.75.

**Capstones.** Each liberates and adds its engine's twist; each matters from the mid-game, when a corrupted city is
in reach.

- Diplomacy, Hostile Buyout: your cube on the city, and you gain 1 ambassador.
- Security, Peacekeeping Mission: up to 3 troops off in the city's country, and no cube.
- Cyber, Root Access: a botnet off, on or next to the city.

**Popular Uprising** liberates once, after 2 Unity tips, then leaves play with its stack. Unity's second tip comes in
the mid-to-late game, so it matters from about round 5.

**Dear powers.** Rush Build, Shelter and Particle Run go from 2 tags to 1 tag + 1 energy: energy is the resource every
seat now lacks.

**The income horizon.**

- `horizon_t1` 4 and `horizon_t2` 2.5 rounds (`Ctx::typical_for`).
- Used for the table's card values, the opening strategy choice and the ledger.
- In a game the agents still count the real rounds left.

## Win rates

| read | B+P | B+R | P+R |
|---|---|---|---|
| r35, strategist d2, fresh 31100 | 50 ±7 | 65 ±7 | 69 ±7 |
| r34 binary, same seeds | 71 ±7 | 81 ±6 | 77 ±6 |
| r35, greedy (engine) d0, 31100 | 77 ±6 | 81 ±6 | 90 ±4 |
| r35, strategist d0, 31100 | 88 ±5 | 83 ±5 | 90 ±4 |
| r34 greedy d0 (082, 30700) | 79 | 98 | 83 |

- Every pair is inside 45-85, and no adversary lever was needed.
- **The win mix moved.** Purge wins fell (B+P 18 → 4 of 48) and node losses rose (1-3 → 4-10 of 48).
- **Seats place fewer nodes:** 11.0 → 9.3, 10.6 → 9.0 and 9.4 → 8.6 a seat, the cost of scarce energy.

**The liberate family at its corrected value** (`screen-090-lib.json`, 31300) was tested against the prices with the
twists alone:

| card | at value | twists alone |
|---|---|---|
| Hostile Buyout | 47 | 34 |
| Peacekeeping Mission | 45 | 23 |
| Root Access | 40 | 23 |
| The Nuclear Option | 15, now 19 without its energy | 12 |
| Power to the People | 25 | 12 |
| Forensic Audit | 27 | 10 |
| Popular Uprising | 19 | 14 |

The gap was +2.1 ±4.7 / −10.4 ±6.1 / −12.5 ±4.8, pooled −6.9 ±3.1. Every pair stayed in band (67 / 67 / 69), so the
prices at value were kept.

## Energy held per seat (as the action phase ends), against the target

| | round 3 | round 6 | last round |
|---|---|---|---|
| r35 fresh d2 (B+P / B+R / P+R) | 0.7 / 1.3 / 1.3 | 1.5 / 1.6 / 1.5 | 1.8 / 2.6 / 2.1 |
| r34, same seeds (end only, `results.json`) | | | 7.3-7.5 / 10.7-11.4 / 6.2-12.4 |

The target is about 1-3 mid-game and near 0 at the end. Mid-game is on target. The end is 1.8-2.6 a seat, down from
6-12 but not yet near 0.

## Use per game, on and off

On 31300, 48 seeds a pair. "Off − on" is the paired win gap, pooled over 144 seeds.

| mechanic | use a game (B+P / B+R / P+R) | off − on |
|---|---|---|
| energy counters (energy paid) | 1.17 / 0.94 / 1.08 | +2.1 ±4.5 |
| energy-cost cards (all, played) | 0.38 / 0.56 / 0.52 | +1.4 ±1.7 |
| energy gift Actions | 0.10 / 0.17 / 0.00 | +0.7 ±1.8 |
| stations freed | 0.06 / 0.02 / 0.02 | +0.7 ±0.7 |
| player stations a flip couldn't take | 0.04 / 0.04 / 0.06 | −0.7 ±0.7 |
| Hostile Buyout, Peacekeeping, Root Access played | 0.38, 0.38, 0.23 / 0.29, 0.27, 0.33 / 0.21, 0.33, 0.23 | twists off −7.6 ±3.3 |
| twists: ambassadors, troops, botnets | 0.38, 0.31, 0.15 / 0.29, 0.04, 0.10 / 0.21, 0.29, 0.15 | (same) |
| Popular Uprising's liberation | 0.12 / 0.10 / 0.10 | +0.7 ±0.7 (r34's repeatable, ungated) |
| Rush Build | 0.15 / 0.10 / – | r34 costs −6.9 ±3.5 (all three) |
| Shelter | 0 / – / 0 | (same) |
| Particle Run | – / 0.56 / 0.44 | (same) |

**Near 0, and why:**

- **Shelter (0).** Its value depends on a destruction that round (0.7 × 4 at the table), which is less than the
  energy (4.5) plus the Security card it sacrifices. A search one action deep never sees the destruction it would
  stop. Its cost was dropped as asked, and it was not pushed further.
- **The energy gift (0-0.17).** Substation Hub and Treaty Grid Exchange are played in about 0.02-0.06 games. A +1
  energy building at 9 credits loses a scarce slot to stronger buildings. Played, Substation Hub's gift is used about
  twice a game.
- **Stations freed (0.02-0.06) and the station defense (0.04-0.06 held).**
  - Two of a seat's ambassadors on one corrupted station is rare: ambassadors cost 20 credits, and the liberation
    fallback and evictions want them more.
  - Reclamation Crew, a card, frees 0.25-0.40 a game instead.
  - No flip effect took a player's station; it held the 2-6 times one tried.
- **Popular Uprising's gate.** It liberates late either way: three Actions of Followers put its liberation after
  Unity's second tip in most games, so the gate and the one-shot rule changed no result.

## The ledger (r35)

See `experiments/results/effect-ledger/outliers.md`. The r34 list of under-value buildings is gone under the corrected
values and the tier horizon. A tier-2 income now counts 2.5 rounds, so Arcology Foundry, Orbital Embassy and Arc
Furnace Complex read at or over value and were left as printed.

Outliers left:

- **Great Liberation:** printed 33 against a reasoned 108, three liberations at 45.
- **Total Containment and Kinetic Interdiction:** "up to N troops" is counted at N.
- **Beamed Power Constellation:** a one-shot burst of energy per node.
- **Energy income:** priced at Matt's 8-10 anchor, against the table's 18 for a tier-1 card.

## Card fit

Popular Uprising's Action prints at 5.4 pt, under the guideline's 7.5. Both the gate and the one-shot discard must be
printed, and neither has an icon. The reason is in `docs/card-pricing-notes.json`.

## The card-id rename (P1)

`node_severance` → `botnet_takedown` and `node_swarm` → `botnet_swarm` change the cards' sort order, and card-id order
feeds play (087). The rename is part of this revision's play change and is not measured separately.

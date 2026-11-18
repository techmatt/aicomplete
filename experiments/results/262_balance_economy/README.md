# 262: balance pass 1, the economy (r123)

Matt's rulings (2026-11-17, locked): cut the Claim standard action (cards claim); strengthen Tidal Grid's converters
(the chip one "2 chips → 3 energy") and any other board or corp converter used under its band, paying out in a
resource that binds; Arms Inspectors gets 259's stronger version; Synthetic Data Forge gets a stronger effect.
Amendment 11997 (staging defaults, all standing): Tidal Grid's cube converter "3 research cubes → 5 energy"; Make
antimatter keeps its 5 cubes and gains 2 energy; Arms Inspectors removes 2 when built too; the Forge's smaller move
that clears `values.json`; Bulwark left alone; `claim_station` / `claim_factory` keep their names as Liberate's.
Amendment 12073 (Matt, D107 and D113 from 261's list): a hosted city can't be bought, and a purchase no longer
unhosts a city; Counterintelligence Sweep and Fusion Commons cost 1 insight with fewer credits. Bumped from the
constant, 122 to 123 (main had 261's r122).

## What changed

- **Claim cut.** `StandardAction::Claim`, its text, its design_check rows and jobs, `STANDARD_NAMES`, the agents'
  references and `board.claim_city` (read by Claim alone) are gone; the payment helpers are Liberate's only. Cards
  claim exactly as before (Power Purchase Agreement, Microgrid Lab Compact, Campus, Commons, the liberate rider).
  `board.std_off` places after Ambassador moved up one.
- **Horizon pays energy:** Tidal Grid 3 cubes → 5 energy (was 3 cubes + 1 energy → 1 antimatter); 2 chips → 3 energy
  (was 1 antimatter). **Make antimatter** 5 cubes → 1 antimatter and `board.antimatter_energy` 2 energy.
- **Arms Inspectors** removes 2 research cubes when built, Action unchanged. **Synthetic Data Forge**'s cash-out also
  draws 1.
- **D107:** Buy energy refuses a city this round's hosting matching hosts (`board::for_sale`); `host_assignment` no
  longer drops a bought city. **D113:** Sweep 1 cr, 1 insight (was 5, 0); Fusion Commons 0 cr, 1 insight (was 4, 0).
- **Agents (`convert_rule`, on):** instead of a pass, a megaproject converter whose gains are all kept resources
  worth more than its cost at `values.json`. The eval holds a chip at 4 and an energy at 0.8, so with Cintamani's chip
  converter switched off Horizon's chip converter was offered 3.0 times a game at production and taken 0 times.

## Give / get (before the first screen)

Anchors: energy 4.5, cube 6, chip 6, antimatter 18, card 5, insight 4; a research cube off the track 2 (259).

| Item | Pay | Get (anchors) | values.json |
|---|---|---|---|
| Tidal Grid cube converter | 3 cubes 18 | 5 energy 22.5 (1.25) | 9 → 22.5 |
| Tidal Grid chip converter | 2 chips 12 | 3 energy 13.5 (1.13) | 8 → 13.5 |
| Make antimatter (standard, a hedge) | 5 cubes 30 | 1 antimatter + 2 energy 27 (0.90, under 1.0) | 15 → 27 |
| Arms Inspectors (8 cr, 0 ins) | 12 | 2 × 2 + 4 uses × 4 = 20 (1.67) | 30 (2.5) |
| Synthetic Data Forge (10 cr) | 14 | 16 + a card 5 = 21 (1.5) | ledger 9.25 → 12.05 against printed 10; 2 datasets read 12.25, the larger move |
| Counterintelligence Sweep (1 cr, 1 ins) | 1 + 4 × 2 = 9 (as before) | 13 (1.44) | 17 |
| Fusion Commons (0 cr, 1 ins, 2 am) | 0 + 4 × 4 + 36 = 52 (as before) | 54 (1.04) | |

Loops: no converter makes its own fuel; Horizon's are once a round each.

## Reads

Specs: `s262-dev2.spec.json` (dev seeds 36800-36831, three pairs, d0-d4, judge and greedy, `convert_rule` on and off)
and `experiments/farm/f262-read.json` (fresh, **49500-49547**, 1,440 games a side; r122, main 74fc6c8, natively on
the same seeds and its own configs), `f262-oracle.json` (49500-49511, d2-d3). Tables: [screen.md](screen.md),
[read.md](read.md), [use.md](use.md), [counters.md](counters.md), [convert-rule.md](convert-rule.md),
[sources.md](sources.md), design_check: [screen](design-check-screen.txt), [read](design-check-read.txt).

**Fresh read against r122** (paired, 144 games a cell): the judge 84.0 / 91.0 / 89.6 / 61.1 / 22.2 at d0-d4
(gaps -6.9 ± 3.9, -0.7, +1.4, **+12.5 ± 5.1**, +5.6), pooled **+2.4 ± 1.7**; greedy 89.6 / 87.5 / 81.2 / 54.2 / 11.8
(+3.5, +3.5, +7.6, +7.6, -4.2 ± 3.4), pooled **+3.6 ± 1.9**. Bands (soft): judge d0 84.0 under 90-100 (as r122's
greedy at 86.1); greedy d0 89.6 and d3 54.2 just under. Full oracle 72/72.

**`convert_rule` on/off** (dev seeds, 480 games an arm): judge 69.6 / 69.6 pooled, greedy 64.6 / 65.0: no win cost;
Horizon's chip converter 0 → 1.27 a game. Shipped on; the fresh read has it on. The insight exclusion came from the
diff review after this screen (Cintamani's cube for 2 insight would be lost at the round's end).

**Speed** ([speed.md](speed.md), `per_decision.py`, 16 games, d2): the judge 1.9 ms a decision on both (0.99×),
greedy 2.1 on both (1.00×). `speed.py check` gives no verdict across revisions (its baseline is r112).

**Browser checks** (both scripts claimed with the Claim action): `e2e:corps` now claims with Power Purchase Agreement
(the tier-1 deck that card alone, so seat 0's opening hand holds it) and checks the converter as 3 cubes → 5 energy;
`e2e:researcher` builds its Onyx station free (`station_chips` 0). Researcher passes; corps fails 4 checks (cube
colors at round 2 read 0,0; seat 0 stores 10, not 5) that main's own script fails on main's build too, beside a
fifth (its converter check, stale since r101) fixed here.

## Use (fresh read, r122 → r123, a seat-game)

| Measure | Band | r122 | r123 |
|---|---|---|---|
| Tidal Grid converter | 0.30-3.00 | 0.08 | **0.42** in |
| Tidal Grid chip converter | 0.30-3.00 | 0.00 | **0.66** in |
| Make antimatter | 0.05-0.75 | 0.02 | **0.01** low |
| Protector Bulwark (a seat-round) | 0.40-0.90 | 0.21 | 0.21 low (left alone) |
| Buy a city's energy | 0.10-1.20 | 0.44 | 0.26 |
| Gain a player ambassador | 0.05-0.75 | 0.72 | **0.87 high** (new flag) |

Spent of gained: credits 104% → 103%, energy 86% → 79%, chips 62% → 74%, **research cubes 62% → 73%**, **antimatter
51% → 51%**, ambassadors 78% → 77%. Hosted energy a game (judge) 14.2 → 16.8, bought 9.0 → 5.3 (D107). Arms
Inspectors: played 0.52 → 0.62 a game (judge), **its Action used 0.58 a game (judge), 0.38 (greedy)** (r122 0.63 /
0.40); research cubes removed 2.60 → 3.23. Synthetic Data Forge kept 0.04 → 0.09, played 0.22 → 0.30, Action 0.16 →
0.30. Sweep played 0.65 → 0.79; Fusion Commons 0.12 → 0.15. Card claims unchanged (1.02 factories, 0.19 stations);
Claim's 0.08-0.10 a game are gone.

## Misses, and why

- **Make antimatter stays under its band (0.01):** the agents take it only through `make_short` (230, amendment
  10186: never the search's pick, a card in the keep plan must need the antimatter), so 2 energy on it changes
  nothing they see. Not dialed: a standard action is a poor hedge (0.90 at the anchors).
- **Antimatter doesn't bind (51%)**: the converters stopped making it but made little (0.1 a seat-game). Supply 3.13
  a seat-game, spend 1.60. **Cubes 73%**: Horizon's converter now takes 29% of cube spend (Cintamani's 20%); still short of binding.
- **New flag, Gain a player ambassador 0.87 (dominant):** the judge's buys 1.84 → 2.32 a game (greedy 1.03 → 1.18);
  on the dev seeds it rose before D107 (1.34 → 1.60) and again with it (1.77): credits freed from Buy energy and
  from Claim's ambassadors go there. Two changes at once; not separated. Reported, not dialed.

## Supply-trim candidates (not shipped; Matt's)

From [sources.md](sources.md) (fresh read, a seat-game):
- **Antimatter, the capstones' income** (Sanctuary 1 a round, Open Archive 1 a round): 1.22, 39% of supply, with
  no spend attached. A lump on reaching the capstone (say 3) instead of a stream: about -0.6. Scarce and earned.
- **Antimatter, the plants' when-built 1** (Penning Trap Array 0.17, Antiproton Foundry 0.15): the plant is the
  steady source; the when-built 1 doubles it early: -0.3.
- **Cubes, Collider Consortium's 8** (0.68, 12%) to 5: -0.25; **Field Grant's 3** (0.53) to 2: -0.18.

## Open for Matt

- **D107, which city is "hosted" at purchase:** the engine reads this round's matching when the buy is made, and a
  city bought before anyone hosted it may still be hosted in the income step (both paid that round; no marker, per
  the ruling). The once-per-city-a-round buy limit still needs a memory of what was bought.
- **Gain a player ambassador** above its hedge band: a price (it reads 0.40 at the anchors) or the agents' idle-credit
  use.

## For new players

There is no Claim action any more: a neutral research station or factory becomes yours only through a card. Horizon,
the energy corp, now turns 3 research cubes into 5 energy and 2 chips into 3 energy, each once a round with a Horizon
city in your reach. Make antimatter (5 cubes) also gives 2 energy. Arms Inspectors takes 2 cubes off the adversary's
research track when built, and again with its Action each round. Synthetic Data Forge's cash-out draws you a card.
You can't buy the energy of a city someone hosts. Counterintelligence Sweep and Fusion Commons each cost 1 insight
to play, and fewer credits. The rule that resisted: Make antimatter is still almost never worth it.

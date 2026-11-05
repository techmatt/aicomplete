# 125: the card pass, priced

Overlay: [overlay.txt](overlay.txt) (`python scripts/cards/apply_overlay.py experiments/results/125_energy_sinks/overlay.txt`,
rerunnable from r49's content). Scenes from the unused pool: Particle Accelerator `antiproton_flask`, Citywide Microgrid
`shared_rooftop_grid`.

## Which cards, and the tag

A building gets energy when a reader of its title would say it needs a lot of power. That's mostly the Builder's
heavy machinery: **Industry and Power are the usual consumers**. AI compute contributes one (Foundation Model Campus).
I passed over Simulation Cluster, Hyperscale Botnet, Synthetic Data Forge and the Power engines that *make* energy
(Substation Hub, Dispatch Control Room, Smart Grid Backbone), to keep the share small and the Researcher's load light.

Energy-cost cards in the deck (151 cards; 9 print an energy cost, 7 of them among the 54 buildings), plus 2 Actions that
spend energy:

| | tier 1 | tier 2 |
|---|---|---|
| play cost | Fabrication Megaplex 2, Lights-Out Assembly 1, Cryptojacking Campaign 2 (program), Cryo-Compute Burn 1 (event) | Orbital Mass Driver 3, Foundation Model Campus 2, Positron Storage Ring 2, Power to the People 4, Citywide Microgrid 8 |
| Action | Grid Analytics Desk 1, Particle Accelerator 2 | |

By tag (a card with an energy play cost counts once under each of its tags): Power 5, Industry 4, Science 2,
Cyber 2, Diplomacy 1, Security 0. Both energy Actions are on Power cards.

## Give / get at `configs/agents/values.json`

Energy is 4.5, insight 2.5, the keep 2.5 (tier 1) or 6.5 (tier 2), a draw 4, Sway 4, a project cube 6, a
liberation 45. "Get" is the effect ledger's `reasoned` value (`scripts/cards/effect_ledger.py`).

| card | give | get | net |
|---|---|---|---|
| Fabrication Megaplex | 25 + 2 energy 9 + play 2.5 + keep 2.5 = 39 | 47 | +8 (r49: 39, the same) |
| Lights-Out Assembly | 10 + 4.5 + 2.5 + 2.5 = 19.5 | 27 | +7.5 |
| Orbital Mass Driver | 15 + 2 chips 3 + 13.5 + 5 + 6.5 = 43 | 64.75 | +21.75 |
| Foundation Model Campus | 13 + 9 + 5 + 6.5 = 33.5 | 40 | +6.5 |
| Positron Storage Ring | 9 + 9 + 5 + 6.5 = 29.5 | 28.6 | −0.9 (r49 the same: an engine, priced at its seat's fire rate) |
| Particle Accelerator | 16 + 2.5 + 2.5 = 21; a use: 2 energy (9) for 3 cubes (18), +9 | 26.7 | +5.7 |
| Citywide Microgrid | 4 + 8 energy 36 + 5 + 6.5 = 51.5 | 45 + 2 Sway (8) × 2.5 × 0.7 + tags = 60.5 | +9 |
| Moonshot Grant (unchanged) | 46 + 2.5 + 2.5 = 51 | 57 | +6 |

A first draft of the Action sink, Overclocked Data Center ("pay 2 energy: gain 4 insight", 9 credits), read +1 a use at
the table and 4.3 in all: a loss for the agents. It had no fix inside the anchor ceiling, because the anchors price insight
at 4 and the table at 2.5. Particle Accelerator replaced it: cubes are worth the same at both.

## The anchor check (pay = credits + 4 × (keep + play insight) + energy 4.5 + chips 3)

| card | tier | was (r49) | now | pay | get | ratio | when it matters |
|---|---|---|---|---|---|---|---|
| Fabrication Megaplex | 1 | 34 cr, 1 ins | 25 cr, 2 energy, 1 ins | 42 | 53 | 1.26 | rounds 2-3, a jump-start a round later (start holds 1 energy) |
| Lights-Out Assembly | 1 | 14 cr, 1 ins | 10 cr, 1 energy, 1 ins | 22.5 | 32.5 | 1.44 | rounds 2-5, the 10+ credit seat |
| Orbital Mass Driver | 2 | 28 cr, 2 chips, 2 ins | 15 cr, 2 chips, 3 energy, 2 ins | 50.5 | 83.75 | 1.66 | rounds 4-6, the Builder (needs 3 Industry) |
| Foundation Model Campus | 2 | 22 cr, 2 ins | 13 cr, 2 energy, 2 ins | 38 | 58 | 1.53 | rounds 4-6, the Researcher (needs 3 Science) |
| Positron Storage Ring | 2 | 18 cr, 2 ins | 9 cr, 2 energy, 2 ins | 34 | 65 | 1.91 | rounds 3-6, a Power seat; its first Power card pays the 2 back |
| Particle Accelerator | 1 | new | 16 cr, 1 ins; Action: 2 energy → 3 cubes | 24 | 26.7 (6 a cube) to 35 (7) | 1.11-1.46 | from round 4, a seat with energy spare after its nodes |
| Citywide Microgrid | 2 | new | 4 cr, 8 energy, 2 ins: liberate; Action: 2 Sway | 56 | 60.5 | 1.08 | round 5 on, a seat holding 8 (the Builder peaks at 8.3-8.5) |
| Moonshot Grant | 1 | 46 cr, 1 ins | unchanged (Matt, message 5254) | 54 | 82 | 1.52 | late, needs 3 Science; offer-bound, not price-bound |

Every re-cost keeps pay within a credit of r49's, so each ratio stands where it stood; the five were inside their
tier's maximum before and still are. Get for the five is their r49 pricing note's. Particle Accelerator's get is a use's
margin × 4 rounds × 0.7 + 1.5 for tags; its uses complete a megaproject now and then (9 a cube), so it reads 1.1 at 6
a cube and 1.46 at 7.

## Card fit

`npm run cards:fit`: the seven 125 cards fit with nothing overflowing. Particle Accelerator's Action box sets at 6.3 pt,
below the 7.5 goal, as five other cards' boxes already do (Retained Earnings, Intrusion Lab, ...). The cause is
`project_cube`'s face text ("add 3 research cubes in your color to a megaproject that isn't complete"), which Field
Grant and Moonshot Grant share; this is the recorded exception. The faces were checked in the viewer's large view at 3×
(D48: Great Liberation's city, Field Grant's `/ station` and Hostile Buyout's `→ cube` all show; D51: Total War's face
now says "one a base while a base has none").

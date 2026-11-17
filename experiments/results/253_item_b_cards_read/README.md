# 253: Item B, part 2: neutral-site cards and the full read

Base: main d67e8db (252, r113). **Rules change: r114** (bumped from the constant, 113 to 114). Matt's ruling 3
(2026-11-16, locked: "some player and adversary cards place neutral stations or factories") and amendment 11525.

Tables: [read.md](read.md) (the fresh read: r112, r113, r114 and r114 with ruling 4 off), [r4off.md](r4off.md) (ruling 4
off paired with r114), [r113-vs-r114.md](r113-vs-r114.md)
(this item alone), [oracle.md](oracle.md) (the ceiling), [screen.md](screen.md) (the dev-seed screen),
[design-check.txt](design-check.txt). Reader: `read253.py` (its docstring). Specs: `experiments/farm/s253-screen.json`,
`f253-read.json`, `f253-r4off.json`, `f253-oracle.json`.

## The changes (every number a candidate)

| Change | Give / get at values.json (anchors) |
|---|---|
| **Open Research Campus** (tier 1 event, 5 cr, Science + Diplomacy): target an empty printed station site in reach; a neutral research station opens on it; place 1 ambassador on it | pay 5 + keep 2.5 = 7.5; get `open_station` 5 + an ambassador placed 6 = 11: **1.47** (anchors 13 for 9: 1.44) |
| **Industrial Commons** (tier 1 event, 4 cr, Industry + Diplomacy): the same with a neutral factory on a free industrial zone | pay 4 + 2.5 = 6.5; get `open_factory` 3 + 6 = 9: **1.38** (anchors 11 for 8: 1.38). 3 cr read 1.64, over guideline 22 (diff review) |
| **Shell Labs** (adversary, stage II, a seventh card): Rally stations; each country a neutral station opens on its lowest-numbered empty station site; +1 agent income; Corrupt stations | harm 38 at values.json against the players' cheapest answer 30: 1.27 (Station Raids 1.00, Mobilization 1.55) |
| `create_site` after a card's target opens one site there (a map op); no site opens on Fallout or a node | |
| **Retake** (amendment 2): Liberate leaves a Liberated marker; the adversary needs `adversary.retake_agents` 1 more agent to corrupt that city, station or factory (off when claimed, retaken or destroyed) | the ruling 4 line "the adversary must commit resources to retake a neutral site" |
| **Cube converter** (amendment 3): 1 cube an insight, once a round (`insight_cubes` 1, `insight_per_round` 1; was 2 cubes, no limit) | 4 for 6 at the anchors: 0.67, 2.47× the median card's price per insight: under the cards |
| `board.liberate_neutral` (1): ruling 4's switch, for screens only | |

**Where they matter.** A neutral station opened next to you puts a neutral cube a round on its country's megaproject
(completion sooner: your cubes there pay, the converter opens) and is a claim target whose first ambassador is already
yours (Claim's 4 pay from the site first; Power Purchase Agreement claims it for 2 energy). A neutral factory next to
you takes a free zone the adversary would build on, feeds its cities' growth and, with your ambassador on it, arms no
base at a Discord tip. Both matter in rounds 2-6, when claims still pay back. **Caps:** each is an event (one use);
printed sites, 1-2 a country, and free zones bound them. **Sim players play them:** the judge 0.12 and 0.16 a game,
greedy 0.14 and 0.29; neither is kept-and-never-played (`play_read.py`), no loop (`loop_smoke.py --run` on the final
build: 96 games, no timeout, no card near the play flag). **Liberated marker** is a printed token (the web board draws it).

## The screen (dev seeds 36800-36831, the judge, 384 games an arm, 252's dials in every arm)

| arm | all | d3 | gap ± SE | |
|---|---|---|---|---|
| base | 78.4 | 50.0 | | |
| ruling 4 off | 88.3 | 70.8 | +9.9 ± 2.1 (d3 +20.8 ± 5.5) | d3 losses ending on a lost node 28 → 15 of 96 |
| retake +1 | 80.7 | 53.1 | +2.3 ± 1.5 (every level ≥ 0) | retakes 1.30 a game at +0 (of 5.8 liberations), 0.81 at +1 (of 5.6): frequent |
| converter 1:1 | 79.7 | 52.1 | +1.3 ± 1.4 | cubes spent 56% → 61%; uses 0.07 → 0.83 a seat-game |
| new cards blanked | 79.7 | 50.0 | +1.3 ± 2.2 | |

**Adopted:** retake +1 (frequent, helps at every level) and the converter at 1:1 once a round (amendment 3: neither
arm left cubes binding; 1:1 moved them toward it and stays under the cards). **Caveat:** the screen's converter arm
ran 1:1 with no limit (the once-a-round limit wasn't enforced until a test caught it after the screen); at 0.83 uses
a seat-game the limit rarely binds, and the fresh read measures the shipped shape. The two levers were adopted
together from one-at-a-time arms; the fresh read measures them together.

## The fresh read (48780-48827, three pairs, d0-d4, 48 seeds a cell; r112 and r113 natively on the same seeds)

| judge | d0 | d1 | d2 | d3 | d4 | all | gap to r112 |
|---|---|---|---|---|---|---|---|
| r112 | 91.7 | 94.4 | 88.9 | 63.9 | 43.8 | 76.5 | |
| r113 (252) | 93.1 | 93.1 | 82.6 | 54.9 | 22.9 | 69.3 | -7.2 ± 1.8 |
| **r114** | 97.2 | 91.7 | 86.8 | 54.9 | 20.8 | 70.3 | -6.2 ± 2.0 (r113 → r114: +1.0 ± 2.0) |
| r114, ruling 4 off | 96.5 | 91.7 | 86.1 | 60.4 | 29.2 | 72.8 | -3.8 ± 1.9 (vs r114 +2.5 ± 1.4) |
| greedy r112 → r114 | 93.1 → 84.0 | 88.9 → 79.9 | 83.3 → 77.1 | 63.9 → 45.1 | 39.6 → 15.3 | 73.8 → 60.3 | -13.5 ± 2.2 |

Bands (judge, soft): d0 ≥ 90 met, d1 80-95 met, d2 75-90 met, **d3 55-70 missed by 0.1** (54.9). **d4 fell 23 points
and greedy 13.5:** the drop is 252's (r113 already -20.8 at d4); this item's changes are +1.0 ± 2.0 on r113. Of
the judge's d4 drop, ruling 4 (free then claim) gives back +8.3 ± 3.9 when off, at d3 +5.6 ± 3.4; the rest comes
with ruling 1 (Place a node cut; the routes place nodes earlier and fewer late, below).

**d3's drop, split (amendment 1):** d3 losses 52 → 65 of 144; ending on a lost node 18 → 40, on an empty deck 33 → 20.
Ruling 4 off: 28 node-ended. So ruling 4 accounts for 12 of the 22 extra node-loss endings, and the other 10 come with
252's node routes. Owned sites: r112 owned every liberated site (about 7.3 a game) plus 1.4 claims; r114 owns 0.5
(claims) and leaves 5.2 liberated sites a game neutral, of which the adversary retakes 0.84.

**Nodes by round (seat-game, judge, r112 → r114):** every board places nodes through round 8: Builder 0.38 → 0.35,
Protector 0.46 → 0.48, Researcher 0.40 → 0.22 at round 8. Round 2 rises on every board (1.24-1.46 → 2.25-2.58) and
round 6 falls on every board (the Researcher 1.04 → 0.55; its round 5 still rises, 1.33 → 1.52); round 1 rises for the Builder alone, and the
Protector's rounds 7-8 hold (0.68 → 0.75, 0.46 → 0.48). By source: **base route (Expand) 46%, board routes 47%, cards 7%**
(r112: board powers 47%, Place a node 38%, cards 14%).

**Standard actions beyond Dispatch and Liberate:** judge 14.5 → **6.95** a seat-game, greedy 13.3 → 6.1 (248's 14).
Converter 3.73 → 0.42, Place a node 3.84 → 0, station 1.55 → 0.23; Buy energy 2.18 → 3.36 (liberated cities sell
their energy). **Wins through cards:** every won game has more card plays and uses than standard actions (100%, as
r112); card share of the actions in won games 72% → 85%.

**Liberate and Claim (amendment 5; the table unchanged):** Liberate 3.11 → 2.24 a seat-game, Claim 0.26 → 0.04.
Liberate doesn't starve (cities liberated 3.89 → 3.39 a game); Claim nearly vanished (claims of liberated sites 0.05 a
game), and Claim a site reads unused in design_check.

**Resource use (judge, spent of gained, r114):** credits 104% and energy 90% bind; insight 84% (a round's attention,
lost if unspent) and ambassadors 82% bind; chips 67% bind loosely; **research cubes 58% don't bind** (3.4 spent a
seat-game; 12.9 at r112 when Peer Review and the 1:1 uncapped converter drank them); antimatter 44% doesn't (its
design_check band 50%).

**Ceiling (full oracle, 12 seeds a level and pair, d2-d3):** 72/72, 9.96 rounds.

**Outpost and Field Survey (amendment 4): kept.** 3.48 and 3.66 a seat-game, inside their bands, 1.19 and 1.08 at the
anchors; late nodes are already the binding loss route, so a dearer route would cut them further.

## design_check (the fresh read against r112)

No price, circular-anchor or job-parity flag on any standard action. Warnings: job-no-card Build a research station
and Claim a city (**Item C**: no card owns a station or claims a city; the new cards open neutral ones);
dials-hide-design (the converter dial, the amendment's lever, beside the design files); usage: Bulwark 0.13 a
seat-round (0.40-0.90), Scrap 2.02 and Audit 2.07 (just over 2.00), Claim a site 0.03 (0.10), Tidal Grid's converters
0.11 and 0.00, antimatter spent 48%; player token moves +11%.

## Speed

`speed.py check`: no verdict across revisions (box loaded: greedy 1.62×, planner 1.48×). `per_decision.py`, r113 →
r114, 6 games, one thread: greedy 1.34× a decision at 1.21× the options (1.11× an option: the new cards and the
converter's offers), the judge 0.68×. Ratios only; the box was loaded.

## Seeds and outputs

Fresh 48780-48827 taken (ledger next free 48828). Spent dev seeds 36800-36831 re-used for the screen. Outputs kept:
`out/keep/253/` (fresh, fresh-r4off, fresh-oracle, screen, r112-games.jsonl, r113-games.jsonl).

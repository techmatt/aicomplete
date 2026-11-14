# 219: antimatter sources by board, converter plants (r97)

`RULES_REVISION` 96 → 97, bumped from the constant (the prompt said r95; 215 had moved it to 96). Built on main
d36f7f3 (218, r96). Old side: r96 from its own tree (`git archive` of d36f7f3, `out/r96src`), natively, on the fresh
seeds. Overlay: [overlay.txt](overlay.txt) (`scripts/cards/apply_overlay.py`). Specs: [spec.py](spec.py). Reads:
[am_read.py](am_read.py) (antimatter gained, spent and by source; burns by board; plants by plant; games with a plant
built), [split_read.py](split_read.py) (a lever read into one folder per lever).

## What changed

- **The board powers are rare burns** (Matt's ruling), each once a round for 1 antimatter, a new op `burn`
  (destroy one of your own factories or stations, your choice, no protection asked; `blight` puts land corruption on
  the hex):
  - Builder **Teardown** (was Refine, 2 chips + 1 energy): 1 chip, one of your own factories, land corruption left.
  - Researcher **Overload** (was Particle Run, a Power card + 1 energy): 2 research cubes, one of your own stations.
  - Protector **Seize** (was Joint Facility, 12 credits + 1 energy): a Security card, 6 credits and 2 energy; frees a
    corrupted station or factory in reach (the cleanse op: its adversary cube off, the site neutral).
- **Tier-2 converter plants**, each a building's Action once a round paying one resource (amendment 9855: five,
  five different resources): Penning Trap Array 1 cube (30 → 10 cr), Antiproton Foundry 1 chip (30 → 10 cr),
  Positron Storage Ring 2 energy (18 → 10 cr; its free income of 1 is gone), new **Dark Fiber Collider** (Cyber,
  2 insight, 6 cr, draw 2) and new **Treaty Isotope Bank** (Diplomacy, 8 credits, 6 cr, an ambassador). Ambassadors,
  the Diplomacy story's resource, are no printable cost (`content.rs`), so the fifth is credits.
- **Make antimatter** 5 research cubes (was 4): the dev screen's lever (below). Its 4 ran as `mc4` in the fresh read.
- **Agents:** `am_source` tries a plant's Action before a board power; `plan_am` counts no burn or seize as income;
  `value.rs` prices a burn at `burn_factory` −10, `burn_station` −15, `burn_blight` −12 (values.json).
- **Harms ledger:** a `your burns` column (facilities destroyed, Discord sway), kept out of the adversary's gaps.

## Give / get (per use, values.json; anchors in parentheses)

| source | give | get 1 antimatter 18 | net |
|---|---|---|---|
| Penning Trap Array | 1 cube 3 (6) | 18 | +15 (+12) |
| Antiproton Foundry | 1 chip 4 (6) | 18 | +14 (+12) |
| Positron Storage Ring | 2 energy 9 (9) | 18 | +9 (+9) |
| Dark Fiber Collider | 2 insight 5 (8) | 18 | +13 (+10) |
| Treaty Isotope Bank | 8 credits | 18 | +10 |
| Teardown | 1 chip 4 + a factory 10 + land corruption 12 | 18 | −8 mid-game; +14 in the last round (nothing left to lose) |
| Overload | 2 cubes 6 + a station 15 | 18 | −3 mid-game; +12 in the last round |
| Seize | a Security card (~9) + 6 + 2 energy 9 | 18 + a cleanse 6-15 | 0 to +9 |
| Make antimatter | 5 cubes 15 (30) | 18 | +3 (−12: 0.60 at the anchors) |

The burns break CLAUDE.md's "no break-even trade" rule mid-game by design (Matt: rare burns of your own engine);
they pay when the burned piece has few rounds left. Flagged, not fixed.

## The anchor check (tier-2 keep 3: pay = credits + 4 × (keep 3 + play insight) + other costs)

| card | pay | get (engine 3.5 uses) | ratio (at 2 uses) |
|---|---|---|---|
| Penning Trap Array | 10 + 16 = 26 | 1 antimatter 18 + 12 × 3.5 = 60 | 2.31 (1.62) |
| Antiproton Foundry | 26 | 60 | 2.31 (1.62) |
| Positron Storage Ring | 10 + 20 + 2 energy 9 = 39 | draw 2 10 + 2 energy a Power play 15.75 + 9 × 3.5 = 57.25 | 1.47 |
| Dark Fiber Collider | 6 + 16 = 22 | draw 2 10 + 10 × 3.5 = 45 | 2.05 (1.36) |
| Treaty Isotope Bank | 22 | 1 ambassador 8 + 10 × 3.5 = 43 | 1.95 (1.27) |

All five inside 1.0-2.5. Cyclotron Lease (tier 1, 2 energy for 1, 22 cr) is r96's.

## Dev screen (dev seeds 36800-36815, 192 games a cell, d0-d3; SE ~3.5 on win)

On the first build (plants at 2 cubes, 2 chips, 3 energy; Treaty Isotope Bank 2 chips), `s219-lev`, a
factorial of Make antimatter's price and cheaper plant Actions (`cheap`: 1 cube, 1 chip, 2 energy):

| cell | win | antimatter made / spent a seat-game | Make antimatter | plant Actions | games with a tier-2 plant built |
|---|---|---|---|---|---|
| base (Make antimatter 4) | 56.8 | 3.96 / 1.64 | 1.54 | 0.19 | 69% |
| `mc5` | 55.7 | 3.55 / 1.46 | 0.35 | 0.19 | 67% |
| `mc6` | 56.8 | 3.49 / 1.50 | 0.22 | 0.19 | 69% |
| `cheap` | 52.1 | 5.02 / 1.76 | 1.62 | 0.89 | 73% |
| `mc6cheap` | 53.1 | 4.30 / 1.62 | 0.22 | 0.89 | 74% |

At the first costs the plants were built but their Actions hardly used (0.19 a seat-game), and Make antimatter at 4
stayed 3 times its band whatever the plants cost. Taken: `mc5` (the least change that puts it in band) with `cheap`,
not screened together (the fresh read confirms the pair). Then, before the fresh read: Overload's cubes 1 → 2 (0.70 a
Researcher seat-game in the screen, the most-used burn), and amendment 9855's distinct resources (Dark Fiber Collider
2 insight, Treaty Isotope Bank 8 credits). Its 16-seed smoke (`s219-dev2`): Overload 0.42, plants 0.71, built in 72%.

## Fresh read (46624-46719, 96 seeds × 3 pairs, d0-d4; r96 from its tree on the same seeds)

Strategist, paired with r96 on the same seeds (bands soft, 215; reported, not chased). `mc4` is the prompt's
number for Make antimatter (4 cubes) on the r97 build.

| d | r97 | r96 | r97 − r96 (paired) | `mc4` | band | r97 greedy |
|---|---|---|---|---|---|---|
| d0 | 81.9 | 86.5 | −4.5 ± 3.0 | 84.0 | ≥ 85, unmet | 82.6 |
| d1 | 62.5 | 61.1 | +1.4 ± 3.5 | 63.2 | 65-80, unmet | 69.1 |
| d2 | 49.3 | 46.9 | +2.4 ± 3.6 | 47.9 | 50-65, unmet | 53.5 |
| d3 | 39.6 | 41.0 | −1.4 ± 3.5 | 42.7 | 35-50, met | 44.1 |
| d4 | 23.3 | 21.9 | +1.4 ± 2.9 | 23.3 | - | 22.2 |

Win rates move inside the noise (no gap over 1.5 SE). Rounds 10.31 / 9.22 / 8.22 / 7.91 / 6.69 (r96 9.80 / 9.10 /
8.40 / 7.80 / 6.65). 216 of 288 seeds a level ran on wasm shards beside native (the engines play alike).

**Antimatter a seat-game** ([am-f219.txt](am-f219.txt), [am-r96.txt](am-r96.txt), [am-f219g.txt](am-f219g.txt)):

| | made | spent | spent / made | Make antimatter | plants' Actions | board powers | Horizon's converter | upkeep (capstones, income) |
|---|---|---|---|---|---|---|---|---|
| r96 | 3.73 | 1.55 | 42% | 1.36 (37%) | 0.30 (8%) | 0.41 (11%) | 0.50 (13%) | 0.95 (26%) |
| r97 | 3.62 | 1.33 | 37% | 0.38 (11%) | 0.53 (15%) | 0.26 (7%) | 1.22 (34%) | 0.92 (25%) |
| r97 `mc4` | 3.76 | 1.36 | 36% | 1.30 (35%) | 0.52 (14%) | 0.28 (7%) | 0.47 (12%) | 0.89 (24%) |
| r97 greedy | 3.80 | 1.36 | 36% | 0.39 | 0.65 (17%) | | | |

**The board powers, a seat-game of their board's seats:** r97 Teardown 0.34, Overload 0.33, Seize 0.11 (r96 Refine
0.31, Particle Run 0.51, Joint Facility 0.42). Used about once in three games (Seize once in nine), never a round
staple.

**The plants** (amendment 9855: games with one or more tier-2 plants built): **62% of games** (25% two or more;
r96 23%, 4%). By plant, played in a share of games and its Action a seat-game: Penning Trap Array 20%, 0.10;
Antiproton Foundry 24%, 0.004; Positron Storage Ring 13%, 0.13; Dark Fiber Collider 23%, 0.12; Treaty Isotope Bank
18%, 0.005; Cyclotron Lease (tier 1) 10%, 0.17. Antiproton Foundry's chip and Treaty Isotope Bank's 8 credits are
almost never paid: the agents hold chips for stations and buildings and credits for plays; both cards are played for
their on-play gain (1 antimatter, an ambassador).

**What took Make antimatter's place:** Horizon's converter (2 cubes + 1 energy for 1, a corp's megaproject
converter, once a round) went 0.50 → 1.22: at 5 cubes the standard action is dearer than it. The plants rose 0.30 →
0.53 but are not yet the steady source the prompt asked for (15% of supply against the converter's 34%); antimatter
spent fell 0.22 a seat-game, as the agents' spend was already 42% of what they made at r96.

## Resource use (fresh, a seat-game, gained / spent; [resources.py](resources.py))

| | credits | insight | energy | chips | antimatter | research cubes | card tokens | ambassadors |
|---|---|---|---|---|---|---|---|---|
| r97 | 269.1 / 283.5 | 51.9 / 43.8 | 37.9 / 27.8 | 9.1 / 5.7 | 3.6 / 1.3 | 17.3 / 11.5 | 2.7 / 2.2 | 20.5 / 15.6 |
| r96 | 259.6 / 274.8 | 50.9 / 42.9 | 37.9 / 26.8 | 8.9 / 5.7 | 3.7 / 1.5 | 17.2 / 11.6 | 2.3 / 1.8 | 19.9 / 15.2 |

Little moves but antimatter spent (1.5 → 1.3) and energy spent (+1.0, the energy plants).

## Candidates (Matt's room to adapt)

- K: Teardown 1 chip, Overload 2 cubes (1 in the dev screen: 0.70 a Researcher seat-game), Seize a Security card,
  6 credits and 2 energy. Seize targets a corrupted station or factory in reach (the cleanse op; cities stay the
  Liberate action's), and clears it to neutral rather than taking it.
- Plant costs: a cube, a chip, 2 energy, 2 insight, 8 credits; prices 10 / 10 / 10 / 6 / 6.
- Make antimatter 5 cubes (a dial beside the design change; `mc4` reads the asked 4).
- A burned piece is gone with no protection asked (your ambassador stays on the hex); with no own piece or no target
  the power can't be taken (feasibility, as every power).
- Left as found: Cyclotron Lease (tier 1, 2 energy for 1); the capstones' free antimatter (Sanctuary, Open Archive,
  25% of supply in upkeep with income); Horizon's converter, now the top source.

## New-player summary

Antimatter, the fuel of the big tier-2 plays, now comes mostly from **plants**: buildings whose Action, once a
round, turns one resource into 1 antimatter (a research cube, a chip, 2 energy, 2 insight or 8 credits, a different
one on each plant). Each AI board also has a last-resort way to make it, once a round, by burning its own engine:
the Builder tears down one of its factories (the land is left corrupted), the Researcher overloads one of its
research stations, the Protector seizes back a corrupted station or factory near its nodes. **Make antimatter**, the
standard action, now costs 5 research cubes. The rule that resisted: the plants are built in most games but their
Actions are used less than the corp converter's, so they are not yet the steady source.

## Checks

- Design check ([design-fresh.txt](design-fresh.txt), r97 fresh against r96 on its seeds): **same-effect gone**
  (the three powers' effects now differ); **cheap-source clear** (antimatter 16% from standard actions or tier 1: Make
  antimatter 11%, Cyclotron Lease 5%; r96 42%); **Make antimatter 0.38 a seat-game, inside its hedge band**
  (0.05-0.50; r96 1.36). New at r97: `tag-story` Dark Fiber Collider (Cyber, makes antimatter) and Seize (a Security
  sacrifice that cleanses), both soft; `usage` Antimatter spent 37% (r96 42%, already outside). The rest as at r96.
- `play_read.py` (fresh, 2,880 strategist games): none kept in over 1% and never played; none never kept.
- `npm run cards:fit`: the same four effect boxes under 7.5 pt as 214; the two new cards fit.
- `loop_smoke.py --run` (16 seeds): no timeouts, no card at the play flag (most plays of one card a game 3).
- Speed (`per_decision.py`, r96 from its tree against r97, 6 games, one thread, a busy box: ratios only, [speed.txt](speed.txt)): strategist 0.90× a decision, greedy 1.01×. `speed.py check` not run: the box had no free thread through the item, where it proves nothing (docs/speed.md); no `--update`.
- `bench/same_results.py`: not needed; no core rules file changed after the fresh read.

## Seeds and runs

Dev seeds 36800-36815 (`s219-dev`, `s219-lev`, `s219-dev2`). Fresh 46624-46719 (2,880 r97 strategist with `mc4`,
farmed; 1,440 r97 greedy, farmed; 1,440 r96 strategist, native from its tree). Ledger next free 46720. Kept:
`out/keep/f219`, `out/keep/f219g`, `out/keep/f219-r96`.

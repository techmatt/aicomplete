# 137: sites and war (r56)

Rules revision 56, bumped from the constant (55 to 56), as the prompt said. Rules: the site model in
[ambassadors.md](../../../docs/rules/ambassadors.md); bases, war and the new-player summary in
[war.md](../../../docs/rules/war.md); stations in [board.md](../../../docs/rules/board.md). Where every old job went:
[jobs.md](jobs.md). Scripts: [read137.py](read137.py) (136's and 135's tables, plus war, sites and armor a game),
[levers.py](levers.py) (a `<pair>_<lever>` screen, paired), [diff_counters.py](diff_counters.py). Specs:
`experiments/read-137.json`, `screen-137-*.json`, `diag-137*.json`, `smoke-137.json`.

## Where things stood

136 ended done at r55: 79.9 / 71.5 / 63.9% at d0-d2 on its fresh seeds, 8.40 / 8.10 / 7.75 rounds, with base and
factory Unity already removed (the ambassador only stopped the troop). The code matched 136's write-up. Nothing in
137 depended on unfinished 136 work.

## The build (each number a candidate)

- **One site model.** A site's output by owner (neutral / player / adversary): a city's energy sold / to its owner /
  diverted; a station's neutral cube / research cubes / adversary compute (new: no adversary megaproject cube); a
  factory's nothing (but 1 corp troop at each Discord tip) / chips / 1 adversary troop. One opposing ambassador blocks
  a site while it stands (`board.blocker_stays` 1; 0 keeps r55's "used up as it blocks" as a dial). Armor: the
  owner's own ambassador on the hex (a node's owner's, or the site owner's, which also covers a node there).
- **Claim** (one standard action for all three types): `claim_city` 4 (today's `corrupt_at`), `claim_station` 2 and
  `claim_factory` 2 (the adversary's `station_flip_at`), +1 for each adversary ambassador on the site, all of which
  cancel; a site claim as before. An arriving ambassador no longer claims.
- **Liberate:** `liberate_at` 5 + `free_city` 5 Unity for a city (the ladder's `free_leaves` too),
  `liberate_facility_at` 3 + `free_facility` 2 Unity for a station or factory, +1 per adversary ambassador; one seat's
  supply, reach only (the setup station's own botnet would block a site claim forever).
- **Empty station sites:** one printed a country (`station_sites` in `content/map.json`: Horizon 5, Cintamani 11,
  Onyx 8, Biomia 10), never a base in any layout; no content creates a station there yet.
- **Bases as front piles:** no base troop, no `base_discord`, no `factory_units`, no first base. Corps make troops at
  Discord tips (each neutral, unoccupied factory: 1 on each of its corp's bases) and from the extra-troop rules; a
  player ambassador on a base, or on a tipping neutral factory, is used up instead of the troop. No botnet forms on
  a base (it has no ambassador rules beyond the troop rule).
- **War every round,** at the end (after threats execute); `board.clock` [3] kept as the restore dial;
  `destruction_discord` 1 (tallied `alignment.discord_from.war`).
- **First-base content** (drafts, then `load_adversary.py`): Factory Drive's and War Drums' troop lands on a factory,
  else a city; War Drums', Border War's and Blitzkrieg's arrivals and every Escalation land on each country's
  lowest-numbered factory (new arrival site `factory`: one a country, as the first base was). The cards' clock words
  ("each production step", "their war step") read "each round" and "fight twice".
- **Agents:** the eval's city and station liberation terms now value saving the Liberate cost in one seat's supply
  (the r55 terms valued ambassadors standing for the fallback and Free a station); new `w.block` 4 (each adversary
  site a player ambassador blocks; each own station or factory an adversary ambassador blocks, negative) and
  `w.armor` 1 (an own ambassador on an own piece's hex).
- **Player-board lever:** each board's printed credits +6 (Builder 24, Protector 22, Researcher 20).

## Diagnosis (dev seeds 36700-36747)

r56 as first built read 50-56% at d0 on 16-32 seeds against r55's 81-88%. Findings, in order:

1. **My first content move was 4x too strong:** moving the base arrivals to industrial zones put an ambassador on
   every zone (4 a country) instead of one a country. Fixed with the `factory` site; d0 base went 55.6 to 58.3 and
   d2 28.5 to 34.7 (144 games, [screen_agents.md](screen_agents.md) is the pre-fix agent screen).
2. **The purge win route mostly goes:** r55 wins 50 / 43 / 36 of 144 by the purge on these seeds, r56 as asked 17 /
   8 / 4. Standing blockers on stations and factories stay on the board and count against the purge, and a second
   ambassador spreading onto one makes a botnet: botnets made 12.6 a game against 6.1 at d0 (by piece: factory 3.2,
   city 1.9, station 1.7, node 1.3 a game).
3. **Neither the claim prices nor war's Discord is the cost:** claims at 1 ambassador read 47 / 25% (d0 / d2, 32
   games) against base 50 / 38; `destruction_discord` 0 62 / 38; the old clock 69 / 34 (all within noise of base).
4. **The standing blocker costs about 5 points:** `blocker_stays` 0 reads 63.2 / 50.7 / 41.0 against 58.3 / 49.3 /
   34.7 (144 games; [screen_levers2.md](screen_levers2.md)).
5. **Agents:** without the saving term (`w.freeing` 0) +2.7 / +2.0 at d0 / d1 (noise); the block and armor terms
   are worth +1.4 / +5.6 / +4.9 (off - on: -1.4 ±4.8, -5.6 ±4.6, -4.9 ±4.0), kept on.

## The lever (dev seeds, 144 games a level; [screen_levers.md](screen_levers.md), [screen_levers2.md](screen_levers2.md))

| Cell | d0 | d1 | d2 | rounds d0 / d1 / d2 |
|---|---|---|---|---|
| r55 (main 6b5d719) | 75.0 | 63.2 | 56.9 | 8.28 / 7.71 / 7.17 |
| r56 as asked (income as printed) | 58.3 | 49.3 | 34.7 | 7.62 / 7.58 / 6.65 |
| income +2 | 63.2 | 56.2 | 38.2 | 7.84 / 7.64 / 6.58 |
| income +4 | 68.1 | 54.9 | 47.2 | 8.20 / 7.56 / 7.17 |
| **income +6** (frozen) | 72.2 | 63.2 | 53.5 | 8.17 / 8.08 / 7.10 |
| `blocker_stays` 0 | 63.2 | 50.7 | 41.0 | 8.00 / 7.61 / 6.85 |
| `blocker_stays` 0 + income +4 | 69.4 | 61.1 | 52.1 | 8.33 / 7.73 / 7.17 |

Board income is the lever 136 named (board income, powers, node costs, capstones). +6 is the smallest step that puts
d0-d1 at the 70 / 55 targets; `blocker_stays` 0 is reported as a rule candidate, not adopted (asked Matt, design;
his default kept the prompt's standing blocker).

## The read (fresh seeds 36488-36535, 48 a cell, the three pairs; r55 = main 6b5d719 on the same seeds)

Tables: [read_r56_vs_r55.md](read_r56_vs_r55.md) (`read137.py out/read56 out/read55 --135`).

| | d0 | d1 | d2 |
|---|---|---|---|
| strategist r56 (income +6) | 73.6 ±3.7 | 62.5 ±4.0 | 49.3 ±4.2 |
| strategist r56 as asked (income as printed) | 61.8 ±4.0 | 43.8 ±4.1 | 27.8 ±3.7 |
| strategist r55 | 74.3 ±3.6 | 57.6 ±4.1 | 43.1 ±4.1 |
| paired r56 - r55 | -0.7 ±4.4 | +4.9 ±5.0 | +6.2 ±5.3 |
| paired as asked - r55 | -12.5 ±5.1 | -13.9 ±4.9 | -15.3 ±5.1 |
| rounds r56 / as asked / r55 | 8.01 / 7.69 / 8.12 | 7.83 / 7.31 / 7.67 | 7.05 / 6.60 / 7.14 |
| paired rounds r56 - r55 | -0.12 ±0.17 | +0.16 ±0.15 | -0.09 ±0.20 |

These seeds are harder than 136's (r55 read 79.9 / 71.5 / 63.9 there). **Game length** holds r55's within noise with
the lever (as asked it is 0.3-0.5 rounds shorter: node losses end games sooner). **How games end** (r56 / r55, of
144): d0 Unity deck 82 / 66, node 35 / 30, purge 24 / 41; d1 79 / 47, 53 / 52, 11 / 36; d2 55 / 39, 72 / 78, 16 /
23. The purge route shrinks and the Unity deck takes over. **Tips a game** (d0-d2): Unity 7.10 / 6.51 / 5.35
against r55's 7.01 / 5.93 / 4.86; Discord 2.44 / 2.80 / 2.58 against 3.17 / 3.58 / 3.35 (the adversary megaproject
cubes and base diversions no longer sway Discord; war adds 1.7-2.0 a game). **Threats** (d2): 23.7 dealt, 18.6
countered, 2.2 uncountered (r55 24.5 / 19.8 / 2.1).

**War a game** (d0 / d1 / d2): fronts fought 5.2 / 5.2 / 4.5, factories destroyed in war 2.0 / 1.8 / 1.7 (r55 1.8
/ 1.8 / 1.8, on its war rounds), Discord from war the same, neutral factories making a tip troop 7.0 / 7.5 / 6.8,
corp troops made 12.6 / 13.4 / 11.9 (r55 18.4 / 16.8 / 15.5 from bases and factories).

**Claims and liberations a game** (d0 / d1 / d2): claim a city 0.09 / 0.02 / 0.01, a station 0.52 / 0.43 / 0.38, a
factory 0.31 / 0.12 / 0.03 (r55: Align 0.74 / 0.58 / 0.44 cities, 2.71 / 2.49 / 2.15 stations and factories taken by
an arriving ambassador); liberate a city 0.78 / 0.80 / 0.65, a station 0.39 / 0.37 / 0.42, a factory 0.99 / 0.90 /
0.74 (r55: 2.95 / 2.38 / 1.91 cities freed by every route, no station or factory freed). Cities liberated by any
route 2.84 / 2.42 / 2.07. **Blocks** 9.0 / 8.1 / 5.6 a game by player ambassadors (adversary cities, stations,
factories), 10.4 / 11.7 / 10.9 site-rounds blocked by adversary ambassadors; **armor** used 0.47 / 0.40 / 0.33
(r55's any-ambassador shield 0.42 / 0.32 / 0.28); the troop rule used up 0.62 / 0.86 / 0.53 player ambassadors.
Adversary stations make 10.3 / 10.6 / 9.5 compute a game. Botnets made 10.2 a game at d1 (r55 7.0): on factories
4.0, cities 2.5, stations 2.2, nodes 1.5.

**Peak holdings** (d2, Builder / Protector / Researcher, r56 vs r55): credits 94 / 70 / 73 vs 80 / 57 / 82; insight
12.1 / 14.5 / 24.6 vs 11.6 / 16.0 / 27.0; energy 12.3 / 8.1 / 8.8 vs 9.9 / 7.8 / 8.6; chips 9.1 / 3.6 / 2.9 vs 8.3 /
3.9 / 4.1; research cubes 1.2 / 0.9 / 1.2 vs 1.1 / 2.1 / 3.2; ambassadors 4.6 / 5.2 / 4.2 vs 4.2 / 5.0 / 5.6.

**The block and armor terms, on and off** (same seeds, r56): off - on -10.8 ±3.1 / -6.6 ±3.2 / -2.1 ±3.2 (pooled
over both income cells; at income +6 alone -13.2 / -8.3 / -4.9); node losses 112 / 151 / 181 off against 89 / 133 /
172 on. Kept on.

**Nodes lost a game by cause** (d2, r56 / r55): corruption 5.80 / 4.91, Discord cards 0.58 / 1.49, twists 0.60 /
0.71, war 0.42 / 0.23, counters 0.24 / 0.35, projects 0.24 / 0.12.

## The death audit, continued (136's `death_audit.py` on r56, fresh seeds, d1 and d2: [death_audit.md](death_audit.md))

125 last-node losses (d1 53, d2 72). **The fatal blow:** corruption 95, Border War 14, Discord cards 14 (Firestorm
8, Offensive 4, Two Fronts 3), a threat 1, war 1. Against 136's r54 audit (116: corruption 64, Discord cards 36,
Border War 15, war 1), corruption's share rises from 55% to 76% and Discord cards' falls: fewer Discord tips (the
adversary megaproject cubes and base diversions no longer sway Discord). **War is no killer:** it takes 44 nodes
from losing seats over their games but ends a seat once. **Seat 0 (Horizon) still dies most** (Builder 82 of 125, the
Protector 20 of 22 at d2 P+R where it sits in seat 0): Horizon #14 still sits in spread range of both corrupted
cities, 136's map finding. A node-safe placement existed in 61 of 95 corruption deaths (136: 50 of 116 overall), and
85 of 95 dying seats had every node within 1 of the corrupting city: the agents still place into the coming blast
(136's `node_exposure` fix is still off). Verdict as 136's: mostly the map and the corruption cascade, with the
agents' placement in about two thirds of the corruption deaths.

## Not done

- No content creates a station on an empty station site yet (the sites are printed and tested only).
- `web/`: `npm run check`, `cards:fit` and the e2e were not run (no node_modules in the worktree); the view's map
  cells carry a new `station_site` field the client ignores.
- `speed.py`: no verdict across a rules revision, and the box was loaded all session (Matt's message 5727), so the
  baseline was not `--update`d. `bench/per_decision.py out/r55cfg out/r56 --agent strategist` (c2 d2, 6 games, a
  ratio on a loaded box): options a decision 1.34x, ms a decision 1.57x, rounds 0.87x. The new standard actions
  (Claim and Liberate list every affordable site) and Move onto more standing positions add options; each eval reads
  the new block and armor terms.

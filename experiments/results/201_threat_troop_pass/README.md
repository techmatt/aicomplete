# 201: the adversary's vibe: threats, events and troops (r89)

`RULES_REVISION` 88 → 89, bumped from the constant (the prompt said r89 too). Built on main b99ed8f (200, r88). Every
number is a candidate. The old side is r88 (`out/r88`: b99ed8f's content and configs, built with 201's stats-only
measures, so both sides tally the same counters; its games are r88's). Matt's vibe (2026-11-12): botnets spread
quickly but are manageable; agents are a problem on par with ambassadors, overwhelming them late; troops are big,
meaningful threats, few and deadly (amendments 9026, 9029). Message 9119 took Annihilate and the antimatter action
out first; message 9148 kept the Wall Muster off and asked what decides games.

## What changed (r89)

| change | r88 | r89 | read (dev seeds 36800-36831 unless named) | why |
|---|---|---|---|---|
| `board.troop_node` | 4 (troops never hit nodes) | **0**: from the start | −47 alone (screen 1): games end by round 2-3 | Matt 9026: troops destroy the players left unchecked |
| `board.troop_steps` | 2 | **1** | +7 against 2 on the troop build (screen 2, st2 vs A) | troops need time to move |
| Event Musters (I / II / III) | 1, 1, 2 on most cards | **0 / 0 / 1** | m011 vs none: −22 → stage I off; m001 +2.6 vs m011 | Matt 9029: fewer troops, each deadlier |
| `adversary.wall_troops` (the board's Muster where the Protector stands) | 1 | **0**, the rule off the board | −1.3 / +1.5 / −1.0 ± 2.1-2.7 (three screens) | 9029's fewer tokens; Matt kept it off (9148) |
| Threat troop landings of 2+ | Orbital Strike 2, Total War 3, Conscription 2, Garrison State 2, Iron Curtain 2 | **1 fewer each** | +2.9 (screen 3, Ttt vs T) | fewer pieces moved |
| Botnets spread (new event line kind, `spread_botnets`) | — | Long Patrols, Signal Jamming, Station Raids, Fifth Column | with the takedown at 3: +1.3 (screen 3) | botnets spread quickly |
| `board.botnet_credits` (Take down a botnet) | 5 | **3** | (with the line above) | but are manageable |
| Agent income on events | 5 of 15 cards | **+1 agent a country a round on every stage-II and III card** (six gain it) | −3.6 (screen 3, F vs Tsp) | agents grow by stage |
| Threat counters' insight | tier 1 6, tier 2 7 | **3, 4** (3 / 5 screened: −0.3) | +0.5 (screen 1) | 199's income (4-10 a round) |
| Annihilate (every board) and the antimatter action | on (2 antimatter; `antimatter_discord` 1) | **off** (message 9119) | −8.3 (screen 3, base vs r88) | a later item redesigns antimatter |
| `compute_income` d0-d4 | 1, 4, 9, 28, 30 | **0, 4, 17, 25, 30** | below | calibration, last |
| `ambassador_income` d0-d4 | 0, 2, 3, 3, 3 | **0, 0, 0, 0, 1** | screen 4: agents 0 at d1-d3 +22.7 vs 200's | the stage lines carry the agents; calibration |
| Agents: eval `troop_exposure` (new, 1.0) | — | a node a troop reaches next march reads exposed (half with armor) | +2.1 on vs off (screen 3), −1.3 on the Annihilate build (screen 2) | 147's rule: the eval sees the new threat |
| Stats only | — | `map.troops_placed.<source>`, `map.troop_strikes`, `map.troop_destroyed.<kind>`, `map.troops_removed`, `map.botnets`, `map.botnets_removed`, `map.agents`, `map.agent_pool`, `map.ambassadors`, `map.ambassadors_held`, `map.agents_over` (all by round) | — | the vibe measures |

**Give / get.** No card or conversion is repriced. The insight counter items: 3 insight (7.5 at `values.json`, 12 at
the anchor's 4) beside tier 1's 3 energy (13.5) and 12 credits; 4 insight (10, 16) beside tier 2's 4 energy (18) and 17
credits: at the anchors the option sits level with the others, as the threat standard asks, where 6 and 7 cost
24-28 at the anchors.

## Ladder (fresh, paired against r88; [`ladder-strategist.md`](ladder-strategist.md), [`ladder-greedy.md`](ladder-greedy.md))

Confirm read on the final dials, seeds 45199-45342 (144 a pair, 432 games a level; greedy 48 a pair):

| d | strategist r89 | r88 (same seeds) | gap | band | verdict | greedy r89 (r88) | rounds r89 (r88) |
|---|---|---|---|---|---|---|---|
| d0 | **83.1** | 93.3 | −10.2 | 90-100 | **unmet by 6.9** | 80.6 (88.2) | 7.61 (6.30) |
| d1 | **78.5** | 75.0 | +3.5 | 70-80 | met | 71.5 (65.3) | 7.60 (7.36) |
| d2 | **61.3** | 62.5 | −1.2 | 55-65 | met | 59.0 (64.6) | 6.87 (7.01) |
| d3 | **43.3** | 43.1 | +0.2 | 40-50 | met | 46.5 (41.0) | 6.26 (6.33) |
| d4 | 20.4 | 35.0 | −14.6 | none | — | 21.5 (38.2) | 5.80 (6.48) |

All levels −4.4 ± 1.2 against r88. The first read (45055-45198, 200's dials) put r89 at 83.6 / 79.9 / 75.5 / 40.5;
dials screened on those spent seeds: d2 compute 9 → 17 (60.0; agents 1 read 55.3), d3 compute 28 → 22 (49.8) or 18
(53.2), so 25 between them, unscreened, confirmed at 43.3; d0 compute 1 → 0 (88.0 there, 83.1 on the confirm seeds).
**d0 is out of the dials' reach:** compute 0 and agents 0 are their floor. Its losses are the round cap (16.9%, r88
6.7%), not troops (0.0% node losses): the players' purge is slower without Annihilate (message 9119: −14.6 at d0 on
the dev seeds, screen 3, before any troop change). 200 removed d0's two extra event cards (`stage_extra`); a longer d0
deck is the lever, a rules change left for Matt.

**How games end (confirm, strategist).** The Discord deck nearly stopped ending games: d2 26.9% → 3.2%, d3 38.9% →
2.3% (no Wall Muster, fewer troops near bases, so Warmongering's extra drones and war's Discord drop), and node loss
took its place: d2 1.9% → 24.1%, d3 5.8% → 36.3%. Troops now end games where Discord did.

## The vibe (strategist, d0-d3; [`vibe.md`](vibe.md))

Confirm seeds 45199-45342, a game, d0-d3 pooled. Snapshots as production starts (after the action phase).

| measure | r89 | r88 |
|---|---|---|
| troops placed a game (factory / Muster / Wall Muster / threat / project, Discord, twist) | **13.3** (4.7 / 2.1 / 0 / 2.9 / 3.6) | 37.3 (8.2 / 13.5 / 6.5 / 4.3 / 4.8) |
| troops placed, rounds 1-4 / 5-8 / 9+ | 4.7 / 6.3 / 2.2 | 18.5 / 16.7 / 2.1 |
| troops placed a game by level, d0 / d1 / d2 / d3 (first read) | 4.8 / 8.2 / 14.1 / 21.9 | 14.3 / 23.8 / 53.2 / 59.4 |
| troops on the map, rounds 1-10 | 0.1 0.4 0.9 1.4 1.7 2.3 3.3 4.0 3.2 3.7 | 2.5 1.1 4.4 6.5 9.6 11.5 11.5 11.0 5.3 3.3 |
| marches that reach a player (strikes) | 5.96 | 4.29 |
| players' losses to troops: nodes / stations / factories | **4.03** / 0.80 / 0.20 | 0 / 2.65 / 0.71 |
| troops the players removed | 1.23 | 16.48 (Annihilate 8.9 uses a game) |
| games lost to troops (a node loss with a troop kill in its last round) | **13.4%** | 0% |
| answered (removed ≥ ¼ of troops placed) / ignored: games, win | 199, **79.4%** / 1529, **64.9%** (lost to troops 15.0%) | 1417, 74.0% / 311, 43.1% |
| botnets on the map, rounds 1-10 | 2.0 2.1 2.0 1.9 1.8 1.8 1.7 1.5 0.9 0.8 | 1.8 1.5 1.2 1.0 0.9 0.8 0.5 0.5 0.2 0.3 |
| botnets the players clear a game | **5.27** | 1.93 |
| agents (map + pools) / player ambassadors (map + hand), rounds 1, 2, 4, 6, 8, 10 | 3.2/3.4, 3.8/2.9, 4.5/3.3, 7.4/3.6, 10.6/6.7, 13.2/13.3 | 7.7/3.6, 7.9/3.3, 8.5/2.6, 10.0/3.6, 9.9/8.7, 10.7/20.5 |
| a country with agents over ambassadors on the map: games, first round | 83%, round 2.0 | 91%, round 1.6 |
| Vigil insight a Protector game (first read) | 4.68 | 9.28 |

**Read.** *Botnets* now hold at about two on the map through round 7 and are cleared 5.3 times a game (r88: a starting
botnet a country, worn down to none). *Agents* start on par with the players' ambassadors (3.2 against 3.4) and pull
ahead from round 2, twice the players' by rounds 6-8; at r88 they were double from round 1 and the players overtook
them late. *Troops* are a third as many, each now a node: 4 nodes a game, 13% of games lost to them, and answering
them is worth about 15 points (79% against 65%), though the strategist answers in only 12% of games (it removes 1.2 a
game: Annihilate was its troop answer, and few cards remain). The ignored-versus-answered split is a correlation
across games, not a controlled read.

## What matters when (message 9148; screen 5, dev seeds, the r89 build at 200's dials)

| cell | win (d0-d3) | gap ± SE vs r89 | troops placed | rounds | node losses to troops | games lost to troops | troops on the map at the end (won games) |
|---|---|---|---|---|---|---|---|
| r89 | 60.9 | — | 14.6 | 7.62 | 4.63 | 19.3% | 0.33 (0.00) |
| the Wall Muster back (`wall_troops` 1) | 59.9 | −1.0 ± 2.1 | 20.7 | 7.47 | 5.37 | 23.2% | 0.27 (0.00) |
| extra troops early (a Muster 1 on every stage-I card) | **16.4** | **−44.5 ± 2.8** | 8.2 | 2.85 | 2.64 | 78.9% | 0.00 |
| the same late (stage-III Musters 2) | 61.2 | +0.3 ± 1.0 | 16.3 | 7.62 | 4.85 | 19.3% | 0.54 (0.00) |

**What matters when.** Troops decide games in the first rounds, while each seat holds one or two nodes: a Muster on
the stage-I cards ends 79% of games by round 3, and the same troops in stage III change nothing (+0.3). A won game ends
with no troop on the map (0.00 in every cell), as the purge counts troops until its sixth Unity tip
(`alignment.purge_at`). The Wall Muster's 7.5 troops a game land on each country's lowest-numbered factory and cost
nodes (+0.7 a game) but few games: a lost node is a loss only when it is the last. That is why r88's
early troops read as noise: they never struck nodes, and Annihilate cleared them (16.5 removed a game).

## Speed

`speed.py check` gives no verdict across the revision; `per_decision.py out/r88 out/r89root` (c2 d2, seed 1000, one
thread, best of 2): greedy **1.45×** a decision, with 1.27× the options a decision (more botnets to take down, at 3
credits) and 1.21× the rounds; the strategist 0.84×. The new eval term costs nothing: greedy with `troop_exposure` 0
against 1 on the r89 binary, 0.91× (on is faster: other games). The baseline is updated at r89. The r88 side
(`out/r88`) was built from 201's code, so its records say rules revision 89; its games are r88's (its content and
configs).

## Files

`screen_spec.py`, `read_spec.py`, `dial_spec.py` (specs in `experiments/farm/s201-*.json`, `f201-*.json`,
`d201-*.json`), `apply_drafts.py` (the drafts edit, rerunnable), `vibe_read.py` (the measures), `paired.py`,
`farmdir.py` (200's), `tables.sh`; tables `screens.md`, `ladder-strategist.md`, `ladder-greedy.md`, `vibe.md`.
Seeds: dev 36800-36831; fresh 45055-45342 spent (ledger next free 45343).

## For new players (r89's changes)

Adversary troops are now rare and deadly: a troop destroys your node, research station or factory on or next to its
hex, and it takes one step a round, so you see it coming a round ahead. Only the last stage's event cards muster
troops, the Militarist no longer musters where the Protector stands, and threats land one troop fewer; adversary
factories still make one a round. Left alone, troops take your nodes, and a player with no node loses the game for
everyone: remove them, harden or stand your own ambassador on what they aim at, or keep the Protector's node in their
way. Botnets spread from four event cards but cost 3 credits to take down. The adversary's agent income starts at 0 and
every middle and last-stage event card adds 1 a country, so agents grow as the game goes on. Paying a threat with
insight costs 3 (tier 1) or 4 (tier 2). Annihilate and the antimatter action are gone for now. **What resisted the
summary:** nothing.

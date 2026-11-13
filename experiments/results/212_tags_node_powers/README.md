# 212: tags follow the story, node powers by position, green land lifts (r93)

`RULES_REVISION` 92 → 93, bumped from the constant (the prompt named no revision). Built on main b5490fa (211, r92).
Old side: r92 = b5490fa's binary and tree (`out/r92src`). Every number is a candidate. Answers: amendment 9513 (the
five staging questions), messages 9540 (bands, the Lift band), 9548 (no tag owns an effect), 9565 (the fresh-spot
rider). Tables: [screen.md](screen.md), [screen2.md](screen2.md), [fresh.md](fresh.md), [design-screen.txt](design-screen.txt),
[design-fresh.txt](design-fresh.txt). Scripts: `spec.py` (specs), `read212.py` (reader), `tagcount.py`, `opkeys.py`;
overlays `overlay.txt` (cards) and `overlay-ais.txt` (powers), rerunnable with `scripts/cards/apply_overlay.py`.

## The tag table

| tag | story | usual effects (`scripts/design_check.json` `tag_story`) | cards (all) r92 → r93 | non-event cards | powers paid in it (B / P / R) r92 → r93 |
|---|---|---|---|---|---|
| Security | force and protection | troops and agents removed, pushes, shelters, cordons, hardening, shields on node hexes | 44 → 46 | 32 → 34 | 0/4/0 → 0/4/0 (Recovery out, Debrief in) |
| Diplomacy | cooperation: trust, Unity, working with humans, green cleanup | ambassadors, Unity Sway, cleanses, marker clearing, land lifts, gifts | 42 → 47 | 35 → 39 | 0/2/0 → 0/2/0 (Debrief out, Recovery in) |
| Science | discovery | insight, research cubes, stations, megaproject cubes, antimatter physics, delays | 43 → 39 | 37 → 34 | 0/0/0 |
| Industry | building | chips, factories, capital, free buildings | 36 → 36 | 33 → 33 | 2/0/1 |
| Power | energy and antimatter | energy, antimatter, the weapons it feeds | 48 → 51 | 42 → 44 | 1/0/1 |
| Cyber | the network | botnets, compute drains, firewalls, node placement, information | 44 → 44 | 37 → 37 | 0/0/2 |

Neutral (any tag): draws, credits, triggers and thresholds, threat counters (`jam`, `auto_counter`: every tag answers
threats its own way), `liberate` (the capstone every tag route reaches), a tag's own payoffs (`gain_per_tag`,
`production_per_tag`, `income_per_tag`, `card_discount`), `reuse`, `choose_player`. Where the content argued against
the draft reading: Power stays the energy tag (Matt, 9513; the draft gave energy to Industry); green cleanup is
Diplomacy's, not Science's (9513); insight is both Science's (discovery) and Cyber's (information: Spoofed Credentials,
Security Ops Floor); antimatter is both Power's and Science's (Penning Trap Array is particle physics); `delay_threat`
is Science's (forecasting). **Rename:** none proposed. "Diplomacy" reads as cooperation in play already (Accord,
Shared Grid Accord, the help cards); "Cooperation" would say it better but renames a tag id that 192 cards, both
`needs` and the faces print, for no rule change.

**The check.** `design_check.py`'s new `tag-story` check (soft, Matt 9548) flags a card or tag-paid power whose
effects hold an owned op, none of them owned by one of its tags. At r92 it flagged 35 under the first table; 22 of
those were the table's fault (threat counters, liberation, tag payoffs, gifts), fixed by calling those neutral; 13
were real, fixed by the retags below and the Protector's two re-costs. At r93 it flags none; the one listed exception
is the Researcher's Disrupt (a Cyber card hacks troops anywhere in a country: its story is Cyber's).

**Retags** (overlay.txt; ids and prices unchanged): Stress Test Suite Science → Security (shields); Field Sensor Grid
Science → Science/Diplomacy (Matt, 9548: it reads as Science); Open-Weights Release Science/Cyber → Science/Diplomacy;
Shadow Broker Scrub Cyber/Science → Cyber/Diplomacy (the three clear markers: cleanup); Reclamation Crew Industry/Power
→ Industry/Diplomacy (a cleanse); Relay Mast Rollout Power → Cyber (places nodes); Hardened Bunker Complex and Multiyear
Defense Budget Security → Security/Power (they make energy); Fusion Commons Science → Power/Diplomacy (an event: its
tags never count in play); Zero-Day Auction Cyber → Cyber/Power and Antiproton Foundry Industry → Industry/Power (they
make antimatter); Field Assay Team Science → Security (removes agents).

**Content::check** no longer refuses a shield, delay or marker clearing off a Science card (046's niche; Matt 9548:
"no tag owns an effect", Magic's colors); a board power still prints no shield or delay.

## Sacrifice and discount check

- **Sacrifice outlets by tag** (the table's last column): unchanged in count. The Protector pays 4 powers in Security
  (Stand Down, Shelter, Debrief, Interdict) and 2 in Diplomacy (Accord, Recovery), Matt's 9513 route; Science still pays
  no power on any board (as at r92), so a Science card is never sacrificed: its value is its discount and thresholds.
- **Discounts.** `science_every` and `industry_every` are unchanged; Science loses 3 non-event cards (37 → 34, −8%),
  Industry none. The Researcher's printed Science tag still counts. Insight gained and spent a seat-game hardly move
  (34.5 / 29.4 against r92's 34.1 / 29.2 on the fresh seeds), so no discount rule needed a change.

## Node powers by position

**Ops (new):** `if_node_next { spots, then, fresh }` (a rider on the node an earlier `place_node` of the list placed:
"if it is on or next to a <spot> [none of your other nodes is on or next to], also ..."), `ambassador_to_spot` (a new
ambassador onto that city, the player choosing among several), `lift_land { count }`. Each has its `value.rs` arm and
`values.json` row (`if_node_next` 0.85, `if_node_next_fresh` 0.5, `ambassador_to_spot` 1.25, `lift_land` 12) and its
ledger mirror. Agents: a node's hex is a prompt the search scores by stepping the engine, so the rider is valued where
it lands; no heuristic was added. Range 2 (Matt: the exception, never the rule) is not built: no content asks for it.

**The plain rider bites on nothing.** On the dev screen (`screen.md`) "on or next to" its spot was met on 99-100% of
node-power uses: a city, a factory and a station are within a hex of almost every placement (210's measure: 85-98%).
The fresh-spot rider (Matt, message 9565) is met on 71-74%; it is the shipped rule.

| | Buy In | Accord | Peer Review | Stand Down | Recovery | Debrief | Interdict | riders met | ambassadors onto a city a game |
|---|---|---|---|---|---|---|---|---|---|
| band | 0.30-0.90 | 0.30-0.90 | 0.30-0.90 | 0.25-2.00 | 0.25-2.00 | 0.25-2.00 | 0.25-2.50 | | |
| r92 (fresh seeds) | 0.61 | 0.32 | 0.17 | 0.60 | 0.45 | 1.04 | 0.68 | - | 0 |
| **r93** | **0.39** | **0.35** | **0.32** | **0.26** | **0.60** | **1.50** | **0.48** | 71% | 0.90 |
| prompt's numbers (plain rider, Peer Review 3 cubes) | 0.61 | 0.41 | 0.18 | 0.27 | 0.43 | 1.52 | 0.45 | 100% | 1.95 |

Node powers a seat-round, the others a seat-game (fresh seeds 46240-46335). Every one is inside its band at r93; Peer
Review is not at the prompt's 3 cubes (0.18), nor at 2 (0.26-0.27 on the dev screens), and is at 1 (0.31-0.32).

**Give / get** (values.json; the rider at its measured 71%): Buy In 6 credits → a node 8 + a chip 4 × 0.71 = 10.8
(r92 12); Accord a Diplomacy card → a node 8 + an ambassador on a city 5 × 0.71 = 11.6 (r92 12); Peer Review 1 cube (3;
6 at the anchors) → a node 8 + an insight 2.5 × 0.71 = 9.8 (r92: 3 cubes, 9 → 10.5, the cause of its 0.17); Stand Down a
Security card → a troop in reach (11 at the anchors; r92 3 credits); the green lifts +8.4 each (a land token 12 × 0.7).
All gain at their prices; none is a break-even conversion.

**Placement spread** (fresh seeds, by board: share of placements on or next to each kind; nodes, countries, groups
and the farthest two nodes at the end, a seat-game):

| board | side | placements | factory / city / station | own factory / city / station | nodes | countries | groups | farthest |
|---|---|---|---|---|---|---|---|---|
| Builder | r92 | 7.10 | 98 / 93 / 87% | 4 / 51 / 24% | 4.04 | 1.09 | 1.14 | 2.46 |
| Builder | r93 | 6.16 | 98 / 93 / 83% | 4 / 53 / 22% | 3.53 | 1.02 | 1.06 | 2.17 |
| Protector | r92 | 8.48 | 93 / 90 / 88% | 8 / 40 / 12% | 7.09 | 1.59 | 1.63 | 4.26 |
| Protector | r93 | 8.46 | 94 / 93 / 88% | 6 / 37 / 12% | 7.08 | 1.65 | 1.69 | 4.29 |
| Researcher | r92 | 8.89 | 84 / 83 / 92% | 3 / 36 / 20% | 4.03 | 1.28 | 1.46 | 2.78 |
| Researcher | r93 | 9.37 | 86 / 86 / 92% | 3 / 33 / 18% | 4.28 | 1.36 | 1.51 | 2.98 |

The boards now move apart: the Builder places a node less a game and ends tighter (farthest 2.46 → 2.17, groups 1.14 →
1.06: Buy In's chip needs a factory none of its nodes touches, and it buys fewer), the Researcher places more and
spreads (farthest 2.78 → 2.98, 1.28 → 1.36 countries: cheap Peer Review hunting new stations), the Protector reaches a
little wider (1.59 → 1.65 countries) and puts 0.90 new ambassadors a game straight onto cities. At r92 the three moved
together with the board's own count of placements. The shifts are small (a tenth to a fifth of a hex or country):
the map is dense in spots, so "fresh" is the lever that bites; a scarcer spot (your own factory: 4%) would bite harder.
(Against r91: 210's r92 table is r91's placements to the measure's precision, 210 changed no node rule.)

## Fresh read (46240-46335, 96 seeds × 3 pairs × d0-d4, r93 against r92's binary on the same seeds)

| d | r93 | r92 | r93 − r92 (paired) | band (message 9540) | | prompt's numbers |
|---|---|---|---|---|---|---|
| d0 | 86.8 | 92.7 | −5.9 ± 2.1 | ≥ 85 | met | 89.6 |
| d1 | 69.1 | 73.3 | −4.2 ± 3.2 | 65-80 | met | 73.6 |
| d2 | 56.6 | 53.1 | +3.5 ± 3.2 | 50-65 | met | 60.1 |
| d3 | 44.4 | 44.4 | +0.0 ± 3.4 | 35-50 | met | 48.6 |
| d4 | 17.7 | 21.5 | −3.8 ± 2.6 | - | | 18.8 |

Rounds 7.09 (r92 7.09). Pooled r93 is about 2 points under r92; the prompt's cell reads +2.8 to +4.5 over r93 at every
level (± 2.3-3.3): the fresh rider costs the seats a little, mostly the Builder's chips. No dial moved (9540).

**Resource use** (fresh, a seat-game, gained / spent, r93 against r92): credits 214 / 233 (215 / 234), insight 34.5 /
29.4 (34.1 / 29.2), energy 31.8 / 23.1 (31.9 / 23.6), chips 6.1 / 4.3 (6.5 / 4.6), antimatter 5.0 / 2.9 (5.1 / 3.0),
research cubes 12.0 / 8.0 (12.2 / 8.6), ambassadors 17.3 / 13.2 (17.9 / 14.0). Land: Lift the action 0.01 a seat-game
(its band 0.005-0.2, 9540), the green cards' lifts 0.09 a game.

## Green cards

Shared Grid Accord, Fusion Startup Stake and Pumped Hydro Storage lift 1 land-corruption token in reach beside what
they did; prices kept (each was already a gain; the lift is worth 8.4 at values.json and fires only with a token in
reach, 0.09 a game). Fusion Commons, Citywide Microgrid and Continental Supergrid carried it first and fell under
7.5 pt in `cards:fit` (5.8, 6.9, 5.8 pt), so they are as at r92. Plays a game, r92 → r93: Shared Grid Accord 0.40 →
0.49, Fusion Startup Stake 0.05 → 0.09, Pumped Hydro Storage 0.01 → 0.03 ([cardplays.md](cardplays.md) has every
touched card).

## Checks

- `loop_smoke.py --run` (16 seeds): no timeouts, most plays of one card a game 5 (Ceasefire Monitors), flag 12.
- `play_read.py` on the fresh read: no card kept in over 1% of games and never played; none never kept.
- Design check ([design-fresh.txt](design-fresh.txt), r93's base cells against r92): no `tag-story`; every node power
  and the Protector's tag powers in band ([usage-fresh.md](usage-fresh.md)); the usage misses (Mint, Grant, Tidal Grid's
  chip converter, Remove a troop high) are r92's too; Particle Run falls 0.33 → 0.24 a Researcher seat-game (band
  0.25), the Researcher's cubes going to Peer Review at 1.
- Speed (`per_decision.py`, 16 games, d2, r92 → r93): strategist 0.96× a decision (0.94× a game), greedy 0.91× (0.81×,
  shorter games). `speed.py check` gives no verdict across a revision; the baseline is not updated: the box was loaded
  (conductor: 0.4 threads free), so its absolute times would be wrong.
- Card viewer: the three green cards' faces and texts render (`card-shots.mjs`; the fresh read's `f212/results.json`
  is committed so the viewer's stats read r93).

## Seeds and runs

Dev 36800-36831 (screen 1: 2,880 games, r92 base 480; screen 2: 1,440). Fresh 46240-46335 (2,880 r93 + prompt, 1,440
r92), native; ledger next free 46336. The first fresh launch went through `farm-seeds` and was stopped for the
card-fit fix; its broker job ran on (docs/simulator.md, *The farm*). Kept: `out/keep/f212`, `out/keep/f212-r92`.

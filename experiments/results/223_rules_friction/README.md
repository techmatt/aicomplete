# 223 (prompt 226): rules friction, r98

Matt's rulings of 2026-11-13 on 224's friction list. `RULES_REVISION` bumped from the constant, 97 to 98. Base: main
e438864 (222 on 219's r97; the judge is `strategist` with strategy=0, as 222 shipped). Readers: [read.py](read.py)
(new against base on the same seeds, by difficulty and pair, counters, powers), [plays.py](plays.py).

## What was built

| # | Ruling | Built |
|---|---|---|
| 1 | One corruption scar (locked) | The corruption marker is gone: `common::MARKER`, `board.node_marker`, `Op::ClearMarkers`, `Removable::Markers`, `board.clear_unity`/`clear_credits`. Land corruption is the one token. Face-down tokens went too (their only source was a node hex; Matt, message 10037, confirmed): `adversary.blight_node_discord`, `NODE_BLIGHT`. Lift, Recovery and cards lift a token. |
| 2 | Node loss is a node rule, no scar (locked) | 2 agents on a player's node hex destroy the node (a city with a node too, at 2; amendment 10004 q1), the agents go to the supply, and nothing marks the hex. A node lost any other way leaves nothing either. |
| 3 | Corrupt X tie (locked) | An agent next to two or more open targets stays (`plan::Converge::ties`). The players' choice, its forecast (`plan::converge_pick`) and the agents' converge rule are gone. |
| 4 | Wall (locked) | `adversary::place_troop` is the one gate: no troop is placed on the Protector's node hex (Muster, factory, botnet, threat, project), tallied `adversary.troops_walled`. Vigil unchanged. |
| 5 | Protector's powers (candidate) | 9 to 7: **Stand Down merged into Interdict** (1 Security: up to 2 troops in one hex in reach; the energy dropped), **Shelter cut** (0.10 uses a game at base; shelter stays as a card effect and Aegis Doctrine). |
| 6 | Per-tag income once (candidate) | **Shipped**: `cards.tag_income_once` 1 freezes a building's tag count at play (`cards::freeze_tag_income`, read by `cards::each_tag_income`). Three buildings carry `income_per_tag`: Foundation Model Campus, Vertical Monopoly, Power to the People. The agents still value it on live counts (`value.rs`), a small overvalue. |

Content the removed mark forced: Recovery is `lift_land` 1 with `unity` 2 (the Unity rides on the lift; a first build
paid the Unity as a separate `sway` and fired 2.3 times a game with nothing to lift, so read 1 is spent and not
reported as the result); Field Sensor Grid, Shadow Broker Feed and Open Weights Release clear land corruption instead
of markers (counts kept); Network Cleanup's three tiers lift land corruption (tier III: within 2, `near_player`, since
a hex holds one token). The parked texts: Teardown and Meltdown (Overload renamed: the name clashed with a tier-2
threat) reach anywhere; Seize turns a site neutral; Buy In, Accord, Peer Review "bring X into your reach".

## Give / get (at `configs/agents/values.json`)

- Recovery: give 1 Diplomacy card (a sacrifice); get a land token lifted (`lift_land` 12 × target_share) + 2 Unity
  (2 × `sway` 4 = 8). Before: a marker cleared (`clear_markers` 10, its 2 Sway included). Gain kept or raised; the
  price unchanged.
- Field Sensor Grid (6), Shadow Broker Feed (2), Open Weights Release (16): each marker cleared (10) becomes a token
  lifted (12): no card weaker at the table, none repriced (amendment 10004 q2). In play they have far fewer targets
  (below).
- Interdict: give 1 Security card; get up to 2 troops in one hex (`damage`, `remove_troops` 7 each). Before: Stand
  Down (1 Security, 1 troop in reach) and Interdict (1 Security + 1 energy, up to 2 in a hex). The merged power is
  worth at least either.

## Fresh read (46816-46847, 32 seeds × 3 pairs × d0-d3, 384 games an agent; base natively on the same seeds)

| agent | d0 | d1 | d2 | d3 | all, gap ± SE | rounds base → new |
|---|---|---|---|---|---|---|
| greedy base | 88.5 | 71.9 | 56.2 | 50.0 | | 8.73 |
| greedy r98 | 84.4 | 77.1 | 58.3 | 47.9 | +0.3 ± 2.4 | 9.10 |
| strategist base | 88.5 | 70.8 | 53.1 | 56.2 | | 8.93 |
| strategist r98 | 86.5 | 68.8 | 55.2 | 46.9 | −2.9 ± 2.6 | 9.15 |

By board pair (gap ± SE): greedy bp −1.6 ± 3.7, br +3.9 ± 4.2, pr −1.6 ± 4.6; strategist bp −4.7 ± 4.1, br −1.6 ± 4.8,
pr −2.3 ± 4.8. Strategist d3 −9.4 ± 5.9 is the largest cell (1.6 SE). Full tables: `out/keep/223_rules_friction/`
read-fresh2.txt; the first read (46784-46815, before the Recovery fix, so spent): greedy +2.3 ± 2.2, strategist
+1.6 ± 2.4.

**Ruling 3's tie:** 2.72 tie hexes a game, 3.68 agents a game staying put (all agents), against 16.1 agents converging
at base (13.4 now). **Wall:** 0.72 troops a game kept off the Protector's hex (0.23 factory, 0.49 project/botnet).

**What moved Unity:** markers cleared paid 3.88 Unity a game at base; now 0. Lifts pay 0.07. Cards' Unity rose 2.0.
Unity tips 7.92 → 7.82; Discord tips 2.24 → 2.35.

## The Protector's powers (uses a seat-game, strategist; greedy alike)

| power | base | r98 |
|---|---|---|
| Bulwark | 4.89 | 5.26 |
| Accord | 3.29 | 3.90 |
| Debrief | 2.09 | 2.44 |
| Patrol | 2.29 | 2.38 |
| Interdict (Stand Down merged in) | 0.59 (+0.43 Stand Down) | 1.12 |
| Seize | 0.06 | 0.07 |
| Recovery | 0.89 | 0.05 |
| Shelter (cut) | 0.07 | — |
| Stand Down (merged) | 0.43 | — |

**Recovery has almost no target now** (land corruption: about 0.2 tokens a game, open land and Teardown only):
design_check flags it unused. Asked Matt (design question 2); default shipped as built, a cut (Protector 6) or a
retarget proposed for a later item. Debrief reads dominant (2.44 against its 0.25-2.00 band).

## Ruling 6: per-tag income counted once (on against off, same build, paired)

Screen (spent seeds 46784-46815, [r6read.py](r6read.py)): greedy +0.3 ± 0.7, strategist +1.0 ± 0.8. **Fresh confirm
(46848-46879):** greedy −1.0 ± 0.7 (d0 −1.0, d1 +0.0, d2 −1.0, d3 −2.1), strategist +0.0 ± 1.0 (d0 +0.0, d1 +1.0,
d2 +3.1, d3 −4.2 ± 2.1); rounds +0.0-0.1. The three buildings' plays a game are unchanged (0.20, 0.11, 0.10). Shipped
on: play holds, and the rule reads one way for programs and buildings. It was screened on the r98 build (rulings 1-5
in), off against on, so the 1-5 read above is without it: the two changes were read apart, not together.

## Not done
- `adversary.wall_troops`, `wall_landing` and `wall_pressure` are dead (0 since r89); left for a cleanup.
- Agent forecasts (`agents/eval.rs::toward`, `agents/moves.rs`' threatened sites) still assume every adjacent agent
  converges; r98 keeps tied ones put. A small accuracy fix.
- Adversary faces still print "Corrupt nodes" for node lines; the rulebook teaches it as the node rule.

# 200: engine overhaul: every resource feeds an engine; antimatter is epic (r88)

`RULES_REVISION` 87 → 88, bumped from the constant (the prompt said 88 too). Built on main aab153e (199, r87). Every
number is a candidate. The old side is r87 (`out/r87`: aab153e's tree and build), on the same seeds.

## What changed (r88)

| change | r87 | r88 | read | why |
|---|---|---|---|---|
| **Annihilate**, a power on every board (last in each list) | none | 2 antimatter, once a round: every agent and troop in one hex in reach comes off, 3 Sway to Unity | Sway 3 over 2: +5.3 ± 1.2 (screen 5, 1440 games); used 1.3-2.8 a seat-game on every board | Matt: antimatter clears the huge stacks; a vehicle from a strong engine to the purge |
| **Antimatter plants** (tier 2, needs 1 tag, 10 credits, 1 insight) | Positron Storage Ring alone | Penning Trap Array (Science: Action 1 research cube + 1 energy → 2 antimatter), Antiproton Foundry (Industry: 1 chip + 1 energy → 2); 1 antimatter when built | needs 2 → 1: +0.8 ± 1.1 (screen 1); 0 play insight: +0.2 ± 0.8 (screen 5, not kept) | earned sources, each fed by its board's component |
| Horizon's converter (the megaproject chain) | 1 cube + 2 energy → 1 antimatter (give 12, get 7) | 1 cube + 1 energy → 2 (give 7.5, get 14) | the main earned source: 1.8 antimatter a seat-game | a loss at the table before |
| Antimatter capstones (2 of 6) | none | Open Archive (Researcher economy) and Sanctuary (Protector economy) add 1 antimatter a round | 0.37 a seat-game | Matt: some capstones, not all |
| Cube engines (tier 1, Science) | none (no card made a cube) | Cryo Sample Vault (7 cr: 1 research cube a round), Field Assay Team (6 cr: Action 1 cube → remove 2 agents in one hex in reach; `values.json` overrides row +6) | +0.4 ± 1.5 (screen 4) | the cube ablation read 0.0 ± 2.0 with no cube engine |
| Biomia's converter | 2 cubes → 1 Unity (give 6, get 4) | 1 cube → 1 Unity | inert in 2-country play (Biomia is the 4th country) | a loss at the table |
| `adversary.stage_extra` (196) | d0: 2 more event cards (13 rounds) | **removed**: d0 deals as d1 (11 rounds) | d0 93.8 fresh without it | Matt: engines over dials |
| `compute_income` d0-d4 | 1, 4, 17, 22, 24 | **1, 4, 9, 28, 30** | Calibration below | last |
| `ambassador_income` d0-d4 | 0, 2, 2, 2, 3 | **0, 2, 3, 3, 3** | Calibration below | last |
| Eval's held antimatter (`w.antimatter`, agents) | 2 | **5** (about 8 credits; `values.json` 7) | Annihilate 3× as often; +0.2 ± 2.6 (screen 1); 2 / 3.5 / 5 within 3 points (screen 3) | Annihilate spends 2 at once: a seat must save |
| Ablation policy `no_spend` (agents, stats only) | none | 1 chips, 2 cubes, 3 antimatter: the seat never takes an action that spends it | Ablations below | the done-when's screens |

## Design check (give / get at `values.json`; the anchors in brackets)

| piece | resource | give | get | gain | who wants it | where it pays (8-10 rounds) |
|---|---|---|---|---|---|---|
| Annihilate | antimatter | 2 am 14 (14) | a hex cleared (2 agents and a troop: 16; the table's flat `destroy_all` 8) + 3 Sway 12 | +14 (+6 at the flat 8) | every board; the Protector most (Wall piles troops by its nodes) | from round 3-4 (Horizon complete by round 3 in 97% of games); once a round |
| Penning Trap Array, its Action | cubes, energy | 1 cube 3 + 1 energy 4.5 = 7.5 (10.5) | 2 am 14 (14) | +6.5 (+3.5) | the Researcher (stations' cubes; Science printed) | played round 4+. Card: pay 12.5, get 19.4 at the table; anchors 22 / 22 (1.0) |
| Antiproton Foundry, its Action | chips, energy | 1 chip 4 + 1 energy 4.5 = 8.5 (10.5) | 2 am 14 | +5.5 (+3.5) | the Builder (Mint, factories; Industry printed) | as above; table 12.5 / 17.6 |
| Horizon's converter | cubes, energy | 7.5 (10.5) | 2 am 14 | +6.5 (+3.5) | whoever has a Horizon city in reach | round 3+, once a round |
| Open Archive, Sanctuary | (capstone) | the track's last space (5 energy) | +1 am a round (7) beside what it did | | Researcher, Protector | reached rounds 5-8 (0.75 capstones a game) |
| Cryo Sample Vault | (cube source) | 7 cr + 1 play insight = 9.5 (the ledger's basis; 15 at the anchors with the keep) | 13.0 at the table (1 cube a round over the horizon) | +3.5; anchors 19 / 15 (1.27) | Researcher | from round 2 |
| Field Assay Team, its Action | cubes | 1 cube 3 (6) | 2 agents in one hex 5 (10) | +2 (+4); card 8.5 / 10.9 with its override | anyone holding cubes | each round with a cube |
| Biomia's converter | cubes | 1 cube 3 (6) | 1 Sway 4 | +1 at the table; **−2 at a cube's past-check anchor 6** (Sway has no anchor) | 4-country games | flagged, not repriced: inert in the reads |

**Antimatter, earn against payoff.** A seat with a source makes 2 a round (a plant's Action or Horizon's converter,
each once a round) from round 3-4; the capstones add 1 a round from round 5-8; the antimatter action adds 1 a round
for 1 Discord. Annihilate spends 2, once a round: a committed seat clears a hex a round for the last 4-6 rounds. The
earn cost is 7.5-8.5 a pair at the table (a plant, Horizon) or 2 Discord (the action); the payoff 20-28 a use.
Measured (fresh, strategist, a seat-game): antimatter gained 4.0-7.4 and spent 3.4-6.2 (r87: 0.4-1.3 and 0.06-0.21);
Annihilate 2.2 uses.

## Ladder (fresh, paired against r87; [`ladder-strategist.md`](ladder-strategist.md), [`ladder-greedy.md`](ladder-greedy.md))

| d | strategist r88 | r87 (same seeds) | gap | band | verdict | greedy r88 (r87) | rounds r88 (r87) |
|---|---|---|---|---|---|---|---|
| d0 | **93.8** | 89.1 | +4.6 | 90-100 | met, with no extra event cards | 91.0 (90.3) | 6.2 (9.3) |
| d1 | **69.9** | 71.3 | −1.4 | 70-80 | **unmet by 0.1** (SE about 2.2) | 70.1 (71.5) | 7.7 (8.6) |
| d2 | **64.6** | 58.8 | +5.8 | 55-65 | met (compute 9, seeds 44911-45054) | 61.8 (63.9) | 7.0 (7.5) |
| d3 | **45.1** | 45.1 | +0.0 | 40-50 | met | 51.4 (41.7) | 6.4 (7.1) |
| d4 | **33.8** | 33.3 | +0.5 | none | antimatter check | 43.8 (43.1) | 6.6 (7.1) |

432 games a level (144 seeds × 3 pairs), seeds 44767-44910; all levels +0.1 ± 1.2 against r87. d2 was read there at
compute 17 (55.1, +3.0 under r87); its final dial (compute 9) was screened on those spent seeds (60.6) and confirmed on
the next free seeds 44911-45054 (64.6, r87 58.8). **d1 sits between two integer steps:** agents 2 reads 69.9, agents 1
reads 78.5-79.6 (any compute 4-12) on 44767+ and 83.1 on the confirm seeds, and d1's compute moves it under a point
(screens 8-9). Agents 2 is kept: its read is a fresh read of exactly the final d1 config and nearer the band. The
level-1 steps move it by about 0 or about +10 too (screen 11-12, on the spent seeds): counters' bonus from level 2 +0.7
± 2.3, the corrupt cut from level 2 +9.5, that with counters dearer at level 1 +10.2. No dial within the rules lands d1
near 75; it needs a fractional lever (ideas, 3).

## Ablations ([`ablations.md`](ablations.md))

Final build, d3-d4, dev seeds 36832-36895, 192 games a level (paired against the full build):

| removed (the sim players never spend it; sources stay) | d3 | d4 | all |
|---|---|---|---|
| none | 41.7 | 42.2 | 41.9 |
| chips | 27.1 (−14.6) | 28.1 (−14.1) | **−14.3 ± 2.4** |
| research cubes | 34.9 (−6.8) | 37.5 (−4.7) | **−5.7 ± 1.8** |
| antimatter | 27.6 (−14.1) | 23.4 (−18.8) | **−16.4 ± 2.4** |

Before the cube engines, removing cubes changed nothing (0.0 ± 2.0); after them, −5.2 ± 1.8 (n3) and −5.7 ± 1.8
(final). The cube cards themselves are played 0.02-0.03 a game: the cube loss comes mostly from Horizon's converter
(cubes into antimatter) and the converters it shares.

## Resource use (fresh, strategist, per seat a game; [`resource-use.md`](resource-use.md))

| d | chips spent | energy spent | cubes spent | antimatter gained / spent | antimatter spent in: games / won games |
|---|---|---|---|---|---|
| d0 | 2.9 (r87 5.4) | 21.7 (34.8) | 2.8 (6.8) | 6.1 / 5.0 (1.3 / 0.15) | 100% / 100% (26% / 21%) |
| d1 | 3.9 (4.4) | 27.0 (30.5) | 4.8 (4.7) | 7.4 / 6.2 (0.9 / 0.21) | 100% / 100% (32% / 25%) |
| d2 | 3.2 (3.5) | 23.0 (24.3) | 3.0 (2.7) | 5.1 / 4.5 (0.5 / 0.12) | 99% / 100% (22% / 20%) |
| d3 | 2.7 (3.1) | 20.0 (22.6) | 1.9 (2.3) | 4.0 / 3.4 (0.4 / 0.09) | 97% / 99% (17% / 12%) |
| d4 | 3.0 (3.0) | 21.4 (22.7) | 2.1 (2.2) | 4.2 / 3.6 (0.4 / 0.06) | 98% / 100% (11% / 12%) |

What each buys (all levels): chips: Procure 1.27, stations 0.89, the corps' chip converters 0.94, the Foundry 0.01;
cubes: Horizon's converter 0.89, the cube converter 0.66, Cintamani's 0.63, Peer Review 0.26, threats 0.3; antimatter:
Annihilate 4.40, the ultimates 0.12. **Where antimatter comes from:** the antimatter action 2.83 (52%), Horizon's
converter 1.78 (33%), capstones 0.37, Particle Run 0.20, the plants 0.11. Games are shorter (d0 6.2 rounds, r87 9.3)
and spend less of everything else: the d0 drop in chips, energy and cubes is length, not substitution.

## Surprises and watch list

- **The cheap source supplies half the antimatter.** At held value 5 the agents take the antimatter action (1 Discord)
  about once a round; at 2 Discord it falls to 65-92% of games spending antimatter and wins −4.9 ± 1.1 (screen 10,
  information only: Matt's ruling keeps it cheap). Discord losses rose: d2-d4 0.34-0.42 of games (r87 0.25-0.38).
- **The plants are kept and seldom played** (kept 0.4 a game each, played 0.03-0.08; `keep_ok` low: unaffordable as
  the play phase begins). At 0 play insight nothing changed (screen 5). The agents' keep plan values them; the play
  doesn't come.
- **d0 games run 6.2 rounds** against the 8-10 target: Annihilate's 3 Sway tips Unity fast at d0.
- Screen 2-3's −8 against r87 on seeds 36800-36831 was the seed set: on 36832-36927 the same build read +2.6 ± 1.5.
- `no_spend` leaks when every legal action spends the banned resource (the policy then picks from all): rare in the
  action phase (a pass is always legal), possible in a pending choice.

## Ideas not built, ranked for Matt

1. **Make the plants the antimatter's main source**: a plant's Action at 3 antimatter, or the antimatter action at 2
   Discord (screen 10: −4.9 ± 1.1, antimatter still spent in 65-98% of games). Today the cheap action supplies 52%.
2. **An agent play rule for kept engines** (the plants are kept 0.4 and played 0.03-0.08 a game): review the keep
   plan against the play eval for Action buildings before the next screen.
3. **d1 between steps**: a fractional d1 lever (an agent income of 1 plus a d1-only threat item, or `counter_more_at`
   1) to land d1 near 75 rather than at 69.9 or 79-83.
4. **Longer d0 games**: Annihilate's Sway at 2 below d2, or the d0 cap back at 13, if 6.2 rounds is too short.
5. **Biomia's converter at the anchors** (1 cube → 2 Sway would gain at both) when 4-country games are read.
6. **More cube income cards** if cubes should matter beyond −5.7: only one card makes cubes.

## Speed

`per_decision.py out/r87 out/r88root` (c2 d2, seed 1000, one thread, best of 2, quiet box): strategist **0.85×** a
decision on 0.91× the options (8 games; 0.64× a game: games 0.74× as long); greedy **0.86×** a decision on 0.92× the
options (16 games). `speed.py check --update` set the baseline at r88.

## Checks

Fast lane and `cargo test-all` green; clippy clean; `aic-sim docs --check` current (`regen.py`); `npm run cards:fit`
green (8 top boxes under 7.5 pt, none new); `npm run cards:check` green after scenes for the four new cards;
`play_read.py`: the four new cards all played (none kept and never played); `board_read.py --powers`: Annihilate
1.3-2.8 a seat-game on every board, Synthesis still 0.00; `loop_smoke.py --run`: no timeouts, most plays of one card 6
(Joint Fabrication Pact). Not run: `bench/same_results.py` (no core rules file changed after the freeze; the d1-d2
dials are config, and their games were read fresh).

## Files

`overlay.txt` (the cards and corps, `apply_overlay.py`), `apply_boards.py` (Annihilate, capstones),
`pricing_notes.py`; readers `paired.py`, `resource_use.py`, `pieces.py`, `farmdir.py`, `tables.sh`; specs
`screen_spec.py`, `read_spec.py` and `experiments/farm/s200-*.json`, `f200-*.json`; tables `screens.md`,
`ladder-*.md`, `ablations.md`, `resource-use.md`, `resource-use-amd2.md`. Seeds: dev 36800-36831 and spent
36832-36927 (screens); fresh 44767-45054 spent (ledger next free 45055).

## For new players (r88's changes)

Antimatter is now the resource that wins hard games. Every board has a new power, **Annihilate**: once a round, pay 2
antimatter to remove every agent and troop from one hex you can reach and push Unity 3 steps. You earn antimatter by
building toward it: two new tier-2 plants turn a research cube (Penning Trap Array) or a chip (Antiproton Foundry),
with an energy, into 2 antimatter each round; Horizon's completed megaproject does the same for a cube and an energy;
the Researcher's Open Archive and the Protector's Sanctuary capstones add 1 a round. The old antimatter action still
gives 1 for a step of Discord. Two cheap Science buildings give research cubes a use: Cryo Sample Vault makes a cube
each round, Field Assay Team spends one to remove 2 agents. Biomia's converter takes 1 cube for 1 Unity. Level 0 no
longer adds two event cards (11 rounds at most, as levels 1-2); level 2's compute is 9 a round with 3 agents a
country, level 3's 28 and level 4's 30. **What resisted the summary:** nothing.

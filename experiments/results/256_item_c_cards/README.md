# 256: Item C, the card pass on r115

Base: main 4981702 (255's cleanup on 254's r115). **Rules change: r116, then r117** (bumped from the constant, 115 to 116 for the pass, 117 for a fix after
the freeze that moves no result on the frozen seeds, below).
Matt's plan C (2026-11-16, locked): more and bigger node, station, freeing and claiming cards, value-checked; 253's
and 254's picks (Item C's cards claim cheaply; a research-cube sink on cards; Campus and Commons repriced here).
Every edit is one overlay, [cards.overlay](cards.overlay) (`scripts/cards/apply_overlay.py`).

Files: [screen.md](screen.md) (dev screen, group arms), [read.md](read.md) (the fresh paired read), [trio.md](trio.md)
(amendment 11725: Positron, Wafer Fab, Antiproton blanked), [design-check.txt](design-check.txt). Scripts:
`arms.py` (the screen's group arms), `merge.py` (r115's native run and the pass's farmed run as one read for
`scripts/bench/read_lib.py`), `use256.py` (the Use tables). Specs: `experiments/farm/s256-screen.json`,
`f256-read.json`, `f256-trio.json`. The r115 arms ran r115's binary (main 4981702) natively on the same specs
(`out/256_item_c_cards/shards.py`): a deck change can't be undone by overlay (`cost` 99 keeps a card in the draft).

## The cards (give / get)

Pay = credits + keep (values.json: 2.5 tier 1, 6.5 tier 2; anchors: 4 and 12) + play insight + printed costs; get
at `values.json` (the effect ledger, `experiments/results/effect-ledger/`) and at the unit anchors
(docs/card-guidelines.md). An Action at 4 uses. An owned station at the anchors is the Build action's 2 chips (12); an
owned factory +1 chip income (19); a node 8 (values.json; its track step is paid on top, as by every route).

| Card | Change | values.json: pay → get | anchors: pay → get (ratio) |
|---|---|---|---|
| **claim_site** (op) | `claim_station` widened: a neutral station **or factory**; `near`: on or next to the target, no reach needed | row `claim_site` 12 (was `claim_station` 12); `near` × `claim_near` 0.5 | |
| Power Purchase Agreement (t1 event, 2 cr, 2 en) | claims a station **or factory** | 2 + 2.5 + 2 en → 12 | 2 + 4 + 9 = 15 → 12-19 (0.8-1.27) |
| Microgrid Lab Compact (t2 building, 18 cr) | Action: 2 energy, claim a station **or factory** | unchanged price | |
| **Open Research Campus** (t1 event) | 5 → **10 cr**: the station it opens on an empty printed site is **yours**, 1 ambassador on it (job-no-card Build a research station cleared) | 10 + 2.5 = 12.5 → open_station 5 + claim_site 12 + ambassador 6 = 23 | 14 → 12 + 8 = 20 (1.43) |
| **Industrial Commons** (t1 event) | 4 → **9 cr**: the factory it opens on a free zone is **yours** (no ambassador) | 11.5 → open_factory 3 + 12 = 15 | 13 → 19 (1.46) |
| The seven liberate cards (Nuclear Option, Power to the People, Forensic Audit, Root Access, Peacekeeping Mission, Popular Uprising's liberate half, Citywide Microgrid) | **claim rider**: claim a facility next to the city (free, then claim; nothing to claim, nothing lost) | +6 each (12 × 0.5) | +6-12 each (an owned station or factory at half) |
| **Relay Mast Rollout** (t1 program, 1 ins) | 2 → **3 nodes**, 6 → **9 cr** | 9 + 2.5 + 2.5 = 14 → 24 + 8 = 32 | 9 + 4 + 4 = 17 → 24 + 10 = 34 gross of the track steps (2.0; ~1.3 net of ~2 energy a step) |
| **Mesh Repeater Network** (new t1 building, Cyber, 6 cr) | Action: pay 1 energy, place 1 node | 8.5 → 10.8 | 6 + 4 + 4 × 4.5 = 28 → 4 × 8 = 32 (1.14) |
| **Research Reactor** (new t1 building, Science + Power, 8 cr) | Action: pay 1 research cube, gain 2 energy (the cube sink) | 10.5 → 18.3 | 8 + 4 + 4 × 6 = 36 → 4 × 9 = 36 (1.0) |
| **Survey Drone Fleet** (new t2 building, Cyber + Science, needs 2 Science, 12 cr) | Action: pay 1 research cube, place 2 nodes within 2 (the cube sink, a node card) | 18.5 → 23.8 | 12 + 12 + 4 × 6 = 48 → 4 × 16 = 64 (1.33) |
| **Orbital Relay Launcher** (new t2 building, Power + Cyber, 12 cr) | Action: pay 1 antimatter, place 2 nodes, gain 3 energy (the antimatter use) | 18.5 → 21.6 | 12 + 12 + 4 × 18 = 96 → 4 × 29.5 = 118 (1.23) |
| Hostile Buyout (parked P1) | agent value only: `own` no longer adds `liberate_align` to the city bucket; `liberate_own` 13 (half of a flip 15 + a claim 12) seen with or without a city | | |

**Where each matters.** Campus and Commons from round 2 (a printed site or free zone in reach; owned, a station makes
a cube a round and a factory a chip, so round 2-5 claims pay back 4-7 times). The claim rider at mid-game, when the
liberate cards come out: every city starts with a neutral factory and station next to it
(`every_city_starts_with_a_neutral_factory_and_a_neutral_station_next_to_it`), less what was claimed or corrupted
since. The node buildings every round from the round they are built (a second or third node a round, beside
Expand). Research Reactor and Survey Drone Fleet once cubes flow (a station's cube a round, Cryo Sample Vault, the
completion payouts). Orbital Relay Launcher from the round a plant runs. **Caps:** every Action once a round; the
sites by the printed sites and free zones; the rider by the facilities next to the city. **Loops:** Research Reactor
turns a cube into 2 energy; no card turns energy into cubes once a round at better than 1:1 (Cyclotron Lease makes
antimatter), so no closed loop; Orbital Relay Launcher's 3 energy can't make its antimatter back at a plant (2 energy
for 1) and place 2 nodes in the same round more than once.

**Card fit:** five liberate faces print the rider below 7.5 pt (`npm run cards:fit`: Popular Uprising's Action box
5.4 pt, Root Access and Peacekeeping Mission 6.3, Nuclear Option 6.9; at r115 they fit), the rider already cut to
"Claim a facility next to it." (the house term): a recorded exception, guideline 9, for a later face pass.

## The dev screen (the judge, dev seeds 36800-36831, three pairs, d0-d4, 480 games an arm)

r115's binary natively against the pass (farmed), and each group restored to r115 by overlay ([screen.md](screen.md),
[screen-use.md](screen-use.md)). Built before the diff review moved the rider's agent value to the city bucket.

| arm | d0 | d1 | d2 | d3 | d4 | all | gap to r115 | gap to the pass |
|---|---|---|---|---|---|---|---|---|
| r115 | 90.6 | 92.7 | 91.7 | 66.7 | 34.4 | 75.2 | | +5.8 ± 2.3 |
| **the pass** | 93.8 | 88.5 | 81.2 | 60.4 | 22.9 | 69.4 | -5.8 ± 2.3 | |
| no sites (Campus, Commons as r115) | 93.8 | 89.6 | 83.3 | 58.3 | 29.2 | 70.8 | -4.4 ± 2.4 | +1.5 ± 1.6 (d4 +6.2 ± 3.9) |
| no rider | 95.8 | 90.6 | 83.3 | 61.5 | 27.1 | 71.7 | -3.5 ± 2.4 | +2.3 ± 1.2 (d4 +4.2 ± 2.1) |
| no nodes (Relay Mast as r115, Mesh blanked) | 89.6 | 91.7 | 88.5 | 57.3 | 30.2 | 71.5 | -3.8 ± 2.4 | +2.1 ± 2.0 (d2 +7.3, d4 +7.3 ± 4.7) |
| no sinks (the three new sink buildings blanked) | 97.9 | 92.7 | 84.4 | 62.5 | 31.2 | 73.8 | -1.5 ± 2.3 | +4.4 ± 2.0 (d4 +8.3 ± 5.3) |

Every group costs the judge a little and none alone carries the loss; the sinks most (2.2 SE). Use on the screen:
owned stations and factories made 1.99 → 2.66 a game (rider claims 0.48, card claims 0.88: Power Purchase Agreement
now claims factories, 0.75); cards' share of nodes 7% → 9%; research cubes spent 57% → 62%, antimatter 42% → 42%: the
sinks are kept 0.32-0.37 a game and used 0.07-0.27 times, too little to bind either. **Not dialed** (bands are soft):
the fresh read measures the pass as designed.

## The fresh read (49020-49067, three pairs, d0-d4, 48 seeds a cell; r115 natively on the same seeds)

[read.md](read.md) (read_lib), [read-use.md](read-use.md) (use256.py), [design-check.txt](design-check.txt).

| | d0 | d1 | d2 | d3 | d4 | all | gap ± SE |
|---|---|---|---|---|---|---|---|
| judge r115 | 91.7 | 91.0 | 86.8 | 64.6 | 22.9 | 71.4 | |
| **judge r116** | 91.7 | 89.6 | 80.6 | **47.9** | 21.5 | 66.2 | **-5.1 ± 1.8** (d2 -6.2 ± 4.0, d3 -16.7 ± 5.0, d4 -1.4 ± 4.2) |
| greedy r115 | 87.5 | 81.2 | 85.4 | 48.6 | 26.4 | 65.8 | |
| greedy r116 | 87.5 | 80.6 | 80.6 | 54.2 | 17.4 | 64.0 | -1.8 ± 2.0 (d3 +5.6 ± 5.2, d4 -9.0 ± 4.6) |
| full oracle r116 (d2-d3, 12 seeds a level and pair) | | | 100 | 100 | | 72/72 | 10.2 rounds |

Bands (judge, soft): d0 ≥ 90 met, d1 80-95 met, d2 75-90 met, **d3 55-70 missed by 7.1** (47.9; r115 on these
seeds 64.6, inside it), d4 unbanded. **The judge's d3 losses 51 → 75 of 144:** ending on a lost node 28 → 42, on an
empty deck 23 → 31. Rounds 8.96 → 9.03 (judge), 9.20 → 9.46 (greedy); d0 10.0 rounds, at the band's top
(design_check). Greedy's d3 rises and its d4 falls: the two agents disagree in sign at d3, so the judge's d3 drop is
the judge's play of the new cards more than the cards' power (the oracle still wins every d2-d3 seed).

**Use (fresh, judge; greedy alike):**
- Owned stations and factories made **2.00 → 2.93 a game** (254 on r115: 2.33): card claims 0.52 → 1.00 (Power
  Purchase Agreement now takes factories, 0.84 of them), the rider 0.50 (0.31 factories, 0.19 stations), Campus 0.07
  and Commons 0.14 sites opened and owned; the Claim action 0.09 → 0.05 a game (0.03 a seat-game: still unused, its
  usage flag stays).
- **Cards' share of nodes 7% → 9%** (0.80 → 1.04 a seat-game; Relay Mast Rollout 0.61 plays a game, Mesh Repeater
  Network 0.12 uses, Survey Drone Fleet 0.05, Orbital Relay Launcher 0.26 uses of 2 nodes).
- **Resources spent of gained (judge):** credits 103%, energy 85% (89%), insight 82%, ambassadors 79% bind; chips 60%
  loosely; **research cubes 57% → 60% and antimatter 43% → 46% still don't bind** (Research Reactor 0.23 uses a game,
  Survey Drone Fleet 0.05, Orbital Relay Launcher 0.26). Energy sinks (amendment 11725): Mesh Repeater Network's
  Action, Orbital's 3 energy out, Research Reactor's 2 energy in; energy spent 34.3 → 34.4 a seat-game.
- Every new or changed card is kept and played (`play_read.py`: none kept and never played; design_check's
  kept-not-played is Exascale Training Cluster, 0.35 kept and 0.08 played, untouched and flagged at 254 too).
  `loop_smoke.py` on the read: no timeout, most plays of one card in a game 5.

**design_check (fresh r116 against fresh r115):** no job-no-card flag (Build a research station: Open Research
Campus, the action 2.0x the card); no new standard-action flag (Make antimatter 0.04, Claim a site 0.03: both on
r115 too); new: kept-not-played Exascale (above), d0's 10.0 rounds; gone: Researcher Audit's dominant flag.

**r117 after the freeze:** the full test lane caught Mesh Repeater Network's Action offered when its 1 energy left
the node's track step unaffordable (paid for nothing); the fix (a building Action's energy counts before its node's
step, `branch_blocked`) bumped 116 → 117. The frozen read's seeds replayed on r117 (`scripts/farm/compare.py`,
`out/256_item_c_cards/fresh-r117.jsonl`): 275 of 1,440 games differ, every one only in `options_total`; no result
moved (paired gap +0.0 ± 0.0), so every table here holds for r117 ([read-r117.md](read-r117.md), arm still named r116).

## Which changes moved d3 and d4

**Fresh group arms** (49164-49211, the judge, d3-d4, three pairs, 288 games an arm; r115 natively on the same
seeds; [groups.md](groups.md)): r115 64.6 / 31.2; the pass 54.2 / 22.9, **-9.4 ± 3.5** (d3 -10.4 ± 4.8, d4 -8.3 ±
5.0): a second fresh set agreeing with the read. Each group restored: sites -9.4 ± 3.5, rider -11.8 ± 3.6, nodes
-10.4 ± 3.4, sinks -7.3 ± 3.7 against r115; against the pass every group is within 1 SE (sinks +2.1 ± 3.1, rider
-2.4 ± 1.9). **No group moves d3 or d4 measurably; the loss is in what every arm shares.**

**Probe of the shared part** (dev seeds, d3-d4, 192 games an arm; [probe.md](probe.md), `probe.py`): the pass -8.9
± 4.6; `core` (every group restored: the claim cards' widening to factories, Hostile Buyout's agent value, the new
cards drafted but blanked) -5.2 ± 4.8; `core` with Power Purchase Agreement and Microgrid Lab Compact blanked -11.5 ±
4.7. Taking the claim cards out costs more, so their widening is not the driver. What is left in `core` (Hostile
Buyout's value, 0.16-0.21 plays a game; four dead cards in the draft, which the pass's own arm doesn't have) is not
separable at this SE. One reading fits all three sets: the loss grows with the weak or dead cards in the draft (the
pass adds four buildings the judge keeps 0.2-0.5 a game and uses 0.05-0.26 times; each blanked arm adds dead ones;
the arm with the most blanks, `core-noclaim`, reads worst), so the new buildings would cost the judge the cards they
displace. **Unresolved:** the d3 drop is the judge's (greedy's d3 rose 5.6 on the read; the oracle
wins every d2-d3 seed), real on two fresh sets, and not one group's.

## Amendment 11725: Positron, Wafer Fab, Antiproton (the engine, d2-d3, 49068-49163, 96 seeds, 576 games an arm)

[trio.md](trio.md). Each blanked against the deck: Positron Storage Ring -0.9 ± 1.6, Wafer Fab Line -2.4 ± 1.8,
Antiproton Foundry -0.9 ± 1.7 (the engine wins **less** without each). 246's -2.4 to -3.1 (wins more without them)
**does not confirm** on fresh seeds: no reprice.

## For new players

Cards now take facilities for you. Power Purchase Agreement and Microgrid Lab Compact claim a neutral research
station or factory; Open Research Campus opens a station on an empty station site and it is yours (an ambassador
on it), Industrial Commons does the same with a factory. Most cards that free a city also claim a facility next to
it, even one out of your reach. Relay Mast Rollout places 3 nodes; Mesh Repeater Network places one each round
for 1 energy; Survey Drone Fleet turns a research cube into 2 nodes up to 2 away; Research Reactor turns a cube into
2 energy; Orbital Relay Launcher turns an antimatter into 2 nodes and 3 energy. No rule resisted a summary.

## Speed and the parked edits

`per_decision.py`, r115 against r116, 6 games (c2, d2, seed 1000, one thread; the box loaded, ratios only; other
rules, so ms a decision): the judge 1.35x a decision at 1.27x the options (the new Actions and the wider claim
targets), greedy 0.88x at 0.97x. `speed.py check` gives no verdict across a revision.
**P1** (Hostile Buyout's agent value): played 0.21 → 0.16 a game (judge, fresh), 0.17 → 0.13 (greedy); no loop,
never kept-and-unplayed. **P2:** design_check credits it to Liberate a station and Liberate a factory.

**Deck:** 201 → 205 cards (tier 1 116 → 118 buildings and programs, tier 2 67 → 69): four buildings added, no cuts.
**Not done (room):** the "host a second city" card (a per-player host cap, a state field; flagged), Dyson
kept-not-played, Tidal Grid's converter, Citywide Microgrid's halved plays (its rider is this pass's only change).

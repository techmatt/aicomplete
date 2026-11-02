# Screen 068: Builder and Protector boards, ambassadors that stay, scarcity (r28)

Planner at d2 unless a line says otherwise, `deals` 4, `lead` 0. Specs: `experiments/screen-068.json`, `experiments/band.json`,
`experiments/gate.json`, `experiments/gate-close-068.json`. Screen seeds 21000-21700; band 22000 (rules before the last lever) and 22100; gate 23000.
Numbers are a win % at 24-48 seeds, ±7-11 a cell, unless a line says more.

## The boards

Each space's income is income a round once uncovered; `cost` is the energy every placement pays there, on top of the power's.

**Builder.** Printed: 10 credits, 4 insight, the Industry tag; 15 starting credits; 5 slots.

| Space | Economy track | Card-play track |
|---|---|---|
| 1 | +1 insight (on the board at setup) | +1 insight |
| 2 | +2 credits | keep upgrade, Industry tag |
| 3 | +1 credit | +1 insight (cost 1) |
| 4 | +2 credits, Power tag (cost 1) | +1 credit, Science tag (cost 2) |
| 5 | nothing (cost 2) | +1 insight (cost 2) |
| 6 | +1 credit (cost 3) | Cyber tag (cost 3) |
| 7 | **Full Capacity**: 1 chip a round per 2 Industry tags (cost 5) | +1 insight, **Master Plan**: buildings cost 2 credits less (cost 5) |

**Protector.** Printed: 9 credits, 4 insight, the Security tag; 25 starting credits; 3 slots.

| Space | Economy track | Card-play track |
|---|---|---|
| 1 | +1 insight (on the board at setup) | +1 insight |
| 2 | +1 energy, +1 credit | keep upgrade, Security tag |
| 3 | +2 credits | +1 insight (cost 1) |
| 4 | +2 credits, Diplomacy tag (cost 1) | +1 credit, Cyber tag (cost 2) |
| 5 | +2 credits (cost 2) | +1 insight (cost 2) |
| 6 | +1 credit (cost 3) | Diplomacy tag (cost 3) |
| 7 | **Sanctuary**: the adversary's spread skips your presence (cost 5) | +1 insight, **Aegis Doctrine**: each round, stop the first destruction in a country you're in (cost 5) |

**Reasoning.**

- *Two tracks, one choice each placement.* Economy pays credits now; card-play pays insight and keeps, which is what turns credits into plays. Insight limited plays in every screen (2-3 cards a round for the team), so the card-play track is the one that spends the other.
- *Printed tags are the board's identity; revealed tags open the card-play track's tier-2 cards* (a third of cards with `needs` want one tag): Power on the Builder's economy track feeds Overcharge's cards, Diplomacy on the Protector's feeds Accord's. They count for `needs` only, so a seat can't sacrifice or discount with them.
- *Energy is the brake.* Step costs run to 5 (sum 11 on a track) so a capstone is a target, not a by-product. At 066's costs a seat filled a track by round 6; at these the first capstones land in rounds 8-10 and a game sees about 1.3, against the design's "around round 8 at 1-2 placements a round". The Builder's space 5 is blank: its chips left over at 4-6 a seat, and the +1 chip there was cut.
- *Capstones use standing rules that already exist,* so a player knows them from cards. Full Capacity turns the tags a Builder hasn't sacrificed into chips; Master Plan is the wide board's discount; Sanctuary and Aegis Doctrine extend Wall's idea, keeping ground once held. No capstone adds income in credits, which scarcity rules out. Antimatter left the tracks: agents ended with 2-5 they never spent.
- *The Protector carries more starting credits and more track credits, the Builder fewer.* That is also what balances the sizes: c3 seats two Builders and one Protector, so moving credits toward the Protector lifts c2 and lowers c3 (14 / 26 read 79 / 60; 16 / 24 read 67 / 78; 15 / 25 was frozen between them).

## What simulation moved

| Screen | Seeds | c2 | c3 | Read |
|---|---|---|---|---|
| As designed: 30 / 38 credits, ambassador 6, tracks to cost 3 | 21000, 32 | 100 | 100 | 7 rounds, 15-21 ambassadors bought, 0 corruptions |
| Engine: credits 20 / 28, ambassador 8 | 21100, 21 | 76 | 67 | `ambassador_credits` is the strongest lever |
| 20 / 28, ambassador 8, step costs to 4 | 21200, 24 | 75 | 87 | |
| ambassador 10 | 21200, 24 | 58 | 67 | |
| credits 12 / 20 | 21200, 24 | 62 | 79 | |
| 16 / 24, ambassador 9 | 21200, 24 | 54 | 67 | |
| same, costs to 5, Mint 3 | 21400, 24 | 67 | 67 | |
| all 126 cards 1 credit dearer, ambassador 8 | 21400, 24 | 67 | 62 | no fall in unspent credits: rejected |
| 16 / 20, Protector space 4 +1 credit | 21500, 48 | 60 | 75 | then 50 / 69 on band seeds 22000, 96 |
| 12 / 16, more track insight | 21500, 48 | 48 | 56 | credits 2 lower; paired -12 ±8, -19 ±8: rejected |
| 16 / 24, Protector space 4 +2 | 21700, 48 | 67 | 78 | |
| 14 / 26, Protector space 4 +2 | 21700, 48 | 79 | 60 | |
| ambassador 8 at 16 / 20 | 21700, 48 | 60 | 77 | |

## Band

Fresh seeds 22100, two sets of 48 a cell, on the frozen build (`release` profile, the rules the gate ran):

| | set 22100 | set 22148 | 96 |
|---|---|---|---|
| c2 | 73 | 73 | **73 ±4.5** |
| c3 | 77 | 73 | **75 ±4.4** |

Both in 60-75; c3 at the top edge. Ends at c2: purge 57, Unity deck 13, Discord deck 18, presence 6, cap 2. Rounds 8.6 / 8.9. The band before it (16 / 20 credits, Protector space 4 +1, seeds
22000, 96 a cell) read 50 ±5 / 69 ±5: that build was rejected, and the last lever moved 1 starting credit from each side and +1 credit on one space.

## Gate

`experiments/gate.json` (gate-068) on fresh seeds 23000: 28 checks a size, `never_tag` 1-6 dropped to fit. Early stop from 12 seeds at 2 SE, cap 48 (the default of question 1 to Matt, unanswered in
the window): 1,320 of 2,784 games, 95 min on 8 threads on the `fast` profile, the frozen source. Planner on these seeds, 96: 72 ±5 / 69 ±5, a third read of the band.

**Close-out (Matt, 2558 and 2670):** every row within 2 SE by gap against paired SE, whatever stopped it, 18 rows, filled to 48 and run to 96 with no early stop
(`scripts/bench/gate_close.py`, specs in `experiments/gate-close-068/`). None dropped. 96 paired seeds:

| variant | check | seeds | win % | gap to its agent | paired se | gap / se |
|---|---|---|---|---|---|---|
| c2 | planner:abandon=1 | 96 | 45 | +27.1 | 6.2 | +4.4 |
| c2 | planner:always_max_presence=1 | 96 | 71 | +1.0 | 5.6 | +0.2 |
| c2 | planner:never_remove=1 | 96 | 69 | +3.1 | 4.3 | +0.7 |
| c2 | planner:never_sacrifice=1 | 96 | 60 | +11.5 | 5.3 | +2.2 |
| c2 | planner:never_second_presence=1 | 96 | 67 | +5.2 | 5.6 | +0.9 |
| c2 | planner:no_powers=1 | 96 | 62 | +9.4 | 5.9 | +1.6 |
| c2 | planner:science_board=1 | 96 | 46 | +26.0 | 6.0 | +4.4 |
| c2 | planner:science_rush=1 | 96 | 46 | +26.0 | 6.0 | +4.4 |
| c2 | planner:tier2_rush=1 | 96 | 65 | +7.3 | 6.3 | +1.2 |
| c3 | planner:always_max_presence=1 | 96 | 55 | +13.5 | 5.9 | +2.3 |
| c3 | planner:free_build_rush=1 | 96 | 54 | +14.6 | 6.1 | +2.4 |
| c3 | planner:never_remove=1 | 96 | 58 | +10.4 | 5.2 | +2.0 |
| c3 | planner:never_sacrifice=1 | 96 | 52 | +16.7 | 6.0 | +2.8 |
| c3 | planner:never_second_presence=1 | 96 | 67 | +2.1 | 5.9 | +0.4 |
| c3 | planner:no_powers=1 | 96 | 56 | +12.5 | 5.6 | +2.2 |
| c3 | planner:science_board=1 | 96 | 42 | +27.1 | 6.0 | +4.5 |
| c3 | planner:science_rush=1 | 96 | 45 | +24.0 | 5.7 | +4.2 |
| c3 | planner:tier2_rush=1 | 96 | 68 | +1.0 | 5.0 | +0.2 |

**No check is ahead of the planner; the done-when (not ahead by more than 1 SE) holds at every row.** Closest: `tier2_rush` +1.0 at c3, `always_max_presence` +1.0 at c2, `never_second_presence` +2.1 at c3,
`never_remove` +3.1 at c2. Those four read level, not behind: a seat loses little by those policies. `no_powers` is only 9-13 behind: the sacrifice powers are worth little to a planner, which
matches their use above.

**The rest stopped clearly behind** (gap past 2 SE, seeds played after each name): c2 `free_build_rush` 24, `tier1_only` 24, `wall_turtle` 16, `ignore_threats` 16, `keep_k` 12, `only_tag=5` 12, `no_counters` 12,
`unity_farm` 12, `ambassador_mass` 12, `keep_all` 12, `no_presence` 12, `only_tag=1,2,3,4,6` 12, `events_only` 12, `turtle` 12, `free_only` 12; c3 `wall_turtle` 16, `tier1_only` 16, `keep_k` 20, `only_tag=5` 16, `no_counters` 20, `ambassador_mass` 12,
`unity_farm` 12, `keep_all` 12, `no_presence` 12, `only_tag=1,2,3,4,6` 12, `events_only` 12, `ignore_threats` 16, `turtle` 12, `free_only` 12, `abandon` 12. `free_build_rush` needs no fallback to "N credits less": it loses ground (42 at c2, 54 at c3).

## Unspent resources

Left when a seat's last action phase ended, planner, band seeds, mean a seat. Flag: credits 12, chips 4, energy 4, antimatter 2, ambassadors 3 (about a round's income of each; `docs/principles.md`).

| Cell | Seat | Credits | Chips | Energy | Antimatter | Ambassadors |
|---|---|---|---|---|---|---|
| c2 | Builder | 13 ! | 4 | 7 ! | 0 | 0 |
| c2 | Protector | 17 ! | 2 | 6 ! | 0 | 0 |
| c3 | Builder | 17 ! | 5 ! | 7 ! | 0 | 0 |
| c3 | Protector | 18 ! | 2 | 7 ! | 0 | 0 |

**Credits and energy end high, by a little.** Antimatter and ambassadors are scarce now (r28's first build left 2-5 antimatter, and 15 ambassadors bought a game). The cause of credits: insight is what binds. A team plays 2-3 cards a round on 5-8 insight a seat,
keeps taking most of it, so at 14 credits a seat a round income outruns plays. Cutting supply cuts wins: 12 / 16 starting credits ended 2 credits lower at 48 / 56, under the band, and 126 cards 1 credit dearer moved the pile by 1. What would clear it is less printed income, which amendment 2558 point 1
leaves as printed. The cause of energy: 2 placements a round is the power cap and cities pay energy whatever a seat places, so a seat with its steps paid has no other sink but Bulwark and cards. Both are about half a round's income above the flag.

## What agents do with the powers

Planner, band seeds, uses a game, c2 / c3 (c3 has two Builders):

| Power | c2 | c3 | When |
|---|---|---|---|
| Builder Deploy | 7.1 | 14.2 | every round to round 8 |
| Buy In | 1.6 | 3.0 | rounds 1-2 |
| Mint | 0.2 | 0.7 | scattered |
| Scrap | 2.5 | 3.6 | rounds 1-2 and again from round 7 |
| Overcharge | 1.4 | 2.6 | rounds 1-4, then thin |
| Rush Build | 0.02 | 0.04 | almost never |
| Protector Deploy | 7.5 | 7.7 | every round to round 8 |
| Accord | 2.2 | 2.2 | rounds 2-6 |
| Bulwark | 3.2 | 3.6 | steady from round 2 |
| Stand Down | 0.7 | 0.6 | scattered |
| Recovery | 0.3 | 0.3 | from round 8 |
| Shelter | 0 | 0 | never |

The design's "late-game liquidation" shows only partly: Scrap and Overcharge also fire early, on starting and cheap cards. The expensive powers are close to unused. Three
tags are rare to hold spare (10.7 tags sacrificed a game in all), and Shelter's value lands only in a rollout where the harm is due: the search scores them at full depth at the top
level since 068, and they still lose to the cards they would burn. Capstones: 1.3 / 1.4 a game, first in round 8-10; the eval's `capstone` weight is what pulls the last steps.
Wall: troops took 6.1 / 3.9 steps a game next to a wall.

**Wall as a barrier isn't learned.** The same 16 seeds with Wall switched off (`ais.protector.innate.passives=[]`, seeds 21900, c2): engine 56 / 50, planner 50 / 62; troop-destroyed pieces 4.4 / 4.4 and 5.6 / 5.2. Engine: 7 of 16 games
play identically to the end, so Wall changed no march there (rules allow it: an agent never placed for it). Planner: 4 of 16 identical; the rest first differ at ordinary choices and cards. `wall_turtle` (forced to place only near troops) loses ground. Neither agent places to block, and
the rule barely bites: a troop needs only a hex next to its target, and presence sits next to the facilities it guards. The eval has a capstone term and no barrier term.

**Followers barely played.** Liaison Corps 0.03 a game, Open Assembly 0.07. Ambassadors come from Consulate Tower's Action (1 credit, 1 ambassador: used 1.0 / 1.8 a game), Embassy Compound
and the project (6.9 / 9.6 at 9 credits). The Followers cycle is priced but not chosen, and Consulate Tower undercuts it. Not moved after the freeze.

## Ambassador sites against the rule

Consumed for a permanent change, stays for a position. **Consumed:** Align; claiming a neutral station or factory; freeing a city; `Op::Sacrifice`; a threat's `LoseAmbassador`; absorbing a destruction.
**Stays:** Remove's evictor; on a base (each production step); on a factory (neutral, player-owned or adversary-owned); on a corrupted station (diverts its cube to the player); on a node; on a city or lone presence, waiting; Move and every card move.
**Adversary:** on a lone presence, stays until removed (design); turning into corruption, consumed (design); on a base, factory, clean or aligned city, owned or corrupted station, used up when it diverts; on a neutral station, waits; on a corrupted city, stays.

Sites that don't fit cleanly, and the call made:

| Site | Call |
|---|---|
| Move or a card's move onto adversary ambassadors (`place_ambassador`) | Cancels 1 for 1, consuming the mover, though the table says Move stays. Moved ambassadors that evicted would clear the board at 1 credit a hop. |
| An adversary arrival on a standing player ambassador (`arrive`) | Cancels it. A position lasts until answered, and it is what keeps pieces from saturating. |
| Absorbing a destruction (`destroy_piece`) | Consumed: the piece it saved is the permanent change. |
| Adversary ambassadors that divert a base, factory, city or station | Used up, though by the principle they hold a position. The table names only presence and corruption, and the build's pressure is calibrated on them leaving. |
| Corruption wiping waiting player ambassadors | Consumed by the adversary's change, as since 024. |
| A second adversary ambassador spreading onto a lone presence | Makes a node, both ambassadors consumed: the one exit from "stays until removed" other than removal. |
| "A card's spend an ambassador" | No card prints this today: the row has no site. |
| Remove's source | Supply only (the design's "capped by supply"): a hop from the board would make one ambassador evict every round. |
| An evictor on a hex with no piece | Stays there; adversary ambassadors can stand there too. |
| A corrupted station | Player ambassador takes the cube (the table's "diverts its output"); an owned station has nothing to divert. |

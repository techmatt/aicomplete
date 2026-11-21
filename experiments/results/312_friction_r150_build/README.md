# 312: 309's friction picks built (r150 → r151)

Matt's picks on [rulebook-friction.md](../../../docs/rulebook-friction.md)'s table (309, r150), D136-D138 and 307's
open picks, as one rules revision: `RULES_REVISION` 150 → 151 (bumped from the constant).

## What was built, row by row

| Row | Pick | Built |
|---|---|---|
| 1 | Escalate takes factories every round (Commandeered Plants folded in) | `adversary::escalating` adds factories to last round's types (round 1 included; a factory escalates once after a factory card, not twice); the Militarist prints no rule (`rules: []`), and `board_rule` and a board Escalate are gone (`Content::check` refuses one) |
| 2 | Threat tokens on the lowest-numbered site of each type not yet the adversary's | `threats/near.rs` `place_tokens`: lowest number, agents not read; two tokens of one threat never share a hex (unchanged) |
| 3 | Discount pips on the tag dials | in words: the aid's *Tracks and markers* (numbers from `cards.science_every`, `industry_every`) and rulebook §2 / §7 describe the printed pips; display only; the web board's tag counts are unchanged |
| 4 | "Move the remainder onto the next project's track before you slide" | the aid's round, step B; rulebook §17 |
| 5 | A Unity floor | a new op, `floor`: "If there are none / If nothing comes off / If no city, research station or factory qualifies, every player gains 2 credits instead", on all 69 Unity tiers (`scripts/content/floors_312.py`); it reads what the per_site (sites), remove_where (pieces off) or prohibit (sites covered, not hexes) before it counted (`effects::Ctx::counted`; a `max` 0 blank counts nothing); values.json `floor` 0.5. A prohibition whose condition names "or a Node" almost always covers a site, so its floor rarely pays |
| 6 | "Its harm, then 1 agent on its anchor" | the aid's round, step F; rulebook §10 |
| 7 | Move your energy marker as a node goes down | the aid's round, step E; rulebook §5 / §9 |
| 8 | Zone arrows and "push, then make 1" printed | the aid's *Zone arrows*, rulebook §2 / §6; the web map already draws the arrows, and its zone hover now says "push, then make 1" |
| 9 | "Arrives ready: from your supply, placed by a card or power, converted" | the aid's *What spends an ambassador*; rulebook §8 |
| 10 | Keep the used markers | rulings.md |
| 11 | Name the halves; "(this card counts)"; Incursion II's tier and country | a two-half Action prints "Action: one of: (A) ...; OR (B) ..." (`face.rs`); Incursion I-II name the tier (this round's adversary card's first threat's) and the country order; **the self-trigger mark is not on the faces**: "(this card counts)" took six token buildings' top boxes to 5.8-6.9 pt and "(this too)" four of them to 6.3-6.9 (`npm run cards:fit` needs 7.5), so the rule is a house term in the aid's *Words on cards* ("When you play a X card: playing this building counts too when it carries X") and rulebook §7 |
| 12 | (done by 309) | |
| 13 | A one-page adversary-turn checklist | rulebook, the end of §6 |
| D136 | A cleanse frees a station or factory as Liberate does | one home, `board::free_site` (agents off, cube off, Unity: `free_city` or `free_facility` 2), used by `liberate_site`, a card's `cleanse` and the Unity card Reclamation's cube removal (`remove_where` cubes); left neutral. The 2 Unity is "as Liberate does" read whole: values.json's `cleanse` 4 → 9.333 and `cleanse_station` 10 → 15.333 and design_check's `station_freed` / `factory_freed` anchors follow (+2 Unity at rates.sway) |
| D137 | Peacekeeping Mission and Siegebreaker Reactor offer only cities they clear | `effects/targets.rs` `target_hexes_before`: a target whose troops on and next to it exceed the removal before its Liberate is no target |
| D138 | Leave | nothing |
| D139 | No forecasts in the cap's last round | `adversary::no_beat_ahead`: no Escalate arrows and no "completes next compute step" once the deck and the next card are out and this round's beat has run |
| D140 | A Redeploy reads as a move; "(2nd of 2)" | the turn panel sums equal adds and takes of one piece as "N ... moved" (`web/src/player/view.ts`); a hex's repeated Strike question says "(2nd of 2)" (`board/troops.rs`) |
| 307 | Shared screen checks; `LoseNodeOr` on one path | `web/scripts/screen-checks.mjs`, imported by `sweep.mjs` and `playthrough.mjs` (playthrough now fails on TODO/FIXME, stale words and page scroll); `LoseNodeOr` always takes `new_ops::lose_node_or` (same options and order; the count-1 prompt's amount reads "1 chip", not "1 chips"); values.json overrides stay out of `one_use` |

## The read: r151 against r150

The judge, `experiments/farm/read.json` (two seats, the three pairs, d0-d3), fresh seeds 51456-51551 (96 a cell, two
standing sets, 288 games a level), paired: r150 is main at d43e9397 (`farm-seeds --at`), r151 this build at 5b05a9d5.
Readers: `bench/ladder.py --measures`, `bench/read_lib.py`.

| Level | r150 | r151 | Paired gap | Band (soft) | Moved > 3? |
|---|---|---|---|---|---|
| d0 | 95.8 | 97.2 | +1.4 ± 1.3 | 90-100, met | no |
| d1 | 93.8 | 91.0 | −2.8 ± 1.7 | 80-95, met | no |
| d2 | 89.2 | 89.6 | +0.3 ± 1.9 | 75-90, met | no |
| d3 | 72.6 | 81.6 | **+9.0 ± 2.4** | 55-70, unmet (r150 already above it) | **yes** |

306 read 96 / 97 / 90 / 74 at r149. Measures at d3, r150 → r151: agents placed 58.0 → 53.7 a game (by Infiltrate 25.5
→ 24.3), corruptions 11.1 → 10.3, compute made 223 → 213, rounds 7.12 → 6.84; d0-d2 within a few percent. Losses by
deck-out 0.099 → 0.081 a game. Not dialed (bands soft).

**What moved d3: no one change.** A scratch build (`e1f181d6`, never merged; `adversary.scratch_off` bits switch each
r151 change back to r150's) farmed d3 on the same seeds (`s312attr`, 288 games an arm, paired; its all-on arm
reproduces r151's 81.6 exactly):

| Arm (r151 with one change back to r150's) | d3 win % | Gap to r151 |
|---|---|---|
| r151 | 81.6 | |
| Escalate's factories doubled again after a factory card (row 1) | 80.2 | −1.4 |
| Threat tokens by most agents (row 2) | 79.5 | −2.1 |
| A cleanse leaves agents, no Unity; Reclamation likewise (D136) | 79.2 | −2.4 |
| Peacekeeping / Siegebreaker may target cities they can't clear (D137) | 81.6 | 0.0 |
| No Unity floor (row 5) | 79.9 | −1.7 |
| All five back | 74.0 | −7.6 |
| All five back, and the judge's old cleanse values (4 / 10) | 72.9 | −8.7 (r150: 72.6) |

Each change gives the players 1-2.5 points at d3 and together they add up (the all-back arm lands on r150 within
noise, so `LoseNodeOr`, the faces and the viewer changes play the same). Every reading here is one arm against
r151 on the same seeds; the single arms overlap within about 1 SE of each other. D136 and the threat tokens are the
largest: a freed facility now also clears its agents and pays 2 Unity; why lowest-numbered tokens help the players
was not measured (a guess: they no longer sit where the agents gather, so they are easier to reach and pay).

## Table time (breakdown.py on 268's seeds, the judge, 14 games a level)

| Level | Minutes a round | Rounds a game | Minutes a game | Checks proxy (budget) | Components proxy (budget) |
|---|---|---|---|---|---|
| d2, r150 (309) | 24.3 | 7.21 | 175 | 15.0 (17) | 38.2 (43) |
| d2, r151 | 24.3 (paired −0.0 ± 0.3) | 7.00 | 170 | 14.3 | 37.1 |
| d4, r150 (309) | 23.4 | 7.07 | 166 | 18.9 (21) | 63.8 (71) |
| d4, r151 | 23.2 (paired −0.2 ± 0.5) | 6.93 | 161 | 17.9 | 62.1 |

The final build (5b05a9d5; Matt's message 14282: the first breakdown ran on 2345e264, before the review's fixes).
Rows 1-2 cut the Card beat's traced checks by about 0.5 a round at d2 and 1.3 at d4 (lines and arrivals −0.12 and
−0.18 minutes), less than the hand count's ~26 a round suggested, because the trace counts an Escalate's hits (sites holding
an agent), not the scan of every site, and counts no threat-token test at all: the scans rows 1-2 remove are in 309's
hand-log model, not the trace. A game is shorter at both levels mostly by rounds (7.21 → 7.00, 7.07 → 6.93). The read's own proxies
(design_check, d0-d3, 1,152 games a side): per-site checks a round −2 to −3% at every level, components −0 to −1%. Budgets
unchanged (both proxies under). Outputs: `out/keep/312/` (kept at `C:/Code/aicomplete-control/.conductor/kept/aicomplete/312_friction_r150_build`):
the two read sides (`r150.jsonl`, `r151.jsonl`, headerless: read as `FILE=experiments/farm/read.json`), the
attribution (`s312attr.jsonl` with its spec), the traces and breakdowns, design_check's output. Seeds spent:
51456-51551 (the ledger's next free is 51552).

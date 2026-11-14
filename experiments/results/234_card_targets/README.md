# 234: card targets (r106)

Matt's rulings of 2026-11-14 (locked): one target per map-touching player card, a target box in the header, Gain
and Place. `RULES_REVISION` bumped from the constant, 103 to 104 (84f51b9, the build f234's first new side
measured), 105 for the review fixes (758964b; message 10473), 106 for Amnesty Decree's `clear_city` (46cba8f, from
f234's design check; message 10485). Base: main aeced1d (233's r103). Prompt numbered the item 235; the queue and
branch say 234.

Files: [make_overlay.py](make_overlay.py) writes [targets.overlay](targets.overlay) (every conversion and the 12
prices; rerun it, then `scripts/cards/apply_overlay.py`), [notes.py](notes.py) (pricing notes),
[finds.py](finds.py) (what targeted removals find, per area and printed number, beside r103's reach),
[card_plays.py](card_plays.py) ([card_plays.txt](card_plays.txt)), [ledger_diff.py](ledger_diff.py),
[fresh.txt](fresh.txt), [finds.txt](finds.txt), [design_check.txt](design_check.txt). Specs:
`experiments/farm/s234a.json` (dev-seed smoke, target rule on/off), `experiments/farm/f234.json`.

## What shipped

- **Schema.** `Op::Target { on: TargetSpec }` opens the list it scopes (play effects, an Action branch, an
  Effect's `then`): the seat picks one hex in reach meeting `on` (agents, troops, agents or troops, botnet, a
  corrupted city / station / factory / site, a neutral station, a free zone, Fallout, a facility, a city with
  agents on or next to it, a piece, land). Map ops then act in `Area` around it: `on`, `near` (on or next to it),
  `country` (its country); `reach` (the default) is the old behavior, kept for board powers, standard actions and
  the adversary. New `PlaceAmbassadors` (Place: onto the map); `Shelter { reach }` (no target: your own nodes and
  facilities on hexes in your reach, a per-seat hex flag).
- **Content check** (`check_targets`, tests in `tests/targets.rs`): a map op with no target before it, two targets
  in a list, two target conditions on one card, a free country pick (`damage_country`, a country Shelter), a move
  past the target (`move_one`, `spread_ambassadors`), a targeted op with no `at`, or a target with nothing after it
  is refused.
- **Cards.** 61 cards print a target box (55 in the deck, 6 out of it); Aegis Grid Lockdown takes the targetless
  Shelter. Before: 58 faces said "in reach" and 4 more picked a country. Tier 1 acts on the target, tier 2 (and
  tier 1's 5-agent Headquarters Raid, and Directed-Energy Array's paid, repeated Action) on or next to it; Fallout
  counts on or next to it. The six country cards as ruled; Amnesty Decree is Matt's text word for word (its clear is
  `clear_city`, Diplomacy's verb, which design_check's tag story wants). Only Amnesty Decree places; every other
  "Gain N Ambassadors" stays a supply gain.
- **Faces and viewer.** `Face::target` (glyph and words) is a tab under the title (`web/card/card.js`); the near mark
  is a new icon. `npm run cards:fit`: 61 target boxes, all at 7.5 pt; no box below its pre-234 size (the first
  placement had squeezed 8 buildings' top boxes to 5.4-5.8 pt; moved). docs/cards.md prints "Target: ..." first.
- **Agents.** `target_rule` (`agents::moves::target`): the hex where the card's map ops find the most, each op's
  finds up to its number at its `values.json` row; a gather's or push's target is its move heuristic's hex; a tie at
  the top is the search's. `values.json`: `rates.area_on` 0.75, `area_near` 0.8, `area_country` 1 (finds over r103's
  reach), rows `target`, `place_ambassadors`, `shelter_reach`; `effect_ledger.py` mirrors them.

## Pricing (anchors; finds measured on s234a, 192 dev-seed games, confirmed on f234)

A conversion that keeps the target in reach and acts on one hex (liberate, claim, botnet, damage, strike, convert,
gather, push, firewall, build) prices as before. Agent removals lost finds: per play, what the card removes over
what r103's reach held, f234's shipped side: 1 agent 1.00, 2 0.72, 3 0.72, 4 0.65, 5 0.73, 6 0.73; troops 2-3,
1.06. Each re-priced card keeps its old anchor ratio: Δget = the removal's nominal anchor × (find share − 1).

| card | was | now | anchor check (pay; get; ratio) |
|---|---|---|---|
| Eviction Worm | 12 | 9 | 13; 3 agents 15×0.73 + 4 compute 8 = 19; 1.44 |
| Counterinsurgency Campaign | 15 | 12 | 16; 11 + 2 credit income 8 = 19; 1.21 |
| Diplomatic Expulsion | 12 | 8 | 12; 4 agents 20×0.72 + 1 Unity 4 = 18.4; 1.5 |
| Killswitch Directive | 16 | 12 | Δget −5.6 at 0.72; over 1.25 |
| Headquarters Raid | 28 | 22 | 30; 18 + draw 4 20 = 38; 1.25 |
| Liberation Accords | 15 | 11 | 27; 24 + 4 ambassadors 16 = 40; 1.48 |
| Martial Cordon | 8 | 3 | 19 (keep 3); 24 − 1 Discord 4 = 20; 1.08 |
| Kinetic Interdiction | 2 | 0 | 30 (keep 3); troops 12 + agents 24 − 4 = 32; 1.07 |
| Joint Task Force | 14 | 11 | 19; 27.4; 1.41 |
| Directed-Energy Array | 11 | 8 | near; about 0.9 of reach a use, 3 uses |
| Back-Channel Talks | 14 | 11 | the second factory next to the first about half as often |
| Annihilation Lance | 10 | 8 | every piece on the target (was any hex) |

Kept: Great Liberation (the target and 2 more in its country finds more than 3 in reach in one country; 1.9
nominal, under 2.5), Amnesty Decree (Place 2 is the same 16 at the anchors, a Dispatch saved), the Shelter cards,
Faraday Yard, the Fallout lifts, the moves. **Give / get at values.json:** the new op `place_ambassadors` is 1.5 ×
the table's ambassador (4) = 6 a piece, beside Gain's 4 (the Dispatch it saves); Amnesty Decree gives 15 credits +
1 keep (2.5) and gets 4 agents cleared 8 + 2 placed 12 = 20 (+2.5). Each re-priced card's net at the table rises by
its cut (its finds already sat under the table's nominal rows).

## Smokes

- s234a (dev seeds 36800-36815, d2, 192 games): target rule on/off, engine 56.2 / 56.2, strategist 68.8 / 70.8
  (16 seeds a cell: noise); fires 1.5 a game. No converted card kept and never played. `loop_smoke.py --run`: no
  timeouts, most plays of one card 5 (Diplomatic Expulsion).
- Speed (`bench/per_decision.py`, r103 → r105, c2 d2, 6 games, on a loaded box: ratios only): strategist 1.01× a
  decision (options a decision 1.14×: the target prompt), greedy 1.05×. `speed.py check` gives no verdict across a
  rules revision.

## Fresh read f234 (47232-47279, taken; 48 seeds a cell, 2 players, B+P / B+R / P+R, d0-d3)

The shipped play (46cba8f; cb8e214 renumbers it r106) against main r103 (aeced1d, built from `git archive`),
paired, 144 games a row ([fresh.txt](fresh.txt)):

| agent | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| greedy r106 | 85.4 (+2.1 ±4.0) | 80.6 (+5.6 ±4.3) | 67.4 (+2.1 ±4.6) | 47.2 (+4.9 ±4.4) |
| greedy r103 | 83.3 | 75.0 | 65.3 | 42.4 |
| strategist r106 | 89.6 (+4.9 ±3.5) | 77.8 (+0.7 ±4.6) | 75.0 (+5.6 ±4.2) | 44.4 (−3.5 ±4.9) |
| strategist r103 | 84.7 | 77.1 | 69.4 | 47.9 |

Replays (CLAUDE.md, *Reads*; message 10473): f234's first new side (84f51b9, r104) against the review-fixed build
(758964b): compare.py +0.3 ±0.7; that against the shipped play (46cba8f, Amnesty's `clear_city`): −0.3 ±0.4.

## Confirming read f234b (47280-47327, taken; message 10485)

Amnesty Decree's `clear_city` came from f234's design check, so the shipped build (cb8e214, r106) was confirmed on
the next free seeds, paired against r103 ([confirm.txt](confirm.txt)):

| agent | d0 | d1 | d2 | d3 |
|---|---|---|---|---|
| greedy r106 | 88.9 (+4.2 ±4.0) | 84.0 (+9.7 ±4.2) | 71.5 (+1.4 ±4.4) | 49.3 (−1.4 ±4.9) |
| greedy r103 | 84.7 | 74.3 | 70.1 | 50.7 |
| strategist r106 | 86.1 (+1.4 ±3.8) | 79.9 (+3.5 ±4.1) | 68.8 (−2.8 ±4.7) | 54.2 (+6.2 ±5.0) |
| strategist r103 | 84.7 | 76.4 | 71.5 | 47.9 |

Both reads, levels pooled: greedy +3.7 and +3.5, strategist +1.9 and +2.1 (SE about 2.2 each read): the game is
about 2-4 points easier, short of 2 SE on each read alone. The cheaper removal cards and the agents' target pick
more than cover the reach the cards lost (agents found 0.65-0.73 of r103's reach a play). Bands (soft, reported, not
chased): d2 sits over 50-65 on both sides of both reads (r103 65.3-71.5), greedy d1 80.6 / 84.0 over 80,
strategist d3 54.2 over 50 on f234b only. Measures (f234): rounds 0.1-0.6 shorter at d0-d2, agents placed 1-6 fewer
at d2. Kept reads: `out/keep/234_card_targets/`.

## Converted cards in the read

Every converted card in the deck was played by both agents on both reads ([card_plays.txt](card_plays.txt),
[card_plays_b.txt](card_plays_b.txt)); `play_read.py`: kept-and-never-played 0 for both agents on both. f234b
agrees with what follows to ±0.05 a game. Plays a game (greedy / strategist, r103 both agents pooled) moved most
on Air Gap (0.23 / 0.22, was 0.36: its target needs agents, and the firewall goes on them), Honeypot Relay (0.54 /
0.54, was 0.35), Directed-Energy Array (0.39 / 0.41, was 0.24; cheaper), Martial Cordon (0.17 / 0.18, was 0.09;
cheaper).

## Not done, and readings taken

- **Board powers and standard actions** (the prompt's lower priority; the staging question went unanswered, its
  default taken: leave them). Two print the same ambiguity: the Builder's Fortify ("remove 1 troop in reach and
  harden a facility in reach", two places) and the standard Remove agents ("up to 3 agents in reach, from any
  hexes"). Proposed one-target forms: Fortify "Target: Troops. Remove 1 Troop on it; harden a facility on or next to
  it"; Remove agents "Target: Agents. Remove up to 3 on or next to it". The Researcher's Disrupt ("in one country")
  is a recorded design exception (206). Their ops keep `at: reach`, the old behavior.
- **Staging questions, decided:** Gain stays your supply on every card but Amnesty Decree (Matt's example); a card
  whose target finds no hex still plays its other parts (r48's "nothing to act on never blocks the rest"), and one
  with nothing else is not offered; own-node shields (Benchmark Suite, Threat Forecast Engine) stay targetless.
- **Shelter** is the player's own nodes and facilities on the hexes in reach when it resolves (a per-seat hex flag);
  the first build sheltered every piece there (the review caught it; r105).
- Megaproject cubes (Field Grant and others) still pick any open megaproject; `move_ambassadors` still takes the
  moved ambassadors from anywhere (only the destination is the target). Both are flagged for Matt in the report.

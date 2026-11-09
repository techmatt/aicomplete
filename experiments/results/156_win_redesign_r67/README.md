# 156: the win redesign (r67): the purge is the only win, three-tier Unity and Discord cards

Matt's rulings (2026-11-08): no Unity-deck win; the purge is the only win, stepped by Unity tips; no late liberation;
`free_leaves` 0; no global effects in anything new. `RULES_REVISION` 66 → 67, bumped from the constant (the prompt also
said r67).

Scripts: [specs156.py](specs156.py) (smoke, screens, reads), [screen156.py](screen156.py) (by lever),
[ends156.py](ends156.py) (win routes by Unity step, loss kinds), [endgame156.py](endgame156.py) (the probe by Unity
step), [tiers156.py](tiers156.py) (each card's tiers at `values.json`), [variant156.py](variant156.py) (card-strength
variants, written but not screened). Tables: [final.md](final.md), [screens.md](screens.md), [smoke.md](smoke.md),
[judge.md](judge.md), [endgame.md](endgame.md), [tiers.md](tiers.md), [greedy.md](greedy.md).

## The rules

- **The purge is the only win.** Its steps now count Unity **tips** (`alignment.tips_unity`), not the resolved
  pile (the reshuffle empties it): 0-2 every piece, 3-5 no adversary ambassadors, 6+ cities only (`purge_at` 3, 6, 6).
- **Three tiers on every card** (`alignment.tier_at` 3, 6): the side's tips before this tip pick I (0-2), II (3-5)
  or III (6+). One deck a side; `unity_tier2`, `discord_tier2` and the cards' `steps` are gone. Content shape:
  `tiers: [{text, effects}] × 3`, `draft: true` on a drafted card (the card viewer badges it).
- **The Unity deck reshuffles** its resolved pile when a tip finds it empty (20 cards: no final-read game did). The
  Discord deck's empty-deck loss stays; with 6 cards the 7th tip loses, so **a Discord tier III never resolves** at
  d0-d3 (asked of Matt, question 1; default kept: printed, unreachable). d4's removed card counts as a tip made, so
  its tiers come a tip sooner.
- **No late liberation** (`liberate_late_at`, `liberate_late_stage` removed); **`free_leaves` 0**.
- **`one_player`** (new team op, Team slot): the team (seat 0 asks) picks a player the line would act for, who resolves
  instant player ops in their own reach. Its value arm (`value.rs`) is the ops' worth at `ops.one_player` 1.

## The cards (give / get at `values.json`, two players; [tiers.md](tiers.md))

A tip brings a card for nothing, so the give is 0 and the get is each tier's worth. Means: Unity 11 / 19 / 34, Discord
-11 / -29 / -42. Each Unity card's tiers rise (156 fixed four whose tier III read below tier II: Think Tank, Ceasefire,
Disarmament Talks, Network Cleanup). **Where each has a place to matter:** tier I at Unity tips 0-2 (rounds 2-5:
income and threats matter most); tier II at 3-5 (rounds 5-7); tier III from the 7th tip (round 7 on at d1-d2; 2-3.7
tier-III cards a game in the final read), when 2-5 corrupted cities still stand (the probe) and the team holds 3-5
ambassadors against a ~20-ambassador liberation bill: there, Volunteer Corps, Popular Mandate, Reclamation and
Citizens' Assembly liberate a city in reach outright (45-53), and five more clear every adversary ambassador around one
(each such ambassador on the city is one fewer to pay). **Flag:** the table prices `clear_city` with max 0 at
`token_pile` (3 ambassadors, 6 credits), while late cities hold more; Town Hall Accord (4 / 8 / 14) reads weak at the
table for that reason. Every card resolves 0.38-0.53 times a game (final read, 1,728 games).

The 6 Discord cards keep their identity: tier I is the old base line, tier II the old first step (Incursion's new: 2
threats and a tier-2 threat in each country), tier III the old strongest step. No node destruction, no global rule.

## Agents (each checked on and off: [smoke.md](smoke.md), 16 dev seeds, d1-d2, 96 games a cell)

- **The purge target:** the horizon aims at the purge's last step (`purge_at`'s 6), not the Unity deck's last card;
  `w.unity_target` = 20 restores the old aim for the check (strategist 52/33 on, 56/31 off: noise; no win route left
  for the old aim to chase). `unity_deck` (the weight) is gone.
- **The tier read** (`w.tier_read` 1): each side's next tip is worth the track's fill × a card's mean worth at the tier
  it would resolve at (Discord's a cost). Greedy on 90/77 vs off 77/62 (+13 / +15); the strategist 52/33 vs 62/27
  (noise either way). Kept on.
- **The judge (question 2, Matt's call pending).** At r67 the strategist plays the purge far worse than greedy (d2:
  32 vs 77 on 48 games); at r66 they were level (71 vs 62). It is the engine's weight set, not its search: the engine
  with greedy's flat weights wins 71, greedy:strategy=1 65 ([judge.md](judge.md)). Greedy frees ~4 cities a game, the
  strategist ~1.4-2. No weight fix was taken (greedy's hand/built weights read +4, inside noise). Greedy ran beside the
  strategist on the final read's seeds (#6878, [greedy.md](greedy.md)): **95.6 / 89.1 / 83.6 / 68.3** pooled (SE 1.0 /
  1.5 / 1.8 / 2.2), against the strategist's 94.9 / 77.8 / 68.1 / 47.7: +0.7 / +11.3 / +15.5 / +20.6. Greedy frees 2.6-3.7
  cities a game, the strategist 1.2-2.0; greedy's losses are mostly the round cap too (16 / 38 / 32 / 104).
- **Cost** ([per_decision.md](per_decision.md), [per_decision_tier.md](per_decision_tier.md), one thread, busy box,
  ratios): the strategist 1.40 × r66 per decision, with 1.50 × the options per decision (the game, not the agent);
  greedy 4.90 × r66, and 7.58 × with the tier read on against off on the same binary (games about as long): the tier
  read is greedy's slowdown, cause not found (its term is a precomputed lookup; suspect the hold-off and `one_player`
  prompts it now explores). Kept on for its +13 / +15; flagged.

## Screens (dev seeds 36800-36831, strategist, 96 games a level; [screens.md](screens.md))

r67 at the prompt's numbers: **82.3 / 51.0 / 32.3 / 7.3** (r66's dials). Single levers: `ambassador_income` 1,1,1,2,2
+10/+17/+10/+5; counters (`counter_bonus_credits` 4, `counter_more_credits` 0) 0/+14/+6/+10; compute 4,4,6,10,10 with
ambassadors 1,1,2,2,2 +10/+16/+3/+12. All three together: 92.7 / 75.0 / 62.5 / 21.9. d3 then (d3's entries only):
ambassadors 1 and compute 6 read 51.0, compute 10 38.5; **frozen at compute 8: 92.7 / 75.0 / 62.5 / 49.0**.

**Frozen dials** (provisional: the next item's tier-2 player cards): `counter_bonus_credits` 8 → 4,
`counter_more_credits` 2 → 0, `compute_income` 5,5,10,16,16 → 4,4,6,8,10, `ambassador_income` 2,2,2,3,3 → 1,1,1,1,2.
Card strengths unchanged. d2 and d3 now share an ambassador income (1); d3's difference is its compute (8 vs 6), its
shorter deck and the counter dials.

## Final read: r67 against r66, fresh seeds 39160-39303 ([final.md](final.md))

| Level | r67 (pooled, 432) | SE | sets | r66 | paired gap | band | verdict |
|---|---|---|---|---|---|---|---|
| d0 | 94.9 | 1.1 | 94.4 / 94.4 / 95.8 | 98.4 | -3.5 ±1.2 | ≥90 | met |
| d1 | 77.8 | 2.0 | 75.7 / 82.6 / 75.0 | 79.2 | -1.4 ±2.6 | 70-80 | met |
| d2 | 68.1 | 2.2 | 61.1 / 72.2 / 70.8 | 56.9 | +11.1 ±3.0 | 55-65 | **unmet** (3.1 over) |
| d3 | 47.7 | 2.4 | 41.0 / 52.1 / 50.0 | 41.0 | +6.7 ±3.0 | 40-50 | met |

Every level was read on three independent sets (d3 at 47.7 is 2.3 from its edge). The final build replays set a
game for game (`same_results.py`, 576 games: only comments, docs and tests changed after the freeze). d2 is the one the dev seeds put
lowest (62.5) and the fresh seeds highest: a d2-only lever after the read (screen 6, information only, #6889: no
retune) reads compute 6 → 8 at d2 -3 (59.4 dev); `counter_more_credits` 2 cuts d2 -9 and d3 -17.

**Win routes:** every r67 win is the purge at step 2 (6+ Unity tips): 410 / 336 / 294 / 206. None at steps 0-1 (troops,
50-100 a game, count there). r66 won 348 / 333 / 239 / 170 by the Unity deck and 77 / 9 / 7 / 7 by the purge.
**Losses (r67):** round cap 19 / 75 / 70 / 201, Discord deck 3 / 21 / 68 / 25, nodes 0. The Discord deck now
loses far less than at r66 (3 / 82 / 184 / 172), the cap far more (r66: 4 / 8 / 2 / 83).

**The end-game probe** (every loss, by Unity step; [endgame.md](endgame.md)): losses at step 2 (6+ tips) are
round-cap losses with 1.6-2.8 corrupted cities left (d3: 154 games, 2.8 cities, 1.0 botnet); at the snapshot (third-
last round) the team held 3.5-11 ambassadors against a Liberate bill of 15-22. Losses at steps 0-1 still count troops
(58-102 standing) and are out of reach. **What the gap needs (for the tier-2 card item):** about 3 more liberations
in the last three rounds at d3 (some 15-17 ambassadors, or 3 tier-III liberations), or getting to 6 Unity tips sooner
(d3's step-1 losses: 55 games).

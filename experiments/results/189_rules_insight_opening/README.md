# 189: insight, keeps and the opening (r82)

Matt's 2026-11-11 rulings on insight, keeps and the opening, 188's follow-ups and one 187 default, built as one
revision, with amendment 8615 (insight printed as totals on the card-play track). The constant was 81, so this is
81 → 82, as the prompt said. Every rule here is a candidate (Matt will playtest).

## What changed

| Ruling | Engine | Test |
|---|---|---|
| Insight read off the board; the setup space adds none (amendment 8615: totals on the card-play track, Spirit Island style) | `AiBoard.insight` 6 (the track's printed start), `Uncover.insight` an absolute total (7, 9, 11, 12, 13 on the Builder's and Protector's; 7, 9, 10, 12, 13 on the Researcher's); `ai::track_insight` is the rightmost uncovered total; the economy track's space 1 is now empty; `Content::check` refuses insight anywhere else and totals that don't rise. Same numbers as r81 | `the_board_prints_6_insight_and_the_setup_space_adds_none`, `the_card_play_track_prints_insight_totals_and_the_rightmost_counts` |
| No credit fallback for keeps | `insight_credits` gone; `keep_need` returns insight only; a keep is legal only with the insight | `keeps_cost_insight_by_tier_in_round_1_and_credits_never_buy_them` |
| Patience keeps all unspent insight | `insight_received` and its carve-out gone | `patience_banks_insight_another_player_gave` |
| Keeps 1 / 2 insight every round, the opening's draft included | the round-1 branch of `keep_need` and `counts_as_kept` gone | (as above) |
| The opening: 2 tier-1 and 2 tier-2 dealt into the hand, free; round 1 drafts | `opening_deal` 2, `opening_deal_tier2` 2, dealt to the hand; `opening_free`, `opening_keep_credits`, `opening_keep_tier2` gone; `cards.deal` runs before `cards.tier`, which now runs in round 1 | `the_opening_deals_2_and_2_free_then_round_1_drafts` |
| Insight isn't an adversary harm | `Op::LoseInsight`, the insight debt and its view row gone; `Content::check` refuses any harm op taking insight; harms ledger loses "insight lost". **Smear Campaign** (tier-1 threat, was "4 less insight next round"): the nearest player pays 2 energy or discards 2 cards (credits would share its counter's currency, which the threat standard refuses). **Data Heist** (project, was "3 insight next round or 2 cards"): each player pays 8 credits or discards 2 cards | `insight_is_never_an_adversary_harm` |
| Expand costs its 1 energy again | the three boards' Expand `cost` | `every_board_s_expand_costs_1_energy_and_the_space_once_a_round`, `a_node_power_pays_its_energy_and_the_step`, `a_track_s_last_steps_cost_more` |
| Aegis Doctrine: Shelter on your own pieces once a round | automatic: the first destruction a round of the holder's node or facility is stopped before any choice (`Guard::Aegis` gone; `board::protected` for the agents' forecasts). Matt's design question 1: the default, confirmed (message 8639) | `the_aegis_doctrine_guards_only_its_holders_own_pieces` |
| A blocked Rally share stays in the supply (as at r79) | `rally_plan` deals over every site; a firewalled or no-Rally site's share stays in the supply (`adversary.rally_blocked`) | `rally_deals_round_robin_in_hex_order_and_a_firewalled_share_stays_in_the_supply`, `a_rally_prohibition_keeps_the_share_in_the_supply` |

The prompt named Data Heist as "a tier-1 threat"; in content the tier-1 threat was Smear Campaign and Data Heist the
project. Both are reworked.

## Give / get (values.json: energy 4.5, node 8, insight 2.5, credit 1, a card ~4)

- **Expand:** gives 1 energy + the space (0-5 energy: 4.5-27); gets a node, 8. A gain on the free spaces (+3.5),
  near break-even on a 1-energy space (−1) and a loss beyond at the node's table price alone; the space's own
  income makes up the rest, as at r80 (the same price).
- **Smear Campaign's harm:** 4 insight next round (10) → 2 energy (9) or 2 cards (~8). **Data Heist's:** 3 insight
  (7.5) → 8 credits (8) or 2 cards (~8). Weights about equal; the player now chooses the cheaper.

## Ladder (fresh seeds 43471-43614, paired against r81)

Strategist 144 seeds a pair (432 a level), greedy 43471-43518 (48 a pair). r82 farmed at d224e59 (`farm-seeds`, main
box, 324 s), r81 natively from `out/r81` (a clean build of 7a59fcd). Specs `experiments/farm/f189-*`. Amendment 8615's
build plays the farmed games exactly (90 games, `same_games.py`); the stats-only `limit.*` counters leave the golden
fingerprint unchanged.

| d | r82 strategist | r81 | gap ± SE | band | r82 greedy | r81 greedy |
|---|---|---|---|---|---|---|
| d0 | 89.6 | 90.3 | −0.7 ± 2.0 | 90-100 **unmet by 0.4** (0.3 SE) | 89.6 | 94.4 |
| d1 | 75.5 | 79.6 | −4.2 ± 2.5 | 70-80 met | 84.7 | 82.6 |
| d2 | 61.1 | 68.1 | −6.9 ± 2.5 | 55-65 met | 58.3 | 67.4 |
| d3 | 49.8 | 59.5 | −9.7 ± 2.8 | 40-50 met | 45.8 | 61.8 |
| d4 | 47.0 | 56.2 | −9.3 ± 2.9 | - | 41.7 | 53.5 |

The d0 miss: pairs bp 92.4, br 94.4, pr 81.9 (pr −5.6 ± 3.8 against r81; `pairs-strategist.md`). No difficulty dial
moved (the prompt). r81 on these seeds was over the d2 and d3 bands (68.1, 59.5): r82 is harder at d1-d4 by 4-10
points, which brings d3 back into its band. Adversary measures within noise (`ladder-strategist.md`). Which ruling
carries the drop is not isolated (no attribution screen was run: all rulings changed at once).

## Resource use (strategist, the ladder's 2,160 games a side; `resource-use.md`, `resource_use.py`)

Per seat a game. "Ran out": the share of seat-rounds a resource ended the action phase at 0 (insight: every seat at
0). The limit counters (`limit.*`, new in 189, stats only, patched into the r81 tree for this read with
`out/patch_limits.py`; same games on both sides, checked) count, each round, every drafted card not kept and every
card left in a hand, by what stopped it.

| resource | spent r82 | r81 | ran out r82 | r81 | cards held for want of it, r82 | r81 |
|---|---|---|---|---|---|---|
| credits | 227 | 236 | 28% | 28% | 70.9 | 77.6 |
| insight | 38.1 | 37.0 | 4% | 3% | 8.8 (and 0.09 keeps) | 8.8 (0.15 keeps bought with credits) |
| energy | 24.8 | 23.4 | **44%** | 29% | 5.4 | 3.8 |
| chips | 4.3 | 4.8 | 64% | 60% | 0.02 | 0.03 |
| research cubes | 3.6 | 3.9 | 87% | 87% | 0.43 | 0.15 |
| ambassadors | 18.0 | 18.8 | 45% | 44% | - | - |

Tags kept 18.0 card-rounds in hand (r81 13.2); 5.5 were playable and held by choice (6.5).

**Insight's keep-versus-play trade (per game, both seats):** to keeps 37.2 (r81 32.9), to plays 32.4 (34.7); cards
drafted 56.8 (48.6, round 1 now drafts), kept 30.3 (27.0), played 33.8 (37.0). A drafted card went unkept for want
of insight 0.09 times a seat a game against 13.1 unkept by choice: **insight almost never stops a keep**. It stops a
play 8.8 card-rounds a seat a game, as at r81, a ninth of what credits do. Insight left after keeps is 109 a game and
73 unspent at round end (the Researcher's bank is most of it: Patience carries 57 a Researcher game). Read: insight
is spent on keeps more than at r81 and the trade bites a little at play time, but with 6 a round it is rarely the
scarce thing; credits are the limit on plays, energy now a close second in running out (Expand's 1 energy: energy to
powers 18.4 a seat a game, r81 14.8). Removing the credit fallback cost nothing measurable (0.15 keeps a game bought
with credits at r81).

## Matt's watch list (amendment 8615; `watch.md`, `watch.py`)

| per game | r82 | r81 |
|---|---|---|
| Rally agents a firewall or no-Rally token sent back to the supply | 2.59 (games with any: 49%) | 0 (dealt to the other sites) |
| of them by a no-Rally token | 1.38 | - |
| agents rallied | 63.9 | 63.9 |
| firewalls laid by cards | 0.71 | 0.63 |
| no-Rally prohibitions made | 0.33 | 0.34 |
| opening tier-2s kept / played (of 4) | 4 (100%, free) / 1.20 (30%) | 1.18 (30%, 1 insight) / 0.80 (20%) |
| Patience: insight carried a Researcher game | 57.2 | 51.8 |
| Patience: share of rounds carrying any | 77% | 81% |

A blocked share is 4% of what the adversary rallies (2.6 of 64 a game). Firewalls are not mandatory to the agents:
0.71 a game, barely up from 0.63, and the agents do not yet weigh a blocked share as a saved agent (the forecast
already drops blocked shares, `plan::next_rally`). Not "too easy" on this read; a firewall card's price was not
touched. The opening's tier-2s: free, all are held and 30% get played (r81: 30% kept at 1 insight, 20% played).

## Other measures

- **Rulebook Part 1** (187's `wc_part1.py`): 7,875 (r81) → **7,929** words (+54); the page 12,005 → 12,075. The
  opening and keeps lost their exceptions (8 dealt, 4 free, 5 credits, opening tier-2 price, credit fallback,
  Patience's carve-out, insight losses); the Spirit Island track sentence, the Aegis step and Rally's "turn in the
  deal" added more than that saved.
- **Loop smoke** (16 seeds): no timeouts, no card at the play flag.
- **Speed:** `speed.py check` gave no verdict across the revision (random 1.07x, greedy 1.05x, engine 0.81x, planner 0.76x a game). `per_decision.py` (planner, 6 games, d2): 0.71x a decision, with 0.77x the options a decision (Expand priced again, so it is legal less often) and 1.06x the rounds. Baseline updated at r82.
- **Same results:** amendment 8615's build plays the farmed d224e59 games exactly (90 games); the limit counters
  leave every game unchanged on both sides (2,160 games each, `same_games.py --play`).

## New-player summary (r82)

You start with 2 tier-1 and 2 tier-2 cards in your hand, free; then every round, the first included, you build a
hand, draft, and keep what you want at 1 insight a tier-1 card and 2 a tier-2 card. Only insight buys a keep. Your
insight each round is the number on your card-play track's rightmost uncovered space: 6 at the start, rising as you
uncover it. The Researcher keeps all unspent insight. No adversary card takes insight. Expand costs 1 energy plus
the space. The Protector's Aegis Doctrine saves the first of its pieces that would be destroyed each round,
automatically. A firewall or no-Rally token sends the agent Rally would place there back to the supply. None
resisted a short statement.

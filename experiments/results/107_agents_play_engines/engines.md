# 107: the agents play r42's engines

The aim was to get the agents to build toward r42's engines and play them. Every fix is an agent switch or a
value-table rate; no content, rule or tunable moved. **Finding:** once the agents play these engines, they read
weaker than the cards they push out. The fixes raise engine plays only a little. Making the strategist keep and play
every engine at once costs it −21.5 ±3.1, and only playing the engines it already keeps at once costs −4.5 ±3.2.
Most of the 106 list stays near zero, for the reasons in the table below.

## Why each went unplayed (debug reads, 48 dev seeds a pair, the strategist)

Two temporary traces are gone now: a keep trace (why the plan dropped each drafted card) and a play trace (whether
a card in hand was legal and, if so, its greedy score against a pass). The causes by group:

| group | what stops it |
|---|---|
| `needs` 3-5 of a tag (Citation Engine, Smart Grid Backbone, Continental Supergrid, Security Contractor HQ, Tidal Barrage, Conglomerate HQ) | The keep plan rules them out as 2+ tags short (`plan_short` 1). In hand they are often still short (Smart Grid Backbone: 128 of 241 hand-decisions short on `needs`). Once legal, many score at or below a pass: Citation Engine −4 to −6 a decision in rounds 4-6. A trigger on one tag fired at the table's 0.3 share whatever the seat held. |
| built over a tag (Foundation Model Campus, Positron Storage Ring, Power to the People, Orbital Mass Driver, Hyperscale Botnet) | Income moved to programs, so few buildings of the tag stand to go over. The plan only allowed a cover over one already built. Once legal, a cover gives up what the covered building earns: Hyperscale Botnet scores a median 13 under a pass (−27 in round 3, −3 in round 9) and was never played. FMC scores +2 to +9 over a pass but loses each decision to a better action until its 20 credits are gone. |
| antimatter (Orbital Mass Driver, Dyson Swarm Tranche, Recursive Self-Improvement) | 1 antimatter and 4 tags, which the plan's budget rarely reaches. |
| energy engines (Dispatch Control Room, Tidal Barrage) | Energy is priced at the heuristic's rate (102's `plan_rates`, about 1.3 credits). Dispatch Control Room scores −1 to −7 under a pass when legal. |
| tier-2 every-tag triggers | Conglomerate HQ and Agency Directorate are thresholds filled at a flat 0.8 tokens a round. Their own trigger fills about 2.2 a round, so they were priced at a third of that. The other four (Interdisciplinary Institute, Command-and-Control Grid, Orbital Embassy, Smart Grid Backbone) were already played 0.1-0.2 a game when legal. |
| Retained Earnings and the Capital engine | value.rs read the income a threshold raises as 0, so Retained Earnings was worth 1.0 against a 9.5 cost and was never planned. Its feeders (Capital Injection, Vendor Financing, Liquidity Event, Collateralized Grid) then had no pile to target. |
| Payroll Automation | Only six cards print a building income to copy, all tier 2 and five of them built over. The plan kept it at `target_share` for a target that never came: 0.28 kept a game, 0.02 played. |

## The fixes, each with its switch, and its use and gap on and off

Dev seeds 33452-33547 (96 a pair, the three pairs at d2; `screen-107-dev96.json`, `-onoff96`). The keep-plan
switches were flipped as cells of one run, with chain progress on at that time. The value terms were read on
binaries rebuilt with one rate off, against all on. The use columns count kept and played a game for the cards
each fix acts on (`fix_use.py`).

| fix (switch) | sentence | acts on | kept on / off | played on / off | strategist on − off | greedy on − off |
|---|---|---|---|---|---|---|
| `plan_reach` | Keep the top of a chain on your strategy's tags when your hand gets you there. | needs (58) | 10.80 / 10.57 | 7.55 / 7.67 | +0.7 ±1.6 | (no strategy) |
| `plan_under` | Build the under-card first. | built-over (9) | 1.65 / 1.43 | 0.81 / 0.73 | +0.0 ±1.3 | |
| `plan_copy` | Don't plan a copy with nothing to copy. | Payroll (1) | 0.01 / 0.29 | 0.01 / 0.02 | −1.7 ±1.5 | |
| `plan_chain`, **off** | Build toward the chain card you hold. | needs (58) | 10.80 / 10.51 | 7.55 / 7.64 | −2.8 ±2.2 | (no strategy) |
| `rates.own_fill` | A pile your card's own trigger feeds fills at that trigger's rate. | 6 thresholds | 1.25 / 1.47 | 0.94 / 1.10 | −2.1 ±2.7 | +0.0 ±1.3 |
| `rates.raised_rounds` 2 | An income a threshold raises pays from then on. | Retained Earnings, feeders | 0.18 / 0.01 | 0.10 / 0.01 | +1.0 ±1.3 | +0.3 ±0.3 |
| `rates.insight_discount_share` 0.25 | Insight off every card only cuts the cards above the floor. | Hyperscale Botnet | 0.15 / 0.20 | 0.00 / 0.00 | −2.4 ±1.3 | +0.0 ±0.0 |
| `rates.own_mix` (`mix_prior` 4) | A Science trigger fires as often as your cards are Science. | 15 one-tag triggers | 2.73 / 2.70 | 2.24 / 2.10 | −1.4 ±2.7 | −1.0 ±1.0 |
| `plan_pile`, dropped | Keep a pile card with its feeder. | | | | +0.0 ±0.0 (never fired) | |

`plan_reach` and `plan_chain` apply only to a seat with a strategy, and only to cards whose `needs` tags are all on
that strategy. Ungated, on 48 seeds, each cost −6.9 (±3.6 and ±4.0). So greedy, the new player, gets no tag
planning. It shares only the value terms and `plan_under`/`plan_copy`, and its lift stays small: +0.7 ±1.7 on the
dev seeds, +0.7 ±1.2 fresh.

**All on less chain progress** (the frozen build), on the dev seeds against r42's agents: strategist −3.8 ±2.9,
greedy +0.7 ±1.7, lead over greedy 31.6 → 27.1 (−4.5 ±3.5). With chain progress on as well: strategist −6.6 ±3.1.

## The probe: what the engines win when played (`engine_rush`, dev seeds)

`strategist:engine_rush=1` drafts and keeps every engine card: 44 cards, any with a trigger, a threshold, per-tag
income, a discount or a copy. It plays one the moment it is legal. `=2` only plays a kept engine at once.

| cell | engines kept / played a game (dev) | strategist, dev (96 a pair) | strategist, fresh 33644-33691 (48 a pair) |
|---|---|---|---|
| the strategist (107) | 7.93 / 6.00 | | |
| `engine_rush=2` (play a kept engine at once) | 7.83 / 9.45 | −4.5 ±3.2 | −2.8 ±3.7 |
| `engine_rush=1` (keep every engine, play at once) | 16.97 / 10.15 | −21.5 ±3.1 | −18.8 ±4.4 |

Even when forced, most of the 106 list stays rare: Smart Grid Backbone 0.10, Citation Engine 0.12, Positron
Storage Ring 0.10, Orbital Mass Driver 0.00 a game. The limit is the tags, antimatter and credits, not the agents'
choice. Where the agents do choose, playing engines earlier or more often loses.

## The read (fresh seeds 33596-33643, 48 a cell, d2; `screen-107-r42.json`)

| pair | strategist r42 → 107 | gap | greedy r42 → 107 | gap |
|---|---|---|---|---|
| bp | 27.1 → 31.2 | +4.2 ±8.9 | 4.2 → 6.2 | +2.1 ±2.1 |
| br | 41.7 → 45.8 | +4.2 ±7.3 | 12.5 → 14.6 | +2.1 ±2.1 |
| pr | 35.4 → 37.5 | +2.1 ±7.0 | 8.3 → 6.2 | −2.1 ±2.1 |
| pooled | | **+3.5 ±4.5** | | **+0.7 ±1.2** |

The final build added the mix cache and the `engine_rush` probe after this read. It plays all 288 of the read's games
the same (`out/same.py`, by won, rounds and decisions). The strategist's lead over greedy goes from +26.4 ±4.6 to +29.2 ±4.6, a change of +2.8 ±4.6 (`lead.py`). On the dev
seeds it fell by 4.5 ±3.5. Both are within noise; read together, the fixes leave the strategist about where it was.

Play counts on these seeds, for every card in the 29 and the Capital engine: [play-counts.md](play-counts.md). Of
the 33 new or changed cards the strategist played under 0.05 a game on r42's agents, 4 reach 0.05 or more:
Conglomerate HQ 0.01 → 0.06, Retained Earnings 0 → 0.08, Smart Grid Backbone 0.03 → 0.06, Consulate Tower
0.03 → 0.09. Agency Directorate goes 0.08 → 0.18. Payroll Automation's wasted keeps go 0.24 → 0.02 a game.

## Speed (quiet box)

| agent | read | rounds new / old | ms a decision new / old |
|---|---|---|---|
| strategist | the fresh read, 144 games a side, 6 threads (`ms_per_decision.py`) | 1.00 | **1.01** (greedy 0.97) |
| strategist | `per_decision.py out/base out/final`, 16 games from seed 1000, one thread | 1.15 | 1.16 |
| planner | `per_decision.py`, 16 games from seed 1000, one thread | 0.87 | **0.89** |

Where the strategist's 1.16× comes from (Matt, message 4538), split on the same 16 seeds:

| per_decision, 16 games from seed 1000 | games | ms a decision |
|---|---|---|
| r42's binary → 107's binary with every 107 term off (four rates at 0 or 1 in a rebuilt `out/alloff`, `plan_reach`, `plan_under`, `plan_copy` = 0) | identical (2 wins, 6.38 rounds, 393 decisions both) | **0.98×** |
| that binary, terms off → on | 6.38 → 7.31 rounds, 393 → 461 decisions | 1.14× |

So the code is no slower on the same games. The 1.16× comes from the games the new agents play on these seeds: they
run 15% longer and hold more engines in play. Every engine in play adds trigger work to each apply the search makes,
and later rounds carry bigger tableaus. On the fresh read's 144 games a pair, where game length is unchanged, the
games cost 1.01× a decision. The seat's tag mix is the only new work per evaluation. The in-play count is cached
with the seat terms, and the hand is read from the card-fact cache; a first version that walked the piles each
evaluation played the same games. The keep-plan terms run once a keep. `speed.py check` gave NO VERDICT because the
games differ. The baseline was `--update`d (greedy, engine and planner rows at 0.84, 0.90 and 0.52 of the old ms a
game, on other games) so the next check has a same-games reference. The table above shows the code itself costs
nothing.

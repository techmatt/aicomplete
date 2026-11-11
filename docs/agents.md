# Agents

In prose the simulator's players are "sim players"; the code keeps "agents" (Matt, 2026-11-09).

The skill ladder (`crates/aic-sim/src/agents/`). One team agent drives every seat, hands
and drafts open, and answers "players choose" questions too. Agents play
through the public API only (`legal_actions_into`, `apply_mut`, `apply_to`, the state).

- **No peeking.** Before a search an agent replaces what the printed game hides with
  a guess from its own RNG (`agents/determinize.rs`); a test checks hidden information alone
  moves no decision (the planner and oracle: the table below).
- **Own randomness.** Each agent's RNG is the game seed on its kind's stream.
- **Decides at the top level:** outside the action phase the planner follows its line. A node
  power is scored deeper than `choices` (`search.rs`: `PLACING_PROMPTS`, `SACRIFICE_PROMPTS`).
- **`choices` counts prompts** (a placement is two: track, hex). An effect resolved a prompt a piece is scored
  part of the way, so it reads as paid for little: 166's moves take a batch a prompt for this.
- **A discard prompt resolves no further** (`discards` 0): a card to discard is valued by the table it leaves
  (`discards=99`: resolved as other prompts).
- **Heuristic** (`agents/eval.rs`, `Weights`, a rule on each field): win/loss, cities,
  a seat's last nodes, the adversary's pieces, the Discord deck's last cards, Alignment (the next
  card's tier, `w.tier_read`; the horizon aims at the purge's last step), projects, holdings,
  unpaid threats by **the tempo trade** (`w.tempo`: one-off harm plus what it leaves times the
  rounds left, over what paying now costs the engine), corruption risk (`w.corrupt_exposure`,
  `w.blight`), the coming Rally (`w.rally_read`), Movement, botnets (−25, D9), cubes, converter
  cities, the liberation push, a darkened city, Patience insight and idle credits.

| Spec | What it does | Budget parameters (defaults) |
|---|---|---|
| `random` | Uniform over legal actions. | none |
| `greedy` | One action at a time, each scored by the heuristic after it, its choices resolved best-first; a pass scores the table as it is (else stalling beats stopping). | `samples` 1 (guesses averaged), `depth` 1 (actions looked ahead), `choices` 2, `discards` 0 |
| `beam` | Beam search over the round's remaining action sequences, scored at the next round's start; replans when a draw or reveal differs from its guess. | `width` 6, `depth` 120 (steps), `branch` 0 (children per sequence, 0 = all), `choices` 2 |
| `rollout` | Across rounds: the beam's `candidates` best end-of-round positions, each run `horizon` more rounds by `rollouts` greedy playouts on fresh guesses; best mean wins. | `width` 4, `depth` 120, `branch` 0, `choices` 2, `candidates` 3, `rollouts` 4, `horizon` 2 |
| `engine` | The heuristic in **horizon mode** (`w.horizon`: income × the rounds left, discounted by danger); holdings by the value table (`w.table=0`: flat terms); the **deck clock** (`w.clock`) charges each round the team's Unity rate would leave it short. Searched like greedy; `width` > 0: a beam, `candidates` > 0: rollouts. | `width` 0, `samples` 1, `choices` 2, `answers` 99 (prompts an answer looks on; 1 loses wins), `discards` 0; weights: `Weights::engine` |
| `strategist` | Greedy's flat weights with strategy and the **purge term** (`w.purge` 3.6: supply ambassadors toward the Liberate bills in reach) and `w.last_node` 120 (177), no search. **The screens' judge**. | `STRATEGIST`, `agents/mod.rs` |
| `planner` | The engine, rolling out a shortlist of action-phase alternatives `horizon` rounds with the engine as policy; the heuristic's best line is taken. A line runs on **full information** but the Alignment deal, valued over `deals` guesses of it; decisions inside it are scored on a guess (`agents/planner.rs`). | `horizon` 2, `alts` 2, `budget` 8, `margin` 40, `deals` 4, `lead` 0. `sweep[:...]` is `planner:deals=1,lead=0`. |
| `oracle` | **A measuring stick, opt in**: the planner's search on **full information**, the deal too, each line played to the game's end; a winning line is followed, so the win rate is a floor on perfect play (`agents/oracle.rs`). | the planner's four, `reseeds` 0, `threads` 1; planner and oracle: `discards` 0 |

**Keeping by plan (`agents/policy.rs`).** Greedy, engine and planner keep drafts by a plan
(`plan_keep`). At its first keep of a round a seat sketches `plan_rounds` rounds from what it
holds and earns, fills each round's budget with the cards it would play, by value per cost
(value: the table at the rounds then left times the strategy multiplier; cost: its price at the
table's rates, `plan_table=0`), and keeps each drafted card, best ratio first, only if the plan
is worth more with it. A play waiting a round is worth 15% less; `needs` may be met by cards
planned earlier, or short now by at most `plan_short` tags; a building over one of the seat's
adds only what it beats the weakest by (`plan_cover`). Each switch's default and rule, and each
**dominance check** (principles.md), is a row of `forced_table!`.

**Move heuristics (`agents/moves.rs`, 166; `move_rules`, on).** Gather and Push prompts: push agents off the next
Corrupt site onto empty land with no clean site next to it; gather (or push) agents onto a spare ambassador (no
armor, no Claim or Liberate it pays toward) to cancel; gather a seat's own free ambassadors onto an adversary site
when that seat can then pay its Liberate (never onto a node hex). Anything else, and every strike, is the search's.
`aic-sim experiment` prints their fires a game. **Converge** (175, `converge_rule`, on): a Corrupt line's agent next
to several sites goes where `plan::converge_pick` sends it. **Buy energy** (176, `energy_rule`, on): a seat with no
card left to play buys a neutral city's energy when its credits plus this round's income, less the price, reach the
eval's credit cap (a pass is scored before income). **Botnet reach** (176,
`w.botnet_reach` 0.3): an ambassador on a botnet's hex is worth that share of it.

Spec syntax: `kind[:key=value,...]`; a weight takes `w.` (`greedy:w.corrupted=40`), a table
number `t.`, a plan or strategy switch goes bare (`strategist:plan_bury=0`). `greedy` and the
strategist read flat terms, so the table's and horizon's levers reach only their keep plan. Sweeps: [`scripts/tune/`](../scripts/tune/README.md).

## The value table

`configs/agents/values.json`, read by `agents/value.rs`: a value in credits for each resource,
tag left in play, token and op, prices' one home (`Weights` are points; card-pricing.md, *The
rates*). A card is worth its effects now,
its standing effects a round times the rounds it counts for, a use of its Action times
`uses_share` of them (a token card's spend over its cycle of adds), and its tags, over the rounds left
(typical context: `horizon_t1` 4, `horizon_t2` 2.5). "Remove up to N troops" counts
`troops_removed` (D26). An `overrides` row adds flat credits to one card, with its reason: the scoped fix for a card the
table under-prices against the anchors; built, the card keeps it, spread over the rounds left.
`t.<key>=v` in a spec sets one number for that agent (`t.<section>.<key>` where a name is in two).

The heuristic reads it in table mode (`w.table`: on for `engine`, off for `greedy`; what each
holding counts, a building's earnings included, is on that field):

- **A card in hand:** `hand_share` (under 1) of its net worth, times the chance of paying and
  the progress term.
- **A card to draft or keep:** its value times the strategy multiplier, less `pick_cost` of its
  play cost, less its keep times `packet_keep` in the packet; times `hand_share`, then `drafted`.
  `pick_cost` 0: the keep plan judges what a seat can pay.
- **A trigger:** one on a tag's plays (or a discount on them) fires at the seat's share of cards
  with the tag in hand and in play, plus `mix_prior` cards at `tag_share` (`own_mix`; the ledger
  reads `tag_share`); a threshold its card's own trigger fills fills at that rate (`own_fill`);
  an income a firing raises counts `raised_rounds` rounds; insight off every card: card-pricing.md.
- **A token:** what its card's spend pays per token (a cash-out's rate, D24), else the held rate.
- **An action** is credited what the board terms can't see after it: a cordon, a peek, a
  shelter's flat worth (`Eval::bonus`; what a shelter saves: the guide, below).

## Strategies

`configs/agents/strategies.json`, read by `agents/strategy.rs`. With `strategy=1`, at its round-1
keep a seat commits to its board's strategy whose multipliers add most to its opening hand and
powers (a power at one use's worth, `strat_powers`); `strat0`-`strat2` fix one, 1-based. A
card's multiplier is 1 plus the weights it matches (`agents/strategy.rs`; its triggers' ops too),
times its categories' largest **phase schedule** multiplier (`w.strat_scale` stretches it). It reaches drafting and keeping, not a card in hand.

As a player says them (the config's `says`):

<!-- generated: strategies -->
**The Builder**

- *Industry income.* Play Industry income early (Gigafactory Charter, Robotic Foundry, Economies of Scale), draw on them with Supplier Network, and stack Industry tags toward Arcology Foundry and Vertical Monopoly; spend two spent Industry cards on Rush Build.
- *Power grid.* Take Power cards and energy income (Baseload Contract, Fusion Startup Stake), climb the Power tags to Fusion Research Campus and Dyson Swarm, and cash spent Power cards with Overcharge; Power to the People frees a city at the end.
- *Capital finance.* Build Capital piles (Retained Earnings, Conglomerate Headquarters, Capital Injection), let them pay out, and save the credits for Hostile Buyout.
- *Wide build.* Fill five slots with Action buildings, use one a round, and push the card-play track to Master Plan.

**The Protector**

- *Troop wall.* Stand your nodes in the troops' way, take the troop removals (Breach and Clear, Security Contractor HQ, Agency Field Station) and finish with Kinetic Interdiction.
- *Counter desk.* Answer threats with the typed jams and counters, build Garrison Command and Joint Task Force, and send Peacekeeping Mission to a corrupted city.
- *Diplomacy and liberation.* Take Diplomacy and Followers (Liaison Corps, Envoy Briefings, Open Assembly, Accord's node), and free cities with Great Liberation and Popular Uprising.
- *Civic shelter.* Keep what you have standing: Bulwark, Faraday Yard, Hardpoint Lockdown, Aegis Grid Lockdown and the Shelter power, toward Aegis Doctrine.

**The Researcher**

- *Cubes and stations.* Build stations early (Field Station), Grant cubes onto megaprojects (Moonshot Grant late), Peer Review your nodes, and spend the cubes on Forensic Audit.
- *Science chain.* Stack Science tags: Research Royalties, Citation Engine and Preprint Server first, then Interdisciplinary Institute and Foundation Model Campus.
- *Patience burst.* Save your insight under Patience and spend it in a big round on draws (Fieldwork, Simulation Cluster, Orbital Observatory Array).
- *Datasets.* Grow Dataset piles (Telemetry Archive, Synthetic Data Forge, Dataset Scraper) and cash them through Synthetic Data Forge.
- *Team support.* Feed your partner: gifts, Open Science's insight, Coalition Charter's Sway, Audit against projects, and Root Access for a corrupted city.
<!-- /generated -->

## Strategy-guide heuristics

Rules a player could say where a one-step search misses a card's point, each a switch, on,
its rule on its field or `forced_table!` row; the spare-insight rule is every agent's (Matt 2026-11-05).

- *Shelter what a destruction showing would take, once you can't counter it* (`agents/guide.rs`,
  `w.shelter_read`, `w.shelter_node`, `w.shelter_counter`; armor as the engine reads it, `w.shelter_armor`, D71).
- *Guard your last node* (177; the strategist's 120, 0 elsewhere): `w.last_node` (eval, converge and push rules).
- *Don't bury a building that still earns more than the new one adds* (`plan_bury`).
- *Count your research cubes like credits* (`plan_cubes`).
- *Pick the strategy your board's powers feed* (`strat_powers`).
- *Insight you can't spend is cheap: keep with it*: insight over `w.insight_cap` (10) is
  worth `w.insight_idle` (0.1), held or as income, and so is a keep paid from it (`keep_spare`, `plan_spare`).
- *A trigger that pays credits is a credits card*: strategies read triggers' and
  thresholds' ops (`w.strat_contents`). Insight income is worth `w.insight_income` (1.5, D53).
  Off (no gain read): `plan_team`. Retired zero-weight probes are refused with their reason (`eval::RETIRED`).
- *A per-tag card pays at the count you'll play it at*: table `tag_count` 5 plus the card's own
  tags; the engine adds `w.tag_hand` (0.5) of its hand's.
- *Spare energy is cheap: draft and use what spends it* (engine): energy over `w.energy_cap`
  (4) is worth `w.energy_idle` (0.2), and energy an Action or card spends costs the idle share.
- *A cube Action with no megaproject open does nothing* (`w.cube_target`, `plan_cube_target`).

## The progress term and the draft

- **`needs`:** a card short of its `needs` counts `needs_floor` (0.25) of its worth plus the
  rest times the square of the share of the tags held; a dear power (`ai::power_is_dear`),
  `power_progress` (0.5) of a use, times the square of its tags' share.
- **Drafting for the neighbor:** a passed hand's cards count for the next seat, by its strategy.

## The tiers

| Tier | Stands for | Spec | Target win % |
|---|---|---|---|
| Greedy, flat terms | A new player (Matt, 2026-11-03) | `greedy` | none; the strategist's lead over it measures depth |
| Greedy with strategy | The d2 judge | `strategist` | d2 45-85 |
| Planner with strategy rollouts | Spot checks | `planner:strategy=1` | none |
| Oracle | A skilled player with full information: d3 95.8, d4 100 at r73 (174, 24 games each) | `oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1` | d4 95 (Matt, 2026-11-08) |

174 (r73, fresh seeds 41701-41844, 432 games a level, greedy 144): the strategist 94.4 / 71.8 / 58.3 / 40.3 / 33.8
at d0-d4, greedy 94.4 / 75.7 / 68.8 / 43.1 / 41.7; each about 2 ms a decision.
After a change to how the game is won, read greedy beside the strategist in the first smoke (16 dev seeds): a greedy lead means the engine weights still encode the old win (156).
A strategy pairing's win rate is the balance read: `strategist:strat0=I,strat1=J` fixes them;
`aic-sim strategies CONFIG` prints the choices.

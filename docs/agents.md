# Agents

In prose the simulator's players are "sim players"; the code keeps "agents". The skill ladder (`crates/aic-sim/src/agents/`). One team agent drives every seat, hands and drafts open, answers
"players choose" questions and uses the public API only (`legal_actions_into`, `apply_mut`, `apply_to`, state).

- **No peeking.** Before a search an agent replaces what the printed game hides with a guess from its own RNG
  (`agents/determinize.rs`); a test checks hidden information alone moves no decision.
- **Own randomness:** the game seed on the agent kind's stream.
- **Decides at the top level:** outside the action phase the planner follows its line. A node placement (a node power, a card) is scored deeper
  than `choices` (`search.rs`: `PLACING_PROMPTS`, `SACRIFICE_PROMPTS`).
- **`choices` counts prompts** (a placement is two: track, hex; moves take a batch a prompt). A discard prompt
  resolves no further (`discards` 0).
- **Heuristic** (`agents/eval.rs`, `Weights`, a rule on each field): the board, holdings, Alignment (`w.tier_read`),
  projects, the adversary's research track (`w.research`: its share of the threshold, as a project), and unpaid threats by **the tempo trade** (`w.tempo`: harm over what paying now costs).

| Spec | What it does | Budget parameters (defaults) |
|---|---|---|
| `random` | Uniform over legal actions. | none |
| `greedy` | One action at a time, scored by the heuristic after it, its choices resolved best-first; a pass scores the table as it is (or stalling beats stopping). | `samples` 1 (guesses averaged), `depth` 1 (actions ahead), `choices` 2, `discards` 0 |
| `beam` | Beam search over the round's remaining action sequences, scored at the next round's start; replans when a draw or reveal differs from its guess. | `width` 6, `depth` 120 (steps), `branch` 0 (children per sequence, 0 = all), `choices` 2 |
| `rollout` | The beam's `candidates` best end-of-round positions, each run `horizon` more rounds by `rollouts` greedy playouts on fresh guesses; best mean wins. | `width` 4, `depth` 120, `branch` 0, `choices` 2, `candidates` 3, `rollouts` 4, `horizon` 2 |
| `engine` | The heuristic in **horizon mode** (`w.horizon`: income × the rounds left, discounted by danger); holdings by the value table (`w.table=0`: flat terms); the **deck clock** (`w.clock`) charges each round the team's Unity rate would leave it short. Searched like greedy; `width` > 0: a beam, `candidates` > 0: rollouts. | `width` 0, `samples` 1, `choices` 2, `answers` 99 (prompts an answer looks on; 1 loses wins); weights: `Weights::engine` |
| `strategist` | **The screens' judge**: the engine in flat mode (`w.horizon=0`, `w.table=0`): greedy's flat weights, one action at a time, no search, no strategy, plus the **purge term** (`w.purge`: ambassadors toward Liberate bills), **cheap hand terms** (`w.hand`, `w.hand_cost`), **a building's earnings** (`w.card_action`) and the rest of `STRATEGIST`'s list (each `Weights` field in `agents/eval.rs` says what it does). | `STRATEGIST`, `agents/mod.rs` |
| `planner` | The engine, rolling out a shortlist of action-phase alternatives `horizon` rounds with the engine as policy; the heuristic's best line is taken. A line runs on **full information** but the Alignment deal, valued over `deals` guesses of it (`planner.rs`). | `horizon` 2, `alts` 2, `budget` 8, `margin` 40, `deals` 4, `lead` 0. `sweep[:...]` is `planner:deals=1,lead=0`. |
| `oracle` | **A measuring stick, opt in**: the planner's search on **full information**, the deal too, each line to the game's end; it follows a winning line, so its win rate is a floor on perfect play (`oracle.rs`). **`worlds`** (249, the real
oracle; alias `real`, the fair yardstick since 317: the engine's eval, 4 worlds, 128 lines a round): the search on worlds sampled from what players
see; a candidate is taken by worlds won, then mean heuristic; `budget` counts lines (it sets the decisions searched:
314); `tries` there only measures (it reads the
true loss). **`tries`**: whole-game restarts on other policy seeds (`truth` every other one); **`truth`**: lines score the (world's) table; **`prove`**: a card proof. | the planner's four, `reseeds` 0, `threads` 1, `tries` 1, `truth` 0, `prove` 0 |

**Keeping by plan (`agents/keep_plan.rs`, `plan_keep`).** Greedy, engine and planner keep drafts by a plan. At its
first keep of a round a seat sketches `plan_rounds` rounds from what it holds and earns, fills each round's budget
with the cards it would play by value per cost (value: the table at the rounds then left × the strategy multiplier;
cost: its price at the table's rates; `plan_table=0`: the printed price), and keeps each drafted card, best ratio first, only if the
plan is worth more with it; a play a round later is worth 15% less. Its switches (`plan_*`), `no_spend` and each **dominance check** (principles.md) are `forced_table!` rows in
`agents/policy.rs`, each with its default and rule.

**Agent behaviors** (`behavior_table!` in `agents/policy.rs`, each row stating its rule): moves (`agents/moves.rs`),
strikes, targets, playing the plan, selling a dead card, saving and making antimatter; on for every
agent, and a dominance check's ban wins. Forecasts read the engine's own targeting (`modules/plan.rs`).

**Decision order** (`Policy::choose`; first pick wins): strategies at the round-1 keep; a keep by plan or forced rule;
the ambassador, free-build and cube rushes; convert for a play (`convert_rule`); sell; strike, target, moves; the other forced
rules and `kept_rule` 2; else the search over what no ban or antimatter reserve rules out, the plan's card replacing
its pick under `kept_rule` 1, 3, 4.

Spec syntax: `kind[:key=value,...]`; a weight takes `w.` (`greedy:w.corrupted=40`), a switch goes bare
(`strategist:plan_bury=0`). Greedy and the strategist read flat terms: the table's and horizon's levers reach their
keep plan, and the table's rates the strategist's buildings' earnings (`w.card_action`) too. Sweeps:
[`scripts/tune/`](../scripts/tune/README.md); no-op weights: scripts.md.

## The value table

`configs/agents/values.json`, read by `agents/value.rs`: the rates' one home, in credits (card-pricing.md, *The
rates*; `Weights` are points). A card is worth its effects now, its standing effects a round × the rounds it counts
for, a use of its Action × `uses_share` of them (a token card's spend over its cycle of adds), and its tags, over the
rounds left (`horizon_t1`, `horizon_t2`). An `overrides` row adds flat credits to one card, with its reason, where the
table under-prices it against the anchors; built, the card keeps it, spread over the rounds left; never one use of its Action (`value.rs` `one_use`). `t.<key>=v` in a
spec sets one number for that agent (`t.<section>.<key>` where a name is in two; `t.overrides.<card>=N` replaces a
card's `overrides` rows with one of N credits, 287). Its resources, a drawn card and the tier-2 horizon sit at the
unit anchors; its agent-removal and drain rows stay under them. **Antimatter's one rate** (`w.am_tie`, 288):
greedy, the judge, beam and rollout hold an antimatter, plan it (`plan_rates`) and price a card's at 2/3 of the
table's 18 (a spec's `w.antimatter` turns it off). The engine and its kin keep 2 points and `ANTIMATTER_COST` 2.
**Idle antimatter** (`w.am_cap` 1, `w.am_idle` 0.2, the judge's since 303; flat mode): beyond 1, or what its hand's
cards (needs met) spend, a held one counts a fifth, as does a card's, up to that count, in the keep plan. Held
antimatter is read in four places: the eval's flat worth (`am_value`), `sink_terms` (fuel, at the full
`w.antimatter`), the keep plan's repriced table (`policy.rs` `plan_ctx`) and `policy::am_reserve` (`am_save`).

The heuristic reads it in table mode (`w.table`, on for `engine`, off for `greedy`; the field says what each holding
counts):

- **A card in hand:** `hand_share` (under 1) of its net worth, times the chance of paying and the progress term.
- **A card to draft or keep:** its value × the strategy multiplier, less `pick_cost` (0: the keep plan judges what a
  seat can pay) of its play cost, less its keep × `packet_keep` in the packet; × `hand_share`, then `drafted`.
- **A trigger or threshold:** as the `when` and `threshold` rows' `per` say (`own_mix`, `mix_prior`, `own_fill`,
  `raised_rounds`); insight off every card: card-pricing.md.
- **A token:** what its card's spend pays per token (a cash-out's rate, D24), else the held rate.
- **An action:** a flat worth for what the board terms can't see after it, a cordon, a peek, a shelter (`Eval::bonus`;
  what a shelter saves: the guide, below).

## Strategies

`configs/agents/strategies.json`, read by `agents/strategy.rs`. With `strategy=1`, at its round-1 keep a seat commits
to its board's strategy whose multipliers add most to its opening hand and powers (a power at one use's worth);
`strat0`-`strat2` fix one, 1-based (on the board's menu; with `strategy=0`, every seat pinned). A card's multiplier is 1 plus the weights it matches (its triggers' ops too),
times its categories' largest **phase schedule** multiplier (`w.strat_scale` stretches it).

As a player says them (the config's `says`):

<!-- generated: strategies -->
**The Builder**

- *Industry income.* Play Industry income early (Gigafactory Charter, Robotic Foundry, Economies of Scale), take a credit on each Industry play with Supplier Network, and stack Industry tags toward Arcology Foundry and Vertical Monopoly; spend two spent Industry cards on Scrap II.
- *Power grid.* Take Power cards and energy income (Baseload Contract, Fusion Startup Stake), climb the Power tags to Fusion Research Campus and Dyson Swarm Tranche, and cash spent Power cards with Overcharge; Power to the People frees a city at the end.
- *Capital finance.* Build Capital piles (Retained Earnings, Conglomerate Headquarters, Capital Injection), let them pay out, and save the credits for Hostile Buyout.
- *Wide build.* Fill five slots with Action buildings, use one a round, and push the card-play track to Master Plan.

**The Protector**

- *Troop wall.* Troops garrison and only strike on a shown icon: let your Wall cover the nodes beside a garrison, Push or remove the troop guarding a city (Patrol, Interdict, Breach and Clear, Security Contractor HQ paying for each) before your ambassador liberates it, and clear a country's agents with Kinetic Interdiction.
- *Counter desk.* Answer threats with the typed jams and counters, build Garrison Command and Joint Task Force, and send Peacekeeping Mission to a corrupted city.
- *Diplomacy and liberation.* Take Diplomacy and Followers (Liaison Corps, Envoy Briefings, Open Assembly, Accord's ambassador), and free cities with Great Liberation and Popular Uprising.
- *Civic shelter.* Keep what you have standing: Patrol II, Faraday Yard, Hardpoint Lockdown, Aegis Grid Lockdown, toward Aegis Doctrine.

**The Researcher**

- *Cubes and stations.* Build stations early (Field Station), put Lab II's cubes onto megaprojects (Moonshot Grant late), Peer Review your nodes, and spend the cubes on Forensic Audit.
- *Science chain.* Stack Science tags: Research Royalties, Citation Engine and Preprint Server first, then Interdisciplinary Institute and Foundation Model Campus.
- *Patience burst.* Save your insight under Patience and spend it in a big round on Lab I and Lab II's draws and on Science plays into Preprint Server.
- *Datasets.* Grow Dataset piles (Telemetry Archive, Synthetic Data Forge, Dataset Scraper) and cash them through Synthetic Data Forge.
- *Team support.* Feed your partner: gifts, Open Science's insight, Coalition Charter's Sway, Hack II against projects, and Root Access for a corrupted city.
<!-- /generated -->

## Strategy-guide heuristics

Rules a player could say that a one-step search misses, each a switch (on), ruled on its field or `forced_table!`
row; the spare-insight rule is every agent's.

- *Shelter what a destruction showing would take, once you can't counter it* (`agents/guide.rs`, the `w.shelter_`
  weights; armor from `board::guards`, D71; a card's reach Shelter).
- *Guard your last node* (the strategist only): `w.last_node` (eval and push rules).
- *Don't bury a building that still earns more than the new one adds* (`plan_bury`).
- *Count your research cubes like credits* (`plan_cubes`).
- *Pick the strategy your board's powers feed* (`strat_powers`).
- *Insight you can't spend is cheap: keep with it*: over `w.insight_cap`, held, income or a keep paid from it, at
  `w.insight_idle` (`keep_spare`, `plan_spare`).
- *A trigger that pays credits is a credits card* (`w.strat_contents`). Insight income: `w.insight_income` (D53).
- *A per-tag card pays at the count you'll play it at*: `tag_count` 5 plus its own tags; the engine adds
  `w.tag_hand` of its hand's.
- *Spare energy is cheap: draft and use what spends it* (engine): over `w.energy_cap`, held or spent, at
  `w.energy_idle`.
- *A cube Action with no megaproject open does nothing* (`w.cube_target`, `plan_cube_target`).

Off (no gain read): `plan_team`. A spec naming a retired probe (`eval::RETIRED`, `policy::RETIRED`) or a value the
agent would misread (`policy::check_param`) is refused with the reason.

## The progress term and the draft

- **`needs`:** a card short of them counts `needs_floor` of its worth plus the rest × the square of the share of the
  tags held; a dear power (`ai::power_is_dear`), `power_progress` of a use × its tags' share squared (table mode only: 0 for greedy and the judge).
- **Drafting for the neighbor:** a passed hand's cards count for the next seat, by its strategy.

## The tiers

| Stands for | Spec | Target win % |
|---|---|---|
| A new player | `greedy` | none; the strategist's lead over it measures depth |
| The judge | `strategist` | the per-level bands ([principles.md](principles.md)) |
| A strategy player, information; spot checks | `strategist:strategy=1`; `planner:strategy=1` | none |
| A skilled player with full information (it sees hidden cards: never a fair player) | `oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3` | d4 95 |
| **The real oracle**: very strong play that can't see the future (the design target) | `real`, the yardstick, `skilled_rows.py`'s `real` row: the engine's eval, 128 lines (316; the alias since 317) | none yet |

After a change to how the game is won, smoke greedy beside the strategist: a
greedy lead means the engine weights still encode the old win. A strategy pairing's win rate (`strat0`,
`strat1`) is the balance read; `aic-sim strategies CONFIG` prints the choices.

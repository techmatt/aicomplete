# Agents

In prose the simulator's players are "sim players"; the code keeps "agents" (Matt, 2026-11-09).

The skill ladder (`crates/aic-sim/src/agents/`). One team agent drives every seat, hands
and drafts open, and answers "players choose" questions too. Agents play
through the public API only (`legal_actions_into`, `apply_mut`, `apply_to`, the state).

- **No peeking.** Before a search an agent replaces what the printed game hides with
  a guess from its own RNG (`agents/determinize.rs`); a test checks hidden information alone
  moves no decision.
- **Own randomness.** Each agent's RNG is the game seed on its kind's stream.
- **Decides at the top level:** outside the action phase the planner follows its line. A node
  power is scored deeper than `choices` (`search.rs`: `PLACING_PROMPTS`, `SACRIFICE_PROMPTS`).
- **`choices` counts prompts** (a placement is two: track, hex). An effect resolved a prompt a piece is scored
  part of the way, so it reads as paid for little: the moves take a batch a prompt for this.
- **A discard prompt resolves no further** (`discards` 0): a card to discard is valued by the table it leaves
  (`discards=99`: resolved as other prompts).
- **Heuristic** (`agents/eval.rs`, `Weights`, a rule on each field): win/loss, cities, last nodes, the adversary's
  pieces, the Discord deck, Alignment (`w.tier_read`), projects, holdings, unpaid threats by **the tempo trade**
  (`w.tempo`: harm over what paying now costs), corruption and troop risk, the Rally, Dispatches, botnets, cubes,
  the liberation push, Patience insight and idle credits.

| Spec | What it does | Budget parameters (defaults) |
|---|---|---|
| `random` | Uniform over legal actions. | none |
| `greedy` | One action at a time, each scored by the heuristic after it, its choices resolved best-first; a pass scores the table as it is (else stalling beats stopping). | `samples` 1 (guesses averaged), `depth` 1 (actions looked ahead), `choices` 2, `discards` 0 |
| `beam` | Beam search over the round's remaining action sequences, scored at the next round's start; replans when a draw or reveal differs from its guess. | `width` 6, `depth` 120 (steps), `branch` 0 (children per sequence, 0 = all), `choices` 2 |
| `rollout` | Across rounds: the beam's `candidates` best end-of-round positions, each run `horizon` more rounds by `rollouts` greedy playouts on fresh guesses; best mean wins. | `width` 4, `depth` 120, `branch` 0, `choices` 2, `candidates` 3, `rollouts` 4, `horizon` 2 |
| `engine` | The heuristic in **horizon mode** (`w.horizon`: income × the rounds left, discounted by danger); holdings by the value table (`w.table=0`: flat terms); the **deck clock** (`w.clock`) charges each round the team's Unity rate would leave it short. Searched like greedy; `width` > 0: a beam, `candidates` > 0: rollouts. | `width` 0, `samples` 1, `choices` 2, `answers` 99 (prompts an answer looks on; 1 loses wins); weights: `Weights::engine` |
| `strategist` | Greedy's flat weights with `w.last_node` and the **purge term** (`w.purge`: supply ambassadors toward the Liberate bills in reach), no search, no strategy (Matt, 2026-11-13: committing to one cost it greedy's 4-point lead; `strategist:strategy=1` is the strategy player). **The screens' judge**. | `STRATEGIST`, `agents/mod.rs` |
| `planner` | The engine, rolling out a shortlist of action-phase alternatives `horizon` rounds with the engine as policy; the heuristic's best line is taken. A line runs on **full information** but the Alignment deal, valued over `deals` guesses of it; decisions inside it are scored on a guess (`agents/planner.rs`). | `horizon` 2, `alts` 2, `budget` 8, `margin` 40, `deals` 4, `lead` 0. `sweep[:...]` is `planner:deals=1,lead=0`. |
| `oracle` | **A measuring stick, opt in**: the planner's search on **full information**, the deal too, each line played to the game's end; a winning line is followed, so the win rate is a floor on perfect play (`agents/oracle.rs`). | the planner's four, `reseeds` 0, `threads` 1 |

**Keeping by plan (`agents/keep_plan.rs`).** Greedy, engine and planner keep drafts by a plan
(`plan_keep`). At its first keep of a round a seat sketches `plan_rounds` rounds from what it
holds and earns, fills each round's budget with the cards it would play, by value per cost
(value: the table at the rounds then left times the strategy multiplier; cost: its price at the
table's rates, `plan_table=0`), and keeps each drafted card, best ratio first, only if the plan
is worth more with it. A play waiting a round is worth 15% less; `needs` may be met by cards
planned earlier, or short now by at most `plan_short` tags; a building over one of the seat's
adds only what it beats the weakest by (`plan_cover`). Each switch's default and rule, and each
**dominance check** (principles.md), is a row of `forced_table!`, `no_spend` too. The eval holds an
antimatter at 2 points. **`plan_am`**: the antimatter the seat's built Actions make once a round (no burn or Reclaim)
counts from the plan's next round, and a shortfall is made from cubes at Make antimatter's price.

**Agent behaviors** (`behavior_table!`, on for every agent; a dominance check's ban wins; each row says its rule).
**Move heuristics** (`move_rules`, `agents/moves.rs`): push agents off the next Corrupt site; gather agents onto a
spare ambassador to cancel; gather ambassadors onto a site the seat can then Liberate. **Strike** (`strike_rule`):
largest batches first. **Converge** (`converge_rule`, `plan::converge_pick`). **Buy energy** (`energy_rule`): when its
credits would sit idle. **Play the plan** (`kept_rule`): the keep plan's card this round over a pass, standard action,
sale or the antimatter action. **Sell a dead card** (`sell_rule`): a card the plan won't play (the deck's last round:
this round; before: over its rounds), when up to three sales let the plan play a card it otherwise wouldn't. **Save
antimatter** (`am_save`): no spend takes the seat below a reachable hand card's antimatter. **Convert for a play**
(`make_short`): cubes to insight or antimatter when the plan then plays a card it can't now; with `am_source` a
plant's Action first, then a board burn. **Botnet reach** (`w.botnet_reach`): an ambassador on a botnet's hex is
worth that share of it.

**Decision order** (`Policy::choose`; the first that picks wins): strategies at the round-1 keep; a keep by plan or
forced rule; the ambassador, free-build and cube rushes; convert for a play; sell; converge, buy energy, strike,
moves; the other forced rules and `kept_rule` 2; else the agent's search over what no ban or antimatter reserve
rules out, the plan's card replacing its pick under `kept_rule` 1, 3, 4.

Spec syntax: `kind[:key=value,...]`; a weight takes `w.` (`greedy:w.corrupted=40`), a plan or strategy switch goes bare (`strategist:plan_bury=0`). Greedy and the
strategist read flat terms: the table's and horizon's levers reach only their keep plan, and a packet card's keep is
2 credits an insight, not `rates.keep_tier2`. No-op weights: `scripts/tune/noop_weights.py`. Sweeps: [`scripts/tune/`](../scripts/tune/README.md).

## The value table

`configs/agents/values.json`, read by `agents/value.rs`: a value in credits for each resource,
tag left in play, token and op, prices' one home (`Weights` are points; card-pricing.md, *The
rates*). A card is worth its effects now,
its standing effects a round times the rounds it counts for, a use of its Action times
`uses_share` of them (a token card's spend over its cycle of adds), and its tags, over the rounds left
(`horizon_t1`, `horizon_t2`). An `overrides` row adds flat credits to one card, with its reason: the scoped fix for a card the
table under-prices against the anchors; built, the card keeps it, spread over the rounds left.
`t.<key>=v` in a spec sets one number for that agent (`t.<section>.<key>` where a name is in two).

The heuristic reads it in table mode (`w.table`: on for `engine`, off for `greedy`; what each
holding counts, a building's earnings included, is on that field):

- **A card in hand:** `hand_share` (under 1) of its net worth, times the chance of paying and
  the progress term.
- **A card to draft or keep:** its value times the strategy multiplier, less `pick_cost` of its
  play cost, less its keep times `packet_keep` in the packet; times `hand_share`, then `drafted`.
  `pick_cost` 0: the keep plan judges what a seat can pay.
- **A trigger:** one on a tag's plays (or a discount on them) fires at the seat's own share of
  the tag's cards (`own_mix`, `mix_prior`), a threshold at its own trigger's fill (`own_fill`); an income a firing raises counts `raised_rounds` rounds; insight off every card: card-pricing.md.
- **A token:** what its card's spend pays per token (a cash-out's rate, D24), else the held rate.
- **An action** is credited what the board terms can't see after it: a cordon, a peek, a
  shelter's flat worth (`Eval::bonus`; what a shelter saves: the guide, below).

## Strategies

`configs/agents/strategies.json`, read by `agents/strategy.rs`. With `strategy=1`, at its round-1
keep a seat commits to its board's strategy whose multipliers add most to its opening hand and
powers (a power at one use's worth); `strat0`-`strat2` fix one, 1-based. A
card's multiplier is 1 plus the weights it matches (its triggers' ops too),
times its categories' largest **phase schedule** multiplier (`w.strat_scale` stretches it).

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

Rules a player could say that a one-step search misses, each a switch (on), ruled on its field or
`forced_table!` row; the spare-insight rule is every agent's (Matt 2026-11-05).

- *Shelter what a destruction showing would take, once you can't counter it* (`agents/guide.rs`,
  the `w.shelter_` weights; armor from `board::guards`, D71).
- *Guard your last node* (the strategist's 120, 0 elsewhere): `w.last_node` (eval, converge and push rules).
- *Don't bury a building that still earns more than the new one adds* (`plan_bury`).
- *Count your research cubes like credits* (`plan_cubes`).
- *Pick the strategy your board's powers feed* (`strat_powers`).
- *Insight you can't spend is cheap: keep with it*: insight over `w.insight_cap` is
  worth `w.insight_idle`, held or as income, and so is a keep paid from it (`keep_spare`, `plan_spare`).
- *A trigger that pays credits is a credits card*: strategies read triggers' and
  thresholds' ops (`w.strat_contents`). Insight income is worth `w.insight_income` (D53).
  Off (no gain read): `plan_team`. Retired probes: `eval::RETIRED`.
- *A per-tag card pays at the count you'll play it at*: table `tag_count` 5 plus the card's own
  tags; the engine adds `w.tag_hand` of its hand's.
- *Spare energy is cheap: draft and use what spends it* (engine): energy over `w.energy_cap`
  is worth `w.energy_idle`, and energy an Action or card spends costs the idle share.
- *A cube Action with no megaproject open does nothing* (`w.cube_target`, `plan_cube_target`).

## The progress term and the draft

- **`needs`:** a card short of its `needs` counts `needs_floor` of its worth plus the
  rest times the square of the share of the tags held; a dear power (`ai::power_is_dear`),
  `power_progress` of a use, times the square of its tags' share.
- **Drafting for the neighbor:** a passed hand's cards count for the next seat, by its strategy.

## The tiers

| Tier | Stands for | Spec | Target win % |
|---|---|---|---|
| Greedy, flat terms | A new player (Matt, 2026-11-03) | `greedy` | none; the strategist's lead over it measures depth |
| Greedy with the purge term and last-node guard | The judge | `strategist` | the per-level bands ([principles.md](principles.md)) |
| Greedy with strategy | A strategy player, information | `strategist:strategy=1` | none |
| Planner with strategy rollouts | Spot checks | `planner:strategy=1` | none |
| Oracle | A skilled player with full information | `oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1` | d4 95 (Matt, 2026-11-08) |

The current ladder: [principles.md](principles.md). After a change to how the game is won,
smoke greedy beside the strategist: a greedy lead means the engine weights still encode the old win.
A strategy pairing's win rate (`strat0`, `strat1`) is the balance read; `aic-sim strategies CONFIG` prints the choices.

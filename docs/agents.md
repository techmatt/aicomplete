# Agents

The skill ladder (`crates/aic-sim/src/agents/`). One team agent drives every seat, hands
and drafts open, and answers "players choose" questions too. Agents play
through the public API only (`legal_actions_into`, `apply_mut`, `apply_to`, the state).

- **No peeking.** Before a search an agent replaces what the printed game hides with
  a guess from its own RNG (`agents/determinize.rs`); a test checks hidden information alone
  moves no decision. The planner's lines and the oracle's read the table (below).
- **Own randomness.** Each agent's RNG is the game seed on its kind's stream, so games
  replay from seed + actions whatever `--threads`.
- **Decides at the top level:** outside the action phase the planner follows its line. A node
  power is scored deeper than `choices` (`search.rs`: `PLACING_PROMPTS`, `SACRIFICE_PROMPTS`).
- **`choices` counts prompts** (a placement is two: track, hex).
- **Heuristic** (`agents/eval.rs`, `Weights`): win/loss, cities, a seat's last nodes, the
  adversary's pieces, the Discord deck's last cards, Alignment (156: the next card's tier, `w.tier_read`;
  the horizon aims at the purge's last step), projects, holdings, unpaid threats (the tempo
  trade, below); and:
  - **the tempo trade** (147, `w.tempo`): an unpaid threat weighs its one-off harm plus what it leaves
    on the table times the rounds left, over what paying now costs the engine (`lasting` in `eval.rs`);
  - **corruption risk** (147, `w.corrupt_exposure`, `w.blight`): a node on a site filling toward corruption,
    that share lost; land corruption, its Discord drip;
  - **the coming Rally** (151, `w.rally_read`): `plan::next_rally`'s expected ambassadors count toward each
    site's corruption a round ahead;
  - **Movement** (149, `w.movement`): a seat's free moves a round over the rounds left;
  - botnets (−25, D9), cubes, converter cities, a corrupted station, the liberation push, a darkened city,
    Patience insight and idle credits each have a weight (`Weights` in `eval.rs`).

| Spec | What it does | Budget parameters (defaults) |
|---|---|---|
| `random` | Uniform over legal actions. | none |
| `greedy` | One action at a time, each scored by the heuristic after it, its choices resolved best-first; a pass scores the table as it is (else stalling beats stopping). | `samples` 1 (guesses averaged), `depth` 1 (actions looked ahead), `choices` 2 |
| `beam` | Beam search over the round's remaining action sequences, scored at the next round's start; plans once, replans when a draw or reveal differs from its guess. | `width` 6, `depth` 120 (steps), `branch` 0 (children per sequence, 0 = all), `choices` 2 |
| `rollout` | Across rounds: the beam's `candidates` best end-of-round positions, each run `horizon` more rounds by `rollouts` greedy playouts on fresh guesses; best mean wins. | `width` 4, `depth` 120, `branch` 0, `choices` 2, `candidates` 3, `rollouts` 4, `horizon` 2 |
| `engine` | The heuristic in **horizon mode**: income is worth `w.horizon` × its weight × the rounds left (the adversary deck's clock, discounted by danger); holdings by the value table (`w.table=0`: flat terms); the **deck clock** (`w.clock`) charges each round the team's Unity rate would leave it short. Searched like greedy; `width` > 0: a beam, `candidates` > 0: rollouts. | `width` 0, `samples` 1, `choices` 2, `answers` 99 (prompts an answer looks on; 1 loses wins); weights: `Weights::engine` in `eval.rs` |
| `strategist` | `engine:strategy=1`: greedy with strategy, no search. **The screens' judge** (*The tiers*). | the engine's |
| `planner` | The engine, rolling out a shortlist of action-phase alternatives `horizon` rounds with the engine as policy; the heuristic's best line is taken. A line runs on the real state but the Alignment deal (**full information**), valued over `deals` guesses of it; decisions inside a line are scored on a guess. The followed line is reused. | `horizon` 2, `alts` 2, `budget` 8, `margin` 40, `deals` 4, `lead` 0 and the rest in `agents/planner.rs`. `sweep[:...]` is `planner:deals=1,lead=0`. |
| `oracle` | **A measuring stick, opt in** (155): the planner's search on **full information**, the Alignment deal too, each line played to the game's end. A winning line is followed: the seed is proved winnable, so the win rate is a floor on perfect play. Others compare on the heuristic `horizon` rounds on; `reseeds`: lines on other policy guesses at a round's first searched decision. | `horizon` 2, `alts` 2, `budget` 8, `margin` 40, `reseeds` 0, `threads` 1 (lines side by side, same play) (`agents/oracle.rs`) |

**Keeping by plan (`agents/policy.rs`).** Greedy, engine and planner keep drafts by a plan
(`plan_keep` 1; 0: the heuristic keeps). At its first keep of a round a seat sketches
`plan_rounds` (4) rounds from what it holds and earns, fills each round's budget with the
cards it would play, by value per cost (value: the table at the rounds then left times
the strategy multiplier; cost: its price at the table's rates, `plan_table=0`),
and keeps each drafted card, best ratio first, only if the plan is worth more with it.
A play waiting a round is worth 15% less; `needs` may be met by cards planned earlier, or short
now by at most `plan_short` tags; a building over one of the seat's adds only what it beats the
weakest by (`plan_cover`); each credit paid for keeps costs `plan_credit`. The other switches
(`plan_niche`, `plan_reach`, `pile_ok`, `plan_copy`, `plan_under`) say their rule in `forced_table!`.

Priced like the play check (102): a card worth no more than its price is not planned, one
that is adds its **gross value** (the prices: `plan_value` in `policy.rs`).

**Dominance checks**: one behavior forced, a row of `forced_table!` (`agents/policy.rs`; principles.md).

Spec syntax: `kind[:key=value,...]`; a weight takes `w.` (`greedy:w.corrupted=40`), a table
number `t.` (*The value table*), a plan or strategy switch goes bare (`strategist:plan_bury=0`). `greedy` reads flat
terms, so the table's and horizon's levers reach only its keep plan. Sweeps: [`scripts/tune/`](../scripts/tune/README.md).

## The value table

`configs/agents/values.json`, read by `agents/value.rs`: a value in credits for each resource,
tag left in play, token and op, prices' one home (`Weights` are points; card-pricing.md, *The
rates*). Every op has its arm and row (`every_op_has_a_row`). A card is worth its effects now,
its standing effects a round times the rounds it counts for, a use of its Action times
`uses_share` of them (a token card's spend over its cycle of adds), and its tags. The rounds are
those left in a game; in the typical context (the opening's choice, the effect ledger) its
tier's horizon, `horizon_t1` 4, `horizon_t2` 2.5. "Remove up to N troops" counts
`troops_removed` (D26). An `overrides` row adds flat credits to one card, with its reason: the scoped fix where the
table under-prices a card's output against the anchors; built, the card keeps it, spread over the rounds left (130).
`t.<key>=v` in a spec sets one number for that agent (`t.<section>.<key>` where a name is in two).

The heuristic reads it in table mode (`w.table`: on for `engine`, off for `greedy`):

- **A card in hand:** `hand_share` of its net worth (value less play cost, times the chance
  of paying and the progress term); under 1, or holding a card would be as good as playing it.
- **A card to draft or keep:** its value times the strategy multiplier, less `pick_cost` of its
  play cost, less its keep times `packet_keep` in the packet; times `hand_share`, then `drafted`.
  `pick_cost` 0: the keep plan judges what a seat can pay.
- **A building in play:** what it earns a round beyond income, times the rounds left.
- **A trigger:** one on a tag's plays (or a discount on them) fires at the seat's share of cards
  with the tag in hand and in play, plus `mix_prior` cards at `tag_share` (`own_mix`; the ledger
  reads `tag_share`); a threshold its card's own trigger fills fills at that rate (`own_fill`);
  an income a firing raises counts `raised_rounds` rounds; insight off every card counts
  `insight_discount_share` of the plays (the discount floor).
- **A token:** what its card's spend pays per token (a cash-out's rate, D24), else the held rate.
- **An action** is credited what the board terms can't see after it: a cordon, a peek, a
  shelter's flat worth (`Eval::bonus`; what a shelter saves: the guide, below).

## Strategies

`configs/agents/strategies.json`, read by `agents/strategy.rs`. With `strategy=1`, at its round-1
keep a seat commits to its board's strategy whose multipliers add most to its opening hand and
powers (a power at one use's worth, `strat_powers`); `strat0`-`strat2` fix one, 1-based. A
strategy weights a card's tags, the resources and tokens it pays in or out (its triggers' too),
its kind and categories; a multiplier is 1 plus the weights matched, times its categories'
largest **phase schedule** multiplier (`w.strat_scale` stretches it). It reaches drafting and
keeping, not a card in hand. Planner rollouts keep the choice.

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

Rules a player could say where a one-step search misses a card's point, each a switch, on
(092); 124's spare-insight rule is every agent's (Matt 2026-11-05).

- *Shelter what a destruction showing would take, once you can't counter it* (`agents/guide.rs`,
  `w.shelter_read`, `w.shelter_node`, `w.shelter_counter`), armor read as the engine does
  (`w.shelter_armor`, D71; one pool a hex, 148).
- *Don't bury a building that still earns more than the new one adds* (`plan_bury`: slots
  full, the keep plan charges a plain building what the top it buries would earn).
- *Count your research cubes like credits* (`plan_cubes`: a budget in the keep plan).
- *Pick the strategy your board's powers feed* (`strat_powers`).
- *Insight you can't spend is cheap: keep with it* (124): insight over `w.insight_cap` (10) is
  worth `w.insight_idle` (0.1), held or as income, and so is a keep paid from it (`keep_spare`, `plan_spare`).
- *A trigger that pays credits is a credits card* (124): strategies read triggers' and
  thresholds' ops (`w.strat_contents`). Insight income is worth `w.insight_income` (1.5, D53).
  Off (no gain read): `w.team_share`, `plan_team`, `w.threshold_tokens`.
- *A per-tag card pays at the count you'll play it at* (128): table `tag_count` 5 (the median
  before such a play) plus the card's own tags; the engine adds `w.tag_hand` (0.5) of its hand's.
- *Spare energy is cheap: draft and use what spends it* (128, engine): energy over `w.energy_cap`
  (4) is worth `w.energy_idle` (0.2), and energy an Action or card spends costs the idle share.
- *A cube Action with no megaproject open does nothing* (128: `w.cube_target`, `plan_cube_target`).

## The progress term and the draft

- **`needs`:** a card short of its `needs` counts `needs_floor` (0.25) of its worth plus the
  rest times the square of the share of the tags held.
- **Dear powers** (`ai::power_is_dear`): `power_progress` (0.5) of a use, times the square of
  the share of its tags held.
- **Drafting for the neighbor:** a passed hand's cards count for the next seat, by its strategy.

## The tiers

| Tier | Stands for | Spec | Target win % |
|---|---|---|---|
| Greedy, flat terms | A new player (Matt, 2026-11-03) | `greedy` | none; the strategist's lead over it measures depth |
| Greedy with strategy | The d2 judge | `strategist` | d2 45-85 |
| Planner with strategy rollouts | Spot checks | `planner:strategy=1` | none |
| Oracle | A skilled player with full information (155): d3 100, d4 96 at r66 | `oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1` | d4 95 (Matt, 2026-11-08) |

The strategist judges (within about 5 points of `sweep` a pair on the same seeds) at a tenth of
its time a game; `sweep` spot-checks.
A strategy pairing's win rate is the balance read: `strategist:strat0=I,strat1=J` fixes them;
`aic-sim strategies CONFIG` prints the choices.

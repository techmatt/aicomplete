# Agents

The skill ladder (`crates/aic-sim/src/agents/`). One team agent drives every seat, with
hands and drafts open, and answers the "players choose" questions too. Agents play
through the public API only (`legal_actions_into`, `apply_mut`, `apply_to`, reading the state).

- **No peeking.** Before any search an agent replaces what the printed game hides with
  a guess from its own RNG (`agents/determinize.rs`: what is unseen redealt, RNG streams
  reseeded). A test checks hidden information alone moves no decision.
- **Own randomness.** Each agent's RNG is the game seed on a stream of its kind's, so games
  replay from seed + actions whatever `--threads`.
- **Decides at the top level:** outside the action phase the planner follows its line. A node
  power is scored deeper than `choices` (`search.rs`: `PLACING_PROMPTS`, `SACRIFICE_PROMPTS`).
- **`choices` counts prompts, not effects:** one placement is two (a track, a hex).
- **A free multi-pick rule costs N²:** agents answer every prompt (`answers` 99), so a rule open at
  every decision with N picks costs about N² evaluations a decision (105's Synthesis: 2× the time).
- **Heuristic** (`agents/eval.rs`; each weight, its default and use a field of
  `Weights`, set by `w.<name>=V` in a spec; hand-set). It scores win/loss; cities
  (corrupted, aligned, the pressure on the rest); a seat's last nodes (`node_danger`); the
  adversary's pieces; the decks' last three cards; Alignment and project fill; the players'
  board, income, resources and cards; unpaid threats. Choices that aren't obvious:
  - botnets −25 (D9): enough that agents make none when an equal free choice exists;
  - `research_cube` 0.3, under what every converter nets; `research_cube_power` 1.2 for a
    seat whose board spends cubes on a power (Peer Review); `converter_city` 2 for a seat with a
    node in a city of a completed megaproject's country, whatever its cubes (`w.converter_any`);
  - a corrupted station's `station_flip` 20 scaled by its country's megaproject still open, 0
    once completed (`w.flip_open`);
  - the liberation push needs `liberate_at` plus the city's own adversary ambassadors (`w.push_adv`);
  - a city an adversary ambassador darkens: `city_dark` 4 for a neutral one in some seat's
    reach, `city_dark_aligned` 6 for an aligned one;
  - a Patience seat's insight carries over, so it keeps its worth at the round's end (`patience` 1.0);
  - credits beyond `w.credit_cap` (12) a seat are worth `w.credit_idle` (0.15).

| Spec | What it does | Budget parameters (defaults) |
|---|---|---|
| `random` | Uniform over legal actions. | none |
| `greedy` | One action at a time: each scored by the heuristic after it, its own choices resolved best-first. A pass scores the table as it is (else stalling beats stopping). | `samples` 1 (guesses averaged), `depth` 1 (actions looked ahead), `choices` 2 |
| `beam` | Beam search over the rest of the round's action sequences, scored at the start of the next round; plans once and replans when a draw or reveal differs from its guess. | `width` 6, `depth` 120 (steps), `branch` 0 (children per sequence, 0 = all), `choices` 2 |
| `rollout` | Across rounds: the beam's `candidates` best end-of-round positions, each continued by `rollouts` greedy playouts for `horizon` more rounds on fresh guesses; best mean wins. | `width` 4, `depth` 120, `branch` 0, `choices` 2, `candidates` 3, `rollouts` 4, `horizon` 2 |
| `engine` | The heuristic in **horizon mode**: income is worth `w.horizon` × its weight × the rounds left (the adversary deck's clock, discounted by danger); cards, buildings, tokens and supply ambassadors by the value table (`w.table=0`: flat terms); the **deck clock** (`w.clock`) charges each round the team's Unity rate would leave it short. Searched like greedy; `width` > 0 makes it a beam, `candidates` > 0 adds rollouts. | `width` 0, `samples` 1, `choices` 2, `answers` 99 (prompts an answer looks on; 1 loses wins, 103); weights: `Weights::engine` in `eval.rs` |
| `strategist` | `engine:strategy=1`: greedy with strategy, no search. **The screens' judge** (*The tiers*). | the engine's |
| `planner` | The engine, with a shortlist of alternatives in the action phase, each rolled out with the engine as policy `horizon` rounds on; the best line by the heuristic is taken. A line runs on the real state but the Alignment deal (**full information**, Matt, 2026-10-29), valued over `deals` guesses of the deal; decisions inside a line are scored on a guess. The followed line is reused. | `horizon` 2, `alts` 2, `budget` 8, `margin` 40, `deals` 4, `lead` 0 and the rest in `agents/planner.rs`. `sweep[:...]` is `planner:deals=1,lead=0`. |

**Keeping by plan (`agents/policy.rs`).** Greedy, engine and planner keep drafted cards by a plan
(`plan_keep` 1; 0 lets the heuristic keep): at its first keep of the round a seat sketches
`plan_rounds` (4) rounds from what it holds and earns, fills each round's budget with the
cards it would play in value-per-cost order (value: the value table at the rounds then left
times the strategy multiplier; cost: this seat's price at the table's rates; `plan_table=0`
uses printed costs), and keeps each drafted card, best ratio first, only if the plan is worth
more with it. A play waiting a round is worth 15% less; `plan_niche` adds for each marker,
threat or node a card would act on now; `needs` may be met by cards planned earlier, short now by at most `plan_short`
tags, counting the hand's and keeps' tags too for a card on the seat's strategy's tags (107, `plan_reach`); a card with
no target now may be planned from the second round at `target_share`, unless it works a pile of the seat's own none
stands or is planned for (`pile_ok`), or copies an income with no income building built or planned (`plan_copy`); a
building over one of the seat's (built, or planned first: `plan_under`) adds only what it beats the weakest by
(`plan_cover`), the covered one counted at what it would still earn (`plan_cover_spent`, 102); credits paid for keeps cost `plan_credit` each. `plan_chain` (107, off: it
kept chain cards never played) credits a chain card the plan brings within `plan_short` tags.

As the play check prices a card: a card worth no more than its price is not
planned (`plan_net`), and one that is adds its **gross value** (`plan_gross`, Matt, 104;
`plan_gross=0` adds value less price); credits over `w.credit_cap` cost `w.credit_idle`
(`plan_idle`); energy and antimatter at the heuristic's rates (about 1.3 and 3.3 credits), a
building's Action at the table's (`plan_rates`); a project discard or drain at what the
heuristic charges for the projects up.

**Dominance checks**: one behavior forced, a row of `forced_table!` (`agents/policy.rs`; principles.md).

Any agent can use horizon mode (`greedy:w.horizon=1,...`); `engine` only changes the
defaults. Its weights' sweep: [`scripts/tune/`](../scripts/tune/README.md).

Spec syntax: `kind[:key=value,...]`: a weight takes `w.` (`greedy:w.corrupted=40`), a keep-plan
or strategy switch goes bare (`strategist:plan_bury=0`).

## The value table

`configs/agents/values.json`, read by `agents/value.rs`: one value in credits for each
resource, tag left in play, token and op (the rates' one home for prices; the search's `Weights` are points, and
card-pricing.md, *The rates*, says which shadow a row and why). Every
op has its arm (no catch-all) and its row (`every_op_has_a_row`). A card is worth its ops: what its effects do now, what
its standing effects pay a round times the rounds it counts for, a use of its Action times
`uses_share` of those rounds (a progressive token card's spend over the uses its cycle of
adds takes), and its tags. In a game those rounds are the rounds left; at the table's typical
context (the opening's strategy choice, the effect ledger) they are its tier's horizon,
`horizon_t1` 4 and `horizon_t2` 2.5 (the rounds left at a typical purchase less a tempo discount).
"Remove up to N troops" counts the troops it removes in play, the table's `troops_removed` (102,
D26: 6 → 3.6), which the effect ledger reads too.

Where the heuristic reads it in table mode (`w.table`; the `engine`'s default, off for `greedy`):

- **A card in hand:** `hand_share` of its net worth (value less play cost, times the chance
  of paying and the progress term); under 1, or holding a card would be as good as playing it.
- **A card to draft or keep:** its value times the strategy multiplier, less `pick_cost` of its
  play cost, less its keep times `packet_keep` in the packet; times `hand_share`, then `drafted`. `pick_cost` is 0: the keep
  plan judges what a seat can pay (at 1 the engine drafts tier 1 only).
- **A building in play:** what it earns a round beyond income, times the rounds left.
- **A trigger (107):** one on a tag's plays (or a discount on them) fires at the seat's share of cards carrying the tag,
  in hand and in play, `mix_prior` cards at `tag_share` added (`own_mix`; the ledger reads `tag_share`); a threshold its
  card's own trigger fills fills at that rate (`own_fill`); an income a firing raises counts `raised_rounds` rounds;
  insight off every card counts `insight_discount_share` of the plays (the discount floor).
- **A token:** what its card's spend pays per token, else the held rate; on a card whose
  Action cashes its own pile out, the cash-out's rate (`w.token_cash`, 102, D24).
- **A supply ambassador:** its row.
- **Energy:** `energy` 0.8 points (0.3 of the table's 4.5; `w.table_energy` 1 played worse).
- **An action** is credited what the board terms can't see after it: a cordon, a peek, a
  shelter's flat worth (`Eval::bonus`; what a shelter saves now is the guide's, below).

## Strategies

`configs/agents/strategies.json`, read by `agents/strategy.rs`. At its round-1 keep each seat
commits to the strategy of its board's menu whose multipliers add most to its opening hand and
its board's powers, a power read as a card at one use's worth (`strat_powers`,
`strat_power_uses` 1) (`strategy=1`; `strat0`-`strat2` fix a seat's, 1-based). A strategy weights
a card's tags, the resources and tokens it pays in or out, its kind and its categories;
a card's multiplier is 1 plus the weights it matches, times the **phase schedule** for its
categories (early, mid and late multipliers in `strategies.json`; a card takes its
categories' largest; `w.strat_scale`, 1, stretches its distance from 1). It reaches drafting and keeping, not a card in hand (a multiplied
hand would beat playing it). The planner hands the choice to every rollout's policy.

As a player says them (the config's `says`, written in by `aic-sim docs`):

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

Rules a player could say aloud where a one-step search misses a card's point (Matt,
2026-11-03), each a switch, all on (`experiments/results/screen-092/`).

- *Shelter what a destruction showing would take, once you can't counter it.* A sheltered
  country is worth the nodes and player factories this round's destroying threats and war
  step would take there, less what hardening or an ambassador absorbs (`agents/guide.rs`;
  `w.shelter_read`; `w.shelter_node` 12 a node over `node` and the danger step;
  `w.shelter_counter` 1 skips a threat the team holds the counter for).
- *Don't bury a building that still earns more than the new one adds* (`plan_bury`: with
  every slot full, the keep plan charges a plain building what the top it buries would earn).
- *Count your research cubes like credits* (`plan_cubes`: a budget in the keep plan).
- *Pick the strategy your board's powers feed* (`strat_powers`).

## The progress term and the draft

- **`needs`:** a card whose `needs` aren't met counts `needs_floor` (0.25) of its worth,
  plus the rest times the square of the share of the tags held toward them: a chain card has
  value before it is playable, and each tag toward it adds more than the last.
- **Dear powers:** a dear board power (`ai::power_is_dear`; `w.power_dear`) counts `power_progress` (0.5) of one
  use's worth, times the square of the share of the tags held toward it.
- **Drafting for the neighbor:** a passed hand's cards count for the next seat, by its
  strategy (a card not kept is discarded).

## The tiers

| Tier | Stands for | Spec | win %, bp / br / pr, r40 (102, seeds 32900, 200 a pair) |
|---|---|---|---|
| Greedy without strategy | A new player | `engine` | d0 56 / 73 / 60 (target 70-85) |
| Greedy with strategy | A moderately skilled player | `strategist` | d2 28 / 42 / 32 (target 45-85: every pair out of band, not retuned) |
| Planner with strategy rollouts | A strong player; spot checks of the ceiling | `planner:strategy=1` | not read at r40 |

The strategist judges (Matt, 2026-11-03: within about 5 points of `sweep` a pair on the
same seeds) at about a tenth of its time a game; `sweep` is the spot check. A strategy
pairing's win rate is the balance read: `strategist:strat0=I,strat1=J` fixes them, and
`aic-sim strategies CONFIG` prints what each seat chooses.

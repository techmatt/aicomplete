# Agents

The skill ladder (`crates/aic-sim/src/agents/`). One team agent drives every seat, with
hands and drafts open, and answers the "players choose" questions too. Agents play
through the public API only (`legal_actions_into`, `apply_mut`, `apply_to`, reading the state).

- **No peeking.** Before any search an agent replaces what the printed game hides with
  a guess from its own RNG (`agents/determinize.rs`): the player-card and threat
  decks are shuffled; the Unity and Discord piles are dealt again from their tier's unresolved
  printed cards; the project deck within each stage; each hidden adversary-deck position gets an unseen card of its stage (the
  top cards stay put under Recon Sweep); RNG streams are reseeded. A test changes only hidden information
  and checks that decisions stay the same.
- **Own randomness.** Each agent's RNG is the game seed on a stream named after the agent
  kind, apart from every engine stream, so games replay from seed + actions whatever `--threads`.
- **Decides at the top level:** outside the action phase (`searched_phase` in `planner.rs`) the
  planner follows its line, so draft and keeps are the rollout policy's. A node power is scored at
  `choices` × 2 at the top level, 2 more when a sacrifice's prompts come first, and at
  `choices`, at least 3, inside rollouts (`search.rs`: `PLACING_PROMPTS`, `SACRIFICE_PROMPTS`).
- **`choices` counts prompts, not effects.** One placement is two prompts (a track, then a hex;
  a one-option prompt doesn't count), so `choices` 2 scores one placement. Only a power
  that places gets more: a card that asks more (Relay Mast Rollout, Aegis Grid Lockdown,
  Beamed Power Constellation, Open-Weights Release) is judged part of the way, and beam's
  search doesn't double at all (only its greedy fallback does).
- **Heuristic** (`agents/eval.rs`; every weight, its default and what it does is a field of
  `Weights`, set by `w.<name>=V` in a spec; hand-set, not swept). It scores win/loss;
  corrupted and aligned cities, the pressure on cities not yet corrupted (squared share of
  the face-up card's `corrupt_at`) and a city one ambassador from corrupting; a seat's last
  nodes (`node_danger`); adversary ambassadors, more on a player piece or factory; troops,
  blockers, markers, shields; the Discord and Unity decks' last three cards (the empty Discord
  tip loses, the last Unity card wins); Alignment fill and each visible adversary project's;
  the players' board (pieces, cubes, insight, the Protector's wall, track progress,
  ambassadors, liberation); income, resources, cards; unpaid threats. Choices that aren't obvious:
  - botnets −25 (D9): low enough that agents don't make one when an equal free choice exists;
  - `research_cube` 0.3, under what every converter nets; `research_cube_power` 1.2 for a
    seat whose board spends cubes on a power (the Researcher's Peer Review);
  - a Patience seat's insight carries over, so it keeps its worth at the round's end (`patience` 1.0);
  - credits beyond `w.credit_cap` (12) a seat are worth `w.credit_idle` (0.15).

| Spec | What it does | Budget parameters (defaults) |
|---|---|---|
| `random` | Uniform over legal actions. | none |
| `greedy` | One action at a time: each scored by the heuristic after it, its own choices resolved best-first. A pass scores the table as it is (else stalling beats stopping). | `samples` 1 (guesses averaged), `depth` 1 (actions looked ahead), `choices` 2 |
| `beam` | Beam search over the rest of the round's action sequences, scored at the start of the next round; plans once, follows the plan while the table matches, replans when a draw or reveal differs from its guess. | `width` 6, `depth` 120 (steps), `branch` 0 (children per sequence, 0 = all), `choices` 2 (with 1, placing nodes, which asks a hex, looks worthless and beam stops growing its engine) |
| `rollout` | Across rounds: the beam's `candidates` best end-of-round positions, each continued by `rollouts` greedy playouts for `horizon` more rounds on fresh guesses; best mean wins. | `width` 4, `depth` 120, `branch` 0, `choices` 2, `candidates` 3, `rollouts` 4, `horizon` 2 |
| `engine` | The heuristic in **horizon mode**: income is worth `w.horizon` × the resource's weight × the rounds left, board production `w.production` of that; the rounds left are the adversary deck's clock, cut short when the team is on course to win and discounted by danger (`w.risk`); cards, buildings, tokens and supply ambassadors are valued by the value table (`w.table=0`: the flat terms); a card in hand counts by the chance it can be paid for before the end; drafted and packet cards count only as far as the round's insight keeps them; resources fade under `w.spend_horizon` rounds left; the **deck clock** (`w.clock`) charges each round the team, at its Unity rate so far, would still be short of the Unity deck's last card when the deck runs out. Searched like greedy by default; `width` > 0 makes it a beam, `candidates` > 0 adds rollouts. | `width` 0, `samples` 1, `choices` 2; beam: `depth` 120, `branch` 0, `candidates` 0, `rollouts` 2, `horizon` 1. Weights: `Weights::engine` in `eval.rs` |
| `strategist` | `engine:strategy=1`: greedy with strategy, no search. **The screens' judge** (*The tiers*). | the engine's |
| `planner` | The engine, with a shortlist of alternatives in the action phase, each rolled out with the engine as policy to the first decision `horizon` rounds on; the best line by the heuristic is taken. A line runs on the real state but the Alignment deal (**full information**, Matt, 2026-10-29), valued as the mean over `deals` guesses of the deal (played once to its first tip, then forked; an alternative must lead by `lead` on the first guess before it and its rival play the rest, then lead on the mean). Each decision inside a line is still scored on a guess: on open decks the one-step heuristic wins far less (it drafts by printed cost). `hide_alignment_deal` is `determinize`'s `redeal_alignment` alone; it reorders every pile it redeals, so at d2 a line sees the fully dealt Discord deck in a guessed order. **Speed:** the followed line is reused, so a decision pays only for its alternatives. | `horizon` 2, `alts` 2 a decision, `budget` 8 a round, `margin` 40 (only actions this close to the engine's best), `choices` 2, `samples` 1, `prune` 12 (actions each seat keeps through a rollout phase after its first full scoring), `powers` 6 (powers kept so; 6, a board's count, is off), `answers` 99 (prompts an answer looks on in a rollout, at most `choices`), `deals` 4, `lead` 0. Keep and dominance parameters are the engine's and hold in its rollouts. `--quick` keeps it. `sweep[:...]` is `planner:deals=1,lead=0`, for price sweeps. |

**Keeping by plan (`agents/policy.rs`).** Greedy, engine and planner keep drafted cards by a plan
(`plan_keep` 1; 0 lets the heuristic keep): at its first keep of the round a seat sketches
`plan_rounds` (4) rounds from what it holds and earns, fills each round's budget with the
cards it would play in value-per-cost order (value: the value table at the rounds then left
times the strategy multiplier; cost: this seat's price at the table's rates; `plan_table=0`
uses printed costs), and keeps each drafted card, best ratio first, only if the plan is worth
more with it. A play waiting a round is worth 15% less; `plan_niche` adds for each marker,
threat or node a card would act on now; `needs` may be met by cards planned earlier, short
by at most `plan_short` tags; a card with no target now may be planned from the second
round at `target_share`, unless it works a pile of the seat's own none stands or is planned for (`pile_ok`); a building
over one of the seat's adds only what it beats the weakest by (`plan_cover`); credits paid
for keeps cost `plan_credit` each.

**Dominance checks** are the same agents with one behavior forced: each is a row of
`forced_table!` in `agents/policy.rs`, with its description (`docs/principles.md`,
*Dominance checks*, lists the gate's). `ignore_threats=1` also refuses an automatic-counter
capstone and a building whose Action jams; `always_liberate=1` tells a bad liberate price from an agent that won't play the card.

Any agent can use horizon mode (`greedy:w.horizon=1,...`); `engine` only changes the
defaults. Agent sweeps with strategy health: `scripts/tune/engine_sweep.py`, logging to
`out/tune/engine.jsonl`; `AIC_SIM` names another binary (an absolute Windows path;
build one with `CARGO_TARGET_DIR=target/alt` to sweep while editing).

Spec syntax: `kind[:key=value,...]`, e.g. `greedy:w.corrupted=40`, played by
`aic-sim run ... --agent SPEC` or an experiment.

## The value table

`configs/agents/values.json`, read by `agents/value.rs`: one value in credits for each
resource, tag left in play, token and op; the table is the rates' one home, and
card-pricing.md keeps the method and the rates known to be off. `value.rs` matches every op
with no catch-all arm, so a new op doesn't compile without its arm, and `every_op_has_a_row`
fails until its row is in the table. A card is worth its ops: what its effects do now, what
its standing effects pay a round times the rounds it counts for, a use of its Action times
`uses_share` of those rounds (a progressive token card's spend over the uses its cycle of
adds takes), and its tags. In a game those rounds are the rounds left; at the table's typical
context (the opening's strategy choice, the effect ledger) they are its tier's horizon,
`horizon_t1` 4 and `horizon_t2` 2.5 (r35: the rounds left at a typical purchase less a tempo
discount).

Where the heuristic reads it in table mode (`w.table`; the `engine`'s default, off for `greedy`):

- **A card in hand:** `hand_share` of its net worth (value less play cost, times the chance
  of paying and the progress term); under 1, or holding a card would be as good as playing it.
- **A card to draft or keep:** its value times the strategy multiplier, less `pick_cost` of its
  play cost, less its keep times `packet_keep` in the packet; times `hand_share`, then `drafted`. `pick_cost` is 0 because the keep
  plan judges what a seat can pay: at 1 a tier-2 card, priced at about its value less its
  keep, reads as worth no more than its keep, and the engine drafts tier 1 only.
- **A building in play:** what it earns a round beyond income, times the rounds left.
- **A token:** what its card's spend pays per token, else the held rate.
- **A supply ambassador:** its row.
- **An action** (a card, an Action, a power) is credited what it does
  that the board terms can't see afterward: a shelter, a cordon, a peek (`Eval::bonus`).

## Strategies

`configs/agents/strategies.json`, read by `agents/strategy.rs`. Each seat commits at its
round-1 keep, from its board and its opening 10, to the strategy of its board's menu whose
multipliers add most to what those cards are worth (`strategy=1`; `strat0`, `strat1`,
`strat2` fix a seat's, 1-based in its board's menu). A strategy is a weight vector over a
card's tags, the resources and tokens it pays in or out, its kind and its categories; a
card's multiplier is 1 plus the weights it matches, times the **phase schedule** for its
categories: early (rounds 1-3) income ×1.4 and engine pieces ×1.3; mid (4-6) chain pieces
(a card with `needs`) ×1.4; late (7 on) capstones ×1.4, liberation ×1.5, purge ×1.4 (cuts too: `strategies.json`); a card takes its categories' largest. The
multiplier reaches what a seat drafts and keeps, not a card in hand: the board terms playing
brings aren't multiplied, so a multiplied hand would beat playing it. The choice is a pure
function of the table at that keep, and the planner hands it to every rollout's policy.

The strategies as a player says them (the config's `says`; a test requires each line verbatim):

**The Builder**

- *Industry income.* Play Industry income early (Payroll Automation, Robotic Foundry), and stack Industry tags toward Arcology Foundry and Vertical Monopoly; spend two spent Industry cards on Rush Build.
- *Power grid.* Take Power cards and energy income (Baseload Contract, Substation Hub), climb the Power tags to Dyson Swarm and the Supergrid, and cash spent Power cards with Overcharge; Power to the People frees a city at the end.
- *Capital finance.* Build Capital piles (Retained Earnings, Expansion Fund, Sovereign Capital Fund), cash them when they are big, and save the credits for Hostile Buyout.
- *Wide build.* Fill your five slots with Action buildings and use one each round, and push the card-play track to Master Plan's discount.

**The Protector**

- *Troop wall.* Stand your nodes in the troops' way, take the troop removals (Interceptor Battery, Kill Team Roster) and finish with Total Containment or Kinetic Interdiction.
- *Counter desk.* Answer threats with the typed jams and counters, build Sentinel Command and Security Contractor HQ, and send Peacekeeping Mission to a corrupted city.
- *Diplomacy and liberation.* Take Diplomacy and Followers (Liaison Corps, Open Assembly, Accord's node), and free cities with Great Liberation and Popular Uprising.
- *Civic shelter.* Keep what you have standing: Bulwark, Hardened Bunker Complex, Safehouse Network and the Shelter power, toward Aegis Doctrine.

**The Researcher**

- *Cubes and stations.* Build stations early (Field Station), Grant cubes onto megaprojects, Peer Review your nodes, and spend the cubes on Forensic Audit.
- *Science chain.* Stack Science tags: Quantum Decryptor and Citation Engine first, then Breakthrough Lab and Foundation Model Campus.
- *Patience burst.* Save your insight under Patience and spend it in a big round on draws (Fieldwork, Simulation Cluster, Orbital Observatory Array).
- *Datasets.* Grow Dataset piles (Telemetry Archive, Synthetic Data Forge) toward Exabyte Archive, and cash them for insight and cards.
- *Team support.* Feed your partner: gifts, Open Science's insight, Policy Institute's Sway, Audit against projects, and Root Access for a corrupted city.

## The progress term and the draft

- **`needs`:** a card whose `needs` aren't met counts `needs_floor` (0.25) of its worth,
  plus the rest times the square of the share of the tags the seat has toward them, so a
  chain card, a dear capstone or a gated liberate card has value before it is playable, and
  each tag played toward it adds more than the last.
- **Dear powers:** a board power that sacrifices two tags or more counts `power_progress`
  (0.5) of one use's worth, times the square of the share of the tags held toward it.
- **Drafting for the neighbor:** a draft hand a seat has taken from goes to the next seat, so
  its cards count for that seat, by that seat's strategy. A card not kept is discarded, so
  there is no keeping for the neighbor.

## The tiers

| Tier | Stands for | Spec | d2 win %, bp / br / pr (seeds 30700) |
|---|---|---|---|
| Greedy without strategy | A new player | `engine` | 68.8 / 85.4 / 72.9; at d0 79 / 98 / 83 (target 70-85), at d4 15 / 23 / 25 |
| Greedy with strategy | A moderately skilled player; **the screens' judge** | `strategist` | 75.0 / 75.0 / 77.1; on fresh seeds 30800 54 / 73 / 79 |
| Planner with strategy rollouts | A strong player; spot checks of the ceiling | `planner:strategy=1` | 79.2 / 87.5 / 91.7 |

The strategist judges (Matt, 2026-11-03: while within about 5 points of `sweep` a pair on the
same seeds) at about an eleventh of its time a game; `sweep`, 2-8 points above, is the spot check. A strategy pairing's
win rate is the balance read: `strategist:strat0=I,strat1=J` fixes them, and `aic-sim
strategies CONFIG --agent strategist` prints what each seat chooses (seeds 30700: never
Capital finance or Datasets, Cubes and stations 2 of 96 Researcher seats). All tables:
`experiments/results/screen-082/agents.md`.

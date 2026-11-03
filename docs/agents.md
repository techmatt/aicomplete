# Agents

The skill ladder (`crates/aic-sim/src/agents/`). One team agent drives every seat, with
hands and drafts open, and answers the "players choose" questions too. Agents play
through the public API only (`legal_actions_into`, `apply_mut`, `apply_to`, reading the state).

- **No peeking.** Before any search an agent replaces what the printed game hides with
  a guess from its own RNG (`agents/determinize.rs`): the player-card and threat
  decks are shuffled (contents are public, order is not); the Unity and Discord piles are dealt again
  from their tier's unresolved printed cards (a game deals a subset face down, 053); each hidden
  adversary-deck position gets an unseen printed card of its stage (stages are stacked and
  public; next round's card is face up, and the top cards stay put under Recon Sweep);
  every RNG stream is reseeded. The guess depends only
  on the public table and the agent's RNG; a test changes only hidden information and
  checks that decisions stay the same.
- **Own randomness.** Each agent's RNG is the game seed on a stream named after the agent
  kind, apart from every engine stream. Games replay from seed + actions, and results
  don't depend on `--threads`.
- **Decides at the top level:** outside the action phase (`searched_phase` in `planner.rs`) the
  planner follows its line, so draft and keep decisions are the rollout policy's, not the
  shortlist's. A prompt inside the action phase is shortlisted like any action there. A presence power
  is scored at `choices` × 2 at the top level, 2 more when a sacrifice's prompts come first, and at
  `choices`, at least 3, inside rollouts (`search.rs`: `PLACING_PROMPTS`, `SACRIFICE_PROMPTS`).
- **`choices` counts prompts, not effects.** One placement is two prompts (a track, then a hex;
  a prompt with one option doesn't count), so `choices` 2 scores one placement. Only
  a power that places gets more: a card that asks more (Relay Mast Rollout, Aegis Grid Lockdown,
  Beamed Power Constellation, Open-Weights Release) is judged part of the way, and beam's
  search doesn't double at all (only its greedy fallback does).
- **Heuristic** (`agents/eval.rs`, 014 board): win/loss; corrupted cities (`corrupted`
  30), a seat down to its last presence (`presence_danger` 60; 0.5 of that with two, 0.25 with three, 0.1 with four), aligned cities
  (`aligned` 8); adversary ambassadors (`adv_ambassador` 3 each, `on_piece` 2 more on a
  player piece, base or factory, `on_factory` 6 more on a factory), the pressure on cities
  not yet corrupted (`pressure` 0.7 × `corrupted` × (ambassadors and markers / threshold)², the
  threshold being `corrupt_at` as the face-up card lowers it), troops (`troop` 12); the
  Discord deck (`discord_danger` 40 per card short of 3 left: the empty tip loses) and the Unity deck (`unity_deck` 40 the same way: the last card wins, 028); the bar's blockers
  (`blocker` 2); Alignment (Unity and Discord tips made, each bucket's fill as a share of
  its next tip, the bar level, and each visible adversary project's fill as a share of
  its cost, `project` 30, 017); the players' board (presence
  4, nodes −25 (022, 020's D9: was −6, and agents made nodes when an equal free choice existed), owned factories 4 (an adversary-owned one −4), chips 2, cities/bases/stations in reach 0.5, owned stations 5, a seat's third on `station_spare` 1 (074), and in horizon mode `station_round` 1 a round left for a seat's first two (a corrupted one −5),
  megaproject cubes 2 (adversary's −1, neutral 0), research cubes held `research_cube` 0.3 (074: low on purpose, under what every converter nets; 075: `research_cube_power` 1.2 for a seat whose board spends cubes on a power, the Researcher's Peer Review, or no card that spends cubes was ever played), insight a Patience seat holds `patience` 1.0 (075: it carries over, so it keeps its worth at the round's end; others' `insight` 0.5), a player facility on a hex with the Protector's presence `wall` 3, twice that with a troop within 4 hexes (075: no troop can strike it), factories 0.5, hardening 1, a track's progress toward its capstone (`track_capstone` 16 × the square of the share uncovered, 068; `capstone` before 080, still read), player ambassadors
  5 in supply (018: was 3, below their 6-credit cost, so agents never bought one; 082, table mode: the table's row, 4 credits) / 2.5 on the board, and toward liberating a corrupted city by the fallback `freeing` 2 ×
  `corrupted` × a share of `liberate_at`: gathered ambassadors, then, once enough are at hand, a climbing bonus for landing them, 027; 080: only seats in reach count),
  each city liberated this game `liberated` 10 (080; with `w.table` on, 082, folded into the corrupted city it frees), a clean or aligned city one adversary ambassador from corrupting `at_risk` 4 on top of
  its `pressure` (080), corruption markers (`marker` 1 each, against), shields (`shield` 3); track income, resources, cards in hand and
  built; unpaid threats (one that waits counts `threat_waits` 0.5, one that can't be reached `threat_stranded` 1.25). Every weight is a parameter: `w.<name>=V` in an agent spec. The
  014 weights are hand-set, not swept.

| Spec | What it does | Budget parameters (defaults) |
|---|---|---|
| `random` | Uniform over legal actions. | none |
| `greedy` | One action at a time: each scored by the heuristic after it, its own choices resolved best-first. A pass scores the table as it is (else stalling beats stopping). | `samples` 1 (guesses averaged), `depth` 1 (actions looked ahead), `choices` 2 |
| `beam` | Beam search over the rest of the round's action sequences, scored at the start of the next round (the end-of-round preview); plans once, follows the plan while the table matches, replans when a draw or reveal differs from its guess. | `width` 6, `depth` 120 (steps), `branch` 0 (children per sequence, 0 = all), `choices` 2 (levels of an action's own choices resolved before scoring it; with 1, placing presence, which asks a hex, looks worthless and beam stops growing its engine) |
| `rollout` | Across rounds: the beam's `candidates` best end-of-round positions, each continued by `rollouts` greedy playouts for `horizon` more rounds on fresh guesses; best mean wins. | `width` 4, `depth` 120, `branch` 0, `choices` 2, `candidates` 3, `rollouts` 4, `horizon` 2 |
| `engine` | The heuristic in **horizon mode** (008): income (tracks, income buildings, rewards) is worth `w.horizon` × the resource's weight × the rounds left, and board production (aligned cities' energy, less what adversary ambassadors divert) `w.production` of that (014: 1 won 65% at d0 against 37% with 0 on the checkpoint's map; level within noise at d1–d3 on the map since edited); the rounds left are the adversary deck's clock (deck plus the face-up card), cut short by when the team is on course to win (Unity Sway still needed for the Unity deck's last card at its rate so far, `w.unity_rate_start`, `w.win_lag`) and discounted by danger (`w.risk` per unit: unpaid threats per country, plus 0.5 with 1 Discord card or none left, plus 0.5 with a seat on its last presence); a building without income earns `w.ability` per round left, and a program in play a flat `w.program`; a card dealt into the draft packet counts as drafted less `w.packet_keep` × its keep price (023), and drafted and packet cards count only as far as the round's insight keeps them, best first (`w.packet_budget` 1, 046); a card in hand counts by the chance it can be paid for before the end (credits, antimatter and research cubes, from what the seat has and earns, factory credits included); since 082 the cards, buildings, tokens and supply ambassadors are valued by the value table (*The value table*, below; `w.table=0` is the pre-082 terms); resources fade when fewer than `w.spend_horizon` rounds are left; and the **deck clock** (011, `w.clock`): a penalty per round the team, at its Unity rate so far, would still be short of the Unity deck's last card (`alignment.unity_cards`) when the deck runs out. Searched like greedy by default; `width` > 0 makes it a beam over the round, `candidates` > 0 adds rollouts. | `width` 0, `samples` 1, `choices` 2; beam: `depth` 120, `branch` 0, `candidates` 0, `rollouts` 2, `horizon` 1. Weights: `horizon` 1, `table` 0.6 (082), `hand_share` 0.5, `pick_cost` 0, `needs_floor` 0.25, `power_progress` 0.5, `built` 1, `built_cost` 0, `risk` 0.1, `spend_horizon` 2, `unity_rate_start` 2 (`opp_rate` still accepted), `win_lag` 1, `production` 1, `clock` 30, `packet_keep` 0.5 (023: 1; 082: half, see *The value table*), `packet_budget` 1 (046); with `table` 0, the pre-082 `hand` 1, `hand_cost` 0.2, `ability` 0.5, `token` 0.5, `ambassador` 5, `liberated` 10, `liberate_progress` 20 |
| `strategist` | 082: the engine with strategies (`engine:strategy=1`): greedy with strategy, no search. Each seat commits at its round-1 keep to one of its board's strategies (*Strategies*, below). **The screens' judge since 082** (*The tiers*, below). | the engine's |
| `planner` | 028 (`agents/planner.rs`): the engine, with a shortlist of alternatives in the action phase each rolled out to the first decision of round `horizon` rounds on, with the engine as the policy, and the best line by the heuristic taken. A line runs on the real state, the Alignment deal aside (**full information**, Matt, 2026-10-29), its value the mean over `deals` guesses of the deal (4 by default since 064, 054: played once to its first tip, then forked; an alternative must lead by `lead` on the first guess, and only then does it and the line it challenges play the others, then lead by `lead` on the mean); each decision inside it is still scored on a guess, because scored on open decks the one-step heuristic wins 19% against 59% (it drafts by printed cost). `redeal_alignment` runs inside `determinize`, so for every caller (Greedy decisions, Beam included), and `hide_alignment_deal` is that call alone on a line's table. It also reorders every pile it redeals: at d2 the Discord deck is fully dealt, so its contents stand but a line sees a guessed order. Draft and keeps follow the line (*Decides at the top level*). **Speed:** the line being followed is reused, so a decision pays only for its alternatives. | `horizon` 2, `alts` 2 a decision, `budget` 8 a round, `margin` 40 (only actions this close to the engine's best), `choices` 2, `samples` 1, `prune` 12 (actions each seat keeps through a phase of a rollout, after the phase's first full scoring), `powers` 6 (with `prune`, powers a seat keeps through that phase, 069: off at 6, a board's count), `answers` 99 (prompts an answer looks on inside a rollout, at most `choices`, 069), `deals` 4, `lead` 0. Its keep and dominance parameters are the engine's: `planner:never_tag=2` holds the check in its rollouts too. `--quick` keeps it. `sweep[:...]` is the planner at `deals=1,lead=0`, for price sweeps (069). |

**Keeping by plan and the dominance checks (022, `agents/policy.rs`).** Greedy and
engine keep drafted cards by a plan (`plan_keep` 1; 0 lets the heuristic decide keeps,
as before 022): at its first keep of the round a seat sketches this round's action phase
and the next three (`plan_rounds` 4; credits, insight, chips, energy and antimatter from what it holds and
its income; insight resets each round, the rest carries over), fills each round's
budget with the cards it would play (hand plus the candidates) in value-per-cost order
(082, `plan_table` 1: value by the value table at the rounds then left, times the seat's
strategy multiplier; cost: what this seat pays at the table's rates, its discounts off;
`plan_table=0` is the pre-082 plan: the printed cost in credits, antimatter 3, insight 2, a
lowered income point 3, plus 1, and 070's capstone jobs), and keeps each
drafted card, best ratio first, only if the plan with it is worth more than without it
(a play waiting a round is worth 15% less; 046: each marker in reach a card would clear, threat it would delay and presence it would shield adds `plan_niche` 3 to the card's value; 082: a card's `needs` may be met by the tags of the cards the plan plays before it, and a card with no target now may be played from the plan's second round on, at the table's `target_share`, unless it works on a token pile of the seat's own with none standing or planned (`pile_ok`); a card whose `needs` are short now is planned only if it is at most `plan_short` (1) tags short; a building that must go over one of the seat's adds only what it beats the weakest one it could cover by (`plan_cover` 1); 080's `plan_liberate` is folded into the table's liberation row). The dominance checks are the same agents with
one behavior forced: `keep_all=1`, `keep_k=2`, `tier1_only=1`, `tier2_rush=1`,
`no_powers=1`, `no_counters=1`, `no_presence=1`, `never_sacrifice=1`, `ambassador_mass=1`, `free_build_rush=1`, `wall_turtle=1`, `home_only=1`, `capstone_rush=1`, `always_buy_cubes=1` (075: the Researcher's Grant at every chance), `always_liberate=1` (080: draft and keep every liberate card and play one the first round it is legal: the row that tells a bad price from an agent that won't play the card), `never_clear=1` (041 as `never_remove`, renamed 080), `hoard_insight=N` (075: a Patience seat keeps and plays cards only in every N-th round, the opening's free keeps aside), `never_convert=1` (075: no converter, and no power that only turns one resource into another); `ignore_threats=1` also refuses an automatic-counter capstone and a building whose Action jams (075, 073's finding 13) (`docs/principles.md`, *Dominance
checks*).

The keep plan also charges credits paid for keeps (`plan_credit` 3); `plan_hand` is a probe
guard and is off (082 dropped `plan_reserve`, `plan_t2` and `w.insight_wait`, all off by
default). In `agents/eval.rs`, credits beyond `w.credit_cap`
(12) a seat are worth `w.credit_idle` (0.15). (`w.guard`, an adversary ambassador next to a corrupted city, went
with the hold in 080.)

Any agent can use horizon mode (`greedy:w.horizon=1,...`); `engine` only changes the
defaults. Agent sweeps with strategy health (win %, buildings and tier-2 cards played per
game, the top buildings) go through `scripts/tune/engine_sweep.py`; it logs to
`out/tune/engine.jsonl`, and `AIC_SIM` points it at another binary (an absolute Windows
path; build one with `CARGO_TARGET_DIR=target/alt` to sweep while editing). Measured: 48
games at difficulty 4 on 12 threads take about 45 s for greedy and `engine`, 2 min with
`samples=3`, 4 min for `engine:width=4`, and longer with rollouts.

Spec syntax: `kind[:key=value,...]`, for example `beam:width=12`,
`rollout:rollouts=8,horizon=3`, `greedy:w.corrupted=40`. `aic-sim run ... --agent SPEC` plays
any of them; an experiment lists several.

## The value table (082)

`configs/agents/values.json`, read by `agents/value.rs`: one value in credits for each
resource (credit 1, chip 1.5, energy 2, antimatter 9, insight 2.5, research cube 3,
ambassador 4), tag left in play (1, then 0.5), token (held 1; fed 3 a Dataset or Capital,
2.5 a Follower, 2 an Agent) and op, transcribed from the rates `docs/card-pricing.md` listed
at r34 (insight at its 2.5, not the old `INSIGHT_COST` 2); the table is their one home now,
and card-pricing.md keeps the method and the rates known to be off. Each op's row says what
one unit is; `value.rs` matches every op with no catch-all arm, so a new op doesn't compile
without its arm, and `every_op_has_a_row` fails until its row is in the table. A card is
worth its ops: what its effects do now, what its standing effects pay a round (income, a
discount, a shelter rule) times the rounds left, a use of its Action times `uses_share`
(0.7) of the rounds left (a progressive token card's spend over the uses its cycle of adds
takes), and its tags. Liberation counts its 25 only while a city is corrupted
(`no_city_share` 0.4 of it otherwise). **Overrides:** none (the table's `overrides` list is
empty; each would carry its reason).

Where the heuristic reads it, in table mode (`w.table` 0.6, a table credit at a credit's
weight; the `engine`'s default since 082, off for the flat `greedy`):

- **A card in hand:** `hand_share` (0.5) of its net worth: its value less its play cost at
  the table's rates, times the chance of paying for it and the progress term. Under 1,
  because playing it brings the board terms, which see about its whole value: at 1 holding
  a card was as good as playing it.
- **A card to draft or keep** (drafted, or in the packet): its value times the seat's
  strategy multiplier, less `pick_cost` (0) of its play cost, less its keep (the table's 2.5
  and 6.5) times `packet_keep` in the packet. The keep plan judges what a seat can pay; with
  the play cost charged (`w.pick_cost=1`), a tier-2 card, priced at about its value less its
  keep, read as worth no more than its keep, and the engine drafted tier 1 only (8-17% a
  pair on the probe seeds, against 71-79% before 082; `experiments/screen-082-probe2.json`).
- **A building in play:** what it earns a round that income doesn't count (its standing
  rules, a share of its Action), times the rounds left: replaces the flat `w.ability`.
- **A token:** what its card's spend pays per token (Popular Uprising's: a city over six
  Followers), else the held rate: replaces `w.token` and `w.liberate_progress`.
- **A supply ambassador:** the table's row (`w.table_ambassador` 1 scales it, a probe).
- **An action** (a card played, an Action used, a board power) is credited what it does
  that the board terms can't see afterward: a shelter, a cordon, a peek (`Eval::bonus`).
- **The keep plan:** values by the table at the rounds then left (`plan_table`).

## Strategies (082)

`configs/agents/strategies.json`, read by `agents/strategy.rs`. Each seat commits at its
round-1 keep, from its board and its opening 10, to the strategy of its board's menu whose
multipliers add most to what those cards are worth (`strategy=1`; `strat0`, `strat1`,
`strat2` fix a seat's, 1-based in its board's menu). A strategy is a weight vector over a
card's tags, the resources and tokens it pays in or out, its kind and its categories; a
card's multiplier is 1 plus the weights it matches, times the **phase schedule** for its
categories: early (rounds 1-3) income ×1.4 and engine pieces ×1.3; mid (4-6) chain pieces
(a card with `needs`) ×1.4; late (7 on) capstones ×1.4, liberation ×1.5, purge ×1.4. The
multiplier reaches what a seat drafts and keeps (the packet and drafted terms, the keep
plan), not a card in hand: what playing brings is the board's terms, which no strategy
multiplies, so a multiplied hand made holding a strategy's cards beat playing them. The
choice is a pure function of the table at that keep, so a game replays, and the planner
hands it to every rollout's policy (`Planner::policy`).

The strategies, as a player would say them (the config's `says`; a test checks they match):

**The Builder**

- *Industry income.* Play Industry income early (Payroll Automation, Robotic Foundry), and stack Industry tags toward Arcology Foundry and Vertical Monopoly; spend two spent Industry cards on Rush Build.
- *Power grid.* Take Power cards and energy income (Baseload Contract, Substation Hub), climb the Power tags to Dyson Swarm and the Supergrid, and cash spent Power cards with Overcharge; Power to the People frees a city at the end.
- *Capital finance.* Build Capital piles (Retained Earnings, Expansion Fund, Sovereign Capital Fund), cash them when they are big, and save the credits for Hostile Buyout.
- *Wide build.* Fill your five slots with Action buildings and use one each round, and push the card-play track to Master Plan's discount.

**The Protector**

- *Troop wall.* Stand your presence in the troops' way, take the troop removals (Interceptor Battery, Kill Team Roster) and finish with Total Containment or Kinetic Interdiction.
- *Counter desk.* Answer threats with the typed jams and counters, build Sentinel Command and Security Contractor HQ, and send Peacekeeping Mission to a corrupted city.
- *Diplomacy and liberation.* Take Diplomacy and Followers (Liaison Corps, Open Assembly, Accord's presence), and free cities with Great Liberation and Popular Uprising.
- *Civic shelter.* Keep what you have standing: Bulwark, Hardened Bunker Complex, Safehouse Network and the Shelter power, toward Aegis Doctrine.

**The Researcher**

- *Cubes and stations.* Build stations early (Field Station), Grant cubes onto megaprojects, Peer Review your presence, and spend the cubes on Forensic Audit.
- *Science chain.* Stack Science tags: Quantum Decryptor and Citation Engine first, then Breakthrough Lab and Foundation Model Campus.
- *Patience burst.* Save your insight under Patience and spend it in a big round on draws (Fieldwork, Simulation Cluster, Orbital Observatory Array).
- *Datasets.* Grow Dataset piles (Telemetry Archive, Synthetic Data Forge) toward Exabyte Archive, and cash them for insight and cards.
- *Team support.* Feed your partner: gifts, Open Science's insight, Policy Institute's Sway, Audit against projects, and Root Access for a corrupted city.

## The progress term and the draft (082)

- **`needs`:** a card whose `needs` aren't met counts `needs_floor` (0.25) of its worth,
  plus the rest times the square of the share of the tags the seat has toward them, so a
  chain card, a dear capstone or a liberate card behind a tag gate has value before it is
  playable, and each tag played toward it adds more than the last.
- **Dear powers:** a board power that sacrifices two tags or more counts `power_progress`
  (0.5) of one use's worth, times the square of the share of the tags the seat holds toward
  it; using it is credited what it does (a shelter is unseen by the board terms, so the
  action is credited it).
- **Drafting for the neighbor:** a draft hand a seat has taken its card from goes to the next
  seat, so its cards count for that seat, by that seat's strategy. No card passes hand to
  hand outside the draft, and a card not kept is discarded, so there is no keeping for the
  neighbor.

## The tiers (082)

| Tier | Stands for | Spec | d2 win %, bp / br / pr (seeds 30700) |
|---|---|---|---|
| Greedy without strategy | A new player | `engine` | 68.8 / 85.4 / 72.9; at d0 79 / 98 / 83 (target 70-85), at d4 15 / 23 / 25 |
| Greedy with strategy | A moderately skilled player; **the screens' judge** | `strategist` | 75.0 / 75.0 / 77.1; on fresh seeds 30800 54 / 73 / 79 |
| Planner with strategy rollouts | A strong player; spot checks of the ceiling | `planner:strategy=1` | 79.2 / 87.5 / 91.7 |

The judge rule (Matt, 2026-11-03): the strategist replaces `sweep` for screens if it reads within about 5 points of it a
pair on the same seeds. Against the sweep before 082 (the judge through 080) it read −2.1 / +6.3 / +4.2 over 96 seeds a pair,
about 11 × faster a game, so it is the judge. `sweep` itself now plays on the table and reads stronger (the strategist
sits −2.1 ±4.7 and −8.3 ±4.5 pooled under it on the two seed sets): it stays the spot check. A strategy pairing's win rate is the
balance read: `strategist:strat0=I,strat1=J` fixes them, and `aic-sim strategies CONFIG --agent strategist` prints what
each seat chooses (seeds 30700: never Capital finance or Datasets, Cubes and stations 2 of 96 Researcher seats; the
choice reads only the opening's cards). Every table: `experiments/results/screen-082/agents.md`.

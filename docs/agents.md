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
  megaproject cubes 2 (adversary's −1, neutral 0), research cubes held `research_cube` 0.3 (074: low on purpose, under what every converter nets), factories 0.5, hardening 1, a track's progress toward its capstone (`capstone` 16 × the square of the share uncovered, 068), player ambassadors
  5 in supply (018: was 3, below their 6-credit cost, so agents never bought one) / 2.5 on the board, and toward freeing a corrupted city `freeing` 2 ×
  `corrupted` × a share of the hold: gathered ambassadors, then, once enough are at hand, a climbing bonus for landing them, 027), corruption markers (`marker` 1 each, against), shields (`shield` 3); track income, resources, cards in hand and
  built; unpaid threats (one that waits counts `threat_waits` 0.5, one that can't be reached `threat_stranded` 1.25). Every weight is a parameter: `w.<name>=V` in an agent spec. The
  014 weights are hand-set, not swept.

| Spec | What it does | Budget parameters (defaults) |
|---|---|---|
| `random` | Uniform over legal actions. | none |
| `greedy` | One action at a time: each scored by the heuristic after it, its own choices resolved best-first. A pass scores the table as it is (else stalling beats stopping). | `samples` 1 (guesses averaged), `depth` 1 (actions looked ahead), `choices` 2 |
| `beam` | Beam search over the rest of the round's action sequences, scored at the start of the next round (the end-of-round preview); plans once, follows the plan while the table matches, replans when a draw or reveal differs from its guess. | `width` 6, `depth` 120 (steps), `branch` 0 (children per sequence, 0 = all), `choices` 2 (levels of an action's own choices resolved before scoring it; with 1, placing presence, which asks a hex, looks worthless and beam stops growing its engine) |
| `rollout` | Across rounds: the beam's `candidates` best end-of-round positions, each continued by `rollouts` greedy playouts for `horizon` more rounds on fresh guesses; best mean wins. | `width` 4, `depth` 120, `branch` 0, `choices` 2, `candidates` 3, `rollouts` 4, `horizon` 2 |
| `engine` | The heuristic in **horizon mode** (008): income (tracks, income buildings, rewards) is worth `w.horizon` × the resource's weight × the rounds left, and board production (aligned cities' energy, less what adversary ambassadors divert) `w.production` of that (014: 1 won 65% at d0 against 37% with 0 on the checkpoint's map; level within noise at d1–d3 on the map since edited); the rounds left are the adversary deck's clock (deck plus the face-up card), cut short by when the team is on course to win (Unity Sway still needed for the Unity deck's last card at its rate so far, `w.unity_rate_start`, `w.win_lag`) and discounted by danger (`w.risk` per unit: unpaid threats per country, plus 0.5 with 1 Discord card or none left, plus 0.5 with a seat on its last presence); a building without income earns `w.ability` per round left, and a program in play a flat `w.program`; a card dealt into the draft packet counts as drafted less `w.packet_keep` × its keep price (023), and drafted and packet cards count only as far as the round's insight keeps them, best first (`w.packet_budget` 1, 046); a card in hand counts by the chance it can be paid for before the end (credits and antimatter, from what the seat has and earns, factory credits included; `w.insight_wait` 1 also queues the hand for insight, but as written it made playing any card look costly and cut wins to 4%, so it is off); resources fade when fewer than `w.spend_horizon` rounds are left; and the **deck clock** (011, `w.clock`): a penalty per round the team, at its Unity rate so far, would still be short of the Unity deck's last card (`alignment.unity_cards`) when the deck runs out. Searched like greedy by default; `width` > 0 makes it a beam over the round, `candidates` > 0 adds rollouts. | `width` 0, `samples` 1, `choices` 2; beam: `depth` 120, `branch` 0, `candidates` 0, `rollouts` 2, `horizon` 1. Weights: `horizon` 1, `hand` 1, `hand_cost` 0.2, `built` 1, `built_cost` 0, `risk` 0.1, `ability` 0.5, `spend_horizon` 2, `unity_rate_start` 2 (`opp_rate` still accepted), `win_lag` 1, `production` 1, `insight_wait` 0, `clock` 30, `packet_keep` 1 (023), `packet_budget` 1 (046) |
| `planner` | 028 (`agents/planner.rs`): the engine, with a shortlist of alternatives in the action phase each rolled out to the first decision of round `horizon` rounds on, with the engine as the policy, and the best line by the heuristic taken. A line runs on the real state, the Alignment deal aside (**full information**, Matt, 2026-10-29), its value the mean over `deals` guesses of the deal (4 by default since 064, 054: played once to its first tip, then forked; an alternative must lead by `lead` on the first guess, and only then does it and the line it challenges play the others, then lead by `lead` on the mean); each decision inside it is still scored on a guess, because scored on open decks the one-step heuristic wins 19% against 59% (it drafts by printed cost). `redeal_alignment` runs inside `determinize`, so for every caller (Greedy decisions, Beam included), and `hide_alignment_deal` is that call alone on a line's table. It also reorders every pile it redeals: at d2 the Discord deck is fully dealt, so its contents stand but a line sees a guessed order. Draft and keeps follow the line (*Decides at the top level*). **Speed:** the line being followed is reused, so a decision pays only for its alternatives. | `horizon` 2, `alts` 2 a decision, `budget` 8 a round, `margin` 40 (only actions this close to the engine's best), `choices` 2, `samples` 1, `prune` 12 (actions each seat keeps through a phase of a rollout, after the phase's first full scoring), `powers` 6 (with `prune`, powers a seat keeps through that phase, 069: off at 6, a board's count), `answers` 99 (prompts an answer looks on inside a rollout, at most `choices`, 069), `deals` 4, `lead` 0. Its keep and dominance parameters are the engine's: `planner:never_tag=2` holds the check in its rollouts too. `--quick` keeps it. `sweep[:...]` is the planner at `deals=1,lead=0`, for price sweeps (069). |

**Keeping by plan and the dominance checks (022, `agents/policy.rs`).** Greedy and
engine keep drafted cards by a plan (`plan_keep` 1; 0 lets the heuristic decide keeps,
as before 022): at its first keep of the round a seat sketches this round's action phase
and the next three (`plan_rounds` 4; credits, insight, chips, energy and antimatter from what it holds and
its income; insight resets each round, the rest carries over), fills each round's
budget with the cards it would play (hand plus the candidates) in value-per-cost order
(value: the card's printed cost in credits, antimatter 3, insight 2, a lowered income
point 3, plus 1; cost: what this seat pays, its building discount off), and keeps each
drafted card, best ratio first, only if the plan with it is worth more than without it
(a play waiting a round is worth 15% less; 046: each marker in reach a card would clear, threat it would delay and presence it would shield adds `plan_niche` 3 to the card's value). The dominance checks are the same agents with
one behavior forced: `keep_all=1`, `keep_k=2`, `tier1_only=1`, `tier2_rush=1`,
`no_powers=1`, `no_counters=1`, `no_presence=1`, `never_sacrifice=1`, `ambassador_mass=1`, `free_build_rush=1`, `wall_turtle=1`, `home_only=1`, `capstone_rush=1` (`docs/principles.md`, *Dominance
checks*).

The keep plan also charges credits paid for keeps (`plan_credit` 3), skips cards it can't
play now and holds back `plan_reserve` (0) insight in later rounds; `plan_hand` and
`plan_t2` exist for probes and are off. In `agents/eval.rs`, credits beyond `w.credit_cap`
(12) a seat are worth `w.credit_idle` (0.15), and `w.guard` prices an adversary ambassador
next to a corrupted city.

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

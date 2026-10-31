# Design principles

The standards every module and every rules change is judged against.

## Pillars

- **Cooperative, against an escalating adversary** (Spirit Island × Terraforming Mars).
- **A real, printable board game.** Every rule is runnable by hand, and the digital
  version adds nothing a physical copy couldn't have.
- **Easy to model, with a predictable future.** The adversary seems smart but moves few
  tokens, makes no free choices, and its threats are visible ahead.
- **Adaptation over one strong general strategy**, especially at high difficulty.
- **Light, rewarding cooperation.** Helping is optional and limited, costs the helper,
  and nets more than it costs. Engines are never tightly interlinked.
- **Scale "for each country"**, with as few "X per player" rules as possible.
- **Complexity earns its place.**
- **Explainable in prose.** The rules must be describable concisely to new players;
  that is a prior on the design, and mechanics may be reworked to make the rules easier
  to explain.

## Small numbers (Matt, 2026-10-29)

> Keep every currency small. Totals should rarely reach the hundreds; the most expensive
> thing costs on the order of 50, as in Terraforming Mars. Prefer "things produce 1"
> wherever it works, for every resource.

**Current check (`configs/baseline.json`, `content/`):** the average cost of what is
played should sit near credit income ÷ insight income: about 11 credits ÷ 5 insight ≈ 2,
and tier-1 cards average about 3 credits with tier 2 about 9.

- **Credits are the outlier.** 20 to start, 10 / 9 a round from each AI board plus the
  Builder's Foundry Core +1, plus the presence track and Unity income (Grassroots Funding,
  New Deal +2 each): a player sees about 120 to 140 credits over an 11-round game before cards,
  so totals do reach the hundreds. Card costs run 0–25 (Orbital Mass Driver 25, Arcology
  Foundry and Dyson Swarm Tranche 16): within "about 50", but the income is not "produce
  1".
- **Credit sinks priced off that income:** threat counters 4–8; a megaproject cube pays
  `project_credits` 10; a player ambassador costs `ambassador_credits` 6; chips sell
  for `chips_credits` 1; Market Crash takes 8 and Propaganda 4–8.
- **Adversary compute:** project costs are 5–10 *a player*, so 15–30 at 3 players and
  20–40 at 4: the top of the range, still under 50. A node makes 1.
- Within the principle: energy (cities make 1, presence costs 1), chips (a factory
  makes 1), insight, antimatter, Sway (tips at 2 a player), the supply (24 a country,
  pieces rather than a currency).

## Difficulty targets

- **Difficulty 0:** random play almost never wins; moderately skilled play essentially
  always wins.
- **Top difficulty:** the strongest agent wins consistently, but narrowly.
- **In between:** win rate falls smoothly with difficulty, a stronger team always does
  better, and the skill gap widens as difficulty rises.

## Design lenses for every module

- strategic: situational, never always right
- modelable without heavy mental simulation
- engine depth
- fun
- table cost: tokens moved, upkeep steps
- scales naturally across 1–4 players
- rewards adaptation
- pulls players together
- readable timing
- printable

## Measures per module, over many simulated games

- win rate by skill tier × difficulty, and the skill gap at each level;
- the effect of removing the module;
- decision closeness (few forced moves);
- pick and play rates (no dead cards, no must-picks);
- build variety among wins, and an adaptation index (a fixed build loses ground against
  some adversaries);
- win probability by round (uncertain into the late game) and margin of victory (mostly
  narrow);
- game length (8–10 rounds) and adversary operations per round (low);
- help and counter usage;
- luck share (most outcome variance comes from decisions, not the shuffle).

Agents get no help the printed game couldn't give players. **Information (Matt,
2026-10-29):** for the foreseeable future agents may play with full information (deck
orders and other hidden state revealed, one agent for the whole team, deterministic): it
is simpler and should give similar balance. Where determinization or hidden-information
handling gets in the way, it may be dropped. The `planner` (028) plays its rollouts on the
real state; each decision inside them still scores on a guess, because scored on open decks
the one-step heuristic wins far less (19% against 59%, c2 d2): it drafts by printed cost.

## Dominance checks (a standing gate; Matt, 2026-10-29)

> Balance should require thought, not an obvious heuristic dominating.

Each check is a fixed, simple policy: the planner with one behaviour forced
(`planner:<check>=1` in an experiment spec; `crates/aic-sim/src/agents/policy.rs`). The
strategy-health gate runs them beside the planner at d2, early-stopped
(`experiments/gate-046.json`, summarised by `scripts/cards/screen.py --gate`).

- **Healthy:** every check loses ground to the planner at d2. Report each check's gap
  (planner win % minus the check's) with its noise.
- **When one doesn't lose:** the check names a rule or price that makes thinking
  unnecessary. Fix it when the fix is a number or a content change; otherwise report
  the structural cause.
- **Early stopping (040):** a spec's `gate` stops a check once its verdict is decided at
  the stated noise (README, *Gate early-stopping*); a check that stays close runs to the cap.
- **The list grows:** a new mechanic adds the obvious fixed policy for it.

| Check (since) | Spec | Policy |
|---|---|---|
| keep-all (022) | `keep_all=1` | keeps every drafted card it can afford |
| keep-cheapest-k (022) | `keep_k=2` | keeps only its k cheapest drafted cards (k = 2) |
| tier-1-only (022) | `tier1_only=1` | never drafts from tier 2 |
| tier-2-rush (022) | `tier2_rush=1` | drafts from tier 2 whenever it can afford the keep |
| no-powers (022) | `no_powers=1` | never uses an AI-board power |
| no-counters (022) | `no_counters=1` | never pays toward a threat's counter |
| no-presence (022) | `no_presence=1` | places no presence beyond setup (no standard action, card or growth that places one) |
| mono-tag (023), one per tag | `only_tag=N` (1-6: Science, Industry, Cyber, Power, Diplomacy, Security) | keeps and plays only cards carrying that tag (AI-unique cards exempt) |
| ignore-X (023), one per tag | `never_tag=N` | never keeps or plays a card carrying that tag |
| event-hoarder (023) | `events_only=1` | keeps and plays only events |
| ignore threats, turtle, free cities only (027) | `ignore_threats=1`, `turtle=1`, `free_only=1` | no counters; stays on the setup hex and defends it; only Diplomacy cards and ambassador moves (turtle and free cities only also pass Clear, which pays Unity, and selling chips) |
| Unity farm (028) | `unity_farm=1` | before anything else, buys ambassadors and lands them on bases and factories, which pay Unity when they produce |
| rich-spend, Science rush (034) | `rich_spend=1`, `science_rush=1` | every spare credit on the rich action; drafts, keeps and plays Science first, for its discounts |
| always-Expand, always-Consolidate, never-Remove, Science board (041) | `always_expand=1`, `always_consolidate=1`, `never_remove=1`, `science_board=1` | the growth option placing the most presence; the one placing none; never clears a corruption marker; the Science rush, board cards first |

022 ran the checks under the engine at d0, d2 and d4
(`experiments/archive/screen-022-dominance.json`, summarised by `scripts/cards/screen_022.py`).
**Since 023 the gate runs at d2 only, 32 seeds a cell** (`experiments/archive/screen-023-dominance.json`
with `--quick --games 32`, summarised by `scripts/cards/screen_023.py`): 022's d4 half
could not pass while d4 is not winnable, and 16 seeds left gaps inside the noise. **Since
028 it runs under the engine and the planner** (`experiments/archive/screen-028-dominance.json`,
summarised by `scripts/cards/screen_028.py --gate`), each check against its own agent: the
rules are judged by play that can plan, and a check is the same fixed rule in both. **The
current gate** is `experiments/gate-046.json`: planner only, d2, early-stopped, every 034
check plus 041's four fixed policies, summarised by `scripts/cards/screen.py --gate`. Its
predecessors are `experiments/archive/gate-034.json` and `experiments/archive/gate-041.json`.
`experiments/gate-exploit.json` (050) is its fast first read: the 12 checks that could still beat
thinking, under the engine, capped at 32 seeds. It gives a direction; the full gate, under the
planner, gives the verdict.

## Sanity checks (028)

Cheap checks that the game rewards what it asks of players. Compute one occasionally, and
only when it is nearly free; these are not a gate on every screen.

- **Drafting should improve an agent's win chances.** Compare an agent against the same
  agent keeping nothing (`keep_none=1`) and keeping at random (`keep_random=1`), at d2. If
  keeping nothing does as well, the draft is decoration.

Weigh each measure's cost against the decision it informs: screen cheaply first, and run expensive measures only on finalists.

Calibrate by reasoning first: before simulating, think through when each decision, card
and number has a place to matter in an 8–10 round game, and set its starting value so
it does; simulation then adjusts and evaluates.

Until the rules are close to final, scoping runs use shallow, fast agents (greedy,
engine, and since 028 the planner, Matt's standing ruling: rollouts before deeper search) and
short runs (`aic-sim experiment --quick`); deep-search agents stay in the code, ready, but
don't get run time until the rules settle.

The agents (random, greedy, beam, rollout, engine, planner) and how each measure is computed are in the
README (*Agents*, *Experiments*); `aic-sim experiment` writes them up as a cycle report.

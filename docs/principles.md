# Design principles

The standards every module and every rules change is judged against.

## Pillars

- **Cooperative, against an escalating adversary** (Spirit Island × Terraforming Mars).
- **A real, printable board game.** Every rule is runnable by hand, and the digital version adds nothing a physical
  copy couldn't have.
- **Easy to model, with a predictable future.** The adversary seems smart but moves few tokens, makes no free
  choices, and its threats are visible ahead.
- **Adaptation over one strong general strategy**, especially at high difficulty.
- **Light, rewarding cooperation.** Helping is optional and limited, costs the helper, and nets more than it costs.
  Engines are never tightly interlinked.
- **Scale "for each country"**, with as few "X per player" rules as possible.
- **Complexity earns its place.**
- **Explainable in prose.** The rules must be describable concisely to new players; that is a prior on the design,
  and mechanics may be reworked to make them easier to explain.

## Small numbers (Matt, 2026-10-29)

> Keep every currency small. Totals should rarely reach the hundreds; the most expensive
> thing costs on the order of 50, as in Terraforming Mars. Prefer "things produce 1"
> wherever it works, for every resource.

**Current check (`configs/baseline.json`, `content/`):** the average cost of what is played should sit near credit
income ÷ insight income (about 25-35 credits ÷ 5-7 insight ≈ 5); tier-1 cards have a median of 11 credits, tier 2 12.

- **Credits are the outlier.** The Builder, Protector and Researcher start with 40 / 46 / 40 (`ai.start_credits`,
  the scaffold below) and print 15 / 13 / 11 a round, plus 2-3 a node on the economy track and Unity
  income (Grassroots Funding, New Deal +2 each): about 200-250 credits a seat over a 9-round game before cards, so
  totals reach the hundreds. Card costs run 0–42 (Moonshot Grant 42, Multiyear Defense Budget 34): within "about
  50", but the income is not "produce 1".
- **Credit sinks, priced off it:** threat counters 12 / 17 credits by tier (the last option on every menu;
  dearer from d1, threats.md); a research cube about 3 credits (payouts are per card); a player ambassador
  `ambassador_credits` (it pays for removals; dear because it stays); a card sells for `sell_credits` 1, chips not
  at all; Data Heist takes 10 credits (or 3 cards) from each player.
- **Adversary compute:** project costs are 7–24 *a board*: 21–72 at 3 countries, 28–96 at 4, so a few a game pass
  50 (as Matt asked). A botnet makes 1.
- Within the principle: energy (cities make 1, a node costs its track space's energy, 1-5 after the free spaces), chips (a factory makes `factory_chips` 1), insight,
  antimatter, Sway (tips at `tip_per_player` 3 a player, Discord's 4), the supply (24 a country, pieces rather than
  a currency).

## Difficulty targets

- **Difficulty 0:** random play almost never wins; moderately skilled play essentially always wins.
- **Top difficulty:** the strongest agent wins consistently, but narrowly.
- **In between:** win rate falls smoothly with difficulty, a stronger team always does better, and the skill gap
  widens as difficulty rises.

**Calibration point (Matt, 2026-10-31): medium difficulty.** At d2 winning takes real skill and a skilled team wins
fairly consistently: the `planner` at 60-75% at 2 and 3 countries, where a gate can tell a good price from a bad
one. Read a band over 96 seeds: at 48 it is ±7, a paired gap ±8-10.

**Ballpark simulation (Matt, 2026-11-03; supersedes the band above until he names a milestone):** two players only,
rotating the three board pairs, no c3; an engine smoke read, then the judge (the strategist) at 48 seeds a pair on
one fresh set; the target is 45-85 a pair, near 60-75 without chasing it; no gate, no pooled second set
([experiments.md](experiments.md), *The ballpark read*).

**Per-level bands (Matt, 2026-11-15: raised, soft):** d0 at least 90, d1 80-95, d2 75-90, d3 55-70 for the judge
(`bench/ladder.py`'s default `--bands`). A miss of a few points is reported, never chased or dialed; a level that
can't meet both keeps its 8-10 rounds over its band. d0 resisted every income dial; its band comes from the
adversary's levers (263): Botnets don't spread at d0 (`adversary.botnet_spread_at`), and a city corrupts at 5 agents
there (3 from d1).

**Cities corrupted and liberated (Matt, 2026-11-03),** a measured target per difficulty: at easy levels a win may
need no liberation (the adversary corrupts no new city); at d2 it corrupts at least one new city in most
games, and a won game liberates about one; at high difficulty expect to liberate at least 2. Read with
`scripts/cards/liberation_read.py`.

**Failure mix (Matt, 2026-10-31).** No clock over about 80% of losses: Discord may be the main cause, and the node
clock stays live. The round cap is a backstop with no share target (locked): a plain loss, its losses examined
(Matt, 2026-11-08).

**Pressure scales per country (Matt, 2026-10-31).** Harm sources count the countries at setup, not a fixed amount
and not the countries still standing, so no loop forms and sizes land in one band.

## Scarcity (Matt, 2026-11-01)

**Every resource stays scarce, credits most of all.** A seat ending with a pile unspent is a balance fault. Fix it
with weaker credit engines, dearer cards or keeps, or lower starting credits; never by cheapening a standard project.

**The starting-economy scaffold (Matt, 2026-11-03).** Until engines carry the economy, every board starts and earns
much more, the economy track the growing income; it comes down as engines take over, adding no track. Credits stay
meaningful through the adversary (its agents rally onto sites each round) and the plain route's price (Gain a
player ambassador at `ambassador_credits` 20): bought ambassadors are a big investment; cards, buildings and powers the better
route.

**Unspent resources:** held as a seat's last action phase ends (the production and upkeep paid after can't be
spent), per seat, resource and agent ([experiments.md](experiments.md)); not insight, which a round loses anyway.
**A resource ends high** when a seat's mean passes about a round's income of it: credits 12, chips 4, energy 4,
antimatter 2, ambassadors 3 (`UNSPENT_HIGH`, `crates/aic-sim/src/experiment.rs`). Games end at different rounds:
read the flag on the planner (its games run longest toward a win) before a fixed policy's.

## Design warnings (Matt, 2026-11-12)

Calibration measures use, not soundness, so `scripts/design_check.py` checks these (when: CLAUDE.md, *Warning
checks*; thresholds, all candidates: `scripts/design_check.json`).

- **Capstone and tier-2 resources (antimatter, and any later) are scarce and earned.** No cheap standard-action
  source; the only standard source is a desperate one (5 research cubes → 1 antimatter, say). *Warns:* over 20% of
  the supply from standard actions or tier-1 cards; any one source over 40% of a resource's supply, or one spend
  over 50% of its use.
- **Boards differ:** no power or effect repeated on every board. *Warns:* one power name or one effect on 2+ boards. Exempt by ruling: Expand, each board's
  base node route (252); exemptions are `repeat_powers.exempt`, listed as information.
- **Standard actions are deliberately poorly priced against cards** (Matt, 2026-11-16: cards are the engine; a
  standard action is a rare fallback). *Warns:* a standard action (each target of Liberate its own row)
  whose get passes its give at the unit anchors ([card-guidelines.md](card-guidelines.md)); **job parity**, its price
  per unit of its job under 1.25 × the median card's for that job, or an outcome stronger than theirs (the site
  yours where cards only free it), or no card for its job (`scripts/design_check_jobs.json`); a **circular anchor**,
  one set from the price of the action it measures. An exemption is per case with a written reason (the city's
  Liberate, the win route), never a whole action.
- **Unity and Discord follow the story:** Unity is trust and cooperation; Discord is fear, force and collateral
  harm. *Warns:* a Sway source whose side disagrees with its story tag (an untagged one is listed, not warned).
- **Fewer tokens, less processing and tracking**, all else equal. *Warns:* tokens placed or moved a round up 10%
  on the previous revision's read.
- **Engines over dials.** A skipped resource means its engines need buffing; at high difficulty victory needs chips,
  energy, antimatter and cubes. *Warns:* a card kept but played under 0.1 a game, one of the four spent in under
  75% of won games at d3-d4; a dial moved 40%+ in the item that changed the design.
- **Every action has an expected use**, a band by its role (a hedge rare but not zero, a board's signature power
  most rounds, a win route most games). *Warns:* `usage`, any power, standard action, converter or resource spend
  outside its band, unused or dominant.
- **Games run 8-10 rounds.** *Warns:* any level's mean outside 8-10.

## Design lenses for every module

Strategic (situational, never always right); modelable without heavy mental simulation; engine depth; fun; table
cost (tokens moved, upkeep steps); scales naturally across 1–4 players; rewards adaptation; pulls players together;
readable timing; printable.

## Measures per module, over many simulated games

- win rate by skill tier × difficulty, and the skill gap at each level;
- the effect of removing the module;
- decision closeness (few forced moves);
- pick and play rates (no dead cards, no must-picks);
- build variety among wins, and an adaptation index (a fixed build loses ground against some adversaries);
- win probability by round (uncertain into the late game) and margin of victory (mostly narrow);
- game length (8–10 rounds) and adversary operations per round (low);
- resources unspent at game end, per seat and resource (*Scarcity*);
- help and counter usage;
- luck share (most outcome variance comes from decisions, not the shuffle).

Agents get no help the printed game couldn't give players. **Information (Matt, 2026-10-29):** for the foreseeable
future agents may play with full information (hidden state such as deck orders revealed, one agent for the team,
deterministic): simpler, and it should balance alike; hidden-information handling that gets in the way may be
dropped. The `planner` rolls out on the real state, each decision inside scored on a guess: on open
decks the one-step heuristic drafts by printed cost and wins far less. **One exception (D20):** no agent, rollouts
included, knows which Alignment cards a game dealt; the planner values a line on a mean of guesses (`deals`,
[agents.md](agents.md)). Only the `oracle` reads the deal, to measure what perfect play could reach; it judges no
balance. **The design target (Matt, 2026-11-16) is a real oracle:** very strong play that can't see the future
(`oracle:worlds=K`, 249: the oracle's search on sampled worlds); the full oracle's wins can rest on deck sight.

## Dominance checks (Matt, 2026-10-29)

> Balance should require thought, not an obvious heuristic dominating.

Each check is a fixed, simple policy: the planner with one behavior forced (`planner:<check>=1` in a spec), one per
`forced_table!` row in `crates/aic-sim/src/agents/policy.rs`. The ballpark ruling suspends the gate (below) until
Matt names a milestone.

- **Healthy:** every check loses ground to the planner at d2. Report each check's gap (planner win % minus the
  check's) with its noise.
- **When one doesn't lose:** the check names a rule or price that makes thinking unnecessary. Fix it when the fix is
  a number or a content change; otherwise report the structural cause.
- **Early stopping:** a spec's `gate` stops a check once its verdict is decided at the stated noise (`gate.rs`); a
  close one runs to the cap.
- **The list grows:** a new mechanic adds the obvious fixed policy for it.

**The gate**, `experiments/gate.json` (its `note`: checks, seeds, stopping), runs the planner, so play that can plan
judges the rules, at d2 (d4 is not winnable), 2 and 3 countries; `screen.py --gate` reads it. `gate-exploit.json`,
under the engine, is a fast first direction, not a verdict; `gate-c3.json` holds the Researcher's checks, at c3.

## Sanity checks

Cheap checks that the game rewards what it asks, run occasionally when nearly free; they gate no screen.

- **Drafting should improve an agent's win chances.** Compare an agent at d2 with itself keeping nothing
  (`keep_none=1`) and keeping at random (`keep_random=1`). If keeping nothing does as well, the draft is decoration.

Weigh each measure's cost against the decision it informs: screen cheaply, run expensive measures only on
finalists. Calibrate by reasoning first: think through when each decision, card and number can matter in an 8–10
round game, and set its starting value so it does; simulation then adjusts and evaluates.

Until the rules are close to final, scoping runs use shallow, fast agents (greedy, engine, and the planner; Matt's
standing ruling: rollouts before deeper search) and short runs (`aic-sim experiment --quick`); deep-search agents
stay in the code, unrun until the rules settle. Agents and measures: [agents.md](agents.md),
[experiments.md](experiments.md).

# 175: rules r74, the adversary side of the rulebook-friction rulings

Built on main 416ceab (r73, with 174's speed work). `RULES_REVISION` 73 → 74, bumped from the constant (the prompt
said 74 too). Matt's rulings of 2026-11-10 on `docs/rulebook-friction.md`, as the prompt quoted them. Message 7994
confirmed my six readings (below). Seeds: fresh 41989-42132 (next free 42133); dev 36800-36831 for screens.

## What r74 changed (engine)

| Ruling | Engine | Where |
|---|---|---|
| 1. Corrupt X, one shape | `plan::converge`: every agent next to a target (a site of the type not yet the adversary's) moves onto it, across borders; one next to several asks seat 0 (`Corrupt cities (at 4): the agent at .. moves onto which?`); agents on a target stay; a firewalled or prohibited target takes none. Then every target holding its number corrupts. New types `coastal_city` and `node`. | `plan.rs`, `adversary.rs` (`corrupt_sites`, `converge_one`) |
| 2. Nodes at 2 | `corrupt_node_at` 2 (no difficulty cut). Natural corruption checks sites, then node hexes. A corrupted node hex loses the node through `destroy_piece`; its agents go to the supply. The node drain is gone (`node_leak`, its insight debt and compute). | `adversary.rs` (`corrupt_node`), `production.rs` |
| 3. No last-node safety | `last_node_safe`, `Hot::LastSafeStage`, `board.last_safe_stage` and six "never the last" floors are gone. Overload's "(never their last)" is gone from its face. `board.start_nodes` (1) is the candidate tool. | `node.rs`, `destroy.rs`, `team.rs`, `new_ops.rs`, `threats/ops.rs` |
| 4. Rally without tiers | `rally_plan`: an even share on every site of the type. The remainder goes one each to the lowest-numbered sites that are open and not the adversary's (then the adversary's if too few). A firewalled or prohibited share stays in the supply. No choice left. | `plan.rs`, `adversary.rs` (`rally`) |
| 5. Deploy | One kind: an event's, a project's and the Protector landing (printed "Deploy 1"). A Deploy prohibition and the d2 surcharge (`deploy_bonus`) apply to each. A Deploy onto a Wall hex stays in the supply (it used to move on to the next factory). | `team.rs` (`troops`), `adversary.rs` (`land_troops`, `wall_pressure`) |
| 6. Compute once a turn | `gain_compute` only banks (`adversary.compute_waiting`; `compute_every` banks apart). `compute_step` runs after the reveal and agent income. It adds income, botnets and corrupted stations (moved out of production and the build) to the bank, then pours it all onto the projects. This is the only place a project resolves. Slots are tracked (`adversary.slot.<id>`) and each refills as it empties. Costs are printed per player count (`docs/rules/adversary.md`, `docs/adversary-cards.md`, the viewer export). | `adversary.rs` |
| 7. Ambassadors don't fight | In `march_troops`, a troop starting a step on a hex with an ambassador removes one (seat 0 chooses whose if several) instead of moving or striking. Armor is kept: it still answers a strike from next door (decision below). | `troops.rs` |

## Decisions

- **Matt's message 7994** (an answer, not an amendment) confirmed all six readings I had asked about:
  1. a firewalled site counts in Rally's split, and its share stays in the supply;
  2. converging crosses borders;
  3. a node hex with 2+ agents loses its node even on a site below that site's number, with site corruption checked first;
  4. agents never converge onto sites the adversary already holds;
  5. the Protector landing is a printed Deploy 1;
  6. start-with-2-nodes is screened against no tool.
- **No last-node tool** (`start_nodes` stays 1). On the dev seeds, start-2 read below no tool at d0 (81 vs 90) and d2 (49 vs 62). Last-node losses are rare without it: 0-8% of losses on the fresh ladder. 177 calibrates the tools.
- **Armor against troops kept.** A troop on the hex now removes ambassadors first, but a strike from next door still meets the armor, so it isn't redundant. 176 rewrites the protection order.
- **The Rally remainder falls back to the adversary's sites** when there are too few others. This is documented; the alternative is to leave the excess in the supply.
- **The harms ledger** now classes "pays N insight or discards" (Data Heist) as insight lost too. Dropping the drain had left that harm in one kind of source, under the check's 2.

## The sim players' new decisions

Each new decision's agent code was reviewed, and then an independent adversarial review of the diff found 8 faults (verified by
a second agent each), all fixed before the fresh read:
- **Converge** (`converge_rule`, on): `moves::converge` answers the converge prompt with `plan::converge_pick`: onto a target corrupting anyway, else the one holding fewest. It reads the number off the prompt, so a project's "(1 fewer)" line is read right.
- **Eval:** node exposure (2 agents, gathered by a Corrupt nodes line or the site's own line); `toward` no longer counts neighbors standing on a same-kind clean site (they don't move); waiting compute is read as if already on the cheapest project.
- **Moves:** a push never lands agents on a node hex, and a node hex is a site at risk.
- **Guide:** the shelter forecast takes a last node, as the engine does.
- **troop_plan:** counts the ambassadors a troop will remove first.
- **Whose ambassador a troop removes** is left to the search (`choose_loss`).

The converge rule on and off (strategist, dev seeds) is flat at every level, before and after the fixes:
[screens.md](screens.md). It is kept on because it is the forecasts' own pick and costs nothing.

## The fresh ladder (r74 against r73, the same seeds)

[fresh-ladder.md](fresh-ladder.md): strategist 432 games a level (3 sets × 3 pairs × 48 seeds), greedy 144 (set a),
d0-d4, paired.

| d | strategist r73 → r74 | gap ± SE | greedy r73 → r74 | gap ± SE | r74 losses (strategist): round cap / Discord deck / last node |
|---|---|---|---|---|---|
| 0 | 94.4 → 89.6 | -4.9 ± 1.7 | 90.3 → 93.8 | +3.5 ± 2.7 | 100 / 0 / 0 % |
| 1 | 73.1 → 72.5 | -0.7 ± 2.8 | 72.9 → 79.2 | +6.2 ± 4.9 | 86 / 13 / 1 % |
| 2 | 59.3 → 53.0 | -6.2 ± 2.7 | 73.6 → 63.2 | -10.4 ± 4.6 | 28 / 67 / 6 % |
| 3 | 38.9 → 35.2 | -3.7 ± 2.8 | 50.7 → 36.8 | -13.9 ± 5.0 | 66 / 26 / 8 % |
| 4 | 31.7 → 40.7 | +9.0 ± 2.7 | 43.1 → 39.6 | -3.5 ± 4.3 | 49 / 43 / 8 % |

Greedy's losses at r74, by level: d0 100/0/0, d1 93/7/0, d2 23/70/8, d3 70/25/4, d4 66/29/6 %. At r73 the last-node
share was 0% at every level for both agents except strategist d4 (1%). Bands are 177's job: nothing here was tuned
toward them. By the standard bands, d1 is met and d0, d2 and d3 are below them. d4 now reads above d3 (40.7 against
35.2), the wrong order, and 177 should look at it first. Measures (`ladder.py --measures`):
- Node losses to corruption rise by 0.2-0.7 a game.
- Compute rises by 3-10%.
- Projects revealed nearly double from d2 (34 against 18 a game): each slot refills alone, so misses reshuffle the deck about 4 times a game. Projects resolved are about the same.
- Agents placed are unchanged.

**Speed:** strategist 1.51 ms a decision against r73's 1.58 and greedy 1.68 against 1.81 (the games' own clock,
both sides at 3 threads on a loaded box: a ratio only); games run 0.3-0.4 rounds longer.

## Checks

- Fast lane green.
- `cargo test-all` and clippy: see the commit.
- `aic-sim docs --check` current.
- `loop_smoke.py --run`: no timeouts, at most 3 plays of one card. It ran after the fresh read, not before it.
- No player card changed, so there is no give/get, `cards:fit` or `play_read`.
- Overload's threat face lost "(never their last)".
- `same_results`: no engine change after the frozen binary.
- Specs: `specs175.py` (screens, fresh), `run175.sh`.
- Readers: `pairs175.py` (paired gaps and ms a decision), `losses175.py` (losses by cause).

## For new players (r74's adversary)

The adversary's card rallies its agents evenly onto every site of the type it names, and deploys troops on each
country's lowest-numbered factory. A corrupt line moves every agent next to a site of its type onto it, and every
such site holding its number corrupts. Two agents on any hex with your node take that node, and nothing protects your
last one but armor, shields and shelters: keep a second node out. All the adversary's compute lands once, at the end
of its turn, so projects only go off then. A troop standing on your ambassador removes it instead of moving.
**What resisted a short summary:** an agent next to two sites asks the players to choose, and Rally's remainder
needs an order (lowest-numbered, not the adversary's, not firewalled).

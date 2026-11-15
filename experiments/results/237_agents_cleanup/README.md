# 237: small agent and tooling fixes left by 229-235 (r107, no rules change)

Base: main 0499631 (236's r107). No `RULES_REVISION` bump: the engine plays the same rules; three agent reads move
the sim players' choices. Files: [smoke.py](smoke.py) (arms paired on cell and seed, and the `cards.unofferable`
tally), [smoke.txt](smoke.txt), [smoke-engine.txt](smoke-engine.txt), [per-decision.txt](per-decision.txt),
[design-check.txt](design-check.txt). Specs: `experiments/farm/s237.json` (greedy and strategist, three pairs,
d0-d3, dev seeds 36800-36831), `s237e.json` (the engine agent, d1-d2, 36800-36815).

## What changed

1. **Alignment `deal_threats {tier}` priced in every country.** The engine deals a `DealThreats` with any field set
   and no `scope: country` in every country (`new_ops.rs`); `value.rs` priced it once unless `each_country`.
   `Op::deals_each_country` is now the rule's one home: the engine, `Op::team_reach` and `value::op_worth` read it.
   Moves Two Fronts tier 2 and tiers 3 of Incursion and Two Fronts from -14 to -28 credits each (per_site 2 sites ×
   -7 × 2 countries), so the mean Discord card in `Eval::tier_worth` drops by 2.0 credits at tier 2 and 4.0 at tier
   3. Tests: `team_ops_price_per_country` and `project_ops_reach_as_declared` now include the alignment deal.
2. **The converge forecast follows the tie rule** (229's pick; r98: an agent next to two or more open targets stays).
   `plan::converge_map` shares `plan::converge`'s move (`goes`); `moves.rs` `tips_last_node`
   and `at_risk` read them. The eval's `toward` and node-line read apply the tie from its token table (a second
   clean site of the kind, or a second node hex, next to the agent's hex); firewalls and prohibitions on the
   second target aren't read there.
3. **D97 (amendment 10591): the shelter read counts a reach Shelter.** `board::sheltered` (country flag or the
   owner's reach Shelter) is the engine's check in `destroy_piece` and the agents' in `moves::last_node_at`;
   `board::reach_sheltered` / `reach_sheltering` are the one read of the reach key, so 238's live-reach change
   lands in one place. `guide::sheltered_saves` credits each seat's reach Shelter with its own pieces on the hexes
   it covers, and a country Shelter no longer counts pieces a reach Shelter keeps anyway.
4. **`TokenKind::label(n)`** in vocab.rs is the one printed name ("operative", "followers", "capital");
   `amount`, `plural` and face.rs's `token` read it. Faces unchanged (`docs --check`).
5. **`cards.unofferable.<id>`**: at game end, each kept card left in a hand with no feasible part
   (`cards::unofferable`, read once by `experiment::finish`). design_check's kept-not-played line names it.
6. **Readers promoted** (amendment 10605): `scripts/bench/resource_use.py` (200's, takes a run folder or a games
   file now; `--limits` adds 189's `limit.*` table, so one script replaces both dated ones) and
   `scripts/bench/projects_read.py` (233's). `gaps.py` stays in 233's folder.
7. Stale docs: agents.md named a converge rule and `plan::converge_pick`, both gone since 223.

## The smoke (dev seeds, 32 a cell, 384 games an agent; arms built in turn on main)

| agent | base | +1 deal_threats | +2 tie rule | +D97 | +1 vs base | +2 vs +1 | D97 vs +2 | all vs base | games moved |
|---|---|---|---|---|---|---|---|---|---|
| greedy | 74.7 | 74.7 | 75.0 | 75.0 | +0.0 ±0.4 | +0.3 ±2.3 | +0.0 ±0.5 | +0.3 ±2.3 | 90% |
| strategist | 70.8 | 71.6 | 70.6 | 71.6 | +0.8 ±0.4 | -1.0 ±2.3 | +1.0 ±0.5 | +0.8 ±2.3 | 91% |

- Pick 1 moves 5% of games (greedy d2-d3, strategist d2-d3), strategist +0.8 ±0.4: the Discord price reaches the
  eval only through `tier_worth`.
- The tie rule moves ~90% of games (the move forecast runs every decision), with no net shift at any level beyond
  noise (greedy d0 -5.2 ±4.0, d3 +6.2 ±5.3; strategist d1 +6.2 ±5.1).
- D97 moves 6% of games (+0.0 / +1.0 ±0.5). Reach Shelters are played 1.37 a game on base, 1.39 with the read
  (`effects.shelter_reach`): the read changes the timing of a few, not how often they are played.
- The engine agent (16 seeds, d1-d2, 96 games): 63.5 → 66.7 (+3.1 ±4.6), 91% of games moved.
- `cards.unofferable` (the last arm, 768 games), per game: Hostile Buyout 0.17, Peacekeeping Mission 0.15, Root Access
  0.13, Nuclear Option 0.10, Field Grant 0.10, Great Liberation 0.08; 39 cards in all. These are kept cards that
  had nothing to act on at the end.

## Speed

`per_decision.py` (16 games, c2 d2, base against this build, [per-decision.txt](per-decision.txt)), ms a decision:
greedy 1.08×, strategist 1.06×, engine 1.05×, with 3-6% more options a decision in these (other) games. The first
build, reading the tie per neighbor, cost 1.06 / 1.10 / 1.16×; the final one reads one site-kind table (`Sites`,
eval.rs) and one converge map (`plan::converge_map`) a position, and replays the smoke's 768 games identically.
`speed.py check` gave no verdict (baseline r103); re-based at r107 on this build (quiet box): greedy 829 ms a game.

## Checks

`design_check.py` on the smoke, the last arm against base ([design-check.txt](design-check.txt)): the same
kept-not-played warnings as base (Dyson Swarm Tranche, Deep State Ledger, neither unofferable); usage 5 against 4,
Protector Patrol 3.08 a seat-game against a 3.00 ceiling (base just inside it on these seeds; 236's fresh read had
it at 3.02). Resource use within a few percent
at every level ([resource-use.md](resource-use.md), `scripts/bench/resource_use.py`).

## Candidates

- None needs a rules change. `cards.unofferable` suggests the agents keep target-hungry tier-2 events (Hostile
  Buyout, Peacekeeping Mission, Root Access, Nuclear Option) without the target in prospect: a keep-plan read of the
  card's target condition is an agents item.

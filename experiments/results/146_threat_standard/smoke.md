# 146: checks on the final build

- **Loop smoke** (`scripts/cards/loop_smoke.py --run`, `use.json`, 16 seeds, d2, engine and strategist): no timeouts,
  no card at the play flag (most plays of one card in a game: 3). Ends 56 Unity deck, 34 Discord deck, 6 round cap of 96.
- **Engine smoke read** (same run, `play_read.py`): kept in over 1% of games and never played: Exascale Training
  Cluster 52% (D57, as before), Checkpoint Network 23%, Cryo-Compute Burn 19% (r60 19%), Autonomous Defense Grid 19%,
  Moonshot Grant 8%, Tidal Barrage 4% (16 seeds; r60's same read had Checkpoint Network and Tidal Barrage never kept).
  `board_read.py --powers`: every power used as at r60 (Shelter 0.2-0.8 a game).
- **The threat-harm heuristic** (`w.threat_harm`, 16-32 dev seeds, d2, strategist, 96 games each): on 65.6 / off 67.7;
  counter rate tier 1 58 / 67%, tier 2 74 / 68%. Kept on: it is what lets an agent read pay against let-through.
- **Speed** (`speed.py check`, then `per_decision.py` r60 against r61 at c2 d2 on one thread): the strategist
  0.68x a decision (0.63x a game), the planner 1.31x a decision (1.72x a game, 20% longer games), greedy 2.44x a
  decision (3.03x a game). Why: a resource harm is now "pays N or discards M" (D62, amendment 6227) and a tie for
  nearest is a choice, so an uncountered threat asks the hit player one or two loss choices that a one-ply agent
  settles by trying each (options a decision 24 to 29). Kept: the strategist, the judge, got faster; the speed baseline
  is updated to r61.
- **Same results**: the final read (37144-37239) replayed on the build after the project-reshuffle RNG fix: all 1,152
  games the same (`bench/same_results.py`).
- **Tests**: `cargo test --workspace`, `cargo test-all`, `cargo clippy --workspace --all-targets` clean; `aic-sim docs
  --check` current; `drafts/adversary/check.py` and `--content` pass (Hall's condition holds at 1 and 4 countries).

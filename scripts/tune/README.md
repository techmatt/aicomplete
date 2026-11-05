# scripts/tune

- `agent_sweep.py` (116): the strategist's parameter sweep. Below.
- `sweep.py`, `log_table.py`, `engine_sweep.py`: older tuning drivers ([docs/experiments.md](../../docs/experiments.md), *Past runs*).

## The agent sweep

How much better than its hand-set values the judge (the strategist, `engine:strategy=1`) could play
under the current rules, and which of its parameters matter. Report only: a swept setting is a spec
string, and the agents' defaults change only when Matt adopts one.

**One command, from the repo root**, after `cargo build --release -p aic-sim`:

```
python scripts/tune/agent_sweep.py --sweep-seed S --confirm-seed T
```

`S` and `T` are fresh blocks from the seed ledger (docs/experiments.md, *Fresh seeds*): the sweep
takes 100 seeds, the confirm 96 (`--sweep-games`, `--confirm-games`). Record both ranges in the
ledger. With no seed flags it uses r45's blocks in `agent_sweep.json`, which a later revision must
not reuse. Output goes to `out/agent-sweep/rNN/` (`--out`): `summary.md` holds the tables,
`log.txt` the progress lines, `cache.jsonl` every game result. Rerunning resumes from the cache,
keyed by agent spec, seeds and the binary's hash, so a new build plays everything again.

**What it plays.** The three board pairs (B+P, B+R, P+R) at d2, two players, every setting on the
same seeds. A setting names every swept parameter in the spec. Before the search it checks that the
hand-set setting plays game for game as plain `strategist`. If it doesn't, the hand-set values in
`agent_sweep.json` are stale: update them to the agent's defaults (`Weights::engine` and
`Weights::default` in `agents/eval.rs`, the keep plan's rows in `agents/policy.rs`).

**The parameters** are in `agent_sweep.json`: each one's spec key, hand-set value and grid of five
points. To sweep another parameter, add a row. Its hand-set value must be on its grid. `w.<name>` is
a heuristic weight; a bare key is a keep-plan parameter.

**Phases** (`--phase all` runs them in order):

1. `sweep`: a coordinate search. Each pass takes the parameters in order. For each one it plays
   every grid point within `--reach` steps of the current best (2 on the first pass, 1 after),
   with the other parameters held. It moves to the best point when the paired gain passes
   `--accept` SE (1.5). Passes repeat until one moves nothing (`--max-passes` 3). `--budget MIN`
   caps the search's time.
2. `profile`: every grid point of every parameter, with the others at the best. This gives each
   parameter's effect around the best. A parameter *matters* when some grid point plays worse than
   the best by more than 2 SE.
3. `confirm`: the best setting, the hand-set strategist and greedy (the reference, unchanged), on
   the fresh confirm seeds and on the sweep seeds. It reports the paired gain, the strategist's lead
   over greedy before and after, and credits left a seat at game end. Read the confirm seeds alone:
   the sweep seeds picked the winner.
4. `read`: the counters that differ most between the best setting and hand-set on the confirm
   games, per game and paired. Turn these into a player's rule by hand.

`--best key=v,...` profiles and confirms a given setting instead of the search's.

**Cost.** One setting is 300 games: 2.5-4 min on 4 threads on a busy box (r45: 1.2-2 games/s). A
pass at `--reach 1` is 20 settings, about an hour. At `--reach 2` (the default) it is 40. The profile
reuses the pass's cells. The confirm is about 12 min. r45's first run (`--reach 1 --max-passes 1`, then
the profile) took about 2 hours: `experiments/results/116_agent_sweep/sweep.md`. The sim runs at
below-normal priority with `--threads` workers (default 6); use fewer while the box is busy.

**Reading it.** At 300 games a setting, one SE of a paired gap is about 2.5 points. A step is taken
past 1.5 SE, but it is the best of many tests on the same seeds, so quote the held-out confirm. r45's
only step read +4.3 on the sweep seeds and +2.1 ±2.6 held out. For the same reason the profile reads
every neighbor of a selected best a few points low. Judge a step against the hand-set setting's
reading in the profile, not against 0, and replay any large profile reading on held-out seeds before
calling a parameter one that matters. At r45, readings of −14 and −11 came back within 1 SE of zero.

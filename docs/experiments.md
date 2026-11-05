# Experiments

```sh
S=target/release/aic-sim
$S experiment experiments/smoke.json --out out/smoke    # a quick check (6 seeds)
$S experiment experiments/ladder-v0.json --games 20 --seed 1000 --threads 6
$S experiment experiments/screen-093.json --quick           # scoping
$S experiment experiments/screen-093.json --quick --resume  # the flags it started with (--out, --games, --seed)
```

## The spec

Configs × variants × difficulties × agents over a seed range, on every core; results never depend on the thread count
and every game is replay-checked. Spec paths resolve against the working directory, so a spec not yet copied into a
second checkout is refused.

| Field | Default | Meaning |
|---|---|---|
| `name`, `note` | | Report title and a line on what it is for. |
| `configs` | | Config files. |
| `variants` | `[{"name": "base", "set": []}]` | Named override sets in `--set` syntax (lists comma-separated, `alignment.unity_ease=1,1,2`), each applied to every config. |
| `difficulties` | `[0]` | `adversary.difficulty` levels. |
| `agents` | | Agent specs, weakest first. |
| `games`, `first_seed` | `first_seed` 0 | Seeds per cell. `--games`/`--seed` override. |
| `selection_seeds` | none | `{"first": N, "games": M}`: the seeds that picked a winner. Adds *Fresh-seed confirmation*; `--resume` refuses a stream started without it. |
| `max_actions` | `5000` | Per-game cap (`timeout`); `--max-actions` overrides. |
| `winprob` | `{"games": 0, "rollouts": 8, "agent": "greedy"}` | **Off unless set.** Win probability by round: `rollouts` playouts by `agent` at each round's start, on each cell's first `games` seeds (`--winprob N`). |
| `closeness` | `{"every": 0, "gap": 2.0}` | **Off unless set.** Every n-th real choice (`--closeness N`), close when the best two 1-ply greedy scores are within `gap` (about credits). |
| `records` | `4` | Records saved per category. |
| `gate` | none | Early stopping (`gate.rs`; gates are suspended, see *The ballpark read*). |
| `inject_panic` | none | `{"cell", "seed"}`: that game panics, for a test. |

`experiment` refuses a config naming a parameter the binary lacks, so re-measuring older rules with their binary needs
a saved copy of that revision's config. Agent search settings go bare, heuristic weights take `w.`
(`engine:samples=2`, `engine:w.packet_keep=0.5`); an unprefixed weight is an error, not a no-op.

## Output

`--out` (default `out/experiments/<name>/`) holds `report.md`, `results.json` (`aic-experiment/2`), `games.csv` (a row
a game), `cards.csv` (per agent, variant, config, difficulty and card), `records/` and `games.jsonl` (`aic-games/2`).

**Paired reads.** *Agents paired, per variant* (`agent_pairs`): A against B on the same seeds; `x:...` pairs with `x`
if listed, a bare agent with the next bare agent. *Fresh-seed confirmation* (`fresh_confirmation`): each variant
against the base on the seeds outside `selection_seeds` alone, never pooled; `first_seed`/`games` must reach
past the selection range for any to exist.

**Results as they go.** Each finished game is appended to `games.jsonl` (actions included), and the report is rewritten
every `--report-every` seconds (default 60), marked **Partial: N of M games**. Games go out seed by seed across all
cells, so a stopped run has every cell's first seeds. `--resume` (same spec, same `--out`) plays only the missing
games and refuses a changed spec, config file, engine, rules revision or content. Without `--resume` an earlier stream
is moved to `games.jsonl.bak`. A partial report that can't be written (`games.csv` open in a spreadsheet locks it on
Windows) is skipped with a warning.

- `report.md` and `results.json` exist from the start: wait on `^time:` in stderr or on the process, not on the file.
- `run_sim.ps1` writes `out/NAME.err` as it runs (`panicked`, or the 10 s progress lines); `out/NAME.exit` marks the end.

**A game that panics** fails alone: stderr and `failed.jsonl` name its seed, config, overrides and panic, `report.md`
lists it under *Failed games* with the line that replays it, `results.json` has `games_failed`, no measure counts it,
and the run exits 1. `--resume` plays it again. `aic-sim run` and `compare` have no such net.

## Measures

- **Skill gap**: top tier's win rate minus the bottom's (each tier by its best agent), also on **survival**: a loss's
  round; a win counts its rounds plus adversary cards left plus 2, so any win ranks above any loss. *Ordered*: no tier
  below the one under it.
- **Pick position** is the card signal (every dealt card is drafted, so how early it goes); played is per card gained.
- **Economy buildings standing**: passive `income` or `income_per_tag` at the end, overlay included.
- **Early lock-in**: p ≤ 0.05 or ≥ 0.95 from round 2 to the end; uncertain checkpoints are 0.1 < p < 0.9.
- **Unspent resources**: at each seat's last action phase (`unspent.<seat>.<resource>` in `per_round`); the flag
  thresholds are `UNSPENT_HIGH` in `experiment.rs` and `scripts/screens.py`.
- **Top agent**: highest win rate at the spec's highest difficulty (ties to the later agent). *Beats greedy* (head to
  head against `greedy`, else the first greedy-kind agent, same seeds, exact sign test) only when ahead with p < 0.05.
- **Variants against the base** pair on seeds; a variant switching a module off reads as what the module is worth.
- **Luck share**: sums-of-squares shares of outcome variance over seeds × agents: seed (luck), agent (skill), rest.
- **Margin**: a win's adversary cards left and ambassador supply; a loss's adversary pieces over the bar
  (`margin.level`; `margin.unity_tips`); every game, `margin.cities_corrupted_end`, `_aligned_end`.

**Counters.** `games.csv` carries the margin columns and the core's `adversary.corruptions`,
`adversary.first_corruption_round`, `board.std_actions` and `board.war_destroyed`; every whole-game counter is in
`games.jsonl`. Per-round tables (`<key>.rNN`, summed over seats) exist only for keys `PER_ROUND_EXPORT` in
`experiment.rs` names: `cards.`, `ai.`, `economy.`, `board.node`, and `unspent.<seat>.energy` (per seat: energy held
as the action phase ends). Add a pair there for another; an aic-sim rebuild, no script change. A zero is written for
every round up to the key's last record (100), so an absent `unspent.*.rNN` means the round's action phase never ended. A read that takes a key's presence as
"reached" (`ai.tag_top`, in `energy_read.py`, `r37_read.py`, `economy_read.py`) reads a seat with no tagged card
differently on an older games.jsonl: don't diff such reads across 100. In the core `cx.count(g, "x", n)` writes
`<module>.x` while `cx.tally` and `cx.per_round` take the full key, so a power use is `ai.power.builder.0`: a screen
column on the bare name prints zeros.

**Saved records** (`records/`, reason in `note`): close losses (fewest pieces over the bar, top difficulty), early
lock-ins, surprising wins (the weakest agent's, or at playout p ≤ 0.25), ranked by measured win rate at the top
difficulty on the base variant. A rerun into the same directory replaces them.

## Opt-in measures and `--quick`

**Screen first, probe finalists.** Win-probability probes cost about 5× an experiment, closeness about one greedy
decision per probed decision. Probe only the finalists: `--variants keep2,uplinks --winprob 10 --closeness 1 --out
out/experiments/x-finalists` (`--variants A,B` keeps those variants plus the base, always first).

**`--quick`** (scoping runs) drops deep search (beam, rollout, `samples`/`depth` > 1, `engine:width` > 0; the planner
stays), caps seeds at 16 a cell unless `--games` is given, and turns probes off unless asked. The only sign of a dropped
agent is one stderr line and a missing cell, so an `engine:samples=2` you meant to compare is easy to lose.

## Content levers

**Content levers run on one binary.** A variant's `set` (or `--set` on `run` and `compare`) takes an overlay,
`<table>.<id>.<path>=<JSON>`: tables `cards`, `threats`, `adversary`, `ais`, `alignment`, `projects`; `*` for every id;
a path through fields and list indexes (`cards.telemetry_archive.cost=3`, `cards.orbital_embassy.passives.0.amount=2`,
`ais.*.economy_track.4.cost=2`, `threats.<id>.counter=[["credits",3]]`; `corps.horizon.project.after=[]` reaches the
map's country pieces by lowercase name, the only part of the map an overlay reaches; `null` clears an optional field,
`cards.<id>.ability=null`). It is checked like `content/`
and lands in the config, so the hashes and records name it and `replay` needs nothing else
(`experiments/overlay-card-cost.json` is a worked sweep). Only player-card text is generated: other text and the
viewer's faces show printed values while the game plays the overlay. `experiment` refuses `--set` and a spec has no
set shared by its variants, so a lever on two sizes is two variants, each repeating the whole list.

**Bisect a batch by groups** of overlays on one binary. A price lever's first read goes on the engine, many times
cheaper than the planner; the planner confirms (an engine read is a direction).

## Reading noise

**Seeds are noisy.** 24 seeds is about ±10 and 48 about ±7; 16 seeds about ±15, too few to separate levers under
about 20 points. Seed sets alone move a pair 10-25 points at 48 seeds.

`cards.csv` splits by variant; `report.md`'s card table pools per agent. `games.jsonl` names agents as the spec writes them (`strategist`), `cards.csv` as resolved (`engine:strategy=1`): map them when joining the two. `kept` and `drawn` in `games.jsonl` include
the opening deal and `offered` doesn't, so kept ÷ offered passes 100%: use share of games. An agent's keeps are its
plan, not a human table's.

## The ballpark read

Matt's ruling, 2026-11-03: simulation is a ballpark check until he names a milestone. Two players only, rotating the
board pairs (Builder + Protector, Builder + Researcher, Protector + Researcher; a pair's variant sets `ai.boards`, as
`ai.slots` and `ai.start_credits` are by board), no c3, no gate; target 45-85 a pair, aiming near 60-75 without
chasing it.

**The judge is the strategist** (`engine:strategy=1`; why: [agents.md](agents.md), *The tiers*); `sweep` is the spot
check of a verdict that matters, `planner:strategy=1` the ceiling, and `sweep:plan_table=0,w.table=0,w.packet_keep=1`
the judge off the value table.

- **Order:** the engine smoke read (`use-090.json`, 16-32 seeds, 20-30 s on 6 threads), then the strategist read.
- **Current recipe:** `screen-102-c.json` (the three pairs at d2, 200 seeds each, about 2 min on 6 threads) and
  `-c-d0` ("greedy at d0" is the `engine` spec, no strategy); `screen-102-a*.json` screen agent switches on and off as
  agent cells of one spec. `screen-098*.json` hold r38's cells: `-terms` (an agent term off, per cell), `-split` / `-t0`
  (each ruling alone), read with `scripts/cards/r38_read.py`; `screen-096*` and `-093*` hold r37's and r36's cells.
- **Costs:** an engine game about 0.7 s, a `sweep` screen of 432 games about 9 min (about 13 ms a planner decision);
  an agent-sweep setting (300 strategist games) about 4 min on 4 threads under load (r45: 1.2-2 games/s).
- **A before/after read is paired** (Matt, 093): run the old binary, with its configs from `git archive <main> configs
  | tar -x`, on the same seeds. Build and copy the old binary before the first edit, or build
  it from a `git archive` with its own `CARGO_TARGET_DIR`. Run it from its own folder, with the spec copied there:
  specs and their configs resolve against the working directory. A counter the read needs that the old binary
  lacks: patch in only the tally, build that as the old binary. The quickest
  check that the new code is play-neutral: the new binary with every content change reverted by overlay should
  reproduce the old numbers exactly (105: 34 overlays, same games). A prompt's "greedy" is the `greedy` agent; the
  house's "greedy at d0" is the `engine` spec.
- **Size levers:** credits feed the purge almost directly through the gain-an-ambassador project, so a size lever's
  first read is ambassadors bought (`board.std.ambassador`). Income levers are overlays (`ais.builder.credits=16`,
  `corps.<name>.project.<path>`) and need no build (the old binary with the new numbers by overlay is a
  content change's numbers-only arm, 117); a rule or map what-if does.
- **Copy the binary before a run, then rebuild freely.** **Preflight** a long run with `SPEC --games 1 --out
  out/preflight`: a crash, bad override or dead agent shows in minutes.
- **Reading:** `scripts/bench/paired.py DIR`, `scripts/cards/use_read.py DIR` and the other read scripts in
  [scripts.md](scripts.md). Credits a seat left at game end are `margin.unspent[seat][0]` in `games.jsonl` (117's
  `experiments/results/117_big_cards_and_income/read.py pair OLD NEW` pairs them). **A won game ends inside a round,
  before upkeep, holding the last upkeep's income** (r45-r46 strategist: about 62 a seat in won games, 8-11 after every
  completed action phase), so the mean mostly tracks late income and the win rate: read what seats hold after their
  last completed action phase (`economy.credits_unspent.rNN`, `117_big_cards_and_income/end_credits.py`).

`experiments/band.json` places a win rate (`--seed S --games 96`, planner and engine); read its win mix (purge vs.
Unity deck) before any rule touching the end state. `experiments/sanity-line.json`: after a change to the rollout
policy or `prune`, the line alone should read near the engine.

**Gates are suspended** until Matt names a milestone. The spec is `experiments/gate.json`, the early-stopping rule
`crates/aic-sim/src/gate.rs` (at `z` 2 about 1% of runs get the sign the cap wouldn't: `scripts/bench/gate_stop_rate.py`).

**Fresh seeds for a gate or a confirming read.** Specs start at seed 0: give a confirming read a `--seed` no screen
of this revision used, or it confirms on the seeds that picked the levers. The ledger, updated by any item that spends
seeds and reset at each bump (seeds below 33836 went to r43 and earlier): r44: 33836-33883 111's read of r44 against
r43 (`screen-111-r44.json`). r45: 33884-33931 112's read of r45 against r44 (`screen-112-r45.json`); 33932-34031 116's agent
sweep (every setting, `scripts/tune/agent_sweep.py`); 34032-34127 116's held-out confirm and step check; 34128-34223 116's `w.horizon=0.75` check. r46: 34224-34271
117's reads (`screen-117-r45.json`, both binaries; `screen-117-income.json`, r46's incomes as overlays on r45:
the income step alone). r47: 34272-351, 34400-1, 34500-47, 34600-2, 34700-7 118. Next free: 34708.

## Reading prices

`python scripts/cards/price_read.py DIR` gives offered, kept and played per game and win deltas stratified on rounds,
from `games.jsonl` (`cards.csv` has no outcome). A price sweep is a spec of overlays in `set`, paired against a base
on the same seeds. How prices are set: [card-pricing.md](card-pricing.md).

## Past runs

Write-ups are indexed in [experiments/history.md](../experiments/history.md). Specs the binary refuses are in
`experiments/archive/`; committed results from r5 and earlier do not replay. Tuning sweeps: `scripts/tune/sweep.py
--agents "greedy;beam" --games 64 --tag T --var "base=" --var "name=<sets;...>"` runs `aic-sim run --json` per variant
and agent into `out/tune/log.jsonl`; `log_table.py TAG...` makes a markdown table.

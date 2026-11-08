# Experiments

```sh
S=target/release/aic-sim
$S experiment experiments/smoke.json --out out/smoke    # a quick check (6 seeds)
$S experiment experiments/ladder-v0.json --games 20 --seed 1000 --threads 6
$S experiment experiments/read-150-f1-a.json --quick --resume  # scoping; resumes with its --out, --games, --seed
```

## The spec

Configs × variants × difficulties × agents over a seed range, on every core; results never depend on the thread count
and every game is replay-checked. Spec paths resolve against the working directory (a spec not yet copied into a
second checkout is refused).

| Field | Default | Meaning |
|---|---|---|
| `name`, `note` | | Report title and a line on what it is for. |
| `configs` | | Config files. |
| `variants` | `[{"name": "base", "set": []}]` | Named override sets in `--set` syntax (lists comma-separated, `alignment.purge_at=1,1,2`), each applied to every config. |
| `difficulties` | `[0]` | `adversary.difficulty` levels. |
| `agents` | | Agent specs, weakest first. |
| `games`, `first_seed` | `first_seed` 0 | Seeds per cell. `--games`/`--seed` override. |
| `selection_seeds` | none | `{"first": N, "games": M}`: the seeds that picked a winner. Adds *Fresh-seed confirmation*; `--resume` refuses a stream started without it. |
| `max_actions` | `5000` | Per-game cap (`timeout`); `--max-actions` overrides. |
| `winprob` | `{"games": 0, "rollouts": 8, "agent": "greedy"}` | **Off unless set.** Win probability by round: `rollouts` playouts by `agent` at each round's start, on each cell's first `games` seeds (`--winprob N`). |
| `closeness` | `{"every": 0, "gap": 2.0}` | **Off unless set.** Every n-th real choice (`--closeness N`), close when the best two 1-ply greedy scores are within `gap` (about credits). |
| `records` | `4` | Records saved per category. |
| `gate` | none | Early stopping (suspended: *The ballpark read*). |
| `inject_panic` | none | `{"cell", "seed"}`: that game panics, for a test. |

`experiment` refuses a config naming a parameter the binary lacks, so an old binary needs its revision's configs
(*The ballpark read*). Agent search settings go bare, heuristic weights take `w.`
(`engine:samples=2`, `engine:w.packet_keep=0.5`); an unprefixed weight is an error, not a no-op.

## Output

`--out` (default `out/experiments/<name>/`) holds `report.md`, `results.json` (`aic-experiment/2`), `games.csv` (a row
a game), `cards.csv` (per agent, variant, config, difficulty and card), `records/` and `games.jsonl` (`aic-games/2`).

**Paired reads.** *Agents paired, per variant* (`agent_pairs`): A against B on the same seeds; `x:...` pairs with `x`
if listed, a bare agent with the next bare agent. *Fresh-seed confirmation* (`fresh_confirmation`): each variant
against the base on the seeds outside `selection_seeds` alone, never pooled; `first_seed`/`games` must reach
past the selection range for any to exist.

**Results as they go.** Each finished game is appended to `games.jsonl` (actions included); the report is rewritten
every `--report-every` seconds (60), marked **Partial: N of M games**. Games go out seed by seed across cells, so a
stopped run has every cell's first seeds. `--resume` (same spec and `--out`) plays only the missing games and refuses
a changed spec, config, engine, rules revision or content; without it an earlier stream moves to `games.jsonl.bak`. A
partial report blocked by a lock (`games.csv` open in a spreadsheet) is skipped with a warning.

- `report.md` and `results.json` exist from the start: wait on `^time:` in stderr or on the process, not on the file.
- `run_sim.ps1` writes `out/NAME.err` as it runs (`panicked`, or the 10 s progress lines); `out/NAME.exit` marks the end.

**A game that panics** fails alone: stderr and `failed.jsonl` name its seed, config, overrides and panic, `report.md`
lists it under *Failed games* with its replay line, `results.json` has `games_failed`, no measure counts it, and the
run exits 1. `--resume` plays it again. `aic-sim run` and `compare` have no such net.

## Measures

- **Skill gap**: top tier's win rate minus the bottom's (each tier by its best agent), also on **survival**: a loss's
  round; a win counts its rounds plus adversary cards left plus 2, so any win ranks above any loss. *Ordered*: no tier
  below the one under it.
- **Pick position** is the card signal (every dealt card is drafted, so how early it goes); played is per card gained.
- **Economy buildings standing**: passive `income` or `income_per_tag` at the end, overlay included.
- **Early lock-in**: p ≤ 0.05 or ≥ 0.95 from round 2 to the end; uncertain checkpoints are 0.1 < p < 0.9.
- **Unspent resources**: at each seat's last action phase (`unspent.<seat>.<resource>` in `per_round`); the flag
  thresholds are `UNSPENT_HIGH` in `experiment.rs` and `scripts/screens.py`. `margin.held_after_actions`: what each
  seat held after its last *completed* action phase, never mid-round.
- **The runaway check**: each resource's peak per seat (`peaks` in `games.jsonl`, `PEAKS` order; *Peak holdings*).
  A peak's rise over setup's `start` far above a round's income (`gained / rounds`) is a runaway engine:
  `scripts/cards/runaway_read.py` flags it. `sources` splits a seat's `gained` by action or phase; `keeps` counts
  picks, keeps and tiers after the opening (`source_read.py`); `spent` and `uses` split falls. 127's
  `read127.py`, `production_read.py` and `seat_read.py` read them (their docstrings). No resource has a limit.
- **Top agent**: highest win rate at the spec's highest difficulty (ties to the later agent). *Beats greedy* (head to
  head against `greedy`, else the first greedy-kind agent, same seeds, exact sign test) only when ahead with p < 0.05.
- **Variants against the base** pair on seeds; a variant switching a module off reads as what the module is worth.
- **Margin**: a win's adversary cards left and ambassador supply (a loss ending `deck_empty` is the round cap); a loss's adversary pieces over the bar
  (`margin.level`; `margin.unity_tips`); every game, `margin.cities_corrupted_end`, `_aligned_end`.

**Counters.** `games.csv` carries the margin columns and the core's `adversary.corruptions` and
`adversary.first_corruption_round` (any site since 138: cities alone are `adversary.corruptions.city`), `board.std_actions` and `board.war_destroyed`; every whole-game counter is in
`games.jsonl`. Per-round tables (`<key>.rNN`, summed over seats) exist only for keys `PER_ROUND_EXPORT` in
`experiment.rs` names: `cards.`, `ai.`, `economy.`, `board.node`, `threats.`, `unspent.<seat>.<resource>` (held as the
action phase ends, before production and upkeep pay; research cubes too since 148); add one there (a rebuild). A zero fills every round up to the key's last record, so an absent
`unspent.*.rNN` means the round's action phase never ended. `cx.count(g, "x", n)` writes
`<module>.x`; `cx.tally` and `cx.per_round` take the full key (`ai.power.builder.0`). The sim adds, as a seat
passes: `actions.skipped|blocked[_energy].<seat>.<card>` (a building's Action offered and not taken, or not offered:
used, unaffordable or no target; `_energy` sums energy held), `energy_cards.held|short.<seat>[.<card>]`, `slots.*`.

**Saved records** (`records/`, reason in `note`): close losses (fewest pieces over the bar, top difficulty), early
lock-ins, surprising wins (the weakest agent's, or at playout p ≤ 0.25), ranked by measured win rate at the top
difficulty on the base variant. A rerun into the same directory replaces them.

## Opt-in measures and `--quick`

**Screen first, probe finalists.** Win-probability probes cost about 5× an experiment, closeness one greedy decision
per probe: `--variants keep2,uplinks --winprob 10 --closeness 1` (`--variants A,B` keeps those plus the base, first).

**`--quick`** (scoping) drops deep search (beam, rollout, `samples`/`depth` > 1, `engine:width` > 0; the planner
stays), caps seeds at 16 a cell unless `--games` is given, and turns probes off unless asked; a dropped agent is one
stderr line and a missing cell.

## Content levers

**Content levers run on one binary.** A variant's `set` (or `--set` on `run` and `compare`) takes an overlay,
`<table>.<id>.<path>=<JSON>`: tables `cards`, `threats`, `adversary`, `ais`, `alignment`, `projects`; `*` for every id;
a path through fields and list indexes (`cards.telemetry_archive.cost=3`, `cards.orbital_embassy.passives.0.amount=2`,
`ais.*.economy_track.4.cost=2`, `threats.<id>.counter=[["credits",3]]`; `corps.horizon.project.cubes=8`: country
pieces by lowercase name, the only part of the map it reaches; `null` clears an optional field,
`cards.<id>.ability=null`; an op's `amount` can't be 0: to switch an income off, rewrite its list without it, as
122's `gen_disproof.py` does). It is checked like `content/` and lands in the config, so the hashes and records name
it and `replay` needs nothing else (`experiments/overlay-card-cost.json` is a worked sweep). Only player-card text is
generated: other text and the viewer's faces show printed values. `experiment` refuses `--set` and a spec has no
shared set, so each variant repeats the whole list.

**Bisect a batch by groups** of overlays on one binary. A price lever's first read is on the engine (many times
cheaper); the planner confirms.

## Reading noise

**Seeds are noisy.** 24 seeds is about ±10, 48 about ±7, 16 about ±15. Seed sets alone move a pair 10-25 points at 48
seeds (148's two d3 sets: 34.0 and 59.7).

`cards.csv` splits by variant; `report.md`'s card table pools per agent. `games.jsonl` names agents as the spec
writes them (`strategist`), `cards.csv` as resolved (`engine:strategy=1`): map them to join. `kept` and `drawn` in `games.jsonl`
include the opening deal and `offered` doesn't, so kept ÷ offered passes 100%: use share of games. An agent's keeps
are its plan, not a human table's.

## The ballpark read

Matt's ruling, 2026-11-03: simulation is a ballpark check until he names a milestone. Two players only, rotating the
board pairs (Builder + Protector, Builder + Researcher, Protector + Researcher; a pair's variant sets `ai.boards`, as
`ai.slots` and `ai.start_credits` are by board), no c3, no gate; target 45-85 a pair, aiming near 60-75 without
chasing it.

**The judge is the strategist** (`engine:strategy=1`; why: [agents.md](agents.md), *The tiers*); `sweep` is the spot
check of a verdict that matters, `planner:strategy=1` the ceiling, and `sweep:plan_table=0,w.table=0,w.packet_keep=1`
the judge off the value table.

- **Order:** the engine smoke read (`use.json`, 16-32 seeds, 20-30 s on 6 threads), then the strategist read.
- **Current recipe:** 150's `specs150.py` (two 48-seed sets, the three pairs, d0-d3; a second config rides on the
  same seeds) on both binaries, read by `150_tags_exascale_r64/ladder150.py` (pooled, paired, by set, by config) and
  147's `read147.py` (win, ends, nodes lost by cause,
  counters by tier); screens are `<pair>_<lever>` cells.
- **Costs:** an engine game about 0.7 s; the r58 standard read (432 strategist games, 9 rounds) 88 s on 6 threads on
  a quiet box (141), twice that with a second run beside.
- **A before/after read is paired**: the old binary, with its configs (`git archive <main> configs | tar -x`), on the
  same seeds. Copy it before the first edit, or build `git archive REV | tar -x -C out/rREV` with its own
  `CARGO_TARGET_DIR`; for new sim-only measures, `patch -p1 < measures.patch` in that tree first (`git apply` in a
  worktree resolves paths against the enclosing repo and silently applies nothing). Run it from its own folder, spec
  copied there. `bench/per_decision.py` plays each ROOT's own `configs/`: copy them beside both binaries.
- **Size levers:** a size lever's first read is ambassadors bought (`board.std.ambassador`). Income levers are
  overlays (`ais.builder.credits=16`) and need no build; a rule or map what-if does.
- **Copy the binary before a run, then rebuild freely.** **Preflight** a long run (`SPEC --games 1 --out
  out/preflight`) for a crash, bad override or dead agent.
- **Reading:** `scripts/bench/paired.py DIR`, `scripts/cards/use_read.py DIR` and others in
  [scripts.md](scripts.md). **A won game ends inside a round, holding the last upkeep's income**, so
  `margin.unspent` tracks late income: read credits held as `margin.held_after_actions[seat][0]`.

`experiments/band.json` places a win rate (`--seed S --games 96`, the planner at 2 and 3 countries); read its win mix (purge vs.
Unity deck) before a rule touching the end state. `sanity-line.json`: after a rollout policy or `prune` change, the
line alone reads near the engine.

**Gates are suspended** until Matt names a milestone. The spec is `experiments/gate.json`, the early-stopping rule
`crates/aic-sim/src/gate.rs` (`z` 2 flips about 1% of verdicts: `scripts/bench/gate_stop_rate.py`).

**Fresh seeds for a gate or a confirming read.** Specs start at seed 0: give a confirming read a `--seed` no screen
of this revision used, or it confirms on the seeds that picked the levers. Any item that spends seeds updates the
ledger, after its last read, not before it. Used: every seed below 38392 (which item spent which is in its write-up;
149's: 37624-37911, three paired reads; 150's: 37912-38391, five). Next free: 38392. **The dev seeds for screens are 36800-36831**; screen at
32 seeds a cell, and expect 48-seed sets to differ by up to ~10 points at d3 (146's confirming sets read
52.8 and 44.4).

## Reading prices

`python scripts/cards/price_read.py DIR` gives offered, kept and played per game and win deltas stratified on rounds,
from `games.jsonl` (`cards.csv` has no outcome). A price sweep is a spec of overlays in `set`, paired against a base
on the same seeds. How prices are set: [card-pricing.md](card-pricing.md).

## Past runs

Write-ups: index in [experiments/history.md](../experiments/history.md). Past revisions' specs, and those the binary
refuses, are in `experiments/archive/` (a fast-lane test parses each spec at the top level and in `gate-close/`: removing a switch
moves its specs there). Tuning sweeps: `scripts/tune/sweep.py --agents "greedy;beam" --games 64 --tag T --var "base="
--var "name=<sets;...>"` runs `aic-sim run --json` per variant and agent into `out/tune/log.jsonl`; `log_table.py
TAG...` makes a markdown table.

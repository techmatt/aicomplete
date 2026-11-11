# Experiments

```sh
S=target/release/aic-sim
$S experiment experiments/smoke.json --out out/smoke    # a quick check (6 seeds)
$S experiment experiments/ladder-v0.json --games 20 --seed 1000 --threads 6
$S experiment experiments/use.json --quick --resume     # scoping; resumes with its --out, --games, --seed
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
| `endgame` | none | `{"back": 3, "agent": "", "rollouts": 1, "all_losses": false}`: each round-cap loss's end state (`all_losses`: every loss's), the bound's inputs at the start of the `back`-th last round, and `agent`'s replays from there (`crates/aic-sim/src/endgame.rs`). |
| `records` | `4` | Records saved per category. |
| `gate` | none | Early stopping (suspended: *The ballpark read*). |
| `inject_panic` | none | `{"cell", "seed"}`: that game panics, for a test. |

`experiment` refuses a config naming a parameter the binary lacks, so an old binary needs its revision's configs.
Agent search settings go bare, heuristic weights take `w.`
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
a changed spec, config, engine, rules revision or content; without it an earlier stream moves to `games.jsonl.bak`.

- `report.md` and `results.json` exist from the start: wait on `^time:` in stderr or on the process, not on the file.

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
- **Unspent resources**: at each seat's last action phase (`unspent.<seat>.<resource>`; flags at `UNSPENT_HIGH`).
  `margin.held_after_actions`: held after the last *completed* action phase, never mid-round. A won game ends inside
  a round holding the last upkeep's income, so `margin.unspent` tracks late income: read credits held as
  `margin.held_after_actions[seat][0]`.
- **The runaway check**: each resource's peak per seat (`peaks`, `PEAKS` order) far above a round's income
  (`gained / rounds`) is a runaway engine: `scripts/cards/runaway_read.py`. `sources` / `uses` / `keeps`:
  `source_read.py`.
- **Top agent**: highest win rate at the top difficulty (ties to the later). *Beats greedy*: ahead of the first
  greedy-kind agent on the same seeds, exact sign test p < 0.05.
- **Variants against the base** pair on seeds; a variant switching a module off reads as what the module is worth.
- **Margin**: a win's adversary cards left and ambassador supply; a loss's adversary pieces over the bar
  (`margin.level`; `margin.unity_tips`); every game, `margin.cities_corrupted_end`, `_aligned_end`. A `deck_empty`
  loss (the round cap) counts `rounds` one past the cap's printed count.

**Counters.** `games.csv` carries the margin columns and the core's `adversary.corruptions` and
`adversary.first_corruption_round` (any site; cities alone are `adversary.corruptions.city`), `board.std_actions` and `board.war_destroyed`; every whole-game counter is in
`games.jsonl`. Per-round tables (`<key>.rNN`, summed over seats) exist only for keys `PER_ROUND_EXPORT` in
`experiment.rs` names (add one there: a rebuild); `unspent.<seat>.<resource>` is held as the action phase ends,
before production and upkeep pay, research cubes too. A zero fills every round up to the key's last record, so an absent
`unspent.*.rNN` means the round's action phase never ended. The sim adds, as a seat
passes: `actions.skipped|blocked[_energy].<seat>.<card>` (a building's Action offered and not taken, or not offered:
used, unaffordable or no target; `_energy` sums energy held), `energy_cards.held|short.<seat>[.<card>]`, `slots.*`.

**Saved records** (`records/`, reason in `note`): close losses (fewest pieces over the bar, top difficulty), early
lock-ins, surprising wins (the weakest agent's, or at playout p ≤ 0.25), ranked by measured win rate at the top
difficulty on the base variant.

## Opt-in measures and `--quick`

**Screen first, probe finalists.** Win-probability probes cost about 5× an experiment, closeness one greedy decision
a probe: `--variants keep2,uplinks --winprob 10 --closeness 1` (keeps those variants plus the base).

**`--quick`** (scoping) drops deep search (beam, rollout, `samples`/`depth` > 1, `engine:width` > 0; the planner
stays), caps seeds at 16 a cell unless `--games` is given, and turns probes off unless asked.

## Content levers

**Content levers run on one binary.** A variant's `set` (or `--set` on `run` and `compare`) takes an overlay,
`<table>.<id>.<path>=<JSON>`: tables `cards`, `threats`, `adversary`, `ais`, `alignment`, `projects`; `*` for every id;
a path through fields and list indexes (`ais.*.economy_track.4.cost=2`; `corps.horizon.project.cubes=8`: country
pieces by lowercase name; `null` clears an optional field; an op's `amount` can't be 0: rewrite its list without it).
It is checked like `content/` and lands in the config, so `replay` needs nothing else
(`overlay-card-cost.json`: a sweep). Each variant repeats its whole list.

**Diagnose a rewrite's gap by part before tuning** (171): restore the old content one part at a time (a side, a tier,
a card group) as overlays, and read which part carries the gap.

## Reading noise

**Seeds are noisy.** 24 seeds is about ±10, 48 about ±7, 16 about ±15. Seed sets alone move a pair 10-25 points at 48
seeds. A screen's SE is about 50/√n points over n paired games: size a screen so its SE is at most half the effect it
must resolve (171).

`cards.csv` splits by variant and names agents as resolved (`engine:strategy=1`; `games.jsonl` as the spec writes
them). `kept` and `drawn` include the opening deal and `offered` doesn't: use share of games.

## The ballpark read

The ruling and its targets: [principles.md](principles.md), *Ballpark simulation*. A board pair's variant sets
`ai.boards`, as `ai.slots` and `ai.start_credits` are by board.

**The judge is the strategist** (`engine:strategy=1`; why: [agents.md](agents.md), *The tiers*); `sweep` is the spot
check of a verdict that matters, `planner:strategy=1` the ceiling, and `sweep:plan_table=0,w.table=0,w.packet_keep=1`
the judge off the value table.

- **Order:** the engine smoke read (`use.json`, 16-32 seeds, 20-30 s on 6 threads), then the strategist read. After a
  change to how the game is won, greedy first ([agents.md](agents.md), *The tiers*): a greedy lead stops calibration
  for a question.
- **Current recipe:** `scripts/bench/read_specs.py read TAG SEED --tag NNN [--probe]` (three 48-seed sets, the three
  pairs, d0-d3, into `out/`; `screen` a dev screen, `--offs` a config per new card switched off) on both binaries
  (`scripts/bench/run_read.sh`; flags in [scripts.md](scripts.md)). Read by `scripts/bench/ladder.py NEW OLD
  --measures` (pooled, paired, by set), and the item readers in `experiments/results/`: `wins158.py` (by lever or
  config), `rates158.py` (card rates), `ends157.py` and `endgame157.py` (by Unity step), `partial157.py` (a live run). Since 174 greedy costs
  about what the strategist does (2 ms a decision, about 1 s a game).
- **Standing measures (amendment 6618), reported old against new every read:** corruption node losses and adversary
  ambassadors placed a game, and the project deck's turnover: projects revealed (`adversary.projects_revealed`, every
  draw) and resolved as a share of the deck, distinct projects revealed, and reshuffles, per level (`--measures`). A
  measure the old binary never tallied goes into its tree as a patch before its first read, or the read is redone.
- **Costs (r73, 174):** a 576-game strategist read set (d0-d3) about 1.5 min on 6 threads (7 min before 174), a
  720-game set with d4 about 2 min; double on a busy box.
- **A before/after read is paired**: the old binary and its configs (README, *This machine*), same seeds, own folder.
  Copy it before the first edit, or build `git archive REV | tar -x -C out/rREV` (own `CARGO_TARGET_DIR`); new
  sim-only measures go in as `patch -p1` there (`git apply` in a worktree silently applies nothing).
- **Farming a read:** `experiments/farm/read.json` is the read as one spec ([simulator.md](simulator.md), *The farm*).
  166: 48 seeds by `farm-seeds` on the main box alone took 342 s (482 s beside 6 other threads); the old side as 6
  native shards of 8 seeds (`aic-sim farm read --seed S --games 8`, its folder's configs) about as long. Assemble each
  side as `aic-sim farm read --header --seed F --games N` then the shards in seed order, and read with `ladder.py`.
- **Levers:** an income or card lever is an overlay (`cards.<id>.effects.0.max=5`, *Content levers*), no build; a
  rule, content or map what-if needs one.
- **Same games:** `experiments/same-noplanner.json` is same.json ([speed.md](speed.md)) with greedy for the planner
  rows (about 21 s a game at r73): for a refactor not reaching the planner or oracle (~2 min a side).
- **Copy the binary before a run, then rebuild freely.** **Preflight** a long run (`SPEC --games 1 --out
  out/preflight`) for a crash, bad override or dead agent.
- **Reading:** `scripts/bench/paired.py DIR`, `scripts/cards/use_read.py DIR` and others in
  [scripts.md](scripts.md).

**Skilled rows (155).** `python scripts/bench/skilled_rows.py READ.json` writes the read's companion spec: the
`oracle` (full information, [agents.md](agents.md)) and the strategist on the read's first 24 seeds a level (8 a
pair), d3 and d4, the end-game probe on every loss. At r73 (174) an oracle game costs 30-160 s (a mean of 67 s),
and the row's wall time, about 20 min on 3 threads, is its slowest game (10-18 min): launch it with the read; `threads=N` plays a decision's lines side by side. Spec and costs: `155_skilled_agents/`.

`experiments/band.json` places a win rate (`--seed S --games 96`, the planner at 2 and 3 countries); read its win mix (the purge by
Unity step, `ends156.py`) before a rule touching the end state. `sanity-line.json`: after a rollout policy or `prune` change, the
line alone reads near the engine.

**Gates are suspended** ([principles.md](principles.md)).

**Fresh seeds for a gate or a confirming read.** Specs start at seed 0: give a confirming read a `--seed` no screen
of this revision used. Any item that spends seeds updates the ledger, after its last read, not before it. Used: every
seed below 42277 (each item's range is in its write-up). Next free: 42277. **The dev seeds for screens are
36800-36831**: screen at 32 seeds a cell.

## Reading prices

`python scripts/cards/price_read.py DIR` gives offered, kept and played per game and win deltas stratified on rounds,
from `games.jsonl` (`cards.csv` has no outcome). A price sweep is a spec of overlays in `set`, paired against a base
on the same seeds. How prices are set: [card-pricing.md](card-pricing.md).

## Past runs

Write-ups: index in [experiments/history.md](../experiments/history.md). Specs the binary refuses go to
`experiments/archive/` (a fast-lane test parses each spec at the top level). Tuning sweeps: `scripts/tune/sweep.py` (usage in its docstring) runs `aic-sim run --json` per variant and agent
into `out/tune/log.jsonl`; `log_table.py TAG...` makes a markdown table.

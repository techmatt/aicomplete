# Experiments

```sh
S=target/release/aic-sim
$S experiment experiments/ladder-v0.json                    # -> out/experiments/ladder-v0/
$S experiment experiments/smoke.json --out out/smoke        # a quick check (6 seeds)
$S experiment experiments/ladder-v0.json --games 20 --seed 1000 --threads 6
$S experiment experiments/archive/screen-014-board.json --quick     # scoping: shallow agents, 16 seeds, no probes
$S experiment experiments/archive/screen-014-board.json --quick --resume  # continue a stopped run: the same flags (--quick, --out, --games, --seed) as it started with
```

## The spec

An experiment is configs × variants × difficulties × agents over a seed range, played
on every core (one game at a time per worker, results sorted, so nothing depends on the
thread count; every game is replay-checked). The spec (`experiments/*.json`; a path resolves against the working directory, so a spec not yet copied into a second checkout is refused):

| Field | Default | Meaning |
|---|---|---|
| `name`, `note` | | Report title and a line on what it is for. |
| `configs` | | Config files. |
| `variants` | `[{"name": "base", "set": []}]` | Named override sets in `--set` syntax, each applied to every config. |
| `difficulties` | `[0]` | `adversary.difficulty` levels. |
| `agents` | | Agent specs, weakest first. |
| `games`, `first_seed` | `first_seed` 0 | Seeds per cell. `--games`/`--seed` override. |
| `selection_seeds` | none | `{"first": N, "games": M}`: the seeds that picked a winner. Adds the report's *Fresh-seed confirmation*. Part of the spec, so `--resume` refuses a stream started without it. |
| `max_actions` | `5000` | Per-game cap (`timeout`). |
| `winprob` | `{"games": 0, "rollouts": 8, "agent": "greedy"}` | **Off unless set.** Win probability by round: for the first `games` seeds of each cell, `rollouts` playouts by `agent` from a guess of the table at the start of each round. `--winprob N` sets `games`. |
| `closeness` | `{"every": 0, "gap": 2.0}` | **Off unless set** (011; it was on, every 1). Probe every n-th real choice: 1-ply greedy scores of every option; close when the best two are within `gap` heuristic points (about credits). `--closeness N` sets `every`. |
| `records` | `4` | Records saved per category. |
| `gate` | none | `{"z", "min_games", "every"}`: early stopping; `games` becomes each check's cap, see *Gate early-stopping*. |
| `inject_panic` | none | `{"cell", "seed"}`: that game panics, see *A game that panics*. |

Variant sweeps: an experiment file with several `variants`, each a list of `--set`
overrides (lists comma-separated, `alignment.unity_ease=1,1,2`).

`experiment` refuses a spec if its config names a parameter the binary lacks, so
re-measuring older rules with their binary needs a saved copy of that revision's config.

Agent parameters come in two kinds: search settings go bare, heuristic weights need the
`w.` prefix (`engine:samples=2`, `engine:w.packet_keep=0.5`). An unprefixed weight is an
error, not a no-op.

## Output

The output directory (`--out`, default `out/experiments/<name>/`) holds `report.md` (the
cycle report: findings, then the tables), `results.json` (`aic-experiment/2`: the
spec, every cell's and group's measures, the card table, the records), `games.csv` (one
row per game; `economy_buildings_standing` follows `buildings_standing`) and `cards.csv`
(per agent, variant, config, difficulty and card), and `records/`, plus
`games.jsonl` (`aic-games/2`, written as the games finish; see *Results as they go*).
Each cell in `results.json` has its `countries` (052), read from the resolved config.

`results.json` and `report.md` also carry **agents paired, per variant** (`agent_pairs`,
*Agents paired, per variant*; none with a single agent): per config × variant × difficulty, A against B on
the same seeds with the paired standard error and sign test. `x:...` pairs with `x` if
listed; a bare agent with the next bare agent listed. With `selection_seeds`, **fresh-seed
confirmation** (`fresh_confirmation`, *Fresh-seed confirmation*): each variant against the
base on the seeds outside `selection_seeds` alone, the selection seeds' result beside it,
never pooled; it says so if no fresh seed has been played. `first_seed`/`games` must reach
past the selection range for any to exist.

**Results as they go.** Every finished game is appended to `DIR/games.jsonl` (a header
line with the spec, engine, rules revision and content hash, then one game per line,
actions included), and the whole report is rewritten from the games so far every
`--report-every` seconds (default 60; files are written whole and renamed into place),
marked **Partial: N of M games** at the top. A stopped or killed run leaves both.
`--resume` (same spec, same `--out`) reads the stream back, replays each game to rebuild
it, plays only the games missing and reports them all; a different spec, edited config
file, engine, rules revision or content is refused. Without `--resume`, an earlier
stream in the directory is moved to `games.jsonl.bak`. A partial report that can't be
written (say, `games.csv` open in a spreadsheet, which locks it on Windows) is skipped
with a warning; the run goes on. Games are handed out seed by seed across all cells, so
a stopped run has every cell's first seeds.

A results dir's `report.md` and `results.json` exist from the start of a run: wait on `^time:` in stderr or on the process, not on the file.

**`run_sim.ps1` and stderr:** the sim writes `out/NAME.err` as it runs (054), so a wait on `panicked` or
a look at the 10 s progress lines works mid-run. `out/NAME.exit` marks the end.

**A game that panics (050).** It fails alone: the run goes on, stderr and `failed.jsonl`
name its seed, config, overrides and panic, `report.md` lists them under *Failed games* with
the line that plays it again, `results.json` has `games_failed`, no measure counts it, and the
run exits 1. `--resume` plays it again. A spec's `"inject_panic": {"cell": 1, "seed": 51}`
makes one on purpose, for a test. `aic-sim run` and `compare` have no such net: a panic there
ends the run.

## Measures

**Measures** (read from the public state, the actions and the engine's counters in `g.stats`):

- **Win rate** by agent × difficulty (± standard error), and the **skill gap**: the top
  tier's win rate minus the bottom tier's (each tier by its best agent, so a budget sweep
  of one kind is one rung), also on **survival**: the round a loss came in; a win counts
  its rounds plus the adversary cards left plus 2, so any win ranks above any loss.
  Survival is the ladder's signal while few games are won. *Ordered* means no tier does
  worse than the one below it.
- **Decision closeness**: forced decisions (one legal action), and of the real choices
  the share whose best two options are within `gap`.
- **Pick and play rates** per card: offered (in a draft packet), mean pick position
  (every dealt card is drafted by someone, so how early it goes is the signal), kept per
  offer, played per card gained (kept or drawn; per game for unique AI cards, which
  start in hand).
- **Economy buildings standing**: per game at the end, buildings with a passive `income`
  or `income_per_tag`, by the content the game played,
  overlay included. Token stores and `convert_upkeep` alone don't count.
- **Build variety**: distinct end-of-game builds (each seat's AI and buildings) and the
  mean Jaccard distance between two games' building sets, over all games and over wins.
- **Win probability by round** from the playouts; checkpoints that are uncertain
  (0.1 < p < 0.9); early lock-in (the result was decided, p ≤ 0.05 or ≥ 0.95 to the end,
  by round 2).
- **Unspent resources** (068): each seat's credits, chips, energy, antimatter and ambassadors when its last action phase ended (`left.<seat>.<resource>` in
  `per_round`, read by `margin.unspent`): *Unspent resources* in `report.md`, `unspent` in `results.json`, the `--gate` readout. The flag is `UNSPENT_HIGH`
  (`crates/aic-sim/src/experiment.rs`, `scripts/screens.py`).
- **Margin**: on a win, adversary cards left (deck plus the face-up card: rounds to
  spare) and the adversary's ambassador supply left; on a loss, the adversary pieces over
  the current bar (`margin.level` is the bar level and `margin.unity_tips` the Unity tip
  count); every game, `margin.cities_corrupted_end` and `margin.cities_aligned_end`
  (also columns of `games.csv`, with the core's `adversary.corruptions`,
  `adversary.first_corruption_round`, `board.std_actions` and `board.war_destroyed`
  counters; every whole-game counter is in `games.jsonl`. Per-round tables reach it as
  `<key>.rNN`, summed over seats, only for keys starting `cards.`, `ai.`, `economy.` or
  `board.presence`: add to the list in `experiment.rs` for another. In the core `cx.count(g,
  "x", n)` writes `<module>.x` and `cx.tally` and `cx.per_round` take the full key, so a
  power use is `ai.power.builder.0`: a screen column on the bare name prints zeros).
- **Game length**, ends by reason and round; **adversary operations** per round (mean,
  max) and at setup.
- **Help and counters**: gifts, payments and withdrawals per game; threats countered of
  those dealt.
- **Strategy health** (008), per cell × agent: buildings played per game (any card of
  kind `building`, AI uniques included), tier-2 cards played per game, cards played per
  game, and the share of wins with 2 or more buildings played (and standing at the
  end). A findings line per difficulty says whether the top agent plays buildings and
  tier 2. *Top agent*: the highest win rate at the spec's highest difficulty (ties to
  the later agent).
- **Head to head against greedy** (008): every agent against plain `greedy` (or the
  first greedy-kind agent) on the same seeds, per cell: won by both / only the agent /
  only greedy / neither, the win-rate difference ± the standard error of the per-seed
  difference, an exact two-sided sign test on the seeds only one of them won, and a
  verdict (*beats greedy* only when ahead with p < 0.05).
- **Variants against the base** and **removing a module** (008), when a spec has
  several variants: each variant's win rate against the first (the base), paired on
  seeds; for variants that switch a module off (`<module>=off`,
  `<module>.variant=off`), the base minus the off-variant: what the module is worth.
- **Luck share**: a two-way split of outcome variance over seeds × agents within each
  config/variant/difficulty: the seed's share (luck), the agent's (skill), the rest
  (interaction and the agents' own randomness). Raw sums-of-squares shares, no
  degrees-of-freedom correction. On win/loss and on survival.

The per-cell split (agent, probes, total) is in each report's *Timing* table and in
`games.csv`. Timings on older rules are in
[experiments/history.md](../experiments/history.md).

**Saved records** (open in the viewer, or `aic-sim replay`), up to `records` each, with
the reason in the record's `note`: *close losses* at the top difficulty (fewest pieces
over the bar, then the longest games); *early lock-ins* (decided by round 2 in games of
3+ rounds, from the win-probability playouts); *surprising wins* (by the weakest agent,
after the playouts gave them ≤ 0.25, or below the top agent at the top difficulty).
"Weakest", "top" and "stronger first" are by **measured** strength (011), not ladder
tier: agents ranked by win rate at the top difficulty on the base variant (the same
reading as the strategy-health *top agent*); equal win rates share a rank, so with all
agents tied nobody is "weakest".
Files are named `<category>-c<cell>-<config>-<variant>-d<difficulty>-<agent>-seed<S>.json`;
a rerun into the same directory replaces them.

## Opt-in measures and `--quick`

**Expensive measures are opt-in: screen first, probe finalists.** Win-probability
probes cost about 5× an experiment (007's `tune-after`: 45 minutes probed, 9–12
without), and closeness about one greedy decision per probed decision. Both are off
unless a spec or the command line turns them on. The two-stage pattern:

```sh
$S experiment experiments/x.json                                     # 1. screen: every variant, no probes
$S experiment experiments/x.json --variants keep2,uplinks \
    --winprob 10 --closeness 1 --out out/experiments/x-finalists     # 2. probe only the finalists
```

`--variants A,B` keeps only those variants of the spec (its first variant, the base,
is always kept, first, so the comparisons still have their reference). The list at the top of the report says which measures were off.

**Fast by default (`--quick`).** Until the rules are close to final, scoping runs use
shallow agents and short runs ([principles.md](principles.md)): `--quick` keeps only shallow agents
(random, greedy and engine at their defaults, and the planner; beam, rollout,
`samples`/`depth` > 1 and `engine:width` > 0 are dropped). The only sign is one stderr
line (`--quick: dropped deep-search agents ...`), and the report's cells just lack them, so
an `engine:samples=2` you meant to compare is easy to lose. It caps seeds at 16 per cell unless
`--games` is given, and turns the probes off unless `--winprob`/`--closeness` is given.

## Content levers

**Content levers run on one binary (040).** A variant's `set` (or `--set` on `run` and
`compare`) takes a content overlay, `<table>.<id>.<path>=<JSON>`: tables `cards`, `threats`, `adversary`, `ais`,
`alignment`, `projects`; `*` for every id; a path through fields and list indexes.
`cards.telemetry_archive.cost=3`, `cards.turnkey_factory.effects=[]`,
`cards.turnkey_factory.passives.0.amount=2`, `ais.*.presence.4.cost=2`,
`threats.<id>.counter=[["credits",3]]`. It goes through the same parsing and checks as
`content/`, so a typo or an illegal op is an error naming the entry. The overlay lands in
the config (`content`), so the config hash, `content_hash` and every saved record name
it and `aic-sim replay` needs nothing else. `experiments/overlay-card-cost.json` is a
worked sweep. The map is out of reach. Only player-card text is generated, so a threat's,
project's, adversary or Alignment card's text keeps its printed numbers, and the viewer
reads the printed content (`wasm.content()`): faces, adversary rows and power labels of
an overlaid record show the printed values, though the game plays the overlay.

A spec has no set shared by its variants and `experiment` refuses `--set`, so a lever on two
sizes is two variants, each repeating the whole list (`c2x`: the lever; `c3x`: `countries=3` and
the lever; `experiments/archive/screen-041-tune.json`).

**Bisect a batch, first read on the engine.** When a batch of card changes moves a win rate, bisect
it by groups as overlays on one binary. A first read of a price lever goes on the engine: about 1.4 s
a game at c2 and 3.9 s at c3 at r26 against the planner's 21 and 70 s, so 400 games take minutes. Size a sweep from `ms_per_game` in a first screen's `results.json`, not from this line: 068's first rules ran the engine at 4.4 s and 20 s. The planner
confirms: an engine read is a direction only. 064's `project_discord_per` moved the engine 4 points
and changed no planner game at c2.

## Reading noise

**Seeds are noisy.** 24 seeds is about ±10: the same rules read 60 / 75 then 50 / 69 at 068. **At c3:** The same r22 rules at c3 d2 read 56 / 38 / 71 on 16 / 16 / 14 seeds
(33 on the first 12 of the 38's): about ±15 on any one set. A 16-seed calibration can't separate levers under about
20 points. Compare on the same seeds, confirm on fresh ones.

`cards.csv` is split by agent, variant, config and difficulty (columns `agent,variant,config,difficulty,card,...`), so per-lever card numbers no longer need a spec per variant. The card table in `report.md` and `results.json` still pools per agent. `kept` and `drawn` in `games.jsonl` include the opening deal and `offered` in `cards.csv` doesn't, so kept ÷ offered passes 100%: use share of games (055).

Don't read an agent's keeps as a human's. Up to r3 (007, 009) agents kept nearly every
drafted card, so a keep-priced lever measured the agent; 022 and 034 made them
keep-aware, and at r19 greedy, engine and planner keep 58-69% (d0, d2, 8 seeds a cell). It
is still their plan, not a table's.

## The band and the gate

**The band and gate at r29 (070).** On the fast profile at 8 threads, c2 and c3 run about 24 planner games a minute together (48 games, 117 s): a 96-seed band is 8 min. Content overlays reach a threat's `source` (`threats.manhunt.source="cyber"`) and any card number, so a what-if needs no build; a rule what-if does. **Copy the binary before a run, then rebuild freely.** `npm run e2e` has a sibling for threat sources: `npm run e2e:sources`.

Gate-070: 1,403 of 2,976 games, 48 min on 8 threads; closes 19 rows, 73 min, c3 45 min of it. A 96-seed band took 9 min, and two fresh ones on the same frozen build read 61 / 57 and 46 / 46: one band can sit 10 points off, so read a pool before calling a miss; a planner game runs about 20 s at c2 and 26 s at c3 on 8 threads at r29.

`experiments/band.json` (name `band`) places a win rate: `--seed S --games 96` (Matt: two sets of 48), planner and engine. `experiments/gate.json` is gate-070: 30 checks and the planner × c2 and c3, early-stopped from 12 seeds at 2 SE, capped at 48. `scripts/cards/screen.py --gate DIR` reads it and prints the unspent readout; `scripts/bench/gate_close.py plan` picks the rows within 2 SE by gap against paired SE, whatever stopped them, and writes `experiments/gate-close-NNN/`: fill to 48, then the next 48 seeds with no early stop. `read` merges the runs; `--min-seeds` keeps 96-seed rows apart from early ones. Read a band's win mix too (how many by purge, how many by the Unity deck) before any rule that touches the board's end state: r28's 75 was 56% purge, and that was the whole story of 070's first band.

`experiments/sanity-line.json`: run it after a change to the rollout policy or `prune`: the line alone should read near the engine (r26, seeds 19000, 48 a cell: engine 50 / 46, `planner:alts=0` 42 / 42, paired -8 ±9, -4 ±11; 160 s on 8 threads).


**Gate early-stopping (040).** A spec with `"gate": {"z": 2.0, "min_games": 8, "every": 4}`
treats `planner:<check>` as a check of `planner` in the same variant and difficulty, and
`games` as each check's cap. Seeds go in blocks (8, then 4 at a time); after each block a
check stops if its gap clears `z` standard errors (sign test; `clears_noise` in `gate.rs`) or the seeds left can't turn
its sign back at that noise (`sign_settled`: so a stopped row can sit inside 2 SE, and isn't "clearly behind"); an agent runs until its last check stops. The verdict is the
gap's sign, as in the old `--gate` tables. Decisions come only from finished blocks, so
the games played don't depend on threads or timing, and `--resume` rebuilds them from
`games.jsonl`. A panicked game counts for neither side and fails the gate (*A game that panics*). `report.md`'s *Gate* section and `results.json`'s `gate` give each check's
games, why it stopped, and the game time against an estimate at the cap. Measured: `python
scripts/bench/gate_stop_rate.py [z]` simulates the rule; at `z` 2 a level check stops early
about half the time and about 1% of runs get the sign the cap wouldn't. `experiments/archive/gate-034.json`
is the 034 gate this way: **872 of 1,728 games, 4.2 h of game time against 7.1 h at the cap and
5.5 h for 034's hand-cut plan, 1h 4m on 4 below-normal threads**, every verdict the same
(`experiments/results/gate-034-earlystop/gate.md`).

**Budgeting a gate.** Cells = agents × variants × difficulties × configs; the first block is
`min_games` × cells and the cap `games` × cells; early stopping played about half the cap at 034.
`experiments/gate.json` (gate-070): 31 agents × 2 sizes = 62 cells, 12 seeds a check, so 744 games in the first block and 2,976 at the cap. Measured, in planner games a minute: **30.7 at c2 and 15.6 at c3 on 8 (r28, 069, `--profile fast`; the engine 258 and 124)**; earlier revisions are in [history.md](../experiments/history.md). A run's time on 8 threads is sum over cells of games × 16 s (c2) or 31 s (c3) ÷ 8, counting the planner row as a cell: gate-068, 56 check cells and 2 planner cells of 48 seeds, is 2.2 h at the cap, and at the 47% of the cap 068 played, about 50 min; closing its 18 rows to 96 seeds about 45 min. Budget 1 h for the gate (gate-070 took 48 min). Start about 2 h before a hard stop with the rules frozen.

Every 10 s stderr prints the games done and the time left at that rate. For a gate that is to the cap, so the most it can take; `report.md`'s "N of M" is the real total. Before a gate, `aic-sim experiment
SPEC --games 1 --out out/preflight` plays one seed a cell: a crash, a bad override or a dead agent
shows in minutes, not at the end of a block (its own `--out`: a run without `--resume` moves an
earlier stream aside). In a long gate, watch stderr too: `panicked` means a game failed while the
run goes on, and `^time: wall` that an `experiment` finished.

**Fresh seeds for a gate.** The specs start at seed 0, as the calibration screens do. Give the
gate a `--seed` no screen of this revision used (046 used 3000), or it confirms on the seeds
that picked the levers. The ledger, updated by any item that spends seeds, and reset at each bump
keeping the seeds that picked that revision: r28: 22000 band of a rejected build, 22100 band, 23000 gate; screens 21000-21700; 069: 24000 frozen speed seeds, 24100 lever screen, 24200 its fresh confirmation (96), 24400 `sweep` screen (96). r27 picked at: 18000-19100, 20200, 20300, 21000. r29 (070): screens 24500-24999, 25000 band, 25200 gate and closes to 25295, 25400 creation-off screen, 25500 engine row, 25600 second band. Next free: 25700.

## Reading prices

**Card pricing read (055, r24)**: `experiments/archive/screen-055-prices.json`, which today's binary refuses (`always_consolidate` went in 066): read it as a recipe (planner, tier-2 rush and
always-Consolidate, paired, c2 and c3, 160 seeds: 960 games, 63 min on 8 threads, **about 15 planner
games a minute** since 054's deals; `--games 48`: 288 games, 19 min), read with `python
scripts/cards/price_read.py DIR [--agent planner] [--csv OUT]`. It gives offered, kept and played per
game, and win deltas stratified on rounds, against the card's tier. `cards.csv` has no outcome, so it
reads games.jsonl. Results: `experiments/results/screen-055-prices/prices.md`, whose `results.json`
and `games.csv` feed the card viewer. A price sweep is a spec of overlays in `set`; paired against a
base on the same seeds a 4-lever sweep on two agents at 48 seeds is 54 min, so a price pass gets about
three sweeps in four hours; one guess (061) is about 0.8 × that, 43 min, so four. Planner games a
minute on 8 threads, one guess: 29.6 at c2, 8.8 at c3. The default since 064, four deals, runs 1.5 × as
long a game (22.7 at c2, 6.9 at c3, from `ms_per_game`, r27), so a sweep costs about what 054's did. r28 since 069, `--profile fast`:
30.7 and 15.6; `sweep` (one guess) about 44 and 24, scaled from 069's screen, so a 4-lever sweep on two agents at 48 seeds is about 26 min and a price pass gets about nine in four hours.

## Tuning sweeps

**Tuning sweeps** (`scripts/tune/`, from the repo root): `sweep.py` runs `aic-sim run --json`
once per variant and agent and prints one row each (win %, rounds, adversary operations,
corruptions and facilities and presence destroyed per game, how games ended), appending every result
to `out/tune/log.jsonl`. `log_table.py TAG...` turns logged rows into a markdown table.

```sh
python scripts/tune/sweep.py --agents "greedy;beam" --games 64 --tag try1 \
    --var "base=" --var "supply8=adversary.supply_per_country=8;cards.keep_tier2=2"
```

**Past sweeps** (007 tuning, 008 engine agent, 009 payback and its content patches,
011 proposals), their recipes and timings, and the 012 speed-up are in
[experiments/history.md](../experiments/history.md). Their write-ups are
`experiments/tuning-007.md`, `engine-008.md`, `payback-009.md` and `proposals-011.md`.

## Older specs

**Specs the binary refuses today** (the rules or configs they name are gone) are in
`experiments/archive/`, listed with why in
[experiments/history.md](../experiments/history.md). Every spec left in `experiments/`
runs on the 014 board. Committed results from r5 and earlier are history: their game
records do not replay.

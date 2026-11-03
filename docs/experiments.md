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
| `closeness` | `{"every": 0, "gap": 2.0}` | **Off unless set.** Probe every n-th real choice: 1-ply greedy scores of every option; close when the best two are within `gap` heuristic points (about credits). `--closeness N` sets `every`. |
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
Each cell in `results.json` has its `countries`, read from the resolved config.

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

**`run_sim.ps1` and stderr:** the sim writes `out/NAME.err` as it runs, so a wait on `panicked` or
a look at the 10 s progress lines works mid-run. `out/NAME.exit` marks the end.

**A game that panics.** It fails alone: the run goes on, stderr and `failed.jsonl`
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
- **Unspent resources**: each seat's credits, chips, energy, antimatter and ambassadors when its last action phase ended (`unspent.<seat>.<resource>` in
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
- **Strategy health**, per cell × agent: buildings played per game (any card of
  kind `building`, AI uniques included), tier-2 cards played per game, cards played per
  game, and the share of wins with 2 or more buildings played (and standing at the
  end). A findings line per difficulty says whether the top agent plays buildings and
  tier 2. *Top agent*: the highest win rate at the spec's highest difficulty (ties to
  the later agent).
- **Head to head against greedy**: every agent against plain `greedy` (or the
  first greedy-kind agent) on the same seeds, per cell: won by both / only the agent /
  only greedy / neither, the win-rate difference ± the standard error of the per-seed
  difference, an exact two-sided sign test on the seeds only one of them won, and a
  verdict (*beats greedy* only when ahead with p < 0.05).
- **Variants against the base** and **removing a module**, when a spec has
  several variants: each variant's win rate against the first (the base), paired on
  seeds; for variants that switch a module off (`<module>=off`,
  `<module>.variant=off`), the base minus the off-variant: what the module is worth.
- **Luck share**: a two-way split of outcome variance over seeds × agents within each
  config/variant/difficulty: the seed's share (luck), the agent's (skill), the rest
  (interaction and the agents' own randomness). Raw sums-of-squares shares, no
  degrees-of-freedom correction. On win/loss and on survival.

The per-cell split (agent, probes, total) is in each report's *Timing* table and in
`games.csv`.

**Saved records** (open in the viewer, or `aic-sim replay`), up to `records` each, with
the reason in the record's `note`: *close losses* at the top difficulty (fewest pieces
over the bar, then the longest games); *early lock-ins* (decided by round 2 in games of
3+ rounds, from the win-probability playouts); *surprising wins* (by the weakest agent,
after the playouts gave them ≤ 0.25, or below the top agent at the top difficulty).
"Weakest", "top" and "stronger first" are by **measured** strength, not ladder
tier: agents ranked by win rate at the top difficulty on the base variant (the same
reading as the strategy-health *top agent*); equal win rates share a rank, so with all
agents tied nobody is "weakest".
Files are named `<category>-c<cell>-<config>-<variant>-d<difficulty>-<agent>-seed<S>.json`;
a rerun into the same directory replaces them.

## Opt-in measures and `--quick`

**Expensive measures are opt-in: screen first, probe finalists.** Win-probability
probes cost about 5× an experiment, and closeness about one greedy decision per probed decision. Both are off
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

**Content levers run on one binary.** A variant's `set` (or `--set` on `run` and
`compare`) takes a content overlay, `<table>.<id>.<path>=<JSON>`: tables `cards`, `threats`, `adversary`, `ais`,
`alignment`, `projects`; `*` for every id; a path through fields and list indexes.
`cards.telemetry_archive.cost=3`, `cards.orbital_embassy.effects=[]`, `corps.horizon.project.after=[]` (the map's
country pieces by lowercase name, the only part of the map an overlay reaches),
`cards.orbital_embassy.passives.0.amount=2`, `ais.*.economy_track.4.cost=2`,
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
it by groups as overlays on one binary. A first read of a price lever goes on the engine, many times cheaper a game than
the planner; size a sweep from `ms_per_game` in a first screen's `results.json`. The planner confirms: an engine read is a
direction only.

## Reading noise

**Seeds are noisy.** 24 seeds is about ±10 and 48 about ±7; one 16-seed set at c3 is about ±15, so a 16-seed read can't
separate levers under about 20 points. Compare on the same seeds, confirm on fresh ones.

`cards.csv` is split by agent, variant, config and difficulty (columns `agent,variant,config,difficulty,card,...`), so per-lever card numbers need no spec per variant. The card table in `report.md` and `results.json` pools per agent. `kept` and `drawn` in `games.jsonl` include the opening deal and `offered` in `cards.csv` doesn't, so kept ÷ offered passes 100%: use share of games.

Don't read an agent's keeps as a human's: it is their plan, not a table's.

## The band and the gate

**The judge: the strategist.** Screens read with `strategist` (greedy with strategy, `engine:strategy=1`), not `sweep`:
over 96 seeds a pair it reads within a few points of the sweep at about a ninth of its time a game (`docs/speed.md`).
`sweep` and `planner` take the engine's defaults, so they too play on the value table; the judge without it is
`sweep:plan_table=0,w.table=0,w.packet_keep=1`. The sweep on the table reads about 2-8 points above the strategist: keep it
as the spot check of a verdict that matters, and `planner:strategy=1` as the ceiling. Read a screen with `python
scripts/bench/paired.py DIR` and `python scripts/cards/use_read.py DIR`. The recipe and its costs: `experiments/screen-082.json`
(eight agents, 1,152 games, 17 min on 6 threads beside another item; four of the agents are planners),
`screen-082-d0.json` and `-d4.json` (the greedy tiers at the ends of the range, about 1 min each),
`screen-082-table-bp/br/pr.json` (each strategy pairing fixed: 2,688 strategist games, about 8 min on 6 threads) and
`screen-082-fresh.json` (three agents, 7.5 min).

**The ballpark read (Matt's ruling, 2026-11-03: simulation is a ballpark check until he names a milestone).** Two players
only, rotating the board pairs (Builder + Protector, Builder + Researcher, Protector + Researcher; `ai.boards`,
`ai.start_credits` and `ai.slots` are seat-ordered, so each pair's variant restates all three), no c3, no gate; the target
is 45-85 a pair, aiming near 60-75 without chasing it. First the engine smoke read (`experiments/use-080.json`, 16-32 seeds,
about 20-30 s on 6 threads; read with `scripts/cards/economy_read.py`, `play_read.py` and `board_read.py --powers`), then
`experiments/screen-080.json` (the `sweep` planner, `sweep:always_liberate=1` and the engine, 48 seeds a pair, 432 games)
and `screen-080-off.json` (the same seeds with the liberate capstones out: `experiments/configs/no_liberate.json`, written by
`scripts/configs/no_liberate.py`; 288 games): both together about 9 min on 6 threads (about 13 ms a planner decision). Read
liberation with `python scripts/cards/liberation_read.py DIR... --cards`: cities corrupted and liberated (card / fallback)
a game, liberations per won game, each liberate card's kept / played, Grant uses, Wall stops. An engine screen of 32 seeds a
cell is about 0.7 s a game, so a 24-cell factorial is under 3 min, and the engine sits close enough under `sweep` to pick
the planner's bracket. Credits feed the purge almost directly through the gain-an-ambassador project: a size lever's first
read is how many ambassadors the seats bought (`board.std.ambassador`). Income levers are overlays (`ais.builder.credits=16`,
`corps.<name>.project.<path>`), so a what-if on printed numbers needs no build; a rule what-if, or one on the map, does.
**Copy the binary before a run, then rebuild freely.** Who keeps and plays which card: `python scripts/cards/play_read.py
DIR... [--min 1] [--agents A,B]`; board powers: `scripts/cards/board_read.py DIR... --powers --cards`.

`experiments/band.json` (name `band`) places a win rate: `--seed S --games 96` (Matt: two sets of 48), planner and engine.
One band can sit 10 points off another on the same build, so read a pool before calling a miss. Read a band's win mix too
(how many by purge, how many by the Unity deck) before any rule that touches the board's end state.
`experiments/gate.json` is a gate: checks and the planner × c2 and c3, early-stopped from 12 seeds at 2 SE, capped at 48.
`scripts/cards/screen.py --gate DIR` reads it and prints the unspent readout; `scripts/bench/gate_close.py plan` picks the
rows within 2 SE by gap against paired SE, whatever stopped them, and writes `experiments/gate-close-NNN/`: fill to 48, then
the next 48 seeds with no early stop. `read` merges the runs; `--min-seeds` keeps 96-seed rows apart from early ones
(`python scripts/bench/gate_close.py read out/gate out/gatec3 out/close-close/* out/close-closec3/* --min-seeds 96`).

`experiments/sanity-line.json`: run it after a change to the rollout policy or `prune`: the line alone should read near the
engine.

**Gate early-stopping.** A spec with `"gate": {"z": 2.0, "min_games": 8, "every": 4}`
treats `planner:<check>` as a check of `planner` in the same variant and difficulty, and
`games` as each check's cap. Seeds go in blocks (8, then 4 at a time); after each block a
check stops if its gap clears `z` standard errors (sign test; `clears_noise` in `gate.rs`) or the seeds left can't turn
its sign back at that noise (`sign_settled`: so a stopped row can sit inside 2 SE, and isn't "clearly behind"); an agent
runs until its last check stops. The verdict is the gap's sign. Decisions come only from finished blocks, so
the games played don't depend on threads or timing, and `--resume` rebuilds them from
`games.jsonl`. A panicked game counts for neither side and fails the gate (*A game that panics*). `report.md`'s *Gate*
section and `results.json`'s `gate` give each check's games, why it stopped, and the game time against an estimate at the
cap. `python scripts/bench/gate_stop_rate.py [z]` simulates the rule; at `z` 2 a level check stops early about half the
time and about 1% of runs get the sign the cap wouldn't.

**Budgeting a gate.** Cells = agents × variants × difficulties × configs; the first block is `min_games` × cells and the
cap `games` × cells; early stopping plays about half the cap. A run's time is the sum over cells of games × a game's time
(from a first screen's `ms_per_game`) ÷ threads, counting the planner row as a cell. The last gate run (r32, 8 threads)
took 2 h 17 min with its closes: budget 2.5 h and start it 3 h before a hard stop with the rules frozen.

Every 10 s stderr prints the games done and the time left at that rate. For a gate that is to the cap, so the most it can take; `report.md`'s "N of M" is the real total. Before a gate, `aic-sim experiment
SPEC --games 1 --out out/preflight` plays one seed a cell: a crash, a bad override or a dead agent
shows in minutes, not at the end of a block (its own `--out`: a run without `--resume` moves an
earlier stream aside). In a long gate, watch stderr too: `panicked` means a game failed while the
run goes on, and `^time: wall` that an `experiment` finished.

**Fresh seeds for a gate.** The specs start at seed 0, as the calibration screens do. Give the
gate a `--seed` no screen of this revision used, or it confirms on the seeds
that picked the levers. The ledger, updated by any item that spends seeds, and reset at each bump
keeping the seeds that picked that revision: r34: 30300 the planner reads (080), 30400 the engine smoke and pace screen,
30500 the fresh confirmation, 30600 the 2x2 after the freeze (`screen-080-next.json`); 30700 the 082 screens, 30800 their
fresh confirmation, 31000 082's antimatter read. Next free: 31100.

## Reading prices

`python scripts/cards/price_read.py DIR [--agent planner] [--csv OUT]` gives offered, kept and played per game, and win
deltas stratified on rounds, against the card's tier; `cards.csv` has no outcome, so it reads games.jsonl. A price sweep is
a spec of overlays in `set`, paired against a base on the same seeds. How prices are set: [card-pricing.md](card-pricing.md).

## Tuning sweeps

**Tuning sweeps** (`scripts/tune/`, from the repo root): `sweep.py` runs `aic-sim run --json`
once per variant and agent and prints one row each (win %, rounds, adversary operations,
corruptions and facilities and presence destroyed per game, how games ended), appending every result
to `out/tune/log.jsonl`. `log_table.py TAG...` turns logged rows into a markdown table.

```sh
python scripts/tune/sweep.py --agents "greedy;beam" --games 64 --tag try1 \
    --var "base=" --var "supply8=adversary.supply_per_country=8;cards.keep_tier2=2"
```

## Past runs

Every item's write-up, by item, is indexed in [experiments/history.md](../experiments/history.md). Specs the binary refuses
today are in `experiments/archive/`; committed results from r5 and earlier do not replay.

# Experiments

```sh
S=target/release/aic-sim
$S experiment experiments/smoke.json --out out/smoke    # a quick check (6 seeds)
$S experiment experiments/ladder-v0.json --games 20 --seed 1000 --threads 6
$S experiment experiments/archive/screen-014-board.json --quick           # scoping
$S experiment experiments/archive/screen-014-board.json --quick --resume  # the flags it started with (--out, --games, --seed)
```

## The spec

Configs × variants × difficulties × agents over a seed range, on every core; results never depend on the thread count
and every game is replay-checked. Spec paths (`experiments/*.json`) resolve against the working directory, so a spec
not yet copied into a second checkout is refused.

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
| `gate` | none | `{"z", "min_games", "every"}`: see *Gate early-stopping*. |
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
cells, so a stopped run has every cell's first seeds. `--resume` (same spec, same `--out`) replays the stream, plays
only the missing games and refuses a changed spec, config file, engine, rules revision or content. Without `--resume`
an earlier stream is moved to `games.jsonl.bak`. A partial report that can't be written (`games.csv` open in a
spreadsheet locks it on Windows) is skipped with a warning.

- `report.md` and `results.json` exist from the start: wait on `^time:` in stderr or on the process, not on the file.
- `run_sim.ps1` writes `out/NAME.err` as it runs (a wait on `panicked` or a look at the 10 s progress lines works
  mid-run); `out/NAME.exit` marks the end.

**A game that panics** fails alone: stderr and `failed.jsonl` name its seed, config, overrides and panic, `report.md`
lists it under *Failed games* with the line that replays it, `results.json` has `games_failed`, no measure counts it,
and the run exits 1. `--resume` plays it again. `aic-sim run` and `compare` have no such net.

## Measures

- **Skill gap**: top tier's win rate minus the bottom's (each tier by its best agent, so a budget sweep of one kind is
  one rung), also on **survival**: a loss's round; a win counts its rounds plus adversary cards left plus 2, so any win
  ranks above any loss. *Ordered*: no tier below the one under it.
- **Pick position** is the card signal (every dealt card is drafted, so how early it goes); played is per card gained.
- **Economy buildings standing**: passive `income` or `income_per_tag` at the end, overlay included; token stores and
  `convert_upkeep` alone don't count.
- **Early lock-in**: p ≤ 0.05 or ≥ 0.95 from round 2 to the end; uncertain checkpoints are 0.1 < p < 0.9.
- **Unspent resources**: at each seat's last action phase (`unspent.<seat>.<resource>` in `per_round`); the flag
  thresholds are `UNSPENT_HIGH` in `experiment.rs` and `scripts/screens.py`.
- **Top agent**: highest win rate at the spec's highest difficulty (ties to the later agent). *Beats greedy* (head to
  head against `greedy`, else the first greedy-kind agent, same seeds, exact sign test) only when ahead with p < 0.05.
- **Variants against the base** pair on seeds; a variant switching a module off (`<module>=off`,
  `<module>.variant=off`) reads as what the module is worth.
- **Luck share**: raw sums-of-squares shares of outcome variance over seeds × agents: seed (luck), agent (skill), rest.
- **Margin**: a win's adversary cards left (rounds to spare) and ambassador supply; a loss's adversary pieces over the
  bar (`margin.level`; `margin.unity_tips` the Unity tips); every game, `margin.cities_corrupted_end`, `_aligned_end`.

**Counters.** `games.csv` carries the margin columns (`games.jsonl` alone: botnets, troops, unspent) and the core's `adversary.corruptions`,
`adversary.first_corruption_round`, `board.std_actions` and `board.war_destroyed`; every whole-game counter is in
`games.jsonl`. Per-round tables (`<key>.rNN`, summed over seats) exist only for keys starting `cards.`, `ai.`,
`economy.` or `board.node`: extend the list in `experiment.rs` for another. In the core `cx.count(g, "x", n)` writes
`<module>.x` while `cx.tally` and `cx.per_round` take the full key, so a power use is `ai.power.builder.0`: a screen
column on the bare name prints zeros.

**Saved records** (`records/`, reason in `note`): close losses (fewest pieces over the bar, top difficulty), early lock-ins,
surprising wins (the weakest agent's, or at playout p ≤ 0.25). Weakest and top agent are by measured
win rate at the top difficulty on the base variant, not tier; ties share a rank (all tied: none is weakest). A rerun
into the same directory replaces them.

## Opt-in measures and `--quick`

**Screen first, probe finalists.** Win-probability probes cost about 5× an experiment, closeness about one greedy
decision per probed decision.

```sh
$S experiment experiments/x.json                                     # 1. screen: every variant, no probes
$S experiment experiments/x.json --variants keep2,uplinks \
    --winprob 10 --closeness 1 --out out/experiments/x-finalists     # 2. probe only the finalists
```

`--variants A,B` keeps those variants plus the base, always first.

**`--quick`** (scoping runs) drops deep search (beam, rollout, `samples`/`depth` > 1, `engine:width` > 0; the planner
stays), caps seeds at 16 a cell unless `--games` is given, and turns probes off unless asked. The only sign of a dropped
agent is one stderr line and a missing cell, so an `engine:samples=2` you meant to compare is easy to lose.

## Content levers

**Content levers run on one binary.** A variant's `set` (or `--set` on `run` and `compare`) takes an overlay,
`<table>.<id>.<path>=<JSON>`: tables `cards`, `threats`, `adversary`, `ais`, `alignment`, `projects`; `*` for every id;
a path through fields and list indexes (`cards.telemetry_archive.cost=3`, `cards.orbital_embassy.passives.0.amount=2`,
`ais.*.economy_track.4.cost=2`, `threats.<id>.counter=[["credits",3]]`; `corps.horizon.project.after=[]` reaches the
map's country pieces by lowercase name, the only part of the map an overlay reaches). It is checked like `content/`
and lands in the config, so the hashes and records name it and `replay` needs nothing else
(`experiments/overlay-card-cost.json` is a worked sweep). Only player-card text is generated: other text and the
viewer's faces show printed values while the game plays the overlay. `experiment` refuses `--set` and a spec has no
set shared by its variants, so a lever on two sizes is two variants, each repeating the whole list.

**Bisect a batch by groups** of overlays on one binary. A price lever's first read goes on the engine, many times
cheaper than the planner; the planner confirms (an engine read is a direction).

## Reading noise

**Seeds are noisy.** 24 seeds is about ±10 and 48 about ±7; a 16-seed set at c3 is about ±15, so it can't separate
levers under about 20 points.

`cards.csv` splits by variant; `report.md`'s card table pools per agent. `kept` and `drawn` in `games.jsonl` include the opening deal and `offered` doesn't, so kept ÷ offered passes
100%: use share of games. An agent's keeps are its plan, not a human table's.

## The band and the gate

**The judge is the strategist** (`engine:strategy=1`; why: [agents.md](agents.md), *The tiers*); `sweep` is the spot check of a verdict that matters, `planner:strategy=1` the ceiling, and
`sweep:plan_table=0,w.table=0,w.packet_keep=1` the judge off the value table. Read a screen with
`scripts/bench/paired.py DIR` and `scripts/cards/use_read.py DIR`. Recipes and costs (6 threads): `screen-082.json`
(1,152 games, 17 min beside another item), `screen-082-table-bp/br/pr.json` (2,688 games, 8 min). The r36 read is
`screen-093.json` (each r36 lever off, 720 games, 2.5 min on 3 threads), `-gl` (Great Liberation on/off), `-d0`
(greedy); older specs' seat-ordered triples need only `ai.boards=` now. `scripts/cards/slots_read.py` gives build-overs and slot fill
by round, `energy_read.py` stations freed and the gift. **A before/after read is paired** (Matt, 093): seed sets
alone move a pair 10-25 points at 48 seeds, so run the old binary from `git archive <main> configs | tar -x` on the
same seeds.

**The ballpark read (Matt's ruling, 2026-11-03: simulation is a ballpark check until he names a milestone).** Two
players only, rotating the board pairs (Builder + Protector, Builder + Researcher, Protector + Researcher; a pair's variant
sets `ai.boards`, since `ai.slots` and `ai.start_credits` are by board from r36), no c3, no gate; target
45-85 a pair, aiming near 60-75 without chasing it. First the engine smoke read (`use-090.json`, 16-32 seeds, 20-30 s
on 6 threads), then the strategist read (`screen-090.json`, 20 s). A `sweep` screen of 432 games is about 9 min
(about 13 ms a planner decision); an engine one about 0.7 s a game. Credits feed the purge almost directly through the
gain-an-ambassador project, so a size lever's first read is ambassadors bought (`board.std.ambassador`). Income levers
are overlays (`ais.builder.credits=16`, `corps.<name>.project.<path>`) and need no build; a rule or map what-if does.
**Copy the binary before a run, then rebuild freely.** The read scripts (`economy_read`, `play_read`, `board_read`,
`liberation_read`) are in [scripts.md](scripts.md).

`experiments/band.json` places a win rate: `--seed S --games 96`, planner and engine. Read a pool before calling a
miss, and its win mix (purge vs. Unity deck) before any rule touching the end state. `experiments/sanity-line.json`: after a change to the
rollout policy or `prune`, the line alone should read near the engine.

`experiments/gate.json`: checks and the planner × c2 and c3, early-stopped from 12 seeds at 2 SE, capped at 48; read
with `scripts/cards/screen.py --gate DIR`. Rows still within 2 SE, whatever stopped them, go on through
`scripts/bench/gate_close.py`: fill to 48, then the next 48 seeds with no early stop (`read ... --min-seeds 96` keeps
those rows apart from early ones).

**Gate early-stopping.** With `"gate": {"z": 2.0, "min_games": 8, "every": 4}` (defaults; `gate.json` has 12), `planner:<check>` is a check of
`planner` in the same variant and difficulty, and `games` each check's cap. After each block (8 seeds, then 4) a check
stops if its gap clears `z` SE (`clears_noise` in `gate.rs`) or the seeds left can't turn its sign back
(`sign_settled`: so a stopped row can sit inside 2 SE and isn't "clearly behind"); an agent runs until its last check
stops. The verdict is the gap's sign. Decisions come only from finished blocks, so play doesn't depend on threads; a
panicked game fails the gate. `scripts/bench/gate_stop_rate.py [z]` simulates the rule.

**Budgeting a gate.** Cells = agents × variants × difficulties × configs, the planner row counting as one; early
stopping plays about half of `games` × cells. Time is games × `ms_per_game` (from a first screen) ÷ threads. The last
gate (r32, 8 threads) took 2 h 17 min with its closes: budget 2.5 h and start it 3 h before a hard stop with the rules
frozen. The 10 s stderr line's time left is to the cap; `report.md`'s "N of M" is the real total. **Preflight** with
`aic-sim experiment SPEC --games 1 --out out/preflight` (its own `--out`, so no stream is moved aside): a crash, bad
override or dead agent shows in minutes. In a long gate, `panicked` in stderr is a failed game (the run goes on),
`^time: wall` the end.

**Fresh seeds for a gate.** Specs start at seed 0: give a gate a `--seed` no screen of this revision used, or it confirms on the seeds that picked the levers. The ledger, updated by any item that spends
seeds, and reset at each bump keeping the seeds that picked that revision: r35: 31100 the fresh d2 read
(`screen-090.json`) and the d0 read (`screen-090-d0.json`), 31200 the engine smoke (`use-090.json`), 31300 the on/off
and liberate-price screens (`screen-090-onoff.json`, `screen-090-lib.json`), 31400 and 31500 the agent
heuristics' first and final reads (`screen-092.json`, `screen-092-d0.json`). Next free: 31600.

## Reading prices

`python scripts/cards/price_read.py DIR [--agent planner] [--csv OUT]` gives offered, kept and played per game and win
deltas stratified on rounds, against the card's tier, from `games.jsonl` (`cards.csv` has no outcome). A price
sweep is a spec of overlays in `set`, paired against a base on the same seeds. How prices are set:
[card-pricing.md](card-pricing.md).

## Tuning sweeps

`scripts/tune/sweep.py` (from the repo root) runs `aic-sim run --json` once per variant and agent, prints a row each
and appends it to `out/tune/log.jsonl`; `log_table.py TAG...` makes them a markdown table.

```sh
python scripts/tune/sweep.py --agents "greedy;beam" --games 64 --tag try1 \
    --var "base=" --var "supply8=adversary.supply_per_country=8;cards.keep_tier2=2"
```

## Past runs

Write-ups are indexed in [experiments/history.md](../experiments/history.md). Specs the binary refuses are in
`experiments/archive/`; committed results from r5 and earlier do not replay.

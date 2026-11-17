# Experiments

```sh
S=target/release/aic-sim
$S experiment experiments/smoke.json --out out/smoke    # a quick check (6 seeds)
$S experiment experiments/ladder-v0.json --games 20 --seed 1000 --threads 6
$S experiment experiments/use.json --quick --resume     # scoping; resumes with its --out, --games, --seed
```

## The spec

Configs × variants × difficulties × agents over a seed range, on every core; results never depend on the thread count
and every game is replay-checked. Spec paths resolve against the working directory.

| Field | Default | Meaning |
|---|---|---|
| `name`, `note` | | Report title and purpose. |
| `configs` | | Config files. |
| `variants` | `[{"name": "base", "set": []}]` | Named override sets in `--set` syntax (lists comma-separated, `alignment.purge_at=1,1,2`), each applied to every config. |
| `difficulties` | `[0]` | `adversary.difficulty` levels. |
| `agents` | | Agent specs, weakest first. |
| `games`, `first_seed` | `first_seed` 0 | Seeds per cell. `--games`/`--seed` override. |
| `selection_seeds` | none | `{"first": N, "games": M}`: the seeds that picked a winner. Adds *Fresh-seed confirmation*; `--resume` refuses a stream started without it. |
| `max_actions` | `5000` | Per-game cap (`timeout`); `--max-actions` overrides. |
| `winprob` | `{"games": 0, "rollouts": 8, "agent": "greedy"}` | **Off unless set.** Win probability: `rollouts` playouts by `agent` at each round's start, on a cell's first `games` seeds. |
| `closeness` | `{"every": 0, "gap": 2.0}` | **Off unless set.** Every n-th real choice, close when the best two 1-ply greedy scores are within `gap` (about credits). |
| `endgame` | none | `{"back": 3, "agent": "", "rollouts": 1, "all_losses": false}`: each round-cap loss's (or every loss's) end state, and `agent`'s replays from the `back`-th last round (`endgame.rs`). |
| `records` | `4` | Records saved per category. |
| `gate` | none | Early stopping (suspended: *The ballpark read*). |
| `inject_panic` | none | `{"cell", "seed"}`: that game panics, for a test. |

`experiment` refuses a config naming a parameter the binary lacks: an old binary needs its revision's configs.
Agent search settings go bare, heuristic weights take `w.` (`engine:samples=2`, `engine:w.packet_keep=0.5`); an
unprefixed weight is an error.

## Output

`--out` (default `out/experiments/<name>/`) holds `report.md`, `results.json` (`aic-experiment/2`), `games.csv` (a row
a game), `cards.csv` (per agent, variant, config, difficulty and card), `records/` and `games.jsonl` (`aic-games/2`).

**Paired reads.** *Agents paired, per variant* (`agent_pairs`) and *Fresh-seed confirmation* (`fresh_confirmation`,
the seeds outside `selection_seeds`): `report.md` says how each pairs.

**Results as they go** (`--report-every`, `--resume`: `aic-sim` help). `games.jsonl` lines carry the actions.
`--resume` needs the same spec and `--out` and refuses any change; without it an earlier
stream moves to `games.jsonl.bak`. `report.md` and `results.json` exist from the start: wait on `^time:` in stderr
or on the process, not on the file.

**A game that panics** fails alone, exit 1 (`report.md`'s *Failed games* says the rest); `run` and `compare` have no
such net.

## Measures

- **Skill gap, survival, top agent, early lock-in, variants against the base**: `report.md` defines each where it
  prints it. *Ordered*: no tier below the one under it. *Beats greedy*: ahead of the first greedy-kind agent on the
  same seeds, exact sign test p < 0.05. A module switched off reads as its worth.
- **Pick position** (how early a dealt card is drafted) is the card signal; played is per card gained.
- **Economy buildings standing**: passive `income` or `income_per_tag` at the end, overlay included.
- **Unspent resources**: at each seat's last action phase (flags at `UNSPENT_HIGH`; *Counters*). A won game ends
  mid-round holding the last upkeep's income, so `margin.unspent` tracks late income: read credits held as
  `margin.held_after_actions[seat][0]` (after the last *completed* action phase).
- **The limit**: `limit.keep_short.insight` / `limit.keep_passed` per drafted card unkept; `limit.play.short.<r>`,
  `.needs_tags`, `.held` per card left in hand as the action phase ends (`bench/resource_use.py NEW OLD --limits`).
  `cards.unofferable.<id>`: a kept card left in hand at the end with no feasible part.
- **Runaway check**: a resource's peak per seat (`peaks`) far above a round's income (`gained / rounds`):
  `cards/runaway_read.py`. `sources` / `uses` / `keeps`: `source_read.py`.
- **Margin**: a win's adversary cards left and ambassador supply; a loss's adversary pieces over the bar
  (`margin.level`: the Unity level, not the difficulty); every game, `margin.cities_corrupted_end`, `_aligned_end`.
  A `deck_empty` loss (the round cap) counts `rounds` one past the cap's printed count, so a level's mean rounds run
  over the rounds played by its round-cap share.

**Counters.** `games.csv` carries the margin columns and a few counters (its header; `adversary.corruptions` counts
any site, cities alone `adversary.corruptions.city`); `games.jsonl` every whole-game counter, and a per-round stat (`cx.per_round`, summed over
seats) as `<key>.rNN` only if `PER_ROUND_EXPORT` (experiment.rs) lists its prefix. `unspent.<seat>.<resource>` is
held as the action phase ends, before production and upkeep, research cubes too, never insight
(`economy.insight_unspent`, all seats); `cards.insight_bound|short.<seat>` mark a seat-round held for want of
insight. A zero fills every round up to the key's last record: an absent `unspent.*.rNN` means that action phase
never ended. As a seat passes: `actions.skipped|blocked[_energy].<seat>.<card>` (a building's Action offered and not
taken, or not offered: used, unaffordable or no target; `_energy` sums energy held),
`energy_cards.held|short.<seat>[.<card>]`, `slots.*`.

**Saved records** (`records/`, each `note` saying why): close losses at the top difficulty, early lock-ins,
surprising wins (the weakest agent's, or at playout p ≤ 0.25).

## Opt-in measures and `--quick`

**Screen first, probe finalists** (flags: `aic-sim` help): `--variants keep2,uplinks --winprob 10 --closeness 1`.
Win-probability probes cost about 5× an experiment, closeness one greedy decision a probe. **`--quick`**'s deep
search is beam, rollout, `samples`/`depth` > 1 and `engine:width` > 0; the planner stays.

## Content levers

**Content levers run on one binary** (a rule or map what-if needs a build). A variant's `set` (or `--set` on `run`
and `compare`) takes an overlay, `<table>.<id>.<path>=<JSON>`: tables `cards`, `threats`, `adversary`, `ais`,
`alignment`, `projects`; `*` for every id; a path through fields and list indexes (`ais.*.economy_track.4.cost=2`;
`corps.horizon.project.cubes=8`: country pieces by lowercase name; `null` clears an optional field; an op's `amount`
can't be 0: rewrite its list without it). It is checked like `content/` and lands in the config, so `replay` needs
nothing else (a sweep: `overlay-card-cost.json`). Each variant repeats its whole list. **A card blanked**,
`cards.<id>.cost=99` (never affordable, so never kept or played), keeps the deck order, so its games pair with the
base's seed for seed: a card's worth to an agent, on against blank (246's `cardvalue.py`). A deck change's
old arm is the old binary on the same spec (256's `merge.py`). `null` drops the field in
both the engine and `apply_overlay.py`. Diagnose a rewrite's gap by
part before tuning: restore the old content a part at a time (a side, a tier, a card group) as overlays (a density
or dial screen: 227's `density_specs.py`). A standard action off: `board.std_off=N,M,` (places in the economy.md
table's code order, `STANDARD`, as the param's doc lists them: a spec from before 255 means other actions;
`board.sell_credits=0` takes Sell); 248's `mkspec.py` and `probe.py` write and read
the arms (4,608 strategist games farmed in 508 s, 30 native and 18 wasm shards).

## Reading noise

**Seeds are noisy.** Per cell (one pair, one level): 16 seeds is about ±15, 24 about ±10, 48 about ±7; seed sets
alone move a pair 10-25 points at 48. A screen's SE is about 50/√n points over n paired games: size it so its SE is
at most half the effect it must resolve.

`cards.csv` splits by variant and names agents as resolved (the strategist as its `engine:` spec;
`games.jsonl` as the spec writes them). `kept` and `drawn` include the opening deal, `offered` doesn't: use share
of games. A record's `cell` is an index, variant-major (each variant's levels × agents): `cells_of` in `scripts/screens.py` expands a spec,
`cells_for` reads a run's.

## The ballpark read

The ruling and its targets: [principles.md](principles.md), *Ballpark simulation*. A board pair's variant sets
`ai.boards`, as `ai.slots` and `ai.start_credits` are by board. Board balance (each pair's gap to the pair mean,
paired): `bench/ladder.py NEW OLD --pairs`; a farmed file without a header reads as `FILE=SPEC`. A power lever is a
content overlay in `set`, by index in `content/ais.json`: `ais.builder.powers.6.cost=[["chips",1]]` (s240).

**The judge is the strategist** (`STRATEGIST` in `agents/mod.rs`; [agents.md](agents.md), *The tiers*); `sweep` spot-checks a verdict that matters, `planner:strategy=1` is the
ceiling, `sweep:plan_table=0,w.table=0,w.packet_keep=1` the judge off the value table.

- **Order:** the engine smoke read (`use.json`, 16-32 seeds, 20-30 s on 6 threads), then the strategist read;
  greedy first after a change to how the game is won (agents.md, *The tiers*): a greedy lead stops calibration for a question.
- **Recipe:** the new side farmed (d0-d4) from a spec in `experiments/farm/`, the old side natively (or both:
  `bench/read_specs.py read`, `run_read.sh`); read by `bench/ladder.py NEW OLD --measures` (pooled, paired, by set).
- **Standing measures, old against new every read** (`--measures`; the list: `ladder.py`'s docstring; projects
  revealed is `adversary.projects_revealed`, every draw: a game's deck use; compute by source is
  `adversary.compute_income`, `adversary.compute_botnets` and `board.station_compute`; the rest of
  `adversary.compute_made` is threats and cards: `adversary/projects.rs`). A measure the old binary never tallied
  is patched into its tree before its first read.
- **Costs (r73):** a 720-game strategist or greedy set (d0-d4) about 2 min on 6 threads; double on a busy box.
  r115 (254): greedy + judge, two arms, three pairs, d0-d4, 48 seeds (2,880 games) farmed in 13-14 min with one
  browser worker (4,320 games, 17 min); the full oracle at d2-d3 (253), 72 games in 212 s on 6 native shards.
- **A before/after read is paired**: the old binary and its configs (README, *This machine*), same seeds, own folder.
  Copy it before the first edit, or build `git archive REV | tar -x -C out/rREV` (own `CARGO_TARGET_DIR`); new
  sim-only measures go in as `patch -p1` there (`git apply` in a worktree silently applies nothing). The old side
  runs on its folder's configs: `aic-sim experiment SPEC --seed F`, or `bench/shards.sh` (768 games in about 3.5
  min on 6 threads).
- **Farming a read:** `experiments/farm/read.json` is the read as one spec; what `farm-seeds` builds, what to commit
  first and reading its output: [simulator.md](simulator.md), *The farm*. Until it returns edit neither `content/`
  nor `configs/` (compiled in, and read from the tree at run time), nor run the release binary while it builds. The
  exe is built once, at launch (`native_build` in conductor.toml): scripts and Rust source may change meanwhile. 720
  games farm in 75-240 s with the laptop joined (1,152 in 260 s; that old side on 5 local shards, 6 min). If the
  broker goes away mid-job (logged only after the first shard), shards run one at a time on one thread: stop it,
  keep the finished 2-seed shards (`<first seed>.out` in `C:\Code\aicomplete-control\.conductor\remote\jobs\<job>\shards\`;
  `job.json` names the out file), play the missing ranges natively as 2-seed shards, `screens.py assemble` all in
  seed order.
- **Same games:** `experiments/same-noplanner.json` (its `note`; same.json: [speed.md](speed.md)), ~45 s a side on 6 threads.
- **Copy the binary before a run** (README, *This machine*); **preflight** a long one (`SPEC --games 1 --out
  out/preflight`). Readers: [scripts.md](scripts.md).

**Skilled rows.** `bench/skilled_rows.py READ.json` writes the oracle's companion spec (its docstring): launch it
with the read. At r109 an oracle game (`tries=3`, one thread, a loaded box) took about 1.4 / 2.5 / 4.6 / 15 / 150 s
at d0-d4; restarts cost only where the first try loses (1.15x at d3, 1.8x at d4); the slowest game bounds the wall
time. The real oracle (`--rows real`, 249) took 170-250 s a game at every level (r112, loaded box): ~135× the
judge; a 4-seed, three-pair d0-d4 read beside the judge and the oracle ran an hour on 4 threads. A card proof is the oracle with `prove=N` (N: the card's row of `aic-sim cards`), seconds a game where it
proves the card and a whole search where it can't: 243's `proofs.py`.

`experiments/band.json` and `sanity-line.json`: their `note`s. Read band's win mix (the purge by Unity step,
`ends156.py`) before a rule touching the end state.

**Fresh seeds.** Specs start at seed 0: give a confirming read a `--seed` no screen of this revision used. An item
that spends seeds updates the ledger after its last read (`bench/seeds.py next N [--take]`; all below *Next free*
are used). Next free: 49212. **The dev seeds are 36800-36831**, for on/off
checks and crash-finding (SE ~5 a cell). A 2-8 point effect (a level's income step): screen it paired on this
revision's spent fresh sets (~500+ games a cell, SE ~2), then confirm once on new seeds.

## Reading prices

`cards/price_read.py DIR`: offered, kept and played per game, win deltas stratified on rounds (`cards.csv` has no
outcome). A price sweep is overlays in `set`, paired against a base on the same seeds. How prices are set:
[card-pricing.md](card-pricing.md).

## Past runs

Write-ups: [experiments/history.md](../experiments/history.md). Specs the binary refuses go to
`experiments/archive/` (a fast-lane test parses the rest). The older tuning drivers (`tune/sweep.py`,
`log_table.py`) say what they do in their docstrings.

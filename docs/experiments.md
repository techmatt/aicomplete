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
| `winprob` | `{"games": 0, "rollouts": 8, "agent": "greedy"}` | **Off unless set.** Win probability: `rollouts` playouts by `agent` at each round's start, on a cell's first `games` seeds (`--winprob N`). |
| `closeness` | `{"every": 0, "gap": 2.0}` | **Off unless set.** Every n-th real choice (`--closeness N`), close when the best two 1-ply greedy scores are within `gap` (about credits). |
| `endgame` | none | `{"back": 3, "agent": "", "rollouts": 1, "all_losses": false}`: each round-cap loss's (or every loss's) end state, and `agent`'s replays from the `back`-th last round (`endgame.rs`). |
| `records` | `4` | Records saved per category. |
| `gate` | none | Early stopping (suspended: *The ballpark read*). |
| `inject_panic` | none | `{"cell", "seed"}`: that game panics, for a test. |

`experiment` refuses a config naming a parameter the binary lacks, so an old binary needs its revision's configs.
Agent search settings go bare, heuristic weights take `w.` (`engine:samples=2`, `engine:w.packet_keep=0.5`); an
unprefixed weight is an error.

## Output

`--out` (default `out/experiments/<name>/`) holds `report.md`, `results.json` (`aic-experiment/2`), `games.csv` (a row
a game), `cards.csv` (per agent, variant, config, difficulty and card), `records/` and `games.jsonl` (`aic-games/2`).

**Paired reads.** *Agents paired, per variant* (`agent_pairs`): A against B on the same seeds; `x:...` pairs with `x`
if listed, a bare agent with the next bare agent. *Fresh-seed confirmation* (`fresh_confirmation`): each variant
against the base on the seeds outside `selection_seeds` alone, never pooled.

**Results as they go.** Each finished game is appended to `games.jsonl` (actions included); the report is rewritten
every `--report-every` seconds (60), marked **Partial: N of M games**. `--resume` (same spec and `--out`) plays only
the missing games and refuses any change; without it an earlier stream moves to `games.jsonl.bak`. `report.md` and
`results.json` exist from the start: wait on `^time:` in stderr or on the process, not on the file.

**A game that panics** fails alone (`failed.jsonl`, *Failed games* in `report.md` with its replay line; no measure
counts it; exit 1); `--resume` replays it. `run` and `compare` have no such net.

## Measures

- **Skill gap**: top tier's win rate minus the bottom's (each tier by its best agent), also on **survival**: a loss's
  round; a win's rounds plus adversary cards left plus 2, so any win ranks above any loss. *Ordered*: no tier
  below the one under it.
- **Pick position** (how early a dealt card is drafted) is the card signal; played is per card gained.
- **Economy buildings standing**: passive `income` or `income_per_tag` at the end, overlay included.
- **Early lock-in**: p ≤ 0.05 or ≥ 0.95 from round 2 to the end; uncertain checkpoints are 0.1 < p < 0.9.
- **Unspent resources**: at each seat's last action phase (`unspent.<seat>.<resource>`; flags at `UNSPENT_HIGH`).
  A won game ends mid-round holding the last upkeep's income, so `margin.unspent` tracks late income: read credits
  held as `margin.held_after_actions[seat][0]` (after the last *completed* action phase).
- **The limit**: `limit.keep_short.insight` / `limit.keep_passed` per drafted card unkept; `limit.play.short.<r>`,
  `.needs_tags`, `.held` per card left in hand as the action phase ends. Read: 189's `resource_use.py NEW OLD`.
- **Runaway check**: a resource's peak per seat (`peaks`) far above a round's income (`gained / rounds`):
  `scripts/cards/runaway_read.py`. `sources` / `uses` / `keeps`: `source_read.py`.
- **Top agent**: highest win rate at the top difficulty (ties to the later). *Beats greedy*: ahead of the first
  greedy-kind agent on the same seeds, exact sign test p < 0.05.
- **Variants against the base** pair on seeds; a module switched off reads as its worth.
- **Margin**: a win's adversary cards left and ambassador supply; a loss's adversary pieces over the bar
  (`margin.level`: the Unity level, not the difficulty); every game, `margin.cities_corrupted_end`, `_aligned_end`.
  A `deck_empty` loss (the round cap) counts `rounds` one past the cap's printed count.

**Counters.** `games.csv` carries the margin columns, `adversary.corruptions` and
`adversary.first_corruption_round` (any site; cities alone: `adversary.corruptions.city`), `board.std_actions` and
`board.war_destroyed`; `games.jsonl` has every whole-game counter, and a per-round stat (`cx.per_round`, summed
over seats) as `<key>.rNN` only if `PER_ROUND_EXPORT` (experiment.rs) lists its prefix. `unspent.<seat>.<resource>` is held as the action phase ends, before production and
upkeep, research cubes too, never insight (`economy.insight_unspent`, all seats); `cards.insight_bound|short.<seat>`
mark a seat-round held for want of insight. A zero fills every round up to the key's last record, so an absent
`unspent.*.rNN` means that action phase never ended. As a seat passes: `actions.skipped|blocked[_energy].<seat>.<card>`
(a building's Action offered and not taken, or not offered: used, unaffordable or no target; `_energy` sums energy
held), `energy_cards.held|short.<seat>[.<card>]`, `slots.*`.

**Saved records** (`records/`, reason in `note`): close losses (fewest pieces over the bar, top difficulty), early
lock-ins, surprising wins (the weakest agent's, or at playout p ≤ 0.25).

## Opt-in measures and `--quick`

**Screen first, probe finalists.** Win-probability probes cost about 5× an experiment, closeness one greedy decision
a probe: `--variants keep2,uplinks --winprob 10 --closeness 1` (those variants plus the base). **`--quick`** drops deep
search (beam, rollout, `samples`/`depth` > 1, `engine:width` > 0; the planner stays), caps seeds at 16 a cell unless
`--games` is given, and drops probes not asked for.

## Content levers

**Content levers run on one binary** (a rule or map what-if needs a build). A variant's `set` (or `--set` on `run` and `compare`) takes an overlay,
`<table>.<id>.<path>=<JSON>`: tables `cards`, `threats`, `adversary`, `ais`, `alignment`, `projects`; `*` for every id;
a path through fields and list indexes (`ais.*.economy_track.4.cost=2`; `corps.horizon.project.cubes=8`: country
pieces by lowercase name; `null` clears an optional field; an op's `amount` can't be 0: rewrite its list without it).
It is checked like `content/` and lands in the config, so `replay` needs nothing else
(a sweep: `overlay-card-cost.json`). Each variant repeats its whole list. Diagnose a rewrite's gap by part before
tuning: restore the old content a part at a time (a side, a tier, a card group) as overlays.

## Reading noise

**Seeds are noisy.** Per cell (one pair, one level): 16 seeds is about ±15, 24 about ±10, 48 about ±7; seed sets
alone move a pair 10-25 points at 48. A screen's SE is about 50/√n points over n paired games: size it so its SE is
at most half the effect it must resolve.

`cards.csv` splits by variant and names agents as resolved (the strategist as `engine:strategy=1,w.built=4,…`; `games.jsonl` as the spec writes
them). `kept` and `drawn` include the opening deal and `offered` doesn't: use share of games.

## The ballpark read

The ruling and its targets: [principles.md](principles.md), *Ballpark simulation*. A board pair's variant sets
`ai.boards`, as `ai.slots` and `ai.start_credits` are by board. Gates are suspended.

**The judge is the strategist** (the alias for `engine:strategy=1` plus its weights, `STRATEGIST` in `agents/mod.rs`; [agents.md](agents.md), *The tiers*); `sweep` spot-checks a
verdict that matters, `planner:strategy=1` is the ceiling, `sweep:plan_table=0,w.table=0,w.packet_keep=1` the judge
off the value table.

- **Order:** the engine smoke read (`use.json`, 16-32 seeds, 20-30 s on 6 threads), then the strategist read; after
  a change to how the game is won, greedy first: a greedy lead stops calibration for a question.
- **Recipe:** the new side farmed (d0-d4) from a spec in `experiments/farm/` (*Farming a read*), the old side
  natively; read by `scripts/bench/ladder.py NEW OLD --measures` (pooled, paired, by set).
  `scripts/bench/read_specs.py read` and `run_read.sh` are the native route.
- **Standing measures, old against new every read** (`--measures`): corruption node losses and adversary ambassadors
  placed a game; per level, projects revealed (`adversary.projects_revealed`) and resolved as a share of the deck,
  distinct projects revealed, reshuffles; by board, slack beside bound seat-rounds, land tokens a round and node
  placement spread. A measure the old binary never tallied is patched into its tree before its first read.
- **Costs (r73):** a 720-game strategist or greedy set (d0-d4) about 2 min on 6 threads; double on a busy box.
- **A before/after read is paired**: the old binary and its configs (README, *This machine*), same seeds, own folder.
  Copy it before the first edit, or build `git archive REV | tar -x -C out/rREV` (own `CARGO_TARGET_DIR`); new
  sim-only measures go in as `patch -p1` there (`git apply` in a worktree silently applies nothing).
- **Farming a read:** `experiments/farm/read.json` is the read as one spec. `farm-seeds` builds the native exe from
  the working tree and the browsers' bundle from HEAD ([simulator.md](simulator.md), *The farm*), and content is
  compiled into both: commit the spec, code and content first, then edit neither `content/` nor `configs/` (read
  from the tree at run time) until it returns, nor run the release binary while it builds. A 720-game set farms in
  75-240 s with the laptop joined; if the broker goes away (logged only after the first shard), shards run one at a
  time on one thread: stop it and run natively (`--threads 6`: 2880 games in 4 min). The old side runs on its
  folder's configs (`aic-sim experiment SPEC --seed F`, or shards: `aic-sim farm read --seed S --games 8`).
  `ladder.py` reads the farm's out file as `OUT=SPEC.json`; shards, after `python scripts/screens.py assemble SPEC
  FIRST COUNT OUT_DIR SHARD...`.
- **Same games:** `experiments/same-noplanner.json` is same.json ([speed.md](speed.md)) with greedy for the planner,
  for a refactor not reaching the planner or oracle (~2 min a side).
- **Copy the binary before a run, then rebuild freely.** **Preflight** a long run (`SPEC --games 1 --out
  out/preflight`). Readers (`bench/paired.py DIR`, `cards/use_read.py DIR`):
  [scripts.md](scripts.md).

**Skilled rows.** `python scripts/bench/skilled_rows.py READ.json` writes a companion spec: the `oracle` and the
strategist on the read's first 24 seeds a level, d3 and d4, the end-game probe on every loss; about 20 min on 3
threads (r73): launch it with the read.

`experiments/band.json` places a win rate (`--seed S --games 96`, the planner at 2 and 3 countries); read its win
mix (the purge by Unity step, `ends156.py`) before a rule touching the end state. `sanity-line.json`: after a
rollout policy or `prune` change, the line alone reads near the engine.

**Fresh seeds.** Specs start at seed 0: give a confirming read a `--seed` no screen of this revision used. An item
that spends seeds updates the ledger after its last read: `python scripts/bench/seeds.py next N` prints the next N,
`--take` moves *Next free* (all below it used) past them. Next free: 46784. **The dev seeds are
36800-36831**, for on/off checks and crash-finding (SE ~5 a cell). A 2-8 point effect (a level's income step):
screen it paired on this revision's spent fresh sets (~500+ games a cell, SE ~2), then confirm once on new seeds.

## Reading prices

`python scripts/cards/price_read.py DIR` gives offered, kept and played per game and win deltas stratified on rounds
(`cards.csv` has no outcome). A price sweep is overlays in `set`, paired against a base on the
same seeds. How prices are set: [card-pricing.md](card-pricing.md).

## Past runs

Write-ups: [experiments/history.md](../experiments/history.md). Specs the binary refuses go to
`experiments/archive/` (a fast-lane test parses the rest). `scripts/tune/sweep.py` runs `aic-sim run
--json` per variant and agent into `out/tune/log.jsonl`; `log_table.py TAG...` tabulates it.

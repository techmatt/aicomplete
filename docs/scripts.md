# Scripts

What each script under `scripts/` does. Run from the repo root.

- `wasm-smoke.mjs`: one game through the wasm package in Node. `wasm-parity.mjs`: replays aic-sim records of each
  `configs/variants.json` variant in wasm.
- `farm/build.mjs BUNDLE`: the farm's wasm bundle. `farm/run.mjs BUNDLE SPEC FIRST COUNT OUT [--browser] [--port N]
  [--fingerprint]`: a shard on it in Node or headless Edge (`web/scripts/cdp.mjs`), each seed timed.
  `farm/compare.py A B [--ms A_ERR B_ERR]`: two engines' outputs of one range: games that differ, win %, paired
  gap, ms a game (simulator.md, *The farm*).
- `regen.py [--no-build] [--check]`: builds release, runs `drafts/adversary/check.py`, checks `content/` against
  `drafts/adversary/` (`load_adversary.py --check`), writes `RULES_REVISION` into `docs/rules/README.md`, runs `aic-sim
  docs`, `cards_doc.py`, `effect_ledger.py`, `no_liberate.py`, `no_tier2.py`; warns of a content file
  `apply_overlay.py` would refuse (not `json.dumps(indent=1)`); prints the content hash and `git status`. `--check`: the checks and the revision line only.
- `design_check.py [DIR] [--old DIR] [--base REV] [--config FILE] [--quiet] [--usage] [--variant V,...]`: the design
  warnings ([principles.md](principles.md)): content checks, plus play checks on DIR, a read's `games.jsonl` or a
  farm's out file as FILE=SPEC (`--old` the previous revision's read on its seeds, `--base` the dials' start, default
  the merge base with main; `--variant` only those variants' games, or a lever's `PAIR-LEVER`/`PAIR_LEVER` cells,
  `base` the bare pairs: a screen's frozen cell, no split). First line: the report's `warnings:`. Thresholds and use
  bands: `design_check.json`. `--usage` prints instead each action's role, band and use. Never exits 1.
- `cards/cards_doc.py`: regenerates `docs/cards.md`. `cards/deck_text.py DECK...` prints a deck as plain text for
  review (`--list` the decks; README, *Card text for review*).
- `cards/apply_overlay.py FILE... [--set ENTRY]... [--check]`: writes overlays (`cards.<id>.<path>=JSON` lines, the
  engine `--set`'s keys; `#` comments) into `content/*.json` in place, path rules as `apply_entry` in content.rs; a
  last step `+` appends to a list (`cards.<id>.passives.+={...}`; not again if an equal value is there).
  `cards.<id>=<JSON object>` writes a whole row, new or replaced. Nothing deletes: drop a card from the baseline deck
  and `cards.json` by hand, and `tools/card-art/scenes_io.py release CARD`. `--check`: dry run. Then `regen.py`. It
  refuses hand-laid `alignment.json`; threats, adversary and projects: edit `drafts/adversary/`.
- `cards/loop_smoke.py [DIR...] [--run] [--spec S] [--games 16] [--seed S] [--threads 6] [--max-plays 12]`: the loop
  smoke (no DIR: runs `use.json` to `out/loop-smoke`): timeouts, ends, most plays of one card a game; exits 1 on a
  timeout or `--max-plays`.
- `cards/counter_names.py DIR... [--all]`: counter keys the read scripts, web scripts and `crates/aic-sim` (agents
  aside) quote that no game in DIR wrote. Rare events and test strings list too: read it, don't gate on it.
- `cards/screen.py`: named columns over experiment outputs (`--columns a,b,c`, `--preset tiers|041|034|018|016`,
  `--sources`, `--list`); `--gate DIR` (gates are suspended) each check against its own agent (paired SE), then
  unspent resources.
  `--preset tiers` (offered, kept, played by tier): a card rework's first screen. `scripts/screens.py` reads a run from Python: `load(dir)` the games with their cells
  (`stream` also takes `FILE=SPEC`), `cnt(game, key)` a counter (a power is `ai.power.<board>.<name>`:
  `scripts/powers.py`). `screens.py assemble SPEC FIRST COUNT OUT_DIR SHARD... [--exe EXE]` writes farmed shards (seed
  order) under the spec's header as `OUT_DIR/games.jsonl`.
- `cards/threat_read.py DIR... [--cards]`: per cell, threats dealt, countered by kind of option, paid abroad, typed
  counters and capstones played.
- `content/load_adversary.py [--check]`: `drafts/adversary/` (the source) into `content/` and `docs/adversary-cards.md`.
  Writes nothing if `drafts/adversary/check.py` gives any ERROR; `--check` only compares.
- `cards/play_read.py DIR... [--min PCT] [--agents A,B]`: per agent, pooled over cells and DIRs: draft cards kept in
  over `--min` percent of games (default 1) and played in none, and those no game kept. Seconds. A farmed read (no
  `results.json`) takes agents by its header spec's names. It doesn't say why: a card is offered only when some part
  of it has something to act on (`feasible` in `crates/aic-core/src/modules/effects/feasible.rs`).
- `cards/runaway_read.py DIR... [--times 5] [--floor 20] [--held 150] [--peak-times 3] [--peak-floor 10] [--top 15]`:
  loops: each trigger, threshold, reuse, discount or converter counter whose per-game max is over `--times` its mean
  where used; games whose insight grew three rounds running past `--held`; per cell, seat and resource the mean peak
  held, and games rising over `--peak-times` their income a round.
- `cards/source_read.py DIR... [--top 5] [--pool] [--resource credits,insight,energy,chips|all]`: per cell (`--pool`:
  per agent) and board, each resource's top sources a game (`own:`/`other:` an action, `phase:` an automatic phase),
  and keeps (offered, kept, rate, tier-2 share, insight at the keep) split by runaway_read's insight flag. Seconds.
- `cards/slots_read.py DIR... [--rounds N]`: per cell, each seat's build-overs a game and buildings standing each
  round (`slots.*`).
- `cards/board_read.py DIR... [--powers] [--cards]`: per cell, pooled across DIRs: wins ± SE, purge wins, Patience's
  carried insight, cubes bought, free stations, the Researcher's second node, Wall strikes stopped, troop hits, nodes
  lost, unspent; `--powers` each board power a game, `--cards` research-cube cards kept, played and used.
- `cards/use_read.py DIR... [--min PCT] [--agents A,B]`: per agent and pair, pooled over DIRs: wins, each dear
  power's uses a game (`ai::power_is_dear`), Grant and Action buildings' uses, tokens gained / spent, liberate cards
  kept / played; then `play_read.py`'s two lists. Seconds.
- `cards/liberation_read.py DIR... [--cards]`: per cell (config, pair, agent): wins ± SE, rounds, cities corrupted
  and liberated (card / action) a game, share of games with a liberation, Grant uses, Wall strikes stopped and troops
  it landed; `--cards` each liberate card's kept / played. Seconds.
- `cards/effect_stats.py READ [--screen DIR] [--flag 5] [--out MD]`: each Unity and Discord card's resolutions a game
  by tier and their effects (`fx.<id>.*`), each prohibition's fire rate and blocks; `--screen` an on/off screen
  (`PAIR_<id>`: tiers blanked) for the paired win effect, flagged past ±5. Seconds.
- `configs/no_liberate.py`: writes `experiments/configs/no_liberate.json`, the baseline less every `liberate` card;
  `configs/no_tier2.py` writes `no_tier2.json`, the baseline with no tier-2 deck (opening tier-2 keeps dealt from
  tier 1). `regen.py` reruns both.
- `cards/economy_read.py DIR... [--rounds 10]`: per cell, pooled, per seat and game: draft cards kept (opening keeps
  apart), played, never played, nodes placed, credit income a round, credits and insight unspent a round; each
  board's first dear power; each seat's most-held tag by round (`ai.tag_top.<seat>`). Seconds.
- `cards/energy_read.py DIR...`: per cell: win %, energy held as the action phase ends at rounds 3, 6 and last
  (`unspent.<seat>.energy.rNN`), each energy mechanic's use a game.
- `cards/corp_read.py DIR...`: per cell, pooled: wins, completions and mean round, cubes paid by color, stored and
  converted, uses per converter, stations claimed and built, adversary compute by source.
- `cards/price_read.py`: per-card rates and stratified win deltas from games.jsonl
  ([experiments.md](experiments.md), *Reading prices*).
- `cards/effect_ledger.py`: splits each card's printed cost over its effects pro rata to the reasoned values
  (`configs/agents/values.json`, an income over its tier's horizon), fits a price multiplier per effect, writes CSVs,
  `ledger.html` and `outliers.md` (tier-2 cards over the ceiling) to `experiments/results/effect-ledger/`.
  `--stdout` prints each card's fit and fair price (card-pricing.md, *A fair price*), writing nothing; `--cards FILE`,
  `--values FILE` read proposed files; `--overlay FILE` (with `--stdout`) prices an unwritten overlay (other lines skipped, noted); `--hold-ratio
  OLD.csv` adds the price keeping an older `cards.csv`'s printed / reasoned ratio. Seconds.
- `cards/payback.py [--cards CARDS.json] [--old CARDS.json [--old-keep-tier2 N]]`: rounds each engine building takes to
  pay back at a committed seat's fire rates (card-pricing.md *Engines*); `--old` adds an older deck.
- `run_sim.ps1 NAME [-NoBuild] ARGS...`: any long run: builds, copies the exe to `out/NAME.exe`, starts it below
  normal, detached; wait on `out/NAME.exit` (its header says more).
- `bench/same_results.py A B`: compares two `aic-sim experiment` outputs (`results.json`, `games.csv`); a farmed read
  has neither: use `farm/compare.py`. `--map OLD=NEW` renames in the old output first;
  `--ignore-content-hash` for a content refactor, `--ignore-config-hash` for a dead param removed (`same_play.py`
  can't prove that one: its replays compare fingerprints, which hash the config).
- `bench/same_play.py`: proves a refactor left play unchanged over countries, difficulties and variants, every
  agent but the beam, old and new binaries (~20 min, 6 threads); saved games replay on the new one. `--map` as above.
  An old binary that refuses today's configs plays its own: `run ... --root out/old`.
- `bench/speed.py`: games a minute per agent: `pair OLD NEW` (each row on both in turn; `--old-config` for an OLD on
  another baseline), `table`, `compare`, `specs SIM A B`; `check SIM` (~2 min, one thread, quiet machine) fails at
  1.3 × `speed_baseline.json` (`--update` moves it; across a revision: NO VERDICT). [speed.md](speed.md).
- `bench/per_decision.py OLD_ROOT NEW_ROOT`: rounds, decisions and ms a game and a decision on the same seeds.
  Across revisions use `--games 16`+ and name the agents a change touched. A
  root is an `.exe` or a directory with `aic-sim.exe` (or `target/release/`'s) and its `configs/`. `--set K=V`
  (repeatable) goes to both sides after the built-ins: a content overlay runs the new binary on old content.
- `bench/seeds.py next N [--take] [--doc FILE]`: the next N fresh seeds from experiments.md; `--take` moves `Next free`.
- `bench/gate_stop_rate.py` (gates are suspended): how often the gate's early stop flips a verdict (`z` 2: about 1%).
- `bench/skilled_rows.py READ.json [--games 24] [--levels 3,4] [--rows ...] [--out ...]`: the oracle's companion spec to a read (experiments.md, *Skilled rows*).
- `bench/read_specs.py {offs|screen|read} ... --tag NNN [--cards a,b,c] [--out-dir DIR]`: a rules item's specs into
  `out/`: `read TAG SEED [AGENT] [--probe] [--levels 0,1,2,3] [SET...]` three 48-seed sets, the three pairs,
  d0-d3; `screen NAME AGENTS LEVELS SEEDS [--offs] [--first N] [LEVER...]` a dev screen (lever `tag:a=1;b=2`);
  `offs` a config per `--cards` card switched off. `bench/run_read.sh NNN EXE ROOT TAG OUT THREADS` runs a read's
  three sets on one binary (ROOT its configs; EXE, OUT absolute or from the repo root).
- `bench/ladder.py NEW[,...] OLD[,...] [--measures] [--bands B] [--deck N]`: the paired ladder per level by revision;
  `--screen DIR[,...]` a screen's levers (`PAIR_L`, `PAIR-L`) against base, paired across DIRs. `FILE=SPEC.json`,
  engines, `--same-engine` as `paired.py`. `--measures`: the adversary's measures; by board slack, bound, land tokens,
  node spread; a screen's rounds (paired gap, won, lost). `--pairs [1-2,0-3]`: win % by unordered board pair (`bp`,
  `pb` pool), gaps to the mean, paired gap, seat order. `bench/split_read.py READ OUT` splits a lever screen.
- `bench/gate_close.py` (gates are suspended): `plan GATE_DIR OUT_DIR [--z 2] [--cap 96] [--fill 48]` writes specs
  carrying the rows within z paired SE on to the cap, reusing their games; `read` merges the runs.
- `bench/paired.py DIR... [--pair A B]... [--against-base] [--same-engine]`: any run's cells with win %, SE, rounds
  and ends, and paired gaps pooled over `--pair`s on the seeds both played (`--same-engine`: one engine's seeds only).
  `bench/test_read_tools.py` tests ladder.py, paired.py and screens.py: run it after editing any of them. A `--pair`
  cell is its exact name (`bp/strategist`, `DIR/bp/strategist`), led by config stem and `dN` where they vary
  (`no_liberate/d2/bp/strategist`). `--against-base`: a price sweep's `cN-x` against `cN`, each lever pooled over
  sizes. Per agent (or lever) by difficulty, pair and all: `--against-agent A` each agent against A (222),
  `--lever-suffix L` each `PAIR-L` (or `PAIR_L`) cell against `PAIR` (226). `--pool`: the DIRs are one read's seed
  ranges; `--other DIR` a read of another build on the same seeds, its agents as `other:AGENT`.
- `bench/shards.sh EXE DIR SPEC FIRST COUNT OUT`: a farm spec natively as `SHARDS` (default 6) one-thread `aic-sim
  farm` shards side by side in DIR, concatenated under the header into OUT: a paired read's base side.
- `bench/first_divergence.py OLD NEW`: where two runs of one spec first differ, per game; `--cells DIR A B` two cells
  of one run.
- `bench/profile_win.py SIM OUT -- run ...`: a sampling profile, no admin (speed.md, *Profiling*). `--runs K` on the
  next seeds each; `--attach S1,S2,...` reads 5 s windows of a long run.
- `bench/card_ranks.py DIR A B [--variant V]...`: Spearman of the cards' keep and play rates, agent B against A (as
  `cards.csv` names them).
- `bench/item_time.py ITEM... [--top N]`: a done conductor item's wall time: sims, builds, tests, the rest.
- `tune/`: `agent_sweep.py`, `sweep.py`, `log_table.py`, `engine_sweep.py` ([README](../scripts/tune/README.md)).
  `noop_weights.py spec [--agent A] [--seeds N]` / `read OUT SPEC`: each `Weights` field moved (to 0, or 1 where 0),
  one pair at d1-d2; lists the weights that change no game of the agent (222).
- Past items' one-off readers: `experiments/archive/scripts/`, kept for the write-ups citing them.

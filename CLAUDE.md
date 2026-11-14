# CLAUDE.md

The standing contract for every executor working in this repo. How to build and test, and this
machine's traps, are in the [README](README.md), which also lists the page for each area.

## What this repo is

**AI Complete**, a board game being turned into a digital version.

- The rules must stay playable as a printed board game. A digital-only mechanic (hidden
  bookkeeping, timing a human can't track) is a design change, not an implementation detail:
  flag it rather than build it.
- A **Rust core** simulates the rules. Clients render it and take input, and hold no rules; the
  first is **wasm + a JS front-end**.
- Design is still being gathered. The core implements the rules in [docs/rules/](docs/rules/README.md),
  whose `README.md` says which content is a placeholder. Don't describe or document code as if it
  does more than it does. **Read that `README.md`, then only the files your task touches.**

## Engine rules

- Keep the core invariants (no I/O, no clock, no global RNG, no hash-order leaks, serde state,
  exact replay, native = wasm). They, and how to add a module, variant or card, are in
  [docs/architecture.md](docs/architecture.md).
- Tunable numbers live in configs, never in code. A new mechanic is a module; another way of
  doing one is a variant.
- One home per rule: its number in a config or in content, its effect in one `Op` arm or the
  rule that reads it. What content does is one schema, `Op` (`crates/aic-core/src/op.rs`);
  resources, tags and token types are enums (`vocab.rs`): *The op schema* in docs/architecture.md.
- **Removing a power, op, param or standard action:** `git grep` its name in `crates/*/src/tests`, `agents/`,
  `scripts/design_check.json` and `web/scripts` before the first build (206 found 19 tests by running them; 219 left
  two browser checks on a removed power).
- **Any change to how games play out:** bump `RULES_REVISION` and update the golden constants
  (README, *Rules changes*). Bump from the constant, not from the prompt's revision number (a bug
  hunt may have taken it), and say so in the rules-history row and the report. A content-only
  edit moves the content hash constants with no bump unless play changes.
- **Every rules change** updates the `docs/rules/` files it touches and adds its row to
  [docs/rules-history.md](docs/rules-history.md) in the same commit.
- **After changing threats, cards, adversary rules, content, tunables or `values.json`:** run
  `python scripts/regen.py`, which builds and runs every generator in order (its steps:
  [docs/scripts.md](docs/scripts.md)). What it generates,
  and what a fast-lane test refuses, is in *Generated docs* in docs/architecture.md: change the
  source, not the output. What [docs/harms.md](docs/harms.md) can't read from content comes from
  `RULE_HARMS` in `crates/aic-sim/src/docs/harms.rs`.
- **Before the report**, `git grep` each renamed name and each changed number, and reread the
  `docs/rules/` file a new mechanic touches: a diff doesn't show the prose it left stale.

## Cards, mechanics and agents

- **Any card work** follows [docs/card-guidelines.md](docs/card-guidelines.md); `npm run cards:fit`
  in `web/` checks the fit, after `aic-sim docs` rewrites the faces it reads.
- **Ids.** An id is unique across all of `content/` (`Content::check` refuses a duplicate). A
  display rename keeps the id: notes, `scenes.json` and records key on it. Card id order feeds
  play, so an id changes only to fix a spelling `docs --check` refuses, in a rules item, with a
  `RULES_REVISION` bump; it strands any note on the old id.
- **Adding or removing a card** edits `configs/baseline.json`, then `python
  scripts/configs/shared_row.py` (`regen.py` runs it); a core test fails until you do.
- A pass editing many cards keeps its edits as one overlay file, written with
  `scripts/cards/apply_overlay.py`, so it reruns from any revision.
- **Give / get before any sim.** An item that adds or reprices a card, a building Action or a
  conversion writes a give / get line for each in its write-up, priced at
  `configs/agents/values.json`. Each shows a gain a player can see at those values: no break-even
  conversions. Loops are capped by limits (once a round, slots, supply), never by making the trade
  worthless. Fix or flag any that fails before the first read. When the table and the anchors
  disagree, the anchors set the printed price and the table is fixed (a rate, or a card's
  `overrides` row), never the price.
- **At a card's creation** also check: its net at the table is above 0 for the seat that would use
  it; its precondition exists in measured play (spare energy at round 6, say); a building beats the
  median building it would cover in a slot. **After any later edit to a card** (card fit included),
  re-read its plays per game.
- **A new op, resource or token type** gets its value in the same change: an op, its arm in
  `crates/aic-sim/src/agents/value.rs` and its row in `configs/agents/values.json`; a resource or
  token type, its held-value term in `agents/eval.rs`.
- **Smokes before reads.** A new op, resource or token type gets a 2-minute engine smoke (16
  seeds) before any band read: `scripts/cards/play_read.py` for cards kept and never played,
  `board_read.py --powers` for powers never used. Before a full read, `python
  scripts/cards/loop_smoke.py --run` (16 seeds: timeouts, most plays of one card a game) catches a loop.
- **A rules item's first freeze** comes after `npm run cards:fit`, `cargo test --workspace` and
  `aic-sim docs --check`, with every touched card priced from the effect ledger
  (`scripts/cards/effect_ledger.py`) before the first screen: a card-fit edit after a freeze redoes
  every frozen read. Before the freeze, also read each new mechanic's use per game and its on/off
  gap on the same seeds.
- **A new decision type for the agents** (a new kind of card to keep, a new action to take): review
  the agent code that makes it (keep plan, eval term, fixed policies) before the first screen.
  **A new agent heuristic** gets a 16-seed engine smoke, on and off, on the dev seeds, and its eval
  diff reviewed against the engine rule it mirrors, both before its first fresh read.

## Reads

- **The prompt's stated numbers run as a cell of every screen**, beside any lever you try, so the
  report can always say what the asked-for numbers did.
- **Compare a lever with base on the same seeds, and confirm on fresh ones.** Pooling new seeds with
  the seeds that picked a winner is not confirmation: report the fresh seeds alone. A fresh read is
  spent the moment you change the build because of it: confirm any later change on the next free
  seeds. A lever added beside the asked one is screened on and off as a factorial. A change to a core
  rules file after a run was frozen replays that run's seeds on the final build before the report,
  compared with `bench/same_results.py` (an experiment output) or `farm/compare.py` (a farmed read's `games.jsonl`):
  keep the frozen run's output directory.
- **On tuning**, settle every dial on screens first and run the full test lane once, at the end.
- **Agents stay fast.** After a change to rules, state or agents, run `python scripts/bench/speed.py
  check target/release/aic-sim.exe` after the first screen, while a fix can still be tested; an item
  that changes an agent includes a greedy line. It fails over 1.3 × the baseline and gives no verdict
  across a rules revision, where `bench/per_decision.py` says what the change cost. Read an agent's
  cost per action with `per_decision.py`, not per game or round: longer games inflate those.
  Under load a failure proves nothing: `speed.py pair OLD NEW --only engine` holds. A slowdown you mean to keep: say why in the report and `--update` the
  baseline. More in [docs/speed.md](docs/speed.md).

## Working rules

- **Commit to the branch you are on.** Leave nothing uncommitted.
- **Two test lanes** (README, *Build and test*). Run `cargo test --workspace`, the fast lane, after
  every change, and `cargo test-all`, the full run, before every commit; a commit touching only
  Markdown outside the generated docs skips the full run. A test slower than about a second goes in
  the slow lane, with a cheaper fast-lane twin where one makes sense. Leave `cargo clippy --workspace
  --all-targets` clean. The repo doesn't use rustfmt. Web's checks: [web/README.md](web/README.md).
- **Background any step over ~30 seconds** (builds, tests, servers), and wait on the real process
  (the `cargo`, `node` or test binary), never on a launcher that returns before the work is done.
  Give a compile its time: a fresh worktree's first build is minutes.
- **One cargo at a time in a target dir.** Parallel helpers need their own `CARGO_TARGET_DIR`.
- **Write scripts with Write, not a Bash heredoc.** A file a script writes passes `encoding='utf-8',
  newline='\n'`.
- **Times are local, 12-hour clock** (2:45pm), in reports, docs and progress lines.
- **American English everywhere:** content, faces, docs, UI, comments and names.
- **LF line endings.** `.gitattributes` sets `* text=auto eol=lf`; don't override it.
- **Cleanups never delete `review/`.** It is git-ignored and holds Matt's card notes: nothing
  regenerates them.
- **Facts learned while working go in the page for their area,** never only in a report: build,
  test and this machine in `README.md`; a run's cost and recipe in `docs/experiments.md`; a script's
  flags in `docs/scripts.md`. A number from a past revision goes in its item's write-up under
  `experiments/results/`; `experiments/history.md` only indexes them.
- **Each doc keeps to 15 KB**: README, CLAUDE.md, the web and card-art READMEs and every `docs/`
  page, `docs/rules/` files included, but those in `BUDGET_EXEMPT`
  (`crates/aic-sim/src/docs/run.rs`); `docs --check` refuses one over. Leave about 1 KB free: an
  item that fills a page trims it first, cutting what the code or a generated table already says,
  never moving text to an archive file.
- **Docs state the current game.** An item updates the current text in place, never appends an
  "NNN's changes" section (`docs --check` refuses the heading); history goes in the item's
  write-up. A card's pricing note for the viewer is one current line in
  `docs/card-pricing-notes.json`, keyed by card id, written by the item that sets the price.

## Report

Write the report to the path conductor's preamble gives, starting with the header it specifies.

- Soft target about 60 lines: findings, decisions and surprises only, no process narration.
- Mention tests only for a red you couldn't fix.
- Answer every amendment and message by number.
- **A rules item with new or changed cards** ticks this list: give / get before the first screen;
  `npm run cards:fit`; `play_read.py`; `loop_smoke.py --run`; `speed.py check`;
  `bench/same_results.py` (or `farm/compare.py`) on a frozen run's seeds.
- A rules item's report ends with a short new-player summary of its rule changes, naming any rule
  that resisted one.
- End your final message with `Report written to <path>`.

## Warning checks

A rules item runs `python scripts/design_check.py READ --old OLD_READ` on its first screen and on
its fresh read (READ a read's folder or `games.jsonl`; OLD_READ the previous revision's read on its
seeds; with no READ only the content and dials checks run). Its first line is the report header's
`warnings:` line: copy it in. Warnings never stop work: an item whose own change trips one
finishes, calibrates, and reports the warning first. The principles are in
[docs/principles.md](docs/principles.md), *Design warnings*; thresholds, all candidates, in
`scripts/design_check.json`.

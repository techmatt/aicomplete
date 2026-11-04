# CLAUDE.md

The standing contract for every executor working in this repo. How to build and test is in
the [README](README.md), which also lists the page for each area: the simulator, agents,
experiments, speed, scripts, web, card art.

## What this repo is

**AI Complete**, a board game being turned into a digital version.

- The rules must stay playable as a printed board game. A digital-only mechanic
  (hidden bookkeeping, timing a human can't track) is a design change, not an
  implementation detail: flag it rather than build it.
- A **Rust core** simulates the rules. Clients render it and take input, and hold no rules;
  the first is **wasm + a JS front-end**.
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
  rule that reads it.
- What content does is one schema, `Op` (`crates/aic-core/src/op.rs`); resources, tags and
  token types are enums (`vocab.rs`). See *The op schema* in docs/architecture.md.
- **A new op, resource or token type** gets its value in the same change (an op: its arm in
  `crates/aic-sim/src/agents/value.rs` and its row in `configs/agents/values.json`; a resource or
  token type: its held-value term in `agents/eval.rs`), and a 2-minute engine smoke read (16 seeds; `scripts/cards/play_read.py`
  for cards kept and never played, `board_read.py --powers` for powers never used) before any
  band read: 075 lost a whole screen to six cards and Grant that no agent valued. **Before a freeze**, read
  each new mechanic's use per game and its on/off gap on the same seeds (080: `liberation_read.py`).
- **A new decision type for the agents** (a new kind of card to keep, a new action to take): review the
  agent code that makes that decision (keep plan, eval term, fixed policies) before the first screen, not
  after it reads low. **A new agent heuristic** gets a 16-seed engine smoke, on and off, on the dev seeds
  before its first fresh read (092 lost a screen and a fresh seed set to two heuristics that lost).
- **The prompt's stated numbers run as a cell of every screen**, beside any lever you try, so the report
  can always say what the asked-for numbers did.
- **Any change to how games play out:** bump `RULES_REVISION` and update the golden constants
  (README, *Rules changes*). Bump from the constant, not from a prompt's revision number, which
  is a guess since a bug hunt may have taken it, and say so in the rules-history row and the
  report. A content-only edit moves the content hash constants with no bump unless play changes.
- **Every rules change** updates the files of `docs/rules/` it touches and adds its row to
  [docs/rules-history.md](docs/rules-history.md) in the same commit.
- **After changing threats, cards, adversary rules, content or tunables:** run `aic-sim docs`,
  then `python scripts/cards/cards_doc.py`; after a card or `values.json` edit also `python
  scripts/cards/effect_ledger.py` (a fast-lane test reads its `cards.csv`). What they generate, and what a fast-lane test
  refuses, is in *Generated docs* in docs/architecture.md; change the source, not the output.
  What [docs/harms.md](docs/harms.md) can't read from content comes from `RULE_HARMS` in
  `crates/aic-sim/src/docs/harms.rs`.
- **Before the report**, `git grep` each renamed name and each changed number, and reread the
  `docs/rules/` file a new mechanic touches. A diff doesn't show the prose it left stale, and a
  sentence saying something never happens has nothing to grep.
- **Adding or removing a card** edits `configs/baseline.json`, then run `python
  scripts/configs/shared_row.py`; a core test fails until you do.
- **Any card work** follows [docs/card-guidelines.md](docs/card-guidelines.md) (`npm run
  cards:fit` in `web/` checks the fit). A display rename keeps the card's id: notes,
  `scenes.json` and records key on it. An id changes only to fix a spelling `docs --check`
  refuses, and only in a rules item: card id order feeds play (087 kept 086's two ids for it),
  so it bumps `RULES_REVISION` and strands any note on the old id.

## Working rules

- **Commit to the branch you are on.** Leave nothing uncommitted.
- **Give / get before any sim.** An item that adds or reprices a card, a building Action or any conversion first
  writes a give / get line for each, priced at `configs/agents/values.json`, in its write-up. Each must show a gain a
  player can see at the values it is priced at: no break-even conversions. Loops are capped by limits (once a round,
  slots, supply), never by making the trade worthless. Fix or flag any that fails before the first read (090's energy
  gift traded 1 energy, worth 4.5, for 2 chips, so it was never played). **At a card's creation** also check: its net
  at the table is above 0 for the seat that would use it; its precondition exists in measured play (spare energy at
  round 6, say); a building beats the median building it would cover in a slot. **After any later edit to a card**
  (card fit included), re-read its plays per game (095's fit edits took the gift buildings from 0.25-0.6 uses to
  0.02-0.06, unflagged).
- **A rules item's first freeze** comes after `npm run cards:fit` (in `web/`, after `aic-sim docs` rewrites the faces it reads), `cargo test --workspace` and `aic-sim
  docs --check`, with every touched card priced from the effect ledger (`scripts/cards/effect_ledger.py`) before the
  first screen: a card-fit edit after a freeze redoes every frozen read (090 redid them all after four, and
  repriced twice after screens).
- **Two test lanes.** Run `cargo test --workspace`, the fast lane, after every change, and
  `cargo test-all`, the full run, before every commit; a commit touching only Markdown outside the
  generated docs skips it, the fast lane still runs. Flags follow the alias with no `--`:
  `cargo test-all --test-threads=2`. A test slower than about a second goes in the slow lane,
  with a cheaper fast-lane twin where one makes sense. See README, *Build and test*. Leave
  `cargo clippy --workspace --all-targets` clean. The repo doesn't use rustfmt. In a fresh
  worktree, web's `npm run check` needs `npm run wasm` first; `npm run e2e` needs the dev server
  running (another port: `AIC_URL`, web/README); e2e's `board fills its pane`
  plays a fixed game (baseline seed 11, first legal action), not the random game, so a rules
  change can move it (web/README).
- **Agents stay fast.** After a change to rules, state or agents, run `python
  scripts/bench/speed.py check target/release/aic-sim.exe`, after the first screen, not last,
  while a fix can still be tested. It fails over 1.3 × the baseline and gives no verdict across
  a rules revision: `bench/per_decision.py` then says what the change cost. On a busy machine a
  failure proves nothing: run `speed.py pair OLD NEW --only engine`, which holds under load. A
  slowdown you mean to keep: say why in the report and `--update` the baseline. Read an agent's
  cost per action, not per game or round, with `per_decision.py`: longer games inflate the others.
- **Compare a lever with base on the same seeds, and confirm on fresh ones.** Pooling new seeds
  with the seeds that picked a winner is not confirmation: report the fresh seeds alone. A lever
  added beside the asked one is screened on and off as a factorial: two changes in one row can't
  say which did it. A change to a core rules file after a run was frozen needs that run's seeds
  replayed on the final build before the report, compared with `bench/same_results.py`: keep
  the frozen run's output directory for this.
- **Background any step over ~30 seconds** (builds, tests, servers), and wait on the real
  process (the `cargo`, `node` or test binary), never on a launcher wrapper that returns before
  the work is done.
- **One cargo at a time in a target dir.** Parallel helpers need their own `CARGO_TARGET_DIR`.
- **Write scripts with Write, not a Bash heredoc:** quoting breaks it, and a long one fails on
  size. A file a script writes passes `encoding='utf-8', newline='\n'`. This box's traps are in
  README, *This machine*.
- **Times are local, 12-hour clock** (2:45pm), in reports, docs and progress lines.
- **American English everywhere:** content, faces, docs, UI, comments and names.
- **LF line endings.** `.gitattributes` sets `* text=auto eol=lf`; don't override it.
- **Cleanups never delete `review/`.** It is git-ignored and holds Matt's card notes: nothing
  regenerates them.
- **Operational facts go in the page for their area:** build, test and this machine in
  `README.md`; a run's cost and recipe in `docs/experiments.md`; a script's flags in
  `docs/scripts.md`. Never leave one only in a report. A number from a past revision goes in
  its item's write-up under `experiments/results/`; `experiments/history.md` only indexes them.
- **Each doc keeps to a 15 KB budget** (README, CLAUDE.md and every `docs/` page, each
  `docs/rules/` file included: `docs --check` refuses one over), except the generated
  `docs/cards.md`. Trim before you add past it.
- **Docs state the current game.** An item updates the current text in place, never appends an
  'NNN's changes' section. History goes in the item's write-up. `aic-sim docs --check` refuses a
  heading like `## 080's changes`. A card's pricing note for the viewer is one current line in
  `docs/card-pricing-notes.json`, keyed by card id, written by the item that sets the price.

## Report

Write the report to the path conductor's preamble gives, starting with the header it specifies.

- Soft target about 60 lines.
- Findings, decisions and surprises only: no process narration.
- Mention tests only for a red you couldn't fix.
- Answer every amendment and message by number.
- A rules item's report ends with a short new-player summary of its rule changes, naming any rule
  that resisted one (Matt, 2026-11-03).
- End your final message with `Report written to <path>`.

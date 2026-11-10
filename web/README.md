# web: the AI Complete viewer

A debug-first browser client on top of `aic-wasm`. Vite + TypeScript, no UI framework.
It renders the engine's **generic view** and offers only what `legal_actions` returns.
See [../docs/architecture.md](../docs/architecture.md#the-generic-view)
for how a module shows up in it.

## Build and run

Needs the root README's *Toolchain*.

```sh
cd web
npm install                     # once per worktree
npm run dev                     # builds the wasm, then serves http://localhost:5173/
npm run dev -- --port 61870     # another port
npm run wasm                    # only rebuild web/src/wasm/ from crates/aic-wasm
npm run check                   # typecheck (needs web/src/wasm/: npm run wasm first)
npm run build                   # wasm + typecheck + static site in web/dist/ (92 s cold)
npm run e2e [-- <record.json>] # browser check, dev server must be running (see below)
npm run e2e:sources            # the same way, as are e2e:corps and e2e:researcher
```

- **The wasm build:** `scripts/build-wasm.mjs` runs `wasm-pack build crates/aic-wasm
  --target web --release` into `web/src/wasm/` (git-ignored); 74 s in a fresh target dir (background it there). `AIC_WASM_PROFILE=dev` builds unoptimized.
- **While the dev server runs**, editing anything under `crates/` (tests and `aic-sim`
  included) or `content/` rebuilds the wasm and reloads the page; a new or edited file in
  `configs/` reloads it. A failed rebuild keeps the last good engine;
  the error is in the dev server's terminal.

## Using it

- **Play:** pick a config (any config in `configs/`; `baseline.json` is the default) and a
  seed, then *New game*. One person plays every seat, in any order during a simultaneous phase. Draft phases offer no pass.
  *Pass every seat*, *Random action* (R) and *Random to end* are for quick debugging.
- **Choices:** the question shows in the amber bar above the board. A hex answer
  highlights its hexes: click one. Two answers on one hex (a sacrifice: the ambassador
  or the node there) also get a button each (the hex takes the first). Other answers are
  buttons in the seat's panel. Text names hexes by country
  (`Cintamani #13`) and cards by name.
- **The board:** hexes filled by `kind` (key at the bottom left), base pennants in their
  `facing` country's color, fronts solid red and pulsing. Player pieces sit on a hex's upper
  row in the seat's color, the adversary's on the lower row in purple. Hover anything for
  details; hexes the shown step changed have an amber outline.
- **Actions:** each seat panel offers cards, threats, gifts, the view's `standard`
  actions, a *power* button per AI-board power, *antimatter* and a *sell* per card in
  hand. AI board tracks (`seats[].board`): solid spaces uncovered, dashed under nodes,
  boxed the last uncovered.
- **Globals:** a `vars` track no module labels folds under *Unlabeled tracks*.
- **Timeline:** Ctrl+Z undoes (every state is a replay of config + seed + actions). ⏮ ◀
  ▶ ⏭, the slider, ← → Home End, Space to auto-play at 1x (4 steps a second), 4x or 16x.
  **Page Up / Down** jump between round starts. Clicking a log entry jumps to its
  step; acting from an earlier step drops the later actions, after asking. Log events
  fold by phase; hovering a heading outlines that phase's hexes.
- **The header** shows the engine version and rules revision. A saved session
  (`localStorage`, replayed on load) whose config changed or no longer loads, or a record
  from another revision, says so in the error bar; one that no longer loads is kept
  (`copy(aic.keptSession())`). Tabs: event log, raw state, generic view, resolved config.

## Game records

Format: [../docs/simulator.md](../docs/simulator.md), *Game records*.

- **Watch a simulated game:** `aic-sim run ... --save DIR` writes records. *Open
  record…* or drop the file on the page; it opens at step 0. A record whose replay
  differs from what was saved loads with a warning.
- **Bug report:** *Record / bug report…* holds the current game as a record, with an
  optional note. *Copy this game*, *Copy up to shown step*, or paste a record and *Load*
  it.

## Browser check

`npm run e2e [-- <record.json>]` (`scripts/e2e.mjs`) drives headless Edge or Chrome over
the DevTools protocol (no packages). `AIC_URL` points it at the server (default
`http://localhost:5173/`), `AIC_BROWSER` at a browser off the usual path. It checks the
panels and board, a baseline game (seed 1) played to the end, the interactions above, no
page scroll at 1366×768, 1920×1080 and 2560×1440, words not ids, under 100 ms a step, a
4-country game (may scroll at 1366×768), the shared row and
the given record; each `check(...)` names itself.
About 15 s. Screenshots go to `web/e2e-out/` (git-ignored); `AIC_SHOTS=<dir>` also saves
`viewer-mid.png` and `viewer-end.png` there.

The face-up adversary card and threat checks run at round 2 (a fresh game has neither).
`board fills its pane` measures a fixed game (baseline seed 11, first legal action): a rules
change can move it, so remeasure before reading a layout change into a failure: `npm run panes`
prints that game's pane heights at the three sizes; diff two revisions' output.

`e2e:corps` and `e2e:researcher` play the baseline
at `station_cubes` and `owned_station_cubes` 5 (corps) or 3 (researcher) on a quiet board (no troop steps or adversary supply, no corruption before round 20)
for cubes, converters (a node on a city) and the Researcher's board; `e2e:sources`
checks a face-up threat's sources and a forged payment refused.

**Against a built site:** a dev-server rebuild breaks a run mid-way (`reading 'view'`), so
while the engine is edited serve a snapshot: `node scripts/build-wasm.mjs && npx vite build
&& npx vite preview --port $PORT --strictPort`, `AIC_URL=http://localhost:$PORT/`
(`npx vite build` alone, about 1 s, after a `web/src/` edit). A killed wrapper's `node` can
hold the port (`--strictPort` fails): kill it.

**An old revision side by side** (about 3 min): `git archive REV | tar -x -C out/rREV`,
then in its `web/` `npm ci` and the snapshot above on another port.

## The sweep

`npm run sweep [-- --quick] [-- --case NAME]` (`scripts/sweep.mjs`), the bug hunt's
checker, checks the page at the moments that matter over many games. It needs a served viewer (`AIC_URL`; see *Against a built site*) and a release `aic-sim` (`AIC_SIM`, default
`../target/release/aic-sim[.exe]`).

- **Cases:** `CASES` in the script (`--quick` keeps the first four, `--case c4-d0` picks
  one). For each, `aic-sim` saves records (greedy unless the case names an agent); the
  sweep **watches** each record step by step and **plays** as many fresh games through the page
  at random (choices, hex picks, the odd undo). `AIC_SWEEP_SEEDS` games per case and mode (3; `--quick` 1), the
  simulated ones from seed `AIC_SWEEP_SEED` (100).
- **Moments:** new rounds, prompts, the first of each key event and counter rise
  (`COUNTER_EVENTS`, `_R41`, `_R55`, `_R70`), the end. At each, the checks at 1366×768, layout at the two
  larger sizes, and a screenshot (at most 14 a game, plus the end and any failure).
- **Fail the run** (exit 1): console errors and blocking dialogs, `stuck` (no legal action
  without an outcome), the error bar, page scroll, placeholder text (`undefined`, `NaN`),
  elements off screen with no scroll to them, `hidden-target`, `clipped` (no ellipsis),
  `token-unknown`, `stale-word` (an old house word on screen: agents, Operatives), `case` (`aic-sim` could not play it). **Reported for a look:** `no-control`
  (a legal action with no control on screen), `raw-id`, `truncated-no-hover`, `truncated`,
  overlaps, undrawn view tokens, steps over 150 ms, heap growth over 50 MB.
- **Output** in `web/sweep-out/` (`AIC_SWEEP_OUT`): `shots/`, `issues.json` (game, step,
  moment), `index.html`, `sheet-N.png` contact sheets (failures framed red), the records.
  `AIC_SWEEP_VERBOSE=1` times each moment. Hooks: `window.aic` (main.ts), `scripts/cdp.mjs`.
- **Running it:** background it, the viewer served by a process that outlives it; don't
  rebuild `aic-sim` mid-sweep. Rules twin: `invariants_hold_in_random_games`. At most one
  sweep beside one browser test lane. Five cases at one seed (541 captures) take about 12
  min: a capture about 1.3 s (three sizes).

## Kept, never played

`npm run probe -- DIR card_id...` (`scripts/legal-probe.mjs`): over saved records, the steps
each card was held, legal to play and played. Legal, never played: the agents' valuation.

## Odd input

`npm run odd` (`scripts/odd.mjs`) runs twelve scenarios against a served viewer
(`AIC_URL`): mashing, mid-choice reloads, resizes during autoplay, full storage, broken
records and sessions, bad seeds. Fails on a console error, a stuck game, a page scroll
or a scenario's own check, and prints what each bad input said. `--only NAME`, `--seeds N`. About 30 s; it counts as one browser
lane. Failure shots go to `sweep-out/odd/`.

## Source

In `src/`: `engine.ts` (typed wasm wrapper), `session.ts` (`Timeline`), `board.ts` (SVG
board), `format.ts` (labels, colors), `panels.ts`, `hover.ts`, `log.ts`, and `main.ts` (wiring).

`card/` is the printed card renderer, used by `tools/card-art/` and the card viewer
(`cards/`): see [tools/card-art/README.md](../tools/card-art/README.md).

## Card viewer

```sh
cd web && npm run cards                            # exports, then serves http://localhost:5180/cards.html until stopped
```

Conductor's button runs that command in the main checkout and opens that URL: keep the
command, port and path as they are. It needs no `npm install`, wasm or cargo: plain Node
over its checkout. The export takes about 0.4 s. `--port N` serves elsewhere; a taken port
says so and exits.

- **Every card in content** (country pieces are map, not cards). Player cards go through
  `web/card/card.js` with the `art-originals/` scene `tools/card-art/scenes.json` names
  (none: drawn without); a kind with no frame gets a plain face. Scenes are 3.6 MB PNGs
  loaded as they scroll in, so a fast scroll shows gray for a second or two.
- **The export** (`web/scripts/cards-export.mjs`, to `out/cards-export.json`) reads
  `content/`, `web/card/faces.json`, `configs/baseline.json` and the stats and pricing files below. It never runs `aic-sim docs`:
  a card edited without it shows its old text. **Rebuild** on the page re-exports in ~0.1 s.
- **Stats** come from the newest `results.json` of 100+ games with a per-card `cards`
  table, in `experiments/results/` or the checkout's `out/experiments/` (highest revision,
  then most plain agents, then most games). Each label carries its revision; the page
  warns (one collapsed *N warnings ▸* line) when it is behind the rules. The default agent
  is the ladder's highest. An item whose screen should feed the viewer commits its
  `results.json`, not only a report.
  - Kept % is keeps per 100 games (the opening deal is kept undrafted, so a report's kept
    ÷ offered can pass 100%). Played % is played ÷ (kept + drawn). Win Δ comes from
    `games.csv`, which names only buildings standing at the end: blank for other cards.
  - A CSV under `experiments/results/` with a `card` (or `id`) column, a row a card, and a
    column with `reason` in its name (`card-prices.csv` by convention) adds the reasoned
    price and printed minus reasoned. One whose `revision` is older than `RULES_REVISION`
    is passed over silently (the CLI export names it); none is current now.
- **Notes:** `review/card-notes.json` in the checkout, git-ignored. Keyed by card id: `{ name, kind, hash, flag, comment, text, at }`. `flag` is
  empty or one of `ok`, `too cheap`, `too dear`, `rework`, `unclear text`; `text` is the
  card's text at saving; `hash` a SHA-256 prefix of its content row and face, key
  order aside (`contentHash` in `out/cards-export.json`). Closing or stepping saves the note (previous
  file kept as `.bak`); a broken notes file shows on the page and refuses saves.
- **Badges:** *changed since your note* when the note's `hash` is stale (saving again clears it).
  *New* for a card missing from `review/card-seen.json`, until opened, noted or *Mark all
  seen*; the first run seeds that ledger with the whole set.
- **Pricing:** a card's one current line from `docs/card-pricing-notes.json`, under
  *Pricing* in the large view (format and rules: root CLAUDE.md; `docs --check` refuses an id that is no player card).
- **Check:** `npm run cards:check -- --port N` (about 20 s) drives a headless browser
  through all that on a copy in `out/cards-check/` (screenshots there). It fails on a card
  overflowing at the last size or title floor, a title off its plate, text wrapping apart
  between zooms, or a deck card with no scene or scene image.
- **Card rules** are in [docs/card-guidelines.md](../docs/card-guidelines.md). A test refuses any face printing
  `*` or "worth".
- **Shots of named cards** in the viewer's large view: `node scripts/card-shots.mjs ID,ID [--port N] [--out NAME]`
  (shots in `out/NAME/`, about 1 s a card).
- **Text sizes:** `npm run cards:fit` (about 2.5 s, no server) fits every player card
  headless with the real renderer and writes `out/card-fit.json` (the repo root's `out/`, as for every `out/` path
  in this section): per id,
  `top` and `effect` box sizes in pt (null: empty box), `title` in pt, `overflow` and
  `where`, and `tight` for a box inside its panel but short of the clearance. It prints
  cards below 7.5 pt and exits 1 on any overflow. It reads the working tree: re-run after
  an edit.

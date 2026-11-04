# web: the AI Complete viewer

A debug-first browser client on top of `aic-wasm`. Vite + TypeScript, no UI framework.
It renders the engine's **generic view** and offers only what `legal_actions` returns;
it knows no rules. See [../docs/architecture.md](../docs/architecture.md#the-generic-view)
for how a module shows up in it.

## Build and run

Needs Node (built with 24), Rust with the `wasm32-unknown-unknown` target, and
wasm-pack (see the root README).

```sh
cd web
npm install                     # once; a fresh worktree has no node_modules
npm run dev                     # builds the wasm, then serves http://localhost:5173/
npm run dev -- --port 61870     # another port
npm run wasm                    # only rebuild web/src/wasm/ from crates/aic-wasm
npm run check                   # typecheck (needs web/src/wasm/: npm run wasm first)
npm run build                   # wasm + typecheck + static site in web/dist/ (92 s cold)
npm run e2e [-- <record.json>] # browser check, dev server must be running (see below)
npm run e2e:sources            # the threat-source check, the same way
npm run e2e:corps              # research cubes and the corps, the same way
npm run e2e:researcher         # the Researcher's board, the same way
```

- **The wasm build:** `scripts/build-wasm.mjs` runs `wasm-pack build crates/aic-wasm
  --target web --release` into `web/src/wasm/` (git-ignored): about 20 s cold, 9 s warm,
  74 s in a fresh target dir, over 60 s under load, so background it there.
  `AIC_WASM_PROFILE=dev` builds unoptimized.
- **While the dev server runs**, editing anything under `crates/` (any `.rs`, tests and
  `aic-sim` included) or `content/` rebuilds the wasm and reloads the page; editing or
  adding a file in `configs/` reloads it (configs are read at build time,
  `import.meta.glob`). A failed rebuild keeps the last good engine; the error is in the
  dev server's terminal.

## Using it

- **Play:** pick a config (any file in `configs/`; `baseline.json` is the default) and a
  seed, then *New game*. Each seat's panel lists that seat's legal actions; one person
  plays every seat, in any order during a simultaneous phase. Draft phases offer no pass.
  *Pass every seat*, *Random action* (R) and *Random to end* are for quick debugging.
- **Choices:** the question shows in the amber bar above the board. A hex answer
  highlights its hexes at once: click one. When two answers name the same hex (a
  sacrifice: the ambassador or the node there), each also gets a button (the hex takes
  the first). Other answers are buttons in the seat's panel. Text names hexes by country
  (`Cintamani #13`) and cards by name.
- **The board:** hexes filled by `kind` (key at the bottom left), base pennants in their
  `facing` country's color, fronts dashed dark red; on a war step (`board.war` = 1) fronts
  turn solid red and pulse and the adversary row shows a **war round** badge. Player
  pieces sit on a hex's upper row in the seat's (its country's) color, the adversary's on
  the lower row in purple. Hover anything for its details; hexes the shown step changed
  have an amber outline.
- **Actions:** each seat panel offers cards, threats, gifts, the view's `standard`
  actions with the rules' words and costs, a *power* button per AI-board power,
  *antimatter* and one *sell* per card in hand. The AI board's tracks come from the
  generic `seats[].board` shape: solid spaces uncovered, dashed under nodes, boxed the
  last uncovered.
- **Globals:** a `vars` track no module labels folds under *Unlabeled tracks* (the
  baseline has none).
- **Timeline:** Ctrl+Z undoes (every state is a replay of config + seed + actions). ⏮ ◀
  ▶ ⏭, the slider, ← → Home End, Space to auto-play at 1x (4 steps a second), 4x or 16x.
  **Page Up / Page Down** jump between round starts. Clicking a log entry jumps to its
  step; acting from an earlier step drops the later actions, after asking. A step that
  runs through phases folds its log events by phase; hovering a heading outlines that
  phase's hexes.
- **The header** shows the engine version and rules revision. A saved session (kept in
  `localStorage`, replayed on load) whose config changed or no longer loads, or a record
  from another revision, says so in the error bar. Data tabs show the event log, raw
  state, generic view and resolved config with its hash.

## Game records

A record is config, seed and actions, plus identity (hashes, engine version, rules
revision) and the saved outcome and final fingerprint (`aic_core::SavedGame`, format
`aic-game-record/1`, one action per line).

- **Watch a simulated game:** `aic-sim run ... --save DIR` writes records. *Open
  record…* or drop the file on the page; it opens at step 0. A record whose replay
  differs from what was saved loads with a warning.
- **Bug report:** *Record / bug report…* holds the current game as a record, with an
  optional note. *Copy this game*, *Copy up to shown step*, or paste a record and *Load*
  it. `aic-sim replay <file>` checks a record natively.

## Browser check

`npm run e2e [-- <record.json>]` (`scripts/e2e.mjs`) drives headless Edge or Chrome over
the DevTools protocol, no extra packages. `AIC_URL` points it at the server (default
`http://localhost:5173/`), `AIC_BROWSER` at a browser off the usual path. It checks every
panel and the board, plays a baseline game (seed 1) to the end through the page, then
the interactions above, no page scroll at 1366×768, 1920×1080 and 2560×1440, words
rather than ids on screen, under 100 ms a step, a 4-country game, the shared row
(`configs/shared_row.json`) and the given record; each `check(...)` names itself.
About 15 s. Screenshots go to `web/e2e-out/` (git-ignored); `AIC_SHOTS=<dir>` also saves
`viewer-mid.png` and `viewer-end.png` there.

A fresh game has no face-up adversary card and no threats yet, so the checks of those run at round 2
of the played game (096). `board fills its pane` measures a fixed game (baseline seed 11, first legal action), not
the random one, which moved the same build by 60 px. A rules change can move it:
remeasure before reading a layout change into a failure.

The other three take the same `AIC_URL`. `e2e:corps` and `e2e:researcher` play the
baseline at `station_cubes` 3 on a quiet board (arrivals, city builds and troop marches
off, `corrupt_at` 20) to check research cubes, converters and the Researcher's board;
`e2e:sources` plays until a face-up threat refuses a seat, then checks its sources and
the engine refusing a forged payment.

**Against a built site:** while anyone edits the engine, a dev-server rebuild breaks a
run mid-way (`Cannot read properties of undefined (reading 'view')`; rerun once built).
Use a snapshot: `node scripts/build-wasm.mjs && npx vite build && npx vite preview --port
$PORT --strictPort`, then `AIC_URL=http://localhost:$PORT/`. After changing only
`web/src/`, `npx vite build` (about 1 s) refreshes `dist/`.

**Stale port:** stopping a backgrounded `npm run dev` or `npx vite preview` wrapper (a
tool runner's time limit included) can leave its `node`/`vite` process holding the port,
and `--strictPort` then fails. Kill the process that holds it.

## The sweep

`npm run sweep [-- --quick] [-- --case NAME]` (`scripts/sweep.mjs`), the bug hunt's
checker, plays and watches many games at test speed and checks the page at the moments
that matter. It needs a served viewer (`AIC_URL`; a `vite preview` snapshot while the
engine is edited) and a release `aic-sim` (`AIC_SIM`, default
`../target/release/aic-sim[.exe]`).

- **Cases:** `CASES` in the script (`--quick` keeps the first four, `--case c4-d0` picks
  one). For each, `aic-sim` plays games (greedy unless the case names an agent) and saves
  records; the sweep **watches** each record step by step and **plays** as many fresh
  games of the resolved config through the page, picking legal actions at random (choices,
  hex picks, the odd undo). `AIC_SWEEP_SEEDS` games per case and mode (3; `--quick` 1).
- **Moments:** new rounds, prompts, war steps, the first of each key event, the end. At
  each, the checks at 1366×768, layout checks at 1920×1080 and 2560×1440, and a
  screenshot (at most 14 a game, plus the end and anything failing).
- **Fail the run** (exit 1): console errors and blocking dialogs, `stuck` (no legal action
  without an outcome), the error bar, page scroll, placeholder text (`undefined`, `NaN`),
  elements off screen with no scroll to them, `hidden-target`, `clipped` (no ellipsis),
  `token-unknown`, `case` (`aic-sim` could not play it). **Reported for a look:** `raw-id`,
  `truncated-no-hover`, `truncated`, overlapping buttons, hex numbers and counts on each
  other, tokens in the view not drawn, step spikes over 150 ms, heap growth over 50 MB.
- **Output** in `web/sweep-out/` (git-ignored; `AIC_SWEEP_OUT` moves it): `shots/`,
  `issues.json` (each issue with game, step and moment), `index.html` and `sheet-N.png`
  contact sheets (failing captures framed red), and the watched records. `AIC_SWEEP_VERBOSE=1` prints each moment's timing.
- **Hooks:** `window.aic` (main.ts) goes through the buttons' own paths;
  `scripts/cdp.mjs` is the DevTools driver.
- **Running it:** give a background run a long timeout and serve the viewer from a
  process that outlives it. Don't rebuild `aic-sim` mid-sweep (newer records load with
  warnings). Its rules twin is the core test `invariants_hold_in_random_games`.
- **Times:** all cases at `AIC_SWEEP_SEEDS=1` (1,337 captures) took 25 min beside a planner
  run, `--quick` 6.7 min (r21). A step costs 7–30 ms, a capture about 1 s (three sizes).
  Run at most one sweep alongside one browser test lane.

## Odd input

`npm run odd` (`scripts/odd.mjs`) runs twelve scenarios against a served viewer
(`AIC_URL`), reloading between them: mashing, mid-choice reloads, resizes during
autoplay, full storage, broken records and sessions, bad seeds and the like. Fails on a
console error, a stuck game, a page scroll or a scenario's own check, and prints what
each bad input said. `--only NAME`, `--seeds N`. About 30 s; it counts as one browser
lane. Failure shots go to `sweep-out/odd/`.

## Source

In `src/`: `engine.ts` (typed wasm wrapper), `session.ts` (`Timeline`: replayed states,
cursor, undo, localStorage), `board.ts` (SVG board), `format.ts` (action and event labels,
colors), `panels.ts`, `hover.ts`, `log.ts`, and `main.ts` (wiring).

`card/` is the printed card renderer, used by `tools/card-art/` and the card viewer
(`cards/`), not by this game viewer: see [tools/card-art/README.md](../tools/card-art/README.md).
`scripts/shots-017.mjs`, `shots-019.mjs` and `shots-021.mjs` save their items' viewer
shots; how to run them is in those write-ups.

## Card viewer

```sh
cd web && npm run cards                            # exports, then serves http://localhost:5180/cards.html until stopped
```

Conductor's button runs that command in the main checkout and opens that URL: keep the
command, port and path as they are. It needs no `npm install`, wasm or cargo: plain Node
over its checkout. The export takes about 0.4 s. `--port N` serves elsewhere; a taken port
says so and exits.

- **Every card in content** (country pieces are map, not cards). Player cards go through
  `web/card/card.js` with their scene from `art-originals/`, the one
  `tools/card-art/scenes.json` names for that card (no scene: drawn without one); a kind
  with no frame gets a plain face from the same export. Scenes are full-size PNGs of
  about 3.6 MB, loaded as they scroll in, so a fast scroll shows gray for a second or two.
- **The export** (`web/scripts/cards-export.mjs`, to `out/cards-export.json`) reads
  `content/`, `web/card/faces.json`, `configs/baseline.json`,
  `docs/card-pricing-notes.json` and `experiments/results/`. It never runs `aic-sim docs`:
  a card edited without it shows its old text. **Rebuild** on the page re-exports in about
  0.1 s.
- **Stats** come from the newest `results.json` of 100+ games with a per-card `cards`
  table, in `experiments/results/` or the checkout's `out/experiments/` (highest revision,
  then most plain agents, then most games); now r34's `screen-080`. Each label carries its
  revision; the page warns (one collapsed *N warnings ▸* line) when it is behind the
  rules. The default agent is the ladder's highest. An item whose screen should feed the
  viewer commits its `results.json`, not only a report.
  - Kept % is keeps per 100 games, about the share of games a one-copy card was kept in
    (the report's kept ÷ offered counts draft packets only, but the opening deal is kept
    undrafted, so that ratio can pass 100%). Played % is played ÷ (kept + drawn). Win Δ comes
    from `games.csv`, which names only buildings standing at the end: blank for other
    cards, a correlation for buildings.
  - A CSV under `experiments/results/` with a `card` (or `id`) column and a column with
    `reason` in its name (`card-prices.csv` by convention) adds the reasoned price and
    printed minus reasoned. One whose `revision` is older than `RULES_REVISION` is passed
    over (no columns, no warnings; the CLI export names it). The only one,
    `screen-055-prices/card-prices.csv`, is at r23, so these columns are off until a price
    read at the current revision commits its own.
- **Notes:** `review/card-notes.json` in the checkout, git-ignored, never deleted by a
  cleanup. Keyed by card id: `{ name, kind, hash, flag, comment, text, at }`. `flag` is
  empty or one of `ok`, `too cheap`, `too dear`, `rework`, `unclear text`; `text` is the
  card's text at saving; `hash` is a SHA-256 prefix of the card's content row and face,
  key order aside (`contentHash` in the export). A planner reads this file directly; a
  note is stale when its `hash` differs from the card's in `out/cards-export.json`. Matt
  reviews in the planner chat, so the page has no *Review* filter. Closing or stepping
  saves the note, keeping the previous file as `review/card-notes.json.bak`; a broken
  notes file shows on the page and refuses saves.
- **Badges:** *changed since your note* when the hash moved (saving again clears it).
  *New* for a card missing from `review/card-seen.json`, until opened, noted or *Mark all
  seen*; the first run seeds that ledger with the whole set.
- **Pricing:** a card's one current line from `docs/card-pricing-notes.json`, under
  *Pricing* in the large view (format and rules: root CLAUDE.md). `aic-sim docs --check`
  refuses an id that is no player card.
- **Check:** `npm run cards:check -- --port N` (about 20 s) drives a headless browser
  through all of the above on a copy under `out/cards-check/` (screenshots there; content
  and real notes untouched). It also fails on a card overflowing at the last text size or
  title floor, a title leaving its plate, or text wrapping differently between zooms.
- **Card rules** (box capacity, the 7.5 pt goal) are in
  [docs/card-guidelines.md](../docs/card-guidelines.md). A test refuses any face printing
  `*` or "worth".
- **Text sizes:** `npm run cards:fit` (about 2.5 s, no server) puts every player card
  through the real renderer and `fitCards` headless and writes `out/card-fit.json`: per id,
  `top` and `effect` box sizes in pt (null: empty box), `title` in pt, `overflow` and
  `where`, and `tight` for a box inside its panel but short of the clearance. It prints
  cards below 7.5 pt and exits 1 on any overflow. It reads the working tree: re-run after
  an edit.

# web: the AI Complete viewer

A debug-first browser client on `aic-wasm`, Vite + TypeScript, no UI framework. It renders the engine's
**generic view** and offers only what `legal_actions` returns; a module's place in it:
[../docs/architecture.md](../docs/architecture.md#the-generic-view).

## Build and run

Needs the root README's *Toolchain*.

```sh
cd web
npm install                    # once per worktree
npm run dev [-- --port N]      # builds the wasm, then serves http://localhost:5173/ (or port N)
npm run wasm                   # only rebuild web/src/wasm/ from crates/aic-wasm
npm run check                  # typecheck (needs web/src/wasm/: npm run wasm first)
npm run build                  # wasm + typecheck + static site in web/dist/ (92 s cold)
```

- **The wasm build:** `scripts/build-wasm.mjs` runs `wasm-pack build crates/aic-wasm --target web --release` into
  `web/src/wasm/` (git-ignored); 74 s in a fresh target dir (background it there). `AIC_WASM_PROFILE=dev` builds
  unoptimized.
- **While the dev server runs**, an edit under `crates/` (tests and `aic-sim` included) or `content/` rebuilds the
  wasm and reloads the page; a new or edited file in `configs/` reloads it. A failed rebuild keeps the last good
  engine, the error in the server's terminal.

## Player view

*Player view* / *Debug view* (V) switches, remembered (`aic.view` in `localStorage`; a fresh browser opens the debug
view, as the scripts expect). The player view (`src/player/`), laid out for 2560×1440, hides the timeline, log, raw
data and random play; Undo and records stay.

- **Layout:** the question over the map; at the right a row per seat (resources, income, hand, buildings, nodes,
  *to act* / *done*: click one to show it), the Alignment and losses, the adversary row, each country's threats;
  across the foot the shown seat (the one deciding, else the first to act; a clicked one until the next action):
  resources, AI board and powers, hand, buildings, then a bar of standard actions, converters and *Done (pass)*.
- **Controls sit on the objects:** *Play* / *Sell* on a hand card, *Use* on a power or building, pay on a threat, a
  hex click for a hex answer; other answers in the question bar. A hand card opens printed (`web/card/`, in a shadow
  root: its `.card` style would clash) on hover. The draft shows the offered cards printed with their keep price.
- **The adversary's turn** opens a panel stepping through it a phase at a time, one line each, outlining the hexes
  the step changed (the map shows the board after the turn).
- **New modules:** an action no component draws is a button under *Other*; card groups, piles and values fall to
  the debug view's generic chips (`panels.ts`).
- **Checks:** `AIC_SWEEP_VIEW=player` sweeps it (`no-control` shows each seat to act in turn); `node
  scripts/player-play.mjs [--config three_seat.json] [--seed N] [--out DIR] [--size WxH]` plays a whole game by its
  controls (random, passing least) and saves the record.

## Using it

- **Play:** pick a config (any in `configs/`; `baseline.json` by default) and a seed, then *New game*. One person
  plays every seat, in any order in a simultaneous phase; draft phases offer no pass. *Pass every seat*, *Random
  action* (R) and *Random to end* speed debugging.
- **Choices:** the question shows in the amber bar. A hex answer highlights its hexes: click one.
  Two answers on one hex (a sacrifice: the ambassador or the node there) also get a button each (the hex takes the
  first). Other answers are buttons in the seat's panel. Hexes are named by country (`Cintamani #13`).
- **The board:** player pieces on a hex's upper row, the adversary's on the lower. Hover anything for details;
  hexes the shown step changed have an amber outline.
- **Actions:** each seat panel offers that seat's legal actions, the view's `standard` ones included. AI board tracks
  (`seats[].board`): solid spaces uncovered, dashed under nodes, boxed the last uncovered. A `vars` track no module
  labels folds under *Unlabeled tracks*.
- **Timeline:** Ctrl+Z undoes (every state is a replay of config + seed + actions). ⏮ ◀ ▶ ⏭, the slider,
  ← → Home End, Space to auto-play at 1x (4 steps a second), 4x or 16x; **Page Up / Down** jump between round
  starts. Clicking a log entry jumps to its step; acting from an earlier step drops the later actions, after
  asking. The log folds by phase; hovering a phase outlines its hexes.
- **The header** shows the engine version and rules revision. The error bar flags a saved session (`localStorage`,
  replayed on load) whose config changed or no longer loads, or a record from another revision; one that no longer
  loads or replays is kept (`copy(aic.keptSession())`).

## Game records

Format: [../docs/simulator.md](../docs/simulator.md), *Game records*.

*Open record…* or drop a file on the page (`aic-sim run ... --save DIR` writes them) to watch it from step 0; a
record whose replay differs from what was saved loads with a warning. *Record / bug report…* holds the current game
as a record with an optional note, to copy whole or up to the shown step, or takes a pasted one to *Load*.

## Browser check

`npm run e2e [-- <record.json>]` (`scripts/e2e.mjs`; `AIC_URL`, `AIC_BROWSER`, `AIC_SHOTS` atop it) drives
headless Edge or Chrome over the DevTools protocol against a served viewer (the dev server, or *Against a built
site*). It checks the panels and board, a baseline game (seed 1) to the end, the interactions above, no page
scroll at 1366×768, 1920×1080 and 2560×1440, words not ids, under 100 ms a step, a 4-country game (may scroll at
1366×768) and the given record; each `check(...)` names itself. About 15 s; shots in `web/e2e-out/`
(git-ignored). The face-up adversary card and threat checks run at round 2 (a fresh game has neither). `board
fills its pane` measures a fixed game (baseline seed 11, first legal action) a rules change can move: before
reading a layout change into a failure, diff two revisions' `npm run panes` (its pane heights at the three sizes).

`npm run e2e:corps` and `e2e:researcher`, run the same way, play a quiet baseline (settings atop their scripts)
for cubes, converters and the Researcher's board; `e2e:sources` checks a face-up threat's sources and a forged
payment refused.

**Against a built site:** a dev-server rebuild breaks a run mid-way (`reading 'view'`), so while the engine is
edited serve a snapshot: `node scripts/build-wasm.mjs && npx vite build && npx vite preview --port $PORT
--strictPort`, `AIC_URL=http://localhost:$PORT/` (after a `web/src/` edit, `npx vite build` alone, ~1 s). A killed
wrapper's `node` can hold the port (`--strictPort` fails): kill it. **An old revision side by side** (about 3
min): `git archive REV | tar -x -C out/rREV`, then in its `web/` `npm ci` and a snapshot on another port.

## The sweep

`npm run sweep [-- --quick] [-- --case NAME]` (`scripts/sweep.mjs`), the bug hunt's checker, checks the page at
the moments that matter over many games. It needs a served viewer (`AIC_URL`; *Against a built site*) and a
release `aic-sim` (`AIC_SIM`); its environment variables and defaults are atop the script.

- **Cases:** `CASES` in the script (`--quick` the first four, `--case c4-d0` one). For each, `aic-sim` saves
  records (greedy, or the case's agent); the sweep **watches** each step by step and **plays** as many
  fresh games through the page at random (`AIC_SWEEP_SEEDS` a case and mode, 3; simulated from seed
  `AIC_SWEEP_SEED`, 100).
- **Moments:** new rounds, prompts, the first of each key event and counter rise (`COUNTER_EVENTS` and its
  per-revision sets), the end: the checks at 1366×768, layout at the two larger sizes, and a screenshot (at most
  14 a game, plus the end and any failure).
- **Fail the run** (exit 1): console errors and blocking dialogs, `stuck` (no legal action without an outcome),
  the error bar, page scroll, placeholder text (`undefined`, `NaN`), elements off screen with no scroll to them,
  `hidden-target`, `clipped` (no ellipsis), `token-unknown`, `stale-word` (an old house word on screen: agents,
  Operatives), `case` (`aic-sim` could not play it). **Reported for a look:** `no-control` (a legal action with no
  control on screen), `raw-id`, `truncated-no-hover`, `truncated`, overlaps, undrawn view tokens, steps over
  150 ms, heap growth over 50 MB, `mark-over-number` (a highlight ring across a hex number) and
  `warn-at-start` (a value drawn as a warning at a game's first phase, as a full allotment drawn as danger).
- **Running it:** background it, the viewer served by a process outliving it; at most one sweep beside one browser lane. Don't rebuild `aic-sim` mid-sweep: its records load with a content-hash warning in the error
  bar, which fails the run (`error-banner`). Five cases at one seed (541 captures) take about 12 min, a capture
  1.3 s (three sizes). Rules twin: `invariants_hold_in_random_games`.
- **Output** in `web/sweep-out/`: `shots/`, `issues.json` (game, step, moment), `index.html`, `sheet-N.png`
  contact sheets (failures framed red), the records. `AIC_SWEEP_VERBOSE=1` times each moment.
  Hooks: `window.aic` (main.ts), `scripts/cdp.mjs`.

## Other checks and the source

- **Kept, never played:** `npm run probe -- DIR card_id...` (`scripts/legal-probe.mjs`): over saved records, the
  steps each card was held, legal to play and played. Legal, never played: the agents' valuation.
- **Odd input:** `npm run odd` (`scripts/odd.mjs`) runs the twelve scenarios listed atop it against a served viewer (`AIC_URL`) and prints what each bad input said. `--only NAME`, `--seeds N`. About 30 s,
  one browser lane.
- **Source:** `src/` is the viewer (`main.ts` wiring, `engine.ts` the wasm wrapper, `session.ts` the `Timeline`). `card/` is the printed card renderer, used by `tools/card-art/` and the card viewer (`cards/`):
  [tools/card-art/README.md](../tools/card-art/README.md).

## Card viewer

```sh
cd web && npm run cards   # exports, then serves http://localhost:5180/cards.html until stopped
```

Conductor's button runs that command in the main checkout and opens that URL: keep the command, port and path.
Plain Node over its checkout (no `npm install`, wasm or cargo); the export takes about 0.4 s. `--port N` serves
elsewhere; a taken port says so and exits.

- **Every card in content** (country pieces are map, not cards), player cards through `web/card/card.js` with the
  `art-originals/` scene `tools/card-art/scenes.json` names, if any; a kind with no frame gets a plain face. Scenes
  are 3.6 MB PNGs loaded on scroll: a fast scroll shows gray for a second or two.
- **The export** (`web/scripts/cards-export.mjs`, to `out/cards-export.json`) reads `content/`,
  `web/card/faces.json`, `configs/baseline.json` and the stats and pricing files below, never running `aic-sim
  docs`: a card edited without it shows its old text. **Rebuild** on the page re-exports in ~0.1 s.
- **Stats:** the newest `results.json` of 100+ games with a per-card `cards` table, in `experiments/results/` or
  the checkout's `out/experiments/` (highest revision, then most plain agents, then most games); labels carry the revision; a
  collapsed *N warnings ▸* line flags one behind the rules. Default agent: the ladder's highest. A screen meant for
  the viewer commits its `results.json`.
  - Kept % is keeps per 100 games (the opening deal is kept undrafted, so a report's kept ÷ offered can pass
    100%). Played % is played ÷ (kept + drawn). Win Δ comes from `games.csv`, which names only buildings standing
    at the end: blank for other cards.
  - A CSV under `experiments/results/` with a `card` (or `id`) column and a column with `reason` in its name
    (`card-prices.csv` by convention) adds the reasoned price and printed minus reasoned; one whose `revision` is
    older than `RULES_REVISION` is passed over silently (the CLI export names it). None is current.
- **Notes:** `review/card-notes.json` in the checkout (git-ignored), by card id (fields: `saveNote` in
  `cards-server.mjs`). `flag`: empty, `ok`, `too cheap`, `too dear`, `rework` or `unclear text`; `text`: the card's
  text at saving; `hash`: a SHA-256 prefix of its content row and face, key order aside (`contentHash` in
  `out/cards-export.json`). Closing or stepping saves (previous file kept as `.bak`); a broken notes file shows on
  the page and refuses saves.
- **Badges:** *changed since your note* when the note's `hash` is stale (saving again clears it); *New* for a card
  missing from `review/card-seen.json`, until opened, noted or *Mark all seen* (the first run seeds it with all).
- **Pricing:** a card's one current line from `docs/card-pricing-notes.json`, under *Pricing* in the large view
  (format and rules: root CLAUDE.md; `docs --check` refuses an id that is no player card).
- **Check:** `npm run cards:check -- --port N` (about 20 s) drives a headless browser through all that on a copy in
  `out/cards-check/` (screenshots there). It fails on a card overflowing at the last size or title floor, a title
  off its plate, text wrapping apart between zooms, or a deck card with no scene or scene image.
- **Card rules:** [docs/card-guidelines.md](../docs/card-guidelines.md); a test refuses a face with `*` or "worth".
- **Shots:** named cards in the large view, `node scripts/card-shots.mjs ID,ID [--port N] [--out NAME]` (in
  `out/NAME/`, about 1 s a card); card backs (`renderBack`, View: Backs, each tier's keep cost),
  `node scripts/back-shots.mjs [--port N] [--out NAME]` (`out/NAME/backs.png`).
- **Text sizes:** `npm run cards:fit` (about 2.5 s, no server) fits every player card headless with the real
  renderer into `out/card-fit.json` (the repo root's `out/`, as for every `out/` path here; fields atop
  `scripts/cards-fit.mjs`; the target box runs 7.5 down to 6 pt, taking no height from a building's top box). It
  prints cards below 7.5 pt and exits 1 on any overflow. It reads the working tree: re-run after an edit.

# web: the AI Complete viewer

A debug-first browser client on `aic-wasm`, Vite + TypeScript, no UI framework. It renders the engine's
**generic view** and offers only what `legal_actions` returns; a module's place in it:
[../docs/architecture.md](../docs/architecture.md#the-generic-view).

## Build and run

Needs the root README's *Toolchain*.

```sh
cd web
npm ci                         # once per worktree (npm run dev runs it when node_modules is stale)
npm run dev [-- --port N]      # serves http://localhost:5173/ (or port N), building a stale wasm
npm run wasm [-- --force]      # only rebuild web/src/wasm/ from crates/aic-wasm
npm run check                  # typecheck (needs web/src/wasm/: npm run wasm first)
npm run build                  # wasm + typecheck + static site in web/dist/ (92 s cold)
```

- **The wasm build:** `scripts/build-wasm.mjs` (`AIC_WASM_PROFILE=dev` unoptimized) into `web/src/wasm/`
  (git-ignored): 74 s cold, ~50 s (wasm-opt) warm; skipped when its inputs are unchanged (its header).
- **The dev server** (`scripts/dev.mjs`, conductor's *Play game*) builds the wasm at its start behind a waiting page,
  and on an edit under `crates/` or `content/`, then reloads; an edit in `configs/` reloads. A failed rebuild keeps
  the last good engine, the error in the server's terminal.

## Player view

*Player view* / *Debug view* (V) switches, remembered (`aic.view` in `localStorage`; a fresh browser opens the debug
view). The player view (`src/player/`) hides the timeline, log, raw data and random play.

- **Layout:** the round and question over the map; at the right a row per seat (click one to show it), the
  Alignment, the adversary, each country's threats; under the map the shown seat (the one deciding, else the one that
  just acted while it has a move in that phase, else the first to act): node powers, board, hand and buildings, then
  a bar of standard actions, Liberate's line (each corrupted site holding its ambassador: the bill, ready or what
  blocks it), converters and *Done (pass)*. From 2200 px wide the seat panel and bar run the full width.
- **Controls sit on the objects:** *Play* / *Sell* on a hand card, *Use* on a power or building, pay on a threat (the
  paying seat named), a hex click for a hex answer; other answers in the question bar. A hand card opens printed
  (`web/card/`, in a shadow root) on hover; hovers drop the debug view's keys. The draft shows the offered cards
  printed with their keep price, shrunk until the Draft / Keep buttons show unscrolled.
- ***Since your last move*** steps through the phases run, atop the side column: the notes, then the step list,
  each adversary step named by its beat (Card or Income). The map, tracks and rows show the board at the end of the
  shown step (`turnView`, from the engine's `apply_phases`), outlining the hexes it changed; the shown seat's hand,
  powers and controls are the board now. ← → step it while open.
- **New modules:** an action no component draws is a button under *Other*; card groups, piles and values fall to
  the debug view's generic chips (`panels.ts`).
- **Checks:** `AIC_SWEEP_VIEW=player` sweeps it. `node scripts/playthrough.mjs [--config three_seat.json] [--seed
  N] [--size 1366x768] [--search A-B] [--board N]` plays a shipped config by `play-policy.mjs` (`solo.json` started
  through the header's Config and Board selects and New game), clicking each move, shoots each
  phase, fails on the sweep's hard screen checks (`screen-checks.mjs`), a raw-JSON hover, ▶ moving between turn
  steps or the view leaving a seat with a move, and ticks each overhaul rule seen (2-4 min). `player-play.mjs` plays
  one at random; `troops-play.mjs` and `ambassadors-play.mjs` click a demo game to the win and require its rules
  (a guard, the Strike notice, *Liberate refused*; a move then Liberate, two node powers a round, a tipped ambassador).

## Using it

- **Play:** pick a config (any in `configs/`; `baseline.json` by default) and a seed, then *New game*. One person
  plays every seat, in any order in a simultaneous phase; draft phases offer no pass. *Pass every seat*, *Random
  action* (R) and *Random to end* speed debugging.
- **Choices:** the question shows in the amber bar. A hex answer highlights its hexes (purple): click one; two
  answers on one hex also get a button each.
- **The board:** player pieces on a hex's upper row, the adversary's on the lower; the legend keys the troops'
  links. Hover anything for details; hexes the shown step changed have an amber outline.
- **Actions:** each seat panel offers that seat's legal actions. AI board tracks: solid spaces uncovered, dashed
  under nodes, boxed the last uncovered. A `vars` track no module labels folds under *Unlabeled tracks*.
- **Timeline:** Ctrl+Z undoes. ⏮ ◀ ▶ ⏭, the slider, ← → Home End, Space to auto-play at 1x, 4x or 16x; **Page Up /
  Down** jump between round starts. Clicking a log entry jumps to its step; acting from an earlier step drops the
  later actions, after asking. The log folds by phase; hovering a phase outlines its hexes.
- **The header** shows the engine and rules revision. The error bar flags a saved session (`localStorage`, replayed
  on load) whose config changed or no longer loads, or a record from another revision; one that no longer loads is
  kept (`copy(aic.keptSession())`).

## Game records

Format: [../docs/simulator.md](../docs/simulator.md), *Game records*.

*Open record…* or drop a file on the page (`aic-sim run ... --save DIR` writes them) to watch it from step 0; a
record whose replay differs from what was saved loads with a warning. *Record / bug report…* holds the current game
as a record with an optional note, to copy whole or up to the shown step, or takes a pasted one to *Load*.

## Browser check

After editing a script, `node --check` it before a long run: in-page code sits in template strings, where a stray
backtick breaks the file (the fast lane's `node_scripts_parse` checks them all).

`npm run e2e [-- <record.json>]` (`scripts/e2e.mjs`; `AIC_URL`, `AIC_BROWSER`, `AIC_SHOTS` atop it) drives
headless Edge or Chrome over the DevTools protocol against a served viewer (the dev server, or *Against a built
site*). It checks the panels and board, a baseline game (seed 1) to the end, the interactions above, no page
scroll at 1366×768, 1920×1080 and 2560×1440, words not ids, under 100 ms a step, a 4-country game (may scroll at
1366×768) and the given record. About 30 s (r150); shots in `web/e2e-out/` (git-ignored). `board fills its pane`
measures a fixed game (baseline seed 11) a rules change can move: diff two revisions' `npm run panes` first.

`npm run e2e:corps`, `e2e:researcher` (cubes, converters, the Researcher's board) and `e2e:sources` (a threat's
sources, a forged payment refused) run the same way.

**Against a built site:** a dev-server rebuild breaks a run mid-way, so while the engine is edited serve a
snapshot: `node scripts/build-wasm.mjs && npx vite build && npx vite preview --port $PORT
--strictPort`, `AIC_URL=http://localhost:$PORT/` (a `web/src/` edit: `npx vite build`, ~1 s). A killed wrapper's
`node` can hold the port: kill it. **An old revision side by side** (~3 min):
`git archive REV | tar -x -C out/rREV`, then `npm ci` and a snapshot there. A `vite build` empties `dist/` under a
running sweep: rebuild between runs.

## The sweep

`npm run sweep [-- --quick] [-- --case NAME]` (`scripts/sweep.mjs`), the bug hunt's checker, checks the page at
the moments that matter over many games. It needs a served viewer (`AIC_URL`; *Against a built site*) and a
release `aic-sim` (`AIC_SIM`); its environment variables and defaults are atop the script.

- **Cases:** `CASES` in the script (`--quick` the first four, `--case NAME` each named). For each, `aic-sim` saves
  records; the sweep **watches** each step by step and **plays** as many fresh games at random.
- **Moments:** new rounds, prompts, the first of each key event and counter rise (`COUNTER_EVENTS`), the end:
  the checks at 1366×768, layout at the larger sizes, a screenshot (at most 14 a game, plus the end and failures).
- **Fail the run** (exit 1): console errors and blocking dialogs, `stuck` (no legal action without an outcome),
  the error bar, page scroll, placeholder text (`undefined`, `NaN`), elements off screen with no scroll to them,
  `hidden-target`, `clipped` (no ellipsis), `token-unknown`, `stale-word` (`STALE` in `screen-checks.mjs`), `case` (`aic-sim` could not play it), `below-fold` (the player view's Draft / Keep
  buttons or turn panel scrolled out of view). **Reported for a look:** `no-control` (a legal action with no
  control on screen), `raw-id`, `truncated-no-hover`, `truncated`, overlaps, undrawn view tokens, steps over
  150 ms, heap kept over 50 MB, `mark-over-number` (a highlight ring across a hex number) and
  `warn-at-start` (a value drawn as a warning at a game's first phase).
- **Running it:** background it, the viewer served by a process outliving it; at most one sweep beside one browser
  lane. Don't rebuild `aic-sim` mid-sweep (a content-hash warning fails the run). A case at one seed (a watched and a
  played game) takes 3-10 min, so a hunt picks `--case`s. Rules twin: `invariants_hold_in_random_games`.
- **Output** in `web/sweep-out/`: `shots/`, `issues.json` (game, step, moment), `index.html`, `sheet-N.png`
  contact sheets (failures framed red), the records.
  Hooks: `window.aic` (main.ts), `scripts/cdp.mjs`.

## Other checks and the source

- **Kept, never played:** `npm run probe -- DIR card_id...`: over saved records, the steps each card was held,
  legal and played.
- **Odd input:** `npm run odd` (`scripts/odd.mjs`, `--only NAME`, `--seeds N`; ~30 s) prints what each bad input said.
- **Cards played in the viewer:** `node scripts/cards-play.mjs ID,ID... [--seed 3] [--rounds 6]` (~1 min for 12)
  stages the cards in a rigged three-seat game (setup atop the script), clicks each Play and Use, and writes
  `report.md`: per card, its questions, notes, resource and map changes, a shot. Read it against the faces.
- **Source:** `src/` (`main.ts` wiring, `engine.ts` the wasm wrapper, `session.ts` the `Timeline`); `card/`, the
  printed card renderer: [tools/card-art/README.md](../tools/card-art/README.md).

## Card viewer

```sh
cd web && npm run cards   # exports, then serves http://localhost:5180/cards.html until stopped
```

Conductor's button runs that command in the main checkout and opens that URL: keep the command, port and path.
Plain Node over its checkout (no `npm install`, wasm or cargo); the export takes about 0.4 s. `--port N` serves
elsewhere; a taken port says so and exits.

- **Every card in content** (country pieces are map, not cards), player cards through `web/card/card.js` with the
  `art-originals/` scene `tools/card-art/scenes.json` names, if any; a kind with no frame gets a plain face. Scenes
  load on scroll (gray for a second).
- **The export** (`web/scripts/cards-export.mjs`, to `out/cards-export.json`) reads `content/`,
  `web/card/faces.json`, `web/card/adversary.json`, `configs/baseline.json` and the stats and pricing files below, never running `aic-sim
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
- **Shots:** `node scripts/card-shots.mjs ID,ID [--port N] [--out NAME]` (large view, `out/NAME/`, ~1 s a card);
  card backs, `node scripts/back-shots.mjs [--port N] [--out NAME]` (`out/NAME/backs.png`).
- **Text sizes:** `npm run cards:fit` (about 2.5 s, no server) fits every player card headless with the real
  renderer into `out/card-fit.json` (fields atop
  `scripts/cards-fit.mjs`; the target box runs 7.5 down to 6 pt, taking no height from a building's top box). It
  prints cards below 7.5 pt and exits 1 on any overflow or any box below 7.5 pt. A
  box first tries smaller icons (`icons-1`) with its text at 7.5 pt, then steps the text down. It reads the working tree: re-run after an edit. AI board faces (`aiBoard` in the export, `powersHtml` in play) aren't fitted. A box's
  size follows its text's length: joining two clauses' icons on one row saves nothing (258).

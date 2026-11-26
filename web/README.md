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

*Player view* / *Debug view* (V) switches, for this page only: **every load opens the player view** (Matt, Nov 25;
scripts start elsewhere with `aic.startView` in `localStorage`). **A fresh browser** (no saved
session) opens it on `solo.json` under the **new-game screen** (*New game…*): the tutorial first,
then solo on any board, then any config. The player view (`src/player/`) hides the timeline, log, raw data and random
play, and the header's form (the debug view's).

- **The tutorial** (`src/tutorial.ts`, `TUTORIAL`: solo, Builder, seed 24): prompts in `docs/teach.md`'s order,
  each outlining what it names, to round 2's Liberate and a hand-off. A "do" step waits for the view to show it done
  (*Skip step*); it suggests, never forces. `scripts/tutorial-play.mjs [--size WxH]` clicks it from a fresh browser to
  the hand-off, shooting each step; it fails on a stuck step or a misplaced prompt (~40 s).
- **Scenes:** the dev server downsizes each printed card's scene from `art-originals/` (`sceneArt` in
  `vite.config.ts`, `scripts/scene-thumb.py`, Pillow; cached in `out/scene-cache/`); a built site prints frames alone.

- **Layout (324, 326):** the map in the middle, under the round's **five steps** (Adversary · Cards · Actions · Income
  · Threats, the current lit; the engine's phases are the debug view's: `playerStep`, format.ts). Left, always shown:
  the adversary's cards (the next face up, with its notices), the turn's replay under them, projects (the leftmost
  whole), research row, then threats, Alignment and loss clocks. Right, *Your move*: the shown seat's legal actions
  only, *Done (pass)* at its foot; a power or Action it can't use is listed shaded, why on hover (`data-why`), and
  those unused this round are named atop it. The foot, a fixed height (the map keeps its share at 1366×768): a row
  per seat (the board's name opens its whole face), then the hand and buildings printed, shaded only on the seat's
  own move in the action phase. The shown seat: `focusSeat`. **No seat numbers:** a seat is its board's name (solo's
  "you": `seatNames`, format.ts).
- **The draft** shows over the map and the right panel's width, *Done keeping* in its head, its cards printed as
  large as one row fits, a caption only for what the face doesn't print (*Show the map* folds it to its head).
- **Undo** (Ctrl+Z) only inside the action phase, back to its start, never past a draw from a deck (`undoRoom`,
  main.ts); the header's unlimited Undo is the debug view's.
- ***Since your last move*** steps through the phases run, under the adversary's cards, each adversary step named by its
  beat; ***Replay the adversary's turn*** reopens the last one all round (D142). The map, tracks and rows show the
  board at the shown step's end (`turnView`, the engine's `apply_phases`); controls are the board now. ← → step it.
- **New modules:** an action no component draws is a button under *Other*; card groups, piles and values fall to
  the debug view's generic chips (`panels.ts`).
- **Checks:** `AIC_SWEEP_VIEW=player` sweeps it. `node scripts/playthrough.mjs` (flags atop it) plays a shipped config
  by `play-policy.mjs`, clicking each move (once a round an Undo and the adversary's replay), shoots each phase,
  fails on the sweep's hard screen checks (`screen-checks.mjs`) and its own (atop it), and ticks each rule seen
  (2-4 min). `player-play.mjs` plays one at random; `troops-play.mjs` and `ambassadors-play.mjs` click a demo game to
  the win and require its rules (a guard, the Strike notice, *Liberate refused*; Liberate, node powers, tipping).

## Using it

- **Play:** a config and a seed, then *New game*. Scripts (`cdp.mjs`'s `openPage`) open the debug view on
  `baseline.json` (`aic.startView`, `aic.firstConfig`) unless `fresh: true`. One person plays every seat, in any order.
- **Choices:** the question shows in the amber bar. A hex answer highlights its hexes (purple): click one; two
  answers on one hex also get a button each. Hexes the shown step changed have an amber outline.
- **Timeline:** Ctrl+Z undoes. ⏮ ◀ ▶ ⏭, the slider, ← → Home End, Space to auto-play at 1x, 4x or 16x; **Page Up /
  Down** jump between round starts. Clicking a log entry jumps to its step; acting from an earlier step drops the
  later actions, after asking. The log folds by phase; hovering a phase outlines its hexes.
- **The error bar** flags a saved session whose config changed or no longer loads (kept:
  `copy(aic.keptSession())`), or a record from another revision.

## Game records

Format: [../docs/simulator.md](../docs/simulator.md), *Game records*.

*Open record…* or drop a file on the page (`aic-sim run ... --save DIR` writes them) to watch it from step 0; a
record whose replay differs from what was saved loads with a warning. *Record / bug report…* holds the current game
as a record with an optional note, to copy whole or up to the shown step, or takes a pasted one to *Load*.

## Browser check

After editing a script, `node --check` it before a long run (in-page code sits in template strings). Before/after
shots of the solo cards and action phases: `node scripts/phase-shots.mjs --out DIR [--size WxH] [--config FILE]`.

`npm run e2e [-- <record.json>]` (`scripts/e2e.mjs`; `AIC_URL`, `AIC_BROWSER`, `AIC_SHOTS` atop it) drives
headless Edge or Chrome over the DevTools protocol against a served viewer (the dev server, or *Against a built
site*). It checks the panels and board, a baseline game (seed 1) to the end, the interactions above, no page
scroll at 1366×768, 1920×1080 and 2560×1440, words not ids, under 100 ms a step, a 4-country game (may scroll at
1366×768) and the given record. About 30 s (r150); shots in `web/e2e-out/` (git-ignored). `board fills its pane`
measures a fixed game (baseline seed 11) a rules change can move: diff two revisions' `npm run panes` first.

`npm run e2e:corps`, `e2e:researcher` (cubes, converters, the Researcher's board) and `e2e:sources` (a threat's
sources, a forged payment refused) run the same way.

**Against a built site:** a dev-server rebuild, or any `web/src/` edit (vite reloads the page), breaks a run mid-way,
so while editing serve a snapshot: `node scripts/build-wasm.mjs && npx vite build && npx vite preview --port $PORT
--strictPort`, `AIC_URL=http://localhost:$PORT/` (a `web/src/` edit: `npx vite build`, ~1 s). A killed wrapper's
`node` can hold the port: kill it. **An old revision:** `git archive REV | tar -x -C out/rREV`, `npm ci`, a snapshot
there. A `vite build` empties `dist/` under a running sweep.

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
  lane; no `aic-sim` rebuild mid-sweep. A case at one seed takes 3-10 min, so a hunt picks `--case`s.
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
  `art-originals/` scene `tools/card-art/scenes.json` names, if any; boards, event cards, projects and threats
  through `web/card/print.js`; Unity and Discord cards get a plain face. Scenes load on scroll.
- **327:** `/faces.html` shows every printed face beyond player cards; `/mockup.html` (`?size=1366`), the solo
  screen's static mockup, draws `web/mockup/state.json`, which `scripts/mockup-state.mjs` (flags atop it) writes
  from a saved record with the dev server up.
- **The export** (`web/scripts/cards-export.mjs`, to `out/cards-export.json`) reads `content/`, `web/card/faces.json`,
  `print-faces.json`, `adversary.json`, `configs/baseline.json` and the stats and pricing files below, never running
  `aic-sim docs`: a card edited without it shows its old text. **Rebuild** re-exports in ~0.1 s.
- **Stats:** the newest `results.json` of 100+ games with a per-card `cards` table, in `experiments/results/` or
  the checkout's `out/experiments/` (highest revision, then most plain agents, then most games); labels carry the revision; a
  collapsed *N warnings ▸* line flags one behind the rules. Default agent: the ladder's highest. A screen meant for
  the viewer commits its `results.json`.
  Column meanings: `buildStats` in `cards-export.mjs`. A CSV under `experiments/results/` with `card` and `reason`
  columns adds the reasoned price; one older than `RULES_REVISION` is passed over. None is current.
- **Notes:** `review/card-notes.json` in the checkout (git-ignored), by card id (fields: `saveNote` in
  `cards-server.mjs`; `hash`, `contentHash` in the export). Closing or stepping saves (the old file kept as `.bak`);
  a broken notes file shows on the page and refuses saves.
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
  renderer into `out/card-fit.json` (fields and the size ladder atop `scripts/cards-fit.mjs`); it exits 1 on any
  overflow or any box below 7.5 pt. It reads the working tree: re-run after an edit. AI board faces aren't fitted. A
  box's size follows its text's length: joining two clauses' icons on one row saves nothing (258).

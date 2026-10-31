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
npm install                     # once
npm run dev                     # builds the wasm, then serves http://localhost:5173/
npm run dev -- --port 61870     # another port
npm run wasm                    # only rebuild web/src/wasm/ from crates/aic-wasm
npm run check                   # typecheck
npm run build                   # wasm + typecheck + static site in web/dist/
npm run e2e [-- <record.json>] # browser check, dev server must be running (see below)
```

- **The wasm build step:** `scripts/build-wasm.mjs` runs `wasm-pack build
  crates/aic-wasm --target web --release` into `web/src/wasm/`, which is git-ignored.
  It takes about 20 s cold and 9 s warm. Set `AIC_WASM_PROFILE=dev` for an unoptimized
  build.
- **Engine changes while the dev server runs:** editing anything under `crates/` or
  `content/` rebuilds the wasm and reloads the page. Editing or adding a file in
  `configs/` reloads the page. The configs are read at build time
  (`import.meta.glob`), so a new config file appears in the picker after the reload.
- The page keeps the last good engine if a rebuild fails; the error is in the dev
  server's terminal.

## Using it

- **Play:** pick a config (any file in `configs/`; `baseline.json` is the default) and
  a seed, then *New game*. Each seat's panel lists that seat's legal actions. Any seat
  may act in any order during a simultaneous phase, and one person plays every seat.
  Growth and draft phases offer no pass: pick one of the buttons.
- **Choices:** when the rules wait for a choice, its question shows in the amber bar
  above the board. A choice answered with a hex highlights its hexes at once: click
  one. When two answers name the same hex (a sacrifice: the ambassador or the presence
  there), each also gets its own button (clicking the hex takes the first). Any other answer
  is a button in that seat's panel. Prompts, options and the log name hexes by country
  (`Northern Union #13`) and cards by name. *Pass every seat* passes
  all the seats still to act. *Random action* (R) and *Random to end* are for quick
  debugging.
- **Reading the board:** hexes are filled by what they are (`kind`: city grey, military
  base khaki, open land pale green, ocean blue; key at the bottom left), with a band in
  the country's colour along each country border and the printed hex number on top.
  A military base carries a pennant in the colour of the country across its front
  (`facing`). Industrial zones carry a dashed brown ring low on the hex. Fronts are
  drawn in dark red: a dashed line along each printed front (the land edges between two
  countries whose bases face each other; pieces touching only at the centre get none)
  and a dashed link between the two bases; on a war step (`board.war` = 1) they turn
  solid bright red and pulse, and the adversary row
  shows a **war round** badge. Each seat's colour is its country's.
  Players' pieces sit on the upper row of a hex: owner cube in the seat's colour (an
  aligned city or owned station), research station (teal dome), factory (brown),
  presence (server rack in the seat's colour), hardening token (grey shield), corp
  troops at a base (chevrons in the base country's colour) and player ambassadors (pawn
  in the seat's colour). The adversary's sit on the lower row, in purple: its owner cube
  (a corrupted city, station or factory), its ambassadors (pawn), corruption markers
  (a crossed lilac and purple disc), its nodes (diamond) and its troops (dark
  spearhead). The clock's step (`board.clock`), adversary compute
  and each seat's chips show as values. A number by a glyph is its count.
- **Around the board:** the Alignment track (the Unity bucket, blue, filling leftward
  and the Discord bucket, red, filling rightward on either side of the Undecided; each
  side's tip count with its next threshold; the
  clear-board bar level and its win condition), the Unity and Discord cards resolved so far (hover a chip for its
  text) with what is left in each deck, and the red **Lose if** group (Discord cards left, adversary cards left, and **presence gone** with the fewest presence of any seat, red at 1); the adversary row
  (stage, difficulty, adversary compute, threat tier and fronts, the adversary ambassadors and troops on the board, then deck →
  next round's card, face up → this round's card → discard, each card with its stage,
  text and escalation/war marks; under it the adversary's projects (017: each visible
  project a two-line box, name and compute bar then its effect text, cut at the end of
  the second line, all of it on hover; ongoing ones, the resolved count and the deck)); the countries strip with each country's name, its
  cities (clean, aligned, corrupted), adversary pieces and corp units, its megaproject
  card (one box per cube place, filled by owner: purple for the adversary, the seat's
  colour for a player; "completed" once done), its round flags (blackout, sabotage,
  shelter, embargo), its threat and counter progress (or the shared threat row; a threat that executes next
  round dims and says **waits**, and a jam shows its amount). Country names
  (the printed pieces', from the view's `countries[].name`) also label the board and the
  hover text.
- **Actions:** besides cards, threats and gifts, each seat panel offers the board's
  standard actions (`standard`: place presence, build a research station, remove an
  adversary ambassador, clear a corruption marker, move an ambassador, gain an
  ambassador, align a city, harden a building, sell a chip, the rich action (a presence
  at a steep price that rises with each use a round) and the standard projects: harden
  a building, gain 1 energy, remove compute from an adversary project) labelled with the words and costs the rules publish
  in the view (`standard`), a *power* button for each AI-board power, the *antimatter*
  action, and one *sell* button per card in hand. A standard action that needs a hex
  asks for it next, as a hex pick.
- **Seat panels:** the AI board's name (hover it for its text, innate rule, slots and
  unique cards), resources with income, then the board's two presence tracks with every printed space: solid spaces are uncovered, dashed ones still under presence, the boxed
  one is the last uncovered, and each space's marks say what it adds (its income, such as
  **+1 energy**, a green **T2 keep** where it lowers a tier-2 keep, and **pay N energy** where a placement pays its printed cost). Hover a step for its details. This is drawn from the generic
  `seats[].board` shape, not from AI-specific keys.
- **Globals** hold only game-wide tracks: modules keep per-seat and per-card
  bookkeeping out of them (`ViewOut::bookkeeping`). A `vars` track no module labels is
  folded under *Unlabelled tracks* below the piles; in the baseline there are none.
- **Undo** (Ctrl+Z) drops the last action. Every state is a replay of config + seed +
  actions, so undo never has to reverse anything.
- **Timeline:** ⏮ ◀ ▶ ⏭, the slider, ← → Home End, and Space to auto-play. The menu by
  *play* sets the autoplay speed: 1x (4 steps a second), 4x or 16x (kept in
  `localStorage`; it applies at once while playing). **Page Up / Page Down** jump to the
  start of the shown round (again: the round before) and to the start of the next round
  (or the last step), in a record or a game being played. Clicking an
  event-log entry jumps to that step. Acting from an earlier step drops the later
  actions, after asking first.
- **Hover** a hex, token, value, card or action button for its details; a hex's details
  list the troop arrows from and to it (the arrows themselves let the mouse through).
  Hovering a log entry outlines the hexes it touched. Hexes changed by the shown step
  have an amber outline.
- **Log phases:** a step that runs through phases (the adversary's whole turn after the
  last pass, a new round) lists its events in one folded group per phase, headed by the
  phase name with its event and hex counts; phases with no events show as a grey line.
  Hovering a group's heading outlines the hexes that phase changed; clicking it unfolds
  the group without moving the timeline. A step that stays in one phase lists its events
  flat.
- **The header** shows the engine version and the rules revision it plays. A saved
  session started with an older copy of its config file, or one that no longer loads,
  says so in the error bar; so does a record from another rules revision.
- **Data tabs:** *Event log*, *Raw state* (the engine's JSON, for anything the board
  doesn't draw), *View* (the generic view JSON) and *Config* (resolved config and hash).
- **Saved session:** the current game (config, seed, actions, cursor) is kept in
  `localStorage` and replayed on load.

## Game records

A record is the config, seed and action list, plus identity (hashes, engine version,
rules revision) and the saved outcome and final fingerprint. The engine writes it
(`aic_core::SavedGame`, format `aic-game-record/1`, one action per line):

- **Watch a simulated game:** `aic-sim run ... --save DIR` writes records (see the root
  README). Use *Open record…* or drop the file on the page. The game opens at step 0;
  step through it with the timeline. A record whose replay differs from what was
  saved (another rules revision or content) loads with a warning.
- **Bug report:** *Record / bug report…* holds the current game as a record, with an
  optional note. *Copy this game* copies it; *Copy up to shown step* stops at the
  cursor. Paste a record there and *Load* it to replay it. `aic-sim replay <file>`
  checks a record natively.

## Browser check

`npm run e2e [-- <record.json>]` (`scripts/e2e.mjs`; the record is optional) drives headless Edge or Chrome
over the DevTools protocol, with no extra packages. Point it at the dev server with
`AIC_URL` (default `http://localhost:5173/`), and at a browser with `AIC_BROWSER` if it
isn't at the usual path. It checks the board drawing (fills by hex kind, borders, base
pennants, fronts, every token stack drawn, owner colours distinct), the adversary row
with the face-up next card's text and the ambassador supply, the megaproject cards,
Alignment track and threats, country names, each seat's AI board with two tracks, and
that no bookkeeping var is among the globals, then plays a baseline game (seed 1) to
the end by clicking any action button (and a highlighted hex for hex choices), then
checks the following:

- hex choices highlight their hexes without a click;
- standard actions and selling a card were offered as buttons;
- war rounds are marked on the adversary row and the fronts;
- undo;
- the restored session;
- the record dialog;
- *Random to end* plays a new baseline game (seed 5) to an outcome;
- hover details;
- no page scroll at 1366×768, 1920×1080 and 2560×1440;
- (020) at the end of the played game, the log names hexes in words (no `C1 #13` in
  any line) and its action lines and the step label name cards in words (no
  `snake_case` ids), the header shows the rules revision, and stepping through the
  game stays under 100 ms a step;
- (022) Page Down / Page Up move between round starts, the speed menu offers 1x 4x 16x,
  and hovering a phase group's heading in the log outlines exactly its hexes;
- a 4-country game (64 hexes) still fits at 1366×768 and 1920×1080;
- the shared threat row (`configs/shared_row.json`), if that config exists;
- loading the record and stepping through it, if one was given.

Screenshots go to `web/e2e-out/` (git-ignored): the start, draft, growth, first
choice, mid-game and end of the game, the layouts, four countries and the shared row.
With `AIC_SHOTS=<dir>` the mid-game and end-of-game shots are also saved there as
`viewer-mid.png` and `viewer-end.png`. A run takes about 15 s (12-20 s measured).

- **Against a built site:** `npm run build`, then `npx vite preview --port <p>
  --strictPort` serves `web/dist/` with no engine watcher, so the run can't be reloaded
  under it (see the first gotcha). After changing only `web/src/`, `npx vite build`
  (about 1 s, reuses `web/src/wasm/`) refreshes `dist/`.

- **Gotcha:** the dev server rebuilds the wasm and reloads the page whenever anything
  under `crates/` or `content/` changes. An e2e run during such a rebuild fails with
  `Cannot read properties of undefined`; run it again once the build is done. Any
  `.rs` edit under `crates/` counts, tests and `aic-sim` included, so while someone
  else edits the engine, run the e2e against `vite preview` instead.
- `board fills its pane` measures a fixed game, baseline seed 11 played by the first legal
  action, not the random game: that one moved the same build by 60 px at 1366x768 (048). Its
  threshold comes from what that game reads. A rules change can move it: remeasure before
  reading a layout change into a failure.
- **Gotcha:** stopping a backgrounded `npm run dev` wrapper can leave the `vite`
  process serving the port; `--strictPort` then fails on the next start. Kill the
  process that holds the port.

## The sweep

`npm run sweep [-- --quick] [-- --case NAME]` (`scripts/sweep.mjs`, 020) plays and
watches many games at test speed and checks the page at the moments that matter. It
needs a served viewer (`AIC_URL`, default `http://localhost:5173/`; use a `vite preview`
snapshot while the engine is edited) and a release `aic-sim` (`AIC_SIM`, default
`../target/release/aic-sim[.exe]`).

- **Cases:** 14 in all (`CASES` in the script; `--quick` keeps the first four, `--case
  c4-d0` picks one): baseline at 1–4 countries and difficulties 0, 2 and 4, the shared
  row, the off- and still-variants, `c2-d1-engine`, `c2-d2-engine` and `c3-d3-engine`
  (the engine agent's fuller games: markers, shields, Clear, delays) and `overlay` (a
  config that overlays content). For each, `aic-sim` plays games (greedy unless the case
  names an agent) and saves them as records; the sweep **watches** each record step by step (wins, purges, late
  rounds) and **plays** as many fresh games of the same resolved config, picking
  legal actions at random through the page (choices, hex picks, now and then an undo).
  `AIC_SWEEP_SEEDS` games per case and mode (3; `--quick` 1).
- **Moments:** a new round, the round-1 phases, each new choice prompt, a war step,
  the first of each key event (tips, projects, corruption, destruction, nodes, marker
  placed and cleared, hardened, threat waits, stage N), the end. At each: a screenshot
  (at most 14 a game, plus always the end, a stuck game, an error, any failing check,
  and marker and waiting-threat moments) and the checks, at 1366×768 and again
  at 1920×1080 and 2560×1440 (layout checks only).
- **Checks that fail the run** (exit 1): console errors and blocking dialogs, no legal
  action without an outcome (`stuck`), the error bar, page scroll, placeholder text
  (`undefined`, `NaN`, ...), elements off the screen with nothing to scroll them into
  view, buttons covered by something else (`hidden-target`), text clipped without an
  ellipsis (`clipped`), a token with no glyph of its own (`token-unknown`), a case
  `aic-sim` could not play (`case`). **Reported for a look:**
  raw ids in the text (`raw-id`), truncated text with no hover (`truncated-no-hover`) or with one (`truncated`),
  overlapping buttons, hex numbers and counts on top of each other, tokens in the view
  not drawn, step-time spikes over 150 ms, heap growth over 50 MB across the run.
- **Output** in `web/sweep-out/` (git-ignored): `shots/`, `issues.json` (every issue
  with its game, step and moment, the spikes and the heap after each game pair),
  `index.html` and `sheet-N.png` contact sheets (24 captures each, red frame on a
  failing one), and the records it watched. `AIC_SWEEP_OUT` moves it;
  `AIC_SWEEP_VERBOSE=1` prints each moment and its timing.
- **Hooks:** `window.aic.play(configJson, seed)`, `open(recordText)`, `act(action)`,
  `seek(i)` and `error` (main.ts) go through the buttons' own paths; nothing changes
  for a user. `scripts/cdp.mjs` is the DevTools driver (no npm packages).
- **Running it:** give a background run a long timeout, and serve the viewer from a process
  that outlives it. A sweep spawns `aic-sim`, so don't rebuild it mid-sweep (records from a
  newer engine than the page's wasm load with warnings). Its rules twin is the core test
  `invariants_hold_in_random_games` (every action of random games: no negative piles, no
  pieces on the ocean, presence accounted for, a legal action until the end).
- **Times:** 044, 14 cases, `AIC_SWEEP_SEEDS=1` (1,337 captures): 25 min beside a planner run, `--quick` 6.7 min. 020, for the 10 cases then: `--quick` about 4.5 min;
  the full run with `AIC_SWEEP_SEEDS=2` (10 cases, 40 games, 1,305 captures) 19.5 min,
  so the default 3 seeds about half an hour.
  A step costs 7–30 ms (render included), a capture about 1 s (three sizes).
  Run at most one sweep alongside one browser test lane.

## Odd input

`npm run odd` (`scripts/odd.mjs`, 044) runs twelve scenarios against a served viewer
(`AIC_URL`), a page reload between each: mashing keys and buttons, *Random to end* twice,
reloading mid-choice, acting after a seek, resizing and switching tabs during autoplay, a
frozen page, full storage, broken records through the dialog, old and junk sessions, bad
seeds. Fails on a console error, a stuck game, a page scroll or a scenario's own check,
and prints what each bad input said. `--only NAME`, `--seeds N`. About 30 s. A failure's
screenshot goes to `sweep-out/odd/` (`AIC_SWEEP_OUT`). It counts as one browser lane.

## Source

| File | What it does |
|---|---|
| `src/engine.ts` | Typed wrapper over the wasm API: view, record and event shapes. |
| `src/session.ts` | `Timeline`: config + seed + actions with every replayed state, cursor, undo, localStorage. |
| `src/board.ts` | SVG hex board drawn from the view: hex kinds, borders, base pennants, fronts, token glyphs. |
| `src/format.ts` | Labels for actions (content names for cards, threats, growth options; standard actions with costs) and events; hex-kind and token colours. |
| `src/panels.ts` | Alignment track, adversary row, countries strip (megaprojects, threats), seat panel contents, piles. |
| `src/hover.ts` | Hover card text for hexes, tokens, piles, values and cards. |
| `src/log.ts` | One log entry: a step's action and its events, folded by phase. |
| `src/main.ts` | Wiring: choice prompt, action buttons, timeline, hover card, record dialog. |

`card/` is the printed card renderer, used by `tools/card-art/` and not yet by the
viewer: see *Card art* in the root README.

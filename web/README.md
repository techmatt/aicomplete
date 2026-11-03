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
npm run e2e:sources            # the threat-source check, the same way
npm run e2e:corps              # research cubes and the corps, the same way
```

- **The wasm build step:** `scripts/build-wasm.mjs` runs `wasm-pack build
  crates/aic-wasm --target web --release` into `web/src/wasm/`, which is git-ignored.
  It takes about 20 s cold and 9 s warm, over 60 s under load: background it there
  (74 s cold in a fresh target dir, measured at 073). Set `AIC_WASM_PROFILE=dev` for an unoptimized
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
  Draft phases offer no pass: pick one of the buttons.
- **Choices:** when the rules wait for a choice, its question shows in the amber bar
  above the board. A choice answered with a hex highlights its hexes at once: click
  one. When two answers name the same hex (a sacrifice: the ambassador or the presence
  there), each also gets its own button (clicking the hex takes the first). Any other answer
  is a button in that seat's panel. Prompts, options and the log name hexes by country
  (`Cintamani #13`) and cards by name. *Pass every seat* passes
  all the seats still to act. *Random action* (R) and *Random to end* are for quick
  debugging.
- **Reading the board:** hexes are filled by what they are (`kind`: city gray, military
  base khaki, open land pale green, ocean blue; key at the bottom left), with a band in
  the country's color along each country border and the printed hex number on top.
  A military base carries a pennant in the color of the country across its front
  (`facing`). Industrial zones carry a dashed brown ring low on the hex. Fronts are
  drawn in dark red: a dashed line along each printed front (the land edges between two
  countries whose bases face each other; pieces touching only at the center get none)
  and a dashed link between the two bases; on a war step (`board.war` = 1) they turn
  solid bright red and pulse, and the adversary row
  shows a **war round** badge. Each seat's color is its country's.
  Players' pieces sit on the upper row of a hex: owner cube in the seat's color (an
  aligned city or owned station), research station (teal dome), factory (brown),
  presence (server rack in the seat's color), hardening token (gray shield), corp
  troops at a base (chevrons in the base country's color) and player ambassadors (pawn
  in the seat's color). The adversary's sit on the lower row, in purple: its owner cube
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
  card (one box per cube, filled by owner: purple for the adversary, gray for a neutral station,
  the seat's color for a player; "completed" and the cubes that paid once done), its round flags (blackout, sabotage,
  shelter, embargo, cordon), its threat and counter progress (or the shared threat row; a threat that executes next
  round dims and says **waits**, and a jam shows its amount). Country names
  (the printed pieces', from the view's `countries[].name`) also label the board and the
  hover text.
- **Actions:** besides cards, threats and gifts, each seat panel offers the board's
  standard actions (`standard`: build a research station, remove adversary
  ambassadors, clear a corruption marker, move an ambassador, gain an
  ambassador, align a city, harden a facility, sell a chip and the standard projects: harden
  a facility, remove compute from an adversary project) labeled with the words and costs the rules publish
  in the view (`standard`), a *power* button for each AI-board power, the *antimatter*
  action, and one *sell* button per card in hand. A standard action that needs a hex
  asks for it next, as a hex pick.
- **Seat panels:** the AI board's name (hover it for its text, innate rule, slots and
  unique cards), resources with income, then the board's economy and card-play tracks with every printed space: solid spaces are uncovered, dashed ones still under presence, the boxed
  one is the last uncovered, and each space's marks say what it adds (its income, such as
  **+1 energy**, a green **T2 keep** where it lowers a tier-2 keep, a tag it adds such as **industry tag**, the capstone's name at a track's end, and **pay N energy** where a placement pays its printed cost). Hover a step for its details. This is drawn from the generic
  `seats[].board` shape, not from AI-specific keys.
- **Globals** hold only game-wide tracks: modules keep per-seat and per-card
  bookkeeping out of them (`ViewOut::bookkeeping`). A `vars` track no module labels is
  folded under *Unlabeled tracks* below the piles; in the baseline there are none.
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
  phase name with its event and hex counts; phases with no events show as a gray line.
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
pennants, fronts, every token stack drawn, owner colors distinct), the adversary row
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

Screenshots go to `web/e2e-out/` (git-ignored): the start, draft, first
choice, mid-game and end of the game, the layouts, four countries and the shared row.
With `AIC_SHOTS=<dir>` the mid-game and end-of-game shots are also saved there as
`viewer-mid.png` and `viewer-end.png`. A run takes about 15 s (12-20 s measured).

`npm run e2e:corps` (`scripts/e2e-corps.mjs`, 074) plays the baseline at `station_cubes` 3, with arrivals, city builds and troop marches off and
`corrupt_at` 20 so nothing disturbs it: a claimed and a neutral station, a card that overflows in round 2, stored
cubes, a usable converter beside an unusable one, and a use. `research cubes` and the converters show in a seat's panel.

`npm run e2e:sources` (`scripts/e2e-sources.mjs`, 070) plays first legal actions until a
face-up threat refuses a seat, then checks the threat's source badge, its sources marked on
the map, the refusal on the card and the engine's refusal of a forged payment. Same `AIC_URL`;
screenshot `web/e2e-out/threat-sources.png`.

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
| `src/format.ts` | Labels for actions (content names for cards, threats, powers; standard actions with costs) and events; hex-kind and token colors. |
| `src/panels.ts` | Alignment track, adversary row, countries strip (megaprojects, threats), seat panel contents, piles. |
| `src/hover.ts` | Hover card text for hexes, tokens, piles, values and cards. |
| `src/log.ts` | One log entry: a step's action and its events, folded by phase. |
| `src/main.ts` | Wiring: choice prompt, action buttons, timeline, hover card, record dialog. |

`card/` is the printed card renderer, used by `tools/card-art/` and the card viewer
(`cards/`), not yet by this game viewer: see [tools/card-art/README.md](../tools/card-art/README.md).

## Viewer notes

- A fresh worktree has no `web/node_modules`: run `npm install` first. Nor has it
  `web/src/wasm/`: `npm run check` needs `npm run wasm` first.
- `npm run build` (release wasm, typecheck, vite) took 92 s cold; `wasm-pack build
  crates/aic-wasm --target web` plus `node scripts/wasm-smoke.mjs` about 21 s; `npm run
  e2e` against a running dev server about 15 s.
- On a fixed port: `npx vite --port $PORT --strictPort` from `web/`, then
  `AIC_URL=http://localhost:$PORT/ npm run e2e`. At 073 it fails two board-size checks at 1366x768 (725x333 against 338, and the 4-country page): the board shrank earlier and the thresholds were left for a decision.
- While Rust or content files are being edited, the dev server rebuilds the wasm and
  reloads mid-run, which breaks e2e (`Cannot read properties of undefined (reading
  'view')`). Check against a snapshot instead: `node scripts/build-wasm.mjs && npx vite
  build && npx vite preview --port $PORT --strictPort`.
- A backgrounded `npx vite preview` outlives its tool-runner wrapper when that is stopped at
  its time limit, so the port stays taken: stop the `node` process that holds it.
- **The sweep** (020, the bug hunt's checker): `npm run sweep -- --quick` from `web/` (with
  `AIC_URL`; needs a release `aic-sim`) plays and watches games and checks the page at the
  moments that matter. Cases, times and gotchas:
  [The sweep](#the-sweep).
- `web/scripts/shots-017.mjs`, `shots-019.mjs` and `shots-021.mjs` save the viewer shots of
  their items; how to run them is in [experiments/history.md](../experiments/history.md).

## Card viewer

```sh
cd web && npm run cards                            # exports, then serves http://localhost:5180/cards.html until stopped
```

Conductor's button runs that command in the main checkout and opens that URL: keep the
command, port and path as they are. It needs no `npm install`, wasm or cargo: plain Node over
the checkout it runs in. The export takes about 0.4 s. `--port N` serves elsewhere (`npm run
cards -- --port 60476`). If the port is taken it says so and exits.

- **Every card in content:** programs, buildings, events, threats, Unity, Discord, adversary
  cards, projects, megaprojects and AI boards. Player cards go through `web/card/card.js` with
  their scene from `art-originals/` (no art folder, no scene, nothing else lost); a kind with
  no frame gets a plain face drawn from the same export. The scene is the one
  `tools/card-art/scenes.json` wrote for that card, its only link to a card. Country pieces are
  map, not cards, and are not shown.
- **The export** (`web/scripts/cards-export.mjs`, to `out/cards-export.json`) reads `content/`,
  `web/card/faces.json`, `configs/baseline.json`, `docs/card-pricing.md` and
  `experiments/results/`. It never runs `aic-sim docs`: faces are as committed, so a card
  edited without it shows its old text (the fast lane stops that reaching `main`). **Rebuild**
  on the page exports again without a restart, about 0.1 s.
- **Stats** come from the newest `results.json` of 100 games or more with a per-card `cards`
  table, in `experiments/results/` or the checkout's `out/experiments/` (highest revision, then
  most plain agents, then most games); the page says which and warns when it is behind the
  rules. Today that is r24 (`screen-055-prices`): later items committed reports, not `results.json`. Kept % is keeps per 100 games of the
  agent, about the share of games a one-copy card was kept in: the report's kept ÷ offered
  counts draft packets only, while the opening deal is kept undrafted, so it read 1000% on a card
  offered four times. Played % is the report's, played ÷ (kept + drawn). Win Δ comes from `games.csv`, which names only the buildings standing at the end, so it is blank
  for other cards and a correlation for buildings. A CSV under `experiments/results/` with a
  `card` (or `id`) column and a column with `reason` in its name, `card-prices.csv` by
  convention, adds the reasoned price and printed minus reasoned; a `revision` column prints in their labels, as the rules whose costs they were reasoned at. No results: no columns.
- **Notes:** `review/card-notes.json` in the checkout, git-ignored, never deleted by a
  cleanup. Keyed by card id: `{ name, kind, hash, flag, comment, text, at }`. `flag` is empty or
  one of `ok`, `too cheap`, `too dear`, `rework`, `unclear text`; `text` is the card's text when
  the note was saved; `hash` is a SHA-256 prefix of the card's content row and face, key order
  aside (`contentHash` in the export). A planner reads this file directly; a note counts as
  stale when its `hash` differs from that card's in `out/cards-export.json`.
- **Badges:** *changed since your note* when the hash moved; saving the note again takes it
  off. *New* for a card missing from `review/card-seen.json`, until it is opened, noted or
  *Mark all seen* is pressed. The first run seeds that ledger with the whole set, so nothing
  is new until content adds a card.
- **Pricing lines** are every table row led by the card's full name and every bullet and
  paragraph naming it, in the doc's order, with those that only pass by marked *in passing*
  and sorted last. A short form of a name
  ("Mutual Aid") doesn't match, and the doc's older sections keep the numbers of their day.
- **Leaving a card keeps what was typed:** closing or stepping saves the note. Each save leaves
  the previous file as `review/card-notes.json.bak`. A broken notes file shows on the page and
  refuses saves rather than being overwritten.
- **Check:** `npm run cards:check -- --port N` (about 20 s) drives a headless browser through
  all of the above on a copy under `out/cards-check/`; screenshots land there. Content and
  real notes aren't touched. It also fails if a card overflows at the last text size or the
  title floor, a title leaves its plate, or text wraps differently between zooms.
- **Card rules:** [docs/card-guidelines.md](../docs/card-guidelines.md). A building's box above the art
  and the one below it hold about 80 characters at 7.5 pt under one icon row; a program's or
  event's about 110. A test refuses any face printing `*` or "worth".
- **Text sizes:** `npm run cards:fit` (about 2.5 s, no server, no port) puts every player card
  through the real renderer and `fitCards` in a headless browser and writes
  `out/card-fit.json`: for each id, `top` and `effect` box sizes in pt (null: nothing in that
  box), `title` in pt, `overflow` and `where`, and `tight` for a box inside its panel but short
  of the clearance. It prints the cards below 7.5 pt and exits 1 if anything overflows. It reads
  content and `web/card/faces.json` as they stand in the working tree, so re-run it after an
  edit. **7.5 pt is the goal**: a smaller size is the ladder rescuing a card, not a target.
- Scenes are full-size PNGs of about 3.6 MB, loaded as they scroll in: a fast scroll shows gray
  windows for a second or two.

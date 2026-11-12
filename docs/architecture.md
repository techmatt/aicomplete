# Architecture

How the engine is put together, and how to change the game. The
rules are in [rules/](rules/README.md).

## Crates

- **`aic-core`**: the rules; no rules live anywhere else.
- **`aic-sim`**: the native CLI ([simulator.md](simulator.md)), agents
  ([agents.md](agents.md)) and experiment harness ([experiments.md](experiments.md)).
- **`aic-wasm`**: wasm-bindgen adapter, JSON strings in and out.
- **`aic-farm`**: the farm's wasm bundle ([simulator.md](simulator.md), *The farm*).
- **`web/`**: the viewer ([web/README.md](../web/README.md)); it draws the
  [generic view](#the-generic-view) and offers only `legal_actions`.

## Core invariants

- **No I/O**: no files, network, env vars, stdout or logging. Content is embedded at
  compile time from `content/`; configs are handed in by the caller.
- **No clock**: no `std::time`, `Instant` or `SystemTime`.
- **No global RNG**: no `thread_rng`, `rand::random` or `getrandom`. Randomness comes
  only from per-purpose streams in `GameState::rng`, keyed by the seed and a hash of the
  purpose name, so adding or reordering modules never shifts another's numbers. Each
  variant declares its purposes (`VariantDef::rng`), none shared between modules. A rule reached from another module's step
  draws through `Cx::deck_rng` or `Cx::threat_rng`.
- **Fixed-width draws**: `Stream` exposes only `below(n)`, on `u32`; rand draws a
  `usize` differently on native and wasm32, so never draw one through rand.
- **No hash-order leaks**: every map is ordered (`BTreeMap`, or `FlatMap` in the state);
  iteration follows seat, piece or `Dir::ALL` order; never `HashMap`/`HashSet` in anything
  that reaches the state or results. The `Key` interner is the one global, never iterated.
- **Serializable state**: `GameState` round-trips through JSON; the seed and RNG
  positions are decimal strings, too big for JavaScript numbers.
- **Exact replay**: config + seed + actions rebuild the identical state; the sim checks
  every game.
- **Native = wasm**: `scripts/wasm-smoke.mjs` checks the wasm build reaches the
  `golden_game` fingerprint (README, *Build and test*).
- **Printable state**: everything the rules read a physical copy could hold; a module
  needing more needs a design look first. `rng` (the dice), `stats` (never
  read by rules) and `pending` (the open choice) are not game components.

## API

Compile `Rules::new(&config)` once per config; the hot path is `legal_actions_into`
and `apply_mut(&mut g, &action, None)` (no event log). The free
functions `new_game`, `legal_actions` and `apply` rebuild `Rules` every call (a UI's, not a
sweep's) and run `check_state` first. `driver::play` plays a game with one team `Agent` for every seat;
`driver::replay` rebuilds one.

## Config

A config is JSON ([`configs/baseline.json`](../configs/baseline.json)): `modules` (at
most one instance per module id, in one variant, with its `params` and, for
`uses_cards` variants, `cards`), and `round`.

- **Every** parameter must be given: no defaults in code. Unknown, missing, wrong-kind
  and out-of-range values are errors naming it.
- **Integers only** (printable). A `list` is a table (by
  difficulty, tier, country or track).
- A card belongs to at most one module; leaving one out removes it from the game.
- `round` lists `"<module>.<phase>"`, each any number of times; `"a.x+b.y"` combines
  several modules' **decision** phases into one simultaneous phase.
- **Config hash**: `Rules::config_hash`, FNV-1a over the resolved config (`name`
  excluded); lists hash as written.
- **Overrides** ([simulator.md](simulator.md), `--set`): `<module>=off` fails if another `requires` it; its `off` *variant* keeps
  the phases and parameters others read.
- **Content overlay** ([experiments.md](experiments.md), *Content levers*): the config's
  `content` list. `Rules::content_hash` is the
  overlaid hash, `aic_core::content_hash()` the embedded one; an empty overlay hashes as absent.

## Modules

A **module** is one rules area; a **variant** is one way of doing it:

| Change | Kind |
|---|---|
| A number inside a mechanic | **parameter**: config only |
| Another way of doing it | **variant**: code, selected by config |
| A new mechanic | **module**: code, added to the config |
| A card, threat, board or piece of existing primitives | **content**: data (`content/`) |
| A new effect primitive | an `Op` ([The op schema](#the-op-schema)) |
| A new kind of player action | `Action`; a structural change |

Modules (`crates/aic-core/src/modules/`) call each other's public functions and read
each other's parameters through `World::int("<module>.<param>")` (a few hot ones also
have a slot, `World::hot(Hot::...)`), declared in `ModuleDef::requires`.

**One path per piece change**, so supply and cancellation hold
everywhere: adversary ambassadors are placed and removed only through `adversary`'s
functions, `board::liberate` and effects' `cleanse`; a node is placed by
`board::place_node`; nodes and facilities are destroyed by `board::destroy_piece`, which
applies every protection and sways the Discord (`board::lose_node` for losses no
protection absorbs).

A variant is a `VariantDef` (`module.rs`; its `phases` are `Auto` or `Decision`); the
`Module` hooks are documented on the trait. The trap (on `Module::act`): check
legality before the first `choose`: a `Halt::Illegal` after an answer panics. Modules change the state only through `Cx` (event
log and counters stay consistent). Counter and var names are `"<module>.<name>"`.

### Choices ("players choose")

Every point where the rules say players choose is a choice in `legal_actions`:

- A module calls `cx.choose(seat, prompt, options)?` mid-resolution; unless the answer is
  given or forced, the first pass stops with `Halt::Ask` (more on `Cx::choose`).
- The engine stores `GameState::pending` (the question, the answers so far and a
  snapshot from before); `legal_actions` is only the options (`Action::Choose`), no pass.
- An answer restores the snapshot and resolves again from the start, replaying the
  answers, so resolution code runs straight through on the state and the answers alone.

### The round

Each `Auto` phase resolves at once, a `Decision` phase waits until every seat has
passed, and an open choice waits for its answer. The game ends at `Cx::end`, or as a
`round_limit` loss after `MAX_ROUNDS`. **Decision phases are simultaneous**: seats act in
any order and `legal_actions` lists every seat's actions, as at a table where the team
talks it through.

### Adding a variant or module

- **Variant**: a `VariantDef` in the module's `DEF.variants` with a build function and a
  `Module` impl.
- **Module**: `modules/<id>.rs` with `pub static DEF: ModuleDef`, listed in `REGISTRY`
  (`modules/mod.rs`), with a config entry and its phases in `round`. For player input,
  reuse an `Action` (claimed by `owns`) and `cx.choose`; add an `Action` variant only if
  neither fits. A `view` hook adds only what the engine doesn't show ([The generic view](#the-generic-view)).
- Either way: add it to `configs/variants.json` (the core tests' `variant_matrix`, `wasm-parity.mjs` and
  `same_play.py` read it), bump
  `RULES_REVISION` if existing games change, and document it in `docs/rules/`.

### Adding content

Content fields are the serde structs in `content.rs`. What isn't plain there:

- **Player cards** have no `text`: `aic_core::face` generates it from the fields. `tokens` names a token type only: no card
  gains tokens by itself (Matt's ruling 12); "here" in an op names this card. `keywords`
  are never printed. Every op and where it prints: [ops.md](ops.md), from op.rs's `///` comments.
- **AI boards**: `tags` count in `cards::tag_counts`; slots and starting credits are config lists
  ([ai-boards.md](rules/ai-boards.md)), so a board's sheet spans both files.
- **Country pieces** (`map.json`) carry the corp's megaproject; `Megaproject::words`
  prints its op lists, so a new op there needs its words.

Actions name content by id (unique across all content), so reordering a file never
breaks action logs. `Content::check` validates references and each op's `Op::slots`.

### The op schema

Everything content does is one enum, `Op` (`op.rs`), read from JSON as
`{"op": "<snake_case>", ...}`. An op means one thing wherever it is printed; a context
(`effects::Ctx`) supplies who chooses, who is hit where, and (for an op shared by threats
and team cards) wording and tally. Three kinds:

- **Instant** ops resolve in `effects::run`, the one dispatch (`modules/effects/mod.rs`);
  `effects::feasible` says whether a player's card has a target.
- **Standing** ops (a building's `passives`, an AI's `innate`) are read through
  `cards::passives` by the rule they change, while the card is in force. A `when` is a
  trigger: `cards/triggers.rs` fires its instant ops on its event (`op::Event`;
  what one seat's action removed is diffed by `cards::fighting`, the one wrapper every action goes through).
- **Twists** (an event card's) are read through `adversary::twists` while it is current; its immediate ones and
  `standing` lines resolve in `adversary.build`.

**A new op parameter** touches `op.rs` (the field, `slots()`), `content.rs` (its check), `face.rs`,
its dispatch (`effects/mod.rs`, whose arm keeps the old behavior by matching the field's default and sends the
rest to `effects/new_ops.rs`; or the rule that reads a standing op), and its price: `agents/value.rs`, `agents/strategy.rs`, `configs/agents/values.json` and the
ledger's hand mirror, `scripts/cards/effect_ledger.py`. A new op touches the same, plus `effects/feasible.rs` (what
it acts on) and its harms row (`docs/harms.rs`, `op_harms`: every op has an arm, 191). The compiler finds the Rust
sites (`face.rs` lists every non-player op); the
mirror test finds the ledger.

### Generated docs

`aic-sim docs` rewrites the `<!-- generated: NAME -->` blocks in [rules/](rules/README.md)
(each in one file, listed in `BLOCKS`) and the `strategies` block of [agents.md](agents.md),
all of [ops.md](ops.md), [harms.md](harms.md) and [player-aid.md](player-aid.md), and
`web/card/faces.json`; `python scripts/cards/cards_doc.py` reads the faces and writes
[cards.md](cards.md) (the fast lane checks it); `scripts/content/load_adversary.py` writes the adversary's `content/` from
`drafts/adversary/` and [adversary-cards.md](adversary-cards.md). A block is a pure function of `Rules` or `Content` in
`crates/aic-sim/src/docs/blocks.rs`, named in `run` (`run.rs`), tested in `tests.rs`.

`aic-sim docs` (written or `--check`) fails on a threat tier with under 5 kinds of harm (`harm_failures`; 191: a thin harm row or a node-share
over half is a gap the ledger lists, not a failure), a missing or doubled
block, a doc page (CLAUDE.md's list) over 15 KB, a stale rules-revision line or card quote in `docs/rules/`, a
British spelling or stale house word (`spelling.rs`), a heading like `## 080's changes` (`current_docs.rs`) and an
id in `docs/card-pricing-notes.json` that is no player card. `--check`, and a fast-lane test, also refuse a stale
file, and in a rules file a backticked `module.param` or bare `snake_case` name
that is neither a tunable nor an op (`NOT_TUNABLES` lists the others it may print); the rules name a tunable, not its value. Two more fast-lane tests refuse a face
icon-line word `web/card/icons.js` has no icon for (`tests/face.rs`), and a committed
`experiments/results/effect-ledger/cards.csv` that values a card otherwise than
`agents/value.rs` (`effect_ledger_mirrors_the_table`; card-pricing.md says what to rerun).

## The generic view

Clients must not need code per module. `Rules::view(&state)` (wasm `view(state)`) turns
any state into the fixed shapes in `view.rs`. Map
geometry is not in the state: `Map::build` derives it (and a military base each side of
each printed front) from `content/map.json`; UIs get it from wasm's `board(config)`.

The engine fills every token stack, each seat's resources, exhausted cards no module
shows (`engine.exhausted`) and every `vars` track as a global. `Module::view` adds, through
`ViewOut`, only what it can't derive: derived values, card groups, piles, a seat's `board`,
standard actions, arrows, and `bookkeeping` (per-seat, per-country or per-card `vars`, a
key ending in `.` a prefix, shown where they belong). An undeclared key stays a global.

A card's `status` is a word the module chooses; the viewer styles `ready`, `used`,
`unaffordable` and `no target` and shows any other as text. The view is display data:
never read by the rules, not in the state, fingerprint or replay; a hook can't draw from
the RNG.

**What the client assumes.** Unknown token kinds draw as counters. Only `Action::Choose`
with a hex target arms a hex pick; any other action is a button labeled from its fields
(a hex field under another name needs viewer work).

**Records.** `SavedGame` (`record.rs`) is the one format for a sim save, a bug report
and a viewer game ([simulator.md](simulator.md)).

## Observability

- **Event log**: `Event`s from `apply` and `new_game_logged`, for UIs and the sim's
  `--trace`.
- **Counters** in `GameState::stats` are left out of the fingerprint (README, *Rules changes*). `adversary.ops` is the proxy for table time.
- **`PlayedGame`** carries the outcome (a stable `reason` code), decision
  counts and the game's identity.

## Known gaps

- **No interception hooks** between modules ("before a building is destroyed"), and
  `Event` is a closed enum; modules call each other instead.
- **No state schema version** (only `Stats`, `piles` and `pending` default): an older
  engine's state may fail to load; records are the durable format.
- **Snapshots cost**: every action and automatic phase first clones the state for a
  choice to replay from.

# Architecture

How the engine is put together, and how to change the game without fighting it. The
rules themselves are in [rules.md](rules.md).

## Crates

- **`aic-core`**: the rules. Pure and deterministic; no rules live anywhere else.
- **`aic-sim`**: the native CLI ([simulator.md](simulator.md)), agents
  ([agents.md](agents.md)) and experiment harness ([experiments.md](experiments.md)).
- **`aic-wasm`**: wasm-bindgen adapter, JSON strings in and out. No rules.
- **`web/`**: the viewer ([web/README.md](../web/README.md)); it draws the
  [generic view](#the-generic-view) and offers only `legal_actions`.

## Core invariants

- **No I/O**: no files, network, env vars, stdout or logging. Content is embedded at
  compile time from `content/`; configs are handed in by the caller.
- **No clock**: no `std::time`, `Instant` or `SystemTime`.
- **No global RNG**: no `thread_rng`, `rand::random` or `getrandom`. Randomness comes
  only from per-purpose streams in `GameState::rng`, keyed by the seed and a hash of the
  purpose name, so adding or reordering modules never shifts another's numbers. Each
  variant declares its purposes (`VariantDef::rng`); `Cx::rng` panics on an undeclared
  one, and two modules may not share one. A rule reached from another module's step
  draws through `Cx::deck_rng` or `Cx::threat_rng`.
- **Fixed-width draws**: `Stream` exposes only `below(n)`, on `u32`. rand samples a
  `usize` from a `u64` natively and a `u32` on wasm32, so native and browser games
  would differ: never draw a `usize` through rand.
- **No hash-order leaks**: every map is ordered (`BTreeMap`, or `FlatMap` in the state),
  iteration follows seat, piece or `Dir::ALL` order; never `HashMap`/`HashSet` in anything that reaches the state or results. The `Key`
  interner is the core's one global, a pointer cache never iterated.
- **Serializable state**: `GameState` round-trips through JSON; the seed and RNG
  positions are decimal strings, too big for JavaScript numbers.
- **Exact replay**: config + seed + actions rebuild the identical state; the sim checks
  every game.
- **Native = wasm**: the `golden_game` test pins a fixed game's final fingerprint, and
  `scripts/wasm-smoke.mjs` checks the wasm build reaches the same one.
- **Printable state**: everything the rules read is something a physical copy could
  hold: tokens on hexes, resources on seat boards, exhausted cards, ordered card piles, a
  round and phase marker, numbered tracks (`vars`). No hidden bookkeeping; a module
  needing more needs a design look first. `rng` (the dice), `stats` (observation, never
  read by rules) and `pending` (the open choice) are not game components.

## API

Compile `Rules::new(&config)` once per config; the hot path is `legal_actions_into`
(reuses a buffer) and `apply_mut(&mut g, &action, None)` (no event log). The free
functions `new_game`, `legal_actions` and `apply` rebuild `Rules` every call (fine for
a UI, wrong for a sweep) and run `check_state` first, so a mismatched state is an error,
not a panic (in wasm, an `unreachable` trap). `driver::play` plays a game with one team
`Agent` for every seat; `driver::replay` rebuilds one.

## Config

A config is JSON ([`configs/baseline.json`](../configs/baseline.json)): `modules` (at
most one instance per module id, in one variant, with its `params` and, for
`uses_cards` variants, `cards`), and `round`.

- **Every** parameter must be given: no defaults in code. Unknown, missing, wrong-kind
  and out-of-range values are errors naming it, so a typo can't
  silently test a default.
- **Integers only**, since every number must be printable. A `list` is a per-round
  schedule (the last entry repeats, `module::scheduled`) or a table.
- A card belongs to at most one module; leaving one out removes it from the game.
- `round` lists `"<module>.<phase>"`, each any number of times; `"a.x+b.y"` combines
  several modules' **decision** phases into one simultaneous phase.
- **Config hash**: `Rules::config_hash` is FNV-1a over the *resolved* config (round
  filled in, lists normalized, `name` excluded). Lists hash as written: `[1,2,3]` and
  `[1,2,3,3]` play alike but hash differently.
- **Overrides** (`Config::apply_override`; syntax in [simulator.md](simulator.md)):
  `<module>=off` is rejected for a module another `requires`; measure a module's effect
  with its `off` *variant*, which keeps its phases and the parameters others read.
- **Content overlay** ([experiments.md](experiments.md)), an experiment lever (a winner
  moves into `content/`): kept in the config's `content` list, so a config replays whole.
  `content::overlaid` keeps each distinct overlay for the process (`Rules` hold
  `&'static Content`). `Rules::content_hash` is the overlaid hash,
  `aic_core::content_hash()` the embedded one; an empty overlay hashes as if absent.

## Modules

A **module** is one rules area; a **variant** is one way of doing it:

| Change | Kind |
|---|---|
| A number inside a mechanic | **parameter**: config only |
| Another way of doing it | **variant**: code, selected by config |
| A new mechanic | **module**: code, added to the config |
| A card, threat, board or piece of existing primitives | **content**: data (`content/`) |
| A new effect primitive | an `Op` with its `slots` and its arm in `effects::run` (standing or twist: the rule that reads it) |
| A new kind of player action | `Action`; a structural change |

Modules (`crates/aic-core/src/modules/`) call each other's public functions and read
each other's parameters through `World::int("<module>.<param>")`; `ModuleDef::requires`
lists the modules whose parameters one reads. `plan.rs` holds the adversary build's
placement rule and `next_build`, a forecast only tests call.

**One path per piece change**, so supply, cancellation and botnet-making hold
everywhere: adversary ambassadors are placed and removed only through `adversary`'s
functions (`place`, `build_all`, `spread`, `corrupt_*`, `remove*`, `damage`, ...),
`board::liberate`/`free_cities` and effects' `cleanse`; `build_all` places what
`plan::build_plan` reads. A node is placed by `board::place_node`; nodes and facilities are
destroyed by `board::destroy_piece`, which applies every protection and sways the
Discord (corruption, sacrifice, Eviction and war's factory-hex hit call
`board::lose_node`, unabsorbed).

A variant is a `VariantDef` (`module.rs`): `params`, `phases` (`Auto` or `Decision`),
`rng` purposes, `uses_cards`, and `build` (validated params to a typed
`Box<dyn Module>`). The `Module` hooks are documented on the trait; a seat `can_pass`
refuses (draft picks) the module must pass itself. The trap: `Halt::Illegal` from `act`
restores the state only before any answered choice; after one it panics, so check
legality before the first `choose`. Modules change the state only through `Cx`, which keeps the event log and
counters consistent. Counter and var names are `"<module>.<name>"`.

### Choices ("players choose")

Every point where the rules say players choose is a choice in `legal_actions`, made
like any other action:

- A module calls `cx.choose(seat, prompt, options)?` mid-resolution. No options gives
  `None`, one gives it without asking; otherwise the first pass stops with `Halt::Ask`.
- The engine stores `GameState::pending` (the question, the answers so far and a
  snapshot from before). The shown state is partly resolved; `legal_actions` is only
  the options (`Action::Choose`), no pass.
- An answer restores the snapshot and resolves again from the start, replaying the
  answers. So resolution code is written straight through, never stores where it
  stopped, and depends only on the state and the answers. Replayed events are not
  logged twice. Answers are ordinary actions, so replay and records need nothing extra.

### The round

Each `Auto` phase resolves at once, a `Decision` phase waits until every seat has
passed, and an open choice waits for its answer. The game ends at `Cx::end`, or as a
`round_limit` loss after `MAX_ROUNDS`.

**Decision phases are simultaneous.** Seats act in any order until all pass;
`legal_actions` lists every seat's actions, seat by seat. Actions need not commute
(pooled counters, giving, the shared board): order matters the way it does at a table
where the team talks it through.

### Adding a variant or module

- **Variant**: a `VariantDef` in the module's `DEF.variants` with a build function and a
  `Module` impl (or one struct with an option, as `threats::Threats` does `shared`).
  Compare it with `aic-sim compare` or `--set-b <module>.variant=<name>`.
- **Module**: `modules/<id>.rs` with `pub static DEF: ModuleDef`, listed in `REGISTRY`
  (`modules/mod.rs`), with a config entry and its phases in `round`. For player input,
  reuse an `Action` (claimed by `owns`) and `cx.choose`; add an `Action` variant only if
  neither fits. The viewer shows its tokens, resources, `vars` and exhausted cards with
  no work; a `view` hook adds the rest.
- Either way: add it to `variant_matrix` (`crates/aic-core/src/tests/mod.rs`), bump
  `RULES_REVISION` if existing games change, and document it in `docs/rules.md`.

### Adding content

Content fields are the serde structs in `content.rs`. What isn't plain there:

- **Player cards** have no `text`: `aic_core::face` generates icons and words from the
  fields, and a new op doesn't compile until it has a face. `tier` 0 is an AI's unique
  card or starting building. `tokens.per_round` is 0 on every card (Matt's ruling 12),
  and "here" in an op names this card. `keywords` are read only by
  `scripts/cards/cards_doc.py` and the viewer, never printed. Adding a card: CLAUDE.md;
  every op and where it prints: [ops.md](ops.md).
- **AI boards**: printed `tags` count for requirements only. Slots and starting credits
  are config lists read by seat (`ai.slots`, `ai.start_credits`), so a board's sheet
  spans both files.
- **Country pieces** (`map.json`) carry the corp's megaproject; `Megaproject::words`
  prints its op lists, so a new op there needs its words.

Ids are unique across all content and actions refer to content by id, so reordering a
file never breaks action logs. Tags, resources and token types are enums (`vocab.rs`),
so a misspelling fails to load; `Content::check` validates references and that every op
sits in a list its `Op::slots` allows.

### The op schema

Everything content does is one enum, `Op` (`op.rs`), read from JSON as
`{"op": "<snake_case>", ...}`. An op means one thing wherever it is printed; a context
(`effects::Ctx`) supplies who chooses and who is hit where. Three kinds:

- **Instant** ops resolve in `effects::run`, the one dispatch (`modules/effects/mod.rs`);
  `effects::feasible` says whether a player's card has a target.
- **Standing** ops (a building's `passives`, an AI's `innate`) are read through
  `cards::passives` by the rule they change, while the card is in force.
- **Twists** (an adversary card's `twists`) are read through `adversary::twists` while
  the card is current; its `sway`, `each_destroy_node` and `pull_unity` resolve once, in
  `adversary.build`, when it becomes this round's card.

An op shared by threats and team cards takes its wording and tally from
the context; `each_country` is the one printed difference.

### Generated docs

`aic-sim docs` rewrites the blocks between `<!-- generated: NAME -->` markers in
[rules.md](rules.md), all of [ops.md](ops.md), [harms.md](harms.md) and
[player-aid.md](player-aid.md), and `web/card/faces.json`; `python
scripts/cards/cards_doc.py` reads the faces and writes [cards.md](cards.md). A block is
a pure function of `Rules` or `Content` in `crates/aic-sim/src/docs/blocks.rs`, named
in `run` (`run.rs`), with unit tests in `tests.rs`. Change the source, not the output.

`aic-sim docs` fails on a broken harm distribution (`harm_failures`). `--check`, and a
fast-lane test, also refuse a stale block, a British spelling (`spelling.rs`), a
heading like `## 080's changes` (`current_docs.rs`), an id in
`docs/card-pricing-notes.json` that is no player card, and in rules.md a backticked
`module.param` or bare `snake_case` name that is neither a tunable nor an op
(`NOT_TUNABLES` lists the other names it may print). rules.md names a tunable rather
than repeating its value.

## Map

`Map::build` derives everything from `content/map.json` (`state.hexes[i]` is
`map.hexes[i]`), including a military base on each side of each printed front: bases
depend on the layout, so they are not printed. Geometry is not in the state; UIs get it
from wasm's `board(config)`.

## The generic view

Modules come and go, so clients must not need code per module. `Rules::view(&state)`
(wasm `view(state)`) turns any state into the fixed shapes in `view.rs`, and the web
viewer draws only these.

The engine fills every token stack, each seat's resources, exhausted cards no module
shows (`engine.exhausted`) and every `vars` track as a global. `Module::view` adds only
what it can't derive, through `ViewOut`: derived values (a global with a `vars` key
replaces that track's entry), card groups, piles, a seat's `board`, standard actions,
arrows, and `bookkeeping`: `vars` that are per-seat, per-country or per-card markers (a
key ending in `.` is a prefix), whose generic globals are dropped so the module can
show them where they belong. An undeclared `vars` key still shows as a global, so a new
module's track is never hidden by accident.

A card's `status` is a word the module chooses; the viewer styles `ready`, `used`,
`unaffordable` and `no target` and shows any other as text. The view is display data:
the rules never read it, it is not in the state, fingerprint or replay, and a hook gets
`&GameState`, so it can't draw from the RNG.

**What the client assumes.** Unknown token kinds draw as counters, and actions are
offered as `legal_actions` lists them. Only `Action::Choose` with a hex target arms a
hex pick; every other action is a button labeled from its fields. So a new `Action`
variant needs no client change, but a hex-valued field under another name gets no
board pick until the viewer is taught.

**Records.** `SavedGame` (`record.rs`) is the one format for a sim save, a bug report
and a viewer game; `SavedGame::replay` warns on any identity or fingerprint mismatch.

## Observability

- **Event log**: `Event`s from `apply` and `new_game_logged`, for UIs and the sim's
  `--trace`. Hot loops pass `None` to skip it.
- **Counters** in `GameState::stats` are in the fingerprint, so adding or renaming one
  moves the golden even when play doesn't. `adversary.ops` is the proxy for table time.
  The sim averages every counter per game without knowing the names.
- **`PlayedGame`** carries the outcome (a stable `reason` code), decision counts and
  identity (config and content hashes, engine version, `rules_revision`, fingerprint).

## Known gaps

Left out on purpose until something needs them:

- **No interception hooks** between modules ("before a building is destroyed"), and
  `Event` is a closed enum; modules call each other instead.
- **Timed-out games** (the action cap) still count toward round statistics, as
  `ended: timeout`.
- **No state schema version** (only `Stats`, `piles` and `pending` default): an older
  engine's state may fail to load. Records are the durable format.
- **Snapshots cost**: every action and automatic phase clones the state first, for a
  choice to replay from.

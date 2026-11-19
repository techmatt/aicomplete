# Architecture

How the engine, `aic-core` (the rules' only home; other crates: README, *Layout*), works, and how to change the
game. The rules: [rules/](rules/README.md).

## Core invariants

- **No I/O**: no files, network, env vars, stdout or logging. `content/` is embedded at compile time; the caller
  hands in configs.
- **No clock**: no `std::time`, `Instant` or `SystemTime`.
- **No global RNG**: no `thread_rng`, `rand::random` or `getrandom`; only `GameState::rng`'s per-purpose streams
  (`rng.rs`), so adding or reordering modules never shifts another's numbers. Each variant declares its purposes
  (`VariantDef::rng`), none shared between modules; a rule reached from another module's step draws through
  `Cx::deck_rng`, `Cx::threat_rng` or `Cx::adversary_rng`. **Fixed-width draws**: only `Stream::below(n)`, on `u32`; never a `usize`
  through rand (native and wasm32 differ).
- **No hash-order leaks**: maps are ordered (`BTreeMap`; `FlatMap` in the state); iteration follows seat, piece or
  `Dir::ALL` order; no `HashMap`/`HashSet` reaching the state or results. The `Key` interner is the one global,
  never iterated.
- **Serializable state**: `GameState` round-trips through JSON (seed and RNG positions as decimal strings).
- **Exact replay**: config + seed + actions rebuild the identical state; the sim checks every game.
- **Native = wasm**: `scripts/wasm-smoke.mjs` checks the wasm build reaches the `golden_game` fingerprint (README,
  *Build and test*).
- **Printable state**: a physical copy could hold all the rules read; a module needing more needs a design look
  first. `rng` (the dice), `stats` (never read by rules) and `pending` (the open choice) are not components.

## API

Compile `Rules::new(&config)` once per config; the hot path is `legal_actions_into` and `apply_mut(&mut g, &action,
None)` (no event log). The free `legal_actions`, `apply` and `view` rebuild `Rules` each call (a UI's, not a
sweep's) and run `check_state` first. `driver::play` plays a game, one team `Agent` for every seat;
`driver::replay` rebuilds one.

## Config

A config is JSON ([`configs/baseline.json`](../configs/baseline.json)): `modules` (at most one instance per module
id, in one variant, with its `params` and, for `uses_cards` variants, `cards`), and `round`.

- **Every** parameter is given: no defaults in code. Unknown, missing, wrong-kind and out-of-range values are errors
  naming it. **Integers only** (printable); a `list` is a table (by difficulty, tier, country or track).
- A card belongs to at most one module; leaving one out removes it from the game.
- `round` lists `"<module>.<phase>"`, each any number of times; `"a.x+b.y"` combines modules' **decision** phases
  into one simultaneous phase.
- **Config hash**: `Rules::config_hash`, FNV-1a over the resolved config (`name` excluded); lists hash as written.
- **Content overlay**: the config's `content` list ([experiments.md](experiments.md), *Content levers*);
  `Rules::content_hash` hashes it overlaid, `aic_core::content_hash()` embedded; an empty overlay hashes as absent.

## Modules

| Change | Kind |
|---|---|
| A number inside a mechanic | **parameter**: config only |
| Another way of doing it | **variant**: code, selected by config |
| A new mechanic | **module**: code, added to the config |
| A card, threat, board or piece of existing primitives | **content**: data (`content/`) |
| A new effect primitive | an `Op` ([The op schema](#the-op-schema)) |
| A new kind of player action | `Action`; a structural change |

Modules call each other's public functions and read each other's parameters (`World::int("<module>.<param>")`,
hot ones `World::hot(Hot::...)`), declared in `ModuleDef::requires`.

**One path per piece change**, so supply and cancellation hold everywhere: adversary ambassadors are placed and
removed only by `adversary`'s functions (`adversary::remove`, which every path that takes an agent off calls:
claims, liberations, cancels, converts, removal ops); nodes placed by
`board::place_node`; nodes and facilities destroyed by `board::destroy_piece`, which applies every protection
(`board::lose_node` for losses no protection absorbs).
**A player owns a station or factory** by its `cube(seat)` (`common::cube_owner`), put on by `board::claim_site`,
`liberate_site` (`own`), effects' `build_station` / `build_factory` and the Station and Factory standard actions;
read wherever ownership matters (production, destruction, corruption, `plan.rs`'s `CreateIf::Player`,
`take_site`). Under `board.cities_unowned` (baseline 1) no city takes a cube: setup's `bound_city` and a
liberation leave it neutral, and a city that pays (`common::city_pays`: not corrupted, not blocked) gives its energy to every
seat with a node on or next to it (`common::node_by`, `board::city_income`);
`Where::Owned` (content's `per_site` `owned`) counts a free city beside a player's node too, blocked or not
(`common::free_city_by_a_player`). `git grep cube_owner`, `city_pays` and `owned` for a rule about ownership or
city energy.

The hooks' trap (on `Module::act`): check legality before the first `choose`; a `Halt::Illegal` after an answer
panics. Modules change the state only through `Cx` (event log and counters stay consistent); counter and var names
are `"<module>.<name>"`.

### Choices ("players choose")

Each "players choose" in the rules is a choice in `legal_actions`. A module calls `cx.choose(seat,
prompt, options)?` mid-resolution; unless the answer is given or forced it halts (`Halt::Ask`),
`GameState::pending` keeping the question, answers so far and a snapshot; `legal_actions` is then the options
(`Action::Choose`), no pass. An answer restores the snapshot and resolves again from the start, replaying the
answers, so resolution code runs straight through. A prompt names cards by printed name, never a raw id (the
invariants test refuses one).

### The round

An `Auto` phase resolves at once, a `Decision` phase waits until every seat has passed, an open choice for its
answer. The game ends at `Cx::end`, or as a `round_limit` loss after `MAX_ROUNDS`. **Decision phases are
simultaneous**: seats act in any order and `legal_actions` lists every seat's actions, as at a table.

### Adding a variant or module

- **Variant**: a `VariantDef` (`module.rs`; `phases` `Auto` or `Decision`) in the module's `DEF.variants`, with a
  build function and a `Module` impl. `<module>=off` fails if another `requires` it; its `off` variant keeps the
  phases and parameters others read.
- **Module**: `modules/<id>.rs` with `pub static DEF: ModuleDef`, in `REGISTRY` (`modules/mod.rs`), with a config
  entry and its phases in `round`. For player input, reuse an `Action` (claimed by `owns`) and `cx.choose`; add an
  `Action` variant only if neither fits.
- A rule that outlives the variant it was screened in gets a module-level param (Matt), never a variant-only one:
  every reader would have to check the variant first. Params live on `VariantDef` today; the first such rule adds
  the module-level list to `ModuleDef`.
- Either way: add it to `configs/variants.json` (read by `variant_matrix`, `wasm-parity.mjs`, `same_play.py`), bump
  `RULES_REVISION` if existing games change, and document it in `docs/rules/`.

### Adding content

Fields are `content.rs`'s serde structs; not plain there:

- **Player cards** have no `text`: `aic_core::face` generates it. `tokens` names a token type only: no card gains
  tokens by itself ([card-guidelines.md](card-guidelines.md), 15); "here" in an op names this card. `keywords` are
  never printed.
- **AI boards**: `tags` count in `cards::tag_counts`; slots and starting credits are config lists
  ([ai-boards.md](rules/ai-boards.md)).
- **Country pieces** (`map.json`) carry the corp's megaproject; `Megaproject::words` prints its op lists, so a new op
  there needs its words.

Actions name content by id, so reordering a file never breaks action logs. `Content::check` validates references
and each op's `Op::slots`.

### The op schema

Everything content does is one enum, `Op` (`op.rs`), read from JSON as `{"op": "<snake_case>", ...}`. An op means
one thing wherever printed; its context (`effects::Ctx`: `op.rs`'s module doc) sets who chooses and who is hit
where, and wording and tally for one shared by threats and team cards. Three kinds:

- **Instant** ops resolve in `effects::run`, the one dispatch (`modules/effects/mod.rs`); `effects::feasible` says
  whether a player's card has a target.
- **Standing** ops (a building's `passives`, an AI's `innate`) are read through `cards::passives` by the rule they
  change, while the card is in force. A `when` is a trigger: `cards/triggers.rs` fires its instant ops on its
  `op::Event` (`cards::fighting`, the wrapper every action goes through, diffs what one seat's action removed).
- **Twists** (an event card's) are read through `adversary::twists` while it is current; its immediate ones and
  `standing` lines resolve in `adversary.build`.

**A new op parameter** touches `op.rs` (field, `slots()`), `content.rs` (check), `face.rs`, its dispatch (an
`effects/mod.rs` arm matching the field's default keeps the old behavior and sends the rest to
`effects/new_ops.rs`; or the rule reading a standing op) and its price (`agents/value.rs`, `agents/strategy.rs`,
`configs/agents/values.json`, the ledger's hand mirror `scripts/cards/effect_ledger.py`). A new op also touches
`effects/feasible.rs` (what it acts on) and `docs/harms.rs`'s `op_harms` (an arm per op). `op.rs`'s `slots`,
`is_map` (it takes a player's target) are exhaustive matches: decide each. The compiler finds the
Rust sites (`face.rs` lists every non-player op) but not the `values.json` row; the mirror test finds the ledger.
**Removing an op**: the compiler finds its Rust arms; delete its `values.json` row and its `effect_ledger.py`
branch too, and `git grep` its snake_case name in `scripts/design_check.json`, `drafts/adversary/check.py` and tests.
Powers print ops like cards (their `target` op too) in `content/ais.json`; a standard action's ops and target are
code: `std_ops` and `std_target` in `board/standard.rs`.

### Generated docs

`aic-sim docs` rewrites the `<!-- generated: NAME -->` blocks (`BLOCKS`, each in one file) in
[rules/](rules/README.md), [agents.md](agents.md) (`strategies`) and [rulebook.md](rulebook.md) §21
(`rulebook_difficulty`); all of [ops.md](ops.md), [harms.md](harms.md) and [player-aid.md](player-aid.md); and
`web/card/faces.json` and `web/card/adversary.json` (267: the reference card's words, `docs/ladder.rs`). The rest of the rulebook, the rules files' prose outside a block and `drafts/adversary/`
texts are hand-written. Most "for new players" paragraphs are prose; rules/README.md's and alignment.md's are
blocks (`new_players_block`, `game_end_block`): edit those in `blocks.rs`. Scripts ([scripts.md](scripts.md)) write [cards.md](cards.md) from the faces (the fast lane checks
it), and the adversary's `content/` and [adversary-cards.md](adversary-cards.md). A block is a pure function of
`Rules` or `Content` in `crates/aic-sim/src/docs/blocks.rs`, named in `run.rs`, tested in `tests.rs`.

`aic-sim docs` (written or `--check`) fails on a threat tier under 5 kinds of harm (`harm_failures`), a missing or doubled block, a page over 15 KB, a stale rules-revision
line or card quote in `docs/rules/`, a British spelling or stale house word (`spelling.rs`), a history heading
(`current_docs.rs`), a pricing-note id that is no player card. `--check` and a fast-lane test also refuse a stale
file and, in a rules file, a backticked `module.param` or bare `snake_case` name neither a tunable nor an op
(`NOT_TUNABLES` aside): rules name a tunable, not its value. Fast-lane tests refuse a face icon-line word with no
icon (`tests/face.rs`) and an effect ledger off `value.rs` ([card-pricing.md](card-pricing.md)).

## The generic view

Clients need no code per module: `Rules::view(&state)` (wasm `view(state)`) turns any state into `view.rs`'s fixed
shapes. Map geometry is not in the state: `Map::build` derives it from `content/map.json`; UIs get it from the view (`View::hexes`); wasm's `board(config)` returns the whole `Map`, read only by `scripts/wasm-smoke.mjs`.

The engine fills token stacks, seats' resources, exhausted cards no module shows (`engine.exhausted`) and every
`vars` track as a global; `Module::view` adds through `ViewOut` only what it can't derive, and its `bookkeeping`
places per-seat, per-country or per-card `vars` (a key ending in `.` a prefix). The view and invariants tests
refuse a global with more than one dot: a new state key's prefix goes in its module's `bookkeeping`.

A card's `status` is the module's word; the viewer styles `ready`, `used`, `unaffordable` and `no target`, any
other as text. The view is display data: never read by the rules, not in the state, fingerprint or replay; a hook
can't draw from the RNG.

**The client assumes:** unknown token kinds draw as counters. Only `Action::Choose` with a hex target arms a
hex pick; any other action is a button labeled from its fields (a hex field under another name needs viewer work).

## Observability

The **event log** (`Event`s from `apply` and `new_game_logged`) serves UIs and `--trace`. **Counters**
(`GameState::stats`) are out of the fingerprint; `adversary.ops` proxies table time. **`PlayedGame`** carries the
outcome (a stable `reason` code), decision counts and the game's identity.

## Known gaps

- **No interception hooks** between modules ("before a building is destroyed"); `Event` is closed; modules call
  each other.
- **No state schema version** (only `Stats`, `piles` and `pending` default): an older engine's state may not load;
  records (`SavedGame`, the one format of sim saves, bug reports and viewer games: [simulator.md](simulator.md))
  are the durable format.
- **Snapshots cost**: every action and automatic phase first clones the state, for a choice to replay from.

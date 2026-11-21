# What a read file and a trace hold

A read's `games.jsonl` holds one game a line (`GameResult`, `crates/aic-sim/src/experiment.rs`); a trace, one
game's actions and events.

## The file

An experiment's `games.jsonl` starts with a header line (`StreamHeader`: `format` `aic-games/2`, `spec`,
`engine_version`, `rules_revision`, `content_hash`, `config_hashes`), no game keys. A farm shard (`aic-sim farm`,
`crates/aic-sim/src/farm.rs`) writes games only, except the shard at the spec's `first_seed`, which leads with the
header over the spec's range (283); `farm --header` prints one for any range. Read with
`scripts/screens.py`'s `stream` / `load` (skip the header; a bare farmed file needs `FILE=SPEC.json`) or
`scripts/design_check.py`'s `load_games` (drops any line with `format`). A farmed line's timings are 0 and its
`engine` is `native` or `wasm`.

## Keys in a games.jsonl line

What the read scripts quote most (`screens.cnt(game, key)`; more in experiments.md, *Counters*):

| Key | What it holds, per game |
|---|---|
| `cell` | the cell's index in grid order: configs, then variants, then difficulties, then agents, agent fastest (`experiment::cells_with`; `screens.cells_of`) |
| `seed`, `difficulty`, `reason` | the seed, a string; the cell's `adversary.difficulty`; how it ended (`liberated` the win, `purge` before r141, `node` a seat left with no nodes, `research` the research track past its last slot, `deck_empty`, `discord_deck`, `round_limit`, `timeout`) |
| `counters['ai.power.<board>.<power>']` | a board power's uses (`modules/ai.rs`; the seats on that board summed), `.rNN` a round's; names: `scripts/powers.py`; `ai.offered.<board>`: uses offered as the action phase begins, a group once |
| `counters['board.std.<name>']` | a standard action's uses; `board.std_actions` all of them |
| `counters['cards.played.<card>']`, `['cards.drawn']` | plays of a card / cards drawn off the decks, all seats (keys are flat, dotted strings) |
| `counters['fx.<card>.tN']` / `['fx.<card>.<measure>']` | an Alignment card resolved at tier N / the change it made to a `FX_MEASURES` total (`alignment.rs`): a signed delta, not a count |
| `cards[<card>]` | `{offered, drafted, kept, played, pick_sum, drawn}`, all seats |
| `adversary_ops`, `setup_ops` | adversary ops a round (round 1 first) and in setup |
| `sources[seat]` / `uses[seat]` | each resource gained / spent by where, `{label: [8 numbers]}` (below) |
| `start`, `gained`, `spent`, `peaks` | per seat, 8 numbers (`start + gained - spent` is the end) |
| `margin` | the final table (below) |
| `actions[i]` | `{type, seat, ...}`: `standard` names its `act`; `power` gives an index, not a name (count powers by the counters) |

## Counters

`counters` is `GameState::stats.counters` (`cx.tally`, `cx.count`, `cx.level`, `cx.peak`), then the exported
per-round tables. A key without a seat pools every seat over the whole game.

| Family | Scope |
|---|---|
| `cards.played.<card>`, `cards.kept.<card>`, `cards.used.<card>` (a building's Action) | game total, seats pooled |
| `board.amb_moves.<seat>`, `slots.over.<seat>`, `slots.cap.<seat>` | per seat |
| `board.converter.<country>`, `board.chip_converter.<country>` | a country's cube / chip converter used, seats pooled |
| `adversary.corruptions[.city\|.coastal_city\|.station\|.factory\|.node]`, `board.cities_freed` | game totals |
| `<key>.rNN` | `cx.per_round` table `<key>`, round NN, seats pooled unless the key names one |
| `<key>_rNN` (`adversary.ambassadors_rNN`, `adversary.research_fired_rNN`) | a plain counter named by round: always exported |

Only a per-round table whose key starts with a `PER_ROUND_EXPORT` prefix reaches the file: `cards.`, `ai.`,
`economy.`, `board.node`, `unspent.`, `threats.`, `map.`, `adversary.land_tokens`. Any other by-round table
(`adversary.ops`, `adversary.compute`) is dropped without a word; add its prefix to read it. An exported table
writes every round up to its last record, zeros included, so an absent `.rNN` means no record by round NN.
`slots.used.<seat>.rNN` is not a table: the simulator writes it directly.

A per-round key shares its prefix with the per-card family: `cards.kept.r03` is round 3's keeps, not a card.
Strip `.rNN` keys before reading `cards.kept.` or `cards.played.` as card ids.

- `board.node_capstones.rNN`: capstones reached in round NN, not held (`modules/board/node.rs`).
- `map.cities_corrupted.rNN`: cities corrupted in play that round, a retaken one too (`adversary/corrupt.rs`);
  setup's are placed, not counted. No counter tracks retakes.
- Troops: `map.troops.rNN` (on the map; the first troop round is its first nonzero),
  `map.troops_placed.<source>.rNN`; `board.strikes` / `map.strikes.rNN`; `board.redeploys` / `map.redeploys.rNN`
  (`board/production.rs`, `adversary/pieces.rs`, `board/troops.rs`). A project: `adversary.revealed.<id>`, and `adversary.project.<id>`
  (`.rNN` too) when it resolves (`adversary/projects.rs`).
- `adversary.ops` (only in `adversary_ops`): token placements and moves, and each compute point put on a project
  one op (`adversary/projects.rs`): a proxy for table time.

## Resource arrays

`start`, `gained`, `spent`, `peaks` and every `sources` / `uses` row are in `PEAKS` order: credits, insight,
energy, chips, antimatter, research cubes, card tokens, ambassadors. That is not the `Resource` enum's order
(`vocab.rs`, alphabetical). `sources` / `uses` hold one map a seat, keyed by label; a label whose row is all zero
is left out. Labels: `own:` or `other:` an action, then `play:<card>`, `use:<card>` (its Action), `power:<Name>`,
`standard:<act>`, `convert:<country>` (`:chips` after it, the chip converter); `phase:<module>.<phase>` an
automatic phase (`phase:economy.upkeep`: income and capstones; `phase:board.produce`, `phase:board.cleanup`).

## margin

From `margin()` in `experiment.rs`, on the final table.

| Key | Holds |
|---|---|
| `level` | always 0 since r141 (it was the purge bar's level; the win has no steps now). Not the difficulty |
| `blockers` | corrupted cities left, all that stands between the players and the win (`adversary/bar.rs`); 0 on a win |
| `deck_left` | adversary cards still to come, face-up one included |
| `supply_left`, `supply_start` | adversary ambassadors in the supply at the end and at the start |
| `cities_corrupted_end`, `cities_aligned_end` | corrupted cities; cities with a player cube, or free with a player's node on or next to them |
| `unity_tips`, `botnets_end`, `troops_end` | Unity tips; botnets and adversary troops on the board |
| `unspent`, `held_after_actions` | per seat, in `UNSPENT` order (`board::LEFT`: credits, chips, energy, antimatter, ambassadors): held as its last action phase ended (`unspent` reads now if the game ended mid-phase) |
| `node_spread` | per seat: [nodes, countries, connected groups, farthest two apart in hexes] |

## The trace

`aic-sim run CONFIG --games 1 --trace` plays the first seed (agent `random` unless `--agent`) and prints it as
JSON lines (`trace` in `crates/aic-sim/src/play.rs`; to stderr with `--json`). Then the run summary prints, its
`seeds ..., agent ..., rules rNNN` line and any `run overrides:` line after every trace line.

- Line 1: `{"trace": <config name>, "seed": "<n>", "config_hash"}`; then setup's events.
- `{"action": {...}, "options": <legal count>}`, then the events it caused; `{"timeout": true, "actions"}` at the
  cap.
- `{"event": {"type", ...}}`, types from `Event` (`crates/aic-core/src/event.rs`):

| Type | Fields |
|---|---|
| `round_started` | `round` |
| `phase_started` | `phase` (`<module>.<phase>`, `a.x+b.y` combined) |
| `resource_changed` | `seat`, `resource`, `delta`, `total` |
| `tokens_changed` | `hex`, `token`, `delta`, `total` |
| `var_changed` | `key`, `delta`, `total` (`GameState::vars`) |
| `card_moved` | `card`, `from`, `to` (`""` the box) |
| `note` | `text`, `hex` if any |
| `asked` | `seat`, `prompt` |
| `passed` | `seat` |
| `game_ended` | `won`, `round`, `reason`, `detail` |

Counters (`cx.tally` and the rest) emit nothing: the trace never shows them. The summary's `/game` rows do.

Piles (`card_moved`), seats and countries from 0, tiers from 1: `cards.deck.<tier>`, `cards.discard.<tier>`,
`cards.packet.<seat>`, `cards.drafted.<seat>`, `cards.hand.<seat>`, `cards.built.<seat>`, `cards.played.<seat>`
(programs in play: a pile, unlike the counter), `cards.events.<seat>`, `cards.resolving.<seat>`,
`cards.under.<seat>.<building>`; `threats.deck.<tier>`, `threats.discard.<tier>`, `threats.on.<country>`;
`adversary.deck`, `.next`, `.current`, `.discard`, `.project_deck`, `.project_discard`, `.projects`,
`.projects_done`; `alignment.deck.<side>`, `alignment.resolved.<side>` (`unity`, `discord`).

Tokens (`modules/common.rs`): `node:`, `cube:`, `ambassador:` + seat or `adversary`; `troop`, `botnet`,
`fallout`, `harden`, `firewall`, `factory`, `station`; a prohibition's `no_infiltrate`, `no_strike`.

A hex is axial `[q, r]`. A country's hex number n (1-16) sits at `anchor + i × axes[0] + j × axes[1]`, n =
4i + j + 1, from its piece in `content/map.json`'s layout for the seat count (`crates/aic-core/src/map.rs`);
`aic-sim map CONFIG` lists each country's hexes by number. Directions: E `[1,0]`, NE `[1,-1]`, NW `[0,-1]`,
W `[-1,0]`, SW `[-1,1]`, SE `[0,1]`.

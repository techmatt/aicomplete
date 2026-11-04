# 093: reach audit (Matt's ruling 3, candidates)

"Reach is 1 for everything unless there's a reason for an exception." Every distance the rules,
content and core use, at r36. **Nothing here was changed in 093**; Matt rules on each.

Reach 1, as the rule (`common::in_reach`, "on or next to one of your nodes"): every card op "in your
reach" (clear markers, shield, damage, remove botnet, build factory or station, liberate, harden,
hack, the move ops' targets), threat countering ("a source in reach"), city energy sales, liberation
by the fallback, site claims, the board powers that say "in reach" (`content/ais.json`: harden a
facility, clear a marker, build a station). Also 1, hard-coded: a troop strikes on or next to its
hex (`board/troops.rs` `strike_from`), a base guards next to it (`board/destroy.rs`), a new factory
goes to the player with most nodes next to its zone (`board/production.rs` `zone_owner`), a botnet
blocks claims next to it.

## Not 1

| What | Where | Value | The reason it has |
|---|---|---|---|
| War's node loss around a destroyed factory | `board.war_reach` (config, 0..1) | 0 | Added at r18 (034) as a tunable: war takes the node on the factory's hex and, by `war_reach`, around it. No recorded reason for 0 over 1 found in the docs or history. Smaller than 1, not larger. |
| Corruption's node loss | `board.corruption_reach` (config) | 1 | Already 1; listed because it is a tunable. |
| Adversary spread | `adversary.spread_range` (config) | 2 | Default 6 states it (spread by priority then distance); no recorded reason for 2 found; it dates from the early tuning (r3, `variants-spread`). Adversary movement, not a player's reach. |
| Spread twist | `spread_reach` +1 on Long Patrols (stage 1) and Blitzkrieg (stage 3), `content/adversary.json` | 3 that round | The twist's whole effect: an escalation. |
| Move an ambassador | `move_one` `range: 2`: Embassy Compound (Action), Peacekeeper Rotation (play), `content/cards.json` | 2 | Default 51: the v0 movement add-on prints "up to 2 hexes"; read as hex distance. |
| Troop march | `board.troop_steps` (config) | 2 steps a march | Movement speed, not reach: a troop walks toward a target; it strikes at 1. |
| Node placement | `board/node.rs` (nearest numbered spot) | any distance | A node goes on the nearest open numbered spot in a country you choose; that is placement, not reach. |

Candidates for Matt: spread range 2 and the two `range: 2` moves are the only player-visible
distances above 1; war reach 0 is the one below it.

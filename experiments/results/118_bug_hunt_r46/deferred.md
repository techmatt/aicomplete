# 118 bug hunt: deferred and won't-fix, in full

The rows of [docs/qa-findings.md](../../../docs/qa-findings.md) this hunt added, with their evidence and options; the
list there keeps one line each.

| # | Area | Finding | Options, and the pick |
|---|---|---|---|
| D43 | rules | Patience banks the team's insight: others give the Researcher their unspent insight before it expires (r46 seed 34304: 400 held by round 11; the 5-insight counters become free). | (a) Given insight expires under Patience too; (b) cap what Patience carries (5); (c) leave it. **Matt's call**; (b) is simplest at a table. |
| D44 | cards | Simulation Cluster's Reuse takes Popular Uprising's other half the same round: a liberation every other round (alone: every fourth). | (a) Reuse repeats the half used; (b) leave it. **Matt's call**; (a). |
| D45 | engine | Fight triggers read the board's net count: an action that removes an adversary ambassador and places one (a liberation's `free_leaves`) fires no "when you remove". | (a) Count at the removal ops; (b) leave it (rare). **Pick (a)** when a card makes it common. |
| D46 | loops | Drain (5 credits, no limit) with Security Ops Floor and Intrusion Lab pays about 12 at values.json for 10 credits, while any project holds compute. | (a) Leave it: compute caps it; (b) fight triggers once a round on a standard action. **Pick (a)**. |
| D47 | agents | Agents misprice r41's engines: a teammate trigger at the owner's tag share, not the teammates' (~2.4× high); strategy categories read no `when`/`threshold` contents; tokens toward an At N at the flat held rate; the value table assumes 2 seats (solo gifts, `both`). | (a) One agent item, screened on and off; (b) leave them. **Pick (a)**. |
| D48 | faces | Icons drop what words say: Great Liberation's cube has no city, Field Grant no station limit, Hostile Buyout no align mark. | Add the marks. **Matt's call** (icon set). |
| D49 | scripts | `card_ranks.py`'s keep rate counts opening keeps, never offered (349 of 894 rows over 100%), and speed.md's 0.985-0.991 bar was measured on it; `r37_read.py` reads a counter dead since 106. | (a) A draft-keep counter, then re-measure the bar; (b) kept per game. **Pick (a)**. |
| D50 | viewer | A new player's notes from two real-speed sessions (eleven: the tier step's price, Rush Build's cards unnamed, "act + act", a struck-through used building, seats by number…): `experiments/results/118_bug_hunt_r46/viewer-notes.md`. | Each a small view change; picks there, **Matt's call** where marked. |
| D51 | threats | Total War's 2 ambassadors on bases may both go on one base (players choose). | (a) One a base while one is free; (b) leave it. **Matt's call**. |
| D52 | viewer | The timeline keeps a parsed state and its JSON a step: a 4-country game holds ~200 MB; a reload replays it in one blocking pass. | Keep every Nth snapshot. **Pick** if long games are watched. |
| D53 | agents | Hyperscale Botnet (tier 2, 4 insight a round) is kept in about 40% of games and played in 2% (r46 too); `npm run probe` on 24 r47 strategist games: legal 138 steps, played 0. The play check, not the engine. | (a) Read the strategist's play value of an insight income against its keep plan's; (b) leave it. **Pick (a)** with D47. |
| D54 | threats | Autonomous Defense Grid does nothing under `shared_row`: it counters in its home country; the row has none. | (a) The row's threats of its type; (b) leave it. **Matt's call**; (a). |
| D55 | rules | At 1 country there is no military base: base arrivals, stage II's Escalation and Total War's base ambassadors place nothing. | (a) A base-less fallback (cities); (b) leave the solo map so. **Matt's call**. |

| # | Area | Report | Why not |
|---|---|---|---|
| W19 | ops | `copy_income` is printed by no card; the agents' copy check counts incomes the engine won't copy. | No card since r43: fix with the card that brings it back. |
| W20 | content | `Content::check`'s place-token loop guard sees only a direct `add_tokens`. | No deck card prints such a trigger. |
| W21 | viewer | A card's hover shows its id and pile key. | A developer aid, as W9. |
| W22 | sim, viewer | A run keeps every game's final state; the viewer's kept unloadable session is one slot. | Bounded by `--games`; the newest loss is the one to recover. |
| W23 | config | `stage_cards`, `project_cards` read 0..20 in tunables.md; more than the content holds is refused. | The content caps it; the refusal says so. |
| W24 | measures | `economy.*_unspent` is read at upkeep, before threats refund part payments; `effects.insight_lost` tallies the printed debt; `Transfer` vetoes at 1 seat (no card prints it). | Reading aids; fix with the read that needs them. |
| W25 | report | "adversary ops/round" counts a game's last, partial round (1 game in 48 at use-090); "Agents (weakest first)" prints the spec's aliases (strategist), not the tables' labels, and doesn't check the order. | Small; the tables name agents by label. Fix with the next report change. |
| D56 | content | `Content::check` lets through content that never runs or breaks a rule: a threshold on an AI board's innate or capstone (boards run triggers with no card, so it never pays), a `here` token op in any board list (skipped), and the Science-niche and `gain_per_tokens here` rules skip a trigger's and threshold's ops (a Cyber building that shields on every play passes). No shipped content does any of it. | (a) Widen the checks to boards and trigger ops, a test each; (b) leave them. **Pick (a)**: F196 did the two simplest. |

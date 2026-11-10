# 165: cleanup (162's and 164's audit notes, Matt's extras, state.md's *For the next cleanup*)

No rule or number changed. Same games: `experiments/same-noplanner.json` (new here: same.json's seeds, pairs and d2
with greedy for the planner rows), the base binary (5b7899a) against this branch: **144 of 144 games the same**
(`bench/same_results.py`), rerun on the final build before the commit. `speed.py pair` (16 games, base against this
branch, games a minute): greedy 1.02-1.03 ×, engine 0.98-1.01 ×. `speed.py check` read NO VERDICT (its baseline was
r66, two revisions back), so 165 moved the baseline to r69 on this build (`--update`: 1.00 × each).

The `--features profile` prompt tally on two greedy games (d2, seeds 1000-1001) names the cost at once: Manhunt's,
Raiding Parties' and Blackout's "discard which card (# of #)?" chains are 17%, 12% and 11% of the agents' time.

## Doc sizes (bytes; the budget is 15,360, `docs --check` now warns over 14,336)

| Doc | Before | After |
|---|---|---|
| README.md | 14,492 | 13,490 |
| web/README.md | 14,615 | 13,500 |
| docs/agents.md | 14,654 | 13,328 |
| docs/principles.md | 14,651 | 13,058 |
| docs/experiments.md | 14,651 | 13,499 |
| docs/scripts.md | 14,523 | 13,498 |
| docs/architecture.md | 14,525 | 13,498 |
| docs/card-pricing.md | 14,120 | 13,492 |
| docs/rules-history.md | 14,246 | 13,176 |
| docs/card-guidelines.md | 14,590 | 13,498 |
| docs/rules/card-grammar.md | 14,339 | 13,497 |
| docs/rules/player-cards.md | 14,533 | 13,466 |
| docs/rules/adversary.md | 14,362 | 13,496 |
| docs/rules/ai-boards.md | 14,270 | 13,498 |

Cut: repetition (text the code, a generated table or another doc already says, now a pointer) and history (item
tags, superseded numbers). Two reviewers read every diff against the old text and restored six facts the trim had
dropped or bent (card-guidelines rule 6, two bump notes in rules-history, three architecture.md rules, the viewer's
data tabs). principles.md's dominance-check table is now a list (names and specs kept; each policy's description is
`forced_table!`'s). card-grammar's Draw bullet lost r44's "a draw never reshuffles": r48 replaced it (cards/deck.rs,
player-cards.md *Decks and zones*), and the bullet now says r48's rule once.

## Pointers left open by the trim (not decided here)

- agents.md *The tiers* gives the strategist's target as "d2 45-85" (the ballpark ruling) beside principles.md's new
  per-level bands (d2 55-65): which one a read is held to is the bands question in the report.
- player-cards.md says the capstones read 1.0-1.35 at Matt's anchors; card-pricing.md says 1.0-1.5 (r45). One is stale.
- card-guidelines.md rule 4's keep "(1 insight or 5 credits)" against the anchors' "the keep, 4 a tier-1 card".
- architecture.md's config hash bullet ("lists hash as written") against `Config::hash`'s doc comment (normalized).

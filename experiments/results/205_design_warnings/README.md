# 205: design warnings that fire on their own (r88, no rules change)

`scripts/design_check.py` reads `content/`, `configs/` and a read's `games.jsonl` and prints every design warning it
trips, each with its number; thresholds and source classes are candidates in `scripts/design_check.json`. The
principles are quoted as checkable rules in [docs/principles.md](../../../docs/principles.md), *Design warnings*;
CLAUDE.md, *Report*, makes every rules item run it on its first screen and fresh read and put the first line it
prints in its report header.

**Data.** 200's r88 reads were not on disk (its worktree is gone). Main (b99ed8f) is r88 with 200's final dials, so
200's fresh read was replayed: `f200-strategist` on seeds 44767-44910 (144 a pair, 2,160 games, farmed), and r87
(aab153e) natively on 44767-44814 (720 games) as the tokens check's old side. No fresh seeds taken. The replay
reproduces 200's ladder rounds (d0 6.2, d1 7.7, d3 6.4, d4 6.6; d2 7.1 at compute 9, where 200 read 7.0 on the
confirm seeds) and its antimatter table (the action 2.86 of 5.37 a seat-game).

## Done when 1: 200's known cases, flagged

`python scripts/design_check.py out/r88full --old out/r87read --base aab153e` ([r88-against-200-base.txt](r88-against-200-base.txt)):

| known case | warning | number |
|---|---|---|
| the antimatter action's share of supply (200: 52%) | `cheap-source antimatter`, `one-source antimatter antimatter-action` | 54% from standard actions or tier-1 cards (max 20%); the action alone 53% (max 40%) |
| Annihilate on all three boards | `same-power Annihilate on 3 boards` | 3 boards (max 1) |
| game length 6.2-6.4 rounds | `rounds` | d0 6.2, d1 7.7, d2 7.1, d3 6.4, d4 6.6 (target 8-10) |
| (also) dials beside the design change | `dials-hide-design` | compute d2 17→9, agents d2 and d3 2→3 in the change that edited 10 design files |
| (also) the action priced over the anchors | `standard-over-anchor antimatter action` | 1.75 (1 Sway, 4, for 1 antimatter, 7) |

Unity for an act of force (Annihilate's 3 Sway) is not a warning yet: no source has a story tag (204 assigns them).
The checker lists the 44 Sway sources as information and marks the Unity ones beside a removal (`force?`): the three
Annihilates and Surgical Strike.

## Done when 2: the sweep of main (r88)

`python scripts/design_check.py out/r88full --old out/r87read` ([sweep-main.txt](sweep-main.txt)). Every warning,
with a fix idea (none built: for 204 and later items).

| # | warning | number | fix idea |
|---|---|---|---|
| 1 | same-power Annihilate | 3 boards | one board's power (the Protector's: Wall piles troops); the other two get their own antimatter sink (Builder: antimatter into a factory or chips burst; Researcher: antimatter drains a project) |
| 2 | same-power Expand | 3 boards | keep each board's own node route (Buy In, Accord, Peer Review) and drop the shared Expand, or make "1 energy → a Node" a poorly priced standard action printed once |
| 3 | same-effect place_node (Expand, Buy In, Accord, Peer Review) | 3 boards | the costs differ by board, which may be the intended difference: if nodes stay a power on every board, exempt `place_node` in the config rather than in the rule |
| 4 | same-effect draw 2 (Procure, Debrief) | 2 boards | Debrief draws differently (draw 2 keep the Diplomacy one, or draw 1 + 1 ambassador) |
| 5 | standard-over-anchor antimatter action | 1.75 | Matt's desperate source: 3-4 research cubes → 1 antimatter (0.29-0.39 at the cube's 6), no Sway |
| 6 | standard-over-anchor Buy a city's energy | 1.12 | `city_energy_credits` 4 → 5 (0.90) |
| 7 | standard-over-anchor Remove agents | 3.75 | the ambassador's value is the gap: `values.json` 4 against a 20-credit price; set an ambassador anchor (about 10) first, then `remove_count` 3 → 2 if still over |
| 8 | standard-over-anchor Clear a corruption marker | 4.00 | `clear_cost` 2 → 4 credits with `clear_unity` 2 → 1 (1.0) |
| 9 | standard-over-anchor Take down a botnet | 4.00 | `botnet_credits` 5 → 10 or the Sway dropped (16 for 5 is a past check's botnet value; recheck it) |
| 10 | standard-over-anchor Project: drain | 1.20 | `drain_amount` 3 → 2 (0.80) or 6 credits |
| 11 | standard-over-anchor Liberate a city | 3.25 | the win route, not a hedge: decide whether liberation counts as a standard action under the principle; if it does, an ambassador anchor (7) brings it to 1.6 |
| 12 | cheap-source antimatter | 54% | as 5; the plants' Action at 3 (200's idea 1) once agents play them (13) |
| 13 | one-source antimatter-action | 53% | as 5 and 12 |
| 14 | one-spend Annihilate | 97% of antimatter use | as 1: board-specific sinks; the ultimates (1 each) are 3% |
| 15 | one-spend Liberate | 77% of ambassador use | other spends priced to compete (Claim, Envoy), or a per-resource cap for ambassadors if liberation should dominate |
| 16 | kept-not-played | 29 cards kept 0.10-0.46 a game, played ≤0.06 | mostly the agents: the seat could play most of these at some play phase (`keep_ok` 10-85% of kept copies, 22 of 29 over 30%; [dead-keeps.md](dead-keeps.md)) and chose not to, so 203 (the strategist plays what it can afford) first, then reread; any still dead after 203 is a card whose play loses to the alternatives at the table: reprice or give it a use |
| 17 | power-unused Focus | 0.02 a seat-game | 2 credits → 1 insight at 2 credits an insight is break-even at the table (2.5): 1 credit, or 3 credits → 2 insight |
| 18 | power-unused Recovery | 0.03 | clearing a marker by power competes with Clear at 2 credits: let it also pay Clear's Unity, or clear land corruption too |
| 19 | power-unused Synthesis | 0.00 | 0 uses means the agents never value it: review the eval's discard (a held card against 2 insight) before repricing |
| 20 | victory-skips research cubes | spent in 49% of won games at d3-d4 (min 75%) | more cube engines (200's idea 6); cubes have one cube card played 0.03 a game |
| 21 | rounds | d0 6.2, d1 7.7, d2 7.1, d3 6.4, d4 6.6 | Annihilate's 3 Unity tips the track fast: its Sway to 1 or to Discord (the story check's direction), then reread; failing that, more Unity to tip (`tip_per_player`) |

No warning: tokens a round (adversary ops 44.6 against r87's 42.9, +4%; player token moves 13.6 against 16.2,
−16%), dials on main (none moved 40% against the merge base), resources gained and not spent (each spent over half),
the cheap-source check's other resources (antimatter is the only one listed as scarce).

## Decisions and caveats

- **Phases are not one source.** `phase:board.produce` (99% of cubes, 43% of energy) pools every producer that
  pays in production, so the one-source check skips `phase:` keys and lists them as information.
- **The anchor check is printed maximums at the anchors** (card-guidelines.md's table, values.json for units it lacks,
  a few from past checks in the config's `extra_anchors`). Five of seven trips involve ambassadors or Sway, which
  have no anchor of their own (values.json's 4 each): an ambassador anchor is the first calibration (rows 7, 11).
- **Thresholds are untuned.** `kept-not-played` uses three bars (kept 0.1 a game, played under 0.1 and under a
  quarter of keeps); the header folds a check with over three warnings into its count and first three.
- **Content checks read the tree**, so a read from another revision gets today's content beside its games (an r87
  read lists Annihilate as unused): run it in the item's own worktree.
- `--base` defaults to the merge base with main: on an item branch that is the item's start, so the dials check
  sees the item's own change.

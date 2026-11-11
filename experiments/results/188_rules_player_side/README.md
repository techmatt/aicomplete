# 188: friction round 3, the player side (r81)

Matt's 2026-11-11 rulings on `docs/rulebook-friction.md`'s rows #16, #47, #51 and #54, built as one revision. The
constant was 80 when the lock came (187 took 80), so this is 80 → 81, as the prompt said.

## What changed

| Row | Ruling | Engine | Test |
|---|---|---|---|
| #47 | Threats print the card tag names and icons | `ThreatTag` Security, Diplomacy, Cyber, Industry, Power, Universal; `ThreatSource` Diplomacy, Cyber, Industry, Security (variant order kept, so play is the same); content, drafts, `check.py`, the viewer's badge classes; a typed jam's face prints `#<tag>` beside the jam; the rulebook's conversion table is gone | `every_threat_tag_is_a_card_tag` |
| #16 | A power or card that places a node costs only its tag or currency; every placement pays the space | Expand costs nothing, Buy In 6 credits, Accord 1 Diplomacy (each was 1 energy more); `PlaceNode.free` and `board::Placing` are gone, so the four "free" cards pay the space; the `node_free` agent rate is gone | `a_card_pays_the_step`, `a_node_power_pays_only_the_step`, `every_board_s_expand_costs_only_the_space_once_a_round` |
| #51 | Drones never protect; Shelter, a hardening token or your own ambassador; one name for the token | `Guard::Base` is gone (and the agents' forecast of it); "shield" left the house terms, the faces ("Harden 3 of your Nodes.") and the rules prose | `a_drone_at_a_base_never_absorbs_a_strike_next_to_it`, `harden_says_facility_and_a_node_s_token_is_hardening_too` |
| #54 | Take down a botnet requires reach | `botnet_targets` adds `in_reach` to the ambassador check | `taking_a_botnet_down_needs_your_ambassador_on_its_hex` (extended) |

**Decisions on Matt's default (question 1, held for him):** Expand costs nothing but the space (energy is not "its
currency"); the four free cards pay the space at unchanged credit prices; Aegis Doctrine stays as a protection, a
capstone naming its own exception. The ladder is read as built, with no compensation.

## Give / get (values.json: energy 4.5, node 8, node_late 6, insight 2.5, credit 1)

- **Expand:** gives the space (0, 1, 2, 3, 5 energy: 0-22.5); gets a node, 8. A gain on spaces costing 0-1; a loss on
  the 2+ spaces at the table's node price alone (the space's own income makes it up, as before, when 1 energy more).
- **Buy In:** 6 credits + the space for a node, 8: a gain on free spaces (was 10.5 for 8, a loss at the table).
- **Accord:** 1 Diplomacy (a sacrifice) + the space for a node.
- **Relay Mast Rollout** (10 credits, 1 insight: 12.5): 2 nodes 16 + 2 cards 8 = 24, less the spaces: +11.5 on free
  spaces, about −2 on a 1-and-2 pair. **Aegis Grid Lockdown** (9 + 2.5): 3 nodes 24 + Shelter 2.8, +15 on free spaces,
  below 0 once the three spaces cost 3+ energy. **National Grid Buildout** (34 + 2.5) and **Beamed Power
  Constellation** (30 + 5) were played 0.01 and 0.00 a game before this item and still are (the effect ledger prices
  them 53.8 and 47.5 before the spaces). Flag: the space now makes the late-game node cards near break-even; prices
  unchanged per Matt's default.

## Ladder (fresh seeds 43327-43470, paired against r80)

Strategist 144 a pair (432 a level), greedy 43327-43374 (48 a pair). r81 farmed (`farm-seeds`, main box, 409 s and
150 s), r80 natively from `out/r80` (a clean build of 9dfae05 and its configs). Specs `experiments/farm/f188-*`.

| d | r81 strategist | r80 | gap ± SE | band | r81 greedy | r80 greedy |
|---|---|---|---|---|---|---|
| d0 | 90.3 | 88.9 | +1.4 ± 1.9 | 90-100 met | 90.3 | 88.2 |
| d1 | 77.5 | 75.7 | +1.9 ± 2.5 | 70-80 met | 77.8 | 78.5 |
| d2 | 63.0 | 54.4 | +8.6 ± 2.5 | 55-65 met | 57.6 | 56.9 |
| d3 | 53.5 | 49.3 | +4.2 ± 2.4 | 40-50 **unmet by 3.5** (1.5 SE) | 59.0 | 47.9 |
| d4 | 50.0 | 42.1 | +7.9 ± 2.5 | - | 46.5 | 41.0 |

The d3 miss is over the band (easier). Its pairs: br +12.5 ± 4.2 (59.0), bp +2.1 ± 4.1, pr −2.1 ± 4.2
(`pairs-strategist.md`). No difficulty dial moved (the prompt). Greedy is ahead of the strategist at d3 again (59.0
against 53.5), as at r80.

**Attribution (indicative: dev seeds 36800-36831, 96 games a level; `s188-attrib.json`, `attrib.md`).** r81 against r81
with the node powers' energy put back (Expand 1, Buy In +1, Accord +1): r81 leads by +1.0 ± 4.1, +9.4 ± 5.6,
+11.5 ± 5.7, +9.4 ± 5.9, +10.4 ± 5.0 at d0-d4. The cheaper node powers carry the whole gain; the other three rulings
(free cards now paying, drones, botnet reach) can't be isolated by overlay and read as small in the uses below.

**Uses a game, strategist, all levels (`uses-strategist.md`):** nodes placed 16.46 → 17.84; Buy In 1.73 → 2.50, Accord
1.16 → 1.33, the Researcher's Expand 3.71 → 4.05; Relay Mast Rollout 0.56 → 0.68 plays, Aegis Grid Lockdown 0.14 →
0.18 (paying the space did not cut them); drones' saves 0.05 → 0 (hardening 2.31 → 2.49, Aegis 0.04 → 0.06 took some);
botnets taken down 1.32 → 1.31 (reach never bit: an ambassador gets onto a botnet only in reach).

## Other measures

- **Rulebook Part 1** (187's `wc_part1.py`): 7,969 → **7,873** words (−96), the page 12,128 → 12,001: the tag table and
  the free-node sentence went, the protection list lost an item.
- **Adversary measures** (`ladder-strategist.md`): within noise of r80 at every level.
- **Loop smoke** (16 seeds, `use.json`): no timeouts, no card at the play flag.
- **Speed:** `speed.py check` gave no verdict across the revision (greedy 0.89×, engine 1.27×, planner 2.01× a game).
  `per_decision.py` (planner, 16 games, d2): 1.20× a decision with 1.18× the options a decision and 1.03× the rounds:
  the free Expand is legal more often, so each decision has more to weigh; per option the planner is flat. Baseline
  updated at r81.
- **Same results:** nothing in a core rules file changed after the farmed commit (9d97d93); later edits are prose.

## New-player summary (r81)

Threats now carry the same tags as your cards (Security, Diplomacy, Cyber, Industry, Power), with the same icons: to
pay one with tags, sacrifice cards showing its tag. Placing a node always costs the energy printed on the track space
you uncover, and nothing else in energy: Expand costs nothing more, the Builder's Buy In 6 credits, the Protector's
Accord a Diplomacy tag, the Researcher's Peer Review 3 research cubes, and a card only its own price. When something
of yours would be destroyed, Shelter stops it for free; otherwise you may spend a hardening token there or your own
ambassador there (the Protector's Aegis Doctrine adds one more); drones only fight in wars. Taking down a botnet needs
it in your reach, like everything else you target. None of these resisted a short statement.

# 317: card pass on r152 (r152 → r153)

`RULES_REVISION` 152 → 153 (bumped from the constant). The card edits are one overlay,
[cards317.overlay](cards317.overlay) (rerun with `scripts/cards/apply_overlay.py`, then `scripts/regen.py`).
Anchors: [card-guidelines.md](../../../docs/card-guidelines.md); pay = credits + 2.67 × (keep 3 + play insight) + other
costs at the anchors (energy 3, chip 4, antimatter 18, ambassador 5.33, a city liberated 20, a station freed 15.33, a
factory freed 9.33, compute 1.33, a project discarded 16). Table net = `effect_ledger.py --stdout`'s reasoned − printed
− the tier-2 keep (6.93).

## 1. Unprinted ops

None of the six is in the source: `adversary_cubes` and `botnet_cubes` went in 293, `counter_all`, `pushback`,
`remove_botnets` and `one_player` in 315 (`git grep` in `crates/`, `scripts/`, `content/`, `docs/`: only
`values.json`'s `pushback` row, kept as `remove_where`'s agent rate, and a comment in `scripts/content/spine_293.py`).
Nothing to delete. One op became unprinted here: `destroy_all` (Annihilation Lance's old wipe) gets its lab run in
`tests/unprinted_ops.rs` and stays until the set freezes (284's rule).

## 2. Field Grant: left as it is

315's r152 judge read (`r315`, seeds 52066-52161, 1,440 games, `price_read.py`): kept in 34.7% of games, played in
25.3% (tier-1 mean 25.1 / 17.3; 101st of 138 tier-1 cards by plays). Win when played −6.0 ± 5.3 raw (tier-1 mean
+1.7: −7.7 relative, 1.5 SE, not a flag). 316's r152 reads agree: kept 38-39%, played 22-29% under every arm
(cheap, E128, E256, J128), and E128 / E256 won 100% of the games that played it. This read: played 15.6% → 15.2%.
Not a weak pick at r152; no change.

## 3. Twelve tier-2 cards as late-game plays

**Which:** the twelve tier-2 cards played least on 315's r152 judge read, past 213's six reworked majors and its
new ones, the finale buildings, the liberate capstones (Great Liberation, Forensic Audit) and Command-and-Control
Grid (the Cyber chain's repeatable draw engine, rulings *Card plays*); three cards in the content are not in the deck.
Pooled table: `out/keep/317/price_r152.csv`.

| Card | r152 | r153 | Value check (anchors) | Table net | Played, % of games, r152 → r153 (fresh) |
|---|---|---|---|---|---|
| Fusion Research Campus | +2 energy a Power card; 2 cr, 2 ins | Action: pay 3 energy, free a corrupted station; 3 energy when built; 6 cr, 1 ins | a use 9 → 15.33 (1.70); 3-4 uses 43.7-52.7 → 55-70: 1.26-1.33 | +9.9 | 4.4 → 9.2 |
| Arcology Foundry | +5 credit, +1 chip income; 7 cr, 2 en | Target a corrupted city: place 4 Ambassadors on it (they pay its Liberate bill), gain 3 chips; 6 cr, 2 en, 1 ins | 22.67 → 38.67: 1.71 | +23.1 | 4.4 → 6.2 |
| Vertical Monopoly | building, 2 credits an Industry tag a round; 11 cr | program: 1 antimatter per 2 Industry tags; 12 cr, 1 ins | 22.67 → 36 at 4-5 tags (1.59), 54 at 6-7 (2.38); 72 at 8-9 (3.2, over: Matt) | +33.1 | 3.2 → 3.5 |
| Annihilation Lance | 1 am, wipe a hex + 2 troops, Discord 2; 5 cr | discard an adversary project, 12 compute off every project; 10 cr, no antimatter | 18 → 32: 1.78 | +1.6 | 2.4 → 26.6 |
| Orbital Mass Driver | 13 credits, 2 chips a round; 4 cr, 1 am | Target a corrupted factory: Action: pay 1 chip, free it; 4 cr, 1 ins | a use 4 → 9.33 (2.33); 3-4 uses 26.7-30.7 → 28-37: 1.05-1.22 | +0.5 | 4.9 → 5.8 |
| Homeland Security Act | +2 energy, +1 credit per 2 Security tags; 12 cr, 2 ins | Target Agents: remove 4 on or next to it, cordon its country; 10 cr, 1 ins | 20.67 → 13.3 + a cordon ~10 = 23.3: 1.13 (0.81-1.61) | +4.3 (override) | 2.8 → 7.6 |
| Continental Supergrid | +2 energy, +1 credit per 2 Power tags; 13 cr | help: 4 energy and 3 Ambassadors for you and another player; 16 cr, 2 ins | 29.33 → 28 + gift 21 = 49: 1.67 | +21.7 | 2.4 → 13.1 |
| Zero-Day Auction | 2 am, draw 3, needs 5 Cyber; 4 cr | 8 cubes off the adversary's research track, 1 antimatter, needs 4 Cyber; 6 cr | 19.33 → 18 + 16 = 34: 1.76 | +16.7 | 3.9 → 6.1 |
| Foundation Model Campus | 2 credits a Science tag a round, draw 2; 11 cr, 2 en | Action: pay 2 insight, sway 4 toward Unity; draw 2 when built; 8 cr | a use 5.33 → 10.67 (2.0); 3-4 uses 37.3-42.7 → 38.7-49.3: 1.04-1.16 | +0.5 | 5.9 → 8.4 |
| Lure Network | Action: remove up to 3 agents; 12 cr | Target a corrupted city: Action: pay 12 credits, rescue it; needs 2 Cyber; 12 cr | a use 12 → 20 (1.67); 3-4 uses 58.7-70.7 → 60-80: 1.02-1.13 | +4.0 (override) | 12.4 → 12.0 |
| Agency Directorate | Operatives, 4 insight at 4; over Security; 2 cr | 2 Ambassadors a Security card played; no tokens, not over Security; 10 cr, 2 ins | 23.33 → 32-43: 1.37-1.83 | +4.5 (override) | 6.2 → 11.7 |
| Beamed Power Constellation | place 3 nodes, 1 energy a node; 9 cr | shelter your nodes and facilities in reach, 1 energy a node; 7 cr | 17.67 → 20.7-26.7: 1.17-1.51 | +0.9 | 10.8 → 7.6 |

Each card's note is in `docs/card-pricing-notes.json`. Three `values.json` overrides (Homeland Security Act, Lure
Network, Agency Directorate) lift a table read under its give to about 1.2×, each with its why; Orbital Mass Driver's
and Annihilation Lance's first overrides went when their redesigns read positive. `design_check_jobs.json` rows
moved or added for nine of the cards.

**Design rounds before the freeze** (none farmed on fresh seeds): an Action can't print an ambassador cost (Lure
Network went to credits); two top boxes ran 6.9 pt (Orbital Mass Driver's megaproject Action, Agency Directorate's
tokens: both redesigned); the loop smoke had the judge keep Fusion Research Campus and never build it (3 energy when
built added, engines pay up front). The diff review then moved Annihilation Lance off antimatter (compute lands on the
leftmost project alone, so its get is about 32 and no price reaches the sinks' 2.0 floor), put Lure Network at 12
credits and needs 2 Cyber (10 credits and no needs put a liberation a round in reach from the first tier-2 draft) and
took Agency Directorate's `replaces` (an engine buries no building).

## The read: r152 against r153 (fresh)

The judge, `experiments/archive/farm/r317.json` (read.json at d0-d4: the three pairs), fresh seeds 52246-52341 (96 a
cell, 288 games a level), paired. r152: `farm-seeds --at a50a1d10` (main and the spec); r153: 090c1ab5. Reader:
`bench/ladder.py --measures`.

| Level | r152 | r153 | Paired gap | Band (soft) |
|---|---|---|---|---|
| d0 | 96.5 | 95.8 | −0.7 ± 1.6 | 90-100, met |
| d1 | 95.8 | 98.6 | +2.8 ± 1.4 | 80-95, unmet (r152 already above it) |
| d2 | 92.4 | 96.9 | +4.5 ± 1.8 | 75-90, unmet (r152 already above it) |
| d3 | 81.6 | 85.4 | +3.8 ± 2.7 | 55-70 (no ceiling since Nov 21), met |
| d4 | 51.4 | 58.3 | +6.9 ± 3.0 | none |

The twelve changed at once, so the gap is the pass's, not any card's. d1, d2 and d4 move by 2.0-2.5 SE: the pass made
the players stronger. Not dialed (bands soft). Rounds shorten: d0 6.54 → 6.24, d1 6.43 → 5.98, d2 6.80 → 6.36,
d3-d4 within 0.05. Agents placed and corruptions fall a little at d0-d2 (d2: 46.9 → 43.1 agents, 8.8 → 8.0
corruptions); d3-d4 unchanged. The screens on 315's spent seeds 52066-52161 against its kept r152 side ran before
the review's fixes: d4 +13.5 ± 3.3 (Lure Network at 10 credits, no needs; Lance at 0 cr, 1 am), then +9.0 ± 3.3 (Lure
at 12 and needs 2 Cyber).

**Resources**, spent of gained, r152 → r153: credits 107 → 108%, insight 77 → 79%, energy 80 → 80%, chips 56 → 56%,
antimatter 54 → 53%, research cubes 56 → 54%, ambassadors 71 → 70%. Insight slack 10.4 → 10.4%, bound 19.6 → 21.0% of
seat-rounds.

## Checks

- `design_check.py` (fresh, against r152): `warnings: design: tag-story 4 (cards:foundation_model_campus,
  cards:orbital_mass_driver, cards:agency_directorate, ...); design: usage Make antimatter 0.03 outside 0.05-0.75;
  design: usage Liberate a site 98% outside 50%-95%; design: usage Tidal Grid converter 0.27 outside 0.30-3.00;
  design: rounds d0 49% in 6-8, 7% at 10+; design: rounds d4 59% in 6-8, 12% at 10+`. New: the four tag-story lines
  (Unity on a Science card, a factory freed by an Industry one, ambassadors on a Security one, projects on a Power
  one: tags are flavor, so Matt's call) and d0's rounds; Tidal Grid 0.28 → 0.27 is the r152 edge.
- `npm run cards:fit`: 0 of 213 below 7.5 pt. `loop_smoke.py --run`: no timeouts, most plays of one card 1.
  `play_read.py` (smoke, 48 judge games at d2): Fusion Research Campus kept 8%, never built (before the when-built
  energy; 9.2% played on the fresh read).
- `speed.py check`: no verdict across a revision (baseline r148); every agent at or under 1.0× (greedy 0.63×,
  engine 0.56×, planner 0.83×); the baseline is not updated.
- No core rules file changed after the freeze; no replay needed.

Outputs: `out/keep/317/` (both read sides as `FILE=experiments/archive/farm/r317.json`, the two screens,
design_check's outputs, the pooled r152 card table). Seeds spent: 52246-52341 (ledger next free 52342).

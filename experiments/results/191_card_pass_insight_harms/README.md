# 191: card pass: insight prices, node cards, the harms ledger (r83)

Matt's 2026-11-11 rulings and amendment 8617, as one content pass at r83. The prompt says "content-only (no rules
revision unless an id or engine rule changes)": no id or engine rule changed, so `RULES_REVISION` stays 83 (190 set
it); the content hash and golden fingerprint move (`7df77c8d9a036d86`, `13a2c78f2c32925e`). That line beat CLAUDE.md's
"bump for any change to how games play out" (the prompt is the more specific ruling).

## What changed

| Item | Change | Files |
|---|---|---|
| Insight to play, spread | tier 1 at 0 / 1 / 2: 29 / 97 / 1 → **83 / 32 / 12**; tier 2: 0 / 25 / 33 → **18 / 18 / 22**. Tier 1: every card that doesn't draw plays for 0; the 12 tier-1 draw cards worth most at the table play for 2 (card-guidelines 7 keeps every draw card at 1+). Tier 2: every card at 1 that doesn't draw plays for 0, the 10 cheapest non-drawing 2s play for 1. Credits move the anchor's 4 an insight, less what would leave a card under a 1-credit gain at the table (never under 2 an insight) | `insight.overlay` (written by `spread.py`), `content/cards.json` |
| Node cards | National Grid Buildout 34 cr, 1 ins → 22, 0; Beamed Power Constellation 30, 2 → 14, 1; Relay Mast Rollout 10, 1 → 6, 1; Aegis Grid Lockdown 9, 1 → 4, 0 | `spread.py` (`NODE`) |
| D80 | Great Liberation prints "Liberate or free 3 corrupted cities, stations or factories" (the city form of `cleanse`) | `crates/aic-core/src/face.rs` |
| D82 | the five cards under 7.5 pt each lose their last rider and its price: Rally the Allies and Mass Defection the 3-Diplomacy ambassadors (18 → 12, 15 → 9), Surgical Strike the 3-Security botnet (19 → 14), Double Agents and Air Gap Sway 1 (12 → 8, 10 → 6). All five now fit at 7.5 pt | `d82.overlay` |
| Card backs | `renderBack(tier)` in the card renderer: the tier's ground, "Tier N", and the keep cost as 1 or 2 insight hexagons ("Keep N"); the viewer's View menu has "Backs" | `web/card/card.js`, `card.css`, `web/cards/viewer.js`, `web/cards.html` |
| Harms ledger | 13 → 26 rows; `op_harms` has an arm for every `Op` (no catch-all); facilities destroyed and seized split; resources lost by kind (energy, chips, antimatter, research cubes); a Corrupt line no longer counts as node destruction and the node-share slack is back to +1 (amendment 8617); a harm in under 2 kinds of source is a gap, listed in `docs/harms.md`, not a failure | `crates/aic-sim/src/docs/harms.rs`, `docs/harms.md` |

Cards whose top box prints under 7.5 pt (Retained Earnings, Intrusion Lab, Open Assembly, Agency Directorate, Popular
Uprising, Particle Accelerator at 6.3; Interceptor Battery, Conglomerate Headquarters at 6.9) did so before the pass
too (`cards:fit` on the pre-pass faces); not touched here, and not in 184's D82 list.

## Give / get

Every changed card's line is in [give-get.md](give-get.md) (values.json and anchors; get is the effect ledger's
`reasoned`). The node cards, with their spaces at 1.5 energy a node:

| card | was | now | give was → now (table) | get | net now |
|---|---|---|---|---|---|
| National Grid Buildout | 34 cr, 1 ins | 22, 0 | 56.8 → 42.2 | 53.8 | +11.5 |
| Beamed Power Constellation | 30, 2 | 14, 1 | 55.2 → 36.8 | 47.5 | +10.8 |
| Relay Mast Rollout | 10, 1 | 6, 1 | 26.0 → 22.0 | 25.0 | +3.0 |
| Aegis Grid Lockdown | 9, 1 | 4, 0 | 31.8 → 24.2 | 26.8 | +2.6 |

**Flagged:** 11 moved cards net under +0.5 at the table (Counterinsurgency Campaign, Dispatch Control Room, Eviction
Worm, Intrusion Lab, Open Assembly, Retained Earnings, Synthetic Data Forge, Telemetry Archive, University Spinout,
Decapitation Strike −12, Killswitch Directive). Each was already priced at a loss at the table before the pass (its
engine or rider the table can't see); the pass moved each 2 credits an insight, so each is 0.5 better off than
before. Not repriced further here.

## Ladder (fresh seeds 43759-43902, paired against the pre-pass cards)

Strategist, 144 seeds a pair (432 a level), d0-d4. New side farmed (`farm-seeds`, main box, 348 s), the old natively
from a build of de6b0aa copied before the first edit (`out/r83pre`). Spec `experiments/farm/f191-strategist.json`;
the full table and standing measures in [ladder-strategist.md](ladder-strategist.md). No dial moved.

| d | pass | pre-pass | gap ± SE | band |
|---|---|---|---|---|
| d0 | 86.8 | 88.2 | −1.4 ± 2.1 | 90-100 unmet |
| d1 | 60.6 | 73.8 | **−13.2 ± 2.8** | 70-80 unmet |
| d2 | 45.1 | 59.7 | **−14.6 ± 2.7** | 55-65 unmet |
| d3 | 35.9 | 53.9 | **−18.1 ± 2.9** | 40-50 unmet |
| d4 | 34.7 | 46.8 | −12.0 ± 2.8 | - |

**Greedy** (seeds 43759-43806, 144 games a level; [ladder-greedy.md](ladder-greedy.md), spec `f191-greedy.json`): 78.5 /
73.6 / 56.2 / 40.3 / 35.4 against 86.8 / 73.6 / 69.4 / 55.6 / 48.6 (gaps −8.3, 0.0, −13.2, −15.3, −13.2, SE 4-5): the
drop is not the strategist's alone.

**Attribution (dev seeds 36800-36831, 96 games a level, d1-d3; [screen1.md](screen1.md), [screen2.md](screen2.md)):**
levers on the built pass. `old` (every touched card back at its pre-pass row) recovers +8.3 / +12.5 / +13.5,
matching the fresh drop. `flat` (insight moved, credits unmoved) recovers +2.1 / +10.4 / +7.3; `half` (2 credits an
insight) +0.0 / +8.3 / +1.0; `flatno2` (flat, and the 12 tier-1 2s back at 1) +2.1 / +19.8 / +8.3. Read: about half
the drop is the credits (the scarce resource: credits ran out in 30% of seat-rounds, insight in 2%), some is the
tier-1 draw engines at 2 insight, the rest within the screen's noise (SE about 5.5). Indicative only: dev seeds, small.

## Resource use (the ladder's 2,160 games a side; [resource-use.md](resource-use.md), 189's `resource_use.py`)

| per seat a game | pass | pre-pass |
|---|---|---|
| credits spent / ran out | 241.5 / 30% | 229.5 / 27% |
| credits to plays | 170.5 | 157.0 |
| insight spent / ran out | 36.5 / 2% | 38.6 / 4% |
| insight to keeps / to plays | 20.0 / 11.2 | 18.7 / 16.0 |

**Insight's keep-versus-play trade (per game, both seats):** to keeps 40.0 (37.5), to plays 23.2 (33.3); cards kept
33.4 (30.8), played 36.1 (34.9); insight unspent at round end 100.7 (74.4), Patience carried 54.7 (39.0). With most
tier-1 cards at 0, insight binds less at play than before, not more: the choice it prices is now which of a few cards
(the 2s) to play, while credits carry more of every play.

## Matt's watch list (amendment 8617; [plays.md](plays.md), `plays.py`)

Plays a game (both seats), by the card's insight to play:

| tier | insight | cards (pass / pre) | plays a game (pass / pre) |
|---|---|---|---|
| 1 | 0 | 83 / 29 | 11.21 / 3.38 |
| 1 | 1 | 32 / 97 | 11.23 / 21.54 |
| 1 | 2 | 12 / 1 | 3.48 / 0.00 |
| 2 | 0 | 18 / 0 | 4.10 / 0 |
| 2 | 1 | 18 / 25 | 4.19 / 6.08 |
| 2 | 2 | 22 / 33 | 1.91 / 3.90 |

Node cards, plays a game: National Grid Buildout **0.11** (0.01), Beamed Power Constellation **0.02** (0.00), Relay
Mast Rollout 0.71 (0.64), Aegis Grid Lockdown 0.36 (0.20). Beamed is still nearly never played (needs 3 Power tags;
the engine smoke kept it in 27% of one cell's games and never played it).

## The harms ledger's gaps (the list for the adversary-card pass)

Under 2 kinds of source: **antimatter lost** (0), **protections stripped** (0), **pieces pushed or reach cut** (0),
**adversary cubes on megaprojects** (1: corruption's stations; `adversary_cubes`/`botnet_cubes` are on no card), **the
clock sped up** (0), **a megaproject or converter denied** (0), **the card market disrupted** (0). Over half of the
node destruction: **tier-2 threats, 6 of 9** (with Corrupt lines out, the rest is 1 event, the corruption and war
rules). Question 1 (Matt's): the share is listed as a gap, not a `docs --check` failure (default; was a failure).

## Smokes and speed

Loop smoke (16 seeds): no timeouts, most plays of one card in a game 2. Play read (16 seeds): see plays above.
`speed.py check`: no verdict (other games at the same revision): random 0.94x, greedy 0.84x, engine 1.90x, planner
0.79x a game. `per_decision.py` (engine, 6 games, d2): **1.15x a decision on 1.18x the options** (with most tier-1
cards at 0 insight, more plays are legal each decision) and 1.07x the decisions. Kept: the options are the pass's
point; baseline updated at r83 with the pass.

**The beam agent (tests only) slowed about 5x a decision.** `beam:width=2` (default depth 120) on seed 3, release:
110 s and 730 decisions a game before the pass, 445 s and 587 after (0.15 → 0.76 s a decision, on fewer options). Its
search runs to the round's end, and a round now holds more plays (most tier-1 cards are free of insight), so each
search goes deeper. The slow lane's `beam_replays_and_repeats` (a whole beam game played twice and replayed, test
profile) ran past 11 minutes and was stopped; its spec is now `beam:width=2,depth=30` (26 s a game in release), as
replay and repeat don't need the depth. The engine (beam's code with the strategist's heuristic) costs 1.15x a decision
(above); no read uses the plain beam.

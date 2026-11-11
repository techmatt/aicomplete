# 178: rules r77, the rulebook-friction rulings (effects, words) and the docs sync

Built on main 5c06ab4 (r76, 177). `RULES_REVISION` 76 → 77, bumped from the constant: the prompt said r76, but 177
(landed while this item staged) took 76. Matt's rulings of 2026-11-10 on `docs/rulebook-friction.md` as the prompt
quoted them; amendments 8044 (piece names), 8112 (compute phase, card station claims), 8118 (answers), 8122
(Drones); message 8240 (load). Seeds: dev 36800-36815 (smoke); fresh from 42853 (177's next free).

What r77 changed, rule by rule, is [changes.md](changes.md) (the spec the docs follow).

## Decisions

- **Liberate cards** carry no `align` any more (every liberation puts the cube on); `free_leaves` (locked 0) is gone
  from the config with `board::liberate`, its only reader. Great Liberation's cities liberate the same way. No card
  prints the align mark (it would be on every liberate card; the new house term **Liberate** says it).
- **Convert** swaps: the agents come off first, then the ambassadors go on where they stood, so two converted on one
  hex both stay; an agent the card left there meets them 1 for 1 (the contact rule).
- **The strike** asks batch by batch (troops or agents, on the hex or next to it) with a stop. The sim players answer
  it with a rule (largest batch of troops, then agents: the order the engine fixed before), not the search, so a
  strike costs one search answer as before (`agents/moves.rs`, `strike`; under `move_rules`).
- **Vigil** fires once per troop per march that ends next to a Protector node, any march; a troop used up striking
  has no end to its march.
- **Cordon**: it is set to 2 when played, counts down at cleanup, and the compute step (the adversary turn's end)
  clears it, so the forecast's reading stands.
- **Threat cards** print one frame line above the text (`threats::THREAT_FRAME`): "Payers: any player with a Node
  or Ambassador on or next to a token. Target: the player with a Node nearest a token." The waits token is the view's
  "Waits token" line.
- **Dispatch** names the action and its allowance ("Dispatches", track spaces "Dispatch +1"); internal ids keep
  `move` / `movement`. **Drone** is `common::DRONE` in generated text; content's hand-written text went through
  `rename178.py` (rerunnable). The rulebook's prose keeps piece names lowercase (amendment 8044).
- **Content check** now refuses "each board" (151 refused "each country").
- **The determinize slow test** (amendment 8112) checks compute-phase prompts again; it skips only a prompt that
  follows a hidden draw inside its own resolution (a project resolved and the row refilled, then the next asks),
  which no phase split can make replayable.

## Give / get (at `configs/agents/values.json`; effect ledger reasoned value against printed cost)

| Card | Change | Printed | Reasoned | Gain |
|---|---|---|---|---|
| Security Contractor HQ | +10 credits once per action that removes troops (was per troop) | 16.5 | 25.0 | +8.5 |
| Data Licensing Deal | +5 credits once per placement (was per token) | 17.5 | 25.5 | +8.0 |
| Hostile Buyout, Liberation Accords, Power to the People, Citywide Microgrid, Nuclear Option, Forensic Audit, Root Access, Popular Uprising | liberate removes the agents and puts your cube on (`liberate_align` 5 now on all) | | +5 each | |
| Kettle Operation, Lure Network, Honeypot Relay | strike on or next to, your choice | 23.0 / 14.5 / 14.5 | 54.2 / 17.1 / 17.7 | + |
| Air Gap, Firewall Array | site doesn't corrupt: `firewall` row 8 → 10 | 8.5 / 17.5 | 20.6 / 27.0 | +12.1 / +9.5 |
| Hardpoint Lockdown | cordon shorter, pool waits: `cordon` row 5 → 3 | 5.0 | 5.8 | +0.8 |
| Mass Defection, Double Agents, Open Source Uprising, Embassy of Exiles | convert in place | 13.5 / 10.5 / 11.5 / 12.5 | 32.7 / 24.9 / 27.5 / 25.3 | + |

Every touched card still shows a gain at the table. No printed price changed. The trigger firing rates
(`fires_troop`, `fires_tokens`) were left as they were (no measured basis to move them).

## The sim players

Reviewed for each changed decision: the strike's hex (search) and batches (the rule above), Clear on land corruption
(the eval already priced land tokens, 147), convert (no node choice left), liberate cards (`value.rs` and its ledger
mirror), firewalled sites (the eval's corruption risk treats them as prohibited), the draft's pass (the eval mirror
in `eval.rs`), Vigil (`fires_wall` unchanged). Engine smoke, 16 dev seeds, d0-d4, three pairs: no timeouts, every
changed mechanic used; move rules on and off flat within SE (strategist 66.7 against 69.2, greedy 67.5 against 68.3 over
240 games each); the strike rule answers 0.92 prompts a game.

## The fresh ladder (r77 against r76, the same seeds)

[fresh-ladder.md](fresh-ladder.md): the strategist on 144 fresh seeds (42853-42996: 432 games a level), greedy on 48
(144 a level), d0-d4, the three pairs, paired. r77 farmed by `farm-seeds` (2,160 games in 303 s, 30 of 72 shards on
a laptop tab in wasm), r76 natively from a clean export of 5c06ab4 (`out/r76/`, its configs). Next free seed 42997.

| d | strategist r76 → r77 | gap ± SE | band | greedy r76 → r77 | gap ± SE |
|---|---|---|---|---|---|
| 0 | 88.7 → 90.7 | +2.1 ± 1.9 | 90-100, met | 93.1 → 92.4 | −0.7 ± 3.0 |
| 1 | 78.2 → 81.0 | +2.8 ± 2.4 | 70-80, **over by 1.0** | 82.6 → 79.2 | −3.5 ± 4.2 |
| 2 | 60.9 → 68.1 | +7.2 ± 2.5 | 55-65, **over by 3.1** | 67.4 → 70.8 | +3.5 ± 4.1 |
| 3 | 49.3 → 55.8 | +6.5 ± 2.5 | 40-50, **over by 5.8** | 61.1 → 62.5 | +1.4 ± 4.3 |
| 4 | 47.2 → 50.0 | +2.8 ± 2.4 | below d3, met | 59.0 → 61.8 | +2.8 ± 3.8 |

The oracle row at d4 (local, 24 games a side, 8 seeds a pair from 42853): 100.0 on both (177 read 91.7 on its
seeds). Strategist losses at r77 (round cap / Discord deck / last node): d1 72 / 28 / 0 %, d2 8 / 89 / 3 %, d3 35 /
62 / 3 %, d4 36 / 57 / 6 %.

**The driver is the Protector's Vigil.** By pair, the strategist's gain is bp +12 / +8 / +6 and pr +16 / +7 / +6 at
d2 / d3 / d4, while br (no Protector) reads −6 / +5 / −4. Vigil now pays each troop that ends a march next to a
Protector node, and troops the Wall stops park there and pay again every round: 62 firings a game against r76's 20
(the smoke's 16 seeds). With Vigil paying 0 (an overlay, on set a's spent seeds), bp and pr read 2-19 points below
r77 and about 6 below r76: the ruled wording more than accounts for the gain. A difficulty dial would also move br,
which didn't move, so no retune was made here: the Protector-side choice went to Matt as a design question (default:
a follow-up item caps Vigil at 2 insight a round, near r76's rate). Many rulings moved at once; only Vigil was
isolated.

## Checks

Fast lane and `cargo test-all` green (after the determinize test's skip above); clippy clean; `aic-sim docs --check`
current; `npm run cards:fit` passes, and no card's fit is worse than r76's (Lure Network's top box and Honeypot Relay's
effect box now fit); `loop_smoke.py --run`: no timeouts, no card at the play flag; `play_read.py`: no touched card
kept and never played. Speed (`per_decision.py`, r76 against r77, 6 games, one thread, quiet box): ms a decision
0.84× (strategist) and 0.90× (greedy); `speed.py check` gives no verdict across a revision. Specs and readers:
`specs178.py`, `localspec178.py`, `assemble178.py`, `use178.py` (mechanic use), `pairs178.py` (gap by pair),
`rename178.py` (the printed-word pass).

## For new players (r77)

The adversary's troops arrive by **Muster**; your node power is **Expand**. A **Dispatch** puts one of your
ambassadors, from anywhere, onto a piece in your reach. The draft always passes to the next player. Every card
effect that says "when" happens once per action, and a per-tag gain counts the card's own tags ("including this").
A strike removes pieces on or next to the hex you pick, your choice. A liberate card works like the Liberate action:
the agents go and your cube goes on. A converted agent becomes your ambassador where it stood. Clear removes land
corruption, and nothing can be built on corrupted land. Firewalls and cordons last until the end of the next adversary
turn; a firewalled site can't be corrupted, and a cordoned country's agents wait. The Protector's board prints its
troop landing (on an adversary or neutral factory only), and Vigil pays when a troop ends its march next to its node.
Each threat card names who may pay (anyone near it) and who it hits (the nearest node). Megacorps' troops are
**Drones**. **What resisted a short summary:** Vigil's payout (a troop that parks next to the node pays every round),
and the cordon's waiting pool, which lands with the next Rally's income.

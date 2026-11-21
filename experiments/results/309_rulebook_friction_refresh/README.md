# 309: rulebook refresh and friction review at r150

No rules, content or engine change (locked): docs, the aid's generator, and the review's tools. `RULES_REVISION` stays
150, and the content hash is unchanged.

## 1. The rulebook, teach and aid at r150

Two read-only audits checked `docs/rulebook.md` (Part 1 against §1-14 and teach.md; Part 2 against §15-24 and the
aid) claim by claim against `docs/rules/`, `configs/baseline.json`, content and the engine. **They found 28
mismatches and no removed mechanic.** The overhaul's cuts (war, the march, Muster, growth, Dispatches, Envoy,
hosting, the purge, Spill and Grow) were already gone from all three docs. **The prompt's three examples are live
at r150:** Fallout (the Sway step's drip, Lift Fallout, event and Unity lines), the threats step (`threats.execute`)
and tipping (`alignment.md` *Tipping*). They stay.

Fixed, by kind (the hand play's wording fixes are in section 2):

- **Wrong:**
  - §3's event deck still read 5 / 5 / 3 (it has been 4 / 4 / 2 since r145).
  - A cordoned country's factories make no agent either (§6).
  - Purge Lists takes an ambassador from each player; only Crackdown sends ambassadors home (§17).
  - Botnet Swarm's spread is gated at level 0 too (§21).
- **Missing:**
  - A card's liberation of a city takes the agents off with the cube (engine `liberate_site`). The Liberate action
    refuses a site with an agent on it.
  - Great Liberation frees any corrupted site (a cleanse): a freed station or factory keeps its agents.
  - The finale buildings rescue each round.
  - A cordon stops Infiltrate.
  - A firewall makes a creation line take the next site.
  - Arms Race is a mid-game rule.
  - Scrap II's requirements are counted after its sacrifice.
  - Lab Purge picks by count.
  - The projects list lacked extra threats, Grand Offensive, compute income, blackouts and the energy cut.
  - The glossary lacked Fallout, Node, Botnet, Rescue, Wall and Vigil, and finale buildings.
- **Unclear:**
  - The threat fallback is "the lowest-numbered city, one not the adversary's first" (two places).
  - "a few tier-1 cards" spend antimatter: it is two.
  - Martial Law's "36 a board" read as all three projects' cost.
  - The Garrison and Infiltrate glossary lines were incomplete.
- **Merged:** §11's reach sentence now points to §2, and §14's check order to §4. Part 2 still restates Part 1
  (§15 of §6, §19 of §11): a reference that stands alone was kept (friction row 13 offers the cut).
- **teach.md:** Garrison's full order; a troop "on or next to" an ambassador.
- **The aid** (`crates/aic-sim/src/docs/aid.rs`, `HOUSE_TERMS`):
  - removed the hosting leftover ("a bought chip on a city (above)");
  - War Research's K prints by level (3 at levels 0-1, 2 from level 2), where it printed level 0's alone;
  - Liberate reads "3 at any site";
  - blocked chips include an agent on a free city;
  - the city-energy marker moves when a node goes next to a city;
  - *In reach* names the ambassador exception for troops (r142/r150);
  - *Liberate* says a card liberating a city takes the agents off;
  - new **Your level** strip (the ladder, cumulative), after the d4 hand play's four lookups to §21.

## 2. The friction review

Recipe: `traces.py` traces 268's seeds 268000-268013 at d2 and d4: 28 judge games, about 15 s, 6 at a time against
278's 14. `trace_rounds.py` (now with card names, each seat's resources and the tracks in `check.md`) split d2 seed
268000 (7 rounds, a win) and d4 seed 268010 (6 rounds, a loss). Two helpers resolved them by hand from the rulebook
and aid alone, under [handplay-brief.md](handplay-brief.md), 268's brief brought up to the r150 round. Logs, counts,
errors and friction rows: [handplay/d2](handplay/d2/), [handplay/d4](handplay/d4/). Each helper took about 13
minutes of wall time.

| Read | d2 | d4 |
|---|---|---|
| Hand play: minutes a round | 29-39 (rounds 1-7; about 250 a game with setup) | 25 / 37 / 44 (rounds 1, 3, 5) |
| Hand play: errors | 3 in rounds 1-6 (round 7's check was read early) | 7 in 3 rounds |
| Hand play: lookups a round | 11-16, flat all game | 6-16 |
| Hand play: memory peak | 4-6 | 2-5 |
| Yardstick (`breakdown.py`, 309's hand model) | 24.3 a round, 7.21 rounds, **175 min** | 23.4 a round, 7.07 rounds, **166 min** |
| Same traces, 268's r125 hand model | 23.6 a round, 170 min | 26.1 a round, 185 min |

Tables: [breakdown-d2.md](breakdown-d2.md), [breakdown-d4.md](breakdown-d4.md). The action phase is 9.8 minutes of
24.3 at d2. The adversary's Card beat (threats, lines and arrivals) is 4.8, and the compute and projects step 2.2.

**Errors (findings):**
- **Fixed in words** in the rulebook:
  - a way jammed to 0 pays the threat off (§10);
  - "neither possible" pays from the supply and discards the whole hand (§10);
  - a contact in your own action fires "when you remove Agents" (§8; checked in `triggers.rs`);
  - an arrow's tie goes to the lower-numbered city (§2; `map.rs`);
  - War Rooms reads any of a threat's tags (§3; `threats/mod.rs`).
- **Open, each a friction row:** an unnamed "A, OR B" half and a self-triggering building (row 11); a city across
  the border (row 7); an anchor agent lost among three threats on one hex (row 6); the Industry discount (row 3).
- **The tool's, not the rules':** the d2 helper's missing Weapons Test (the script named it `orbital_strike`), now
  fixed in `trace_rounds.py`.
- **A slip, not an engine mismatch:** the d4 helper's Turncoats. The trace shows a 3-card hand, so the engine took
  the discard.

**The top rows for Matt** (full table, options and rulings: [docs/rulebook-friction.md](../../../docs/rulebook-friction.md)):

1. **Escalate + Commandeered Plants.** Two scans of the same sites: about 85 checks a d2 game for 4 hits, and 29 for
   none in 3 d4 rounds.
   - Pick: fold the board's rule into Escalate ("factories always escalate").
   - Touches the Spine ruling (each board's one rule) and *Card-driven* (no board sweeps).
2. **Threat location tokens by "most agents".** Up to 13 checks a round.
   - Pick: the lowest-numbered site of each type, as every other adversary line picks.
   - Touches the Spine ruling's "no 'most agents' picks", which the threats kept.
3. **Discount stacking on keeps and plays.**
   - Pick: discount pips on the tag dials (a component).
   - Touches *Table time and the card economy* (discounts kept); the plays-only option would change Insight's keep
     prices (*Resources and engines*).
4. **Two projects in one compute step at d4.**
   - Pick: an aid line, the remainder onto the next project's track before the slide.
5. **Unity cards that count 0.** 5 of 8 tips did nothing.
   - Pick: a small floor on each card (content).
   - Touches the fear-card shape (kept).
6. **Threat execution volume at d4.**
   - Pick: the aid's two-step row (harm, then the anchor agent).
7. **Production recounts.**
   - Pick: move the energy marker as a node goes down (the aid's node-power line).
   - Keeps *City energy*'s income track (*Resources and engines*).
8. **Used markers.** They hold memory at 4-6 (12-22 at r125), but `rulings.md` lists used markers as rejected.
   - Pick: keep them and amend the ruling.

## 3. Budgets

`table_cost` budgets reset where the overhaul moved them on purpose, read with `table_cost_read.py`:

- **Checks:** 49-59 → **12 / 14 / 17 / 18 / 21**. r149's fresh read (306's kept, 1,152 games, d0-d3) measures 10.7-15.6
  and r150's d4 traces 18.9. The march, per-site passes and growth are gone.
- **Components:** 24 / 36 / 49 / 75 / 88 → **22 / 31 / 43 / 55 / 71**. Measured: 19.4 / 27.5 / 38.3 / 49.5, d4 63.8.
  Troop marches and spills are gone.
- **Memory:** unchanged at 4.5 (measured 2.9-3.4: no rule moved it on purpose).

The new budgets are the read plus about 10%. The d4 budget rests on 14 games only.

## Tools changed

- `experiments/results/268_table_cost_audit/trace_rounds.py`: card names beside ids, plus seat resources and the
  project, research and board tracks in `check.md`.
- `scripts/bench/breakdown.py`:
  - `--hand DIR`, which defaults to this item's r150 log (`--hand 268` for r125's);
  - r150's step labels;
  - the d4 lookups and modifiers read from the hand log's `d4/counts.csv` (268's three constants were the same numbers);
  - `Production` for "Production and hosting";
  - the troop checks moved under the Card beat, with no March row.
- New: `traces.py` (6 at a time), `table_cost_read.py`, `handplay-brief.md`.

Kept: the r150 traces and their experiment, at `out/keep/309/traces-r150/`.

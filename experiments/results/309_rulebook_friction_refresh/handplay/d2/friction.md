# Friction: d2 hand-play (seed 268000, 2 players, level 2, won in round 7)

Fix marks: **W** wording or layout, **A** player aid, **C** component, **P** a rules change.

## The numbers the brief asks for

- **Two-beat adversary turn.** Bookkeeping checks per round in B (lines + Income): 19, 31, 29, 26, 23, 26, 25 (rounds 1-7, mean 26). Components moved in B: 7, 28, 25, 15, 26, 31, 17.
  - **Escalate**: 3-7 sites checked every round from round 2. It fired 3 times in 6 rounds (C15 corrupted in round 2, C16 helped along in round 7).
  - **Commandeered Plants**: 6-9 factories checked every round, 7 rounds. It fired 4 times and corrupted 2 sites (C3 in round 4, C16 in round 7). The two "holds an agent" lines cost about 85 checks over the game for 3 corruptions.
  - **Infiltrate**: 2-8 sites a round. 9 agents landed on parked ambassadors and cancelled them one for one (round 2: 2, round 4: 3, round 5: 2, round 6: 1, round 7: 1). Players parked ambassadors on purpose, so every Infiltrate means checking ambassadors as well as agents.
  - **Arrivals**: about 45 arrival checks over the game. 5 corrupted a site (C15, C3, H2, C3 again, C16) and 1 took a node (Night Raid on C7).
  - **Corrupted sites' outputs**: 3-7 a round. A corrupted factory is two steps (push along the arrow, then make 1 if it is empty). In round 5 the push was what set up Night Raid's node kill.
  - **Compute step**: once a round, with 4 project resolutions, each a resolve + slide + deal (3 components). The sum is income + botnets + waiting, 3-4 terms every time.
  - **Research check**: 7 checks and 3 advances. Slot 2 (Infiltrate: factories, at once) landed 8 agents in the middle of the Income beat, an Infiltrate in a place the round table doesn't show.
- **Threats.** 17 dealt (Provocation's Defection included). 3 were paid in resources (4 insight, 11 cr, 4 en). 4 were jammed to 0 by a program or building (FPO, Civic Watch twice, Threat Hunt). 6 executed and 4 never came due (the game ended). Token placement: 2-13 site checks per round, rising with the threat count. 1 fallback token (Call-Up, round 1). **3 of the 17 threats aren't printed** in adversary-cards.md (Orbital Strike, Garrison State, War Economy), and that was 1 error. 6 harm choices, all "discard", each followed by picking 1-2 cards.
- **Ambassadors.** About 30 moves, 6 Liberate actions (each bill: 1 on the site + 2 from supply), 1 card rescue (Fusion Torch), 5 converts (Embassy 4, Open Source Uprising 1), about 14 one-for-one contacts (9 Infiltrate cancels, 3 moves onto agents, 1 push onto own ambassador, 1 more in round 6), 1 botnet takedown. Which ambassadors are tipped was held in memory every action phase (1 item, up to 5 pieces).
- **Troops.** None all game: research stopped at slot 3, and no troop project or Standing Army came up. 2 Strike icons (rounds 5, 6) each cost a "no troops" check. Garrison, Redeploy, Wall and Vigil were never exercised.
- **Builder's action phase.** Science discount from round 4 (−1 insight per play and per tier-2 keep, −2 from round 6). Master Plan −1 credit per building from round 6. No Industry discount all game (1 Industry tag). Tag counts for requirements came up about 10 times, 3 of them counting covered buildings (Fusion Torch's 3 Power). 2 Overcharge sacrifices. 4-6 plays or uses a round; 12-14 modifiers in the busy rounds (4, 5).
- **Unity and Discord tipping.** 8 Unity tips and 1 Discord tip. Each tip means drawing a card, reading its strength off the tip strip (aid: 3 lookups) and counting its condition across the map (New Deal: 5 sites counted, cap 3). **5 of the 8 Unity cards did nothing** (Neighborhood Watch, Peacekeepers, Decommission, Global Coalition, Civil Defense): the full count came out 0.
- **Card faces with long text.** The Unity cards (three 150-200 character strengths each), Embassy of Exiles (target + convert + when played), Survey Drone Fleet (4 effects), Root Access (4 effects, one conditional on reach), Agency Directorate (a self-trigger with an At N), Great Liberation. Neighborhood Watch's strength I runs 160 characters to say "no agent lands where an ambassador stands".
- **Rulebook length.** 90.8 KB, 1,335 lines, 24 sections, plus a 12.4 KB aid and 62 KB of card and adversary lists. 100 mid-game lookups in 7 rounds (11-16 a round, flat: practice didn't bring them down, because each round brought a new kind of thing).
- **Turn and setup time** (formula in the brief): setup 4 min (52 components). Rounds 29-39 min each, 243 min for 7 rounds.

## Rows

| rule (§) | what it cost | how often met | how hard (one line) | the smallest fix |
|---|---|---|---|---|
| §6 Escalate + Commandeered Plants ("holds an agent") | bookkeeping | 7 rounds, ~85 checks | two separate scans of the same factories for the same condition, fired 4 times | **P** merge into one line ("each free site of last round's type or factory holding an agent takes 1 more"); or **A** one checklist row |
| §6 Infiltrate onto parked ambassadors | bookkeeping, components | 9 cancels | each Infiltrate site is a two-piece check; easy to forget the ambassador is lost too | **A** add "an agent landing on an ambassador: both off" to the step B row |
| §6 research slot 2 (Infiltrate factories at once) | bookkeeping, components | 1 (8 agents) | an Infiltrate in the Income beat that the round table doesn't show | **W** in §4's B row: "a research slot's icon happens now" |
| §6 corrupted factory output | bookkeeping | 2-3 sites x 4 rounds | push, then make 1 if empty: two steps, arrow needed | **C** print the arrow and the two steps on the zone |
| map.json zone arrows | lookup | 2 zones (H11, C14) touch two cities | the arrow isn't in the map data; with two cities next to a zone I had to guess | **C** print each arrow (and put it in map.json) |
| §10/§18 threat token placement | bookkeeping | 17 threats, up to 13 checks a round | "most agents, tie lowest" per type per threat; grows with threat count | **A** a one-line placement order on the aid |
| §10 a threat jammed to 0 | lookup, memory | 4 | nothing says a threat needing 0 is paid off by itself | **W** one sentence in §10: "a threat that needs nothing is paid off" |
| adversary-cards.md missing threats | error | 3 threats (Orbital Strike, Garrison State, War Economy) | can't resolve a threat without its text | **C** print them (the list is out of step with the decks) |
| §7 a building triggers itself | error | 1 (Agency Directorate) | one sentence in §7; the card's own play is easy to leave out | **W** print "(this card counts)" on self-triggering faces |
| §7 "A, OR B" Actions | error | 1 (Safehouse Network) | the halves have no names, so "take the gain" is ambiguous | **W** name the halves on the face (Add / Cash in) |
| §5 insight set at round start, not on uncovering | lookup | 1 (and every later uncover) | the space reads "+4 insight", which looks immediate | **W** print the new total on the space ("from next round: 7") |
| §7 keep and play discounts stacking (keep upgrade, Science per 2) | modifier | every keep from round 4 (~20) | 2 modifiers on each tier-2 keep, the Science count changing as cards land | **A** a keep-cost row: 3, −1 upgrade, −1 per 2 Science, min 0 |
| §7 tags on covered buildings | memory, modifier | ~3 requirement checks | covered cards still count, so the tableau shows less than the dials | **C** tag dials (already ruled) carry it if kept up |
| §8 ambassadors arrive ready (supply, card, convert) | lookup, memory | ~30 moves | which are ready, which tipped, and which "moves" are free | **C** tip the piece (already ruled); **A** "arrives ready: from supply, placed, converted" |
| §8 Liberate bill | modifier | 6 | 1 on site + 2 from supply, every time | none needed: one modifier, clear |
| §8 Rescue claim | lookup, bookkeeping | 1 | 4 facilities checked for a claim, including one holding the other player's node | **A** fine as printed in the aid |
| §12 Unity card conditions | bookkeeping | 8 tips | map-wide count every tip; 5 of 8 came out 0 | **P** (option) a fallback for an empty condition; **W** shorten the faces |
| §12 tip strip strength | lookup | 9 | strength by tip count, read off the aid each time | **C** print strength on the tip-count track spaces (aid says so; do it) |
| §13/§15 a creation line corrupts and takes the node | lookup | 2 (Plant Seizure, Night Raid) | the node loss comes through corruption, stated in §6 and §15 but not on the project | **W** add "a node there is lost" to creation-line projects |
| §16 cordon and the compute step | lookup, memory | 1 (Night Raid inside the cordon window) | "until the end of the next adversary turn" with step 4 of Income: needs §6's step list | **A** "off at Income step 4" beside the token |
| §17 compute step sum | modifier | 7 | income + botnets + waiting, 3-4 terms, then the overflow onto the next project | **C** a running compute marker per project (already ruled) |
| §9 production by recount | bookkeeping | 7 rounds, 14-19 checks | the track method assumes few changes; with nodes added every round I recounted 5 cities x 2 seats | **A** keep the per-city recount as the fallback the aid names |
| §20 power order | lookup | 2 (Overcharge) | the tiered power table is one dense cell per board | **W** one row per power |
| rulebook length | lookup | 100 lookups | 11-16 lookups a round with no drop-off | **A** a one-page adversary-turn checklist would take about 5 a round |

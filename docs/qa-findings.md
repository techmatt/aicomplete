# QA findings

What a bug hunt reads first: the findings still open, the reports ruled won't-fix (never re-report one), and the
faults past hunts kept finding. Fixed findings are in git (the 020 hunt, r10 → 11, and the 044 hunt, r20 → 21, numbered
F1–F100; the full ledger is `git show 1617d75:docs/qa-findings.md`: don't re-report one; the 099 hunt's, r38 → 39, F101–F131, are in
`experiments/results/099_bug_hunt_r38/fixed.md`). A new finding takes the next number: F132, D41, W19. Each is **fixed** (one line, the change in git),
**deferred** to Matt (options and a recommendation) or **won't fix** (the reason).

## Checkers

`web/scripts/sweep.mjs` (the viewer: web/README.md *The sweep*; 099 added r35-r38 cases, moments on the engine's
counters, `no-control` and `AIC_SWEEP_SEED`), `web/scripts/legal-probe.mjs` (`npm run probe`: a card kept and never
played, legal or not, over saved records), `web/scripts/odd.mjs` (odd input, `npm run odd`:
*Odd input* there), `crates/aic-core/src/tests/invariants.rs` (the rules, the words players read, nothing paid for
nothing), `scripts/wasm-parity.mjs` (native = wasm over the variant matrix) and the e2e's checks (words on screen, the
rules revision, step time). What they already catch, so a hunt needn't hand-check it:

- The sweep flags step-time spikes over 150 ms, `raw-id` (a card or threat named by id), `clipped` (text cut with no
  ellipsis: fails the run), `truncated-no-hover`, `console` errors and `token-unknown` (a token drawn with the default
  glyph).
- `invariants.rs` fails on a raw token key in any label, and `no_action_is_paid_for_nothing` on an action that costs
  something and changes nothing.
- The adversary's twist loop names where each twist op is read; an unread one panics (a catch-all arm once hid a
  twist that never ran).
- The sweep's `no-control` (099) flags a legal action with no button or hex pick on screen.

## Where hunts found faults

Each hunt found most of its faults in the same places. Look there first:

- **Options with nothing to act on**: a move onto the hex the ambassador already holds, a removal on a hex holding only
  what it can't remove, a second Shelter or cut, a cost knob at 0 making a free endless action. The invariants test
  catches what a seed reaches; a card's own target test (`modules/effects/feasible.rs`) is where the rest hide.
- **Words that contradict the code**: hand-written threat and card texts, the rules (one hunt found thirteen
  statements the code contradicted), architecture.md and README. Check each against what runs.
- **Raw names in what players read**: ids, token keys, hex and country indices in prompts, notes, traces and labels.
- **Printed cost against charge**: a button or prompt showing the printed base where a discount or surcharge applies.
- **Report statistics**: a per-round mean that counts games which never reached the round, or games already over;
  a column whose label doesn't say what it averages.
- **The hunt's own fixes**: every independent review of a hunt's diff found five to eight faults in it. Get one.
- **Untestable here**: the scene tools' kill of a hung draw on Windows is not exercised (it needs a real hang).
- **An offer judged before its price** (099): Rush Build's energy and sacrifice; a gift at one country. Random games
  rarely reach them: write the state.
- **Measures gone quiet** (099): a definition matching nothing after a rules change, a counter under the module that ran
  it, 2 seats written in. An empty column reads as a measurement.
- **Kept, never played** (099): keep plan and play check price a card differently. `play_read.py`, then `npm run probe`.

## Open, deferred to Matt

| # | Area | Finding | Options, and the pick |
|---|---|---|---|
| D2 | variants | The off-variants don't measure what a module adds (the rules say so): with `alignment.variant=off` nothing tips, so botnets always count against the purge; with `cards.variant=off` only Decommission removes a botnet; `threats.variant=off` reads low because Unity leans on the threats' churn. | (a) Leave them, documented; (b) a stand-in (alignment off: botnets never count; cards off: a botnet-removal standard action); (c) retire them. **Open:** (b) for alignment only if Matt still uses it to measure Alignment. |
| D6 | agents | After a mid-step reshuffle of a drawn-out deck, the pending-choice guess keeps the true RNG for **every** stream, not just the one that reshuffled (`determinize.rs`, the `drawn_out` branch). Rare; the comment admits one reshuffled order is shown. | (a) Keep the true stream only for the pile that reshuffled (map pile to RNG purpose); (b) leave it. **Pick (b)** until a balance verdict hinges on late Discord or Unity draws. |
| D8 | cards | `move_ambassadors` (Transmission Corridor, up to 2) offers every piece in reach (`targets::pieces_in_reach`), including the hex the seat's only ambassador already holds: picking it spends a move on nothing. `move_one` filters with `move_one_dest`; this loop doesn't. | (a) Share the board move's destination test and use it in the loop; (b) leave it. **Pick (a)**: a two-line fix. |
| D16 | rules text | A Harden or a "you may move" with no target doesn't veto a card that also does something else (Storm Hardening, Peacekeeper Rotation, Transmission Corridor): the card plays and that clause does nothing. Only a card that would do nothing at all is not offered, as card-grammar.md says. | (a) Keep that reading; (b) the strict reading, a veto, which would keep such cards unplayable until their clause has a target. **Pick (a)**: (b) is a balance change for no gain in clarity. |
| D23 | agents | The keep plan prices energy and antimatter at the table (4.5, 7), the play check at the eval's weights (about 1.3, 3.3): Capacitor Discharge, Positron Storage Ring, Tidal Barrage kept (19%, 16%, 7% of 400 engine games) and never played (the probe: legal, unplayed). | (a) The plan reads the eval's rates when `table_energy` / `table_antimatter` are off; (b) turn those on. **Pick (a)**, then a band read. |
| D24 | agents | `token_worth` values a token on a here-cash-out card (Exabyte Archive, Bonded Warehouse, Sovereign Capital Fund) at the held rate 1, not its cash-out rate; Exabyte also covers a science building worth more a round. Kept 15%, played 0. | (a) Price such a token at its cash-out rate; (b) leave it. **Pick (a)**, with Exabyte's yield a pricing call (Matt's call). |
| D25 | cards | Losing conversions at `values.json` (the break-even rule): Demand Response Grid 3 energy (13.5) → 10 credits, Simulation Cluster 2 energy (9) → 2 draws (8); Data Licensing Deal's 4 a Dataset is under Grad Student Pool's spend (a Dataset → 2 draws, 8), so it is kept 29% and never played though legal; Arc Furnace Complex nets about 2.5 before the building it covers. | Reprice each to a visible gain (e.g. Demand Response 3 → 15, Simulation Cluster 1 energy → 2 draws, Data Licensing 8 a Dataset). **Matt's call** (prices). |
| D26 | agents | Agents value "remove up to N troops" at 7 × N (`value.rs`), the effect ledger at what it removes in play (6 → 3.6): r38's troop prices came from the ledger. | (a) `value.rs` reads the ledger's expected removals; (b) leave it. **Pick (a)**: one table for both. |
| D27 | agents | `converter_city` (2.0) counts only while a seat holds a cube, so the last cube is never spent on Cintamani's or Horizon's converter (096 measured 1.11 uses against 1.10 with it off). | (a) Count it whatever the cubes; (b) leave it. **Pick (a)** at the next agents item; the effect is small. |
| D28 | agents | The eval charges a corrupted station 25 (station + `station_flip`) after its country's megaproject completed, when no cube it makes counts: an agent pays 6 credits and 2 ambassadors to free one for almost nothing. Frees are rare (0.06-0.15 a game). | Scale `station_flip` by the project still open. **Pick it** with D27. |
| D29 | viewer | A dark city (an adversary ambassador on a city, r37/r38) carries no mark: the hover lists the ambassador, nothing says the city makes no energy. | (a) A `dark` country value and a hover line; (b) leave it to the token. **Pick (a)**: r38's rule is the one being read. |
| D30 | viewer | The log and step line name a converter use "the converter of country 2" (they have no view). | Give the label context the country names. **Pick it**; small. |
| D31 | cards | Popular Uprising's face says "then discard it"; the engine discards the stack under it too, as player-cards.md says. "and the cards under it" takes its top box to 5.8 pt. | (a) A shorter wording ("built over" is the house term); (b) leave the face. **Matt's call** (wording). |
| D32 | rules | Any lost Follower (Sabotage, Purge Protocols) leaves Popular Uprising short of 6 for good unless Manifesto Leak refills it; Hostile Buyout's cube lands on a city the liberation's own adversary ambassador darkens. Both follow the rules as written. | Design notes. **Matt's call**. |
| D33 | agents | The eval's dear-power progress term (`power_progress`, eval.rs) still skips powers under 2 tags: dead since r35, as the measure was (F110). | (a) Read `ai::power_is_dear`; (b) retire the term and its weight. **Pick (a)** as a screened lever: it changes play. |
| D34 | agents | The keep plan adds a card's gross table value, not less its price, so spare credits keep cards the play check never plays. The net patch (`experiments/results/099_bug_hunt_r38/`, engine d2, 4 × 100): kept-never-played 12 → 6, wins +3.0 ± 2.3 (32100+), +0.5 ± 2.3 fresh (32200+); 6 cards go never-kept (Killswitch Directive: 0.11-0.14 plays a game → 0). | (a) Net the price and value situational cards in the plan; (b) the patch; (c) leave it. **Pick (a)**. Not landed. |
| D35 | agents | The eval's fallback push (eval.rs, `at_hand >= need`) counts `liberate_at` and misses the city's own adversary ambassador, which cancels the first arrival, so it can land 5 and stall at 4 (F122's rule). | Add the city's adversary ambassadors to `need`. **Pick it** with D33-D34. |
| D36 | sim | `aic-sim strategies` defaults to the random agent: it prints strategies no seat plays. | Default to `strategist`. **Pick it**. |
| D37 | cards | One icon, several scopes: the cleanse cube icon is the same for Reclamation Crew (stations only), Back-Channel Talks (factories only) and Great Liberation (cities too); Peacekeeping Mission's troops ("in its country") and Root Access's botnet ("on or next to it") print the plain icons. The words are right. | (a) A facility or city glyph on the narrowed icons (as Liberate's city); (b) leave the words to carry it. **Matt's call** (card art): (a) keeps "every icon one meaning". |
| D38 | rules | Aligning a dark neutral city spends the ambassador and leaves the city dark (the adversary ambassador stays; a site claim on a station or factory cancels instead). Now in economy.md's table. | (a) Align cancels one adversary ambassador, as a claim does; (b) keep it. **Matt's call**; (a) reads as the table expects. |
| D39 | cards | Station Seizure and Station Coup say "unless enough adversary ambassadors" without the number (`station_flip_at` 2). | Print the number (a test keeps it in step with the tunable). **Matt's call** (wording). |
| D40 | sim | `board.stations_free` (built paying no chips) and `board.stations_freed` (Free a station) read as one. | Rename the first, with board_read.py and an old-name map. **Pick it**. |
| D21 | picks | `picks/card_frames_r2.json` and `scene_style_r1.json` point at images that are gone: opening those pickers shows broken tiles. Superseded rounds. | (a) Delete the two manifests; (b) leave them as a record. **Pick (b)** unless they confuse: their rulings are in the frames and the preamble. |

## Won't fix

| # | Area | Report | Why not |
|---|---|---|---|
| W2 | alignment | "The last-drawn Alignment card is shuffled back before it resolves." | Not a bug: alignment.md (resolved cards shuffle back the moment a draw empties both decks). |
| W3 | view | "`PhaseView.waiting` is stale during a choice in an automatic phase." | Not reached (no choice before round 1's first decision phase in 62 seeds); the viewer reads who decides from the choice. |
| W4 | plan | "`plan::next_build` leaves out ongoing projects (Recruitment Drive)." | It forecasts the build and escalation, as documented; only tests call it (no agent or client reads it), so this is no visible bug. |
| W5 | viewer | The event log shows engine events raw (`var changed · key adversary.supply · delta +1`). | The viewer is debug-first (web/README.md); the board, panels and prompts carry the readable story. |
| W6 | viewer | With the *View* tab open a 4-country step costs up to ~140 ms (it re-renders ~85 KB of view JSON); *Raw state* ~75 ms. | Debug tabs, closed by default; the default log tab steps in ~25–45 ms. |
| W7 | sim | In counter rows "-" means both "never tallied" and "no game reached this round"; `adversary ops at setup` is always 0 (setup's station botnets are printed setup, not operations); a "mean where reached" carries no count of the games that reached it. | Reading aids, not wrong numbers; the per-game records and `games.jsonl` have the detail. If they bite: print the reached count beside each conditional mean. |
| W8 | view | The seat's income in the view leaves out an upkeep conversion (`convert_upkeep`). | No card uses the op now; the income passives the view reads don't model conversions. Fix it if a card brings the op back. |
| W9 | viewer | A grouped hex-pick button's hover shows the raw JSON of its first action only. | A developer aid. |
| W10 | viewer | Below the supported 1366×768 (a 1366 laptop at 150% zoom, 1024×768) the board pane has no scroll fallback: the strips take the board's height. | The stated floor is 1366×768 (style.css, web/README.md); a real 1366×768 window (about 1366×650 of viewport) measured board 248 px, no page scroll. A `min-height` on the board and a scrolling pane would extend it if smaller screens matter. |
| W11 | view | The shared discard piles print insight prices discounted by seat 0's Science tags. | Hover text on a discard pile; the price at play time is right. |
| W12 | adversary | A station that flips keeps a marker, where a corrupted city's marker goes. | No rule says otherwise; war.md's sentence is about placing a marker. A rule to write if it ever matters. |
| W13 | viewer | A record overlaying adversary cards or AI boards shows the printed text. | Documented in experiments.md, *Content levers*; the game plays the overlay. |
| W14 | viewer | Growth buttons name no cost, and Expand with energy for one step places one. | ai-boards.md: a placement you can't pay doesn't happen. The cost is on the track (`pay N energy`). |
| W15 | scripts | `scripts/cards/screen.py`'s "cap" column counts deck-out losses. | That is the round cap: adversary.md's word. |
| W16 | viewer | The sweep's `truncated` issues (thousands): threat and project text and adversary lines cut to fit, with a hover. | The no-scroll budget at 1366×768 (W10); every one has a hover or an ellipsis. |
| W17 | cards | "Liquidity Event is dominated by Bonded Warehouse's cash-out." | It cashes a pile without spending the Action, which can then add. |
| W18 | report | A one-agent run's findings say "skill gap +0 pts; the ladder is ordered". | True of a one-rung ladder. |

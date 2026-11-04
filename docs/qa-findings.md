# QA findings

What a bug hunt reads first: the findings still open, the reports ruled won't-fix (never re-report one), and the
faults past hunts kept finding. Fixed findings are in git (the 020 hunt, r10 → 11, and the 044 hunt, r20 → 21, numbered
F1–F100; the full ledger is `git show 1617d75:docs/qa-findings.md`: don't re-report one). A new finding takes the next number: F101, D23, W17. Each is **fixed** (one line, the change in git),
**deferred** to Matt (options and a recommendation) or **won't fix** (the reason).

## Checkers

`web/scripts/sweep.mjs` (the viewer: web/README.md *The sweep*), `web/scripts/odd.mjs` (odd input, `npm run odd`:
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

## Where hunts found faults

Each hunt found most of its faults in the same places. Look there first:

- **Options with nothing to act on**: a move onto the hex the ambassador already holds, a removal on a hex holding only
  what it can't remove, a second Shelter or cut, a cost knob at 0 making a free endless action. The invariants test
  catches what a seed reaches; a card's own target test (`modules/effects/feasible.rs`) is where the rest hide.
- **Words that contradict the code**: hand-written threat and card texts, rules.md (one hunt found thirteen
  statements the code contradicted), architecture.md and README. Check each against what runs.
- **Raw names in what players read**: ids, token keys, hex and country indices in prompts, notes, traces and labels.
- **Printed cost against charge**: a button or prompt showing the printed base where a discount or surcharge applies.
- **Report statistics**: a per-round mean that counts games which never reached the round, or games already over;
  a column whose label doesn't say what it averages.
- **The hunt's own fixes**: every independent review of a hunt's diff found five to eight faults in it. Get one.
- **Untestable here**: the scene tools' kill of a hung draw on Windows is not exercised (it needs a real hang).

## Open, deferred to Matt

| # | Area | Finding | Options, and the pick |
|---|---|---|---|
| D2 | variants | The off-variants don't measure what a module adds (rules.md §9 says so): with `alignment.variant=off` nothing tips, so botnets always count against the purge; with `cards.variant=off` only Decommission removes a botnet; `threats.variant=off` reads low because Unity leans on the threats' churn. | (a) Leave them, documented; (b) a stand-in (alignment off: botnets never count; cards off: a botnet-removal standard action); (c) retire them. **Open:** (b) for alignment only if Matt still uses it to measure Alignment. |
| D6 | agents | After a mid-step reshuffle of a drawn-out deck, the pending-choice guess keeps the true RNG for **every** stream, not just the one that reshuffled (`determinize.rs`, the `drawn_out` branch). Rare; the comment admits one reshuffled order is shown. | (a) Keep the true stream only for the pile that reshuffled (map pile to RNG purpose); (b) leave it. **Pick (b)** until a balance verdict hinges on late Discord or Unity draws. |
| D8 | cards | `move_ambassadors` (Transmission Corridor, up to 2) offers every piece in reach (`targets::pieces_in_reach`), including the hex the seat's only ambassador already holds: picking it spends a move on nothing. `move_one` filters with `move_one_dest`; this loop doesn't. | (a) Share the board move's destination test and use it in the loop; (b) leave it. **Pick (a)**: a two-line fix. |
| D16 | rules text | A Harden or a "you may move" with no target doesn't veto a card that also does something else (Storm Hardening, Peacekeeper Rotation, Transmission Corridor): the card plays and that clause does nothing. Only a card that would do nothing at all is not offered, and rules.md says so. | (a) Keep that reading; (b) the strict reading, a veto, which would keep such cards unplayable until their clause has a target. **Pick (a)**: (b) is a balance change for no gain in clarity. |
| D21 | picks | `picks/card_frames_r2.json` and `scene_style_r1.json` point at images that are gone: opening those pickers shows broken tiles. Superseded rounds. | (a) Delete the two manifests; (b) leave them as a record. **Pick (b)** unless they confuse: their rulings are in the frames and the preamble. |

## Won't fix

| # | Area | Report | Why not |
|---|---|---|---|
| W2 | alignment | "The last-drawn Alignment card is shuffled back before it resolves." | Not a bug: rules.md Defaults 40 (resolved cards shuffle back the moment a draw empties both decks). |
| W3 | view | "`PhaseView.waiting` is stale during a choice in an automatic phase." | Not reached (no choice before round 1's first decision phase in 62 seeds); the viewer reads who decides from the choice. |
| W4 | plan | "`plan::next_build` leaves out ongoing projects (Recruitment Drive)." | It forecasts the build and escalation, as documented; only tests call it (no agent or client reads it), so this is no visible bug. |
| W5 | viewer | The event log shows engine events raw (`var changed · key adversary.supply · delta +1`). | The viewer is debug-first (web/README.md); the board, panels and prompts carry the readable story. |
| W6 | viewer | With the *View* tab open a 4-country step costs up to ~140 ms (it re-renders ~85 KB of view JSON); *Raw state* ~75 ms. | Debug tabs, closed by default; the default log tab steps in ~25–45 ms. |
| W7 | sim | In counter rows "-" means both "never tallied" and "no game reached this round"; `adversary ops at setup` is always 0 (setup's station botnets are printed setup, not operations); a "mean where reached" carries no count of the games that reached it. | Reading aids, not wrong numbers; the per-game records and `games.jsonl` have the detail. If they bite: print the reached count beside each conditional mean. |
| W8 | view | The seat's income in the view leaves out an upkeep conversion (`convert_upkeep`). | No card uses the op now; the income passives the view reads don't model conversions. Fix it if a card brings the op back. |
| W9 | viewer | A grouped hex-pick button's hover shows the raw JSON of its first action only. | A developer aid. |
| W10 | viewer | Below the supported 1366×768 (a 1366 laptop at 150% zoom, 1024×768) the board pane has no scroll fallback: the strips take the board's height. | The stated floor is 1366×768 (style.css, web/README.md); a real 1366×768 window (about 1366×650 of viewport) measured board 248 px, no page scroll. A `min-height` on the board and a scrolling pane would extend it if smaller screens matter. |
| W11 | view | The shared discard piles print insight prices discounted by seat 0's Science tags. | Hover text on a discard pile; the price at play time is right. |
| W12 | adversary | A station that flips keeps a marker, where a corrupted city's marker goes. | No rule says otherwise; rules.md's sentence is about placing a marker. A rule to write if it ever matters. |
| W13 | viewer | A record overlaying adversary cards or AI boards shows the printed text. | Documented in experiments.md, *Content levers*; the game plays the overlay. |
| W14 | viewer | Growth buttons name no cost, and Expand with energy for one step places one. | rules.md: a placement you can't pay doesn't happen. The cost is on the track (`pay N energy`). |
| W15 | scripts | `scripts/cards/screen.py`'s "cap" column counts deck-out losses. | That is the round cap: rules.md's word for it. |
| W16 | viewer | The sweep's `truncated` issues (thousands): threat and project text and adversary lines cut to fit, with a hover. | The no-scroll budget at 1366×768 (W10); every one has a hover or an ellipsis. |

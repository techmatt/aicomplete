# QA findings

Every bug-hunt finding, once: **fixed** (one line: what, and that it is fixed; the change is in git), **deferred** to Matt
(with the options and a recommendation) or **won't fix** (with the reason). Read this before a hunt and never re-report an
item on it. Checkers: `web/scripts/sweep.mjs` (the viewer, web/README.md *The sweep*), `web/scripts/odd.mjs` (odd input,
*Odd input* there), `crates/aic-core/src/tests/invariants.rs` (the rules, the words players read, nothing paid for nothing),
`scripts/wasm-parity.mjs` (native = wasm over the variant matrix) and the e2e's checks (words on screen, the rules
revision, step time).

## 020 bug hunt (rules revision 10 → 11, 2026-10-29)

### Fixed

| # | Area | Finding, fixed |
|---|---|---|
| F1 | threats / board | An uncountered Blackout or Sabotage did nothing but its compute (cleanup cleared the flag first). Fixed, r11: the flag lasts through the next round's production and upkeep. **Matt to confirm** the "next round" reading (the alternative is `threats.execute` before `board.produce`). |
| F2 | board | Move was offered onto the hex the seat's only ambassador already held, for 1 energy and no move. Fixed, r11. |
| F3 | board | Troop strike options and notes read raw token keys (`presence:3 at C3 #3`). Fixed (`common::piece_name`). |
| F4 | view | The seat value `presence_reserve` showed its raw key. Fixed. |
| F5 | threats / view | The threat tier global showed this round's tier labeled as next round's. Fixed. |
| F6 | rules.md | Difficulty 2's clock length was documented wrong. Fixed. |
| F7 | rules.md | The `clear_win_stage` hold on the purge was documented at every level, not level 0 only. Fixed. |
| F8 | agents | The heuristic's corrupt forecast read the empty face-up slot between build and reveal. Fixed. |
| F9 | agents (test) | A pending-choice guess test didn't exempt a project row dealt from the hidden deck. Fixed. |
| F10 | sim | `aic-sim --help` said "unknown command" and exited 2. Fixed. |
| F11 | sim | `aic-sim experiment` console lines didn't name the config and variant. Fixed. |
| F12 | viewer | Stepping through a long game was slow (the event log rebuilt every step). Fixed; the sweep flags step-time spikes over 150 ms. |
| F13 | viewer | Hexes were named by country index (`C1 #13`). Fixed (`placeNames`). |
| F14 | viewer | Options and prompts named cards and threats by id. Fixed; sweep: `raw-id`. |
| F15 | viewer | A saved session restored silently with a config that had since changed. Fixed: it says so on load. |
| F16 | viewer | A saved session that no longer loads showed its reason for an instant. Fixed. |
| F17 | viewer | Two answers to one hex choice on the same hex were one click. Fixed: a button each. |
| F18 | viewer | Troop arrows caught the mouse during a hex pick. Fixed. |
| F19 | viewer | The deck box's `top: …` line clipped with no ellipsis or hover. Fixed; sweep: `clipped`. |
| F20 | viewer | Every page load logged a `favicon.ico` 404. Fixed; sweep: `console`. |
| F21 | viewer | A record from another rules revision said only "invalid config". Fixed: it names both revisions. |
| F22 | viewer | A resource at 0 with no income was left out of a seat's panel. Fixed. |
| F23 | view | Country token totals were labeled with token keys. Fixed; the invariants test fails on a raw token key in any label. |
| F24 | board | The presence leak took adversary ambassadors belonging to a city, base or station on the hex. Fixed, r11: only a lone presence leaks. |
| F25 | board | A Sabotage flag also skipped the presence leak, which no rule says. Fixed, r11. |
| F26 | economy / view | A sabotaged seat's compute income showed its full `+N`. Fixed (`economy::upkeep_income`). |
| F27 | config | `board.chips_credits`, `ambassador_credits` or `harden_hardware` at 0 made a free endless action. Fixed: their minimum is 1. |
| F28 | config | `alignment.undecided_per_player` below `tip_per_player` was accepted. Fixed: rejected with a reason. |
| F29 | sim | `--resume` refused a `--quick` run without saying what differed. Fixed. |
| F30 | docs | README and rules.md misstated `<module>=off`, the build rate and the boards' tier-2 timing. Fixed. |
| F31 | sim | `aic-sim replay` called a viewer bug report saved mid-game a "timeout". Fixed. |
| F32 | review | An independent review of the hunt's own diff found eight faults in its fixes. All fixed. |
| F33 | viewer | The game-over line led with the reason code. Fixed. |
| F34 | viewer | Keys and buttons pressed before the engine loaded threw. Fixed. |
| F35 | viewer | Autoplay moved the cursor without saving the session. Fixed. |
| F36 | alignment / projects | Purge the Unaligned offered free hexes, not the seat's presence, so its sacrifice cost nothing. Fixed. |
| F37 | content | Threat texts (Overrun, Mass production, Infiltration) contradicted the code. Fixed. |
| F38 | viewer | Options and prompts named countries "country 0". Fixed. |
| F39 | checker | The sweep's key-event moments never fired, with several other sweep and e2e faults. All fixed. |
| F40 | sim | `*_round` counters averaged in games without the event as round 0. Fixed: "mean where reached". |
| F41 | view | A board played by two seats listed the second seat's unique cards as "gone". Fixed. |
| F42 | adversary | Infiltration Drive's log note said "Discord". Fixed. |
| F43 | view / viewer | A project discarded by Zero-Day Wipe counted as resolved. Fixed. |
| F44 | sim | A round's own counters averaged in games already over. Fixed: "/game reaching it". |
| F45 | sim | "Game length for the strongest agent" reported only the first variant, unlabeled. Fixed. |
| F46 | sim | `ms/game` read as one game's cost. Fixed: labeled `wall ms/game (all threads, incl. replay)`. |
| F47 | sim reports | report.md's countered share, skill-gap header and skill-gap finding were wrong or unlabeled. Fixed. |
| F48 | viewer | Panels styled countdowns as warnings, left out nodes, put non-loss tracks under "Lose if" and showed insight as "+4". Fixed. |
| F49 | viewer | War-step links and hover dot colors didn't match the board. Fixed. |
| F50 | core | The engine named hexes and countries by index in prompts, notes and traces (first filed as D5). Fixed, r11 (`hex_name`, `country_opt`). |
| F51 | review | A second independent review of the hunt's diff found five more faults. All fixed. |
| F52 | economy | `economy.give_insight` 0 still offered Give. Fixed. |
| F53 | viewer / docs | Record revision errors, keys lost while the slider had focus, and a misplaced README line. Fixed. |
| F54 | docs | architecture.md's `requires` table, config sample, view fields and names were wrong. Fixed. |
| F55 | scripts | `screen_019.py` missed troops placed by threats; `tune/sweep.py` crashed on a cp1252 stdout. Fixed. |
| F56 | config / sim | Bad `--set` values, unknown modules, empty decks and zero-valued agent specs were silent or misread. Fixed. |
| F57 | viewer | Space on a focused button, a record dropped under the open dialog, an unreadable file failing silently. Fixed. |
| F58 | viewer | Focus fell to the page body after each action. Fixed. |
| F59 | viewer | A resource chip for a resource the seat never held had no hover. Fixed. |
| F60 | sim | `show --set-a`, `replay` and `modules` silently ignored options they don't take. Fixed: refused. |
| F61 | sim / records | Partial reports counted empty cells as 0%; a bad record seed wasn't named. Fixed. |

### Deferred to Matt

| # | Area | Finding | Options, and the pick |
|---|---|---|---|
| D1 | board | A player ambassador on an adversary-owned factory sat there forever. | Fixed, (a): it is used up for `factory_unity` and the troop is still made. |
| D2 | variants | The **off-variants don't measure what a module adds**: `alignment.variant=off` and `cards.variant=off` are unwinnable (nodes always count against the purge and only a card removes them); `threats.variant=off` reads low because Unity leans on the threats' churn. | (a) Leave them, and say so in rules.md §9 (done); (b) give the off-variants a stand-in (alignment off: nodes never count; cards off: a node-removal standard action); (c) retire them. **Open:** (b) for alignment only if Matt still uses it to measure Alignment. |
| D3 | viewer | The adversary project row was unreadable at 1366×768. | Fixed, (a): two-line project boxes, hover for the rest. |
| D4 | viewer | Watching a record had no speed control or round jump. | Fixed, (a): a speed menu and Page Up / Page Down. |
| D6 | agents | After a mid-step reshuffle of a drawn-out deck, the pending-choice guess keeps the true RNG for **every** stream, not just the one that reshuffled (`determinize.rs`, the `drawn_out` branch): rare, and the comment admits one reshuffled order is shown. | (a) Keep the true stream only for the pile that reshuffled (map pile to RNG purpose); (b) leave it. **Pick (b)** until the agents are used for balance verdicts that hinge on late Discord or Unity draws. |
| D7 | viewer | The adversary's whole turn landed in one step. | Fixed, (a): one folded log group per phase, with hex outlines on hover. |
| D8 | cards | The **Evacuate** card (`Effect::MoveAmbassadors`) has F2's flaw: it offers the hex the seat's only ambassador already holds, and picking it spends one of the card's 3 moves on nothing. | (a) Share F2's destination test as a board helper and use it in the card's loop; (b) let the card rework drop it. **Pick (a)** if Evacuate survives the rework: a two-line fix. |
| D9 | agents | The heuristic priced a permanent node like two ambassadors, so agents made nodes when an equal free choice existed. | Fixed, (a): `w.node` 25 for every agent. |
| D10 | AI boards / cards | Hardening could go on adversary-owned buildings, and both of "harden 2" on one building. | Fixed, (a): player-owned uncorrupted facilities, N different ones. |
| D11 | threats | An extra threat dealt after the counter phase executed the same round, uncounterable. | Fixed, (a): it is stamped `threats.late.<id>` and waits a round. |
| D12 | threats / content | The tier-2 threat deck was too small for 4 countries. | Fixed: four more tier-2 threats and the `threats.short` tally. |
| D13 | adversary / alignment | With `alignment.adversary_discord` ≥ 1, a Sway-phase Discord tip read last round's card twists. | Fixed, (a): before the build step the twists are the face-up card's. |
| D14 | config | A non-integer parameter value failed with serde's untagged-enum message and no key. | Fixed, (a): the message names the key. |

### Won't fix

| # | Area | Report | Why not |
|---|---|---|---|
| W2 | alignment | "The last-drawn Alignment card is shuffled back before it resolves." | Verified not a bug: matches rules.md Defaults 40 (resolved cards shuffle back the moment a draw empties both decks). |
| W3 | view | "PhaseView.waiting is stale during a choice in an automatic phase." | Not reached in practice (no choice before round 1's first decision phase in 62 seeds), and the viewer shows who decides from the choice, not from `waiting`. |
| W4 | plan | "plan::next_build leaves out ongoing projects (Recruitment Drive)." | `next_build` forecasts the build and escalation, as documented; the agents use its total only for supply slack, so a missed Recruitment Drive is an agent-tuning gap, not a visible bug. |
| W5 | viewer | The event log shows engine events raw (`var changed · key adversary.supply · delta +1`). | The viewer is debug-first (web/README.md); the board, panels and prompts carry the readable story. |
| W6 | viewer | With the *View* tab open, a 4-country step costs up to ~140 ms (it re-renders 85 KB of view JSON); *Raw state* ~75 ms. | Debug tabs, closed by default; the log tab (the default) steps in ~25–45 ms. |
| W7 | sim | In counter rows "-" means both "never tallied" (zero in every game) and "no game reached this round"; `adversary ops at setup` is always 0 (setup's station nodes are printed setup, not operations); a "mean where reached" carries no count of the games that reached it. | Reading aids, not wrong numbers; the per-game records and `games.jsonl` have the detail. If they bite: print the reached count beside each conditional mean. |
| W8 | economy / cards | The seat's `+antimatter` / `+energy` income in the view leaves out a building's upkeep conversion (Antimatter Refinery: +1 antimatter for 4 energy in upkeep). | Card content the rework replaces: the income passives the view reads don't model conversions. |
| W9 | viewer | Hover details: a board name drawn from the config fallback (a record whose view has no seat board) has no hover; a grouped hex-pick button's hover shows the raw JSON of its first action only. | Debug-level: the engine always sends the board (the unreachable fallback is gone), and the raw action JSON is a developer aid. |
| W10 | viewer | Below the supported 1366×768 (150% zoom on a 1366 laptop, 1024×768) the board pane has no scroll fallback: the strips take the board's height. | The layout's stated floor is 1366×768 (style.css, web/README); a real 1366×768 window (about 1366×650 of viewport) measured board 248 px, no page scroll. A `min-height` on the board and a scrolling pane would extend it if smaller screens matter. |

## 044 bug hunt (rules revision 20 → 21, 2026-10-30)

Focus: revisions 18–20, the overlay and gate, the scene art tools.

### Fixed

| # | Area | Finding, fixed |
|---|---|---|
| F62 | adversary | Stage-III cards' "2 Sway leaves the Unity bucket" never happened (the twist loop dropped `PullUnity`). Fixed, r21. |
| F63 | cards | A card or ability that only hardens or moves one ambassador was offered with nothing to act on. Fixed, r21: not offered when the card does nothing else (see D16). |
| F64 | cards | A small removal was offered on a hex holding only a troop it couldn't remove. Fixed, r21 (`adversary::can_damage`). |
| F65 | cards | A cash-in was refused as "no target" though the same card adds the tokens first. Fixed, r21. |
| F66 | cards | "Up to N" Science ops (Clear, Shield, Delay) forced N. Fixed, r21. |
| F67 | config | `stage_cards` was checked against the wrong deck at each difficulty. Fixed: checks what the difficulty deals. |
| F68 | core / viewer | Placement prompts named the printed energy cost, not the charge. Fixed. |
| F69 | viewer | The presence and rich-action buttons showed the printed base, not the charge. Fixed. |
| F70 | view / viewer | A presence step's printed cost reached no client. Fixed: a `pay N energy` mark on the step. |
| F71 | viewer | The corruption marker fell to the default glyph. Fixed; sweep: `token-unknown`. |
| F72 | viewer | A waiting or jammed threat looked like any other. Fixed. |
| F73 | viewer | The "Lose if" group left out presence. Fixed. |
| F74 | view | Clear's button didn't say what the reward is. Fixed. |
| F75 | viewer | The second track's first step showed "in effect" with nothing uncovered. Fixed. |
| F76 | core | A raw id in a prompt ("harden_buy on which hex?"). Fixed. |
| F77 | viewer | Card text clipped with no ellipsis. Fixed: cards grow with their text; sweep: `clipped` fails the run. |
| F78 | faces | Move-one text said "an ambassador on the board" where only your own move. Fixed. |
| F79 | view / core | Shelter's hover and prompt said buildings only, though it protects presence too. Fixed. |
| F80 | rules.md | Stage-III presence count, the insight split and `needs` wording were wrong. Fixed. |
| F81 | sim reports | Short gate checks led the report's top agent; multi-config gate rows hid their config; `compare` printed one content hash under an overlay. Fixed. |
| F82 | scripts | `speed.py pair` ran old, old, new, new instead of old, new, new, old. Fixed. |
| F83 | docs | README's overlay example, preset list and web/README's legend were stale. Fixed. |
| F84 | scene tools | Hung draws never timed out on Windows, plus several scene-gen and picker faults. All fixed (the kill itself is not exercised: it needs a real hang). |
| F85 | checker | The sweep's key-event moments were masked by the round moment, and it lacked several moments and cases. Fixed. |
| F86 | content | Hand-written texts (Fifth Column, Occupation, Purge protocols, Total war, Decommission, Consolidate) contradicted what runs. Fixed. |
| F87 | economy | Smear campaign's lost insight was misexplained in the seat chip. Fixed. |
| F88 | view | A drafted card showed a keep price during an Open Market free purchase. Fixed: "keep free". |
| F89 | viewer | A card's insight price showed only in the hover. Fixed: a chip on the card and the price on the button. |
| F90 | threats | A seat that lost its presence in a country couldn't withdraw a payment there. Fixed, r21. |
| F91 | view | `alignment.variant=off` showed antimatter actions that don't exist. Fixed. |
| F92 | docs (generated) | The harms ledger undercounted a printed corruption; the standard-actions table said Harden is Protector only. Fixed. |
| F93 | rules.md | Thirteen statements the code contradicted. Fixed. |
| F94 | docs | architecture.md, principles.md and README had stale timing, fields, specs, weights and terms. Fixed. |
| F95 | cards | A second Shelter or a second cut was offered and paid for with no effect. Fixed, r21 (test `no_action_is_paid_for_nothing`). |
| F96 | adversary | The twist loop's catch-all arm hid F62. Fixed: every twist op names where it is read; an unread one panics. |
| F97 | checker | No check tried odd input or watched for paid-for-nothing. Fixed: `web/scripts/odd.mjs` (`npm run odd`). |
| F98 | review | An independent review of the hunt's diff found eight faults in its fixes. All fixed. |
| F99 | content | The AI boards' energy-for-credits rate appeared on no board or view. Fixed: the innate text says it. |
| F100 | viewer | The Lose-if row could wrap at 1366×768; the step label had no hover. Fixed. |

### Deferred to Matt

| # | Area | Finding | Options, and the pick |
|---|---|---|---|
| D15 | adversary balance | The gate-041 screens were played without F62's Unity pull, and r21 removed some dud plays; a 32-seed planner read from r20 to r21 moved within noise, but no gate saw r21 then. | (a) Rerun `experiments/archive/gate-041.json` at r21; (b) leave it until the next revision's screen. **Pick (a)**, about an hour at below-normal priority: it is the cap's "harder end", which Matt's ruling leaned on. |
| D16 | rules text | rules.md says a program or event is offered only when *each* targeted effect has a target. A Harden or a "you may move" with no target doesn't veto a card that also gains something (Blackstart Protocol, Shuttle Diplomacy; 15 cards): the card plays and that clause does nothing. F63 vetoes only cards that would do nothing at all, and rules.md says so. | (a) Keep F63's reading: the card is still worth playing; (b) the strict reading: a veto, which would stop Aegis Grid Lockdown in the early rounds. **Pick (a)**: (b) is a real balance change for no gain in clarity. |
| D17 | rules / Science | A shield on a presence standing on the seat's own building outlived the presence. | Done, (a): rules.md says the token stays and protects the building. |
| D18 | Discord text | "3+ Discord tips" steps read the count before the tip, but the card read as the 3rd tip. | Done, (a): printed "after 3 Discord tips". |
| D19 | card icons | Delay and Jam printed the same icon. | Done, (a): a clock icon for Delay. |
| D20 | agents | "No peeking" leaked which Alignment cards a game dealt. | Fixed, r23: the guess deals each Alignment pile from the unresolved printed cards, the planner's lines too. |
| D21 | picks | `picks/card_frames_r2.json` and `scene_style_r1.json` point at 51 images that are gone (their folders are empty): opening those pickers shows broken tiles. Superseded rounds. | (a) Delete the two manifests; (b) leave them as a record of the choice. **Pick (b)** unless they confuse: their rulings are recorded in the frames and the preamble. |
| D22 | scene art | The framing line's subject band was wider than a building's window. | Done, (a): "the middle two-thirds" in the framing line. |

### Won't fix

| # | Area | Report | Why not |
|---|---|---|---|
| W11 | view | The shared discard piles print insight prices discounted by seat 0's Science tags. | Debug-level: hover text on a discard pile; the price at play time is right. |
| W12 | adversary | A station that flips keeps a marker, where a corrupted city's marker goes. | No rule says otherwise; rules.md's sentence is about placing a marker. A rule to write if it ever matters. |
| W13 | viewer | A record overlaying adversary cards or AI boards shows the printed text. | Documented in README's overlay paragraph; the game plays the overlay. |
| W14 | viewer | Growth buttons name no cost, and Expand with energy for one step places one. | rules.md's rule: a placement you can't pay doesn't happen. F70 puts the cost on the track. |
| W15 | sim | `screen.py`'s "cap" column counts deck-out losses. | That is the round cap: rules.md's word for it. |
| W16 | viewer | The sweep's `truncated` issues (6,405 at r21): threat and project text and adversary lines cut to fit, with a hover. | The no-scroll budget at 1366×768 (D3, W10); every one has a hover or an ellipsis. |

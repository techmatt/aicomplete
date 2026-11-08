# QA findings

What a bug hunt reads first: the findings still open, the reports ruled won't-fix (never re-report one), and the
faults past hunts kept finding. Fixed findings are in git (the 020 hunt, r10 → 11, and the 044 hunt, r20 → 21, numbered
F1–F100; the full ledger is `git show 1617d75:docs/qa-findings.md`: don't re-report one; the 099 hunt's, r38 → 39, F101–F131, are in
`experiments/results/099_bug_hunt_r38/fixed.md`, the 118 hunt's, r46 → 47, F132 on, in `experiments/results/118_bug_hunt_r46/fixed.md`, its D43 on and W19 on in full
in `deferred.md` there; the 145 hunt's, r59 → 60, F165 on and D57 on in full in `experiments/results/145_bug_hunt/`).
A new finding takes the next number after the last in either list (D, W below). Each is **fixed** (one line, the change in git),
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
- 118: `scripts/cards/runaway_read.py` (loops in a run's counters), `aic-sim docs --check` on a rules quote a card no
  longer prints, wasm-parity's engine games (r41+ engines), the sweep's r41+ moments and strategist and 1-player cases.
- 145: `scripts/cards/counter_names.py` (counter keys a read script quotes that no game writes), the sweep's r55-r59
  moments (claims, liberations, Envoy, hold-offs, node loss by cause, Corrupt X by site) and d3 cases.

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
- **Rules prose quoting a card** (118): each card pass leaves the rules' examples a revision behind (fifteen in one
  hunt); `docs --check` now catches a quoted face.
- **Who an effect belongs to** (118): a trigger inside a teammate's action, a counter fed by another card.
- **Agents matching words** (118): a prompt's rewording turned an agent heuristic off; grep the agents for a prompt.
- **Order inside a round** (145): a Discord tip in the Sway phase read last round's card; a project acted in the build
  that filled it; a troop rule spent an ambassador where no troop came. Ask what has happened yet at each phase.
- **One op, two wordings** (145): faces of the same op printed differently (face down, standing troops, pay or
  discard), and a face's "here" with two tokens. Group faces by op and read them side by side.

## Open, deferred to Matt

Closed by 121 and 124 (git has them): D16, D43, D44, D47, D48, D51, D53, D54.

| # | Area | Finding | Options, and the pick |
|---|---|---|---|
| D2 | variants | The off-variants don't measure what a module adds (the rules say so): with `alignment.variant=off` nothing tips, so botnets always count against the purge; with `cards.variant=off` only Decommission removes a botnet; `threats.variant=off` reads low because Unity leans on the threats' churn. | (a) Leave them, documented; (b) a stand-in (alignment off: botnets never count; cards off: a botnet-removal standard action); (c) retire them. **Open:** (b) for alignment only if Matt still uses it to measure Alignment. |
| D6 | agents | After a mid-step reshuffle of a drawn-out deck, the pending-choice guess keeps the true RNG for **every** stream, not just the one that reshuffled (`determinize.rs`, the `drawn_out` branch). Rare; the comment admits one reshuffled order is shown. | (a) Keep the true stream only for the pile that reshuffled (map pile to RNG purpose); (b) leave it. **Pick (b)** until a balance verdict hinges on late Discord or Unity draws. |
| D32 | rules | Any lost Follower (Sabotage, Purge Protocols) leaves Popular Uprising short of 4 for good unless Manifesto Leak refills it; Hostile Buyout's cube lands on a city the liberation's own adversary ambassador darkens. Both follow the rules as written. | Design notes. **Matt's call**. |
| D41 | rules | Aligning a dark city cancels one adversary ambassador (r40, D38), but Remove costs the same ambassador, clears up to 3 and stays: no agent took the cancel in 1,200 r40 games (102). | (a) Leave it: the rule reads as a site claim does; (b) let Align on a dark city both cancel and align; (c) price Remove up. **Matt's call**; (a) until a player asks for it. |
| D42 | cards | Data Licensing Deal (10 a Dataset at r40) is still unplayed: it needs a Dataset on a card, and the agents spend Grad Student Pool's and Telemetry Archive's at once, so the 4-Dataset pile it pays back from never stands; cost 1 or 12 a Dataset played it 0.01 a game (102). | (a) A Dataset building that holds its pile (a cap, no spend branch); (b) cash from every pile at once; (c) leave it. **Matt's call**. |
| D21 | picks | `picks/card_frames_r2.json` and `scene_style_r1.json` point at images that are gone: opening those pickers shows broken tiles. Superseded rounds. | (a) Delete the two manifests; (b) leave them as a record. **Pick (b)** unless they confuse: their rulings are in the frames and the preamble. |
| D45 | engine | Fight triggers read the board's net count: a removal plus a placement in one action fires nothing. | Count at the removal ops. **Pick** when common. |
| D46 | loops | Drain with Security Ops Floor and Intrusion Lab: ~12 for 10 credits, while projects hold compute. | Leave it: compute caps it. **Pick**. |
| D49 | scripts | `card_ranks.py`'s keep rate counts opening keeps (over 100%); speed.md's bar sits on it. | A draft-keep counter, re-measure. **Pick**. |
| D52 | viewer | The timeline keeps a state a step (~200 MB a 4-country game; a blocking reload). | Keep every Nth snapshot. **Pick** if needed. |
| D55 | rules | At 1 country no military base: base arrivals, Escalation, Total War's base ambassadors place nothing. | **Matt's call**. |
| D56 | content | `Content::check` lets through ops that never run: a threshold or `here` op on an AI board, the Science niche or `gain_per_tokens` in a trigger's ops. | Widen the checks (F196 did two). **Pick**. |
| D50 | viewer | A new player's notes from two real-speed sessions: [viewer-notes.md](../experiments/results/118_bug_hunt_r46/viewer-notes.md). | Picks there. |
| D57 | cards | Exascale Training Cluster kept in 59-72% of games, played in 1-3% (r62: 5 energy, override cut; 10% even at 3 energy): the keep plan wants it, the play eval doesn't. | Fix the keep plan (agents), then more power **Matt's call** (148). |
| D69 | card art | Power Purchase Agreement and Microgrid Lab Compact have no art scene. | Write and render them. **Matt's call**. |
| D58 | speed | Greedy 1.63× a decision r58 → r59, 3.0× r60 → r61: settling the loss prompts threat execution asks (pay or discard, nearest ties) in `search::settled` (147's profile). | A cheaper settle of an uncountered threat's loss prompts (moves greedy's play). **Pick**. |
| D66 | docs | principles.md's credits a game and card-pricing's cost range unmeasured at r60. | Re-read, restate. **Pick**. |

## Won't fix

| # | Area | Report | Why not |
|---|---|---|---|
| W2 | alignment | "The last-drawn Alignment card is shuffled back before it resolves." | Not a bug: a drawn card goes face up on its side's resolved pile before it resolves, and neither deck ever reshuffles (alignment.md); an empty deck ends the game instead. |
| W3 | view | "`PhaseView.waiting` is stale during a choice in an automatic phase." | Not reached (no choice before round 1's first decision phase in 62 seeds); the viewer reads who decides from the choice. |
| W4 | plan | "`plan::next_build` leaves out ongoing projects (Recruitment Drive)." | It forecasts the build and escalation, as documented; only tests call it (no agent or client reads it), so this is no visible bug. |
| W5 | viewer | The event log shows engine events raw (`var changed · key adversary.supply · delta +1`). | The viewer is debug-first (web/README.md); the board, panels and prompts carry the readable story. |
| W6 | viewer | With the *View* tab open a 4-country step costs up to ~140 ms (it re-renders ~85 KB of view JSON); *Raw state* ~75 ms. | Debug tabs, closed by default; the default log tab steps in ~25–45 ms. |
| W7 | sim | In counter rows "-" means both "never tallied" and "no game reached this round"; a "mean where reached" carries no count of the games that reached it. | Reading aids, not wrong numbers; `games.jsonl` has the detail. |
| W9 | viewer | A grouped hex-pick button's hover shows the raw JSON of its first action only. | A developer aid. |
| W10 | viewer | Below the supported 1366×768 (a 1366 laptop at 150% zoom, 1024×768) the board pane has no scroll fallback: the strips take the board's height. | The stated floor is 1366×768 (style.css, web/README.md); a real 1366×768 window measured board 248 px, no page scroll. |
| W11 | view | The shared discard piles print insight prices discounted by seat 0's Science tags. | Hover text on a discard pile; the price at play time is right. |
| W12 | adversary | A station that flips keeps a marker, where a corrupted city's marker goes. | No rule says otherwise; war.md's sentence is about placing a marker. A rule to write if it ever matters. |
| W13 | viewer | A record overlaying adversary cards or AI boards shows the printed text. | Documented in experiments.md, *Content levers*; the game plays the overlay. |
| W15 | scripts | `scripts/cards/screen.py`'s "cap" column counts deck-out losses. | That is the round cap: adversary.md's word. |
| W16 | viewer | The sweep's `truncated` issues (thousands): threat and project text and adversary lines cut to fit, with a hover. | The no-scroll budget at 1366×768 (W10); every one has a hover or an ellipsis. |
| W17 | cards | "Liquidity Event is dominated by Bonded Warehouse's cash-out." | It cashes a pile without spending the Action, which can then add. |
| W18 | report | A one-agent run's findings say "skill gap +0 pts; the ladder is ordered". | True of a one-rung ladder. |
| W19 | ops | `copy_income` has no card; the agents' copy check is off. | Fix with its card. |
| W21 | viewer | A card's hover shows its id and pile key. | A developer aid, as W9. |
| W22 | sim, viewer | A run keeps every game's final state; the kept unloadable session is one slot. | Bounded; the newest loss is the one. |
| W23 | config | `stage_cards` reads 0..20; more than the content holds is refused. | The refusal says so. |
| W24 | measures | Unspent read before threat refunds; `insight_lost` tallies the debt; `Transfer` vetoes solo (no card). | Fix with the read that needs it. |
| W25 | report | "adversary ops/round" counts a game's last, partial round; "Agents (weakest first)" prints the spec's aliases. | Small (1 game in 48); the tables name agents by label. |
| W26 | viewer | Two stack counts overlap on a hex holding 15+ troops (the sweep's `board-text-overlap`, random play at 95 troops). | Random play only; agents' games never stack so high. |
| W20 | content | The place-token loop guard sees only a direct `add_tokens`. | No deck card prints such a trigger. |

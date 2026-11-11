# QA findings

What a bug hunt reads first: the findings still open, the reports ruled won't-fix (never re-report one), and the
faults past hunts kept finding. Fixed findings, never to re-report: F1–F100 (020, 044) in `git show
1617d75:docs/qa-findings.md`; then each hunt's `fixed.md` and `deferred.md` under `experiments/results/`: 099 (r38,
F101 on), 118 (r46, F132, D43, W19 on), 145 (r59, F165, D57 on), 169 (r72 → 73, F205, D72 on), 184 (r77 → 78, F225,
D77, W28 on). A new finding takes the next number. Each is **fixed** (the change in git), **deferred** to Matt (options, a pick) or **won't fix** (why).

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
- `runaway_read.py` (loops in counters, 118), `counter_names.py` (keys no game writes, 145); `docs --check` on a rules
  quote a card no longer prints (118) and on a stale house word in what players read, the rulebook too (169, 184).
- The sweep's moments by revision (`COUNTER_EVENTS_*`: r35, r41, r55, r70, r74) and `stale-word` on screen (169, 184).

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
- **A new verb against old guards** (169): Push skipped Wall, `per_site` restarted "one per country in turn" each
  site, a card's own play missed its own trigger. Check each new op against every rule about arrivals and order.
- **A rename by one phrase** (169): replacing "adversary ambassador" left the bare word in labels, faces and prose.
- **A rules item's other surfaces** (184): r74-r77 changed the code and the rules, and left icons, viewer labels,
  prompts, pricing notes, the card viewer's export, e2e lanes and read scripts on the old rule. List every surface.

## Open, deferred to Matt

Closed by 121 and 124 (git has them): D16, D43, D44, D47, D48, D51, D53, D54; by 152 (both scenes): D69; by 178 (r77 #27): D76.

| # | Area | Finding | Options, and the pick |
|---|---|---|---|
| D2 | variants | The off-variants don't measure what a module adds (the rules say so). | Leave; a stand-in; retire. **Open:** a stand-in for alignment if Matt measures with it. |
| D6 | agents | After a mid-step reshuffle of a drawn-out deck, the pending-choice guess keeps the true RNG for every stream, not just the reshuffled one (`determinize.rs`, `drawn_out`). Rare. | (a) Map pile to RNG purpose; (b) leave it. **Pick (b)** until a verdict hinges on late Alignment draws. |
| D32 | rules | Any lost Follower (Sabotage, Purge Protocols) leaves Popular Uprising short of 4 for good unless Manifesto Leak refills it, as the rules say (the Hostile Buyout half went with `free_leaves`, 178). | A design note. **Matt's call**. |
| D41 | rules | Aligning a dark city cancels one agent (r40), but Remove costs the same ambassador and clears up to 3: no agent took the cancel (102). | Leave; cancel and align; price Remove up. **Matt's call**; leave. |
| D42 | cards | Data Licensing Deal is unplayed: the agents spend every Dataset pile at once, so its 4-Dataset pile never stands (102). | A pile that holds; cash every pile; leave. **Matt's call**. |
| D21 | picks | `picks/card_frames_r2.json` and `scene_style_r1.json` point at images that are gone: opening those pickers shows broken tiles. Superseded rounds. | (a) Delete the two manifests; (b) leave them as a record. **Pick (b)** unless they confuse: their rulings are in the frames and the preamble. |
| D45 | engine | Fight triggers read the board's net count: a removal plus a placement in one action fires nothing. | Count at the removal ops. **Pick** when common. |
| D46 | loops | Drain with Security Ops Floor and Intrusion Lab: ~12 for 10 credits, while projects hold compute. | Leave it: compute caps it. **Pick**. |
| D49 | scripts | `card_ranks.py`'s keep rate counts opening keeps (over 100%); speed.md's bar sits on it. | A draft-keep counter, re-measure. **Pick**. |
| D52 | viewer | The timeline keeps a state a step (~200 MB a 4-country game; a blocking reload). | Keep every Nth snapshot. **Pick** if needed. |
| D55 | rules | At 1 country no military base: base arrivals, Escalation, Total War's base ambassadors place nothing. | **Matt's call**. |
| D56 | content | `Content::check` lets through ops that never run: a threshold or `here` op on an AI board, the Science niche or `gain_per_tokens` in a trigger's ops. | Widen the checks (F196 did two). **Pick**. |
| D50 | viewer | A new player's notes from two real-speed sessions: [viewer-notes.md](../experiments/results/118_bug_hunt_r46/viewer-notes.md). | Picks there. |
| D57 | cards | Exascale Training Cluster kept in 59-72% of games, played in 1-3% (r62): the keep plan wants it, the play eval doesn't. | One pricing for keep and play (an agents item), then more power **Matt's call** (148, 150). |
| D58 | speed | Greedy grew 1.63× a decision at r59, 3.0× at r61: at r68 86-91% of its time answers threats' loss prompts (`score_choose`, 157). | A cheaper settle (moves greedy's play). **Pick**. |
| D66 | docs | principles.md's credits a game and card-pricing's cost range unmeasured at r60. | Re-read, restate. **Pick**. |
| D72 | agents | Push heuristics count agents across a border and drop firewalled sites; the liberate gather takes an ambassador off a neighbor's own bill. | The fix is in 169's `agent_moves_fix.diff`; its smoke moved greedy d2 -3.1 (amendment 7657's bar ±2). **Pick:** the sim players item. |
| D73 | cards | Network Cleanup III (2 markers a hex) is II: a hex never holds 2. | III widens where (`near_player`). **Matt's call**. |
| D74 | agents | The strategist keeps 43 cards it never plays (Open-Weights Release 71%, Microgrid Lab Compact 69%; r77, 16 seeds); the engine agent 18. | With D57: one pricing for keep and play. **Pick**. |
| D75 | viewer | Agents' d3 games stack 18-29 troops; a two-digit count sits on the next hex's number at 1366×768 (W26's premise gone). | Draw 10+ smaller or on a pill. **Pick**. |
| D77 | rules | A project flip (Station Coup, Lab Takeover, Station Seizure) or Overrun corrupts a firewalled or prohibited site; the words now say so. | Guard all three, a bump. **Matt's call** (question 1). |
| D78 | rules | War Economy's troop lands on the factory under its token, not the one it builds. | `at: new_factory`, a bump. **Matt's call**. |
| D79 | rules | The adversary-factory fallback lets players send agents to a firewalled factory: nothing lands. | Skip firewalled factories. **Matt's call** (the printed rule allows it). |
| D80 | words | Great Liberation "frees" cities it liberates; the rulebook never defines "free". | "Liberate or free", or a rulebook line. **Matt's call**. |
| D87 | viewer | Five small faults from sweep 2's sheets (184's deferred.md). | One viewer item. **Pick**. |
| D86 | rules | A forced threat pull (0.1 a game) has no end in the rulebook's draw (§18): it reshuffles forever. | One rulebook sentence: take the next card. **Matt's call**. |
| D82 | cards | Five cards print under 7.5 pt with no recorded exception (Rally the Allies 5.4 pt). | Cut words or record. **Matt's call**. |
| D83 | art | Ten tier-2 cards (158) have no drawn scene: `cards:check` fails on main. | An art batch. **Pick**. |
| D84 | boards | Focus (Builder) and Recovery (Protector) are never used (r77, 96 games). | With the board redesign. **Matt's call**. |
| D85 | viewer | Prompts: "which other player?" names no card; "Corrupt factories (at 2): ... onto which?" (agents parse it); "1 seat 1's ambassador" (a record key). | Prompts and the agents' parser together. **Pick**. |

## Won't fix

| # | Area | Report | Why not |
|---|---|---|---|
| W2 | alignment | "The last-drawn Alignment card is shuffled back before it resolves." | Not a bug: a drawn card goes on its resolved pile first. |
| W3 | view | "`PhaseView.waiting` is stale during a choice in an automatic phase." | Not reached in 62 seeds; the viewer reads who decides from the choice. |
| W5 | viewer | The event log shows engine events raw (`var changed · key adversary.supply · delta +1`). | The viewer is debug-first (web/README.md). |
| W6 | viewer | With the *View* tab open a 4-country step costs up to ~140 ms. | A debug tab, closed by default. |
| W7 | sim | In counter rows "-" means "never tallied" and "no game reached this round". | Reading aids; `games.jsonl` has the detail. |
| W9 | viewer | A grouped hex-pick button's hover shows the raw JSON of its first action only. | A developer aid. |
| W10 | viewer | Below 1366×768 (1024×768, 150% zoom) the board pane has no scroll fallback. | 1366×768 is the stated floor (web/README.md). |
| W11 | view | The shared discard piles print insight prices discounted by seat 0's Science tags. | Hover text on a discard pile; the price at play time is right. |
| W12 | adversary | A station that flips keeps a marker, where a corrupted city's marker goes. | No rule says otherwise; war.md's sentence is about placing a marker. A rule to write if it ever matters. |
| W13 | viewer | A record overlaying adversary cards or AI boards shows the printed text. | Documented in experiments.md, *Content levers*; the game plays the overlay. |
| W15 | scripts | `scripts/cards/screen.py`'s "cap" column counts deck-out losses. | That is the round cap: adversary.md's word. |
| W16 | viewer | The sweep's `truncated` issues (thousands): threat and project text and adversary lines cut to fit, with a hover. | The no-scroll budget (W10); each has a hover or an ellipsis. |
| W17 | cards | "Liquidity Event is dominated by Bonded Warehouse's cash-out." | It cashes a pile without spending the Action, which can then add. |
| W18 | report | A one-agent run's findings say "skill gap +0 pts; the ladder is ordered". | True of a one-rung ladder. |
| W19 | ops | `copy_income` has no card; the agents' copy check is off. | Fix with its card. |
| W21 | viewer | A card's hover shows its id and pile key. | A developer aid, as W9. |
| W22 | sim, viewer | A run keeps every game's final state; the kept unloadable session is one slot. | Bounded; the newest loss is the one. |
| W23 | config | `stage_cards` reads 0..20; more than the content holds is refused. | The refusal says so. |
| W24 | measures | Unspent read before threat refunds; `insight_lost` tallies the debt; `Transfer` vetoes solo (no card). | Fix with the read that needs it. |
| W25 | report | "adversary ops/round" counts a game's last, partial round; "Agents (weakest first)" prints the spec's aliases. | Small (1 game in 48); the tables name agents by label. |
| W20 | content | The place-token loop guard sees only a direct `add_tokens`. | No deck card prints such a trigger. |

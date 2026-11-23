# QA findings

What a bug hunt reads first: the findings still open, the reports ruled won't-fix (never re-report one), and the
faults past hunts kept finding. Fixed findings, never to re-report: F1–F100 (020, 044) in `git show
1617d75:docs/qa-findings.md`; then each hunt's `fixed.md` and `deferred.md` under `experiments/results/`: 099 (r38,
F101 on), 118 (r46, F132, D43, W19 on), 145 (r59, F165, D57 on), 169 (r72 → 73, F205, D72 on), 184 (r77 → 78, F225,
D77, W28 on), 197 (r86, F272, D88, W29 on), 235 (r106, F289, D92, W31 on; 238 fixed D92-D96, D98, D100, D103-D105, r108), 261 (r121 → r122, F322, D106, W34 on; 265 fixed D108, D109, D112, D115, D116; 266 D117), 285 (r135 → r136, F379,
D120 on; fixed D102; 286 closed D121, D122, D124, D125, D126 (restated), D128, D130, D131, D132 at r137; 288 fixed D106 (built in) and D120 at r138; 289 closed D123, D127, D129, D133 (the troops prompt's words to D85); 305 closed D85, D87, D135 and D50's picks, D52 to won't fix; 307 closed D84 (Focus cut, 206) and D134 (Spill and Grow gone, r140), and dropped W17 and W33 (their cards gone)), 308 (r149 → r150, F424, D136 on; 312 built D136, D137, D139 and D140 at r151 (320 put D139's Escalate notice on the engine's `escalate_ahead`), and D138 is left by Matt's ruling). A new finding takes the next number. Each is **fixed** (the change in git), **deferred** to Matt (options, a pick) or **won't fix** (why).

## Checkers

`web/scripts/sweep.mjs` (web/README.md *The sweep*), `legal-probe.mjs` (`npm run probe`: kept, never played),
`odd.mjs` (`npm run odd`), `crates/aic-core/src/tests/invariants.rs`, `scripts/wasm-parity.mjs` (native = wasm) and
the e2e lanes. What they already catch, so a hunt needn't hand-check it:

- The sweep: what fails the run and what it reports for a look (raw ids, clipping, spikes, `no-control`,
  `mark-over-number`, `warn-at-start`, `stale-word`, the moments by revision) are in web/README.md, *The sweep*.
- `invariants.rs` fails on a raw token key in any label, `no_action_is_paid_for_nothing` on an action that costs
  something and changes nothing, and a log line saying a node was destroyed while one stands there.
- The adversary's twist loop names where each twist op is read; an unread one panics.
- `runaway_read.py` (loops in counters), `counter_names.py` (keys no game writes, and the sweep's moments no key
  matches); `docs --check` on a rules quote a card no longer prints and on a stale house word in what players read,
  the rulebook too. The fast lane's `node_scripts_parse` runs `node --check` on every Node script and module.

## Where hunts found faults

Each hunt found most of its faults in the same places. Look there first:

- **Options with nothing to act on**: a move onto the hex the ambassador already holds, a removal on a hex holding only
  what it can't remove, a cost knob at 0 making a free endless action; a target hex where the card's op does nothing.
  The invariants test catches what a seed reaches; `feasible.rs` and `targets.rs` hide the rest.
- **Words that contradict the code**: hand-written faces, the rules and rulebook (whole sections revisions behind),
  architecture.md and README. Check each against what runs.
- **A log line ahead of its act**: a note written before a placement or protection decides ("lands", "destroy"), a
  count with no plural. Read each note against what can stop it.
- **An inverted warning**: a countdown or allotment drawn as danger while full.
- **Raw names in what players read**: ids, token keys, hex and country indices in prompts, notes, traces and labels.
- **Printed cost against charge**: a button or prompt showing the printed base where a discount or surcharge applies.
- **Report statistics**: a per-round mean over games that never reached the round; a label that hides its average.
- **The hunt's own fixes**: every independent review of a hunt's diff found five to eight faults in it. Get one.
- **Untestable here**: the scene tools' kill of a hung draw on Windows is not exercised (it needs a real hang).
- **An offer judged before its price**: random games rarely reach it: write the state.
- **Measures gone quiet**: a definition matching nothing after a rules change; an empty column reads as a measure.
- **Kept, never played**: keep plan and play check price a card differently. `play_read.py`, then `npm run probe`.
- **Who an effect belongs to**: a trigger inside a teammate's action, a counter fed by another card.
- **Agents matching words**: a prompt's rewording turned an agent heuristic off; grep the agents for a prompt.
- **Order inside a round**: a Discord tip in the Sway phase read last round's card; a project acted in the build that
  filled it; a troop rule spent an ambassador where no troop came. Ask what has happened yet at each phase.
- **One op, two wordings**: faces of the same op printed differently (face down, standing troops, pay or discard),
  and a face's "here" with two tokens. Group faces by op and read them side by side.
- **A new verb against old guards**: Push skipped Wall. Check each new op against every rule about arrivals.
- **A rename by one phrase**: replacing "adversary ambassador" left the bare word in labels, faces and prose.
- **A rules item's other surfaces**: one changed the code and the rules, and left icons, viewer labels, prompts,
  pricing notes, the card viewer's export, e2e lanes and read scripts on the old rule. List every surface.
- **261's**: a rider read after the step it rides on (Peer Review's paid for its own free station); a lane that
  never loaded (e2e:researcher, a SyntaxError for eight items); an agent fix that weakened the judge (read paired).
- **308's**: a reach rule (a troop by your ambassador) honored by the target but dropped by the op's own hex list (Push,
  Kettle); "here" falling back to another location's token; a view flag read inside the beat it forecasts.
- **285's**: a country guard checked where a line starts, not where it lands (hex neighbors cross borders); a note
  read after an arrival check emptied the hex; a check's fixture copy missing a file a later item reads.

## Open, deferred to Matt

| # | Area | Finding | Options, and the pick |
|---|---|---|---|
| D2 | variants | The off-variants don't measure what a module adds (the rules say so). | Leave; a stand-in; retire. **Open:** a stand-in for alignment if Matt measures with it. |
| D6 | agents | After a mid-step reshuffle the pending-choice guess keeps the true RNG for every stream (`determinize.rs`). Rare. | Map pile to RNG purpose; leave. **Pick** leave. |
| D32 | rules | A lost Follower leaves Popular Uprising short of 4 for good unless Manifesto Leak refills it. | A design note. **Matt's call**. |
| D41 | rules | Aligning a dark city cancels one agent; no agent takes it over Remove. | Leave; price Remove up. **Matt's call**. |
| D42 | cards | Data Licensing Deal is unplayed: the agents spend every Dataset pile at once, so its 4-Dataset pile never stands. | A pile that holds; cash every pile; leave. **Matt's call**. |
| D21 | picks | `picks/card_frames_r2.json`, `scene_style_r1.json` point at gone images (superseded rounds). | Delete; keep as a record. **Pick** keep. |
| D45 | engine | Fight triggers read the board's net count: a removal plus a placement in one action fires nothing. | Count at the removal ops. **Pick** when common. |
| D46 | loops | Drain with Security Ops Floor and Intrusion Lab: ~12 for 10 credits. | Leave: compute caps it. **Pick**. |
| D49 | scripts | `card_ranks.py`'s keep rate counts opening keeps (over 100%). | A draft-keep counter. **Pick**. |
| D56 | content | `Content::check` lets through ops that never run (a `here` op on an AI board, `gain_per_tokens` in a trigger). | Widen the checks. **Pick**. |
| D50 | viewer | A new player's notes from two real-speed sessions: [viewer-notes.md](../experiments/results/118_bug_hunt_r46/viewer-notes.md); 305 did its picks. | Its three Matt's calls stand: one income figure or a hover on each; the 1-country draft; a wider 4-country strip or a tab. **Matt's call**. |
| D57 | cards | Exascale Training Cluster kept in 59-72% of games, played in 1-3% (r62). | One pricing for keep and play (an agents item). **Matt's call**. |
| D58 | speed | At r68 86-91% of greedy's time answers threats' loss prompts (`score_choose`). | A cheaper settle (moves play). **Pick**. |
| D66 | docs | principles.md's credits a game and card-pricing's cost range unmeasured at r60. | Re-read, restate. **Pick**. |
| D72 | agents | Push heuristics count agents across a border and drop firewalled sites; the liberate gather takes an ambassador off a neighbor's bill. | 169's `agent_moves_fix.diff` (greedy d2 -3.1). **Pick:** a sim players item. |
| D74 | agents | The strategist keeps 43 cards it never plays (r77); the engine agent 18. | With D57. **Pick**. |
| D75 | viewer | A two-digit troop count sits on the next hex's number at 1366×768 (d3 stacks 18-29). | Draw 10+ smaller. **Pick**. |
| D77 | rules | A project flip or Overrun corrupts a firewalled or prohibited site; the words say so. | Guard them, a bump. **Matt's call**. |
| D78 | rules | War Economy's troop lands on the factory under its token, not the one it builds. | `at: new_factory`, a bump. **Matt's call**. |
| D79 | rules | The adversary-factory fallback lets players send agents to a firewalled factory: nothing lands. | Skip firewalled factories. **Matt's call** (the printed rule allows it). |
| D80 | words | Great Liberation "frees" cities it liberates; the rulebook never defines "free". | "Liberate or free", or a rulebook line. **Matt's call**. |
| D82 | cards | Five cards print under 7.5 pt with no recorded exception (Rally the Allies 5.4 pt). | Cut words or record. **Matt's call**. |
| D83 | art | 12 deck cards name no scene and 6 scenes aren't served: `cards:check` fails. | An art batch or a pool scene. **Pick**. |
| D107 | rules | Nothing marks a city bought this round. | A Bought marker. **Matt's call**. |
| D113 | cards | Counterintelligence Sweep, Fusion Commons draw at 0 insight. | 1 insight; no draw. **Matt's call**. |
| D114 | cards | Five liberate cards' pricing notes use pre-r113 keeps. | Restate. **Pick**. |
| D118 | rules | The Liberated marker binds Corrupt lines only, not cards that take a site. | Leave; guard. **Matt's call**. |
| D119 | cards | Faces against house terms: node "Harden", Faraday Yard's box, two-way wordings. | One card-words item. **Pick**. |
| D141 | agents | The push and strike heuristics count agents on a hex reached only through an ambassador (troops only there, F424). | Mirror `pushable` in `agents/moves.rs`. **Pick:** an agents item. |

## Won't fix

| # | Area | Report | Why not |
|---|---|---|---|
| W2 | alignment | "The last-drawn Alignment card is shuffled back before it resolves." | It goes on its resolved pile first. |
| W3 | view | "`PhaseView.waiting` is stale during a choice in an automatic phase." | The viewer reads who decides from the choice. |
| W5 | viewer | The event log shows engine events raw (`var changed · ...`). | The viewer is debug-first. |
| W6 | viewer | With the *View* tab open a 4-country step costs ~140 ms. | A debug tab, closed by default. |
| W7 | sim | In counter rows "-" means "never tallied" and "no game reached this round". | Reading aids; `games.jsonl` has the detail. |
| W9 | viewer | A grouped hex-pick button's hover shows the raw JSON of its first action only. | A developer aid. |
| W10 | viewer | Below 1366×768 (1024×768, 150% zoom) the board pane has no scroll fallback. | 1366×768 is the stated floor (web/README.md). |
| W11 | view | The shared discard piles print insight prices discounted by seat 0's Science tags. | Hover text on a discard pile; the price at play time is right. |
| W13 | viewer | A record overlaying content shows the printed text. | experiments.md, *Content levers*; play is the overlay's. |
| W15 | scripts | `screen.py`'s "cap" column counts deck-out losses. | The round cap: adversary.md's word. |
| W16 | viewer | The sweep's `truncated` (thousands): threat, project and adversary text cut to fit. | The no-scroll budget (W10); each has a hover. |
| W18 | report | A one-agent run says "the ladder is ordered". | True of a one-rung ladder. |
| W19 | ops | `copy_income` has no card; the agents' copy check is off. | Fix with its card. |
| W21 | viewer | A card's hover shows its id and pile key. | A developer aid, as W9. |
| W22 | sim, viewer | A run keeps every game's final state; the kept unloadable session is one slot. | Bounded. |
| W23 | config | `stage_cards` over what the content holds is refused. | The refusal says so. |
| W24 | measures | Unspent read before threat refunds. | Fix with the read that needs it. |
| W25 | report | "adversary ops/round" counts a game's last, partial round. | Small (1 game in 48). |
| W20 | content | The place-token loop guard sees only a direct `add_tokens`. | No deck card prints such a trigger. |
| W31 | cards | Peacekeeper Rotation's move onto a Troops target with no piece does nothing. | Not in the deck: fix with the card. |
| W34 | scripts | `screen.py`'s 041 preset reads the r98-retired marker counters. | A historical preset, kept as 041's record. |
| W35 | viewer | (D52) The timeline keeps a state a step. | 305 measured the heap after a whole r148 game in the player view: 14 MB (2 seats, 306 actions), 22 MB (3 seats, 461), 29 MB (4 countries, 624; ~200 MB was an older build's). |

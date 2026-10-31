# History

What the README used to carry that is history rather than how to build, run and test
today: past sweeps and their recipes, timings measured on rules that have since
changed, and the specs the binary no longer accepts. Moved here in 016, otherwise
as written. Every number is as measured at the time, on the rules revision named; spec
paths are as they were then (the refused specs now live in [`archive/`](archive/)).

## Tuning run (007) and engine agent (008)

The 007 run's log, change log and proposals: `experiments/tuning-007.md`. The 008
engine agent, its sweep and the building-engine verdict: `experiments/engine-008.md`;
rerun `experiments/archive/engine-agent-008.json` (~28 min alone on 12 threads, 2,400 games:
greedy-search agents take 6–16 s a game each with every core busy) to judge a payback
lever.

## Payback sweep (009)

Write-up `experiments/payback-009.md`. Config levers are plain specs:
`experiments/payback-009-singles.json` (13 variants × d0, d4 × greedy, engine × 96
seeds = 4,992 games, **46 min** alone on 12 threads) and
`experiments/payback-009-combos.json` (both refused today, archived). Levers that config
can't reach (card costs and income, tier-2 costs, the AI tracks) run as **content
patches** built into separate binaries from a copy of the repo under `out/`. Committed
content is never touched. The scripts are r3's, and 028 removed `scripts/payback/` (last at
`8624980^`); the content-lever specs are in `archive/payback-009-content/`:

```sh
python scripts/payback/patch_content.py --list                      # the patches
bash scripts/payback/build_patches.sh insight_inc econ_half+insight_inc   # -> out/patch-bin/<patch>.exe, ~1 min each
bash scripts/payback/run_content.sh insight_inc insight_inc:insight_inc+keep0  # <binary>:<spec> in experiments/archive/payback-009-content/
python scripts/payback/summarize.py out/payback-009-singles out/payback-009-content/*/ --picks 20
```

`scripts/payback/resources.py` (unspent resources by round from a `--trace` game) was
retired in 015 (it keyed on the phase `adversary.ravage`, removed in 014) and deleted in
016, as was `scripts/tune/trace_summary.py` (a round-by-round summary of one `--trace`
game, reading pre-014 token and track names).

One content experiment (384 games) takes about 5 minutes alone. Records from a patched
binary carry its content hash, so they replay only on a binary built from the same
patch (main's `aic-sim replay` fails on them, typically "unaffordable"). Running two or
three experiments at once gives the same total throughput (about 1.3 games/s for greedy
+ engine), and even `tail` on a log can take 30+ s under that load.

## Proposals (011)

Write-up `experiments/proposals-011.md`: 007's accepted proposals as variants and new
defaults (rules r4). The timings below are r4's, and `proposals-011.json` and two of the
screens are refused today. Screens (no probes, greedy):
`experiments/screen-011-difficulty*.json` (512–1,024 games, 9–16 min each with another
run beside it) chose the new level 1 and 3 effects; `screen-011-clock*.json` ran on
009's `t2_half+t2_noam` patch binary (`bash scripts/payback/build_patches.sh
t2_half+t2_noam`, then `out/patch-bin/t2_half+t2_noam.exe experiment ...`; 192–480
greedy + engine games, 7–12 min). The measurement, `experiments/proposals-011.json`
(10 variants × d0–d4 × greedy, engine × 64 seeds = 6,400 games, no probes), was
stopped unfinished: 2,109 games in 28 minutes alone on 12 threads (about 1.25 greedy +
engine games a second), so about 85 minutes for the whole grid. Since 012 the whole
grid ran in **10.6 minutes** (633 s, about 10 games a second; report 0.3 s; measured
before the last heuristic fixes, so somewhat less after), and a stopped run keeps
its games. `proposals-011 --quick` was 1,600 greedy + engine games, about 3 minutes (r4).

## Specs the binary refuses (archived in 016)

The rules or configs they name are gone; kept as history until rewritten, in
[`archive/`](archive/): `tune-before` (`configs/v0_untuned.json` is missing),
`variants-spread` (the `clusters` and `assets` configs), `proposals-011` and
`variants-modules` (the removed `opposition` module), `screen-011-difficulty` and
`screen-011-difficulty2` (`adversary.foundry_bonus`), `payback-009-singles`,
`payback-009-combos`, `payback-009-content/insight_inc+fc4.json` and
`payback-009-content/econ_half+insight_inc+d4tune.json` (removed parameters), and
`tune-agents` (removed agent weights). Committed results from r5 and earlier are
history: their game records do not replay.

## Speed (012)

Measured on r4, before 013 and 014 changed the game. A 400-game slice of
`proposals-011` (`--games 4`: every variant, difficulty and agent) took **263.8 s**
before 012 and **34.5 s** after, alone on 12 threads, with identical games (`python
scripts/bench/same_results.py DIR_A DIR_B` compares two output directories, timing
aside). Greedy then took about 0.6 s a game and the engine agent about 1.3 s, with every
core busy. Where the time went, and what fixed it (times for a 20-game slice):

- The agents are 99% of a run; rules apply, legal actions, the replay check and setup
  are each under 1%, and writing the report well under a second.
- Most of the agents' time was the system allocator: search clones game states by the
  hundred thousand, and on 12 threads Windows' heap serialized them (a state clone took
  134 µs on 12 threads, 24 µs on one). `aic-sim` uses **mimalloc** (19.6 s → 4.8 s on a
  20-game slice).
- A state shares its config, and a pending choice its snapshot, through `Arc` (they
  serialize as before), and searches call `Rules::apply_to`, which copies the state once
  where clone + `apply_mut` copied it twice (the action's snapshot): 4.8 s → 2.7 s.
- The heuristic (the engine agent's horizon mode most): the economy's production built
  token keys per hex and looked at the adversary's reach everywhere; each building in
  hand worked the seat's building discount out again; cards were found by id with a
  linear search (now an index in `Content`); the defense values and unpaid threats were
  each worked out twice per position. 2.7 s → 2.3 s, the heuristic 38 → 22 µs a call.
- Left: the heuristic (~50% of search time, ~22 µs a call) and apply with its state copy
  (~40%, ~17 µs). A state is ~200 small allocations (string keys and card ids); interning
  them is the next big step.

## Timing before 012 (v0 baseline)

These predate 012's speed-up (about 7.6× faster on 12 threads, 5× on one: 10 games of
`proposals-011` on one thread took 29.0 s, then 5.8 s).

v0 baseline, 2 countries, release build, this 12-thread machine; games last 3–5 rounds,
about 85–120 decisions. One game on one thread:

| Agent | ms/game | ms/decision |
|---|---|---|
| `random` | 1.4 | 0.02 |
| `greedy` | ~900 | ~7.5 |
| `beam` | ~1,100 | ~10 |
| `rollout` | ~2,500 | ~21 |

With all 12 threads busy each game takes 1.5–3× as long (shared cores), so an
experiment's throughput is about 4–5 greedy or 3–4 beam games a second. Probes add to
that: win probability costs `rollouts` playouts per round of each probed game (8
greedy playouts ≈ 8 × 0.5 s per round, the bulk of a probed game's time); closeness
costs about as much as a greedy decision per probed decision. Measured:
`experiments/ladder-v0.json` (600 games: 2 difficulties × random, greedy, beam × 100
seeds, 10 probed seeds per cell) took **3.7–4.3 minutes** (two runs); `experiments/smoke.json` (36
games) 30 s.

## Timing on the tuned baseline (007)

Games lasted 7–8 rounds, about 200 decisions; 12 threads busy, so per-game times are
under load:

| Agent | s/game under load | Throughput (12 threads) |
|---|---|---|
| `random` | 0.003 | ~300 games/s |
| `greedy` | 4–9 | ~1.5–2 games/s |
| `beam` | 45–60 | ~0.2–0.25 games/s |
| `rollout` | 60–80 | ~0.15–0.2 games/s |

- `experiments/archive/tune-after.json` (2,000 games: random and greedy × d0–d4 × 200, 10 seeds
  per cell probed for win probability): **45 minutes**. The probes are most of it;
  without them it is about 12 minutes.
- `experiments/archive/tune-after-search.json` (greedy, beam, rollout × d0, d4 × 24): 8 minutes.
- `experiments/variants-modules.json` (3,072 games, no probes): 11 minutes.
- `experiments/payback-009-singles.json` (4,992 greedy + engine games): 46 minutes;
  `payback-009-combos.json` (1,152): about 20 minutes alone. A content-patch spec (384
  games) takes 5–6 minutes alone. The engine agent takes 7–25 s a game under load,
  about twice greedy.
- `scripts/tune/sweep.py`: one `--set` point of 64 greedy games takes about 45–60 s.

## Slow test lane on 014

`cargo test -p aic-sim` with `--include-ignored` took 78–122 s on 014 (the whole-game
`rollout` and `beam` tests); 015's fix to the beam's plan check brought it to about
25 s.

## Past screens (013-027)

How each screen was run, moved from the README in 048, as written. The scripts named
read the counters of their revision; the README's *Scripts* says which still run.

**Alignment screen (013)**, `experiments/archive/screen-013-alignment.json` (base, `no_am` and
`off` × d0, d4 × greedy, engine × 32 seeds = 384 games): **45 s** alone on 12 threads
with `--quick --games 32`. Committed with the report in
`experiments/results/screen-013-alignment/` (`alignment.md`). Tips by round come from
the counters `alignment.tip_<side>.rNN`, so `aic-sim run` shows them too.

**Board screen (014)**, `experiments/archive/screen-014-board.json` (`c2` and `c3`, that is 2
and 3 countries, × d0, d4 × greedy, engine): `aic-sim experiment
experiments/archive/screen-014-board.json --quick` (16 seeds a cell, 128 games). Committed with
the report and records in
`experiments/results/screen-014-board/` (`board.md`).

**Endgame screen and gate (027, r16)**: `experiments/archive/screen-027-cards.json` and
`experiments/archive/screen-027-dominance.json` (024's checks plus `ignore_threats`, `turtle`,
`free_only`), run with `--quick` (the gate `--games 32`), then `python
scripts/cards/screen_027.py DIR` and `--gate DIR`; `screen-027-tune.json` is the tuning probe
(engine, d0 and d2, ~3 min on 12 threads). Results: `experiments/results/screen-027-endgame/`.

**Scarier-adversary screen and dominance gate (024, r15)**: `experiments/archive/screen-024-cards.json`
and `experiments/archive/screen-024-dominance.json` (023's two specs at r15; same cells), run with the
same flags, then `python scripts/cards/screen_024.py out/screen-024-cards
out/screen-024-dominance --before DIR` (DIR: the same cards screen on the r14 binary). **~3 min** the cards screen and **~12 min** the gate, side by side on 12 threads.
`experiments/archive/tune-024.json` is the small probe (greedy and engine, d0 and d2).
Agent changes in 024: the keep plan (`agents/policy.rs`) charges credits paid for keeps
(`plan_credit` 3; the opening's 5-credit keeps took all 20 starting credits: this one
change took the engine at c2 d2 from 37% to 81%), skips cards it can't play now (a built-over
building with nothing to go over, a card with no target) and holds back `plan_reserve` (0; 034: was 2, with `plan_rounds` 4, was 3, which fixed the under-keeping)
insight in later rounds; `plan_hand` and `plan_t2` exist for probes and are off. In
`agents/eval.rs`: `w.project` 30 (projects can corrupt cities now), credits beyond
`w.credit_cap` (12) a seat are worth `w.credit_idle` (0.15), `w.guard` for an adversary
ambassador next to a corrupted city. A results dir's `report.md` exists from the start of a
run: wait on the process or the log's per-cell lines, not on the file. Committed in
`experiments/results/screen-024-adversary/`.

**Card-set screen and dominance gate (023, r14)**: two specs.
`experiments/archive/screen-023-cards.json` (`c2`, `c3` × d0–d4 × random, greedy, engine; 480
games) and `experiments/archive/screen-023-dominance.json` (d2 only, `c2`, `c3` × the engine,
022's seven checks and 023's mono-tag ×6 (`only_tag=1..6`), ignore-X ×6
(`never_tag=1..6`) and event-hoarder (`events_only=1`); 1,344 games): `aic-sim experiment
experiments/archive/screen-023-cards.json --quick --out out/screen-023-cards` (**~3 min** alone,
~5 min beside the gate) and `aic-sim experiment experiments/archive/screen-023-dominance.json
--quick --games 32 --out out/screen-023-dominance` (**~12 min** on 12 threads, most of it
3-country engine games at 5–15 s each), then `python scripts/cards/screen_023.py
out/screen-023-cards out/screen-023-dominance`: win rates, the strategy-health gate
(021's, plus engine at d0 ≥ 85%), the card economy and by-round tables, the tags in the
engine's wins, per-card keep and play rates with the dead cards and must-picks named,
and the dominance table with each gap's standard error. `experiments/archive/tune-023.json` is
the small probe used for tuning (engine at d0 and d2; ~1.5 min). `python
scripts/cards/cards_doc.py` regenerates [docs/cards.md](docs/cards.md) and prints the
set's shape against 023's targets. Committed in `experiments/results/screen-023-cards/`
(the write-up `cards.md`; `cards/`, `dominance/` and `first-cut/` hold the runs; `records/`
the seeds named for the viewer). Agent changes in 023 (`agents/eval.rs`): the draft
packet counts like drafted cards less its keep price (`w.packet_keep` 1), so a tier pick
is a choice (before, tier 2 was drafted only once the tier-1 deck ran dry: 52 cards ran
dry, 108 don't); a program in play with an ability earns `w.ability` a round like a
building. Counter new in 023: `cards.drawn`.

**Insight and dominance screen (022, r13)**: two specs. `experiments/archive/screen-022-cards.json`
(`c2`, `c3` × d0–d4 × random, greedy, engine; 480 games) and
`experiments/archive/screen-022-dominance.json` (`c2`, `c3` × d0, d2, d4 × the engine and the seven
dominance checks, `engine:keep_all=1` and so on; 768 games): `aic-sim experiment
<spec> --quick --out out/<name>` (**~6 min** the cards screen and **~8.5 min** the
dominance screen, run side by side on 12 threads; alone, about half that: engine games
take 5–11 s each at c3), then `python scripts/cards/screen_022.py out/screen-022-cards
out/screen-022-dominance`: win rates; the card economy per game (kept / played, insight
spent on keeps / plays, credits left unspent, presence); by round (hand size after
keeping, kept, played, insight on keeps and on plays, credits unspent; greedy and engine
at d0 and d2, c2); and the **dominance table**: each check's gap to the engine with one
standard error, and the gate (`docs/principles.md`, *Dominance checks*). Counters new in
022: `cards.insight_kept`, `cards.insight_played` (also per round), `threats.short`.
Committed in `experiments/results/screen-022-insight/`.

**Card and resource screen (021, r12)**, `experiments/archive/screen-021-cards.json` (`c2`, `c3` ×
d0–d4 × random, greedy, engine; 480 games): `aic-sim experiment
experiments/archive/screen-021-cards.json --quick --out out/screen-021` (**~3 min** wall on 12
threads: engine games take 3–11 s each at r12, about twice r11's, from the bigger hands
and the per-card tier picks), then `python scripts/cards/screen_021.py out/screen-021`:
per cell the win rate and ends; the card economy per game (kept, played, tier-2 dealt,
powers and tags sacrificed, buildings replaced, tokens, presence) with the gate (in the
script's docstring); and d0's by-round table (hand size after keeping, kept, played,
insight and credits left unspent at upkeep, powers). Counters new in 021:
`cards.replaced`, `.sacrificed`, `.tokens_gained`, `.tokens_spent`, `.dealt_tier1|2`,
`.insight_bought`, `ai.powers_used`, `ai.tags_sacrificed`, `board.city_energy_bought`,
`.chips_sold` (`.hardware_sold` before 026), `.presence_drained`, `effects.nodes_cut`, `threats.tokens_lost`; and
per-round `cards.hand_size`, `.kept`, `.played`, `.insight_after_keep`,
`economy.credits_unspent`, `.insight_unspent`, which `aic-sim experiment` writes into
`games.jsonl` as `<key>.rNN` (summed over seats; 021).

**Troop and node screen (019, r10)**, `experiments/archive/screen-019-troops.json` (`c2`, `c3` ×
d0–d4 × random, greedy, engine; 480 games): `aic-sim experiment
experiments/archive/screen-019-troops.json --quick --out out/screen-019` (**~2 min** wall on 12
threads with the tuned r10, 107–129 s; 211 s for r10 before tuning, whose games ran to
the deck's end). Counters new in 019: `board.troop_steps`, `.troop_moves`,
`.troop_strikes`, `.troop_destroyed.<presence|station|factory>`, `.troop_absorbed`,
`adversary.nodes_peak`, `alignment.purge_blocked_by_nodes` (017's `board.troop_advances`,
`.advance_strikes`, `.advance_destroyed` are gone, and 034 removed the reserve's
`.presence_from_reserve`, so the script no longer reads it).
Variant sweeps: an experiment file with several `variants`, each a list of `--set`
overrides (lists comma-separated, `alignment.unity_ease=1,1,2`), as in
`experiments/results/screen-019-troops/tuning.md`. Committed in
`experiments/results/screen-019-troops/` (`troops.md`, `before-tuning.md`, `tuning.md`).

**Calibration screen (018, r9)**, `experiments/archive/screen-018-calibration.json` (`c2`, `c3`
× d0–d4 × random, greedy, engine; 480 games): `aic-sim experiment
experiments/archive/screen-018-calibration.json --quick --out out/screen-018` (**~3 min** wall on
12 threads: r9 games run longer, 6–12 rounds, and c3 engine games go the full 11),
then `python scripts/cards/screen.py out/screen-018 --preset 018` (the columns whose
counters survive; 018's own script is gone): per cell the win rate, rounds, what was left on the board at the end (corrupted cities, nodes, troops: the
purge's blockers), tips and ends, then the strategy-health gate per game (presence,
factories per side and owned at the end, chips made and spent, nodes made and
removed, compute blocked by a player ambassador on a node, Security Patch, cities
freed, cards). `margin.nodes_end` / `troops_end` in `games.jsonl` are new in 018.
Committed in `experiments/results/screen-018-calibration/` (`calibration.md`).

**Projects screen (017, r8)**, `experiments/archive/screen-017-projects.json` (the same cells):
`aic-sim experiment experiments/archive/screen-017-projects.json --quick --out out/screen-017`
(128 games, **~8 s** wall on 12 threads: r8 games are short, ~4 rounds at d0). Committed in
`experiments/results/screen-017-projects/` (`projects.md`). Gotcha: on Windows a
Python script printing an en dash to a redirected stdout writes cp1252, not UTF-8;
`screen_017.py` reconfigures stdout to UTF-8.

**Presence screen (016, r7)**, `experiments/archive/screen-016-presence.json` (the same cells
as 014's): `aic-sim experiment experiments/archive/screen-016-presence.json --quick` (128 games,
**20 s** wall on 12 threads; d0 engine at 3 countries is the slowest cell, ~5 s a
game per worker), then `python scripts/cards/screen.py DIR --preset 016` (the columns
whose counters survive; 016's own script is gone): per cell the win rate, rounds, tips
and ends, then per game presence placed, nodes made and removed, factories built, destroyed and owned per side, chips made, bought and
spent. Committed in
`experiments/results/screen-016-presence/` (`presence.md`).

## Viewer notes (008-021)

Moved from the README in 048, as written. Run the shots scripts from `web/`.

- The board draws each adversary troop's next target (019, `View.arrows`): a dashed red
  arrow from the troop to the presence, station or factory it is heading for, solid when
  it strikes in the next march; hover for the text. `node scripts/shots-019.mjs` (with
  `AIC_URL` and `AIC_SHOTS=<dir>`, ~1–3 min: it often needs a few seeds) plays baseline
  seeds with Random actions and saves the board with a dashed arrow, then a solid one on
  a presence, then after the march that hits it (019's shots:
  `experiments/results/screen-019-troops/viewer/`). e2e (019, 1366x768 and up): every
  check passes. Gotcha: a backgrounded `npx vite preview` outlives its tool-runner
  wrapper when that is stopped at its time limit, so the port stays taken: stop the
  `node` process that holds it.
- `node scripts/shots-021.mjs` (with `AIC_URL` and `AIC_SHOTS=<dir>`, ~2-4 min: Random
  actions over a few seeds) saves the six 021 scenes: the opening hand of 8 to keep, a
  draft hand built card by card (tier picks), the draft after hands passed, a building
  stacked on another, a token card with 2+ tokens and a power being paid by sacrificing
  tags (021's shots: `experiments/results/screen-021-cards/viewer/`). Run it against
  `vite preview` (the dev server reloads mid-run when crates change).
- `node scripts/shots-017.mjs` (with `AIC_URL` and `AIC_SHOTS=<dir>`, ~30–90 s) plays
  baseline seeds and saves viewer shots of the adversary project row mid-fill and after
  a project resolved (017).
- e2e at 1366x768 (018): every check passes. `4 countries: no page scroll` has the board
  340 px tall against 307 needed (017: 260 to 281), `board fills its pane` 382 to 406
  against 346 (it varies with the game the run plays). Below 820 px of height
  (`@media (max-height: 820px)` in `web/src/style.css`) the strips around the board take
  tighter lines and the adversary cards one line of text.
- The view (008) names each country (`countries[].name`), shows each seat's AI board as a
  `board` (name, innate rule, tracks with every step, slots, unique cards), and keeps
  module bookkeeping vars (threat payments, discounts, flags, ...) out of the globals:
  each module declares them and shows what players need where it belongs.

## Card art draws (026-045)

Moved from the README in 048, as written.

**Scene draws.** Round 1 (035): 12 draws at 3 at once, 30–55 s each, about
2.5 min, 3% of the 5-hour window. Round 2 (036): 39–76 s each, about 3 min for 12, 24 draws in all. Batch 1 (037):
27 draws, 32–73 s each, about 7 min for 24, 7% of the window. Batch 2 (039): 40 draws at
3 at once, 31–82 s each, about 10 min, 10% of the window, none refused. Batch 3 (043):
40 draws at 3 at once, 30–77 s each, about 11 min, 10% of the window, none refused. Batch 4
(045): 60 draws at 3 at once, 31–103 s each, about 17.5 min, 17% of the window, none refused.

- **Choosing a batch** (40 scenes, the rule behind batches 1–3). Leave out scenes already
  drawn and any group whose content awaits a redesign (adversary, Discord, Unity, all
  drawn already). Judge a scene by the card it was written for. Tier-0 starters first, all
  drawn by batch 2; then one tier-2 scene per tag, the cheapest; the rest tier 1. Within a
  tier, cheapest first: cost, then insight, then order in the pool. Split the 40 across the
  six tags so their drawn totals stay level, ties going to the tag with the most left
  undrawn. Each scene's first card is its own; the second fills the other art window (one
  program or event, one building), from the same tag and tier frame where one is free, and
  no card appears twice in a batch. Batch 4 took all 60 left: one tier-2 scene per tag, then
  tier 1, then the other tier 2. It needed 60 buildings of the 56 there are, so four tier-2
  buildings appear twice, as second cards only. All 158 card scenes are drawn, 25 in each of
  cyber, diplomacy, power and science, 29 in each of industry and security; none remain. With
  the 6 adversary, Discord and Unity scenes, that is 164 drawn.

- **Numbers (2026-10-30, 026):** asking for 69:94 with a same-shape guide returns
  1075×1464, the right shape already, so the crop takes at most a pixel. 44–71 s an image,
  about 0.35% of the 5-hour Plus window each; three at once ran fine. Round 2 (029):
  23 draws, 48–73 s each, about 7% of the window. Independent draws drift by up to 3.5 mm:
  hence the one master.

## Older measurements

Moved from the README in 048, as written.

Measured on this machine (r11, 2 countries, random agent, 4:09pm, with a browser sweep
running): about 3.7 ms per game, replay included (every action and automatic phase
clones the state for choice replay); 2,000 games take about 7.5 s. Random games last
about 8 rounds (5..12) and all lose: 1,233 of 2,000 on the adversary's ambassador
supply, 598 on every city corrupted, 169 on the deck running out. (014: 1.5 ms, 5
rounds, almost all on the supply.)

Measured (v0, 2 countries): a record is about 9 KB (about 90 actions); saving 300
records with their games takes 0.8 s, and `replay` of 300 records 1.2 s. Saving every
game of a big run is slow: Windows file writes are the bottleneck (placeholder-era
figure: 200,000 records took 82 s). A shell glob over more than a few hundred records hits Windows'
command-line length limit, so replay big sets in batches.

**Timing on the 014 board** (baseline, release, 12 threads): `smoke.json --quick` (24
games; beam dropped) 0.8–1.3 s wall, greedy 0.3–0.6 s per game at d0 (at the old difficulty-4
ladder d4 games ended in round 1; rebuilt, rules.md §3); `screen-014-board.json --quick` (128 games) 11–13 s wall, per game greedy
0.7 s / engine 1.1–1.4 s with 2 countries, 1.9–2.3 s / 3.2–3.7 s with 3. Probes add to
that: win probability costs `rollouts` greedy playouts per round of each probed game
(the bulk of a probed game's time), closeness about one greedy decision per probed
decision. The per-cell split (agent, probes, total) is in each report's *Timing* table
and in `games.csv`.

Format 2 (016) renamed `margin.points` to `margin.unity_tips` (the `games.csv` column
too) and made each card's counts an object (`offered`, `drafted`, `kept`, `played`,
`pick_sum`, `drawn`) instead of a 6-number array; a format-1 stream can't be resumed.

`Cargo.lock` started as a copy of corelight-engine's lock, so dependency versions match a
known-good wasm build.

The golden fingerprint covers the config `name`, so a rename that plays the same moves it
with no bump (026: hardware to chips,
still r15). The exception is that rename: records from before 026 are r15 too and no
longer load (`board.factory_hardware: not a parameter`).

## Speed and costs (030-042)

Moved from the README in 048, as written.

Thread time a game at d2, r17 (030), old and new binaries under the same load, from the
32-seed screen below (12 threads run about 11 at once):

| | before | after | × |
|---|---|---|---|
| engine c2 | 2.43 s | 1.30 s | 1.87 |
| engine c3 | 10.1 s | 4.50 s | 2.25 |
| planner c2 | 22.2 s | 11.9 s | 1.87 |
| planner c3 | 77.7 s | 33.6 s | 2.32 |

The legacy `rollout` tier does a full beam search per decision: 92 s a game at c2, 373 s
at c3 (r17, one thread), so its `speed.py table` rows take 4-15 min each, and its
`same_play.py` trace cells 3-5 min each.

**What r19 and r20 cost (042, c2 d2, 16 seeds, one thread):** the agents make more calls
now, and that is the rules, not waste. Heuristic calls a decision, engine: 41.7 at r18, 49.5
at r19, 59.4 at r20, of which 041's doubled Growth look-ahead is 3.7. A call costs `eval`
13.7 us at r18 and 17.3 at r20 (35 before 042's fix), `apply` 12.4 and 14.8. Together:
planner 9.8 to 16.3 ms a decision, engine 1.2 to 2.1. About 2 in 3 `eval` calls inside
one sample return a value already seen there, but only 1 in 8 are the same state by
fingerprint, so a memo would have to key on what `eval` reads: not tried. Pricing a hand
goes through `cards::prices`, and anything counting several tags through
`cards::tag_counts`: a tag count walks the seat's cards.

Random is 2.4 × at c2, on 1 thread and on 12 (`speed.py pair`).

**Planner speed (030, r17 d2, one thread's time a game):** 11.9 s at 2 countries and 33.6 s at 3,
against the engine's 1.3 s and 4.5 s. A 32-seed d2 screen at both sizes with both agents
(`experiments/archive/screen-028.json`, 128 games) takes **about 2.5 min** on 12 threads: 153 s and
27 min of thread time, against 352 s and 60 min before 030, on the same busy machine.
`budget` 8 with `prune` 0 cost twice that and won no more within noise (028).

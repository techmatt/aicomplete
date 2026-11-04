# 103: where the last five rules items' wall time went

From `python scripts/bench/item_time.py 090_balance_pass_r35 093_rules_r36 096_rules_r37_followups
098_rules_r38_troops_dark 102_agents_and_r40`. Runs: 090 4:42pm-5:36pm, 093 6:51pm-7:39pm,
096 7:46pm-8:18pm, 098 9:02pm-9:47pm (all 2026-11-03), 102 4:29am-5:57am (2026-11-04).

| item | wall min | sims | builds | tests | rest (model alone) |
|---|---:|---:|---:|---:|---:|
| 090_balance_pass_r35 | 54 | 8 (15%) | 7 (12%) | 7 (14%) | 32 (59%) (27, 49%) |
| 093_rules_r36 | 48 | 15 (31%) | 4 (8%) | 5 (11%) | 24 (50%) (21, 44%) |
| 096_rules_r37_followups | 32 | 6 (18%) | 2 (7%) | 3 (10%) | 21 (66%) (17, 55%) |
| 098_rules_r38_troops_dark | 45 | 29 (64%) | 1 (2%) | 0 (0%) | 16 (34%) (14, 31%) |
| 102_agents_and_r40 | 88 | 40 (46%) | 6 (7%) | 7 (9%) | 34 (39%) (29, 33%) |
| **total** | 267 | 98 (37%) | 20 (7%) | 23 (9%) | 126 (47%) (109, 41%) |

Minutes (share of wall). Rest is model time plus short tools (reads, edits, greps, quick scripts);
the bracket is the model alone, when no tool call was open.

Background work that ran behind the model's own time, not counted above: sims 24 min, tests 9 min,
builds 5 min over the five items. A waited-on step (10 s or more): sim median 1.3 min (50 steps,
longest 10 min, a `per_decision.py --games 16`), build median 0.6 min (27 steps, longest 1.7),
test median 0.7 min (31 steps, longest 2.3).

## Method

Wall runs from the run.json's `started` to the main transcript's last line (090 and 093 each had a
2-3 minute launch before a parked gap, left out). Each foreground tool call (tool_use to
tool_result) is classed by its command; each `wait_for` by the command that last redirected into
the log or marker it names (the segment writing it, else the work just before an echoed marker);
every second no call covers is the model's. Overlaps go to sim, then build, then test. Unclassified
calls over 20 s total 5 min (3 min of it 090 waiting on its read-only Workflow), all counted as rest.

## Answer

Sim time is not a small slice: it is 37% of wall over the five items, the largest share after the
model's own time, and the largest of all in 098 (64%) and 102 (46%), the two items whose read
leaned on screens, fresh seeds and `speed.py` / `per_decision.py`. Builds (7%) and tests (9%) do
not dominate anywhere: a release build is about half a minute to a minute and a test step under a
minute, and much of each already runs detached behind the model. The model's own thinking and
writing is the other big share (41%, rest 47% with short tools). So cutting sim wall (fewer or
cheaper cells, the per-decision read, the speed check pair) is where waiting time is, alongside
fewer model turns; build and test time is a minor cost at today's sizes.

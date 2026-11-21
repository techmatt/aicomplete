# Running a rules, tuning or pricing item

The conventions such an item reads, screens and reports by, beside [CLAUDE.md](../CLAUDE.md)'s *Reads* and *Report*
(same seeds, fresh confirmation, frozen runs, speed: not repeated here). Matt's rulings, the bands and the
simulation scale among them, are in [rulings.md](rulings.md); recipes and costs in
[experiments.md](experiments.md); the warning checks in [principles.md](principles.md), *Design warnings*.

## Read before stating

- A mechanic comes from [rules/](rules/README.md), a number from its results file with its scope: never from a
  summary or memory. A number in a report is a claim until the report shows the check.
- A number you relay carries its revision and item. A prompt naming its base as "main's current revision" says
  where to read its starting numbers: read them there.
- A measured effect from an older revision is re-read before anything is built on it; say what follows if it no
  longer holds (240 planned levers on r100's B+P lead, gone by r108).
- A told change to a running item's base means re-reading the levers picked before it.

## Before adding a mechanic, card or number

- Reason it first: its place to matter in a 6-8 round game, its starting value, whether sim players can play it,
  its loops and caps, give against get ([card-guidelines.md](card-guidelines.md), *The anchor check*).
- A new harm gets its [harms.md](harms.md) row in the same item.
- Matt's examples' tags and numbers are variables, not rulings.
- Before naming a default for a staged design question, ask what a designer would object to: tier, symmetry across boards,
  narrative, token cost.

## Seeds and reads

- Each rules or tuning item reads the previous binary on fresh seeds, paired.
- A band check pools two independent sets.
- A sim-player heuristic gate pools at least 32 seeds.
- An item editing the agents' eval traces one judge game (`aic-sim run --trace`) or reads `per_decision.py`'s
  rounds column before its first farm: a collapsed horizon shows there in a minute.
- Items running alongside each other use disjoint seeds and share no logs.

## Screening levers

- Each lever is reachable, named by its parameter and screened on and off. A spec pins the off arm
  (`strategist:w.am_cap=0`, not the bare `strategist`): the bare agent changes when the item ships the lever.
- The prompt's *What you may change* closes the list, and says whether a lever you find outside it may ship flagged
  or is report-only.
- After a change to how the game is won, greedy is read beside the strategist.
- A retuned value comes from a paired screen of at least 500 games a cell, confirmed once on new seeds.

## Bars

- Each done-when line is a measure as read and a bar: scored strictly, reported against, not chased.
- Method is a suggested route unless the prompt marks it `(required)`.
- With several bars on one level, the prompt says which gives way first.
- An oracle bar is a floor, read on 48 or more games.
- A bar on an arm the repo has never measured gets its baseline read first.
- In a pricing pass, a card breaking no ruling keeps its price.
- A pass or overhaul edits and measures: it is not a read-only audit.
- A band miss is reported with what the prompt says follows from it.

## What the report carries

- A rules item: the new-player summary of its rule changes, and how each resource is spent and whether it binds.
- Your pick is written as a pick ("Matt overrides") until Matt rules: never "Matt agreed" ahead of him.
- Design warnings lead, for Matt.

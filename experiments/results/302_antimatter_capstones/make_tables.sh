#!/bin/sh
# 302: rewrites fresh.md from the fresh read under out/302_antimatter_capstones/fresh/ (git-ignored; kept under
# C:/Code/aicomplete-control/.conductor/kept/aicomplete/302_antimatter_capstones/302). Reads with bench/rounds.py,
# 301's caps_read.py and bench/resource_use.py. Run from the repo root.
set -e
F=out/302_antimatter_capstones/fresh
S=experiments/results/302_antimatter_capstones
R=experiments/farm/read.json
{
  echo "# 302: the fresh read"
  echo
  echo "The judge, \`read.json\` (two seats, the three pairs, d0-d3), fresh seeds 51168-51263, 288 games a level: r146"
  echo "is main at 05d7043f farmed with \`--at\`; r147 is this item's build (0b0c8d77). Columns as in 301's"
  echo "[screens.md](../301_d0_early_wins_and_liberate_pricing/screens.md)."
  echo
  python scripts/bench/rounds.py $R r146=$F/r146-judge.jsonl r147=$F/r147-judge.jsonl
  echo
  echo "## The liberate cards, per 100 games"
  echo
  python experiments/results/301_d0_early_wins_and_liberate_pricing/caps_read.py r146=$F/r146-judge.jsonl r147=$F/r147-judge.jsonl
  echo
  echo "## Antimatter, per seat a game (bench/resource_use.py; new = r147)"
  echo
  python scripts/bench/resource_use.py $F/r147-judge.jsonl=$R $F/r146-judge.jsonl=$R | sed -n '3,13p'
} > $S/fresh.md

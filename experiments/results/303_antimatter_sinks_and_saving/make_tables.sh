#!/bin/sh
# 303: rewrites fresh.md and screen.md from the reads under out/303_antimatter_sinks_and_saving/ (git-ignored; kept
# under C:/Code/aicomplete-control/.conductor/kept/aicomplete/303_antimatter_sinks_and_saving). Reads with bench/rounds.py,
# bench/am_cards.py (this item's am_cards.py), this item's am_spend.py, and bench/read_lib.py. Run from the repo root.
set -e
O=out/303_antimatter_sinks_and_saving
S=experiments/results/303_antimatter_sinks_and_saving
R=experiments/farm/read.json
{
  echo "# 303: the fresh read"
  echo
  echo "The judge, \`read.json\` (two seats, the three pairs, d0-d3), fresh seeds 51264-51359, 288 games a level: r147"
  echo "is main at 2c78093c farmed with \`--at\`; r148 is this item's build."
  echo
  python scripts/bench/rounds.py $R r147=$O/fresh/r147-judge.jsonl r148=$O/fresh/r148-judge.jsonl
  echo
  echo "## Wins, paired (bench/read_lib.py)"
  echo
  python scripts/bench/read_lib.py $O/fresh/r147-judge.jsonl=$R@r147 $O/fresh/r148-judge.jsonl=$R@r148 --base r147/base | sed -n '3,13p'
  echo
  echo "## Antimatter: the sinks per 100 games, and spent ÷ gained (am_cards.py)"
  echo
  python scripts/bench/am_cards.py r147=$O/fresh/r147-judge.jsonl r148=$O/fresh/r148-judge.jsonl --also clearance_strike
  echo
  echo "## Antimatter by use and source, per 100 games (am_spend.py)"
  echo
  python $S/am_spend.py r147=$O/fresh/r147-judge.jsonl r148=$O/fresh/r148-judge.jsonl --top 20
} > $S/fresh.md
SP=experiments/farm/s303.json
{
  echo "# 303: the dev screen (seeds 36800-36831, 384 games an arm)"
  echo
  echo "\`s303.json\`: \`nocards\` farmed on b3134094 (the agent lever alone, r147's deck), \`cards\` on 71557a95 (the"
  echo "three screened cards, Clearance Strike among them, and the lever before the diff review's fixes). Arms: the judge"
  echo "off, and \`w.am_cap=1\` with \`w.am_idle\` 0.5 or 0.2."
  echo
  python scripts/bench/am_cards.py off=$O/screen/nocards.jsonl:3:0 idle05=$O/screen/nocards.jsonl:3:1 idle02=$O/screen/nocards.jsonl:3:2 cards-off=$O/screen/cards.jsonl:3:0 cards-idle05=$O/screen/cards.jsonl:3:1 cards-idle02=$O/screen/cards.jsonl:3:2 --also clearance_strike
  echo
  python $S/am_spend.py off=$O/screen/nocards.jsonl:3:0 idle02=$O/screen/nocards.jsonl:3:2 cards-off=$O/screen/cards.jsonl:3:0 cards-idle02=$O/screen/cards.jsonl:3:2 --top 14
} > $S/screen.md

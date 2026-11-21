#!/bin/sh
# 306: rewrites fresh.md and screen.md from the reads under out/306_finale_buildings/ (git-ignored; kept under
# C:/Code/aicomplete-control/.conductor/kept/aicomplete/306_finale_buildings). The screen's arm files are split out of
# the farmed s306 / s306b / s306c files by cell (the specs are in experiments/archive/farm/); pool-r148 / pool-r149
# are the screen's r148 / r149final and the fresh read's two sides, concatenated. Readers:
# bench/rounds.py, bench/read_lib.py, 303's am_cards.py and am_spend.py, this item's finale.py. Run from the repo root.
set -e
O=out/306_finale_buildings
S=experiments/results/306_finale_buildings
S3=experiments/results/303_antimatter_sinks_and_saving
R=experiments/farm/read.json
F="r148=$O/fresh/r148.jsonl r149=$O/fresh/r149.jsonl"
{
  echo "# 306: the fresh read"
  echo
  echo "The judge, \`read.json\` (two seats, the three pairs, d0-d3), fresh seeds 51360-51455, 288 games a level: r148"
  echo "is main at 065566d7 farmed with \`--at\`; r149 is this item's build."
  echo
  python scripts/bench/rounds.py $R $F
  echo
  echo "## Wins, paired (bench/read_lib.py)"
  echo
  python scripts/bench/read_lib.py $O/fresh/r148.jsonl=$R@r148 $O/fresh/r149.jsonl=$R@r149 --base r148/base | sed -n '3,13p'
  echo
  echo "## The finale buildings and the older antimatter Actions (finale.py)"
  echo
  python $S/finale.py $F
  echo
  echo "Paired (am_paired.py):"
  echo
  python $S/am_paired.py $O/fresh/r148.jsonl $O/fresh/r149.jsonl
  echo
  echo "## Antimatter: the sinks per 100 games, and spent ÷ gained (303's am_cards.py)"
  echo
  python $S3/am_cards.py $F
  echo
  echo "## Antimatter by use and source, per 100 games (303's am_spend.py)"
  echo
  python $S3/am_spend.py $F --top 24
} > $S/fresh.md
SC=$O/screen
{
  echo "# 306: the screen (seeds 51264-51359, 303's spent fresh set, 1,152 games an arm, paired)"
  echo
  echo "r148 is main at 065566d7 (it reproduces 303's kept r148 read on these seeds exactly). Arms on r149's first deck"
  echo "(Siegebreaker Reactor needing 3 Security): \`base\`; \`eased\` (Siegebreaker, Fusion Torch, Singularity need 2,"
  echo "Concord 8 credits); \`idleoff\` (the judge with w.am_cap=0); \`am_act\` (a keep lever, reverted); \`pair\`"
  echo "(Siegebreaker needs 3 Power, Fusion Torch 3 Industry); \`up\` (each finale gains 1 antimatter when built); \`both\`;"
  echo "\`final\`, r149 as shipped (Siegebreaker 3 Power, Singularity gaining 2 insight)."
  echo
  python $S/am_paired.py $SC/r148.jsonl $SC/r149final.jsonl
  echo
  echo "Pooled with the fresh read (2,304 games, r148 against r149 as shipped):"
  echo
  python $S/am_paired.py $O/pool-r148.jsonl $O/pool-r149.jsonl
  echo
  python scripts/bench/read_lib.py $SC/r148.jsonl=$R@r148 $SC/r149a.jsonl=$R@base $SC/s306-eased-judge.jsonl=$R@eased $SC/s306-base-idleoff.jsonl=$R@idleoff $SC/b-on.jsonl=$R@am_act $SC/c-pair.jsonl=$R@pair $SC/c-up.jsonl=$R@up $SC/c-both.jsonl=$R@both $SC/r149final.jsonl=$R@final --base r148/base | sed -n '3,25p'
  echo
  python $S3/am_cards.py r148=$SC/r148.jsonl base=$SC/r149a.jsonl eased=$SC/s306-eased-judge.jsonl idleoff=$SC/s306-base-idleoff.jsonl am_act=$SC/b-on.jsonl pair=$SC/c-pair.jsonl up=$SC/c-up.jsonl both=$SC/c-both.jsonl final=$SC/r149final.jsonl | grep -E '^\| (Read|---|[a-z0-9_]+ \| all)'
  echo
  python $S/finale.py base=$SC/r149a.jsonl eased=$SC/s306-eased-judge.jsonl pair=$SC/c-pair.jsonl up=$SC/c-up.jsonl final=$SC/r149final.jsonl --cards siegebreaker_reactor,concord_reactor,fusion_torch_array,singularity_exploit
} > $S/screen.md

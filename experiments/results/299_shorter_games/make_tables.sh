#!/bin/sh
# 299: rewrites screens.md and fresh.md from the farmed games under out/299_shorter_games/ (git-ignored; the fresh
# files are kept under C:/Code/aicomplete-control/.conductor/kept/aicomplete/299_shorter_games). Run from the repo root.
set -e
D=out/299_shorter_games
S=experiments/results/299_shorter_games
K=C:/Code/aicomplete-control/.conductor/kept/aicomplete/298_overhaul_6_calibration/298
H="python $S/hist.py"
{
  echo "# 299: the screens"
  echo
  echo "Written by \`make_tables.sh\` (\`hist.py\`): the judge, two seats, the three pairs pooled; a row a level. Columns:"
  echo "wins, mean rounds, the share of games ending in rounds 6-8 (bar: over 50%), the share at 10+ (bar: under ~10%),"
  echo "losses at the round cap of all losses, wins by round 3, then the histogram in percent. A cap game records the"
  echo "deck's cards + 1 rounds (it ends in that round's Card beat), so a 10-card deck's cap losses show at 11."
  echo
  echo "## Screen 1 (dev seeds 36800-36831, 96 games a level; r144 at fdbfcae4; \`s299a.json\`)"
  echo
  echo "lib4: Liberate at 4; amb2: two starting ambassadors; deck: 4/4/2 (d3 4/3/2); both: all three."
  echo
  $H experiments/farm/s299a.json s1=$D/s1/s299a.jsonl --by-arm
  echo
  echo "## Screens 2-7 (298's spent fresh seeds 50688-50783, 288 games a level)"
  echo
  echo "base: 298's kept r144 read on the same seeds. d11: 5/4/2 (d3 5/3/2); d10: 4/4/2 (d3 4/3/2); lib3/lib4: Liberate"
  echo "at 3/4; amb0/amb2: starting ambassadors; st3: a station at 3; free3: a freed city sways 3 Unity; c6: cities"
  echo "corrupt at 6 agents (4 from d1); tip2: Unity tips at 2 a player; ready0: supply ambassadors arrive spent. Screens"
  echo "6 and 7 run on r145 (lib3d10), so their arms name only what they add."
  echo
  $H experiments/farm/read.json base=$K/r144-judge.jsonl
  echo
  $H experiments/farm/s299b.json s2=$D/s2/s299b.jsonl --by-arm
  echo
  $H experiments/farm/s299c.json s3=$D/s3/s299c.jsonl --by-arm
  echo
  $H experiments/farm/s299d.json s4=$D/s4/s299d.jsonl --by-arm
  echo
  $H experiments/farm/s299e.json s5=$D/s5/s299e.jsonl --by-arm
  echo
  $H experiments/farm/s299f.json s6=$D/s6/s299f.jsonl --by-arm
  echo
  $H experiments/farm/s299g.json s7=$D/s7/s299g.jsonl --by-arm
  echo
  echo "## Three seats (dev seeds, 32 games a level; \`configs/three_seat.json\`; \`s299t.json\`, \`s299u.json\`)"
  echo
  $H experiments/farm/s299t.json t=$D/s2/s299t.jsonl --by-arm
  echo
  $H experiments/farm/s299u.json u=$D/s4/s299u.jsonl --by-arm
} > $S/screens.md
{
  echo "# 299: the fresh reads"
  echo
  echo "The judge; r144 is main at 68fc022c farmed with \`--at\` on the same seeds. Two seats: \`read.json\`, two sets of 48"
  echo "pooled, the three pairs, 288 games a level. Three seats: \`read3.json\` (48 games a level; r144's side is"
  echo "\`s298t\`'s bpr-base cell). Columns as in [screens.md](screens.md)."
  echo
  echo "## r145 as shipped (878be62d = 036a1e2e's play: Liberate 3, deck 4/4/2): seeds 50784-50879, three seats 50880-50927"
  echo
  F=$D/fresh
  $H experiments/farm/read.json r144=$F/r144-judge.jsonl r145=$F/r145-judge.jsonl
  echo
  python $S/hist.py experiments/farm/s298t.json r144=$F/r144-3seat.jsonl --by-arm | grep -v "c1 \|lib4 "
  echo
  python $S/hist.py experiments/farm/read3.json r145=$F/r145-3seat.jsonl
  echo
  echo "## The alternative, built and not kept (70f7b43b: Liberate 4, cities at 6 agents, 4 from d1, deck 4/4/2): seeds"
  echo "50928-51023, three seats 51024-51071"
  echo
  C=$D/confirm
  $H experiments/farm/read.json r144=$C/r144-judge.jsonl alt=$C/r145-judge.jsonl
  echo
  python $S/hist.py experiments/farm/s298t.json r144=$C/r144-3seat.jsonl --by-arm | grep -v "c1 \|lib4 "
  echo
  python $S/hist.py experiments/farm/read3.json alt=$C/r145-3seat.jsonl
  echo
  echo "## The two builds paired: the alternative (70f7b43b, \`--at\`) on r145's seeds 50784-50879"
  echo
  $H experiments/farm/read.json r145=$F/r145-judge.jsonl alt=$F/alt-judge.jsonl
  echo
  echo "## The adversary's spread (Matt's message 13811), two seats: all three builds on 50784-50879, then on 50928-51023"
  echo
  python $S/spread.py r144=$F/r144-judge.jsonl r145=$F/r145-judge.jsonl alt=$F/alt-judge.jsonl
  echo
  python $S/spread.py r144=$C/r144-judge.jsonl alt=$C/r145-judge.jsonl
} > $S/fresh.md

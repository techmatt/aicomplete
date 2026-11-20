#!/bin/sh
# 301: rewrites screens.md and fresh.md from the farmed games under out/301_d0_early_wins_and_liberate_pricing/
# (git-ignored; the fresh files are kept under C:/Code/aicomplete-control/.conductor/kept/aicomplete/
# 301_d0_early_wins_and_liberate_pricing). Reads with 299's hist.py. Run from the repo root.
set -e
D=out/301_d0_early_wins_and_liberate_pricing
S=experiments/results/301_d0_early_wins_and_liberate_pricing
K=C:/Code/aicomplete-control/.conductor/kept/aicomplete/299_shorter_games/299
H="python experiments/results/299_shorter_games/hist.py"
{
  echo "# 301: the screens"
  echo
  echo "The judge, two seats, the three pairs pooled, 288 games a level, all on 299's spent fresh seeds 50784-50879"
  echo "(r145's own read, kept: \`base\`, \`r145\`). Columns as in 299's [screens.md](../299_shorter_games/screens.md);"
  echo "\"Won by r3\" counts wins in rounds 2-3 (bar: about 1%, 3 of 288)."
  echo
  echo "## Screen 1 (r145): d0 compute income 2/3/5; a station at 3 (\`s301a.json\`)"
  echo
  $H experiments/farm/s301a.json s1=$D/s1/s301a.jsonl --by-arm
  echo
  echo "## Screen 2 (r145 + P1): agents on each starting corrupted city at setup, level 0 (\`archive/s301b.json\`)"
  echo
  $H experiments/archive/s301b.json s2=$D/s2/s301b.jsonl --by-arm
  echo
  echo "## Screen 3 (the priced build: a station at 3, the liberate cards repriced, values.json's liberate 20, P1;"
  echo "\`archive/s301c.json\`, \`s301d.json\`): \`price\` alone; level 0's agents on the starting corrupted cities (sa) and"
  echo "on the free ones (seed); d1-d3 against r145's kept read"
  echo
  $H experiments/archive/s301c.json s3=$D/s3/s301c.jsonl --by-arm
  echo
  $H experiments/farm/read.json r145=$K/r145-judge.jsonl | grep -v "| d0 |"
  echo
  $H experiments/farm/s301d.json price=$D/s3/s301d.jsonl | tail -3
  echo
  echo "## Screen 4 (the priced build): more agents on the free cities, a troop a country at setup (\`archive/s301e.json\`)"
  echo
  $H experiments/archive/s301e.json s4=$D/s4/s301e.jsonl --by-arm
} > $S/screens.md
{
  echo "# 301: the fresh read"
  echo
  echo "The judge, \`read.json\` (two seats, the three pairs, d0-d3), fresh seeds 51072-51167, 288 games a level: r145"
  echo "is main at a099a19d farmed with \`--at\`; r146 is this item's frozen build. Columns as in [screens.md](screens.md)."
  echo
  F=$D/fresh
  $H experiments/farm/read.json r145=$F/r145-judge.jsonl r146=$F/r146-judge.jsonl
  echo
  echo "d0's lever off (\`s301f.json\`: r146 with \`setup_city_agents\` 0, the pricing alone), same seeds:"
  echo
  $H experiments/farm/s301f.json r146-nolever=$F/r146-nolever.jsonl
} > $S/fresh.md

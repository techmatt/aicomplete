#!/bin/sh
# 298: rewrites screens.md and fresh.md from the farmed games under out/298_overhaul_6_calibration/ (git-ignored;
# the fresh files are kept under C:/Code/aicomplete-control/.conductor/kept/aicomplete/298_overhaul_6_calibration).
# Run from the repo root.
set -e
D=out/298_overhaul_6_calibration
S=experiments/results/298_overhaul_6_calibration
SMOKE=experiments/results/297_overhaul_5_content_sweep/smoke_table.py
{
  echo "# 298: the dev-seed screens"
  echo
  echo "Written by \`make_tables.sh\` (\`screen.py\`) from farmed games; dev seeds 36800-36831, 32 a pair, the judge. Arms"
  echo "are \`set\` lists (the specs: \`experiments/farm/s298a.json\`, \`s298r.json\`, \`s298t.json\`). Per-seat columns divide by"
  echo "the seats; Accord is per Protector seat-round."
  echo
  echo "## Screen 1 (f3facbcc: r144's Redeploy and Offensive in, the r143 judge; the three pairs pooled, 96 games a cell)"
  echo
  echo "base: Arms Race off (\`arms_race_at=5\`), r143 prices; arms: Arms Race on; price: + Martial Law 36, Gain a player"
  echo "ambassador 20, Accord 3 energy, Remove agents 3 ambassadors; home: price + the Strike rider (\`board.strike_home=1\`);"
  echo "k3: price + \`research_at=3,3,2,2,2\`."
  echo
  python $S/screen.py $D/s1/spec.json $D/s1/s298a.jsonl
  echo
  echo "## The Strike rider on the final build (4ad672dc: r144 and the retaught judge; Matt's message 13713)"
  echo
  python $S/screen.py experiments/farm/s298r.json $D/s2/s298r.jsonl
  echo
  echo "## Three seats (4ad672dc; configs/three_seat.json, Builder + Protector + Researcher, 32 games a cell)"
  echo
  echo "base: the config; c1: one corrupted city a country at setup; lib4: Liberate at 4 ambassadors."
  echo
  python $S/screen.py experiments/farm/s298t.json $D/s2/s298t.jsonl
  echo
  echo "Greedy at three seats (\`s298tg.json\`):"
  echo
  python $S/screen.py experiments/farm/s298tg.json $D/s2/s298tg.jsonl
  echo
  echo "## The judge's own terms against greedy (46bc29cd, the final build; \`s298j.json\`, 384 games an agent; wins (rounds))"
  echo
  python $S/agents_table.py experiments/farm/s298j.json $D/s3/s298j.jsonl
} > $S/screens.md
{
  echo "# 298: the fresh read"
  echo
  echo "Seeds 50688-50783 (two independent sets of 48, pooled), two players, the three pairs pooled, 288 games a row; r143"
  echo "is main at eca621c2be farmed with \`--at\`, r144 this item's build at 050cda02 (\`read.json\`, the judge;"
  echo "\`read295g.json\`, greedy). Written by \`make_tables.sh\` with 297's \`smoke_table.py\`."
  echo
  F=$D/fresh
  python $SMOKE r143-judge=$F/r143-judge.jsonl r144-judge=$F/r144-judge.jsonl r143-greedy=$F/r143-greedy.jsonl r144-greedy=$F/r144-greedy.jsonl
} > $S/fresh.md

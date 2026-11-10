#!/bin/sh
# 160: ms a game on one core with the box otherwise quiet: each engine plays the same N seeds from FIRST in one
# process, one engine after another, native std first and last (drift). From the repo root; needs what
# measure160.sh needs.
#   sh experiments/results/160_wasm_sim/measure160_one.sh FIRST N OUT     (OUT: relative, forward slashes)
FIRST=$1; N=$2; OUT=$3
mkdir -p "$OUT"
one() {  # NAME, then the command with FILE as the output's placeholder
  name=$1; shift
  cmd=$(echo "$*" | sed "s|FILE|$OUT/$name.jsonl|")
  sh -c "$cmd" 2> "$OUT/$name.err" > "$OUT/$name.out" || echo "$name FAILED" >> "$OUT/progress.log"
  echo "$name done $(date +%H:%M:%S)" >> "$OUT/progress.log"
}
one native-std "out/new/aic-sim.exe farm cell-d2 --seed $FIRST --games $N --out FILE"
one wasm-node "node scripts/farm/run.mjs out/bundle cell-d2 $FIRST $N FILE"
one wasm-browser "node scripts/farm/run.mjs out/bundle cell-d2 $FIRST $N FILE --browser --port 51459"
one native-libm "out/libm/aic-sim.exe farm cell-d2 --seed $FIRST --games $N --out FILE"
one native-std2 "out/new/aic-sim.exe farm cell-d2 --seed $FIRST --games $N --out FILE"
echo "ALL DONE" >> "$OUT/progress.log"

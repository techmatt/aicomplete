#!/bin/sh
# 151: runs r64's binary (out/r64, its own configs) on read specs, one after another.
# sh run_old.sh TAG... (from the repo root): out/r64/experiments/read-151-TAG-{a,b}.json -> out/old151/TAG-{a,b}
set -e
ROOT=$(pwd)
for tag in "$@"; do
  for half in a b; do
    cp "experiments/read-151-$tag-$half.json" "out/r64/experiments/"
    (cd out/r64 && nice -n 10 ./aic-sim.exe experiment "experiments/read-151-$tag-$half.json" --threads 3 --out "$ROOT/out/old151/$tag-$half" > "$ROOT/out/old151/$tag-$half.log" 2>&1)
  done
  echo "OLD DONE $tag" >> "$ROOT/out/old151/done.log"
done

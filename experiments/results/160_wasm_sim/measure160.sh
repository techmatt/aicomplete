#!/bin/sh
# 160: the native / wasm measurements, from the repo root, on one cell (experiments/farm/cell-d2.json: the strategist,
# bp, d2). Each engine plays the same seeds as P shards of N seeds side by side (one process a shard, one core
# each), engines one after another, native std again last for drift.
#   sh experiments/results/160_wasm_sim/measure160.sh FIRST N P OUT
# OUT: a relative, forward-slash path (it goes through sed and sh -c). Needs out/new/aic-sim.exe (release,
# default), out/libm/aic-sim.exe (release, --features libm) and the bundle in out/bundle (node
# scripts/farm/build.mjs out/bundle). Writes OUT/<engine>/<shard>.{jsonl,err} and OUT/<engine>.jsonl; a shard that
# fails is a `FAILED` line in OUT/progress.log (rerun it alone), and the rest go on.
FIRST=$1; N=$2; P=$3; OUT=$4
mkdir -p "$OUT"
run() {  # NAME, then the command with SEED, COUNT, FILE as placeholders
  name=$1; shift
  mkdir -p "$OUT/$name"
  rm -f "$OUT/$name"/*.jsonl
  i=0
  while [ $i -lt "$P" ]; do
    s=$((FIRST + i * N))
    cmd=$(echo "$*" | sed "s|SEED|$s|; s|COUNT|$N|; s|FILE|$OUT/$name/$i.jsonl|")
    nice -n 10 sh -c "$cmd || echo '$name shard $i FAILED' >> '$OUT/progress.log'" 2> "$OUT/$name/$i.err" > "$OUT/$name/$i.out" &
    i=$((i + 1))
  done
  wait
  i=0; : > "$OUT/$name.jsonl"; : > "$OUT/$name.err"
  while [ $i -lt "$P" ]; do
    [ -f "$OUT/$name/$i.jsonl" ] && cat "$OUT/$name/$i.jsonl" >> "$OUT/$name.jsonl"
    cat "$OUT/$name/$i.err" >> "$OUT/$name.err"
    i=$((i + 1))
  done
  echo "$name done $(date +%H:%M:%S)" >> "$OUT/progress.log"
}
run native-std "out/new/aic-sim.exe farm cell-d2 --seed SEED --games COUNT --out FILE"
run native-libm "out/libm/aic-sim.exe farm cell-d2 --seed SEED --games COUNT --out FILE"
run wasm-node "node scripts/farm/run.mjs out/bundle cell-d2 SEED COUNT FILE"
run wasm-browser "node scripts/farm/run.mjs out/bundle cell-d2 SEED COUNT FILE --browser"
run native-std2 "out/new/aic-sim.exe farm cell-d2 --seed SEED --games COUNT --out FILE"
echo "ALL DONE" >> "$OUT/progress.log"

#!/bin/sh
# 251: the remaining profile-feature splits: the full oracle at d2 on both builds, the real oracle d0-d4 on the new.
# Usage: sh prof_more.sh BASE_PROF_BIN NEW_PROF_BIN OUTDIR
B=$1
N=$2
OUT=$3
mkdir -p "$OUT"
"$B" experiment experiments/farm/i251-full.json --games 4 --seed 36800 --threads 1 --out "$OUT/full-base" > "$OUT/full-base.log" 2>&1
"$N" experiment experiments/farm/i251-full.json --games 4 --seed 36800 --threads 1 --out "$OUT/full-new" > "$OUT/full-new.log" 2>&1
"$N" experiment out/251_profiling/real-levels.json --games 1 --seed 36800 --threads 3 --out "$OUT/real-new" > "$OUT/real-new.log" 2>&1
echo PROFMOREDONE

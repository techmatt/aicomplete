#!/bin/sh
# 251: the profile-feature runs, one per arm, each printing its split to its log.
# Usage: sh prof_runs.sh BIN OUTDIR [TAG]
BIN=$1
OUT=$2
TAG=${3:-x}
mkdir -p "$OUT"
S=experiments/farm
"$BIN" experiment $S/i251-judges.json --games 2 --seed 36800 --threads 1 --out "$OUT/judges-$TAG" > "$OUT/judges-$TAG.log" 2>&1
"$BIN" experiment $S/p248j.json --games 1 --seed 36800 --threads 1 --out "$OUT/p248j-$TAG" > "$OUT/p248j-$TAG.log" 2>&1
echo PROFDONE

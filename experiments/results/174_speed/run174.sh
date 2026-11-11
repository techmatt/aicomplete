#!/bin/sh
# Runs specs on one binary, one after another, from the repo root (174): each spec SPEC writes OUT/SPEC/ and
# OUT/SPEC.log with its wall seconds appended (`WALL N`), then `RUN DONE` goes to OUT/run.done.
#
#   sh experiments/results/174_speed/run174.sh EXE OUT THREADS SPEC...
#
# EXE: the binary; OUT: an output folder under out/; SPEC: a spec's path (configs/ read from the repo root).
set -e
EXE=$1; OUT=$2; T=$3; shift 3
mkdir -p "$OUT"
for spec in "$@"; do
  name=$(basename "$spec" .json)
  start=$(date +%s)
  nice -n 10 "$EXE" experiment "$spec" --threads "$T" --out "$OUT/$name" > "$OUT/$name.log" 2>&1
  echo "WALL $(( $(date +%s) - start ))" >> "$OUT/$name.log"
done
echo "RUN DONE" >> "$OUT/run.done"

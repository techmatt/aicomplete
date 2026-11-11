#!/bin/sh
# Runs a read's three sets on one binary, one after another, from the repo root (157's and 158's run_read.sh,
# promoted in 159). The specs are out/read-NNN-TAG-{a,b,c}.json, as `bench/read_specs.py read TAG SEED --tag NNN`
# writes them.
#
#   sh scripts/bench/run_read.sh NNN EXE ROOT TAG OUTPREFIX THREADS
#
# NNN: the specs' item number (read_specs.py's --tag); EXE: the binary; ROOT: the folder it runs in (its configs/:
# `.` for this tree, an old revision's worktree for the old side), the specs are copied into ROOT/out/; OUTPREFIX:
# each set writes OUTPREFIX-{a,b,c}/ and its log OUTPREFIX-{a,b,c}.log; THREADS: --threads. Appends
# `READ DONE TAG` to OUTPREFIX.done when all three are done (wait on that file). Run it in the background.
set -e
P=$1; EXE=$2; ROOT=$3; TAG=$4; OUT=$5; T=$6
TOP=$(pwd)
# 182: EXE and OUTPREFIX may be absolute (C:/... or /...) or relative to the repo root.
case "$EXE" in /*|[A-Za-z]:*) ;; *) EXE="$TOP/$EXE" ;; esac
case "$OUT" in /*|[A-Za-z]:*) ;; *) OUT="$TOP/$OUT" ;; esac
mkdir -p "$ROOT/out"
for half in a b c; do
  [ "$ROOT" = "." ] || cp "out/read-$P-$TAG-$half.json" "$ROOT/out/"
  (cd "$ROOT" && nice -n 10 "$EXE" experiment "out/read-$P-$TAG-$half.json" --threads "$T" --out "$OUT-$half" > "$OUT-$half.log" 2>&1)
done
echo "READ DONE $TAG" >> "$OUT.done"

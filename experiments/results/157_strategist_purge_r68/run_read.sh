#!/bin/sh
# 157: runs a read's three sets on one binary, one after another (from the repo root).
# sh run_read.sh EXE ROOT TAG OUTPREFIX THREADS
#   EXE: the binary; ROOT: the folder it runs in (its configs/); specs out/read-157-TAG-{a,b,c}.json are copied there.
set -e
EXE=$1; ROOT=$2; TAG=$3; OUT=$4; T=$5
TOP=$(pwd)
mkdir -p "$ROOT/out"
for half in a b c; do
  [ "$ROOT" = "." ] || cp "out/read-157-$TAG-$half.json" "$ROOT/out/"
  (cd "$ROOT" && nice -n 10 "$TOP/$EXE" experiment "out/read-157-$TAG-$half.json" --threads "$T" --out "$TOP/$OUT-$half" > "$TOP/$OUT-$half.log" 2>&1)
done
echo "READ DONE $TAG" >> "$TOP/$OUT.done"

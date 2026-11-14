#!/bin/sh
# A farm spec played natively as one-thread shards side by side, concatenated under its header: the base side of
# a paired read (`farm-seeds` builds only the item's commit, so an older revision's binary runs here, locally).
# 223's and 227's route, promoted from experiments/results/227_fallout/shards.sh in 229.
#
#   sh scripts/bench/shards.sh EXE DIR SPEC FIRST COUNT OUT
#
# EXE runs in DIR (its configs/ are read from there; EXE and SPEC absolute, or relative to DIR; OUT relative to here). Seeds FIRST ..
# FIRST+COUNT-1 split into SHARDS (environment, default 6) contiguous shards. `aic-sim farm` has no --threads: a
# shard is one thread, so SHARDS is the cores this takes; each runs below normal (`nice`). Shard outputs go to
# OUT.parts/N.jsonl (each game's time to OUT.parts/N.err); OUT is `aic-sim farm SPEC --header` for the whole range,
# then the shards in seed order: a games file with its header, which the read scripts take as it is (no =SPEC).
# Prints SHARDS-DONE OUT at the end; a failed shard stops it before OUT is written.
set -e
[ $# -eq 6 ] || { echo "usage: shards.sh EXE DIR SPEC FIRST COUNT OUT" >&2; exit 2; }
EXE=$1; DIR=$2; SPEC=$3; FIRST=$4; COUNT=$5; OUT=$6
N=${SHARDS:-6}
case "$OUT" in /*|?:*) ;; *) OUT="$(pwd)/$OUT" ;; esac  # the shards run in DIR: OUT is from here
mkdir -p "$OUT.parts"
per=$(( (COUNT + N - 1) / N ))
i=0
pids=""
while [ $i -lt $N ]; do
  s=$(( FIRST + i * per ))
  left=$(( FIRST + COUNT - s ))
  [ $left -le 0 ] && break
  n=$per; [ $n -gt $left ] && n=$left
  (cd "$DIR" && nice -n 10 "$EXE" farm "$SPEC" --seed $s --games $n --out "$OUT.parts/$i.jsonl" 2>"$OUT.parts/$i.err") &
  pids="$pids $!"
  i=$(( i + 1 ))
done
for p in $pids; do wait $p; done
(cd "$DIR" && "$EXE" farm "$SPEC" --header --seed $FIRST --games $COUNT) > "$OUT"
j=0
while [ $j -lt $i ]; do cat "$OUT.parts/$j.jsonl" >> "$OUT"; j=$(( j + 1 )); done
echo SHARDS-DONE "$OUT"

#!/bin/sh
# 227: run a farm spec natively as 6 one-thread shards and concatenate them (223's route for the base side).
# usage: shards.sh EXE WORKDIR SPEC FIRST COUNT OUT
# EXE runs in WORKDIR (its configs/ are read from there); SPEC is an absolute path.
set -e
EXE=$1; DIR=$2; SPEC=$3; FIRST=$4; COUNT=$5; OUT=$6
N=6
mkdir -p "$OUT.parts"
per=$(( (COUNT + N - 1) / N ))
i=0
pids=""
while [ $i -lt $N ]; do
  s=$(( FIRST + i * per ))
  left=$(( FIRST + COUNT - s ))
  [ $left -le 0 ] && break
  n=$per; [ $n -gt $left ] && n=$left
  (cd "$DIR" && nice -n 10 "$EXE" farm "$SPEC" --seed $s --games $n --out "$OUT.parts/$i.jsonl" 2>/dev/null) &
  pids="$pids $!"
  i=$(( i + 1 ))
done
for p in $pids; do wait $p; done
(cd "$DIR" && "$EXE" farm "$SPEC" --header --seed $FIRST --games $COUNT) > "$OUT"
j=0
while [ $j -lt $i ]; do cat "$OUT.parts/$j.jsonl" >> "$OUT"; j=$(( j + 1 )); done
echo SHARDS-DONE "$OUT"

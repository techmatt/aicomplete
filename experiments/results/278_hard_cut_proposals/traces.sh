#!/bin/sh
# 278: trace 268's seeds (268000-268013), d2 and d4, the judge, into OUTDIR as d<level>-<seed>.out, plus the same
# games as an experiment (per-round counters) in OUTDIR/exp, its spec's one variant carrying the same overrides.
#   sh experiments/results/278_hard_cut_proposals/traces.sh OUTDIR [k=v ...]
out=$1; shift
mkdir -p "$out"
S=target/release/aic-sim.exe
sets=""
list=""
for kv in "$@"; do
  [ -n "$kv" ] || continue  # an empty argument would pass a bare --set
  sets="$sets --set $kv"
  list="$list${list:+, }\"$kv\""
done
for L in 2 4; do
  for s in $(seq 268000 268013); do
    nice -n 10 $S run configs/baseline.json --games 1 --seed $s --agent strategist --set adversary.difficulty=$L $sets --trace > "$out/d$L-$s.out" 2>&1 &
  done
  wait
done
printf '{"name": "s278-trace", "configs": ["configs/baseline.json"], "variants": [{"name": "v", "set": [%s]}], "difficulties": [2, 4], "agents": ["strategist"], "games": 14, "first_seed": 268000, "records": 0}\n' "$list" > "$out/spec.json"
nice -n 10 $S experiment "$out/spec.json" --out "$out/exp" --threads 6 > "$out/exp.log" 2>&1
echo traced "$out"

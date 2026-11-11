#!/bin/sh
# 176's fresh ladder: r75 then r74 on the same specs (seeds 42133-42276), one after another, from the repo root.
set -e
TOP=$(pwd)
mkdir -p out/fresh
for s in f176-strategist-a f176-greedy-a f176-strategist-b f176-strategist-c; do
  nice -n 10 out/r75final.exe experiment out/$s.json --threads 6 --out out/fresh/new-$s > out/fresh/new-$s.log 2>&1
  echo "NEW $s" >> out/fresh/progress
done
cd out/r74
for s in f176-strategist-a f176-greedy-a f176-strategist-b f176-strategist-c; do
  nice -n 10 ./aic-sim.exe experiment out/$s.json --threads 6 --out "$TOP/out/fresh/old-$s" > "$TOP/out/fresh/old-$s.log" 2>&1
  echo "OLD $s" >> "$TOP/out/fresh/progress"
done
echo "FRESH DONE" >> "$TOP/out/fresh/progress"

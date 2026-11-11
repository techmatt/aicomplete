#!/bin/sh
# 175's fresh ladder: r74 then r73 on the same specs, 3 threads, one after another.
set -e
TOP=$(pwd)
for s in f175-strategist-a f175-greedy-a f175-strategist-b f175-strategist-c; do
  nice -n 10 out/r74final.exe experiment out/$s.json --threads 3 --out out/fresh/new-$s > out/fresh/new-$s.log 2>&1
  echo "NEW $s" >> out/fresh/progress
done
cd out/r73
for s in f175-strategist-a f175-greedy-a f175-strategist-b f175-strategist-c; do
  nice -n 10 ./aic-sim.exe experiment out/$s.json --threads 3 --out "$TOP/out/fresh/old-$s" > "$TOP/out/fresh/old-$s.log" 2>&1
  echo "OLD $s" >> "$TOP/out/fresh/progress"
done
echo "FRESH DONE" >> "$TOP/out/fresh/progress"

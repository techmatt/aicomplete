#!/bin/sh
# 251: the second paired CPU read, after the farms (a quieter box). Usage: sh cpu_pairs2.sh OLD NEW OUT
OLD=$1
NEW=$2
OUT=$3
mkdir -p "$OUT"
CP=experiments/results/251_profiling/cpu_pair.py
REAL="oracle:strategy=0,worlds=4,alts=3,budget=32,answers=1,truth=1,w.built=4,w.built_cost=0.5,w.clock=0,w.energy_cap=0,w.hand=1.5,w.hand_cost=0.1,w.horizon=0,w.insight_income=0,w.movement=0,w.packet_keep=1,w.production=0,w.strat_contents=0,w.table=0,w.tag_hand=0,w.purge=3.6,w.last_node=120"
echo "== judge d2, 6 games"
python $CP "$OLD" "$NEW" 6 -- run configs/baseline.json --agent strategist --games 6 --seed 36800 --threads 1 --set adversary.difficulty=2
echo "== greedy d2, 6 games"
python $CP "$OLD" "$NEW" 6 -- run configs/baseline.json --agent greedy --games 6 --seed 36800 --threads 1 --set adversary.difficulty=2
echo "== p248j at d2, seed 36800, 57 games"
python $CP "$OLD" "$NEW" 4 -- farm i251-p248j-d2 --seed 36800 --games 1 --out "$OUT/p248j-pair.jsonl"
echo "== real oracle d2, seed 36801, 1 game"
python $CP "$OLD" "$NEW" 2 -- run configs/baseline.json --agent "$REAL" --games 1 --seed 36801 --threads 1 --set adversary.difficulty=2
echo CPUPAIRS2DONE

#!/bin/sh
# 251: the paired CPU reads, OLD against NEW on the same games (cpu_pair.py). Usage: sh cpu_pairs.sh OLD NEW OUT
OLD=$1
NEW=$2
OUT=$3
mkdir -p "$OUT"
CP=experiments/results/251_profiling/cpu_pair.py
REAL="oracle:strategy=0,worlds=4,alts=3,budget=32,answers=1,truth=1,w.built=4,w.built_cost=0.5,w.clock=0,w.energy_cap=0,w.hand=1.5,w.hand_cost=0.1,w.horizon=0,w.insight_income=0,w.movement=0,w.packet_keep=1,w.production=0,w.strat_contents=0,w.table=0,w.tag_hand=0,w.purge=3.6,w.last_node=120"
echo "== real oracle d2, seed 36800, 1 game"
python $CP "$OLD" "$NEW" 2 -- run configs/baseline.json --agent "$REAL" --games 1 --seed 36800 --threads 1 --set adversary.difficulty=2
echo "== p248j at d2, seed 36800, 57 games"
python $CP "$OLD" "$NEW" 3 -- farm i251-p248j-d2 --seed 36800 --games 1 --out "$OUT/p248j-pair.jsonl"
echo "== greedy d2, 6 games"
python $CP "$OLD" "$NEW" 4 -- run configs/baseline.json --agent greedy --games 6 --seed 36800 --threads 1 --set adversary.difficulty=2
echo "== full oracle d2, 4 games"
python $CP "$OLD" "$NEW" 2 -- run configs/baseline.json --agent "oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3" --games 4 --seed 36800 --threads 1 --set adversary.difficulty=2
echo CPUPAIRSDONE

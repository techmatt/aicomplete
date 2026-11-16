#!/bin/sh
# 251: OLD against NEW on the same games, one thread, in turn (per_decision.py): the judge, greedy, the
# real oracle at d2. Usage: sh pairs.sh OLD_ROOT NEW_ROOT
OLD=$1
NEW=$2
PD=scripts/bench/per_decision.py
REAL="oracle:strategy=0,worlds=4,alts=3,budget=32,answers=1,truth=1,w.built=4,w.built_cost=0.5,w.clock=0,w.energy_cap=0,w.hand=1.5,w.hand_cost=0.1,w.horizon=0,w.insight_income=0,w.movement=0,w.packet_keep=1,w.production=0,w.strat_contents=0,w.table=0,w.tag_hand=0,w.purge=3.6,w.last_node=120"
echo "== strategist"
python $PD "$OLD" "$NEW" --agent strategist --games 12 --first 36800 --pairs 2
echo "== greedy"
python $PD "$OLD" "$NEW" --agent greedy --games 12 --first 36800 --pairs 2
echo "== real oracle d2"
python $PD "$OLD" "$NEW" --agent "$REAL" --games 2 --first 36800 --pairs 1 --set adversary.difficulty=2
echo PAIRSDONE

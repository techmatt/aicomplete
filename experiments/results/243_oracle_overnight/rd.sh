#!/bin/sh
# rd.sh FIRST CHUNK THREADS: the wide read in chunks of CHUNK seeds from FIRST (strategist + the oracle with
# restarts, three pairs, d0-d3) on the rd/ exe until out/243_oracle_overnight/rd.stop exists.
cd "$(dirname "$0")/../.." || exit 1
O=out/243_oracle_overnight
first=$1; chunk=$2; th=$3
k=0
while [ ! -e $O/rd.stop ]; do
  s=$((first + k * chunk))
  name="rd-$s"
  python $O/mkspec.py "$name" "$s" "$chunk" 0,1,2,3 strategist "oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3"
  (cd $O/rd && AIC_ORACLE_TRACE=1 nice -n 10 ./aic-sim.exe experiment "../$name.json" --threads "$th" --out "../$name" 2>&1 >"../$name.log" | grep -a "OTRY\|games\|time:" > "../$name.trace")
  echo "CHUNK $name done $(date +%H:%M)"
  k=$((k + 1))
done
echo "RD-STOPPED"

#!/bin/sh
# 155: runs one stage's three pair specs in turn (sh run155.sh STAGE [THREADS]), from the repo root, on
# out/sim155.exe (a copy of the build), each into out/s155-STAGE-PAIR/; writes out/s155-STAGE.done last.
STAGE=$1
T=${2:-6}
for P in bp br pr; do
  nice -n 10 ${SIM:-out/sim155.exe} experiment out/s155-$STAGE-$P.json --threads $T --out out/s155-$STAGE-$P > out/s155-$STAGE-$P.log 2>&1
  echo "exit $?" >> out/s155-$STAGE-$P.log
done
echo done > out/s155-$STAGE.done

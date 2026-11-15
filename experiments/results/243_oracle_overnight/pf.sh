#!/bin/sh
# pf.sh PAR: runs every spec in pf/jobs.txt not yet run, PAR at a time, one thread each, on the committed exe in pfx/
cd "$(dirname "$0")/pfx" || exit 1
here=$(pwd)
xargs -P "${1:-4}" -I{} sh -c 'cd "'"$here"'" && [ -e "../pf/{}/results.json" ] || nice -n 10 ./aic-sim.exe experiment "../pf/{}.json" --threads 1 --out "../pf/{}" > "../pf/{}.log" 2>&1' < ../pf/jobs.txt
echo PF-DONE

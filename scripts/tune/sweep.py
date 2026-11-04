"""Tuning sweep driver: runs `aic-sim run --json` once per (variant, agent) and
prints one compact row each. Every result is appended to out/tune/log.jsonl.

    python scripts/tune/sweep.py SPEC.json
    python scripts/tune/sweep.py --config configs/baseline.json --agents random,greedy \
        --games 48 --seed 0 --var "supply8=adversary.supply_per_country=8" --var "base="

A variant is `name=set1;set2;...` (each a `--set K=V`). A SPEC file holds
{"config", "agents", "games", "seed", "common": [sets], "variants": {name: [sets]}}.
"""

import argparse
import json
import os
import subprocess
import sys
import time

SIM = os.environ.get("AIC_SIM", os.path.join("target", "release", "aic-sim"))

# A redirected stdout on Windows is cp1252: its ± and the like would not encode (020).
sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def run(config, agent, games, seed, sets):
    cmd = [SIM, "run", config, "--games", str(games), "--seed", str(seed),
           "--agent", agent, "--json", "--max-actions", "20000"]
    for s in sets:
        cmd += ["--set", s]
    t = time.time()
    p = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
    if p.returncode != 0:
        raise SystemExit(f"aic-sim failed ({p.returncode}): {' '.join(cmd)}\n{p.stderr[-2000:]}")
    d = json.loads(p.stdout)
    return d["results"][0], time.time() - t


def row(name, agent, r, secs):
    c = r["counters"]
    ended = " ".join(f"{k}:{v}" for k, v in sorted(r["ended"].items()))
    return (f"{name:<22} {agent:<10} win {100 * r['win_rate']:5.1f}% ±{100 * r['win_rate_se']:4.1f} "
            f"rnd {r['rounds_mean']:4.2f} ops {r['adversary_ops_per_round']:5.1f} "
            f"corr {c.get('adversary.corruptions', 0):4.1f} dest {c.get('board.destroyed', 0):4.1f} "
            f"[{ended}] {secs:5.0f}s")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("spec", nargs="?")
    ap.add_argument("--config", default="configs/baseline.json")
    ap.add_argument("--agents", default="random,greedy")
    ap.add_argument("--games", type=int, default=48)
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--common", action="append", default=[])
    ap.add_argument("--var", action="append", default=[])
    ap.add_argument("--tag", default="")
    a = ap.parse_args()

    if a.spec:
        with open(a.spec, encoding="utf-8") as f:
            spec = json.load(f)
        config = spec.get("config", a.config)
        agents = spec.get("agents", a.agents.split(","))
        games = spec.get("games", a.games)
        seed = spec.get("seed", a.seed)
        common = spec.get("common", [])
        variants = list(spec["variants"].items())
    else:
        config, games, seed, common = a.config, a.games, a.seed, a.common
        agents = a.agents.split(";") if ";" in a.agents else a.agents.split(",")
        variants = []
        for v in a.var or ["base="]:
            name, _, sets = v.partition("=")
            variants.append((name, [s for s in sets.split(";") if s]))

    os.makedirs(os.path.join("out", "tune"), exist_ok=True)
    log = open(os.path.join("out", "tune", "log.jsonl"), "a", encoding="utf-8", newline="\n")
    for name, sets in variants:
        for agent in agents:
            r, secs = run(config, agent, games, seed, common + sets)
            print(row(name, agent, r, secs), flush=True)
            log.write(json.dumps({"tag": a.tag, "name": name, "agent": agent, "config": config,
                                  "games": games, "seed": seed, "sets": common + sets,
                                  "win_rate": r["win_rate"], "rounds": r["rounds_mean"],
                                  "ended": r["ended"], "end_round": r.get("end_round"),
                                  "counters": r["counters"], "secs": secs,
                                  "time": time.strftime("%H:%M")}) + "\n")
            log.flush()


if __name__ == "__main__":
    sys.exit(main())

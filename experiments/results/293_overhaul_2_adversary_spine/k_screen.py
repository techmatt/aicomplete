"""293: the K screen's table from `farm-seeds ... spine293` (experiments/archive/farm/spine293.json: three variants an arm).

    python k_screen.py OUT.jsonl
"""
import collections
import json
import sys

ARMS = ["K 3, start 0 (asked)", "K 4", "K 5", "K 3, start 1", "K 2", "K 1", "K 2, start 1"]
PAIRS = ["bp", "br", "pr"]


def main():
    with open(sys.argv[1], encoding="utf-8") as f:
        rows = [r for r in (json.loads(line) for line in f if line.strip()) if "cell" in r]
    print("| Arm | Games | Wins (bp / br / pr) | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Research cubes made: mean / max | Agents on board, peak: mean / max |")
    print("|---|---|---|---|---|---|---|---|")
    for a, name in enumerate(ARMS):
        gs = [r for r in rows if r["cell"] // 3 == a]
        if not gs:
            continue
        n = len(gs)
        wins = [sum(r["won"] for r in gs if r["cell"] % 3 == v) for v in range(3)]
        reasons = collections.Counter(r["reason"] for r in gs if not r["won"])
        slots = collections.Counter(r["counters"].get("adversary.research_fired", 0) for r in gs)
        rounds = [r["rounds"] for r in gs]
        cubes = [r["counters"].get("adversary.research_made", 0) for r in gs]
        peak = [max([v for k, v in r["counters"].items() if k.startswith("adversary.ambassadors_r")] or [0]) for r in gs]
        how = ", ".join(f"{k} {v}" for k, v in sorted(reasons.items())) or "-"
        slot = " ".join(f"{k}:{v}" for k, v in sorted(slots.items()))
        print(f"| {name} | {n} | {sum(wins)} ({' / '.join(map(str, wins))}) | {sum(rounds) / n:.1f} ({min(rounds)}-{max(rounds)}) | {how} | {slot} "
              f"| {sum(cubes) / n:.1f} / {max(cubes)} | {sum(peak) / n:.1f} / {max(peak)} |")


if __name__ == "__main__":
    main()

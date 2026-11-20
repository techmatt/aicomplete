"""298: games with a project troop by round 3, by which troop project could have made it (a read's games file).

    python early_troops.py LABEL=FILE [LABEL=FILE ...]

No counter names the project that resolved in a round, so a game counts toward Martial Law when its project troops
came with no botnet or factory troops (Botnet Conscripts, War Plants) in the game, and toward those otherwise.
"""
import collections
import json
import sys


def main():
    print("| Read | Level | Games | Project troop by r3 | of which Martial Law only | Botnet Conscripts or War Plants |")
    print("|---|---|---|---|---|---|")
    for arg in sys.argv[1:]:
        label, path = arg.split("=", 1)
        n, early, ml, other = (collections.Counter() for _ in range(4))
        for line in open(path, encoding="utf-8"):
            g = json.loads(line)
            if "cell" not in g:
                continue
            k, d = g["counters"], g["difficulty"]
            n[d] += 1
            if any(k.get(f"map.troops_placed.project.r0{r}", 0) for r in (1, 2, 3)):
                early[d] += 1
                if k.get("adversary.botnet_troops", 0) + k.get("adversary.factory_troops", 0) == 0:
                    ml[d] += 1
                else:
                    other[d] += 1
        for d in sorted(n):
            print(f"| {label} | d{d} | {n[d]} | {early[d]} | {ml[d]} | {other[d]} |")


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")
    main()

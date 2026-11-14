"""236: what the targeted powers and standard actions find, and how often each is used, new against base.

    python experiments/results/236_target_followups/finds.py NEW.jsonl BASE.jsonl [--seats]

Per side, pooled over agents, levels and pairs: uses a seat-game of each changed power (over its board's seats) and
of the standard actions; Remove agents' agents removed a use beside what reach held (the new side's tallies), and
Fortify's hardening found beside what reach held (`effects.harden_found.near.1` / `effects.harden_in_reach.near.1`).
"""
import json
import sys
from collections import defaultdict

POWERS = [
    ("builder", "fortify"), ("protector", "bulwark"), ("protector", "debrief"), ("protector", "patrol"),
    ("protector", "interdict"), ("protector", "reclaim"), ("researcher", "field_station"), ("researcher", "audit"),
    ("researcher", "disrupt"),
]
STD = ["remove", "troop", "lift", "station", "move", "claim", "liberate", "botnet", "buy_energy"]
BOARDS = {"bp": ["builder", "protector"], "br": ["builder", "researcher"], "pr": ["protector", "researcher"]}
TALLIES = [
    "board.remove_agents_removed", "board.remove_agents_in_reach",
    "effects.harden_found.near.1", "effects.harden_in_reach.near.1",
    "effects.targets", "effects.targets_missed",
]


def read(path):
    n = 0
    seats = defaultdict(int)
    sums = defaultdict(float)
    with open(path, encoding="utf-8") as f:
        for line in f:
            d = json.loads(line)
            if "counters" not in d:
                continue
            n += 1
            # Cells are variant-major (bp, br, pr), each over 4 levels x 2 agents.
            pair = ["bp", "br", "pr"][d["cell"] // 8]
            for b in BOARDS[pair]:
                seats[b] += 1
            c = d["counters"]
            for k, v in c.items():
                if k.startswith("ai.power.") and k.count(".") == 3:
                    sums[k] += v
                elif k.startswith("board.std.") and k.count(".") == 2:
                    sums[k] += v
                elif k in TALLIES:
                    sums[k] += v
    return n, seats, sums


def main():
    new, base = sys.argv[1], sys.argv[2]
    rows = []
    for label, path in (("new", new), ("base", base)):
        n, seats, s = read(path)
        rows.append((label, n, seats, s))
    print("| side | games | " + " | ".join(f"{p}" for _, p in POWERS) + " |")
    print("|---" * (len(POWERS) + 2) + "|")
    for label, n, seats, s in rows:
        cells = []
        for b, p in POWERS:
            k = f"ai.power.{b}.{p}"
            cells.append(f"{s[k] / max(seats[b], 1):.2f}")
        print(f"| {label} | {n} | " + " | ".join(cells) + " |")
    print()
    print("Uses a seat-game (a power: over its board's seats).")
    print()
    print("| side | " + " | ".join(STD) + " |")
    print("|---" * (len(STD) + 1) + "|")
    for label, n, seats, s in rows:
        per_seat = 2 * n
        print(f"| {label} | " + " | ".join(f"{s['board.std.' + a] / per_seat:.3f}" for a in STD) + " |")
    print()
    print("Standard actions a seat-game (2 seats a game).")
    print()
    label, n, seats, s = rows[0]
    uses = s["board.std.remove"]
    if uses:
        print(f"Remove agents ({label}): {uses:.0f} uses; removed {s['board.remove_agents_removed'] / uses:.2f} a use, reach held "
              f"{s['board.remove_agents_in_reach'] / uses:.2f} (find share {s['board.remove_agents_removed'] / max(s['board.remove_agents_in_reach'], 1):.2f})")
    f_uses = s["ai.power.builder.fortify"]
    if f_uses:
        print(f"Fortify ({label}): {f_uses:.0f} uses; hardened {s['effects.harden_found.near.1'] / f_uses:.2f} a use, reach held "
              f"{s['effects.harden_in_reach.near.1'] / f_uses:.2f}")
    print(f"targets ({label}): {s['effects.targets'] / n:.2f} a game, missed {s['effects.targets_missed'] / n:.2f}")


if __name__ == "__main__":
    main()

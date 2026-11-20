"""301: the liberate cards' plays a game, and liberations by card and by the action, per read.

    python caps_read.py LABEL=FILE [LABEL=FILE ...]

Each FILE a farmed read (one game a line). Prints a Markdown table: each liberate card's plays per 100 games,
then cities liberated by the action and by card per game, and stations liberated by the action per game.
"""
import json
import sys

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
CARDS = ["citywide_microgrid", "power_to_the_people", "hostile_buyout", "root_access", "peacekeeping_mission",
         "forensic_audit", "popular_uprising", "nuclear_option", "great_liberation"]
reads = []
for a in sys.argv[1:]:
    label, _, path = a.partition("=")
    with open(path, encoding="utf-8") as f:
        gs = [g for g in (json.loads(l) for l in f if l.strip()) if "cell" in g]
    reads.append((label, gs))


def per(gs, key, scale=1.0):
    return scale * sum(g["counters"].get(key, 0) for g in gs) / len(gs)


print("| Per 100 games | " + " | ".join(l for l, _ in reads) + " |")
print("|---|" + "---|" * len(reads))
for c in CARDS:
    print(f"| {c} played | " + " | ".join(f"{per(gs, 'cards.played.' + c, 100):.1f}" for _, gs in reads) + " |")
for key, name in [("board.std.liberate", "Liberate actions"), ("board.liberated_card.city", "cities freed by card"),
                  ("board.liberated.city", "cities liberated"), ("effects.cleansed", "sites cleansed by card")]:
    print(f"| {name} | " + " | ".join(f"{per(gs, key, 100):.0f}" for _, gs in reads) + " |")

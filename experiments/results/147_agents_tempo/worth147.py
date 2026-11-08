"""147 (amendment 6307): a node's and a troop's worth measured in play, from a run's games.jsonl.

    python experiments/results/147_agents_tempo/worth147.py DIR [DIR ...]

Node, the revealed price: the table credits seats pay on the powers that place nodes (`uses`, `own:power:<name>` for
each AI-board power with a `place_node` op; the track step's energy is paid with it), over the nodes those powers
place (`board.node_placed` less what cards place is not split out, so the price is over all nodes placed: a floor).
Troop, what one costs in play: what troops destroy (stations struck, factories lost in war, nodes lost to troops or
war) at the table's rows, plus the Discord war sways at `rates.sway`, over the adversary troops placed.
"""

import json
import sys
from collections import Counter
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
ROOT = Path(__file__).resolve().parents[3]
VALUES = json.loads((ROOT / "configs/agents/values.json").read_text(encoding="utf-8"))
RES = ["credits", "chips", "energy", "antimatter", "insight", "research_cubes", "ambassadors"]


def node_powers():
    ais = json.loads((ROOT / "content/ais.json").read_text(encoding="utf-8"))
    names = set()

    def walk(x, name=None):
        if isinstance(x, dict):
            n = x.get("name", name)
            if x.get("op") == "place_node":
                names.add(name)
            for v in x.values():
                walk(v, n if "cost" in x or "effects" in x or "effect" in x else name)
        elif isinstance(x, list):
            for v in x:
                walk(v, name)

    walk(ais)
    return {n for n in names if n}


def price(vec):
    rates = VALUES["resources"]
    return sum(n * rates.get(RES[i], 0) for i, n in enumerate(vec[:len(RES)]))


def main():
    powers = node_powers()
    paid, placed, games = 0.0, 0, 0
    c = Counter()
    for d in sys.argv[1:]:
        with open(Path(d) / "games.jsonl", encoding="utf-8") as f:
            f.readline()
            for line in f:
                g = json.loads(line)
                games += 1
                for u in g["uses"]:
                    for k, v in u.items():
                        if k.startswith("own:power:") and k[len("own:power:"):] in powers:
                            paid += price(v)
                placed += g["counters"].get("board.node_placed", 0)
                for k, v in g["counters"].items():
                    c[k] += v
    print(f"{games} games; node powers: {', '.join(sorted(powers))}")
    print(f"node: {paid / max(1, placed):.2f} credits paid a node placed ({placed / max(1, games):.1f} placed a game)")
    # The `troops` row prices an adversary troop a threat (or event) lands; front units (corp troops
    # built, project troops) fight the war and are priced by their own rows.
    troops = c["threats.troops"]
    rows = VALUES.get("ops", {})

    def row(k, d):
        return rows.get(k, {}).get("credits", d) if isinstance(rows.get(k), dict) else d

    lost_nodes = sum(v for k, v in c.items() if k.startswith("board.node_lost.") and "troop" in k)
    node = paid / max(1, placed)
    destroyed = {
        "stations struck": (c["board.troop_destroyed.station"], row("build_station", 10)),
        "nodes lost to troops": (lost_nodes, node),
    }
    total = sum(n * v for n, v in destroyed.values())
    print(f"troops threats placed: {troops} ({troops / max(1, games):.1f} a game; front units built {c['board.troops_built']}, "
          f"projects {c['adversary.project_troops']}, not counted)")
    for k, (n, v) in destroyed.items():
        print(f"  {k}: {n} ({n / max(1, games):.2f} a game) at {v}")
    print(f"troop: {total / max(1, troops):.2f} credits of harm a troop placed")


if __name__ == "__main__":
    main()

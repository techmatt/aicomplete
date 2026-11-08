"""148's card read: per difficulty (seat pairs pooled), each named card's share of games offered, kept and played,
its Action uses a game where played, and the measures items 1-5 and 7 ask for.

    python experiments/results/148_card_pass_r62/cards148.py DIR [DIR ...] [--cards a,b,c] [--variants bp,br,pr] [--config N]

`--config N`: only the spec's N-th config (0-based; a spec listing two configs plays each on every cell).

Also: claims by card (`board.claimed_by.card.station`) and by the Claim action, adversary ambassadors canceled by a
card claim, the Auto Counter's cut (`threats.auto_countered`, `threats.countered_by.standing`), and how the d-level's
losses ended (deck_empty is the round cap: the event deck ran out).
"""

import json
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "147_agents_tempo"))
from read147 import load  # noqa: E402

sys.stdout.reconfigure(encoding="utf-8", newline="\n")

CARDS = ["exascale_training_cluster", "microgrid_lab_compact", "power_purchase_agreement", "autonomous_defense_grid",
         "market_stabilization_board"]


def main():
    a = sys.argv[1:]
    cards = CARDS
    variants = None
    if "--cards" in a:
        cards = a[a.index("--cards") + 1].split(",")
    if "--variants" in a:
        variants = set(a[a.index("--variants") + 1].split(","))
    config = int(a[a.index("--config") + 1]) if "--config" in a else None
    dirs = [x for i, x in enumerate(a) if not x.startswith("--") and (i == 0 or a[i - 1] not in ("--cards", "--variants", "--config"))]
    by = defaultdict(list)
    for d in dirs:
        with open(Path(d) / "games.jsonl", encoding="utf-8") as f:
            spec = json.loads(f.readline())["spec"]
        per = len(spec.get("variants") or [1]) * len(spec.get("difficulties", [0])) * len(spec["agents"])
        for g in load(d):
            if variants and g["_pair"] not in variants:
                continue
            if config is not None and g["cell"] // per != config:
                continue
            by[(g["_diff"], g["_agent"])].append(g)
    for (diff, agent), gs in sorted(by.items()):
        n = len(gs)
        wins = sum(g["won"] for g in gs)
        print(f"## d{diff} {agent}: {n} games, win {100 * wins / n:.1f}")
        for c in cards:
            st = [g["cards"].get(c, {}) for g in gs]
            off = sum(1 for s in st if s.get("offered", 0) or s.get("drawn", 0))
            kept = sum(1 for s in st if s.get("kept", 0))
            played = sum(1 for s in st if s.get("played", 0))
            used = [g["counters"].get(f"cards.used.{c}", 0) for g in gs if g["cards"].get(c, {}).get("played", 0)]
            skipped = sum(g["counters"].get(f"actions.skipped.{s}.{c}", 0) for g in gs for s in range(4))
            blocked = sum(g["counters"].get(f"actions.blocked.{s}.{c}", 0) for g in gs for s in range(4))
            print(f"  {c}: offered {100 * off / n:.0f}%, kept {100 * kept / n:.0f}%, played {100 * played / n:.0f}%"
                  + (f", uses {sum(used) / len(used):.2f} a game played" if used else "")
                  + (f", skipped {skipped / n:.2f} blocked {blocked / n:.2f} a game" if skipped or blocked else ""))
        cnt = lambda k: sum(g["counters"].get(k, 0) for g in gs) / n
        print(f"  claims a game: card {cnt('board.claimed_by.card.station'):.2f}, Claim action stations "
              f"{cnt('board.claimed_by.ambassadors.station'):.2f}, all stations {cnt('board.claimed.station'):.2f}; canceled {cnt('board.canceled'):.2f}")
        print(f"  auto counter: cut {cnt('threats.auto_countered'):.2f} a game, countered by standing {cnt('threats.countered_by.standing'):.2f}")
        losses = [g for g in gs if not g["won"]]
        ends = Counter(g["reason"] for g in losses)
        cap = ends.get("deck_empty", 0)
        print(f"  losses {len(losses)}: {dict(ends.most_common())}; round cap {cap} ({100 * cap / max(1, len(losses)):.0f}% of losses)")


if __name__ == "__main__":
    main()

"""154: classify the round-cap losses from the end-game probe (`"endgame"` in a spec, crates/aic-sim/src/endgame.rs).

python endgame154.py DIR[,DIR...] [--list]

Per difficulty, for each game lost to the round cap: the end state (what the purge still needed at the game's own
Unity level, by kind; Unity cards drawn; Discord cards left), a feasibility bound from the snapshot (the start of the
third-last round), and the stronger player's replays from that snapshot.

The bound is generous to the players (it can only say "can't win" when no team could have):
- the purge route, at the deeper of the snapshot's and the end's level: ambassadors needed (each corrupted city's
  price at the snapshot's stage, plus a third of the adversary ambassadors on it, cleared first by Remove at 3 a use;
  with ambassadors counting, a Remove use per 3 elsewhere and the
  `free_leaves` landing back on each liberated city), against ambassadors held, on the board, bought with every credit
  held and earned to the end (20 each) and gained at the game's own rate; liberate cards in hand or play each cover a
  city outright; botnets (if counted) against botnet cards (an Action each round) and Decommission still in the deck;
  troops (if counted) against troop cards at 4 a use each round;
- the Unity-deck route: Sway still needed for the 9th card, against 5 a liberation the ambassadors above afford,
  2 a corruption marker in reach Cleared, 3 a Unity card still to come from tier 1 (Volunteer Corps), and the game's
  own Unity rate over the rounds left, doubled.
A game is "can't win" when both routes fail; the stronger player's replays settle the rest: a replay win is an agent
mistake, a replay loss is tight (recorded as a design mistake).
"""
import json
import math
import sys
from collections import Counter, defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
UNITY_CARDS = 9
AMB_CREDITS = 20


def counts(level):
    """Which kinds count at a level's steps (purge_at 3, 6, 6: steps 0, 1 or 3)."""
    return {"ambassadors": level < 1, "troops": level < 2, "botnets": level < 3}


def load(dirs):
    for d in dirs.split(","):
        res = json.load(open(f"{d}/results.json", encoding="utf-8"))
        cells = {c["cell"]: c for c in res["cells"]}
        with open(f"{d}/games.jsonl", encoding="utf-8") as f:
            f.readline()
            for line in f:
                g = json.loads(line)
                yield cells[g["cell"]], g


def need_end(e):
    """What the purge still needed at the end, at the game's own level, by kind."""
    c = counts(e["level"])
    p = e["end"]
    return {"cities": p["cities"], "botnets": p["botnets"] if c["botnets"] else 0,
            "troops": p["troops"] if c["troops"] else 0, "ambassadors": p["ambassadors"] if c["ambassadors"] else 0}


def bound(e, g):
    rounds_left = e["end_round"] - e["snap_round"] + 1
    played = max(g["rounds"], 1)
    t, p = e["team"], e["snap"]
    level = max(e["snap_level"], e["level"])
    c = counts(level)
    gained = g.get("gained") or []
    amb_rate = sum(s[7] for s in gained) / played
    credit_rate = sum(s[0] for s in gained) / played
    ambassadors = (t["ambassadors_supply"] + t["ambassadors_board"] + amb_rate * rounds_left
                   + (t["credits"] + credit_rate * rounds_left) / AMB_CREDITS)
    # The purge route: the adversary ambassadors on a corrupted city go first by Remove (3 a use, one ambassador),
    # then the city liberates at its printed price; liberate cards cover cities outright.
    on = p["ambassadors_on_corrupted"]
    lib_need = (p["liberate_cost"] - on) + on / 3
    if p["cities"] and t["liberate_cards"]:
        per_city = lib_need / p["cities"]
        lib_need = max(0.0, lib_need - per_city * t["liberate_cards"])
    amb_need = lib_need + ((p["remove_actions"] + p["remove_after_liberate"]) if c["ambassadors"] else 0)
    short = {}
    if amb_need > ambassadors:
        short["ambassadors"] = amb_need - ambassadors
    if c["botnets"] and p["botnets"] > t["botnet_cards"] * rounds_left + t["unity_botnet_left"]:
        short["botnets"] = p["botnets"] - t["botnet_cards"] * rounds_left - t["unity_botnet_left"]
    if c["troops"] and p["troops"] > t["troop_cards"] * 4 * rounds_left:
        short["troops"] = p["troops"] - t["troop_cards"] * 4 * rounds_left
    purge_ok = not short
    # The Unity-deck route.
    sway_need = (UNITY_CARDS - t["unity_drawn"]) * t["unity_capacity"] - t["unity_track"]
    unity_rate = g["margin"]["unity_tips"] * t["unity_capacity"] / played
    liberations = min(p["cities"], ambassadors / max(p["liberate_cost"] / max(p["cities"], 1), 5) + t["liberate_cards"])
    sway = 5 * liberations + 2 * p["markers_in_reach"] + 3 + 2 * unity_rate * rounds_left
    unity_ok = sway >= sway_need
    return purge_ok, unity_ok, short, sway_need - sway, rounds_left


def main():
    dirs = sys.argv[1]
    listing = "--list" in sys.argv
    by = defaultdict(list)
    for cell, g in load(dirs):
        if g["won"] or g["reason"] != "deck_empty":
            continue
        e = g.get("endgame")
        if not e:
            continue
        by[cell["difficulty"]].append((cell, g, e))
    print("## End state at the round cap (what the purge still needed, at the game's own level)\n")
    print("| Level | Cap losses | Unity drawn (mean; 0-2 / 3-5 / 6-8) | Discord left (mean) | Purge left (median) "
          "| Corrupted cities (mean) | Troops counted (mean, games) | Botnets counted (mean) | Adv. ambassadors counted (mean) "
          "| Near misses (purge left ≤ 2, or 8 Unity drawn) |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for d in sorted(by):
        rows = by[d]
        n = len(rows)
        drawn = [e["unity_drawn"] for _, _, e in rows]
        bands = Counter("0-2" if x < 3 else "3-5" if x < 6 else "6-8" for x in drawn)
        needs = [need_end(e) for _, _, e in rows]
        left = sorted(sum(x.values()) for x in needs)
        near = sum(1 for x, e in zip(needs, [r[2] for r in rows]) if sum(x.values()) <= 2 or e["unity_drawn"] >= 8)
        troops_games = sum(1 for x in needs if x["troops"])
        print(f"| d{d} | {n} | {sum(drawn) / n:.1f}; {bands['0-2']} / {bands['3-5']} / {bands['6-8']} | "
              f"{sum(e['discord_left'] for _, _, e in rows) / n:.1f} | {left[n // 2]} | "
              f"{sum(x['cities'] for x in needs) / n:.1f} | {sum(x['troops'] for x in needs) / n:.0f} ({troops_games}) | "
              f"{sum(x['botnets'] for x in needs) / n:.1f} | {sum(x['ambassadors'] for x in needs) / n:.1f} | {near} |")
    print("\n## Bound and replay, by level\n")
    print("| Level | Cap losses | Can't win (bound) | Could win | of them replay won (agent mistake) | replay lost (tight) "
          "| Purge route short of (games) | Ambassadors short (median, purge route) | Sway short (median, Unity route) |")
    print("|---|---|---|---|---|---|---|---|---|")
    totals = Counter()
    for d in sorted(by):
        rows = by[d]
        cant = could = agent = tight = 0
        shorts = Counter()
        amb_short, sway_short = [], []
        for cell, g, e in rows:
            purge_ok, unity_ok, short, sway_gap, _ = bound(e, g)
            for k in short:
                shorts[k] += 1
            amb_short.append(short.get("ambassadors", 0))
            sway_short.append(max(sway_gap, 0))
            if not purge_ok and not unity_ok:
                cant += 1
                verdict = "can't win"
            else:
                could += 1
                if e["replays"] and e["replay_wins"] > 0:
                    agent += 1
                    verdict = "agent mistake"
                elif e["replays"]:
                    tight += 1
                    verdict = "tight"
                else:
                    verdict = "unsettled"
            if listing:
                print(f"<!-- d{d} {cell['variant']} {g['seed']}: {verdict}; short {dict(short)}; sway gap {sway_gap:.1f}; "
                      f"replays {e['replay_wins']}/{e['replays']} {e['replay_reasons']} {e['replay_purge']} -->")
        totals.update({"cant": cant, "agent": agent, "tight": tight})
        sk = ", ".join(f"{k} {v}" for k, v in shorts.most_common())
        print(f"| d{d} | {len(rows)} | {cant} | {could} | {agent} | {tight} | {sk} | "
              f"{sorted(amb_short)[len(amb_short) // 2]:.1f} | {sorted(sway_short)[len(sway_short) // 2]:.1f} |")
    print(f"\nDesign mistakes (can't win + tight): {totals['cant'] + totals['tight']}; agent mistakes: {totals['agent']}.")


if __name__ == "__main__":
    main()

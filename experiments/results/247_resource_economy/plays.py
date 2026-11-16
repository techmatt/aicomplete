"""Share of games each named card was kept and played in, two reads side by side.
usage: python plays.py NEW.jsonl OLD.jsonl CARD [CARD ...]"""
import json, sys
sys.stdout.reconfigure(encoding='utf-8', newline='\n')
new, old, cards = sys.argv[1], sys.argv[2], sys.argv[3:]


def read(path):
    n = 0
    kept = {c: 0 for c in cards}
    played = {c: 0 for c in cards}
    with open(path, encoding='utf-8') as f:
        for l in f:
            if not l.startswith('{"cell"'):
                continue
            g = json.loads(l)
            n += 1
            for c in cards:
                cc = g['cards'].get(c, {})
                kept[c] += cc.get('kept', 0) > 0
                played[c] += cc.get('played', 0) > 0
    return n, kept, played


nn, kn, pn = read(new)
no, ko, po = read(old)
print(f"| card | kept % old -> new | played % old -> new |  ({no} / {nn} games)")
print("|---|---|---|")
for c in cards:
    print(f"| {c} | {100 * ko[c] / no:.1f} -> {100 * kn[c] / nn:.1f} | {100 * po[c] / no:.1f} -> {100 * pn[c] / nn:.1f} |")

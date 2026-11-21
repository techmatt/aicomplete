"""313: the judge's `choose` misses against the oracle (`AIC_AGREE_SHOW=choose` lines), by what the recorded and the
picked option were: a track uncover (by track), a hex, a card, or the text's first words.

    python choose_kinds.py SHOW.txt [GAMES]
"""
import collections
import re
import sys


def cat(o):
    if " track:" in o:
        return o.split(":")[0]
    if "#" in o:
        return "hex"
    if re.fullmatch(r"[a-z0-9_]+", o):
        return "card"
    return " ".join(o.split()[:3])


def main():
    c = collections.Counter()
    games = int(sys.argv[2]) if len(sys.argv) > 2 else 1
    for line in open(sys.argv[1], encoding="utf-8"):
        m = re.search(r'recorded Choose \{ seat: \d+, option: "([^"]*)".*picked Choose \{ seat: \d+, option: "([^"]*)"', line)
        if not m:
            m2 = re.search(r"recorded (\w+).*picked (\w+)", line)
            c[("(" + (m2.group(1) if m2 else "?") + ")", "(" + (m2.group(2) if m2 else "?") + ")")] += 1
            continue
        c[(cat(m.group(1)), cat(m.group(2)))] += 1
    for (r, p), v in c.most_common(20):
        print(f"{v / games:6.2f} a game  recorded {r!r:40} picked {p!r}")


if __name__ == "__main__":
    main()

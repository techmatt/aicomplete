"""Count the rulebook's Part 1 words (sections 1-14, "# Part 1" up to "# Part 2"), skipping Sources lines."""
import re
import sys

WORD = re.compile(r"\b[\w'-]+\b")


def count(path):
    with open(path, encoding="utf-8") as f:
        text = f.read()
    start = text.index("\n# Part 1")
    end = text.index("\n# Part 2")
    part = text[start:end]
    lines = [ln for ln in part.splitlines() if not ln.lstrip().startswith(("Sources", "*Sources"))]
    part1 = sum(len(WORD.findall(ln)) for ln in lines)
    whole = len(WORD.findall(text))
    return part1, whole


for p in sys.argv[1:]:
    p1, whole = count(p)
    print(f"{p}: Part 1 {p1} words, whole page {whole}")

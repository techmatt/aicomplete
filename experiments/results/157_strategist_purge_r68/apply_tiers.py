"""Applies tier3.overlay's alignment rows to content/alignment.json, keeping its hand layout."""
import json
import re
import sys

OVL, PATH = sys.argv[1], "content/alignment.json"
text = open(PATH, encoding="utf-8").read()
lines = text.split("\n")
for raw in open(OVL, encoding="utf-8"):
    raw = raw.strip()
    if not raw or raw.startswith("#"):
        continue
    key, val = raw.split("=", 1)
    _, cid, _, idx = key.split(".")
    idx = int(idx)
    tier = json.loads(val)
    start = next(i for i, l in enumerate(lines) if l.strip() == f'"id": "{cid}",')
    tiers = [i for i in range(start, len(lines)) if lines[i].startswith('   {"text": ')]
    i = tiers[idx]
    end_tail = lines[i + 1][lines[i + 1].rindex("]") + 1:]  # "}" or "},"
    lines[i] = '   {"text": ' + json.dumps(tier["text"], ensure_ascii=False) + ","
    lines[i + 1] = '    "effects": ' + json.dumps(tier["effects"], ensure_ascii=False) + end_tail
out = "\n".join(lines)
json.loads(out)
with open(PATH, "w", encoding="utf-8", newline="\n") as f:
    f.write(out)

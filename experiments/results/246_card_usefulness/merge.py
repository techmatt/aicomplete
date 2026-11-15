"""246: merge card-value reads played by one binary on the same seeds into one stream (header from the first, each
later file's cells shifted past the variants before it), so cardvalue.py can pair an arm from one with a blank from
another.

python merge.py OUT IN1 IN2 ...
"""
import json
import sys

out, ins = sys.argv[1], sys.argv[2:]
heads, bodies = [], []
for p in ins:
    fh = open(p, encoding="utf-8")
    heads.append(json.loads(fh.readline()))
    bodies.append([l for l in fh if l.strip()])
spec = heads[0]["spec"]
for h in heads[1:]:
    assert h["spec"]["agents"] == spec["agents"] and h["spec"]["difficulties"] == spec["difficulties"], "agents and levels must match"
per = len(spec["agents"]) * len(spec["difficulties"])
head = dict(heads[0])
head["spec"] = dict(spec)
head["spec"]["variants"] = [v for h in heads for v in h["spec"]["variants"]]
with open(out, "w", encoding="utf-8", newline="\n") as w:
    w.write(json.dumps(head) + "\n")
    shift = 0
    for h, body in zip(heads, bodies):
        for l in body:
            g = json.loads(l)
            g["cell"] += shift
            w.write(json.dumps(g) + "\n")
        shift += len(h["spec"]["variants"]) * per

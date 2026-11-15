"""245: one agent's games of a read, as a read of that agent alone (header spec and cell indexes rewritten).

python one_agent.py READ/games.jsonl AGENT_INDEX OUT/games.jsonl
"""
import json
import os
import sys

src, k, out = sys.argv[1], int(sys.argv[2]), sys.argv[3]
fh = open(src, encoding="utf-8")
head = json.loads(fh.readline())
n = len(head["spec"]["agents"])
head["spec"]["agents"] = [head["spec"]["agents"][k]]
head["config_hashes"] = head["config_hashes"][k::n]
os.makedirs(os.path.dirname(out), exist_ok=True)
with open(out, "w", encoding="utf-8", newline="\n") as o:
    o.write(json.dumps(head) + "\n")
    for line in fh:
        g = json.loads(line)
        if g["cell"] % n == k:
            g["cell"] //= n
            o.write(json.dumps(g) + "\n")

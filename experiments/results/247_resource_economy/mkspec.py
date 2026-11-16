"""Write a 247 farm spec: arms (lever sets) x the three board pairs, the strategist, d0-d3.
usage: python mkspec.py NAME NOTE ARM=SET1;SET2 ...   (an arm with no sets: ARM=)"""
import json, sys
name, note, arms = sys.argv[1], sys.argv[2], sys.argv[3:]
PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]
variants = []
for arm in arms:
    a, _, sets = arm.partition('=')
    sets = [x for x in sets.split(';') if x]
    for p, ps in PAIRS:
        variants.append({"name": f"{p}-{a}" if a else p, "set": ps + sets})
spec = {
    "name": name,
    "note": note,
    "configs": ["configs/baseline.json"],
    "variants": variants,
    "difficulties": [0, 1, 2, 3],
    "agents": ["strategist"],
    "games": 64,
    "records": 0,
}
with open(f"experiments/farm/{name}.json", 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(spec, indent=1) + "\n")
print(len(variants), "variants")

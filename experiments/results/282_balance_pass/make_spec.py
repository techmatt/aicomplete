"""Write a lever x pair screen spec: python make_spec.py NAME OUT.json LEVELS AGENTS LEVER=set;set ...

Each lever becomes three variants (bp, br, pr), named <lever>/<pair>; a lever's sets are ';'-separated.
"""
import json
import sys

PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]


def main():
    name, out, levels, agents = sys.argv[1:5]
    variants = []
    for arg in sys.argv[5:]:
        lever, _, sets = arg.partition('=')
        extra = [s for s in sets.split(';') if s]
        for pair, ps in PAIRS:
            variants.append({"name": f"{lever}/{pair}", "set": ps + extra})
    spec = {
        "name": name,
        "note": "282 screen: levers x pairs (variant-major), see experiments/results/282_balance_pass/README.md",
        "configs": ["configs/baseline.json"],
        "variants": variants,
        "difficulties": [int(x) for x in levels.split(',')],
        "agents": agents.split(','),
        "games": 32,
        "first_seed": 36800,
        "records": 0,
    }
    with open(out, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(spec, f, indent=1)
        f.write('\n')
    print(out, len(variants), 'variants')


if __name__ == '__main__':
    main()

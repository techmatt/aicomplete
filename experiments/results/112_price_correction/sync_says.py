"""112: rewrite docs/agents.md's strategy lines ("- *Name.* says") from configs/agents/strategies.json.

Run from the repo root. The format is the one strategy.rs's docs test checks.
"""
import json

with open('configs/agents/strategies.json', encoding='utf-8') as f:
    strat = json.load(f)['strategies']
with open('docs/agents.md', encoding='utf-8') as f:
    lines = f.read().split('\n')
n = 0
for s in strat:
    prefix = f"- *{s['name']}.* "
    for i, line in enumerate(lines):
        if line.startswith(prefix):
            new = prefix + s['says']
            if line != new:
                lines[i] = new
                n += 1
with open('docs/agents.md', 'w', encoding='utf-8', newline='\n') as f:
    f.write('\n'.join(lines))
print(n, 'lines rewritten')

"""117: a pool scene for each of r46's four new cards; the three cut cards release theirs.

Edits tools/card-art/scenes.json in place. Run once from the repo root:
    python experiments/results/117_big_cards_and_income/assign_scenes.py
Each new card takes a drawn scene suggested for no card (its first card was retired); none was drawn for it, so
all four are listed for art in the write-up.
"""
import json
from pathlib import Path

F = Path('tools/card-art/scenes.json')
s = json.loads(F.read_text(encoding='utf-8'))

REUSE = {
    'fabrication_megaplex': 'foundry_furnace_hall',  # drawn for Foundry Core: a foundry's crucible hall
    'frontier_lab_charter': 'breakthrough_lab_flash',  # drawn for Breakthrough Lab: a hilltop laboratory
    'national_grid_buildout': 'corridor_through_forest',  # drawn for Transmission Corridor: pylons to a city
    'decapitation_strike': 'strike_team_rooftop',  # drawn for Quick Reaction Force: a team fast-roping onto a roof
}
CUT = ['plant_overdrive', 'grid_census', 'shadow_broker_feed']

for tag, group in s['scenes'].items():
    for sid, sc in list(group.items()):
        if sc.get('card') in CUT:
            card = sc.pop('card')
            group[sid] = {'was': sc.pop('was', card), **sc}

for card, sid in REUSE.items():
    tag = next(t for t, g in s['scenes'].items() if sid in g)
    sc = s['scenes'][tag][sid]
    assert 'card' not in sc, sid
    s['scenes'][tag][sid] = {'card': card, **sc}

def inner(d):
    """The file's own layout (114's assign.py): one line per scene, tag and batch row."""
    if isinstance(d, dict):
        return '{ ' + ', '.join(f'{json.dumps(k)}: {json.dumps(v, ensure_ascii=False)}' for k, v in d.items()) + ' }'
    return json.dumps(d, ensure_ascii=False)


def dump(obj):
    lines = ['{']
    keys = list(obj)
    for i, k in enumerate(keys):
        comma = ',' if i < len(keys) - 1 else ''
        v = obj[k]
        if k == 'scenes':
            lines.append('  "scenes": {')
            tags = list(v)
            for j, t in enumerate(tags):
                lines.append(f'    {json.dumps(t)}: {{')
                ids = list(v[t])
                for m, sid in enumerate(ids):
                    lines.append(f'      {json.dumps(sid)}: {inner(v[t][sid])}' + (',' if m < len(ids) - 1 else ''))
                lines.append('    }' + (',' if j < len(tags) - 1 else ''))
            lines.append('  }' + comma)
        elif k in ('tags', 'batch'):
            lines.append(f'  {json.dumps(k)}: {{')
            ids = list(v)
            for m, sid in enumerate(ids):
                lines.append(f'    {json.dumps(sid)}: {inner(v[sid])}' + (',' if m < len(ids) - 1 else ''))
            lines.append('  }' + comma)
        else:
            lines.append(f'  {json.dumps(k)}: {json.dumps(v, ensure_ascii=False)}' + comma)
    lines.append('}')
    return '\n'.join(lines) + '\n'


F.write_text(dump(s), encoding='utf-8', newline='\n')

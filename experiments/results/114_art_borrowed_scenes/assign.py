"""114: give the 11 cards that borrowed a scene (or kept an old name's) a scene of their own.

Edits tools/card-art/scenes.json in place: two reuses, nine new scenes in batch 7.
Run once from the repo root: python experiments/results/114_art_borrowed_scenes/assign.py
"""
import json
from pathlib import Path

F = Path('tools/card-art/scenes.json')
s = json.loads(F.read_text(encoding='utf-8'))
cards = {c['id']: c for c in json.loads(Path('content/cards.json').read_text(encoding='utf-8'))}

# Reused: drawn for an earlier card of the same or a near name, suggested for nothing now.
REUSE = {
    'safehouse_network': 'safehouse_stairs',  # drawn for the old Safehouse Network itself (b2)
    'preprint_server': 'preprint_exchange_board',  # drawn for Preprint Exchange (b5)
}

# New: scene id, its tag group, its subject.
NEW = {
    'bug_bounty_board': ('cyber', 'bounty_board_hunters', "A big corkboard of bounty notices in a bright open co-working loft, each notice pinned with a painted beetle sketch and a cash envelope, two hackers with laptops pulling a notice down to claim it."),
    'envoy_briefings': ('diplomacy', 'envoy_briefing_room', "A sunlit briefing room where a handful of envoys in travel coats sit at a long table with open folders and a large plain world map on the wall, a senior diplomat pointing at it, suitcases waiting by the door."),
    'supplier_network': ('industry', 'supplier_roads_converge', "Seen from a hill, small workshops and parts plants dotted across green farmland, delivery trucks on every road converging on one big modern assembly plant at the center."),
    'wartime_retooling': ('industry', 'retooled_assembly_line', "A modern car assembly line being refitted in a hurry, workers swapping robot arms and wheeling in new heavy power generators and transformer units, half the line already turning out olive-drab utility vehicles."),
    'fusion_research_campus': ('power', 'fusion_campus_torus', "A modern university campus of glass buildings on a green lawn around a big doughnut-shaped fusion reactor hall with its roof cut away, scientists crossing the lawn, the reactor's ring glowing softly inside."),
    'moonshot_grant': ('science', 'moonshot_rocket_lawn', "A tall gleaming research rocket on a launch pad at the edge of a university research park, a pale moon in the daytime sky above it, scientists in shirtsleeves wheeling crates of glowing sky-blue research cubes to its base."),
    'headquarters_raid': ('security', 'raid_on_the_tower_lobby', "A security team in coral-trimmed tactical gear sweeping through the glass doors of a sleek corporate headquarters lobby in daylight, startled suited figures with raised hands, papers drifting off a reception desk."),
    'demand_response_grid': ('power', 'grid_analytics_desk', "A bright analysts' desk overlooking a city, three people at a long curved desk studying large painted charts of the city's power use with a model of the grid's pylons and substations laid out on the table in front of them."),
    'interceptor_battery': ('security', 'checkpoint_chain_road', "A road winding over green hills with a small checkpoint on each rise, each a booth and a striped barrier with two guards checking a car, the chain of checkpoints running away to the horizon."),
}

# Where each card's current scene sits, so it can be released.
where = {}
for tag, group in s['scenes'].items():
    for sid, sc in group.items():
        if 'card' in sc:
            where[sc['card']] = (tag, sid)

def release(card):
    tag, sid = where[card]
    sc = s['scenes'][tag][sid]
    del sc['card']
    # A borrowed scene keeps its first `was`; an old-name scene gets the card's id.
    s['scenes'][tag][sid] = {'was': sc.pop('was', card), **sc}
    return sid

log = []
for card, sid in REUSE.items():
    old = release(card)
    tag = next(t for t, g in s['scenes'].items() if sid in g)
    sc = s['scenes'][tag][sid]
    assert 'card' not in sc, sid
    s['scenes'][tag][sid] = {'card': card, **sc}
    log.append((card, old, sid, 'reused'))

def partner(card):
    """A deck card of the same first tag and the other window shape, for the batch's sample pair."""
    c = cards[card]
    want_building = c['kind'] != 'building'
    for other in sorted(where):
        o = cards.get(other)
        if o and other != card and o['tags'][:1] == c['tags'][:1] and (o['kind'] == 'building') == want_building and other not in NEW and other not in REUSE:
            return other
    raise SystemExit(f'no partner for {card}')

batch = {}
for card, (tag, sid, subject) in NEW.items():
    old = release(card)
    assert all(sid not in g for g in s['scenes'].values()), sid
    s['scenes'][tag][sid] = {'card': card, 'subject': subject}
    p = partner(card)
    batch[sid] = [p, card] if cards[card]['kind'] == 'building' else [card, p]
    log.append((card, old, sid, 'new'))

s['round'] = 'b7'
s['picker'] = 'scene_batch_7'
s['batch'] = batch

# Keep the file's one-scene-a-line layout.
def inner(d):
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
for row in log:
    print(*row, sep='\t')
print('batch', json.dumps(batch))

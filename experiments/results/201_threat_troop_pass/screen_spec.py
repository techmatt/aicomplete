"""201: screen specs (three pairs, d0-d3 by default), cells as overlays on one binary.

    python experiments/results/201_threat_troop_pass/screen_spec.py SCREEN [AGENT] [GAMES] > experiments/farm/s201-SCREEN.json

`base` is r88 as 200 left it. Cell names are the lever list joined by `+` in CELLS; LEVERS gives each lever's sets.
"""
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def load(name):
    with open(os.path.join(ROOT, 'content', name), encoding='utf-8') as f:
        return json.load(f)


def insight_counters(t1, t2):
    """Every insight counter item to t1 (tier 1) / t2 (tier 2)."""
    out = []
    for th in load('threats.json'):
        for i, opt in enumerate(th['counters']):
            for j, it in enumerate(opt):
                if it.get('resource') == 'insight':
                    out.append(f"threats.{th['id']}.counters.{i}.{j}.amount={t1 if th['tier'] == 1 else t2}")
    return out


def musters(by_stage):
    """Each event's Muster line to by_stage[stage] troops (0 drops the line)."""
    out = []
    for ev in load('adversary.json')['events']:
        st = ev['standing']
        idx = [i for i, e in enumerate(st) if e['op'] == 'troops' and e.get('each_country')]
        if not idx:
            continue
        n = by_stage[ev['stage']]
        if n == 0:
            st2 = [e for k, e in enumerate(st) if k not in idx]
            out.append(f"adversary.{ev['id']}.standing={json.dumps(st2, separators=(',', ':'))}")
        else:
            out.append(f"adversary.{ev['id']}.standing.{idx[0]}.count={n}")
    return out


def add_line(ids, line):
    """Appends `line` to each event's standing lines (its other lines; before the Corrupt twists)."""
    out = []
    evs = {e['id']: e for e in load('adversary.json')['events']}
    for i in ids:
        st = evs[i]['standing'] + [line]
        out.append(f"adversary.{i}.standing={json.dumps(st, separators=(',', ':'))}")
    return out


def threat_troops_less():
    """Each threat landing 2+ troops lands 1 fewer."""
    out = []
    for th in load('threats.json'):
        for i, e in enumerate(th['effects']):
            if e['op'] == 'troops' and e.get('count', 1) >= 2:
                out.append(f"threats.{th['id']}.effects.{i}.count={e['count'] - 1}")
    return out


SPREAD = ['mil_s1_long_patrols', 'mil_s1_signal_jamming', 'mil_s2_station_raids', 'mil_s2_fifth_column']
AGENTS = ['mil_s2_station_raids', 'mil_s2_border_war', 'mil_s2_supply_lines', 'mil_s2_forced_march',
          'mil_s3_blitzkrieg', 'mil_s3_naval_blockade']
def stage_muster(stage, n):
    """Every event of `stage` musters `n` (a Muster line added after its Rally where it has none)."""
    out = []
    for ev in load('adversary.json')['events']:
        if ev['stage'] != stage:
            continue
        st = list(ev['standing'])
        idx = [i for i, e in enumerate(st) if e['op'] == 'troops' and e.get('each_country')]
        if idx:
            st[idx[0]] = dict(st[idx[0]], count=n)
        else:
            st.insert(1, {'op': 'troops', 'count': n, 'each_country': True})
        out.append(f"adversary.{ev['id']}.standing={json.dumps(st, separators=(',', ':'))}")
    return out


LEVERS = {
    'w1': ['adversary.wall_troops=1'],
    'e1': stage_muster(1, 1),
    'l2': stage_muster(3, 2),
    'sp': add_line(SPREAD, {'op': 'spread_botnets', 'count': 1}),
    'ai': add_line(AGENTS, {'op': 'increase_income', 'ambassadors': 1}),
    'p': insight_counters(3, 4),
    'p5': insight_counters(3, 5),
    'tn': ['board.troop_node=0'],
    'ts': ['board.troop_spent=0'],
    'st1': ['board.troop_steps=1'],
    'm011': musters({1: 0, 2: 1, 3: 1}),
    'm001': musters({1: 0, 2: 0, 3: 1}),
    'm012': musters({1: 0, 2: 1, 3: 2}),
    'nw': ['adversary.wall_troops=0'],
    'bc3': ['board.botnet_credits=3'],
    'tt': threat_troops_less(),
    'a1': ['adversary.ambassador_income=0,1,1,1,2'],
    'a0': ['adversary.ambassador_income=0,0,0,0,0'],
    'c1': ['adversary.compute_income=1,1,1,1,1'],
}
CELLS = {
    '1': ['base', 'p', 'p+tn', 'p+tn+ts', 'p+tn+m011', 'p+tn+ts+m011', 'p+tn+ts+m011+st1'],
    '2': ['base', 'p+tn+m011+nw+st1', 'p+tn+m001+nw+st1', 'p+tn+m011+st1', 'p+tn+m011+nw+st1+ts', 'p+tn+m011+nw',
          'p+tn+m011+nw+st1+sp+bc3', 'p+tn+m011+nw+st1+ai'],
    '2x': ['base', 'p+tn+m011+nw+st1'],
    # Screen 3 on message 9119's base (Annihilate off every board, `antimatter_discord` 0, in content/configs).
    '3': ['base', 'p', 'p+tn+m011+st1', 'p+tn+m011+st1+tt', 'p+tn+m011+st1+tt+sp+bc3', 'p+tn+m011+st1+tt+sp+bc3+ai',
          'p+tn+m001+st1+tt+sp+bc3+ai', 'p5+tn+m011+st1+tt+sp+bc3+ai'],
    '3x': ['base', 'p+tn+m011+st1+tt+sp+bc3+ai'],
    # Screen 4: the fewest troops (no stage-I/II Musters, no Wall Muster) and the dials' reach.
    # Screen 5 (message 9148), on the r89 build as frozen before the dials: the Wall Muster back (w1), and the
    # same extra troops early (a Muster 1 on every stage-I card, e1) or late (stage-III Musters 2, l2).
    '5': ['base', 'w1', 'e1', 'l2'],
    '4': ['p+tn+m001+st1+tt+nw+sp+bc3+ai', 'p+tn+m001+st1+tt+nw+sp+bc3+ai+a1', 'p+tn+m001+st1+tt+nw+sp+bc3+ai+a0',
          'p+tn+m001+st1+tt+nw+sp+bc3+ai+a0+c1'],
}


def sets(cell):
    if cell == 'base':
        return []
    out = []
    for lever in cell.split('+'):
        out += LEVERS[lever]
    return out


def spec(name, cells, agent='strategist', games=32, diffs=(0, 1, 2, 3)):
    variants = []
    for c in cells:
        for p, ps in PAIRS:
            variants.append({'name': p if c == 'base' else f'{p}_{c}', 'set': ps + sets(c)})
    return {'name': name, 'note': '201 screen', 'configs': ['configs/baseline.json'],
            'variants': variants, 'difficulties': list(diffs), 'agents': [agent], 'games': games, 'records': 0}


if __name__ == '__main__':
    which = sys.argv[1]
    agent = sys.argv[2] if len(sys.argv) > 2 else 'strategist'
    games = int(sys.argv[3]) if len(sys.argv) > 3 else 32
    print(json.dumps(spec(f's201-{which}', CELLS[which], agent, games), indent=2))

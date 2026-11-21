"""268: turn an `aic-sim run --trace` stdout into
  script.md  - what a table sees revealed and what the team decided, per round (no engine resolution)
  counts.json - per-round mechanical counts: components moved, decisions and options, round-scoped memory vars
  check.md   - each round's end state (pieces per hex, tracks), to check a hand resolution against
Usage: python trace_rounds.py TRACE.out OUTDIR [countries]
"""
import json, sys, os, collections

DIRS = {'E': (1, 0), 'W': (-1, 0), 'SE': (0, 1), 'NW': (0, -1), 'SW': (-1, 1), 'NE': (1, -1)}


def hex_names(countries):
    m = json.load(open('content/map.json', encoding='utf-8'))
    lay = next(l for l in m['layouts'] if l['countries'] == countries)
    names = {}
    for ci, p in enumerate(lay['pieces']):
        name = m['countries'][ci]['name']
        a0, a1 = DIRS[p['axes'][0]], DIRS[p['axes'][1]]
        for i in range(16):
            row, col = divmod(i, 4)
            q = p['anchor'][0] + row * a0[0] + col * a1[0]
            r = p['anchor'][1] + row * a0[1] + col * a1[1]
            names[(q, r)] = f'{name} #{i + 1}'
    return names


# Round-scoped bookkeeping a player must remember with no physical marker (var prefixes).
MEMORY = ['ai.power_used.', 'board.energy_bought.', 'board.converter_round.', 'board.chip_converter_round.',
          'board.vigil_round.', 'board.vigil_paid.', 'board.moved.', 'board.hflag.', 'cards.used.', 'board.std_once',
          'board.cube_insight', 'board.troop_removed', 'prohibit.', 'threats.late.', 'threats.jam.', 'board.cordon.',
          'cards.disabled.']
# Physical track markers: a var change is one marker moved.
TRACKS = ['alignment.discord', 'alignment.unity', 'adversary.compute', 'adversary.compute_waiting',
          'adversary.pool.', 'adversary.research', 'adversary.income_', 'adversary.project.', 'board.project.',
          'board.track', 'cards.tokens.', 'alignment.unity_tips', 'alignment.discord_tips']


def main():
    src, out = sys.argv[1], sys.argv[2]
    countries = int(sys.argv[3]) if len(sys.argv) > 3 else 2
    names = hex_names(countries)
    card_names = {}
    for f in ('cards', 'threats', 'projects'):
        for x in json.load(open(f'content/{f}.json', encoding='utf-8')):
            card_names[x['id']] = x.get('name', x['id'])
    nm = lambda h: names.get(tuple(h), str(h)) if h else ''
    os.makedirs(out, exist_ok=True)
    rnd, phase = 0, 'setup'
    script = []
    counts = collections.defaultdict(lambda: collections.Counter())
    opts = collections.defaultdict(list)
    memory_keys = collections.defaultdict(set)
    hexes = collections.defaultdict(collections.Counter)
    tracks = {}
    snaps = []
    last_ask = None

    def snap(label):
        lines = [f'## {label}', '']
        for h in sorted(hexes, key=lambda h: nm(h)):
            pieces = ', '.join(f'{k} {v}' for k, v in sorted(hexes[h].items()) if v)
            if pieces:
                lines.append(f'- {nm(h)}: {pieces}')
        lines.append('')
        lines.append('Tracks: ' + ', '.join(f'{k}={v}' for k, v in sorted(tracks.items())))
        lines.append('')
        snaps.append('\n'.join(lines))

    for l in open(src, encoding='utf-8'):
        if not l.startswith('{'):
            continue
        d = json.loads(l)
        if 'action' in d:
            a = d['action']; n = d.get('options', 1)
            c = counts[rnd]
            c['decisions'] += 1
            if n > 1:
                c['decisions_real'] += 1
                opts[rnd].append(n)
            desc = {k: v for k, v in a.items() if k not in ('type', 'seat')}
            q = f' (asked: {last_ask})' if a['type'] == 'choose' and last_ask else ''
            script.append(f'- seat {a.get("seat", "-")} **{a["type"]}** {json.dumps(desc)}{q} [{n} options]')
            last_ask = None
            continue
        e = d.get('event')
        if not e:
            continue
        t = e['type']
        c = counts[rnd]
        if t == 'round_started':
            if rnd:
                snap(f'End of round {rnd}')
            else:
                snap('After setup')
            rnd = e['round']
            script.append(f'\n## Round {rnd}\n')
        elif t == 'phase_started':
            phase = e['phase']
            script.append(f'\n*phase {phase}*')
        elif t == 'asked':
            last_ask = e['prompt']
        elif t == 'note':
            txt = e['text']
            if txt.startswith('Adversary card:') or 'resolves:' in txt or 'tips' in txt:
                script.append(f'- REVEAL: {txt}')
        elif t == 'card_moved':
            c['cards_moved'] += 1
            fr = e['from']
            if 'deck' in fr:
                # 309: the printed name beside the id (an id like orbital_strike prints as Weapons Test).
                nmc = card_names.get(e['card'])
                shown = f'{e["card"]} ({nmc})' if nmc and nmc.lower().replace(' ', '_') != e['card'] else e['card']
                script.append(f'- REVEAL: {shown} from {fr} to {e["to"]}')
        elif t == 'tokens_changed':
            k = e['token'].split(':')[0]
            c['tokens_moved'] += abs(e['delta'])
            c['tok.' + k] += abs(e['delta'])
            hexes[tuple(e['hex'])][e['token']] = e['total']
        elif t == 'resource_changed':
            c['resource_changes'] += 1
            tracks[f'seat{e["seat"]}.{e["resource"]}'] = e['total']  # 309: each seat's resources in check.md
        elif t == 'var_changed':
            k = e['key']
            if any(k.startswith(p) for p in MEMORY):
                memory_keys[rnd].add(k)
            if k.startswith('adversary.project.') and e['delta'] > 0:
                c['compute_onto_projects'] += e['delta']
            if k.startswith('adversary.pool.') and e['delta'] > 0:
                c['agents_into_pools'] += e['delta']
            if any(k.startswith(p) for p in TRACKS):
                c['marker_moves'] += 1
                if k.startswith(('alignment.', 'adversary.compute', 'adversary.research', 'adversary.pool',
                                 'adversary.project.', 'board.project.', 'board.track')):  # 309: projects, AI tracks
                    tracks[k] = e['total']
        elif t == 'game_ended':
            script.append(f'- GAME ENDS: {json.dumps(e)}')
            snap(f'End of round {rnd} (game end)')
    res = {}
    for r in sorted(counts):
        c = dict(counts[r])
        o = opts[r]
        c['options_mean'] = round(sum(o) / len(o), 1) if o else 0
        c['options_max'] = max(o) if o else 0
        c['memory_vars'] = len(memory_keys[r])
        c['memory_kinds'] = sorted({k.rsplit('.', 1)[0] for k in memory_keys[r]})
        res[r] = c
    open(os.path.join(out, 'script.md'), 'w', encoding='utf-8', newline='\n').write('\n'.join(script) + '\n')
    open(os.path.join(out, 'counts.json'), 'w', encoding='utf-8', newline='\n').write(json.dumps(res, indent=1))
    open(os.path.join(out, 'check.md'), 'w', encoding='utf-8', newline='\n').write('\n'.join(snaps))
    for r, c in res.items():
        print(r, {k: v for k, v in c.items() if not k.startswith('tok.') and k != 'memory_kinds'})


main()

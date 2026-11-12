"""200: the antimatter pieces' use a game, per DIR (comma lists pool): win %, each new or changed piece's use, and the
antimatter plants' offered / kept / played / Action uses.

    python experiments/results/200_engine_overhaul/pieces.py DIR [DIR...]
"""
import json
import sys
from collections import Counter

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
CARDS = ['penning_trap_array', 'antiproton_foundry', 'positron_storage_ring', 'nuclear_option', 'kinetic_interdiction',
         'great_liberation', 'singularity_exploit', 'zero_day_auction']
POW = ['ai.power.builder.annihilate', 'ai.power.protector.annihilate', 'ai.power.researcher.annihilate',
       'ai.power.researcher.particle_run']


def read(dirs):
    n = won = 0
    c = Counter()
    by_d = Counter()
    by_dw = Counter()
    for d in dirs.split(','):
        with open(f'{d}/games.jsonl', encoding='utf-8') as f:
            next(f)
            for line in f:
                g = json.loads(line)
                n += 1
                won += bool(g['won'])
                by_d[int(g['difficulty'])] += 1
                by_dw[int(g['difficulty'])] += bool(g['won'])
                for k in POW:
                    c[k] += g['counters'].get(k, 0)
                for cid in CARDS:
                    e = g['cards'].get(cid) or {}
                    for kk in ('offered', 'kept', 'played'):
                        c[f'{cid}.{kk}'] += e.get(kk, 0)
                for u in g.get('uses') or []:
                    for src, row in u.items():
                        if src.startswith('own:use:') or src.startswith('own:convert'):
                            c[src] += 1 if any(row) else 0
    return n, won, c, by_d, by_dw


for d in sys.argv[1:]:
    n, won, c, by_d, by_dw = read(d)
    lv = ', '.join(f'd{k} {100 * by_dw[k] / by_d[k]:.1f}' for k in sorted(by_d))
    print(f'## {d}: {n} games, win {100 * won / n:.1f}% ({lv})')
    for k in POW:
        print(f'  {k}: {c[k] / n:.2f}')
    for cid in CARDS:
        print(f"  {cid}: offered {c[cid + '.offered'] / n:.2f} kept {c[cid + '.kept'] / n:.2f} played {c[cid + '.played'] / n:.2f}"
              f" Action-uses {c['own:use:' + cid] / n:.2f}")
    print(f"  converters (uses with a cost): " + ', '.join(f'{k} {v / n:.2f}' for k, v in sorted(c.items()) if k.startswith('own:convert')))

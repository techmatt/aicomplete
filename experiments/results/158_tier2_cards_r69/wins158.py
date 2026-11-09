"""158: win % by cell group (config / level / lever, the three pairs pooled), with the SE and the paired difference
from a reference group on the same seeds.

    python wins158.py DIR [DIR ...] [--ref GROUP-SUBSTRING]

Run from the repo root. A group is the cell label less its pair (bp / br / pr): CONFIG/dN/LEVER/AGENT.
"""
import argparse
import collections
import math
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cell_labels, stream  # noqa: E402


def group(label):
    out = []
    for p in label.split('/'):
        for pair in ('bp', 'br', 'pr'):
            if p == pair:
                p = ''
            elif p.startswith(pair + '_'):
                p = p[len(pair) + 1:]
        if p:
            out.append(p)
    return '/'.join(out)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    a = ap.parse_args()
    res = collections.defaultdict(dict)  # group -> (pair, seed) -> won
    reasons = collections.defaultdict(collections.Counter)
    for d in a.dirs:
        it = stream(d)
        labels = cell_labels(next(it)['spec'])
        for g in it:
            lab = labels[g['cell']]
            pair = next((p for p in lab.split('/') if p[:2] in ('bp', 'br', 'pr')), '')[:2]
            k = group(lab)
            res[k][(pair, g['seed'])] = 1 if g['won'] else 0
            reasons[k][g['reason']] += 1
    levels = collections.defaultdict(list)
    for k in res:
        lv = next((p for p in k.split('/') if p.startswith('d') and p[1:].isdigit()), '')
        levels[lv].append(k)
    for lv in sorted(levels):
        ks = sorted(levels[lv])
        base = next((k for k in ks if k.startswith('baseline/') or '/' not in k.replace(lv + '/', '')), ks[0])
        print(f'\n### {lv or "all"} (paired against {base})\n')
        print('| group | games | win % | SE | diff | paired SE | ends |')
        print('|---|---|---|---|---|---|---|')
        for k in ks:
            v = list(res[k].values())
            n = len(v)
            p = sum(v) / n
            se = math.sqrt(p * (1 - p) / n)
            common = [s for s in res[k] if s in res[base]]
            dd = [res[k][s] - res[base][s] for s in common]
            m = sum(dd) / len(dd) if dd else 0.0
            sd = math.sqrt(sum((x - m) ** 2 for x in dd) / max(len(dd) - 1, 1)) / math.sqrt(max(len(dd), 1)) if dd else 0.0
            ends = ', '.join(f'{r} {c}' for r, c in reasons[k].most_common())
            print(f'| {k} | {n} | {100 * p:.1f} | {100 * se:.1f} | {100 * m:+.1f} | {100 * sd:.1f} | {ends} |')


if __name__ == '__main__':
    main()

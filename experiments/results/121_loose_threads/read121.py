"""121's ballpark read: r48 against r47, paired on seed, cell by cell.

    python experiments/results/121_loose_threads/read121.py OLD_DIR NEW_DIR [--md OUT.md]

For each cell (VARIANT/AGENT): win % in each run and the paired gap (NEW - OLD) with one SE,
mean rounds, how games ended, credits held after the last completed action phase per seat
(`margin.held_after_actions[seat][0]`), each resource's mean peak per seat (`peaks`, PEAKS
order) and its highest game, the most plays of one card in a game, and timeouts.
"""
import argparse
import math
import os
import sys
from collections import Counter

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cell_labels, stream  # noqa: E402

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
SHORT = ['cr', 'ins', 'en', 'chip', 'am', 'cube', 'tok', 'amb']
BOARDS = {'bp': ['Builder', 'Protector'], 'br': ['Builder', 'Researcher'], 'pr': ['Protector', 'Researcher']}


def load(d):
    lines = stream(d)
    head = next(lines)
    names = cell_labels(head['spec'])
    cells = {n: {} for n in names}
    for g in lines:
        g.pop('actions', None)
        cells[names[g['cell']]][g['seed']] = g
    return head.get('rules_revision'), cells


def mean(xs):
    xs = list(xs)
    return sum(xs) / len(xs) if xs else float('nan')


def se(xs):
    xs = list(xs)
    if len(xs) < 2:
        return float('nan')
    m = mean(xs)
    return math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1) / len(xs))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('old')
    ap.add_argument('new')
    ap.add_argument('--md')
    a = ap.parse_args()
    rold, old = load(a.old)
    rnew, new = load(a.new)
    out = []
    p = out.append
    p(f'# r{rnew} against r{rold}, paired on seed\n')
    p('| cell | r%s win | r%s win | gap ± SE | rounds | ends (new) | timeouts | most plays (card) |' % (rold, rnew))
    p('|---|---|---|---|---|---|---|---|')
    pooled = {}
    for name in new:
        if name not in old:
            continue
        seeds = sorted(set(old[name]) & set(new[name]))
        wo = [old[name][s]['won'] for s in seeds]
        wn = [new[name][s]['won'] for s in seeds]
        d = [int(y) - int(x) for x, y in zip(wo, wn)]
        agent = name.split('/')[1]
        pooled.setdefault(agent, []).extend(d)
        ro = mean(old[name][s]['rounds'] for s in seeds)
        rn = mean(new[name][s]['rounds'] for s in seeds)
        ends = Counter(new[name][s]['reason'] for s in seeds)
        tout = sum(new[name][s]['reason'] == 'timeout' for s in seeds)
        best = max(((v.get('played', 0), c, s) for s in seeds for c, v in new[name][s]['cards'].items()), default=(0, '', 0))
        p(f'| {name} | {100*mean(wo):.0f}% | {100*mean(wn):.0f}% | {100*mean(d):+.0f} ± {100*se(d):.0f} pp | '
          f'{ro:.2f} → {rn:.2f} | {", ".join(f"{k} {v}" for k, v in ends.most_common())} | {tout} | {best[0]} ({best[1]}, seed {best[2]}) |')
    p('')
    for agent, d in pooled.items():
        p(f'- {agent}, three pairs pooled: {100*mean(d):+.1f} ± {100*se(d):.1f} pp over {len(d)} paired games')
    p('\n## Credits held after the last completed action phase (mean per seat, r%s → r%s)\n' % (rold, rnew))
    p('| cell | seat 0 | seat 1 |')
    p('|---|---|---|')
    for name in new:
        if name not in old:
            continue
        seeds = sorted(set(old[name]) & set(new[name]))
        v = name.split('/')[0]
        row = []
        for s in range(2):
            o = mean(old[name][x]['margin']['held_after_actions'][s][0] for x in seeds)
            n = mean(new[name][x]['margin']['held_after_actions'][s][0] for x in seeds)
            row.append(f'{BOARDS.get(v, ["?", "?"])[s]} {o:.1f} → {n:.1f}')
        p(f'| {name} | {" | ".join(row)} |')
    p('\n## Peak holdings per seat, mean (highest game), r%s → r%s\n' % (rold, rnew))
    p('Columns: ' + ', '.join(f'{s} = {l}' for s, l in zip(SHORT, PEAKS)) + '.\n')
    p('| cell | seat | ' + ' | '.join(SHORT) + ' |')
    p('|---|---|' + '---|' * len(SHORT))
    for name in new:
        if name not in old:
            continue
        seeds = sorted(set(old[name]) & set(new[name]))
        v = name.split('/')[0]
        for s in range(2):
            cells = []
            for i in range(len(PEAKS)):
                o = [old[name][x]['peaks'][s][i] for x in seeds]
                n = [new[name][x]['peaks'][s][i] for x in seeds]
                cells.append(f'{mean(o):.0f} ({max(o)}) → {mean(n):.0f} ({max(n)})')
            p(f'| {name} | {BOARDS.get(v, ["?", "?"])[s]} | ' + ' | '.join(cells) + ' |')
    text = '\n'.join(out) + '\n'
    sys.stdout.reconfigure(encoding='utf-8')
    print(text)
    if a.md:
        with open(a.md, 'w', encoding='utf-8', newline='\n') as f:
            f.write(text)


if __name__ == '__main__':
    main()

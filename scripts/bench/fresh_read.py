"""A fresh read against the previous revision's, both sides on the same seeds, as one Markdown table (286: promoted
from 282's and 283's `experiments/results/<item>/read.py`, which took two games files and their specs).

    python scripts/bench/fresh_read.py NEW OLD [--md OUT] [--growth SLUG[,SLUG...]]

NEW and OLD are reads in `read_lib.py`'s forms: a run's folder, a games file with its header, or FILE=SPEC (a farmed
JSONL and its farm spec). Per agent (as the spec writes it) and level, old → new: win %, rounds, cities corrupted by
round 4 (`map.cities_corrupted.r01`-`r04`) and at the end (`margin.cities_corrupted_end`), node endings (games),
research troops (`adversary.research_troops`) and nodes lost to the march (`board.node_lost.march`) a game, and card
plays and non-card turns (standard actions, board powers but Growth, card uses) a seat-round. A row only one side
has shows `-` on the other. `--growth`: the board-power slugs left out of non-card turns (default: every power of
group `Growth` in content/ais.json, slugged as the `ai.power.<board>.<slug>` counters are). `--md OUT` also writes
the table to OUT. Python's standard library only.
"""
import json
import os
import re
import sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..'))
from screens import cells_of, stream  # noqa: E402

ROOT = os.path.normpath(os.path.join(HERE, '..', '..'))


def growth_slugs():
    """The tree's Growth powers' counter slugs (`Buy In` -> `buy_in`)."""
    with open(os.path.join(ROOT, 'content', 'ais.json'), encoding='utf-8') as f:
        boards = json.load(f)
    return {re.sub(r'[^a-z0-9]+', '_', p['name'].lower()).strip('_')
            for b in boards for p in b.get('powers', []) if p.get('group') == 'Growth'}


def cnt(g, k):
    v = g['counters'].get(k, 0)
    return v if isinstance(v, (int, float)) else 0


def side(read, growth):
    """{(agent, level): the row's measures} of READ."""
    lines = stream(read)
    cells = cells_of(next(lines)['spec'])
    rows = defaultdict(list)
    for g in lines:
        if 'cell' not in g or 'won' not in g:
            continue
        c = cells[g['cell']]
        rows[(c['agent'], c['difficulty'])].append(g)
    out = {}
    for key, gs in rows.items():
        n = len(gs)
        sr = sum(len(g['spent']) * g['rounds'] for g in gs) or 1
        plays = std = powers = uses = 0
        for g in gs:
            for k, v in g['counters'].items():
                p = k.split('.')
                if not isinstance(v, (int, float)):
                    continue
                if k.startswith('cards.played.') and len(p) == 3:
                    plays += v
                elif k.startswith('board.std.') and len(p) == 3:
                    std += v
                elif (k.startswith('ai.power.') and len(p) == 4 and not re.fullmatch(r'r\d+', p[3])
                      and p[3] not in growth):
                    powers += v
                elif k.startswith('cards.used.') and len(p) == 3:
                    uses += v
        out[key] = dict(
            win=100 * sum(g['won'] for g in gs) / n, rounds=sum(g['rounds'] for g in gs) / n,
            r4=sum(sum(cnt(g, f'map.cities_corrupted.r0{r}') for r in range(1, 5)) for g in gs) / n,
            end=sum((g.get('margin') or {}).get('cities_corrupted_end', 0) for g in gs) / n,
            node=sum(g['reason'] == 'node' for g in gs), rt=sum(cnt(g, 'adversary.research_troops') for g in gs) / n,
            march=sum(cnt(g, 'board.node_lost.march') for g in gs) / n, plays=plays / sr,
            noncard=(std + powers + uses) / sr)
    return out


FORMS = [('win', '{:.1f}'), ('rounds', '{:.2f}'), ('r4', '{:.2f}'), ('end', '{:.2f}'), ('node', '{}'),
         ('rt', '{:.1f}'), ('march', '{:.1f}')]


def table(new, old):
    """The Markdown lines, old → new (the new win % bold)."""
    L = ['| agent | d | win % old → new | rounds | cities corrupted by r4 | at the end | node endings | research troops '
         '| nodes lost to the march | plays / non-card turns a seat-round |',
         '|---|---|---|---|---|---|---|---|---|---|']
    for key in sorted(set(new) | set(old), key=lambda k: (k[0], k[1])):
        a, b = old.get(key), new.get(key)
        cells = []
        for m, form in FORMS:
            o = form.format(a[m]) if a else '-'
            w = form.format(b[m]) if b else '-'
            cells.append(f'{o} → **{w}**' if m == 'win' else f'{o} → {w}')
        pc = lambda s: f"{s['plays']:.2f} / {s['noncard']:.2f}" if s else '-'  # noqa: E731
        L.append(f'| {key[0]} | d{key[1]} | ' + ' | '.join(cells) + f' | {pc(a)} → {pc(b)} |')
    return L


def main(argv):
    opts = {'--md': None, '--growth': None}
    reads, cur = [], None
    for x in argv:
        if cur:
            opts[cur], cur = x, None
        elif x in opts:
            if opts[x] is not None:
                sys.exit(f'{x} given twice')
            cur = x
        elif x.startswith('--'):
            sys.exit(f'fresh_read: unknown flag {x}; the flags are --md, --growth')
        else:
            reads.append(x)
    if cur:
        sys.exit(f'{cur} needs a value')
    if len(reads) != 2:
        sys.exit(__doc__)
    growth = set(opts['--growth'].split(',')) if opts['--growth'] else growth_slugs()
    text = '\n'.join(table(side(reads[0], growth), side(reads[1], growth))) + '\n'
    print(text, end='')
    if opts['--md']:
        with open(opts['--md'], 'w', encoding='utf-8', newline='\n') as f:
            f.write(text)


if __name__ == '__main__':
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    main(sys.argv[1:])

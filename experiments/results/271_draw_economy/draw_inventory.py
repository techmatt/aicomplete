"""271: inventory of card draw and any-card triggers in content/. Run from the repo root.

python experiments/results/271_draw_economy/draw_inventory.py [--md] [--content=DIR]

Counts every card's draw (on play, in an Action, in a trigger), across all 208 content cards (the 191 in the draft
plus the opening's and liberate capstones), by tag and source, and the boards', converters' and megaprojects' draw ops.
"""
import json
import sys
from collections import Counter, defaultdict


CONTENT = next((a.split('=', 1)[1] for a in sys.argv if a.startswith('--content=')), 'content')


def load(name):
    with open(f'{CONTENT}/{name}.json', encoding='utf-8') as f:
        return json.load(f)


def walk(node, path, out):
    """Yield (path, draw count) for each draw op under node; path records the enclosing keys/ops."""
    if isinstance(node, dict):
        if node.get('op') == 'draw':
            out.append((tuple(path), node.get('count', 0)))
        for k, v in node.items():
            p = path + [k]
            if isinstance(v, dict) and 'op' in v:
                p = p + [v['op']]
            walk(v, p, out)
    elif isinstance(node, list):
        for v in node:
            p = path + ([v['op']] if isinstance(v, dict) and 'op' in v else [])
            walk(v, p, out)


def source_of(path):
    if 'passives' in path:
        return 'trigger'
    if 'ability' in path:
        return 'action'
    return 'on play'


def any_card_triggers(card):
    rows = []
    for p in card.get('passives', []):
        if p.get('op') == 'when' and p.get('on', {}).get('event') == 'play':
            on = p['on']
            if not on.get('tag') and not on.get('kind') and not on.get('min_cost') is None or (
                    not on.get('tag') and not on.get('kind')):
                rows.append(json.dumps(on))
    return rows


def main():
    md = '--md' in sys.argv
    cards = load('cards')
    by_tag = Counter()
    by_src = Counter()
    by_count = Counter()
    rows = []
    for c in cards:
        out = []
        walk(c, [], out)
        if not out:
            continue
        total = sum(n for _, n in out)
        srcs = sorted({source_of(p) for p, _ in out})
        tag = '/'.join(c.get('tags', [])) or '-'
        for t in c.get('tags', []) or ['-']:
            by_tag[t] += total
        for s in srcs:
            by_src[s] += 1
        by_count[total] += 1
        rows.append((c['id'], c['tier'], c['kind'], tag, total, ','.join(srcs)))
    print(f'cards that draw: {len(rows)} of {len(cards)}; cards drawn (sum of counts): {sum(r[4] for r in rows)}')
    print('by count:', dict(sorted(by_count.items())))
    print('by tag (draws, a card counted under each tag):', dict(by_tag.most_common()))
    print('by source (cards):', dict(by_src))
    if md:
        print('| Card | Tier | Kind | Tags | Draw | Source |')
        print('|---|---|---|---|---|---|')
        for r in sorted(rows, key=lambda r: (-r[4], r[0])):
            print('| ' + ' | '.join(str(x) for x in r) + ' |')
    # boards, converters, megaprojects
    for name in ('ais', 'map', 'projects'):
        out = []
        walk(load(name), [], out)
        print(f'{name}: {len(out)} draw ops, {sum(n for _, n in out)} cards')
    anyc = []
    for c in cards:
        for p in c.get('passives', []):
            on = p.get('on', {}) if p.get('op') == 'when' else {}
            if on.get('event') == 'play' and not on.get('tag'):
                anyc.append((c['id'], json.dumps(on)))
    print(f'any-card play triggers (no tag): {len(anyc)}')
    for a in anyc:
        print('  ', *a)


if __name__ == '__main__':
    main()

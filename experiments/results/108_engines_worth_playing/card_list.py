"""108: the card list, every new, changed and cut card of r43 with its reason and value check.

    python experiments/results/108_engines_worth_playing/card_list.py > experiments/results/108_engines_worth_playing/cards.md

Run from the repo root after `aic-sim docs` (it reads web/card/faces.json, r42's from git at 57469c6) and
notes_r43.py (the value check is each card's line in docs/card-pricing-notes.json, from its "Give").
"""
import io
import json
import re
import subprocess
import sys

R42 = '57469c6'
sys.path.insert(0, 'experiments/results/108_engines_worth_playing')
import apply_r43  # noqa: E402


def git_json(path):
    return json.loads(subprocess.run(['git', 'show', f'{R42}:{path}'], capture_output=True, check=True).stdout.decode('utf-8'))


def face(f):
    if not f:
        return ''
    return ' '.join(x for x in (f.get('top_text', ''), f.get('text', '')) if x)


def cost(c):
    parts = [f"{c.get('cost', 0)}c", f"{c.get('insight', 0)}i"]
    for r in ('energy', 'chips', 'antimatter', 'research_cubes'):
        if c.get(r):
            parts.append(f'{c[r]} {r}')
    if c.get('needs'):
        parts.append('needs ' + ', '.join(f'{t} {n}' for t, n in c['needs']))
    if c.get('replaces'):
        parts.append(f"over {c['replaces']}")
    return ', '.join(parts)


def why(old, new):
    """The reasons that apply, from what moved (the item's levers, numbered as the prompt's changes)."""
    out = []
    if old.get('antimatter') and not new.get('antimatter'):
        out.append('5: antimatter only on ultimates')
    if old.get('replaces') and not new.get('replaces'):
        out.append('6: not built over, so it buries nothing it should not')
    on, nn = dict(map(tuple, old.get('needs', []))), dict(map(tuple, new.get('needs', [])))
    if any(nn.get(t, 0) < n for t, n in on.items()):
        out.append('4: needs lowered')
    if new['id'] == 'payroll_automation':
        out.append('7: a Capital feeder (nothing to copy)')
    elif new['id'] in ('retained_earnings', 'vendor_financing', 'materials_hedge_desk', 'liquidity_event',
                       'conglomerate_headquarters'):
        out.append('7: the Capital engine')
    if new['kind'] == 'building' and apply_r43.is_engine(new) and new.get('effects') and not old.get('effects'):
        out.append('2-3: pays up front, back within 2 rounds')
    elif new['kind'] == 'building' and apply_r43.is_engine(new) and not out:
        out.append('3: payback within 2 rounds')
    if old.get('cost', 0) <= 5 and new.get('cost', 0) > old.get('cost', 0):
        out.append('8: fewer cheap cards, bigger and dearer')
    if new['id'] == 'zero_day_auction':
        out.append('4: a chain top made much bigger')
    if not out:
        out.append('8: bigger and dearer' if new.get('cost', 0) > old.get('cost', 0) else 'see note')
    return '; '.join(out)


def check(note):
    m = re.search(r'(Give .*)$', note or '')
    return m.group(1) if m else ''


def main():
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    old = {c['id']: c for c in git_json('content/cards.json')}
    new = {c['id']: c for c in json.load(open('content/cards.json', encoding='utf-8'))}
    fo = git_json('web/card/faces.json')
    fn = json.load(open('web/card/faces.json', encoding='utf-8'))
    notes = json.load(open('docs/card-pricing-notes.json', encoding='utf-8'))
    changed = [i for i in new if i in old and new[i] != old[i]]
    added = [i for i in new if i not in old]
    cut = [i for i in old if i not in new]
    print('# 108: the r43 card list\n')
    print('Every card r43 changed, added or cut: its face before and after (generated faces), why (the prompt\'s change it')
    print('serves), and its value check (`docs/card-pricing-notes.json`: an engine building at a committed seat\'s fire rate,')
    print('`payback.py`; any other card at `configs/agents/values.json`). Costs: credits, insight to play, other costs, needs.\n')
    print(f'{len(changed)} changed, {len(added)} new, {len(cut)} cut; the deck holds {len(new)} cards.\n')
    print('| card | r42 | r43 | why | value check |')
    print('|---|---|---|---|---|')
    for i in sorted(changed, key=lambda i: (new[i]['tags'][0], new[i]['name'])):
        o, n = old[i], new[i]
        print(f"| {n['name']} (t{n['tier']} {n['kind']}) | {cost(o)}: {face(fo.get(i))} | {cost(n)}: {face(fn.get(i))} | "
              f"{why(o, n)} | {check(notes.get(i))} |")
    for i in added:
        n = new[i]
        print(f"| **new** {n['name']} (t{n['tier']} {n['kind']}) | | {cost(n)}: {face(fn.get(i))} | 8: card throughput | "
              f"{check(notes.get(i))} |")
    for cid, w in apply_r43.CUTS:
        o = old[cid]
        print(f"| **cut** {o['name']} (t{o['tier']} {o['kind']}) | {cost(o)}: {face(fo.get(cid))} | | 8: {w} | |")


if __name__ == '__main__':
    main()

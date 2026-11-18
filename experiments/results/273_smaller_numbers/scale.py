"""273: smaller numbers, every credit ÷ 1.5 (Matt's ruling, 2026-11-18).

    python experiments/results/273_smaller_numbers/scale.py [--write]

Reads r127's files (git show aea5535805:<path>, so a rerun starts from the same base) and writes:

- scale.overlay: the cards, AI boards and map (corps) entries, applied with scripts/cards/apply_overlay.py;
- drafts/adversary/threats_t1.json, threats_t2.json, projects.json: credit counters and payments, in place
  (then scripts/content/load_adversary.py writes content/);
- configs/baseline.json: the credit tunables, in place;
- configs/agents/values.json: every credit-valued number, in place (text edits keep its hand layout);
- scale_table.md: every credit number changed, by source, old → new.

Without --write it only writes scale_table.md and the overlay.

Rounding (the write-up, *Rounding*): every number is the nearest whole number to x / 1.5, never under 1 (Matt,
message 12506: prices to nearest, as gains; 271's pick had rounded prices up). A **price** a player pays (a card's
cost, an Action's or power's credit cost, a threat counter, a payment an adversary card demands, a standard action)
and a **gain** (gains, incomes, payouts, discounts) differ only in the floor: a card's cost is floored at 2 (a card
printed under 3 keeps its price). A per-tag rate of 1 credit per N tags becomes 1 per round(1.5 N) (per 2 → 3
is exact); one paying A > 1 a step pays round(A / 1.5) at the same N.
"""
import json
import math
import os
import re
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
HERE = os.path.dirname(os.path.abspath(__file__))
BASE = 'aea5535805'
F = 1.5

# Ops whose credit amount the player loses or pays (a price; the table names them).
LOSS_OPS = {'players_lose', 'requisition', 'theft', 'lose_income', 'lose_node_or', 'lose_ambassadors_or'}
PER_TAG_OPS = {'income_per_tag', 'production_per_tag', 'gain_per_tag'}

rows = []  # (source, where, what, old, new)


def base_text(path):
    return subprocess.run(['git', 'show', f'{BASE}:{path}'], cwd=ROOT, capture_output=True, check=True,
                          encoding='utf-8').stdout


def price(x):
    return x if x <= 0 else max(1, int(math.floor(x / F + 0.5)))


def card_price(x):
    return x if x <= 2 else max(2, int(math.floor(x / F + 0.5)))


def gain(x):
    return x if x <= 0 else max(1, int(math.floor(x / F + 0.5)))


def note(source, where, what, old, new):
    rows.append((source, where, what, old, new))


def scale_cost_list(lst, source, where, what):
    """[["credits", n], ...]: an Action's, power's or converter's cost."""
    for item in lst:
        if isinstance(item, list) and len(item) == 2 and item[0] == 'credits':
            new = price(item[1])
            note(source, where, what + ' cost', item[1], new)
            item[1] = new


def scale_income_list(lst, source, where, what):
    for item in lst:
        if isinstance(item, list) and len(item) == 2 and item[0] == 'credits':
            new = gain(item[1])
            note(source, where, what, item[1], new)
            item[1] = new


def walk_ops(x, source, where, path=''):
    """Scale every credit amount an op prints, recursively."""
    if isinstance(x, list):
        for i, v in enumerate(x):
            walk_ops(v, source, where, f'{path}[{i}]')
        return
    if not isinstance(x, dict):
        return
    op = x.get('op')
    if x.get('resource') == 'credits':
        if op in PER_TAG_OPS:
            amount = x.get('amount', 1)
            if amount > 1:
                new = gain(amount)
                note(source, where, f'{op} ({x.get("tag")}) per {x.get("per", 1)} tags: amount', amount, new)
                x['amount'] = new
            else:
                per = x.get('per', 1)
                new = int(math.floor(per * F + 0.5))
                note(source, where, f'{op} ({x.get("tag")}) 1 credit per N tags: N', per, new)
                x['per'] = new
        elif op == 'cash_out':
            new = gain(x['rate'])
            note(source, where, 'cash_out rate (credits a token)', x['rate'], new)
            x['rate'] = new
        elif 'amount' in x:
            if op in LOSS_OPS:
                new = price(x['amount'])
                note(source, where, f'{op} (a payment)', x['amount'], new)
            else:
                new = gain(x['amount'])
                note(source, where, op or 'amount', x['amount'], new)
            x['amount'] = new
    if op == 'building_discount':
        new = gain(x['amount'])
        note(source, where, 'building_discount', x['amount'], new)
        x['amount'] = new
    on = x.get('on')
    if isinstance(on, dict) and on.get('event') == 'play_cost':
        new = card_price(on['at'])
        note(source, where, 'play_cost trigger: printed cost at least', on['at'], new)
        on['at'] = new
    for k, v in x.items():
        if k == 'cost' and isinstance(v, list):
            scale_cost_list(v, source, where, 'Action')
        elif isinstance(v, (dict, list)):
            walk_ops(v, source, where, f'{path}.{k}')


def overlay_entries(table, key, old_row, new_row):
    out = []
    for k in new_row:
        if new_row[k] != old_row.get(k):
            out.append(f'{table}.{key}.{k}=' + json.dumps(new_row[k], separators=(',', ':'), ensure_ascii=True))
    return out


# Hand text that prints a credit number (AI boards' powers and capstones, the map's megaprojects).
TEXT_EDITS = {
    'draw 1 card; gain 5 credits': 'draw 1 card; gain 3 credits',
    'Your buildings cost 2 credits less.': 'Your buildings cost 1 credit less.',
    'each player cube pays 3 energy and 3 credits': 'each player cube pays 3 energy and 2 credits',
    'each player cube pays 9 credits': 'each player cube pays 6 credits',
    '1 chip for 1 drawn card and 2 credits': '1 chip for 1 drawn card and 1 credit',
    'each player cube pays 2 chips and 3 credits': 'each player cube pays 2 chips and 2 credits',
    'each player cube pays 1 Sway to Unity and 4 credits': 'each player cube pays 1 Sway to Unity and 3 credits',
}


def edit_texts(x, used):
    if isinstance(x, dict):
        for k, v in x.items():
            if isinstance(v, str):
                for a, b in TEXT_EDITS.items():
                    if a in v:
                        v = v.replace(a, b)
                        used.add(a)
                x[k] = v
            else:
                edit_texts(v, used)
    elif isinstance(x, list):
        for v in x:
            edit_texts(v, used)


def content_overlay():
    lines = ['# 273: smaller numbers, credits ÷ 1.5 (generated by scale.py; rerun it, not edit this)']
    used = set()
    # Cards.
    for card in json.loads(base_text('content/cards.json')):
        new = json.loads(json.dumps(card))
        if new['cost'] != card_price(new['cost']):
            note('cards', card['id'], 'cost', new['cost'], card_price(new['cost']))
            new['cost'] = card_price(new['cost'])
        for k in ('effects', 'passives', 'ability'):
            if k in new:
                walk_ops(new[k], 'cards', card['id'])
        lines += overlay_entries('cards', card['id'], card, new)
    # AI boards.
    for board in json.loads(base_text('content/ais.json')):
        new = json.loads(json.dumps(board))
        note('ais', board['id'], 'credit production (printed)', new['credits'], gain(new['credits']))
        new['credits'] = gain(new['credits'])
        for track in ('economy_track', 'play_track', 'insight_track', 'dispatch_track'):
            for i, step in enumerate(new.get(track, [])):
                scale_income_list(step.get('income', []), 'ais', board['id'], f'{track} step {i} income')
                if 'capstone' in step:
                    walk_ops(step['capstone'], 'ais', board['id'] + f' {track} capstone')
        for t in new:
            if t.endswith('_track') and t not in ('economy_track', 'play_track', 'insight_track', 'dispatch_track'):
                raise SystemExit(f'unhandled track {t}')
        for p in new.get('powers', []):
            scale_cost_list(p.get('cost', []), 'ais', f"{board['id']} {p['name']}", 'power')
            walk_ops(p.get('effects', []), 'ais', f"{board['id']} {p['name']}")
        walk_ops(new.get('innate', {}), 'ais', board['id'] + ' innate')
        edit_texts(new, used)
        lines += overlay_entries('ais', board['id'], board, new)
    # The map's country pieces (megaprojects and converters).
    for c in json.loads(base_text('content/map.json'))['countries']:
        new = json.loads(json.dumps(c))
        for k in new:
            if isinstance(new[k], (dict, list)):
                walk_ops(new[k], 'map', c['name'] + ' ' + k)
        edit_texts(new, used)
        lines += overlay_entries('corps', c['name'].lower(), c, new)
    missing = set(TEXT_EDITS) - used
    if missing:
        raise SystemExit(f'text edits that matched nothing: {missing}')
    return lines


def drafts(write):
    # Threat drafts: hand laid out; every credit amount there is a counter or a payment (a price).
    for name in ('threats_t1.json', 'threats_t2.json'):
        path = f'drafts/adversary/{name}'
        text = base_text(path)

        def sub(m):
            old = int(m.group(3))
            new = price(old)
            note('threats', name, 'credits counter or payment', old, new)
            return f'{m.group(1)}{m.group(2)}{new}'
        text2 = re.sub(r'("resource": "credits",)(\s*"amount": )(\d+)', sub, text)
        if write:
            open(os.path.join(ROOT, path), 'w', encoding='utf-8', newline='\n').write(text2)
    path = 'drafts/adversary/projects.json'
    data = json.loads(base_text(path))
    for p in data['cards']:
        before = json.dumps(p)
        walk_ops(p.get('effects', []), 'projects', p['id'])
        if json.dumps(p) != before:
            for old, new in ((10, 7),):
                p['text'] = p['text'].replace(f'pays {old} credits', f'pays {new} credits')
            if 'pays 7 credits' not in p['text']:
                raise SystemExit(f"{p['id']}: text not updated")
    if write:
        open(os.path.join(ROOT, path), 'w', encoding='utf-8', newline='\n').write(
            json.dumps(data, indent=1, ensure_ascii=False) + '\n')


BASELINE_PRICES = ['troop_credits', 'ambassador_credits', 'botnet_credits', 'city_energy_credits']
BASELINE_GAINS = ['sell_credits']


def baseline(write):
    path = 'configs/baseline.json'
    text = base_text(path)
    for k in BASELINE_PRICES + BASELINE_GAINS:
        m = re.search(rf'"{k}": (\d+)', text)
        old = int(m.group(1))
        new = price(old) if k in BASELINE_PRICES else gain(old)
        note('baseline', k, 'standard action' if k != 'sell_credits' else 'a sold card', old, new)
        text = text[:m.start()] + f'"{k}": {new}' + text[m.end():]
    m = re.search(r'"start_credits": \[([\d,\s]+)\]', text)
    vals = [int(v) for v in m.group(1).replace('\n', ' ').split(',')]
    new = [gain(v) for v in vals]
    note('baseline', 'ai.start_credits', 'starting credits by board', vals, new)
    inner = m.group(1)
    for o, n in zip(vals, new):
        inner = re.sub(rf'\b{o}\b', str(n), inner, count=1) if o != n else inner
    text = text[:m.start(1)] + inner + text[m.end(1):]
    # cards.industry_every: 1 credit off a building per N Industry tags -> per 3 (exact: 0.5 / 1.5).
    m = re.search(r'"industry_every": (\d+)', text)
    note('baseline', 'cards.industry_every', '1 credit off buildings per N Industry tags: N', int(m.group(1)), 3)
    text = text[:m.start()] + '"industry_every": 3' + text[m.end():]
    if write:
        open(os.path.join(ROOT, path), 'w', encoding='utf-8', newline='\n').write(text)


# values.json: ops rows that are a credit value (÷ 1.5); the rest are shares of a resource's rate or of another
# row (unchanged). Classified by how agents/value.rs reads each row.
OPS_SHARE = {
    'gain', 'gain_per_node', 'choose_player', 'other_gain', 'transfer', 'place_node', 'damage', 'damage_country',
    'gain_ambassadors', 'place_ambassadors', 'target', 'typed_jam', 'production', 'host_more', 'production_per_tag',
    'add_tokens', 'if_tags', 'if_node_next', 'if_node_next_fresh', 'ambassador_to_spot', 'draw', 'gain_per_tokens',
    'cash_out', 'gain_per_tag', 'sway', 'players_lose', 'lose_node_or', 'requisition', 'team_income',
    'free_purchases', 'one_player', 'per_site', 'remove_where', 'income', 'income_per_tag', 'building_discount',
    'logistics', 'convert_upkeep', 'card_discount', 'when', 'threshold', 'discard_hand', 'troop_steps',
    'standing_troops_bonus', 'threat_compute', 'discard', 'lose_ambassadors_or', 'lose_income', 'lose_or', 'theft',
}
RATES_CREDIT = {'node', 'node_late', 'movement', 'sway', 'drawn_card', 'keep_tier1', 'keep_tier2', 'rebate_span',
                'dead_card'}


def num(x):
    v = round(x / F, 3)
    return str(int(v)) if v == int(v) else repr(v)


def values(write):
    path = 'configs/agents/values.json'
    text = base_text(path)
    data = json.loads(text)

    def sect(name):
        a = text.index(f'"{name}": ')
        return a, a + len(f'"{name}": ')

    def replace_in(span_start, key, old):
        nonlocal text
        pat = re.compile(rf'("{re.escape(key)}": )(-?[\d.]+)')
        m = pat.search(text, span_start)
        assert m and float(m.group(2)) == old, (key, old)
        text = text[:m.start(2)] + num(old) + text[m.end(2):]

    for k, v in data['resources'].items():
        if k != 'credits':
            replace_in(sect('resources')[1], k, v)
            note('values.json', 'resources', k, v, num(v))
    for k, v in data['tags'].items():
        replace_in(sect('tags')[1], k, v)
        note('values.json', 'tags', k, v, num(v))
    for k, v in data['tokens'].items():
        replace_in(sect('tokens')[1], k, v)
        note('values.json', 'tokens', k, v, num(v))
    for k in RATES_CREDIT:
        v = data['rates'][k]
        replace_in(sect('rates')[1], k, v)
        note('values.json', 'rates', k, v, num(v))
    ops_start = sect('ops')[1]
    for k, row in data['ops'].items():
        if k in OPS_SHARE:
            continue
        v = row['credits']
        if v == 0:
            continue
        a = text.index(f'"{k}": {{', ops_start)
        replace_in(a, 'credits', v)
        note('values.json', 'ops', k, v, num(v))
    a = sect('overrides')[1]
    for o in data['overrides']:
        b = text.index(f'"card": "{o["card"]}"', a)
        replace_in(b, 'credits', o['credits'])
        note('values.json', 'overrides', o['card'], o['credits'], num(o['credits']))
    if write:
        open(os.path.join(ROOT, path), 'w', encoding='utf-8', newline='\n').write(text)


def main():
    write = '--write' in sys.argv
    lines = content_overlay()
    open(os.path.join(HERE, 'scale.overlay'), 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
    drafts(write)
    baseline(write)
    values(write)
    out = ['# Every credit number scaled (generated by scale.py)', '',
           '| Source | Where | What | r127 | r128 |', '|---|---|---|---|---|']
    for s, w, what, o, n in rows:
        out.append(f'| {s} | {w} | {what} | {o} | {n} |')
    open(os.path.join(HERE, 'scale_table.md'), 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
    print(f'{len(rows)} numbers, {len(lines) - 1} overlay entries')


if __name__ == '__main__':
    main()

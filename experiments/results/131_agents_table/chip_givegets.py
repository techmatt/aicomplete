"""131: each chip card's give / get at the value table's rates, at several chip rates.

give = `printed` (credit cost, keep and other resources at the table), get = `reasoned` (its effects at the
table), both from `scripts/cards/effect_ledger.py --stdout --values` on a copy of values.json with `chips`
set. Prints a Markdown table. Usage: python chip_givegets.py [RATE ...] (default 1.5 2 2.5 3).
"""
import csv
import io
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
CHIP_CARDS = None  # every card whose text names chips


def chip_cards():
    cards = json.loads((ROOT / 'content' / 'cards.json').read_text(encoding='utf-8'))
    cards = cards['cards'] if isinstance(cards, dict) else cards
    return [d['id'] for d in cards if 'chips' in json.dumps(d)]


def ledger(rate):
    values = json.loads((ROOT / 'configs' / 'agents' / 'values.json').read_text(encoding='utf-8'))
    values['resources']['chips'] = rate
    out = ROOT / 'out' / f'values-chips-{rate}.json'
    out.parent.mkdir(exist_ok=True)
    out.write_text(json.dumps(values, indent=2), encoding='utf-8', newline='\n')
    text = subprocess.run([sys.executable, str(ROOT / 'scripts' / 'cards' / 'effect_ledger.py'), '--stdout',
                           '--values', str(out)], cwd=ROOT, capture_output=True, text=True, encoding='utf-8',
                          check=True).stdout
    table = text.split('\n\n')[0]
    return {r['card']: r for r in csv.DictReader(io.StringIO(table))}


def main():
    rates = [float(x) for x in sys.argv[1:]] or [1.5, 2.0, 2.5, 3.0]
    ids = chip_cards()
    by = {r: ledger(r) for r in rates}
    print('| card | ' + ' | '.join(f'give / get / net at {r}' for r in rates) + ' |')
    print('|---|' + '---|' * len(rates))
    for c in ids:
        cells = []
        for r in rates:
            row = by[r].get(c)
            if row is None:
                cells.append('-')
                continue
            give, get = float(row['printed']), float(row['reasoned'])
            cells.append(f'{give:.1f} / {get:.1f} / {get - give:+.1f}')
        print(f'| {c} | ' + ' | '.join(cells) + ' |')


if __name__ == '__main__':
    main()

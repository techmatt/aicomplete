"""268: the table-cost proxies by level on a read (design_check's table_cost), to set budgets.
Usage: python measure_read.py READ.jsonl [READ2.jsonl]"""
import json, os, sys
sys.path.insert(0, os.path.join(os.getcwd(), 'scripts'))
import design_check as dc  # noqa: E402

cfg = json.load(open('scripts/design_check.json', encoding='utf-8'))
countries = json.load(open('content/map.json', encoding='utf-8'))['countries']
for path in sys.argv[1:]:
    gs = [json.loads(l) for l in open(path, encoding='utf-8') if l.startswith('{')]
    print(path, len(gs))
    for d, lv in dc.by_level(gs).items():
        r = dc.table_cost(lv, cfg, countries)
        print(f'  d{d}: ' + ', '.join(f'{k} {v:.1f}' for k, v in r.items()), f'({len(lv)} games)')

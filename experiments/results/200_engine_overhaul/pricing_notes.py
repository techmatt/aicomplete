"""200: the new cards' pricing notes into docs/card-pricing-notes.json (one current line a card, keys sorted).

    python experiments/results/200_engine_overhaul/pricing_notes.py
"""
import json
import os

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
PATH = os.path.join(ROOT, 'docs', 'card-pricing-notes.json')
NOTES = {
    'penning_trap_array': 'r88 (200): antimatter plant, research cubes; Action 1 cube + 1 energy -> 2 antimatter (give 7.5, get 14 at values.json; 10.5 / 14 at the anchors); values.json: pay 12.5; get 19.4 (+6.9); anchors: pay 22 (10 cr, 3 ins), get 7 when built + 4 uses x 3.5 + tag 1 = 22 (1.0).',
    'antiproton_foundry': 'r88 (200): antimatter plant, chips; Action 1 chip + 1 energy -> 2 antimatter (give 8.5, get 14 at values.json; 10.5 / 14 at the anchors); values.json: pay 12.5; get 17.6 (+5.1); anchors: pay 22 (10 cr, 3 ins), get 22 (1.0).',
    'cryo_sample_vault': 'r88 (200): cube source, +1 research cube a round; values.json: pay 9.5; get 13.0 (+3.5); anchors: pay 15 (7 cr, 2 ins), get 1 cube a round x 4 rounds x 0.8 x 6 = 19 (1.27).',
    'field_assay_team': 'r88 (200): cube sink into the purge, Action 1 cube -> remove 2 agents in one hex in reach; values.json: pay 8.5; get 4.9 at the table, 10.9 with its overrides row (+6: the table prices 2 agents at 5, anchor 10); anchors: pay 14 (6 cr, 2 ins), get 4 uses x (10 - 6) = 16 (1.14).',
}
with open(PATH, encoding='utf-8') as f:
    notes = json.load(f)
notes.update(NOTES)
with open(PATH, 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(dict(sorted(notes.items())), indent=2, ensure_ascii=False) + '\n')
print('wrote', PATH)

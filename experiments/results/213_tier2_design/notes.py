"""213: the card viewer's pricing note for each card 213 reworked or added (docs/card-pricing-notes.json, keys sorted).
Run from the repo root: python experiments/results/213_tier2_design/notes.py
"""
import json

P = 'docs/card-pricing-notes.json'
NOTES = {
    'dyson_swarm_tranche': 'r94 (213): a major: 12 cr, 1 am, 1 ins, needs 2 Power: 10 energy, +2 energy income; anchors: pay 42; get 73 = 1.74; values.json: pay 32.5; get 64.5.',
    'recursive_self_improvement': 'r94 (213): a major: 5 cr, 1 am, 2 ins: +3 insight income, draw 3; anchors: pay 39; get 51 = 1.31; values.json: pay 28; get 32.3.',
    'peacekeeping_mission': 'r94 (213): a major: 20 cr, 1 am: liberate, up to 3 troops in its country; anchors: pay 46; get 60 = 1.30; values.json: pay 38; get 59.',
    'root_access': 'r94 (213): a major: 18 cr, 1 am: liberate, a botnet by it; anchors: pay 44; get 61 = 1.39; values.json: pay 36; get 57.',
    'hostile_buyout': 'r94 (213): a major: 22 cr, 1 am: liberate, 3 ambassadors; anchors: pay 48; get 69 = 1.44; values.json: pay 40; get 62.',
    'decapitation_strike': 'r94 (213): a troop major: 4 cr, 1 am: up to 3 troops in one country, a botnet in reach; anchors: pay 30; get 31 = 1.03; values.json: pay 22; get 20.3 + 6 (override: a botnet at 8 against its anchor 16).',
    'collider_consortium': 'r94 (213, new): a major: 8 cr, 1 am, 1 ins: 8 research cubes, another player 3; anchors: pay 38; get 66 = 1.74; values.json: pay 28.5; get 31.8.',
    'catalyzed_fab_run': 'r94 (213, new): a major: 2 cr, 1 am: 6 chips, another player 2; anchors: pay 28; get 48 = 1.71; values.json: pay 20; get 30.',
    'antimatter_accord': 'r94 (213, new): a major: 8 cr, 1 am, 1 ins: 6 ambassadors, 4 Unity; anchors: pay 38; get 64 = 1.68; values.json: pay 28.5; get 41.',
    'cyclotron_lease': 'r94 (213, new): a dear tier-1 source: 22 cr; Action: 2 energy for 1 antimatter (+9 a use at the anchors); 4 uses: pay 62; get 72 = 1.16; values.json: pay 22; get 25.2.',
    'accelerator_diplomacy': 'r94 (213, new): a dear tier-1 spend: 4 cr, 1 am: 5 Unity, 2 ambassadors; anchors: pay 26; get 36 = 1.38; values.json: pay 22; get 28.',
}
d = json.load(open(P, encoding='utf-8'))
d.update(NOTES)
with open(P, 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(dict(sorted(d.items())), indent=2, ensure_ascii=False) + '\n')
print(len(d), 'notes')

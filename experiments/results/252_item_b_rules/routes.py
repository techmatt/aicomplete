"""252: node routes (Matt's ruling 1). Each board: a base route (Expand, no cost beyond the track step's
energy), its middle route (Buy In, Accord, Peer Review re-costed off research cubes) and a top route in its own
currency. Idempotent: rerunning leaves the file as it is."""
import json
import pathlib

P = pathlib.Path(__file__).resolve().parents[3] / 'content' / 'ais.json'
EXPAND = {"name": "Expand", "tags": [], "text": "place 1 Node", "effects": [{"op": "place_node", "count": 1}], "cost": []}
TOP = {
    'builder': {"name": "Annex", "tags": [], "text": "place 1 Node within 2 of one of yours",
                "effects": [{"op": "place_node", "count": 1, "range": 2}], "cost": [["chips", 1]]},
    'protector': {"name": "Outpost", "tags": [], "text": "place 1 Node; gain 1 ambassador",
                  "effects": [{"op": "place_node", "count": 1}, {"op": "gain", "resource": "ambassadors", "amount": 1}],
                  "cost": [["energy", 3]]},
    'researcher': {"name": "Field Survey", "tags": [], "text": "place 1 Node; draw 1 card",
                   "effects": [{"op": "place_node", "count": 1}, {"op": "draw", "count": 1}],
                   "cost": [["insight", 3]]},
}
MIDDLE = {'builder': 'Buy In', 'protector': 'Accord', 'researcher': 'Peer Review'}

d = json.loads(P.read_text(encoding='utf-8'))
for b in d:
    ps = b['powers']
    names = [p['name'] for p in ps]
    mid = names.index(MIDDLE[b['id']])
    if b['id'] == 'researcher':
        ps[mid]['cost'] = [["insight", 2]]
    if 'Expand' not in names:
        ps.insert(mid, dict(EXPAND))
        mid += 1
    top = TOP[b['id']]
    if top['name'] in names:
        ps[names.index(top['name']) if 'Expand' in names else mid + 1] = top
    else:
        ps.insert(mid + 1, top)
P.write_text(json.dumps(d, indent=1, ensure_ascii=False) + '\n', encoding='utf-8', newline='\n')
print('ok')

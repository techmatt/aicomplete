"""121: give 117's four new cards a scene of their own, drawn new in batch 8.

Edits tools/card-art/scenes.json through scenes_io (its hand-laid layout kept).
Run once from the repo root: python experiments/results/121_loose_threads/assign_art.py
"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path('tools/card-art').resolve()))
import scenes_io as sio  # noqa: E402

s = sio.load()
cards = {c['id']: c for c in json.loads(Path('content/cards.json').read_text(encoding='utf-8'))}

# card: (tag group, new scene id, subject)
NEW = {
    'fabrication_megaplex': ('industry', 'megaplex_fabrication_floor', "A vast modern fabrication megaplex seen from a high gantry, rows of robotic assembly cells under a long sawtooth roof running to the far wall, conveyor lines carrying finished parts and trays of green circuit chips, small workers on the floor below; no flags or emblems."),
    'frontier_lab_charter': ('science', 'frontier_lab_opening', "A brand-new frontier research lab of glass and white steel opening on a hilltop campus, scientists in lab coats carrying the first crates of glowing sky-blue research cubes through its tall doors while a founder holds up a blank rolled charter to a small cheering crowd; no flags or emblems."),
    'national_grid_buildout': ('power', 'grid_buildout_crews', "A national power grid being built across open country, crews and cranes raising a long line of new lattice pylons over farmland toward the horizon, cable drums on trucks, three fresh substations taking shape along the route, a power station far off; no flags or emblems."),
    'decapitation_strike': ('security', 'strike_on_the_rogue_hub', "A security team in coral-trimmed tactical gear storming a hijacked rooftop server hub in daylight, one pulling the thick cables from a glowing cold-blue machine core while others lead a line of dark-suited adversary envoys away, figures seen from behind; no flags or emblems."),
}


def partner(card):
    """A deck card of the same first tag and the other window shape, for the batch's sample pair."""
    c = cards[card]
    want_building = c['kind'] != 'building'
    held = sorted(sc['card'] for g in s['scenes'].values() for sc in g.values() if 'card' in sc)
    for other in held:
        o = cards.get(other)
        if o and other not in NEW and o['tags'][:1] == c['tags'][:1] and (o['kind'] == 'building') == want_building:
            return other
    raise SystemExit(f'no partner for {card}')


batch = {}
for card, (tag, sid, subject) in NEW.items():
    assert all(sid not in g for g in s['scenes'].values()), sid
    s['scenes'][tag][sid] = {'subject': subject}
    old = sio.assign(s, card, sid)
    p = partner(card)
    batch[sid] = [p, card] if cards[card]['kind'] == 'building' else [card, p]
    print(card, old, sid, sep='\t')

s['round'] = 'b8'
s['picker'] = 'scene_batch_8'
s['batch'] = batch
sio.save(s)
print('batch', json.dumps(batch))

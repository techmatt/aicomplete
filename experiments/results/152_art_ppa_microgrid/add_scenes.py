"""152: six candidate scenes, three each for Power Purchase Agreement and Microgrid Lab Compact.

Run once from the repo root on the r65 scenes.json: `python experiments/results/152_art_ppa_microgrid/add_scenes.py`.
Edits through scenes_io (one scene a line), never by hand:
- adds the six under `power` (both cards' first tag), each `for` the card it was written for;
- sets `round` b9, `picker` art_d69 and `batch` to the six, each on the two cards;
- notes `for` in `_note`.
`--assign` then suggests each card's first candidate (provisional until Matt's pick replaces it);
`--note` points `_note`'s list of earlier batches at art_d69.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'tools' / 'card-art'))
import scenes_io as sio  # noqa: E402

PPA = 'power_purchase_agreement'
MLC = 'microgrid_lab_compact'
NO = 'no flags, logos or emblems.'
SCENES = {
    'ppa_handshake_at_the_truck': (PPA, 'A power-company engineer in an amber hard hat and a research-station director in a field jacket shaking hands over a signed blank contract laid on the hood of a utility truck, a crew behind them unspooling a thick new cable from a small substation across a meadow to a domed research station; ' + NO),
    'ppa_contract_at_the_wind_farm': (PPA, 'Two negotiators at a folding table set out in the grass beneath a row of wind turbines, one sliding a signed blank contract across to the other, a single amber cable running from the nearest turbine past the table to a domed research station on the next rise, crates of glowing sky-blue research cubes waiting at its door; ' + NO),
    'ppa_meter_handover': (PPA, 'At the gate of a domed research station, a utility clerk handing a large brass power meter and a rolled blank contract to a smiling scientist, a line of fresh wooden poles carrying new wire back over the hills to a distant power plant; ' + NO),
    'microgrid_lab_battery_yard': (MLC, 'A self-powered research campus in a green valley, low white laboratories set around a central yard of battery cabinets and a small control hut, two wind turbines on the slope and a little river-turbine house on the stream feeding it, an engineer checking a panel of amber lamps while scientists wheel crates of glowing sky-blue research cubes between the labs; no solar panels, ' + NO),
    'microgrid_lab_control_room': (MLC, "Inside a bright research campus's own power control room, a tall window looking out over its battery yard, wind turbines and white lab buildings, an engineer pulling a large lever as a row of amber lamps lights up on a hand-drawn map of the campus grid, a scientist alongside holding a glowing sky-blue research cube; no solar panels, " + NO),
    'microgrid_lab_island': (MLC, 'A small research lab campus on a rocky island running on its own grid, tidal turbines churning in the channel, a stone battery house and a few wind turbines on the cliffs, cables climbing to a domed research station on the headland where scientists unload crates of glowing sky-blue research cubes from a boat; no solar panels, ' + NO),
}
NOTE = ' A scene with `for` was drawn as a candidate for that card (152) and is suggested for none; a pick moves it to `card`.'


OLD_BATCHES = 'Earlier batches are in picks/scene_batch_<n>.json, whose row ids are the scenes already drawn.'
NEW_BATCHES = 'Earlier batches are in picks/scene_batch_<n>.json (batch 9: picks/art_d69.json), whose row ids (art_d69: option keys) are the scenes already drawn.'


def main(argv):
    s = sio.load()
    if argv == ['--note']:
        s['_note'] = s['_note'].replace(OLD_BATCHES, NEW_BATCHES)
        sio.save(s)
        return 0
    if argv == ['--assign']:
        for card in (PPA, MLC):
            sid = next(i for i, (c, _) in SCENES.items() if c == card)
            sc = s['scenes']['power'][sid]
            sc.pop('for', None)
            print(f'{card}: {sio.assign(s, card, sid) or "(no scene)"} -> {sid}')
        sio.save(s)
        return 0
    power = s['scenes']['power']
    for sid, (card, subject) in SCENES.items():
        if sid in power:
            raise SystemExit(f'{sid} already in scenes.json')
        power[sid] = {'for': card, 'subject': subject}
    s['round'], s['picker'] = 'b9', 'art_d69'
    s['batch'] = {sid: [PPA, MLC] for sid in SCENES}
    if NOTE not in s['_note']:
        s['_note'] += NOTE
    sio.save(s)
    print(f'added {len(SCENES)} scenes, batch b9, picker art_d69')
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))

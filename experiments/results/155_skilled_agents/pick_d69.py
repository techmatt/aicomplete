"""155 (parked P1): Matt's art_d69 pick (2026-11-08), through scenes_io.

Power Purchase Agreement keeps `ppa_handshake_at_the_truck` and Microgrid Lab Compact
`microgrid_lab_battery_yard` (152 set them as each card's provisional scene); the four unpicked
candidates leave the pool, and the batch holds the two picks. Run once from the repo root.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'tools' / 'card-art'))
import scenes_io as sio  # noqa: E402

PICKS = {'power_purchase_agreement': 'ppa_handshake_at_the_truck',
         'microgrid_lab_compact': 'microgrid_lab_battery_yard'}
DROP = ['ppa_contract_at_the_wind_farm', 'ppa_meter_handover',
        'microgrid_lab_control_room', 'microgrid_lab_island']


def main():
    s = sio.load()
    for card, sid in PICKS.items():
        tag = sio.find(s, sid)
        sc = s['scenes'][tag][sid]
        sc.pop('for', None)
        sc.pop('provisional', None)
        if sc.get('card') != card:
            sio.assign(s, card, sid)
    for sid in DROP:
        tag = sio.find(s, sid)
        if s['scenes'][tag][sid].get('card'):
            raise SystemExit(f'{sid} is suggested for a card')
        del s['scenes'][tag][sid]
    s['batch'] = {sid: v for sid, v in s['batch'].items() if sid not in DROP}
    sio.save(s)
    print('picked:', ', '.join(f'{c} -> {sid}' for c, sid in PICKS.items()), '; dropped', len(DROP))


if __name__ == '__main__':
    main()

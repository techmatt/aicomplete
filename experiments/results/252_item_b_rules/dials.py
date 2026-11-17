"""252: the board dials (Matt's rulings 1, 2, 5), set by text so the configs keep their layout. Idempotent."""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[3]
DIALS = {
    'owned_station_cubes': 1,   # ruling 2: an owned station makes 1 research cube a round
    'station_chips': 2,         # ruling 2: Build a research station at 2 chips
    'node_credits': 0,          # ruling 1: Place a node cut (0 with node_energy 0: no such action)
    'botnet_credits': 20,       # ruling 5: 1.25 x the median botnet card (15) is 18.75
    'claim_station': 4,         # ruling 5: 1.25 x the median station card (19.5) is 24.4; 4 ambassadors is 32
    'claim_factory': 3,         # ruling 5: over the anchors at 2 (16 for a factory's 19)
}
NEW = {'insight_cubes': ('cube_insight', 2)}  # ruling 5: the cube converter taxed, 2 cubes an insight

for name in sys.argv[1:] or ['configs/baseline.json', 'configs/three_seat.json']:
    p = ROOT / name
    s = p.read_text(encoding='utf-8')
    for k, v in DIALS.items():
        s, n = re.subn(rf'("{k}": )\d+', rf'\g<1>{v}', s, count=1)
        assert n == 1, (name, k)
    for k, (after, v) in NEW.items():
        if f'"{k}"' not in s:
            s, n = re.subn(rf'(\n(\s*)"{after}": \d+,)', rf'\1\n\g<2>"{k}": {v},', s, count=1)
            assert n == 1, (name, k)
    p.write_text(s, encoding='utf-8', newline='\n')
print('ok')

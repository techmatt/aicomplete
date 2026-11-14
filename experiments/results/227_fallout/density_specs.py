"""227: write the density screen's specs: f227.json's grid with content overlays that take Fallout lines back off
(the original line restored), so one build screens every density.

    python experiments/results/227_fallout/density_specs.py   -> out/227_fallout/f227-<level>.json
"""
import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SPEC = json.loads((ROOT / 'experiments/farm/f227.json').read_text(encoding='utf-8'))

SWAY2 = {"op": "sway", "side": "discord", "amount": 2, "narrative": "fear"}
OFF = {
    'agitators': ['threats.agitators.effects=' + json.dumps([{"op": "create", "count": 3, "at": "location"}, SWAY2])],
    'provocation': ['threats.provocation.effects=' + json.dumps(
        [{"op": "deal_threats", "count": 1, "tier": 1, "scope": "country"}, SWAY2])],
    'asset_seizure': ['projects.asset_seizure.effects=' + json.dumps([{"op": "take_site", "site": "factory"}])],
    'station_coup': ['projects.station_coup.effects=' + json.dumps([{"op": "take_site", "site": "station"}])],
    'infiltration_drive': ['projects.infiltration_drive.effects=' + json.dumps(
        [{"op": "rally", "site": "city", "count": 3}, {"op": "corrupt", "site": "city", "cut": 1}])],
    'casus_belli_1': ['threats.casus_belli.effects=' + json.dumps([{"op": "extra_war"}, {"op": "fallout"}])],
}
LEVELS = {
    'mid': ['agitators', 'asset_seizure'],
    'lo': ['agitators', 'asset_seizure', 'provocation', 'station_coup'],
    'lo3': ['agitators', 'asset_seizure', 'provocation', 'infiltration_drive'],
    'lo2': ['agitators', 'asset_seizure', 'provocation', 'infiltration_drive', 'casus_belli_1'],
}


def main():
    out = ROOT / 'out/227_fallout'
    out.mkdir(parents=True, exist_ok=True)
    for name, offs in LEVELS.items():
        spec = copy.deepcopy(SPEC)
        spec['name'] = f'f227-{name}'
        extra = [s for o in offs for s in OFF[o]]
        for v in spec['variants']:
            v['set'] = v['set'] + extra
        path = out / f'f227-{name}.json'
        path.write_text(json.dumps(spec, indent=2) + '\n', encoding='utf-8', newline='\n')
        print(path)


if __name__ == '__main__':
    main()

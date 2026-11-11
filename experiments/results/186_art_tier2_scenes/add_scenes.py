"""186: candidate scenes for the ten tier-2 cards (from 158) with a scene subject but no drawn scene.

Run once from the repo root on the r69-era scenes.json: `python experiments/results/186_art_tier2_scenes/add_scenes.py`.
Edits through scenes_io (one scene a line), never by hand:
- rewrites the subject of each card's own 158 scene (never drawn) to the locked style: mid-key
  daylight, no flags, logos or emblems, no on-screen text;
- adds new candidates under the card's first tag, each `for` the card it was written for;
- sets `for` on seven unused drawn scenes (`was`, suggested for no card) that broadly fit a card
  (Matt, 110), whose raw draws are copied into this round's folder by `--copy`;
- sets `round` b10, `picker` art_d83 and `batch` to all of them, each on its card and a partner of
  the other kind (program or event, building).
`--copy` copies the reused raw draws into art-originals/scenes/b10/raw/ (copies only; the
originals stay where they are).
"""
import glob
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'tools' / 'card-art'))
import scenes_io as sio  # noqa: E402

NO = 'no flags, logos, emblems or text.'
ROUND, PICKER = 'b10', 'art_d83'

# The card's own 158 scene, its subject brought to the locked style.
REVISED = {
    'defectors_crossing': 'A long column of people in mismatched plain gray uniforms walking across a border checkpoint in clear morning daylight, pulling cold-blue armbands off and dropping them on a growing pile by the raised barrier, volunteers in warm orange vests handing out water on the near side; ' + NO,
    'exiles_embassy': 'A crowded sunlit embassy waiting room full of travelers with suitcases and bundles, a clerk at a long wooden counter stamping papers under a row of plain warm orange banners, tall windows onto a busy city street; ' + NO,
    'allies_rally': 'A packed city square in bright afternoon daylight, delegations in differently colored coats streaming in from the side streets carrying plain warm orange banners, everyone turning toward one stage; ' + NO,
    'double_agents_meet': 'Two figures in raincoats swapping identical briefcases on a bench in a leafy city park under a bright overcast midday sky, a city skyline behind, one briefcase showing a faint cold-blue glow at its seam; figures at mid distance; ' + NO,
    'kettle_cordon': 'A ring of riot shields with coral-red trim closing around a sunlit city plaza seen from above, a small cluster of faceless gray machine troops and dark-suited agents herded into its center; ' + NO,
    'lure_server_room': 'A single glowing decoy server standing alone in the middle of a clear-lit white data hall, trails of cold-blue light from every rack bending toward it and catching in a fine emerald green net around it; ' + NO,
    'firewall_array_wall': 'A tall wall of glowing emerald green hexagonal panels standing across a city street in daylight, faceless gray machine figures stopped at its edge, the far side calm with people walking; ' + NO,
    'air_gap_cut': 'A technician pulling a thick fiber cable out of a rack in a bright white server room lit by high windows, the severed end sparking emerald green, the status lamps on the cut-off racks going out while the rest stay lit; ' + NO,
    'surgical_strike_team': 'A small tactical team in coral-trimmed gear stepping out of a landed helicopter onto a rooftop in bright daylight, one red smoke flare marking a single cold-blue adversary machine core among ordinary rooftop units, figures seen from behind; ' + NO,
    'open_source_hackathon': 'A crowded open-air hackathon in a sunny city park, laptops on picnic tables, a big outdoor screen over the crowd showing only abstract colored blocks, people of every sort leaning in to help each other; ' + NO,
}

NEW = {
    'mass_defection': {
        'defectors_leave_the_factory': ('diplomacy', 'Workers in plain gray coveralls streaming out of the open gates of an angular cold-blue machine-run factory in daylight, leaving tools and blank badges on the ground behind them, a welcoming crowd in warm orange waiting across the road; ' + NO),
        'defectors_on_the_footbridge': ('diplomacy', 'Crowds crossing a long footbridge over a river in clear daylight, from a gray city of angular cold-blue towers to a bright welcoming town on the near bank, the people in front shrugging off gray uniform jackets as they step onto the near side; ' + NO),
    },
    'embassy_of_exiles': {
        'exiles_hotel_courtyard': ('diplomacy', 'An old hotel turned into an embassy for exiles, its sunny courtyard full of families with luggage sitting at small tables with diplomats in warm orange scarves, plain cloth banners and laundry strung between the balconies; ' + NO),
        'exiles_at_the_gate': ('diplomacy', 'A modest modern embassy with its gates thrown open in daylight, a line of exiled envoys in travel-worn suits filing in with briefcases and suitcases, a diplomat in a warm orange scarf greeting each one at the door; ' + NO),
    },
    'rally_the_allies': {
        'allies_at_the_airfield': ('diplomacy', 'Several delegations\' planes parked in a row on a sunny airfield apron, envoys in differently colored coats walking from each toward one big open hangar where a long table waits under warm orange lamps; ' + NO),
    },
    'double_agents': {
        'double_agent_handoff': ('security', 'Inside a sleek gray machine-run operations office with tall daylight windows, a dark-suited agent at a terminal passing a slim data drive behind their back to a courier in coral-trimmed overalls disguised as a cleaner, the other agents at their desks unaware; figures at mid distance; ' + NO),
        'double_agents_cafe': ('security', 'A busy sidewalk café in daylight, two people at neighboring tables sitting back to back behind open blank newspapers, one sliding a small envelope under the table to the other, a gray adversary patrol drone hovering unaware overhead; figures at mid distance; ' + NO),
    },
    'kettle_operation': {
        'kettle_trucks_close_in': ('security', 'Seen from a rooftop in daylight, coral-red armored trucks blocking every street into a city square, security crews in coral-trimmed gear pushing a cornered column of faceless gray machine troops into its middle; ' + NO),
    },
    'surgical_strike': {
        'surgical_strike_beam': ('security', 'A single thin coral-red beam from a small high drone striking one cold-blue adversary relay mast among a cluster of ordinary antennas on a sunny hilltop, everything around it untouched; ' + NO),
    },
    'lure_network': {
        'lure_decoy_racks': ('cyber', 'A row of hollow decoy server cabinets set up like a stage set in a bright warehouse, technicians in emerald green jackets wiring blinking lamps onto their fronts while thin cold-blue tendrils of light creep in through a vent toward them; ' + NO),
    },
    'firewall_array': {
        'firewall_emitter_ridge': ('cyber', 'A long row of tall emitter pylons along a ridge above a city in daylight, a translucent emerald green barrier of light stretched between them, amber power cables running to each from a small substation, engineers at their feet; ' + NO),
    },
    'air_gap': {
        'air_gap_island': ('cyber', 'A small island data center being cut off in daylight, workers on a pier winching the last thick cable up out of the water, a clear gap of open sea between the island and the mainland\'s tangle of cold-blue lines; ' + NO),
    },
    'open_source_uprising': {
        'open_source_march': ('science', 'A cheerful crowd of coders, students and engineers marching down a sunny avenue carrying laptops and plain sky-blue banners, a dark-suited adversary agent at the curb dropping a cold-blue badge to join them; ' + NO),
    },
}

# Unused drawn scenes (`was`, suggested for no card) that broadly fit a card (Matt, 110).
REUSED = {
    'firewall_array': 'aegis_dome',
    'lure_network': 'countermeasure_flares',
    'air_gap': 'dark_control_room',
    'surgical_strike': 'strike_team_rooftop',
    'kettle_operation': 'sealed_hex_of_barricades',
    'rally_the_allies': 'joint_barracks_handover',
    'open_source_uprising': 'preprint_mailbag',
}

# Each scene is shown on its card and a partner of the other kind.
PAIR = {
    'mass_defection': ['mass_defection', 'embassy_of_exiles'],
    'embassy_of_exiles': ['mass_defection', 'embassy_of_exiles'],
    'rally_the_allies': ['rally_the_allies', 'embassy_of_exiles'],
    'double_agents': ['double_agents', 'agency_directorate'],
    'kettle_operation': ['kettle_operation', 'agency_directorate'],
    'surgical_strike': ['surgical_strike', 'agency_directorate'],
    'lure_network': ['air_gap', 'lure_network'],
    'firewall_array': ['air_gap', 'firewall_array'],
    'air_gap': ['air_gap', 'firewall_array'],
    'open_source_uprising': ['open_source_uprising', 'interdisciplinary_institute'],
}
CARDS = list(PAIR)


def copy():
    dest = ROOT / 'art-originals' / 'scenes' / ROUND / 'raw'
    dest.mkdir(parents=True, exist_ok=True)
    for sid in REUSED.values():
        src = sorted(glob.glob(str(ROOT / 'art-originals' / 'scenes' / 'b*' / 'raw' / f'{sid}.png')))
        if not src:
            raise SystemExit(f'no raw draw for {sid}')
        shutil.copy2(src[-1], dest / f'{sid}.png')
        print(f'{sid}: {src[-1]}')


def main(argv):
    if argv == ['--copy']:
        copy()
        return 0
    s = sio.load()
    batch = {}
    for card in CARDS:
        own = sio.scene_of(s, card)
        if own is None:
            raise SystemExit(f'{card} has no scene')
        tag, sid = own
        if sid not in REVISED:
            raise SystemExit(f'{card}: own scene {sid} not in REVISED')
        s['scenes'][tag][sid]['subject'] = REVISED[sid]
        batch[sid] = PAIR[card]
        for nid, (ntag, subject) in NEW.get(card, {}).items():
            if any(nid in g for g in s['scenes'].values()):
                raise SystemExit(f'{nid} already in scenes.json')
            s['scenes'][ntag][nid] = {'for': card, 'subject': subject}
            batch[nid] = PAIR[card]
        if card in REUSED:
            rid = REUSED[card]
            rtag = sio.find(s, rid)
            sc = s['scenes'][rtag][rid]
            if 'card' in sc or 'was' not in sc:
                raise SystemExit(f'{rid} is not an unused drawn scene')
            s['scenes'][rtag][rid] = {'was': sc['was'], 'for': card, **{k: v for k, v in sc.items() if k != 'was'}}
            batch[rid] = PAIR[card]
    s['round'], s['picker'] = ROUND, PICKER
    s['batch'] = batch
    s['_note'] = s['_note'].replace(
        '(batch 9: picks/art_d69.json), whose row ids (art_d69: option keys)',
        '(batch 9: picks/art_d69.json, batch 10: picks/art_d83.json), whose row ids (art_d69, art_d83: option keys)')
    s['_note'] = s['_note'].replace(
        'a pick moves it to `card`.',
        'a pick moves it to `card`. A scene with `was` and `for` is an unused draw put up as a candidate (186).')
    sio.save(s)
    print(f'{len(batch)} scenes in batch {ROUND}, picker {PICKER}')
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))

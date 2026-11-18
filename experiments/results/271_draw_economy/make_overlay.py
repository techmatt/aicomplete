"""271: write the draw-economy overlay (draw.overlay) from content/cards.json at r126. Run from the repo root.

    python experiments/results/271_draw_economy/make_overlay.py BASE_CARDS_JSON

BASE_CARDS_JSON is r126's content/cards.json (git show a4196fe193:content/cards.json), so a rerun on the
edited tree writes the same file. Each on-play draw is set to its new count (NEW; absent = 0) and the card's
credit cost drops by the removed cards at values.json's drawn_card rate (4), floored at FLOOR. The hand edits
(engines, any-card triggers, the Builder's loop, converters, megaproject payouts, card text) are in HAND.
"""
import csv
import io
import json
import subprocess
import sys

DRAWN = 4  # configs/agents/values.json rates.drawn_card
FLOOR = 3

# The scale (Matt's ruling): 1 good, 2 very strong, 3 exceptional (about 2 cards in the game).
NEW = {
    # exceptional
    'zero_day_auction': 3, 'recursive_self_improvement': 3,
    # very strong: Science, Cyber, Diplomacy
    'breakthrough_leak': 2, 'foundation_model_campus': 2, 'data_licensing_deal': 2,
    'envoy_briefings': 2, 'orbital_embassy': 2, 'shadow_broker_feed': 2,
    # good: Science
    'predictive_modeling': 1, 'field_sensor_grid': 1, 'moonshot_grant': 1, 'grad_student_pool': 1,
    'citation_engine': 1, 'shared_compute_grant': 1, 'synthetic_data_forge': 1,
    # Cyber
    'exascale_training_cluster': 1, 'dataset_scraper': 1, 'darknet_exchange': 1, 'exploit_broker': 1,
    'bug_bounty_board': 1,
    # Diplomacy
    'protocol_office': 1, 'fusion_commons': 1,
    # a few one-time draws: Industry, Power, Security
    'materials_hedge_desk': 1, 'supplier_network': 1,
    'demand_response_grid': 1, 'wafer_fab_line': 1,
    'recruit_operatives': 1, 'headquarters_raid': 1,
}

# Engines and triggers: whole fields set by hand (their on-play draw goes by NEW, absent = 0).
HAND = [
    # Science engine: the chain's end, AI Central's shape.
    'cards.preprint_server.needs=[["science",4]]',
    'cards.preprint_server.insight=2',
    # Cyber engine: a paid Action.
    'cards.command_and_control_grid.needs=[["cyber",3]]',
    'cards.command_and_control_grid.passives=null',
    'cards.command_and_control_grid.ability={"cost":[["energy",1]],"effects":[{"op":"draw","count":2}]}',
    # Diplomacy engine: draw through other players' Diplomacy plays.
    'cards.consulate_tower.needs=[["diplomacy",3]]',
    'cards.consulate_tower.passives=[{"op":"when","on":{"event":"team_play","tag":"diplomacy"},"then":[{"op":"draw","count":1}]}]',
    # Power and Security lose their repeatable draws.
    'cards.fusion_research_campus.passives=[{"op":"when","on":{"event":"play","tag":"power"},"then":[{"op":"gain","resource":"energy","amount":1}]}]',
    'cards.supplier_network.passives=[{"op":"when","on":{"event":"play","tag":"industry"},"then":[{"op":"gain","resource":"credits","amount":2}]}]',
    'cards.reconciliation_office.passives=[{"op":"when","on":{"event":"remove_ambassadors"},"then":[{"op":"gain","resource":"credits","amount":3}]}]',
    # Repeatable Action draws off the three chains' ends become gains worth more than their cost.
    'cards.orbital_observatory_array.ability={"cost":[["energy",1]],"effects":[{"op":"gain","resource":"insight","amount":2}]}',
    'cards.demand_response_grid.ability={"cost":[["energy",1]],"effects":[{"op":"gain","resource":"credits","amount":6}]}',
    'cards.positron_storage_ring.ability={"cost":[["energy",2]],"effects":[{"op":"gain","resource":"antimatter","amount":1}]}',
    'cards.patent_library.ability={"cost":[["credits",3]],"effects":[{"op":"gain","resource":"insight","amount":2}]}',
    'cards.bug_bounty_board.ability={"cost":[["credits",4]],"effects":[{"op":"drain_project","amount":5}]}',
    'cards.safehouse_network.ability={"effects":[{"op":"add_tokens","kind":"agents","amount":1,"here":true}],"or":{"tokens":2,"effects":[{"op":"gain","resource":"credits","amount":8}]}}',
    'cards.wafer_fab_line.ability={"cost":[["energy",2]],"effects":[{"op":"gain","resource":"chips","amount":2},{"op":"gain","resource":"credits","amount":2}]}',
    'cards.synthetic_data_forge.ability={"effects":[{"op":"add_tokens","kind":"datasets","amount":1,"here":true}],"or":{"effects":[{"op":"gain_per_tokens","resource":"insight","kind":"datasets","per":1,"here":true}]}}',
    # Any-card triggers become tag-specific.
    'cards.agency_directorate.passives=[{"op":"when","on":{"event":"play","tag":"security"},"then":[{"op":"add_tokens","kind":"agents","amount":1,"here":true}]},{"op":"threshold","at":4,"then":[{"op":"gain","resource":"insight","amount":2}]}]',
    'cards.interdisciplinary_institute.passives=[{"op":"when","on":{"event":"play_tag","tag":"science"},"then":[{"op":"gain","resource":"insight","amount":1}]}]',
    'cards.conglomerate_headquarters.passives=[{"op":"when","on":{"event":"play","tag":"industry"},"then":[{"op":"add_tokens","kind":"capital","amount":1,"here":true}]},{"op":"threshold","at":3,"then":[{"op":"gain","resource":"credits","amount":12}]}]',
    'cards.lights_out_assembly.passives=[{"op":"when","on":{"event":"play","tag":"industry"},"then":[{"op":"gain","resource":"credits","amount":2}]}]',
    'cards.capital_projects_office.passives=[{"op":"when","on":{"event":"play","tag":"industry"},"then":[{"op":"gain","resource":"credits","amount":3}]}]',
    # The Builder's credits -> chips -> cards loop.
    'cards.parts_exchange.ability={"cost":[["chips",1]],"effects":[{"op":"gain","resource":"credits","amount":5}]}',
    'ais.builder.powers.7.cost=[["chips",2]]',
    'ais.builder.powers.7.text=draw 1 card; gain 5 credits',
    'ais.builder.powers.7.effects=[{"op":"draw","count":1},{"op":"gain","resource":"credits","amount":5}]',
    # Megaprojects: payouts and converters on the scale.
    'corps.cintamani.project.player=[{"op":"gain","resource":"credits","amount":9}]',
    'corps.cintamani.project.chip_converter.cost=[["chips",1]]',
    'corps.cintamani.project.chip_converter.effects=[{"op":"draw","count":1},{"op":"gain","resource":"credits","amount":2}]',
    'corps.cintamani.project.text=At completion each player cube pays 9 credits, each adversary cube 1 adversary compute. Afterward, once a round, with a city here in your reach: 1 research cube for 2 insight; and, once a round, 1 chip for 1 drawn card and 2 credits. Afterward an adversary station here puts its cubes onto the adversary\'s research track.',
    'corps.biomia.project.player=[{"op":"sway","side":"unity","amount":1,"narrative":"cooperation"},{"op":"gain","resource":"credits","amount":4}]',
    'corps.biomia.project.text=At completion each player cube pays 1 Sway to Unity and 4 credits, each adversary cube 1 Sway to Discord. Afterward, once a round, with a city here in your reach: 1 research cube for 1 Sway to Unity; and, once a round, 2 chips for 2 Sway to Unity and 1 drawn card. Afterward an adversary station here puts its cubes onto the adversary\'s research track.',
]

# Cost moves set by hand after the ledger (card id -> new credit cost), overriding the rate rule.
COST = {
    'preprint_server': 18,
    'command_and_control_grid': 20,
    'consulate_tower': 14,
}


def draws(card):
    """(path, count) of each on-play draw in effects."""
    return [(i, e['count']) for i, e in enumerate(card.get('effects', [])) if e.get('op') == 'draw']


OUT = 'experiments/results/271_draw_economy/draw.overlay'


def fair(base, overlay=None):
    """card id -> the effect ledger's reasoned value (values.json's rates; the fit moves with every card)."""
    args = [sys.executable, 'scripts/cards/effect_ledger.py', '--stdout', '--cards', base]
    if overlay:
        args += ['--overlay', overlay]
    out = subprocess.run(args, capture_output=True, text=True, encoding='utf-8', check=True).stdout
    text = out[out.index('card,name'):]
    text = text[:text.index('\n\n')] if '\n\n' in text else text
    return {r['card']: float(r['reasoned']) for r in csv.DictReader(io.StringIO(text))}


def write(lines):
    with open(OUT, 'w', encoding='utf-8', newline='\n') as f:
        f.write('\n'.join(lines) + '\n')


def main():
    base = sys.argv[1]
    with open(base, encoding='utf-8') as f:
        cards = json.load(f)
    lines = ['# 271: the draw economy (generated by make_overlay.py; rerun it, not edit this)']
    for c in cards:
        d = draws(c)
        if not d:
            continue
        new = NEW.get(c['id'], 0)
        effects = [e for e in c['effects'] if e.get('op') != 'draw']
        if new:
            effects.append({'op': 'draw', 'count': new})
        if not effects and not c.get('passives') and not c.get('ability') and c['id'] not in (
                'preprint_server', 'command_and_control_grid', 'consulate_tower'):
            print('EMPTY', c['id'], file=sys.stderr)
        lines.append(f'cards.{c["id"]}.effects={json.dumps(effects, separators=(",", ":"))}')
    lines += HAND
    # Price: each changed card keeps its printed-minus-ledger gap (its value check against the table).
    write(lines)
    before, after = fair(base), fair(base, OUT)
    edited = {line.split('.')[1] for line in lines if line.startswith('cards.')}
    costs = []
    for c in cards:
        if c['id'] in COST:
            cost = COST[c['id']]
        elif c['id'] in edited and abs(before[c['id']] - after[c['id']]) > 0.01:
            # The floor never raises a card that lost value (Fusion Commons at 0 stays 0).
            cost = max(min(FLOOR, c['cost']), round(c['cost'] - (before[c['id']] - after[c['id']])))
        else:
            continue
        if cost != c['cost']:
            costs.append(f'cards.{c["id"]}.cost={cost}')
    write(lines + costs)
    print(len(lines) + len(costs) - 1, 'entries')


if __name__ == '__main__':
    main()

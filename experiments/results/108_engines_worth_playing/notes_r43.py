"""108 (r43): rewrite docs/card-pricing-notes.json for every card r43 changed, added or cut.

Run from the repo root after apply_r43.py and with out/cards_r42.json (git show 57469c6:content/cards.json).
An engine building's note is its value check at a committed seat's fire rate (payback.py); another card's note is
its give against its get at configs/agents/values.json, from the effect ledger's reasoned value.
"""
import csv
import io
import json
import subprocess
import sys

sys.path.insert(0, 'experiments/results/108_engines_worth_playing')
import apply_r43  # noqa: E402
import payback  # noqa: E402

NOTES = 'docs/card-pricing-notes.json'
WHEN = {
    'telemetry_archive': 'Researcher or any Science seat, from round 2',
    'citation_engine': 'Researcher, once it has 2 Science tags',
    'grad_student_pool': 'Science seat, early',
    'foundation_model_campus': 'Researcher, rounds 4-6',
    'interdisciplinary_institute': 'any multi-tag seat, mid-game',
    'data_licensing_deal': 'a Dataset seat (Telemetry Archive, Synthetic Data Forge)',
    'retained_earnings': 'Builder, early; the Capital pile its feeders fill',
    'lights_out_assembly': 'any seat buying 10+ cards, early',
    'standardized_tooling': 'Builder, early',
    'vertical_monopoly': 'Builder, rounds 4-6',
    'capital_projects_office': 'any seat buying 10+ cards, mid-game',
    'conglomerate_headquarters': 'a multi-tag Industry seat, mid-game',
    'orbital_mass_driver': 'Builder, rounds 4-6',
    'honeypot_relay': 'a seat removing ambassadors, mid-game',
    'intrusion_lab': 'a Cyber seat draining compute: the antimatter for the ultimates',
    'darknet_exchange': 'Cyber seat, early',
    'hyperscale_botnet': 'Cyber seat, mid-game',
    'security_ops_floor': 'a Cyber seat draining compute',
    'exploit_broker': 'Cyber seat, early',
    'command_and_control_grid': 'any multi-tag seat, mid-game',
    'substation_hub': 'a Power team (both seats gain)',
    'positron_storage_ring': 'Power seat, mid-game',
    'dispatch_control_room': 'Power seat, early',
    'smart_grid_backbone': 'Power seat, mid-game',
    'open_assembly': 'Diplomacy seat, early',
    'consulate_tower': 'a Diplomacy team (both seats gain)',
    'reconciliation_office': 'a seat removing ambassadors',
    'protocol_office': 'Diplomacy seat, early',
    'agency_field_station': 'Security seat, early',
    'security_contractor_hq': 'Security seat removing troops',
    'joint_task_force': 'a Security team',
}


def ledger(cards_path):
    out = subprocess.run([sys.executable, 'scripts/cards/effect_ledger.py', '--stdout', '--cards', cards_path],
                         capture_output=True, text=True, encoding='utf-8', check=True).stdout
    rows = {}
    for r in csv.DictReader(io.StringIO(out.split('\n\n')[0])):
        if r.get('card'):
            rows[r['card']] = r
    return rows


def main():
    new = {c['id']: c for c in json.load(open('content/cards.json', encoding='utf-8'))}
    led = ledger('content/cards.json')
    notes = json.load(open(NOTES, encoding='utf-8'))
    import card_list
    old = {c['id']: c for c in card_list.git_json('content/cards.json')}
    why = {i: card_list.why(old[i], c) for i, c in new.items() if i in old and c != old[i]}
    why.update({c['id']: '8: card throughput, ' + w for c, w in apply_r43.NEW})
    for cid, _ in apply_r43.CUTS:
        notes.pop(cid, None)
    for cid, w in why.items():
        c = new[cid]
        g = payback.give(c, 2)
        th = next(((p['at'], p['then']) for p in c.get('passives', []) if p['op'] == 'threshold'), None)
        per, own = payback.engine_rows(c) if c['kind'] == 'building' else (0, 0)
        if per and not any(e['op'] == 'liberate' for e in c.get('effects', [])):
            up = sum(payback.op_worth(e, th) for e in c.get('effects', []))
            rounds = max(0.0, (g - up - own) / per)
            text = (f'r43 (108): {w}. Give {g:.0f} (credits, insight to play and keep at 2.5). Get {up:.0f} when built'
                    f' + {own:.0f} on its own play, then {per:.1f} a round at a committed seat: pays back in {rounds:.1f} rounds.'
                    f' {WHEN.get(cid, "")}'.rstrip())
        elif any(e['op'] == 'production_per_tag' for e in c.get('effects', [])):
            # a per-tag income program at a committed seat: 5 of its tags when played (needs 2-3, itself, one more)
            get = sum(payback.RES[e['resource']] * 5 / e['per'] * 4 for e in c['effects'] if e['op'] == 'production_per_tag')
            get += sum(payback.op_worth(e) for e in c['effects'] if e['op'] != 'production_per_tag') + 1
            text = (f'r43 (108): {w}. Give {g:.0f} (credits, insight to play and keep at 2.5); get {get:.0f} at a committed'
                    f' seat holding 5 of its tags when played (4 rounds of income).')
        else:
            r = led.get(cid, {})
            text = (f'r43 (108): {w}. Give {g:.0f} (credits, insight to play and keep at 2.5); get {float(r.get("reasoned", 0)):.0f}'
                    f' at the table (effect ledger, reasoned).')
        notes[cid] = text
    with open(NOTES, 'w', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps(notes, indent=2, ensure_ascii=False) + '\n')
    print(len(why), 'notes written')


if __name__ == '__main__':
    main()

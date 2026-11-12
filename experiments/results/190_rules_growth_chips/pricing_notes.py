"""190: one current pricing line per card the r83 chip rescale touched (docs/card-pricing-notes.json)."""
import json

P = 'docs/card-pricing-notes.json'
NOTES = {
    'joint_fabrication_pact': 'r83 (190): chips halved (6 and 3 to 3 and 2) at a chip doubled (anchor 6): your share 18, as at r82; the gift 12 (r82 9, rounded up).',
    'vendor_financing': 'r83 (190): its gift 2 chips to 1 at a chip doubled (anchor 6): worth unchanged; anchors as at r45: pay 18; get 20.5 = 1.14.',
    'arcology_foundry': 'r83 (190): 2 chips a production to 1 at a chip doubled (anchor 6): worth unchanged; anchors: pay 27.5; get 31 = 1.13.',
    'peaker_turbines': 'r83 (190): 2 chips a production to 1 at a chip doubled (anchor 6): worth unchanged; anchors: pay 23; get 27.9 = 1.21.',
    'orbital_mass_driver': 'r83 (190): cost 2 chips to 1 and income 3 chips to 2 at a chip doubled (anchor 6): the income rounds up, +3 a round at the anchors (flagged); anchors: pay 50.5; get about 95 = 1.9.',
    'antitrust_action': 'r83 (190): 1 chip per 4 Industry tags (was 2) at a chip doubled (anchor 6): worth unchanged; anchors: pay 11; get 15 = 1.36.',
    'wafer_fab_line': 'r83 (190): a use draws 2 and gains 1 chip (was 2) at a chip doubled (anchor 6): worth unchanged; anchors: pay 20; get 21.1 = 1.06; values.json override +10 stands.',
    'fabrication_megaplex': 'r83 (190): its 1 chip a round can\'t halve: at a chip doubled (anchor 6) it gains 3 a round at the anchors (flagged); anchors: pay 42; get about 65 = 1.5.',
}
d = json.load(open(P, encoding='utf-8'))
d.update(NOTES)
with open(P, 'w', encoding='utf-8', newline='\n') as f:
    json.dump(d, f, indent=2, ensure_ascii=False, sort_keys=True)
    f.write('\n')

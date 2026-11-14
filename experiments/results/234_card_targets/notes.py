"""234: writes the re-priced cards' lines in docs/card-pricing-notes.json (one current line a card)."""
import json
import os

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
PATH = os.path.join(ROOT, 'docs', 'card-pricing-notes.json')

# rel: agents found a play on the target (or on or next to it) over r103's reach, s234a3 (dev seeds, 192 games).
NOTES = {
    "eviction_worm": "r104 (234, one target): 12 to 9 cr; 3 Agents on it find 0.73 of reach's; anchors: pay 13; get 15 x 0.73 + 4 compute 8 = 19 (1.44, as before).",
    "counterinsurgency_campaign": "r104 (234): 15 to 12 cr; 3 Agents on it at 0.73 of reach's; anchors: pay 16; get 11 + 2 credit income 8 = 19 (1.21, as before).",
    "diplomatic_expulsion": "r104 (234): 12 to 8 cr; 4 Agents on it at 0.72 of reach's; anchors: pay 12; get 14.4 + 1 Unity 4 = 18.4 (1.5, as before).",
    "killswitch_directive": "r104 (234): 16 to 12 cr; 4 Agents on or next to it at 0.72 of reach's (get -5.6 at the anchors, over 1.25).",
    "headquarters_raid": "r104 (234): 28 to 22 cr; 5 Agents on or next to it at 0.72 of reach's; anchors: pay 30; get 18 + draw 4 20 = 38 (1.25, as before).",
    "liberation_accords": "r104 (234): 15 to 11 cr; 6 Agents on or next to it at 0.80 of reach's; anchors: pay 27; get 24 + 4 ambassadors 16 = 40 (1.48, as before).",
    "martial_cordon": "r104 (234): 8 to 3 cr; 6 Agents on or next to the target (was one country) at 0.80; anchors at keep 3: pay 19; get 24 - 1 Discord 4 = 20 (1.08, as before).",
    "kinetic_interdiction": "r104 (234): 2 to 0 cr; 2 Troops (1.2 of reach's) and 6 Agents (0.80) on or next to it; anchors at keep 3: pay 30; get 12 + 24 - 4 = 32 (1.07).",
    "joint_task_force": "r104 (234): 14 to 11 cr; 2 Agents on it a fire at 0.83 of reach's; anchors: pay 19; get 27.4 (1.41, as before).",
    "directed_energy_array": "r104 (234): 11 to 8 cr; on or next to the target (a paid, repeated Action: on it alone found 0.73 of reach's, under its 2 energy); about 0.9 of reach's a use, 3 uses.",
    "back_channel_talks": "r104 (234): 14 to 11 cr; frees the target and 1 corrupted factory next to it (was 2 in reach): the second about half as often.",
    "annihilation_lance": "r104 (234): 10 to 8 cr; every Agent and Troop on the target (was any hex), 2 Troops on or next to it.",
    "great_liberation": "r104 (234): price kept (32 cr, 1 am); the target and 2 more in its country (was 3 in reach, one country): more found, still under 2.5 nominal.",
    "amnesty_decree": "r104 (234): price kept (15 cr); Place 2 Ambassadors on the target (was Gain 2): an ambassador at 8 either way, a Dispatch saved.",
}


def main():
    notes = json.load(open(PATH, encoding='utf-8'))
    # 234 shipped as r106 (r104 was its first commit, 84f51b9; r105 its review fixes, 758964b).
    notes.update({k: v.replace('r104 (234', 'r106 (234') for k, v in NOTES.items()})
    with open(PATH, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(dict(sorted(notes.items())), f, indent=2, ensure_ascii=False)
        f.write('\n')


main()

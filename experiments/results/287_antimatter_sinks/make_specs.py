"""287: the screen specs (dev seeds 36800-36831, 32 a cell) and the fresh read's spec, written to experiments/farm/
(archived to experiments/archive/farm/ after the read; `farm-seeds --at 589b47ce` finds them in experiments/farm/).

    python experiments/results/287_antimatter_sinks/make_specs.py

s287-arms: on this item's build, three content arms a pair (bp, br, pr), the judge and greedy, d0-d4:
  `<pair>-old`: the five cards set back to r137 (the base, same build);
  `<pair>-cr`:  r137 with Matt's Containment Reactor alone (6 energy: the prompt's stated numbers);
  `<pair>`:     the shipped sinks (this build's content).
s287-ov: the shipped sinks with the agents' `overrides` row for Singularity Exploit (t.overrides, 287) on, the judge
  and greedy: against s287-arms' `<pair>` arm, the row's on/off screen (amendment 13116's default 2: an overrides
  row is a sim-player weight, used only where a sink's table net is at or under 0, screened on and off).
f287-read: the fresh read, the judge and greedy, three pairs, d0-d4, 48 seeds a cell from 50220 (the next free),
  farmed on this build and with `farm-seeds --at` on r137 (main at 35c98d12, the spec added in a side commit).
"""
import json

PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]
OLD_KINETIC = [{"op": "target", "on": "adversary"}, {"op": "remove_troops", "count": 2, "at": "near"},
               {"op": "remove_ambassadors_country", "max": 6, "at": "near"},
               {"op": "sway", "side": "discord", "amount": 1, "narrative": "force"}]
OLD_FUSION = [{"op": "gain", "resource": "energy", "amount": 6},
              {"op": "sway", "side": "unity", "amount": 3, "narrative": "cooperation"}, {"op": "draw", "count": 1}]
CR_OLD = ["cards.containment_reactor.ability.effects.0.amount=4"]
SINKS_OLD = [
    "cards.kinetic_interdiction.effects=" + json.dumps(OLD_KINETIC, separators=(',', ':')),
    "cards.orbital_mass_driver.cost=15", "cards.orbital_mass_driver.chips=1", "cards.orbital_mass_driver.energy=3",
    "cards.orbital_mass_driver.insight=2", "cards.orbital_mass_driver.antimatter=0",
    "cards.orbital_mass_driver.passives.0.amount=9",
    "cards.fusion_commons.antimatter=2", "cards.fusion_commons.effects=" + json.dumps(OLD_FUSION, separators=(',', ':')),
    "cards.singularity_exploit.cost=3", "cards.singularity_exploit.insight=1",
]
OVERRIDE = "t.overrides.singularity_exploit=20"
# Message 13138 (raise Kinetic Interdiction to 2.0+): the sweep as a building's Action, 1 antimatter a use (2.41 a use
# at the p90 country), the building 4 credits.
KI_BUILDING = [
    "cards.kinetic_interdiction.kind=building", "cards.kinetic_interdiction.cost=4",
    "cards.kinetic_interdiction.antimatter=0", "cards.kinetic_interdiction.effects=[]",
    'cards.kinetic_interdiction.ability={"cost":[["antimatter",1]],"effects":[{"op":"target","on":"adversary"},'
    '{"op":"remove_troops","count":3,"at":"country"},{"op":"remove_ambassadors_country","max":10,"at":"country"}]}',
]


def spec(name, note, variants, agents, games, first):
    return {"name": name, "note": note, "configs": ["configs/baseline.json"], "variants": variants,
            "difficulties": [0, 1, 2, 3, 4], "agents": agents, "games": games, "first_seed": first, "records": 0}


def write(s):
    with open(f"experiments/farm/{s['name']}.json", 'w', encoding='utf-8', newline='\n') as f:
        json.dump(s, f, indent=1)
        f.write('\n')


def main():
    arms = []
    for p, sets in PAIRS:
        arms.append({"name": f"{p}-old", "set": sets + CR_OLD + SINKS_OLD})
        arms.append({"name": f"{p}-cr", "set": sets + SINKS_OLD})
        arms.append({"name": p, "set": sets})
    write(spec("s287-arms", "287 screen: r137's five cards (old), Matt's Containment Reactor alone (cr), the shipped "
               "sinks (base arm); dev seeds. experiments/results/287_antimatter_sinks/make_specs.py.",
               arms, ["strategist", "greedy"], 32, 36800))
    write(spec("s287-ov", "287 screen: the shipped sinks, the agents' Singularity Exploit overrides row on "
               "(t.overrides); dev seeds. experiments/results/287_antimatter_sinks/make_specs.py.",
               [{"name": p, "set": s} for p, s in PAIRS], [f"strategist:{OVERRIDE}", f"greedy:{OVERRIDE}"], 32, 36800))
    write(spec("s287-ki", "287 screen (message 13138): Kinetic Interdiction as a building whose Action sweeps the "
               "country for 1 antimatter; against s287-arms' base arm, dev seeds.",
               [{"name": f"{p}-ki", "set": s + KI_BUILDING} for p, s in PAIRS], ["strategist", "greedy"], 32, 36800))
    write(spec("s287-final", "287 screen: the final build (Kinetic Interdiction a building, agents alone), the judge and "
               "greedy, against s287-arms' old arm, dev seeds.", [{"name": f"{p}-fin", "set": s} for p, s in PAIRS],
               ["strategist", "greedy"], 32, 36800))
    ki_ov = "t.overrides.kinetic_interdiction=20"
    write(spec("s287-final-ov", "287 screen: the final build with the agents' Kinetic Interdiction overrides row on "
               "(the table reads its Action under its antimatter), against s287-final, dev seeds.",
               [{"name": f"{p}-fin", "set": s} for p, s in PAIRS], [f"strategist:{ki_ov}", f"greedy:{ki_ov}"], 32, 36800))
    write(spec("f287-read", "287 (item 287_antimatter_sinks)'s fresh paired read: the final build against r137 (main "
               "at 35c98d12, farmed with --at from this same spec), the judge and greedy, three pairs, d0-d4, 48 seeds "
               "a cell, fresh seeds 50220-50267.", [{"name": p, "set": s} for p, s in PAIRS],
               ["strategist", "greedy"], 48, 50220))


if __name__ == '__main__':
    main()

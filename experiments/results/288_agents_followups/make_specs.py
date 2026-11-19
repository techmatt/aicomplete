"""288: the screen specs (dev seeds 36800-36831, 32 a cell) and the fresh read's spec (with any argument), as
farmed, written to experiments/archive/farm/ (farmed from experiments/farm/; copy one back to farm it again).

    python experiments/results/288_agents_followups/make_specs.py

s288-base: the judge and greedy as built, three pairs, d0-d4: farmed on this item's first build and with
  `farm-seeds --at` on main (01f78e3e, r138), the check that removing the burn op and retiring `keep_units` and
  `research_delay` (built in at their shipped values) changed no game.
s288-arms: every lever on and off as an agent arm beside the bare agent (the base, the prompt's stated numbers:
  values.json 18, plan_rates 2.22, ANTIMATTER_COST 2, ANTIMATTER_INCOME_UNIT 3):
  `s1`  D120, the strict test: a sacrifice counts the cards' tags alone (plan_sac=1, w.sac_tags=1);
  `s15` D120, the rework: the cards' tags plus half the hand's (plan_sac=1.5, w.sac_tags=1.5);
  `sp`  D134, the Spill and Grow forecast (w.spill_read=1);
  `tc`  antimatter tied at today's flat rate, 2.22 credits (w.am_tie=1, t.resources.antimatter=2.222);
  `t6`  tied at 6 credits; `t18` tied at values.json's 18 (w.am_tie=1).
s288-income: screen 4, flat antimatter income tied (`w.am_income_tie`, a knob since folded into `am_tie`: the binary
  refuses this spec now).
f288-read: the fresh read, the judge and greedy, three pairs, d0-d4, 48 seeds a cell from 50268 (the next free),
  farmed on the final build and with `farm-seeds --at` on main 01f78e3e.
"""
import json
import sys

PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]
LEVERS = {
    "s1": "plan_sac=1,w.sac_tags=1",
    "s15": "plan_sac=1.5,w.sac_tags=1.5",
    "sp": "w.spill_read=1",
    "tc": "w.am_tie=1,t.resources.antimatter=2.222",
    "t6": "w.am_tie=1,t.resources.antimatter=6",
    "t18": "w.am_tie=1",
}
# Screen 2 (after s288-arms: no lever won): D120 with the hand's tags in full, D134 at half weight, the tie at 12.
LEVERS2 = {
    "s2": "plan_sac=2,w.sac_tags=2",
    "sp05": "w.spill_read=0.5",
    "t12": "w.am_tie=1,t.resources.antimatter=12",
}
# Screen 3: the tie as a share of values.json's 18 (Matt's 283 ruling keeps the table at 18): t12's and t6's rates
# for the flat agents with the table's own reads (card_action, hand worth) left at 18. `am_tie=1` is s288-arms' t18.
LEVERS3 = {
    "a067": "w.am_tie=0.6667",
    "a033": "w.am_tie=0.3333",
}


def spec(name, note, agents, games, first):
    return {"name": name, "note": note, "configs": ["configs/baseline.json"],
            "variants": [{"name": p, "set": s} for p, s in PAIRS],
            "difficulties": [0, 1, 2, 3, 4], "agents": agents, "games": games, "first_seed": first, "records": 0}


# The specs as farmed go to the archive: the screens' knob `am_income_tie` is gone, so the binary refuses
# s288-income, and experiments/farm/'s specs are parsed by a fast-lane test. Copy one back to farm it again.
FARM = "experiments/archive/farm"


def write(s):
    with open(f"{FARM}/{s['name']}.json", 'w', encoding='utf-8', newline='\n') as f:
        json.dump(s, f, indent=1)
        f.write('\n')


def main():
    base = ["strategist", "greedy"]
    write(spec("s288-base", "288: the judge and greedy as built (burn op removed, keep_units and research_delay built "
               "in), farmed here and --at main 01f78e3e: no game may differ.", base, 32, 36800))
    arms = list(base)
    for a in base:
        arms += [f"{a}:{v}" for v in LEVERS.values()]
    write(spec("s288-arms", "288: D120 (s1, s15), D134 (sp) and the antimatter tie (tc, t6, t18) as agent arms "
               "against the bare agents, dev seeds.", arms, 32, 36800))
    arms2 = list(base)
    for a in base:
        arms2 += [f"{a}:{v}" for v in LEVERS2.values()]
    write(spec("s288-arms2", "288: screen 2, the levers' second settings (s2, sp05, t12) against the bare agents, "
               "dev seeds.", arms2, 32, 36800))
    arms3 = list(base)
    for a in base:
        arms3 += [f"{a}:{v}" for v in LEVERS3.values()]
    write(spec("s288-arms3", "288: screen 3, the antimatter tie as a share of the table's 18 (the table itself "
               "unchanged) against the bare agents, dev seeds.", arms3, 32, 36800))
    # Screen 4 (the diff review: flat antimatter income was left out of the tie): on the shipped build.
    write(spec("s288-income", "288: screen 4, flat antimatter income tied too (w.am_income_tie=1) against the "
               "shipped agents, dev seeds.", base + [f"{a}:w.am_income_tie=1" for a in base], 32, 36800))
    if len(sys.argv) > 1:
        # The shipped set, as agent options (empty: the agents as built); the read pairs this build with main.
        write(spec("f288-read", "288's fresh read: the final build against r138 (main at 01f78e3e, farmed with --at "
                   "from this same spec), the judge and greedy, three pairs, d0-d4, 48 seeds a cell from 50268.",
                   base, 48, 50268))


if __name__ == '__main__':
    main()

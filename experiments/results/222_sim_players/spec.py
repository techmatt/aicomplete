"""222: the item's farm specs: agents side by side on the three board pairs, so every agent pairs on the same seeds.

    python experiments/results/222_sim_players/spec.py NAME > experiments/farm/NAME.json
    farm-seeds FIRST COUNT out/NAME.jsonl NAME
    python experiments/results/222_sim_players/read.py out/NAME.jsonl experiments/farm/NAME.json [--base AGENT]

NAME: s222-fac (the greedy / strategist factorial on the strategist's three extra switches, dev seeds),
others as added below.
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def spec(name, agents, diffs, note):
    return {'name': name, 'note': note, 'configs': ['configs/baseline.json'],
            'variants': [{'name': p, 'set': s} for p, s in PAIRS],
            'difficulties': diffs, 'agents': agents, 'games': 16, 'records': 0}


SPECS = {
    # The strategist is greedy's flat weights plus strategy=1, w.purge 3.6 and w.last_node 120 (agents/mod.rs).
    's222-fac': (['greedy', 'strategist',
                  'strategist:strategy=0', 'strategist:w.purge=0', 'strategist:w.last_node=0',
                  'strategist:strategy=0,w.purge=0,w.last_node=0',
                  'greedy:strategy=1'],
                 [0, 1, 2, 3], '222: greedy against the strategist and its switches off one at a time and all off'),
    # Screen 2 (dev seeds): the purge term toward one bill, the purge weight, the strategy's stretch, keep_tier2.
    's222-lev': (['greedy', 'strategist',
                  'strategist:w.purge_one=1', 'strategist:w.purge=1.8', 'strategist:w.strat_scale=0.5',
                  'strategist:w.purge_one=1,w.strat_scale=0.5', 'strategist:t.keep_tier2=7.5'],
                 [0, 1, 2, 3], '222: screen 2, levers on the strategy x purge interaction; keep_tier2 at a 3-insight keep'),
    # Screen 3 on 219's spent fresh seeds 46624-46719 (96 seeds; its greedy kept as f219g): purge x burn_last.
    's222-am': (['strategist', 'strategist:burn_last=0', 'strategist:w.purge=3.6', 'strategist:w.purge=3.6,burn_last=0'],
                [0, 1, 2, 3], '222: screen 3, the purge term and burn_last as a factorial, 219 fresh seeds'),
    # Screen 4 on the same seeds: burn_late on and off, and the strategy's weight (the purge term back at 3.6).
    's222-bl': (['strategist', 'strategist:burn_late=0', 'strategist:strategy=0', 'strategist:w.strat_scale=0.5'],
                [0, 1, 2, 3], '222: screen 4, burn_late on/off and the strategy lever, 219 fresh seeds'),
    # The second fresh read (new seeds 46752-46783): the judge at strat_scale 0.5 against the r97 judge and greedy;
    # strategy=0 for information (Matt's question 1).
    'f222b': (['greedy', 'strategist', 'strategist:w.strat_scale=1', 'strategist:strategy=0'],
              [0, 1, 2, 3], '222: the second fresh read, 46752-46783; strategist:w.strat_scale=1 is the r97 judge'),
    # The fresh read (new seeds 46720-46751): the new judge (purge term off) against the old judge and greedy.
    'f222': (['greedy', 'strategist', 'strategist:w.purge=3.6'],
             [0, 1, 2, 3], '222: the fresh read, new seeds 46720-46751; strategist:w.purge=3.6 is the base judge'),
}

name = sys.argv[1]
if name not in SPECS:
    sys.exit(f'unknown spec {name}; known: {", ".join(SPECS)}')
agents, diffs, note = SPECS[name]
print(json.dumps(spec(name, agents, diffs, note), indent=2))

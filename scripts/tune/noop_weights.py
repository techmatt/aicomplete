"""222: which heuristic weights change no game of an agent (a no-op weight smoke).

    python scripts/tune/noop_weights.py spec [--agent strategist] [--seeds 8] > experiments/farm/noop.json
    farm-seeds FIRST N out/noop.jsonl noop        (or aic-sim farm noop --seed FIRST --games N --out ...)
    python scripts/tune/noop_weights.py read out/noop.jsonl experiments/farm/noop.json

`spec` writes a farm spec of the agent and, per `Weights` field (read from crates/aic-sim/src/agents/eval.rs), the
agent with that weight moved: to 0 where the agent's value is not 0, else to 1. The agent's value is `Weights::default`
with the strategist's overrides (`STRATEGIST` in agents/mod.rs) for `strategist`, else the default. One board pair
(Builder and Protector), d1 and d2. `read` lists each weight whose games all match the agent's game for game (won,
rounds, decisions, options scored): moving it changed no decision on these seeds. A weight listed is a no-op for this
agent at these seeds, not proof it never matters; one that matters in other agents (table or horizon mode) shows here
too, so read the list against the field's own rule.
"""
import json
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


def weights(agent):
    with open(os.path.join(ROOT, 'crates/aic-sim/src/agents/eval.rs'), encoding='utf-8') as f:
        src = f.read()
    struct = src[src.index('pub struct Weights {'):]
    struct = struct[:struct.index('\n}\n')]
    names = re.findall(r'^\s+pub (\w+): f64,', struct, re.M)
    default = src[src.index('impl Default for Weights {'):]
    default = default[:default.index('\n}\n')]
    vals = {k: float(v) for k, v in re.findall(r'^\s+(\w+): (-?[\d.]+(?:e-?\d+)?),', default, re.M)}
    if agent == 'strategist':
        with open(os.path.join(ROOT, 'crates/aic-sim/src/agents/mod.rs'), encoding='utf-8') as f:
            mod = f.read()
        m = re.search(r'const STRATEGIST: &str = "(.*?)";', mod, re.S)
        for k, v in re.findall(r'w\.(\w+)=(-?[\d.]+)', m.group(1).replace('\\\n', '')):
            vals[k] = float(v)
    return [(n, vals.get(n)) for n in names]


def spec(agent, games):
    agents = [agent]
    for n, v in weights(agent):
        probe = 1 if v in (None, 0.0) else 0
        agents.append(f'{agent}:w.{n}={probe}')
    return {'name': 'noop', 'note': f'222: the no-op weight smoke for {agent} (scripts/tune/noop_weights.py)',
            'configs': ['configs/baseline.json'], 'variants': [{'name': 'bp', 'set': []}],
            'difficulties': [1, 2], 'agents': agents, 'games': games, 'records': 0}


def read(out, spec_path):
    with open(spec_path, encoding='utf-8') as f:
        sp = json.load(f)
    cells = [(v['name'], d, a) for c in sp['configs'] for v in sp['variants'] for d in sp['difficulties']
             for a in sp['agents']]
    games = {}
    with open(out, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' in g:
                v, d, a = cells[g['cell']]
                games[(a, d, g['seed'])] = (g['won'], g['rounds'], g['decisions'], g['options_total'])
    base = sp['agents'][0]
    keys = [k[1:] for k in games if k[0] == base]
    same, moved = [], []
    for a in sp['agents'][1:]:
        diff = sum(1 for k in keys if games.get((a,) + k) != games[(base,) + k])
        (same if diff == 0 else moved).append((a.split(':', 1)[1], diff))
    print(f'{len(same)} of {len(sp["agents"]) - 1} weights moved change no game of {base} ({len(keys)} games):')
    for a, _ in same:
        print(f'  {a}')
    print(f'moved games (of {len(keys)}): ' + ', '.join(f'{a} {d}' for a, d in moved))


if __name__ == '__main__':
    if sys.argv[1] == 'spec':
        agent = sys.argv[sys.argv.index('--agent') + 1] if '--agent' in sys.argv else 'strategist'
        n = int(sys.argv[sys.argv.index('--seeds') + 1]) if '--seeds' in sys.argv else 8
        print(json.dumps(spec(agent, n), indent=2))
    else:
        read(sys.argv[2], sys.argv[3])

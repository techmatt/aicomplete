"""200: screen specs (dev seeds 36800-36831, three pairs, 32 games a pair; d0-d4, or d3-d4 for the ablations).

    python experiments/results/200_engine_overhaul/screen_spec.py 1 [AGENT] > out/s1.json

`base` is the prompt's build as first written (r88: Annihilate on every board, the two plants at needs 2, Horizon's
converter, the two antimatter capstones, no `stage_extra`); the old side is r87 (`out/r87`) on the same seeds.
- screen 1: `n1`, the plants at needs 1 (the board's printed tag plays them); the agent lever (held antimatter
  `w.antimatter` 2 -> 5) runs as its own spec, so the four cells are a 2x2 factorial.
- screen A (ablations, d3-d4): with `no_spend` 1 / 2 / 3 the sim players never spend chips / research cubes /
  antimatter (sources stay); the agent spec carries it.
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
N1 = ['cards.penning_trap_array.needs=[["science",1]]', 'cards.antiproton_foundry.needs=[["industry",1]]']
CELLS = {
    '1': [('base', []), ('n1', N1)],
    'A': [('base', [])],
}


def spec(name, cells, agent='strategist', games=32, diffs=(0, 1, 2, 3, 4)):
    variants = []
    for c, cs in cells:
        for p, ps in PAIRS:
            variants.append({'name': p if c == 'base' else f'{p}_{c}', 'set': ps + cs})
    return {'name': name, 'note': '200 screen (dev seeds 36800-36831)', 'configs': ['configs/baseline.json'],
            'variants': variants, 'difficulties': list(diffs), 'agents': [agent], 'games': games, 'records': 0}


if __name__ == '__main__':
    which = sys.argv[1] if len(sys.argv) > 1 else '1'
    agent = sys.argv[2] if len(sys.argv) > 2 else 'strategist'
    diffs = (3, 4) if which == 'A' else (0, 1, 2, 3, 4)
    # The ablations: 64 seeds a pair (36800-36863), 192 games a level.
    games = 64 if which == 'A' else 32
    print(json.dumps(spec(f's200-{which}', CELLS[which], agent, games=games, diffs=diffs), indent=2))

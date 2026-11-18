"""274: what each table-time cut removes, per round, from `aic-sim run --trace` output(s), at 268's rates.

    python trace_cuts.py TRACE.out [TRACE.out ...]

Counts per game-round (averaged over the traces, each game's rounds pooled):
  troop_moves      troops moved one hex (components: one each, as before; a stack is still N pieces)
  march_paths      path checks: one per troop step before 274, one per stack step after (a move event is one stack step)
  march_choices    tied-step choices asked (options > 1); before 274 each troop of a stack asked its own
  march_choices_per_troop  the same choices weighted by the troops that moved on them: what the old rule asked
  threats_turned   threat cards turned up in dealing: dealt + rejected (a turned card is a check and a component)
  threats_rejected turned and discarded (tag or site mismatch)
  project_takes    TakeSite / StripHarden resolutions noted (each reads its candidates: 7 hexes a candidate before
                   274, 1 after; the candidates come from the games' `adversary.project_hexes_read` counter)
  compute_points   compute onto projects (components as cubes); compute_moves, the same as marker moves
A trace from before 274 counts a march path per troop step; one from after, per stack step, so the same trace also
gives the counterfactual: `march_paths_per_troop` (= troop_moves) is what the old rule would have checked.
"""
import collections
import json
import sys

RATES = {'component': 3, 'check': 10, 'lookup': 20, 'modifier': 5, 'decision': 15}


def read(path):
    rnd, phase = 0, ''
    rows = collections.defaultdict(collections.Counter)
    last_ask = ''
    pending = False  # a tied-step choice was just made: the next move is its stack's
    for line in open(path, encoding='utf-8'):
        if not line.startswith('{'):
            continue
        d = json.loads(line)
        if 'action' in d:
            n = d.get('options', 1)
            if 'to which hex' in last_ask and 'adversary troop' in last_ask and n > 1:
                rows[rnd]['march_choices'] += 1
                pending = True
            if n > 1:
                rows[rnd]['decisions_real'] += 1
            last_ask = ''
            continue
        e = d.get('event') or {}
        t = e.get('type')
        if t == 'round_started':
            rnd = e['round']
        elif t == 'phase_started':
            phase = e['phase']
        elif t == 'asked':
            last_ask = e.get('prompt', '')
        elif t == 'tokens_changed' and e['token'] == 'troop' and e['delta'] < 0 and phase == 'board.march':
            # A move is a -n on the hex left and a +n on the one entered; a strike is a -1 with no +.
            rows[rnd]['troop_out_events'] += 1
            rows[rnd]['troop_out'] += -e['delta']
        elif t == 'tokens_changed' and e['token'] == 'troop' and e['delta'] > 0 and phase == 'board.march':
            rows[rnd]['march_paths'] += 1
            rows[rnd]['troop_moves'] += e['delta']
            if pending:
                rows[rnd]['march_choices_per_troop'] += e['delta']
                pending = False
        elif t == 'card_moved' and e['from'].startswith('threats.deck.'):
            rows[rnd]['threats_turned'] += 1
            if e['to'].startswith('threats.discard.'):
                rows[rnd]['threats_rejected'] += 1
        elif t == 'var_changed' and e['key'].startswith('adversary.project.') and e['delta'] > 0:
            # Compute onto a project: as cubes, one component a point; as a marker on its track, one move.
            rows[rnd]['compute_points'] += e['delta']
            rows[rnd]['compute_moves'] += 1
        elif t == 'note':
            txt = e.get('text', '')
            if 'goes to the adversary' in txt or 'turns neutral' in txt or 'is stripped' in txt:
                rows[rnd]['project_takes'] += 1
    rows.pop(0, None)
    return rows


def main():
    games = [read(p) for p in sys.argv[1:]]
    tot = collections.Counter()
    rounds = 0
    for g in games:
        for r, c in g.items():
            rounds += 1
            tot.update(c)
    print(f'{len(games)} traces, {rounds} game-rounds')
    keys = ['troop_moves', 'march_paths', 'march_choices', 'march_choices_per_troop', 'threats_turned', 'threats_rejected', 'project_takes',
            'compute_points', 'compute_moves', 'decisions_real']
    for k in keys:
        print(f'{k:18s} {tot[k] / max(rounds, 1):7.2f} a round')


main()

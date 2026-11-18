"""274: what each cut removes per round of one traced game, in components, checks and decisions, and minutes at
268's rates (3 s a component, 10 s a check, 20 s a lookup, 5 s a modifier, 15 s a decision).

    python minutes.py BEFORE.out AFTER.out [--hexes-read N] [--seats 2] [--income-lines 8.5]

BEFORE / AFTER: `aic-sim run --trace` of the same seed on r128 and r129. From AFTER, per round:
  march      path checks one a troop step (old) against one a stack step (new); tied-step choices one a troop (old)
             against one a stack (new)
  compute    compute onto projects as cubes (one component a point) against a marker on each project's compute
             track (one move a project per beat that adds compute)
  threats    cards turned and rejected in dealing (a component and a check each): the skip removes those in a
             skipped country (none at 2-4 countries)
  projects   `--hexes-read`, the project candidates read a game-round (a farmed read's
             `adversary.project_hexes_read`): 6 checks each fewer (7 hexes to 1)
  income     `--income-lines` printed income lines a seat re-adds each round (268: 7-10) as modifiers, against reading
             one marker a resource on an income track
"""
import argparse
import collections
import json

R = {'component': 3, 'check': 10, 'lookup': 20, 'modifier': 5, 'decision': 15}


def per_round(path):
    rnd, phase = 0, ''
    rows = collections.defaultdict(collections.Counter)
    last_ask, pending, last_compute = '', False, None
    for line in open(path, encoding='utf-8'):
        if not line.startswith('{'):
            continue
        d = json.loads(line)
        if 'action' in d:
            n = d.get('options', 1)
            if 'adversary troop' in last_ask and 'to which hex' in last_ask and n > 1:
                rows[rnd]['choices'] += 1
                pending = True
            last_ask = ''
            continue
        e = d.get('event') or {}
        t = e.get('type')
        if t == 'round_started':
            rnd = e['round']
        elif t == 'phase_started':
            phase = e['phase']
            last_compute = None
        elif t == 'asked':
            last_ask = e.get('prompt', '')
        elif t == 'tokens_changed' and e['token'] == 'troop' and e['delta'] > 0 and phase == 'board.march':
            rows[rnd]['stack_steps'] += 1
            rows[rnd]['troop_steps'] += e['delta']
            if pending:
                rows[rnd]['choices_old'] += e['delta']
                pending = False
        elif t == 'var_changed' and e['key'].startswith('adversary.project.') and e['delta'] > 0:
            rows[rnd]['compute_points'] += e['delta']
            if last_compute != (phase, e['key']):
                rows[rnd]['compute_marker_moves'] += 1
                last_compute = (phase, e['key'])
        elif t == 'card_moved' and e['from'].startswith('threats.deck.') and e['to'].startswith('threats.discard.'):
            rows[rnd]['threats_rejected'] += 1
    rows.pop(0, None)
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('before')
    ap.add_argument('after')
    ap.add_argument('--hexes-read', type=float, default=0.16)
    ap.add_argument('--seats', type=int, default=2)
    ap.add_argument('--income-lines', type=float, default=8.5)
    a = ap.parse_args()
    before, after = per_round(a.before), per_round(a.after)
    print('| Round | March: checks / choices cut | Compute: components cut | Projects: checks cut | Income: modifiers cut '
          '| Minutes cut | Threat cards still turned and rejected |')
    print('|---|---|---|---|---|---|---|')
    tot = collections.Counter()
    for r in sorted(after):
        c = after[r]
        march_checks = c['troop_steps'] - c['stack_steps']
        march_choices = c['choices_old'] - c['choices']
        compute = c['compute_points'] - c['compute_marker_moves']
        projects = 6 * a.hexes_read
        income = a.seats * a.income_lines
        secs = (march_checks + projects) * R['check'] + march_choices * R['decision'] + compute * R['component'] \
            + income * R['modifier']
        tot.update({'march_checks': march_checks, 'march_choices': march_choices, 'compute': compute,
                    'projects': projects, 'income': income, 'secs': secs, 'rejected': c['threats_rejected'],
                    'rounds': 1})
        print(f"| {r} | {march_checks} / {march_choices} | {compute} | {projects:.1f} | {income:.0f} | {secs / 60:.1f} "
              f"| {c['threats_rejected']} |")
    n = tot['rounds']
    print(f"| **mean** | {tot['march_checks'] / n:.1f} / {tot['march_choices'] / n:.1f} | {tot['compute'] / n:.1f} | "
          f"{tot['projects'] / n:.1f} | {tot['income'] / n:.0f} | **{tot['secs'] / n / 60:.1f}** | "
          f"{tot['rejected'] / n:.1f} |")
    same = all(before[r]['troop_steps'] == after[r]['troop_steps'] for r in after) and len(before) == len(after)
    print(f'\nbefore and after play the same troop moves every round: {same}')


main()

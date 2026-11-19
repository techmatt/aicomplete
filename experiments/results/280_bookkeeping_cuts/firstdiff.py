"""First difference between two traces, ignoring adversary.compute_waiting var events."""
import sys


def lines(p):
    out = []
    for i, line in enumerate(open(p, encoding='utf-8')):
        if '"adversary.compute_waiting"' in line or not line.startswith('{"'):
            continue
        out.append((i + 1, line.rstrip()))
    return out


a, b = lines(sys.argv[1]), lines(sys.argv[2])
for (ia, la), (ib, lb) in zip(a, b):
    if la != lb:
        print('A', ia, la[:300])
        print('B', ib, lb[:300])
        ctx = [x for x in a if ia - 40 <= x[0] < ia]
        for n, l in ctx:
            if 'phase_started' in l or 'round_started' in l or 'note' in l:
                print('  ctx', n, l[:200])
        break
else:
    print('no difference in', min(len(a), len(b)), 'lines; lengths', len(a), len(b))

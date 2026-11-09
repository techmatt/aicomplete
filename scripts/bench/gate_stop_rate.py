"""How often the gate's early stop flips a verdict (040): the sign of the gap at the cap,
simulated per seed for an agent at win rate p and a check at q, independent, under the rule
in crates/aic-sim/src/gate.rs. Prints, per true gap: the share of runs stopped early, the
share whose stopped verdict differs from the one the cap would give (a level gap at the
cap counts as `loses` false, as in the gate), and the mean seeds a check takes.

    python scripts/bench/gate_stop_rate.py [z] [min_games] [every] [cap]
"""
import math
import random
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def run(p, q, z, first, every, cap, rng):
    seq = [(rng.random() < p) - (rng.random() < q) for _ in range(cap)]
    final = sum(seq) > 0
    d = k = 0
    for n in range(1, cap):
        d += seq[n - 1]
        k += seq[n - 1] != 0
        if n < first or (n - first) % every or d == 0:
            continue
        m = abs(d)
        if m >= z * math.sqrt(k) or m >= z * math.sqrt((cap - n) * k / n):
            return n, (d > 0) != final
    return cap, False


def main():
    a = [float(x) for x in sys.argv[1:]]
    z, first, every, cap = (a + [2.0, 8, 4, 32][len(a):])
    first, every, cap = int(first), int(every), int(cap)
    rng = random.Random(40)
    print(f'z {z}, first look {first}, every {every}, cap {cap}; 20000 runs a row')
    print('| agent % | check % | stopped early % | stopped verdict flips % | mean seeds |')
    print('|---|---|---|---|---|')
    for p in (0.66, 0.38):
        for gap in (0.0, 0.05, 0.1, 0.2, 0.3, p):
            q = max(p - gap, 0.0)
            runs = [run(p, q, z, first, every, cap, rng) for _ in range(20000)]
            early = sum(n < cap for n, _ in runs) / len(runs)
            flip = sum(f for _, f in runs) / len(runs)
            mean = sum(n for n, _ in runs) / len(runs)
            print(f'| {100 * p:.0f} | {100 * q:.0f} | {100 * early:.0f} | {100 * flip:.1f} | {mean:.1f} |')


if __name__ == '__main__':
    main()

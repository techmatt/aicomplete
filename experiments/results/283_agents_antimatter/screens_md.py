"""283: screens.md from the four screens' read_lib.py outputs (out/283_agents_antimatter/s283-{a,b,c,d}.read.txt),
arm names shortened to make_screen.py's.

    python scripts/bench/read_lib.py out/283_agents_antimatter/s283-X.jsonl --agent-arms > out/283_agents_antimatter/s283-X.read.txt
    python experiments/results/283_agents_antimatter/screens_md.py
"""
import re

OUT = 'experiments/results/283_agents_antimatter/screens.md'
SCREENS = [('a', 'base, T, Tnk, K, D, A4, A4s'), ('b', 'base, Tr, Trm, Th, Tf, A3, A3s'), ('c', 'base, C, CK, CP'),
           ('d', 'base, R, RK, RKA'), ('e', 'base = the shipped build, am12'),
           ('f', 'base = the shipped build, S1, A3s')]


def name(arm, screen):
    if arm == 'base':
        return 'base'
    if screen == 'a':
        if 'trateskeep_tier12667' in arm:
            return 'T (table at the anchors, keep 8)'
        if arm.startswith('tresources'):
            return 'Tnk (T without the keep rows)'
    if screen == 'b':
        if arm.startswith('tresources'):
            return 'Tr (resources and drawn card at the anchors)'
        if arm.startswith('topsremove'):
            return 'Trm (agent removed 3.33, drain 1.33, discard 10)'
        if 'horizon' in arm:
            return 'Th (horizon_t2 3.5)'
        if 'tag_share' in arm:
            return 'Tf (tag_share 0.5)'
    if screen == 'c':
        if 'keep_units1' in arm:
            return 'CK (C + keep_units)'
        if 'plan_target1' in arm:
            return 'CP (C + plan_target)'
        return 'C (Tr + keep + Th + w.antimatter 3 + D106)'
    if screen == 'f':
        return {'sink1': 'S1 (w.sink 1)', 'antimatter3,sink1': 'A3s (w.antimatter 3 + w.sink 1)'}.get(arm, arm)
    if screen == 'e' and 'antimatter12' in arm:
        return "am12 (the table's antimatter 12, not the ruling's 18)"
    if screen == 'd':
        if arm.startswith('antimatter3'):
            return 'RKA (RK + w.antimatter 3)'
        if arm.startswith('keep_units1'):
            return 'RK (R + keep_units): shipped'
        if arm.startswith('tresources'):
            return 'R (Tr + keep 2.67 / 6.93 + Th)'
    return {'keep_units1': 'K (keep_units)', 'research_delay1': 'D (D106 research_delay)',
            'antimatter4': 'A4 (w.antimatter 4)', 'antimatter4,sink1': 'A4s (A4 + w.sink 1)',
            'antimatter3': 'A3 (w.antimatter 3)', 'antimatter3,sink1': 'A3s (A3 + w.sink 1)'}.get(arm, arm)


def main():
    out = ['# 283 screens (dev seeds 36800-36831, three pairs, d0-d4, 480 games an arm per agent, r135)', '',
           'Each arm paired against base (the agents as built) on the same pair, level and seed: win % and the gap ± '
           'one SE. Arms: `make_screen.py`; reads: `scripts/bench/read_lib.py FILE --agent-arms`.', '']
    for s, arms in SCREENS:
        text = open(f'out/283_agents_antimatter/s283-{s}.read.txt', encoding='utf-8').read()
        block = text.split('\n\n')[1]
        out.append(f'## Screen {s} ({arms})')
        out.append('')
        for line in block.splitlines():
            cells = line.split('|')
            if len(cells) > 3 and cells[2].strip() not in ('arm', '---'):
                cells[2] = ' ' + name(cells[2].strip(), s) + ' '
            out.append('|'.join(cells))
        gaps = text.split('### Paired gaps by level')[1].split('\n\n')[1]
        out.append('')
        out.append('Paired gaps by level:')
        out.append('')
        for line in gaps.splitlines():
            cells = line.split('|')
            if len(cells) > 3 and cells[2].strip() not in ('arm', '---'):
                cells[2] = ' ' + name(cells[2].strip(), s) + ' '
            out.append('|'.join(cells))
        out.append('')
    with open(OUT, 'w', encoding='utf-8', newline='\n') as f:
        f.write('\n'.join(out))
    print(OUT)


if __name__ == '__main__':
    main()

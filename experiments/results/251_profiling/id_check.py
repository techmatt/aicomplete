"""251: the identity check, base against new, per arm: a pinned fingerprint of each side (a hash over every game's
(seed, cell, won, reason, rounds, decisions) in (seed, cell) order) and the count of whole lines that differ once
the engine field and every timing are set aside (as scripts/farm/compare.py reads them).

    python id_check.py DIR ARM...   (DIR holds base-ARM.jsonl and new-ARM.jsonl)
"""
import hashlib
import json
import sys

TIMING = ('ms', 'agent_ms', 'probe_ms', 'setup_ms', 'legal_ms', 'apply_ms', 'replay_ms')


def games(path):
    out = {}
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'seed' not in g:
                continue
            for k in TIMING + ('engine',):
                g.pop(k, None)
            out[(int(g['seed']), g['cell'])] = g
    return out


def rounds(g):
    return g.get('rounds', g.get('round'))


def fingerprint(gs):
    h = hashlib.sha256()
    for key in sorted(gs):
        g = gs[key]
        h.update(json.dumps([key[0], key[1], g['won'], g['reason'], rounds(g), g['decisions']]).encode())
    return h.hexdigest()[:16]


d = sys.argv[1]
print('| arm | games | seeds | fingerprint base | fingerprint new | lines that differ |')
print('|---|---|---|---|---|---|')
bad = 0
for arm in sys.argv[2:]:
    a, b = games(f'{d}/base-{arm}.jsonl'), games(f'{d}/new-{arm}.jsonl')
    keys = set(a) | set(b)
    differ = sum(1 for k in keys if a.get(k) != b.get(k))
    bad += differ + (set(a) != set(b))
    seeds = len({k[0] for k in keys})
    print(f'| {arm} | {len(a)} / {len(b)} | {seeds} | {fingerprint(a)} | {fingerprint(b)} | {differ} |')
sys.exit(1 if bad else 0)

"""163: reads runs A (a local experiment) and the farmed runs (B, C): wall time, shards by worker with each shard's
start, end and seeds a minute a core, re-runs and when, and the reclaim after a killed tab.

    python runs.py A_DIR RUN=OUT_JSONL,JOB_DIR,BOARD_JSONL,LOG [RUN=...]

A_DIR: the experiment's output (games.jsonl with per-game ms). OUT_JSONL: farm-seeds' output; JOB_DIR: its
`.conductor/remote/jobs/<job>`; BOARD_JSONL: kill_at.py's record of the job; LOG: the farm-seeds log whose last line is
`X_EXIT code start_ms S end_ms E`.
"""
import json
import os
import re
import sys
import time

SEEDS_A_CELL = 12  # the read spec's cells: a seed is 12 games


def clock(t):
    return time.strftime('%I:%M:%S%p', time.localtime(t)).lstrip('0').lower()


def wall(log):
    with open(log, encoding='utf-8') as f:
        m = re.search(r'start_ms (\d+) end_ms (\d+)', f.read())
    return int(m[1]) / 1000, int(m[2]) / 1000


def run_a(d, log):
    s, e = wall(log)
    games = [json.loads(l) for l in open(os.path.join(d, 'games.jsonl'), encoding='utf-8')][1:]
    seeds = len({g['seed'] for g in games})
    ms = sum(g['ms'] for g in games)
    print(f'## A: local, 6 threads\n\nwall {e - s:.1f} s ({clock(s)} to {clock(e)}), {seeds} seeds, {len(games)} games; '
          f'game time summed {ms / 1000:.0f} s = {ms / 1000 / 6:.0f} s a thread; '
          f'**{seeds * 60000 / ms:.2f} seeds a minute a core** (per-game ms, all games); '
          f'{seeds * 60 / (e - s):.2f} seeds a minute on the box (wall)\n')
    return seeds * 60000 / ms


def run_farm(name, out, job, board, log):
    s, e = wall(log)
    j = json.load(open(os.path.join(job, 'job.json'), encoding='utf-8'))
    snaps = [json.loads(l) for l in open(board, encoding='utf-8')]
    kill = next((x for x in snaps if 'killed' in x), None)
    snaps = [x for x in snaps if 'shards' in x]
    # each shard's leases over the job (the first time each worker held it), and when its result first showed
    leases, done_at = {}, {}
    for x in snaps:
        for sh in x['shards']:
            for w, t in sh['leases'].items():
                leases.setdefault(sh['first'], {}).setdefault(w, t)
            if sh['done'] and sh['first'] not in done_at:
                done_at[sh['first']] = x['at']
    print(f'## {name}\n\nwall {e - s:.1f} s ({clock(s)} to {clock(e)}); engines {j["engines"]}, mixed {j["mixed"]}; '
          f'gate failures {len(j["gate_failures"])}; references differ {j["references"]["differ"]}\n')
    print('| shard | leased to (s after start) | won by | engine | ms | seeds/min/core | in at |')
    print('|---|---|---|---|---|---|---|')
    rates = {}
    for sh in j['shards']:
        ls = ', '.join(f'{w} +{t - s:.0f}' for w, t in sorted(leases.get(sh['first'], {}).items(), key=lambda kv: kv[1]))
        r = sh['count'] * 60000 / sh['ms']
        rates.setdefault((sh['worker'], sh['label'], sh['engine']), []).append(r)
        at = done_at.get(sh['first'])
        print(f'| {sh["first"]} | {ls} | {sh["worker"]} | {sh["engine"]} | {sh["ms"]} | {r:.2f} | '
              f'{f"+{at - s:.0f}" if at else "?"} |')
    print()
    for (w, label, eng), rs in rates.items():
        print(f'- {w} ({label}, {eng}): {len(rs)} shards, {sum(rs) / len(rs):.2f} seeds a minute a core '
              f'(mean of its shards)')
    reruns = [l.strip() for l in open(os.path.join(job, 'main.log'), encoding='utf-8') if 'rerun' in l]
    print(f'- re-runs: {len(reruns)}' + ''.join(f'\n  - {r} (job ends {clock(e)})' for r in reruns))
    if kill:
        print(f'- tab killed at {clock(kill["killed"])} (+{kill["killed"] - s:.0f} s), {kill["done"]} of {kill["of"]} '
              f'shards in: {kill["taskkill"]}')
        for first, ws in leases.items():
            held = [w for w, t in ws.items() if t <= kill['killed'] and w not in ('main',)]
            after = {w: t for w, t in ws.items() if t > kill['killed']}
            if held and after:
                print(f'  - shard {first}: held by {held}, re-leased to ' +
                      ', '.join(f'{w} {t - kill["killed"]:.1f} s after the kill' for w, t in after.items()))
    print()
    return rates


def main():
    a_dir = sys.argv[1]
    run_a(a_dir, a_dir + '.log')
    for arg in sys.argv[2:]:
        name, paths = arg.split('=', 1)
        run_farm(name, *paths.split(','))


if __name__ == '__main__':
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    main()

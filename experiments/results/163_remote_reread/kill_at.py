"""163's runs B and C: records the farm job begun after this starts, from the broker's board.json (each shard's leases
with their times, and who finished it), one JSON line a change, until the job leaves the board. With a PIDFILE, kills
that browser's whole tree (the tab lost, run C) once AFTER seconds of the job have passed and the tab (its worker id
in PIDFILE.wid) holds an unfinished shard, and logs when.

    python kill_at.py BOARD_JSON OUT_JSONL [PIDFILE AFTER]
"""
import json
import os
import subprocess
import sys
import time

board, out = sys.argv[1], sys.argv[2]
pidfile, fraction = (sys.argv[3], float(sys.argv[4])) if len(sys.argv) > 4 else (None, None)
start = time.time()
job_id, last, killed = None, None, False
with open(out, 'w', encoding='utf-8', newline='\n') as log:
    while True:
        try:
            with open(board, encoding='utf-8') as f:
                b = json.load(f)
        except (OSError, ValueError):
            time.sleep(0.5)
            continue
        jobs = [j for j in b.get('jobs', []) if j['created'] >= start - 5]
        job = next((j for j in jobs if j['id'] == job_id), None) if job_id else (jobs[-1] if jobs else None)
        if job_id and job is None:
            break  # the job closed
        if job:
            job_id = job['id']
            snap = {'shards': [{'id': s['id'], 'first': s['first'], 'leases': s['leases'], 'done': s['done']}
                               for s in job['shards']]}
            if snap != last:
                log.write(json.dumps({'at': b['at'], 'job': job_id, **snap}) + '\n')
                log.flush()
                last = snap
            done = sum(1 for s in job['shards'] if s['done'])
            wid = open(pidfile + '.wid', encoding='utf-8').read().strip() if pidfile and os.path.exists(pidfile + '.wid') else None
            holds = wid and any(wid in s['leases'] and not s['done'] for s in job['shards'])
            if pidfile and not killed and holds and time.time() - job['created'] >= fraction:
                with open(pidfile, encoding='utf-8') as f:
                    pid = f.read().strip()
                now = time.time()
                r = subprocess.run(['taskkill', '/F', '/T', '/PID', pid], capture_output=True, text=True)
                log.write(json.dumps({'killed': now, 'pid': pid, 'done': done, 'of': len(job['shards']),
                                      'taskkill': (r.stdout or r.stderr).strip()}) + '\n')
                log.flush()
                killed = True
        time.sleep(0.5)

"""The strategist's parameter sweep (116): how much better than its hand-set values the judge
could play under the current rules, and which of its parameters matter.

    python scripts/tune/agent_sweep.py                       # everything, resumable
    python scripts/tune/agent_sweep.py --phase confirm       # one phase (sweep, profile, confirm, read)
    python scripts/tune/agent_sweep.py --sweep-seed S --confirm-seed T   # a later revision's fresh seeds

What it plays (scripts/tune/agent_sweep.json): the strategist at d2 on the three board pairs,
every setting on the same sweep seeds. A setting is the strategist with each swept parameter
named explicitly; the hand-set setting must play game for game as plain `strategist`, or the
JSON's hand-set values are stale and the script stops.

1. sweep: coordinate search over each parameter's grid. A pass takes the parameters in order
   and plays every grid point within `--reach` steps of the current best (the others held);
   the best point is taken when its paired gain over the current best passes `--accept` SE.
   Passes repeat (the reach drops to 1 after the first) until one moves nothing.
2. profile: every grid point of every parameter, the others at the best setting: each
   parameter's effect around the best, and which are flat.
3. confirm: the best setting, the hand-set strategist and greedy on the fresh confirm seeds,
   paired; greedy on the sweep seeds as well; credits left at game end.
4. read: what the best setting does differently, counter by counter, on the confirm games.

Every game result is cached in OUT/cache.jsonl under (agent spec, seeds, binary hash), so a
stopped run resumes and a phase reruns in seconds. OUT/summary.md holds the tables; OUT/log.txt
the progress lines. The sim runs at below-normal priority.
"""

import argparse
import hashlib
import json
import math
import os
import re
import shutil
import subprocess
import sys
import time
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..'))
from screens import cell_labels, stream  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

BELOW_NORMAL = 0x4000  # Windows BELOW_NORMAL_PRIORITY_CLASS


def clock():
    """2:45pm."""
    t = time.localtime()
    return f"{(t.tm_hour % 12) or 12}:{t.tm_min:02d}{'am' if t.tm_hour < 12 else 'pm'}"


def revision():
    with open(os.path.join('crates', 'aic-core', 'src', 'lib.rs'), encoding='utf-8') as f:
        return int(re.search(r'RULES_REVISION: u32 = (\d+);', f.read()).group(1))


def file_hash(path):
    h = hashlib.sha256()
    with open(path, 'rb') as f:
        for block in iter(lambda: f.read(1 << 20), b''):
            h.update(block)
    return h.hexdigest()[:16]


def num(v):
    """A value as a spec writes it: 0.5, 12, 0."""
    return f'{v:g}'


class Sweep:
    def __init__(self, a, cfg):
        self.a, self.cfg = a, cfg
        self.params = cfg['params']
        # What a game result depends on: the binary, the config it reads from disk, and the
        # pairs, difficulty and agent this file names.
        h = hashlib.sha256(file_hash(a.sim).encode())
        with open(cfg['config'], 'rb') as f:
            h.update(f.read())
        h.update(json.dumps([cfg['config'], cfg['pairs'], cfg['difficulty'], cfg['agent']], sort_keys=True).encode())
        self.sim_hash = h.hexdigest()[:16]
        os.makedirs(a.out, exist_ok=True)
        self.cache_path = os.path.join(a.out, 'cache.jsonl')
        self.cache = {}
        if os.path.exists(self.cache_path):
            with open(self.cache_path, encoding='utf-8') as f:
                for line in f:
                    try:
                        r = json.loads(line)
                    except json.JSONDecodeError:
                        continue
                    self.cache[(r['spec'], r['seeds'], r['sim'])] = r['units']
        self.logf = open(os.path.join(a.out, 'log.txt'), 'a', encoding='utf-8', newline='\n')
        self.t0 = time.time()
        self.games_played = 0
        self.batch = 0

    def log(self, msg):
        line = f'{clock()} {msg}'
        print(line, flush=True)
        self.logf.write(line + '\n')
        self.logf.flush()

    # --- settings ---------------------------------------------------------------------------

    def hand(self):
        return tuple(self.params[i]['grid'].index(self.params[i]['hand']) for i in range(len(self.params)))

    def spec(self, idx):
        """The agent spec of a setting (a tuple of grid indexes), every parameter named."""
        parts = [f"{p['key']}={num(p['grid'][i])}" for p, i in zip(self.params, idx)]
        return f"{self.cfg['agent']}:{','.join(parts)}"

    def short(self, idx):
        """Only what differs from hand-set: `w.energy=1.1,plan_credit=2` (or `hand-set`)."""
        h = self.hand()
        parts = [f"{p['key']}={num(p['grid'][i])}" for p, i, j in zip(self.params, idx, h) if i != j]
        return ','.join(parts) or 'hand-set'

    # --- playing ----------------------------------------------------------------------------

    def seeds_key(self, which):
        s = self.a.sweep_seed if which == 'sweep' else self.a.confirm_seed
        n = self.a.sweep_games if which == 'sweep' else self.a.confirm_games
        return f'{s}+{n}'

    def play(self, specs, which, keep=None):
        """{spec: {unit: game summary}} for each agent spec on seed set `which`, from the cache
        or by one experiment over those not cached. `keep`: the run directory's name, kept for
        a later read (else reused)."""
        key = self.seeds_key(which)
        todo = [s for s in dict.fromkeys(specs) if (s, key, self.sim_hash) not in self.cache]
        if keep and self.kept(keep) != self.keep_tag(specs, which):
            # A kept run holds exactly these specs on these seeds from this build, or is replayed.
            todo = list(dict.fromkeys(specs))
        if todo:
            self.run(todo, which, keep)
        return {s: self.cache[(s, key, self.sim_hash)] for s in specs}

    def keep_tag(self, specs, which):
        return {'specs': list(dict.fromkeys(specs)), 'seeds': self.seeds_key(which), 'sim': self.sim_hash}

    def kept(self, keep):
        d = os.path.join(self.a.out, 'runs', keep)
        try:
            with open(os.path.join(d, 'kept.json'), encoding='utf-8') as f:
                tag = json.load(f)
        except (OSError, json.JSONDecodeError):
            return None
        return tag if os.path.exists(os.path.join(d, 'games.jsonl')) else None

    def run(self, specs, which, keep):
        first = self.a.sweep_seed if which == 'sweep' else self.a.confirm_seed
        games = self.a.sweep_games if which == 'sweep' else self.a.confirm_games
        self.batch += 1
        name = keep or f'batch-{os.getpid()}-{self.batch}'
        d = os.path.join(self.a.out, 'runs', name)
        os.makedirs(d, exist_ok=True)
        spec = {
            'name': f'agent-sweep-{name}',
            'note': 'scripts/tune/agent_sweep.py writes this; see scripts/tune/README.md.',
            'configs': [self.cfg['config']],
            'variants': [{'name': k, 'set': v} for k, v in self.cfg['pairs'].items()],
            'difficulties': [self.cfg['difficulty']],
            'agents': specs,
            'games': games,
            'first_seed': first,
            'records': 0,
        }
        spec_path = os.path.join(d, 'spec.json')
        with open(spec_path, 'w', encoding='utf-8', newline='\n') as f:
            json.dump(spec, f, indent=1)
        n = len(specs) * len(self.cfg['pairs']) * games
        self.log(f'playing {len(specs)} setting(s) x {len(self.cfg["pairs"])} pairs x {games} seeds = {n} games ({which})')
        t = time.time()
        cmd = [self.a.sim, 'experiment', spec_path, '--out', d, '--games', str(games),
               '--seed', str(first), '--threads', str(self.a.threads)]
        flags = BELOW_NORMAL if os.name == 'nt' else 0
        with open(os.path.join(d, 'sim.log'), 'w', encoding='utf-8', newline='\n') as logf:
            p = subprocess.run(cmd, stdout=logf, stderr=subprocess.STDOUT, creationflags=flags)
        if p.returncode != 0:
            raise SystemExit(f'aic-sim failed ({p.returncode}): {" ".join(cmd)}; see {d}/sim.log')
        secs = time.time() - t
        self.games_played += n
        self.log(f'  {n} games in {secs:.0f} s ({n / secs:.1f} games/s)')
        lines = stream(d)
        header = next(lines)
        labels = cell_labels(header['spec'])
        out = defaultdict(dict)
        for g in lines:
            variant, agent = labels[g['cell']].split('/', 1)
            unspent = g['margin']['unspent']
            out[agent][f"{variant}/{g['seed']}"] = {
                'won': int(bool(g['won'])),
                'credits': sum(u[0] for u in unspent) / len(unspent),
                'rounds': g['rounds'],
            }
        key = self.seeds_key(which)
        with open(self.cache_path, 'a', encoding='utf-8', newline='\n') as f:
            for s in specs:
                units = out.get(s, {})
                if len(units) != n // len(specs):
                    raise SystemExit(f'{s}: {len(units)} games, expected {n // len(specs)}; see {d}')
                self.cache[(s, key, self.sim_hash)] = units
                f.write(json.dumps({'spec': s, 'seeds': key, 'sim': self.sim_hash, 'units': units}) + '\n')
        if keep:
            with open(os.path.join(d, 'kept.json'), 'w', encoding='utf-8', newline='\n') as f:
                json.dump(self.keep_tag(specs, which), f)
        else:
            # The cache holds what the search reads; the batch's games need not stay.
            for fn in os.listdir(d):
                path = os.path.join(d, fn)
                if fn in ('spec.json', 'sim.log'):
                    continue
                if os.path.isdir(path):
                    shutil.rmtree(path)
                else:
                    os.remove(path)

    # --- reading ----------------------------------------------------------------------------

    @staticmethod
    def win(units):
        return 100 * sum(u['won'] for u in units.values()) / len(units)

    @staticmethod
    def gap(base, lever, field='won'):
        """lever less base on the units both played: (mean, SE, n), wins in points."""
        k = 100 if field == 'won' else 1
        d = [k * (lever[u][field] - base[u][field]) for u in base if u in lever]
        n = len(d)
        m = sum(d) / n
        var = sum((x - m) ** 2 for x in d) / (n - 1) if n > 1 else 0.0
        return m, math.sqrt(var / n), n

    @staticmethod
    def mean(units, field):
        return sum(u[field] for u in units.values()) / len(units)

    def elapsed(self):
        return (time.time() - self.t0) / 60

    # --- phases -----------------------------------------------------------------------------

    def check_hand(self):
        """The hand-set setting must play as plain strategist, game for game."""
        h = self.spec(self.hand())
        r = self.play([self.cfg['agent'], h], 'sweep')
        a, b = r[self.cfg['agent']], r[h]
        diff = [u for u in a if a[u] != b.get(u)]
        if diff:
            raise SystemExit(f'the hand-set values in {self.a.config} play differently from plain '
                             f'{self.cfg["agent"]} on {len(diff)} games (first {diff[0]}): update their "hand" '
                             'to the agent\'s current defaults (eval.rs, Weights::engine; policy.rs, forced_table)')
        self.log(f'hand-set check: {h} plays as {self.cfg["agent"]} ({len(a)} games, {self.win(a):.1f}% won)')

    def sweep(self):
        cur = self.hand()
        self.check_hand()
        trail = []
        for npass in range(1, self.a.max_passes + 1):
            reach = self.a.reach if npass == 1 else 1
            moved = False
            for i, p in enumerate(self.params):
                if self.a.budget and self.elapsed() > self.a.budget:
                    self.log(f'budget of {self.a.budget} min spent: the search stops at pass {npass}, {p["key"]}')
                    return cur, trail, False
                cands = []
                for j in range(len(p['grid'])):
                    if j != cur[i] and abs(j - cur[i]) <= reach:
                        c = list(cur)
                        c[i] = j
                        cands.append(tuple(c))
                if not cands:
                    continue
                specs = [self.spec(cur)] + [self.spec(c) for c in cands]
                r = self.play(specs, 'sweep')
                base = r[specs[0]]
                scored = []
                for c, s in zip(cands, specs[1:]):
                    m, se, _ = self.gap(base, r[s])
                    scored.append((m, se, c))
                m, se, best = max(scored, key=lambda x: x[0])
                take = se > 0 and m > self.a.accept * se
                row = ' '.join(f"{num(p['grid'][c[i]])}:{mm:+.1f}" for mm, _, c in sorted(scored, key=lambda x: x[2][i]))
                self.log(f'pass {npass} {p["key"]} at {num(p["grid"][cur[i]])} ({self.win(base):.1f}%): {row}'
                         f' -> {"take " + num(p["grid"][best[i]]) if take else "stay"} (best {m:+.1f} ±{se:.1f})')
                trail.append({'pass': npass, 'key': p['key'], 'from': p['grid'][cur[i]],
                              'to': p['grid'][best[i]] if take else p['grid'][cur[i]], 'gain': m, 'se': se,
                              'win': self.win(base), 'points': {num(p['grid'][c[i]]): mm for mm, _, c in scored}})
                if take:
                    cur = best
                    moved = True
            if not moved:
                break
        return cur, trail, True

    def profile(self, best):
        """Each parameter's grid points within `--profile-reach` steps of `best` (0: the whole
        grid), the others at `best`: (point, gain vs best, SE, credits left a seat). One
        experiment a parameter, so a stopped profile resumes from the cache."""
        reach = self.a.profile_reach or 99
        base = self.play([self.spec(best)], 'sweep')[self.spec(best)]
        out = []
        for i, p in enumerate(self.params):
            if self.a.budget and self.elapsed() > self.a.budget:
                self.log(f'budget of {self.a.budget} min spent: the profile stops before {p["key"]}')
                break
            cells = {}
            for j in range(len(p['grid'])):
                if abs(j - best[i]) <= reach:
                    c = list(best)
                    c[i] = j
                    cells[j] = self.spec(tuple(c))
            r = self.play([self.spec(best)] + [s for s in cells.values() if s != self.spec(best)], 'sweep')
            pts = []
            for j, s in cells.items():
                m, se, _ = self.gap(base, r[s])
                pts.append((p['grid'][j], m, se, self.mean(r[s], 'credits')))
            self.log(f'profile {p["key"]}: ' + ' '.join(f'{num(v)}:{m:+.1f}±{se:.1f}' for v, m, se, _ in pts))
            # Matters: some grid point plays worse than the best by more than 2 SE.
            matters = any(se > 0 and -m > 2 * se for _, m, se, _ in pts)
            out.append({'key': p['key'], 'best': p['grid'][best[i]], 'hand': p['hand'], 'points': pts,
                        'matters': matters, 'says': p['says']})
        return self.win(base), out

    def confirm(self, best):
        agent, ref = self.cfg['agent'], self.cfg['reference']
        b = self.spec(best)
        rc = self.play([agent, b, ref], 'confirm', keep='confirm')
        rs = self.play([agent, b, ref], 'sweep')
        res = {}
        for which, r in (('confirm', rc), ('sweep', rs)):
            hand, swept, greedy = r[agent], r[b], r[ref]
            res[which] = {
                'n': len(hand),
                'win': {'hand': self.win(hand), 'best': self.win(swept), 'greedy': self.win(greedy)},
                'gain': self.gap(hand, swept),
                'lead_before': self.gap(greedy, hand),
                'lead_after': self.gap(greedy, swept),
                'credits': {k: self.mean(v, 'credits') for k, v in (('hand', hand), ('best', swept), ('greedy', greedy))},
                'credits_gap': self.gap(hand, swept, 'credits'),
                'rounds': {k: self.mean(v, 'rounds') for k, v in (('hand', hand), ('best', swept), ('greedy', greedy))},
                'by_pair': {pair: self.gap({u: x for u, x in hand.items() if u.startswith(pair + '/')}, swept)
                            for pair in self.cfg['pairs']},
            }
        return res

    def read(self, best, top=30):
        """Counters of the confirm games, best setting less hand-set, paired by game, the
        largest standardized gaps first."""
        d = os.path.join(self.a.out, 'runs', 'confirm')
        b = self.spec(best)
        if self.kept('confirm') != self.keep_tag([self.cfg['agent'], b, self.cfg['reference']], 'confirm'):
            self.log('read: no confirm run of this setting on these seeds and build: run --phase confirm')
            return None
        lines = stream(d)
        labels = cell_labels(next(lines)['spec'])
        per = {self.cfg['agent']: {}, b: {}}
        for g in lines:
            variant, agent = labels[g['cell']].split('/', 1)
            if agent in per:
                c = dict(g['counters'])
                for k, v in g.get('cards', {}).items():
                    if isinstance(v, dict):
                        for kk, vv in v.items():
                            if isinstance(vv, (int, float)):
                                c[f'card.{kk}.{k}'] = vv
                c['rounds'] = g['rounds']
                c['credits_left'] = sum(u[0] for u in g['margin']['unspent'])
                # unspent is [credits, chips, energy, antimatter, ambassadors] a seat (board::LEFT).
                c['chips_left'] = sum(u[1] for u in g['margin']['unspent'])
                c['energy_left'] = sum(u[2] for u in g['margin']['unspent'])
                per[agent][f"{variant}/{g['seed']}"] = c
        hand, swept = per[self.cfg['agent']], per[b]
        keys = sorted({k for g in hand.values() for k in g} | {k for g in swept.values() for k in g})
        rows = []
        for k in keys:
            d_ = [swept[u].get(k, 0) - hand[u].get(k, 0) for u in hand if u in swept]
            n = len(d_)
            m = sum(d_) / n
            sd = math.sqrt(sum((x - m) ** 2 for x in d_) / (n - 1)) if n > 1 else 0
            if sd == 0:
                continue
            h = sum(hand[u].get(k, 0) for u in hand) / len(hand)
            rows.append((abs(m) / (sd / math.sqrt(n)), k, h, h + m, m, sd / math.sqrt(n)))
        rows.sort(reverse=True)
        return rows[:top]


def table_md(rows, head):
    out = ['| ' + ' | '.join(head) + ' |', '|' + '---|' * len(head)]
    out += ['| ' + ' | '.join(str(c) for c in r) + ' |' for r in rows]
    return '\n'.join(out)


def pm(g):
    return f'{g[0]:+.1f} ±{g[1]:.1f}'


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    ap.add_argument('--config', default=os.path.join(HERE, 'agent_sweep.json'))
    ap.add_argument('--sim', default=os.environ.get('AIC_SIM', os.path.join('target', 'release', 'aic-sim.exe' if os.name == 'nt' else 'aic-sim')))
    ap.add_argument('--out', help='default out/agent-sweep/rNN')
    ap.add_argument('--threads', type=int, default=6)
    ap.add_argument('--phase', choices=['all', 'sweep', 'profile', 'confirm', 'read'], default='all')
    ap.add_argument('--sweep-seed', type=int)
    ap.add_argument('--sweep-games', type=int)
    ap.add_argument('--confirm-seed', type=int)
    ap.add_argument('--confirm-games', type=int)
    ap.add_argument('--accept', type=float, default=1.5, help='SE a step must gain by (default 1.5)')
    ap.add_argument('--reach', type=int, default=2, help='grid steps the first pass tries each way (default 2)')
    ap.add_argument('--max-passes', type=int, default=3)
    ap.add_argument('--budget', type=float, default=0, help='minutes the search may take (0: no cap)')
    ap.add_argument('--profile-reach', type=int, default=1,
                    help='grid steps each way the profile plays around the best (default 1; 0: the whole grid)')
    ap.add_argument('--best', help='a setting to profile and confirm instead of the search\'s, as `key=v,...`')
    a = ap.parse_args()
    with open(a.config, encoding='utf-8') as f:
        cfg = json.load(f)
    rev = revision()
    a.out = a.out or os.path.join('out', 'agent-sweep', f'r{rev}')
    a.sweep_seed = a.sweep_seed if a.sweep_seed is not None else cfg['sweep_seeds']['first']
    a.sweep_games = a.sweep_games or cfg['sweep_seeds']['games']
    a.confirm_seed = a.confirm_seed if a.confirm_seed is not None else cfg['confirm_seeds']['first']
    a.confirm_games = a.confirm_games or cfg['confirm_seeds']['games']
    if not os.path.exists(a.sim):
        sys.exit(f'no simulator at {a.sim}: cargo build --release -p aic-sim, or --sim PATH')
    # Windows' CreateProcess finds no relative `out/x.exe`.
    a.sim = os.path.abspath(a.sim)
    for p in cfg['params']:
        if p['hand'] not in p['grid']:
            sys.exit(f"{p['key']}: hand-set {p['hand']} is not on its grid {p['grid']}")
    s = Sweep(a, cfg)
    s.log(f'r{rev}, {a.sim} ({s.sim_hash}), sweep seeds {s.seeds_key("sweep")}, confirm {s.seeds_key("confirm")}, out {a.out}')

    state_path = os.path.join(a.out, 'state.json')
    state = {}
    # A saved state holds grid indexes: it stands only for the same keys and grids.
    grids = [[p['key'], p['hand'], p['grid']] for p in cfg['params']]
    if os.path.exists(state_path):
        with open(state_path, encoding='utf-8') as f:
            state = json.load(f)
        if state.get('sim') != s.sim_hash or state.get('seeds') != s.seeds_key('sweep') \
                or state.get('grids') != grids:
            state = {}

    def save():
        state.update({'sim': s.sim_hash, 'seeds': s.seeds_key('sweep'), 'rev': rev, 'grids': grids})
        with open(state_path, 'w', encoding='utf-8', newline='\n') as f:
            json.dump(state, f, indent=1)

    if a.best is not None:
        want = dict(kv.split('=') for kv in a.best.split(',') if kv)
        best = tuple(p['grid'].index(float(want[p['key']])) if p['key'] in want else p['grid'].index(p['hand'])
                     for p in cfg['params'])
        if state.get('best') != list(best):
            for k in ('profile', 'confirm', 'read'):
                state.pop(k, None)
        state['best'] = list(best)
    if a.phase in ('all', 'sweep') and a.best is None:
        best, trail, finished = s.sweep()
        if state.get('best') != list(best):
            for k in ('profile', 'confirm', 'read'):
                state.pop(k, None)
        state.update({'best': list(best), 'trail': trail, 'finished': finished})
        save()
        s.log(f'search best: {s.short(best)}')
    if 'best' not in state:
        sys.exit('no best setting yet: run --phase sweep first, or give --best')
    best = tuple(state['best'])
    if a.phase in ('all', 'profile'):
        win, prof = s.profile(best)
        state['profile'] = {'win': win, 'params': prof}
        save()
    if state.get('confirm', {}).get('seeds') not in (None, s.seeds_key('confirm')):
        state.pop('confirm', None)
        state.pop('read', None)
    if a.phase in ('all', 'confirm'):
        state['confirm'] = s.confirm(best)
        state['confirm']['seeds'] = s.seeds_key('confirm')
        save()
    if a.phase in ('all', 'read', 'confirm'):
        state['read'] = s.read(best)
        save()
    write_summary(s, state, best, rev)
    s.log(f'done: {s.games_played} games played this run, {s.elapsed():.0f} min; {os.path.join(a.out, "summary.md")}')


def write_summary(s, state, best, rev):
    cfg, a = s.cfg, s.a
    L = [f'# Agent sweep, r{rev}', '',
         f'{cfg["agent"]} at d{cfg["difficulty"]}, pairs {", ".join(cfg["pairs"])}; sweep seeds {s.seeds_key("sweep")}, '
         f'confirm seeds {s.seeds_key("confirm")}; binary {s.sim_hash}. Written by scripts/tune/agent_sweep.py at {clock()}.', '',
         f'**Best setting:** `{s.short(best)}`', '', f'Spec: `{s.spec(best)}`', '']
    c = state.get('confirm')
    if c:
        for which in ('confirm', 'sweep'):
            r = c[which]  # noqa
            L += [f'## {"Held-out confirm" if which == "confirm" else "On the sweep seeds"} ({r["n"]} games a setting)', '',
                  table_md([
                      ['won %', f'{r["win"]["hand"]:.1f}', f'{r["win"]["best"]:.1f}', f'{r["win"]["greedy"]:.1f}'],
                      ['credits left a seat', f'{r["credits"]["hand"]:.1f}', f'{r["credits"]["best"]:.1f}', f'{r["credits"]["greedy"]:.1f}'],
                      ['rounds', f'{r["rounds"]["hand"]:.2f}', f'{r["rounds"]["best"]:.2f}', f'{r["rounds"]["greedy"]:.2f}'],
                  ], ['', 'hand-set', 'best', 'greedy']), '',
                  f'- best less hand-set: {pm(r["gain"])} points (paired); by pair: '
                  + ', '.join(f'{k} {pm(v)}' for k, v in r['by_pair'].items()),
                  f'- strategist less greedy: before {pm(r["lead_before"])}, after {pm(r["lead_after"])}',
                  f'- credits left a seat, best less hand-set: {pm(r["credits_gap"])}', '']
    p = state.get('profile')
    if p:
        L += [f'## Profile around the best ({p["win"]:.1f}% won on the sweep seeds)', '',
              f'Each parameter within {a.profile_reach or "all"} grid step(s) of the best, the others at the best setting: '
              'won-% gain over the best, paired, ± SE '
              '(credits left a seat after). *Matters*: some grid point plays worse than the best by over 2 SE.', '']
        rows = []
        for q in p['params']:
            pts = ' · '.join(f'{num(v)}{"*" if v == q["best"] else ""}: {m:+.1f} ±{se:.1f} ({cr:.0f})' for v, m, se, cr in q['points'])
            rows.append([f'`{q["key"]}`', num(q['hand']), num(q['best']), 'yes' if q['matters'] else 'no', pts])
        L += [table_md(rows, ['parameter', 'hand-set', 'best', 'matters', 'grid (* best)']), '']
    t = state.get('trail')
    if t:
        L += ['## Search trail', '', table_md(
            [[x['pass'], f'`{x["key"]}`', num(x['from']), num(x['to']), f'{x["win"]:.1f}', f'{x["gain"]:+.1f} ±{x["se"]:.1f}',
              ' '.join(f'{k}:{v:+.1f}' for k, v in sorted(x['points'].items(), key=lambda kv: float(kv[0])))] for x in t],
            ['pass', 'parameter', 'from', 'to', 'won % before', 'best step', 'each point']), '']
        if not state.get('finished', True):
            L += ['The search stopped at its time budget.', '']
    r = state.get('read')
    if r:
        L += ['## What the best setting does differently (confirm games, per game, paired)', '',
              table_md([[k, f'{h:.2f}', f'{b:.2f}', f'{m:+.2f} ±{se:.2f}'] for _, k, h, b, m, se in r],
                       ['counter', 'hand-set', 'best', 'gap']), '']
    with open(os.path.join(a.out, 'summary.md'), 'w', encoding='utf-8', newline='\n') as f:
        f.write('\n'.join(L))


if __name__ == '__main__':
    sys.exit(main())

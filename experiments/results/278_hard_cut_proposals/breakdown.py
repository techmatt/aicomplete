"""278: every minute of a traced round at 268's rates, by phase and step and by kind.

    python breakdown.py DIR [--level 2] [--json OUT.json]
    python breakdown.py BASE_DIR FLAG_DIR --compare [--level 2]

DIR holds `traces.sh`'s output: `aic-sim run --trace` of 268's seeds as d<level>-<seed>.out, and the same games as an
experiment in DIR/exp/games.jsonl (per-round counters). Run from the repo root (reads content/ais.json and 268's log).

Rates (268): 15 s a decision, 3 s a component, 10 s a check, 20 s a lookup, 5 s a modifier. Counting, per event:
  decisions   an action with more than one option (274-276's count, at the full 15 s), attributed to the phase and,
              in the action phase, to the turn it belongs to (a power's, play's or Action's later choices included)
  components  a token moved (|delta|), a resource paid or gained (one a transaction), a card moved, a track marker
              moved (Discord, Unity, tips, research, compute and each project's compute track, card tokens, the
              node track's space), a used marker set (power, Buy energy, converter, Vigil) and cleared at cleanup
  checks      adversary turn: `map.adversary_checks` (each site a line names, each arrival, each botnet) in its
              lines' step, each troop on the map one in the March, each threat card turned in dealing one;
              the action phase: the board uses a seat could make (`ai.offered.<board>`, 276) at its phase's
              start and each target a power or card asks for, and 5 a Buy energy (268: the host matching re-run;
              1 with `board.host_fixed`); production: every factory, station and city read
              for its producer and blockers, plus each host slot matched; threats executed: each threat on the map;
              war: each front (one a country)
  faces       a card's first showing in the game (a draft packet, a threat placed, an adversary event or project
              card): 268's 20 s a new face read, counted as a lookup
  lookups,    not in the trace: 268's hand-play log (r125, d2 seed 268003) gives them per round step; its per-round
  modifiers   totals (counts.csv) are shared out by the steps' parsed shares, then 274's income tracks (E5's
              modifiers) and 275's growth and natural-corruption removal (E6-8) are taken out. Marked "modeled".
"""
import argparse
import collections
import glob
import json
import os
import re
import statistics

RATE = {'decisions': 15, 'components': 3, 'checks': 10, 'lookups': 20, 'modifiers': 5}
KINDS = list(RATE)
ACT = 'cards.play+threats.counter+board.act+ai.power'
SECTION = {
    '': 'Round start', 'alignment.sway': 'Sway', 'adversary.event': 'Adversary: event card',
    'threats.deal': 'Adversary: threats dealt', 'adversary.build': 'Adversary: lines and arrivals',
    'board.march': 'Adversary: march', 'adversary.reveal': 'Adversary: next card', 'adversary.compute':
    'Adversary: compute and projects', 'cards.deal': 'Draft', 'cards.tier': 'Draft', 'cards.draft': 'Draft',
    'cards.keep': 'Draft', 'board.produce': 'Production and hosting', 'economy.upkeep': 'Income and upkeep',
    'board.bases': 'Bases', 'threats.execute': 'Threats executed', 'board.war': 'War', 'board.cleanup': 'Cleanup',
}
TRACK_KEYS = ('alignment.discord', 'alignment.unity', 'alignment.unity_tips', 'alignment.discord_tips',
              'adversary.research', 'adversary.compute_waiting', 'adversary.project.', 'board.project.',
              'cards.tokens.', 'board.track.', 'threats.jam.', 'threats.late.')
MARKERS = ('ai.power_used.', 'board.energy_bought.', 'board.converter_round.', 'board.chip_converter_round.',
           'board.vigil_round.', 'board.moved.', 'cards.used.')
SEATS = ('Builder', 'Protector')
BUY_CHECKS = 5  # 268's hand play: a Buy energy reads the unblocked neutral cities against the host slots
HAND_LOG = 'experiments/results/268_table_cost_audit/d2/log.md'
HAND_COUNTS = 'experiments/results/268_table_cost_audit/d2/counts.csv'
# 268's d4 spot check (rounds 1, 5, 10): lookups and modifiers a round.
D4_HAND = {'lookups': (14 + 22 + 24) / 3, 'modifiers': (17 + 34 + 44) / 3}


def boards():
    with open('content/ais.json', encoding='utf-8') as f:
        return [[p['name'] for p in b['powers']] for b in json.load(f)]


def act_step(a, names):
    """The action-phase row a seat's turn belongs to."""
    t = a['type']
    if t == 'power':
        s = a['seat']
        i = a['power']
        return 'Growth' if i < 3 else f"Power: {names[s][i] if i < len(names[s]) else i}"
    if t == 'standard':
        return f"Standard: {a.get('act', '?')}"
    return {'play': 'Card plays', 'use': 'Building Actions', 'convert': 'Converters', 'pay': 'Threat counters',
            'pass': 'Passing'}.get(t, t)


def read_trace(path, names):
    """Per round: Counter keyed (section, step, kind). Steps of the action phase and draft carry the seat."""
    rows = collections.defaultdict(collections.Counter)
    rnd, phase = 0, ''
    turn = {}  # seat -> step of the turn in progress
    actor = None
    offered = {}
    moved = set()
    faces = set()
    with open(path, encoding='utf-8') as f:
        # The run summary (with its overrides line) follows the trace: read it first.
        host_fixed = any(x.startswith('run overrides:') and 'board.host_fixed=1' in x for x in f)
    for line in open(path, encoding='utf-8'):
        if line.startswith('ai.offered.'):
            parts = line.split()
            offered[parts[0].split('.')[2]] = float(parts[-1])
        if not line.startswith('{'):
            continue
        d = json.loads(line)
        sec = SECTION.get(phase, 'Action phase') if phase != ACT else 'Action phase'
        if 'action' in d:
            a, n = d['action'], d.get('options', 1)
            s = a.get('seat')
            if phase == ACT and s is not None:
                if a['type'] != 'choose':
                    turn[s] = act_step(a, names)
                step = f"{SEATS[s]}: {turn.get(s, 'Card plays')}"
                actor = s
                if a['type'] == 'standard' and a.get('act') == 'buy_energy':
                    # A city is for sale only if no player hosts or could host it: the table re-runs the round's
                    # matching (268 counted 5 checks a buy); with hosts fixed it reads the host tokens (1).
                    rows[rnd][(sec, step, 'checks')] += 1 if host_fixed else BUY_CHECKS
            elif sec == 'Draft' and s is not None:
                step = f"{SEATS[s]}: {dict(tier='tier choice', draft='pick', keep='keep').get(a['type'], a['type'])}"
            else:
                step = 'choices'
            if n > 1:
                rows[rnd][(sec, step, 'decisions')] += 1
            continue
        e = d.get('event')
        if not e:
            continue
        t = e['type']
        if t == 'round_started':
            rnd = e['round']
            continue
        if t == 'phase_started':
            phase = e['phase']
            actor = None
            turn = {}
            continue
        if rnd == 0:
            continue
        if phase == ACT:
            seat = e.get('seat', actor)
            step = f"{SEATS[seat]}: {turn.get(seat, 'Card plays')}" if seat is not None else 'shared'
        elif sec == 'Draft':
            seat = e.get('seat')
            step = f"{SEATS[seat]}: cards and keeps" if seat is not None else 'cards dealt and passed'
        else:
            step = {'threats.deal': 'cards turned and placed'}.get(phase, 'pieces, cards and tracks')
        if t == 'tokens_changed':
            rows[rnd][(sec, step, 'components')] += abs(e['delta'])
        elif t == 'resource_changed':
            rows[rnd][(sec, step, 'components')] += 1
        elif t == 'card_moved':
            rows[rnd][(sec, step, 'components')] += 1
            # 268's 20 s a new card face read: a card's first showing in the game (draft packets, threats placed,
            # the adversary's event and project cards), read by the table once.
            to = e['to']
            if (to.startswith(('cards.packet', 'threats.on', 'adversary.next', 'adversary.projects'))
                    and e['card'] not in faces):
                faces.add(e['card'])
                rows[rnd][(sec, 'card faces read (first showing)', 'lookups')] += 1
            if phase == 'threats.deal' and e['from'].startswith('threats.deck'):
                rows[rnd][(sec, step, 'checks')] += 1
            if phase == 'threats.execute' and e['from'].startswith('threats.on'):
                rows[rnd][(sec, 'threats read', 'checks')] += 1
        elif t == 'var_changed':
            k = e['key']
            if k.startswith(TRACK_KEYS):
                # A marker moved N spaces is one component (268): once a key a phase.
                if (rnd, phase, k) not in moved:
                    moved.add((rnd, phase, k))
                    rows[rnd][(sec, step, 'components')] += 1
            elif k.startswith(MARKERS):
                rows[rnd][(sec, step if phase == ACT else 'used markers cleared', 'components')] += 1
        elif t == 'asked' and phase == ACT and 'target' in e.get('prompt', '').lower() and actor is not None:
            rows[rnd][(sec, step, 'checks')] += 1
        elif t == 'note' and phase == 'board.war' and e['text'].startswith('War: the fronts fight'):
            rows[rnd][(sec, 'fronts', 'checks')] += 2  # one front a country (2 countries at 2 players)
    played = max(rows) if rows else 1
    for s, b in enumerate(('builder', 'protector')):
        for r in range(1, played + 1):
            rows[r][('Action phase', f'{SEATS[s]}: weighing board uses', 'checks')] += offered.get(b, 0) / played
    return rows


def add_counters(rows, g):
    """Per-round counters from the experiment's games.jsonl line for the same game."""
    c = g['counters']
    for r in rows:
        tag = f'r{r:02d}'
        rows[r][('Adversary: lines and arrivals', 'sites, arrivals, botnets', 'checks')] += c.get(f'map.adversary_checks.{tag}', 0)
        rows[r][('Adversary: march', 'troops', 'checks')] += c.get(f'map.troops.{tag}', 0)
        sites = sum(c.get(f'map.{k}.{o}.{tag}', 0) for k in ('factories', 'stations') for o in ('adversary', 'neutral', 'player'))
        rows[r][('Production and hosting', 'sites read', 'checks')] += sites + 6  # + 6 cities (2 countries x 3)
        rows[r][('Production and hosting', 'host matching', 'checks')] += c.get(f'map.hosted.{tag}', 0)


def hand_model(level):
    """Lookups and modifiers a round by section (modeled from 268's r125 hand play)."""
    shares = {'lookups': collections.Counter(), 'modifiers': collections.Counter()}
    rnd = None
    for line in open(HAND_LOG, encoding='utf-8'):
        m = re.match(r'## Round (\d+)', line)
        if m:
            rnd = int(m.group(1))
            continue
        if not rnd or not line.startswith('| ') or line.startswith('| step') or line.startswith('|---'):
            continue
        cells = [x.strip() for x in line.strip().strip('|').split('|')]
        if len(cells) < 7 or cells[0].startswith('**'):
            continue
        st = cells[0]
        if st.startswith('A'):
            sec = 'Sway'
        elif st.startswith('B'):
            low = st.lower()
            sec = ('Adversary: threats dealt' if 'threat' in low else 'Adversary: march' if 'march' in low else
                   'Adversary: compute and projects' if 'income' in low else 'Adversary: lines and arrivals')
        elif st.startswith('C'):
            sec = 'Draft'
        elif st.startswith('D Builder'):
            sec = 'Action phase|Builder'
        elif st.startswith('D Protector'):
            sec = 'Action phase|Protector'
        elif st.startswith('E1') or st.startswith('E3'):
            sec = 'Production and hosting'
        elif st.startswith('E5'):
            sec = 'Income and upkeep|E5'
        elif st.startswith('E6'):
            sec = 'removed: city growth'
        elif st.startswith('F'):
            sec = 'Threats executed'
        elif st.startswith('G'):
            sec = 'War'
        elif st.startswith('Round start'):
            sec = 'Round start'
        else:
            continue
        lk = 0 if cells[1] in ('—', '-', '') else len(re.findall(r'§\d+', cells[1]))
        # A cell's count is its "total N" if it has one, else its last "= N", else its one number.
        mods = (re.findall(r'total\s*~?(\d+)', cells[2]) or re.findall(r'=\s*~?(\d+)', cells[2])
                or re.findall(r'\d+', cells[2]))
        md = int(mods[-1]) if mods and cells[2] not in ('0', '—') else 0
        shares['lookups'][sec] += lk
        shares['modifiers'][sec] += md
    with open(HAND_COUNTS, encoding='utf-8') as f:
        rows = [r.split(',') for r in f.read().split('\n')[1:] if r and not r.startswith('0,')]
    per_round = {'lookups': statistics.mean(int(r[1]) for r in rows), 'modifiers': statistics.mean(int(r[2]) for r in rows)}
    if level == 4:
        per_round = dict(D4_HAND)
    out = collections.Counter()
    for kind in ('lookups', 'modifiers'):
        tot = sum(shares[kind].values()) or 1
        for sec, v in shares[kind].items():
            if sec.startswith('removed') or sec.endswith('|E5') and kind == 'modifiers':
                continue  # 275 removed city growth; 274's income tracks replaced E5's sums
            sec2, _, seat = sec.partition('|')
            step = f'{seat}: rules and arithmetic (modeled)' if seat and seat != 'E5' else 'rules and arithmetic (modeled)'
            out[(sec2, step, kind)] += per_round[kind] * v / tot
    return out


def load(folder, level):
    names = boards()
    games = {}
    exp = {}
    jp = os.path.join(folder, 'exp', 'games.jsonl')
    if os.path.exists(jp):
        for line in open(jp, encoding='utf-8'):
            g = json.loads(line)
            if 'seed' in g:
                exp[(int(g['difficulty']), int(g['seed']))] = g
    for p in sorted(glob.glob(os.path.join(folder, f'd{level}-*.out'))):
        seed = int(re.search(r'-(\d+)\.out$', p).group(1))
        rows = read_trace(p, names)
        g = exp.get((level, seed))
        if g:
            add_counters(rows, g)
        games[seed] = rows
    return games


def per_round(games, level):
    """Mean a round (pooled over every game-round) of each (section, step, kind), with the modeled kinds added."""
    tot = collections.Counter()
    n = 0
    for rows in games.values():
        for r, c in rows.items():
            if r == 0:
                continue
            n += 1
            tot.update(c)
    mean = collections.Counter({k: v / max(n, 1) for k, v in tot.items()})
    mean.update(hand_model(level))
    rounds = statistics.mean(len([r for r in rows if r]) for rows in games.values())
    return mean, rounds


def minutes(c):
    return sum(v * RATE[k[2]] for k, v in c.items()) / 60


def table(mean, rounds, level):
    secs = collections.defaultdict(lambda: collections.defaultdict(collections.Counter))
    for (sec, step, kind), v in mean.items():
        secs[sec][step][kind] += v
    order = ['Round start', 'Sway', 'Adversary: event card', 'Adversary: threats dealt', 'Adversary: lines and arrivals',
             'Adversary: march', 'Adversary: next card', 'Adversary: compute and projects', 'Draft', 'Action phase',
             'Production and hosting', 'Income and upkeep', 'Bases', 'Threats executed', 'War', 'Cleanup']
    out = [f'### d{level}: a round, {rounds:.2f} rounds a game', '',
           '| Section | Step | Decisions | Components | Checks | Lookups | Modifiers | Minutes |', '|---|---|---|---|---|---|---|---|']
    total = collections.Counter()
    for sec in order + sorted(set(secs) - set(order)):
        if sec not in secs:
            continue
        sub = collections.Counter()
        small = collections.Counter()
        steps = secs[sec]
        for step in sorted(steps, key=lambda s: -sum(steps[s][k] * RATE[k] for k in KINDS)):
            k = steps[step]
            m = sum(k[x] * RATE[x] for x in KINDS) / 60
            sub.update(k)
            if m < 0.05 and len(steps) > 1:
                small.update(k)  # shown as one "other steps" row
                continue
            out.append(f"| {sec} | {step} | " + ' | '.join(f'{k[x]:.1f}' for x in KINDS) + f' | {m:.2f} |')
        if small:
            m = sum(small[x] * RATE[x] for x in KINDS) / 60
            out.append(f"| {sec} | other steps (each under 0.05) | " + ' | '.join(f'{small[x]:.1f}' for x in KINDS) + f' | {m:.2f} |')
        total.update(sub)
        m = sum(sub[x] * RATE[x] for x in KINDS) / 60
        out.append(f"| **{sec}** | **all** | " + ' | '.join(f'**{sub[x]:.1f}**' for x in KINDS) + f' | **{m:.2f}** |')
    m = sum(total[x] * RATE[x] for x in KINDS) / 60
    out.append('| **Round** | | ' + ' | '.join(f'**{total[x]:.1f}**' for x in KINDS) + f' | **{m:.1f}** |')
    out.append('| *minutes by kind* | | ' + ' | '.join(f'{total[x] * RATE[x] / 60:.1f}' for x in KINDS) + f' | {m:.1f} |')
    return '\n'.join(out), m


def extras(folder, level):
    """Counts the write-up quotes beside the table, a round (pooled over every game-round)."""
    n = 0
    c = collections.Counter()
    by_round = collections.defaultdict(list)  # round -> Builder card plays, one entry a game
    for p in sorted(glob.glob(os.path.join(folder, f'd{level}-*.out'))):
        phase, rnd = '', 0
        plays = collections.Counter()
        for line in open(p, encoding='utf-8'):
            if not line.startswith('{'):
                continue
            d = json.loads(line)
            e = d.get('event')
            if e and e['type'] == 'phase_started':
                phase = e['phase']
            elif e and e['type'] == 'round_started':
                rnd = e['round']
                n += 1
            elif e and e['type'] == 'card_moved' and phase == 'threats.deal' and e['from'].startswith('threats.deck'):
                c['turned'] += 1
                c['dealt' if e['to'].startswith('threats.on') else 'rejected'] += 1
            elif 'action' in d and phase == ACT:
                t, s = d['action']['type'], d['action'].get('seat')
                if t in ('power', 'standard', 'use', 'convert'):
                    c[f'nonc{s}'] += 1
                if t == 'play' and s == 0:
                    plays[rnd] += 1
        for r in range(1, rnd + 1):
            by_round[r].append(plays[r])
    jp = os.path.join(folder, 'exp', 'games.jsonl')
    arr = []
    if os.path.exists(jp):
        for line in open(jp, encoding='utf-8'):
            g = json.loads(line)
            if g.get('difficulty') == level:
                k = g['counters']
                arr.extend(k.get(f'map.arrival_checks.r{i:02d}', 0) for i in range(1, g['rounds'] + 1))
    n = max(n, 1)
    late = ', '.join(f'r{r} {statistics.mean(v):.1f}' for r, v in sorted(by_round.items()) if len(v) >= 3)
    return (f"Counts a round: arrival checks {statistics.mean(arr) if arr else 0:.1f} (within the adversary's checks); "
            f"threat cards turned in dealing {c['turned'] / n:.1f} ({c['dealt'] / n:.1f} dealt, {c['rejected'] / n:.1f} "
            f"rejected); non-card turns (powers, standard actions, building Actions, converters) Builder "
            f"{c['nonc0'] / n:.1f}, Protector {c['nonc1'] / n:.1f}; Builder card plays by round (rounds 3+ games "
            f"reach): {late}.")


def seat_split(mean):
    """Minutes in each seat's own rows (its draft and action phase) and outside them."""
    seat = collections.Counter()
    for (sec, step, kind), v in mean.items():
        who = next((s for s in SEATS if step.startswith(s + ':')), None)
        seat[who or 'outside'] += v * RATE[kind] / 60
    return seat


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--level', type=int, default=2)
    ap.add_argument('--compare', action='store_true')
    ap.add_argument('--json')
    a = ap.parse_args()
    if a.compare:
        gb, gn = load(a.dirs[0], a.level), load(a.dirs[1], a.level)
        base, rb = per_round(gb, a.level)
        new, rn = per_round(gn, a.level)
        # Paired by seed: each game's minutes a round, the flag's side less base's (the games diverge once the
        # rule bites, so this SE is the drift a section's direct cut is read against).
        diffs = [minutes(per_round({s: gn[s]}, a.level)[0]) - minutes(per_round({s: gb[s]}, a.level)[0])
                 for s in gb if s in gn]
        se = statistics.stdev(diffs) / len(diffs) ** 0.5 if len(diffs) > 1 else 0
        bysec = collections.Counter()
        for k in set(base) | set(new):
            bysec[k[0]] += (new[k] - base[k]) * RATE[k[2]] / 60
        bykind = collections.Counter()
        for k in set(base) | set(new):
            bykind[k[2]] += (new[k] - base[k]) * RATE[k[2]] / 60
        mb, mn = minutes(base), minutes(new)
        print(f'd{a.level}: {mb:.1f} -> {mn:.1f} minutes a round, pooled ({mn - mb:+.1f}; per game, paired '
              f'{statistics.mean(diffs):+.1f} +/- {se:.1f}); rounds a game {rb:.2f} -> {rn:.2f}; '
              f'game {mb * rb:.0f} -> {mn * rn:.0f} minutes')
        print('  by section: ' + ', '.join(f'{s} {v:+.2f}' for s, v in sorted(bysec.items(), key=lambda x: x[1]) if abs(v) >= 0.05))
        print('  by kind: ' + ', '.join(f'{s} {v:+.2f}' for s, v in bykind.items() if abs(v) >= 0.05))
        if a.json:
            with open(a.json, 'w', encoding='utf-8', newline='\n') as f:
                json.dump({'level': a.level, 'base': mb, 'new': mn, 'se': se, 'rounds_base': rb, 'rounds_new': rn,
                           'by_section': bysec, 'by_kind': bykind}, f, indent=1)
        return
    mean, rounds = per_round(load(a.dirs[0], a.level), a.level)
    text, m = table(mean, rounds, a.level)
    print(text)
    print('\n' + extras(a.dirs[0], a.level))
    s = seat_split(mean)
    print(f"\nIn the seats' own rows (draft and action phase): Builder {s['Builder']:.1f}, Protector {s['Protector']:.1f}; "
          f"outside them {s['outside']:.1f} (shared draft dealing, adversary turn, production, threats, war, cleanup and "
          f"the modeled lookups and modifiers not tied to a seat). Game: {m:.1f} x {rounds:.2f} = {m * rounds:.0f} minutes.")
    if a.json:
        with open(a.json, 'w', encoding='utf-8', newline='\n') as f:
            json.dump({'level': a.level, 'rounds': rounds, 'minutes': m,
                       'rows': [[k[0], k[1], k[2], v] for k, v in sorted(mean.items())]}, f, indent=1)


main()

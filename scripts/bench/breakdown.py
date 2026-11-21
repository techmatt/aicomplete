"""Every minute of a traced round at 268's rates, by phase and step and by kind (278's breakdown, 280's tracks,
298's fixes). Matt's ruling: the sim's minutes are assumed wrong until he plays it and corrects them; this is the
yardstick, not a clock.

    python scripts/bench/breakdown.py DIR [--level 2] [--json OUT.json] [--sites]
    python scripts/bench/breakdown.py BASE_DIR FLAG_DIR --compare [--level 2] [--sites]

DIR holds traces of judge games: `aic-sim run configs/baseline.json --games 1 --seed S --agent strategist --set
adversary.difficulty=L --trace` as DIR/d<L>-<S>.out, and the same games as an experiment in DIR/exp/games.jsonl
(per-round counters; 278's traces.sh is the recipe). Run from the repo root (reads content/ais.json and 268's log).

Rates (268): 15 s a decision, 3 s a component, 10 s a check, 20 s a lookup or a new card face, 5 s a modifier.
Counting, per event:
  decisions   an action with more than one option, attributed to the phase and, in the action phase, to the turn it
              belongs to (a power's, play's or Action's later choices included)
  components  a token moved (|delta|), a resource paid or gained (one a transaction), a card moved, a track marker
              moved (once a key a phase), a used marker set and cleared (a tiered power's tiers share one marker),
              in the draft each hand passed
  checks      adversary turn: `map.adversary_checks` (each site a line names, each arrival, each botnet), each troop
              on the map in the March, each threat card turned in dealing; the action phase: the board uses a seat
              could make (`ai.offered.<board>`) and each target a power or card asks for;
              production: each producing income track, blocked chip and adversary factory (280's cut 2, the rule
              since r133; `--sites` reads every factory, station and city instead, 278's model); threats executed:
              each threat on the map
  faces       a card's first showing in the game (a draft hand, a threat placed, an adversary event or project card)
  lookups,    not in the trace: 268's r125 hand log's per-round totals shared out by step (278). Marked "modeled".
  modifiers

298's fixes to 278's model:
  At once     the draft and the action phase are played by both seats at once (rulebook: "All players act at once";
              each drafter tiers, picks and keeps their own hand). A seat's rows show what it does; an "overlap" row
              takes off the faster seat's time, so the section costs its shared rows plus the slower seat. The
              draft's tier choices, picks and keeps are three steps both seats finish before the next (the slower
              seat in each); the action phase is one step (each seat's whole phase).
  Draft rates drafting players are fast and skilled (Matt, 2026-11-18): a draft decision is 6 s (a tier between two
              decks, a pick among at most 4 known cards, a keep of a drafted card), not 268's 15 s timed on
              action-phase choices across the board; a face first shown in a draft hand is read inside the tier
              choice or pick that shows it (0 s), not 268's 20 s new-reader lookup. The draft's card moves belong
              to the seat whose hand they enter or leave, and each hand passed is one component.
  Node powers r142's three node powers a board (index 0-2, each with its own used marker), not 278's one Growth.
"""
import argparse
import collections
import glob
import json
import os
import re
import statistics

RATE = {'decisions': 15, 'components': 3, 'checks': 10, 'lookups': 20, 'faces': 20, 'modifiers': 5, 'overlap': 1}
DRAFT_RATE = {'decisions': 6, 'faces': 0}  # drafting players fast and skilled (298)
KINDS = ['decisions', 'components', 'checks', 'lookups', 'faces', 'modifiers']  # counted; 'overlap' is seconds
ACT = 'cards.play+threats.counter+board.act+ai.power'
SECTION = {
    '': 'Round start', 'alignment.sway': 'Sway', 'adversary.event': 'Adversary: event card',
    'threats.deal': 'Adversary: threats dealt', 'adversary.build': 'Adversary: lines and arrivals',
    'board.march': 'Adversary: march', 'adversary.reveal': 'Adversary: next card', 'adversary.compute':
    'Adversary: compute and projects', 'cards.deal': 'Draft', 'cards.tier': 'Draft', 'cards.draft': 'Draft',
    'cards.keep': 'Draft', 'board.produce': 'Production and hosting', 'economy.upkeep': 'Income and upkeep',
    'board.bases': 'Bases', 'threats.execute': 'Threats executed', 'board.war': 'War', 'board.cleanup': 'Cleanup',
}
DRAFT_STEP = {'cards.deal': 'opening deal', 'cards.tier': 'tier choice', 'cards.draft': 'pick', 'cards.keep': 'keep'}
AT_ONCE = ('Draft', 'Action phase')
TRACK_KEYS = ('alignment.discord', 'alignment.unity', 'alignment.unity_tips', 'alignment.discord_tips',
              'adversary.research', 'adversary.compute_waiting', 'adversary.project.', 'board.project.',
              'cards.tokens.', 'board.track.', 'board.track2.', 'threats.jam.', 'threats.late.')
MARKERS = ('ai.power_used.', 'board.energy_bought.', 'board.converter_round.', 'board.chip_converter_round.',
           'board.vigil_round.', 'board.moved.', 'cards.used.')
SEATS = ('Builder', 'Protector')
NODE_POWERS = 3  # r142: powers 0-2 of every board
HAND_LOG = 'experiments/results/268_table_cost_audit/d2/log.md'
HAND_COUNTS = 'experiments/results/268_table_cost_audit/d2/counts.csv'
# 268's d4 spot check (rounds 1, 5, 10): lookups and modifiers a round.
D4_HAND = {'lookups': (14 + 22 + 24) / 3, 'modifiers': (17 + 34 + 44) / 3}
HAND_SEAT = re.compile(r'^cards\.(packet|drafted|hand)\.(\d)$')


def rate(sec, kind):
    return DRAFT_RATE.get(kind, RATE[kind]) if sec == 'Draft' else RATE[kind]


def secs_of(c):
    return sum(v * rate(k[0], k[2]) for k, v in c.items())


def boards():
    """Each board's power names and each power's used-marker id (a tiered power's tiers share their group's)."""
    with open('content/ais.json', encoding='utf-8') as f:
        bs = json.load(f)
    return ([[p['name'] for p in b['powers']] for b in bs],
            [[p.get('group') or i for i, p in enumerate(b['powers'])] for b in bs])


def cities_in_play():
    """The cities on the traced map: content/map.json's first N countries' (the engine's), N the baseline's
    `countries` (the traces run configs/baseline.json)."""
    with open('configs/baseline.json', encoding='utf-8') as f:
        n = json.load(f)['countries']
    with open('content/map.json', encoding='utf-8') as f:
        return sum(len(c['cities']) for c in json.load(f)['countries'][:n])


def act_step(a, names):
    """The action-phase row a seat's turn belongs to."""
    t = a['type']
    if t == 'power':
        s = a['seat']
        i = a['power']
        return 'Node powers' if i < NODE_POWERS else f"Power: {names[s][i] if i < len(names[s]) else i}"
    if t == 'standard':
        return f"Standard: {a.get('act', '?')}"
    return {'play': 'Card plays', 'use': 'Building Actions', 'convert': 'Converters', 'pay': 'Threat counters',
            'pass': 'Passing'}.get(t, t)


class Tracks:
    """280, cut 2: the board replayed from the trace's tokens, for production read from the income tracks."""

    def __init__(self):
        self.tok = collections.defaultdict(collections.Counter)  # hex -> token -> count
        self.state = {}  # hex -> (producer, blocked)
        # hex -> its state before a cube came off: a cube swap (corruption takes the player's off, then puts the
        # adversary's on) is one change of hands, settled at the hex's next token or another hex's, or a phase's end.
        self.pending = {}

    def site(self, h):
        """(producer, blocked) of hex h: producer a (kind, owner) track, or None (neutral or no site)."""
        t = self.tok[h]
        owner = next((k.split(':')[1] for k, v in t.items() if k.startswith('cube:') and v > 0), None)
        if owner is None and t['station'] > 0:
            owner = 'neutral'  # a neutral station's cubes go onto its megaproject: the megaproject's marker
        if owner is None:
            return (None, False)
        players = sum(v for k, v in t.items() if k.startswith('ambassador:') and k != 'ambassador:adversary')
        agents = t['ambassador:adversary']
        kind = 'factory' if t['factory'] > 0 else 'station' if t['station'] > 0 else 'city'
        blocked = players > 0 if owner == 'adversary' else agents > 0
        return ((kind, owner), blocked)

    def change(self, h, token, delta):
        """Applies a token change; the components the table spends keeping the tracks and chips right."""
        self.tok[h][token] += delta
        if not (token.startswith(('cube:', 'ambassador:')) or token in ('factory', 'station')):
            return 0
        n = sum(self.settle(p) for p in list(self.pending) if p != h)
        if token.startswith('cube:') and delta < 0:
            self.pending.setdefault(h, self.state.get(h, (None, False)))
            return n
        return n + self.settle(h)

    def settle(self, h):
        """Brings hex h's state up to its tokens: the track markers moved and chips placed or lifted."""
        old = self.pending.pop(h, self.state.get(h, (None, False)))
        new = self.site(h)
        self.state[h] = new
        n = 0
        if old[0] != new[0]:
            n += (old[0] is not None) + (new[0] is not None)  # each producer's track marker moves
        was = old[0] is not None and old[1]
        now = new[0] is not None and new[1]
        n += was != now  # a chip placed or lifted
        return n

    def flush(self):
        return sum(self.settle(p) for p in list(self.pending))

    def reads(self):
        """What the table reads at production: each producing track, each chip, each adversary factory (it makes
        its troop where it stands, so it is read one by one, not as a track)."""
        tracks = {p for p, b in self.state.values() if p is not None and not b and p != ('factory', 'adversary')}
        chips = sum(1 for p, b in self.state.values() if p is not None and b)
        adv_factories = sum(1 for p, b in self.state.values() if p == ('factory', 'adversary') and not b)
        return len(tracks) + chips + adv_factories


def read_trace(path, info, tracks=True):
    """Per round: Counter keyed (section, step, kind). Steps of the action phase and draft carry the seat."""
    names, groups = info
    rows = collections.defaultdict(collections.Counter)
    board = Tracks() if tracks else None
    rnd, phase = 0, ''
    turn = {}  # seat -> step of the turn in progress
    actor = None
    offered = {}
    moved = set()
    marked = set()
    faces = set()
    picks = collections.Counter()  # seat -> picks this draft
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
            elif sec == 'Draft' and s is not None:
                step = f"{SEATS[s]}: {DRAFT_STEP.get(phase, phase)}"
                actor = s
                if a['type'] == 'draft':
                    picks[s] += 1
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
            if board is not None:
                n = board.flush()  # a cube swap left open by the phase that ends: counted in that phase
                if n and rnd > 0:
                    rows[rnd][(sec, 'income tracks and blocked chips', 'components')] += n
            if phase == 'cards.draft' and rnd > 0:
                for s, k in picks.items():
                    rows[rnd][('Draft', f'{SEATS[s]}: pick', 'components')] += max(k - 1, 0)  # each hand passed
            picks.clear()
            phase = e['phase']
            actor = None
            turn = {}
            if board is not None and phase == 'board.produce' and rnd > 0:
                rows[rnd][('Production and hosting', 'tracks and chips read', 'checks')] += board.reads()
            continue
        if board is not None and t == 'tokens_changed':
            n = board.change(tuple(e['hex']), e['token'], e['delta'])
            if n and rnd > 0:
                rows[rnd][(sec, 'income tracks and blocked chips', 'components')] += n
        if rnd == 0:
            continue
        if phase == ACT:
            seat = e.get('seat', actor)
            step = f"{SEATS[seat]}: {turn.get(seat, 'Card plays')}" if seat is not None else 'shared'
        elif sec == 'Draft':
            seat = e.get('seat')
            if seat is None and t == 'card_moved':
                m = HAND_SEAT.match(e['to']) or HAND_SEAT.match(e['from'])
                seat = int(m.group(2)) if m else actor
            step = f"{SEATS[seat]}: {DRAFT_STEP.get(phase, phase)}" if seat is not None else 'shared'
        else:
            step = {'threats.deal': 'cards turned and placed'}.get(phase, 'pieces, cards and tracks')
        if t == 'tokens_changed':
            rows[rnd][(sec, step, 'components')] += abs(e['delta'])
        elif t == 'resource_changed':
            rows[rnd][(sec, step, 'components')] += 1
        elif t == 'card_moved':
            rows[rnd][(sec, step, 'components')] += 1
            # 268's 20 s a new card face read: a card's first showing in the game (draft hands, threats placed,
            # the adversary's event and project cards), read by the table once. In the draft, by its holder.
            to = e['to']
            if (to.startswith(('cards.packet', 'threats.on', 'adversary.next', 'adversary.projects'))
                    and e['card'] not in faces):
                faces.add(e['card'])
                fstep = step if sec == 'Draft' else 'card faces read (first showing)'
                rows[rnd][(sec, fstep, 'faces')] += 1
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
                mid = k
                if k.startswith('ai.power_used.'):
                    s, i = (int(x) for x in k.split('.')[2:4])
                    g = groups[s][i] if i < len(groups[s]) else i
                    mid = ('power', s, g)
                # A tiered power's tiers share one marker: set once, cleared once.
                if (rnd, phase, mid, e['delta'] > 0) not in marked:
                    marked.add((rnd, phase, mid, e['delta'] > 0))
                    rows[rnd][(sec, step if phase == ACT else 'used markers cleared', 'components')] += 1
        elif t == 'asked' and phase == ACT and 'target' in e.get('prompt', '').lower() and actor is not None:
            rows[rnd][(sec, step, 'checks')] += 1
    played = max(rows) if rows else 1
    for s, b in enumerate(('builder', 'protector')):
        for r in range(1, played + 1):
            rows[r][('Action phase', f'{SEATS[s]}: weighing board uses', 'checks')] += offered.get(b, 0) / played
    return rows


def add_counters(rows, g, tracks=True):
    """Per-round counters from the experiment's games.jsonl line for the same game."""
    c = g['counters']
    cities = cities_in_play() if not tracks else 0
    for r in rows:
        tag = f'r{r:02d}'
        rows[r][('Adversary: lines and arrivals', 'sites, arrivals, botnets', 'checks')] += c.get(f'map.adversary_checks.{tag}', 0)
        rows[r][('Adversary: march', 'troops', 'checks')] += c.get(f'map.troops.{tag}', 0)
        sites = sum(c.get(f'map.{k}.{o}.{tag}', 0) for k in ('factories', 'stations') for o in ('adversary', 'neutral', 'player'))
        if not tracks:  # 278's model: every site read at production (280's tracks count the tracks and chips read)
            rows[r][('Production and hosting', 'sites read', 'checks')] += sites + cities
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


def overlap(c):
    """298: seconds the faster seat's rows overlap the slower's in a phase played at once (negative, one row a
    section). The draft's steps (tier choice, pick, keep) are each finished by both seats before the next."""
    out = collections.Counter()
    for sec in AT_ONCE:
        blocks = collections.defaultdict(collections.Counter)  # block -> seat -> seconds
        for (s, step, kind), v in c.items():
            if s != sec or kind == 'overlap':
                continue
            seat, _, rest = step.partition(': ')
            if seat not in SEATS:
                continue
            block = rest if sec == 'Draft' and rest in DRAFT_STEP.values() else '' if sec != 'Draft' else 'other'
            blocks[block][seat] += v * rate(sec, kind)
        cut = sum(sum(b.values()) - max(b.values()) for b in blocks.values() if b)
        if cut:
            out[(sec, 'overlap: seats at once, the slower sets the pace', 'overlap')] -= cut
    return out


def load(folder, level, tracks=True):
    info = boards()
    games = {}
    exp = {}
    jp = os.path.join(folder, 'exp', 'games.jsonl')
    if os.path.exists(jp):
        for line in open(jp, encoding='utf-8'):
            g = json.loads(line)
            if 'seed' in g:
                exp[(int(g['difficulty']), int(g['seed']))] = g
    hand = hand_model(level)
    for p in sorted(glob.glob(os.path.join(folder, f'd{level}-*.out'))):
        seed = int(re.search(r'-(\d+)\.out$', p).group(1))
        rows = read_trace(p, info, tracks)
        g = exp.get((level, seed))
        if g:
            add_counters(rows, g, tracks)
        for r in rows:
            if r:
                rows[r].update(hand)  # the modeled kinds, a round, before the seats' overlap is taken
                rows[r].update(overlap(rows[r]))
        games[seed] = rows
    return games


def per_round(games):
    """Mean a round (pooled over every game-round) of each (section, step, kind)."""
    tot = collections.Counter()
    n = 0
    for rows in games.values():
        for r, c in rows.items():
            if r == 0:
                continue
            n += 1
            tot.update(c)
    mean = collections.Counter({k: v / max(n, 1) for k, v in tot.items()})
    rounds = statistics.mean(len([r for r in rows if r]) for rows in games.values())
    return mean, rounds


def minutes(c):
    return secs_of(c) / 60


def table(mean, rounds, level):
    secs = collections.defaultdict(lambda: collections.defaultdict(collections.Counter))
    for (sec, step, kind), v in mean.items():
        secs[sec][step][kind] += v
    order = ['Round start', 'Sway', 'Adversary: event card', 'Adversary: threats dealt', 'Adversary: lines and arrivals',
             'Adversary: march', 'Adversary: next card', 'Adversary: compute and projects', 'Draft', 'Action phase',
             'Production and hosting', 'Income and upkeep', 'Bases', 'Threats executed', 'War', 'Cleanup']
    out = [f'### d{level}: a round, {rounds:.2f} rounds a game', '',
           '| Section | Step | Decisions | Components | Checks | Lookups | Faces | Modifiers | Minutes |',
           '|---|---|---|---|---|---|---|---|---|']
    total = collections.Counter()
    by_kind = collections.Counter()

    def m_of(sec, k):
        return sum(k[x] * rate(sec, x) for x in KINDS + ['overlap']) / 60

    for sec in order + sorted(set(secs) - set(order)):
        if sec not in secs:
            continue
        sub = collections.Counter()
        small = collections.Counter()
        steps = secs[sec]
        for step in sorted(steps, key=lambda s: -abs(m_of(sec, steps[s]))):
            k = steps[step]
            m = m_of(sec, k)
            sub.update(k)
            for x in KINDS + ['overlap']:
                by_kind[x] += k[x] * rate(sec, x) / 60
            if abs(m) < 0.05 and len(steps) > 1:
                small.update(k)  # shown as one "other steps" row
                continue
            out.append(f"| {sec} | {step} | " + ' | '.join(f'{k[x]:.1f}' for x in KINDS) + f' | {m:.2f} |')
        if small:
            out.append(f"| {sec} | other steps (each under 0.05) | " + ' | '.join(f'{small[x]:.1f}' for x in KINDS)
                       + f' | {m_of(sec, small):.2f} |')
        total.update(sub)
        out.append(f"| **{sec}** | **all** | " + ' | '.join(f'**{sub[x]:.1f}**' for x in KINDS)
                   + f' | **{m_of(sec, sub):.2f}** |')
    m = sum(by_kind.values())
    out.append('| **Round** | | ' + ' | '.join(f'**{total[x]:.1f}**' for x in KINDS) + f' | **{m:.1f}** |')
    out.append('| *minutes by kind* | | ' + ' | '.join(f'{by_kind[x]:.1f}' for x in KINDS)
               + f' | {m:.1f} (overlap {by_kind["overlap"]:.1f}) |')
    return '\n'.join(out), m


def extras(folder, level):
    """Counts the write-up quotes beside the table, a round (pooled over every game-round)."""
    n = 0
    c = collections.Counter()
    for p in sorted(glob.glob(os.path.join(folder, f'd{level}-*.out'))):
        phase = ''
        for line in open(p, encoding='utf-8'):
            if not line.startswith('{'):
                continue
            d = json.loads(line)
            e = d.get('event')
            if e and e['type'] == 'phase_started':
                phase = e['phase']
            elif e and e['type'] == 'round_started':
                n += e['round'] > 0
            elif e and e['type'] == 'card_moved' and phase == 'threats.deal' and e['from'].startswith('threats.deck'):
                c['turned'] += 1
            elif e and e['type'] == 'var_changed' and e['key'].startswith('ai.power_used.'):
                if int(e['key'].split('.')[3]) < NODE_POWERS:
                    c['node_markers'] += 1  # each set and each clear
            elif 'action' in d and phase == ACT:
                a = d['action']
                if a['type'] == 'power' and a['power'] < NODE_POWERS:
                    c['node_uses'] += 1
    jp = os.path.join(folder, 'exp', 'games.jsonl')
    arr = []
    if os.path.exists(jp):
        for line in open(jp, encoding='utf-8'):
            g = json.loads(line)
            if g.get('difficulty') == level:
                k = g['counters']
                arr.extend(k.get(f'map.arrival_checks.r{i:02d}', 0) for i in range(1, g['rounds'] + 1))
    n = max(n, 1)
    return (f"Counts a round: arrival checks {statistics.mean(arr) if arr else 0:.1f} (within the adversary's checks); "
            f"threat cards turned in dealing {c['turned'] / n:.1f}; node powers used {c['node_uses'] / n:.1f} "
            f"(both seats), their used markers set and cleared {c['node_markers'] / n:.1f} "
            f"({c['node_markers'] / n * RATE['components'] / 60:.2f} min at 3 s).")


def seat_split(mean):
    """Minutes in each seat's own rows (its draft and action phase), the overlap, and the rest."""
    seat = collections.Counter()
    for (sec, step, kind), v in mean.items():
        who = next((s for s in SEATS if step.startswith(s + ':')), None)
        seat['overlap' if kind == 'overlap' else who or 'outside'] += v * rate(sec, kind) / 60
    return seat


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--level', type=int, default=2)
    ap.add_argument('--compare', action='store_true')
    ap.add_argument('--json')
    ap.add_argument('--sites', action='store_true', help="278's production model: every site read")
    a = ap.parse_args()
    tracks = not a.sites
    if a.compare:
        gb, gn = load(a.dirs[0], a.level, tracks), load(a.dirs[1], a.level, tracks)
        base, rb = per_round(gb)
        new, rn = per_round(gn)
        # Paired by seed: each game's minutes a round, the flag's side less base's.
        diffs = [minutes(per_round({s: gn[s]})[0]) - minutes(per_round({s: gb[s]})[0]) for s in gb if s in gn]
        se = statistics.stdev(diffs) / len(diffs) ** 0.5 if len(diffs) > 1 else 0
        bysec = collections.Counter()
        bykind = collections.Counter()
        for k in set(base) | set(new):
            bysec[k[0]] += (new[k] - base[k]) * rate(k[0], k[2]) / 60
            bykind[k[2]] += (new[k] - base[k]) * rate(k[0], k[2]) / 60
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
    mean, rounds = per_round(load(a.dirs[0], a.level, tracks))
    text, m = table(mean, rounds, a.level)
    print(text)
    print('\n' + extras(a.dirs[0], a.level))
    s = seat_split(mean)
    print(f"\nIn the seats' own rows (draft and action phase): Builder {s['Builder']:.1f}, Protector {s['Protector']:.1f}; "
          f"overlap {s['overlap']:.1f} (the faster seat's time in the phases played at once); outside them "
          f"{s['outside']:.1f}. Game: {m:.1f} x {rounds:.2f} = {m * rounds:.0f} minutes.")
    if a.json:
        with open(a.json, 'w', encoding='utf-8', newline='\n') as f:
            json.dump({'level': a.level, 'rounds': rounds, 'minutes': m,
                       'rows': [[k[0], k[1], k[2], v] for k, v in sorted(mean.items())]}, f, indent=1)


if __name__ == '__main__':
    main()

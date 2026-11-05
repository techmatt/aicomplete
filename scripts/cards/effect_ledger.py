"""The effect ledger (088): where card prices disagree with each other and with 082's
reasoned values. A diagnostic for picking the extreme outliers to discuss, not a
pricing formula.

    python scripts/cards/effect_ledger.py          # rewrites experiments/results/effect-ledger/
    python scripts/cards/effect_ledger.py --stdout [--cards PROPOSED.json] [--values VALUES.json]
                                                   # prints each card's fit and fair price; writes nothing

Re-run it at each card revision; it reads only content/cards.json,
configs/agents/values.json and RULES_REVISION, and writes the same files for the same
inputs (no clock, no randomness, sorted output).

What it does:

1. Decomposes each player card into effects: one row per op (its slot: play, passive
   or action; its parameters), plus the card's tags as one pseudo-effect. An effect's
   key is its op, its resource or token kind, and its slot. Tier and `needs` are
   card-level factors. `choose_player` is a condition on the gift that follows it
   (082 values it 0), so it is no effect of its own.
2. Reasoned value of each effect: a Python mirror of `crates/aic-sim/src/agents/value.rs`
   (`op_worth`, `ability_worth`, `CardValue::total` at `Ctx::typical_for` the card's tier:
   income and Actions over `horizon_t1` or `horizon_t2` rounds, r35). An action's
   value goes to the ops of its best branch, net of the branch's cost, pro rata; the
   other branches (and a cycle card's add branch) carry 0. The mirror is checked
   against nothing in Rust: if value.rs changes, change `op_worth` here too.
3. Joint fit: printed cost (credits + other resources at the table's rates, `lowers`
   over the tier's horizon) = sum over effects of beta_e x reasoned value + a per-tier
   constant + a credit rate per needed tag. beta_e is one multiplier per effect key, so
   the fit reads what an effect is priced at, relative to its reasoned value, across
   all the cards carrying it; fitted value per unit = beta_e x reasoned per unit. An
   effect on one or two cards is *unidentified*: it enters the fit at its reasoned
   value (beta fixed at 1) and gets no fitted value. Tags enter the same way (fixed).
4. Per instance: each card's printed cost is split over its effects in proportion to
   their reasoned values; per unit is that share over the effect's quantity (amount,
   count, max, rate; 1/per for a per-tag or per-token op; else 1).

Outputs (experiments/results/effect-ledger/): effects.csv, instances.csv, cards.csv,
factors.csv, ledger.html (self-contained, sortable, opens from disk), outliers.md.
"""
import argparse
import csv
import io
import json
import math
import os
import re
import statistics
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
OUT = os.path.join(ROOT, 'experiments', 'results', 'effect-ledger')
TAGS = ['science', 'industry', 'cyber', 'power', 'diplomacy', 'security']

# Outlier thresholds (outliers.md). Diagnostic choices, not game numbers.
SPREAD_RATIO = 2.5      # an effect whose per-unit max / min exceeds this
CARD_GAP_SD = 2.0       # a card whose gap is this many SDs from the mean gap
BETA_FAR = (0.5, 2.0)   # a fitted multiplier outside this band, on 4+ cards
TOP_N = 8               # at most this many rows per outliers.md list


def load_json(*parts):
    with open(os.path.join(ROOT, *parts), encoding='utf-8') as f:
        return json.load(f)


def revision():
    with open(os.path.join(ROOT, 'crates', 'aic-core', 'src', 'lib.rs'), encoding='utf-8') as f:
        return int(re.search(r'RULES_REVISION: u32 = (\d+)', f.read())[1])


# ---------- the mirror of value.rs ----------

class Worth:
    """value.rs `Worth`: per_tag and income_per_tag as dicts by tag."""
    __slots__ = ('now', 'node', 'per_node', 'per_tag', 'city', 'income', 'standing', 'income_per_tag')

    def __init__(self, **kw):
        self.now = self.node = self.per_node = self.city = self.income = self.standing = 0.0
        self.per_tag = {}
        self.income_per_tag = {}
        for k, v in kw.items():
            setattr(self, k, v)

    def scaled(self, k):
        w = Worth(now=self.now * k, node=self.node * k, per_node=self.per_node * k, city=self.city * k,
                  income=self.income * k, standing=self.standing * k)
        w.per_tag = {t: v * k for t, v in self.per_tag.items()}
        w.income_per_tag = {t: v * k for t, v in self.income_per_tag.items()}
        return w

    def add(self, o):
        for a in ('now', 'node', 'per_node', 'city', 'income', 'standing'):
            setattr(self, a, getattr(self, a) + getattr(o, a))
        for t, v in o.per_tag.items():
            self.per_tag[t] = self.per_tag.get(t, 0.0) + v
        for t, v in o.income_per_tag.items():
            self.income_per_tag[t] = self.income_per_tag.get(t, 0.0) + v


class Table:
    def __init__(self, raw):
        self.raw = raw
        self.left = self.rate('income_rounds')
        self.horizon = self.left
        self.node_count = self.rate('node_count')
        self.tag_count = self.rate('tag_count')

    def rate(self, k):
        return self.raw['rates'][k]

    def op(self, k):
        return self.raw['ops'][k]['credits']

    def res(self, r):
        return self.raw['resources'][r]

    def feed(self, kind):
        return self.raw['tokens']['feed_' + kind]

    def tags_value(self, card):
        if card['kind'] == 'event' or not card['tags']:
            return 0.0
        return self.raw['tags']['first'] + self.raw['tags']['more'] * (len(card['tags']) - 1)

    def at_tier(self, tier):
        """Ctx::typical_for (r35): income and Actions counted over the tier's horizon."""
        self.horizon = self.rate('horizon_t2') if tier >= 2 else self.rate('horizon_t1')

    def cost(self, card):
        """Table::cost at the printed price, lowered income over the card's horizon."""
        self.at_tier(card['tier'])
        v = card.get('cost', 0) + self.res('insight') * card.get('insight', 0)
        for r in ('energy', 'chips', 'antimatter', 'research_cubes'):
            v += self.res(r) * card.get(r, 0)
        for r, n in card.get('lowers', []):
            v += self.res(r) * n * max(self.horizon, 1.0)
        return v

    # Ctx::typical: left = income_rounds, node = node_count, every tag at tag_count,
    # a city corrupted.
    def instant(self, w):
        node = self.op('place_node') * (self.rate('node_late') if self.left <= self.rate('late_rounds') else self.rate('node'))
        tags = sum(v * self.tag_count for v in w.per_tag.values())
        return w.now + w.node * node + w.per_node * self.node_count + w.city + tags

    def per_round(self, w):
        return w.income + w.standing + sum(v * self.tag_count for v in w.income_per_tag.values())

    def whole(self, w):
        """What a played op is worth: now plus a round's worth each round of its horizon."""
        return self.instant(w) + self.horizon * self.per_round(w)

    def use_value(self, w):
        """One use of an action: instant plus a round (as CardValue::total reads use_)."""
        return self.instant(w) + self.per_round(w)


def income_r(t, resource):
    """value.rs `income_rate` (093): energy income at `rates.energy_income_share` of its rate."""
    return t.res(resource) * (t.rate('energy_income_share') if resource == 'energy' else 1.0)


# "Remove up to N troops" at what it removes in play, not N (093): troops removed per
# resolution of `remove_troops` with that N, measured in 093's r36 read
# (experiments/results/093_rules_r36/, 1,008 games). Since 102 (D26) the rows live in
# values.json's `troops_removed`, which value.rs reads too: one table for both. An N not
# there counts as N.
TROOPS_REMOVED = {}


def expected_troops(n):
    return TROOPS_REMOVED.get(n, n)


def damage(t, o):
    return max(t.op('remove_ambassador') * o.get('ambassadors', 0), t.op('remove_troop') * o.get('troops', 0))


def op_worth(t, o):
    """value.rs `op_worth`, for the ops player cards carry. An op with no arm here raises."""
    op = o['op']
    row = t.op
    r = t.res
    n = lambda k: o.get(k, 0)
    if op == 'gain':
        return Worth(now=row('gain') * r(o['resource']) * n('amount'))
    if op == 'gain_per_node':
        return Worth(per_node=row('gain_per_node') * r(o['resource']))
    if op == 'choose_player':
        return Worth(now=row('choose_player'))
    if op == 'other_gain':
        return Worth(now=row('other_gain') * r(o['resource']) * n('amount'))
    if op == 'transfer':
        return Worth(now=row('transfer') * r(o['resource']) * n('max'))
    if op == 'place_node':
        return Worth(node=n('count'), now=t.rate('node_free') * n('count') if o.get('free') else 0.0)
    if op == 'clear_markers':
        return Worth(now=row('clear_markers') * t.rate('target_share') * n('count'))
    if op == 'shield_node':
        return Worth(now=row('shield_node') * t.rate('target_share') * n('count'))
    if op == 'delay_threat':
        return Worth(now=row('delay_threat') * n('count'))
    if op == 'damage':
        return Worth(now=row('damage') * damage(t, o))
    if op == 'damage_country':
        return Worth(now=row('damage_country') * damage(t, o))
    if op in ('destroy_all', 'remove_botnet', 'build_factory', 'free_building', 'build_station',
              'counter_threat', 'discard_project', 'spread_ambassadors', 'move_one'):
        return Worth(now=row(op))
    if op == 'remove_ambassadors_country':
        m = t.rate('token_pile') if n('max') == 0 else n('max')
        sway = math.floor(m / n('unity_per')) * t.rate('sway') if n('unity_per') > 0 else 0.0
        return Worth(now=row(op) * m + sway)
    if op == 'remove_troops':
        return Worth(now=row(op) * expected_troops(n('count')))
    if op == 'cleanse':
        share = 0.8 if o.get('one_country') else 1.0
        if o.get('cities'):
            return Worth(city=row('liberate') * share * n('amount'))
        each = row('cleanse') if o.get('only') == 'factory' else row('cleanse_station')
        return Worth(now=each * share * n('amount'))
    if op == 'liberate':
        return Worth(city=row('liberate') + (row('liberate_align') if o.get('align') else 0.0)
                     + row('gain_ambassadors') * r('ambassadors') * n('ambassadors')
                     + row('liberate_troops') * n('troops')
                     + (row('liberate_botnet') if o.get('botnet') else 0.0))
    if op == 'clear_city':
        m = t.rate('token_pile') if n('max') == 0 else n('max')
        return Worth(now=row(op) * m)
    if op in ('harden', 'peek'):
        return Worth(now=row(op) * n('count'))
    if op == 'move_ambassadors' or op == 'consolidate':
        return Worth(now=row(op) * n('max'))
    if op == 'gain_ambassadors':
        return Worth(now=row(op) * r('ambassadors') * n('count'))
    if op == 'shelter':
        return Worth(now=row('shelter') * t.rate('target_share'))
    if op == 'cordon':
        return Worth(now=row('cordon'))
    if op == 'project_cube':
        return Worth(now=row(op) * n('count'))
    if op == 'jam':
        typed = row('typed_jam') if o.get('source') else 1.0
        return Worth(now=row('jam') * typed * n('amount'))
    if op == 'drain_project' or op == 'drain_all':
        return Worth(now=row(op) * n('amount'))
    if op == 'production':
        return Worth(income=row(op) * income_r(t, o['resource']) * n('amount'))
    if op == 'production_per_tag':
        return Worth(income_per_tag={o['tag']: row(op) * income_r(t, o['resource']) / max(n('per'), 1)})
    if op == 'add_tokens':
        return Worth(now=row(op) * t.feed(o['kind']) * n('amount'))
    if op == 'cut_botnet':
        return Worth(now=row(op))
    if op == 'draw':
        return Worth(now=row('draw') * t.rate('drawn_card') * n('count'))
    if op == 'gain_per_tokens':
        return Worth(now=row(op) * r(o['resource']) * math.floor(t.rate('token_pile') / max(n('per'), 1)))
    if op == 'cash_out':
        return Worth(now=row(op) * r(o['resource']) * n('rate') * t.rate('token_pile'))
    if op == 'gain_per_tag':
        return Worth(per_tag={o['tag']: row(op) * r(o['resource']) / max(n('per'), 1)})
    if op == 'sway':
        sign = 1.0 if o['side'] == 'unity' else -1.0
        return Worth(now=row('sway') * sign * t.rate('sway') * n('amount'))
    if op == 'team_income':
        return Worth(income=row(op) * r(o['resource']) * n('amount'))
    if op == 'income':
        return Worth(income=row(op) * income_r(t, o['resource']) * n('amount'))
    if op == 'free_purchases':
        return Worth(now=row(op) * t.rate('keep_tier1') * n('count'))
    if op == 'counter_all':
        return Worth(now=row(op))
    if op == 'pushback':
        return Worth(now=row(op) * t.rate('countries') * n('ambassadors'))
    if op == 'remove_botnets':
        return Worth(now=row(op) * n('count'))
    if op == 'income_per_tag':
        amount = o.get('amount', 1)
        return Worth(income_per_tag={o['tag']: row(op) * income_r(t, o['resource']) * amount / max(n('per'), 1)})
    if op in ('building_discount', 'slots', 'project_drain', 'unity_bonus', 'supply_insight'):
        return Worth(standing=row(op) * n('amount'))
    if op == 'auto_counter':
        return Worth(standing=row(op) * n('amount'))
    if op == 'harden_action':
        return Worth(standing=max(row(op) - 0.5 * r('chips') * n('chips') / 2.0, 0.0))
    if op == 'logistics':
        return Worth(standing=row(op) * r(o['resource']) * n('amount'))
    if op in ('spread_shield', 'civic_shelter', 'tags_from_hand'):
        return Worth(standing=row(op))
    if op == 'convert_upkeep':
        return Worth(standing=max(row(op) * (r(o['to']) * n('gain') - r(o['from']) * n('amount')), 0.0))
    # 105: the engine wording, as value.rs prices it.
    if op == 'card_discount':
        # 107: insight off every card x insight_discount_share (the floor's share).
        share = t.rate('tag_share') if o.get('tag') else (
            t.rate('insight_discount_share') if o['resource'] == 'insight' else 1.0)
        return Worth(standing=row(op) * t.rate('fires_play') * share * r(o['resource']) * n('amount'))
    if op == 'when':
        on = o['on']
        both = 1.0 + row('other_gain') if on['event'] == 'team_play' and on.get('both') else 1.0
        return Worth(standing=row(op) * fires(t, on) * triggered(t, o['then']) * both)
    if op == 'threshold':
        return Worth(standing=row(op) * t.rate('threshold_fill') / max(n('at'), 1) * triggered(t, o['then']))
    if op == 'copy_income':
        return Worth(income=row(op))
    if op == 'reuse':
        return Worth(now=row(op))
    if op == 'discard_hand':
        return Worth(now=row(op) * t.rate('hand_discards') * (r(o['resource']) * n('amount') - t.rate('dead_card')))
    raise KeyError(f'effect_ledger.py has no arm for op {op!r}: mirror value.rs op_worth')


def fires(t, on):
    """value.rs `fires` (105): how often a trigger fires a round."""
    ev = on['event']
    tagged = t.rate('tag_share') if on.get('tag') else 1.0
    if ev == 'play':
        return t.rate('fires_play') * tagged
    if ev == 'play_tag':
        return t.rate('fires_play') * (t.rate('tag_share') if on.get('tag') else t.rate('tags_per_card'))
    if ev == 'team_play':
        return t.rate('fires_play') * tagged * max(t.rate('countries') - 1.0, 1.0)
    if ev == 'play_cost':
        return t.rate('fires_play') * min(max(1.0 - on['at'] / t.rate('rebate_span'), 0.05), 1.0)
    return t.rate({'remove_troop': 'fires_troop', 'remove_compute': 'fires_compute',
                   'remove_ambassadors': 'fires_ambassadors', 'place_tokens': 'fires_tokens',
                   'wall_stops': 'fires_wall'}[ev])


def triggered(t, then):
    """value.rs `triggered` (105): one firing, tokens added here aside (a threshold's)."""
    w = Worth()
    for o in then:
        if o['op'] == 'add_tokens' and o.get('here'):
            continue
        w.add(op_worth(t, o))
    # 107: an income a firing raises counts rates.raised_rounds rounds.
    return w.now + w.node * t.op('place_node') * t.rate('node') + w.income * t.rate('raised_rounds')


def passive_worths(t, ops):
    """value.rs `passives_worth` (107) at the table's context: a threshold whose tokens the card's own
    trigger adds fills at that trigger's rate (rates.own_fill), each passive's Worth in order."""
    own = 0.0
    for o in ops:
        if o['op'] == 'when':
            added = sum(e['amount'] for e in o['then'] if e['op'] == 'add_tokens' and e.get('here'))
            own += fires(t, o['on']) * added
    out = []
    for o in ops:
        if o['op'] == 'threshold' and own > 0 and t.rate('own_fill') > 0:
            out.append(Worth(standing=t.op('threshold') / max(o['at'], 1) * own * triggered(t, o['then'])))
        else:
            out.append(op_worth(t, o))
    return out


def branches(ability):
    out = []
    a = ability
    while a:
        out.append(a)
        a = a.get('or')
    return out


def ability_parts(t, ability):
    """value.rs `ability_worth`, keeping each op's part: [(branch index, op, use value)]
    for the best branch's ops, net of its cost; the other branches' ops at 0."""
    bs = branches(ability)
    add_here = max([e.get('amount', 0) for b in bs for e in b.get('effects', [])
                    if e['op'] == 'add_tokens' and e.get('here')] or [0])
    best = None
    for i, b in enumerate(bs):
        effs = b.get('effects', [])
        adds = all(e['op'] == 'add_tokens' and e.get('here') for e in effs)
        if adds and add_here > 0 and len(bs) > 1:
            continue
        ws = [op_worth(t, e) for e in effs]
        w = Worth()
        for x in ws:
            w.add(x)
        cost = sum(t.res(res) * k for res, k in b.get('cost', []))
        w.now -= cost
        scale = 1.0
        counts_here = any(e['op'] in ('cash_out', 'gain_per_tokens') and e.get('here') for e in effs)
        if add_here > 0 and (b.get('tokens', 0) > 0 or counts_here):
            need = b['tokens'] if b.get('tokens', 0) > 0 else t.rate('token_pile')
            scale = 1.0 / (math.ceil(need / add_here) + 1.0)
            w = w.scaled(scale)
            ws = [x.scaled(scale) for x in ws]
        key = w.now + w.city + w.node * 3.0
        if best is None or key > best[0]:
            best = (key, i, w, ws)
    parts = []
    if best is None:
        return [(i, e, 0.0) for i, b in enumerate(bs) for e in b.get('effects', [])], 0.0
    _, bi, w, ws = best
    net = max(t.use_value(w), 0.0)
    gross = [t.use_value(x) for x in ws]
    gsum = sum(gross)
    for i, b in enumerate(bs):
        for j, e in enumerate(b.get('effects', [])):
            if i == bi and gsum > 0:
                parts.append((i, e, net * gross[j] / gsum))
            else:
                parts.append((i, e, 0.0))
    return parts, net


# ---------- decomposition ----------

QTY_KEYS = ('amount', 'count', 'max', 'rate')
CAT_KEYS = ('resource', 'kind')
DESCRIBE_SKIP = ('op',)


def quantity(o):
    if o['op'] in ('gain_per_tag', 'income_per_tag', 'gain_per_tokens', 'production_per_tag'):
        return o.get('amount', 1) / max(o.get('per', 1), 1) if o['op'] == 'income_per_tag' else 1.0 / max(o.get('per', 1), 1)
    if o['op'] == 'damage':
        return max(o.get('ambassadors', 0), o.get('troops', 0)) or 1
    for k in QTY_KEYS:
        if o.get(k):
            return float(o[k])
    return 1.0


def effect_key(o, slot):
    cat = [str(o[k]) for k in CAT_KEYS if o.get(k)]
    if o['op'] == 'sway':
        cat.append(o['side'])
    if o['op'] == 'damage':
        cat.append('troops' if o.get('troops') else 'ambassadors')
    if o['op'] == 'jam':
        cat.append('typed' if o.get('source') else 'any')
    if o['op'] == 'liberate':
        for k in ('align', 'ambassadors', 'troops', 'botnet'):
            if o.get(k):
                cat.append(k)
    if o['op'] == 'cleanse' and o.get('only'):
        cat.append(o['only'])
    if o['op'] == 'when':
        cat.append(' '.join(str(v) for v in o['on'].values()))
    if o['op'] == 'place_node' and o.get('free'):
        cat.append('free')
    if o['op'] == 'cleanse' and o.get('cities'):
        cat.append('cities')
    return ' '.join([o['op']] + cat) + f' [{slot}]'


def describe(o):
    return ', '.join(f'{k}={v}' for k, v in o.items() if k not in DESCRIBE_SKIP)


def needs_total(card):
    return sum(n for _, n in card.get('needs', []))


def needs_text(card):
    return ' '.join(f'{t} {n}' for t, n in card.get('needs', []))


def decompose(t, card):
    """Each effect instance of one card: dicts with key, slot, quantity, reasoned."""
    t.at_tier(card['tier'])
    rows = []

    def add_list(ops, slot, values=None):
        chosen = False
        for i, o in enumerate(ops):
            if o['op'] == 'choose_player':
                chosen = True
                continue
            v = t.whole(op_worth(t, o)) if values is None else values[i]
            rows.append({'key': effect_key(o, slot), 'op': o['op'], 'slot': slot,
                         'params': describe(o) + ('; to a chosen player' if chosen else ''),
                         'quantity': quantity(o), 'reasoned': v})

    # value.rs `CardValue::of` (r37): a one-shot Action is used once, and the tokens the
    # card arrives with are its fuel, counted in that use.
    ab = card.get('ability')
    once = bool(ab and ab.get('once') and not ab.get('or'))
    play = card.get('effects', [])
    if once:
        play = [o for o in play if not (o['op'] == 'add_tokens' and o.get('here'))]
    add_list(play, 'play')
    passives = card.get('passives', [])
    add_list(passives, 'passive', values=[t.whole(w) for w in passive_worths(t, passives)])
    if card.get('ability'):
        parts, _ = ability_parts(t, card['ability'])
        uses = 1.0 if once else t.horizon * t.rate('uses_share')
        bs = branches(card['ability'])
        for bi, b in enumerate(bs):
            ops = b.get('effects', [])
            vals = [uses * v for (i, _, v) in parts if i == bi]
            live = any(v > 0 for v in vals)
            slot = 'action' if live or len(bs) == 1 else 'action, unused branch'
            cond = []
            if b.get('cost'):
                cond.append('pays ' + ' '.join(f'{k} {r}' for r, k in b['cost']))
            if b.get('tokens'):
                cond.append(f"spends {b['tokens']} tokens")
            before = len(rows)
            add_list(ops, slot, values=vals)
            for r_ in rows[before:]:
                if cond:
                    r_['params'] += '; ' + '; '.join(cond)
    tv = t.tags_value(card)
    if tv:
        rows.append({'key': 'tags [card]', 'op': 'tags', 'slot': 'card',
                     'params': ' '.join(card['tags']), 'quantity': float(len(card['tags'])), 'reasoned': tv})
    extra = sum(o['credits'] for o in t.raw['overrides'] if o['card'] == card['id'])
    if extra:
        rows.append({'key': 'override [card]', 'op': 'override', 'slot': 'card', 'params': '',
                     'quantity': 1.0, 'reasoned': extra})
    return rows


# ---------- least squares (no numpy on this box) ----------

def solve(a, b):
    """Gaussian elimination with partial pivoting; a is square."""
    n = len(a)
    m = [row[:] + [b[i]] for i, row in enumerate(a)]
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(m[r][c]))
        if abs(m[p][c]) < 1e-12:
            raise ValueError('singular normal equations')
        m[c], m[p] = m[p], m[c]
        for r in range(n):
            if r != c and m[r][c]:
                f = m[r][c] / m[c][c]
                for k in range(c, n + 1):
                    m[r][k] -= f * m[c][k]
    return [m[i][n] / m[i][i] for i in range(n)]


def inverse(a):
    n = len(a)
    cols = [solve(a, [1.0 if i == j else 0.0 for i in range(n)]) for j in range(n)]
    return [[cols[j][i] for j in range(n)] for i in range(n)]


def fit(cards, inst, keys_fitted, t):
    """Printed = sum beta_e x reasoned_e (fitted keys) + fixed reasoned (the rest)
    + tier constants + rate x needed tags. Returns coefficients, SEs and fitted costs."""
    cols = list(keys_fitted) + ['tier 1', 'tier 2', 'per needed tag']
    idx = {c: i for i, c in enumerate(cols)}
    X, y = [], []
    for c in cards:
        row = [0.0] * len(cols)
        offset = 0.0
        for r_ in inst[c['id']]:
            if r_['key'] in idx:
                row[idx[r_['key']]] += r_['reasoned']
            else:
                offset += r_['reasoned']
        row[idx[f"tier {c['tier']}"]] = 1.0
        row[idx['per needed tag']] = needs_total(c)
        X.append(row)
        y.append(t.cost(c) - offset)
    p = len(cols)
    xtx = [[sum(X[k][i] * X[k][j] for k in range(len(X))) for j in range(p)] for i in range(p)]
    xty = [sum(X[k][i] * y[k] for k in range(len(X))) for i in range(p)]
    beta = solve(xtx, xty)
    resid = [y[k] - sum(X[k][i] * beta[i] for i in range(p)) for k in range(len(X))]
    dof = max(len(X) - p, 1)
    s2 = sum(e * e for e in resid) / dof
    inv = inverse(xtx)
    se = [math.sqrt(max(inv[i][i] * s2, 0.0)) for i in range(p)]
    fitted = {c['id']: t.cost(c) - resid[k] for k, c in enumerate(cards)}
    return {cols[i]: (beta[i], se[i]) for i in range(p)}, fitted, math.sqrt(s2), dof


# ---------- outputs ----------

def r2(x):
    return None if x is None else round(x, 2)


def write_csv(name, header, rows):
    buf = io.StringIO(newline='')
    w = csv.writer(buf, lineterminator='\n')
    w.writerow(header)
    for r_ in rows:
        w.writerow(['' if v is None else v for v in r_])
    with open(os.path.join(OUT, name), 'w', encoding='utf-8', newline='\n') as f:
        f.write(buf.getvalue())


def load_path(path):
    with open(path, encoding='utf-8') as f:
        return json.load(f)


def main(stdout=False, cards_path=None, values_path=None, hold_ratio=None):
    raw = load_path(values_path) if values_path else load_json('configs', 'agents', 'values.json')
    t = Table(raw)
    TROOPS_REMOVED.clear()
    TROOPS_REMOVED.update({int(k): v for k, v in raw.get('troops_removed', {}).items() if k != 'note'})
    cards = sorted(load_path(cards_path) if cards_path else load_json('content', 'cards.json'), key=lambda c: c['id'])
    if not stdout:
        os.makedirs(OUT, exist_ok=True)
    out = (lambda *_: None) if stdout else write_csv
    rev = revision()

    inst = {c['id']: decompose(t, c) for c in cards}
    by_id = {c['id']: c for c in cards}
    printed = {c['id']: t.cost(c) for c in cards}
    reasoned = {cid: sum(r_['reasoned'] for r_ in rows) for cid, rows in inst.items()}

    # Per instance: the printed cost split pro rata to reasoned values.
    for cid, rows in inst.items():
        tot = reasoned[cid]
        for r_ in rows:
            r_['share'] = printed[cid] * r_['reasoned'] / tot if tot > 0 else None
            r_['per_unit'] = r_['share'] / r_['quantity'] if r_['share'] is not None else None
            r_['reasoned_unit'] = r_['reasoned'] / r_['quantity']

    # Effects: the cards carrying each key with a nonzero reasoned value.
    effects = {}
    for cid, rows in inst.items():
        for r_ in rows:
            e = effects.setdefault(r_['key'], {'key': r_['key'], 'op': r_['op'], 'slot': r_['slot'], 'rows': []})
            e['rows'].append((cid, r_))
    for e in effects.values():
        live = [(cid, r_) for cid, r_ in e['rows'] if r_['reasoned'] != 0 and r_['per_unit'] is not None]
        e['live'] = live
        e['cards'] = sorted({cid for cid, _ in e['rows']})
        e['n'] = len({cid for cid, _ in live})
        if e['slot'] == 'card':
            e['status'] = 'fixed'
        elif not live:
            e['status'] = 'no value'
        elif e['n'] <= 2:
            e['status'] = 'unidentified'
        else:
            e['status'] = 'fitted'
    fitted_keys = sorted(k for k, e in effects.items() if e['status'] == 'fitted')
    coef, fitted_cost, sigma, dof = fit(cards, inst, fitted_keys, t)

    eff_rows = []
    for k in sorted(effects):
        e = effects[k]
        live = e['live']
        ru = statistics.median(r_['reasoned_unit'] for _, r_ in live) if live else None
        lo = min(live, key=lambda x: (x[1]['per_unit'], x[0])) if live else None
        hi = max(live, key=lambda x: (x[1]['per_unit'], x[0])) if live else None
        beta, se = coef.get(k, (None, None))
        fv = beta * ru if beta is not None and ru is not None else None
        spread = hi[1]['per_unit'] - lo[1]['per_unit'] if live else None
        ratio = hi[1]['per_unit'] / lo[1]['per_unit'] if live and lo[1]['per_unit'] > 0 else None
        e['out'] = {
            'effect': k, 'slot': e['slot'], 'status': e['status'], 'cards': e['n'],
            'fitted': r2(fv), 'beta': r2(beta), 'beta_se': r2(se), 'reasoned': r2(ru),
            'min': r2(lo[1]['per_unit']) if live else None, 'min_card': lo[0] if live else '',
            'max': r2(hi[1]['per_unit']) if live else None, 'max_card': hi[0] if live else '',
            'spread': r2(spread), 'max_over_min': r2(ratio),
            'all_cards': ' '.join(e['cards']),
        }
        eff_rows.append(e['out'])
    eff_cols = ['effect', 'slot', 'status', 'cards', 'fitted', 'beta', 'beta_se', 'reasoned', 'min', 'min_card',
                'max', 'max_card', 'spread', 'max_over_min', 'all_cards']
    out('effects.csv', eff_cols, [[r_[c] for c in eff_cols] for r_ in eff_rows])

    inst_cols = ['card', 'effect', 'slot', 'params', 'quantity', 'reasoned', 'reasoned_per_unit', 'printed_share',
                 'per_unit']
    inst_rows = []
    for cid in sorted(inst):
        for r_ in inst[cid]:
            inst_rows.append({'card': cid, 'effect': r_['key'], 'slot': r_['slot'], 'params': r_['params'],
                              'quantity': r2(r_['quantity']), 'reasoned': r2(r_['reasoned']),
                              'reasoned_per_unit': r2(r_['reasoned_unit']), 'printed_share': r2(r_['share']),
                              'per_unit': r2(r_['per_unit'])})
    out('instances.csv', inst_cols, [[r_[c] for c in inst_cols] for r_ in inst_rows])

    card_rows = []
    for c in cards:
        cid = c['id']
        gap = printed[cid] - reasoned[cid]
        card_rows.append({
            'card': cid, 'name': c['name'], 'tier': c['tier'], 'kind': c['kind'], 'needs': needs_text(c),
            'cost': c.get('cost', 0), 'insight': c.get('insight', 0), 'printed': r2(printed[cid]),
            'reasoned': r2(reasoned[cid]), 'gap': r2(gap),
            'ratio': r2(printed[cid] / reasoned[cid]) if reasoned[cid] > 0 else None,
            'fit': r2(fitted_cost[cid]), 'fit_resid': r2(printed[cid] - fitted_cost[cid]),
            'effects': ' + '.join(f"{r_['key']} {r_['reasoned']:.1f}" for r_ in inst[cid]),
        })
    card_rows.sort(key=lambda r_: (-abs(r_['gap']), r_['card']))
    card_cols = ['card', 'name', 'tier', 'kind', 'needs', 'cost', 'insight', 'printed', 'reasoned', 'gap', 'ratio',
                 'fit', 'fit_resid', 'effects']
    out('cards.csv', card_cols, [[r_[c] for c in card_cols] for r_ in card_rows])

    fac_cols = ['factor', 'value', 'se']
    fac_rows = [[k, r2(coef[k][0]), r2(coef[k][1])] for k in ('tier 1', 'tier 2', 'per needed tag')]
    fac_rows.append(['residual sd', r2(sigma), ''])
    fac_rows.append(['degrees of freedom', dof, ''])
    out('factors.csv', fac_cols, fac_rows)

    if stdout:
        # The fair credit price (docs/card-pricing.md, *A fair price*): the fit's printed
        # value less what the card costs in other resources at the table's rates.
        # --hold-ratio: the credit price that keeps each card's printed / reasoned ratio of an
        # older cards.csv after a value change (098 repriced 12 cards so, by hand).
        old_ratio = {}
        if hold_ratio:
            with open(hold_ratio, encoding='utf-8', newline='') as f:
                old_ratio = {r_['card']: float(r_['ratio']) for r_ in csv.DictReader(f) if r_['ratio']}
        w = csv.writer(sys.stdout, lineterminator='\n')
        w.writerow(['card', 'name', 'tier', 'cost', 'printed', 'reasoned', 'fit', 'fair_credits']
                   + (['held_credits'] if hold_ratio else []))
        for r_ in sorted(card_rows, key=lambda r_: r_['card']):
            other = printed[r_['card']] - r_['cost']
            held = old_ratio.get(r_['card'])
            w.writerow([r_['card'], r_['name'], r_['tier'], r_['cost'], r_['printed'], r_['reasoned'], r_['fit'],
                        r2(fitted_cost[r_['card']] - other)]
                       + ([r2(held * reasoned[r_['card']] - other) if held is not None else ''] if hold_ratio else []))
        print()
        for row in fac_rows:
            print(f"{row[0]}: {row[1]}" + (f" (se {row[2]})" if row[2] != '' else ''))
        return

    meta ={'revision': rev, 'cards': len(cards), 'effects': len(effects), 'fitted': len(fitted_keys),
            'unidentified': sum(1 for e in effects.values() if e['status'] == 'unidentified'),
            'sigma': r2(sigma), 'dof': dof, 'factors': {k: [r2(v[0]), r2(v[1])] for k, v in coef.items()
                                                        if k in ('tier 1', 'tier 2', 'per needed tag')}}
    write_outliers(meta, effects, eff_rows, card_rows, inst)
    write_html(meta, eff_rows, eff_cols, card_rows, card_cols, inst_rows, inst_cols)
    print(f"r{rev}: {len(cards)} cards, {len(effects)} effects ({len(fitted_keys)} fitted, "
          f"{meta['unidentified']} unidentified); residual sd {sigma:.2f} cr on {dof} dof. Wrote {OUT}")


def write_outliers(meta, effects, eff_rows, card_rows, inst):
    L = [f"# Effect ledger: extreme outliers (r{meta['revision']})", '',
         'Generated by `python scripts/cards/effect_ledger.py`; the full tables are in `ledger.html` and the CSVs.',
         'Values are credits per unit of the effect (printed cost shared out pro rata to 082\'s reasoned values).', '']
    spread = [e for e in eff_rows if e['max_over_min'] and e['max_over_min'] > SPREAD_RATIO and e['cards'] >= 3
              and e['slot'] != 'card']
    spread.sort(key=lambda e: (-e['max_over_min'], e['effect']))
    L.append(f'## Effects priced most unevenly across cards (max / min per unit over {SPREAD_RATIO}, 3+ cards)')
    L.append('')
    for e in spread[:TOP_N]:
        L.append(f"- **{e['effect']}** ({e['cards']} cards, reasoned {e['reasoned']}): "
                 f"{e['min']} on `{e['min_card']}` to {e['max']} on `{e['max_card']}` (x{e['max_over_min']}).")
    if not spread:
        L.append('- none')
    L.append('')
    far = [e for e in eff_rows if e['status'] == 'fitted' and e['cards'] >= 4 and e['beta'] is not None
           and not (BETA_FAR[0] <= e['beta'] <= BETA_FAR[1])]
    far.sort(key=lambda e: (-abs(math.log(max(e['beta'], 0.01))), e['effect']))
    L.append(f'## Effects whose fitted price is far from the reasoned one (beta outside {BETA_FAR[0]}-{BETA_FAR[1]}, 4+ cards)')
    L.append('')
    for e in far[:TOP_N]:
        L.append(f"- **{e['effect']}** ({e['cards']} cards): fitted {e['fitted']} per unit vs reasoned "
                 f"{e['reasoned']} (beta {e['beta']} ± {e['beta_se']}).")
    if not far:
        L.append('- none')
    L.append('')
    gaps = [c['gap'] for c in card_rows]
    mu, sd = statistics.mean(gaps), statistics.pstdev(gaps)
    ext = [c for c in card_rows if abs(c['gap'] - mu) > CARD_GAP_SD * sd]
    ext.sort(key=lambda c: (-abs(c['gap'] - mu), c['card']))
    L.append(f'## Cards whose printed cost is furthest from their summed reasoned value '
             f'(gap over {CARD_GAP_SD} SD from the mean gap {mu:.1f} ± {sd:.1f})')
    L.append('')
    for c in ext[:TOP_N * 2]:
        L.append(f"- `{c['card']}` (tier {c['tier']} {c['kind']}{', needs ' + c['needs'] if c['needs'] else ''}): "
                 f"printed {c['printed']} vs reasoned {c['reasoned']} (gap {c['gap']:+}). {c['effects']}.")
    if not ext:
        L.append('- none')
    L.append('')
    f = meta['factors']
    L.append(f"Fit: {meta['fitted']} effects fitted, {meta['unidentified']} unidentified (on 1-2 cards, held at reasoned); "
             f"tier 1 constant {f['tier 1'][0]} ± {f['tier 1'][1]}, tier 2 {f['tier 2'][0]} ± {f['tier 2'][1]}, "
             f"{f['per needed tag'][0]} ± {f['per needed tag'][1]} cr per needed tag; residual sd {meta['sigma']} cr "
             f"on {meta['dof']} dof.")
    with open(os.path.join(OUT, 'outliers.md'), 'w', encoding='utf-8', newline='\n') as fh:
        fh.write('\n'.join(L) + '\n')


HTML_HEAD = """<!doctype html>
<html lang="en"><head><meta charset="utf-8"><title>Effect ledger r{rev}</title>
<style>
body {{ font: 13px/1.4 system-ui, sans-serif; margin: 16px; color: #222; }}
h1 {{ font-size: 18px; margin: 0 0 4px; }}
.note {{ color: #555; max-width: 70em; }}
nav button {{ font: inherit; padding: 4px 10px; margin-right: 4px; cursor: pointer; }}
nav button.on {{ background: #224; color: #fff; }}
input {{ font: inherit; padding: 3px 6px; width: 20em; margin-left: 8px; }}
table {{ border-collapse: collapse; margin-top: 8px; }}
th, td {{ border-bottom: 1px solid #ddd; padding: 3px 6px; text-align: left; vertical-align: top; }}
th {{ position: sticky; top: 0; background: #f2f2f6; cursor: pointer; white-space: nowrap; }}
th.asc::after {{ content: " \\25B2"; }} th.desc::after {{ content: " \\25BC"; }}
td.n {{ text-align: right; font-variant-numeric: tabular-nums; }}
td.wrap {{ max-width: 40em; color: #555; }}
tr.sub td {{ background: #fafaf4; }}
tr.click {{ cursor: pointer; }} tr.click:hover td {{ background: #eef; }}
.unidentified {{ color: #a60; }} .fixed, .novalue {{ color: #888; }}
</style></head><body>
<h1>Effect ledger, r{rev}</h1>
<p class="note">{cards} player cards, {effects} effects ({fitted} fitted, {unident} unidentified on 1-2 cards).
Per-unit values are credits per unit of the effect: the card's printed cost (credits plus other resources at 082's rates)
shared over its effects pro rata to 082's reasoned values. <b>fitted</b> = beta x reasoned per unit from the joint fit
(tier constants {t1} / {t2}, {pn} cr per needed tag, residual sd {sigma} cr). Click a header to sort; click an effect row for its cards.
A diagnostic, not a pricing formula. Generated by <code>python scripts/cards/effect_ledger.py</code>.</p>
<nav><button data-t="effects" class="on">Effects</button><button data-t="cards">Cards (printed vs reasoned)</button><button data-t="instances">Instances</button>
<input id="q" placeholder="filter rows"></nav>
<div id="view"></div>
<script>
const DATA = {data};
"""

HTML_JS = r"""
const NUM = new Set(['cards','fitted','beta','beta_se','reasoned','min','max','spread','max_over_min','tier','cost',
  'insight','printed','gap','ratio','fit','fit_resid','quantity','reasoned_per_unit','printed_share','per_unit']);
const HIDE = {effects: ['all_cards'], cards: [], instances: []};
let tab = 'effects', sortCol = {effects: 'spread', cards: null, instances: null}, sortDir = {effects: -1, cards: 1, instances: 1};
let open = new Set();
function cmp(a, b, c) {
  const x = a[c], y = b[c];
  if (x === null || x === '') return 1; if (y === null || y === '') return -1;
  return NUM.has(c) ? x - y : String(x).localeCompare(String(y));
}
function cell(c, v) {
  const td = document.createElement('td');
  td.textContent = v === null ? '' : v;
  if (NUM.has(c)) td.className = 'n';
  if (c === 'effects' || c === 'params') td.className = 'wrap';
  if (c === 'status') td.className = String(v).replace(' ', '');
  return td;
}
function render() {
  const t = DATA[tab], cols = t.cols.filter(c => !HIDE[tab].includes(c));
  const q = document.getElementById('q').value.toLowerCase();
  let rows = t.rows.filter(r => !q || JSON.stringify(r).toLowerCase().includes(q));
  const sc = sortCol[tab];
  if (sc) { rows = rows.slice().sort((a, b) => { const r = cmp(a, b, sc); return (a[sc] === null || b[sc] === null || a[sc] === '' || b[sc] === '') ? r : r * sortDir[tab]; }); }
  const table = document.createElement('table'), head = document.createElement('tr');
  for (const c of cols) {
    const th = document.createElement('th'); th.textContent = c.replace(/_/g, ' ');
    if (c === sc) th.className = sortDir[tab] > 0 ? 'asc' : 'desc';
    th.onclick = () => { if (sortCol[tab] === c) sortDir[tab] = -sortDir[tab]; else { sortCol[tab] = c; sortDir[tab] = NUM.has(c) ? -1 : 1; } render(); };
    head.appendChild(th);
  }
  table.appendChild(head);
  for (const r of rows) {
    const tr = document.createElement('tr');
    for (const c of cols) tr.appendChild(cell(c, r[c]));
    table.appendChild(tr);
    if (tab === 'effects') {
      tr.className = 'click';
      tr.onclick = () => { open.has(r.effect) ? open.delete(r.effect) : open.add(r.effect); render(); };
      if (open.has(r.effect)) {
        const subs = DATA.instances.rows.filter(i => i.effect === r.effect).sort((a, b) => (b.per_unit ?? -1e9) - (a.per_unit ?? -1e9));
        for (const i of subs) {
          const s = document.createElement('tr'); s.className = 'sub';
          const td = document.createElement('td'); td.colSpan = cols.length;
          td.textContent = `  ${i.card}: ${i.per_unit ?? '-'} per unit (share ${i.printed_share ?? '-'} of printed, reasoned ${i.reasoned}, qty ${i.quantity}) — ${i.params}`;
          s.appendChild(td); table.appendChild(s);
        }
      }
    }
  }
  const v = document.getElementById('view'); v.innerHTML = ''; v.appendChild(table);
}
for (const b of document.querySelectorAll('nav button')) b.onclick = () => {
  tab = b.dataset.t; for (const x of document.querySelectorAll('nav button')) x.classList.toggle('on', x === b); render();
};
document.getElementById('q').oninput = render;
render();
</script></body></html>
"""


def write_html(meta, eff_rows, eff_cols, card_rows, card_cols, inst_rows, inst_cols):
    data = {'effects': {'cols': eff_cols, 'rows': eff_rows}, 'cards': {'cols': card_cols, 'rows': card_rows},
            'instances': {'cols': inst_cols, 'rows': inst_rows}}
    blob = json.dumps(data, sort_keys=True, ensure_ascii=False).replace('</', '<\\/')
    f = meta['factors']
    head = HTML_HEAD.format(rev=meta['revision'], cards=meta['cards'], effects=meta['effects'], fitted=meta['fitted'],
                            unident=meta['unidentified'], t1=f['tier 1'][0], t2=f['tier 2'][0],
                            pn=f['per needed tag'][0], sigma=meta['sigma'], data=blob)
    with open(os.path.join(OUT, 'ledger.html'), 'w', encoding='utf-8', newline='\n') as fh:
        fh.write(head + HTML_JS)


def args():
    p = argparse.ArgumentParser(description=__doc__.split('\n\n')[0],
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument('--stdout', action='store_true',
                   help='print the cards table (with fair credits) and the factors; write nothing')
    p.add_argument('--cards', help='a cards.json to read instead of content/cards.json (proposed prices)')
    p.add_argument('--values', help='a values.json to read instead of configs/agents/values.json')
    p.add_argument('--hold-ratio', metavar='OLD_CARDS_CSV',
                   help='with --stdout: add each card\'s credit price at its printed / reasoned ratio in an older '
                        'cards.csv (git show REV:experiments/results/effect-ledger/cards.csv > out/old.csv)')
    a = p.parse_args()
    if a.hold_ratio and not a.stdout:
        p.error('--hold-ratio prints: pass --stdout')
    return a


if __name__ == '__main__':
    a = args()
    main(stdout=a.stdout, cards_path=a.cards, values_path=a.values, hold_ratio=a.hold_ratio)

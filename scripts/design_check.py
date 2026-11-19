"""205: design warnings that fire on their own. Reads content/ and configs/ (the tree), and a read's farmed games, and
prints every design warning it trips, each with its number. Warnings never stop work: no exit code but 0.

    python scripts/design_check.py [DIR[,DIR...]] [--old DIR[,DIR...]] [--base REV] [--config FILE] [--quiet] [--usage]
                                   [--variant V[,V...]] [--agent A[,A...]]

207: `usage` warns on every player action (standard actions, board powers, converters, card plays, building Actions,
threat counters, resource spends) whose use falls outside its expected band, both ways; `--usage` prints the inventory
table (role, band, use pooled, by board and by level) instead of the warnings. Bands: design_check.json's `usage`.

DIR is a read's folder (its games.jsonl: a farmed read assembled with its header, or an experiment's output); comma
lists pool; a farm's bare out file goes as FILE=SPEC. With no DIR only the content checks run. `--variant` (229)
keeps only the games of those variants on DIR's side (`--old` keeps every variant): a name, or a lever screen's lever
(`PAIR_LEVER` or `PAIR-LEVER` variants; `base` the bare pairs), so a screen's frozen cell reads without `bench/split_read.py`; `--agent` (281) likewise keeps only
those agents' games, each named as the spec writes it (`strategist:w.sink=1`), on both sides (286: `--old` too, so the
token and table-cost rise checks compare one agent mix). `--old` is the previous revision's read on the same seeds (the
tokens-a-round check reads against it). `--base` is the revision the item started from (default: the merge base with
main); the dials check diffs configs/baseline.json against it. Thresholds and source classes are in
scripts/design_check.json, all candidates. The principles are docs/principles.md, *Design warnings*.

The first line printed is the report header's: `warnings: design: <check> <number>; design: <check> <number>; ...` or `warnings: none`.
"""
import json
import os
import re
import subprocess
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from screens import PEAKS, cells_of, games_in, header, keeps_variant, source  # noqa: E402
from powers import power_slug  # noqa: E402

SHARE_RES = ['energy', 'chips', 'antimatter', 'research_cubes', 'ambassadors']


def jload(rel):
    with open(os.path.join(ROOT, rel), encoding='utf-8') as f:
        return json.load(f)


# ---- inputs -----------------------------------------------------------------------------------------------------

def agent_list(text):
    """`--agent`'s comma list, split between agents only: an agent's options hold commas too
    (`strategist:w.a=1,w.b=2`), as play_read's `--agents` reads them."""
    return {x for x in re.split(r',(?![a-z_]+\.[a-z_]+=)', text) if x}


def load_games(dirs, variants=None, agents=None):
    """The games of DIRS (comma list: a read folder, a games file, or FILE=SPEC for a farm's bare out file).
    VARIANTS (`--variant`): only the games of those variants (`screens.keeps_variant`); AGENTS (`--agent`, 281): only
    those agents', each as the spec writes it (`strategist:w.sink=1`); both read by cell from the header's spec."""
    flag = '--variant' if variants else '--agent'
    games = []
    for d in dirs.split(','):
        path, spec_path = source(d)
        if not os.path.isfile(path):
            sys.exit(f"{d.partition('=')[0]}: no games file ({path})")
        cells = None
        if spec_path:
            with open(spec_path, encoding='utf-8') as f:
                spec = json.load(f)
        else:  # a bare farm out file with no SPEC reads whole: only the filters need a spec
            spec = (header(path) or {}).get('spec')
        for g in games_in(path):
            if variants or agents:
                if spec is None:
                    sys.exit(f"{flag}: {d.partition('=')[0]} has no header; give it as FILE=SPEC")
                if cells is None:  # the cell order of crates/aic-sim/src/experiment.rs
                    cells = cells_of(spec)
                c = cells[g['cell']]
                if variants and not keeps_variant(c['variant'], variants):
                    continue
                if agents and c['agent'] not in agents:
                    continue
            games.append(g)
    if (variants or agents) and not games:
        asked = ' '.join(f'{f} {",".join(sorted(w))}' for f, w in (('--variant', variants), ('--agent', agents)) if w)
        sys.exit(f'{asked}: no game in {dirs}')
    return games


def flat_baseline(cfg):
    out = {}
    for k, v in cfg.items():
        if k == 'modules':
            for m in v:
                for p, x in (m.get('params') or {}).items():
                    out[f"{m['module']}.{p}"] = x
                if 'variant' in m:
                    out[f"{m['module']}.variant"] = m['variant']
        elif k != 'name':
            out[k] = v
    return out


def anchors():
    """Matt's unit anchors from docs/card-guidelines.md's table, then values.json for units it lacks."""
    a = dict(jload('configs/agents/values.json')['resources'])
    names = {'remove 1 agent': 'agent', 'remove 1 troop': 'troop', '1 energy': 'energy', '1 insight': 'insight',
             'draw 1 card': 'card', '1 chip': 'chips', 'remove 1 compute from a project': 'compute',
             '1 antimatter': 'antimatter', '1 ambassador': 'ambassadors'}
    with open(os.path.join(ROOT, 'docs/card-guidelines.md'), encoding='utf-8') as f:
        text = f.read()
    sec = text.split('**Unit anchors**', 1)[1].split('\n\n', 2)[1]
    for row in sec.splitlines():
        cells = [c.strip() for c in row.strip('|').split('|')]
        for i in range(0, len(cells) - 1, 3):
            key = names.get(cells[i].lower())
            m = re.match(r'([\d.]+)', cells[i + 1]) if key else None
            if m:
                a[key] = float(m.group(1))
    a['research_cubes'] = 4  # card-guidelines.md's derived line (values.json at the anchors too since 283)
    return a


# ---- the checks -------------------------------------------------------------------------------------------------

class Out:
    def __init__(self):
        self.warn = []      # (check, number, line)
        self.info = []

    def w(self, check, number, line):
        self.warn.append((check, number, line))

    def i(self, line):
        self.info.append(line)


def strip_side(src):
    s = src.split(':', 1)[1] if src.startswith(('own:', 'other:')) else src
    return 'antimatter-action' if s == 'antimatter' else s


def tally(games, key):
    """resource -> source -> per seat a game, pooled over the read (`sources` or `uses`)."""
    seats = sum(len(g['spent']) for g in games) or 1
    t = defaultdict(lambda: defaultdict(float))
    for g in games:
        for per_seat in g.get(key) or []:
            for src, row in per_seat.items():
                for i, r in enumerate(PEAKS):
                    if row[i]:
                        t[r][strip_side(src)] += row[i] / seats
    return t


def check_scarce(games, cfg, cards, o):
    sc = cfg['scarce_resources']
    std = [re.compile(p) for p in cfg['source_classes']['standard']]
    tier1 = {c['id'] for c in cards if c.get('tier') == 1}
    src = tally(games, 'sources')

    def cheap(s):
        if any(p.search(s) for p in std):
            return 'standard'
        m = re.match(r'(play|use):(.+)$', s)
        if m and m.group(2) in tier1:
            return 'tier 1'
        return None
    for r in sc['resources']:
        tot = sum(src[r].values())
        if not tot:
            o.i(f'scarce: {r} never gained in this read')
            continue
        parts = {s: v for s, v in src[r].items() if cheap(s)}
        share = sum(parts.values()) / tot
        detail = ', '.join(f'{s} {v / tot:.0%}' for s, v in sorted(parts.items(), key=lambda x: -x[1])[:3]
                           if v / tot >= 0.01)
        line = (f'{r}: {share:.0%} of its supply ({tot:.2f} a seat-game) from standard actions or tier-1 cards '
                f'(max {sc["cheap_share_max"]:.0%}): {detail or "none"}')
        if share > sc['cheap_share_max']:
            o.w('cheap-source', f'{r} {share:.0%}', line)
        else:
            o.i(line)


def check_shares(games, cfg, o):
    for key, cap, word in (('sources', cfg['source_share_max'], 'source'), ('uses', cfg['use_share_max'], 'spend')):
        t = tally(games, key)
        for r in SHARE_RES:
            tot = sum(t[r].values())
            if tot < cfg['share_min_per_seat']:
                continue
            # a phase (production, upkeep) pools every engine that pays in it: not one source
            phases = {s: v for s, v in t[r].items() if s.startswith('phase:')}
            rest = {s: v for s, v in t[r].items() if s not in phases}
            if phases:
                o.i(f'{r} {key}, phases (each pools many engines): ' +
                    ', '.join(f'{s} {v / tot:.0%}' for s, v in sorted(phases.items(), key=lambda x: -x[1])))
            if not rest:
                continue
            s, v = max(rest.items(), key=lambda x: x[1])
            line = f'{r}: one {word}, {s}, is {v / tot:.0%} of its {key} ({v:.2f} of {tot:.2f} a seat-game; max {cap:.0%})'
            why = cfg.get('share_exempt', {}).get(f'{r} {s}')
            if v / tot > cap and why:
                o.i(f'{line}; exempt: {why}')
            elif v / tot > cap:
                o.w(f'one-{word}', f'{r} {s} {v / tot:.0%}', line)
            else:
                o.i(line)


def check_repeats(boards, cfg, o):
    need = cfg['repeat_powers']['boards_min']
    # 281: a key names one power or a group, comma-joined ("Sprawl, Outpost"), that one reason covers
    groups = cfg['repeat_powers'].get('exempt', {})
    exempt = {n.strip() for names in groups for n in names.split(',')}
    by_name, by_fx = defaultdict(set), defaultdict(set)
    for names, why in sorted(groups.items()):
        o.i(f'repeat-power exemption {names}: {why}')
    for b in boards:
        for p in b['powers']:
            if p['name'] in exempt:
                continue
            by_name[p['name']].add(b['id'])
            by_fx[json.dumps(p['effects'], sort_keys=True)].add((b['id'], p['name']))
    for n, bs in sorted(by_name.items()):
        if len(bs) >= need:
            o.w('same-power', f'{n} on {len(bs)} boards', f'power {n} is on {len(bs)} boards: {", ".join(sorted(bs))}')
    for fx, held in sorted(by_fx.items()):
        bs = {b for b, _ in held}
        names = {n for _, n in held}
        if len(bs) >= need and len(names) > 1:
            txt = ', '.join(f'{b} {n}' for b, n in sorted(held))
            o.w('same-effect', f'{"/".join(sorted(names))} on {len(bs)} boards',
                f'one effect ({fx[:70]}) on {len(bs)} boards under {len(names)} names: {txt}')


def check_standard(cfg, base, o, games):
    sa = cfg['standard_actions']
    a = anchors()
    a.update({k: v for k, v in sa['extra_anchors'].items() if k != 'note'})
    cap = cfg['standard_anchor_max']
    n = len(games) or 1

    def val(side, priced_in=()):
        tot, parts = 0.0, []
        for unit, q in side.items():
            qty = base.get(q, 0) if isinstance(q, str) else q
            if isinstance(qty, list):
                qty = qty[0]
            if unit in priced_in:
                # 226: a unit another unit's anchor already holds (a liberated city's 45 holds its Sway)
                parts.append(f'{qty} {unit} (in the anchor)')
                continue
            tot += qty * a[unit]
            parts.append(f'{qty} {unit}')
        return tot, ' + '.join(parts)
    for row in sa['rows']:
        # 208 (P1): a row whose `off` tunable is 0 or missing is no action in this game: skip it, not "inf".
        if row.get('off') and not base.get(row['off'], 0):
            o.i(f'{row["name"]}: off (`{row["off"]}` 0)')
            continue
        give, gtxt = val(row['give'])
        get, ttxt = val(row['get'], row.get('priced_in', ()))
        ratio = get / give if give else float('inf')
        c = row['src']
        counter = 'board.std.' + c.split(':', 1)[-1]
        uses = sum(g['counters'].get(counter, 0) for g in games) / n if games else None
        used = f'; {uses:.2f} uses a game' if uses is not None else ''
        if used and row.get('case'):
            used += ' (every case of the action)'
        line = f'{row["name"]}: give {gtxt} = {give:g}, get {ttxt} = {get:g} at the anchors: {ratio:.2f}{used} (max {cap})'
        why = exemption(row, o)
        if ratio > cap and why:
            o.i(f'{line}; exempt ({row["case"]}): {why}')
        elif ratio > cap:
            o.w('standard-over-anchor', f'{row["name"]} {ratio:.2f}', line)
        else:
            o.i(line)
        circular(row, sa, a, give, o)
    check_jobs(cfg, base, a, o)


def exemption(row, o=None):
    """249: a row's exemption, per case: {case, why} naming the row's own `case`. A blanket one (a bare string, no
    case, or another case) is flagged (when `o` is given) and exempts nothing."""
    ex = row.get('exempt')
    if not ex:
        return None
    if isinstance(ex, dict) and ex.get('case') and ex.get('case') == row.get('case') and ex.get('why'):
        return ex['why']
    if o is not None:
        o.w('exempt-whole-action', row['name'], f'{row["name"]}: its exemption names no case of its own ({ex!r}): an '
            'exemption is per case, with a written reason (249)')
    return None


def circular(row, sa, a, give, o):
    """249: an anchor set from the price of the action it measures: the row then measures the action against
    itself. Flagged when `anchor_sources.set_from` names it, or when the row's single priced get is an extra anchor
    worth exactly its give with no `independent` derivation."""
    src = sa.get('anchor_sources', {})
    extra = sa['extra_anchors']
    for unit in row['get']:
        if unit in row.get('priced_in', ()) or unit not in extra:
            continue
        set_from = src.get('set_from', {}).get(unit)
        if set_from and set_from.split(' (')[0] == row['name']:
            o.w('circular-anchor', f'{unit} ({row["name"]})', f'{row["name"]}: the {unit} anchor ({a[unit]:g}) was set '
                f'from this action\'s price ({set_from}): its ratio measures the action against itself')
        elif not set_from and unit not in src.get('independent', {}) and len(row['get']) == 1:
            q = row['get'][unit]
            if isinstance(q, (int, float)) and abs(q * a[unit] - give) < 1e-9:
                o.w('circular-anchor', f'{unit} ({row["name"]})', f'{row["name"]}: the {unit} anchor ({a[unit]:g}) '
                    'equals the action\'s give, with no independent derivation in anchor_sources')


def card_pay(c, keep):
    """A card's pay at the anchors (248's price table): credits + 2.67 x (keep + play insight) + printed costs
    (273: r127's 4, 4.5, 6, 18 and 6, ÷ 1.5 with the credits)."""
    p = c.get('cost', 0) + 8 / 3 * (keep[str(c['tier'])] + c.get('insight', 0))
    return p + 3 * c.get('energy', 0) + 4 * c.get('chips', 0) + 12 * c.get('antimatter', 0) + 4 * c.get('research_cubes', 0)


def check_jobs(cfg, base, a, o):
    """249, job parity (Matt, 2026-11-16: cards are the engine, a standard action a rare fallback): each standard
    row's price per unit of its job against the median card for the job (scripts/design_check_jobs.json, 248's
    hand-read table), and its outcome against theirs."""
    sa = cfg['standard_actions']
    jp = sa.get('job_parity')
    if not jp:
        return
    data = jload(jp['file'])
    units = {k: (a[v] if isinstance(v, str) else v) for k, v in data['units'].items()}
    names = dict(units, uses=data['uses'], tags=data['tags'])
    deck = {c['name']: c for c in jload('content/cards.json')}

    def num(e):
        return e if isinstance(e, (int, float)) else eval(e, {'__builtins__': {}}, names)  # noqa: S307: our own table
    rank = jp['outcome_rank']
    for row in sa['rows']:
        if row.get('off') and not base.get(row['off'], 0):
            continue
        give = 0.0
        for unit, q in row['give'].items():
            qty = base.get(q, 0) if isinstance(q, str) else q
            give += (qty[0] if isinstance(qty, list) else qty) * a[unit]
        unit, q = next(iter(row['get'].items()))
        qty = base.get(q, 0) if isinstance(q, str) else q
        std = give / qty if qty else float('inf')
        job = data['jobs'].get(row['name'])
        why = exemption(row)  # check_standard flags a blanket one

        def flag(check, number, line):
            if why:
                o.i(f'{line}; exempt ({row["case"]}): {why}')
            else:
                o.w(check, number, line)
        if not job or not [c for c in job['cards'] if not c.get('power')]:
            flag('job-no-card', row['name'], f'{row["name"]}: no card does its job ({unit}); the action is the only '
                 'route' + (f' beside {", ".join(c["name"] for c in job["cards"])}' if job else ''))
            continue
        prices = []
        for c in job['cards']:
            if c.get('power'):
                continue
            card = deck.get(c['name'])
            if card is None:
                o.w('job-table', c['name'], f'{row["name"]}: job card {c["name"]!r} is not in content: refresh '
                    'scripts/design_check_jobs.json')
                continue
            uses = (c.get('uses', data['uses']) if num(c['use']) else 0)
            prices.append(((card_pay(card, data['keep_insight']) + uses * num(c['use']) - num(c['other'])) / num(c['units']), c['name']))
        if not prices:
            continue
        prices.sort()
        med = prices[len(prices) // 2][0] if len(prices) % 2 else (prices[len(prices) // 2 - 1][0] + prices[len(prices) // 2][0]) / 2
        line = (f'{row["name"]}: {std:.1f} a {unit} against the median card for its job {med:.1f} ({len(prices)} cards, '
                f'best {prices[0][1]} {prices[0][0]:.1f}): {std / med if med > 0 else float("inf"):.2f} x (min {jp["min"]})')
        if med > 0 and std < jp['min'] * med:
            flag('job-parity', f'{row["name"]} {std / med:.2f}', line)
        else:
            o.i(line)
        bad = [x for x in (row.get('outcome'), job.get('card_outcome')) if x and x not in rank]
        if bad:
            o.w('job-table', row['name'], f'{row["name"]}: outcome {bad[0]!r} is not in job_parity.outcome_rank')
        elif row.get('outcome') and job.get('card_outcome') and rank.index(row['outcome']) > rank.index(job['card_outcome']):
            flag('job-outcome', row['name'], f'{row["name"]}: the action leaves the site {row["outcome"]}; the cards '
                 f'for its job only leave it {job["card_outcome"]}')


def walk_sway(obj, where, tag, found, force_ops, ops_above=frozenset()):
    """Every sway op under obj: (where, side, amount, story tag, a removal beside it or above it)."""
    if isinstance(obj, dict):
        tag = obj.get('narrative', tag)
        for k, v in obj.items():
            walk_sway(v, where, tag, found, force_ops, ops_above)
    elif isinstance(obj, list):
        ops = {e.get('op') for e in obj if isinstance(e, dict)} | ops_above
        for e in obj:
            if isinstance(e, dict) and e.get('op') == 'sway':
                found.append((where, e.get('side'), e.get('amount'), e.get('narrative', tag), bool(ops & force_ops)))
            else:
                walk_sway(e, where, tag, found, force_ops, frozenset(ops))


def content_items(f, data):
    if isinstance(data, list):
        return data
    out = []
    for v in data.values():
        if isinstance(v, list):
            out += [x for x in v if isinstance(x, dict)]
        elif isinstance(v, dict):
            out.append(v)
    return out


def check_sway(cfg, base, o):
    sw = cfg['sway']
    force = set(sw['force_ops'])
    found = []
    for f in ('cards', 'ais', 'alignment', 'adversary', 'map', 'projects', 'threats'):
        for it in content_items(f, jload(f'content/{f}.json')):
            label = f'{f}:{it.get("id") or it.get("name") or "?"}'
            if f == 'ais':
                for p in it['powers']:
                    walk_sway(p, f'{label}:{p["name"]}', p.get('narrative'), found, force)
                walk_sway(it['economy_track'], f'{label}:track', None, found, force)
                continue
            walk_sway(it, label, None, found, force)
    # the standard actions' sway, from their params
    for row in cfg['standard_actions']['rows']:
        for side_key, side in (('give', 'discord'), ('get', 'unity')):
            q = row[side_key].get('sway')
            if q is not None:
                amt = base.get(q, 0) if isinstance(q, str) else q
                if amt:
                    found.append((f'standard:{row["name"]}', side, amt, row.get('narrative'), False))
    seen = set()
    untagged = []
    for where, side, amt, tag, forceful in found:
        if (where, side) in seen:
            continue
        seen.add((where, side))
        want = sw['tag_side'].get(tag) if tag else None
        if tag and want and want != side:
            o.w('sway-story', f'{where} {side} for {tag}', f'{where} sways {amt} to {side} but its story tag is {tag}')
        elif not tag:
            untagged.append(f'{where} {side} {amt}' + (' (beside a removal: force?)' if forceful and side == 'unity' else ''))
    if untagged:
        o.i(f'sway sources with no story tag ({len(untagged)}; 204 assigns them, information only): ' + '; '.join(untagged))


def op_keys(obj, acc):
    """Every op key under obj (212): `op`, with `:resource`, `:kind` and `:side` where the op names them."""
    if isinstance(obj, dict):
        if 'op' in obj:
            k = obj['op']
            for f in ('resource', 'kind', 'side'):
                if isinstance(obj.get(f), str):
                    k += ':' + obj[f]
            acc.add(k)
        for v in obj.values():
            op_keys(v, acc)
    elif isinstance(obj, list):
        for v in obj:
            op_keys(v, acc)
    return acc


def check_tag_story(cfg, cards, boards, o):
    """212: a card or tag-paid power whose owned effects are none of them its tags' (the tag table, `tag_story`)."""
    ts = cfg.get('tag_story')
    if not ts:
        return
    owner = defaultdict(set)
    for tag, row in ts['tags'].items():
        for k in row['ops']:
            owner[k].add(tag)
    items = [(f'cards:{c["id"]}', c['tags'], c) for c in cards]
    for b in boards:
        for p in b['powers']:
            tags = [t for t, _ in p.get('tags', [])]
            if tags:
                items.append((f'ais:{b["id"]}:{p["name"]}', tags, p.get('effects', [])))
    for where, tags, body in items:
        owned = {k: owner[k] for k in op_keys(body, set()) if owner.get(k)}
        if not owned or any(set(tags) & ts_ for ts_ in owned.values()):
            continue
        if where in ts.get('exceptions', {}):
            o.i(f'tag-story exception {where}: {ts["exceptions"][where]}')
            continue
        want = sorted(set().union(*owned.values()))
        o.w('tag-story', where, f'{where} is {"/".join(tags)} but its effects ({", ".join(sorted(owned))}) are '
            f'{"/".join(want)}')


def check_dead(games, cfg, boards, o):
    d = cfg['dead']
    n = len(games)
    kept, played, stuck = defaultdict(int), defaultdict(int), defaultdict(int)
    for g in games:
        for cid, c in (g.get('cards') or {}).items():
            kept[cid] += c.get('kept', 0)
            played[cid] += c.get('played', 0)
        # 237: a kept card left in hand at the end with no feasible part (`cards::unofferable`).
        for k, v in (g.get('counters') or {}).items():
            if k.startswith('cards.unofferable.'):
                stuck[k[len('cards.unofferable.'):]] += v
    dead = []
    for cid in kept:
        k, p = kept[cid] / n, played[cid] / n
        if k >= d['kept_min_per_game'] and p < d['played_max_per_game'] and p < d['played_per_kept_max'] * k:
            dead.append((cid, k, p))
    for cid, k, p in sorted(dead, key=lambda x: -x[1]):
        why = f'; left unofferable (no target) {stuck[cid] / n:.2f} a game' if stuck.get(cid) else ''
        o.w('kept-not-played', f'{cid} {p:.2f}', f'{cid}: kept {k:.2f} a game, played {p:.2f}{why}')
    # 207: powers unused (and dominant) are the usage check's; a resource gained and not spent is too, when the
    # usage config lists it
    listed = {r['key'] for r in cfg.get('usage', {}).get('rows', [])}
    seats = sum(len(g['spent']) for g in games) or 1
    for i, r in enumerate(PEAKS):
        if r in ('insight', 'card_tokens') or f'spend:{r}' in listed:
            continue
        gained = sum(row[i] for g in games for row in g['gained']) / seats
        spent = sum(row[i] for g in games for row in g['spent']) / seats
        if gained >= cfg['share_min_per_seat'] and spent / gained < d['spent_per_gained_min']:
            o.w('gained-not-spent', f'{r} {spent / gained:.0%}', f'{r}: gained {gained:.2f} a seat-game, spent {spent:.2f} ({spent / gained:.0%})')


# ---- 207: expected use ------------------------------------------------------------------------------------------

USE_KINDS = {'standard', 'power', 'convert', 'sell', 'use', 'play', 'pay', 'counter_with', 'withdraw'}


def power_order(games, builds, boards):
    """255: each board's powers in the read's own order, {board: {index: name}}, and the slugs its counters name.

    An action gives a power's index in the board of the read's revision; its counters (`ai.power.<board>.<slug>`)
    give its name. Across the games seating a board, index i's uses a game match exactly one slug's counter (powers
    used identically in every game tally the same either way), so the order is solved from the read itself and a read
    of an older revision is read right without its `content/ais.json`. An index no counter matches (a pre-198 read,
    keyed by index) keeps the current name."""
    cur = {b['id']: [p['name'] for p in b['powers']] for b in boards}
    by_slug = {b: {power_slug(n): n for n in ns} for b, ns in cur.items()}
    idx, cnt, seen = {}, {}, defaultdict(set)   # (board, index | slug) -> {game: uses}
    for gi, (g, sb) in enumerate(zip(games, builds)):
        for a in g['actions']:
            if a.get('type') == 'power':
                d = idx.setdefault((sb[a['seat']], a['power']), {})
                d[gi] = d.get(gi, 0) + 1
        for k, v in (g.get('counters') or {}).items():
            p = k.split('.')
            if len(p) >= 4 and p[0] == 'ai' and p[1] in ('power', 'dear'):
                seen[p[2]].add(p[3])
                if p[1] == 'power' and len(p) == 4 and v:
                    cnt.setdefault((p[2], p[3]), {})[gi] = v
    order = {b: dict(enumerate(ns)) for b, ns in cur.items()}
    for b in {b for b, _ in idx}:
        free = sorted((s for bb, s in cnt if bb == b and not s.isdigit()),
                      key=lambda s: (list(by_slug.get(b, {})).index(s) if s in by_slug.get(b, {}) else 99, s))
        for i in sorted(i for bb, i in idx if bb == b):
            s = next((s for s in free if cnt[(b, s)] == idx[(b, i)]), None)
            if s is not None:
                free.remove(s)
                order.setdefault(b, {})[i] = by_slug.get(b, {}).get(s) or s.replace('_', ' ').title()
    return order, seen


def action_key(a, seat_board, order, countries):
    """An action's usage key (`standard:troop`, `power:builder:Mint`, `convert:Tidal Grid[:chips]`, `play`, ...), or
    None for one that is not a player's use (pass, choose, the draft). ORDER: `power_order`'s names by index."""
    t = a.get('type')
    if t not in USE_KINDS:
        return None
    if t == 'standard':
        return f'standard:{a["act"]}'
    if t == 'power':
        b = seat_board[a['seat']]
        return f'power:{b}:{order.get(b, {}).get(a["power"], a["power"])}'
    if t == 'convert':
        c = a['country']
        name = countries[c]['project']['name'] if c < len(countries) else str(c)
        return f'convert:{name}' + (':chips' if a.get('chips') else '')
    return t


def usage_counts(games, boards, countries, listed=()):
    """key -> tallies over the seat-games that could take it (a power: its board's seats): uses, seat-games, seat-rounds,
    seat-games with a use, and uses and seat-games by board and by level. Resource spends (`spend:<resource>`): spent
    and gained, by board and level, in `uses` and `seats`. Also the boards whose power order in the read is not the
    current one (`power_order`): their unused powers are listed only when the read's counters name them."""

    def cell():
        return {'uses': 0, 'seats': 0, 'rounds': 0, 'with': 0}
    T = defaultdict(lambda: dict(cell(), board=defaultdict(cell), level=defaultdict(cell)))

    def add(k, b, lvl, u, seats, rounds, w):
        row = T[k]
        for c in (row, row['board'][b], row['level'][lvl]):
            c['uses'] += u
            c['seats'] += seats
            c['rounds'] += rounds
            c['with'] += w
    builds = [[p.split(':', 1)[0] for p in (g.get('build') or '').split('|') if p] for g in games]
    # every key seen anywhere, and every power of a seated board, is tallied over every seat-game that could take it
    seated = {b for sb in builds for b in sb}
    order, seen = power_order(games, builds, boards)
    # 255: a board whose order differs is from another revision: a current power its counters never name may not
    # have existed then, so it is no row of zeros
    moved = {b['id'] for b in boards if b['id'] in seated
             and any(order[b['id']].get(i) != p['name'] for i, p in enumerate(b['powers']))}

    def known(k):
        b, name = k.split(':', 2)[1:]
        return b not in moved or power_slug(name) in seen[b]
    keys = {f'power:{b["id"]}:{p["name"]}' for b in boards if b['id'] in seated for p in b['powers']}
    # a listed action never taken is a row of zeros (a power only when its board was seated)
    keys |= {k for k in listed if not k.startswith('spend:') and not (k.startswith('power:') and k.split(':')[1] not in seated)}
    keys = {k for k in keys if not k.startswith('power:') or known(k)}
    per_game = []
    for g, sb in zip(games, builds):
        per = defaultdict(lambda: defaultdict(int))     # key -> seat -> uses
        for a in g['actions']:
            k = action_key(a, sb, order, countries)
            if k:
                per[k][a['seat']] += 1
                keys.add(k)
        per_game.append(per)
    for g, sb, per in zip(games, builds, per_game):
        lvl = int(g['difficulty'])
        for k in keys:
            pb = k.split(':')[1] if k.startswith('power:') else None
            for s, b in enumerate(sb):
                if pb and pb != b:
                    continue
                u = per[k][s] if k in per else 0
                add(k, b, lvl, u, 1, g['rounds'], int(u > 0))
        for i, r in enumerate(PEAKS):
            for s, b in enumerate(sb):
                # a resource: `uses` spent, `seats` gained
                add(f'spend:{r}', b, lvl, g['spent'][s][i], g['gained'][s][i], 0, 0)
    return T, sorted(moved)


def usage_value(row, measure):
    """The row's use in its measure: `seat_game` uses a seat-game, `seat_round` uses a seat-round, `games` the share of
    seat-games with a use, `spent` a resource's spent over gained."""
    if measure == 'seat_round':
        return row['uses'] / row['rounds'] if row['rounds'] else 0.0
    if measure == 'games':
        return row['with'] / row['seats'] if row['seats'] else 0.0
    return row['uses'] / row['seats'] if row['seats'] else 0.0


def usage_rows(games, cfg, boards, countries, base):
    """Each action seen or listed, with its role, measure, band and use: (key, name, role, measure, band, value, row,
    status) where status is `in`, `low`, `high`, `off` or `unbanded`."""
    u = cfg.get('usage') or {}
    roles = u.get('roles', {})
    T, moved = usage_counts(games, boards, countries, [r['key'] for r in u.get('rows', []) if r.get('role') != 'off'])
    out, seen = [], set()
    for b in moved:
        out.append((f'order:{b}', b, '', 'seat_game', None, None, None, 'reordered'))
    for r in u.get('rows', []):
        k = r['key']
        seen.add(k)
        if 'name' not in r and k.startswith('power:'):
            r = dict(r, name=f'{k.split(":")[1].capitalize()} {k.split(":", 2)[2]}')
        role = roles.get(r.get('role'), {})
        measure = r.get('measure') or role.get('measure', 'seat_game')
        band = r.get('band') or role.get('band')
        row = T.get(k)
        off = r.get('off')
        if off is True or (isinstance(off, str) and not base.get(off, 0)):
            out.append((k, r.get('name', k), r.get('role', ''), measure, band, None, row, 'off'))
            continue
        if row is None or not row['seats']:
            absent = row is None and k.startswith('power:') and k.split(':')[1] in moved
            out.append((k, r.get('name', k), r.get('role', ''), measure, band, None, row, 'absent' if absent else 'unseated'))
            continue
        v = usage_value(row, 'seat_game' if measure == 'spent' else measure)
        status = 'unbanded' if not band else 'low' if v < band[0] else 'high' if v > band[1] else 'in'
        out.append((k, r.get('name', k), r.get('role', ''), measure, band, v, row, status))
    for k in sorted(T):
        if k not in seen and not k.startswith('spend:'):
            row = T[k]
            out.append((k, k, '', 'seat_game', None, usage_value(row, 'seat_game'), row, 'unbanded'))
    return out


def fmt_use(v, measure):
    return '-' if v is None else f'{v:.0%}' if measure in ('games', 'spent') else f'{v:.2f}'


def check_usage(games, cfg, boards, countries, base, o):
    for k, name, role, measure, band, v, row, status in usage_rows(games, cfg, boards, countries, base):
        what = {'seat_game': 'a seat-game', 'seat_round': 'a seat-round', 'games': 'of seat-games',
                'spent': 'of what is gained, spent'}[measure]
        btxt = f'{fmt_use(band[0], measure)}-{fmt_use(band[1], measure)}' if band else ''
        if status in ('low', 'high'):
            o.w('usage', f'{name} {fmt_use(v, measure)} outside {btxt}',
                f'{name} ({role}): {fmt_use(v, measure)} {what}, outside its band {btxt} '
                f'({"unused" if status == "low" else "dominant"})')
        elif status == 'unbanded' and v:
            o.i(f'usage: {name}: {fmt_use(v, measure)} {what}, no band (add a row to `usage`)')
        elif status == 'unseated':
            o.i(f'usage: {name}: no seat could take it in this read (a board not seated, or a stale `usage` row)')
        elif status == 'absent':
            o.i(f'usage: {name}: the read\'s counters never name it (not on its board in that revision, or never used)')
        elif status == 'reordered':
            o.i(f'usage: board {name}: the read orders its powers otherwise than content/ais.json (another revision): '
                'powers read by their counters; one it never names is no row')


def usage_table(games, cfg, boards, countries, base):
    """The inventory as Markdown: every action's role, band, pooled use, by board and by level, and in or out."""
    rows = usage_rows(games, cfg, boards, countries, base)
    lv = sorted({int(g['difficulty']) for g in games})
    bd = [b['id'] for b in boards]
    hd = ['action', 'key', 'role', 'measure', 'band', 'use', 'in?'] + bd + [f'd{x}' for x in lv]
    lines = ['| ' + ' | '.join(hd) + ' |', '|' + '---|' * len(hd)]
    for k, name, role, measure, band, v, row, status in rows:
        if status == 'reordered':
            continue

        def part(p):
            if not p or not p['seats']:
                return '-'
            return fmt_use(usage_value(p, 'seat_game' if measure == 'spent' else measure), measure)
        btxt = f'{fmt_use(band[0], measure)}-{fmt_use(band[1], measure)}' if band else ''
        cells = [name, f'`{k}`', role, measure, btxt, fmt_use(v, measure), status]
        cells += [part(row['board'].get(b)) if row else '-' for b in bd]
        cells += [part(row['level'].get(x)) if row else '-' for x in lv]
        lines.append('| ' + ' | '.join(cells) + ' |')
    return '\n'.join(lines)


def check_victory_needs(games, cfg, o):
    v = cfg['victory_needs']
    won = [g for g in games if g['won'] and int(g['difficulty']) in v['levels']]
    if not won:
        o.i(f'victory needs: no won games at d{v["levels"]}')
        return
    for r in v['resources']:
        i = PEAKS.index(r)
        share = sum(1 for g in won if sum(row[i] for row in g['spent']) > 0) / len(won)
        line = f'{r}: spent in {share:.0%} of won games at d{",".join(map(str, v["levels"]))} ({len(won)} won; min {v["spent_share_min"]:.0%})'
        if share < v['spent_share_min']:
            o.w('victory-skips', f'{r} {share:.0%}', line)
        else:
            o.i(line)


def by_level(games):
    lv = defaultdict(list)
    for g in games:
        lv[int(g['difficulty'])].append(g)
    return dict(sorted(lv.items()))


def check_rounds(games, cfg, o):
    lo, hi = cfg['rounds']['min'], cfg['rounds']['max']
    for d, gs in by_level(games).items():
        m = sum(g['rounds'] for g in gs) / len(gs)
        won = [g['rounds'] for g in gs if g['won']]
        mw = f', won games {sum(won) / len(won):.1f}' if won else ''
        line = f'd{d}: {m:.1f} rounds a game{mw} ({len(gs)} games; target {lo}-{hi})'
        if not lo <= m <= hi:
            o.w('rounds', f'd{d} {m:.1f}', line)
        else:
            o.i(line)


def tokens(games, cfg):
    t = cfg['tokens']
    adv = ply = rounds = 0
    for g in games:
        r = max(g['rounds'], 1)
        rounds += r
        adv += sum(g.get('adversary_ops') or [])
        c = g['counters']
        ply += sum(c.get(k, 0) for k in t['player_counters'])
        ply += sum(v for k, v in c.items() if any(k.startswith(p) for p in t['player_prefixes']))
    return adv / max(rounds, 1), ply / max(rounds, 1)


def check_tokens(games, old, cfg, o):
    adv, ply = tokens(games, cfg)
    if not old:
        o.i(f'tokens a round: adversary ops {adv:.1f}, player token moves {ply:.1f} (no --old: nothing to compare)')
        return
    oadv, oply = tokens(old, cfg)
    cap = cfg['tokens']['rise_max']
    for name, new, was in (('adversary ops', adv, oadv), ('player token moves', ply, oply)):
        rise = new / was - 1 if was else 0
        line = f'tokens a round, {name}: {new:.1f} against {was:.1f} ({rise:+.0%}; max +{cap:.0%})'
        if rise > cap:
            o.w('tokens', f'{name} {rise:+.0%}', line)
        else:
            o.i(line)


def counter_sum(c, keys, prefixes=()):
    return sum(c.get(k, 0) for k in keys) + sum(v for k, v in c.items() if any(k.startswith(p) for p in prefixes))


def table_cost(gs, cfg, countries):
    """268: what a round costs a human at the table, three proxies from a read's counters (each a sum over the games,
    over their rounds): components moved a round (the adversary's ops, the tokens check's player moves and the
    production pieces); per-site checks a round (each site on the map times the passes that check every site, the
    cities times growth's passes, each troop's march step, each agent Rally deals and each threat's location test); and
    once-a-round uses a seat a round, each something a player remembers with no marker on the table."""
    t, tc = cfg['tokens'], cfg['table_cost']
    comp = check = mem = rounds = seat_rounds = 0
    for g in gs:
        c, r = g['counters'], max(g['rounds'], 1)
        seats = len(g.get('start') or [0, 0])
        rounds += r
        seat_rounds += r * seats
        comp += sum(g.get('adversary_ops') or [])
        # A troop's step is already an adversary op: the tokens check's troop counters would count it again.
        comp += counter_sum(c, [k for k in t['player_counters'] if k not in tc['ops_already']], t['player_prefixes'])
        comp += counter_sum(c, tc['production_counters'])
        cities = sum(len(x['cities']) for x in countries[:seats])
        for i in range(1, r + 1):
            sites = counter_sum(c, [f'map.{k}.{own}.r{i:02d}' for k in ('factories', 'stations')
                                    for own in ('adversary', 'neutral', 'player')])
            check += tc['site_passes'] * sites + tc['city_passes'] * cities + c.get(f'map.troops.r{i:02d}', 0)
        check += counter_sum(c, tc['check_counters'], tc.get('check_prefixes', ()))
        mem += counter_sum(c, tc['memory_counters'], tc['memory_prefixes'])
    return {'components': comp / max(rounds, 1), 'checks': check / max(rounds, 1), 'memory': mem / max(seat_rounds, 1)}


def check_table_cost(games, old, cfg, countries, o):
    """268: each table-cost proxy against its soft budget by level (design_check.json `table_cost`, set from 268's
    hand-play pass and 267's final read), and against --old's same level (the tokens check's rise_max)."""
    tc = cfg['table_cost']
    names = {'components': 'components moved a round', 'checks': 'per-site checks a round',
             'memory': 'once-a-round uses a seat a round'}
    olv = by_level(old) if old else {}
    cap = cfg['tokens']['rise_max']
    for d, gs in by_level(games).items():
        now = table_cost(gs, cfg, countries)
        was = table_cost(olv[d], cfg, countries) if d in olv else None
        for k, v in now.items():
            budget = tc['budget'][k][d] if d < len(tc['budget'][k]) else tc['budget'][k][-1]
            line = f'd{d}: {names[k]} {v:.1f} (budget {budget})'
            if was and was[k]:
                rise = v / was[k] - 1
                line += f', {rise:+.0%} on --old ({was[k]:.1f})'
                if rise > cap:
                    o.w(f'table-{k}-rise', f'd{d} {rise:+.0%}', line + f' (max +{cap:.0%})')
            if v > budget:
                o.w(f'table-{k}', f'd{d} {v:.1f}', line)
            else:
                o.i(line)


def check_friction_fresh(cfg, o):
    """268: the friction review's refresh trigger. docs/rulebook-friction.md's title names the rules revision it was
    last run at; warn once RULES_REVISION has moved `refresh_every` past it."""
    every = cfg['table_cost']['refresh_every']
    with open(os.path.join(ROOT, 'docs', 'rulebook-friction.md'), encoding='utf-8') as f:
        m = re.search(r'as of rules revision (\d+)', f.readline())
    with open(os.path.join(ROOT, 'crates', 'aic-core', 'src', 'lib.rs'), encoding='utf-8') as f:
        rev = int(re.search(r'pub const RULES_REVISION: u32 = (\d+);', f.read()).group(1))
    if not m:
        o.w('friction-stale', 'no revision', 'docs/rulebook-friction.md: no "as of rules revision N" in its title')
        return
    was = int(m.group(1))
    line = f'friction review: run at r{was}, rules now r{rev} (rerun every {every})'
    if rev - was >= every:
        o.w('friction-stale', f'r{was} at r{rev}', line + ': rerun it (docs/rulebook-friction.md, *Refreshing*)')
    else:
        o.i(line)


def git(*args):
    return subprocess.run(['git', *args], cwd=ROOT, capture_output=True, text=True, encoding='utf-8').stdout


def check_dials(cfg, base_rev, o):
    dc = cfg['dials']
    rev = base_rev or git('merge-base', 'HEAD', 'main').strip()
    if not rev:
        o.i('dials: no base revision')
        return
    old_txt = git('show', f'{rev}:configs/baseline.json')
    if not old_txt:
        o.i(f'dials: no configs/baseline.json at {rev}')
        return
    old = flat_baseline(json.loads(old_txt))
    new = flat_baseline(jload('configs/baseline.json'))
    design = [f for f in git('diff', '--name-only', rev, '--', *dc['design_paths']).split() if f]
    moved = []
    for k in sorted(set(old) & set(new)):
        a, b = old[k], new[k]
        pairs = list(zip(a, b)) if isinstance(a, list) and isinstance(b, list) and len(a) == len(b) else [(a, b)]
        for i, (x, y) in enumerate(pairs):
            if isinstance(x, (int, float)) and isinstance(y, (int, float)) and not isinstance(x, bool) and x != y:
                share = abs(y - x) / max(abs(x), 1)
                if share >= dc['move_min']:
                    moved.append(f'{k}{"[%d]" % i if len(pairs) > 1 else ""} {x}->{y}')
    if not moved:
        o.i(f'dials: none moved {dc["move_min"]:.0%} or more since {rev[:9]}')
        return
    line = f'since {rev[:9]}: {len(moved)} dial(s) moved {dc["move_min"]:.0%}+ ({", ".join(moved)})'
    if design:
        o.w('dials-hide-design', f'{len(moved)} dials beside {len(design)} design files',
            line + f' in the same change as {len(design)} design file(s) ({", ".join(design[:4])}{"..." if len(design) > 4 else ""})')
    else:
        o.i(line + ', no design files changed')


def check_ladder(cfg, o):
    """267: the difficulty ladder's budget (soft): over about one new mid-game rule at a level, or about three active at
    the top level. Reads web/card/adversary.json, the ladder `aic-sim docs` writes from content and the config. A rule
    counts by its mid-game changes (one a mid-game dial, `new_changes`), so a named rule merging two counts two."""
    lc = cfg['ladder']
    levels = jload('web/card/adversary.json')['levels']
    for v in levels:
        if v['new_changes'] > lc['max_new']:
            o.w('ladder-new', f'level {v["level"]} {v["new_changes"]}',
                f'level {v["level"]} ({v["name"]}) adds {v["new_changes"]} mid-game changes in {v["new_rules"]} named '
                f'rule(s) (budget about {lc["max_new"]})')
    top = levels[-1]
    if top['active_changes'] > lc['max_active']:
        o.w('ladder-active', f'level {top["level"]} {top["active_changes"]}',
            f'level {top["level"]} has {top["active_changes"]} mid-game changes active in {top["active_rules"]} named '
            f'rules (budget about {lc["max_active"]})')
    o.i('ladder: named rules / mid-game changes active by level ' +
        ', '.join(f'{v["level"]}: {v["active_rules"]} / {v["active_changes"]}' for v in levels))


# ---- main -------------------------------------------------------------------------------------------------------

def main():
    argv = sys.argv[1:]

    def opt(name):
        if name in argv:
            i = argv.index(name)
            if i + 1 >= len(argv) or argv[i + 1].startswith('--'):
                sys.exit(f'{name} needs a value')
            v = argv[i + 1]
            del argv[i:i + 2]
            if name in argv:
                sys.exit(f'{name} given twice: one, a comma list for several')
            return v
        return None
    cfg_path = opt('--config') or os.path.join(ROOT, 'scripts', 'design_check.json')
    old_dirs = opt('--old')
    base_rev = opt('--base')
    variants = opt('--variant')
    variants = set(variants.split(',')) if variants else None
    agents = opt('--agent')
    agents = agent_list(agents) if agents else None
    quiet = '--quiet' in argv
    table = '--usage' in argv
    argv = [a for a in argv if a not in ('--quiet', '--usage')]
    # 266 (D117): an unknown flag or a second DIR was ignored, so the read ran on less than was asked.
    flags = [a for a in argv if a.startswith('-')]
    if flags:
        sys.exit(f"unknown flag {flags[0]}: the flags are --old, --base, --config, --variant, --agent, --quiet, "
                 "--usage")
    if len(argv) > 1:
        sys.exit(f"one DIR only (got {len(argv)}): pool reads as a comma list, {','.join(argv)}")
    with open(cfg_path, encoding='utf-8') as f:
        cfg = json.load(f)
    cards = jload('content/cards.json')
    boards = jload('content/ais.json')
    base = flat_baseline(jload('configs/baseline.json'))
    games = load_games(argv[0], variants, agents) if argv else []
    if (variants or agents) and not argv:
        sys.exit(f"{'--variant' if variants else '--agent'} needs a read")
    old = load_games(old_dirs, agents=agents) if old_dirs else []  # 286: --agent filters both sides
    countries = jload('content/map.json')['countries']
    if table:
        if not games:
            sys.exit('--usage needs a read')
        print(usage_table(games, cfg, boards, countries, base))
        return
    o = Out()
    check_repeats(boards, cfg, o)
    check_standard(cfg, base, o, games)
    check_sway(cfg, base, o)
    check_tag_story(cfg, cards, boards, o)
    check_dials(cfg, base_rev, o)
    check_ladder(cfg, o)
    check_friction_fresh(cfg, o)
    if games:
        check_scarce(games, cfg, cards, o)
        check_shares(games, cfg, o)
        check_dead(games, cfg, boards, o)
        check_usage(games, cfg, boards, countries, base, o)
        check_victory_needs(games, cfg, o)
        check_rounds(games, cfg, o)
        check_tokens(games, old, cfg, o)
        check_table_cost(games, old, cfg, countries, o)
    else:
        o.i('no read given: the play checks (supply shares, dead cards and powers, rounds, tokens, table cost) did not '
            'run')
    # the header: each check's warnings, a check with many as its count and its first three
    by = defaultdict(list)
    for c, n, _ in o.warn:
        by[c].append(n)
    parts = []
    for c, ns in by.items():
        if len(ns) > 3:
            parts.append(f'{c} {len(ns)} ({", ".join(ns[:3])}, ...)')
        else:
            parts += [f'{c} {n}' for n in ns]
    # each part led by its kind, so conductor files every one under `design`
    head = '; '.join(f'design: {p}' for p in parts)
    print('warnings: ' + (head or 'none'))
    print()
    if games:
        print(f'{len(games)} games' + (f', old side {len(old)}' if old else ''))
    print(f'## Warnings ({len(o.warn)})')
    for c, _, line in o.warn:
        print(f'- [{c}] {line}')
    if not quiet:
        print(f'\n## Information ({len(o.info)})')
        for line in o.info:
            print(f'- {line}')


if __name__ == '__main__':
    main()

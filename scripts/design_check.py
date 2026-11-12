"""205: design warnings that fire on their own. Reads content/ and configs/ (the tree), and a read's farmed games, and
prints every design warning it trips, each with its number. Warnings never stop work: no exit code but 0.

    python scripts/design_check.py [DIR[,DIR...]] [--old DIR[,DIR...]] [--base REV] [--config FILE] [--quiet]

DIR is a read's folder (its games.jsonl: a farmed read assembled with its header, or an experiment's output); comma
lists pool. With no DIR only the content checks run. `--old` is the previous revision's read on the same seeds (the
tokens-a-round check reads against it). `--base` is the revision the item started from (default: the merge base with
main); the dials check diffs configs/baseline.json against it. Thresholds and source classes are in
scripts/design_check.json, all candidates. The principles are docs/principles.md, *Design warnings*.

The first line printed is the report header's: `warnings: design: <check> <number>; ...` or `warnings: none`.
"""
import json
import os
import re
import subprocess
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
SHARE_RES = ['energy', 'chips', 'antimatter', 'research_cubes', 'ambassadors']


def jload(rel):
    with open(os.path.join(ROOT, rel), encoding='utf-8') as f:
        return json.load(f)


def snake(name):
    return re.sub(r'[^a-z0-9]+', '_', name.lower()).strip('_')


# ---- inputs -----------------------------------------------------------------------------------------------------

def load_games(dirs):
    games = []
    for d in dirs.split(','):
        path = d if d.endswith('.jsonl') else os.path.join(d, 'games.jsonl')
        with open(path, encoding='utf-8') as f:
            for line in f:
                g = json.loads(line)
                if 'format' in g:
                    continue
                games.append(g)
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
             'draw 1 card': 'card', '1 chip': 'chips', 'remove 1 compute from a project': 'compute'}
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
    a['research_cubes'] = 6  # past checks (card-guidelines.md): 3 at values.json
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
            if v / tot > cap:
                o.w(f'one-{word}', f'{r} {s} {v / tot:.0%}', line)
            else:
                o.i(line)


def check_repeats(boards, cfg, o):
    need = cfg['repeat_powers']['boards_min']
    by_name, by_fx = defaultdict(set), defaultdict(set)
    for b in boards:
        for p in b['powers']:
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

    def val(side):
        tot, parts = 0.0, []
        for unit, q in side.items():
            qty = base.get(q, 0) if isinstance(q, str) else q
            if isinstance(qty, list):
                qty = qty[0]
            tot += qty * a[unit]
            parts.append(f'{qty} {unit}')
        return tot, ' + '.join(parts)
    for row in sa['rows']:
        give, gtxt = val(row['give'])
        get, ttxt = val(row['get'])
        ratio = get / give if give else float('inf')
        c = row['src']
        counter = 'alignment.antimatter_actions' if c == 'antimatter' else 'board.std.' + c.split(':', 1)[-1]
        uses = sum(g['counters'].get(counter, 0) for g in games) / n if games else None
        used = f'; {uses:.2f} uses a game' if uses is not None else ''
        line = f'{row["name"]}: give {gtxt} = {give:g}, get {ttxt} = {get:g} at the anchors: {ratio:.2f}{used} (max {cap})'
        if ratio > cap:
            o.w('standard-over-anchor', f'{row["name"]} {ratio:.2f}', line)
        else:
            o.i(line)


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
                found.append((where, e.get('side'), e.get('amount'), tag, bool(ops & force_ops)))
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


def check_dead(games, cfg, boards, o):
    d = cfg['dead']
    n = len(games)
    kept, played = defaultdict(int), defaultdict(int)
    for g in games:
        for cid, c in (g.get('cards') or {}).items():
            kept[cid] += c.get('kept', 0)
            played[cid] += c.get('played', 0)
    dead = []
    for cid in kept:
        k, p = kept[cid] / n, played[cid] / n
        if k >= d['kept_min_per_game'] and p < d['played_max_per_game'] and p < d['played_per_kept_max'] * k:
            dead.append((cid, k, p))
    for cid, k, p in sorted(dead, key=lambda x: -x[1]):
        o.w('kept-not-played', f'{cid} {p:.2f}', f'{cid}: kept {k:.2f} a game, played {p:.2f}')
    # powers: uses a seat-game for each board, over the games that seated it
    seated = defaultdict(int)
    for g in games:
        for part in (g.get('build') or '').split('|'):
            if part:
                seated[part.split(':', 1)[0]] += 1
    for b in boards:
        if not seated[b['id']]:
            continue
        for p in b['powers']:
            key = f'ai.power.{b["id"]}.{snake(p["name"])}'
            u = sum(g['counters'].get(key, 0) for g in games) / seated[b['id']]
            if u < d['power_used_max_per_seat_game']:
                o.w('power-unused', f'{b["id"]} {p["name"]} {u:.2f}', f'{b["id"]} power {p["name"]}: used {u:.2f} a seat-game')
    seats = sum(len(g['spent']) for g in games) or 1
    for i, r in enumerate(PEAKS):
        if r in ('insight', 'card_tokens'):
            continue
        gained = sum(row[i] for g in games for row in g['gained']) / seats
        spent = sum(row[i] for g in games for row in g['spent']) / seats
        if gained >= cfg['share_min_per_seat'] and spent / gained < d['spent_per_gained_min']:
            o.w('gained-not-spent', f'{r} {spent / gained:.0%}', f'{r}: gained {gained:.2f} a seat-game, spent {spent:.2f} ({spent / gained:.0%})')


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


# ---- main -------------------------------------------------------------------------------------------------------

def main():
    argv = sys.argv[1:]

    def opt(name):
        if name in argv:
            i = argv.index(name)
            v = argv[i + 1]
            del argv[i:i + 2]
            return v
        return None
    cfg_path = opt('--config') or os.path.join(ROOT, 'scripts', 'design_check.json')
    old_dirs = opt('--old')
    base_rev = opt('--base')
    quiet = '--quiet' in argv
    argv = [a for a in argv if a != '--quiet']
    with open(cfg_path, encoding='utf-8') as f:
        cfg = json.load(f)
    cards = jload('content/cards.json')
    boards = jload('content/ais.json')
    base = flat_baseline(jload('configs/baseline.json'))
    games = load_games(argv[0]) if argv else []
    old = load_games(old_dirs) if old_dirs else []
    o = Out()
    check_repeats(boards, cfg, o)
    check_standard(cfg, base, o, games)
    check_sway(cfg, base, o)
    check_dials(cfg, base_rev, o)
    if games:
        check_scarce(games, cfg, cards, o)
        check_shares(games, cfg, o)
        check_dead(games, cfg, boards, o)
        check_victory_needs(games, cfg, o)
        check_rounds(games, cfg, o)
        check_tokens(games, old, cfg, o)
    else:
        o.i('no read given: the play checks (supply shares, dead cards and powers, rounds, tokens) did not run')
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
    head = '; '.join(parts)
    print('warnings: ' + (f'design: {head}' if head else 'none'))
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

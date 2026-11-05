"""082: what agents use, per agent and board pair, pooled over DIRs: the dear powers
(`ai::power_is_dear`: two tags or more, or a tag and an energy and no node), Grant, Action buildings' uses, tokens
gained and spent, each liberate card kept / played (Popular Uprising's Action uses),
and per agent the draft cards kept in more than --min percent of games and never played,
and those never kept.

    python scripts/cards/use_read.py DIR... [--min PCT] [--agents A,B]

Reads `games.jsonl` and `results.json` (scripts/screens.py). Seconds.
"""
import argparse
import collections
import json
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))
from screens import content, load, cnt  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

LIBERATE = ['nuclear_option', 'power_to_the_people', 'hostile_buyout', 'forensic_audit',
            'root_access', 'peacekeeping_mission', 'popular_uprising', 'great_liberation']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--min', type=float, default=1.0)
    ap.add_argument('--agents', default='')
    a = ap.parse_args()
    want = set(x for x in a.agents.split(',') if x)
    cards = content('cards')
    boards = content('ais')
    draft = {c['id']: c for c in cards if c.get('tier') in (1, 2)}
    names = {c['id']: c['name'] for c in cards}
    dear = {}
    for b in boards:
        for k, p in enumerate(b['powers']):
            need = sum(n for _, n in p.get('tags', []))
            # 099: the engine's dear powers (ai::power_is_dear): 2+ tags, or a tag and an energy (r35's);
            # 2+ tags alone matched none after r35.
            places = any(e['op'] == 'place_node' for e in p['effects'])
            if need >= 2 or (need >= 1 and not places and any(r == 'energy' and n > 0 for r, n in p.get('cost') or [])):
                dear[f"ai.power.{b['id']}.{k}"] = f"{b['name']} {p['name']}"
            if p['name'] == 'Grant':
                grant = f"ai.power.{b['id']}.{k}"
    by = collections.defaultdict(list)
    by_agent = collections.defaultdict(list)
    # 118: only the runs' decks (each cell's config's `cards` list, and any card a game saw):
    # a card cut from the deck (117's three) is still in content and read as "never kept".
    dealt = set()
    for d in a.dirs:
        res = json.load(open(os.path.join(d, 'results.json'), encoding='utf-8'))
        dealt.update(cid for per in res.get('cards', {}).values() for cid in per)
        for path in {c.get('config') for c in res['cells'] if c.get('config')}:
            full = path if os.path.isabs(path) else os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))), path)
            if os.path.exists(full):
                for m in json.load(open(full, encoding='utf-8')).get('modules', []):
                    if m.get('module') == 'cards':
                        dealt.update(m.get('cards', []))
    draft = {k: v for k, v in draft.items() if k in dealt}
    for d in a.dirs:
        cells, games = load(d)
        for cid, c in cells.items():
            if want and c['agent'] not in want:
                continue
            by[(c['agent'], c.get('variant', ''))].extend(games[cid])
            by_agent[c['agent']].extend(games[cid])
    head = ['agent', 'pair', 'games', 'win %', *dear.values(), 'Grant', 'Actions used', 'tokens gained / spent',
            *(names[x] for x in LIBERATE)]
    print('| ' + ' | '.join(head) + ' |')
    print('|' + '---|' * len(head))
    for (agent, pair), gs in sorted(by.items()):
        n = len(gs)
        m = lambda k: sum(cnt(g, k) for g in gs) / n  # noqa: E731
        used = sum(v for g in gs for k, v in g['counters'].items()
                   if k.startswith('cards.used.') and '.r' not in k[len('cards.used.'):]) / n
        lib = []
        for x in LIBERATE:
            s = f"{m('cards.kept.' + x):.2f} / {m('cards.played.' + x):.2f}"
            if x == 'popular_uprising':
                s += f" ({m('cards.used.' + x):.2f})"
            lib.append(s)
        win = 100 * sum(g['won'] for g in gs) / n
        print(f"| {agent} | {pair} | {n} | {win:.1f} | " +
              ' | '.join(f"{m(k):.2f}" for k in dear) +
              f" | {m(grant):.2f} | {used:.2f} | {m('cards.tokens_gained'):.1f} / {m('cards.tokens_spent'):.1f} | " +
              ' | '.join(lib) + ' |')
    print()
    for agent, gs in sorted(by_agent.items()):
        n = len(gs)
        kept = collections.Counter()
        played = collections.Counter()
        for g in gs:
            for x in draft:
                if cnt(g, 'cards.kept.' + x) > 0:
                    kept[x] += 1
                if cnt(g, 'cards.played.' + x) > 0:
                    played[x] += 1
        never_played = sorted(((100 * kept[x] / n, x) for x in draft
                               if played[x] == 0 and 100 * kept[x] / n > a.min), reverse=True)
        never_kept = sorted(names[x] for x in draft if kept[x] == 0)
        print(f"**{agent}** ({n} games): kept in over {a.min:g}% of games and never played: " +
              (', '.join(f"{names[x]} {p:.0f}%" for p, x in never_played) or 'none') +
              '. Never kept: ' + (', '.join(never_kept) or 'none') + '.')
        print()


if __name__ == '__main__':
    main()

"""167: the Unity and Discord decks in fear-card shape (Matt, design doc *Adversary system*, 2026-11-09).

    python experiments/results/167_unity_discord_r71/deck167.py [--check]

Rewrites content/alignment.json from the table below: each card one verb, scoped by a board condition (never reach,
never "one player"), its tiers widening it. Unity gains three prohibitions. Rerunnable: the table is the source.
Cards not in the table (Blight, Offensive, Firestorm) keep their text: they already name a board condition.
"""
import json
import os
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..', '..'))
PATH = os.path.join(ROOT, 'content', 'alignment.json')

W = {
    'ambassador': 'hex holding an ambassador',
    'by_ambassador': 'hex on or next to an ambassador',
    'by_player': 'hex on or next to an ambassador or a node',
    'near_player': 'hex within 2 of an ambassador or a node',
    'ambassador_city': 'city holding an ambassador',
    'ambassador_station': 'research station holding an ambassador',
    'ambassador_factory': 'factory holding an ambassador',
    'contested': 'hex holding both agents and an ambassador',
    'corrupted_city': 'corrupted city',
    'by_corrupted_city': 'hex on or next to a corrupted city',
    'corrupted': 'corrupted city, research station or factory',
    'owned': 'city, research station or factory a player owns',
}
PLURAL = {
    'ambassador': 'hexes holding an ambassador',
    'by_ambassador': 'hexes on or next to an ambassador',
    'by_player': 'hexes on or next to an ambassador or a node',
    'near_player': 'hexes within 2 of an ambassador or a node',
    'by_corrupted_city': 'hexes on or next to a corrupted city',
}


def per_site(where, mx, then):
    return {'op': 'per_site', 'where': where, 'max': mx, 'then': then}


def remove(pieces, count, per, where):
    return {'op': 'remove_where', 'pieces': pieces, 'count': count, 'per': per, 'where': where}


def prohibit(step, where):
    return {'op': 'prohibit', 'step': step, 'where': where}


def tier(text, *effects):
    return {'text': text, 'effects': list(effects)}


def income(resource, where, maxes, noun):
    return [tier(f"Every player's {noun} income rises by 1 for each {W[where]} (at most {m}), for the rest of the game.",
                 per_site(where, m, [{'op': 'team_income', 'resource': resource, 'amount': 1}])) for m in maxes]


VARIANT = next((a.split('=', 1)[1] for a in sys.argv if a.startswith('--variant=')), 'H')
# The screens' variants (dev seeds 36800-36847): `first` the first rewrite, A-D stronger agent removal and ambassador
# gains (C and D build on B), E (the deck in content) D with the counters and Sway on common sites.
CHAIN = {'first': ['first'], 'A': ['A', 'first'], 'B': ['B', 'first'], 'C': ['C', 'B', 'first'],
         'D': ['D', 'B', 'first'], 'E': ['E', 'D', 'B', 'first'], 'F': ['F', 'E', 'D', 'B', 'first'],
         'G': ['G', 'F', 'E', 'D', 'B', 'first'], 'H': ['H', 'G', 'F', 'E', 'D', 'B', 'first']}


def V(**by):
    """The table entry for this run's `--variant=`, by its chain (`final` is the first rewrite's entry)."""
    by.setdefault('first', by.get('final'))
    for v in CHAIN[VARIANT]:
        if v in by:
            return by[v]


def gain(resource, amount, where, maxes, noun, ambassadors=(0, 0, 0)):
    """Every player gains AMOUNT NOUN (and, by tier, AMBASSADORS more) for each WHERE: one verb, gain."""
    out = []
    for m, a in zip(maxes, ambassadors):
        then = [{'op': 'gain', 'resource': resource, 'amount': amount}]
        what = f"{amount} {noun}"
        if a:
            then.append({'op': 'gain', 'resource': 'ambassadors', 'amount': a})
            what += f" and {a} ambassador{'s' if a > 1 else ''}"
        out.append(tier(f"Every player gains {what} for each {W[where]} (at most {m}).", per_site(where, m, then)))
    return out


def counter(where, maxes):
    return [tier(f"Counter an active threat for each {W[where]} (at most {m}).",
                 per_site(where, m, [{'op': 'counter_threat'}])) for m in maxes]


def sway(where, maxes):
    return [tier(f"Sway 1 toward Unity for each {W[where]} (at most {m}).",
                 per_site(where, m, [{'op': 'sway', 'side': 'unity', 'amount': 1}])) for m in maxes]


def up_to(n, what):
    return f"up to {n} {what}" if n > 1 else f"1 {what[:-1] if what.endswith('s') else what}"


def each_hex(pieces, rows):
    """Remove up to N pieces from each hex where it holds: rows of (count, where)."""
    out = []
    for n, where in rows:
        what = {'agents': 'agents', 'troops': 'troops'}[pieces]
        out.append(tier(f"Remove {up_to(n, what)} from each {W[where]}.", remove(pieces, n, 'hex', where)))
    return out


def each_player(pieces, rows):
    out = []
    for n, where in rows:
        out.append(tier(f"Each player removes up to {n} {pieces} from one {W[where]}.", remove(pieces, n, 'player', where)))
    return out


def each_country(pieces, rows):
    out = []
    for n, where in rows:
        out.append(tier(f"In each country piece, remove up to {n} {pieces} from {PLURAL[where]}.",
                        remove(pieces, n, 'country', where)))
    return out


def deal(where, rows):
    """Discord: (max, tier) rows; tier None: the round's tier, in all; 2: a tier-2 threat in each country piece."""
    out = []
    for m, t in rows:
        if t is None:
            out.append(tier(f"Deal 1 extra threat for each {W[where]} (at most {m}).",
                            per_site(where, m, [{'op': 'deal_threats', 'count': 1}])))
        else:
            out.append(tier(f"For each {W[where]} (at most {m}), deal a tier-2 threat in each country piece.",
                            per_site(where, m, [{'op': 'deal_threats', 'count': 1, 'tier': 2}])))
    return out


SITE = {
    'ambassador': 'holding an ambassador',
    'by_ambassador': 'on or next to an ambassador',
    'by_player': 'on or next to an ambassador or a node',
    'near_player': 'within 2 of an ambassador or a node',
    'ambassador_city': 'holding an ambassador',
}
BAN = {
    'rally': "In the next adversary turn, no Rally lands on a site {s}: its share stays in the supply.",
    'deploy': "In the next adversary turn, no troop is deployed on a site {s}: it stays in the supply.",
    'corrupt': "In the next adversary turn, no site {s} corrupts, and nothing is pulled into it.",
}


def ban(step, wheres):
    out = []
    for where in wheres:
        text = BAN[step].format(s=SITE[where])
        if where == 'ambassador_city':
            text = text.replace('no site', 'no city')
        out.append(tier(text, prohibit(step, where)))
    return out


UNITY = {
    # Economy: resources counted by a board condition (Unity may give interesting resources).
    # H (after the confirm read missed d1 and d2): the conditions that seldom hold (a city, station or factory
    # holding an ambassador: 0.5 income, 0.2-0.5 free keeps a resolution) move to the sites the players own.
    'grassroots_funding': income('credits', V(final='ambassador_city', H='owned'), [2, 3, 4], 'credit'),
    'open_market': [tier(f"For each {W[V(final='ambassador_station', H='owned')]} (at most {m}), the next card any player "
                         f"keeps from the draft is free.",
                         per_site(V(final='ambassador_station', H='owned'), m, [{'op': 'free_purchases', 'count': 1}]))
                    for m in [2, 3, 4]],
    'think_tank': income('insight', 'owned', [1, 1, 2], 'insight'),
    # G (after the full read missed: lost games end at 6+ Unity tips with 1-2 cities left to Liberate): more
    # ambassadors, the Liberate fuel, from tier II on.
    'new_deal': gain('credits', 2, 'owned', [2, 3, 4], 'credits', V(final=(0, 0, 0), B=(0, 1, 2), G=(1, 2, 3))),
    'mutual_aid': gain('energy', 1, V(final='ambassador_factory', H='owned'), [1, 2, 3], 'energy',
                       V(final=(0, 0, 0), B=(0, 1, 2), G=(1, 2, 3))),
    'rebuilding_fund': gain('credits', 2, 'corrupted_city', [2, 3, 4], 'credits', V(final=(0, 0, 0), B=(0, 1, 2), G=(1, 2, 3))),
    'popular_mandate': V(final=gain('ambassadors', 1, 'corrupted_city', [2, 3, 4], 'ambassador'),
                         A=gain('ambassadors', 2, 'corrupted_city', [1, 2, 3], 'ambassadors'),
                         B=gain('ambassadors', 2, 'corrupted_city', [2, 3, 4], 'ambassadors'),
                         G=gain('ambassadors', 3, 'corrupted_city', [1, 2, 3], 'ambassadors')),
    'safe_corridors': V(final=gain('ambassadors', 1, 'owned', [2, 3, 4], 'ambassador'),
                        A=gain('ambassadors', 1, 'owned', [3, 4, 6], 'ambassador'),
                        B=gain('ambassadors', 1, 'owned', [3, 4, 6], 'ambassador'),
                        G=gain('ambassadors', 2, 'owned', [2, 3, 4], 'ambassadors')),
    # Threats countered, by the board.
    'ceasefire': V(final=counter('corrupted_city', [1, 2, 3]), E=counter('owned', [1, 2, 3])),
    'global_coalition': V(final=counter('ambassador_city', [1, 2, 3]), E=counter('corrupted_city', [1, 2, 3])),
    # Sway.
    'volunteer_corps': V(final=sway('ambassador_city', [2, 3, 4]), E=sway('owned', [2, 3, 4])),
    'citizens_assembly': V(final=sway('owned', [2, 3, 4]), E=sway('corrupted', [2, 3, 4])),
    # Agents off.
    'civil_defense': each_country('agents', V(final=[(2, 'by_player'), (3, 'by_player'), (3, 'near_player')],
                                              A=[(2, 'by_player'), (3, 'by_player'), (5, 'near_player')],
                                              B=[(2, 'by_player'), (3, 'by_player'), (5, 'near_player')],
                                              C=[(2, 'by_player'), (3, 'by_player'), (6, 'near_player')],
                                              D=[(2, 'by_corrupted_city'), (3, 'by_corrupted_city'), (6, 'near_player')])),
    'town_hall_accord': each_player('agents', V(final=[(3, 'by_player'), (4, 'by_player'), (4, 'near_player')],
                                                A=[(3, 'by_player'), (4, 'by_player'), (6, 'by_player')],
                                                B=[(3, 'by_player'), (4, 'by_player'), (6, 'by_player')],
                                                C=[(3, 'by_player'), (4, 'by_player'), (6, 'near_player')],
                                                D=[(3, 'by_corrupted_city'), (4, 'by_corrupted_city'), (6, 'near_player')])),
    'reconciliation_panel': each_hex('agents', V(final=[(1, 'by_player'), (2, 'by_player'), (2, 'near_player')],
                                                 A=[(1, 'by_player'), (2, 'by_player'), (3, 'by_player')],
                                                 B=[(1, 'by_player'), (2, 'by_player'), (3, 'by_player')],
                                                 C=[(1, 'by_player'), (2, 'by_player'), (3, 'near_player')],
                                                 D=[(1, 'by_corrupted_city'), (2, 'by_corrupted_city'), (3, 'near_player')])),
    # Troops off.
    'peacekeepers': each_hex('troops', [(1, 'by_ambassador'), (2, 'by_ambassador'), (2, 'by_player')]),
    'disarmament_talks': each_player('troops', [(2, 'by_ambassador'), (3, 'by_ambassador'), (4, 'by_ambassador')]),
    # Botnets, markers and cubes off, on the hexes the players hold.
    'decommission': [tier(f"Remove each botnet on a {W[w]}.", remove('botnets', 1, 'hex', w))
                     for w in ['ambassador', 'by_ambassador', 'by_player']],
    'network_cleanup': [tier(f"Clear {n} corruption marker{'s' if n > 1 else ''} on each {W[w]}, for its reward." if n > 1
                             else f"Clear the corruption marker on each {W[w]}, for its reward.", remove('markers', n, 'hex', w))
                        for n, w in [(1, 'by_ambassador'), (1, 'by_player'), (2, 'by_player')]],
    'reclamation': [tier(f"Take the adversary cube off each corrupted research station or factory on a {W[w]}.",
                         remove('cubes', 1, 'hex', w)) for w in ['ambassador', 'by_ambassador', 'by_player']],
}

# 167: the prohibitions (Matt: Rally, Corrupt, Deploy; tier III a wide condition, never a whole step off).
NEW_UNITY = [
    ('neighborhood_watch', 'Neighborhood Watch', ban('rally', ['ambassador', 'by_ambassador', 'by_player'])),
    ('rule_of_law', 'Rule of Law', ban('corrupt', ['by_ambassador', 'by_player', 'near_player'])),
    # The full read (40808-40855): tier I on or next to an ambassador fired in 3 of 24 games (Deploy lands on the
    # lowest-numbered factory, seldom by an ambassador): tier I takes II's condition, II and III the wide one.
    ('demilitarized_zone', 'Demilitarized Zone', ban('deploy', V(first=['by_ambassador', 'by_player', 'near_player'],
                                                               F=['by_player', 'near_player', 'near_player']))),
]

DISCORD = {
    'incursion': deal('corrupted_city', [(2, None), (4, None), (3, 2)]),
    'two_fronts': deal('corrupted', [(2, None), (2, 2), (3, 2)]),
    'propaganda': [tier(f"Every player loses 2 credits for each corrupted city (at most {m}).",
                        per_site('corrupted_city', m, [{'op': 'players_lose', 'resource': 'credits', 'amount': 2}]))
                   for m in [2, 3, 4]],
}


def fix_text(t):
    # "a hex ..." reads "a site on a hex ..." in the Rally and Corrupt prohibitions.
    return (t.replace("a site that is a hex holding", "a site holding")
             .replace("a site that is a hex on or next to", "a site on or next to")
             .replace("no troop is deployed on a hex holding", "no troop is deployed on a hex holding"))


def render(cards):
    out = ['[']
    for i, c in enumerate(cards):
        lines = [' {', f'  "id": {json.dumps(c["id"])},', f'  "name": {json.dumps(c["name"], ensure_ascii=False)},',
                 f'  "side": {json.dumps(c["side"])},']
        if c.get('draft'):
            lines.append('  "draft": true,')
        lines.append('  "tiers": [')
        for j, t in enumerate(c['tiers']):
            end = ',' if j < len(c['tiers']) - 1 else ''
            lines.append(f'   {{"text": {json.dumps(t["text"], ensure_ascii=False)},')
            lines.append(f'    "effects": {json.dumps(t["effects"], ensure_ascii=False)}}}{end}')
        lines.append('  ]')
        extra = {k: v for k, v in c.items() if k not in ('id', 'name', 'side', 'tiers', 'draft')}
        if extra:
            lines[-1] += ','
            for k, (key, v) in enumerate(extra.items()):
                lines.append(f'  {json.dumps(key)}: {json.dumps(v, ensure_ascii=False)}' + (',' if k < len(extra) - 1 else ''))
        lines.append(' }' + (',' if i < len(cards) - 1 else ''))
        out.extend(lines)
    out.append(']')
    return '\n'.join(out) + '\n'


def main():
    with open(PATH, encoding='utf-8') as f:
        cards = json.load(f)
    ids = {c['id'] for c in cards}
    for c in cards:
        table = UNITY if c['side'] == 'unity' else DISCORD
        if c['id'] in table:
            c['tiers'] = table[c['id']]
    # New Unity cards go after the last Unity card (card id order feeds play: a rules item).
    last = max(i for i, c in enumerate(cards) if c['side'] == 'unity')
    for k, (cid, name, tiers) in enumerate(NEW_UNITY):
        if cid in ids:
            next(c for c in cards if c['id'] == cid)['tiers'] = tiers
        else:
            cards.insert(last + 1 + k, {'id': cid, 'name': name, 'side': 'unity', 'draft': True, 'tiers': tiers})
    text = render(cards)
    json.loads(text)
    with open(PATH, encoding='utf-8') as f:
        old = f.read()
    if '--check' in sys.argv:
        print('changes' if old != text else 'same')
        sys.exit(1 if old != text else 0)
    # `--out=FILE`: a screen's variant deck, read by specs167.py `decks` (content stays as it is).
    out = next((a.split('=', 1)[1] for a in sys.argv if a.startswith('--out=')), PATH)
    with open(out, 'w', encoding='utf-8', newline='\n') as f:
        f.write(text)
    print(f'wrote {out}: {len(cards)} cards')


if __name__ == '__main__':
    main()

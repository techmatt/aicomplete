"""The one screen summary: a table of named columns over `aic-sim experiment` outputs.

    python scripts/cards/screen.py DIR [DIR ...] --columns variant,d,agent,n,win [--sources]
    python scripts/cards/screen.py DIR [DIR ...] --preset 041    # or tiers, 034, 028, 018, 016
    python scripts/cards/screen.py --gate DIR [DIR ...]
    python scripts/cards/screen.py --list

Each DIR holds results.json and games.jsonl. Without --columns or --preset it prints preset
041. --sources adds the node lost by source. --gate prints each check's gap to its own
agent (`x:...` against `x`), with games per check if results.json has a "gate" key, then the
unspent readout (what each seat held after its last action phase). --list prints the columns and presets. Columns on a counter no game has warn on stderr: that 0 is
not a measurement.
"""
import argparse
import collections
import functools
import json
import os
import re
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
import screens as s  # noqa: E402
from screens import ROUND, avg, cnt, hist, mean  # noqa: E402


def ais():
    return s.content('ais')


@functools.lru_cache(maxsize=None)
def tiers():
    """Each player card's tier."""
    return {c['id']: c['tier'] for c in s.content('cards')}


def tokens():
    """Nodes on an AI board's two tracks."""
    ai = ais()[0]
    return len(ai['economy_track']) + len(ai['play_track'])


def power_key(board, k):
    return f'ai.power.{board}.{k}'


def power_keys():
    """Every board's tally per power, as (board, index, name), in content order."""
    return [(b['id'], k, p['name']) for b in ais() for k, p in enumerate(b['powers'])]


def wins(gs):
    return [g for g in gs if g['won']]


def losses(gs):
    return [g for g in gs if not g['won']]


def ends(gs):
    return collections.Counter(g['reason'] for g in gs)


def reads(*keys):
    """Marks a column with the counters it reads, for `warn_gone`."""
    def mark(f):
        f.counters = keys
        return f
    return mark


def per_game(*keys):
    """Means over the cell of counters, joined by ` / `."""
    @reads(*keys)
    def f(c, gs):
        return ' / '.join(f'{mean(gs, lambda g: cnt(g, k)):.1f}' for k in keys)
    return f


def per_round(*keys):
    """Counters per round played, joined by ` / `."""
    @reads(*keys)
    def f(c, gs):
        played = sum(g['rounds'] for g in gs)
        return ' / '.join(f'{sum(cnt(g, k) for g in gs) / played if played else 0.0:.1f}' for k in keys)
    return f


def split(key):
    """A counter's mean over wins / over losses."""
    @reads(key)
    def f(c, gs):
        return f"{avg([cnt(g, key) for g in wins(gs)]):.1f} / {avg([cnt(g, key) for g in losses(gs)]):.1f}"
    return f


def wins_mix(c, gs):
    e = ends(gs)
    return f"{e['purge']} / {e['unity_deck']}"


def loss_mix(c, gs):
    e = ends(gs)
    n = len(losses(gs)) or 1
    return (f"{e['discord_deck']} / {e['node']} / {e['deck_empty']} "
            f"({100 * e['discord_deck'] // n} / {100 * e['node'] // n} / {100 * e['deck_empty'] // n})")


def loss_ends(c, gs):
    e = ends(gs)
    return f"{e['discord_deck']} / {e['node']} / {e['deck_empty']}"


def ends_text(c, gs):
    return s.ends_text(gs)


@reads('board.node_placed')
def tokens_left(c, gs):
    return f"{avg([tokens() - 1 - cnt(g, 'board.node_placed') / s.seats(c) for g in gs]):.1f}"


@reads(*(power_key(b, k) for b, k, _ in power_keys()))
def power_uses(c, gs):
    """Uses a game of each board's powers, in content order."""
    return ' / '.join(f"{mean(gs, lambda g: cnt(g, power_key(b, k))):.1f}" for b, k, _ in power_keys())


def by_tier(field):
    """A `cards` field of games.jsonl per game, by tier: tier 1 / tier 2. Offered against
    played says whether a card rework is one the agents ever see."""
    def f(c, gs):
        def tier(t):
            return mean(gs, lambda g: sum(n[field] for k, n in g['cards'].items() if tiers().get(k) == t))
        return f'{tier(1):.1f} / {tier(2):.1f}'
    return f


@reads('board.node_placed.r01', 'board.node_placed.r02')
def placed_by_round(c, gs):
    """Tokens on or off the board a seat has taken by rounds 6 and 10, the setup token in."""
    seats = s.seats(c)
    def by(r):
        reached = [g for g in gs if g['rounds'] >= r]
        n = [1 + sum(cnt(g, f'board.node_placed.r{k:02}') for k in range(1, r + 1)) / seats for g in reached]
        return f'{avg(n):.1f}' if n else '-'
    return f'{by(6)} / {by(10)}'


def peak_ambassadors(c, gs):
    peak = 0.0
    for g in gs:
        on = [v for k, v in g['counters'].items() if re.fullmatch(r'adversary\.ambassadors_r\d\d', k)]
        peak += max(on) if on else 0
    return f'{peak / len(gs):.0f}'


def total(prefix):
    return lambda c, gs: f"{mean(gs, lambda g: sum(v for k, v in g['counters'].items() if k.startswith(prefix) and not ROUND.search(k))):.1f}"


# Column name -> (header, function(cell, games)), from the tables of screen_016 to screen_034,
# where the counter still exists. A function gets the cell's results.json entry and its games.
COLUMNS = {
    'variant': ('variant', lambda c, gs: c['variant']),
    'd': ('d', lambda c, gs: c['difficulty']),
    'agent': ('agent', lambda c, gs: c['agent']),
    'n': ('n', lambda c, gs: len(gs)),
    'win': ('win %', lambda c, gs: f"{100 * len(wins(gs)) / len(gs):.0f}"),
    'rounds': ('rounds', lambda c, gs: f"{mean(gs, lambda g: g['rounds']):.1f}"),
    'win_rounds': ('win rounds', lambda c, gs: hist(g['rounds'] for g in wins(gs))),
    'loss_rounds': ('loss rounds', lambda c, gs: hist(g['rounds'] for g in losses(gs))),
    'ends': ('ends', ends_text),
    'wins_mix': ('wins: purge / Unity', wins_mix),
    'loss_mix': ('losses: Discord / node / cap (% of losses)', loss_mix),
    'wins_split': ('wins: purge / Unity deck', wins_mix),
    'loss_ends': ('losses: Discord / node / deck-out', loss_ends),
    'node_rounds': ('node-loss rounds', lambda c, gs: hist(g['rounds'] for g in gs if g['reason'] == 'node')),
    'tokens_left': ('tokens left a seat', tokens_left),
    'power_uses': ('uses a game of each power (content order)', power_uses),
    'markers': ('markers placed / removed', per_game('board.markers', 'board.markers_removed')),
    'markers_city': ('markers removed from cities', per_game('board.markers_removed_city')),
    'placed_by_round': ('tokens placed a seat by r6 / r10', placed_by_round),
    'delayed': ('threats delayed', per_game('threats.delayed')),
    'node_lost': ('node lost: wins / losses', split('board.node_lost')),
    'node_lost_mean': ('node lost', per_game('board.node_lost')),
    'node': ('node', per_game('board.node_placed')),
    'freed': ('freed', per_game('board.cities_freed')),
    'bases_united': ('bases united', per_game('board.bases_united')),
    'tips': ('Unity / Discord tips', per_game('alignment.tips_unity', 'alignment.tips_discord')),
    'arrivals': ('arrivals / removed a round', per_round('adversary.arrivals', 'adversary.ambassadors_removed')),
    'peak_ambassadors': ('peak ambassadors', peak_ambassadors),
    'botnets_peak': ('botnets peak', per_game('adversary.botnets_peak')),
    'botnets': ('botnets made / removed', per_game('adversary.botnets_made', 'adversary.botnets_removed')),
    'freed_corruptions': ('freed / corruptions', per_game('board.cities_freed', 'adversary.corruptions')),
    'projects': ('projects', per_game('adversary.projects_resolved')),
    'supply_short': ('supply short', per_game('adversary.supply_short')),
    'compute': ('compute made / blocked', per_game('adversary.compute_made', 'adversary.compute_blocked')),
    'left': ('left: corrupted / botnets / troops', lambda c, gs: ' / '.join(
        f"{mean(gs, lambda g: g['margin'].get(k, 0)):.1f}" for k in ('cities_corrupted_end', 'botnets_end', 'troops_end'))),
    'factories': ('factories P / A / N', per_game('board.factories_built.player', 'board.factories_built.adversary', 'board.factories_built.neutral')),
    'factories_end': ('owned at end', per_game('board.factories_end.player')),
    'chips': ('chips made / spent', per_game('board.chips_made', 'board.chips_spent')),
    'troop_strikes': ('steps / moves / strikes', per_game('board.troop_steps', 'board.troop_moves', 'board.troop_strikes')),
    'troop_destroyed': ('destroyed P / S / F', per_game('board.troop_destroyed.node', 'board.troop_destroyed.station', 'board.troop_destroyed.factory')),
    'kept': ('kept', total('cards.kept.')),
    'played': ('played', total('cards.played.')),
    'offered_tier': ('offered: tier 1 / 2', by_tier('offered')),
    'kept_tier': ('kept: tier 1 / 2', by_tier('kept')),
    'played_tier': ('played: tier 1 / 2', by_tier('played')),
    'tier2_dealt': ('tier-2 dealt', per_game('cards.dealt_tier2')),
    'powers': ('powers / tags', per_game('ai.powers_used', 'ai.tags_sacrificed')),
    'replaced': ('replaced', per_game('cards.replaced')),
    'token_flow': ('tokens + / -', per_game('cards.tokens_gained', 'cards.tokens_spent')),
    'insight': ('insight: keeps / plays', per_game('cards.insight_kept', 'cards.insight_played')),
}

PRESETS = {
    'tiers': ('variant,d,agent,n,win,offered_tier,kept_tier,played_tier', False),
    '041': ('variant,d,agent,n,win,win_rounds,loss_rounds,wins_mix,loss_mix,tokens_left,power_uses,markers,'
            'markers_city,placed_by_round,node_lost,freed,delayed', True),
    '034': ('variant,d,agent,n,win,win_rounds,loss_rounds,wins_mix,loss_mix,node_rounds,tokens_left,'
            'node_lost,freed', True),
    '028': ('variant,d,agent,n,win,win_rounds,loss_rounds,wins_split,loss_ends,tips,node_lost,freed,'
            'bases_united', True),
    '018': ('variant,d,agent,n,win,rounds,left,tips,ends,node,factories,factories_end,chips,botnets,compute,'
            'freed,played', False),
    '016': ('variant,d,agent,n,win,rounds,tips,ends,node,botnets,factories,factories_end,chips', False),
}


def table(columns, cells, games):
    print('| ' + ' | '.join(COLUMNS[k][0] for k in columns) + ' |')
    print('|' + '---|' * len(columns))
    key = lambda c: (cells[c]['variant'], cells[c]['difficulty'], cells[c]['agent'])
    for cell in sorted(games, key=key):
        print('| ' + ' | '.join(str(COLUMNS[k][1](cells[cell], games[cell])) for k in columns) + ' |')


def warn_gone(columns, games):
    """Counter columns no game has: their zeros are not measurements."""
    probe = s.checked_cnt(games)
    for k in columns:
        for key in getattr(COLUMNS[k][1], 'counters', ()):
            probe({'counters': {}}, key)


def gate_rows(d):
    with open(f'{d}/results.json', encoding='utf-8') as f:
        gate = json.load(f).get('gate')
    return gate['rows'] if gate else None


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('dirs', nargs='*')
    ap.add_argument('--columns', help='comma-separated column names')
    ap.add_argument('--preset', choices=sorted(PRESETS))
    ap.add_argument('--sources', action='store_true', help='add the node lost by source')
    ap.add_argument('--gate', nargs='*', metavar='DIR', help='the gate table of these DIRs')
    ap.add_argument('--list', action='store_true', help='print the columns and presets')
    a = ap.parse_args()
    if a.list:
        for k, (head, _) in COLUMNS.items():
            print(f'{k}: {head}')
        for k, (cols, src) in PRESETS.items():
            print(f"preset {k}: {cols}{' --sources' if src else ''}")
        return
    if a.columns and a.preset:
        ap.error('--columns or --preset, not both')
    dirs = a.dirs + (a.gate or [])
    if not dirs:
        ap.error('no DIR')
    spec, src = (a.columns, False) if a.columns else PRESETS[a.preset or '041']
    columns = [k.strip() for k in spec.split(',') if k.strip()]
    unknown = [k for k in columns if k not in COLUMNS]
    if unknown:
        ap.error(f"unknown column(s) {', '.join(unknown)}; --list names them")
    for d in dirs:
        cells, games = s.load(d)
        if a.gate is not None:
            s.gate(cells, games, gate_rows(d))
            s.unspent(cells, games)
        else:
            warn_gone(columns, games)
            table(columns, cells, games)
            if src or a.sources:
                s.sources(games)


if __name__ == '__main__':
    main()

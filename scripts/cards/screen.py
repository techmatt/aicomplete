"""The one screen summary: a table of named columns over `aic-sim experiment` outputs.

    python scripts/cards/screen.py DIR [DIR ...] --columns variant,d,agent,n,win [--sources]
    python scripts/cards/screen.py DIR [DIR ...] --preset 041
    python scripts/cards/screen.py --gate DIR [DIR ...]
    python scripts/cards/screen.py --list

Each DIR holds results.json and games.jsonl. Without --columns or --preset it prints preset
041. --sources adds the presence lost by source. --gate prints each check's gap to its own
agent (`x:...` against `x`), with games per check if results.json has a "gate" key. --list
prints the columns and presets. Columns on a counter no game has warn on stderr: that 0 is
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

CONTENT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'content', 'ais.json')


@functools.lru_cache(maxsize=None)
def tokens():
    """Presence tokens on an AI board's two tracks."""
    with open(CONTENT, encoding='utf-8') as f:
        ai = json.load(f)[0]
    return len(ai['presence']) + len(ai['presence2'])


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
    return (f"{e['discord_deck']} / {e['presence']} / {e['deck_empty']} "
            f"({100 * e['discord_deck'] // n} / {100 * e['presence'] // n} / {100 * e['deck_empty'] // n})")


def loss_ends(c, gs):
    e = ends(gs)
    return f"{e['discord_deck']} / {e['presence']} / {e['deck_empty']}"


def ends_text(c, gs):
    return s.ends_text(gs)


@reads('board.presence_placed')
def tokens_left(c, gs):
    return f"{avg([tokens() - 1 - cnt(g, 'board.presence_placed') / s.seats(c) for g in gs]):.1f}"


@reads(*(f'ai.growth.{b}.{k}' for b in ('builder', 'protector') for k in range(3)))
def growth(c, gs):
    """Share of growth picks by option (Expand, Develop, Consolidate), over both boards."""
    picks = [sum(cnt(g, f'ai.growth.{b}.{k}') for g in gs for b in ('builder', 'protector')) for k in range(3)]
    return ' / '.join(f'{100 * n / max(1, sum(picks)):.0f}' for n in picks)


@reads('board.presence_placed.r01', 'board.presence_placed.r02')
def placed_by_round(c, gs):
    """Tokens on or off the board a seat has taken by rounds 6 and 10, the setup token in."""
    seats = s.seats(c)
    def by(r):
        reached = [g for g in gs if g['rounds'] >= r]
        n = [1 + sum(cnt(g, f'board.presence_placed.r{k:02}') for k in range(1, r + 1)) / seats for g in reached]
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
    'loss_mix': ('losses: Discord / presence / cap (% of losses)', loss_mix),
    'wins_split': ('wins: purge / Unity deck', wins_mix),
    'loss_ends': ('losses: Discord / presence / deck-out', loss_ends),
    'presence_rounds': ('presence-loss rounds', lambda c, gs: hist(g['rounds'] for g in gs if g['reason'] == 'presence')),
    'tokens_left': ('tokens left a seat', tokens_left),
    'growth': ('growth: Expand / Develop / Consolidate %', growth),
    'markers': ('markers placed / removed', per_game('board.markers', 'board.markers_removed')),
    'markers_city': ('markers removed from cities', per_game('board.markers_removed_city')),
    'placed_by_round': ('tokens placed a seat by r6 / r10', placed_by_round),
    'delayed': ('threats delayed', per_game('threats.delayed')),
    'rich_uses': ('rich uses a game', per_game('board.std.presence_buy')),
    'presence_lost': ('presence lost: wins / losses', split('board.presence_lost')),
    'presence_lost_mean': ('presence lost', per_game('board.presence_lost')),
    'presence': ('presence', per_game('board.presence_placed')),
    'freed': ('freed', per_game('board.cities_freed')),
    'bases_united': ('bases united', per_game('board.bases_united')),
    'tips': ('Unity / Discord tips', per_game('alignment.tips_unity', 'alignment.tips_discord')),
    'arrivals': ('arrivals / removed a round', per_round('adversary.arrivals', 'adversary.ambassadors_removed')),
    'peak_ambassadors': ('peak ambassadors', peak_ambassadors),
    'nodes_peak': ('nodes peak', per_game('adversary.nodes_peak')),
    'nodes': ('nodes made / removed', per_game('adversary.nodes_made', 'adversary.nodes_removed')),
    'freed_corruptions': ('freed / corruptions', per_game('board.cities_freed', 'adversary.corruptions')),
    'projects': ('projects', per_game('adversary.projects_resolved')),
    'supply_short': ('supply short', per_game('adversary.supply_short')),
    'compute': ('compute made / blocked', per_game('adversary.compute_made', 'adversary.compute_blocked')),
    'left': ('left: corrupted / nodes / troops', lambda c, gs: ' / '.join(
        f"{mean(gs, lambda g: g['margin'].get(k, 0)):.1f}" for k in ('cities_corrupted_end', 'nodes_end', 'troops_end'))),
    'factories': ('factories P / A / N', per_game('board.factories_built.player', 'board.factories_built.adversary', 'board.factories_built.neutral')),
    'factories_end': ('owned at end', per_game('board.factories_end.player')),
    'chips': ('chips made / spent', per_game('board.chips_made', 'board.chips_spent')),
    'troop_strikes': ('steps / moves / strikes', per_game('board.troop_steps', 'board.troop_moves', 'board.troop_strikes')),
    'troop_destroyed': ('destroyed P / S / F', per_game('board.troop_destroyed.presence', 'board.troop_destroyed.station', 'board.troop_destroyed.factory')),
    'kept': ('kept', total('cards.kept.')),
    'played': ('played', total('cards.played.')),
    'tier2_dealt': ('tier-2 dealt', per_game('cards.dealt_tier2')),
    'powers': ('powers / tags', per_game('ai.powers_used', 'ai.tags_sacrificed')),
    'replaced': ('replaced', per_game('cards.replaced')),
    'token_flow': ('tokens + / -', per_game('cards.tokens_gained', 'cards.tokens_spent')),
    'insight': ('insight: keeps / plays', per_game('cards.insight_kept', 'cards.insight_played')),
}

PRESETS = {
    '041': ('variant,d,agent,n,win,win_rounds,loss_rounds,wins_mix,loss_mix,tokens_left,growth,markers,'
            'markers_city,placed_by_round,rich_uses,presence_lost,freed,delayed', True),
    '034': ('variant,d,agent,n,win,win_rounds,loss_rounds,wins_mix,loss_mix,presence_rounds,tokens_left,'
            'rich_uses,presence_lost,freed', True),
    '028': ('variant,d,agent,n,win,win_rounds,loss_rounds,wins_split,loss_ends,tips,presence_lost,freed,'
            'bases_united', True),
    '018': ('variant,d,agent,n,win,rounds,left,tips,ends,presence,factories,factories_end,chips,nodes,compute,'
            'freed,played', False),
    '016': ('variant,d,agent,n,win,rounds,tips,ends,presence,nodes,factories,factories_end,chips', False),
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
    sys.stdout.reconfigure(encoding='utf-8')
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('dirs', nargs='*')
    ap.add_argument('--columns', help='comma-separated column names')
    ap.add_argument('--preset', choices=sorted(PRESETS))
    ap.add_argument('--sources', action='store_true', help='add the presence lost by source')
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
        else:
            warn_gone(columns, games)
            table(columns, cells, games)
            if src or a.sources:
                s.sources(games)


if __name__ == '__main__':
    main()

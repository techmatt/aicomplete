"""137's read: one site model, bases as front piles and war every round (r56) against r55 on the same seeds.

    python experiments/results/137_sites_and_war/read137.py NEW_DIR OLD_DIR [--md OUT.md]

136's tables (win %, game length, how games end, tips, the paired gaps and the on/off gap between NEW's agents)
for both dirs, then per difficulty and agent: war a game (fights, factories destroyed, Discord from war, corp troops
made at Discord tips), claims and liberations a game by site type, blocks and armor a game, threats countered, and
135's tables (counter kinds, nodes lost by cause, peak holdings).
"""
import sys
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / '135_adversary_model'))
sys.path.insert(0, str(HERE.parent / '136_alignment_and_win'))
import read135  # noqa: E402
import read136  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

_load = read135.load


def load_relabeled(d):
    """A variant `<pair>_<lever>` becomes pair `<pair>` with agent `<agent> [<lever>]`, so the tables split levers
    and the paired gaps pair each lever against OLD's pair on the same seed."""
    head, games = _load(d)
    out = defaultdict(list)
    for (diff, var, agent), gs in games.items():
        pair, _, lever = var.partition('_')
        out[(diff, pair, f'{agent} [{lever}]' if lever else agent)].extend(gs)
    return head, dict(out)


read135.load = load_relabeled

KINDS = ('city', 'station', 'factory')

# (column, counters summed); a counter missing from a game counts 0.
COLUMNS = [
    ('war fights', ['board.war_fights']),
    ('factories destroyed in war', ['board.war_destroyed']),
    ('Discord from war', ['alignment.discord_from.war']),
    ('tip troop factories', ['board.tip_factory_troops']),
    ('corp troops made', ['board.units_built', 'board.units_extra', 'board.units_extra_now']),
] + [(f'claim {k}', [f'board.claimed.{k}']) for k in KINDS] + [
    ('aligned (r55 align)', ['board.cities_aligned']),
    ('owned st/fa (r55 claim)', ['board.stations_owned', 'board.factories_owned']),
] + [(f'liberate {k}', [f'board.liberated.{k}']) for k in KINDS] + [
    ('liberated cities (any route)', ['board.cities_freed']),
    ('r55 freed st/fa', ['board.stations_freed', 'board.factories_freed']),
    ('blocks', ['board.blocked.city_adversary', 'board.blocked.station_adversary', 'board.blocked.factory_adversary']),
    ('blocked by adversary', ['board.blocked.station_player', 'board.blocked.station_neutral',
                              'board.blocked.factory_player', 'board.blocked.factory_neutral']),
    ('armor used', ['board.ambassador_shield']),
    ('base ambassador used', ['board.base_ambassador_used', 'board.factory_ambassador_used']),
    ('station compute', ['board.station_compute']),
]


def sites_table(d, games, out):
    p = out.append
    p(f'### {d}: war, sites and armor a game')
    p('')
    p('| Difficulty | Agent | Games | ' + ' | '.join(c for c, _ in COLUMNS) + ' |')
    p('|---|---|---|' + '---|' * len(COLUMNS))
    by = defaultdict(list)
    for (diff, var, agent), gs in games.items():
        by[(diff, agent)].extend(gs)
    for (diff, agent), gs in sorted(by.items()):
        n = len(gs)
        cells = []
        for _, keys in COLUMNS:
            cells.append(f'{sum(read136.c(g, k) for g in gs for k in keys) / n:.2f}')
        p(f'| d{diff} | {agent} | {n} | ' + ' | '.join(cells) + ' |')
    p('')


def main():
    args = sys.argv[1:]
    md = None
    if '--md' in args:
        i = args.index('--md')
        md = args[i + 1]
        args = args[:i] + args[i + 2:]
    out = []
    new = read136.table(args[0], out)
    old = read136.table(args[1], out)
    read136.paired(new, old, out)
    sites_table(args[0], new, out)
    sites_table(args[1], old, out)
    if '--135' in sys.argv:
        # 135's tables (threats, counter kinds, nodes lost by cause, peak holdings), levers split as above.
        import contextlib
        import io
        for d in args[:2]:
            buf = io.StringIO()
            saved = sys.argv
            sys.argv = ['read135.py', d]
            with contextlib.redirect_stdout(buf):
                read135.main()
            sys.argv = saved
            out.append(buf.getvalue())
    text = '\n'.join(out) + '\n'
    print(text)
    if md:
        Path(md).write_text(text, encoding='utf-8', newline='\n')


if __name__ == '__main__':
    main()

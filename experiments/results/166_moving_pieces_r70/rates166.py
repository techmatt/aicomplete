"""166: per card, from games.jsonl lines (farm shards or a run's games.jsonl): games where it was drafted, kept,
played, and the share of drafted games it was played in (the done-when line: >= 15% for each move card).

    python rates166.py FILE_OR_DIR... [--cards a,b,c] [--min-drafted N] [--sort played_share]

A DIR is read for every *.jsonl in it (recursively). Each line is one game; `cards[id]` holds that game's
offered / drafted / kept / played counts, summed over seats.
"""
import json
import pathlib
import sys


def files(args):
    for a in args:
        p = pathlib.Path(a)
        if p.is_dir():
            yield from sorted(p.rglob('*.jsonl'))
        else:
            yield p


def main():
    args = sys.argv[1:]
    only, min_drafted, sort = None, 0, 'played_share'
    paths = []
    i = 0
    while i < len(args):
        if args[i] == '--cards':
            only = set(args[i + 1].split(','))
            i += 2
        elif args[i] == '--min-drafted':
            min_drafted = int(args[i + 1])
            i += 2
        elif args[i] == '--sort':
            sort = args[i + 1]
            i += 2
        else:
            paths.append(args[i])
            i += 1
    games = 0
    rows = {}
    for f in files(paths):
        with open(f, encoding='utf-8') as fh:
            for line in fh:
                line = line.strip()
                if not line.startswith('{') or '"cards"' not in line:
                    continue
                d = json.loads(line)
                games += 1
                for card, c in d.get('cards', {}).items():
                    if only and card not in only:
                        continue
                    r = rows.setdefault(card, [0, 0, 0, 0, 0])
                    drafted = c.get('drafted', 0) > 0
                    r[0] += drafted
                    r[1] += c.get('kept', 0) > 0
                    r[2] += c.get('played', 0) > 0
                    r[3] += drafted and c.get('played', 0) > 0
                    r[4] += c.get('played', 0)
    print(f'{games} games')
    print('| card | drafted | kept | played | played when drafted | share | plays a game |')
    print('|---|---|---|---|---|---|---|')
    out = []
    for card, (dr, kp, pl, dp, plays) in rows.items():
        if dr < min_drafted:
            continue
        share = dp / dr if dr else 0.0
        out.append((card, dr, kp, pl, dp, share, plays / max(games, 1)))
    key = {'played_share': lambda r: r[5], 'drafted': lambda r: -r[1], 'card': lambda r: r[0]}[sort]
    for card, dr, kp, pl, dp, share, pg in sorted(out, key=key):
        print(f'| {card} | {dr} | {kp} | {pl} | {dp} | {100 * share:.1f}% | {pg:.2f} |')


if __name__ == '__main__':
    main()

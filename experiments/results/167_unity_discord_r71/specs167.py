"""167: the item's farm specs, written into experiments/farm/.

    python experiments/results/167_unity_discord_r71/specs167.py split OLD_ALIGNMENT_JSON
    python experiments/results/167_unity_discord_r71/specs167.py onoff

`split`: the read's three pairs, each as `PAIR` (r71), `PAIR_olddisc` (r70's Discord tiers on the cards 167 changed)
and `PAIR_oldunity` (r70's Unity tiers, the three prohibitions blanked): which side of the rewrite moves the wins.
`onoff`: `PAIR` and `PAIR_<id>` for each Unity and Discord card, its three tiers blanked (a `per_site` of max 0:
nothing happens, the deck order and the Discord clock stand), for `scripts/cards/effect_stats.py --screen`.
"""
import json
import os
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..', '..'))
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
BLANK = [{'op': 'per_site', 'where': 'owned', 'max': 0, 'then': [{'op': 'counter_all'}]}]


def cards():
    with open(os.path.join(ROOT, 'content', 'alignment.json'), encoding='utf-8') as f:
        return json.load(f)


def blank(cid):
    return [f'alignment.{cid}.tiers.{t}.effects={json.dumps(BLANK)}' for t in range(3)]


def spec(name, note, levers):
    variants = []
    for pair, sets in PAIRS:
        for lever, extra in levers:
            variants.append({'name': pair if lever is None else f'{pair}_{lever}', 'set': sets + extra})
    return {'name': name, 'note': note, 'configs': ['configs/baseline.json'], 'variants': variants,
            'difficulties': [0, 1, 2, 3], 'agents': ['strategist'], 'games': 8, 'records': 0}


def main():
    kind = sys.argv[1]
    now = cards()
    if kind == 'split':
        with open(sys.argv[2], encoding='utf-8') as f:
            old = {c['id']: c for c in json.load(f)}
        disc, unity = [], []
        for c in now:
            o = old.get(c['id'])
            for t in range(3):
                if o is None:
                    if c['side'] == 'unity':
                        unity += blank(c['id'])[t:t + 1]
                    continue
                if o['tiers'][t]['effects'] != c['tiers'][t]['effects']:
                    line = f"alignment.{c['id']}.tiers.{t}.effects={json.dumps(o['tiers'][t]['effects'])}"
                    (disc if c['side'] == 'discord' else unity).append(line)
        s = spec('split167', '167: r71 against r70\'s Discord or Unity tiers (overlays), the read\'s pairs, d0-d3',
                 [(None, []), ('olddisc', disc), ('oldunity', unity)])
    elif kind == 'decks':
        # `decks NAME=FILE...`: each variant deck (deck167.py --variant=X --out=FILE) as overlays on the current one.
        levers = [(None, [])]
        for arg in sys.argv[2:]:
            name, path = arg.split('=', 1)
            with open(path, encoding='utf-8') as f:
                other = {c['id']: c for c in json.load(f)}
            lines = [f"alignment.{c['id']}.tiers.{t}={json.dumps(other[c['id']]['tiers'][t])}"
                     for c in now for t in range(3) if other[c['id']]['tiers'][t] != c['tiers'][t]]
            levers.append((name, lines))
        s = spec('decks167', '167: Unity deck variants as overlays on the current deck, the read\'s pairs, d0-d3', levers)
    elif kind == 'onoff' and len(sys.argv) > 2:
        # `onoff OLD_ALIGNMENT_JSON`: also `PAIR_old-<id>`, one r70 Unity card's tiers restored on the r71 deck:
        # which of r70's effects the rewrite's gap comes from.
        with open(sys.argv[2], encoding='utf-8') as f:
            old = {c['id']: c for c in json.load(f)}
        levers = [(None, [])] + [(c['id'], blank(c['id'])) for c in now]
        for c in now:
            o = old.get(c['id'])
            if c['side'] == 'unity' and o is not None:
                levers.append((f"old-{c['id']}", [f"alignment.{c['id']}.tiers.{t}={json.dumps(o['tiers'][t])}"
                                                  for t in range(3)]))
        s = spec('onoff167', '167: each Unity and Discord card on (base) and off (its tiers blanked), and each r70 '
                 'Unity card restored (old-<id>), the read\'s pairs, d0-d3; read by scripts/cards/effect_stats.py '
                 '--screen', levers)
    else:
        levers = [(None, [])] + [(c['id'], blank(c['id'])) for c in now]
        s = spec('onoff167', '167: each Unity and Discord card on (base) and off (its tiers blanked), the read\'s pairs, '
                 'd0-d3; read by scripts/cards/effect_stats.py --screen', levers)
    path = os.path.join(ROOT, 'experiments', 'farm', f"{s['name']}.json")
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(s, f, indent=2)
        f.write('\n')
    print(path, len(s['variants']), 'variants')


if __name__ == '__main__':
    main()

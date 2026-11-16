"""251: writes the identity specs built from others: six of p248j's cells (d0-d3), all of p248j at d2,
and the full oracle (the ladder's) at d2 (experiments/farm/i251-*.json)."""
import json


def write(name, s):
    with open(f'experiments/farm/{name}.json', 'w', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps(s, indent=1) + '\n')


s = json.load(open('experiments/farm/p248j.json', encoding='utf-8'))
keep = ['-node|bp', '-dispatch|br', '-claim|pr', 'b+node|bp', 'b+insight|br', 'b+claim|pr']
sub = dict(s, name='i251-p248j', variants=[v for v in s['variants'] if v['name'] in keep], games=32,
           first_seed=36800,
           note=("251: the identity check for a heavy spec: six of p248j's 57 variants (two of each pair, "
                 "both kinds), d0-d3, on the dev seeds 36800-36831. Before and after a speed change, "
                 "every game line must match."))
write('i251-p248j', sub)
d2 = dict(s, name='i251-p248j-d2', difficulties=[2], games=32, first_seed=36800,
          note=("251 (amendment 11454): the identity check for p248j: all 57 variants at d2, on the dev seeds "
                "36800-36831. Before and after a speed change, every game line must match."))
write('i251-p248j-d2', d2)
real = json.load(open('experiments/farm/i251-real.json', encoding='utf-8'))
full = dict(real, name='i251-full', agents=['oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3'],
            note=("251 (amendment 11454): the identity check for the full oracle (the ladder's), the "
                  "Builder-Protector pair, d2, on the dev seeds 36800-36831. Before and after a speed change, "
                  "every game line must match."))
write('i251-full', full)
print('written')

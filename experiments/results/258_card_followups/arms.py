"""258: the group probe's spec: the pass, and each group of its card edits restored to r117 by overlay.

    python experiments/results/258_card_followups/arms.py OLD_CARDS_JSON OUT_SPEC

OLD_CARDS_JSON is r117's content/cards.json (`git show 11f8539:content/cards.json`). A field the pass added that r117
lacks is set empty (`effects` []); Twin City Compact, new, is priced out (cost 99: a dead card in the draft, as 256's
arms did). The judge only, d3-d4, the fresh read's seeds (49356-49403): a diagnostic on spent seeds, not a read.
"""
import json
import os
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
GROUPS = {
    'nodes': ['mesh_repeater_network', 'orbital_relay_launcher', 'survey_drone_fleet'],
    'sinks': ['research_reactor', 'field_assay_team', 'antiproton_foundry'],
    'flags': ['dyson_swarm_tranche', 'exascale_training_cluster', 'citywide_microgrid', 'microgrid_lab_compact'],
}
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def restore(old, new, ids):
    sets = []
    for cid in ids:
        o, n = old[cid], new[cid]
        for k in sorted(set(o) | set(n)):
            if o.get(k) == n.get(k):
                continue
            v = o.get(k, [] if k in ('effects', 'passives') else None)
            assert v is not None, (cid, k)
            sets.append(f'cards.{cid}.{k}={json.dumps(v, separators=(",", ":"))}')
    return sets


def main():
    old_path, out = sys.argv[1:3]
    old = {c['id']: c for c in json.load(open(old_path, encoding='utf-8'))}
    new = {c['id']: c for c in json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))}
    arms = {'all': []}
    for g, ids in GROUPS.items():
        arms[f'no-{g}'] = restore(old, new, ids)
    arms['no-twin'] = ['cards.twin_city_compact.cost=99']
    variants = [{'name': f'{p}-{a}', 'set': ps + s} for p, ps in PAIRS for a, s in arms.items()]
    spec = {
        'name': 's258-groups',
        'note': '258: group probe on the fresh read\'s seeds (spent: a diagnostic), the judge, d3-d4; each group of the '
                'card edits restored to r117 by overlay (arms.py). Variant-major: pair-arm.',
        'configs': ['configs/baseline.json'],
        'variants': variants,
        'difficulties': [3, 4],
        'agents': ['strategist'],
        'games': 48,
        'first_seed': 49356,
        'records': 0,
    }
    with open(out, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(spec, f, indent=1)
        f.write('\n')
    for a, s in arms.items():
        print(a, len(s), 'sets')


if __name__ == '__main__':
    main()

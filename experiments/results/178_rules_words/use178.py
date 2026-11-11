"""178: how often r77's changed mechanics and touched cards come up, per agent, a game.

    python experiments/results/178_rules_words/use178.py DIR [DIR ...]

Reads each DIR's games.jsonl (an experiment's, or an assembled farm read). Prints, per agent, the mean a game of
the r77 counters (Vigil, land cleared, firewall held, strikes, conversions, liberate cards, the Protector landing)
and the plays of every card the rulings touched, then the win rate.
"""
import json
import os
import sys
from collections import defaultdict

COUNTERS = [
    ('vigil', 'board.vigil'),
    ('land cleared', 'board.land_cleared'),
    ('firewall held', 'adversary.firewall_held'),
    ('struck', 'effects.struck'),
    ('converted', 'effects.converted'),
    ('liberate card', 'board.liberated_card'),
    ('Protector landing', 'adversary.wall_troops'),
    ('standing spent', 'board.site_standing_spent'),
    ('land corruption made', 'adversary.corruptions.land'),
]
CARDS = [
    'security_contractor_hq', 'data_licensing_deal', 'kettle_operation', 'lure_network', 'honeypot_relay',
    'firewall_array', 'air_gap', 'hardpoint_lockdown', 'mass_defection', 'double_agents', 'open_source_uprising',
    'embassy_of_exiles', 'hostile_buyout', 'great_liberation', 'liberation_accords', 'power_to_the_people',
    'citywide_microgrid', 'nuclear_option', 'forensic_audit', 'root_access', 'popular_uprising',
]


sys.path.insert(0, os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts')))
from screens import cells_of  # noqa: E402


def main():
    sums = defaultdict(lambda: defaultdict(float))
    games = defaultdict(int)
    wins = defaultdict(int)
    for d in sys.argv[1:]:
        with open(os.path.join(d, 'games.jsonl'), encoding='utf-8') as f:
            cells = cells_of(json.loads(f.readline())['spec'])
            for line in f:
                g = json.loads(line)
                a = cells[g['cell']]['agent']
                games[a] += 1
                wins[a] += 1 if g['won'] else 0
                c = g.get('counters', {})
                for name, key in COUNTERS:
                    sums[a][name] += c.get(key, 0)
                for card in CARDS:
                    sums[a][card] += c.get(f'cards.played.{card}', 0)
    for a in sorted(games):
        n = games[a]
        print(f'## {a} ({n} games, win {100 * wins[a] / n:.1f}%)')
        print('  ' + ', '.join(f'{name} {sums[a][name] / n:.2f}' for name, _ in COUNTERS))
        played = [(card, sums[a][card] / n) for card in CARDS]
        print('  plays a game: ' + ', '.join(f'{c} {v:.2f}' for c, v in played))


if __name__ == '__main__':
    main()

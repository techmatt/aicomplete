"""136: Envoy and the Unity-for-Discord choice, on and off, on the same seeds.

    python experiments/results/136_alignment_and_win/choices_read.py READ_DIR CHOICES_DIR

READ_DIR is read-136 (r55, `strategist`: both on); CHOICES_DIR is read-136-choices (variants `<pair>_noenvoy`,
`<pair>_nohold`). Prints, per difficulty and arm, win % and the paired gap off - on, rounds, node and Discord-deck
losses, and Envoy / held-off plays a game.
"""
import math
import statistics
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / '135_adversary_model'))
import read135  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

_, on = read135.load(sys.argv[1])
_, off = read135.load(sys.argv[2])
base = {(d, v, g['seed']): g for (d, v, a), gs in on.items() if a == 'strategist' for g in gs}
arms = defaultdict(list)
for (d, v, a), gs in off.items():
    pair, arm = v.split('_', 1)
    for g in gs:
        arms[(d, arm)].append((g, base[(d, pair, g['seed'])]))
print('| Difficulty | Off | Win % off / on | Gap off - on | Rounds off / on | Node losses off / on | '
      'Discord-deck losses off / on | Envoy a game off / on | Unity held off a game off / on |')
print('|---|---|---|---|---|---|---|---|---|')
for (d, arm), ps in sorted(arms.items()):
    n = len(ps)
    w = lambda i: sum(p[i]['won'] for p in ps) / n  # noqa: E731
    gap = [int(a['won']) - int(b['won']) for a, b in ps]
    se = statistics.pstdev(gap) / math.sqrt(n) * 100
    r = lambda i: sum(p[i]['rounds'] for p in ps) / n  # noqa: E731
    e = lambda i, why: sum(p[i]['reason'] == why for p in ps)  # noqa: E731
    c = lambda i, k: sum(p[i]['counters'].get(k, 0) for p in ps) / n  # noqa: E731
    print(f'| d{d} | {arm[2:]} | {100 * w(0):.1f} / {100 * w(1):.1f} | {100 * statistics.mean(gap):+.1f} ±{se:.1f} | '
          f'{r(0):.2f} / {r(1):.2f} | {e(0, "node")} / {e(1, "node")} | {e(0, "discord_deck")} / {e(1, "discord_deck")} | '
          f'{c(0, "board.std.envoy"):.2f} / {c(1, "board.std.envoy"):.2f} | '
          f'{c(0, "alignment.unity_held_off"):.2f} / {c(1, "alignment.unity_held_off"):.2f} |')

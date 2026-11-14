"""227 (ruling 1, "code identifiers may follow where cheap"): identifier renames for Fallout and the corrupted
factory. Word-bounded; skips experiments/ and the rules history.

    python experiments/results/227_fallout/rename_ids.py [--dry]
"""
import re
import subprocess
import sys

PAIRS = [
    (r'\bLAND_CORRUPTION\b', 'FALLOUT'),
    (r"(['\"])land_corruption(['\"])", r'\1fallout\2'),
    (r'\bland_corruption\b(?=\s*:)', 'fallout'),
    (r'\blift_land_corruption\b', 'lift_fallout'),
    (r'\blift_land\b', 'lift_fallout'),
    (r'\bLiftLand\b', 'LiftFallout'),
    (r'\bRemovable::Land\b', 'Removable::Fallout'),
    (r'"pieces": "land"', '"pieces": "fallout"'),
    (r'\bSeizeFactory\b', 'CorruptFactory'),
    (r'\bseize_factory\b', 'corrupt_factory'),
    (r'\bFACILITIES_SEIZED\b', 'FACILITIES_CORRUPTED'),
    (r'\bfactories_seized\b', 'factories_corrupted'),
    (r'\blift_land_takes_a_token_in_reach\b', 'lift_fallout_takes_a_token_in_reach'),
    (r'\bland_corruption_blocks_a_node\b', 'fallout_blocks_a_node'),
    (r'\blift_takes_land_corruption\b', 'lift_takes_fallout'),
    (r'\ba_burn_destroys_your_own_factory_and_leaves_land_corruption\b',
     'a_burn_destroys_your_own_factory_and_leaves_fallout'),
]
EXT = ('.rs', '.json', '.md', '.ts', '.mjs', '.py', '.html')
SKIP = ('experiments/', 'docs/rules-history.md', 'review/', 'out/')


def main():
    dry = '--dry' in sys.argv
    files = subprocess.run(['git', 'ls-files'], capture_output=True, text=True, encoding='utf-8').stdout.split('\n')
    for f in files:
        if not f.endswith(EXT) or f.startswith(SKIP):
            continue
        try:
            text = open(f, encoding='utf-8', newline='').read()
        except (UnicodeDecodeError, FileNotFoundError):
            continue
        new = text
        n = 0
        for a, b in PAIRS:
            new, k = re.subn(a, b, new)
            n += k
        if n:
            print(f'{f}: {n}')
            if not dry:
                open(f, 'w', encoding='utf-8', newline='').write(new)


if __name__ == '__main__':
    main()

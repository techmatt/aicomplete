"""227 (ruling 1): display renames across the tracked tree: land corruption is Fallout, a seized facility is a
corrupted one. Code identifiers (`LAND_CORRUPTION`, `lift_land`) are renamed separately. Skips experiments/ and
the rules history (a record of its day).

    python experiments/results/227_fallout/rename.py [--dry]
"""
import subprocess
import sys

PAIRS = [
    ('land-corruption tokens', 'Fallout tokens'),
    ('land-corruption token', 'Fallout token'),
    ('Lift land corruption', 'Lift Fallout'),
    ('lift land corruption', 'lift Fallout'),
    ('Land corruption', 'Fallout'),
    ('land corruption', 'Fallout'),
    ('land-corruption', 'Fallout'),
    ('facilities destroyed or seized', 'facilities destroyed or corrupted'),
    ('facilities seized', 'facilities corrupted'),
    ('seizes the factory', 'corrupts the factory'),
]
EXT = ('.rs', '.json', '.md', '.ts', '.mjs', '.py', '.html', '.css', '.txt')
SKIP = ('experiments/', 'docs/rules-history.md', 'review/', 'out/')


def main():
    dry = '--dry' in sys.argv
    files = subprocess.run(['git', 'ls-files'], capture_output=True, text=True, encoding='utf-8').stdout.split('\n')
    total = 0
    for f in files:
        if not f.endswith(EXT) or f.startswith(SKIP):
            continue
        try:
            text = open(f, encoding='utf-8', newline='').read()
        except (UnicodeDecodeError, FileNotFoundError):
            continue
        new = text
        for a, b in PAIRS:
            new = new.replace(a, b)
        if new != text:
            n = sum(text.count(a) for a, _ in PAIRS)
            total += n
            print(f'{f}: {n}')
            if not dry:
                open(f, 'w', encoding='utf-8', newline='').write(new)
    print('total', total)


if __name__ == '__main__':
    main()

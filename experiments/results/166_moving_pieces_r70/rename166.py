"""166 (Matt, 2026-11-09): printed text calls the adversary's pieces agents. Rewrites "adversary ambassador(s)"
in the printed and rules text (content, drafts, faces, the generated-doc sources, docs/rules, the player aid, the
card viewer); code names stay. Rerunnable: a file already renamed changes nothing.

    python experiments/results/166_moving_pieces_r70/rename166.py [--check]
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[3]
FILES = [
    'content/alignment.json', 'content/threats.json', 'content/adversary.json', 'content/projects.json',
    'content/cards.json', 'content/ais.json',
    'crates/aic-core/src/face.rs', 'crates/aic-core/src/op.rs',
    'docs/player-aid.md', 'docs/card-guidelines.md', 'docs/card-pricing.md', 'docs/principles.md',
]
GLOBS = ['crates/aic-core/src/modules/**/*.rs', 'crates/aic-core/src/tests/*.rs', 'crates/aic-core/src/view.rs',
         'drafts/adversary/*.json', 'drafts/adversary/*.md', 'docs/rules/*.md', 'crates/aic-sim/src/docs/*.rs',
         'web/card/*.js', 'web/src/*.ts', 'web/scripts/*.mjs']

# Order matters: longest first. Articles: "an adversary ambassador" -> "an agent". A line break may split the words.
PAIRS = [
    (r"\bAn adversary\s+ambassador\b", "An agent"),
    (r"\ban adversary\s+ambassador\b", "an agent"),
    (r"\bAdversary\s+ambassadors\b", "Agents"),
    (r"\badversary\s+ambassadors\b", "agents"),
    (r"\bAdversary\s+ambassador\b", "Agent"),
    (r"\badversary\s+ambassador\b", "agent"),
    (r"\bthe adversary's\s+ambassadors\b", "the agents"),
    (r"\bthe adversary's\s+ambassador\b", "the agent"),
]


def targets():
    seen = set()
    for f in FILES:
        p = ROOT / f
        if p.exists():
            seen.add(p)
    for g in GLOBS:
        for p in ROOT.glob(g):
            seen.add(p)
    return sorted(seen)


def main():
    check = '--check' in sys.argv
    changed = 0
    for p in targets():
        s = p.read_text(encoding='utf-8')
        t = s
        for a, b in PAIRS:
            t = re.sub(a, b, t)
        if t != s:
            changed += 1
            n = sum(len(re.findall(a, s)) for a, _ in PAIRS)
            print(f'{p.relative_to(ROOT).as_posix()}: {n}')
            if not check:
                p.write_text(t, encoding='utf-8', newline='\n')
    print(f'{changed} files {"would change" if check else "changed"}')
    sys.exit(1 if check and changed else 0)


if __name__ == '__main__':
    main()

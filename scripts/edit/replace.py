"""Mechanical multi-site edits, all or nothing (281): for the edits a one-off replace script used to make.

    python scripts/edit/replace.py SPEC.json [--dry-run]
    python scripts/edit/replace.py --word OLD NEW PATH... [--dry-run]

SPEC.json is a list of `{"file": PATH, "old": TEXT, "new": TEXT, "count": N}`: each exact `old` is replaced by
`new`, and `old` must occur exactly `count` times (default 1; "all": at least once, every one replaced). Entries on
one file apply in order, each to the text the last one left. Any missing or miscounted entry is reported and
nothing is written (exit 1). Files are read and written as UTF-8 bytes as they are: LF stays LF.

`--word` renames OLD to NEW as a whole word (letters, digits and `_` on neither side) in every tracked file under
the PATHs that `git grep -lw` finds, and reports the count a file. `--dry-run` reports without writing.
Paths are relative to the current directory (run from the repo root).
"""
import json
import re
import subprocess
import sys


def apply(spec):
    """{file: new text} for the spec, or the list of failures."""
    texts, fails = {}, []
    for i, e in enumerate(spec):
        f = e['file']
        if f not in texts:
            try:
                with open(f, encoding='utf-8', newline='') as h:
                    texts[f] = h.read()
            except OSError as err:
                fails.append(f'#{i} {f}: {err.strerror}')
                texts[f] = None
        if texts[f] is None:
            continue
        want = e.get('count', 1)
        n = texts[f].count(e['old'])
        if n == 0 or (want != 'all' and n != want):
            fails.append(f"#{i} {f}: found {n}, want {want}: {e['old'][:80]!r}")
            continue
        texts[f] = texts[f].replace(e['old'], e['new'])
    return texts, fails


def word(old, new, paths):
    """{file: new text} renaming `old` as a whole word in the tracked files under `paths`."""
    found = subprocess.run(['git', 'grep', '-lw', '-F', old, '--', *paths], capture_output=True, text=True,
                           encoding='utf-8')
    if found.returncode not in (0, 1):
        sys.exit(found.stderr.strip())
    pat = re.compile(r'(?<![A-Za-z0-9_])' + re.escape(old) + r'(?![A-Za-z0-9_])')
    texts = {}
    for f in found.stdout.splitlines():
        with open(f, encoding='utf-8', newline='') as h:
            text, n = pat.subn(new.replace('\\', '\\\\'), h.read())
        if n:
            texts[f] = text
            print(f'{f}: {n}')
    return texts


def main(argv):
    dry = '--dry-run' in argv
    argv = [a for a in argv if a != '--dry-run']
    if argv[:1] == ['--word'] and len(argv) >= 4:
        texts = word(argv[1], argv[2], argv[3:])
    elif len(argv) == 1:
        with open(argv[0], encoding='utf-8') as h:
            texts, fails = apply(json.load(h))
        for x in fails:
            print('FAIL', x)
        if fails:
            print(f'{len(fails)} failed: nothing written')
            return 1
    else:
        sys.exit(__doc__)
    if not dry:
        for f, text in texts.items():
            with open(f, 'w', encoding='utf-8', newline='') as h:
                h.write(text)
    print(f"{len(texts)} file(s) {'would change' if dry else 'written'}")
    return 0


if __name__ == '__main__':
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    sys.exit(main(sys.argv[1:]))

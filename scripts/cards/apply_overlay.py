"""Write content overlays into content/*.json in place (123).

    python scripts/cards/apply_overlay.py FILE... [--set ENTRY]... [--check]

Each FILE holds overlay entries, one a line: `cards.<id>.<path>=<JSON>`, the keys `--set`
takes (a line starting `#` is a comment; blank lines are skipped). The paths follow `apply_entry` in
`crates/aic-core/src/content.rs`: `<table>.<id>.<path>`, the table one of corps (the map's
countries, by lowercase name), cards, threats, adversary, ais, alignment, projects; `id` may
be `*` (every row); `<table>.<id>=<JSON object>` with no path writes a whole row (125: a new card
is appended, an existing row replaced); the path walks object fields and list indexes, its last step may add a
field; the value is JSON, or a bare word taken as a string. An entry setting a value already
there changes nothing, so a card pass keeps its edits as one overlay file and reruns it.

`--check` prints each entry that would change something and writes nothing (exit 1 if any
would). A file whose text isn't what `json.dumps(indent=1)` writes (projects.json's one-line
effect lists) is refused: edit it by hand. Afterwards run `python scripts/regen.py`, which
builds, regenerates the docs and validates the content as the engine parses it.
"""
import argparse
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
# content.rs OVERLAY_TABLES: table -> content file.
TABLES = {
    'corps': 'map.json',
    'cards': 'cards.json',
    'threats': 'threats.json',
    'adversary': 'adversary.json',
    'ais': 'ais.json',
    'alignment': 'alignment.json',
    'projects': 'projects.json',
}


def dump(data):
    return json.dumps(data, indent=1, ensure_ascii=True) + '\n'


def parse_value(text):
    try:
        return json.loads(text)
    except ValueError:
        return text


def apply_entry(files, entry):
    """Apply one entry to the loaded files; returns True if it changed a value."""
    key, sep, value = entry.partition('=')
    parts = key.strip().split('.')
    if sep and len(parts) == 2 and parts[0] in TABLES and parts[0] != 'corps' and parts[1] != '*':
        return put_row(files, parts[0], parts[1], parse_value(value.strip()), entry)
    if not sep or parts[0] not in TABLES or len(parts) < 3:
        raise ValueError(f'{entry!r}: expected <table>.<id>.<path>=<value>')
    table, ident, path = parts[0], parts[1], parts[2:]
    if path[0] == 'id':
        raise ValueError(f"{entry!r}: an id can't be overlaid")
    value = parse_value(value.strip())
    data = files[TABLES[table]]
    rows = data.get('countries') if table == 'corps' else data
    if not isinstance(rows, list):
        raise ValueError(f'{entry!r}: not a list')
    hit = changed = False
    for row in rows:
        name = (row.get('name') or '').lower() if table == 'corps' else row.get('id')
        if ident != '*' and name != ident:
            continue
        hit = True
        at = row
        for step in path[:-1]:
            if isinstance(at, list) and step.isdigit() and int(step) < len(at):
                at = at[int(step)]
            elif isinstance(at, dict) and step in at:
                at = at[step]
            else:
                raise ValueError(f'{entry!r}: no {step!r} on the way')
        last = path[-1]
        if isinstance(at, list):
            if not (last.isdigit() and int(last) < len(at)):
                raise ValueError(f'{entry!r}: no list entry {last!r}')
            i = int(last)
            changed |= at[i] != value or type(at[i]) is not type(value)
            at[i] = value
        elif isinstance(at, dict):
            changed |= last not in at or at[last] != value or type(at[last]) is not type(value)
            at[last] = value
        else:
            raise ValueError(f'{entry!r}: {last!r} is inside a plain value')
    if not hit:
        raise ValueError(f'{entry!r}: no id {ident!r} in {table}')
    return changed


def put_row(files, table, ident, row, entry):
    """125: `<table>.<id>=<JSON object>` writes a whole row: a new id is appended
    (a new card), an existing one replaced. Returns True if it changed anything."""
    if not isinstance(row, dict) or row.get('id', ident) != ident:
        raise ValueError(f'{entry!r}: a whole row is a JSON object whose id, if given, is {ident!r}')
    row = {'id': ident, **{k: v for k, v in row.items() if k != 'id'}}
    rows = files[TABLES[table]]
    for i, old in enumerate(rows):
        if old.get('id') == ident:
            rows[i] = row
            return old != row
    rows.append(row)
    return True


def read_entries(paths, sets):
    out = []
    for p in paths:
        with open(p, encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith('#'):
                    out.append(line)
    return out + list(sets)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    ap.add_argument('files', nargs='*', help='overlay files, one entry a line')
    ap.add_argument('--set', action='append', default=[], help='one more entry')
    ap.add_argument('--check', action='store_true', help='print what would change, write nothing')
    args = ap.parse_args()
    entries = read_entries(args.files, args.set)
    if not entries:
        ap.error('no entries')
    files, texts = {}, {}
    for name in sorted({TABLES[e.split('.', 1)[0]] for e in entries if e.split('.', 1)[0] in TABLES}):
        path = os.path.join(ROOT, 'content', name)
        with open(path, encoding='utf-8', newline='') as f:
            texts[name] = f.read()
        files[name] = json.loads(texts[name])
    touched, failed = set(), False
    for e in entries:
        table = e.split('.', 1)[0]
        try:
            if apply_entry(files, e):
                touched.add(TABLES[table])
                if args.check:
                    print('would change:', e)
        except (ValueError, KeyError) as err:
            print('error:', err, file=sys.stderr)
            failed = True
    if failed:
        sys.exit(2)
    for name in sorted(touched):
        if dump(json.loads(texts[name])) != texts[name]:
            print(f'error: content/{name} is not in json.dumps(indent=1) form: edit it by hand',
                  file=sys.stderr)
            sys.exit(2)
    if args.check:
        print(f'{len(entries)} entries, {"would change " + ", ".join(sorted(touched)) if touched else "nothing to change"}')
        sys.exit(1 if touched else 0)
    for name in sorted(touched):
        with open(os.path.join(ROOT, 'content', name), 'w', encoding='utf-8', newline='\n') as f:
            f.write(dump(files[name]))
    print(f'{len(entries)} entries; wrote {", ".join(sorted(touched)) or "nothing"}')


if __name__ == '__main__':
    main()

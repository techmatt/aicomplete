"""Write content overlays into content/*.json in place (123).

    python scripts/cards/apply_overlay.py FILE... [--set ENTRY]... [--check] [--from REV]

Each FILE holds overlay entries, one a line: `cards.<id>.<path>=<JSON>`, the keys `--set`
takes (a line starting `#` is a comment; blank lines are skipped). The paths follow `apply_entry` in
`crates/aic-core/src/content.rs`: `<table>.<id>.<path>`, the table one of corps (the map's
countries, by lowercase name), cards, threats, adversary, ais, alignment, projects; `id` may
be `*` (every row); `<table>.<id>=<JSON object>` with no path writes a whole row (125: a new card
is appended, an existing row replaced; this form is apply_overlay's own, the engine's `--set` has
none). Nothing deletes a row: to drop a card, take it out of the deck in configs/baseline.json,
then edit content/cards.json by hand and run
`python tools/card-art/scenes_io.py release CARD`. The path walks object fields and list indexes, its last step may add a
field, or be `+` to append the value to the list it reaches (`cards.<id>.passives.+={"op":"..."}`, as the engine's
`--set`; `+` elsewhere, or on what isn't a list, is refused; a list already holding an equal value is left as it
is, so a rerun appends nothing); the value is JSON (`null` drops an object's field; one no row of the table has and content.rs declares nowhere is refused as a
misspelling), or a bare word taken as a string. An entry setting a value already
there changes nothing, so a card pass keeps its edits as one overlay file and reruns it.

The overlay only sets: a changed overlay rerun over content it already edited leaves the values it no longer
sets. `--from REV` applies it over each touched file as `git show REV:content/<file>` holds it and writes the
result wherever it differs from the working tree; before that it warns of each `<table>.<id>.<field>` where the
working tree differs from REV and no entry sets that field (a by-hand edit the write drops).

`--check` prints each entry that would change something and writes nothing (exit 1 if any
would). A file whose text isn't what `json.dumps(indent=1)` writes (today only alignment.json, its
tiers laid out by hand; 157 wrote experiments/results/157_strategist_purge_r68/apply_tiers.py for
it) is refused: edit it by hand. A `threats.`, `adversary.` or `projects.` entry is refused too:
those files are written from drafts/adversary/ by scripts/content/load_adversary.py, so an overlay
there is lost on the next load and fails regen.py's `load_adversary.py --check`; edit the drafts
instead (the engine's `--set` still takes them for a one-off run). Afterwards run `python scripts/regen.py`, which
builds, regenerates the docs and validates the content as the engine parses it.
"""
import argparse
import json
import os
import re
import subprocess
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
# Written from drafts/adversary/ by scripts/content/load_adversary.py: an overlay there is lost.
DRAFTED = ('threats', 'adversary', 'projects')


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
    if parts[0] in DRAFTED:
        raise ValueError(f'{entry!r}: content/{TABLES[parts[0]]} is written from drafts/adversary/ by '
                         'scripts/content/load_adversary.py, so an overlay there is lost: edit the drafts')
    if sep and len(parts) == 2 and parts[0] in TABLES and parts[0] != 'corps' and parts[1] not in ('*', '+'):
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
            if step == '+':
                raise ValueError(f"{entry!r}: '+' appends, so it is only the last step")
            if isinstance(at, list) and step.isdigit() and int(step) < len(at):
                at = at[int(step)]
            elif isinstance(at, dict) and step in at:
                at = at[step]
            else:
                raise ValueError(f'{entry!r}: no {step!r} on the way')
        last = path[-1]
        if last == '+':
            # 229 (Matt): `+` appends to the list it names, as the engine's `--set` does. A list already
            # holding an equal value is left alone, so a rerun of the overlay changes nothing.
            if not isinstance(at, list):
                raise ValueError(f"{entry!r}: '+' appends to a list, and this is not one")
            if not any(x == value and type(x) is type(value) for x in at):
                at.append(value)
                changed = True
        elif isinstance(at, list):
            if not (last.isdigit() and int(last) < len(at)):
                raise ValueError(f'{entry!r}: no list entry {last!r}')
            i = int(last)
            changed |= at[i] != value or type(at[i]) is not type(value)
            at[i] = value
        elif isinstance(at, dict) and value is None:
            # 214: `null` on an object field drops the field (as the engine's overlay reads an absent field)
            # 266 (D117): a misspelled field dropped nothing, silently (the engine's overlay refuses it). A field
            # no row of the table holds and no content struct declares is a misspelling; a real one the row
            # lacks (dropped by an earlier run) is nothing to change, so the overlay still reruns.
            if last not in at and last not in field_names(rows) | schema_fields():
                raise ValueError(f'{entry!r}: no field {last!r} to drop: no row of {table} has one, and '
                                 'crates/aic-core/src/content.rs declares none')
            changed |= last in at
            at.pop(last, None)
        elif isinstance(at, dict):
            changed |= last not in at or at[last] != value or type(at[last]) is not type(value)
            at[last] = value
        else:
            raise ValueError(f'{entry!r}: {last!r} is inside a plain value')
    if not hit:
        raise ValueError(f'{entry!r}: no id {ident!r} in {table}')
    return changed


def schema_fields():
    """The field names content.rs's structs declare (`pub name:`): what the engine's parse takes."""
    path = os.path.join(ROOT, 'crates', 'aic-core', 'src', 'content.rs')
    with open(path, encoding='utf-8') as f:
        return set(re.findall(r'^\s*pub (?:r#)?([a-z_][a-z0-9_]*)\s*:', f.read(), re.M))


def field_names(node, out=None):
    """Every object key anywhere in NODE (a table's rows)."""
    out = set() if out is None else out
    if isinstance(node, dict):
        out.update(node)
        for v in node.values():
            field_names(v, out)
    elif isinstance(node, list):
        for v in node:
            field_names(v, out)
    return out


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


def git_text(rev, name):
    """content/NAME's text at revision REV (--from)."""
    p = subprocess.run(['git', 'show', f'{rev}:content/{name}'], cwd=ROOT, capture_output=True)
    if p.returncode != 0:
        raise ValueError(f'--from {rev}: no content/{name} there ({p.stderr.decode("utf-8", "replace").strip()})')
    return p.stdout.decode('utf-8').replace('\r\n', '\n')


def row_fields(table, data):
    """(row id, top-level field) -> value for each row of a table's file, plus (None, key) for the
    file's other top-level keys (map.json's own fields beside its countries)."""
    out = {}
    if table == 'corps':
        rows = data.get('countries', []) if isinstance(data, dict) else []
        out.update({(None, k): v for k, v in data.items() if k != 'countries'} if isinstance(data, dict) else {})
    else:
        rows = data if isinstance(data, list) else []
    for row in rows:
        ident = (row.get('name') or '').lower() if table == 'corps' else row.get('id')
        out[(ident, '<row>')] = True
        out.update({(ident, k): v for k, v in row.items()})
    return out


def unset_drift(table, work, base, entries):
    """--from: the `<table>.<id>.<field>` paths where the working tree differs from the base and no entry
    sets that field, so writing the overlaid base would drop a by-hand edit."""
    covered = set()
    for e in entries:
        parts = e.partition('=')[0].strip().split('.')
        if parts[0] != table or len(parts) < 2:
            continue
        covered.add((parts[1], parts[2] if len(parts) > 2 else '*'))
    w, b = row_fields(table, work), row_fields(table, base)
    out = []
    for key in sorted(set(w) | set(b), key=lambda k: (str(k[0]), k[1])):
        ident, field = key
        if (key in w) != (key in b) and (ident, '<row>') in w and (ident, '<row>') in b:
            pass  # a field one side lacks: reported below like a changed value
        elif ident is not None and ((ident, '<row>') in w) != ((ident, '<row>') in b):
            # a row on one side only: one line for the row, none for its fields
            if field == '<row>' and (ident, '*') not in covered:
                out.append(f'{table}.{ident} (a row only in the {"working tree" if key in w else "base"})')
            continue
        elif w.get(key) == b.get(key) and type(w.get(key)) is type(b.get(key)):
            continue
        if ident is not None and ({(ident, field), ('*', field), (ident, '*')} & covered):
            continue
        out.append(f'{table}.{ident}.{field}' if ident is not None else f'{table} top-level {field!r}')
    return out


def read_entries(paths, sets):
    out = []
    for p in paths:
        with open(p, encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith('#'):
                    out.append(line)
    return out + list(sets)


def apply_from(args, entries, files, texts, work):
    """--from REV: the entries over REV's files; a file is written where the result differs from the
    working tree (an entry REV already holds still restores a working tree that lost it)."""
    failed = False
    for e in entries:
        try:
            if apply_entry(files, e) and args.check:
                print(f'would change (from {args.rev}):', e)
        except (ValueError, KeyError) as err:
            print('error:', err, file=sys.stderr)
            failed = True
    if failed:
        sys.exit(2)
    for name in sorted(files):
        if dump(json.loads(texts[name])) != texts[name]:
            print(f'error: content/{name} at {args.rev} is not in json.dumps(indent=1) form: edit it by hand',
                  file=sys.stderr)
            sys.exit(2)
    touched = sorted(n for n in files if dump(files[n]) != work[n])
    for name in touched:
        table = next(t for t, n in TABLES.items() if n == name and any(e.startswith(t + '.') for e in entries))
        drift = unset_drift(table, json.loads(work[name]), json.loads(texts[name]), entries)
        if drift:
            print(f'warning: content/{name} differs from {args.rev} where the overlay sets nothing; '
                  f'{"writing would drop" if args.check else "dropped"}: ' + ', '.join(drift), file=sys.stderr)
    if args.check:
        print(f'{len(entries)} entries from {args.rev}, '
              f'{"would change " + ", ".join(touched) if touched else "nothing to change"}')
        sys.exit(1 if touched else 0)
    for name in touched:
        with open(os.path.join(ROOT, 'content', name), 'w', encoding='utf-8', newline='\n') as f:
            f.write(dump(files[name]))
    print(f'{len(entries)} entries from {args.rev}; wrote {", ".join(touched) or "nothing"}')


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    ap.add_argument('files', nargs='*', help='overlay files, one entry a line')
    ap.add_argument('--set', action='append', default=[], help='one more entry')
    ap.add_argument('--check', action='store_true', help='print what would change, write nothing')
    ap.add_argument('--from', dest='rev', metavar='REV',
                    help="apply over each touched file as it is at git revision REV, not the working tree's "
                         '(a changed overlay drops what it no longer sets); warns of working-tree edits it would '
                         'drop in fields the overlay does not set')
    args = ap.parse_args(argv)
    entries = read_entries(args.files, args.set)
    if not entries:
        ap.error('no entries')
    files, texts, work = {}, {}, {}
    for name in sorted({TABLES[e.split('.', 1)[0]] for e in entries if e.split('.', 1)[0] in TABLES}):
        path = os.path.join(ROOT, 'content', name)
        with open(path, encoding='utf-8', newline='') as f:
            texts[name] = work[name] = f.read()
        if args.rev:
            try:
                texts[name] = git_text(args.rev, name)
            except ValueError as err:
                print('error:', err, file=sys.stderr)
                sys.exit(2)
        files[name] = json.loads(texts[name])
    if args.rev:
        return apply_from(args, entries, files, texts, work)
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

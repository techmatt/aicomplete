"""165: a deck as plain text to copy into a review: each card's name, then its tags, location and
cost, then each line as printed. Read from the generated docs only (docs/cards.md with
web/card/faces.json, docs/adversary-cards.md), so run `aic-sim docs`, `cards_doc.py` and
`load_adversary.py` (or `regen.py`) first after a content edit.

    python scripts/cards/deck_text.py DECK [DECK ...]     # to stdout
    python scripts/cards/deck_text.py --list              # the deck names

Decks: `player` (every player card, by primary tag), a tag (`science`, `industry`, `cyber`,
`power`, `diplomacy`, `security`), `unity`, `discord`, `board` (the Militarist's), `events`,
`threats1`, `threats2`, `projects`, or `all`.
"""
import json
import os
import re
import sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
TAGS = ['science', 'industry', 'cyber', 'power', 'diplomacy', 'security']
# docs/adversary-cards.md's sections, by deck name.
ADVERSARY = {'board': 'The Militarist (board)', 'events': 'Event deck', 'threats1': 'Threats, tier 1',
             'threats2': 'Threats, tier 2', 'projects': 'Projects'}
DECKS = ['player'] + TAGS + ['unity', 'discord'] + list(ADVERSARY)
# Columns whose text is several printed lines (one a sentence): an event's or a project's steps.
MULTI_LINE = {'text'}


def read(path):
    with open(os.path.join(ROOT, path), encoding='utf-8') as f:
        return f.read()


def sections(md):
    """{heading text: body} for every `##` and `###` heading."""
    out, name, body = {}, None, []
    for line in md.split('\n'):
        m = re.match(r'#{2,3} (.+)', line)
        if m:
            if name is not None:
                out[name] = '\n'.join(body)
            name, body = m.group(1).strip(), []
        else:
            body.append(line)
    if name is not None:
        out[name] = '\n'.join(body)
    return out


def section(secs, prefix):
    """The body of the one heading starting with `prefix` ("Science" finds "Science (32)")."""
    found = [v for k, v in secs.items() if k == prefix or k.startswith(prefix + ' (')]
    if len(found) != 1:
        sys.exit(f'deck_text: {len(found)} sections named {prefix!r}: regenerate the docs?')
    return found[0]


def table(body):
    """The first markdown table in `body`: (header, rows), cells stripped."""
    lines = [l for l in body.split('\n') if l.startswith('|')]
    if len(lines) < 2:
        return [], []
    cells = lambda l: [c.strip() for c in l.strip().strip('|').split(' | ')]
    return [h.lower() for h in cells(lines[0])], [cells(l) for l in lines[2:]]


def plain(s):
    """Markdown emphasis off: `**x**` and `*x*` read as x."""
    return re.sub(r'\*{1,2}([^*]+)\*{1,2}', r'\1', s).strip()


def sentences(s):
    return [p for p in re.split(r'(?<=[.!?])\s+(?=[A-Z])', s) if p]


def faces():
    """Each player card's printed text as its face lines: (top line, effect line), keyed by the joined
    text docs/cards.md prints (cards_doc.py's `face_text`)."""
    with open(os.path.join(ROOT, 'web', 'card', 'faces.json'), encoding='utf-8') as f:
        data = json.load(f)
    out = {}
    for face in data.values():
        lines = [t for t in (face.get('top_text'), face.get('text')) if t]
        out[' '.join(lines).replace('|', '/')] = lines
    return out


def player(tag, secs, face_lines):
    header, rows = table(section(secs, tag.capitalize()))
    out = []
    for r in rows:
        c = dict(zip(header, r))
        tags = [tag.capitalize()] + [t.strip().capitalize() for t in c['other tags'].split(',') if t.strip() not in ('', '-')]
        out.append([plain(c['card']),
                    f"{' + '.join(tags)} | tier {c['tier']} {c['kind']}",
                    f"Cost: {c['cost']} | insight {c['insight']}"]
                   + face_lines.get(c['effect'], [c['effect']]))
    return out


def alignment(side, secs):
    header, rows = table(section(secs, side.capitalize()))
    return [[plain(r[0])] + [f'{n}: {t}' for n, t in zip(['I', 'II', 'III'], r[1:])] for r in rows]


def adversary(deck, secs):
    body = section(secs, ADVERSARY[deck])
    header, rows = table(body)
    if not rows:  # the board: a line, then one bullet a rule
        lines = [plain(l.lstrip('- ')) for l in body.split('\n') if l.strip()]
        return [['The Militarist'] + lines]
    out = []
    for r in rows:
        card = [plain(r[0])]
        for h, v in zip(header[1:], r[1:]):
            name = h[0].upper() + h[1:]
            if h in MULTI_LINE:
                card += sentences(v)
            else:
                card.append(f'{name}: {v}')
        out.append(card)
    return out


def deck(name, player_secs, adv_secs, face_lines):
    if name == 'player':
        return [c for t in TAGS for c in player(t, player_secs, face_lines)]
    if name in TAGS:
        return player(name, player_secs, face_lines)
    if name in ('unity', 'discord'):
        return alignment(name, player_secs)
    return adversary(name, adv_secs)


def main(argv):
    if not argv or argv[0] in ('-h', '--help'):
        print(__doc__)
        return 0
    if argv[0] == '--list':
        print(' '.join(DECKS + ['all']))
        return 0
    names = DECKS if argv == ['all'] else argv
    bad = [n for n in names if n not in DECKS]
    if bad:
        print(f"deck_text: no deck {', '.join(bad)}; decks: {' '.join(DECKS)} all", file=sys.stderr)
        return 2
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    player_secs, adv_secs, face_lines = sections(read('docs/cards.md')), sections(read('docs/adversary-cards.md')), faces()
    for name in names:
        cards = deck(name, player_secs, adv_secs, face_lines)
        print(f'== {name} ({len(cards)}) ==\n')
        for card in cards:
            print('\n'.join(card) + '\n')
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))

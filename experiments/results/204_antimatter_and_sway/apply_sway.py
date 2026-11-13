"""204: the Sway pass. Tags every content sway op with its story (`narrative`) and moves the sides the pass
changes, in place, keeping each file's formatting. Rerunnable: a tagged op is retagged, a moved side stays moved.

Run from the repo root: python experiments/results/204_antimatter_and_sway/apply_sway.py
"""
import re
import sys

# (item id or name) -> (new side, new amount, story); None keeps the side and amount.
CHANGE = {
    'martial_cordon': ('discord', 1, 'force'),
    'kettle_operation': ('discord', 1, 'force'),
    'surgical_strike': ('discord', 1, 'force'),
    'nuclear_option': ('discord', 2, 'collateral'),
}
# The story of each source that keeps its side (by item; a side default below covers the rest).
STORY = {
    'shared_grid_accord': 'cooperation',
    'open_assembly': 'trust',
    'manifesto_leak': 'trust',
    'back_channel_talks': 'cooperation',
    'coalition_charter': 'cooperation',
    'planetary_referendum': 'trust',
    'diplomatic_expulsion': 'trust',
    'truth_commission': 'trust',
    'peace_conference': 'cooperation',
    'citywide_microgrid': 'cooperation',
    'holographic_assembly_hall': 'trust',
    'mass_defection': 'trust',
    'rally_the_allies': 'cooperation',
    'open_source_uprising': 'cooperation',
    'annihilation_lance': 'collateral',
    'fusion_commons': 'cooperation',
    'kinetic_interdiction': 'force',
    'volunteer_corps': 'cooperation',
    'citizens_assembly': 'trust',
    'Biomia': None,  # by side: its seawall's Unity is cooperation, an adversary cube's Discord fear
}
SIDE_DEFAULT = {'unity': 'cooperation', 'discord': 'fear'}

FILES = [
    'content/cards.json', 'content/alignment.json', 'content/map.json', 'content/threats.json',
    'content/adversary.json', 'content/projects.json',
    'drafts/adversary/militarist.json', 'drafts/adversary/projects.json',
    'drafts/adversary/threats_t1.json', 'drafts/adversary/threats_t2.json',
]

SWAY = re.compile(
    r'"op": "sway",(?P<s1>\s*)"side": "(?P<side>\w+)",(?P<s2>\s*)"amount": (?P<amount>\d+)'
    r'(?:,\s*"narrative": "\w+")?'
)
ITEM = re.compile(r'"id": "([^"]+)"')


def owner(text, pos):
    """The id of the item that holds position pos: the last id before it (map.json has none: side defaults)."""
    last = None
    for m in ITEM.finditer(text, 0, pos):
        last = m.group(1)
    return last


def main():
    seen = []
    for path in FILES:
        try:
            text = open(path, encoding='utf-8').read()
        except FileNotFoundError:
            continue
        out, at = [], 0
        for m in SWAY.finditer(text):
            who = owner(text, m.start())
            # A country's megaproject: its item is the country (the project's name is nested under it).
            side, amount = m.group('side'), int(m.group('amount'))
            if who in CHANGE:
                side, amount, story = CHANGE[who]
            else:
                story = STORY.get(who) or SIDE_DEFAULT[side]
            new = f'"op": "sway",{m.group("s1")}"side": "{side}",{m.group("s2")}"amount": {amount}, "narrative": "{story}"'
            out.append(text[at:m.start()])
            out.append(new)
            at = m.end()
            seen.append((path, who, side, amount, story))
        out.append(text[at:])
        new_text = ''.join(out)
        if new_text != text:
            with open(path, 'w', encoding='utf-8', newline='\n') as f:
                f.write(new_text)
    for row in seen:
        print(*row, sep='\t')
    return 0


if __name__ == '__main__':
    sys.exit(main())

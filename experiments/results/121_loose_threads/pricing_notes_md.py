"""Write pricing-notes.md: the summary plus each hand-written note's old text (from notes_before.txt)."""
import re
import sys

D = r'C:\Code\aicomplete-worktrees\aicomplete\121_loose_threads\experiments\results\121_loose_threads'
sys.path.insert(0, D)
from pricing_notes_anchors import HAND  # noqa: E402

old = {}
for line in open(D + r'\notes_before.txt', encoding='utf-8'):
    m = re.match(r'\[.\] ([a-z0-9_]+): (.*)$', line.rstrip('\n'))
    old[m.group(1)] = m.group(2)

head = """# Card-pricing notes: one anchor shape (121)

Matt's ruling on 119's deferred call 16: every note in `docs/card-pricing-notes.json` ends with
`anchors: pay N; get M = R.` (plain totals, R = M/N to 2 decimals). Done by
`pricing_notes_anchors.py` (this folder); `notes_before.txt` holds all 158 notes as they were.
158 notes before and after, same ids, same order, same layout (the writer reproduces the old file
byte for byte before writing).

Counts: 66 already in shape and untouched (68 less the two repriced here); 47 rewritten by regex
(13 B, `pay 29 + 16 = 45` or 117's `pay 34 + 4 x 2 = 42`, and 34 C,
`pay 12+4+4=20; get <breakdown> = M = R`: pay is the last pay number, get and ratio the last two,
the breakdown dropped, the stated ratio checked against M/N); 45 written by hand (below): 3 repriced
in 121, 13 E (r42/106 `Pay = N. Get = M`), 9 F (r43/108 `Give N ... Get`), 3 G, 17 X (ranges or
conditions; a range takes its midpoint, an alternative such as "alone / with Open Assembly" is
priced at the unconditional case with the other in words before the clause).

Judgment calls:
- F buildings (7) printed "Get B when built, then P a round, back in K rounds" with no total. Get is
  priced as B + its own play + 3.5 rounds of P (the 3.5-round horizon the r44 notes use), so these
  read 1.81-2.48; the payback time stays in words.
- E notes priced insight at 2.5 (r42's anchor); their pay and get totals are kept as printed, not
  repriced at today's 4.
- ceasefire_monitors had no pay/get at all: priced at today's unit anchors (8 cr + 2 x 4 insight = 16;
  3 ambassadors off 15 + a troop off 5 + 2 ambassadors 8 = 28).
- Two already-in-shape notes round off by a cent and were left: logic_bomb (16.9/17 = 0.99, printed
  1.00, its rationale says "reads 1.0") and liaison_corps (18/16 = 1.125, printed 1.12).
- market_stabilization_board's mojibake (a replacement character for the times sign) went with its breakdown.
- Cards with no note (breakthrough_leak, shared_compute_grant, prefab_plant_kit, shadow_broker_feed,
  transmission_corridor, hardpoint_lockdown, antitrust_action) still have none.

## Hand-written notes, old text

"""
with open(D + r'\pricing-notes.md', 'w', encoding='utf-8', newline='\n') as f:
    f.write(head)
    for k in HAND:
        f.write(f'- `{k}`: {old[k]}\n')
print('ok', len(HAND))

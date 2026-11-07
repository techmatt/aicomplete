"""136: the Undecided pool is gone; card text that moved Sway back to it now removes Unity from the Unity track.

    python experiments/results/136_alignment_and_win/track_words.py      # (a record of 136; moved at 144) rewrites drafts/adversary/*.json and content/alignment.json in place

Rerunnable: a file with no old phrase is left as it is. Run scripts/content/load_adversary.py after it.
"""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
FILES = sorted((ROOT / "drafts" / "adversary").glob("*.json")) + [ROOT / "content" / "alignment.json"]
OLD = re.compile(r"([Mm])ove (\d+) Sway from (?:the )?Unity(?: bucket)? to the Undecided")


def new(m):
    verb = "Remove" if m.group(1) == "M" else "remove"
    return f"{verb} {m.group(2)} Unity from the Unity track"


for f in FILES:
    text = f.read_text(encoding="utf-8")
    out = OLD.sub(new, text)
    if out != text:
        f.write_text(out, encoding="utf-8", newline="\n")
        print(f"{f.relative_to(ROOT)}: {len(OLD.findall(text))} rewritten")

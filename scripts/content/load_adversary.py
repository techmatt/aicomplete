"""Load the adversary drafts into content/ (135).

    python scripts/content/load_adversary.py           # write content/threats.json, projects.json, adversary.json
                                                       # and docs/adversary-cards.md (every card, generated)
    python scripts/content/load_adversary.py --check   # exit 1 if any of them differs from what it would write

The source of truth is drafts/adversary/*.json (format: experiments/results/134_militarist_drafts/spec.md). The loader
keeps the fields the engine reads and drops the review ones: every op loses `new` and `why`; cards lose `value`,
`check`, `flags` and `reuses`; pulls lose `per` (always "country"); a board rule's entries with no `op` (notes naming
today's rules) are dropped; the board's `build` numbers are not written (configs/baseline.json `adversary.*` is their
one home). 151: the board prints no creation line (its Garrisons rule is an income line).
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DRAFTS = ROOT / "drafts" / "adversary"
CONTENT = ROOT / "content"


def read(name):
    return json.loads((DRAFTS / name).read_text(encoding="utf-8"))


def strip(x):
    """Drop `new` and `why` from every op, at any depth (lose_or options included)."""
    if isinstance(x, list):
        return [strip(v) for v in x]
    if isinstance(x, dict):
        return {k: strip(v) for k, v in x.items() if not ("op" in x and k in ("new", "why"))}
    return x


def ops(effects):
    return [strip(e) for e in effects or [] if isinstance(e, dict) and "op" in e]


def threat(c):
    return {"id": c["id"], "name": c["name"], "tier": c["tier"], "tags": c["tags"], "locations": c["locations"],
            "text": c["text"], "counters": c["counters"], "effects": ops(c["effects"]), "harms": c["harms"]}


def project(c):
    out = {"id": c["id"], "name": c["name"], "tags": c["tags"], "cost": c["cost"], "ongoing": c["ongoing"],
           "text": c["text"], "effects": ops(c["effects"])}
    if c.get("resolve"):
        out["resolve"] = ops(c["resolve"])
    out["harms"] = c["harms"]
    return out


def event(c):
    if "arrivals" in c or "escalation" in c:
        raise SystemExit(f"{c['id']}: arrivals and escalation are retired (138): print creation lines")
    twists = ops(c["twists"])
    out = {"id": c["id"], "name": c["name"], "stage": c["stage"],
           "pulls": [{"tier": p["tier"], "tags": p["tags"]} for p in c["pulls"]],
           "standing": ops(c["standing"]), "twists": twists, "text": c["text"]}
    out["harms"] = c["harms"]
    return out


def board(b):
    return {"id": b["id"], "name": b["name"], "project_slots": b["project_slots"],
            "rules": [{"name": r["name"], "text": r["text"], "effects": ops(r["effects"])} for r in b["rules"]],
            "difficulty": {"text": b["difficulty"]["text"], "effects": ops(b["difficulty"]["effects"])},
            "harms": b["harms"]}


SACRIFICE = {"security": "Security", "diplomacy": "Diplomacy", "cyber": "Cyber", "industry": "Industry",
             "power": "Power", "science": "Science"}


def counter_words(option):
    parts = []
    for it in option:
        if "resource" in it:
            parts.append(f"{it['amount']} {it['resource'].replace('_', ' ')}")
        elif "sacrifice" in it:
            parts.append(f"{it['amount']} {SACRIFICE[it['sacrifice']]} tags")
        else:
            parts.append(f"{it['nodes_near']} nodes near it")
    return " + ".join(parts)


def cell(x):
    return str(x).replace("|", "/")


def cards_page(threats, projects, adversary):
    """docs/adversary-cards.md: every adversary card as printed (generated, like docs/cards.md)."""
    out = ["# Adversary cards (generated)", "",
           "*Written by `python scripts/content/load_adversary.py` from `drafts/adversary/` (the source); edit the "
           "drafts, not this page. The rules are in [rules/threats.md](rules/threats.md) and "
           "[rules/adversary.md](rules/adversary.md).*", ""]
    b = adversary["board"]
    slots = ", ".join((s["tag"] or "open").capitalize() for s in b["project_slots"])
    out += [f"## {b['name']} (board)", "", f"Project slots: {slots}.", ""]
    for r in b["rules"]:
        out.append(f"- **{r['name']}.** {r['text']}")
    out += [f"- **Difficulty.** {b['difficulty']['text']}", ""]
    out += ["## Event deck", "", "| Card | Stage | Text |", "|---|---|---|"]
    for e in adversary["events"]:
        out.append(f"| {cell(e['name'])} | {e['stage']} | {cell(e['text'])} |")
    for tier in (1, 2):
        out += ["", f"## Threats, tier {tier}", "", "| Threat | Tags | Locations | If not countered | Counter (one of) |",
                "|---|---|---|---|---|"]
        for t in (t for t in threats if t["tier"] == tier):
            tags = " + ".join(x.capitalize() for x in t["tags"])
            locs = ", ".join(x.replace("_", " ") for x in t["locations"])
            menu = "; ".join(counter_words(o) for o in t["counters"])
            out.append(f"| {cell(t['name'])} | {tags} | {locs} | {cell(t['text'])} | {cell(menu)} |")
    out += ["", "## Projects", "", "| Project | Tags | Compute per player | Resolves | Text |", "|---|---|---|---|---|"]
    for p in projects:
        tags = " + ".join(x.capitalize() for x in p["tags"])
        out.append(f"| {cell(p['name'])} | {tags} | {p['cost']} | {'ongoing' if p['ongoing'] else 'once'} | "
                   f"{cell(p['text'])} |")
    return "\n".join(out) + "\n"


def build():
    threats = [threat(c) for tier in (1, 2) for c in read(f"threats_t{tier}.json")["cards"]]  # 146: no tier 3
    projects = [project(c) for c in read("projects.json")["cards"]]
    mil = read("militarist.json")
    adversary = {"board": board(mil["board"]), "events": [event(e) for e in mil["events"]]}
    page = {"board": adversary["board"], "events": adversary["events"]}
    return {"content/threats.json": dump(threats), "content/projects.json": dump(projects),
            "content/adversary.json": dump(adversary),
            "docs/adversary-cards.md": cards_page(threats, projects, page)}


def dump(data):
    return json.dumps(data, indent=1, ensure_ascii=False) + "\n"


def main():
    check = "--check" in sys.argv
    stale = []
    for name, text in build().items():
        path = ROOT / name
        old = path.read_text(encoding="utf-8") if path.exists() else None
        if old == text:
            continue
        if check:
            stale.append(name)
        else:
            path.write_text(text, encoding="utf-8", newline="\n")
            print(f"wrote {name}")
    if stale:
        print("differs from the drafts: " + ", ".join(stale)
              + " (run python scripts/content/load_adversary.py)")
        sys.exit(1)


if __name__ == "__main__":
    main()

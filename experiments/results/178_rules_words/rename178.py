"""178's printed-word pass over content text (Matt's rulings #17, #22; amendments 8044, 8122).

    python experiments/results/178_rules_words/rename178.py [--check]

Rewrites only the "text" and "name" string values of the files below (never ids, ops or notes), in place:
- the adversary's troop line Deploy -> Muster (deployed -> mustered);
- the node power's name Deploy -> Expand (content/ais.json, a "name" field);
- adversary cards' "board" meaning a country -> "country";
- corp troop(s) -> Drone(s) (amendment 8122);
- piece names on cards capitalized: Ambassador, Agent, Node, Botnet, Troop (amendment 8044).
Then run scripts/content/load_adversary.py to write content/ from drafts/adversary/. Rerunnable: a second pass
changes nothing.
"""
import os
import re
import sys

TOP = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", ".."))
FILES = [
    "drafts/adversary/militarist.json",
    "drafts/adversary/projects.json",
    "drafts/adversary/threats_t1.json",
    "drafts/adversary/threats_t2.json",
    "content/alignment.json",
    "content/map.json",
    "content/ais.json",
]

WORDS = [
    (r"\bcorp troops\b", "Drones"),
    (r"\bCorp troops\b", "Drones"),
    (r"\bcorp troop\b", "Drone"),
    (r"\bCorp troop\b", "Drone"),
    (r"\bDeploys\b", "Musters"),
    (r"\bDeploy\b", "Muster"),
    (r"\bdeployed\b", "mustered"),
    (r"\bdeploy\b", "muster"),
    (r"\bon each board where\b", "in each country where"),
    (r"\bon the board with the most\b", "in the country with the most"),
    (r"\beach board\b", "each country"),
    (r"\bEach board\b", "Each country"),
    (r"\ba board\b", "a country"),
    (r"\bambassador(s?)\b", r"Ambassador\1"),
    (r"\bagent(s?)\b", r"Agent\1"),
    (r"\bnode(s?)\b", r"Node\1"),
    (r"\bbotnet(s?)\b", r"Botnet\1"),
    (r"\btroop(s?)\b", r"Troop\1"),
]

FIELD = re.compile(r'("(?:text|name)":\s*")((?:[^"\\]|\\.)*)(")')


def words(s):
    for pat, rep in WORDS:
        s = re.sub(pat, rep, s)
    return s


def field(m):
    key, body, end = m.group(1), m.group(2), m.group(3)
    # The node power: an AI board's power named Deploy is Expand (the adversary's line is Muster).
    if key.startswith('"name"') and body == "Deploy":
        return f"{key}Expand{end}"
    return f"{key}{words(body)}{end}"


def main():
    check = "--check" in sys.argv
    changed = []
    for rel in FILES:
        p = os.path.join(TOP, rel)
        with open(p, encoding="utf-8") as f:
            old = f.read()
        new = FIELD.sub(field, old)
        if new != old:
            changed.append(rel)
            if not check:
                with open(p, "w", encoding="utf-8", newline="\n") as f:
                    f.write(new)
    print(("would change: " if check else "changed: ") + (", ".join(changed) or "nothing"))
    sys.exit(1 if check and changed else 0)


if __name__ == "__main__":
    main()

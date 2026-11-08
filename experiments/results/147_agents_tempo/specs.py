"""147: write the smoke and read specs (experiments/screen-147-*.json, read-147-*.json).

    python experiments/results/147_agents_tempo/specs.py screen NAME AGENT[;AGENT...] [--diffs 2] [--games 32] [--seed 36800]
    python experiments/results/147_agents_tempo/specs.py final|final2|final3 TAG [AGENT;...]

Every cell runs the three seat pairs (Builder + Protector, Builder + Researcher, Protector + Researcher). Screens run
on the dev seeds (36800-36831, 36800-36863 at --games 64) unless --seed says otherwise. The first final read (F1) is
two 48-seed sets from the ledger's next free (37240-37287, 37288-37335); the agents changed after it
(`w.corrupt_exposure` 1 to 0.25), so the final read (F2) is the next two (37336-37383, 37384-37431). Each plays the
final agents and the r61-equivalent spec (r61's agents on 147's binary, the same games: same147.py) on the same seeds.
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
DEV = 36800
FINAL = (("a", 37240, 48), ("b", 37288, 48))
FINAL2 = (("a", 37336, 48), ("b", 37384, 48))
# After F2 a review fixed D67's armor (the site owner's ambassador is no armor against corruption; a hardening
# token and Sanctuary are), its doubled danger step and the war forecast's factory armor: the final read, F3.
FINAL3 = (("a", 37432, 48), ("b", 37480, 48))


def spec(name, note, agents, diffs, games, seed, levers=None):
    """`levers`: name -> config sets, each run on every pair as its own variant (`<pair>_<name>`)."""
    variants = [{"name": p, "set": s} for p, s in PAIRS.items()]
    for lever, sets in (levers or {}).items():
        variants += [{"name": f"{p}_{lever}", "set": s + sets} for p, s in PAIRS.items()]
    return {"name": name, "note": note, "configs": ["configs/baseline.json"], "variants": variants,
            "difficulties": diffs, "agents": agents, "games": games, "first_seed": seed, "records": 0}


def write(name, doc):
    p = ROOT / "experiments" / f"{name}.json"
    p.write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(p.relative_to(ROOT))


def main():
    a = sys.argv[1:]
    opt = lambda k, d: a[a.index(k) + 1] if k in a else d
    if a[0] == "screen":
        name, agents = a[1], a[2].split(";")
        diffs = [int(x) for x in opt("--diffs", "2").split(",")]
        games, seed = int(opt("--games", "32")), int(opt("--seed", str(DEV)))
        # --lever NAME=K=V,K=V (repeatable): a config variant beside the base, on every pair.
        levers = {}
        for i, x in enumerate(a):
            if x == "--lever":
                lever, _, sets = a[i + 1].partition("=")
                levers[lever] = sets.split(",")
        write(f"screen-147-{name}", spec(f"screen-147-{name}", f"147 smoke {name}: {'; '.join(agents)}, seeds from {seed}",
                                         agents, diffs, games, seed, levers))
    elif a[0] in ("final", "final2", "final3"):
        tag = a[1]
        agents = a[2].split(";") if len(a) > 2 else ["strategist"]
        for part, seed, games in {"final": FINAL, "final2": FINAL2, "final3": FINAL3}[a[0]]:
            write(f"read-147-{tag}-{part}", spec(f"read-147-{tag}-{part}",
                                                 f"147 final paired read ({tag}), set {part}: seeds {seed}-{seed + games - 1}, the three pairs, d0-d3.",
                                                 agents, [0, 1, 2, 3], games, seed))


if __name__ == "__main__":
    main()

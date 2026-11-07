"""Check the adversary drafts (134) and write their listing and picker.

    python drafts/adversary/check.py            # checks, then the pull simulation
    python drafts/adversary/check.py --write    # also the listing and the picker
    python drafts/adversary/check.py --content  # the same checks on what content/ loaded (135's loader)

Checks: ids unique, deck sizes and tags, the tier rules, counters (no credits under 5), harms named from the ledger's
list, the harms ledger's rule (every harm from 2+ kinds of source, each threat tier 5+ kinds), ops the engine lacks.
The pull simulation plays the draw rule over random Militarist decks at 1-4 countries and every difficulty's stage
counts and extra pulls (configs/baseline.json `adversary`), and fails if a pull would need a second reshuffle of a deck.
`--content` rebuilds the draft shape from content/ (threats by tier, arrivals read back from the arrivals twist);
the value and check lines and `new` marks aren't loaded, so those checks are skipped.
"""

import json
import random
import re
import sys
import textwrap
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = ROOT / "drafts" / "adversary"
TAGS = ["military", "influence", "cyber", "economic", "grid"]
HARMS = [
    "node destroyed", "credits or income lost", "cards discarded", "insight lost", "ambassadors lost",
    "tokens stripped from cards", "facilities destroyed or seized", "production blacked out",
    "adversary pieces placed", "projects advanced (compute)", "extra threats", "Discord sway", "Unity pulled back",
    "actions or claims blocked", "energy lost", "war brought forward", "troops made",
]
PERMANENT = {"node destroyed", "facilities destroyed or seized"}
LOCATIONS = {"city", "coastal_city", "station", "factory", "base"}
SACRIFICE = {"security", "diplomacy", "cyber", "industry", "power", "science"}
RESOURCES = {"credits", "energy", "insight", "research_cubes", "chips", "ambassadors"}
BRITISH = ["colour", "flavour", "neighbour", "behaviour", "armour", "favour", "harbour", "defence", "offence",
           "centre", "mobilis", "organis", "towards", "whilst", "amongst", "grey", "labour", "rumour"]
PRINTED = {1: 5, 2: 6, 3: 4}  # Militarist events printed per stage
CONTENT_MODE = "--content" in sys.argv


def adversary_params():
    cfg = json.loads((ROOT / "configs" / "baseline.json").read_text(encoding="utf-8"))
    return next(m["params"] for m in cfg["modules"] if m["module"] == "adversary")


def stage_deals():
    """(difficulty, stage counts dealt, extra pulls of each card's first pull) for difficulty 0-4
    (docs/rules/adversary.md: each level includes the ones below)."""
    a = adversary_params()
    out = []
    for d in range(5):
        s1, s2, s3 = a["stage_cards"]
        if d >= 2:
            s1, s3 = s1 - a["stage1_removed"], s3 + a["stage3_added"]
        if d >= 3:
            s2 -= a["stage2_removed"]
        out.append((d, (s1, s2, s3), a["extra_threats"] if d >= 1 else 0))
    return out


STAGE_DEALS = stage_deals()

errors, warnings = [], []


def load():
    if CONTENT_MODE:
        return load_content()
    t = {tier: json.loads((HERE / f"threats_t{tier}.json").read_text(encoding="utf-8"))["cards"] for tier in (1, 2, 3)}
    projects = json.loads((HERE / "projects.json").read_text(encoding="utf-8"))["cards"]
    mil = json.loads((HERE / "militarist.json").read_text(encoding="utf-8"))
    return t, projects, mil


def load_content():
    """The draft shape, rebuilt from content/ (scripts/content/load_adversary.py's output)."""
    def read(name):
        return json.loads((ROOT / "content" / name).read_text(encoding="utf-8"))
    t = {tier: [] for tier in (1, 2, 3)}
    for c in read("threats.json"):
        t.setdefault(c["tier"], []).append(c)
    adv = read("adversary.json")
    events = []
    for e in adv["events"]:
        arr = [tw for tw in e["twists"] if tw.get("op") == "arrivals"]
        events.append(dict(e, twists=[tw for tw in e["twists"] if tw.get("op") != "arrivals"],
                           arrivals=arr[0]["site"] if arr else None))
        if len(arr) > 1:
            errors.append(f"{e['id']}: {len(arr)} arrivals twists")
    a = adversary_params()
    build = {k: a[k] for k in ("city_build", "factory_build", "arrivals", "corrupt_at", "spread_range", "wall_troops",
                               "escalation_ambassadors", "botnet_compute") if k in a}
    return t, read("projects.json"), {"board": dict(adv["board"], build=build), "events": events}


def engine_ops():
    text = (ROOT / "docs" / "ops.md").read_text(encoding="utf-8")
    ops = set(re.findall(r"^\| `([a-z_]+)` \|", text, re.M))
    adv = json.loads((ROOT / "content" / "adversary.json").read_text(encoding="utf-8"))
    for card in adv["events"] if isinstance(adv, dict) else adv:  # twist ops live in content
        for tw in card.get("twists", []):
            ops.add(tw["op"])
    return ops


def walk_effects(effects):
    for e in effects or []:
        if isinstance(e, dict):
            yield e
            for k in ("then", "effects", "or", "options"):
                v = e.get(k)
                if isinstance(v, list):
                    yield from walk_effects(v)


def fits(card_tags, pull_tags):
    if "any" in pull_tags or "universal" in card_tags:
        return True
    return bool(set(card_tags) & set(pull_tags))


def check_card_common(c, kind, ops, new_ops):
    cid = c.get("id", "?")
    for h in c.get("harms", []):
        if h not in HARMS:
            errors.append(f"{cid}: harm {h!r} not in the list")
    for e in walk_effects(c.get("effects", []) + c.get("standing", []) + c.get("twists", [])):
        op = e.get("op")
        if e.get("new") or op not in ops:
            new_ops[op].append(cid)
            if not e.get("new") and not CONTENT_MODE:  # content doesn't carry the marks
                warnings.append(f"{cid}: op {op!r} isn't an engine op and isn't marked new")
    blob = json.dumps(c).lower()
    for b in BRITISH:
        if b in blob:
            errors.append(f"{cid}: British spelling {b!r}")
    for k in () if CONTENT_MODE else ("value", "check"):
        if not c.get(k):
            errors.append(f"{cid}: no {k}")


def check_threats(t, ops, new_ops):
    for tier, cards in t.items():
        tags = Counter()
        for c in cards:
            cid = c["id"]
            check_card_common(c, f"t{tier}", ops, new_ops)
            if c.get("tier") != tier:
                errors.append(f"{cid}: tier {c.get('tier')} in the tier-{tier} deck")
            for g in c["tags"]:
                if g not in TAGS + ["universal"]:
                    errors.append(f"{cid}: tag {g}")
                tags[g] += 1
            if "universal" in c["tags"] and len(c["tags"]) > 1:
                errors.append(f"{cid}: Universal with another tag")
            if not 1 <= len(c.get("locations", [])) <= 2 or set(c["locations"]) - LOCATIONS:
                errors.append(f"{cid}: locations {c.get('locations')}")
            if not c.get("counters"):
                errors.append(f"{cid}: no counters")
            for opt in c.get("counters", []):
                if not isinstance(opt, list):
                    errors.append(f"{cid}: a counter option is not a list of items")
                    continue
                for item in opt:
                    if "resource" in item:
                        if item["resource"] not in RESOURCES:
                            errors.append(f"{cid}: counter resource {item['resource']}")
                        if item["resource"] == "credits" and item["amount"] < 5:
                            errors.append(f"{cid}: a credits counter under 5")
                    elif "sacrifice" in item:
                        if item["sacrifice"] not in SACRIFICE:
                            errors.append(f"{cid}: sacrifice {item['sacrifice']}")
                    elif "nodes_near" not in item:
                        errors.append(f"{cid}: counter item {item}")
            if tier == 1 and PERMANENT & set(c.get("harms", [])):
                errors.append(f"{cid}: a permanent harm at tier 1")
        kinds = {h for c in cards for h in c.get("harms", [])}
        if len(kinds) < 5:
            errors.append(f"tier {tier}: only {len(kinds)} kinds of harm")
        print(f"tier {tier}: {len(cards)} cards, tags {dict(tags)}, {len(kinds)} kinds of harm")


def ledger(t, projects, mil):
    """harm -> {source kind: count}"""
    table = defaultdict(Counter)
    for tier, cards in t.items():
        for c in cards:
            for h in set(c.get("harms", [])):
                table[h][f"tier-{tier} threats"] += 1
    for c in projects:
        for h in set(c.get("harms", [])):
            table[h]["projects"] += 1
    for c in mil["events"]:
        for h in set(c.get("harms", [])):
            table[h]["Militarist events"] += 1
    for h in set(mil["board"].get("harms", [])):
        table[h]["Militarist board"] += 1
    for h in HARMS:
        if len(table[h]) < 2:
            errors.append(f"harm {h!r}: {len(table[h])} kind(s) of source {dict(table[h])}")
    return table


class Deck:
    def __init__(self, cards, rng):
        self.rng = rng
        self.draw = list(cards)
        rng.shuffle(self.draw)
        self.discard = []

    def pull(self, pull_tags, countries):
        """Draw until a card fits; return (card, reshuffles) or (None, n) when a second reshuffle would be needed.
        A card fits when it carries a pulled tag and has a location type in the country (no bases at 1 country)."""
        reshuffles = 0
        while True:
            if not self.draw:
                if reshuffles == 1 or not self.discard:
                    return None, reshuffles + 1
                reshuffles += 1
                self.draw = self.discard
                self.discard = []
                self.rng.shuffle(self.draw)
            c = self.draw.pop()
            if fits(c["tags"], pull_tags) and lands(c, countries):
                return c, reshuffles
            self.discard.append(c)


def simulate(t, mil, games=4000, seed=134):
    rng = random.Random(seed)
    by_stage = defaultdict(list)
    for e in mil["events"]:
        by_stage[e["stage"]].append(e)
    worst = Counter()
    fails = Counter()
    pulls = 0
    for g in range(games):
        countries = 1 + g % 4
        _, deals, extra = STAGE_DEALS[(g // 4) % len(STAGE_DEALS)]
        order = []
        for stage, n in zip((1, 2, 3), deals):
            pool = list(by_stage[stage])
            rng.shuffle(pool)
            order += pool[:n]
        decks = {tier: Deck(cards, rng) for tier, cards in t.items()}
        for ev in order:
            out = defaultdict(list)  # this round's threats, out of the deck until cleanup
            for i, p in enumerate(ev["pulls"]):
                # difficulty 1+: `extra_threats` more pulls of the card's first pull, countries in turn
                for _ in range(countries * p.get("count", 1) + (extra if i == 0 else 0)):
                    pulls += 1
                    card, rs = decks[p["tier"]].pull(p["tags"], countries)
                    worst[(p["tier"], tuple(p["tags"]))] = max(worst[(p["tier"], tuple(p["tags"]))], rs)
                    if card is None:
                        fails[(ev["id"], countries)] += 1
                    else:
                        out[p["tier"]].append(card)
            for tier, cards in out.items():
                decks[tier].discard += cards
    return pulls, worst, fails


def lands(card, countries):
    """A threat lands in a country that has one of its location types: 1 country has no military bases."""
    return countries > 1 or any(loc != "base" for loc in card["locations"])


def worst_case(t, mil):
    """Hall's condition with the whole deck in hand, at 1 and 4 countries: every set of a card's pulls on one tier
    must have at least as many fitting cards as slots."""
    from itertools import combinations
    extra = max(x for _, _, x in STAGE_DEALS)
    for countries in (1, 4):
        for ev in mil["events"]:
            by_tier = defaultdict(list)
            for i, p in enumerate(ev["pulls"]):
                by_tier[p["tier"]] += [p["tags"]] * (countries * p.get("count", 1) + (extra if i == 0 else 0))
            for tier, slots in by_tier.items():
                kinds = [list(k) for k in {tuple(s) for s in slots}]
                for r in range(1, len(kinds) + 1):
                    for sub in combinations(kinds, r):
                        need = sum(1 for s in slots if list(s) in sub)
                        have = sum(1 for c in t[tier] if lands(c, countries) and any(fits(c["tags"], s) for s in sub))
                        if have < need:
                            errors.append(f"{ev['id']}: tier-{tier} pulls {sub} need {need} at {countries} "
                                          f"countries, deck fits {have}")


def check_militarist(mil, projects, ops, new_ops):
    b = mil["board"]
    check_card_common(b, "board", ops, new_ops)
    for r in b.get("rules", []):
        for e in walk_effects(r.get("effects", [])):
            if "rule" in e:  # a rule of today's engine, named, not an op
                continue
            if e.get("new") or e.get("op") not in ops:
                new_ops[e.get("op")].append(b["id"])
    for e in walk_effects(b.get("difficulty", {}).get("effects", [])):
        if e.get("new") or e.get("op") not in ops:
            new_ops[e.get("op")].append(b["id"])
    stages = Counter(e["stage"] for e in mil["events"])
    for s, n in PRINTED.items():
        if stages[s] != n:
            errors.append(f"Militarist stage {s}: {stages[s]} printed, want {n}")
    for d, deals, _ in STAGE_DEALS:
        for s, n in zip((1, 2, 3), deals):
            if n > stages[s]:
                errors.append(f"difficulty {d}: deals {n} stage-{s} events, {stages[s]} printed")
    for e in mil["events"]:
        if e.get("arrivals") not in ("coastal", "industrial", "stations", "factory", None):
            errors.append(f"{e['id']}: arrivals {e.get('arrivals')!r}")
    for s in (1, 2, 3):
        kinds = {e.get("arrivals") for e in mil["events"] if e["stage"] == s} - {None}
        print(f"stage {s} arrivals: {sorted(kinds)}")
    for e in mil["events"]:
        check_card_common(e, "event", ops, new_ops)
    tagged = [s["tag"] for s in b["project_slots"] if s.get("tag")]
    for tag in tagged:
        n = sum(1 for p in projects if fits(p["tags"], [tag]))
        if n < len(b["project_slots"]):
            errors.append(f"projects: only {n} fit a {tag} slot")
    pulled = Counter(g for e in mil["events"] for p in e["pulls"] for g in p["tags"])
    for g in TAGS:
        if not pulled[g] and not pulled["any"]:
            errors.append(f"Militarist: no pull names {g}")
    print(f"Militarist: stages {dict(stages)}, pull tags {dict(pulled)}")


def check_projects(projects, ops, new_ops):
    tags = Counter(g for p in projects for g in p["tags"])
    for p in projects:
        check_card_common(p, "project", ops, new_ops)
    print(f"projects: {len(projects)}, tags {dict(tags)}, costs {sorted(p['cost'] for p in projects)}")


# ---- listing and picker -------------------------------------------------------------------------------------------

LABEL = {"military": "Military", "influence": "Influence", "cyber": "Cyber", "economic": "Economic", "grid": "Grid",
         "universal": "Universal", "any": "any"}
SAC = {"security": "Security", "diplomacy": "Diplomacy", "cyber": "Cyber", "industry": "Industry", "power": "Power",
       "science": "Science"}
LOC = {"city": "a city", "coastal_city": "a coastal city", "station": "a research station",
       "factory": "a factory or industrial zone", "base": "a military base"}
COLOR = {"military": "#b03a2e", "influence": "#7d3c98", "cyber": "#1f78b4", "economic": "#b7950b",
         "grid": "#17a589", "universal": "#555555"}


def counter_text(opts):
    out = []
    for opt in opts:
        parts = []
        for i in opt:
            if "resource" in i:
                parts.append(f"{i['amount']} {i['resource'].replace('_', ' ')}")
            elif "sacrifice" in i:
                parts.append(f"sacrifice {i['amount']} {SAC[i['sacrifice']]} tag{'s' if i['amount'] > 1 else ''}")
            else:
                n = i["nodes_near"]
                parts.append(f"remove {n} of your node{'s' if n > 1 else ''} on or next to it")
        out.append(" + ".join(parts))
    return " OR ".join(out)


def tag_text(c, tier=None):
    return " + ".join(f"{LABEL[g]}{' ' + str(tier) if tier else ''}" for g in c["tags"])


def pull_text(p):
    tags = " or ".join(LABEL[g] for g in p["tags"]) if p["tags"] != ["any"] else "any tag"
    return f"{p.get('count', 1)}x {tags} {p['tier']} per country"


def card_lines(c, kind):
    """(title, subtitle, body lines, value, check) for one card."""
    if kind.startswith("t"):
        tier = int(kind[1])
        sub = f"Threat, tier {tier} | {tag_text(c, tier)} | at {' and '.join(LOC[x] for x in c['locations'])}"
        body = [f"If not countered: {c['text']}", f"Counter (anyone near it): {counter_text(c['counters'])}"]
    elif kind == "project":
        sub = f"Project | {tag_text(c)} | {c['cost']} compute per player{' | ongoing' if c.get('ongoing') else ''}"
        body = [c["text"]]
    elif kind == "event":
        sub = f"Militarist event, stage {c['stage']} | arrivals: {c.get('arrivals') or 'none'}"
        body = [c["text"]]
    else:
        sub = "Militarist board"
        body = (c.get("text") or "").split(" | ")
    flags = c.get("flags") or []
    if flags:
        body.append("Flags: " + "; ".join(flags))
    return c["name"], sub, body, c.get("value", ""), c.get("check", "")


def board_card(b):
    slots = ", ".join(LABEL[s["tag"]] if s.get("tag") else "open" for s in b["project_slots"])
    rules = [f"{r['name']}: {r['text']}" for r in b["rules"]]
    build = ", ".join(f"{k} {v}" for k, v in b["build"].items())
    text = " | ".join([f"Project slots: {slots}"] + rules + [f"Build: {build}", f"Difficulty: {b['difficulty']['text']}"])
    return dict(b, text=text, tags=["military"])


def esc(s):
    return s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def svg(c, kind):
    title, sub, body, value, check = card_lines(c, kind)
    w = 720
    lines = []
    for para in body:
        lines += [(l, "body") for l in textwrap.wrap(para, 80)] + [("", "gap")]
    lines += [(l, "value") for l in textwrap.wrap("Value: " + value, 96)]
    lines += [(l, "check") for l in textwrap.wrap("Check: " + check, 96)]
    subs = textwrap.wrap(sub, 88)
    h = 92 + 18 * (len(subs) - 1) + 21 * len(lines) + 16
    color = COLOR.get(c["tags"][0], "#555") if c.get("tags") else "#555"
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">',
           f'<rect width="{w}" height="{h}" fill="#fbfaf6" stroke="{color}" stroke-width="6"/>',
           f'<rect width="{w}" height="58" fill="{color}"/>',
           f'<text x="16" y="38" font-family="Georgia, serif" font-size="26" fill="#fff">{esc(title)}</text>',
           ] + [f'<text x="16" y="{80 + 18 * i}" font-family="Arial, sans-serif" font-size="14" fill="#333">{esc(s)}</text>'
                for i, s in enumerate(subs)]
    y = 106 + 18 * (len(subs) - 1)
    for text, style in lines:
        if style == "gap":
            y += 6
            continue
        fill, size, ital = {"body": ("#111", 17, ""), "value": ("#1a5276", 14, ""),
                            "check": ("#555", 14, ' font-style="italic"')}[style]
        out.append(f'<text x="16" y="{y}" font-family="Arial, sans-serif" font-size="{size}" fill="{fill}"{ital}>'
                   f'{esc(text)}</text>')
        y += 21
    out.append("</svg>")
    return "\n".join(out)


def write_outputs(t, projects, mil, table, new_ops):
    groups = [("Threats, tier 1", "t1", t[1]), ("Threats, tier 2", "t2", t[2]), ("Threats, tier 3", "t3", t[3]),
              ("Projects", "project", projects), ("Militarist board", "board", [board_card(mil["board"])]),
              ("Militarist event deck", "event", sorted(mil["events"], key=lambda e: e["stage"]))]
    img_dir = ROOT / "art-originals" / "picks" / "adversary_drafts"
    img_dir.mkdir(parents=True, exist_ok=True)
    rows, md = [], ["# Adversary drafts (134): the listing", "",
                    "Generated by `python drafts/adversary/check.py --write` from `drafts/adversary/`. One line per "
                    "card: its text, counter or pulls, and value. Spec: [spec.md](spec.md).", ""]
    n = 0
    for title, kind, cards in groups:
        md += [f"## {title}", ""]
        for c in cards:
            n += 1
            name, sub, body, value, check = card_lines(c, kind)
            md.append(f"- **{name}** ({sub}). " + " ".join(body) + f" *Value:* {value} *Check:* {check}")
            (img_dir / f"{c['id']}.svg").write_text(svg(c, kind), encoding="utf-8", newline="\n")
            rows.append({"id": c["id"], "label": f"{n}. {title}: {name}", "note": sub, "kind": "many",
                         "options": [{"key": "keep", "image": f"../art-originals/picks/adversary_drafts/{c['id']}.svg",
                                      "label": "goes forward"}]})
        md.append("")
    current = {r["id"] for r in rows}
    for old in img_dir.glob("*.svg"):  # this picker's own tiles for cards since renamed
        if old.stem not in current:
            old.unlink()
    ops = engine_ops()
    md += ["## Ops the engine lacks", "", "Marked `new: true` in the drafts. A new op, then an op the engine has that a "
           "draft prints with a new field or scope (its `why` says which).", ""]
    for op, ids in sorted(new_ops.items(), key=lambda x: (x[0] in ops, str(x[0]))):
        what = "new field or scope" if op in ops else "new op"
        md.append(f"- `{op}` ({what}): {', '.join(sorted(set(ids)))}")
    md += ["", "## The harms ledger across the drafts", "",
           "| Harm | " + " | ".join(SOURCES) + " |", "|---|" + "---|" * len(SOURCES)]
    for h in HARMS:
        md.append(f"| {h} | " + " | ".join(str(table[h].get(s, "")) for s in SOURCES) + " |")
    out = ROOT / "experiments" / "results" / "134_militarist_drafts" / "listing.md"
    out.write_text("\n".join(md) + "\n", encoding="utf-8", newline="\n")
    manifest = {"name": "adversary_drafts", "title": "Adversary drafts: threats, projects, the Militarist",
                "intro": "One row per drafted card (134), grouped by deck: tier-1, tier-2 and tier-3 threats, the "
                         "project deck, the Militarist board, the Militarist event deck. Each tile shows the card's "
                         "full text, its value line and its check. Toggle on what goes forward; nothing is "
                         "preselected. Numbers are sane starting values, not tuned. Notes per row are welcome.",
                "rows": rows}
    (ROOT / "picks" / "adversary_drafts.json").write_text(json.dumps(manifest, indent=1, ensure_ascii=False) + "\n",
                                                         encoding="utf-8", newline="\n")
    print(f"wrote {n} cards: listing.md, picks/adversary_drafts.json, {img_dir}")


SOURCES = ["tier-1 threats", "tier-2 threats", "tier-3 threats", "projects", "Militarist events", "Militarist board"]


def main():
    t, projects, mil = load()
    ops = engine_ops()
    new_ops = defaultdict(list)
    ids = [c["id"] for cards in t.values() for c in cards] + [p["id"] for p in projects] + \
          [e["id"] for e in mil["events"]] + [mil["board"]["id"]]
    for i, k in Counter(ids).items():
        if k > 1:
            errors.append(f"id {i} used {k} times")
    check_threats(t, ops, new_ops)
    check_projects(projects, ops, new_ops)
    check_militarist(mil, projects, ops, new_ops)
    table = ledger(t, projects, mil)
    before = len(errors)
    worst_case(t, mil)
    print(f"Hall's condition at 1 and 4 countries (with difficulty 1's extra pulls): "
          f"{'holds' if len(errors) == before else 'fails'}")
    print("stage deals by difficulty (stage I, II, III; extra pulls): "
          + ", ".join(f"d{d} {s} +{x}" for d, s, x in STAGE_DEALS))
    pulls, worst, fails = simulate(t, mil)
    print(f"simulated {pulls} pulls; most reshuffles in one pull by (tier, tags): "
          + ", ".join(f"{k}: {v}" for k, v in sorted(worst.items(), key=str)))
    for k, v in fails.items():
        errors.append(f"pull failed (needs a second reshuffle): {k} x{v}")
    print("ops the engine lacks:", {k: len(v) for k, v in new_ops.items()})
    for w in warnings:
        print("warning:", w)
    for e in errors:
        print("ERROR:", e)
    if "--write" in sys.argv and CONTENT_MODE:
        print("ERROR: --write reads the drafts (value and check lines); run it without --content")
        sys.exit(1)
    if "--write" in sys.argv:
        write_outputs(t, projects, mil, table, new_ops)
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()

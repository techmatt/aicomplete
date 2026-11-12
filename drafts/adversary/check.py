"""Check the adversary drafts (134).

    python drafts/adversary/check.py            # checks, then the pull simulation
    python drafts/adversary/check.py --content  # the same checks on what content/ loaded (135's loader)

Checks: ids unique, deck sizes and tags, the tier rules, counters (no credits under 5), Matt's threat card standard
(146: two consequences at most, two counter options at most, no shared currency, no each-player harm, no special
pieces, no tier 3), harms named from the ledger's list, the harms ledger's rule (every harm from 2+ kinds of source, each threat tier 5+ kinds), ops the engine lacks.
The pull simulation plays the draw rule over random Militarist decks at 1-4 countries and every difficulty's stage
counts and extra pulls (configs/baseline.json `adversary`), and fails if a pull would need a second reshuffle of a deck.
`--content` rebuilds the draft shape from content/ (threats by tier);
the value and check lines and `new` marks aren't loaded, so those checks are skipped.
"""

import json
import random
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = ROOT / "drafts" / "adversary"
TAGS = ["security", "diplomacy", "cyber", "industry", "power"]
HARMS = [
    "node destroyed", "credits or income lost", "cards discarded", "ambassadors lost",
    "tokens stripped from cards", "facilities destroyed or seized", "production blacked out",
    "adversary pieces placed", "projects advanced (compute)", "extra threats", "Discord sway", "Unity pulled back",
    "actions or claims blocked", "energy lost", "war brought forward", "troops made",
    # 195: the project deck's new harms (the ledger's rows).
    "protections stripped", "the clock sped up", "adversary cubes on megaprojects",
]
# 195: harms the drafts may print from one kind of source: the project deck's new rows (the engine's ledger,
# docs/harms.md, counts the board rules beside them and lists any thin one under *Gaps*).
ONE_SOURCE = {"protections stripped", "the clock sped up", "adversary cubes on megaprojects"}
PERMANENT = {"node destroyed", "facilities destroyed or seized"}
LOCATIONS = {"city", "coastal_city", "station", "factory", "base"}
SACRIFICE = {"security", "diplomacy", "cyber", "industry", "power", "science"}
RESOURCES = {"credits", "energy", "insight", "research_cubes", "chips", "ambassadors"}
BRITISH = ["colour", "flavour", "neighbour", "behaviour", "armour", "favour", "harbour", "defence", "offence",
           "centre", "mobilis", "organis", "towards", "whilst", "amongst", "grey", "labour", "rumour"]
PRINTED = {1: 5, 2: 6, 3: 4}  # Militarist events printed per stage
# 146: the ladder of an event card's pulls by stage: stage I one tier-1 pull, stage II one tier-2 (or tier 1 and tier
# 2), stage III two tier-2 pulls.
LADDER = {1: [(1,)], 2: [(2,), (1, 2)], 3: [(2, 2)]}
CONTENT_MODE = "--content" in sys.argv
TIERS = (1, 2)  # 146: no tier 3


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
        if d >= a["stage_swap_at"]:
            s1, s3 = s1 - a["stage1_removed"], s3 + a["stage3_added"]
        if d >= 3:  # adversary.rs `build`'s literal 3, not a config key (182: deferred)
            s1, s2 = s1 + a["stage1_added"], s2 - a["stage2_removed"]
        s1, s2 = max(s1, 0), max(s2, 0)
        # 196: `stage_extra` cards at this level go to stage I while it has cards left (PRINTED, checked against
        # the deck below), then stage II (adversary/mod.rs `build`).
        extra = (a.get("stage_extra") or [])[d:d + 1]
        extra = extra[0] if extra else 0
        to_one = min(extra, max(PRINTED[1] - s1, 0))
        s1, s2 = s1 + to_one, s2 + extra - to_one
        out.append((d, (s1, s2, s3), a["extra_threats"] if d >= a["extra_threats_at"] else 0))
    return out


STAGE_DEALS = stage_deals()

errors, warnings = [], []


def load(here=HERE):
    if CONTENT_MODE:
        return load_content()
    t = {tier: json.loads((here / f"threats_t{tier}.json").read_text(encoding="utf-8"))["cards"] for tier in TIERS}
    projects = json.loads((here / "projects.json").read_text(encoding="utf-8"))["cards"]
    mil = json.loads((here / "militarist.json").read_text(encoding="utf-8"))
    return t, projects, mil


def load_content():
    """The draft shape, rebuilt from content/ (scripts/content/load_adversary.py's output)."""
    def read(name):
        return json.loads((ROOT / "content" / name).read_text(encoding="utf-8"))
    t = {tier: [] for tier in TIERS}
    for c in read("threats.json"):
        t.setdefault(c["tier"], []).append(c)
    adv = read("adversary.json")
    events = []
    for e in adv["events"]:
        events.append(dict(e))
    a = adversary_params()
    # 138: the build numbers left after creation lines replaced city_build, arrivals and escalation.
    build = {k: a[k] for k in ("corrupt_at", "corrupt_station_at", "corrupt_factory_at", "corrupt_land_at",
                               "wall_troops", "botnet_compute") if k in a}
    return t, read("projects.json"), {"board": dict(adv["board"], build=build), "events": events}


def engine_ops():
    text = (ROOT / "docs" / "ops.md").read_text(encoding="utf-8")
    ops = set(re.findall(r"^\| `([a-z_]+)` \|", text, re.M))
    adv = json.loads((ROOT / "content" / "adversary.json").read_text(encoding="utf-8"))
    for card in adv["events"]:  # twist ops live in content
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


# 142: what a threat's harm takes from the players near it, by op (the Rust twin is `content::harm_currencies`).
# Sacrificing tags (cards in play) and discarding (cards in hand) are different currencies.
HARM_CURRENCY = {
    "lose_insight": {"insight"}, "lose_ambassador": {"ambassadors"}, "each_destroy_node": {"nodes"},
    "destroy_node": {"nodes"},
    "discard": {"hand"}, "discard_most_expensive": {"hand"},
}


def harm_currencies(c, tier):
    out = set()
    for e in walk_effects(c.get("effects", [])):
        op = e.get("op")
        out |= HARM_CURRENCY.get(op, set())
        if op in ("players_lose", "lose_income", "theft", "requisition", "lose_node_or", "lose_ambassadors_or"):
            out.add(e.get("resource", "credits"))
        if op == "requisition":
            out.add("hand")
        if op == "lose_node_or":
            out.add("nodes")
        if op == "lose_ambassadors_or":
            out.add("ambassadors")
        if op == "troops" and e.get("destroy"):
            out.add("nodes")
        if op == "destroy_facility" or (op == "corrupt_city" and e.get("at") == "location"):
            out.add("nodes")
    return out


def standard_breaks(c):
    """146: what in a threat breaks the mechanical rules of Matt's threat card standard (the Rust twin is
    `content::check_threat` with `threat_rule_break`)."""
    out = []
    if c.get("tier") not in TIERS:
        out.append(f"tier {c.get('tier')} (no tier 3)")
    if len(c.get("counters", [])) > 2:
        out.append(f"{len(c['counters'])} counter options (two at most)")
    effects = c.get("effects", [])
    if len(effects) > 2:
        out.append(f"{len(effects)} consequences (two at most)")
    for e in walk_effects(effects):
        op = e.get("op")
        if op == "troops" and e.get("stand"):
            out.append("a troop that stands (no special pieces)")
        if op == "occupy_factory" or (op == "corp_troops" and e.get("past_cap")):
            out.append(f"{op}: a special piece")
        if op == "each_destroy_node":
            out.append("each player destroys a node (the nearest player's: destroy_node)")
        if op == "flag":
            out.append(f"flag {e.get('name')} hits every player near it, not the nearest")
        if e.get("each_country"):
            out.append(f"{op}: an each-country line")
        if op == "corrupt_city" and e.get("at") == "location":
            out.append("corrupt_city at location destroys every player's nodes near it")
        if op == "theft" and e.get("who") != "nearest":
            out.append("theft hits the nearest player")
    return out


def counter_currencies(c):
    out = set()
    for opt in c.get("counters", []):
        for item in opt if isinstance(opt, list) else []:
            if "resource" in item:
                out.add(item["resource"])
            elif "nodes_near" in item:
                out.add("nodes")
            elif "sacrifice" in item:
                out.add("tags")
    return out


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
            for b in standard_breaks(c):
                errors.append(f"{cid}: {b}")
            shared = counter_currencies(c) & harm_currencies(c, tier)
            if shared:
                errors.append(f"{cid}: a counter and its uncountered harm share {sorted(shared)}")
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
        if len(table[h]) < (1 if h in ONE_SOURCE else 2):
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
                # from `extra_threats_at`: `extra_threats` more pulls of the card's first pull, countries in turn
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


def event_step(e):
    """151: where an event line sits in the fixed step order: rally 0, muster 1, other lines 2, corrupt 3."""
    if e.get("op") == "rally" and not e.get("count"):
        return 0
    if e.get("op") == "troops" and e.get("each_country") and not any(e.get(k) for k in ("at", "stand", "location", "if")):
        return 1
    return 3 if e.get("op") == "corrupt" else 2


def event_breaks(ev):
    """151, the event standard (the Rust twin is `content::event_rule_break`): one Rally X, the fixed order (threat,
    rally, muster, other lines, corrupt), at most two other lines, no per-event compute or creation line, "each
    country", never "each board" (178, Matt's ruling #17; 151 had it the other way)."""
    out = []
    if "each board" in ev.get("text", "").lower():
        out.append('says "each board" (each country)')
    steps = [event_step(e) for e in ev.get("standing", []) + ev.get("twists", [])]
    if steps.count(0) != 1:
        out.append("one Rally X line")
    if any(a > b for a, b in zip(steps, steps[1:])) or any(event_step(e) < 2 for e in ev.get("twists", [])):
        out.append("a step out of the fixed order (threat, rally, muster, other lines, corrupt)")
    if steps.count(2) > 2:
        out.append("more than two other lines (consequences)")
    if any(e.get("op") == "corrupt" for e in ev.get("standing", [])):
        out.append("a Corrupt line before the corrupt step")
    if any(e.get("op") == "rally" and e.get("count") for e in ev.get("standing", []) + ev.get("twists", [])):
        out.append("an event's Rally lands its pool (no count)")
    if any(e.get("op") in ("compute", "create") for e in ev.get("standing", [])):
        out.append("a per-event compute or creation line")
    return out


def project_breaks(p):
    """151: a project's consequences (one, rarely two) and its words (the Rust twin is `content::project_rule_break`)."""
    out = []
    if "each board" in p.get("text", "").lower():
        out.append('says "each board" (each country)')
    if len(p.get("effects", [])) + len(p.get("resolve", [])) > 2:
        out.append("more than two consequences")
    return out


def check_militarist(mil, projects, ops, new_ops):
    b = mil["board"]
    check_card_common(b, "board", ops, new_ops)
    for r in b.get("rules", []) + [b.get("difficulty", {})]:
        if "each board" in r.get("text", "").lower():
            errors.append(f"board rule {r.get('name', 'difficulty')}: says \"each board\" (each country)")
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
        check_card_common(e, "event", ops, new_ops)
        for why in event_breaks(e):
            errors.append(f"{e['id']}: off the event standard: {why}")
        tiers =tuple(sorted(p["tier"] for p in e["pulls"]))
        if tiers not in LADDER[e["stage"]]:
            errors.append(f"{e['id']}: stage-{e['stage']} pulls tiers {tiers}, the ladder wants one of {LADDER[e['stage']]}")
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
        for why in project_breaks(p):
            errors.append(f"{p['id']}: off the standard: {why}")
    print(f"projects: {len(projects)}, tags {dict(tags)}, costs {sorted(p['cost'] for p in projects)}")


def run(here=HERE):
    """Every check and the pull simulation on the drafts in `here` (printing as it goes); returns the errors.
    scripts/content/load_adversary.py calls it and refuses to write content/ on any."""
    errors.clear()
    warnings.clear()
    t, projects, mil = load(here)
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
    ledger(t, projects, mil)
    before = len(errors)
    worst_case(t, mil)
    print(f"Hall's condition at 1 and 4 countries (with the extra pulls): "
          f"{'holds' if len(errors) == before else 'fails'}")
    print("stage deals by difficulty (stage I, II, III; extra pulls): "
          + ", ".join(f"d{d} {s} +{x}" for d, s, x in STAGE_DEALS))
    pulls, worst, fails = simulate(t, mil)
    print(f"simulated {pulls} pulls; most reshuffles in one pull by (tier, tags): "
          + ", ".join(f"{k}: {v}" for k, v in sorted(worst.items(), key=str)))
    for k, v in fails.items():
        errors.append(f"pull failed (needs a second reshuffle): {k} x{v}")
    print("ops the engine lacks:", {k: len(v) for k, v in new_ops.items()})
    return errors


def main():
    run()
    for w in warnings:
        print("warning:", w)
    for e in errors:
        print("ERROR:", e)
    if "--write" in sys.argv:
        print("ERROR: --write is retired (165): its listing and picker were 134's; content/ is read by load_adversary.py")
        sys.exit(1)
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()

"""136 death audit: every game lost because a seat had no nodes left.

For each saved `node` loss record, replays its seed with `--trace` on the given
binary (same overrides) and reads the event log: each seat's nodes at the end of
every round, nodes placed each action phase and the placements it could still
afford when it passed, every node removal with its cause, and whether a
node-safe line plausibly existed.

Usage (from the repo root; the binary is run from its own directory so it reads
its configs):

    python experiments/results/136_alignment_and_win/death_audit.py \
        --bin out/r54/aic-sim.exe --out out/death \
        out/death/rec_d2_bp out/death/rec_d2_br out/death/rec_d2_pr ...

Records come from
    aic-sim run configs/baseline.json --games 48 --seed 36392 --agent strategist \
        --set adversary.difficulty=D [--set ai.boards=0,2] --threads 3 \
        --save out/death/rec_dD_PAIR --save-when node --save-max 48

Writes <out>/audit.json (every loss, in full) and prints the markdown tables.
"""
import argparse
import collections
import concurrent.futures as cf
import glob
import json
import os
import re
import subprocess
import sys

ACTION_PHASE = "cards.play+threats.counter+alignment.act+board.act+ai.power"
BOARDS = ["Builder", "Protector", "Researcher"]
TRACK_LEN = 7
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", ".."))


def load_names():
    """Card name -> kind: discord / unity / adversary event / threat."""
    kinds = {}

    def read(name):
        with open(os.path.join(REPO, "content", name), encoding="utf-8") as f:
            return json.load(f)

    for c in read("alignment.json"):
        kinds[c["name"]] = "discord:" + c["name"] if c.get("side", c.get("kind")) == "discord" else "unity:" + c["name"]
    adv = read("adversary.json")
    for c in adv.get("events", []):
        kinds[c["name"]] = "adversary:" + c["name"]
    for c in read("threats.json"):
        kinds[c["name"]] = "threat:" + c["name"]
    return kinds


AXES = {"E": (1, 0), "W": (-1, 0), "SE": (0, 1), "NW": (0, -1), "SW": (-1, 1), "NE": (1, -1)}


def map_content(countries=2):
    """Hex coords (axial, as the trace prints them) of the layout: hex n of a piece is
    anchor + i·axis0 + j·axis1 with n = 1 + size·i + j. Returns {coord: (country, n)}, oceans, cities."""
    with open(os.path.join(REPO, "content", "map.json"), encoding="utf-8") as f:
        m = json.load(f)
    size = m["piece"]["size"]
    lay = next(l for l in m["layouts"] if l["countries"] == countries)
    hexes, ocean, cities = {}, set(), set()
    for c, p in enumerate(lay["pieces"]):
        a0, a1 = AXES[p["axes"][0]], AXES[p["axes"][1]]
        for i in range(size):
            for j in range(size):
                h = (p["anchor"][0] + i * a0[0] + j * a1[0], p["anchor"][1] + i * a0[1] + j * a1[1])
                n = 1 + size * i + j
                hexes[h] = (c, n)
                if n in m["countries"][c]["ocean"]:
                    ocean.add(h)
                if n in m["countries"][c]["cities"]:
                    cities.add(h)
    return hexes, ocean, cities


def ai_content():
    with open(os.path.join(REPO, "content", "ais.json"), encoding="utf-8") as f:
        return json.load(f)


def step_cost(track, taken):
    if taken >= len(track):
        return None
    return track[taken].get("cost", 0)


def trace_game(binary, rec_path, out_dir):
    with open(rec_path, encoding="utf-8") as f:
        rec = json.load(f)
    note = rec["note"]
    over = note.split("overrides: ", 1)[1].split() if "overrides: " in note else []
    seed = rec["seed"]
    tag = os.path.basename(os.path.dirname(rec_path))
    tpath = os.path.join(out_dir, "traces", f"{tag}-{seed}.txt")
    if not os.path.exists(tpath):
        os.makedirs(os.path.dirname(tpath), exist_ok=True)
        bdir = os.path.dirname(os.path.abspath(binary))
        cmd = ["nice", "-n", "10", os.path.abspath(binary), "run", "configs/baseline.json", "--games", "1",
               "--seed", str(seed), "--agent", "strategist", "--threads", "1", "--trace"]
        for o in over:
            cmd += ["--set", o]
        p = subprocess.run(cmd, cwd=bdir, capture_output=True, text=True, encoding="utf-8")
        with open(tpath, "w", encoding="utf-8", newline="\n") as f:
            f.write(p.stdout + p.stderr)
    boards = [0, 1]
    for m in rec["config"]["modules"]:
        if m["module"] == "ai":
            boards = m["params"].get("boards", boards)
        if m["module"] == "board":
            corrupt_at = None
        if m["module"] == "adversary":
            corrupt_at = m["params"].get("corrupt_at", 4)
    return rec, tag, tpath, boards, corrupt_at


def hexkey(h):
    return (h[0], h[1])


def dist(a, b):
    # axial coordinates
    dq, dr = a[0] - b[0], a[1] - b[1]
    return (abs(dq) + abs(dr) + abs(dq + dr)) // 2


def classify(src, kinds, phase):
    if src is None:
        return {"adversary.corrupt": "corruption", "board.war": "war", "board.march": "troop strike",
                "threats.execute": "threat"}.get(phase, "other:" + phase)
    k = kinds.get(src)
    if k:
        kind = k.split(":")[0]
        return {"discord": "Discord card", "adversary": "adversary card", "threat": "threat",
                "unity": "Unity card"}[kind] + ": " + src
    if src.startswith("Adversary troop"):
        return "troop strike"
    return "other: " + src


def audit(rec, tag, tpath, boards, corrupt_at, kinds, ais):
    lines = []
    with open(tpath, encoding="utf-8") as f:
        for l in f:
            if l.startswith("{"):
                lines.append(json.loads(l))
    seats = len(boards)
    rnd, phase = 0, ""
    tok = collections.defaultdict(int)  # (hex, token) -> total
    res = collections.defaultdict(int)  # (seat, resource) -> total
    var = collections.defaultdict(int)
    last_prompt = None  # (index, text)
    last_act = None  # the last non-choose action: (index, category)
    spent = collections.defaultdict(collections.Counter)  # (round, seat) -> category -> energy spent
    start_e = {}  # (round, seat) -> energy when the action phase began
    last_note = ""
    adv_card = ""  # this round's adversary event card
    ctx_card = None  # card whose effect is resolving (from a note)
    removals = []
    placements = []
    rounds = {}  # round -> per seat dict
    placed = collections.defaultdict(int)  # (round, seat) -> nodes placed in the action phase
    powers = collections.defaultdict(list)
    corrupted_cities = []  # (round, hex, name)
    snap_adv = {}  # round -> {hex: adversary ambassadors + markers} at the end of the action phase
    end = None
    in_action = False

    def nodes(s):
        return sum(v for (h, t), v in tok.items() if t == f"node:{s}" and v > 0)

    def node_hexes(s):
        return [h for (h, t), v in tok.items() if t == f"node:{s}" and v > 0]

    def snapshot():
        out = {}
        for s in range(seats):
            b = ais[boards[s]]
            t1, t2 = var.get(f"board.track.{s}", 0), var.get(f"board.track2.{s}", 0)
            steps = [c for c in (step_cost(b["economy_track"], t1), step_cost(b["play_track"], t2)) if c is not None]
            step = min(steps) if steps else None
            e, cr, cubes = res[(s, "energy")], res[(s, "credits")], res[(s, "research_cubes")]
            used1 = var.get(f"ai.power_used.{s}.1", 0) > 0
            used2 = var.get(f"ai.power_used.{s}.2", 0) > 0
            spare = 0
            budget_e = e
            if step is not None and not used1 and budget_e >= 1 + step:
                spare += 1
                budget_e -= 1 + step
                # the next step is then usually the same or one dearer; read it as the same
            if step is not None and not used2:
                name = b["powers"][2]["name"]
                if name == "Buy In" and budget_e >= 1 + step and cr >= 6:
                    spare += 1
                elif name == "Peer Review" and cubes >= 3 and budget_e >= step:
                    spare += 1
                elif name == "Accord" and budget_e >= 1 + step:
                    spare += 0.5  # needs a Diplomacy card to sacrifice: unknown here
            out[s] = {"nodes": nodes(s), "node_hexes": node_hexes(s), "energy": e, "credits": cr, "cubes": cubes,
                      "ambassadors": res[(s, "ambassadors")], "step": step,
                      "deploy_used": used1, "second_used": used2, "spare_placements": spare,
                      "placed": placed[(rnd, s)], "powers": list(powers[(rnd, s)]),
                      "start_energy": start_e.get((rnd, s)), "spent": dict(spent[(rnd, s)])}
        return out

    for i, d in enumerate(lines):
        if "action" in d:
            a = d["action"]
            if a["type"] == "power" and in_action:
                powers[(rnd, a["seat"])].append(a["power"])
            if a["type"] != "choose":
                t = a["type"]
                if t == "power":
                    b = ais[boards[a["seat"]]]
                    cat = "node power" if a["power"] in (1, 2) else "power " + b["powers"][a["power"]]["name"]
                elif t in ("play", "use"):
                    cat = "card " + a.get("card", "")
                elif t in ("pay", "counter_with", "withdraw"):
                    cat = "counter " + str(a.get("threat", ""))
                elif t == "standard":
                    cat = "standard " + str(a.get("act", ""))
                else:
                    cat = t
                last_act = (i, cat, a)
            continue
        e = d.get("event")
        if not e:
            continue
        ty = e["type"]
        if ty == "round_started":
            rnd = e["round"]
            ctx_card = None
        elif ty == "phase_started":
            if in_action:
                rounds.setdefault(rnd, {})["action_end"] = snapshot()
                snap_adv[rnd] = {h: tok[(h, "ambassador:adversary")] + tok[(h, "corruption")]
                                 for (h, t) in list(tok) if t == "ambassador:adversary"}
            phase = e["phase"]
            in_action = phase == ACTION_PHASE
            if in_action:
                for s in range(seats):
                    start_e[(rnd, s)] = res[(s, "energy")]
            ctx_card = None
            last_act = None
            if phase == "alignment.sway":
                if rnd - 1 >= 0:
                    rounds.setdefault(rnd - 1, {})["end_nodes"] = {s: nodes(s) for s in range(seats)}
        elif ty == "asked":
            last_prompt = (i, e["prompt"])
        elif ty == "note":
            last_note = e["text"]
            m = re.match(r"Adversary card: ([^(]+?) \(", e["text"])
            if m:
                adv_card = m.group(1)
                ctx_card = m.group(1)
            m = re.match(r"(.+?) \((discord|unity)\) resolves", e["text"])
            if m:
                ctx_card = m.group(1).strip()
            m = re.match(r"Threat (.+?) executes", e["text"])
            if m:
                ctx_card = m.group(1).strip()
            m = re.match(r"(.+) is corrupted$", e["text"])
            if m and "hex" in e:
                corrupted_cities.append((rnd, hexkey(e["hex"]), m.group(1), phase))
        elif ty == "resource_changed":
            res[(e["seat"], e["resource"])] = e["total"]
            if in_action and e["resource"] == "energy" and e["delta"] < 0 and last_act:
                cat = last_act[1]
                if cat.startswith("card "):
                    cat = "card"
                spent[(rnd, e["seat"])][cat] += -e["delta"]
        elif ty == "var_changed":
            var[e["key"]] = e["total"]
        elif ty == "tokens_changed":
            h = hexkey(e["hex"])
            t = e["token"]
            if t.startswith("node:") and e["delta"] > 0 and in_action:
                # the legal hexes before this placement (board/node.rs node_hexes), and which of them
                # stand 2+ from every threatened clean city (adversary ambassadors + marker >= corrupt_at - 2)
                s = int(t.split(":")[1])
                mine = set(node_hexes(s))
                legal = [x for x in MAP[0] if x not in MAP[1] and tok[(x, "corruption")] == 0
                         and not any(tok[(x, f"node:{k}")] > 0 for k in range(seats))
                         and any(dist(x, y) == 1 for y in mine)]
                threat = [c for c in MAP[2] if tok[(c, "cube:adversary")] == 0
                          and tok[(c, "ambassador:adversary")] + tok[(c, "corruption")] >= corrupt_at - 2]
                safe = [x for x in legal if all(dist(x, c) >= 2 for c in threat)]
                chosen_threat = any(dist(h, c) <= 1 for c in threat)
                pending_place = {"legal": len(legal), "safe": len(safe), "chosen_threatened": chosen_threat,
                                 "legal_hexes": legal}
            else:
                pending_place = None
            tok[(h, e["token"])] = e["total"]
            if t.startswith("node:"):
                s = int(t.split(":")[1])
                if e["delta"] > 0 and in_action:
                    placed[(rnd, s)] += e["delta"]
                    opts = lines[i - 1].get("options") if i and "action" in lines[i - 1] else None
                    # the last choose before it carries the hex prompt's option count
                    for j in range(i - 1, max(0, i - 4), -1):
                        if "action" in lines[j] and lines[j]["action"]["type"] == "choose":
                            opts = lines[j].get("options")
                            break
                    pl = {"round": rnd, "seat": s, "hex": h, "options": opts}
                    if pending_place:
                        pl.update(pending_place)
                    placements.append(pl)
                if e["delta"] < 0:
                    src = None
                    if last_prompt and i - last_prompt[0] <= 3:
                        txt = last_prompt[1]
                        if "destroy" in txt:
                            src = txt.split(":")[0]
                    if src is None and in_action and last_act and last_act[1].startswith("counter"):
                        src = "counter:" + last_act[1].split(" ", 1)[1]
                    if src is None and phase not in ("adversary.corrupt",) and ctx_card:
                        src = ctx_card
                    if src is None and phase == "adversary.build":
                        src = adv_card
                    removals.append({"round": rnd, "phase": phase, "seat": s, "hex": h, "n": -e["delta"],
                                     "src": src, "left": nodes(s), "line": i,
                                     "adv_card": adv_card})
        elif ty == "game_ended":
            end = e
            if in_action:
                rounds.setdefault(rnd, {})["action_end"] = snapshot()
            rounds.setdefault(rnd, {})["end_nodes"] = {s: nodes(s) for s in range(seats)}
    for r in removals:
        r["cause"] = classify(r["src"], kinds, r["phase"])
    m = re.search(r"seat (\d+) has no nodes", end["detail"]) if end else None
    loser = int(m.group(1)) if m else None
    R = end["round"] if end else rnd
    fatal = [r for r in removals if r["seat"] == loser and r["round"] == R]
    # the fatal phase: the phase of the last removal; the fatal hit: all removals of that seat in that phase
    fphase = fatal[-1]["phase"] if fatal else None
    fhit = [r for r in fatal if r["phase"] == fphase]
    before_fatal = sum(r["n"] for r in fhit) + 0  # nodes held when the fatal phase began
    # action phases that came before the fatal phase: this round's only if the fatal phase follows it
    order = ["alignment.sway", "adversary.event", "threats.deal", "adversary.build", "adversary.spread",
             "adversary.corrupt", "board.march", "adversary.reveal", "cards.tier", "cards.deal", "cards.draft",
             "cards.keep", ACTION_PHASE, "board.bases", "board.produce", "economy.upkeep", "board.war",
             "threats.execute", "board.cleanup"]
    after_action = fphase in order and order.index(fphase) > order.index(ACTION_PHASE)
    last_actions = [R, R - 1] if after_action else [R - 1, R - 2]
    acts = {r: rounds.get(r, {}).get("action_end", {}).get(loser) for r in last_actions}
    spare = sum((a or {}).get("spare_placements", 0) for a in acts.values())
    causes = collections.Counter(r["cause"].split(":")[0] for r in fhit)
    fcause = "+".join(sorted(causes))
    # positional hit (corruption, a hex) vs a count ("each player destroys N")
    count_hit = any(r["src"] and ("Discord" in r["cause"] or "adversary card" in r["cause"]) for r in fhit)
    # corruption foresight: the fatal city's ambassadors (and marker) at the end of the last action phase
    foresee = None
    if "corruption" in causes:
        cities = [c for c in corrupted_cities if c[0] == R and c[3] == fphase]
        last_snap = snap_adv.get(R - 1, {})
        foresee = [{"city": c[2], "hex": c[1], "adv_before": last_snap.get(c[1], 0)} for c in cities]
    # extra placements: per action phase, the unused node powers the energy left over plus the energy
    # spent on other things would have paid for (each at 1 + the cheapest next step)
    extra = 0
    for x in acts.values():
        if not x or x["step"] is None:
            continue
        unused = (0 if x["deploy_used"] else 1) + (0 if x["second_used"] or boards[loser] == 1 else 1)
        if boards[loser] == 2 and not x["second_used"] and x["cubes"] < 3:
            unused -= 1
        if boards[loser] == 0 and not x["second_used"] and x["credits"] < 6:
            unused -= 1
        other = sum(v for k, v in x["spent"].items() if k != "node power")
        budget = x["energy"] + other
        extra += min(unused, budget // (1 + x["step"]))
    # what a node-safe line needed: a positional hit (corruption, Offensive's hex, war, a strike) one
    # node standing outside it; a count hit ("each player destroys N") N + 1 - held
    count_n = 0
    for r in fhit:
        c = r["cause"]
        if "Border War" in c or c.startswith("threat") or c.startswith("other: counter"):
            count_n = max(count_n, 1)
        elif "Offensive" in c and r["phase"] != "adversary.corrupt":
            count_n = max(count_n, 2 if r["src"] else 0)
        elif c.startswith("Discord card") or c.startswith("adversary card"):
            count_n = max(count_n, 2)
    held = sum(r["n"] for r in fhit)
    needed = max(1, count_n + 1 - held) if count_n else 1
    safe_spare = extra >= needed
    # placements in the run-up (the two action phases) that stood within 1 of a city that corrupted in
    # the fatal phase, with the hex prompt's option count
    blast = None
    if foresee:
        blast = []
        for p in placements:
            if p["seat"] == loser and p["round"] in last_actions:
                near = any(dist(p["hex"], c["hex"]) <= 1 for c in foresee)
                lh = p.get("legal_hexes", [])
                outside = sum(1 for x in lh if all(dist(x, c["hex"]) >= 2 for c in foresee))
                blast.append({"round": p["round"], "in_blast": near, "options": p["options"],
                              "legal": p.get("legal"), "outside_blast": outside,
                              "safe_by_foresight": p.get("safe"), "chosen_threatened": p.get("chosen_threatened")})
        last = acts.get(max(last_actions))
        if last:
            hs = last["node_hexes"]
            for c in foresee:
                c["nodes_near_at_last_action"] = sum(1 for h in hs if dist(h, c["hex"]) <= 1)
            foresee_all = sum(1 for h in hs if any(dist(h, c["hex"]) <= 1 for c in foresee))
            foresee.append({"exposed": foresee_all, "of": len(hs)})
    elsewhere = collections.Counter()
    for x in acts.values():
        if x:
            elsewhere.update({k: v for k, v in x["spent"].items() if k != "node power"})
    rec_out = {
        "tag": tag, "seed": rec["seed"], "round": R, "loser": loser,
        "board": BOARDS[boards[loser]] if loser is not None else None,
        "end_nodes": {r: rounds.get(r, {}).get("end_nodes", {}).get(loser) for r in range(max(1, R - 3), R + 1)},
        "acts": acts, "spare": spare, "fatal_phase": fphase, "fatal_cause": fcause,
        "fatal_srcs": sorted({r["cause"] for r in fhit}), "fatal_lost": sum(r["n"] for r in fhit),
        "count_hit": count_hit, "foresee": foresee, "safe_spare": safe_spare, "extra": extra,
        "needed": needed, "elsewhere": dict(elsewhere),
        "placed_in_blast": blast,
        "loser_placements": [{k: v for k, v in p.items() if k != "legal_hexes"} for p in placements if p["seat"] == loser],
        "adv_card": fhit[-1]["adv_card"] if fhit else None,
        "removals": [{k: (list(v) if k == "hex" else v) for k, v in r.items() if k != "line"}
                     for r in removals if r["round"] >= R - 2],
        "all_removals_loser": collections.Counter(r["cause"].split(":")[0] for r in removals if r["seat"] == loser),
        "all_removals": collections.Counter(r["cause"].split(":")[0] for r in removals),
        "unattributed": [r["cause"] for r in removals if r["cause"].startswith("other")],
    }
    return rec_out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--bin", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--jobs", type=int, default=3)
    ap.add_argument("dirs", nargs="+")
    a = ap.parse_args()
    kinds = load_names()
    ais = ai_content()
    global MAP
    MAP = map_content(2)
    recs = []
    for d in a.dirs:
        recs += sorted(glob.glob(os.path.join(d, "*.json")))
    with cf.ThreadPoolExecutor(a.jobs) as ex:
        traced = list(ex.map(lambda p: trace_game(a.bin, p, a.out), recs))
    rows = [audit(*t, kinds, ais) for t in traced]
    with open(os.path.join(a.out, "audit.json"), "w", encoding="utf-8", newline="\n") as f:
        json.dump(rows, f, indent=1, default=str)
    report(rows)


def fmt_act(x):
    if not x:
        return "-"
    other = sum(v for k, v in x["spent"].items() if k != "node power")
    return (f"+{x['placed']} e{x['start_energy']}>{x['energy']}"
            + (f" (o{other})" if other else "") + f" c{x['credits']}"
            + (f" q{x['cubes']}" if x["cubes"] else "") + f" st{x['step']}")


def report(rows):
    print("| cell | seed | rd | seat | nodes end R-3..R | action phases (2 before the hit) | extra / needed | fatal phase | fatal hit | lost | city adv. amb. before |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    for r in rows:
        en = " ".join(str(v) if v is not None else "-" for v in r["end_nodes"].values())
        acts = " / ".join(f"r{k}: " + fmt_act(r["acts"][k]) for k in sorted(r["acts"]))
        fs = "; ".join(s.replace("adversary card: ", "adv: ").replace("Discord card: ", "Discord: ") for s in r["fatal_srcs"])
        fo = ", ".join(f"{c['city']} {c['adv_before']}" for c in (r["foresee"] or []) if "city" in c)
        ex = [c for c in (r["foresee"] or []) if "exposed" in c]
        if ex:
            fo += f"; exposed {ex[0]['exposed']}/{ex[0]['of']}"
        if r["placed_in_blast"]:
            fo += "; run-up placements " + ",".join(
                f"{'IN' if p['in_blast'] else 'out'}({p['outside_blast']}/{p['legal']} out)" for p in r["placed_in_blast"])
        print(f"| {r['tag'][4:]} | {r['seed']} | {r['round']} | {r['loser']} {r['board']} | {en} | {acts} | "
              f"{r['extra']} / {r['needed']} | {r['fatal_phase'].split('.')[-1] if r['fatal_phase'] else '-'} | {fs} | {r['fatal_lost']} | {fo} |")
    print()
    for cell in sorted({r["tag"] for r in rows}) + ["all"]:
        rs = [r for r in rows if cell in ("all", r["tag"])]
        by = collections.Counter(r["fatal_cause"] for r in rs)
        boards = collections.Counter(r["board"] for r in rs)
        print(f"{cell}: losses {len(rs)}, boards {dict(boards)}, fatal cause {dict(by.most_common())}, "
              f"safe line (extra >= needed) {sum(1 for r in rs if r['safe_spare'])}")
    src = collections.Counter(s for r in rows for s in r["fatal_srcs"])
    print("fatal sources:", dict(src.most_common()))
    print("energy left at the action phase's end (sum, the 2 phases):",
          collections.Counter(sum((x or {}).get("energy", 0) for x in r["acts"].values()) for r in rows))
    ew = collections.Counter()
    for r in rows:
        ew.update(r["elsewhere"])
    print("energy spent off nodes in those phases (all losses):", dict(ew.most_common()))
    pb = [p for r in rows for p in (r["placed_in_blast"] or [])]
    print("corruption deaths: run-up placements inside the blast", sum(p["in_blast"] for p in pb), "of", len(pb),
          "; of those with a legal hex outside it", sum(1 for p in pb if p["in_blast"] and p["outside_blast"]),
          "; my legal count == the prompt's", sum(1 for p in pb if p["legal"] == p["options"]), "of", len(pb))
    pl = [p for r in rows for p in r["loser_placements"]]
    print("loser's placements, whole game:", len(pl), "; into a threatened city's reach",
          sum(1 for p in pl if p.get("chosen_threatened")), "; of those with a safe legal hex",
          sum(1 for p in pl if p.get("chosen_threatened") and p.get("safe")))
    # corruption deaths with a node-safe placement line: some run-up placement went into the blast while a legal
    # hex outside it existed (one node there would have survived the corruption)
    cs = [r for r in rows if r["foresee"]]
    print("corruption deaths with a node-safe placement:", sum(
        1 for r in cs if any(p["in_blast"] and p["outside_blast"] for p in (r["placed_in_blast"] or []))), "of", len(cs))
    ex = [c for r in rows for c in (r["foresee"] or []) if "exposed" in c]
    print("corruption deaths: nodes within 1 of the city at the last action phase's end",
          sum(c["exposed"] for c in ex), "of", sum(c["of"] for c in ex),
          "; seats with every node exposed", sum(1 for c in ex if c["exposed"] == c["of"]), "of", len(ex))
    fo = [c["adv_before"] for r in rows for c in (r["foresee"] or []) if "city" in c]
    print("fatal corrupting cities, adversary ambassadors + marker at the prior action phase's end:",
          dict(sorted(collections.Counter(fo).items())))
    print("fatal hit took every node held at the end of round R-1:",
          sum(1 for r in rows if r["end_nodes"].get(r["round"] - 1) == r["fatal_lost"]))
    tot = collections.Counter()
    for r in rows:
        tot.update(r["all_removals_loser"])
    print("all removals of the losing seat, whole game:", dict(tot.most_common()))
    unat = collections.Counter(u for r in rows for u in r["unattributed"])
    print("unattributed:", dict(unat.most_common()))


if __name__ == "__main__":
    main()

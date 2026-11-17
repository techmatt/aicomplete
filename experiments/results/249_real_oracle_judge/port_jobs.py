"""249: ports 248's hand-read job table (experiments/results/248_standard_actions_probe/price_cards.py, JOBS) into
scripts/design_check_jobs.json, the job-parity check's data. Each card's other effects and use cost stay expressions
over the job units (`A["amb"]` -> `amb`), so an anchor change moves the check.

    python experiments/results/249_real_oracle_judge/port_jobs.py

A record, not a generator: later items edit the json by hand (252 on), and a rerun would overwrite their rows.
"""
import ast
import json
import os
import re

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
SRC = os.path.join(ROOT, "experiments/results/248_standard_actions_probe/price_cards.py")

# 248's job name -> design_check.json's standard row name
ROW = {"Liberate a city (exempt)": "Liberate a city", "Claim a site: station": "Claim a station",
       "Claim a site: factory": "Claim a factory"}


def expr(node, text):
    s = ast.get_source_segment(text, node)
    s = re.sub(r'A\["(\w+)"\]', r"\1", s)
    s = re.sub(r"\bTAGS\b", "tags", s)
    s = re.sub(r"\bU\b", "uses", s)
    try:
        return ast.literal_eval(s)
    except (ValueError, SyntaxError):
        return s


def main():
    text = open(SRC, encoding="utf-8").read()
    tree = ast.parse(text)
    jobs = {}
    for node in tree.body:
        if isinstance(node, ast.Assign) and getattr(node.targets[0], "id", "") == "JOBS":
            for job in node.value.elts:
                name, unit, entries = job.elts
                name = ast.literal_eval(name)
                cards = []
                for e in entries.elts:
                    parts = e.elts
                    card = {"name": ast.literal_eval(parts[0]), "units": expr(parts[1], text),
                            "other": expr(parts[2], text), "use": expr(parts[3], text)}
                    note = parts[4]
                    if isinstance(note, ast.JoinedStr):
                        note = re.sub(r"\{(\w+)\}", r"\1", ast.get_source_segment(text, note)[2:-1])
                    else:
                        note = ast.literal_eval(note)
                    card["note"] = note.replace("TAGS", "5")
                    if len(parts) > 5:
                        card["uses"] = ast.literal_eval(parts[5])
                    if card["name"].startswith("@"):
                        card["power"] = True
                        card["name"] = card["name"][1:]
                    cards.append(card)
                jobs[ROW.get(name, name)] = {"unit": ast.literal_eval(unit), "cards": cards}
        if isinstance(node, ast.Assign) and getattr(node.targets[0], "id", "") == "A":
            units = {k.arg: ast.literal_eval(k.value) for k in node.value.keywords}
            # a unit design_check's anchors hold goes by the anchor's name, so the anchor moves it
            named = {"energy": "energy", "insight": "insight", "card": "card", "amb": "ambassadors", "chip": "chips",
                     "agent": "agent", "troop": "troop", "am": "antimatter", "cube": "research_cubes", "sway": "sway",
                     "node": "node", "city": "city_liberated", "botnet": "botnet", "compute": "compute",
                     "harden": "harden", "credit": "credits", "factory": "factory", "station_owned": "station_owned"}
            units = {k: named.get(k, v) for k, v in units.items()}
    # 249: Liberate's facility cases, which 248 priced in its summary only: the cards free the site (neutral)
    jobs["Liberate a station"] = {"unit": "station freed", "card_outcome": "neutral", "cards": [
        {"name": "Reclamation Crew", "units": 1, "other": 0, "use": 0, "note": "free a corrupted station"}]}
    jobs["Liberate a factory"] = {"unit": "factory freed", "card_outcome": "neutral", "cards": [
        {"name": "Back-Channel Talks", "units": 2, "other": "2 * sway", "use": 0,
         "note": "free a corrupted factory and 1 next to it; sway 2"}]}
    out = {
        "note": "249: the job-parity check's data (design_check.py check_jobs), ported from 248's hand-read price "
                "table (experiments/results/248_standard_actions_probe/price_cards.py). Per standard row (by name): "
                "the cards and board powers that do its job, each card's job units, its other effects and an "
                "Action's use cost as expressions over `units` (a number, or a design_check anchor's name), "
                "`uses` (rounds of an Action or trigger) and `tags` (a tag count at play). A card's pay is read "
                "from content (credits + 4 x (keep + play insight) + printed energy, chips, antimatter, cubes). "
                "Unit price = (pay + uses x use - other) / units. A power costs its use, once.",
        "uses": 4,
        "tags": 5,
        "keep_insight": {"1": 1, "2": 3},
        "units": units,
        "jobs": jobs,
    }
    path = os.path.join(ROOT, "scripts/design_check_jobs.json")
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(out, f, indent=1, ensure_ascii=False)
        f.write("\n")
    print(path, len(jobs), "jobs", sum(len(j["cards"]) for j in jobs.values()), "rows")


if __name__ == "__main__":
    main()

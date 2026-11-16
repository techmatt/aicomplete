"""248 price check: each standard action against the cards and powers that do its job, at the unit anchors.

Reads each card's printed costs from content/cards.json (and the deck from configs/baseline.json), so a reprice
moves the table; the job units and other effects per card are hand-read from its text (docs/cards.md) and listed
below with the reading used. Prints the per-job detail as Markdown.

    python experiments/results/248_standard_actions_probe/price_cards.py [--out FILE]

Anchors: docs/card-guidelines.md *Unit anchors*; scripts/design_check.json extra_anchors; values.json where none.
pay = credits + 4 x (keep + play insight) + other printed costs at the anchors (energy 4.5, chip 6, antimatter 18,
cube 6). Unit price = (pay - other effects at the anchors) / job units: what the card charges per unit of the job.
Repeatable cards: 4 rounds of uses (an Action 4 times, a trigger at 1 card of its tag a round), use costs added.
"""
import json
import statistics
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
A = dict(energy=4.5, insight=4, card=5, amb=8, chip=6, agent=5, troop=5, am=18, cube=6, sway=4, node=8,
         city=45, botnet=16, credit=1, compute=2, harden=3, move=2, project=24, firewall=5, jam=0.7,
         shelter=4, follower=2.5, cr_inc=4, ins_inc=12, en_inc1=14, en_inc2=9, chip_inc=19,
         station_owned=20, factory=19, station_freed=15, factory_freed=6)
KEEP = {1: 1, 2: 3}
TAGS = 5  # a tag count at play for per-tag effects (values.json rates.tag_count)
USES = 4  # rounds of an Action or a trigger

cards = {c["id"]: c for c in json.load(open(ROOT / "content/cards.json", encoding="utf-8"))}
by_name = {c["name"]: c for c in cards.values()}
base = json.load(open(ROOT / "configs/baseline.json", encoding="utf-8"))
deck = set(next(m for m in base["modules"] if m["module"] == "cards")["cards"])


def pay_of(c):
    p = c.get("cost", 0) + 4 * (KEEP[c["tier"]] + c.get("insight", 0))
    p += A["energy"] * c.get("energy", 0) + A["chip"] * c.get("chips", 0)
    p += A["am"] * c.get("antimatter", 0) + A["cube"] * c.get("research_cubes", 0)
    return p


def cost_str(c):
    bits = [f"{c.get('cost', 0)} cr"]
    for k, n in (("energy", "en"), ("chips", "ch"), ("antimatter", "am"), ("research_cubes", "rc")):
        if c.get(k):
            bits.append(f"{c[k]} {n}")
    bits.append(f"T{c['tier']} keep {KEEP[c['tier']]}, play {c.get('insight', 0)} ins")
    return ", ".join(bits)


# Each job: (std name, std give, std units, unit name, [entries]).
# An entry: (card name, job units, other effects value, use cost per use (Action), note)
# job units and other effects are the whole card over USES rounds where it repeats.
U = USES
JOBS = [
    ("Place a node", "node", [
        ("Relay Mast Rollout", 2, 2 * A["card"], 0, "place 2; draw 2"),
        ("Aegis Grid Lockdown", 3, A["shelter"], 0, "place 3; shelter (values 4)"),
        ("National Grid Buildout", 3, 2 * A["en_inc1"], 0, "place 3; +2 energy income 28"),
        ("Beamed Power Constellation", 3, 6 * A["energy"], 0, "place 3; 1 energy a node (~6 nodes) 27"),
        ("@Buy In (Builder)", 1, 0.72 * A["chip"], 2 * A["energy"], "2 energy; chip by a fresh factory (72%)"),
        ("@Accord (Protector)", 1, 0.72 * A["amb"], 2 * A["energy"], "2 energy; ambassador by a fresh city (72%)"),
        ("@Peer Review (Researcher)", 1, 0.72 * A["insight"], A["cube"], "1 cube; insight by a fresh station (72%)"),
    ]),
    ("Remove agents", "agent", [
        ("Eviction Worm", 3, 4 * A["compute"], 0, "3 agents; 4 compute 8"),
        ("Diplomatic Expulsion", 4, A["sway"], 0, "4 agents; sway 1"),
        ("Counterinsurgency Campaign", 3, 2 * A["cr_inc"], 0, "3 agents; +2 credit income 8"),
        ("Headquarters Raid", 5, 4 * A["card"], 0, "5 agents; draw 4"),
        ("Amnesty Decree", 4, 2 * A["amb"], 0, "clear 4 by a corrupted city; 2 ambassadors placed 16"),
        ("Ceasefire Monitors", 3, 2 * A["amb"], 0, "3 agents (or 1 troop); 2 ambassadors 16"),
        ("Honeypot Relay", 2, A["card"] + 2 * 3, 0, "up to 2 now; draw 1; 3 cr a removal event (~2 more) 6"),
        ("Agency Field Station", U, 2 * A["card"], 0, "1 agent a Security play (4 in 4 rounds); draw 2"),
        ("Joint Task Force", 2 * U, 2 * A["card"], 0, "2 agents an other player's Security play (4 fires); draw 2"),
        ("Directed-Energy Array", 3 * U, 0, 2 * A["energy"], "Action 2 energy: 3 agents, x4"),
        ("Field Assay Team", 2 * U, 0, A["cube"], "Action 1 cube: 2 agents, x4"),
        ("Air Gap", 3, A["firewall"], 0, "3 agents; firewall ~5"),
        ("Killswitch Directive", 4, A["project"], 0, "4 agents; discard a project 24"),
        ("Lure Network", 3 * U, 2 * A["card"], 0, "Action free: up to 3, x4; draw 2"),
        ("Liberation Accords", 6, 4 * A["amb"], 0, "6 agents; 4 ambassadors 32"),
        ("Martial Cordon", 6, -A["sway"], 0, "6 agents; sway 1 Discord -4"),
        ("Kettle Operation", 6, -A["sway"], 0, "up to 6 agents or troops; Discord -4"),
        ("Surgical Strike", 4, -A["sway"], 0, "4 agents (or 3 troops); Discord -4"),
        ("Kinetic Interdiction", 6, 2 * A["troop"] - A["sway"], 0, "6 agents; 2 troops 10; Discord -4"),
        ("Deep State Ledger", 3, 3 * A["ins_inc"], 0, "3 agents; +3 insight income 36"),
        ("Mass Defection", 5, 5 * A["amb"] + 2 * A["sway"], 0, "convert 5 (ambassadors where they stood 40); sway 2"),
        ("Double Agents", 3, 3 * A["amb"] + 2 * A["troop"], 0, "convert 3 (24); 2 troops 10"),
        ("Open Source Uprising", 4, 4 * A["amb"] + 2 * A["card"] + A["sway"], 0, "convert 4 (32); draw 2; sway 1"),
    ]),
    ("Remove a troop", "troop", [
        ("Breach and Clear", 1, 2 * A["card"], 0, "1 troop; draw 2"),
        ("Surgical Strike", 3, -A["sway"], 0, "3 troops (or 4 agents); Discord -4"),
        ("Double Agents", 2, 3 * (A["agent"] + A["amb"]), 0, "2 troops; convert 3 (39)"),
        ("Decapitation Strike", 3, A["botnet"], 0, "3 troops; botnet 16"),
        ("Kinetic Interdiction", 2, 6 * A["agent"] - A["sway"], 0, "2 troops; 6 agents 30; Discord -4"),
        ("Annihilation Lance", 3, 6 * A["agent"] - 2 * A["sway"], 0, "~1 on it + 2 near; ~6 agents 30; Discord -8"),
        ("Kettle Operation", 2, 4 * A["agent"] - A["sway"], 0, "~2 troops (p90 3 a country) + 4 agents 20; -4"),
        ("@Fortify (Builder)", 1, A["harden"], A["chip"], "1 chip: 1 troop; harden 3"),
        ("@Disrupt (Researcher)", 2, 0, A["cube"], "1 Cyber sacrifice + 1 cube: up to 2 (sacrifice unpriced)"),
    ]),
    ("Lift Fallout", "Fallout lifted", [
        ("Field Sensor Grid", 3, 3 * A["card"], 0, "lift 3 near; draw 3"),
        ("Fusion Startup Stake", 1, A["en_inc1"] + 3 * A["cr_inc"], 0, "lift 1; +1 energy, +3 credit income 26"),
        ("Pumped Hydro Storage", 2, 5 * A["cr_inc"], 0, "lift 2 near; +5 credit income 20"),
        ("Shared Grid Accord", 1, 0.75 * 2 * A["energy"] + 2 * A["sway"], 0, "lift 1; gift 2 energy 6.75; sway 2"),
        ("Open-Weights Release", 4, 0.75 * 6 * A["insight"], 0, "lift 4 near; gift 6 insight 18"),
    ]),
    ("Gain a player ambassador", "ambassador", [
        ("Envoy Briefings", 2, 3 * A["card"], 0, "2; draw 3"),
        ("Fact-Check Network", 2, 5 * A["jam"], 0, "2; typed jam 5 (3.5)"),
        ("Baseload Contract", 2, 2 * A["en_inc1"], 0, "2; +2 energy income 28"),
        ("Consular Pouch", TAGS, 0, 0, f"1 a Diplomacy tag ({TAGS})"),
        ("Mutual Aid Pact", 3, 0.75 * 3 * A["insight"], 0, "3; gift 3 insight 9"),
        ("Liaison Corps", 3, 4 * A["move"], 0, "3; move 4 (8)"),
        ("Summit Caucus", 3, 6 * A["move"], 0, "3; consolidate 6 (12)"),
        ("Peace Conference", 3, 8 * A["sway"], 0, "3; sway 8 (32)"),
        ("Trade Delegation", 2, 3 * A["cr_inc"], 0, "2; +3 credit income 12"),
        ("Ceasefire Monitors", 2, 3 * A["agent"], 0, "2; 3 agents 15"),
        ("Holographic Assembly Hall", 2 * U, 2 * U * A["sway"], 2 * A["energy"], "Action 2 energy: 2 + sway 2, x4"),
        ("Accelerator Diplomacy", 2, 5 * A["sway"], 0, "2; sway 5 (20)"),
        ("Antimatter Accord", 6, 4 * A["sway"], 0, "6; sway 4 (16)"),
        ("Exodus Corridors", 4, 6 * A["move"], 0, "4; gather 6 (12)"),
        ("Rally the Allies", 5, 3 * A["move"] + 2 * A["sway"], 0, "5; gather 3 (6); sway 2"),
        ("Liberation Accords", 4, 6 * A["agent"], 0, "4; 6 agents 30"),
        ("Embassy of Exiles", 3 + 2 * U, 2 * U * A["agent"], 0, "3; Action convert 2 (agents 10 + 2 placed), x4"),
    ]),
    ("Take down a botnet", "botnet", [
        ("Botnet Takedown", 1, 6 * A["compute"], 0, "botnet; 6 compute 12"),
        ("Singularity Exploit", 1, A["project"], 0, "botnet; discard a project 24"),
        ("Decapitation Strike", 1, 3 * A["troop"], 0, "botnet; 3 troops 15"),
    ]),
    ("Buy a city's energy", "energy", [
        ("Capacitor Discharge", TAGS, 0, 0, f"1 a Power tag ({TAGS})"),
        ("Dispatch Control Room", 2 + U, 0, 0, "2 when built; 1 a Power play (4)"),
        ("Substation Hub", 2 + 2, 0, 0, "2 when built; 1 an other player's Power play (~2)"),
        ("Collateralized Grid", 2 + 2 * 2, 0, 0, "2; +2 income at tier 2 (9 = 2 energy each)"),
        ("Wartime Retooling", 2 * 14 / 4.5, 4 * A["card"], 0, "+2 energy income (28 = 6.2 energy); draw 4"),
        ("National Grid Buildout", 2 * 14 / 4.5, 3 * A["node"], 0, "+2 energy income (6.2); 3 nodes 24"),
        ("Baseload Contract", 2 * 14 / 4.5, 2 * A["amb"], 0, "+2 energy income (6.2); 2 ambassadors 16"),
        ("Peaker Turbines", 14 / 4.5, A["chip_inc"], 0, "+1 energy income (3.1); +1 chip income 19"),
        ("Faraday Yard", 14 / 4.5, 3 * A["harden"], 0, "+1 energy income (3.1); harden 3 (9)"),
        ("Tidal Barrage", 2 * 14 / 4.5, 0, 0, f"+1 a 2 Power tags (+2 at {TAGS}: 6.2)"),
        ("Dyson Swarm Tranche", 10 + 2 * 2, 0, 0, "10; +2 income at tier 2 (4)"),
        ("Fusion Commons", 6, 3 * A["card"] + 3 * A["sway"], 0, "6; draw 3; sway 3"),
        ("Containment Reactor", 4 * U, U * A["sway"], A["am"], "Action 1 antimatter: 4 + sway 1, x4"),
    ]),
    ("Turn a cube into insight", "insight", [
        ("Credential Harvest", TAGS, 0, 0, f"1 a Cyber tag ({TAGS})"),
        ("Cryo-Compute Burn", TAGS, 0, 0, f"1 a Science tag ({TAGS})"),
        ("Endowed Research Chair", 2 * 3, 0, 0, "+2 insight income (24 = 6 insight)"),
        ("Neural Mirror", 2 * 3, 0, 0, f"+1 a 2 Cyber tags (+2 at {TAGS}: 6)"),
        ("Frontier Lab Charter", 2 * 3, 2 * A["cr_inc"], 0, "+2 insight income (6); +2 credit income 8"),
        ("Telemetry Archive", 6, 0, 0, "3 at 4 Datasets, 2 a Science play: ~2 fires in 4 rounds"),
        ("Hyperscale Botnet", 4 * U, 3 * A["card"], 0, "4 a round x4; draw 3"),
        ("Exascale Training Cluster", 3 * U, 6 * A["cube"], 0, "3 a round x4; 6 cubes 36"),
        ("Interdisciplinary Institute", 3 * U, 3 * A["card"], 0, "1 a tag played (~3 a round) x4; draw 3"),
        ("Recursive Self-Improvement", 3 * 3, 3 * A["card"], 0, "+3 insight income (9); draw 3"),
    ]),
    ("Envoy", "sway to Unity", [
        ("Manifesto Leak", 3, A["follower"], 0, "sway 3; a Follower 2.5"),
        ("Peace Conference", 8, 3 * A["amb"], 0, "sway 8; 3 ambassadors 24"),
        ("Coalition Charter", 6, 0.75 * 4 * A["insight"], 0, "sway 6; gift 4 insight 12"),
        ("Open Assembly", 6, 0, 0, "3 at 2 Followers, 1 a Diplomacy play: ~2 fires in 4 rounds"),
        ("Accelerator Diplomacy", 5, 2 * A["amb"], 0, "sway 5; 2 ambassadors 16"),
        ("Holographic Assembly Hall", 2 * U, 2 * U * A["amb"], 2 * A["energy"], "Action 2 energy: sway 2 + 2 amb, x4"),
        ("Citywide Microgrid", 2 * U, A["city"] - 5 * A["sway"], A["energy"], "Action 1 energy: sway 2, x4; liberates (city less its sway, 25)"),
        ("Antimatter Accord", 4, 6 * A["amb"], 0, "sway 4; 6 ambassadors 48"),
    ]),
    ("Make antimatter", "antimatter", [
        ("Penning Trap Array", 1 + U, 0, A["cube"], "1 when built; Action 1 cube, x4"),
        ("Antiproton Foundry", 1 + U, 0, A["chip"], "1 when built; Action 1 chip, x4"),
        ("Dark Fiber Collider", U, 2 * A["card"], 2 * A["insight"], "Action 2 insight, x4; draw 2"),
        ("Treaty Isotope Bank", U, A["amb"], 8, "Action 8 credits, x4; 1 ambassador"),
        ("Cyclotron Lease", U, 0, 2 * A["energy"], "Action 2 energy, x4"),
        ("Positron Storage Ring", U, 2 * U * A["energy"] + 2 * A["card"], 2 * A["energy"],
         "Action 2 energy x4; 2 energy a Power play (8); draw 2"),
        ("Zero-Day Auction", 2, 6 * A["card"], 0, "2; draw 6"),
        ("Intrusion Lab", 2, 0, 0, "1 at 2 Operatives, 1 a compute removal: ~2 in 4 rounds"),
        ("@Teardown (Builder)", 1, -(10 + 12), A["chip"], "1 chip; your factory gone (-10) and a Fallout (-12)"),
        ("@Meltdown (Researcher)", 1, -15, 2 * A["cube"], "2 cubes; your station gone (-15)"),
    ]),
    ("Liberate a city (exempt)", "city liberated", [
        ("Forensic Audit", 1, 2 * A["card"], 0, "draw 2"),
        ("Root Access", 1, 0.6 * A["botnet"], 0, "a botnet near (x0.6) ~10"),
        ("Citywide Microgrid", 1, 2 * U * A["sway"], A["energy"], "Action 1 energy: sway 2, x4"),
        ("Power to the People", 1, U * TAGS, 0, f"1 cr a Power tag a round ({TAGS}) x4"),
        ("The Nuclear Option", 1, -2 * A["sway"], 0, "Discord 2 (-8)"),
        ("Hostile Buyout", 1, 3 * A["amb"], 0, "3 ambassadors 24"),
        ("Peacekeeping Mission", 1, 1.5 * A["troop"], 0, "up to 3 troops in the country (~1.5)"),
        ("Great Liberation", 1, 2 * A["station_freed"], 0, "+2 more corrupted sites (stations 30; cities: 23 a city)"),
        ("Popular Uprising", 1, 0, 0, "Action: a Follower, or 4 Followers: liberate (once in ~5 rounds)"),
    ]),
    ("Claim a site: station", "owned station", [
        ("Power Purchase Agreement", 1, 0, 0, "2 energy instead of 3 ambassadors"),
        ("Microgrid Lab Compact", 2, 0, 2 * A["energy"], "Action 2 energy: claim, ~2 neutral stations in reach (2 uses)", 2),
        ("@Field Station (Researcher)", 1, 0, 0, "1 Industry sacrifice: build a station (sacrifice unpriced)"),
    ]),
    ("Claim a site: factory", "owned factory", [
        ("Prefab Plant Kit", 1, 3 * A["energy"], 0, "build a factory; 1 energy a Power tag (~3) 13.5"),
    ]),
]

STD = {  # name: (give at anchors, units, give text, get text)
    "Place a node": (6, 1, "6 cr (+ the track step, every route)", "node 8"),
    "Remove agents": (2 * A["amb"], 3, "2 amb 16", "3 agents 15"),
    "Remove a troop": (10, 1, "10 cr", "troop 5"),
    "Lift Fallout": (2 * A["cube"], 1, "2 cubes 12", "Fallout lifted 12"),
    "Gain a player ambassador": (20, 1, "20 cr", "amb 8"),
    "Take down a botnet": (16, 1, "16 cr", "botnet 16 (Sway 0)"),
    "Buy a city's energy": (5, 1, "5 cr", "energy 4.5"),
    "Turn a cube into insight": (A["cube"], 1, "cube 6", "insight 4"),
    "Envoy": (A["amb"], 1, "amb 8", "1 Discord off 4"),
    "Make antimatter": (5 * A["cube"], 1, "5 cubes 30", "antimatter 18"),
    "Liberate a city (exempt)": (5 * A["amb"], 1, "5 amb 40", "city 45 (its 5 Sway in)"),
    "Claim a site: station": (3 * A["amb"], 1, "3 amb 24", "owned station 20"),
    "Claim a site: factory": (2 * A["amb"], 1, "2 amb 16", "factory 19"),
}


def rows_for(job, entries):
    out = []
    for entry in entries:
        name, units, other, use, note = entry[:5]
        if name.startswith("@"):  # a board power: its cost is its use cost, once
            pay, cs, uses, label = 0.0, "power", 1, name[1:]
        else:
            c = by_name[name]
            assert c["id"] in deck, name
            pay, cs, label = pay_of(c), cost_str(c), name
            uses = (entry[5] if len(entry) > 5 else U) if use else 0
        if name.startswith("@"):
            total = use
        else:
            total = pay + uses * use
        unit = (total - other) / units
        out.append(dict(name=label, cs=cs, pay=total, other=other, units=units, unit=unit,
                        ratio=(other + units * A_UNIT[job]) / total if total else float("inf"), note=note,
                        power=name.startswith("@"), marginal=(use / (units / uses) if uses and not name.startswith("@") else None)))
    return out


A_UNIT = {"Place a node": 8, "Remove agents": 5, "Remove a troop": 5, "Lift Fallout": 12,
          "Gain a player ambassador": 8, "Take down a botnet": 16, "Buy a city's energy": 4.5,
          "Turn a cube into insight": 4, "Envoy": 4, "Make antimatter": 18, "Liberate a city (exempt)": 45,
          "Claim a site: station": 20, "Claim a site: factory": 19}


def main():
    lines, summary = [], []
    for job, unit_name, entries in JOBS:
        rows = rows_for(job, entries)
        cardrows = sorted([r for r in rows if not r["power"]], key=lambda r: r["unit"])
        g, n, gt, tt = STD[job]
        std_unit = g / n
        units = [r["unit"] for r in cardrows]
        med = statistics.median(units)
        best = units[0]
        lines.append(f"### {job}: {gt} for {tt}; {std_unit:.1f} per {unit_name}")
        lines.append("")
        show = cardrows
        if len(cardrows) > 8:
            mid = len(cardrows) // 2
            show = cardrows[:4] + [cardrows[mid]] + [cardrows[-1]]
            lines.append(f"{len(cardrows)} cards; the best 4, the median and the worst:")
            lines.append("")
        for r in show:
            m = f"; a use {r['marginal']:.1f} per {unit_name}" if r["marginal"] is not None else ""
            lines.append(f"- **{r['name']}** ({r['cs']}): pay {r['pay']:.1f}; {r['note']}; other {r['other']:.1f}; "
                         f"ratio {r['ratio']:.2f}; **{r['unit']:.1f} per {unit_name}**{m}")
        for r in rows:
            if r["power"]:
                lines.append(f"- *{r['name']}* (power): pay {r['pay']:.1f}; {r['note']}; **{r['unit']:.1f} per {unit_name}**")
        lines.append("")
        summary.append((job, std_unit, len(cardrows), best, med, std_unit / med if med > 0 else float("inf")))
    lines.append("| job | standard per unit | cards | best | median | standard / median |")
    lines.append("|---|---|---|---|---|---|")
    for job, s, k, b, m, q in summary:
        lines.append(f"| {job} | {s:.1f} | {k} | {b:.1f} | {m:.1f} | {q:.2f} |")
    text = "\n".join(lines) + "\n"
    if "--out" in sys.argv:
        open(sys.argv[sys.argv.index("--out") + 1], "w", encoding="utf-8", newline="\n").write(text)
    else:
        sys.stdout.write(text)


if __name__ == "__main__":
    main()

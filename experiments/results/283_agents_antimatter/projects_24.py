"""283: Martial Law, Recruitment Drive and Grand Offensive held at 24 compute a board, one Surge line cut from each
(Matt's 282 pick). Matt (283, message 12916): the bundle rule is soft; a single harm is fine if 24 is fair for it
(strengthen it rather than add a line; no line borrowed from another project). Candidates:

- Martial Law: its factory Surge cut; station Surge 2 -> 3 (one harm: a station capture outright).
- Recruitment Drive: its station Surge cut; city Surge 4 -> 5 (one harm: a city capture at every level).
- Grand Offensive: its city Surge 3 cut: "this round's Infiltrate lines land again" + Surge 2 factory.

Rerunnable: python experiments/results/283_agents_antimatter/projects_24.py, then python scripts/regen.py.
"""
import json

PATH = 'drafts/adversary/projects.json'

INFILTRATE = "Each country: this round's Infiltrate lines land again."
RECALL = ("Each country: every player Ambassador on its hex holding the most (ties: lowest-numbered) goes back to "
          "its owner's supply.")
SURGE_STATION3 = "Surge 3: each country, 3 Agents onto its research station holding the most Agents."
SURGE_FACTORY = "Surge 2: each country, 2 Agents onto its factory holding the most Agents."
SURGE_CITY5 = "Surge 5: each country, 5 Agents onto its city holding the most Agents."

OLD = {'martial_law': 29, 'recruitment_drive': 32, 'grand_offensive': 32}
FLAG = "283 (Matt's 282 pick): held at 24 compute (was {old}), one Surge line cut; {why} (a candidate)"

EDITS = {
    # Matt (283, message 12916): the bundle rule is soft; one harm is fine at 24 if sized to it (strengthen the
    # effect rather than add a line; never borrow another project's line).
    'martial_law': dict(
        text=SURGE_STATION3,
        effects=[{"op": "surge", "site": "station", "count": 3}],
        harms=["adversary pieces placed", "facilities destroyed or corrupted"],
        why="its factory Surge cut; the station Surge 2 -> 3, a capture of the unguarded station whatever it holds "
            "(stations corrupt at 3), a player's own included",
        value="283: give 24 compute a board (48 at 2 countries): one harm sized to the cost (Matt, 283): Surge 3 "
              "corrupts the station holding the most agents in each country outright unless an ambassador cancels one "
              "(stations corrupt at 3), a player's own included, against Station Coup's 24 (a neutral station, "
              "Fallout, 2 track cubes) and 282's ~16 a station capture; its Surge line lands 3 agents a country, 10.0 "
              "at values.json's surge (1.667 an agent, 2 countries), which underprices a corruption.",
        check="283: 1 consequence(s) (small; a third only at cost 18 and up), every board, one-shot. When it matters: "
              "any round. Who plays it: automatic for the adversary (compute step); players answer: ambassadors on "
              "stations; clear station stacks.",
    ),
    'recruitment_drive': dict(
        text=SURGE_CITY5,
        effects=[{"op": "surge", "site": "city", "count": 5}],
        harms=["adversary pieces placed", "facilities destroyed or corrupted"],
        why="its station Surge cut; the city Surge 4 -> 5, a capture of the city holding the most agents at every "
            "level (cities corrupt at 5; 3 from level 1, so through two ambassadors there)",
        value="283: give 24 compute a board (48 at 2 countries): one harm sized to the cost (Matt, 283): Surge 5 "
              "corrupts the city holding the most agents in each country at every level (5 at level 0, 3 from level 1, "
              "so through two ambassadors), against Infiltration Drive's 22 (Surge 4 and Fallout) and the card "
              "anchors' 30 a city; its Surge line lands 5 agents a country, 16.7 at values.json's surge (1.667 an "
              "agent, 2 countries), which underprices a corruption.",
        check="283: 1 consequence(s) (small; a third only at cost 18 and up), every board, one-shot. When it matters: "
              "early is worst. Who plays it: automatic for the adversary (compute step); players answer: drain it; "
              "ambassadors on cities; clear city stacks.",
    ),
    'grand_offensive': dict(
        text=f"{INFILTRATE} {SURGE_FACTORY}",
        effects=[{"op": "adversary_build"}, {"op": "surge", "site": "factory", "count": 2}],
        harms=["adversary pieces placed", "facilities destroyed or corrupted"],
        why="its city Surge 3 cut (Infiltrate again 5 + factory capture 13)",
        value="283: give 24 compute a board (48 at 2 countries): this round's Infiltrate lines again (~5) and a "
              "factory capture (~13, 282's givget.md); its Surge line lands 2 agents a country, 6.7 at values.json's "
              "surge (1.667 an agent, 2 countries); a Surge 2 onto a factory corrupts it on arrival unless an "
              "ambassador cancels one, which the per-agent row underprices (a corruption, a lost node there).",
        check="283: 2 consequence(s) (big; a third only at cost 18 and up), every board, one-shot. When it matters: "
              "mid to late. Who plays it: automatic for the adversary (compute step); players answer: clear factory "
              "stacks; ambassadors on factories.",
    ),
}


def main():
    with open(PATH, encoding='utf-8') as f:
        doc = json.load(f)
    done = 0
    for card in doc['cards']:
        e = EDITS.get(card['id'])
        if not e:
            continue
        old = OLD[card['id']]
        card['cost'] = 24
        card['text'] = e['text']
        card['effects'] = e['effects']
        card['harms'] = e['harms']
        card['value'] = e['value']
        card['check'] = e['check']
        flag = FLAG.format(old=old, why=e['why'])
        flags = [x for x in card.get('flags', []) if not x.startswith('283 ')]
        card['flags'] = flags + [flag]
        done += 1
    assert done == 3, done
    with open(PATH, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(doc, f, indent=1, ensure_ascii=False)
        f.write('\n')
    print(done, 'projects')


if __name__ == '__main__':
    main()

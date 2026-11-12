"""201: writes r89's adversary content changes into drafts/adversary/ (the source of content/), rerunnable: each edit
checks the draft's current value and skips what is already done. Then run `python scripts/content/load_adversary.py`.

    python experiments/results/201_threat_troop_pass/apply_drafts.py

- Musters by stage (`MUSTERS`): each event's Muster line set to its stage's count, dropped at 0 (text too).
- Threat troop landings of 2+ land 1 fewer (`tt`), text too.
- Insight counter items: tier 1 -> 3, tier 2 -> 4.
- A Botnet spread line ("Spread: each country, a Botnet spreads ...") on the `SPREAD` events.
- An agent income line (+1 Agent a country a round) on the `AGENTS` events that lack one.
"""
import json
import re
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = Path(__file__).resolve().parents[3]
D = ROOT / 'drafts' / 'adversary'
MUSTERS = {1: 0, 2: 0, 3: 1}
INSIGHT = {1: 3, 2: 4}
SPREAD = ['mil_s1_long_patrols', 'mil_s1_signal_jamming', 'mil_s2_station_raids', 'mil_s2_fifth_column']
AGENTS = ['mil_s2_station_raids', 'mil_s2_border_war', 'mil_s2_supply_lines', 'mil_s2_forced_march',
          'mil_s3_blitzkrieg', 'mil_s3_naval_blockade']
FLAG = '201 (r89, Matt 2026-11-12): '
SPREAD_TEXT = ('Botnets spread: each country, a Botnet onto its lowest-numbered site next to a Botnet (none: its '
               'lowest-numbered research station).')
INCOME_TEXT = 'Income: +1 Agent a country a round.'
NUM = {1: 'a', 2: '2', 3: '3', 4: '4'}


def read(name):
    return json.loads((D / name).read_text(encoding='utf-8'))


def write(name, data):
    (D / name).write_text(json.dumps(data, indent=1, ensure_ascii=False) + '\n', encoding='utf-8', newline='\n')


def flag(c, note):
    fl = c.setdefault('flags', [])
    if FLAG + note not in fl:
        fl.append(FLAG + note)


def troops_text(n):
    return f'{n} adversary Troop{"s" if n != 1 else ""} land{"s" if n == 1 else ""}'


def threats():
    for name in ('threats_t1.json', 'threats_t2.json'):
        d = read(name)
        for c in d['cards']:
            for opt in c['counters']:
                for it in opt:
                    if it.get('resource') == 'insight' and it['amount'] != INSIGHT[c['tier']]:
                        old = it['amount']
                        it['amount'] = INSIGHT[c['tier']]
                        c['value'] = c.get('value', '').replace(f'{old} insight', f'{it["amount"]} insight')
                        flag(c, f'insight counter {old} -> {it["amount"]} (the new insight income)')
            for e in c['effects']:
                if e['op'] == 'troops' and e.get('count', 1) >= 2 and not any(
                        f.startswith(FLAG + 'troops') for f in c.get('flags', [])):
                    old = e['count']
                    e['count'] = old - 1
                    c['text'] = c['text'].replace(troops_text(old), troops_text(old - 1))
                    flag(c, f'troops {old} -> {old - 1} (fewer troops, each deadlier: they strike nodes)')
        write(name, d)


def events():
    d = read('militarist.json')
    for ev in d['events']:
        st = ev['standing']
        idx = [i for i, e in enumerate(st) if e['op'] == 'troops' and e.get('each_country')]
        if idx:
            want = MUSTERS[ev['stage']]
            old = st[idx[0]].get('count', 1)
            if want == 0:
                del st[idx[0]]
                ev['text'] = re.sub(r' Muster \d: each country, \d Troops? on its lowest-numbered factory \(else city\)\.',
                                    '', ev['text'])
                ev['harms'] = [h for h in ev.get('harms', []) if h != 'troops made']
                flag(ev, f'Muster {old} removed (fewer troops, each deadlier)')
            elif want != old:
                st[idx[0]]['count'] = want
                ev['text'] = ev['text'].replace(f'Muster {old}: each country, {old} Troops', f'Muster {want}: each country, '
                                                f'{want} Troop' + ('s' if want != 1 else ''))
                flag(ev, f'Muster {old} -> {want}')
        if ev['id'] in SPREAD and not any(e['op'] == 'spread_botnets' for e in st):
            st.append({'op': 'spread_botnets', 'count': 1})
            ev['text'] = re.sub(r'( Corrupt: .*)$', ' ' + SPREAD_TEXT + r'\1', ev['text'])
            if 'adversary pieces placed' not in ev.get('harms', []):
                ev.setdefault('harms', []).append('adversary pieces placed')
            flag(ev, 'a Botnet spread line (botnets spread quickly)')
        if ev['id'] in AGENTS and not any(e['op'] == 'increase_income' and e.get('ambassadors') for e in st):
            st.append({'op': 'increase_income', 'ambassadors': 1})
            ev['text'] = re.sub(r'( Corrupt: .*)$', ' ' + INCOME_TEXT + r'\1', ev['text'])
            flag(ev, 'an agent income line (agents grow by stage)')
    write('militarist.json', d)


if __name__ == '__main__':
    threats()
    events()
    print('drafts written; now: python scripts/content/load_adversary.py')

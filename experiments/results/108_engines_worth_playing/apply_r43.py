"""108 (r43): the card changes, applied to r42's content/cards.json (57469c6) and written over content/cards.json.

Run from the repo root: python experiments/results/108_engines_worth_playing/apply_r43.py
Idempotent: it always starts from r42's file in git. Each change carries its one-line reason; `--list` prints them.
The reasons give each card's first step; STRONGER (adopted) then adds 1 card to every engine building's when-built draw
and 2 credits (1 insight or energy) to each fire, so the final numbers are content/cards.json's and cards.md's.
"""
import json
import subprocess
import sys

R42 = '57469c6'


def r42_cards():
    out = subprocess.run(['git', 'show', f'{R42}:content/cards.json'], capture_output=True, check=True)
    return json.loads(out.stdout.decode('utf-8'))


def draw(n):
    return {'op': 'draw', 'count': n}


def gain(res, n):
    return {'op': 'gain', 'resource': res, 'amount': n}


def tokens_here(kind, n):
    return {'op': 'add_tokens', 'kind': kind, 'amount': n, 'here': True}


CHANGES = []  # (id, reason, fn)
CUTS = []  # (id, reason)
NEW = []  # (card, reason)


def change(cid, reason):
    def deco(fn):
        CHANGES.append((cid, reason, fn))
        return fn
    return deco


def when(c):
    return next(p for p in c['passives'] if p['op'] == 'when')


def thresh(c):
    return next(p for p in c['passives'] if p['op'] == 'threshold')


def disc(c):
    return next(p for p in c['passives'] if p['op'] == 'card_discount')


def set_needs(c, n):
    c['needs'] = [[c['needs'][0][0], n]]


# ---------------------------------------------------------------- Science
@change('telemetry_archive', 'arrives with 2 Datasets, so its own Science play fills it at once; the threshold pays 6 insight (was 5); 12c')
def _(c):
    c['cost'] = 12
    c['effects'] = [tokens_here('datasets', 2)]
    thresh(c)['then'][0]['amount'] = 6


@change('citation_engine', 'needs Science 2 (was 3); 7 credits a Science play (was 6); draws 2 when built; 14c')
def _(c):
    c['cost'] = 14
    set_needs(c, 2)
    when(c)['then'][0]['amount'] = 7
    c['effects'] = [draw(2)]


@change('grad_student_pool', 'Science cards 5 credits less (was 4); draws 2 when built; 11c')
def _(c):
    c['cost'] = 11
    disc(c)['amount'] = 5
    c['effects'] = [draw(2)]


@change('foundation_model_campus', 'needs Science 3 (was 4); 3 credits a Science tag (was 2); no longer built over (it buried a Science building it should sit beside); draws 3 when built; 22c')
def _(c):
    c['cost'] = 22
    c['passives'][0]['amount'] = 3
    set_needs(c, 3)
    c.pop('replaces')
    c['effects'] = [draw(3)]


@change('interdisciplinary_institute', 'needs Science 2 (was 3); draws 2 when built; 12c')
def _(c):
    c['cost'] = 12
    set_needs(c, 2)
    c['effects'] = [draw(2)]


@change('data_licensing_deal', 'draws 2 when built; 15c')
def _(c):
    c['cost'] = 15
    c['effects'] = [draw(2)]


@change('recursive_self_improvement', 'no antimatter (an engine, not an ultimate): its 7 goes into credits, 24c')
def _(c):
    c.pop('antimatter')
    c['cost'] = 24


@change('research_royalties', 'needs Science 2 (was 3)')
def _(c):
    set_needs(c, 2)


# ---------------------------------------------------------------- Industry
@change('payroll_automation', 'a Capital feeder (copy had nothing to copy): add 4 Capital to one of your cards and raise credit income by 2; 1 insight (was 2), 12c')
def _(c):
    c['insight'] = 1
    c['effects'] = [{'op': 'add_tokens', 'kind': 'capital', 'amount': 4},
                    {'op': 'production', 'resource': 'credits', 'amount': 2}]


@change('retained_earnings', 'arrives with 2 Capital and adds 2 a play (was 1), so its own play raises income at once; 10c')
def _(c):
    c['cost'] = 10
    c['effects'] = [tokens_here('capital', 2)]
    when(c)['then'][0]['amount'] = 2


@change('vendor_financing', 'adds 5 Capital (was 4): one Retained Earnings fill and change; 10c')
def _(c):
    c['cost'] = 10
    for e in c['effects']:
        if e['op'] == 'add_tokens':
            e['amount'] = 5


@change('materials_hedge_desk', 'adds 8 Capital (was 6): two fills; 12c')
def _(c):
    c['cost'] = 12
    c['effects'][0]['amount'] = 8


@change('liquidity_event', 'cashes Capital at 5 credits (was 4); 7c')
def _(c):
    c['cost'] = 7
    c['effects'][0]['rate'] = 5


@change('lights_out_assembly', '5 credits a rebate (was 4); draws 2 when built; 11c')
def _(c):
    c['cost'] = 11
    when(c)['then'][0]['amount'] = 5
    c['effects'] = [draw(2)]


@change('standardized_tooling', 'Industry cards 5 credits less (was 4); draws 2 when built; 11c')
def _(c):
    c['cost'] = 11
    disc(c)['amount'] = 5
    c['effects'] = [draw(2)]


@change('vertical_monopoly', 'needs Industry 3 (was 4); 3 credits an Industry tag (was 2); draws 3 when built; 16c')
def _(c):
    c['cost'] = 16
    c['passives'][0]['amount'] = 3
    set_needs(c, 3)
    c['effects'] = [draw(3)]


@change('capital_projects_office', 'no longer built over; draws 2 when built; 16c')
def _(c):
    c['cost'] = 16
    c.pop('replaces')
    c['effects'] = [draw(2)]


@change('conglomerate_headquarters', 'needs Industry 2 (was 4); arrives with 2 Capital; 12 credits at 3 (was 10); 14c')
def _(c):
    c['cost'] = 14
    set_needs(c, 2)
    c['effects'] = [tokens_here('capital', 2)]
    thresh(c)['then'][0]['amount'] = 12


@change('orbital_mass_driver', 'no antimatter (its 7 into credits), needs Industry 3 (was 4), no longer built over; draws 3 when built; 28c')
def _(c):
    c.pop('antimatter')
    c.pop('replaces')
    c['cost'] = 28
    set_needs(c, 3)
    c['effects'] = [draw(3)]


@change('economies_of_scale', 'needs Industry 2 (was 3)')
def _(c):
    set_needs(c, 2)


# ---------------------------------------------------------------- Cyber
@change('honeypot_relay', '7 credits (was 6); draws 2 when built; 12c')
def _(c):
    c['cost'] = 12
    when(c)['then'][0]['amount'] = 7
    c['effects'] = [draw(2)]


@change('intrusion_lab', 'arrives with 1 Agent; 2 antimatter at 2 Agents (was 1 at 3); 9c, 1 insight (was 0)')
def _(c):
    c['cost'] = 9
    c['insight'] = 1
    c['effects'] = [tokens_here('agents', 1)]
    thresh(c)['at'] = 2
    thresh(c)['then'][0]['amount'] = 2


@change('darknet_exchange', 'draws 2 when built; 12c')
def _(c):
    c['cost'] = 12
    c['effects'] = [draw(2)]


@change('hyperscale_botnet', 'no longer built over (it buried a better Cyber building), needs Cyber 2 (was 3), 3 insight a round (was 2), draws 2 when built; 14c (was 6)')
def _(c):
    c.pop('replaces')
    c['cost'] = 14
    set_needs(c, 2)
    for p in c['passives']:
        if p['op'] == 'income':
            p['amount'] = 3
    c['effects'] = [draw(2)]


@change('security_ops_floor', '4 insight (was 3); draws 2 when built; 12c')
def _(c):
    c['cost'] = 12
    when(c)['then'][0]['amount'] = 4
    c['effects'] = [draw(2)]


@change('exploit_broker', '6 credits a Cyber play (was 5); draws 2 when built; 12c')
def _(c):
    c['cost'] = 12
    when(c)['then'][0]['amount'] = 6
    c['effects'] = [draw(2)]


@change('command_and_control_grid', 'needs Cyber 2 (was 3); draws 2 when built')
def _(c):
    set_needs(c, 2)
    c['effects'] = [draw(2)]


@change('ransomware_syndicate', 'needs Cyber 2 (was 3)')
def _(c):
    set_needs(c, 2)


@change('zero_day_auction', 'a chain top made much bigger: 1 antimatter per Cyber tag (was per 2); 10c (was 6)')
def _(c):
    c['cost'] = 10
    c['effects'][0]['per'] = 1


# ---------------------------------------------------------------- Power
@change('substation_hub', '2 energy each (was 1); gains 2 energy when built; 12c')
def _(c):
    c['cost'] = 12
    when(c)['then'][0]['amount'] = 2
    c['effects'] = [gain('energy', 2)]


@change('positron_storage_ring', 'needs Power 2 (was 3), no longer built over; gains 2 energy when built; 18c')
def _(c):
    c.pop('replaces')
    c['cost'] = 18
    set_needs(c, 2)
    c['effects'] = [gain('energy', 2)]


@change('dispatch_control_room', '2 energy a Power play (was 1); gains 2 energy when built; 16c')
def _(c):
    c['cost'] = 16
    when(c)['then'][0]['amount'] = 2
    c['effects'] = [gain('energy', 2)]


@change('smart_grid_backbone', 'needs Power 3 (was 4); gains 2 energy when built; 20c')
def _(c):
    c['cost'] = 20
    set_needs(c, 3)
    c['effects'] = [gain('energy', 2)]


@change('dyson_swarm_tranche', 'no antimatter: pays 2 energy instead; needs Power 3 (was 4)')
def _(c):
    c.pop('antimatter')
    c['energy'] = 2
    set_needs(c, 3)


@change('continental_supergrid', 'needs Power 3 (was 5)')
def _(c):
    set_needs(c, 3)


@change('tidal_barrage', 'needs Power 2 (was 3)')
def _(c):
    set_needs(c, 2)


@change('capacity_market', 'needs Power 2 (was 3)')
def _(c):
    set_needs(c, 2)


@change('power_to_the_people', 'needs Power 2 (was 3)')
def _(c):
    set_needs(c, 2)


# ---------------------------------------------------------------- Diplomacy
@change('open_assembly', 'arrives with 1 Follower; sways at 2 (was 3), so its own play sways at once; 10c')
def _(c):
    c['cost'] = 10
    c['effects'] = [tokens_here('followers', 1)]
    thresh(c)['at'] = 2


@change('consulate_tower', 'needs Diplomacy 2 (was 3); 10 credits each (was 6); draws 2 when built; 12c')
def _(c):
    c['cost'] = 12
    when(c)['then'][0]['amount'] = 10
    set_needs(c, 2)
    c['effects'] = [draw(2)]


@change('reconciliation_office', '7 credits (was 5); draws 2 when built; 12c')
def _(c):
    c['cost'] = 12
    when(c)['then'][0]['amount'] = 7
    c['effects'] = [draw(2)]


@change('protocol_office', 'Diplomacy cards 5 credits less (was 4); draws 2 when built; 11c')
def _(c):
    c['cost'] = 11
    disc(c)['amount'] = 5
    c['effects'] = [draw(2)]


@change('orbital_embassy', 'needs Diplomacy 2 (was 3)')
def _(c):
    set_needs(c, 2)


@change('peace_dividend', 'needs Diplomacy 2 (was 3)')
def _(c):
    set_needs(c, 2)


# ---------------------------------------------------------------- Security
@change('agency_field_station', 'removes 2 troops when built; 14c')
def _(c):
    c['cost'] = 14
    c['effects'] = [{'op': 'remove_troops', 'count': 2}]


@change('security_contractor_hq', 'needs Security 2 (was 3); draws 2 when built; 14c')
def _(c):
    c['cost'] = 14
    set_needs(c, 2)
    c['effects'] = [draw(2)]


@change('joint_task_force', 'removes 2 troops when built; 12c')
def _(c):
    c['cost'] = 12
    c['effects'] = [{'op': 'remove_troops', 'count': 2}]


@change('defense_appropriations', 'needs Security 2 (was 3)')
def _(c):
    set_needs(c, 2)


@change('homeland_security_act', 'needs Security 3 (was 4)')
def _(c):
    set_needs(c, 3)


# ---------------------------------------------------------------- cheap cards made bigger
@change('signal_hijack', 'bigger and dearer: removes 3 troops (was 2); 6c (was 3)')
def _(c):
    c['cost'] = 6
    c['effects'][0]['count'] = 3


@change('red_team_audit', 'bigger and dearer: jam 6 (was 4), draws 2 (was 1); 8c (was 4)')
def _(c):
    c['cost'] = 8
    c['effects'][0]['amount'] = 6
    c['effects'][1]['count'] = 2


@change('supply_chain_audit', 'bigger and dearer: jam 6 (was 5), raises credit income by 2 (was 1); 7c (was 4)')
def _(c):
    c['cost'] = 7
    c['effects'][0]['amount'] = 6
    c['effects'][1]['amount'] = 2


@change('threat_hunt', 'bigger and dearer: jam 6 (was 5), drains 8 (was 4); 8c (was 5)')
def _(c):
    c['cost'] = 8
    c['effects'][0]['amount'] = 6
    c['effects'][1]['amount'] = 8


@change('manifesto_leak', 'bigger and dearer: 6 Followers (was 4); 8c (was 5)')
def _(c):
    c['cost'] = 8
    c['effects'][0]['amount'] = 6


for _id in ('sentinel_mesh', 'civic_watch', 'market_watchdog', 'garrison_command'):
    def _mk(cid):
        @change(cid, 'the typed-jam Action building made bigger and dearer: jam 6 a round; 7c')
        def _(c):
            c['cost'] = 7
            c['ability']['effects'][0]['amount'] = 6
    _mk(_id)


# ---------------------------------------------------------------- cuts and new throughput cards
CUTS += [
    ('bridge_loan', 'a 0-credit card that only made credits, which already pile up (Matt\'s ruling)'),
    ('surplus_auction', 'a 0-credit card that turned scarce energy into credits, which already pile up'),
]

NEW += [
    ({
        'id': 'patent_library',
        'name': 'Patent Library',
        'tier': 1,
        'kind': 'building',
        'cost': 8,
        'insight': 1,
        'tags': ['science'],
        'ability': {'cost': [['credits', 3]], 'effects': [draw(2)]},
    }, 'card throughput: spare credits into cards, an Action, so once a round'),
    ({
        'id': 'parts_exchange',
        'name': 'Parts Exchange',
        'tier': 1,
        'kind': 'building',
        'cost': 8,
        'insight': 1,
        'tags': ['industry'],
        'ability': {'cost': [['chips', 2]], 'effects': [draw(2)]},
    }, 'card throughput: spare chips into cards, an Action, so once a round'),
]


def is_engine(c):
    """107's probe's engine card (crates/aic-sim/src/agents/policy.rs, `is_engine`)."""
    ps = c.get('passives', [])
    return any(p['op'] in ('when', 'threshold', 'income_per_tag', 'card_discount') for p in ps) or any(
        e['op'] in ('production_per_tag', 'copy_income') for e in c.get('effects', []))


# Engines are paid in credits, not insight: an engine building's insight to play drops by 1 and its credits rise by
# 3 (insight 2.5 at the table, and credits pile up while insight is the round's scarce attention). Applied after the
# card's own change, to every engine building.
ENGINE_CREDITS = False  # screened on the dev seeds and lost on every cell (write-up): kept off
ENGINE_CREDITS_WHY = 'engines paid in credits: 1 insight less to play, 3 credits more'


# A screened step (write-up, *Screens*): every engine building's when-built draw +1 and each fire's gain +1 (credits +2).
STRONGER = True


STRONGER_FIRES = False  # the fire half: no gain in the probe on the dev seeds, and credits left 46 → 87 a seat there (write-up)
STRONGER_DRAW = True  # the draw half: most of the probe gain on the dev seeds; bigger hands cost the agents (write-up)


def stronger(c):
    for e in c.get('effects', []):
        if e['op'] == 'draw' and STRONGER_DRAW:
            e['count'] += 1
    for p in c.get('passives', []):
        if p['op'] == 'when' and STRONGER_FIRES:
            for t in p['then']:
                if t['op'] == 'gain':
                    t['amount'] += 2 if t['resource'] == 'credits' else 1


# A second screened step: one more card on every engine building's when-built draw.
MORE_DRAW = False


def _fit_consulate(c):
    when(c)['then'][0]['amount'] = 9


# The card fit (npm run cards:fit): Consulate Tower's two-digit gain, printed twice (yours and the other player's),
# took its top box to 6.9 pt; at 9 a fire it fits as r42's 6 did. (Six other top boxes sit under 7.5 pt as they did
# at r42: Agency Directorate, Retained Earnings, Intrusion Lab, Open Assembly, Popular Uprising, Conglomerate HQ,
# Joint Task Force; r43 left their top lines as long as they were.)
FIT = {
    'consulate_tower': ('card fit: you both gain 9 (a single digit, as r42 printed 6)', _fit_consulate),
}


def build():
    cards = r42_cards()
    by = {c['id']: c for c in cards}
    for cid, _, fn in CHANGES:
        fn(by[cid])
    if STRONGER:
        for c in cards:
            if c['kind'] == 'building' and is_engine(c):
                stronger(c)
    if MORE_DRAW:
        for c in cards:
            if c['kind'] == 'building' and is_engine(c):
                for e in c.get('effects', []):
                    if e['op'] == 'draw':
                        e['count'] += 1
    for c in cards:
        if c['id'] in FIT:
            FIT[c['id']][1](c)
    if ENGINE_CREDITS:
        for c in cards:
            if c['kind'] == 'building' and is_engine(c) and c.get('insight', 0) > 0:
                c['insight'] -= 1
                c['cost'] += 3
    cut = {cid for cid, _ in CUTS}
    cards = [c for c in cards if c['id'] not in cut]
    cards += [c for c, _ in NEW]
    return cards


def main():
    if '--list' in sys.argv:
        for cid, why, _ in CHANGES:
            print(f'changed|{cid}|{why}')
        for cid, why in CUTS:
            print(f'cut|{cid}|{why}')
        for c, why in NEW:
            print(f'new|{c["id"]}|{why}')
        if ENGINE_CREDITS:
            for c in build():
                if c['kind'] == 'building' and is_engine(c):
                    print(f'engine-credits|{c["id"]}|{ENGINE_CREDITS_WHY}')
        return
    cards = build()
    out = sys.argv[1] if len(sys.argv) > 1 else 'content/cards.json'
    with open(out, 'w', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps(cards, indent=1, ensure_ascii=False) + '\n')
    print(f'{len(cards)} cards -> {out}')


if __name__ == '__main__':
    main()

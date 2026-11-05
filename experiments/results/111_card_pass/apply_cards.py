"""111: build content/cards.json and the baseline deck from the proposals (out/props.json) plus the consolidation
decisions below. Cuts leave the deck (baseline.json); new cards join content and the deck."""
import copy
import json

props = json.load(open('out/props.json', encoding='utf-8'))
content = json.load(open('content/cards.json', encoding='utf-8'))
by_id = {c['id']: c for c in content}

verdict = {}
proposed = {}
reason = {}
for lab, r in props:
    if not lab or not lab.startswith('propose'):
        continue
    for c in r['cards']:
        cid = c['id']
        if cid == 'credential_harvest_keep':
            cid = 'spoofed_credentials'
        verdict[cid] = c['verdict']
        reason[cid] = c
        if c.get('card'):
            proposed[cid] = c['card']

# Proposal slips the proposers corrected in their notes.
proposed['pumped_hydro_storage']['cost'] = 18
proposed['autonomous_defense_grid']['cost'] = 7
for e in proposed['plant_overdrive']['effects']:
    if e['op'] == 'gain_per_tag':
        e['tag'] = 'industry'

CUTS = {
    'sample_repository', 'data_licensing_deal', 'liquidity_event', 'market_stabilization_board', 'adaptive_malware',
    'zero_click_breach', 'signal_hijack', 'transmission_corridor', 'truth_commission', 'planetary_referendum',
    'bounty_board', 'hardened_bunker_complex', 'peacekeeper_rotation', 'total_containment', 'quick_reaction_force',
}
NEW = ['preprint_server', 'moonshot_grant', 'supplier_network', 'wartime_retooling', 'bug_bounty_board',
       'fusion_research_campus', 'envoy_briefings', 'safehouse_network', 'headquarters_raid']


def card(cid):
    """The card as it will print: the proposal if changed or new, else the current one."""
    if cid in proposed and verdict.get(cid) in ('change', 'new'):
        return copy.deepcopy(proposed[cid])
    return copy.deepcopy(by_id[cid])


def base(cid):
    return copy.deepcopy(by_id[cid])


OVERRIDES = {}


def o(cid):
    def deco(f):
        OVERRIDES[cid] = f
        return f
    return deco


# Per-tag income programs: one shape, one price (reviewers): 6 credits, 1 insight, needs 2.
for cid in ['research_royalties', 'economies_of_scale', 'ransomware_syndicate', 'capacity_market', 'peace_dividend',
            'defense_appropriations']:
    def f(c):
        c['cost'] = 6
        c['insight'] = 1
        return c
    OVERRIDES[cid] = f

# The plain typed-jam Action buildings: one price (reviewers), 5 credits, 0 insight.
for cid in ['sentinel_mesh', 'market_watchdog', 'civic_watch', 'garrison_command']:
    def f(c, cid=cid):
        c = base(cid)
        c['cost'] = 5
        c['insight'] = 0
        return c
    OVERRIDES[cid] = f


@o('dataset_scraper')
def _(c):
    c['cost'] = 6
    c['effects'] = [{'op': 'draw', 'count': 2}, {'op': 'add_tokens', 'kind': 'datasets', 'amount': 1}]
    return c


@o('university_spinout')
def _(c):
    c['cost'] = 16
    return c


@o('forensic_audit')
def _(c):
    c = base('forensic_audit')
    c['cost'] = 20
    c['effects'] = [{'op': 'liberate'}, {'op': 'draw', 'count': 2}]
    return c


@o('materials_hedge_desk')
def _(c):
    c['cost'] = 7
    return c


@o('parts_exchange')
def _(c):
    c['ability']['cost'] = [['chips', 1]]
    c['ability']['effects'] = [{'op': 'draw', 'count': 2}]
    return c


@o('payroll_automation')
def _(c):
    c['cost'] = 8
    for e in c['effects']:
        if e['op'] == 'add_tokens':
            e['amount'] = 2
        if e['op'] == 'production':
            e['amount'] = 4
    return c


@o('collateralized_grid')
def _(c):
    c['cost'] = 6
    return c


@o('vertical_monopoly')
def _(c):
    c['cost'] = 20
    return c


@o('spoofed_credentials')
def _(c):
    c = base('spoofed_credentials')
    c['cost'] = 7
    return c


@o('eviction_worm')
def _(c):
    c['cost'] = 8
    c['effects'] = [{'op': 'remove_ambassadors_country', 'max': 3, 'reach': True}, {'op': 'drain_project', 'amount': 4}]
    return c


@o('logic_bomb')
def _(c):
    c['cost'] = 6
    c['effects'] = [{'op': 'drain_project', 'amount': 6}, {'op': 'draw', 'count': 1}]
    return c


@o('hyperscale_botnet')
def _(c):
    c = base('hyperscale_botnet')
    c['cost'] = 18
    c['passives'] = [{'op': 'income', 'resource': 'insight', 'amount': 4}]
    return c


@o('root_access')
def _(c):
    c = base('root_access')
    c['cost'] = 32
    return c


@o('capacitor_discharge')
def _(c):
    c['cost'] = 8
    return c


@o('relay_mast_rollout')
def _(c):
    c = base('relay_mast_rollout')
    c['cost'] = 8
    c['effects'] = [{'op': 'place_node', 'count': 2, 'free': True}, {'op': 'draw', 'count': 2}]
    return c


@o('power_to_the_people')
def _(c):
    c = base('power_to_the_people')
    c.pop('chips', None)
    return c


@o('nuclear_option')
def _(c):
    c = base('nuclear_option')
    c['cost'] = 14
    for e in c['effects']:
        if e['op'] == 'sway':
            e['amount'] = 1
    return c


@o('back_channel_talks')
def _(c):
    c = base('back_channel_talks')
    c['cost'] = 8
    for e in c['effects']:
        if e['op'] == 'cleanse':
            e['amount'] = 2
    return c


@o('manifesto_leak')
def _(c):
    c['cost'] = 8
    c['insight'] = 0
    for e in c['effects']:
        if e['op'] == 'add_tokens':
            e['amount'] = 1
    return c


@o('hostile_buyout')
def _(c):
    c['cost'] = 36
    return c


@o('popular_uprising')
def _(c):
    c['cost'] = 22
    return c


@o('counterinsurgency_campaign')
def _(c):
    c = base('counterinsurgency_campaign')
    c['cost'] = 12
    c['effects'] = [{'op': 'remove_ambassadors_country', 'max': 3, 'reach': True},
                    {'op': 'production', 'resource': 'credits', 'amount': 3}]
    return c


@o('recruit_operatives')
def _(c):
    c['cost'] = 7
    for e in c['effects']:
        if e['op'] == 'add_tokens':
            e['amount'] = 1
    return c


@o('deep_state_ledger')
def _(c):
    c['cost'] = 6
    for e in c['effects']:
        if e['op'] == 'add_tokens':
            e['amount'] = 2
        if e['op'] == 'production':
            e['amount'] = 3
    return c


@o('kinetic_interdiction')
def _(c):
    c = base('kinetic_interdiction')
    c['cost'] = 0
    c['effects'] = [{'op': 'remove_troops', 'count': 2}, {'op': 'remove_ambassadors_country', 'max': 6, 'reach': True}]
    return c


@o('agency_directorate')
def _(c):
    # cards:fit: "remove 3 adversary ambassadors in reach" printed the box at 5.4 pt; the payoff is intelligence.
    for p in c['passives']:
        if p['op'] == 'threshold':
            p['then'] = [{'op': 'draw', 'count': 3}]
    return c


@o('kinetic_interdiction')
def _(c):
    c = base('kinetic_interdiction')
    c['cost'] = 2
    c.pop('energy', None)
    c['effects'] = [{'op': 'remove_troops', 'count': 2}, {'op': 'remove_ambassadors_country', 'max': 6, 'reach': True}]
    return c


@o('martial_cordon')
def _(c):
    c = base('martial_cordon')
    c['cost'] = 4
    c.pop('keywords', None)
    c['effects'] = [{'op': 'remove_ambassadors_country', 'max': 6}, {'op': 'cordon'}]
    return c


@o('peacekeeping_mission')
def _(c):
    c = base('peacekeeping_mission')
    c['cost'] = 30
    c['effects'] = [{'op': 'liberate', 'troops': 2}]
    return c


# Round 2: the final-deck review (two readers on docs/cards.md after the first r44 read).
def r2(c):
    cid = c['id']
    eff = c.get('effects', [])
    if cid == 'intrusion_lab':  # the drain standard action fires it with no limit a round
        c['passives'][1]['then'] = [{'op': 'gain', 'resource': 'antimatter', 'amount': 1}]
    elif cid == 'security_ops_floor':
        c['cost'] = 12
        c['passives'][0]['then'] = [{'op': 'gain', 'resource': 'insight', 'amount': 1}]
    elif cid == 'honeypot_relay':  # with Checkpoint Network it paid for its own Action
        c['cost'] = 10
        c['passives'][0]['then'] = [{'op': 'gain', 'resource': 'credits', 'amount': 3}]
    elif cid == 'reconciliation_office':
        c['cost'] = 14
    elif cid == 'agency_directorate':
        for p in c['passives']:
            if p['op'] == 'threshold':
                p['at'] = 4
        for e in eff:
            if e['op'] == 'add_tokens':
                e['amount'] = 3
    elif cid == 'positron_storage_ring':  # three effects: the when-built energy goes
        c.pop('effects', None)
    elif cid == 'lights_out_assembly':
        for e in eff:
            if e['op'] == 'draw':
                e['count'] = 2
    elif cid == 'darknet_exchange':
        c['cost'] = 11
        for e in eff:
            if e['op'] == 'draw':
                e['count'] = 2
    elif cid == 'great_liberation':
        c['cost'] = 32
    elif cid == 'exodus_corridors':
        for e in eff:
            if e['op'] == 'gain_ambassadors':
                e['count'] = 4
    elif cid == 'shared_grid_accord':  # pay 2 energy, give 4: circular with the team as one pool
        c['cost'] = 3
        c.pop('energy', None)
        for e in eff:
            if e['op'] == 'other_gain':
                e['amount'] = 2
    elif cid == 'mutual_aid_pact':  # pay credits, give credits: circular
        for e in eff:
            if e['op'] == 'other_gain':
                e['resource'], e['amount'] = 'insight', 3
    elif cid == 'vendor_financing':
        for e in eff:
            if e['op'] == 'other_gain':
                e['resource'], e['amount'] = 'chips', 2
    elif cid == 'recursive_self_improvement':
        c['cost'] = 20
    elif cid == 'breach_and_clear':
        c['effects'] = [{'op': 'remove_troops', 'count': 1, 'reach': True}, {'op': 'draw', 'count': 1}]
    elif cid == 'cryo_compute_burn':
        c['cost'] = 2
        c['energy'] = 1
    elif cid == 'field_sensor_grid':  # the Clear standard action is 2 credits a marker
        c['cost'] = 7
        c['effects'] = [{'op': 'clear_markers', 'count': 3}, {'op': 'draw', 'count': 2}]
    elif cid == 'shadow_broker_feed':
        c['cost'] = 4
        for e in eff:
            if e['op'] == 'draw':
                e['count'] = 2
    elif cid == 'open_weights_release':
        for e in eff:
            if e['op'] == 'other_gain':
                e['amount'] = 6
    elif cid == 'zero_day_auction':  # 6-7 antimatter for 4 sinks of 1
        c['effects'] = [{'op': 'gain', 'resource': 'antimatter', 'amount': 2}, {'op': 'draw', 'count': 6}]
    elif cid == 'moonshot_grant':
        c['cost'] = 22
    elif cid == 'tidal_barrage':
        c['cost'] = 11
    elif cid == 'collateralized_grid':  # it dominated Baseload Contract
        c['cost'] = 10
    elif cid == 'interceptor_battery':  # never kept at r44's read: the agents' 2.2 an ambassador made 4 credits break-even
        c['cost'] = 12
        c['ability']['cost'] = [['credits', 2]]
    elif cid == 'martial_cordon':  # "in one country" twice read as one country
        c['effects'] = [{'op': 'remove_ambassadors_country', 'max': 6}, {'op': 'sway', 'side': 'unity', 'amount': 2}]
    return c


def build():
    out = []
    for c0 in content:
        cid = c0['id']
        c = card(cid) if verdict.get(cid) == 'change' else copy.deepcopy(c0)
        if cid in OVERRIDES:
            c = OVERRIDES[cid](c)
        c = r2(c)
        if c['id'] != cid:
            raise SystemExit(f'id moved: {cid} -> {c["id"]}')
        out.append(c)
    for cid in NEW:
        c = card(cid)
        if cid in OVERRIDES:
            c = OVERRIDES[cid](c)
        out.append(r2(c))
    return out


def main():
    cards = build()
    with open('content/cards.json', 'w', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps(cards, indent=1, ensure_ascii=False) + '\n')
    b = json.load(open('configs/baseline.json', encoding='utf-8'))
    for m in b['modules']:
        for key, v in list(m.items()):
            pass
    text = open('configs/baseline.json', encoding='utf-8').read()
    cfg = json.loads(text)
    found = False

    def walk(x):
        nonlocal found
        if isinstance(x, dict):
            for k, v in x.items():
                if isinstance(v, list) and v and all(isinstance(i, str) for i in v) and 'telemetry_archive' in v:
                    x[k] = [i for i in v if i not in CUTS] + [n for n in NEW if n not in v]
                    found = True
                else:
                    walk(v)
        elif isinstance(x, list):
            for v in x:
                walk(v)
    walk(cfg)
    assert found
    with open('configs/baseline.json', 'w', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps(cfg, indent=2, ensure_ascii=False) + '\n')
    print(len(cards), 'cards in content;', 'cuts', len(CUTS), 'new', len(NEW))


if __name__ == '__main__':
    main()

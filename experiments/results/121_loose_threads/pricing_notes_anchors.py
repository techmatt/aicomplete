"""Normalize every note in docs/card-pricing-notes.json to end with
`anchors: pay N; get M = R.` (Matt's ruling on 119's deferred call 16).

Shape A (already conforming) is left untouched. Shapes B and C (an `anchors: pay ...;`
clause whose last two numbers are the get total and the ratio) are rewritten by regex.
Everything else (E, F, G, X and the three cards repriced in 121) is in HAND.
Run with --write to write the file; without it, a dry run.
"""
import json
import re
import sys

PATH = r'C:\Code\aicomplete-worktrees\aicomplete\121_loose_threads\docs\card-pricing-notes.json'
FINAL = re.compile(r'anchors: pay -?\d+(\.\d+)?; get -?\d+(\.\d+)? = \d+\.\d\d\.$')
NUM = r'-?\d+(?:\.\d+)?'

HAND = {
    # 121's three repriced cards, text given by the lead.
    'peace_conference': "r48 (121): Sway 8 (from 5), two Unity tips at two players now that Sway carries over a tip; 28 cr (from 22); anchors: pay 36; get 44 = 1.22.",
    'popular_uprising': "r48 (121): tier 1 (from tier 2), 24 cr and 1 insight (from 28 cr, 2 insight, needs 3 Diplomacy); Action: add 1 Follower, OR pay 4 Followers to liberate; a liberation by its fifth Action; anchors: pay 32; get 45 = 1.41.",
    'endowed_research_chair': "r48 (121): its 1-credit income cut folded into the price (no card prints 1 credit), 12 cr (from 8); anchors: pay 20; get 24 = 1.20.",
    # E (r42/106): Pay ... = N. Get ... = M, no ratio.
    'market_stabilization_board': "r42 (106): home-country economic threats; anchors: pay 18; get 19.75 = 1.10.",
    'peaker_turbines': "r42 (106): the Builder in rounds 1-4; anchors: pay 23; get 27.9 = 1.21.",
    'hardened_bunker_complex': "r42 (106): Protector early; anchors: pay 21; get 23.4 = 1.11.",
    'bounty_board': "r42 (106): about 5 of the get from a Security seat, plus 10 with Security Contractor HQ; anchors: pay 15; get 17 = 1.13.",
    'total_containment': "r42 (106): plus 23.7 with Security Contractor HQ; anchors: pay 19.5; get 21.6 = 1.11.",
    'orbital_observatory_array': "r42 (106): Researcher (insight to keep is not needed: drawn cards cost no keep); with a reuse each round, +25; anchors: pay 21.5; get 24 = 1.12.",
    'arcology_foundry': "r42 (106): rounds 4-6, Industry seat; anchors: pay 27.5; get 31 = 1.13.",
    'planetary_referendum': "r42 (106): anchors: pay 25.5; get 32 = 1.25.",
    'peacekeeper_rotation': "r42 (106): Protector, 2 troops in reach; plus 20 with Security Contractor HQ; anchors: pay 17; get 25 = 1.47.",
    'fact_check_network': "r42 (106): on an influence threat, a jam of 5 often counters it outright; anchors: pay 8.5; get 12.5 = 1.47.",
    'truth_commission': "r42 (106): anchors: pay 8.5; get 12.5 = 1.47.",
    'sample_repository': "r42 (106): Researcher, early; anchors: pay 10.5; get 13 = 1.24.",
    'quick_reaction_force': "r42 (106): rounds 3-8; with Interceptor Battery and HQ about 58 at the table; anchors: pay 14; get 15 = 1.07.",
    # F (r43/108): Give N ... Get ... M. Buildings: get when built + its own play + 3.5 rounds of its per-round value.
    'signal_hijack': "r43 (108): 8: fewer cheap cards, bigger and dearer; insight to play and keep at 2.5, get at the table (effect ledger, reasoned); anchors: pay 11; get 12 = 1.09.",
    'liquidity_event': "r43 (108): 7: the Capital engine; 8: fewer cheap cards, bigger and dearer; insight to play and keep at 2.5, get at the table (effect ledger, reasoned); anchors: pay 12; get 15 = 1.25.",
    'data_licensing_deal': "r43 (108): 2-3: pays up front, back within 2 rounds; insight to play and keep at 2.5; get 12 when built, then 10 a round at a committed seat (a Dataset seat: Telemetry Archive, Synthetic Data Forge) over 3.5 rounds, back in 1.1 rounds; anchors: pay 22; get 47 = 2.14.",
    'security_contractor_hq': "r43 (108): 4: needs lowered; 2-3: pays up front, back within 2 rounds; insight to play and keep at 2.5; get 12 when built, then 10 a round at a committed seat (Security, removing troops) over 3.5 rounds, back in 0.7 rounds; anchors: pay 19; get 47 = 2.47.",
    'foundation_model_campus': "r43 (108): 6: not built over, so it buries nothing it should not; 4: needs lowered; 2-3: pays up front, back within 2 rounds; insight to play and keep at 2.5; get 16 when built, then 12 a round at a committed seat (Researcher, rounds 4-6) over 3.5 rounds, back in 1.3 rounds; anchors: pay 32; get 58 = 1.81.",
    'orbital_mass_driver': "r43 (108): 5: antimatter only on ultimates; 6: not built over, so it buries nothing it should not; 4: needs lowered; insight to play and keep at 2.5; get 12 when built, then 20.5 a round at a committed seat (Builder, rounds 4-6) over 3.5 rounds, back in 1.4 rounds; anchors: pay 41; get 83.75 = 2.04.",
    'interdisciplinary_institute': "r43 (108): 4: needs lowered; 2-3: pays up front, back within 2 rounds; insight to play and keep at 2.5; get 12 when built and 5 on its own play, then 7.5 a round at a committed seat (any multi-tag seat, mid-game) over 3.5 rounds, back in 0.7 rounds; anchors: pay 22; get 43.25 = 1.97.",
    'conglomerate_headquarters': "r43 (108): 4: needs lowered; 7: the Capital engine; 2-3: pays up front, back within 2 rounds; insight to play and keep at 2.5; get 8 when built and 4 on its own play, then 12 a round at a committed seat (a multi-tag Industry seat, mid-game) over 3.5 rounds, back in 0.8 rounds; anchors: pay 22; get 54 = 2.45.",
    'command_and_control_grid': "r43 (108): 4: needs lowered; 2-3: pays up front, back within 2 rounds; insight to play and keep at 2.5; get 12 when built and 5 on its own play, then 15 a round at a committed seat (any multi-tag seat, mid-game) over 3.5 rounds, back in 0.7 rounds; anchors: pay 28; get 69.5 = 2.48.",
    # G: no pay/get clause.
    'ceasefire_monitors': "r38: a troop off is 11 (15 less its Sway, alignment.troop 0), 7 in a country; value 25 to 21, 10 to 8 at its ratio (0.5); at today's unit anchors 8 cr and 2 insight (keep, play), get 3 ambassadors off, a troop off, 2 ambassadors; anchors: pay 16; get 28 = 1.75.",
    'zero_click_breach': "r42 (106): with Darknet and Hyperscale it pays 10 (+10.8), and Ops Floor adds 7.5; anchors: pay 17.5; get 20.8 = 1.19.",
    'adaptive_malware': "r42 (106): Cyber seat, mid-game; anchors: pay 11; get 14.2 = 1.29.",
    # X: ranges or conditions; a range takes its midpoint, an alternative condition stays in words.
    'collateralized_grid': "r44 (111): priced alone (1.89-2.09 with 3-4 Capital); anchors: pay 22; get 28 = 1.27.",
    'neural_mirror': "r44 (111): +2 insight income at 4 Cyber tags (1.00 at 3 tags, 2.25 at 6); anchors: pay 16; get 24 = 1.50.",
    'open_assembly': "r44 (111): anchors: pay 23; get 33 = 1.43.",
    'orbital_embassy': "r44 (111): the pay leaves out the Diplomacy building it covers; anchors: pay 34; get 62 = 1.82.",
    'exploit_broker': "r44 (111): anchors: pay 20; get 28 = 1.40.",
    'multiyear_defense_budget': "r44 (111): anchors: pay 38; get 48 = 1.26.",
    'manifesto_leak': "r44 (111): priced alone (1.50 with Open Assembly); anchors: pay 12; get 12 = 1.00.",
    'spoofed_credentials': "r44 (111): at 3-4 Cyber tags, priced at the midpoint; anchors: pay 11; get 14 = 1.27.",
    'great_liberation': "r44 (111): at 2 cities and a station (2.45 at 3 cities, 1.36 at 1 city and 2 stations); anchors: pay 55; get 105 = 1.91.",
    'cryo_compute_burn': "r44 (111): at 3-4 Science tags, priced at the midpoint; anchors: pay 10.5; get 14 = 1.33.",
    'field_grant': "r44 (111): with a station in reach (about 1.05 with none in reach yet); anchors: pay 12; get 18 = 1.50.",
    'synthetic_data_forge': "r44 (111): at 4 Actions (about 1.25 at 3); anchors: pay 16; get 24 = 1.50.",
    'safehouse_network': "r44 (111): 2 spends over about 4 rounds, feeders add spends; anchors: pay 22; get 30 = 1.36.",
    'power_to_the_people': "r44 (111): at 4-5 Power tags, priced at the midpoint (recorded); anchors: pay 48; get 63 = 1.31.",
    'capital_projects_office': "r44 (111): at 0.75-1 fires a round, priced at the midpoint; anchors: pay 28; get 50.5 = 1.80.",
    'joint_fabrication_pact': "r44 (111): its own share alone 1.11; anchors: pay 18; get 26.75 = 1.49.",
    'vertical_monopoly': "r44 (111): at 5 tags (2.31 at 6); anchors: pay 36; get 72.5 = 2.01.",
}


def ratio(m, n):
    return '%.2f' % (float(m) / float(n) + 1e-9)


def generic(v):
    """B/C: `<prefix>anchors: pay <expr>; <get breakdown> M = R.`"""
    i = v.find('anchors: pay ')
    if i < 0:
        return None, 'no anchors clause'
    prefix = v[:i]
    body = v[i + len('anchors: pay '):]
    if ';' not in body:
        return None, 'no ; after pay'
    pay_seg, rest = body.split(';', 1)
    nums = re.findall(NUM, pay_seg)
    if not nums:
        return None, 'no pay number'
    n = nums[-1]
    mm = re.search(r'(' + NUM + r')\s*=\s*(\d+(?:\.\d+)?)\.?\s*$', rest)
    if not mm:
        return None, 'clause does not end in M = R'
    m_, r_ = mm.group(1), mm.group(2)
    r = ratio(m_, n)
    if abs(float(r) - float(r_)) > 0.015:
        return None, f'stated ratio {r_} but {m_}/{n} = {r}'
    return f'{prefix}anchors: pay {n}; get {m_} = {r}.', None


def main():
    raw = open(PATH, encoding='utf-8').read()
    old = json.loads(raw)
    new, shape, fails = {}, {}, []
    for k, v in old.items():
        if k in HAND:
            new[k], shape[k] = HAND[k], 'hand'
        elif FINAL.search(v):
            new[k], shape[k] = v, 'A'
        else:
            out, why = generic(v)
            if out is None:
                fails.append((k, why))
                new[k], shape[k] = v, 'fail'
            else:
                new[k], shape[k] = out, 'B/C'
    for k in HAND:
        if k not in old:
            fails.append((k, 'hand entry for a card with no note'))
    # checks
    bad = [k for k, v in new.items() if not FINAL.search(v) or '\n' in v]
    moji = [k for k, v in new.items() if '\ufffd' in v]
    for k, v in new.items():
        m = re.search(r'anchors: pay (' + NUM + r'); get (' + NUM + r') = (\d+\.\d\d)\.$', v)
        if m and ratio(m.group(2), m.group(1)) != m.group(3):
            print('rounding note (left as is):', k, m.group(0), 'computes', ratio(m.group(2), m.group(1)))
            if abs(float(ratio(m.group(2), m.group(1))) - float(m.group(3))) > 0.011:
                bad.append(k + ' (ratio off)')
    counts = {}
    for s in shape.values():
        counts[s] = counts.get(s, 0) + 1
    print('notes', len(old), '->', len(new), 'same keys in order:', list(old) == list(new))
    print('counts', counts)
    print('fails', fails)
    print('bad', bad, 'mojibake', moji)
    changed = [k for k in old if old[k] != new[k]]
    print('changed', len(changed))
    if '--sample' in sys.argv:
        for k in changed[::max(1, len(changed) // 10)][:10]:
            print('\n' + k + '\n  OLD: ' + old[k] + '\n  NEW: ' + new[k])
    if '--write' in sys.argv and not fails and not bad:
        text = '{\n' + ',\n'.join('  ' + json.dumps(k, ensure_ascii=False) + ': ' + json.dumps(v, ensure_ascii=False) for k, v in new.items()) + '\n}\n'
        # the old file must round-trip through the same writer, so the layout is the file's own
        rt = '{\n' + ',\n'.join('  ' + json.dumps(k, ensure_ascii=False) + ': ' + json.dumps(v, ensure_ascii=False) for k, v in old.items()) + '\n}\n'
        assert rt == raw, 'writer does not reproduce the current layout'
        open(PATH, 'w', encoding='utf-8', newline='\n').write(text)
        json.loads(open(PATH, encoding='utf-8').read())
        print('written')


if __name__ == '__main__':
    main()

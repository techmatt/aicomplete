"""111: the card list for the write-up: every changed, new and cut card, its printed text (r43 -> r44), one line of
reasoning and its anchor check. Writes experiments/results/111_card_pass/cards.md."""
import json
import subprocess

props = json.load(open('out/props.json', encoding='utf-8'))
new = {c['id']: c for c in json.load(open('content/cards.json', encoding='utf-8'))}
old = {c['id']: c for c in json.loads(subprocess.run(['git', 'show', 'HEAD:content/cards.json'], capture_output=True,
                                                         encoding='utf-8').stdout)}
faces = json.load(open('web/card/faces.json', encoding='utf-8'))
old_faces = json.loads(subprocess.run(['git', 'show', 'HEAD:web/card/faces.json'], capture_output=True,
                                      encoding='utf-8').stdout)
deck = set(next(m for m in json.load(open('configs/baseline.json', encoding='utf-8'))['modules']
                if m['module'] == 'cards')['cards'])

entries = {}
for lab, r in props:
    if not lab or not lab.startswith('propose'):
        continue
    sec = lab.split(':')[1]
    for c in r['cards']:
        cid = 'spoofed_credentials' if c['id'] == 'credential_harvest_keep' else c['id']
        entries[cid] = dict(c, section=sec)

# Where the consolidation (reviewers, the rulings across sections, cards:fit, Matt's capstone default) moved a
# proposal: the line that replaces the proposer's.
MINE = {
    'research_royalties': ('One per-tag income shape across the six tags (reviewers): 6 credits, 1 insight, needs 2; it read 0.89 at 10.', 'pay 6+4+4=14; get +4-5 credit income 16-20 = 1.14-1.43 (tag payoff, 6 tags 1.71)'),
    'economies_of_scale': ('The shared per-tag income shape: 6 credits (from 10).', 'pay 14; 4-5 Industry tags 16-20 = 1.14-1.43'),
    'ransomware_syndicate': ('The shared per-tag income shape: 6 credits, 1 insight (from 10).', 'pay 14; 4-5 Cyber tags 16-20 = 1.14-1.43'),
    'capacity_market': ('The shared per-tag income shape: 6 credits (from 10).', 'pay 14; 4-5 Power tags 16-20 = 1.14-1.43'),
    'peace_dividend': ('The shared per-tag income shape: 6 credits (from 10); it read 0.89-1.11 at 10.', 'pay 14; 4-5 Diplomacy tags 16-20 = 1.14-1.43'),
    'defense_appropriations': ('The shared per-tag income shape: 6 credits (from 10).', 'pay 14; 4-5 Security tags 16-20 = 1.14-1.43'),
    'sentinel_mesh': ('The plain typed-jam Action buildings at one price (reviewers): 5 credits, 0 insight (from 7 and 1); a jam only pays in rounds its threat type is up.', 'pay 5+4=9; jam 6 about half the rounds, ~3 a round x 4 = 12 = 1.33'),
    'market_watchdog': ('The plain typed-jam Action buildings at one price: 5 credits, 0 insight (from 7 and 1).', 'pay 9; ~3 a round x 4 = 12 = 1.33'),
    'civic_watch': ('The plain typed-jam Action buildings at one price: 5 credits, 0 insight (from 7 and 1).', 'pay 9; ~3 a round x 4 = 12 = 1.33'),
    'garrison_command': ('The plain typed-jam Action buildings at one price: 5 credits, 0 insight (from 7 and 1). The proposed "OR remove 1 troop" was dropped: an A-OR-B Action is a token card\'s choice (guideline 8).', 'pay 9; ~3 a round x 4 = 12 = 1.33'),
    'dataset_scraper': ('Feeder (ruling 20): draw 2 is its main effect, a little below the rate alone; 1 Dataset is the bonus (a Dataset is 4 at Synthetic Data Forge, reviewer).', 'pay 6+4=10; alone draw 2 10 = 1.0; with Forge +4 = 1.4'),
    'university_spinout': ('+8 income read 1.0 at 24; 16 (reviewers: it gave less than Cryptojacking Campaign behind a needs 2).', 'pay 16+4+4=24; get +8 credit income 32 = 1.33'),
    'forensic_audit': ('Liberate capstone, kept a dearer late sink (Matt\'s default, ask 1); 20 credits and a draw 2 rider so it is not the weakest liberation.', 'pay 20+8+4+4 cubes 12=44; get city 45 + draw 2 10 = 55 = 1.25 (under the tier-2 band, recorded)'),
    'materials_hedge_desk': ('Feeder (ruling 20): draw 3 main effect, 2 Capital bonus; 7 credits (a Capital is 4 at Conglomerate HQ, reviewer).', 'pay 7+4+4=15; alone draw 3 15 = 1.0; with HQ +8 = 1.53'),
    'parts_exchange': ('Its Action was the Builder\'s Procure word for word (2 chips: draw 2) and read 0.7; now 1 chip: draw 2, matching Patent Library\'s rate (reviewer: 2 chips for 3 cards read 1.85).', 'pay 9+4+4=17; (2 cards 10 - 1 chip 3) x 3.5 = 24.5 = 1.44'),
    'payroll_automation': ('Feeder (ruling 20): +4 credit income main effect, 2 Capital bonus, 8 credits (reviewers: 3 Capital at HQ went over 1.5).', 'pay 8+4+4=16; alone +4 income 16 = 1.0; with HQ +8 = 1.5'),
    'collateralized_grid': ('Useful alone (ruling 19): +2 energy income main effect; the Capital cash-out at 1 energy each is the bonus, one use (ruling 24); 6 credits (reviewer: cashing gives up the Capital\'s worth on its engine).', 'pay 6+8+4=18; alone 28 = 1.56; with 3-4 Capital, net +3-5 = 1.72-1.83'),
    'vertical_monopoly': ('3 credits per Industry tag a round reached 2.9 at 6 tags at 16; 20 credits, the price of its Science twin Foundation Model Campus (22).', 'pay 20+8+8=36; 5 tags 52.5 + draw 4 20 = 72.5 = 2.01; 6 tags 2.31'),
    'spoofed_credentials': ('Tag payoff over the tier-1 cap at 4 Cyber tags (1.78); 7 credits (from 5).', 'pay 7+4=11; 3-4 Cyber tags 12-16 = 1.09-1.45'),
    'eviction_worm': ('Matt\'s example: "remove 8 in one country" for 10 was a disguised remove-all. Now 3 in reach plus 4 compute, a Cyber angle so it is not Diplomatic Expulsion again (reviewer).', 'pay 8+4+4=16; get 3 ambassadors 15 + 4 compute 8 = 23 = 1.44'),
    'logic_bomb': ('10 compute was a disguised discard; 6 compute and draw 1. The proposed troop rider would veto the card with no troop in reach (feasibility needs every target).', 'pay 6+4+4=14; get 6 compute 12 + draw 1 5 = 17 = 1.21'),
    'hyperscale_botnet': ('Three effects (income, insight discount, draw) and it undercut Recursive Self-Improvement (reviewers): the discount goes, the income rises to 4 insight a round, 18 credits.', 'pay 18+8+4=30; get 4 insight income 48 + draw 3 15 = 63 = 2.1'),
    'root_access': ('Liberate capstone, kept a late sink (Matt\'s default, ask 1): 32 credits (from 40).', 'pay 32+8+4=44; get city 45 + botnet next to it ~12 = 57 = 1.3 (under the tier-2 band, recorded)'),
    'capacitor_discharge': ('Tag payoff over the tier-1 cap at 4 Power tags at the proposed 6; 8 credits (from 10), Plant Overdrive\'s price.', 'pay 8+4=12; 3-4 Power tags 13.5-18 = 1.13-1.5'),
    'relay_mast_rollout': ('"Place 4 nodes" paid track steps a seat can\'t reach (6-8 energy); the proposed "3 nodes free" duplicated Aegis Grid Lockdown (reviewer). Now 2 free nodes and draw 2.', 'pay 8+4+4=16; get 2 free nodes 12 + draw 2 10 = 22 = 1.38'),
    'power_to_the_people': ('Liberate capstone kept a dearer sink (Matt\'s default): the 3-chip extra cost goes, so it is paid in its engine\'s energy; 18 credits stay.', 'pay 18+4+8+4 energy 18=48; get city 45 + 4-5 Power tags x 1 a round 16-20 = 61-65 = 1.27-1.35 (recorded)'),
    'nuclear_option': ('Liberate capstone kept a sink: 14 credits (from 19), Discord sway 1 (from 2) still a price you see.', 'pay 14+7 am+8+4=33; get city 45 - sway 4 = 41 = 1.24 (recorded)'),
    'back_channel_talks': ('"Free 3 corrupted factories in reach" was more than is usually in reach (reviewer); 2, at 8 credits.', 'pay 8+4+4=16; get 2 factories 12 + sway 2 8 = 20 = 1.25'),
    'manifesto_leak': ('Feeder (ruling 20): sway 3 main effect; 1 Follower bonus (a Follower is 6-7.5 at Open Assembly or Popular Uprising, reviewers); 8 credits, 0 insight.', 'pay 8+4=12; alone sway 3 12 = 1.0; with Open Assembly 18 = 1.5'),
    'hostile_buyout': ('Liberate capstone kept the late sink (Matt\'s default): 36 credits (from 42), 3 ambassadors (from 1, a trivial rider).', 'pay 36+8+4=48; get city 45 + your cube 5 + 3 ambassadors 12 = 62 = 1.29 (recorded)'),
    'popular_uprising': ('Liberate capstone kept a sink: 22 credits (from 28; the proposer\'s 16 dropped it below the family).', 'pay 22+8+8=38; ~1.3 liberations 58 = 1.53 (recorded)'),
    'counterinsurgency_campaign': ('Matt named it for ruling 21: its "3 troops in one country" removed every troop and was dead with none. Now 3 ambassadors in reach and +3 income (reviewer).', 'pay 12+4+4=20; get 3 ambassadors 15 + 3 income 12 = 27 = 1.35'),
    'recruit_operatives': ('Feeder (ruling 20): draw 2 main effect; 1 Agent bonus (an Agent is 7-7.5 at Safehouse Network or Intrusion Lab, reviewers); 7 credits.', 'pay 7+4=11; alone 10 = 0.91; with Safehouse +7.5 = 1.59'),
    'deep_state_ledger': ('Feeder (ruling 20): +3 insight income main effect (alone 1.64, a little under the tier-2 rate), 2 Agents bonus; 6 credits.', 'pay 6+8+8=22; alone 36 = 1.64; with Safehouse +15 = 2.32'),
    'kinetic_interdiction': ('"Remove 6 troops in one country" was a disguised remove-all (p90 3). Now the tier-2 sweep: 2 troops in one country and 6 ambassadors in reach; the energy cost went for cards:fit (5.8 pt); 2 credits and an antimatter.', 'pay 2+7 am+8+4=21; get ~1.5 troops 8 + 6 ambassadors 30 = 38 = 1.81'),
    'martial_cordon': ('"10 in one country" was above p90; one of the few tier-2 one-country sweeps at 6 (a median country) with the cordon its name promises, 4 credits (reviewer).', 'pay 4+8+8=20; get ~5.5 removed 27.5 + cordon ~10 = 37.5 = 1.88'),
    'peacekeeping_mission': ('Liberate capstone kept the late sink (Matt\'s default): 30 credits (from 42); its troop twist 2 (3 was a remove-all in a country).', 'pay 30+8+4=42; get city 45 + ~1.5 troops 8 = 53 = 1.26 (recorded)'),
    'agency_directorate': ('Its "3 troops in one country" a fire was a repeated remove-all; "3 ambassadors in reach" printed at 5.4 pt (cards:fit), so the payoff is intelligence: draw 3.', 'pay 16+8+8=32; ~1 fire a round x 15 x 3.5 = 52.5 + its 2 Agents start = 1.7'),
}

R2 = '(final review) '
MINE.update({
    'intrusion_lab': (R2 + 'The drain standard action fires it with no limit a round (5 credits: 3 compute + an Agent); at 2 it gains 1 antimatter (from 2). Antimatter has four 1-a-card sinks.', 'pay 9+4+4=17; ~2 drains a round: 3.5 fires x 7 = 24.5 = 1.44'),
    'security_ops_floor': (R2 + 'Each 5-credit drain paid 4 then 2 insight: a loop capped only by compute on the projects. 1 insight a fire, 12 credits.', 'pay 12+4+4=20; 1.5 fires x 4 x 3.5 = 21 + draw 2 10 = 31 = 1.55'),
    'honeypot_relay': (R2 + 'With Checkpoint Network (4 credits: 2 ambassadors) it paid for its own Action, and the Remove standard action fires it too: 3 credits a fire, 10 credits.', 'pay 10+4+4=18; 1.5 fires x 3 x 3.5 = 15.75 + draw 2 10 = 25.75 = 1.43'),
    'reconciliation_office': (R2 + 'A card a removal fires off the Remove standard action too (1.5-2 a round): 14 credits.', 'pay 14+4+4=22; 1.5 fires x 5 x 3.5 = 26 + draw 2 10 = 36 = 1.65'),
    'agency_directorate': (R2 + 'Its "3 troops in one country" a fire was a repeated remove-all, and "3 ambassadors in reach" printed at 5.4 pt (cards:fit), so the payoff is intelligence; at 4 (1 card a tag was a free chain with Interdisciplinary Institute), arriving with 3 Agents so its own play fills the first.', 'pay 16+8+8=32; first fire 15 + 0.75 fires a round x 15 x 3.5 = 39 = 54 = 1.7'),
    'positron_storage_ring': (R2 + 'Three effects (income, Effect, when-built energy): the when-built 2 energy goes; its own Power play fires the Effect.', 'pay 18+8+8=34; antimatter income 24.5 + self-fire 9 + 3.5 Power plays x 9 = 31.5 = 65 = 1.91'),
    'lights_out_assembly': (R2 + 'The engine rule\'s two-fires draw, as on its siblings: draw 2 (from 3); 53% of the deck is printed at 10+.', 'pay 11+4+4=19; self-fire 5 + 5 x 1 a round x 3.5 = 17.5 + draw 2 10 = 32.5 = 1.71 at a dear-card seat, 1.47 at 0.75'),
    'darknet_exchange': (R2 + 'The family shape: draw 2, 11 credits (it read 1.63 beside 1.45 siblings).', 'pay 11+4+4=19; 5 off x 1 a round x 3.5 = 17.5 + draw 2 10 = 27.5 = 1.45'),
    'great_liberation': (R2 + 'Up to 3 cities for 21 credits read 2.4-3.1 beside 30-36-credit single liberations: 32 credits.', 'pay 32+7 am+8+8=55; 2 cities + a station 105 = 1.91; 3 cities 2.45; 1 city + 2 stations 1.36'),
    'exodus_corridors': (R2 + 'Gain 6 ambassadors is 6 Remove standard actions, a remove-all in reach: 4.', 'pay 5+8+8=21; 4 ambassadors at 1.5 removals 30 + the step 15 = 45 = 2.14'),
    'shared_grid_accord': (R2 + 'Pay 2 energy, another player gains 4 is circular with the team as one pool (ruling 25): another player gains 2 energy and Unity sways 2, 3 credits.', 'pay 3+4+4=11; gift 2 energy 6.75 + sway 2 8 = 14.75 = 1.34'),
    'mutual_aid_pact': (R2 + 'Pay 10 credits, another player gains 8 credits is circular with the team as one pool: the gift is 3 insight.', 'pay 10+4=14; gift 3 insight 9 + 3 ambassadors 12 = 21 = 1.5'),
    'vendor_financing': (R2 + 'Feeder rework (ruling 20) made it +4 credit income and a gift; credits for credits is circular with the team as one pool, so the gift is 2 chips.', 'pay 8+4+4=16; +4 income 16 + gift 2 chips 4.5 = 20.5 = 1.28'),
    'recursive_self_improvement': (R2 + 'Hyperscale Botnet\'s 4 insight a round for 18 still undercut it: 20 credits (from 24).', 'pay 20+8+8=36; +4 insight income 48 + draw 4 20 = 68 = 1.89'),
    'breach_and_clear': (R2 + '"Remove 4 troops in one country" was a disguised remove-all, and 2 in reach still is (a reach holds 0-1): 1 troop in reach and draw 1.', 'pay 4+4=8; 1 troop 5 + draw 1 5 = 10 = 1.25'),
    'cryo_compute_burn': (R2 + 'Under the band beside Credential Harvest: pay 1 energy (from 2), 2 credits.', 'pay 2+4+4.5=10.5; 3-4 Science tags 12-16 = 1.14-1.52'),
    'field_sensor_grid': (R2 + 'The Clear standard action is 2 credits a marker, no limit: the card lost to it (0.43). Clear 3 and draw 2, 7 credits.', 'pay 7+4=11; 3 clears saved 6 + draw 2 10 = 16 = 1.45'),
    'shadow_broker_feed': (R2 + 'It lost to the Clear standard action (0.64): draw 2 (from 1), 4 credits.', 'pay 4+4+4=12; 2 clears saved 4 + draw 2 10 = 14 = 1.17'),
    'open_weights_release': (R2 + 'Clears lose to the standard action; the gift rises to 6 insight, 4 credits. Under the tier-2 band, recorded: Science\'s one tier-2 help card.', 'pay 4+8+8=20; 4 clears saved 8 + gift 6 insight 18 = 26 = 1.3 (recorded)'),
    'zero_day_auction': (R2 + '6-7 antimatter for a deck with four 1-a-card sinks was a face value nobody can spend: 2 antimatter and draw 6, needs 5 Cyber.', 'pay 10+8+4=22; 2 antimatter 14 + draw 6 30 = 44 = 2.0'),
    'tidal_barrage': (R2 + 'At 4-5 Power tags it was Baseload Contract behind a needs 3: 11 credits (ruling 16: the requirement is the price).', 'pay 11+4+4=19; 4-5 tags +2 energy income 28 = 1.47; 6 tags 2.2 (tag payoff)'),
    'collateralized_grid': (R2 + 'Useful alone (ruling 19): +2 energy income main effect, the Capital cash-out the bonus, one use (ruling 24); 10 credits so it no longer dominates Baseload Contract.', 'pay 10+8+4=22; alone 28 = 1.27; with 3-4 Capital 1.89-2.09'),
    'martial_cordon': (R2 + '"10 in one country" was above p90; one of the few tier-2 one-country sweeps at 6, 4 credits; the cordon rider printed "in one country" twice (two readings), so Sway 2 stays.', 'pay 4+8+8=20; ~5.5 removed 27.5 + sway 2 8 = 35.5 = 1.78'),
})
MINE['interceptor_battery'] = ('"1 energy: 2 troops in one country" broke ruling 21 and spent rare energy; now a credit sink with impact: pay 2 credits, remove 2 adversary ambassadors in reach (at 4 credits r44\'s read never kept it: the agents price a removal at 2.2), 12 credits. Renamed Checkpoint Network (id kept).', 'pay 12+4+4=20; (2 ambassadors 10 - 2) x 3.5 = 28 = 1.4')
NEW_FIX = {'moonshot_grant': (R2 + 'A late sink for a Science seat: 6 cubes on a megaproject and draw 2; 22 credits so it is not Field Grant doubled at a dearer rate (ruling 22).', 'pay 22+4+4=30; 6 cubes x 6 36 + draw 2 10 = 46 = 1.53')}
MINE.update(NEW_FIX)

CUT_REASON = {
    'quick_reaction_force': ('An exact near-duplicate of Simulation Cluster (reuse plus draw 2, reviewer); Safehouse Network takes its slot.', 'pay 11+4+4=19; reuse 17.5 + draw 2 10 = 1.45, the same card twice'),
    'signal_hijack': ('After ruling 21 it is Breach and Clear again ("2 troops in reach"), and troops are 0-1 a country: two often-dead copies (reviewers).', 'pay 3+4=7; ~1.5 troops 7.4 = 1.06'),
}


def text(f):
    if not f:
        return ''
    return ' '.join(x for x in (f.get('top_text'), f.get('text')) if x)


def cost(c):
    parts = [f"{c.get('cost', 0)} cr"]
    for k in ('energy', 'chips', 'antimatter', 'research_cubes'):
        if c.get(k):
            parts.append(f"{c[k]} {k.replace('research_cubes', 'cubes')}")
    parts.append(f"{c.get('insight', 0)} ins")
    if c.get('needs'):
        parts.append('needs ' + ', '.join(f'{n} {t}' for t, n in c['needs']))
    return ', '.join(parts)


rows = {'change': [], 'new': [], 'cut': []}
for cid, c in new.items():
    o = old.get(cid)
    if cid not in deck and (o is not None):
        kind = 'cut'
    elif o is None:
        kind = 'new'
    elif json.dumps(o, sort_keys=True) != json.dumps(c, sort_keys=True):
        kind = 'change'
    else:
        continue
    e = entries.get(cid, {})
    why, anchor = MINE.get(cid) or CUT_REASON.get(cid) or (e.get('reason', ''), e.get('anchor', ''))
    was = f"{cost(o)}: {text(old_faces.get(cid))}" if o else ''
    now = f"{cost(c)}: {text(faces.get(cid))}" if kind != 'cut' else ''
    tier = f"t{c['tier']}" + (f" (was t{o['tier']})" if o and o['tier'] != c['tier'] else '')
    rows[kind].append((c['tags'][0], c['name'] + (f" (was {o['name']})" if o and o['name'] != c['name'] else ''), tier,
                       was, now, why, anchor))

out = ['# 111: the card list (r43 -> r44)', '',
       'Generated by `writeup_cards.py` (here) from `proposals.json` (the six proposers and three reviewers, round 1) and the consolidation in `apply_cards.py`, both run from the repo root with `out/props.json` a copy of `proposals.json`. Anchor checks: pay = credits +',
       '4 x (keep + play insight) + other costs at the anchors; get = effects at the anchors (docs/card-guidelines.md, 111).',
       '"recorded": below the tier-2 band on purpose (the liberate capstones stay late sinks, Matt\'s default to ask 1).', '']
for kind, title in (('change', 'Changed'), ('new', 'New'), ('cut', 'Cut (out of the deck, kept in content)')):
    out += [f'## {title} ({len(rows[kind])})', '',
            '| card | tier | r43 | r44 | why | anchor check |' if kind == 'change' else
            ('| card | tier | r44 | why | anchor check |' if kind == 'new' else '| card | tier | r43 | why | anchor check (r43) |'),
            '|---|---|---|---|---|---|' if kind == 'change' else '|---|---|---|---|---|']
    for tag, name, tier, was, now, why, anchor in sorted(rows[kind]):
        cells = [name, tier] + ([was, now] if kind == 'change' else [now] if kind == 'new' else [was]) + [why, anchor]
        out.append('| ' + ' | '.join(x.replace('|', '/') for x in cells) + ' |')
    out.append('')
with open('experiments/results/111_card_pass/cards.md', 'w', encoding='utf-8', newline='\n') as f:
    f.write('\n'.join(out))
print({k: len(v) for k, v in rows.items()})

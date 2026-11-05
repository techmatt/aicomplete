"""110: give every r43 deck card a scene. Reassigns unused pool scenes and adds the new
scenes of batch 6 to tools/card-art/scenes.json. Run once from the repo root."""
import json, os

R = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
F = os.path.join(R, 'tools', 'card-art', 'scenes.json')
s = json.load(open(F, encoding='utf-8'))

# Reused: scene id -> the deck card it now goes on.
REUSE = {
    'founder_pitch': 'university_spinout',
    'policy_institute_hall': 'interdisciplinary_institute',
    'crawler_in_the_stacks': 'patent_library',
    'stacked_container_wing': 'economies_of_scale',
    'arc_furnace': 'plant_overdrive',
    'fund_site_plans': 'capital_projects_office',
    'chip_market_stalls': 'parts_exchange',
    'siphon_pipes': 'cryptojacking_campaign',
    'broker_in_the_cafe': 'exploit_broker',
    'power_auction': 'capacity_market',
    'armored_conduit_trench': 'smart_grid_backbone',
    'envoy_jet': 'trade_delegation',  # not treaty_port_cranes: it flies real nations' flags
    'council_chamber': 'peace_conference',
    'trust_bureau_counters': 'reconciliation_office',
    'walled_embassy': 'protocol_office',
    'shoulder_to_shoulder_shields': 'joint_task_force',
    'strike_team_rooftop': 'quick_reaction_force',
    'triage_ops_table': 'counterinsurgency_campaign',
}

# New: tag -> scene id -> (card, subject).
NEW = {
    'science': {
        'royalties_from_inventions': ('research_royalties', 'A sunlit university workshop of small working inventions on display plinths, a thin ribbon of gold coins trickling from each one into a shared brass tray that a smiling researcher is emptying into a satchel.'),
    },
    'industry': {
        'matched_tool_wall': ('standardized_tooling', 'A long factory wall of identical tool heads, jigs and wrenches hung in neat matching rows, a machinist lifting one down to fit the nearest of a line of identical machines.'),
        'gigafactory_groundbreaking': ('gigafactory_charter', 'A colossal new factory hall rising on an open plain, its roof running out of sight, officials cutting a ribbon at the gate while cranes and trucks swarm the site beyond.'),
        'conglomerate_tower_plaza': ('conglomerate_headquarters', 'A tall glass corporate headquarters at the heart of a business park, a factory, a lab, a warehouse and a shipyard crane around it all joined to the tower by covered walkways, executives crossing the bright plaza.'),
        'cash_into_the_works': ('materials_hedge_desk', 'An armored bank truck backed up to a busy factory loading bay, workers wheeling heavy cases of gold bars off its tail into the plant while a manager waves them in.'),
    },
    'cyber': {
        'padlocked_server_room': ('ransomware_syndicate', 'A server room door wrapped in heavy glowing green chains and a giant padlock, a nervous manager in the corridor handing a briefcase to a masked courier.'),
        'zero_day_bidders': ('zero_day_auction', 'A hushed back-room auction, masked bidders on folding chairs raising paddles while an auctioneer holds up a small glowing green key in a glass case.'),
        'loft_command_console': ('command_and_control_grid', 'A hidden warehouse loft with one operator at a central console, bright green threads fanning out from it through the tall windows to thousands of rooftop devices across the city.'),
    },
    'power': {
        'mountain_reservoir_pipes': ('pumped_hydro_storage', 'A reservoir high on a mountain shoulder above a lower lake, giant pipes running straight down the slope to a turbine hall at the water, white water rushing out below it.'),
        'dispatch_wall_map': ('dispatch_control_room', 'A bright grid dispatch room, operators at curved desks before a huge wall map of power lines drawn as lines of light, one pushing a lever up as a line on the map brightens.'),
    },
    'security': {
        'budget_before_the_dais': ('defense_appropriations', 'A legislative committee room, members at a curved wooden dais raising their hands to approve, crates of military equipment and a model jet set out on the floor before them.'),
        'depot_rows_to_horizon': ('multiyear_defense_budget', 'A vast military depot yard with rows of new armored vehicles stretching to the horizon, an officer and a budget clerk walking the nearest row with a thick bound plan.'),
        'act_on_the_capitol_steps': ('homeland_security_act', "A capitol's broad steps where officials hold up a freshly signed act, guards, emergency crews and power engineers in plain unmarked uniforms, no emblems or crosses, lined up together in the plaza below."),
    },
}

# Each new scene's two cards for the picker: one program or event, one building.
BATCH = {
    'royalties_from_inventions': ['research_royalties', 'patent_library'],
    'matched_tool_wall': ['economies_of_scale', 'standardized_tooling'],
    'gigafactory_groundbreaking': ['gigafactory_charter', 'capital_projects_office'],
    'conglomerate_tower_plaza': ['materials_hedge_desk', 'conglomerate_headquarters'],
    'cash_into_the_works': ['materials_hedge_desk', 'conglomerate_headquarters'],
    'padlocked_server_room': ['ransomware_syndicate', 'exploit_broker'],
    'zero_day_bidders': ['zero_day_auction', 'exploit_broker'],
    'loft_command_console': ['cryptojacking_campaign', 'command_and_control_grid'],
    'mountain_reservoir_pipes': ['pumped_hydro_storage', 'smart_grid_backbone'],
    'dispatch_wall_map': ['capacity_market', 'dispatch_control_room'],
    'budget_before_the_dais': ['defense_appropriations', 'joint_task_force'],
    'depot_rows_to_horizon': ['multiyear_defense_budget', 'quick_reaction_force'],
    'act_on_the_capitol_steps': ['homeland_security_act', 'joint_task_force'],
}

where = {sid: tag for tag, g in s['scenes'].items() for sid in g}
for sid, card in REUSE.items():
    sc = s['scenes'][where[sid]][sid]
    old = sc.pop('card', None)
    was = sc.pop('was', old)
    # `card`, `was`, then the rest, as elsewhere in the pool.
    s['scenes'][where[sid]][sid] = {'card': card, **({'was': was} if was else {}), **sc}

# The scene Capital Injection carried was written for its old name.
for tag, g in s['scenes'].items():
    for sid, sc in g.items():
        if sc.get('card') == 'materials_hedge_desk':
            g[sid] = {'was': 'materials_hedge_desk', **{k: v for k, v in sc.items() if k != 'card'}}

for tag, g in NEW.items():
    for sid, (card, subject) in g.items():
        assert sid not in where, sid
        s['scenes'][tag][sid] = {'card': card, 'subject': subject}

# A scene still suggested for a card 106 or 108 cut takes `was` in its place, as the pool's note says.
def ids(p):
    v = json.load(open(os.path.join(R, 'content', p), encoding='utf-8'))
    rows = v if isinstance(v, list) else v.get('countries', [])
    return {r['id'] for r in rows if isinstance(r, dict) and 'id' in r}
known = set().union(*(ids(p) for p in ['cards.json', 'threats.json', 'alignment.json', 'adversary.json', 'projects.json', 'map.json', 'ais.json']))
retired = []
for tag, g in s['scenes'].items():
    for sid, sc in g.items():
        if sc.get('card') and sc['card'] not in known:
            retired.append(sid)
            g[sid] = {'was': sc['card'], **{k: v for k, v in sc.items() if k not in ('card', 'was')}}
print(len(retired), 'scenes of cut cards now `was`')

s['round'] = 'b6'
s['picker'] = 'scene_batch_6'
s['batch'] = BATCH
s['_note'] = s['_note'].rstrip() + ' A scene with both `card` and `was` was written for `was` and reused (110) on `card`.'

def fmt(v, ind=0):
    """The pool's layout: a flat object or list on one line, anything else one key a line."""
    d = lambda x: json.dumps(x, ensure_ascii=False)
    flat = lambda x: all(not isinstance(y, (dict, list)) for y in (x.values() if isinstance(x, dict) else x))
    if isinstance(v, dict):
        if ind and flat(v):
            return '{ ' + ', '.join(f'{d(k)}: {d(x)}' for k, x in v.items()) + ' }'
        pad = '  ' * (ind + 1)
        return '{\n' + ',\n'.join(f'{pad}{d(k)}: {fmt(x, ind + 1)}' for k, x in v.items()) + '\n' + '  ' * ind + '}'
    if isinstance(v, list):
        return '[' + ', '.join(d(x) for x in v) + ']'
    return d(v)

if __name__ == '__main__':
    import subprocess, sys
    if '--selftest' in sys.argv:
        t = subprocess.run(['git', 'show', '7ba72ab:tools/card-art/scenes.json'], cwd=R, capture_output=True, encoding='utf-8').stdout
        print('layout round-trips:', fmt(json.loads(t)) + '\n' == t)
        sys.exit()
    with open(F, 'w', encoding='utf-8', newline='\n') as f:
        f.write(fmt(s) + '\n')
    with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'reused.json'), 'w', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps(REUSE, indent=1) + '\n')
    print('ok')

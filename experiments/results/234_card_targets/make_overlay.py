"""234: writes targets.overlay, the one-target conversion of every map-touching player card (Matt's rulings,
2026-11-14). Each entry replaces one op list whole; rerun it, then `python scripts/cards/apply_overlay.py
experiments/results/234_card_targets/targets.overlay` and `python scripts/regen.py`.

Tier-1 removals act on the target; tier 2 (and a tier-1 card printing 5 or more) on or next to it; a count
of Fallout tokens (about one a hex) on or next to it. Gain stays your supply; only Amnesty Decree places.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def T(on):
    return {"op": "target", "on": on}


def agents(n, at, **kw):
    return {"op": "remove_ambassadors_country", **kw, "max": n, "at": at}


def troops(n, at):
    return {"op": "remove_troops", "count": n, "at": at}


def lift(n, at):
    return {"op": "lift_fallout", "count": n, "at": at}


def draw(n):
    return {"op": "draw", "count": n}


def gain_amb(n):
    return {"op": "gain_ambassadors", "count": n}


def sway(side, n, why):
    return {"op": "sway", "side": side, "amount": n, "narrative": why}


LIBERATE = T("corrupted_city")

CARDS = {
    "field_sensor_grid": {"effects": [T("fallout"), lift(3, "near"), draw(3)]},
    "open_weights_release": {"effects": [T("fallout"), lift(4, "near"), {"op": "choose_player"},
                                         {"op": "other_gain", "resource": "insight", "amount": 6}]},
    "prefab_plant_kit": {"effects": [T("free_zone"), {"op": "build_factory"},
                                     {"op": "gain_per_tag", "resource": "energy", "tag": "power", "per": 1}]},
    "station_prefab_order": {"effects": [T("corrupted_station"),
                                         {"op": "cleanse", "amount": 1, "one_country": False, "only": "station", "at": "on"}]},
    "eviction_worm": {"effects": [T("agents"), agents(3, "on"), {"op": "drain_project", "amount": 4}]},
    "honeypot_relay": {"effects": [draw(1), T("agents"), {"op": "gather", "max": 2, "pieces": ["agents"], "strike": True}]},
    "shadow_broker_feed": {"effects": [T("fallout"), lift(2, "near"), draw(2)]},
    "signal_hijack": {"effects": [T("troops"), troops(2, "on")]},
    "botnet_takedown": {"effects": [T("botnet"), {"op": "remove_botnet"}, {"op": "drain_project", "amount": 6}]},
    "killswitch_directive": {"effects": [{"op": "discard_project"}, T("agents"), agents(4, "near")]},
    "singularity_exploit": {"effects": [T("botnet"), {"op": "remove_botnet"}, {"op": "discard_project"}]},
    "shared_grid_accord": {"effects": [{"op": "choose_player"}, {"op": "other_gain", "resource": "energy", "amount": 2},
                                       sway("unity", 2, "cooperation"), T("fallout"), lift(1, "on")]},
    "transmission_corridor": {"effects": [{"op": "place_node", "count": 2}, T("piece"),
                                          {"op": "move_ambassadors", "max": 2, "at": "on"}]},
    "fusion_startup_stake": {"effects": [{"op": "production", "resource": "energy", "amount": 1},
                                         {"op": "production", "resource": "credits", "amount": 3}, T("fallout"), lift(1, "on")]},
    "faraday_yard": {"effects": [{"op": "production", "resource": "energy", "amount": 1}, T("facility"),
                                 {"op": "harden", "count": 3, "at": "near"}]},
    "aegis_grid_lockdown": {"effects": [{"op": "shelter", "reach": True}, {"op": "place_node", "count": 3}]},
    "summit_caucus": {"effects": [T("piece"), {"op": "consolidate", "max": 6}, gain_amb(3)]},
    "liaison_corps": {"effects": [gain_amb(3), T("piece"), {"op": "move_ambassadors", "max": 4, "at": "near"}]},
    # clear_city is "remove up to N Agents on or next to the corrupted city": the target (Diplomacy's own verb,
    # design_check's tag story; remove_ambassadors_country reads as Security).
    "amnesty_decree": {"effects": [T("corrupted_city"), {"op": "clear_city", "max": 4},
                                   {"op": "place_ambassadors", "count": 2, "at": "on"}]},
    "ceasefire_monitors": {"effects": [T("adversary"), {"op": "damage", "ambassadors": 3, "troops": 1}, gain_amb(2)]},
    "back_channel_talks": {"effects": [T("corrupted_factory"),
                                       {"op": "cleanse", "amount": 2, "one_country": False, "only": "factory", "at": "near"},
                                       sway("unity", 2, "cooperation")]},
    "liberation_accords": {"effects": [T("agents"), agents(6, "near", unity_per=0), gain_amb(4)]},
    "exodus_corridors": {"effects": [T("land"), {"op": "gather", "max": 6, "pieces": ["ambassadors"]}, gain_amb(4)]},
    "great_liberation": {"effects": [T("corrupted_site"),
                                     {"op": "cleanse", "amount": 3, "one_country": False, "cities": True, "at": "country"}]},
    "bounty_board": {"effects": [T("troops"), {"op": "damage", "troops": 1},
                                 {"op": "gain_per_tag", "resource": "credits", "tag": "security", "per": 1}]},
    "peacekeeper_rotation": {"effects": [T("troops"), {"op": "damage", "ambassadors": 0, "troops": 2},
                                         {"op": "consolidate", "max": 1}]},
    "diplomatic_expulsion": {"effects": [T("agents"), agents(4, "on"), sway("unity", 1, "trust")]},
    "agency_field_station": {"passives": [{"op": "when", "on": {"event": "play", "tag": "security"},
                                           "then": [T("agents"), agents(1, "on")]}]},
    "interceptor_battery": {
        "effects": [T("adversary"), {"op": "push", "max": 3, "pieces": ["agents", "troops"]}],
        "ability": {"effects": [T("adversary"), {"op": "push", "max": 2, "pieces": ["agents", "troops"]}]},
    },
    "breach_and_clear": {"effects": [T("troops"), troops(1, "on"), draw(2)]},
    "hardpoint_lockdown": {"effects": [{"op": "shelter", "reach": True}, T("land"), {"op": "cordon"}]},
    "martial_cordon": {"effects": [T("agents"), agents(6, "near"), sway("discord", 1, "force")]},
    "kinetic_interdiction": {"effects": [T("adversary"), troops(2, "near"), agents(6, "near"), sway("discord", 1, "force")]},
    "annihilation_lance": {"effects": [T("adversary"), {"op": "destroy_all"}, troops(2, "near"),
                                       sway("discord", 2, "collateral")]},
    "total_containment": {"effects": [T("troops"), troops(4, "near"), {"op": "shelter", "reach": True}]},
    "nuclear_option": {"effects": [LIBERATE, {"op": "liberate"}, sway("discord", 2, "collateral")]},
    "power_to_the_people": {"effects": [LIBERATE, {"op": "liberate"}]},
    "hostile_buyout": {"effects": [LIBERATE, {"op": "liberate", "ambassadors": 3}]},
    "forensic_audit": {"effects": [LIBERATE, {"op": "liberate"}, draw(2)]},
    "root_access": {"effects": [LIBERATE, {"op": "liberate", "botnet": True}]},
    "peacekeeping_mission": {"effects": [LIBERATE, {"op": "liberate", "troops": 3}]},
    "pumped_hydro_storage": {"effects": [{"op": "production", "resource": "energy", "amount": 1},
                                         {"op": "gain", "resource": "energy", "amount": 4}, T("fallout"), lift(1, "on")]},
    "joint_task_force": {"passives": [{"op": "when", "on": {"event": "team_play", "tag": "security"},
                                       "then": [T("agents"), agents(2, "on")]}]},
    "counterinsurgency_campaign": {"effects": [T("agents"), agents(3, "on"),
                                               {"op": "production", "resource": "credits", "amount": 2}]},
    "headquarters_raid": {"effects": [T("agents"), agents(5, "near"), draw(4)]},
    "decapitation_strike": {"effects": [T("botnet"), {"op": "remove_botnet"}, troops(3, "near")]},
    "citywide_microgrid": {"effects": [LIBERATE, {"op": "liberate"}]},
    "power_purchase_agreement": {"effects": [T("neutral_station"), {"op": "claim_station"}]},
    "mass_defection": {"effects": [T("agent_city"), {"op": "convert", "max": 5}, sway("unity", 2, "trust")]},
    "double_agents": {"effects": [T("agent_city"), {"op": "convert", "max": 3}, troops(2, "near")]},
    "kettle_operation": {"effects": [T("adversary"), {"op": "gather", "max": 6, "pieces": ["agents", "troops"], "strike": True},
                                     sway("discord", 1, "force")]},
    "air_gap": {"effects": [T("agents"), {"op": "firewall"}, {"op": "damage", "ambassadors": 3}]},
    "rally_the_allies": {"effects": [gain_amb(5), T("land"), {"op": "gather", "max": 3, "pieces": ["ambassadors"]},
                                     sway("unity", 2, "cooperation")]},
    "surgical_strike": {"effects": [T("adversary"), {"op": "damage", "ambassadors": 4, "troops": 3}, sway("discord", 1, "force")]},
    "open_source_uprising": {"effects": [T("agent_city"), {"op": "convert", "max": 4}, draw(2), sway("unity", 1, "cooperation")]},
}

# Actions: the ability's effects (and its OR branch) open with the target; the cost stays.
ABILITY = {
    "popular_uprising": ("or.effects", [LIBERATE, {"op": "liberate"}]),
    # A paid, repeated Action: on or next to (on the target alone it found 0.73 of r103's reach a use, under its
    # 2 energy's 9 at the anchors).
    "directed_energy_array": ("effects", [T("agents"), agents(3, "near")]),
    "microgrid_lab_compact": ("effects", [T("neutral_station"), {"op": "claim_station"}]),
    "embassy_of_exiles": ("effects", [T("agent_city"), {"op": "convert", "max": 2}]),
    "lure_network": ("effects", [T("agents"), {"op": "gather", "max": 3, "pieces": ["agents"], "strike": True}]),
    "firewall_array": ("effects", [T("land"), {"op": "firewall"}]),
    "field_assay_team": ("effects", [T("agents"), {"op": "damage", "ambassadors": 2}]),
}


#: 234's re-pricing at the anchors (write-up, *Pricing*): what the conversion took from each card, at its old ratio.
PRICES = {
    "eviction_worm": 9,
    "counterinsurgency_campaign": 12,
    "diplomatic_expulsion": 8,
    "killswitch_directive": 12,
    "headquarters_raid": 22,
    "liberation_accords": 11,
    "martial_cordon": 3,
    "kinetic_interdiction": 0,
    "joint_task_force": 11,
    "directed_energy_array": 8,
    "back_channel_talks": 11,
    "annihilation_lance": 8,
}


def main():
    lines = ["# 234: one target per map-touching player card (make_overlay.py writes this file)."]
    for cid, cost in PRICES.items():
        lines.append(f"cards.{cid}.cost={cost}")
    for cid, parts in CARDS.items():
        for key, val in parts.items():
            if key == "ability":
                lines.append(f"cards.{cid}.ability.effects={json.dumps(val['effects'], separators=(',', ':'))}")
            else:
                lines.append(f"cards.{cid}.{key}={json.dumps(val, separators=(',', ':'))}")
    for cid, (path, val) in ABILITY.items():
        lines.append(f"cards.{cid}.ability.{path}={json.dumps(val, separators=(',', ':'))}")
    with open(os.path.join(HERE, "targets.overlay"), "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines) + "\n")
    print(len(CARDS) + len(ABILITY), "cards")


main()

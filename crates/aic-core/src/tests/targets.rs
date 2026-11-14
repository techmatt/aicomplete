//! 234 (Matt, 2026-11-14): one target per map-touching player card.
use super::engine::table_with;
use super::*;
use crate::content::Op;
use crate::modules::board;

/// Plays seat 0's Signal Hijack (the tests' card), leaving its target prompt open.
fn start(rules: &Rules, g: &mut GameState) {
    rules.apply_mut(g, &Action::Play { seat: 0, card: "signal_hijack".into() }, None).unwrap();
}

fn refused(set: &str) -> String {
    Rules::new(&baseline_with(&[set])).err().unwrap_or_else(|| panic!("accepted: {set}"))
}

/// Every map-touching player card prints a target box, and the content check refuses a map effect with no
/// target, two targets, a free country pick, a reach past the target, or a targeted op with no place.
#[test]
fn a_map_effect_needs_one_target() {
    for c in &content().cards {
        let map = c.all_ops().iter().any(|o| o.is_map());
        let face = crate::face::card_face(c);
        assert_eq!(map, face.target.is_some(), "{}: {}", c.id, c.text);
        if map {
            assert!(!c.text.contains("in reach,") && !c.text.contains("in one country"), "{}: {}", c.id, c.text);
        }
    }
    let none = refused(r#"cards.eviction_worm.effects=[{"op":"remove_ambassadors_country","max":3,"at":"on"}]"#);
    assert!(none.contains("no target before it"), "{none}");
    let two = refused(
        r#"cards.eviction_worm.effects=[{"op":"target","on":"agents"},{"op":"target","on":"troops"},{"op":"remove_troops","count":1,"at":"on"}]"#,
    );
    assert!(two.contains("two targets"), "{two}");
    let two_boxes = refused(
        r#"cards.interceptor_battery.ability.effects=[{"op":"target","on":"troops"},{"op":"push","max":2,"pieces":["troops"]}]"#,
    );
    assert!(two_boxes.contains("two target conditions"), "{two_boxes}");
    let country = refused(r#"cards.eviction_worm.effects=[{"op":"target","on":"agents"},{"op":"damage_country","ambassadors":1}]"#);
    assert!(country.contains("picks a country freely"), "{country}");
    let far = refused(r#"cards.eviction_worm.effects=[{"op":"target","on":"agents"},{"op":"move_one","range":2}]"#);
    assert!(far.contains("past the target"), "{far}");
    let place = refused(r#"cards.eviction_worm.effects=[{"op":"target","on":"agents"},{"op":"remove_ambassadors_country","max":3}]"#);
    assert!(place.contains("needs its `at`"), "{place}");
    let idle = refused(r#"cards.eviction_worm.effects=[{"op":"target","on":"agents"},{"op":"draw","count":1}]"#);
    assert!(idle.contains("no map effect after it"), "{idle}");
}

/// A removal acts on its target only: agents next to it stay unless the card says "on or next to".
#[test]
fn a_removal_acts_on_the_target_and_next_to_it_where_printed() {
    let on = r#"cards.signal_hijack.effects=[{"op":"target","on":"agents"},{"op":"remove_ambassadors_country","max":5,"at":"on"}]"#;
    let near = r#"cards.signal_hijack.effects=[{"op":"target","on":"agents"},{"op":"remove_ambassadors_country","max":5,"at":"near"}]"#;
    for (set, left) in [(on, 2), (near, 0)] {
        let (rules, mut g) = table_with(&[set], &["signal_hijack"], &[]);
        let m = &rules.map;
        let home = m.numbered(0, piece(0).node);
        let next = m.hexes[home].neighbors.iter().copied().find(|&h| kind(m, h) != HexKind::Ocean).unwrap();
        for h in 0..g.hexes.len() {
            put(&mut g, h, ADV_AMBASSADOR, 0);
        }
        put(&mut g, home, ADV_AMBASSADOR, 2);
        put(&mut g, next, ADV_AMBASSADOR, 2);
        start(&rules, &mut g);
        pick(&rules, &mut g, next);
        answer_all(&rules, &mut g);
        assert_eq!(g.tokens(next, ADV_AMBASSADOR), 0, "{set}");
        assert_eq!(g.tokens(home, ADV_AMBASSADOR), left, "{set}");
    }
}

/// Place puts new ambassadors on the target; Gain puts them in your supply.
#[test]
fn place_goes_on_the_target_and_gain_to_the_supply() {
    let set = r#"cards.signal_hijack.effects=[{"op":"target","on":"piece"},{"op":"place_ambassadors","count":2,"at":"on"},{"op":"gain_ambassadors","count":1}]"#;
    let (rules, mut g) = table_with(&[set], &["signal_hijack"], &[]);
    let home = rules.map.numbered(0, piece(0).node);
    let before = g.resource(0, Resource::Ambassadors);
    let on = g.tokens(home, ambassador(0));
    start(&rules, &mut g);
    pick(&rules, &mut g, home);
    answer_all(&rules, &mut g);
    assert_eq!(g.tokens(home, ambassador(0)), on + 2);
    assert_eq!(g.resource(0, Resource::Ambassadors), before + 1);
    assert_eq!(face_text("amnesty_decree"), "Target: a corrupted city. Remove 4 Agents on or next to it. Place 2 Ambassadors on it.");
}

/// 236 (Matt, 2026-11-14): every board power that touches the map names one target, its text opening with it,
/// but Disrupt's one country (206); every standard action that touches the map prints one, and the op-backed ones
/// pass the cards' check. The content check refuses a power with a map op and no target.
#[test]
fn powers_and_standard_actions_name_one_target() {
    let mut aimed = Vec::new();
    for b in &content().ais {
        for p in &b.powers {
            crate::content::check_power_targets(p, &b.id).unwrap();
            let map = p.effects.iter().any(|o| o.is_map());
            let target = p.effects.iter().any(|o| matches!(o, Op::Target { .. }));
            assert_eq!(map && p.name != "Disrupt", target, "{} {}: {}", b.id, p.name, p.text);
            assert!(!p.text.contains("in reach"), "{} {}: {}", b.id, p.name, p.text);
            if target {
                aimed.push(p.name.clone());
            }
        }
    }
    let names = ["Fortify", "Bulwark", "Debrief", "Patrol", "Interdict", "Reclaim", "Field Station"];
    assert_eq!(aimed, names, "the powers that print a target");
    let fortify = content().ais.iter().flat_map(|b| &b.powers).find(|p| p.name == "Fortify").unwrap();
    let mut bare = fortify.clone();
    bare.effects.retain(|o| !matches!(o, Op::Target { .. }));
    let err = crate::content::check_power_targets(&bare, "builder").unwrap_err();
    assert!(err.contains("no target before it"), "{err}");
    let mut loose = fortify.clone();
    loose.text = "remove 1 Troop in reach".into();
    let err = crate::content::check_power_targets(&loose, "builder").unwrap_err();
    assert!(err.contains("opens with its target"), "{err}");

    let rules = Rules::new(&baseline()).unwrap();
    let w = rules.world();
    let t = |k: &str| k.to_string();
    let mut printed = 0;
    for act in board::STANDARD {
        let what = &board::std_rule_text(act, &w, &t)[2];
        assert_eq!(board::std_target(act).is_some(), what.starts_with("Target: "), "{act:?}: {what}");
        assert!(!what.contains("in reach"), "{act:?}: {what}");
        crate::content::check_target_lists(&[board::std_ops(act)], &format!("{act:?}"), false).unwrap();
        if board::std_target(act).is_some() && board::std_offered(&w, act) {
            printed += 1;
        }
    }
    // Station, Remove agents, Lift, Dispatch, Claim, Liberate, Botnet, Buy energy, Remove a troop (Factory is off).
    assert_eq!(printed, 9);
}

/// 236 (Matt, 2026-11-14; message 10585): a move card's board ambassadors come from on or next to its target, never
/// from anywhere; the supply stays a source.
#[test]
fn a_move_takes_ambassadors_from_on_or_next_to_the_target() {
    let set = r#"cards.signal_hijack.effects=[{"op":"target","on":"piece"},{"op":"move_ambassadors","max":2,"at":"on"}]"#;
    let (rules, mut g) = table_with(&[set], &["signal_hijack"], &[]);
    let m = &rules.map;
    let home = m.numbered(0, piece(0).node);
    let land = |h: usize| kind(m, h) != HexKind::Ocean;
    let next = m.hexes[home].neighbors.iter().copied().find(|&h| land(h)).unwrap();
    let far = (0..m.hexes.len()).find(|&h| land(h) && m.distance(home, h) >= 3).unwrap();
    for h in 0..g.hexes.len() {
        put(&mut g, h, ambassador(0), 0);
    }
    put(&mut g, next, ambassador(0), 1);
    put(&mut g, far, ambassador(0), 1);
    set_res(&mut g, 0, Resource::Ambassadors, 0);
    start(&rules, &mut g);
    pick(&rules, &mut g, home);
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(home, ambassador(0)), g.tokens(next, ambassador(0)), g.tokens(far, ambassador(0))), (1, 0, 1));
    assert_eq!(
        face_text("transmission_corridor"),
        "Target: a piece. Place 2 Nodes. Move up to 2 Ambassadors from your supply or next to it onto it."
    );
}

fn face_text(id: &str) -> String {
    crate::face::card_face(content().cards.iter().find(|c| c.id == id).expect("a card")).full_text()
}

/// The six country cards: a country-sized effect is the target's country; Shelter takes no target.
#[test]
fn country_cards_use_the_target_s_country() {
    assert_eq!(
        face_text("great_liberation"),
        "Target: a corrupted site. Pay 1 antimatter. Liberate or free it and 2 more corrupted cities, stations or factories in its country."
    );
    let shelter = "Shelter your Nodes and facilities in reach this round.";
    assert!(face_text("hardpoint_lockdown").contains(&format!("{shelter} Cordon its country")), "{}", face_text("hardpoint_lockdown"));
    assert!(face_text("aegis_grid_lockdown").starts_with(shelter), "{}", face_text("aegis_grid_lockdown"));
    assert_eq!(
        face_text("back_channel_talks"),
        "Target: a corrupted factory. Free it and 1 corrupted factory next to it. Sway 2 toward Unity."
    );
}


use super::*;
use crate::content::Op;
use crate::modules::common::{FIREWALL, FALLOUT};
use crate::modules::{adversary, alignment, board, effects, plan};
use crate::op::{CorruptSite, CreateAt, CreateIf, Facility, RallyAt};
use crate::modules::common::STATION;

/// Country `c`'s hexes passing `f`, lowest-numbered first.
fn numbered_where(m: &Map, c: u8, f: impl Fn(usize) -> bool) -> Vec<usize> {
    let mut v = hexes_where(m, |h| m.hexes[h].country == c && f(h));
    v.sort_by_key(|&h| m.hexes[h].number);
    v
}

/// 175: the build's Rally lands each board's pool on the sites of the card's type (r80: round-robin, so each
/// gets the even share or one more); the agents' forecast reads the same plan, and the pool empties.
#[test]
fn rally_lands_each_boards_pool_evenly_on_its_eligible_sites() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let m = &rules.map;
    let w = rules.world();
    g.vars.insert(SUPPLY.into(), 40);
    let players: Vec<Key> = (0..g.seats.len()).map(|s| Key::new(ambassador(s))).collect();
    for h in Arc::make_mut(&mut g.hexes).iter_mut() {
        Arc::make_mut(h).tokens.retain(|k, _| !players.contains(k));
    }
    // War Drums rallies to cities; only board 0 has a pool.
    set_pile(&mut g, NEXT, &["mil_s1_war_drums"]);
    for c in 0..m.countries.len() {
        g.vars.insert(adversary::pool_key(c).into(), 0);
    }
    g.vars.insert(adversary::pool_key(0).into(), 5);
    let sites = plan::rally_sites(&w, &g, RallyAt::City, 0);
    let cities0 = numbered_where(m, 0, |h| kind(m, h) == HexKind::City);
    assert_eq!(sites, cities0, "every city, whoever owns it");
    let before: Vec<i64> = (0..g.hexes.len()).map(|h| g.tokens(h, ADV_AMBASSADOR)).collect();
    let fore = plan::next_rally(&w, &g);
    assert!((fore.iter().map(|&(_, n)| n).sum::<f64>() - 5.0).abs() < 1e-9);
    run_answered(&rules, &mut g);
    let gained: Vec<(usize, i64)> = (0..g.hexes.len())
        .map(|h| (h, g.tokens(h, ADV_AMBASSADOR) - before[h]))
        .filter(|&(_, n)| n != 0)
        .collect();
    assert_eq!(gained.iter().map(|&(_, n)| n).sum::<i64>(), 5);
    let each = 5 / sites.len() as i64;
    assert!(gained.iter().all(|&(h, n)| sites.contains(&h) && (n == each || n == each + 1)), "{gained:?}");
    for (h, n) in fore {
        assert_eq!(gained.iter().find(|&&(x, _)| x == h).map_or(0, |&(_, k)| k) as f64, n, "the forecast is the landing");
    }
    assert_eq!(g.var(&adversary::pool_key(0)), 0);
    assert_eq!(g.var(SUPPLY), 35);
}

/// r80 (ruling #43): Rally deals the pool round-robin, one agent onto each site of the type in hex order (the
/// adversary's included, no remainder rule), until the pool is empty. r82 (Matt, 2026-11-11, as at r79): a
/// firewalled site keeps its turn in the deal and its share stays in the supply.
#[test]
fn rally_deals_round_robin_in_hex_order_and_a_firewalled_share_stays_in_the_supply() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let cities0 = numbered_where(m, 0, |h| kind(m, h) == HexKind::City);
    assert!(cities0.len() >= 3);
    for &h in &cities0 {
        Arc::make_mut(&mut Arc::make_mut(&mut g.hexes)[h]).tokens.clear();
    }
    put(&mut g, cities0[0], ADV_CUBE, 1);
    let k = cities0.len() as i64;
    let p = plan::rally_plan(&w, &g, RallyAt::City, 0, k + 1).unwrap();
    let want: Vec<(usize, i64)> = cities0.iter().enumerate().map(|(i, &h)| (h, if i == 0 { 2 } else { 1 })).collect();
    assert_eq!(p.shares, want, "the adversary's city first in hex order takes the extra agent");
    put(&mut g, cities0[1], FIREWALL, 1);
    let p = plan::rally_plan(&w, &g, RallyAt::City, 0, k + 1).unwrap();
    assert_eq!(p.shares, want, "a firewalled site keeps its turn in the deal");
    g.vars.insert(SUPPLY.into(), 40);
    with_cx(&rules, |cx| adversary::rally(&mut g, cx, RallyAt::City, k + 1, Some(0), "test")).unwrap();
    assert_eq!(g.tokens(cities0[1], ADV_AMBASSADOR), 0, "nothing lands on it");
    assert_eq!(g.tokens(cities0[2], ADV_AMBASSADOR), 1, "and no one else takes its share");
    assert_eq!(g.var(SUPPLY), 40 - k, "its 1 stays in the supply");
    assert_eq!(g.stats.counters["adversary.rally_blocked"], 1);
}

/// 138: a creation line's condition picks among the sites, and `sites` how
/// many in each country.
#[test]
fn a_creation_line_lands_on_the_lowest_numbered_sites_meeting_its_condition() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let cities0 = numbered_where(m, 0, |h| kind(m, h) == HexKind::City);
    let cities1 = numbered_where(m, 1, |h| kind(m, h) == HexKind::City);
    assert!(cities0.len() >= 3);
    put(&mut g, cities0[1], ADV_AMBASSADOR, 1);
    put(&mut g, cities0[2], ADV_AMBASSADOR, 1);
    let line = |if_: CreateIf, sites: i64| plan::CreateLine { count: 2, at: CreateAt::City, if_, sites };
    let at = |l| plan::create_plan(&w, &g, l, None, &[], None);
    let mut firsts = vec![(cities0[0], 2), (cities1[0], 2)];
    firsts.sort_unstable();
    assert_eq!(at(line(CreateIf::Any, 1)), firsts);
    assert_eq!(at(line(CreateIf::Ambassadors, 1)), vec![(cities0[1], 2)], "country 1 has no city holding one");
    let mut two = vec![(cities0[1], 2), (cities0[2], 2)];
    two.sort_unstable();
    assert_eq!(at(line(CreateIf::Ambassadors, 2)), two);
    let first_empty = |l| plan::create_plan(&w, &g, l, Some(0), &[], None);
    assert_eq!(first_empty(line(CreateIf::NoAmbassadors, 1)), vec![(cities0[0], 2)]);
    put(&mut g, cities0[0], cube(0), 1);
    assert_eq!(plan::create_plan(&w, &g, line(CreateIf::Player, 3), Some(0), &[], None), vec![(cities0[0], 2)]);
    // Run as an op, it places what the plan says.
    let supply = g.var(SUPPLY);
    with_cx(&rules, |cx| {
        let op = Op::Create { count: 2, at: CreateAt::City, if_: CreateIf::Ambassadors, sites: 1 };
        effects::run(&mut g, cx, &[op], &mut effects::Ctx::team().named("test"))
    })
    .unwrap();
    assert_eq!(g.tokens(cities0[1], ADV_AMBASSADOR), 3);
    assert_eq!(g.var(SUPPLY), supply - 2);
}

/// 175: Corrupt X: every agent next to an X moves onto it (all of them, not just enough), then every X
/// holding its number corrupts and its agents go to the supply. Short of the number the agents still move.
#[test]
fn corrupt_x_moves_every_agent_next_door_onto_the_site() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let n = adversary::corrupt_n(&w, CorruptSite::City, 0);
    assert!(n >= 3);
    let t = numbered_where(m, 0, |h| kind(m, h) == HexKind::City)[0];
    // Neighbors of `t` next to no other city, so their agents have one place to go.
    let mut around: Vec<usize> = m.hexes[t]
        .neighbors
        .iter()
        .copied()
        .filter(|&x| {
            kind(m, x) != HexKind::Ocean
                && kind(m, x) != HexKind::City
                && m.hexes[x].neighbors.iter().all(|&y| y == t || kind(m, y) != HexKind::City)
        })
        .collect();
    around.sort_by_key(|&x| m.hexes[x].number);
    assert!(around.len() >= 2, "a city with two lone neighbors");
    let (a, b) = (around[0], around[1]);

    let mut g = g0.clone();
    put(&mut g, t, ADV_AMBASSADOR, 1);
    put(&mut g, a, ADV_AMBASSADOR, n - 2);
    put(&mut g, b, ADV_AMBASSADOR, 2);
    let supply = g.var(SUPPLY);
    assert_eq!(plan::corrupt_plan(&w, &g, CorruptSite::City, n, None), vec![(t, n + 1)]);
    with_cx(&rules, |cx| adversary::corrupt_sites(&mut g, cx, CorruptSite::City, 0, None)).unwrap();
    assert_eq!((g.tokens(t, ADV_CUBE), g.tokens(t, ADV_AMBASSADOR)), (1, 0));
    assert_eq!((g.tokens(a, ADV_AMBASSADOR), g.tokens(b, ADV_AMBASSADOR)), (0, 0), "every agent next door moved");
    assert_eq!(g.var(SUPPLY), supply + n + 1);
    assert_eq!(g.stats.counters["adversary.corruptions.city"], 1);

    // One short: the agents still move onto it, nothing corrupts.
    let mut g = g0.clone();
    put(&mut g, t, ADV_AMBASSADOR, 1);
    put(&mut g, a, ADV_AMBASSADOR, n - 2);
    assert!(plan::corrupt_plan(&w, &g, CorruptSite::City, n, None).is_empty());
    with_cx(&rules, |cx| adversary::corrupt_sites(&mut g, cx, CorruptSite::City, 0, None)).unwrap();
    assert_eq!((g.tokens(t, ADV_CUBE), g.tokens(t, ADV_AMBASSADOR), g.tokens(a, ADV_AMBASSADOR)), (0, n - 1, 0));

    // A line's cut lowers the number.
    with_cx(&rules, |cx| adversary::corrupt_sites(&mut g, cx, CorruptSite::City, 1, None)).unwrap();
    assert_eq!(g.tokens(t, ADV_CUBE), 1);
}

/// 175: a node hex holding a node's corrupt number (2) loses the node at natural corruption, last node
/// included; its agents go to the supply and the hex's site is untouched.
#[test]
fn a_node_with_two_agents_is_lost_last_node_included() {
    let (rules, mut g) = lab(&["adversary.natural"]);
    let m = &rules.map;
    let w = rules.world();
    assert_eq!(adversary::corrupt_n(&w, CorruptSite::Node, 0), 2);
    // A city below its own number: the node still goes.
    let h = numbered_where(m, 0, |h| kind(m, h) == HexKind::City)[0];
    put(&mut g, h, node(0), 1);
    put(&mut g, h, ADV_AMBASSADOR, 1);
    assert!(plan::natural_targets(&w, &g, CorruptSite::Node, 2).is_empty());
    put(&mut g, h, ADV_AMBASSADOR, 2);
    assert_eq!(plan::natural_targets(&w, &g, CorruptSite::Node, 2), vec![h]);
    run_answered(&rules, &mut g);
    assert_eq!(g.tokens(h, node(0)), 0, "the last node goes");
    assert_eq!(g.tokens(h, ADV_AMBASSADOR), 0);
    assert_eq!(g.tokens(h, ADV_CUBE), 0, "the city doesn't corrupt");
}

/// 138: the corrupt step runs only the round's card's Corrupt lines (Long
/// Patrols: Corrupt stations); a city already holding its number waits for
/// natural corruption.
#[test]
fn the_corrupt_step_runs_the_cards_corrupt_lines() {
    let (rules, mut g) = lab(&["adversary.corrupt"]);
    let m = &rules.map;
    let w = rules.world();
    set_pile(&mut g, CURRENT, &["mil_s1_long_patrols"]);
    let open0 = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open);
    let s = open0[0];
    let next = m.hexes[s].neighbors.iter().copied().find(|&x| m.hexes[x].country == 0 && kind(m, x) == HexKind::Open).unwrap();
    put(&mut g, s, STATION, 1);
    put(&mut g, next, ADV_AMBASSADOR, adversary::corrupt_n(&w, CorruptSite::Station, 0));
    let city = numbered_where(m, 1, |h| kind(m, h) == HexKind::City)[0];
    put(&mut g, city, ADV_AMBASSADOR, adversary::corrupt_n(&w, CorruptSite::City, 0));
    run(&rules, &mut g);
    assert_eq!((g.tokens(s, ADV_CUBE), g.tokens(next, ADV_AMBASSADOR)), (1, 0));
    assert_eq!(g.tokens(city, ADV_CUBE), 0);
}

/// D39: a station flip's face prints `station_flip_at`, so the two stay in step.
#[test]
fn a_station_flip_face_prints_station_flip_at() {
    let rules = Rules::new(&baseline()).unwrap();
    let n = rules.world().int("adversary.station_flip_at");
    let want = format!("unless {n} Agents");
    // A forced flip (135: "even a player's") reads no tunable.
    let flips = |effects: &[Op]| effects.iter().any(|e| matches!(e, Op::FlipStation { force: false, .. }));
    let c = crate::content();
    let texts: Vec<(&str, &str)> = c
        .threats
        .iter()
        .filter(|t| flips(&t.effects))
        .map(|t| (t.id.as_str(), t.text.as_str()))
        .chain(c.projects.iter().filter(|p| flips(&p.effects)).map(|p| (p.id.as_str(), p.text.as_str())))
        .collect();
    // 142: Station Coup is the one unforced flip left.
    assert!(!texts.is_empty(), "{texts:?}");
    for (id, text) in texts {
        assert!(text.contains(&want), "{id} should say \"{want}\": {text}");
    }
}

/// 138: natural corruption: a city, station or factory already holding its
/// number on its own hex corrupts with no move; ambassadors next to it don't
/// count.
#[test]
fn natural_corruption_takes_sites_holding_their_number_on_their_own_hex() {
    let (rules, mut g) = lab(&["adversary.natural"]);
    let m = &rules.map;
    let w = rules.world();
    let n = adversary::corrupt_n(&w, CorruptSite::City, 0);
    let [a, b, aligned] = [0, 1, 2].map(|i| m.numbered(0, piece(0).cities[i]));
    put(&mut g, a, ADV_AMBASSADOR, n);
    put(&mut g, b, ADV_AMBASSADOR, n - 1);
    let by_b = m.hexes[b].neighbors.iter().copied().find(|&x| kind(m, x) == HexKind::Open).unwrap();
    put(&mut g, by_b, ADV_AMBASSADOR, 1);
    put(&mut g, aligned, ADV_AMBASSADOR, n);
    put(&mut g, aligned, cube(0), 1);
    let open1 = numbered_where(m, 1, |h| kind(m, h) == HexKind::Open);
    let (station, factory) = (open1[0], open1[1]);
    put(&mut g, station, STATION, 1);
    put(&mut g, station, ADV_AMBASSADOR, adversary::corrupt_n(&w, CorruptSite::Station, 0));
    put(&mut g, factory, FACTORY, 1);
    put(&mut g, factory, ADV_AMBASSADOR, adversary::corrupt_n(&w, CorruptSite::Factory, 0) - 1);
    run(&rules, &mut g);
    // Their ambassadors go back to the supply; an aligned city loses its cube.
    assert_eq!((g.tokens(a, ADV_CUBE), g.tokens(a, ADV_AMBASSADOR)), (1, 0));
    assert_eq!((g.tokens(b, ADV_CUBE), g.tokens(b, ADV_AMBASSADOR), g.tokens(by_b, ADV_AMBASSADOR)), (0, n - 1, 1));
    assert_eq!((g.tokens(aligned, ADV_CUBE), g.tokens(aligned, cube(0))), (1, 0));
    assert_eq!((g.tokens(station, ADV_CUBE), g.tokens(station, ADV_AMBASSADOR)), (1, 0));
    assert_eq!(g.tokens(factory, ADV_CUBE), 0);
    assert_eq!(g.stats.counters["adversary.corruptions"], 3);
    assert_eq!(g.stats.counters["adversary.corruptions_natural"], 3);
}

/// 138: corruption destroys only a node on the corrupted hex (its owner's
/// ambassador there is armor); a node next to it stays.
#[test]
fn corruption_destroys_only_the_node_on_its_hex_and_an_ambassador_is_armor() {
    let (rules, mut g0) = lab(&[]);
    let m = &rules.map;
    g0.vars.insert(STAGE.into(), 2);
    let h = m.numbered(0, piece(0).cities[0]);
    let next = m.hexes[h].neighbors.iter().copied().find(|&x| kind(m, x) == HexKind::Open).unwrap();
    put(&mut g0, h, node(0), 1);
    put(&mut g0, next, node(1), 1);
    // Spares elsewhere, so neither is a last node.
    put(&mut g0, m.numbered(1, piece(1).node), node(0), 1);
    put(&mut g0, m.numbered(1, piece(1).cities[0]), node(1), 1);

    let mut g = g0.clone();
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h)).unwrap();
    assert_eq!(g.tokens(h, node(0)), 0);
    assert_eq!(g.tokens(next, node(1)), 1, "no blast next to it");
    assert_eq!(g.stats.counters["board.node_lost.corruption"], 1);

    // r75 (Matt's ruling 5): the owner is asked whether to spend its ambassador there; spent, it keeps the node.
    let mut g = g0.clone();
    put(&mut g, h, ambassador(0), 1);
    let asked = std::cell::RefCell::new(Vec::new());
    with_answers(&rules, &mut g, |a| {
        asked.borrow_mut().push((a.seat, a.options.len()));
        0
    }, |g, cx| adversary::corrupt_city(g, cx, h));
    assert_eq!(asked.into_inner(), [(0, 2)], "the owner's ambassador, or none");
    assert_eq!((g.tokens(h, node(0)), g.tokens(h, ambassador(0)), g.tokens(h, ADV_CUBE)), (1, 0, 1));
}

/// 138: the Protector's Sanctuary capstone (`CorruptionShield`): corruption
/// never destroys its nodes.
#[test]
fn the_corruption_shield_keeps_a_node_on_a_corrupted_hex() {
    let (rules, mut g) = lab(&[]);
    let w = rules.world();
    let m = &rules.map;
    g.vars.insert(STAGE.into(), 2);
    let shielded = |g: &GameState| crate::modules::cards::has_passive(&w, g, 1, |p| matches!(p, Op::CorruptionShield));
    assert!(!shielded(&g));
    g.vars.insert(board::track_key(1).into(), 7);
    assert!(shielded(&g), "seat 1 plays the Protector");
    let h = m.numbered(0, piece(0).cities[0]);
    put(&mut g, h, node(1), 1);
    put(&mut g, m.numbered(1, piece(1).node), node(1), 1);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h)).unwrap();
    assert_eq!((g.tokens(h, ADV_CUBE), g.tokens(h, node(1))), (1, 1));
}

/// 138: Corrupt land puts Fallout on an empty land hex that gathers
/// its number, and no node goes there after.
#[test]
fn fallout_blocks_a_node() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let x = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open)[0];
    let mine = m.hexes[x].neighbors.iter().copied().find(|&y| kind(m, y) == HexKind::Open).unwrap();
    put(&mut g, mine, node(0), 1);
    assert!(board::node_sites(&w, &g, 0).contains(&x));
    put(&mut g, x, ADV_AMBASSADOR, adversary::corrupt_n(&w, CorruptSite::Land, 0));
    with_cx(&rules, |cx| adversary::corrupt_sites(&mut g, cx, CorruptSite::Land, 0, Some(0))).unwrap();
    assert_eq!((g.tokens(x, FALLOUT), g.tokens(x, ADV_AMBASSADOR), g.tokens(x, ADV_CUBE)), (1, 0, 0));
    assert!(!board::node_sites(&w, &g, 0).contains(&x));
    assert!(!plan::corrupt_target(&w, &g, x, CorruptSite::Land), "it corrupts once");
}

/// 138: site creation puts a neutral station on each country's lowest-numbered
/// empty printed station site, a factory on its lowest-numbered free zone.
#[test]
fn site_creation_opens_the_lowest_numbered_empty_site_in_each_country() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let make = |g: &mut GameState, kind: Facility| {
        with_cx(&rules, |cx| {
            effects::run(g, cx, &[Op::CreateSite { kind }], &mut effects::Ctx::team().named("test"))
        })
        .unwrap()
    };
    let sites = |c: u8| numbered_where(m, c, |h| m.hexes[h].station_site);
    let zones = |c: u8| numbered_where(m, c, |h| m.hexes[h].industrial);
    // 190: country 1's one site became a setup station (Cintamani 11): it creates no station.
    assert!(!sites(0).is_empty() && sites(1).is_empty());
    for c in 0..2 {
        assert!(zones(c).len() >= 2, "country {c}");
    }
    // Country 0's station site and lowest zone are built on: its station has
    // nowhere to go, its factory goes on the next zone.
    put(&mut g, sites(0)[0], FACTORY, 1);
    put(&mut g, zones(0)[0], STATION, 1);
    make(&mut g, Facility::Station);
    make(&mut g, Facility::Factory);
    let built = |g: &GameState, c: u8, k: &str| numbered_where(m, c, |h| g.tokens(h, k) > 0);
    assert_eq!(built(&g, 0, STATION), vec![zones(0)[0]], "no new station in country 0");
    let factories0: Vec<usize> = zones(0).into_iter().skip(1).filter(|&h| h != sites(0)[0]).take(1).chain([sites(0)[0]]).collect();
    let mut want0 = factories0;
    want0.sort_by_key(|&h| m.hexes[h].number);
    assert_eq!(built(&g, 0, FACTORY), want0);
    assert_eq!(built(&g, 1, STATION), Vec::<usize>::new());
    let first_zone1 = zones(1)[0];
    assert_eq!(built(&g, 1, FACTORY), vec![first_zone1]);
    assert!(built(&g, 1, STATION).into_iter().chain(built(&g, 1, FACTORY)).all(|h| cube_free(&g, h)), "neutral");
}

/// No player or adversary cube: a created site is neutral.
fn cube_free(g: &GameState, h: usize) -> bool {
    g.tokens(h, ADV_CUBE) == 0 && (0..g.seats.len()).all(|s| g.tokens(h, cube(s)) == 0)
}

/// 138: with the spread gone an arriving ambassador never forms a botnet; it
/// still cancels a player ambassador.
#[test]
fn an_arriving_ambassador_cancels_a_player_ambassador_and_forms_no_botnet() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let s = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open)[0];
    put(&mut g, s, STATION, 1);
    put(&mut g, s, ADV_AMBASSADOR, 1);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, s, 2)).unwrap();
    assert_eq!((g.tokens(s, ADV_AMBASSADOR), g.tokens(s, BOTNET)), (3, 0));
    let c = m.numbered(1, piece(1).cities[0]);
    put(&mut g, c, ambassador(0), 1);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, c, 1)).unwrap();
    assert_eq!((g.tokens(c, ADV_AMBASSADOR), g.tokens(c, ambassador(0))), (0, 0));
    assert_eq!(total(&g, BOTNET), 0);
}

#[test]
fn botnets_start_on_the_stations_and_make_adversary_compute() {
    // 175: botnets make compute in the compute step (the reveal step's income), with the income and the
    // corrupted stations; here income 0, so botnets and the two corrupted stations.
    let mut c = lab_config(&["adversary.reveal", "adversary.compute", ACTIONS]);
    apply_sets(&mut c, &["adversary.compute_income=0,0,0,0,0"]);
    let rules = Rules::new(&c).unwrap();
    let mut g = rules.new_game(1);
    answer_all(&rules, &mut g);
    let m = &rules.map;
    for c in 0..2 {
        assert_eq!(g.tokens(m.numbered(c, piece(c).stations[0]), BOTNET), 1);
    }
    assert_eq!(total(&g, BOTNET), 2);
    // Round 1's compute step has run: 3 compute a botnet (lab: botnet_compute 3).
    assert_eq!(g.stats.counters["adversary.compute_botnets"], 6);
    // 185: a corrupted station makes compute only once its megaproject is complete (none is here).
    let stations = 0;
    let before = g.var(adversary::COMPUTE);
    run_answered(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE), before + 6 + stations);
    put(&mut g, m.numbered(0, piece(0).stations[0]), BOTNET, 0);
    run_answered(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE), before + 6 + stations + 3 + stations);
}

/// 151: setup sets the income from the difficulty's list entry plus the board's Garrisons (+1 ambassador) and
/// fills each board's pool once; each reveal step then adds the compute income onto the projects and the
/// ambassador income to every pool.
#[test]
fn the_adversary_gains_its_income_at_the_end_of_its_turn() {
    for (d, compute, ambassadors) in [(0, 5, 1), (3, 7, 2)] {
        let mut c = lab_config(&["adversary.reveal", "adversary.compute", ACTIONS]);
        let level = format!("adversary.difficulty={d}");
        apply_sets(&mut c, &[&level, "adversary.compute_income=5,5,5,7,7", "adversary.ambassador_income=1,1,1,2,2"]);
        let rules = Rules::new(&c).unwrap();
        let mut g = rules.new_game(1);
        answer_all(&rules, &mut g);
        let boards = rules.map.countries.len();
        assert_eq!(g.var(adversary::INCOME_COMPUTE), compute, "d{d}");
        assert_eq!(g.var(adversary::INCOME_AMBASSADORS), ambassadors + 1, "d{d}: Garrisons");
        // Setup's pools, then round 1's reveal step (no build here to land them).
        for b in 0..boards {
            assert_eq!(g.var(&adversary::pool_key(b)), 2 * (ambassadors + 1), "d{d} board {b}");
        }
        assert_eq!(g.stats.counters["adversary.compute_income"], compute, "d{d}");
        // A row too dear to fill: the next reveal step puts the income on it, resolving nothing.
        row(&mut g, &["accelerated_timetable"]);
        set_pile(&mut g, adversary::PROJECTS_DONE, &[]);
        g.vars.insert(adversary::project_key("accelerated_timetable").into(), 0);
        let before = g.var(adversary::COMPUTE);
        // 175: the compute step adds the botnets' and corrupted stations' compute with the income.
        let botnets = total(&g, BOTNET) * rules.world().int("adversary.botnet_compute");
        // 185: only those whose megaproject is complete.
        let done = |h: usize| g.var(&crate::modules::board::project_done(rules.map.hexes[h].country as usize)) > 0;
        let stations = (0..g.hexes.len())
            .filter(|&h| g.tokens(h, STATION) > 0 && g.tokens(h, ADV_CUBE) > 0 && done(h))
            .count()
            as i64
            * rules.world().int("board.station_cubes");
        run_answered(&rules, &mut g);
        let all = compute + botnets + stations;
        assert_eq!(g.var(adversary::COMPUTE), before + all, "d{d}");
        assert_eq!(g.var(&adversary::project_key("accelerated_timetable")), all, "d{d}");
        assert!(g.pile(adversary::PROJECTS_DONE).is_empty());
        for b in 0..boards {
            assert_eq!(g.var(&adversary::pool_key(b)), 3 * (ambassadors + 1), "d{d} board {b}");
        }
    }
}

#[test]
fn the_players_lose_when_the_next_round_has_no_adversary_card() {
    let rules = Rules::new(&lab_config(&[
        "adversary.build",
        "adversary.reveal",
        ACTIONS,
    ]))
    .unwrap();
    let mut g = rules.new_game(1);
    // Round 1's card may ask where its troops land; so does Occupation's.
    answer_all(&rules, &mut g);
    // The last card is face up: its round plays, the one after is lost.
    g.piles.remove(DECK);
    set_pile(&mut g, NEXT, &["mil_s3_occupation"]);
    run_answered(&rules, &mut g);
    assert!(!g.is_over());
    assert_eq!(g.pile(CURRENT), ["mil_s3_occupation"]);
    assert!(g.pile(NEXT).is_empty());
    let deck = rules
        .view(&g)
        .globals
        .into_iter()
        .find(|v| v.key == "adversary.deck")
        .unwrap();
    assert_eq!(deck.value, 0);
    run(&rules, &mut g);
    let o = g.outcome.as_ref().expect("a loss");
    assert_eq!((o.won, o.reason.as_str()), (false, "deck_empty"));
}

/// 022 (D13): before this round's build step (a Discord tip in the Sway
/// phase resolving Buildup or Offensive) the round's twists are the face-up
/// card's, the one about to become this round's; after it, the current card's.
#[test]
fn twists_before_the_build_step_are_the_face_up_cards() {
    let (rules, mut g) = lab(&["alignment.sway", "adversary.build"]);
    let w = rules.world();
    set_pile(&mut g, CURRENT, &["mil_s1_probing_strikes"]);
    set_pile(&mut g, NEXT, &["mil_s1_long_patrols"]);
    let stations = |g: &GameState| {
        adversary::twists(&w, g)
            .iter()
            .any(|t| matches!(t, Op::Corrupt { site: CorruptSite::Station, .. }))
    };
    g.phase = 0;
    assert!(stations(&g), "the Sway phase reads the face-up card");
    g.phase = 2;
    assert!(!stations(&g), "after the build step, this round's card");
}

#[test]
fn a_stage_iii_card_pulls_sway_out_of_the_unity_bucket() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let w = rules.world();
    let card = w.content.adversary.iter().find(|c| c.stage == 3).unwrap();
    let pull = card.twists.iter().find_map(|t| match t {
        Op::PullUnity { amount } => Some(*amount),
        _ => None,
    });
    let pull = pull.expect("a stage-III card prints the pull");
    g.vars.insert(alignment::track(Side::Unity).into(), pull + 1);
    g.piles.insert(NEXT.into(), vec![card.id.as_str().into()].into());
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.stats.counters.get("alignment.unity_pulled").copied(), Some(pull));
    assert!(g.var(alignment::track(Side::Unity)) <= 1, "the bucket loses the pull");
}

/// 151: every event card opens on one Rally X (its pool lands), keeps to the event standard (no per-event
/// compute or creation line), and prints a Corrupt line; each stage corrupts cities, stations and factories.
#[test]
fn every_event_card_rallies_and_corrupts() {
    for stage in 1..=3u8 {
        let mut seen: Vec<CorruptSite> = Vec::new();
        for a in content().adversary.iter().filter(|a| a.stage == stage) {
            assert!(matches!(a.standing.first(), Some(Op::Rally { count: 0, .. })), "{} rallies first", a.id);
            assert!(adversary::rally_site(a).is_some(), "{} rallies", a.id);
            assert_eq!(crate::content::event_rule_break(a), None, "{} keeps the event standard", a.id);
            assert!(
                !a.standing.iter().any(|e| matches!(e, Op::Compute { .. } | Op::Create { .. })),
                "{} prints no per-event compute or creation line",
                a.id
            );
            let sites: Vec<CorruptSite> = a
                .twists
                .iter()
                .filter_map(|t| match t {
                    Op::Corrupt { site, .. } => Some(*site),
                    _ => None,
                })
                .collect();
            assert!(!sites.is_empty(), "{} corrupts", a.id);
            seen.extend(sites);
        }
        for site in [CorruptSite::City, CorruptSite::Station, CorruptSite::Factory] {
            assert!(seen.contains(&site), "stage {stage} corrupts {}", site.name());
        }
    }
}

/// 151: the load check refuses an event off the event standard (one Rally X, the fixed step order, at most two
/// other lines, "each board") and a project with more than two consequences.
#[test]
fn the_load_check_refuses_an_event_or_a_project_off_the_standard() {
    use crate::content::{event_rule_break, project_rule_break};
    let c = content();
    // War Drums: Rally, Muster | Sway, Corrupt.
    let base = c.adversary.iter().find(|a| a.id == "mil_s1_war_drums").unwrap();
    assert_eq!(event_rule_break(base), None);
    let rally = base.standing[0].clone();
    let sway = base.twists[0].clone();
    let corrupt = base.twists.last().unwrap().clone();
    assert!(matches!(rally, Op::Rally { .. }) && matches!(corrupt, Op::Corrupt { .. }));
    let refused = |edit: &dyn Fn(&mut crate::content::AdversaryCard), why: &str| {
        let mut a = base.clone();
        edit(&mut a);
        let got = event_rule_break(&a);
        assert!(got.as_deref().is_some_and(|r| r.contains(why)), "{why}: {got:?}");
    };
    refused(&|a| a.standing.insert(0, rally.clone()), "one Rally X");
    refused(&|a| a.standing.insert(0, corrupt.clone()), "out of the fixed order");
    refused(
        &|a| {
            a.twists.insert(0, sway.clone());
            a.twists.insert(0, sway.clone());
        },
        "more than two other lines",
    );
    refused(&|a| a.text = "Muster 1 on each board.".into(), "each board");
    // Hostile Takeover (cost 8) prints two consequences; a third is refused below cost 14 (227, ruling 4) and allowed
    // from it.
    let p = c.projects.iter().find(|p| p.id == "hostile_takeover").unwrap();
    assert_eq!(p.effects.len() + p.resolve.len(), 2);
    assert_eq!(project_rule_break(p), None);
    let mut three = p.clone();
    three.effects.push(p.effects[0].clone());
    assert!(project_rule_break(&three).is_some_and(|r| r.contains("more than 2")));
    three.cost = crate::content::THIRD_EFFECT_COST;
    assert_eq!(project_rule_break(&three), None);
    let mut country = p.clone();
    country.text = "Each board: a threat.".into();
    assert!(project_rule_break(&country).is_some_and(|r| r.contains("each board")));
}

/// r98 (Matt, 2026-11-13, 224 #2): 2 agents on a node hex destroy the node and leave no mark; the agents go to the
/// supply.
#[test]
fn two_agents_destroy_a_node_and_leave_no_mark() {
    let (rules, mut g) = lab(&["adversary.natural"]);
    let m = &rules.map;
    let h = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open)[0];
    put(&mut g, h, node(0), 1);
    put(&mut g, h, ADV_AMBASSADOR, 2);
    run_answered(&rules, &mut g);
    assert_eq!((g.tokens(h, node(0)), g.tokens(h, ADV_AMBASSADOR)), (0, 0));
    assert_eq!(g.tokens(h, FALLOUT), 0);
}

/// r98: a node a protection saves leaves no mark either (r83-r97 laid Fallout saved or not).
#[test]
fn a_saved_node_hex_takes_no_mark() {
    let (rules, mut g) = lab(&["adversary.natural"]);
    let m = &rules.map;
    let h = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open)[0];
    put(&mut g, h, node(0), 1);
    put(&mut g, h, HARDEN, 1);
    put(&mut g, h, ADV_AMBASSADOR, 2);
    run(&rules, &mut g);
    answer_with(&rules, &mut g, "spend the hardening token");
    answer_all(&rules, &mut g);
    assert_eq!((g.tokens(h, node(0)), g.tokens(h, HARDEN)), (1, 0), "saved");
    assert_eq!(g.tokens(h, FALLOUT), 0);
}

/// r83 (Matt, 2026-11-11, #57): project slot 1's tag is gone: every slot of the Militarist's board is open.
#[test]
fn every_project_slot_is_open() {
    assert!(crate::content().adversary_board.project_slots.iter().all(|s| s.tag.is_none()));
}

/// r80 (ruling #48): the Protector's landing is a normal Muster 1 in each country where the Protector has a node:
/// the lowest-numbered factory, a player's included (178-r79 skipped players' factories), else a city.
#[test]
fn the_protectors_landing_is_a_normal_muster() {
    let (rules, mut g) = lab_with(&[], &["adversary.wall_troops=1"]);
    let m = &rules.map;
    let w = rules.world();
    let s = (0..g.seats.len()).find(|&s| board::has_wall(&w, s)).expect("a seat with Wall");
    let c = 0u8;
    let open = numbered_where(m, c, |h| kind(m, h) == HexKind::Open);
    put(&mut g, open[0], node(s), 1);
    let zones = numbered_where(m, c, |h| m.hexes[h].industrial && !m.hexes[open[0]].neighbors.contains(&h) && h != open[0]);
    assert!(zones.len() >= 2);
    put(&mut g, zones[0], FACTORY, 1);
    put(&mut g, zones[0], cube(0), 1);
    put(&mut g, zones[1], FACTORY, 1);
    with_cx(&rules, |cx| adversary::wall_pressure(&mut g, cx)).unwrap();
    assert_eq!(g.tokens(zones[0], TROOP), 1, "the lowest-numbered factory, a player's");
    assert_eq!(total(&g, TROOP), 1, "Muster 1, one country");
    // No factory in the country: the lowest-numbered city.
    let (_, mut g) = lab_with(&[], &["adversary.wall_troops=1"]);
    put(&mut g, open[0], node(s), 1);
    with_cx(&rules, |cx| adversary::wall_pressure(&mut g, cx)).unwrap();
    let city = numbered_where(m, c, |h| kind(m, h) == HexKind::City)[0];
    assert_eq!(g.tokens(city, TROOP), 1);
}

/// r80 (184's D79 a): an adversary-factory line with no free zone lands its fallback agents on a factory, never a
/// firewalled one (as a creation line skips it).
#[test]
fn the_adversary_factory_fallback_skips_a_firewalled_factory() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let zones = hexes_where(m, |h| m.hexes[h].industrial);
    for &z in &zones {
        put(&mut g, z, STATION, 1);
    }
    let (a, b) = (zones[0], zones[1]);
    for z in [a, b] {
        put(&mut g, z, STATION, 0);
        put(&mut g, z, FACTORY, 1);
    }
    put(&mut g, a, FIREWALL, 1);
    g.vars.insert(SUPPLY.into(), 40);
    let op = Op::AdversaryFactory { count: 1, where_: None, fallback_ambassadors: 2 };
    with_cx(&rules, |cx| effects::run(&mut g, cx, &[op], &mut effects::Ctx::team().named("test"))).unwrap();
    assert_eq!((g.tokens(a, ADV_AMBASSADOR), g.tokens(b, ADV_AMBASSADOR)), (0, 2));
}

/// r80 (ruling #31): every event card's lines come from the fixed list of line kinds, each with an icon; a line
/// of another kind is off the event standard.
#[test]
fn event_lines_come_from_the_fixed_list() {
    use crate::content::{event_line_kind, event_rule_break, EVENT_LINE_KINDS};
    for a in &crate::content().adversary {
        for e in a.standing.iter().chain(&a.twists) {
            let k = event_line_kind(e).unwrap_or_else(|| panic!("{}: {e:?}", a.id));
            assert!(EVENT_LINE_KINDS.iter().any(|(n, icon)| *n == k && !icon.is_empty()));
        }
    }
    let mut a = crate::content().adversary[0].clone();
    a.standing.push(Op::Gain { resource: crate::content::Resource::Credits, amount: 1 });
    assert!(event_rule_break(&a).is_some_and(|why| why.contains("fixed list")));
}

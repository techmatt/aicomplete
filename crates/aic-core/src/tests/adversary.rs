use std::collections::BTreeMap;

use super::*;
use crate::content::Op;
use crate::modules::common::{LAND_CORRUPTION, MARKER};
use crate::modules::{adversary, alignment, board, effects, plan};
use crate::op::{CorruptSite, CreateAt, CreateIf, Facility};

/// Country `c`'s hexes passing `f`, lowest-numbered first.
fn numbered_where(m: &Map, c: u8, f: impl Fn(usize) -> bool) -> Vec<usize> {
    let mut v = hexes_where(m, |h| m.hexes[h].country == c && f(h));
    v.sort_by_key(|&h| m.hexes[h].number);
    v
}

/// 138: the build runs the event card's creation lines, then the board's
/// (Militarist: 2 on each country's lowest-numbered adversary city), each on
/// the lowest-numbered matching sites; the forecast reads the same plan.
#[test]
fn the_build_runs_the_cards_and_the_boards_creation_lines_on_the_lowest_numbered_sites() {
    let (rules, mut g) = lab(&["adversary.build"]);
    let m = &rules.map;
    let w = rules.world();
    g.vars.insert(SUPPLY.into(), 40);
    // Two adversary cities in country 0: only the lower-numbered one gains.
    let cities0 = numbered_where(m, 0, |h| kind(m, h) == HexKind::City);
    let (low, high) = (cities0[0], cities0[1]);
    put(&mut g, high, ADV_CUBE, 1);
    put(&mut g, low, ADV_CUBE, 1);
    // Three botnets in country 0: Signal Jamming creates on the 2 lowest.
    let open0 = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open);
    let bots = [open0[0], open0[1], open0[2]];
    for &h in &bots {
        put(&mut g, h, BOTNET, 1);
    }
    set_pile(&mut g, NEXT, &["mil_s1_signal_jamming"]);

    let mut want: BTreeMap<usize, i64> = BTreeMap::new();
    for c in 0..m.countries.len() as u8 {
        for &h in numbered_where(m, c, |h| kind(m, h) == HexKind::City && m.hexes[h].coastal).iter().take(2) {
            *want.entry(h).or_default() += 1;
        }
    }
    *want.entry(bots[0]).or_default() += 1;
    *want.entry(bots[1]).or_default() += 1;
    *want.entry(low).or_default() += 2;
    let want: Vec<(usize, i64)> = want.into_iter().collect();
    assert_eq!(plan::next_build(&w, &g), want, "the forecast is the build");

    run_answered(&rules, &mut g);
    assert_eq!(g.pile(CURRENT), ["mil_s1_signal_jamming"]);
    let got: Vec<(usize, i64)> =
        (0..g.hexes.len()).map(|h| (h, g.tokens(h, ADV_AMBASSADOR))).filter(|&(_, n)| n > 0).collect();
    assert_eq!(got, want);
    assert!(g.tokens(high, ADV_AMBASSADOR) < 2, "the board's 2 land on the lowest-numbered only");
    assert_eq!(g.tokens(bots[2], ADV_AMBASSADOR), 0);
    assert_eq!(g.var(SUPPLY), 40 - want.iter().map(|&(_, n)| n).sum::<i64>());
    assert_eq!(g.var(STAGE), 1);
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

/// 138: Corrupt X: a target gathers its number from its own hex, then from
/// its same-country neighbors lowest-numbered first, pulling just enough; the
/// rest stay. Short of the number nothing moves.
#[test]
fn corrupt_x_pulls_just_enough_from_the_lowest_numbered_neighbors() {
    let (rules, g0) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let n = adversary::corrupt_n(&w, CorruptSite::City, 0);
    assert!(n >= 3);
    let t = numbered_where(m, 0, |h| kind(m, h) == HexKind::City)[0];
    let mut around: Vec<usize> = m.hexes[t]
        .neighbors
        .iter()
        .copied()
        .filter(|&x| m.hexes[x].country == 0 && kind(m, x) != HexKind::Ocean && kind(m, x) != HexKind::City)
        .collect();
    around.sort_by_key(|&x| m.hexes[x].number);
    let (a, b) = (around[0], around[1]);

    let mut g = g0.clone();
    put(&mut g, t, ADV_AMBASSADOR, 1);
    put(&mut g, a, ADV_AMBASSADOR, n - 2);
    put(&mut g, b, ADV_AMBASSADOR, 2);
    let supply = g.var(SUPPLY);
    assert_eq!(
        plan::corrupt_plan(&w, &g, CorruptSite::City, n, None),
        vec![(t, vec![(t, 1), (a, n - 2), (b, 1)])]
    );
    with_cx(&rules, |cx| adversary::corrupt_sites(&mut g, cx, CorruptSite::City, 0, None));
    assert_eq!((g.tokens(t, ADV_CUBE), g.tokens(t, ADV_AMBASSADOR)), (1, 0));
    assert_eq!((g.tokens(a, ADV_AMBASSADOR), g.tokens(b, ADV_AMBASSADOR)), (0, 1));
    assert_eq!(g.var(SUPPLY), supply + n);
    assert_eq!(g.stats.counters["adversary.corruptions.city"], 1);

    // One short: nothing moves, nothing corrupts.
    let mut g = g0.clone();
    put(&mut g, t, ADV_AMBASSADOR, 1);
    put(&mut g, a, ADV_AMBASSADOR, n - 2);
    assert!(plan::corrupt_plan(&w, &g, CorruptSite::City, n, None).is_empty());
    with_cx(&rules, |cx| adversary::corrupt_sites(&mut g, cx, CorruptSite::City, 0, None));
    assert_eq!((g.tokens(t, ADV_CUBE), g.tokens(t, ADV_AMBASSADOR), g.tokens(a, ADV_AMBASSADOR)), (0, 1, n - 2));

    // A line's cut lowers the number.
    with_cx(&rules, |cx| adversary::corrupt_sites(&mut g, cx, CorruptSite::City, 1, None));
    assert_eq!(g.tokens(t, ADV_CUBE), 1);
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
    let want = format!("unless {n} adversary");
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
    assert!(texts.len() >= 2, "{texts:?}");
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
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h));
    assert_eq!(g.tokens(h, node(0)), 0);
    assert_eq!((g.tokens(next, node(1)), g.tokens(next, MARKER)), (1, 0), "no blast next to it");
    assert_eq!(g.stats.counters["board.node_lost.corruption"], 1);

    let mut g = g0.clone();
    put(&mut g, h, ambassador(0), 1);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h));
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
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h));
    assert_eq!((g.tokens(h, ADV_CUBE), g.tokens(h, node(1))), (1, 1));
}

/// 138: Corrupt land puts land corruption on an empty land hex that gathers
/// its number, and no node goes there after.
#[test]
fn land_corruption_blocks_a_node() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let x = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open)[0];
    let mine = m.hexes[x].neighbors.iter().copied().find(|&y| kind(m, y) == HexKind::Open).unwrap();
    put(&mut g, mine, node(0), 1);
    assert!(board::node_hexes(&w, &g, 0).contains(&x));
    put(&mut g, x, ADV_AMBASSADOR, adversary::corrupt_n(&w, CorruptSite::Land, 0));
    with_cx(&rules, |cx| adversary::corrupt_sites(&mut g, cx, CorruptSite::Land, 0, Some(0)));
    assert_eq!((g.tokens(x, LAND_CORRUPTION), g.tokens(x, ADV_AMBASSADOR), g.tokens(x, ADV_CUBE)), (1, 0, 0));
    assert!(!board::node_hexes(&w, &g, 0).contains(&x));
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
    for c in 0..2 {
        assert!(!sites(c).is_empty() && zones(c).len() >= 2, "country {c}");
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
    assert_eq!(built(&g, 1, STATION), vec![sites(1)[0]]);
    let first_zone1 = zones(1).into_iter().find(|&h| h != sites(1)[0]).unwrap();
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
    with_cx(&rules, |cx| adversary::place(&mut g, cx, s, 2));
    assert_eq!((g.tokens(s, ADV_AMBASSADOR), g.tokens(s, BOTNET)), (3, 0));
    let c = m.numbered(1, piece(1).cities[0]);
    put(&mut g, c, ambassador(0), 1);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, c, 1));
    assert_eq!((g.tokens(c, ADV_AMBASSADOR), g.tokens(c, ambassador(0))), (0, 0));
    assert_eq!(total(&g, BOTNET), 0);
}

#[test]
fn botnets_start_on_the_stations_and_make_adversary_compute() {
    let rules = Rules::new(&lab_config(&["adversary.build", ACTIONS])).unwrap();
    let mut g = rules.new_game(1);
    answer_all(&rules, &mut g);
    let m = &rules.map;
    for c in 0..2 {
        assert_eq!(g.tokens(m.numbered(c, piece(c).stations[0]), BOTNET), 1);
    }
    assert_eq!(total(&g, BOTNET), 2);
    // Round 1's build has run: 3 compute a botnet (lab: botnet_compute 3), and
    // (135) the event card's standing compute.
    assert_eq!(g.stats.counters["adversary.compute_botnets"], 6);
    // Probing Strikes and War Drums each print 2 standing compute.
    let before = g.var(adversary::COMPUTE);
    set_pile(&mut g, NEXT, &["mil_s1_probing_strikes"]);
    run_answered(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE), before + 6 + 2);
    put(&mut g, m.numbered(0, piece(0).stations[0]), BOTNET, 0);
    set_pile(&mut g, NEXT, &["mil_s1_war_drums"]);
    run_answered(&rules, &mut g);
    assert_eq!(g.var(adversary::COMPUTE), before + 8 + 3 + 2);
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

/// 138: every event card prints a creation line and a Corrupt line, and each
/// stage corrupts cities, stations and factories.
#[test]
fn every_event_card_creates_and_corrupts() {
    for stage in 1..=3u8 {
        let mut seen: Vec<CorruptSite> = Vec::new();
        for a in content().adversary.iter().filter(|a| a.stage == stage) {
            assert!(a.standing.iter().any(|e| matches!(e, Op::Create { .. })), "{} creates", a.id);
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

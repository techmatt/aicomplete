
use super::*;
use crate::content::Op;
use crate::modules::common::{FIREWALL, FALLOUT};
use crate::modules::{adversary, alignment, board, effects, plan};
use crate::op::{CorruptSite, CreateAt, CreateIf, Facility, SiteType};
use crate::modules::common::STATION;

/// Country `c`'s hexes passing `f`, lowest-numbered first.
fn numbered_where(m: &Map, c: u8, f: impl Fn(usize) -> bool) -> Vec<usize> {
    let mut v = hexes_where(m, |h| m.hexes[h].country == c && f(h));
    v.sort_by_key(|&h| m.hexes[h].number);
    v
}

/// Every agent and player ambassador off the board.
fn clear_agents(g: &mut GameState) {
    let gone: Vec<Key> = (0..g.seats.len()).map(|s| Key::new(ambassador(s))).chain([Key::new(ADV_AMBASSADOR)]).collect();
    for h in Arc::make_mut(&mut g.hexes).iter_mut() {
        Arc::make_mut(h).tokens.retain(|k, _| !gone.contains(k));
    }
}

/// 275 (Matt, 2026-11-18): Infiltrate X lands `infiltrate_agents` (1 at d0) on each site of its type not yet the
/// adversary's, whoever owns it; a corrupted one takes none, and nothing else on the board gains an agent.
#[test]
fn an_infiltrate_lands_one_agent_on_each_site_of_its_type_and_skips_a_corrupted_one() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    assert_eq!(adversary::infiltrate_agents(&w), 1);
    set_var(&mut g, SUPPLY, 40);
    clear_agents(&mut g);
    let cities0 = numbered_where(m, 0, |h| kind(m, h) == HexKind::City);
    assert!(cities0.len() >= 3);
    put(&mut g, cities0[0], ADV_CUBE, 1);
    put(&mut g, cities0[1], cube(0), 1);
    let sites = plan::infiltrate_sites(&w, &g, SiteType::City, 0);
    assert_eq!(sites, cities0[1..].to_vec(), "every city not yet the adversary's, a player's included");
    with_cx(&rules, |cx| adversary::infiltrate(&mut g, cx, SiteType::City, Some(0), "test")).unwrap();
    let gained: Vec<(usize, i64)> = (0..g.hexes.len()).map(|h| (h, g.tokens(h, ADV_AMBASSADOR))).filter(|&(_, n)| n != 0).collect();
    assert_eq!(gained, sites.iter().map(|&h| (h, 1)).collect::<Vec<_>>());
    assert_eq!(g.var(SUPPLY), 40 - sites.len() as i64);
    assert_eq!(g.stats.counters["adversary.infiltrated"], sites.len() as i64);
}

/// 275: a firewalled site is still named by an Infiltrate, but nothing lands on it: its agent stays in the supply
/// (and out of the forecast), and no other site takes it.
#[test]
fn an_infiltrate_leaves_a_firewalled_sites_agent_in_the_supply() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    set_var(&mut g, SUPPLY, 40);
    clear_agents(&mut g);
    let cities0 = numbered_where(m, 0, |h| kind(m, h) == HexKind::City);
    put(&mut g, cities0[1], FIREWALL, 1);
    let k = cities0.len() as i64;
    assert!(plan::infiltrate_sites(&w, &g, SiteType::City, 0).contains(&cities0[1]));
    with_cx(&rules, |cx| adversary::infiltrate(&mut g, cx, SiteType::City, Some(0), "test")).unwrap();
    assert_eq!(g.tokens(cities0[1], ADV_AMBASSADOR), 0, "nothing lands on it");
    assert_eq!(g.tokens(cities0[0], ADV_AMBASSADOR), 1, "and no one else takes its agent");
    assert_eq!(g.var(SUPPLY), 40 - (k - 1), "its 1 stays in the supply");
    assert_eq!(g.stats.counters["adversary.firewalled"], 1);
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

/// 275 (Matt, 2026-11-18): arrival corruption: an arrival that brings a research station to its number (2)
/// corrupts it and its agents go to the supply; one that brings a city to 4 doesn't (d0: a city's 5), and the next
/// one does.
#[test]
fn an_arrival_at_a_sites_number_corrupts_it_and_one_short_does_not() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    assert_eq!(adversary::corrupt_n(&w, CorruptSite::Station, 0), 2);
    assert_eq!(adversary::corrupt_n(&w, CorruptSite::City, 0), 5, "d0");
    set_var(&mut g, SUPPLY, 40);
    let s = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open)[0];
    put(&mut g, s, STATION, 1);
    put(&mut g, s, ADV_AMBASSADOR, 1);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, s, 1)).unwrap();
    assert_eq!((g.tokens(s, ADV_CUBE), g.tokens(s, ADV_AMBASSADOR)), (1, 0), "the station corrupts");
    assert_eq!(g.var(SUPPLY), 40 - 1 + 2, "its agents go to the supply");
    assert_eq!(g.stats.counters["adversary.corruptions_arrival"], 1);
    let c = numbered_where(m, 0, |h| kind(m, h) == HexKind::City)[0];
    put(&mut g, c, ADV_AMBASSADOR, 3);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, c, 1)).unwrap();
    assert_eq!((g.tokens(c, ADV_CUBE), g.tokens(c, ADV_AMBASSADOR)), (0, 4), "a city at 4 holds at d0");
    with_cx(&rules, |cx| adversary::place(&mut g, cx, c, 1)).unwrap();
    assert_eq!((g.tokens(c, ADV_CUBE), g.tokens(c, ADV_AMBASSADOR)), (1, 0), "at 5 it corrupts");
}

/// 275: nothing is checked where nothing arrived: a city or station already holding its number stays neutral
/// while agents land next to it, and corrupts when one lands on it.
#[test]
fn a_site_holding_its_number_waits_for_an_arrival_on_it() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    set_var(&mut g, SUPPLY, 40);
    let n = adversary::corrupt_n(&w, CorruptSite::City, 0);
    let c = numbered_where(m, 0, |h| kind(m, h) == HexKind::City)[0];
    let next = m.hexes[c].neighbors.iter().copied().find(|&x| kind(m, x) == HexKind::Open).unwrap();
    put(&mut g, c, ADV_AMBASSADOR, n);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, next, 1)).unwrap();
    assert_eq!((g.tokens(c, ADV_CUBE), g.tokens(c, ADV_AMBASSADOR)), (0, n), "an arrival next door checks nothing here");
    assert!(!g.stats.counters.contains_key("adversary.corruptions"));
    with_cx(&rules, |cx| adversary::place(&mut g, cx, c, 1)).unwrap();
    assert_eq!((g.tokens(c, ADV_CUBE), g.tokens(c, ADV_AMBASSADOR)), (1, 0));
}

/// 275: a firewall stops the arrival itself (the site holds, its agent stays in the supply), and a Corrupt
/// prohibition's token lets the agent land but holds the corruption.
#[test]
fn a_firewall_or_a_corrupt_prohibition_holds_an_arrival_corruption() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    set_var(&mut g, SUPPLY, 40);
    let open0 = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open);
    let (a, b) = (open0[0], open0[1]);
    for s in [a, b] {
        put(&mut g, s, STATION, 1);
        put(&mut g, s, ADV_AMBASSADOR, 1);
    }
    put(&mut g, a, FIREWALL, 1);
    put(&mut g, b, crate::modules::common::prohibit_token(crate::op::Step::Corrupt), 1);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, a, 1)).unwrap();
    assert_eq!((g.tokens(a, ADV_CUBE), g.tokens(a, ADV_AMBASSADOR), g.var(SUPPLY)), (0, 1, 40), "nothing arrives");
    with_cx(&rules, |cx| adversary::place(&mut g, cx, b, 1)).unwrap();
    assert_eq!((g.tokens(b, ADV_CUBE), g.tokens(b, ADV_AMBASSADOR)), (0, 2), "it lands; the token holds the site");
    assert_eq!(g.stats.counters["adversary.guard_held.arrival"], 1);
}

/// 175, 275: an arrival that brings a player's node hex to a node's number (2) loses the node, last node included;
/// its agents go to the supply and the hex's site (a city below its own number) is untouched.
#[test]
fn a_node_with_two_agents_is_lost_last_node_included() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    assert_eq!(adversary::corrupt_n(&w, CorruptSite::Node, 0), 2);
    let h = numbered_where(m, 0, |h| kind(m, h) == HexKind::City)[0];
    put(&mut g, h, node(0), 1);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, h, 1)).unwrap();
    assert_eq!((g.tokens(h, node(0)), g.tokens(h, ADV_AMBASSADOR)), (1, 1), "one is short");
    with_answers(&rules, &mut g, |_| 0, |g, cx| adversary::place(g, cx, h, 1));
    assert_eq!(g.tokens(h, node(0)), 0, "the last node goes");
    assert_eq!(g.tokens(h, ADV_AMBASSADOR), 0);
    assert_eq!(g.tokens(h, ADV_CUBE), 0, "the city doesn't corrupt");
}

/// 275: Surge N: X puts its N on the X not yet the adversary's holding the most agents (ties: the
/// lowest-numbered), and on no other.
#[test]
fn a_surge_lands_on_the_most_infested_site() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    set_var(&mut g, SUPPLY, 40);
    clear_agents(&mut g);
    let cities0 = numbered_where(m, 0, |h| kind(m, h) == HexKind::City);
    put(&mut g, cities0[0], ADV_AMBASSADOR, 1);
    put(&mut g, cities0[1], ADV_AMBASSADOR, 2);
    // The adversary's city holds more, but it is no longer a site.
    put(&mut g, cities0[2], ADV_CUBE, 1);
    put(&mut g, cities0[2], ADV_AMBASSADOR, 3);
    with_cx(&rules, |cx| adversary::surge(&mut g, cx, SiteType::City, 2, Some(0), "test")).unwrap();
    let held: Vec<i64> = cities0[..3].iter().map(|&h| g.tokens(h, ADV_AMBASSADOR)).collect();
    assert_eq!(held, vec![1, 4, 3]);
    // A tie: the lowest-numbered.
    put(&mut g, cities0[0], ADV_AMBASSADOR, 4);
    with_cx(&rules, |cx| adversary::surge(&mut g, cx, SiteType::City, 1, Some(0), "test")).unwrap();
    let after = |h: usize| (g.tokens(h, ADV_CUBE), g.tokens(h, ADV_AMBASSADOR));
    assert_eq!((after(cities0[0]), after(cities0[1])), ((1, 0), (0, 4)), "its 5th agent corrupts it (d0: a city's 5)");
}

/// 275: Spill: X into Y does nothing below `spill_at` (2); from it, each X holding 2 or more moves 1 onto the Y
/// next to it, an arrival there.
#[test]
fn a_spill_waits_for_its_level_then_moves_one_agent() {
    for (d, moved) in [(0, false), (2, true)] {
        let level = format!("adversary.difficulty={d}");
        let (rules, mut g) = lab_with(&[], &[&level]);
        let m = &rules.map;
        assert_eq!(rules.world().int("adversary.spill_at"), 2);
        set_var(&mut g, SUPPLY, 40);
        clear_agents(&mut g);
        let (c, lonely) = city_with_lonely_neighbors(m);
        let f = lonely[0];
        put(&mut g, f, FACTORY, 1);
        put(&mut g, c, ADV_AMBASSADOR, 2);
        with_cx(&rules, |cx| adversary::spill(&mut g, cx, SiteType::City, SiteType::Factory, None)).unwrap();
        let want = if moved { (1, 1) } else { (2, 0) };
        assert_eq!((g.tokens(c, ADV_AMBASSADOR), g.tokens(f, ADV_AMBASSADOR)), want, "d{d}");
        assert_eq!(total(&g, ADV_AMBASSADOR), 2, "d{d}: a move, no new agent");
    }
}

/// 275 (Matt, 2026-11-18): each botnet with no player ambassador on it seeds 1 agent onto the site next to it
/// holding the most agents, in the compute step; a player ambassador on the botnet stops it.
#[test]
fn a_botnet_seeds_an_agent_on_the_busiest_site_next_to_it() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    set_var(&mut g, SUPPLY, 40);
    clear_agents(&mut g);
    for h in 0..g.hexes.len() {
        put(&mut g, h, BOTNET, 0);
    }
    let (c, lonely) = city_with_lonely_neighbors(m);
    let b = lonely[0];
    put(&mut g, b, BOTNET, 1);
    put(&mut g, c, ADV_AMBASSADOR, 1);
    with_cx(&rules, |cx| adversary::botnets_seed(&mut g, cx)).unwrap();
    assert_eq!((g.tokens(c, ADV_AMBASSADOR), total(&g, ADV_AMBASSADOR)), (2, 2));
    put(&mut g, b, ambassador(0), 1);
    with_cx(&rules, |cx| adversary::botnets_seed(&mut g, cx)).unwrap();
    assert_eq!(total(&g, ADV_AMBASSADOR), 2, "a player ambassador on it stops it");
}

/// r133 (280, one move a botnet): a botnet passed as blocked whose ambassador a later botnet's seed cancels adds
/// its compute in the same pass, as r132's second read of the botnets did (seen on the traced seeds: Cintamani's two
/// starting botnets sit side by side).
#[test]
fn a_seed_that_cancels_a_blocking_ambassador_frees_that_botnets_compute() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let per = rules.world().int("adversary.botnet_compute");
    set_var(&mut g, SUPPLY, 40);
    clear_agents(&mut g);
    for h in 0..g.hexes.len() {
        put(&mut g, h, BOTNET, 0);
    }
    let (c, lonely) = city_with_lonely_neighbors(m);
    let b = *lonely.iter().find(|&&x| x > c).expect("a lonely neighbor after the city in hex order");
    put(&mut g, c, ADV_CUBE, 0);
    put(&mut g, c, BOTNET, 1);
    put(&mut g, c, ambassador(1), 1);
    put(&mut g, c, ADV_AMBASSADOR, 1);
    put(&mut g, b, BOTNET, 1);
    let pass = with_cx(&rules, |cx| adversary::botnets_seed(&mut g, cx)).unwrap();
    assert_eq!(g.tokens(c, ambassador(1)), 0, "the seed canceled the ambassador");
    assert_eq!((pass.compute, pass.blocked), (2 * per, 0));
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

/// 275: an arrival that corrupts a player's city or factory takes its cube off for the adversary's; a factory one
/// short holds.
#[test]
fn an_arrival_corruption_takes_a_players_cube() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    set_var(&mut g, SUPPLY, 40);
    let n = adversary::corrupt_n(&w, CorruptSite::City, 0);
    let aligned = m.numbered(0, piece(0).cities[0]);
    put(&mut g, aligned, ADV_AMBASSADOR, n - 1);
    put(&mut g, aligned, cube(0), 1);
    let open1 = numbered_where(m, 1, |h| kind(m, h) == HexKind::Open);
    let (factory, held) = (open1[0], open1[1]);
    for f in [factory, held] {
        put(&mut g, f, FACTORY, 1);
        put(&mut g, f, cube(1), 1);
    }
    put(&mut g, factory, ADV_AMBASSADOR, adversary::corrupt_n(&w, CorruptSite::Factory, 0) - 1);
    for h in [aligned, factory, held] {
        with_cx(&rules, |cx| adversary::place(&mut g, cx, h, 1)).unwrap();
    }
    assert_eq!((g.tokens(aligned, ADV_CUBE), g.tokens(aligned, cube(0)), g.tokens(aligned, ADV_AMBASSADOR)), (1, 0, 0));
    assert_eq!((g.tokens(factory, ADV_CUBE), g.tokens(factory, cube(1))), (1, 0));
    assert_eq!((g.tokens(held, ADV_CUBE), g.tokens(held, cube(1)), g.tokens(held, ADV_AMBASSADOR)), (0, 1, 1));
    assert_eq!(g.stats.counters["adversary.corruptions_arrival"], 2);
}

/// 138: corruption destroys only a node on the corrupted hex (its owner's
/// ambassador there is armor); a node next to it stays.
#[test]
fn corruption_destroys_only_the_node_on_its_hex_and_an_ambassador_is_armor() {
    let (rules, mut g0) = lab(&[]);
    let m = &rules.map;
    set_var(&mut g0, STAGE, 2);
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
    set_var(&mut g, STAGE,2);
    let shielded = |g: &GameState| crate::modules::cards::has_passive(&w, g, 1, |p| matches!(p, Op::CorruptionShield));
    assert!(!shielded(&g));
    set_var(&mut g, board::track_key(1), 7);
    assert!(shielded(&g), "seat 1 plays the Protector");
    let h = m.numbered(0, piece(0).cities[0]);
    put(&mut g, h, node(1), 1);
    put(&mut g, m.numbered(1, piece(1).node), node(1), 1);
    with_cx(&rules, |cx| adversary::corrupt_city(&mut g, cx, h)).unwrap();
    assert_eq!((g.tokens(h, ADV_CUBE), g.tokens(h, node(1))), (1, 1));
}

/// 235: 2 agents on a Sanctuary seat's node hex go to the supply and the node holds; the log says so (it said
/// "Agents destroy the node" with the node still standing). Unshielded, the log says destroy.
#[test]
fn the_node_rule_logs_what_happened_to_the_node() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    set_var(&mut g, STAGE,2);
    let h = m.numbered(1, piece(1).node);
    put(&mut g, h, node(1), 1);
    put(&mut g, m.numbered(0, piece(0).node), node(1), 1);
    let notes = |rules: &Rules, g: &mut GameState| {
        let mut events = Vec::new();
        let mut choices = module::Choices::new(Vec::new());
        let mut cx = test_cx(rules, &mut choices);
        cx.events = Some(&mut events);
        adversary::corrupt_hex(g, &mut cx, h, CorruptSite::Node).unwrap();
        events
            .into_iter()
            .filter_map(|e| match e {
                crate::Event::Note { text, .. } => Some(text),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" | ")
    };
    set_var(&mut g, board::track_key(1), 7);
    put(&mut g, h, ADV_AMBASSADOR, 2);
    let said = notes(&rules, &mut g);
    assert_eq!((g.tokens(h, node(1)), g.tokens(h, ADV_AMBASSADOR)), (1, 0));
    assert!(said.contains("the node holds") && !said.contains("destroy"), "{said}");
    set_var(&mut g, board::track_key(1), 0);
    put(&mut g, h, ADV_AMBASSADOR, 2);
    let said = notes(&rules, &mut g);
    assert_eq!(g.tokens(h, node(1)), 0);
    assert!(said.contains("Agents destroy the node"), "{said}");
}

/// 138: Fallout on an empty land hex keeps a node off it; 275: agents arriving on open land corrupt nothing
/// (Corrupt land is gone; Fallout comes only from printed lines).
#[test]
fn fallout_blocks_a_node() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let w = rules.world();
    let x = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open)[0];
    let mine = m.hexes[x].neighbors.iter().copied().find(|&y| kind(m, y) == HexKind::Open).unwrap();
    put(&mut g, mine, node(0), 1);
    assert!(board::node_sites(&w, &g, 0).contains(&x));
    set_var(&mut g, SUPPLY, 40);
    with_cx(&rules, |cx| adversary::place(&mut g, cx, x, 6)).unwrap();
    assert_eq!((g.tokens(x, FALLOUT), g.tokens(x, ADV_AMBASSADOR), g.tokens(x, ADV_CUBE)), (0, 6, 0));
    with_cx(&rules, |cx| adversary::place_fallout(&mut g, cx, x, "test"));
    assert!(!board::node_sites(&w, &g, 0).contains(&x));
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
    // Open land: no site to corrupt at 3 agents (275).
    let s = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open)[0];
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

/// 151: setup sets the compute income from the difficulty's list entry (275: no agent income); each compute step
/// then adds it onto the projects.
#[test]
fn the_adversary_gains_its_income_at_the_end_of_its_turn() {
    for (d, compute) in [(0, 5), (3, 7)] {
        let mut c = lab_config(&["adversary.reveal", "adversary.compute", ACTIONS]);
        let level = format!("adversary.difficulty={d}");
        apply_sets(&mut c, &[&level, "adversary.compute_income=5,5,5,7,7"]);
        let rules = Rules::new(&c).unwrap();
        let mut g = rules.new_game(1);
        answer_all(&rules, &mut g);
        let boards = rules.map.countries.len();
        // 231: the compute income is a country's.
        let compute = compute * boards as i64;
        assert_eq!(g.var(adversary::INCOME_COMPUTE), compute, "d{d}");
        assert_eq!(g.stats.counters["adversary.compute_income"], compute, "d{d}");
        // A row too dear to fill: the next reveal step puts the income on it, resolving nothing.
        row(&mut g, &["accelerated_timetable"]);
        set_pile(&mut g, adversary::PROJECTS_DONE, &[]);
        set_var(&mut g, adversary::project_key("accelerated_timetable"), 0);
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
    // Probing Strikes grows cities, Signal Jamming stations.
    set_pile(&mut g, CURRENT, &["mil_s1_probing_strikes"]);
    set_pile(&mut g, NEXT, &["mil_s1_signal_jamming"]);
    let stations = |g: &GameState| {
        adversary::twists(&w, g)
            .iter()
            .any(|t| matches!(t, Op::Grow { site: SiteType::Station }))
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
    set_var(&mut g, alignment::track(Side::Unity), pull + 1);
    g.piles.insert(NEXT.into(), vec![card.id.as_str().into()].into());
    run(&rules, &mut g);
    answer_all(&rules, &mut g);
    assert_eq!(g.stats.counters.get("alignment.unity_pulled").copied(), Some(pull));
    assert!(g.var(alignment::track(Side::Unity)) <= 1, "the bucket loses the pull");
}

/// 275: every event card opens on its Infiltrate X (two at stage III: Matt's look-ahead names next round's sites a
/// round ahead), keeps to the event standard (no per-event compute or creation line), and the deck's Infiltrates
/// name every site type.
#[test]
fn every_event_card_infiltrates_first() {
    let mut seen: Vec<SiteType> = Vec::new();
    for a in &content().adversary {
        assert!(matches!(a.standing.first(), Some(Op::Infiltrate { .. })), "{} infiltrates first", a.id);
        let sites = adversary::infiltrate_sites(a);
        assert_eq!(sites.len(), if a.stage == 3 { 2 } else { 1 }, "{}", a.id);
        assert_eq!(crate::content::event_rule_break(a), None, "{} keeps the event standard", a.id);
        assert!(
            !a.standing.iter().any(|e| matches!(e, Op::Compute { .. } | Op::Create { .. })),
            "{} prints no per-event compute or creation line",
            a.id
        );
        seen.extend(sites);
    }
    for site in [SiteType::City, SiteType::CoastalCity, SiteType::Station, SiteType::Factory, SiteType::Botnet] {
        assert!(seen.contains(&site), "no card infiltrates {site:?}");
    }
}

/// 151, 275: the load check refuses an event off the event standard (one Infiltrate X, the fixed step order, at
/// most two other lines, "each board") and a project with more than two consequences.
#[test]
fn the_load_check_refuses_an_event_or_a_project_off_the_standard() {
    use crate::content::{event_rule_break, project_rule_break};
    let c = content();
    // War Drums: Infiltrate | Sway, Grow.
    let base = c.adversary.iter().find(|a| a.id == "mil_s1_war_drums").unwrap();
    assert_eq!(event_rule_break(base), None);
    let infiltrate = base.standing[0].clone();
    let sway = base.twists[0].clone();
    let grow = base.twists.last().unwrap().clone();
    assert!(matches!(infiltrate, Op::Infiltrate { .. }) && matches!(grow, Op::Grow { .. }));
    let refused = |edit: &dyn Fn(&mut crate::content::AdversaryCard), why: &str| {
        let mut a = base.clone();
        edit(&mut a);
        let got = event_rule_break(&a);
        assert!(got.as_deref().is_some_and(|r| r.contains(why)), "{why}: {got:?}");
    };
    refused(&|a| a.standing.insert(0, infiltrate.clone()), "one Infiltrate X");
    refused(&|a| a.standing.insert(0, grow.clone()), "out of the fixed order");
    refused(&|a| a.twists.push(grow.clone()), "one Spill line and one Grow line");
    refused(
        &|a| {
            a.twists.insert(0, sway.clone());
            a.twists.insert(0, sway.clone());
        },
        "more than two other lines",
    );
    refused(&|a| a.text = "Muster 1 on each board.".into(), "each board");
    // Martial Law (15 a board) prints two consequences; a third is refused below `THIRD_EFFECT_COST` (227,
    // ruling 4; 231: 18 on the per-board scale) and allowed from it.
    let p = c.projects.iter().find(|p| p.id == "martial_law").unwrap();
    assert_eq!(p.effects.len(), 2);
    assert!(p.cost < crate::content::THIRD_EFFECT_COST);
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
/// supply. 275: on the arrival of the second.
#[test]
fn two_agents_destroy_a_node_and_leave_no_mark() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let h = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open)[0];
    put(&mut g, h, node(0), 1);
    put(&mut g, h, ADV_AMBASSADOR, 1);
    with_answers(&rules, &mut g, |_| 0, |g, cx| adversary::place(g, cx, h, 1));
    assert_eq!((g.tokens(h, node(0)), g.tokens(h, ADV_AMBASSADOR)), (0, 0));
    assert_eq!(g.tokens(h, FALLOUT), 0);
}

/// r98: a node a protection saves leaves no mark either (r83-r97 laid Fallout saved or not).
#[test]
fn a_saved_node_hex_takes_no_mark() {
    let (rules, mut g) = lab(&[]);
    let m = &rules.map;
    let h = numbered_where(m, 0, |h| kind(m, h) == HexKind::Open)[0];
    put(&mut g, h, node(0), 1);
    put(&mut g, h, HARDEN, 1);
    put(&mut g, h, ADV_AMBASSADOR, 1);
    let spend = |a: &crate::state::Ask| a.options.iter().position(|o| o.option == "spend the hardening token").expect("the hardening offered");
    with_answers(&rules, &mut g, spend, |g, cx| adversary::place(g, cx, h, 1));
    assert_eq!((g.tokens(h, node(0)), g.tokens(h, HARDEN)), (1, 0), "saved");
    assert_eq!(g.tokens(h, FALLOUT), 0);
}

/// r83 (Matt, 2026-11-11, #57): project slot 1's tag is gone: every slot of the Militarist's board is open.
#[test]
fn every_project_slot_is_open() {
    assert!(crate::content().adversary_board.project_slots.iter().all(|s| s.tag.is_none()));
}

/// r80 (184's D79 a): an adversary-factory line with no free zone lands its fallback agents on a factory, never a
/// firewalled one (as a creation line skips it). A factory's number is raised past the 2 here, so the arrival
/// doesn't corrupt it (275).
#[test]
fn the_adversary_factory_fallback_skips_a_firewalled_factory() {
    let (rules, mut g) = lab_with(&[], &["adversary.corrupt_factory_at=3"]);
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
    set_var(&mut g, SUPPLY,40);
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

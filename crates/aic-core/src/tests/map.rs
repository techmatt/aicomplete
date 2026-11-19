use super::*;

#[test]
fn a_neutral_station_can_be_claimed_at_setup_in_every_layout() {
    // 074: on its hex, next to the starting node, with no setup botnet next
    // to it, across a border too.
    let content = crate::content();
    for n in 1..=4u8 {
        let map = Map::build(content, n).unwrap();
        for c in 0..n as usize {
            let p = &content.map.countries[c];
            let h = map.numbered(c, p.neutral_station);
            assert_eq!(map.hexes[h].kind, HexKind::Open, "{n} countries, country {c}");
            assert!(map.adjacent(h, map.numbered(c, p.node)), "{n} countries, country {c}: in reach");
            for (o, country) in content.map.countries.iter().enumerate().take(n as usize) {
                for &s in &country.stations {
                    assert!(!map.adjacent(h, map.numbered(o, s)), "{n} countries, country {c}: next to a botnet");
                }
            }
        }
    }
}

#[test]
fn every_layout_builds_and_every_piece_follows_the_design() {
    let content = crate::content();
    for n in 1..=4u8 {
        let map = Map::build(content, n).unwrap();
        assert_eq!(map.hexes.len(), 16 * n as usize);
        for c in 0..n as usize {
            let p = &content.map.countries[c];
            let at = |num: u8| map.numbered(c, num);
            assert_eq!(p.cities.len(), 3, "country {c}");
            assert!(
                p.corrupted.len() >= 2,
                "country {c}: two cities to corrupt at difficulty 4"
            );
            assert_eq!(p.stations.len(), 1, "country {c}");
            // 190: 2-3, so each city has one next to it.
            assert!((2..=3).contains(&p.factories.len()), "country {c}");
            assert_eq!(p.industry.len(), 4, "country {c}: about 4 industrial zones");
            assert!(p.ocean.len() <= 2, "country {c}");
            let count = |k: HexKind| {
                map.countries[c]
                    .hexes
                    .iter()
                    .filter(|&&h| kind(&map, h) == k)
                    .count()
            };
            assert_eq!(count(HexKind::City), 3);
            assert_eq!(count(HexKind::Ocean), p.ocean.len());
            for &f in &p.industry {
                let h = at(f);
                assert!(map.hexes[h].industrial);
                assert!(
                    map.hexes[h]
                        .neighbors
                        .iter()
                        .any(|&x| map.hexes[x].country as usize == c
                            && kind(&map, x) == HexKind::City),
                    "country {c}: zone #{f} touches no city of its piece"
                );
            }
            for &city in &p.cities {
                let zones = p
                    .industry
                    .iter()
                    .filter(|&&z| map.adjacent(at(z), at(city)))
                    .count();
                assert!(
                    (1..=3).contains(&zones),
                    "country {c}: city #{city} has {zones} zones next to it"
                );
            }
            for &o in &p.ocean {
                assert!(
                    map.hexes[at(o)].neighbors.len() < 6,
                    "country {c}: ocean #{o} is inland with {n} countries"
                );
            }
            let start = at(p.node);
            assert!(
                map.hexes[start]
                    .neighbors
                    .iter()
                    .any(|&x| kind(&map, x) == HexKind::City),
                "country {c}: the starting node touches no city"
            );
        }
    }
}

#[test]
fn station_sites_are_open_free_land() {
    // 137: empty station sites are printed, at most 2 a country (190: two countries' went to setup stations), on
    // open land with no setup piece or industrial zone.
    let content = crate::content();
    for n in 1..=4u8 {
        let map = Map::build(content, n).unwrap();
        for (c, p) in content.map.countries.iter().enumerate().take(n as usize) {
            assert!(p.station_sites.len() <= 2, "country {c}");
            for &s in &p.station_sites {
                let h = map.numbered(c, s);
                assert_eq!(map.hexes[h].kind, HexKind::Open, "{n} countries, country {c}, site {s}");
                assert!(map.hexes[h].station_site);
                assert!(!map.hexes[h].industrial);
                assert!(!p.stations.contains(&s) && p.neutral_station != s && p.node != s);
                assert!(!p.more_stations.contains(&s));
            }
        }
    }
}

#[test]
fn every_city_starts_with_a_neutral_factory_and_a_neutral_station_next_to_it() {
    // 190 (Matt, 2026-11-11): where the map allows, in every layout; on this map it allows everywhere.
    let content = crate::content();
    for n in 1..=4u8 {
        let map = Map::build(content, n).unwrap();
        for (c, p) in content.map.countries.iter().enumerate().take(n as usize) {
            let stations: Vec<u8> = [&[p.neutral_station][..], &p.more_stations].concat();
            for &city in &p.cities {
                let h = map.numbered(c, city);
                let near = |l: &[u8]| l.iter().any(|&x| map.adjacent(h, map.numbered(c, x)));
                assert!(near(&p.factories), "{n} countries, country {c}, city {city}: a factory");
                assert!(near(&stations), "{n} countries, country {c}, city {city}: a station");
            }
        }
    }
}

#[test]
fn every_piece_prints_two_coastal_cities() {
    let (rules, _) = action_only(&["countries=4"]);
    for c in 0..4 {
        let n = rules.map.countries[c]
            .hexes
            .iter()
            .filter(|&&h| rules.map.hexes[h].coastal)
            .count();
        assert_eq!(n, 2, "country {c}");
    }
}

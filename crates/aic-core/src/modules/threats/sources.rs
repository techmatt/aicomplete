//! What feeds a threat (070): the pieces of its type, the target it names,
//! who may counter it and the source it makes when it executes.

use super::*;

/// Whether hex `h` holds an adversary piece of type `source` (070).
fn is_source(w: &World, g: &GameState, h: usize, source: ThreatSource) -> bool {
    match source {
        ThreatSource::Influence => corrupted_city(w, g, h),
        ThreatSource::Cyber => g.tokens(h, BOTNET) > 0 || corrupted_station(g, h),
        ThreatSource::Economic => corrupted_factory(g, h),
        ThreatSource::Military => g.tokens(h, TROOP) > 0,
    }
}

/// The sources of type `source` in country `c`, in hex order.
pub fn sources(w: &World, g: &GameState, c: usize, source: ThreatSource) -> Vec<usize> {
    w.map.countries[c].hexes.iter().copied().filter(|&h| is_source(w, g, h, source)).collect()
}

/// The source type `threat` prints.
pub(super) fn source_of(w: &World, threat: &str) -> ThreatSource {
    def(w, threat).source
}

/// The hex `threat` named as its target when dealt with no source, if any.
pub fn target(g: &GameState, threat: &str) -> Option<usize> {
    let v = g.var(&target_key(threat));
    (v > 0).then(|| v as usize - 1)
}

/// What `threat` in country `c` counts as its sources: the adversary's pieces
/// of its type there, and the target it named.
pub fn threat_sources(w: &World, g: &GameState, threat: &str, c: usize) -> Vec<usize> {
    let mut hexes = sources(w, g, c, source_of(w, threat));
    if let Some(h) = target(g, threat) {
        if !hexes.contains(&h) {
            hexes.push(h);
        }
    }
    hexes
}

/// Whether seat `s` may counter `threat` in `scope`: one of its sources in
/// reach, or its ambassador on one (070). A shared-row threat is anyone's.
pub fn may_counter(w: &World, g: &GameState, s: usize, threat: &str, scope: Option<usize>) -> bool {
    let Some(c) = scope else {
        return true;
    };
    let k = ambassador(s);
    let open = |h: usize| g.tokens(h, k) > 0 || in_reach(w, g, s, h);
    let source = source_of(w, threat);
    target(g, threat).is_some_and(open)
        || w.map.countries[c].hexes.iter().any(|&h| is_source(w, g, h, source) && open(h))
}

/// The active threats seat `s` may counter, of type `source` if given.
pub fn counterable(w: &World, g: &GameState, s: usize, source: Option<ThreatSource>) -> Vec<Key> {
    active(g)
        .into_iter()
        .filter(|(t, scope)| {
            source.is_none_or(|x| x == source_of(w, t)) && may_counter(w, g, s, t, *scope)
        })
        .map(|(t, _)| t)
        .collect()
}

/// Whether a face-up threat of `source` in `h`'s country holds the adversary's
/// pieces of that type there (070: they can't be destroyed or freed until it
/// is countered). A shared-row threat has no country and holds nothing.
pub fn shielded(w: &World, g: &GameState, h: usize, source: ThreatSource) -> bool {
    g.pile(&on(w.map.hexes[h].country as usize)).iter().any(|t| def(w, t).source == source)
}

/// What a threat with no source in country `c` names (070): the lowest clean
/// piece of its type, neutral before player-owned; for a factory or station
/// with none to take, the lowest site one would be built on; for troops the
/// lowest base a Wall leaves open, else the lowest base. `None`: nothing to name.
pub(super) fn pick_target(w: &World, g: &GameState, c: usize, source: ThreatSource) -> Option<usize> {
    let hexes = &w.map.countries[c].hexes;
    let lowest = |f: &dyn Fn(usize) -> bool| hexes.iter().copied().filter(|&h| f(h)).min();
    let clean = |kind: &str| {
        let on = |h: usize| g.tokens(h, kind) > 0 && !corrupted(g, h);
        lowest(&|h| on(h) && cube_owner(g, h).is_none()).or_else(|| lowest(&on))
    };
    match source {
        ThreatSource::Influence => lowest(&|h| clean_city(w, g, h)).or_else(|| lowest(&|h| collecting_city(w, g, h))),
        ThreatSource::Cyber => clean(STATION).or_else(|| lowest(&|h| buildable(w, g, h) && !w.map.hexes[h].industrial)),
        ThreatSource::Economic => clean(FACTORY).or_else(|| lowest(&|h| w.map.hexes[h].industrial && !has_building(g, h))),
        ThreatSource::Military => lowest(&|h| is_base(w, h) && !board::walled(w, g, h)).or_else(|| lowest(&|h| is_base(w, h))),
    }
}

/// An uncountered threat's target becomes its source, before its own effect:
/// the city or piece is corrupted, built first if it must be, or the troop
/// lands. One a country a round; a Wall stops the landing.
pub(super) fn make_source(g: &mut GameState, cx: &mut Cx, c: usize, source: ThreatSource, h: usize) {
    let w = cx.world;
    if g.var(&made_key(c)) == g.round as i64 {
        return;
    }
    match source {
        ThreatSource::Influence => {
            if corrupted(g, h) {
                return;
            }
            adversary::corrupt_city(g, cx, h);
        }
        ThreatSource::Cyber => {
            if g.tokens(h, STATION) == 0 {
                if !buildable(&w, g, h) {
                    return;
                }
                cx.add_tokens(g, h, STATION, 1);
            }
            if corrupted(g, h) {
                return;
            }
            adversary::corrupt_station(g, cx, h);
        }
        ThreatSource::Economic => {
            if g.tokens(h, FACTORY) == 0 {
                if has_building(g, h) {
                    return;
                }
                cx.add_tokens(g, h, FACTORY, 1);
            }
            if corrupted(g, h) {
                return;
            }
            adversary::corrupt_factory(g, cx, h);
        }
        ThreatSource::Military => {
            if board::walled(&w, g, h) {
                return;
            }
            adversary::place_troop(g, cx, h);
        }
    }
    cx.set_key(g, &made_key(c), g.round as i64);
    cx.tally(g, "threats.sources_made", 1);
    cx.note(None, || format!("{} source appears in {}", source.a_word(), country_name(&w, c)));
}

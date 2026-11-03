//! War on the fronts, and the march of adversary troops.

use super::*;

/// The nearest hex to `from` among `hexes` (ties: country, then lower hex number).
pub(super) fn nearest(w: &World, from: usize, hexes: impl Iterator<Item = usize>) -> Option<usize> {
    hexes.min_by_key(|&h| {
        (
            w.map.distance(from, h),
            w.map.hexes[h].country,
            w.map.hexes[h].number,
        )
    })
}

/// Production step 2, on a war round: on each front the units cancel 1 for
/// 1; each surplus unit crosses and destroys the rival factory nearest its
/// base, taking any presence on its hex (027: a presence was the target with
/// no factory left). All units at the front are used up.
pub(super) fn battle(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    if !war_round(&w, g) {
        return;
    }
    cx.note(None, || "War round: the fronts fight".into());
    let mut destroyed = 0;
    for f in &w.map.fronts {
        let [ba, bb] = f.bases;
        let (ua, ub) = (g.tokens(ba, UNIT), g.tokens(bb, UNIT));
        cx.add_tokens(g, ba, UNIT, -ua);
        cx.add_tokens(g, bb, UNIT, -ub);
        let (from, rival, n) = if ua > ub {
            (ba, f.countries[1], ua - ub)
        } else {
            (bb, f.countries[0], ub - ua)
        };
        for _ in 0..n {
            let hexes = &w.map.countries[rival as usize].hexes;
            let target = nearest(
                &w,
                from,
                hexes
                    .iter()
                    .copied()
                    .filter(|&h| g.tokens(h, FACTORY) > 0 && !corrupted(g, h)),
            );
            let Some(h) = target else {
                break;
            };
            if destroy_piece(g, cx, h, FACTORY, Cause::War) {
                destroyed += 1;
                // War that destroys a facility takes the presence on its hex
                // (027) and, by `war_reach`, around it (034).
                for x in within(&w, h, w.int("board.war_reach")) {
                    if let Some(s) = presence_owner(g, x) {
                        lose_presence(g, cx, x, s, "war");
                    }
                }
            }
        }
    }
    cx.tally(g, "board.war_rounds", 1);
    cx.tally(g, "board.war_destroyed", destroyed);
}

/// The adversary phase's march (019): every adversary troop, one at a time
/// (in hex-number order, fixed when the march starts), takes `troop_steps`
/// steps. A step destroys a target on or next to its hex (players choose)
/// or, with none there, moves one hex toward the nearest target (players
/// choose among equally good steps; with no target on the board it stays).
/// A troop that strikes is used up (`troop_spent`).
pub(super) fn march(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let steps = w.int("board.troop_steps");
    let troops: Vec<usize> = (0..g.hexes.len())
        .flat_map(|h| std::iter::repeat_n(h, g.tokens(h, TROOP).max(0) as usize))
        .collect();
    // Troops that have already marched, per hex (none marches twice).
    let mut marched = vec![0i64; g.hexes.len()];
    for from in troops {
        if g.tokens(from, TROOP) - marched[from] <= 0 {
            continue;
        }
        let mut at = from;
        let mut alive = true;
        for _ in 0..steps {
            // 075: the measure's: a facility a Wall kept this troop off.
            if next_to(&w, at, |x| walled(&w, g, x) && under_wall(g, x)) {
                cx.tally(g, "board.wall_strikes", 1);
            }
            if has_troop_target(&w, g, at) {
                cx.tally(g, "board.troop_steps", 1);
                if !strike_from(g, cx, at)? {
                    break;
                }
                if w.int("board.troop_spent") > 0 {
                    alive = false;
                    break;
                }
            } else {
                let Some(to) = step_toward(g, cx, at)? else {
                    break;
                };
                if w.map.hexes[at].neighbors.iter().any(|&n| walled(&w, g, n)) {
                    cx.tally(g, "board.wall_steps", 1);
                }
                cx.add_tokens(g, at, TROOP, -1);
                cx.add_tokens(g, to, TROOP, 1);
                cx.per_round(g, adversary::OPS, 1);
                cx.tally(g, "board.troop_steps", 1);
                cx.tally(g, "board.troop_moves", 1);
                at = to;
            }
            if g.is_over() {
                return Ok(());
            }
        }
        if alive {
            marched[at] += 1;
        }
    }
    Ok(())
}

/// What an adversary troop may destroy on hex `h`, as token keys: a
/// presence, and a player-owned research station or factory (019: not
/// neutral or corrupted ones, and never cities).
pub(super) fn troop_targets_on(w: &World, g: &GameState, h: usize) -> Vec<String> {
    let mut out = Vec::new();
    // 075: a Wall stops the strike as well as the walk.
    if walled(w, g, h) {
        return out;
    }
    if cube_owner(g, h).is_some() && !corrupted(g, h) {
        if g.tokens(h, FACTORY) > 0 {
            out.push(FACTORY.to_string());
        }
        if g.tokens(h, STATION) > 0 {
            out.push(STATION.to_string());
        }
    }
    if g.var(adversary::STAGE) >= w.int("board.troop_presence") {
        if let Some(s) = presence_owner(g, h) {
            out.push(presence(s).to_string());
        }
    }
    out
}

/// Whether hex `h` holds a player-owned facility, what a Wall there saves.
fn under_wall(g: &GameState, h: usize) -> bool {
    cube_owner(g, h).is_some() && !corrupted(g, h) && (g.tokens(h, FACTORY) > 0 || g.tokens(h, STATION) > 0)
}

/// Whether hex `h` holds anything an adversary troop targets.
fn is_troop_target(w: &World, g: &GameState, h: usize) -> bool {
    !troop_targets_on(w, g, h).is_empty()
}

/// Whether an adversary troop at `h` has a target on or next to it.
fn has_troop_target(w: &World, g: &GameState, h: usize) -> bool {
    next_to(w, h, |x| is_troop_target(w, g, x))
}

/// Whether seat `s` has Wall: `Content::check` keeps it to innates, so no
/// card's passives need reading.
pub fn has_wall(w: &World, seat: usize) -> bool {
    ai::board(w, seat).innate.passives.iter().any(|p| matches!(p, Op::Wall))
}

/// Whether hex `h` holds presence of a seat with Wall (068): adversary troops
/// can't enter it.
pub fn walled(w: &World, g: &GameState, h: usize) -> bool {
    presence_owner(g, h).is_some_and(|s| has_wall(w, s))
}

/// The hexes adversary troops can't enter now, as a mask.
fn wall_mask(w: &World, g: &GameState) -> Vec<bool> {
    let mut mask = vec![false; g.hexes.len()];
    for s in (0..g.seats.len()).filter(|&s| has_wall(w, s)) {
        let k = presence(s);
        for (h, m) in mask.iter_mut().enumerate() {
            *m |= g.tokens(h, k) > 0;
        }
    }
    mask
}

/// Walking distance over land from the nearest of `sources`, for every hex
/// (`None`: no land path). Troops never enter ocean, and some pieces print
/// ocean between hexes, so straight-line distance would strand a troop. Nor
/// do they enter a walled hex, though one can be a source: a target behind a
/// Wall is struck from the hex next to it.
fn land_distances(w: &World, walls: &[bool], sources: impl Iterator<Item = usize>) -> Vec<Option<u32>> {
    let mut dist: Vec<Option<u32>> = vec![None; w.map.hexes.len()];
    let mut queue = std::collections::VecDeque::new();
    for h in sources {
        if dist[h].is_none() {
            dist[h] = Some(0);
            queue.push_back(h);
        }
    }
    while let Some(h) = queue.pop_front() {
        let d = dist[h].unwrap_or(0);
        for &n in &w.map.hexes[h].neighbors {
            if dist[n].is_none() && !is_ocean(w, n) && !walls[n] {
                dist[n] = Some(d + 1);
                queue.push_back(n);
            }
        }
    }
    dist
}

/// Walking distance from the nearest troop target, for every hex (`None`
/// with no target reachable over land).
fn target_gaps(w: &World, g: &GameState) -> Vec<Option<u32>> {
    land_distances(w, &wall_mask(w, g), (0..g.hexes.len()).filter(|&h| is_troop_target(w, g, h)))
}

/// The land hexes next to `at` that bring a troop closer to the nearest
/// target, in hex order, none of them walled (068).
pub fn troop_steps_from(w: &World, g: &GameState, at: usize) -> Vec<usize> {
    let gaps = target_gaps(w, g);
    // A walled hex has no distance of its own unless it holds a target: a troop
    // standing on one (it was there first, or a factory made it) walks off.
    let open = w.map.hexes[at].neighbors.iter().filter_map(|&n| gaps[n]).min().map(|d| d + 1);
    let Some(d) = gaps[at].or(open) else {
        return Vec::new();
    };
    w.map.hexes[at]
        .neighbors
        .iter()
        .copied()
        .filter(|&n| !is_ocean(w, n) && !walled(w, g, n) && gaps[n].is_some_and(|x| x < d))
        .collect()
}

/// One move of the adversary troop at `at` toward the nearest target
/// (players choose among equally good steps). `None`: nowhere to go.
fn step_toward(g: &mut GameState, cx: &mut Cx, at: usize) -> Res<Option<usize>> {
    let w = cx.world;
    let steps = troop_steps_from(&w, g, at);
    let opts = steps.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = format!(
        "The adversary troop at {} moves to which hex?",
        hex_name(&w, at)
    );
    Ok(cx.choose(0, &prompt, opts)?.map(|i| steps[i]))
}

/// One adversary troop at `h` strikes a target on or next to its hex
/// (players choose): it is destroyed or the hit absorbed, and the troop is
/// used up (`troop_spent`). Returns false when it had no target.
fn strike_from(g: &mut GameState, cx: &mut Cx, h: usize) -> Res<bool> {
    let w = cx.world;
    let mut targets: Vec<(usize, String)> = Vec::new();
    for x in within(&w, h, 1) {
        for k in troop_targets_on(&w, g, x) {
            targets.push((x, k));
        }
    }
    let opts = targets
        .iter()
        .map(|(x, k)| opt(format!("{} at {}", piece_name(k), hex_name(&w, *x))))
        .collect();
    let prompt = format!(
        "Adversary troop at {}: which target does it destroy?",
        hex_name(&w, h)
    );
    let Some(i) = cx.choose_loss(0, &prompt, opts)? else {
        return Ok(false);
    };
    let (x, k) = targets[i].clone();
    cx.tally(g, "board.troop_strikes", 1);
    if destroy_piece(g, cx, x, &k, Cause::Troop) {
        let kind = if k == FACTORY || k == STATION { k.as_str() } else { "presence" };
        cx.tally(g, &format!("board.troop_destroyed.{kind}"), 1);
        cx.tally(g, "board.troop_destroyed", 1);
    } else {
        cx.tally(g, "board.troop_absorbed", 1);
    }
    if w.int("board.troop_spent") > 0 {
        cx.add_tokens(g, h, TROOP, -1);
        cx.tally(g, "board.troops_spent", 1);
    }
    Ok(true)
}

/// Where the adversary troops on hex `h` are heading, for the view: the
/// nearest target over land (ties: country, then hex number; the players
/// choose in play) and the march in which they strike (1: the next one).
pub fn troop_plan(w: &World, g: &GameState, h: usize) -> Option<(usize, i64)> {
    if w.int("board.troop_steps") <= 0 {
        return None;
    }
    // The walk from the troop's side: a walled target ends a walk, so it is
    // measured as one step past the nearest open hex next to it.
    let walls = wall_mask(w, g);
    let from = land_distances(w, &walls, std::iter::once(h));
    let reach = |x: usize| -> Option<u32> {
        if !walls[x] || x == h {
            from[x]
        } else {
            w.map.hexes[x].neighbors.iter().filter_map(|&n| from[n]).min().map(|d| d + 1)
        }
    };
    let target = (0..g.hexes.len())
        .filter(|&x| is_troop_target(w, g, x) && reach(x).is_some())
        .min_by_key(|&x| (reach(x), w.map.hexes[x].country, w.map.hexes[x].number))?;
    let steps = w.int("board.troop_steps");
    // Moves until the target is on or next to the troop, then the strike.
    let moves = (reach(target).unwrap_or(0) as i64 - 1).max(0);
    Some((target, (moves + steps) / steps))
}

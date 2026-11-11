//! Adversary compute and the project deck: the slots, the compute step, resolving, draining and discarding.

use super::*;

/// Compute on visible project `id`.
pub fn project_key(id: &str) -> KeyBuf {
    KeyBuf::new("adversary.project.").text(id)
}

/// The visible projects, in row order.
pub fn projects(g: &GameState) -> Vec<Key> {
    g.pile(PROJECTS).to_vec()
}

/// Visible projects holding compute: what a drain can act on.
pub fn projects_with_compute(g: &GameState) -> Vec<Key> {
    let mut ids = projects(g);
    ids.retain(|p| g.var(&project_key(p)) > 0);
    ids
}

/// Seat `seat` picks a project holding compute, and `amount` drains from it.
pub fn choose_drain(g: &mut GameState, cx: &mut Cx, seat: usize, amount: i64) -> Res {
    let ids = projects_with_compute(g);
    let opts = ids.iter().map(|p| opt(*p)).collect();
    let prompt = || format!("Seat {seat}: remove compute from which adversary project?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        drain_project(g, cx, &ids[i], amount);
    }
    Ok(())
}

pub(super) fn project<'w>(w: &World<'w>, id: &str) -> &'w ProjectCard {
    w.content.project(id)
}

/// Compute that fills project `id`: its printed cost per player.
pub fn project_cost(w: &World, g: &GameState, id: &str) -> i64 {
    project(w, id).cost * g.seats.len().max(1) as i64
}

/// Draws the top of the project deck; an empty deck reshuffles the discard
/// first (counted in `reshuffles`). None: both are empty.
fn draw_project(g: &mut GameState, cx: &mut Cx, reshuffles: &mut u32) -> Option<Key> {
    if g.pile(PROJECT_DECK).is_empty() {
        let mut ids = g.pile(PROJECT_DISCARD).to_vec();
        if ids.is_empty() {
            return None;
        }
        cx.set_pile(g, PROJECT_DISCARD, Vec::new());
        shuffle(&mut ids, cx.adversary_rng(g));
        cx.set_pile(g, PROJECT_DECK, ids);
        *reshuffles += 1;
        cx.tally(g, "adversary.project_reshuffles", 1);
    }
    g.pile(PROJECT_DECK).first().cloned()
}

/// Fills the board's project slots by the draw rule (135): draw; the card goes
/// into the first empty slot it fits (a slot's tag, or any; Universal fits
/// every slot), else to the discard; the discard reshuffles when the deck runs
/// out; a slot still unfilled after one reshuffle takes the next card anyway
/// (tallied: a content bug). The row is the slots in order. 175: only the empty slots are dealt (each slot
/// refills as it empties); a visible project keeps its slot (`slot_key`).
pub(super) fn deal_projects(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let slots = &w.content.adversary_board.project_slots;
    let mut filled: Vec<Option<Key>> = vec![None; slots.len()];
    let mut loose = Vec::new();
    for id in g.pile(PROJECTS).to_vec() {
        let k = g.var(&slot_key(&id)) - 1;
        if k >= 0 && (k as usize) < filled.len() && filled[k as usize].is_none() {
            filled[k as usize] = Some(id);
        } else {
            loose.push(id);
        }
    }
    // A visible project with no slot recorded (a state built by hand) takes the first empty slot it fits.
    for id in loose {
        let tags = &project(&w, &id).tags;
        let fits = |k: usize| filled[k].is_none() && slots[k].tag.is_none_or(|t| ThreatTag::fits(tags, &[PullTag::Is(t)]));
        if let Some(k) = (0..slots.len()).find(|&k| fits(k)).or_else(|| filled.iter().position(Option::is_none)) {
            cx.set_key(g, &slot_key(&id), k as i64 + 1);
            filled[k] = Some(id);
        }
    }
    let mut reshuffles = 0u32;
    while filled.iter().any(Option::is_none) {
        let Some(id) = draw_project(g, cx, &mut reshuffles) else {
            break;
        };
        // 151 (amendment 6618): what share of the deck a game turns up.
        cx.tally(g, "adversary.projects_revealed", 1);
        cx.tally(g, &KeyBuf::new("adversary.revealed.").text(&id), 1);
        let tags = &project(&w, &id).tags;
        let fits = |k: usize| {
            filled[k].is_none()
                && slots[k].tag.is_none_or(|t| ThreatTag::fits(tags, &[PullTag::Is(t)]))
        };
        let at = (0..slots.len()).find(|&k| fits(k));
        let at = match at {
            Some(k) => Some(k),
            None if reshuffles >= 2 => {
                cx.tally(g, "adversary.project_pull_forced", 1);
                filled.iter().position(Option::is_none)
            }
            None => None,
        };
        let to = if at.is_some() { PROJECTS } else { PROJECT_DISCARD };
        cx.move_card(g, &id, PROJECT_DECK, to, false)
            .expect("top of the project deck");
        if let Some(k) = at {
            cx.set_key(g, &slot_key(&id), k as i64 + 1);
            filled[k] = Some(id);
        }
    }
    // The row in slot order.
    let row: Vec<Key> = filled.into_iter().flatten().collect();
    cx.set_pile(g, PROJECTS, row);
}

/// 175: the slot a visible project sits in (1-based; 0: none).
fn slot_key(id: &str) -> KeyBuf {
    KeyBuf::new("adversary.slot.").text(id)
}

/// A project leaves its slot (resolved or discarded): the slot refills at once (175).
fn leave_slot(g: &mut GameState, cx: &mut Cx, id: &str) {
    cx.set_key(g, &slot_key(id), 0);
    deal_projects(g, cx);
}

/// 175: compute waiting for the adversary's turn: every source banks here, and the compute step adds it to
/// the projects.
pub const WAITING: &str = "adversary.compute_waiting";
/// 175: compute waiting to go onto every visible project (`compute_every`), in the same step.
pub const WAITING_EVERY: &str = "adversary.compute_every_waiting";

/// The adversary gains `n` compute (175): it waits beside the projects until the compute step of the
/// adversary's turn ([`compute_step`]); no project resolves outside that step.
pub fn gain_compute(g: &mut GameState, cx: &mut Cx, n: i64) -> Res {
    if n <= 0 {
        return Ok(());
    }
    cx.add_key(g, WAITING, n);
    cx.add_key(g, COMPUTE, n);
    cx.tally(g, "adversary.compute_made", n);
    cx.per_round(g, "adversary.compute", n);
    Ok(())
}

/// 175, the compute step (the end of the adversary's turn): all compute is added once: the printed income,
/// each botnet without a player ambassador on it (and not hacked this round), each corrupted research station
/// without one, and everything banked since the last step (threats, cards, projects). `compute_every` goes
/// on every visible project first; then each point onto the cheapest visible project, a full one resolving
/// at once and its slot refilling.
pub fn compute_step(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let income = g.var(INCOME_COMPUTE).max(0);
    cx.tally(g, "adversary.compute_income", income);
    let per = w.int("adversary.botnet_compute");
    let (mut botnets, mut blocked) = (0, 0);
    for h in 0..g.hexes.len() {
        let n = g.tokens(h, BOTNET) * per;
        if n == 0 {
            continue;
        }
        let cut = cut_key(h);
        if g.var(&cut) > 0 {
            // A hack cut it off (021): no compute this turn.
            cx.set_key(g, &cut, 0);
            blocked += n;
        } else if player_ambassador(g, h).is_some() {
            blocked += n;
        } else {
            botnets += n;
        }
    }
    if blocked > 0 {
        cx.tally(g, "adversary.compute_blocked", blocked);
        cx.per_round(g, "adversary.compute_blocked", blocked);
    }
    cx.tally(g, "adversary.compute_botnets", botnets);
    // 137: an adversary station makes compute; one player ambassador on it blocks it.
    let cubes = w.int("board.station_cubes");
    let mut stations = 0;
    for h in 0..g.hexes.len() {
        if g.tokens(h, STATION) == 0 || !corrupted(g, h) {
            continue;
        }
        if player_ambassador(g, h).is_some() {
            cx.tally(g, "board.blocked.station_adversary", 1);
        } else {
            stations += cubes;
        }
    }
    cx.tally(g, "board.station_compute", stations);
    let fresh = income + botnets + stations;
    cx.add_key(g, COMPUTE, fresh);
    cx.tally(g, "adversary.compute_made", fresh);
    cx.per_round(g, "adversary.compute", fresh);
    cx.add_key(g, WAITING, fresh);
    pour_waiting(g, cx)
}

/// 175: the waiting compute goes onto the projects (the compute step's second half): `compute_every` on every
/// visible project first, then each point onto the cheapest one (ties: the first slot), a full one resolving at
/// once and its slot refilling. With no project left, compute is lost.
pub fn pour_waiting(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let every = g.var(WAITING_EVERY);
    cx.set_key(g, WAITING_EVERY, 0);
    if every > 0 {
        spend_every(g, cx, every)?;
        if g.is_over() {
            return Ok(());
        }
    }
    let n = g.var(WAITING);
    cx.set_key(g, WAITING, 0);
    for _ in 0..n {
        let Some(id) = g
            .pile(PROJECTS)
            .iter()
            .min_by_key(|id| project_cost(&w, g, id))
            .cloned()
        else {
            cx.tally(g, "adversary.compute_lost", 1);
            continue;
        };
        cx.add_key(g, &project_key(&id), 1);
        cx.per_round(g, OPS, 1);
        if g.var(&project_key(&id)) >= project_cost(&w, g, &id) {
            resolve_project(g, cx, &id)?;
            if g.is_over() {
                return Ok(());
            }
        }
    }
    Ok(())
}

/// The adversary gains `n` compute on every visible project (074): 175, it waits for the compute step.
pub fn compute_every(g: &mut GameState, cx: &mut Cx, n: i64) -> Res {
    if n > 0 {
        cx.add_key(g, WAITING_EVERY, n);
    }
    Ok(())
}

/// `n` compute on every visible project, in the compute step. A project that fills resolves at once, and one
/// dealt by that gains nothing this time.
fn spend_every(g: &mut GameState, cx: &mut Cx, n: i64) -> Res {
    let w = cx.world;
    for id in projects(g) {
        cx.add_key(g, COMPUTE, n);
        cx.tally(g, "adversary.compute_made", n);
        cx.tally(g, "adversary.compute_corp", n);
        cx.tally(g, "adversary.compute_every", n);
        cx.per_round(g, "adversary.compute", n);
        cx.per_round(g, OPS, n);
        cx.add_key(g, &project_key(&id), n);
        if g.var(&project_key(&id)) >= project_cost(&w, g, &id) {
            resolve_project(g, cx, &id)?;
            if g.is_over() {
                return Ok(());
            }
        }
    }
    Ok(())
}

fn ongoing_from_key(id: &str) -> KeyBuf {
    KeyBuf::new("adversary.ongoing_from.").text(id)
}

/// The ongoing projects acting this round, in the order they came into play:
/// one that resolved this round acts from the next (145).
pub fn ongoing_acting(g: &GameState) -> Vec<Key> {
    g.pile(PROJECTS_ONGOING).iter().filter(|id| g.var(&ongoing_from_key(id.as_str())) <= g.round as i64).copied().collect()
}

/// A full project resolves: once (to the resolved pile), or as an ongoing
/// project from next round's build on. An empty row deals a new one.
fn resolve_project(g: &mut GameState, cx: &mut Cx, id: &str) -> Res {
    let w = cx.world;
    let p = project(&w, id);
    cx.set_key(g, &project_key(id), 0);
    let to = if p.ongoing {
        // 145: it acts from next round on (it acted in the build that filled it).
        cx.set_key(g, &ongoing_from_key(id), g.round as i64 + 1);
        PROJECTS_ONGOING
    } else {
        PROJECTS_DONE
    };
    cx.move_card(g, id, PROJECTS, to, false)
        .expect("visible project");
    cx.tally(g, "adversary.projects_resolved", 1);
    cx.tally(g, &KeyBuf::new("adversary.project.").text(id), 1);
    if g.var(FIRST_PROJECT) == 0 {
        cx.set_key(g, FIRST_PROJECT, g.round as i64);
        cx.tally(g, "adversary.first_project_round", g.round as i64);
    }
    cx.note(None, || {
        format!("Adversary project {} resolves: {}", p.name, p.text)
    });
    leave_slot(g, cx, id);
    let once = if p.ongoing { &p.resolve } else { &p.effects };
    effects::run(g, cx, once, &mut effects::Ctx::team().named("project").why(&p.name))?;
    Ok(())
}

/// Removes up to `n` compute from visible project `id` (Security cards).
pub fn drain_project(g: &mut GameState, cx: &mut Cx, id: &str, n: i64) {
    let k = n.min(g.var(&project_key(id))).max(0);
    if k > 0 {
        cx.add_key(g, &project_key(id), -k);
        cx.tally(g, "adversary.compute_drained", k);
    }
}

/// Discards visible project `id` with its compute; an empty row deals a
/// new one.
pub fn discard_project(g: &mut GameState, cx: &mut Cx, id: &str) {
    let w = cx.world;
    cx.set_key(g, &project_key(id), 0);
    cx.move_card(g, id, PROJECTS, PROJECTS_DONE, false)
        .expect("visible project");
    cx.tally(g, "adversary.projects_discarded", 1);
    cx.note(None, || {
        format!("Adversary project {} is discarded", project(&w, id).name)
    });
    leave_slot(g, cx, id);
}

/// An ongoing project's op that is read where it acts, not resolved each
/// round (135): a front fighting twice, corp troops each production.
pub(super) fn standing_project_op(e: &Op) -> bool {
    matches!(
        e,
        Op::ExtraWar { ongoing_use: true, .. }
            | Op::CorpTroops { when: crate::op::CorpWhen::EachProduction, .. }
    )
}

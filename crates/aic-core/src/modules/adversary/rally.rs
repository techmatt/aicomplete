//! Agent income and pools, Rally X, out-of-turn builds and creation lines.

use super::*;

/// 151: board `c`'s ambassador pool, waiting for the next Rally.
pub fn pool_key(c: usize) -> String {
    format!("adversary.pool.{c}")
}

/// 151: the income-list entry for this difficulty (the last entry past the list's end).
pub(super) fn income_at(w: &World, param: &str) -> i64 {
    let l = w.list(param);
    let d = w.int("adversary.difficulty").max(0) as usize;
    l.get(d).or(l.last()).copied().unwrap_or(0)
}

/// 151: each board's pool gains its ambassador income (the supply caps it at rally).
pub(super) fn fill_pools(g: &mut GameState, cx: &mut Cx) {
    let w = cx.world;
    let n = g.var(INCOME_AMBASSADORS).max(0);
    for c in 0..w.map.countries.len() {
        cx.add_key(g, &pool_key(c), n);
    }
}


/// 151: "increase income": for the rest of the game.
pub fn increase_income(g: &mut GameState, cx: &mut Cx, compute: i64, ambassadors: i64) {
    cx.add_key(g, INCOME_COMPUTE, compute);
    cx.add_key(g, INCOME_AMBASSADORS, ambassadors);
    cx.tally(g, "adversary.income_raised_compute", compute);
    cx.tally(g, "adversary.income_raised_ambassadors", ambassadors);
}

/// The site type the card ahead rallies to (its Rally line).
pub fn rally_site(c: &AdversaryCard) -> Option<crate::op::RallyAt> {
    c.standing.iter().find_map(|e| if let Op::Rally { site, count: 0 } = e { Some(*site) } else { None })
}

/// 151, Rally X in board `scope` or every board: each board places its pool (or `count`, a project's rally)
/// by [`super::plan::rally_plan`]: r80 (ruling #43) dealt round-robin onto the sites of the type in hex order;
/// r82: a firewalled or prohibited site's share stays in the supply (`adversary.rally_blocked`). The supply is a
/// cap. Tallied as `adversary.rallied` and by source.
pub fn rally(g: &mut GameState, cx: &mut Cx, site: crate::op::RallyAt, count: i64, scope: Option<usize>, source: &str) -> Res {
    let w = cx.world;
    let boards: Vec<usize> = match scope {
        Some(c) => vec![c],
        None => (0..w.map.countries.len()).collect(),
    };
    for c in boards {
        // 178 (Matt's ruling #30): a cordoned country's pool waits in the supply for its next Rally (it was lost).
        if board::flagged(g, crate::content::RoundFlag::Cordon, c) {
            continue;
        }
        let n = if count > 0 {
            count
        } else {
            let p = g.var(&pool_key(c));
            cx.set_key(g, &pool_key(c), 0);
            p
        };
        if n <= 0 {
            continue;
        }
        let Some(plan) = super::plan::rally_plan(&w, g, site, c, n) else {
            cx.tally(g, "adversary.rally_nowhere", n);
            continue;
        };
        let before = g.var(SUPPLY);
        // r82: a blocked site's share stays in the supply.
        for &(h, k) in &plan.shares {
            if k <= 0 {
                continue;
            }
            if firewalled(g, h) {
                cx.tally(g, "adversary.rally_blocked", k);
                continue;
            }
            match super::plan::prohibition_on(g, Step::Rally, h) {
                Some(wh) => {
                    cx.tally(g, "adversary.rally_blocked", k);
                    prohibit_tally(g, cx, Step::Rally, wh, k)
                }
                None => place(g, cx, h, k)?,
            }
        }
        let made = before - g.var(SUPPLY);
        cx.tally(g, "adversary.rallied", made);
        cx.tally(g, &format!("adversary.rallied.{source}"), made);
    }
    Ok(())
}

/// "The adversary builds now" (an out-of-turn build: Grand Offensive, the Offensive Discord card), in board
/// `scope` or every board. 151: each board in scope rallies its ambassador income now, to this round's card's
/// Rally site (none: nothing).
pub fn build_all(g: &mut GameState, cx: &mut Cx, scope: Option<usize>) -> Res {
    let w = cx.world;
    let Some(id) = g.pile(round_pile(g)).first().cloned() else {
        return Ok(());
    };
    let Some(site) = rally_site(card(&w, &id)) else {
        return Ok(());
    };
    let n = g.var(INCOME_AMBASSADORS);
    if n <= 0 {
        return Ok(());
    }
    rally(g, cx, site, n, scope, "build")
}

/// 138, a creation line: `count` agents on each of the
/// `sites` lowest-numbered matching sites in each country in scope (one
/// source with the agents' forecast, [`super::plan::create_plan`]). A
/// cordoned country gains none. Tallied by `source` (event, threat, project,
/// discord, unity, other).
pub fn create(g: &mut GameState, cx: &mut Cx, line: super::plan::CreateLine, scope: Option<usize>, near: &[usize], location: Option<usize>, source: &str) -> Res {
    let w = cx.world;
    let mut made = 0;
    for (h, n) in super::plan::create_plan(&w, g, line, scope, near, location) {
        let before = g.var(SUPPLY);
        place(g, cx, h, n)?;
        made += before - g.var(SUPPLY);
    }
    cx.tally(g, "adversary.created", made);
    cx.tally(g, &format!("adversary.created.{source}"), made);
    Ok(())
}

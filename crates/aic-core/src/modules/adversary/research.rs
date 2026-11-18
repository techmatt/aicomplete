//! 259 (Matt, 2026-11-16: every adversary gets a research rule): the adversary's research track. Its cubes come
//! from its corrupted stations once their country's megaproject is complete (in the compute step, in place of
//! compute), from projects and from adversary cards ([`gain_research`]); players' cards take them off
//! ([`remove_research`]). Once a round, at the end of the compute step, a track at or past the threshold
//! (`adversary.research_at` a country, lower with difficulty) spends it, keeps the overflow, and fires the board's
//! research rule: its stronger effect from `adversary.research_strong_at`.

use crate::module::{Cx, Res, World};
use crate::state::GameState;

use super::super::effects;
use super::level;
use super::rally::income_at;

/// The track: the adversary's research cubes (public, one counter on its board).
pub const RESEARCH: &str = "adversary.research";

/// The cubes that fire the research rule: `adversary.research_at` at this difficulty, a country (times the
/// countries, as compute is a board's). 0: never (no rule).
pub fn research_threshold(w: &World) -> i64 {
    if w.content.adversary_board.research.is_none() {
        return 0;
    }
    income_at(w, "adversary.research_at").max(0) * w.map.countries.len() as i64
}

/// The research rule is in play: the board prints one and this difficulty's threshold is above 0 (`research_at`
/// all 0 is the off arm: stations make compute again and research lines do nothing).
pub fn research_on(w: &World) -> bool {
    research_threshold(w) > 0
}

/// The adversary gains `n` research cubes onto its track.
pub fn gain_research(g: &mut GameState, cx: &mut Cx, n: i64, source: &str) {
    if n <= 0 || !research_on(&cx.world) {
        return;
    }
    cx.add_key(g, RESEARCH, n);
    // 261: say where the track's cubes came from (the playtest saw it fill with no word why).
    let (w, held) = (cx.world, g.var(RESEARCH));
    let from = match source {
        "stations" => "its corrupted stations",
        "threats" => "a threat",
        "event" => "its card",
        "project" => "a project",
        other => other,
    };
    cx.note(None, || {
        let cubes = if n == 1 { "cube" } else { "cubes" };
        format!("{n} {cubes} onto the adversary's research track from {from} ({held} of {})", research_threshold(&w))
    });
    cx.tally(g, "adversary.research_made", n);
    cx.tally(g, &format!("adversary.research_made.{source}"), n);
    cx.per_round(g, "adversary.research", n);
}

/// A player's card takes up to `n` cubes off the track.
pub fn remove_research(g: &mut GameState, cx: &mut Cx, n: i64) {
    let k = n.min(g.var(RESEARCH)).max(0);
    if k > 0 {
        cx.add_key(g, RESEARCH, -k);
        cx.tally(g, "adversary.research_removed", k);
    }
}

/// The end of the compute step: a track holding the threshold spends it and fires the rule once (the overflow
/// stays for next round).
pub fn research_step(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let at = research_threshold(&w);
    let Some(rule) = w.content.adversary_board.research.as_ref() else {
        return Ok(());
    };
    if at <= 0 || g.var(RESEARCH) < at {
        return Ok(());
    }
    cx.add_key(g, RESEARCH, -at);
    let strong = level(&w, w.int("adversary.research_strong_at"));
    cx.tally(g, "adversary.research_fired", 1);
    cx.tally(g, &format!("adversary.research_fired_r{:02}", g.round), 1);
    let ops = if strong { &rule.strong.effects } else { &rule.effects };
    cx.note(None, || format!("{} fires{}", rule.name, if strong { " (stronger)" } else { "" }));
    effects::run(g, cx, ops, &mut effects::Ctx::team().named("research").why(&rule.name))
}

//! 293 (Matt, 2026-11-19): the adversary's research track, a row of icon slots on every adversary board. Its cubes
//! come from its corrupted stations (one each a round, from round 1: the adversary's cubes go nowhere else), from
//! projects and from threats ([`gain_research`]); players' cards take them off ([`remove_research`]). In the Income
//! beat, after the compute step, the track spends every K cubes it holds (`adversary.research_at` a country) to
//! advance its marker one slot, and the slot's icon fires once, in each country
//! ([`crate::content::ResearchRow`]: the row is content, each slot an icon of the board's vocabulary). The slot
//! past the row's last is a loss. It replaced 259's research rule (one effect at a threshold, a stronger one by
//! difficulty).

use crate::content::RowIcon;
use crate::module::{Cx, Res, World};
use crate::state::GameState;
use crate::view::ViewOut;

use super::super::effects;
use super::rally::income_at;

/// The track's cubes toward its next slot (public, on the adversary's board).
pub const RESEARCH: &str = "adversary.research";
/// The slots the track's marker has reached (0: none; the row's length + 1: the loss).
pub const RESEARCH_SLOT: &str = "adversary.research_slot";

/// K for this game: the cubes that advance the track one slot: `adversary.research_at` at this difficulty, a
/// country (times the countries, as compute is a board's). 0: no track.
pub fn research_threshold(w: &World) -> i64 {
    if w.content.adversary_board.research.is_none() {
        return 0;
    }
    income_at(w, "adversary.research_at").max(0) * w.map.countries.len() as i64
}

/// The research track is in play: the board prints a row and this difficulty's K is above 0 (`research_at` all 0
/// is the off arm: stations make compute and research lines do nothing).
pub fn research_on(w: &World) -> bool {
    research_threshold(w) > 0
}

/// The icon in slot `slot` (1-based) of the board's row; none past its end.
pub fn slot_icon<'w>(w: &World<'w>, slot: i64) -> Option<&'w RowIcon> {
    let b = &w.content.adversary_board;
    let id = b.research.as_ref()?.slots.get(usize::try_from(slot - 1).ok()?)?;
    b.icons.iter().find(|i| &i.id == id)
}

/// The slots the board's row prints (six); the next one is the loss.
pub fn row_len(w: &World) -> i64 {
    w.content.adversary_board.research.as_ref().map_or(0, |r| r.slots.len() as i64)
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
        let at = research_threshold(&w);
        let fire = if held >= at { ": it advances at the research check" } else { "" };
        format!("{n} {cubes} onto the adversary's research track from {from} ({held} of {at}{fire})")
    });
    cx.tally(g, "adversary.research_made", n);
    cx.tally(g, &format!("adversary.research_made.{source}"), n);
    cx.per_round(g, "adversary.research", n);
}

/// A player's card takes up to `n` cubes off the track (never a slot back).
pub fn remove_research(g: &mut GameState, cx: &mut Cx, n: i64) {
    let k = n.min(g.var(RESEARCH)).max(0);
    if k > 0 {
        cx.add_key(g, RESEARCH, -k);
        cx.tally(g, "adversary.research_removed", k);
    }
}

/// The research check, the end of the Income beat: while the track holds K cubes it spends K and its marker
/// advances one slot, whose icon fires once (in each country, as it prints); the slot past the row's last ends
/// the game, lost.
pub fn research_step(g: &mut GameState, cx: &mut Cx) -> Res {
    let w = cx.world;
    let at = research_threshold(&w);
    if at <= 0 {
        return Ok(());
    }
    let len = row_len(&w);
    while g.var(RESEARCH) >= at && !g.is_over() {
        cx.add_key(g, RESEARCH, -at);
        cx.add_key(g, RESEARCH_SLOT, 1);
        let slot = g.var(RESEARCH_SLOT);
        cx.tally(g, "adversary.research_fired", 1);
        cx.tally(g, &format!("adversary.research_fired_r{:02}", g.round), 1);
        if slot > len {
            cx.note(None, || format!("The adversary's research track reaches slot {slot}: the game is lost"));
            cx.end(g, false, "research", format!("the adversary's research track reached slot {slot}"));
            return Ok(());
        }
        let Some(icon) = slot_icon(&w, slot) else {
            continue;
        };
        cx.note(None, || format!("The adversary's research track reaches slot {slot}: {} ({})", icon.name, icon.text));
        effects::run(g, cx, &icon.effects, &mut effects::Ctx::team().named("research").why(&icon.name))?;
    }
    Ok(())
}

/// The track as the board shows it: the marker's slot, the cubes toward the next, and the row.
pub(super) fn research_view(g: &GameState, w: &World, out: &mut ViewOut) {
    let (Some(r), true) = (&w.content.adversary_board.research, research_on(w)) else {
        return;
    };
    let at = research_threshold(w);
    let len = row_len(w);
    let slot = g.var(RESEARCH_SLOT);
    out.global(
        "research",
        "adv. research",
        g.var(RESEARCH),
        Some(at),
        &format!("the adversary's research track ({}): every {at} cubes advance it one slot in its Income beat", r.name),
    );
    let row: Vec<String> = (1..=len)
        .map(|k| {
            let name = slot_icon(w, k).map_or("?", |i| i.name.as_str());
            if k <= slot {
                format!("[{k} {name}]")
            } else {
                format!("{k} {name}")
            }
        })
        .collect();
    out.global(
        "research_slot",
        "adv. research slot",
        slot,
        Some(len + 1),
        &format!("the slots its marker has reached (bracketed), each icon fired once: {}; slot {}: the game is lost", row.join(", "), len + 1),
    );
}

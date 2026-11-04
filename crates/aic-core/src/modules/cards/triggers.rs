//! 105: the engine wording. Triggers (`Op::When`: an Effect, "when X, Y") on a
//! building, an AI board's innate or a capstone fire each time their event
//! happens, with no per-round limit; a `Threshold` cashes in the tokens a
//! trigger leaves on its card. Also the instant ops that read a seat's cards:
//! copy an income, reuse an Action, discard from hand for a resource.

use super::*;
use crate::content::Event;
use crate::state::Opt;

/// A seat's triggers, in a fixed order (innate, capstones, buildings by
/// slot): the card that holds each (none for a board's), its event and ops.
type Trigger<'w> = (Option<Key>, Event, &'w [Op]);

fn triggers<'w>(w: &World<'w>, g: &GameState, seat: usize) -> Vec<Trigger<'w>> {
    let mut out = Vec::new();
    let mut push = |card: Option<Key>, p: &'w Op| {
        if let Op::When { on, then } = p {
            out.push((card, *on, then.as_slice()));
        }
    };
    ai::board(w, seat).innate.passives.iter().for_each(|p| push(None, p));
    ai::capstones(w, g, seat).flat_map(|c| c.passives.iter()).for_each(|p| push(None, p));
    for id in g.pile(&built(seat)) {
        def(w, id).passives.iter().for_each(|p| push(Some(*id), p));
    }
    out
}

/// Whether seat `s` holds a trigger `f` accepts: the hot paths' early out.
fn has_trigger(w: &World, g: &GameState, seat: usize, f: impl Fn(&Event) -> bool) -> bool {
    holds(w, g, seat, |p| matches!(p, Op::When { on, .. } if f(on)))
}

/// Resolves a trigger's ops `times` times for `owner` ("here" is `card`), then
/// its card's threshold.
fn fire(g: &mut GameState, cx: &mut Cx, owner: usize, card: Option<Key>, then: &[Op], times: i64) -> Res {
    for _ in 0..times {
        let mut ctx = match card {
            Some(c) => effects::Ctx::ability(owner, c),
            None => effects::Ctx::seat(owner),
        };
        effects::run(g, cx, then, &mut ctx)?;
        cx.tally(g, "cards.triggers", 1);
        match card {
            Some(c) => cx.tally(g, &KeyBuf::new("cards.trigger.").text(&c), 1),
            None => cx.tally(g, &KeyBuf::new("cards.trigger.").text(&ai::board(&cx.world, owner).id), 1),
        }
        if g.is_over() {
            return Ok(());
        }
    }
    match card {
        Some(c) => threshold(g, cx, owner, c),
        None => Ok(()),
    }
}

/// A threshold counter: while `card` holds `at` or more tokens, `at` come off
/// and its ops resolve.
fn threshold(g: &mut GameState, cx: &mut Cx, owner: usize, card: Key) -> Res {
    let w = cx.world;
    for p in &def(&w, &card).passives {
        if let Op::Threshold { at, then } = p {
            while *at > 0 && g.var(&tokens_key(owner, &card)) >= *at {
                add_tokens(g, cx, owner, &card, -at);
                cx.tally(g, "cards.thresholds", 1);
                cx.tally(g, &KeyBuf::new("cards.threshold.").text(&card), 1);
                effects::run(g, cx, then, &mut effects::Ctx::ability(owner, card))?;
                if g.is_over() {
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

fn carries(d: &CardDef, tag: Option<Tag>) -> bool {
    tag.is_none_or(|t| d.tags.contains(&t))
}

/// Seat `player` has played `d` (it is in play, its effects resolved): every
/// seat's play triggers fire, seat by seat.
pub fn on_play(g: &mut GameState, cx: &mut Cx, player: usize, d: &CardDef) -> Res {
    let w = cx.world;
    let plays = |e: &Event| matches!(e, Event::Play { .. } | Event::PlayTag { .. } | Event::TeamPlay { .. } | Event::PlayCost { .. });
    for owner in 0..g.seats.len() {
        // The walk alone: most seats hold no play trigger (a play is a hot path).
        if !has_trigger(&w, g, owner, plays) {
            continue;
        }
        for (card, on, then) in triggers(&w, g, owner) {
            let mine = owner == player;
            let times = match on {
                Event::Play { tag } if mine => carries(d, tag) as i64,
                Event::PlayTag { tag } if mine => d.tags.iter().filter(|t| **t == tag).count() as i64,
                Event::TeamPlay { tag, .. } if !mine => carries(d, tag) as i64,
                Event::PlayCost { at } if mine => (d.cost >= at) as i64,
                _ => 0,
            };
            if times == 0 {
                continue;
            }
            fire(g, cx, owner, card, then, times)?;
            if let Event::TeamPlay { both: true, .. } = on {
                // Content::check: gains only, so the player's share asks nothing.
                for _ in 0..times {
                    effects::run(g, cx, then, &mut effects::Ctx::seat(player))?;
                }
            }
            if g.is_over() {
                return Ok(());
            }
        }
    }
    Ok(())
}

/// What the fight triggers count, before an action resolves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Watch {
    ambassadors: i64,
    troops: i64,
    compute: i64,
}

fn watch_now(g: &GameState) -> Watch {
    let (mut ambassadors, mut troops) = (0, 0);
    for h in 0..g.hexes.len() {
        ambassadors += g.tokens(h, ADV_AMBASSADOR).max(0);
        troops += g.tokens(h, TROOP).max(0);
    }
    let compute = adversary::projects(g).iter().map(|p| g.var(&adversary::project_key(p))).sum();
    Watch { ambassadors, troops, compute }
}

fn fights(e: &Event) -> bool {
    matches!(e, Event::RemoveTroop | Event::RemoveCompute | Event::RemoveAmbassadors)
}

/// The counts before seat `s` resolves an action, if it holds a fight trigger.
pub fn fight_watch(w: &World, g: &GameState, seat: usize) -> Option<Watch> {
    has_trigger(w, g, seat, fights).then(|| watch_now(g))
}

/// After seat `s`'s action: its fight triggers fire for what it removed (a
/// troop: each; compute or ambassadors: once for the action).
pub fn fight_fire(g: &mut GameState, cx: &mut Cx, seat: usize, before: Option<Watch>) -> Res {
    let Some(b) = before else {
        return Ok(());
    };
    if g.is_over() {
        return Ok(());
    }
    let w = cx.world;
    let now = watch_now(g);
    for (card, on, then) in triggers(&w, g, seat) {
        let times = match on {
            Event::RemoveTroop => (b.troops - now.troops).max(0),
            Event::RemoveCompute => (b.compute > now.compute) as i64,
            Event::RemoveAmbassadors => (b.ambassadors > now.ambassadors) as i64,
            _ => 0,
        };
        if times > 0 {
            fire(g, cx, seat, card, then, times)?;
        }
    }
    Ok(())
}

/// Seat `s` placed `n` tokens of `kind` on its cards.
pub fn on_tokens(g: &mut GameState, cx: &mut Cx, seat: usize, kind: TokenKind, n: i64) -> Res {
    let w = cx.world;
    if n <= 0 || !has_trigger(&w, g, seat, |e| *e == Event::PlaceTokens { kind }) {
        return Ok(());
    }
    for (card, on, then) in triggers(&w, g, seat) {
        if on == (Event::PlaceTokens { kind }) {
            fire(g, cx, seat, card, then, n)?;
        }
    }
    Ok(())
}

/// Seat `s`'s Wall stopped a troop in the march (Vigil).
pub fn on_wall(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
    for (card, on, then) in triggers(&w, g, seat) {
        if on == Event::WallStops {
            fire(g, cx, seat, card, then, 1)?;
        }
    }
    Ok(())
}

/// Whether seat `s` holds a Wall-stops trigger (the march asks before it looks).
pub fn has_wall_trigger(w: &World, g: &GameState, seat: usize) -> bool {
    has_trigger(w, g, seat, |e| *e == Event::WallStops)
}

// ---------- the instant ops that read a seat's cards ----------

/// A building's flat or per-tag income a copy can take (ambassadors are pieces,
/// in no income track).
fn copies(p: &Op) -> bool {
    matches!(p, Op::Income { resource, .. } | Op::IncomePerTag { resource, .. } if *resource != Resource::Ambassadors)
}

/// Seat `s`'s active buildings whose Each round output a copy can raise income by.
pub fn income_buildings(w: &World, g: &GameState, seat: usize) -> Vec<Key> {
    g.pile(&built(seat)).iter().filter(|id| def(w, id).passives.iter().any(copies)).cloned().collect()
}

/// Copy: seat `s` raises its income by one of its buildings' Each round output,
/// as it pays now.
pub fn copy_income(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
    let ids = income_buildings(&w, g, seat);
    let opts = ids.iter().map(|c| opt(*c)).collect();
    let prompt = || format!("Seat {seat}: raise your income by which building's output?");
    let Some(i) = cx.choose(seat, prompt, opts)? else {
        return Ok(());
    };
    let counts = tag_counts(&w, g, seat);
    for p in def(&w, &ids[i]).passives.iter().filter(|p| copies(p)) {
        let (r, n) = match p {
            Op::Income { resource, amount } => (*resource, *amount),
            Op::IncomePerTag { resource, tag, per } => (*resource, counts[*tag as usize] / (*per).max(1)),
            _ => continue,
        };
        if n != 0 {
            cx.add_key(g, &economy::production_key(seat, r), n);
        }
    }
    cx.tally(g, "cards.copied", 1);
    Ok(())
}

/// Whether `id`'s Action is itself a reuse: never reused (two would loop).
fn reuses(w: &World, id: &str) -> bool {
    def(w, id).ability.as_ref().is_some_and(|a| a.branches().iter().any(|b| b.effects.iter().any(|e| matches!(e, Op::Reuse))))
}

/// Seat `s`'s buildings, other than `this`, whose Action it used this round and
/// could use again now.
pub fn reusable(w: &World, g: &GameState, seat: usize, this: Option<&str>) -> Vec<Key> {
    g.pile(&built(seat))
        .iter()
        .filter(|id| {
            Some(id.as_str()) != this
                && !reuses(w, id)
                && g.seats[seat].exhausted.iter().any(|c| c == *id)
                && def(w, id).ability.as_ref().is_some_and(|a| a.branches().into_iter().any(|b| branch_blocked(w, g, seat, id, b).is_none()))
        })
        .cloned()
        .collect()
}

/// Reuse: seat `s` uses another of its buildings' Actions again this round.
pub fn reuse(g: &mut GameState, cx: &mut Cx, seat: usize, this: Option<Key>) -> Res {
    let w = cx.world;
    let ids = reusable(&w, g, seat, this.as_ref().map(|k| k.as_str()));
    let opts = ids.iter().map(|c| opt(*c)).collect();
    let prompt = || format!("Seat {seat}: use which building's Action again?");
    let Some(i) = cx.choose(seat, prompt, opts)? else {
        return Ok(());
    };
    cx.refresh(g, seat, &ids[i]);
    cx.tally(g, "cards.reused", 1);
    use_ability(g, cx, seat, &ids[i])
}

/// Seat `s` discards a card from its hand and gains `amount` of `resource`.
pub fn discard_hand(g: &mut GameState, cx: &mut Cx, seat: usize, resource: Resource, amount: i64) -> Res {
    let ids: Vec<Key> = g.pile(&hand(seat)).to_vec();
    let opts: Vec<Opt> = ids.iter().map(|c| opt(*c)).collect();
    let prompt = || format!("Seat {seat}: discard which card?");
    if let Some(i) = cx.choose(seat, prompt, opts)? {
        discard(g, cx, &ids[i], &hand(seat));
        cx.add_resource(g, seat, resource, amount);
        cx.tally(g, "cards.hand_discarded", 1);
    }
    Ok(())
}

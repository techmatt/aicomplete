//! Sacrifices for AI-board powers (021).

use super::*;

// ---------- sacrifices for AI-board powers (021) ----------

/// Something a seat can sacrifice to pay a power's tags: a program in its
/// play area, a whole building stack (named by its top), or (with a
/// `tags_from_hand` building) a card in hand.
#[derive(Debug, Clone)]
pub struct Source {
    pub label: String,
    /// The pile the cards leave, and the cards (a stack: all of it).
    pub cards: Vec<(String, Key)>,
    pub tags: Vec<Tag>,
}

/// What seat `s` could sacrifice now, in a fixed order: programs, stacks,
/// then hand cards a `tags_from_hand` building counts.
pub fn sources(w: &World, g: &GameState, seat: usize) -> Vec<Source> {
    let mut out = Vec::new();
    for id in g.pile(&played(seat)) {
        out.push(Source {
            label: format!("program {id}"),
            cards: vec![(played(seat).to_string(), id.clone())],
            tags: def(w, id).tags.clone(),
        });
    }
    for top in g.pile(&built(seat)) {
        let mut cards: Vec<(String, Key)> = g
            .pile(&under(seat, top))
            .iter()
            .map(|c| (under(seat, top).to_string(), c.clone()))
            .collect();
        cards.push((built(seat).to_string(), top.clone()));
        let tags = cards
            .iter()
            .flat_map(|(_, c)| def(w, c).tags.iter().copied())
            .collect();
        out.push(Source {
            label: format!("stack {top}"),
            cards,
            tags,
        });
    }
    let from_hand: Vec<TagOrAny> = passives(w, g, seat)
        .into_iter()
        .filter_map(|p| match p {
            Op::TagsFromHand { tag } => Some(*tag),
            _ => None,
        })
        .collect();
    if !from_hand.is_empty() {
        for id in g.pile(&hand(seat)) {
            let tags: Vec<Tag> = def(w, id)
                .tags
                .iter()
                .copied()
                .filter(|&t| from_hand.iter().any(|x| x.accepts(t)))
                .collect();
            if !tags.is_empty() {
                out.push(Source {
                    label: format!("hand {id}"),
                    cards: vec![(hand(seat).to_string(), id.clone())],
                    tags,
                });
            }
        }
    }
    out
}

/// A power's tag cost still unpaid: specific tags, and `any`.
#[derive(Debug, Clone)]
pub struct TagNeed {
    pub specific: BTreeMap<Tag, i64>,
    pub any: i64,
}

impl TagNeed {
    pub fn new(cost: &[(TagOrAny, i64)]) -> TagNeed {
        let mut need = TagNeed {
            specific: BTreeMap::new(),
            any: 0,
        };
        for (t, n) in cost {
            match t {
                TagOrAny::Any => need.any += n,
                TagOrAny::Is(t) => *need.specific.entry(*t).or_default() += n,
            }
        }
        need
    }

    pub fn met(&self) -> bool {
        self.any <= 0 && self.specific.values().all(|&n| n <= 0)
    }

    /// Whether `tags` would pay any of what is still needed.
    pub fn helps(&self, tags: &[Tag]) -> bool {
        tags.iter()
            .any(|t| self.any > 0 || self.specific.get(t).is_some_and(|&n| n > 0))
    }

    /// Pays with `tags`: each tag fills its own requirement first, then `any`.
    pub fn pay(&mut self, tags: &[Tag]) {
        for t in tags {
            match self.specific.get_mut(t) {
                Some(n) if *n > 0 => *n -= 1,
                _ => {
                    if self.any > 0 {
                        self.any -= 1
                    }
                }
            }
        }
    }

    /// Whether the tags of all of `sources` together could pay it (exact:
    /// a tag fills its own requirement first, the rest go to `any`).
    pub fn payable(&self, sources: &[Source]) -> bool {
        let mut left = self.clone();
        for s in sources {
            left.pay(&s.tags);
        }
        left.met()
    }
}

/// Seat `s` pays a power's tag cost by sacrificing sources one at a time
/// (its choice among those that still help), until it is paid.
pub fn sacrifice_for(g: &mut GameState, cx: &mut Cx, seat: usize, cost: &[(TagOrAny, i64)]) -> Res {
    let w = cx.world;
    let mut need = TagNeed::new(cost);
    let mut given = 0;
    // What can be sacrificed is fixed when the power is used: sacrificing a
    // `tags_from_hand` building mid-payment doesn't take its hand cards away.
    let mut left = sources(&w, g, seat);
    while !need.met() {
        let helping: Vec<Source> = left
            .iter()
            .filter(|s| need.helps(&s.tags))
            .cloned()
            .collect();
        if helping.is_empty() {
            return Err(Halt::Illegal("not enough tags to sacrifice".into()));
        }
        let opts = helping.iter().map(|s| opt(format!("sacrifice {}", s.label))).collect();
        let prompt = format!("Seat {seat}: sacrifice what for the power's tags?");
        let Some(i) = cx.choose(seat, &prompt, opts)? else {
            return Err(Halt::Illegal("no sacrifice chosen".into()));
        };
        let src = &helping[i];
        left.retain(|x| x.label != src.label);
        need.pay(&src.tags);
        given += src.tags.len() as i64;
        for (pile, card) in &src.cards {
            cx.set_key(g, &tokens_key(seat, card), 0);
            discard(g, cx, card, pile);
            cx.tally(g, "cards.sacrificed", 1);
        }
    }
    cx.tally(g, "ai.tags_sacrificed", given);
    cx.per_round(g, "ai.tags_sacrificed", given);
    Ok(())
}

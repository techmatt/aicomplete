//! Who resolves an op, who it hits and where.

use super::*;

/// Who is resolving an op.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectSource {
    /// A player's card, ability or power: that seat chooses.
    Seat(usize),
    /// An executing threat: seat 0 chooses for the team, the rest by scope.
    Threat,
    /// A Unity or Discord card or a project: seat 0 chooses for the team.
    Team,
}

/// How an op's placement pays (041).
pub(super) fn placing(free: bool) -> board::Placing {
    if free {
        board::Placing::Free
    } else {
        board::Placing::Step
    }
}

/// The context an op resolves in.
#[derive(Debug)]
pub struct Ctx {
    pub source: EffectSource,
    /// Where it applies: a threat's country, or the whole board.
    pub scope: Option<usize>,
    /// What earlier effects of the same card chose (the other player).
    pub other: Option<usize>,
    /// The kind of source, for tallying what it destroys.
    pub label: &'static str,
    /// The building whose Action this is: what "here" names (076).
    pub card: Option<Key>,
}

impl Ctx {
    pub(super) fn new(source: EffectSource, scope: Option<usize>) -> Ctx {
        Ctx { source, scope, other: None, label: "card", card: None }
    }

    /// The Action of seat `seat`'s building `card`: "here" is that card (076).
    pub fn ability(seat: usize, card: Key) -> Ctx {
        let mut c = Ctx::seat(seat);
        c.card = Some(card);
        c
    }

    /// Names the kind of source, for tallies (`discord`, `threat2`, ...).
    pub fn named(mut self, label: &'static str) -> Ctx {
        self.label = label;
        self
    }

    pub fn seat(seat: usize) -> Ctx {
        Ctx::new(EffectSource::Seat(seat), None)
    }

    pub fn threat(scope: Option<usize>) -> Ctx {
        Ctx::new(EffectSource::Threat, scope)
    }

    pub fn team() -> Ctx {
        Ctx::new(EffectSource::Team, None)
    }

    /// The seat that answers a choice.
    pub(super) fn chooser(&self) -> usize {
        match self.source {
            EffectSource::Seat(s) => s,
            EffectSource::Threat | EffectSource::Team => 0,
        }
    }

    /// The seats an effect on "players" hits.
    pub(super) fn seats(&self, w: &World, g: &GameState) -> Vec<usize> {
        match self.source {
            EffectSource::Seat(s) => vec![s],
            EffectSource::Threat | EffectSource::Team => seats_in_scope(w, g, self.scope),
        }
    }

    /// The hex sets an effect acts in: the scope as one, or with
    /// `each_country` one per country in it.
    pub(super) fn regions(&self, w: &World, each_country: bool) -> Vec<Vec<usize>> {
        if each_country {
            scope_countries(w, self.scope)
                .into_iter()
                .map(|c| scope_hexes(w, Some(c)))
                .collect()
        } else {
            vec![scope_hexes(w, self.scope)]
        }
    }

    /// `threat` from a threat, else `team`: the two wordings of a shared op.
    pub(super) fn says<T>(&self, threat: T, team: T) -> T {
        if self.source == EffectSource::Threat {
            threat
        } else {
            team
        }
    }
}

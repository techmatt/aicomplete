//! Who resolves an op, who it hits and where.

use super::*;

/// Who is resolving an op.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectSource {
    /// A player's card, ability or power: that seat chooses.
    Seat(usize),
    /// An executing threat: its harm hits one seat, `Ctx::hit` (146: the
    /// nearest by node); seat 0 chooses for the team.
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
    /// What is resolving, by name, for prompts (a threat's).
    pub why: Option<String>,
    /// The threat resolving (135): its tokens are "the location".
    pub threat: Option<Key>,
    /// 146: the one seat an executing threat's harm hits (the nearest by
    /// node; `None` with no node on the board).
    pub hit: Option<usize>,
}

impl Ctx {
    pub(super) fn new(source: EffectSource, scope: Option<usize>) -> Ctx {
        Ctx { source, scope, other: None, label: "card", card: None, why: None, threat: None, hit: None }
    }

    /// The threat whose effects these are (135).
    pub fn with_threat(mut self, t: Key) -> Ctx {
        self.threat = Some(t);
        self
    }

    /// 146: the seat the threat's harm hits.
    pub fn hitting(mut self, seat: Option<usize>) -> Ctx {
        self.hit = seat;
        self
    }

    /// Names what is resolving in its prompts (118: a threat's "discard which card?"
    /// named no threat).
    pub fn why(mut self, name: &str) -> Ctx {
        self.why = Some(name.to_string());
        self
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
            // 146: an executing threat hits one player, the nearest.
            EffectSource::Threat => match &self.threat {
                Some(_) => self.hit.into_iter().collect(),
                None => seats_in_scope(w, g, self.scope),
            },
            EffectSource::Team => seats_in_scope(w, g, self.scope),
        }
    }

    /// The hexes an effect on "pieces in scope" acts in (135: a threat's, the
    /// hexes on or next to its tokens; else its country, or the board).
    /// A threat op acting on hexes (Create, flags) reads these near hexes, while
    /// one acting on a seat reads the hit seat (`hitting`): two different targets.
    /// Fine while no threat prints a hex harm; revisit if one returns (146).
    pub(super) fn hexes(&self, w: &World, g: &GameState) -> Vec<usize> {
        match (&self.source, &self.threat) {
            (EffectSource::Threat, Some(t)) => threats::near_hexes(w, g, t),
            _ => scope_hexes(w, self.scope),
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

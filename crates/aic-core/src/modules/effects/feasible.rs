//! Whether an op list has something to act on.

use super::*;

/// Whether every targeted effect in `effects` has something to act on
/// (a card is not offered when it would do nothing).
pub fn feasible(w: &World, g: &GameState, seat: usize, effects: &[Op]) -> bool {
    feasible_after(w, g, seat, effects, 0)
}

/// The same once seat `s` has paid `spent` energy for them (066: a node
/// power pays its own energy, then the track step).
pub fn feasible_after(w: &World, g: &GameState, seat: usize, effects: &[Op], spent: i64) -> bool {
    feasible_on(w, g, seat, effects, spent, None)
}

/// [`feasible_after`] for the Action of seat `seat`'s building `card`: what
/// "here" counts (076).
pub fn feasible_on(w: &World, g: &GameState, seat: usize, effects: &[Op], spent: i64, card: Option<&str>) -> bool {
    let here_held = || card.map_or(0, |c| g.var(&cards::tokens_key(seat, c)));
    // 041: energy the same effects gain first can pay a placement's step.
    let gained: i64 = effects
        .iter()
        .take_while(|e| !matches!(e, Op::PlaceNode { .. }))
        .map(|e| match e {
            Op::Gain { resource: Resource::Energy, amount } => *amount,
            _ => 0,
        })
        .sum::<i64>()
        - spent;
    // 044: tokens the same effects add first can be cashed in or counted.
    let added = |kind: &TokenKind, upto: usize| -> i64 {
        let has = cards::has_token_card(w, g, seat, Some(*kind));
        effects[..upto]
            .iter()
            .map(|e| match e {
                Op::AddTokens { kind: k, amount, here: false } if k == kind && has => *amount,
                _ => 0,
            })
            .sum()
    };
    // 044: these only veto a card that would do nothing at all: a card that
    // also gains something is still worth playing ("you may move"). 076: a gain
    // per tag with too few tags on your cards is one of them (an event counts
    // none of its own; a program, which counts itself, carries other effects).
    // 099: a gift with no other player (one country: the energy gift Actions took
    // their energy and gave nothing) and a draw with the tier-1 deck empty
    // (Fieldwork; r44: a draw never reshuffles its discards) are of them too. 111 (Matt's ruling 2): a feeder's tokens on
    // one of your cards, and a cash-out from one, are a bonus beside the card's
    // main effect, so neither vetoes it.
    let only_these = effects.iter().all(|e| {
        matches!(
            e,
            Op::Harden { .. }
                | Op::MoveOne { .. }
                | Op::GainPerTag { .. }
                | Op::ChoosePlayer
                | Op::OtherGain { .. }
                | Op::Draw { .. }
                | Op::AddTokens { here: false, .. }
                | Op::CashOut { here: false, .. }
                // 118: "move up to N" (Summit Caucus, Liaison Corps) is a "you may move" too.
                | Op::Consolidate { .. }
                | Op::MoveAmbassadors { .. }
                // 118: a delay with no threat you may counter doesn't veto the draw or
                // shields beside it (Predictive Modeling, Early Warning Drill).
                | Op::DelayThreat { .. }
                | Op::Sway { .. }
        )
    });
    let one_acts = effects.iter().enumerate().any(|(at, e)| match e {
        Op::Harden { .. } => !harden_hexes(w, g, seat).is_empty(),
        Op::MoveOne { range } => !movable_one(w, g, seat, *range).is_empty(),
        Op::GainPerTag { tag, per, .. } => cards::tags(w, g, seat, *tag) >= *per,
        // 111: alone, "another player" means you, so a gift always lands.
        Op::OtherGain { .. } => true,
        Op::Draw { .. } => cards::can_draw(g),
        Op::AddTokens { kind, here: false, .. } => cards::has_token_card(w, g, seat, Some(*kind)),
        Op::CashOut { kind, here: false, .. } => cash_out_feasible(w, g, seat, kind, added(kind, at)),
        Op::Consolidate { .. } => {
            let mine: Vec<usize> = ambassador_hexes(g, seat).into_iter().filter(|&h| in_reach(w, g, seat, h)).collect();
            pieces_in_reach(w, g, seat).iter().any(|d| mine.iter().any(|a| a != d))
        }
        Op::MoveAmbassadors { .. } => movable_ambassadors(g, seat) > 0,
        Op::DelayThreat { .. } => !undelayed_threats(w, g, seat).is_empty(),
        Op::Sway { .. } => w.variant("alignment") != Some("off"),
        _ => false,
    });
    effects.iter().enumerate().all(|(at, e)| match e {
        Op::Damage { ambassadors, troops } => !damage_hexes(w, g, seat, *ambassadors, *troops).is_empty(),
        Op::DamageCountry { ambassadors, troops } => any_hex(g, |h| adversary::can_damage(w, g, h, *ambassadors, *troops)),
        Op::DestroyAll => any_hex(g, |h| adversary::can_destroy_all(w, g, h)),
        Op::RemoveBotnet => any_hex(g, |h| botnet_in_reach(w, g, seat, h)),
        Op::CutBotnet => !cuttable_botnets(w, g, seat).is_empty(),
        Op::Shelter => !unsheltered(w, g).is_empty(),
        Op::FreeBuilding => cards::free_building_exists(w, g, seat),
        Op::BuildStation => !board::station_sites(w, g, seat).is_empty(),
        Op::ProjectCube { station, .. } => !cube_projects(w, g, seat, *station).is_empty(),
        Op::AddTokens { here: true, .. } => card.is_some(),
        Op::AddTokens { here: false, .. } | Op::CashOut { here: false, .. } => !only_these || one_acts,
        Op::RemoveTroops { reach, .. } => {
            any_hex(g, |h| adversary::can_damage(w, g, h, 0, 1) && (!*reach || in_reach(w, g, seat, h)))
        }
        Op::Consolidate { .. } | Op::MoveAmbassadors { .. } => !only_these || one_acts,
        Op::RemoveAmbassadorsCountry { reach, .. } => {
            any_hex(g, |h| g.tokens(h, ADV_AMBASSADOR) > 0 && (!*reach || in_reach(w, g, seat, h)))
        }
        Op::BuildFactory => !factory_sites(w, g, seat).is_empty(),
        Op::Cleanse { cities, only, .. } => {
            any_hex(g, |h| cleansable(w, g, h, *cities, *only) && in_reach(w, g, seat, h))
        }
        Op::Liberate { .. } => !liberable_cities(w, g, seat).is_empty(),
        Op::ClearCity { .. } => !guarded_cities(w, g, seat).is_empty(),
        Op::Jam { source, .. } => !threats::counterable(w, g, seat, *source).is_empty(),
        Op::CounterThreat => !threats::counterable(w, g, seat, None).is_empty(),
        Op::DrainProject { .. } | Op::DrainAll { .. } => !adversary::projects_with_compute(g).is_empty(),
        Op::DiscardProject => !adversary::projects(g).is_empty(),
        // 024: too few to count, nothing to cash in: the card would do nothing.
        // 076: one card's tokens, this building's with "here" (an Action's own
        // token cost comes off first), else the best of yours.
        Op::GainPerTokens { here: true, per, .. } => here_held() >= *per,
        Op::CashOut { here: true, .. } => here_held() > 0,
        Op::GainPerTokens { kind, per, .. } => {
            let cards = cards::token_cards(w, g, seat, Some(*kind));
            cards.iter().map(|c| g.var(&cards::tokens_key(seat, c))).max().unwrap_or(0) + added(kind, at) >= *per
        }
        Op::Harden { .. } | Op::MoveOne { .. } | Op::GainPerTag { .. } | Op::OtherGain { .. } | Op::Draw { .. } => {
            !only_these || one_acts
        }
        // 105: a copy needs a building with an income, a reuse an Action used
        // this round, a discard a card in hand.
        Op::CopyIncome => !cards::income_buildings(w, g, seat).is_empty(),
        Op::Reuse => !cards::reusable(w, g, seat, card).is_empty(),
        Op::DiscardHand { .. } => !g.pile(&cards::hand(seat)).is_empty(),
        Op::PlaceNode { free, .. } => {
            board::can_pay_placement(w, g, seat, placing(*free), gained)
                && !board::node_hexes(w, g, seat).is_empty()
        }
        Op::ClearMarkers { .. } => any_hex(g, |h| marker_in_reach(w, g, seat, h)),
        Op::ShieldNode { .. } => any_hex(g, |h| shieldable(g, seat, h)),
        Op::DelayThreat { .. } => !only_these || one_acts,
        // Giving needs someone to give to (Vendor Financing and Development Loan still pay you).
        Op::Transfer { .. } => g.seats.len() > 1,
        // 118: with alignment off a Sway moves nothing: a card that only sways (or sways beside
        // another "you may" op with nothing to act on) does nothing; beside a real effect it plays.
        Op::Sway { .. } => !only_these || one_acts,
        _ => true,
    })
}

/// A cash-out from one of your cards has tokens to take: on a card, or added by
/// the same effects first (044).
fn cash_out_feasible(w: &World, g: &GameState, seat: usize, kind: &TokenKind, added: i64) -> bool {
    added > 0
        || cards::token_cards(w, g, seat, Some(*kind))
            .iter()
            .any(|c| g.var(&cards::tokens_key(seat, c)) > 0)
}

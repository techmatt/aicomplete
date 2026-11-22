//! Whether an op list has something to act on.

use super::*;

/// Whether any effect in `effects` has something to act on (r48: a card is
/// offered unless it would do nothing; a part with nothing to act on does
/// nothing and never blocks the rest).
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
    feasible_aimed(w, g, seat, effects, spent, card, None)
}

/// 234: `aim` is the hexes an enclosing list's target may take. A map op after a target acts when some such hex
/// lets it act (the condition names what the card acts on; a move or firewall checks its own hex test there);
/// the target itself is no part.
fn feasible_aimed(
    w: &World,
    g: &GameState,
    seat: usize,
    effects: &[Op],
    spent: i64,
    card: Option<&str>,
    aim: Option<&[usize]>,
) -> bool {
    let own: Option<Vec<usize>> = effects.iter().enumerate().find_map(|(i, e)| match e {
        Op::Target { on } => Some(target_hexes_before(w, g, seat, *on, &effects[i + 1..])),
        _ => None,
    });
    let aim: Option<&[usize]> = own.as_deref().or(aim);
    let aimed_acts = |e: &Op, at: &[usize]| -> bool {
        match narrowed(w, g, seat, e, at) {
            Some(hexes) => hexes.iter().any(|h| at.contains(h)),
            None => !at.is_empty(),
        }
    };
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
    // r48 (Matt, 2026-11-05; generalizing 044, 099, 111 and 118's moves,
    // delays and Sway): any part of a card with nothing to act on never blocks
    // the rest. The list is offered when at least one effect acts; one with
    // nothing to act on does nothing. A gain per tag with too few tags, a draw
    // with no card to take, a removal with no target in reach: each is a
    // part. A list where nothing acts would be a play for nothing, so it isn't
    // offered.
    let acts = |at: usize, e: &Op| -> bool {
        if let Some(at) = aim.filter(|_| e.is_map()) {
            return aimed_acts(e, at);
        }
        match e {
            Op::Target { .. } => false,
            // 253: a second Shelter the same round adds nothing (paid_for_nothing found a replay of Aegis Grid Lockdown).
            Op::Shelter => node_count(g, seat) > 0 && g.var(&board::reach_shelter(seat)) == 0,
            Op::PlaceAmbassadors { .. } => false,
            Op::Harden { .. } => !harden_hexes(w, g, seat).is_empty(),
            Op::MoveOne { range } => !movable_one(w, g, seat, *range).is_empty(),
            Op::GainPerTag { tag, per, .. } => cards::tags(w, g, seat, *tag) >= *per,
            // A choice of player is the gift beside it, not an effect of its own.
            Op::ChoosePlayer => false,
            // 111: alone, "another player" means you, so a gift always lands.
            Op::OtherGain { .. } => true,
            Op::Draw { .. } => cards::can_draw(g),
            Op::AddTokens { here: true, .. } => card.is_some(),
            Op::AddTokens { kind, here: false, .. } => cards::has_token_card(w, g, seat, Some(*kind)),
            Op::CashOut { kind, here: false, .. } => cash_out_feasible(w, g, seat, kind, added(kind, at)),
            Op::Consolidate { .. } => {
                let mine: Vec<usize> = ambassador_hexes(g, seat).into_iter().filter(|&h| in_reach(w, g, seat, h)).collect();
                pieces_in_reach(w, g, seat).iter().any(|d| mine.iter().any(|a| a != d))
            }
            Op::MoveAmbassadors { .. } => movable_ambassadors(g, seat) > 0,
            Op::Convert { .. } => !convert_cities(w, g, seat).is_empty(),
            Op::Gather { pieces, strike, .. } => !gather_hexes(w, g, seat, pieces, *strike).is_empty(),
            Op::Push { pieces, .. } => !push_hexes(w, g, seat, pieces).is_empty(),
            Op::Firewall => !firewall_hexes(w, g, seat).is_empty(),
            // The threshold is a rider: it acts only with the tags, and then as its own ops do.
            Op::IfTags { tag, count, then } => {
                cards::tags(w, g, seat, *tag) >= *count && feasible_aimed(w, g, seat, then, 0, None, aim)
            }
            // 212: a rider on the node placed before it: never a part that acts on its own.
            Op::IfNodeNext { .. } | Op::AmbassadorToSpot => false,
            Op::LiftFallout { .. } => !board::land_in_reach(w, g, seat).is_empty(),
            Op::DelayThreat { .. } => !undelayed_threats(w, g, seat).is_empty(),
            // 184: a Discord sway is the card's price (The Nuclear Option's until 302), never a part that acts.
            Op::Sway { side, .. } => *side == Side::Unity && !w.hot.alignment_off,
            Op::Damage { ambassadors, troops } => !damage_hexes(w, g, seat, *ambassadors, *troops).is_empty(),
            Op::DamageCountry { ambassadors, troops } => any_hex(g, |h| adversary::can_damage(g, h, *ambassadors, *troops)),
            Op::DestroyAll => any_hex(g, |h| adversary::can_destroy_all(g, h)),
            Op::RemoveBotnet => any_hex(g, |h| botnet_in_reach(w, g, seat, h)),
            Op::CutBotnet => !cuttable_botnets(w, g, seat).is_empty(),
            Op::FreeBuilding => cards::free_building_exists(w, g, seat),
            Op::BuildStation => !board::station_sites(w, g, seat).is_empty(),
            Op::ClaimSite { .. } => !board::card_claim_sites(w, g, seat).is_empty(),
            Op::ProjectCube { station, .. } => !cube_projects(w, g, seat, *station).is_empty(),
            Op::RemoveTroops { reach, .. } => {
                any_hex(g, |h| adversary::can_damage(g, h, 0, 1) && (!*reach || in_reach(w, g, seat, h)))
            }
            Op::RemoveAmbassadorsCountry { .. } => any_hex(g, |h| g.tokens(h, ADV_AMBASSADOR) > 0 && in_reach(w, g, seat, h)),
            Op::BuildFactory => !factory_sites(w, g, seat).is_empty(),
            Op::Cleanse { cities, only, .. } => {
                any_hex(g, |h| cleansable(w, g, h, *cities, *only) && in_reach(w, g, seat, h))
            }
            // 254: an `own` liberate takes a corrupted station or factory too.
            Op::Liberate { own: true, .. } => !liberable_sites(w, g, seat).is_empty(),
            Op::Liberate { .. } => !liberable_cities(w, g, seat).is_empty(),
            Op::ClearCity { .. } => !guarded_cities(w, g, seat).is_empty(),
            Op::Jam { source, .. } => !threats::counterable(w, g, seat, *source).is_empty(),
            Op::CounterThreat => !threats::counterable(w, g, seat, None).is_empty(),
            Op::DrainProject { .. } | Op::DrainAll { .. } => !adversary::projects_with_compute(g).is_empty(),
            Op::DiscardProject => !adversary::projects(g).is_empty(),
            // 259: an empty research track: nothing to remove.
            Op::RemoveResearch { .. } => g.var(adversary::RESEARCH) > 0,
            // 024: too few to count, nothing to cash in: it would do nothing.
            // 076: one card's tokens, this building's with "here" (an Action's own
            // token cost comes off first), else the best of yours.
            Op::GainPerTokens { here: true, per, .. } => here_held() >= *per,
            Op::CashOut { here: true, .. } => here_held() > 0,
            Op::GainPerTokens { kind, per, .. } => {
                let cards = cards::token_cards(w, g, seat, Some(*kind));
                cards.iter().map(|c| g.var(&cards::tokens_key(seat, c))).max().unwrap_or(0) + added(kind, at) >= *per
            }
            // 105: a copy needs a building with an income, a reuse an Action used
            // this round, a discard a card in hand.
            Op::CopyIncome => !cards::income_buildings(w, g, seat).is_empty(),
            Op::Reuse => !cards::reusable(w, g, seat, card).is_empty(),
            Op::DiscardHand { .. } => !g.pile(&cards::hand(seat)).is_empty(),
            Op::PlaceNode { range, .. } => {
                board::can_pay_placement(w, g, seat, gained)
                    && !board::node_sites_within(w, g, seat, *range).is_empty()
            }
            Op::ShieldNode { .. } => any_hex(g, |h| shieldable(g, seat, h)),
            // Giving needs someone to give to (Vendor Financing and Development Loan still pay you).
            Op::Transfer { .. } => g.seats.len() > 1,
            // The rest always act: a gain, a standing passive, or a team or adversary op no player list prints.
            // Listed, not `_`, so a new op must decide here (259: a `_` let an op that can do nothing through).
            Op::Gain { .. } | Op::GainPerNode { .. } | Op::GainAmbassadors { .. } | Op::Cordon
            | Op::Peek { .. } | Op::Production { .. } | Op::ProductionPerTag { .. } | Op::SpreadAmbassadors
            | Op::Troops { .. } | Op::ThreatAtNode { .. } | Op::EachDestroyNode { .. } | Op::DestroyNode { .. }
            | Op::PullUnity { .. } | Op::PullDiscord { .. } | Op::FlipStation { .. } | Op::CorruptCity { .. } | Op::PlayersLose { .. }
            | Op::AdversaryBuild | Op::LoseNodeOr { .. } | Op::Flag { .. } | Op::Requisition { .. }
            | Op::LoseAmbassador | Op::DiscardMostExpensive | Op::Compute { .. } | Op::ComputeEvery { .. }
            | Op::LoseTokens { .. } | Op::AdversaryFactory { .. } | Op::ComputePerMissing { .. }
            | Op::DestroyFacility { .. } | Op::DisableBuilding { .. } | Op::Discard { .. }
            | Op::LoseAmbassadorsOr { .. } | Op::LoseIncome { .. } | Op::LoseOr { .. }
            | Op::Strike | Op::Redeploy | Op::OccupyFactory { .. } | Op::PlaceBotnet { .. } | Op::CorruptFactory
            | Op::Theft { .. } | Op::TeamIncome { .. } | Op::FreePurchases { .. }
            | Op::DealThreats { .. } | Op::Create { .. } | Op::CreateSite { .. }
            | Op::Infiltrate { .. } | Op::Escalate { .. }
            | Op::IncreaseIncome { .. } | Op::AdversaryResearch { .. }
            | Op::BotnetTroops { .. }
            | Op::SpreadBotnets { .. } | Op::TakeSite { .. } | Op::FactoryTroops { .. } | Op::AgentsOnNodes { .. }
            | Op::RecallAmbassadors | Op::StripCubes { .. } | Op::StripHarden | Op::Fallout { .. }
            | Op::Clock { .. } | Op::PerSite { .. } | Op::RemoveWhere { .. }
            | Op::Prohibit { .. } | Op::Floor { .. } | Op::StandingTroopsBonus { .. } | Op::ThreatCompute { .. } | Op::Income { .. }
            | Op::IncomePerTag { .. } | Op::BuildingDiscount { .. } | Op::CardDiscount { .. } | Op::When { .. }
            | Op::Threshold { .. } | Op::Slots { .. } | Op::Logistics { .. } | Op::ProjectDrain { .. }
            | Op::AutoCounter { .. } | Op::UnityBonus { .. } | Op::CorruptionShield | Op::CivicShelter | Op::Wall
            | Op::Patience | Op::SupplyInsight { .. } | Op::TagsFromHand { .. } | Op::ConvertUpkeep { .. }
            => true,
        }
    };
    effects.is_empty() || effects.iter().enumerate().any(|(at, e)| acts(at, e))
}

/// A cash-out from one of your cards has tokens to take: on a card, or added by
/// the same effects first (044).
fn cash_out_feasible(w: &World, g: &GameState, seat: usize, kind: &TokenKind, added: i64) -> bool {
    added > 0
        || cards::token_cards(w, g, seat, Some(*kind))
            .iter()
            .any(|c| g.var(&cards::tokens_key(seat, c)) > 0)
}

/// `d`'s effects as its play check judges them, the engine's and the agents'
/// (121, Matt: as in Terraforming Mars). A card's own tags never count toward
/// its own `needs`: `requirements_met` reads the cards in play, and the card is
/// still in hand. Its effects are another matter: a program enters play before
/// it resolves, so its own tags count toward its gain per tag (118: Consular
/// Pouch was refused with the one Diplomacy tag it brings); an event's never do.
/// Borrowed unless it changes something: the play check runs for every card in
/// hand on every legal-action pass.
pub fn own_counted(d: &crate::content::CardDef) -> std::borrow::Cow<'_, [Op]> {
    let own = |tag: crate::content::Tag| if d.is_event() { 0 } else { d.tags.iter().filter(|t| **t == tag).count() as i64 };
    if !d.effects.iter().any(|e| matches!(e, Op::GainPerTag { tag, .. } if own(*tag) > 0)) {
        return std::borrow::Cow::Borrowed(&d.effects);
    }
    std::borrow::Cow::Owned(
        d.effects
            .iter()
            .map(|e| match e {
                Op::GainPerTag { resource, tag, per } => Op::GainPerTag { resource: *resource, tag: *tag, per: (*per - own(*tag)).max(0) },
                other => other.clone(),
            })
            .collect(),
    )
}

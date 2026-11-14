//! What every [`Op`] does: the one place an instant op's meaning is
//! dispatched, whichever card, threat or board prints it. [`Ctx`] says who
//! chooses, who is hit and where (see `op.rs`). Rules that belong to a module
//! stay there; an arm here calls them. Standing ops and the twist-only ops are
//! read by the rule they change and never resolve.

use crate::content::{Facility, Op, RoundFlag, Side};
use crate::op::{NodeSpot, Spot};
use crate::key::Key;
use crate::module::{Cx, PromptText, Res, World};
use crate::state::{GameState, Opt};
use crate::vocab::TokenKind;

use super::common::*;
use super::{adversary, alignment, board, cards, economy, threats};

mod ambassadors;
mod ctx;
mod endgame;
mod feasible;
mod moves;
mod new_ops;
mod removal;
mod resources;
mod science;
mod targets;
mod team;

pub use ctx::*;
pub use endgame::{convert_cities, firewall_hexes};
pub use moves::{gather_hexes, push_hexes, GATHER_INTO, GATHER_PIECE, PUSH_FROM, PUSH_PIECE, STOP, STRIKE_AT, STRIKE_PIECE};
pub use feasible::*;
pub use targets::*;

/// Resolves `ops` in order.
pub fn run(g: &mut GameState, cx: &mut Cx, ops: &[Op], ctx: &mut Ctx) -> Res {
    for e in ops {
        one(g, cx, e, ctx)?;
        if g.is_over() {
            break;
        }
    }
    Ok(())
}

/// Asks `seat` to pick among `opts`, "stop" added unless this is the `first`
/// pick of an "up to N". `None`: no pick, or stop.
fn pick_or_stop(cx: &mut Cx, seat: usize, prompt: impl PromptText, mut opts: Vec<Opt>, first: bool) -> Res<Option<usize>> {
    let n = opts.len();
    if !first {
        opts.push(opt("stop"));
    }
    Ok(cx.choose(seat, prompt, opts)?.filter(|&i| i < n))
}

fn one(g: &mut GameState, cx: &mut Cx, e: &Op, ctx: &mut Ctx) -> Res {
    let w = cx.world;
    let seat = ctx.chooser();
    match e {
        Op::Gain { resource, amount } => {
            for s in ctx.seats(&w, g) {
                cx.add_resource(g, s, *resource, *amount);
            }
        }
        Op::GainPerNode { resource } => {
            let n = node_count(g, seat);
            cx.add_resource(g, seat, *resource, n);
        }
        Op::ChoosePlayer => {
            // 111 (Matt's ruling 7): in a 1-player game, "another player" means you.
            let others: Vec<usize> = (0..g.seats.len()).filter(|&s| s != seat || g.seats.len() == 1).collect();
            let opts = others.iter().map(|&s| seat_opt(s)).collect();
            let prompt = || format!("Seat {seat}: which other player?");
            ctx.other = cx.choose(seat, prompt, opts)?.map(|i| others[i]);
        }
        Op::OtherGain { resource, amount } => {
            if let Some(o) = ctx.other {
                cx.add_resource(g, o, *resource, *amount);
            }
        }
        Op::Transfer { resource, max } => resources::transfer(g, cx, ctx, seat, *resource, *max)?,
        Op::PlaceNode { count } => {
            for _ in 0..*count {
                let Some(h) = board::place_node(g, cx, seat)? else { break };
                ctx.node = Some(h);
            }
        }
        // 212: the node power's rider: on or next to its spot, else nothing.
        Op::IfNodeNext { spots, then, fresh } => {
            if ctx.node.is_some_and(|h| !board::spots_near(&w, g, seat, h, spots, *fresh).is_empty()) {
                cx.tally(g, "effects.if_node_next_met", 1);
                ctx.fresh = *fresh;
                run(g, cx, then, ctx)?;
            }
        }
        Op::AmbassadorToSpot => {
            let near = |h| board::spots_near(&w, g, seat, h, &[NodeSpot::City], ctx.fresh);
            let cities = ctx.node.map(near).unwrap_or_default();
            let opts = cities.iter().map(|&h| hex_opt(&w, h)).collect();
            let prompt = || format!("Seat {seat}: a new ambassador onto which city?");
            if let Some(i) = cx.choose(seat, prompt, opts)? {
                cx.tally(g, "effects.ambassador_to_spot", 1);
                board::place_ambassador(g, cx, seat, cities[i]);
            }
        }
        Op::LiftFallout { count } => {
            for _ in 0..*count {
                let hexes = board::land_in_reach(&w, g, seat);
                let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
                let prompt = || format!("Seat {seat}: lift Fallout on which hex?");
                let Some(i) = cx.choose(seat, prompt, opts)? else { break };
                adversary::lift_fallout(g, cx, hexes[i]);
                cx.tally(g, "effects.land_lifted", 1);
            }
        }
        // 044: "up to": after the first, "stop" (as Cleanse does).
        Op::ShieldNode { count } => science::shield_node(g, cx, seat, *count)?,
        Op::DelayThreat { count } => science::delay_threat(g, cx, seat, *count)?,
        Op::Damage { ambassadors, troops } => removal::damage(g, cx, seat, *ambassadors, *troops)?,
        Op::DamageCountry { ambassadors, troops } => removal::damage_country(g, cx, seat, *ambassadors, *troops)?,
        Op::DestroyAll => removal::destroy_all(g, cx, seat)?,
        Op::RemoveBotnet => removal::remove_botnet(g, cx, seat)?,
        Op::RemoveAmbassadorsCountry { unity_per, max, reach } => {
            removal::remove_ambassadors_country(g, cx, seat, *unity_per, *max, *reach)?
        }
        Op::BuildFactory => resources::build_factory(g, cx, seat)?,
        Op::Burn { facility, blight } => resources::burn(g, cx, seat, *facility, *blight)?,
        Op::Cleanse { amount, one_country, cities, only } => {
            removal::cleanse(g, cx, seat, *amount, *one_country, *cities, *only)?
        }
        Op::Liberate { ambassadors, troops, botnet } => {
            removal::liberate_city(g, cx, seat, *ambassadors, *troops, *botnet)?
        }
        Op::ClearCity { max } => removal::clear_city(g, cx, seat, *max)?,
        Op::Harden { count } => resources::harden(g, cx, seat, *count)?,
        Op::MoveAmbassadors { max } => ambassadors::move_ambassadors(g, cx, seat, *max)?,
        Op::GainAmbassadors { count } => cx.add_resource(g, seat, Resource::Ambassadors, *count),
        Op::Shelter => {
            let cs = unsheltered(&w, g);
            let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
            let prompt = || format!("Seat {seat}: shelter which country?");
            if let Some(i) = cx.choose(seat, prompt, opts)? {
                board::shelter(g, cx, cs[i]);
            }
        }
        Op::Cordon => {
            // 118: a country cordoned only this round (the flag at 1) may be cordoned for the next.
            let cs: Vec<usize> = (0..w.map.countries.len()).filter(|&c| g.var(&board::flag(RoundFlag::Cordon, c)) < 2).collect();
            let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
            let prompt = || format!("Seat {seat}: cordon which country?");
            if let Some(i) = cx.choose(seat, prompt, opts)? {
                // 178 (Matt's ruling #30): until the end of the next adversary turn (the compute step clears
                // it); cleanup counts it down, so the forecast reads 2 as "played this round".
                cx.set_key(g, &board::flag(RoundFlag::Cordon, cs[i]), 2);
            }
        }
        Op::FreeBuilding => cards::play_free_building(g, cx, seat)?,
        Op::BuildStation => resources::build_station(g, cx, seat)?,
        Op::ClaimStation => resources::claim_station(g, cx, seat)?,
        Op::ProjectCube { count, station } => resources::project_cube(g, cx, seat, *count, *station)?,
        Op::Peek { count } => adversary::peek(g, cx, *count),
        Op::Jam { amount, source } => {
            let active = threats::counterable(&w, g, seat, *source);
            let opts = active.iter().map(|t| opt(*t)).collect();
            let prompt = || format!("Seat {seat}: jam which threat?");
            if let Some(i) = cx.choose(seat, prompt, opts)? {
                threats::jam(g, cx, &active[i], *amount);
                if source.is_some() {
                    cx.tally(g, "threats.typed_counters", 1);
                }
            }
        }
        Op::CounterThreat => {
            // 070: a player's card reaches what they may counter; a reward any threat.
            let active: Vec<_> = match ctx.source {
                EffectSource::Seat(_) => threats::counterable(&w, g, seat, None),
                _ => threats::active(g).into_iter().map(|(t, _)| t).collect(),
            };
            let opts = active.iter().map(|t| opt(*t)).collect();
            let prompt = match ctx.source {
                EffectSource::Seat(_) => format!("Seat {seat}: counter which threat?"),
                _ => "Unity: counter which threat?".to_string(),
            };
            if let Some(i) = cx.choose(seat, &prompt, opts)? {
                threats::counter(g, cx, &active[i]);
            }
        }
        Op::DrainProject { amount } => adversary::choose_drain(g, cx, seat, *amount)?,
        Op::DiscardProject => {
            let ids = adversary::projects(g);
            let opts = ids.iter().map(|p| opt(*p)).collect();
            let prompt = || format!("Seat {seat}: discard which adversary project?");
            if let Some(i) = cx.choose(seat, prompt, opts)? {
                adversary::discard_project(g, cx, &ids[i]);
            }
        }
        Op::DrainAll { amount } => {
            for p in adversary::projects(g) {
                adversary::drain_project(g, cx, &p, *amount);
            }
        }
        Op::Draw { count } => cards::draw(g, cx, seat, *count)?,
        Op::GainPerTokens { resource, kind, per, here } => {
            if let Some(c) = resources::token_card(g, cx, ctx, seat, *kind, *here, *per)? {
                let held = g.var(&cards::tokens_key(seat, &c));
                cx.add_resource(g, seat, *resource, held / (*per).max(1));
            }
        }
        Op::CashOut { kind, resource, rate, here } => {
            if let Some(c) = resources::token_card(g, cx, ctx, seat, *kind, *here, 1)? {
                let k = g.var(&cards::tokens_key(seat, &c));
                cards::add_tokens(g, cx, seat, &c, -k);
                cx.add_resource(g, seat, *resource, k * rate);
            }
        }
        Op::GainPerTag { resource, tag, per } => {
            let n = cards::tags(&w, g, seat, *tag) / (*per).max(1);
            cx.add_resource(g, seat, *resource, n);
        }
        Op::RemoveTroops { count, reach } => removal::remove_troops(g, cx, seat, *count, *reach)?,
        Op::Sway { side: Side::Discord, amount, .. } => alignment::discord_from(g, cx, ctx.label, *amount),
        Op::Sway { side: _, amount, .. } => alignment::unity_from(g, cx, ctx.label, *amount),
        Op::Production { resource, amount } => economy::raise_production(g, cx, seat, *resource, *amount),
        // 106: counted once, now; a program is in play when it resolves.
        Op::ProductionPerTag { resource, tag, per } => {
            let n = cards::tags(&w, g, seat, *tag) / (*per).max(1);
            economy::raise_production(g, cx, seat, *resource, n);
        }
        Op::AddTokens { kind, amount, here: true } => {
            if let Some(c) = ctx.card {
                cards::add_tokens(g, cx, seat, &c, *amount);
                // 105: a token placed fires the seat's place-token triggers.
                if cards::def(&w, &c).tokens.as_ref().is_some_and(|t| t.kind == *kind) {
                    cards::on_tokens(g, cx, seat, *kind, *amount)?;
                }
                // 118: At N pays when its tokens reach N, whatever put them there.
                cards::threshold(g, cx, seat, c)?;
            }
        }
        Op::AddTokens { kind, amount, here: false } => {
            let ids = cards::token_cards(&w, g, seat, Some(*kind));
            let opts = ids.iter().map(|c| opt(*c)).collect();
            let prompt = || format!("Seat {seat}: add {amount} {kind} to which card?");
            if let Some(i) = cx.choose(seat, prompt, opts)? {
                cards::add_tokens(g, cx, seat, &ids[i], *amount);
                cards::on_tokens(g, cx, seat, *kind, *amount)?;
                cards::threshold(g, cx, seat, ids[i])?;
            }
        }
        Op::CopyIncome => cards::copy_income(g, cx, seat)?,
        Op::Reuse => cards::reuse(g, cx, seat, ctx.card)?,
        Op::DiscardHand { resource, amount } => cards::discard_hand(g, cx, seat, *resource, *amount)?,
        Op::MoveOne { range } => ambassadors::move_one(g, cx, seat, *range)?,
        Op::SpreadAmbassadors => ambassadors::spread_ambassadors(g, cx, seat)?,
        Op::Consolidate { max } => ambassadors::consolidate(g, cx, seat, *max)?,
        Op::Convert { max } => endgame::convert(g, cx, seat, *max)?,
        Op::Gather { max, pieces, strike } => moves::gather(g, cx, seat, *max, pieces, *strike)?,
        Op::Push { max, pieces } => moves::push(g, cx, seat, *max, pieces)?,
        Op::Firewall => endgame::firewall(g, cx, seat)?,
        Op::IfTags { tag, count, then } => {
            if cards::tags(&w, g, seat, *tag) >= *count {
                cx.tally(g, "effects.if_tags_met", 1);
                run(g, cx, then, ctx)?;
            }
        }
        Op::CutBotnet => removal::cut_botnet(g, cx, seat)?,

        // Threats and the team: the wordings differ, the rule doesn't.
        Op::Troops { count, each_country, destroy, at: None, stand: false, location: None, if_: None } => {
            team::troops(g, cx, ctx, *count, *each_country, *destroy)?
        }
        Op::FlipStation { each_country, force: false } => team::flip_station(g, cx, ctx, *each_country)?,
        Op::PlayersLose { resource, amount, all: false } => {
            for s in ctx.seats(&w, g) {
                let k = (*amount).min(g.resource(s, *resource));
                cx.add_resource(g, s, *resource, -k);
            }
        }
        Op::AdversaryBuild => adversary::build_all(g, cx, ctx.scope)?,
        Op::ThreatAtNode { tier } => team::threat_at_node(g, cx, ctx, *tier)?,
        Op::EachDestroyNode { count, bases } => team::each_destroy_node(g, cx, ctx, *count, *bases)?,
        Op::DestroyNode { count } => team::destroy_nearest_node(g, cx, ctx, *count)?,
        Op::PullUnity { amount } => alignment::pull_unity(g, cx, *amount),

        // Threats.
        Op::Flag { name, scope: None, rounds: 1, everywhere: false } => {
            // Threats execute after production and upkeep: the flag lasts through
            // the next round (cleanup counts it down), or it would never land (020).
            for c in scope_countries(&w, ctx.scope) {
                cx.set_key(g, &board::flag(*name, c), 2);
            }
        }
        Op::Requisition { resource, amount, cards: n, next_round: false } => {
            threats::requisition(g, cx, &ctx.seats(&w, g), *resource, *amount, *n, ctx.why.as_deref())?
        }
        Op::LoseAmbassador => threats::lose_ambassador(g, cx, &ctx.seats(&w, g), ctx.why.as_deref())?,
        Op::LoseNodeOr { resource, amount, count: 1 } => {
            threats::lose_node_or(g, cx, &ctx.seats(&w, g), &ctx.hexes(&w, g), *resource, *amount, ctx.label, ctx.why.as_deref())?
        }
        Op::DiscardMostExpensive => threats::discard_most_expensive(g, cx, &ctx.seats(&w, g), ctx.why.as_deref())?,
        Op::Compute { amount } => {
            if ctx.label == "megaproject" {
                cx.tally(g, "adversary.compute_corp", *amount);
            }
            adversary::gain_compute(g, cx, *amount)?
        }
        Op::ComputeEvery { amount } => adversary::compute_every(g, cx, *amount)?,
        Op::LoseTokens { amount } => threats::lose_tokens(g, cx, &ctx.seats(&w, g), *amount, ctx.why.as_deref())?,

        // The team.
        Op::TeamIncome { resource, amount } => alignment::add_team_income(g, cx, *resource, *amount),
        Op::FreePurchases { count } => alignment::add_free_purchases(g, cx, *count),
        Op::CounterAll => {
            for (t, _) in threats::active(g) {
                threats::counter(g, cx, &t);
            }
        }
        Op::Pushback { ambassadors } => {
            for c in 0..w.map.countries.len() {
                for _ in 0..*ambassadors {
                    adversary::pushback(g, cx, c)?;
                }
            }
        }
        Op::DealThreats { count, tier: None, scope: None, each_country: false } => threats::deal_extra(g, cx, *count),
        Op::RemoveBotnets { count } => team::remove_botnets(g, cx, *count)?,
        Op::AdversaryCubes { count } => team::adversary_cubes(g, cx, *count),
        Op::BotnetCubes { count } => team::botnet_cubes(g, cx, *count)?,
        // 195: the project deck's board verbs.
        Op::BotnetTroops { count, troops } => team::botnet_troops(g, cx, *count, *troops),
        Op::SpreadBotnets { count } => team::spread_botnets(g, cx, *count),
        Op::TakeSite { site } => team::take_site(g, cx, *site)?,
        Op::FactoryTroops { count } => team::factory_troops(g, cx, *count),
        Op::RaidNode => team::raid_node(g, cx, ctx)?,
        Op::StripHarden => team::strip_harden(g, cx),
        Op::Clock { count } => team::clock(g, cx, *count),
        Op::OnePlayer { then } => {
            // 156: the team picks who acts, among the players it would act for.
            let able: Vec<usize> = (0..g.seats.len()).filter(|&s| feasible(&w, g, s, then)).collect();
            let who = match able.len() {
                0 => None,
                1 => Some(able[0]),
                _ => {
                    let opts = able.iter().map(|&s| seat_opt(s)).collect();
                    let why = ctx.why.clone().unwrap_or_default();
                    let prompt = || format!("{why}: which player acts?");
                    cx.choose(seat, prompt, opts)?.map(|i| able[i])
                }
            };
            if let Some(s) = who {
                let mut inner = Ctx::seat(s).named(ctx.label);
                inner.why = ctx.why.clone();
                run(g, cx, then, &mut inner)?;
            }
        }
        // 167: fear-card shape: board conditions, never reach.
        // 169: extra threats for each site are one batch, so "one per country in turn" runs across them all (each
        // site's own deal started again at country 0).
        Op::PerSite { where_, max, then } if matches!(then.as_slice(), [Op::DealThreats { tier: None, scope: None, each_country: false, .. }]) => {
            let [Op::DealThreats { count, .. }] = then.as_slice() else { unreachable!() };
            threats::deal_extra(g, cx, count * team::per_site_count(&w, g, *where_, *max));
        }
        Op::PerSite { where_, max, then } => {
            for _ in 0..team::per_site_count(&w, g, *where_, *max) {
                run(g, cx, then, ctx)?;
                if g.is_over() {
                    break;
                }
            }
        }
        Op::RemoveWhere { pieces, count, per, where_ } => team::remove_where(g, cx, *pieces, *count, *per, *where_)?,
        Op::Prohibit { step, where_ } => adversary::prohibit(g, cx, *step, *where_),
        // 151: the adversary's rally and income.
        Op::Rally { site, count } => {
            let source = if ctx.label == "standing" { "event" } else { ctx.label };
            adversary::rally(g, cx, *site, *count, ctx.scope, source)?
        }
        Op::IncreaseIncome { compute, ambassadors } => adversary::increase_income(g, cx, *compute, *ambassadors),

        // 135: the new adversary model's ops, and the old ops with a new field set.
        Op::Troops { .. }
        | Op::FlipStation { .. }
        | Op::CorruptCity { .. }
        | Op::PlayersLose { .. }
        | Op::Create { .. }
        | Op::CreateSite { .. }
        | Op::Corrupt { .. }
        | Op::Flag { .. }
        | Op::Requisition { .. }
        | Op::LoseNodeOr { .. }
        | Op::DealThreats { .. }
        | Op::AdversaryFactory { .. }
        | Op::ComputePerMissing { .. }
        | Op::CorpTroops { .. }
        | Op::DestroyFacility { .. }
        | Op::DisableBuilding { .. }
        | Op::Discard { .. }
        | Op::ExtraWar { .. }
        | Op::LoseAmbassadorsOr { .. }
        | Op::LoseIncome { .. }
        | Op::LoseOr { .. }
        | Op::MarchNow { .. }
        | Op::OccupyFactory { .. }
        | Op::PlaceBotnet { .. }
        | Op::CorruptFactory
        | Op::Fallout { .. }
        | Op::Theft { .. } => new_ops::one(g, cx, e, ctx)?,

        // Read, not resolved; `Content::check` keeps them out of resolving lists.
        Op::Income { .. }
        | Op::IncomePerTag { .. }
        | Op::BuildingDiscount { .. }
        | Op::CardDiscount { .. }
        | Op::When { .. }
        | Op::Threshold { .. }
        | Op::Slots { .. }
        | Op::Logistics { .. }
        | Op::ProjectDrain { .. }
        | Op::AutoCounter { .. }
        | Op::UnityBonus { .. }
        | Op::CorruptionShield
        | Op::CivicShelter
        | Op::Wall
        | Op::Patience
        | Op::SupplyInsight { .. }
        | Op::TagsFromHand { .. }
        | Op::ConvertUpkeep { .. }
        | Op::StandingTroopsBonus { .. }
        | Op::ThreatCompute { .. }
        | Op::TroopSteps { .. } => unreachable!("{e:?} is read, not resolved"),
    }
    Ok(())
}

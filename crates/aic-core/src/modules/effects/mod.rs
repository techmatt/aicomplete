//! What every [`Op`] does: the one place an instant op's meaning is
//! dispatched, whichever card, threat or board prints it. [`Ctx`] says who
//! chooses, who is hit and where (see `op.rs`). Rules that belong to a module
//! stay there; an arm here calls them. Standing ops and the twist-only ops are
//! read by the rule they change and never resolve.

use crate::content::{Facility, Op, RoundFlag, Side};
use crate::key::Key;
use crate::module::{Cx, PromptText, Res, World};
use crate::state::{GameState, Opt};
use crate::vocab::TokenKind;

use super::common::*;
use super::{adversary, alignment, board, cards, economy, threats};

mod ambassadors;
mod ctx;
mod feasible;
mod removal;
mod resources;
mod science;
mod targets;
mod team;

pub use ctx::*;
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
        Op::PlaceNode { count, free } => {
            for _ in 0..*count {
                if !board::place_node(g, cx, seat, placing(*free))? {
                    break;
                }
            }
        }
        // 044: "up to": after the first, "stop" (as Cleanse does).
        Op::ClearMarkers { count } => science::clear_markers(g, cx, seat, *count)?,
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
        Op::Cleanse { amount, one_country, cities, only } => {
            removal::cleanse(g, cx, seat, *amount, *one_country, *cities, *only)?
        }
        Op::Liberate { align, ambassadors, troops, botnet } => {
            removal::liberate_city(g, cx, seat, *align, *ambassadors, *troops, *botnet)?
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
            let cs: Vec<usize> = (0..w.map.countries.len()).filter(|&c| !board::flagged(g, RoundFlag::Cordon, c)).collect();
            let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
            let prompt = || format!("Seat {seat}: cordon which country?");
            if let Some(i) = cx.choose(seat, prompt, opts)? {
                // Up through the next round's build: cleanup counts it down.
                cx.set_key(g, &board::flag(RoundFlag::Cordon, cs[i]), 2);
            }
        }
        Op::FreeBuilding => cards::play_free_building(g, cx, seat)?,
        Op::BuildStation => resources::build_station(g, cx, seat)?,
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
        Op::Sway { side: Side::Discord, amount } => alignment::discord_from(g, cx, ctx.label, *amount),
        Op::Sway { side, amount } => alignment::sway(g, cx, *side, *amount),
        Op::Production { resource, amount } => {
            cx.add_key(g, &economy::production_key(seat, *resource), *amount)
        }
        // 106: counted once, now; a program is in play when it resolves.
        Op::ProductionPerTag { resource, tag, per } => {
            let n = cards::tags(&w, g, seat, *tag) / (*per).max(1);
            if n != 0 {
                cx.add_key(g, &economy::production_key(seat, *resource), n)
            }
        }
        Op::AddTokens { kind, amount, here: true } => {
            if let Some(c) = ctx.card {
                cards::add_tokens(g, cx, seat, &c, *amount);
                // 105: a token placed fires the seat's place-token triggers.
                if cards::def(&w, &c).tokens.as_ref().is_some_and(|t| t.kind == *kind) {
                    cards::on_tokens(g, cx, seat, *kind, *amount)?;
                }
            }
        }
        Op::AddTokens { kind, amount, here: false } => {
            let ids = cards::token_cards(&w, g, seat, Some(*kind));
            let opts = ids.iter().map(|c| opt(*c)).collect();
            let prompt = || format!("Seat {seat}: add {amount} {kind} to which card?");
            if let Some(i) = cx.choose(seat, prompt, opts)? {
                cards::add_tokens(g, cx, seat, &ids[i], *amount);
                cards::on_tokens(g, cx, seat, *kind, *amount)?;
            }
        }
        Op::CopyIncome => cards::copy_income(g, cx, seat)?,
        Op::Reuse => cards::reuse(g, cx, seat, ctx.card)?,
        Op::DiscardHand { resource, amount } => cards::discard_hand(g, cx, seat, *resource, *amount)?,
        Op::MoveOne { range } => ambassadors::move_one(g, cx, seat, *range)?,
        Op::SpreadAmbassadors => ambassadors::spread_ambassadors(g, cx, seat)?,
        Op::Consolidate { max } => ambassadors::consolidate(g, cx, seat, *max)?,
        Op::CutBotnet => removal::cut_botnet(g, cx, seat)?,

        // Threats and the team: the wordings differ, the rule doesn't.
        Op::Troops { count, each_country, destroy } => team::troops(g, cx, ctx, *count, *each_country, *destroy)?,
        Op::FlipStation { each_country } => team::flip_station(g, cx, ctx, *each_country)?,
        Op::CorruptCity => {
            for c in scope_countries(&w, ctx.scope) {
                let city = |h: usize| collecting_city(&w, g, h);
                if let Some(h) = adversary::most_infiltrated_city(&w, g, c, city) {
                    adversary::corrupt_city(g, cx, h);
                }
            }
        }
        Op::PlayersLose { resource, amount } => {
            for s in ctx.seats(&w, g) {
                let k = (*amount).min(g.resource(s, *resource));
                cx.add_resource(g, s, *resource, -k);
            }
        }
        Op::AdversaryBuild => adversary::build_all(g, cx, ctx.scope),
        Op::DestroyNode => team::destroy_node(g, cx, ctx),
        Op::EachDestroyNode { count } => team::each_destroy_node(g, cx, ctx, *count)?,
        Op::PullUnity { amount } => alignment::pull_unity(g, cx, *amount),
        Op::LoseInsight { amount } => {
            for s in ctx.seats(&w, g) {
                cx.add_key(g, &economy::insight_debt(s), *amount);
                cx.tally(g, "effects.insight_lost", *amount);
            }
        }

        // Threats.
        Op::Ambassadors { at, count } => threats::place_ambassadors(g, cx, ctx.scope, *at, *count)?,
        Op::Flag { name } => {
            // Threats execute after production and upkeep: the flag lasts through
            // the next round (cleanup counts it down), or it would never land (020).
            for c in scope_countries(&w, ctx.scope) {
                cx.set_key(g, &board::flag(*name, c), 2);
            }
        }
        Op::Requisition { resource, amount, cards: n } => {
            threats::requisition(g, cx, ctx.scope, *resource, *amount, *n)?
        }
        Op::LoseAmbassador => threats::lose_ambassador(g, cx, ctx.scope)?,
        Op::LoseNodeOr { resource, amount } => {
            threats::lose_node_or(g, cx, ctx.scope, *resource, *amount, ctx.label)?
        }
        Op::DiscardMostExpensive => threats::discard_most_expensive(g, cx, ctx.scope)?,
        Op::Compute { amount } => {
            if ctx.label == "megaproject" {
                cx.tally(g, "adversary.compute_corp", *amount);
            }
            adversary::gain_compute(g, cx, *amount)?
        }
        Op::ComputeEvery { amount } => adversary::compute_every(g, cx, *amount)?,
        Op::LoseTokens { amount } => threats::lose_tokens(g, cx, ctx.scope, *amount)?,

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
        Op::DealThreats { count } => threats::deal_extra(g, cx, *count),
        Op::Infiltrate { amount } => adversary::infiltrate(g, cx, *amount),
        Op::AdversarySpread => adversary::spread(g, cx, None)?,
        Op::Sacrifice => {
            for s in ctx.seats(&w, g) {
                alignment::sacrifice(g, cx, s)?;
            }
        }
        Op::BotnetAmbassadors { amount } => team::botnet_ambassadors(g, cx, *amount),
        Op::CleanCityAmbassadors { amount } => team::clean_city_ambassadors(g, cx, *amount),
        Op::RemoveBotnets { count } => team::remove_botnets(g, cx, *count)?,

        // Read, not resolved; `Content::check` keeps them out of resolving lists.
        Op::Income { .. }
        | Op::IncomePerTag { .. }
        | Op::BuildingDiscount { .. }
        | Op::CardDiscount { .. }
        | Op::When { .. }
        | Op::Threshold { .. }
        | Op::HardenAction { .. }
        | Op::Slots { .. }
        | Op::Logistics { .. }
        | Op::ProjectDrain { .. }
        | Op::AutoCounter { .. }
        | Op::UnityBonus { .. }
        | Op::GiveBonus { .. }
        | Op::SpreadShield
        | Op::CivicShelter
        | Op::Wall
        | Op::Patience
        | Op::SupplyInsight { .. }
        | Op::TagsFromHand { .. }
        | Op::ConvertUpkeep { .. }
        | Op::CityBuild { .. }
        | Op::CleanFactories { .. }
        | Op::StationBuild { .. }
        | Op::SpreadReach { .. }
        | Op::CorruptSooner { .. }
        | Op::Arrivals { .. }
        | Op::War => unreachable!("{e:?} is read, not resolved"),
    }
    Ok(())
}

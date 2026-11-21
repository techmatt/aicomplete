//! What every [`Op`] does: the one place an instant op's meaning is
//! dispatched, whichever card, threat or board prints it. [`Ctx`] says who
//! chooses, who is hit and where (see `op.rs`). Rules that belong to a module
//! stay there; an arm here calls them. Standing ops and the twist-only ops are
//! read by the rule they change and never resolve.

use crate::content::{Area, Facility, Op, RoundFlag, Side, TargetSpec};
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

/// 234: the prompt that asks for a card's target (the agents' target rule reads it).
pub const TARGET_PROMPT: &str = "target which hex?";

/// Resolves `ops` in order.
pub fn run(g: &mut GameState, cx: &mut Cx, ops: &[Op], ctx: &mut Ctx) -> Res {
    for (i, e) in ops.iter().enumerate() {
        match e {
            Op::Target { on } => aim(g, cx, *on, &ops[i + 1..], ctx)?,
            _ => one(g, cx, e, ctx)?,
        }
        if g.is_over() {
            break;
        }
    }
    Ok(())
}

/// 276: the targets among `targets` whose area `at` holds a facility a hardening token can protect.
pub(super) fn hardens_at(w: &World, g: &GameState, targets: &[usize], at: Area) -> Vec<usize> {
    targets.iter().copied().filter(|&t| area_of(w, t, at).iter().any(|&h| board::hardenable(g, h))).collect()
}

/// 236: the hexes, among a target's candidates `among`, where a map op that narrows its target can act: a push, a
/// gather, a firewall, or (276, Patrol II) a hardening with a facility it can protect in its area; `None` for any
/// other op, which acts wherever the target lands. One home for [`aim`] (what a target offers) and `feasible`
/// (whether an aimed list acts).
pub(super) fn narrowed(w: &World, g: &GameState, seat: usize, op: &Op, among: &[usize]) -> Option<Vec<usize>> {
    match op {
        Op::Push { pieces, .. } => Some(push_hexes(w, g, seat, pieces)),
        Op::Gather { pieces, strike, .. } => Some(gather_hexes(w, g, seat, pieces, *strike)),
        Op::Firewall => Some(firewall_hexes(w, g, seat)),
        Op::Harden { at, .. } => Some(hardens_at(w, g, among, *at)),
        _ => None,
    }
}

/// 234: the seat picks its one target among the hexes in reach meeting `on`; `rest` is the list after it. 236: when
/// every map op in `rest` narrows its target ([`narrowed`]), only hexes where one of them can act are offered, as
/// `feasible` reads them: a target where nothing could move paid for nothing (Patrol, 236's slow lane).
fn aim(g: &mut GameState, cx: &mut Cx, on: TargetSpec, rest: &[Op], ctx: &mut Ctx) -> Res {
    let w = cx.world;
    let seat = ctx.chooser();
    let mut hexes = target_hexes_before(&w, g, seat, on, rest);
    let map: Vec<&Op> = rest.iter().filter(|o| o.is_map()).collect();
    let sets: Option<Vec<Vec<usize>>> = map.iter().map(|o| narrowed(&w, g, seat, o, &hexes)).collect();
    if let Some(sets) = sets.filter(|_| !map.is_empty()) {
        let ok: Vec<usize> = sets.concat();
        hexes.retain(|h| ok.contains(h));
    }
    let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
    // The card whose Action or Effect this is, when one is (the agents' target rule reads it).
    let card = ctx.card.map(|c| format!(" [{}]", cards::def(&w, &c).name)).unwrap_or_default();
    let prompt = || format!("Seat {seat}: {TARGET_PROMPT} ({}){card}", on.words());
    let t = cx.choose(seat, prompt, opts)?.map(|i| hexes[i]);
    cx.tally(g, if t.is_some() { "effects.targets" } else { "effects.targets_missed" }, 1);
    // 235: a lone target is taken unasked and a missing one skips the map lines in silence: say which.
    cx.note(t, || match t {
        Some(h) => format!("Seat {seat}{card} targets {} ({})", hex_name(&w, h), on.words()),
        None => format!("Seat {seat}{card}: nothing in reach to target ({}); its map lines do nothing", on.words()),
    });
    ctx.target = Some(t);
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
            // 305 (D85): the prompt names the card (a building's Action by its card; a played card by `why`).
            let name = ctx.why.clone().or_else(|| ctx.card.as_ref().map(|c| cards::def(&w, c.as_str()).name.clone()));
            let prompt = || match &name {
                Some(n) => format!("Seat {seat}: {n}: which other player?"),
                None => format!("Seat {seat}: which other player?"),
            };
            ctx.other = cx.choose(seat, prompt, opts)?.map(|i| others[i]);
        }
        Op::OtherGain { resource, amount } => {
            if let Some(o) = ctx.other {
                cx.add_resource(g, o, *resource, *amount);
            }
        }
        Op::Transfer { resource, max } => resources::transfer(g, cx, ctx, seat, *resource, *max)?,
        Op::PlaceNode { count, range } => {
            for _ in 0..*count {
                let built = |g: &GameState| -> Vec<usize> {
                    (0..g.hexes.len()).filter(|&x| g.tokens(x, STATION) > 0 || g.tokens(x, FACTORY) > 0).collect()
                };
                let before = built(g);
                let Some(h) = board::place_node_within(g, cx, seat, *range)? else { break };
                ctx.node = Some(h);
                ctx.node_spots_before = before;
            }
        }
        // 212: the node power's rider: on or next to its spot, else nothing. 261: a station or factory its own
        // uncovered space built is none (it was not there to bring into reach).
        Op::IfNodeNext { spots, then, fresh } => {
            let stood = |x: &usize| is_city(&w, *x) || ctx.node_spots_before.contains(x);
            if ctx.node.is_some_and(|h| board::spots_near(&w, g, seat, h, spots, *fresh).iter().any(stood)) {
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
        Op::Target { on } => aim(g, cx, *on, &[], ctx)?,
        Op::LiftFallout { count, at } => {
            let zone = ctx.area(&w, g, seat, *at);
            for _ in 0..*count {
                let mut hexes = board::land_in_reach(&w, g, seat);
                if ctx.target.is_some() {
                    hexes = zone.iter().copied().filter(|&h| g.tokens(h, FALLOUT) > 0).collect();
                }
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
        Op::Damage { ambassadors, troops } => {
            let zone = ctx.area(&w, g, seat, Area::On);
            removal::damage(g, cx, seat, *ambassadors, *troops, &zone, ctx.target.is_some())?
        }
        Op::DamageCountry { ambassadors, troops } => removal::damage_country(g, cx, seat, *ambassadors, *troops)?,
        Op::DestroyAll => {
            let zone = ctx.target.map(|_| ctx.area(&w, g, seat, Area::On));
            removal::destroy_all(g, cx, seat, zone.as_deref())?
        }
        Op::RemoveBotnet => {
            let zone = ctx.area(&w, g, seat, Area::On);
            removal::remove_botnet(g, cx, seat, &zone)?
        }
        Op::RemoveAmbassadorsCountry { max, at } => {
            let zone = ctx.area(&w, g, seat, *at);
            removal::remove_ambassadors_country(g, cx, seat, *max, &zone, ctx.label)?
        }
        Op::BuildFactory => {
            let zone = ctx.area(&w, g, seat, Area::On);
            resources::build_factory(g, cx, seat, &zone)?
        }
        Op::Cleanse { amount, one_country, cities, only, at } => {
            let zone = ctx.area(&w, g, seat, *at);
            let target = ctx.target.flatten();
            removal::cleanse(g, cx, seat, *amount, *one_country, *cities, *only, &zone, target)?
        }
        Op::Liberate { own, ambassadors, troops, botnet } => {
            let zone = ctx.area(&w, g, seat, Area::On);
            removal::liberate_city(g, cx, seat, *own, *ambassadors, *troops, *botnet, &zone)?
        }
        Op::ClearCity { max } => {
            let zone = ctx.area(&w, g, seat, Area::On);
            removal::clear_city(g, cx, seat, *max, &zone)?
        }
        Op::Harden { count, at } => {
            let zone = ctx.area(&w, g, seat, *at);
            resources::harden(g, cx, seat, *count, &zone, ctx.target.flatten())?
        }
        Op::MoveAmbassadors { max, at } => {
            let zone = ctx.area(&w, g, seat, *at);
            // 236: from on or next to the target (or the supply); with no target, from anywhere.
            let sources = ctx.target.map(|_| ctx.area(&w, g, seat, Area::Near));
            ambassadors::move_ambassadors(g, cx, seat, *max, &zone, sources.as_deref())?
        }
        Op::GainAmbassadors { count } => cx.add_resource(g, seat, Resource::Ambassadors, *count),
        Op::PlaceAmbassadors { count, at } => {
            let zone = ctx.area(&w, g, seat, *at);
            ambassadors::place_ambassadors(g, cx, seat, *count, &zone)?
        }
        // 234 (Matt: "Shelter your nodes and facilities in reach this round"): a player card picks no country;
        // the seat's own pieces on every hex in its reach, this round (238, D96: its reach when struck).
        Op::Shelter => {
            cx.set_key(g, &board::reach_shelter(seat), 1);
            cx.tally(g, "effects.shelter_reach", 1);
        }
        // 234: with a target, the target's country (Hardpoint Lockdown).
        Op::Cordon if ctx.target.is_some() => {
            if let Some(t) = ctx.target.flatten() {
                let c = w.map.hexes[t].country as usize;
                if g.var(&board::flag(RoundFlag::Cordon, c)) < 2 {
                    cx.set_key(g, &board::flag(RoundFlag::Cordon, c), 2);
                }
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
        Op::BuildStation => {
            let zone = ctx.area(&w, g, seat, Area::On);
            resources::build_station(g, cx, seat, &zone)?
        }
        Op::ClaimSite { near } => {
            // 297: a Rescue's claim follows its liberation. With the target still a corrupted city (Peacekeeping
            // Mission's removal left a guard, so nothing was liberated), nothing is claimed.
            let unfreed = *near && ctx.area(&w, g, seat, Area::On).iter().any(|&h| crate::modules::common::corrupted_city(&w, g, h));
            if unfreed {
                cx.tally(g, "effects.claim_unfreed", 1);
            } else {
                let zone = ctx.area(&w, g, seat, if *near { Area::Near } else { Area::On });
                resources::claim_site(g, cx, seat, &zone, *near)?
            }
        }
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
        // 259: cubes off the adversary's research track.
        Op::RemoveResearch { amount } => adversary::remove_research(g, cx, *amount),
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
        Op::RemoveTroops { count, reach, at } => {
            let zone = ctx.target.map(|_| ctx.area(&w, g, seat, *at));
            removal::remove_troops(g, cx, seat, *count, *reach, zone.as_deref(), ctx.label)?
        }
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
            // 235: the printed word ("1 operative"), not the token key ("1 agents": the adversary's piece).
            let prompt = || format!("Seat {seat}: add {} to which card?", kind.amount(*amount));
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
        Op::Consolidate { max } => ambassadors::consolidate(g, cx, seat, *max, ctx.target)?,
        Op::Convert { max } => {
            let zone = ctx.area(&w, g, seat, Area::On);
            endgame::convert(g, cx, seat, *max, &zone)?
        }
        Op::Gather { max, pieces, strike } => {
            let zone = ctx.area(&w, g, seat, Area::On);
            moves::gather(g, cx, seat, *max, pieces, *strike, &zone)?
        }
        Op::Push { max, pieces } => {
            let zone = ctx.area(&w, g, seat, Area::On);
            moves::push(g, cx, seat, *max, pieces, &zone)?
        }
        Op::Firewall => {
            let zone = ctx.area(&w, g, seat, Area::On);
            endgame::firewall(g, cx, seat, &zone)?
        }
        Op::IfTags { tag, count, then } => {
            if cards::tags(&w, g, seat, *tag) >= *count {
                cx.tally(g, "effects.if_tags_met", 1);
                run(g, cx, then, ctx)?;
            }
        }
        Op::CutBotnet => {
            let zone = ctx.area(&w, g, seat, Area::On);
            removal::cut_botnet(g, cx, seat, &zone)?
        }

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
        Op::EachDestroyNode { count, factories } => team::each_destroy_node(g, cx, ctx, *count, *factories)?,
        Op::DestroyNode { count } => team::destroy_nearest_node(g, cx, ctx, *count)?,
        Op::PullUnity { amount } => alignment::pull_unity(g, cx, *amount),
        Op::PullDiscord { amount } => alignment::pull_discord(g, cx, *amount),

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
        // 195: the project deck's board verbs.
        Op::BotnetTroops { count, troops, least } => team::botnet_troops(g, cx, *count, *troops, *least)?,
        Op::SpreadBotnets { count } => team::spread_botnets(g, cx, *count),
        Op::TakeSite { site } => team::take_site(g, cx, *site)?,
        Op::FactoryTroops { count } => team::factory_troops(g, cx, *count),
        Op::Strike => board::strike(g, cx)?,
        Op::Redeploy => board::redeploy(g, cx)?,
        Op::AgentsOnNodes { count } => team::agents_on_nodes(g, cx, *count)?,
        Op::RecallAmbassadors => team::recall_ambassadors(g, cx),
        Op::StripCubes { count } => team::strip_cubes(g, cx, *count),
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
            let n = team::per_site_count(&w, g, *where_, *max);
            // 254: how often a per-site card pays, and how many of its owned sites are free cities by a player's node (stats only).
            cx.tally(g, &format!("effects.per_site.{}", where_.name()), n);
            if *where_ == crate::op::Where::Owned {
                let cities = super::plan::sites_where(&w, g, *where_).into_iter().filter(|&h| is_city(&w, h) && cube_owner(g, h).is_none()).count();
                cx.tally(g, "effects.per_site.owned_cities", cities as i64);
            }
            for _ in 0..n {
                run(g, cx, then, ctx)?;
                if g.is_over() {
                    break;
                }
            }
        }
        Op::RemoveWhere { pieces, count, per, where_ } => team::remove_where(g, cx, *pieces, *count, *per, *where_)?,
        Op::Prohibit { step, where_ } => adversary::prohibit(g, cx, *step, *where_),
        // 275, 293: the adversary's Infiltrate and Escalate lines, and its compute income.
        Op::Infiltrate { site } => {
            let source = if ctx.label == "standing" { "event" } else { ctx.label };
            adversary::infiltrate(g, cx, *site, ctx.scope, source)?
        }
        Op::Escalate { site } => {
            let source = if ctx.label == "standing" { "event" } else { ctx.label };
            adversary::escalate(g, cx, *site, ctx.scope, source)?
        }
        Op::IncreaseIncome { compute } => adversary::increase_income(g, cx, *compute),
        // 259: research cubes onto the adversary's track (a project's: a country each).
        Op::AdversaryResearch { amount, each_country } => {
            let n = if *each_country { *amount * w.map.countries.len() as i64 } else { *amount };
            let source = ctx.says("threats", if ctx.label == "standing" { "event" } else { ctx.label });
            adversary::gain_research(g, cx, n, source);
        }

        // 135: the new adversary model's ops, and the old ops with a new field set.
        Op::Troops { .. }
        | Op::FlipStation { .. }
        | Op::CorruptCity { .. }
        | Op::PlayersLose { .. }
        | Op::Create { .. }
        | Op::CreateSite { .. }
        | Op::Flag { .. }
        | Op::Requisition { .. }
        | Op::LoseNodeOr { .. }
        | Op::DealThreats { .. }
        | Op::AdversaryFactory { .. }
        | Op::ComputePerMissing { .. }
        | Op::DestroyFacility { .. }
        | Op::DisableBuilding { .. }
        | Op::Discard { .. }
        | Op::LoseAmbassadorsOr { .. }
        | Op::LoseIncome { .. }
        | Op::LoseOr { .. }
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
        => unreachable!("{e:?} is read, not resolved"),
    }
    Ok(())
}

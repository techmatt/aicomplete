//! The new adversary model's ops (135): threat effects that read the threat's
//! location tokens and the players near it, and the old ops' new fields.
//! With a threat resolving (`ctx.threat`), "here" is its token and "near it"
//! the hexes within 1 of its tokens; "each player near it" is `ctx.seats`.

use super::*;
use crate::key::KeyBuf;
use crate::op::{CorpAt, CorpNear, CorpWhen, DestroyAt, FactoryWhere, Scope, TheftWho, TroopAt, TroopIf, Until};
use crate::vocab::{Location, PullTag};

/// Resolve one of the 135 ops (or an old op with a 135 field set).
pub(super) fn one(g: &mut GameState, cx: &mut Cx, e: &Op, ctx: &mut Ctx) -> Res {
    let w = cx.world;
    cx.tally(g, &KeyBuf::new("effects.").text(op_name(e)), 1);
    match e {
        Op::Troops { count, each_country, destroy, at, stand, location, if_ } => {
            let land = Landing { count: *count, each_country: *each_country, destroy: *destroy, stand: *stand };
            troops(g, cx, ctx, land, *at, *location, *if_)?
        }
        Op::FlipStation { each_country, force: false } => team::flip_station(g, cx, ctx, *each_country)?,
        Op::FlipStation { each_country, force: true } => force_flip(g, cx, ctx, *each_country)?,
        Op::CorruptCity { at } => corrupt_city(g, cx, ctx, *at)?,
        Op::PlayersLose { resource, amount, all } => {
            for s in ctx.seats(&w, g) {
                let held = g.resource(s, *resource);
                let k = if *all { held } else { (*amount).min(held) };
                cx.add_resource(g, s, *resource, -k.max(0));
            }
        }
        Op::Create { .. } => {
            let line = super::super::plan::CreateLine::of(e).expect("a creation line");
            let near = if line.if_ == crate::op::CreateIf::Near { ctx.hexes(&w, g) } else { Vec::new() };
            let location = token(&w, g, ctx, None);
            adversary::create(g, cx, line, ctx.scope, &near, location, create_source(ctx))?;
        }
        Op::CreateSite { kind } => create_site(g, cx, ctx, *kind),
        Op::Corrupt { site, cut } => adversary::corrupt_sites(g, cx, *site, *cut, ctx.scope)?,
        Op::Flag { name, scope, rounds, everywhere } => flag(g, cx, ctx, *name, *scope, *rounds, *everywhere),
        Op::Requisition { resource, amount, cards: n, next_round } => {
            requisition(g, cx, ctx, *resource, *amount, *n, *next_round)?
        }
        Op::LoseNodeOr { resource, amount, count } => lose_node_or(g, cx, ctx, *resource, *amount, *count)?,
        Op::DealThreats { count, tier, scope, each_country } => {
            let tier = tier.unwrap_or_else(|| adversary::threat_tier(&w, g));
            let countries = if *each_country || *scope != Some(Scope::Country) {
                (0..w.map.countries.len()).collect()
            } else {
                scope_countries(&w, ctx.scope)
            };
            for c in countries {
                for _ in 0..*count {
                    threats::deal_tier(g, cx, tier, &[PullTag::Any], c)?;
                }
            }
        }
        Op::AdversaryFactory { count, where_, fallback_ambassadors } => {
            for _ in 0..*count {
                adversary_factory(g, cx, ctx, *where_, *fallback_ambassadors)?;
            }
        }
        Op::ComputePerMissing { resource, amount } => {
            let n = ctx.seats(&w, g).into_iter().filter(|&s| g.resource(s, *resource) <= 0).count() as i64;
            adversary::gain_compute(g, cx, n * amount)?;
        }
        Op::CorpTroops { count, at, near, when, past_cap, permanent, fill, rounds } => {
            let c = Corp { count: *count, at: *at, near: *near, past_cap: *past_cap, permanent: *permanent, fill: *fill, rounds: *rounds };
            corp_troops(g, cx, ctx, c, *when)
        }
        Op::DestroyFacility { at: DestroyAt::Factory } => destroy_factory(g, cx, ctx)?,
        Op::DisableBuilding { count, until: Until::EndOfNextRound } => {
            for s in ctx.seats(&w, g) {
                for _ in 0..*count {
                    let ids: Vec<Key> = g.pile(&cards::built(s)).iter().filter(|c| !cards::disabled(g, s, c)).cloned().collect();
                    let opts = ids.iter().map(|c| opt(*c)).collect();
                    let prompt = || format!("{}: turn which building face down?", head(ctx, s));
                    let Some(i) = cx.choose_loss(s, prompt, opts)? else {
                        break;
                    };
                    cards::disable(g, cx, s, &ids[i], g.round + 1);
                }
            }
        }
        Op::Discard { count } => {
            for s in ctx.seats(&w, g) {
                discard(g, cx, ctx, s, *count)?;
            }
        }
        Op::ExtraWar { ongoing_use: true, .. } => {
            // Read by the war step while its ongoing project stands (`board::war_steps`).
        }
        Op::ExtraWar { ongoing_use: false, .. } => {
            // Threats execute after the war step: "next round" is the next war step.
            if let Some(base) = token(&w, g, ctx, Some(Location::Base)) {
                for (i, f) in w.map.fronts.iter().enumerate() {
                    if f.bases.contains(&base) {
                        cx.set_key(g, &board::extra_war_key(i), g.round as i64 + 1);
                    }
                }
            }
        }
        Op::LoseAmbassadorsOr { count, resource, amount } => {
            for s in ctx.seats(&w, g) {
                lose_ambassadors_or(g, cx, ctx, s, *count, *resource, *amount)?;
            }
        }
        Op::LoseIncome { resource, amount } => {
            for s in ctx.seats(&w, g) {
                if *resource == Resource::Insight {
                    cx.add_key(g, &economy::insight_debt(s), *amount);
                } else {
                    cx.add_key(g, &economy::income_debt(s, *resource), *amount);
                }
            }
        }
        Op::LoseOr { options } => {
            for s in ctx.seats(&w, g) {
                let opts = options.iter().map(|o| opt(option_words(o))).collect();
                let prompt = || format!("{}: which do you lose?", head(ctx, s));
                if let Some(i) = cx.choose_loss(s, prompt, opts)? {
                    let mut c = Ctx::seat(s).named(ctx.label);
                    c.why = ctx.why.clone();
                    super::run(g, cx, std::slice::from_ref(&options[i]), &mut c)?;
                }
            }
        }
        Op::MarchNow { steps } => {
            if let Some(t) = &ctx.threat {
                let at = g.var(&board::landed_key(t));
                let n = g.var(&board::landed_count_key(t));
                if at > 0 && n > 0 {
                    board::march_now(g, cx, at as usize - 1, n, *steps)?;
                }
            }
        }
        Op::OccupyFactory { else_compute } => {
            if let Some(h) = token(&w, g, ctx, Some(Location::Factory)) {
                if g.tokens(h, TROOP) > 0 {
                    // The troop stays: it never marches while it holds the site.
                    board::add_standing(g, cx, h, 1);
                }
                if g.tokens(h, FACTORY) > 0 {
                    if g.tokens(h, TROOP) > 0 {
                        cx.set_key(g, &board::occupied_key(h), 1);
                    }
                } else {
                    adversary::gain_compute(g, cx, *else_compute)?;
                }
            }
        }
        Op::PlaceBotnet { count } => {
            for _ in 0..*count {
                if !place_botnet(g, cx, ctx)? {
                    break;
                }
            }
        }
        Op::SeizeFactory => {
            if let Some(h) = token(&w, g, ctx, Some(Location::Factory)) {
                if g.tokens(h, FACTORY) > 0 {
                    if !corrupted(g, h) {
                        adversary::corrupt_factory(g, cx, h)?;
                    }
                } else if w.map.hexes[h].industrial && !has_building(g, h) {
                    build_factory(g, cx, ctx, h);
                }
            }
        }
        Op::Theft { who: TheftWho::Nearest, resource, amount, compute, cards } => {
            // 146: the nearest player, as every threat harm hits (was the richest near it); D62: pay or discard,
            // as `Requisition`, so holding none dodges nothing.
            let before: i64 = ctx.seats(&w, g).iter().map(|&s| g.resource(s, *resource)).sum();
            requisition(g, cx, ctx, *resource, *amount, *cards, false)?;
            let after: i64 = ctx.seats(&w, g).iter().map(|&s| g.resource(s, *resource)).sum();
            cx.tally(g, "effects.theft_taken", before - after);
            adversary::gain_compute(g, cx, *compute)?;
        }
        other => unreachable!("{other:?} is not a 135 op"),
    }
    Ok(())
}

/// The op's name for its tally (`effects.<name>`): its serde tag, for each op
/// the dispatcher (`effects::one`) sends to [`one`].
fn op_name(e: &Op) -> &'static str {
    match e {
        Op::Troops { .. } => "troops",
        Op::FlipStation { .. } => "flip_station",
        Op::CorruptCity { .. } => "corrupt_city",
        Op::PlayersLose { .. } => "players_lose",
        Op::Create { .. } => "create",
        Op::CreateSite { .. } => "create_site",
        Op::Corrupt { .. } => "corrupt",
        Op::Flag { .. } => "flag",
        Op::Requisition { .. } => "requisition",
        Op::LoseNodeOr { .. } => "lose_node_or",
        Op::DealThreats { .. } => "deal_threats",
        Op::AdversaryFactory { .. } => "adversary_factory",
        Op::ComputePerMissing { .. } => "compute_per_missing",
        Op::CorpTroops { .. } => "corp_troops",
        Op::DestroyFacility { .. } => "destroy_facility",
        Op::DisableBuilding { .. } => "disable_building",
        Op::Discard { .. } => "discard",
        Op::ExtraWar { .. } => "extra_war",
        Op::LoseAmbassadorsOr { .. } => "lose_ambassadors_or",
        Op::LoseIncome { .. } => "lose_income",
        Op::LoseOr { .. } => "lose_or",
        Op::MarchNow { .. } => "march_now",
        Op::OccupyFactory { .. } => "occupy_factory",
        Op::PlaceBotnet { .. } => "place_botnet",
        Op::SeizeFactory => "seize_factory",
        Op::Theft { .. } => "theft",
        _ => unreachable!("new_ops::one runs only the ops listed here"),
    }
}

/// Who a prompt asks, named for the threat resolving.
fn head(ctx: &Ctx, s: usize) -> String {
    match &ctx.why {
        Some(n) => format!("{n}: seat {s}"),
        None => format!("Seat {s}"),
    }
}

/// The resolving threat's token of type `loc`, or with none, its first token.
fn token(w: &World, g: &GameState, ctx: &Ctx, loc: Option<Location>) -> Option<usize> {
    let t = ctx.threat.as_ref()?;
    match loc {
        Some(l) => threats::token_of(w, g, t, l).or_else(|| threats::tokens(g, t).first().copied()),
        None => threats::tokens(g, t).first().copied(),
    }
}

/// The threat's city token (a city or a coastal city).
fn city_token(w: &World, g: &GameState, ctx: &Ctx) -> Option<usize> {
    let t = ctx.threat.as_ref()?;
    [Location::City, Location::CoastalCity]
        .into_iter()
        .find_map(|l| threats::token_of(w, g, t, l))
        .or_else(|| threats::tokens(g, t).into_iter().find(|&h| is_city(w, h)))
}

#[derive(Debug, Clone, Copy)]
struct Landing {
    count: i64,
    each_country: bool,
    destroy: bool,
    stand: bool,
}

/// Troops (135 fields): on the threat's token or the factory the threat just
/// built (137 cut "each country's first base"); `if` empty_zone, only where the token's
/// zone held no factory (before the threat's own destruction); `stand`, they
/// never march or strike.
fn troops(
    g: &mut GameState,
    cx: &mut Cx,
    ctx: &Ctx,
    land: Landing,
    at: Option<TroopAt>,
    location: Option<Location>,
    if_: Option<TroopIf>,
) -> Res {
    let w = cx.world;
    let mut spots: Vec<usize> = match at {
        // No new landing place: today's (a factory near it, else a city).
        None => return team::troops(g, cx, ctx, land.count, land.each_country, land.destroy),
        Some(TroopAt::Location) => token(&w, g, ctx, location).into_iter().collect(),
        Some(TroopAt::NewFactory) => ctx
            .threat
            .as_ref()
            .map(|t| g.var(&board::new_factory_key(t)))
            .filter(|&v| v > 0)
            .map(|v| v as usize - 1)
            .into_iter()
            .collect(),
    };
    if if_ == Some(TroopIf::EmptyZone) {
        let emptied = ctx.threat.as_ref().map_or(0, |t| g.var(&board::destroyed_key(t)));
        spots.retain(|&h| g.tokens(h, FACTORY) == 0 && emptied != h as i64 + 1);
    }
    let tally = ctx.says("threats.troops", "adversary.project_troops");
    for h in spots {
        // 068: a landing enters its hex, so a Wall stops it too.
        if board::walled(&w, g, h) {
            cx.tally(g, "effects.troops_walled", land.count);
            continue;
        }
        for _ in 0..land.count {
            adversary::place_troop(g, cx, h);
            cx.per_round(g, adversary::OPS, 1);
        }
        cx.tally(g, tally, land.count);
        if land.stand {
            board::add_standing(g, cx, h, land.count);
        }
        if land.destroy {
            for s in 0..g.seats.len() {
                if g.tokens(h, node(s)) > 0 {
                    board::destroy_piece(g, cx, h, node(s), board::Cause::Effect(ctx.label))?;
                }
            }
        }
        if let Some(t) = &ctx.threat {
            cx.set_key(g, &board::landed_key(t), h as i64 + 1);
            cx.set_key(g, &board::landed_count_key(t), land.count);
        }
    }
    Ok(())
}

/// Lab Takeover: the station here is corrupted, even a player's.
fn force_flip(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, each_country: bool) -> Res {
    let w = cx.world;
    for hexes in team::regions(&w, g, ctx, each_country) {
        let mut spots: Vec<usize> = hexes.into_iter().filter(|&h| g.tokens(h, STATION) > 0 && !corrupted(g, h)).collect();
        // 146 (D63): a threat's "the station here" is its token's station, or nothing once that one is corrupted.
        if let Some(t) = ctx.threat.as_ref() {
            let here = threats::token_of(&w, g, t, Location::Station);
            spots.retain(|h| Some(*h) == here);
        }
        let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = || format!("{}: which research station flips to the adversary?", ctx.why.as_deref().unwrap_or("Threat"));
        if let Some(i) = cx.choose(0, prompt, opts)? {
            // 146 (D60): corruption, with its cleanup, even a player's station.
            adversary::corrupt_hex(g, cx, spots[i], crate::op::CorruptSite::Station)?;
            cx.tally(g, "adversary.stations_flipped", 1);
        }
    }
    Ok(())
}

/// CorruptCity (135): the threat's city. `at` location (Regime Change) also destroys the nodes on and next
/// to it, never a last one. The no-location form (the most infiltrated city) is gone (142, P1): the load
/// check refuses it.
fn corrupt_city(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, at: Option<Spot>) -> Res {
    let w = cx.world;
    let Some(h) = city_token(&w, g, ctx) else { return Ok(()) };
    if !corrupted(g, h) {
        adversary::corrupt_city(g, cx, h)?;
        if at == Some(Spot::Location) {
            for x in within(&w, h, 1) {
                for s in 0..g.seats.len() {
                    if g.tokens(x, node(s)) > 0 {
                        board::lose_node(g, cx, x, s, ctx.label);
                    }
                }
            }
        }
    }
    Ok(())
}

/// Where a creation line comes from, for its tally (138): a threat, the
/// event card (its standing lines), the board, a project, a Discord or Unity card.
fn create_source(ctx: &Ctx) -> &'static str {
    if ctx.threat.is_some() {
        return "threat";
    }
    match ctx.label {
        "standing" => "event",
        "board" => "board",
        "project" => "project",
        "discord" => "discord",
        "unity" => "unity",
        _ => "other",
    }
}

/// 138, site creation: in each country in scope, a neutral research station
/// on the lowest-numbered empty printed station site, or a neutral factory on
/// the lowest-numbered free industrial zone.
fn create_site(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, kind: Facility) {
    let w = cx.world;
    let zones = free_zones(&w, g);
    for c in scope_countries(&w, ctx.scope) {
        let mut spots: Vec<usize> = w.map.countries[c]
            .hexes
            .iter()
            .copied()
            .filter(|&h| match kind {
                Facility::Station => w.map.hexes[h].station_site && !has_building(g, h) && node_owner(g, h).is_none(),
                Facility::Factory => zones.contains(&h),
            })
            .collect();
        spots.sort_by_key(|&h| w.map.hexes[h].number);
        let Some(&h) = spots.first() else {
            continue;
        };
        let (token, name) = match kind {
            Facility::Station => (STATION, "research station"),
            Facility::Factory => (FACTORY, "factory"),
        };
        cx.add_tokens(g, h, token, 1);
        adversary::clear_markers_on(g, cx, h);
        cx.tally(g, &format!("adversary.sites_created.{}", if kind == Facility::Station { "station" } else { "factory" }), 1);
        cx.note(Some(h), || format!("A neutral {name} opens at {}", hex_name(&w, h)));
    }
}

/// Flag (135 fields): up for `rounds` more rounds (the var holds rounds + 1:
/// a threat executes after the round's production). A hex's flag (blackout,
/// embargo) goes on the hexes near it, the rest on the country; `scope`
/// country, the country; `everywhere`, every country.
#[allow(clippy::too_many_arguments)]
fn flag(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, name: RoundFlag, scope: Option<Scope>, rounds: i64, everywhere: bool) {
    let w = cx.world;
    let v = rounds.max(1) + 1;
    let raise = |g: &mut GameState, cx: &mut Cx, k: &str| {
        if g.var(k) < v {
            cx.set_key(g, k, v);
        }
    };
    if everywhere {
        for c in 0..w.map.countries.len() {
            raise(g, cx, &board::flag(name, c));
        }
    } else if name.on_hexes() && scope != Some(Scope::Country) && (scope == Some(Scope::Near) || ctx.threat.is_some()) {
        for h in ctx.hexes(&w, g) {
            raise(g, cx, &board::hex_flag(name, h));
        }
    } else {
        for c in scope_countries(&w, ctx.scope) {
            raise(g, cx, &board::flag(name, c));
        }
    }
}

/// Requisition (135: near it; `next_round`, the payment comes off next
/// round's `resource`, always payable): each player discards `n` cards or
/// pays; with neither possible, what they have and their hand.
#[allow(clippy::too_many_arguments)]
fn requisition(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, r: Resource, amount: i64, n: i64, next_round: bool) -> Res {
    let w = cx.world;
    for s in ctx.seats(&w, g) {
        let can_pay = next_round || g.resource(s, r) >= amount;
        let mut opts = Vec::new();
        if can_pay {
            let words = if next_round { format!("{} less next round", r.amount(amount)) } else { format!("pay {}", r.amount(amount)) };
            opts.push(opt(words));
        }
        if g.pile(&cards::hand(s)).len() as i64 >= n {
            opts.push(opt(format!("discard {n} cards")));
        }
        let prompt = || format!("{}: lose {} or discard {n} cards?", head(ctx, s), r.amount(amount));
        match cx.choose_loss(s, prompt, opts)? {
            Some(0) if can_pay => {
                if !next_round {
                    cx.add_resource(g, s, r, -amount);
                } else if r == Resource::Insight {
                    cx.add_key(g, &economy::insight_debt(s), amount);
                } else {
                    cx.add_key(g, &economy::income_debt(s, r), amount);
                }
            }
            Some(_) => discard(g, cx, ctx, s, n)?,
            None => {
                let k = g.resource(s, r);
                cx.add_resource(g, s, r, -k);
                for c in g.pile(&cards::hand(s)).to_vec() {
                    cards::discard(g, cx, &c, &cards::hand(s));
                }
            }
        }
    }
    Ok(())
}

/// Seat `s` discards `n` cards from hand, its choice.
fn discard(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, s: usize, n: i64) -> Res {
    let hand_key = cards::hand(s);
    for k in 0..n {
        let hand = g.pile(&hand_key);
        let prompt = || format!("{}: {} ({} of {n})?", head(ctx, s), cards::DISCARD_PROMPT, k + 1);
        let Some(j) = cx.choose_key(s, prompt, hand)? else {
            break;
        };
        let c = hand[j];
        cards::discard(g, cx, &c, &hand_key);
    }
    Ok(())
}

/// LoseNodeOr (135: `count` nodes, near it, never the last): each player near
/// it pays or loses nodes there (their choice); with neither, what they have.
fn lose_node_or(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, r: Resource, amount: i64, count: i64) -> Res {
    let w = cx.world;
    let hexes = ctx.hexes(&w, g);
    for s in ctx.seats(&w, g) {
        let mine = |g: &GameState| -> Vec<usize> {
            hexes.iter().copied().filter(|&h| g.tokens(h, node(s)) > 0).collect()
        };
        let can_pay = g.resource(s, r) >= amount;
        let mut opts = Vec::new();
        if can_pay {
            opts.push(opt(format!("pay {}", r.amount(amount))));
        }
        let off = opts.len();
        let first = mine(g);
        opts.extend(first.iter().map(|&h| labeled_hex_opt(&w, "lose nodes", h)));
        let prompt = || format!("{}: pay {} or lose {count} nodes?", head(ctx, s), r.amount(amount));
        match cx.choose_loss(s, prompt, opts)? {
            Some(i) if i < off => cx.add_resource(g, s, r, -amount),
            Some(i) => {
                board::lose_node(g, cx, first[i - off], s, ctx.label);
                for k in 1..count {
                    let more = mine(g);
                    let opts = more.iter().map(|&h| hex_opt(&w, h)).collect();
                    let prompt = || format!("{}: lose which node ({} of {count})?", head(ctx, s), k + 1);
                    let Some(j) = cx.choose_loss(s, prompt, opts)? else {
                        break;
                    };
                    board::lose_node(g, cx, more[j], s, ctx.label);
                }
            }
            None => {
                let k = g.resource(s, r);
                cx.add_resource(g, s, r, -k);
            }
        }
    }
    Ok(())
}

/// Seat `s` pays `amount` `r` or loses `count` ambassadors (supply first,
/// then theirs on the board, their choice); with neither, what they have.
#[allow(clippy::too_many_arguments)]
fn lose_ambassadors_or(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, s: usize, count: i64, r: Resource, amount: i64) -> Res {
    let w = cx.world;
    let held = g.resource(s, Resource::Ambassadors) + board::ambassadors_on_board(g, s);
    let can_pay = g.resource(s, r) >= amount;
    let mut opts = Vec::new();
    if can_pay {
        opts.push(opt(format!("pay {}", r.amount(amount))));
    }
    if held > 0 {
        opts.push(opt(format!("lose {count} ambassadors")));
    }
    let prompt = || format!("{}: pay {} or lose {count} ambassadors?", head(ctx, s), r.amount(amount));
    match cx.choose_loss(s, prompt, opts)? {
        Some(0) if can_pay => cx.add_resource(g, s, r, -amount),
        Some(_) => {
            for _ in 0..count {
                if g.resource(s, Resource::Ambassadors) > 0 {
                    cx.add_resource(g, s, Resource::Ambassadors, -1);
                    continue;
                }
                let on = ambassador_hexes(g, s);
                let opts = on.iter().map(|&h| hex_opt(&w, h)).collect();
                let prompt = || format!("{}: lose the ambassador on which hex?", head(ctx, s));
                let Some(i) = cx.choose_loss(s, prompt, opts)? else {
                    break;
                };
                cx.add_tokens(g, on[i], ambassador(s), -1);
            }
        }
        None => {
            let k = g.resource(s, r);
            cx.add_resource(g, s, r, -k);
        }
    }
    Ok(())
}

/// What a LoseOr option says, for its choice.
fn option_words(o: &Op) -> String {
    match o {
        Op::PlayersLose { resource, amount, all: false } => format!("lose {}", resource.amount(*amount)),
        Op::PlayersLose { resource, all: true, .. } => format!("lose all your {resource}"),
        Op::LoseInsight { amount } => format!("{} less next round", Resource::Insight.amount(*amount)),
        Op::Discard { count } => format!("discard {count} card{}", if *count == 1 { "" } else { "s" }),
        other => format!("{other:?}"),
    }
}

/// The adversary builds a factory on free zone `z` (its cube on it).
fn build_factory(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, z: usize) {
    let w = cx.world;
    cx.add_tokens(g, z, FACTORY, 1);
    cx.add_tokens(g, z, ADV_CUBE, 1);
    adversary::clear_markers_on(g, cx, z);
    cx.tally(g, "board.factories_built.adversary", 1);
    cx.tally(g, "effects.adversary_factories", 1);
    if let Some(t) = &ctx.threat {
        cx.set_key(g, &board::new_factory_key(t), z as i64 + 1);
    }
    cx.note(Some(z), || format!("The adversary builds a factory at {}", hex_name(&w, z)));
}

/// AdversaryFactory: on the free zone nearest the threat's token (ties:
/// country, then hex number); `where` most_free_zones, in the country with the
/// most free zones (ties: the first), on its zone next to an adversary
/// ambassador if any, else its lowest-numbered. None free: `fallback`
/// agents on a factory (players choose).
fn adversary_factory(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, at: Option<FactoryWhere>, fallback: i64) -> Res {
    let w = cx.world;
    let zones = free_zones(&w, g);
    let country = |z: usize| w.map.hexes[z].country as usize;
    let number = |z: usize| w.map.hexes[z].number;
    let pick = match at {
        Some(FactoryWhere::MostFreeZones) => {
            let most = (0..w.map.countries.len()).rev().max_by_key(|&c| zones.iter().filter(|&&z| country(z) == c).count());
            most.and_then(|c| {
                zones
                    .iter()
                    .copied()
                    .filter(|&z| country(z) == c)
                    .min_by_key(|&z| (!next_to(&w, z, |x| g.tokens(x, ADV_AMBASSADOR) > 0), number(z)))
            })
        }
        None => {
            let from = token(&w, g, ctx, Some(Location::Factory));
            zones.iter().copied().min_by_key(|&z| (from.map_or(0, |f| w.map.distance(f, z)), country(z), number(z)))
        }
    };
    if let Some(z) = pick {
        build_factory(g, cx, ctx, z);
        return Ok(());
    }
    if let Some(t) = &ctx.threat {
        cx.set_key(g, &board::new_factory_key(t), 0);
    }
    if fallback > 0 {
        let factories: Vec<usize> = (0..g.hexes.len()).filter(|&h| g.tokens(h, FACTORY) > 0).collect();
        let opts = factories.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = || format!("{}: agents land on which factory?", ctx.why.as_deref().unwrap_or("The adversary"));
        if let Some(i) = cx.choose_loss(0, prompt, opts)? {
            adversary::place(g, cx, factories[i], fallback)?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Corp {
    count: i64,
    at: CorpAt,
    near: Option<CorpNear>,
    past_cap: bool,
    permanent: bool,
    fill: bool,
    rounds: i64,
}

/// CorpTroops. `now`: the bases (the threat's base, or every base) make them
/// at once (General Mobilization past the cap, War Footing to the cap).
/// `each_production` from a threat registers them on its base for `rounds`
/// rounds from the next (or for good, `permanent`); an ongoing project's and
/// the adversary board's are read by the production step, so here do nothing.
fn corp_troops(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, c: Corp, when: CorpWhen) {
    let w = cx.world;
    let mut bases: Vec<usize> = match c.at {
        CorpAt::Location => token(&w, g, ctx, Some(Location::Base)).filter(|&h| is_base(&w, h)).into_iter().collect(),
        CorpAt::EveryBase => (0..g.hexes.len()).filter(|&h| is_base(&w, h)).collect(),
    };
    if c.near == Some(CorpNear::AdversaryTroop) {
        bases.retain(|&b| next_to(&w, b, |x| g.tokens(x, TROOP) > 0));
    }
    match when {
        CorpWhen::Now => {
            let cap = w.int("board.unit_cap");
            for b in bases {
                let have = g.tokens(b, UNIT);
                let n = if c.fill {
                    (cap - have).max(0)
                } else if c.past_cap {
                    c.count
                } else {
                    c.count.min(cap - have).max(0)
                };
                if n > 0 {
                    // 137: a player ambassador on the base is used up instead of each troop.
                    let made = board::corp_troops_at(g, cx, b, n, true);
                    cx.tally(g, "board.units_extra_now", made);
                }
            }
        }
        CorpWhen::EachProduction => {
            if ctx.threat.is_none() {
                return;
            }
            let until = if c.permanent { 1_000_000 } else { g.round + c.rounds.max(1) as u32 };
            for b in bases {
                cx.add_key(g, &board::corp_extra_key(b, until), c.count);
            }
        }
    }
}

/// Scorched Earth: the factory under the threat's token is destroyed (as any
/// destruction, absorbed the same way), with the nodes on its hex, never a
/// last one.
fn destroy_factory(g: &mut GameState, cx: &mut Cx, ctx: &Ctx) -> Res {
    let w = cx.world;
    let Some(h) = token(&w, g, ctx, Some(Location::Factory)) else {
        return Ok(());
    };
    if g.tokens(h, FACTORY) == 0 {
        return Ok(());
    }
    if !board::destroy_piece(g, cx, h, FACTORY, board::Cause::Effect(ctx.label))? {
        return Ok(());
    }
    if g.tokens(h, FACTORY) == 0 && g.tokens(h, STATION) == 0 && !is_city(&w, h) && corrupted(g, h) {
        cx.add_tokens(g, h, ADV_CUBE, -1);
    }
    if let Some(t) = &ctx.threat {
        cx.set_key(g, &board::destroyed_key(t), h as i64 + 1);
    }
    // r75 (Matt's ruling 5): the node on the factory's hex is guarded like any node.
    for s in 0..g.seats.len() {
        if g.tokens(h, node(s)) > 0 {
            board::destroy_piece(g, cx, h, node(s), board::Cause::Effect(ctx.label))?;
        }
    }
    Ok(())
}

/// Sleeper Cells: a botnet on the threat's station, else on the nearest piece
/// without one (players choose among equals). Returns whether one landed.
fn place_botnet(g: &mut GameState, cx: &mut Cx, ctx: &Ctx) -> Res<bool> {
    let w = cx.world;
    let Some(from) = token(&w, g, ctx, Some(Location::Station)) else {
        return Ok(false);
    };
    let free = |h: usize| has_piece(&w, g, h) && g.tokens(h, BOTNET) == 0;
    let spots: Vec<usize> = if free(from) {
        vec![from]
    } else {
        let d = (0..g.hexes.len()).filter(|&h| free(h)).map(|h| w.map.distance(from, h)).min();
        (0..g.hexes.len()).filter(|&h| free(h) && Some(w.map.distance(from, h)) == d).collect()
    };
    let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
    let prompt = || format!("{}: a botnet lands on which piece?", ctx.why.as_deref().unwrap_or("Threat"));
    let Some(i) = cx.choose_loss(0, prompt, opts)? else {
        return Ok(false);
    };
    let h = spots[i];
    cx.add_tokens(g, h, BOTNET, 1);
    cx.tally(g, "adversary.botnets_made", 1);
    cx.peak(g, "adversary.botnets_peak", adversary::botnet_count(g, None));
    cx.note(Some(h), || format!("A botnet lands at {}", hex_name(&w, h)));
    Ok(true)
}

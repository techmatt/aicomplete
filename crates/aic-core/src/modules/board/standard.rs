//! The standard actions and projects: what each costs, targets and does.

use super::*;

/// A standard action. `Action::Standard` carries its [`name`](Self::name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardAction {
    Station,
    Remove,
    /// 210 (Matt, candidate): research cubes lift 1 Fallout token in reach, the one standard action that
    /// removes corruption (r91 cut Clear, which took markers for credits).
    Lift,
    /// 149: a free move from the seat's Dispatch (the paid Move is gone).
    Move,
    Ambassador,
    /// 137: claim a neutral city, station or factory (it was Align, cities only).
    Claim,
    /// 137: liberate an adversary site with massed ambassadors (it replaced
    /// Free a station and the city fallback).
    Liberate,
    /// 105: the general converter, a research cube for insight, no limit a round.
    Insight,
    /// 136: spend an ambassador to move 1 Discord off the Discord track.
    Envoy,
    /// r75 (Matt's ruling 3): credits, your ambassador on a botnet's hex (it stays): the botnet comes off.
    Botnet,
    /// r75 (Matt's ruling 6): credits for a neutral city's energy in the action phase (it was sold in production).
    BuyEnergy,
    /// 204 (Matt, 2026-11-12): the desperate source, research cubes for 1 antimatter.
    Antimatter,
    /// 206 (Matt, 2026-11-12): the one node placement every board has, poorly priced (Expand left every board).
    Node,
    /// 206 (amendment 9247): a very expensive hedge against troops: credits for 1 troop in reach.
    Troop,
    /// 206 (amendment 9212, candidate): a factory, yours, for chips (`factory_build_chips` 0: off).
    Factory,
}

/// The standard actions, in the order they are offered and published.
pub const STANDARD: [StandardAction; 15] = [
    StandardAction::Station,
    StandardAction::Remove,
    StandardAction::Lift,
    StandardAction::Move,
    StandardAction::Ambassador,
    StandardAction::Claim,
    StandardAction::Liberate,
    StandardAction::Insight,
    StandardAction::Envoy,
    StandardAction::Botnet,
    StandardAction::BuyEnergy,
    StandardAction::Antimatter,
    StandardAction::Node,
    StandardAction::Troop,
    StandardAction::Factory,
];

impl StandardAction {
    /// The action's name: `Action::Standard.act`, and its tally `board.std.<name>`.
    pub const fn name(self) -> &'static str {
        match self {
            StandardAction::Station => "station",
            StandardAction::Remove => "remove",
            StandardAction::Lift => "lift",
            StandardAction::Move => "move",
            StandardAction::Ambassador => "ambassador",
            StandardAction::Claim => "claim",
            StandardAction::Liberate => "liberate",
            StandardAction::Insight => "insight",
            StandardAction::Envoy => "envoy",
            StandardAction::Botnet => "botnet",
            StandardAction::BuyEnergy => "buy_energy",
            StandardAction::Antimatter => "antimatter",
            StandardAction::Node => "node",
            StandardAction::Troop => "troop",
            StandardAction::Factory => "factory",
        }
    }

    /// The action called `name`, if there is one.
    pub fn parse(name: &str) -> Option<StandardAction> {
        STANDARD.into_iter().find(|a| a.name() == name)
    }
}

/// What something costs: the one shape every standard action and every
/// placement of node is priced in.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Price {
    pub energy: i64,
    pub credits: i64,
    pub chips: i64,
    /// 066: player ambassadors from the supply (removing, aligning).
    pub ambassadors: i64,
    /// 105: research cubes (the general converter).
    pub cubes: i64,
}

impl Price {
    /// What it charges, in print order.
    fn parts(self) -> [(i64, Resource); 5] {
        [
            (self.energy, Resource::Energy),
            (self.credits, Resource::Credits),
            (self.chips, Resource::Chips),
            (self.ambassadors, Resource::Ambassadors),
            (self.cubes, Resource::ResearchCubes),
        ]
    }

    /// The resources it charges, in print order.
    pub(super) fn words(self) -> Vec<String> {
        self.parts().iter().filter(|(n, _)| *n > 0).map(|(n, r)| r.amount(*n)).collect()
    }

    /// The same for a label: `1 credit`.
    pub(super) fn label(self) -> Vec<String> {
        self.parts().iter().filter(|(n, _)| *n > 0).map(|(n, r)| r.words(*n)).collect()
    }

    pub fn affordable(self, g: &GameState, seat: usize) -> bool {
        self.parts().iter().all(|(n, r)| g.resource(seat, *r) >= *n)
    }

    pub(super) fn pay(self, g: &mut GameState, cx: &mut Cx, seat: usize) {
        for (n, r) in self.parts() {
            cx.add_resource(g, seat, r, -n);
        }
        cx.tally(g, "board.chips_spent", self.chips);
    }
}

/// A standard action's price as the rules print it (066: each resource paid
/// in its home domain).
pub fn printed_price(w: &World, act: StandardAction) -> Price {
    let i = |k: &str| w.int(&format!("board.{k}"));
    let credits = |credits| Price { credits, ..Price::default() };
    let chips = |chips| Price { chips, ..Price::default() };
    let ambassadors = |ambassadors| Price { ambassadors, ..Price::default() };
    match act {
        StandardAction::Station => Price { chips: i("station_chips"), ambassadors: i("station_ambassadors"), ..Price::default() },
        StandardAction::Remove => ambassadors(i("remove_ambassadors")),
        StandardAction::Lift => Price { cubes: i("land_cubes"), ..Price::default() },
        // 149: paid from the seat's Dispatch, not a resource (`moves_left`).
        StandardAction::Move => Price::default(),
        StandardAction::Ambassador => credits(i("ambassador_credits")),
        // 137: the ambassadors depend on the site: [`target_price`].
        StandardAction::Claim | StandardAction::Liberate => Price::default(),
        StandardAction::Insight => Price { cubes: 1, ..Price::default() },
        StandardAction::Envoy => ambassadors(i("envoy_ambassadors")),
        StandardAction::Botnet => credits(i("botnet_credits")),
        StandardAction::BuyEnergy => credits(i("city_energy_credits")),
        StandardAction::Antimatter => Price { cubes: i("antimatter_cubes"), ..Price::default() },
        StandardAction::Node => Price { credits: i("node_credits"), energy: i("node_energy"), ..Price::default() },
        StandardAction::Troop => credits(i("troop_credits")),
        StandardAction::Factory => chips(i("factory_build_chips")),
    }
}

/// The part of seat `seat`'s standard action's price that depends on its target hex `h`
/// (137; 140's hook): Claim's and Liberate's ambassadors from the supply, the site's
/// `claim_cost` or `liberate_cost` less the seat's own ambassadors standing on it (r75's one payment rule;
/// `None`: no such site there). The rest are priced whole by [`printed_price`] and add nothing here. The
/// targets list only hexes the seat can pay this for, and the action pays it on the hex chosen.
pub fn target_price(w: &World, g: &GameState, seat: usize, act: StandardAction, h: usize) -> Option<Price> {
    let ambassadors = |ambassadors| Price { ambassadors, ..Price::default() };
    match act {
        StandardAction::Claim => site_supply_cost(w, g, seat, h, false).map(ambassadors),
        StandardAction::Liberate => site_supply_cost(w, g, seat, h, true).map(ambassadors),
        _ => Some(Price::default()),
    }
}

/// Set while neutral city `h`'s energy has been bought this round (r75's Buy energy: once per city a round), a
/// round mark cleared at the round's end.
pub fn bought_key(h: usize) -> KeyBuf {
    KeyBuf::new(BOUGHT).num(h)
}

/// Prefix of [`bought_key`].
pub(super) const BOUGHT: &str = "board.energy_bought.";

/// 206: how often seat `s` has taken limited standard action `act` (Place a node, Remove a troop) this round, a
/// round mark.
pub fn std_once_key(act: StandardAction, seat: usize) -> KeyBuf {
    KeyBuf::new(STD_ONCE).text(act.name()).text(".").num(seat)
}

/// Prefix of [`std_once_key`].
pub(super) const STD_ONCE: &str = "board.std_once.";

/// 206: how often a seat may take standard action `act` a round (`node_per_round`, `troop_per_round`), or `None`
/// with no limit.
fn per_round(w: &World, act: StandardAction) -> Option<i64> {
    match act {
        StandardAction::Node => Some(w.int("board.node_per_round")),
        StandardAction::Troop => Some(w.int("board.troop_per_round")),
        _ => None,
    }
}

/// Whether seat `seat` may buy neutral city `h`'s energy now (r75, Matt's ruling 6): a neutral city in reach,
/// producing (no blackout, no agent on it), with energy to sell, not bought this round.
pub fn energy_for_sale(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    is_city(w, h)
        && !corrupted(g, h)
        && cube_owner(g, h).is_none()
        && in_reach(w, g, seat, h)
        && !city_dark(w, g, h)
        && city_output(w, h) > 0
        && g.var(&bought_key(h)) == 0
}

/// The hexes holding a botnet that seat `seat` may take down (r75, Matt's ruling 3): its own ambassador
/// stands on the hex, and (r81, friction #54) the hex is in its reach, like every other action's target.
pub fn botnet_targets(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| g.tokens(h, BOTNET) > 0 && g.tokens(h, ambassador(seat)) > 0 && in_reach(w, g, seat, h))
        .collect()
}

/// The var counting seat `s`'s Dispatch used this round (149), cleared in cleanup.
pub fn moved_key(seat: usize) -> KeyBuf {
    KeyBuf::new(MOVED).num(seat)
}

/// Prefix of [`moved_key`].
pub(super) const MOVED: &str = "board.moved.";

/// Seat `s`'s Dispatch left this round (149): its board's [`ai::movement`]
/// less the free moves it has made.
pub fn moves_left(w: &World, g: &GameState, seat: usize) -> i64 {
    ai::movement(w, g, seat) - g.var(&moved_key(seat))
}

/// 206: the ops an op-backed standard action runs (node placement, troop removal, factory building), so its
/// targets and effect are the cards' own (`effects::feasible`, `effects::run`).
pub fn std_ops(act: StandardAction) -> &'static [Op] {
    match act {
        StandardAction::Node => &[Op::PlaceNode { count: 1 }],
        // 236: each names its target first, as a card does.
        StandardAction::Troop => &[
            Op::Target { on: crate::op::TargetSpec::Troops },
            Op::RemoveTroops { count: 1, reach: true, at: crate::op::Area::On },
        ],
        StandardAction::Factory => &[Op::Target { on: crate::op::TargetSpec::FreeZone }, Op::BuildFactory],
        _ => &[],
    }
}

/// 206: a standard action whose price param is 0 is off (Lift, Node, Troop, Factory). 210 (P2) deleted the cut Clear,
/// Drain and Harden project.
/// 221: the price params live in [`printed_price`] alone (their minimum is 0, so "free" is "off").
/// 248: `board.std_off` switches any of them off, by its place in [`STANDARD`] from 1 (0: none), for a probe of
/// how much play leans on them; the shipped game has none off.
fn cut(w: &World, act: StandardAction) -> bool {
    let off = w.list("board.std_off");
    (matches!(act, StandardAction::Lift | StandardAction::Node | StandardAction::Troop | StandardAction::Factory)
        && printed_price(w, act) == Price::default())
        || off.iter().any(|&i| i > 0 && STANDARD.get(i as usize - 1) == Some(&act))
}

/// Whether standard action `act` is in the rules (206: a cut one prints no row and is never offered).
pub fn std_offered(w: &World, act: StandardAction) -> bool {
    !cut(w, act)
}

/// 236 (Matt, 2026-11-14: one target): the target a standard action that touches the map names, a hex in your
/// reach, printed first ("Target: Fallout."). Place a node has none: a placement goes next to your nodes.
pub fn std_target(act: StandardAction) -> Option<&'static str> {
    Some(match act {
        StandardAction::Station => "an open hex",
        StandardAction::Remove => "Agents",
        StandardAction::Lift => "Fallout",
        StandardAction::Move => "a piece or a Botnet",
        StandardAction::Claim => "a neutral site with no Agent on it",
        StandardAction::Liberate => "an adversary site with no Agent on it",
        StandardAction::Botnet => "a Botnet on a hex holding your ambassador",
        StandardAction::BuyEnergy => "a producing neutral city",
        StandardAction::Troop => "Troops",
        StandardAction::Factory => "a free industrial zone",
        StandardAction::Ambassador
        | StandardAction::Insight
        | StandardAction::Envoy
        | StandardAction::Antimatter
        | StandardAction::Node => return None,
    })
}

/// "Target: X. " before a standard action's words, where it names one.
fn aimed(act: StandardAction, text: String) -> String {
    match std_target(act) {
        Some(t) => format!("Target: {t}. {text}"),
        None => text,
    }
}

/// What a standard action does, for the view.
pub(super) fn std_text(w: &World, act: StandardAction) -> String {
    let text = match act {
        // 235: the cubes in the words, as Antimatter's: the view's price has no cubes, so the button showed it free.
        StandardAction::Lift => format!("Spend {}: lift it", Resource::ResearchCubes.words(w.int("board.land_cubes"))),
        StandardAction::Station => "Build a research station on it (yours)".into(),
        StandardAction::Remove => {
            let n = w.int("board.remove_ambassadors");
            let who = if n == 1 { "an ambassador".to_string() } else { format!("{n} ambassadors") };
            format!("Spend {who} from your supply: remove up to {} on or next to it", w.int("board.remove_count"))
        }
        // 178 (Matt's ruling #22): Dispatch, not Move (it plays the same: the ambassador comes from anywhere).
        StandardAction::Move => "Use 1 Dispatch: put a player ambassador on it, from your supply or the board".into(),
        StandardAction::Ambassador => "gain a player ambassador".into(),
        StandardAction::Claim => format!(
            "Claim it: {} ambassadors for a city, {} for a station, {} for a factory; yours on it pay first",
            claim_ambassadors(w, SiteKind::City),
            claim_ambassadors(w, SiteKind::Station),
            claim_ambassadors(w, SiteKind::Factory)
        ),
        StandardAction::Liberate => format!(
            // 235: a factory's price is its own (claim_factory), not the station's.
            "Liberate it: {} ambassadors for a city, {} for a station, {} for a factory; yours on it pay first; it becomes yours",
            liberate_ambassadors(w, SiteKind::City),
            liberate_ambassadors(w, SiteKind::Station),
            liberate_ambassadors(w, SiteKind::Factory)
        ),
        StandardAction::Insight => format!("turn 1 research cube into {}", Resource::Insight.words(w.int("board.cube_insight"))),
        StandardAction::Envoy => "move 1 Discord off the Discord track".into(),
        StandardAction::Botnet => "Take it down (your ambassador stays)".into(),
        StandardAction::BuyEnergy => {
            format!("Buy its energy (once per city a round): {} credits", w.int("board.city_energy_credits"))
        }
        StandardAction::Antimatter => {
            format!("turn {} into 1 antimatter", Resource::ResearchCubes.words(w.int("board.antimatter_cubes")))
        }
        StandardAction::Node => {
            format!("place 1 node next to yours (paying its track step), {} a round", times(w.int("board.node_per_round")))
        }
        StandardAction::Troop => format!("Remove 1 on it, {} a round", times(w.int("board.troop_per_round"))),
        StandardAction::Factory => "Build a factory on it (yours)".into(),
    };
    aimed(act, text)
}

/// What a standard action does, in the rule book's longer words: its name,
/// cost and effect, the row `aic-sim docs` prints in docs/rules/economy.md
/// (121: one home for them, beside the view's [`std_text`]). `t` prints a
/// tunable by its key (the docs print `` `remove_count` 3 ``); `w` drops a clause whose tunable is 0 (221).
pub fn std_rule_text(act: StandardAction, w: &World, t: &dyn Fn(&str) -> String) -> [String; 3] {
    let (name, cost, what): (&str, String, String) = match act {
        StandardAction::Station => (
            "Build a research station",
            if w.int("board.station_ambassadors") > 0 {
                format!("{} chips and {} ambassadors, used up", t("board.station_chips"), t("board.station_ambassadors"))
            } else {
                format!("{} chips", t("board.station_chips"))
            },
            "a research station goes on it (not an industrial zone or Fallout); it is yours (your cube)".into(),
        ),
        // r75 (Matt's ruling 1): it places nothing (it stood on the hex and removed up to 3 there). 236: on or next
        // to its target (it took them from any hexes in reach).
        StandardAction::Remove => (
            "Remove agents",
            format!("{} of your ambassadors, from your supply, used up", t("board.remove_ambassadors")),
            format!("remove up to {} agents on or next to it, your choice", t("board.remove_count")),
        ),
        // 210 (Matt, candidate: "costly but not absurd").
        StandardAction::Lift => (
            "Lift Fallout",
            format!("{} research cubes", t("board.land_cubes")),
            "its Fallout token comes off: the one standard action that removes Fallout".into(),
        ),
        StandardAction::Move => (
            "Dispatch a player ambassador",
            "1 of your Dispatches this round (your board's, unused ones lost)".into(),
            "a player ambassador goes on it, from your supply or anywhere on the board (not the target itself)".into(),
        ),
        StandardAction::Ambassador => (
            "Gain a player ambassador",
            format!("{} credits", t("board.ambassador_credits")),
            "into your supply; a standard project too".into(),
        ),
        // 137: one action for every site type (it was Align, cities only, for 1).
        StandardAction::Claim => (
            "Claim a site",
            format!(
                "{} ambassadors for a city, {} for a station, {} for a factory (Liberate's table too); used up: yours on the site first, your supply the rest",
                t("board.claim_city"),
                t("board.claim_station"),
                t("board.claim_factory")
            ),
            "a neutral city, research station or factory, no embargo on it: your cube goes on it".into(),
        ),
        // 137: it replaced Free a station and the city fallback. r75 (Matt's ruling 2): Claim's payment, and
        // the site becomes yours.
        StandardAction::Liberate => (
            "Liberate a site",
            format!(
                "Claim's: {} ambassadors for a city, {} for a station, {} for a factory; used up: yours on the site first, your supply the rest",
                t("board.liberate_at"),
                t("board.claim_station"),
                t("board.claim_factory")
            ),
            format!(
                "an adversary city, station or factory: the adversary's cube comes off, yours goes on, and it sways Unity ({} for a city, {} for a station or factory)",
                t("alignment.free_city"),
                t("alignment.free_facility")
            ),
        ),
        // 105: the general converter every player has.
        StandardAction::Insight => (
            "Turn a cube into insight",
            "1 research cube".into(),
            format!("gain {} insight, as often as you have cubes", t("board.cube_insight")),
        ),
        // 136: any seat, as often as it has ambassadors.
        StandardAction::Envoy => (
            "Envoy",
            format!("{} of your ambassadors, from your supply, used up", t("board.envoy_ambassadors")),
            "move 1 Discord off the Discord track".into(),
        ),
        // r75 (Matt's ruling 3): the first standard way to take a botnet down.
        StandardAction::Botnet => (
            "Take down a botnet",
            format!("{} credits", t("board.botnet_credits")),
            if w.int("alignment.botnet") > 0 {
                format!("the botnet comes off (your ambassador stays): sway {} toward Unity", t("alignment.botnet"))
            } else {
                "the botnet comes off (your ambassador stays)".into()
            },
        ),
        // r75 (Matt's ruling 6): it was a sale in production.
        StandardAction::BuyEnergy => (
            "Buy a city's energy",
            format!("{} credits", t("board.city_energy_credits")),
            "producing (no blackout, no agent on it): gain its energy, once per city a round".into(),
        ),
        // 204 (Matt, 2026-11-12): the one standard source, a desperate one; the engines (the plants, r97) are the
        // real supply.
        StandardAction::Antimatter => (
            "Make antimatter",
            format!("{} research cubes", t("board.antimatter_cubes")),
            "gain 1 antimatter, as often as you have cubes (a desperate trade: the engines make it cheaper)".into(),
        ),
        // 206 (Matt, 2026-11-12): Expand, Buy In, Accord and Peer Review were one power under four names.
        StandardAction::Node => (
            "Place a node",
            if w.int("board.node_energy") > 0 {
                format!("{} credits and {} energy, and the track step's energy", t("board.node_credits"), t("board.node_energy"))
            } else {
                format!("{} credits, and the track step's energy", t("board.node_credits"))
            },
            format!(
                "place 1 node next to yours, off either track, at most {} a round (the boards' own node powers are the better route)",
                t("board.node_per_round")
            ),
        ),
        // 206 (amendment 9247): a hedge, never the better route.
        StandardAction::Troop => (
            "Remove a troop",
            format!("{} credits", t("board.troop_credits")),
            format!("remove 1 troop on it, at most {} a round", t("board.troop_per_round")),
        ),
        // 206 (amendment 9212, candidate).
        StandardAction::Factory => (
            "Build a factory",
            format!("{} chips", t("board.factory_build_chips")),
            "a factory goes on it; it is yours (your cube)".into(),
        ),
    };
    let what = match std_target(act) {
        Some(on) => {
            let mut c = what.chars();
            let first = c.next().map(|f| f.to_uppercase().collect::<String>()).unwrap_or_default();
            format!("Target: {on}. {first}{}", c.as_str())
        }
        None => what,
    };
    [name.to_string(), cost, what]
}

/// Where seat `s` may build a research station: an open hex in reach that is
/// not an industrial zone.
pub fn station_sites(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len())
        .filter(|&h| buildable(w, g, h) && !w.map.hexes[h].industrial && in_reach(w, g, seat, h))
        .collect()
}

/// Whether seat `s` can claim neutral site `h` now: a site claim on it and
/// the ambassadors in its supply (137).
pub fn claimable(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    target_price(w, g, seat, StandardAction::Claim, h).is_some_and(|p| p.affordable(g, seat)) && can_claim(w, g, seat, h)
}

/// Whether seat `s` can liberate adversary site `h` now: in its reach (a
/// botnet doesn't stop it: the setup station holds one) and the ambassadors
/// in its supply (137).
pub fn liberable(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    target_price(w, g, seat, StandardAction::Liberate, h).is_some_and(|p| p.affordable(g, seat)) && in_reach(w, g, seat, h)
}

/// The Fallout tokens seat `seat` can lift: any in its reach (210's Lift, 212's card bonus).
pub fn land_in_reach(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len()).filter(|&h| in_reach(w, g, seat, h) && g.tokens(h, FALLOUT) > 0).collect()
}

/// The hexes a standard action of `seat` may target (empty: not offered).
pub(super) fn std_targets(w: &World, g: &GameState, seat: usize, act: StandardAction) -> Vec<usize> {
    let reach = |h: usize| in_reach(w, g, seat, h);
    let all = 0..g.hexes.len();
    if cut(w, act) {
        return Vec::new();
    }
    match act {
        StandardAction::Station => station_sites(w, g, seat),
        StandardAction::Remove => all.filter(|&h| reach(h) && g.tokens(h, ADV_AMBASSADOR) > 0).collect(),
        StandardAction::Lift => land_in_reach(w, g, seat),
        // Onto a piece in reach, from the supply or another hex: never onto the
        // hex the seat's only ambassador already holds (020: that charged the
        // energy and moved nothing).
        StandardAction::Move if moves_left(w, g, seat) <= 0 => Vec::new(),
        StandardAction::Move => {
            let supply = g.resource(seat, Resource::Ambassadors) > 0;
            let on_board = ambassadors_on_board(g, seat);
            all.filter(|&h| {
                // r75: a botnet's hex too, where an ambassador takes it down.
                (supply || on_board > g.tokens(h, ambassador(seat)))
                    && reach(h)
                    && (has_piece(w, g, h) || g.tokens(h, BOTNET) > 0)
            })
            .collect()
        }
        StandardAction::Claim => all.filter(|&h| claimable(w, g, seat, h)).collect(),
        StandardAction::Liberate => all.filter(|&h| liberable(w, g, seat, h)).collect(),
        // Gaining an ambassador, credits or insight needs no target.
        StandardAction::Ambassador => vec![usize::MAX],
        StandardAction::Insight if w.int("board.cube_insight") > 0 => vec![usize::MAX],
        StandardAction::Insight => Vec::new(),
        // 136: with no Discord on the track it would do nothing.
        StandardAction::Envoy if w.int("board.envoy_ambassadors") > 0 && g.var(alignment::track(Side::Discord)) > 0 => {
            vec![usize::MAX]
        }
        StandardAction::Envoy => Vec::new(),
        StandardAction::Botnet => botnet_targets(w, g, seat),
        StandardAction::BuyEnergy => all.filter(|&h| energy_for_sale(w, g, seat, h)).collect(),
        StandardAction::Antimatter if w.int("board.antimatter_cubes") > 0 => vec![usize::MAX],
        StandardAction::Antimatter => Vec::new(),
        StandardAction::Node | StandardAction::Troop | StandardAction::Factory => {
            // 247: a node's own energy is paid first, so its track step is paid from what is left.
            let step_paid = act != StandardAction::Node || node::can_pay_placement(w, g, seat, -printed_price(w, act).energy);
            if step_paid
                && per_round(w, act).is_none_or(|n| g.var(&std_once_key(act, seat)) < n)
                && effects::feasible(w, g, seat, std_ops(act))
            {
                vec![usize::MAX]
            } else {
                Vec::new()
            }
        }
    }
}

/// Remove agents' option to stop short of `remove_count`.
pub const STOP_REMOVING: &str = "stop removing";

/// The hexes on or next to Remove agents' target `t` holding agents (its picks after the first, 236).
fn agent_hexes_near(w: &World, g: &GameState, t: usize) -> Vec<usize> {
    let mut near = vec![t];
    near.extend(w.map.hexes[t].neighbors.iter().copied());
    near.sort_unstable();
    near.into_iter().filter(|&h| g.tokens(h, ADV_AMBASSADOR) > 0).collect()
}

/// A standard action's effect, paid for and tallied; `targets` are its
/// [`std_targets`].
pub(super) fn std_resolve(
    g: &mut GameState,
    cx: &mut Cx,
    s: usize,
    act: StandardAction,
    targets: &[usize],
) -> Res {
    let w = cx.world;
    let on_hex = |cx: &mut Cx| -> Res<Option<usize>> {
        let opts = targets.iter().map(|&h| hex_opt(&w, h)).collect();
        // 184: the printed action's words, not its key (Dispatch's key is still "move", 178).
        let prompt = || match act {
            StandardAction::Move => format!("Seat {s}: dispatch an ambassador to which hex?"),
            StandardAction::Botnet => format!("Seat {s}: take down a botnet on which hex?"),
            StandardAction::BuyEnergy => format!("Seat {s}: buy energy from which city?"),
            StandardAction::Station => format!("Seat {s}: build a research station on which hex?"),
            // 236: Remove agents names its target, the cards' prompt (the agents' target rule answers it).
            StandardAction::Remove => format!("Seat {s}: {} (Agents) [Remove agents]", effects::TARGET_PROMPT),
            StandardAction::Lift => format!("Seat {s}: lift Fallout on which hex?"),
            _ => format!("Seat {s}: {} on which hex?", act.name().split('_').next().unwrap_or_default()),
        };
        Ok(cx.choose(s, prompt, opts)?.map(|i| targets[i]))
    };
    match act {
        StandardAction::Ambassador => cx.add_resource(g, s, Resource::Ambassadors, 1),
        StandardAction::Insight => {
            cx.add_resource(g, s, Resource::Insight, w.int("board.cube_insight"));
            cx.tally(g, "board.cubes_converted", 1);
        }
        StandardAction::Envoy => {
            alignment::hold_off(g, cx, 1);
        }
        StandardAction::Antimatter => {
            cx.add_resource(g, s, Resource::Antimatter, 1);
            cx.tally(g, "board.cubes_to_antimatter", 1);
        }
        StandardAction::Station => {
            if let Some(h) = on_hex(cx)? {
                cx.add_tokens(g, h, STATION, 1);
                cx.add_tokens(g, h, cube(s), 1);
            }
        }
        StandardAction::Remove => {
            // r75 (Matt's ruling 1): the ambassadors paid are used up and nothing is placed; up to
            // `remove_count` agents come off, one at a time (the player may stop). 236: the first on the target, the
            // rest on or next to it (it took them from any hexes in reach).
            if let Some(h) = on_hex(cx)? {
                adversary::remove_piece(g, cx, Some(s), h, ADV_AMBASSADOR);
                // 236: what one use finds, beside what its reach held (the pricing read).
                let held: i64 = (0..g.hexes.len()).filter(|&x| in_reach(&w, g, s, x)).map(|x| g.tokens(x, ADV_AMBASSADOR)).sum();
                cx.tally(g, "board.remove_agents_in_reach", (held + 1).min(w.int("board.remove_count")));
                let mut removed = 1;
                for _ in 1..w.int("board.remove_count") {
                    let hexes = agent_hexes_near(&w, g, h);
                    if hexes.is_empty() {
                        break;
                    }
                    let mut opts: Vec<crate::state::Opt> = hexes.iter().map(|&x| hex_opt(&w, x)).collect();
                    opts.push(opt(STOP_REMOVING));
                    let prompt = || format!("Seat {s}: remove another agent, from which hex?");
                    let Some(&x) = cx.choose(s, prompt, opts)?.and_then(|i| hexes.get(i)) else { break };
                    adversary::remove_piece(g, cx, Some(s), x, ADV_AMBASSADOR);
                    removed += 1;
                }
                cx.tally(g, "board.remove_agents_removed", removed);
            }
        }
        StandardAction::Botnet => {
            if let Some(h) = on_hex(cx)? {
                // r75 (Matt's ruling 3): the ambassador stays; the botnet's Unity is `alignment.botnet`.
                adversary::remove_botnet(g, cx, h);
                cx.tally(g, "board.botnets_taken_down", 1);
            }
        }
        StandardAction::BuyEnergy => {
            if let Some(h) = on_hex(cx)? {
                let n = city_output(&w, h);
                cx.set_key(g, &bought_key(h), 1);
                cx.add_resource(g, s, Resource::Energy, n);
                cx.tally(g, "board.city_energy_bought", n);
            }
        }
        StandardAction::Lift => {
            if let Some(h) = on_hex(cx)? {
                adversary::lift_fallout(g, cx, h);
                cx.tally(g, "board.land_lifted", 1);
            }
        }
        StandardAction::Move => {
            if let Some(h) = on_hex(cx)? {
                cx.add_key(g, &moved_key(s), 1);
                move_ambassador(g, cx, s, h)?;
            }
        }
        StandardAction::Claim => {
            if let Some(h) = on_hex(cx)? {
                // r75: the seat's ambassadors on the site first, its supply the rest.
                pay_site(g, cx, s, h, false);
                claim_site(g, cx, s, h, "ambassadors");
            }
        }
        StandardAction::Liberate => {
            if let Some(h) = on_hex(cx)? {
                pay_site(g, cx, s, h, true);
                liberate_site(g, cx, s, h);
            }
        }
        StandardAction::Node | StandardAction::Troop | StandardAction::Factory => {
            if per_round(&w, act).is_some() {
                cx.add_key(g, &std_once_key(act, s), 1);
            }
            effects::run(g, cx, std_ops(act), &mut effects::Ctx::seat(s).named("standard"))?;
        }
    }
    Ok(())
}

/// "once", "twice", "3 times".
fn times(n: i64) -> String {
    match n {
        1 => "once".into(),
        2 => "twice".into(),
        _ => format!("{n} times"),
    }
}


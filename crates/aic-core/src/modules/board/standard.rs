//! The standard actions and projects: what each costs, targets and does.

use super::*;

/// A standard action. `Action::Standard` carries its [`name`](Self::name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardAction {
    Station,
    Remove,
    /// 210 (Matt, candidate): research cubes lift 1 land-corruption token in reach, the one standard action that
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
    /// 066: the Harden action a card grants, paid in chips.
    HardenChips,
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
    /// 204: the standard antimatter spend, a shared breakthrough: antimatter for Unity (0: off).
    Breakthrough,
    /// 206 (Matt, 2026-11-12): the one node placement every board has, poorly priced (Expand left every board).
    Node,
    /// 206 (amendment 9247): a very expensive hedge against troops: credits for 1 troop in reach.
    Troop,
    /// 206 (amendment 9212, candidate): a factory, yours, for chips (`factory_build_chips` 0: off).
    Factory,
}

/// The standard actions, in the order they are offered and published.
pub const STANDARD: [StandardAction; 17] = [
    StandardAction::Station,
    StandardAction::Remove,
    StandardAction::Lift,
    StandardAction::Move,
    StandardAction::Ambassador,
    StandardAction::Claim,
    StandardAction::Liberate,
    StandardAction::HardenChips,
    StandardAction::Insight,
    StandardAction::Envoy,
    StandardAction::Botnet,
    StandardAction::BuyEnergy,
    StandardAction::Antimatter,
    StandardAction::Breakthrough,
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
            StandardAction::HardenChips => "harden_chips",
            StandardAction::Insight => "insight",
            StandardAction::Envoy => "envoy",
            StandardAction::Botnet => "botnet",
            StandardAction::BuyEnergy => "buy_energy",
            StandardAction::Antimatter => "antimatter",
            StandardAction::Breakthrough => "breakthrough",
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
    /// 204: antimatter (Breakthrough).
    pub antimatter: i64,
}

impl Price {
    /// What it charges, in print order.
    fn parts(self) -> [(i64, Resource); 6] {
        [
            (self.energy, Resource::Energy),
            (self.credits, Resource::Credits),
            (self.chips, Resource::Chips),
            (self.ambassadors, Resource::Ambassadors),
            (self.cubes, Resource::ResearchCubes),
            (self.antimatter, Resource::Antimatter),
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
pub(super) fn printed_price(w: &World, act: StandardAction) -> Price {
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
        // 080: the price is the card's (`seat_price`); none printed here.
        StandardAction::HardenChips => Price::default(),
        StandardAction::Insight => Price { cubes: 1, ..Price::default() },
        StandardAction::Envoy => ambassadors(i("envoy_ambassadors")),
        StandardAction::Botnet => credits(i("botnet_credits")),
        StandardAction::BuyEnergy => credits(i("city_energy_credits")),
        StandardAction::Antimatter => Price { cubes: i("antimatter_cubes"), ..Price::default() },
        StandardAction::Breakthrough => Price { antimatter: i("breakthrough_antimatter"), ..Price::default() },
        StandardAction::Node => credits(i("node_credits")),
        StandardAction::Troop => credits(i("troop_credits")),
        StandardAction::Factory => chips(i("factory_build_chips")),
    }
}

/// The chips seat `s` pays for the Harden action its cards grant (080: printed
/// on the card; the cheapest if several): `None` with no such card.
pub fn harden_card_chips(w: &World, g: &GameState, seat: usize) -> Option<i64> {
    let mut best: Option<i64> = None;
    cards::each_passive(w, g, seat, |p| {
        if let Op::HardenAction { chips } = p {
            best = Some(best.map_or(*chips, |b| b.min(*chips)));
        }
    });
    best
}

/// A standard action's price for seat `s`: the printed price, or the card's
/// for the Harden action a card grants (080).
pub fn seat_price(w: &World, g: &GameState, seat: usize, act: StandardAction) -> Price {
    match (act, harden_card_chips(w, g, seat)) {
        (StandardAction::HardenChips, Some(chips)) => Price { chips, ..Price::default() },
        _ => printed_price(w, act),
    }
}

/// The part of seat `seat`'s standard action's price that depends on its target hex `h`
/// (137; 140's hook): Claim's and Liberate's ambassadors from the supply, the site's
/// `claim_cost` or `liberate_cost` less the seat's own ambassadors standing on it (r75's one payment rule;
/// `None`: no such site there). The rest are priced whole by [`seat_price`] and add nothing here. The
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

/// 204: set while seat `s` has taken Breakthrough this round (once a round), a round mark.
pub fn broke_through_key(seat: usize) -> KeyBuf {
    KeyBuf::new(BROKE_THROUGH).num(seat)
}

/// Prefix of [`broke_through_key`].
pub(super) const BROKE_THROUGH: &str = "board.broke_through.";

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

/// Takes the corruption marker off hex `h` (the action or a card), for its
/// reward (041): `clear_unity` Sway toward Unity and `clear_credits` to `seat`.
pub fn clear_marker(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let w = cx.world;
    cx.add_tokens(g, h, MARKER, -1);
    cx.tally(g, "board.markers_removed", 1);
    if is_city(&w, h) {
        cx.tally(g, "board.markers_removed_city", 1);
    }
    alignment::unity_from(g, cx, "clear", w.int("board.clear_unity"));
    cx.add_resource(g, seat, Resource::Credits, w.int("board.clear_credits"));
}

/// 206: the ops an op-backed standard action runs (node placement, troop removal, factory building), so its
/// targets and effect are the cards' own (`effects::feasible`, `effects::run`).
pub fn std_ops(act: StandardAction) -> &'static [Op] {
    match act {
        StandardAction::Node => &[Op::PlaceNode { count: 1 }],
        StandardAction::Troop => &[Op::RemoveTroops { count: 1, reach: true }],
        StandardAction::Factory => &[Op::BuildFactory],
        _ => &[],
    }
}

/// 206: a standard action whose price param is 0 is off (Lift, Node, Troop, Factory). 210 (P2) deleted the cut Clear,
/// Drain and Harden project.
fn cut(w: &World, act: StandardAction) -> bool {
    let k = match act {
        StandardAction::Lift => "land_cubes",
        StandardAction::Node => "node_credits",
        StandardAction::Troop => "troop_credits",
        StandardAction::Factory => "factory_build_chips",
        _ => return false,
    };
    w.int(&format!("board.{k}")) <= 0
}

/// Whether standard action `act` is in the rules (206: a cut one prints no row and is never offered).
pub fn std_offered(w: &World, act: StandardAction) -> bool {
    !cut(w, act)
}

/// What a standard action does, for the view.
pub(super) fn std_text(w: &World, act: StandardAction) -> String {
    let text = match act {
        StandardAction::Lift => "lift 1 land-corruption token in reach",
        StandardAction::Station => "build a research station in reach (yours)",
        StandardAction::Remove => {
            let n = w.int("board.remove_ambassadors");
            let who = if n == 1 { "an ambassador".to_string() } else { format!("{n} ambassadors") };
            return format!("spend {who} from your supply: remove up to {} agents in reach", w.int("board.remove_count"));
        }
        // 178 (Matt's ruling #22): Dispatch, not Move (it plays the same: the ambassador comes from anywhere).
        StandardAction::Move => "use 1 Dispatch: put a player ambassador, from your supply or the board, onto a piece or botnet in reach",
        StandardAction::Ambassador => "gain a player ambassador",
        StandardAction::Claim => {
            return format!(
                "claim a neutral site in reach: {} ambassadors for a city, {} for a station, {} for a factory, plus 1 for each agent on it; yours on it pay first",
                claim_ambassadors(w, SiteKind::City),
                claim_ambassadors(w, SiteKind::Station),
                claim_ambassadors(w, SiteKind::Factory)
            );
        }
        StandardAction::Liberate => {
            return format!(
                "liberate an adversary site in reach: {} ambassadors for a city, {} for a station or factory, plus 1 for each agent on it; yours on it pay first; it becomes yours",
                liberate_ambassadors(w, SiteKind::City),
                liberate_ambassadors(w, SiteKind::Station)
            );
        }
        StandardAction::HardenChips => "the Harden action a card grants: harden a facility in reach",
        StandardAction::Insight => {
            return format!("turn 1 research cube into {}", Resource::Insight.words(w.int("board.cube_insight")));
        }
        StandardAction::Envoy => "move 1 Discord off the Discord track",
        StandardAction::Botnet => "take down a botnet in reach on a hex holding your ambassador (it stays): Unity",
        StandardAction::BuyEnergy => {
            return format!("buy a neutral city's energy (in reach, producing; once per city a round): {} credits", w.int("board.city_energy_credits"));
        }
        StandardAction::Antimatter => {
            return format!("turn {} into 1 antimatter", Resource::ResearchCubes.words(w.int("board.antimatter_cubes")));
        }
        StandardAction::Breakthrough => {
            return format!(
                "a shared breakthrough: {} for {} Sway toward Unity",
                Resource::Antimatter.words(w.int("board.breakthrough_antimatter")),
                w.int("board.breakthrough_unity")
            );
        }
        StandardAction::Node => {
            return format!("place 1 node next to yours (paying its track step), {} a round", times(w.int("board.node_per_round")));
        }
        StandardAction::Troop => {
            return format!("remove 1 troop in reach, {} a round", times(w.int("board.troop_per_round")));
        }
        StandardAction::Factory => "build a factory on a free industrial zone in reach (yours)",
    };
    text.to_string()
}

/// What a standard action does, in the rule book's longer words: its name,
/// cost and effect, the row `aic-sim docs` prints in docs/rules/economy.md
/// (121: one home for them, beside the view's [`std_text`]). `t` prints a
/// tunable by its key (the docs print `` `remove_count` 3 ``).
pub fn std_rule_text(act: StandardAction, t: &dyn Fn(&str) -> String) -> [String; 3] {
    let (name, cost, what): (&str, String, String) = match act {
        StandardAction::Station => (
            "Build a research station",
            format!("{} chips (and {} ambassadors, used up: a candidate, 0 none)", t("board.station_chips"), t("board.station_ambassadors")),
            "on an open hex in reach that is not an industrial zone or corrupted land; it is yours (your cube)".into(),
        ),
        // r75 (Matt's ruling 1): it places nothing (it stood on the hex and removed up to 3 there).
        StandardAction::Remove => (
            "Remove agents",
            format!("{} of your ambassadors, from your supply, used up", t("board.remove_ambassadors")),
            format!("remove up to {} agents in reach, from any hexes, your choice", t("board.remove_count")),
        ),
        // 210 (Matt, candidate: "costly but not absurd").
        StandardAction::Lift => (
            "Lift land corruption",
            format!("{} research cubes", t("board.land_cubes")),
            "a land-corruption token in reach, face up or face down, comes off: the one standard action that removes corruption".into(),
        ),
        StandardAction::Move => (
            "Dispatch a player ambassador",
            "1 of your Dispatches this round (your board's, unused ones lost)".into(),
            "from your supply or the board onto a piece or botnet in reach (not the hex it comes from)".into(),
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
            "a neutral city, research station or factory in reach with no agent on it (no embargo on it): your cube goes on".into(),
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
                "an adversary city, station or factory in reach with no agent on it: the adversary's cube comes off, yours goes on, and it sways Unity ({} for a city, {} for a station or factory)",
                t("alignment.free_city"),
                t("alignment.free_facility")
            ),
        ),
        StandardAction::HardenChips => (
            "Harden a facility (a card that grants it; none does: Faraday Yard hardens when played)",
            "the chips the card prints, per token".into(),
            "a player-owned station or factory in reach that is not corrupted, one a troop can strike".into(),
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
            format!("a botnet on a hex holding your ambassador (it stays) comes off: sway {} toward Unity", t("alignment.botnet")),
        ),
        // r75 (Matt's ruling 6): it was a sale in production.
        StandardAction::BuyEnergy => (
            "Buy a city's energy",
            format!("{} credits", t("board.city_energy_credits")),
            "a neutral city in reach that is producing (no blackout, no agent on it), once per city a round: gain its energy".into(),
        ),
        // 204 (Matt, 2026-11-12): the one standard source, a desperate one; the engines (plants, the Builder's
        // Forge, Particle Run, Tidal Grid) are the real supply.
        StandardAction::Antimatter => (
            "Make antimatter",
            format!("{} research cubes", t("board.antimatter_cubes")),
            "gain 1 antimatter, as often as you have cubes (a desperate trade: the engines make it cheaper)".into(),
        ),
        // 204: the standard antimatter spend, off unless `breakthrough_unity` is above 0.
        StandardAction::Breakthrough => (
            "Breakthrough",
            format!("{} antimatter", t("board.breakthrough_antimatter")),
            format!("sway {} toward Unity, once a round (`breakthrough_unity` 0: no such action)", t("board.breakthrough_unity")),
        ),
        // 206 (Matt, 2026-11-12): Expand, Buy In, Accord and Peer Review were one power under four names.
        StandardAction::Node => (
            "Place a node",
            format!("{} credits, and the track step's energy", t("board.node_credits")),
            format!(
                "place 1 node next to yours, off either track, up to {} times a round (the boards' own node powers are the better route)",
                t("board.node_per_round")
            ),
        ),
        // 206 (amendment 9247): a hedge, never the better route.
        StandardAction::Troop => (
            "Remove a troop",
            format!("{} credits", t("board.troop_credits")),
            format!("remove 1 adversary troop in reach, up to {} times a round", t("board.troop_per_round")),
        ),
        // 206 (amendment 9212, candidate).
        StandardAction::Factory => (
            "Build a factory",
            format!("{} chips", t("board.factory_build_chips")),
            "on a free industrial zone in reach; it is yours (your cube)".into(),
        ),
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
        StandardAction::Lift => all.filter(|&h| reach(h) && g.tokens(h, LAND_CORRUPTION) > 0).collect(),
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
        StandardAction::HardenChips => {
            if harden_card_chips(w, g, seat).is_some() {
                all.filter(|&h| reach(h) && hardenable(g, h)).collect()
            } else {
                Vec::new()
            }
        }
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
        StandardAction::Breakthrough
            if w.int("board.breakthrough_unity") > 0
                && g.var(&broke_through_key(seat)) == 0
                && w.variant("alignment") != Some("off") =>
        {
            vec![usize::MAX]
        }
        StandardAction::Breakthrough => Vec::new(),
        StandardAction::Node | StandardAction::Troop | StandardAction::Factory => {
            if per_round(w, act).is_none_or(|n| g.var(&std_once_key(act, seat)) < n) && effects::feasible(w, g, seat, std_ops(act)) {
                vec![usize::MAX]
            } else {
                Vec::new()
            }
        }
    }
}

/// Remove agents' option to stop short of `remove_count`.
pub const STOP_REMOVING: &str = "stop removing";

/// The hexes in seat `seat`'s reach holding agents (Remove agents' picks).
fn agent_hexes(w: &World, g: &GameState, seat: usize) -> Vec<usize> {
    (0..g.hexes.len()).filter(|&h| in_reach(w, g, seat, h) && g.tokens(h, ADV_AMBASSADOR) > 0).collect()
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
            StandardAction::Remove => format!("Seat {s}: remove an agent, from which hex?"),
            StandardAction::Lift => format!("Seat {s}: lift land corruption on which hex?"),
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
        StandardAction::Breakthrough => {
            cx.set_key(g, &broke_through_key(s), 1);
            alignment::unity_from(g, cx, "breakthrough", w.int("board.breakthrough_unity"));
        }
        StandardAction::Station => {
            if let Some(h) = on_hex(cx)? {
                cx.add_tokens(g, h, STATION, 1);
                cx.add_tokens(g, h, cube(s), 1);
            }
        }
        StandardAction::Remove => {
            // r75 (Matt's ruling 1): the ambassadors paid are used up and nothing is placed; up to
            // `remove_count` agents come off, one at a time, from any hexes in reach (the player may stop).
            if let Some(h) = on_hex(cx)? {
                adversary::remove_piece(g, cx, Some(s), h, ADV_AMBASSADOR);
                for _ in 1..w.int("board.remove_count") {
                    let hexes = agent_hexes(&w, g, s);
                    if hexes.is_empty() {
                        break;
                    }
                    let mut opts: Vec<crate::state::Opt> = hexes.iter().map(|&x| hex_opt(&w, x)).collect();
                    opts.push(opt(STOP_REMOVING));
                    let prompt = || format!("Seat {s}: remove another agent, from which hex?");
                    let Some(&x) = cx.choose(s, prompt, opts)?.and_then(|i| hexes.get(i)) else { break };
                    adversary::remove_piece(g, cx, Some(s), x, ADV_AMBASSADOR);
                }
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
                adversary::lift_land_corruption(g, cx, h);
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
        StandardAction::HardenChips => {
            if let Some(h) = on_hex(cx)? {
                cx.add_tokens(g, h, HARDEN, 1);
            }
        }
        StandardAction::Node | StandardAction::Troop | StandardAction::Factory => {
            if per_round(&w, act).is_some() {
                cx.add_key(g, &std_once_key(act, s), 1);
            }
            effects::run(g, cx, std_ops(act), &mut effects::Ctx::seat(s))?;
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


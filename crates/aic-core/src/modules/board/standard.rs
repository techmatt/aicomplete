//! The standard actions and projects: what each costs, targets and does.

use super::*;

/// A standard action. `Action::Standard` carries its [`name`](Self::name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardAction {
    Station,
    Remove,
    Clear,
    /// 149: a free move from the seat's Movement (the paid Move is gone).
    Move,
    Ambassador,
    /// 137: claim a neutral city, station or factory (it was Align, cities only).
    Claim,
    /// 137: liberate an adversary site with massed ambassadors (it replaced
    /// Free a station and the city fallback).
    Liberate,
    /// 066: the Harden action a card grants, paid in chips.
    HardenChips,
    // 024: the credit sinks, Terraforming Mars' standard projects.
    HardenBuy,
    Drain,
    /// 105: the general converter, a research cube for insight, no limit a round.
    Insight,
    /// 136: spend an ambassador to move 1 Discord off the Discord track.
    Envoy,
}

/// The standard actions, in the order they are offered and published.
pub const STANDARD: [StandardAction; 12] = [
    StandardAction::Station,
    StandardAction::Remove,
    StandardAction::Clear,
    StandardAction::Move,
    StandardAction::Ambassador,
    StandardAction::Claim,
    StandardAction::Liberate,
    StandardAction::HardenChips,
    StandardAction::HardenBuy,
    StandardAction::Drain,
    StandardAction::Insight,
    StandardAction::Envoy,
];

impl StandardAction {
    /// The action's name: `Action::Standard.act`, and its tally `board.std.<name>`.
    pub const fn name(self) -> &'static str {
        match self {
            StandardAction::Station => "station",
            StandardAction::Remove => "remove",
            StandardAction::Clear => "clear",
            StandardAction::Move => "move",
            StandardAction::Ambassador => "ambassador",
            StandardAction::Claim => "claim",
            StandardAction::Liberate => "liberate",
            StandardAction::HardenChips => "harden_chips",
            StandardAction::HardenBuy => "harden_buy",
            StandardAction::Drain => "drain",
            StandardAction::Insight => "insight",
            StandardAction::Envoy => "envoy",
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
pub(super) fn printed_price(w: &World, act: StandardAction) -> Price {
    let i = |k: &str| w.int(&format!("board.{k}"));
    let credits = |credits| Price { credits, ..Price::default() };
    let chips = |chips| Price { chips, ..Price::default() };
    let ambassadors = |ambassadors| Price { ambassadors, ..Price::default() };
    match act {
        StandardAction::Station => chips(i("station_chips")),
        StandardAction::Remove => ambassadors(i("remove_ambassadors")),
        StandardAction::Clear => credits(i("clear_cost")),
        // 149: paid from the seat's Movement, not a resource (`moves_left`).
        StandardAction::Move => Price::default(),
        StandardAction::Ambassador => credits(i("ambassador_credits")),
        // 137: the ambassadors depend on the site (`claim_cost`, `liberate_cost`):
        // the targets list only sites the seat can pay for, and the action pays.
        StandardAction::Claim | StandardAction::Liberate => Price::default(),
        // 080: the price is the card's (`seat_price`); none printed here.
        StandardAction::HardenChips => Price::default(),
        StandardAction::HardenBuy => credits(i("harden_credits")),
        StandardAction::Drain => credits(i("drain_credits")),
        StandardAction::Insight => Price { cubes: 1, ..Price::default() },
        StandardAction::Envoy => ambassadors(i("envoy_ambassadors")),
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

/// The var counting seat `s`'s Movement used this round (149), cleared in cleanup.
pub fn moved_key(seat: usize) -> KeyBuf {
    KeyBuf::new(MOVED).num(seat)
}

/// Prefix of [`moved_key`].
pub(super) const MOVED: &str = "board.moved.";

/// Seat `s`'s Movement left this round (149): its board's [`ai::movement`]
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

/// What a standard action does, for the view.
pub(super) fn std_text(w: &World, act: StandardAction) -> String {
    let text = match act {
        // 044: Clear's words say what its reward is.
        StandardAction::Clear => return clear_words(w),
        StandardAction::Station => "build a research station in reach (yours)",
        StandardAction::Remove => {
            let n = w.int("board.remove_ambassadors");
            let who = if n == 1 { "an ambassador".to_string() } else { format!("{n} ambassadors") };
            return format!("move {who} from your supply onto a hex in reach: it removes up to {} adversary ambassadors there and stays", w.int("board.remove_count"));
        }
        StandardAction::Move => "use 1 Movement: move a player ambassador onto a piece in reach",
        StandardAction::Ambassador => "gain a player ambassador",
        StandardAction::Claim => {
            return format!(
                "claim a neutral site you can claim: {} ambassadors for a city, {} for a station, {} for a factory, plus 1 for each adversary ambassador on it",
                claim_ambassadors(w, SiteKind::City),
                claim_ambassadors(w, SiteKind::Station),
                claim_ambassadors(w, SiteKind::Factory)
            );
        }
        StandardAction::Liberate => {
            return format!(
                "liberate an adversary site in reach: {} ambassadors for a city, {} for a station or factory, plus 1 for each adversary ambassador on it; it turns neutral",
                liberate_ambassadors(w, SiteKind::City),
                liberate_ambassadors(w, SiteKind::Station)
            );
        }
        StandardAction::HardenChips => "the Harden action a card grants: harden a facility in reach",
        StandardAction::HardenBuy => "standard project: harden a facility in reach",
        StandardAction::Drain => "standard project: remove compute from an adversary project",
        StandardAction::Insight => {
            return format!("turn 1 research cube into {}", Resource::Insight.words(w.int("board.cube_insight")));
        }
        StandardAction::Envoy => "move 1 Discord off the Discord track",
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
            format!("{} chips", t("board.station_chips")),
            "on an open hex in reach that is not an industrial zone; it is yours (your cube)".into(),
        ),
        StandardAction::Remove => (
            "Remove adversary ambassadors",
            format!("{} of your ambassadors, from your supply", t("board.remove_ambassadors")),
            format!("it goes onto one hex in reach, removes up to {} there and stays", t("board.remove_count")),
        ),
        StandardAction::Clear => (
            "Clear a corruption marker",
            format!("{} credits", t("board.clear_cost")),
            format!("in reach: sway {} toward Unity, gain {} credits", t("board.clear_unity"), t("board.clear_credits")),
        ),
        StandardAction::Move => (
            "Move a player ambassador",
            "1 of your Movement this round (your board's, unused ones lost)".into(),
            "from your supply or the board onto a piece in reach (not the hex it comes from)".into(),
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
                "{} ambassadors for a city, {} for a station, {} for a factory, from your supply, used up; 1 more for each adversary ambassador on it",
                t("board.claim_city"),
                t("board.claim_station"),
                t("board.claim_factory")
            ),
            "a neutral city, research station or factory in reach, with a site claim: the adversary ambassadors on it cancel and your cube goes on".into(),
        ),
        // 137: it replaced Free a station and the city fallback.
        StandardAction::Liberate => (
            "Liberate a site",
            format!(
                "{} ambassadors for a city, {} for a station or factory, from your supply, used up; 1 more for each adversary ambassador on it",
                t("board.liberate_at"),
                t("board.liberate_facility_at")
            ),
            format!(
                "an adversary city, station or factory in reach: it turns neutral and sways Unity ({} for a city, {} for a station or factory); a city takes {} adversary ambassadors as it frees",
                t("alignment.free_city"),
                t("alignment.free_facility"),
                t("board.free_leaves")
            ),
        ),
        StandardAction::HardenChips => (
            "Harden a facility (a card that grants it; none does: Faraday Yard hardens when played)",
            "the chips the card prints, per token".into(),
            "a player-owned station or factory in reach that is not corrupted, one a troop can strike".into(),
        ),
        StandardAction::HardenBuy => (
            "Project: harden (`harden_buy`)",
            format!("{} credits", t("board.harden_credits")),
            "any player: a hardening token on a player-owned station or factory in reach".into(),
        ),
        StandardAction::Drain => (
            "Project: drain (`drain`)",
            format!("{} credits", t("board.drain_credits")),
            format!("remove up to {} compute from one visible adversary project", t("board.drain_amount")),
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
    claim_cost(w, g, h).is_some_and(|n| g.resource(seat, Resource::Ambassadors) >= n) && can_claim(w, g, seat, h)
}

/// Whether seat `s` can liberate adversary site `h` now: in its reach (a
/// botnet doesn't stop it: the setup station holds one) and the ambassadors
/// in its supply (137).
pub fn liberable(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    liberate_cost(w, g, h).is_some_and(|n| g.resource(seat, Resource::Ambassadors) >= n) && in_reach(w, g, seat, h)
}

/// The hexes a standard action of `seat` may target (empty: not offered).
pub(super) fn std_targets(w: &World, g: &GameState, seat: usize, act: StandardAction) -> Vec<usize> {
    let reach = |h: usize| in_reach(w, g, seat, h);
    let all = 0..g.hexes.len();
    match act {
        StandardAction::Station => station_sites(w, g, seat),
        StandardAction::Remove => all.filter(|&h| reach(h) && g.tokens(h, ADV_AMBASSADOR) > 0).collect(),
        StandardAction::Clear => all.filter(|&h| reach(h) && g.tokens(h, MARKER) > 0).collect(),
        // Onto a piece in reach, from the supply or another hex: never onto the
        // hex the seat's only ambassador already holds (020: that charged the
        // energy and moved nothing).
        StandardAction::Move if moves_left(w, g, seat) <= 0 => Vec::new(),
        StandardAction::Move => {
            let supply = g.resource(seat, Resource::Ambassadors) > 0;
            let on_board = ambassadors_on_board(g, seat);
            all.filter(|&h| {
                (supply || on_board > g.tokens(h, ambassador(seat)))
                    && reach(h)
                    && has_piece(w, g, h)
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
        StandardAction::HardenBuy => all.filter(|&h| reach(h) && hardenable(g, h)).collect(),
        StandardAction::Drain => {
            if !adversary::projects_with_compute(g).is_empty() {
                vec![usize::MAX]
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
    }
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
        let verb = act.name().split('_').next().unwrap_or_default();
        let prompt = || format!("Seat {s}: {verb} on which hex?");
        Ok(cx.choose(s, prompt, opts)?.map(|i| targets[i]))
    };
    match act {
        StandardAction::Ambassador => cx.add_resource(g, s, Resource::Ambassadors, 1),
        StandardAction::Drain => adversary::choose_drain(g, cx, s, w.int("board.drain_amount"))?,
        StandardAction::Insight => {
            cx.add_resource(g, s, Resource::Insight, w.int("board.cube_insight"));
            cx.tally(g, "board.cubes_converted", 1);
        }
        StandardAction::Envoy => {
            alignment::hold_off(g, cx, 1);
        }
        StandardAction::Station => {
            if let Some(h) = on_hex(cx)? {
                cx.add_tokens(g, h, STATION, 1);
                cx.add_tokens(g, h, cube(s), 1);
            }
        }
        StandardAction::Remove => {
            if let Some(h) = on_hex(cx)? {
                // 066: `remove_ambassadors` clear up to `remove_count` from the hex.
                for _ in 0..w.int("board.remove_count").min(g.tokens(h, ADV_AMBASSADOR)) {
                    adversary::remove_piece(g, cx, Some(s), h, ADV_AMBASSADOR);
                }
                // 068: it left the supply for the hex and stays there: a
                // position. It evicted them, so it arrives with no cancel.
                cx.add_tokens(g, h, ambassador(s), w.int("board.remove_ambassadors"));
            }
        }
        StandardAction::Clear => {
            if let Some(h) = on_hex(cx)? {
                clear_marker(g, cx, s, h);
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
                let n = claim_cost(&w, g, h).unwrap_or(0);
                cx.add_resource(g, s, Resource::Ambassadors, -n);
                claim_site(g, cx, s, h, "ambassadors");
            }
        }
        StandardAction::Liberate => {
            if let Some(h) = on_hex(cx)? {
                let n = liberate_cost(&w, g, h).unwrap_or(0);
                cx.add_resource(g, s, Resource::Ambassadors, -n);
                liberate_site(g, cx, h);
            }
        }
        StandardAction::HardenChips | StandardAction::HardenBuy => {
            if let Some(h) = on_hex(cx)? {
                cx.add_tokens(g, h, HARDEN, 1);
            }
        }
    }
    Ok(())
}

/// Clear's words with its reward (the configs' `clear_unity`, `clear_credits`).
fn clear_words(w: &World) -> String {
    let (unity, credits) = (w.int("board.clear_unity"), w.int("board.clear_credits"));
    let mut reward = Vec::new();
    if unity > 0 {
        reward.push(format!("{unity} Sway toward Unity"));
    }
    if credits > 0 {
        reward.push(Resource::Credits.words(credits));
    }
    if reward.is_empty() {
        "clear a corruption marker in reach".into()
    } else {
        format!("clear a corruption marker in reach: {}", reward.join(" and "))
    }
}

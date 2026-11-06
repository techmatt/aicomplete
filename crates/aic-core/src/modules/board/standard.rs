//! The standard actions and projects: what each costs, targets and does.

use super::*;

/// A standard action. `Action::Standard` carries its [`name`](Self::name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardAction {
    Station,
    Remove,
    Clear,
    Move,
    Ambassador,
    Align,
    /// r35: free a corrupted research station your ambassadors stand on.
    FreeStation,
    /// 066: the Harden action a card grants, paid in chips.
    HardenChips,
    Chips,
    // 024: the credit sinks, Terraforming Mars' standard projects.
    HardenBuy,
    Drain,
    /// 105: the general converter, a research cube for insight, no limit a round.
    Insight,
}

/// The standard actions, in the order they are offered and published.
pub const STANDARD: [StandardAction; 12] = [
    StandardAction::Station,
    StandardAction::Remove,
    StandardAction::Clear,
    StandardAction::Move,
    StandardAction::Ambassador,
    StandardAction::Align,
    StandardAction::FreeStation,
    StandardAction::HardenChips,
    StandardAction::Chips,
    StandardAction::HardenBuy,
    StandardAction::Drain,
    StandardAction::Insight,
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
            StandardAction::Align => "align",
            StandardAction::FreeStation => "free_station",
            StandardAction::HardenChips => "harden_chips",
            StandardAction::Chips => "chips",
            StandardAction::HardenBuy => "harden_buy",
            StandardAction::Drain => "drain",
            StandardAction::Insight => "insight",
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
        StandardAction::Move => credits(i("move_credits")),
        StandardAction::Ambassador => credits(i("ambassador_credits")),
        StandardAction::Align => ambassadors(1),
        // r35: the ambassadors it uses up stand on the station, not in the supply.
        StandardAction::FreeStation => credits(i("free_station_credits")),
        // 080: the price is the card's (`seat_price`); none printed here.
        StandardAction::HardenChips => Price::default(),
        StandardAction::Chips => chips(1),
        StandardAction::HardenBuy => credits(i("harden_credits")),
        StandardAction::Drain => credits(i("drain_credits")),
        StandardAction::Insight => Price { cubes: 1, ..Price::default() },
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

/// Takes the corruption marker off hex `h` (the action or a card), for its
/// reward (041): `clear_unity` Sway toward Unity and `clear_credits` to `seat`.
pub fn clear_marker(g: &mut GameState, cx: &mut Cx, seat: usize, h: usize) {
    let w = cx.world;
    cx.add_tokens(g, h, MARKER, -1);
    cx.tally(g, "board.markers_removed", 1);
    if is_city(&w, h) {
        cx.tally(g, "board.markers_removed_city", 1);
    }
    alignment::sway(g, cx, Side::Unity, w.int("board.clear_unity"));
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
        StandardAction::Move => "move a player ambassador onto a piece in reach",
        StandardAction::Ambassador => "gain a player ambassador",
        StandardAction::Align => "align a clean city you can claim",
        StandardAction::FreeStation => {
            return format!(
                "free a corrupted research station in reach where {} of your ambassadors stand and no adversary ambassador: they are used up and it becomes neutral",
                w.int("board.free_station_ambassadors")
            );
        }
        StandardAction::HardenChips => "the Harden action a card grants: harden a facility in reach",
        StandardAction::Chips => "sell 1 chip for credits",
        StandardAction::HardenBuy => "standard project: harden a facility in reach",
        StandardAction::Drain => "standard project: remove compute from an adversary project",
        StandardAction::Insight => {
            return format!("turn 1 research cube into {}", Resource::Insight.words(w.int("board.cube_insight")));
        }
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
            format!("{} credit", t("board.move_credits")),
            "from your supply or the board onto a piece in reach (not the hex it comes from)".into(),
        ),
        StandardAction::Ambassador => (
            "Gain a player ambassador",
            format!("{} credits", t("board.ambassador_credits")),
            "into your supply; a standard project too".into(),
        ),
        // r40 (D38): on a dark city it cancels, as an ambassador arriving on a piece does.
        StandardAction::Align => (
            "Align a clean city",
            "1 ambassador".into(),
            "in reach, with a site claim: your cube on it; on a dark city it cancels one adversary ambassador there instead (both removed), and a later Align aligns the city once it is clean".into(),
        ),
        // 099: board.md and ambassadors.md (docs/rules/) send readers here for it.
        StandardAction::FreeStation => (
            "Free a research station",
            format!("{} credits; {} of your ambassadors on it, used up", t("board.free_station_credits"), t("board.free_station_ambassadors")),
            "a corrupted station in reach with no adversary ambassador on it and no face-up cyber threat holding it: it becomes neutral (a botnet there stays)".into(),
        ),
        StandardAction::HardenChips => (
            "Harden a facility (a card that grants it; none does: Faraday Yard hardens when played)",
            "the chips the card prints, per token".into(),
            "a player-owned station or factory in reach that is not corrupted, one a troop can strike".into(),
        ),
        StandardAction::Chips => ("Sell chips", "1 chip".into(), format!("gain {}", t("board.chips_credits"))),
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

/// A corrupted station seat `s` can free (r35): `free_station_ambassadors` of
/// its ambassadors on it, no adversary ambassador, no face-up cyber threat
/// holding it. A botnet there stays, and still blocks a claim.
pub fn station_freeable(w: &World, g: &GameState, seat: usize, h: usize) -> bool {
    corrupted_station(g, h)
        && g.tokens(h, ambassador(seat)) >= w.int("board.free_station_ambassadors")
        && g.tokens(h, ADV_AMBASSADOR) == 0
        && !threats::shielded(w, g, h, ThreatSource::Cyber)
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
        StandardAction::Align => all.filter(|&h| clean_city(w, g, h) && can_claim(w, g, seat, h)).collect(),
        StandardAction::FreeStation => all.filter(|&h| reach(h) && station_freeable(w, g, seat, h)).collect(),
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
        // 118: at `chips_credits` 0 selling a chip pays nothing: no such action, as `cube_insight` 0.
        StandardAction::Chips if w.int("board.chips_credits") > 0 => vec![usize::MAX],
        StandardAction::Chips => Vec::new(),
        StandardAction::Insight if w.int("board.cube_insight") > 0 => vec![usize::MAX],
        StandardAction::Insight => Vec::new(),
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
        StandardAction::Chips => {
            cx.add_resource(g, s, Resource::Credits, w.int("board.chips_credits"));
            cx.tally(g, "board.chips_sold", 1);
        }
        StandardAction::Drain => adversary::choose_drain(g, cx, s, w.int("board.drain_amount"))?,
        StandardAction::Insight => {
            cx.add_resource(g, s, Resource::Insight, w.int("board.cube_insight"));
            cx.tally(g, "board.cubes_converted", 1);
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
                move_ambassador(g, cx, s, h)?;
            }
        }
        StandardAction::Align => {
            if let Some(h) = on_hex(cx)? {
                // r40 (D38): on a dark city the ambassador cancels one adversary
                // ambassador there, as it does arriving on any piece, and aligns
                // nothing; the city is aligned by a later Align once it is clean.
                if g.tokens(h, ADV_AMBASSADOR) > 0 {
                    adversary::remove(g, cx, h, 1);
                    cx.tally(g, "board.canceled", 1);
                    cx.tally(g, "board.align_canceled", 1);
                    cx.note(Some(h), || format!("Seat {s}'s ambassador cancels an adversary ambassador at {}", hex_name(&w, h)));
                } else {
                    cx.add_tokens(g, h, cube(s), 1);
                    cx.tally(g, "board.cities_aligned", 1);
                }
            }
        }
        StandardAction::FreeStation => {
            if let Some(h) = on_hex(cx)? {
                cx.add_tokens(g, h, ambassador(s), -w.int("board.free_station_ambassadors"));
                cx.add_tokens(g, h, ADV_CUBE, -1);
                cx.tally(g, "board.stations_freed", 1);
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

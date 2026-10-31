//! The standard actions and projects: what each costs, targets and does.

use super::*;

/// A standard action. `Action::Standard` carries its [`name`](Self::name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardAction {
    Presence,
    Station,
    Remove,
    Clear,
    Move,
    Ambassador,
    Align,
    Harden,
    Chips,
    // 024: the credit sinks, Terraforming Mars' standard projects.
    PresenceBuy,
    HardenBuy,
    Energy,
    Drain,
}

/// The standard actions, in the order they are offered and published.
pub const STANDARD: [StandardAction; 13] = [
    StandardAction::Presence,
    StandardAction::Station,
    StandardAction::Remove,
    StandardAction::Clear,
    StandardAction::Move,
    StandardAction::Ambassador,
    StandardAction::Align,
    StandardAction::Harden,
    StandardAction::Chips,
    StandardAction::PresenceBuy,
    StandardAction::HardenBuy,
    StandardAction::Energy,
    StandardAction::Drain,
];

impl StandardAction {
    /// The action's name: `Action::Standard.act`, and its tally `board.std.<name>`.
    pub const fn name(self) -> &'static str {
        match self {
            StandardAction::Presence => "presence",
            StandardAction::Station => "station",
            StandardAction::Remove => "remove",
            StandardAction::Clear => "clear",
            StandardAction::Move => "move",
            StandardAction::Ambassador => "ambassador",
            StandardAction::Align => "align",
            StandardAction::Harden => "harden",
            StandardAction::Chips => "chips",
            StandardAction::PresenceBuy => "presence_buy",
            StandardAction::HardenBuy => "harden_buy",
            StandardAction::Energy => "energy",
            StandardAction::Drain => "drain",
        }
    }

    /// The action called `name`, if there is one.
    pub fn parse(name: &str) -> Option<StandardAction> {
        STANDARD.into_iter().find(|a| a.name() == name)
    }
}

/// What something costs: the one shape every standard action and every
/// placement of presence is priced in.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Price {
    pub energy: i64,
    pub credits: i64,
    pub chips: i64,
}

impl Price {
    /// The resources it charges, in print order.
    pub(super) fn words(self) -> Vec<String> {
        [(self.energy, Resource::Energy), (self.credits, Resource::Credits), (self.chips, Resource::Chips)]
            .iter()
            .filter(|(n, _)| *n > 0)
            .map(|(n, r)| r.amount(*n))
            .collect()
    }

    /// The same for a label: `1 credit`.
    pub(super) fn label(self) -> Vec<String> {
        [(self.energy, Resource::Energy), (self.credits, Resource::Credits), (self.chips, Resource::Chips)]
            .iter()
            .filter(|(n, _)| *n > 0)
            .map(|(n, r)| r.words(*n))
            .collect()
    }

    pub fn affordable(self, g: &GameState, seat: usize) -> bool {
        g.resource(seat, Resource::Energy) >= self.energy
            && g.resource(seat, Resource::Credits) >= self.credits
            && g.resource(seat, Resource::Chips) >= self.chips
    }

    pub(super) fn pay(self, g: &mut GameState, cx: &mut Cx, seat: usize) {
        cx.add_resource(g, seat, Resource::Energy, -self.energy);
        cx.add_resource(g, seat, Resource::Credits, -self.credits);
        cx.add_resource(g, seat, Resource::Chips, -self.chips);
        cx.tally(g, "board.chips_spent", self.chips);
    }
}

/// What seat `s` pays for a standard action now; a placement, at its
/// cheapest step, through [`placing_price`].
pub(super) fn seat_price(w: &World, g: &GameState, seat: usize, act: StandardAction) -> Price {
    let step = cheapest_step(w, g, seat).unwrap_or(0);
    match act {
        StandardAction::Presence => placing_price(w, g, seat, Placing::Action, step),
        StandardAction::PresenceBuy => placing_price(w, g, seat, Placing::Rich, step),
        _ => printed_price(w, act),
    }
}

/// A standard action's price as the rules print it, before any seat's track.
pub(super) fn printed_price(w: &World, act: StandardAction) -> Price {
    let i = |k: &str| w.int(&format!("board.{k}"));
    let energy = |energy| Price { energy, ..Price::default() };
    let credits = |credits| Price { credits, ..Price::default() };
    match act {
        StandardAction::Presence => energy(i("presence_energy")),
        StandardAction::Station => Price { energy: i("build_energy"), chips: i("station_chips"), ..Price::default() },
        StandardAction::Remove => energy(i("remove_energy")),
        StandardAction::Clear => energy(i("clear_energy")),
        StandardAction::Move => energy(i("move_energy")),
        StandardAction::Ambassador => credits(i("ambassador_credits")),
        StandardAction::Align => energy(i("align_energy")),
        StandardAction::Harden => energy(i("harden_energy")),
        StandardAction::Chips => Price { chips: 1, ..Price::default() },
        StandardAction::PresenceBuy => credits(i("presence_credits")),
        StandardAction::HardenBuy => credits(i("harden_credits")),
        StandardAction::Energy => credits(i("energy_credits")),
        StandardAction::Drain => credits(i("drain_credits")),
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
        StandardAction::Presence if w.int("board.presence_action_cap") > 0 => {
            return format!(
                "place a presence next to yours (uncovers your track's next space); {} a round",
                w.int("board.presence_action_cap")
            )
        }
        StandardAction::Presence => "place a presence next to yours (uncovers your track's next space)",
        StandardAction::Station => "build a research station in reach (yours)",
        StandardAction::Remove => "remove an adversary ambassador in reach",
        StandardAction::Move => "move a player ambassador onto a piece in reach",
        StandardAction::Ambassador => "gain a player ambassador",
        StandardAction::Align => "align a clean city you can claim (1 ambassador)",
        StandardAction::Harden => "harden a building in reach (Harden passive)",
        StandardAction::Chips => "sell 1 chip for credits",
        StandardAction::PresenceBuy => "the rich action: place a presence next to yours; steep, and rises with each use a round",
        StandardAction::HardenBuy => "standard project: harden a building in reach",
        StandardAction::Energy => "standard project: gain 1 energy",
        StandardAction::Drain => "standard project: remove compute from an adversary project",
    };
    text.to_string()
}

/// The hexes a standard action of `seat` may target (empty: not offered).
pub(super) fn std_targets(w: &World, g: &GameState, seat: usize, act: StandardAction) -> Vec<usize> {
    let reach = |h: usize| in_reach(w, g, seat, h);
    let all = 0..g.hexes.len();
    match act {
        StandardAction::Presence | StandardAction::PresenceBuy => {
            let open = match act {
                StandardAction::Presence => placement_open(w, g, seat),
                _ => g.var(adversary::STAGE) >= w.int("board.presence_stage"),
            };
            if open && ai::can_place(w, g, seat) {
                presence_hexes(w, g, seat)
            } else {
                Vec::new()
            }
        }
        StandardAction::Station => all
            .filter(|&h| buildable(w, g, h) && !w.map.hexes[h].industrial && reach(h))
            .collect(),
        StandardAction::Remove => all.filter(|&h| reach(h) && g.tokens(h, ADV_AMBASSADOR) > 0).collect(),
        StandardAction::Clear => all.filter(|&h| reach(h) && g.tokens(h, MARKER) > 0).collect(),
        // Onto a piece in reach, from the supply or another hex: never onto the
        // hex the seat's only ambassador already holds (020: that charged the
        // energy and moved nothing).
        StandardAction::Move => {
            let supply = g.resource(seat, Resource::Ambassadors) > 0;
            let on_board = ambassadors_on_board(g, seat);
            all.filter(|&h| {
                (supply || on_board > g.tokens(h, &ambassador(seat)))
                    && reach(h)
                    && has_piece(w, g, h)
            })
            .collect()
        }
        StandardAction::Align => {
            if g.resource(seat, Resource::Ambassadors) > 0 {
                all.filter(|&h| clean_city(w, g, h) && can_claim(w, g, seat, h))
                    .collect()
            } else {
                Vec::new()
            }
        }
        StandardAction::Harden => {
            if cards::has_passive(w, g, seat, |p| matches!(p, Op::HardenAction)) {
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
        // Gaining an ambassador, credits or energy needs no target.
        StandardAction::Ambassador | StandardAction::Chips | StandardAction::Energy => {
            vec![usize::MAX]
        }
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
        let prompt = format!("Seat {s}: {} on which hex?", act.name().trim_end_matches("_buy"));
        Ok(cx.choose(s, &prompt, opts)?.map(|i| targets[i]))
    };
    match act {
        StandardAction::Ambassador => cx.add_resource(g, s, Resource::Ambassadors, 1),
        StandardAction::Chips => {
            cx.add_resource(g, s, Resource::Credits, w.int("board.chips_credits"));
            cx.tally(g, "board.chips_sold", 1);
        }
        StandardAction::Presence => {
            place_presence(g, cx, s, Placing::Action)?;
            cx.add_key(g, &placed_key(s), 1);
        }
        StandardAction::PresenceBuy => {
            place_presence(g, cx, s, Placing::Rich)?;
        }
        StandardAction::Energy => cx.add_resource(g, s, Resource::Energy, 1),
        StandardAction::Drain => {
            let ids = adversary::projects_with_compute(g);
            let opts = ids.iter().map(|p| opt(p.clone())).collect();
            let prompt = format!("Seat {s}: remove compute from which adversary project?");
            if let Some(i) = cx.choose(s, &prompt, opts)? {
                adversary::drain_project(g, cx, &ids[i], w.int("board.drain_amount"));
            }
        }
        StandardAction::Station => {
            if let Some(h) = on_hex(cx)? {
                cx.add_tokens(g, h, STATION, 1);
                cx.add_tokens(g, h, &cube(s), 1);
            }
        }
        StandardAction::Remove => {
            if let Some(h) = on_hex(cx)? {
                adversary::remove_piece(g, cx, Some(s), h, ADV_AMBASSADOR);
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
                cx.add_resource(g, s, Resource::Ambassadors, -1);
                cx.add_tokens(g, h, &cube(s), 1);
                cx.tally(g, "board.cities_aligned", 1);
            }
        }
        StandardAction::Harden | StandardAction::HardenBuy => {
            if let Some(h) = on_hex(cx)? {
                cx.add_tokens(g, h, HARDEN, 1);
            }
        }
    }
    Ok(())
}

/// Clear's words with its reward (the configs' `clear_unity`, `clear_credits`).
pub(super) fn clear_words(w: &World) -> String {
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

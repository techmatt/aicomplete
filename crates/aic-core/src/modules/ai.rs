//! AI boards: which AI each seat plays, its presence track (read from the
//! board module, which owns presence), the keep upgrade, income, credit
//! production, starting resources and growth.

use crate::action::Action;
use crate::content::{AiBoard, Resource, Uncover};
use crate::key::Key;
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::state::GameState;
use crate::view::{BoardView, CardView, InnateView, TrackMark, TrackStep, TrackView, ViewOut};

use super::{board, cards, effects};

pub static DEF: ModuleDef = ModuleDef {
    id: "ai",
    summary: "AI boards: the presence track, growth, income, credit production, starting resources",
    requires: &["board"],
    variants: &[VariantDef {
        name: "boards",
        summary: "Each seat plays an AI board from content/ais.json",
        params: &[
            ParamSpec::list("boards", 0, 99, "AI board (index into content/ais.json) of each seat; the list repeats for more seats"),
            ParamSpec::int("start_credits", 0, 100, "credits each player starts with"),
            ParamSpec::list("slots", 1, 10, "building slots of each seat's AI board, in the order of boards; the list repeats"),
        ],
        phases: &[
            PhaseSpec::decision("growth", "each player picks one growth option on their board"),
            PhaseSpec::decision("power", "use AI-board powers, paid by sacrificing played cards' tags (combine with the action phase)"),
        ],
        rng: &[],
        uses_cards: false,
        build,
    }],
};

/// The AI board seat `s` plays.
pub fn board<'w>(w: &World<'w>, seat: usize) -> &'w AiBoard {
    let list = w.list("ai.boards");
    &w.content.ais[list[seat % list.len()] as usize]
}

/// Spaces uncovered on seat `s`'s presence track (the board module counts
/// the tokens taken off it, the starting one included).
pub fn uncovered(w: &World, g: &GameState, seat: usize) -> usize {
    board::track_taken(g, seat).clamp(1, board(w, seat).presence.len())
}

/// Spaces uncovered on seat `s`'s second presence track (021; none at setup).
fn uncovered2(w: &World, g: &GameState, seat: usize) -> usize {
    board::track2_taken(g, seat).min(board(w, seat).presence2.len())
}

/// Income from the AI board and its presence track: `(resource, amount)`
/// pairs (insight sets the round's insight; the rest arrive in upkeep).
pub fn track_income(w: &World, g: &GameState, seat: usize) -> Vec<(Resource, i64)> {
    let b = board(w, seat);
    let mut out: Vec<(Resource, i64)> = b.income.clone();
    for u in b.presence[..uncovered(w, g, seat)].iter().chain(&b.presence2[..uncovered2(w, g, seat)]) {
        out.extend(u.income.iter().cloned());
    }
    out
}

/// Whether seat `s` has uncovered its board's keep upgrade (021: tier-2
/// keeps cost `cards.keep_tier2_upgraded`).
pub fn keep2(w: &World, g: &GameState, seat: usize) -> bool {
    let b = board(w, seat);
    b.presence[..uncovered(w, g, seat)]
        .iter()
        .chain(&b.presence2[..uncovered2(w, g, seat)])
        .any(|u| u.keep2)
}

/// Building slots of seat `s`'s AI board (`ai.slots`, 021).
pub fn slots(w: &World, seat: usize) -> i64 {
    let list = w.list("ai.slots");
    list[seat % list.len()]
}

/// Set once seat `s` has used power `k` this round.
fn power_used(seat: usize, k: usize) -> String {
    format!("ai.power_used.{seat}.{k}")
}

/// Why seat `s` can't use its board's power `k` now, or `None` if it can.
fn power_blocked(w: &World, g: &GameState, seat: usize, k: usize) -> Option<&'static str> {
    let Some(p) = board(w, seat).powers.get(k) else {
        return Some("no such power");
    };
    if g.var(&power_used(seat, k)) > 0 {
        return Some("used");
    }
    if p.cost.iter().any(|(r, n)| g.resource(seat, *r) < *n) {
        return Some("unaffordable");
    }
    if !cards::TagNeed::new(&p.tags).payable(&cards::sources(w, g, seat)) {
        return Some("needs tags");
    }
    if !effects::feasible(w, g, seat, &p.effects) {
        return Some("no target");
    }
    None
}

/// Whether seat `s` has a presence token left on its track.
pub fn can_place(w: &World, g: &GameState, seat: usize) -> bool {
    board::track_taken(g, seat) < board(w, seat).presence.len()
        || board::track2_taken(g, seat) < board(w, seat).presence2.len()
}

/// What uncovering a space gives, in the words of its option (a record key:
/// `+1 credits, T2 keep -1`).
pub fn uncover_label(u: &Uncover) -> String {
    let mut parts: Vec<String> = u.income.iter().map(|(r, n)| format!("+{}", r.amount(*n))).collect();
    if u.keep2 {
        parts.push("T2 keep -1".into());
    }
    parts.join(", ")
}

/// The same for people to read (`+1 credit, T2 keep`; the track step's hover has the rest).
pub fn uncover_words(u: &Uncover) -> String {
    let mut parts: Vec<String> = u.income.iter().map(|(r, n)| format!("+{}", r.words(*n))).collect();
    if u.keep2 {
        parts.push("T2 keep".into());
    }
    parts.join(", ")
}

#[derive(Debug)]
struct Ai {
    start_credits: i64,
}

fn power_text(p: &crate::content::Power) -> String {
    let mut cost: Vec<String> = p.tags.iter().map(|(t, n)| format!("{n} {t}")).collect();
    cost.extend(p.cost.iter().map(|(r, n)| r.words(*n)));
    format!("Sacrifice {}: {} (once a round)", cost.join(" + "), p.text)
}

fn build(b: &Build) -> Result<Box<dyn Module>, String> {
    let boards = b.list("boards")?;
    if let Some(&x) = boards
        .iter()
        .find(|&&x| x as usize >= b.world.content.ais.len())
    {
        return Err(format!(
            "boards: no AI board {x} (content/ais.json has {})",
            b.world.content.ais.len()
        ));
    }
    Ok(Box::new(Ai {
        start_credits: b.int("start_credits")?,
    }))
}

impl Module for Ai {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            cx.add_resource(g, s, Resource::Credits, self.start_credits);
        }
    }

    fn round_start(&self, g: &mut GameState, cx: &mut Cx) {
        let w = cx.world;
        for s in 0..g.seats.len() {
            for k in 0..board(&w, s).powers.len() {
                cx.set_key(g, &power_used(s, k), 0);
            }
        }
    }

    fn legal(&self, phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        if phase == 1 {
            for k in 0..board(w, seat as usize).powers.len() {
                if power_blocked(w, g, seat as usize, k).is_none() {
                    out.push(Action::Power { seat, power: k as u8 });
                }
            }
            return;
        }
        for option in 0..board(w, seat as usize).growth.len() {
            out.push(Action::Growth {
                seat,
                option: option as u8,
            });
        }
    }

    fn can_pass(&self, phase: usize, _g: &GameState, _seat: u8) -> bool {
        phase == 1
    }

    fn owns(&self, phase: usize, a: &Action) -> bool {
        matches!(
            (phase, a),
            (0, Action::Growth { .. }) | (1, Action::Power { .. })
        )
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        if let Action::Power { seat, power } = a {
            let (s, k) = (*seat as usize, *power as usize);
            if let Some(why) = power_blocked(&w, g, s, k) {
                return Err(Halt::Illegal(format!("can't use power {k}: {why}")));
            }
            let p = &board(&w, s).powers[k];
            for (r, n) in &p.cost {
                cx.add_resource(g, s, *r, -n);
            }
            cx.set_key(g, &power_used(s, k), 1);
            cards::sacrifice_for(g, cx, s, &p.tags)?;
            cx.count(g, &format!("power.{}.{k}", board(&w, s).id), 1);
            cx.tally(g, "ai.powers_used", 1);
            cx.per_round(g, "ai.powers_used", 1);
            return effects::run(g, cx, &p.effects, &mut effects::Ctx::seat(s));
        }
        let Action::Growth { seat, option } = a else {
            return Err(Halt::Illegal("not a growth".into()));
        };
        let s = *seat as usize;
        let growth = board(&w, s)
            .growth
            .get(*option as usize)
            .ok_or("no such growth option")?;
        cx.count(g, &format!("growth.{}.{option}", board(&w, s).id), 1);
        effects::run(g, cx, &growth.effects, &mut effects::Ctx::seat(s))?;
        g.seats[s].passed = true;
        Ok(())
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        // Powers used show as their status on the board below.
        out.bookkeeping(&["ai.power_used."]);
        let track_view = |label: &str, steps: &[crate::content::Uncover], up: usize| TrackView {
            label: label.into(),
            resource: String::new(),
            steps: steps
                .iter()
                .enumerate()
                .map(|(i, u)| {
                    let mut marks: Vec<TrackMark> = u
                        .income
                        .iter()
                        .map(|(r, n)| TrackMark {
                            key: r.to_string(),
                            label: format!("+{}", r.words(*n)),
                            doc: format!("+{} every round once uncovered", r.words(*n)),
                        })
                        .collect();
                    if u.keep2 {
                        marks.push(TrackMark {
                            key: "tier2".into(),
                            label: "T2 keep".into(),
                            doc: "uncovering this lowers a tier-2 keep (cards.keep_tier2_upgraded insight)".into(),
                        });
                    }
                    // 044: the step's printed cost reached no client.
                    if u.cost > 0 {
                        marks.push(TrackMark {
                            key: "cost".into(),
                            label: format!("pay {}", Resource::Energy.words(u.cost)),
                            doc: "placing this token costs this much energy on top, unless a card says free (the rich action pays it in credits)".into(),
                        });
                    }
                    TrackStep {
                        value: i as i64 + 1,
                        marks,
                    }
                })
                .collect(),
            uncovered: up,
            current: up.saturating_sub(1),
        };
        for s in 0..g.seats.len() {
            let b = board(w, s);
            let up = uncovered(w, g, s);
            let mut tracks = vec![track_view("presence track", &b.presence, up)];
            if !b.presence2.is_empty() {
                tracks.push(track_view("second presence track", &b.presence2, uncovered2(w, g, s)));
            }
            let (hand, built) = (cards::hand(s), cards::built(s));
            let mut unique: Vec<CardView> = b
                .cards
                .iter()
                .map(|id| {
                    // A board two seats play has one copy of each card (rules.md
                    // §7): say who holds it (020: the second seat read "gone").
                    let key = Key::new(id);
                    let other = (0..g.seats.len()).find(|&o| {
                        o != s && (g.pile(&cards::hand(o)).contains(&key) || cards::in_play(g, o).contains(&key))
                    });
                    let status = if g.pile(&hand).contains(&key) {
                        "in hand".to_string()
                    } else if g.pile(&built).contains(&key) {
                        "built".to_string()
                    } else if cards::in_play(g, s).contains(&key) {
                        "in play".to_string()
                    } else if let Some(o) = other {
                        format!("with seat {o}")
                    } else {
                        "gone".to_string()
                    };
                    let (name, text) = match w.content.card_index(id) {
                        Some(i) => (
                            w.content.cards[i].name.clone(),
                            w.content.cards[i].text.clone(),
                        ),
                        None => (id.clone(), String::new()),
                    };
                    CardView {
                        id: id.clone(),
                        name,
                        text,
                        status: status.into(),
                        cost: None,
                    }
                })
                .collect();
            // The board's powers (021), with their status this round.
            for (k, p) in b.powers.iter().enumerate() {
                unique.push(CardView {
                    id: format!("{}.power.{k}", b.id),
                    name: p.name.clone(),
                    text: power_text(p),
                    status: power_blocked(w, g, s, k).unwrap_or("ready").into(),
                    cost: None,
                });
            }
            out.board(
                s,
                "board",
                BoardView {
                    key: String::new(),
                    source: String::new(),
                    id: b.id.clone(),
                    name: b.name.clone(),
                    text: b.text.clone(),
                    innate: Some(InnateView {
                        name: b.innate.name.clone(),
                        text: b.innate.text.clone(),
                    }),
                    tracks,
                    slots: Some(slots(w, s)),
                    cards: unique,
                },
            );
            out.seat(
                s,
                "credit_production",
                "credit production",
                b.credits,
                None,
                "credits this AI board makes every round",
            );
        }
    }
}

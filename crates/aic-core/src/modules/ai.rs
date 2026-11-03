//! AI boards: which AI each seat plays, its nodes track (read from the
//! board module, which owns nodes), the keep upgrade, income, credit
//! production, starting resources and powers.

use crate::action::Action;
use crate::content::{AiBoard, Innate, Op, Power, Resource, Tag, Uncover};
use crate::key::{Key, KeyBuf};
use crate::module::{
    Build, Cx, Halt, Module, ModuleDef, ParamSpec, PhaseSpec, Res, VariantDef, World,
};
use crate::state::GameState;
use crate::view::{BoardView, CardView, InnateView, TrackMark, TrackStep, TrackView, ViewOut};

use super::{board, cards, effects};

pub static DEF: ModuleDef = ModuleDef {
    id: "ai",
    summary: "AI boards: the node track, powers, income, credit production, starting resources",
    requires: &["board"],
    variants: &[VariantDef {
        name: "boards",
        summary: "Each seat plays an AI board from content/ais.json",
        params: &[
            ParamSpec::list("boards", 0, 99, "AI board (index into content/ais.json) of each seat; the list repeats for more seats"),
            ParamSpec::list("start_credits", 0, 100, "credits each seat's AI board starts with, in the order of boards; the list repeats"),
            ParamSpec::int("start_energy", 0, 20, "energy each player starts with"),
            ParamSpec::list("slots", 1, 10, "building slots of each seat's AI board, in the order of boards; the list repeats"),
        ],
        phases: &[
            PhaseSpec::decision("power", "use AI-board powers, each once a round: place nodes, or sacrifice played cards' tags (combine with the action phase)"),
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

/// Spaces uncovered on seat `s`'s node track (the board module counts
/// the tokens taken off it, the starting one included).
pub fn uncovered(w: &World, g: &GameState, seat: usize) -> usize {
    board::track_taken(g, seat).clamp(1, board(w, seat).economy_track.len())
}

/// Spaces uncovered on seat `s`'s second node track (021; none at setup).
fn uncovered2(w: &World, g: &GameState, seat: usize) -> usize {
    board::track2_taken(g, seat).min(board(w, seat).play_track.len())
}

/// Income from the AI board and its nodes track: `(resource, amount)`
/// pairs (insight sets the round's insight; the rest arrive in upkeep).
pub fn track_income(w: &World, g: &GameState, seat: usize) -> Vec<(Resource, i64)> {
    let b = board(w, seat);
    let mut out: Vec<(Resource, i64)> = b.income.clone();
    for u in uncovered_spaces(w, g, seat) {
        out.extend(u.income.iter().cloned());
    }
    out
}

/// [`track_income`] a pair at a time, for a caller that only adds them up.
pub fn each_track_income(w: &World, g: &GameState, seat: usize, mut f: impl FnMut(Resource, i64)) {
    let b = board(w, seat);
    b.income.iter().for_each(|(r, n)| f(*r, *n));
    for u in uncovered_spaces(w, g, seat) {
        u.income.iter().for_each(|(r, n)| f(*r, *n));
    }
}

/// Spaces seat `s` has uncovered on both tracks.
fn uncovered_spaces<'w>(w: &World<'w>, g: &GameState, seat: usize) -> impl Iterator<Item = &'w Uncover> {
    let b = board(w, seat);
    b.economy_track[..uncovered(w, g, seat)].iter().chain(&b.play_track[..uncovered2(w, g, seat)])
}

/// The capstones seat `s` has uncovered (068): a track's last space, so a
/// hot path needn't walk the tracks.
pub fn capstones<'w>(w: &World<'w>, g: &GameState, seat: usize) -> impl Iterator<Item = &'w Innate> {
    let b = board(w, seat);
    let end = |t: &'w [Uncover], up: usize| t.last().filter(|_| up >= t.len()).and_then(|u| u.capstone.as_ref());
    end(&b.economy_track, uncovered(w, g, seat)).into_iter().chain(end(&b.play_track, uncovered2(w, g, seat)))
}

/// The tags printed on seat `s`'s board and those its tracks have revealed
/// (068): they count for what a card requires.
pub fn board_tags<'w>(w: &World<'w>, g: &GameState, seat: usize) -> impl Iterator<Item = &'w Tag> {
    board(w, seat).tags.iter().chain(uncovered_spaces(w, g, seat).flat_map(|u| &u.tags))
}

/// Whether seat `s` has uncovered its board's keep upgrade (021: tier-2
/// keeps cost `cards.keep_tier2_upgraded`).
pub fn keep2(w: &World, g: &GameState, seat: usize) -> bool {
    uncovered_spaces(w, g, seat).any(|u| u.keep2)
}

/// Credits seat `s`'s AI board starts with (`ai.start_credits`, 068).
pub fn start_credits(w: &World, seat: usize) -> i64 {
    let list = w.list("ai.start_credits");
    list[seat % list.len()]
}

/// Building slots of seat `s`'s AI board (`ai.slots`, 021).
pub fn slots(w: &World, seat: usize) -> i64 {
    let list = w.list("ai.slots");
    list[seat % list.len()]
}

/// Whether power `k` of seat `s`'s board places nodes.
pub fn power_places(w: &World, seat: usize, k: usize) -> bool {
    board(w, seat).powers.get(k).is_some_and(|p| p.effects.iter().any(|e| matches!(e, Op::PlaceNode { .. })))
}

/// Whether power `k` of seat `s`'s board plays a building for free (068).
pub fn power_builds(w: &World, seat: usize, k: usize) -> bool {
    board(w, seat).powers.get(k).is_some_and(|p| p.effects.iter().any(|e| matches!(e, Op::FreeBuilding)))
}

/// Whether power `k` of seat `s`'s board buys a research cube (075).
pub fn power_buys_cube(w: &World, seat: usize, k: usize) -> bool {
    board(w, seat).powers.get(k).is_some_and(|p| p.effects.iter().any(|e| matches!(e, Op::ProjectCube { .. })))
}

/// Whether power `k` of seat `s`'s board only turns one resource into another
/// (075, a power menu's conversions): it sacrifices nothing and places nothing.
pub fn power_converts(w: &World, seat: usize, k: usize) -> bool {
    board(w, seat).powers.get(k).is_some_and(|p| {
        p.tags.is_empty() && !p.cost.is_empty() && !p.effects.iter().any(|e| matches!(e, Op::PlaceNode { .. } | Op::Harden { .. }))
    })
}

/// Whether power `k` of seat `s`'s board sacrifices more than one tag: its
/// sacrifice prompts come before what it does (068).
pub fn power_sacrifices_many(w: &World, seat: usize, k: usize) -> bool {
    board(w, seat).powers.get(k).is_some_and(|p| p.tags.iter().map(|(_, n)| n).sum::<i64>() > 1)
}

/// Set once seat `s` has used power `k` this round.
pub(crate) fn power_used(seat: usize, k: usize) -> KeyBuf {
    KeyBuf::new("ai.power_used.").num(seat).text(".").num(k)
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
    // 066: a power that places pays its energy first, and the step after it.
    let spent: i64 = p.cost.iter().filter(|(r, _)| *r == Resource::Energy).map(|(_, n)| *n).sum();
    if power_places(w, seat, k) && !board::can_pay_placement(w, g, seat, board::Placing::Step, -spent) {
        return Some("unaffordable");
    }
    if !p.tags.is_empty() && !cards::TagNeed::new(&p.tags).payable(&cards::sources(w, g, seat)) {
        return Some("needs tags");
    }
    if !effects::feasible_after(w, g, seat, &p.effects, spent) {
        return Some("no target");
    }
    None
}

/// 075: what seat `s` could still do with its dear powers as the action phase
/// ends, and why it can't: the measure's, so a power near unused has a cause.
pub fn tally_dear_powers(g: &mut GameState, cx: &mut Cx, seat: usize) {
    let w = cx.world;
    let b = board(&w, seat);
    for (k, p) in b.powers.iter().enumerate() {
        if p.tags.iter().map(|(_, n)| n).sum::<i64>() < 2 {
            continue;
        }
        let why = power_blocked(&w, g, seat, k);
        let key = match why {
            None => "idle",
            Some("used") => continue,
            Some("needs tags") => "no_tags",
            Some("no target") => "no_target",
            Some(_) => "unpaid",
        };
        cx.tally(g, &format!("ai.dear.{}.{k}.{key}", b.id), 1);
    }
}

/// Whether seat `s` has a node left on its track.
pub fn can_place(w: &World, g: &GameState, seat: usize) -> bool {
    board::track_taken(g, seat) < board(w, seat).economy_track.len()
        || board::track2_taken(g, seat) < board(w, seat).play_track.len()
}

/// Whether uncovering `u` builds a research station (075).
pub fn builds_station(u: &Uncover) -> bool {
    u.effects.iter().any(|e| matches!(e, Op::BuildStation))
}

/// What uncovering a space gives, in the words of its option (a record key:
/// `+1 credits, T2 keep -1`).
pub fn uncover_label(u: &Uncover) -> String {
    let mut parts: Vec<String> = u.income.iter().map(|(r, n)| format!("+{}", r.amount(*n))).collect();
    if u.keep2 {
        parts.push("T2 keep -1".into());
    }
    parts.extend(u.tags.iter().map(|t| format!("{t} tag")));
    if builds_station(u) {
        parts.push("free station".into());
    }
    if let Some(c) = &u.capstone {
        parts.push(format!("capstone {}", c.name));
    }
    parts.join(", ")
}

/// The same for people to read (`+1 credit, T2 keep`; the track step's hover has the rest).
pub fn uncover_words(u: &Uncover) -> String {
    let mut parts: Vec<String> = u.income.iter().map(|(r, n)| format!("+{}", r.words(*n))).collect();
    if u.keep2 {
        parts.push("T2 keep".into());
    }
    parts.extend(u.tags.iter().map(|t| format!("{t} tag")));
    if builds_station(u) {
        parts.push("free station".into());
    }
    if let Some(c) = &u.capstone {
        parts.push(format!("capstone: {}", c.name));
    }
    parts.join(", ")
}

#[derive(Debug)]
struct Ai {
    start_energy: i64,
}

/// What a power costs, in words: `pay 1 energy, sacrifice 2 diplomacy`.
fn power_cost(p: &Power) -> String {
    let mut cost = Vec::new();
    if !p.cost.is_empty() {
        let paid: Vec<String> = p.cost.iter().map(|(r, n)| r.words(*n)).collect();
        cost.push(format!("pay {}", paid.join(" + ")));
    }
    if !p.tags.is_empty() {
        let tags: Vec<String> = p.tags.iter().map(|(t, n)| format!("{n} {t}")).collect();
        cost.push(format!("sacrifice {}", tags.join(" + ")));
    }
    cost.join(", ")
}

fn power_text(p: &Power) -> String {
    let cost = power_cost(p);
    let mut first = cost.chars();
    let cost: String = first.next().map_or(String::new(), |c| c.to_uppercase().chain(first).collect());
    format!("{cost}: {} (once a round)", p.text)
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
        start_energy: b.int("start_energy")?,
    }))
}

impl Module for Ai {
    fn setup(&self, g: &mut GameState, cx: &mut Cx) {
        for s in 0..g.seats.len() {
            cx.add_resource(g, s, Resource::Credits, start_credits(&cx.world, s));
            cx.add_resource(g, s, Resource::Energy, self.start_energy);
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

    fn legal(&self, _phase: usize, g: &GameState, w: &World, seat: u8, out: &mut Vec<Action>) {
        for k in 0..board(w, seat as usize).powers.len() {
            if power_blocked(w, g, seat as usize, k).is_none() {
                out.push(Action::Power { seat, power: k as u8 });
            }
        }
    }

    fn can_pass(&self, _phase: usize, _g: &GameState, _seat: u8) -> bool {
        true
    }

    fn owns(&self, _phase: usize, a: &Action) -> bool {
        matches!(a, Action::Power { .. })
    }

    fn act(&self, _phase: usize, g: &mut GameState, cx: &mut Cx, a: &Action) -> Res {
        let w = cx.world;
        let Action::Power { seat, power } = a else {
            return Err(Halt::Illegal("not a power".into()));
        };
        let (s, k) = (*seat as usize, *power as usize);
        // An answer replays this from a table that passed it.
        if !cx.replaying() {
            if let Some(why) = power_blocked(&w, g, s, k) {
                return Err(Halt::Illegal(format!("can't use power {k}: {why}")));
            }
        }
        let p = &board(&w, s).powers[k];
        for (r, n) in &p.cost {
            cx.add_resource(g, s, *r, -n);
        }
        cx.set_key(g, &power_used(s, k), 1);
        cards::sacrifice_for(g, cx, s, &p.tags)?;
        cx.count(g, &format!("power.{}.{k}", board(&w, s).id), 1);
        cx.per_round(g, &format!("ai.power.{}.{k}", board(&w, s).id), 1);
        cx.tally(g, "ai.powers_used", 1);
        cx.per_round(g, "ai.powers_used", 1);
        effects::run(g, cx, &p.effects, &mut effects::Ctx::seat(s))
    }

    fn view(&self, g: &GameState, w: &World, out: &mut ViewOut) {
        // Powers used show as their status on the board below.
        out.bookkeeping(&["ai.power_used."]);
        let track_view = |label: &str, steps: &[Uncover], up: usize| TrackView {
            label: label.into(),
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
                    for t in &u.tags {
                        marks.push(TrackMark {
                            key: t.to_string(),
                            label: format!("{t} tag"),
                            doc: "this tag counts for what a card requires once uncovered".into(),
                        });
                    }
                    if builds_station(u) {
                        marks.push(TrackMark {
                            key: "station".into(),
                            label: "free station".into(),
                            doc: "uncovering this builds a research station in reach, paying no chips".into(),
                        });
                    }
                    if let Some(c) = &u.capstone {
                        marks.push(TrackMark {
                            key: "capstone".into(),
                            label: c.name.clone(),
                            doc: format!("capstone: {}", c.text),
                        });
                    }
                    // 044: the step's printed cost reached no client.
                    if u.cost > 0 {
                        marks.push(TrackMark {
                            key: "cost".into(),
                            label: format!("pay {}", Resource::Energy.words(u.cost)),
                            doc: "placing this token costs this much energy on top, unless a card says free".into(),
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
            let mut tracks = vec![track_view("economy track", &b.economy_track, up)];
            if !b.play_track.is_empty() {
                tracks.push(track_view("card-play track", &b.play_track, uncovered2(w, g, s)));
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
                        status,
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
                    tags: b.tags.iter().map(|t| t.to_string()).collect(),
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

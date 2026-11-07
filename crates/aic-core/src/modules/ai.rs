//! AI boards: which AI each seat plays, its nodes track (read from the
//! board module, which owns nodes), the keep upgrade, income, credit
//! production, starting resources and powers.

use crate::action::Action;
use crate::content::{AiBoard, Innate, Op, Power, Resource, Tag, Uncover};
use crate::key::KeyBuf;
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
            ParamSpec::list("start_credits", 0, 100, "credits each AI board starts with, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board"),
            ParamSpec::int("start_energy", 0, 20, "energy each player starts with"),
            ParamSpec::list("slots", 1, 10, "building slots of each AI board, by board (Builder, Protector, Researcher), whichever seat plays it; one value sets every board"),
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
    &w.content.ais[board_index(w, seat)]
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

/// The AI board (index into content/ais.json) seat `s` plays.
fn board_index(w: &World, seat: usize) -> usize {
    let list = w.ai_boards;
    list[seat % list.len()] as usize
}

/// `key`'s value for seat `s`'s board: the list is by board (093), one value
/// for every board or one per board in content/ais.json.
fn by_board(w: &World, key: &str, seat: usize) -> i64 {
    let list = w.list(key);
    list[board_index(w, seat) % list.len()]
}

/// Credits seat `s`'s AI board starts with (`ai.start_credits`, 068; by board, 093).
pub fn start_credits(w: &World, seat: usize) -> i64 {
    by_board(w, "ai.start_credits", seat)
}

/// Building slots of seat `s`'s AI board (`ai.slots`, 021; by board, 093).
pub fn slots(w: &World, seat: usize) -> i64 {
    by_board(w, "ai.slots", seat)
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
    // 118: with every node token placed the power read "unaffordable".
    if power_places(w, seat, k) && !can_place(w, g, seat) {
        return Some("no node left");
    }
    if power_places(w, seat, k) && !board::can_pay_placement(w, g, seat, board::Placing::Step, -spent) {
        return Some("unaffordable");
    }
    if !p.tags.is_empty() && !cards::TagNeed::new(&p.tags).payable(&cards::sources(w, g, seat)) {
        return Some("needs tags");
    }
    if !effects::feasible_after(w, g, seat, &p.effects, spent) {
        return Some("no target");
    }
    if p.effects.iter().any(|e| matches!(e, Op::FreeBuilding)) && !free_building_after(w, g, seat, p) {
        return Some("no target");
    }
    None
}

/// 099: whether a building is still free to play once a free-building power
/// (Rush Build) is paid: its resources spent and some sacrifice made, since a
/// building may have needed the tags it sacrifices, or its energy, or the stack
/// it covers (docs/rules/card-grammar.md). Without this the power took its energy and card
/// and played nothing. A sacrifice of more than one source isn't tried (no
/// such power prints one): the resources alone are checked then.
fn free_building_after(w: &World, g: &GameState, seat: usize, p: &Power) -> bool {
    let mut paid = g.clone();
    for (r, n) in &p.cost {
        *paid.seats[seat].resources.entry(*r).or_insert(0) -= n;
    }
    let need = cards::TagNeed::new(&p.tags);
    if need.met() || p.tags.iter().map(|(_, n)| n).sum::<i64>() > 1 {
        return cards::free_building_exists(w, &paid, seat);
    }
    let sources = cards::sources(w, &paid, seat);
    sources.iter().filter(|s| need.helps(&s.tags)).any(|s| builds_after(w, &mut paid, seat, s))
}

/// Whether a building is still free to play once source `s` is sacrificed:
/// tried on `g` itself, its piles put back after (141: a copy of the state for
/// each source was a sixth of a long game's time). `g` leaves as it came.
fn builds_after(w: &World, g: &mut GameState, seat: usize, s: &cards::SacrificeSource) -> bool {
    let mut saved = Vec::with_capacity(s.cards.len());
    for (pile, card) in &s.cards {
        if let Some(cards_in) = g.piles.get_mut(pile.as_str()) {
            saved.push((pile, cards_in.clone()));
            std::sync::Arc::make_mut(cards_in).retain(|c| c != card);
        }
    }
    let free = cards::free_building_exists(w, g, seat);
    for (pile, old) in saved.into_iter().rev() {
        *g.piles.get_mut(pile.as_str()).expect("a pile just written") = old;
    }
    free
}

/// A dear power: one that sacrifices 2 or more tags (075), or a tag and an
/// energy and places no node (r35's Rush Build, Shelter, Particle Run; Accord
/// is a node power). 099: the 2-tag test alone matched no power after r35, and
/// the measure went silent.
pub fn power_is_dear(p: &Power) -> bool {
    let tags: i64 = p.tags.iter().map(|(_, n)| n).sum();
    let places = p.effects.iter().any(|e| matches!(e, Op::PlaceNode { .. }));
    tags >= 2 || (tags >= 1 && !places && p.cost.iter().any(|(r, n)| *r == Resource::Energy && *n > 0))
}

/// 075: what seat `s` could still do with its dear powers as the action phase
/// ends, and why it can't: the measure's, so a power near unused has a cause.
pub fn tally_dear_powers(g: &mut GameState, cx: &mut Cx, seat: usize) {
    let w = cx.world;
    let b = board(&w, seat);
    for (k, p) in b.powers.iter().enumerate() {
        if !power_is_dear(p) {
            continue;
        }
        let why = power_blocked(&w, g, seat, k);
        let key = match why {
            None => "idle",
            Some("used") => continue,
            Some("needs tags") => "no_tags",
            Some("no target") | Some("no node left") => "no_target",
            Some(_) => "unpaid",
        };
        cx.tally(g, &format!("ai.dear.{}.{k}.{key}", b.id), 1);
        // 099: the cost is checked first, so a round short of both read "unpaid"
        // alone: the tags are counted on their own as well.
        if key == "unpaid" && !cards::TagNeed::new(&p.tags).payable(&cards::sources(&w, g, seat)) {
            cx.tally(g, &format!("ai.dear.{}.{k}.no_tags", b.id), 1);
        }
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
    let cap = |s: &str| {
        let mut first = s.chars();
        first.next().map_or(String::new(), |c| c.to_uppercase().chain(first).collect::<String>())
    };
    let cost = power_cost(p);
    // 118: a power with no price (Synthesis) printed ": discard a card...".
    if cost.is_empty() {
        return format!("{} (once a round)", cap(&p.text));
    }
    format!("{}: {} (once a round)", cap(&cost), p.text)
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
    let n = b.world.content.ais.len();
    for key in ["start_credits", "slots"] {
        let len = b.list(key)?.len();
        if len != 1 && len != n {
            return Err(format!("{key}: one value, or one per AI board ({n}), not {len}"));
        }
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
        if p.effects.iter().any(|e| matches!(e, Op::FreeBuilding)) {
            // 099: only a sacrifice that leaves a building to play (its energy is paid).
            cards::sacrifice_keeping(g, cx, s, &p.tags, &|st: &mut GameState, src| builds_after(&w, st, s, src))?;
        } else {
            cards::sacrifice_for(g, cx, s, &p.tags)?;
        }
        cx.count(g, &format!("power.{}.{k}", board(&w, s).id), 1);
        cx.per_round(g, &format!("ai.power.{}.{k}", board(&w, s).id), 1);
        cx.tally(g, "ai.powers_used", 1);
        cx.per_round(g, "ai.powers_used", 1);
        // 105: what a power removes fires the seat's fight triggers.
        cards::fighting(g, cx, s, |g, cx| effects::run(g, cx, &p.effects, &mut effects::Ctx::seat(s)))
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
                            // 118: the hover printed the config key, not the price.
                            // With the cards module off there is no keep to price.
                            doc: match w.variant("cards") {
                                Some(v) if v != "off" => format!(
                                    "uncovering this lowers a tier-2 keep from {} to {} insight, before Science discounts",
                                    w.int("cards.keep_tier2"),
                                    w.int("cards.keep_tier2_upgraded")
                                ),
                                _ => "uncovering this lowers a tier-2 keep".into(),
                            },
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
            // The board's powers (021), with their status this round.
            let powers: Vec<CardView> = b
                .powers
                .iter()
                .enumerate()
                .map(|(k, p)| CardView {
                    id: format!("{}.power.{k}", b.id),
                    name: p.name.clone(),
                    text: power_text(p),
                    status: power_blocked(w, g, s, k).unwrap_or("ready").into(),
                    cost: None,
                })
                .collect();
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
                    cards: powers,
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

use std::collections::BTreeSet;
use std::sync::Arc;

use rand::{RngCore, SeedableRng};

use super::*;
use crate::content::{HexKind, Resource, Side, TokenKind};
use crate::modules::adversary::{CURRENT, DECK, NEXT, PROJECTS, STAGE, SUPPLY};
use crate::modules::common::{
    ambassador, cube, node, node_of, ADV_AMBASSADOR, ADV_CUBE, FACTORY,
    HARDEN, BOTNET, STATION, TROOP, UNIT,
};

mod adversary;
mod alignment;
mod ambassadors;
mod boards;
mod cards;
mod config;
mod determinism;
mod endgame;
mod engine;
mod face;
mod factories;
mod freeing;
mod invariants;
mod map;
mod march;
mod new_ops;
mod ops;
mod overlay;
mod plan;
mod node_powers;
mod production;
mod projects;
mod researcher;
mod science;
mod setup;
mod standard_actions;
mod threat_sources;
mod threats;
mod view;
mod war;

const BASELINE: &str = include_str!("../../../../configs/baseline.json");
const SHARED_ROW: &str = include_str!("../../../../configs/shared_row.json");

/// The action phase of the baseline round.
const ACTIONS: &str = "cards.play+threats.counter+alignment.act+board.act+ai.power";

/// The baseline playing only the action phase, after setup.
#[track_caller]
fn action_only(sets: &[&str]) -> (Rules, GameState) {
    let mut c = baseline_with(sets);
    c.round = vec![ACTIONS.into()];
    let rules = rules_of(&c, "baseline_with", sets);
    let g = rules.new_game(4);
    (rules, g)
}

fn baseline() -> Config {
    Config::from_json(BASELINE).unwrap()
}

/// The baseline with overrides. A stale override names itself and the test
/// that passed it (136: one stale `lab_config` line failed ~60 tests with a
/// bare "not a parameter").
#[track_caller]
fn baseline_with(sets: &[&str]) -> Config {
    let mut c = baseline();
    apply_sets(&mut c,sets);
    c
}

/// Applies `sets` to `c`, naming the override and the test on a refusal.
#[track_caller]
fn apply_sets(c: &mut Config, sets: &[&str]) {
    for o in sets {
        if let Err(e) = c.apply_override(o) {
            panic!("override {o:?} (passed from {}): {e}", std::panic::Location::caller());
        }
    }
}

/// The rules of a helper's config, naming the parameter, the helper whose
/// overrides may be stale (`helper`) and the test that called it.
#[track_caller]
fn rules_of(c: &Config, helper: &str, sets: &[&str]) -> Rules {
    Rules::new(c).unwrap_or_else(|e| {
        panic!(
            "{e}\n  (the config from {helper}, plus this test's overrides {sets:?}; called from {})",
            std::panic::Location::caller()
        )
    })
}

/// The baseline playing only `round` (give the action phase too), with
/// buckets too roomy to tip, the clear-board win held back to stage III and
/// no outright Unity win: a crafted state stays put until the test acts.
#[track_caller]
fn lab_config(round: &[&str]) -> Config {
    let mut c = baseline_with(&[
        "alignment.tip_per_player=50",
        "alignment.discord_tip_per_player=50",
        "alignment.clear_win_stage=3",
        // The board starts empty: no node is no loss here.
        "alignment.node_loss=0",
        // Rule tests read these numbers, not the calibration.
        "adversary.botnet_compute=3",
        "board.ambassador_discord=1",
        "board.troop_node=0",
        "board.free_leaves=0",
        "board.unit_cap=3",
        "board.destruction_discord=1",
        "board.liberate_at=1",
        "board.last_safe_stage=0",
        "board.troop_steps=2",
        "board.troop_spent=1",
        // The build asks nothing: no Wall-pressure landing (080). Pin any new
        // build-step choice off here. (138: the board's creation line lands
        // only on adversary cities, which a lab board has none of until a test
        // puts one.)
        "adversary.wall_troops=0",
        // 021: no credits, so no neutral city's energy is bought unasked.
        "ai.start_credits=0",
    ]);
    c.round = round.iter().map(|s| s.to_string()).collect();
    c
}

/// Rules playing `phases` then the action phase, and a game of them at the
/// action phase of round 1 with the board emptied (no tokens anywhere).
#[track_caller]
fn lab(phases: &[&str]) -> (Rules, GameState) {
    lab_with(phases, &[])
}

/// `lab` with overrides on top of the lab config, whose params are not the
/// baseline's (`liberate_at` 1): a test of a tunable's value uses `baseline_with`.
#[track_caller]
fn lab_with(phases: &[&str], sets: &[&str]) -> (Rules, GameState) {
    let mut round = phases.to_vec();
    round.push(ACTIONS);
    let mut c = lab_config(&round);
    apply_sets(&mut c,sets);
    let rules = rules_of(&c, "lab_config", sets);
    let mut g = rules.new_game(1);
    // 137: round 1's card may land troops on a factory of the players' choice.
    answer_all(&rules, &mut g);
    assert!(g.pending.is_none() && !g.is_over());
    for h in Arc::make_mut(&mut g.hexes).iter_mut() {
        Arc::make_mut(h).tokens.clear();
    }
    (rules, g)
}

/// Rules playing only the action phase (the lab config), and a game of
/// them after setup, the board as set up.
#[track_caller]
fn lab_actions() -> (Rules, GameState) {
    lab_actions_with(&[])
}

/// `lab_actions` with overrides on top: a test of a mechanic pins the card
/// numbers it reads, `cards.<id>.cost=N`, so a repricing doesn't break it.
#[track_caller]
fn lab_actions_with(sets: &[&str]) -> (Rules, GameState) {
    let mut c = lab_config(&[ACTIONS]);
    apply_sets(&mut c,sets);
    let rules = rules_of(&c, "lab_config", sets);
    let g = rules.new_game(1);
    (rules, g)
}

/// The lab at its action phase after setup, where every seat has room to
/// grow: free open hexes next to its starting node.
fn actions_with_room() -> (Rules, GameState) {
    let (rules, g) = lab_actions();
    let w = rules.world();
    for s in 0..2 {
        assert!(
            crate::modules::board::node_hexes(&w, &g, s).len() >= 2,
            "seat {s} has no room to grow at setup"
        );
    }
    (rules, g)
}

/// Runs the round's phases again from the first, up to the next decision
/// (or the first choice).
fn run(rules: &Rules, g: &mut GameState) {
    g.phase = 0;
    rules.enter(g, None);
}

/// `run`, then answers every choice with its first option (135: an event
/// card's standing troops ask where they land, in the build).
fn run_answered(rules: &Rules, g: &mut GameState) {
    run(rules, g);
    answer_all(rules, g);
}

/// Deals Manhunt (military) to country 0 with a troop on a hex holding seat 0's
/// node, so seat 0 may counter it (118: a delay reaches only such threats).
fn counterable_manhunt(rules: &Rules, g: &mut GameState) {
    let h = rules.map.numbered(0, piece(0).node);
    put(g, h, crate::modules::common::node(0), 1);
    put(g, h, crate::modules::common::TROOP, 1);
    deal(g, "manhunt", 0);
}

fn put(g: &mut GameState, h: usize, kind: &str, n: i64) {
    if n == 0 {
        Arc::make_mut(&mut Arc::make_mut(&mut g.hexes)[h]).tokens.remove(kind);
    } else {
        Arc::make_mut(&mut Arc::make_mut(&mut g.hexes)[h]).tokens.insert(kind.into(), n);
    }
}

fn total(g: &GameState, kind: &str) -> i64 {
    (0..g.hexes.len()).map(|h| g.tokens(h, kind)).sum()
}

fn set_res(g: &mut GameState, seat: usize, r: Resource, n: i64) {
    g.seats[seat].resources.insert(r, n);
}

fn hexes_where(m: &Map, f: impl Fn(usize) -> bool) -> Vec<usize> {
    (0..m.hexes.len()).filter(|&h| f(h)).collect()
}

fn kind(m: &Map, h: usize) -> HexKind {
    m.hexes[h].kind
}

fn cities(m: &Map) -> Vec<usize> {
    hexes_where(m, |h| kind(m, h) == HexKind::City)
}

/// Makes pile `pile` exactly `cards`.
fn set_pile(g: &mut GameState, pile: &str, cards: &[&str]) {
    g.piles.insert(pile.into(), Arc::new(cards.iter().map(|c| Key::new(c)).collect()));
}

/// Corrupts every city but `keep`.
fn corrupt_cities_except(m: &Map, g: &mut GameState, keep: &[usize]) {
    for h in cities(m) {
        if !keep.contains(&h) {
            put(g, h, ADV_CUBE, 1);
        }
    }
}

/// A city with at least three open neighbors that touch no other city.
fn city_with_lonely_neighbors(m: &Map) -> (usize, Vec<usize>) {
    for c in cities(m) {
        let lonely: Vec<usize> = m.hexes[c]
            .neighbors
            .iter()
            .copied()
            .filter(|&n| kind(m, n) == HexKind::Open)
            .filter(|&n| {
                m.hexes[n]
                    .neighbors
                    .iter()
                    .all(|&x| x == c || kind(m, x) != HexKind::City)
            })
            .collect();
        if lonely.len() >= 3 {
            return (c, lonely);
        }
    }
    panic!("no city with three lonely neighbors");
}

fn piece(c: usize) -> &'static content::CountryPiece {
    &crate::content().map.countries[c]
}

/// Answers the open choice with option `i`.
fn answer(rules: &Rules, g: &mut GameState, i: usize) {
    let p = g.pending.as_ref().expect("a choice");
    let o = p.ask.options[i].clone();
    let seat = p.ask.seat;
    rules
        .apply_mut(
            g,
            &Action::Choose {
                seat,
                option: o.option,
                target: o.target,
            },
            None,
        )
        .unwrap();
}

/// Answers the open choice with the option labeled `label`.
fn answer_with(rules: &Rules, g: &mut GameState, label: &str) {
    let p = g.pending.as_ref().expect("a choice");
    let i = p.ask.options.iter().position(|o| o.option == label);
    let i = i.unwrap_or_else(|| panic!("{label:?} is not an option of {:?}", p.ask));
    answer(rules, g, i);
}

/// Answers the open choice with the option naming hex `h`.
fn answer_hex(rules: &Rules, g: &mut GameState, h: usize) {
    let at = rules.map.hexes[h].hex;
    let p = g.pending.as_ref().expect("a choice");
    let i = p.ask.options.iter().position(|o| o.target == Some(at));
    let i = i.unwrap_or_else(|| panic!("hex {h} is not an option of {:?}", p.ask));
    answer(rules, g, i);
}

/// Answers every choice with its first option.
fn answer_all(rules: &Rules, g: &mut GameState) {
    while g.pending.is_some() {
        answer(rules, g, 0);
    }
}

/// Applies `a` and answers whatever it asks with first options.
fn act(rules: &Rules, g: &mut GameState, a: Action) {
    rules
        .apply_mut(g, &a, None)
        .unwrap_or_else(|e| panic!("{a:?}: {e}"));
    answer_all(rules, g);
}

/// Where the power called `name` sits on seat `seat`'s board.
fn power_index(rules: &Rules, seat: usize, name: &str) -> usize {
    let w = rules.world();
    let found = modules::ai::board(&w, seat).powers.iter().position(|p| p.name == name);
    found.unwrap_or_else(|| panic!("seat {seat} has no power {name:?}"))
}

/// The power called `name` on seat `seat`'s board, as that seat's action.
fn power(rules: &Rules, seat: usize, name: &str) -> Action {
    Action::Power { seat: seat as u8, power: power_index(rules, seat, name) as u8 }
}

fn std_act(seat: u8, act: &str) -> Action {
    Action::Standard {
        seat,
        act: act.into(),
    }
}

/// Every seat passes the action phase, the lab's whole round, into the next.
fn next_round(rules: &Rules, g: &mut GameState) {
    let round = g.round;
    while g.round == round {
        let legal = rules.legal_actions(g);
        let pass = legal.iter().find(|a| matches!(a, Action::Pass { .. })).unwrap().clone();
        rules.apply_mut(g, &pass, None).unwrap();
    }
}

/// Takes `card` out of the player-card decks (to put it in a hand or in play).
fn out_of_decks(g: &mut GameState, card: &str) {
    for tier in [1, 2] {
        if let Some(p) = g.piles.get_mut(format!("cards.deck.{tier}").as_str()) {
            Arc::make_mut(p).retain(|c| c != card);
        }
    }
}

/// Every seat passes, then whatever that asks takes first options.
fn pass_round(rules: &Rules, g: &mut GameState) {
    for s in 0..g.seats.len() as u8 {
        rules.apply_mut(g, &Action::Pass { seat: s }, None).unwrap();
    }
    answer_all(rules, g);
}

/// Seat 0 turns 1 research cube into insight (the cube converter): an action
/// that touches nothing else, so the end checks run (r48: it was a gift of
/// insight, an action since removed).
fn any_action(rules: &Rules, g: &mut GameState) {
    set_res(g, 0, Resource::ResearchCubes, 1);
    act(rules, g, std_act(0, "insight"));
}

/// A lab with seat 0's node on the board and both tracks at `taken` steps.
fn at_step(sets: &[&str], taken: i64) -> (Rules, GameState) {
    let (rules, mut g) = lab_with(&[], sets);
    let a = rules.map.numbered(0, piece(0).node);
    put(&mut g, a, node(0), 1);
    g.vars.insert(crate::modules::board::track_key(0).into(), taken);
    g.vars.insert(crate::modules::board::track2_key(0).into(), taken);
    (rules, g)
}

/// The cheaper of seat 0's two tracks at `taken` steps.
fn step_cost(w: &module::World, taken: usize) -> i64 {
    let b = modules::ai::board(w, 0);
    b.economy_track[taken].cost.min(b.play_track[taken].cost)
}

#[track_caller]
fn threat_lab() -> (Rules, GameState) {
    threat_lab_with(&[])
}

/// `threat_lab` with overrides on top: a test pins the card numbers it reads.
#[track_caller]
fn threat_lab_with(sets: &[&str]) -> (Rules, GameState) {
    let mut c = lab_config(&[ACTIONS, "threats.execute"]);
    // 138: the threat tests read the printed counters and harms; the
    // calibration dials have their own tests (146: no tier 3, no unprinted harm).
    apply_sets(&mut c, &["adversary.counter_pct=100,100"]);
    apply_sets(&mut c,sets);
    let rules = rules_of(&c, "lab_config and threat_lab", sets);
    let g = rules.new_game(3);
    (rules, g)
}

/// 119: Synthetic Data Forge's Action pinned as printed at r46 (add 1 here, or 1
/// insight a Dataset here), so a reprice of the forge leaves the tests reading it alone.
const FORGE_ABILITY: &str = r#"cards.synthetic_data_forge.ability={"effects":[{"op":"add_tokens","kind":"datasets","amount":1,"here":true}],"or":{"effects":[{"op":"gain_per_tokens","resource":"insight","kind":"datasets","per":1,"here":true}]}}"#;

/// Puts `threat` on country `c` (out of its deck).
fn deal(g: &mut GameState, threat: &str, c: usize) {
    for t in crate::modules::threats::TIERS {
        if let Some(p) = g.piles.get_mut(format!("threats.deck.{t}").as_str()) {
            Arc::make_mut(p).retain(|x| x != threat);
        }
    }
    Arc::make_mut(g.piles.entry(format!("threats.on.{c}")).or_default()).push(threat.into());
}

/// Puts `threat` on country `c` with its location tokens on `hexes`, in its
/// locations' order (135: `deal` leaves it token-less, near the whole country).
fn deal_at(g: &mut GameState, threat: &str, c: usize, hexes: &[usize]) {
    deal(g, threat, c);
    for (i, &h) in hexes.iter().enumerate() {
        g.vars.insert(format!("threats.loc.{threat}.{i}").into(), h as i64 + 1);
    }
}

fn row(g: &mut GameState, ids: &[&str]) {
    set_pile(g, PROJECTS, ids);
}

/// Seat `seat` with the means to play any card.
fn fund(g: &mut GameState, seat: usize) {
    set_res(g, seat, Resource::Credits, 30);
    set_res(g, seat, Resource::Insight, 5);
    set_res(g, seat, Resource::Antimatter, 2);
}

/// The programs carrying `tag`, in content order.
fn programs<'a>(w: &'a module::World, tag: content::Tag) -> impl Iterator<Item = &'a content::CardDef> {
    w.content.cards.iter().filter(move |c| c.tags.contains(&tag) && !c.is_building() && !c.is_event())
}

/// Stacks `side`'s Alignment deck: `card` is drawn next, whatever the tier.
fn next_alignment_card(g: &mut GameState, side: Side, card: &str) {
    for tier in [1, 2] {
        set_pile(g, &format!("alignment.deck.{}.{tier}", side.name()), &[card]);
    }
}

/// Fills `side`'s track so the next action tips it.
fn fill_bucket(rules: &Rules, g: &mut GameState, side: Side) {
    let cap = crate::modules::alignment::capacity(&rules.world(), g, side);
    g.vars.insert(format!("alignment.{}", side.name()).into(), cap);
}

/// 136: `n` Unity cards drawn (the purge's steps count them), with the tip
/// count to match.
fn unity_drawn(g: &mut GameState, n: usize) {
    let ids: Vec<&str> = crate::content().alignment.iter().filter(|c| c.side == Side::Unity).map(|c| c.id.as_str()).cycle().take(n).collect();
    set_pile(g, "alignment.resolved.unity", &ids);
    g.vars.insert("alignment.unity_tips".into(), n as i64);
}

/// A Cx for calling module functions directly on a crafted state.
fn with_cx<T>(rules: &Rules, f: impl FnOnce(&mut module::Cx) -> T) -> T {
    with_cx_picks(rules, Vec::new(), f)
}

/// `with_cx` with the choices already answered by `picks`.
fn with_cx_picks<T>(
    rules: &Rules,
    picks: Vec<crate::state::Pick>,
    f: impl FnOnce(&mut module::Cx) -> T,
) -> T {
    let mut choices = module::Choices::new(picks);
    let mut cx = module::Cx {
        world: rules.world(),
        module: "test",
        purposes: &[],
        events: None,
        choices: &mut choices,
        fought: [0; 3],
    };
    f(&mut cx)
}

/// Runs `ops` for seat 0, its choices answered by `picks`.
fn run_ops_with(rules: &Rules, g: &mut GameState, picks: Vec<crate::state::Pick>, ops: &[content::Op]) {
    with_cx_picks(rules, picks, |cx| modules::effects::run(g, cx, ops, &mut modules::effects::Ctx::seat(0))).unwrap();
}

/// Runs `ops` for seat 0 on a crafted state: they may ask nothing.
fn run_ops(rules: &Rules, g: &mut GameState, ops: &[content::Op]) {
    run_ops_with(rules, g, Vec::new(), ops);
}

/// Answers a hex choice, if it was asked (a single option is taken for you).
fn pick(rules: &Rules, g: &mut GameState, h: usize) {
    if g.pending.is_some() {
        answer_hex(rules, g, h);
    }
}

/// Passes (else plays the first legal action) until `phase` is the current
/// decision: first-action play could pay and withdraw a threat counter forever.
fn advance_to(rules: &Rules, g: &mut GameState, phase: &str) {
    for _ in 0..10_000 {
        if g.pending.is_none() && rules.phase(g).is_some_and(|p| p.name == phase) {
            return;
        }
        let legal = rules.legal_actions(g);
        let a = legal
            .iter()
            .find(|a| matches!(a, Action::Pass { .. }))
            .unwrap_or(&legal[0])
            .clone();
        rules.apply_mut(g, &a, None).unwrap();
    }
    panic!("never reached {phase}");
}

fn random_game(rules: &Rules, seed: u64) -> (GameState, Vec<Action>) {
    struct Rand(rand_chacha::ChaCha8Rng);
    impl Agent for Rand {
        fn choose(&mut self, _: &Rules, _: &GameState, legal: &[Action]) -> usize {
            (self.0.next_u32() as usize) % legal.len()
        }
    }
    let mut agent = Rand(rand_chacha::ChaCha8Rng::seed_from_u64(seed ^ 0x5eed));
    let (g, rec) = play(rules, seed, &mut agent, 100_000);
    (g, rec.actions)
}

/// The variant matrix of `configs/variants.json` (threat and Alignment
/// variants, difficulty 0 and 4, the draft variants and 1-4 countries), which
/// `scripts/wasm-parity.mjs` and `scripts/bench/same_play.py` read too.
fn variant_matrix() -> Vec<Config> {
    #[derive(serde::Deserialize)]
    struct Variants {
        variants: Vec<Vec<String>>,
    }
    let v: Variants = serde_json::from_str(include_str!("../../../../configs/variants.json")).unwrap();
    v.variants
        .iter()
        .map(|s| {
            let s: Vec<&str> = s.iter().map(String::as_str).collect();
            let c = baseline_with(&s);
            Rules::new(&c).unwrap_or_else(|e| panic!("{s:?}: {e}"));
            c
        })
        .collect()
}

// ---------- the golden game ----------

const GOLDEN_REVISION: u32 = 63;
const GOLDEN_FINGERPRINT: &str = "0be96fe4e744a5f6";
const GOLDEN_CONTENT_HASH: &str = "977267a5ab0e6912";

/// A fixed game (baseline, seed u64::MAX; action `k` is legal action
/// `(k * 7919) % count`: "always the first" can pay and withdraw forever).
/// `scripts/wasm-smoke.mjs` checks the wasm build reaches the same state.
/// If this fails after an intended rules change, bump `RULES_REVISION`
/// and update `GOLDEN_REVISION` and both fingerprints. `GOLDEN_CONTENT_HASH`
/// is there because this game may never pay for a repriced card: it moves on any
/// edit to `content/`, a name or a space included, so update it (and the one in
/// wasm-smoke) on its own when play doesn't change.
#[test]
fn golden_game() {
    let rules = Rules::new(&baseline()).unwrap();
    let mut g = rules.new_game(u64::MAX);
    for k in 0usize.. {
        let legal = rules.legal_actions(&g);
        if legal.is_empty() {
            break;
        }
        let a = legal[k * 7919 % legal.len()].clone();
        rules.apply_mut(&mut g, &a, None).unwrap();
    }
    assert!(g.is_over());
    assert_eq!(
        (RULES_REVISION, g.fingerprint().as_str()),
        (GOLDEN_REVISION, GOLDEN_FINGERPRINT)
    );
    assert_eq!(rules.content_hash, GOLDEN_CONTENT_HASH);
}

#[test]
fn wasm_smoke_expects_the_golden_fingerprint() {
    let smoke = include_str!("../../../../scripts/wasm-smoke.mjs");
    assert!(smoke.contains(GOLDEN_FINGERPRINT), "scripts/wasm-smoke.mjs holds another fingerprint");
    assert!(smoke.contains(GOLDEN_CONTENT_HASH), "scripts/wasm-smoke.mjs holds another content hash");
}

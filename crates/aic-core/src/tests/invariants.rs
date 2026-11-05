//! Invariants checked after every action of many random games (020's bug
//! hunt): what a printed table can't hold (negative piles, two node of one
//! seat on a hex, pieces on the ocean, more nodes out than taken off the
//! track), a game that waits for nobody, a state the rules reject, a view
//! that can't be drawn. 044 adds r19 and r20's: a marker's hex, a shield with
//! something under it, every token off a track accounted for.

use super::*;
use crate::modules::common::{corrupted, has_building, node_owner, MARKER};
use crate::modules::{ai, board};

/// Random play that mostly acts: a pass only now and then, so games reach the
/// late rounds (uniform random play passes away most of its turns).
struct Busy(rand_chacha::ChaCha8Rng);

impl Agent for Busy {
    fn choose(&mut self, _: &Rules, _: &GameState, legal: &[Action]) -> usize {
        let acting: Vec<usize> = (0..legal.len())
            .filter(|&i| !matches!(legal[i], Action::Pass { .. }))
            .collect();
        let r = self.0.next_u32() as usize;
        if !acting.is_empty() && r % 100 < 85 {
            acting[(r / 100) % acting.len()]
        } else {
            r % legal.len()
        }
    }
}

/// Every broken invariant of `g`, as text.
fn broken(rules: &Rules, g: &GameState) -> Vec<String> {
    let w = rules.world();
    let mut out = Vec::new();
    if let Err(e) = rules.check_state(g) {
        out.push(format!("check_state: {e}"));
    }
    if !g.is_over() && rules.legal_actions(g).is_empty() {
        out.push("no legal action and no outcome".into());
    }
    for (h, hex) in g.hexes.iter().enumerate() {
        for (k, &n) in &hex.tokens {
            if n < 0 {
                out.push(format!("hex {h}: {n} {k}"));
            }
            if n != 0 && w.map.hexes[h].kind == HexKind::Ocean {
                out.push(format!("hex {h} is ocean but holds {n} {k}"));
            }
        }
        for s in 0..g.seats.len() {
            if g.tokens(h, node(s)) > 1 {
                out.push(format!("hex {h}: {} node of seat {s}", g.tokens(h, node(s))));
            }
        }
        if g.tokens(h, BOTNET) > 1 {
            out.push(format!("hex {h}: {} botnets", g.tokens(h, BOTNET)));
        }
        // 044: what a marker and a shield promise (r19, r20).
        let marker = g.tokens(h, MARKER);
        if marker > 1 {
            out.push(format!("hex {h}: {marker} corruption markers"));
        }
        if marker > 0 && node_owner(g, h).is_some() {
            out.push(format!("hex {h}: a marked hex holds node"));
        }
        if marker > 0 && corrupted(g, h) {
            out.push(format!("hex {h}: a marker on a hex with corruption"));
        }
        if g.tokens(h, HARDEN) > 0 && node_owner(g, h).is_none() && !has_building(g, h) {
            out.push(format!("hex {h}: a hardening token with nothing under it"));
        }
    }
    // 044: a token off a track is on the board or lost (it leaves the game, r19).
    let lost = g.stats.counters.get("board.node_lost").copied().unwrap_or(0);
    let out_of_tracks: i64 = (0..g.seats.len())
        .map(|s| (board::track_taken(g, s) + board::track2_taken(g, s)) as i64)
        .sum();
    let on_board: i64 = (0..g.seats.len()).map(|s| total(g, node(s))).sum();
    if on_board + lost != out_of_tracks {
        out.push(format!("{on_board} node on the board + {lost} lost, but {out_of_tracks} off the tracks"));
    }
    for (s, seat) in g.seats.iter().enumerate() {
        for (r, &n) in &seat.resources {
            if n < 0 {
                out.push(format!("seat {s}: {n} {r}"));
            }
        }
        let track = ai::board(&w, s).economy_track.len() + ai::board(&w, s).play_track.len();
        let taken = board::track_taken(g, s) + board::track2_taken(g, s);
        let placed = total(g, node(s));
        if taken > track {
            out.push(format!("seat {s}: {taken} node taken off a track of {track}"));
        }
        if placed > taken as i64 {
            out.push(format!("seat {s}: {placed} node on the board but {taken} taken off the track"));
        }
    }
    if g.var(SUPPLY) < 0 {
        out.push(format!("adversary supply {}", g.var(SUPPLY)));
    }
    if g.var("adversary.compute") < 0 {
        out.push(format!("adversary compute {}", g.var("adversary.compute")));
    }
    // Words players read carry no raw token keys (020: "node:3 at C3 #3").
    // ... nor the engine's old hex names ("C1 #5"; 020: the piece's name now).
    let index_name = |s: &str| {
        s.match_indices(" #").any(|(i, _)| {
            let head = &s[..i];
            let digits = head.trim_end_matches(|c: char| c.is_ascii_digit());
            digits.len() < head.len() && digits.ends_with('C')
                && digits[..digits.len() - 1].chars().last().is_none_or(|c| !c.is_alphanumeric())
        })
    };
    let raw_key = |s: &str| {
        ["node:", "cube:", "ambassador:"]
            .iter()
            .any(|k| s.contains(k))
            || index_name(s)
    };
    if let Some(p) = &g.pending {
        for text in std::iter::once(&p.ask.prompt).chain(p.ask.options.iter().map(|o| &o.option)) {
            if raw_key(text) {
                out.push(format!("raw token key in a choice: {text:?}"));
            }
        }
        // 118: a prompt names a card, not its id ("build joint_task_force where?");
        // options may carry ids, which replays match on.
        if p.ask.prompt.contains('_') {
            out.push(format!("raw id in a prompt: {:?}", p.ask.prompt));
        }
    }
    // The view is what clients draw: building it must not panic.
    let v = rules.view(g);
    if v.hexes.len() != g.hexes.len() {
        out.push("view hex count".into());
    }
    // Labels are words, not keys (020: `node_reserve`).
    let labels = v
        .globals
        .iter()
        .chain(v.seats.iter().flat_map(|s| s.values.iter()))
        .chain(v.countries.iter().flat_map(|c| c.values.iter()));
    for val in labels {
        if val.label.contains('_') || raw_key(&val.label) {
            out.push(format!("view label {:?} for {}", val.label, val.key));
        }
    }
    out
}

fn check_games(configs: &[Config], seeds: std::ops::Range<u64>) {
    let mut failures = Vec::new();
    for config in configs {
        let rules = Rules::new(config).unwrap();
        for seed in seeds.clone() {
            let mut agent = Busy(rand_chacha::ChaCha8Rng::seed_from_u64(seed ^ 0xb57));
            let mut g = rules.new_game(seed);
            let mut step = 0;
            while !g.is_over() && step < 20_000 {
                for b in broken(&rules, &g) {
                    failures.push(format!("{} seed {seed} step {step} round {}: {b}", config.hash(), g.round));
                }
                if failures.len() > 20 {
                    break;
                }
                let legal = rules.legal_actions(&g);
                if legal.is_empty() {
                    break;
                }
                let a = legal[agent.choose(&rules, &g, &legal)].clone();
                rules.apply_mut(&mut g, &a, None).unwrap();
                step += 1;
            }
            for b in broken(&rules, &g) {
                failures.push(format!("{} seed {seed} at the end: {b}", config.hash()));
            }
            if !g.is_over() {
                failures.push(format!("{} seed {seed}: not over after {step} actions", config.hash()));
            }
        }
    }
    assert!(failures.is_empty(), "{} broken invariants:\n{}", failures.len(), failures.join("\n"));
}

/// What seat `s` got for an action, other than paying: a token moved, a pile
/// outside its own cards changed, a var moved (once-a-round marks and payments
/// aside) or a resource rose. 044: cards were offered and paid for with no effect.
fn got_something(before: &GameState, after: &GameState, s: usize, played: bool) -> bool {
    // 076: a token added to a card is something (a progressive card's "add 1 here").
    let spent = |k: &str| {
        k.contains(".used") || k.starts_with("ai.power_used") || (k.starts_with("cards.") && !k.starts_with("cards.tokens."))
    };
    let hexes = before.hexes.iter().zip(after.hexes.iter()).any(|(a, b)| a.tokens != b.tokens);
    let vars = before.vars.iter().any(|(k, v)| !spent(k) && after.var(k) != *v)
        || after.vars.iter().any(|(k, v)| !spent(k) && before.var(k) != *v);
    let piles = after.piles.iter().any(|(k, p)| !k.starts_with("cards.") && before.pile(k) != p.as_ref() as &[_]);
    let rose = (0..before.seats.len()).any(|t| {
        after.seats[t].resources.iter().any(|(r, n)| {
            let had = before.seats[t].resources.get(r).copied().unwrap_or(0);
            if t == s { *n > had } else { *n != had }
        })
    });
    let hand = crate::modules::cards::hand(s);
    let drew = after.pile(&hand).len() + usize::from(played) > before.pile(&hand).len();
    // Rush Build: a new building is worth something in play, as in
    // `paid_for_nothing`, even if it took the place of a sacrificed stack.
    let built = crate::modules::cards::built(s);
    let built = after.pile(&built).iter().any(|c| !before.pile(&built).contains(c));
    hexes || vars || piles || rose || drew || built || after.is_over()
}

fn paid_for_nothing(configs: &[Config], seeds: std::ops::Range<u64>) -> Vec<String> {
    let mut found = Vec::new();
    for config in configs {
        let rules = Rules::new(config).unwrap();
        for seed in seeds.clone() {
            let mut agent = Busy(rand_chacha::ChaCha8Rng::seed_from_u64(seed ^ 0xc0));
            let mut g = rules.new_game(seed);
            for _ in 0..20_000 {
                if g.is_over() {
                    break;
                }
                let legal = rules.legal_actions(&g);
                let a = legal[agent.choose(&rules, &g, &legal)].clone();
                let watched = match &a {
                    Action::Use { seat, card } | Action::Play { seat, card } => {
                        Some((*seat as usize, format!("{:?} {card}", std::mem::discriminant(&a))))
                    }
                    Action::Standard { seat, .. } | Action::Power { seat, .. } => Some((*seat as usize, format!("{a:?}"))),
                    _ => None,
                };
                let before = g.clone();
                rules.apply_mut(&mut g, &a, None).unwrap();
                // Answering "stop" to a "you may" is the player's own choice, not a use that paid for nothing.
                let mut declined = false;
                while g.pending.is_some() && !g.is_over() {
                    let legal = rules.legal_actions(&g);
                    let c = legal[agent.choose(&rules, &g, &legal)].clone();
                    declined |= matches!(&c, Action::Choose { option, .. } if option == "stop");
                    rules.apply_mut(&mut g, &c, None).unwrap();
                }
                if let (false, Some((s, what))) = (declined, watched) {
                    // A program, a building and a card's own tags are worth something
                    // in play: only an event or a use has to do something.
                    let lasts = matches!(&a, Action::Play { card, .. } if rules.world().content.card_index(card).is_some_and(|i| rules.world().content.cards[i].kind != content::CardKind::Event));
                    if !lasts && !got_something(&before, &g, s, matches!(a, Action::Play { .. })) {
                        found.push(format!("seed {seed} round {}: {what}", g.round));
                    }
                }
            }
        }
    }
    found
}

/// 099: wherever Rush Build is offered in random games (106: 2..48 seeds, the
/// smaller share of buildings in r42's deck offered it in none of 2..24), every way of paying it
/// builds: tried on a copy for each first answer to its sacrifice. A net, not
/// the proof: with 099's fix taken out these games still never reached a
/// stranding hand (the `boards.rs` tests do), as `paid_for_nothing` never did.
fn rush_build_strandings(configs: &[Config], seeds: std::ops::Range<u64>) -> (usize, Vec<String>) {
    let mut tried = 0;
    let mut found = Vec::new();
    for config in configs {
        let rules = Rules::new(config).unwrap();
        let w = rules.world();
        for seed in seeds.clone() {
            let mut agent = Busy(rand_chacha::ChaCha8Rng::seed_from_u64(seed ^ 0x99));
            let mut g = rules.new_game(seed);
            for _ in 0..20_000 {
                if g.is_over() {
                    break;
                }
                let legal = rules.legal_actions(&g);
                for a in &legal {
                    let Action::Power { seat, power } = a else { continue };
                    let s = *seat as usize;
                    let free = ai::board(&w, s).powers[*power as usize].effects.iter().any(|e| matches!(e, content::Op::FreeBuilding));
                    if !free {
                        continue;
                    }
                    let built = |x: &GameState| x.pile(&crate::modules::cards::built(s)).to_vec();
                    let before = built(&g);
                    let mut t = g.clone();
                    rules.apply_mut(&mut t, a, None).unwrap();
                    let first: Vec<usize> = t.pending.as_ref().map_or(vec![0], |p| (0..p.ask.options.len()).collect());
                    for i in first {
                        tried += 1;
                        let mut u = t.clone();
                        let mut k = i;
                        while u.pending.is_some() && !u.is_over() {
                            let l = rules.legal_actions(&u);
                            rules.apply_mut(&mut u, &l[k.min(l.len() - 1)], None).unwrap();
                            k = 0;
                        }
                        if built(&u).iter().all(|c| before.contains(c)) {
                            found.push(format!("seed {seed} round {}: seat {s} answer {i} built nothing", g.round));
                        }
                    }
                }
                let a = legal[agent.choose(&rules, &g, &legal)].clone();
                rules.apply_mut(&mut g, &a, None).unwrap();
            }
        }
    }
    (tried, found)
}

#[test]
fn rush_build_always_builds() {
    let (_, found) = rush_build_strandings(&[baseline()], 0..2);
    assert!(found.is_empty(), "{}", found.join("\n"));
}

#[test]
#[ignore = "slow lane: Rush Build over more games and sizes"]
fn rush_build_always_builds_more_seeds() {
    let (tried, found) = rush_build_strandings(&[baseline(), baseline_with(&["countries=3"])], 2..48);
    assert!(found.is_empty(), "{} of {tried} tries built nothing:\n{}", found.len(), found.join("\n"));
    assert!(tried > 0, "no game offered Rush Build");
}

#[test]
fn no_action_is_paid_for_nothing() {
    let found = paid_for_nothing(&[baseline()], 0..2);
    assert!(found.is_empty(), "{} actions did nothing:\n{}", found.len(), found.join("\n"));
}

#[test]
#[ignore = "slow lane: paid-for-nothing over more games and sizes"]
fn no_action_is_paid_for_nothing_more_seeds() {
    let found = paid_for_nothing(&[baseline(), baseline_with(&["countries=3", "adversary.difficulty=2"])], 2..30);
    assert!(found.is_empty(), "{} actions did nothing:\n{}", found.len(), found.join("\n"));
}

/// Fast lane: one game of the baseline and one of a short variant (~1 s); the
/// slow lane plays every variant, 3 and 4 countries included.
#[test]
fn invariants_hold_in_random_games() {
    let configs: Vec<Config> = [&[][..], &["countries=1", "adversary.difficulty=4"]]
        .iter()
        .map(|s| baseline_with(s))
        .collect();
    check_games(&configs, 0..1);
}

#[test]
#[ignore = "slow lane: the invariant check in every variant, one seed each"]
fn invariants_hold_in_random_games_every_variant() {
    // One seed a variant (~14 s): the slow lane's slowest aic-core test.
    check_games(&variant_matrix(), 2..3);
}

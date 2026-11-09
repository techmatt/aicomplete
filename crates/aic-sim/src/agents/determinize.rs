//! Determinization: the agents' answer to hidden information. A state holds
//! the true order of every shuffled deck and the dice (`rng`) that will
//! shuffle them again; a search that simulated the state as it is would
//! peek. Before simulating, an agent replaces everything the printed game
//! hides with a guess drawn from its own RNG, consistent with what players
//! can see:
//!
//! - the player-card and threat decks: their contents are known (every card
//!   not seen elsewhere), their order is not, so they are shuffled;
//! - the Unity and Discord decks: a game deals a subset of the printed cards
//!   face down (the Unity deck always, the Discord deck at difficulty 4), so
//!   each pile is dealt again from the printed cards of its tier not yet
//!   resolved, at its size ([`redeal_alignment`], D20);
//! - the adversary deck: its size and the stage of each position are public
//!   (the stages are stacked), next round's card (`adversary.next`) is face
//!   up, and the top `adversary.peek` cards are shown (Recon Sweep). Every
//!   other position gets a card of its stage drawn from the printed cards of
//!   that stage not seen face up, current, in the discard or the peek: the
//!   game may deal a subset of each stage, so which ones is hidden too;
//! - the adversary project deck (135: one shuffled deck): its contents are
//!   known (every project not in the row, in play or discarded), its order
//!   is not, so it is shuffled.
//! - every RNG stream is reseeded, so later reshuffles are guesses too.
//!
//! A pending choice's snapshot gets the same guess, so answering it replays
//! the partly resolved step consistently.

use std::sync::Arc;

use aic_core::content::Side;
use aic_core::modules::{adversary, alignment};
use aic_core::{GameState, Key, Rules};
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Piles whose order is hidden but whose contents are known.
pub(crate) const SHUFFLED: [&str; 9] = [
    "cards.deck.1",
    "cards.deck.2",
    "threats.deck.1",
    "threats.deck.2",
    "threats.deck.3",
    DEALT[0].2,
    DEALT[1].2,
    DEALT[2].2,
    DEALT[3].2,
];

/// The Alignment piles a game may deal a subset of: (side, tier, pile).
const DEALT: [(Side, u8, &str); 4] = [
    (Side::Unity, 1, "alignment.deck.unity.1"),
    (Side::Unity, 2, "alignment.deck.unity.2"),
    (Side::Discord, 1, "alignment.deck.discord.1"),
    (Side::Discord, 2, "alignment.deck.discord.2"),
];

/// Deals each Alignment pile again from the printed cards of its tier not
/// yet resolved, at the pile's size (D20): which cards were dealt is face
/// down. A pile with no such cards to fill it (only a test's) stays.
///
/// `turn` rotates each shuffled pool, so deals 0, 1, 2... of one `rng` seed
/// open on different cards while each alone is a uniform deal: a sample of
/// deals covers the next card, the one a line feels first (054).
fn redeal_alignment(rules: &Rules, g: &mut GameState, rng: &mut ChaCha8Rng, turn: usize) {
    let w = rules.world();
    for (side, tier, key) in DEALT {
        let n = g.pile(key).len();
        if n == 0 {
            continue;
        }
        let seen = g.pile(&alignment::resolved(side));
        let mut pool: Vec<Key> = w
            .content
            .alignment
            .iter()
            .filter(|c| c.side == side && c.tier == tier && !seen.iter().any(|s| s.as_str() == c.id))
            .map(|c| Key::new(&c.id))
            .collect();
        pool.shuffle(rng);
        if pool.len() >= n {
            let turn = turn % pool.len();
            pool.rotate_left(turn);
            pool.truncate(n);
            g.piles.insert(key.into(), Arc::new(pool));
        }
    }
}

/// A snapshot's pile when the step drew before it asked: `was` is the cards it
/// drew (public now) on top of the state's `now`, so they go back on top of
/// `guessed`. `None` when the step did no such thing.
fn drawn_on_top(was: &[Key], now: &[Key], guessed: &[Key]) -> Option<Vec<Key>> {
    (was.len() > now.len() && was.ends_with(now)).then(|| {
        let mut pile = was[..was.len() - now.len()].to_vec();
        pile.extend_from_slice(guessed);
        pile
    })
}

/// The planner's lines (D20): the table as it is, decks and dice included,
/// except for which Alignment cards were dealt and the order of each dealt pile
/// (at d2 the Discord deck is dealt whole: its contents stand, its order is a
/// guess), the `sample`th guess fixed by `seed` so a line repeats. A pending
/// choice's snapshot gets the same piles, under what its step drew, as in [`guess`].
pub fn hide_alignment_deal(rules: &Rules, g: &mut GameState, seed: u64, sample: usize) {
    let original = g.pending.is_some().then(|| g.clone());
    redeal_alignment(rules, g, &mut ChaCha8Rng::seed_from_u64(seed), sample);
    let guessed = DEALT.map(|(_, _, key)| g.pile(key).to_vec());
    let (Some(p), Some(original)) = (g.pending.as_mut().map(Arc::make_mut), &original) else {
        return;
    };
    let snap = Arc::make_mut(&mut p.snapshot);
    for ((_, _, key), guess) in DEALT.into_iter().zip(guessed) {
        let (was, now) = (snap.pile(key), original.pile(key));
        let pile = match drawn_on_top(was, now, &guess) {
            Some(pile) => pile,
            None if was == now => guess,
            None => continue,
        };
        snap.piles.insert(key.into(), Arc::new(pile));
    }
}

/// Puts `g`'s RNG streams on `seeds`.
fn reseed(g: &mut GameState, seeds: &[(Key, u64)]) {
    for (purpose, seed) in seeds {
        g.rng
            .insert(*purpose, Arc::new(aic_core::rng::Stream::new(*seed, purpose)));
    }
}

/// Replaces the hidden parts of `g` with a sample drawn from `rng`.
pub fn determinize(rules: &Rules, g: &mut GameState, rng: &mut ChaCha8Rng) {
    crate::prof::time(crate::prof::Part::Determinize, || guess(rules, g, rng))
}

fn guess(rules: &Rules, g: &mut GameState, rng: &mut ChaCha8Rng) {
    let original = g.pending.is_some().then(|| g.clone());
    for key in SHUFFLED {
        if let Some(pile) = g.piles.get_mut(key) {
            let pile = Arc::make_mut(pile);
            // Sorted first, so the guess depends on the contents (public)
            // and the agent's RNG, never on the true order.
            pile.sort_unstable();
            pile.shuffle(rng);
        }
    }
    redeal_alignment(rules, g, rng, 0);
    resample_adversary_deck(rules, g, rng);
    shuffle_project_stages(rules, g, rng);
    let seeds: Vec<(Key, u64)> = g.rng.keys().map(|k| (*k, rng.next_u64())).collect();
    reseed(g, &seeds);
    if let (Some(p), Some(original)) = (g.pending.as_mut().map(Arc::make_mut), &original) {
        let snap = Arc::make_mut(&mut p.snapshot);
        // Usually the step that asked has not drawn, so its snapshot's
        // hidden piles are the same as the state's: give them the same
        // guess. A step may draw before it asks: then the snapshot's pile is the drawn
        // cards (now public) on top of the state's, and gets them on top of
        // the same guess, so replaying draws them again. A pile the step
        // drew to the bottom and reshuffled keeps its (drawn) order.
        let mut drawn_out = false;
        for key in SHUFFLED
            .iter()
            .copied()
            .chain([adversary::DECK, adversary::PROJECT_DECK])
        {
            let (was, now) = (snap.pile(key), original.pile(key));
            if was == now {
                match g.piles.get(key) {
                    Some(v) => snap.piles.insert(key.into(), v.clone()),
                    None => snap.piles.remove(key),
                };
            } else if let Some(pile) = drawn_on_top(was, now, g.piles.get(key).map_or(&[], |v| v.as_slice())) {
                snap.piles.insert(key.into(), Arc::new(pile));
            } else if was.len() > now.len() && was.starts_with(now) {
                let mut pile = g.piles.get(key).map_or_else(Vec::new, |v| v.to_vec());
                pile.extend(was[now.len()..].iter().cloned());
                snap.piles.insert(key.into(), Arc::new(pile));
            } else if key == adversary::DECK {
                resample_adversary_deck(rules, snap, rng);
            } else if key == adversary::PROJECT_DECK {
                shuffle_project_stages(rules, snap, rng);
            } else {
                // The step drew the snapshot's whole pile (public now) and
                // reshuffled (017-026: a tip reshuffled the moment its deck
                // empties): the snapshot keeps its order, so the replay
                // draws the same cards.
                drawn_out = true;
            }
        }
        // After such a reshuffle the step may draw again from the new
        // order, so the snapshot keeps its true RNG streams and the replay
        // reshuffles as the table did. This shows the agent that one
        // reshuffled order (rare: a step that empties a deck and draws
        // again); every other case gets the guessed streams.
        if !drawn_out {
            reseed(snap, &seeds);
        }
    }
}

/// Shuffles the project deck (135: one shuffled deck; its contents are
/// public, the projects not in the row, in play or discarded).
fn shuffle_project_stages(_rules: &Rules, g: &mut GameState, rng: &mut ChaCha8Rng) {
    let Some(deck) = g.piles.get_mut(adversary::PROJECT_DECK) else {
        return;
    };
    let deck = Arc::make_mut(deck);
    deck.sort_unstable();
    deck.shuffle(rng);
}

fn resample_adversary_deck(rules: &Rules, g: &mut GameState, rng: &mut ChaCha8Rng) {
    let w = rules.world();
    let content = w.content;
    let stage =
        |id: &str| content.adversary[content.adversary_index(id).expect("adversary card")].stage;
    let deck = g.pile(adversary::DECK).to_vec();
    if deck.is_empty() {
        return;
    }
    let peek = (g.var(adversary::PEEK).max(0) as usize).min(deck.len());
    let mut seen: Vec<&str> = deck[..peek].iter().map(|s| s.as_str()).collect();
    for key in [adversary::NEXT, adversary::CURRENT, adversary::DISCARD] {
        seen.extend(g.pile(key).iter().map(|s| s.as_str()));
    }
    let mut out = deck[..peek].to_vec();
    let mut pools: Vec<Vec<Key>> = (1..=3u8)
        .map(|s| {
            let mut v: Vec<Key> = content
                .adversary
                .iter()
                .filter(|c| c.stage == s && !seen.contains(&c.id.as_str()))
                .map(|c| Key::new(&c.id))
                .collect();
            v.shuffle(rng);
            v
        })
        .collect();
    for id in &deck[peek..] {
        let s = stage(id) as usize;
        // The pool always has a card: the true one is in it.
        out.push(pools[s - 1].pop().unwrap_or(*id));
    }
    g.piles.insert(adversary::DECK.into(), Arc::new(out));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::tests::{config_rules, rules};

    #[test]
    fn keeps_public_information() {
        for d in [0, 4] {
            let r = rules(d);
            for seed in 0..20 {
                let g = r.new_game(seed);
                let mut h = g.clone();
                determinize(&r, &mut h, &mut ChaCha8Rng::seed_from_u64(seed + 99));
                assert_eq!(g.hexes, h.hexes);
                assert_eq!(g.vars, h.vars);
                assert_eq!(g.seats, h.seats);
                for (k, v) in &g.piles {
                    let mut a = v.to_vec();
                    let mut b = h.pile(k).to_vec();
                    if k == adversary::DECK {
                        // Same stage at every position.
                        let w = r.world();
                        let st = |id: &Key| {
                            w.content.adversary[w.content.adversary_index(id).unwrap()].stage
                        };
                        assert_eq!(
                            a.iter().map(st).collect::<Vec<_>>(),
                            b.iter().map(st).collect::<Vec<_>>()
                        );
                        continue;
                    }
                    if DEALT.iter().any(|(_, _, key)| key == k) {
                        // Its size is public; which cards were dealt is not.
                        assert_eq!(a.len(), b.len(), "{k}");
                        continue;
                    }
                    // The project deck (135: one shuffled deck) keeps its contents
                    // only, as SHUFFLED piles do.
                    if k != adversary::PROJECT_DECK && !SHUFFLED.contains(&k.as_str()) {
                        assert_eq!(&a, &b, "{k}");
                    }
                    a.sort();
                    b.sort();
                    assert_eq!(a, b, "{k}");
                }
                // Legal actions are the same: the agent faces the same decision.
                assert_eq!(r.legal_actions(&g), r.legal_actions(&h));
            }
        }
    }

    /// Two tables that differ only in what is hidden (deck order, which
    /// adversary cards were dealt, the dice) give the same guess.
    #[test]
    fn guess_ignores_hidden_information() {
        let r = rules(0);
        for seed in 0..10 {
            let a = r.new_game(seed);
            // Swap a hidden adversary card for an unseen one of its stage.
            let w = r.world();
            let st = |id: &str| w.content.adversary[w.content.adversary_index(id).unwrap()].stage;
            let last = *a.pile(adversary::DECK).last().unwrap();
            let all: Vec<Key> = a.piles.values().flat_map(|p| p.iter()).cloned().collect();
            let other = w
                .content
                .adversary
                .iter()
                .find(|c| c.stage == st(&last) && !all.contains(&Key::new(&c.id)))
                .map(|c| Key::new(&c.id));
            // The same hidden difference on the table and, mid-resolution,
            // on the pending choice's snapshot (one table, seen twice).
            let hide = |g: &mut GameState| {
                for key in SHUFFLED {
                    if let Some(p) = g.piles.get_mut(key) {
                        Arc::make_mut(p).reverse();
                    }
                }
                for (purpose, s) in g.rng.iter_mut() {
                    *s = Arc::new(aic_core::rng::Stream::new(seed + 1000, purpose));
                }
                if let (Some(other), Some(deck)) = (&other, g.piles.get_mut(adversary::DECK)) {
                    for id in Arc::make_mut(deck).iter_mut().filter(|id| **id == last) {
                        *id = *other;
                    }
                }
            };
            let mut b = a.clone();
            hide(&mut b);
            if let Some(p) = b.pending.as_mut() {
                hide(Arc::make_mut(&mut Arc::make_mut(p).snapshot));
            }
            assert_ne!(a, b);
            let mut ga = a.clone();
            let mut gb = b.clone();
            determinize(&r, &mut ga, &mut ChaCha8Rng::seed_from_u64(5));
            determinize(&r, &mut gb, &mut ChaCha8Rng::seed_from_u64(5));
            assert_eq!(ga, gb, "seed {seed}");
        }
    }

    /// D20: the Unity deal's order is hidden (136: every printed card is
    /// dealt, so only the order is unknown): the guess may put any printed card
    /// on top, and a table holding the same cards in another order gives the
    /// same guess.
    #[test]
    fn the_unity_deal_is_hidden() {
        let r = rules(0);
        let w = r.world();
        let key = "alignment.deck.unity.1";
        let g = r.new_game(3);
        let printed = w.content.alignment.iter().filter(|c| c.side == Side::Unity && c.tier == 1).count();
        assert_eq!(printed, g.pile(key).len());
        let last = *g.pile(key).last().unwrap();
        let reached = (0..40).any(|s| {
            let mut h = g.clone();
            determinize(&r, &mut h, &mut ChaCha8Rng::seed_from_u64(s));
            h.pile(key).first() == Some(&last)
        });
        assert!(reached, "the guess never moved the dealt order");
        let mut other = g.clone();
        Arc::make_mut(other.piles.get_mut(key).unwrap()).reverse();
        let (mut ga, mut gb) = (g.clone(), other);
        hide_alignment_deal(&r, &mut ga, 7, 0);
        hide_alignment_deal(&r, &mut gb, 7, 0);
        assert_eq!(ga.pile(key), gb.pile(key));
    }

    /// 054: a planner's samples are all blind to the deal, and open on
    /// different cards while the printed cards last.
    #[test]
    fn sampled_deals_are_blind_and_open_differently() {
        let r = rules(0);
        let w = r.world();
        let key = "alignment.deck.unity.1";
        let g = r.new_game(3);
        let printed = w.content.alignment.iter().filter(|c| c.side == Side::Unity && c.tier == 1).count();
        let mut other = g.clone();
        Arc::make_mut(other.piles.get_mut(key).unwrap()).reverse();
        let deals = 4.min(printed);
        let mut tops = Vec::new();
        for sample in 0..deals {
            let (mut a, mut b) = (g.clone(), other.clone());
            hide_alignment_deal(&r, &mut a, 7, sample);
            hide_alignment_deal(&r, &mut b, 7, sample);
            assert_eq!(a.pile(key), b.pile(key), "sample {sample} read the table's deal");
            tops.push(a.pile(key)[0]);
        }
        tops.sort();
        tops.dedup();
        assert_eq!(tops.len(), deals);
    }

    /// In the middle of a resolution (a pending choice), answering on the
    /// guess plays out on the same public table as answering on the truth,
    /// up to the next hidden draw (the round's end, or a tip).
    /// Fast lane: seeds 0..8; the slow lane plays 8..40.
    #[test]
    fn pending_choices_replay_on_the_guess() {
        for name in ["baseline", "shared_row"] {
            pending_choices_replay(&config_rules(name, 0), 0..8);
        }
    }

    #[test]
    #[ignore = "slow lane: more seeds of the pending-choice guess check"]
    fn pending_choices_replay_on_the_guess_more_seeds() {
        for name in ["baseline", "shared_row"] {
            pending_choices_replay(&config_rules(name, 0), 8..40);
        }
    }

    /// 3 countries at d0 and d4 (017, before 027: the Discord deck reshuffled mid-step
    /// often; 019: the troops' march asks every round).
    #[test]
    #[ignore = "slow lane: the pending-choice guess check at 3 countries"]
    fn pending_choices_replay_on_the_guess_three_countries() {
        for d in [0, 4] {
            let r = crate::agents::tests::config_rules_with("baseline", d, &["countries=3"]);
            pending_choices_replay(&r, 0..30);
        }
    }

    fn pending_choices_replay(r: &Rules, seeds: std::ops::Range<u64>) {
        let mut found = 0;
        let mut legal = Vec::new();
        for seed in seeds {
            let mut g = r.new_game(seed);
            let mut k = seed;
            while !g.is_over() {
                r.legal_actions_into(&g, &mut legal);
                if g.pending.is_some() {
                    found += 1;
                    let mut h = g.clone();
                    determinize(r, &mut h, &mut ChaCha8Rng::seed_from_u64(k));
                    for a in &legal {
                        let mut x = g.clone();
                        let mut y = h.clone();
                        r.apply_mut(&mut x, a, None).unwrap();
                        r.apply_mut(&mut y, a, None)
                            .expect("the guess accepts every answer");
                        // Until the round ends or a bucket tips (a tip
                        // draws from a hidden Unity or Discord deck).
                        let tips = |s: &GameState| {
                            (
                                s.var("alignment.unity_tips"),
                                s.var("alignment.discord_tips"),
                            )
                        };
                        // Or a new project row is dealt (from the hidden
                        // project deck; 020's r11 games met one).
                        let row = |s: &GameState| s.pile(aic_core::modules::adversary::PROJECT_DECK).len();
                        // Or threats are dealt (from the hidden threat decks;
                        // 023's card set reaches a deal right after a choice).
                        let threats = |s: &GameState| {
                            (s.pile("threats.deck.1").len(), s.pile("threats.deck.2").len())
                        };
                        // 154: or a threat or project comes out that wasn't out before (a deck that
                        // empties and reshuffles mid-deal keeps its length while dealing a different
                        // card: d4's compute 16 turns the project deck over).
                        let dealt = |s: &GameState| {
                            s.vars.keys().any(|k| {
                                let k = k.as_str();
                                (k.starts_with("threats.loc.") || k.starts_with("adversary.project."))
                                    && g.vars.get(k).is_none()
                            })
                        };
                        if x.round == g.round
                            && tips(&x) == tips(&g)
                            && tips(&y) == tips(&g)
                            && row(&x) == row(&g)
                            && row(&y) == row(&g)
                            && threats(&x) == threats(&g)
                            && threats(&y) == threats(&g)
                            && !dealt(&x)
                            && !dealt(&y)
                        {
                            assert_eq!(
                                (&x.hexes, &x.vars, &x.seats),
                                (&y.hexes, &y.vars, &y.seats),
                                "seed {seed}, {a:?}"
                            );
                        }
                    }
                }
                k = k
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let i = (k >> 33) as usize % legal.len();
                r.apply_mut(&mut g, &legal[i].clone(), None).unwrap();
            }
        }
        assert!(found > 20, "only {found} pending choices met");
    }

    #[test]
    fn hides_the_order() {
        let r = rules(0);
        let g = r.new_game(7);
        let differ = (0..10).any(|i| {
            let mut h = g.clone();
            determinize(&r, &mut h, &mut ChaCha8Rng::seed_from_u64(i));
            h.pile("cards.deck.1") != g.pile("cards.deck.1")
        });
        assert!(differ);
    }
}

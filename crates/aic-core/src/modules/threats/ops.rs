//! What the ops threats print do to the players they hit (135: a threat's,
//! the players near it) on the hexes in reach (a threat's: near its tokens).

use super::*;

/// Who a prompt asks, and for which threat when one is resolving (118: "discard which
/// card?" named nothing).
fn head(why: Option<&str>, s: usize) -> String {
    match why {
        Some(n) => format!("{n}: seat {s}"),
        None => format!("Seat {s}"),
    }
}

/// `count` adversary ambassadors from the supply, each onto a piece of kind
/// `at` in `scope` (players choose each). r48 (D51): on military bases, one a
/// base while a base is free of this threat's ambassadors (Total War's two go
/// on two bases where there are two).
pub fn place_ambassadors(g: &mut GameState, cx: &mut Cx, hexes: &[usize], at: Spot, count: i64, why: Option<&str>) -> Res {
    let w = cx.world;
    let mut used: Vec<usize> = Vec::new();
    for _ in 0..count {
        let mut spots: Vec<usize> = hexes
            .iter()
            .copied()
            .filter(|&h| match at {
                Spot::PlayerPieces => player_owned(g, h),
                Spot::CleanCities => clean_city(&w, g, h),
                Spot::Factories => g.tokens(h, FACTORY) > 0,
                Spot::Bases => is_base(&w, h),
                Spot::Location | Spot::City | Spot::CitiesNear => false,
            })
            .collect();
        if at == Spot::Bases && spots.iter().any(|h| !used.contains(h)) {
            spots.retain(|h| !used.contains(h));
        }
        let opts = spots.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = || format!("{}: an adversary ambassador goes on which piece?", why.unwrap_or("Threat"));
        let Some(i) = cx.choose_loss(0, prompt, opts)? else {
            break;
        };
        adversary::place(g, cx, spots[i], 1);
        used.push(spots[i]);
    }
    Ok(())
}

/// Each player in `scope` loses `amount` tokens off their cards, their choice.
pub fn lose_tokens(g: &mut GameState, cx: &mut Cx, seats: &[usize], amount: i64, why: Option<&str>) -> Res {
    let w = cx.world;
    for &s in seats {
        for _ in 0..amount {
            let cards: Vec<Key> = cards::token_cards(&w, g, s, None)
                .into_iter()
                .filter(|c| g.var(&cards::tokens_key(s, c)) > 0)
                .collect();
            let opts = cards.iter().map(|c| opt(*c)).collect();
            let prompt = || format!("{}: lose a token from which card?", head(why, s));
            let Some(i) = cx.choose_loss(s, prompt, opts)? else {
                break;
            };
            cards::add_tokens(g, cx, s, &cards[i], -1);
            cx.tally(g, "threats.tokens_lost", 1);
        }
    }
    Ok(())
}

/// Each player in `scope` discards `n` cards or pays `amount` `resource`
/// (their choice); with neither possible, what they have and their hand.
pub fn requisition(
    g: &mut GameState,
    cx: &mut Cx,
    seats: &[usize],
    resource: Resource,
    amount: i64,
    n: i64,
    why: Option<&str>,
) -> Res {
    for &s in seats {
        let mut opts = Vec::new();
        let can_pay = g.resource(s, resource) >= amount;
        if can_pay {
            opts.push(opt(format!("pay {}", resource.amount(amount))));
        }
        if g.pile(&cards::hand(s)).len() as i64 >= n {
            opts.push(opt(format!("discard {n} cards")));
        }
        let prompt = || format!("{}: pay {} or discard {n} cards?", head(why, s), resource.amount(amount));
        match cx.choose(s, prompt, opts)? {
            Some(0) if can_pay => {
                cx.add_resource(g, s, resource, -amount);
            }
            Some(_) => {
                for k in 0..n {
                    let hand = g.pile(&cards::hand(s)).to_vec();
                    let opts = hand.iter().map(|c| opt(*c)).collect();
                    let prompt = || format!("{}: discard which card ({} of {n})?", head(why, s), k + 1);
                    if let Some(j) = cx.choose(s, prompt, opts)? {
                        cards::discard(g, cx, &hand[j], &cards::hand(s));
                    }
                }
            }
            None => {
                // Neither possible: pay what you have and discard your hand.
                let k = g.resource(s, resource);
                cx.add_resource(g, s, resource, -k);
                for c in g.pile(&cards::hand(s)).to_vec() {
                    cards::discard(g, cx, &c, &cards::hand(s));
                }
            }
        }
    }
    Ok(())
}

/// Each player in `scope` loses a player ambassador: from their supply, else
/// one of theirs on the board (their choice).
pub fn lose_ambassador(g: &mut GameState, cx: &mut Cx, seats: &[usize], why: Option<&str>) -> Res {
    let w = cx.world;
    for &s in seats {
        if g.resource(s, Resource::Ambassadors) > 0 {
            cx.add_resource(g, s, Resource::Ambassadors, -1);
            continue;
        }
        let k = ambassador(s);
        let on = ambassador_hexes(g, s);
        let opts = on.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = || format!("{}: lose the ambassador on which hex?", head(why, s));
        if let Some(i) = cx.choose_loss(s, prompt, opts)? {
            cx.add_tokens(g, on[i], k, -1);
        }
    }
    Ok(())
}

/// Each player in `scope` loses a node in `scope` or pays `amount`
/// `resource` (their choice). Losing one needs 2 or more on the board (027:
/// this never takes a last node); with neither possible, what they have.
#[allow(clippy::too_many_arguments)]
pub fn lose_node_or(
    g: &mut GameState,
    cx: &mut Cx,
    seats: &[usize],
    hexes: &[usize],
    resource: Resource,
    amount: i64,
    source: &'static str,
    why: Option<&str>,
) -> Res {
    let w = cx.world;
    for &s in seats {
        let mine: Vec<usize> = if node_count(g, s) >= 2 {
            hexes.iter().copied().filter(|&h| g.tokens(h, node(s)) > 0).collect()
        } else {
            Vec::new()
        };
        let mut opts = Vec::new();
        let can_pay = g.resource(s, resource) >= amount;
        if can_pay {
            opts.push(opt(format!("pay {amount} {resource}")));
        }
        let off = opts.len();
        opts.extend(mine.iter().map(|&h| labeled_hex_opt(&w, "lose nodes", h)));
        let prompt = || format!("{}: pay {amount} {resource} or lose a node?", head(why, s));
        match cx.choose_loss(s, prompt, opts)? {
            Some(i) if i < off => cx.add_resource(g, s, resource, -amount),
            Some(i) => {
                board::lose_node(g, cx, mine[i - off], s, source);
            }
            None => {
                let k = g.resource(s, resource);
                cx.add_resource(g, s, resource, -k);
            }
        }
    }
    Ok(())
}

/// Each player in `scope` discards their dearest card in hand (their choice
/// on a tie).
pub fn discard_most_expensive(g: &mut GameState, cx: &mut Cx, seats: &[usize], why: Option<&str>) -> Res {
    let w = cx.world;
    for &s in seats {
        let hand = g.pile(&cards::hand(s)).to_vec();
        let cost = |id: &str| cards::list_cost(&w, id);
        let Some(top) = hand.iter().map(|c| cost(c)).max() else {
            continue;
        };
        let tied: Vec<Key> = hand.into_iter().filter(|c| cost(c) == top).collect();
        let opts = tied.iter().map(|c| opt(c.as_str())).collect();
        let prompt = || format!("{}: discard which of your dearest cards?", head(why, s));
        if let Some(i) = cx.choose(s, prompt, opts)? {
            cards::discard(g, cx, &tied[i], &cards::hand(s));
        }
    }
    Ok(())
}

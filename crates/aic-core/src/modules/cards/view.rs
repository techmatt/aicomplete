//! What the cards show: hands, buildings, the draft and the discards.

use super::*;

/// One card as a seat sees it, its status and costs in its text.
fn card_view(w: &World, g: &GameState, s: usize, id: &str, status: &str, cost: Option<i64>) -> CardView {
    let d = def(w, id);
    let mut bits = vec![format!("{:?}", d.kind).to_lowercase(), format!("tier {}", d.tier)];
    if !d.tags.is_empty() {
        bits.push(d.tags.iter().map(|t| t.name()).collect::<Vec<_>>().join("/"));
    }
    // A discard is no seat's: it shows its printed insight.
    let insight = if status == "discarded" { d.insight } else { play_insight(w, g, s, d) };
    for (r, n) in extra_costs_at(d, insight) {
        if n > 0 {
            bits.push(format!("+{}", r.words(n)));
        }
    }
    for (r, n) in &d.lowers {
        bits.push(format!("-{} income", r.words(*n)));
    }
    if !d.needs.is_empty() {
        let needs: Vec<String> = d.needs.iter().map(|(t, n)| format!("{n} {t}")).collect();
        bits.push(format!("needs {}", needs.join(", ")));
    }
    if let Some(t) = &d.replaces {
        bits.push(format!("built over a {t} building"));
    }
    let mut text = format!("[{}] {}", bits.join(", "), d.text);
    let mut name = d.name.clone();
    if let Some(t) = &d.tokens {
        let n = g.var(&tokens_key(s, id));
        text.push_str(&format!(" ({} on it)", t.kind.amount(n)));
        // The count on the card itself, as tokens sit on a printed card.
        if n > 0 && status != "discarded" && status != "draft" {
            name = format!("{} · {}", d.name, t.kind.amount(n));
        }
    }
    let under_ids = g.pile(&under(s, id));
    if !under_ids.is_empty() {
        let names: Vec<&str> = under_ids.iter().map(|c| def(w, c).name.as_str()).collect();
        text.push_str(&format!(" Splayed under it: {}.", names.join(", ")));
    }
    CardView {
        id: id.to_string(),
        name,
        text,
        status: status.to_string(),
        cost,
    }
}

pub(super) fn cards_view(m: &Cards, g: &GameState, w: &World, out: &mut ViewOut) {
    let card = |s: usize, id: &str, status: &str, cost: Option<i64>| card_view(w, g, s, id, status, cost);
    for s in 0..g.seats.len() {
        let hand_cards: Vec<CardView> = g
            .pile(&hand(s))
            .iter()
            .map(|id| {
                let st = blocked(w, g, s, id).unwrap_or("ready");
                card(s, id, st, Some(play_cost(w, g, s, def(w, id))))
            })
            .collect();
        out.cards(s, "hand", "hand", hand_cards);
        let row: Vec<CardView> = g
            .pile(&built(s))
            .iter()
            .map(|id| {
                let st = if def(w, id).ability.is_some() {
                    ability_blocked(w, g, s, id).unwrap_or("ready")
                } else if !g.pile(&under(s, id)).is_empty() {
                    "stacked"
                } else {
                    "built"
                };
                card(s, id, st, None)
            })
            .collect();
        let label = format!("buildings {}/{}", slotted(g, s).len(), capacity(w, g, s));
        out.cards(s, "built", &label, row);
        let progs: Vec<CardView> = g
            .pile(&played(s))
            .iter()
            .map(|id| card(s, id, "in play", None))
            .collect();
        if !progs.is_empty() {
            out.cards(s, "played", "programs in play", progs);
        }
        let pk: Vec<CardView> = g
            .pile(&packet(s))
            .iter()
            .map(|id| card(s, id, "draft", None))
            .collect();
        if !pk.is_empty() {
            out.cards(s, "packet", "draft hand", pk);
        }
        let dr: Vec<CardView> = g
            .pile(&drafted(s))
            .iter()
            .map(|id| {
                // 044: an Open Market free keep waives the price the status showed.
                let (ins, cr) = if alignment::free_purchases(g) > 0 { (0, 0) } else { m.keep_price(w, g, s, id) };
                let st = match (ins, cr) {
                    (0, 0) => "keep free".to_string(),
                    (i, 0) => format!("keep {i} insight"),
                    (0, c) => format!("keep {c} credits"),
                    (i, c) => format!("keep {i} insight + {c} credits"),
                };
                card(s, id, &st, None)
            })
            .collect();
        if !dr.is_empty() {
            let label = if g.round <= 1 { "opening hand (keep?)" } else { "drafted (keep?)" };
            out.cards(s, "drafted", label, dr);
        }
        out.seat(
            s,
            "slots",
            "building slots",
            capacity(w, g, s),
            None,
            "building slots (AI board + expansions)",
        );
        for tag in TAGS {
            let n = tags(w, g, s, tag);
            if n > 0 {
                out.seat(
                    s,
                    &format!("tag.{tag}"),
                    &format!("{tag} tags"),
                    n,
                    None,
                    "tags on your cards in play (buildings, splayed ones included, and programs)",
                );
            }
        }
        if g.var(&civic_used(s)) > 0 {
            out.seat(
                s,
                "civic_used",
                "Aegis Dome Array used",
                1,
                None,
                "your Aegis Dome Array has saved a facility this round",
            );
        }
    }
    // Per-seat markers: shown above as seat values, card text (tokens) or
    // the draft piles (`picked`).
    out.bookkeeping(&[
        "cards.picked.",
        "cards.civic_used.",
        "cards.kept_round.",
        "cards.tokens.",
    ]);
    for tier in [1u8, 2] {
        out.pile(
            &format!("deck.{tier}"),
            &format!("Tier-{tier} deck"),
            None,
            g.pile(&deck(tier)).len(),
            Vec::new(),
        );
        let d = g.pile(&discard_pile(tier));
        out.pile(
            &format!("discard.{tier}"),
            &format!("Tier-{tier} discard"),
            None,
            d.len(),
            d.iter().map(|id| card(0, id, "discarded", None)).collect(),
        );
    }
}

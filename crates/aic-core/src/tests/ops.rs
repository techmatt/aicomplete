//! The op schema and the word lists: what can go where, and the orderings
//! the state and the agents depend on.

use super::*;
use crate::content::{check_ops, Op, Slot, Tag, TagOrAny, RESOURCES, TAGS};
use crate::vocab::TokenKind;

#[test]
fn an_op_in_the_wrong_list_fails_to_load() {
    let bad = [
        (Op::EachDestroyNode { count: 1, bases: false }, Slot::Team),
        (Op::HardenAction { chips: 1 }, Slot::Threat),
        (Op::ThreatAtNode { tier: 2 }, Slot::Player),
        (Op::CutBotnet, Slot::Team),
        (Op::Sway { side: crate::content::Side::Unity, amount: 1, narrative: None }, Slot::Standing),
        (Op::Compute { amount: 1 }, Slot::Twist),
    ];
    for (op, slot) in bad {
        assert!(check_ops(std::slice::from_ref(&op), slot, "test").is_err(), "{op:?} in {slot:?}");
    }
}

#[test]
fn ops_check_the_resources_they_name() {
    let gives = Op::Gain { resource: Resource::Ambassadors, amount: 1 };
    assert!(check_ops(&[gives], Slot::Player, "test").is_ok());
    let income = Op::TeamIncome { resource: Resource::Energy, amount: 1 };
    assert!(check_ops(&[income], Slot::Team, "test").is_err());
    let each = Op::Troops {
        count: 1,
        each_country: true,
        destroy: false,
        at: None,
        stand: false,
        location: None,
        if_: None,
    };
    assert!(check_ops(&[each], Slot::Threat, "test").is_err());
}

#[test]
fn a_misspelled_word_does_not_load() {
    assert!(serde_json::from_str::<Op>(r#"{"op":"gain","resource":"credit","amount":1}"#).is_err());
    assert!(serde_json::from_str::<Tag>(r#""sciense""#).is_err());
    assert!(serde_json::from_str::<TokenKind>(r#""tokens""#).is_err());
    assert!(serde_json::from_str::<TagOrAny>(r#""all""#).is_err());
}

/// `Seat::resources` serializes in `Ord` order: the fingerprint depends on it
/// being the words' order.
#[test]
fn resources_order_as_their_words() {
    use Resource::*;
    let all = [Ambassadors, Antimatter, Chips, Credits, Energy, Insight];
    assert!(all.windows(2).all(|p| p[0] < p[1] && p[0].name() < p[1].name()));
    assert_eq!((Chips.amount(1), Chips.amount(2)), ("1 chip".to_string(), "2 chips".to_string()));
    for r in all {
        assert_eq!(serde_json::to_string(&r).unwrap(), format!("\"{}\"", r.name()));
        assert_eq!(r.is_player(), RESOURCES.contains(&r));
    }
    for t in TAGS {
        assert_eq!(serde_json::to_string(&t).unwrap(), format!("\"{}\"", t.name()));
        assert_eq!(serde_json::to_string(&TagOrAny::Is(t)).unwrap(), format!("\"{}\"", t.name()));
    }
    assert_eq!(serde_json::to_string(&TagOrAny::Any).unwrap(), "\"any\"");
}

/// Every number in row `v`'s ops, not its name or text, nor its own tier or
/// stage (135: a pull's tier, "a Security 2 threat", is printed and counts).
fn op_numbers(v: &serde_json::Value, out: &mut Vec<i64>) {
    fn nested(v: &serde_json::Value, out: &mut Vec<i64>) {
        match v {
            serde_json::Value::Number(n) => out.extend(n.as_i64()),
            serde_json::Value::Array(a) => a.iter().for_each(|x| nested(x, out)),
            // 146: an op also read back through `Op`, so a defaulted field
            // (Theft's `cards`, 2) counts as a number it has.
            serde_json::Value::Object(o) if o.contains_key("op") && serde_json::from_value::<Op>(v.clone()).is_ok() => {
                let op = serde_json::to_value(serde_json::from_value::<Op>(v.clone()).unwrap()).unwrap();
                for full in [v, &op] {
                    if let serde_json::Value::Object(f) = full {
                        f.values().for_each(|x| nested(x, out));
                    }
                }
            }
            serde_json::Value::Object(o) => o
                .iter()
                .filter(|(k, _)| !["id", "name", "text"].contains(&k.as_str()))
                .for_each(|(_, x)| nested(x, out)),
            _ => {}
        }
    }
    match v {
        serde_json::Value::Object(o) => o
            .iter()
            .filter(|(k, _)| !["id", "name", "text", "tier", "stage"].contains(&k.as_str()))
            .for_each(|(_, x)| nested(x, out)),
        other => nested(other, out),
    }
}

/// How many `lose_ambassador` ops row `v` holds, nested ones included.
fn lose_ambassadors(v: &serde_json::Value) -> i64 {
    match v {
        serde_json::Value::Array(a) => a.iter().map(lose_ambassadors).sum(),
        serde_json::Value::Object(o) => {
            i64::from(o.get("op").and_then(|x| x.as_str()) == Some("lose_ambassador"))
                + o.values().map(lose_ambassadors).sum::<i64>()
        }
        _ => 0,
    }
}

/// Numbers a text prints, a "3+" (a step's tip count, read off no op) aside.
fn text_numbers(text: &str) -> Vec<i64> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_digit() {
            let start = i;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            if b.get(i) != Some(&b'+') {
                out.push(text[start..i].parse().unwrap());
            }
        } else {
            i += 1;
        }
    }
    out
}

/// Only player-card text is generated: the hand-written rest must not print a
/// number its ops don't have (044's content sweep found none; keep it so).
/// A station flip prints `station_flip_at` (D39), which
/// `a_station_flip_face_prints_station_flip_at` keeps in step instead.
#[test]
fn hand_written_text_prints_no_number_its_ops_lack() {
    let n = Rules::new(&baseline()).unwrap().world().int("adversary.station_flip_at");
    let tunable = format!("unless {n} adversary");
    fn walk(v: &serde_json::Value, tunable: &str, bad: &mut Vec<String>) {
        match v {
            serde_json::Value::Array(a) => a.iter().for_each(|x| walk(x, tunable, bad)),
            serde_json::Value::Object(o) => {
                if let (Some(id), Some(text)) = (o.get("id"), o.get("text").and_then(|t| t.as_str())) {
                    let mut have = Vec::new();
                    op_numbers(v, &mut have);
                    // 142: `lose_ambassador` carries no number: each takes 1,
                    // and N of them print "loses N ambassadors".
                    let lose = lose_ambassadors(v);
                    if lose > 0 {
                        have.extend([1, lose]);
                    }
                    let text = text.replace(tunable, "");
                    let missing: Vec<i64> = text_numbers(&text).into_iter().filter(|n| !have.contains(n)).collect();
                    if !missing.is_empty() {
                        bad.push(format!("{id}: {missing:?} in {text:?}"));
                    }
                }
                o.values().for_each(|x| walk(x, tunable, bad));
            }
            _ => {}
        }
    }
    let mut bad = Vec::new();
    for file in [
        include_str!("../../../../content/threats.json"),
        include_str!("../../../../content/adversary.json"),
        include_str!("../../../../content/alignment.json"),
    ] {
        walk(&serde_json::from_str(file).unwrap(), &tunable, &mut bad);
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

//! The agents' card tests: which content cards count as capstones, engines and
//! liberate cards, read by the keep plan, the strategies and `aic-sim cards`.

use aic_core::content::{CardDef, CardKind, Op};

/// A tier-2 capstone (070): a tier-2 building whose standing effect ends an
/// engine: income per tag, an automatic counter or project drain.
pub(crate) fn is_capstone(d: &CardDef) -> bool {
    d.tier == 2
        && d.kind == CardKind::Building
        && d.passives
            .iter()
            .any(|p| matches!(p, Op::IncomePerTag { .. } | Op::AutoCounter { .. } | Op::ProjectDrain { .. }))
}

/// An engine card (107's probe): one whose standing effects fire on plays or
/// other events, fill a threshold, pay by tags or cut play costs, or whose play
/// raises an income by tags or copies one.
pub(crate) fn is_engine(d: &CardDef) -> bool {
    d.passives.iter().any(|p| {
        matches!(p, Op::When { .. } | Op::Threshold { .. } | Op::IncomePerTag { .. } | Op::CardDiscount { .. })
    }) || d.effects.iter().any(|e| matches!(e, Op::ProductionPerTag { .. } | Op::CopyIncome))
}

/// 119: `aic-sim cards`: every content card with the agents' card tests
/// ([`is_engine`], [`is_liberate`]), so a script reads them here instead of
/// copying them (108 copied `is_engine` into Python twice).
pub fn cards_csv() -> String {
    let mut out = String::from("id,kind,tier,engine,liberate\n");
    for d in &aic_core::content::content().cards {
        let kind = serde_json::to_value(d.kind).ok().and_then(|v| v.as_str().map(str::to_string)).unwrap_or_default();
        out.push_str(&format!("{},{kind},{},{},{}\n", d.id, d.tier, is_engine(d) as u8, is_liberate(d) as u8));
    }
    out
}

/// A liberate card (080): one that liberates a corrupted city when played or
/// by its Action, in any of its branches (Great Liberation's cleanse of cities too).
pub(crate) fn is_liberate(d: &CardDef) -> bool {
    let lib = |e: &Op| matches!(e, Op::Liberate { .. } | Op::Cleanse { cities: true, .. });
    let ability = d.ability.iter().flat_map(|a| a.branches()).flat_map(|b| &b.effects);
    d.effects.iter().chain(ability).any(lib)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 119: `aic-sim cards` lists every content card once, with the agents' own tests.
    #[test]
    fn the_cards_listing_carries_the_engine_test() {
        let csv = cards_csv();
        let rows: Vec<&str> = csv.lines().skip(1).collect();
        let all = &aic_core::content::content().cards;
        assert_eq!(rows.len(), all.len());
        let engines = rows.iter().filter(|r| r.split(',').nth(3) == Some("1")).count();
        assert_eq!(engines, all.iter().filter(|d| is_engine(d)).count());
        assert!(engines > 0 && rows.iter().all(|r| r.split(',').count() == 5), "{csv}");
    }
}

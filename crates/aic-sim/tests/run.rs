//! Runs the built binary: what `run` reports.

mod common;

use common::{assert_ok, sim};

/// 020: a `*_round` counter ("the round of the first corruption") is a mean over
/// the games that reached it; averaged over all games, short games without one
/// pulled it toward round 0.
#[test]
fn round_counters_average_over_the_games_that_reached_them() {
    let out = sim()
        .args(["run", "configs/baseline.json", "--games", "12", "--json"])
        .args(["--set", "countries=1", "--set", "adversary.difficulty=4"])
        .output()
        .expect("runs aic-sim");
    assert_ok(&out);
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let counters = &v["results"][0]["counters"];
    // Averages of round numbers, so never below round 1 (024: at 1 country a
    // botnet's 3 compute and a threat's 3 can resolve a project in round 1).
    for k in ["adversary.first_corruption_round", "adversary.first_project_round"] {
        if let Some(x) = counters[k].as_f64() {
            assert!(x >= 1.0, "{k} {x}");
        }
    }
}

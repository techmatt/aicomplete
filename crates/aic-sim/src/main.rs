//! Simulation CLI: plays seeded games for one or two configs with a
//! selectable team agent (random by default), prints win rate, game length
//! and counters side by side, and replay-checks every game; runs experiment
//! grids into a cycle report. See docs/simulator.md for usage.
//!
//! The code is the library (`lib.rs`), so the farm's wasm bundle
//! (`crates/aic-farm`) plays the same games (160).

fn main() {
    aic_sim::main()
}

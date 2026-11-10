//! The clock a game's timings read (160). Native: `std::time::Instant`. wasm32
//! has no clock (`Instant::now` panics there), so a game played in the farm's
//! bundle reads every timing as 0: its timings are on the worker's side, not
//! in its game lines.

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, Debug)]
pub struct Instant;

#[cfg(target_arch = "wasm32")]
impl Instant {
    pub fn now() -> Instant {
        Instant
    }

    pub fn elapsed(&self) -> std::time::Duration {
        std::time::Duration::ZERO
    }
}

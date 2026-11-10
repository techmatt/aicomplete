//! The float functions beyond IEEE's exact ones (+ - * / sqrt) that the sim
//! calls (160). By default they are std's: on native, the platform's math
//! library; on wasm32, Rust's port of musl's `libm`. The `libm` build flag
//! (`cargo build --features libm -p aic-sim`, not the default) makes native
//! call that same `libm`, for a native worker that computes exactly as a wasm
//! one. A game line reads only `powi` (the eval's share squared, exact either
//! way); `exp` and `ln` are the report's.

#[cfg(feature = "libm")]
pub fn powi(x: f64, n: i32) -> f64 {
    libm::pow(x, n as f64)
}

#[cfg(not(feature = "libm"))]
pub fn powi(x: f64, n: i32) -> f64 {
    x.powi(n)
}

#[cfg(feature = "libm")]
pub fn exp(x: f64) -> f64 {
    libm::exp(x)
}

#[cfg(not(feature = "libm"))]
pub fn exp(x: f64) -> f64 {
    x.exp()
}

#[cfg(feature = "libm")]
pub fn ln(x: f64) -> f64 {
    libm::log(x)
}

#[cfg(not(feature = "libm"))]
pub fn ln(x: f64) -> f64 {
    x.ln()
}

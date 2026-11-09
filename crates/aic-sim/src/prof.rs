//! Opt-in timing of the search's hot calls: build with `--features profile`
//! and `experiment` prints where the agents' time went, with call counts, and
//! what the process allocated. Off, [`time`] is the call itself and costs
//! nothing.
//!
//! Parts nest: `line` holds the planner's rollouts whole, and the rest of its
//! time shows up again under `apply`, `apply_mut`, `legal` and `eval`; `top` and
//! `extend` are the rest of the planner, and `score_*` an action's whole scoring by kind. `clone`
//! is timed on a throwaway copy beside each `apply` (030), so `apply` less
//! `clone` is the rules; that copy adds its own time to the run, and the
//! timers cost about 50 ns a call.
//!
//! Each worker thread adds to its own counters and merges them into the
//! process total with [`flush`] when it runs out of games.

#[derive(Clone, Copy)]
pub enum Part {
    Apply,
    ApplyMut,
    #[cfg_attr(not(feature = "profile"), allow(dead_code))]
    Clone,
    Legal,
    Eval,
    Determinize,
    Line,
    Sync,
    Policy,
    /// The planner's own scoring of a decision (`greedy_scores` at the top level).
    Top,
    /// A followed line run on to the new stop (`Planner::incumbent`).
    Extend,
    /// One legal action's scoring, whole, by its kind (069).
    ScorePower,
    ScorePlay,
    ScoreStandard,
    ScoreChoose,
    ScoreOther,
}

/// Each [`Part`]'s name, in its order (`Part as usize` indexes it).
#[cfg(feature = "profile")]
pub const PARTS: [&str; 16] = [
    "apply", "apply_mut", "clone", "legal", "eval", "determinize", "line", "sync", "policy", "top",
    "extend", "score_power", "score_play", "score_standard", "score_choose", "score_other",
];

// A part added to `Part` needs its name in `PARTS`.
#[cfg(feature = "profile")]
const _: () = assert!(PARTS.len() == Part::ScoreOther as usize + 1);

#[cfg(feature = "profile")]
mod on {
    use std::alloc::{GlobalAlloc, Layout};
    use std::cell::RefCell;
    use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
    use std::sync::Mutex;

    pub type Totals = [(f64, u64); super::PARTS.len()];

    thread_local! {
        pub static LOCAL: RefCell<Totals> = const { RefCell::new([(0.0, 0); super::PARTS.len()]) };
    }
    pub static TOTAL: Mutex<Totals> = Mutex::new([(0.0, 0); super::PARTS.len()]);

    pub static ALLOCS: AtomicU64 = AtomicU64::new(0);
    pub static BYTES: AtomicU64 = AtomicU64::new(0);

    /// mimalloc, counted.
    pub struct Counting;

    // SAFETY: passes every call straight to mimalloc.
    unsafe impl GlobalAlloc for Counting {
        unsafe fn alloc(&self, l: Layout) -> *mut u8 {
            ALLOCS.fetch_add(1, Relaxed);
            BYTES.fetch_add(l.size() as u64, Relaxed);
            mimalloc::MiMalloc.alloc(l)
        }
        unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
            mimalloc::MiMalloc.dealloc(p, l)
        }
        unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
            ALLOCS.fetch_add(1, Relaxed);
            BYTES.fetch_add(n as u64, Relaxed);
            mimalloc::MiMalloc.realloc(p, l, n)
        }
    }
}

#[cfg(feature = "profile")]
pub use on::Counting;

/// Runs `f`, adding its time to `part` when profiling.
#[inline(always)]
pub fn time<R>(part: Part, f: impl FnOnce() -> R) -> R {
    #[cfg(feature = "profile")]
    {
        let t = std::time::Instant::now();
        let r = f();
        let ms = t.elapsed().as_secs_f64() * 1000.0;
        on::LOCAL.with(|l| {
            let e = &mut l.borrow_mut()[part as usize];
            e.0 += ms;
            e.1 += 1;
        });
        r
    }
    #[cfg(not(feature = "profile"))]
    {
        let _ = part;
        f()
    }
}

/// Merges this thread's counters into the process total.
pub fn flush() {
    #[cfg(feature = "profile")]
    on::LOCAL.with(|l| {
        let mut l = l.borrow_mut();
        let mut t = on::TOTAL.lock().expect("profile lock");
        for (a, b) in t.iter_mut().zip(l.iter_mut()) {
            a.0 += b.0;
            a.1 += b.1;
            *b = (0.0, 0);
        }
    });
}

/// The process total, `(part, ms, calls)`; `None` unless built with `profile`.
pub fn totals() -> Option<Vec<(&'static str, f64, u64)>> {
    #[cfg(feature = "profile")]
    {
        let t = on::TOTAL.lock().expect("profile lock");
        Some(
            PARTS
                .iter()
                .zip(t.iter())
                .map(|(n, (ms, c))| (*n, *ms, *c))
                .collect(),
        )
    }
    #[cfg(not(feature = "profile"))]
    None
}

/// What the process allocated so far: `(allocations, bytes)`.
pub fn allocated() -> Option<(u64, u64)> {
    #[cfg(feature = "profile")]
    {
        use std::sync::atomic::Ordering::Relaxed;
        Some((on::ALLOCS.load(Relaxed), on::BYTES.load(Relaxed)))
    }
    #[cfg(not(feature = "profile"))]
    None
}

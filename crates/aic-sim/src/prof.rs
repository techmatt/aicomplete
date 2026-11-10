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
//!
//! 165 (162's ask): [`prompt`] also tallies each prompt's scorings (an answer
//! to "discard which card (1 of 3)?" and the like) by its text, digits as `#`:
//! decisions, applies, evals and time, so the costly prompt is named before
//! anyone optimizes (docs/speed.md).

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

    /// A prompt's (decisions, applies, evals, ms), by its text with digits as `#`.
    pub type Prompts = std::collections::BTreeMap<String, (u64, u64, u64, f64)>;

    thread_local! {
        pub static LOCAL_PROMPTS: RefCell<Prompts> = const { RefCell::new(Prompts::new()) };
        /// Scorings under way on this thread: only the outermost is tallied.
        pub static DEPTH: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    }
    pub static TOTAL_PROMPTS: Mutex<Prompts> = Mutex::new(Prompts::new());

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

/// Runs `f`, a decision's whole scoring, and (profiling, outermost only)
/// tallies it under `prompt`, the open question's text: none, not a prompt.
#[inline(always)]
pub fn prompt<R>(prompt: Option<&str>, f: impl FnOnce() -> R) -> R {
    #[cfg(feature = "profile")]
    if let Some(text) = prompt {
        if on::DEPTH.get() == 0 {
            let counts = || {
                on::LOCAL.with(|l| {
                    let l = l.borrow();
                    (l[Part::Apply as usize].1 + l[Part::ApplyMut as usize].1, l[Part::Eval as usize].1)
                })
            };
            let (applies, evals) = counts();
            let t = std::time::Instant::now();
            on::DEPTH.set(1);
            let r = f();
            on::DEPTH.set(0);
            let ms = t.elapsed().as_secs_f64() * 1000.0;
            let (applies2, evals2) = counts();
            let key: String = text.chars().map(|c| if c.is_ascii_digit() { '#' } else { c }).collect();
            on::LOCAL_PROMPTS.with(|p| {
                let mut p = p.borrow_mut();
                let e = p.entry(key).or_default();
                e.0 += 1;
                e.1 += applies2 - applies;
                e.2 += evals2 - evals;
                e.3 += ms;
            });
            return r;
        }
    }
    let _ = prompt;
    f()
}

/// Merges this thread's counters into the process total.
pub fn flush() {
    #[cfg(feature = "profile")]
    on::LOCAL_PROMPTS.with(|l| {
        let mut l = l.borrow_mut();
        let mut t = on::TOTAL_PROMPTS.lock().expect("profile lock");
        for (k, (d, a, e, ms)) in std::mem::take(&mut *l) {
            let x = t.entry(k).or_default();
            x.0 += d;
            x.1 += a;
            x.2 += e;
            x.3 += ms;
        }
    });
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

/// One prompt's tally: `(prompt, decisions, applies, evals, ms)`.
pub type PromptRow = (String, u64, u64, u64, f64);

/// The process total by prompt, the costliest first; `None` unless built
/// with `profile`.
pub fn prompts() -> Option<Vec<PromptRow>> {
    #[cfg(feature = "profile")]
    {
        let t = on::TOTAL_PROMPTS.lock().expect("profile lock");
        let mut v: Vec<_> = t.iter().map(|(k, (d, a, e, ms))| (k.clone(), *d, *a, *e, *ms)).collect();
        v.sort_by(|a, b| b.4.total_cmp(&a.4));
        Some(v)
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

//! Dominance-gate early stopping (`docs/principles.md`, *Dominance checks*):
//! a spec's `gate` runs each check (`planner:never_tag=1`) against its own
//! agent (`planner`) on the same seeds, and stops the check once its verdict
//! is decided at the stated noise. The verdict is the sign of the gap at the
//! cap (`games`): the agent's wins minus the check's.
//!
//! Seeds go in blocks and decisions are taken only between blocks, on finished
//! blocks. So which games a run plays depends on the games alone, never on
//! timing or the thread count, and [`evaluate`] rebuilds every decision from
//! the games (a resumed run, a report mid-run). Inside a block the order is
//! still seed by seed, so a run stopped mid-block keeps every live cell's
//! first seeds; within a seed the agents go first, then the checks that look
//! most decided per millisecond (the order changes when games finish, not
//! which are played).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::experiment::{Cell, FailedGame, GameResult, Spec};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Gate {
    /// The stated noise: how many standard errors decide a verdict.
    pub z: f64,
    /// Seeds played before the first decision.
    pub min_games: u64,
    /// Seeds between decisions after that.
    pub every: u64,
}

/// `z` 2 over about 7 looks: see *Experiments* in the README for what that
/// costs in flipped verdicts.
impl Default for Gate {
    fn default() -> Self {
        Gate {
            z: 2.0,
            min_games: 8,
            every: 4,
        }
    }
}

impl Gate {
    /// The seed blocks, as offsets from the first seed.
    pub fn blocks(&self, cap: u64) -> Vec<(u64, u64)> {
        let mut out = Vec::new();
        let (mut from, mut to) = (0, self.min_games.max(1).min(cap));
        while from < cap {
            out.push((from, to));
            from = to;
            to = (to + self.every.max(1)).min(cap);
        }
        out
    }

    /// Whether `d` (wins of the agent minus the check's, `k` seeds won by just
    /// one of them, over `n` seeds of `cap`) settles the verdict: the gap
    /// clears the noise now (`d / sqrt(k)`, the sign test's z), or the seeds
    /// left can't turn it back (they add a sum of `r` steps, a share `k / n`
    /// of them discordant and the rest level, so a spread of `sqrt(r k / n)`).
    pub fn decided(&self, d: i64, k: u64, n: u64, cap: u64) -> Option<Decided> {
        let m = d.unsigned_abs() as f64;
        if d == 0 || n == 0 {
            return None;
        }
        if m >= self.z * (k as f64).sqrt() {
            return Some(Decided::ClearsNoise);
        }
        let left = (cap - n.min(cap)) as f64 * k as f64 / n as f64;
        (m >= self.z * left.sqrt()).then_some(Decided::SignSettled)
    }
}

/// Why a check stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Decided {
    ClearsNoise,
    SignSettled,
}

/// One check's row.
#[derive(Debug, Clone, Serialize)]
pub struct CheckRow {
    pub config: String,
    pub variant: String,
    pub difficulty: i64,
    pub check: String,
    pub agent: String,
    pub cell: usize,
    pub agent_cell: usize,
    /// Seeds both played, so far.
    pub games: u64,
    pub cap: u64,
    /// The seeds it stopped at, with why; `None` while it runs on or at the cap.
    pub stopped_at: Option<u64>,
    pub decided: Option<Decided>,
    pub agent_win_rate: f64,
    pub check_win_rate: f64,
    pub only_agent: u64,
    pub only_check: u64,
    /// Agent minus check, on the seeds both played, ± the unpaired standard
    /// error the gate tables print, then the paired one.
    pub gap: f64,
    pub se: f64,
    pub paired_se: f64,
    /// The check loses ground: gap above 0.
    pub loses: bool,
    /// Summed `ms` of its games, and what the cap would have cost at its mean.
    pub ms: f64,
    pub ms_at_cap: f64,
}

/// A gate's standing: every check, and which cells are done.
#[derive(Debug, Clone, Serialize)]
pub struct GateState {
    pub gate: Gate,
    pub rows: Vec<CheckRow>,
    /// Per cell, the seeds it plays: its stop, or the cap.
    pub plays: Vec<u64>,
    /// Games played, and games at the cap for every cell.
    pub games: usize,
    pub games_cap: usize,
    /// Of all finished games' time, in seconds, and an estimate of the same at
    /// the cap (each stopped cell's mean game times the seeds it skipped).
    pub seconds: f64,
    pub seconds_at_cap: f64,
    /// Games that panicked: their seeds count for neither side.
    pub failed: usize,
    /// Every check is decided or at the cap; every one loses ground, and no
    /// game panicked.
    pub finished: bool,
    pub passes: bool,
    #[serde(skip)]
    cap: u64,
    #[serde(skip)]
    agents: BTreeSet<usize>,
}

impl GateState {
    /// Cells still playing at seed offset `from`: a stopped check isn't, nor is
    /// an agent all of whose checks have stopped.
    pub fn live_at(&self, from: u64) -> Vec<usize> {
        (0..self.plays.len()).filter(|&c| self.plays[c] > from).collect()
    }

    /// Each cell's place within a seed: agents, then checks by how decided
    /// they look per unit of cost; a cell with no games yet in spec order.
    pub fn order(&self, cells: &[Cell], games: &[GameResult]) -> Vec<usize> {
        let mut ms = vec![(0.0, 0.0); cells.len()];
        for g in games {
            ms[g.cell].0 += g.ms;
            ms[g.cell].1 += 1.0;
        }
        let mut score = vec![0.0f64; cells.len()];
        for r in &self.rows {
            let k = (r.only_agent + r.only_check).max(1) as f64;
            let z = (r.only_agent as f64 - r.only_check as f64).abs() / k.sqrt();
            let (t, n) = ms[r.cell];
            score[r.cell] = if n > 0.0 { (1.0 + z) * n / t.max(1e-9) } else { 0.0 };
        }
        let mut by: Vec<usize> = (0..cells.len()).collect();
        by.sort_by(|&a, &b| {
            let is_check = |c: &usize| !self.agents.contains(c);
            is_check(&a)
                .cmp(&is_check(&b))
                .then(score[b].total_cmp(&score[a]))
                .then(a.cmp(&b))
        });
        let mut place = vec![0; cells.len()];
        for (i, c) in by.into_iter().enumerate() {
            place[c] = i;
        }
        place
    }

    /// Games the run plays as things stand, for "partial" in the report.
    pub fn planned(&self) -> usize {
        self.plays.iter().map(|&n| n as usize).sum()
    }

    /// `report.md`'s section.
    pub fn markdown(&self) -> String {
        let mut w = String::new();
        let g = &self.gate;
        let _ = writeln!(w, "\n## Gate\n");
        let _ = writeln!(
            w,
            "Each check against its own agent on the same seeds; gap: the agent's win % minus the check's, ± one standard error (unpaired, as the gate tables have it). The verdict is the gap's sign at the cap of {} seeds. A check stops, looking at {} seeds and every {} after, once the gap clears {} standard errors (sign test) or the seeds left can't turn it back at that noise.\n",
            self.cap, g.min_games, g.every, g.z
        );
        let _ = writeln!(
            w,
            "{} of {} games ({:.0}%), {:.0} s of game time of about {:.0} s at the cap ({:.0}%). {}\n",
            self.games,
            self.games_cap,
            100.0 * self.games as f64 / self.games_cap.max(1) as f64,
            self.seconds,
            self.seconds_at_cap,
            100.0 * self.seconds / self.seconds_at_cap.max(1e-9),
            match (self.finished, self.passes, self.failed) {
                (false, ..) => "**Not finished.**".to_string(),
                (true, true, _) => "**Passes:** every check loses ground.".to_string(),
                (true, false, 0) => "**Does not pass:** see the ✗ rows.".to_string(),
                (true, false, n) => format!("**Does not pass:** {n} game(s) panicked, see *Failed games*."),
            }
        );
        let _ = writeln!(w, "| variant | d | check | games | stopped | win % | gap to its agent | paired se |");
        let _ = writeln!(w, "|---|---|---|---|---|---|---|---|");
        // 044: with several configs, rows of one check differ only by config.
        let several = self.rows.iter().any(|r| r.config != self.rows[0].config);
        for r in &self.rows {
            let variant = if several { format!("{} ({})", r.variant, r.config) } else { r.variant.clone() };
            let stop = match r.decided {
                Some(Decided::ClearsNoise) => "clears the noise",
                Some(Decided::SignSettled) => "sign settled",
                None if r.games < r.cap => "running",
                None => "cap",
            };
            let _ = writeln!(
                w,
                "| {} | {} | {} | {} | {stop} | {:.0} | {:+.0} ±{:.0}{} | {:.0} |",
                variant,
                r.difficulty,
                r.check,
                r.games,
                100.0 * r.check_win_rate,
                100.0 * r.gap + 0.0,
                100.0 * r.se,
                if r.loses { "" } else { " ✗" },
                100.0 * r.paired_se
            );
        }
        w
    }
}

/// The check's agent in its group of cells, by label: `planner` of
/// `planner:never_tag=1`.
fn agent_of(cells: &[Cell], c: usize) -> Option<usize> {
    let label = cells[c].agent.label();
    let (kind, _) = label.split_once(':')?;
    let group = cells[c].group();
    (0..cells.len()).find(|&i| cells[i].group() == group && cells[i].agent.label() == kind)
}

/// A gate needs agents to measure against: every `x:...` agent's `x` in the
/// spec, and at least one such pair. A check with no agent would run to the
/// cap, off the gate's table.
pub fn check_spec(cells: &[Cell]) -> Result<(), String> {
    let mut pairs = 0;
    for c in 0..cells.len() {
        let label = cells[c].agent.label();
        match (label.contains(':'), agent_of(cells, c)) {
            (true, None) => {
                let kind = label.split(':').next().unwrap_or("");
                return Err(format!("gate: check {label:?} needs the agent {kind:?} in the spec"));
            }
            (true, Some(_)) => pairs += 1,
            (false, _) => {}
        }
    }
    if pairs == 0 {
        return Err("gate: no checks (an `x:...` agent beside its `x`)".into());
    }
    Ok(())
}

/// A check's decision, block by block over the seeds both have played in
/// full (a seed that panicked is played, and counts for neither): why it
/// stops, and the look it stops at.
fn decide(
    gate: &Gate,
    (agent, agent_failed): (&BTreeMap<u64, bool>, &BTreeSet<u64>),
    (check, check_failed): (&BTreeMap<u64, bool>, &BTreeSet<u64>),
    looks: &[u64],
    cap: u64,
) -> Option<(Decided, u64)> {
    let full = |m: &BTreeMap<u64, bool>, f: &BTreeSet<u64>, n: u64| {
        (0..n).all(|k| m.contains_key(&k) || f.contains(&k))
    };
    let (mut oa, mut oc, mut n) = (0u64, 0u64, 0u64);
    for &look in looks {
        if !full(check, check_failed, look) || !full(agent, agent_failed, look) {
            break;
        }
        while n < look {
            match (agent.get(&n), check.get(&n)) {
                (Some(true), Some(false)) => oa += 1,
                (Some(false), Some(true)) => oc += 1,
                _ => {}
            }
            n += 1;
        }
        if look >= gate.min_games && look < cap {
            if let Some(d) = gate.decided(oa as i64 - oc as i64, oa + oc, n, cap) {
                return Some((d, look));
            }
        }
    }
    None
}

/// The table's row for check `c` against its agent `a`, passed as `(a, c)`.
/// Rows read every seed both played, finished block or not.
fn check_row(
    cells: &[Cell],
    (a, c): (usize, usize),
    won: &[BTreeMap<u64, bool>],
    ms: &[f64],
    cap: u64,
    stop: Option<(Decided, u64)>,
) -> CheckRow {
    let seeds: Vec<u64> = won[c].keys().filter(|k| won[a].contains_key(k)).copied().collect();
    let m = seeds.len() as u64;
    let wins = |x: usize| seeds.iter().filter(|k| won[x][k]).count() as f64;
    let nf = m.max(1) as f64;
    let (p, q) = (wins(a) / nf, wins(c) / nf);
    let (oa, oc) = seeds.iter().fold((0u64, 0u64), |(x, y), k| match (won[a][k], won[c][k]) {
        (true, false) => (x + 1, y),
        (false, true) => (x, y + 1),
        _ => (x, y),
    });
    let gap = p - q;
    let var = ((oa + oc) as f64 - nf * gap * gap) / (nf - 1.0).max(1.0);
    CheckRow {
        config: cells[c].config_path.clone(),
        variant: cells[c].variant.clone(),
        difficulty: cells[c].difficulty,
        check: cells[c].agent.label(),
        agent: cells[a].agent.label(),
        cell: c,
        agent_cell: a,
        games: m,
        cap,
        stopped_at: stop.map(|(_, look)| look),
        decided: stop.map(|(d, _)| d),
        agent_win_rate: p,
        check_win_rate: q,
        only_agent: oa,
        only_check: oc,
        gap,
        se: ((p * (1.0 - p) + q * (1.0 - q)) / nf).sqrt(),
        paired_se: (var.max(0.0) / nf).sqrt(),
        loses: gap > 0.0,
        ms: ms[c],
        ms_at_cap: ms[c] / nf * cap as f64,
    }
}

/// Every decision so far, rebuilt from `games`.
pub fn evaluate(
    gate: &Gate,
    spec: &Spec,
    cells: &[Cell],
    games: &[GameResult],
    failed: &[FailedGame],
) -> GateState {
    let cap = spec.games;
    let mut lost: Vec<BTreeSet<u64>> = vec![BTreeSet::new(); cells.len()];
    for f in failed {
        lost[f.cell].insert(f.seed - spec.first_seed);
    }
    let mut won: Vec<BTreeMap<u64, bool>> = vec![BTreeMap::new(); cells.len()];
    let mut ms = vec![0.0; cells.len()];
    for g in games {
        won[g.cell].insert(g.seed - spec.first_seed, g.won);
        ms[g.cell] += g.ms;
    }
    let looks: Vec<u64> = gate.blocks(cap).iter().map(|b| b.1).collect();
    let mut plays = vec![cap; cells.len()];
    let mut rows = Vec::new();
    let mut agents = BTreeSet::new();
    let mut checks_of: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for c in 0..cells.len() {
        let Some(a) = agent_of(cells, c) else { continue };
        agents.insert(a);
        checks_of.entry(a).or_default().push(c);
        let stop = decide(gate, (&won[a], &lost[a]), (&won[c], &lost[c]), &looks, cap);
        if let Some((_, look)) = stop {
            plays[c] = look;
        }
        rows.push(check_row(cells, (a, c), &won, &ms, cap, stop));
    }
    // An agent plays as far as its last check.
    for (&a, checks) in &checks_of {
        plays[a] = checks.iter().map(|&c| plays[c]).max().unwrap_or(cap);
    }
    let played: Vec<usize> = won.iter().map(BTreeMap::len).collect();
    let seconds = ms.iter().sum::<f64>() / 1000.0;
    let seconds_at_cap = (0..cells.len())
        .map(|c| ms[c] / played[c].max(1) as f64 * cap as f64)
        .sum::<f64>()
        / 1000.0;
    let finished = (0..cells.len()).all(|c| (played[c] + lost[c].len()) as u64 >= plays[c]);
    GateState {
        gate: gate.clone(),
        passes: finished && failed.is_empty() && !rows.is_empty() && rows.iter().all(|r| r.loses),
        rows,
        plays,
        games: games.len(),
        games_cap: spec.total_games(cells),
        seconds,
        seconds_at_cap,
        failed: failed.len(),
        finished,
        cap,
        agents,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment::{cells, Variant};

    fn spec(agents: &[&str], games: u64) -> Spec {
        Spec {
            name: "t".into(),
            note: String::new(),
            configs: vec!["../../configs/baseline.json".into()],
            variants: vec![Variant { name: "base".into(), set: Vec::new() }],
            difficulties: vec![2],
            agents: agents.iter().map(|s| s.to_string()).collect(),
            first_seed: 100,
            games,
            max_actions: 50,
            winprob: Default::default(),
            closeness: Default::default(),
            records: 0,
            selection_seeds: None,
            gate: Some(Gate::default()),
            inject_panic: None,
        }
    }

    fn game(cell: usize, seed: u64, won: bool) -> GameResult {
        let mut g: GameResult = serde_json::from_value(serde_json::json!({
            "cell": cell, "seed": seed, "won": won, "reason": "x", "rounds": 1, "decisions": 1,
            "options_total": 1, "forced": 0, "probed": 0, "close": 0, "gap_sum": 0.0,
            "margin": {"deck_left": 0, "supply_left": 0, "supply_start": 0,
                "cities_corrupted_end": 0, "cities_aligned_end": 0, "blockers": 0, "level": 0,
                "unity_tips": 0, "nodes_end": 0, "troops_end": 0},
            "adversary_ops": [], "setup_ops": 0, "counters": {}, "cards": {}, "gives": 0,
            "pays": 0, "withdraws": 0, "build": "", "winprob": [], "ms": 1000.0, "agent_ms": 0.0,
            "probe_ms": 0.0, "setup_ms": 0.0, "legal_ms": 0.0, "apply_ms": 0.0, "replay_ms": 0.0,
            "replay_ok": true, "actions": []
        }))
        .unwrap();
        g.final_state = None;
        g
    }

    #[test]
    fn blocks_cover_the_seeds() {
        let g = Gate::default();
        assert_eq!(g.blocks(16), [(0, 8), (8, 12), (12, 16)]);
        assert_eq!(g.blocks(10), [(0, 8), (8, 10)]);
        assert_eq!(g.blocks(4), [(0, 4)]);
    }

    #[test]
    fn a_lopsided_gap_decides_and_a_level_one_never_does() {
        let g = Gate::default();
        assert_eq!(g.decided(5, 5, 8, 32), Some(Decided::ClearsNoise));
        assert_eq!(g.decided(-5, 5, 8, 32), Some(Decided::ClearsNoise));
        assert_eq!(g.decided(0, 8, 8, 32), None);
        assert_eq!(g.decided(2, 4, 16, 32), None);
        // Three up with two seeds left and half the seeds discordant.
        assert_eq!(g.decided(3, 15, 30, 32), Some(Decided::SignSettled));
    }

    #[test]
    fn a_check_stops_its_agent_runs_on_and_decisions_rebuild_from_the_games() {
        let s = spec(&["planner", "planner:no_counters=1", "planner:never_tag=1"], 16);
        let cs = cells(&s).unwrap();
        let g = s.gate.as_ref().unwrap();
        let (agent, lost, level) = (0, 1, 2);
        assert_eq!(evaluate(g, &s, &cs, &[], &[]).live_at(0), [0, 1, 2]);
        let mut games = Vec::new();
        for k in 0..8u64 {
            games.push(game(agent, 100 + k, true));
            games.push(game(lost, 100 + k, false));
            games.push(game(level, 100 + k, true));
        }
        let st = evaluate(g, &s, &cs, &games, &[]);
        assert_eq!(st.live_at(8), [agent, level]);
        assert_eq!(st.plays, [16, 8, 16]);
        assert!(!st.finished);
        assert_eq!(st.planned(), 40);
        for k in 8..16u64 {
            games.push(game(agent, 100 + k, true));
            games.push(game(level, 100 + k, true));
        }
        let st = evaluate(g, &s, &cs, &games, &[]);
        assert!(st.finished);
        assert!(!st.passes, "a level check doesn't lose ground");
        assert!(st.live_at(16).is_empty());
        let row = st.rows.iter().find(|r| r.cell == lost).unwrap();
        assert_eq!((row.games, row.stopped_at), (8, Some(8)));
        assert_eq!(row.decided, Some(Decided::ClearsNoise));
        assert_eq!(row.gap, 1.0);
        assert_eq!(st.games, 40);
        assert_eq!(st.games_cap, 48);
        assert!((st.seconds_at_cap - 48.0).abs() < 1e-9);
    }

    /// A block half played decides nothing, whichever games happen to be done.
    #[test]
    fn an_unfinished_block_decides_nothing() {
        let s = spec(&["planner", "planner:no_counters=1"], 16);
        let cs = cells(&s).unwrap();
        let g = s.gate.as_ref().unwrap();
        let mut games = Vec::new();
        for k in 0..7u64 {
            games.push(game(0, 100 + k, true));
            games.push(game(1, 100 + k, false));
        }
        assert_eq!(evaluate(g, &s, &cs, &games, &[]).plays, [16, 16]);
    }

    /// One panicked seed neither stalls the check nor counts for a side.
    #[test]
    fn a_panicked_seed_counts_for_neither_and_the_gate_finishes_unpassed() {
        let s = spec(&["planner", "planner:no_counters=1"], 8);
        let cs = cells(&s).unwrap();
        let g = s.gate.as_ref().unwrap();
        let mut games = Vec::new();
        for k in 0..8u64 {
            games.push(game(0, 100 + k, true));
            if k != 3 {
                games.push(game(1, 100 + k, false));
            }
        }
        let lost: FailedGame = serde_json::from_value(serde_json::json!({
            "cell": 1, "seed": "103", "config": "c", "variant": "base", "difficulty": 2,
            "agent": "planner:no_counters=1", "overrides": [], "message": "boom"
        }))
        .unwrap();
        let st = evaluate(g, &s, &cs, &games, &[lost]);
        assert!(st.finished);
        assert!(!st.passes, "a panicked game fails the gate");
        assert_eq!((st.rows[0].games, st.rows[0].only_agent), (7, 7));
        assert!(st.markdown().contains("1 game(s) panicked"));
    }

    #[test]
    fn a_gate_with_a_check_and_no_agent_is_refused() {
        assert!(cells(&spec(&["planner", "planner:no_counters=1"], 16)).is_ok());
        let e = |a: &[&str]| cells(&spec(a, 16)).map(|_| ()).unwrap_err();
        assert!(e(&["planner:no_counters=1"]).contains("needs the agent"));
        assert!(e(&["planner"]).contains("no checks"));
    }

    #[test]
    fn agents_come_first_in_a_seed() {
        let s = spec(&["planner:no_counters=1", "planner"], 16);
        let cs = cells(&s).unwrap();
        let st = evaluate(s.gate.as_ref().unwrap(), &s, &cs, &[], &[]);
        assert_eq!(st.order(&cs, &[]), [1, 0]);
    }

    #[test]
    fn a_spec_with_no_gate_field_reads_as_none_and_writes_none() {
        let s: Spec = serde_json::from_str(
            r#"{"name":"x","configs":["c"],"agents":["greedy"],"games":1}"#,
        )
        .unwrap();
        assert!(s.gate.is_none());
        assert!(!serde_json::to_string(&s).unwrap().contains("gate"));
    }
}

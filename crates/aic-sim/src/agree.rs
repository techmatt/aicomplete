//! `aic-sim agree` (245): how often an agent picks what a recorded agent picked. Each game of a `games.jsonl`
//! played by the `--who` kind (default `oracle`; a whole spec, that agent alone; won games only) is replayed from its seed and actions; at every
//! decision with more than one legal action each `--agent` (or each line of `--agents FILE`) is asked to choose on
//! the recorded table, and its pick is tallied against the recorded one by the recorded action's kind. The agents
//! follow the recorded game (teacher forcing), so every agent sees the same tables. `later`: the pick is one of the
//! recorded actions still to come this round (the same play in another order).

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use aic_core::Action;
use serde::Deserialize;

use crate::agents::AgentSpec;
use crate::cli::{fail, Options};
use crate::experiment::{cells, StreamHeader};

#[derive(Deserialize)]
struct Line {
    cell: usize,
    #[serde(with = "crate::experiment::seed_string")]
    seed: u64,
    won: bool,
    actions: Vec<Action>,
}

/// The tally's key for an action: its kind.
fn kind(a: &Action) -> &'static str {
    match a {
        Action::Pass { .. } => "pass",
        Action::Choose { .. } => "choose",
        Action::Tier { .. } => "tier",
        Action::Draft { .. } => "draft",
        Action::Keep { .. } => "keep",
        Action::Play { .. } => "play",
        Action::Use { .. } => "use",
        Action::Power { .. } => "power",
        Action::Pay { .. } | Action::Withdraw { .. } | Action::CounterWith { .. } => "threat",
        Action::Standard { .. } => "standard",
        Action::Sell { .. } => "sell",
        Action::Convert { .. } => "convert",
    }
}

/// Agree, agree later this round, decisions: by group (`all`, `dN`) and kind.
type Tally = BTreeMap<String, BTreeMap<String, [u64; 3]>>;

pub fn run(o: &Options) -> i32 {
    let mut specs: Vec<String> = o.agents.clone();
    if let Some(f) = &o.agents_file {
        let text = std::fs::read_to_string(f).unwrap_or_else(|e| fail(&format!("{f}: {e}")));
        specs.extend(text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).map(String::from));
    }
    if specs.is_empty() {
        fail("agree needs --agent SPEC (repeatable) or --agents FILE");
    }
    let agents: Vec<AgentSpec> =
        specs.iter().map(|s| AgentSpec::parse(s).unwrap_or_else(|e| fail(&e))).collect();
    let who = o.who.clone().unwrap_or_else(|| "oracle".to_string());
    // 249: a whole spec (`oracle:worlds=3,...`) picks that agent's cells alone, beside another of its kind.
    let label = who.contains(':').then(|| AgentSpec::parse(&who).unwrap_or_else(|e| fail(&e)).label());
    let picked = |a: &AgentSpec| label.as_ref().map_or(a.kind == who, |l| a.label() == *l);
    let mut jobs = Vec::new();
    let mut cellsets = Vec::new();
    for (pi, path) in o.paths.iter().enumerate() {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| fail(&format!("{path}: {e}")));
        let mut lines = text.lines();
        let head: StreamHeader = lines
            .next()
            .and_then(|l| serde_json::from_str(l).ok())
            .unwrap_or_else(|| fail(&format!("{path}: no games.jsonl header")));
        if head.rules_revision != aic_core::RULES_REVISION {
            fail(&format!("{path}: rules revision r{}, the binary's r{}", head.rules_revision, aic_core::RULES_REVISION));
        }
        let cs = cells(&head.spec).unwrap_or_else(|e| fail(&format!("{path}: {e}")));
        for l in lines.filter(|l| !l.trim().is_empty()) {
            let g: Line = serde_json::from_str(l).unwrap_or_else(|e| fail(&format!("{path}: {e}")));
            if g.cell < cs.len() && picked(&cs[g.cell].agent) && g.won {
                jobs.push((pi, g));
            }
        }
        cellsets.push(cs);
    }
    if o.given.contains("--games") {
        jobs.truncate(o.games as usize);
    }
    let total: Mutex<Vec<Tally>> = Mutex::new(vec![Tally::new(); agents.len()]);
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..o.threads.max(1) {
            s.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::Relaxed);
                let Some((pi, g)) = jobs.get(k) else { break };
                let cell = &cellsets[*pi][g.cell];
                let tallies = one(&cell.rules, cell.difficulty, g, &agents);
                let mut t = total.lock().expect("tally lock");
                for (dst, src) in t.iter_mut().zip(tallies) {
                    for (group, kinds) in src {
                        for (kd, v) in kinds {
                            let e = dst.entry(group.clone()).or_default().entry(kd).or_insert([0; 3]);
                            for i in 0..3 {
                                e[i] += v[i];
                            }
                        }
                    }
                }
            });
        }
    });
    let total = total.into_inner().expect("tally lock");
    for (spec, t) in specs.iter().zip(total) {
        let line = serde_json::json!({ "agent": spec, "games": jobs.len(), "by": t });
        println!("{line}");
    }
    0
}

/// One recorded game, each agent asked at each decision.
fn one(rules: &aic_core::Rules, difficulty: i64, g: &Line, agents: &[AgentSpec]) -> Vec<Tally> {
    let group = format!("d{difficulty}");
    // The round each action is taken in, for `later`.
    let mut rounds = Vec::with_capacity(g.actions.len());
    let mut s = rules.new_game(g.seed);
    for a in &g.actions {
        rounds.push(s.round);
        if rules.apply_mut(&mut s, a, None).is_err() {
            fail(&format!("seed {}: a recorded action fails to replay", g.seed));
        }
    }
    let mut built: Vec<_> = agents.iter().map(|a| a.build(rules, g.seed)).collect();
    let mut out = vec![Tally::new(); agents.len()];
    let mut s = rules.new_game(g.seed);
    let mut legal = Vec::new();
    // `AIC_AGREE_SHOW=<kind>`: each disagreement of that kind to stderr.
    let show = std::env::var("AIC_AGREE_SHOW").ok();
    for (k, rec) in g.actions.iter().enumerate() {
        rules.legal_actions_into(&s, &mut legal);
        if legal.len() > 1 {
            let kd = kind(rec).to_string();
            // 249: `miss`, the decisions where the first agent (the base) picked otherwise: where a recorded agent
            // built on the base overrode it, so a change to the base reads there apart from where it already agreed.
            let mut miss = false;
            for (ai, agent) in built.iter_mut().enumerate() {
                let i = agent.choose(rules, &s, &legal);
                let pick = &legal[i];
                let same = pick == rec;
                if ai == 0 {
                    miss = !same;
                }
                if !same && show.as_deref() == Some(kd.as_str()) {
                    eprintln!("seed {} d{difficulty} action {k} r{}: recorded {rec:?}, picked {pick:?}", g.seed, rounds[k]);
                }
                let later = same
                    || g.actions[k + 1..]
                        .iter()
                        .zip(&rounds[k + 1..])
                        .take_while(|(_, r)| **r == rounds[k])
                        .any(|(a, _)| a == pick);
                // 246: a recorded play also tallies under `play:<card>`, per card.
                let card_key = match rec {
                    Action::Play { card, .. } => Some(format!("play:{card}")),
                    _ => None,
                };
                for grp in ["all", group.as_str(), if miss { "miss" } else { "" }].into_iter().filter(|g| !g.is_empty()) {
                    for key in [kd.as_str(), "any"].into_iter().chain(card_key.as_deref()) {
                        let e = out[ai].entry(grp.to_string()).or_default().entry(key.to_string()).or_insert([0; 3]);
                        e[0] += same as u64;
                        e[1] += later as u64;
                        e[2] += 1;
                    }
                }
            }
        }
        rules.apply_mut(&mut s, rec, None).expect("replayed above");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An agent asked on its own game's tables, in order, picks what it played every time (its RNG and plan state
    /// follow the same calls), and another agent doesn't: the replay hands every agent the recorded table.
    #[test]
    fn an_agent_agrees_with_its_own_game() {
        let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../configs/baseline.json")).unwrap();
        let mut c = aic_core::Config::from_json(&text).unwrap();
        c.apply_override("adversary.difficulty=1").unwrap();
        let rules = aic_core::Rules::new(&c).unwrap();
        let (seed, spec) = (5, AgentSpec::parse("greedy").unwrap());
        let mut agent = spec.build(&rules, seed);
        let mut s = rules.new_game(seed);
        let (mut legal, mut actions) = (Vec::new(), Vec::new());
        while actions.len() < 60 {
            rules.legal_actions_into(&s, &mut legal);
            let a = legal[agent.choose(&rules, &s, &legal)].clone();
            rules.apply_mut(&mut s, &a, None).unwrap();
            actions.push(a);
        }
        let line = Line { cell: 0, seed, won: true, actions };
        let t = one(&rules, 1, &line, &[spec, AgentSpec::parse("random").unwrap()]);
        let [same, later, n] = t[0]["all"]["any"];
        assert!(n > 20, "{n} decisions");
        assert_eq!((same, later), (n, n));
        assert!(t[1]["all"]["any"][0] < n, "random picks what greedy did at every decision");
        assert_eq!(t[0]["d1"]["any"], t[0]["all"]["any"]);
    }
}

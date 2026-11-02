//! Plays whole games with an [`Agent`] and records what a simulation sweep
//! needs: the action list (for replay), decision counts, the outcome and
//! the config/content hashes. Agents themselves live outside the core.

use crate::action::Action;
use crate::rules::Rules;
use crate::state::{GameState, Outcome, Stats};

/// The team agent: one policy for all seats, seeing the open state.
pub trait Agent {
    /// Picks an index into `legal` (never empty).
    fn choose(&mut self, rules: &Rules, state: &GameState, legal: &[Action]) -> usize;
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayedGame {
    pub seed: u64,
    /// Hash of the resolved config (see `Config::hash`).
    pub config_hash: String,
    /// Hash of the content played: the embedded files, overlay applied.
    pub content_hash: String,
    pub engine_version: String,
    /// `crate::RULES_REVISION` of the engine that played the game.
    pub rules_revision: u32,
    pub actions: Vec<Action>,
    /// `None` if the action cap stopped the game first.
    pub outcome: Option<Outcome>,
    /// Last round reached.
    pub rounds: u32,
    /// One per applied action.
    pub decisions: u32,
    /// Sum over decisions of the number of legal actions offered (all
    /// seats' plays and passes).
    pub options_total: u64,
    /// Decisions with exactly one legal action.
    pub forced: u32,
    pub max_options: u32,
    pub stats: Stats,
    /// `GameState::fingerprint` of the final state.
    pub final_fingerprint: String,
}

/// Plays one game to the end or to `max_actions`.
pub fn play(
    rules: &Rules,
    seed: u64,
    agent: &mut dyn Agent,
    max_actions: usize,
) -> (GameState, PlayedGame) {
    let mut g = rules.new_game(seed);
    let mut legal = Vec::new();
    let mut actions = Vec::new();
    let (mut options_total, mut forced, mut max_options) = (0u64, 0u32, 0u32);
    while actions.len() < max_actions {
        rules.legal_actions_into(&g, &mut legal);
        if legal.is_empty() {
            break;
        }
        let n = legal.len() as u32;
        options_total += n as u64;
        forced += (n == 1) as u32;
        max_options = max_options.max(n);
        let i = agent.choose(rules, &g, &legal);
        let a = legal[i].clone();
        rules
            .apply_mut(&mut g, &a, None)
            .expect("a legal action applies");
        actions.push(a);
    }
    let record = PlayedGame {
        seed,
        config_hash: rules.config_hash.clone(),
        content_hash: rules.content_hash.to_string(),
        engine_version: crate::VERSION.to_string(),
        rules_revision: crate::RULES_REVISION,
        decisions: actions.len() as u32,
        actions,
        outcome: g.outcome.clone(),
        rounds: g.round,
        options_total,
        forced,
        max_options,
        stats: g.stats.clone(),
        final_fingerprint: g.fingerprint(),
    };
    (g, record)
}

/// Rebuilds a game from seed + actions.
pub fn replay(rules: &Rules, seed: u64, actions: &[Action]) -> Result<GameState, String> {
    let mut g = rules.new_game(seed);
    for (i, a) in actions.iter().enumerate() {
        rules
            .apply_mut(&mut g, a, None)
            .map_err(|e| format!("action {i} ({a:?}): {e}"))?;
    }
    Ok(g)
}

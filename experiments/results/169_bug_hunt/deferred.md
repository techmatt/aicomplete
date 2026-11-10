# 169 bug hunt: deferred (D72 on) and won't fix (W27 on)

Each: what a user meets, the options, and the pick. `(Matt's call)` marks taste, scope or a rules term.

## Deferred

- **D72 agents: two move-heuristic misreads** (`crates/aic-sim/src/agents/moves.rs`; the fix is
  [agent_moves_fix.diff](agent_moves_fix.diff)). (a) `at_risk` / `pressure` count a site's agents next door across a
  country border and drop a firewalled site, where the Corrupt pull takes only its own country's neighbors and a
  firewalled site still corrupts on what it holds: Patrol, Checkpoint Network and its Action push agents that press on
  nothing. (b) The liberate gather counts an ambassador standing on a neighboring adversary site as free and takes it
  off that site's own bill. Amendment 7657 lets such a fix land only if a 32-seed paired on/off smoke moves win % by
  at most ±2 at every level of d1-d3; it moved more (seeds 41500-41531, the three pairs, 96 games a cell, on minus
  off, paired SE in brackets): strategist d1 -2.1 (1.5), d2 +0.0, d3 +0.0; greedy d1 -2.1 (1.5), **d2 -3.1 (1.8)**,
  d3 +1.0 (2.3). **Pick:** land the diff with the sim players item, on fresh seeds, after 168's bands are read.
- **D73 cards: Network Cleanup's tier III is tier II** (`content/alignment.json`): III removes 2 markers a hex where
  II removes 1, but a hex never holds more than 1 marker (`lose_node` adds one only to an untainted hex), so the
  widening a reader is promised never happens. Options: (a) III widens the place, not the count (`near_player`, as
  Rule of Law III); (b) III also pays more a marker; (c) leave it. **Pick (a)** `(Matt's call)`.
- **D74 agents: the strategist keeps tier-2 cards it never plays** (16-seed engine smoke `use.json`, seeds
  41432-41447, r72): Microgrid Lab Compact kept in 81% of the strategist's games, Open-Weights Release 73%, Orbital
  Observatory Array 50%, Exascale Training Cluster 48%, Autonomous Defense Grid 38%, Collateralized Grid, Deep State
  Ledger: 38 cards in all kept, never played (the engine: 9). Most are the opening's tier-2 keeps (drawn 0), and
  several were repriced at 166's tier-2 ceiling. D57 (Exascale) is one of them. **Pick:** the sim players item: one
  pricing for keep and play (D57's own pick), then re-read `play_read.py`.
- **D75 viewer: stack counts on hex numbers in agents' games.** W26 (overlaps at 15+ troops, random play only) no
  longer holds: the d3 strategist's games stack 18-29 troops on a hex, and at 1366×768 a two-digit count sits on the
  next hex's number (the sweep: 64 `board-text-overlap` in watched games, `c2-d3-strategist`, `c4-d3-engine`).
  Options: (a) a count of 10+ drawn smaller, or on a pill inside its hex; (b) leave it (a hover shows the hex).
  **Pick (a).**
- **D76 rules: a strike chooses for the player.** Kettle Operation and Lure Network's Action take troops first, then
  agents in hex order, with no stop; a plain Gather lets the player pick each batch and stop. 169 wrote the rule as
  the code plays it (ambassadors.md, ops.md). Options: (a) keep, now printed; (b) a strike asks batches like any
  Gather. **Pick (a)** `(Matt's call)`.

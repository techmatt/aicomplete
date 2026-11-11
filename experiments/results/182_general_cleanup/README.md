# 182: general cleanup (no rules change)

Built on main 0116966 (r77, 180). No rule, number or card changed; `RULES_REVISION` stays 77 and the golden
fingerprint did not move. The audit (seven read-only passes over the repo, each checking the earlier items' notes
against the code) is [findings.md](findings.md): 103 findings, marked clear or design, with files and lines.

**Same games.** `experiments/same-noplanner.json` (144 games), the base binary (0116966) against this branch:
**144 of 144 the same** (`bench/same_results.py`), rerun on the final build. The planner rows of `same.json`
(`planner`, `planner:strategy=1`, the three pairs, d2, seeds 1000-1007: 48 games), base against the branch: 48 of
48 the same.

## What changed

- **Code moves:** `modules/adversary.rs` (1,759 lines) is `modules/adversary/` (`mod`, `projects`, `rally`,
  `corrupt`, `pieces`, `bar`; glob re-exports keep every `adversary::` path). `agents/policy.rs` (1,631 lines) keeps
  the dials and the Policy agent; the keep plan is `agents/keep_plan.rs`, the card classifiers `agents/cards.rs`.
- **One home:** the converge prompt's words (`adversary::converge_prompt`, `read_converge_prompt`; the agent parsed
  them with a hard-coded offset); the draft direction (`cards::pass_to`, engine and agents); the energy rule's idle
  line reads the eval's `credit_cap` (no named agent sets it); `is_liberate` over `Ability::branches`; the endgame
  probe's liberate cost from `board::liberate_cost`; the playout loop of win probability and the endgame probe
  (`experiment::play_out`); the docs blocks as one table of name and body; one test `Cx` constructor.
- **Names:** adversary and board phase indices named (with a test each); `muster_bonus`; `land_troops` takes one
  `muster` flag (its two flags were always equal); Deploy/Movement comments say Muster/Dispatch; the dead `"board"`
  label arm; `threats.deck.3` (no tier 3 since 146) out of determinize's piles; three test modules named by topic.
- **Docs generated from dials:** the difficulty table's level-4 row prints a clause only for a dial that does
  something (`extra_corrupted` 0 printed "optionally more corrupted cities"), its level-1 row names the sites
  `corrupt_cut` lowers (it listed land, cut 0); harms.md reads `troop_node` and `ambassador_discord` from the rules.
- **Checks:** a test that every shipped config's round lists every module phase (178's hazard), and that the labs'
  `ACTIONS` is the baseline's; wasm `record_text` checks its state.
- **Scripts:** `drafts/adversary/check.py` was red on main (178 flipped "each board" to "each country" in Rust, not
  here); fixed, and `regen.py` runs it. `screens.py assemble` has a command line; `screens.load`, `dealt` and
  `play_read.py` read a farmed run (no results.json) from its header; `run_read.sh` takes an absolute EXE and OUT;
  `screen.py` drops columns for counters no engine writes (preset 028); `effect_stats.py` labels a prohibition two tiers
  share ("2-3"; it gave all to the last tier); the 026-102 counter rename map is gone. Farm specs of 162-177 moved to
  `experiments/archive/farm/` (the bundle no longer builds them in; `s177-s3` and `s168-d4b` no longer parsed).
- **Docs:** seed ledger next free 42997 (178 spent 42853-42996); the farmed read as the current recipe with one
  assembly recipe; r77 ladder in principles.md and agents.md; README test lanes; history rows 165 and 178.
- **Web:** the unused at-most requirement plate, `cdp.mjs` `shotsPage`, `.escal` CSS.

`npm run cards:check` fails one check: ten deck cards (mass_defection, embassy_of_exiles, double_agents,
kettle_operation, lure_network, firewall_array, air_gap, rally_the_allies, surgical_strike, open_source_uprising)
have no scene image served. That reads the shared `art-originals/`, which this item didn't touch; all its other
checks pass.

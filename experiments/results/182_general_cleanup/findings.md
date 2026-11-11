
# core-modules

## F1 [clear/small] Poorly structured (167: holds)
files: crates/aic-core/src/modules/adversary.rs:1469-1480, crates/aic-core/src/modules/effects/team.rs:28-34, crates/aic-core/src/modules/adversary.rs:1157-1159, crates/aic-core/src/tests/march.rs:266

land_troops still takes 9 positional arguments behind #[allow(clippy::too_many_arguments)]. Two of them are redundant: every call site passes `lowest` and `deploy` with the same value. team.rs:30 has `let deploy = lowest;`, wall_pressure passes `true, true`, and the test passes `false, false`. Both callers also add `deploy_bonus(&w)` to `count` themselves before the call (team.rs:31, adversary.rs:1158).

ACTION: Merge `lowest` and `deploy` into one `muster: bool`, or better a `Landing { hexes, count, prompt, tally, destroy, muster }` struct. Move the `deploy_bonus` addition inside land_troops when `muster` is set, and drop the clippy allow. Behavior stays the same.

## F2 [clear/small] Poorly structured (175: holds)
files: crates/aic-core/src/modules/effects/ctx.rs:35,79; crates/aic-core/src/modules/effects/team.rs:28,58; crates/aic-core/src/modules/effects/new_ops.rs:355-366; crates/aic-core/src/modules/effects/mod.rs:284,347

Whether a landing counts as a Muster (lowest-numbered, surcharge, prohibition) is still decided by matching the stringly typed `Ctx::label` against "standing" | "project" | "board" in two places in team.rs. Nothing ever calls `.named("board")` (git grep finds only the matches), so the "board" arm is dead in team.rs:28, team.rs:58 and create_source (new_ops.rs:361). The "standing"=>"event" mapping is also written twice (new_ops.rs:360 and mod.rs:347). The labels in use are card, standing, twist, project, megaproject, discord, unity and the threat tier labels.

ACTION: Replace `label: &'static str` with a `Source` enum (Card, Event, Twist, Project, Megaproject, Discord, Unity, Threat(tier)) with `is_muster()` and `tally_name()` methods. That puts the "standing"->"event" mapping in one home and drops the dead "board" arm. Keep the tally strings byte-identical so results don't move. At minimum, delete the dead "board" arms now.

## F3 [design/small] Refactor or clean up (167: partly)
files: crates/aic-core/src/op.rs:1070-1076,1134-1142; docs/ops.md:117-127; crates/aic-core/src/modules/effects/mod.rs:294-315; crates/aic-core/src/face.rs:692-697; crates/aic-sim/src/agents/value.rs:661-667; crates/aic-sim/src/agents/strategy.rs:153-157; crates/aic-sim/src/endgame.rs:149,154; crates/aic-core/src/content.rs:1497

docs/ops.md still counts 0 prints for counter_all, pushback, remove_botnets and one_player. free_purchases now counts 3 (content/alignment.json:21-25), so the earlier note is wrong about FreePurchases. docs/ops.md lists about 30 more 0-count ops besides these (transfer, damage_country, destroy_all, peek, cut_botnet, lose_income, occupy_factory, standing_troops_bonus, slots, logistics, ...). Note that standing_troops_bonus and threat_compute are read from adversary_board.difficulty and rules, so 0 there may mean the counter misses those slots rather than that nothing uses them.

ACTION: Decide whether to retire CounterAll, Pushback, RemoveBotnets and OnePlayer. Retiring one removes its Op variant, its effects arm, its face.rs and slots() rows, its value.rs arm and values.json row, and its sim matches. It changes the generated docs/ops.md and could move content or serde hash constants. Keep FreePurchases. Also check whether the ops.md counter reads adversary_board difficulty and rules lines before trusting any other 0.

## F4 [clear/small] Poorly structured (175: holds)
files: crates/aic-core/src/modules/adversary.rs (1759 lines): 349-705 Module impl, 809-1005 compute/projects, 1027-1135 pools/Rally, 1282-1470 Corrupt, 1670-1759 PurgeLevel/loss

adversary.rs is still about 1760 lines and mixes setup and phases, the project and compute economy, Rally and pools, Corrupt and natural corruption, troop landing, removal helpers and the purge bar.

ACTION: Split into adversary/mod.rs (module def, phases, flip), adversary/projects.rs (gain_compute..reveal, project keys), adversary/rally.rs (fill_pools, rally, increase_income), adversary/corrupt.rs (corrupt_*, natural_corruption, converge_one) and adversary/bar.rs (PurgeLevel, loss). Pure code motion with re-exports.

## F5 [clear/trivial] Poorly structured 
files: crates/aic-core/src/modules/adversary.rs:54-62,392-500

Adversary::run dispatches on bare phase indices (4 => flip is listed first, then 0,1,2,3,5). Each one is tied by position to the PHASES array (build=0, corrupt=1, natural=2, reveal=3, event=4, compute=5). Inserting a phase, as 178 did with compute, silently shifts them.

ACTION: Add `const BUILD: usize = 0;` etc. next to PHASES (or match on PHASES[phase].id) and use the names in run.

## F6 [design/small] Needs a decision (168: holds)
files: crates/aic-core/src/modules/adversary.rs:45-51,192,262-263; crates/aic-core/src/modules/alignment.rs:498,507; crates/aic-core/src/modules/board/mod.rs:275; crates/aic-core/src/modules/threats/mod.rs:294

Hard-coded difficulty levels remain. `difficulty >= 3` gates stage2_removed and stage1_added (adversary.rs:262-263). `level(w, 4)` gates tier2_early (adversary.rs:192, threats/mod.rs:294), discord_removed (alignment.rs:498,507) and extra_corrupted (board/mod.rs:275). The other dials use `*_at` config thresholds (stage_swap_at, counter_bonus_at, corrupt_cut_at, board_difficulty_at, extra_threats_at), which is a CLAUDE.md breach ('tunable numbers live in configs').

ACTION: Add `stage_trim_at` (3) and `hard_at` (4) params, or one `_at` param per dial, defaulting to today's values in the configs, and read them in place of the literals. Behavior is unchanged at the defaults, but it adds config keys and picks a shape, so it is Matt's call.

## F7 [clear/trivial] Refactor or clean up (175: partly (the code remnants are gone; two stale comments remain))
files: crates/aic-core/src/modules/effects/new_ops.rs:332-334,689-691

Two doc comments still promise last-node protection that 175 removed. corrupt_city says Regime Change destroys nodes 'never a last one', and destroy_factory says 'with the nodes on its hex, never a last one'. The code calls board::lose_node or destroy_piece with no last-node check, and node.rs:183 and op.rs:997 say 'a last node too'. No Hot::LastSafeStage or last_node_safe remains anywhere (git grep is clean), so only these comments are left over.

ACTION: Change both comments to '(175: a last node too)'.

## F8 [clear/small] Refactor or clean up (178: holds)
files: crates/aic-core/src/modules/adversary.rs:308,338,1157,1483,1508; crates/aic-core/src/modules/effects/team.rs:29; crates/aic-core/src/content.rs:537; crates/aic-core/src/modules/ai.rs:100-101; crates/aic-core/src/modules/board/standard.rs:11,137,215,223; crates/aic-core/src/modules/board/view.rs:25; crates/aic-core/src/modules/board/mod.rs:501; crates/aic-core/src/content.rs:660-661,693

Code comments still say Deploy (the printed word is now Muster, op.rs:380 Step::Muster) and Movement (now Dispatch in docs/rules and ai.rs:487). The identifiers `deploy_bonus` and the `deploy` parameter of land_troops, and `ai::movement`, carry the old words too.

ACTION: Reword the comments to Muster and Dispatch. Rename `deploy_bonus` to `muster_bonus` (sim callers included) and `ai::movement` to `ai::dispatch`. This touches names only, no behavior.

## F9 [clear/trivial] Refactor or clean up (178: holds)
files: crates/aic-core/src/modules/cards/play.rs:361-365; crates/aic-sim/src/agents/eval.rs:2104-2107

The draft direction is still written twice as `let step = 1;`: once in cards::draft_pick and once in the agents' packet pass, which comments that it mirrors draft_pick. If one changes and the other doesn't, the agents value the wrong packet.

ACTION: Add `pub fn draft_next(s: usize, n: usize) -> usize { (s + 1) % n }` in cards and call it from both places.

## F10 [design/small] Needs a decision (178: wrong (reachable when the factory under the token is destroyed before execution))
files: crates/aic-core/src/modules/effects/new_ops.rs:157-166,225-231; crates/aic-core/src/modules/threats/near.rs:42-53

The SeizeFactory 'free zone gets a new factory' branch is not strictly unreachable. near.rs no longer deals a factory token onto a free zone, but the factory under a token can be destroyed between the deal and execution (destroy_factory, Scorched Earth), leaving a free industrial zone under the token, and then build_factory fires. The branch is now a rare edge case, not the common free-zone path that docs/ops.md:109 implies.

ACTION: Decide whether a token over a destroyed factory should build one (keep it, and reword the op doc to say 'if its factory is gone') or do nothing (delete the free_zones branch, which is a rules change and needs a RULES_REVISION bump if any seed hits it).

## F11 [design/small] Needs a decision (175: holds (now documented in a comment))
files: crates/aic-core/src/modules/plan.rs:390-406

next_corrupt_n still reads a CoastalCity Corrupt line as a City line (line 396, now commented). A city forecast therefore takes a coastal line's number even for inland cities, and a CoastalCity query never matches a City line.

ACTION: Either keep the approximation (it is documented) or have the agents' callers query CoastalCity separately. That changes agent forecasts, so it is a design call.

## F12 [clear/trivial] Refactor or clean up (176: fixed)
files: crates/aic-core/src/modules/adversary.rs:1185,1539; crates/aic-core/src/modules/board/destroy.rs:74

adversary::place now returns Res, corrupt_station returns Res<bool>, and board::destroy_piece returns Res<bool> (whether the piece went), so the earlier ()/bool concern has been addressed.

ACTION: None.

## F13 [clear/trivial] Refactor or clean up (166: holds)
files: crates/aic-core/src/modules/common.rs:230-239; crates/aic-core/src/view.rs:461-469; crates/aic-core/src/modules/adversary.rs:20-62 (ParamSpec text)

Piece words are still built in two near-copies that have drifted. common::piece_name maps STATION to 'research station', and token_words in view.rs does not ('station'). token_words names any seat-keyed token as "seat N's x", while piece_name only does so for node. Both carry the 166 'agent' special case.

ACTION: Make view::token_words call common::piece_name (extended with the generic seat arm), or move both into one words module. The view text for stations changes to 'research station': a viewer-only string, no play change.

## F14 [clear/trivial] Refactor or clean up 
files: crates/aic-core/src/content.rs:222-225; crates/aic-core/src/modules/adversary.rs:185-186,198

Doc comments hard-code 'difficulty 4' (content.rs:223 for extra_corrupted, adversary.rs:185 and 198 for tier2_early), so whoever moves those literals to config keys must also reword these comments.

ACTION: When the levels move to config keys, reword these to name the key (for example `adversary.hard_at`).

# core-rest

## F15 [clear/small] Refactor or clean up (176: holds)
files: crates/aic-core/src/tests/mod.rs:477-549; crates/aic-core/src/tests/node_powers.rs:35-41; callers ambassadors.rs:332, science.rs:13

The 176 note still holds. `with_cx` (47 callers) still gives no way to read `choices.asked`. Three test helpers each build their own `module::Cx { world, module: "test", purposes: &[], events: None, choices, fought: [0; 3] }`: `destroy_with` (mod.rs:492), `with_answers` (mod.rs:517) and `with_cx_picks` (mod.rs:540). node_powers.rs:38 builds a fourth one inline to read the asked prompt. `destroy_with` is a one-ask special case of `with_answers`: it chooses option 0 if `spend`, else the last option.

ACTION: Add one `test_cx(rules, &mut Choices) -> Cx` constructor and use it in all four places. Rewrite `destroy_with` as `with_answers(rules, g, |a| if spend { 0 } else { a.options.len() - 1 }, |g, cx| board::destroy_piece(g, cx, h, kind, cause))`, or inline that at its 2 call sites. Add `first_ask(rules, g, f) -> Option<Ask>` for node_powers.rs's `prompt_for`.

## F16 [clear/small] Needs a decision (178: holds)
files: README.md:133-134; crates/aic-core/src/rules.rs:155-196 (compile_round); crates/aic-core/src/tests/config.rs (no such test)

The 178 note holds: the safeguard is only the README sentence. `compile_round` checks that every round entry names a real phase, but not that every phase of every configured module appears in the round. No test checks that configs/baseline.json, shared_row.json or experiments/configs/no_liberate.json list every module phase. A new phase left out of `round` silently never runs, in the baseline as well as in labs.

ACTION: Add a fast-lane test in tests/config.rs: for each shipped config, every `<module>.<phase>` from `describe_modules`/`variant.phases` appears in some `round` entry, split on '+'. Doing the same inside `Rules::new` would refuse the lab configs, which drop phases on purpose, so keep it a test of the shipped configs only.

## F17 [clear/small] Poorly structured 
files: crates/aic-core/src/tests/mod.rs:132-156 (lab_with), 104-130 (lab_config); crates/aic-core/src/tests/ambassadors.rs:485 vs 83; factories.rs:356

Lab rounds are hand-written phase lists with no order check. ambassadors.rs:485 runs `["board.bases", "board.produce"]`, but the baseline order is produce, upkeep, bases (baseline.json:380-382), and ambassadors.rs:83 and factories.rs:356 use the baseline order. `lab_with` always appends ACTIONS last, so labbed post-action phases run before the actions, unlike the baseline. A test can pass on an order the game never plays. This is the same family as the 178 hazard.

ACTION: In `lab_config`, assert that the given non-ACTIONS entries keep their baseline-round relative order, naming the caller with `#[track_caller]`. Reorder ambassadors.rs:485 to `["board.produce", "board.bases"]` and rerun that test.

## F18 [clear/trivial] Refactor or clean up 
files: crates/aic-core/src/tests/mod.rs:52; configs/baseline.json:379; tests/setup.rs:20,95; tests/adversary.rs:464

`ACTIONS` is a hand copy of baseline's combined decision entry "cards.play+threats.counter+alignment.act+board.act+ai.power". If a module adds a decision part to that phase in baseline.json, every lab, `action_only` and setup test silently plays without it. Nothing compares the two.

ACTION: Derive it once from `baseline().round`, the entry containing '+' (for example a `fn actions() -> String` or a `LazyLock`). Alternatively, add a one-line test asserting `baseline().round.contains(ACTIONS)`.

## F19 [clear/trivial] Refactor or clean up (176: fixed)
files: crates/aic-core/src/modules/adversary.rs:64-66; crates/aic-sim/src/agents/determinize.rs:437-454, 41-49; crates/aic-core/src/content.rs:1090

The 176 determinize note is fixed. 178 moved the compute step into its own `adversary.compute` phase, and the test's whole-reveal-phase skip is gone. The pending-choice replay test (determinize.rs:437-454) now skips only a prompt whose resolution drew from a hidden pile, compared by pile lengths against the snapshot. Separately, `SHUFFLED` (determinize.rs:41-49) still lists "threats.deck.3", but `THREAT_TIERS = [1, 2]` (content.rs:1090; tier 3 was removed in 146), so that entry names a pile that never exists.

ACTION: Drop "threats.deck.3" from `SHUFFLED` and size the array to 6, or build the threat entries from `THREAT_TIERS`. No play change: determinize skips absent piles.

## F20 [clear/small] Slower than expected 
files: crates/aic-core/src/tests/determinism.rs:28-41, 46-48, 57-78; README.md:115-117

Two fast-lane tests loop over configs in one test body. `random_games_replay_exactly_in_every_variant` plays and replays one random game for each of the 13 configs in configs/variants.json, plus a JSON-path replay on every fourth. `every_country_count_plays_to_an_end_and_replays` plays and replays 16 games (4 sizes x 4 seeds). README says a test looping over variants should split into one test or one scoped thread each, because the fast lane's wall clock is the sum of each binary's slowest test. invariants.rs:349 puts one invariant-checked seed per variant at ~14 s. I did not time these two tests (read-only audit).

ACTION: Run each variant (and each country count) on its own scoped thread inside the test, as README prescribes, or split by country count. Time them before and after with `cargo test -p aic-core determinism -- --nocapture`.

## F21 [clear/trivial] Refactor or clean up 
files: crates/aic-core/src/tests/moves_166.rs:10-15; crates/aic-core/src/tests/tier2_158.rs:10-15

`by_a_city` is defined twice, word for word: a lonely-neighbor city, seat 0's node on the first neighbor, and the city plus three neighbors returned.

ACTION: Move one copy next to `city_with_lonely_neighbors` in tests/mod.rs and delete both local definitions.

## F22 [clear/trivial] Refactor or clean up 
files: crates/aic-core/src/state.rs:96-101, 132-137; tests/mod.rs:498, 525; tests/cards.rs:592; tests/moves_166.rs:24; tests/standard_actions.rs:417

`Pick` and `Opt` are the same struct with the same serde attributes (`option: String` and `target: Option<Hex>`, skipped when None). Tests convert one to the other by hand five times: `Pick { option: o.option, target: o.target }`.

ACTION: Add `impl From<Opt> for Pick` in state.rs and use `o.into()` at the five sites. Serialization is unchanged, so fingerprints are too.

## F23 [clear/trivial] Refactor or clean up 
files: crates/aic-core/src/tests/mod.rs:29, 35, 43; tests/moves_166.rs, tests/fear_167.rs, tests/tier2_158.rs

Three test modules are named after the work item that added them (`moves_166`, `fear_167`, `tier2_158`). Every other test module is named by topic (march, threats, war...), so a reader looking for Gather/Push/Liberate, prohibitions or convert/firewall tests can't find them by name. No doc or script references these filenames.

ACTION: Rename by topic, e.g. `moves_166` -> `gather_push`, `fear_167` -> `fear_cards`, `tier2_158` -> `endgame_ops`. Update the `mod` lines and keep the item numbers in each file's `//!` line.

## F24 [clear/small] Poorly structured 
files: crates/aic-core/src/module.rs:166-181; crates/aic-core/src/modules/adversary.rs:58-67, 393-470; modules/board/mod.rs:406-410; modules/cards/mod.rs:311-397

`Module::run`/`act`/`legal` take a bare phase index, and modules dispatch on numeric literals that must match their `PHASES` array order. The adversary's PHASES are declared build(0), corrupt(1), natural(2), reveal(3), event(4), compute(5), while the round plays event, build, corrupt, natural, reveal, compute. Its `run` matches `4 => flip`, `0 => build`, ... `5 => compute`. When 178 inserted `compute`, it could go in only at the end. Inserting anywhere else silently remaps every later arm.

ACTION: Give each module named `const` indices next to its `PHASES` (e.g. `const BUILD: usize = 0; ... const COMPUTE: usize = 5;`), match on those, and add a debug assertion or test that `PHASES[COMPUTE].name == "compute"`. Behavior is unchanged.

## F25 [design/small] Needs a decision 
files: crates/aic-core/src/modules/adversary.rs:237-239, 396-403; crates/aic-core/src/state.rs:173-176

Rules code carries a branch that exists only for lab rounds. When the round has no `adversary.event` phase, `round_pile` keys off `adversary.build` and `run` flips the adversary card in the build step ("lab rounds"). Every shipped config lists `adversary.event` (baseline.json:368, shared_row.json:368, no_liberate.json:360). So these game-rules paths run only in tests, and tests built on them exercise a flip timing the game never has.

ACTION: Either (a) keep the branch, a documented test convenience, or (b) have `lab_config` add `adversary.event` whenever an adversary phase is listed and delete both fallbacks. Option (b) changes what several lab tests run, so Matt or the item owner should choose.

## F26 [clear/trivial] Docs that would have saved time 
files: crates/aic-core/src/module.rs:166-169, 659-677

The `Module` trait doc says randomness comes only via `Cx::rng` or `Cx::deck_rng` and `Cx::threat_rng` for a deck reshuffle. It leaves out `Cx::adversary_rng` (module.rs:672-677; added in 146, used at adversary.rs:719 for the project-deck reshuffle in another module's step). The doc lists exceptions to the declared-purpose rule and is now incomplete.

ACTION: Add "or [`Cx::adversary_rng`] (the project deck)" to the trait doc at module.rs:169.

## F27 [clear/trivial] Docs that would have saved time 
files: crates/aic-core/src/tests/mod.rs:1-12 (no module doc); helpers at 54-188, 401-417, 477-559

No page or module doc says which core test fixture to use when. The options are `action_only` (the baseline action phase, seed 4), `lab`/`lab_with` (board emptied, lab params such as `liberate_at` 1), `lab_actions` (board as set up), `threat_lab` (counter_pct pinned), `with_cx` (cannot read an ask), `with_answers`, `run_ops`, `destroy_with`, `deal`/`deal_at`. Each one's doc comment is the only guide, and several pin non-baseline params (mod.rs:105-127). docs/ and README never mention them, and README (14.3 KB) has no room.

ACTION: Add a short `//!` header to tests/mod.rs: one line per fixture saying what it pins and when to use it, plus the 178 rule that a lab round must list each phase the test needs, in baseline order.

## F28 [clear/trivial] Refactor or clean up 
files: crates/aic-wasm/src/lib.rs:117-123; crates/aic-core/src/lib.rs:62-67

`record_text` builds `Rules::new(&state.config)` and never calls `check_state`. The other state-taking wasm exports (`view`, `legal_actions`, `apply`) go through `aic_core::rules_for`, which does. A state that doesn't fit its config therefore gives a record that fails only on replay, not a clear error at save time.

ACTION: Call `r.check_state(&state).map_err(err)?` after building the rules, or make `rules_for` public and use it. Only a malformed client state is affected; games are unchanged.

# sim-agents

## F29 [clear/small] Poorly structured (166: holds)
files: crates/aic-sim/src/agents/policy.rs:557-563; crates/aic-sim/src/agents/moves.rs:44-73, 79-87, 145-149, 251-257, 294; crates/aic-core/src/modules/adversary.rs:1389; crates/aic-core/src/modules/effects/moves.rs:152, 236

Still true. The fixed policies and move heuristics recognise a prompt by its text and read data back out of it. wall_hex matches board::NODE_PROMPT. moves::pick matches effects::GATHER_INTO/GATHER_PIECE/PUSH_FROM/PUSH_PIECE/STRIKE_PIECE, then named_hex reads a hex name back after the literals " (into " / " (from ". batch() parses option labels such as "2 agents from X" and "seat {s}'s ambassador". Worst case: moves::converge matches "Corrupt ", "moves onto which?" and " (at " as bare literals (line 83), with a hard-coded `+ 5` offset (line 87) that must match the format! in adversary.rs:1389. No shared const covers these, so rewording that prompt would turn converge_rule off with no error, which is the same way 118 silently disabled wall_turtle.

ACTION: Now: add a pub const for the converge prompt pieces in adversary.rs (as NODE_PROMPT does), use it in both the format! and moves::converge, and replace the `+ 5` with the const's len(). Later, as a design call: a typed ask tag on Pending (an AskKind variant or a field carrying site/hex/seat), which would also replace named_hex and the label parsing. Note that a typed tag changes the serialized state.

## F30 [clear/small] Poorly structured (166: holds)
files: crates/aic-sim/src/agents/moves.rs:20-40; crates/aic-sim/src/agents/policy.rs:697-700; crates/aic-sim/src/agents/planner.rs:151-166; crates/aic-sim/src/experiment_run.rs:136-138

Still true, and the printed number is skewed. FIRES is a process-wide static, divided by games.len() across every cell, so a run with move_rules off in one cell and on in another prints a pooled average. planner.rs:166 wraps every rollout policy in Policy, which calls moves::fired, so for planner agents the count includes rollout answers, not only moves actually played. The comment at moves.rs:20 promises a per-game tally `agents.move.<name>` that does not exist (git grep finds only the comment).

ACTION: Count fires through the game's own stats instead, e.g. a cx.tally-style counter or a per-agent counter returned with GameResult, keyed per cell. Skip counting when the Policy is a rollout policy (pass a flag from rollout_policy). Fix or remove the `agents.move.<name>` comment.

## F31 [design/small] Needs a decision (166: holds)
files: crates/aic-sim/src/agents/policy.rs:84-90, 186-195, 289-292; crates/aic-sim/src/agents/mod.rs:503-511

Still true, with a concrete consequence. forced_table! holds both dominance checks/fixed policies (default 0) and the standard agent behaviours move_rules, converge_rule and energy_rule (default 1). The three behaviours are marked active=false, so they never make an agent a Policy by themselves. A spec such as `greedy:plan_keep=0` with no other active row is not wrapped (mod.rs:507), so all three rules switch off with no warning. The table's doc says active=false rows are 'a plan's numbers', which these three are not.

ACTION: Choose one: (a) mark move_rules/converge_rule/energy_rule active=true; or (b) move them out of Forced into a separate agent-behaviour struct that always wraps the agent. Either way, document the result in the forced_table doc comment. This changes which agents get wrapped, so it can change games for plan_keep=0 specs.

## F32 [design/trivial] Needs a decision 
files: crates/aic-sim/src/agents/policy.rs:686-702, 759-766, 495-533

The 175/176/166 agent rules (converge_rule, energy_rule, move_rules) return their pick straight away, before the ruled_out filter at line 759. antimatter_rule (line 666) does check ruled_out. As a result a dominance check can be broken by a rule that is on by default. `turtle` and `free_only` forbid every standard action outside their list (lines 531-532), yet buy_energy returns Standard "buy_energy". `abandon` and `home_only` forbid Choose options in the given-up or foreign country (lines 508-511), yet moves::pick and moves::converge can return those Choose actions.

ACTION: Add `&& !self.ruled_out(&world, g, &legal[i])` to the three early returns, as antimatter_rule does. This changes what the fixed-policy agents play, so it is a design change.

## F33 [clear/small] Refactor or clean up (175: holds)
files: crates/aic-sim/src/agents/eval.rs:1183-1206; crates/aic-core/src/modules/board/ambassadors.rs:72-82; crates/aic-sim/src/agents/moves.rs:331-338; crates/aic-core/src/modules/plan.rs:157-162

Still true. Eval::toward has its own site classifier over HexTokens: a closure returning u8 codes 1/2/3, 0 when adv_cube>0. It re-implements board::site_kind (city > station > factory) plus common::corrupted. moves::at_risk holds a third copy of the rule that agents on a clean site of the same kind stay put, while the engine's own copy is in plan::converge.

ACTION: Add a site_kind helper on HexTokens that returns Option<board::SiteKind>, with the city check through common::is_city, and use it in toward instead of the u8 codes. Point the at_risk filter and toward at one shared 'agents that stay' predicate. Both should keep behaviour; confirm with same_results.

## F34 [design/small] Poorly structured (176: holds)
files: crates/aic-sim/src/agents/guide.rs:45-111; crates/aic-core/src/modules/board/destroy.rs:47-66

Still true, and the copy has drifted. guide.rs's absorbed/factory_absorbed/HexArmor::spend re-implement board::destroy_piece's protections as hardening, then the owner's ambassador. The engine's guards() also offers Guard::Base (a corp troop at an adjacent base, on Cause::Troop) and Guard::Aegis (the Protector's capstone). The forecast models neither, so shelter_saves over-credits a shelter for a seat with Aegis ready, or for a node next to a manned base.

ACTION: Make board::guards (or a read-only `protections(w, g, h, owner, cause) -> Vec<Guard>`) pub in aic-core and have guide.rs count from it. Base and Aegis then enter the forecast, which changes the eval's shelter credit, so this is a design change.

## F35 [design/small] Needs a decision 
files: crates/aic-sim/src/agents/guide.rs:32-43, 51-53, 174-180, 233-254, 265-267; crates/aic-sim/src/agents/eval.rs:170, 562, 1589

Armor::AnyAmbassador, the pre-147 forecast where any player's ambassador armors a node, is the off arm of w.shelter_armor (default 1). Only the archived 147 screens (experiments/archive/read-147-*.json, screen-147-*.json) set w.shelter_armor=0. It models a rule r75 removed (only the owner's ambassador guards), and keeps two extra match arms in shelter_saves.

ACTION: Retire the AnyAmbassador arm and w.shelter_armor, adding a RETIRED row in eval.rs with the reason, as 165 did for the zero-weight probes. This changes what an old spec does, so Matt's call.

## F36 [clear/trivial] Refactor or clean up (176: holds)
files: crates/aic-sim/src/agents/policy.rs:327-345; crates/aic-sim/src/agents/eval.rs:215-218, 581, 1150, 1174

Still true. buy_energy compares against a const IDLE_CREDITS = 12, a copy of Weights::default().credit_cap. It ignores a spec's w.credit_cap and ignores credit_cap<=0, which eval reads as no cap (f64::MAX). Policy can already reach the eval through self.inner.eval().

ACTION: Pass the cap into buy_energy from `self.eval().map(|e| e.w.credit_cap)`, treating <=0 as no cap and falling back to the default, and delete IDLE_CREDITS. No alias overrides credit_cap, so every named agent plays the same games.

## F37 [design/small] Needs a decision (177: partly)
files: crates/aic-sim/src/agents/eval.rs:454-463, 1238-1242, 1567-1601, 1777-1779; crates/aic-sim/src/agents/mod.rs:317-319

Partly holds. The share table now has one home, danger_share, used for both the danger term and the shelter step. Still true: danger_share(..=1) and rounds_left's `thin <= 1` read a 0-node seat the same as a 1-node seat. The only correction is gated on w.last_node > 0: the `-w.win` return when adversary::loss(g) is set mid-resolution (line 1240), plus the last-node charges and credits. Only the strategist alias sets last_node (120), so greedy, engine and planner score a position that is already lost as ordinary danger. One dial also controls five separate behaviours: the lost-game return, the Rally cancel bookkeeping, the exposure charge, the shelter credit, and the moves/converge push guard.

ACTION: Decide whether the adversary::loss early return should apply to every agent, as an engine fact rather than a heuristic. If yes, take it out of the w.last_node gate (it changes greedy/engine/planner play, so it needs a smoke). Optionally split w.last_node into the eval charge and the move-rule guard.

## F38 [design/small] Needs a decision (178: holds)
files: configs/agents/values.json:44-52; crates/aic-sim/src/agents/value.rs:797-815; scripts/cards/effect_ledger.py:341-355; experiments/results/178_rules_words/README.md:47-48

Still true. The trigger firing rates fires_troop 0.3, fires_compute 0.4, fires_ambassadors 0.8, fires_tokens 1.0 and fires_wall 0.5 have been guesses since 105. 178 left them alone for lack of a measured basis, and no script measures them. Every trigger card's table value (and the effect ledger, which mirrors value.rs) rests on them.

ACTION: Add a read that measures them: tally how often each Event fires per seat per round in records (an engine counter per Event kind), then set the rates from a band read in a rules item. Moving the rates changes agent valuations and card pricing.

## F39 [clear/trivial] Docs that would have saved time 
files: docs/agents.md:47-51; crates/aic-sim/src/agents/moves.rs:1-11, 32, 62-64, 263-274

178 added a fourth move heuristic, strike_piece (NAMES[3] = "strike"), which answers every strike batch prompt. docs/agents.md:50 still says "Anything else, and every strike, is the search's." The moves.rs module doc lists three heuristics and says a strike's choices are left to the search. report()'s doc reads "push N, cancel N, liberate N".

ACTION: Update docs/agents.md:50 and the moves.rs module doc to list the strike rule (a strike's hex is the search's, its batches the rule's: troops, then agents, largest first). Fix report()'s doc string to include strike.

## F40 [design/trivial] Needs a decision 
files: crates/aic-sim/src/agents/moves.rs:62-64, 263-274; crates/aic-sim/src/agents/policy.rs:697-702

strike_piece reproduces the engine's fixed pre-r77 order so a strike costs one search answer. It sits inside moves::pick, so move_rules=0 turns it off along with the push/cancel/liberate heuristics. The move-rules on/off read (178: strategist 66.7 vs 69.2) therefore also changed how every strike's batches were answered, mixing two levers in one row, which CLAUDE.md asks to screen as a factorial.

ACTION: Give the strike rule its own switch (a strike_rule row, default on) or make it unconditional, so move_rules covers only the 166 heuristics.

## F41 [design/small] Needs a decision 
files: crates/aic-sim/src/agents/policy.rs:103, 221, 244, 258; docs/agents.md:139

Four Forced rows are off by default, set by no alias and used only in archived screens: plan_hand (082 probe guard, 0), antimatter_rule (082, 'never fired in 288 games'), plan_chain (107, off: -2.8), and plan_team (124, off: no gain read). 165 retired the matching zero-weight Weights probes into eval::RETIRED, but these keep-plan and policy probes stayed, each with its code path in plan()/choose.

ACTION: Retire them as 165 did, with a refusal and reason list for Forced like eval::RETIRED, and delete their code paths. Row order feeds spec labels, so check AgentSpec::label output.

## F42 [clear/large] Poorly structured 
files: crates/aic-sim/src/agents/eval.rs:1228-1703, 2034-2193; crates/aic-sim/src/agents/policy.rs:625-767

Eval::value is one function of about 475 lines that mixes the outcome check, Rally forecast, per-hex corruption pressure, node danger, shelter, Discord, alignment and threats, followed by horizon_value at 160 lines. eval.rs is 2,610 lines. Policy::choose is about 140 lines of sequential early-return rules. These sizes make it hard to find which term a dial reaches: w.last_node alone is read in five places across value().

ACTION: Split value() into named term functions with the same arithmetic in the same order (board pressure, node danger and shelter, alignment, threats), checked with bench/same_results.py and speed.py check. In choose, put the fixed-policy rules in a table or helper.

## F43 [clear/small] Poorly structured 
files: crates/aic-sim/src/agents/policy.rs:869-1631; crates/aic-sim/src/agents/planner.rs

policy.rs (1,631 lines) holds three things: the forced_table/Forced dials, the Policy agent with its fixed policies, and the whole keep planner (Budget, PlanCtx, PlanTable, plan_keeps, plan, value, buried_value, and others, about 760 lines). It also holds the card classifiers is_engine/is_liberate/is_capstone used by strategy.rs and `aic-sim cards`. The keep planner is easy to confuse with planner.rs, the search Planner agent.

ACTION: Move the keep plan (lines 869-1631 apart from the classifiers) into agents/keep_plan.rs, and the card classifiers plus cards_csv into a small agents/cards.rs. Code moves only, no behaviour change.

## F44 [clear/trivial] Refactor or clean up 
files: crates/aic-sim/src/agents/policy.rs:1147-1153, 349-354; crates/aic-core/src/content.rs:431-437

is_liberate walks a card's ability by hand as `a.effects` plus `a.or`'s effects, which is only two levels. counters_threats, a few lines up, uses Ability::branches(), which follows the whole `or` chain. No card in content/ nests an `or` three deep today (checked), so the two agree for now, but a third branch would silently fall out of is_liberate (the 080 always_liberate policy and the strategy LIBERATE category).

ACTION: Rewrite is_liberate's ability check as `d.ability.iter().flat_map(|a| a.branches()).any(|b| b.effects.iter().any(lib))`. Behaviour is identical on current content.

## F45 [clear/trivial] Docs that would have saved time 
files: docs/agents.md (14,493 bytes)

docs/agents.md is at 14.5 KB against the 15 KB budget, so less than the roughly 1 KB of free space CLAUDE.md asks a page to keep. The next agent item that adds a line, such as the strike-rule fix above, must trim the page first.

ACTION: Trim repetition and history lines (for example the per-item numbers in the Strategy-guide list that agents.md already points to in experiments/results) to about 14 KB.

# sim-rest

## F46 [clear/small] Poorly structured (177: partly)
files: crates/aic-sim/src/docs/blocks.rs:364-392, docs/rules/adversary.md:180, configs/baseline.json:93,108

The 177 note partly holds. Only the discord_removed clause is now conditional (blocks.rs:384). The level-4 row still prints 'tier-2 threats sooner' whatever tier2_early is, and 'optionally more corrupted cities per country at setup' whatever extra_corrupted is. The baseline has extra_corrupted 0, so docs/rules/adversary.md:180 currently describes a setup change that doesn't happen. The rows are also a fixed list of 5 (levels 0-4), while round_cap_block (blocks.rs:343-345) reads the levels from the adversary.difficulty tunable's min..max.

ACTION: Build each level-4 clause from its own dial, as discord_removed already is: print the tier-2 clause only when int(r, "adversary.tier2_early") > 0, and print the corrupted-cities clause only when extra_corrupted > 0, naming the count. Size the rows from tunable(r, "adversary.difficulty").max + 1. Then run aic-sim docs and add a tests.rs case (as in a_config_change_changes_its_block) with extra_corrupted=1.

## F47 [clear/trivial] Refactor or clean up (176: holds)
files: crates/aic-sim/src/endgame.rs:115,132,140-141,61; crates/aic-core/src/modules/board/ambassadors.rs:110-116,233

The 176 note holds. pieces() recomputes the Liberate bill itself (lib_at = w.int("board.liberate_at"); liberate_cost += lib_at + amb) when board::liberate_cost(w, g, h) already returns exactly that for a corrupted city (liberate_ambassadors(City) = liberate_at(w) + agents on the hex). remove_actions counts agents on every hex that isn't a corrupted city, although the rule's Remove takes agents 'in reach' (standard.rs:301), so it re-states the rule rather than reading it. The Pieces doc comment at line 61 also hard-codes 'Clear sways 2 Unity for 2 credits' (board.clear_unity and clear_cost).

ACTION: Replace lib_at + amb with board::liberate_cost(&w, g, h).unwrap_or(0). The two give the same numbers, because liberate_at's range is 1..10, so .max(1) never changes anything. Reword the line-61 comment to name the tunables instead of their values. Whether remove_actions should count only agents in reach is a probe-definition question: leave it as it is and say in the field's doc that it ignores reach.

## F48 [design/small] Needs a decision (175: holds)
files: scripts/content/load_adversary.py:115-120, crates/aic-sim/src/docs/blocks.rs:157-175, docs/rules/adversary.md:95

The 175 note holds. Two generators print each project's cost at 2, 3 and 4 players: load_adversary.py writes docs/adversary-cards.md's Projects table ('Compute (2 / 3 / 4 players)'), and projects_block writes the projects block in docs/rules/adversary.md. Both multiply p.cost by 2, 3 and 4. A change to how cost prints has to be made in both, in two languages.

ACTION: Pick one home. Either drop the cost columns from projects_block and link to adversary-cards.md#projects, which is how threats_block and alignment_block already defer to the card pages, or have aic-sim docs write adversary-cards.md's Projects section. Either way, run aic-sim docs.

## F49 [design/small] Needs a decision (171: holds)
files: crates/aic-sim/src/farm.rs:80-92,192-195; crates/aic-sim/src/cli.rs:84-92,115-121

The 171 note holds. A farm shard takes only a contiguous range (--seed F --games N, and Farm::play(first, count)). There is no seed list, so neither a sparse set of seeds nor a replay of one failed seed in a shard can be expressed, except as count-1 shards.

ACTION: Decide whether conductor needs seed lists. If it does, add --seeds A,B,C as an alternative to --seed/--games in farm::cli and Farm::play. Header assembly (StreamHeader takes first_seed and games) would then need a rule for non-contiguous sets.

## F50 [clear/trivial] Docs that would have saved time (166/177: holds)
files: crates/aic-sim/src/cli.rs:115-121,135-146; crates/aic-sim/src/farm.rs:69-73; docs/simulator.md:90-91

The 166/177 note holds: aic-sim farm takes no --threads. This is by design: a shard is one thread, and from_spec refuses an agent with threads > 1. But `aic-sim farm read --threads 4` dies with only 'farm does not take --threads', because hint() has no farm arm. The explanation is only in simulator.md ('A shard is one thread').

ACTION: Add ("farm", "--threads") => " (a shard is one thread: run more shards in parallel)" to cli::hint. No behaviour change.

## F51 [clear/small] Refactor or clean up 
files: crates/aic-sim/src/experiment.rs:641-652; crates/aic-sim/src/endgame.rs:228-243

The same playout loop appears twice: build an agent seeded from g.seed + salt + k*0x9e37_79b9, then loop while n < max_actions over legal_actions_into, choose and apply_mut, stopping on empty or Err. It is in win_probability and again in endgame::probe; only the salt differs (0x5eed_0000+round and 0x0e9d_9a3e).

ACTION: Extract fn play_out(rules, s: &mut GameState, agent: &AgentSpec, seed: u64, max_actions) in experiment.rs and call it from both places, passing the same seed expressions. The RNG streams are unchanged, so results are identical.

## F52 [clear/trivial] Refactor or clean up 
files: crates/aic-sim/src/experiment.rs:234-236; crates/aic-sim/src/farm.rs:173-176

experiment::cells passes an inline closure `|path| std::fs::read_to_string(path).map_err(|e| e.to_string())` that is identical to farm::disk.

ACTION: Make cells call cells_with(spec, &crate::farm::disk), or move disk into experiment.rs and have farm use it.

## F53 [clear/trivial] Docs that would have saved time 
files: crates/aic-sim/src/cli.rs:30-32; crates/aic-core/src/config.rs:189-208; docs/simulator.md:28

USAGE's --set line lists countries=N, <module>.<param>=V, <module>.variant=NAME and content overlays. It leaves out two forms apply_override accepts and simulator.md documents: <module>=off, which removes a module, and an empty value, which unsets a parameter.

ACTION: Add '<module>=off; <module>.<param>= (empty) unsets' to the --set text in cli::USAGE.

## F54 [clear/trivial] Docs that would have saved time 
files: docs/architecture.md:178-183; crates/aic-sim/src/docs/run.rs:519-570

Generated docs says `aic-sim docs` fails only on a broken harm distribution, and that `--check` 'also' refuses spelling, history headings, card notes, a missing or doubled block and pages over 15 KB. In the code, plain `aic-sim docs` already exits 1 on all of those (run.rs:520-526, 534-537, 568). Only staleness and unknown backticked names are --check-only (541-557). The paragraph also never mentions two checks the run makes: the rules README's 'at rules revision N' line (revision_line_failures) and card quotes a face no longer prints (card_quote_failures). docs/mod.rs:9-17 has the split right.

ACTION: Rewrite the paragraph: the run (with or without --check) fails on the harm checks, a missing or doubled block, a page over budget, a stale revision line, a stale card quote, spelling, history headings and card notes; --check adds staleness and unknown names.

## F55 [clear/small] Refactor or clean up 
files: crates/aic-sim/src/docs/run.rs:232-282

Every block name is written twice: once in BLOCKS (21 entries) and once in block()'s match arms. A new block must be added to both, and only a test (every_listed_block_has_a_body) catches a mismatch.

ACTION: Make BLOCKS a single table, [(&str, fn(&Rules) -> String); 21], with the content-reading blocks wrapped as |_| countries_block(content()) and so on. Have block() look names up in it. Output is unchanged.

## F56 [clear/small] Poorly structured 
files: crates/aic-sim/src/docs/harms.rs:282-291; configs/baseline.json:16,26

harms_md() hard-codes tunable values in its prose: 'take facilities only at `board.troop_node` 4' and '`board.ambassador_discord` is 0'. This is the same static-prose-over-a-dial pattern as the 177 note. Both match the baseline today, but a config change would leave docs/harms.md wrong while --check still passes.

ACTION: Pass &Rules into harms_md (run.rs:506 already has r) and print both values with tick() from blocks.rs, or with the w.int value. Output is identical at the baseline.

## F57 [clear/small] Poorly structured 
files: crates/aic-sim/src/experiment.rs:659-811 (counters block 746-792); crates/aic-sim/src/report/findings.rs:15-219; crates/aic-sim/src/report/records.rs:51-225

play_one is about 150 lines. About 47 of them (746-792) are inline slot, building-Action and energy-card tallies that write into a local `slots` map, mixed in with the game loop. findings() is about 205 lines of independent finding paragraphs, and save_records about 175.

ACTION: Move play_one's 092/125/128 tallies into fn tally_slots(rules, g, a, legal, slots, fill_round), alongside tally_action and tally_keeps. Split findings() into one function per finding (ladder, skill gap, game length, cards, winprob), as comparison_findings already is. These are pure moves.

## F58 [design/small] Needs a decision 
files: crates/aic-sim/src/lib.rs:50,53-61; crates/aic-sim/src/cli.rs:19-25,112,122; experiments/results/124_agents_engines/read124.py:39

Two subcommands have almost no callers. `aic-sim cards` is invoked only by an archived write-up script (read124.py), and no script runs `aic-sim strategies`; it appears only as recipes in docs/simulator.md:21 and docs/agents.md:166. Both add USAGE lines, COMMANDS rows and lib.rs arms.

ACTION: Matt's call: keep them as hand tools, which is how simulator.md presents them, or drop them. If dropped, keep policy::cards_csv and print_choices only if a test uses them, and update simulator.md and agents.md.

## F59 [clear/trivial] Refactor or clean up 
files: crates/aic-sim/src/docs/tests.rs:214-229; crates/aic-sim/tests/docs.rs:22-55

The two checks that the card-art files name cards in content live in different test homes. samples.json is checked in the unit tests (the_art_samples_name_cards_in_content) and scenes.json in the integration tests (every_art_scene_names_a_card_in_content), and each collects known ids differently: content().cards against walking six content JSON files.

ACTION: Move the samples test next to the scenes test in tests/docs.rs, or the reverse, so the art checks have one home. No behaviour change.

## F60 [clear/small] Docs that would have saved time 
files: crates/aic-sim/src/docs/run.rs:20-28; docs/speed.md, docs/experiments.md, docs/agents.md

BUDGET_EXEMPT matches CLAUDE.md's list (cards.md, adversary-cards.md, ops.md, rulebook.md, rulebook-friction.md). Three budgeted pages are over DOC_WARN (14,336 bytes), so every docs run already warns about them: docs/speed.md 14,372, docs/experiments.md 14,389, docs/agents.md 14,493. The next item to touch one of them has to trim it first.

ACTION: Trim each of the three below 14 KB, cutting what the code or a generated table already says. docs/experiments.md's flag prose repeats cli::USAGE's experiment section, for example.

# scripts

## F61 [clear/small] Poorly structured (166/167/168/171/177: holds)
files: experiments/results/147_agents_tempo/specs.py, 148_card_pass_r62/specs.py, 149.../specs149.py, 150.../specs150.py, 151.../specs151.py, 154.../specs154.py, 155.../specs155.py, 156.../specs156.py, 157.../specs157.py, 158.../specs158.py, 162_judge_purge/specs162.py, 167_unity_discord_r71/specs167.py, 168_recalibration/specs168.py, 174_speed/specs174.py, 175.../specs175.py, 176.../specs176.py, 177_recalibration/specs177.py:1-30, 178_rules_words/specs178.py:1-30; scripts/bench/read_specs.py:1-15,49-57,123-129

Every rules item since read_specs.py was promoted (159) still writes its own spec writer: 17 copies (specs*.py), each with the same PAIRS table (bp/br/pr) and lever syntax `tag:a=1;b=2`. read_specs.py can't serve them because it only writes into out/ (dump() at :59-63), while a farmed spec must be committed under experiments/farm/ (docs/simulator.md:95-101). Its `read` mode also hard-codes d0-d3 (:126-128), but 168-178 read d0-d4 (f178-*.json, s177-fa.json).

ACTION: Give read_specs.py an `--out-dir` option (default out/, or experiments/farm/ for a farm spec) and a `--levels` option for read (default 0,1,2,3, so `read` output stays as it is). Add `--nobase` and `--games` as in specs177.py. Point docs/experiments.md *Current recipe* and docs/scripts.md at it. Leave the item scripts in experiments/results as the record of their day.

## F62 [clear/small] Poorly structured (166/167/168/171/177: holds (scripts/cards/screen.py reads only native outputs with results.json; it is not the farmed-screen reader))
files: experiments/results/168_recalibration/screen168.py:1-25 (asm + read); experiments/results/177_recalibration/screen177.py:1-30,121-136 (asm + read --causes); experiments/results/178_rules_words/assemble178.py:1-15; scripts/screens.py:117-134 (assemble, function only); scripts/bench/ladder.py:149-177 (--screen)

The farmed-screen read has been rewritten three times. The `asm` step is only a function, screens.assemble, with no command line, so 168, 177 and 178 each wrapped it themselves. screen177.py's `asm` also takes exactly one shard (`[a[4]]`, :124). The `read` step (each lever's win % and its paired gap to base on the same (dir, pair, seed), by agent and level) is what `bench/ladder.py --screen` already prints (ladder.py:149-177), only laid out one row per level. ladder.py lacks two of their features: screen177's loss-cause shares (`--causes`) and a `--base LEVER` choice. The pairs-by-board-pair readers repeat the same pattern: 175/pairs175.py, 178/pairs178.py, 167/pair167.py.

ACTION: Add a CLI to scripts/screens.py (`python scripts/screens.py assemble SPEC FIRST COUNT OUT_DIR SHARD...`). Add `--causes` (loss reasons as a share of losses), `--base LEVER` and a `--by-pair` split to ladder.py --screen and the ladder mode. Document all of it in docs/scripts.md and in docs/experiments.md *Farming a read*.

## F63 [clear/trivial] Docs that would have saved time 
files: docs/experiments.md:134-138, 149-153; docs/scripts.md (bench/read_specs.py, bench/run_read.sh entries)

docs/experiments.md *Current recipe* still prescribes `read_specs.py read` into out/ and `run_read.sh` on both binaries. It names 157/158 item scripts as the readers (`wins158.py`, `rates158.py`, `ends157.py`, `endgame157.py`, `partial157.py`) by bare name with no path, though they live in experiments/results/15x_*/. Every read from 162 to 178 was instead farmed from committed experiments/farm specs and assembled. *Farming a read* (:149-153) gives a different assembly recipe (`aic-sim farm read --header` then the shards by hand) from docs/simulator.md:113 (`screens.assemble`). The page is at 14,389 bytes, under the 1 KB of headroom CLAUDE.md asks for.

ACTION: Rewrite the recipe bullet as the farmed path: spec writer, commit, farm-seeds, assemble, ladder.py, with the old side run natively. Cite the 15x readers by full path, or drop them. Give one assembly recipe and point to simulator.md for the rest. Trim to restore about 1 KB of headroom.

## F64 [clear/small] Refactor or clean up (168: holds (and two of the specs no longer parse))
files: experiments/farm/s177-s3.json:35-36; experiments/farm/s168-d4b.json (variant set `adversary.discord_removed_tips`); crates/aic-sim/src/experiment.rs:1404-1432; crates/aic-farm/build.rs:8

experiments/farm/ holds 36 specs, all built into the wasm bundle. Two are already refused by today's binary. s177-s3.json names agent `strategist:w.last_node_threat=120`, a weight 177 cut that is no longer in Weights (eval.rs has only `last_node`, :386). s168-d4b.json sets `adversary.discord_removed_tips`, which no code in crates/ reads. The fast-lane test every_live_spec_loads_and_its_agents_parse reads only experiments/*.json at the top level, so it never sees farm/. Nothing archives a finished item's s162-*/s168-*/s177-*/f178-* specs, though README.md:47 says specs not tied to the current revision go to experiments/archive/.

ACTION: Move every per-item farm spec (s162-*, s168-*, s177-*, f178-*, onoff167) to experiments/archive/farm/, keeping read.json and cell-d2.json, which the bundle tests use (aic-farm/src/lib.rs:76). Fix effect_stats.py:13 and the write-ups that cite onoff167 to point at the archive. Extend the spec test to parse experiments/farm/*.json. The bundle shrinks, but no game changes.

## F65 [clear/small] Refactor or clean up 
files: scripts/cards/screen.py:97-99, 176, 189-190, 212, 220-231

screen.py's presets read things the engine no longer writes. wins_mix and wins_split (the same function under two headers) print `unity_deck` wins, a reason gone since r67 (state.rs:164-166), and the default preset 041 includes it. `tips` reads alignment.tips_unity and alignment.tips_discord, but the engine now writes alignment.tip_<side>.rNN (alignment.rs:403). `bases_united` (board.bases_united) and `tier2_dealt` (cards.dealt_tier2) are written nowhere in crates/. Presets 028, 018 and 016 are built from them, and loss_mix and loss_ends leave out `round_limit`.

ACTION: Drop the `unity_deck` half of wins_mix and delete wins_split. Point `tips` at a sum of alignment.tip_<side>.r* or drop it. Delete bases_united, tier2_dealt and presets 028, 018 and 016, which are kept for screens from r7-r10 (experiments/archive/scripts). Update docs/scripts.md's preset list.

## F66 [clear/trivial] Refactor or clean up 
files: scripts/cards/use_read.py:22-23; scripts/cards/liberation_read.py:16-17; scripts/configs/no_liberate.py:13-18

use_read.py and liberation_read.py each hard-code the same list of 'liberate cards'. It is stale: great_liberation now prints `cleanse` (content/cards.json) and no `liberate` op. It also misses citywide_microgrid, a deck card that prints `{"op": "liberate"}`. no_liberate.py already derives the set from content, and its no_liberate.json correctly keeps great_liberation and drops citywide_microgrid, so both readers misreport liberate keeps and plays against that config.

ACTION: Move prints_liberate into scripts/screens.py, have both readers build LIBERATE from content/cards.json restricted to the run's deck (screens.dealt), and have no_liberate.py import it. This changes reader output only, not play.

## F67 [clear/trivial] Refactor or clean up (171: holds)
files: scripts/cards/effect_stats.py:35-44, 76-85

deck() keys prohibitions by (step, where) and maps each to one (card, tier). demilitarized_zone's tiers 2 and 3 both prohibit ('muster', 'near_player'), so the later tier overwrites the earlier: tier 2's made, fired and blocked counts print as tier 3's and tier 2 has no row. The engine's tally `prohibit.<step>.<where>.*` (adversary.rs:1174-1180) carries no tier, so the reader can't split them.

ACTION: Map each (step, where) to the list of (card, tier) that print it. Print a shared key once per card, labeled with all its tiers ('tiers 2-3'), instead of attributing it to the last one. If a split is ever needed, adding a tier to the tally is a separate engine change (design).

## F68 [clear/small] Refactor or clean up (166 (play_read needs results.json): holds; docs/scripts.md says so, and four other readers share the limit)
files: scripts/screens.py:137-149 (load), 27-45 (dealt); scripts/cards/play_read.py:37-49; scripts/cards/liberation_read.py:20-32; scripts/cards/economy_read.py:52-54; scripts/cards/energy_read.py:125; scripts/cards/use_read.py:51-54; scripts/cards/screen.py:250,282; scripts/bench/ladder.py:52-71

Readers split two ways. corp_read, board_read, slots_read, threat_read, runaway_read, source_read, loop_smoke, effect_stats, paired and ladder take the cells from the games.jsonl header (cell_labels or cells_of), so they read a farmed run. play_read, use_read, liberation_read, economy_read, energy_read, price_read and screen.py need results.json, either through screens.load and screens.dealt or their own copy of that loop (play_read and liberation_read reimplement it). A farmed run has no results.json. docs/scripts.md's advice for play_read ('a farmed read has no results.json: run it on a screen') no longer helps, since screens have been farmed since 162 too.

ACTION: Give screens.load and screens.dealt the fallback ladder.load already has (cells_of on the header spec when results.json is absent; deck from the header's configs). Switch play_read and liberation_read to screens.load, and drop the play_read caveat from docs/scripts.md. Outputs with results.json read as before.

## F69 [clear/trivial] Refactor or clean up (166 (run_read.sh, no --farm mode): holds)
files: scripts/bench/run_read.sh:1-21; experiments/results/157_strategist_purge_r68/run_read.sh, 158_tier2_cards_r69/run_read.sh, 174_speed/run174.sh, 175_rules_adversary/run175.sh, 176_rules_players/run176.sh

run_read.sh only runs `aic-sim experiment` on read-NNN-TAG-{a,b,c}.json from out/. It has no farm mode and no other spec names, so 174, 175 and 176 each wrote their own runner for the native old side (run176.sh loops four f176-* specs on two binaries). It also builds `"$TOP/$EXE"` (:18), so an absolute EXE path breaks it.

ACTION: Generalize it to `run_read.sh EXE ROOT OUTPREFIX THREADS SPEC...`, any spec list and an absolute or relative EXE. Keep the NNN TAG form as a shorthand, or drop it with read_specs' out/ naming. Document it as the native old side of a farmed ladder (docs/simulator.md:117-120).

## F70 [clear/trivial] Refactor or clean up (175: holds (lines moved to 152-169))
files: scripts/screens.py:150-194 (RENAMED, old_names, cnt), 223-234 (checked_cnt)

The RENAMED map (now at :152-169, no longer :160-164) and old_names still translate counters renamed at 086 and 102, plus chips→hardware from before 026. No games.jsonl is committed anywhere under experiments/ (git ls-files finds none), and the oldest side any current read compares against is the previous revision. Every lookup through cnt and checked_cnt walks a chain that can no longer match.

ACTION: Delete RENAMED and old_names, reduce cnt to `g['counters'].get(k, 0)`, simplify checked_cnt, and drop the RENAMED sentence from docs/scripts.md. Results on any current output are unchanged.

## F71 [design/small] Poorly structured (167: holds)
files: content/alignment.json; scripts/cards/apply_overlay.py:18-21,155-161; experiments/results/157_strategist_purge_r68/apply_tiers.py:1-8

alignment.json is still laid out by hand, one tier per line with text and effects split, not in json.dumps(indent=1) form. apply_overlay.py refuses it (:159 'edit it by hand'), and the docstring still points at 157's apply_tiers.py as the workaround. It is the only content file not written by a tool (threats, adversary and projects come from drafts/adversary/ via load_adversary.py), so each alignment pass edits it by hand or with a one-off. Items 156-178 changed it six times.

ACTION: Either (a) rewrite alignment.json once in json.dumps(indent=1) form so apply_overlay handles it, or (b) teach apply_overlay a writer that keeps the tier-per-line layout. (a) loses the hand layout reviewers read. That is a format choice, but play and the content hash are unchanged if the hash is over parsed content: check that first.

## F72 [clear/trivial] Refactor or clean up 
files: experiments/history.md:9-15; experiments/results/178_rules_words/README.md; experiments/results/165_cleanup/README.md

history.md (the index of write-ups, newest first) has no row for 178 (rules r77, which has a write-up) or for 165_cleanup. The table jumps from 179 to 177 and from 166 to 163. Item 182's own row will also be needed.

ACTION: Add the 178 row (r77, link 178_rules_words/README.md, one-line summary from its header) and a 165 row.

## F73 [clear/trivial] Refactor or clean up 
files: scripts/regen.py:1-16, 105, 111, 116-124

regen.py sets docs/rules/README.md's revision line twice: from RULES_REVISION in source (:111), then from `aic-sim show`'s rules revision (:122). Both come from the same constant, so the second call is a no-op that reprints. The docstring also states the step twice (:6-7 and :10-11).

ACTION: Keep the source-based call. After `show`, only print the hash and revision and warn if they disagree with source_revision(). Fold the docstring into one mention.

## F74 [design/small] Needs a decision 
files: scripts/bench/gate_close.py; scripts/bench/gate_stop_rate.py; scripts/cards/screen.py:4,248-252,285-287 (--gate); scripts/screens.py:249-328 (gate_games, gate, unspent); experiments/gate.json, gate-c3.json, gate-exploit.json; docs/experiments.md:30,172; docs/principles.md:146

Gates have been suspended since the 2026-11-03 ballpark ruling, but all the gate tooling is live and documented: two bench scripts, screen.py --gate, about 80 lines of screens.py, three top-level specs and their docs/scripts.md entries. screens.gate_games is used only by screens.gate.

ACTION: Matt's call: keep it until a milestone revives gates, or archive the scripts and specs to experiments/archive/ (and experiments/archive/scripts/) and trim docs/scripts.md, docs/experiments.md and README.md:47.

## F75 [clear/trivial] Docs that would have saved time 
files: docs/scripts.md (loop_smoke, payback, gate_close entries); scripts/cards/loop_smoke.py (add_argument --seed, --threads); scripts/cards/payback.py (--cards); scripts/bench/gate_close.py (--cap, --fill, --z); scripts/tune/sweep.py:39 (reads `board.destroyed`)

Every script under scripts/ has an entry in docs/scripts.md, and every entry names a file that exists. Every aic-sim flag the scripts pass is still in cli.rs. Every file write passes encoding='utf-8' with newline='\n' (price_read uses newline='' with csv lineterminator '\n', which is equivalent). A few flags are undocumented: loop_smoke's --seed and --threads, payback's --cards (a proposed cards file), gate_close's --cap, --fill and --z. docs/scripts.md is 14,029 bytes, about 1.3 KB under the limit.

ACTION: Add loop_smoke `--seed S --threads N` and payback `--cards FILE` to their entries. Leave gate_close to the gate decision above.

# docs

## F76 [clear/trivial] Docs that would have saved time (167: holds)
files: docs/experiments.md:174-177; experiments/results/178_rules_words/README.md:6,61-63

The seed ledger still says 'Used: every seed below 42853 ... Next free: 42853'. 178 spent 42853-42996 (fresh ladder: strategist 42853-42996, greedy 42853-42900, oracle 42853-42860) and its write-up says 'Next free seed 42997'. 179 and 180 spent none. The next item that trusts the ledger would reuse 178's fresh seeds as if they were fresh.

ACTION: In docs/experiments.md:176, change 'below 42853' and 'Next free: 42853' to 42997.

## F77 [design/small] Docs that would have saved time (177: partly)
files: docs/experiments.md:131-139,149-153; scripts/bench/read_specs.py:128-131; experiments/farm/f178-strategist.json; docs/experiments.md:115-117

'The ballpark read' still calls `read_specs.py read` the 'Current recipe'. That subcommand hard-codes d0-d3 (read_specs.py:130, `[0, 1, 2, 3]`). The last two ladders (177, 178) ran d0-d4 from committed farm specs (experiments/farm/s177-fa.json, f178-strategist.json) through farm-seeds, and docs/principles.md:65-67 tracks d4. The farm route appears only as a bullet further down (149-153), and its assembly recipe (`farm read --header` then the shards) differs from simulator.md:113 (`screens.assemble`). 'Reading noise' (115) gives SE per seed count without saying whether that is per cell, per pair or per level: a standard 48-seed set is 144 games a level (about ±4), not ±7.

ACTION: Make the farmed d0-d4 spec the current recipe, with one assembly method. Either add d4 to read_specs `read` or mark it legacy (adding d4 to the standard read changes what is measured: Matt's call). Say 'per cell (one pair, one level)' in the noise sentence.

## F78 [clear/trivial] Docs that would have saved time (171: fixed)
files: docs/experiments.md:110-111,115-117

Both 171 items are now present. The screen-sizing line is at 116-117 ('A screen's SE is about 50/√n points over n paired games: size a screen so its SE is at most half the effect', 171). 'Diagnose a rewrite's gap by part before tuning' (171) is at 110-111.

ACTION: None.

## F79 [clear/trivial] Docs that would have saved time (177: fixed)
files: docs/rules/threats.md:115-120; configs/baseline.json:77-82,108-112; content/adversary.json:47-48; docs/rulebook.md:1011-1021

The 177 note is fixed. threats.md now says level 1 adds '4 more credits' (`counter_bonus_credits` 4, `counter_bonus_at` 1) and level 2 adds 1 more again with credits unchanged (`counter_more_credits` 0, `counter_more_at` 2), plus the Militarist's line of 4 compute for each unpaid threat (`board_difficulty_at` 2; adversary.json:48). That matches the rulebook's §21 table. One small inaccuracy remains: 'difficulty 1 lets sites corrupt with one fewer agent' (116), but `corrupt_cut` is 1,1,1,0, so land and nodes keep their numbers (adversary.md:55 says so).

ACTION: In threats.md:116, write 'lets cities, stations and factories corrupt with one fewer agent'.

## F80 [clear/trivial] Docs that would have saved time (177: holds)
files: README.md:113-119,131-132; .cargo/config.toml:7-9

README still never says that `--no-fail-fast` is a cargo flag and so cannot follow `cargo test-all` (the alias ends in `-- --include-ignored`, so anything after it reaches the test binary). A full run that should not stop at the first red crate is `cargo test --workspace --no-fail-fast -- --include-ignored`, which the README doesn't give. The same no-fail-fast fact also appears twice, at 118-119 and at 131-132.

ACTION: In README.md:118-119, write: 'A red crate stops the run: `cargo test --workspace --no-fail-fast` (fast lane) or `cargo test --workspace --no-fail-fast -- --include-ignored` (full; not through `test-all`, whose args go to the test binary)'. Drop the parenthetical at 132.

## F81 [clear/small] Docs that would have saved time (168/167/177: partly)
files: docs/simulator.md:91-94,99-101,106-112,113,117-120

Most of the farm notes are now in. Lines 99-101 say a spec built after the bundle isn't on remote workers; 106-112 say farm-seeds runs the worktree exe and builds from HEAD, and that the out file appears only at the end; 117-120 say the old revision runs natively. Still missing: (a) how many local processes the farm-seeds client runs when the laptop is offline. 168's write-up (README.md:116) saw 12 aic-sim processes at once on the main box. (b) The shard costs at 91-94 ('a read seed (12 cells) costs about 51 s natively on one core since 162's judge') predate 174's ~5× speedup: 178's old side ran 2,160 games in ~9 min on 4 threads, about 1 s a game or ~12 s a read seed. (c) 99-101 and 106-108 repeat each other (commit the spec before farming). (d) Line 113 names `screens.assemble` without its path, scripts/screens.py:117.

ACTION: Replace the 51 s / 3.5 min shard costs with r77 figures from 178. Add one line on the client's local process count with the laptop away. Merge the two commit-first sentences. Write `scripts/screens.py`'s `assemble`.

## F82 [clear/trivial] Docs that would have saved time (178: fixed)
files: README.md:133-134

The 178 note is fixed. README *Rules changes* now says 'A new phase goes in `configs/baseline.json`'s `round` list and in every lab test config that lists its neighbor (178: `adversary.compute` beside `adversary.reveal`), or it silently never runs there.'

ACTION: None.

## F83 [clear/trivial] Refactor or clean up (175: holds)
files: docs/harms.md:31-37; crates/aic-sim/src/docs/harms.rs:51-53

harms.md is current: it is at revision 77, and 175's Data Heist 'insight lost' row is there. But `RULE_HARMS` lists two separate 'corruption, node destroyed' rows (harms.rs:51 and :53). The first sits between the income-and-rally rows and the botnets row, so the generated list reads rally, rally, rally, rally, corruption, botnets, corruption... (harms.md:31-37).

ACTION: Move the harms.rs:51 entry next to :53 (or merge the two into one sentence), then rerun `aic-sim docs`. Only the order and wording of the generated prose move, not play.

## F84 [clear/trivial] Docs that would have saved time 
files: docs/agents.md:158; docs/principles.md:65

The tier table gives the strategist a target of 'd2 45-85'. principles.md:65 sets the per-level bands Matt picked at 159 (d0 90-100, d1 70-80, d2 55-65, d3 40-50; `ladder.py --bands`), and 178 judged against those. 45-85 is the older ballpark 'a pair' target (principles.md:62), so agents.md states a superseded target as the current one.

ACTION: In agents.md:158, give the per-level bands (or 'the per-level bands, principles.md') in place of 'd2 45-85'.

## F85 [clear/small] Docs that would have saved time 
files: docs/agents.md:160,162-163; docs/principles.md:65-67; experiments/results/178_rules_words/README.md:65-73

The current ladder numbers are r73's. agents.md:160 gives the oracle as 'd3 95.8, d4 100 at r73 (174)' and 162-163 give 174's r73 strategist and greedy ladder. principles.md:65-67 strings 168, 174 and 177 together and stops at r76. Since then, 178 (r77) read the strategist at 90.7 / 81.0 / 68.1 / 55.8 / 50.0, over band at d1-d3, and the oracle d4 at 100.0. Accumulating per-item numbers in a page goes against CLAUDE.md's 'Docs state the current game ... History goes in the item's write-up'.

ACTION: Replace agents.md:160-163 and principles.md:65-67 with the r77 numbers from 178, one line each, and leave the history to the write-ups and rules-history. This also frees bytes on agents.md, which is over the warn line.

## F86 [clear/small] Slower than expected 
files: scripts/bench/speed_baseline.json (rules_revision 73); scripts/bench/speed.py:229-258; CLAUDE.md:100-106; crates/aic-core/src/lib.rs:60

The speed baseline is still at r73 while `RULES_REVISION` is 77. speed.py prints 'NO VERDICT' and passes whenever the revisions differ (lines 244-252), so the `speed.py check` that CLAUDE.md requires of every rules, state or agent change has given no verdict for 175, 176, 177 and 178. Agent changes such as `w.last_node`, `energy_rule`, `converge_rule` and `botnet_reach` were never checked against a same-revision baseline.

ACTION: Run `python scripts/bench/speed.py check target/release/aic-sim.exe --update` on a quiet box with a known-good r77 build (after `per_decision.py` r73 against r77 confirms no unexplained slowdown), and commit speed_baseline.json.

## F87 [clear/small] Refactor or clean up 
files: docs/agents.md (14493 B); docs/experiments.md (14389 B); docs/speed.md (14372 B); crates/aic-sim/src/docs/run.rs:20-23

Three budgeted pages are over `DOC_WARN` (14 × 1024 = 14336 bytes), so `docs --check` warns on each and the next item that adds a line must trim first. CLAUDE.md asks to 'leave about 1 KB free'. README.md (14286 B) has only 50 bytes of margin before the warn line.

ACTION: Trim each below 14336. agents.md: replace 160-163's r73 history with one current line. experiments.md: merge the two farm-assembly recipes and shorten the 'Costs (r73, 174)' bullet. speed.md: cut 174's profiling history already in its write-up.

## F88 [clear/small] Refactor or clean up 
files: experiments/farm/ (34 specs: s162-*, s168-*, s177-s1..s15, s177-f*, f178-*, onoff167); crates/aic-farm/build.rs:3; README.md:47,49

experiments/farm/ holds 34 specs, of which only `read` and `cell-d2` are standing (simulator.md:95). The other 32 are one item's screens from r69-r77 and are all compiled into the farm bundle (aic-farm/build.rs embeds the whole folder). README.md:47 says specs not tied to the current revision go to `experiments/archive/`, but the farm folder was never pruned. README's `experiments/` row also doesn't mention `experiments/farm/` or `experiments/configs/`, and its standing-spec list leaves out `same-noplanner.json`.

ACTION: Move the item specs from earlier revisions (s162-*, s168-*, onoff167, and s177-* if r76-only) to experiments/archive/farm/, which the bundle doesn't embed. Add `farm/`, `configs/` and `same-noplanner` to README.md:47. Moving them changes the bundle's contents only, not play.

## F89 [clear/trivial] Refactor or clean up 
files: docs/rules-history.md:15

The r74 (175) row is the only recent row without its write-up link. Rows 76, 75 and 77 end with `([NNN](../experiments/results/...))`, but r74 ends at the agents sentence, though experiments/results/175_rules_adversary/README.md exists.

ACTION: Append `([175](../experiments/results/175_rules_adversary/README.md))` to the r74 row.

## F90 [design/trivial] Refactor or clean up 
files: README.md:77-85; docs/scripts.md:16; README.md:60-75; web/README.md:151-154

README's 'Card text for review' section repeats `deck_text.py`'s usage, which docs/scripts.md:16 already documents. CLAUDE.md says a script's flags belong in docs/scripts.md. The Viewer and Card viewer sections repeat web/README; the card viewer one is kept on purpose for conductor's button (line 74). README is near its warn line (14286 B).

ACTION: Reduce 'Card text for review' to a one-line pointer to scripts.md, or ask Matt whether he wants it at the top level as a quick reference. Keep the card viewer block.

## F91 [design/trivial] Refactor or clean up 
files: docs/principles.md:54-59

The 'Calibration point (2026-10-31)' paragraph, about the planner at 60-75% and reading a band over 96 seeds, is superseded twice: by 'Ballpark simulation' (59, which says 'supersedes the band above') and by the per-level bands (65). The page still states it in the present tense, beside text that overrides it.

ACTION: Ask Matt whether to drop the superseded calibration paragraph (git keeps it) or keep it as the milestone target he may return to.

# web-content

## F92 [clear/trivial] Refactor or clean up (178: holds)
files: web/card/card.js:49,55,182; web/card/card.css:32,39; docs/card-guidelines.md:171

The red 'at most' requirement plate is still rendered for `card.at_most`, but no face sets that field. `git grep at_most` in crates/ and content/ finds nothing, and card-guidelines.md:171 itself says 'no card requires at most'. The JS branch, the CSS rule, the JSDoc mention at card.js:182 and the comments at card.js:49 and card.css:32 are all dead.

ACTION: Delete the `card.at_most?.length ? plate('at-most', ...)` line (card.js:55) and `.card .requirement.at-most` (card.css:39). Drop 'at most red' from card.js:49, 'at_most' from the JSDoc at card.js:182 and 'at most' from the card.css:32 comment. Run `npm run cards:check` to confirm no face changes.

## F93 [design/small] Needs a decision (178: holds)
files: content/map.json:214,227-232,292,304,365,378,443,455; crates/aic-core/src/modules/board/production.rs:391,401,419-421; crates/aic-core/src/modules/effects/resources.rs:99; crates/aic-core/src/content.rs:1365; docs/rules/board.md:98-99

All four megaprojects print 'each adversary cube <payout>' and 'An adversary cube made here afterward is lost', and carry an `adversary` op list. Nothing ever writes `project_key(c, None)`: project_cube writes `Some(seat)`, and stations write a seat or the neutral key (production.rs:364-367). So `complete_projects` always reads adv=0 and never runs `p.adversary`. board.md:98 admits the payout never pays, but the cards still print it. content.rs:1365 also refuses a megaproject with an empty `adversary` list.

ACTION: Matt decides: either (a) delete the adversary payout and the 'adversary cube ... lost' sentence from the four map.json texts, drop the `adversary` field (and the non-empty check at content.rs:1365), and remove the adv branch in complete_projects. That moves the content hash and docs, but not play: the payout never fires, so it is a content-hash move with no RULES_REVISION bump. Or (b) add a rule that puts adversary cubes there.

## F94 [design/small] Needs a decision (178: holds)
files: content/ais.json:325 (Protector innate 'Wall' text); drafts/adversary/militarist.json:27-36; content/adversary.json:28-29; crates/aic-core/src/modules/adversary.rs:1157

The Protector's Muster landing ('1 Troop lands on the lowest-numbered adversary or neutral factory of each country where you have a Node') is printed on the Protector's innate in ais.json. It is printed again as the Militarist board rule 'Troops where the Protector stands'. The engine implements it once, in the adversary module, reading `wall_troops`. The innate's passives are only `wall` and the Vigil `when`, so the ais.json sentence is a reminder, not an effect.

ACTION: Matt decides which component carries the printed rule. Most likely, keep it on the Militarist board, where the engine reads it, and cut the sentence from the Protector's innate text, or shorten it to a pointer. This is a face text change: rerun `aic-sim docs`, cards_doc.py and `npm run cards:fit`.

## F95 [clear/trivial] Docs that would have saved time 
files: drafts/adversary/README.md:15 ('militarist.json ... written by the same generator: rerun it, never edit by hand'); drafts/adversary/militarist.json:28-34; experiments/results/151_adversary_income_r65/adversary_cards.py:314-316

The README says the drafts are written by the 146/151 generators and must never be edited by hand. In fact militarist.json, threats_t1.json and projects.json were hand-edited by 166, 169, 175 and 178 after the generators' last commit. Example: militarist.json:29 reads 'Muster 1 on its lowest-numbered adversary or neutral factory (none: none lands)', while adversary_cards.py:315 still prints '1 adversary troop lands on its lowest-numbered factory (else city), never a hex Wall holds'. Rerunning the generator as the README says would silently revert those rulings.

ACTION: Rewrite the README row: the drafts JSON is now the source and is edited directly. The 146 and 151 generators are historical and must not be rerun.

## F96 [clear/trivial] Refactor or clean up 
files: web/scripts/cdp.mjs:66-73 (shotsPage), cdp.mjs:38 (ACTING)

`shotsPage()` has no caller anywhere in the repo. Its only users were web/scripts/shots-017/019/021.mjs, which 165 deleted. AIC_SHOTS is now handled inline by e2e.mjs:32-38. `ACTING` is exported but used only inside cdp.mjs (line 45).

ACTION: Delete shotsPage (and `mkdirSync` from the import if it is then unused). Drop the `export` on ACTING.

## F97 [clear/trivial] Refactor or clean up 
files: web/src/style.css:180 (.escal), web/src/style.css:297 (.row.cards-packet, .row.cards-drafted)

Dead CSS. `.escal` was emitted only by the adversary-card escalation mark, which 138 removed (commit 0fa33f7). No TS emits a `row cards-*` class: panels.ts:308 builds `class="row"` only. The packet and drafted rows are styled by nothing that renders.

ACTION: Delete style.css:180 and :297.

## F98 [design/small] Poorly structured 
files: drafts/adversary/check.py:51-62; crates/aic-core/src/modules/adversary.rs:256-264

The deck dealt at each difficulty is computed twice: once in Rust's `build` and once in check.py's `stage_deals`. Both hard-code `difficulty >= 3` for stage1_added/stage2_removed, a tunable threshold in code where the other thresholds are config keys (`stage_swap_at`, `extra_threats_at`, `board_difficulty_at`). check.py also carries stale fallbacks: `a.get('stage_swap_at', 2)` (baseline is 4) and `a.get('extra_threats_at', 1)` (baseline is 5). These keys always exist, so the fallbacks only mislead a reader.

ACTION: Clear part: in check.py use `a["stage_swap_at"]` and `a["extra_threats_at"]` with no fallback. Design part: replace the literal 3 with a `stage2_cut_at` param (value 3) in baseline.json and both readers. That changes the config hash and shared_row.json and needs a golden-constant check.

## F99 [clear/small] Refactor or clean up 
files: web/card/card.js:8; web/cards/plain.js:4; web/scripts/sweep.mjs:425; web/src/format.ts:301

There are four HTML-escape helpers. card.js and sweep.mjs escape `& < > "`, and card.js turns null into 'null'. plain.js also escapes `'` and maps null/undefined to ''. format.ts has escapeHtml. card.js and plain.js are loaded on the same viewer page.

ACTION: Have web/card/card.js import `esc` from a single shared module (for example, export it from card.js and have plain.js reuse it, or the reverse). Pick the plain.js variant, which also escapes `'`. Then check with `npm run cards:check` that no face text changes.

## F100 [clear/trivial] Refactor or clean up (175: wrong)
files: experiments/results/163_remote_reread/tab_worker.mjs:4,9-16

The note is wrong as stated: tab_worker.mjs does not create a %TEMP% profile. PROFILE_DIR is a required argument (line 10), created with mkdirSync at the caller's path. It never removes that directory afterward, unlike web/scripts/cdp.mjs:176. Nothing in the repo invokes it; it is a frozen run record.

ACTION: None needed. Optionally add a line to its header comment saying the caller owns and must delete PROFILE_DIR.

## F101 [clear/trivial] Refactor or clean up (178: holds)
files: experiments/results/154_endgame_r66/endgame154.py:75; crates/aic-sim/src/endgame.rs:58

endgame154.py:75 still reads `p["remove_after_liberate"]`. endgame.rs:58 records that 178 removed that field from the probe, so running the script on a current build's results.json raises KeyError. It is a frozen 154 record, and nothing current runs it.

ACTION: Either leave it as a record and add a header line saying it reads r66-era probe output only, or change line 75 to `p.get("remove_after_liberate", 0)`.

## F102 [clear/trivial] Docs that would have saved time 
files: configs/baseline.json (modules[].params); configs/agents/values.json (rates, ops)

I checked for config keys no code reads and found none. Every baseline.json tunable has a `"module.key"` lookup in non-test Rust, or a ParamSpec plus `b.list` for stage_cards. Every values.json rate is read by value.rs. The six op rows that are not Op variants (remove_ambassador, cleanse_station, liberate_align/troops/botnet, typed_jam) are sub-rows listed in value.rs:94 and read at value.rs:536-588. No action needed. Recorded so the next cleanup doesn't redo the sweep.

ACTION: None. Optionally note in docs/architecture.md that value.rs:94 lists the op rows that are not Op variants.

## F103 [clear/trivial] Refactor or clean up 
files: picks/scene_batch_1.json .. scene_batch_5.json, picks/scene_style_r2.json; docs/qa-findings.md:76

Six picks manifests are referenced by nothing in the repo: no script, doc or write-up names them. qa-findings D21 already chose to keep superseded manifests as a record, and README.md:57 describes picks/ as the pickers' round manifests. They are records, not dead code.

ACTION: No change, per D21's pick (b). If wanted, add one line to README.md:57 saying older rounds stay as records and nothing reads them.

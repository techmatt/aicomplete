# 266: refuse bad input in agent specs and read scripts (r124, no bump)

261's D117 ([deferred.md](../261_bug_hunt/deferred.md)): twelve inputs the agent specs and read scripts took
silently. Each now stops with a message naming the input and what is allowed, or (two of them) reads right. No rules
change; a valid spec plays as before: `same266` (strategist, engine, `strategist:strategy=1,strat0=2`,
`greedy:kept_rule=2`; the three pairs, d2, seeds 1000-1015, 192 games), old binary against new: `bench/same_results.py`
"same: 192 games".

| # | input | before | now | test |
|---|---|---|---|---|
| 1 | `plan_credit` | documented, read by nothing since 189 | retired (`policy::RETIRED`), refused with the reason; its row left `scripts/tune/agent_sweep.json` | `specs_refuse_what_they_would_misread` |
| 2 | a value read as another: an on/off setting not 0 or 1, a count's fraction, `kept_rule` > 4, `sell_rule` > 3, `burn_late` > 2, `no_spend` > 3, `truth` > 2, `prove` past the card list, a tier's count (`width=2.5`) | played as another setting under its own label (`kept_rule=5` as 3, `no_spend=4` off, `width=2.5` searched 3 and kept 2.5's label; `prove` past the list panicked on an index) | refused at parse (`ForcedValue::check` by type, `policy::MOST`, `policy::check_param`); `margin`, `lead`, `plan_niche` stay real numbers | same |
| 3 | `strat0`-`strat2` past the seat's board's menu | played the menu's first | `AgentSpec::check(&Rules)` refuses it in `experiment::cells_with` and `run`; `strategy::choose` panics with the reason on any other path | `pinned_strategies_checked_against_the_game` |
| 4 | one seat pinned with `strategy=0` | every unpinned seat chose a strategy too | refused unless every seat is pinned: set `strategy=1` (the rest choose) | same |
| 5 | `t.troops_removed.<name>` | any name taken, read by nothing | a count of 1 or more, as written (`03` refused) | `specs_refuse_what_they_would_misread` |
| 6 | the target rule's table | the global table whatever the spec's `t.` numbers | the agent's own table (`moves::target` takes it); a spec with no `t.` reads the same table object | `target_rule_reads_the_agents_table` (slow lane: the rule at both tables, and a `t.`-zeroed strategist leaving a prompt the plain one answers to its search) |
| 7 | `apply_overlay.py` `null` on a misspelled field | dropped nothing, silently | refused when no row of the table holds the field and content.rs declares none (a real field the row lacks is a no-op, so an overlay reruns) | `scripts/test_input_checks.py` |
| 8 | `play_read.py` / `use_read.py` pooling a native folder with a farmed read | one agent in two (the resolved label against the spec's name) | `screens.cells_for` names agents as the spec writes them (`named_agents`, as `ladder.py`); `--agents` takes the name or the resolved label, mapped to the name | `CardReadsPool` in `bench/test_read_tools.py` |
| 9 | `design_check.py` unknown flag | ignored | refused (and a flag with no value or given twice, and a DIR with no games file) | `scripts/test_input_checks.py` |
| 10 | `design_check.py` second DIR | ignored | refused: one DIR, comma lists pool | same |
| 11 | `read_lib.py --base` an arm it lacks | the default arm, silently | refused, listing the arms | `ReadLib` |
| 12 | `paired_gap` on one paired seed | gap 0, new = base (`screens.gap` gives 0 below two pairs) | the real difference, SE `None`, printed `+100.0, n 1` | `ReadLib.test_one_paired_seed` |

## Decisions (the staging defaults; no answer came)

- **6 fixed, not refused**: a spec with `t.` numbers now prices targets at its own table, so its games change; no
  live spec, judge or baseline sets `t.` (the `same266` check covers the defaults).
- **4 refused, not reinterpreted**: unpinned seats still commit under `strategy=1`; `strategy=0` with a seat unpinned
  is refused. `experiments/archive/screen-082-table-*.json` pin two of three seats under the strategist and would be
  refused if rerun: left as they are (nothing reads `archive/`).
- **On/off settings take 0 or 1 only.** No live spec in `experiments/*.json` set another value (their test passes,
  and now runs `check` on each spec that pins).
- **12 reported, not refused**: one paired seed is a real (if thin) read; it prints with no SE.

## Side effects

- Every read script using `screens.cells_for` or `load` (`play_read`, `use_read`, `liberation_read`,
  `economy_read`, `energy_read`, `price_read`, `screen.py`) now names a native run's agent as the spec writes it
  (`strategist`, not its resolved `engine:w.built=4,...`); game numbers are unchanged. Two outputs move beyond the
  label: `screens.gate` (gates are suspended) now pairs `strategist:keep_all=1` with `strategist`, its own agent as
  its docstring says, where both resolved to `engine:...` and paired with a plain `engine` cell; and `price_read.py
  --agent` takes the spec's name or the resolved label.
- `apply_overlay.py` knows a field from the table's rows and from the `pub` fields content.rs declares, so an
  overlay rerun after its `null` dropped the last copy of a real field is still a no-op, not a refusal.
- `read_lib.py --base base` finds a read- or config-led arm (`stem/base`) by its last part, as the default does.

Real reads: `play_read.py` on `same266` (native) with 263's kept farmed read
(`f263-read.jsonl=f263-read.spec.json`) gives one strategist of 768 games (48 + 720); `use_read.py`, `read_lib.py`
and `design_check.py` ran on `same266` unchanged but for the labels.

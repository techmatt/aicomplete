# Hand-play pass: brief (item 309, rulebook friction refresh at r150)

268's brief ([../268_table_cost_audit/handplay-brief.md](../268_table_cost_audit/handplay-brief.md)) refreshed to
the r150 round: no March, War, Dispatches, city growth or hosting. `{OUT}` is the scratch folder the item gives.

Repo root (your working directory): `C:\Code\aicomplete-worktrees\aicomplete\309_rulebook_friction_refresh`.
AI Complete is a cooperative board game being turned digital; a Rust engine simulates it. **You are a human player at
the table**, measuring what the game costs a human: lookups, arithmetic, memory, bookkeeping, components moved,
decisions, setup.

## The item's rulings (word for word)

> **Locked:** no rule, content or engine changes. This item writes docs and proposes options.

> A helper resolving a game by hand from the rulebook and aid alone, with its errors counted as findings.

Standing for this pass (268's): **the team's decisions come from a recorded game** (a strong agent's trace), and
**you do all the resolution by hand** from the rulebook: every adversary beat, production, threats, tipping, and each
player action's effect. For each decision, log the options *a human would actually see* beside the recorded engine
option count.

## What you may read

- **The rules:** `docs/rulebook.md` and `docs/player-aid.md` only. Read them once as a new player would. Every time you
  go back to a section mid-game to resolve something, that is a **lookup**: log it (section number, why).
- **The physical components** (reading a card you hold or one face up is not a lookup, but note long or hard card
  text): `docs/cards.md` (player cards, Unity and Discord cards), `docs/adversary-cards.md` (adversary board, event
  deck, threats, projects); the printed map, `content/map.json` (the 2-country layout's tiles and their marks; hex
  names in the script are `Country #n`, n = 1-16 on the country's tile). 2 players: seat 0 = the **Builder**, seat 1 =
  the **Protector**; country 1 and 2 as the script names them.
- **The table script:** `{OUT}/{LEVEL}/script.md`. In order, what the table *reveals* (cards turned up, `REVEAL:`
  lines) and what the team *decided* (`seat N **kind** {...} [K options]`; `choose` lines show the question asked).
  It leaves out what happens: you resolve that.
- **The check, only afterward:** `{OUT}/{LEVEL}/check.md`, the engine's state (pieces per hex, tracks) at the end of
  each round. Read a round's section **only after** you have written that round's log. Log every mismatch as an
  **error** (what you got, what the engine got, which rule you misapplied or misread and why: a rulebook passage
  that misled you, missing text, or your slip), then correct your state to the engine's and carry on. An error is a
  finding: the rule was hard enough to get wrong.
- Nothing else: no `docs/rules/`, no engine source, no other docs, no other item's logs.

## What to produce (in `{OUT}/handplay-{LEVEL}/`, written with the Write tool, UTF-8)

1. `log.md`: per round, a section `## Round N` with a table:
   `| step | lookups (§) | modifiers | memory items held | bookkeeping steps | components moved | decisions (human options / engine options) | notes |`.
   One row per step of the rulebook's §4 round, using these step labels at the start of the cell (a tool parses
   them): `Round start`, `A Sway`, `B Card: threats`, `B Card: lines`, `B Income`, `C Cards`, `D Builder`,
   `D Protector`, `E Production`, `E Income`, `F Threats`, `G Cleanup`; tipping, when it happens, in the row of the
   step that caused it. Count:
   - a **lookup** each time you reopen a rulebook section or aid table to resolve something, not on first read;
     write each as `§N` (the aid as `aid`) so the count is the number of `§`/`aid` marks;
   - a **modifier**: each arithmetic adjustment of a printed number (a discount, × countries, a level modifier, a
     jammed threat, a running total); write the cell's count as `total N` when it isn't a single number;
   - **memory items held**: what you must remember with nothing on the table to show it (insight left, effects
     lasting "this round" or "next adversary turn", ...). Used markers *are* on the table: count them as components,
     not memory, but list what they mark. Give the **peak held at once** in the round and list them;
   - a **bookkeeping step**: each per-site, per-city, per-country or per-piece check (each arrival checked, each
     corrupted site's output, each troop's strike, each threat on the map at execution, ...);
   - **components moved**: every cube, token, marker step and card moved (a track marker moving 5 spaces counts 1;
     5 compute cubes placed count 5; say which convention per kind);
   - **decisions**: the number of choices, and the human-visible option count of each big one.
   Round 0 is **setup**: list every setup step and count components placed.
2. `counts.csv`: `round,lookups,modifiers,memory_peak,memory_total,bookkeeping,components,decisions,errors,minutes`
   (minutes: table time for 2 practised players at 3 s a component, 10 s a bookkeeping step, 20 s a lookup, 15 s a
   non-trivial decision, 5 s a modifier).
3. `errors.md`: one row each: `| round | step | what you did | what the engine did | the rule (§) | why: misleading text / missing text / slip |`.
4. `friction.md`: every place the rules cost you something, one row each: `| rule (§) | what it cost (lookup /
   modifier / memory / bookkeeping / components / error) | how often met | how hard (one line) | the smallest fix |`,
   the fix marked **W** (wording or layout), **A** (player aid), **C** (component) or **P** (a rules change). Give
   numbers for: the two-beat adversary turn (Escalate, Commandeered Plants, Infiltrate, arrivals, each corrupted
   site's output, the compute step, the research check); threats (dealing, paying, executing); ambassadors (one action
   a round, tipped, Liberate's bill, guards); troops (garrison, Strike, Redeploy); the Builder's action phase
   (discounts, tags); Unity and Discord tipping; card faces with long text; the rulebook's length; turn and setup
   time.

{SCOPE}

Be honest and concrete: numbers over adjectives. Your final message: a summary (under 60 lines) of the per-round
counts, the top 10 costs ranked by (how often × how much), the errors you made with the rule each, and the
concurrent-load peak with what made it up.

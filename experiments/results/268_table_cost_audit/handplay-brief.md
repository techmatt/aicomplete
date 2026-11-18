# Hand-play pass: brief (item 268, table-cost audit)

Repo root (your working directory): `C:\Code\aicomplete-worktrees\aicomplete\268_table_cost_audit`.
AI Complete is a cooperative board game being turned digital; a Rust engine simulates it. **You are a human player at
the table**, measuring what the game costs a human: lookups, arithmetic, memory, bookkeeping, components moved,
decisions, setup.

## The prompt's ruling (word for word)

> **A hand-play pass.** An agent plays one full 2-player game at d2 (and spot-checks d4) using only `rulebook.md` and
> `player-aid.md`, as a human would. It may consult the engine only to check its results afterward. Log every:
> - **lookup:** a page or table consulted mid-game;
> - **modifier applied:** arithmetic on a printed number;
> - **memory item:** something to remember across turns or rounds, such as once-a-round uses or what was bought;
> - **bookkeeping step:** something resolved for every city, site or country;
> - **component moved:** cubes, tokens and cards per round;
> - **decision:** with its option count;
> - **setup step.**

Ruling for this pass (staging default, standing): **the team's decisions come from a recorded game** (a strong agent's
trace), and **you do all the resolution by hand** from the rulebook: every adversary beat, production, threats, war,
tipping, and each player action's effect. For each decision, log the options *a human would actually see* beside the
recorded engine option count.

## What you may read

- **The rules:** `docs/rulebook.md` and `docs/player-aid.md` only. Read them as a new player would. Every time you
  go back to a section mid-game to resolve something, that is a **lookup**: log it (section number, why).
- **The physical components** (reading a card you hold or that is face up is not a lookup, but reading text is a
  cost: note long or hard card text): `docs/cards.md` (player cards, Unity and Discord cards),
  `docs/adversary-cards.md` (adversary board, event deck, threats, projects); the printed map:
  `{OUT}/map2.txt` (every hex, kind, neighbors) and the printed tile marks in `content/map.json` → `countries[0..1]`
  (`cities`; `corrupted[0]` the marked corrupted city; `stations[0]` the marked corrupted station that also gets a
  botnet; `neutral_station`, `more_stations`, `factories` start neutral; `station_sites` empty station sites;
  `industry` industrial zones; `node` the starting hex; `coast` extra coastal cities). 2 players = Horizon
  (country 1 in the rulebook's numbering, seat 0 = the **Builder**) and Cintamani (seat 1 = the **Protector**).
- **The table script**: `{OUT}/{LEVEL}/script.md`. It lists, in order, what the table *reveals* (cards turned up,
  `REVEAL:` lines) and what the team *decided* (`seat N **kind** {...} [K options]`; `choose` lines show the question
  asked). It deliberately leaves out what happens: you resolve that.
- **The check, only afterward:** `{OUT}/{LEVEL}/check.md` holds the engine's state (pieces per hex, tracks) at the end
  of each round. Read a round's section **only after** you have written that round's log. Log every mismatch as an
  **error** (what you got, what the engine got, which rule you misapplied or misread and why), then correct your
  state to the engine's and carry on. An error is itself a finding: the rule was hard enough to get wrong.
- Nothing else: no `docs/rules/`, no engine source, no other docs.

## What to produce (in `{OUT}/handplay-{LEVEL}/`, write files with the Write tool; UTF-8)

1. `log.md`: per round, a section with a step-by-step table: `| step | lookups (§) | modifiers | memory items held |
   bookkeeping steps | components moved | decisions (human options / engine options) | notes |`. Step granularity:
   each phase of §4's round (Round start, A Sway, B Card / Corrupt / March / Income, C Cards, D Action phase (per
   player, grouped), E Production steps 1-8, F Threats, G War, H Cleanup) and tipping when it happens. Count:
   - a **lookup** each time you reopen a rulebook section (or table) to resolve something, not on first read;
   - a **modifier** each arithmetic adjustment of a printed number (discounts, ×countries, +1 for a Liberated
     marker, level modifiers, a jammed threat, ...);
   - **memory items held**: what you must remember with nothing on the table to show it (powers used this round,
     cities bought from, converters used, Vigil count, effects lasting "this round"/"next adversary turn", Dispatches
     left, insight left, ...). Give the **peak held at once** in the round (the concurrent load) and list them;
   - a **bookkeeping step** each per-site / per-city / per-country / per-piece check (e.g. city growth: one per city;
     natural corruption: one per site checked; rally: one per site dealt to; march: one per troop);
   - **components moved**: every cube, token, marker step and card physically moved (a track marker moving 5 spaces
     counts 1 move; 5 compute cubes placed counts 5 — say which convention you use per kind);
   - **decisions**: the number of choices, and the human-visible option count of each big one.
   Round 0 is **setup**: list every setup step and count components placed.
2. `counts.csv`: `round,lookups,modifiers,memory_peak,memory_total,bookkeeping,components,decisions,errors,minutes`
   (minutes: your estimate of table time for 2 practised players, from the counts, at stated rates: e.g. 3 s a
   component, 10 s a bookkeeping step, 20 s a lookup, 15 s a non-trivial decision; state your rates in log.md).
3. `friction.md`: every place the rules cost you something, one row each: `| rule (§) | what it cost (lookup /
   modifier / memory / bookkeeping / components / error) | how often met | how hard (one line) | the smallest fix |`,
   and mark the fix **W** (wording or layout), **A** (player aid), **C** (component, e.g. a marker) or **P** (needs a
   rules change). Give special attention, with numbers, to: city growth (§9); hosting (who hosts which city, as a
   matching problem); once-a-round uses with no physical marker; Unity's two tables (§12 tip strength, §14 purge);
   counting agents on seven hexes for projects (§17); compute volume (cubes moved a round); card faces whose text is
   long; the rulebook's length; turn and setup time.

{SCOPE}

Be honest and concrete: numbers over adjectives. Your final message: a summary (under 60 lines) of the per-round
counts, the top 10 costs ranked by (how often × how much), the errors you made, and the concurrent-load peak with
what made it up.

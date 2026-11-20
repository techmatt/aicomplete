# Standing rulings

Matt's standing rulings, as the planner held them on 2026-11-19. Conductor's preamble names this page among the
locked rulings and the diff review is given it whole. A prompt, amendment or later ruling outranks it; an item that
gets a new standing ruling folds it into its line here. Each ruling is recorded as ruled on its date: the game's
current numbers and power names are in [rules/](rules/README.md), and how a rules, tuning or pricing item reads and
reports is in [tuning-items.md](tuning-items.md). "The design doc" is the planner's, outside this repo.

Areas: [Names and terms](#names-and-terms) · [Difficulty](#difficulty) ·
[Simulation and sim players](#simulation-and-sim-players) · [Measures](#measures) ·
[Resources and engines](#resources-and-engines) · [Pricing and standard actions](#pricing-and-standard-actions) ·
[Cards](#cards) · [Boards and pieces](#boards-and-pieces) · [Adversary and harms](#adversary-and-harms) ·
[Unity, Discord and winning](#unity-discord-and-winning) · [Rulebook and audits](#rulebook-and-audits) ·
[Design principles](#design-principles) · [Table time and the card economy](#table-time-and-the-card-economy)

## Names and terms

- **Names (Matt, 2026-11-09/10):** players' pieces are **ambassadors**, the adversary's **agents** and **troops**;
  a player's token is a **Node**, the adversary's a **Botnet**; the Security token prints
  **Operatives**; simulator players are **sim players**. Piece names on cards print capitalized or as the piece's
  symbol; rulebook prose uses plain lowercase.
- **Terms (locked):** "corrupt" is the adversary's one verb for taking sites; the land scar is **Fallout**;
  node loss is "destroy your node".

## Difficulty

- **Difficulty (Matt):** the ladder raises intensity, not count (extra threats only at d4). Bands (raised by Matt
  2026-11-15 for 245's judge, "closer to what I want, we'll see"): d0 at least 90, d1 80-95, d2 75-90, d3 55-70 (no
  ceiling). **Soft until development is further along:** misses are reported, never chased or dialed. Raise player
  power through player-board levers, not cheaper cards; never price anything as a share of income; no `stage_extra`.
- **Matt, 2026-11-17:** at higher difficulty the adversary must spread, corrupting more cities before the players
  can purge them (an adversary purged before it takes new cities is bad design).

## Simulation and sim players

- **Simulation scale (Matt, 2026-11-03):** 2 players, pairs B+P, B+R, P+R, 48 seeds a cell, d0-d3 (d4 read too),
  paired against the previous binary on the same seeds; a band check pools two independent sets; dev screens at 32
  seeds. A rules item is about 3-4 hours.
- **Sim player tiers (Matt, 2026-11-03/09):** greedy = new player; strategist = the judge; planner = spot checks.
  Fixes are sentence-sized heuristics; sim players don't shape the design.
- **The judge:** since 245, greedy's weights with a card in hand worth `w.hand` 1.5 + `w.hand_cost` 0.1 × its cost,
  + `w.purge` 3.6 + last-node guard, **no strategy**; since 249 also `w.card_action` 3: a building counts what it
  earns a round beyond income, its Action's share included, for 3 rounds. Matt 2026-11-15: a better judge is wanted,
  a good player but not oracle-scale, and the bands are a heuristic re-read on it; win rate on fresh seeds decides;
  oracle agreement (`aic-sim agree`) guides tuning only against an oracle not built on the judge.
  `strategist:strategy=1` is an information cell.
- **The real oracle** (249: `oracle:worlds=K`, skilled_rows.py `real`, sampled hidden information, built on the
  pre-249 judge weights, `tries` refused with `worlds`) reads at the judge's level (80.0 vs 78.3% d0-d4, about 217 s
  a game against 1.6) because it mostly plays the judge's policy on a guessed world: the full oracle's edge is sight
  of the deck.
- **The oracle** (`scripts/bench/skilled_rows.py`, local; since 243
  `oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3`, restarts whose wins are a superset of the
  single oracle's; card proofs `oracle:...,prove=N`; its keeps come from the engine's keep plan, never its search)
  proves a seed winnable; short of 95 is a direction for buffs, not a blocker (2026-11-10).
- **Remote workers (Matt, 2026-11-09):** take speed over bit-exactness unless the gain is trivial; paired reads pair
  same-engine seeds.

## Measures

- **Measures:** same seeds, paired; credits read after the last completed action phase; leftover resources and
  their sinks (soft). **Every resource should be an interesting thing to spend (Matt, 2026-11-11):** each rules item
  reports how each resource is spent and whether it binds.
- **Watch list (Matt, 2026-11-11):** insight binding at keep and play; Patience carry-over; neutral claim timing;
  Vigil; Fallout's Discord drip.
- **Harms ledger:** [harms.md](harms.md), kept current by every prompt touching threats, cards or the adversary; 191
  makes it complete (no insight row; "costs raised" paged).

## Resources and engines

- **Board facts (candidates):** slots 5 / 4 / 4; boards print 10 / 9 / 7 credits and +1 energy (Protector 2);
  reach 1; converters need a node in a city of their country.
- **City energy (Matt, 2026-11-19):** each city that is neither corrupted nor blocked pays 1 energy to every player
  with a node on or next to it, on the energy income track; no hosts and no buying. **Resources are not merged.**
- **Insight:** this round's attention, spent this turn; the card-play track prints absolute values (starts at 3,
  then 199's early-heavy t4 steps); keeps cost 1 insight tier-1, **3 tier-2** (2 upgraded); Patience keeps unspent
  insight (caps banned); no adversary card takes insight; the opening deals 2 tier-1 + 2 tier-2 free; tier-1 play
  costs are credits (4 a point). Matt's aims (design doc *Economy*): about 6 a round
  to drafting and 2-4 to plays (directional); more tier 2 earlier; keeps are a plan; insight must bind, judged by
  slack beside bound rounds, no target. **Draft hand exactly 4 cards (locked).**
- **Chips (Matt, 2026-11-11/18):** a chip is worth 4 credits as a pricing anchor, not an exchange:
  chips are their own resource, not cut, never trivially exchanged for credits without an engine. City growth is
  gone: the Grow line left the event cards at r140, and new neutral sites come from site-opening lines.
- **Engines over dials (Matt, 2026-11-12):** chips, energy, antimatter and research cubes feed engines; when players
  skip one, buff its engines until skipping it is clearly bad play; easy levels are met by stronger engines, not
  adversary patches. A claim makes 1 chip a round; megacorps convert chips once their project completes.
- **Converters (Matt, 2026-11-17):** strengthen Tidal Grid's converters (chip converter "2 chips → 3 energy") and
  the other unused converters.
- **Antimatter and majors (Matt; design doc *Economy*, *Cards and draft*):** antimatter is epic, anchor 18. Every
  board keeps a source, each a different decision: board powers are rare burns of your own engine (built r97); the
  steady source is tier-2 **converter plants**, a different resource each, no free producers; Make antimatter (5
  cubes) stays the desperate hedge. **Tier-2 cards are the majors:** big payoffs funnelling resources into own or
  others' board actions, enough to win, many spending antimatter; winnable without them at d0 and under the oracle;
  the draft price is their only acquisition cost. **Goal (Matt, 2026-11-19):** antimatter is powerful when it works,
  something you work toward; not every player need get it working, but at high difficulty probably some do; other
  things must also win, so not every strong end-game engine is antimatter-focused (sinks start at 2.0-2.5×, scaled
  back if overwhelming).

## Pricing and standard actions

- **Pricing:** at the unit anchors (fair 1.0-1.5×; at most ~1.5× tier 1, ~2.5× tier 2); never lowered to keep a
  band; unplayed cards get stronger, not cheaper; when `values.json` disagrees, fix the table. No holding or
  production limits: a runaway pile is fixed at card level. The anchors' table and the worked checks:
  [card-guidelines.md](card-guidelines.md).
- **Standard actions (Matt, 2026-11-12):** poorly priced hedges, never the better route; none removes corruption
  except Lift (about 2 research cubes); Liberate needs a site free of agents, and of troops on or next to it
  (2026-11-19), and is the win route.
- **Matt, 2026-11-17:** **cut the Claim standard action** (cards claim).
- **Sell a card is exempt from job-no-card** (Matt, 2026-11-16): no card needs to sell.

## Cards

- **Card and board rules (design doc):** programs one-shot, buildings ongoing; tier is efficiency, not magnitude;
  useful alone; removal in reach; exchanges one use; no frictionless transfers; gifts say "another player"; no "up
  to N a round"; no paid Move; a sold card gives 1 credit. Fallout also comes off as a side bonus on green-feeling
  cards, never a bare removal card or a tier-2 multi-clear. Convert and engine-use cards (no anchor for their unit)
  are priced in band under both readings. The checklist that works these through, with examples:
  [card-guidelines.md](card-guidelines.md).
- **Tags are flavor and expectations, not rules (Matt, as MtG colors):** no tag owns an effect; an effect needs only
  a story fit.
- **Card targets (Matt, 2026-11-14, locked; built r104-r107):** a map card, board power or standard action names
  one target, a hex in reach (1, fixed); a header target box prints its condition; effects act on the target only
  unless printed "on or next to the target" (most tier 1 target-only; tier 2 sizes up with radius 1, never 2); never
  two targets; **Gain** = your supply, **Place** = onto the map; no card picks a country freely ("in the target's
  country"); Shelter is "your nodes and facilities in reach". Kept by 234's picks: megaproject-cube cards keep any
  open megaproject; Peacekeeping Mission's "in its country".

## Boards and pieces

- **Nodes (design doc):** nothing places a node anywhere (next to your nodes, occasionally range 2; cards may target); node
  powers give their bonus by position.
- **Board shape (Matt, 2026-11-18):** boards keep Spirit Island's shape (Growth one easy + two costly, tiered
  sacrifice powers, at most a few "≥ N tags" threshold powers where hoarding is the point).
- **Moving pieces (Matt, 2026-11-09):** Gather and Push: up to N, distance 1, into or out of a hex in reach;
  ambassadors, agents and troops move, never botnets or markers. Liberate spends any seat's ambassadors on the site
  first.
- **Ambassador actions (Matt, idea, 2026-11-10):** ambassadors stand "ready" and tip over when used (move 1 hex by
  default; some boards add a botnet removal); would replace the Movement resource. Open whether it adds more than it
  costs.

## Adversary and harms

- **Adversary (locked):** the threat card standard (146: one consequence, icons over text, one or two counters, no
  hidden harm, no tier 3); the round cap stays a plain loss.
- **Threats (Matt, 2026-11-19, locked):** a threat that takes effect harms the player of the country it was dealt
  in; any player with a node or ambassador on or next to one of its tokens may pay. **No other rule may depend on a
  player's home country.** Threat decks are built once at setup, with the mix the adversary's board names (its lean:
  the Militarist's stays Security-heavy), and dealing never rejects a card in a round: the dealt card is the next
  one, and one with no legal site takes one plain fallback.
- **War is cut (Matt, 2026-11-19):** no fronts, military bases or megacorp armies and no War step; megacorp conflict
  lives only as content on Discord and event cards, each doing one clear thing.
- **Card-driven (Matt, 2026-11-18):** the adversary is card-driven with look-ahead and checks on arrival, never a
  board sweep.
- **Adversary spine (Matt, 2026-11-19):** an event card is Infiltrate X plus escalate: each site of last round's
  type that holds an agent takes 1 more (Spill and Surge's "most agents" picks go). **One output a corrupted site:**
  city 1 Discord; station 1 research cube; factory 1 agent (made on itself, pushed next round along its zone's
  printed arrow to its city); botnet 1 compute and no seeding. Adversary cubes go only onto its research track;
  megaprojects take neutral and player cubes only. **Research track:** a row of six icon slots on every adversary
  board, a slot's icon happening once, in each country, as the marker arrives; slot 7 is a loss. Boards differ by
  their row, their event mix and one rule; the map shows which threat is growing. **Rows are data**, content not
  code; the Militarist's row now holds Garrison and Strike icons (294, by editing the row). The adversary's
  Grow need not be kept: escalate and the one-output rule may replace or reshape it wherever that makes its turn
  simpler (its line left the event cards at r140; the players' Growth is untouched).
- **Matt, 2026-11-19:** the research track advances every K cubes, start about 3 a country, cubes from round 1
  (overturns 2026-11-17's threshold of 5, 5, 5, 4, 4; the start is the building item's to propose: 0 since 293).
  **K is 2 a country** (on 293's read; overturns 293's K 1), and the row is fed so that
  it reaches its higher, scary slots when players don't deal with it: threats, projects, Discord cards and the like
  open more adversary research stations or add research cubes, each addition kept close to an existing line.
- **Threat vibe (Matt, 2026-11-12; design doc *Adversary system*):** botnets spread quickly but manageable; agents
  on par with ambassadors; troops fewer, deadlier, destroy players left unchecked; early troops fine; players
  protect themselves by an expensive standard action, a different troop power on each board, and cards.
- **Troops garrison (Matt, 2026-11-19):** troops garrison where placed (a corrupted factory, else a city); a
  corrupted site can't be liberated with a troop on or next to it; they Strike on an icon shown a round ahead (the
  start: another timing that reads or plays better may be tried and said) and rarely Redeploy toward the nearest
  corrupted city. They come from the research row, projects and Discord cards: late and significant, something
  players see coming and must plan and invest to meet. The march, Muster and the corrupted factory's troop are gone;
  the Protector's Wall and Vigil are redone for troops that garrison and strike. It overturns "early troops fine"
  above and makes 2026-11-10's "troops ignore ambassadors" moot (no march to stop; ambassadors still stop no troop).
- **Harm coverage (Matt, 2026-11-13; a design principle, not a checklist):** threats, Discord cards and adversary
  projects are where adversity lives: a kind of harm belongs wherever it fits, at the tier it fits (corrupting or
  neutralizing a city reads as tier 2); events and the adversary board may lean. **Fallout goes on all of them**,
  its density tuned until it matters (227). Threats stay specific (one consequence); projects are bundles sized by
  cost (cheap one effect, mid a main effect + a side harm, only the most expensive a third; **soft**, Matt
  2026-11-18: a single harm is fine at any cost if its compute is well calibrated; no two projects share a line),
  each effect lands on every board (Matt, 2026-11-13 night: no single-country effects, as Spirit Island; a costly
  project may add one local effect on top); Discord cards keep the fear-card shape.

## Unity, Discord and winning

- **Winning (Matt, 2026-11-19; overturns 156's stepped purge):** the win is no corrupted city. Purge steps are
  gone: botnets, agents and troops never count. An early d0 win by coordinated players is fine, though not easy.
  Card tiers count the current tip.
- **Unity and Discord cards (Matt, 2026-11-09; direction locked):** Spirit Island fear-card shape: board conditions,
  "each player", one verb per card, tiers widening it; Unity prohibitions switch an adversary step off where a
  condition holds. Unity cards help, they don't clear: at most one tier-III card clears cities (2026-11-12).
- **Unity never cancels Discord (Matt, 2026-11-19):** Unity gained always goes onto the Unity track.

## Rulebook and audits

- **Rulebook rulings (Matt, 2026-11-10/13):** the turn's beats (as r130 prints them; two since r141: Card, Income). No last-node safety rule ever
  (tools instead). What's left of 224's list: [rulebook-friction.md](rulebook-friction.md). **Rejected:** a single
  keep price; used-markers (powers are implicitly once a round); purge back to 6. **Print pass components:** one tip
  track per side, agent-count glyphs on project faces, event icon rows, an ambassador box on the aid, the harm box's
  "Target: this country's player" line, the Discord track's tip procedure.
- **Rules audits (Matt and planner, 2026-11-17):** read [rulebook-friction.md](rulebook-friction.md) (*What a
  per-rule review misses*: combined load, generated tables read as data, stale reviews, one-more-dial balance fixes,
  sim players resolving everything free) before any rules or friction audit; "a dial is a rule"; `design_check`'s
  `ladder` and `table_cost` checks watch it.

## Design principles

- **Design principles (Matt, 2026-11-12; [principles.md](principles.md) *Design warnings*, checked by
  `scripts/design_check.py`):** capstone and tier-2 resources scarce and earned; no power or effect on every board;
  standard actions poor hedges; Unity and Discord follow the story (extreme acts raise Discord); fewer tokens and
  less tracking; engines over dials; games aim at 7-9 rounds (Matt, 2026-11-19). Warnings never gate; they go to Matt first.

## Table time and the card economy

- **Table time (Matt, 2026-11-18):** the ~2 h target is **relaxed: games aim at 7-9 rounds (Matt, 2026-11-19), and the sim's minutes
  are assumed wrong until Matt plays it himself and corrects them.** Yardstick: 278's `breakdown.py` at 268's rates
  (15 s a decision) plus the sense that similar strategy and play quality with less bookkeeping, token maintenance,
  math and tracking is better. Complexity slightly (not significantly) above medium Spirit Island. Hard
  (strategy-changing) cuts are discussed with Matt first; more dramatic cuts are revisited after his playthrough.
- **Card plays (Matt, 2026-11-18):** **never a card-play limit and no turn budget:** too many non-card turns means things are too
  cheap. Plays are limited TM-style by costly draw; everyone gets the 4-card draft; repeatable draw engines end the
  Science, Cyber and Diplomacy chains, priced like AI Central (2 a round is very strong); on a card draw 1 is good,
  2 very strong, 3 exceptional. Discounts about TM's count (a card may be free; free cards becoming a problem means
  too-cheap cards and too much discount). Smaller numbers where possible.
- **Passing in the draft is locked and non-negotiable (Matt, 2026-11-18):** players may turn it off; sims always draft with passing
  except as ablations; drafting players are assumed fast and skilled.

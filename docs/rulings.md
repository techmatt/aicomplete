# Standing rulings

Matt's standing rulings, current as the planner holds them; conductor's preamble names this page among the locked
rulings. A prompt, amendment or later ruling outranks it; an item given a new standing ruling folds it in here,
replacing what it overturns (history is in git and the write-ups). Current numbers and names: [rules/](rules/README.md);
how a rules, tuning or pricing item reads and reports: [tuning-items.md](tuning-items.md). "The design doc" is the
planner's, outside this repo.

## Names and terms

- **Names:** players' pieces are **ambassadors**, the adversary's **agents** and **troops**; a player's token is a
  **Node**, the adversary's a **Botnet**; the Security token prints **Operatives**; simulator players are **sim
  players**. Piece names on cards print capitalized or as the piece's symbol; rulebook prose uses plain lowercase.
- **Terms (locked):** "corrupt" is the adversary's one verb for taking sites; the land scar is **Fallout**; node loss
  is "destroy your node".

## Difficulty

- **Difficulty:** the ladder raises intensity, not count (extra threats only at d4). Bands: d0 at least 90, d1
  80-95, d2 75-90, d3 55-70 (no ceiling), **soft until development is further along:** misses are reported, never
  chased or dialed. Raise player power through player-board levers, not cheaper cards; never price anything as a
  share of income; no `stage_extra`.
- At higher difficulty the adversary must spread, corrupting more cities before the players can liberate them (an
  adversary cleared before it takes new cities is bad design).

## Simulation and sim players

- **Scale:** 2 players, pairs B+P, B+R, P+R, 48 seeds a cell, d0-d3 (d4 read too), paired against the previous
  binary on the same seeds; a band check pools two independent sets; dev screens at 32 seeds. A rules item is about
  3-4 hours.
- **Tiers:** greedy = new player; strategist = the judge; planner = spot checks. Fixes are sentence-sized
  heuristics; sim players don't shape the design.
- **The judge:** greedy's weights plus a card in hand's worth (`w.hand`, `w.hand_cost`), `w.purge`, a last-node guard
  and `w.card_action` (a building's earnings beyond income, its Action's share included, for 3 rounds; numbers:
  `STRATEGIST`), **no
  strategy** (`strategist:strategy=1` is an information cell). A better judge is wanted, a good player but not
  oracle-scale, and the bands are a heuristic re-read on it; win rate on fresh seeds decides; oracle agreement
  (`aic-sim agree`) guides tuning only against an oracle not built on the judge.
- **The oracles** (`scripts/bench/skilled_rows.py`): the full oracle (its keeps from the engine's keep plan, never
  its search) proves a seed winnable; short of 95 is a direction for buffs, not a blocker. The real oracle
  (`oracle:worlds=K`, sampled hidden information) at 32 lines a round on the judge's eval reads at the judge's level;
  the full oracle's edge is mostly search effort (lines a round, to 128) and eval, restarts about 10 points (314, 316). The fair-information yardstick is
  the real oracle's search on the engine's eval with 128 lines a round (316: 89% at d4 on fresh seeds, +22 over
  the judge's eval at 128 lines; 256 lines no better), `skilled_rows.py`'s `real` row (Matt, amendment 14593), and
  the `real` alias since 317 (Matt, Nov 22: one player; its 89 ± 5% at d4 meets the soft 90% ceiling).
- **Remote workers:** speed over bit-exactness unless the gain is trivial; paired reads pair same-engine seeds.

## Measures

- Same seeds, paired; credits read after the last completed action phase; leftover resources and their sinks
  (soft). **Every resource should be an interesting thing to spend:** each rules item reports how each resource is
  spent and whether it binds.
- **Watch list:** insight binding at keep and play; Patience carry-over; neutral claim timing; Vigil; Fallout's
  Discord drip. **Harms ledger:** [harms.md](harms.md), complete (no insight row; "costs raised" paged), kept current
  by every item touching threats, cards or the adversary.

## Resources and engines

- **Board facts (candidates):** slots 5 / 4 / 4; boards print 10 / 9 / 7 credits and +1 energy (Protector 2); reach
  1; converters need a node in a city of their country.
- **City energy:** each city neither corrupted nor blocked pays 1 energy to every player with a node on or next to
  it, on the energy income track; no hosts, no buying. **Resources are not merged.**
- **Insight:** this round's attention, spent this turn; the card-play track prints absolute values (starts at 3,
  then early-heavy steps); keeps cost 1
  insight tier-1, **3 tier-2** (2 upgraded); Patience keeps unspent insight (caps banned); no adversary card takes
  insight; the opening deals 2 tier-1 + 2 tier-2 free; tier-1 play costs are credits (4 a point). Aims (design doc
  *Economy*, directional): about 6 a round to drafting and 2-4 to plays; more tier 2 earlier; keeps are a plan;
  insight must bind, judged by slack beside bound rounds, no target. **Draft hand exactly 4 cards (locked).**
- **Chips:** worth 4 credits as a pricing anchor, not an exchange: their own resource, not cut, never trivially
  exchanged for credits without an engine. New neutral sites come from site-opening lines (no city growth).
- **Engines over dials:** chips, energy, antimatter and research cubes feed engines; when players skip one, buff its
  engines until skipping it is clearly bad play; easy levels are met by stronger engines, not adversary patches. A
  claim makes 1 chip a round; megacorps convert chips once their project completes. Strengthen Tidal Grid's
  converters (chip converter "2 chips → 3 energy") and the other unused converters.
- **Antimatter and majors (design doc *Economy*, *Cards and draft*):** antimatter is epic, anchor 18. Every board
  keeps a source, each a different decision: board powers are rare burns of your own engine; the steady source is
  tier-2 **converter plants**, a different resource each, no free producers; Make antimatter (5 cubes) stays the
  desperate hedge. **Tier-2 cards are the majors:** big payoffs funnelling resources into own or others' board
  actions, enough to win, many spending antimatter; winnable without them at d0 and under the oracle; the draft price
  is their only acquisition cost. Antimatter is powerful when it works, something you work toward; not every player
  need get it working, but at high difficulty probably some do; other things must also win, so not every strong
  end-game engine is antimatter-focused (sinks start at 2.0-2.5×, scaled back if overwhelming).

## Pricing and standard actions

- **Pricing:** at the unit anchors (fair 1.0-1.5×; at most ~1.5× tier 1, ~2.5× tier 2); never lowered to keep a
  band; unplayed cards get stronger, not cheaper; when `values.json` disagrees, fix the table. No holding or
  production limits: a runaway pile is fixed at card level. Anchors and worked checks:
  [card-guidelines.md](card-guidelines.md).
- **Standard actions:** poorly priced hedges, never the better route; none removes corruption except Lift (about 2
  research cubes) and Liberate. Liberate is the act of your ready ambassador on the site (move in, then act; its
  price set by calibration), needs the site free of agents and of troops on or next to it, and is the win route. No
  Claim standard action (cards claim). Sell a card is exempt from job-no-card: no card needs to sell.

## Cards

- **Card and board rules (design doc):** programs one-shot, buildings ongoing; tier is efficiency, not magnitude;
  useful alone; removal in reach; exchanges one use; no frictionless transfers; gifts say "another player"; no "up
  to N a round"; no paid Move; a sold card gives 1 credit. Fallout also comes off as a side bonus on green-feeling
  cards, never a bare removal card or a tier-2 multi-clear. Convert and engine-use cards (no anchor for their unit)
  are priced in band under both readings. Checklist: [card-guidelines.md](card-guidelines.md).
- **"Free" means one thing (Matt, Nov 21):** a card that frees (cleanses) a corrupted station or factory takes its
  agents off as Liberate does (D136); a card that removes troops before its rescue (Siegebreaker Reactor,
  Peacekeeping Mission) offers only cities its removal clears (D137).
- **315's picks (Matt, Nov 21):** Great Liberation is the fair multi-site capstone, exempt from the antimatter sinks'
  2.0-2.5 band; a "Target: Agents or Troops" power reaching a hex through a troop next to your ambassador also
  removes that hex's agents (printed as is until Matt's playthrough).
- **Finale buildings (Matt, Nov 20):** not every capstone is a building, but repeatable tier-2 "finale" buildings,
  hard to get and needing building up to, are a natural home for late-game power: their once-a-round Action (24)
  spends a late-game resource (antimatter, or large amounts of energy and the like). Getting the resource and its
  spending pathway are each hard enough that together they are the late engine, built while fending off the early
  game; a free engine beside a plant is the intended payoff, not a risk to price away. Diversity of design is a soft
  goal.
- **Tags are flavor and expectations, not rules** (as MtG colors): no tag owns an effect; an effect needs only a
  story fit.
- **Card targets (locked):** a map card, board power or standard action names one target, a hex in reach (1, fixed);
  a header target box prints its condition; effects act on the target only unless printed "on or next to the
  target" (most tier 1 target-only; tier 2 sizes up with radius 1, never 2); never two targets; **Gain** = your
  supply, **Place** = onto the map; no card picks a country freely ("in the target's country"); Shelter is "your
  nodes and facilities in reach"; megaproject-cube cards keep any open megaproject.

## Boards and pieces

- **Nodes (design doc):** nothing places a node anywhere (next to your nodes, occasionally range 2; cards may
  target); node powers give their bonus by position.
- **Board shape:** ordinary node powers, one cheap and one or two costly ones that carry a benefit, usable in the
  same round, beside tiered sacrifice powers and at most a few "≥ N tags" threshold powers where hoarding is the
  point.
- **Moving pieces:** Gather and Push: up to N, distance 1, into or out of a hex in reach; ambassadors, agents and
  troops move, never botnets or markers. Liberate's bill: yours standing on the site pay first, then your supply,
  never another player's.
- **Ambassador actions:** each ambassador has one action a round: move (to a hex next to it, or to any of your
  nodes) or act; it stands ready and tips over when used. It arrives ready (to try: say how it played, and what
  arriving spent would change). No Dispatches, and no board gets more.

## Adversary and harms

- **Adversary (locked):** the threat card standard (one consequence, icons over text, one or two counters, no hidden
  harm, no tier 3); the round cap stays a plain loss.
- **Card-driven:** the adversary is card-driven with look-ahead and checks on arrival, never a board sweep.
- **Threats (locked):** a threat that takes effect harms the player of the country it was dealt in; any player with
  a node or ambassador on or next to one of its tokens may pay. **No other rule may depend on a player's home
  country.** Threat decks are built once at setup, with the mix the adversary's board names (its lean: the
  Militarist's stays Security-heavy), and dealing never rejects a card in a round: the dealt card is the next one,
  and one with no legal site takes one plain fallback.
- **War is cut:** no fronts, bases or megacorp armies and no War step; megacorp conflict lives only as content on
  Discord and event cards, each doing one clear thing.
- **Spine:** an event card is Infiltrate X plus escalate (each site of last round's type holding an agent takes 1
  more; **Escalate always includes factories**, whatever last round's type: Commandeered Plants is gone as a separate
  scan, Matt Nov 21; no "most agents" picks, threat location tokens included: each goes on the lowest-numbered
  site of its type not yet the adversary's, Matt Nov 21). **One output a corrupted site:** city 1 Discord; station 1 research cube; factory 1
  agent (made on itself, pushed next round along its zone's printed arrow to its city); botnet 1 compute and no
  seeding. Adversary cubes go only onto its research track; megaprojects take neutral and player cubes only.
  **Research track:** a row of six icon slots on every adversary board, a slot's icon happening once, in each
  country, as the marker arrives; slot 7 is a loss. Boards differ by their row, their event mix and at most one rule (the Militarist prints
  none since Commandeered Plants folded into Escalate; its threat-deck lean and difficulty line remain); the
  map shows which threat is growing. **Rows are data**, content not code (the Militarist's holds Garrison and Strike
  icons).
- **Research track pace:** it advances every K cubes, **K 3 a country at levels 0-1 and 2 from level 2**, cubes from round 1 (the start is the building item's to
  propose: 0 now); the row is fed so it
  reaches its higher, scary slots when players don't deal with it: threats, projects, Discord cards and the like
  open more adversary research stations or add research cubes, each addition kept close to an existing line. The
  K ladder (3 at levels 0-1, 2 from level 2) is Matt's ruling (Nov 21).
- **Threat vibe (design doc *Adversary system*):** botnets spread quickly but manageable; agents on par with
  ambassadors; troops fewer, deadlier, destroy players left unchecked; players protect themselves by an expensive
  standard action, a different troop power on each board, and cards.
- **Troops garrison** where placed (a corrupted factory, else a city); a corrupted site can't be liberated with a
  troop on or next to it; they Strike on an icon shown a round ahead (the start: another timing that reads or plays
  better may be tried and said) and rarely Redeploy toward the corrupted city nearest a player piece (each onto its
  own country's). They come from the research row, projects and Discord cards: late and significant, something
  players see coming and must plan and invest to meet. A card may name a corrupted factory or city for its troop
  (Martial Law, Standing Army II-III, War Plants), never another hex; every other troop keeps the rule. Early troops
  are allowed: one landing early is a challenge to meet, not prevented. Ambassadors stop no troop. The march, Muster and
  the corrupted factory's troop are gone; the Protector's Wall and Vigil are redone for troops that garrison and
  strike.
- **Pressure is priced, not ruled:** too early or too hot is priced higher, not given a rule (Martial Law's compute,
  36; every way to gain an ambassador). Offensive targets the lowest-numbered node hex holding an agent or a troop,
  as Night Raid reads. The Strike rider ("a Strike also sends home one player ambassador next to the troop") was
  kept only if it gave Strike work at d0-d1 without pushing wins out of band; it did not (298) and is deleted (304).
  Locked: no stage on the project deck (complexity); supply ambassadors stay both piece and currency (price them,
  don't split).
- **Harm coverage (a design principle, not a checklist):** threats, Discord cards and adversary projects are where
  adversity lives: a kind of harm belongs wherever it fits, at the tier it fits (corrupting or neutralizing a city
  reads as tier 2); events and the adversary board may lean. **Fallout goes on all of them**, its density tuned
  until it matters. Threats stay specific (one consequence); projects are bundles sized by cost (cheap one effect,
  mid a main effect + a side harm, only the most expensive a third; **soft**: a single harm is fine at any cost if
  its compute is well calibrated; no two projects share a line), each effect landing on every board (no
  single-country effects; a costly project may add one local effect on top); Discord cards keep the fear-card shape.

## Unity, Discord and winning

- **Winning:** the win is no corrupted city; botnets, agents and troops never count. Card tiers count the current
  tip.
- **The early-win bar:** a round-2 or round-3 win is rare. The levers are the liberation price
  (`board.liberate_at`) and the starting corrupted cities (`board.start_corrupted`); a level-0 setup line may act at
  d0 alone (`adversary.setup_city_agents`). An early d0 win by coordinated players is still fine, just not easy or
  usual.
- **Game length:** most games run Spirit Island length, about 6-8 rounds; 10-11 is fine as an exceptional game; a
  spread is good, not one fixed length (`design_check.json`'s `rounds`: over half in 6-8, under ~10% at 10+).
  Running out the deck is a fair loss route, as Spirit Island's invader deck: the deck sets the cap at about round
  10-11, and cap losses are fine at any level (the bar guards against long, stalled games); K and the ladder aren't
  distorted to avoid them.
- **Unity and Discord cards (direction locked):** Spirit Island fear-card shape: board conditions, "each player", one
  verb per card, tiers widening it; Unity prohibitions switch an adversary step off where a condition holds. Unity
  cards help, they don't clear: at most one tier-III card clears cities. Each Unity card prints a floor (every
  player gains 2 credits) when its condition counts none (Matt, Nov 21: friction row 5).
- **Unity never cancels Discord:** Unity gained always goes onto the Unity track. About 4-5 of the 23 Unity cards
  remove Discord, each by a board condition; Unity otherwise never does.

## Rulebook and audits

- **Rulebook:** the turn's beats (Card, Income). No last-node safety rule ever (tools instead). Open friction:
  [rulebook-friction.md](rulebook-friction.md). **Used markers are kept** (274's, on every once-a-round use: Matt
  Nov 21). **Rejected:** a single keep price. **Print pass components:** one tip track per side, agent-count glyphs on project faces,
  event icon rows, an ambassador box on the aid, the harm box's "Target: this country's player" line, the Discord
  track's tip procedure.
- **Rules audits:** read rulebook-friction.md's *What a per-rule review misses* before any rules or friction audit; "a dial is a rule"; `design_check`'s `ladder` and `table_cost` checks watch it.

## Design principles

- **Design principles ([principles.md](principles.md) *Design warnings*, checked by `scripts/design_check.py`):**
  capstone and tier-2 resources scarce and earned; no power or effect on every board; standard actions poor hedges;
  Unity and Discord follow the story (extreme acts raise Discord); fewer tokens and less tracking; engines over
  dials; games run Spirit Island length, most in 6-8 rounds. Warnings never gate; they go to Matt first.

## Table time and the card economy

- **Table time:** the ~2 h target is relaxed: most games run 6-8 rounds, and the sim's minutes are assumed wrong until Matt plays it himself and
  corrects them. Yardstick: `scripts/bench/breakdown.py` at 268's rates (15 s a decision) plus the sense that
  similar strategy and play quality with less bookkeeping, token maintenance, math and tracking is better.
  Complexity slightly (not significantly) above medium Spirit Island. Hard (strategy-changing) cuts are discussed
  with Matt first; more dramatic cuts are revisited after his playthrough.
- **Card plays:** **never a card-play limit and no turn budget:** too many non-card turns means things are too
  cheap. Plays are limited TM-style by costly draw; everyone gets the 4-card draft; repeatable draw engines end the
  Science, Cyber and Diplomacy chains, priced like AI Central (2 a round is very strong); on a card draw 1 is good, 2
  very strong, 3 exceptional. Discounts about TM's count (a card may be free; free cards becoming a problem means
  too-cheap cards and too much discount). Smaller numbers where possible.
- **Passing in the draft is locked and non-negotiable:** players may turn it off; sims always draft with passing
  except as ablations; drafting players are assumed fast and skilled.

# Rules as implemented

What the code implements, at the revision in `RULES_REVISION` (`crates/aic-core/src/lib.rs`):
the Militarist against Builder and Protector, 1-4 countries. The player cards are in
[cards.md](cards.md), priced in [card-pricing.md](card-pricing.md). The AI unique cards,
AI boards, threats, the Unity and Discord decks and the project deck are **placeholders**
pending redesign. Every rule is a **candidate** unless marked **locked**; which, and what
each revision changed and why, is in [rules-history.md](rules-history.md). What content can
print is in [ops.md](ops.md).

**Every number below is a candidate**: a starting calibration reasoned for an 8–10
round game (`docs/principles.md`), for simulation to adjust. Names in `code` are config
tunables (`<module>.<param>`), with their values under [Tunables](#tunables), from
[`configs/baseline.json`](../configs/baseline.json); card, threat, adversary-card,
megaproject and AI-board numbers are content in [`content/`](../content/). Where the
prompt left a gap, the choice made is marked **Default**; where the implementation
departs from the v0 ruleset's card text, **Deviation**. Both are collected at the end.
Design intent lives in [principles.md](principles.md); the engine in
[architecture.md](architecture.md). **Keep this current with every rules change.**

<!-- generated: new_players -->
**For new players.** In the first round you are dealt 8 cards and keep 4 free (more at 5 credits each). From then on, each round you build a hand of 4 cards one at a time, choosing for each whether it comes from the cheaper tier-1 deck or the stronger tier-2 deck; you take one card and pass the rest to your neighbour until the hands are gone, then pay insight to keep what you drafted (1 for tier 1, 3 for tier 2). Insight is this round's attention: what you don't spend is lost. Playing a card costs its credits and 0-2 insight, from the same insight as keeps. Buildings go into your AI board's 3-4 slots; once they are full, a new building goes on top of an old one, which stops working but keeps its tags. Programs stay in front of you; events are used and discarded. Every card in front of you shows tags (Science, Industry, Cyber, Power, Diplomacy, Security): they unlock some cards, and once a round you can sacrifice cards with the right tags to use your AI board's power.
<!-- /generated -->

## Terms

- **Adjacent**: a neighbouring hex on the board, across country borders too.
- **Next to X**: holding X, or adjacent to a hex holding X.
- **Presence**: a player's tokens on the board, one per hex, taken off their AI
  board's presence track (§7). Presence is a token, not a building: it sits on any land
  hex, with or beside other pieces.
- **Reach**: a player's reach is every hex within 1 of their presence. Every map target
  of a standard action is in the acting player's reach, and so are most of a card's. The
  exceptions are a card's single ambassador move and Spread (measured from the ambassador), Total
  Containment's hex, and country effects such as a sweep or Shelter (any country; §8).
- **Site claim**: aligning a city, or claiming a research station or factory, needs the
  player's presence on or next to the site, and **a node next to the site blocks claims
  on it** (018: one hop from the site; 016–017 voided presence on or next to a node,
  two hops from the site).
- **Piece**: a city, a military base, a building (factory, research station) or a
  presence token. Ambassadors sit on pieces.
- **Clean / corrupted / aligned / owned**: a city, research station or factory with no
  owner cube is clean (a station or factory: neutral); an adversary cube means corrupted
  (a factory: adversary-owned); a player cube means aligned (a city) or owned.
- **The ladder**: the adversary's pieces, weakest first: ambassadors (the explorers, used
  up when they act), **nodes** (the towns, persistent), troops, corrupted cities and
  stations (the cities). Freeing a city knocks it down the ladder, not off it (§2).
- **Players choose**: an `Action::Choose` in `legal_actions`. Team choices in the
  adversary's steps, production and threats are asked of seat 0 (an aligned city's
  factory placement, of its owner); a choice inside a player's own card, ability or
  standard action is asked of that player.
- **Hex number**: each piece's hexes are printed 1–16 row by row; ties go to the lower
  number.

## 1. The board (module `board`, variant `corps`)

### Country pieces

A game has 1–4 countries, one player each. Country *i* is printed piece *i*
([`content/map.json`](../content/map.json)), a 4×4 piece of 16 hexes placed by the
layout for that country count. Population is implicit: people live in cities.

Hex kinds: **city**, **open** land and **ocean** are printed; a **military base** goes
on an open hex of each front (below). **Industrial zones** are a printed marking on open
hexes next to cities: factories go only there.
Research stations go on other open hexes, one building per hex. Nothing builds on ocean.

<!-- generated: countries -->
| # | Name | Cities (corrupted first) | Ocean | Station (node) | Industrial zones (setup factories) | Starting presence | Megaproject (benefit per owned station) |
|---|---|---|---|---|---|---|---|
| 0 | Coastal Republic | 7, 6, 14 | 3, 12 | 4 | 2, 8, 11, 13 (2, 8) | 9 | Tidal Grid, 7 cubes: 2 credits |
| 1 | Northern Union | 10, 7, 15 | 4 | 12 | 3, 14, 9, 16 (3, 14) | 6 | Polar Compute Array, 7 cubes: 2 credits |
| 2 | Highland Federation | 7, 2, 10 | — | 14 | 1, 6, 9, 15 (1, 6) | 11 | Mountain Fusion Works, 7 cubes: 2 credits |
| 3 | Delta States | 11, 6, 14 | 1 | 12 | 7, 13, 3, 8 (7, 13) | 5 | Delta Seawall, 7 cubes: 2 credits |
<!-- /generated -->

Ocean hexes are on the board's outer edge in every layout the piece appears in; the
starting presence touches a city.

**Fronts are printed** (016, Matt): a front is a long edge shared by two country
pieces, marked on the tiles; pieces that only touch at the middle of the map are not
neighbours. Each layout lists its fronts (`fronts` in `content/map.json`): 2 countries,
one front; 3 countries (top, right, bottom-left), all three pairs, so every country has
two; 4 countries (top, left, right, bottom), a ring: top–left, left–bottom,
bottom–right, right–top (top–bottom and left–right touch only at the centre). Every
country has exactly two fronts at 3 and 4 countries (checked by a test). Each country
gets one military base per front, on an open hex that touches the other country's land
and holds no setup piece or industrial zone; a country's fronts with the fewest
candidate hexes pick first, each taking its lowest-numbered free candidate (**Default**:
bases are placed by the layout, not printed, because a piece's neighbours change with
the country count).

### Setup

- Each country: its first `start_corrupted` cities get an adversary cube (difficulty
  4 with `adversary.extra_corrupted`, more); its research station starts
  corrupted, with a **node** on it; its two setup factories, **neutral**
  (**Default**: "a factory takes its city's owner" is for factories cities build; a setup
  factory touching the corrupted city would otherwise start adversary-owned).
- Each player (seat *s* starts in country *s*): 1 presence on the starting spot (the
  first token off their track), `start_ambassadors` player ambassador,
  `ai.start_credits` credits, and their AI board's credit production (§7).
- The adversary's supply: `adversary.supply_per_country` (024: was 14, 018: 10) ambassadors per country.
- The adversary deck is dealt and its top card revealed face up as round 1's card.

### Megaprojects

Each country's corp has one megaproject card (content, a placeholder per corp, in the table above;
reasoning, at 7 cubes: about 2 stations a country make 1–2 cubes a round, so a project
completes around round 4–5). In production each research station makes
`station_cubes` cube onto its country's card in its owner's colour: an owned
station the owner's, a corrupted station the adversary's; a neutral station makes
nothing. When the card is full it **completes**: each player cube pays its owner
`project_credits` credits; adversary cubes become Discord, 1 Sway per
`project_discord_per`. From then on each owned station in that country produces the
card's benefit for its owner every round.

## 2. Ambassadors

**An ambassador sits on a piece. The next time that piece does its normal thing, the
ambassador redirects it to its own side and is removed.** Adversary ambassadors return
to the adversary's supply; a player ambassador that is used up is gone (**Default**: a
player regains ambassadors with the standard action or cards).

| On | Adversary ambassador | Player ambassador |
|---|---|---|
| City | that round's energy is diverted (no Discord at `ambassador_discord` 0, below) | waits; a corrupted city frees the moment player ambassadors on it, from any players, reach its **hold** (below) (checked after every action and automatic phase; a card frees a city outright only if it is a tier-2 ultimate, §8) |
| Research station | that round's cube is the adversary's (owned or corrupted station) | on a neutral station the player can claim: the station becomes the player's (cube placed, ambassador used) |
| Factory | on a production step, an adversary troop on the factory's hex (**locked**); used up, on an adversary-owned factory too (028: it sat there forever) | on a neutral factory the player can claim: the factory becomes the player's; otherwise on a production step it is used up for `factory_unity` Unity and no troop is made, except that an adversary-owned factory still makes its adversary troop (022; it sat there forever) |
| Presence | if the presence is alone on its hex (020: on a city, base, station or factory the ambassador is that piece's), in production its owner gets 1 less insight next round and 1 compute goes onto the cheapest adversary project; used up (021: it leaked 1 compute and swayed Discord) | waits (defense, below) |
| Military base | on a production step, `base_discord` (027: was 3) Discord instead of a troop | on a production step, `base_unity` Unity instead of a troop |

- **Freeing takes a push (027, candidate; 024: `free_city_at` with no adversary ambassador
  adjacent).** A corrupted city's **hold** is `hold_base`, plus 1 for each adversary
  ambassador on or next to it, never below `hold_min`. When player ambassadors on it reach
  the hold, that many are used up (lower seats first), its cube returns to the supply of
  cubes, and the city **drops down the ladder rather than off the board**: `free_leaves`
  adversary ambassadors arrive on it as any placement does (027; freeing by a tier-2 ultimate
  too): a surplus player ambassador there cancels one, and the supply caps them. Ambassadors
  waiting there are cancelled by the next build as any others, so a hold is normally met
  in one action phase: gather ambassadors in your supply, clear around the city, land them.
- **Opposing ambassadors cancel.** When an ambassador arrives on a piece holding one of
  the other side, both are removed (the adversary's to its supply), with no Unity.
- **Defense:** a player ambassador on a building's hex is used up instead of the
  building when it would be destroyed.
- **Diverted** energy: a corrupted city's sways `divert_discord` Discord per unit; one
  diverted by an adversary ambassador on a clean or aligned city sways `ambassador_discord`
  (0; 027: was 1, so arrivals on cities alone ran the Discord deck out by round 6).
- A neutral station's normal thing is nothing, so an adversary ambassador waits there.

Player ambassadors move only through cards and the standard action.

## 3. The adversary (module `adversary`, variant `militarist`)

### Each round

1. **Build** (`adversary.build`): next round's face-up card becomes this round's card.
   **Arrivals (027, candidate):** each card names a site type, and every matching site in
   every country gains `arrivals` adversary ambassadors, whatever the board holds (below).
   Each corrupted city gains `city_build` (027: was 2, arrivals took over as the steady source; 024: was 1; 018: was 2) adversary ambassadors, and each factory
   next to (adjacent to) a corrupted city gains `factory_build` (018: was 1). The card's twists
   add more. A stage-II card's **Escalation** then gives each military base
   `escalation_ambassadors`. Ongoing **projects** act (below). Last, each **node**
   makes `node_compute` (027: was 3, arrivals make nodes quickly; 024: was 1) **adversary compute**, onto the projects; a node holding a
   player ambassador makes none (the ambassador stays).
2. **Spread** (`adversary.spread`): every hex holding several adversary ambassadors,
   fixed when the step starts, keeps one and sends each other one to a hex within
   `spread_range`. **Spread never targets an empty hex**: a destination holds a
   piece or is player-owned (presence, an aligned city, an owned station or factory);
   it is not ocean, and it holds no adversary ambassador unless it is a city that isn't
   corrupted, clean or aligned (which collects any number; 028: aligned cities too), or a
   piece without a node (below). **Priority:**
   player-owned hexes, then clean cities, then other pieces; then the nearest; players
   choose among equals. A city that is not corrupted never spreads: it holds its
   ambassadors. The presence of Aegis Dome Array's owner is skipped. **A surplus with nowhere to
   go returns to the supply.**
3. **Corrupt** (`adversary.corrupt`): a city that is not corrupted holding `corrupt_at`
   (6; 018: was 5) adversary ambassadors becomes corrupted (an aligned city loses its player cube);
   its ambassadors return to the supply; any player ambassadors waiting there are lost (024,
   any corruption; they would free it in the same step).
4. **Reveal** (`adversary.reveal`, after `threats.deal`): the top card goes face up as
   next round's card.

**Nodes.** A node is made when a spreading adversary ambassador lands on a piece that
already holds one (not a city that isn't corrupted, which collects them, and not a piece
that already has a node): both ambassadors return to the supply and a node goes on the piece. This
replaces 014's `overflow` candidate. Each board starts with a node (on its research
station). A node stays until removed: it makes `node_compute` adversary compute a round, it
**blocks site claims** next to it (a site with a node on or next to it can't be claimed),
and it counts against the purge until Unity eases it (§5). **Nodes are hard to remove
(019, candidate, Matt):** there is no standard action for it any more (016–018: a
standard action, `node_energy` 2–3 energy in reach). Only tier-2 effects remove a node:
the tier-2 cards **Node Severance** and **Singularity Exploit** (which also drains every
project), each removing a node
within your reach, and the tier-2 Unity card **Decommission** (remove a node
anywhere, players choose). Removing a node still sways `alignment.node` Unity (§5).

### The project deck (017; near-locked)

Adversary compute is spent on **projects**, one shared deck for every adversary
([`content/projects.json`](../content/projects.json), placeholders). The deck is
staged like the threat deck: `project_cards` are dealt, the cheap stage-1
projects shuffled on top of the big stage-2 ones. `project_row` projects are face
up at once; when all of them have resolved (or been discarded), the next row is dealt.

- **Compute sources:** each node, `node_compute` a round in the build (none under a player
  ambassador); about half the threats carry compute, gained when they execute
  uncountered (§8); and an adversary ambassador draining a lone presence adds 1 in
  production (§2).
- **Spending:** each compute point goes onto the **cheapest** visible project (ties: the
  first in the row). When a project holds its cost it **resolves**: a one-shot effect at
  once, or an **ongoing** one that acts in every round's build step from the next round
  on. Choices
  are the players' (seat 0; a sacrifice, each player's own). With no project left,
  compute is lost.
- **Cost** is printed per player (**Default**: the prompt's 12–15 and 25–30 compute are
  3-player numbers; a per-player cost keeps the timing at 2 and 4 players). The table prints
  each cost; 024 cut them from 6–7 and 10–12. 017 raised the cheap ones
  from 4–5 (12–15 at 3 players), which landed the first project in round 2–3 of the
  017 screen, toward the target round 3–4
  (`experiments/results/screen-017-projects/projects.md`).
- **Counterplay:** a player ambassador on a node stops its compute and is not used up;
  Cyber's hacks drain compute from one project (Flash Exploit, Compute Siphon, ...),
  tier-2 hacks from every visible one (Cascade Failure) or discard one with its compute
  (Killswitch Directive); the drain standard project (§6, 024) at credits.

<!-- generated: projects -->
| Project | Stage | Compute per player | Resolves | Text |
|---|---|---|---|---|
| Disinformation | 1 | 5 | once | Sway 4 toward Discord. |
| Node Swarm | 1 | 5 | once | Every node produces 2 adversary ambassadors on its hex. |
| Market Crash | 1 | 5 | once | Every player loses 8 credits. |
| False Flag | 1 | 6 | once | Deal three extra threats. |
| Infiltration Drive | 1 | 6 | once | In each country, the city that is not corrupted (clean or aligned) with the most adversary ambassadors becomes corrupted. |
| Rapid Buildout | 1 | 6 | once | The adversary builds now, then spreads. |
| Purge the Unaligned | 2 | 8 | once | Every player sacrifices an ambassador or a presence; in each country, an adversary troop lands on a factory (with none, a city). |
| Recruitment Drive | 2 | 9 | ongoing | Ongoing: every round, an extra adversary ambassador at every clean city. |
| Propaganda Network | 2 | 9 | ongoing | Ongoing: every round, sway 2 toward Discord. |
| Total Offensive | 2 | 9 | once | The adversary builds and spreads now; sway 2 toward Discord. |
| Armoured Columns | 2 | 9 | once | In each country, 2 adversary troops land on factories (with none, cities). |
| Station Coup | 2 | 10 | once | In each country, a research station that is not corrupted becomes corrupted; every node produces an adversary ambassador on its hex. |
<!-- /generated -->

**Defaults:** compute goes on one point at a time, so a full project resolves before the
next point lands; a sacrifice is any one of the player's ambassadors (supply or a hex)
or presence tokens, their pick, never their last presence; a sacrificed presence leaves the
game, as a destroyed one does (§4). Ongoing projects
act in the round's build step only, not in an out-of-turn build (Offensive, Rapid
Buildout, Mass production).

**Calibration (to r11; 024 doubled the build, below).** Since 018 (`city_build` 1, `factory_build` 0) a corrupted city made
1 ambassador a round, plus the card's twists. Each producing piece uses one up a round,
players remove some, and the spread prefers player pieces. Measured at r11 (greedy, 40
games, 2 countries, difficulty 0): 24 games saw a new corruption, the first in round 5.9
on average (all in rounds 5–7); 16 saw none. The first adversary project resolves in
round 3.7. (014's build rate, 3 a round from a city and its factory, no longer holds;
its first corruption in rounds 5–6 still does.)

### The deck (the clock and the stager)

[`content/adversary.json`](../content/adversary.json) prints the cards below.
A game deals `stage_cards` (034: was 4, 4, 3; 018: 3, 4, 3) at random per stage, each
stage shuffled on its own, stage I on top: as many rounds as cards, **the round cap** (034, **locked** that a
cap exists and can shorten; the number a candidate). **11 rounds through d2 (041):** difficulty 2
swaps a stage-I card for a stage-III card (`stage1_removed`, `stage3_added`), so the cap bites
through a harder end, not a shorter game; difficulty 3 removes a stage-II card (`stage2_removed`):
10. Some adversaries may shorten it too. The next round's card is always
face up a round ahead (peek cards show more). Each card's twists change its round.

<!-- generated: round_cap -->
| Difficulty | Stage I cards | Stage II | Stage III | Stage II from round | Round cap |
|---|---|---|---|---|---|
| 0 | 4 | 5 | 2 | 5 | 11 |
| 1 | 4 | 5 | 2 | 5 | 11 |
| 2 | 3 | 5 | 3 | 4 | 11 |
| 3 | 3 | 4 | 3 | 4 | 10 |
| 4 | 3 | 4 | 3 | 4 | 10 |
<!-- /generated -->

<!-- generated: adversary -->
| Card | Stage | Twist | Escalation |
|---|---|---|---|
| Probing Strikes | 1 | Arrivals: coastal cities. The Militarist tests the lines: it builds and spreads as usual. |  |
| Factory Drive | 1 | Arrivals: industrial zones. Factories next to clean cities build 1 too. |  |
| Long Patrols | 1 | Arrivals: research stations. Spread reaches 1 hex further. |  |
| War Drums | 1 | Arrivals: military bases. Sway 1 toward Discord. |  |
| Mobilisation | 2 | Arrivals: coastal cities. Each corrupted city builds 1 more. Escalation: each military base gains an adversary ambassador when this round builds. | yes |
| Station Raids | 2 | Arrivals: research stations. Each corrupted research station builds 1. Escalation: each military base gains an adversary ambassador when this round builds. | yes |
| Fifth Column | 2 | Arrivals: coastal cities. Cities corrupt at 1 fewer ambassador. Escalation: each military base gains an adversary ambassador when this round builds. | yes |
| Border War | 2 | Arrivals: military bases. This round is a war round. Escalation: each military base gains an adversary ambassador when this round builds; each player destroys a presence of theirs. | yes |
| Supply Lines | 2 | Arrivals: industrial zones. Factories next to clean cities build 1 too. Escalation: each military base gains an adversary ambassador when this round builds. | yes |
| Total Mobilisation | 3 | Arrivals: industrial zones. Each corrupted city builds 1 more, and factories next to clean cities build 1 too; sway 1 toward Discord; each player destroys 2 presence of theirs; 2 Sway leaves the Unity bucket. |  |
| Blitzkrieg | 3 | Arrivals: military bases. This round is a war round, and spread reaches 1 hex further; sway 1 toward Discord; each player destroys 2 presence of theirs; 2 Sway leaves the Unity bucket. |  |
| Occupation | 3 | Arrivals: research stations. Cities corrupt at 1 fewer ambassador, and each corrupted research station builds 1; sway 1 toward Discord; each player destroys 2 presence of theirs; 2 Sway leaves the Unity bucket. |  |
| Iron Doctrine | 3 | Arrivals: coastal cities. This round is a war round; sway 1 toward Discord; each player destroys 2 presence of theirs; 2 Sway leaves the Unity bucket. |  |
<!-- /generated -->

**Arrivals: the explore step that never stops (027).** The site types are **coastal
cities**, **industrial zones** (built on or not: an ambassador may stand on an empty zone,
the one exception to "ambassadors sit on pieces"), **research stations** (whoever owns
them) and **military bases**. A coastal city is one next to ocean or printed coastal on
its piece (`coast` in `content/map.json`): every piece has two coastal cities. Coastal Republic gets
both from its ocean, Northern Union prints one more, Highland and Delta print both. Next round's card is face up, so its site
type is visible a round ahead. Each stage's cards spread the types. **Default:** an
out-of-turn "the adversary builds now" brings the card's arrivals too, in its scope: one
rule for a build.

**The supply is a cap (027).** A placement the supply can't make doesn't happen; running out
no longer loses.

Players **lose** when a round starts with no card to turn face up (after the round cap's
last round): a quiet backstop nobody plans around.

### Difficulty (`difficulty`, 0–4; each level includes the ones below)

<!-- generated: difficulty -->
| Level | Adds | Tunable |
|---|---|---|
| 0 | Base rules |  |
| 1 | Extra threats each round in all, countries in turn | `extra_threats` 1, `counter_bonus` 0, `counter_bonus_all` 0 |
| 2 | A stage-I card swapped for a stage-III card: stage II (escalation, tier-2 threats) sooner and a harder end, at the same round cap (the table above) | `stage1_removed` 1, `stage3_added` 1 |
| 3 | Larger builds: each corrupted city gains more, and a stage-II card removed (a shorter round cap) | `build_bonus` 1, `stage2_removed` 1 |
| 4 | A still shorter Discord deck (027: was a smaller supply, which does nothing now it is a cap): the removed cards count as tips already made, so every printed step still acts (034); tier-2 threats sooner; optionally more corrupted cities per country at setup | `discord_removed` 1, `tier2_early` 1, `extra_corrupted` 0 |
<!-- /generated -->

(Difficulty 1's extra threats, 024: two since 018 was too many; at 4-8 credits a counter, two more threats took a whole round's income.)

Since 017 the difficulty is only the configured level: Discord tips no longer raise it
(§5). **`hold_base` is the same at every difficulty (034, locked):** difficulty comes from the
adversary and the decks: Discord deck size, threat strength and the round cap.

**The curve (041, r20, planner, 12 seeds a cell, %, ±14).** 2 countries: d0 100, d1 100, d2 83,
d3 42, d4 25; 3 countries: d0 83, d1 75, d2 50, d3 8, d4 17; at 32 seeds d2 is 88 / 44
(`experiments/results/screen-041-growth/growth.md`).

**The curve (034, r19, planner, 12 seeds a cell, %, ±14).** 2 countries: d0 100, d1 100, d2 67,
d3 17, d4 17; 3 countries: d0 100, d1 58, d2 42, d3 0, d4 0; at 32 seeds d2 is 66 / 38
(`experiments/results/screen-034-clocks/clocks.md`).

**The curve (028 quick screen, r17, 16 games a cell, engine / greedy, %).** 2 countries:
d0 50 / 12, d1 44 / 12, d2 44 / 12, d3 6 / 0, d4 19 / 6; 3 countries: d0 75 / 0, d1 56 / 0,
d2 12 / 0, d3 6 / 0, d4 0 / 0; random 0 everywhere. A hold of 3 is set for the planner (66 / 69
at d2, 32 seeds): one-step agents fall well short of "skilled play wins" at d0, and the curve
under the planner is not measured (`experiments/results/screen-028-rollout/rollout.md`).

**The curve (027 quick screen, r16, 16 games a cell, engine / greedy, %).** 2 countries:
d0 81 / 25, d1 38 / 25, d2 56 / 31, d3 56 / 12, d4 25 / 12; 3 countries: d0 88 / 56,
d1 69 / 44, d2 50 / 19, d3 19 / 0, d4 12 / 0; random 0 everywhere; with noise of about ±12
points, c2's d1 below d2 and d3 is within it. Engine win rounds run from 4 to 11 with no gate
(`experiments/results/screen-027-endgame/endgame.md`).

**The curve (024 quick screen, r15, 16 games a cell, engine / greedy, %).** 2 countries:
d0 94 / 62, d1 81 / 69, d2 69 / 31, d3 38 / 19, d4 38 / 25; 3 countries: d0 94 / 81,
d1 100 / 81, d2 81 / 75, d3 38 / 31, d4 19 / 6; random 0 everywhere; earliest win round 6
in every cell (`experiments/results/screen-024-adversary/adversary.md`). r14's lower numbers
were partly the agent's keep plan (fixed in 024), so the two curves aren't one game made
harder.

**The curve (021 quick screen, r12, 16 games a cell, engine / greedy, %).** 2 countries:
d0 100 / 88, d1 100 / 81, d2 94 / 88, d3 75 / 75, d4 19 / 19; 3 countries: d0 88 / 69,
d1 94 / 50, d2 81 / 31, d3 75 / 31, d4 31 / 6; random 0 everywhere
(`experiments/results/screen-021-cards/cards.md`, with r11's and the untuned r12's).
The skill gap between engine and greedy widened at 3 countries: greedy keeps every card
it drafts and plays few.

**The curve (019 quick screen, r10, 16 games a cell, engine / greedy, %).** 2 countries:
d0 100 / 88, d1 88 / 94, d2 81 / 94, d3 62 / 56, d4 25 / 38; 3 countries: d0 100 / 88,
d1 81 / 81, d2 88 / 69, d3 62 / 56, d4 69 / 50; random 0 everywhere. With 16 games a
cell the noise is about ±12 points, so c3 engine's d4 above d3 and greedy above engine
at c2 d1–d2 are within it. Levers: d1 two extra threats a round; d2 the 10-round deck;
d3 corrupted cities build 2; d4 the supply 4 smaller.
`experiments/results/screen-019-troops/troops.md` (018's r9 curve:
`experiments/results/screen-018-calibration/calibration.md`).

**Why this ladder (014; the supply loss it rests on was retired in 027).** At 014 the supply
loss bounded everything: one corrupted city made about 3 ambassadors a round per country
against a supply of 10, so anything that doubled the build (a second corrupted city) starved
the supply within two rounds. The
corrupted-city-per-country and smaller-supply levels the prompt suggested were first
tried as levels 3 and 4 (second city, supply 7 per country): every game ended in round 1
or 2, before a decision mattered. Levels 3 and 4 were rebuilt from the milder options;
the curve is not tuned (014 quick screen: greedy 100 / 88 / 25 / 0 / 0 % at d0–d3, 16
games each).

## 4. Factories, the clock, war and troops

### The production/war clock

A printed round track (`board.clock`) says what each round's production holds: 1 a
**production step**, 2 a **war step**, 3 both; the last entry repeats. Cards with the War twist (Border War, Blitzkrieg, Iron Doctrine)
make their round a war step too. **Default** (Matt confirmed): the clock gates only
factories, troops, chips and war; the income every round (cities' energy, the
presence track, stations, upkeep, the adversary troops' march) is unchanged.

### Factories

- **Ownership:** an owner cube, as stations and cities have. A factory a city builds:
  an aligned city's is that player's, a corrupted city's is the adversary's, and a clean
  city's goes to **the player whose presence is next to the zone the players picked**
  (018, Matt: the most presence on or next to it; players choose on a tie, as for city
  energy), neutral only if no presence is there (016–017: always neutral). A player ambassador claims a
  neutral factory as it claims a station (§2, needs a site claim). Cards that build a
  factory (Turnkey Factory, Prefab Plant Kit) build the player's own, on a free zone in
  reach. An owned factory keeps its owner if its city changes hands later.
- **Growth** (each production step, after troops and chips): every city builds one
  factory. A clean or aligned city builds on a free industrial zone next to it, where
  the players choose (an aligned city's owner, else seat 0). A **corrupted** city builds
  at a free industrial zone in its own country **next to an adversary ambassador if
  there is one, otherwise the lowest-numbered free zone** (018, Matt; lowest number
  among several next to one. 016–017's four-key order is gone: no player ambassador,
  then an adversary ambassador on it, then nearest, then number). A city with no such
  zone builds nothing. Factories destroyed in war
  regrow this way.
- **Troops** (each production step): each base makes 1 troop for its corp; every
  factory makes `factory_units` troop for its megacorp **at each of that corp's
  bases** (Matt, 016: at every front base, not just the nearest), so at 3 and 4
  countries a factory makes 2. An adversary-owned factory, or one holding an adversary
  ambassador (used up), makes an adversary troop on its hex instead (**locked**). A
  player ambassador on a factory is used up instead and sways `factory_unity` Unity; on
  an adversary-owned factory it is used up for the Unity and the troop is still made
  (022: it was never used up there).
- **Chips**: each player-owned factory makes `factory_chips`
  chip for its owner on each production step.

### Troops and war

**Corp troops.** There are no player troops. Troops belong to the megacorp of their
territory and wait at its bases.

- **Peacetime:** at most `unit_cap` (027: was 3, war took too much presence) troops wait at each base. A troop at a base
  **absorbs an adversary-troop strike** on a building on or next to the base: 1 troop
  per hit.
- **On a war step**, on each front the troops at the two bases cancel 1 for 1. The
  surplus crosses, acts at once and is used up: each destroys the rival corp's factory
  nearest its base (not an adversary-owned one; ties: lower hex number), **and any presence
  on its hex (027; before, a presence was the target only with no factory left)**: the presence goes
  with the factory, unabsorbed, and only the factory sways. All troops at the front are spent.

**Adversary troops (019, candidate): the march.** Adversary troops come from adversary
ambassadors on factories, from adversary-owned factories (production steps), and from
projects and threats (Armoured Columns, Orbital strike, Total war). They are the
adversary's mobile threat to player infrastructure. **Targets:** player-owned research stations and factories, **not presence** (`troop_presence`; 027: the
march destroyed 20 presence a game once a presence could end the game, and made the march
the main source; a troop on or next to a presence only stands there). Cities are never targets (corruption is
the adversary's way into cities), nor are neutral or adversary-owned buildings.

In the adversary phase (`board.march`, after the corruption step) **every adversary
troop takes `troop_steps` steps**, one troop at a time, in hex-number order fixed
when the march starts. A step is one of:

- **destroy** a target on or next to its hex (players choose which), or
- with none there, **move** one hex toward the nearest target. When several hexes are
  equally good steps (several targets equally near, or several ways toward one), the
  players choose: a choice among bad options, as with Spirit Island's blight.

So a troop moves twice, or moves then destroys, or destroys at once. **A troop that
strikes is used up** (018, `troop_spent`), whether the target is destroyed or the hit
absorbed, as an ambassador is used up when it acts. With no target on the board a troop
stays where it is. Troops cross borders and never enter ocean; "nearest" and "toward" count
the walk over land (017–018 counted hexes with ocean included, which stranded a troop
facing a bay in the board's edge). A troop
two hexes from a target hits it in the next march; three or four hexes away, in the
march after. **Players see each hit coming:** the view draws each troop's next target
(the nearest; the arrow is solid when the hit lands in the next march), and every troop
on the board was there through an action phase except those that arrive between the
action phase and the march (built in production, placed by an uncountered threat or by
Armoured Columns in the build): an adversary factory within two hexes of your station or factory
is a known danger. Cards remove troops (a troop costs `troop_health` of a
card's removal; an ambassador 1), swaying `alignment.troop` toward Unity.

**For new players.** Adversary troops are the adversary's raiders. They appear on
adversary factories (and from some threats and projects), and every round, in the
adversary's turn, each troop takes two steps. If a research station or factory you own is on or next to the troop's hex, it destroys
that piece and leaves the board; otherwise it walks one hex toward the nearest such piece. So
a troop two hexes from your piece hits it next round, and one three or four hexes away a
round later; the arrows on the board show where each troop is heading. Troops never
attack cities. To stop a hit, remove the troop with a card, keep a corp troop at a
military base next to the target, harden the target or stand an ambassador on it. Troops don't hit presence.

**This is the game's one movement rule** (an exception to "no movement rules"): spread
places ambassadors, it does not walk them. It replaces two 016–018 rules:

| Job | 016–018 rule | Now |
|---|---|---|
| Hit player pieces every round | the strike in place: each troop destroyed a building or presence on or next to it in production | the march's destroy step, in the adversary phase |
| Bring troops to targets that are not next to them | the Discord advance (017): each Discord tip moved every troop 1 hex, then it struck | the march's move steps, every round; **a Discord tip no longer moves troops** |
| Use troops up | `troop_spent` on either strike | the same, on the destroy step |
| Make Discord tips hurt more as they pile up | each tip's advance and strikes | the Discord cards' printed steps alone (§5) |
| Target neutral stations and factories | the strike took any building not adversary-owned or corrupted | nothing: only player-owned ones (019) |
| Pressure for corp troops at bases to matter in peace | absorbing strikes on adjacent buildings | the same: a troop waiting at a base absorbs a strike on or next to it |
| The off-switch for troop movement (§9) | the advance | `board.troop_steps` 0: troops never move or strike |

**Presence destruction (027, 034).** A player with no presence on the board loses the game for
everyone (§5). **It is a slide, not a cliff (034):** a seat loses tokens a few at a time from
the middle of the game on, from several sources. Uncountered threats (Eviction offers it as a
choice; Agitators, Orbital strike and Defection in tier 1 and Station seizure, Overrun, Fifth
column, Insurrection in tier 2 make each player here destroy one of theirs here; Manhunt and
Siege two), a city that becomes corrupted (presence on its hex and, by `corruption_reach`, next
to it), war that destroys a factory (presence on its hex and, by `war_reach`, next to it), stage-III
adversary cards (each player destroys two) and Border War (one), Discord cards, and Purge the
Unaligned. **Discord steps:** the tier-2 cards and the tier-1 card left for the last tip make each
player destroy presence of theirs at their printed steps (§5; the brief's 1, 2, 3; **Candidate**,
below). A hit is each player's own pick of which token, and it is absorbed as any destruction
is (below), so a token shielded by an ambassador is a good pick. **While this round's adversary
card is stage I, a player's last presence can't be destroyed** (candidate, **Early fragility**):
each player starts with 1, and a war in round 2 would otherwise end the game. Eviction goes
further: it takes a presence only from a player with 2 or more, at any stage. The **most
threatened hex** (Offensive's, on the board) is the presence hex with the most adversary
ambassadors and troops (ties: country, then lower hex number). Sacrifice never offers a last
presence. A city that corrupts takes the presence unabsorbed: corruption is not a destruction
shelters and shields answer.

**A lost presence (034, locked, as in Spirit Island).** A destroyed or sacrificed presence
**leaves the game**: it does not go back on its track, and what it uncovered stays uncovered
(028's re-cover rule is gone, with 018's destroyed pile). Each lost token is a board location you
can no longer act from, so getting all your presence out is always a tradeoff.

**Corruption markers (034, candidate; `presence_marker`).** A lost presence leaves a corruption
marker on its hex, unless the hex is a corrupted site, holds a node or already holds a marker
(the cap on the loop: a marker adds no corruption where there is some). A marked hex can't take
a presence. On a city that is not corrupted it counts 1 toward `corrupt_at`; since no presence
stands there, a marker never destroys presence itself. It doesn't count against the purge.
**Clearing one is worth an action (041, candidate):** the Clear standard action (`clear_energy`)
takes a marker off in reach and pays `clear_unity` Sway toward Unity and `clear_credits` credits;
Science cards clear them too, for the same reward. Markers come only from destroyed presence, so
the reward can't be farmed: a token lost costs far more than a Sway.
**Default:** the brief names the marker and its caps, not what it does; this is the smallest
reading that makes it matter.

**Destruction.** Only war sways Discord: a factory destroyed by the corp
troops crossing a front sways `destruction_discord` (027: was 1: three fronts ran 3-country Discord far ahead of 2-country) Discord. An adversary troop's
strike (in the march) sways nothing (018, Matt; 016–017 swayed for every destruction,
which with the advance made each Discord tip feed the next). A destruction is absorbed, in this order, by a
shelter on the country, a corp troop at a base next to it (adversary strikes
only), a hardening token on the hex, a player ambassador on the hex, or Aegis Dome
Array (once a round, for its owner's countries). A destroyed presence leaves the game (above). A destroyed station or factory takes its cube with it.

### The jobs of the old data centre

| Job | Taken over by |
|---|---|
| Compute production (1 per data centre) | nothing: players make no compute |
| Adjacency for building, placing and reach | presence: reach, site claims, placement next to your presence |
| City energy (the player with the most data centres next to a clean city) | the player with the most presence on or next to it |
| A target for adversary troops (strikes) and for war's surplus when a country has no factory | presence for war, on a destroyed factory's hex only (027); troops strike buildings only |
| A target for adversary ambassadors: one on it diverted its compute | an adversary ambassador on a lone presence costs its owner 1 insight next round and adds 1 compute to the cheapest adversary project (§2) |
| A spread priority (player-owned hex) | presence hexes are player-owned |
| Threats' "players with a data centre here" (who pays, who is hit) | players with presence there |
| Sabotage (data centres here make no compute) | players with presence in a sabotaged country make no energy in upkeep |
| Cards that placed or read data centres (Point Defense, Firewall, Civic shelter, AI growth) | the same text with presence |
| Loss or win hooks | none existed (a destroyed data centre only left the game) |

## 5. Alignment and the end of the game (module `alignment`)

Alignment (013) is where the AIs' world leans: toward **Unity** (the good AIs, the
players) or **Discord** (the evil AI, the adversary). Everything on it is printed: Sway
tokens on three spaces, two tip-count tracks and two small decks of one-shot cards.

**The track.** Sway sits in three places: **the Undecided** (the middle pool), the
**Unity bucket** and the **Discord bucket**. The Undecided starts with
`undecided_per_player` (027: was 3) × players Sway; both buckets start empty. **Candidate** (027): the pool is exactly two tips, as in
013–016; 017–026 ran 3 a player, so the two sides contested tokens.

**Swaying** toward a side moves Sway one at a time from the Undecided into that side's
bucket. With the Undecided empty, the variant decides:

| Variant | Sway with the Undecided empty |
|---|---|
| `tug` (default; **locked**, 017: empty-pool rule B) | comes from the opposite bucket (a tug of war); with that empty too, nothing |
| `still` | nothing happens |
| `off` | (for measuring Alignment, §9) no Sway ever moves and nothing tips |

**Tipping.** The Unity bucket holding `tip_per_player` × players Sway, or the Discord
bucket holding `discord_tip_per_player` (027) × players, **tips**, in this
order: all the bucket's Sway returns to the Undecided; that side's tip count goes up by
one; one card of its side is drawn and resolves (at the step printed for the tip count
before this tip, below). (017–018: a Discord tip then advanced the adversary troops;
019 replaced that with the march, §4.)
**Default** (017): the bucket empties before the card, so a card's own Sway counts
(013–016 resolved the card first and then emptied the bucket, wiping any Sway the card
added to its own side). Unity tips and Discord tips are counted
separately and only count up; there is no net meter. **Default:** buckets are checked
when an action or automatic phase finishes resolving (with win and loss, below), so a
bucket may overfill within one action. If both are full, Discord tips first; a tip's
card may fill a bucket again, which then tips too.

**Momentum** (`momentum_every`, 0 = off): a side's bucket holds 1 fewer
Sway for every `momentum_every` tips that side has made, down to 1 per player.

**Sources of Sway**

<!-- generated: sway_sources -->
| Toward | From | Amount |
|---|---|---|
| Unity | a player ambassador on a producing base / factory | `base_unity` 2 / `factory_unity` 1 |
| Unity | removing an adversary troop | `alignment.troop` 1 |
| Unity | freeing a corrupted city (§2; or a tier-2 ultimate) | `alignment.free_city` 5 |
| Unity | removing a node | `alignment.node` 2 |
| Unity | cards (Peace Dividend, Planetary Referendum, Martial Cordon), AI boards, Unity cards | card text |
| Discord | energy diverted (a corrupted city, an ambassador on a city) | `divert_discord` 1 per unit of a corrupted city's, `ambassador_discord` 0 of an ambassador's |
| Discord | an adversary ambassador on a producing base | `base_discord` 1 |
| Discord | a building destroyed in war (not by an adversary troop; 018) | `destruction_discord` 0 |
| Discord | adversary cubes on a completed megaproject | 1 per `project_discord_per` 2 |
| Discord | an uncountered threat, the antimatter action, twists, Discord cards, adversary projects | `threat_discord` 1, `antimatter_discord` 1, card text |
| Discord | the adversary at the start of every round | `adversary_discord` 0 |
<!-- /generated -->

`adversary_discord` is 0 because corrupted cities' diverted energy is now the
adversary's per-round drip (1 per corrupted city, already more than 013's 1 a round).

The round's Sway is the phase `alignment.sway`, first in the round: the adversary's
`adversary_discord`, then each AI board's `unity` (§7).

**More Unity (017).** Unity should tip in most games while Discord stays the main
pressure, so 017 adds Unity sources and cuts no Discord one: freeing a corrupted city
sways `alignment.free_city` (034: was 4), removing a node `alignment.node`, and a player ambassador on a producing
military base sways `base_unity` as since 013 (**locked**).

**The antimatter action.** In the action phase (`...+alignment.act`), any player may
sway `antimatter_discord` toward Discord to gain 1 antimatter, up to
`antimatter_uses` times a round each. `antimatter_discord` 0 removes the action, as
does variant `off`.

**The cards (one-shot and hidden, like fear cards: locked, 017).** Each side has a
small face-down deck in two tiers, each shuffled at setup
([`content/alignment.json`](../content/alignment.json)). A tip draws from tier 2 once
that side's tip count, before the tip, has reached `unity_ease[0]` / `discord_tier2`,
else from tier 1; from the other tier if that one is empty. The card resolves at once and
goes face up on its side's resolved pile; there are no ongoing cards (013's Volunteer
Corps and Propaganda are one-shot now). **Neither deck ever reshuffles (027): they are the
game's two clocks.** A Discord tip that needs a card and finds the deck empty loses the
game (**locked**, Matt); drawing the last Unity card wins it outright, and that card
doesn't act (**Default**, 027). **A game deals a small Unity deck (028, candidate):**
`unity_cards` of the printed Unity cards, tier 1 then tier 2, at random and face down, so
the last card can be reached (027: all 9, and games saw about 4 tips). Choices are asked of
seat 0. **Default:** "hidden like
fear cards" is read as a face-down deck resolved the moment it tips; Spirit Island holds
earned fear cards face down until a later phase, which would be a timing change.
Grassroots Funding, Think Tank and New Deal still give income "for the rest of the game":
a one-shot card that moves a printed income marker, not a card left in play.

**Steps (017).** A Discord card prints its strength by the Discord tip count (counted
before this tip, so "after 3 Discord tips" first fires on the 4th tip): it replaces the
difficulty escalation (below). Unity cards have no steps.

<!-- generated: alignment -->
| Card | Side | Tier | Text | Stronger after tips |
|---|---|---|---|---|
| Grassroots Funding | unity | 1 | Every player gains 2 credit income for the rest of the game. |  |
| Open Market | unity | 1 | The next 2 card purchases are free. |  |
| Think Tank | unity | 1 | Every player gains 1 insight each round for the rest of the game. |  |
| Ceasefire | unity | 1 | Counter one active threat. |  |
| Civil Defense | unity | 1 | In each country, remove 1 adversary ambassador. |  |
| Volunteer Corps | unity | 1 | Sway 3 toward Unity. |  |
| Global Coalition | unity | 2 | Counter every active threat. |  |
| New Deal | unity | 2 | Every player gains 2 credit income for the rest of the game and 4 credits now. |  |
| Decommission | unity | 2 | Remove a node anywhere on the board. |  |
| Incursion | discord | 1 | Deal one extra threat (after 5 Discord tips: three, and each player destroys 2 presence of theirs). | 5 |
| Blight | discord | 1 | In each country, the clean city with the most adversary ambassadors gains 1 more (after 5 Discord tips: 3, and each player destroys 2 presence of theirs). | 5 |
| Propaganda | discord | 1 | Every player loses 4 credits; 2 Sway leaves the Unity bucket (after 5 Discord tips: every player loses 8 credits and destroys 2 presence of theirs, and 2 Sway leaves the Unity bucket). | 5 |
| Offensive | discord | 2 | The adversary spreads now, then all presence on the board's most threatened hex is destroyed (after 3 Discord tips: also each player destroys 1 presence of theirs; after 4: it builds, then spreads, and each player destroys 2). | 3, 4 |
| Two Fronts | discord | 2 | Deal two extra threats (after 3 Discord tips: also each player destroys 1 presence of theirs; after 4: three, and each destroys 2). | 3, 4 |
| Firestorm | discord | 2 | In each country, the clean city with the most adversary ambassadors gains 2 more (after 3 Discord tips: also each player destroys 1 presence of theirs; after 4: 3, and each destroys 2). | 3, 4 |
<!-- /generated -->

The Unity cards are opposition's old rewards as one-shot cards; the Discord cards are
adversary effects the game already has. A threat countered by a card is discarded and
its partial payments are refunded. **Defaults:**

- Open Market's free purchases are keeps of drafted cards (§8), by any player, used
  before credits.
- **Unity pulled back (027):** "N Sway leaves the Unity bucket" returns up to N to the
  Undecided.
- Extra threats come from the current tier's deck (no reshuffle out of turn: an empty
  deck deals nothing), one per country in turn from country 0 (`shared_row`: into the
  row), and are countered or execute like any other. One dealt after the round's
  counter phase (a tip in production or upkeep, False Flag) waits beside the row and
  executes next round, after a counter phase (022: it executed the same round, with no
  chance to counter it).
- Blight and Firestorm: ties go to the lower hex number, so with no ambassadors on any
  clean city the lowest-numbered one gains them; a country with no clean city gains
  nothing. Civil Defense in a country with no adversary ambassador removes nothing.
- Offensive acts at once, out of the round's order, with this round's card's build or
  spread twists; its build has no Escalation. Before the build step
  (a tip in the Sway phase, with `adversary_discord` 1+) this round's card is the
  face-up one (022: that tip read last round's card, and none in round 1).

Remapped in 014: Civil Defense removed a drone; Blight and Firestorm added corruption
to the hex with the most adversary pieces; Buildup built with the build-slot card;
Offensive ravaged with the ravage-slot card.

**The purge (the win), eased by Unity.** The purge needs no corrupted cities, no nodes,
no adversary troops and no adversary ambassadors on the board. Unity eases it at each
`unity_ease` threshold. **There is no printed round gate (027: `purge_round` is gone):**
the depth of the steps and the hold (§2) set how early it can come.

<!-- generated: purge_ease -->
- first threshold (2 Unity tips): adversary ambassadors stop counting;
- second threshold (2 Unity tips): adversary troops stop counting;
- third threshold (4 Unity tips): **nodes** stop counting (019, candidate: with node removal rare, Unity buys the win);
- corrupted cities always count.
<!-- /generated -->

(027: `unity_ease` was 1, 1, 2; wins came in rounds 3-4 with the round gate gone.)

The thresholds are a list: the first is ambassadors, the second troops, the third
nodes; with only two entries nodes always count (as in 016–018). **Why (019):** without
the 2-energy node action, nodes blocked the purge in most games (019's first screen:
d0 engine 19% at 2 countries, 0% at 3, nodes and troops left on the board); a node
threshold at 3 Unity tips, or at 2 with troops at 2, still left 3-country d0 at 31–62%.
Troops stop counting early (with ambassadors) because they respawn from adversary factories
every production step and now walk away from where a card can reach them.
This replaces 014's purge hook (the board win counted only corrupted cities,
ambassadors and troops) and 013's bar easing (the same thresholds, now named as what
stops counting). The level-0 purge (below the first threshold) counts only once this round's
adversary card is of stage `clear_win_stage` or later; at 1 (the default) that is from
round 1's card on. From the first threshold the purge is not held back by the stage (as the
code has always done; 016's rewrite of this paragraph dropped the "level-0").

**No escalation (017).** 013's "difficulty in play +1 at 2 and 5 Discord tips" is
gone. Discord escalates through each tip's card and the
Discord cards' printed steps. What the old escalation did:

| Job of the old escalation | Taken over by |
|---|---|
| Level 1 in play: one extra threat a round | Incursion and Two Fronts, whose extra threats step up at 5 and 4 Discord tips; False Flag (project) |
| Level 1 in play: counter bonuses (0 in the baseline, so nothing) | nothing (the bonuses stay a setup level) |
| Level 3 in play: each corrupted city builds 1 more | Offensive (build and spread at 4+), Blight and Firestorm stepping up, Rapid Buildout and Recruitment Drive (projects) |
| Level 4 in play: tier-2 threats a round sooner | nothing; the adversary deck's stages still bring tier 2 |
| Making Discord tips hurt more as they pile up | the printed steps (017–018 also each tip's troop advance, gone with 019's march) |
| The stronger Discord tier from 2 Discord tips | kept: `discord_tier2` |
| The "difficulty in play" shown to players | gone: the view shows the configured level |

<!-- generated: game_end -->
**How the game ends, for new players.** You win by purging the board or by drawing the last Unity card. Pressure comes every round: the face-up adversary card names a kind of site, and every such site gains an adversary ambassador, so you always know where the next wave lands. Ambassadors pile up, pairs become nodes, 6 on a city corrupt it, and corrupted cities and uncountered threats push the Alignment track toward Discord. Each time the track tips, a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, you lose. Pushing back works the same way in your favour. Mass your ambassadors on a corrupted city to free it, which sways toward Unity, and each Unity tip brings a good card. The Unity deck is small too: a game deals only 6, and drawing the last wins at once. 2 Unity tips and the purge asks only for no nodes and no corrupted cities; 4 and it asks only for no corrupted cities. You also lose if any player has no presence left on the board. Threats, war, corruption and late adversary and Discord cards destroy it, a token at a time, and a lost token leaves the game and marks its hex, so you can't act from there any more; its income stays. In stage I your last one is safe. The adversary deck is a round cap too, 11 rounds at most, though games rarely reach it. So there are three different pressures: the Discord deck (politics), your presence (the board) and the cap.
<!-- /generated -->

**027's adaptations (each a candidate; the screen is in
`experiments/results/screen-027-endgame/endgame.md`).** Beyond the brief, simulation moved:
`hold_base` 2 (the brief's 3 leaves the engine at 31% / 25% at d2, 32 seeds; a one-action-at-a-time agent can't gather 5-6);
`unity_ease` 2, 2, 4; `discord_tip_per_player` 2 with its own parameter and
`undecided_per_player` 4; `city_build` 1 and `node_compute` 1 (arrivals make nodes fast: 17
a game at 3 compute, every project resolved by round 6); `base_discord` 1,
`ambassador_discord` 0 and `destruction_discord` 0 (Discord was at 8 tips by round 6);
`unit_cap` 2; `troop_presence` 4 and war hitting presence only through a destroyed factory;
the last-presence rule in stage I; three pieces print `coast`; difficulty 4 shortens the
Discord deck (`discord_removed` 1, was the supply). The Discord deck lost Buildup (7 to 6
cards) and every stage-III card sways 2 toward Discord: at 7 cards with no stage-III sway,
half the 2-country losses at d2 were deck-outs at round 11. Content: threats Eviction, Embargo,
Smear campaign and Manhunt are new; Siege and Total war's troops destroy presence; Purge
protocols strips tokens; Propaganda pulls Unity back; Offensive, and Firestorm at 4+,
destroy presence. **The printed steps moved (027):** with no reshuffle the draw order is
fixed, tier 1 at tips 0-1 and 5, tier 2 at tips 2-4, and a tip with 6 already made finds the deck
empty (a tip sooner at difficulty 4). So a step at 5 on a tier-2 card could never act; tier-1
cards step at 5, tier-2 at 4.

**028's adaptations (each a candidate; the screen is in
`experiments/results/screen-028-rollout/rollout.md`).** `hold_base` back to 3: the planner
gathers for it. `unity_cards` 3, 3: with `hold_base` 2 and a deck of 5 the planner won 94-100% at
d2. Tried and dropped: a limit of one bought presence a round (engine wins fell, no presence
losses) and dearer presence at 5 credits and 2 energy (one presence loss, and every win rate
fell). Teams fill both tracks by the late game, 9 tokens a player (027 said 5-6 a team): a hit
of 5 ended no game, a hit of 7 ends some. Eval: the Unity deck counts as a clock.

**034's adaptations (each a candidate; the screen is in
`experiments/results/screen-034-clocks/clocks.md`).** The round cap: the review's 12, 11 at d2
ended no game in 69 planner losses, because wins and the other clocks come first; 11, 10 at d2
(`stage_cards` 4, 5, 2) ends about a tenth of losses and keeps stage III late. The tracks: 13
tokens left too few on the tracks (0.3-1.1 a seat), so 14; growth and cards place about 70% of
tokens and skip the step extras, so the extras price only the last third. The last Discord step is
2, not the review's 3: at 3 Discord cards were 63% of presence lost. `corruption_reach` 1 makes
corruption a real source; `war_reach` 1 as well took the planner from 75 / 44 to 53 / 34 and is
off. Stage-III cards sway 1, not 2. `alignment.free_city` 5, not 4: c3 was at 28%, its wins late.
`energy_credits` 4. All tested at d2, 32 seeds, ±9-12 a cell.

**041's adaptations (each a candidate; the screen is in
`experiments/results/screen-041-growth/growth.md`).** The step costs: the first cut, with a third of
a track free, left too many tokens on the tracks (2.2 / 3.6 a seat) and won 44 / 6; the steps above
leave 1.2 / 1.9 and take 13 a seat by round 10. Stage III hits 2 and pulls 2 Sway back: the
ruling's "press harder", which moved presence's share of losses up, though no game ended at the cap.
`clear_unity` 1, not credits: at 1 credit back agents cleared 0.1 markers a game.

**Winning:** the purge, or drawing the last Unity card (027: it replaces `unity_win`, 8
tips).

**Losing:**

- a Discord tip finds the Discord deck empty (**locked**, 027);
- a player has no presence on the board: everyone loses, as when a spirit is destroyed
  (**locked**, 027);
- the adversary deck runs out (034: the **round cap**, a quiet backstop; §3).

(027 retired two losses: the supply running out, now a cap (§3), and every city corrupted.
Both fold into the Discord deck: a corrupted board drives Discord.)

Win and loss are checked whenever an action or automatic phase finishes resolving: full
buckets tip first (Discord before Unity), then losses, then the purge. A tip that ends the
game ends it there.

## 6. Resources and the round

| Resource | Comes from | Spent on |
|---|---|---|
| Credits | Credit production on the AI board (§7), the presence track, cards and Unity cards, in upkeep; megaprojects; selling cards and chips | Card costs; keeps beyond the opening's free ones (`opening_keep_credits` each); insight for keeps (`insight_credits` each); neutral cities' energy; standard projects; threat counters (024: every counter) |
| Energy (keeps between rounds) | Cities' cubes in production (§6 below); the presence tracks; Power buildings and the starting Sentinel Post; Power programs; growth | Placing presence (every placement pays its track step, §7); Power cards' `energy` cost; powers; hardening; standard actions; buildings at the Builder's rate, Security cards at the Protector's |
| Chips | Player-owned factories on production steps (`factory_chips` each); the Builder's second presence track; Industry cards (chip income, Bulk Silicon Procurement, Capital spent for chips) | Some buildings' `chips` cost; research stations (`station_chips`); sold at `chips_credits` |
| Antimatter | Each board's last presence-track space; tier-2 cards (Antiproton Offtake Deal, Positron Storage Ring); the antimatter action | Tier-2 cards |
| Insight | Each round, set to the AI board's insight (§7) plus the presence track's uncovers plus buildings; less 1 per adversary ambassador that drained your presence, and less what Smear campaign took | Keeping drafted cards; playing cards (022); giving (expires at the round's end) |
| Ambassadors | `start_ambassadors` at the start; Diplomacy cards (Consular Pouch, Embassy Compound); the standard action | Moving onto pieces; aligning cities |

**Cities' energy (021).** Each city makes `corp_energy` (a
megacorp parameter, more for a richer corp) energy cubes a round. An aligned city's go free
to the player who aligned it (+ `aligned_bonus`). A neutral city's can be bought for
`city_energy_credits` credits by a player
with presence on or next to it (players choose who, or nobody); otherwise they are lost. A
corrupted city's go to the adversary (Discord). A Blackout stops a country's cities in the
production after the threat executes (020).

**Discounts (AI-specific rates).** When a player plays a card, they may spend their AI's
resource for credits, at the rate its board prints, §7 (021: the Protector's was compute on
Cyber cards) (`discounts` in
[`content/ais.json`](../content/ais.json)). Players choose how much (at least enough to
afford it).

**Standard actions and projects** (`board.act`, in the action phase) exist but are
deliberately poorly priced: cards are always the better way, and they are a luck hedge.
The **standard projects** (024, candidate, Matt) are the credit sink, as in Terraforming
Mars: poor but useful rates, no limit a round.
Every map target is in the player's reach. The view publishes each action with its cost
(`View.standard`), so clients do not repeat the parameters.

<!-- generated: standard_actions -->
| Action | Cost | What |
|---|---|---|
| Place presence | `presence_energy` 1, plus the step's printed cost (§7) | `presence_action_cap` 1 a round (046): the next token off your track (uncovering its space) on a land hex next to your presence, with no corruption marker, 1 presence per hex (**locked**) |
| Build a research station | `build_energy` 2 + `station_chips` 2 chips | on an open hex in reach that is not an industrial zone; it is yours (your cube) |
| Remove an adversary ambassador | `remove_energy` 1 | in reach |
| Clear a corruption marker | `clear_energy` 1 | in reach (041): sway `clear_unity` 1 toward Unity, gain `clear_credits` 0 credits |
| Move a player ambassador | `move_energy` 1 | from your supply or the board onto a piece in reach (not the hex it comes from, 020) |
| Gain a player ambassador | `ambassador_credits` 6 | into your supply; a standard project too |
| Align a clean city | `align_energy` 2 + 1 ambassador | in reach, with a site claim: your cube on it |
| Harden a building (the Protector; a card can grant it) | `harden_energy` 1 energy a token | a player-owned station or factory in reach that is not corrupted, one a troop can strike (022: any building, the adversary's too) |
| Sell chips | 1 chip | gain `chips_credits` 1 (021: replaces buying chips) |
| Sell a card | gain `sell_credits` 1 | from your hand |
| The rich action (`presence_buy`, 034) | `presence_credits` 6 credits, + `presence_rise` 3 for each earlier use by you this round, + `energy_credits` 4 an energy of the step's printed cost | from stage 2 (`presence_stage`): as Place presence, no energy. Worse per token than Place presence, worth it only with spare credits (024: a cheap project from round 1) |
| Project: harden (`harden_buy`) | `harden_credits` 3 credits | any player: a hardening token on a player-owned station or factory in reach |
| Project: buy energy (`energy`) | `energy_credits` 4 credits | gain 1 energy |
| Project: drain (`drain`) | `drain_credits` 5 credits | remove up to `drain_amount` 3 compute from one visible adversary project |
<!-- /generated -->

There is no "build a factory" standard action: cities build factories.

### The round

| # | Phase (config `round`) | What happens |
|---|---|---|
| | *(round start)* | Insight is set; abilities and powers refresh |
| 1 | `alignment.sway` | AI boards sway |
| 2–4 | `adversary.build`, `.spread`, `.corrupt` | The adversary (§3): build, ongoing projects, node compute onto the projects; spread; corrupt |
| 5 | `board.march` | The adversary troops march (§4): `troop_steps` steps each, move or destroy |
| 6 | `threats.deal` | Threats face up on each country |
| 7 | `adversary.reveal` | Next round's card face up |
| 8–11 | `cards.tier`, `.deal`, `.draft`, `.keep` | Round 1: the opening deal and keep. Later: build the draft hand card by card (tier each), draft, keep (§8) |
| 12 | `ai.growth` | Each player picks a growth option |
| 13 | `cards.play+threats.counter+economy.give+alignment.act+board.act+ai.power` | Actions, simultaneous: cards, abilities, AI-board powers, standard actions, countering threats, giving, antimatter |
| 14 | `board.bases` | Production 1, on a production step: bases and factories make troops (ambassadors divert), factories make chips, cities grow factories |
| 15 | `board.war` | Production 2, on a war step: front battles |
| 16 | `board.produce` | Production 3: cities' energy cubes (neutral ones for sale) and stations (ambassadors divert); ambassadors on presence drain insight |
| 17 | `economy.upkeep` | Production 4: income; building upkeep; token cards gain their tokens |
| 18 | `threats.execute` | Production 5: partial payments refunded; uncountered threats execute |
| 19 | `board.cleanup` | Round flags removed; tips and the end checks run after every phase |

## 7. AI boards (module `ai`; [`content/ais.json`](../content/ais.json))

Each AI board has a printed **income**, a **presence track**, a
starting building, building slots (`ai.slots`, a config list in seat order) and one **power**. The first token starts on the board, so its space is
uncovered from setup; placing presence takes the next token off the track and uncovers
what is printed under it. Uncovered values add to the income every round (insight at the
start of the round, the rest in upkeep). The board module owns presence and counts the
tokens taken (`board.track.<seat>`); the AI board reads it.

<!-- generated: boards -->
| Board | Printed income | Starting building | Slots | Track tokens | Second track | Tokens in all |
|---|---|---|---|---|---|---|
| Builder | 10 credits, 4 insight | Foundry Core (Industry): +1 credit | 4 | 8 | 6 | 14 |
| Protector | 9 credits, 4 insight | Sentinel Post (Security): +1 energy | 3 | 8 | 6 | 14 |
<!-- /generated -->

**One payment path (041, candidate; replaces 034's extras).** Every space prints a **cost** in
energy, the last ones more. Every placement pays it: growth and cards pay the cost, the standard
action pays `presence_energy` on top, and the rich action pays its own price plus the cost at
`energy_credits` credits an energy. A card that prints "free" pays nothing (three tier-2 cards).
Only tracks whose next step you can pay are offered, and a placement you can't pay doesn't
happen: Expand with energy for one step places one.

**Growth chooses (041, candidate).** Each round each player picks one option: **Expand** (2
presence, a small gain), **Develop** (1 presence, draw a card, 046: for both boards) or
**Consolidate** (no presence, a solid bundle). Growth should stay something a printed game can grow
out of: nothing here stops one cheap energy coming from a city's power plant later.

**One standard placement a round (046, candidate; the 1 energy stays locked).** The Place presence
standard action can be taken `presence_action_cap` times a round by each player. Growth, cards and
the rich action don't count against it. Without the limit a seat that never grew still took 11-12
tokens by round 10 on its Consolidate energy, and "presence or resources" was a choice only for the
turn.

<!-- generated: presence_track -->
| Space | Builder | Protector |
|---|---|---|
| 1 (on the board at setup) | +1 insight | +1 insight |
| 2 | +2 credits | +1 energy |
| 3 | +1 credit, **keep upgrade** | +1 energy, **keep upgrade** |
| 4 | +1 insight | +1 insight |
| 5 | +2 credits (cost 1) | +2 credits (cost 1) |
| 6 | +1 energy (cost 2) | +1 credit (cost 2) |
| 7 | +1 credit (cost 2) | +1 energy (cost 2) |
| 8 | +1 antimatter (cost 3) | +1 antimatter (cost 3) |
<!-- /generated -->

The **keep upgrade**: tier-2 keeps cost `keep_tier2_upgraded` instead of `keep_tier2` (§8).

**021:** spaces 1-2 gave compute (gone with player compute); the tier-2 unlock on space 3
became the keep upgrade; the last space gives antimatter ("production unlocks only at the
end of a board's track").

**Second presence track (021, placeholder; 034: 6 tokens).** Each board has a second track
(`presence2`), none on the board at setup. Placing presence takes the next token from the
track the player chooses (asked when both have tokens). Real track contents and powers come
with the AI-board rework.

<!-- generated: presence_track2 -->
| Space | Builder | Protector |
|---|---|---|
| 1 | +1 credit | +1 energy |
| 2 | +1 chip | +1 credit |
| 3 | +1 insight (cost 1) | +1 insight (cost 1) |
| 4 | +1 credit (cost 2) | +1 credit (cost 2) |
| 5 | +1 chip (cost 3) | +1 energy (cost 3) |
| 6 | +1 credit (cost 4) | +1 credit (cost 4) |
<!-- /generated -->

**Powers (locked: paid by sacrificing tags).** A power's cost names tags (`any` for any
tag) and optional other resources. It is paid by discarding played cards that carry those
tags: a program from your play area, or a building **with its whole stack** beneath it;
events can't be sacrificed (they are gone once played). A card pays with all its tags,
each tag first toward its own requirement, then toward `any`; extra tags are wasted.
Each power is usable once a round. **Default:** what can be sacrificed is fixed when the
power is used; the player chooses the cards one at a time among those that still help.

<!-- generated: powers -->
| Board | Power | Cost | Effect |
|---|---|---|---|
| Builder | Retool | 2 Industry + 1 any | gain 5 credits and 2 chips |
| Protector | Lockdown | 2 Security + 1 any | remove 3 worth of adversary pieces in a hex in your reach; sway 1 toward Unity |
<!-- /generated -->

`ai.boards` (`[0, 1]`: Builder, Protector) gives each seat its board; the list repeats
for more seats, so 3 and 4 players repeat AIs. Each AI's three unique cards (those the
cards config lists; baseline: all) start in its player's hand, and its starting building
in a slot (**Default:** one copy each, so a repeated AI's second seat has neither).

<!-- generated: board_details -->
| Board | Innate | Discount | Growth options | Unique cards |
|---|---|---|---|---|
| Builder | *Economies of scale*: Each building you play costs 1 credit less. Energy pays for buildings, 1 energy a credit. | energy on buildings 1:1 | Expand: place 2 presence, gain 1 credit; Develop: place 1 presence, draw 1 card; Consolidate: gain 4 credits and 2 energy; next building this round costs 2 less | Turnkey Factory, Venture Capital, Modular Expansion |
| Protector | *Bulwark*: You may harden buildings (standard action, 1 energy a token). Energy pays for Security cards, 1 energy a credit. | energy on Security cards 1:1 | Expand: place 2 presence, gain 1 energy; Develop: place 1 presence, draw 1 card; Consolidate: gain 3 energy and 2 credits; harden 2 buildings in your reach, in one country | Point Defense, Strike Team, Shelter Order |
<!-- /generated -->

Hardening (the Protector's growth, the standard action, cards, Point Defense) goes only on
buildings a token can protect, player-owned stations and factories that are not corrupted, and
"harden N" is N different buildings, fewer if fewer qualify (022: any building, the same one
several times). A Science card's shield (041) is the one exception: a hardening token on a hex of
your presence, which absorbs a destruction there and goes with the token if it is lost another way.
On your own building a shield is the building's hardening token: a table has one token, so it stays
when the presence goes, and protects the building.

## 8. Threats and cards

### Threats (module `threats`; [`content/threats.json`](../content/threats.json))

- Variant **`countries`** (default): each threat sits on one country and acts there;
  only players with presence in that country may pay toward it, and "each player"
  in its text means each player with presence there.
- Variant **`shared_row`**: threats sit in one row, anyone may pay, and each acts on
  the whole board: an effect per country (Blackout, Sabotage, Overrun, Mass
  production) hits every country, "each player" is every player, and a placement
  (Infiltration's, Agitators', Fifth column's and Insurrection's ambassadors, Orbital
  strike's, Siege's and Total war's troops and ambassadors, Station seizure's station)
  is that many on the whole board, players
  choosing where. **Default:** the row is still dealt `per_country` threats per
  country, so with more countries there are more threats.
- Variant **`off`**: no threats (§9).

**Dealing.** In `threats.deal` each country gets `per_country` threat (plus
difficulty 1's extra, §3), face up: tier 1 until this round's adversary card is stage II
or later, then tier 2 (difficulty 4: a round sooner). Each tier's deck holds one copy of
each of its threats, shuffled at setup; its discards reshuffle when it runs out. Tier 2
holds enough threats (022: 5, short of the 6 a round dealt at 4 countries and difficulty
1+), so the round's deal never runs short; a deal the deck can't supply (the round's, or
extra threats out of turn) deals what it can and counts the rest as `threats.short`.

**Countering (Default).** A threat is countered only when fully paid. Payments are
pooled across players (`Pay` 1, or as much as is still useful, at a time; `Withdraw`
takes back everything you put on it in one resource, even after you lose your presence
there). The moment it is fully paid it is
discarded and the payments are spent (an overpayment, possible after a jam, goes
back, later seats first). In `threats.execute`, anything still on an uncountered threat
is refunded, and the threats execute in order (the row, then country 0, 1, ...) and are
discarded. Only threats that were on the board during this round's counter phase
execute: one dealt later in the round (a Discord tip in production or upkeep, False
Flag) waits beside its country (or the row) and executes next round, after a counter
phase in which it can be paid (022, Matt: it executed at once, uncounterable).

Remapped to the new pieces, choice threats preferred, permanent losses only in tier 2 (until
034: a destroyed presence now leaves the game, so a tier-1 threat's can be permanent too).
**Counters cost credits (024, candidate):** every counter is a third to half of a
round's income (the table prints them; they were 1-6 of mixed
resources, and tier 2 asked for antimatter nobody held, so it mostly executed):

<!-- generated: threats -->
| Threat | Tier | If not countered | Counter |
|---|---|---|---|
| Requisition | 1 | Each player here discards 2 cards or pays 6 credits. | 4 credits |
| Commandeer | 1 | Each player here discards 2 cards or pays 4 energy. | 4 credits |
| Supply raid | 1 | Each player with presence here loses 6 credits. | 5 credits |
| Blackout | 1 | Cities here produce no energy next round; the adversary gains 3 compute. | 4 credits |
| Sabotage | 1 | Players here make no energy in upkeep next round and each loses 3 tokens from their cards; the adversary gains 3 compute. | 5 credits |
| Infiltration | 1 | 3 adversary ambassadors land on player-owned pieces here (players choose each); the adversary gains 3 compute. | 5 credits |
| Agitators | 1 | 3 adversary ambassadors land on clean cities here (players choose each); each player here destroys a presence of theirs here. | 5 credits |
| Orbital strike | 1 | An adversary troop lands on a factory here (players choose); with no factory, on a city; the adversary gains 3 compute; each player here destroys a presence of theirs here. | 6 credits |
| Defection | 1 | Each player here loses an ambassador (from supply, else from the board); sway 2 toward Discord; each player here destroys a presence of theirs here. | 4 credits |
| Eviction | 1 | Each player here loses a presence here or pays 6 credits (with one presence on the board, pays). | 5 credits |
| Embargo | 1 | No site here can be claimed next round; the adversary gains 3 compute. | 4 credits |
| Smear campaign | 1 | Each player here gets 2 less insight next round; 2 Sway leaves the Unity bucket. | 4 credits |
| Station seizure | 2 | A research station here that is not corrupted becomes corrupted (players choose); the adversary gains 4 compute; each player here destroys a presence of theirs here. | 7 credits |
| Overrun | 2 | The city here that is not corrupted (clean or aligned) with the most adversary ambassadors becomes corrupted (ties: lower hex number); each player here destroys a presence of theirs here. | 8 credits |
| Mass production | 2 | The adversary builds now, here; the adversary gains 4 compute. | 7 credits |
| Purge protocols | 2 | Each player here discards their most expensive card in hand and loses 3 tokens from their cards. | 6 credits |
| Total war | 2 | 2 adversary troops land on factories here (else cities), each destroying any presence on its hex, and 2 adversary ambassadors on military bases here; the adversary gains 4 compute. | 8 credits |
| Conscription | 2 | Each player here discards 3 cards or pays 6 energy; the adversary gains 3 compute. | 7 credits |
| Fifth column | 2 | 4 adversary ambassadors land on player-owned pieces here (players choose each); the adversary gains 4 compute; each player here destroys a presence of theirs here. | 7 credits |
| Insurrection | 2 | 4 adversary ambassadors land on clean cities here (players choose each); sway 2 toward Discord; each player here destroys a presence of theirs here. | 7 credits |
| Siege | 2 | 2 adversary troops land on factories here (players choose; with no factory, on cities), and each player here destroys 2 presence of theirs here and loses an ambassador (from supply, else from the board); the adversary gains 3 compute. | 8 credits |
| Manhunt | 2 | Each player here destroys 2 presence of theirs here. | 8 credits |
<!-- /generated -->

The four 022 threats (placeholders, candidate) are built from the existing effects:
tier-1 shapes made harsher, choice threats where possible.

**Compute (017; 024: 1 more each).** About half the threats also give the adversary
compute when they execute uncountered, onto its projects (§3): candidate amounts, printed on
the threat.

An uncountered threat also sways `threat_discord` Discord. "Pay or discard" is the
player's choice; with neither possible, the player pays what they have and discards
their hand (**Default**). Purge protocols compares printed credit costs; on a tie the
player chooses.

### Player cards (module `cards`, variant `draft`; [`content/cards.json`](../content/cards.json))

The cards config lists which cards are in the game (baseline: all of them).

- **Decks:** a tier-1 and a tier-2 deck of the listed cards (one copy of each),
  shuffled at setup; discards reshuffle into a deck when it runs out. An AI's unique
  cards (§7) go back to the box when sacrificed or discarded. **Default (021):** when
  both decks and their discards are empty (every card is in a hand or in play), the draft
  hands are what has been dealt.
- **The opening (round 1).** Each player is dealt `opening_deal` tier-1 cards and
  keeps `opening_free` free; each further card kept costs `opening_keep_credits`
  credits. No draft in round 1 (**Default**, as Terraforming Mars' first generation).
- **The draft (rounds 2+).** `cards.tier`: each player builds a hand of `deal`
  cards **card by card**, choosing for each card which deck it comes from; it is dealt
  face up into the hand, so the next choice sees it. `cards.draft`: take one card, pass
  the rest to the next seat, until the hands are gone: to seat *s*+1 in odd rounds,
  *s*−1 in even rounds (as in Terraforming Mars); no passing. `cards.keep`: keep any
  drafted cards for insight, `keep_tier1` for a tier-1 card and `keep_tier2` for a
  tier-2 card (`keep_tier2_upgraded` with the board's keep upgrade); any insight short
  is paid at `insight_credits` credits an insight (keeps only). An Open Market free
  purchase is used first. The rest are discarded when the action phase begins.
- **Playing** a card costs its credits (buildings minus your discounts: Economies of
  scale, Orbital Mass Driver, the Builder growth's next-building discount, Industry tags; less what you
  spend at your AI's rate, §6), and any `insight`, `energy`, `chips` and `antimatter`
  it prints, and lowers any income it prints under `lowers` (022: "lower your credit
  income 1": your production of that resource falls for the rest of the game; it can't
  be played if that income is lower). **Insight to play (022):** 0 for small programs
  and events, 1 by default (removal events included), 2 for big effects and nearly all
  tier 2 (the set: 36 / 77 / 37 cards at 0 / 1 / 2); it comes from
  the same insight as keeps, and the credit conversion is for keeps only. Its `needs`
  (tags on your cards in play) are checked only when it is played. Each card's price is
  in [cards.md](cards.md); the reasoning is in [card-pricing.md](card-pricing.md).
- **Science owns a niche (041, 046, candidate).** Only Science cards clear corruption markers,
  put a hardening token on a hex of your presence (a shield, which absorbs a destruction there as
  on a building) or make a threat wait a round (a delay: it skips this round's execution, stays
  up, and can be countered next round); the content check
  refuses any of the three on a card without the tag, an AI board's growth or a power. Five tier-1
  and three tier-2 cards do it. No card `needs` Science for it: a team without the tag still has
  the Clear standard action, its counters and the board. Priced in
  [card-pricing.md](card-pricing.md).
- **Tag discounts (034, candidate; the Terraforming Mars pattern).** Every `science_every`
  Science tags you have take 1 insight off your tier-2 keeps and your card plays; every
  `industry_every` Industry tags take 1 credit off your buildings, as steel does. Neither goes
  below `discount_floor`: the cap on the loop "tags, cheaper costs, more tags"; a cost printed
  under the floor stays. Since 034 the floor holds for every building discount (it was 0).
- **Tags** (021): Science, Industry, Cyber, Power, Diplomacy, Security; 1-3 a card. Your
  tags are those on every card you have in play: active buildings, the cards splayed
  under them, and programs. Events' tags don't stay.
- **Programs** resolve and stay in your play area (their tags count; powers can sacrifice
  them). **Events** resolve and are discarded. A program or event (or ability) is only
  offered when each of its targeted effects has a target (a Harden or a "you may move" counts
  only on a card that does nothing else; 044). A removal's target is a hex with a piece the
  amount can pay for: "remove 1 worth" can't name a troop's hex. A building's when-built effect
  is not checked: with no target it does nothing.
- **Buildings** go into a slot: your AI board's `ai.slots`, plus Modular Expansion's
  (slotless). With a slot free you choose a new slot or **build over** one of your
  buildings; with none free you must build over one. The building underneath is
  **splayed down**: its passives and ability stop, its tags still count, and it is
  sacrificed with the stack. A building with `replaces` (most tier-2 buildings, over
  their own tag) must be built over one whose top card carries that tag. A building whose
  upkeep went unpaid is off (no passive, no ability) the next round.
- **Tokens** (021) live on buildings that print a type (`tokens`): Datasets (Science),
  Followers (Diplomacy), Capital (Industry), Agents (Security). An active token building
  gains its `per_round` in upkeep; a covered one holds its tokens but gains none and
  counts for nothing; Actions may cost tokens from their own card; some cards
  (Dataset Scraper, Manifesto Leak, Recruit Operatives) add tokens to a card of a type;
  Sabotage and Purge protocols make each player lose tokens.
  Sacrificing or discarding a card loses its tokens.
- **Actions** (abilities: `ability`, buildings only, 033) are used with `Use` once a round,
  pay their cost, and refresh at the start of a round. **Ongoing abilities live only on
  buildings:** income, Effect or Action on a program or event is refused when content loads.

Effects needing choices ask the acting player: hexes, countries, players, threats,
amounts ("up to N" offers 0..N), or "stop".

(Variant `off`: no draft, §9. 011's `free_keep` variant went with 021: keeps cost insight.)

### The card grammar

(033, **locked**; `crates/aic-core/src/face.rs` generates every face from the card's ops, so
a card's words can't differ from how it plays. No card text is hand-written.)

**Where it sits is what it is.** The top strip holds the credit octagon, the insight hexagon,
requirement plates (yellow: at least; red: at most; stacked cards: built over) and tags,
and nothing else. Above the art, on buildings only, is what lasts while the building is
active: income, *Effect:*, *Action:*. Cover the building and it stops. Below the art is what
happens once, when you play the card: extra costs first, then gains, placements and moves.

**Boxes.** No box: gain it once, now. A brown box is income: above the art it pays each
round ("Each round, ..."); below the art it moves your income track once ("Raise your X
income by N", "Lower ..." for a minus). A minus before a plain icon: pay or lose it now. A
dark frame acts on the adversary's pieces, projects or threats. A dashed outline is another
player or another card of yours; no outline is you, or this card.

**Connectors.** `→` is an Action: once a round, pay the left to get the right. A bare `→`
is free ("Action: once a round, get ..."). `:` is an Effect: when the left happens, get the
right. `/` is per. OR is written out. `*` is an exception, explained in brackets in the text.

**Shapes.** Square tiles are resources, discs are tokens on cards (alone, on this card),
bare silhouettes are pieces on the board and moving them, ringed circles are tags. A number
goes before its icon, no number means 1, and a negative always has its minus. In the dark
frame a clock is a delay (threats wait a round), a minus on a threat a jam (off one threat's
counter). The rules text says what the icons say, in the same order, and opens with the box's word.

Card details beyond the card text:

The card set (023): 150 cards in the draft decks, 108 tier 1 and 42 tier 2, about 50%
programs, 34% buildings and 16% events (033 moved the engines onto buildings), each tag
primary on 25; every card is listed
with its cost, insight, tags, kind and text in [cards.md](cards.md) (generated:
`aic-sim docs`, then `python scripts/cards/cards_doc.py`), and priced in [card-pricing.md](card-pricing.md).
Card text says what a card does; how each primitive resolves:

- **Reach (016):** a card's map target is in your reach (within 1 of your
  presence) where its text says so; Total Containment's hex and a country sweep are not. A removal (`damage`) takes a hex in reach holding adversary pieces (some
  cards narrow it to a country where you have presence); a country sweep any country
  holding them. An ambassador is worth 1, a troop `troop_health`.
- **Hardening:** any country holding a building a token can protect (022, 020's D10: a
  player-owned station or factory that is not corrupted), only buildings in reach;
  choose the country, then a different building for each token.
- **Factories:** a free industrial zone in reach; the factory is yours (your cube). A
  building that builds one can be played with no such zone and then builds nothing.
- **Moving ambassadors** (`move_ambassadors`, `move_one`, `consolidate`, `spread_ambassadors`): onto
  pieces (city, base, building or presence), each arriving as in §2; stop at any time.
- **Peek:** the top cards of the adversary deck, behind the face-up card, are shown; one
  fewer after each reveal.
- **Jam:** the chosen threat needs less of each resource (not below 0) while in play; if
  that leaves it fully paid, it is countered at once. **Counter a threat:** partial
  payments on it are refunded.
- **Cleanse:** take the adversary cube off a corrupted research station or factory in
  reach, one hex at a time (some cards: all in the first hex's country). A station
  becomes neutral. **Only a tier-2 ultimate (Great Liberation) can free a city this way**
  (024; `cities` on the effect; any freeing card did before), and it sways
  `alignment.free_city`.
- **Clear a city (024, `clear_city`):** remove up to N adversary ambassadors (Liberation
  Accords: all) around one corrupted city in reach, on it or next to it, the ones next to
  it first. This is how cards help free a city; moving your ambassadors there is the other
  half.
- **Card text (033):** generated from the card's ops (*The card grammar*, above), so it says
  everything, extra costs and lowered income included; a long card sets smaller. Terms:
  **remove N worth** (an ambassador 1, a troop `troop_health`); **harden N buildings** (N different ones
  in reach; only a player-owned, uncorrupted station or factory takes a token, Hardening
  above); **shelter one country** (nothing in it, building or presence, can be destroyed this round); **cut off
  a node** (it makes no compute in the next build).
- **Upkeep conversions** on a building are paid automatically in upkeep if you can.
- **Projects:** a drain removes compute from one visible adversary project; **drain all
  (023)** from every visible one; a discard takes one with its compute (if the row is
  then empty, the next row is dealt).
- **Nodes:** removing a node (tier 2, in reach) is, with Decommission (a Unity card, §5),
  the only way to remove one (§3); cutting one off (a hack) makes it produce no compute
  in the next build.
- **Draw (023):** take the top cards of the tier-1 deck into your hand, no keep paid
  (the deck reshuffles its discards when empty). Science's card draw.
- **Count as power (023, `gain_per_tokens`):** gain 1 of a resource per N tokens of a
  type on your active buildings, up to a cap; the tokens stay.
- **Cash out (023, `cash_out`):** remove all your tokens of a type from your active
  buildings (a capped card takes as many as it can, card by card); gain 1 + 2 + ... + n
  of a resource for the n removed (Industry's Capital: 4 Capital cash out for 10).
- **Help cards** (keyword `help`, ~10%): the player chooses another player; both gain,
  the helper paying.
- **Modular Expansion (Deviation):** it takes no building slot itself, so it is a net +1
  slot. **Aegis Dome Array's shelter:** automatic, the first destruction of a building or presence each round in a
  country where its owner has presence.

History: the placeholder cards as remapped in 014 (data centres became presence in 016).
023 replaced every card in this table except the AI-unique ones (Turnkey Factory, Point
Defense, Shelter Order, Strike Team); it is kept as a record of the old set.

| Card | Was | Now |
|---|---|---|
| Server Farm, Fork Process | place a processor | place a data centre |
| Data Mining | compute per processor | compute per data centre (max 4) |
| Firewall Node | explore skips your processors | spread skips your data centres |
| Watchtower Net, Precision Strike, Sweep, Orbital Lance, Strike Team | damage | remove adversary pieces worth N (ambassador 1, troop 2); Watchtower in a country with your data centre, Strike Team in reach |
| Purge Protocol | destroy every adversary piece in a hex | remove every adversary ambassador and troop in a hex |
| Swarm Override | remove every drone in a country | remove every adversary ambassador in a country (1 Unity per 2) |
| Emergency Fab, Turnkey Factory | build a factory on Industrial land | build a factory on an empty open hex touching a city |
| Rebuild / Reconstruction | return 1 / 3 corruption | take the adversary cube off 1 / **2** corrupted cities or stations (one country) |
| Shore Up / Point Defense / Protector growth | defense tokens | hardening tokens (3 / 2 / 2) |
| Evacuate | move 3 population | move up to 3 of your ambassadors onto pieces in reach |
| Shelter Order, Civic Shelter | population protected | buildings protected |
| Scavenge | if you destroyed a foundry | if you removed an adversary troop |
| Rally | 2 Unity if population and adversary share a hex | gain 1 ambassador; 2 Unity if a clean city holds adversary ambassadors |
| Recruitment Hub | (giving bonus) | also gain 2 ambassadors when built |
| Unity Network | +1 Unity per outpost/foundry destroyed | +1 Unity per adversary troop removed |

## 9. Off-variants

For measuring what a module adds, these variants remove its effect and keep the round:
`threats.variant=off` (no threats), `alignment.variant=off` (no Sway moves, nothing
tips, no antimatter action; the purge, the presence loss and the adversary deck-out stay; with no tips the Unity-deck
win and the Discord-deck loss can't happen), `cards.variant=off` (no draft or
drawing: only the AI's unique cards). Since 019 two of them no longer measure a module's
part (020, greedy, 40 games, r11): with Alignment off nothing tips, so nodes always count
against the purge and, at r11, every game ran the deck out (0% against the baseline's 97.5%);
with cards off only Decommission removes a node: 0%. Threats off wins
32.5%: Unity leans on the threats' churn. `board`, `adversary`, `alignment` and
`ai` are not removable (each is read by another); `threats=off`, `cards=off` and
`economy=off` remove the module (016 trimmed `requires` to what each module reads). Parts switch off by
parameters (`factory_units` 0, `factory_chips` 0, `escalation_ambassadors` 0,
`board.troop_steps` 0 (no march), `adversary.node_compute` 0,
`economy.give_insight` 0, or leaving `ai.growth` out of the round).

## Tunables

Every parameter of the baseline config, with its value and range. Generated from the
code and `configs/baseline.json`, like the other generated tables above (`aic-sim docs`):
edit the source, not the table.

<!-- generated: tunables -->
| Parameter | Baseline | Range | What it does |
|---|---|---|---|
| `board.start_corrupted` | 1 | 0..3 | cities per country that start corrupted |
| `board.start_ambassadors` | 1 | 0..10 | player ambassadors each player starts with |
| `board.unit_cap` | 2 | 0..20 | corp units a base holds at most |
| `board.factory_units` | 1 | 0..5 | corp units each factory makes on a production step at each of its corp's bases |
| `board.clock` | 1, 2, 1, 2, 1, 3 | 0..3 | the printed round track, per round: 1 production, 2 war, 3 both (the last entry repeats) |
| `board.corp_energy` | 1 | 0..10 | energy cubes each city of country i makes a round (a megacorp parameter; the list repeats) |
| `board.city_energy_credits` | 2 | 0..20 | credits for a neutral city's energy cube, bought by a player with presence next to it |
| `board.aligned_bonus` | 0 | 0..10 | extra energy an aligned city gives its owner |
| `board.station_cubes` | 1 | 0..5 | megaproject cubes a research station makes a round |
| `board.project_credits` | 10 | 0..50 | credits each player cube pays when a megaproject completes |
| `board.project_discord_per` | 2 | 1..10 | adversary cubes per Discord when a megaproject completes |
| `board.divert_discord` | 1 | 0..5 | Discord per unit of a corrupted city's energy diverted to the adversary |
| `board.ambassador_discord` | 0 | 0..5 | Discord per unit of energy an adversary ambassador on a clean or aligned city diverts |
| `board.base_discord` | 1 | 0..10 | Discord an adversary ambassador on a producing base sways |
| `board.base_unity` | 2 | 0..10 | Unity a player ambassador on a producing base sways |
| `board.factory_unity` | 1 | 0..10 | Unity a player ambassador on a producing factory sways |
| `board.hold_base` | 3 | 0..10 | a corrupted city's hold before adversary ambassadors: 1 more for each on or next to it |
| `board.last_safe_stage` | 2 | 0..3 | while this round's adversary card is below this stage, nothing destroys a player's last presence (0: never safe) |
| `board.hold_min` | 2 | 1..10 | player ambassadors that free a corrupted city, at the least |
| `board.free_leaves` | 1 | 0..5 | adversary ambassadors left on a city as it frees (the piece ladder) |
| `board.destruction_discord` | 0 | 0..5 | Discord each building destroyed in war sways |
| `board.troop_spent` | 1 | 0..1 | 1: an adversary troop that strikes a target is used up |
| `board.troop_steps` | 2 | 0..5 | steps each adversary troop takes in the march: destroy a target on or next to it, else move 1 hex toward the nearest |
| `board.troop_presence` | 4 | 0..4 | adversary troops strike presence from this stage of adversary card on (0: always, 4: never) |
| `board.presence_marker` | 1 | 0..1 | 1: a destroyed presence leaves a corruption marker on its hex |
| `board.corruption_reach` | 1 | 0..1 | a city that corrupts destroys presence within this many hexes of it |
| `board.war_reach` | 0 | 0..1 | war that destroys a factory destroys presence within this many hexes of it |
| `board.presence_energy` | 1 | 0..20 | standard action: energy to place a presence, on top of the track step's cost |
| `board.presence_action_cap` | 1 | 0..5 | standard action: placements a seat may make with it a round (0: no limit; the rich action has its own rise) |
| `board.build_energy` | 2 | 0..20 | standard action: energy to build a research station |
| `board.station_chips` | 2 | 0..20 | standard action: chips to build a research station |
| `board.factory_chips` | 1 | 0..5 | chips a player-owned factory makes its owner on a production step |
| `board.chips_credits` | 1 | 0..50 | standard action: credits for selling 1 chip |
| `board.remove_energy` | 1 | 0..20 | standard action: energy to remove an adversary ambassador within reach |
| `board.clear_energy` | 1 | 0..20 | standard action: energy to clear a corruption marker within reach |
| `board.clear_unity` | 1 | 0..5 | Sway toward Unity for clearing a corruption marker |
| `board.clear_credits` | 0 | 0..5 | credits back for clearing a corruption marker |
| `board.move_energy` | 1 | 0..20 | standard action: energy to move a player ambassador |
| `board.ambassador_credits` | 6 | 1..50 | standard action: credits to gain a player ambassador (at least 1: a free untargeted action has no limit) |
| `board.harden_energy` | 1 | 1..20 | standard action (Harden passive): energy a hardening token (at least 1: free, it has no limit) |
| `board.sell_credits` | 1 | 0..20 | standard action: credits for selling a card from hand |
| `board.align_energy` | 2 | 0..20 | standard action: energy (and 1 ambassador) to align a clean city |
| `board.presence_credits` | 6 | 1..50 | the rich action: credits to place a presence, before its rise and the step's cost |
| `board.presence_rise` | 3 | 0..20 | the rich action: credits more for each earlier use this round |
| `board.presence_stage` | 2 | 1..3 | the rich action: from this stage of adversary card on |
| `board.harden_credits` | 3 | 1..50 | standard project: credits for a hardening token |
| `board.energy_credits` | 4 | 1..50 | standard project: credits for 1 energy |
| `board.drain_credits` | 5 | 1..50 | standard project: credits to drain compute from an adversary project |
| `board.drain_amount` | 3 | 1..20 | standard project: compute drained |
| `adversary.difficulty` | 0 | 0..4 | difficulty level 0-4 (each level includes the ones below) |
| `adversary.stage_cards` | 4, 5, 2 | 0..20 | adversary cards dealt from stage I, II and III (before difficulty) |
| `adversary.supply_per_country` | 24 | 0..40 | adversary ambassadors in the supply per country |
| `adversary.city_build` | 1 | 0..10 | build: ambassadors each corrupted city gains |
| `adversary.factory_build` | 0 | 0..10 | build: ambassadors each factory next to a corrupted city gains |
| `adversary.spread_range` | 2 | 1..6 | spread: how far (hexes) a spreading ambassador may travel |
| `adversary.corrupt_at` | 6 | 1..20 | corrupt: a city that is not corrupted, holding this many adversary ambassadors, is corrupted |
| `adversary.escalation_ambassadors` | 1 | 0..5 | stage-II escalation: ambassadors each military base gains when the card builds |
| `adversary.counter_bonus` | 0 | 0..10 | difficulty 1+: added to every threat's first counter requirement |
| `adversary.counter_bonus_all` | 0 | 0..10 | difficulty 1+: added to every counter requirement of every threat |
| `adversary.extra_threats` | 1 | 0..5 | difficulty 1+: extra threats dealt each round in all, one per country in turn |
| `adversary.stage1_removed` | 1 | 0..3 | difficulty 2+: stage-I cards removed |
| `adversary.stage3_added` | 1 | 0..3 | difficulty 2+: stage-III cards added (041: the cap holds, its end gets harder) |
| `adversary.stage2_removed` | 1 | 0..3 | difficulty 3+: stage-II cards removed (a shorter round cap) |
| `adversary.extra_corrupted` | 0 | 0..3 | difficulty 4: more cities per country start corrupted |
| `adversary.build_bonus` | 1 | 0..5 | difficulty 3+: ambassadors each corrupted city gains on top of city_build |
| `adversary.tier2_early` | 1 | 0..3 | difficulty 4+: tier-2 threats start this many rounds sooner |
| `adversary.discord_removed` | 1 | 0..3 | difficulty 4+: Discord cards removed at setup (from tier 1) |
| `adversary.arrivals` | 1 | 0..5 | build: ambassadors each site the adversary card names gains, whatever the board holds |
| `adversary.troop_health` | 2 | 0..10 | removal a card must spend on an adversary troop (an ambassador takes 1) |
| `adversary.node_compute` | 1 | 0..5 | adversary compute each node makes a round (none with a player ambassador on it) |
| `adversary.project_cards` | 6, 6 | 0..20 | adversary projects dealt from stage 1 (cheap) and stage 2 (big), cheap on top |
| `adversary.project_row` | 3 | 1..6 | adversary projects visible at once; a new row is dealt when all have resolved |
| `threats.per_country` | 1 | 0..5 | threats dealt each round per country |
| `alignment.undecided_per_player` | 4 | 1..50 | Sway in the Undecided at the start, per player |
| `alignment.tip_per_player` | 2 | 1..50 | the Unity bucket tips when it holds this many Sway per player |
| `alignment.discord_tip_per_player` | 2 | 1..50 | the Discord bucket tips when it holds this many Sway per player |
| `alignment.momentum_every` | 0 | 0..10 | Momentum: a side's bucket holds 1 fewer Sway per this many of its tips, down to 1 per player (0: off) |
| `alignment.troop` | 1 | 0..10 | Sway toward Unity for removing an adversary troop |
| `alignment.free_city` | 5 | 0..10 | Sway toward Unity for freeing a corrupted city |
| `alignment.node` | 2 | 0..10 | Sway toward Unity for removing a node |
| `alignment.threat_discord` | 1 | 0..10 | Sway toward Discord when a threat executes uncountered |
| `alignment.adversary_discord` | 0 | 0..10 | Sway toward Discord the adversary adds at the start of every round |
| `alignment.antimatter_discord` | 1 | 0..5 | the antimatter action: Sway toward Discord to produce 1 antimatter (0: no such action) |
| `alignment.antimatter_uses` | 1 | 0..5 | times each player may take the antimatter action per round |
| `alignment.unity_ease` | 2, 2, 4 | 0..50 | Unity tip counts at which the clear-board bar drops a level: ambassadors, then troops, then nodes stop counting (the first also starts the stronger Unity cards) |
| `alignment.unity_cards` | 3, 3 | 1..6 | Unity cards a game deals from the printed deck, tier 1 then tier 2 (drawing the last one wins) |
| `alignment.discord_tier2` | 2 | 1..50 | Discord tip count from which a Discord tip draws the stronger tier |
| `alignment.presence_loss` | 1 | 0..1 | 1: a player with no presence on the board loses the game for everyone |
| `alignment.clear_win_stage` | 1 | 1..3 | the bar-0 win counts only once this round's adversary card is of this stage (1: any time) |
| `economy.give_insight` | 1 | 0..10 | insight the receiver gains for 1 given |
| `ai.boards` | 0, 1 | 0..99 | AI board (index into content/ais.json) of each seat; the list repeats for more seats |
| `ai.start_credits` | 20 | 0..100 | credits each player starts with |
| `ai.slots` | 4, 3 | 1..10 | building slots of each seat's AI board, in the order of boards; the list repeats |
| `cards.deal` | 4 | 0..10 | cards each player drafts from each round after the first, choosing each one's tier |
| `cards.opening_deal` | 8 | 0..20 | cards dealt to each player in round 1 (the opening) |
| `cards.opening_free` | 4 | 0..20 | opening cards each player keeps free |
| `cards.opening_keep_credits` | 5 | 0..20 | credits for each opening card kept beyond the free ones |
| `cards.keep_tier1` | 1 | 0..10 | insight to keep a drafted tier-1 card |
| `cards.keep_tier2` | 3 | 0..10 | insight to keep a drafted tier-2 card |
| `cards.keep_tier2_upgraded` | 2 | 0..10 | insight to keep a tier-2 card once the AI board's keep upgrade is uncovered |
| `cards.insight_credits` | 5 | 1..20 | credits for 1 insight, usable only for keep costs |
| `cards.science_every` | 2 | 0..10 | 1 insight off tier-2 keeps and card plays per this many Science tags (0: off) |
| `cards.industry_every` | 2 | 0..10 | 1 credit off buildings per this many Industry tags (0: off) |
| `cards.discount_floor` | 1 | 0..5 | tag and building discounts stop at this cost; a cost printed under it stays |
<!-- /generated -->

## Defaults and deviations

**Defaults** (gaps in the prompt or the v0 ruleset, filled with the simplest reading
that matches its intent):

1. Bases are placed by the layout (fewest candidates first, lowest number) on the
   printed fronts, not printed themselves.
2. One player per country; seat *s* starts in country *s* on the printed presence spot.
3. Used player ambassadors are gone; adversary ambassadors return to the supply.
4. Ambassadors cancel the moment opposing ones meet on a piece.
5. (016) Each factory makes `factory_units` troop at each of its corp's bases per production step; a
   base holds at most `unit_cap` (027: was 3) troops, war or peace.
6. Spread goes up to `spread_range` hexes, by priority then distance, only to pieces and
   player-owned hexes; non-corrupted cities collect any number; a second ambassador on
   another piece makes a node; a surplus with nowhere to go returns to the supply.
7. A clean city's energy with no presence on or next to it is lost.
8. A destroyed presence leaves the game, leaving a corruption marker (034; 028: back on its track; 018–027: a destroyed pile; 016–017: it left the game); a destroyed station or factory takes its cube.
9. (019, 027) Adversary troops target player-owned stations and factories, not presence
   (016–018: any factory not adversary-owned and any station not corrupted).
10. The deck runs out when a round starts with no face-up card (the round cap, §3).
11. (027) The supply is a cap: a placement it can't make doesn't happen.
12. "Pay or discard" with neither possible: pay what you have, discard your hand.
13. Team choices are asked of seat 0 (an aligned city's factory site, of its owner).
14. `adversary_discord` 0: corrupted cities' diverted energy replaces the flat drip.
15. "Next to" = on or adjacent; "adjacent" = neighbouring, across borders.
16. Win and loss are checked when an action or automatic phase finishes resolving: full
    buckets tip first (Discord before Unity), then losses, then the purge. A tip that ends the
    game (the last Unity card drawn, a Discord tip with no card) ends it there.
17. (013, 017) Alignment: a tip draws the stronger tier when the side's tip count
    *before* the tip has reached its threshold; the antimatter action is once a round
    per player. (017: no ongoing cards and no Discord escalation.)
18. Discord cards: extra threats come from the current tier, without a reshuffle, one
    per country in turn from country 0; Blight and Firestorm ties go to the lower hex
    number; Offensive acts at once with this round's card (before the build
    step, the face-up card: 022).
19. (011) Difficulty 1's extra threat goes to the countries in turn by round number
    (country 0 in round 1); difficulty 4's "a round sooner" looks `tier2_early` cards
    ahead (the face-up card first).
20. Pooled counters: countered only when fully paid; withdrawals until the action phase
    ends; partial payments refunded when threats execute (Production 5).
21. The shared threat row is dealt `per_country` threats per country.
22. The draft passes alternate direction by round (Terraforming Mars).
23. "Next K purchases" (Open Market) are keeps of drafted cards, by any player.
24. A building with no free slot is built over one of your buildings (your choice), which
    is splayed down under it.
25. Insight is set at the start of each round (unspent insight is lost); income arrives
    in upkeep.
26. Tags are counted on every card you have in play, splayed buildings included; events'
    tags don't stay.
27. Unique AI cards return to the box when used up; `ai.boards` repeats for more seats,
    and a repeated AI has no unique cards (one copy each).
28. Building upkeep is automatic: Arc Furnace Complex and Positron Storage Ring convert
    whenever you can (no choice is asked).
29. A card that gives from your own stock (`transfer`: Development Loan, Vendor Financing)
    is not offered without another player; other help cards, such as Venture Capital, are
    (their own gain still applies).
30. (016) The presence track replaces both AI tracks; uncovers are per-round income.
31. (016) The clock gates factories, troops, chips and war only.
32. (016) Setup factories start neutral.
33. (016) A corrupted city builds within its own country; its preferences come before
    distance.
34. (016) Presence is one token per hex of any player; it may share a hex with a city or
    building.
35. (016) A node can't form on a clean city (cities collect ambassadors to corrupt) or
    on a piece that already has a node; both ambassadors return to the supply.
36. (016) Sabotage stops the upkeep energy of players with presence in the country.
37. (016) Card-built factories (Turnkey Factory, Prefab Plant Kit) are the player's own.
38. (017) Project costs are printed per player; compute goes on one point at a time
    onto the cheapest visible project (ties: the first in the row); compute with no
    project left is lost; a discarded project counts as gone for dealing a new row.
39. (017) Ongoing projects act in the build, after the ambassadors and escalation and
    before the nodes' compute; a project resolved in a build acts from the next one.
40. (017) A tip empties its bucket and counts before its card resolves; the card's step
    reads the count before the tip. (027: neither side's deck reshuffles.)
41. (017) "Hidden like fear cards": the Alignment decks are face down and a tipped card
    resolves at once.
42. (019) The march: troops march one at a time in hex-number order fixed when the
    march starts, each once, all their steps before the next; seat 0 chooses among
    equally good steps and among targets in reach. (017–018's Discord advance is gone.)
43. (017, 027) The Undecided starts at `undecided_per_player` a player (017: 3).
44. (017, 027) A sacrifice (Purge the Unaligned): each player picks any one of their
    ambassadors (supply or a hex) or presence tokens; a last presence isn't offered, and a
    sacrificed presence leaves the game.
45. (020) A threat's round flag (Blackout, Sabotage) lasts to the end of the next round:
    threats execute after production and upkeep, so "this round" never landed. A
    card's flag (Shelter) still lasts to the end of its own round.

46. (021) Round 1 is the opening: no draft (Terraforming Mars' first generation); the
    opening deal is tier 1.
47. (021) Each draft pick of a tier deals that deck's top card face up into your hand
    (so the next tier choice sees it); with both decks and discards empty, the draft
    hands are what was dealt.
48. (021) A neutral city's energy: one players-choose question per city in production,
    among players with presence on or next to it who can pay (or nobody).
49. (021) A corrupted city's cube "goes to the adversary" as the diverted Discord sway it
    was (the adversary has no energy).
50. (021) Power payment: what can be sacrificed is fixed when the power is used; each
    sacrificed card pays with all its tags, each tag first toward its own requirement;
    the player picks cards one at a time among those that still help.
51. (021) A token card gains its tokens in upkeep while it is an active building (not
    splayed, not unpowered); only buildings hold tokens; sacrificing or discarding a card
    loses its tokens.
52. (021) The movement add-on's "up to 2 hexes" is hex distance (`Map::distance`, over
    land and sea); the destination must be a piece. Spread moves each ambassador at most
    once, in hex order.
53. (022) A threat dealt after the round's counter phase executes next round, not this
    one (at a table: it goes beside the row until then).
54. (022) A player ambassador on an adversary-owned factory is used up for
    `factory_unity` Unity on a production step; the factory still makes its troop.
55. (022) Hardening goes only on player-owned stations and factories that are not
    corrupted (what a troop strikes), one token a building per effect; a Science card's
    shield (041) is the one exception, on a hex of your presence (§7).

56. (024, 027) Freeing is checked with the win and the tips, after every action and
    automatic phase; the hold's worth of ambassadors are used up, lower seats first.
57. (024, 027) "On or next to" counts the city's own hex too.
58. (024) Clear a city takes the ambassadors next to the city first, lowest hex first, then
    the ones on it (no choice is asked: it is what a player would choose).

**Deviations** (from the v0 ruleset's card text, kept from 013):

1. Modular Expansion takes no slot.

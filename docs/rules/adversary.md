# The adversary (module `adversary`, variant `militarist`)

*The adversary's round, its project deck, its card deck and round cap, and difficulty. Terms and conventions are in [README.md](README.md).*

## Each round

1. **Build** (`adversary.build`): next round's face-up card becomes this round's card.
   **Arrivals (candidate):** each card names a site type, and every matching site in every
   country gains `arrivals` adversary ambassadors, whatever the board holds (below). Each
   corrupted city gains `city_build` (2, Matt: the adversary makes ambassadors faster than
   the party can easily deal with) adversary ambassadors, and each factory next to a
   corrupted city gains `factory_build`. The card's twists add more. **Troop pressure where
   the Protector stands (candidate):** then, in each country where a seat with Wall has a
   node, `wall_troops` adversary troops land as any landing does (on a factory there,
   players choose which, else a city; never on a hex a Wall holds), so Wall has strikes to
   stop ([war.md](war.md)). A stage-II card's **Escalation** then gives each country's first base (below)
   `escalation_ambassadors`. Ongoing **projects** act (below). Last, each **botnet** makes
   `botnet_compute` **adversary compute**, onto the projects; a botnet holding a player
   ambassador makes none (the ambassador stays).
2. **Spread** (`adversary.spread`): every hex holding several adversary ambassadors, fixed
   when the step starts, keeps one and sends each other one to a hex within `spread_range`.
   **Why 2:** spread is the adversary's movement, a dial on its pressure, not a player's
   reach, so the players' reach-1 rule doesn't bind it; the +1 of Long Patrols and
   Blitzkrieg is those twists' whole escalation.
   **Spread never targets an empty hex**: a destination holds a piece or is player-owned
   (node, an aligned city, an owned station or factory); it is not ocean, and it holds no
   adversary ambassador unless it is a city that isn't corrupted, clean or aligned (which
   collects any number), or a piece without a botnet (below); a hex holding nodes of a seat
   with `spread_shield` (a card's Effect, or the Sanctuary capstone) is no destination.
   **Priority:** player-owned hexes, then clean cities, then other pieces; then the nearest;
   players choose among equals. A city that is not corrupted never spreads: it holds its
   ambassadors. Each ambassador moves at most once a spread, in hex order. **A surplus with nowhere to go returns to the supply.**
3. **Corrupt** (`adversary.corrupt`): a city that is not corrupted holding `corrupt_at` (4)
   adversary ambassadors becomes corrupted (an aligned city loses its player cube); its
   ambassadors return to the supply; any player ambassadors waiting there are lost (on any
   corruption; they would free it in the same step).
4. **Reveal** (`adversary.reveal`, after `threats.deal`): the top card goes face up as next
   round's card.

**Botnets.** A botnet is made when a spreading adversary ambassador lands on a piece that
already holds one (not a city that isn't corrupted, which collects them, and not a piece
that already has a botnet): both ambassadors return to the supply and a botnet goes on the
piece. Each board starts with a botnet (on its research station). A botnet stays until
removed: it makes `botnet_compute` adversary compute a round, it **blocks site claims** (a
site with a botnet on or next to it can't be claimed), and it counts against the purge until
Unity eases it ([alignment.md](alignment.md)). **Botnets are hard to remove (candidate, Matt):** no standard action
does it, only tier-2 effects: the tier-2 cards **Botnet Takedown** and **Singularity
Exploit** (which also discards a project), each removing a botnet in your reach; **Root Access**, one on or next to the city it liberates; and the
tier-2 Unity card **Decommission** (a botnet anywhere, players choose). Removing a botnet
sways `alignment.botnet` Unity ([alignment.md](alignment.md)).

## The project deck (near-locked)

Adversary compute is spent on **projects**, one shared deck for every adversary
([`content/projects.json`](../../content/projects.json), placeholders). The deck is staged like
the threat deck: `project_cards` are dealt, the cheap stage-1 projects shuffled on top of
the big stage-2 ones. `project_row` projects are face up at once; when all of them have
resolved (or been discarded), the next row is dealt.

- **Compute sources:** each botnet, `botnet_compute` a round in the build (none under a
  player ambassador); about half the threats carry compute, gained when they execute
  uncountered ([threats.md](threats.md)); an adversary ambassador draining a lone node, 1 in production ([ambassadors.md](ambassadors.md)); the
  corp cards, an adversary cube at Cintamani's completion ([board.md](board.md)).
- **Spending:** each compute point goes onto the **cheapest** visible project (ties: the
  first in the row); content may print compute on every visible project (`compute_every`);
  none does. When a project holds its cost it **resolves**: a one-shot effect at once, or
  an **ongoing** one that acts in every round's build step from the next round on. Choices
  are the players' (seat 0; Purge the Unaligned's loss, each player's own). With no project
  left, compute is lost.
- **Cost** is printed per player (the prompt's 12–15 and 25–30 compute are
  3-player numbers; a per-player cost keeps the timing at 2 and 4 players). The table
  prints each cost.
- **Counterplay:** a player ambassador on a botnet stops its compute and is not used up;
  Cyber's hacks drain 3-6 compute from one project (Threat Hunt, Logic Bomb, Bug Bounty Board, ...) or
  discard one with its compute (Killswitch Directive, Singularity Exploit); the drain standard project
  ([economy.md](economy.md)) does it for credits. No card drains every project (r44: compute sits on the cheapest).

<!-- generated: projects -->
| Project | Stage | Compute per player | Resolves | Text |
|---|---|---|---|---|
| Disinformation | 1 | 5 | once | Sway 4 toward Discord. |
| Botnet Swarm | 1 | 5 | once | Every botnet produces 2 adversary ambassadors on its hex. |
| Market Crash | 1 | 5 | once | Every player loses 8 credits. |
| False Flag | 1 | 6 | once | Deal 3 extra threats. |
| Infiltration Drive | 1 | 6 | once | In each country, the uncorrupted city with the most adversary ambassadors becomes corrupted. |
| Rapid Buildout | 1 | 6 | once | The adversary builds now, then spreads. |
| Purge the Unaligned | 2 | 8 | once | Every player loses an ambassador or a node (never their last node). In each country, 1 troop lands on a factory, or a city if no factory. |
| Recruitment Drive | 2 | 9 | ongoing | Ongoing: every round, an extra adversary ambassador at every clean city. |
| Propaganda Network | 2 | 9 | ongoing | Ongoing: every round, sway 2 toward Discord. |
| Total Offensive | 2 | 9 | once | The adversary builds and spreads now; sway 2 toward Discord. |
| Armored Columns | 2 | 9 | once | In each country, 2 troops land on factories, or cities if no factories. |
| Station Coup | 2 | 10 | once | In each country, an uncorrupted research station becomes corrupted (a player's holds unless 2 adversary ambassadors and no player ambassador stand on it: an adversary ambassador lands there instead). Every botnet produces 1 adversary ambassador on its hex. |
<!-- /generated -->

Compute goes on one point at a time, so a full project resolves before the
next point lands; Purge the Unaligned's loss is any one of the player's ambassadors (supply
or a hex) or nodes, their pick, never their last node; a node lost this way leaves the game,
as a destroyed one does ([war.md](war.md)). Ongoing projects act in the round's build step only, not in an
out-of-turn build (Offensive, Rapid Buildout, Mass Production).

## The deck (the clock and the stager)

[`content/adversary.json`](../../content/adversary.json) prints the cards below. A game deals
`stage_cards` at random per stage, each stage shuffled on its own, stage I on top: as many
rounds as cards, **the round cap** (**locked** that a cap exists and can shorten; the number
a candidate). **11 rounds through d2:** difficulty 2 swaps a stage-I card for a stage-III
card (`stage1_removed`, `stage3_added`), so the cap bites through a harder end, not a
shorter game; difficulty 3 removes a stage-II card (`stage2_removed`): 10. Some adversaries
may shorten it too. The next round's card is always face up a round ahead (peek cards show
more). Each card's twists change its round.

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
| Probing Strikes | 1 | Arrivals: coastal cities. |  |
| Factory Drive | 1 | Arrivals: industrial zones. Factories next to clean cities gain 1 adversary ambassador. |  |
| Long Patrols | 1 | Arrivals: research stations. Spread reaches 1 hex further. |  |
| War Drums | 1 | First, sway 1 toward Discord. Arrivals: each country's first base. |  |
| Mobilization | 2 | Arrivals: coastal cities. Each corrupted city gains 1 more adversary ambassador. Escalation: each country's first base gains an adversary ambassador. | yes |
| Station Raids | 2 | Arrivals: research stations. Each corrupted research station gains 1 adversary ambassador. Escalation: each country's first base gains an adversary ambassador. | yes |
| Fifth Column | 2 | Arrivals: coastal cities. Cities corrupt at 1 fewer adversary ambassador. Escalation: each country's first base gains an adversary ambassador. | yes |
| Border War | 2 | First, each player destroys 1 of their nodes. Arrivals: each country's first base. This round is a war round. Escalation: each country's first base gains an adversary ambassador. | yes |
| Supply Lines | 2 | Arrivals: industrial zones. Factories next to clean cities gain 1 adversary ambassador. Escalation: each country's first base gains an adversary ambassador. | yes |
| Total Mobilization | 3 | First: sway 1 toward Discord; each player destroys 2 of their nodes; move 2 Sway from the Unity bucket to the Undecided. Arrivals: industrial zones. Each corrupted city gains 1 more adversary ambassador; factories next to clean cities gain 1 too. |  |
| Blitzkrieg | 3 | First: sway 1 toward Discord; each player destroys 2 of their nodes; move 2 Sway from the Unity bucket to the Undecided. Arrivals: each country's first base. This round is a war round, and spread reaches 1 hex further. |  |
| Occupation | 3 | First: sway 1 toward Discord; each player destroys 2 of their nodes; move 2 Sway from the Unity bucket to the Undecided. Arrivals: research stations. Cities corrupt at 1 fewer adversary ambassador, and each corrupted research station gains 1 adversary ambassador. |  |
| Iron Doctrine | 3 | First: sway 1 toward Discord; each player destroys 2 of their nodes; move 2 Sway from the Unity bucket to the Undecided. Arrivals: coastal cities. This round is a war round. |  |
<!-- /generated -->

**Arrivals: the explore step that never stops.** The site types are **coastal cities**,
**industrial zones** (built on or not: an ambassador may stand on an empty zone, the one
exception to "ambassadors sit on pieces"), **research stations** (whoever owns them) and
**each country's first base**. A coastal city is one next to ocean or printed coastal on its
piece (`coast` in `content/map.json`): every piece has two coastal cities. Horizon gets both
from its ocean, Cintamani prints one more, Onyx and Biomia print both. Next round's card is
face up, so its site type is visible a round ahead. Each stage's cards spread the types.
An out-of-turn "the adversary builds now" brings the card's arrivals too, in its
scope: one rule for a build.

**A country's first base (candidate; `base_sites_per_country`):** its lowest-numbered
military base. Arrivals and Escalation reach only that one, so what reaches bases counts
countries, not fronts (a country has one front at 2 countries and two at 3 and 4). The count
is fixed by the printed board, so it can't feed back. A base's other rules hold: an
ambassador that spreads onto a second base still diverts it, and a threat that lands on
bases here (Total War) lands on all of them.

**For new players.** An adversary card may name military bases, and stage II cards send an
extra ambassador to them. Either way it lands on one base in each country, the
lowest-numbered. A base with an ambassador on it swings the world toward Discord when it
next produces, so guard your first base. **What resisted this:** "first base" is a new term
for a printed number, it only means anything at 3 and 4 countries, and an ambassador can
still spread onto the second base.

**The supply is a cap.** A placement the supply can't make doesn't happen; running out
doesn't lose.

Players **lose** when a round starts with no card to turn face up (after the round cap's
last round): a quiet backstop nobody plans around.

## Difficulty (`difficulty`, 0–4; each level includes the ones below)

<!-- generated: difficulty -->
| Level | Adds | Tunable |
|---|---|---|
| 0 | Base rules |  |
| 1 | Extra threats each round in all, countries in turn | `extra_threats` 1, `counter_bonus` 0, `counter_bonus_all` 0 |
| 2 | A stage-I card swapped for a stage-III card: stage II (escalation, tier-2 threats) sooner and a harder end, at the same round cap (the table above) | `stage1_removed` 1, `stage3_added` 1 |
| 3 | Larger builds: each corrupted city gains more, and a stage-II card removed (a shorter round cap) | `build_bonus` 1, `stage2_removed` 1 |
| 4 | A still shorter Discord deck: the removed cards count as tips already made, so every printed step still acts; tier-2 threats sooner; optionally more corrupted cities per country at setup | `discord_removed` 1, `tier2_early` 1, `extra_corrupted` 0 |
<!-- /generated -->


Difficulty 1's extra threat goes to the countries in turn by round number (country 0 in round 1); difficulty 4's
"tier-2 threats sooner" looks `tier2_early` cards ahead (the face-up card first).

The difficulty is only the configured level: Discord tips don't raise it ([alignment.md](alignment.md)). **The
fallback's `liberate_at` is the same at every difficulty:** difficulty comes from the
adversary and the decks: Discord deck size, threat strength and the round cap. The target
is set in `docs/principles.md`; the current reads are indexed in
[experiments/history.md](../../experiments/history.md).

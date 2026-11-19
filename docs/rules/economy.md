# Resources and the round

*Terms and conventions are in [README.md](README.md).*

| Resource | Comes from | Spent on |
|---|---|---|
| Credits | Credit production on the AI board ([ai-boards.md](ai-boards.md)), the node track, cards and Unity cards, in upkeep; megaproject payouts; Growth (Buy In and Levy 1, Accord and Peer Review 2, Annex 6); selling a card (`sell_credits` 1) | Card costs (never keeps); standard actions (table below); the Researcher's Lab II (2); threat counters |
| Energy (keeps between rounds; candidate: about 1-3 held mid-game, near 0 at the end) | Cities' energy in the income step (1 from each unblocked, uncorrupted city with your node on or next to it, *Cities' energy* below); Horizon's completion payout and its two converters (2 cubes for 4, 1 chip for 2); Make antimatter's 2; the node tracks; Power programs (+2 at most on any other card), the Power engines' when-built 2 and Power triggers ([cards.md](../cards.md)); the Builder's Overcharge; each board's printed energy; `ai.start_energy` at the start | **Energy powers things** (Matt, 2026-11-15): every track step, whatever places the node ([ai-boards.md](ai-boards.md)); the board powers: Outpost, Sprawl and Accord (1 each), Patrol I / II (1 / 2) ([ai-powers.md](ai-powers.md)); building Actions, most at 1-2 ([cards.md](../cards.md)); cards' `energy` cost (power-hungry cards, counted in [player-cards.md](player-cards.md)); threat counters that offer energy (by tag, [threats.md](threats.md) *Counters*) |
| Chips (4 credits at the anchors and `values.json`) | Player-owned factories each round (`factory_chips` each); Onyx's converter; the Builder's Scrap I (1) and Full Capacity capstone; cards (chip income on four, a one-time gain on three, Wafer Fab Line's Action 2 a use: [cards.md](../cards.md)). | The Builder's Fortify I / II (1 / 2) and Annex (1); Parts Exchange's Action; research stations (`station_chips`); the corps' chip converters, 1 a use (Biomia's 2; [board.md](board.md)); Antiproton Foundry's Action. Every chip price is 1 but a station's, Fortify II's and Biomia's converter's, 2. Chips can't be sold |
| Antimatter (18 at the anchors; the majors' fuel) | The plants, the steady source: a building's Action, once a round, 1 for a cube, a chip, 2 energy, 2 insight or 5 credits (each its own plant; tier 1: 2 energy; two give a second
thing beside it: Antiproton Foundry 2 energy, Treaty Isotope Bank an ambassador). The Builder's Overcharge II ([ai-powers.md](ai-powers.md)). Sanctuary's and Open Archive's 2 once, as the capstone is uncovered (263: was 1 a round); Zero-Day Auction, Intrusion Lab. The desperate Make antimatter (below) | The majors: 15 tier-2 cards at 1 (liberations, troop strikes and more; the big sinks pay 2.0-2.5× at the anchors, Matt 287); Orbital Relay Launcher's Action (1 for 3 nodes and 3 energy, once a round) and Kinetic Interdiction's (1 for 10 agents in a country); two tier-1 cards ([cards.md](../cards.md)) |
| Research cubes (keep between rounds) | Your stations, once their country's megaproject is complete; Cryo Sample Vault's 1 a round (tier 1); the Researcher's Open Archive capstone; cards: Field Grant (3), Particle Accelerator's Action (3, its OR half), Collider Consortium (8, another player 3), Survey Drone Fleet when played (2) | The standard actions Turn a cube into insight, Make antimatter and Lift Fallout (below); the corps' converters ([board.md](board.md): Horizon's 2 for 4 energy, Biomia's 1 Unity a cube); Penning Trap Array's, Field Assay Team's, Research Reactor's (3 energy) and Survey Drone Fleet's (2 nodes and 2 energy) Actions; Forensic Audit |
| Insight | Each round, set to the card-play track's total (3 at its start; each board's top: [ai-boards.md](ai-boards.md)) plus buildings' and programs' insight production and Think Tank's 1; no adversary card takes it. Mid-round: the cube converter (standard action) and Cintamani's (1 cube for 2, once a round, a Cintamani city in reach); Vigil ([ai-powers.md](ai-powers.md)); card triggers; the Researcher's Open Science (others 2 a round) | Keeping drafted cards; playing cards; threat counters that offer it ([threats.md](threats.md) *Counters*); Dark Fiber Collider's Action; the Researcher's Growth (Peer Review 1, Field Station 1) and Lab (1). Only cards that print it pass it between players. It expires at the round's end, but the Researcher's carries over (Patience, [ai-boards.md](ai-boards.md)) |
| Ambassadors | `start_ambassadors` at the start; Diplomacy cards, Followers through a card's Action, Orbital Embassy's 1 a Diplomacy tag on each card you play; the Protector's Accord (1 a use); the standard project | Dispatching onto pieces (they stay, [ambassadors.md](ambassadors.md)); Remove agents, liberating sites (consumed) |
| Dispatches (not held) | Each round, your board's printed Dispatches plus its uncovered Dispatch spaces ([ai-boards.md](ai-boards.md), *Dispatches*) | The Dispatch action, 1 each; what you don't use this round is lost, as insight is |

**Cities' energy.** Each city makes `corp_energy` energy a round (1 for every corp; the list can
set one a corp). **No player owns a city** (254, Matt's candidate, `cities_unowned` 1). **Every city pays everyone beside it** (r139,
Matt 2026-11-19): in the income step each city that is neither corrupted nor blocked pays its energy to **every**
player with a node on or next to it, free, however many players that is and however many cities a player stands
beside. There is no host, no limit and nothing to buy. It sits on each player's energy income track, which moves
only when a node is placed or lost or a city changes state (corrupted, liberated, an agent arrives or the last one
leaves, a blackout). **An agent on a city stays, and the city is blocked**: every round it stands there the city
pays no one. One agent blocks a city; `corrupt_at` of them corrupt it, so its neighbors clear agents early.
A corrupted city's go to the adversary (Discord), unless a player ambassador stands on it and blocks them. (With
`cities_unowned` 0, r114: an aligned city, claimed or bound at setup, gave its owner its energy + `aligned_bonus`.)

**Cities' energy, for new players.** No city is ever yours, but every free city with your node on or next to it
gives you 1 energy every round, and gives the same to every other player beside it: sharing a city costs no one
anything. An agent
standing on any city shuts it off until someone removes it, and more of them corrupt the city. Clear agents early. A blackout (Naval Blockade's event
line, the Rolling Blackouts project) stops every city's energy while it lasts; the Blackout threat instead takes 3
energy or 3 cards.

**No limits** (as in Terraforming Mars and Spirit Island). No resource has a holding limit and no
global cap limits production: a runaway pile is fixed at card level, never with a ceiling. Unspent energy stays
energy; drafting sinks for it is the player's job, providing them the designers'. Plays a round are bounded by
cards in hand, never by a play limit: **draw is a precious resource** (*Card draw*, [player-cards.md](player-cards.md)),
and by insight, which credits can't buy: every card that draws prints at least 1 insight to play (Science discounts can take it to 0).

**Each resource is paid in its home domain (principle accepted):** energy for nodes and
Power, credits for the market, chips for facilities, ambassadors for influence, insight for
the turn, tags for powers. A station costs chips alone.

**Standard actions and projects** (`board.act`, in the action phase) are deliberately poorly priced against cards
(locked): a luck hedge and a credit sink, never the better route, each at or under par at the unit
anchors (`design_check.py`'s `standard-over-anchor`); Liberate, the win route, is exempt. No standard action spends
antimatter: it is the majors' fuel alone. Cards are the engine and beat standard actions; a standard action is a
fallback, rare unless a build specializes in it (Matt, 2026-11-16, locked). Keeping your things safe is Fortify's, Patrol's and cards', not standard actions'. **Lift Fallout** (Matt: "costly but not
absurd") is the one standard action that removes Fallout; a corrupted site's is Liberate (cards answer both too). Gaining an ambassador is dear on purpose: cards, buildings and powers do it better.

<!-- generated: standard_actions -->
| Action | Cost | What |
|---|---|---|
| Build a research station | `station_chips` 2 chips | Target: an open hex. A research station goes on it (not an industrial zone or Fallout); it is yours (your cube) |
| Remove agents | `remove_ambassadors` 2 of your ambassadors, from your supply, used up | Target: Agents. Remove up to `remove_count` 3 agents on or next to it, your choice |
| Remove a troop | `troop_credits` 7 credits | Target: Troops. Remove 1 troop on it, at most `troop_per_round` 1 a round |
| Lift Fallout | `land_cubes` 2 research cubes | Target: Fallout. Its Fallout token comes off: the one standard action that removes Fallout |
| Dispatch a player ambassador | 1 of your Dispatches this round (your board's, unused ones lost) | Target: a piece or a Botnet. A player ambassador goes on it, from your supply or anywhere on the board (not the target itself) |
| Gain a player ambassador | `ambassador_credits` 16 credits | into your supply; a standard project too |
| Liberate a site | `liberate_at` 5 ambassadors for a city, `claim_station` 4 for a station, `claim_factory` 3 for a factory; used up: yours on the site first, your supply the rest | Target: an adversary site with no Agent on it. An adversary city, station or factory: the adversary's cube comes off, it is left neutral (a card may claim it from there), and it sways Unity (`alignment.free_city` 5 for a city, `free_facility` 2 for a station or factory) |
| Take down a botnet | `botnet_credits` 13 credits | Target: a Botnet on a hex holding your ambassador. The botnet comes off (your ambassador stays) |
| Sell a card | gain `sell_credits` 1 | from your hand |
| Turn a cube into insight | `insight_cubes` 1 research cubes | gain `cube_insight` 1 insight, at most `insight_per_round` 1 a round |
| Make antimatter | `antimatter_cubes` 5 research cubes | gain 1 antimatter and `antimatter_energy` 2 energy, as often as you have cubes (a desperate trade: the engines make it cheaper) |
<!-- /generated -->

Build a factory (`factory_build_chips`, 0: off) is a screened candidate; the event cards' Grow lines build neutral factories, which cards claim. The Claim action is cut (262, Matt, 2026-11-17): cards claim.

## The round

| # | Phase (config `round`) | What happens |
|---|---|---|
| | *(round start)* | Insight is set; abilities and powers refresh |
| 1 | `alignment.sway` | Discord: the adversary board's printed drip (the Militarist's is 0), and `blight_discord` 1 per Fallout token |
| 2 | `adversary.event` | The adversary's Card beat ([adversary.md](adversary.md)): the face-up event card becomes this round's |
| 3 | `threats.deal` | Its threats: the next card of the tier's deck for each country, its tokens on the board ([threats.md](threats.md)) |
| 4 | `adversary.build` | The card's Infiltrate X, Muster, other lines, Spill and Grow; each arrival checks the hex it lands on (each placement puts all its agents for a hex on, then checks it once; a Spill moves one agent at a time, each move checked as it lands) |
| 5 | `board.march` | The March beat ([troops.md](troops.md)): `troop_steps` steps each, move or destroy (past ambassadors) |
| 6 | `adversary.reveal` | The Income beat: next round's card face up (its Infiltrate and Grow sites shown) |
| 7 | `adversary.compute` | The compute step: each botnet seeds `botnet_seed` 1 agent next to it and adds its compute (the compute even when a cordon stops the seed or it has nowhere to land); then income and the round's banked compute, all at once onto the leftmost project, which resolves as it fills (once; none is ongoing); then the research check (a full track fires once, [adversary.md](adversary.md)); then every firewall, cordon and prohibition token comes off, ending the adversary turn |
| 8–11 | `cards.deal`, `.tier`, `.draft`, `.keep` | Round 1's opening hand, then every round's draft and keeps ([player-cards.md](player-cards.md)) |
| 12 | `cards.play+threats.counter+board.act+ai.power` | Actions, simultaneous: cards, abilities, AI-board powers (nodes come from them), standard actions, countering threats |
| 13 | `board.produce` | The income step (one step with 14, in this order): factories make chips (a player's) or a troop (the adversary's); each free, unblocked city's energy to every player with a node on or next to it, corrupted cities' Discord; stations' cubes (an adversary station's research cube, once its megaproject is complete, waits for the compute step); full megaprojects complete. One opposing ambassador or agent blocks a site. The table reads it from the tracks and blocked chips ([troops.md](troops.md)) |
| 14 | `economy.upkeep` | The income step, continued: board and card income; building upkeep |
| 15 | `threats.execute` | Partial payments refunded; uncountered threats execute (each also lands 1 agent on its anchor, [threats.md](threats.md)) |
| 16 | `board.cleanup` | Round flags removed; tips and the end checks run after every phase |

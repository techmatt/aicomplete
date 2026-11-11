# Resources and the round

*Each resource's sources and uses, city energy, the standard actions and the round's phases. Terms and conventions are in [README.md](README.md).*

| Resource | Comes from | Spent on |
|---|---|---|
| Credits | Credit production on the AI board ([ai-boards.md](ai-boards.md)), the node track, cards and Unity cards, in upkeep; megaproject payouts; selling a card (`sell_credits` 1) | Card costs; keeps beyond the opening's free ones (`opening_keep_credits` each); insight for keeps (`insight_credits` each); neutral cities' energy; standard projects and Clear; the Builder's Buy In; threat counters |
| Energy (keeps between rounds; candidate: about 1-3 held mid-game, near 0 at the end) | Aligned cities' energy in the income step (*The round* below); neutral cities' bought with Buy energy; Horizon's completion payout; the node tracks; Power programs (106: energy income lives on programs, +2 at most on any card: Baseload Contract; Tidal Barrage per 2 Power tags); the Power engines' when-built 2 (Substation Hub, Dispatch Control Room, Smart Grid Backbone; 108) and Power triggers (Positron Storage Ring's 2 a Power card); the Builder's Overcharge; each board's printed energy (1, the Protector's 2; 142); `ai.start_energy` at the start | The node powers and every track step ([ai-boards.md](ai-boards.md)); the dear powers (Rush Build, Shelter, Particle Run); cards' `energy` cost (power-hungry cards, counted in [player-cards.md](player-cards.md)); the Actions of Grid Analytics Desk, Particle Accelerator, Directed-Energy Array, Holographic Assembly Hall, Wafer Fab Line and Microgrid Lab Compact; threat counters that offer energy (by tag, [threats.md](threats.md) *Counters*); Bulwark's hardening |
| Chips | Player-owned factories each round (`factory_chips` each); Onyx's converter; the Builder's tracks and its Mint and Scrap; cards (chip income: Peaker Turbines, Arcology Foundry, Orbital Mass Driver, Fabrication Megaplex; Antitrust Action's chips per 2 Industry tags; Joint Fabrication Pact) | Some buildings' `chips` cost; draw 2 cards: the Builder's Procure (2 chips) and Parts Exchange's Action (1 chip); research stations (`station_chips`); the Harden action a card grants (`harden_chips`; no card does). Chips can't be sold (149) |
| Antimatter | Cards (Positron Storage Ring, Zero-Day Auction, Intrusion Lab's threshold); the antimatter action; Horizon's converter; the Researcher's Particle Run | Tier-2 cards |
| Research cubes (keep between rounds) | Your stations, once their country's megaproject is complete; the Researcher's Open Archive capstone | The cube converter (a standard action: 1 cube for `cube_insight` insight, no limit a round); the corps' converters ([board.md](board.md)); the Researcher's Peer Review ([ai-boards.md](ai-boards.md)); Forensic Audit |
| Insight | Each round, set to the AI board's insight (5 printed, [ai-boards.md](ai-boards.md)) plus the node track's uncovers, buildings and programs' insight production (Endowed Research Chair, Frontier Lab Charter, Deep State Ledger, Recursive Self-Improvement) and the Unity card Think Tank's 1; less what Smear Campaign and Data Heist took. Mid-round: the cube converter (a standard action, 1 cube for `cube_insight`) (no limit a round) and Cintamani's (1 cube for 2, once a round, with a Cintamani city in reach); each board's conversion (Focus, Vigil, Synthesis: [ai-powers.md](ai-powers.md)); card triggers (Interdisciplinary Institute, Telemetry Archive, Security Ops Floor); the Researcher's Open Science capstone gives others 2 a round | Keeping drafted cards; playing cards; threat counters that offer insight (8 / 10 / 12 by tier, by tag: [threats.md](threats.md) *Counters*). It passes between players only through cards that print it (help cards such as Shared Compute Grant and Mutual Aid Pact, at their cost; r48: no standard action gives it). It expires at the round's end, but the Researcher's own carries over: Patience, [ai-boards.md](ai-boards.md) |
| Ambassadors | `start_ambassadors` at the start; Diplomacy cards (Consular Pouch, Peace Conference), Followers through a card's Action, Orbital Embassy's 1 a Diplomacy tag on each card you play; the standard project | Moving onto pieces (they stay, [ambassadors.md](ambassadors.md)); Remove agents, claiming and liberating sites, Envoy (consumed) |
| Movement (not held) | Each round, your board's printed Movement plus its uncovered Movement spaces ([ai-boards.md](ai-boards.md), *Movement*) | The Move action, 1 a move; what you don't use this round is lost, as insight is |

**Cities' energy.** Each city makes `corp_energy` (a megacorp parameter, more for a richer
corp) energy a round. An aligned city's goes free to its owner in the income step (+
`aligned_bonus`). **A neutral city's is bought in the action phase** (r75, Matt's ruling 6; it was sold in
production): the Buy energy standard action, `city_energy_credits` (4) credits for its energy, a neutral city in
reach that is producing (no blackout, no agent on it), once per city a round (any one player). The price is 4, not
the ruled 5: at 5 credits an energy (4.5 at `values.json` and the unit anchor) the buy was a loss. **An agent on a
city stays, and the city is dark** (Matt: neutral cities at r37, aligned ones at r38): every round it stands there, a
neutral city has nothing to buy and an aligned city gives its owner nothing. One agent makes a
city dark; `corrupt_at` of them corrupt it, so clearing agents from the cities you buy
from, and above all from the cities you aligned, comes well before corruption. **Aligning** a city is the Claim
action (137: `claim_city` ambassadors, plus one for each agent on it, which cancel;
[ambassadors.md](ambassadors.md)). A corrupted city's go to the adversary (Discord), unless a player ambassador
stands on it and blocks them.

**Cities' energy, for new players.** A city you aligned gives you its energy every round.
A neutral city in your reach sells you its energy in the action phase, 4 credits, once a round. An agent standing on
any city shuts it off: your aligned city gives you nothing, a neutral one sells nothing,
every round until someone removes the agent, and more of them corrupt the city. Claiming
a dark city costs one more ambassador for each agent on it. If
agents get that close to your city, work to stop them: clear them early. A Blackout stops a country's cities
in the production after the threat executes.

**No limits** (Matt, 2026-11-05, as in Terraforming Mars and Spirit Island). No resource has a holding limit and no
global cap limits production. A runaway pile means a runaway engine, fixed at card level (its numbers, its Action, a
redesign), never with a ceiling. Unspent energy stays energy: nothing converts it at round end. Finding and drafting
sinks for your energy is the player's job; providing them is the designers' (energy costs on power-hungry cards,
the Actions of Grid Analytics Desk, Particle Accelerator, Directed-Energy Array, Holographic Assembly Hall, Wafer
Fab Line and Microgrid Lab Compact, the tier-2 Citywide Microgrid and Exascale Training Cluster). Plays a round are bounded by
insight, which credits can't buy: every card that draws costs at least 1 insight to play.

**Each resource is paid in its home domain (principle accepted):** energy for nodes and
Power, credits for the market, chips for facilities, ambassadors for influence, insight for
the turn, tags for powers. A research station costs chips alone.

**Standard actions and projects** (`board.act`, in the action phase) are deliberately poorly
priced: cards are always the better way, and they are a luck hedge. **The gain-an-ambassador
project stays dear on purpose (Matt: 20 credits; overwhelming the adversary with bought
ambassadors takes a big credit investment, and cards, buildings and powers are the better
route):** so ambassadors come from Diplomacy cards, Followers and buildings, and Remove agents spends one to take up
to `remove_count` agents in reach. The **standard projects** (candidate, Matt) are the
credit sink, as in Terraforming Mars: poor but useful rates, no limit a round. Every map
target is in the player's reach, but Take down a botnet's, where the player's ambassador stands on it. The view publishes each action with its cost
(`View.standard`), so clients do not repeat the parameters.

<!-- generated: standard_actions -->
| Action | Cost | What |
|---|---|---|
| Build a research station | `station_chips` 2 chips | on an open hex in reach that is not an industrial zone; it is yours (your cube) |
| Remove agents | `remove_ambassadors` 1 of your ambassadors, from your supply, used up | remove up to `remove_count` 3 agents in reach, from any hexes, your choice |
| Clear a corruption marker | `clear_cost` 2 credits | in reach: sway `clear_unity` 2 toward Unity, gain `clear_credits` 0 credits |
| Move a player ambassador | 1 of your Movement this round (your board's, unused ones lost) | from your supply or the board onto a piece in reach (not the hex it comes from) |
| Gain a player ambassador | `ambassador_credits` 20 credits | into your supply; a standard project too |
| Claim a site | `claim_city` 5 ambassadors for a city, `claim_station` 3 for a station, `claim_factory` 3 for a factory, 1 more for each agent on it; used up: yours on the site first, your supply the rest | a neutral city, research station or factory in reach (no embargo on it): the agents on it cancel and your cube goes on |
| Liberate a site | `liberate_at` 5 ambassadors for a city, `liberate_facility_at` 3 for a station or factory, 1 more for each agent on it; used up: yours on the site first, your supply the rest | an adversary city, station or factory in reach: the agents on it cancel, your cube goes on, and it sways Unity (`alignment.free_city` 5 for a city, `free_facility` 2 for a station or factory) |
| Take down a botnet | `botnet_credits` 5 credits | a botnet on a hex holding your ambassador (it stays) comes off: sway `alignment.botnet` 1 toward Unity |
| Buy a city's energy | `city_energy_credits` 4 credits | a neutral city in reach that is producing (no blackout, no agent on it), once per city a round: gain its energy |
| Harden a facility (a card that grants it; none does: Faraday Yard hardens when played) | the chips the card prints, per token | a player-owned station or factory in reach that is not corrupted, one a troop can strike |
| Sell a card | gain `sell_credits` 1 | from your hand |
| Project: harden (`harden_buy`) | `harden_credits` 3 credits | any player: a hardening token on a player-owned station or factory in reach |
| Project: drain (`drain`) | `drain_credits` 5 credits | remove up to `drain_amount` 3 compute from one visible adversary project |
| Turn a cube into insight | 1 research cube | gain `cube_insight` 1 insight, as often as you have cubes |
| Envoy | `envoy_ambassadors` 1 of your ambassadors, from your supply, used up | move 1 Discord off the Discord track |
<!-- /generated -->

There is no "build a factory" standard action: cities build factories.

## The round

| # | Phase (config `round`) | What happens |
|---|---|---|
| | *(round start)* | Insight is set; abilities and powers refresh |
| 1 | `alignment.sway` | AI boards sway |
| 2 | `adversary.event` | The face-up event card becomes this round's ([adversary.md](adversary.md)) |
| 3 | `threats.deal` | Its pulls: a threat a pull in each country, its tokens on the board ([threats.md](threats.md)) |
| 4–6 | `adversary.build`, `.corrupt`, `.natural` | The card's Rally X (each board's pool lands), Deploy and other lines, the board's Deploy, ongoing projects; the card's Corrupt lines; natural corruption (sites, then node hexes) |
| 7 | `board.march` | The adversary troops march ([war.md](war.md)): `troop_steps` steps each, remove an ambassador, move or destroy |
| 8 | `adversary.reveal` | Next round's card face up; the adversary's income (agents into each board's pool), then the compute step: income, botnets, corrupted stations and the round's banked compute onto the projects, which resolve as they fill (175) |
| 9–12 | `cards.tier`, `.deal`, `.draft`, `.keep` | Round 1: the opening deal and keep. Later: build the draft hand card by card (tier each), draft, keep ([player-cards.md](player-cards.md)) |
| 13 | `cards.play+threats.counter+alignment.act+board.act+ai.power` | Actions, simultaneous: cards, abilities, AI-board powers (nodes come from them), standard actions, countering threats, antimatter |
| 14 | `board.produce` | The income step (r75, Matt's ruling 6: one step in this printed order, with 15): factories make chips (a player's) or a troop (the adversary's); aligned cities' energy to their owners, corrupted cities' Discord; stations' cubes (an adversary station's compute waits for the compute step); full megaprojects complete. One opposing ambassador or agent blocks a site |
| 15 | `economy.upkeep` | The income step, continued: board and card income; building upkeep |
| 16 | `board.bases` | Growth: extra corp troops, then each city grows a factory on a free zone next to it ([war.md](war.md)) |
| 17 | `threats.execute` | Partial payments refunded; uncountered threats execute |
| 18 | `board.war` | War, at the end of every round: front battles; what it destroys is gone for next round's production |
| 19 | `board.cleanup` | Round flags removed; tips and the end checks run after every phase |

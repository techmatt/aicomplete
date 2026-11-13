# Resources and the round

*Each resource's sources and uses, city energy, the standard actions and the round's phases. Terms and conventions are in [README.md](README.md).*

| Resource | Comes from | Spent on |
|---|---|---|
| Credits | Credit production on the AI board ([ai-boards.md](ai-boards.md)), the node track, cards and Unity cards, in upkeep; megaproject payouts; selling a card (`sell_credits` 1) | Card costs (never keeps, r82); neutral cities' energy; the standard actions (Place a node, Remove a troop, Take down a botnet, Buy energy); the Builder's Buy In; threat counters |
| Energy (keeps between rounds; candidate: about 1-3 held mid-game, near 0 at the end) | Aligned cities' energy in the income step (*The round* below); neutral cities' bought with Buy energy; Horizon's completion payout; the node tracks; Power programs (106: energy income lives on programs, +2 at most on any card), the Power engines' when-built 2 and Power triggers ([cards.md](../cards.md)); the Builder's Overcharge; each board's printed energy (1, the Protector's 2; 142); `ai.start_energy` at the start | Every track step, whatever places the node ([ai-boards.md](ai-boards.md)); the dear powers (Rush Build, Shelter, Particle Run); cards' `energy` cost (power-hungry cards, counted in [player-cards.md](player-cards.md)); building Actions ([cards.md](../cards.md)); threat counters that offer energy (by tag, [threats.md](threats.md) *Counters*); Bulwark's hardening; Interdict |
| Chips (4 credits at `values.json`, 6 at the anchors, r83) | Player-owned factories each round (`factory_chips` each); Onyx's converter; the Builder's tracks and its Mint and Scrap; cards (chip income: Peaker Turbines, Arcology Foundry, Orbital Mass Driver, Fabrication Megaplex; once, when played: Joint Fabrication Pact, Antitrust Action's 1 per 4 Industry tags). A city's own chips (growth, [war.md](war.md)) are no player's | Some buildings' `chips` cost; the Builder's Procure, Fortify and Refine; Parts Exchange's Action; research stations (`station_chips`); the Harden action a card grants (`harden_chips`; no card does); the corps' chip converters, 2 a use (r87, [board.md](board.md)); Antiproton Foundry's Action. Every chip price is 1. Chips can't be sold (149) |
| Antimatter (r90: earned, 18 at the anchors; r94: the majors' fuel) | Each board's power, once a round, each costly (r94): the Builder's Refine (2 chips + 1 energy), the Protector's Joint Facility (12 credits + 1 energy), the Researcher's Particle Run (1 Power + 1 energy); the plants (Penning Trap Array, Antiproton Foundry: a cube or a chip and an energy into 2; Positron Storage Ring's 1 a round; tier 1: Cyclotron Lease, 2 energy for 1); Horizon's converter (2 cubes + 1 energy for 1); Sanctuary, Open Archive; Zero-Day Auction, Intrusion Lab. The desperate standard action: `antimatter_cubes` (4) research cubes for 1 | The majors (r94; Breakthrough is gone): 14 tier-2 cards at 1 (the liberations, troop strikes, Annihilation Lance since r95, Antimatter Accord, Collider Consortium), Fusion Commons at 2; tier 1: Accelerator Diplomacy, Containment Reactor ([cards.md](../cards.md)) |
| Research cubes (keep between rounds) | Your stations, once their country's megaproject is complete; Cryo Sample Vault's 1 a round (tier 1, r88); the Researcher's Open Archive capstone | The cube converter (a standard action: 1 cube for `cube_insight` insight, no limit a round); Make antimatter and Lift land corruption (standard actions, below); the corps' converters ([board.md](board.md): Horizon's makes antimatter, Biomia's 1 Unity a cube, r88); Penning Trap Array's and Field Assay Team's Actions; the Researcher's Peer Review ([ai-boards.md](ai-boards.md)); Forensic Audit |
| Insight | Each round, set to the card-play track's total (3 at its start, up to 10, 9 or 8 by board: [ai-boards.md](ai-boards.md)) plus buildings' and programs' insight production and Think Tank's 1; no adversary card takes it. Mid-round: the cube converter (1 cube for `cube_insight`, no limit) and Cintamani's (1 cube for 2, once a round, a Cintamani city in reach); Vigil and Peer Review ([ai-powers.md](ai-powers.md)); card triggers; the Researcher's Open Science (others 2 a round) | Keeping drafted cards; playing cards; threat counters that offer it ([threats.md](threats.md) *Counters*). Only cards that print it pass it between players. It expires at the round's end, but the Researcher's carries over (Patience, [ai-boards.md](ai-boards.md)) |
| Ambassadors | `start_ambassadors` at the start; Diplomacy cards (Consular Pouch, Peace Conference), Followers through a card's Action, Orbital Embassy's 1 a Diplomacy tag on each card you play; the standard project | Dispatching onto pieces (they stay, [ambassadors.md](ambassadors.md)); Remove agents, claiming and liberating sites, Envoy (consumed) |
| Dispatches (not held) | Each round, your board's printed Dispatches plus its uncovered Dispatch spaces ([ai-boards.md](ai-boards.md), *Dispatches*) | The Dispatch action, 1 each; what you don't use this round is lost, as insight is |

**Cities' energy.** Each city makes `corp_energy` (a megacorp parameter, more for a richer
corp) energy a round. An aligned city's goes free to its owner in the income step (+
`aligned_bonus`). **A neutral city's is bought in the action phase** (r75, Matt's ruling 6): the Buy energy standard action (the
table below). **An agent on a city stays, and the city is dark**: every round it stands there, a
neutral city has nothing to buy and an aligned city gives its owner nothing. One agent makes a
city dark; `corrupt_at` of them corrupt it, so clearing agents from the cities you buy
from, and above all from the cities you aligned, comes well before corruption. **Aligning** a city is the Claim
action ([ambassadors.md](ambassadors.md)). A corrupted city's go to the adversary (Discord), unless a player ambassador
stands on it and blocks them.

**Cities' energy, for new players.** A city you aligned gives you its energy every round.
A neutral city in your reach sells you its energy in the action phase, 5 credits, once a round. An agent standing on
any city shuts it off: your aligned city gives you nothing, a neutral one sells nothing,
every round until someone removes the agent, and more of them corrupt the city. A
dark city can't be claimed until its agents are gone. If
agents get that close to your city, work to stop them: clear them early. A blackout (Naval Blockade's event
line, the Rolling Blackouts project) stops every city's energy while it lasts; the Blackout threat instead takes 3
energy or 3 cards.

**No limits** (Matt, 2026-11-05, as in Terraforming Mars and Spirit Island). No resource has a holding limit and no
global cap limits production: a runaway pile is fixed at card level, never with a ceiling. Unspent energy stays
energy; drafting sinks for it is the player's job, providing them the designers'. Plays a round are bounded by
insight, which credits can't buy: every card that draws costs at least 1 insight to play.

**Each resource is paid in its home domain (principle accepted):** energy for nodes and
Power, credits for the market, chips for facilities, ambassadors for influence, insight for
the turn, tags for powers. A research station costs chips alone.

**Standard actions and projects** (`board.act`, in the action phase) are deliberately poorly priced against cards
(locked, Matt, 2026-11-12): a luck hedge and a credit sink, never the better route, each at or under par at the unit
anchors (`design_check.py`'s `standard-over-anchor`); Liberate, the win route, is exempt. r91 cut Clear and the drain and harden projects (keeping your
things safe is Bulwark's, Shelter's, Fortify's and cards'), and made node placement one standard action (Place a node)
in place of Expand on every board. **Lift land corruption** (r92, candidate, Matt: "costly but not absurd") is the one
standard action that removes corruption: `land_cubes` research cubes take one land-corruption token in reach off.
Corruption markers and corrupted sites have no standard action (cards, Recovery and Liberate answer them). Remove agents is unchanged (Matt: fine as is) and gaining an ambassador
dear on purpose (Matt: cards, buildings and powers are the better route). The standard projects are the credit sink, as in Terraforming Mars; Place a node is limited to
`node_per_round` a round, Remove a troop to `troop_per_round`. Every target is in the player's reach. The view
publishes each action with its cost (`View.standard`).

<!-- generated: standard_actions -->
| Action | Cost | What |
|---|---|---|
| Place a node | `node_credits` 6 credits, and the track step's energy | place 1 node next to yours, off either track, up to `node_per_round` 2 times a round (the boards' own node powers are the better route) |
| Build a research station | `station_chips` 1 chips (and `station_ambassadors` 0 ambassadors, used up: a candidate, 0 none) | on an open hex in reach that is not an industrial zone or corrupted land; it is yours (your cube) |
| Remove agents | `remove_ambassadors` 1 of your ambassadors, from your supply, used up | remove up to `remove_count` 3 agents in reach, from any hexes, your choice |
| Remove a troop | `troop_credits` 10 credits | remove 1 adversary troop in reach, up to `troop_per_round` 1 times a round |
| Lift land corruption | `land_cubes` 2 research cubes | a land-corruption token in reach, face up or face down, comes off: the one standard action that removes corruption |
| Dispatch a player ambassador | 1 of your Dispatches this round (your board's, unused ones lost) | from your supply or the board onto a piece or botnet in reach (not the hex it comes from) |
| Gain a player ambassador | `ambassador_credits` 20 credits | into your supply; a standard project too |
| Claim a site | `claim_city` 5 ambassadors for a city, `claim_station` 3 for a station, `claim_factory` 2 for a factory (Liberate's table too); used up: yours on the site first, your supply the rest | a neutral city, research station or factory in reach with no agent on it (no embargo on it): your cube goes on |
| Liberate a site | Claim's: `liberate_at` 5 ambassadors for a city, `claim_station` 3 for a station, `claim_factory` 2 for a factory; used up: yours on the site first, your supply the rest | an adversary city, station or factory in reach with no agent on it: the adversary's cube comes off, yours goes on, and it sways Unity (`alignment.free_city` 5 for a city, `free_facility` 2 for a station or factory) |
| Take down a botnet | `botnet_credits` 16 credits | a botnet on a hex holding your ambassador (it stays) comes off: sway `alignment.botnet` 0 toward Unity |
| Buy a city's energy | `city_energy_credits` 5 credits | a neutral city in reach that is producing (no blackout, no agent on it), once per city a round: gain its energy |
| Harden a facility (a card that grants it; none does: Faraday Yard hardens when played) | the chips the card prints, per token | a player-owned station or factory in reach that is not corrupted, one a troop can strike |
| Sell a card | gain `sell_credits` 1 | from your hand |
| Turn a cube into insight | 1 research cube | gain `cube_insight` 1 insight, as often as you have cubes |
| Envoy | `envoy_ambassadors` 1 of your ambassadors, from your supply, used up | move 1 Discord off the Discord track |
| Make antimatter | `antimatter_cubes` 4 research cubes | gain 1 antimatter, as often as you have cubes (a desperate trade: the engines make it cheaper) |
<!-- /generated -->

Build a factory (`factory_build_chips`, 0: off) is a screened candidate (amendment 9212); cities build neutral factories, which players Claim.

## The round

| # | Phase (config `round`) | What happens |
|---|---|---|
| | *(round start)* | Insight is set; abilities and powers refresh |
| 1 | `alignment.sway` | Discord: the adversary board's printed drip (the Militarist's is 0), and `blight_discord` 1 per face-up land corruption token (a node hex's goes face down: `blight_node_discord` 0, r84) |
| 2 | `adversary.event` | The adversary's Card beat ([adversary.md](adversary.md)): the face-up event card becomes this round's |
| 3 | `threats.deal` | Its pulls: a threat a pull in each country, its tokens on the board ([threats.md](threats.md)) |
| 4 | `adversary.build` | The card's Rally X (each country's pool dealt round-robin), Muster and other lines; the Protector's Muster |
| 5–6 | `adversary.corrupt`, `.natural` | The Corrupt beat: the card's Corrupt lines; natural corruption (sites, then node hexes) |
| 7 | `board.march` | The March beat ([war.md](war.md)): `troop_steps` steps each, move or destroy (r83: past ambassadors) |
| 8 | `adversary.reveal` | The Income beat: next round's card face up; agents into each country's pool |
| 9 | `adversary.compute` | The compute step: income, botnets, corrupted stations and the round's banked compute onto the leftmost project, which resolves as it fills (175); ongoing projects act; then every firewall, cordon and prohibition token comes off, ending the adversary turn |
| 10–13 | `cards.tier`, `.deal`, `.draft`, `.keep` | Round 1 first deals the opening hand, kept free (r82). Every round, round 1 included: build the draft hand card by card (tier each), draft (always passing to the next seat), keep ([player-cards.md](player-cards.md)) |
| 14 | `cards.play+threats.counter+board.act+ai.power` | Actions, simultaneous: cards, abilities, AI-board powers (nodes come from them), standard actions, countering threats |
| 15 | `board.produce` | The income step (one step with 16, in this order): factories make chips (a player's) or a troop (the adversary's); aligned cities' energy to their owners, corrupted cities' Discord; stations' cubes (an adversary station's compute waits for the compute step); full megaprojects complete. One opposing ambassador or agent blocks a site |
| 16 | `economy.upkeep` | The income step, continued: board and card income; building upkeep |
| 17 | `board.bases` | Growth: extra drones, then city growth: each city gathers a chip from each unclaimed neutral factory next to it, and at 3 builds a neutral factory or station ([war.md](war.md)) |
| 18 | `threats.execute` | Partial payments refunded; uncountered threats execute |
| 19 | `board.war` | War, at the end of every round: front battles; what it destroys is gone for next round's production |
| 20 | `board.cleanup` | Round flags removed; tips and the end checks run after every phase |

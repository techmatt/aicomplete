# Resources and the round

*Terms and conventions are in [README.md](README.md).*

| Resource | Comes from | Spent on |
|---|---|---|
| Credits | Credit production on the AI board ([ai-boards.md](ai-boards.md)), the node track, cards and Unity cards, in upkeep; megaproject payouts; selling a card (`sell_credits` 1) | Card costs (never keeps); standard actions (table below); the Builder's Mint, the Researcher's Grant; threat counters |
| Energy (keeps between rounds; candidate: about 1-3 held mid-game, near 0 at the end) | Hosted cities' energy in the income step (a neutral city with your node on or next to it, *Cities' energy* below); other neutral cities' bought with Buy energy; Horizon's completion payout; the node tracks; Power programs (+2 at most on any other card), the Power engines' when-built 2 and Power triggers ([cards.md](../cards.md)); the Builder's Overcharge; each board's printed energy; `ai.start_energy` at the start | **Energy powers things** (Matt, 2026-11-15): every track step, whatever places the node ([ai-boards.md](ai-boards.md)); the board powers: Buy In and Accord (2 each), Bulwark and Patrol (1), the dear Rush Build and Reclaim ([ai-powers.md](ai-powers.md)); building Actions, most at 1-2 ([cards.md](../cards.md)); cards' `energy` cost (power-hungry cards, counted in [player-cards.md](player-cards.md)); threat counters that offer energy (by tag, [threats.md](threats.md) *Counters*) |
| Chips (4 credits at `values.json`, 6 at the anchors) | Player-owned factories each round (`factory_chips` each); Onyx's converter; the Builder's tracks and its Mint and Scrap; cards (chip income on four, a one-time gain on two: [cards.md](../cards.md)). A city's own chips (growth, [war.md](war.md)) are no player's | Some buildings' `chips` cost; the Builder's Procure, Fortify and Teardown; Parts Exchange's Action; research stations (`station_chips`); the corps' chip converters, 2 a use ([board.md](board.md)); Antiproton Foundry's Action. Every chip price is 1 but a station's 2. Chips can't be sold |
| Antimatter (18 at the anchors; the majors' fuel) | The plants, the steady source: a building's Action, once a round, 1 for a cube, a chip, 2 energy, 2 insight or 8 credits (each its own plant; tier 1: 2 energy; some pay a second
thing beside it: Antiproton Foundry 2 energy, Positron Storage Ring a card, Treaty Isotope Bank an ambassador). Each board's rare burn ([ai-powers.md](ai-powers.md)): Teardown, Meltdown, Reclaim. Horizon's converters (3 cubes + 1 energy, or 2 chips); Sanctuary, Open Archive; Zero-Day Auction, Intrusion Lab. The desperate Make antimatter (below) | The majors: 14 tier-2 cards at 1 (liberations, troop strikes and more), one at 2; Orbital Relay Launcher's Action (1 for 3 nodes and 3 energy, once a round); two tier-1 cards ([cards.md](../cards.md)) |
| Research cubes (keep between rounds) | Your stations, once their country's megaproject is complete; Cryo Sample Vault's 1 a round (tier 1); the Researcher's Open Archive capstone | The standard actions Turn a cube into insight, Make antimatter and Lift Fallout (below); the corps' converters ([board.md](board.md): Horizon's makes antimatter, Biomia's 1 Unity a cube); Penning Trap Array's, Field Assay Team's, Research Reactor's (3 energy) and Survey Drone Fleet's (2 nodes and 2 energy) Actions; the Researcher's Disrupt and Meltdown ([ai-boards.md](ai-boards.md)); Forensic Audit |
| Insight | Each round, set to the card-play track's total (3 at its start; each board's top: [ai-boards.md](ai-boards.md)) plus buildings' and programs' insight production and Think Tank's 1; no adversary card takes it. Mid-round: the cube converter (standard action) and Cintamani's (1 cube for 2, once a round, a Cintamani city in reach); Vigil and Peer Review ([ai-powers.md](ai-powers.md)); card triggers; the Researcher's Open Science (others 2 a round) | Keeping drafted cards; playing cards; threat counters that offer it ([threats.md](threats.md) *Counters*); Dark Fiber Collider's Action; the Researcher's node routes (Peer Review, Field Survey) and Synthesis. Only cards that print it pass it between players. It expires at the round's end, but the Researcher's carries over (Patience, [ai-boards.md](ai-boards.md)) |
| Ambassadors | `start_ambassadors` at the start; Diplomacy cards, Followers through a card's Action, Orbital Embassy's 1 a Diplomacy tag on each card you play; the standard project | Dispatching onto pieces (they stay, [ambassadors.md](ambassadors.md)); Remove agents, claiming and liberating sites, Envoy (consumed) |
| Dispatches (not held) | Each round, your board's printed Dispatches plus its uncovered Dispatch spaces ([ai-boards.md](ai-boards.md), *Dispatches*) | The Dispatch action, 1 each; what you don't use this round is lost, as insight is |

**Cities' energy.** Each city makes `corp_energy` (a megacorp parameter, more for a richer
corp) energy a round. **No player owns a city** (254, Matt's candidate, `cities_unowned` 1). **Hosting** (254,
`host_city` 1, `host_reach` 1): in the income step a neutral city with your node on or next to it makes its energy
for you, free, unless its energy was bought this round. Each player hosts `host_city` city at most (258: one more for
each `host_more` card they played, Twin City Compact) and each city has one host; the players choose who hosts what (the engine hosts as many as can be: a matching). **A neutral city's
energy may still be bought in the action phase** with the Buy energy standard action, by anyone with it in reach;
bought, it hosts no one that round. **An agent on a city stays, and the city is dark**: every round it stands there,
a neutral city has nothing to buy or to host. One agent makes a city dark; `corrupt_at` of them corrupt it, so a
host clears agents early.
A corrupted city's go to the adversary (Discord), unless a player ambassador stands on it and blocks them. (With
`cities_unowned` 0, r114: an aligned city, claimed or bound at setup, gave its owner its energy + `aligned_bonus`.)

**Cities' energy, for new players.** No city is ever yours, but a free city with your node on or next to it gives
you its energy every round (one city each, two with Twin City Compact; one player a city). A free city in your reach also sells its energy in
the action phase, 5 credits, once a round; a city sold that round gives its host nothing. An agent
standing on any city shuts it off until someone removes it, and more of them corrupt the city. Clear agents early. A blackout (Naval Blockade's event
line, the Rolling Blackouts project) stops every city's energy while it lasts; the Blackout threat instead takes 3
energy or 3 cards.

**No limits** (as in Terraforming Mars and Spirit Island). No resource has a holding limit and no
global cap limits production: a runaway pile is fixed at card level, never with a ceiling. Unspent energy stays
energy; drafting sinks for it is the player's job, providing them the designers'. Plays a round are bounded by
insight, which credits can't buy: every card that draws costs at least 1 insight to play.

**Each resource is paid in its home domain (principle accepted):** energy for nodes and
Power, credits for the market, chips for facilities, ambassadors for influence, insight for
the turn, tags for powers. A station costs chips alone.

**Standard actions and projects** (`board.act`, in the action phase) are deliberately poorly priced against cards
(locked): a luck hedge and a credit sink, never the better route, each at or under par at the unit
anchors (`design_check.py`'s `standard-over-anchor`); Liberate, the win route, is exempt. No standard action spends
antimatter: it is the majors' fuel alone. Cards are the engine and beat standard actions; a standard action is a
fallback, rare unless a build specializes in it (Matt, 2026-11-16, locked). Keeping your things safe is Bulwark's, Fortify's and cards', not standard actions'. **Lift Fallout** (Matt: "costly but not
absurd") alone removes corruption; corrupted sites have none (cards, Reclaim and Liberate answer them). Gaining an ambassador is dear on purpose: cards, buildings and powers do it better.

<!-- generated: standard_actions -->
| Action | Cost | What |
|---|---|---|
| Build a research station | `station_chips` 2 chips | Target: an open hex. A research station goes on it (not an industrial zone or Fallout); it is yours (your cube) |
| Remove agents | `remove_ambassadors` 2 of your ambassadors, from your supply, used up | Target: Agents. Remove up to `remove_count` 3 agents on or next to it, your choice |
| Remove a troop | `troop_credits` 10 credits | Target: Troops. Remove 1 troop on it, at most `troop_per_round` 1 a round |
| Lift Fallout | `land_cubes` 2 research cubes | Target: Fallout. Its Fallout token comes off: the one standard action that removes Fallout |
| Dispatch a player ambassador | 1 of your Dispatches this round (your board's, unused ones lost) | Target: a piece or a Botnet. A player ambassador goes on it, from your supply or anywhere on the board (not the target itself) |
| Gain a player ambassador | `ambassador_credits` 20 credits | into your supply; a standard project too |
| Claim a site | `claim_station` 4 ambassadors for a station, `claim_factory` 3 for a factory (Liberate's table too); used up: yours on the site first, your supply the rest | Target: a neutral site with no Agent on it. A neutral research station or factory, no embargo on it: your cube goes on it (never a city) |
| Liberate a site | Claim's: `liberate_at` 5 ambassadors for a city, `claim_station` 4 for a station, `claim_factory` 3 for a factory; used up: yours on the site first, your supply the rest | Target: an adversary site with no Agent on it. An adversary city, station or factory: the adversary's cube comes off, it is left neutral (Claim or a card takes it from there), and it sways Unity (`alignment.free_city` 5 for a city, `free_facility` 2 for a station or factory) |
| Take down a botnet | `botnet_credits` 20 credits | Target: a Botnet on a hex holding your ambassador. The botnet comes off (your ambassador stays) |
| Buy a city's energy | `city_energy_credits` 5 credits | Target: a producing neutral city. Producing (no blackout, no agent on it): gain its energy, once per city a round |
| Sell a card | gain `sell_credits` 1 | from your hand |
| Turn a cube into insight | `insight_cubes` 1 research cubes | gain `cube_insight` 1 insight, at most `insight_per_round` 1 a round |
| Envoy | `envoy_ambassadors` 1 of your ambassadors, from your supply, used up | move 1 Discord off the Discord track |
| Make antimatter | `antimatter_cubes` 5 research cubes | gain 1 antimatter, as often as you have cubes (a desperate trade: the engines make it cheaper) |
<!-- /generated -->

Build a factory (`factory_build_chips`, 0: off) is a screened candidate; cities build neutral factories, which players Claim.

## The round

| # | Phase (config `round`) | What happens |
|---|---|---|
| | *(round start)* | Insight is set; abilities and powers refresh |
| 1 | `alignment.sway` | Discord: the adversary board's printed drip (the Militarist's is 0), and `blight_discord` 1 per Fallout token |
| 2 | `adversary.event` | The adversary's Card beat ([adversary.md](adversary.md)): the face-up event card becomes this round's |
| 3 | `threats.deal` | Its pulls: a threat a pull in each country, its tokens on the board ([threats.md](threats.md)) |
| 4 | `adversary.build` | The card's Rally X (each country's pool dealt round-robin), Muster and other lines |
| 5–6 | `adversary.corrupt`, `.natural` | The Corrupt beat: the card's Corrupt lines; natural corruption (sites, then node hexes) |
| 7 | `board.march` | The March beat ([war.md](war.md)): `troop_steps` steps each, move or destroy (past ambassadors) |
| 8 | `adversary.reveal` | The Income beat: next round's card face up; agents into each country's pool |
| 9 | `adversary.compute` | The compute step: income, botnets, corrupted stations and the round's banked compute onto the leftmost project, which resolves as it fills (once; none is ongoing); then every firewall, cordon and prohibition token comes off, ending the adversary turn |
| 10–13 | `cards.deal`, `.tier`, `.draft`, `.keep` | Round 1's opening hand, then every round's draft and keeps ([player-cards.md](player-cards.md)) |
| 14 | `cards.play+threats.counter+board.act+ai.power` | Actions, simultaneous: cards, abilities, AI-board powers (nodes come from them), standard actions, countering threats |
| 15 | `board.produce` | The income step (one step with 16, in this order): factories make chips (a player's) or a troop (the adversary's); hosted cities' energy to their hosts (254), corrupted cities' Discord; stations' cubes (an adversary station's compute waits for the compute step); full megaprojects complete. One opposing ambassador or agent blocks a site |
| 16 | `economy.upkeep` | The income step, continued: board and card income; building upkeep |
| 17 | `board.bases` | Growth: extra drones, then city growth ([war.md](war.md)) |
| 18 | `threats.execute` | Partial payments refunded; uncountered threats execute |
| 19 | `board.war` | War, at the end of every round: front battles; what it destroys is gone for next round's production |
| 20 | `board.cleanup` | Round flags removed; tips and the end checks run after every phase |

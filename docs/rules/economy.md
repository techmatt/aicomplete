# Resources and the round

*Each resource's sources and uses, city energy, the standard actions and the round's phases. Terms and conventions are in [README.md](README.md).*

| Resource | Comes from | Spent on |
|---|---|---|
| Credits | Credit production on the AI board ([ai-boards.md](ai-boards.md)), the node track, cards and Unity cards, in upkeep; megaproject payouts; selling cards and chips | Card costs; keeps beyond the opening's free ones (`opening_keep_credits` each); insight for keeps (`insight_credits` each); neutral cities' energy; standard projects, Clear and Move; the Builder's Buy In; threat counters |
| Energy (keeps between rounds; candidate: about 1-3 held mid-game, near 0 at the end) | Cities' cubes in production (*The round* below); Horizon's completion payout; the node tracks; Power programs (106: energy income lives on programs, +2 at most on any card: Baseload Contract; Tidal Barrage per 2 Power tags); the Power engines' when-built 2 (Substation Hub, Dispatch Control Room, Smart Grid Backbone; 108) and Power triggers (Positron Storage Ring's 2 a Power card); the Builder's Overcharge; the Protector's printed energy; `ai.start_energy` at the start | The node powers and every track step ([ai-boards.md](ai-boards.md)); the dear powers (Rush Build, Shelter, Particle Run); cards' `energy` cost (thirteen, power-hungry cards, [player-cards.md](player-cards.md)); the Actions of Grid Analytics Desk, Particle Accelerator, Directed-Energy Array, Holographic Assembly Hall and Wafer Fab Line; Blackout's, Sabotage's and Commandeer's counters; Bulwark's hardening |
| Chips | Player-owned factories on production steps (`factory_chips` each); Onyx's converter; the Builder's tracks and its Mint and Scrap; cards (chip income: Peaker Turbines, Arcology Foundry, Orbital Mass Driver, Fabrication Megaplex; Antitrust Action's chips per 2 Industry tags; Joint Fabrication Pact) | Some buildings' `chips` cost; draw 2 cards: the Builder's Procure (2 chips) and Parts Exchange's Action (1 chip); research stations (`station_chips`); the Harden action a card grants (`harden_chips`; no card does); sold at `chips_credits` |
| Antimatter | Cards (Positron Storage Ring, Zero-Day Auction, Intrusion Lab's threshold); the antimatter action; Horizon's converter; the Researcher's Particle Run | Tier-2 cards |
| Research cubes (keep between rounds) | Your stations, once their country's megaproject is complete; the Researcher's Open Archive capstone | The cube converter (a standard action: 1 cube for `cube_insight` insight, no limit a round); the corps' converters ([board.md](board.md)); the Researcher's Peer Review ([ai-boards.md](ai-boards.md)); Forensic Audit |
| Insight | Each round, set to the AI board's insight (5 printed, [ai-boards.md](ai-boards.md)) plus the node track's uncovers, buildings and programs' insight production (Endowed Research Chair, Frontier Lab Charter, Deep State Ledger, Recursive Self-Improvement) and the Unity card Think Tank's 1; less 1 per adversary ambassador that drained your nodes, and less what Smear Campaign and Deepfake Wave took. Mid-round: the cube converter (a standard action, 1 cube for `cube_insight`) and Cintamani's (1 cube for 2), both with no limit a round; each board's conversion (Focus, Vigil, Synthesis: [ai-powers.md](ai-powers.md)); card triggers (Interdisciplinary Institute, Telemetry Archive, Security Ops Floor); the Researcher's Open Science capstone gives others 2 a round | Keeping drafted cards; playing cards; the four threats countered in insight ([threats.md](threats.md)). It passes between players only through cards that print it (help cards such as Shared Compute Grant and Mutual Aid Pact, at their cost; r48: no standard action gives it). It expires at the round's end, but the Researcher's own carries over: Patience, [ai-boards.md](ai-boards.md) |
| Ambassadors | `start_ambassadors` at the start; Diplomacy cards (Consular Pouch, Peace Conference), Followers through a card's Action, Orbital Embassy's 1 a Diplomacy tag on each card you play; the standard project | Moving onto pieces and evicting (they stay, [ambassadors.md](ambassadors.md)); aligning cities, claiming sites, freeing cities (consumed) |

**Cities' energy.** Each city makes `corp_energy` (a megacorp parameter, more for a richer
corp) energy cubes a round. An aligned city's go free to the player who aligned it (+
`aligned_bonus`). A neutral city's can be bought for `city_energy_credits` credits by a
player with a node on or next to it (players choose who, or nobody); otherwise they are
lost. **An adversary ambassador on a city stays, and the city is dark** (Matt: neutral
cities at r37, aligned ones at r38): every round it stands there, a neutral city sells none of
its cubes and an aligned city gives its owner none, and they are lost. One ambassador makes a
city dark; `corrupt_at` of them corrupt it, so clearing ambassadors from the cities you buy
from, and above all from the cities you aligned, comes well before corruption. **Aligning a
dark city cancels one adversary ambassador on it** (r40, D38), as a site claim on a station or
factory does: both are removed and no cube goes on; once the city is clean, a second Align
aligns it. A corrupted city's go to the adversary (Discord).

**Cities' energy, for new players.** A city you aligned gives you its energy every round.
A neutral city next to your node sells you its energy. An adversary ambassador standing on
any city shuts it off: your aligned city gives you nothing, a neutral one sells nothing,
every round until someone removes the ambassador, and more of them corrupt the city. Aligning
a dark city spends your ambassador knocking one of theirs off; align again once it is clean. If
enemy ambassadors get that close to your city, work to stop them: clear them early. A Blackout stops a country's cities
in the production after the threat executes.

**No limits** (Matt, 2026-11-05, as in Terraforming Mars and Spirit Island). No resource has a holding limit and no
global cap limits production. A runaway pile means a runaway engine, fixed at card level (its numbers, its Action, a
redesign), never with a ceiling. Unspent energy stays energy: nothing converts it at round end. Finding and drafting
sinks for your energy is the player's job; providing them is the designers' (energy costs on power-hungry cards,
the Actions of Grid Analytics Desk, Particle Accelerator, Directed-Energy Array, Holographic Assembly Hall and Wafer
Fab Line, the tier-2 Citywide Microgrid and Exascale Training Cluster). Plays a round are bounded by
insight, which credits can't buy: every card that draws costs at least 1 insight to play.

**Each resource is paid in its home domain (principle accepted):** energy for nodes and
Power, credits for the market, chips for facilities, ambassadors for influence, insight for
the turn, tags for powers. A research station costs chips alone.

**Standard actions and projects** (`board.act`, in the action phase) are deliberately poorly
priced: cards are always the better way, and they are a luck hedge. **The gain-an-ambassador
project stays dear on purpose (Matt: 20 credits; overwhelming the adversary with bought
ambassadors takes a big credit investment, and cards, buildings and powers are the better
route):** so ambassadors come from Diplomacy cards, Followers and buildings, and one placed
evicts up to `remove_count` and stays. The **standard projects** (candidate, Matt) are the
credit sink, as in Terraforming Mars: poor but useful rates, no limit a round. Every map
target is in the player's reach. The view publishes each action with its cost
(`View.standard`), so clients do not repeat the parameters.

<!-- generated: standard_actions -->
| Action | Cost | What |
|---|---|---|
| Build a research station | `station_chips` 2 chips | on an open hex in reach that is not an industrial zone; it is yours (your cube) |
| Remove adversary ambassadors | `remove_ambassadors` 1 of your ambassadors, from your supply | it goes onto one hex in reach, removes up to `remove_count` 3 there and stays |
| Clear a corruption marker | `clear_cost` 2 credits | in reach: sway `clear_unity` 2 toward Unity, gain `clear_credits` 0 credits |
| Move a player ambassador | `move_credits` 1 credit | from your supply or the board onto a piece in reach (not the hex it comes from) |
| Gain a player ambassador | `ambassador_credits` 20 credits | into your supply; a standard project too |
| Align a clean city | 1 ambassador | in reach, with a site claim: your cube on it; on a dark city it cancels one adversary ambassador there instead (both removed), and a later Align aligns the city once it is clean |
| Free a research station or factory | `free_station_credits` 6 credits; `free_station_ambassadors` 2 of your ambassadors on it, used up | a corrupted station or adversary factory in reach with no adversary ambassador on it: it becomes neutral (a botnet there stays) |
| Harden a facility (a card that grants it; none does: Faraday Yard hardens when played) | the chips the card prints, per token | a player-owned station or factory in reach that is not corrupted, one a troop can strike |
| Sell chips | 1 chip | gain `chips_credits` 1 |
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
| 4–6 | `adversary.build`, `.spread`, `.corrupt` | The rest of the card (standing lines, twists, arrivals and build, Escalation), ongoing projects, botnet compute onto the projects; spread; corrupt |
| 7 | `board.march` | The adversary troops march ([war.md](war.md)): `troop_steps` steps each, move or destroy |
| 8 | `adversary.reveal` | Next round's card face up |
| 9–12 | `cards.tier`, `.deal`, `.draft`, `.keep` | Round 1: the opening deal and keep. Later: build the draft hand card by card (tier each), draft, keep ([player-cards.md](player-cards.md)) |
| 13 | `cards.play+threats.counter+alignment.act+board.act+ai.power` | Actions, simultaneous: cards, abilities, AI-board powers (nodes come from them), standard actions, countering threats, antimatter |
| 14 | `board.bases` | Production, on a production step: bases and factories make troops (ambassadors divert), factories make chips, cities grow factories |
| 15 | `board.produce` | Production: cities' energy cubes (neutral ones for sale) and stations (ambassadors divert); ambassadors on nodes drain insight; full megaprojects complete |
| 16 | `economy.upkeep` | Production: income; building upkeep |
| 17 | `board.war` | War, on a war step: front battles; what it destroys is gone for next round's production |
| 18 | `threats.execute` | Partial payments refunded; uncountered threats execute |
| 19 | `board.cleanup` | Round flags removed; tips and the end checks run after every phase |

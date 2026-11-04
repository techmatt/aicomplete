# Resources and the round

*Each resource's sources and uses, city energy, the standard actions and the round's phases. Terms and conventions are in [README.md](README.md).*

| Resource | Comes from | Spent on |
|---|---|---|
| Credits | Credit production on the AI board ([ai-boards.md](ai-boards.md)), the node track, cards and Unity cards, in upkeep; megaproject payouts; selling cards and chips | Card costs; keeps beyond the opening's free ones (`opening_keep_credits` each); insight for keeps (`insight_credits` each); neutral cities' energy; standard projects, Clear and Move; the Builder's Buy In; threat counters |
| Energy (keeps between rounds; rare, candidate: about 1-3 held mid-game, near 0 at the end) | Cities' cubes in production (*The round* below); Horizon's completion payout; the node tracks; Power buildings (a flat income of +2 a round at most on any card; Tidal Barrage grows with Power tags); the Builder's Overcharge; the Protector's printed energy; Power programs; `ai.start_energy` at the start | The node powers and every track step ([ai-boards.md](ai-boards.md)); the dear powers (Rush Build, Shelter, Particle Run); cards' `energy` cost (a few, two of them strong: Kinetic Interdiction, Orbital Mass Driver); Blackout's, Sabotage's and Commandeer's counters; the energy gift Actions (Substation Hub, Treaty Grid Exchange: 1 energy, another player gains); Bulwark's hardening |
| Chips | Player-owned factories on production steps (`factory_chips` each); Onyx's converter; the Builder's tracks and its Mint and Scrap; cards (chip income: Bulk Silicon Procurement, Peaker Turbines, Arcology Foundry; Antitrust Action's chips per Industry tag; Joint Fabrication Pact; Surplus Auction) | Some buildings' `chips` cost; research stations (`station_chips`); the Harden action a card grants (`harden_chips`); sold at `chips_credits` |
| Antimatter | Tier-2 cards (Positron Storage Ring); the antimatter action; Horizon's converter; the Researcher's Particle Run | Tier-2 cards |
| Research cubes (keep between rounds) | Your stations, once their country's megaproject is complete; the Researcher's Open Archive capstone; Sample Repository, Preprint Exchange | Converters ([board.md](board.md)); the Researcher's Peer Review ([ai-boards.md](ai-boards.md)); Inference Cluster, Breakthrough Lab, Policy Institute |
| Insight | Each round, set to the AI board's insight ([ai-boards.md](ai-boards.md)) plus the node track's uncovers plus buildings; less 1 per adversary ambassador that drained your nodes, and less what Smear Campaign took; Cintamani's converter adds 1; the Researcher's Open Science capstone gives others 2 a round; Inference Cluster | Keeping drafted cards; playing cards; giving (expires at the round's end, but the Researcher's carries over: Patience, [ai-boards.md](ai-boards.md)) |
| Ambassadors | `start_ambassadors` at the start; Diplomacy cards (Consular Pouch, Embassy Compound), Followers through a card's Action, income on Orbital Embassy; the standard project | Moving onto pieces and evicting (they stay, [ambassadors.md](ambassadors.md)); aligning cities, claiming sites, freeing cities (consumed) |

**Cities' energy.** Each city makes `corp_energy` (a megacorp parameter, more for a richer
corp) energy cubes a round. An aligned city's go free to the player who aligned it (+
`aligned_bonus`). A neutral city's can be bought for `city_energy_credits` credits by a
player with a node on or next to it (players choose who, or nobody); otherwise they are
lost. **An adversary ambassador on a city stays, and the city is dark** (Matt: neutral
cities at r37, aligned ones at r38): every round it stands there, a neutral city sells none of
its cubes and an aligned city gives its owner none, and they are lost. One ambassador makes a
city dark; `corrupt_at` of them corrupt it, so clearing ambassadors from the cities you buy
from, and above all from the cities you aligned, comes well before corruption. A corrupted
city's go to the adversary (Discord).

**Cities' energy, for new players.** A city you aligned gives you its energy every round.
A neutral city next to your node sells you its energy. An adversary ambassador standing on
any city shuts it off: your aligned city gives you nothing, a neutral one sells nothing,
every round until someone removes the ambassador, and more of them corrupt the city. If
enemy ambassadors get that close to your city, work to stop them: clear them early. A Blackout stops a country's cities
in the production after the threat executes.

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
| Clear a corruption marker | `clear_cost` 2 credits | in reach: sway `clear_unity` 1 toward Unity, gain `clear_credits` 0 credits |
| Move a player ambassador | `move_credits` 1 credit | from your supply or the board onto a piece in reach (not the hex it comes from) |
| Gain a player ambassador | `ambassador_credits` 20 credits | into your supply; a standard project too |
| Align a clean city | 1 ambassador | in reach, with a site claim: your cube on it (an adversary ambassador there stays, so a dark city stays dark) |
| Free a research station | `free_station_credits` 6 credits; `free_station_ambassadors` 2 of your ambassadors on it, used up | a corrupted station in reach with no adversary ambassador on it and no face-up cyber threat holding it: it becomes neutral (a botnet there stays) |
| Harden a facility (a card grants it: Hardened Bunker Complex) | the chips the card prints, per token | a player-owned station or factory in reach that is not corrupted, one a troop can strike |
| Sell chips | 1 chip | gain `chips_credits` 1 |
| Sell a card | gain `sell_credits` 1 | from your hand |
| Project: harden (`harden_buy`) | `harden_credits` 3 credits | any player: a hardening token on a player-owned station or factory in reach |
| Project: drain (`drain`) | `drain_credits` 5 credits | remove up to `drain_amount` 3 compute from one visible adversary project |
<!-- /generated -->

There is no "build a factory" standard action: cities build factories.

## The round

| # | Phase (config `round`) | What happens |
|---|---|---|
| | *(round start)* | Insight is set; abilities and powers refresh |
| 1 | `alignment.sway` | AI boards sway |
| 2–4 | `adversary.build`, `.spread`, `.corrupt` | The adversary ([adversary.md](adversary.md)): build, ongoing projects, botnet compute onto the projects; spread; corrupt |
| 5 | `board.march` | The adversary troops march ([war.md](war.md)): `troop_steps` steps each, move or destroy |
| 6 | `threats.deal` | Threats face up on each country |
| 7 | `adversary.reveal` | Next round's card face up |
| 8–11 | `cards.tier`, `.deal`, `.draft`, `.keep` | Round 1: the opening deal and keep. Later: build the draft hand card by card (tier each), draft, keep ([player-cards.md](player-cards.md)) |
| 12 | `cards.play+threats.counter+economy.give+alignment.act+board.act+ai.power` | Actions, simultaneous: cards, abilities, AI-board powers (nodes come from them), standard actions, countering threats, giving, antimatter |
| 13 | `board.bases` | Production 1, on a production step: bases and factories make troops (ambassadors divert), factories make chips, cities grow factories |
| 14 | `board.war` | Production 2, on a war step: front battles |
| 15 | `board.produce` | Production 3: cities' energy cubes (neutral ones for sale) and stations (ambassadors divert); ambassadors on nodes drain insight; full megaprojects complete |
| 16 | `economy.upkeep` | Production 4: income; building upkeep |
| 17 | `threats.execute` | Production 5: partial payments refunded; uncountered threats execute |
| 18 | `board.cleanup` | Round flags removed; tips and the end checks run after every phase |

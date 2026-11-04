# Ambassadors

*What an ambassador does on each piece, liberating cities, canceling and evicting. Terms and conventions are in [README.md](README.md).*

**An ambassador sits on a piece. The next time that piece does its normal thing, the
ambassador redirects it to its own side.** An adversary ambassador is then used up and
returns to the adversary's supply, unless it stands on a lone node or a city: on a city it redirects nothing, but
stays (a clean or aligned city goes dark; a corrupted one's energy is diverted anyway: the table). **A player ambassador
is consumed for a permanent change and stays for a position (candidate, Matt):** aligning a
city, claiming a site, freeing a station and liberating a city by the fallback consume it (no card spends one);
evicting, standing on a base, factory, station or botnet, and moving leave it on the board.
A consumed ambassador is gone: new ones come from cards, Followers and the standard project.

| On | Adversary ambassador | Player ambassador |
|---|---|---|
| City | it stays, and the city is **dark** while it stands there: a neutral city sells no energy (r37), an aligned city gives its owner none (r38; no Discord at `ambassador_discord` 0, below) | waits; a corrupted city is liberated the moment `liberate_at` player ambassadors stand on it from seats with a node on or next to it (the fallback, below; checked after every action and automatic phase) |
| Research station | that round's cube is the adversary's (on any station, a neutral one too); `station_flip_at` (2) of them on a player's station let an effect flip it ([board.md](board.md)) | on a neutral station the player can claim: the station becomes the player's (cube placed, ambassador used); on a corrupted station, that round's cube is the player's, and it stays; 2 of them there, with no adversary ambassador, can free it (the *Free a station* action, [economy.md](economy.md): both used up) |
| Factory | on a production step, an adversary troop on the factory's hex (**locked**); used up, on an adversary-owned factory too | on a neutral factory the player can claim: the factory becomes the player's; otherwise each production step it sways `factory_unity` Unity and no corp troop is made, except that an adversary-owned factory still makes its adversary troop; it stays |
| Node | if the node is alone on its hex (on a city, base, station or factory the ambassador is that piece's), each production its owner gets 1 less insight next round and 1 compute goes onto the cheapest adversary project; it stays until removed | waits (defense, below) |
| Military base | on a production step, `base_discord` Discord instead of a troop | on a production step, `base_unity` Unity instead of a troop; it stays |

- **Liberating a city (candidate; Matt).** A corrupted city is **liberated** by a liberate
  card ([player-cards.md](player-cards.md), *The liberate capstones*) or by the **fallback**: `liberate_at` (5) player
  ambassadors on it. **Both need the liberating seat's node on or next to the city**
  (getting close is the risk): a card liberates a corrupted city in its player's reach, and
  the fallback counts only ambassadors of seats with a node on or next to it. The fallback's
  ambassadors are used up (lower seats first). Either way its adversary cube returns to the
  supply, the city is **clean** (a card's extra may put its player's cube on it: aligned),
  every liberation sways `alignment.free_city` toward Unity, and the city **drops down the
  ladder rather than off the board**: `free_leaves` adversary ambassadors arrive on it as
  any placement does (a surplus player ambassador there cancels one, and the supply caps
  them). Its factories keep their owners ([war.md](war.md)). A face-up influence threat on the city holds
  it for both routes until it is countered.
  **The fallback is the bad route** (Matt): at `ambassador_credits` 20 each it costs about
  100 credits, for a seat with far too many. Ambassadors waiting on a city are canceled by
  the next build like any others, so the fallback is met in one action phase: gather five
  in your supply, stand next to the city, land them, plus one for each adversary ambassador
  on the city (a corrupted city keeps one through the action phase, and the first arrival
  cancels it: six, about 120 credits), unless the Remove action clears them first (its
  ambassador evicts them and stays, and counts).
- **Opposing ambassadors cancel.** When an ambassador arrives on a piece holding one of the
  other side, both are removed (the adversary's to its supply), with no Unity. **Two
  arrivals don't cancel:** the ambassador the Remove action moves evicts what stands there
  and stays. Any other move onto adversary ambassadors cancels one, consuming the mover:
  moved ambassadors don't evict, or one ambassador at 1 credit a hop would clear the board.
  An adversary ambassador arriving on a standing player ambassador cancels it, so a
  position is held only until the next arrival.
- **Defense:** a player ambassador on a facility's hex is used up instead of the facility
  when it would be destroyed.
- **Diverted** energy: a corrupted city's sways `divert_discord` Discord per unit, whatever
  stands on it. A clean or aligned city an adversary ambassador stands on is dark instead
  ([economy.md](economy.md)): nothing is diverted, the energy is lost; an aligned one sways `ambassador_discord` (0) per unit each round it is dark.

Player ambassadors move only through cards and the standard actions (Move, Remove).

**Liberating a city, for new players.** A corrupted city pours Discord and adversary
ambassadors into the world until someone liberates it. You need to stand next to it: your
nodes on the city or a hex beside it. Then either play a liberate card, the tier-2 capstone
of your engine (antimatter, power and chips, credits, research cubes, Cyber, Security or
Followers), or, the expensive way, land 5 of your ambassadors on it (at 20 credits each from
the standard project), one more for each adversary ambassador standing there, since the first
cancels it. The city is clean again (some cards also put your cube on it), the
world sways 5 toward Unity, and 1 adversary ambassador lands on it. In the draft, pass a
liberate card to the player whose engine can pay for it. **What resisted this summary:** a
fallback where two players pool ambassadors counts only the ambassadors of players standing
next to the city, and an influence threat face up on the city holds it until it is countered.

**For new players.** Ambassadors you spend change the board for good: aligning a city,
claiming a station or factory, liberating a city. Ambassadors you place hold a position: one
that evicts adversary ambassadors from a piece stays on it, and on a base, factory or botnet
it keeps working every round. The next adversary ambassador to land there cancels it, so a
position lasts until the adversary answers it.

**Evicting is capped by supply:** Remove takes the ambassador from your supply, never from
the board, so each eviction needs a new ambassador; that, and pieces filling up, cap the
loop. Adversary ambassadors can stand with no piece under them (on an empty
industrial zone; on a hex whose node was lost), so an evicting ambassador can too: it does
nothing there until a piece comes or it moves. An adversary ambassador left on a hex by a
lost node stays until removed.

# Alignment and the end of the game (module `alignment`)

*Sway, tipping, the Unity and Discord decks, the purge, and how the game is won and lost. Terms and conventions are in [README.md](README.md).*

Alignment is where the AIs' world leans: toward **Unity** (the good AIs, the players) or
**Discord** (the evil AI, the adversary). Everything on it is printed: Sway tokens on three
spaces, two tip-count tracks and two small decks of one-shot cards.

**The track.** Sway sits in three places: **the Undecided** (the middle pool), the **Unity
bucket** and the **Discord bucket**. The Undecided starts with `undecided_per_player` ×
players Sway; both buckets start empty. **Candidate:** the pool is exactly two tips.
**Swaying** toward a side moves Sway one at a time from the Undecided into that side's
bucket. With the Undecided empty, the variant decides:

| Variant | Sway with the Undecided empty |
|---|---|
| `tug` (default; **locked**: empty-pool rule B) | comes from the opposite bucket (a tug of war); with that empty too, nothing |
| `still` | nothing happens |
| `off` | (for measuring Alignment, [README.md](README.md#off-variants)) no Sway ever moves and nothing tips |

**Tipping.** The Unity bucket holding `tip_per_player` × players Sway, or the Discord bucket
holding `discord_tip_per_player` × players, **tips**, in this order: all the bucket's Sway
returns to the Undecided; that side's tip count goes up by one; one card of its side is
drawn and resolves (at the step printed for the tip count before this tip, below).
The bucket empties before the card, so a card's own Sway counts. Unity tips and
Discord tips are counted separately and only count up; there is no net meter. Buckets are checked when an action or automatic phase finishes resolving (with win and loss,
below), so a bucket may overfill within one action. If both are full, Discord tips first; a
tip's card may fill a bucket again, which then tips too.

**Momentum** (`momentum_every`, 0 = off): a side's bucket holds 1 fewer Sway for every
`momentum_every` tips that side has made, down to 1 per player.

**Sources of Sway**

<!-- generated: sway_sources -->
| Toward | From | Amount |
|---|---|---|
| Unity | a player ambassador on a producing base / factory | `base_unity` 2 / `factory_unity` 1 |
| Unity | removing an adversary troop | `alignment.troop` 0 |
| Unity | Biomia's converter | 1 for 2 research cubes |
| Unity | liberating a corrupted city (ambassadors.md: a liberate card or the fallback) | `alignment.free_city` 5 |
| Unity | removing a botnet | `alignment.botnet` 2 |
| Unity | clearing a corruption marker (the Clear action, Recovery, cards) | `clear_unity` 1 |
| Unity | player cubes on a megaproject as it completes, as its corp's card prints (Delta Seawall) | 1 each |
| Unity | cards (Peace Conference, Coalition Charter, Open Assembly), Unity cards | card text |
| Discord | energy diverted (a corrupted city) or darkened (an ambassador on an aligned city) | `divert_discord` 1 per unit of a corrupted city's, `ambassador_discord` 0 per unit, each round, of a dark aligned city's |
| Discord | an adversary ambassador on a producing base | `base_discord` 1 |
| Discord | a facility destroyed in war (not by an adversary troop) | `destruction_discord` 0 |
| Discord | adversary cubes on a megaproject as it completes, as its corp's card prints | 1 each |
| Discord | an uncountered threat, the antimatter action, twists, Discord cards, adversary projects | `threat_discord` 1, `antimatter_discord` 1, card text |
| Discord | the adversary at the start of every round | `adversary_discord` 0 |
<!-- /generated -->

`adversary_discord` is 0 because corrupted cities' diverted energy is the adversary's
per-round drip (1 per corrupted city).

The round's Sway is the phase `alignment.sway`, first in the round: the adversary's
`adversary_discord`, and nothing else (no AI board prints a Unity Sway).

**More Unity.** Unity should tip in most games while Discord stays the main pressure:
liberating a corrupted city sways `alignment.free_city`, removing a botnet
`alignment.botnet`, and a player ambassador on a producing military base sways `base_unity`
(**locked**).

**The antimatter action.** In the action phase (`...+alignment.act`), any player may sway
`antimatter_discord` toward Discord to gain 1 antimatter, up to `antimatter_uses` times a
round each. `antimatter_discord` 0 removes the action, as does variant `off`.

**The cards (one-shot and hidden, like fear cards: locked).** Each side has a small
face-down deck in two tiers, each shuffled at setup
([`content/alignment.json`](../../content/alignment.json)). A tip draws from tier 2 once that
side's tip count, before the tip, has reached `unity_ease[0]` / `discord_tier2`, else from
tier 1; from the other tier if that one is empty. The card resolves at once and goes face up
on its side's resolved pile; there are no ongoing cards. **Neither deck ever reshuffles:
they are the game's two clocks.** A Discord tip that needs a card and finds the deck empty
loses the game (**locked**, Matt); drawing the last Unity card wins it outright, and that
card doesn't act. **A game deals a small Unity deck (candidate):**
`unity_cards` of the printed Unity cards, tier 1 then tier 2, at random and face down, so
the last card can be reached. Choices are asked of seat 0. "hidden like fear
cards" is read as a face-down deck resolved the moment it tips; Spirit Island holds earned
fear cards face down until a later phase, which would be a timing change. Grassroots
Funding, Think Tank and New Deal give income "for the rest of the game": a one-shot card
that moves a printed income marker, not a card left in play.

**Steps.** A Discord card prints its strength by the Discord tip count (counted before this
tip, so "3+ earlier Discord tips" first fires on the 4th tip). Unity cards have no steps.

<!-- generated: alignment -->
| Card | Side | Tier | Text | Stronger after tips |
|---|---|---|---|---|
| Grassroots Funding | unity | 1 | Every player gains 2 credits each round for the rest of the game. |  |
| Open Market | unity | 1 | The next 2 cards any player keeps from the draft are free. |  |
| Think Tank | unity | 1 | Every player gains 1 insight each round for the rest of the game. |  |
| Ceasefire | unity | 1 | Counter an active threat. |  |
| Civil Defense | unity | 1 | In each country, remove 1 adversary ambassador. |  |
| Volunteer Corps | unity | 1 | Sway 3 toward Unity. |  |
| Global Coalition | unity | 2 | Counter every active threat. |  |
| New Deal | unity | 2 | Every player gains 4 credits now, and 2 credits each round for the rest of the game. |  |
| Decommission | unity | 2 | Remove a botnet anywhere on the board. |  |
| Incursion | discord | 1 | Deal 1 extra threat. 5+ earlier Discord tips: deal 3 instead, and each player destroys 2 of their nodes. | 5 |
| Blight | discord | 1 | In each country, the clean city with the most adversary ambassadors gains 1 more. 5+ earlier Discord tips: 3 more instead, and each player destroys 2 of their nodes. | 5 |
| Propaganda | discord | 1 | Every player loses 4 credits; move 2 Sway from the Unity bucket to the Undecided. 5+ earlier Discord tips: lose 8 credits instead, and each player destroys 2 of their nodes. | 5 |
| Offensive | discord | 2 | The adversary spreads, then destroy all nodes on the node hex with the most adversary ambassadors and troops. 3+ earlier Discord tips: each player also destroys 1 of their nodes. 4+ instead: the adversary builds, then spreads, and each player destroys 2. | 3, 4 |
| Two Fronts | discord | 2 | Deal 2 extra threats. 3+ earlier Discord tips: each player also destroys 1 of their nodes. 4+ instead: deal 3, and each player destroys 2. | 3, 4 |
| Firestorm | discord | 2 | In each country, the clean city with the most adversary ambassadors gains 2 more. 3+ earlier Discord tips: each player also destroys 1 of their nodes. 4+ instead: 3 more, and each player destroys 2. | 3, 4 |
<!-- /generated -->

A threat countered by a card is discarded and its partial payments are refunded.
**Details:**

- Open Market's free cards are keeps of drafted cards ([player-cards.md](player-cards.md)), by any player, used before
  credits.
- **Unity pulled back:** "Move N Sway from the Unity bucket to the Undecided" returns up
  to N.
- Extra threats come from the current tier's deck (an empty deck reshuffles, as for the
  round's deal), one per country in turn from country 0 (`shared_row`: into the row), and
  are countered or execute like any other. One dealt after the round's counter phase (a tip
  in production or upkeep, False Flag) waits beside the row and executes next round, after a
  counter phase.
- Blight and Firestorm: ties go to the lower hex number, so with no ambassadors on any clean
  city the lowest-numbered one gains them; a country with no clean city gains nothing. Civil
  Defense in a country with no adversary ambassador removes nothing.
- Offensive acts at once, out of the round's order, with this round's card's build or spread
  twists; its build has no Escalation. Before the build step (a tip in the Sway phase, with
  `adversary_discord` 1+) this round's card is the face-up one.

**The purge (the win), eased by Unity.** The purge needs no corrupted cities, no botnets, no
adversary troops and no adversary ambassadors on the board. Unity eases it at each
`unity_ease` threshold. **There is no printed round gate:** the depth of the steps and
liberating the corrupted cities ([ambassadors.md](ambassadors.md)) set how early it can come.

<!-- generated: purge_ease -->
- first threshold (2 Unity tips): adversary ambassadors stop counting;
- second threshold (2 Unity tips): adversary troops stop counting;
- third threshold (4 Unity tips): **botnets** stop counting (candidate: with botnet removal rare, Unity buys the win);
- corrupted cities always count.
<!-- /generated -->

The thresholds are a list: the first is ambassadors, the second troops, the third botnets;
with only two entries botnets always count. **Why:** with no standard action to remove a
botnet, botnets would block the purge in most games. Troops stop counting early (with
ambassadors) because they respawn from adversary factories every production step and walk
away from where a card can reach them. The level-0 purge (below the first threshold) counts
only once this round's adversary card is of stage `clear_win_stage` or later; at 1 (the
default) that is from round 1's card on. From the first threshold the stage doesn't hold the
purge back.

**No escalation in play.** Discord tips don't raise the difficulty. Discord escalates
through each tip's card and the Discord cards' printed steps, and from `discord_tier2` tips
draws from the stronger tier. The view shows the configured level.

<!-- generated: game_end -->
**How the game ends, for new players.** You win by purging the board or by drawing the last Unity card. Pressure comes every round: the face-up adversary card names a kind of site, and every such site gains an adversary ambassador, so you always know where the next wave lands. Ambassadors pile up, pairs become botnets, 4 on a city corrupt it, and corrupted cities and uncountered threats push the Alignment track toward Discord. Each time the track tips, a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, you lose. Pushing back works the same way in your favor. Mass your ambassadors on a corrupted city to free it, which sways toward Unity, and each Unity tip brings a good card. The Unity deck is small too: a game deals only 6, and drawing the last wins at once. 2 Unity tips and the purge asks only for no botnets and no corrupted cities; 4 and it asks only for no corrupted cities. You also lose if any player has no nodes left on the board. Threats, war, corruption and late adversary and Discord cards destroy it, a token at a time, and a lost token leaves the game and marks its hex, so you can't act from there any more; its income stays. In stage I your last one is safe. The adversary deck is a round cap too, 11 rounds at most, though games rarely reach it. So there are three different pressures: the Discord deck (politics), your nodes (the board) and the cap.
<!-- /generated -->

**The printed steps:** with no reshuffle the draw order is fixed, tier 1 at tips 0-1 and 5,
tier 2 at tips 2-4, and a tip with 6 already made finds the deck empty (a tip sooner at
difficulty 4). So tier-1 cards step at 5, tier-2 at 4.

**Winning:** the purge, or drawing the last Unity card.

**Losing:**

- a Discord tip finds the Discord deck empty (**locked**);
- a player has no nodes on the board: everyone loses, as when a spirit is destroyed
  (**locked**);
- the adversary deck runs out (the **round cap**, a quiet backstop; [adversary.md](adversary.md)).

The supply running out is not a loss (it is a cap, [adversary.md](adversary.md)), nor is every city corrupted: a
corrupted board drives Discord.

Win and loss are checked whenever an action or automatic phase finishes resolving: full
buckets tip first (Discord before Unity), then losses, then the purge. A tip that ends the
game ends it there.

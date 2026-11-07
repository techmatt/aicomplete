# Alignment and the end of the game (module `alignment`)

*Sway, the two tracks and tipping, the Unity and Discord decks, the purge, and how the game is won and lost. Terms and conventions are in [README.md](README.md).*

Alignment is where the AIs' world leans: toward **Unity** (the good AIs, the players) or
**Discord** (the evil AI, the adversary). Everything on it is printed: two Sway tracks, two
tip-count tracks and two small decks of one-shot cards.

**Two independent tracks (r55, candidate).** Discord Sway fills the **Discord track** and Unity
Sway the **Unity track**; both start empty, there is no shared pool and nothing on one side
empties the other. **Holding Discord off:** each Unity a player sways may instead move 1 Discord
off the Discord track (`hold_off` 1), and Envoy ([economy.md](economy.md), a standard action) does
the same for one of your ambassadors, so the team chooses between racing toward Unity and putting
off the next Discord card. Unity on its own track never stops a Discord tip. The choice is made once
the action or automatic phase that swayed the Unity has resolved, before any track tips, asked of
seat 0 for the team (one decision for the whole amount). Variant `tracks` is the rule; `off`
(for measuring Alignment, [README.md](README.md#off-variants)) moves no Sway and tips nothing.

**Tipping.** The Unity track holding `tip_per_player` × players Sway, or the Discord track
holding `discord_tip_per_player` × players, **tips**: one tip's worth comes off and, on the Unity
side, the rest stays toward the next tip (`carry` 1, r48: a big Sway can tip more than once); a
Discord tip empties its track (`carry` 2 would carry Discord's too, 0 neither); that side's tip count
goes up by one; one card of its side is drawn and resolves (at the step printed for the tip count
before this tip, below). Unity tips and Discord tips are counted separately and only count up.
Tracks are checked when an action or automatic phase finishes resolving (with win and loss, below),
so a track may overfill within one action. If both are full, **Discord tips first**; a tip's card may
fill a track again, which then tips too. **Candidate thresholds** (136): see *Tunables*; they are
set so that the Discord track fills faster than the team can hold it off without giving up Unity.

**Momentum** (`momentum_every`, 0 = off): a side's track holds 1 fewer Sway for every
`momentum_every` tips that side has made, down to 1 per player.

**Sources of Sway**

<!-- generated: sway_sources -->
| Toward | From | Amount |
|---|---|---|
| Unity | removing an adversary troop | `alignment.troop` 0 |
| Unity | Biomia's converter | 1 for 2 research cubes |
| Unity | liberating a corrupted city (ambassadors.md: a liberate card or the fallback) | `alignment.free_city` 5 |
| Unity | removing a botnet | `alignment.botnet` 1 |
| Unity | clearing a corruption marker (the Clear action, Recovery, cards) | `clear_unity` 2 |
| Unity | player cubes on a megaproject as it completes, as its corp's card prints (Delta Seawall) | 1 each |
| Unity | cards (Peace Conference, Coalition Charter, Open Assembly), Unity cards | card text |
| Discord | energy diverted (a corrupted city) or darkened (an ambassador on an aligned city) | `divert_discord` 1 per unit of a corrupted city's, `ambassador_discord` 0 per unit, each round, of a dark aligned city's |
| Discord | an adversary ambassador on a producing base | `base_discord` 1 |
| Discord | a facility destroyed in war (not by an adversary troop) | `destruction_discord` 0 |
| Discord | adversary cubes on a megaproject as it completes, as its corp's card prints | 1 each |
| Discord | an uncountered threat, the antimatter action, twists, Discord cards, adversary projects | `threat_discord` 1, `antimatter_discord` 1, card text |
| Discord | the adversary board's drip, at the start of every round | as printed (The Militarist: 0) |
<!-- /generated -->

Corrupted cities' diverted energy is the adversary's per-round drip (1 per corrupted city). The
round's Sway is the phase `alignment.sway`, first in the round: the adversary board's printed
drip (in its content; the Militarist prints none), and nothing else (no AI board
prints a Unity Sway). An uncountered threat sways `threat_discord` (an unprinted rule, kept).

**More Unity.** Unity comes from pushing the adversary back: liberating a corrupted city sways
`alignment.free_city`, clearing a corruption marker `clear_unity`, removing a botnet
`alignment.botnet`, and cards. A player ambassador on a producing base or factory stops its
troop and sways nothing (r55).

**The antimatter action.** In the action phase (`...+alignment.act`), any player may sway
`antimatter_discord` toward Discord to gain 1 antimatter, up to `antimatter_uses` times a
round each. `antimatter_discord` 0 removes the action, as does variant `off`.

**The cards (one-shot and hidden, like fear cards: locked).** Each side has a small
face-down deck in two tiers, each shuffled at setup
([`content/alignment.json`](../../content/alignment.json)). A tip draws from tier 2 once that
side's tip count, before the tip, has reached `unity_tier2` / `discord_tier2`, else from
tier 1; from the other tier if that one is empty. The card resolves at once and goes face up
on its side's resolved pile; there are no ongoing cards. **Neither deck ever reshuffles:
they are the game's two clocks.** A Discord tip that needs a card and finds the deck empty
loses the game (**locked**, Matt); drawing the last Unity card wins it outright, and that
card doesn't act. **Every printed Unity card is dealt** (r55: 9; drawing the 9th wins). Choices
are asked of seat 0. "hidden like fear cards" is read as a face-down deck resolved the moment it
tips; Spirit Island holds earned fear cards face down until a later phase, which would be a timing
change. Grassroots Funding, Think Tank and New Deal give income "for the rest of the game": a
one-shot card that moves a printed income marker, not a card left in play.

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
| Propaganda | discord | 1 | Every player loses 4 credits; remove 2 Unity from the Unity track. 5+ earlier Discord tips: lose 8 credits instead, and each player destroys 2 of their nodes. | 5 |
| Offensive | discord | 2 | The adversary spreads, then destroy all nodes on the node hex with the most adversary ambassadors and troops. 3+ earlier Discord tips: each player also destroys 1 of their nodes. 4+ instead: the adversary builds, then spreads, and each player destroys 2. | 3, 4 |
| Two Fronts | discord | 2 | Deal 2 extra threats. 3+ earlier Discord tips: each player also destroys 1 of their nodes. 4+ instead: deal 3, and each player destroys 2. | 3, 4 |
| Firestorm | discord | 2 | In each country, the clean city with the most adversary ambassadors gains 2 more. 3+ earlier Discord tips: each player also destroys 1 of their nodes. 4+ instead: 3 more, and each player destroys 2. | 3, 4 |
<!-- /generated -->

A threat countered by a card is discarded and its partial payments are refunded.
**Details:**

- Open Market's free cards are keeps of drafted cards ([player-cards.md](player-cards.md)), by any player, used before
  credits.
- **Unity pulled back:** "Remove N Unity from the Unity track" removes up to N (r55: it
  used to return them to the Undecided).
- Extra threats come from the current tier's deck (an empty deck reshuffles, as for the
  round's deal), one per country in turn from country 0 (`shared_row`: into the row), and
  are countered or execute like any other. One dealt after the round's counter phase (a tip
  in production or upkeep, False Flag) waits beside the row and executes next round, after a
  counter phase.
- Blight and Firestorm: ties go to the lower hex number, so with no ambassadors on any clean
  city the lowest-numbered one gains them; a country with no clean city gains nothing. Civil
  Defense in a country with no adversary ambassador removes nothing.
- Offensive acts at once, out of the round's order, with this round's card's build or spread
  twists; its build has no Escalation. Before the build step (a tip in the Sway phase, from a
  board's drip) this round's card is the face-up one.

**The purge (the win), by Unity cards drawn (r55, candidate).** What you must remove from the
board to win depends on how many Unity cards have been drawn (`purge_at`: ambassadors, troops,
botnets stop counting at those counts). **There is no printed round gate:** the depth of the
steps and liberating the corrupted cities ([ambassadors.md](ambassadors.md)) set how early it
can come.

<!-- generated: purge_ease -->
- 0-2 Unity cards drawn: remove the corrupted cities, botnets, adversary troops, adversary ambassadors;
- 3-5 Unity cards drawn: remove the corrupted cities, botnets, adversary troops;
- 6+ Unity cards drawn: remove only the corrupted cities;
- corrupted cities always count.
<!-- /generated -->

The steps are a list: the first is ambassadors, the second troops, the third botnets;
with only two entries botnets always count. **Why:** with no standard action to remove a
botnet, botnets would block the purge in most games, and troops respawn from adversary
factories every production step. The level-0 purge (below the first step) counts only once this
round's adversary card is of stage `clear_win_stage` or later; at 1 (the default) that is from
round 1's card on.

**No escalation in play.** Discord tips don't raise the difficulty. Discord escalates
through each tip's card and the Discord cards' printed steps, and from `discord_tier2` tips
draws from the stronger tier. The view shows the configured level.

<!-- generated: game_end -->
**How the game ends, for new players.** You win by purging the board or by drawing the last Unity card. Pressure comes every round: the face-up adversary event card deals threats by your sites and names a kind of site, and every such site gains an adversary ambassador, so you always know where the next wave lands. Ambassadors pile up, pairs become botnets, 4 on a city corrupt it, and corrupted cities and uncountered threats fill the Discord track. Each time it fills, it tips: a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, you lose. Unity has its own track, which nothing on the Discord side empties: free a corrupted city, clear corruption or remove a botnet and you sway Unity, and each Unity tip brings a good card. Each Unity you sway may instead take 1 Discord off its track, and Envoy trades one of your ambassadors for the same, so you choose between racing toward Unity and holding off the next Discord card. All 9 Unity cards are dealt, and drawing the last wins at once. With 3 Unity cards drawn the purge no longer asks you to clear adversary ambassadors; with 6, it asks only for no corrupted cities. You also lose if any player has no nodes left on the board. Threats, war, corruption and late adversary and Discord cards destroy it, a token at a time, and a lost token leaves the game and marks its hex, so you can't act from there any more; its income stays. In stage I your last one is safe. The adversary deck is a round cap too, 11 rounds at most, though games rarely reach it. So there are three different pressures: the Discord deck (politics), your nodes (the board) and the cap.
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

Win and loss are checked whenever an action or automatic phase finishes resolving: the Unity
just swayed is placed (or holds Discord off), full tracks tip (Discord before Unity), then
losses, then the purge. A tip that ends the game ends it there.

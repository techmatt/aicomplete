# Alignment and the end of the game (module `alignment`)

*Sway, the two tracks and tipping, the Unity and Discord decks, the purge, and how the game is won and lost. Terms and conventions are in [README.md](README.md).*

Alignment is where the AIs' world leans: toward **Unity** (the good AIs, the players) or
**Discord** (the evil AI, the adversary). Everything on it is printed: two Sway tracks, two
tip-count tracks and two decks of one-shot cards, each printing three tiers.

**Two independent tracks (r55, candidate).** Discord Sway fills the **Discord track** and Unity
Sway the **Unity track**; both start empty, there is no shared pool and nothing on one side
empties the other. **Holding Discord off:** each Unity a player sways may instead move 1 Discord
off the Discord track (`hold_off` 1), and Envoy ([economy.md](economy.md), a standard action) does
the same for one of your ambassadors, so the team chooses between racing toward Unity and putting
off the next Discord card. Unity on its own track never stops a Discord tip. The choice is made once
the action or automatic phase that swayed the Unity has resolved, before any track tips, asked of
seat 0 for the team (one decision for the whole amount); only an amount that keeps the Discord track
from tipping is offered, since a tip empties it anyway. Variant `off` moves no Sway
([README.md](README.md#off-variants)).

**Tipping.** The Unity track holding `tip_per_player` × players Sway, or the Discord track
holding `discord_tip_per_player` × players, **tips**: one tip's worth comes off and, on the Unity
side, the rest stays toward the next tip (`carry` 1, r48: a big Sway can tip more than once); a
Discord tip empties its track (`carry` 2 would carry Discord's too, 0 neither); that side's tip count
goes up by one; one card of its side is drawn and resolves (at the step printed for the tip count
before this tip, below). Unity tips and Discord tips are counted separately and only count up.
Tracks are checked when an action or automatic phase finishes resolving (with win and loss, below),
so a track may overfill within one action. If both are full, **Discord tips first**; a tip's card may
fill a track again, which then tips too. Thresholds: *Tunables*.

**Sources of Sway**

<!-- generated: sway_sources -->
| Toward | From | Amount |
|---|---|---|
| Unity | removing an adversary troop | `alignment.troop` 0 |
| Unity | Biomia's converter | 1 for 2 research cubes |
| Unity | liberating an adversary city (ambassadors.md: a liberate card or the Liberate action) | `alignment.free_city` 5 |
| Unity | liberating an adversary station or factory (the Liberate action) | `free_facility` 2 |
| Unity | removing a botnet | `alignment.botnet` 1 |
| Unity | clearing a corruption marker (the Clear action, Recovery, cards) | `clear_unity` 2 |
| Unity | player cubes on a megaproject as it completes, as its corp's card prints (Delta Seawall) | 1 each |
| Unity | cards (Peace Conference, Coalition Charter, Open Assembly), Unity cards | card text |
| Discord | energy diverted (a corrupted city) or darkened (an ambassador on an aligned city) | `divert_discord` 1 per unit of a corrupted city's, `ambassador_discord` 0 per unit, each round, of a dark aligned city's |
| Discord | a factory destroyed in war (not by an adversary troop) | `destruction_discord` 1 |
| Discord | adversary cubes on a megaproject as it completes, as its corp's card prints | 1 each |
| Discord | the antimatter action, twists, threats, Discord cards, adversary projects | `antimatter_discord` 1, card text |
| Discord | the adversary board's drip, at the start of every round | as printed (The Militarist: 0) |
| Discord | each land-corruption token (Blitzkrieg, Naval Blockade), at the start of every round | `blight_discord` 1 |
<!-- /generated -->

Corrupted cities' diverted energy sways Discord in production (1 per corrupted city). The
round's Sway is the phase `alignment.sway`, first in the round: the adversary board's printed
drip (in its content; the Militarist prints none) and `blight_discord` (1) for each land-corruption
token ([adversary.md](adversary.md)), and nothing else (no AI board
prints a Unity Sway). A threat sways Discord only where it prints it (146).

**The antimatter action.** In the action phase (`...+alignment.act`), any player may sway
`antimatter_discord` toward Discord to gain 1 antimatter, up to `antimatter_uses` times a
round each. `antimatter_discord` 0 removes the action, as does variant `off`.

**The cards (one-shot and hidden, like fear cards: locked).** Each side has one face-down
deck, shuffled at setup ([`content/alignment.json`](../../content/alignment.json)), every printed
card dealt. **Three tiers (r67, candidate):** every card prints a tier I, II and III effect, as
Spirit Island's fear cards; its side's tip count before this tip picks which resolves, by
`tier_at` (I below 3 earlier tips, II from 3, III from 6). For Unity these are the purge's own
steps (below). The card resolves at once and goes face up on its side's resolved pile; there are
no ongoing cards. **The Discord deck never reshuffles: it is the clock.** A Discord tip that
finds it empty loses (**locked**, Matt); with 6 cards the 7th tip loses, so a Discord tier III
never resolves at baseline (asked of Matt, 156). **The Unity deck reshuffles** its resolved pile
when a tip finds it empty (r67: 20 cards, so it rarely does); drawing it out wins nothing.
Choices are asked of seat 0. **One player** (`one_player`): the team picks a player it would act
for, who resolves the line as their own card would, in their reach. Grassroots Funding, Think
Tank and New Deal move a printed income marker "for the rest of the game". Cards marked
(draft) are 156's drafts, for Matt to keep or prune.

<!-- generated: alignment -->
- **unity** (20): Grassroots Funding, Open Market, Think Tank, Ceasefire, Civil Defense, Volunteer Corps, Global Coalition, New Deal, Decommission, Peacekeepers (draft), Town Hall Accord (draft), Popular Mandate (draft), Reclamation (draft), Mutual Aid (draft), Disarmament Talks (draft), Network Cleanup (draft), Citizens' Assembly (draft), Rebuilding Fund (draft), Safe Corridors (draft), Reconciliation Panel (draft).
- **discord** (6): Incursion, Blight, Propaganda, Offensive, Two Fronts, Firestorm.
- Each card's three tiers: [cards.md](../cards.md#unity-and-discord-cards).
<!-- /generated -->

A threat countered by a card is discarded and its partial payments are refunded.
**Details:**

- Open Market's free cards are keeps of drafted cards ([player-cards.md](player-cards.md)), by any player, used before
  credits.
- **Unity pulled back:** "Remove N Unity from the Unity track" removes up to N.
- Extra threats come from the deck of the tier of this round's adversary card's first pull (an empty deck reshuffles), one per country in turn from country 0,
  and are countered or execute like any other. One dealt after the round's counter
  phase (a tip in production or upkeep, False Flag) waits and executes next round, after a counter phase.
- Blight and Firestorm create on the lowest-numbered city holding adversary ambassadors (138: a creation line); a
  country with none gains nothing. Blight and Civil Defense also open a neutral factory or research station on each
  board. Civil Defense in a country with no adversary ambassador removes nothing.
- **Discord cards destroy no nodes (139, Matt) and change no rule for the whole map (156):** their higher tiers add
  pressure on the board (more or tier-2 threats, creation, Corrupt lines) or strip player pieces (credits, Unity,
  ambassadors); nodes fall only where the board is overrun ([war.md](war.md)). "Two tier-2 threats in each country"
  pulls from the tier-2 deck, any tag, two a country (146).
- Offensive acts at once: Corrupt cities (tier III: the adversary builds first, each board rallying its ambassador
  income to this round's card's Rally site), then a tier-2 threat (two from tier II), any tag, is dealt into the
  country of the node hex with the most adversary ambassadors and troops (ties: country, then lower hex number), its
  first token on that hex; dealt after the counter phase, it waits a round. Before the build step this round's card
  is the face-up one.
- **Unity tier III leans to freeing cities** (156): liberating a corrupted city in reach, clearing every adversary
  ambassador around one, or removing troops or a botnet in reach. Tier III helps; the purge never needs it.

**The purge: the only win (r67, locked: Matt, 2026-11-08), stepped by Unity tips** as Spirit
Island's terror levels are. What you must remove from the board depends on the Unity tips made
(`purge_at`: ambassadors, troops, botnets stop counting at those counts): at 0 almost everything;
from 6 only the cities, and the players win the moment the adversary owns no city. No round
gate: the steps and liberating cities ([ambassadors.md](ambassadors.md)) set how early it comes.

<!-- generated: purge_ease -->
- 0-2 Unity tips: remove the corrupted cities, botnets, adversary troops, adversary ambassadors;
- 3-5 Unity tips: remove the corrupted cities, botnets, adversary troops;
- 6+ Unity tips: remove only the corrupted cities;
- corrupted cities always count.
<!-- /generated -->

The steps are a list: ambassadors, then troops, then botnets (two entries: botnets always count).
**Why:** botnets are hard to remove and troops respawn from adversary factories every round. The
level-0 purge counts only from this round's adversary card of stage `clear_win_stage` (1: from round 1).

**No escalation in play:** Discord tips don't raise the difficulty; the cards' tiers escalate.

<!-- generated: game_end -->
**How the game ends, for new players.** The only way to win is the purge, and Unity eases it, step by step, like terror levels. At 0 Unity tips you must purge almost everything: corrupted cities, botnets, adversary troops and adversary ambassadors. After 3 Unity tips the purge no longer asks you to clear adversary ambassadors; after 6, it asks only that the adversary owns no city, and you win the moment it doesn't. Pressure comes every round: the face-up adversary event card deals threats by your sites and says where its new adversary ambassadors land (the lowest-numbered sites of a kind), so you always know where the next wave lands. Ambassadors pile up, a card's Corrupt line pulls 4 of them on and next to a city into it and corrupts it, and corrupted cities and uncountered threats fill the Discord track. Each time it fills, it tips: a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, you lose. Discord cards never destroy your nodes directly, only through the threats and corruptions they bring: they deal more or stronger threats (Offensive aims one at the node hex most crowded with adversary pieces), create ambassadors, run Corrupt lines, and take credits, Unity or your ambassadors. Unity has its own track, which nothing on the Discord side empties: free a corrupted city, clear corruption or remove a botnet and you sway Unity, and each Unity tip brings a good card. Every Unity and Discord card prints three tiers: its side's tips so far pick which one resolves (tier I below 3, II from 3, III from 6), so Unity's late cards help free cities and Discord's late cards hit harder. The 20 Unity cards reshuffle when the deck runs out. Each Unity you sway may instead take 1 Discord off its track, and Envoy trades one of your ambassadors for the same, so you choose between racing toward Unity and holding off the next Discord card. You also lose if any player has no nodes left on the board. Nodes are lost only where the board is overrun: an uncountered threat near them, a corruption of their own hex, war across a front (it takes the factory's hex) and Border War (nodes by a front's base). A lost token leaves the game and marks its hex, so you can't act from there any more; its income stays. In stage I your last one is safe. The adversary deck is a round cap too, 11 rounds at most (fewer at the harder levels), and reaching it loses. So there are three different pressures: the Discord deck (politics), your nodes (the board) and the cap.
<!-- /generated -->

**Losing:**

- a Discord tip finds the Discord deck empty (**locked**);
- a player has no nodes on the board: everyone loses, as when a spirit is destroyed
  (**locked**);
- the adversary deck runs out (the **round cap**, a plain loss, **locked**; [adversary.md](adversary.md)).

The supply running out is not a loss (it is a cap, [adversary.md](adversary.md)), nor is every city corrupted: a
corrupted board drives Discord.

Win and loss are checked whenever an action or automatic phase finishes resolving: the Unity
just swayed is placed (or holds Discord off), full tracks tip (Discord before Unity), then
losses, then the purge. A tip that ends the game ends it there, and a player with no node loses
before a Unity tip is drawn.

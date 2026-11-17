# Alignment and the end of the game (module `alignment`)

*Sway, the two tracks and tipping, the Unity and Discord decks, the purge, and how the game is won and lost. Terms and conventions are in [README.md](README.md).*

**Unity** is the players' side, **Discord** the adversary's. All of it is printed: two Sway tracks, two tip-count
tracks and two decks of one-shot cards.

**Two independent tracks (candidate).** Discord Sway fills the **Discord track**, Unity Sway the **Unity track**; both
start empty, with no shared pool, and nothing on one side empties the other. **Holding Discord off:** each Unity a
player sways may instead move 1 Discord off its track (`hold_off` 1), and Envoy (a standard action,
[economy.md](economy.md)) does the same for one of your ambassadors. Unity on its own track never stops a Discord tip.
The acting seat (seat 0 in a phase) chooses as the action or phase that swayed the Unity resolves, before any track
tips, one decision for the whole amount: **any amount, no filter**.

**Tipping.** The Unity track holding `tip_per_player` × players Sway, or the Discord track holding
`discord_tip_per_player` × players (3 and 4), **tips**: one tip's worth comes off and the rest stays toward the next
tip, **on both tracks** (`carry` 2; a big Sway can tip more than once; 1 carries Unity's only, 0 neither); that side's
tip count goes up by one (the two counts are separate and only count up); a Discord tip arms the bases by neutral
factories ([war.md](war.md)); one card of its side is drawn and resolves (its strength: below). Tracks are checked
with win and loss (last paragraph), so a track may overfill within one action; if both are full, **Discord tips
first**; a tip's card may fill a track again, which then tips too.

**Sources of Sway**

<!-- generated: sway_sources -->
| Toward | From | Amount |
|---|---|---|
| Unity | removing an adversary troop | `alignment.troop` 0 |
| Unity | Biomia's converter | 1 research cube → 1 Sway to Unity (once a round) |
| Unity | liberating an adversary city (ambassadors.md: a liberate card or the Liberate action) | `alignment.free_city` 5 |
| Unity | liberating an adversary station or factory (the Liberate action) | `free_facility` 2 |
| Unity | removing a botnet | `alignment.botnet` 0 |
| Unity | player cubes on a megaproject as it completes, as its corp's card prints (Delta Seawall) | 1 each |
| Unity | cards (Peace Conference, Coalition Charter, Open Assembly), Unity cards | card text |
| Discord | energy diverted (a corrupted city) or darkened (an agent on an aligned city) | `divert_discord` 1 per unit of a corrupted city's, `ambassador_discord` 0 per unit, each round, of a dark aligned city's |
| Discord | a factory destroyed in war (not by an adversary troop) | `destruction_discord` 1 |
| Discord | adversary cubes on a megaproject as it completes, as its corp's card prints | 1 each |
| Discord | twists, threats, Discord cards, adversary projects | card text |
| Discord | the adversary board's drip, at the start of every round | as printed (The Militarist: 0) |
| Discord | each Fallout token, at the start of every round | `blight_discord` 1 |
<!-- /generated -->

The round's Sway is the phase `alignment.sway`, first in the round: the table's two start-of-round rows
([adversary.md](adversary.md)), nothing else. A threat sways Discord only where it prints it.

**Sway by narrative.** A source's side is its story: Unity for trust and cooperation (liberating, clearing
corruption, accords, shared breakthroughs), Discord for fear, force and collateral harm (threats, propaganda, the
adversary's projects, the players' own strikes); each content sway prints its tag (`narrative`).

**The cards (one-shot and hidden, like fear cards: locked).** Each side has one face-down deck, shuffled at setup
([`content/alignment.json`](../../content/alignment.json)), every printed card dealt. **Three strengths
(candidate):** every card prints strengths I, II and III; **the tip itself counts**, for both sides, by `tier_at`
(*How the game ends* below). The card resolves at once and
goes face up on its side's resolved pile; there are no ongoing cards. **The Discord deck never reshuffles: it is the
clock.** A Discord tip that finds it empty loses (**locked**). **The Unity deck reshuffles** its resolved pile when a
tip finds it empty (rarely, at 23 cards); drawing it out wins nothing. **Fear-card shape (cards candidates):** each
card is one verb scoped by a **board condition**, never reach or "one player"; its strengths widen the count or the
condition. Shapes: "for each X (at most N)" (`per_site`, counted on the whole board; "a site a player owns" counts a city a
player hosts, 254, since no city is owned); "remove up to N from each X / in
each country piece from X / each player from one X" (`remove_where`); **prohibitions** (`prohibit`, the firewall
generalized; *In the next adversary turn, no X*, [README.md](README.md#terms)): no Rally, Muster or Corrupt; at a
token's hex a Muster troop (any Muster) stays in the supply, a site doesn't corrupt (by a line, naturally, or by any
flip or threat) and no agent moves onto it. One made during an adversary turn comes off at that turn's end. Choices
are seat 0's (each player's own, theirs). Grassroots Funding and Think Tank move a printed income marker "for the
rest of the game".

<!-- generated: alignment -->
- **unity** (23): Grassroots Funding, Open Market, Think Tank, Ceasefire, Civil Defense, Volunteer Corps, Global Coalition, New Deal, Decommission, Peacekeepers (draft), Town Hall Accord (draft), Popular Mandate (draft), Reclamation (draft), Mutual Aid (draft), Disarmament Talks (draft), Network Cleanup (draft), Citizens' Assembly (draft), Rebuilding Fund (draft), Safe Corridors (draft), Reconciliation Panel (draft), Neighborhood Watch (draft), Rule of Law (draft), Demilitarized Zone (draft).
- **discord** (7): Incursion, Blight, Propaganda, Scorched Earth, Offensive, Two Fronts, Firestorm.
- Each card's three strengths: [cards.md](../cards.md#unity-and-discord-cards).
<!-- /generated -->

**Details:**

- A threat countered by a card is discarded and its partial payments are refunded.
- Open Market's free cards are keeps of drafted cards by any player, used first ([player-cards.md](player-cards.md)).
- **Unity pulled back:** "Remove N Unity from the Unity track" removes up to N.
- Extra threats come from the deck of the tier of this round's adversary card's first pull (an empty deck
  reshuffles), one per country in turn from country 0, countered or executed like any other; one dealt after the
  counter phase (by a tip after the action phase, or Offensive) waits a round ([threats.md](threats.md), *Executing*).
  False Flag's come before the counter phase: countered or executed that round.
- Blight and Firestorm create on the lowest-numbered city holding agents (a creation line); a country with none gains
  nothing. Blight also opens a neutral factory in each country.
- **Discord cards destroy no nodes and change no rule for the whole map:** their higher strengths add board pressure
  or take credits, scaled by the corrupted sites (*How the game ends* below); nodes fall only where the board is
  overrun ([war.md](war.md)). "A tier-2 threat in each country piece" pulls from the tier-2 deck, any tag.
- Offensive acts at once: Corrupt cities (strength III builds first: each country rallies its agent income to this
  round's card's Rally site, the face-up card before the build step), then deals its tier-2 threats, any tag, into the
  country of the node hex with the most agents and troops (ties: country, then lower hex), first token on that hex.
- **Unity strength III never frees a city:** it widens its card's condition or count, never to a whole step or the
  whole map (a prohibition's: on or next to, or within 2 of, an ambassador or a node); none clears around corrupted
  cities, and Reclamation's cubes are stations' and factories' only. The players still reach a city and Liberate it.

**The purge: the only win (locked), stepped by Unity tips** (`purge_at`, below) as Spirit Island's terror levels are;
at the last step the players win the moment the adversary owns no city. No round gate: the steps and liberating
cities ([ambassadors.md](ambassadors.md)) set how early it comes.

<!-- generated: purge_ease -->
- 0-2 Unity tips: remove the corrupted cities, botnets, adversary troops, agents;
- 3-6 Unity tips: remove the corrupted cities, botnets, adversary troops;
- 7+ Unity tips: remove only the corrupted cities;
- corrupted cities always count.
<!-- /generated -->

**Why:** botnets are hard to remove and troops respawn from adversary factories every round. The level-0 purge counts
only from an adversary card of stage `clear_win_stage` (1: from round 1).

<!-- generated: game_end -->
**How the game ends, for new players.** The only way to win is the purge, and Unity eases it, step by step, like terror levels. At 0 Unity tips you must purge almost everything: corrupted cities, botnets, adversary troops and agents. After 3 Unity tips the purge no longer asks you to clear agents; after 7, it asks only that the adversary owns no city, and you win the moment it doesn't. Pressure comes every round: the face-up adversary event card deals threats by your sites and says where its new agents land (dealt one at a time onto every site of a kind), so you always know where the next wave lands. Agents pile up, a card's Corrupt line moves every agent next to a city onto it, a city holding 4 corrupts, a hex with your node and 2 loses the node, and corrupted cities and uncountered threats fill the Discord track. Each time it fills, it tips: a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, you lose. Discord cards never destroy your nodes directly, only through the threats and corruptions they bring: they deal more or stronger threats (Offensive aims one at the node hex most crowded with adversary pieces), create agents, run Corrupt lines, take credits and land Fallout (Scorched Earth), more of each the more sites are corrupted. Unity has its own track, which nothing on the Discord side empties: free a corrupted city and you sway Unity (removing a botnet sways none), and each Unity tip brings a good card. Like fear cards, each acts by what is on the board, never by your reach; some forbid the adversary's next Rally, Muster or Corrupt where your pieces stand. Every Unity and Discord card prints three strengths, and the tip's own count on its side picks which one resolves (strength I on tips below 3, II from tip 3, III from tip 6): a later strength counts more or reaches wider, and Discord's late cards hit harder. The 23 Unity cards reshuffle when the deck runs out. Each Unity you sway may instead take 1 Discord off its track, and Envoy trades one of your ambassadors for the same, so you choose between racing toward Unity and holding off the next Discord card. You also lose if any player has no nodes left on the board. Nodes are lost only where the board is overrun: an uncountered threat near them, a corruption of their own hex (or two agents on it: Night Raid lands them), war taking a factory on their hex, and Border War (nodes by a front's base). A lost token leaves the game and nothing marks its hex; its income stays. No stage protects your last node: keep a second one on the board. The adversary deck is a round cap too, 13 rounds at most (fewer at the harder levels), and reaching it loses. So there are three different pressures: the Discord deck (politics), your nodes (the board) and the cap.
<!-- /generated -->

**Losing** (each **locked**): a Discord tip finds the Discord deck empty; a player has no nodes on the board (everyone
loses, as when a spirit is destroyed); the adversary deck runs out (the **round cap**, a plain loss,
[adversary.md](adversary.md)), found after the round's Sway step and its tips. Neither the supply running out (a cap)
nor every city corrupted is a loss: a corrupted board drives Discord.

Win and loss are checked whenever an action or automatic phase finishes resolving: the Unity just swayed is placed (or
holds Discord off), full tracks tip (Discord first), then losses, then the purge. A tip that ends the game ends it
there; a player with no node loses before a Unity tip is drawn.

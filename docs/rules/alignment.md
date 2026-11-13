# Alignment and the end of the game (module `alignment`)

*Sway, the two tracks and tipping, the Unity and Discord decks, the purge, and how the game is won and lost. Terms and conventions are in [README.md](README.md).*

Alignment is where the AIs' world leans: toward **Unity** (the good AIs, the players) or
**Discord** (the evil AI, the adversary). Everything on it is printed: two Sway tracks, two
tip-count tracks and two decks of one-shot cards, each printing three strengths.

**Two independent tracks (r55, candidate).** Discord Sway fills the **Discord track** and Unity
Sway the **Unity track**; both start empty, there is no shared pool and nothing on one side
empties the other. **Holding Discord off:** each Unity a player sways may instead move 1 Discord
off the Discord track (`hold_off` 1), and Envoy ([economy.md](economy.md), a standard action) does
the same for one of your ambassadors, so the team chooses between racing toward Unity and putting
off the next Discord card. Unity on its own track never stops a Discord tip. The choice is made as
the action or automatic phase that swayed the Unity resolves, before any track tips, asked of
seat 0 for the team (one decision for the whole amount): **any amount, no filter** (r75, Matt's ruling 7). Variant `off` moves no Sway
([README.md](README.md#off-variants)).

**Tipping.** The Unity track holding `tip_per_player` × players Sway, or the Discord track
holding `discord_tip_per_player` × players, **tips**: one tip's worth comes off and the rest stays toward
the next tip, **on both tracks** (`carry` 2, r75, Matt's ruling 7: a Discord tip emptied its track; a big Sway can
tip more than once; 1 carries Unity's only, 0 neither); that side's tip count
goes up by one; on a Discord tip each country with an unblocked neutral factory puts a drone on each of its bases ([war.md](war.md)); one card of its side is drawn and resolves (at the strength picked by its side's tip
count, this tip included, below). Unity tips and Discord tips are counted separately and only count up.
Tracks are checked when an action or automatic phase finishes resolving (with win and loss, below),
so a track may overfill within one action. If both are full, **Discord tips first**; a tip's card may
fill a track again, which then tips too. Thresholds: *Tunables*.

**Sources of Sway**

<!-- generated: sway_sources -->
| Toward | From | Amount |
|---|---|---|
| Unity | removing an adversary troop | `alignment.troop` 0 |
| Unity | Biomia's converter | 1 research cube → 1 Sway to Unity (once a round) |
| Unity | liberating an adversary city (ambassadors.md: a liberate card or the Liberate action) | `alignment.free_city` 5 |
| Unity | liberating an adversary station or factory (the Liberate action) | `free_facility` 2 |
| Unity | removing a botnet | `alignment.botnet` 0 |
| Unity | Breakthrough (1 antimatter, once a round) | `breakthrough_unity` 2 |
| Unity | clearing a corruption marker (the Clear action, Recovery, cards) | `clear_unity` 2 |
| Unity | player cubes on a megaproject as it completes, as its corp's card prints (Delta Seawall) | 1 each |
| Unity | cards (Peace Conference, Coalition Charter, Open Assembly), Unity cards | card text |
| Discord | energy diverted (a corrupted city) or darkened (an agent on an aligned city) | `divert_discord` 1 per unit of a corrupted city's, `ambassador_discord` 0 per unit, each round, of a dark aligned city's |
| Discord | a factory destroyed in war (not by an adversary troop) | `destruction_discord` 1 |
| Discord | adversary cubes on a megaproject as it completes, as its corp's card prints | 1 each |
| Discord | twists, threats, Discord cards, adversary projects | card text |
| Discord | the adversary board's drip, at the start of every round | as printed (The Militarist: 0) |
| Discord | each land-corruption token (Blitzkrieg, Naval Blockade), at the start of every round | `blight_discord` 1 |
<!-- /generated -->

The round's Sway is the phase `alignment.sway`, first in the round: the board's drip and `blight_discord` (1) for
each face-up land-corruption token (a node hex's is face down: `blight_node_discord`, 0, r84; [adversary.md](adversary.md)), nothing else. A threat sways Discord only where it prints it.

**Sway by narrative (r90, Matt's ruling, 2026-11-12).** Every source's side is what the act means in the story:
Unity for trust and cooperation (liberating, clearing corruption, accords, shared breakthroughs), Discord for fear,
force and collateral harm (threats, propaganda, the adversary's projects, and the players' own strikes: Surgical
Strike, Kettle Operation, Martial Cordon and Kinetic Interdiction 1, The Nuclear Option and Annihilation Lance 2).
Taking a botnet down tells neither (`botnet` 0). Each content sway prints its story tag (`narrative`); the table of
every source is in 204's write-up. Nothing sways for antimatter itself: the r88 antimatter action is gone.

**The cards (one-shot and hidden, like fear cards: locked).** Each side has one face-down
deck, shuffled at setup ([`content/alignment.json`](../../content/alignment.json)), every printed
card dealt. **Three strengths (r67, candidate):** every card prints a strength I, II and III effect, as
Spirit Island's fear cards; **the tip itself counts** (r68, Matt, 2026-11-09), for both sides: by
`tier_at`, strength I on a side's 1st and 2nd tips, II on its 3rd to 5th, III from its 6th. The card
resolves at once and goes face up on its side's resolved pile; there are no ongoing cards.
**The Discord deck never reshuffles: it is the clock.** A Discord tip that finds it empty loses
(**locked**, Matt); with 6 cards the 6th tip resolves a strength III and the 7th loses. **The Unity deck reshuffles** its resolved pile
when a tip finds it empty (r71: 23 cards, so it rarely does); drawing it out wins nothing.
**Fear-card shape (r71, Matt, 2026-11-09; cards candidates):** each card is one verb scoped by a **board condition**,
never reach or "one player"; its strengths widen the count or the condition. Shapes: "for each X (at most N)" (`per_site`,
counted on the whole board); "remove up to N from each X / in each country piece from X / each player from one X"
(`remove_where`; a cleared marker pays its reward); **prohibitions** (`prohibit`, 158's firewall generalized; r80,
Matt's ruling #53): a no-Rally, no-Muster or no-Corrupt token goes on every hex where the condition holds **when the
card resolves**; the step skips a hex holding its token: Rally's share for it stays in the supply (r82), a Muster troop (any Muster) stays
in the supply, a site doesn't corrupt (by a line, naturally, or by any flip or threat) and no agent moves onto it. The tokens come off with the firewalls, at
the end of the next adversary turn (one made during an adversary turn, at that turn's end). Choices are seat 0's (each player's own, theirs). Grassroots Funding and Think
Tank move a printed income marker "for the rest of the game". Cards marked (draft) are for Matt to keep or prune.

<!-- generated: alignment -->
- **unity** (23): Grassroots Funding, Open Market, Think Tank, Ceasefire, Civil Defense, Volunteer Corps, Global Coalition, New Deal, Decommission, Peacekeepers (draft), Town Hall Accord (draft), Popular Mandate (draft), Reclamation (draft), Mutual Aid (draft), Disarmament Talks (draft), Network Cleanup (draft), Citizens' Assembly (draft), Rebuilding Fund (draft), Safe Corridors (draft), Reconciliation Panel (draft), Neighborhood Watch (draft), Rule of Law (draft), Demilitarized Zone (draft).
- **discord** (6): Incursion, Blight, Propaganda, Offensive, Two Fronts, Firestorm.
- Each card's three strengths: [cards.md](../cards.md#unity-and-discord-cards).
<!-- /generated -->

A threat countered by a card is discarded and its partial payments are refunded.
**Details:**

- Open Market's free cards are keeps of drafted cards ([player-cards.md](player-cards.md)), by any player, used before
  credits.
- **Unity pulled back:** "Remove N Unity from the Unity track" removes up to N.
- Extra threats come from the deck of the tier of this round's adversary card's first pull (an empty deck reshuffles), one per country in turn from country 0,
  and are countered or execute like any other. One dealt after the round's counter
  phase (a tip after the action phase) takes a waits token and executes next round. False Flag's come before the
  counter phase: countered or executed that round.
- Blight and Firestorm create on the lowest-numbered city holding agents (138: a creation line); a
  country with none gains nothing. Blight also opens a neutral factory in each country.
- **Discord cards destroy no nodes (139, Matt) and change no rule for the whole map (156):** their higher strengths add
  pressure on the board (more or tier-2 threats, creation, Corrupt lines) or take credits, scaled by the corrupted
  sites (r71); nodes fall only where the board is overrun ([war.md](war.md)). "A tier-2 threat in each country piece"
  pulls from the tier-2 deck, any tag (146).
- Offensive acts at once: Corrupt cities (strength III: the adversary builds first, each country rallying its agent
  income to this round's card's Rally site), then a tier-2 threat (two from strength II), any tag, is dealt into the
  country of the node hex with the most agents and troops (ties: country, then lower hex number), its
  first token on that hex; dealt after the counter phase, it waits a round. Before the build step this round's card
  is the face-up one.
- **Unity strength III never frees a city** (r68, Matt, 2026-11-09): it widens its card's condition or count, never to a
  whole step or the whole map (a prohibition's strength III: on or next to, or within 2 of, an ambassador or a node); no strength III clears
  around corrupted cities (r68's clearing, rejected) and Reclamation's cubes are stations' and factories' only. The
  players still reach a city and Liberate it.

**The purge: the only win (r67, locked: Matt, 2026-11-08), stepped by Unity tips** as Spirit
Island's terror levels are. What you must remove from the board depends on the Unity tips made
(`purge_at`: agents, troops, botnets stop counting at those counts): at 0 almost everything;
from 6 only the cities, and the players win the moment the adversary owns no city. No round
gate: the steps and liberating cities ([ambassadors.md](ambassadors.md)) set how early it comes.

<!-- generated: purge_ease -->
- 0-2 Unity tips: remove the corrupted cities, botnets, adversary troops, agents;
- 3-5 Unity tips: remove the corrupted cities, botnets, adversary troops;
- 6+ Unity tips: remove only the corrupted cities;
- corrupted cities always count.
<!-- /generated -->

The steps are a list: agents, then troops, then botnets (troops and botnets both at 6).
**Why:** botnets are hard to remove and troops respawn from adversary factories every round. The
level-0 purge counts only from this round's adversary card of stage `clear_win_stage` (1: from round 1).

**No escalation in play:** Discord tips don't raise the difficulty; the cards' strengths escalate.

<!-- generated: game_end -->
**How the game ends, for new players.** The only way to win is the purge, and Unity eases it, step by step, like terror levels. At 0 Unity tips you must purge almost everything: corrupted cities, botnets, adversary troops and agents. After 3 Unity tips the purge no longer asks you to clear agents; after 6, it asks only that the adversary owns no city, and you win the moment it doesn't. Pressure comes every round: the face-up adversary event card deals threats by your sites and says where its new agents land (dealt one at a time onto every site of a kind), so you always know where the next wave lands. Agents pile up, a card's Corrupt line moves every agent next to a city onto it, a city holding 4 corrupts, a hex with your node and 2 loses the node, and corrupted cities and uncountered threats fill the Discord track. Each time it fills, it tips: a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, you lose. Discord cards never destroy your nodes directly, only through the threats and corruptions they bring: they deal more or stronger threats (Offensive aims one at the node hex most crowded with adversary pieces), create agents, run Corrupt lines, and take credits, more of each the more sites are corrupted. Unity has its own track, which nothing on the Discord side empties: free a corrupted city, clear corruption or remove a botnet and you sway Unity, and each Unity tip brings a good card. Like fear cards, each acts by what is on the board, never by your reach: for each site a player owns or each corrupted one, on each hex on or next to your pieces or a corrupted city, or each player from one such hex; some forbid the adversary's next Rally, Muster or Corrupt where your pieces stand, so place them before its turn. Every Unity and Discord card prints three strengths, and the tip's own count on its side picks which one resolves (strength I on tips below 3, II from tip 3, III from tip 6): a later strength counts more or reaches wider, and Discord's late cards hit harder. The 23 Unity cards reshuffle when the deck runs out. Each Unity you sway may instead take 1 Discord off its track, and Envoy trades one of your ambassadors for the same, so you choose between racing toward Unity and holding off the next Discord card. You also lose if any player has no nodes left on the board. Nodes are lost only where the board is overrun: an uncountered threat near them, a corruption of their own hex (or agents on it), war taking a factory on their hex, a project's raid (the node hex with the most agents), and Border War (nodes by a front's base). A lost token leaves the game and marks its hex, so you can't act from there any more; its income stays. No stage protects your last node: keep a second one on the board. The adversary deck is a round cap too, 11 rounds at most (fewer at the harder levels), and reaching it loses. So there are three different pressures: the Discord deck (politics), your nodes (the board) and the cap.
<!-- /generated -->

**Losing:**

- a Discord tip finds the Discord deck empty (**locked**);
- a player has no nodes on the board: everyone loses, as when a spirit is destroyed
  (**locked**);
- the adversary deck runs out (the **round cap**, a plain loss, **locked**; [adversary.md](adversary.md)), found
  after the round's Sway step and its tips.

The supply running out is not a loss (it is a cap, [adversary.md](adversary.md)), nor is every city corrupted: a
corrupted board drives Discord.

Win and loss are checked whenever an action or automatic phase finishes resolving: the Unity
just swayed is placed (or holds Discord off), full tracks tip (Discord before Unity), then
losses, then the purge. A tip that ends the game ends it there, and a player with no node loses
before a Unity tip is drawn.

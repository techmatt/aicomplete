# Alignment and the end of the game (module `alignment`)

*Sway, the two tracks and tipping, the Unity and Discord decks, and how the game is won and lost. Terms and conventions are in [README.md](README.md).*

**Unity** is the players' side, **Discord** the adversary's. All of it is printed: two Sway tracks, two tip-count
tracks and two decks of one-shot cards.

**Two independent tracks (candidate).** Discord Sway fills the **Discord track**, Unity Sway the **Unity track**; both
start empty, with no shared pool, and nothing on one side empties the other. **Unity never cancels Discord** (r139, Matt 2026-11-19): Unity swayed always
goes onto the Unity track, with no choice to make, and never stops a Discord tip. Nothing moves Discord off the
Discord track except a card that prints it: **four Unity cards do** (r143; Matt, 2026-11-19: about 4-5 of the 23,
each by a board condition): Volunteer Corps (1 for each site a player holds), Safe Corridors (1 for each hex
holding an ambassador), Reconciliation Panel (1 for each free city with no agent on it) and Town Hall Accord (2 for each city
holding an ambassador), each "at most N", N by strength; they take off what is there, never below 0.

**Tipping.** The Unity track holding `tip_per_player` × players Sway, or the Discord track holding
`discord_tip_per_player` × players (3 and 4), **tips**: one tip's worth comes off and the rest stays toward the next
tip, **on both tracks** (`carry` 2; a big Sway can tip more than once; 1 carries Unity's only, 0 neither); that side's
tip count goes up by one (the two counts are separate and only count up); one card of its side is drawn and resolves (its strength: below). Tracks are checked
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
| Discord | a corrupted city's output, or an aligned city darkened (an agent on it) | `divert_discord` 1 a corrupted city each round (the adversary's Income beat), `ambassador_discord` 0 per unit, each round, of a dark aligned city's |
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
condition. Shapes: "for each X (at most N)" (`per_site`, counted on the whole board; "a site a player owns" counts a free city with a
player's node on or next to it, once, since no city is owned; an agent or a blackout on it changes nothing here); "remove up to N from each X / in
each country piece from X / each player from one X" (`remove_where`); **prohibitions** (`prohibit`, the firewall
generalized; *In the next adversary turn, no X*, [README.md](README.md#terms)): no Infiltrate, Strike or Corrupt; at
a token's hex an Infiltrate's agent stays in the supply, no troop strikes a piece (a no-Strike token; the troop
stays where it is), and a site doesn't corrupt
(by an arrival, a flip or a threat; a node there isn't lost to agents), though agents still land on a no-Corrupt hex. One made during an adversary turn comes off at that turn's end.
**The floor (r151, `floor`):** every Unity card's strengths end "If there are none" ("If nothing comes off"; on a
prohibition "If no city, research station or factory qualifies": the sites under its tokens) "every player gains 2
credits instead": it pays when the count before it is 0, so no tip reads, counts and does nothing. Choices
are seat 0's (each player's own, theirs). Grassroots Funding and Think Tank move a printed income marker "for the
rest of the game".

<!-- generated: alignment -->
- **unity** (23): Grassroots Funding, Open Market, Think Tank, Ceasefire, Civil Defense, Volunteer Corps, Global Coalition, New Deal, Decommission, Peacekeepers (draft), Town Hall Accord (draft), Popular Mandate (draft), Reclamation (draft), Mutual Aid (draft), Disarmament Talks (draft), Network Cleanup (draft), Citizens' Assembly (draft), Rebuilding Fund (draft), Safe Corridors (draft), Reconciliation Panel (draft), Neighborhood Watch (draft), Rule of Law (draft), Demilitarized Zone (draft).
- **discord** (7): Incursion, Captured Labs, Propaganda, Scorched Earth, Offensive, Standing Army, Firestorm.
- Each card's three strengths: [cards.md](../cards.md#unity-and-discord-cards).
<!-- /generated -->

**Details:**

- A threat countered by a card is discarded and its partial payments are refunded.
- Open Market's free cards are keeps of drafted cards by any player, used first ([player-cards.md](player-cards.md)).
- **Unity pulled back:** "Remove N Unity from the Unity track" removes up to N.
- Extra threats are the next cards of the deck of the tier of this round's adversary card's first threat (an empty
  deck reshuffles), one per country in turn from country 0, countered or executed like any other; one dealt after the
  counter phase (by a tip after the action phase, or Offensive) waits a round ([threats.md](threats.md), *Executing*).
  False Flag's come before the counter phase: countered or executed that round.
- Firestorm creates on the lowest-numbered city holding agents (a creation line); a country with none gains
  nothing. Captured Labs puts 2 cubes on the research track for each corrupted research station (at most 2, 3, 4);
  Standing Army is the deck's troop card ([troops.md](troops.md)).
- **Discord cards destroy no nodes and change no rule for the whole map:** their higher strengths add board pressure
  or take credits, scaled by the corrupted sites (*How the game ends* below); nodes fall only where the board is
  overrun ([troops.md](troops.md)). "A tier-2 threat in each country piece" deals the tier-2 deck's next cards.
- Offensive deals its tier-2 threats (1, 2, 3 by strength: its one verb since r143) onto the lowest-numbered node hex
  holding an agent or a troop (country, then hex number; r144, as Night Raid reads: the last "most" pick went), first
  token on that hex; with none, it deals nothing. A located line acts
  only on its own kind of hex (r137, D132): unless that hex is the kind its first location names, the lines placing
  on it ("here", "the factory here": Overrun's corruption, Closed City's agents, any such
  destruction) do nothing; a line measured from it (a factory nearest here,
  Fallout next to it, a botnet's nearest other piece) still measures from it, and it marks who is near and the anchor. Discord cards corrupt
  nothing outright: their creations and Escalates are arrivals, so a city they bring to its number corrupts.
- **Unity strength III never frees a city:** it widens its card's condition or count, never to a whole step or the
  whole map (a prohibition's: on or next to, or within 2 of, an ambassador or a node); Civil Defense's removal by
  corrupted cities stays by them and only raises the count, and
  Reclamation's cubes are stations' and factories' only, and never a troop-guarded site's. The players still reach a city and Liberate it.

**The only win: no corrupted city** (Matt, 2026-11-19). The players win the moment no city on the board is
corrupted. Botnets, agents and troops never count, at any Unity tip count: Unity tips only set card strength. Each
country starts with `board.start_corrupted` (2) corrupted cities; liberating cities ([ambassadors.md](ambassadors.md)) is the whole route, and a troop on or next to a corrupted
city must go first ([troops.md](troops.md), *Guarding*). No round gate: the win counts from an adversary card of
stage `clear_win_stage` (1: from round 1). The engine's end reason is `liberated`.

**Why:** the win has no steps to track. An early win at d0 by coordinated players is fine, though not easy (Matt).

<!-- generated: game_end -->
**How the game ends, for new players.** You win the moment no city on the board is corrupted: liberate every corrupted city. Botnets, agents and troops never count, but a troop guards: a corrupted site with a troop on or next to it can't be liberated until the troop is removed or pushed away. Troops come late (the adversary's research row, some projects and Discord cards), sit where they are placed, and on a Strike icon, shown a round ahead on the face-up event card, each destroys one of your nodes, research stations or factories on or next to it. Pressure comes every round: the face-up adversary event card deals threats by your sites and names where its new agents land (one on every site of a kind), and next round every one of those sites still holding an agent takes one more, so you always know where the next wave lands. Agents pile up only where something lands them: a city they land on holding 5 at level 0 (3 from level 1) corrupts, a hex with your node they land on holding 2 loses the node, and corrupted cities, Fallout and the threats that print it fill the Discord track. Each time it fills, it tips: a Discord card hurts you, and the Discord deck is small: when a tip finds it empty, you lose. Discord cards never destroy your nodes directly, only through the threats and corruptions they bring: they deal more or stronger threats (Offensive aims one at the lowest-numbered node hex holding an agent or a troop), create agents, escalate them onto a city, take credits, land Fallout (Scorched Earth), feed the research track by its corrupted stations (Captured Labs) and bring troops onto a fallen factory or a corrupted city you stand by (Standing Army), more of each the more sites are corrupted. Unity has its own track, which nothing on the Discord side empties: free a corrupted city and you sway Unity (removing a botnet sways none), and each Unity tip brings a good card. Like fear cards, each acts by what is on the board, never by your reach; some forbid the adversary's next Infiltrate, Strike or corruption where your pieces stand. Every Unity and Discord card prints three strengths, and the tip's own count on its side picks which one resolves (strength I on tips below 3, II from tip 3, III from tip 6): a later strength counts more or reaches wider, and Discord's late cards hit harder. The 23 Unity cards reshuffle when the deck runs out. Unity you sway always goes onto the Unity track: nothing takes Discord off its track but a Unity card that says so (four do, each by what stands on the board). You also lose if any player has no nodes left on the board. Nodes are lost only where the board is overrun: an uncountered threat near them, a corruption of their own hex (or two agents on it: Night Raid adds one where one stands) and a troop's Strike on or next to it. A lost token leaves the game and nothing marks its hex; its income stays. No stage protects your last node: keep a second one on the board. The adversary deck is a round cap too, 10 rounds at most (fewer at the harder levels), and reaching it loses; and the adversary's research track loses the game at its seventh slot. So there are four different pressures: the Discord deck (politics), your nodes (the board), the research track (its stations) and the cap.
<!-- /generated -->

**Losing** (each **locked**): a Discord tip finds the Discord deck empty; a player has no nodes on the board (everyone
loses, as when a spirit is destroyed); the adversary deck runs out (the **round cap**, a plain loss,
[adversary.md](adversary.md)), found after the round's Sway step and its tips; the adversary's research marker
passes its sixth slot (**slot 7**, end reason `research`, r140; found in its research check). Neither the supply running out (a cap)
nor every city corrupted is a loss: a corrupted board drives Discord.

Win and loss are checked whenever an action or automatic phase finishes resolving: the Unity just swayed is placed, full tracks tip (Discord first), then losses, then the win. A tip that ends the game ends it
there; a player with no node loses before a Unity tip is drawn.

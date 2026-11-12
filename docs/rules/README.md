# Rules as implemented

The rules are split by area. Read this page, then only the files your task touches:

- This page: scope, conventions, the new-player summary, [Terms](#terms) (the house words, reach, site claims,
  pieces, ownership, the ladder and who chooses) and the [off-variants](#off-variants).
- [board.md](board.md): country pieces, fronts and bases, setup, research stations, megaprojects and converters.
- [ambassadors.md](ambassadors.md): the one site model (cities, stations, factories), claiming, liberating, blocking,
  armor, canceling, removing agents.
- [adversary.md](adversary.md): the adversary board, its income and event deck (the round cap), Rally X, Corrupt X, botnets,
  the project slots, difficulty.
- [war.md](war.md): production and war every round, factories, bases as troop piles, war, the adversary march, Wall,
  node loss.
- [alignment.md](alignment.md): Sway, tipping, the Unity and Discord decks, the purge, winning and losing.
- [economy.md](economy.md): each resource's sources and uses, city energy, standard actions, the round's phases.
- [ai-boards.md](ai-boards.md): the three boards, their special rules, node tracks and capstones.
- [ai-powers.md](ai-powers.md): the boards' powers (node, sacrifice and insight), the insight conversions and loops,
  shelters and hardening.
- [threats.md](threats.md): the threat decks, dealing, location tokens, who is near, the counter menu, what threats do.
- [player-cards.md](player-cards.md): the opening, the draft, keeps, play, slots, tags, tokens, the liberate
  capstones.
- [card-grammar.md](card-grammar.md): how a card's printed words resolve: costs, reach, targets, effects.
- [tunables.md](tunables.md): every baseline config parameter with its value, range and meaning (generated).

What the code implements, at rules revision 85 (`RULES_REVISION`, `crates/aic-core/src/lib.rs`):
the Militarist against Builder, Protector and Researcher (two countries seat Builder and
Protector, three add the Researcher), 1-4 countries. The player cards are in
[cards.md](../cards.md), priced in [card-pricing.md](../card-pricing.md). The
AI boards are **placeholders** pending redesign; the Unity and Discord decks are candidates in fear-card shape
(r71, Matt's direction locked); the adversary's
threat decks (rewritten by 146 to Matt's threat card standard, locked), event deck, board and project deck are drafts
(`drafts/adversary/`), all in play until Matt's picks prune them. Every rule is a **candidate** unless marked **locked**; which, and what
each revision changed and why, is in [rules-history.md](../rules-history.md). What content can
print is in [ops.md](../ops.md).

**Every number below is a candidate**: a starting calibration reasoned for an 8–10
round game (`docs/principles.md`), for simulation to adjust. Names in `code` are config
tunables (`<module>.<param>`), with their values under [Tunables](tunables.md), from
[`configs/baseline.json`](../../configs/baseline.json); card, threat, adversary-card,
megaproject and AI-board numbers are content in [`content/`](../../content/).
Design intent lives in [principles.md](../principles.md); the engine in
[architecture.md](../architecture.md). **Keep these files current with every rules change**, each under 15 KB.

<!-- generated: new_players -->
**For new players.** You start with 2 tier-1 and 2 tier-2 cards in your hand, free: the tier-2 cards are something to plan toward, to play once you meet their needs. Then every round, the first included, you build a hand of 4 cards one at a time, choosing for each whether it comes from the cheaper tier-1 deck or the stronger tier-2 deck; you take one card and pass the rest to the next player in seat order until the hands are gone, then pay insight to keep what you drafted (1 for tier 1, 2 for tier 2; nothing else buys a keep). Insight is this round's attention: your card-play track shows it, the number on the rightmost space you've uncovered (6 before any), and what you don't spend is lost. Playing a card costs its credits and 0-2 insight, from the same insight as keeps. Buildings go into your AI board's 3-5 slots; once they are full, a new building goes on top of an old one, which stops working but keeps its tags. Programs stay in front of you and are where lasting income comes from ("raise your credit income by 1 per 2 Science tags on your cards"); buildings are your engine: Actions you use once a round and Effects that pay each time their trigger happens; events are used once and set aside, never to return. Every card in front of you shows tags (Science, Industry, Cyber, Power, Diplomacy, Security): they unlock some cards, and you can sacrifice cards with the right tags to use your AI board's powers. There is no limit on what you hold or on how far cards raise your income; energy you don't spend stays energy, so drafting cards that spend it is up to you.

**Nodes, for new players.** Nodes don't grow on their own. In the action phase you place nodes with your board's two node powers, each once a round, so 0, 1 or 2 tokens a round from them, and cards can add more. Every board's Expand costs 1 energy; the second costs a tag or the board's own currency. Every placement, by power or card, pays the energy printed on the track space you uncover. You start with 3 energy and one city bound to you, which makes energy every round.

**Dispatches, tags and sales, for new players.** A Dispatch puts one of your ambassadors, from your supply or anywhere on the map, onto a piece or botnet in your reach and costs nothing, but you have only your board's Dispatches each round (1-2 printed, more as Dispatch spaces are uncovered), and what you don't use is lost, like insight. **One tag rule:** the tag printed on your AI board, and any a late, costly track space reveals, count like the tags on your cards, for everything a tag does; the only difference is that a board tag can't be sacrificed. You may sell a card from your hand for 1 credit; chips can't be sold.
<!-- /generated -->

## Terms

The house words, as the player aid prints them ([player-aid.md](../player-aid.md)):

<!-- generated: house_terms -->
- **Facility**: a research station or factory on the board. A building is always a card. Harden goes on a facility and nothing else.
- **Ambassador**: a player's piece, any player's: a card never says whose.
- **Agent**: the adversary's piece on the board (the Security token is an Operative).
- **Troop**: an adversary troop; a megacorp's is a Drone, its own piece, waiting at its military bases.
- **In reach**: on a hex within 1 of your nodes.
- **A country you're in**: a country where you have a node.
- **Your home country**: the country you start in.
- **A threat**: on your card: one you may counter, near it (your node or ambassador on or next to one of its tokens).
- **Near it**: on a threat: on or next to one of its location tokens.
- **Here**: on a building: on this card, where its tokens sit.
- **Remove N**: as many as there are, up to N.
- **At N**: on a building's Effect: when its tokens here reach N, N of them come off and the rest happens.
- **Draw**: from the tier-1 deck, with no keep to pay; if it is empty, reshuffle its discard first.
- **On your cards**: on tags: on your cards in play, covered buildings included, and your board's tags, printed or revealed by its tracks. Tokens count only on the card that holds them.
- **Action**: once a round, always; "A, OR B" is one use, either half.
- **Open megaproject**: one that isn't complete; research cubes in your color go only onto one.
- **Liberate**: a corrupted site: the agents on it come off, the adversary's cube comes off and yours goes on (the action pays for it in ambassadors; a card's is unpaid).
- **Convert**: an agent on or next to the city leaves its stack and becomes your ambassador where it stood.
- **Gather up to N**: pieces (ambassadors, agents, troops; botnets and markers never move) into a hex in your reach, each from a hex next to it; you may stop at any piece (a strike, "remove up to N on or next to a hex", moves nothing).
- **Push up to N**: pieces from a hex in your reach, each to a hex next to it that you choose; you may stop at any piece.
- **Firewall**: a token on a hex: no agent lands or moves there (a Rally's share for it stays in the supply) and nothing corrupts or flips its site; it comes off at the end of the next adversary turn.
- **In the next adversary turn, no X**: a Unity card's prohibition: a token on each hex where the condition holds when the card resolves; that step skips a hex with its token (a Rally's share for it stays in the supply), and the tokens come off with the firewalls.
<!-- /generated -->

The rest:

- **Adjacent**: a neighboring hex on the board, across country borders too.
- **Next to X**: holding X, or adjacent to a hex holding X.
- **Node**: one of a player's tokens on the board (one per hex, of any player), taken off their AI
  board's node track ([ai-boards.md](ai-boards.md)). A node is a token, not a facility: it sits on any land
  hex, with or beside other pieces.
- **Botnet**: the adversary's persistent token ([adversary.md](adversary.md)), placed only by printed lines. Nodes are always the players', botnets always the adversary's.
- **Reach**: every map target of a standard action is in the acting player's reach, and so
  are most of a card's. The exceptions are Spread (Exodus Corridors, measured from
  the ambassador) and country effects such as a tier-2 sweep or Shelter (any country;
  [card-grammar.md](card-grammar.md)). Reach is 1 for every player rule: no deck card's
  move goes further. Reach bounds where a piece goes, not where it comes from: a Dispatch, or a card's "move up to N ambassadors", takes one from anywhere. The adversary's distances (a Corrupt line moving agents from next door, a troop's
  strike) are its own, not a player's reach ([adversary.md](adversary.md), [war.md](war.md)). A converter needs a city of its country in your
  reach ([board.md](board.md)).
- **Site**: a city, research station or factory (137: one site model, [ambassadors.md](ambassadors.md)). A military
  base is no site: it is a front's troop pile ([war.md](war.md)).
- **Site claim**: claiming a neutral site (the Claim action) needs the site in the
  player's reach and no embargo on it; a botnet on or next to it doesn't stop a claim (r75). Liberating an
  adversary site needs reach only. Both pay with the player's own ambassadors on the site first, then the supply.
- **Piece**: a site, a military base or a node. Ambassadors sit on pieces.
- **Clean / corrupted / aligned / owned**: a city, research station or factory with no
  owner cube is clean (a station or factory: neutral); an adversary cube means corrupted
  (a factory: adversary-owned); a player cube means aligned (a city) or owned.
- **The ladder**: the adversary's pieces, weakest first: agents (the explorers, used
  up when they act), **botnets** (the towns, persistent), troops, corrupted cities and
  stations (the cities). The Liberate action takes a site off the ladder: its agents cancel and it becomes the
  liberator's ([ambassadors.md](ambassadors.md)); a liberate card does the same, unpaid.
- **The supply is a cap**: a placement it can't make doesn't happen.
- **Players choose**: an `Action::Choose` in `legal_actions`. Team choices in the
  adversary's steps, production and threats are asked of seat 0 (an aligned city's
  factory placement, of its owner; which protection a destruction spends, of the piece's
  owner, [war.md](war.md)); a choice inside a player's own card, ability or
  standard action is asked of that player.
- **Hex number**: each piece's hexes are printed 1–16 row by row; ties go to the lower
  number.

## Off-variants

For measuring what a module adds, these variants remove its effect and keep the round: `threats.variant=off` (no
threats), `alignment.variant=off` (no Sway moves, nothing tips, no antimatter action; the purge, the node loss and the
adversary deck-out stay; with no tips the Discord-deck loss can't happen and the purge never eases), `cards.variant=off`
(no draft or drawing, so no player cards at all). Two don't isolate their module: with
Alignment off nothing tips, so botnets always count against the purge; with cards off only Decommission and the Take down a botnet action
remove a botnet. `board`, `adversary`, `alignment` and `ai` are not removable (each is read by another); `threats=off`,
`cards=off` and `economy=off` remove the module. Parts switch off by parameters (`destruction_discord` 0, `factory_chips` 0,
`board.troop_steps` 0 (no march), `adversary.botnet_compute` 0).

# Rules as implemented

The rules are split by area. Read this page, then only the files your task touches:

- This page: scope, conventions, the new-player summary, [Terms](#terms) (the house words, reach, site claims,
  pieces, ownership, the ladder and who chooses) and the [off-variants](#off-variants).
- [board.md](board.md): country pieces, fronts and bases, setup, research stations, megaprojects and converters.
- [ambassadors.md](ambassadors.md): the one site model (cities, stations, factories), claiming, liberating, blocking,
  armor, canceling, evicting.
- [adversary.md](adversary.md): the adversary board, its income and event deck (the round cap), Rally X, Corrupt X, botnets,
  the project slots, difficulty.
- [war.md](war.md): production and war every round, factories, bases as troop piles, war, the adversary march, Wall,
  node loss.
- [alignment.md](alignment.md): Sway, tipping, the Unity and Discord decks, the purge, winning and losing.
- [economy.md](economy.md): each resource's sources and uses, city energy, standard actions, the round's phases.
- [ai-boards.md](ai-boards.md): the three boards, their special rules, node tracks and capstones.
- [ai-powers.md](ai-powers.md): the boards' powers (node, sacrifice and insight), the insight conversions and loops,
  shelters, hardening and shields.
- [threats.md](threats.md): the threat decks, dealing, location tokens, who is near, the counter menu, what threats do.
- [player-cards.md](player-cards.md): the opening, the draft, keeps, play, slots, tags, tokens, the liberate
  capstones.
- [card-grammar.md](card-grammar.md): how a card's printed words resolve: costs, reach, targets, effects.
- [tunables.md](tunables.md): every baseline config parameter with its value, range and meaning (generated).

What the code implements, at rules revision 69 (`RULES_REVISION`, `crates/aic-core/src/lib.rs`):
the Militarist against Builder, Protector and Researcher (two countries seat Builder and
Protector, three add the Researcher), 1-4 countries. The player cards are in
[cards.md](../cards.md), priced in [card-pricing.md](../card-pricing.md). The
AI boards and the Unity and Discord decks are **placeholders** pending redesign; the adversary's
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
**For new players.** In the first round you are dealt 8 tier-1 cards and keep 4 free (more at 5 credits each), and 2 tier-2 cards, each kept for 1 insight, to play once you meet its needs: something to plan toward. From then on, each round you build a hand of 4 cards one at a time, choosing for each whether it comes from the cheaper tier-1 deck or the stronger tier-2 deck; you take one card and pass the rest to your neighbor until the hands are gone, then pay insight to keep what you drafted (1 for tier 1, 2 for tier 2). Insight is this round's attention: what you don't spend is lost. Playing a card costs its credits and 0-2 insight, from the same insight as keeps. Buildings go into your AI board's 3-5 slots; once they are full, a new building goes on top of an old one, which stops working but keeps its tags. Programs stay in front of you and are where lasting income comes from ("raise your credit income by 1 per 2 Science tags on your cards"); buildings are your engine: Actions you use once a round and Effects that pay each time their trigger happens; events are used and discarded. Every card in front of you shows tags (Science, Industry, Cyber, Power, Diplomacy, Security): they unlock some cards, and you can sacrifice cards with the right tags to use your AI board's powers. There is no limit on what you hold or on how far cards raise your income; energy you don't spend stays energy, so drafting cards that spend it is up to you.

**Nodes, for new players.** There is no growth step. In the action phase you place nodes with your board's two node powers, each once a round, so 0, 1 or 2 tokens a round from them, and cards can add more. Every board has 1 energy for 1 node; the second costs more, in the board's own currency. Either way you also pay the energy printed on the track space you uncover. You start with 3 energy and one city bound to you, which makes energy every round.

**Movement, tags and sales, for new players.** Moving an ambassador costs nothing, but you have only your board's Movement each round (1-2 printed, more as Movement spaces are uncovered): one move each, and what you don't use is lost, like insight. **One tag rule:** the tag printed on your AI board, and any a late, costly track space reveals, count like the tags on your cards, for everything a tag does; the only difference is that a board tag can't be sacrificed. You may sell a card from your hand for 1 credit; chips can't be sold.
<!-- /generated -->

## Terms

The house words, as the player aid prints them ([player-aid.md](../player-aid.md)):

<!-- generated: house_terms -->
- **Facility**: a research station or factory on the board. A building is always a card. Harden goes on a facility and nothing else.
- **Shield**: a hardening token on a hex with your node.
- **Ambassador**: yours; the other side's is always an adversary ambassador.
- **Troop**: an adversary troop, unless it says corp troop (a megacorp's, waiting at its military bases).
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
- **Convert**: an adversary ambassador on or next to the city leaves its stack and becomes yours, on your node whose reach you used.
- **Gather at a city**: every adversary ambassador on a hex next to it moves onto it (troops too, when the card names them).
- **Firewall**: a token on a hex: no adversary ambassador lands or is pulled there; it comes off at the end of the next adversary turn.
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
  the ambassador), and country effects such as a tier-2 sweep or Shelter (any country;
  [card-grammar.md](card-grammar.md)). Reach is 1 for every player rule: no deck card's
  move goes further. The adversary's distances (a Corrupt line's gathering from next door, `war_reach`) are its dials, not
  a player's reach ([adversary.md](adversary.md), [war.md](war.md)). A converter is not a map target: it needs a node in a city of
  its country ([board.md](board.md)).
- **Site**: a city, research station or factory (137: one site model, [ambassadors.md](ambassadors.md)). A military
  base is no site: it is a front's troop pile ([war.md](war.md)).
- **Site claim**: claiming a neutral site (the Claim action) needs the
  player's node on or next to the site, and **a botnet next to the site blocks claims
  on it** (one hop from the site). Liberating an adversary site needs reach only.
- **Piece**: a site, a military base or a node. Ambassadors sit on pieces.
- **Clean / corrupted / aligned / owned**: a city, research station or factory with no
  owner cube is clean (a station or factory: neutral); an adversary cube means corrupted
  (a factory: adversary-owned); a player cube means aligned (a city) or owned.
- **The ladder**: the adversary's pieces, weakest first: ambassadors (the explorers, used
  up when they act), **botnets** (the towns, persistent), troops, corrupted cities and
  stations (the cities). Liberating a city knocks it down the ladder, not off it ([ambassadors.md](ambassadors.md)).
- **The supply is a cap**: a placement it can't make doesn't happen.
- **Players choose**: an `Action::Choose` in `legal_actions`. Team choices in the
  adversary's steps, production and threats are asked of seat 0 (an aligned city's
  factory placement, of its owner); a choice inside a player's own card, ability or
  standard action is asked of that player.
- **Hex number**: each piece's hexes are printed 1–16 row by row; ties go to the lower
  number.

## Off-variants

For measuring what a module adds, these variants remove its effect and keep the round: `threats.variant=off` (no
threats), `alignment.variant=off` (no Sway moves, nothing tips, no antimatter action; the purge, the node loss and the
adversary deck-out stay; with no tips the Discord-deck loss can't happen and the purge never eases), `cards.variant=off`
(no draft or drawing, so no player cards at all). Two don't isolate their module: with
Alignment off nothing tips, so botnets always count against the purge; with cards off only Decommission removes a
botnet. `board`, `adversary`, `alignment` and `ai` are not removable (each is read by another); `threats=off`,
`cards=off` and `economy=off` remove the module. Parts switch off by parameters (`destruction_discord` 0, `factory_chips` 0,
`board.troop_steps` 0 (no march), `adversary.botnet_compute` 0).

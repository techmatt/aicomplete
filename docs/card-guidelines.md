# Card guidelines

The checklist for every card, read as an experienced hobby player opening the box would.
Matt's principles (2026-10-31) with his card rulings 1-12 (2026-11-02) merged in. A
card that breaks one is a bug, or a design call for Matt, never a quiet exception.

1. **No exceptions: no `*`.** Every symbol means exactly what it always means. A card that
   needs a footnote is redesigned, repriced or simplified. If the exception is the fun, find
   another way to get that play and take it to Matt.
2. **Plain words, one reading only.** A player reads a card once and knows what to do. Name the pieces
   and the number: "remove 2 agents or 1 troop", never "2 worth". A "per" or an OR binds to exactly
   one thing; "you have" is spelled out on the face ("on your cards"); every icon keeps its locked meaning (ruling 5:
   *Kinetic Interdiction* read as in every hex or in one). Fix a phrase in `face.rs`, and every card using it improves.
3. **No arithmetic.** At most a simple "per X". No sums, series, formulas or running totals. **No "per hex of a
   country"** (ruling 6): every country has 16 hexes, so "1 per hex" is a hidden fixed number; print the number
   ("remove 6 troops in one country"). **Big numbers over "remove all X"** (*Martial Cordon*).
4. **No trivial cards; every rider earns its text.** Drafting costs a keep (1 insight at tier 1, 2 at tier 2; never credits) before the play
   cost, so a card must be worth that (ruling 1: *Surplus Auction*). A rider or conditional extra that adds
   symbols and parsing but no decision ("gain 1 credit") is noise: fold its value into the base number, or cut it. **Fold a near-constant rider into the number**
   (ruling 3: *Blackout Script*: a botnet makes 1 compute, so "no compute next build" is one more compute removed).
5. **No circular payments** (ruling 2). Paying credits to gain credits back is netted out, never printed both ways
   (*Peace Dividend*: pay 3, gain 2 credits).
6. **Clear timing.** Each effect reads as once now (below the art), each round, or as an Action.
   "Pay N, gain M" is a one-off; don't print a rate where a fixed trade will do (24).
7. **No caps by default.** Price the uncapped effect. A card that breaks without a cap gets a
   lower rate or a new effect, not a cap. Nor does the game cap it (Matt; [rules/economy.md](rules/economy.md),
   *No limits*), so a runaway pile is fixed at the cards that feed it. A card that draws costs
   at least 1 insight to play, so a round's chain of plays stops when its insight does.
8. **One clear idea per card.** Two effects at most, unless a third is the extra cost. A building
   holds income or tokens, and one Action or Effect; an Action may be "A, OR B" when it is a token card's choice
   (rule 15).
9. **Fits at the default size.** Rules text prints at 7.5 pt. A building's box above the art holds
   about 80 characters on two lines; the box below the art about the same; a program or event about
   110 (guides nothing checks). Cut words, split ideas or simplify the op rather than shrink the type. A card that
   still can't fit is a recorded exception, its reason in the item's write-up. Measure with `npm run cards:fit` in
   `web/` (box sizes: `art/frames/r3/fit.json`, `web/card/card.js`). A kind with no frame (threats, Alignment,
   adversary and project cards) has no budget until it gets one.
10. **The name says what it does,** and the art matches the name.
11. **No near-duplicates without a reason.** Same effect at another price or kind is fine only
    when the kind matters (an event leaves no tag; a program stays). A card that repeats an AI board's
    power needs a real price, or goes (*Shelter Order*).
12. **Tier is power; costs and effects are big** (ruling 4). Some complexity is fine but
    not required. Tier-2 income is far stronger, and few cards cost 0 credits; tier is not a proxy for cost (23). The curve in use is in
    [card-pricing.md](card-pricing.md), *The method*.
13. **A card beats the plain route it replaces** (direct payment, standard action, board power), counting its
    keep and insight (ruling 7; *Threat Hunt*'s "counts as 5 credits toward a cyber threat" lost to paying 5 credits).
14. **Outright threat counters stay rare** (ruling 8): a special case, not the common fix. A typed counter is a big
    jam of its type plus a rider of its tag, never a full counter (*Rapid Response*).
15. **No recounting across cards** (ruling 12). A token effect counts only the tokens **on that card**, never "you
    have" across cards: recounting every round is tracking load. Token cards are progressive with a choice:
    "Action: add 1 Operative here, OR get 1 credit per Operative here" (*Agency Field Station*), with other cards that add
    tokens to one card (*Recruit Operatives*). Nothing gains tokens by itself each round. A program that cashes
    tokens in takes them from one card.
16. **Tag chains** (ruling 10). Science, and other tags where it fits, has requirement chains: cards that need 3-4
    of a tag, and a few that need 6, giving a powerful effect once the requirement is met. The requirement is the price; the credit cost stays modest. A chain card's `needs` names one tag (the cost
    rule: N of one tag or N of any, never a mix).
17. **Per-tag payoffs** (ruling 11). Cards that pay "1 income per X tag" each round, and one-off cards that produce
    "1 energy (or another resource) per X tag on your cards".
18. **Critical analysis of every card** (ruling 9). Think each card through as a player would: what it does, step by
    step; its full cost against the plain route (13); when in an 8-10 round game it matters, and for which board;
    its riders (4); whether the text has exactly one reading (2); what decision it creates.

## Matt's rulings of 2026-11-04

A card that prices out on the value table but reads absurd to a Terraforming Mars or Spirit Island player is a bug (*Eviction Worm*: "remove 8 agents in one country" for 10 credits).

19. **Useful alone.** A card is never useless alone, even if it is better with others; tag payoffs ("1 income per X
    tag") are the exception (*Collateralized Grid* needed a Capital card).
20. **Feeders carry a main effect.** A token feeder ("add N X to one of your X buildings") has a standalone main effect
    (credit or insight income, say) priced a little below the rate; its tokens are a bonus that beats the rate with a
    target, as Terraforming Mars' Large Convoy does (*Dataset Scraper*, *Recruit Operatives*).
21. **Removal is in reach.** Removing troops or ambassadors is "in reach", not "in one country", bar a few big
    tier-2 sweeps kept as a deliberate power step (*Counterinsurgency Campaign*).
22. **Numbers make sense, not only price out.** A card beats its anchors (below) by about 1.5× at tier 1 and about 2.5×
    at tier 2, never more: beyond that, report it rather than ship it. A ceiling, not a target: a card at 1.0-1.5 is
    fair, its price never cut toward the ceiling. A dearer card has a stronger or different effect,
    never the same effect with a multiplied number (*Diplomatic Expulsion* went from 4 to 8 ambassadors). A
    printed number larger than what is usually within reach is a disguised "remove all", or the piece is too common:
    size every removal against the board state below.
23. **Tier is efficiency, not magnitude.** Tier 1 is not "cheap": a tier-1 card at 40 credits for more than +4 income is
    fine. Tier 2 is about efficiency and relative potency. Don't cap numbers or income by tier.
24. **Exchanges are one use.** A major exchange on a card is a fixed Action, "Action: spend X, gain Y" (once a round),
    never "convert X to Y freely at a ratio". The megacorp converters are the exception (a board rule, not a card);
    a one-shot program that empties one card's tokens (*Collateralized Grid*) is one use, so it stays.
25. **Gifts stay "another player"; loops count the team.** In a 1-player game,
    "another player" means you: a gift is yours, and a card that fires on another player's play fires on yours (a
    shared gain lands once). Trace every loop with the team as one pool: two
    cooperating players can pass a gift back and forth.

**Unit anchors** (Matt, candidates). Price every card against them; the anchor beats `values.json` for the printed
price ([card-pricing.md](card-pricing.md), *The method*).

| Unit | Credits | | Unit | Credits |
|---|---|---|---|---|
| Remove 1 agent | 5 | | 1 chip | 6 (r83: 3 before) |
| Remove 1 troop | 5 | | +1 credit income | 4 at mid-game |
| 1 energy | 4.5 | | +1 insight income | 12 at mid-game |
| 1 insight | 4 | | Remove 1 compute from a project | 2 (stopping projects is hard) |
| Draw 1 card | 5 | | 1 antimatter | 18 (r90: a big investment that pays off) |

Derived: the keep, 4 a tier-1 card and 8 a tier-2 one (1 and 2 insight); +1 energy income 14 (4 rounds × 4.5 × 0.8);
+1 chip income about 19 (4 rounds × 6 × 0.8). Antimatter's 18 is three research cubes (the desperate standard action,
at par, never a gain), so every engine that makes it (a plant's Action, Pair Forge, Particle Run) shows a clear gain
and every spend is a big one. Units with no anchor take `values.json` (an ambassador gained 4, a sway 4, a node 8)
or [card-pricing.md](card-pricing.md) (a city liberated 45). From past checks: a research cube 6
(9 when the play completes the megaproject), a botnet removed 16.

**The anchor check**, one line per card in its item's write-up: pay = credits + 4 × (keep + play insight) + other
costs at the anchors; get = the effects at the anchors (an engine at a committed seat's rate: about 1 card of its tag
and 3 tags a round, for 3-4 rounds); the ratio get ÷ pay stays under ruling 22's ceiling;
under 1.0 the card is too dear.

**Nothing to act on never blocks a card** (Matt; [rules/player-cards.md](rules/player-cards.md)): a removal rider
is a bonus, never a veto.

**Tracked amounts matter** (Matt, 2026-11-05): anything a player tracks or pays moves the game. No threat counter,
card cost or rider prints 1 credit: fold it into the price or raise it. A counter is 5+ credits or energy (1 or 2);
a per-tag rate of 1 is a rate, not an amount.

**Board state to size removals against:** agents median 6 a country (p90 10) and 6 in a seat's
reach (p90 12); troops 0-1 a country (p90 3); a project holds under its cost (5-10 compute a player).

## House wording

Use these, and let the rulebook carry the rest. The house terms are defined once, in `HOUSE_TERMS` in
`crates/aic-sim/src/docs/blocks.rs`, which prints them in [rules/README.md](rules/README.md)'s *Terms* and in
[player-aid.md](player-aid.md) (*Facility*: "building" is always a card, Harden goes on a facility).

**American English** everywhere (Mobilization, Armored, Modeling): `aic-sim docs --check` refuses a British spelling
(`spelling.rs`).

## Legend

The sheet is `icons.png` from `node tools/card-art/render.mjs`.

| Mark | Means |
|---|---|
| Square tile | a resource (cards included) |
| Disc | a token on a card: alone, on this card ("here") |
| Bare silhouette | a piece on the board, or moving one. Each piece has one symbol, the map's where the map draws one: ambassador, agent (a figure in a brimmed hat), node, botnet, troop, drone (chevrons) |
| Place after a piece (dark frame) | where: a city, station or factory glyph, only that kind (a cube) or on or next to it (a botnet, agents, ambassadors); a country, in one country |
| Ringed circle | a tag; an empty ring, a tag of any kind |
| Number before an icon | how many; no number means 1 |
| Minus before an icon | pay or lose it, 1 included, and nothing else: every paid cost, an Action's included |
| Less (number over a double chevron down) | that much less: a discount, a jam |
| `≤`N | up to N, your choice: a give, paid and received at the number you pick |
| No box | once, now, you or this card |
| Brown box | income: each round above the art. With a track arrow (up: raise, down: lower), a one-time track move: below the art, or after an Effect's `:` |
| Dark frame | acts on the adversary's pieces, projects or threats; its numbers are Remove N (the jam sits outside it) |
| Dashed outline | another player |
| Thin solid outline | your cards: a disc in it goes to (or comes from) one of your cards |
| `→` | an Action: pay the left, get the right. A bare `→` is free |
| OR at a line's start | the other half of an Action: once a round, one half or the other |
| `:` | an Effect: when the left happens, the right. The less mark after it (`building :`, a tag, `play`): those cards cost that much less |
| Play (a card with an arrow onto it) | a card you play, with its tag if any; in the dashed outline, another player's |
| `≥`N | at least N: a rebate's printed credit cost |
| Ringed number, discs, `:` (At N) | when the tokens here reach N, they come off and the right happens |
| Move, then a pin | move onto one piece (a bare move: onto pieces) |
| `/` | per |
| OR, ALL, ACT | written out; ACT is an Action granted |
| Shield | one stop on one piece: alone, a hardening token on a facility; with a node, a shield |
| Roof | cover over many pieces: with a country, Shelter (this round); with a node, all your pieces (Aegis: any one, once a round; Sanctuary: nodes, from corruption) |
| Node | place it, paying its track step |
| Threat (`!`) | a threat. Counter is the threat struck through |
| Jam (threat with a down arrow) | after the less mark: one threat needs that much less of each resource |
| Clock | delay only: a threat waits a round |
| Wall (bricks) | no agent may arrive: with an agent and a hex, a firewall; with a country, a cordon |
| Requirement plates | yellow: at least (no card requires at most); stacked cards: built over |

## Event lines

An event card prints only the line kinds in `content::EVENT_LINE_KINDS` (r80, ruling #31; `Content::check` holds
to it), each with its icon there: Rally, Muster, Income, New factory, Discord, Unity back, Blackout, Node loss, Forced
March, Botnet spread (r89, Matt's amendment 9026), Corrupt. Event faces have no icon row yet. A new kind is Matt's call.

A price rise on paper needs a screen: small raises add up. Reason every price in
[card-pricing.md](card-pricing.md), and record the change in [rules-history.md](rules-history.md). Center a rate sheet
on the printed prices before reading outliers: an off-center rate makes most of the set read wrong.

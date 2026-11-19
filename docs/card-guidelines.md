# Card guidelines

The checklist for every card, read as an experienced hobby player opening the box would: Matt's principles
(2026-10-31) and card rulings 1-12 (2026-11-02). A card that breaks one is a bug, or a design call for Matt, never a
quiet exception.

1. **No exceptions: no `*`.** Every symbol means exactly what it always means. A card needing a footnote is
   redesigned, repriced or simplified; if the exception is the fun, find another way to that play and take it to
   Matt.
2. **Plain words, one reading only.** A player reads a card once and knows what to do. Name the pieces and the
   number: "remove 2 agents or 1 troop", never "2 worth". A "per" or an OR binds to exactly one thing; "you have" is
   spelled out ("on your cards"); every icon keeps its locked meaning (ruling 5). Fix a phrase in `face.rs` and every
   card using it improves.
3. **No arithmetic.** At most a simple "per X". No sums, series, formulas or running totals. **No "per hex of a
   country"** (ruling 6): every country has 16 hexes, so "1 per hex" is a hidden fixed number; print the number
   ("remove 6 troops in its country"). **Big numbers over "remove all X"** (*Martial Cordon*).
4. **No trivial cards; every rider earns its text.** Drafting costs a keep (1 insight at tier 1, 3 at tier 2, 2 with the keep
   upgrade; never credits) before the play cost, so a card must be worth that (ruling 1: *Surplus Auction*). A rider or
   conditional extra adding symbols and parsing but no decision ("gain 1 credit") is noise: fold its value into the
   base number, or cut it. **Fold a near-constant rider into the number** (ruling 3: *Blackout Script*: a botnet
   makes 1 compute, so "no compute next build" is one more compute removed).
5. **No circular payments** (ruling 2). Paying credits to gain credits back is netted out, never printed both ways
   (*Peace Dividend*: pay 3, gain 2 credits).
6. **Clear timing.** Each effect reads as once now (below the art), each round, or as an Action. "Pay N, gain M" is a
   one-off; don't print a rate where a fixed trade will do (24).
7. **No caps by default.** Price the uncapped effect. A card that breaks uncapped gets a lower rate or a new effect,
   not a cap. Nor does the game cap it ([rules/economy.md](rules/economy.md), *No limits*): a runaway pile is fixed
   at the cards feeding it. A card that draws prints at least 1 insight to play, so a round's chain of plays stops
   when its insight does (Science discounts can take a play to 0).
8. **One clear idea per card.** Two effects at most, unless a third is the extra cost. A building holds income or
   tokens, and one Action or Effect; an Action may be "A, OR B" when it is a token card's choice (15).
9. **Fits at the default size.** Rules text prints at 7.5 pt. A building's box above the art holds about 80
   characters on two lines with one icon row, the box below about the same, a program or event about 110 with one
   icon row and about 78 with two (264; guides: the fit decides). Cut words, split ideas or simplify the op rather
   than shrink the type; a card that still can't fit is a play change for Matt, and `cards:fit` fails until it is
   settled (264: no recorded exceptions). Measure with `npm run cards:fit` ([web/README.md](../web/README.md),
   *Text sizes*). A kind with no frame (threats, Alignment, adversary and project cards) has no budget until it gets
   one.
10. **The name says what it does,** and the art matches the name.
11. **No near-duplicates without a reason.** Same effect at another price or kind only when the kind matters (an
    event leaves no tag; a program stays). A card repeating an AI board's power needs a real price, or goes
    (*Shelter Order*).
12. **Tier is power; costs and effects are big** (ruling 4). Some complexity is fine but not required. Tier-2 income
    is far stronger, few cards cost 0 credits, and tier is not a proxy for cost (23). The curve in use:
    [card-pricing.md](card-pricing.md), *The method*.
13. **A card beats the plain route it replaces** (direct payment, standard action, board power), counting its keep
    and insight (ruling 7: *Threat Hunt* lost to paying 5 credits).
14. **Outright threat counters stay rare** (ruling 8): a special case, not the common fix. A typed counter is a big
    jam of its type plus a rider of its tag, never a full counter (*Rapid Response*).
15. **No recounting across cards** (ruling 12). A token effect counts only the tokens **on that card**, never "you
    have" across cards: recounting every round is tracking load. Token cards are progressive with a choice
    ("Action: add 1 Dataset here, OR gain 1 insight per Dataset here": *Synthetic Data Forge*), and other cards add
    tokens to one card (*Recruit Operatives*). Nothing gains tokens by itself each round. A program cashing tokens in
    takes them from one card.
16. **Tag chains** (ruling 10). Science, and other tags where it fits, has requirement chains: cards needing 3-4 of a
    tag, and a few needing 6, with a powerful effect once met. The requirement is the price; the credit cost stays
    modest. A chain card's `needs` names one tag (the cost rule: N of one tag or N of any, never a mix).
17. **Per-tag payoffs** (ruling 11): cards paying "1 income per X tag" each round, and one-off cards producing "1
    energy (or another resource) per X tag on your cards".
18. **Critical analysis of every card** (ruling 9). Think each card through as a player would: what it does, step by
    step; its full cost against the plain route (13); when in an 8-10 round game it matters, and for which board;
    its riders (4); whether the text has exactly one reading (2); what decision it creates.

## Matt's rulings of 2026-11-04

A card that prices out on the value table but reads absurd to a Terraforming Mars or Spirit Island player is a bug
(*Eviction Worm*: "remove 8 agents in one country" for 10 credits).

19. **Useful alone.** A card is never useless alone, even if better with others; tag payoffs ("1 income per X tag")
    are the exception.
20. **Feeders carry a main effect.** A token feeder ("add N X to one of your X buildings") has a standalone main
    effect (credit or insight income, say) priced a little below the rate; its tokens are a bonus beating the rate
    with a target, as Terraforming Mars' Large Convoy does (*Dataset Scraper*, *Recruit Operatives*).
21. **Removal is at the target.** Removing troops or agents acts on the card's one target, on or next to it, or in
    its country where printed (*Target*, card-grammar.md); no card picks a country freely.
22. **Numbers make sense, not only price out.** A card beats its anchors (below) by about 1.5× at tier 1 and about
    2.5× at tier 2, never more: beyond that, report it rather than ship it. A ceiling, not a target: a card at
    1.0-1.5 is fair, its price never cut toward the ceiling. A dearer card has a stronger or different effect, not
    the same one with a multiplied number. A printed number above what is usually within reach is a disguised
    "remove all", or the piece is too common: size every removal against the board state below.
23. **Tier is efficiency, not magnitude.** Tier 1 is not "cheap": a tier-1 card at 27 credits for more than +3
    income is fine (r127's 40 and +4). Tier 2 is about efficiency and relative potency. Don't cap numbers or income by tier.
24. **Exchanges are one use.** A major exchange on a card is a fixed Action, "Action: spend X, gain Y" (once a
    round), never "convert X to Y freely at a ratio". The megacorp converters are the exception (a board rule, not a
    card); a one-shot program emptying one card's tokens (*Collateralized Grid*) is one use, so it stays.
25. **Gifts stay "another player"; loops count the team.** In a 1-player game "another player" means you: a gift is
    yours, and a card firing on another player's play fires on yours (a shared gain lands once). Trace every loop
    with the team as one pool: two cooperating players can pass a gift back and forth.

**Unit anchors** (Matt, candidates). Price every card against them; the anchor beats `values.json` for the printed
price ([card-pricing.md](card-pricing.md), *The method*). Since r128 (273, Matt: smaller numbers) every credit
number is r127's ÷ 1.5, so a third reads .33 or .67; the ratios are r127's.

| Unit | Credits | | Unit | Credits |
|---|---|---|---|---|
| Remove 1 agent | 3.33 | | 1 chip | 4 |
| Remove 1 troop | 3.33 | | +1 credit income | 4 at mid-game |
| 1 energy | 3 | | +1 insight income | 8 at mid-game |
| 1 insight | 2.67 | | Remove 1 compute from a project | 1.33 (stopping projects is hard) |
| Draw 1 card | 3.33 | | 1 antimatter | 18 (epic: Matt, 283; was 12) |
| 1 ambassador | 5.33 | | | |

Derived: the keep, 2.67 a tier-1 card and 8 a tier-2 one (1 and 3 insight); +1 energy income 9.6 (4 rounds × 3 ×
0.8); +1 chip income about 13 (4 rounds × 4 × 0.8). A research cube is 4 (6 when the play completes the
megaproject). Make antimatter (5 cubes, 20) reads 24 at antimatter 18 (1 antimatter and 2 energy), yet stays an unused
hedge (Matt, 283); every engine making it (a plant's Action: Positron Storage Ring, Cyclotron Lease and four more)
shows a clear gain, and every spend is a big one. Other units take `values.json` (a sway 2.67, a
node 5.33) or [card-pricing.md](card-pricing.md) (a city liberated 30); a botnet removed is 10 (252: the median card for the job). A
card whose unit has no anchor sits inside 1.0-2.5× under both readings: a convert at its output's anchor and its
input's, an engine's uses at the measured count and the conservative one.

**The anchor check**, one line per card in its item's write-up: pay = credits + 2.67 × (keep + play insight) + other
costs at the anchors; get = the effects at the anchors (an engine at a committed seat's rate: about 1 card of its
tag and 3 tags a round, for 3-4 rounds); get ÷ pay stays under ruling 22's ceiling; under 1.0 the card is too dear.

**Nothing to act on never blocks a card** (Matt; [rules/player-cards.md](rules/player-cards.md)): a removal rider is
a bonus, never a veto.

**Tracked amounts matter** (Matt, 2026-11-05): anything a player tracks or pays moves the game. No threat counter,
card cost or rider prints 1 credit: fold it into the price or raise it. A counter is 3+ credits or energy (1 or 2);
a per-tag rate of 1 is a rate, not an amount.

**Board state to size removals against:** agents median 6 a country (p90 10) and 6 in a seat's reach (p90 12: a
power's or standard action's area; a card's is its target); troops 0-1 a country (p90 3); a project holds under its
cost (7-32 compute a board).

## House wording

Use the house terms (`HOUSE_TERMS` in `crates/aic-sim/src/docs/blocks.rs`, printed in
[player-aid.md](player-aid.md); *Facility*: "building" is always a card, Harden goes on a facility) and let the
rulebook carry the rest. **American English** everywhere (Mobilization, Armored, Modeling): `docs --check` refuses a
British spelling.

## Legend

The sheet is `icons.png` from `node tools/card-art/render.mjs`. Boxes, outlines, `→`, OR at a line's start, `/` and
requirement plates are in [rules/card-grammar.md](rules/card-grammar.md) (*Where it sits*, *Boxes*, *Connectors*).

| Mark | Means |
|---|---|
| Square tile | a resource (cards included) |
| Disc | a token on a card: alone, on this card ("here") |
| Bare silhouette | a piece on the board, or moving one. Each piece has one symbol, the map's where the map draws one: ambassador, agent (a figure in a brimmed hat), node, botnet, troop, drone (chevrons) |
| Place after a piece (dark frame) | where: a city, station or factory glyph, only that kind (a cube) or on or next to it (a botnet, agents, ambassadors); a country, in the target's country |
| Ringed circle | a tag; an empty ring, a tag of any kind |
| Number before an icon | how many; no number means 1 |
| Minus before an icon | pay or lose it, 1 included, and nothing else: every paid cost, an Action's included |
| Less (number over a double chevron down) | that much less: a discount, a jam |
| `≤`N | up to N, your choice: a give, paid and received at the number you pick |
| Dark frame | acts on the adversary's pieces, projects or threats; its numbers are Remove N (the jam sits outside it) |
| `:` | an Effect: when the left happens, the right. The less mark after it (`building :`, a tag, `play`): those cards cost that much less |
| Play (a card with an arrow onto it) | a card you play, with its tag if any; in the dashed outline, another player's |
| `≥`N | at least N: a rebate's printed credit cost |
| Ringed number, discs, `:` (At N) | when the tokens here reach N, they come off and the right happens |
| Move, then a pin | move onto one piece (a bare move: onto pieces) |
| OR, ALL, ACT | written out; ACT is an Action granted |
| Shield | one stop on one piece: alone, a hardening token on a facility; with a node, a shield |
| Roof | cover over many pieces, with a node: Shelter (your nodes and facilities in reach, this round); Aegis (any one, once a round); Sanctuary (nodes, from corruption and agents) |
| Node | place it, paying its track step |
| Threat (`!`) | a threat. Counter is the threat struck through |
| Jam (threat with a down arrow) | after the less mark: one threat needs that much less of each resource |
| Clock | delay only: a threat waits a round |
| Wall (bricks) | no agent may arrive: with an agent and a hex, a firewall; with a country, a cordon |

## Event lines

An event card prints only the line kinds in `content::EVENT_LINE_KINDS`, each with its icon there (`Content::check`
holds to it): Infiltrate, Muster, Income, New factory, Discord, Unity back, Blackout, Node loss, Forced March,
Botnet spread, Fallout, Spill and Grow. Lines print in a fixed order: the threat, Infiltrate (two at stage III),
Muster, at most two other lines, Spill, Grow. Event faces have no icon row yet. A new kind is Matt's call.

A price rise on paper needs a screen: small raises add up. Reason every price in [card-pricing.md](card-pricing.md),
record the change in [rules-history.md](rules-history.md). Center a rate sheet on the printed prices before reading
outliers: off center, most of the set reads wrong.

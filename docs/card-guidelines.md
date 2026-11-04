# Card guidelines

The checklist for every card, read as an experienced hobby player opening the box would.
Matt's principles (2026-10-31) and Matt's card rulings (2026-11-02, rulings 1-12, merged in
below where they overlap; each names its example card). A card that breaks one is a bug, or a design call for Matt,
never a quiet exception.

1. **No exceptions: no `*`.** Every symbol means exactly what it always means. A card that
   needs a footnote is redesigned, repriced or simplified. If the exception is the fun, find
   another way to get that play and take it to Matt.
2. **Plain words, one reading only.** A player reads a card once and knows what to do. Name the pieces
   and the number: "remove 2 adversary ambassadors or 1 troop", never "2 worth". A "per" or an OR binds to exactly
   one thing; "you have" is spelled out on the face ("on your cards"); every icon keeps its locked meaning (ruling 5: *Kinetic Interdiction* read as 3 ambassadors or 1 troop in every hex, or in one; *Agency Field Station*
   counted Agents on every card). Fix a phrase in `face.rs`, and every card using it improves.
3. **No arithmetic.** At most a simple "per X". No sums, series, formulas or running totals. **No "per hex of a
   country"** (ruling 6): every country has 16 hexes, so "1 per hex" is a hidden fixed number; print the number
   ("remove 6 troops in one country"). **Big numbers over "remove all X"** (*Kinetic Interdiction*, *Agency
   Directorate*, *Martial Cordon*).
4. **No trivial cards; every rider earns its text.** Drafting costs a keep (1 insight or 5 credits) before the play
   cost, so a card must be worth that (ruling 1: *Surplus Auction*, *Peace Dividend*, *Agency Field Station*). Cut a
   small rider that adds symbols and parsing but no decision ("gain 1 credit"). A conditional extra that creates no
   decision is noise: fold its value into the base number, or cut it. **Fold a near-constant rider into the number**
   (ruling 3: *Blackout Script*: a botnet makes 1 compute, so "no compute next build" is one more compute removed).
5. **No circular payments** (ruling 2). Paying credits to gain credits back is netted out, never printed both ways
   (*Peace Dividend*: pay 3, gain 2 credits).
6. **Clear timing.** Each effect reads as once now (below the art), each round, or as an Action.
   "Pay N, gain M" is a one-off. Don't print a rate where a fixed trade will do.
7. **No caps by default.** Price the uncapped effect. A card that breaks without a cap gets a
   lower rate or a new effect, not a cap.
8. **One clear idea per card.** Two effects at most, unless a third is the extra cost. A building
   holds income or tokens, and one Action or Effect; an Action may be "A, OR B" when it is a token card's choice
   (rule 15).
9. **Fits at the default size.** Rules text prints at 7.5 pt. A building's box above the art holds
   about 80 characters on two lines; the box below the art about the same; a program or event about
   110. Cut words, split ideas or simplify the op rather than shrink the type. A card that still
   can't fit is a recorded exception: its reason goes in the item's write-up. Measure with `npm run cards:fit` in `web/`. A box's
   capacity derives from its height in `art/frames/r3/fit.json` (which overrides `layout.json`),
   less `CLEARANCE_MM` (`web/card/card.js`) and its icon rows, at about 3.2 mm a line: size and line
   height are `.rules`, side padding `.effect`, in `web/card/card.css`. The character counts above
   are guides nothing checks. A kind with no frame (threats, Alignment, adversary and project
   cards) has no budget yet: it comes with the frame.
10. **The name says what it does,** and the art matches the name.
11. **No near-duplicates without a reason.** Same effect at another price or kind is fine only
    when the kind matters (an event leaves no tag; a program stays). A card that repeats an AI board's
    power needs a real price, or goes (*Shelter Order* duplicated the Protector's Shelter at 0 credits).
12. **Tier is power; costs and effects are big** (ruling 4). Tier 2 is more powerful; some complexity is fine but
    not required. Many tier-1 cards carry strong effects at high prices, tier-2 income is far stronger still, and
    few cards cost 0 credits. **Tier is not a proxy for cost:** a dear tier-1 card is fine. The curve in use is in
    [card-pricing.md](card-pricing.md), *The method*. (Terraforming Mars sits nearer 10, with cards like "cost 40,
    gain 10 income".)
13. **A card beats the plain route it replaces,** counting its keep and insight (ruling 7). *Threat Hunt*: 3 credits
    and about 2 insight for "counts as 5 credits toward a cyber threat" lost to paying 5 credits. Name the plain
    route (direct payment, a standard action, a board power) and show the card wins.
14. **Outright threat counters stay rare** (ruling 8): a special case, not the common fix. A typed counter is a big
    jam of its type plus a rider of its tag, never a full counter (*Threat Hunt*, *Rapid Response*). Counter design
    waits for the adversary-card pass.
15. **No recounting across cards** (ruling 12). A token effect counts only the tokens **on that card**, never "you
    have" across cards: recounting every round is tracking load. Token cards are progressive with a choice:
    "Action: add 1 Agent here, OR get 1 credit per Agent here" (*Agency Field Station*), with other cards that add
    tokens to one card (*Recruit Operatives*). Nothing gains tokens by itself each round. A program that cashes
    tokens in takes them from one card.
16. **Tag chains** (ruling 10). Science, and other tags where it fits, has requirement chains: cards that need 3-4
    of a tag, and a few that need 6, giving a powerful effect at a reasonably cheap price once the requirement is
    met. The requirement is the price; the credit cost stays modest. A chain card's `needs` names one tag (the cost
    rule: N of one tag or N of any, never a mix).
17. **Per-tag payoffs** (ruling 11). Cards that pay "1 income per X tag" each round, and one-off cards that produce
    "1 energy (or another resource) per X tag on your cards".
18. **Critical analysis of every card** (ruling 9). Think each card through as a player would: what it does, step by
    step; its full cost (credits, insight to play, the keep); what the plain route gets for the same (direct
    payment, a standard action, a board power); when in an 8-10 round game it matters, and for which board; any
    near-constant or negligible rider; whether the text has exactly one reading; what decision it creates. The last full
    verdict on every card is the table in
    [experiments/results/screen-076-cards/cards.md](../experiments/results/screen-076-cards/cards.md).

## House wording

Use these, and let the rulebook carry the rest. The terms (facility, shield, ambassador, troop,
in reach, a country you're in, here, Remove N, Draw, on your cards, Action) are defined once, in
`HOUSE_TERMS` in `crates/aic-sim/src/docs/blocks.rs`, which prints them in [rules/README.md](rules/README.md)'s
*Terms* and in [player-aid.md](player-aid.md). `face.rs` prints the phrases as plain strings.

**"Building" is always a card, "facility" a station or factory.** "Harden" goes on a facility;
the same token on nodes is a "shield". "You gain the Harden action" is a standard action a card grants, paid in
chips (the card prints the price); the Protector's Bulwark is a power, paid in energy. It is not a keyword.

**American English.** Content, faces, docs, UI and comments: Mobilization, Armored, Modeling.
`aic-sim docs --check`, so the fast lane, refuses a British spelling (`crates/aic-sim/src/spelling.rs`).

## Legend

What every mark on a card means. Each means one thing; the sheet is `icons.png` from
`node tools/card-art/render.mjs`.

| Mark | Means |
|---|---|
| Square tile | a resource (credits, insight, chips, energy, antimatter, compute, cards) |
| Disc | a token on a card: alone, on this card ("here") |
| Bare silhouette | a piece on the board, or moving one |
| Place after a piece (dark frame) | where: a city, station or factory glyph, only that kind (a cube) or on or next to it (a botnet, ambassadors); a country, in one country |
| Ringed circle | a tag |
| Number before an icon | how many; no number means 1 |
| Minus before an icon | pay or lose it now, 1 included |
| `≤`N | up to N, your choice: a give, paid and received at the number you pick |
| No box | once, now, you or this card |
| Brown box | income: each round above the art, a track move below it |
| Dark frame | acts on the adversary's pieces, projects or threats; its numbers are Remove N |
| Dashed outline | another player |
| Thin solid outline | your cards: a disc in it goes to (or comes from) one of your cards |
| `→` | an Action: pay the left, get the right. A bare `→` is free |
| OR at a line's start | the other half of an Action: once a round, one half or the other |
| `:` | an Effect: when the left happens, the right |
| `/` | per |
| OR, ALL, ACT | written out; ACT is an Action granted |
| Shield | stops one destruction: alone, a hardening token on a facility; with a node, a shield; with a country, Aegis Dome Array's |
| Shelter (roof) | a whole country, this round |
| Node | place it, paying its track step |
| Node with a struck-through coin | place it free |
| Threat (`!`) | a threat. Counter is the threat struck through |
| Jam (threat with a down arrow) | one threat needs that much less of each resource |
| Clock | delay: a threat waits a round |
| Requirement plates | yellow: at least; red: at most; stacked cards: built over |

A price rise on paper needs a screen: nine small raises once cost c3 about 30 points. Every price is
a candidate: reason it in [card-pricing.md](card-pricing.md), and record the change
in [rules-history.md](rules-history.md). Center a rate sheet on the printed prices before reading
outliers: an off-center rate makes most of the set read wrong (insight at 4 read 85 of 150 too dear).

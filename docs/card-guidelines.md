# Card guidelines

The checklist for every card, read as an experienced hobby player opening the box would.
Matt's principles (2026-10-31), plus what 059 found. A card that breaks one is a bug, or a
design call for Matt, never a quiet exception.

1. **No exceptions: no `*`.** Every symbol means exactly what it always means. A card that
   needs a footnote is redesigned, repriced or simplified. If the exception is the fun, find
   another way to get that play and take it to Matt.
2. **Plain words, no jargon.** A player reads a card once and knows what to do. Name the pieces
   and the number: "remove 2 adversary ambassadors or 1 troop", never "2 worth". Fix a phrase
   in `face.rs`, and every card using it improves.
3. **No arithmetic.** At most a simple "per X". No sums, series, formulas or running totals.
4. **Every rider earns its text.** A conditional extra that creates no decision is noise: fold
   its value into the base number, or cut it.
5. **Clear timing.** Each effect reads as once now (below the art), each round, or as an Action.
   "Pay N, gain M" is a one-off. Don't print a rate where a fixed trade will do.
6. **No caps by default.** Price the uncapped effect. A card that breaks without a cap gets a
   lower rate or a new effect, not a cap.
7. **One clear idea per card.** Two effects at most, unless a third is the extra cost. A building
   holds income or tokens, and one Action or Effect.
8. **Fits at the default size.** Rules text prints at 7.5 pt: a smaller card is the exception and
   carries a reason. A building's box above the art holds about 80 characters on two lines; the
   box below the art about the same; a program or event about 110. Cut words, split ideas or
   simplify the op. Never shrink the type. Measure with `npm run cards:fit` in `web/`.
9. **The name says what it does,** and the art matches the name.
10. **No near-duplicates without a reason.** Same effect at another price or kind is fine only
    when the kind matters (an event leaves no tag; a program stays).

## House wording

The generator keeps these in one place: use them, and let the rulebook carry the rest.

| Say | Because |
|---|---|
| "ambassador" alone | yours; the other side's is always "adversary ambassador" |
| "troop" | troops are always the adversary's |
| "in reach" | your reach, within 1 of your presence |
| "a country you're in" | a country where you have presence |
| "here" on a building | on this card, where its tokens sit |
| "Draw 1 card" | every draw is from the tier-1 deck |
| "Remove N" | as many as there are, up to N |
| "you have", on tags and tokens | on your cards in play; covered buildings hold tokens that don't count |
| "Action:" | once a round, always |

A price rise on paper needs a screen: 059's nine small raises cost c3 about 30 points. Every price is
a candidate: reason it in [card-pricing.md](card-pricing.md), and record the change
in [rules-history.md](rules-history.md).

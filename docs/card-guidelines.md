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
   simplify the op. Never shrink the type. Measure with `npm run cards:fit` in `web/`. A box's
   capacity derives from its height in `art/frames/r3/fit.json` (which overrides `layout.json`),
   less `CLEARANCE_MM` (`web/card/card.js`) and its icon rows, at about 3.2 mm a line: size and line
   height are `.rules`, side padding `.effect`, in `web/card/card.css`. The character counts above
   are guides nothing checks. A kind with no frame (threats, Alignment, adversary and project
   cards) has no budget yet: it comes with the frame.
9. **The name says what it does,** and the art matches the name.
10. **No near-duplicates without a reason.** Same effect at another price or kind is fine only
    when the kind matters (an event leaves no tag; a program stays).

## House wording

Use these, and let the rulebook carry the rest. The terms (facility, shield, ambassador, troop,
in reach, a country you're in, here, Remove N, Draw, you have, Action) are defined once, in
`HOUSE_TERMS` in `crates/aic-sim/src/docs/blocks.rs`, which prints them in [rules.md](rules.md)'s
*Terms* and in [player-aid.md](player-aid.md). `face.rs` prints the phrases as plain strings.

**"Building" is always a card, "facility" a station or factory.** "Harden" goes on a facility;
the same token on presence is a "shield". "You gain the Harden action" is the Protector's action,
not a keyword.

**American English.** Content, faces, docs, UI and comments: Mobilization, Armored, Modeling.
`aic-sim docs --check`, so the fast lane, refuses a British spelling (`crates/aic-sim/src/spelling.rs`).

## Legend

What every mark on a card means. Each means one thing; the sheet is `icons.png` from
`node tools/card-art/render.mjs`.

| Mark | Means |
|---|---|
| Square tile | a resource (credits, insight, chips, energy, antimatter, compute, cards) |
| Disc | a token on a card: alone, on this card |
| Bare silhouette | a piece on the board, or moving one |
| Ringed circle | a tag |
| Number before an icon | how many; no number means 1 |
| Minus before an icon | pay or lose it now, 1 included |
| `≤`N | up to N, your choice: a give, paid and received at the number you pick |
| No box | once, now, you or this card |
| Brown box | income: each round above the art, a track move below it |
| Dark frame | acts on the adversary's pieces, projects or threats; its numbers are Remove N |
| Dashed outline | another player |
| Thin solid outline | counted across your cards, the "you have" box |
| `→` | an Action: pay the left, get the right. A bare `→` is free |
| `:` | an Effect: when the left happens, the right |
| `/` | per |
| OR, ALL, ACT | written out; ACT is an Action granted |
| Shield | stops one destruction: alone, a hardening token on a facility; with presence, a shield; with a country, Aegis Dome Array's |
| Shelter (roof) | a whole country, this round |
| Presence | place it, paying its track step |
| Presence with a struck-through coin | place it free |
| Threat (`!`) | a threat. Counter is the threat struck through |
| Jam (threat with a down arrow) | one threat needs that much less of each resource |
| Clock | delay: a threat waits a round |
| Requirement plates | yellow: at least; red: at most; stacked cards: built over |

A price rise on paper needs a screen: 059's nine small raises cost c3 about 30 points. Every price is
a candidate: reason it in [card-pricing.md](card-pricing.md), and record the change
in [rules-history.md](rules-history.md). Center a rate sheet on the printed prices before reading
outliers: an off-center rate makes most of the set read wrong (055: insight at 4, 85 of 150 too dear).

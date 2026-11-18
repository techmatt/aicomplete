# 265: viewer fixes from 261's hunt (r124, no rules change)

The five viewer and player-view items 261 deferred: D108, D109, D112, D115, D116
([261's deferred.md](../261_bug_hunt/deferred.md)). No play changes, so no `RULES_REVISION` bump. Two changes reach
engine text: the node prompt's words (`ai::uncover_words`; the option record keys from `uncover_label` are unchanged)
and the card faces (`face.rs`, `op.rs` `TargetSpec::glyph`).

## What changed

- **D116** (`session.ts`): a record that stops replaying keeps its unreplayed actions (`Timeline.unreplayed`) and
  saves whole, so a reload replays to the same stop and the error bar names it again. Before, it wasn't saved at all
  and a reload brought back the game before it without saying so. An action or undo drops the tail.
  `npm run odd -- --only stopped-record-reload`.
- **D109** (`format.ts`): a standard action is labeled with its printed name (`STANDARD_NAMES`) and cost, for example
  "Gain a player ambassador (20 credits)". Research cubes are part of the cost, read from the doc's "; costs ..."
  because the view's price fields drop them. Its rule text is the button's hover, in both the player bar and the debug
  seat panel (`actionRule`). Before, the label was the rule's first clause, and "Buy a city's energy" printed its
  cost twice.
- **D112:**
  - The legend has the dotted red outline, worded "a face-up threat's source".
  - The player seat panel lets columns shrink and wraps the programs row (no sideways scroll). The powers have their
    own grid area: under the values below 2200 px wide, and above it a column of their own, split in two, beside the
    values. In player-play shots at 2560×1440, every power of the Builder (11) and the Researcher shows without the
    panel scrolling.
  - An unusable converter shows what it trades, after why it can't be used.
  - The debug step bar shows a power by its name and cost (`LabelContext.brief`), with the full label in its hover.
  - A megaproject's name and status wrap in the player view's country column. In the debug strip they still cut to
    an ellipsis, as 184 chose, but the hover now holds the name and status (it said only "megaproject").
- **D115** (`player/view.ts`):
  - The turn panel is always headed "Since your last move" and lists only the rules' notes. Other events are counted,
    with the debug log named as the place to see them. The step lines no longer fall back to "seat 0 +1 energy" or
    "cards moved".
  - Track gains say "a round" wherever a person reads them: in the hex prompt (`uncover_words`) and in the track
    option's button (format.ts fixes up the record-key words). One-off gains keep "once". This follows staging
    default 2, as Matt confirmed.
- **D108** (faces):
  - `TargetSpec::glyph` names each kind a target takes, and `targetIcon` draws them with "/" between. Any corrupted
    site (Hostile Buyout, Great Liberation) draws city / station / factory. A neutral site (Power Purchase
    Agreement, Microgrid Lab Compact) draws station / factory.
  - A liberation or city-form Cleanse draws the card's target kinds in its chip: Great Liberation's chip was
    "−3 cube city" for any site.
  - Hostile Buyout's "a station or factory becomes yours" now prints the claim icons. Its chip drops the site glyph,
    because three glyphs there and two more after took the box to 5.4 pt; the target box already names the kinds.
    The diff review's point stands for Matt: its line (−1 cube, then station / factory) is one glyph short of a
    Rescue's (−1 cube city, then station / factory), and a bare cube elsewhere means a station's or factory's.
  - A card that claims the one site it opens prints that kind: Open Research Campus's station, Industrial
    Commons's factory.
  - `cards:fit` lists the same four faces below 7.5 pt as 264 left: Retained Earnings, Positron Storage Ring, Popular
    Uprising and Particle Accelerator. Nothing new.
  - D108's option (b), a generic "site" glyph, needs new art and is left for Matt.

## Checks

The new `npm run odd` cases are `stopped-record-reload` (D116) and `player-labels-legend-turn` (D109's bar and
hover, the legend, D115's heading and raw events, the step bar's power, an unusable converter's trade, a cube-priced
standard button's price). Each check runs only if a random game reaches its case, and the scenario logs any that
didn't come up. In the final run no cube-priced action came up; the label's cube parse was checked against the doc
format by hand.

Results on the final build:
- `npm run odd`: 15 of 15 pass.
- `e2e`, `e2e:researcher` and `e2e:sources` pass.
- `player-play.mjs` passes at 1366×768 and at 2560×1440.
- `AIC_SWEEP_VIEW=player npm run sweep -- --case c4-d0` (1 seed): exit 0. Its reports are look-only `truncated`
  items: hand-tile names and the turn step lines, which are one line by design, with the full text in the hover.
- `e2e:corps` fails the same four checks D102 records as red on main (cube counts from before this week's rules).
  None of them touches a line this item changed.

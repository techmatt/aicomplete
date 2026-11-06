# 134: the adversary decks drafted for the new model

Content for review, not play: the engine and `content/` stay r53. The drafts are in
[`drafts/adversary/`](../../../drafts/adversary/), their format and value scale in [spec.md](spec.md), every card
with its value and check line in [listing.md](listing.md), and the picker is `picks/adversary_drafts.json`
(99 rows, nothing selected).

| Deck | Cards | Tags (a card may carry two) |
|---|---|---|
| Threats, tier 1 | 25 | Military 6, Influence 7, Cyber 6, Economic 6, Grid 5 (5 lead each tag) |
| Threats, tier 2 | 28 | Military 6, Influence 7, Cyber 6, Economic 7, Grid 5, Universal 3 |
| Threats, tier 3 | 11 | Universal 3, Military 4, Cyber 2, Economic 2, Influence 1, Grid 1 |
| Projects | 20 | Military 6, Influence 3, Cyber 3, Economic 3, Grid 2, Universal 3; cost 4-18 compute per player |
| Militarist | board + 14 events | 5 / 6 / 3 printed, 4 / 5 / 2 dealt; 3 project slots, one Military |

## The draw rule holds

`python drafts/adversary/check.py` deals 4,000 Militarist games (1-4 countries; the stage counts of difficulty 0-1,
2 and 3-4) through the draw rule: 133,330 pulls, none needing a second reshuffle, and at most one reshuffle in any
pull. Hall's condition holds for every event card at 1 and 4 countries with the whole deck in hand, counting that a
base-only threat can't land at 1 country. Fitting cards per pull (4 needed at 4 countries): the tightest is
Probing Strikes' lone Military 1 (6 cards), then Total Mobilization's Military 3 (7: 4 Military, 3 Universal),
Mobilization's and Border War's Military 2 (9); the rest have 9-28. A tier-1 deck with fewer than 5 Military cards
would break Probing Strikes at 4 countries after one unlucky reshuffle.

## How the drafts were made

Five drafters (one per deck), each followed by a reviewer that recomputed every value line and fixed the file; then
four cross-deck reviewers (rules and wording, value, coverage and duplicates, loops and counterability) found 50
problems, of which the real ones were fixed per file; a last audit (text against effects, harms, value lines, stale
flags) found 18 more, all fixed. The review's biggest finds, now in the spec as table rules:

- **Placement.** "The site with the most adversary pieces near it" sent tokens to the adversary's own sites, where
  half the cards did nothing. A city or station token now goes on an uncorrupted one, a factory token on one that
  isn't the adversary's (else a free zone); the most-threatened among those.
- **No bases at 1 country**: seven threats named only a base. The pull now skips a threat whose location types the
  country lacks.
- **Nobody near**: a threat whose harm is all "each player near it" was free to ignore. Today's 1 Discord sway per
  uncountered threat stays as the floor, unprinted.
- **Corp troops at the cap**: a base sits at its cap after almost every production step, so "extra corp troops"
  did nothing. They now go past the cap, and print when.
- **Theft** hits the richest player *near it*, so the victim can pay. Names that clashed (two Mobilizations, two
  Hostile Takeovers) and near-duplicates (Asset Freeze and Market Crash; Siege and the tier-3 troop card; Blackout and
  Load Shedding; Sabotage and Zero Day; War Footing and General Mobilization) were reworked.

## Readings (scope lines I read one way)

- **"Universal pulls"** are pulls any tag fits (`["any"]`), not pulls only a Universal card fits: with 3 Universal
  threats a tier, a Universal-only pull fails at 4 countries.
- **"Nodes next to the location"** (Military's counter) is spent: the nodes leave the game. Read as "hold N nodes
  there" it would be a presence test, which is also printable; the drafts would need only the counter label.
- **Near**: a node or a player ambassador within 1 hex of a location token. A threat's tags all sit at its
  deck's tier ("Military 2 + Economic 2"), so a pull draws from one deck.

## Open, for Matt

1. **The tag sacrifice is the cheapest counter almost everywhere** (5 a tag at the spec's scale). Against it many
   threats read above 2x (up to about 3.4x); against credits most read 1.5-2.3x. Either a tag is worth more than 5 mid-game, or tag
   counters should ask 2 at tier 1. Calibration was out of scope; the value lines give both ratios.
2. **Stage III has no spare above difficulty 1**: difficulty 2-4 deal 3 stage-III cards, all three printed. A fourth
   stage-III card, or difficulty that swaps differently, keeps N+1.
3. **"A war round brought forward" is blank from round 6**: the clock (1,2,1,2,1,3) makes every later round a war
   round. The drafts print the harm as **a second war step** on the fronts next to the threat's base instead
   (`extra_war`: Casus Belli, General Mobilization, Declaration of War), which stays live; the Militarist's events
   carry no war twist. If the model means the clock itself, it needs a clock with peace rounds late.
4. **Adversary factories have no removal** in today's rules, and three drafts build them (Mass Production, Shadow
   Factories, War Economy). A later rules item needs a route (a liberate twist, or a troop removal that can take one).
5. **Warmongering's loop**: a base with an adversary troop beside it makes an extra corp troop each production step,
   past the cap, until the troop is removed. Capped only by removal; read its stacks before keeping it.
6. **Regime Change** (tier 3) reads 3.6x at the 45-credit liberation anchor, 2.1x if a city corrupted in round 10
   is worth about 22. The anchor late in the game is Matt's call.
7. **Conscription** (tier 2: a base makes an extra corp troop every production step for the rest of the game) is
   priced at about one war's worth; its harm runs to the end, so it needs a price or a cap from play.
8. **Faces**: threats fit about 130 characters; Militarist events run 150-306 (pull, standing, twists, arrivals).
   Adversary cards have no frame yet; an icon for the standing lines would halve them.
9. **Ops**: 17 new ops and 9 existing ops with a new field or scope, listed in [listing.md](listing.md#ops-the-engine-lacks).
   Event cards' `pulls` and `standing` are new card sections; today a deal comes from `threats.deal`.

## For new players (the model)

Each round the adversary's event card comes first. Its top line deals threats: in each country, turn up cards from
the named tier's threat deck until one has a listed tag (a Universal card has every tag), discard the misses, and put
its location tokens on the board. Any player with a node or ambassador within 1 hex of a token is *near* it and may
help pay it off; the card lists a few ways to pay, and the table pays one in full. Next come the event's standing
lines (the adversary gains compute and lands ambassadors or troops), then its twists, arrivals, build, spread,
corruption and march. Compute fills the cheapest project on the adversary's board; a full project fires and its slot
waits until every slot has fired, then they refill. At the end of the round each threat nobody paid does what it
says, near its tokens, and also sways 1 toward Discord. Tier-1 threats cost you this round; tier 2 can take nodes,
cities and stations for good; tier 3 arrives in the last rounds, each beside a second, lighter threat. **What resisted a summary:** where a location
token goes (the most-threatened uncorrupted site of its type), and why a threat sometimes skips a country (it needs
a location the country has).

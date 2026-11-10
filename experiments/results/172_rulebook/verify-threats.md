# threats: engine-verified rules (172 verifier)

Rules revision 73. Default variant `countries`. `shared_row` plays the same (mod.rs:62-70). `off` deals nothing (mod.rs:71-91).

**Components**
- There are two threat decks, tier 1 (25 cards) and tier 2 (32 cards). There is no tier 3. Each deck is shuffled at setup (mod.rs:384-392; content/threats.json). Each deck has its own discard pile.
- Each threat card prints:
  - 1 or 2 tags: Military, Influence, Cyber, Economic, Grid or Universal. Universal counts as every tag.
  - 1 or 2 location types: city, coastal city, research station, factory, military base.
  - Its counter menu, which in the current content is always exactly two options: one scarce payment and one credits payment.
  - One or two consequences.

**When threats happen in the round** (configs/baseline.json:365-386)
1. The adversary flips its event card (`adversary.event`).
2. **Deal** (`threats.deal`).
3. The adversary builds (rally and deploy), corrupts, and does its other steps, then the march.
4. Players draft. Then comes one shared action phase: playing cards, **countering**, alignment actions, board actions and AI powers all happen in this same phase.
5. Bases, production and upkeep.
6. **Execute** (`threats.execute`).
7. War, then cleanup.

**Dealing** (mod.rs:357-381, 198-233)
1. Each pull the event card prints is done in turn. For each pull, go through the countries in order (0, 1, 2...), one threat into each country. There is one country per player.
2. **Draw rule:** turn up the top card of that tier's deck. It stays if both are true:
   - it carries one of the pull's tags (Universal fits any tag; an "any" pull takes every card);
   - the country has at least one available site of one of its location types (near.rs:73-75).
   Otherwise discard it to that tier's discard and turn up the next.
3. **Empty deck:** shuffle the discard to form a new deck. If a second reshuffle is ever needed, the next card is taken whatever it is. This is a content fault and never happens in practice. If both deck and discard are empty, no threat is dealt for that pull (mod.rs:201-219).
4. **Event card pulls by stage:** a stage-I card pulls one tier-1 threat per country; stage-II pulls one tier-2 (Supply Lines pulls one tier-1 and one tier-2); stage-III pulls two tier-2s (content/adversary.json).
5. **Difficulty 4, extra threats:** add `extra_threats` (1) more pulls of the card's first pull, same tier and tags (mod.rs:375-380). The country rotates: country 0 in round 1, country 1 in round 2, and so on.
6. **Difficulty 4, tier 2 sooner:** a stage-I card's tier-1 pulls become tier 2 when the next face-up adversary card is stage II or III (`tier2_early` 1). This also applies to the extra pull (mod.rs:366-367; adversary.rs:183-200).
7. **Threats dealt by other things:**
   - A threat whose consequence deals a threat (Provocation, Full Spectrum Assault, Last Ultimatum) deals one of its printed tier, any tag, into its own country (new_ops.rs:44-55).
   - Discord cards' extra threats use this round's card's first-pull tier, any tag, one per country in turn from country 0 (mod.rs:293-303).
   - Offensive deals one any-tag threat into a country and puts its first token on a chosen node hex. If its second token was on that hex, the second token comes off (mod.rs:247-253; near.rs:114-122).

**Location tokens** (near.rs:45-111)
- For each location type, in printed order, the threat puts one token in its country on an available site:
  - **City:** a city that is not corrupted.
  - **Coastal city:** a coastal city that is not corrupted.
  - **Station:** a research station that is not corrupted.
  - **Factory:** a factory that is not corrupted. If the country has none, use an industrial zone with no building on it.
  - **Base:** any military base.
- **Which site:** the one with the most adversary pieces (agents, troops, botnets) on it or next to it. On a tie, the lowest hex number.
- A threat's own two tokens never share a hex. Different threats' tokens can.
- If placing in printed order leaves a token with no site, place the type with fewer sites first instead, if that places more tokens.
- A type with no site gets no token.
- Tokens stay where they are until the threat leaves, even if the site changes.

**Near** (near.rs:138-197)
- You are near a threat if one of your nodes or one of your ambassadors on the board is on or next to any of its tokens.
- A threat with no token counts its whole country instead (only after a forced pull).

**Countering** (mod.rs:452-525; counter.rs)
- **Pooled payments.** Any player near a threat may put resources from the counter menu on it: credits, energy, insight, research cubes, chips, or ambassadors from your supply.
  - You may add any amount up to the largest amount still missing from any option that uses that resource.
  - Several players' payments add together.
  - You may take back everything you put on it in one resource at any time during the action phase, even if you are no longer near.
  - As soon as the payments cover every item of one resources-only option, the threat is countered and discarded with its tokens. If two options are covered, the first in menu order is used. Everything else is returned, the later seats getting theirs back first (counter.rs:220-280).
- **Tag sacrifice.** One near player alone gives up cards in play carrying that many tags of the named kind, as for an AI-power cost:
  - tier 1 asks 2 tags; tier 2 asks 3;
  - the card kinds are Security, Diplomacy, Cyber, Industry and Power;
  - any pooled payments on the threat are returned (counter.rs:283-361).
- **Card effects** (effects/mod.rs:137-161):
  - **Counter a threat:** reaches only threats you are near. Unity rewards reach any threat. Partial payments are returned (mod.rs:328-335).
  - **Jam N:** reaches threats you are near. A typed jam reaches only threats with its tag or Universal. It takes N off every resource item of every option, down to 0 at the lowest, and lasts while the threat is up. If that leaves it fully paid, it is countered at once (mod.rs:338-341). It does not reduce a tag option.
  - **Delay:** reaches threats you are near that are not already waiting. The threat waits: it skips this round's execution, stays up, and can be paid next round. It can be delayed again in a later round (science.rs:41-55; targets.rs:106-110).
- **Standing counters:** a card's automatic counter takes its amount off every resource item of every threat of its type, wherever it is and whenever it was dealt. Every player's standing counters add together. A threat this leaves fully paid is countered as it lands, or as soon as the card comes into play (counter.rs:155-171; mod.rs:232, 307-311).
- **Printed amounts:**
  - Tier 1: 3 energy, 3 ambassadors, 6 insight, 3 research cubes, 6 chips, or 2 tags; or 12 credits.
  - Tier 2: 4 energy, 4 ambassadors, 7 insight, 5 research cubes, 7 chips, or 3 tags; or 17 credits.
  - Scaling is `counter_pct` 100/100 and `counter_credit_pct` 100.
- **Difficulty (resource items only; jams and standing counters are subtracted after):**
  - From difficulty 1: +1 on each non-credit item, +4 on a credits item, +0 more on the first option (`counter_bonus` 0).
  - From difficulty 2: another +1 on each non-credit item, +0 more on credits.
  - So from difficulty 2: +2 non-credit and +4 credits (counter.rs:51-91).

**Executing** (mod.rs:403-449)
1. **Payments first.** Any threat now fully paid is countered. Every other threat's payments go back to the payers. This includes a waiting threat, so its payments must be made again next round.
2. **Order.** Countries in order; within a country, the order dealt. A threat dealt after the action phase began, or delayed, is skipped this round (mod.rs:228-230, 414-418).
3. **Each other threat in turn:**
   - **(a) Pick the player it hits.** The player whose node is fewest hexes from any of its tokens, wherever that node is (near.rs:211-229). On a tie, the tied players pick one. Players with no node on the board can't be hit. If nobody has a node, only the board consequences happen.
   - **(b) Resolve its printed consequences.** Player harms hit only that player. Pieces it places go on its token: the token of the named type, or else its first token. "Near it" effects use the land hexes on or next to its tokens (ctx.rs:104-127).
   - **(c) Compute.** The adversary gains the board's printed compute per uncountered threat: 0 normally, +4 from difficulty 2 (Militarist difficulty line; adversary.rs:325-331).
   - **(d) Discard it** with its tokens.
- **Harm conventions:**
  - **"Pay N or discard M cards":** your choice. If you can do neither, pay all you have and discard your whole hand (new_ops.rs:431-465). Theft works the same, and the adversary gains its compute either way (new_ops.rs:168-176).
  - **"Destroy a node":** takes the hit player's node closest to the threat. It never takes a last node (team.rs:107-125).
- A threat nobody is near still executes.

## Player count and difficulty

**Player count.** There is one country per player, 1 to 4 (docs/rules/board.md:7). Each pull deals one threat per country, so the number of threats each round grows with the player count. Counter costs do not change with player count.

**Solo (1 country).**
- With one country there are no fronts, so there are no military bases. A threat whose only location is a base can never be dealt; the draw rule skips it. A threat with a base and another location gets only its other token.
- Ties for the nearest player can't happen.
- Pooling counters is just your own payment.

**2 to 4 countries.** At difficulty 4, the extra threat rotates through the countries by round: country (round − 1) mod n.

**Difficulty** (`adversary.difficulty` 0-4; configs/baseline.json):
- **d0:** printed costs.
- **d1** (`counter_bonus_at` 1):
  - each non-credit counter item +1 (`counter_bonus_all`);
  - each credits item +4 (`counter_bonus_credits`);
  - the first option gets +0 more (`counter_bonus`).
- **d2** (`counter_more_at` 2):
  - non-credit items +1 more, so +2 in total (`counter_more_all`);
  - credits +0 more (`counter_more_credits`), so +4 in total.
- **d2** (`board_difficulty_at` 2): the Militarist difficulty line gives the adversary +4 compute for each threat that executes uncountered (and 1 more Deploy troop a board, which belongs to another area).
- **d3:** no direct threat change. `stage2_removed` 2 makes the game shorter, with fewer stage-II rounds.
- **d4:**
  - `extra_threats` 1: an extra pull of the event card's first pull each round, with the country rotating;
  - `tier2_early` 1: a stage-I card's tier-1 pulls become tier 2 once a stage-II or later card is the next face-up card; this also sets the tier of Discord extra threats;
  - `stage_swap_at` 4: one stage-I card is removed and one stage-III card is added, which means more two-pull rounds.

Below d4 no level adds a threat.

## Mismatches (engine vs docs/rules)

- **From counter_bonus_at a credits item costs counter_bonus_credits (3) more** (docs/rules/threats.md:71): engine: counter_bonus_credits is 4 (configs/baseline.json:80; crates/aic-core/src/modules/threats/counter.rs:57,83)
- **From counter_more_at credits cost counter_more_credits (2) more again** (docs/rules/threats.md:73): engine: counter_more_credits is 0: difficulty 2 adds nothing more to credits (total +4 from d1 on) (configs/baseline.json:83; crates/aic-core/src/modules/threats/counter.rs:59-62)
- **New-player summary: difficulty 1 makes counters cost 1 more of each thing (3 more credits), 2 adds 1 more again (3 credits)** (docs/rules/threats.md:111-112): engine: d1: +1 non-credit, +4 credits; d2: +1 more non-credit, +0 credits (configs/baseline.json:79-84; counter.rs:51-91)
- **New-player summary: difficulty 3 gives the adversary 4 compute for each unpaid threat** (docs/rules/threats.md:112): engine: The Militarist difficulty line (threat_compute 4) applies from board_difficulty_at = 2, i.e. difficulty 2 (configs/baseline.json:112; content/adversary.json:54-56; crates/aic-core/src/modules/adversary.rs:325-331)
- **New-player summary: 'only 4 deals an extra threat a round' as d4's threat change** (docs/rules/threats.md:113): engine: d4 also turns a stage-I card's tier-1 pulls into tier 2 when the next face-up card is stage II+ (tier2_early 1); threats.md never states this (crates/aic-core/src/modules/threats/mod.rs:366-367; adversary.rs:183-200)
- **Threats that deal a threat: Provocation, Last Ultimatum** (docs/rules/threats.md:33-34): engine: Full Spectrum Assault (tier 2) also deals a tier-1 threat into its country (content/threats.json (full_spectrum_assault deal_threats); crates/aic-core/src/modules/effects/new_ops.rs:44-55)
- **New-player summary: turn up cards until one has a listed tag** (docs/rules/threats.md:98-99): engine: The card must also have an available site of one of its location types in that country, or it is discarded (crates/aic-core/src/modules/threats/mod.rs:215; near.rs:73-75)

## Hard to state

- [medium] **Token placement fallback: tokens go in printed order, but if that leaves one off the board, the type with fewer sites goes first**: It is an exception, depends on counting sites, and a player would not predict it; the doc itself says a printed game does it by eye Proposed: Place the token whose type has fewer available sites first, always (no conditional retry); or print one location type per threat
- [medium] **Choosing which site gets a token: most adversary pieces on or next to it, ties lowest hex, factory falls back to an empty industrial zone, cities and stations must be uncorrupted but bases need not be**: Four sub-conditions differ by site type; needs counting over 7 hexes per candidate Proposed: 'Put each token on the uncorrupted site of that type with the most adversary pieces on or next to it (ties: lowest number). No factory? Use an empty industrial zone.' and make bases follow the same 'uncorrupted' wording
- [low] **Draw rule skips a card lacking a site in the country; after a second reshuffle the card is forced**: Location condition is a hidden second filter; forced pull is an exception that never happens Proposed: State only 'skip a card that doesn't fit the tags or has no site here'; drop the forced-pull clause from the rulebook (content guarantees it)
- [medium] **Timing of waiting threats: a threat dealt after the action phase has begun waits a round; one dealt during it (by a card) executes this round**: Needs knowing which phase a deal happened in; threats dealt by threats at execution wait, ones dealt in Sway don't Proposed: 'A threat executes only at the end of a round in which it was on the board during the whole action phase'; or put a 'waits' clock marker on any threat dealt after the action phase
- [medium] **Partial payments on a delayed (waiting) threat are refunded at round end**: Undocumented; a player would expect money on a threat to stay since the threat stays Proposed: Either keep payments on a waiting threat, or say 'at round end all unfinished payments come back, on every threat'
- [medium] **Who it hits: nearest node by hex distance, wherever, not necessarily a player 'near' it; ties picked by the tied players**: Two different distance concepts (near = 1 hex with node or ambassador, for paying; nearest = node only, any distance, for harm) Proposed: Name them distinctly on the card ('payers' vs 'target'), and state ambassadors never count for the target
- [low] **Difficulty counter surcharges: +1 per non-credit item and +4 credits from d1, +1 more non-credit from d2, applied only to resource items, before jams and standing counters**: Different steps for credits vs other resources, a separate first-option bonus (0), and ordering relative to jams Proposed: One line per level on the difficulty card: 'd1: counters +1 (credits +4); d2: counters +2 (credits +4)'; drop the unused first-option bonus
- [low] **Jam / delay / card counter reach only threats you are near; typed ones only matching tag or Universal; Unity rewards reach any; standing counters reach every threat of their type anywhere**: Four reach rules for similar effects Proposed: 'A card you play affects threats you are near; Unity and standing counters affect any threat' as one sentence
- [low] **Extra threats at d4 rotate the starting country by round; Discord extra threats always start at country 0**: Two rotation conventions for 'extra threat, countries in turn' Proposed: Use one rule for both (e.g., always start with the country after the last one that received an extra)
- [low] **'Here' on a threat with two tokens means the token of its named type, else its first placed token**: Forward-reference to printed order and to which tokens actually landed Proposed: Always print the target type on any placing consequence ('at its city token')

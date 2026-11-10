# adversary (the Militarist: round steps, income, Rally, Deploy, Corrupt, natural corruption, botnets, projects, event deck and round cap, difficulty 0-4), RULES_REVISION 73: engine-verified rules (172 verifier)

**Pieces and setup** (adversary.rs:360-400, board/mod.rs:271-290)
- The adversary has **agents** (from a shared supply of `supply_per_country` 24 × countries; one country per player), **troops** (unlimited, never from a supply), **botnets**, **adversary cubes** on corrupted sites, and **land corruption** tokens. (Operatives are a player Security card token, not an adversary piece.)
- Setup: each country starts with `start_corrupted` 1 corrupted city and its printed research station corrupted with a botnet on it. The event deck is built (below) and its top card goes face up as round 1's card. Incomes are set (below) and each board's pool gets one agent income for round 1. The project deck (all 20 projects) is shuffled and dealt into the slots.

**The event deck: the clock** (adversary.rs:245-302, 369-382)
- Deal `stage_cards` (4 / 4 / 3) random cards from each stage, adjusted by difficulty; shuffle each stage alone and stack stage I on top, then II, then III. Rounds = cards: 11 at d0-2, 9 at d3-4.
- One card is always face up as next round's card. If a round starts with no face-up card, the players lose.

**The adversary's turn, in engine order** (configs/baseline.json round list; adversary.rs:402-521). After the alignment sway step:
1. **Event.** The face-up card becomes this round's card; the old one is discarded. None: players lose.
2. **Threat.** The card's pulls deal threats on each board (threats area). At d4 there are extra pulls (see Difficulty).
3. **Build**, in this order:
   1. **Rally X**: each board's pool lands (below).
   2. **Deploy N**: on each board, N troops, each on the lowest-numbered factory (any owner) that holds no Protector node, else the lowest-numbered such city. From d2, +1 troop a board on any card that prints a Deploy line. A card with no Deploy line gets no troops.
   3. The card's other lines (at most two): an income increase (it counts at this round's income step), a site opening, a blackout, a Sway toward Discord.
   4. **Troops where the Protector stands**: on each board where a player with Wall has a node, `wall_troops` 1 troop lands by the Deploy rule. The d2 +1 does not apply to it.
   5. **Ongoing projects** act, in the order they resolved. A project resolved this round starts next round.
   6. **Botnets**: each botnet makes `botnet_compute` 1 compute for the projects. It makes none if a player ambassador is on its hex, or if a hack cut it this round.
4. **Corrupt**: the card's Corrupt X lines, in printed order (below).
5. **Natural corruption**: every city, then every research station, then every factory not already the adversary's that holds at least its corrupt number of agents on its own hex corrupts, with no pulling. A city's corruption markers count as agents. Land never corrupts this way.
6. **March** (war area).
7. **Reveal and income**: the top deck card goes face up as next round's card. Each board's pool gains the agent income, then compute income goes onto the projects. Then every firewall comes off, and any prohibition that acted this turn ends.
Then the players' turn, production, upkeep, threat execution and war follow.

**Income** (adversary.rs:115-119, 383-394, 936-954)
- **Compute income** (one number for the whole game, not per board): `compute_income` by difficulty, 3 / 4 / 18 / 24 / 13.
- **Agent income** (per board): `ambassador_income` 0 / 3 / 3 / 3 / 2, plus 1 from the Militarist's **Garrisons**, so 1 / 4 / 4 / 4 / 3 for each board each round.
- Event lines and Recruitment Drive raise agent income by 1 for the rest of the game. Propaganda Network raises compute income by 2.

**Rally X** (adversary.rs:961-1016, plan.rs:221-257)
- X is city, coastal city, research station, factory or botnet hex. Each board empties its whole pool onto its X sites.
- **Eligible sites**: the neutral X sites (no cube). If there are none, the players' X sites. If there are none of those, the adversary's. Firewalled sites are skipped before this choice.
- Every eligible site gets pool ÷ sites (rounded down). The players then place the remainder one each on different sites.
- The pool is emptied even when nothing lands: a cordoned board, a board with no X site, or an empty supply loses those agents.
- Each arriving agent on a hex holding a player ambassador cancels it: the ambassador is removed and the agent goes back to the supply.
- A Rally prohibition keeps a covered site's share in the supply. The remainder goes only to uncovered sites, and any excess stays in the supply.
- A project's "Rally N" lands N on each board by the same rule and does not touch the pool. "The adversary builds now" (Grand Offensive) rallies one agent income on each board to this round's card's X.

**Corrupt X** (adversary.rs:1232-1270, plan.rs:100-187)
- **Corrupt numbers**: city `corrupt_at` 4, research station 3, factory 3, land 3. From d1, city, station and factory are each 1 lower (3 / 2 / 2; land stays 3). "(1 fewer)" lowers it one more. The minimum is 1.
- **Targets**: every X not already the adversary's. Land means an open land hex with no building and no land corruption.
- Board by board, targets go in hex-number order. A target counts the agents on its hex (a city's markers count too) plus the agents on adjacent hexes in its board.
- If the total reaches the number, the target takes all its own agents plus just enough from neighbors, lowest-numbered first. All of them go back to the supply, and the target corrupts. An agent pulled once can't count for a later target.
- A firewalled target can't pull. It corrupts only on what it already holds.
- A target under a Corrupt prohibition pulls nothing and doesn't corrupt.

**What corruption does** (adversary.rs:1200-1230, board/destroy.rs:34-80)
- A city, station or factory gets the adversary cube and loses any player cube. Land gets a land corruption token, which sways `blight_discord` 1 toward Discord every round.
- The agents on the hex go back to the supply.
- A node on that hex alone is destroyed, unless one of these stops it, checked in order:
  1. It is the owner's last node before stage `last_safe_stage`.
  2. A Shelter is up.
  3. A hardening token is on the hex (used up).
  4. The owner's ambassador is on the hex (used up).
  5. Sanctuary or Civic Shelter applies.
- After that, every player ambassador on the hex is lost and markers come off.

**Botnets** (adversary.rs:363-367, 445-469, 1125-1134; board/ambassadors.rs:9-13)
- Each board starts with one. More come only from printed lines.
- Each botnet makes 1 compute a round (above). No one can claim a site on or next to a botnet.
- Botnets count against the purge until its third step.
- Only card effects remove botnets. Each removal sways `alignment.botnet` 1 toward Unity.

**Projects** (adversary.rs:723-903)
- The Militarist has 3 slots: Military, open, open.
- **Dealing**: turn up the top project. It goes into the first empty slot it fits (Universal fits all), otherwise to the project discard. When the deck is empty, the discard is shuffled in.
- The slots refill only when all three are empty. Resolved projects never come back.
- **Cost** = printed cost × number of players.
- **Placing compute**: each point goes, one at a time, onto the cheapest visible project (on a tie, the earlier slot). A full project resolves at once: a one-shot effect, or "ongoing" (its "Now" part at once, its ongoing part every build from next round). If no project is left anywhere, compute is lost.
- **Compute sources**:
  - compute income and botnets;
  - adversary research stations (`station_cubes` 1 each in production, none with a player ambassador on it);
  - an agent on a lone node, in production (1);
  - threats' printed compute;
  - from d2, 4 for each threat that executes uncountered;
  - Polar Compute Array's adversary cubes.
  So projects can resolve during production and threat execution as well as in the adversary turn.
- Projects pick fixed targets (lowest-numbered).

**Difficulty** (each check is "difficulty ≥ level")
- **d0**: the base rules.
- **d1+**: corrupt numbers −1 (city, station, factory). Threat counters cost more: each non-credit item +1 (`counter_bonus_all`), each credit item +4 (`counter_bonus_credits`); `counter_bonus` (extra on the first option) is 0.
- **d2+**: counters again +1 on each non-credit item (`counter_more_all`), +0 credits. The board's difficulty line: Deploy +1 troop a board, and +4 compute per uncountered threat.
- **d3+**: 2 fewer stage-II cards (`stage2_removed`), so a 9-round deck (4/2/3).
- **d4**:
  - Swap one stage-I card for a stage-III card (3/2/4, still 9).
  - 1 Discord card removed, counted as a tip already made.
  - When next round's card (the deck top) is stage II, a stage-I card's tier-1 pulls deal tier 2.
  - 1 extra pull of the card's first pull each round, on country (round−1) mod countries.
  - `extra_corrupted` 0 more corrupted cities.
- **Incomes** follow their own lists at every level (see Income); d4's are lower than d3's.

## Player count and difficulty

**Player count** (one country/board per player, 1-4; README, board.md):
- Per board, so it scales with players: the agent supply (24 per country, shared), agent income and pools, threats (each pull on each board), Deploy troops, Protector troops (boards where Wall has a node), starting botnets (one per country, so botnet compute ≈ players a round), and Rally and Corrupt (each board separately).
- Compute income is fixed (not per board), while project cost = printed × players (adversary.rs:171-173). Solo: projects cost their printed number and fill fast from income (3 a round at d0). At 4 players they cost 4×, so income alone resolves projects much more slowly.
- d4's extra threat is 1 in total, rotating by round over the countries (always country 0 in solo).
- Event deck length and round cap don't depend on player count.

**Difficulty** (baseline.json adversary params; each check is ≥ level):
- **d0**: incomes 3 compute, 1 agent a board (0 + Garrisons 1). Deck 4/4/3 = 11 rounds. Corrupt numbers city 4, station 3, factory 3, land 3.
- **d1**: incomes 4 compute, 4 agents a board. Corrupt numbers −1 (3/2/2, land 3). Counter items +1 (credits +4 instead).
- **d2**: incomes 18 compute, 4 agents. Counter non-credit items +1 more. The board's difficulty line: each printed Deploy +1 troop a board, and +4 compute per uncountered threat.
- **d3**: incomes 24 compute, 4 agents. 2 fewer stage-II cards (4/2/3 = 9 rounds; stage II from round 5).
- **d4**:
  - Incomes 13 compute, 3 agents (lower than d3).
  - Deck 3/2/4 = 9 rounds, stage II from round 4.
  - 1 Discord card removed and counted as a tip.
  - Tier-1 pulls deal tier 2 when next round's card is stage II.
  - 1 extra threat a round (a copy of the first pull, countries in turn).
  - `extra_corrupted` 0 (no effect at current numbers).
- Player claim and liberate costs do not change with difficulty.

## Mismatches (engine vs docs/rules)

- **Difficulty: 'each level includes the ones below'; the generated table credits income only to level 3 ('The level's incomes')** (docs/rules/adversary.md, ## Difficulty heading and generated table row 3): engine: Incomes are per-level list entries read at every level: compute 3/4/18/24/13, agents 0/3/3/3/2 (+1 Garrisons). They change at d1, d2, d3 and d4, and d4 is lower than d3 (13 < 24 compute, 2 < 3 agents), so d4 does not include d3's incomes (crates/aic-core/src/modules/adversary.rs:115-119; configs/baseline.json adversary params; crates/aic-sim/src/docs/blocks.rs:376)
- **Corruption: the node's owner's ambassador on the hex is armor, used up instead (the only protection named besides Sanctuary)** (docs/rules/adversary.md, Corrupt X, 'A corrupted site' bullet): engine: Node destruction goes through destroy_piece: a last node is safe before stage last_safe_stage, then a Shelter flag, then a hardening token (used first, so the ambassador survives only to be removed as 'player ambassadors left on the hex'), then the owner's ambassador, then Civic Shelter. Sanctuary (CorruptionShield) skips destruction entirely (crates/aic-core/src/modules/board/destroy.rs:34-80; crates/aic-core/src/modules/adversary.rs:1209-1218)
- **Corrupt X counts 'agents (on it plus next to it)'; markers are said to count only for natural corruption** (docs/rules/adversary.md, Corrupt X paragraph vs step 5): engine: A Corrupt X line also counts a target city's corruption markers toward its number (held()) (crates/aic-core/src/modules/plan.rs:114-116,159)
- **d4 'tier-2 threats sooner' looks tier2_early cards ahead 'the face-up card first'** (docs/rules/adversary.md, paragraph after the difficulty table): engine: At the threat deal the face-up slot is empty (it was flipped in the Event step and the reveal comes later), so the window is the top deck card, which is next round's card. With tier2_early 1: the last stage-I card's tier-1 pulls deal tier 2 when next round's card is stage II (crates/aic-core/src/modules/threats/mod.rs:358-367; crates/aic-core/src/modules/adversary.rs:183-200)
- **A cordoned board's pool 'lands nowhere' (and the doc says nothing about a Rally with no site of type X)** (docs/rules/adversary.md, Rally X paragraph): engine: The pool is emptied before the cordon or no-site check, so those agents are lost, not kept for the next Rally. A Rally with no eligible site also empties the pool (crates/aic-core/src/modules/adversary.rs:971-984)
- **Deploy N: troops on the lowest-numbered factory, else city, 'none on a hex Wall holds'** (docs/rules/adversary.md, step 3): engine: A walled hex is skipped and the troop lands on the next lowest-numbered unwalled factory (cities only if every factory is walled or there is none). A Deploy prohibition, by contrast, does not redirect: that troop stays in the supply (crates/aic-core/src/modules/adversary.rs:1311-1341)
- **A firewall keeps every arrival and Corrupt pull off its hex** (docs/rules/adversary.md, Rally X paragraph; README glossary 'Firewall'): engine: True for arrivals and pulls onto the hex. But natural corruption ignores firewalls, and a firewalled target still corrupts on the agents it already holds. Agents on a firewalled hex can still be pulled out to a neighboring target. Firewalling the only neutral site makes Rally fall to the players' sites (crates/aic-core/src/modules/plan.rs:152-157,191-197,233-246)

## Hard to state

- [high] **Rally eligibility tiers (neutral, else players', else adversary's) combined with firewall skipping**: A three-tier fallback plus an exception: firewalling a neutral site can push the whole pool onto your own sites, which a player would not predict Proposed: Rally lands on every X site that isn't firewalled, evenly; drop the neutral-first tiers (or let a firewalled site keep its share in the supply, like a prohibition)
- [high] **Corrupt X gathering: own agents plus just enough from neighbors in the same board, lowest-numbered first, each agent once, targets in hex order, markers count for cities**: More than three sentences; a cascade order matters, and hand counting is error-prone (the doc itself flags it) Proposed: A target counts only agents on its own hex and adjacent hexes, and takes all of them (no 'just enough'); or drop pulling and keep only natural corruption with a lower number
- [high] **Incomes per difficulty are a separate list; d4's incomes are lower than d3's**: Contradicts 'each level includes the ones below'; a player can't predict it from the ladder Proposed: Print the incomes on a per-level row of the difficulty card and drop the 'includes the ones below' claim, or keep incomes monotone
- [medium] **Corruption's node protections: last node safe, Shelter, hardening, the ambassador's armor, Sanctuary/Civic Shelter, in order; then all player ambassadors there are lost**: A long ordered list spread over other rule files; hardening is spent before the armor, yet the ambassador is lost anyway Proposed: One rule: 'Corruption destroys the node on that hex unless a protection on it is spent'; corruption removes ambassadors only when nothing protected the node
- [medium] **Deploy skips a walled hex to the next factory, but a Deploy prohibition keeps the troop in supply**: Two blockers with opposite outcomes (redirect versus vanish) for the same step Proposed: Treat both the same: a blocked troop stays in the supply (or both redirect)
- [medium] **Difficulty-2 Deploy +1 applies only to cards that print Deploy, not to Deploy-0 cards or Protector troops**: Needs an exception ('a card with no Deploy line gets none'); the board says 'each event's Deploy' Proposed: Print the d2 bonus as 'Deploy +1 on every card' (Deploy 0 becomes 1), or put the bonus on the card faces
- [medium] **Pool emptied when the board is cordoned, has no site of type X, or the supply runs out**: Silent loss a player won't predict; 'lands nowhere' reads as kept Proposed: State it outright: 'Agents that can't land return to the supply'
- [medium] **d4 tier-2-sooner look-ahead (tier2_early)**: A forward reference to deck composition and a timing window that depends on when the card was flipped Proposed: 'At d4, stage-I cards' tier-1 pulls deal tier 2 in the last stage-I round' (or mark the last stage-I card)
- [low] **Projects refill only when all slots are empty; resolved projects leave the game but missed draws reshuffle**: Two different discard fates, and an emptying row changes where compute lands (the cheapest left) Proposed: Refill each slot as soon as it empties, and use one discard pile
- [low] **Ongoing projects: the Now part at once, the ongoing part from next round's build, except war and production ones, which act in those steps**: An exception and a delayed start Proposed: Print the step on each ongoing card ('Each build: ...', 'Each production: ...')

# 135: the new adversary model (r54)

The design doc's *Adversary model* is built and plays all of 134's drafts (`drafts/adversary/`, the source;
`scripts/content/load_adversary.py` writes `content/` and the generated listing `docs/adversary-cards.md`). Rules
revision 54 (bumped from the constant, 53 to 54, as the prompt said). Rules: [threats.md](../../../docs/rules/threats.md),
[adversary.md](../../../docs/rules/adversary.md), the round in [economy.md](../../../docs/rules/economy.md), war in
[war.md](../../../docs/rules/war.md).

## The read (fresh seeds 36392-36439, 48 a cell, the three pairs; r53 = main cfeb4b7 on the same seeds)

Tables: [read_r54.md](read_r54.md), [read_r53.md](read_r53.md), [read_counters.md](read_counters.md); script
[read135.py](read135.py); specs `experiments/read-135.json`, `read-135-counters.json`.

| | d0 | d1 | d2 |
|---|---|---|---|
| strategist r54 | 81.9 ±3.2 | 71.5 ±3.8 | 61.8 ±4.0 |
| strategist r53 | 92.4 ±2.2 | 81.9 ±3.2 | 70.8 ±3.8 |
| paired r54 - r53 | -10.4 ±3.7 | -10.4 ±4.7 | -9.0 ±4.9 |
| greedy r54 | 75.7 ±3.6 | 57.6 ±4.1 | 45.1 ±4.1 |
| greedy r53 | 81.2 ±3.3 | 64.6 ±4.0 | 50.7 ±4.2 |
| paired r54 - r53 | -5.6 ±4.3 | -6.9 ±5.3 | -5.6 ±5.1 |

d2 strategist by pair: B+P 64.6, B+R 50.0, P+R 70.8 (the ballpark's 45-85 holds). **Game length** is unchanged:
strategist 6.40 / 6.18 / 5.86 rounds at d0 / d1 / d2 (r53 6.47 / 5.97 / 5.85); wins come by the Unity deck around
round 6, so stage III (tier-3 threats) is rarely reached. **Losses** are node losses almost all the way (d2
strategist: 52 of 55; r53 37 of 42): a seat runs out of nodes, mostly to corruption (3.65 nodes a game at d2, r53
3.06) and Discord cards (1.69); threats themselves take 0.28 a game (nodes spent as a counter) and next to none
uncountered.

**Threats** (d2 strategist): 19.9 dealt a game, 16.3 countered, 1.6 executed uncountered (r53: 19.3, 15.7, 2.6);
per round about 2.5-3 countered and 0.5 uncountered in rounds 1-3, near 0 uncountered from round 4. Greedy leaves
about 7 a game uncountered. Counter kinds a game (d2 strategist): credits 9.9, energy 1.0, insight 0.7, ambassadors
0.08, research cubes 0.10, tags 0.11, nodes 0.23, a card 2.5, a standing counter cut to nothing 1.7.

**Peak holding per seat** (d2 strategist, r54 / r53): credits 54.6 / 65.1 (Builder), 61.3 / 63.0 (Protector), 57.0 /
61.2 (Researcher); insight 9.4-19.6; energy 6.7-9.2; chips 2.8-3.6; research cubes 0.8-1.8; ambassadors 2.9-4.9.
Every cell is in read_r54.md.

**The new counter kinds** (done-when 2), d2 strategist, on and off (`old_counters`: credits, energy, insight, chips
only) on the same seeds: ambassadors 0.08 / 0, research cubes 0.10 / 0, tags 0.11 / 0, nodes 0.23 / 0 a game. The
win gap off - on is -1.4 ±2.8 (no measurable value yet: tags cost 2-3 cards in play, ruling 1).

**Amendment 5572 (calibration).** d0-d1 are within reach (strategist 82% / 72%, greedy 76% / 58%), so no player
lever was screened. The adversary is about 10 points harder than r53 at every level.

## What the build does (each a candidate)

- **Round:** sway; `adversary.event` (the card flips; none: loss); `threats.deal` (its pulls); `adversary.build`
  (standing lines, twists, arrivals and build, Wall troops, Escalation, ongoing projects, botnets); spread; corrupt;
  march; reveal; draft and keep; actions; production (bases, produce, upkeep) then war; threats execute; cleanup.
- **Threats:** three decks; the draw rule (`threats.pull_forced` counts a forced pull: none in this read); tokens on
  the most-threatened site; near = a node or ambassador within 1; the menu (resources pooled with Pay/Withdraw;
  a tag or node option paid whole by one seat with the new `Action::CounterWith`; a node option never takes a last
  node); a jam or standing counter comes off every resource item; execute after war, then the unprinted Discord sway.
- **Projects:** one shuffled deck into the board's slots (Military, open, open) by the same draw rule; all resolved:
  refill. An ongoing project's `ongoing_use` and `each_production` ops are read where they act; its other ops act each
  build. New field `resolve` on a project: an ongoing project's one-shot line (Declaration of War's troops).
- **New ops** (17) and new fields (9): `crates/aic-core/src/modules/effects/new_ops.rs`. Readings of our own: a front
  fighting twice cancels its troops once and strikes twice, and without a war step that round fights once; "troops
  at base" = each country's lowest-numbered base (none at 1 country); extra corp troops come even when an ambassador
  diverts the base; `cities_near` takes uncorrupted cities; Cooling Failure stops only the near seat's own stations;
  Load Shedding stops building Actions that cost energy, not AI-board powers (the card says Actions).
- **Rulings** (Matt, 2026-11-06): 1 tag counters 2 at tier 1 and 3 at tier 2 (42 cards); 2 a 4th stage-III event,
  **Naval Blockade** (tier-3 Economic or Grid + tier-1 Military or Cyber; 4 compute; 2 troops a country; Factories
  next to clean cities gain 1; coastal arrivals); 3 an adversary factory frees as a corrupted station does (the Free
  a station action; a player ambassador on one stops its troop and stays: H3's question to Matt timed out on that
  default); 4 at most 1 extra corp troop a base a production step from every source together; 5 Regime Change 27
  credits / 7 ambassadors / 5 Diplomacy tags / 14 credits + 3 ambassadors; 6 nothing built; 7 Conscription 3 rounds.
- **Draft edits** (candidates, flagged on the cards): Cryptojacking's compute counts before its cube loss; Station
  Seizure's and Station Coup's "1 lands" reads "an adversary ambassador lands"; Declaration of War's troops are its
  resolve line.
- **The board's difficulty line** (Militarist, harder: +1 troop a standing troop line) applies from difficulty 3
  (`adversary.board_difficulty_at`, new). Difficulty 1's extra threats are extra pulls of the card's first pull;
  difficulty 4's "tier 2 sooner" deals a stage-I card's tier-1 pulls from tier 2.

## Replacing the old model, job by job

Sources and who may counter: near a token. The shield: gone. "No source: name a target": the tokens. `per_country`
and tier 2 from stage II: the pulls. The staged project deck (`project_cards`, `project_row`): the slots. The old
Militarist deck: the 15 drafted events (also the round cap: 11 through d2, 10 at d3-4). Steps 13-16: one snapshot,
then war. **Kept, with no new home:** the `shared_row` threats variant (plays as `countries`; its config and tests
load it); the `War` twist op (no card prints it; the clock still reads it); `threat_tier` (old-form `DealThreats`,
Discord cards); late-dealt threats waiting a round, delay, jam, typed counters (by tag now), counter_bonus; Escalation
and Wall troops (config numbers); a token-less threat reaching its whole country (the pull never deals one).

## Not done

- Tier-3 threats and stage-III events barely play (games end by round 6-7); their numbers are unread.
- `web/`: types and the cards-check fixture updated, but `npm run check` and the e2e were not run (no node_modules in
  the worktree).
- `speed.py`: no verdict across a rules revision; agent ms per decision read 3.4 (r54) against 4.7 (r53) for the
  strategist on a loaded box: a ratio, not a timing.

# 177: recalibrate r75 to the bands, with last-node tools (r76)

Built on main 0b6d8ec (r75, 176). `RULES_REVISION` 75 → 76, bumped from the constant (the prompt named no revision).
Seeds: dev 36800-36831 and 36832-36895 (screens), fresh 42277-42852 (sets a-l, 48 each); next free 42853.
Amendments 8069/8079 (Matt's answers), 8111 (Claim); messages 8147, 8161 (load), 8165 (outline).

## The premise had moved

The prompt was written before 176 landed. 176's fresh ladder (r75) already read 90.3 / 77.3 / 63.4 / 50.0, with d4
at 44.2 and last-node losses at 0-4% of the strategist's. So the work was the climb (friction #20), d4, centering,
and the agent's last-node play, not a rescue.

## Final dials (all candidates)

| Dial | r75 | r76 |
|---|---|---|
| `compute_income` d0-d4 | 3, 4, 18, 24, 13 | **1, 4, 22, 24, 24** |
| `ambassador_income` d0-d4 | 0, 3, 3, 3, 2 | 0, 3, 3, 3, **3** |
| `discord_removed` (d4) | 1 | **0** (d4 keeps every Discord card) |
| `extra_threats_at` | 4 | **5** (no level deals an extra threat) |
| strategist `w.last_node` (agent) | (none) | **120** |

Kept after a screen: `start_nodes` 1, `claim_city` 5, the corrupt numbers, `counter_more_credits` 0, `stage1_added` 0,
`tier2_early` 1, `stage_swap_at` 4. Both incomes now climb (never down), so level 4 is level 3 plus the stage swap and
earlier tier-2 threats. For the difficulty card: d4's old trade (lower incomes, a Discord card removed, an extra
threat a round) isn't needed. Its climbing incomes with the deck whole read below d3 for the strategist.

## Freeze line

The prompt's measure: two fresh 48-seed sets pooled (144 games a level a set). Each level reads only its own income
entries, so a level retuned after the first freeze was confirmed alone on the next free seeds.

| Done when | Value | Seeds | Met |
|---|---|---|---|
| d0 ≥ 90 | 88.5 ± 1.9 (a+b); c+d 88.5 ± 1.9; four sets 88.5 ± 1.3 | 42277-42468 | **unmet** (0.8 SE) |
| d1 70-80 | 79.9 ± 2.4 (a+b); c+d 79.5 | 42277-42468 | met |
| d2 55-65 | 58.3 ± 2.9 (i+j, compute 22) | 42661-42756 | met |
| d3 40-50 | 44.1 ± 2.9 (a+b); c+d 50.7, k+l 52.4; six sets 49.1 | 42277-42468, 42757-42852 | met on a+b; later pairs drift high |
| d4 below d3 | 46.9 ± 2.9 against d3 52.4 on the same seeds (k+l) | 42757-42852 | met |
| d4 oracle 95 | 91.7 ± 4.0 (44 of 48; d3 95.8 on 24) | sets a and b, first 8 a pair | **unmet** by value (0.8 SE) |
| last node < 10% of strategist losses, d1-d3 | 0 / 1 / 1% (3% on a-d) | as above | met |

Tables: [fresh1.md](fresh1.md) (a+b), [fresh2.md](fresh2.md) (c+d and all four), [confirm.md](confirm.md) (d2, d3/d4,
the slope screens and both oracle rows). Greedy beside it (set a, freeze 1: d2 at 18, d4 at 27): 92.4 / 77.1 / 68.8 /
57.6 / 46.5.

**d0 resists every allowed lever.** Its losses are 98% the round cap. Compute 3 / 2 / 1 / 0 read 86.5 / 88.5 / 91.7 /
84.4 on dev seeds. Raising the base corrupt numbers (with `corrupt_cut` holding d1 and up) read −3 to +1. A
player-board lever would lift d1 (79.7) over its ceiling too. 1 is d0's best compute.

**d2 sits on a steep step.** Paired on the four spent sets (576 games a cell), compute 18 / 20 / 21 / 22 / 23 / 24 /
25 / 26 / 27 / 28 read 66.8 / 61.8 / 62.7 / 59.5 / 57.8 / 56.2 / 58.9 / 54.7 / 55.2 / 53.8. The 64-seed dev screens had
24 flat and 28 at −6.8. Seed sets alone moved a level 10+ points: one 64-seed set read d3 at 56.8 against the fresh
44.1. Two retunes were spent on fresh seeds before the paired slope: 28 (e+f, 56.2) and 26 (g+h, 54.5).

**d4 against its oracle.** The oracle read 87.5 at compute 27 and 91.7 at 24, on the same 24 games, one game apart.
`tier2_early` 0 read 87.5 (strategist +1.0). Easing d4 further breaks the order: 2 agents at d4 read 47.7, above d3's
47.4 on the same seeds. So the order and the climb hold d4's oracle at 87.5-91.7 (24 games, SE about 6); the final dials read 91.7 again on
set b's seeds, 44 of 48 in all. Reaching 95
needs a d4-only easing that doesn't lift the strategist, and no allowed dial found one.

## Losses by cause (strategist, % of losses: round cap / Discord deck / last node)

| d | r75 (176, fresh) | r76 final | seeds |
|---|---|---|---|
| 0 | 100 / 0 / 0 | 98 / 2 / 0 | a-d |
| 1 | 70 / 30 / 0 | 76 / 24 / 0 | a-d |
| 2 | 8 / 90 / 2 | 8 / 91 / 1 | i+j |
| 3 | 28 / 68 / 4 | 36 / 63 / 1 | k+l |
| 4 | 17 / 80 / 2 | 27 / 67 / 6 | k+l |

## The last node (Matt: tools, not protection; config-only, amendment 8069)

- **`start_nodes` 2** (screen 1, r75's strategist, dev seeds): +1.0 / −5.2 / +4.2 / +3.1 / −1.0 at d0-d4, all
  within SE, so it was not kept. Cheaper placement isn't needed: the first spaces of both tracks already cost no energy.
- **The strategist learns it.** An adversarial review (3 finders, 12 verifiers) confirmed five eval faults. With
  them, the strategist read a seat's last node as just one more node:
  - 0 nodes scored like 1;
  - an ambassador on a node hex counted as armor against corruption, though r75's arriving agents cancel it first;
  - nothing priced an exposed last node beyond about 4 points;
  - the converge and push rules could tip a last node.

  Behind one weight, `w.last_node` (0 for every agent but the strategist's 120), the fix changes 295 of 480 dev games.
  Wins are flat within SE (+1.0 / −1.1 / −4.1 / +2.0 / −2.1) and last-node losses go to 0 at every level
  ([screen3.md](screen3.md)). A threat-forecast twin (`w.last_node_threat`) changed 2 of 480 games and was cut.
- **Mostly avoidable:** the share was 0-4% before the agent fix and is 0-6% after, d4 the highest. A share that
  small leaves too few losses to classify.

## Claim (amendment 8111)

A city Claim is legal in 24% of seat-rounds (a probe over 20 d0 games). The sim players never take it, because
Liberate, at the same 5 ambassadors, also makes a city yours and dominates it. Prices 5, 4, 3 and 2 played identical
games. At 1 the strategist claims 0.36 cities a game at d0 and 0.03-0.12 above that, with d0 −4.2 ± 4.7 (dev). That
is still not a live choice, and a city at 1 would undercut a station's 3. **Claim's city price stays 5: a city claim
is a card's job.** An eval link from `aligned` to the city's output (with `aligned_bonus`) is the route if Matt wants
it live.

## Standing measures (set a, r76 at d2 28 / d4 27 against r75)

From `ladder.py --measures`: d2 and d4 compute made 285 → 368 and 301 → 366 a game, agents placed at d4 75 → 96, node
corruption losses at d4 2.8 → 3.6, and projects resolved at d2 15 → 18. d0-d1 and d3 are within noise. These
predate d2's retune (22) and d4's (24).

## Checks

- Fast lane, `cargo test-all` and clippy: green.
- `docs --check` current.
- `loop_smoke.py --run`: no timeouts; at most 4 plays of one card.
- No card changed, so no give/get, `cards:fit` or `play_read` (Claim's give/get is above).
- The agent change's cost (`per_decision.py`, same binary, `w.last_node` 0 against 120, d2, 6 games, one thread):
  1.3 against 1.4 ms a decision (0.95×). The speed baseline isn't updated: it is r73's and gives no verdict across
  revisions.
- No rules file changed after the freeze binary (1ad42f1): later dials were configs, and later code was test constants
  and an unused import. The final binary plays the frozen one's games: 0 of 1,152 differ (d3/d4 on sets a-d,
  `samedirs177.py`).

Specs and readers: `specs177.py` (farm specs), `screen177.py` (assemble, paired read, causes), `same177.py` (games
differing between agents), `oracle_spec177.py`.

## For new players (r76's difficulty)

Pick a level 0-4. Each level up costs the players a little more against threats and gives the adversary at least
as much compute and as many agents as the level below. Level 3 shortens the event deck, and level 4 swaps a stage-I
card for a stage-III card and brings tier-2 threats a card sooner. There are no extra threats and no missing Discord
card at any level. Keep a second node out early: the first track spaces cost no energy.

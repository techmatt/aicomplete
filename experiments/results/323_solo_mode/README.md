# 323: one-player mode (r153, no bump)

Matt's rulings (Nov 25), now in [docs/rulings.md](../../../docs/rulings.md): solo is a valid game and the learning
mode (one board, one country, its two corrupted cities to win; any board, the Builder suggested), with level 1's
numbers built in and shown resolved, never as "N (M from level X)".

## What already worked, and what was added

`--set countries=1` played from source unchanged: the map's one-country layout, tipping per player, a country's
compute, projects and K (× 1), threats dealt in the one country, and Matt's ruling 7 (r44): "another player" means
you (gifts land on you, teammate triggers fire on your own plays, the Researcher's capstone insight comes to you).
Added only:

- **`cards.take`** (crates/aic-core/src/modules/cards/): a draw-and-keep draft. The seat takes `take` cards one at a
  time from the hand it built, nothing passed; once it has them (or the hand is gone) the rest are discarded. 0 (every
  other config) is the passing draft, untouched.
- **`configs/solo.json`**, derived by `scripts/configs/solo.py` (run by `regen.py`): countries 1, difficulty 1,
  `ai.boards` [0], `deal` 7, `take` 4, and a content overlay that prints solo's texts resolved.
- **The viewer:** a Board select beside Config for a one-country config (Builder marked suggested); the game's own
  content (its overlay applied, new wasm `content_for`) for adversary texts; solo hides the level chip and prints the
  research row's K as "K 3". A 7-card hand fits one row at 1366×768.
- **The agents:** the eval valued every card in the hand as a possible keep; under draw-and-keep only the best
  `take` less those drafted can be (the rest are discarded, not passed to a partner), so it now counts those alone.
  16 dev seeds a board, judge and engine, on and off: within noise, all at 88-100%.

## The draft counts: build 7, take 4

In a 2-player draft (the design target) a seat sees its own 4 cards and the 3 its partner passes it (the partner's
first pick never comes round): **7 different cards, 4 drafted.** Solo builds a 7-card hand (choosing each card's
tier, as ever) and takes 4, so it sees what a 2-player seat sees and drafts the same count; the keep (insight) is
unchanged. A 3-player seat sees 9; 7 matches the 2-player game the rules are tuned on. Solo picks all 7 tiers itself
where a 2-player seat picks 4; that small gain in control was left as it is.

## Solo's adjustments (also in [rules/board.md](../../../docs/rules/board.md#solo))

| Rule | At d1, 2 players | Solo |
|---|---|---|
| Level | chosen (0-4) | level 1's numbers built in; no level chip in the viewer |
| Draft | build 4, pass | build 7, take 4, discard 3 |
| "(level 1+)" on Long Patrols, Station Raids, the board's Botnets spread icon, Botnet Swarm | printed | dropped (content overlay) |
| Research row text, K | "Every K cubes (the level line's cubes a country, times the countries)"; K 3 = 3 a country | "Every 3 cubes"; K 3 |
| "Another player" (help cards, teammate triggers, Researcher capstone) | a partner | you (ruling 7, r44, unchanged) |
| Tipping, projects, compute, K | per player / country | × 1 (unchanged rules) |
| Threats: who pays, who is harmed | any player near / the country's player | you |

Nothing else needed a change: Unity floors' "every player", the four loss clocks, Liberate's 3 ambassadors, the
two-beat adversary turn and every multi-seat rule are as they were. Multi-seat play is identical to main: the
standard read spec on 8 dev seeds (96 games), 0 games differ, before and after the eval fix.

## The judge read (fresh seeds 52342-52533, 192 a board, 576 games)

Spec `experiments/archive/farm/solo.json` (`farm solo` finds it natively); kept at `out/keep/323/solo-fresh/`.

| Board | Games | Win % ± SE | Rounds mean | Min-max | 6-8 | 10+ | Rounds (games) | Losses |
|---|---|---|---|---|---|---|---|---|
| Builder | 192 | 92.2 ± 1.9 | 5.94 | 1-11 | 38% | 7% | 1:7 2:5 3:11 4:26 5:41 6:29 7:27 8:16 9:16 10:7 11:7 | Discord deck 7, event deck 7, research 1 |
| Protector | 192 | 99.5 ± 0.5 | 4.76 | 1-11 | 30% | 1% | 1:9 2:17 3:30 4:34 5:37 6:27 7:17 8:13 9:7 11:1 | event deck 1 |
| Researcher | 192 | 100.0 ± 0.0 | 4.72 | 1-9 | 24% | 0% | 1:3 2:6 3:17 4:66 5:52 6:31 7:7 8:8 9:2 | — |

Soft numbers, not dialed. **Solo plays easier and shorter than the 2-player d1 game:** the Builder sits in the d1
band (80-95), the Protector and Researcher above it; games average 4.7-5.9 rounds (30% in 6-8); 105 of 576 wins come
in rounds 1-3 (19 in round 1), against the rulings' "a round-2 or round-3 win is rare". One country has 2 corrupted
cities to free with one player's whole draft and reach; the 2-player game has 4 with two players. If solo should
read as d1 does, the levers are solo's own (its start, a solo level row), behind the solo config.

Greedy (the new player, native, the same seeds): Builder 94.3 ± 1.7, Protector 98.4 ± 0.9, Researcher 99.5 ± 0.5,
rounds 5.9 / 4.9 / 4.7: as a learning mode, a first game is very likely won on any board.

Per game (judge): 1.7-2.4 cards kept a round in rounds 1-4 of the 4 drafted (insight bound in ~1 round a game);
3 tier-1 and 0.2-1.1 tier-2 threats run uncountered; 2.7-2.8 city liberations (the 2 starting cities plus those
the adversary takes).

`design_check.py` on the read: `warnings: design: usage 11 (Builder Fortify II 0.03 outside 0.05-2.50, Researcher
Hack I 0.13 outside 0.25-2.50, Make antimatter 0.02 outside 0.05-0.75, ...); design: rounds d1 30% in 6-8, 3% at 10+`
(the usage bands are the 2-player game's; Polar Compute Array's converters read 0 because solo's one country has
only its own megacorp). On the tree with no read: `warnings: none`.

## Not changed, for Matt

- The rulings lock "Draft hand exactly 4 cards"; solo's 7-card build follows this prompt's solo draft (rulings.md:
  a prompt outranks it). The 4 drafted keep the count.
- A browser session or record saved before this change has no `cards.take` and no longer loads (every param is
  required, as with 278's `no_pass`).
- The playthrough's small policy (`play-policy.mjs`) loses its solo games (round 8-9, the research track); the
  judge wins them. Its games finish, which is the bar.

# 154 classification, r65 (seeds 38872-38967, strategist, three pairs, d0-d3)

Replay agent A: `planner:strategy=1,w.blocker=8` (values the purge), 2 replays a game (always identical: the planner's choices don't follow the agent seed), both sets. Each game's verdict is an HTML comment in the source of this file.


### Read

| Level | Games | Win % | Losses | Round cap | Cap share of losses | Discord deck | Nodes | Other |
|---|---|---|---|---|---|---|---|---|
| d0 | 288 | 95.8 | 12 | 8 | 67% | 4 | 0 | 0 |
| d1 | 288 | 78.8 | 61 | 10 | 16% | 51 | 0 | 0 |
| d2 | 288 | 61.5 | 111 | 9 | 8% | 102 | 0 | 0 |
| d3 | 288 | 47.2 | 152 | 74 | 49% | 78 | 0 | 0 |

## End state at the round cap (what the purge still needed, at the game's own level)

| Level | Cap losses | Unity drawn (mean; 0-2 / 3-5 / 6-8) | Discord left (mean) | Purge left (median) | Corrupted cities (mean) | Troops counted (mean, games) | Botnets counted (mean) | Adv. ambassadors counted (mean) | Near misses (purge left ≤ 2, or 8 Unity drawn) |
|---|---|---|---|---|---|---|---|---|---|
| d0 | 8 | 7.1; 0 / 2 / 6 | 3.2 | 5 | 3.6 | 19 (2) | 0.2 | 0.0 | 6 |
| d1 | 10 | 6.2; 0 / 3 / 7 | 0.8 | 6 | 4.5 | 31 (3) | 0.7 | 0.0 | 3 |
| d2 | 9 | 6.1; 0 / 3 / 6 | 0.1 | 6 | 4.9 | 38 (3) | 0.6 | 0.0 | 1 |
| d3 | 74 | 5.4; 7 / 29 / 38 | 0.9 | 6 | 5.1 | 43 (36) | 1.1 | 1.8 | 13 |

## Bound and replay, by level

| Level | Cap losses | Can't win (bound) | Could win | of them replay won (agent mistake) | replay lost (tight) | Purge route short of (games) | Ambassadors short (median, purge route) | Sway short (median, Unity route) |
|---|---|---|---|---|---|---|---|---|
<!-- d0 pr 38872: tight; short {}; sway gap -28.0; replays 0/2 ['deck_empty', 'deck_empty'] [1, 1] -->
<!-- d0 bp 38876: tight; short {'troops': 53}; sway gap -10.0; replays 0/2 ['deck_empty', 'deck_empty'] [83, 83] -->
<!-- d0 bp 38887: agent mistake; short {}; sway gap -31.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d0 br 38881: agent mistake; short {}; sway gap -43.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d0 br 38915: tight; short {}; sway gap -30.0; replays 0/2 ['deck_empty', 'deck_empty'] [1, 1] -->
<!-- d0 pr 38916: agent mistake; short {}; sway gap -29.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d0 bp 38915: tight; short {}; sway gap -31.0; replays 0/2 ['deck_empty', 'deck_empty'] [2, 2] -->
<!-- d0 br 38909: tight; short {'ambassadors': 2.979166666666668, 'troops': 52}; sway gap -10.6; replays 0/2 ['deck_empty', 'deck_empty'] [79, 79] -->
| d0 | 8 | 0 | 8 | 3 | 5 | troops 2, ambassadors 1 | 0.0 | 0.0 |
<!-- d1 br 38872: agent mistake; short {}; sway gap -41.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d1 bp 38875: tight; short {'ambassadors': 11.308333333333334, 'botnets': 1, 'troops': 89}; sway gap -9.2; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d1 bp 38876: tight; short {'ambassadors': 2.404166666666665, 'botnets': 1, 'troops': 69}; sway gap -6.6; replays 0/2 ['deck_empty', 'deck_empty'] [101, 101] -->
<!-- d1 pr 38875: agent mistake; short {}; sway gap -44.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d1 bp 38882: tight; short {}; sway gap -13.0; replays 0/2 ['discord_deck', 'discord_deck'] [92, 92] -->
<!-- d1 br 38923: tight; short {}; sway gap -4.0; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d1 pr 38929: agent mistake; short {}; sway gap -26.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d1 br 38942: tight; short {'ambassadors': 5.769444444444442, 'botnets': 2, 'troops': 79}; sway gap -1.2; replays 0/2 ['deck_empty', 'deck_empty'] [117, 117] -->
<!-- d1 br 38945: tight; short {'ambassadors': 9.773611111111116}; sway gap -15.1; replays 0/2 ['discord_deck', 'discord_deck'] [5, 5] -->
<!-- d1 bp 38942: tight; short {}; sway gap -12.0; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
| d1 | 10 | 0 | 10 | 3 | 7 | ambassadors 4, botnets 3, troops 3 | 0.0 | 0.0 |
<!-- d2 bp 38875: tight; short {'ambassadors': 2.5166666666666657}; sway gap -16.4; replays 0/2 ['discord_deck', 'discord_deck'] [125, 125] -->
<!-- d2 bp 38882: tight; short {'troops': 78}; sway gap -16.0; replays 0/2 ['discord_deck', 'discord_deck'] [5, 5] -->
<!-- d2 bp 38891: can't win; short {'ambassadors': 9.037500000000001, 'botnets': 2, 'troops': 105}; sway gap 5.0; replays 0/2 ['discord_deck', 'discord_deck'] [160, 160] -->
<!-- d2 bp 38895: agent mistake; short {}; sway gap -28.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d2 bp 38916: tight; short {'ambassadors': 7.902777777777779, 'botnets': 1, 'troops': 94}; sway gap -12.9; replays 0/2 ['deck_empty', 'deck_empty'] [127, 127] -->
<!-- d2 bp 38919: agent mistake; short {}; sway gap -27.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d2 br 38928: tight; short {}; sway gap -37.0; replays 0/2 ['deck_empty', 'deck_empty'] [3, 3] -->
<!-- d2 bp 38938: tight; short {}; sway gap -14.0; replays 0/2 ['deck_empty', 'deck_empty'] [140, 140] -->
<!-- d2 br 38944: tight; short {}; sway gap -26.7; replays 0/2 ['discord_deck', 'discord_deck'] [4, 4] -->
| d2 | 9 | 1 | 8 | 2 | 6 | ambassadors 3, troops 3, botnets 2 | 0.0 | 0.0 |
<!-- d3 bp 38872: tight; short {}; sway gap -39.2; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 br 38873: tight; short {}; sway gap -25.8; replays 0/2 ['deck_empty', 'deck_empty'] [1, 1] -->
<!-- d3 br 38875: can't win; short {'ambassadors': 17.04555555555556, 'botnets': 2, 'troops': 56}; sway gap 10.0; replays 0/2 ['deck_empty', 'deck_empty'] [117, 117] -->
<!-- d3 bp 38875: can't win; short {'ambassadors': 19.658888888888885, 'botnets': 3, 'troops': 75}; sway gap 10.4; replays 0/2 ['deck_empty', 'deck_empty'] [113, 113] -->
<!-- d3 pr 38875: tight; short {'ambassadors': 4.2666666666666675, 'botnets': 1, 'troops': 60}; sway gap -10.5; replays 0/2 ['deck_empty', 'deck_empty'] [102, 102] -->
<!-- d3 pr 38876: tight; short {'ambassadors': 0.9400000000000013}; sway gap -29.7; replays 0/2 ['deck_empty', 'deck_empty'] [124, 124] -->
<!-- d3 bp 38878: tight; short {'ambassadors': 7.573333333333327}; sway gap -7.3; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 br 38883: tight; short {}; sway gap -29.2; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d3 bp 38883: tight; short {}; sway gap -37.8; replays 0/2 ['deck_empty', 'deck_empty'] [3, 3] -->
<!-- d3 pr 38882: tight; short {}; sway gap -15.6; replays 0/2 ['discord_deck', 'discord_deck'] [100, 100] -->
<!-- d3 bp 38885: tight; short {}; sway gap -14.4; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 br 38887: can't win; short {'ambassadors': 24.912777777777777, 'botnets': 1, 'troops': 70}; sway gap 10.0; replays 0/2 ['deck_empty', 'deck_empty'] [114, 114] -->
<!-- d3 bp 38891: tight; short {'ambassadors': 2.9738888888888866, 'botnets': 2, 'troops': 78}; sway gap -7.2; replays 0/2 ['deck_empty', 'deck_empty'] [114, 114] -->
<!-- d3 br 38891: tight; short {}; sway gap -18.6; replays 0/2 ['deck_empty', 'deck_empty'] [3, 3] -->
<!-- d3 pr 38891: tight; short {'ambassadors': 1.6916666666666664}; sway gap -33.9; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 br 38892: tight; short {'ambassadors': 5.056666666666668, 'botnets': 1, 'troops': 67}; sway gap -2.4; replays 0/2 ['deck_empty', 'deck_empty'] [94, 94] -->
<!-- d3 br 38894: can't win; short {'ambassadors': 7.019999999999996, 'troops': 62}; sway gap 8.8; replays 0/2 ['deck_empty', 'deck_empty'] [80, 80] -->
<!-- d3 bp 38894: tight; short {'ambassadors': 12.293333333333337}; sway gap -8.1; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d3 bp 38895: can't win; short {'ambassadors': 13.044999999999998, 'botnets': 1, 'troops': 80}; sway gap 17.2; replays 0/2 ['deck_empty', 'deck_empty'] [139, 139] -->
<!-- d3 br 38895: can't win; short {'ambassadors': 13.826666666666668, 'botnets': 1, 'troops': 71}; sway gap 8.4; replays 0/2 ['deck_empty', 'deck_empty'] [99, 99] -->
<!-- d3 bp 38897: tight; short {'ambassadors': 10.165}; sway gap -0.8; replays 0/2 ['deck_empty', 'deck_empty'] [93, 93] -->
<!-- d3 br 38899: can't win; short {'ambassadors': 2.7550000000000026, 'botnets': 3, 'troops': 37}; sway gap 1.9; replays 0/2 ['deck_empty', 'deck_empty'] [61, 61] -->
<!-- d3 bp 38900: tight; short {'ambassadors': 1.7688888888888918}; sway gap -35.9; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d3 bp 38903: tight; short {'ambassadors': 1.5}; sway gap -25.0; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 bp 38908: tight; short {'botnets': 1, 'troops': 65}; sway gap -15.0; replays 0/2 ['deck_empty', 'deck_empty'] [91, 91] -->
<!-- d3 pr 38905: tight; short {}; sway gap -31.8; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d3 br 38909: tight; short {'ambassadors': 13.328888888888883}; sway gap -14.8; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 bp 38911: tight; short {'ambassadors': 4.496111111111112, 'botnets': 1, 'troops': 69}; sway gap -11.1; replays 0/2 ['deck_empty', 'deck_empty'] [99, 99] -->
<!-- d3 br 38911: tight; short {'ambassadors': 2.6116666666666646}; sway gap -16.9; replays 0/2 ['deck_empty', 'deck_empty'] [80, 80] -->
<!-- d3 pr 38912: tight; short {}; sway gap -28.8; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 bp 38914: can't win; short {'ambassadors': 30.095, 'botnets': 1, 'troops': 73}; sway gap 21.9; replays 0/2 ['deck_empty', 'deck_empty'] [132, 132] -->
<!-- d3 bp 38916: tight; short {}; sway gap -26.1; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 pr 38916: tight; short {'ambassadors': 13.903888888888886}; sway gap -16.5; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 bp 38917: tight; short {'ambassadors': 3.6416666666666657, 'botnets': 1, 'troops': 72}; sway gap -7.0; replays 0/2 ['deck_empty', 'deck_empty'] [116, 116] -->
<!-- d3 br 38919: can't win; short {'ambassadors': 20.18333333333333, 'botnets': 1, 'troops': 71}; sway gap 12.3; replays 0/2 ['deck_empty', 'deck_empty'] [99, 99] -->
<!-- d3 bp 38919: tight; short {'ambassadors': 10.386666666666663}; sway gap -18.9; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 br 38920: can't win; short {'ambassadors': 6.803333333333331, 'botnets': 1, 'troops': 63}; sway gap 7.9; replays 0/2 ['deck_empty', 'deck_empty'] [3, 98] -->
<!-- d3 br 38923: can't win; short {'ambassadors': 11.703333333333331, 'botnets': 1, 'troops': 63}; sway gap 10.0; replays 0/2 ['deck_empty', 'deck_empty'] [109, 109] -->
<!-- d3 br 38924: tight; short {'ambassadors': 12.876666666666665}; sway gap -14.2; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 bp 38925: tight; short {'ambassadors': 7.309999999999999, 'troops': 77}; sway gap -1.1; replays 0/2 ['deck_empty', 'deck_empty'] [105, 105] -->
<!-- d3 br 38926: tight; short {'ambassadors': 3.540000000000001, 'botnets': 1, 'troops': 67}; sway gap -4.2; replays 0/2 ['deck_empty', 'deck_empty'] [98, 98] -->
<!-- d3 br 38928: can't win; short {'ambassadors': 9.538333333333334, 'botnets': 2, 'troops': 68}; sway gap 2.8; replays 0/2 ['deck_empty', 'deck_empty'] [102, 102] -->
<!-- d3 pr 38923: tight; short {'troops': 59}; sway gap -19.0; replays 0/2 ['deck_empty', 'deck_empty'] [2, 2] -->
<!-- d3 br 38929: can't win; short {'ambassadors': 1.8116666666666639, 'botnets': 1, 'troops': 49}; sway gap 12.8; replays 0/2 ['deck_empty', 'deck_empty'] [94, 94] -->
<!-- d3 bp 38929: can't win; short {'ambassadors': 7.012777777777778, 'botnets': 1, 'troops': 58}; sway gap 0.4; replays 0/2 ['deck_empty', 'deck_empty'] [101, 101] -->
<!-- d3 pr 38928: tight; short {}; sway gap -32.8; replays 0/2 ['deck_empty', 'deck_empty'] [1, 1] -->
<!-- d3 pr 38929: tight; short {'botnets': 2, 'troops': 62}; sway gap -1.4; replays 0/2 ['deck_empty', 'deck_empty'] [112, 112] -->
<!-- d3 bp 38930: tight; short {}; sway gap -32.0; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 br 38925: tight; short {}; sway gap -17.6; replays 0/2 ['deck_empty', 'deck_empty'] [2, 2] -->
<!-- d3 bp 38931: tight; short {'botnets': 1, 'troops': 68}; sway gap -1.4; replays 0/2 ['deck_empty', 'deck_empty'] [98, 98] -->
<!-- d3 bp 38933: can't win; short {'ambassadors': 16.221111111111114, 'botnets': 1, 'troops': 65}; sway gap 17.6; replays 0/2 ['deck_empty', 'deck_empty'] [138, 138] -->
<!-- d3 br 38933: can't win; short {'ambassadors': 15.056111111111115, 'botnets': 1, 'troops': 62}; sway gap 7.9; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 pr 38933: agent mistake; short {'ambassadors': 13.025555555555556}; sway gap -24.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 br 38931: tight; short {'botnets': 2, 'troops': 56}; sway gap -9.4; replays 0/2 ['deck_empty', 'deck_empty'] [85, 85] -->
<!-- d3 br 38934: tight; short {'ambassadors': 1.1566666666666627}; sway gap -28.5; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 br 38937: can't win; short {'ambassadors': 8.226111111111116, 'botnets': 2, 'troops': 68}; sway gap 1.8; replays 0/2 ['deck_empty', 'deck_empty'] [95, 95] -->
<!-- d3 br 38942: tight; short {'ambassadors': 12.524999999999999, 'troops': 60}; sway gap -4.8; replays 0/2 ['deck_empty', 'deck_empty'] [3, 3] -->
<!-- d3 pr 38942: tight; short {}; sway gap -26.2; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 br 38945: can't win; short {'ambassadors': 14.508333333333336}; sway gap 3.3; replays 0/2 ['deck_empty', 'deck_empty'] [3, 3] -->
<!-- d3 br 38947: tight; short {}; sway gap -25.8; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d3 pr 38945: tight; short {}; sway gap -44.8; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 bp 38947: tight; short {'ambassadors': 8.885000000000002}; sway gap -16.7; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 pr 38948: tight; short {}; sway gap -21.2; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 bp 38949: can't win; short {'ambassadors': 16.189999999999998, 'botnets': 1, 'troops': 78}; sway gap 7.3; replays 0/2 ['deck_empty', 'deck_empty'] [108, 108] -->
<!-- d3 br 38950: tight; short {}; sway gap -22.6; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 bp 38951: agent mistake; short {}; sway gap -30.2; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 br 38952: agent mistake; short {}; sway gap -15.8; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 br 38954: can't win; short {'ambassadors': 21.591666666666665, 'botnets': 3, 'troops': 69}; sway gap 20.1; replays 0/2 ['deck_empty', 'deck_empty'] [111, 111] -->
<!-- d3 pr 38956: tight; short {'botnets': 3, 'troops': 60}; sway gap -1.4; replays 0/2 ['deck_empty', 'deck_empty'] [92, 92] -->
<!-- d3 bp 38957: tight; short {}; sway gap -29.5; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 br 38957: tight; short {'botnets': 1, 'troops': 50}; sway gap -17.0; replays 0/2 ['deck_empty', 'deck_empty'] [73, 73] -->
<!-- d3 br 38961: tight; short {'ambassadors': 2.113333333333337, 'botnets': 1, 'troops': 65}; sway gap -15.0; replays 0/2 ['deck_empty', 'deck_empty'] [89, 89] -->
<!-- d3 bp 38963: tight; short {'ambassadors': 5.010555555555552}; sway gap -20.1; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 bp 38966: can't win; short {'ambassadors': 2.7283333333333353, 'botnets': 1, 'troops': 50}; sway gap 0.7; replays 0/2 ['deck_empty', 'deck_empty'] [88, 88] -->
| d3 | 74 | 21 | 53 | 3 | 50 | ambassadors 46, troops 36, botnets 32 | 2.8 | 0.0 |

Design mistakes (can't win + tight): 90; agent mistakes: 11.

## Cross-check: replay agent B, `planner:strategy=1` (the ceiling, default weights), set a only (38872-38919)

## Bound and replay, by level

| Level | Cap losses | Can't win (bound) | Could win | of them replay won (agent mistake) | replay lost (tight) | Purge route short of (games) | Ambassadors short (median, purge route) | Sway short (median, Unity route) |
|---|---|---|---|---|---|---|---|---|
| d0 | 8 | 0 | 8 | 3 | 5 | troops 2, ambassadors 1 | 0.0 | 0.0 |
| d1 | 5 | 0 | 5 | 2 | 3 | ambassadors 2, botnets 2, troops 2 | 0.0 | 0.0 |
| d2 | 6 | 1 | 5 | 2 | 3 | ambassadors 3, troops 3, botnets 2 | 2.5 | 0.0 |
| d3 | 36 | 9 | 27 | 0 | 27 | ambassadors 25, troops 15, botnets 14 | 3.6 | 0.0 |

Design mistakes (can't win + tight): 48; agent mistakes: 7.

# The same seeds at r66 (late liberation 3, counter_bonus_credits 8, compute 5,5,8,16,16: before the final d2 step to 10), agent A


### Read

| Level | Games | Win % | Losses | Round cap | Cap share of losses | Discord deck | Nodes | Other |
|---|---|---|---|---|---|---|---|---|
| d0 | 288 | 96.9 | 9 | 7 | 78% | 2 | 0 | 0 |
| d1 | 288 | 78.8 | 61 | 8 | 13% | 53 | 0 | 0 |
| d2 | 288 | 67.4 | 94 | 4 | 4% | 90 | 0 | 0 |
| d3 | 288 | 45.8 | 156 | 61 | 39% | 95 | 0 | 0 |

## End state at the round cap (what the purge still needed, at the game's own level)

| Level | Cap losses | Unity drawn (mean; 0-2 / 3-5 / 6-8) | Discord left (mean) | Purge left (median) | Corrupted cities (mean) | Troops counted (mean, games) | Botnets counted (mean) | Adv. ambassadors counted (mean) | Near misses (purge left ≤ 2, or 8 Unity drawn) |
|---|---|---|---|---|---|---|---|---|---|
| d0 | 7 | 7.6; 0 / 0 / 7 | 3.6 | 3 | 2.7 | 0 (0) | 0.0 | 0.0 | 5 |
| d1 | 8 | 6.8; 0 / 2 / 6 | 0.6 | 5 | 4.0 | 24 (2) | 0.6 | 0.0 | 4 |
| d2 | 4 | 6.8; 0 / 0 / 4 | 0.2 | 5 | 4.5 | 0 (0) | 0.0 | 0.0 | 0 |
| d3 | 61 | 6.4; 1 / 15 / 45 | 0.9 | 5 | 4.6 | 24 (16) | 0.5 | 0.8 | 19 |

## Bound and replay, by level

| Level | Cap losses | Can't win (bound) | Could win | of them replay won (agent mistake) | replay lost (tight) | Purge route short of (games) | Ambassadors short (median, purge route) | Sway short (median, Unity route) |
|---|---|---|---|---|---|---|---|---|
<!-- d0 bp 38876: tight; short {}; sway gap -16.8; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d0 pr 38872: tight; short {}; sway gap -28.0; replays 0/2 ['deck_empty', 'deck_empty'] [1, 1] -->
<!-- d0 bp 38887: agent mistake; short {}; sway gap -30.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d0 pr 38916: agent mistake; short {}; sway gap -27.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d0 br 38909: agent mistake; short {'ambassadors': 2.679166666666667}; sway gap -13.9; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d0 bp 38915: tight; short {}; sway gap -31.0; replays 0/2 ['deck_empty', 'deck_empty'] [2, 2] -->
<!-- d0 br 38915: tight; short {}; sway gap -30.0; replays 0/2 ['deck_empty', 'deck_empty'] [1, 1] -->
| d0 | 7 | 0 | 7 | 3 | 4 | ambassadors 1 | 0.0 | 0.0 |
<!-- d1 br 38880: tight; short {}; sway gap -21.0; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d1 bp 38885: tight; short {'ambassadors': 5.901388888888889, 'botnets': 1, 'troops': 77}; sway gap -4.6; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d1 br 38911: tight; short {}; sway gap -17.0; replays 0/2 ['deck_empty', 'deck_empty'] [3, 3] -->
<!-- d1 bp 38919: agent mistake; short {}; sway gap -20.4; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d1 bp 38921: agent mistake; short {}; sway gap -36.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d1 pr 38928: agent mistake; short {}; sway gap -22.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d1 br 38942: tight; short {'ambassadors': 2.625, 'botnets': 1, 'troops': 74}; sway gap -4.3; replays 0/2 ['deck_empty', 'deck_empty'] [108, 108] -->
<!-- d1 br 38961: agent mistake; short {}; sway gap -29.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
| d1 | 8 | 0 | 8 | 4 | 4 | ambassadors 2, botnets 2, troops 2 | 0.0 | 0.0 |
<!-- d2 bp 38885: agent mistake; short {'ambassadors': 12.1375}; sway gap -11.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d2 br 38945: tight; short {}; sway gap -21.0; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d2 bp 38949: tight; short {'ambassadors': 3.9652777777777786}; sway gap -15.7; replays 0/2 ['discord_deck', 'discord_deck'] [4, 4] -->
<!-- d2 br 38952: can't win; short {'ambassadors': 7.454166666666666}; sway gap 3.5; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
| d2 | 4 | 1 | 3 | 1 | 2 | ambassadors 3 | 7.5 | 0.0 |
<!-- d3 br 38875: tight; short {'ambassadors': 1.7283333333333317, 'botnets': 1, 'troops': 51}; sway gap -2.9; replays 0/2 ['deck_empty', 'deck_empty'] [100, 100] -->
<!-- d3 br 38877: tight; short {}; sway gap -31.8; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d3 br 38882: agent mistake; short {}; sway gap -26.2; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 br 38883: agent mistake; short {}; sway gap -36.8; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 bp 38883: tight; short {}; sway gap -28.8; replays 0/2 ['deck_empty', 'deck_empty'] [3, 3] -->
<!-- d3 pr 38879: agent mistake; short {}; sway gap -36.8; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 pr 38885: tight; short {'ambassadors': 2.8711111111111123}; sway gap -24.2; replays 0/2 ['deck_empty', 'deck_empty'] [107, 107] -->
<!-- d3 bp 38885: tight; short {'ambassadors': 2.133333333333333}; sway gap -14.8; replays 0/2 ['deck_empty', 'deck_empty'] [111, 111] -->
<!-- d3 bp 38887: tight; short {}; sway gap -22.2; replays 0/2 ['deck_empty', 'deck_empty'] [3, 3] -->
<!-- d3 pr 38887: tight; short {}; sway gap -17.6; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d3 br 38888: agent mistake; short {}; sway gap -21.8; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 bp 38895: tight; short {'ambassadors': 8.74166666666667}; sway gap -7.6; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 br 38899: tight; short {}; sway gap -41.8; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 bp 38899: tight; short {}; sway gap -19.7; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d3 pr 38897: tight; short {'ambassadors': 8.212222222222223, 'botnets': 1, 'troops': 54}; sway gap -1.4; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 bp 38900: can't win; short {'ambassadors': 12.484444444444446, 'botnets': 2, 'troops': 61}; sway gap 6.1; replays 0/2 ['deck_empty', 'deck_empty'] [92, 92] -->
<!-- d3 br 38900: tight; short {}; sway gap -19.8; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 bp 38907: tight; short {'ambassadors': 9.134999999999998, 'botnets': 2, 'troops': 72}; sway gap -14.0; replays 0/2 ['deck_empty', 'deck_empty'] [108, 108] -->
<!-- d3 br 38907: agent mistake; short {'ambassadors': 2.3633333333333333}; sway gap -32.4; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 pr 38907: agent mistake; short {}; sway gap -49.8; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 bp 38911: agent mistake; short {}; sway gap -30.8; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 pr 38914: agent mistake; short {'ambassadors': 14.481666666666662}; sway gap -8.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 bp 38916: can't win; short {'ambassadors': 22.898333333333337}; sway gap 8.8; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 br 38916: can't win; short {'ambassadors': 11.980000000000004, 'botnets': 2, 'troops': 71}; sway gap 0.2; replays 0/2 ['deck_empty', 'deck_empty'] [90, 90] -->
<!-- d3 pr 38916: tight; short {}; sway gap -31.8; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 bp 38919: tight; short {'ambassadors': 16.044999999999998, 'botnets': 1, 'troops': 63}; sway gap -0.2; replays 0/2 ['deck_empty', 'deck_empty'] [85, 85] -->
<!-- d3 br 38919: can't win; short {'ambassadors': 21.565, 'botnets': 1, 'troops': 71}; sway gap 17.1; replays 0/2 ['deck_empty', 'deck_empty'] [112, 112] -->
<!-- d3 br 38920: tight; short {'botnets': 1, 'troops': 67}; sway gap -2.8; replays 0/2 ['deck_empty', 'deck_empty'] [89, 89] -->
<!-- d3 pr 38921: tight; short {'ambassadors': 2.3716666666666697}; sway gap -36.0; replays 0/2 ['deck_empty', 'deck_empty'] [3, 3] -->
<!-- d3 bp 38924: can't win; short {'ambassadors': 10.11666666666667, 'botnets': 1, 'troops': 57}; sway gap 12.9; replays 0/2 ['deck_empty', 'deck_empty'] [156, 156] -->
<!-- d3 bp 38923: tight; short {}; sway gap -28.1; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 bp 38927: tight; short {'ambassadors': 16.306666666666665, 'troops': 51}; sway gap -3.4; replays 0/2 ['deck_empty', 'deck_empty'] [89, 89] -->
<!-- d3 br 38923: tight; short {}; sway gap -21.2; replays 0/2 ['deck_empty', 'deck_empty'] [91, 91] -->
<!-- d3 pr 38927: tight; short {'ambassadors': 5.9666666666666615}; sway gap -2.0; replays 0/2 ['discord_deck', 'discord_deck'] [79, 79] -->
<!-- d3 bp 38928: tight; short {}; sway gap -19.2; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d3 br 38925: tight; short {'ambassadors': 8.045000000000002, 'botnets': 1, 'troops': 77}; sway gap -7.0; replays 0/2 ['discord_deck', 'discord_deck'] [103, 103] -->
<!-- d3 br 38928: tight; short {'ambassadors': 4.092222222222219}; sway gap -23.9; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 bp 38930: tight; short {}; sway gap -7.6; replays 0/2 ['deck_empty', 'deck_empty'] [120, 120] -->
<!-- d3 br 38929: can't win; short {'ambassadors': 7.266111111111115, 'botnets': 1, 'troops': 45}; sway gap 12.3; replays 0/2 ['deck_empty', 'deck_empty'] [104, 104] -->
<!-- d3 pr 38929: tight; short {'botnets': 1, 'troops': 61}; sway gap -13.0; replays 0/2 ['deck_empty', 'deck_empty'] [92, 92] -->
<!-- d3 bp 38933: tight; short {'ambassadors': 13.68, 'botnets': 1, 'troops': 68}; sway gap -2.2; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 pr 38933: tight; short {'ambassadors': 15.651666666666664}; sway gap -3.1; replays 0/2 ['deck_empty', 'deck_empty'] [90, 90] -->
<!-- d3 br 38933: tight; short {'ambassadors': 0.884999999999998}; sway gap -17.4; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 bp 38937: tight; short {'ambassadors': 10.98777777777778}; sway gap -12.2; replays 0/2 ['deck_empty', 'deck_empty'] [101, 101] -->
<!-- d3 br 38938: agent mistake; short {'ambassadors': 7.786666666666665}; sway gap -24.3; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 pr 38940: tight; short {'ambassadors': 7.331666666666667}; sway gap -15.3; replays 0/2 ['discord_deck', 'discord_deck'] [3, 3] -->
<!-- d3 bp 38942: tight; short {'ambassadors': 14.53444444444445}; sway gap -24.7; replays 0/2 ['deck_empty', 'deck_empty'] [5, 5] -->
<!-- d3 pr 38942: agent mistake; short {}; sway gap -30.4; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 br 38942: tight; short {}; sway gap -45.9; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 br 38941: tight; short {}; sway gap -25.5; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 bp 38945: can't win; short {'ambassadors': 8.706666666666667, 'botnets': 1, 'troops': 79}; sway gap 10.6; replays 0/2 ['deck_empty', 'deck_empty'] [128, 128] -->
<!-- d3 br 38939: agent mistake; short {}; sway gap -26.0; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 bp 38948: tight; short {'ambassadors': 6.589999999999996}; sway gap -12.9; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 pr 38949: tight; short {}; sway gap -32.8; replays 0/2 ['deck_empty', 'deck_empty'] [4, 4] -->
<!-- d3 br 38950: tight; short {'ambassadors': 5.618888888888893}; sway gap -15.0; replays 0/2 ['deck_empty', 'deck_empty'] [3, 3] -->
<!-- d3 pr 38950: tight; short {}; sway gap -20.8; replays 0/2 ['deck_empty', 'deck_empty'] [3, 3] -->
<!-- d3 pr 38948: tight; short {}; sway gap -37.8; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
<!-- d3 br 38962: can't win; short {'ambassadors': 9.760000000000002, 'botnets': 1, 'troops': 62}; sway gap 7.9; replays 0/2 ['deck_empty', 'deck_empty'] [153, 153] -->
<!-- d3 pr 38962: agent mistake; short {'ambassadors': 8.96}; sway gap -28.9; replays 2/2 ['unity_deck', 'unity_deck'] [0, 0] -->
<!-- d3 pr 38963: tight; short {'ambassadors': 1.5883333333333383}; sway gap -34.4; replays 0/2 ['discord_deck', 'discord_deck'] [6, 6] -->
<!-- d3 br 38961: tight; short {}; sway gap -34.8; replays 0/2 ['deck_empty', 'deck_empty'] [6, 6] -->
| d3 | 61 | 8 | 53 | 12 | 41 | ambassadors 33, troops 16, botnets 15 | 1.7 | 0.0 |

Design mistakes (can't win + tight): 60; agent mistakes: 20.

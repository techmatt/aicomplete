# 136: alignment and the win (r55)

Rules revision 55 (bumped from the constant, 54 to 55, as the prompt said). Rules:
[alignment.md](../../../docs/rules/alignment.md), Envoy in [economy.md](../../../docs/rules/economy.md).
Working papers: [death_audit.md](death_audit.md) ([death_audit.py](death_audit.py)), the read tables
[read_r55_vs_r54.md](read_r55_vs_r54.md) and [read_tables.md](read_tables.md) (135's tables on both), script
[read136.py](read136.py), specs `experiments/read-136.json`, `read-136-choices.json`.

## Where things stood (135's README against the code)

The code matched 135's write-up: r54, all of 134's drafts live, `alignment.tug` with the 6-card Unity deal
(`unity_cards` 3, 3), `unity_ease` 2, 2, 4. On 136's fresh seeds r54 read 82.6 / 70.8 / 61.1% (d0-d2), median 6
rounds, 107 / 102 / 88 of 144 games won on the Unity deck, 16-21 a cell before round 6 (earliest round 3).

## The build (each number a candidate)

- **Two tracks.** `alignment.unity` / `alignment.discord` are tracks; Discord Sway goes straight on its track, Unity
  waits in `alignment.unity_new` until the action or phase resolves, then seat 0 (the team) chooses how much of it moves
  Discord off instead, 1 for 1 (`hold_off` 1), before any tip. Discord tips first. Thresholds `tip_per_player` 3 and
  `discord_tip_per_player` 3 (6 a tip at 2 players; were 2 and 2 with the pool); `carry` 1 (Unity carries, a Discord
  tip empties its track).
- **Envoy**, standard action: 1 ambassador from your supply (`board.envoy_ambassadors`), used up: 1 Discord off. Offered
  only with Discord on the track.
- **Unity deck:** all 9 dealt; tier 2 from 3 drawn (`unity_tier2` 3, the job `unity_ease[0]` did). **Purge:**
  `purge_at` 3, 6, 6 by Unity cards drawn (the resolved pile).
- **Unity sources:** base / factory Unity removed (the params `base_unity`, `factory_unity` too: the ambassador still
  stops the troop); `alignment.botnet` 1; `board.clear_unity` 2; `free_city` 5. New tallies `alignment.unity_from.*`.
- **Discord drip:** `adversary_discord` (0) became the adversary board's content field `discord_drip` (the Militarist
  0), read in the round's Sway phase.

**Where each old job went.** The Undecided (a cap on Sway in play, and through the tug the way Unity cancelled
Discord): gone, no cap; cancelling is now the players' hold-off choice and Envoy. `tug` / `still`: one variant
`tracks` (`off` stays); `still`'s "no cancel" is `hold_off` 0. A tip returning Sway to the pool: the tip's Sway comes
off the track. "Move N Sway from the Unity bucket to the Undecided": "Remove N Unity from the Unity track"
(`pull_unity`; Propaganda and five adversary drafts, rewritten by `scripts/content/track_words.py`).
`undecided_per_player`: gone, with its build check. `unity_cards`: gone.

**Give / get** (configs/agents/values.json): Envoy gives 1 ambassador (4.0, `resources.ambassadors`; the Ambassador
action sells one for 20 credits) and gets 1 Discord off (4.0 at `rates.sway`): **break-even at the table, flagged.**
The prompt sets its price ("as poorly as the other standard actions"); its gain is timing, the round a Discord tip
is near. The hold-off choice is 1 Unity (4.0) for 1 Discord (4.0), a choice of side, not a conversion. No table row
changed: agents read both through the eval's track terms.

## Screens (dev seeds 36600-36647, strategist, 48 games a cell then 144)

| Cell | d0 | d1 | d2 | median rounds | Discord / Unity tips (d2) |
|---|---|---|---|---|---|
| Unity 2 / Discord 2 (48 games) | 72.9 | | 45.8 | 8 / 7 | 4.2 / 5.6 |
| Unity 3 / Discord 2 (48) | 58.3 | | 43.8 | 7 / 7 | 4.3 / 4.8 |
| **Unity 3 / Discord 3** (144) | 80.6 | 61.8 | 55.6 | 8 / 8 / 8 | 3.3 / 5.8 |
| Unity 3 / Discord 3, Discord carry (`carry` 2, 144) | 70.1 | 54.2 | | 8 / 8 | |

r54 on the first 48 dev games: 79.2 / 52.1 at d0 / d2, median 6 / 5. Discord 2 a player loses too often; carrying
Discord's overflow sits on the 70 / 55 targets with no margin (asked Matt, design: frozen at carry 1).

## The read (fresh seeds 36440-36487, 48 a cell, the three pairs; r54 = main db0dee8 on the same seeds)

| | d0 | d1 | d2 |
|---|---|---|---|
| strategist r55 | 79.9 ±3.3 | 71.5 ±3.8 | 63.9 ±4.0 |
| strategist r54 | 82.6 ±3.2 | 70.8 ±3.8 | 61.1 ±4.1 |
| paired r55 - r54 | -2.8 ±4.3 | +0.7 ±4.4 | +2.8 ±5.2 |
| rounds r55: mean, median, p10-p90, range | 8.40, 8, 6-11, 5-12 | 8.10, 8, 6-10, 5-12 | 7.75, 8, 4-10, 4-12 |
| rounds r54 | 6.36, 6, 5-7, 3-11 | 6.14, 6, 5-8, 4-9 | 5.86, 6, 4-8, 4-8 |
| paired rounds gap | +2.03 ±0.16 | +1.97 ±0.14 | +1.89 ±0.16 |

**How games end** (r55, of 144): d0 Unity deck 60, purge 55, node 16, round cap 11, Discord deck 2; d1 68 / 35 / 37 /
3 / 1; d2 54 / 38 / 49 / 1 / 2. r54 had almost no purges (12 / 0 / 0) and no cap. **No Unity-deck win before round 6**
(earliest round 7 at every level; r54: 19 / 21 / 16 before round 6, earliest rounds 3-4).

**Tips a game** (r55 / r54): Unity 7.28 / 5.31, 6.88 / 4.94, 6.12 / 4.40; Discord 2.89 / 3.87, 3.35 / 4.10, 3.33 /
3.94. Discord tips fewer: the team holds 2.0-4.0 Discord a game off (Envoy 2.04 / 1.36 / 0.95 plays, Unity held off
1.97 / 1.35 / 1.08). **Threats a game** (d0 / d1 / d2, r55): dealt 18.9 / 26.6 / 26.5, countered 16.5 / 22.3 / 21.8,
uncountered 1.2 / 2.1 / 2.1 (r54 15.1 / 21.0 / 20.1 dealt, 1.0 / 1.9 / 2.1 uncountered): longer games reach tier 3.
**Peak holding per seat** (d2, Builder / Protector / Researcher, r55 vs r54): credits 82 / 79 / 95 vs 49 / 53 / 64;
insight 12.6 / 18.4 / 29.1 vs 9.4 / 12.6 / 20.7; energy 10.6 / 9.9 / 10.0 vs 8.7 / 7.6 / 7.9; chips 8.7 / 6.9 / 5.8 vs
3.8 / 3.4 / 3.1; ambassadors 4.7 / 7.3 / 7.1 vs 2.8 / 4.2 / 4.6. Every cell is in read_tables.md.

**Nodes lost a game** (d2): corruption 4.61 (r54 3.74), Discord cards 1.33 (1.87), adversary twists 0.78 (0.60).

**The two new choices, on and off** (same fresh seeds, `read-136-choices.json`, [read_choices.md](read_choices.md)),
gap off - on: Envoy off 0.0 ±1.7 / 0.0 ±1.4 / -2.1 ±1.5 (d0-d2; with it off the team holds off more Unity instead,
2.08 against 1.97 at d0); Unity-for-Discord off +2.1 ±3.5 / -4.2 ±3.8 / -3.5 ±4.1 (node losses 18 / 42 / 53 against 16
/ 37 / 49; Envoy plays rise to 2.42 / 1.77 / 1.28 to cover). Both are used every game at every level, and each covers
for the other: neither alone is worth more than a few points.

**Speed** (`bench/per_decision.py`, strategist, c2 d2, 12 games, one thread): ms a decision 1.2 (r54) to 3.8 (r55),
games 5.9 to 8.5 rounds, options a decision 15.9 to 23.4. With Envoy and the hold-off choice both off the r55 cost is
the same (4.0 against 4.0). Moving game length on each binary (strategist, d2 B+P, seeds 1000-1011, one thread, best
of 2):

| Run | rounds | options/decision | ms/decision | ms per option |
|---|---|---|---|---|
| r54 | 5.92 | 15.9 | 1.15 | 0.072 |
| r54, Unity 3 / Discord 3 (longer) | 7.50 | 20.9 | 2.60 | 0.124 |
| r55 | 8.50 | 23.4 | 3.34 | 0.143 |
| r55, Unity 1 (shorter) | 5.50 | 14.7 | 1.08 | 0.073 |
| r55, 6-card adversary deck | 5.33 | 13.5 | 0.91 | 0.067 |

At r54's length the r55 code costs the same or less. The whole rise is length, in two parts: 47% more options a
decision and twice the cost of each option, because every eval reads a bigger late-game state (more cards in play,
pieces and held resources). Kept, since longer games are the item's goal; `speed.py check --update` after this read.

## The death audit (r54's 116 last-node losses) and the agent fix

Corruption took the last node in 64, Discord cards in 36, Border War in 15, war 1, threats 0. 82 of 116 were one
blow that took every node the seat held. Nodes cluster (placement is bound to adjacency): 186 of 198 stood within 1 of
the city that corrupted, and in 36 of 64 two cities corrupted at once (a lost node's marker tips the next city). Seat
0 (Horizon) loses 92 of 116 whatever its board: Horizon #14 sits in spread range of both starting corrupted cities
(a seat swap moves the deaths with the seat). A node-safe line existed in 50 of 116 (a node placed into the coming
blast with a safe hex free, or a node power left unpaid with the energy for it). **Verdict: mostly the rules (66 of
116, the map and the corruption cascade), the agents in 50** (`eval.rs` charges `node_danger` on the raw node count
and values every held hex alike; nothing reads a filling city's blast or a coming "each player destroys N").

**The fix screened**, `w.node_exposure` (eval.rs): a node next to a filling clean city counts as that share lost,
in `node` and in the count `node_danger` reads. On / off (strategist): dev +1.3 / +4.2 / -3.5 (d0-d2, 144 games),
weight 2 -0.7 / -5.6 (d1 / d2); fresh +6.2 ±3.8 / -1.4 ±5.0 / -4.2 ±4.6, node losses 13 / 36 / 51 against 16 / 37 /
49. **Not adopted** (off by default): it cuts d0 node deaths but not d2's, where the cascade and Horizon #14 decide.
The last-node weight itself (`w.node_danger`, 60) moved both ways on the dev seeds (144 games, d1 / d2): 120 reads
67.4 / 54.2 (60: 61.8 / 55.6, node losses 41 / 59 against 49 / 58), 30 reads 62.5 / 56.9: no level beats 60 at both.
The agents already weigh a thin seat; what they miss is where the blow lands, and the rule-side causes dominate d2.

## Not done

- The rule-side death causes (Horizon #14's place, the marker cascade, adjacency-bound placement) are for Matt.
- `web/`: `panels.ts` drops the Undecided; `npm run check` and the e2e were not run (no node_modules here).

# 269 (item 270_face_followups): the four unfit faces, Hostile Buyout's price, the spread lines' level

Rules revision 126, bumped from the constant (125 to 126: 267 took 125, so the prompt's "264 left 124" was stale).
Base: main at 22ef424 (after 268, which changed no play). Play changes: Positron Storage Ring and Hostile Buyout
only; the rest is faces and card text.

## The fit

`npm run cards:fit`: 208 player cards, **0 below 7.5 pt** (exits 0). Before / after at print size (300 dpi, rendered
and fitted as cards:fit does; `out/270_face_followups/face_shots.mjs`), left the r125 face, right this one:
[shots/](shots/).

| Card | Before (r125) | pt | After | pt |
|---|---|---|---|---|
| Popular Uprising | Action: add 1 Follower here, OR pay 4 Followers, rescue it. Icons: → +1 Follower │ OR −4 Followers → [−1 cube city] station / factory | 5.4 | Same words. Icons: → +1 Follower │ OR −4 Followers → **rescue** | 7.5 |
| Retained Earnings | When you play an Industry card, add 2 Capital here; at 4, raise your credit income by 3. When played: Add 2 Capital here. | 6.3 | ... at 4, **+3 credit income**. ... | 7.5 |
| Particle Accelerator | Action: pay 2 energy: add 3 research cubes in your color to an open megaproject, OR get them. | 6.3 | Action: pay 2 energy: **3 research cubes, yours or on an open megaproject.** Icons in that order (cubes OR cubes megaproject) | 7.5 |
| Positron Storage Ring | When you play a Power card, gain 2 energy. Action: pay 2 energy, get 1 antimatter, draw 1 card. When played: Pay 2 energy. Draw 2 cards. | 5.8 | Action: pay 2 energy, get 1 antimatter, draw 1 card. When played: Pay 2 energy. Draw 2 cards. (10 → 6 credits) | 7.5 |
| Hostile Buyout | [−1 cube] station / factory | 7.5 | **[−1 cube site]** station / factory (22 → 23 credits) | 7.5 |
| Root Access, The Nuclear Option, Peacekeeping Mission, Forensic Audit, Citywide Microgrid, Power to the People (the other Rescue cards) | [−1 cube city] station / factory | 7.5 | **rescue** | 7.5 |

**Glyphs** (`web/card/icons.js`; `face.rs` folds the Rescue clause; card-grammar.md):

- **rescue**: the city silhouette under a flag on a pole: liberate it and claim a facility next to it, one glyph. It is
  drawn bare like a board piece, not in the dark adversary chip, so nothing on a Rescue card looks like a removal.
- **site**: a hex outline holding a facility block, any city, station or factory. Hostile Buyout's chip is the cube off
  a site, then the station / factory that becomes yours. A bare cube in a chip now always means a station's or
  factory's again, and Buyout shares no glyph with a Rescue (the fast-lane `icons_show_the_city` checks both).

Particle Accelerator's trials (each with both icon rows, full and shortened):

| Wording | Characters | pt |
|---|---|---|
| get 3 research cubes, OR add them to an open megaproject. | 79 | 6.9 |
| get 3 research cubes, or put them on an open megaproject. | 79 | 6.9 |
| pay 2 energy, get 3 research cubes or add them to an open megaproject. | 78 | 6.9 |
| **3 research cubes, yours or on an open megaproject.** | 72 | 7.5 |

So both halves stay and the megaproject half isn't dropped. The price stays at 20 credits (no change to the effect).

## Prices (`effect_ledger.py --stdout`, values.json; the anchor check from card-guidelines.md)

**Hostile Buyout**, 22 → 23 credits, 1 antimatter. The table gives pay 41 + the tier-2 keep 6.5 = 47.5 against get 70,
so **1.47**, under the 1.5 ceiling (1.51 at 22). Give / get: you give 23 credits and 1 antimatter (41 at the table) and
get a corrupted site freed and made yours plus 3 ambassadors (70). A player can see the gain. The smallest lever: a
credit, not an ambassador (3 → 2 would read 62 / 46.5 = 1.33 and change what the card does).

**Positron Storage Ring**, the Effect dropped, 10 → 6 credits (2 insight and 2 energy kept). Table: pay 20 + 6.5 =
26.5 against reasoned 31.75, so **1.20** (at 10 credits it would be 1.04; r125's was 41.88 / 30.5 = 1.37). Anchors,
four uses as 258 counted them: pay 6 + 4 × (3 keep + 2 play insight) + 2 energy 9 + 4 uses × 2 energy 36 = 71, against
draw 2 for 10 plus 4 × (1 antimatter 18 + a card 5) for 92 = 102, so **1.44** (was 75 / 118 = 1.57). Both are inside
the 1.0-1.5 fair band and well under tier 2's 2.5. Give / get per use: 2 energy (9) for 1 antimatter and a card (23).

Unchanged and keeping their prices: Popular Uprising (26), Retained Earnings (16) and Particle Accelerator (20). Their
effects didn't move.

## Amendment 12344: "(level 1+)" on Botnets spread

The four Militarist events' lines now read "Botnets spread (level 1+): ...". Sleeper Cells' rule is only "When a
Corrupt step checks a city, it falls at 3 agents, not 5." Botnet Swarm (a project) prints "Each country, twice
(level 1+): ..." too: its spread obeys the same `botnet_spread_at` gate, and without the mark that gate would be
printed nowhere once Sleeper Cells stopped naming it. That makes it five cards, not four.

- **Fit:** the event and project cards have no fitted face yet (frameless text in the viewer, outside `cards:fit`), so
  there is no 7.5 pt box to miss. Each line grows by 12 characters.
- **Checks:** `docs/ladder.rs` drops `botnet_spread_at` from the mid-game dials. A spread is card text, so level 1
  counts one change, not two (design_check's ladder line reads it). Its check now refuses "Botnets spread" in a level's
  rule, and refuses any spreading card whose "(level N+)" disagrees with `botnet_spread_at`.
- **Play:** no change (d0 still never spreads; same engine dial).

## Reads

There is no screen: the asked-for numbers were the only cell (the prompt names no other lever). Fresh read
`experiments/farm/f269-read.json` (judge and greedy, three pairs, d0-d4, 48 seeds a cell, seeds 49692-49739, now spent).
The new side was farmed (12 shards native, 12 wasm). The old side is r125 (main 22ef424) built from `git archive` and
run natively on the same seeds. Both are kept in `out/keep/` (see below). Read with `bench/ladder.py NEW OLD`:

| agent | d | r126 win % | r125 | gap ± SE (paired) | band |
|---|---|---|---|---|---|
| strategist | d0 | 98.6 | 98.6 | +0.0 ± 1.0 | met |
| strategist | d1 | 91.7 | 91.7 | +0.0 ± 2.2 | met |
| strategist | d2 | 84.0 | 86.8 | −2.8 ± 2.2 | met |
| strategist | d3 | 66.0 | 63.9 | +2.1 ± 2.7 | met |
| strategist | d4 | 29.9 | 35.4 | −5.6 ± 2.2 | – |
| greedy | d0 | 97.9 | 99.3 | −1.4 ± 1.0 | met |
| greedy | d1 | 87.5 | 84.0 | +3.5 ± 3.0 | met |
| greedy | d2 | 80.6 | 79.9 | +0.7 ± 2.7 | met |
| greedy | d3 | 56.2 | 60.4 | −4.2 ± 1.9 | met |
| greedy | d4 | 27.8 | 29.9 | −2.1 ± 2.5 | – |

Every band is still met. Two of the ten gaps are past 2 SE (judge d4 −5.6, greedy d3 −4.2), and the mean gap is about
−1 point. The two cards can't carry that much directly. Per game (old → new): Positron Storage Ring was offered
0.36 → 0.37, kept 0.26 → 0.27, played 0.21 → 0.20. Hostile Buyout was offered 0.51 → 0.51, kept 0.20 → 0.20, played
0.15 → 0.15. So the agents value both much as before. The gaps are mostly games that diverge once a changed price
moves a decision; the cards' own effect is small. Read d4 as a slightly weaker player deck: a Positron that stopped
paying energy.

- **`design_check.py READ --old OLD`:** 4 warnings. Three are as on main on the same seeds: Forensic Audit
  kept-not-played 0.03; Protector Bulwark 0.21; Make antimatter 0.01, against r125's 0.02. One is new:
  **victory-skips antimatter 75%**, which sits at the 75% minimum. Positron makes antimatter, so the new warning is
  plausibly this change. Gone: r125's `rounds d4 8.0`. The ladder line reads level 1 as 1 / 1 (one named rule, one
  change).
- **After the freeze**, the diff review touched `face.rs` (two wording guards; faces only). Seed 49692 was replayed on
  the final build: `farm/compare.py`, 30 games, 0 differ.
- **`loop_smoke.py --run`** (dev seeds 36800-36815): no timeouts, and at most 5 plays of one card in a game.
- **`play_read.py`** on the fresh read: no card kept in more than 1% of games and never played, for either agent.
- **Speed:** `speed.py check` gives no verdict, because the baseline is r112. `speed.py pair` r125 → r126 (engine and
  greedy, c2 and c3) reads 0.91-1.08× on 8 threads. Those games differ, so each ratio is speed times game length. No
  agent code changed, and per-game time didn't move outside noise. The 1-thread column swings from 0.82× to 1.37×
  over 2-14 games.

## New-player summary

- Rescue cards show one picture, a city under your flag: free the city, then take a station or factory next to it.
- Hostile Buyout's picture is the cube coming off any site. It costs 23 credits.
- Positron Storage Ring no longer gives energy when you play Power cards, and costs 6 credits.
- Botnets spread lines say "(level 1+)": at level 0, skip them.

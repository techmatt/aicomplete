# 325: the solo tutorial (r153, no rules change)

A scripted first game that teaches solo inside 324's player view, plus Matt's two picks of Nov 25 (amendment
#14951): a fresh browser opens the player view on `solo.json` under a new-game screen offering the tutorial first,
and printed cards show their scenes.

## What a new player sees

- **A fresh browser** (no saved view or session) opens the player view on `configs/solo.json`, with the **new-game
  screen** over it: *Tutorial: your first game* first, then *Solo game* (Board select, the Builder suggested, a
  random seed), then *Other setup* (any config, a seed). *New game…* in the header reopens it; the header's old
  Config / Seed form is now the debug view's only.
- **The tutorial**: solo.json on the Builder at **seed 24** (`TUTORIAL` in `web/src/tutorial.ts`; d1's numbers
  are solo's own). 20 prompts in `docs/teach.md`'s order and wording, each pinned to what it names (an amber outline
  that pulses), then a hand-off. Info steps wait for *Next* (and have *Back*); "do" steps wait until the view shows
  the thing done and offer *Skip step*. *Restart tutorial* (the same game afresh, step 1) and *Skip tutorial* are on
  every prompt. The step is kept across a reload. It suggests and never forces: every legal move stays open, and a
  step the player did early is passed over.
- **Scenes**: the hand, buildings, draft and hovers print each card's scene, downsized on the fly by the dev
  server from `art-originals/` (480 px JPEG, about 30 KB, 0.2 s the first time, cached in `out/scene-cache/`); no
  image is committed. 185 cards have a drawn scene. A built site (no dev server) prints frames alone, as before.

## The steps (seed 24, as `tutorial-play.mjs` met them)

| # | Step | Kind | Round | Outlines | Shot |
|---|---|---|---|---|---|
| 0 | New-game screen | | | | [00](shots/1366x768-00-new-game.jpg) |
| 1 | What you are | info | 1 | the map | [01](shots/1366x768-01-welcome.jpg) |
| 2 | How you win | info | 1 | the two corrupted cities | [02](shots/1366x768-02-win.jpg) |
| 3 | How you lose: the four clocks | info | 1 | the Alignment and clocks | [03](shots/1366x768-03-lose.jpg) |
| 4 | A round, in order | info | 1 | the round bar | [04](shots/1366x768-04-round.jpg) |
| 5 | The adversary's next card (Card beat) | info | 1 | the face-up next card | [05](shots/1366x768-05-card.jpg) |
| 6 | The Income beat, projects, research row | info | 1 | projects, research row | [06](shots/1366x768-06-income.jpg) |
| 7 | Cards: build 7 | do | 1 | the deck buttons | [07](shots/1366x768-07-build.jpg) |
| 8 | Take 4 (programs, buildings, events, tags) | do | 1 | the draft | [08](shots/1366x768-08-take.jpg) |
| 9 | Keep for insight | do | 1 | the draft, *Done keeping* | [09](shots/1366x768-09-keep.jpg) |
| 10 | Your board, insight, energy from cities | info | 1 | the board strip | [10](shots/1366x768-10-board.jpg) |
| 11 | Place a node next to a corrupted city | do | 1 | the node powers, then the question | [11](shots/1366x768-11-node.jpg) |
| 12 | Bring an ambassador in | do | 1 | *Move an ambassador* | [12](shots/1366x768-12-arrive.jpg) |
| 13 | Move in | do | 1 | *Move*, the corrupted cities | [13](shots/1366x768-13-movein.jpg) |
| 14 | Threats | info | 1 | the country's threats | [14](shots/1366x768-14-threats.jpg) |
| 15 | Done for the round | do | 1 | *Done (pass)* | [15](shots/1366x768-15-pass.jpg) |
| 16 | The adversary's turn, its replay | info | 2 | the turn panel | [16](shots/1366x768-16-replay.jpg) |
| 17 | Cards again | do | 2 | the draft | [17](shots/1366x768-17-cards2.jpg) |
| 18 | Liberate (the bill, what blocks it, buying an ambassador) | do | 2 | Liberate's line, *Gain a player ambassador* | [18](shots/1366x768-18-liberate.jpg) |
| 19 | Unity and Discord | info | 2 | the Alignment | [19](shots/1366x768-19-unity.jpg) |
| 20 | You're on your own: three tips | info | 3 | | [20](shots/1366x768-20-handoff.jpg) |

At 1920×1080: [new game](shots/1920x1080-00-new-game.jpg), [take](shots/1920x1080-08-take.jpg),
[node](shots/1920x1080-11-node.jpg), [Liberate](shots/1920x1080-18-liberate.jpg),
[hand-off](shots/1920x1080-20-handoff.jpg). After the hand-off: [1366](shots/1366x768-99-after-handoff.jpg).

## The seed

Seeds 1-30 played by `play-policy.mjs` (the scripts' small policy: a node next to a corrupted city, an ambassador
in, onto the city in round 1, Liberate in round 2) all show a node, an ambassador's move-in in round 1, a threat up
in round 1 and a city freed in round 2; none is won by that policy (it loses in rounds 7-11). Seed 24 was picked
because it lasts to the deck's end (round 11) on that line, so a learner is not lost soon after the hand-off. On
seed 24 a third city (#14) falls in round 2, so the hand-off names two cities still to take; the Liberate step
waits for one of the two starting cities to be free, not for fewer corrupted cities.

## Checks

- `web/scripts/tutorial-play.mjs` (new) passes at 1366×768 and 1920×1080. It opens a fresh browser, checks the
  view, config and new-game screen, clicks *Tutorial*, and plays to the hand-off. Moves are clicked through
  play-policy's controls; *Next* is clicked on info steps. The 20 steps were all shown, with no stuck step, in
  49 moves. No prompt ran off screen, and none covered everything it outlines. It then checks reload, Back,
  Skip step, *Restart tutorial*, *Skip tutorial* and *New game…*.
  It passes the same way on a built site, which has no dev server, so no scenes.
- The other browser lanes start as they did before (the debug view on baseline.json): `cdp.mjs`'s `openPage`
  presets that unless `fresh: true`. They all pass:
  - e2e, e2e:sources, e2e:corps and e2e:researcher;
  - playthrough on solo and on three_seat;
  - player-play, troops-play and ambassadors-play;
  - odd: 18 of 18 on a built site. `odd.mjs`'s Space check now presses *New game…*, the player view's header button.
    On the dev server its `freeze-mid-autoplay` trips on vite's websocket, a dev-server message.
- The diff review found 9 issues and 8 were fixed:
  - The tutorial's numbers are now read from the game's config: hand size, take, keep prices, starting corrupted
    cities, agents to destroy a node, and city energy.
  - Two wordings now follow teach.md: a factory makes a new agent only if empty, and tier-2 cards keep for 3.
  - The button is named as it reads: *Gain a player ambassador*.
  - Scene downsizes get a 30 s timeout, at most 3 at a time.
  - Round 2's cards step can't be skipped by a question in the adversary's turn.
  - A waiting tip can be skipped.
  - Every step but the hand-off must outline something.
  - The new-game screen no longer covers a restore error.
- Left as is: the hand-off counts from the round the tutorial passed its Liberate step, not the round of the
  Liberate itself. These differ only if a player leaves an earlier prompt open across a round.

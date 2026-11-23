# 321 bug hunt (r153): the player view for a three-seat playthrough

A hunt on recent changes since hunt 308 (r150; items 309-320), aimed by the planner at the player view as Matt will use
it: one person playing every seat of `configs/three_seat.json` at d0, 320's turn replay (◀ ▶ over a past board while
the controls stay live), and 317's twelve reworked tier-2 cards played in the viewer. Start 11:42pm, cap 6:42am;
stopped on yield at about 12:30am. [fixed.md](fixed.md) lists F446-F458, [deferred.md](deferred.md) D142-D147;
[docs/qa-findings.md](../../../docs/qa-findings.md) keeps the open list. No rules change (r153 throughout); no seed
farmed (sweeps on 100 and 1000, playthroughs on seeds 1, 5, 9, hand sessions on 3, 4, 7, 11, 21).

## Rounds (local time)

| when | lens | found |
|---|---|---|
| 11:42-11:50 | setup: release and wasm builds, a snapshot on port 52954, a scripted browser for hand play | — |
| 11:50-12:05 | round 1, recent changes as the user: three-seat games by hand at 1366×768 (turn replay, seat flow, hovers, prompts); the twelve cards in a rigged game (tier-2 deck = the twelve, needs cleared) | 7 user-facing (▶ moving, the view snapping back to seat 0, raw-JSON hovers on every control, debug internals in hovers, no hover on a project answer, unnamed paying seat, "needs tags" with no count), 2 polish (no ← →, no count on a removal question) |
| 12:05-12:22 | round 2: the sweep in the player view (c3-d0, c3-d2, c3-d3-engine; 575 captures), independent review of round 1's diff, `cards-play.mjs` on the twelve | 1 user-facing ("no target" drawn as ready) and 3 faults in the hunt's own fixes; the sweep green (only W16's `truncated`) |
| 12:22-12:26 | round 3: the draft and keep with needs, end screens | 1 user-facing (a keep's price shown without its unmet needs), 1 polish (Liberate refused beside the end banner) |
| 12:26-12:28 | round 4: e2e, odd input (18 scenarios), the turn panel and draft at 1920 and 2560 | thin: green |
| 12:28-12:30 | round 5: a clicked three-seat game at 1920×1080 (seed 9) with the new checks | thin: green |

Rounds 4 and 5 were thin in a row.

## The twelve cards in the viewer

Each was played (and each building's Action used) through the player view's own buttons, its prompts answered there:
all twelve do what their faces say. Fusion Research Campus (3 energy when built; its Action frees a corrupted station
for 3 energy), Arcology Foundry (4 Ambassadors on the target city, 3 chips; they pay its Liberate), Vertical Monopoly
(1 antimatter per 2 Industry tags, its own counted), Annihilation Lance (asks which project to discard, then 12 compute
off each), Orbital Mass Driver (1 chip frees a corrupted factory, its agents off), Homeland Security Act (4 Agents off on
or next to the target, the player picking hexes; the country cordoned), Continental Supergrid (asks which other player;
4 energy and 3 Ambassadors each), Zero-Day Auction (8 research cubes off, 1 antimatter), Foundation Model Campus (draws 2
when built; its Action's 4 sway tipped Unity), Lure Network (12 credits liberates a corrupted city, once a round),
Agency Directorate (2 Ambassadors a Security card played, its own play included: D146), Beamed Power Constellation
(shelter, 1 energy a node). Their Actions show as a Use button on the built tile with the Action's words; needs now say
which and how many in hand (F453) and in the draft and keep (F454). One card plays with nothing to do on its map half:
Arcology Foundry with no corrupted city in reach (F455's warning). `web/scripts/cards-play.mjs` reruns the whole check.

## Checker

`web/scripts/playthrough.mjs` fails on three new things (F458); `web/scripts/cards-play.mjs` (new) plays named cards
through the player view in a rigged three-seat game and writes a per-card report (questions, notes, resource and map
changes, a shot): web/README.md, *Other checks*. The engine status has a fast-lane test.

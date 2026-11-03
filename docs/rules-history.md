# Rules history

One line per rules revision, newest first: the item that made it, what changed, and its
status where that still applies. [rules.md](rules.md) is the current rules and marks every
rule **locked** or a candidate; the reasoning, reads and numbers of a revision are in its
item's write-up under `experiments/results/` and in git. Every rules change adds its line
here in the same commit (CLAUDE.md).

| Rev | Item | What changed | Status |
|---|---|---|---|
| 34 | 080 | Liberating cities: a corrupted city is liberated by a liberate card or by `board.liberate_at` ambassadors, both needing presence on or next to it; a liberated city is clean and sways Unity. Seven tier-2 liberate capstones and the `liberate` op; a card may cost research cubes; `adversary.corrupt_at` 4; troop pressure where Wall stands (`adversary.wall_troops`); Grant costs 3. ([screen-080](../experiments/results/screen-080/liberation.md)) | candidate |
| 33 | 078 | A bigger starting economy as a scaffold (no new tracks) and a faster adversary; the ambassador project at 20 or more. ([screen-078](../experiments/results/screen-078/economy.md)) | candidate |
| 32 | 076 | The card review pass: every player card fixed, repriced, merged or cut under Matt's card rulings ([card-guidelines.md](card-guidelines.md)). ([screen-076-cards](../experiments/results/screen-076-cards/cards.md)) | card iconography, board powers once a round, a cost names N of one tag or N of any: **locked**; the rest candidate |
| 31 | 075 | The Researcher board (seated at three countries); the open board rulings; the corps amended. ([screen-075-researcher](../experiments/results/screen-075-researcher/researcher.md)) | every board power once a round, powers paid by sacrificing played cards, no rule exceptions without a very good reason: **locked**; the rest candidate |
| 30 | 074 | Research cubes and the four corps; a country is a corp. ([screen-074-corps](../experiments/results/screen-074-corps/corps.md)) | country = corp, one piece one corp: **locked**; the rest candidate |
| 29 | 070 | Threat sources, typed counters, tier-2 capstones. ([gate-070](../experiments/results/gate-070/gate.md)) | candidate |
| 28 | 068 | Builder and Protector boards, ambassadors that stay, scarcity. ([screen-068-boards](../experiments/results/screen-068-boards/boards.md)) | one payment path for placements, tag costs on powers are sacrifices: **locked**; the rest candidate |
| 27 | 066 | Presence powers replace growth. ([screen-066-presence](../experiments/results/screen-066-presence/presence.md)) | candidate |
| 26 | 064 | d2 re-centered for the four-deal planner; American spelling everywhere, ids included. ([screen-064-deals](../experiments/results/screen-064-deals/deals.md)) | the spelling: **locked**; the rest candidate |
| 25 | 059 | The card design pass: every card exception-free (no `*`, cap, footnote or sum on a face). ([screen-059-cards](../experiments/results/screen-059-cards/design.md)) | candidate |
| 24 | 055 | The first price pass, cards only. ([screen-055-prices](../experiments/results/screen-055-prices/prices.md)) | candidate |
| 23 | 053 | Calibrated at d2; pressure per country. ([screen-053-calibration](../experiments/results/screen-053-calibration/calibration.md)) | candidate |
| 22 | 046 | One standard placement, a Science niche, a better Develop. ([screen-046-niche](../experiments/results/screen-046-niche/niche.md)) | the round cap's quiet backstop: **locked**; the rest candidate |
| 21 | 044 | Bug hunt: rules that didn't run, cards offered for nothing; fixes only. ([qa-findings.md](qa-findings.md)) | fixes |
| 20 | 041 | Growth as a choice, the marker worth clearing, Science on the board. ([screen-041-growth](../experiments/results/screen-041-growth/growth.md)) | that a round cap exists and can shorten: **locked**; the rest candidate |
| 19 | 034 | Three distinct loss pressures, Spirit Island presence, earlier Science and Industry. ([screen-034-clocks](../experiments/results/screen-034-clocks/clocks.md)) | the five end conditions, distinct loss clocks, a destroyed presence leaves the game: **locked**; the rest candidate |
| 18 | 033 | Ongoing abilities only on buildings; card text generated from ops. | both **locked** |
| 17 | 028 | Presence loss that bites; a Unity deck that can run out. ([screen-028-rollout](../experiments/results/screen-028-rollout/rollout.md)) | candidate |
| 16 | 027 | Steady pressure; the two decks as the clocks; presence loss. ([screen-027-endgame](../experiments/results/screen-027-endgame/endgame.md)) | the Discord deck never reshuffles, no presence means everyone loses: **locked** |
| 15 | 024 | A scarier adversary, a purge that can't come early, credits that matter. ([screen-024-adversary](../experiments/results/screen-024-adversary/adversary.md)) | candidate |
| 14 | 023 | The player card set, about 150 cards on 021's primitives and 022's costs. ([screen-023-cards](../experiments/results/screen-023-cards/cards.md)) | candidate |
| 13 | 022 | Insight costs on cards, shared between keeping and playing. ([screen-022-insight](../experiments/results/screen-022-insight/insight.md)) | candidate |
| 12 | 021 | Card primitives and the resource model; AI-board powers paid by sacrificing tags. ([screen-021-cards](../experiments/results/screen-021-cards/cards.md)) | sacrifice payment, no hand cap: **locked**; the rest candidate |
| 11 | 020 | Five bug fixes from the bug hunt. | fixes |
| 10 | 019 | Adversary troops march toward player infrastructure; nodes are hard to remove. ([screen-019-troops](../experiments/results/screen-019-troops/troops.md)) | candidate |
| 9 | 018 | Structural fixes and the calibration. ([screen-018-calibration](../experiments/results/screen-018-calibration/calibration.md)) | candidate |
| 8 | 017 | The adversary project deck; one-shot Alignment cards and more Unity sources. ([screen-017-projects](../experiments/results/screen-017-projects/projects.md)) | Alignment cards one-shot and hidden, empty-pool rule B, ambassadors on producing bases sway Unity: **locked** |
| 7 | 016 | Player presence in place of data centers; factories, chips, the production/war clock, the purge eased by Unity. ([screen-016-presence](../experiments/results/screen-016-presence/presence.md)) | presence placed with energy next to your presence, adversary factories make troops: **locked** |
| 6 | 014 | The board model: hexes, the adversary ladder. ([screen-014-board](../experiments/results/screen-014-board/board.md)) | superseded where later rows say |
| 1-5 | v0-013 | The v0 draft, its tuning (007, 009, 011) and Alignment in place of opposition (013); the board they ran on is gone and their records do not replay. ([experiments/history.md](../experiments/history.md)) | history |

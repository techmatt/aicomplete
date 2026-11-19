# 276: the traced games, player side, r130 against r131

`python trace_seats.py BEFORE AFTER --before-content BASE/content/ais.json [--level 4]` on `aic-sim run --trace` of
seeds 268000-268013 (268's, 274's and 275's traced seeds), the judge, Builder and Protector, d2 and d4. Before: r130
built from 8d5893b (`git archive` into `out/276_board_rework/base`) with 276's stats-only `ai.offered` count patched
in; after: r131 with the same count. Rates are 268's: 15 s a decision, 3 s a component, 10 s a check; each board
use a seat could make as its action phase began ("weighed", a group counted once) is a 10 s check, the yes/no read
a player gives it.

### d2: the traced games (14 before, 14 after), a seat-round

| Seat | Side | Decisions | Action turns | Powers used | Power steps | Menu (options a turn) | Weighed (board uses offered) | Components | Checks | Minutes |
|---|---|---|---|---|---|---|---|---|---|---|
| Builder | before | 30.4 | 10.2 | 2.4 | 2.2 | 33.3 | 4.8 | 33.2 | 0.4 | 10.1 |
| Builder | after | 30.6 | 9.7 | 1.4 | 2.3 | 34.3 | 2.1 | 34.4 | 0.1 | 9.7 |
| Builder | **moved** | +0.2 | -0.5 | -1.0 | +0.1 | +1.0 | -2.8 | +1.2 | -0.3 | **-0.4** |
| Protector | before | 27.0 | 8.4 | 1.8 | 2.7 | 29.2 | 3.5 | 28.3 | 0.7 | 8.8 |
| Protector | after | 28.7 | 8.8 | 1.6 | 3.2 | 28.5 | 2.0 | 30.9 | 0.6 | 9.1 |
| Protector | **moved** | +1.7 | +0.4 | -0.2 | +0.6 | -0.8 | -1.5 | +2.7 | -0.1 | **+0.3** |

Rounds a game: 8.86 before, 8.86 after.
Both seats: -0.1 minutes a round; 275's 52.7 becomes 52.6 by these rates.

### d4: the traced games (14 before, 14 after), a seat-round

| Seat | Side | Decisions | Action turns | Powers used | Power steps | Menu (options a turn) | Weighed (board uses offered) | Components | Checks | Minutes |
|---|---|---|---|---|---|---|---|---|---|---|
| Builder | before | 27.6 | 8.7 | 2.7 | 2.6 | 30.2 | 4.6 | 36.7 | 0.5 | 9.6 |
| Builder | after | 26.2 | 7.2 | 1.4 | 2.4 | 31.0 | 1.6 | 34.9 | 0.2 | 8.6 |
| Builder | **moved** | -1.4 | -1.4 | -1.3 | -0.2 | +0.8 | -3.0 | -1.8 | -0.3 | **-1.0** |
| Protector | before | 23.9 | 7.0 | 1.7 | 2.8 | 27.6 | 3.2 | 25.8 | 0.5 | 7.9 |
| Protector | after | 23.5 | 6.6 | 1.5 | 3.3 | 26.9 | 1.8 | 24.3 | 0.5 | 7.5 |
| Protector | **moved** | -0.4 | -0.4 | -0.3 | +0.5 | -0.6 | -1.4 | -1.5 | +0.0 | **-0.4** |

Rounds a game: 6.50 before, 5.93 after.
Both seats: -1.4 minutes a round.


**Reading it.** What a seat weighs a round falls by about half (d2: Builder 4.8 → 2.1, Protector 3.5 → 2.0 offered
uses; printed, 10 / 8 / 9 powers become 4 / 3 / 3 groups), and the uses taken fall by 0.2-1.0 a round. But the
judge never took more than about 2 board powers a round, so the decisions *taken* (268's count) don't move, and the
two-node options and Accord's ambassador add components. Net at d2: **-0.1 minutes a round** (275's 52.7 → 52.6);
at d4 -1.4. The patched counter replays r130's games but one: the before side's Protector reads 27.0 decisions here
against 27.2 without it (one seed's late round), within the noise of these 14 games.

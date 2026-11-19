# 278: the dev-seed screen (indicative)

`s278-screen.json`: r132, the judge (Builder and Protector), dev seeds 36800-36831 (32 a cell), each flag on
against base, paired by seed (`python screen.py games.jsonl`). d4's base wins 3%: at the floor, it can't show a
cost. `top3` is no-pass, any tag and hosts fixed; `hostfix` and `top3` need `host_fixed.patch`.

| Flag | Level | Win % (flag) | Win % (base) | Gap ± SE (points) | Rounds (flag) | Rounds (base) |
|---|---|---|---|---|---|---|
| jam | d2 | 34 | 47 | -12 ± 7 | 9.19 | 9.16 |
| jam | d4 | 3 | 3 | +0 ± 0 | 6.44 | 6.44 |
| nopass | d2 | 53 | 47 | +6 ± 11 | 9.38 | 9.16 |
| nopass | d4 | 0 | 3 | -3 ± 3 | 6.41 | 6.44 |
| anytag | d2 | 34 | 47 | -12 ± 11 | 8.88 | 9.16 |
| anytag | d4 | 3 | 3 | +0 ± 4 | 6.00 | 6.44 |
| hostfix | d2 | 31 | 47 | -16 ± 9 | 9.38 | 9.16 |
| hostfix | d4 | 0 | 3 | -3 ± 3 | 6.00 | 6.44 |
| stdoff | d2 | 62 | 47 | +16 ± 8 | 8.97 | 9.16 |
| stdoff | d4 | 3 | 3 | +0 ± 4 | 6.41 | 6.44 |
| top3 | d2 | 44 | 47 | -3 ± 11 | 9.22 | 9.16 |
| top3 | d4 | 0 | 3 | -3 ± 3 | 5.94 | 6.44 |

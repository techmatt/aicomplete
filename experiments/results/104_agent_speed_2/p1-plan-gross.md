# 104 P1: `plan_gross=1` made the default (r40)

Matt (2026-11-04): the strategist's keep plan takes 099's form, `plan_gross=1` (a card worth no
more than its price is still dropped, one that is planned adds its gross value); the net form
stays a switch, `plan_gross=0`. The default is shared by every agent with a keep plan.

**Confirming read**, fresh r40 seeds 33300-33499, the three pairs, 200 seeds each
(`experiments/screen-104-p1.json`, `screen-104-p1-d0.json`; `bench/paired.py`):

| agent | d | bp | br | pr | pooled (600 seeds) |
|---|---|---|---|---|---|
| strategist, gross minus net | 2 | +1.5 ±2.7 | -4.5 ±3.5 | +2.5 ±3.6 | -0.2 ±1.9 |
| greedy, gross minus net | 0 | +1.5 ±2.5 | -0.5 ±2.9 | +0.0 ±2.6 | +0.3 ±1.5 |

Win rates, strategist d2 gross / net: bp 27.0 / 25.5, br 39.0 / 43.5, pr 31.5 / 29.0.
Cards kept in over 1% of games and never played (`play_read.py`), strategist: 8 gross, 7 net;
greedy d0: 36 gross, 40 net.

102's +5.7 ±1.9 at d2 (seeds 32500-32699) did not reproduce: a null, not a loss. Matt kept the
default on it (his answer to 104's question 1).

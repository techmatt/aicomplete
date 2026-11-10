## A: local, 6 threads

wall 342.4 s (9:41:24pm to 9:47:06pm), 32 seeds, 384 games; game time summed 1969 s = 328 s a thread; **0.97 seeds a minute a core** (per-game ms, all games); 5.61 seeds a minute on the box (wall)

## B

wall 565.3 s (9:53:28pm to 10:02:53pm); engines ['native', 'wasm'], mixed True; gate failures 0; references differ False

| shard | leased to (s after start) | won by | engine | ms | seeds/min/core | in at |
|---|---|---|---|---|---|---|
| 40072 | main +10 | main | native | 392938 | 0.61 | +404 |
| 40076 | main +10 | main | native | 457110 | 0.53 | +469 |
| 40080 | main +10 | main | native | 486019 | 0.49 | +497 |
| 40084 | main +10 | main | native | 555401 | 0.43 | ? |
| 40088 | main +10 | main | native | 347241 | 0.69 | +357 |
| 40092 | main +10 | main | native | 405317 | 0.59 | +416 |
| 40096 | b271746cba5 +10, main +357 | b271746cba5 | wasm | 391681 | 0.61 | +402 |
| 40100 | b271746cba5 +10, main +403 | b271746cba5 | wasm | 510679 | 0.47 | +521 |

- main (main box, native): 6 shards, 0.56 seeds a minute a core (mean of its shards)
- b271746cba5 (Chrome on Win32, wasm): 2 shards, 0.54 seeds a minute a core (mean of its shards)
- re-runs: 2
  - 21:59:25 rerun shard 6 seeds 40096+4 (job ends 10:02:53pm)
  - 22:00:10 rerun shard 7 seeds 40100+4 (job ends 10:02:53pm)

## C

wall 424.3 s (10:06:38pm to 10:13:43pm); engines ['native'], mixed False; gate failures 0; references differ False

| shard | leased to (s after start) | won by | engine | ms | seeds/min/core | in at |
|---|---|---|---|---|---|---|
| 40072 | main +3 | main | native | 199519 | 1.20 | +204 |
| 40076 | main +3 | main | native | 226359 | 1.06 | +230 |
| 40080 | main +3 | main | native | 236083 | 1.02 | +240 |
| 40084 | main +3 | main | native | 294247 | 0.82 | +298 |
| 40088 | main +3 | main | native | 179094 | 1.34 | +184 |
| 40092 | main +3 | main | native | 207557 | 1.16 | +212 |
| 40096 | bfbde0748e3 +4, main +182 | main | native | 187317 | 1.28 | +371 |
| 40100 | bfbde0748e3 +4, main +203 | main | native | 220959 | 1.09 | ? |

- main (main box, native): 8 shards, 1.12 seeds a minute a core (mean of its shards)
- re-runs: 2
  - 22:09:41 rerun shard 6 seeds 40096+4 (job ends 10:13:43pm)
  - 22:10:01 rerun shard 7 seeds 40100+4 (job ends 10:13:43pm)
- tab killed at 10:08:37pm (+118 s), 0 of 8 shards in: SUCCESS: The process with PID 33248 (child process of PID 7552) has been terminated.
SUCCESS: The process with PID 40528 (child process of PID 7552) has been terminated.
SUCCESS: The process with PID 41588 (child process of PID 7552) has been terminated.
SUCCESS: The process with PID 35036 (child process of PID 7552) has been terminated.
SUCCESS: The process with PID 14016 (child process of PID 7552) has been terminated.
SUCCESS: The process with PID 39488 (child process of PID 7552) has been terminated.
SUCCESS: The process with PID 24864 (child process of PID 7552) has been terminated.
SUCCESS: The process with PID 11024 (child process of PID 7552) has been terminated.
SUCCESS: The process with PID 7552 (child process of PID 31232) has been terminated.
  - shard 40096: held by ['bfbde0748e3'], re-leased to main 64.1 s after the kill
  - shard 40100: held by ['bfbde0748e3'], re-leased to main 84.5 s after the kill

## B2

wall 410.3 s (10:17:13pm to 10:24:03pm); engines ['native'], mixed False; gate failures 0; references differ False

| shard | leased to (s after start) | won by | engine | ms | seeds/min/core | in at |
|---|---|---|---|---|---|---|
| 40072 | main +4 | main | native | 186525 | 1.29 | +191 |
| 40076 | main +4 | main | native | 212511 | 1.13 | +217 |
| 40080 | main +4 | main | native | 221148 | 1.09 | +226 |
| 40084 | main +4 | main | native | 279898 | 0.86 | +284 |
| 40088 | main +4 | main | native | 168079 | 1.43 | +173 |
| 40092 | main +4 | main | native | 195486 | 1.23 | +199 |
| 40096 | main +172 | main | native | 185472 | 1.29 | +358 |
| 40100 | main +190 | main | native | 219875 | 1.09 | ? |

- main (main box, native): 8 shards, 1.17 seeds a minute a core (mean of its shards)
- re-runs: 0

## B3

wall 422.0 s (10:24:54pm to 10:31:56pm); engines ['native'], mixed False; gate failures 0; references differ False

| shard | leased to (s after start) | won by | engine | ms | seeds/min/core | in at |
|---|---|---|---|---|---|---|
| 40072 | main +4 | main | native | 202868 | 1.18 | +207 |
| 40076 | main +4 | main | native | 227691 | 1.05 | +232 |
| 40080 | main +4 | main | native | 235381 | 1.02 | +240 |
| 40084 | main +4 | main | native | 293246 | 0.82 | +298 |
| 40088 | main +4 | main | native | 182857 | 1.31 | +187 |
| 40092 | main +4 | main | native | 210700 | 1.14 | +216 |
| 40096 | main +186 | main | native | 184048 | 1.30 | +372 |
| 40100 | main +207 | main | native | 215196 | 1.12 | ? |

- main (main box, native): 8 shards, 1.12 seeds a minute a core (mean of its shards)
- re-runs: 0

## B4

wall 283.0 s (10:41:50pm to 10:46:33pm); engines ['native', 'wasm'], mixed True; gate failures 0; references differ False

| shard | leased to (s after start) | won by | engine | ms | seeds/min/core | in at |
|---|---|---|---|---|---|---|
| 40072 | main +4 | main | native | 194590 | 1.23 | +200 |
| 40076 | main +4 | main | native | 218436 | 1.10 | +224 |
| 40080 | main +4 | main | native | 226692 | 1.06 | +232 |
| 40084 | main +4 | main | native | 279157 | 0.86 | ? |
| 40088 | main +4 | main | native | 175036 | 1.37 | +180 |
| 40092 | main +4 | main | native | 202393 | 1.19 | +208 |
| 40096 | b271746cba5 +4, main +179 | b271746cba5 | wasm | 179162 | 1.34 | +184 |
| 40100 | b271746cba5 +4, main +183 | b271746cba5 | wasm | 216611 | 1.11 | +222 |

- main (main box, native): 6 shards, 1.13 seeds a minute a core (mean of its shards)
- b271746cba5 (WOrk laptop, wasm): 2 shards, 1.22 seeds a minute a core (mean of its shards)
- re-runs: 2
  - 22:44:48 rerun shard 6 seeds 40096+4 (job ends 10:46:33pm)
  - 22:44:53 rerun shard 7 seeds 40100+4 (job ends 10:46:33pm)

## B5

wall 977.0 s (10:48:07pm to 11:04:24pm); engines ['native', 'wasm'], mixed True; gate failures 0; references differ False

| shard | leased to (s after start) | won by | engine | ms | seeds/min/core | in at |
|---|---|---|---|---|---|---|
| 40072 | main +3 | main | native | 208398 | 1.15 | +213 |
| 40076 | main +3 | main | native | 237068 | 1.01 | +241 |
| 40080 | main +3 | main | native | 252018 | 0.95 | +257 |
| 40084 | main +3 | main | native | 336614 | 0.71 | +340 |
| 40088 | main +3 | main | native | 187721 | 1.28 | +193 |
| 40092 | main +3 | main | native | 213845 | 1.12 | +219 |
| 40096 | b271746cba5 +45, main +212 | main | native | 255641 | 0.94 | +469 |
| 40100 | b271746cba5 +45, main +217 | main | native | 315672 | 0.76 | +534 |
| 40104 | b271746cba5 +45, main +241 | main | native | 231353 | 1.04 | +473 |
| 40108 | b271746cba5 +45, main +256 | b271746cba5 | wasm | 297639 | 0.81 | +344 |
| 40112 | b271746cba5 +45, main +340 | main | native | 343396 | 0.70 | +685 |
| 40116 | b271746cba5 +45, main +343 | main | native | 342854 | 0.70 | +687 |
| 40120 | b271746cba5 +45, main +468 | main | native | 509027 | 0.47 | ? |
| 40124 | main +191 | main | native | 306597 | 0.78 | +499 |

- main (main box, native): 13 shards, 0.89 seeds a minute a core (mean of its shards)
- b271746cba5 (WOrk laptop, wasm): 1 shards, 0.81 seeds a minute a core (mean of its shards)
- re-runs: 7
  - 22:51:39 rerun shard 6 seeds 40096+4 (job ends 11:04:24pm)
  - 22:51:45 rerun shard 7 seeds 40100+4 (job ends 11:04:24pm)
  - 22:52:08 rerun shard 8 seeds 40104+4 (job ends 11:04:24pm)
  - 22:52:23 rerun shard 9 seeds 40108+4 (job ends 11:04:24pm)
  - 22:53:47 rerun shard 10 seeds 40112+4 (job ends 11:04:24pm)
  - 22:53:50 rerun shard 11 seeds 40116+4 (job ends 11:04:24pm)
  - 22:55:55 rerun shard 12 seeds 40120+4 (job ends 11:04:24pm)


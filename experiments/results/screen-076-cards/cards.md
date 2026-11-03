# 076: the card review pass (r32)

Every number is a candidate. Matt's card rulings (2026-11-02, 1-12 with amendment #3207) are in
[docs/card-guidelines.md](../../../docs/card-guidelines.md), merged with the existing rules; the rates and the new
cost curve are in [docs/card-pricing.md](../../../docs/card-pricing.md). Planner, d2, ±1 SE. Screens ran on seeds
29000-29300, the band on 29400 and 29500, the gate on 29600.

## The band

Planner, d2, 96 a cell each (printed credits Builder 16, Protector 14, Researcher 12). **Final build** (with the
per-tag feasibility fix that the full lane found after the gate, below):

| Cell | 29400 | 29500 | **Pooled, 192** | Win mix (purge, Unity deck) | Losses (Discord deck, presence, round cap) |
|---|---|---|---|---|---|
| c2 | 69.8 ±4.7 | 67.7 ±4.8 | **68.8 ±3.3** | 78, 54 | 44, 13, 3 |
| c3 | 70.8 ±4.6 | 75.0 ±4.4 | **72.9 ±3.2** | 68, 72 | 16, 32, 4 |

The frozen build, before the fix, on the same seeds: 70.8 ±4.6 / 66.7 ±4.8 and 67.7 ±4.8 / 72.9 ±4.5, pooled
**68.8 ±3.3 / 70.3 ±3.3** (`bench/same_results.py`: the games differ, 988 and 1,031 rows, as a changed legal set does).

**Both sizes in band (60-75)** on the first freeze, after four screens (about 1 h 10 min of them). r31's band was
66.7 / 79.2; the c3 overshoot 075 reported is gone, and c3's win mix moved toward the Unity deck (72 of 140 wins, r31
38 of 152), consistent with the set's bigger Sway (Planetary Referendum 6, Coalition Charter 5, Peace Dividend 2).
c3 loses more presence-out games (32 of 192, r31 12): read it before a rule that touches presence.

## The screens (printed credits; the card prices as built)

| Read | Seeds | Builder / Protector / Researcher | c2 | c3 |
|---|---|---|---|---|
| s1 | 29000, 32 | 13 / 12 / 8 (r31) | 46.9 ±8.8 | 34.4 ±8.4 |
| s1 | 29000, 32 | 16 / 15 / 11 | 84.4 ±6.4 | 53.1 ±8.8 |
| s1 | 29000, 32 | 19 / 18 / 14 | 84.4 ±6.4 | 90.6 ±5.2 |
| s2 | 29100, 48 | 14 / 13 / - | 50.0 ±7.2 | |
| s2 | 29100, 48 | 15 / 14 / 10, 13, 16 | 56.2 ±7.2 | 62.5, 62.5, 91.7 |
| s2 | 29100, 48 | 16 / 15 / - | 66.7 ±6.8 | |
| s2 | 29100, 48 | 17 / 16 / 12 | | 70.8 ±6.6 |
| s3 | 29200, 48 | 16 / 15 / 13 | 77.1 ±6.1 | 79.2 ±5.9 |
| s3 | 29200, 48 | 17 / 16 / 14 | 75.0 ±6.2 | 81.2 ±5.6 |
| s4 | 29300, 48 | **16 / 14 / 12 (frozen)** | 70.8 ±6.6 | 83.3 ±5.4 |
| r31 as shipped | 29000, 24 | 13 / 12 / 8, the r31 cards | 54.2 ±10.2 | 75.0 ±8.8 |

- **The new set at r31's income reads 47 / 34:** the dearer cards cost more than their bigger effects pay back at the
  planner's horizon, so the printed income rises to pay for them (ruling 4's "raise printed credit income to match").
  +3 a board on top moved c2 37 points and c3 19; +6 took c3 to 91.
- **c3 is the noisier size:** one income read 62.5 on 29100 and 79-83 on 29200-29300. The frozen raise is +3, +2 and
  +4 (Builder 16, Protector 14, Researcher 12), the reads' middle; c2 rests on Builder and Protector alone.
- **Credits left at the last action phase** rose with income (10 at r31's income on the new set, 17-19 at the
  frozen): winners end high because a purge ends the game mid-round (075's reading), not a pile at upkeep.

## An agent probe, not a lever (MESSAGE #3307)

Many Action and token buildings are kept and never played (below). `planner:w.ability=2` (a building's Action valued at
2 a round left, not 0.5) on the frozen income, s4, 48 seeds: **60.4 ±7.1 / 70.8 ±6.6 against the planner's 70.8 / 83.3**,
paired on the same seeds. Valuing the Actions higher plays them and loses: the planner is not missing a weight it
should have. The r31 cards on the same seeds left 32 cards kept and never played, the same kinds (Deep State Ledger,
Kill Team Roster, Liaison Corps, Commons Grid, Intrusion Lab, Sovereign Capital Fund): the blind spot predates 076.

## Matt's reviewed cards, as shipped

| Card | His take (or the planner's agreed fix) | Shipped |
|---|---|---|
| Blackout Script | remove 5 compute from a project, about 4 | as his: 5 compute, 4 credits, 0 insight (Flash Exploit, the same 5, merged in) |
| Peace Dividend | Sway 2 toward Unity, no credits, about 5 | as his: Sway 2, 5 credits |
| Kinetic Interdiction | a printed troop number in one country, tier-2 price | remove 6 troops in one country; 16 credits, 1 antimatter, 2 insight, needs 2 Security |
| Agency Directorate | the same | progressive: add 2 Agents here, OR pay 2 to remove 3 troops in one country; arrives with 2; 14, needs 2 Security, over Security |
| Surplus Auction | pay 3 energy, gain 5 chips, a real credit cost | as his: 2 credits (the analyst's 8 chips was over his number) |
| Agency Field Station | #3207: "add 1 Agent here, OR gain 1 credit per Agent here" | as his, 4 credits (the analyst's cash-out was set back to his shape) |
| Threat Hunt and the typed counters | beat the plain route: a big jam and a tag rider, not a counter | one-off jam 5 and a rider of the tag (Threat Hunt: 4 compute); the four Action ones a free jam 3 a round; Rapid Response and Incident Response merged in |
| Shelter Order | a real price, or cut | 3 credits (the other AI-unique cards kept, as the prompt says) |

075 left "1 Science each" as a candidate for Breakthrough Lab and Policy Institute; ruling 10 (chains) took them the
other way (needs 4 and 3, priced as chain cards). In 768 band games they were kept 3 and 27 times and never played.

## What the engine gained (076)

- `Ability.or`: an "A, OR B" Action, one use a round, the player picks a half it can use (ruling 12).
- `here` on `add_tokens`, `gain_per_tokens` and `cash_out`: this card's tokens only; `gain_per_tokens` without
  `here` is refused, and `cash_out` from a program takes one card's pile (ruling 12). A building's play effects may
  load it ("Add 2 Agents here").
- `gain_per_tag` (ruling 11), `remove_troops` and a number on `remove_ambassadors_country` (ruling 6).
- Faces: "on your cards" for every per-tag count (ruling 5); an OR Action prints a line a half, the second opening OR.

## Play

`python scripts/cards/play_read.py` over every planner game of the frozen build (the band, the gate and its closes, the probe: 804 games); every agent of the gate in [play.md](play.md). A card counts when kept in more than 1% of games and played in none.

Kept in more than 1% of games and never played: 25

| card | tier | kind | games kept, % |
|---|---|---|---|
| Intrusion Lab | 1 | building | 55.2 |
| Demand Response Grid | 1 | building | 54.9 |
| Inference Cluster | 1 | building | 50.9 |
| Commons Grid | 1 | building | 50.0 |
| Honeypot Relay | 1 | building | 49.6 |
| Liaison Corps | 1 | building | 48.1 |
| Kill Team Roster | 1 | building | 47.4 |
| Hardpoint Lockdown | 1 | event | 46.5 |
| Agency Directorate | 2 | building | 46.1 |
| Open Assembly | 1 | building | 44.8 |
| Preprint Exchange | 1 | program | 41.2 |
| Simulation Cluster | 1 | building | 39.9 |
| Cryo-Compute Burn | 1 | event | 37.8 |
| Orbital Observatory Array | 2 | building | 36.2 |
| Credential Harvest | 1 | event | 34.5 |
| Sovereign Capital Fund | 2 | building | 30.0 |
| Deep State Ledger | 2 | building | 29.0 |
| Arc Furnace Complex | 2 | building | 24.1 |
| Quantum Decryptor | 1 | building | 20.0 |
| Planetary Assembly | 2 | building | 6.5 |
| Endowed Research Chair | 1 | program | 6.1 |
| Synthetic Data Forge | 1 | building | 4.2 |
| Policy Institute | 2 | building | 3.0 |
| Positron Storage Ring | 2 | building | 2.4 |
| Recursive Self-Improvement | 2 | program | 1.6 |

Never kept: 7: Capital Injection, Data Licensing Deal, Dataset Scraper, Exabyte Archive, Foundation Model Campus, Manifesto Leak, Vendor Financing

- **Most are Action and token buildings,** as at r31 (32 on r31's cards, 24 seeds of 29000: Deep State Ledger, Kill
  Team Roster, Liaison Corps, Commons Grid, Intrusion Lab, Sovereign Capital Fund among them). Their Actions work
  (`cards.used.*`: Point Defense 498 uses in the band, Agency Field Station's OR 22); the planner rarely builds them.
  Two eval probes (diagnostics, not levers, MESSAGE #3307) rule out the obvious weights: `w.ability=2` plays more of
  them and loses (above), and `w.credit_cap=24` (the eval values credits past 12 at 0.15, a cap from when seats made
  12 a round) moved neither play nor wins (seeds 29800, 48 a cell: 68.8 / 72.9 against the planner's 72.9 / 70.8).
- **Never kept: the feeders and the Dataset cash-outs** (Capital Injection, Vendor Financing, Dataset Scraper,
  Manifesto Leak, Data Licensing Deal) need a token card in play, which the planner seldom builds; and the
  needs-3-to-6 Science capstones (Exabyte Archive, Foundation Model Campus), whose gates a seat reaches late if at
  all (075's dear powers read the same).
- **Events that pay in energy or insight are kept and never played** (Cryo-Compute Burn, Credential Harvest,
  Hardpoint Lockdown): energy past presence and insight past the round buy the planner little (075's measured energy).
- **Pre-existing, not 076's:** the four typed-jam Actions (Garrison Command, Market Watchdog, Civic Watch, Security
  Ops Floor) were built 12 times in the band and used 0; on r31's cards 25 times and 0. Worth a look in `threats::counterable`.

## The gate

**On the frozen build, not replayed on the final one** (MESSAGE #3413: the band replay is enough; simulation is a
ballpark check until Matt names a milestone). The per-tag feasibility fix landed after it. Of a replay of the nearest
rows, only c2's finished (`experiments/gate-076-replay/`, 96 seeds, final build): planner 74.0, `never_convert` +8.3
±5.7, `only_tag=5` +10.4 ±6.0, `free_build_rush` +6.2 ±5.5, all behind; c3's was stopped at 150 of 672 games, unread.

Planner, d2, seeds 29600: `experiments/gate.json` (gate-075's 62 rows, c2 and c3) and `experiments/gate-075-c3.json`
(the Researcher's two), early-stopped from 12 seeds at 2 SE, capped at 48; then every row within 2 SE closed to 96
paired seeds (`experiments/gate-close-076/`, `gate-close-076-c3/`). Gap is the planner's win rate less the check's:
negative means the check is ahead.

| Row (96 paired seeds) | Gap | Gap / SE |
|---|---|---|
| **c3 `always_buy_cubes`** | **-6.2 ±5.5** | **-1.1** |
| c3 `tier2_rush` | -4.2 ±6.1 | -0.7 |
| c3 `never_second_presence`, `no_powers` | +2.1 ±5.3, +3.1 ±5.4 | +0.4, +0.6 |
| c2 / c3 `never_convert` | +4.2 ±5.9 / +4.2 ±6.1 | +0.7 / +0.7 |
| c3 `capstone_rush`, `home_only` | +5.2 ±5.6, +7.3 ±5.2 | +0.9, +1.4 |
| c2 `free_build_rush`, `only_tag=5`, `never_sacrifice` | +7.3 ±6.0, +7.3 ±6.5, +8.3 ±5.7 | +1.2, +1.1, +1.5 |
| c3 `always_max_presence`, `free_build_rush` | +9.4 ±6.1, +9.4 ±5.8 | +1.5, +1.6 |
| c2 `never_remove`, `home_only` | +10.4 ±5.6, +11.5 ±5.7 | +1.8, +2.0 |
| c3 `never_sacrifice` | +15.6 ±6.2 | +2.5 |

Every other row stopped early, behind (c2 `always_max_presence` +22 ±11 at 32, `no_powers` +29 ±11 at 24; c3
`hoard_insight=3` +50 ±15 at 12). The planner on the gate's seeds (96): 71.9 ±4.6 / 71.9 ±4.6.

**One check fails: c3 `always_buy_cubes` is ahead by 1.1 SE.** 48 more seeds (29696, information only) read it 79.2
against 62.5: **9.7 ±4.9 ahead on 144, 2.0 SE.** The policy buys 3.6 cubes a game with the Researcher's Grant (4 credits
a cube); the planner buys 0.0-0.1 (075: 0.00, when the check closed +2.1 behind). The structural cause, as measured:
076 raised the Researcher's printed credits 8 → 12 for the band, so a Researcher holds credits a Grant turns into
completion payouts (about 9 a cube), and the planner, which never values a bought cube over 4 credits, leaves them.
No lever: the gap was inside 2 SE at 96 seeds, Grant is a board power (out of this item's reach), and Researcher
income was the band's lever. The question went to Matt (aicomplete__076_card_review_pass__1); no answer came in 10
minutes, so its default stands: frozen as is, reported. **Gate-076 time:** first pass 66 min, the c3 checks 9 min, the
closes 62 min: 2 h 17 min on 8 threads, against gate-075's 3 h 18 min and `max_hours` (none in the spec; the
gate ran every check, so none dropped).

## Every card's verdict

Ruling 9's analysis, card by card: what it does, its full cost (credits, insight, the keep), the plain route, when it
matters. Prices are credits, `+` the other costs, `i` the insight to play. 152 cards in the decks (168 at r31): 16
merged or cut. The per-tag analysts' full rows (step-by-step analysis and the rate arithmetic for every card) were
the working papers; the "why" below is each card's one line.

| Card | Tier | r31 | r32 | Verdict | Why |
|---|---|---|---|---|---|
| Breakthrough Leak | 1 | 1, i1, needs 2 science | 6, i1, needs 3 science | fix | Draw 3 for 1 credit behind a 2-Science gate is cheap and small; it becomes Science's first draw chain card: needs 3, draw 4. |
| Citation Engine | 1 | 2, i1, needs 1 science | 9, i1, needs 3 science | fix | Per 2 Science tags behind a 1-Science gate at 2 credits; it becomes the chain payoff (rulings 10, 11): 1 credit per Science tag on your cards, needs 3, at 9, the shape every tag's per-tag income card shares. |
| Cryo-Compute Burn | 1 | 0 +2en, i0 | 1 +2en, i0 | fix | Pay 2 energy (4) for 2 insight (5) loses to the keep it costs (ruling 7); it becomes a one-off per-tag payoff (ruling 11). |
| Dataset Scraper | 1 | 0, i0 | 5, i1 | reprice | At 0 credits for 3 Datasets this was a free feeder; with no Dataset card gaining tokens by itself (ruling 12), feeders carry the token cards, so it takes a real price (3 Datasets, 5 credits, 1 insight). |
| Endowed Research Chair | 1 | 4, i1, needs 1 science | 5, i1, needs 2 science | fix | Credit income for insight income at 1-for-1 plus a draw nets about 2 under the price; it becomes 1 credit income for 2 insight income, with no draw rider. |
| Field Grant | 1 | 2, i0 | 6, i0 | fix | 2 cubes for 2 credits is a small effect; it gives 3 cubes and is priced against the Researcher's Grant (4 credits a cube on a project). |
| Field Sensor Grid | 1 | 1, i0 | 10, i0 | fix | 2 markers (8.4) plus a 1-insight rider for 1 credit was far under the rates; the trivial insight is cut and it clears 3 markers. |
| Grad Student Pool | 1 | 3, i1 | 5, i1 | fix | Ruling 12: no Dataset arrives by itself; progressive: add 1 Dataset here, OR pay 1 here to draw 2 cards. |
| Inference Cluster | 1 | 2, i1 | 7, i1 | fix | 1 cube for 2 insight is close to an even trade (2-3 for 5) and creates little; it pays 3 insight a cube. |
| Literature Crawler | 1 | 0, i0 | 5, i0 | fix | Draw 1 for 0 credits nets about 1 over its keep (ruling 1); it becomes the cheap tag-starter that draws 2. |
| Neural Cartographer | 1 | 1, i1 | - | merge | into Zero-Knowledge Exploit: A cut node is 1 compute (ruling 3), so the card was a draw plus 1 compute; folded into Zero-Knowledge Exploit as more compute removed. |
| Open Preprint | 1 | 0, i0 | - | merge | into Shared Compute Grant: 0 credits for a 1-insight gift and a draw is trivial (ruling 1) and a near-duplicate of Shared Compute Grant; merged into it. |
| Predictive Modeling | 1 | 1, i0 | 4, i0 | fix | 1 delay and 1 peek for 1 credit is small; it becomes 2 delays and a 2-card look, where the look decides which threats to delay. |
| Preprint Exchange | 1 | 2, i0 | 8, i0 | fix | 1 cube and a draw for 2 credits was small; 2 research cubes and a draw at a held cube's 3: 8. |
| Red-Team Audit | 1 | 2, i1 | 4, i1 | fix | Jam 1 is a trivial rider; it becomes an untyped jam 4 with the draw, still short of a counter (ruling 8). |
| Sample Repository | 1 | 5, i1, needs 1 science | 8, i1, needs 3 science | fix | 1 cube a round at 5 credits sat at a held cube's 1-3; it becomes a needs-3 chain card at a held cube's 3, still 1 cube a round, at 8. |
| Shared Compute Grant | 1 | 0, i1 | 6, i1 | fix | Absorbs Open Preprint: one help card, a bigger gift and a draw for you, so paying a keep to give is worth it. |
| Simulation Cluster | 1 | 2 +1ch, i1 | 9, i1 | fix | Its Action was 3 credits for 1 card (about even, ruling 7) with an extra chip and on-play draw; it becomes Science's energy sink, 2 energy for 2 cards. |
| Stress Test Suite | 1 | 1, i0 | 7, i0 | fix | 2 shields plus a trivial insight for 1 credit; the insight is cut and it shields 3 presence at a real price. |
| Synthetic Data Forge | 1 | 3, i1, needs 1 science | 8, i1, needs 2 science | fix | 4 Datasets for 2 insight lost value each use, and tokens arrived by themselves (ruling 12); now add 1 Dataset here, OR get 1 insight per Dataset here. |
| Telemetry Archive | 1 | 1, i1 | 5, i1 | fix | Ruling 12 makes it progressive: add 1 Dataset here, OR pay 1 here to delay 2 threats; one delay for 2 Datasets was small. |
| Zero-Knowledge Exploit | 1 | 1, i1 | 6, i1 | fix | Absorbs Neural Cartographer: the cut node's 1 compute is folded in, so it removes 7 compute and draws 1, beating the drain project (5 credits for 3). |
| Adversarial Inference Sweep | 2 | 1, i1, needs 1 cyber | 7, i1, needs 1 cyber | fix | 1 compute off each project plus 2 delays (6.65) was under a tier-2 keep at any price; it is scaled to a real tier-2 sweep. |
| Breakthrough Lab | 2 | 7, i2, needs 2 science | 10, i2, needs 4 science | fix | 2 cubes for a node at 7 sat under its tier-2 keep and was never played; it is a needs-4 chain card, 2 cubes for a node, at 10. |
| Data Licensing Deal | 2 | 0, i1 | 4, i1 | fix | cash_out now takes one card's pile (ruling 12), and tier 2 sells Datasets at 4; a 0-credit tier-2 card gets a price. |
| Exabyte Archive | 2 | 4, i2, needs 1 science | 6, i2, needs 3 science | fix | Ruling 12: no Dataset arrives by itself; it arrives loaded (2 Datasets here when built), then add 2 here OR cash this card's pile at 3, behind needs 3 Science. |
| Foundation Model Campus | 2 | 7 +2ch +1am, i2, needs 3 science | 16 +1am, i2, needs 6 science | fix | Science's needs-6 capstone: 3 insight and 1 credit per Science tag a round, the chip cost dropped, at 16 and 1 antimatter. |
| Open-Weights Release | 2 | 7, i1 | 11, i2 | fix | At 7 for 3 markers and a 2-insight gift it sat at the recipe but small for tier 2; it clears 4 and gives 3. |
| Orbital Observatory Array | 2 | 7 +1ch, i2, needs 1 cyber | 16, i2, needs 1 cyber | fix | 2 insight income plus 3 credits for 2 cards was a modest tier-2 engine; the Action draws 3 and the chip cost goes, priced as the tag's big tier-2 income card. |
| Policy Institute | 2 | 5, i1, needs 2 science | 6, i1, needs 3 science | fix | 2 cubes for 2 Sway at 5 sat under its tier-2 keep and was never played; Sway 3 for 2 cubes, needs 3 Science, at 6. |
| Recursive Self-Improvement | 2 | 7 +1am, i2, needs 2 science | 16 +1am, i2, needs 3 science | fix | +2 insight income and 3 cards for 7 and 1 antimatter barely cleared the antimatter; an ultimate now: +3 insight income and 4 cards, at 16 and 1 antimatter, needs 3 Science. |
| Antitrust Action | 1 | 1, i0 | 6, i0 | fix | A 1-credit jam of 4 nearly counters every tier-1 economic threat (4-5) for less than the plain payment. That is a cheap near-counter with no decision. |
| Bonded Warehouse | 1 | 4, i1 | 6, i1 | fix | Per-round Capital is retired (ruling 12) and the 2-chip build rider had no decision; now add 2 Capital here, OR remove them for 3 credits each. |
| Bridge Loan | 1 | 0, i0 | 0, i0 | reprice | 6 credits for -1 credit income (about 5) nets about 1, under the keep it costs. |
| Bulk Silicon Procurement | 1 | 1, i0 | 3, i1 | fix | 3 chips for 1 credit + the keep is a small one-off with no decision (ruling 1). |
| Chip Spot Market | 1 | 0 +3ch, i0 | - | cut | 3 chips for 6 credits nets about 1.5 after the keep (ruling 1); at the analyst's 5 chips for 10 it still nets 0. Selling chips stays the standard action; Industry's chips go to buildings and Harden. |
| Collateralized Grid | 1 | 2, i1 | 6, i1 | fix | "Gain 1 energy per 2 Capital you have" counts tokens across cards (ruling 12), and it is a halved rate that needs arithmetic. |
| Expansion Fund | 1 | 0, i1 | 4, i1 | fix | Capital arrived by itself (ruling 12) and the 3-Capital factory came in round 5 or later; now add 1 Capital here, OR pay 1 here to build a factory. |
| Grid-Tied Smelter | 1 | 4, i1 | 9, i1, needs 3 power | fix | 1 credit per Power tag at tier 1 with no needs broke the per-tag rate; it needs 3 Power now, at 9, the per-tag income shape. |
| Joint Fabrication Pact | 1 | 0, i1 | 6, i1 | fix | 2 chips each for 0 credits + 1 insight + the keep: trivial. The keep and insight alone cost more than the 3 chips you gain. |
| Lights-Out Assembly | 1 | 2, i1 | 5, i1 | fix | 1 energy (2) for 1 chip + 2 credits (3.5) nets 1.5 a use, with two gains on one face. |
| Liquidity Event | 1 | 0, i1 | 3, i1 | fix | "Remove all the Capital you have" recounts across cards (ruling 12). Megacorp IPO did the same job at tier 2, so it merges in here. |
| Market Watchdog | 1 | 2, i1 | 4, i1 | fix | 1 energy for a typed jam of 3 lost money each use; the jam is now free each round (jam 3: a free jam 4 a round would cancel most tier-1 economic threats, a standing counter). |
| Materials Hedge Desk → Capital Injection | 1 | 0, i1 | 7, i1 | fix | Selling every Capital for 1 chip each was a weak, near-duplicate Liquidity Event. Ruling 12 needs feeders instead. |
| Payroll Automation | 1 | 2, i1 | 8, i2 | fix | +1 credit income + 2 chips for 2 credits: a small income plus a rider that adds a symbol but no decision. |
| Prefab Plant Kit | 1 | 2, i1 | 5, i1 | fix | The flat 2-energy rider created no decision. A per-tag one-off (ruling 11) rewards the Industry-Power seat it is printed for. |
| Reclamation Crew | 1 | 2, i1 | 6, i1 | fix | Freeing one facility plus a 2-chip rider was small, and the rider had no decision. |
| Retained Earnings | 1 | 2, i0 | 6, i1 | fix | Capital no longer arrives by itself (ruling 12), and 1 Capital for 1 chip was trivial; now add 2 Capital here, OR get 1 credit per Capital here (they stay). |
| Robotic Foundry | 1 | 6 +1ch, i1 | 12 +1ch, i1 | reprice | +3 credit income for 6 credits + 1 chip sat about 3 under the recipe. It is now one of the tag's strong 10-14 cards. |
| Supply Chain Audit | 1 | 2, i1 | 4, i1 | fix | Jam 5 for 2 credits, 1 insight and the keep lost to paying 5 (ruling 7); now jam 5 and +1 credit income, the tag's rider. |
| Supply Chain Treaty | 1 | 3, i1 | 9, i1 | fix | Its effects were scaled up to the new curve: +1 income plus a 3-credit gift was small. |
| Vendor Financing | 1 | 0, i0 | 6, i1 | fix | "Give up to 4" lets the helper give 0 and take 3 Capital for free. |
| Arc Furnace Complex | 2 | 9 +2ch, i2, needs 1 power | 10 +2ch, i2, needs 1 power | fix | At tier 2, 2 energy for 6 credits barely beat the tier-1 converter, and the income was 1 chip. |
| Arcology Foundry | 2 | 11 +2ch, i2, needs 2 industry | 15 +2ch, i2, needs 2 industry | reprice | Its income was scaled up so tier 2 is far stronger than Robotic Foundry's tier-1 4 credits (ruling 4). |
| Market Stabilization Board | 2 | 4, i1, needs 1 industry | 8, i1, needs 2 industry | fix | Same shape as the other three sources' standing counters: 2 credit income when built and 5 off economic threats at home, needs 2 Industry (070: alone it was kept and never played). |
| Megacorp IPO | 2 | 4, i2 | - | merge | into Liquidity Event: With cash-outs limited to one card, its job (big Capital sale) is Sovereign Capital Fund's own 4-a-Capital cash-out plus Liquidity Event. |
| Orbital Mass Driver | 2 | 17 +3ch +1am, i2, needs 2 industry 1 power | 20 +3ch +1am, i2, needs 3 industry 1 power | reprice | The ultimate's +7 / +2 was matched by the rescaled Arcology. It needs to be the clear top of the tag and to sit in the 18-30 band. |
| Sovereign Capital Fund | 2 | 7, i2, needs 1 industry | 9, i2, needs 3 industry | fix | 2 Capital a round arriving by itself breaks ruling 12, and its antimatter Action duplicated Power's sources; it arrives loaded (2 Capital) and cashes this card's pile at 4, behind needs 3 Industry. |
| Vertical Monopoly | 2 | 9, i2, needs 2 industry | 14, i2, needs 4 industry | fix | 1 credit per Industry tag alone, about 5 a round for 3 rounds, sat under its 9 credits + 2 insight + keep. It is now the tag's chain capstone. |
| Adaptive Malware | 1 | 2, i1 | 5, i1, needs 3 cyber | fix | 3 compute and a jam of 1 were two small riders; it is Cyber's chain card: needs 3 Cyber, 6 compute off each project, at 5. |
| Blackout Script | 1 | 1, i0 | 4, i0 | fix | Matt's take: the node cut is about 1 compute, so fold it in: remove 5 compute, about 4 credits. It also absorbs Flash Exploit, which was the same 5-compute drain. |
| Compute Siphon | 1 | 3, i1 | - | merge | into Zero-Click Breach: It was a plain 6-compute drain, a near-duplicate of Flash Exploit and Zero-Click Breach at another size (guideline 10). |
| Darknet Exchange | 1 | 1, i1 | 4, i1 | reprice | The per-tag income is right for tier 1 (1 per 2 tags, no needs), but at 1 credit it was underpriced for about 1.75 credits a round. |
| Eviction Worm | 1 | 2, i1 | 7, i1 | fix | 2 ambassadors or 1 troop in one hex was worth about 5 against a cost of about 7. It becomes a big country-wide ambassador purge (ruling 6: a printed number). |
| Flash Exploit | 1 | 2, i1 | - | merge | into Blackout Script: Its 5-compute event is exactly Matt's new Blackout Script; two of the same is a near-duplicate, and an event leaving no tag is no reason here. |
| Honeypot Relay | 1 | 2, i1 | 6, i1 | fix | Paying 1 credit to remove 1 ambassador gained about 1 credit a use; now a free Action: remove 2 adversary ambassadors from a hex in reach. |
| Incident Response | 1 | 1, i0 | - | merge | into Threat Hunt: A second typed cyber jam program (4) beside Threat Hunt (5): one card does the job, with a Cyber rider so it beats the plain route. |
| Intrusion Lab | 1 | 3, i1 | 7, i1 | fix | 1 energy for 2 compute was about 0.2 a use; now 1 energy for 4 compute, and it absorbs Rootkit Cache (a 1-credit node cut is 1 compute). |
| Logic Bomb | 1 | 5, i1 | 11, i1 | fix | 6 compute plus 1 ambassador was a drain with a small rider. It becomes a big drain plus a real troop strike, a strong tier-1 card at a high price. |
| Neural Mirror | 1 | 6, i1 | 6, i0 | reprice | +1 insight income is worth about 8 at tier 1, so 6 cr + 1 insight was about right. Moved to 0 insight at the same total so it can be played on a tight insight round. |
| Quantum Decryptor | 1 | 4 +1ch, i1 | 6 +1ch, i1 | fix | '1 credit, plus 1 insight per 2 Science tags' was two incomes with a near-constant flat credit; fold it out and keep the per-tag idea. |
| Rootkit Cache | 1 | 1, i0 | - | merge | into Intrusion Lab: Paying 1 credit to cut a node (1 compute) is a trivial action (rulings 1, 3). |
| Security Ops Floor | 1 | 2, i1 | 4, i1 | fix | 1 energy for a typed jam of 3 was worth about 0; the jam is now a free Action (jam 3, not 4: a standing counter otherwise). |
| Sentinel Mesh | 1 | 3, i1 | 3, i1 | fix | 1 energy for a jam of 2 netted 0 a use. It is now free, the any-type jam beside Security Ops Floor's bigger cyber-only one. |
| Shadow Broker Scrub | 1 | 1, i0 | 8, i1 | fix | 1 marker plus a draw at 1 credit was underpriced and small. It now has a bigger number at a real price. |
| Shared Exploit Kit | 1 | 3, i0 | 6, i1 | fix | 3 compute and 1 insight to a friend were both small; the help card now does a real drain and a real gift. |
| Signal Hijack | 1 | 2, i1 | 5, i1 | fix | 3 ambassadors or 1 troop in reach was worth about 6 against a cost of about 7. It now has a printed troop number in one country (ruling 6). |
| Spoofed Credentials → Credential Harvest | 1 | 0, i0 | 5, i0 | fix | A 0-credit '1 ambassador in reach' was a trivial card (ruling 1). It is renamed Credential Harvest and becomes Cyber's one-off per-tag payoff (ruling 11). |
| Threat Hunt | 1 | 2, i1 | 5, i0 | fix | Matt's example of ruling 7 (3 credits + about 2 insight for 'counts as 5' lost to paying 5): now jam 5 and 4 compute removed, 0 insight. Incident Response merges in. |
| Zero-Click Breach | 1 | 4, i2 | 8, i2 | fix | The node cut folds into compute (ruling 3), and it absorbs Compute Siphon: 12 compute at 8 (18 for 12 matched tier-2 Killswitch Directive). |
| Autonomous Defense Grid | 2 | 4, i1, needs 1 cyber | 8, i1, needs 2 cyber | fix | An automatic counter is worth only its hits (about 0.3 a round, home country only, 070), so at 4 alone it was kept and never played; the four sources' standing counters are now one shape: 2 credit income when built and 5 off its type at home, needs 2 of the tag. |
| Black Ice Citadel | 2 | 6 +1ch, i2, needs 1 cyber | 7 +1ch, i2, needs 1 cyber | fix | 1 compute off each project a round was about 6.6 over a tier-2 lifetime, below its cost of about 18.5. Tier-2 effects go up a lot. |
| Hyperscale Botnet | 2 | 12 +1ch, i2, needs 2 cyber | 10 +1ch, i2, needs 4 cyber | fix | Cyber's capstone income becomes a needs-4 chain (ruling 10). 2 insight a round makes the income far stronger, and the requirement keeps the price modest. |
| Killswitch Directive | 2 | 9, i2 | 12, i2 | reprice | One clear idea, kept. A near-full stage-2 project (about 20 compute plus its effect) is worth about 25, so 9 was cheap. |
| Node Severance | 2 | 5, i2, needs 1 cyber | 8, i2, needs 1 cyber | fix | A lone node removal (about 12) was below its cost of about 16.5. Adding a real drain makes it worth a tier-2 keep. |
| Singularity Exploit | 2 | 13 +1am, i2, needs 1 cyber | 18 +1am, i2, needs 2 cyber | reprice | The ultimate keeps its identity (a node and a discard, about 37 of effect); 13 and 1 antimatter was cheap: 18. |
| Baseload Contract | 1 | 2, i1 | 5, i1 | reprice | +1 energy income for 2 is a small effect; doubled, it is the tag's plain no-slot energy income |
| Blackstart Protocol | 1 | 0, i0 | - | merge | into Hardened Conduits: Gain 2 energy plus harden 1 is two small riders at 0 credits; its hardening joins Hardened Conduits and its energy burst joins Capacitor Discharge |
| Brownout Relief | 1 | 1, i1 | 6, i1 | fix | Two small gifts (2 energy and 1 insight, which is lost at round end) and a paid presence; now one gift and a free presence |
| Capacitor Discharge | 1 | 0, i0 | 3, i0 | fix | Gain 2 energy and 1 credit is trivial (ruling 1); now the one-off per-tag energy payoff (ruling 11) |
| Demand Response Grid | 1 | 2, i1 | 9, i1 | reprice | The tag's energy-to-credits converter: energy past presence measures under its rate, so this sink is what makes Power's income pay |
| Faraday Yard | 1 | 4, i1 | 6, i1 | fix | Pay 1 energy to harden 2 was near-negligible after keep; a free hardening Action is the tag's standing protection |
| Fusion Startup Stake | 1 | 5, i1 | 10, i1 | reprice | +1/+1 income at 5 sat at the rate; doubling the credit side makes it a strong tier-1 engine at a high price |
| Grid Census | 1 | 1, i1 | 6, i1 | reprice | Priced for 1-2 presence; a mid-game seat has 5-6 presence, so it was a cheap energy burst |
| Hardened Conduits | 1 | 0 +1en, i0 | 2, i0 | fix | Pay 1 energy then harden 3 was near-circular with Blackstart's gain 2 energy, harden 1; one cheap harden card is enough |
| Peaker Turbines | 1 | 4, i1 | 9, i1 | fix | Lowering credit income 1 to gain 2 energy income valued under its 4-credit price, since energy measures under 2; now two incomes that bridge to Industry |
| Pylon Drop | 1 | 0, i0 | - | merge | into Relay Mast Rollout: 1 energy plus 1 presence at 0 credits is a filler, and the energy only pays the step it uncovers (circular) |
| Relay Mast Rollout | 1 | 1, i1 | 7, i1 | fix | 2 presence for 1 was a small card; 3 presence absorbs Pylon Drop's job as the tag's presence burst |
| Shared Grid Accord | 1 | 0 +2en, i0 | 1 +2en, i1 | fix | Pay 2 energy, gain 2 credits was a weak return; Sway is a helper's return that is neither credits nor trivial |
| Storm Hardening | 1 | 0 +2en, i1 | 5, i0 | fix | Pay 2 energy plus 1 insight for shelter and harden 2 came out under its keep; now a credit-paid reaction with harden 3 |
| Substation Hub | 1 | 4, i1 | 5, i1 | reprice | Sound bridge card; priced up a step to the rate |
| Surplus Auction | 1 | 0 +2en, i0 | 2 +3en, i0 | fix | Matt's take: pay 3 energy, gain 5 chips, at a real credit cost; the 1-credit rider goes (ruling 1). |
| Tidal Barrage | 1 | 4, i1 | 9, i1, needs 3 power | fix | Flat 2 energy duplicated Baseload Contract in a slot; now the Power chain card: 1 energy per Power tag on your cards, needs 3, at 9. |
| Transmission Corridor | 1 | 1, i1 | 6, i1 | fix | 1 presence and a 2-hex move was small; 2 presence and up to 2 ambassadors moved makes it the Power/Diplomacy bridge |
| Aegis Grid Lockdown | 2 | 4, i2 | 9, i1 | fix | Shelter plus 2 free presence at 2 insight was under its keep; 3 free presence and 1 insight make it a real emergency play |
| Antiproton Offtake Deal | 2 | 6, i2, needs 2 power | - | merge | into Positron Storage Ring: Antimatter income with a credit-income tax repeats Positron Storage Ring; antimatter has too few sinks for two such cards |
| Beamed Power Constellation | 2 | 7, i2, needs 1 power | 13, i2, needs 2 power | reprice | 3 free presence plus about 8 energy was cheap at 7; priced up to the tier-2 curve, with energy counted at its measured worth |
| Continental Supergrid | 2 | 10, i2, needs 2 power | 12, i2, needs 6 power | fix | Per-tag energy moved to Tidal Barrage; this is the needs-6 chain payoff, paying credits per Power tag |
| Dyson Swarm Tranche | 2 | 11 +1am, i2, needs 2 power | 18 +1am, i2, needs 4 power | reprice | The ultimate paid 5 energy and 2 credits, mostly energy, which measures weak; credits now carry it at an ultimate price |
| Interconnector Treaty | 2 | 4 +2en, i2, needs 1 power | 13, i2, needs 1 power | fix | Paying 2 energy to give 3 energy and gain 2 energy income came out under its keep; now a large gift plus credit income |
| Positron Storage Ring | 2 | 10, i2, needs 1 power | 14, i2, needs 3 power | fix | Takes in Antiproton Offtake Deal (two antimatter-income cards were near-duplicates); bigger energy, gated by 3 Power |
| Amnesty Decree | 1 | 2, i1 | 7, i0 | fix | At 2 removals it lost to the standard Remove action: one ambassador clears 3 in a hex and stays on the board. |
| Back-Channel Talks | 1 | 1, i1 | 7, i1 | reprice | Freeing 1 facility plus 1 Sway was underpriced at 1 credit; doubling the cleanse makes it a real choice. |
| Ceasefire Monitors | 1 | 4, i1 | 10, i1 | fix | Its 1-ambassador removal was a trivial rider; it now removes 3 ambassadors or 1 troop, so the removal is a real choice. |
| Civic Watch | 1 | 2, i1 | 4, i1 | fix | A 3-jam at 1 energy a use, behind a keep, lost to paying; now a free Action (jam 3). |
| Commons Grid | 1 | 4, i1 | 7, i1 | fix | Ruling 12: no automatic Followers, and no 'per 2 Followers you have' counted across cards. It is now a progressive card that counts only its own Followers. |
| Consular Pouch | 1 | 4, i0 | 7, i1 | merge | Absorbs Shuttle Diplomacy, its near-duplicate; the move rider was too small to keep (ruling 1), so it is now a bigger single gain. |
| Consulate Tower | 1 | 6, i1 | 9, i1, needs 3 diplomacy | fix | Three effects on one face, and its 4-credit ambassador Action sat at the rate; one chain payoff now: 1 credit per Diplomacy tag on your cards, needs 3, at 9. |
| Development Loan | 1 | 0, i1 | - | merge | into Mutual Aid Pact: 'Give up to 6 credits, gain 1 ambassador' at 0 lets the helper give 0 and take the ambassador; a printed gift is Mutual Aid Pact's job, which takes its second ambassador. |
| Embassy Compound | 1 | 4, i1 | 9, i1 | fix | Its 1-credit move Action was only the standard Move action. It is now a free move each round. |
| Fact-Check Network | 1 | 1, i0 | 5, i0 | fix | A bare jam of 4 lost to paying 4 once the keep counts (ruling 7); now jam 5 and an ambassador. |
| Joint Security Compact | 1 | 2, i1 | 6, i1 | merge | Absorbs Peacekeeping Detail's harden job. Two small gifts become one, and the card now hardens 3 facilities. |
| Liaison Corps | 1 | 4, i1 | 6, i1 | fix | Ruling 12: Followers no longer arrive by themselves. It is now a progressive card that cashes its Followers for ambassadors. |
| Manifesto Leak | 1 | 0, i0 | 5, i0 | fix | A 0-credit add of 3 was close to trivial. Ruling 12 makes feeders the way to speed a token card, so it now adds 4 at a real price. |
| Mutual Aid Pact | 1 | 2, i0 | 8, i0 | merge | Absorbs Development Loan: one help card, a printed 6-credit gift and 2 ambassadors for you. |
| Open Assembly | 1 | 2, i1 | 6, i1 | fix | Ruling 12; and it duplicated Liaison Corps (Followers for ambassadors, only slower). It is now the tier-1 Followers-to-Sway card. |
| Peace Dividend | 1 | 2, i0 | 5, i0 | fix | Circular payment (pay 2, gain 2: ruling 2). Matt's take: Sway 2, no credits, about 5. |
| Peacekeeping Detail | 1 | 1, i0 | - | merge | into Joint Security Compact: Two small riders (harden 2, a 2-hex move) and no decision (ruling 1). Its harden job goes to Joint Security Compact, its move to Embassy Compound. |
| Shuttle Diplomacy | 1 | 3, i0 | - | merge | into Consular Pouch: A near-duplicate of Consular Pouch (2 ambassadors) with a 1.5-credit move rider (rulings 1, guideline 10). |
| Summit Caucus | 1 | 4, i1 | 9, i1 | reprice | The city-freeing setup card, scaled up: consolidating 4 meets a hold of 3 plus 1 adjacent. |
| Treaty Grid Exchange | 1 | 5, i1 | 11, i1 | reprice | A plain income building; it is raised to the new curve with 2 credits and 1 energy a round. |
| Truth Commission | 1 | 2, i1 | 5, i0 | fix | Jam 5 for 2 credits, 1 insight and the keep lost to paying 5 (ruling 7); now jam 5 and Sway 1, no insight. |
| Civic Trust Bureau | 2 | 4, i1, needs 1 diplomacy | 8, i1, needs 2 diplomacy | fix | Same shape as the other three sources' standing counters (stays tier 2): 2 credit income when built and 5 off influence threats at home, needs 2 Diplomacy. |
| Coalition Charter | 2 | 7, i2 | 14, i2 | fix | Three effects (two gifts and Sway). It now has one gift and a bigger Sway, priced to the tier-2 curve. |
| Exodus Corridors | 2 | 6, i2, needs 1 diplomacy | 12, i2, needs 1 diplomacy | reprice | Its 2 ambassadors were too small beside the spread for tier 2; it now gains 4. |
| Great Liberation | 2 | 15 +1am, i2, needs 1 diplomacy | 21 +1am, i2, needs 4 diplomacy | fix | The ultimate, scaled up: it frees 3 corrupted cities, stations or factories, behind a needs-4 chain. |
| Liberation Accords | 2 | 8, i2, needs 1 diplomacy | 14, i2, needs 1 diplomacy | fix | Ruling 6: no 'remove every'. It now prints 6, and gains 3 ambassadors, enough to meet a base hold. |
| Orbital Embassy | 2 | 9 +1am, i2, needs 1 diplomacy | 17 +1am, i2, needs 3 diplomacy | fix | Three incomes plus a gain was a crowded face, and weak for an antimatter ultimate. It is now 2 ambassadors and 3 credits a round, behind needs 3. |
| Planetary Assembly | 2 | 13 +1ch, i2, needs 2 diplomacy | 12 +1ch, i2, needs 3 diplomacy | fix | Ruling 12: no automatic Followers. It is now progressive, with a Sway payout of 5, about a tip, behind needs 3. |
| Planetary Referendum | 2 | 3, i2 | 11, i2 | reprice | Sway 4 for 3 credits sat far under the tier-2 curve. It is now Sway 6, a full tip, at 11. |
| Agency Field Station | 1 | 2, i1 | 4, i1 | fix | Counted Agents 'you have' across cards (ruling 12) and gained them by itself; now Matt's progressive card: add 1 Agent here, OR get 1 credit per Agent here (they stay). |
| Bounty Board | 1 | 2, i1 | 4, i1 | fix | 'Gain 1 credit' was a trivial rider (ruling 1). It now pays per Security tag, the one-off per-tag payoff (ruling 11). |
| Breach and Clear | 1 | 3, i1 | 10, i1 | fix | 'Gain 2 credits' was a rider with no decision, and '2 ambassadors or 1 troop in a hex' was small. It is now the big tier-1 troop strike. |
| Diplomatic Expulsion | 1 | 2, i0 | 7, i1 | fix | 1 ambassador and 1 Sway at 2 credits was small. It is now a country-wide expulsion with a printed number (ruling 6). |
| Early Warning Drill | 1 | 1, i1 | 6, i1 | fix | Delay 1 and shield 1 at 1 credit was cheap and small. Two shields make it the Science-tagged defensive program. |
| Force Protection Order | 1 | 2, i1 | 5, i1 | fix | Jam 5 at 2 credits, 1 insight and the keep lost to paying (ruling 7); now jam 5 and a troop in reach. Rapid Response, its near-duplicate, merges in. |
| Garrison Command | 1 | 2, i1 | 4, i1 | fix | A 3-jam at 1 energy lost to paying; now a free Action (jam 3). |
| Hardened Bunker Complex | 1 | 4, i1 | 7, i1 | fix | 1 energy of income plus the Harden action was a weak building for its slot. It now has more income, at a higher price. |
| Hardpoint Lockdown | 1 | 3, i0 | 5, i0 | reprice | Shelter plus cordon is two real effects, worth about 9, priced at 3. |
| Interceptor Battery | 1 | 2, i1 | 3, i1 | fix | At the energy rate of 2, '2 energy for 1 troop in reach' was worth about nothing. Any country, plus 075's finding that spare energy buys little, makes it a real energy sink. |
| Intrusion Countermeasures | 1 | 2, i1 | 4, i1 | fix | The jam of 1 was a near-constant rider (rulings 1 and 3). One bigger drain does the card's job. |
| Kill Team Roster | 1 | 3, i1 | 6, i1 | fix | Automatic Agents (ruling 12), and 2 Agents for 1 troop in reach lost to Interceptor Battery; now add 1 Agent here, OR pay 1 to remove 2 troops in one country. |
| Mutual Defense Pact | 1 | 1, i1 | 5, i0 | fix | A 2-credit gift was a token rider on a 1-credit card. The help is now worth the card. |
| Peacekeeper Rotation | 1 | 1, i1 | 3, i1 | fix | One ambassador in reach was a removal too small to draft. It now removes three, keeping the ambassador stretch. |
| Rapid Response | 1 | 1, i0 | - | merge | into Force Protection Order: A jam of 4 military at 1 credit plus the keep is about 3.5 for about 2.8, and it near-duplicated Force Protection Order at a different number. |
| Recruit Operatives | 1 | 0, i0 | 6, i0 | reprice | The feeder is the only card that adds Agents to a card, and under ruling 12 Agents are worth more than the 1 a held token is rated, so a 0-credit price undercharges it. |
| Safehouse Network | 1 | 2, i1 | 4, i1 | fix | Automatic Agents (ruling 12). 2 Agents for one shelter made it a shelter every other round at best. |
| Security Contractor HQ | 1 | 1, i1 | 9, i1, needs 3 security | fix | Per 2 Security tags at 1 credit was cheap filler; the chain payoff (rulings 10, 11): 1 credit per Security tag on your cards, needs 3, at 9. |
| Sentry Drone Wing | 1 | 2, i1 | 4, i1 | fix | Automatic Agents (ruling 12). At 2 Agents for 2 hardenings, the hardening was too thin to pay for a slot. |
| Shared Threat Intel | 1 | 1, i0 | 6, i0 | fix | A jam of 1 plus 1 insight lost to paying 1 credit (ruling 7). Threat Triage merges in, and the help card now carries a real jam. |
| Threat Triage | 1 | 1, i0 | - | merge | into Shared Threat Intel: A jam of 2 for 1 credit plus the keep (3.5) loses to paying 2 credits (ruling 7), and it nearly duplicated Shared Threat Intel. |
| Aegis Dome Array | 2 | 5, i2, needs 1 security | 9, i2, needs 1 security | fix | A standing shelter alone lost to its tier-2 keep and insight; it now also hardens 3 facilities when built (income would not fit the box). |
| Agency Directorate | 2 | 9, i2, needs 2 security | 14, i2, needs 2 security | fix | '1 troop per hex of a country' was a hidden fixed number (ruling 6) and its Agents arrived by themselves (ruling 12); it arrives loaded (2 Agents) and pays 2 Agents to remove 3 troops in one country. |
| Deep State Ledger | 2 | 7, i2 | 6, i2, needs 3 security | fix | Counted Agents 'you have' across cards (ruling 12); now it arrives loaded (3 Agents here when built) and streams: add 3 Agents here, OR get 1 credit per Agent here, behind needs 3 Security. |
| Kinetic Interdiction | 2 | 5 +1am, i2, needs 1 security | 16 +1am, i2, needs 2 security | fix | Matt's ruling 6: no 'per hex', no ambassador OR: remove 6 troops in one country, a tier-2 ultimate's price (16 and 1 antimatter), needs 2 Security. |
| Martial Cordon | 2 | 7, i1, needs 1 diplomacy | 12, i2, needs 1 diplomacy | fix | 'Remove every ambassador' is an 'all' removal (ruling 6), and 'sway 1 per 3 removed' was arithmetic. Both are now printed numbers. |
| Sentinel Command | 2 | 4, i1, needs 1 security | 8, i1, needs 2 security | fix | Same shape as the other three sources' standing counters: 2 credit income when built and 5 off military threats at home, needs 2 Security. |
| Total Containment | 2 | 8, i2, needs 1 security | 13, i2, needs 1 security | fix | 'Every ambassador and troop in one hex' is an 'all' removal. It is now a printed troop number plus the shelter. |
| Foundry Core | 0 | 0, i0 | 0, i0 | keep | Builder's starting building: 1 credit a round in a slot. Kept (setup piece, prompt). |
| Modular Expansion | 0 | 4, i1 | 4, i1 | keep | Builder's: 4 credits for 2 more building slots, the Builder's engine; no plain route gives slots. Kept. |
| Turnkey Factory | 0 | 3, i1 | 3, i1 | keep | Builder's: 3 credits for a factory in reach and 1 credit a round; it starts in hand (no keep), so it beats the factory route. The prompt keeps the AI-unique cards for now; the 1-credit income is a ruling-1 rider to revisit with them. |
| Venture Capital | 0 | 0, i0 | 0, i0 | keep | Builder's: 5 credits now and 2 to a partner for nothing; a start-of-game tempo card with no keep. Kept for now (prompt); a gift with no decision. |
| Point Defense | 0 | 3, i1 | 3, i1 | keep | Protector's: 1 energy for 2 hardenings a round beside its Bulwark power; small but repeatable, no keep. Kept for now (prompt). |
| Sentinel Post | 0 | 0, i0 | 0, i0 | keep | Protector's starting building: 1 energy a round in a slot. Kept (setup piece, prompt). |
| Shelter Order | 0 | 0, i0 | 3, i0 | reprice | 0 credits for a shelter duplicated the Protector's Shelter power for free (guideline 11); it takes a real price, 3 credits (a shelter is 4; it starts in hand, so no keep). |
| Strike Team | 0 | 1, i1 | 1, i1 | keep | Protector's: 1 credit and 1 insight for 3 ambassadors or 1 troop in reach, under the standard Remove's price; starts in hand. Kept for now (prompt). |


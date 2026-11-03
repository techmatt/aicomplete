# Card pricing: the reasoning (023, rules r14; current rates to r29)

Every price here is a **candidate**. The card list itself is [cards.md](cards.md),
generated from `content/cards.json`. The screen that checked these prices, and the
patches it led to, are in
[experiments/results/screen-023-cards/cards.md](../experiments/results/screen-023-cards/cards.md).

## Current rates (to r30)

This table and its notes are the only home of a current rate; every section below keeps the numbers of its day. The
dated sections run newest first, then the 023 sections by tag, and the cards as they stand are in [cards.md](cards.md).

| thing | rate (credits) | why |
|---|---|---|
| insight | 2.5 | **Was 3-5.** Unspent insight is lost each round, so credits bind, not insight. At 4 the set read 85 of 150 cards too dear. **A fit:** 2.5 centers tier 1 on its printed prices (median gap 0). It is not a measure: unspent insight was not read at 055 (last at 021, r12), and `economy.insight_unspent` would settle it. Credits binding is the reasoning, not a result |
| keep | tier 1 2.5, tier 2 6.5 | insight at 2.5, tier 2 between 3 and the upgraded 2 |
| energy | 2 | was 1. 066: no standard action sells energy or spends it on the map now, so this rests on a presence (1 energy) and on cards; a price pass owes it a look |
| chip | 1.5 | sells at 1, Chip Spot Market pays 2 |
| antimatter | 9 | 1 a round costs 8-11 of credits |
| token held | 1 | spent, it is worth what its action buys |
| Capital or Dataset sold | 2 credits, 1 chip, or 4 on the tier-2 card | 059: replaces 1 + 2 + ... + n. 4 Capital paid 10, now 8; 8 paid 15, capped, now 16 |
| research cube | 3 | new (074): a station costs 2 chips and makes about 3 cubes before its card completes. Reasoned, not fitted |
| drawn tier-1 card | 4 | was 1 insight; a keep saved, blind, and it still costs its play |
| tag left in play | 1, then 0.5 each | new: discounts, `needs`, powers. Events leave none |
| Sway toward Unity | 4 | **was 1.5-2.** New: a tip is 4-6 Sway, about 5 tips win, and a tip pays a Unity card. Reasoned, not fitted |
| presence | 4 early, 3 late; +2 if it prints free | new: income track plus a life |
| per presence, uncapped | as 055: the track has only 9 spaces | 059: Grid Census 2, Beamed Power Constellation 8 |
| ambassador | 4; a move up to 2 hexes 1.5 | 068: it stays for a position, so worth more; project ceiling `ambassador_credits`, Consular Pouch 2 for 5 |
| Follower | 1.5, at 2 for an ambassador; a building earns its first ambassador about round 4 | 068: Liaison Corps 2 for 1; Open Assembly, cheaper and two tags, 3 for 1 |
| removal | 1 worth 2, 2 worth 5, 3 worth 6 | a troop is 2 worth and sways 1 |
| troop removal | 1 troop 5; 3 ambassadors or 1 troop 6 | 059: the card names a kind: one choice, no mix |
| marker cleared | 6, × 0.7 for a target | 2 credits (066), 1 Sway, a hex back |
| shield 4, building token 1.5, shelter 4 | | conditional; project ceiling 3 |
| delay 2.5, jam N is N, counter 7 | | |
| typed jam of N | about 0.7 N | 070: it needs a threat of its type that you may counter |
| automatic counter | by its hits, not its N | 070: see *070's changes* |
| compute removed | 1.1; every project × 1.5 | project: 3 for 5 |
| adversary compute made | 1.1 against the players; on every visible project 1.1 a project | new (074): the removal's rate read the other way. **Measured dearer than this:** see *074's changes* |
| cut node | 1 | **was 3:** `adversary.node_compute` is 1 |
| node removed | 12 | 2 Sway plus its compute and the block on claims |
| city freed | 25 | 5 Sway plus a city |
| income, tier 1 / tier 2 | credit 5 / 3, energy 5 / 3.5, chip 4 / 2.5, insight 8 / 6 | about 6 incomes in tier 1, 3 in tier 2. Insight's pair was built at 4, not rebased |
| a gift to another seat | 0.75 × face | new: they may not need it then |
| income per tag | 1 per 2 tags at tier 1, 1 per tag only with a `needs` or a tier-2 price | 059: a rate where a cap used to be. 055 priced it, capped, at credit income × about 1.2 |
| repeatable Action | (value less its cost) × uses × 0.7 | tier 1 about 5 uses, tier 2 about 2-3 |
| a fixed trade | the two sides at these rates | 059: replaces every `convert`: nothing to cap, nothing to count |

## 074's changes (r30): research cubes and the corps

The corp cards against *Current rates*, in credits, every line a candidate. **Payouts** are a player
cube, near a cube's 3; **converters** give what they take; **after** is a cube's worth to the adversary.

| Corp | Player cube | Adversary cube | Converter: in → out | Each adversary cube after |
|---|---|---|---|---|
| Horizon | 1 energy, 2 | 1 Sway to Discord, 4 | cube + 2 energy, 7 → antimatter, 9 | 2 compute, 2.2 |
| Cintamani | 3 credits, 3 | 1 compute, 1.1 | cube, 3 → insight, 2.5 | 1 compute on 3 projects, 3.3 |
| Onyx | 3 credits, 3 | 1 Sway to Discord, 4 | cube, 3 → 2 chips, 3 | 3 compute, 3.3 |
| Biomia | 1 card, 4 | 1 Sway to Discord, 4 | 2 cubes, 6 → 1 Sway to Unity, 4 | 1 Sway to Discord, 4 |

**More than 1.5× off its neighbors:**

- **Cintamani's adversary cube at completion (1.1 against 4, 3.6×):** by design, the one card that pays no Discord.
- **Horizon's after line (2.2 against Biomia's 4, 1.8×).** The rest sit within 1.5×: Biomia's converter
  is exactly at it (6 in, 4 out), Horizon's player cube is 0.67× (the card's "a little under"), Biomia's is
  1.33×, *over*: at the 4 a drawn card is priced at, it isn't the "a little under" the item reasoned.
- Horizon's converter pays 2 over (7 in, 9 out), the one that gains, and it sinks the energy 068 saw piling up.
- Cintamani's turns a cube into insight that is lost at the round's end: a cube kept is worth more unless
  it is spent that turn.

## 070's changes (r29): typed counters and capstones

See *Revision 29* in [rules-history.md](rules-history.md) for the numbers that shipped and
why each moved. **A typed jam of N is worth about 0.7 N:** it needs a threat of its type that
you may counter, and a counter card is worth what it saves. **An automatic counter is priced by
its hits, not its N:** a type is about a quarter of the deck and a country sees about 1.3
threats a round, so it fires about 0.3 times a round: at 9 and 12 credits it was kept and never played.

## 068's changes (r28)

Ambassadors now stay when they evict, so one is worth more than the 066 rate and the cards that hand them out pay less per ambassador. Each is a candidate.

| Card | Was | Now | Why |
|---|---|---|---|
| Consular Pouch | 5 credits, gain 3 ambassadors | 5 credits, gain 2 | 4 worth each less a little to get it played |
| Liaison Corps | Action: 1 Follower, 1 ambassador | Action: 2 Followers, 1 ambassador | one a round undercut the project from round 2; at 2 the first arrives about round 4 |
| Open Assembly | 2 credits; Action: 2 Followers, 1 Sway | 3 credits; Action: 3 Followers, 1 ambassador | the slow, cheap Follower building; two tags, so 1 Follower slower than Liaison Corps |
| Orbital Embassy | +4 credits, +1 insight; Action: 1 credit, 2 ambassadors | +1 ambassador, +2 credits, +1 insight a round | weaker credit engine; prints ambassador income (`Op::Income` extended) |
| Hardpoint Lockdown | 3 credits; shelter one country, remove 1 adversary ambassador | 4 credits; shelter one country, cordon one country | a Security card blocks arrivals; a cordon stops 2 arrivals or more a country, about 5 worth |

Cyber already expels without an ambassador (Spoofed Credentials, Eviction Worm, Signal Hijack, Honeypot Relay), so no Cyber card changed.

## 066's changes (r27)

066, r27: 38 starting credits and the Consolidate bundle gone. Seven Diplomacy cards were raised and repriced at 2 credits an extra ambassador, the cards' rate of 3 less a little to get them played: Consular Pouch 3 ambassadors for 5, Shuttle Diplomacy 2 for 4, Ceasefire Monitors 2 for 5, Summit Caucus 2 for 5, Consulate Tower an Action of 1 credit for 1 ambassador at 7, Liaison Corps 1 Follower for 1 ambassador at 5, Orbital Embassy an Action of 1 credit for 2. All candidates, reasoned; the screen can't tell them from noise, since each is played under once every two games..

## 059's changes (r25): the card design pass

Every card was read against [card-guidelines.md](card-guidelines.md). 91 of 158 player cards changed in
what they do or cost, 43 in words only. The list is in
[experiments/results/screen-059-cards/design.md](../experiments/results/screen-059-cards/design.md).
The rates it added are in *Current rates*.

**Paper prices are not enough.** A reviewed set with nine reasoned raises (Breakthrough Leak, Grid-Tied
Smelter, Fusion Startup Stake, Asteroid Salvage Rights, Back-Channel Talks, Joint Fabrication Pact,
Blackout Script, Blackstart Protocol, Early Warning Drill) won 8 of 24 at c3; the same set with them put
back won 15. They are back at 055's prices. c3's win rate rests on the cheap cards, as 055 found with
`needs` against credits. Kinetic Interdiction with troops only, and Total Containment without its
shelter, cost another 10 of 40; Grid Census at 4 and Beamed at 11 cost 7.

## 055's changes (r24): the first price pass

Priced at d2 with the planner. The list, the measure and every sweep are in
[experiments/results/screen-055-prices/prices.md](../experiments/results/screen-055-prices/prices.md);
reasoned prices are in `card-prices.csv` beside it. The rates it set are in *Current rates*.

**`needs` is the price that separates a rush.** Credits hit the planner and the rush alike: the planner
drafts tier 2 about two picks in three, and the rush has only half the planner's tier-1 plays, so
what it lacks is tags in play. Dearer tier-2 credits took the planner down more than the rush (52 / 55
against 62 / 65, 40 seeds) with the six `needs` in; six `needs` took the rush down 18-20 and the planner 15
at c2, none at c3 (against the 40-seed base, 80 / 65 and 90 / 78).
Tier-1 cuts did nothing for either. The Sway cards keep no gate but Great Liberation: all nine gates,
three of them on Sway cards, took the planner to 46 at c2 (the win clock).

Which lever moves which: `needs` the rush, tier-2 credits both and the planner more, tier-1 cuts neither,
presence payoffs and dearer presence neither gap.

| card | was | r24 | why |
|---|---|---|---|
| Great Liberation | 14 cr | 16 cr, needs 1 Diplomacy | both lists: frees a city and more, reasoned 21; +20 against tier 2 |
| Beamed Power Constellation | 5 cr | 7 cr | both lists: 3 free presence and up to 6 energy, reasoned 15; +14 |
| Peace Dividend | 2 cr | 3 cr | 2 Sway at 0 insight, reasoned 6; +9 (z 1.8) |
| Darknet Exchange | 4 cr | 2 cr | both lists: 1 credit per 2 tags, below Payroll Automation; reasoned 1 |
| Security Contractor HQ | 4 cr | 3 cr | the same text as Darknet Exchange; reasoned 1 |
| Rootkit Cache | Action 2 cr | Action 1 cr | both lists: at `node_compute` 1 a cut was worth less than its 2 |
| Kinetic Interdiction | no needs | needs 1 Security | the rush's one-shot: played 59-90% |
| Node Severance | no needs | needs 1 Cyber | the same |
| Singularity Exploit | no needs | needs 1 Cyber | the same |
| Exodus Corridors | no needs | needs 1 Diplomacy | the same |
| Mesh Intrusion Pact | no needs | needs 1 Cyber | the same; the rush played it 19 points more |
| Adversarial Inference Sweep | no needs | needs 1 Science | the same; 12 points more |

**Always-Consolidate was not answered by price (growth is gone since 066).** It was to be answered through cards that pay off
presence and draws. Field Sensor Grid at 2 insight, Grid Census at 8 energy and Beamed Power at
8 moved neither gap beyond the noise, nor did dearer presence cards, so neither shipped. Its wins are
purges (22 of 33 at c2, 30 of 35 at c3, fresh seeds), it loses half the presence, and the bundle that pays for it is the growth rule.

## 046's changes (r22)

Shields, delays and marker clearing are Science's alone (the content check refuses them elsewhere;
all five cards that carried one already had the tag). The niche was real but thin: five cards in 150,
kept 1.7-2.5 a game, played 1-1.5, and ignoring Science stayed level. So eight cards now do it, and
each pays back by itself within two rounds of a marker or a threat landing near a city. Same
yardsticks as 041; a marker on a city is also a step of `corrupt_at` 5, so one or two clears take a
city back out of reach of a Fifth Column round.

| card | was | r22 | payback |
|---|---|---|---|
| Field Sensor Grid (T1) | clear 1, up to 1 insight | clear 2, up to 1 insight | 2 cr for up to 2 × (1 energy + 1 Sway) plus 4 credits of insight: the round it is played |
| Benchmark Suite, now Stress Test Suite (T1) | 1 insight, shield 1, 1 Dataset | 1 insight, shield 2 | 2 cr less 4 of insight is free; each hit absorbed saves a token and a marker, about 7 |
| Predictive Modeling (T1) | 2 cr + 1 insight | 2 cr, 0 insight | half a counter, 2-3 credits, the round it is played: one copy pays back on one threat |
| Threat Forecast Engine, now Early Warning Drill (T1) | peek 2, draw 1 | peek 1, one threat waits | the same at three tags; a second delay answers the round the first comes back |
| Shadow Broker Feed, now Shadow Broker Scrub (T1) | peek 2, draw 1 | clear 1, draw 1 | 2 cr for about 3 of marker and a blind card |
| Mesh Intrusion Pact (T2) | drain all 3, gift, 2 credits | drain all 3, gift, shield 2 | the 2 credits become up to 14 when Discord's presence hits start |

Loop: Science niche, fewer losses, more insight, more Science. Each effect is fixed, the discount
floor stays at 1 and eight of 150 cards do it, so a table can't assemble more than a few a game. The
Clear action stays at `clear_energy` 1: at 2 the team without Science lost as much as the team with
it (16 seeds: 94 / 88 at c2, 44 / 31 at c3, against 100 / 88 and 56 / 38), so board play stays the
team's fallback, as the ruling says, and Science is the cheaper, bigger answer.

**The tier-2 keep stays 3, 2 with the upgrade.** A dearer keep doesn't make the mix beat the tier-2
rush; it only makes the game harder. Planner and rush win %, 12 seeds, c2 / c3: 3 and 2: 100 and 100 /
33 and 42; 3 and 3: 67 and 75 / 8 and 17; 4 and 3: 83 and 75 / 8 and 25.

## 041's changes (r20)

Science paid only in insight and cards, and insight becomes board change only through other tags'
cards, so ignoring it cost nothing (-16 ±11 at c2 at r19). Six cards now change the board directly;
each effect is fixed and the floor on discounts stays at 1, so "board action, fewer threats, more
insight" has no loop to run. Yardsticks: a marker cleared is 1 energy, 1 Sway (about 2 credits) and a
hex back; a shield is a token kept (about 4 credits plus its marker); a threat delayed is half a
counter (2-3 credits: it still has to be answered, or it hits next round); a move is 1 energy.

| card | was | r20 | why |
|---|---|---|---|
| Field Sensor Grid (T1) | 3 cr, up to 2 insight | 2 cr, clear 1 marker, up to 1 insight | about 3 of marker and 2 of insight; a tag early, when no marker stands |
| Predictive Modeling (T1) | 2 cr + 1 insight, peek 2, +1 insight next round | peek 1, one threat waits | 2-3 for the delay and 1 for the peek against 6 paid |
| Benchmark Suite, now Stress Test Suite (T1) | 2 cr, 1 insight, 2 Datasets | 2 cr, 1 insight, shield 1, 1 Dataset | the shield replaces a Dataset and a credit's worth of filler at 0 insight, since it only pays if the hex is hit |
| Shared Compute Grant (T1) | give 2 insight (+1), draw 1 | give 2 insight (+1), move an ambassador 2 | a move for a draw: cheaper, on the map |
| Open-Weights Release (T2) | 6 cr + 2 insight, gifts, draw 2, +1 insight | 4 cr + 1 insight, clear 2, shield 2, give 2 insight | the keep (3 insight) is most of the price; about 11 for about 18 when markers stand |
| Adversarial Inference Sweep (T2) | 7 cr + 2 insight, needs 1 Cyber, drain all 1, draw 2 | 5 cr + 1 insight, drain all 1, 2 threats wait, draw 1 | a swing round's worth of counters put off; the draw shrinks to pay for it |

The two tier-2 prices come from the screen (`experiments/archive/screen-041-science2.json`): at 6 and 7 credits
the planner kept them a third of the time and played one in eight dealt. Agents draft tier 2 three
picks in four, so tier-2 Science is the part of the tag they see most (046: Develop's draws now reach
the tier-1 deck too).

**Placing presence.** Every placement now pays its track step, so a placing card's price is its own
plus the step's. Pylon Drop gains its energy first, so the card can pay a cheap step. Tier-1 cards pay;
the three tier-2 placements (Beamed Power Constellation, Interconnector Treaty, Aegis Grid Lockdown)
print "free": late steps cost 2-4 energy each, and they land when only late steps are left.

## 034's changes (r19)

Ignoring Science or Industry cost a planning agent nothing: Industry paid over rounds and
the wins come in rounds 6-8, so its payback had no time to land. As in Terraforming Mars,
the two tags now pay in the deciding rounds: Science through cheap cards and tag
requirements, Industry through money up front plus income. These prices are candidates, screened in
[034's gate](../experiments/results/screen-034-clocks/clocks.md): ignoring Industry now costs 12
points at c2 (level at c3), and ignoring Science is still ahead (-16 / -9). 034 also moved the
ceilings in *The credit budget*: 4 credits for 1 energy (the buy-energy project, gone in 066), and a token by credits was the rich action,
6 and rising, from stage II.

**The sum.** Value = immediate gain + income × rounds left. Aim: payback in 3-4 rounds
when played in rounds 2-3. Payback here is (credits + play insight − immediate gain) ÷
value a round, with 1 insight = 4 credits, 1 chip = 1.5, 1 Capital or Dataset about 1, a
drawn tier-1 card = 1 insight. The keep (1 insight) comes on top for every card, so it
is left out: it adds about 4 ÷ value a round.

**Industry: immediate chips or credits plus income.**

| card | was | r19 | payback |
|---|---|---|---|
| Payroll Automation | 4 cr, +1 income | 3 cr, +1 income, 2 chips | (3 + 4 − 3) ÷ 1 = 4 (was 8) |
| Robotic Foundry | 7 cr, 1 chip, insight 2 | 7 cr, 1 chip, insight 1 | 12.5 ÷ 3 = 4.2 (was 5.5), under 4 once the building discount applies |
| Grid-Tied Smelter | 5 cr | 4 cr, gain 2 credits | 6 ÷ 2 Power tags = 3 (was 4.5); 6 on its own tag alone |
| Bonded Warehouse | 5 cr | 5 cr, gain 2 chips | 6 ÷ about 2 = 3-4 |
| Expansion Fund | 3 cr | 3 cr, gain 2 credits | 5 up front; the factory still comes 3 rounds later |
| Supply Chain Treaty | 7 cr | 4 cr | 4 for the table (it gives 3 away; the move is worth about 1) |

Four of the six are buildings at 3-7 credits, which is where the core's separate Industry
discount on buildings (every N Industry tags, floor 1) pays; it is not priced in here.
Bridge Loan, Lights-Out Assembly and Retained Earnings
already paid fast and are unchanged; so is tier 2, which already repays in 3-4 rounds
(Full Automation Mandate 3.2, Arcology Foundry 3.6).

**Science: cheap cards with a small gain now, and requirements that unlock the strong
ones.** A tag in play comes cheaply from the 0-insight programs (Literature Crawler,
Benchmark Suite (now Stress Test Suite), Cryo-Compute Burn, Field Sensor Grid). Events don't leave a tag.

| card | was | r19 | why |
|---|---|---|---|
| Grad Student Pool | 2 cr | 3 cr, starts with 2 Datasets | a draw at once, then one every 2 rounds; (3 + 4 − 4) ÷ 2 = 1.5 |
| Simulation Cluster | 5 cr, 1 chip | 3 cr, 1 chip, draw 1 | 3 credits a draw stays; (3 + 1.5 + 4 − 4) ÷ 1 = 4.5 (was 10.5) |
| Citation Engine | 5 cr | 4 cr, needs 1 Science | pays 2 from its first round: 8 ÷ 2 = 4 |
| Synthetic Data Forge | 5 cr | 4 cr, needs 1 Science | 1 insight a round for 8 credits: 2 rounds plus the 2 to fill it |
| Endowed Research Chair | 5 cr | same, needs 1 Science | already the strongest tier-1 Science income, so it takes the gate rather than a price cut |
| Breakthrough Leak | 3 cr | 2 cr, needs 2 Science | draw 3 is the tag's payoff for committing |

Only these 4 of the 18 tier-1 Science cards carry a requirement, at most 2 tags, so a seat
without Science loses those cards and nothing else: no tag becomes mandatory. The core's separate Science discount on insight costs (tier-2 keeps and card
plays, floor 1) is not priced in.

**The economy.** The twelve changes add about 31 credit-equivalents if one game played them
all (Industry 19, Science 12), against about 800 over a game (3 seats × 8 rounds × about
12 credits and 5-6 insight). That is under 4% and only a rough sum, before the core's two
discounts and any compounding, so the 10% ceiling needs a measured check. Chip Spot Market
pays 2 credits a chip, so Payroll's and the Warehouse's 2 chips are worth up to 4 credits
to a seat holding it, as Bulk Silicon Procurement's already were.

## 024's changes (old rates)

- **Freeing is no longer a card's effect.** A corrupted city frees when one player holds
  `free_city_at` of their ambassadors on it and no adversary ambassador is on or next to it
  (rules.md §2). Cards help meet that: Amnesty Decree (4 cr, clear 2 around a city, a move)
  and Liberation Accords (9 cr, clear them all, 2 ambassadors). Back-Channel Talks and
  Station Prefab Order (now Reclamation Crew) now take cubes off stations and factories only. **Great Liberation
  is the one card that frees a city outright** (14 cr, 1 antimatter, tier 2; up to 2 sites
  in a country; was 10 cr, up to 3).
- **The credit budget.** A round's income is about 12 a seat. A threat counter is a third to
  a half of that (4-6 in tier 1, 6-8 in tier 2), so a seat countering its country's threat
  has about half its income left for cards. The standard projects (rules.md §6) set a
  ceiling on what a credit buys: 3 credits for 1 energy or a presence or a hardening token,
  5 for 3 compute drained, 6 for an ambassador. A card should beat those rates by half or
  more, or it's a bad buy.
- **Card text.** Generated from ops since 033 (rules.md, *The card grammar*). 024's
  audit against the JSON corrected Hyperscale Botnet (said 3 Cyber, needs 2), the factory
  cards (a free industrial zone in reach, not "an open hex touching a city") and the give
  cards' bonus (only if you gave any).

## The yardsticks (023, r14: old rates, see Current rates)

- **The budget.** A board makes 5 insight a round from round 1 (4 printed, +1 on the first
  presence space, uncovered at setup), 6 once the track's 4th space is uncovered, and
  about 9-10 credits plus buildings. Four cards are drafted a round. Keeps cost 1 insight
  (tier 1) or 3 (tier 2); credits buy keep insight only, at 5 per insight. A round's
  insight is roughly "keep 2, play 3", or "keep a tier-2 card and play two".
- **Exchange rates used for pricing:** 1 insight is worth 3-5 credits (5 when it buys a
  keep); a drawn tier-1 card about 1 insight (it is blind, not chosen); a point of
  income about 3-4 credits on a tier-1 card (Payroll Automation 4 for +1 credit, Baseload
  Contract 4 for +1 energy, Tidal Barrage 5 for +2 energy); a Dataset, Capital, Follower or Agent about 1 credit;
  a lowered point of income 3 credits (the keep plan's rate, 022).
- **The clock.** Games run 8-10 rounds; most wins end in rounds 6-8. A tier-1 income
  card bought in round 2 earns for 5-7 rounds, so a 3-5 round payback is fair. Tier-2
  cards land in rounds 4-7 and earn for 2-4 rounds, so **tier-2 slow income must be
  extremely strong** (Arcology Foundry-class: +5 a round, or several resources at once),
  and tier-2 one-shots must swing the board now (country sweeps, project discards,
  node removal, Unity tips).
- **Insight to play:** 0 for small one-offs worth 1-3 credits and cheap fillers whose
  real value is the tag left in play; 1 by default (a building, a program that stays,
  a removal event); 2 for big effects and nearly all tier 2. Events are not
  automatically 0 (022: free removal events made keep-all beat the engine).
- **Other costs follow the tag:** Science and Cyber pay insight; Industry pays
  chips and lowered income; Power pays energy and credits; Diplomacy credits;
  Security credits and energy. Antimatter appears only on tier-2 ultimates (8 cards,
  1-2 a tag). Strong income cards carry an income cut.
- **Tokens.** Science's Datasets, Industry's Capital, Diplomacy's Followers and
  Security's Agents. Cards that hold them gain them each upkeep; abilities spend them
  at a rate; tier-2 cards count them (`gain_per_tokens`, the "count as power" pattern,
  built in 023) or cash them out at a growing rate (`cash_out`: 1 + 2 + ... + n).
- **New primitives (023):** `draw` (Science's card draw, tier-1 deck, no keep paid),
  `drain_all` (Cyber's "every project −X"), `gain_per_tokens`, `cash_out`. Not built,
  so not on any card: a keep discount, turning an adversary ambassador, permanent extra reach,
  presence on any hex, spending an ambassador as a cost.

The per-tag reasoning below was written with the cards, one designer per tag, and
edited after the screen where a card changed.

## Science (023, r14: old rates)

Yardsticks: a tier-1 keep costs 1 insight (5 credits when short), so **one drawn tier-1 card ~ 1 insight ~ 3-5 credits** (cheaper than a keep because it is blind, not chosen). Players make 5-6 insight and ~9-10 credits a round over an 8-10 round game. A tier-1 income point costs ~3-4 credits (Payroll Automation 4, Baseload Contract 4).

- **Small draw and filler programs (rounds 1-4).** Literature Crawler (1 cr, 0 insight, draw 1 + peek 1) and Cryo-Compute Burn (energy into insight 1:1) are cheaper than the keep they replace, so they cost 0 insight; their real payoff is the Science tag left in play for Citation Engine, the tier-2 `needs` and AI-board powers.
- **Insight converters.** Field Sensor Grid (3 cr for up to 2 insight) and Benchmark Suite (now Stress Test Suite; 2 cr, 1 insight + 2 Datasets) price insight at ~1.5-2 credits, well under the 5:1 keep rate, which is the tag's identity; capped at 2 so they never pay more than half a round's insight.
- **Draw-for-insight cards cost 1 insight.** Breakthrough Leak (draw 3) nets +2 cards; the hacks with a draw rider (Neural Cartographer, Zero-Knowledge Exploit, Red-Team Audit) pay 1 insight because the draw replaces it and the hack is the real effect (Zero-Knowledge Exploit drains only 2, against Flash Exploit's 5).
- **Help cards** (Open Preprint, Shared Compute Grant, Open-Weights Release): the helper pays credits and insight and draws; the other player gets the insight. Net table gain is +1 insight and a card, so they cost about what a draw costs.
- **Dataset engines (rounds 2-5).** One Dataset ~ 1 credit. Telemetry Archive and Grad Student Pool yield one insight/card every 2 rounds: they earn their 2-3 credits + 1 insight back in ~4 rounds, so they are round-2-to-4 buys. Synthetic Data Forge (2 a round, 5 cr) is the faster battery. Every Dataset spent is one the tier-2 counters no longer see: that is the tension.
- **Endowed Research Chair** (5 cr, 1 insight, credit income -1): +1 insight a round and draw 1. Played in round 3 it returns ~6 insight (~25 credit-equivalent at the keep rate) for ~13 credits of cost and lost income; played after round 6 it barely breaks even, which is the intended window.
- **Simulation Cluster** (5 cr + 1 chip): 3 credits a draw is a premium over a keep's insight, but it never runs out; a mid-game credit sink.
- **Citation Engine** (5 cr): +1 credit per Science tag (max 3); with 2 tags it beats Payroll Automation by round 5.

Tier 2 lands in rounds 5-8, so it pays back in 2-4 rounds or not at all:

- **Exabyte Archive** (8 cr, 1 chip, 2 insight, over a Science building): +2 Datasets on build, +2 a round, and a free ability worth 1 credit per 2 Datasets (max 4). With one other Dataset card it reaches 4 a round in two rounds: ~10-12 credits over 3 rounds, at a lower price than Arcology Foundry because it takes a Science slot and the Datasets must be kept, not spent.
- **Orbital Observatory Array** (9 cr, 1 chip, needs 1 Cyber): +1-2 insight income and 2 cards every other round; the Cyber-hook building.
- **Data Licensing Deal** (0 cr, 1 insight): one-shot cash-out of a Dataset pile, up to 8 credits + 2 insight, kept tokens. Cheap because its value is entirely what you already built; the tier-2 keep (3 insight) is its real price.
- **Ultimates carry antimatter (1 each)**: Recursive Self-Improvement (10 cr, -1 credit income, +2 insight production, draw 3) and Foundation Model Campus (15 cr, 2 chips, needs 3 Science: +2 insight income, +1 credit per Science tag max 6, draw 2). From round 6 each must return ~10 insight-equivalents in 3 rounds to beat their keep + play cost, hence numbers above any tier-1 rate.
- **Adversarial Inference Sweep** (7 cr, 2 insight): drain_all 1 is a Cyber primitive borrowed at half strength; the draw 2 is the Science half.
- No keep-discount primitive exists. Predictive Modeling and Open-Weights Release approximated one with `next_round_insight` 1 (every player) until 041 changed them (*041's changes*, above); no card prints it now.

## Industry (023, r14: old rates)

Baseline: 20 starting credits, ~9-10 credits and 5-6 insight a round, games of 8-10
rounds. One permanent +1 credit is worth about (rounds left) credits, so ~8 if played
in round 2 and ~3 in round 7. Prices are against Payroll Automation (4 for +1 credit),
Bridge Loan (0 for 6 credits and -1 income) and Arcology Foundry (16 + 3 chips for +5
credits +2 chips).

- **Tier-1 income (Payroll Automation 4, Supply Chain Treaty 7, Grid-Tied Smelter 5,
  Bonded Warehouse 5):** earns its cost by round 4-5 when played in rounds 1-3; a dead
  draft pick after round 6. Insight 1: the default price.
- **Robotic Foundry (7 + 1 chip, insight 2):** +3 credit income is the strongest
  tier-1 income, so it carries the chip cost and the second insight.
- **Instant gains (Bridge Loan, Bulk Silicon Procurement, Chip Spot Market, Station
  Prefab Order, now Reclamation Crew):** small, any-round tempo cards; insight 0 where the gain is about 3-5
  credits' worth. Bridge Loan's 6 credits are paid back by its -1 income in an early
  round and are nearly free in round 8+, which is fine for a cheap tempo card.
- **Factories (Prefab Plant Kit 3, Expansion Fund 3):** priced at about 2 for the factory
  plus the rider; a factory pays 1 chip on production steps and a site, so these are
  best in rounds 2-5.
- **Capital engines (Retained Earnings 2/0, Expansion Fund, Bonded Warehouse, Sovereign
  Capital Fund 2 a round):** tokens are worth little alone; their value is the cash-out.
  cash_out pays n(n+1)/2, so 4 Capital is 10 credits and 6 is 21 (Liquidity Event caps at
  5, Megacorp IPO at 6). Holding two engines
  from round 3 and cashing out once in round 7 is the intended arc; Materials Hedge Desk (3, 033: one-time) cashes out for chips instead.
- **Liquidity Event (0, insight 1)** and **Megacorp IPO (3, insight 2, +2 Capital first):**
  the insight is the gate, not the credits, because the payout scales with how long you
  saved. IPO's +2 Capital adds 2n+3 credits on top of an n-token cash-out.
- **Collateralized Grid (3, 033: one-time, its 1-credit use folded into the price):** the
  "count as power" pattern; up to 3 energy once, with 6 Capital held, tokens kept.
- **Help cards (Joint Fabrication Pact, Supply Chain Treaty, Vendor Financing):** the
  helper pays credits and gets a little less than the receiver; Vendor Financing's
  2 Capital is the helper's gain and needs a Capital card to be playable.
- **Tier 2 (rounds 5-9, ~3-5 incomes left):** slow income must pay back in 3 rounds.
  Full Automation Mandate (12, +4 credit +1 chip production, insight 1) repays by its
  third round; Arcology Foundry (16 + 3 chips, +5/+2) is the tag's big income card
  short of the ultimate; Arc Furnace Complex
  turns 2 energy into 6 credits a round for a Power-heavy player.
- **Orbital Mass Driver (25, 1 antimatter, 4 chips):** the tag's one ultimate: +7
  credits, +2 chips and a 2-credit building discount; ~4 incomes left make it ~40
  credits of value, so the 25 is the brake. The only antimatter cost in the tag.
- **Asteroid Salvage Rights (5, insight 2):** a big one-off (10 credits, 4 chips,
  1 antimatter) that is equally good in any late round; its cost is the tier-2 keep (3
  insight) plus the play insight.

## Cyber (023, r14: old rates)

Cyber has no token. Its timing comes from holding a hack until a project is nearly full, so
the value of a drain is the project it stops, not the raw compute. A cheap project is 15-18
compute at 3 players and the 3 starting nodes make 9 a round, more as nodes form, so 5-8
compute is roughly half to one round of adversary progress: enough to stop a project that would resolve this build.

- **Rate.** About 1.5-2 credit-equivalents per compute (1 insight ~ 5 credits). Flash Exploit
  (3 cr + 1 insight, 5 compute) and Compute Siphon (4 cr + 1 insight, 6) sit at the cheap end because a
  single-target drain is wasted if no project is close; Zero-Click Breach (5 cr + 2 insight,
  8 plus a cut) pays a premium for the one burst big enough to stop a full stage-1 project.
- **Cut node** is worth `node_compute` (3) compute and is the cheap filler: Blackout Script and
  Rootkit Cache are 0 insight, and Rootkit's 2 credits a use is a fair rate for 3 compute a round.
- **Expel (damage)** is priced against Breach and Clear (3 cr + 1 energy + 1 insight for 2 worth
  in reach): Eviction Worm is a program (tag stays) at 3 cr + 1 insight.
- **Repeatable hacks** (Intrusion Lab, Honeypot Relay, Sentinel Mesh) earn their cost from
  round 3-4 on: 4-6 uses over the game. Bought in round 7+ they don't pay back; that is fine,
  the cheap one-shots cover late play.
- **Insight.** 0 for small hacks and the help kit (they must be casual plays), 1 for removal
  and standard drains, 2 for the 8-compute burst and most tier-2 cards. Insight is the cost
  that competes with keeps, so it is what stops a player from chaining hacks every round.
- **Help cards** (Shared Exploit Kit, Mesh Intrusion Pact): the helper pays, the target gains
  1 insight or 2 credits, the helper gets a small refund plus the shared drain.
- **Tier 2** lands in rounds 5-8 with 2-4 rounds left. Discard (Killswitch Directive, 10 cr +
  2 insight) removes 15-30 compute and a whole effect, so it is the dearest one-shot without
  antimatter. Node Severance (6 cr + 2 insight) is cheaper: a node is 3 compute a round for
  the rest of the game plus the Unity sway. drain_all 5 (Cascade Failure, 9 cr) is up to 15.
- **Ultimate.** Singularity Exploit is the only antimatter card (1): node removal plus
  drain_all 4, at 14 cr + 2 insight.
- **Tier-2 income** must pay back in ~3 rounds: Hyperscale Botnet (13 cr, needs 2 Cyber,
  built over a Cyber building) yields 4-6 credits and 1 insight a round, ~20-25 credits'
  worth by game end. Black Ice Citadel is an upgrade path for Intrusion Lab (4 compute per
  energy).
- **Income cut.** Neural Mirror's permanent +1 insight costs 1 credit income: insight is the
  scarcer resource, so the swap is the reward.

## Power (023, r14: old rates)

Anchors: Baseload Contract 4 credits for +1 energy production, Capacitor Discharge 0 for 2 energy and 1 credit, Pylon Drop 1 for 1 presence and 1 energy, and the standard actions: 1 energy to place a presence, 1 energy for each hardening token (Protector only).

- **Exchange rates.** Energy is worth about 1 credit: the Builder's discount rate, and the hardening and presence actions price it there. A presence is worth more than 1 energy, because each one uncovers a track space that pays every round (insight, credits, energy, and finally antimatter). The tracks have only 9 spaces (6 + 3), though, so the presence cards in this tag place 1 to 3 at a time, never more.
- **Tier-1 income (rounds 1-4, 6-9 rounds of payoff).** +1 energy production costs 4 as a program (Baseload Contract), which takes no slot and can't be built over. A building pays for itself by round 3-4: Tidal Barrage costs 5 for +2. Peaker Turbines gives +2 energy production for 5 credits and lowers your credit income by 1, a net gain of about 1 a round. It is the strong income card, and the income cut keeps it from being an automatic keep.
- **Presence cards earn their cost at once.** Pylon Drop (1 credit, 0 insight, 1 presence + 1 energy) is barely better than the standard action, and that is on purpose: it is a filler card, also worth playing for its tag. Relay Mast Rollout (2 presence for 2 credits) is worth playing early, while track spaces still have many rounds left to pay out.
- **Hardening is cheap but conditional.** A token only lands on a station or factory a troop can strike, so harden cards cost 0-1 credits and usually 1 energy, which is the standard action's price paid once for several tokens. They are worth it in war rounds and dead otherwise, so the 0-insight ones stay cheap enough to hold on to.
- **Tier 2 (rounds 5+, 3-5 rounds left).** Slow income has to pay back in about 3 rounds, so it is priced well above tier-1 rates. Dyson Swarm Tranche gives +5 energy and +2 credits a round (about 7) for 16 credits, 3 energy, 2 chips and 1 antimatter, and it has to be built over a Power building. Continental Supergrid gives up to +5 energy for 10. Antimatter output, the scarce tier-2 resource, is priced at about 1 antimatter a round for 8-11 credits: Antiproton Offtake Deal (also 1 credit of income), and Positron Storage Ring, a 3:1 conversion that needs a Power building to replace.
- **One-shot tier-2 cards pay back at once.** Beamed Power Constellation places 3 presence and then pays energy for each presence you have, so it is worth playing even in round 8. Interconnector Treaty's +2 energy production is backed by an immediate gift and a presence, so it is not dead late.
- **Insight.** It is set at 0 for one-shot fillers and helps that net you little (Pylon Drop, Capacitor Discharge, Surplus Auction, Shared Grid Accord, the cheap harden cards). It is 1 for anything that lasts (production, buildings, abilities) and for Storm Hardening, a strong event that blocks a destruction. Every tier-2 card costs 2. With 5-6 insight a round, shared with keeps, a 1-insight income card effectively costs a second card.
- **Energy as a cost.** The tag's own currency gates its strong plays: helps and harden cards spend energy, and most tier-2 cards cost 2-3 energy. That keeps a Power engine from being only credits in disguise, and it makes Power's income matter to Power's own cards.
- **Antimatter as a cost** appears only on the ultimate, Dyson Swarm Tranche. Aegis Grid Lockdown (a shelter, 4 hardening tokens and 2 presence in one event) pays credits and energy instead.
- **Not used:** the four new ops. Power has no token, `draw` belongs to Science, and `drain_all` and `cash_out` belong to other tags. `gain_per_presence` already covers Power's version of "count it as power".
- **Help cards** (Shared Grid Accord, Brownout Relief, Interconnector Treaty): the helper pays and still gets something (credits, a presence, production), and the other player gets more energy than the helper spent. That is the Power-Diplomacy hook.

## Diplomacy (023, r14: old rates)

Baseline: 20 starting credits, ~9-10 credits and 5-6 insight a round, 8-10 rounds. Freeing a corrupted city sways 4 Unity and takes one ambassador once the adversary's around it are cleared; we price 1 ambassador at about 1.5-2 credits, a Move-2 rider at about 1, Consolidate 3 at about 2, 1 Unity at about 1.5-2.

- **Insight 0 (6 cards)**: the cheap round-1 to round-3 tools (Consular Pouch, Shuttle Diplomacy, Peace Dividend, Peacekeeping Detail, Mutual Aid Pact, Manifesto Leak). Each is one or two small effects; charging insight would crowd out the keeps, and they are what a Diplomacy player fills a turn with.
- **Insight 1 (12 cards)**: everything that persists (token buildings, buildings) or does two real things (Summit Caucus, Ceasefire Monitors, Amnesty Decree). Amnesty Decree is an event but a removal (it clears a city), so it pays 1 like other removal events.
- **Insight 2 (all 7 tier 2)**: tier-2 effects change the board state (clear or free cities, 3-4 Unity, spread) and should compete with a tier-2 keep (3 insight).
- **Token buildings** (033: programs before; Liaison Corps, Open Assembly, Commons Grid) earn their cost only if played by round 3-4: 1 Follower a round means the first spend lands two rounds later, so 4-5 spends a game at most. Priced 2-4 credits. Manifesto Leak (1 credit, +3 Followers) is the catch-up that makes a late token card fire immediately.
- **Commons Grid** is the "count as power" pattern: Followers stay, so it rewards stacking several Follower cards; the 1-credit activation keeps it from being free energy, max 3 caps it.
- **Tier-1 buildings** (4-6 credits) follow the tier-1 income rate: Treaty Grid Exchange is +1 energy +1 credit at 6 (a slot and three tags); Consulate Tower scales with Diplomacy tags, max 3, so it is only strong in a committed build. Embassy Compound pays for its 2 ambassadors up front and sells movement at 2 credits a use.
- **Help cards** (4): the helper pays (credits, or the card and its insight); the receiver gets slightly more than the helper spends (Mutual Aid: 3 credits for 4 plus an ambassador to you; Development Loan gives a +2 bonus on up to 6). Each gives the helper something back (ambassador, hardening, Unity) so playing one is never a pure gift. Coalition Charter's 3 Unity helps everyone.
- **Tier 2 arrives around round 5-6, so 3-4 rounds remain.** One-shots must pay off now: Liberation Accords (9 credits, needs 1 Diplomacy) clears one city in reach and gives the 2 ambassadors to free it, ~4 Unity; Great Liberation frees up to 2 in one country (~9 Unity) and so is an ultimate carrying 1 antimatter. Planetary Referendum (4 credits, 4 Unity, no needs) is the plain Unity event.
- **Slow tier-2 income has to be extremely strong.** Planetary Assembly gets 2 Followers a round and spends 2 for 2 Unity plus an ambassador every round: ~6-8 Unity and 3-4 ambassadors over the remaining rounds, for 12 credits, 1 chip, a credit-income cut and building over a Diplomacy building. Orbital Embassy (+4 credits, +1 insight, 2 ambassadors) returns ~16 credits and 4 insight by game end against 14 credits, 2 chips and 1 antimatter, which is why it is the tag's second (and last) antimatter card.
- **Spread is rare**: only Exodus Corridors (7 credits, plus 2 ambassadors to spread with).
- No Diplomacy card spends ambassadors as a cost (there is no primitive for it); credits are the extra cost throughout, with chips and needs/replaces on the two tier-2 buildings.

## Security (023, r14: old rates)

Yardsticks: troop = 2 removal worth (`adversary.troop_health` 2); Breach and Clear pays 3 credits + 1 energy + 1 insight for 2 worth in reach, jam N takes N credits off a counter, Threat Neutralization Mandate pays 7 credits + 2 energy + 2 insight for a counter. Income ~9-10 credits and 5-6 insight a round; an insight is worth ~5 credits (the keep conversion rate).

- **Agent engines (tier 1, rounds 1-4).** Sentry Drone Wing, Kill Team Roster, Safehouse Network, Agency Field Station gain 1 Agent a round, so they fire every 2-3 rounds: 3-5 uses in a 10-round game if played by round 3, about 1-2 if played after round 6. Priced 3-4 credits + 1 insight so they pay back after two uses. Kill Team Roster spends 3 for a troop (2 worth), the largest spend for the most removal, which is why it carries the extra 1 energy.
- **Recruit Operatives (1, 0 insight)** adds 3 Agents: one extra firing, only playable with an Agent card out.
- **Counting Agents as power.** Agency Field Station caps at 3 credits for 1 energy a round and only reaches it at 6 hoarded Agents; spending Agents on removal costs income, which is the tension. Deep State Ledger (tier 2) pays 1 per Agent up to 5: from round 6 with ~3-5 Agents in play that is ~4 net credits a round for 4 rounds, ~16 credits back on 8 credits + 2 insight. Tier-2 income has to be this strong because it only has 3-4 rounds to run.
- **Removal (tier 1).** Breach and Clear removes 2 worth in reach, plus 2 credits on a troop kill, so 3 credits + 1 energy + 1 insight (removal events cost 1 insight). Peacekeeper Rotation and Diplomatic Expulsion remove 1 worth (an ambassador only), so they are 2 credits; the rider (move 2 / 1 Unity) is the reason to take them, and Expulsion's insight is 0 because it is limited to presence countries.
- **Interceptor Battery (5 credits)** pays 3 energy for a troop in reach every round. It earns from round 3 on, when troops are moving; 3 energy a use keeps it from replacing removal cards.
- **Threat counters.** Every counter is 4-8 credits, so a jam takes its amount in credits off one. Threat Triage (jam 2) costs 4 credits + 1 insight. Intrusion Countermeasures (drain 3 + jam 1) and Shared Threat Intel (jam 1, and it gives 1 insight away) stay cheap because they are halves of a counter.
- **Protection.** Hardpoint Lockdown (shelter + harden 2) is a 0-insight event: it only matters in a war round, so it is a reaction card. Hardened Bunker Complex is +1 energy income (about 3) plus the Protector's Harden action, so 5.
- **Security Contractor HQ** is tag-scaled credit income: 4 credits, max 3, needs 6 Security tags to cap, so it is a round 2-4 buy.
- **Help cards.** Mutual Defense Pact (3 credits: harden 2, the partner gets 2 credits, you 1 energy) and Shared Threat Intel: the helper pays roughly what they gain plus the gift.
- **Tier 2 sweeps.** Kinetic Interdiction (damage 3 in every hex of a country: a troop and an ambassador per hex) is the ultimate: 6 credits + 1 antimatter + 2 insight, the tag's only antimatter card. Total Containment (destroy_all + shelter) pays credits and 2 energy, not antimatter, so 9 credits and needs 1 Security. Martial Cordon is a program (tags stay) with a move and 1 Unity per 3, so 8 credits at 1 insight.
- **Threat Neutralization Mandate** is a counter with no antimatter: 7 credits + 2 energy + 2 insight and needs 1 Security. A tier-2 threat uncountered costs a permanent loss plus 3-4 compute, worth 10+ credits late.
- **Agency Directorate (12 credits, 2 chips, built over Security, needs 2)**: 2 Agents a round fires a 2-worth country sweep every other round, about 2 sweeps in the rounds left. Aegis Dome Array bundles Firewall and Civic shelter with a harden ability: 10 credits + 2 energy + 1 chip, worth it only where a war front sits near your buildings.


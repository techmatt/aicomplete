# Rules history

The resource called hardware in older rows is **Chips** since item 026 (name only).

What each rules revision changed, what it replaced and whether it is **locked** or a
**candidate**, newest first. [rules.md](rules.md) is the current rules; this file is the
record, moved out of it in 025 otherwise as written, so a row quoting a number or a card
is as of its revision. Add a table here with every rules change.

**Revision 31 (075): the Researcher, the open board rulings, the corps amended.** (Bumped from the constant, 30.) Design: Matt, 2026-11-02 (the brief, and amendment #3106 on the corps). Locked: every board power once a round; powers paid by sacrificing played cards; no rule exceptions without an extremely good reason; a power's cost names N of one tag or N of any tag, never a mix (Matt, 2026-11-02). Everything else a **candidate**. Two countries seat Builder and Protector; three add the Researcher. The band: planner, d2, fresh seeds 25900 and 26000, 96 a cell each, pooled to 192, frozen build: **66.7 ±3.4 / 79.2 ±2.9** (r30: 12.5 / 7.3): c2 in band, **c3 4.2 points over 75, 1.45 SE**, inside 2 SE, so no lever after the freeze; the cause, as measured: the three-seat game took more from the cheaper cards (+23 against +8) and the Researcher's Patience is worth 15.6 ±11.1 to it. The base band after the corp change alone (27600, 96): 27.1 / 35.4, under 074's 62 / 62 what-if because flavor payouts buy less than 10 credits (w1: 29 / 33 against 54 / 62.5 on the same seeds). The gate: planner, seeds 28200, gate-075 (62 rows) and gate-075-c3 (the Researcher's own checks), then 23 rows closed at 96 paired seeds: **no check ahead of the planner by more than 1 SE.** The nearest: c2 `tier2_rush` -3.1 ±6.0, c3 `never_second_presence` -1.0 ±5.8, `free_build_rush` 0.0 ±5.3, `always_buy_cubes` +2.1 ±6.3; c2 `home_only` led by 25 at 16 seeds and closed at +4.2 ±5.3. `hoard_insight=3` trails by 31 ±12 at 16. The gate's own planner row (96): 63.5 / 72.9. Reasoning, the screens and every read: [experiments/results/screen-075-researcher/researcher.md](../experiments/results/screen-075-researcher/researcher.md).

| Rule | Was | Status |
|---|---|---|
| The Researcher: Patience (unspent insight carries over, no cap); Fieldwork 1 energy → 3 cards; Grant 4 credits → 1 research cube on an incomplete megaproject in your color; Deploy; Peer Review 3 research cubes → 1 presence (no energy, the step still paid); Field Station (1 Industry) a free station in reach; Audit (1 Cyber) 3 compute off one project; Particle Run (3 Power) 1 antimatter; Science printed; 2 slots; 8 credits and 4 insight printed, 15 starting credits | new board | candidate (Matt's sheet; credits, insight and starting credits 075's) |
| Its tracks: economy 1 insight, 2 credits, a free station, 2 credits and Power, 1, 1, Open Archive (1 research cube a round); card-play 1 insight, 1 insight, keep upgrade and Science, 1 insight and Cyber, 1 insight, 1 credit and Industry, 1 insight and Open Science (2 insight a round from the supply to the other players, split as it chooses, in upkeep for the next round) | new | candidate, 075's call within the sheet |
| A track space may do something once when uncovered (`effects`, `Op::BuildStation`) | spaces only paid income, tags, the keep upgrade or a capstone | candidate |
| `ai.boards` 0, 1, 2; c3 seats the Researcher where it seated a second Builder; 4 players repeat the Builder | 0, 1 | candidate |
| Wall also stops a strike into a hex with the Protector's presence | it stopped entering only | candidate (Matt) |
| Rush Build sacrifices 3 Industry, Shelter 3 Security | 2 of the tag + 1 any | **locked** (the cost rule) |
| `Content::check` refuses a power that names two tags | none | follows the lock |
| Six research-cube cards: Field Grant, Sample Repository, Preprint Exchange, Inference Cluster, Breakthrough Lab, Policy Institute | new | candidate |
| Every tier-1 and tier-2 card 1 credit cheaper | as printed | candidate (075; the brief's lever was the other way: see the results) |
| Builder 13 printed credits, Protector 12 | 10, 9 | candidate |
| Builder economy space 5: +1 credit | blank | candidate (073's item) |
| Consulate Tower's Action: 4 credits → 1 ambassador | 1 credit | candidate |
| Megaprojects (#3106): no after line, an adversary cube after completion is lost | 2 compute, 1 compute on every project, 3 compute, 1 Discord | candidate (Matt) |
| Player payouts: Horizon 3 energy and 3 credits; Cintamani a drawn card and 5 credits; Onyx 4 chips and 3 credits; Biomia 1 Sway, a drawn card and 1 credit | 1 energy; 3 credits; 3 credits; a drawn card | candidate (Matt's 6-10 a cube) |
| A converter needs presence in its country | an aligned city | candidate (Matt) |
| Megaproject size 8 | 8 | 10 screened and rejected: completion moved to round 4.1-4.2, -6 to -19 points |
| Content overlay table `corps.<name>.project.<path>` | the map unreachable | not a rule |
| Eval: `patience` 1.0, `wall` 3, `research_cube_power` 1.2 | none | agents, not rules |
| Fixed policies `always_buy_cubes`, `hoard_insight`, `never_convert`; `ignore_threats` refuses automatic counters | none | agents, not rules |

**Revision 30 (074): research cubes and the four corps.** (Bumped from the constant, 29.) Design: Matt, 2026-11-02. Everything here is a **candidate** but the two lines marked locked. The band, read and not tuned: planner, d2, seeds 25700 and 25800, 96 a cell each, pooled to 192, frozen build: **12.5 ±2.4 / 7.3 ±1.9** against r29's 53.5 / 50. Structural causes, by what-ifs that changed one thing each: a player cube pays about 3 where it paid 10, and adversary cubes make compute every round after completion; both undone on other seeds reads close to r29. The card, the converters against *Current rates* and the use: [experiments/results/screen-074-corps/corps.md](../experiments/results/screen-074-corps/corps.md).

| Rule | Was | Status |
|---|---|---|
| Country = corp: one piece, one corp; the pieces take the corps' names (Horizon, Cintamani, Onyx, Biomia) | Coastal Republic, Northern Union, Highland Federation, Delta States | **locked** (Matt) |
| Every board power, sacrifice ones included, is once a round | as it stood | **locked** (Matt) |
| Setup: one neutral station a country; no player starts with a station | one corrupted station | candidate (Matt) |
| Neutral stations on hex 10, 2, 12, 9: next to the starting presence, clear of a node and a base in every layout | new | candidate, 074's call (first 5, 5, 12, 9: two stood next to a node at 3 and 4 countries) |
| Every station makes a research cube, in its owner's color, neutral ones too | neutral made none | candidate (Matt) |
| An adversary ambassador takes a neutral station's cube too, and is used up | it waited there | candidate: the rule as written, on a station that now produces |
| Completion is checked once, at the end of the production step, all colors filling the card; every cube pays, overflow included, and is spent | at the cube that filled it, per station | candidate (Matt) |
| Payouts per card; `project_credits` 10 and `project_discord_per` 1 go | 10 credits a player cube, 1 Discord an adversary cube | candidate (Matt) |
| After completion: cubes to the owner's board (`research_cubes`, a new resource kept between rounds); the adversary's convert at once | the card's benefit, 2 credits a round per owned station | candidate (Matt) |
| Converters: once a round per player per card, with an aligned city in the country; `Action::Convert` | new | candidate (Matt) |
| Megaproject size 8 | 7 | candidate (Matt) |
| A player ambassador on a corrupted station takes the cube onto their board after completion, as before it onto the card | it took the card's benefit | candidate, 074's call |
| `compute_every`: 1 compute on every visible project | new op | candidate (Matt) |
| Eval: `research_cube` 0.3, `station_round` 1, `station_spare` 1 | no such weights; `station` 5 for every owned station | agents, not rules |

**Revision 29 (070): threat sources, typed counters, tier-2 capstones.** (Bumped from the constant, 28.) Design: Matt, 2026-11-01; the target rule, Matt, 2026-11-02. Everything here is a **candidate**. The band: planner, d2, fresh seeds 25000, 96 a cell, on the frozen build, threat costs as printed: **61 ±5.0 / 57 ±5.0**, then a second fresh set, seeds 25600: **46 ±5.1 / 46 ±5.1**. With the gate's own planner row (53 / 47, 96 seeds) that pools to **53.5 ±2.9 / 50.0 ±2.9** on 288 frozen-build seeds a cell, none of which picked a lever: **the band (60-75) is missed by about 7-10 points.** Threat costs stay as printed. The cause, as far as it is measured: reach. Threats go uncountered 3 a game (r28: 0.3), each a Sway and its harm, and the planner wins by purge 41% of games (r28: 56% on 48). The gate: planner, seeds 25200, gate-070, 62 rows: no check ahead of the planner by more than 1 SE; 19 within 2 SE closed at 96 paired seeds, the nearest c3 `never_second_presence` -1.0 ±7.0 and c2 / c3 `always_max_presence` +3.1 ±6.4; `home_only` +11.5 ±5.1 / +18.8 ±6.0, `capstone_rush` +25 / +19.8, `no_counters` stopped at 12 seeds. Unspent: credits and energy flag on the planner, because a purge ends the game mid-round with that round's income in hand: winners hold 20.5 / 17.8 credits, losers 7.5 / 6.0, under the flag.

**How the target rule came.** Three builds on the same cards. Creation at the deal (the prompt's first text): 36 ±4.9 / 38 ±4.9, purge wins 12 of 192: a threat re-corrupted a freed city before the counter phase, and corrupted cities always count against the purge. A threat with no source set aside: 79 ±4.1 / 81 ±4.0, purge 140 of 192. The target rule: 53.5 / 50 pooled, purge 237 of 576 games, the planner countering every threat in 5.0 rounds of 8.4. Creation by type, off one at a time, 48 paired seeds a cell, c2 / c3 (base 50 / 58, purge 20 / 22): influence 50 / 58, purge 20 / 22; cyber 50 / 58, 20 / 22; economic 50 / 60, 20 / 23; military 52 / 56, 20 / 21. No type's creation costs measurably: the planner lets 0.5 a game happen, most of them troops.

| Rule | Status |
|---|---|
| Every threat prints a source type; its sources are all adversary pieces of that type in its country | candidate (Matt) |
| Sources: influence corrupted cities; cyber corrupted stations and nodes; economic adversary factories; military troops | candidate (Matt) |
| Who may counter: a source in reach, or their ambassador on one; replaces presence in the country | candidate (Matt) |
| A player's card that jams or counters reaches only threats they may counter; a Unity reward reaches any | candidate, 070's call |
| Countering doesn't consume the ambassador | candidate (Matt) |
| No source when dealt: it names a target, its source while it is up: the lowest clean piece of its type, neutral before player-owned; a site to build on; the lowest base not walled | candidate (Matt, 2026-11-02; neutral first and the Wall are 070's call) |
| Countered, nothing is created; uncountered, the target becomes a source as the threat executes, before its effect | candidate (Matt, 2026-11-02) |
| At most one creation a country a round; nothing is set aside for lack of a source; with nothing to name, nobody can pay and it makes nothing | candidate (Matt; the last is 070's call) |
| `uncreated`: a screen's switch, 0 in play, for the creation-off read | not a rule |
| A threat whose sources have left can't be paid and still executes | Default (070; question default) |
| Shield: pieces of its type in its country can't be destroyed or freed while it is face up | candidate (Matt) |
| Extra threats reshuffle an empty deck as the round's deal does: that, not deck size, made every short deal | candidate, 070's call |
| Tier-2 threat deck 10 to 12: Asset Freeze, Deepfake Wave (7 credits) | candidate |
| Typed counters: a jam can name a source type; 12 tier-1 cards | candidate |
| `project_drain` and `auto_counter`: standing ops on tier-2 capstones | candidate |
| Retired: Threat Neutralization Mandate, Cascade Failure, Mesh Intrusion Pact, Full Automation Mandate, Asteroid Salvage Rights | candidate |
| Black Ice Citadel: was an Action, 6 credits, 1 energy, drain 2 from each project; now 7 credits, needs 1 Cyber, standing drain 1 | candidate |

**Source types (24 threats; no text changed).** Influence: Agitators, Defection, Smear Campaign, Overrun, Insurrection, Deepfake Wave. Cyber: Blackout, Sabotage, Infiltration, Station Seizure, Sleeper Cells, Purge Protocols. Economic: Requisition, Commandeer, Supply Raid, Embargo, Mass Production, Conscription, Asset Freeze. Military: Orbital Strike, Eviction, Total War, Siege, Manhunt.

**Typed counters.** Each counts as more than its cost toward its type. Military (Security): Rapid Response 4 for 2 credits, Force Protection Order 5 for 3 and 1 insight, Garrison Command, a building at 3 and 1 insight, Action, 1 energy: 3. The same shape: influence (Diplomacy) Fact-Check Network, Truth Commission, Civic Watch; cyber (Cyber) Incident Response, Threat Hunt, Security Ops Floor; economic (Industry) Antitrust Action, Supply Chain Audit, Market Watchdog. The deck's shape holds: tier 1 is 68 programs, 38 buildings, 14 events (60 / 34 / 14).

| Capstone | Tag | Does | Price | Why |
|---|---|---|---|---|
| Foundation Model Campus (kept) | Science | 2 insight; 1 credit per Science tag | 8, antimatter, 2 chips, 2 insight, needs 3 | the existing end of the engine |
| Vertical Monopoly | Industry | 1 credit per Industry tag | 10, 2 insight, needs 2 | capped by tags, which grow only by cards played |
| Market Stabilization Board | Industry | economic threats at home need 4 less | 5, 1 insight, needs 1 | at 9-12 it was never played: it fires about 0.3 times a round |
| Hyperscale Botnet (kept) | Cyber | 1 insight; 1 credit per Cyber tag | 13, 2 insight, 1 chip, needs 2 | the existing end of the engine |
| Black Ice Citadel (reworked) | Cyber | each upkeep, 1 compute off each project | 7, 2 insight, 1 chip, needs 1 | N 1, so projects finish slowed |
| Autonomous Defense Grid | Cyber | cyber threats at home need 4 less | 5, 1 insight, needs 1 | as the Board |
| Continental Supergrid (kept) | Power | 1 energy per Power tag | 11, 2 insight, needs 2 | the existing end of the engine |
| Civic Trust Bureau | Diplomacy | influence threats at home need 4 less | 5, 1 insight, needs 1 | as the Board |
| Sentinel Command | Security | military threats at home need 4 less | 5, 1 insight, needs 1 | as the Board |

**Loops.** Income per tag: tags grow only by cards played; 0.07 Vertical Monopoly played a game. Automatic counters: N 4 is under a typical cost, one copy a type at one seat's home holds them to one per country and type, and they save 4.7 credits a game. Project drain: N 1, 0.01 played a game; no project was stopped. **Ambassadors on sources are not learned:** payments by ambassador alone run 0.03 / 0.06 a game for the planner and at most 0.17 for any check (`threats.paid_by_ambassador`, 2,604 games). The standard Move reaches only hexes in reach, where an ambassador adds nothing, so only the five `move_one` cards stretch, and a seat holds about 0.1 ambassadors. A keep-plan bonus for those cards, `plan_stretch` 8, a what-if not shipped: 0.00 / 0.02, wins 30 and 27 of 48 against 31 and 28. Typed counters are little played: 0.27 / 0.31 a game for the planner, 0.33 / 0.48 for the engine; capstones 0.89 / 1.43 and 0.94 / 1.62.

**Revision 28 (068): Builder and Protector boards, ambassadors that stay, scarcity.** (Bumped from the constant, 27.) Design: Matt,
2026-11-01. The band: planner, d2, fresh seeds 22100, 96 a cell: **73 ±4.5 / 75 ±4.4** (target 60-75), on the frozen build. The gate: planner, seeds 23000, no check ahead of the planner by more than 1 SE: 18 rows within 2 SE at 96 paired seeds, the other 38 clearly behind early, `never_tag` dropped. Credits and energy end high; the cause is in the results. Reasoning, tracks and the screens: `experiments/results/screen-068-boards/boards.md`. Every row is a **candidate** unless marked.

| Rule | Status |
|---|---|
| Builder: Breadth, 5 building slots (boards default to 3); `ai.slots` 5, 3 | candidate (Matt) |
| Protector: Wall, adversary troops can't enter a hex with its presence | candidate (Matt) |
| Wall's reach: marches and landings; a strike from next door stands; a troop already on the hex walks off; a factory's own troop isn't entering | candidate, 068's call |
| Conversions in the power menu: Mint 3 credits for 1 chip; Bulwark 1 energy, harden 1 facility | candidate (Matt; Mint's rate, simulation) |
| New cheap powers: Overcharge (1 Power, 2 energy), Recovery (1 Security, clear a marker in reach, for its reward) | candidate (Matt) |
| New expensive powers: Rush Build (2 Industry + 1 any: a building from hand, no credits, other costs paid), Shelter (2 Security + 1 any: one country this round) | candidate (Matt) |
| Sacrifice tags are costs paid first; a tag sacrificed for Rush Build no longer counts for the building's requirement | candidate, 068's call |
| Two tracks per board, economy and card-play, 7 spaces each, each ending in a capstone | candidate (Matt); contents are draft candidates |
| Capstones: Full Capacity, Master Plan (Builder); Sanctuary, Aegis Doctrine (Protector) | draft candidates |
| Printed and revealed tags count for card requirements only | candidate (Matt; "only" is 068's call) |
| Late track spaces: step costs to 5; no antimatter on the tracks | candidate; one payment path stays **locked** |
| An ambassador is consumed for a permanent change and stays for a position | candidate (Matt) |
| Remove: an ambassador from the supply goes onto the hex, evicts up to `remove_count` and stays | candidate (Matt; supply only is 068's call) |
| Player ambassadors on bases, factories and corrupted stations stay; on a corrupted station the round's cube is theirs | candidate (Matt; factories and the cube are 068's call) |
| An adversary ambassador on a lone presence stays until removed | candidate (Matt) |
| A Move or card move onto adversary ambassadors still cancels 1 for 1, consuming the mover | candidate, 068's call |
| An adversary arrival still cancels a standing player ambassador | candidate, 068's call |
| Research stations cost chips alone | candidate (Matt) |
| `ai.start_credits` 15 / 25, per board | candidate (Matt: Builder lower, Protector higher) |
| `ambassador_credits` 9 | candidate, simulation (Matt: 6 or more) |
| Followers: Liaison Corps 2 for an ambassador, Open Assembly 3 for one | candidate, simulation |
| `Op::Income` prints ambassadors (Orbital Embassy) | candidate |
| Hardpoint Lockdown cordons a country: no arrivals there next round | candidate |
| Unspent-resources measure and its flag (`docs/principles.md`) | candidate |

Gone: Economies of scale and every AI credit discount (`discounts` in `content/ais.json`, `Discount`, `DiscountOn`, `discount_room`); Retool; Bulwark's Security-card
clause and its standard action (`harden_energy`); Lockdown; `build_energy`. Science's niche keeps marker clearing among cards; a power may print it now (Recovery).

**Revision 27 (066): presence powers replace growth.** (Bumped from the constant, 26.) Design: Matt, 2026-11-01. The
band: planner, d2, fresh seeds 20200 and 20300, 96 a cell: **59 ±5 / 61 ±5** (target 60-75: c2 just under, no lever added for it), on the frozen build;
`experiments/results/screen-066-presence/presence.md`. The gate: no check ahead of the planner by more than 1 SE; 96 paired
seeds for the five closest, 48 for the rest, `never_tag` left at 8. Every row is a **candidate** unless marked.

| Rule | Status |
|---|---|
| No growth phase: presence comes from presence powers, each once a round, 0-2 a round from powers | candidate (Matt) |
| Placements pay the printed step: one payment path, `board::place_presence` | **locked** |
| Every board's Deploy: 1 energy for 1 presence | candidate (Matt) |
| Builder's Buy In: 1 energy + 6 credits (the design's 10 read 54 / 58 against 67 / 62 on the same 48 seeds, 20200, where 6 sat above the band's floor at both sizes; 3-4 credits ran higher at c3 on 25 seeds, far from a steep price, while at c2 the same screen read 36 and 75: noise) | candidate, simulation |
| Protector's Accord: 1 energy + sacrifice 1 Diplomacy (the design's 2 went unused, 0.1 a game) | candidate, simulation |
| Tag costs on powers are sacrifices; no thresholds on them | **locked** |
| Cheap powers: Scrap (1 Industry for 2 chips), Stand Down (1 Security for 3 credits) | candidate (Matt) |
| Each player starts bound to the clean city nearest their first presence | candidate (Matt) |
| `ai.start_energy` 1: production comes after the action phase, so a seat at 0 can't place in round 1 | candidate, the prompt's calibration read it in |
| `ai.start_credits` 38 (was 20): the credit half of Consolidate's bundle | candidate, simulation |
| Track income: Builder space 3 +2 credits, space 4 +1 credit; Protector spaces 2-4 +1 credit | candidate, simulation |
| Resource domains: remove spends `remove_ambassadors` 1 ambassador, align its ambassador only, clear `clear_cost` 2 credits, move `move_credits` 1 credit, a card's Harden `harden_chips` 1 chip | candidate (Matt) |
| `remove_count` 3: one ambassador removes up to 3 from a hex | candidate (Matt) |
| `ambassador_credits` stays 6 (Matt: standard projects are the exception to most builds) | candidate (Matt) |
| Consular Pouch 3 for 5, Shuttle Diplomacy 2 for 4, Ceasefire Monitors 2 for 5, Summit Caucus 2 for 5, Consulate Tower an Action (1 credit, 1 ambassador) at 7, Liaison Corps 1 Follower for 1 ambassador at 5, Orbital Embassy an Action (1 credit, 2 ambassadors) | candidate (Matt's route), reasoned: no effect measured |

Gone: Expand, Develop, Consolidate and `ai.growth`; the Place presence standard action and `presence_action_cap`;
the rich action (`presence_buy`, `presence_credits`, `presence_rise`, `presence_stage`); the buy-energy project
(`energy_credits`); `presence_energy` (now Deploy's printed cost); `remove_energy`, `clear_energy`, `move_energy`,
`align_energy`; the op `next_building_discount`, which only the Builder's Consolidate printed. No card named a removed
mechanic, so none had to adapt: the Bunker's "Harden action" stays and says it is paid in chips. Agents: a presence power
is scored at `choices` × 2 at the top level and at least 3 prompts in rollouts (short of that it reads as paying and
placing nothing); `no_powers` no longer rules out the presence powers; `rich_spend`, `always_expand` and
`always_consolidate` are gone with their mechanics, `always_max_presence` and `never_second_presence` are new.

**Revision 26 (064): d2 re-centered for the four-deal planner.** (Bumped from the constant, 25.) The default
planner is now `deals` 4, `lead` 0 (an agent change, chosen on fresh seeds, r25: 81 / 73 against one guess's
69 / 60). Matt's standing ruling: push difficulty up where it can go. Measured on seeds 16000, nothing
picked there: 75 / 71 (base 75 / 77); on the gate's seeds 17000, 67 / 52; the 96 together 71 / 62. See
`experiments/results/screen-064-deals/deals.md`. Always-Consolidate at c3 still leads the gate (+17.5 ±9.4).

| What | Was | Now | Status |
|---|---|---|---|
| `adversary.corrupt_at` | 6 | **5** (018 had raised it from 5) | candidate |
| `board.project_discord_per` | 2 | **1**: every adversary cube on a completed megaproject is 1 Discord | candidate |

**Still revision 25 (063): card terms and marks.** Matt's rulings on 059's design calls (2026-11-01). Words and
marks only: no op, number or play changed, so no bump (the constant stays 25). The golden fingerprint moves because
ids in the state were respelled; `same_play` compared every game field by field.

| What | Was | Now | Status |
|---|---|---|---|
| Facility | "building" also meant a station or factory | **facility**; a building is always a card | candidate |
| Harden | a facility or a hex of presence | a facility only; on presence it is a **shield**. "Harden action" stays, no keyword | candidate |
| House words | in the guidelines and a rules paragraph | once, `HOUSE_TERMS`: rules.md's *Terms* and `docs/player-aid.md` | candidate |
| Jam mark | a minus on the threat in the dark frame | its own mark, a threat with a down arrow | candidate |
| Free placement | the presence mark | its own mark, presence with a struck-through coin | candidate |
| "Up to N" gives | `−4` and 4 in the dashed box | `−≤4` and `≤4`: `≤`N is up to N | candidate |
| Dashed box | another player, or your tokens | another player; a thin solid box counts across your cards | candidate |
| Shelter mark | a whole country, and Aegis Dome Array's one stop | a whole country; Aegis Dome Array prints the shield mark and a country | candidate |
| Bounty card | 059's suggestion | none for now | decided |
| Spelling | mixed | American everywhere, ids included (Mobilization, Armored, Modeling); `aic-sim docs --check` refuses British spellings | locked |

**Revision 25 (059): the card design pass.** (Bumped from the constant, 24.) Matt's principles
(2026-10-31), [card-guidelines.md](card-guidelines.md). Every card is exception-free: no face prints a `*`,
a cap, a footnote or a sum. 91 of 158 player cards changed in what they do or cost, 43 in words; all
candidates. The list is `experiments/results/screen-059-cards/design.md`, the prices `docs/card-pricing.md`.
Threats, Unity, Discord, adversary, project and megaproject text was reworded, ops untouched.

| What | Was | Now | Status |
|---|---|---|---|
| Removal | "remove N worth" (`adversary.troop_health` 2) | a card names its pieces, "2 adversary ambassadors or 1 troop": one kind (`troop_health` gone) | candidate |
| Hardening from a card | all in one country | any different buildings in reach | candidate |
| Cash out | 1 + 2 + ... + n, some capped | a plain rate per token | candidate |
| Income per tag, per token, per presence | a `max` | no cap | candidate |
| `convert`, `if`, `income_if_tags`, `next_round_insight`, `slotless`, `transfer`'s bonus, `harden`'s and `damage`'s country rules | ops and fields | gone: fixed trades, plain effects | candidate |
| Modular Expansion | +1 slot, takes no slot | +2 slots, sits in one | candidate |
| Aegis Dome Array | spread skips your presence, first destruction saved, Harden Action | first destruction saved only | candidate |
| Clear a city | next to, then on | on or next to, your pick | candidate |
| Adversarial Inference Sweep (Matt, #2190) | needs Science (055) | needs Cyber: no card needs Science for its niche | candidate |
| A site corrupted with a marker on it | the marker stayed | corruption replaces it, as on a city (slow-lane invariant, new content hit it; the golden game didn't move) | fix |
| Fonts, renderer | character-count text size; 058's picker fonts | measured fit, measured titles; Saira Condensed and Source Serif 4 only | candidate |

**Revision 24 (055): first price pass.** (Bumped from the constant, 23: 054 took none.) Cards only, no
rule changed. Screen: `experiments/results/screen-055-prices/prices.md`; rates and reasons in
`docs/card-pricing.md`. Planner 79 / 75 on 160 seeds before, 67 / 65 on 48 fresh seeds after (other seeds, not paired).

| What | Was | Now | Status |
|---|---|---|---|
| Great Liberation | 14 credits | 16, needs 1 Diplomacy | candidate |
| Beamed Power Constellation, Peace Dividend | 5, 2 credits | 7, 3 | candidate |
| Darknet Exchange, Security Contractor HQ | 4, 4 credits | 2, 3 | candidate |
| Rootkit Cache's Action | 2 credits | 1 | candidate |
| Kinetic Interdiction, Node Severance, Singularity Exploit, Exodus Corridors, Mesh Intrusion Pact, Adversarial Inference Sweep | no needs | needs 1 of Security, Cyber, Cyber, Diplomacy, Cyber, Science | candidate |

**Words only, no revision (050).** Play held (the golden game didn't move). The Discord cards say
"after 3 Discord tips", which is what the rule always did: the step first fires on the 4th tip (D18).
Delay prints a clock, not Jam's minus on a threat (D19). rules.md says a shield on your own building
stays as its hardening token when the presence goes (D17). Unused `AiBoard.unity` and `CardDef.upkeep`
are gone, with the Unity-a-round column and the building-upkeep rule: no board or card used them.

**Revision 23 (053): calibrated at d2, pressure per country.** Matt's rulings of 2026-10-31. (The
prompt's number; the constant was 22, so it held.) The planner sits at 60-75% at d2 at both sizes on
fresh seeds: one rule plus an agent fix. The agent fix did most of the pushing up: 96 / 54 to 67 / 38
on 24 seeds with no rule changed, so ruling 2's general push added no lever (each one tried left the
band). Screen: `experiments/results/screen-053-calibration/calibration.md`.

| What | Was | Now | Status |
|---|---|---|---|
| Arrivals at military bases, Escalation | every base: 1 a country at 2 countries, 2 at 3 and 4 | each country's first base, its lowest-numbered (`base_sites_per_country` 1) | candidate |
| War Drums, Border War, Blitzkrieg; every Escalation | "military bases" | "each country's first base" (text) | candidate |
| Dominance gate | gate-046 | gate-r23 (`experiments/gate.json`): gate-046's checks plus abandon; 8 seeds a check | gate-046 retired |
| Failure mix target | each clock under about 60% of losses | under about 80% (`docs/principles.md`) | candidate |

Tried and dropped, all on top of bases per country unless marked: a country sways once for its bases
(alone, 42 at c3); Discord bucket 7 or 8 at c3 (no bases rule: 54 / 75, but it fixes c3 with the cause
left in); 3 and 5 (58 / 46 and softened steps); tier-2 and tier-1 Discord steps taking 1 presence
instead of 2 (82 / 68: all three clocks live, c2 out of the band); those with `extra_threats` 2 (50 /
46); with `escalation_ambassadors` 2 (83 / 54); with `project_discord_per` 1 (75 / 71); that last on
its own (70 / 62, 56 seeds: inside the band, and the first number to try if more push is wanted).

Agents, no rules change, 061: the default planner is one guess, `deals` 1, `lead` 0, `prune` 12. 48
fresh seeds at r25 read 67 / 75 against `deals=4,lead=60`'s 67 / 69, for 0.8 × the time. See
`experiments/results/screen-061-one-guess/one-guess.md`.

Agents, no rules change, 054: the planner's lines average over 4 sampled Unity deals, an alternative
must lead by `lead` 60, and rollouts keep `prune` 12 actions (6 crippled them). r23's band was read on
the old planner: see `experiments/results/screen-054-deals/deals.md`.

Agents, no rules change: D20, which Alignment cards a game dealt is hidden from every agent, the
planner's lines included (`hide_alignment_deal`). The `abandon` check. Stats, no rules change: Discord
Sway is tallied by source (`alignment.discord_from.*`). 052's clamp on `niche_jobs` is what this item
asked for there.

**Revision 22 (046): one standard placement, a Science niche, a better Develop.** Matt's rulings on
041 and 042 (2026-10-30). (The prompt called this r21; 044 took that number.) The round cap's quiet
backstop is **locked**: 0 cap losses is fine and it has no share target; all else is a candidate.

| What | Was | Now | Status |
|---|---|---|---|
| Place presence (standard action) | any number a round | `presence_action_cap` 1 a seat a round; the price stays 1 energy (locked, 016); growth, cards and the rich action don't count | candidate |
| Develop | Builder: 1 presence, a card; Protector: 1 presence, 1 insight | both: 1 presence, draw 1 tier-1 card | candidate |
| Shields, threat delays, marker clearing on cards | any card, by custom | Science only, refused at load (no card needed moving; `needs` unchanged, no tag mandatory) | candidate |
| Field Sensor Grid | clear 1 | clear 2 | candidate |
| Benchmark Suite | 1 insight, shield 1, 1 Dataset | 1 insight, shield 2 | candidate |
| Predictive Modeling | 1 insight to play | 0 | candidate |
| Threat Forecast Engine | peek 2, draw 1 | peek 1, one threat waits | candidate |
| Shadow Broker Feed | peek 2, draw 1 | clear 1, draw 1 | candidate |
| Mesh Intrusion Pact | gain 2 credits | shield 2 instead | candidate |

Agents, no rules change: growth is scored at full depth at the top level only, and the planner's line
follows that pick; drafted and packet cards count only as far as the round's insight keeps them
(`w.packet_budget`); the keep plan adds `plan_niche` for each marker a card would clear, threat it would delay
and shield it prints. Screen: `experiments/results/screen-046-niche/niche.md`.

**Revision 21 (044, bug hunt): rules that didn't run, cards offered for nothing.** All fixes;
none changes a number.

| What | Was | Now | Status |
|---|---|---|---|
| Stage-III cards' "2 Sway leaves the Unity bucket" | printed since r20, never ran (the twist was dropped): 041's screen was played without it | runs | fix |
| A card or ability that only hardens or moves one ambassador | offered and paid for with no target | not offered | fix |
| "Remove 1 worth" | offered on a hex holding only a troop (worth 2): paid, nothing removed | such a hex is no target | fix |
| A cash-in after an add-tokens on the same card | refused when the tokens weren't there yet | counts what the card adds first | fix |
| "Up to N" on Clear, Shield, Delay | forced to N | "stop" after the first, as Cleanse | fix |
| A second Shelter on a country, a second cut on a node | offered and paid for, no effect | only what it can act on is offered | fix |
| Withdraw after losing your presence in a threat's country | not offered: the payment stayed until the threat executed | offered; paying isn't | fix |
| `stage_cards` check | added `stage3_added` at every difficulty, took off no removal | checks what the difficulty deals | fix |

**Revision 20 (041): growth as a choice, the marker worth clearing, Science on the board.**
Matt's rulings on 034 (2026-10-30); only "a cap exists and can shorten" is locked.

| What | Was | Now | Status |
|---|---|---|---|
| Round cap | 11, 10 from difficulty 2 | 11 through d2: difficulty 2 swaps a stage-I card for a stage-III (`stage3_added`); difficulty 3 removes a stage-II card (`stage2_removed`): 10 | candidate (that a cap exists and can shorten: **locked**) |
| Stage-III cards | each player destroys 1; 1 Discord | each destroys 2; 1 Discord; 2 Sway leaves the Unity bucket (ruling: the cap bites through a harder end) | candidate |
| Growth | three board-specific options, one placing 1 presence free | Expand (2 presence, small gain), Develop (1 presence, a draw or insight), Consolidate (no presence, a bundle) | candidate |
| Presence payment | growth and cards free, the action paid an extra, the rich action paid it in credits | one path: every placement pays the step's printed `cost` unless a card prints free; the action adds `presence_energy`, the rich action pays credits | candidate |
| Track steps | extras 0, 0, 0, 0, 1, 2, 3, 4 and 0, 0, 1, 3, 4, 5 | costs 0, 0, 0, 0, 1, 2, 2, 3 and 0, 0, 1, 2, 3, 4 | candidate |
| Tier-2 placing cards | free | still free, printed so | candidate |
| Clearing a marker | part of Remove, after any ambassador on the hex, no reward | its own action, `clear_energy` 1, sways `clear_unity` 1 toward Unity | candidate |
| Science | insight and cards only | six cards reworked to clear markers, shield presence, delay threats, move an ambassador | candidate |
| Shield | hardening on buildings only | Science's `shield_presence`: a hardening token on a hex of your presence | candidate |
| Delay | | `delay_threat`: a threat waits a round, and can be countered next round | candidate |

Agents, no rules change: growth is scored 4 choice levels deep (at 2, Expand looked like its first
token); eval weights `marker`, `shield`, `threat_waits`, and a marker counts as city pressure.
Screen: `experiments/results/screen-041-growth/growth.md`.

**Revision 19 (034): three distinct loss pressures, Spirit Island presence, earlier Science and Industry.**
(The review called this r18; 033 took that number.)

| What | Was | Now | Status |
|---|---|---|---|
| End conditions | | five: purge, the last Unity card; the Discord deck, presence, a round cap | **locked** (Matt, 2026-10-30) |
| The three loss clocks | | distinct pressures; which ends a game varies | **locked** (Matt, 2026-10-30) |
| Round cap | the adversary deck, 11 rounds | still the deck, `stage_cards` 4, 5, 2: 11 rounds, 10 from difficulty 2 (the review's 12 and 11 ended no game: 0 of 69 d2 losses) | cap **locked**; the number a candidate |
| Destroyed presence | back on its track, re-covering the space | leaves the game; its income stays | **locked** (Matt, 2026-10-30) |
| Corruption marker | none | a lost presence marks its hex unless the hex has corruption; a marked hex takes no presence, counts 1 toward corrupting a city; Remove takes one | the marker a candidate; what it does a **Default** (the review named only its caps) |
| Presence count | 6 + 3 tokens | 8 + 6, last steps cost a printed extra (cards and growth skip it, and place 70% of tokens) | candidate |
| The rich action | a 3-credit project from round 1 | `presence_buy`: from stage II, 6 credits, + 3 for each earlier use a round, + the extra at `energy_credits` | candidate |
| `energy_credits` | 3 | 4: at 3, the project laundered credits into a 1-energy token | candidate |
| Discord presence hits | each player 2 at 4+, 7 at 5+ | 1 at 3+, 2 at 4+, 2 at 5+ (the review's 3 at 5+ left too few other sources) | candidate |
| More presence destruction | | seven more threats, Manhunt and Siege 2, stage-III cards and Border War, `corruption_reach` 1 (war: `war_reach` 0) | candidate |
| Stage-III Discord sway | 2 | 1, so more games reach the cap | candidate |
| `alignment.free_city` | 4 | 5: c3 won 28%, its wins late | candidate |
| Difficulty 4 | removed card, a step never reached | the removed card counts as a tip made: all steps reached | fix |
| `hold_base` | | the same at every difficulty | **locked** (Matt, 2026-10-30) |
| Science | | `science_every` 2: 1 insight off tier-2 keeps and plays per 2 tags, floor 1; cheaper cards with immediate gains; `needs` on strong ones | candidate |
| Industry | | `industry_every` 2: 1 credit off buildings per 2 tags, floor 1 (the floor now holds for every building discount); immediate gains | candidate |

Agents, no rules change: keep plan `plan_rounds` 4, `plan_reserve` 0; presence danger from 4
tokens down. `engine:samples=2` tried and dropped. Screen:
`experiments/results/screen-034-clocks/clocks.md`.

**Revision 18 (033): ongoing abilities only on buildings, card text from ops.**

| What | Was | Now | Status |
|---|---|---|---|
| Ongoing abilities | programs could gain tokens each round and carry an ability | buildings only, refused at load | **locked** (Matt, 2026-10-30) |
| Grad Student Pool, Synthetic Data Forge, Retained Earnings, Expansion Fund, Sovereign Capital Fund, Liaison Corps, Open Assembly, Sentry Drone Wing, Kill Team Roster, Safehouse Network, Deep State Ledger, Lights-Out Assembly, Honeypot Relay, Rootkit Cache, Demand Response Grid | program with an Action | building, every number the same | candidate |
| Collateralized Grid | 2 credits; Action: pay 1 credit, 1 energy per 2 Capital (max 3) | 3 credits; once, when played: 1 energy per 2 Capital (max 3) | candidate |
| Materials Hedge Desk | 2 credits; Action: pay 1 credit, remove all Capital, 1+2+...+n chips | 3 credits; once, when played: remove all Capital, 1+2+...+n chips | candidate |
| Card text | hand-written in `content/cards.json` | generated from ops (`aic_core::face`) | **locked** (Matt, 2026-10-30) |

The grammar is in rules.md, *The card grammar*.

**Revision 17 (028): presence loss that bites, a Unity deck that can run out.**

| What | Was | Now | Status |
|---|---|---|---|
| Destroyed presence | a destroyed pile, its track space stayed uncovered | back on the track, covering the furthest uncovered space (`presence_returns` 2) | candidate |
| Late presence hits | one hex a hit | each player destroys presence of their own: the higher Discord steps, Manhunt, Siege (op `each_destroy_presence`) | candidate |
| Unity deck | all 9 cards | a game deals `unity_cards` of the 9 printed | candidate |
| Hold | `hold_base` 2 | `hold_base` 3, the brief's number, which a planning agent can gather | candidate |
| Aligned cities | a second spreading ambassador made a node | collect ambassadors as clean cities do | candidate |
| A corrupted factory's adversary ambassador | stayed forever | used up when it makes its troop | candidate |
| A hack on a node's hex | outlived the node and cut a node formed there later | cleared with the node | candidate |
| Discord steps and threats | Offensive and Firestorm at 4+ took one hex; Manhunt took one hex; Siege's troops took presence where they landed | Offensive and Firestorm at 4+: each player destroys 2; Incursion, Blight and Propaganda at 5+: each player destroys 7; Manhunt and Siege: each player here destroys 1 | candidate |

Unchanged and **locked**: the Discord-deck loss and the presence loss. Screen:
`experiments/results/screen-028-rollout/rollout.md`.

**Revision 16 (027): steady pressure, two decks as the clocks, presence loss.**

| What | Was | Now | Status |
|---|---|---|---|
| Discord deck | reshuffled | never reshuffles; a tip with no card loses | **locked** (Matt, 2026-10-30) |
| Presence loss | none | a player with no presence on the board: everyone loses | **locked** (Matt, 2026-10-30) |
| Unity win | `unity_win` 8 tips | drawing the last Unity card; never reshuffles | candidate |
| Purge | counted from `purge_round` 6; `unity_ease` 1, 1, 2 | no round gate; `unity_ease` 2, 2, 4 | candidate |
| Arrivals | none | each adversary card names a site type; every such site gains 1 ambassador in the build | candidate |
| Freeing | `free_city_at`, no adversary ambassador adjacent | hold: `hold_base` 2 + adversary ambassadors on or next to it, any players, at least `hold_min` 2 | candidate |
| Piece ladder | freeing cleared the city | a freed city keeps `free_leaves` 1 adversary ambassador | candidate |
| Presence destruction | troops, war with no factory left; a sacrifice left the game | threats, tier-2 Discord cards, corruption, war through a destroyed factory, Purge the Unaligned; all to the destroyed pile (018's reserve); troops strike buildings only; stage I never takes a last presence | candidate |
| Supply | ran out: a loss | a cap | candidate |
| Every city corrupted | a loss | no loss | candidate |
| Calibration | `city_build` 2, `node_compute` 3, `base_discord` 3, `destruction_discord` 1, `unit_cap` 3, one bucket size, `undecided_per_player` 3 | `city_build` 1, `node_compute` 1, `base_discord` 1, `destruction_discord` 0, `ambassador_discord` 0, `unit_cap` 2, `discord_tip_per_player` 2, `undecided_per_player` 4 | candidate |
| Difficulty 4 | supply smaller (`supply_reduction`) | one Discord card fewer (`discord_removed`) | candidate |
| Content | | Eviction, Embargo, Smear campaign, Manhunt; Siege, Total war, Purge protocols, Propaganda, Offensive and Firestorm changed; the Discord deck loses Buildup (6 cards) and steps at reachable tips; stage-III cards sway 2 Discord; `docs/harms.md` | candidate |

Screen: `experiments/results/screen-027-endgame/endgame.md`.

What the code implemented at r16: **rules revision 16** (027: **steady pressure, two decks as the
clocks, presence loss**, above; on 024's revision 15: **a scarier adversary**, a purge
that can't come early, and credits that matter, below; on 023's revision 14: **the player card set**, about 150
cards built on 021's primitives and 022's costs, with four new effect primitives, §8; on
022's revision 13: **insight costs on cards**, shared
between keeping and playing, and 020's deferred calls, below; on 021's revision 12:
**card primitives and the resource model**; on 020's revision 11: five bug fixes from the bug hunt; on 019's revision 10: adversary troops **march**
toward player infrastructure and nodes are **hard to remove**, below; on 018's revision 9,
the structural fixes and the calibration; on 017's revision 8): the 014 board model with player
**presence** in place of data centers, the adversary's pieces as a readable **ladder**
(ambassadors, nodes, troops, corrupted cities and stations), **factories** that grow
from cities on printed industrial zones and make a new resource, **hardware**, a printed
**production/war clock**, and the **purge** win eased by Unity. 017 gives adversary
compute something to spend on, a shared **adversary project deck** (§3), made each
Discord tip advance the adversary troops (019 replaced that with the march, §4), drops 013's Discord escalation, makes
every Alignment card **one-shot** and adds Unity sources (§5). The Militarist against
Builder and Protector, 1–4 countries. The player cards are 023's set (§8, listed in
[cards.md](cards.md), priced in [card-pricing.md](card-pricing.md)): every card a
**candidate**. The AI unique cards, AI boards, threats, the Unity and Discord decks and
the project deck are still **placeholders** pending redesign (the adversary content is
the next item).

**Status (024).** Every 024 change is a **candidate**. Results:
`experiments/results/screen-024-adversary/`. The first eight rows are Matt's; the rest are
024's adaptations to keep the game healthy.

| 024 rule | Was | Status |
|---|---|---|
| **Freeing a corrupted city** (§2): one player holds `free_city_at` (1) ambassadors of their own on it and no adversary ambassador is on or next to it; those ambassadors are used up | 2 of anyone's, or a card, regardless of what stood around it | candidate (Matt's rule; X 024) |
| **No card frees a city outright** except the tier-2 ultimate Great Liberation (§8); `cleanse` takes stations and factories, and prints `cities` only there. Cards help instead: `clear_city` (Amnesty Decree: 2 around a city; Liberation Accords: all, plus 2 ambassadors) | 5 cards freed cities | candidate (Matt) |
| **More ambassadors:** `city_build` 2 | 1 | candidate (Matt's axis; the number 024) |
| **Costlier counters, in credits** (§8): every threat 4-6 credits in tier 1, 6-8 in tier 2 | 1-6 of mixed resources, antimatter in tier 2 | candidate (Matt, the threat-design menu: a third to a half of a round's income) |
| **Threats hurt more** (§8): more ambassadors, troops and cards, 1 more compute, heavier losses | see the 022 table | candidate (Matt's axis; the numbers 024) |
| **Projects players must answer** (§3): stronger effects at lower cost; `node_compute` 3 so they resolve in the game's first half; **Infiltration Drive corrupts a city in each country** (`corrupt_cities`, new) | weak one-shots, compute 1 a node | candidate (Matt's axis; the content 024) |
| **Standard projects** (§6): presence 3 credits, harden 3 (any player), 1 energy 3, drain 3 compute 5, ambassador 6 | ambassador only | candidate (Matt; the prices 024) |
| **Card text** (§8): every card repeats its rules text in at most 100 characters; the viewer prints it on the card | up to 255 characters, tooltip only | candidate (Matt) |
| **The purge counts only from round `purge_round` (6)** at every Unity level (§5) | any round | candidate (024: the smallest rule that closes it; harder freeing alone left round-4 wins, and a stage-II gate would be round 4 from d2) |
| `alignment.free_city` 4 | 5 | candidate (024: at 5, two frees made two tips at 2 countries, the round-3 win; 4 is one tip there, and with `free_city_at` 1 gave 92 / 92 at d0 and 54 / 50 at d2 on the tuning probe) |
| `adversary.supply_per_country` 24 | 14 | candidate (024 adaptation: with the build doubled the supply ran out by round 5-6) |
| `adversary.extra_threats` 1 (difficulty 1) | 2 | candidate (024 adaptation: 2 more threats at 4-8 credits each took a whole round's income; d2 was 0%) |
| Tier-2 `gain_per_tokens` and `cash_out` cards are offered only with tokens to count | offered, did nothing | candidate (024 fix) |
| Great Liberation 14 credits, up to 2 sites (was 10, up to 3); Liberation Accords no longer moves; Hyperscale Botnet's text said 3 Cyber (it needs 2) | | candidate (024) |

**Not built in 024:** no insight generation was added (Matt: report it first; see the
results); `free_city_at` 2 is Matt's likely intent for "X ambassadors" and is one config
line, but the one-step agents then can't line up the two conditions in one action phase
(c2 d2 44%, c3 d2 31%, with ignore-Cyber ahead).

**Gate (d2, 32 seeds): unmet at 2 countries.** ignore-Science, -Industry and -Cyber don't
lose to the engine there (−9, −22, −9, ±10-11); at 3 countries they do or are level (+3,
+9, +16), and ignore-Security loses at both. Cause and the full table:
`experiments/results/screen-024-adversary/adversary.md`.

**Status (023).** Every 023 change is a **candidate**. Results:
`experiments/results/screen-023-cards/`.

| 023 rule | Was | Status |
|---|---|---|
| **The player card set** (§8): 150 cards in the draft decks (108 tier 1, 42 tier 2; 90 programs, 36 buildings, 24 events; each tag primary on 25), names only, listed in [cards.md](cards.md) | the 52 ported placeholders | candidate (Matt's shape and tag identities; the cards 023) |
| **Draw** (`draw`): Science's card draw, from the tier-1 deck, no keep paid | no drawing (021) | candidate (Matt's tag table) |
| **Drain all** (`drain_all`): Cyber tier 2's "every project −X" | | candidate (Matt's tag table) |
| **Count as power** (`gain_per_tokens`) and **cash out** (`cash_out`, 1 + 2 + ... + n, an optional cap) | tokens spent at a fixed rate only | candidate (Matt: "build count as power if cards need it"; cash out for Capital's growing rate) |
| A card whose effect is an `if` is offered only when the branch that would fire has a target | always offered (the `if` was not checked) | candidate (023 fix, found by the card audit) |
| `alignment.free_city` **5** Sway toward Unity for freeing a corrupted city (§5) | 3 | candidate (023 adaptation: with 150 cards the few freeing cards are drawn less often, and d0 losses were cities still corrupted at the deck's end; 5 took the engine at d0 from 75% / 56% to 88% / 81% at 2 / 3 countries on the tuning probe) |
| More cards free cities: Station Prefab Order (Industry) and Back-Channel Talks (Diplomacy) take a cube off; tier-2 `needs` one tag lower | 3 freeing cards in 150 | candidate (023 adaptation: 50% / 25% → 69% / 44% engine at d0) |

**Status (022).** Every 022 change is a **candidate**. Results:
`experiments/results/screen-022-insight/`.

| 022 rule | Was | Status |
|---|---|---|
| **Insight to play** (§8): 0 for small programs and events, 1 by default, 2 for big effects and most tier-2 buildings; it comes from the same insight as keeps (keeps still cost 1 / 3, credits still buy keep insight only, at 5:1) | no insight to play (021) | candidate (Matt; the per-card prices 022) |
| The removal events (Precision Strike, Sweep, Strike Team) cost **1** insight, not an event's 0 | (Matt's distribution: events 0) | candidate (022 adaptation: at 0 the cards that win were free to play once kept, and keeping every drafted card beat the engine, 91% to 59% at c2 d2) |
| **Mixed costs** (§8): a card may also **lower one of your incomes** (`lowers`): Fusion Plant, Antimatter Refinery and Orbital Lance lower your credit income by 1 | credits, energy, hardware, antimatter | candidate (Matt; which cards 022) |
| Base insight **4** a board (§7) | 3 | candidate (022 adaptation: with plays paying insight, 3 cut the engine to 75% at d0) |
| No hand cap | (none) | locked (Matt) |
| Threats dealt after the counter phase **wait a round** (§8, 020's D11) | executed the same round | candidate (Matt took the pick) |
| A player ambassador on an adversary-owned factory is **used up** for `factory_unity`; the troop is still made (§2, D1) | sat there forever | candidate (Matt took the pick) |
| **Four more tier-2 threats** (Conscription, Fifth column, Insurrection, Siege; §8, D12): 4 countries at d1+ never run short; a short deal is tallied (`threats.short`) | 5 tier-2 threats | candidate (Matt took the pick; the threats 022) |
| **Hardening** only on buildings a token can protect, and "harden N" means N different buildings (§6, §8, D10) | anyone's building, the same one twice | candidate (Matt took the pick) |
| Before the build step, card twists are the **face-up card's** (a Sway-phase Discord tip; §3, D13) | last round's card | candidate (Matt took the pick) |

**Status (021).** Every 021 change is a **candidate** except AI-board powers paid by
sacrificing tags, which Matt marked **locked** (2026-10-29). The rows marked (021) are
021's adaptations and defaults; results: `experiments/results/screen-021-cards/`.

| 021 rule | Was | Status |
|---|---|---|
| **Players have no compute**; compute is the adversary's alone (§3) | presence-track compute, drawing, Cyber discounts | candidate (Matt) |
| Presence-track spaces that gave compute give insight, credits or energy; each board's last space gives 1 antimatter a round (§7) | +1 compute on spaces 1-2 | candidate (Matt; the values 021) |
| Cyber cards that made or spent compute are **hacks**: Data Mining cuts a node off (no compute in the next build) and drains 1 project compute (§8) | compute per presence | candidate (Matt; the card 021) |
| An adversary ambassador on a lone presence costs its owner 1 insight next round and adds 1 compute to the cheapest project; it is used up (§2) | leaked 1 compute, 1 Discord | candidate (Matt) |
| **City energy cubes** (§6): an aligned city's goes free to its aligner; a neutral city's can be bought by a player with presence next to it for `city_energy_credits` (2) credits, else it is lost; a corrupted city's goes to the adversary (Discord, as before) | the most presence next to a clean city took it free; aligned +1 bonus | candidate (Matt; the price 021) |
| `board.corp_energy` (a list by country, 1 each): a megacorp parameter, 2 for 2 energy a city | `city_energy` 1 | candidate (Matt; off by default) |
| Energy pays presence, Power cards (`energy` in a card's cost), standard actions and **hardening** (`harden_energy` 1) | hardening 1 hardware | candidate (Matt) |
| **Hardware** is a requirement in some buildings' cost (`hardware`); it comes from player-owned factories and Industry cards (Fab Line +1 a round, Holding Company); the standard action sells 1 hardware for `hardware_credits` (1) credit | bought at 4 credits | candidate (Matt) |
| **Antimatter**: no tier-1 source (Scavenge gives 3 credits instead); the last presence-track space and tier-2 cards (Antimatter Refinery) make it; the Discord-for-antimatter action stays (§5) | Scavenge 1 | candidate (Matt; keeping the action, 021) |
| **Small numbers**: card costs halved (rounded up; Megastructure 13, Orbital Lance 10); credit production 10 (Builder) / 9 (Protector) plus a starting building's +1; base insight 3 a board (+1 on the first space) | costs 0-25; 8 credits; insight 2/1 | candidate (021) |
| **Opening** (round 1): each player is dealt `opening_deal` (8) tier-1 cards, keeps `opening_free` (4) free and each further one for `opening_keep_credits` (5) credits; no draft in round 1; `ai.start_credits` 20 | 25 credits and a round-1 draft | candidate (Matt; 20 and the no-draft round 021) |
| **The draft** (rounds 2+): each player builds a hand of `deal` (4) **card by card, choosing each card's tier** (each pick deals that deck's top card face up into the hand), then hands pass as before | one tier for the whole hand, tier 2 only once unlocked | candidate (Matt) |
| **Keep costs** insight: `keep_tier1` 1, `keep_tier2` 3, `keep_tier2_upgraded` 2 once the board's keep upgrade (presence-track space 3) is uncovered; any shortfall at `insight_credits` (5) credits an insight, for keeps only | 1 credit a card | candidate (Matt) |
| **No tier-2 gate**: tier 2 is drafted and played by anyone | unlocked on track space 3 | candidate (Matt) |
| Playing a card costs no insight (021: insight pays keeps, counters and giving) | 0-2 insight a card | superseded by 022: 0-2 insight a card again |
| **Drawing** with compute is gone (`Action::Draw`, `cards.draw_cost`) | 3 compute a card | candidate (021) |
| **Tags** Science, Industry, Cyber, Power, Diplomacy, Security, 1-3 a card; tags count on every card in play (buildings, splayed ones and programs); requirements are checked only on play | economy/cyber/power/unity/discord on buildings only | candidate (Matt) |
| **Programs** stay in your play area (their tags count; they can be sacrificed); **events** (strikes, hacks, one-off gains) are discarded | programs discarded | candidate (Matt; which are events 021) |
| **Buildings**: `ai.slots` (Builder 4, Protector 3) slots; building over a building **splays it down** (its effect ends, its tags stay); a building may need to replace one of yours carrying a tag (`replaces`: Fusion Plant over Power, Megastructure over Industry); a building may give a one-time permanent gain (Trade Office: +1 credit production); each board starts with a building carrying a built-in tag (Foundry Core, Industry; Sentinel Post, Security) | 6/3 slots, a full row discarded one | candidate (Matt) |
| **Token engines**: tokens live on cards that print a type (Datasets, Followers, Capital, Agents); patterns built: gain N a round, add tokens to a card of a type (Seed Funding), spend at a rate (abilities), adversary effects that remove them (Sabotage) | none | candidate (Matt) |
| **AI-board powers**, once a round, paid by sacrificing played cards carrying the cost's tags (a program alone; a building with its whole stack; never an event), plus optional other costs; one placeholder per board (Builder *Retool*: 2 Industry + 1 any, gain 5 credits and 2 hardware; Protector *Lockdown*: 2 Security + 1 any, remove 3 worth in reach and sway 1 Unity) | none | **locked** (Matt) |
| Field Office: while you have it, cards discarded from hand count their tags for powers | none | candidate (Matt's primitive; the card 021) |
| **Movement keywords**: the common add-on "move one of your ambassadors up to 2 hexes" (onto a piece; on Rally); **Spread** (rare: each of your ambassadors on the board may move 1 hex onto an adjacent piece; Diaspora, tier 2); **Consolidate** (move up to N of your ambassadors within your reach onto one piece in your reach; Rally Point, N 3). A moved ambassador arrives as in §2 (cancels, claims, waits) | ambassadors moved only from supply or board into reach | candidate (Matt; the cards 021) |
| **Two presence tracks** a board: placing presence takes the next token from the track the player chooses; each board has a placeholder second track of 3 spaces (§7) | one track of 6 | candidate (Matt; contents placeholders) |
| `adversary.troop_health` (moved from `board`), Unity Network (was Opposition Network), card costs are plain numbers (no `{"param"}`) | | 015's calls 7 and 12 |
| `remove_energy` 1 (was 2) | | candidate (021: tuning, below) |

**Not built in 021** (follow-up): the single `Op` enum and real `Tag` and `Resource` enums
(015's call 2; 025 built them, no rules change); the "count as power" token pattern (023
built it as `gain_per_tokens`); a Cyber token type. The `keywords` field exists (`hack`, `tokens`, `move`, `spread`,
`consolidate`) but nothing reads it.

**Status (019).** Every 019 change is a **candidate** (Matt, 2026-10-29). The first
four rows are Matt's; the rest are 019's adaptations to keep the game winnable once
nodes became hard to remove (results: `experiments/results/screen-019-troops/`).

| 019 rule | Was | Status |
|---|---|---|
| Adversary troops **march** in the adversary phase: 2 steps each (`troop_steps`), a step destroys a target on or next to the troop or moves 1 hex toward the nearest; players choose on ties (§4) | 017's Discord-tip advance and 016's strike in place in production | candidate (Matt) |
| Troop targets: presence and player-owned stations and factories, never cities (§4) | any building not adversary-owned or corrupted, and presence | candidate (Matt) |
| A troop that strikes is used up (kept from 018, `troop_spent` 1); a troop at a base still absorbs a strike next to it | same | candidate (Matt; the screen gave no reason to change it) |
| No standard action removes a node; only tier-2 effects do: Node Takedown (card) and Decommission (Unity card) (§3) | `node_energy` 2 energy, in reach | candidate (Matt) |
| Nodes stop counting against the purge at 2 Unity tips; troops at 1 (`unity_ease` 1, 1, 2; §5) | nodes always counted; troops at 3 | candidate (019) |
| Removing a node sways `alignment.node` 2 Unity | 1 | candidate (019) |
| Freeing a corrupted city sways `alignment.free_city` 3 Unity (023: 5) | 2 | candidate (019) |

**Status (020).** Bug fixes, not design changes; Matt to confirm the first
(`docs/qa-findings.md`).

| 020 fix | Was | Status |
|---|---|---|
| An uncountered Blackout or Sabotage hits the **next** round's production and upkeep (§8) | "this round": threats execute after production and upkeep, so the flag was cleared before anything read it and both threats did nothing but their compute | fix (020) |
| The move standard action is never offered onto the hex the seat's only ambassador already holds (§6) | offered; it charged the energy and moved nothing | fix (020) |
| An adversary ambassador leaks a presence's compute only when the presence is alone on its hex (§2) | the leak took any adversary ambassador on a presence hex: a presence on a corrupted city stopped its spread for good, and one on a clean city or base ate the ambassadors gathering there | fix (020; the code's own comment said "with no other piece") |
| Sabotage only stops upkeep compute (§8) | it also skipped the leak in its country, a data-center-era leftover that became reachable with the first fix | fix (020) |
| Purge the Unaligned's sacrifice offers the seat's own presence tokens (§3, Defaults 44) | it offered the free hexes where the seat could place one, so a "sacrificed presence" removed nothing | fix (020) |

**Status (018).** Every 018 change is a **candidate**. Matt's structural fixes
(2026-10-29) are the first five rows; the rest are 018's adaptations, made because the
fixes alone left every cell at 0% (results:
`experiments/results/screen-018-calibration/`).

| 018 rule | Was | Status |
|---|---|---|
| Only war destruction sways Discord; adversary troop strikes don't (§4) | every destruction | candidate (Matt) |
| A clean city's new factory goes to the player with presence next to its zone; neutral only with none (§4) | always neutral | candidate (Matt) |
| Removing a node: `node_energy` 2 | 3 | candidate (Matt) |
| A corrupted city's factory: next to an adversary ambassador if there is one, else the lowest-numbered free zone (§4) | a four-key order | candidate (Matt) |
| A node next to a site blocks claims on it (Terms) | presence on or next to a node didn't count | candidate (Matt) |
| An adversary troop that strikes is used up, `troop_spent` 1 (§4) | troops stayed | candidate (018) |
| A destroyed presence goes to its owner's reserve, placed before the track, `presence_returns` 1 (§4) | it left the game | candidate (018) |
| Supply `supply_per_country` 14 | 10 | candidate (018) |
| Corrupted cities build `city_build` 1; factories next to them `factory_build` 0 | 2; 1 | candidate (018) |
| Clean cities corrupt at `corrupt_at` 6 | 5 | candidate (018) |
| `free_city_at` 2 player ambassadors free a corrupted city | 3 | candidate (018) |
| Unity easing `unity_ease` 1, 3 (ambassadors stop counting at 1 Unity tip, troops at 3) | 2, 5 | candidate (018) |
| The adversary deck deals `stage_cards` 4, 4, 3: 11 rounds | 3, 4, 3 (10) | candidate (018) |
| Difficulty 1: `extra_threats` 2; difficulty 4: `supply_reduction` 4 (§3) | 1; 2 | candidate (018) |

**Status (017).** Matt marked these **locked**: Alignment cards are one-shot and
hidden, like fear cards; with the Undecided empty, Sway comes from the middle first, then
the other bucket (empty-pool rule B, `tug`); a player ambassador on a military base
sways Unity. The project deck is **near-locked** (Matt, 2026-10-29). Everything else
017 added is a **candidate**.

| 017 rule | Status |
|---|---|
| Alignment cards one-shot and hidden (no ongoing cards) | **locked** |
| Empty-pool rule B: an empty Undecided draws from the other bucket (`tug`, the default) | **locked** |
| A player ambassador on a producing military base sways `base_unity` Unity | **locked** (built since 013; tested) |
| One shared, staged adversary project deck; 3 visible; compute fills the cheapest; a full one resolves; a new row when all 3 have resolved | near-locked (Matt, 2026-10-29) |
| Nodes make 1 compute a round; about half the threats carry compute when they execute uncountered | near-locked (compute sources); the threat amounts are candidates |
| A player ambassador on a node stops its compute (not used up) | near-locked |
| Card primitives: remove compute from a project; discard a visible project (tier 2, very expensive) | near-locked; the two placeholder cards are candidates |
| Project costs (per player) and effects | candidate |
| Each Discord tip advances every adversary troop one hex toward player pieces, then it strikes | candidate (an exception to "no movement rules") |
| No difficulty escalation from Discord tips; Discord cards print stronger steps by Discord tip count | candidate |
| The Undecided smaller than twice a tip (3 a player against a tip of 2 a player) | candidate |
| Unity for freeing a corrupted city (2) and for removing a node (1) | candidate |

**Status (016).** Matt marked one decision **locked**: data centers are player
presence, placed with energy next to presence you already have (§1), and an
adversary-owned factory, or one holding an adversary ambassador, makes an adversary
troop (§4). Everything else 016 added is a **candidate**: the presence track and its
uncovers, reach and site claims, nodes, industrial zones and factory growth, hardware,
the clock, the purge easing.

| 016 rule | Status |
|---|---|
| Presence replaces data centers; 1 energy, next to your presence, 1 per hex | **locked** |
| A factory that is adversary-owned or holds an adversary ambassador makes an adversary troop | **locked** |
| Presence track (6 tokens, 1 on the board) and its uncovers | candidate |
| Reach (card and standard-action map targets within 1 of your presence) | candidate |
| Site claims (presence on or next to the site; a node contests it) | candidate |
| Spread never targets an empty hex; surplus returns to the supply | candidate |
| Nodes (made by a second spreading ambassador; 1 adversary compute a round; contest presence; 3 energy to remove) | candidate |
| Industrial zones; cities grow factories; factory ownership | candidate (Matt, 2026-10-29) |
| Factory troops at every one of the corp's bases | candidate (Matt, 016 message) |
| Peacetime: 3 troops a base, a troop absorbs a strike next to it | candidate |
| Hardware (factories make it; stations, hardening, 4 credits → 1) | candidate |
| The production/war clock | candidate |
| Printed fronts: every country has two at 3 and 4 countries | candidate (Matt, 016 message) |
| The purge and its Unity easing | candidate |

Gone with 017: ongoing Alignment cards (Volunteer Corps and Propaganda are one-shot now),
the Discord escalation of the difficulty in play (`adversary.escalation`,
`alignment.discord_escalate`; replaced by `alignment.discord_tier2`), and the
`Stats.adversary_ops` / `setup_ops` fields (now the generic per-round counter
`adversary.ops`). Gone with 016: data centers and their compute, the AI boards' two tracks, the player
"build a factory" standard action, the `overflow` adversary variant (nodes replace it),
the fixed war rounds (`board.war`; the clock replaces them) and the unused per-country
card strip. Gone with 014: population, processors, drones, outposts, foundries, uplinks,
typed land, defense tokens, the corruption pool, the explore → build → ravage pipeline.
Results and records from revisions 1–6 stay in `experiments/results/` as history; they
do not replay.

## Older curves (moved from rules.md in 073)

**The curve (041, r20, planner, 12 seeds a cell, %, ±14).** 2 countries: d0 100, d1 100, d2 83,
d3 42, d4 25; 3 countries: d0 83, d1 75, d2 50, d3 8, d4 17; at 32 seeds d2 is 88 / 44
(`experiments/results/screen-041-growth/growth.md`).

**The curve (034, r19, planner, 12 seeds a cell, %, ±14).** 2 countries: d0 100, d1 100, d2 67,
d3 17, d4 17; 3 countries: d0 100, d1 58, d2 42, d3 0, d4 0; at 32 seeds d2 is 66 / 38
(`experiments/results/screen-034-clocks/clocks.md`).

**The curve (028 quick screen, r17, 16 games a cell, engine / greedy, %).** 2 countries:
d0 50 / 12, d1 44 / 12, d2 44 / 12, d3 6 / 0, d4 19 / 6; 3 countries: d0 75 / 0, d1 56 / 0,
d2 12 / 0, d3 6 / 0, d4 0 / 0; random 0 everywhere. A hold of 3 is set for the planner (66 / 69
at d2, 32 seeds): one-step agents fall well short of "skilled play wins" at d0, and the curve
under the planner is not measured (`experiments/results/screen-028-rollout/rollout.md`).

**The curve (027 quick screen, r16, 16 games a cell, engine / greedy, %).** 2 countries:
d0 81 / 25, d1 38 / 25, d2 56 / 31, d3 56 / 12, d4 25 / 12; 3 countries: d0 88 / 56,
d1 69 / 44, d2 50 / 19, d3 19 / 0, d4 12 / 0; random 0 everywhere; with noise of about ±12
points, c2's d1 below d2 and d3 is within it. Engine win rounds run from 4 to 11 with no gate
(`experiments/results/screen-027-endgame/endgame.md`).

**The curve (024 quick screen, r15, 16 games a cell, engine / greedy, %).** 2 countries:
d0 94 / 62, d1 81 / 69, d2 69 / 31, d3 38 / 19, d4 38 / 25; 3 countries: d0 94 / 81,
d1 100 / 81, d2 81 / 75, d3 38 / 31, d4 19 / 6; random 0 everywhere; earliest win round 6
in every cell (`experiments/results/screen-024-adversary/adversary.md`). r14's lower numbers
were partly the agent's keep plan (fixed in 024), so the two curves aren't one game made
harder.

**The curve (021 quick screen, r12, 16 games a cell, engine / greedy, %).** 2 countries:
d0 100 / 88, d1 100 / 81, d2 94 / 88, d3 75 / 75, d4 19 / 19; 3 countries: d0 88 / 69,
d1 94 / 50, d2 81 / 31, d3 75 / 31, d4 31 / 6; random 0 everywhere
(`experiments/results/screen-021-cards/cards.md`, with r11's and the untuned r12's).
The skill gap between engine and greedy widened at 3 countries: greedy keeps every card
it drafts and plays few.

**The curve (019 quick screen, r10, 16 games a cell, engine / greedy, %).** 2 countries:
d0 100 / 88, d1 88 / 94, d2 81 / 94, d3 62 / 56, d4 25 / 38; 3 countries: d0 100 / 88,
d1 81 / 81, d2 88 / 69, d3 62 / 56, d4 69 / 50; random 0 everywhere. With 16 games a
cell the noise is about ±12 points, so c3 engine's d4 above d3 and greedy above engine
at c2 d1–d2 are within it. Levers: d1 two extra threats a round; d2 the 10-round deck;
d3 corrupted cities build 2; d4 the supply 4 smaller.
`experiments/results/screen-019-troops/troops.md` (018's r9 curve:
`experiments/results/screen-018-calibration/calibration.md`).

**Why this ladder (014; the supply loss it rests on was retired in 027).** At 014 the supply
loss bounded everything: one corrupted city made about 3 ambassadors a round per country
against a supply of 10, so anything that doubled the build (a second corrupted city) starved
the supply within two rounds. The
corrupted-city-per-country and smaller-supply levels the prompt suggested were first
tried as levels 3 and 4 (second city, supply 7 per country): every game ended in round 1
or 2, before a decision mattered. Levels 3 and 4 were rebuilt from the milder options;
the curve is not tuned (014 quick screen: greedy 100 / 88 / 25 / 0 / 0 % at d0–d3, 16
games each).

## The 014 card remap (moved from rules.md in 073)

History: the placeholder cards as remapped in 014 (data centers became presence in 016).
023 replaced every card in this table except the AI-unique ones (Turnkey Factory, Point
Defense, Shelter Order, Strike Team); it is kept as a record of the old set.

| Card | Was | Now |
|---|---|---|
| Server Farm, Fork Process | place a processor | place a data center |
| Data Mining | compute per processor | compute per data center (max 4) |
| Firewall Node | explore skips your processors | spread skips your data centers |
| Watchtower Net, Precision Strike, Sweep, Orbital Lance, Strike Team | damage | remove adversary pieces worth N (ambassador 1, troop 2); Watchtower in a country with your data center, Strike Team in reach |
| Purge Protocol | destroy every adversary piece in a hex | remove every adversary ambassador and troop in a hex |
| Swarm Override | remove every drone in a country | remove every adversary ambassador in a country (1 Unity per 2) |
| Emergency Fab, Turnkey Factory | build a factory on Industrial land | build a factory on an empty open hex touching a city |
| Rebuild / Reconstruction | return 1 / 3 corruption | take the adversary cube off 1 / **2** corrupted cities or stations (one country) |
| Shore Up / Point Defense / Protector growth | defense tokens | hardening tokens (3 / 2 / 2) |
| Evacuate | move 3 population | move up to 3 of your ambassadors onto pieces in reach |
| Shelter Order, Civic Shelter | population protected | facilities protected |
| Scavenge | if you destroyed a foundry | if you removed an adversary troop |
| Rally | 2 Unity if population and adversary share a hex | gain 1 ambassador; 2 Unity if a clean city holds adversary ambassadors |
| Recruitment Hub | (giving bonus) | also gain 2 ambassadors when built |
| Unity Network | +1 Unity per outpost/foundry destroyed | +1 Unity per adversary troop removed |

# round-economy: engine-verified rules (172 verifier)

**Setup (resources)**
- Each player gets one country (player n gets country n, so player count = country count, 1-4; `common.rs:161`). Each AI board starts with credits by board (`ai.start_credits`: Builder 40, Protector 46, Researcher 40), 3 energy (`ai.start_energy`), 1 ambassador in supply (`board.start_ambassadors`), a node on the country's printed start hex, and the clean city nearest that node is already yours (your cube on it) (`ai.rs:380-385`, `board/mod.rs:290-305`). The first space of the first node track starts uncovered.

**The round, in engine order** (`configs/baseline.json` `round`; `rules.rs:716-783`)
0. Round start (`round_start` hooks in module order): your antimatter action is ready again; **insight is set** (below); your AI-board powers refresh; your exhausted cards refresh, and your keep count resets.
1. Sway: the adversary board's printed drip toward Discord (the Militarist prints none) plus 1 Discord (`adversary.blight_discord`) per land-corruption token (`alignment.rs:523-530`).
2. The face-up event card becomes this round's (none left: the adversary wins).
3. Threats are dealt from its pulls.
4-6. Adversary build (Rally, Deploy, its lines, ongoing projects, botnet compute); Corrupt X lines; natural corruption.
7. Adversary troops march (`board.troop_steps` 2 steps each).
8. Next event card revealed; adversary income (compute, agents to pools); every firewall comes off.
9-12. Cards: round 1 deals the opening hand and you keep; later rounds build a 4-card hand tier by tier, draft, keep.
13. **Action phase**, simultaneous: play cards, use card Actions, AI-board powers, standard actions and projects, megaproject converters, counter threats, the antimatter action, in any order. You may act until you pass; once you pass you are out for the phase; it ends when everyone has passed (`rules.rs:615-626`).
14. Production, part 1 (`production.rs:81-85`): (a) bases make extra corp troops; (b) each factory produces: a player's factory gives its owner 2 chips (`board.factory_chips`), an adversary factory makes 1 troop, a neutral one nothing; an agent on a player's factory, or a player ambassador on an adversary factory, blocks it; a troop occupying it blocks it; (c) each city grows a factory.
15. Production, part 2 (`production.rs:300-305`), in this order: (a) city energy; (b) research stations; (c) node drain; (d) full megaprojects complete.
16. **Upkeep** (`economy.rs:221-247`), player by player: collect income (everything except insight), pay building upkeep, then the Open Science capstone hands out its insight.
17. Threats: partial payments refunded, uncountered threats execute.
18. War on the fronts.
19. Cleanup: country/hex flags count down; unused Movement is lost.
After every phase and every action, tipping and the end checks run.

**Insight (expires; set at round start)** (`economy.rs:189-219`)
- At round start your insight becomes: board insight (5 on every board) + insight on uncovered track spaces (the first one, +1, is uncovered from setup, so normally 6 in round 1) + insight production from programs + team insight from Unity cards + any Open Science gift, minus your insight losses, never below 0. Anything unspent is discarded.
- Patience (Researcher): unspent insight you earned yourself carries over and is added on top. Insight another player gave you this round does not carry.
- Insight losses taken this round count against next round: 1 per hex where your node stands alone (no city, base, station or factory on that hex) with at least one agent on it (`production.rs:434-461`; that hex also gives the adversary 1 compute), and threat/project losses (Smear Campaign 4; Data Heist's "3 less next round").
- Mid-round: 1 research cube -> 1 insight (`board.cube_insight`), no limit; card triggers, board powers, converters.
- Spent on: keeping cards (tier 1: 1, tier 2: 2, tier 2 after the keep upgrade: 1, reduced by Science tags), playing cards (0-2 printed), threat counters. If you are short for a **keep** only, each missing insight is paid with 5 credits (`cards.insight_credits`; `cards/play.rs:9-16`).

**Credits (carry over)**
- In upkeep: the board's credit production (Builder 15, Protector 13, Researcher 11, printed on the board), credit income on uncovered track spaces, card income and programs' production, and the team credit bonus from Unity cards (`economy.rs:133-158`). Also: selling a card from hand (+1, `board.sell_credits`), megaproject payouts, cards.
- Spent on: card costs, opening keeps past the 4 free (5 each), insight shortfalls on keeps (5 per insight), buying a neutral city's energy (5), standard projects, Clear, Buy In, threat counters.

**Energy (carries over)**
- In upkeep: board energy (Builder 1, Protector 2, Researcher 1), uncovered track energy, energy production on programs and building income.
- In production: your cities' energy (below). Also card gains, Overcharge, Horizon's payout.
- Spent on: every node placement (the energy printed on the track space uncovered) and node powers, energy-costing powers, card energy costs, card Actions, threat counters.

**City energy (production step 15a; `production.rs:308-381`)**: each city makes `board.corp_energy` (1) energy. Checked in this order for each city:
1. Under a blackout: nothing.
2. Adversary city: its energy becomes Discord (1 per energy, `board.divert_discord`), unless a player ambassador stands on it, which blocks that.
3. Any agent on a neutral or owned city: the city is dark and makes nothing.
4. Owned city: its owner gets the energy free (+`board.aligned_bonus` 0).
5. Neutral city: one player who has a node on it or next to it and holds 5 credits may buy all its energy for 5 credits (the players choose who, or nobody); otherwise it is lost.

**Chips (carry over)**: your factories (2 each, step 14), chip income in upkeep (cards, the Builder's Full Capacity capstone), Mint, Scrap, Onyx's converter, cards. Spent on station building (2), some buildings' chip cost, Procure, card Actions. Chips can't be sold.

**Research cubes (carry over)**: in step 15b each research station produces. A player's station makes 2 (`board.owned_station_cubes`), a neutral one 1 (`board.station_cubes`). Before its country's megaproject completes, the cubes go on the megaproject in the station's color. Afterwards, a player's cubes go to the owner's board and a neutral station's are lost. An agent on a neutral or player station blocks it. An adversary station makes compute unless a player ambassador blocks it. The Open Archive capstone gives +1 cube in upkeep. Spent on: the cube->insight action, megaproject converters (once complete, with your node on a city of that country), Peer Review, Forensic Audit.

**Antimatter (carries over)**: the antimatter action (once a round: +1 antimatter, +1 Discord), cards, Particle Run, Horizon's converter, income on cards. Spent on tier-2 cards.

**Ambassadors (pieces)**: start 1; from cards, ambassador income in upkeep, and the 20-credit project.

**Movement (resets each round)**: board Movement (Builder 1, Protector 1, Researcher 2) + uncovered Movement spaces (`ai.rs:102-104`). 1 per Move, and unused Movement is lost in cleanup.

**Standard actions and projects** (step 13, any number, each needs a legal target; "in reach" = on or next to your node; `board/standard.rs`)
- Build a research station: 2 chips; an open, building-free, non-industrial hex in reach. It is yours.
- Remove agents: 1 ambassador from supply onto a hex in reach that has agents; removes up to 3 and stays.
- Clear a corruption marker in reach: 2 credits; 2 Sway toward Unity.
- Move: 1 Movement; one of your ambassadors (supply or any hex) onto any piece in reach other than the hex it leaves. If an agent is there, the two cancel (both removed).
- Gain an ambassador: 20 credits.
- Claim a neutral city / station / factory in reach (no botnet on or next to it, no embargo): 4 / 3 / 3 ambassadors from supply, +1 per agent on it. The agents cancel and your cube goes on.
- Liberate an adversary city / station / factory in reach: 5 / 3 / 3 ambassadors, +1 per agent. The ambassadors already on the site pay first (yours, then other players'), and your supply pays the rest. It turns neutral and sways Unity 5 (city) / 2 (facility).
- Harden (chips): only if a card grants it (none does).
- Project: harden a player-owned, uncorrupted station or factory in reach: 3 credits.
- Project: drain up to 3 compute from one visible adversary project: 5 credits.
- Cube -> 1 insight.
- Envoy: 1 ambassador, used up; 1 Discord off the Discord track (only offered while Discord > 0).
- Sell a card from hand: +1 credit.

**Upkeep detail**: income per resource, less any income loss a threat put on it. Upkeep energy income is 0 if you have a node in a country under Sabotage (`economy.rs:95-111`; no content raises this flag now). Building upkeep: no current card has one. No resource has a holding limit or a cap.

## Player count and difficulty

Player count: one player per country, 1-4 (seats = `countries`, `rules.rs:342-350`; home country = seat number, `common.rs:161`). Boards cycle `ai.boards` [0,1,2]: 1 player Builder; 2 Builder+Protector (baseline `countries` 2); 3 adds Researcher; 4 adds a second Builder. Starting credits and slots go by board, not count (40/46/40; 5/4/3). Every country's cities make 1 energy (`corp_energy` [1] repeats). Solo: the Open Science capstone's "other players" means you, so its 2 insight comes to you. Under Patience that gift then carries over, because a solo gift doesn't count as received (`economy.rs:166, 209`). Card help ops that transfer to another player aren't offered solo (`effects/feasible.rs:123`). With more players there is more competition for neutral city energy (one buyer per city, players choose). Difficulty: nothing in economy.rs, standard.rs or production reads difficulty. Indirect effects: at level 4 `adversary.extra_corrupted` (0 at baseline) more cities start corrupted, so fewer neutral and sellable cities (`board/mod.rs:275`). Counter costs rise at `counter_bonus_at` 1 (+1 each resource item, +4 credits) and `counter_more_at` 2 (+1 more) (`threats/counter.rs:52-62`), which raises the price of countering threats.

## Mismatches (engine vs docs/rules)

- **Phase 1 `alignment.sway`: 'AI boards sway'** (docs/rules/economy.md, The round table row 1): engine: The adversary board's printed drip toward Discord (Militarist prints none) plus `adversary.blight_discord` (1) Discord per land-corruption token; AI boards do nothing in this phase (crates/aic-core/src/modules/alignment.rs:47-50, 523-530)
- **Phase 14 order: factories make chips/troops, extra corp troops, cities grow factories** (docs/rules/economy.md, The round row 14): engine: Extra corp troops first, then factory output, then cities grow factories (crates/aic-core/src/modules/board/production.rs:81-85)
- **Insight is 'less 1 per agent that drained your nodes'** (docs/rules/economy.md, resource table, Insight row): engine: Loses 1 per hex holding your node and at least one agent (any number of agents counts once), and only for a lone node: a node on a city, base, station or factory hex is never drained; each drained hex also gives the adversary 1 compute (crates/aic-core/src/modules/board/production.rs:440-455)
- **Insight is less 'what Smear Campaign and Deepfake Wave took'** (docs/rules/economy.md, Insight row): engine: Deepfake Wave takes no insight (sways 3 Discord, pulls 2 Unity). Next-round insight loss comes from Smear Campaign (lose_insight 4) and the Data Heist project (requisition 3 insight next round) (content/threats.json:1311-1345; crates/aic-core/src/modules/effects/new_ops.rs:446-449; crates/aic-core/src/modules/effects/mod.rs:257-262)
- **'A Blackout stops a country's cities in the production after the threat executes.'** (docs/rules/economy.md, Cities' energy, for new players): engine: The Blackout threat makes the nearest player pay 3 energy or discard 3 cards and sets no flag. The blackout flag comes only from an event card's standing line and the Rolling Blackouts project, both 'everywhere' (every country). It is read in city energy (content/threats.json:822-853; content/adversary.json:694-698; content/projects.json:349-353; crates/aic-core/src/modules/board/production.rs:322)
- **Upkeep collects income (no exceptions stated)** (docs/rules/economy.md, The round row 16; resource table Energy row): engine: A seat with a node in a country under the Sabotage flag gets 0 upkeep energy income, and threat income losses (income_debt) reduce upkeep income. The doc omits both. No content sets Sabotage today, so that part is dormant (crates/aic-core/src/modules/economy.rs:95-111)
- **Phase 8 reveal: next card face up; adversary income** (docs/rules/economy.md, The round row 8): engine: Also removes every firewall at that point (crates/aic-core/src/modules/adversary.rs:61, 501)
- **Chip sources list 'Antitrust Action's chips per 2 Industry tags' beside chip income** (docs/rules/economy.md, Chips row): engine: Antitrust Action is a one-time gain_per_tag on play, not income (content/cards.json antitrust_action (gain_per_tag); crates/aic-core/src/modules/economy.rs:133-158)

## Hard to state

- [high] **Income arrives at four different moments: insight at round start, chips from factories in production step 14, energy and research cubes in step 15, all other income (credits, board/track/card energy, chips, antimatter, ambassadors) in upkeep after production**: A player can't predict from the board when each resource arrives. Energy comes in two places (cities in production, everything else in upkeep), and so do chips (factories, then upkeep) Proposed: One 'Income' step at round end that pays everything in a printed order (factories, cities, stations, then board/cards), with insight set right after it as next round's
- [high] **Insight reset with Patience, gifts, and next-round losses**: Insight expires, unless you are the Researcher, but gifted insight still expires, and the Open Science gift counts as 'received' except when you are alone. The losses are bookkept on a 'debt' that a human must track across the round boundary Proposed: Patience carries all unspent insight. Losses are tokens placed on your board, removed when next round's insight is set
- [medium] **Node drain: 1 insight per hex where your node is alone with an agent, not on a city/base/station/factory**: Exception on the site type, counts hexes not agents, and is paid next round Proposed: Each agent on a hex with your node costs you 1 insight next round, wherever it is
- [medium] **City energy resolution: blackout, then adversary (ambassador blocks diversion), then dark if any agent, then owned, then neutral sale**: Five-way priority. A player ambassador blocks an adversary city, but an agent darkens yours Proposed: Print the priority on the player aid as a numbered list, and state it as 'opposing piece blocks, adversary city diverts, otherwise owner takes or sells'
- [medium] **Buying a neutral city's energy happens in production, needs 5 credits held then and a node on or next to it; one buyer per city, the players choose**: It happens after the action phase, so you must remember to keep 5 credits unspent. Rival buyers must agree, and the engine asks seat 0 Proposed: Make it a standard action in the action phase ('buy a neutral city's energy in reach: 5 credits, once per city a round')
- [low] **Keep shortfall paid at 5 credits per insight, keeps only, not plays**: An exception to 'insight can't be bought' Proposed: Either allow it for plays too, or drop it and let opening keeps be the only credit keeps
- [medium] **Liberate spends ambassadors already on the site first, including other players'**: Your action spends another player's pieces. A player would not predict this Proposed: Spend only your own ambassadors (those on the site first, then supply)
- [medium] **Ambassadors meeting agents: Move cancels 1-for-1 (both removed), Remove evicts up to 3 and the ambassador stays, Claim cancels all agents for +1 ambassador each**: Three different exchange rates for the same contact Proposed: One rule: an arriving ambassador removes up to 3 agents and stays. Claim/Liberate add +1 per agent
- [low] **Claim needs no botnet on or next to the site and no embargo, but Liberate only needs reach**: Unless-clauses that differ between two sibling actions Proposed: Same reach test for both. Let botnets add cost instead of forbidding
- [low] **Round 1 insight is 6, not the printed 5 (the starting track space is uncovered)**: Poorly telegraphed: the printed number isn't the number you get Proposed: Print 6 on the board, or leave the first space without income
- [low] **Sabotage zeroes upkeep energy income; building upkeep step; chip-paid Harden action**: Rules with no content behind them: a printed rule nobody can trigger Proposed: Delete them from the rulebook (and round table) until a card uses them
- [low] **Name collision: Blackout threat (pay energy) vs the blackout flag (cities make nothing)**: The same word does two unrelated things Proposed: Rename the flag 'Grid down' or the threat 'Commandeered power'

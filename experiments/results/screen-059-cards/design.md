# 059: the card design pass (r25)

Every card in content (226) read as an experienced hobby player would, against
[docs/card-guidelines.md](../../../docs/card-guidelines.md). Started from 055's prices. `review/card-notes.json`
does not exist in the main checkout, so no note of Matt's could be addressed.

## Results

No card face prints a `*`, a cap, a footnote or a sum (`no_face_prints_an_exception` holds it). Rules text is
at 7.5 pt on 156 of 158 player cards, against 88 at 055; every title sits on one line in its plate.

| Read | planner c2 | planner c3 | engine c2 | engine c3 |
|---|---|---|---|---|
| Band, 48 fresh seeds (`--seed 11000`) | 73 | 65 | 62 | 52 |

**The gate** (gate-r23, 8 seeds a check, `--seed 12000`): 428 of 512 games, 26 min on 8 threads, 0 panics. Not passed by
the tool's reading, which fails any check level or ahead on the sign, and for 055's reason: the planner itself read 50 at
both sizes on those 8 seeds (73 / 65 at 48). So each check level or ahead went to 48 seeds, paired on the band's seeds
(`experiments/archive/gate-close-r25-c2.json`, `-c3.json`). Gap = planner minus check, ± one SE.

| check | c2 | c3 | 055 (c2 / c3) |
|---|---|---|---|
| always-Consolidate | +21 ±9 | -6 ±9 | -2 / -8 |
| tier-2-rush | +8 ±7 | -2 ±9 | -8 / +4 |
| no-powers | +2 ±9 | -4 ±8 | -6 / -4 |
| ignore-Science | +8 ±9 | +2 ±10 | +8 / |
| ignore-Industry | +6 ±9 | | +15 / |
| ignore-Cyber | +2 ±8 | | +23 / +17 |
| ignore-Security | | +35 ±10 | +27 / |
| science-rush | | +21 ±9 | |
| only-Power | +56 ±8 | | |

No check is ahead beyond one SE: the gate holds at 053's and 055's reading. Three stay level at c3, as they did at 055.
No-powers is AI-board work. Ignoring Cyber at c2 went from +23 to +2 ±8, the one step back here; the cause wasn't traced, and it
is the first thing to look at in the next price pass.

Band and gate ran on the 10:21pm build. Two small fixes came after it, on the same frozen content: a sweep is no longer
offered with none of the pieces it names, and corruption replaces a marker on a site. The band's seeds were replayed on the
final build, then the gate's: all 192 and all 428 games came out the same in every recorded column.


## Still below 7.5 pt

| card | size | reason |
|---|---|---|
| Agency Directorate | 6.3, above the art | Its three parts (tokens, cost, a per-hex sweep) need three lines. Any shorter loses the country sweep, the card's point |
| Kinetic Interdiction | 6.3, below the art | The choice of pieces on a sweep is the widest icon line in the set and wraps to a second row |

## Levers

Paired on the seeds that picked them, then the band above on fresh seeds. Planner wins.

| Build | c2 | c3 | |
|---|---|---|---|
| Schema and the building cuts | 12 of 24 | 18 of 24 | before the review |
| The review's changes, all | 15 of 24 | 8 of 24 | c3 fell, on presence |
| Those, nine price raises put back (G4) | | 15 of 24 | **shipped** |
| Those again, new seeds | 28 of 40 | 17 of 40 | |
| With Kinetic's two kinds and Total Containment's shelter back | | 27 of 40 | **shipped** |
| With Beamed at 8, Grid Census at 2 | | 24 of 40 | **shipped** |
| With protection back (shields on Mesh, Open-Weights; hardens on two cards) | | 23 of 40 | not shipped: three effects a card |
| With moves back on four cards | | 21 of 40 | not shipped: riders |
| All shipped, the same seeds | 28 of 40 | 25 of 40 | |

## Flagged, left as they are

| card | reason |
|---|---|
| Foundation Model Campus, Hyperscale Botnet, Exabyte Archive, Data Licensing Deal, Recursive Self-Improvement | Kept under 10% of the time even after the price cuts: tier-2 buildings and income still lose to one-shots, 055's finding. Their fix is structural, so no further cut was tried |
| Manifesto Leak, Recruit Operatives, Collateralized Grid | Need a token building in play; kept under 10%. Fine as catch-up cards, weak as draft picks |
| Adaptive Malware, Amnesty Decree, Liberation Accords, Neural Cartographer, Mutual Aid Pact, Peacekeeping Detail | Review suggested renames (Twin-Payload Malware, Defector Amnesty, Withdrawal Accords, Neural Dropout, Relief Convoy, Gate Guard Detail). Left: the name still reads, and a rename strands its art |
| Art now behind its card | Stress Test Suite, Early Warning Drill, Reclamation Crew, Shadow Broker Scrub, Aegis Dome Array, Telemetry Archive, Sentinel Mesh, Exabyte Archive: the scene shows the old idea. No new art made: art work wasn't in this item |
| British spellings | Mobilisation, Armoured, Modelling left as they are: one spelling for the game is Matt's call |
| Threat, adversary, Discord and AI-board text | Placeholders pending redesign: reworded, not redesigned |

## Design calls for Matt

1. **"Harden" and "building" each mean two things.** A building is a kind of card and also a station or factory on
   the board; "harden" covers a building and a presence hex, and the icon line now shows which. Options: (a) leave
   it; (b) a rulebook term, **facility** (a station or factory): "harden 2 facilities in reach", 1 character longer.
   **Pick (b):** a new player trips on it the first time they harden with a building in hand. It needs a word in
   the rulebook, so it is not done.
2. **House wording as rulebook terms.** Cards now lean on these: "ambassador" alone is yours, "troop" is always the
   adversary's, "in reach", "here" on a building, "Draw" is always tier 1, "Remove N" means up to N. They are in
   `docs/rules.md` and the guidelines. Options: keep them as rules text, or print a reference line on the player aid.
   **Pick: both:** with the rest of this pass they took the cards from 2,674 printed words to 1,693, and the aid is where a player looks.
3. **"Harden action".** "You gain the Harden action" replaces "you may Harden as a standard action". Options:
   keep, or name it a keyword. **Pick: keep,** it is the same action the Protector has.
4. **A conditional reward is the one exception that was fun.** Bounty Board and Breach and Clear rewarded removing a
   troop this round: a small plan across a turn. A keyword could bring that back without a `*`: **Bounty: gain 3
   credits when you remove a troop this round** as a building Effect, which is a sentence, not a footnote. Options:
   (a) leave them plain, as now; (b) add that Effect as a new op on one building. **Pick (b)** if Matt wants a
   troop-hunt plan in Security: plain text, and it is decided by a play, not read off a list.

## The other pools

Unchanged in size. Ops untouched, so no price moved. Old and new text, card by card: [wording.md](wording.md).

## Every player card that changed

134 of 158; 91 changed in what they do or cost, the rest in words only.

| card | problem | change: new text | price |
|---|---|---|---|
| Turnkey Factory | wordy text | Phrase, in `face.rs`: Each round, 1 credit. Build a factory in reach. | none |
| Modular Expansion | Footnote slot exception | Gives 2 slots and sits in one: Effect: you have 2 more building slots. | none |
| Point Defense | footnote text | Phrase, in `face.rs`: Action: pay 1 energy, harden 2 buildings in reach. | none |
| Strike Team | "worth" text | Phrase, in `face.rs`: Remove 3 adversary ambassadors or 1 troop from a hex in reach. | none |
| Telemetry Archive | Worse Grad Student Pool | Redesign: Action delays a threat: Each round, 1 Dataset here. Action: pay 2 Datasets, delay 1 threat one round. | 3 cr, 1 ins → 2 cr, 1 ins |
| Simulation Cluster | wordy text | Phrase, in `face.rs`: Action: pay 3 credits, draw 1 card. Pay 1 chip. Draw 1 card. | none |
| Citation Engine | Cap | Per 2 tags, no cap: the safe rate: Each round, 1 credit per 2 Science tags you have. | 4 cr, 1 ins → 3 cr, 1 ins |
| Breakthrough Leak | wordy text | Phrase, in `face.rs`: Draw 3 cards. | none |
| Zero-Knowledge Exploit | wordy text | Phrase, in `face.rs`: Remove 2 compute from an adversary project. Draw 1 card. | none |
| Open Preprint | wordy text | Phrase, in `face.rs`: Another player gains 1 insight. Draw 1 card. | none |
| Grad Student Pool | Rider; too cheap | Rider cut, dearer: Each round, 1 Dataset here. Action: pay 2 Datasets, draw 1 card. | 3 cr, 1 ins → 4 cr, 1 ins |
| Stress Test Suite (was Benchmark Suite) | Name says nothing of the shield | Renamed: Gain 1 insight. Harden 2 of your presence. | none |
| Dataset Scraper | Rider | Credit cut: Add 3 Datasets to one of your Dataset buildings. | none |
| Literature Crawler | Rider | Peek cut: Draw 1 card. | none |
| Shared Compute Grant | Paid to give insight away; move unrelated | Redesign: a plain gift of 3: Another player gains 3 insight. | 2 cr, 1 ins → 1 cr, 1 ins |
| Predictive Modeling | wordy text | Phrase, in `face.rs`: Look at the next card of the adversary deck. Delay 1 threat one round. | none |
| Neural Cartographer | footnote text | Phrase, in `face.rs`: A node in reach makes no compute next build. Draw 1 card. | none |
| Cryo-Compute Burn | Matt: timing and cap | Fixed trade, once, no cap: Pay 2 energy. Gain 2 insight. | 1 cr, 0 ins → 1 cr, 2 en, 0 ins |
| Field Sensor Grid | A "max 1" meaning 1 | Gain 1 insight: Clear 2 corruption markers in reach. Gain 1 insight. | none |
| Endowed Research Chair | wordy text | Phrase, in `face.rs`: Lower your credit income by 1. Raise your insight income by 1. Draw 1 card. | none |
| Red-Team Audit | wordy text | Phrase, in `face.rs`: One threat needs 1 less of each resource. Draw 1 card. | none |
| Synthetic Data Forge | wordy text | Phrase, in `face.rs`: Each round, 2 Datasets here. Action: pay 4 Datasets, get 2 insight. | none |
| Exabyte Archive | Free rate pressed every round | Redesign: spend Datasets for credits: Each round, 2 Datasets here. Action: pay 2 Datasets, get 5 credits. | 8 cr, 1 ch, 2 ins → 5 cr, 2 ins |
| Foundation Model Campus | Rider; played in 1% of games | Draw cut, cheaper: Each round, 2 insight, plus 1 credit per Science tag you have. Pay 2 chips and 1 antimatter. | 15 cr, 2 ch, 1 am, 2 ins → 8 cr, 2 ch, 1 am, 2 ins |
| Orbital Observatory Array | Footnote threshold; income, tokens and Action | Redesign: 2 insight, Action buys cards: Each round, 2 insight. Action: pay 3 credits, draw 2 cards. Pay 1 chip. | 9 cr, 1 ch, 2 ins → 8 cr, 1 ch, 2 ins |
| Recursive Self-Improvement | Too many effects; played in 1% of games | Income cut gone, cheaper: Pay 1 antimatter. Raise your insight income by 2. Draw 3 cards. | 10 cr, 1 am, 2 ins, -1 credits income → 8 cr, 1 am, 2 ins |
| Data Licensing Deal | Two capped rates | Redesign: sell all Datasets at 3: Remove all the Datasets you have: gain 3 credits per Dataset removed. | none |
| Open-Weights Release | Three effects, a bundle | Clear 3 and a gift: Clear 3 corruption markers in reach. Another player gains 2 insight. | 4 cr, 1 ins → 8 cr, 1 ins |
| Adversarial Inference Sweep | Three effects. #2190: needed Science for the niche | Draw cut, needs Cyber: Remove 1 compute from each adversary project. Delay 2 threats one round. | 5 cr, 1 ins → 2 cr, 1 ins |
| Retained Earnings | Too good beside the plain rate | 1 credit dearer: Each round, 1 Capital here. Action: pay 1 Capital, get 1 chip. | 2 cr, 0 ins → 3 cr, 0 ins |
| Expansion Fund | Rider | Rider cut, price down by it: Each round, 1 Capital here. Action: pay 3 Capital, build a factory in reach. | 3 cr, 1 ins → 1 cr, 1 ins |
| Prefab Plant Kit | wordy text | Phrase, in `face.rs`: Build a factory in reach. Gain 2 energy. | none |
| Chip Spot Market | Cap, rate read as standing | Fixed trade: Pay 3 chips. Gain 6 credits. | 0 cr, 0 ins → 0 cr, 3 ch, 0 ins |
| Reclamation Crew (was Station Prefab Order) | Name says build, card clears | Renamed: Free 1 corrupted station or factory in reach. Gain 2 chips. | none |
| Collateralized Grid | footnote text | Phrase, in `face.rs`: Gain 1 energy per 2 Capital you have. | none |
| Joint Fabrication Pact | Too cheap for its value | 1 insight: Gain 2 chips. Another player gains 2 chips. | 1 cr, 0 ins → 1 cr, 1 ins |
| Supply Chain Treaty | Rider | Move cut: Another player gains 3 credits. Raise your credit income by 1. | none |
| Vendor Financing | Footnote rider | Bonus cut, 1 more Capital in its place: Give another player up to 4 credits. Add 3 Capital to one of your Capital buildings. | none |
| Materials Hedge Desk | Matt: arithmetic | Plain rate: 1 chip a Capital: Remove all the Capital you have: gain 1 chip per Capital removed. | 3 cr, 1 ins → 1 cr, 1 ins |
| Liquidity Event | Matt: arithmetic, cap | Plain rate: 2 a Capital: Remove all the Capital you have: gain 2 credits per Capital removed. | none |
| Grid-Tied Smelter | Cap, rider | No cap, rider cut: Each round, 1 credit per Power tag you have. | 4 cr, 1 ins → 5 cr, 1 ins |
| Bonded Warehouse | Income, tokens, Action, rider | Action turns Capital into credits: Each round, 1 Capital here. Action: pay 2 Capital, get 3 credits. Gain 2 chips. | none |
| Arcology Foundry | Played in at most 1% of games | Cheaper: Each round, 5 credits and 2 chips. Pay 2 chips. | 16 cr, 3 ch, 2 ins → 12 cr, 2 ch, 2 ins |
| Orbital Mass Driver | Two ideas, four costs | Discount cut, cheaper: Each round, 7 credits and 2 chips. Pay 3 chips and 1 antimatter. | 25 cr, 4 ch, 1 am, 2 ins → 18 cr, 3 ch, 1 am, 2 ins |
| Arc Furnace Complex | Forced conversion, read as a rule | Action, your choice: Each round, 1 chip. Action: pay 2 energy, get 6 credits. Pay 2 chips. | none |
| Sovereign Capital Fund | Antimatter too cheap at a Capital rate | Action 4 Capital: Each round, 2 Capital here. Action: pay 4 Capital, get 1 antimatter. | 7 cr, 2 ins → 8 cr, 2 ins |
| Megacorp IPO | Arithmetic, cap, rider | Plain rate: 4 a Capital, rider cut: Remove all the Capital you have: gain 4 credits per Capital removed. | 3 cr, 2 ins → 5 cr, 2 ins |
| Compute Siphon | wordy text | Phrase, in `face.rs`: Remove 6 compute from an adversary project. | none |
| Blackout Script | footnote text | Phrase, in `face.rs`: A node in reach makes no compute next build. Remove 3 compute from an adversary project. | none |
| Eviction Worm | "worth" text | Phrase, in `face.rs`: Remove 2 adversary ambassadors or 1 troop from a hex in reach. | none |
| Logic Bomb | "worth" text | Phrase, in `face.rs`: Remove 6 compute from an adversary project. Remove 1 adversary ambassador in reach. | none |
| Honeypot Relay | Too dear for a tier-1 Action | Action 1 credit: Action: pay 1 credit, remove 1 adversary ambassador in reach. | none |
| Shadow Broker Scrub (was Shadow Broker Feed) | Name says peek, card clears | Renamed: Clear 1 corruption marker in reach. Draw 1 card. | none |
| Shared Exploit Kit | Three effects | Credit cut: Remove 3 compute from an adversary project. Another player gains 1 insight. | 3 cr, 0 ins → 4 cr, 0 ins |
| Zero-Click Breach | footnote text | Phrase, in `face.rs`: Remove 8 compute from an adversary project. A node in reach makes no compute next build. | none |
| Neural Mirror | Near-duplicate of Endowed Research Chair | Income cut gone, dearer: Raise your insight income by 1. | 4 cr, 1 ins, -1 credits income → 7 cr, 1 ins |
| Rootkit Cache | footnote text | Phrase, in `face.rs`: Action: pay 1 credit, a node in reach makes no compute next build. | none |
| Spoofed Credentials | Rider | Credit cut: Remove 1 adversary ambassador in reach. | none |
| Adaptive Malware | Footnote condition | Redesign: drain plus a jam, answers both clocks: Remove 3 compute from an adversary project. One threat needs 1 less of each resource. | none |
| Flash Exploit | wordy text | Phrase, in `face.rs`: Remove 5 compute from an adversary project. | none |
| Signal Hijack | Two removals, too long | One bigger removal: Remove 3 adversary ambassadors or 1 troop from a hex in reach. | none |
| Intrusion Lab | Too dear | Cheaper: Action: pay 1 energy, remove 2 compute from an adversary project. | 5 cr, 1 ins → 4 cr, 1 ins |
| Darknet Exchange | footnote text | Phrase, in `face.rs`: Each round, 1 credit per 2 Cyber tags you have. | none |
| Sentinel Mesh | Two hacks, 134 characters | Redesign: repeatable jam: Action: pay 1 energy, one threat needs 2 less of each resource. | 5 cr, 1 ins → 4 cr, 1 ins |
| Quantum Decryptor | Footnote threshold | Insight per 2 Science tags: Each round, 1 credit, plus 1 insight per 2 Science tags you have. Pay 1 chip. | none |
| Killswitch Directive | wordy text | Phrase, in `face.rs`: Discard an adversary project. | none |
| Node Severance | wordy text | Phrase, in `face.rs`: Remove a node in reach. | none |
| Cascade Failure | wordy text | Phrase, in `face.rs`: Remove 5 compute from each adversary project. | none |
| Mesh Intrusion Pact | Three effects | Shield cut: Remove 3 compute from each adversary project. Another player gains 2 credits. | 7 cr, 1 ins → 6 cr, 1 ins |
| Singularity Exploit | Too long | Redesign: discard, not drain: Pay 1 antimatter. Remove a node in reach. Discard an adversary project. | none |
| Black Ice Citadel | Income plus Action; dear | Redesign: drains every project: Action: pay 1 energy, remove 2 compute from each adversary project. Pay 2 chips. | 9 cr, 2 ch, 2 ins → 6 cr, 2 ch, 2 ins |
| Hyperscale Botnet | footnote text | Phrase, in `face.rs`: Each round, 1 insight, plus 1 credit per Cyber tag you have. Pay 1 chip. | none |
| Baseload Contract | Too dear (055) | Cheaper: Raise your energy income by 1. | 4 cr, 1 ins → 3 cr, 1 ins |
| Hardened Conduits | footnote text | Phrase, in `face.rs`: Pay 1 energy. Harden 3 buildings in reach. | none |
| Demand Response Grid | Too dear | 6 credits an Action: Action: pay 2 energy, get 6 credits. | none |
| Grid Census | Cap | No cap: Gain 1 energy per presence you have on the board. | 1 cr, 1 ins → 2 cr, 1 ins |
| Transmission Corridor | Three effects | Harden cut: Place 1 presence. Move 1 ambassador on the board up to 2 hexes. | none |
| Blackstart Protocol | wordy text | Phrase, in `face.rs`: Gain 2 energy. Harden 1 building in reach. | none |
| Substation Hub | Footnote threshold | Credit per 2 Industry tags: Each round, 1 energy, plus 1 credit per 2 Industry tags you have. | none |
| Faraday Yard | footnote text | Phrase, in `face.rs`: Each round, 1 energy. Action: pay 1 energy, harden 2 buildings in reach. | none |
| Storm Hardening | Near-duplicate price | Cheaper: Pay 2 energy. Shelter one country this round. Harden 2 buildings in reach. | 1 cr, 2 en, 1 ins → 0 cr, 2 en, 1 ins |
| Surplus Auction | Cap, rate read as standing | Fixed trade: Pay 2 energy. Gain 3 chips and 1 credit. | 0 cr, 0 ins → 0 cr, 2 en, 0 ins |
| Positron Storage Ring | Conversion plus income plus costs | Redesign: 1 energy and 1 antimatter a round: Each round, 1 energy and 1 antimatter. | 11 cr, 2 en, 1 ch, 2 ins → 11 cr, 2 ins |
| Dyson Swarm Tranche | Five costs; played in 1% of games | One extra cost, cheaper: Each round, 5 energy and 2 credits. Pay 1 antimatter. | 16 cr, 3 en, 2 ch, 1 am, 2 ins → 12 cr, 1 am, 2 ins |
| Continental Supergrid | Cap; two ideas | No cap, Harden action cut: Each round, 1 energy per Power tag you have. | 10 cr, 1 ch, 2 ins → 11 cr, 2 ins |
| Beamed Power Constellation | Cap | No cap: Place 3 presence free. Gain 1 energy per presence you have on the board. | 7 cr, 2 ins → 8 cr, 2 ins |
| Antiproton Offtake Deal | Too dear | Energy cut: Lower your credit income by 1. Raise your antimatter income by 1. | 8 cr, 3 en, 2 ins, -1 credits income → 7 cr, 2 ins, -1 credits income |
| Interconnector Treaty | Four effects | Gift and income only: Pay 2 energy. Another player gains 3 energy. Raise your energy income by 2. | 6 cr, 2 en, 2 ins → 5 cr, 2 en, 2 ins |
| Aegis Grid Lockdown | Three effects | Harden and energy cut: Shelter one country this round. Place 2 presence free. | 5 cr, 2 en, 2 ins → 5 cr, 2 ins |
| Shuttle Diplomacy | Order read as moving the new one | Move first: Move 1 ambassador on the board up to 2 hexes. Gain 1 ambassador. | none |
| Summit Caucus | Order read as moving the new one | Move first: Move up to 3 ambassadors in reach onto one piece. Gain 1 ambassador. | none |
| Liaison Corps | Two ideas, too long | Action: 2 ambassadors: Each round, 1 Follower here. Action: pay 2 Followers, get 2 ambassadors. | none |
| Open Assembly | Two ideas | Action: sway only: Each round, 1 Follower here. Action: pay 2 Followers, sway 1 toward Unity. | none |
| Commons Grid | Fee on a rate; too long | Free Action, dearer card: Each round, 1 Follower here. Action: get 1 energy per 2 Followers you have. | 4 cr, 1 ins → 5 cr, 1 ins |
| Manifesto Leak | wordy text | Phrase, in `face.rs`: Add 3 Followers to one of your Follower buildings. | none |
| Development Loan | Matt: rider earned nothing | Bonus cut; still gives up to 6: Give another player up to 6 credits. Gain 1 ambassador. | none |
| Joint Security Compact | wordy text | Phrase, in `face.rs`: Another player gains 2 energy and 1 insight. Harden 2 buildings in reach. | none |
| Amnesty Decree | Rider | Move cut: Remove 2 adversary ambassadors on or next to one corrupted city in reach. | 4 cr, 1 ins → 3 cr, 1 ins |
| Peace Dividend | Footnote condition | Both halves, smaller, always: Sway 1 toward Unity. Gain 2 credits. | none |
| Embassy Compound | Action dearer than its move | Action 1 credit: Action: pay 1 credit, move 1 ambassador on the board up to 2 hexes. Gain 2 ambassadors. | none |
| Consulate Tower | footnote text | Phrase, in `face.rs`: Each round, 1 credit per 2 Diplomacy tags you have. Gain 1 ambassador. | none |
| Peacekeeping Detail | footnote text | Phrase, in `face.rs`: Harden 2 buildings in reach. Move 1 ambassador on the board up to 2 hexes. | none |
| Ceasefire Monitors | "worth" text | Phrase, in `face.rs`: Remove 1 adversary ambassador in reach. Gain 1 ambassador. | none |
| Back-Channel Talks | wordy text | Phrase, in `face.rs`: Free 1 corrupted station or factory in reach. Sway 1 toward Unity. | none |
| Liberation Accords | wordy text | Phrase, in `face.rs`: Remove every adversary ambassador on or next to one corrupted city in reach. Gain 2 ambassadors. | none |
| Exodus Corridors | Order read as moving the new one | Move first: Each of your ambassadors on the board may move 1 hex onto an adjacent piece. Gain 2 ambassadors. | none |
| Great Liberation | Rider: the card already sways | Sway cut: Pay 1 antimatter. Free 2 corrupted cities, stations or factories in reach, in one country. | none |
| Planetary Assembly | Too long, four costs | Action: sway only; income cut gone: Each round, 2 Followers here. Action: pay 2 Followers, sway 2 toward Unity. Pay 1 chip. | 12 cr, 1 ch, 2 ins, -1 credits income → 14 cr, 1 ch, 2 ins |
| Orbital Embassy | Dear for what it does | Chips cut: Each round, 4 credits and 1 insight. Pay 1 antimatter. Gain 2 ambassadors. | 14 cr, 2 ch, 1 am, 2 ins → 10 cr, 1 am, 2 ins |
| Sentry Drone Wing | footnote text | Phrase, in `face.rs`: Each round, 1 Agent here. Action: pay 2 Agents, harden 2 buildings in reach. | none |
| Kill Team Roster | Too long | Action removes a troop; energy cost gone: Each round, 1 Agent here. Action: pay 2 Agents, remove 1 troop in reach. | 4 cr, 1 en, 1 ins → 4 cr, 1 ins |
| Safehouse Network | wordy text | Phrase, in `face.rs`: Each round, 1 Agent here. Action: pay 2 Agents, shelter one country this round. | none |
| Recruit Operatives | wordy text | Phrase, in `face.rs`: Add 3 Agents to one of your Agent buildings. | none |
| Mutual Defense Pact | Three effects; near-duplicate | Redesign: shelter and a gift: Shelter one country this round. Another player gains 2 credits. | 3 cr, 1 ins → 2 cr, 1 ins |
| Shared Threat Intel | Three effects | Credit cut: One threat needs 1 less of each resource. Another player gains 1 insight. | none |
| Threat Triage | Too dear (055) | Cheaper: One threat needs 2 less of each resource. | 4 cr, 1 ins → 2 cr, 0 ins |
| Bounty Board | Footnote condition; reward for something else | Redesign: removes a troop itself: Remove 1 troop in reach. Gain 1 credit. | 0 cr, 0 ins → 3 cr, 1 ins |
| Peacekeeper Rotation | "worth" text | Phrase, in `face.rs`: Remove 1 adversary ambassador in reach. Move 1 ambassador on the board up to 2 hexes. | none |
| Diplomatic Expulsion | Too close to Peacekeeper Rotation | 1 credit dearer: Remove 1 adversary ambassador in reach. Sway 1 toward Unity. | 2 cr, 0 ins → 3 cr, 0 ins |
| Intrusion Countermeasures | wordy text | Phrase, in `face.rs`: Remove 3 compute from an adversary project. One threat needs 1 less of each resource. | none |
| Early Warning Drill (was Threat Forecast Engine) | Same card as Predictive Modeling | Redesign and rename: a delay and a shield: Delay 1 threat one round. Harden 1 of your presence. | 2 cr, 0 ins → 2 cr, 1 ins |
| Agency Field Station | Fee on a small rate | Free Action, cheaper: Each round, 1 Agent here. Action: get 1 credit per 2 Agents you have. | 4 cr, 1 ins → 3 cr, 1 ins |
| Hardened Bunker Complex | wordy text | Phrase, in `face.rs`: Each round, 1 energy. Effect: you gain the Harden action. | none |
| Interceptor Battery | Too long | Troop only, 1 energy cheaper: Action: pay 2 energy, remove 1 troop in reach. | 5 cr, 1 ins → 3 cr, 1 ins |
| Security Contractor HQ | Too dear (055) | Cheaper: Each round, 1 credit per 2 Security tags you have. | 3 cr, 1 ins → 2 cr, 1 ins |
| Breach and Clear | Footnote condition | Credits always, no energy: Remove 2 adversary ambassadors or 1 troop from a hex in reach. Gain 2 credits. | 3 cr, 1 en, 1 ins → 4 cr, 1 ins |
| Hardpoint Lockdown | Near-duplicate of Storm Hardening | Shelter and a removal: Shelter one country this round. Remove 1 adversary ambassador in reach. | none |
| Agency Directorate | Matt: "worth" | Names the piece: 1 troop per hex: Each round, 2 Agents here. Action: pay 4 Agents, remove 1 troop per hex of a country. | 12 cr, 2 ch, 2 ins → 10 cr, 2 ins |
| Aegis Dome Array | Two Effects, an Action, three costs, 227 characters | Redesign: one Effect: Effect: each round, stop the first destruction in a country you're in. | 10 cr, 2 en, 1 ch, 2 ins → 6 cr, 2 ins |
| Threat Neutralization Mandate | Five to pay; title too long | Energy cut, cheaper: Counter one threat. | 7 cr, 2 en, 2 ins → 6 cr, 1 ins |
| Martial Cordon | Rider | Move cut: Remove every adversary ambassador in one country, and sway 1 toward Unity per 3 removed. | none |
| Deep State Ledger | footnote text | Phrase, in `face.rs`: Each round, 1 Agent here. Action: pay 1 energy, get 1 credit per Agent you have. | none |
| Kinetic Interdiction | "worth" | Names the pieces: Pay 1 antimatter. Remove 3 adversary ambassadors or 1 troop per hex of a country. | none |
| Total Containment | Energy cost on top | Energy cut: Remove every adversary ambassador and troop in one hex. Shelter one country this round. | 9 cr, 2 en, 2 ins → 9 cr, 2 ins |

Unchanged: Venture Capital, Shelter Order, Foundry Core, Sentinel Post, Payroll Automation, Bulk Silicon Procurement, Lights-Out Assembly, Bridge Loan, Robotic Foundry, Full Automation Mandate, Asteroid Salvage Rights, Relay Mast Rollout, Pylon Drop, Peaker Turbines, Shared Grid Accord, Brownout Relief, Fusion Startup Stake, Tidal Barrage, Capacitor Discharge, Consular Pouch, Mutual Aid Pact, Treaty Grid Exchange, Coalition Charter, Planetary Referendum.

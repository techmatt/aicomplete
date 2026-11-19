# 285 bug hunt: fixed (F379 on)

r135 → r136. Four fixes change play (F379-F381, F416: one bump, bumped from the constant; the golden game reaches none, so
its fingerprint held and the content hash moved); the rest are words, notes, labels, faces, agents' fixed policies,
tooling and the checker. Notes and labels moved, so a session saved in the viewer before this hunt may replay with a
warning (the record's revision).

## Play (r136)

- **F379** A cordon (Hardpoint Lockdown) kept a line from starting in its country but not from landing there: a Spill
  from a neighboring country's site, a botnet's seed across the border, Night Raid and War Plants' fallback agents
  landed in the cordoned country, and a Grow from an open country's site could build there. Rulebook §16 and
  card-grammar.md said no agent lands and no Grow builds there. Every agent placement now checks the cordon of the hex
  it lands on (`pieces::place_as`), Spill's targets, the seed's targets (and its forecast `next_seeds`), Grow's build
  hexes and Night Raid's countries skip a cordoned country, and War Plants doesn't offer a cordoned factory. Test
  `a_cordon_keeps_spills_seeds_and_grows_out_across_a_border`.
- **F380** Cross-border ties (a Spill's target, a seed's, a Grow's build hex) broke by hex number alone, so hex 2 of
  country 1 beat hex 5 of country 0; §2 breaks every tie by the lower country first (`plan::tie`).
- **F381** Field Station (1 insight) was offered and charged with no open station site in reach: it bought only the
  node Field Survey gives free (the any-part rule passed it). It isn't offered now (`ai::power_blocked`), as Lab II
  without an open megaproject.

- **F416** Offensive (a Discord card) deals its tier-2 threats with the first token on a node hex, whatever stands
  there; Overrun's "corrupt the city" then took that open hex as a city (an adversary cube on open land, the node
  destroyed). It corrupts only a city now (`new_ops::city_token`).

## Log lines and the player view

- **F382** Grow's note said ", an agent on it" before the placement, which a firewall, the cordon, an empty supply or
  an ambassador can stop; the note now follows the placement and says which, and `adversary.grew_infested` counts
  landed agents only.
- **F383** Night Raid's and a station flip's notes read the agents left after the placement: an arrival that brought a
  node hex to 2 (the node rule sends every agent home) read as "cancels an ambassador there". They read the arrivals and
  cancels now.
- **F384** The adversary's turn named none of its Infiltrate, Surge, Spill or seed lines: only corruption, Grow and
  projects wrote notes, so *Since your last move* showed a Spill as nothing (its -1/+1 netted out) and counted map
  changes as "resources, cards, tracks". Each line now writes a note naming where its agents landed (or what stopped
  them), before the arrival's own note (`pieces::place_line`, the label built only when events are read); the turn
  panel counts piece changes apart.
- **F385** A two-node Growth option (Sprawl, Outpost, Peer Review) with one node token left said "unaffordable: ...
  energy" whatever the seat's energy; it says "no node left for every node" now.
- **F386** The phase bar called the Drone step "E growth" (city growth went at r130); the phase texts for bases, the
  march (r83: troops ignore ambassadors and strike on or next to their hex), production and the compute beat (the
  research check) were stale.
- **F387** Liberate's hover listed its 3-5 ambassador price and then "costs nothing".
- **F388** The no-Corrupt token's hover and pending-marker text named the Corrupt step r130 removed, and left out that
  it also keeps a node from 2 agents; Rule of Law's faces too.
- **F389** The corrupted-cities hover said each one "builds agents" (pre-r130); it diverts its energy as Discord.
- **F390** Blackout's hover and Rolling Blackouts' face said "no energy"; the blackout also stops a corrupted city's
  Discord (Naval Blockade printed both).
- **F391** The new-player summary's ways to lose a node left out an adversary troop's strike, and named a Corrupt step.
- **F392** The Cordon hover listed lines, not "no agent lands" (F379's rule).
- **F393** The player view's seat strip showed insight as set ("→N") under Patience and left out a gift, against the seat
  panel; its "+N" income is upkeep's (factories' chips and stations' cubes come in production): its hover says so.
- **F394** Viewer: R, Space, the arrows, V and Ctrl+Z acted on the game behind the open record dialog once a click on its
  text moved the focus to the page; Undo left a record's "loaded with warnings" or "stopped replaying" bar up after the
  game became a played one.

- **F408** The player-view session (two games through the page's controls) found the turn panel still silent where a
  player loses track: a troop's march (a move netted to "bookkeeping only"), threats dealt and the card turned up
  ("cards changed"), a threat's or Discord card's agents ("+2 agent", no hex), an uncountered threat's anchor agent,
  and lines that did nothing (an Infiltrate with no site, a no-Infiltrate token, a Grow with no room, a Spill below its
  level). Each writes a note now, and the panel's map line nets per hex (a move shows as +1 and -1).
- **F409** The Alignment box's bold "purge: no corrupted city, botnet, ..." read as a fact about the board while 3
  cities were corrupted: "win by purge: ...".
- **F414** (D123) While a question is open the hand showed the play check's status: Scrap II's prompt building a card
  free sat beside that card's "17 unaffordable". The hand, buildings and powers read "waiting on the question" then.
- **F410** A track answer read "+1 credits a round" (the record key's plural); the prompt beside it "+1 credit".

- **F418** A second player-view session (two games to the end, every note checked against the map: agents' notes all
  matched) and the third diff review: a troop placed by a threat, a project or the research track's Muster now says
  where; a firewall or prohibition token coming off says so; a botnet with nowhere to seed and a creation line with no
  site say so; the research track's "(16 of 15)" says it fires; one ambassador sent home is singular; the map line
  reads "+5 agents", short names, the description in the hover; Surge's and Infiltrate's no-site notes read alike; a
  building's or power's "used" or "face down" still shows during a question (F414 had hidden it); Botnet Recruiters'
  face says its compute comes without its seed; Grow's op doc names its fallback type; the arrival-check words (every
  placement lands its agents for a hex, then checks once; a Spill moves one at a time), the glossary's land, the
  base hex's wording, D130's draft checks. A fourth review of these (8 real, all fixed): a project's troops noted
  twice (a quiet placement for callers with their own note), an empty creation under a cordon said "no site", Surge's
  firewalled note read "none ... but every one", a building's "no target" survived the question, the map line kept
  descriptions and dropped "(adversary)" ("factorys"), "it fires" said before the research check, two seats'
  ambassadors sent to "their supply", a dead "research" arm.

- **F423** Round 11 (the viewer's board, panels and timeline, verified): the generic Piles list showed a discard pile's
  three oldest cards (its `visible` is top first); acting at an earlier step of a game that stopped replaying asked
  about the replayed actions only (and at the last step asked nothing) while it also dropped the unreplayed ones; a
  saved session stopped at its first action ignored Undo though its error bar named Undo.

## Faces and the rules' words

- **F395** The printed reference card's level setup gave compute income and the research threshold without "a
  country" (the engine multiplies both by the countries: a table set up from the card ran at half rate at 2).
- **F396** Faces left out "not yet the adversary's" where the engine passes over the adversary's sites: Botnet
  Recruiters' seed, every Surge and Spill line (projects, events, Offensive); the rulebook's glossary too.
- **F397** Stale rules prose (verified against the code): economy.md's chips row (Wafer Fab Line makes 2; Fortify II
  and Biomia's converter cost 2) and its Buy energy (4 credits; a hosted city can't be bought); board.md's and
  ai-powers.md's give / get on superseded tables; principles.md's cube value and project costs; card-pricing.md's card
  throughput (pre-271 draw); the rulebook's power counts (§5, §20), "none while hacked" (no card cuts a botnet), the
  d4 Infiltrate cap (keyed on the line's type, not what is on the hex; the generated difficulty row and harms too),
  production from the tracks (a station's cube sits on its megaproject until it completes; the player aid too), the
  no-Corrupt row; the cordon and tie words (F379, F380). Round 3: ambassadors.md's troops removing ambassadors (gone
  at r83) and two cut cards; ai-boards.md's burn, Dispatch give / get and chip sources.
- **F415** The rulebook read whole against the code (round 4, 3 finders by section, verified, 9 real of 10): a line's
  agents all land, then the site is checked once (it read one check per agent); a botnet's compute comes under a
  cordon or with nowhere to seed; a base is no open land (no Fallout, no station); printed empty station sites take a
  player's station or a Grow; §1's agents "waiting to land" (pools, gone at r130); a card's "free" on a facility takes
  only the cube (defined; the rule is D80's, Matt's call); a factory's chip and a station's cube paid once (§9 read
  as twice); Warmongering's troop condition (§19); an occupied-factory chip no card makes (and the aid's).
- **F417** Round 5 (Unity / Discord and threat faces, the doc generators' own text, a review of round 4's words; 12 real
  of 13): alignment.md's "no strength III clears around corrupted cities"; Neighborhood Watch's faces' "its share" (a
  Rally word); the player aid, economy.md, harms and the compute phase on the per-line arrival check and the seed's
  compute (F415's rule); the aid's keep prices without the Science discount; Spill's and Grow's op docs (targets,
  ties, cordon); round 4's own base-hex wording (a base is land: nodes may go there) and its Spill check; the threat
  drafts' notes on retired counters and Corrupt lines (their value lines flagged stale).
- **F420** Round 8 (architecture, README, simulator, experiments, reads: nothing; card-grammar, player-cards, board):
  board.md told new players to claim the neutral station with 4 ambassadors (cards claim since 262), and said every
  country prints an empty station site (Cintamani and Biomia print none); player-cards.md's drawing converters left
  out Delta Seawall's, and a building's per-tag income read "2 a tag" (Power to the People pays 1 per 2).
- **F421** Round 9 (war, economy, ambassadors: nothing; design and agent pages): principles.md's tier-2 median cost (7,
  not 6); card-pricing.md's research cube at 2 (4), a tier-2 +1 energy income at 6 (8.4 at the 3.5 horizon), Liberate's
  5 ambassadors at 67 (27); speed.md's removed `Sites` type. Left: card-pricing.md's energy-sink totals for Citywide
  Microgrid and Exascale Training Cluster (30, 23) don't reproduce at today's anchors, but the page doesn't say what
  they count (a pricing pass's to restate).
- **F422** Round 10 (web and card-art READMEs: nothing): rulebook-friction.md's links to "the turn: four beats" (the
  heading is three beats) and its "track component stays open" against its own row 11 (done in 274).
- **F398** `strategies.json`'s descriptions (docs/agents.md) named a Shelter power, Fieldwork and draw sources that no
  longer draw; drafts/adversary/README.md stated the pre-275 event standard; project value notes said a Surge 2 takes a
  facility (they fall at 3 since r134).

## Agents and tooling

- **F399** `power_converts` (the `never_convert` fixed policy, live in the gate specs) read Fortify I, Patrol I and Lab
  as resource conversions; it means powers that only gain (none since r131).
- **F400** `npm run cards:check` crashed at its export since 267 (its copy lacked `web/card/adversary.json`).
- **F401** `scripts/bench/am_read.py` refused to run since 276 (its burns class named three cut powers); dropped.
- **F402** `resource_split.py` crashed with a bare ValueError on an unknown `--resources` name; refused with the list.

- **F405** Measures gone quiet since r130: `ladder.py --measures`' "ambassadors placed" summed creation lines and the
  retired `rallied` (about 5% of the agents placed); it reads every arrival less the canceled (an older side keeps the
  old sum). `spread_read.py`'s retakes column printed 0 (no game tallies retakes since r130): it prints `-`.
- **F406** `spread_read.py` read a `PAIR_LEVER` variant (`bp_cap`) as `base`, pooling it with base.
- **F407** `payback.py` priced the per-tag triggers that name a tag (Orbital Embassy, Interdisciplinary Institute) as
  counting every tag (2-3x their fire).

- **F411** (D102) `npm run e2e:corps`, red since 235: each country's printed station starts corrupted, and since 185 a
  corrupted station puts `station_cubes` adversary cubes a round on its open megaproject, so at the lane's
  `station_cubes` 5 both cards completed in round 1 and the round-2 colors read 0,0 (the "overflow paid" check passed
  on round 1's completion). No engine fault: the lane plays `station_cubes` 2 and pins the cubes by color (adversary's
  included), both completions' paid lines and the stored cubes at numbers derived from r136.

(The keep plan's board-tag count, `am_makes`, was fixed and reverted: D120.)

## Checker

- **F403** The sweep's moments for r130-r136 (`COUNTER_EVENTS_R131`: Infiltrate blocked or from a build, guards
  holding, Wall stopping a landing, firewall held, purge held off by botnets, research troops, a skipped spread, node
  capstones, cubes bought, chips made, a card drawn, a cordon keeping an agent out) and two judge cases at 3 and 4
  countries (`c3-d3-strategist`, `c4-d1-strategist`).
- **F412** A fast-lane test, `cards_check_copies_what_the_card_export_reads` (`crates/aic-sim/tests/scripts_parse.rs`):
  every path `cards-export.mjs` joins onto its root is in `cards-check.mjs`'s fixture copy (F400's fault class).
- **F413** `npm run odd` gains `keys-behind-record-dialog` and `undo-clears-record-warning` (F394; both fail on the
  old viewer) and `player-view-turn-panel-names-lines` (an Infiltrate note in *Since your last move*, F384).
- **F419** The card viewer: the adversary reference card said "a country" twice after F395 and headed per-game and
  per-site setup rows "Setup (a country)"; a note on the reference card or an AI board saved its blurb only (their
  words are in sections), so "Changed since your note" showed nothing of what it said (`cards-server.mjs` `saveNote`).
- **F404** The sweep's memory check compared the heap after the last case with the second, so a 4-country case run last
  read as 68 MB of growth (the state a step, D52). It now plays the sweep's first fresh game again at the end and
  compares the two (what the page keeps once its games are gone); the heap after each case still prints.

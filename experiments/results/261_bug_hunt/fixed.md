# 261 bug hunt: fixed (F322 on)

r121 → r122. Four fixes change play (F323, F324, F358, and F345 by Matt's answer: one bump; the golden content hash
moved, and the golden game reaches none of them, so its fingerprint held); the rest are words, labels, agents,
measures and the checker. Option strings moved (the node
track prompt, the threat pay buttons), so a session saved in the viewer before this hunt may stop replaying there.

## Play (r122)

- **F323** Industrial Commons and Open Research Campus could target a free zone or empty station site with an agent on
  it: the site opened, and the printed "Claim it" did nothing (a site with an agent can't be claimed), with no word
  why. A card whose list opens a site and then claims it now skips such a hex (`targets::target_hexes_before`, read by
  `aim`, `feasible` and the hand's "map part: no target" mark); a card that only builds there (Prefab Plant Kit) still
  may. Test `a_card_opening_a_site_to_claim_skips_a_zone_with_an_agent`.
- **F324** Peer Review's rider ("if the new node brings a research station into your reach, gain 1 insight") was read
  after the uncovered track space resolved, so the Researcher's free-station space could build the very station that
  paid it. The rider now reads the stations and factories standing before the placement (`Ctx::node_spots_before`).
  Test `a_rider_ignores_a_station_its_own_track_space_built`.

- **F358** A node route is used for its node (118's intent): Outpost and Field Survey were refused with no token or no
  payable step, but offered with no free hex in range, where the player paid 3 energy or 3 insight and got the
  ambassador or card alone. `ai::power_blocked` now reads "no target" there; and `place_node_within` with no free hex
  returns before asking which track (it asked, then placed nothing, silently), with a log line. Test
  `a_node_route_with_no_free_hex_is_not_offered`. (Card Actions that place and gain still play for their gain:
  rulebook §7's "any part can act".) Rulebook §5 and ai-boards.md say it.

- **F364** Factory Occupation corrupted a firewalled factory (or one under a Corrupt prohibition): 185 (D77) guarded
  every other path that corrupts one site and missed this one, while the rulebook, the player aid and harms.md say a
  firewall stops every threat that corrupts. Guarded (`guard_held.corrupt_factory`); test
  `a_firewall_guards_factory_occupation` (the firewall). Its card now prints the fallback ("with none there, it builds
  one on the empty zone"), which, as Mass Production's build, no guard stops: an empty zone has no site.

## Agents

- **F322** The beam's stall (259's seed 3): it paid a threat's counter a credit at a time across decisions and then
  withdrew it, round after round of plans, past 3,000 actions. `search::undoes` guarded one plan only; the beam now
  keeps the phase's moves (`PhaseHistory`, as greedy does) and masks an undo in its search and its fallback.
  `beam_replays_and_repeats` is back on seed 3 (ends in round 11, 18 s in the test build).
- **F346** The card-target heuristic (`moves::finds`, taken without the search when its best is unique) scored a
  liberate card by agents *next to* the city, which a liberation never removes, and gave the 256 claim rider nothing:
  it now counts the agents on the site, a claimable neutral facility beside it (`claim_site`), and Hostile Buyout's
  `own` on a station or factory (`liberate_own`). Paired on 48 dev seeds (100-147, strategist), against the build
  without it: d0 and d1 the same, d2 -2.1 (one game).

## Words a player reads

- **F325** The track-step hover said a step's energy is paid "unless a card says free"; no card places a node free
  since r81.
- **F326** The cube converter (Turn a cube into insight) was "no limit" or "2 cubes for 1 insight" in five rules
  lines (board.md, ai-boards.md, ai-powers.md and its new-player paragraph); it is 1 for 1, once a round, since r114.
- **F327** economy.md's resource table and the rulebook's left out the 252 node routes (Outpost's energy and
  ambassador, Annex's chip, the Researcher's insight routes).
- **F328** The rulebook, ambassadors.md and the viewer said a *freed* site carries the Liberated marker; only a
  liberated one does (a card that frees a station or factory leaves none). Words fixed (card-grammar says it too).
- **F329** The viewer's Liberated-marker name printed the raw tunable `retake_agents`.
- **F330** The rulebook's Pieces table had no Liberated marker.
- **F331** Eight Unity cards printed "for each city, research station or factory a player owns"; no player owns a city
  since 254 and the engine counts a hosted one: "... or city a player hosts".
- **F332** `host_city`'s tunable text said hosted cities are "not for sale" and need a node on them (a node next to
  one hosts it; Buy energy takes its energy; `host_more` raises the cap); ai-boards.md said "node on it" twice and
  named one node power for the Builder and Protector.
- **F333** ambassadors.md billed Claim and Liberate "+1 per agent" (r75), 25 lines above its own r91 rule: a site with
  an agent can't be claimed or liberated.
- **F334** Claim's rule text, the rulebook and ambassadors.md promised a block by "embargo" that nothing in the game
  makes (the Embargo threat doesn't); now "no agent on it".
- **F335** A card's claim drew a station icon on factory claims (Industrial Commons, Power Purchase Agreement,
  Microgrid Lab Compact): station / factory, as the rider.
- **F336** The four megaproject cards said "An adversary cube made here afterward is lost"; since 259 a corrupted
  station there feeds the research track.
- **F337** "Remove 4 adversary research cubes" (Counterintelligence Sweep, Arms Inspectors) named the track's cubes
  with Backdoor Research's words for megaproject cubes: "cubes from the adversary's research track"; Backdoor
  Research says "adversary cubes onto each open megaproject"; their pricing notes follow.
- **F338** The rulebook's §21 and adversary.md's difficulty tables left out the research threshold (5, 5, 5, 4, 4)
  and the stronger Weapons Program from level 3.
- **F341** A hosted city's energy had no log line, and the dark-city hover still named "its owner": a note per hosted
  city ("X makes 1 energy for Seat 0 (hosted)"), and the hover says "not sold, not hosted".
- **F342** board.md said Liberate takes an adversary station for the liberator (free, then claim since 252).
- **F343** Rulebook §5 gave 6-8 powers a board; 8-10 since the node routes.
- **F344** "No card makes research cubes" (ai-powers.md, board.md, economy.md, rulebook): four cards do; the loop note
  priced a 1-chip station (2 since 252); Cryo Sample Vault is an income, not an Action.
- **F345** Survey Drone Fleet's r119 "+2 energy" ran after its Action's two node placements, so it couldn't pay their
  track steps as 258 priced it. Matt's answer (message 11995): reorder, in r122. The Action gains its energy first;
  its pricing note and `design_check_jobs.json` row say so. Same ops and price, so its give / get holds (258's:
  pay 48, get 112). Orbital Relay Launcher keeps its order (no claim was made for it).
- **F348** Accord's ambassador stood "toward the city's claim" (ai-powers.md, ops.md); no city is claimed.
- **F349** The research hover at d3-d4 read "(Muster 3 instead.)" alone: the rule, then its stronger line.
- **F350** card-grammar grouped Summit Caucus's `consolidate` with `move_ambassadors` ("yours come from your supply");
  it moves only yours on the board. player-cards.md priced the liberate capstones 4-42 with three antimatter ones
  (6-26, four); card-pricing.md gave Peacekeeping Mission 2 troops (3).
- **F351** The node placement's prompts: "track 1 / track 2" where the board prints economy and card-play tracks, and
  a card-play space that only costs read "place a node (uncovers , costs 2 energy)"; both now name the tracks and say
  "uncovers nothing". The hex prompt words the space as the track question does ("+4 insight", not the new total),
  in people's words ("T2 keep", not the record key's "T2 keep -1": round 3 caught this hunt's first try).
- **F352** The player view drew every other seat "done" while one seat answered a prompt; and the threat pay buttons
  read "+1 credits".
- **F353** The research track filled with no word why: each gain now has a log line ("2 cubes onto the adversary's
  research track from a threat (7 of 15)").

- **F359** Rulebook §8 said standard actions run "as often as you can pay, Remove a troop once a round"; Turn a cube
  into insight is once a round too (253) and Buy energy once a round a city. Its table named that action "Research
  cubes"; the engine, viewer and economy.md say "Turn a cube into insight".
- **F360** The rulebook's Pieces table had no research track (259).
- **F361** The player view's draft priced a keep at the config's base (a tier 2 at 3 insight) after the keep upgrade
  or Science made it less: a draft tile's status carries the seat's keep price, the packet note reads it, and the
  tier buttons say a tier 2 keeps for "up to" the base.

- **F365** Round 4's content sweep: the Blackout and Sabotage hovers credited "an uncountered threat" (no threat
  makes either; their cards say how long); threats.md's new-player list of threat harms left out the research track
  (it now points at harms.md); the rules' Terms said a player cube on a city means "aligned"; rulebook §20 filed
  Interdict under conversions and timed Open Science two ways; "two more in the board's own currency at rising
  cost" is false for the Builder (energy, then chips); the Researcher's board said "turn budget" (not a house term);
  ambassadors.md missed Microgrid Lab Compact's claim when built.

- **F366** A node route its next track space's energy blocked read "Expand ... unaffordable" with no price on it
  (Expand prints none): it now says "unaffordable: the next track space's energy, after the power's own" (contact
  sheets, 3 captures), and the debug view keeps its warn color.

- **F367** The Weapons Program put 6 troops on the board at 3 countries, d2, where the rule reads Muster 2 + the
  level's 1 = 9: Walls and Muster prohibitions stopped the rest (3.3 walled landings a game at c3 d2, 12 greedy
  games), with no word why. Each hex's stopped landings now get one log line ("The Protector's Wall keeps 3 troops
  off X"). (Was D111.)

- **F369** Player view (round 5's code read of the viewer's state): a record opened, dropped or loaded there, or a
  session restored, showed its setup step with no timeline to move on, and any control offered to drop the whole
  record (it now plays from its last step, as switching views did); Space after a mouse click on a header button
  pressed it again (New game, Undo, the view toggle: 235 fixed this in the debug view only); pinning another seat
  while one answered a question emptied the bar that says "answer here". A restored session's error bar read "saved
  session: saved session from ...". `npm run odd` gains `player-view-record-space-pin`: it fails on the old viewer
  (all three) and passes here.

## Measures and scripts

- **F339** `corp_read.py`'s "adv station compute" reads 0 since 259: it prints the stations' research cubes beside it.
- **F354** `liberation_read.py` and `energy_read.py` counted Hostile Buyout's station and factory frees as city
  liberations by card (and the action's could go negative): `board.liberated_card.<kind>` is tallied, and both read
  the city count (an older run falls back to the old key).
- **F355** `ladder.py --measures`' "own city" read 0% since 254: `board.node_at.<seat>.own_city` counts a city the
  seat hosts.
- **F368** The experiment report's "cities corrupted / aligned at end" read 0 in every game since 254 (no city takes
  a player cube): `cities_aligned_end` also counts a hosted city (as the Unity cards do), the column reads "aligned or
  hosted"; the field keeps its name for games.csv's readers (experiments.md says so). (Was D110.)
- **F356** experiments.md named `board.station_compute` as a compute source (0 since 259).

- **F371** Round 6, analysts' tooling since 235 (verified; the rest is D117): `read_lib` refused an agent-lever
  screen ("strategist", "strategist:w.sink=0.5": one name after the cut at ':') as "pooled reads overlap": it keeps
  the whole spec where short names collide; `am_read` merged `PAIR-LEVER` arms (253, 254) into one "base" row and
  divided burns by games; `resource_use --limits` dropped the research-cube shortfall row (its '.r' filter); `paired
  --pool` overwrote a seed two folders share, silently (now refused); the "Place a node" job (255 cut the action)
  left in `design_check_jobs.json`; per_decision.py and seeds.py promising a column and a section that don't exist;
  agents.md on what the value table and `w.sink` reach; experiments.md's spec table without `drop`.

- **F372** Round 7, words against code in the pages not yet read (30 real of 30): war.md (Scorched Earth "destroys
  nodes", a card by its id's old name, troop sources without the Weapons Program, Factory Occupation's "cleanup"),
  the rulebook's War Takes Ground pointing at §19; the card pages' counts and examples from before 206-259 (feeders,
  sinks, antimatter cards, primary tags, the tier-2 keep of 2, a removed card as an anchor, Dyson Swarm Tranche's
  +3); architecture.md, principles.md, simulator.md and agents.md (players owning cities, project costs 7-22, a
  removed `board::liberate`, a `contexts` match that is `slots`, `node_spread`, the chip converter's label, "a node
  costs 1", Expand on every board).
- **F373** Round 8, the last unread pages (5 real of 5): economy.md's chip prices (the converters are 2), chip sources
  (Buy In; three one-time gains; Wafer Fab Line), Reclaim's 6 credits, "more energy for a richer corp" (1 for every
  corp); web/README's "twelve scenarios" in odd.mjs (13 now: it names how to find them instead).
- **F374** Round 9, the r113-r122 verbs against the old guards (the players' verbs: nothing): the rulebook,
  ambassadors.md and the viewer's hover promised the Liberated marker's extra agent against any retake; the engine
  (and adversary.md) apply it to Corrupt lines and natural corruption only, so Station Coup, Asset Seizure, a station
  flip or Overrun take a liberated site with no agent. The words now say so (the rule's scope is D118).
- **F376** Round 10, the rulebook's sections not yet read whole (§1-4 and §12-18: nothing): §9 said only a
  corrupted city builds its facility next to an agent first; a neutral city does too (the code, war.md and
  `grow_cities`' doc agree), which since 254 is every city. The rulebook and war.md now say "neutral or corrupted
  alike".
- **F377** Round 11, every face grouped by op (10 real; 3 fixed, the rest D119): Peacekeeping Mission's "remove up
  to 3 Troops" (every other removal prints "remove N", already up to N); Summit Caucus's and Liaison Corps's moves
  print "of your ambassadors" (the ops move only yours; a bare "Ambassadors" is any player's); the Militarist's four
  Botnets-spread events counted "a Botnet" next door where the engine counts the country's own.
- **F378** (leftover budget, was D116's first half) An answer that isn't on offer, as an r121 record's reworded
  node-track option will be after this hunt, said only "not an option of ...": it now lists what is offered (both the
  action check and the replay path); test `a_wrong_answer_names_the_options_on_offer`.
- **F375** Rounds 6-9's diff, reviewed (3 real, small): `paired.py` without `--pool` on two reads both named
  `games` stopped with "pooled reads overlap" (it now says the two share a name); architecture.md's list of
  `adversary::remove`'s callers read as complete; card-guidelines named two antimatter plants of six.

## The checker

- **F340** The sweep watched four counters no engine writes (`project_acts`, `node_raids`, `cards.agents_in_reach`,
  `troops_in_reach`) and none of r107-r121's: `COUNTER_EVENTS_R107` adds each board power's first use, a claim of a
  Liberated site, a card's or rider's claim, a hosted city's energy, a site opened, a chip converter, the research
  track (made, fired, removed), a retake, compute blocked, a march cut short.
- **F347** `npm run e2e:researcher` had not run since 252: a second `const insight` in its Peer Review check was a
  SyntaxError at load, so the lane failed before its first check. Renamed; the lane is green (20 checks).
- **F357** A fast-lane test, `node_scripts_parse` (`crates/aic-sim/tests/scripts_parse.rs`), runs `node --check` on
  every `.mjs` the repo runs (web/scripts, scripts, scripts/farm, tools/card-art; 0.4 s, skipped without `node`). It
  would have caught F347, and caught this hunt's own sweep edit (a backtick inside the page library's
  `String.raw` template). The hunt's diff review added web/card's modules (`originals.mjs`, `card.js`, `icons.js`).
- **F370** Rounds 4-5's diff, reviewed independently (11 real of 11): F369's record fix made a player-view sweep open
  each watched record at its end (the sweep now seeks to step 0 first); its Space guard also stopped Space on a
  summary or the page (now on a button only); F364's guard on the empty-zone build disagreed with Mass Production
  (dropped: no site there); one stopped-Muster line per troop (now per hex, counted); the odd scenario could pass
  without pinning; the route status lost its warn color; docs for `_aligned_end`; the ranges here.
- **F363** `counter_names.py` also checks each top-level alternative of the sweep's `COUNTER_EVENTS*` moments against
  the keys games wrote: on main's sweep and 96 loop-smoke games it names exactly the three dead moments F340 found by
  hand; on this hunt's, none.
- **F362** The hunt's own diff, reviewed independently (13 real of 14): the r122 history row split across lines, the
  draft tile's keep price ignoring Open Market's free keep and repeated on the player view's tile, the no-hex note
  saying "next to its nodes" for a range-2 placement, IfNodeNext's op doc, the rulebook not saying a node route needs
  its node, fixed.md's own count; all fixed. rules-history.md drops its oldest row (107) to keep 1 KB free.

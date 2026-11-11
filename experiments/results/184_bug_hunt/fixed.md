# 184 bug hunt: fixed (F225 on)

r77 → r78. Each line: the fault as a user met it, then the change. Most are r74-r77's renames and rules that a
surface (a face, a label, a doc, the viewer) missed.

## Rules (r78)

- **F225** The Nuclear Option (liberate, then sway Discord 1) was offered with no corrupted city in reach: 19 credits,
  antimatter and insight for a Discord sway, since any sway counted as a part that acts. A Discord sway is now a price
  (`effects/feasible.rs`); player-cards.md and the rulebook say so. Test `a_discord_sway_is_a_price_not_a_part_that_acts`.
  The golden game never reached it: its fingerprint held, only the revision moved.

## Faces and content (no play change; the content hash moved)

- **F226** Security Contractor HQ printed "−1 Troop : 10 credits" (per troop) though r77 fires it once an action: a
  bare Troop icon now, as the agent trigger (`face.rs`).
- **F227** The convert cards (Mass Defection, Embassy of Exiles, Double Agents, Open Source Uprising) ended their icon
  row in a Node: 158's "onto a node", which r77 #32 replaced with "where it stood" (`face.rs`).
- **F228** Rule of Law said "nothing is pulled into it" (r74 took the pull away) and "no site ... corrupts", where the
  code skips only Corrupt lines and natural corruption (a project flip or a threat still corrupts). Its three
  strengths now say what runs; the rulebook's firewall sentence too (question 1, default (a)).
- **F229** Station Coup's face left out that a player's station that holds takes 1 Agent.
- **F230** Mass Production and War Economy said "An adversary factory is built here": since r77 "here" is a factory
  already built, so the new one goes on the nearest free industrial zone, as the code does; the faces say so.
- **F231** Solo games have no printed project cost: the faces and tables gave 2, 3 and 4 players. A 1-player column in
  docs/rules/adversary.md, adversary-cards.md and the card viewer.

## Words players and viewer users read (no play change)

- **F232** r77's renames missed: the viewer's Drone token ("Drone (corp troop)") and country strip ("units", "corp
  units"); the Unity tips hover ("tier"), `tier_at`'s doc and error, `unit_cap`'s doc, the bases phase label, the
  content check's step order ("deploy"), cards.md's Unity/Discord table ("tier I-III"), ops.md (Movement add-on,
  "each board", "corp troops", the cordoned pool "lands nowhere", which r77 made wait).
- **F233** Dispatch's hex prompt read "move on which hex?"; Buy energy, Take down a botnet, Build a research station
  and Remove agents asked "buy / botnet / station / remove on which hex?" (`standard.rs`).
- **F234** The viewer's step line and log named standard actions by key: "standard: move" for Dispatch, "standard:
  harden buy" (`format.ts`, the printed names).
- **F235** Dispatch's button and economy.md said "onto a piece": the code also takes a botnet's hex.
- **F236** Rulebook: a blackout lasted "this round" (Naval Blockade and Rolling Blackouts: to the end of next round);
  the waits token came "after the action phase began" (the code: after it ended); liberate cards were "some tier-2
  cards" (Popular Uprising is tier 1).
- **F237** Rules files: player-cards.md's firewall "for a round" (r77: to the end of the next adversary turn);
  card-grammar.md's liberate icon "always ends → cube" (none prints); ai-powers.md and economy.md's insight counters
  "8 / 10 / 12 by tier" (6 and 7); alignment.md's False Flag threats "wait" (they come before the counter phase);
  war.md's troop sources left out event-card Musters.
- **F238** The cordon hover said "no arrivals land here": troops still land, only Rally and creation lines stop.
- **F239** The land-corruption hover said only "no node may be placed": r77 forbids every building.
- **F240** Pricing notes for Kettle Operation, Lure Network and Honeypot Relay described r70's gather.
- **F241** The Data Heist prompt asked "lose 3 insight" for "3 insight less next round"; a gather note read "1 pieces".
- **F242** The insight hover blamed "agents on your nodes" (r74 removed that drain).

## The viewer (no play change)

- **F243** A waiting threat never showed its "waits" badge or dimmed style since r77 (the text became "Waits token:").
- **F244** Every threat in the countries strip opened with r77's 115-character Payers/Target frame, which filled the
  3-line clamp: the effect never showed. The strip drops the frame (the hover keeps it).
- **F245** The projects row: once the ongoing chips appeared, active projects were cut mid-name with no ellipsis and
  their compute count vanished. The chips wrap; a project keeps 9em.
- **F246** The megaproject name wrapped round its cubes and was cut at the card's foot at 3 countries: one line, an
  ellipsis.
- **F247** "Unity 5/6" was unreadable once the fill reached the label: the label sits on a light pill.
- **F248** "Next round (face up)" headed an empty "not revealed yet" slot; the Deck box and the lose-if chip gave two
  "left" counts (the chip counts the face-up card): the box says "left (+1)", its hover why ("(+1 face up)" clipped:
sweep 2).
- **F249** The insight badge "→6" sat beside "insight lost next round 3": it shows what next round sets (income, plus
  gift, less debt).
- **F250** "Unity tips 0 (next at 3)" beside "Discord 4/6" read as the track's tip point: "(next step at 3 tips)".
- **F251** A seat answer read "seat 0: seat 0", a card answer "seat 0: Liberation Accords" (a discard, read as a play):
  "choose Seat 0 (Builder)", "choose Liberation Accords".
- **F252** The draft phases' prompt said only "tier: Seat 0 to act": it says what the panel's buttons are for.

## The card viewer (no play change)

- **F253** Threat counters read the raw key "research_cubes" and lowercase tags; a threat's locations (its "here") were
  never shown; AI boards never showed Dispatches (the allowance, the "Dispatch +1" spaces); Unity and Discord cards
  said "Three tiers", "tier I-III"; the Win Δ column was always blank and Kept % blank with a screen kept without
  games.csv (Kept % now counts the cells' games; Win Δ shows only with data).

## Read scripts (no play change)

- **F254** energy_read's and liberation_read's "Liberate action" column counted card liberations too since 178 (a card
  liberates through the action's path): it subtracts them.
- **F255** screen.py's default preset and board_read printed "purge / Unity" and "purge / deck" wins: the second half
  has been 0 since r67, read as a measurement. Purge wins only.
- **F256** play_read and use_read with `--agents strategist` (an alias) printed an empty read: they now stop and list
  the agents the run names.

## The sim players (no rules change; play moves)

- **F257** Three misreads of r75-r77 ([agent_fixes.diff](agent_fixes.diff), in the tree): Great Liberation's cities
  priced without the cube 178 now puts on (`value.rs`, `effect_ledger.py`); the eval read corruption pressure on a
  firewalled or prohibited owned station or factory, which r77 #30 keeps from corrupting (`eval.rs`, as cities
  already did); the push heuristic's node-hex risk counted agents on neighboring node hexes, which stay on their own
  target (`moves.rs`, as `tips_last_node` reads). Smoke, `use.json` (engine and strategist, three pairs, d2), seeds
  43013-43028, off then on: 13 of 96 games play differently, no win changes (every cell's win % equal).

## The hunt's own diff (an independent review found 26 faults; all fixed)

- **F258** Twins the fixes missed: the strike and push notes' "1 pieces"; the cordon's op docs (ops.md) and the
  Dispatches hover, the new-player summary and the firewall glossary (README, player aid) still on the old words;
  MoveOne called a "Dispatch add-on"; tests' comments on Deploy; Station Coup without its player Ambassador;
  card-grammar's cube line; the waits wording when a threat deals another; insight counters "many ... by tag"; the
  Patience insight badge (it skipped debt and gift); web/README's e2e lanes; the card viewer's "nodes near it"; the
  read scripts' "fallback" wording, a missing space, `--agents` splitting a resolved label at its commas and dropping a
  second unknown name; `docs --check`'s message for the new words. The rulebook said "Rules revision 77": `docs
  --check` now checks its revision line (test `the_rulebook_names_the_rules_revision`).
- **F259** Two e2e lanes were red on main since r75 and the 8-cube megaproject: `e2e:corps` checked "no node" and a
  converter used twice a round, `e2e:researcher` at 3 cubes a round filled Onyx's card a round late. Both green; the
  main e2e's eviction check followed r75 (Remove agents places nothing).

## Round 4 and sweep 2 (code read 4; the off-variant, shared-row and 1- and 4-country sheets)

- **F262** (rules, r78) A threat line for "the base here" or "the factory here" fell back to the threat's other token:
  at 1 country (no base) Garrison State and Manhunt landed their troops on the city; Iron Curtain's line named no
  type. A missing type now lands nothing (`new_ops.rs` `troops`), Iron Curtain names its factory, threats.md says so.
  Test `a_troop_line_for_a_missing_token_type_lands_nothing`. The golden fingerprint held.
- **F263** Saved sessions: a watched record's restored warning dropped the "saved session from rules revision N" note;
  a session that stopped replaying under a new revision lost its unreplayed actions at the next New game (now kept,
  `copy(aic.keptSession())`, as one that fails to load); the Record dialog dropped its "stamped as this engine's
  revision" line for such a game.
- **F264** Words: the new-player summary and ai-boards.md said "There is no growth step" (production has one); the
  rulebook glossary's Dispatch "onto a piece"; threat counters printed "ambassadors" lowercase beside "Ambassadors";
  the protection prompt read "the seat 3's node"; "threat tier 2/3" (there is no tier 3); the last-node hover
  promised "early on, nothing destroys a last node" (175 took that away); the report's "tier-2 cards played (buildings
  and programs)" counts events too.
- **F265** The end-game probe counted station and factory cleanses as liberations and left the convert cards and
  Amnesty Decree out of agent removal.
- **F266** The lose-if chips stayed red under a WON banner.

## Round 5 (long runs, state across games and sessions)

- **F268** On/off country flags printed a constant "1" ("Sheltered 1", "Cordon 1"): the label alone (from D87).
- **F269** The viewer's tooltip and hex outlines stayed after the mouse left the window, until the next step.
- **F270** A resumed `aic-sim experiment` divided its "move heuristics a game" line by every game, the resumed ones
  (replayed with no agent) included: about a tenth of the rate after a 90% resume.
- **F271** The card viewer's "Mark all seen" failed silently on a broken ledger or a held file: it says so now.

## The checker

- **F261** The sweep's moments add r74-r77's counters (contact cancels, converging, nodes corrupted at 2, Remove
  agents, Dispatch, troops taking an ambassador, the last node lost) and its `stale-word` list r77's old words (corp
  troops, Deploy, each board, tier I-III, Movement +N). `docs --check` refuses "corp troop" and "deploy" in player
  text, and now reads the rulebook.
- **F267** `Content::check` refuses a threat's troop line on a token type it doesn't print, or on a two-token threat
  with no type named (F262's Iron Curtain); `docs --check` refuses a stale rulebook revision line (F258). Tests in
  `overlay.rs` and `docs/tests.rs`.

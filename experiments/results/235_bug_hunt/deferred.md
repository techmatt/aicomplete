# 235 bug hunt: deferred (D92 on) and won't fix (W31 on)

Each changes play (a `RULES_REVISION` bump and goldens), or is Matt's to rule; most important first.

- **D92** A tied threat asks "which of the nearest players takes its harm?" even when the harm reads no seat (Orbital
  Strike, Call-Up, Casus Belli: a troop or Fallout lands on the threat's hex, a front fights). A human hunts for a
  difference that isn't there; the playtest met it in rounds 1, 2 and 7 (`threats/mod.rs` `hit_seat`). Options: (a)
  ask only when an op of the harm reads the hit seat (a per-op flag beside the twist loop's reader list); (b) word the
  prompt "(it makes no difference here)". Pick (a): fewer clicks, and the prompt keeps its meaning. A bump: the
  choice leaves the action list.
- **D93** Targets that offer dead picks (234's one target; `feasible.rs` checks only that some hex works): Firewall
  Array's land target takes a hex that already holds a firewall; Exodus Corridors' and Rally the Allies' land target
  takes a hex with no ambassador next to it, where the Gather moves none; Summit Caucus's piece target takes a piece
  with none of your ambassadors next to it, where the consolidate moves none. The card is paid (or the Action spent)
  and only its other lines pay. The agents' target rule picks a live hex, so reads never show it; a player picking
  from the highlighted hexes will. Options: (a) a target spec per op condition, sharing its test with the op
  (`gather_hexes`, `consolidate`'s destination, `firewall_hexes`), faces updated; (b) leave them. Pick (a), one bump
  with D92 and D94.
- **D94** After the last node falls in the march, the players still answer that round's remaining troop moves before
  the loss shows (playtest round 10). Options: (a) end the march once a seat has no node; (b) leave it. Pick (a), a bump.
- **D95** "1 Unity swayed: how much holds Discord off instead?" is asked of seat 0 when seat 1's card swayed, and is
  hard to parse (Diplomatic Expulsion in the playtest). Options: (a) ask the card's player, "Put 1 Unity on the
  track, or take 1 Discord off?"; (b) reword only. Pick (a) with (b); who chooses is play: a bump. Joins D85.
- **D96** (Matt's call: bookkeeping a table can't track) A reach Shelter covers the hexes in reach when the card
  resolves; Aegis Grid Lockdown shelters before it places its 3 Nodes, so a node it chains outward is bare while the
  face says "your Nodes ... this round". Options: (a) place first, then shelter (content order, a bump); (b) a
  Shelter that reads reach live (no snapshot to remember); (c) print "in your reach now". Pick (b): a printed game
  can't remember which hexes were in reach at play.
- **D97** The agents' shelter read (`guide.rs` `sheltered_saves`, `eval.rs` shelter term, `moves.rs`) sees only the
  country Shelter flag, which no content sets since r106: sim players time the three Shelter cards blind. Pick: read
  `reach_shelter` (an agents item; their play moves).
- **D98** At 1366×768 from round 6 the countries strip shows about one threat a country; the rest, payments
  included, sit behind a 40 px scroll (playtest). Options: (a) more height for threats in the action phase; (b) one
  compact line a threat. Pick (b). (The alignment row's duplicate Discord deck chip went: F320.)
- **D99** (Matt's call: a rules term) Teardown's Fallout lands under a node standing on the torn-down factory's hex
  (`resources.rs` `burn`), the one way a node and Fallout share a hex; the rulebook now says so. Options: (a) keep;
  (b) put it next to the hex like a project's. Pick (a): Teardown is the player's own choice.
- **D100** (Matt's call: pacing) At 3 countries, d2 (seed 2353), the first project shows 24/27 compute when the
  players first act and resolves in round 2's compute step; the next stood at 24/39. With no cyber card in round 1
  nothing can answer it (Audit needs one), so the opening's first harm (every seat discards 3) is unavoidable.
  Options: (a) leave it (the projects' pressure is the design); (b) the deck's opening projects start empty or
  compute waits for round 2; (c) the viewer marks a project that completes next compute step. Pick (c) now, (b) if
  the playtest finds the opening unfair.
- **D101** Fixed after the hunt-end as F321 (the button names a lone sacrifice).
- **D102** `npm run e2e:corps` is red on main (4cd57d8, built side by side) as on this branch, with the same five
  checks: Horizon's card shows 0 player cubes at round 2 (its card completes in round 2 with 5 player and 5
  adversary cubes), the seat stores 10 cubes where the lane wants 5, and its converter check still expects "a cube
  and 2 energy" for 1 antimatter. The lane's numbers predate this week's station, project and antimatter changes. Pick:
  re-derive its numbers from the current rules (a test item; no player-facing fault found behind it).
- **D103** Fallout's landing hex next to a site sorts the free neighbors by hex number alone (`new_ops.rs`
  `fallout_spot`), comparing one country's #1 with another's #13 and falling back to direction order, where the
  rulebook's §2 tie-break says the lower country first, then the lower hex number. At 3-4 countries 11 site hexes
  have their lowest-numbered free neighbor across a border (Onyx #14's lands on Cintamani #1). Pick: sort by
  (country, number) as printed; a bump.
- **D104** (Matt's call: printed components) Things a table must remember with nothing on it: Rolling Blackouts' and
  Sabotage Teams' flags last through next round after the project leaves the game (and a cordon has no token); the
  adversary's compute and agent incomes are running totals raised by event cards and projects, with no track (and
  the discard mixes resolved and clocked-away events, so they can't be rebuilt). Options: a round-flag marker and two
  income tracks on the adversary board; or leave such projects face up until their flag ends. Pick both markers.
- **D105** The viewer: a card whose map half has no target still reads "ready" at full price (Back-Channel Talks,
  Faraday Yard; the map half skips, as card-grammar says), where a card with only a map half reads "no target"; a
  building's text runs its lasting part and its one-time part together ("Effect: when you remove Agents, gain 3
  credits. Draw 1 card. Remove up to 2 Agents ..."); a played card's target prompt names no card (D85). Pick: a
  "map part: no target" mark on the chip, and "When played:" before the one-time part. The log now names each
  target and a missed one (F318).
- **D83** (extended) `cards:check` also fails on six deck cards whose named scene image isn't served (Penning Trap
  Array, Antiproton Foundry, Cryo Sample Vault, Field Assay Team, Dark Fiber Collider, Treaty Isotope Bank), beside
  D83's three with no scene: an art batch.
- **D98** (extended) The "a seat's last node gone" chip is red from round 1 at the baseline's one node, so it
  signals nothing; mark the seat (or the hex) at its last node instead. Grant's choice offers country names, not the
  megaprojects and their fill (with D85: option words and the agents' parser together). A round-1 threat no one is
  near says so, with no hint how to get near.
- **D85** (extended) The node-protection prompt names no threat ("seat 0's node at Horizon #8 would be destroyed:
  spend which protection?") where other threat prompts start with the card's name.

## Won't fix

| # | Area | Report | Why not |
|---|---|---|---|
| W31 | cards | Peacekeeper Rotation targets Troops; its move onto the target does nothing on a hex with no piece. | Not in the deck; fix with the card if it returns (a consolidate target that accepts any hex). |
| W32 | words | A threat box says "no one near it: no one can counter" while its prompt asks "which of the nearest players takes its harm?". | Both are the rules' words: near (to pay) and nearest (to be hit) are different tests. |

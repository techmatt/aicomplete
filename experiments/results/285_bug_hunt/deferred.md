# 285 bug hunt: deferred (D120 on)

Most important first. Each changes the agents' strength or play, or is Matt's to rule.

- **D120** The keep plan's antimatter count (`agents/keep_plan.rs` `am_makes`) counts the board's printed and revealed
  tags toward a power's sacrifice, which the engine never does (`cards::sources`): the Builder's Overcharge II
  (sacrifice 2 Power) is planned for its antimatter with one Power card in play once the economy track reveals Power.
  `eval.rs`'s `power_progress` does the same for every sacrifice power (Scrap II gets progress from the printed
  Industry). The engine's own test (`TagNeed::payable`) in `am_makes` was screened on 16 paired dev seeds (0-15,
  strategist, baseline and three_seat at d1 and d3): 42 → 40 wins of 64 (two cells identical, two one game worse),
  reverted. Options: (a) fix both with a fresh paired read in an agents item; (b) leave: the over-count keeps
  antimatter cards a little earlier. **Pick** (a), in the next agents item.
- **D121** A two-node Growth option (Sprawl, Outpost, Peer Review) is offered when only one free hex is in its range
  and the second node finds none: paid in full for one node (rare: the second usually lands next to the first; coasts
  and Fallout boards). Options: test each candidate on a copy with the first node placed (`ai::power_blocked`); leave.
  **Pick** the copy test, with the next boards item (it changes play: a bump).
- **D122** `burn_late` (on by default in the agent specs) and keep_plan's burn-or-Reclaim filter match no power since
  r131 (no board power burns or cleanses); the help text describes cut powers. Options: retire the parameter with a
  retired-parameter message (its tests and MOST list) or point it at a current self-spending effect. **Pick** retire,
  in the next agents-and-tools cleanup (removing a param: `git grep` first).
- **D123** Scrap II's free build (and any choice that builds a hand card free) leaves the card marked "17
  unaffordable" in the hand while the prompt asks where to build it (sweep shot c2-d2-strategist step 399): the hand's
  status is the play check, stale during any pending choice. Options: the hand's status reads the pending choice
  (free: "being built"); blank the hand's price marks while a choice is pending. **Pick** the second, a viewer item.
- **D124** The player view's seat strip "+N" is upkeep's income: factories' chips and stations' cubes come in
  production, so a player with a factory sees a smaller chip income than the board's track shows (the hover now says
  so, F393). Options: add production's share to the figure (the economy view's `income.*` plus a production term);
  label the strip "upkeep". **Pick** add it, so the strip matches the printed track (r133's rule).
- **D126** card-pricing.md (*Engines*) and player-cards.md say every engine building pays back within 2 rounds at a
  committed seat; `payback.py` (F407's fix) reads 20+ buildings over 2 at r136 (Consulate Tower 10, Power to the
  People 17, Honeypot Relay 6.5, Preprint Server 5). Options: reprice toward the 2-round bar in a balance pass; restate
  the bar as the range it is. **(Matt's call)**: a pricing rule; restate it until a balance pass reprices.
- **D127** Player view, from the session (screens in out/285_bug_hunt/play/shots/, not kept): at 1366×768 the draft's
  and keep's printed cards push their Draft / Keep buttons below the fold (the fourth card cut off) while "Done keeping
  (pass)" shows, so a new player can pass without seeing Keep; at 2560×1440 the packet wraps two a row and half sits
  off-screen beside an empty buildings column. **Pick**: smaller printed cards at 1366, the packet takes the buildings
  column's width in the draft and keep; one viewer item with D123.
- **D128** Patrol II (2 energy against Patrol I's 1) is offered and charged with no facility in reach to harden: the
  hardening is its whole difference (the any-part rule passes it, as F381's Field Station). Options: refuse it as
  F381 does (a bump); a "no facility to harden" note. **Pick** the refusal, with D121 in the next boards item.
- **D129** Player view, smaller: about 23 changed-hex outlines stay up during a hex answer, so the 2 hexes to pick are
  hard to find (and their blue matches Onyx's border); "unaffordable: the next track space's energy, after the power's
  own" gives no numbers ("needs 4 energy: 1 + 3 for the next space; you have 2"); the war's result and where Drones
  arrive have no note ("War: the fronts fight", then "-8 Drone"); after a loss the turn panel opens at step 1, not the
  step that ended the game; powers read "ready" on the end screen; the tier build's heading says "take one; the rest
  pass on" before any Draft button. Prompts that don't name their power ("take a node off which track?") are D85's.
  **Pick**: one viewer item.
- **D130** Uncountered threats can take their site by their own creation: tier-1 Embargo and Insider Access create 2
  agents on their anchor (a factory or station), and the uncountered threat's anchor agent (`threat_agents` 1, r130)
  makes 3, the facility's number since r134; tier-1 Agitators (3 on a city) with one agent already there, and from
  level 1 (a city falls at 3) by its creation alone; tier-2 Insurrection and Trade Blockade (4 + 1 on a city at 5;
  from level 1 the 4 alone), Garrison State (3 on a city, from level 1) and Sleeper Cells likewise. The tier standard
  says a tier-1 harm is never permanent, and neither the harms ledger (`harms.rs` maps a creation to pieces placed
  only) nor `check.py` sees it. Options: (a) creation counts under the site's number at every level with the anchor
  agent (2 on a city, 1 on a facility); (b) no anchor agent when the card creates on its own anchor (a bump; it fixes
  the facilities and level 0 only, not the city threats from level 1); (c) accept it and have the ledger count the
  capture. **(Matt's call)**: (a) for tier 1, where the standard binds; (c) for tier 2, where a capture fits the tier.
- **D131** "Sleeper Cells" names both the level-1 difficulty rule and a tier-2 threat card, and "War Economy" both
  level 4 and a tier-2 threat. **(Matt's call)**: rename the threats' display names (the ids stay), e.g. "Dormant
  Cells", "Wartime Industry".
- **D132** F416 fixed Overrun only: Offensive deals its tier-2 threats with the first token on a node hex whatever
  stands there, so Garrison State (create at its location: 3 agents) and Iron Curtain and Manhunt (a troop at their
  factory or base location) put their pieces on that node hex though it is no city, factory or base. Options: a
  located line acts only where the token's hex is the kind its face names (as `city_token` now does; a bump); Offensive
  places the first token only for a matching kind. **Pick** the first, in a rules item (alignment.md names Overrun's
  case only).
- **D133** From the second session: at 1366×768 a step's notes sit below the turn panel's step list (scroll inside a
  small panel); a troop's strike has no note naming the troop or that it is used up, and a strike answered in its own
  frame (the owner's protection choice) is missing from the panel; "The adversary troops at X move to which hex?" for
  one troop (`troops.rs` prompt: agents may read prompts, so with D85); "Agents destroy the node" drops that the agents
  go to the supply (keep it when D125 drops the duplicate); "seat 0's node" beside "Seat 0". **Pick**: with D129's
  viewer item; the prompt with D85.
- **D134** The agents' arrival forecast (`eval.rs`'s `fore`: `plan::next_infiltrate`, `next_seeds`) leaves out the
  card's Spill (10 of 16 event cards print one, from level 2) and Grow's agent, so the agents misjudge which sites
  corrupt next turn. Options: a `plan::next_spill` matching `rally::spill` (and Grow's agent through `grow_plan`),
  added to `fore`; leave. **Pick** the forecast, in an agents item (a heuristic: 16-seed on / off smoke, fresh read,
  `speed.py check`).
- **D125** A node lost to 2 agents logs twice ("seat 1's node at Cintamani #10 destroyed", then "Agents destroy the
  node at Cintamani #10"). **Pick** drop the second when `destroy_piece` wrote the first.

# 173: gaps the cold readers found, checked in the engine

Questions the first draft didn't answer, checked in the code by a read-only verifier (paths under
`crates/aic-core/src/`). Each answer is now in the second draft.

| # | Question | Engine | Evidence |
|---|---|---|---|
| 1 | Can land corruption be removed? | No. Nothing removes the token; Clear, Liberate and Cleanse act on markers and cubes only. It doesn't stop a factory or station being built there. | `modules/adversary.rs:1202-1203` (only add); `op.rs:372-380` (`Removable` has no entry); `common.rs:154` |
| 2 | How are projects dealt and refilled? | Setup and each new row: turn up projects until every slot is full, misfits to the discard. A resolved project (once or ongoing) leaves its slot; a new row comes only when the row is empty, before the last project's effect. Cheapest = lowest cost × players. An ongoing project's "Now:" line fires on resolution, its body every round after. | `adversary.rs:395-399, 745-812, 850-878, 437-443` |
| 3 | What is a cordon? | Hardpoint Lockdown only: until the end of next round no agents land in the chosen country from Rally or card lines (troops, Gather and Push still do). The cordoned pool is emptied and lost. | `content/cards.json:2703-2718`; `effects/mod.rs:122-131`; `adversary.rs:970-979` |
| 4 | Open Science: 2 each or 2 in all? | 2 in all, from the supply, split by the Researcher in upkeep; added at the receivers' next round start. | `content/ais.json:950-957`; `economy.rs:163-199, 244` |
| 5 | Rush Build: what is still paid? | Everything but the credits: insight, extra costs, requirements. | `cards/play.rs:98-134, 301-317` |
| 6 | Remove agents leaves agents beside its ambassador: then? | It never cancels them; the next arriving agent cancels it. | `board/standard.rs:437-446`; `adversary.rs:1112-1122` |
| 7 | Field Station: where? | Exactly where Build a research station could go, without the chips. | `effects/resources.rs:63-75`; `board/standard.rs:335-359` |
| 8 | Patrol: corp troops? | No: agents and adversary troops only. No agent onto a firewall, no troop onto the Protector's node hex. | `content/ais.json:559-577`; `effects/moves.rs:26-71` |
| 9 | Draft passing | Odd rounds to seat + 1, even rounds to seat − 1; players sit in player order. | `cards/play.rs:356-364` |
| 10 | Do adversary troops cross borders? | Yes; with no reachable target a troop stays and looks again next march. | `board/troops.rs:198-235, 291-346` |
| 11 | Which bases does a neutral factory feed at a Discord tip? | Each base of its own country (1 at 2 countries, 2 at 3-4), cap 2. | `board/production.rs:14-76` |
| 12 | Who decides team choices; is there an action order? | Seat 0 (the first player) for team choices; a tie for a threat's nearest player is chosen by the tied players. The action phase has no seat order. | `effects/ctx.rs:97-102`; `threats/mod.rs:256-273`; `rules.rs:458-485` |
| 13 | Does a firewall or a Corrupt prohibition stop natural corruption? | A firewall doesn't; a Corrupt prohibition does. | `adversary.rs:1275-1290`; `plan.rs:151-196, 330-337` |
| 14 | Sway step in round 1; corrupted-city Discord | The Sway step runs every round; corrupted cities add Discord in production's city-energy step, unless blocked or blacked out. | `alignment.rs:521-529`; `board/production.rs:300-330` |
| 15 | Converters: any city? | Any city in that country, corrupted ones too. | `common.rs:205-213` |
| 16 | Does corruption leave a marker? | Never (an adversary cube on a site; markers cleared on land). | `adversary.rs:1200-1219`; `board/node.rs:207-211` |
| 17 | Vigil: which hex? | A troop starting a step next to the Protector's node hex, which holds a station or factory owned by any player; once per troop per march; usable this round. | `board/troops.rs:146-169, 238-251` |
| 18 | What counts as a Deploy? (designer, second pass) | Only an event card's Deploy line gets level 2's +1 and a Deploy prohibition; the Protector landing and a project's Deploy get neither. | `adversary.rs:331-355, 1069`; `effects/team.rs:26-33` |
| 19 | Do projects act outside the adversary's turn? (designer, second pass) | Yes: a project resolves the moment compute fills it, and compute also arrives in production and step F. | `adversary.rs:788-812`; `board/production.rs:400, 455` |

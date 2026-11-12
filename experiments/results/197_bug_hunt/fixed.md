# 197 bug hunt: fixed (F272 on)

r86 throughout; no play changed (the golden game's fingerprint held; the content hash moved for the faces).

- **F272** Three places still said projects never destroy a node, though three r85 projects raid one (Night Raid,
  Mob Rule, Sabotage Teams): `docs/rules/war.md` ("Nothing else destroys a node: Discord cards, projects ..."), the
  generated new-player sentence in `docs/rules/alignment.md` (`docs/blocks.rs`), and the rulebook's §13 destruction
  table. All three now name a project's raid.
- **F273** Factory Garrisons and War Plants printed "musters 1 Troop", but Muster is a keyword whose rules (the level
  surcharge, Wall, a no-Muster token) `factory_troops` never applies, and adversary.md says no project musters. Both
  now say "makes"; War Plants gains the "(a player Ambassador on it stops it)" clause the code already applied (one
  op, two wordings). The engine's note and the rulebook's §17 line say "makes" too.
- **F274** Asset Seizure and Station Coup read "with none neutral, its player-owned factory with the most turns
  neutral" (the noun dropped: the most what?). Now "its player-owned factory picked the same way turns neutral". Their
  "goes to the adversary" is now "is corrupted": the code corrupts the hex, which takes a node and player ambassadors
  on it, and the face never said so.
- **F275** `op_harms`' `TakeSite` arm left out `NODE_DESTROYED`, so docs/harms.md didn't list Asset Seizure and
  Station Coup among the node-destroying cards (the corruption above).
- **F276** The clock (Accelerated Timetable) put its discard at the bottom of the event discard, where every flip
  puts its card on top; the viewer's "last:" reads the top, so it named the previous round's card. Now on top
  (`team.rs`); `tests/projects.rs` checks it.
- **F277** The rulebook said each board has six powers besides its node powers; the Builder has five since Focus was
  cut (r86).
- **F278** `scripts/bench/ladder.py --measures` divided project turnover by a 20-card deck; 195 made it 22, so the
  share columns in 195's and 196's ladders read about 10% high. The default is now the content's deck size.
- **F279** docs/card-guidelines.md priced a keep at "1 insight or 5 credits"; credits haven't bought a keep since r82.
- **F280** docs/principles.md cited Market Crash (no longer a project) and a 4-18 project cost range (now 5-14 a
  player).
- **F281** The card viewer labeled every project "Stage undefined" (`web/scripts/cards-export.mjs`); now "Project" or
  "Ongoing project".
- **F282** `drafts/adversary/check.py`'s pull simulation ignored 196's `stage_extra`, so it checked an 11-card d0 deck
  where the engine deals 13. It now mirrors `adversary/mod.rs` (stage I while it has cards, then stage II).
- **F283** `scripts/cards/board_read.py`'s "2nd node" column read `ai.power.researcher.2`, Peer Review since 075. It
  now looks Peer Review up by name and says so.

- **F284** The viewer drew four token types as anonymous hash-colored dots with no name: a city's growth chip
  (`city_chip`, r83) and the prohibition tokens (`no_rally`, `no_muster`, `no_corrupt`, r80), all newer than 184's
  hunt. The sweep failed every game on them (7,316 `token-unknown` across 12 games). Each now has a glyph (a gold coin;
  a "no" sign filled by step) and a hover name (`board.ts`, `format.ts`), and the sweep's token list knows them.

- **F285** (round 2's review of round 1) F275 fixed one of three corruption arms: `SeizeFactory` and `FlipStation`
  (Station Seizure, Factory Occupation) corrupt their hex through the same `corrupt_hex`, taking the node and
  ambassadors there, and their harms rows now say so (docs/harms.md moves).
- **F286** The rulebook's §13 Corruption row named "a threat that corrupts a site" (now "a threat or project"), and
  §17's "What projects do" left out Station Coup ("seizes a research station").
- **F287** The rulebook (§6, §17) and adversary.md paid compute for every corrupted station; the engine (and §9) pays
  it only once that country's megaproject is complete. All three now say so.
- **F288** Accelerated Timetable's face promised "the round cap comes a round sooner", but with the face-down deck empty
  (the second-to-last round) it does nothing. The face now says "the top face-down card ..., if any". Making it bite
  then is D90.

## The checker

`web/scripts/sweep.mjs` gains `COUNTER_EVENTS_R85` (a moment and its checks the first time each project op acts:
`adversary.project_acts.*`, botnets made and spread, sites taken or neutralized, factory troops and blocks, node
raids, hardening stripped, the clock) and a `c3-d4-strategist` case (d4's dearest projects, the track at 3).

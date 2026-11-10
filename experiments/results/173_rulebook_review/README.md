# 173: rulebook review and second draft (r73)

No rules change: no engine, content or `docs/rules/` edit, no `RULES_REVISION` bump. The second draft of
[docs/rulebook.md](../../../docs/rulebook.md) and the merged [friction table](../../../docs/rulebook-friction.md)
come from the review below.

## The review

| Reader | Saw | Did |
|---|---|---|
| G, a general hobby gamer (cold) | the first draft only | 129 paragraph notes; 25 questions |
| V, a Spirit Island veteran (cold) | the first draft only | 139 paragraph notes with false familiarity; 25 questions |
| D, a critical designer | the first draft, the friction table, `docs/rules/` | 157 paragraph ratings; structure; 8 worked examples; friction re-rank |
| 4 verifiers | the engine | every G and V answer checked in the code (file:line) |
| a gap verifier | the engine | 17 questions no draft answered ([gap-check.md](gap-check.md)) |
| G2, V2 (fresh, cold) | the second draft | the same 50 questions plus 10 each of their own, graded against the engine |
| G3 (fresh, cold) and D again | the final draft | G3: 20 questions on the retest's trouble spots; D: the new friction table |

Every answer, with the engine's answer and evidence: [answer-key.md](answer-key.md). Raw outputs: [raw/](raw/).

## Results

| Draft | Readers | Right | Partly | Wrong |
|---|---|---|---|---|
| First (172) | G, V: 50 | 43 | 4 | 3 |
| Second, before its last fixes | G2, V2: 50 shared + 20 own | 111 | 9 | 0 |
| Final, on the trouble spots | G3: 20 | 19 | 1 | 0 |
| Committed, on the last fixes (F4, Deploy scope, projects, Convert, land corruption, the march example) | G4: 7 | 7 | 0 | 0 |

- **The first draft's three wrong answers** came from two text faults. "A threat never takes a last node" is false:
  Overrun and Station Seizure corrupt a site, and corruption destroys a last node from stage II (G19, V17; also
  `rules/threats.md:95`, now M47). And it didn't say that difficulty leaves tag items alone (V19).
- **The retest's 9 partly answers:** 4 traced to one fault the second draft introduced (the threat-harm tie is the tied
  players' choice, not the earliest player's), 1 to a missing definition (coastal city), and 4 to readers leaving out a
  qualifier the book states. All were fixed or clarified before G3's read.
- **G3's partly answer** (F4: what can corrupt a site) was missing Discord cards' and projects' Corrupt lines. It was
  fixed after the read, and a fourth cold reader (G4) answered it and six more questions on the last fixes right
  against the facts checked here (graded by the lead against [gap-check.md](gap-check.md)).

## What the second draft changes (wording, structure, examples)

- **Order of play.** Part 1 teaches in play order (overview, map, setup, round, board, the adversary's turn, cards,
  actions, production, threats, war, Unity and Discord, protection, winning). Part 2 holds the reference: the
  corruption procedure, piece moves, projects, threat dealing, corp troops, powers, difficulty, player count, the legend.
- **Six worked examples:** node cost, corruption, block-liberate-claim, near and nearest, holding Discord off. The war
  case is worked into §11.
- **Tables:** Claim against Liberate side by side; the protection order; last node by source and stage; threat tag
  against card tag; difficulty as one table, a full row per level, with "agent income includes Garrisons".
- **Words.** "Next to" now means sharing an edge, as on the printed cards ("on or next to" when it includes the hex).
  172 had defined it to include the hex, against the cards. Three site states (neutral, a player's, corrupted) plus
  "blocked", with "clean", "aligned" and "dark" dropped (no card prints them). "Natural corruption" keeps the rules
  docs' name: "Overrun" is a threat's name. Node and botnet stay lowercase in prose, like ambassador and agent (pieces);
  the card tokens (Datasets, Followers, Capital, Operatives) keep their printed capitals.
- **Facts the first draft lacked or got wrong,** each checked in the engine: team choices go to the first player;
  Shelter never covers the adversary's turn; war's node rule (only the factory owner's ambassador helps); land
  corruption is permanent and doesn't stop building; corruption never leaves a marker; how projects are dealt and
  resolve, mid-production included; the cordon; Open Science is 2 in all, from the supply; Rush Build pays all but
  credits; Deploy skips a Wall hex to the next factory, else a city; only an event card's Deploy line takes level 2's
  +1 or a Deploy prohibition; every event card has a Rally; a Corrupt prohibition stops natural corruption.

## The friction table

One ranked table, merged rather than appended: 172's 47 entries became 40.

- **Merged:** income timing, station output and cities' energy (#7); near and location tokens (#8); insight and keeps
  (#2).
- **Dropped as fixed by wording or components:** bases and home city (the tiles print them), the round cap (now one
  sentence in §14), the icon key (§23 names every icon).
- **Removed from the evidence where wording fixed it:** the Garrisons double count, "next to", "strike" for cards,
  the cordon's absence from the rules, the draft direction's seating.
- **New:** what counts as a Deploy (#12); a blocked landing moves on or stays by what blocked it (#9); projects
  resolving mid-production (#11); land corruption not stopping buildings (#28).
- **Re-ranked by D's evidence:** factory growth up to #5, projects up to #11, last node down to #10 (rare, and now stated
  right), words down to #17.

Wording-only fixes D proposed that weren't taken: none of substance. Smaller cuts (the Sell repeats, the designer notes)
were taken.

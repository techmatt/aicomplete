# 261 bug hunt: deferred (D106 on) and won't fix (W34 on)

Most important first. Each changes play, the agents' strength or a card's art, or is Matt's to rule.

- **D106** The agents value a research-track removal at its printed cubes whatever the track holds, so the judge plays
  Counterintelligence Sweep on a near-empty track (2.5 cubes a Sweep at r121). The obvious fix (value only the cubes
  on the track for a play now) made the judge weaker on 48 paired dev seeds (strategist, seeds 100-147): wins d0
  89.6 → 83.3, d1 93.8 → 91.7, d2 81.2 → 79.2; Sweeps 0.81 → 0.56 a game, the track fired 0.85 → 1.04 a game at d2,
  cubes removed about the same. An early removal delays the firing as much as a late one, which "cubes on the track
  now" misses. Reverted. Options: (a) value a removal by the firing it delays (the threshold less the track, against
  the rounds left); (b) leave it: the over-valuation plays the card early, which works. **Pick** (b) until a balance
  pass needs the Sweep's use measured; (a) in an agents item.
- **D107** A printed table can't track which cities were bought this round. Before 254, Buy energy's "bought" mark
  only stopped a second purchase in the action phase; since 254 it also lasts into the income step, where a bought
  city hosts no one (`common::host_assignment`). Nothing on the city changes and the Pieces table has no marker.
  Options: (a) Buy energy puts the round-flag marker (or a Bought token) on the city, off in Cleanup, with a Pieces
  row and a line in §8 and §9 (words only); (b) a bought city makes nothing more this round, so production needs no
  memory (a bump). **(Matt's call)**: (a), it costs a component, not a rule.
- **D108** Target-box glyphs show one kind of site where the card takes two or three: Power Purchase Agreement and
  Microgrid Lab Compact (neutral station or factory) draw a station; Hostile Buyout (any corrupted site) draws a city,
  and its liberate icons print `-1 cube city` with nothing for "becomes yours". The claim clause now draws station /
  factory (F335), so Open Research Campus, which only claims the station it opens, draws both. Options: (a) a
  two-glyph target box and per-target claim icons (`op.rs` `glyph`, `web/card/icons.js`, `face.rs`); (b) a generic
  "site" glyph (new art). **Pick** (a), with a `cards:fit` run. (Matt's call on (b): art.)
- **D109** The player view's action bar labels standard actions with the first clause of their rule text ("Target: a
  piece or a Botnet. Use 1 Dispatch: ..."; "Buy its energy (once per city a round): 5 credits (5 credits)", its cost
  twice), not their names (`format.ts` `standard` case), wrapping to several lines at 1366×768. Options: the name
  and cost on the button, the rule text in its hover (the e2e lanes and `odd.mjs` match some labels: update them
  together). **Pick** that.
- **D112** Player view, small: the dotted red hex outline (a threat's location) isn't in the legend and reads as
  corruption; at 2560×1440 the seat panel needs its own scroll to reach the last powers (Outpost, Patrol, Procure,
  Teardown, Field Survey) and grows a horizontal scrollbar once the programs row is long; an unusable converter says
  why ("can't pay it") but not what it trades (the usable one does); the debug view's step bar cuts a node route's
  label in its rule text ("power: Buy In (2 energy): place 1 Node; if the new node brings a factory into your
  reach, a…", 75 captures; the log has it whole), and the megaproject name and status in a country box ("◆ Ti…",
  `truncated`, W16's kind). **Pick**: one viewer item; the step bar names the power and its cost, the rule in its
  hover.

- **D115** The player view's turn panel (headed "The adversary's turn", though its steps start at the players' "D
  Actions" and "E income") lists raw engine events under each step ("resource changed · seat 0 · resource energy ·
  delta +1 · total 1", "note · text ... · hex @-3,4", "card moved · ... · to adversary.projects_done"): W5 accepts
  them in the debug view's log, not in the player view, whose step lines already say it. Its node prompts word a
  track space's insight as "+4 insight" (a level a round, not a one-off), beside the track's "insight 7". **Pick**:
  the panel shows notes only, headed "Since your last move"; "+4 insight a round".
- **D116** A record opened in the viewer that stops replaying isn't saved, so a reload brings back the game before
  it, silently. (Its other half, the error naming what is offered now, is F378.) **Pick**: keep the record's text
  with its error, so a reload shows the same stop.
- **D117** Analysts' tooling (round 6, verified; details in this item's out/ notes, r8.txt): agent specs take silently
  what they should refuse: `plan_credit` is a documented parameter nothing reads since 189; an out-of-range value of
  an enumerated parameter plays as another setting under its own label; `strat0`-`strat2` past a board's menu play
  its first strategy, and one pinned seat makes every seat commit; `t.troops_removed.<anything>` takes any name; the
  target rule prices hexes on the global table, not the spec's `t.` settings. Scripts: `apply_overlay.py` accepts
  `null` on a misspelled field (the engine refuses it); `play_read.py`/`use_read.py` split one agent in two when a
  native folder pools with a farmed read; `design_check.py` ignores an unknown flag or a second DIR; `read_lib --base`
  falls back silently on an arm it doesn't find; `paired_gap` reports new = base on one paired seed. **Pick**: one
  agents-and-tools cleanup item, refusing each silently accepted input.
- **D118** The Liberated marker (253, Matt's ruling 4: "the adversary commits more to take back what you freed")
  costs only a Corrupt line or natural corruption an extra agent; a card that takes or flips a site outright (Station
  Coup, Asset Seizure, a station flip, Overrun) retakes a liberated, unclaimed site with none, and `take_site` treats
  it as any neutral site. The words now say so (F374). Options: (a) leave the scope (the marker answers the agent
  economy, not cards); (b) the take/flip paths skip a marked site, or pay the extra agent from the board (a bump).
  **(Matt's call)**: (a), unless ruling 4 meant every retake.
- **D119** Faces worded against the house terms (round 11, verified; out/ notes r10.txt): "Harden 3 of your Nodes"
  (Stress Test Suite, Early Warning Drill) where the house term and rulebook §16 put hardening on a facility only;
  Faraday Yard's box "a facility" where only a player's uncorrupted one is legal; a threat's Botnet that finds its
  station taken goes to the nearest piece, unprinted; the three research threats say "the adversary gains N research
  cubes" where the projects say "onto the research track" (round 4's verifier ruled this one fine, round 11's not);
  Offensive III's "the adversary builds" (Grand Offensive's words define it); `pull_unity` and Theft each printed two
  ways. **Pick**: one card-words item (house terms first: a node shield's word, then the faces), with `cards:fit`.
- **D113** Counterintelligence Sweep (259: 5 cr, 0 insight, remove 4 research cubes, draw 1) draws at 0 insight,
  against card-guidelines 7 and economy.md ("every card that draws costs at least 1 insight to play"); Fusion Commons
  (r90, draws 3 at 0 insight) too. Options: (a) 1 insight and fewer credits (2 cr: pay 10 against get 13 at the
  anchors, 1.3x); (b) drop the draw and remove 6; (c) restate the guideline. **(Matt's call)**: (a) for the Sweep, a
  card item with its checklist; Fusion Commons with it.
- **D114** Five liberate cards' pricing notes (Hostile Buyout, Root Access, Peacekeeping Mission, Power to the People,
  Popular Uprising) give anchor pays from before the tier-2 keep of 3 and Power to the People's 4 energy, and leave
  out 256's claim rider. **Pick**: restate each from `effect_ledger.py` in the next card item (a price note is written
  by the item that sets the price).

## Won't fix

- **W34** `screen.py`'s `041` preset reads `board.markers` and `board.markers_removed_city`, retired with the corruption
  marker at r98 (`counter_names.py` lists them): a preset that reproduces item 041's screen, kept as its record.
- **W35** "Hosting: the rulebook says the players choose who hosts which city; the engine always gives a contested
  city to the lowest seat": economy.md says the engine hosts as many as can be (a matching); every maximum matching
  gives the team the same energy.

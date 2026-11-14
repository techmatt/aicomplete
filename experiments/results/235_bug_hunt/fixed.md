# 235 bug hunt: fixed (F289 on)

r106 throughout; no play changed: the golden game's fingerprint held, and the content hash moved for the adversary
faces (Siege, four Botnets-spread events, Sabotage Teams, Slander Campaign, Factory Drive). Option strings moved
(plurals, the Liberate hover), so a session saved in the viewer before this hunt may stop replaying at such a prompt.

## Rules text a player reads (r99-r106 left behind)

- **F289** The Discord deck has 7 cards since r99 (Scorched Earth); the rulebook (§1, §12, glossary) and
  `rules/alignment.md` said 6 and a loss on the 7th tip. Now 7 and the 8th. The generated new-player line names
  Scorched Earth's Fallout, and the Fallout drip row drops its stale two-card source list (`docs/blocks.rs`).
- **F290** Face-down Fallout went at r98, yet the rulebook's §12 drip said "face-up Fallout" and the Lift Fallout rule
  row "face up or face down ... the one standard action that removes corruption" (Liberate removes corruption too).
  Now "removes Fallout"; the station row says "Fallout", not "corrupted land" (`standard.rs`, rulebook §8).
- **F291** Rulebook §6 said §21's agent-income column includes Garrisons; the column lists only what a level adds.
  `rules/threats.md` said agent income "starts at 0 below level 4" and compute rises "most at 2" (it is 1 a country
  from Garrisons, and the big step is at level 3, +8 a country).
- **F292** Project costs quoted as 5-22 a board in principles.md and card-guidelines.md; since 233 they run 7-22
  (21-66 at 3 countries, 28-88 at 4).
- **F293** `rules/economy.md`: insight "up to 10, 9 or 8 by board"; the Researcher reaches 9 since r100.
  `rules/ai-boards.md`: Remove agents "spends its ambassador"; it costs 2 since r100.
- **F294** A Shelter was described as covering a country (`rules/war.md`, card-guidelines' legend and guideline 21);
  since r106 every Shelter card covers the player's own nodes and facilities in reach. The rulebook's roof legend gave
  "with a node" two meanings; it now separates a card's Shelter, Aegis and Sanctuary ("from corruption and agents").
- **F295** `rules/README.md` said flatly that no card reaches past 1 from its target, two lines after allowing "in
  its country". The rulebook glossary had two **Target:** entries; merged, in order.
- **F296** Rulebook: Fallout "never on a building" (a building is a card: facility); the §2 site table gave a corrupted
  station 1 compute from round 1 (it feeds an adversary cube to its megaproject until that is complete); §17's
  "shuffle the discard back in" for projects (a resolved or discarded project leaves the game). Fallout placement now
  says a threat's lands on or next to its location, and Teardown's goes under any node there.
- **F297** Siege's face hid that a node on the factory's hex goes with it, and printed "on its hex" for Fallout that
  lands on or next to it.
- **F298** Four event faces and the rulebook said Botnets spread falls back to a research station when no site is free
  next to a botnet; the engine falls back only when the country has no botnet. Sabotage Teams said players lose
  "energy from their nodes" (nodes make none; the engine zeroes boards' and cards' energy). Factory Drive opened its
  factory on a "free site" (a free industrial zone). Slander Campaign's "Pull 4 Sway back" now reads as its op's
  defined phrase, "Remove 4 Unity from the Unity track".
- **F299** The new-player insight rule read "the number on the rightmost space you've uncovered"; some spaces print
  none ("that shows one", rulebook §5, ai-powers.md, `blocks.rs`). The node-power line said "a tag or the board's own
  currency" (Buy In costs credits, Peer Review a cube).

## Faces, prompts and the log

- **F300** Typed jams printed "counts as 6 toward a cyber threat" (read as 6 paid on one option; the jam cuts every
  option, which often counters outright), the untyped one "needs 6 less of each resource", AutoCounter a third way.
  One wording now (`face.rs`).
- **F301** The log said "Agents destroy the node at X" when Sanctuary, a Shelter, Aegis or a hardening kept it; it now
  says the node holds. Night Raid's line said an agent landed before the placement, which an empty supply or a
  canceling ambassador stops. War Plants' line read "makes 2 troop", and botnet troops claimed troops Wall kept off.
- **F302** Prompts: "add 1 agents to which card?" and "count the agents on which card?" (the token key: the face says
  operative), "discard 1 cards" (Supply Raid, Cryptojacking), "lose 1 ambassadors" and "lose 1 nodes", "remove 3
  agent" (Surgical Strike, Ceasefire Monitors), "The discord track tips" and a resolved card's "unity: ...".
- **F303** The Liberate hover priced a factory at the station's 3; it costs 2.

## The viewer

- **F304** Lift Fallout's button showed no price (the view's price has no cubes); it now says "spend 2 research
  cubes". A test holds every cube-priced standard action to naming its cubes.
- **F305** A Shelter card's reach Shelter (r106) showed nowhere; the seat now shows "sheltered hexes N" (its own
  nodes and facilities' hexes), naming them in its hover. War Plants' and botnet troops' tallies count what landed.
- **F306** The agent supply and the Dispatches chips were drawn as warnings while full (the safe end), calming as they
  emptied.
- **F307** The empty "project discard (reshuffles)" pile was always listed; it shows only when it holds a card.
- **F308** A targeted building's use button read "remove up to 3 Agents on or next to it" with no "it"; it names the
  target.
- **F309** The threat-source ring struck through the hex numbers, on the hexes prompts name; the numbers now draw
  above the rings, with a pale halo.
- **F310** The end banner tacked the reason code on ("... no nodes left on the board node"); it moved to the hover.
  Hex-pick labels said "Onyx #12 @-1,2"; the axial pair goes when the option names its hex.
- **F311** Round 1's drafted cards were labelled "opening hand" (the opening hand is the free cards in hand); a draft
  card now shows its tier, which sets the keep price the pick commits to. A part payment read like a full one ("pay 8
  credits → Requisition" toward 12): the button says how much more it needs.
- **F312** After a mouse click, Space (play/pause) pressed the clicked button instead, unseen: it took an action,
  undid another, or (Random seed) replaced the game. A focused button takes Space only when focused from the keyboard.
- **F314** The sacrifice prompt read "sacrifice what for the power's tags?" with "sacrifice stack Intrusion Lab"; it
  asks which card pays the power's tag cost, and a stack reads "(with any buildings under it)".
- **F315** Discord rose with only "var changed: alignment.discord +1" in the log: a corrupted city's diverted energy,
  a dark aligned city and the round's drip (printed and Fallout) now each say where it came from.
- **F316** The card viewer: tier-2 backs printed a keep of 2 insight (3 since r95; `card.js` hard-coded it, now
  the export's `keep` from the config); the AI boards dropped every card-play space's insight and the start 3 (the
  Researcher's first space read "—"); a card's Ops left out its Action's OR half.
- **F317** The phase bar and log headings read engine keys ("deal" twice, "build" for the Card beat's lines, "bases"
  for growth): they now read the rulebook's §4 steps ("B Card", "C draft", "D Actions", "E growth"), the config name
  in the hover. `rules/economy.md` listed `cards.tier` before `.deal`.
- **F318** A target picked unasked (one hex) or missed (none) left no trace but raw token lines; the log now says
  "Seat 0 targets Biomia #3 (a corrupted city)" or that nothing was in reach. The log named a moved threat or
  project by its one-word id ("card backdoor"). A project's hover says its cost a board × the countries (the chip
  shows 0/52 for a printed 13). Lab Purge's face prints the engine's order: one cube at a time from the player with
  the most, ties to the earliest in player order (it read as all 4 from the leader).
- **F319** A second review of the later fixes (7 claims, all real): the Discord notes claimed Discord with Alignment
  off; a war-destroyed factory's Discord went unnamed; the draft hints keyed on the old phase names went silent; the
  option bars lost the "or" between a threat's options; `:focus-visible` turns on at the next keydown in Chromium, so
  F312 now tracks the last pointer press instead; the card viewer printed an OR half's ops but not its cost or tokens;
  the sacrifice prompt said "card" where a stack goes.
- **F320** The alignment row repeated the Discord deck's count beside "Lose if" ("Discord deck runs out 4"), pushing
  the loss group mid-line; the clock shows once.
- **F321** (D101, after the hunt-end) A power whose tags one source alone can pay takes it unasked; the power's text
  and button now name it ("Scrap (sacrifice 1 industry) (sacrifices Payroll Automation)"). A view test holds it.
- **F313** A threat's progress bars split its status on commas only: a sacrifice option ("sacrifice 3 power tags or
  0/17 credits") vanished, and two "or" options ran into one bar. Each option now shows.

## Checkers

- `sweep.mjs`: moments for r99-r106 (`COUNTER_EVENTS_R99`: Fallout placed and lifted, Night Raid, Crackdown, Lab
  Purge, cubes to antimatter, burns, reach Shelters, targets in reach), a Fallout key event (the marker's went at r98),
  and two checks reported for a look: `mark-over-number` (a highlight ring crossing a hex number) and `warn-at-start`
  (a value drawn as a warning at a game's first phase).
- `invariants.rs`: the random games check that a log line about a node's fate matches the board (`log_lies`).
- `adversary.rs`: the node rule's log under Sanctuary and without; `standard_actions.rs`: cube prices in words.

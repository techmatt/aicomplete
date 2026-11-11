# 184 bug hunt: deferred (D77 on) and won't fix (W28 on)

D81 was written for three agent misreads, then fixed in the hunt (F257): its number stays unused.

Each: what a user meets, the options, and the pick. `(Matt's call)` marks taste, scope or a rules term.

## Deferred

- **D77 rules: a flip or a threat corrupts a firewalled or prohibited site.** Station Coup's flip
  (`adversary/corrupt.rs` `corrupt_station`), Lab Takeover and Station Seizure (`new_ops.rs` `force_flip`) and Overrun
  (`corrupt_city`) check neither a firewall nor a Corrupt prohibition, where Corrupt lines and natural corruption check
  both. Projects resolve in the compute step, before firewalls and prohibitions end, so a firewalled station flips.
  (a) Words to the code (done, question 1's default: Rule of Law and the rulebook's firewall name Corrupt lines and
  natural corruption); (b) guard all three paths, a rules bump. **Matt's call**; (a) stands until he picks (b).
- **D78 rules: War Economy's troop lands on the factory under its token**, not on the factory it builds (which since
  r77 is never "here", F230). (a) Keep, as printed now; (b) `at: new_factory`, a rules bump. **Matt's call**.
- **D79 rules: the adversary-factory fallback sends agents to a firewalled factory** (`new_ops.rs` `adversary_factory`,
  players choose among all factories): nothing lands, where a creation line skips a firewalled site. (a) Skip
  firewalled factories (narrow play change, bump); (b) leave: the printed rule ("a blocked landing stays in the
  supply") allows it. **Matt's call**; (a) reads closer to a creation line.
- **D80 words: Great Liberation "frees" 3 sites**, but a city is liberated (agents off, your cube on, Unity) while a
  station or factory only loses the adversary's cube; the rulebook never defines "free". (a) "Liberate or free";
  (b) a rulebook sentence for "free". **Matt's call**.
- **D82 cards: five cards print under 7.5 pt** with no recorded exception (guideline 9): Rally the Allies 5.4,
  Mass Defection 5.8, Surgical Strike 5.8, Double Agents 6.3, Air Gap 6.9 (`npm run cards:fit`). (a) Cut words;
  (b) record each as an exception. **Matt's call** (the text is his).
- **D83 art: ten tier-2 cards (158) have a scene subject and no drawn scene**, so `npm run cards:check` fails on main
  (mass_defection, embassy_of_exiles, double_agents, kettle_operation, lure_network, firewall_array, air_gap,
  rally_the_allies, surgical_strike, open_source_uprising). **Pick:** an art batch (`npm run art:alts`).
- **D84 boards: two powers never used.** Builder's Focus (5 credits for 1 insight) and Protector's Recovery
  (a Security tag for Clear's effect, which Clear sells for 2 credits) read 0 a game in 96 r77 games (16 seeds, three
  pairs, engine and strategist). The boards are placeholders. **Matt's call** with the board redesign.
- **D85 viewer prompts**: "which other player?" names no card or effect (related to D50); "Corrupt factories (at 2):
  the agent at X moves onto which?" leaves "(at 2)" and the noun unexplained, and the agents parse that text
  (`read_converge_prompt`); a gather answer reads "1 seat 1's ambassador" (option labels are record keys: a rename
  breaks old records). **Pick:** prompts only, with the agents' parser, in a viewer item.
- **D86 rulebook: the threat draw can loop at the table.** A forced pull (no card in the reshuffled tier deck fits)
  came 10 times in 96 r77 games (6 games; 146 measured 0.01-0.06 a game: threats that deal threats empty a tier deck).
  threats.md calls it "a content bug" and the rulebook's §18 (discard, turn up the next, reshuffle) never ends there;
  rulebook-friction's ruling rested on "content guarantees it". (a) One rulebook sentence: "If a reshuffled deck has
  no card that fits, take the next card anyway" (what the engine does), threats.md restated; (b) content so it can't
  happen. **Matt's call** (it reopens a friction ruling); (a).
- **D87 viewer: sweep 2's sheets** (off-variants, 4 countries, shared row). Alignment off still draws the whole
  Alignment row with a "Discord deck runs out" loss that can't happen; the counter chips by "Lose if" change order as a track first gets a value (globals
  seeded from sorted vars); the legend has no entry for troop arrows, drawn in nearly a front's red; the Horizon and
  Biomia board labels vanish at 25% opacity; a free-play choice lists cards the hand still badges "unaffordable".
  **Pick:** one viewer item; each is small.
- **D74 (update)**: at r77 the strategist keeps 43 cards it never plays (16 seeds), the engine agent 18.

## Closed

- **D76** reversed by r77 (178, ruling #27: a strike is the player's choice and may stop).
- **D32**'s Hostile Buyout half: a liberation lands no agent since `free_leaves` went (178); the Popular Uprising
  half stands.

## Won't fix

- **W28** The strike heuristic keeps the pre-r77 order (troops first, never stopping): 178 chose it on purpose, one
  search answer a strike, behind the `move_rules` dial (`agents/moves.rs`).

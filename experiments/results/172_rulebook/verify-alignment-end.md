# alignment-end (Sway, the two tracks and tipping, the Unity and Discord decks, the purge, winning and losing): engine-verified rules (172 verifier)

Rules revision 73. Numbers come from `configs/baseline.json` (alignment block lines 120-146 and adversary block lines 50-112) unless marked as content. Note: `modules/effects/endgame.rs` holds only the convert and firewall card effects. No end condition lives there; the end conditions are all in `alignment.rs` and `adversary.rs`.

**Components**
- Two Sway tracks, **Unity** and **Discord**. Both start empty (alignment.rs:490). They are independent: no shared pool, and nothing that adds Unity removes Discord, except the hold-off below.
- Two tip counters, one per side. They start at 0 and only count up (alignment.rs:491, 404).
- Two face-down decks, shuffled at setup, holding every printed card of their side (alignment.rs:492-506). Content: Unity has 23 cards, Discord has 6. Every card prints a tier I, II and III effect. Each side has a face-up resolved pile.

**Tip thresholds**
- The Unity track tips at `tip_per_player` (3) × players. The Discord track tips at `discord_tip_per_player` (3) × players (alignment.rs:198-210). With 1/2/3/4 players that is 3/6/9/12 on each track.
- `momentum_every` is 0 (off).

**Gaining Sway**
- **Discord Sway** goes straight onto the Discord track (alignment.rs:231-235). Sources:
  - Production: each corrupted city diverts its energy (1) and sways 1 Discord, unless a player's ambassador stands on it or a blackout covers it (production.rs:322-330).
  - A factory destroyed in war (not by an adversary troop): 1.
  - The antimatter action: 1.
  - Each land-corruption token at the round's start: `blight_discord` 1.
  - The adversary board's drip: The Militarist prints 0.
  - Threats, Discord cards and adversary projects, as printed.
- **Unity Sway** waits beside the track until the current action or automatic phase has finished (alignment.rs:99-101, 232). Sources:
  - Liberating a corrupted city: 5.
  - Liberating an adversary station or factory: 2.
  - Clearing a corruption marker: 2.
  - Removing a botnet: 1.
  - Removing an adversary troop: 0.
  - Cards and Unity cards, as printed.
- **The round's Sway** is the first step of every round (baseline.json:366). It adds the board's drip and 1 Discord per land-corruption token, and nothing else (alignment.rs:523-531).
- **The antimatter action**: during the action phase, each player may take it once per round (`antimatter_uses` 1, reset at round start). It gives you 1 antimatter and sways 1 Discord (`antimatter_discord` 1) (alignment.rs:533-555).
- **Envoy** (a standard action, which costs 1 ambassador) moves 1 Discord off the Discord track (standard.rs:428-430).
- Some Unity cards "remove N Unity from the Unity track". They remove up to N (alignment.rs:363-369).

**The end-of-action check.** This runs after every action and every automatic phase, once it has fully resolved, including all of its choices. It does not run on a pass or at round start (rules.rs:666-704; alignment.rs:557-610). Steps in order:
1. **Place the waiting Unity.** Seat 0 decides for the team, in one decision, how much of it moves Discord off the Discord track instead (1 for 1, at most what is on the Discord track). The rest goes onto the Unity track. You are offered only "none", or an amount that leaves the Discord track below its threshold. If no such amount exists, there is no question and everything goes onto Unity (alignment.rs:253-292).
2. **Tip full tracks, Discord first.** If the Discord track is at or above its threshold, it tips. Otherwise, if the Unity track is, Unity tips. After each tip, any Unity swayed by that card is placed (step 1 again), and the check restarts with Discord. So a card that refills a track tips it again, and a big Unity gain can tip Unity more than once (alignment.rs:566-589).
   - Before a Unity tip, if any player has no nodes on the board, tipping stops. The same happens after any tip that left a player without nodes (alignment.rs:573-584).
3. **Losses:** a player with no nodes on the board means everyone loses (alignment.rs:591-596).
4. **The purge** (below). If it is met, the players win (alignment.rs:597-608).

**A tip, in order** (alignment.rs:392-440):
1. Take Sway off the track. A **Unity** tip removes one threshold's worth, and the rest stays toward the next tip (`carry` 1). A **Discord** tip empties its track.
2. Add 1 to that side's tip counter.
3. **Discord only:** each neutral factory makes 1 corp troop on each of its corp's bases (production.rs:48-74).
4. Draw the top card of that side's deck.
   - **Discord:** if the deck is empty, the players lose at once (alignment.rs:411-416). The Discord deck never reshuffles.
   - **Unity:** if the deck is empty, shuffle the resolved pile back in first (alignment.rs:376-385). Drawing it out has no other effect.
5. The card resolves at once at the tier set by this tip's number on its side: tips 1-2 tier I, tips 3-5 tier II, tip 6 and later tier III (`tier_at` 3, 6; alignment.rs:179-182, 421). It then goes face up on its side's resolved pile. No card stays in play.
   - Choices on Unity and Discord cards are seat 0's, except where a card asks each player (ctx.rs:97-101).
   - Prohibitions from Unity cards apply to the adversary's next Rally, Deploy or Corrupt. That is this round's if the build step has not passed, otherwise next round's (adversary.rs:1076-1085).
   - Some Unity cards change income or give free keeps "for the rest of the game"; those are tracked values, not cards in play.

With 6 Discord cards, the 6th Discord tip resolves a tier III and the 7th Discord tip loses.

**The purge: the only win.** What must be gone from the board depends on Unity tips made (`purge_at` 3, 6, 6; adversary.rs:1505-1511, 1556-1574):
- 0-2 Unity tips: no corrupted city, no botnet, no adversary troop and no agent on the board.
- 3-5 Unity tips: no corrupted city, botnet or adversary troop. Agents no longer count.
- 6+ Unity tips: no corrupted city. You win the moment the adversary holds no city.
- Corrupted cities always count. Corrupted stations, factories and land never count, and corp troops never count.
- The 0-2 level counts only from round 1's adversary card on (`clear_win_stage` 1), which in practice is always.
- The purge is checked after the tips in the same check, so a Unity tip can ease the purge and win in the same action.

**Ways to lose**
1. A Discord tip finds the Discord deck empty. Checked in the tip itself.
2. A player has no nodes on the board, and everyone loses. Checked at step 3, and also before any Unity tip.
3. The adversary deck runs out: at a round's event step there is no card to turn up (adversary.rs:290-293). The deck holds 11 cards at difficulty 0-2 and 9 at difficulty 3-4. This comes after that round's Sway step and its check (baseline.json:366-367).

Running out of supply is not a loss, and neither is every city being corrupted. The engine also ends a game as a loss at round 1000; that is a safety stop, not a rule.

**Variant `off`.** No Sway moves, nothing tips and there is no antimatter action. The purge stays at its hardest level, and the node loss and adversary-deck loss remain (alignment.rs:228, 561-565).

## Player count and difficulty

Player count (1-4; players = countries = seats, rules.rs:336):
- **Tip thresholds scale linearly.** Unity tips at 3 × players and Discord at 3 × players (alignment.rs:198-210): 3/6/9/12.
- **Most other amounts do not scale.** Sway from liberating a city (5), a station or factory (2), clearing a marker (2) and removing a botnet (1) are fixed per event.
- **Fixed counts.** The tier steps (tips 3 and 6), the purge steps (Unity tips 3 and 6) and the Discord deck size (6) are all fixed, independent of players.
- **Effect on pace.** One city liberation tips Unity at once in solo (5 ≥ 3, carrying 2), but fills less than half the track with 4 players (threshold 12).
- **Discord sources** scale roughly with the board, since there are more cities and more land-corruption tokens with more countries. The antimatter action is per player, each player once per round.
- **Seat 0 decides team questions:** the hold-off amount and the choices on Unity and Discord cards (alignment.rs:281; ctx.rs:97-101). In solo that is the only player.
- **Node loss** checks every seat: any one player losing their last node loses the game for all.

Solo has no other special rules in this area.

Difficulty (adversary.difficulty 0-4):
- **Difficulty 4 only:** one Discord card (`discord_removed` 1) is cut from the shuffled Discord deck, leaving 5 cards. The Discord tip counter starts at 1 (alignment.rs:500-512). The real Discord tips therefore resolve tiers I, II, II, II, III, and the 6th real tip loses. At difficulty 0-3 it is I, I, II, II, II, III, and the 7th tip loses.
- **The round cap** (adversary deck) is 11 rounds at difficulty 0-2 and 9 at difficulty 3-4 (stage2_removed 2 at 3+; the swap at 4 keeps the count; adversary.rs:251-258).
- **Discord tips never raise difficulty.** No other alignment number changes by difficulty.

Variant `alignment=off` (a measurement variant): no Sway and no tips, the purge stays at full strength, and the node and deck-out losses remain.

## Mismatches (engine vs docs/rules)

- **"The steps are a list: agents, then troops, then botnets (two entries: botnets always count)."** (docs/rules/alignment.md, The purge paragraph (after the purge_ease table)): engine: purge_at is [3, 6, 6], which has three entries. At 6 Unity tips both later steps are reached, so botnets stop counting (only corrupted cities count). The generated table in the same doc says so, so this prose is stale. (configs/baseline.json:131-135; crates/aic-core/src/modules/adversary.rs:1505-1511)
- **Tipping: the Sway comes off, the tip count goes up, and one card is drawn and resolves (the full list of a tip's steps).** (docs/rules/alignment.md, Tipping): engine: A Discord tip also makes 1 corp troop at each neutral factory, on each of its corp's bases. This happens after the counter rises and before the card is drawn, so it happens even on the tip that loses to an empty deck. It is documented only in war.md and ambassadors.md. (crates/aic-core/src/modules/alignment.rs:407-416; crates/aic-core/src/modules/board/production.rs:48-74)
- **"Corrupted cities' diverted energy sways Discord in production (1 per corrupted city)."** (docs/rules/alignment.md, paragraph after the Sources of Sway table): engine: A corrupted city with a player's ambassador on it diverts nothing and gives no Discord. A city under a blackout is skipped as well. (crates/aic-core/src/modules/board/production.rs:322-330)
- **Losing when the adversary deck runs out is a loss when a round starts with no card (adversary.md: 'a round starts with no card to turn face up (step 1)').** (docs/rules/alignment.md Losing list; docs/rules/adversary.md 'Players lose when a round starts...'): engine: The round's first step is alignment.sway, and the event flip that detects deck-out is second. The final round's Sway step and its end-of-action check (tips, losses, purge) resolve first, so a Discord tip there can end the game on the empty Discord deck instead. (configs/baseline.json:366-367; crates/aic-core/src/modules/adversary.rs:290-293)
- **A prohibition applies "in the next adversary turn (this round's if its build is ahead)".** (docs/rules/alignment.md, Fear-card shape paragraph): engine: 'Ahead' is tested as phase <= the build phase's index. The end-of-action check runs before the phase index advances, so a Unity tip during the build phase's own check is assigned to this round even though this round's Rally and Deploy have already run. Only Corrupt can still use it. This is an edge case and needs Unity swayed during the build step. (crates/aic-core/src/modules/adversary.rs:1076-1085; crates/aic-core/src/rules.rs:731-740)
- **"a tip's card may fill a track again, which then tips too" (no limit)** (docs/rules/alignment.md, Tipping): engine: The tip loop is capped at 64 iterations per end-of-action check. This is an undocumented safety cap; a printed game has no such limit. (crates/aic-core/src/modules/alignment.rs:561-566)

## Hard to state

- [high] **Hold-off: Unity swayed during an action waits, then seat 0 decides once for the team how much moves Discord off instead, and only amounts that leave the Discord track below its threshold are offered.**: It has deferred placement, a team decision by one seat, and an 'only if it prevents the tip' restriction. That takes more than three sentences and adds hidden pending Sway, and a player would not predict the restriction. Proposed: Whenever you gain Unity, choose at once to put it on the Unity track or remove that much Discord. Drop the offered-amount filter, so any Unity spent on a track that still tips is simply wasted.
- [medium] **Asymmetric carry: a Unity tip removes one threshold's worth and keeps the overflow; a Discord tip empties the track.**: The two tracks look identical on the board but behave differently, and players would not predict that from the board. Proposed: Treat both the same way. Either both keep overflow, or print 'empty this track when it tips' on the Discord track only.
- [medium] **Tier selection: a card's tier is set by its side's tip count including this tip (tier I on tips 1-2, II on 3-5, III from 6). At difficulty 4 the Discord counter starts at 1 because a removed card counts as a tip already made.**: The thresholds are off-by-one prone ('this tip included'), and the difficulty-4 pre-count is a hidden adjustment. Proposed: Print the tier bands directly on each tip-count track (spaces 1-2 I, 3-5 II, 6+ III). At difficulty 4, start the Discord marker on space 1 as a setup instruction.
- [medium] **Purge levels by Unity tips (0-2 everything, 3-5 no agents, 6+ cities only), with a config list [3,6,6] that skips the troops step and a clear_win_stage gate.**: The prose describes three steps (agents, troops, botnets), but play has only three levels and the troops-only level never occurs. clear_win_stage is inert at 1, which adds a term used once. Proposed: Print the three levels on the Unity tip track and drop the stage gate from the rulebook while it is 1. Say 'at 6 Unity tips, win when the adversary holds no city.'
- [medium] **Order of the end-of-action check: place Unity, tip Discord before Unity, re-check after every card, stop before a Unity tip if a player has no nodes, then losses, then purge.**: It has five ordered steps with a mid-loop exception (the node check before a Unity tip). Proposed: Use: 'At the end of each action: resolve Discord tips, check losses, resolve Unity tips, check the purge.' This makes the node-loss-before-Unity rule fall out of the order instead of being an exception.
- [medium] **A Discord tip makes 1 corp troop at each neutral factory.**: It is a forward reference to war rules, hidden inside the tip procedure and not stated in the alignment rules. Proposed: Print it on the Discord track ('when this tips: each neutral factory makes 1 corp troop, then draw a Discord card') or move it into Discord card text.
- [low] **A corrupted city diverts 1 Discord in production unless a player's ambassador stands on it (or a blackout covers it).**: It needs an 'unless' exception, stated in another area. Proposed: Keep the rule but print it on the production step of the player aid next to diversion: 'each corrupted city with no ambassador of yours: +1 Discord'.
- [medium] **Prohibition timing: 'the next adversary turn (this round's if its build is ahead)'.**: Its timing is conditional on the phase, and the edge case at the build step itself behaves unexpectedly. Proposed: Say 'until the end of the next Corrupt step', or let a prohibition last until the end of the next round's adversary turn regardless of when it is made.
- [low] **Round cap: the adversary deck running out is checked at the event step, after the round's Sway step.**: 'The round starts with no card' is not literally true: a Sway step (and possible Discord tip) happens first. Proposed: Move the Sway step after the event flip, or state the cap as 'after the last adversary card's round ends, you lose'.

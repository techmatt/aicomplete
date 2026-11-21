# Friction: d4 spot check, seed 268010 (rounds 0, 1, 3, 5 played by hand)

Table time by the brief's rates (3 s a component, 10 s a bookkeeping step, 20 s a lookup, 15 s a non-trivial decision, 5 s a
modifier): setup 22 min, round 1 25 min, round 3 37 min, round 5 44 min. Bookkeeping steps are 40-50% of every round's time.

## Numbers the brief asks for

**The two-beat adversary turn (B rows; round 1 / 3 / 5):** bookkeeping 28 / 50 / 58 (+6 in round 5's Sway for Standing Army),
components 15 / 53 / 54, lookups 2 / 1 / 6; about 6 / 12 / 15 minutes of table time, a third of each round.
- Escalate: 0 / 4 / 6 checks, **0 agents in all three rounds**. Commandeered Plants: 5 / 9 / 5 checks, **0 agents**. 29 checks
  for nothing.
- Infiltrate: 2 / 8 / 4 agents on 1 / 8 / 2 sites; round 5's 4 met 3 ambassadors (one-for-one each).
- Arrivals checked: 2 / 21 (Infiltrate 8, Cells: factories 8, threat anchors 5) / 10.
- Corrupted sites' outputs: 8 / 11 / 19 (round 5: 4 cities, 4 stations, 7 corrupted factories each read against its arrow, 4 botnets).
- Compute step: 32 / 46 / 45 compute; 0 / 2 / 2 projects resolved, each with a slide and a refill mid-pile.
- Research check: 1 / 1 / 1, but round 3's fired Cells: factories (8 more arrivals) and round 5's fired a Strike (3 troops,
  Wall on 5 Protector hexes, Vigil capped at 2).

**Threats:** dealt 2 / 6 (+1 waiting) / 6; 10 / 13 / 10 location tokens. Paid in full: 1 / 1 (+1 jammed to 0) / 0. Executed:
1 / 5 / 6, with 2 / 12 / 13 bookkeeping steps and 11 / 44 / 38 components. Five part-payments (1 credit, 3 x 1 credit, 2 cubes)
changed nothing but had to be put on and handed back.

**Ambassadors:** moves 3 / 3 / 4; contacts with agents 0 / 1 / 5; Liberates 0 / 0 / 2 (each: the ready check, the agents-on and
troops-on-or-next checks, the 3-ambassador bill split site / supply, then a Unity tip). Converts (Double Agents, Embassy of
Exiles) made 5 ambassadors from agents in round 5: which are ready was not written anywhere.

**Troops (round 5 only):** 2 landed (Standing Army II), 2 redeployed (distance from every player piece to 3 corrupted cities), 3
struck (2 targets for one, 1 for the others, Wall on the Protector's hexes), 3 removed by cards and powers.

**The Builder's action phase:** 7 / 3 / 7 decisions, 25 / 11 / 30 components. Its discount (1 credit a building per 3 Industry)
never applied: it had 1 Industry tag. The Protector's did, and I missed it (an error).

**Unity and Discord tipping:** Discord tipped 0 / 1 / 2 times (round 5: in the Sway, then in the Income beat); Unity twice back to back
in round 5's action phase. Each tip: take off 6 or 8, move the tip marker, read the strength from the aid's strip, resolve a
card that reads the whole board: 4-8 bookkeeping steps each.

**Rulebook length:** 90.8 KB, 1,335 lines, 24 sections, plus the 12.4 KB aid; about 50 minutes to read once. 29 lookups in 3
played rounds plus 4 in setup.

## What d4 costs beyond d2 (from the level ladder and this game)

| dial | d2 | d4 | what it cost here |
|---|---|---|---|
| Compute income | 5 a country (10) | 15 a country (30, 36 by round 5) | 32-66 compute a round onto projects; 8 projects resolved in 6 rounds, two in one step in rounds 3, 5 and 6: each a slide, a refill and a remainder to hold (+1 memory, about 6 bookkeeping and 8 components). At 10 a round the same costs (24-48) land about every 2-3 rounds. |
| Infiltrate on cities | 1 a city | 2 a city | Twice the agents on city lines (round 1: 2; round 5: 4) and twice the ambassadors cancelled; station, factory and botnet lines unchanged (round 3's 8 factories cost the same at d2). |
| Arms Race | off | 2 troops a country a Garrison | Round 4's garrison put 4 troops, not 2; round 6's would have put 4 more. Each troop is a Strike's worth of targeting and Wall checks. |
| Event deck | 4 / 4 / 2 | 2 / 4 / 3 | Stage II (tier-2 threats, Supply Lines' two) from round 3, not round 5; stage III's 2 tier-2 a country from round 6, not round 9. With Accelerated Timetable and Incursion, 23 threats were dealt in 6 rounds. |
| War Footing (d2 too) | 4 compute an unpaid threat | same | 16 and 24 compute waiting in rounds 3 and 5: more of the d4 compute is this. |
| Round cap | 10 | 9 (8 after Accelerated Timetable) | nothing to track: the deck is the clock. |

Net: per played round d4 added about 8-12 agents and their arrival checks, 1-2 extra project resolutions, 1-3 extra threats, and
(late) twice the troops. None of the level rules is on the aid: the "2 a city" Infiltrate, War Footing's 4 compute and Arms
Race's 2 troops each cost me a lookup to §21 or §6.

## Friction rows

| rule (§) | what it cost | how often met | how hard (one line) | the smallest fix |
|---|---|---|---|---|
| §6 Escalate + Commandeered Plants | bookkeeping | every round: 29 checks in 3 rounds | 29 checks, 0 agents: the cost is in proving nothing happens | **A**: a line on the aid "only sites holding an agent: scan for agents, not sites"; **P**: drop Commandeered Plants when it repeats Escalate on factories |
| §6 corrupted factory output (arrows) | bookkeeping, error risk | round 5: 7 factories | H11 and C14 each touch two cities; the arrow is the only tiebreak and it must be printed | **C**: print every zone's arrow on the map (the content dump has no arrows) |
| §17 two projects in one compute step | memory, bookkeeping | rounds 3, 5 (and 6) | hold the remainder across a slide and a refill | **A**: "write the remainder on the compute track's next project before sliding" |
| §10 jam to 0 = paid | error | round 3 | the engine discards it; the rulebook never says so | **W**: "a price jammed to 0 is paid: discard the threat" |
| §10 neither price possible | error | round 5 (Turncoats) | "pay what they have and discard their hand" reads as both | **W**: "discard your whole hand (and pay nothing)", or say both if both is meant |
| §10 part-payments | components | 5 times in 3 rounds | resources put on and handed back for no effect | **W**: "put resources on a threat only when they complete a price" (the take-back rule then does nothing at the table) |
| §10 three threats on one hex | error | round 3 (C7) | each anchor agent is separate; corruption mid-sequence hides the last one | **A**: "after each threat: its harm, then its agent" as a two-step row on the aid |
| §8 contact as "remove Agents" | error | round 3 | Reconciliation Office fired on an ambassador-agent contact | **W**: one line in §8 or the glossary: "a contact removes the agent: Effects that count removed agents fire" |
| §7 Industry discount | error | round 5 | a second reading of a tag dial that nothing prompts | **C**: a discount pip on the Industry dial at 3, 6, 9 |
| §18 Incursion II's text | missing text | round 5 | "deal 1 extra threat for each corrupted city": no tier, no country | **W**: say tier 2 and which country (III does) |
| §18 fallback token, located lines | lookup, ambiguity | round 5 (Rootkit) | which of Rootkit's lines are "located"? Its botnet line has its own fallback; its agents line is "here" | **W**: mark located lines on the card face |
| §12 Open Market | memory | round 5 | "the next 3 cards kept are free" lasts into next round with no token | **C**: 3 free-keep tokens to hand out |
| §3 War Rooms "every Security threat" | lookup, bookkeeping | setup | primary tag or any tag? 57 cards sorted | **W**: "every threat with a Security tag"; **C**: print the build lists |
| §21 level rules not on the aid | lookups | 4 in played rounds | Infiltrate 2 a city, War Footing, Arms Race read from §21/§6 only | **A**: a level strip on the adversary board's aid: the three active rules |
| §2/§9 cities across a border | error | round 1 | a node pays from a city in the other country | **A**: production line "check cities in both countries" |
| §11 Redeploy distances | bookkeeping | round 5 | 3 troops x 3 corrupted cities x every player piece | **W**: fine as written; **A**: "a city with a node on it is distance 0" |
| §8 converts and gathers | memory | round 5 | which ambassadors are ready after Convert and Gather | **W**: one line: "converted ambassadors arrive ready" (or tipped) |
| §20 sacrifice not shown | memory, reconstruction | rounds 3-5 | which card a tag cost took decides slots later; I rebuilt it from slot counts | **C**: a played-pile for sacrificed cards, face up |
| Card faces, long text | lookups at the card | Open Market, Incursion, Standing Army, Checkpoint Network, Embassy of Exiles, Exodus Corridors, Station Seizure | 30-50 words each, three strengths on Unity/Discord cards | **C**: one strength a row, the current tip strength marked |
| Rulebook length | reading time | once | 90.8 KB plus the aid; 33 lookups in setup and 3 rounds | **W**: put the level-4 numbers in §3's setup list |

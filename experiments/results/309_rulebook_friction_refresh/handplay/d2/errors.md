# Errors: d2 hand-play (seed 268000)

Three errors in rounds 1-6. Round 7 isn't measured: I saw its check before resolving it (log.md, end of round 6).

| round | step | what you did | what the engine did | the rule (§) | why: misleading text / missing text / slip |
|---|---|---|---|---|---|
| 2 | D Protector | I read the script's Safehouse Network choice "Gain" as the first half ("add 1 Operative here"). That left P 1 credit short for Logic Bomb and 5 credits low at round end (15 against 20) | It took the second half: pay 2 Operatives, get 5 credits (Operatives 2 -> 0) | §7 (*Buildings and slots*: "A, OR B" is one use), the card's face | **slip**, made likely by the card: "add 1 Operative here, OR pay 2 Operatives, get 5 credits" gives its halves no names, so a table that says "I'll take the gain" is ambiguous. Running out of credits was the warning. I didn't trust it and carried a negative balance on |
| 4 | B Card: threats / F Threats | I couldn't execute Orbital Strike (Horizon, tier 1). I left research at 2 and compute waiting at 8 | It went unpaid: 2 cubes onto the research track (4) and +4 compute from War Footing (12 waiting). Its anchor agent was stopped by the Hardpoint Lockdown cordon | §10, §18; adversary-cards.md's tier-1 table | **missing text**: Orbital Strike isn't in adversary-cards.md. Garrison State (round 5) and War Economy (round 7) are missing too, but neither cost me a number (the first was paid in insight, the second never executed) |
| 4 | D Protector | I played Agency Directorate for its "When played: Add 3 Operatives" only. P's insight ended at 0 | Agency is itself a Security card, so its Effect "When you play a Security card, add 2 Operatives here; at 4, gain 4 insight" fired on its own play: 5 Operatives, 4 come off, +4 insight (P 4) | §7 (*Buildings and slots*: "A building can trigger itself") | **slip** against text that is there but far from the card: one sentence in §7, nothing on the face. A self-triggering building is easy to miss because its "When played" line looks like the whole on-play story |

Near misses (I got them right, but only after a lookup and some doubt; friction.md counts them):
- A threat jammed to 0 (Force Protection Order on Call-Up, Civic Watch on Insider Access and Propaganda Blitz, Threat Hunt on Shock Doctrine): §10 never says a threat needing 0 is paid off by itself. I treated each as paid, and the engine agreed (no harm, no agent, no War Footing compute).
- Neighborhood Watch (round 1) resolved after the liberating ambassador was spent, so I placed no token. The engine agreed (round 2's Infiltrate cancelled the parked ambassadors).
- Back-Channel Talks (round 7) had no corrupted factory in P's reach, so I played it for its +2 Unity only (§7: play a card if any part acts). The engine agreed.
- Plant Seizure (round 6) corrupted H2 and C3 and took B's nodes there (§6, §15). I wasn't sure a creation line destroys a node through corruption, but it does.

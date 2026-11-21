# Errors: d4 spot check, seed 268010 (rounds 0, 1, 3, 5 played)

7 errors in 3 played rounds (0 in setup). Each was corrected to the engine's state before going on.

| round | step | what you did | what the engine did | the rule (§) | why: misleading text / missing text / slip |
|---|---|---|---|---|---|
| 1 | E Production | Paid H14's energy to the Builder only (Protector 3 energy) | Paid the Protector too (4): its node on C13 is next to Horizon's H14 | §2 (next to, across borders), §9 (cities) | slip: "across country borders too" is in §2, but a city in another country is easy to miss when you scan your own country's cities; nothing on the board marks it |
| 3 | D Protector / F Threats | Treated Casus Belli, jammed by Shared Threat Intel from 4 ambassadors to 0 (11 credits to 5), as unpaid and resolved it (3 Discord, 2 Fallout tried, +4 compute, anchor agent) | A threat jammed to 0 of a price is paid off: discarded with no action | §10 (paying it off, jam) | missing text: §10 says "pay one way in full" and that a jam "needs less of each resource", never that a price at 0 is already paid |
| 3 | F Threats | Put 1 agent on C7 after Manhunt corrupted it, forgetting Deepfake Wave's own anchor agent (C7 is its first token too) | 2 agents on C7 | §10 (each threat lands 1 agent on its first token) | slip: three threats anchored on one hex; the third one's agent was lost in the corruption bookkeeping |
| 3 | carried state | Carried the Builder's supply ambassadors as 0 | 1 in supply all round | (state) | slip: misread my own transcription of round 2's end; the ambassador count is a pile of figures in two places (supply, map) |
| 3 | D Protector | No credits for Reconciliation Office when the Protector's ambassador landed on H10's agent (both removed) | +2 credits: "When you remove Agents, gain 2 credits" fired on the contact | §8 (every contact is one for one), card text | misleading text: §8 says the agent and the ambassador "are removed"; whether that is "you remove Agents" for an Effect is not said |
| 5 | F Threats | Turncoats (pay 4 ambassadors or discard 3; Builder had 2 and 2 cards): paid the 2 ambassadors and discarded the 2 cards | Discarded the 2 cards only; the ambassadors stayed | §10 ("with neither possible, they pay what they have and discard their hand") | misleading text: "pay what they have and discard their hand" reads as both |
| 5 | D Protector | Fusion Torch Array at its printed 8 credits | 7 credits: the Protector had 3 Industry tags (Pact, Prefab, Antiproton Foundry), so 1 off each building | §7 (discounts: every 3 Industry tags) | slip: the third Industry tag arrived in a round I did not play; the tag dial would show it, but the discount is a second reading of that dial that nothing prompts |

Not counted as errors (my reconstruction, confirmed): Annex's chip in round 1 (from the Pact, played just before); Interdict II
in round 3 paying "2 Security" with a 1-Security card shown in the script (the engine evidently took Checkpoint Network too: the
slot count in rounds 4-5 only works that way); Incursion II's country split (the script showed it).

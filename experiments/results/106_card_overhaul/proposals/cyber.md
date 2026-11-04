# 106 proposal: Cyber (primary tag Cyber)

25 cards (24 today): 18 tier 1, 7 tier 2; 12 programs, 9 buildings, 4 events; median printed cost 9 (today 6.5),
big cards at 18, 22 and 40. Four cut, five new. The cards are in `cyber.json`.

**The engine.** Cyber turns fighting into a payout. Its hacks drain compute and remove ambassadors. Security Ops Floor,
Intrusion Lab and Honeypot Relay pay for every one of those removals, and Darknet Exchange, Exploit Broker and Hyperscale
Botnet make the next hack cheaper. Its income is in programs: Ransomware Syndicate gives credits per Cyber tag, Neural
Mirror insight per 2 Cyber tags, and Cryptojacking Campaign gives a flat 8. A Cyber seat that plays one Cyber card a
round with Darknet and Broker out pays about 0 credits net for it and collects 3 insight from Ops Floor.

**Pricing.** Pay = credits + 2.5 × insight + the keep (2.5 tier 1, 6.5 tier 2) + other costs at `values.json`
(chips 1.5, antimatter 7). Get = the effect at the rates. Income counts for 4 rounds (tier 1) or 2.5 (tier 2). Tags
count 1 for the first and 0.5 for each one after. Engines are priced at a Cyber seat's fire rate: about 1 Cyber card
a round, about 1 compute removal a round (its drain programs), about 1 ambassador removal a round, and 2.5 tags a round
on all cards played. The generic rate is given where it differs. Four cards use the new extension ops, so I priced
them by hand: Neural Mirror and Ransomware Syndicate (`production_per_tag`) and Command-and-Control Grid (`play_tag`
with no tag). Zero-Day Auction was also priced by hand: its op is `gain_per_tag` of antimatter, a combination the
ledger has not seen. The effect ledger (run on the other 21) reads every trigger card low, because it prices them at
the generic fire rate (Darknet fit 6.9, Exploit Broker 6.8, Ops Floor 9.1). That is the gap the brief asks us to
price past.

| id | status | reasoning | value check (pay vs get, when, for whom) | loop and cap |
|---|---|---|---|---|
| eviction_worm | changed | Cyber's ambassador hack, raised from 6 to 8 and to 10 credits. It is the one that fires Honeypot Relay | 10+2.5+2.5 = 15 vs 8 × 2 + 1 = 17 (+2), +6 more with Honeypot. Mid-game, any seat facing a crowded country | none (one-off) |
| logic_bomb | changed | price only, 9 to 11. It was netting +9.5 | 11+2.5+2.5 = 16 vs drain 10 (11) + 1 troop (11) + 1.5 = 23.5 (+7.5). Protector / Cyber, any round | none |
| shadow_broker_feed | kept | Science-niche clear on a Cyber card, already fair | 8+2.5+2.5 = 13 vs 2 markers 12 + draw 4 + 1.5 = 17.5 (+4.5) | none |
| shared_exploit_kit | changed | the help hack was trivial. Now drain 11, and the other player gets 3 insight (was 6 and 2), for 10 credits | 10+2.5+2.5 = 15 vs 12.1 + 0.75 × 7.5 + 2 = 19.7 (+4.7) | none |
| zero_click_breach | changed | the big single drain went 12 → 18 for 10 credits. At 2 insight it is the card Hyperscale Botnet discounts | 10+5+2.5 = 17.5 vs 19.8 + 1 = 20.8 (+3.3). With Darknet and Hyperscale, 5 + 2.5 + 2.5 = 10 (+10.8), and Ops Floor adds 7.5 | none |
| neural_mirror | changed | was +1 insight income. Now the per-tag insight program: "raise your insight income by 1 per 2 Cyber tags" | 8+0+2.5 = 10.5 vs, at 4 Cyber tags, 2 × 2.5 × 4 + 1.5 = 21.5 (+11). At 2 tags, 11.5 (+1): a mid-game play | insight → Cyber plays → tags. Counted once, at play (a one-off) |
| adaptive_malware | changed | drain-all 6 was break-even. Now drain-all 8 for 6 credits, still needs 3 Cyber (the chain is the price) | 6+2.5+2.5 = 11 vs 8 × 1.65 + 1 = 14.2 (+3.2). Cyber seat, mid-game | none |
| threat_hunt | kept | typed counter program (jam 5 cyber + drain 4). Kept at the family's shape and price | 5+0+2.5 = 7.5 vs 3.5 + 4.4 + 1 = 8.9 (+1.4). With Darknet it costs 1 | none |
| ransomware_syndicate | **new** | the per-tag credit program: "raise your credit income by 1 per Cyber tag on your cards", needs 3 Cyber | 9+2.5+2.5 = 14 vs, at 4 tags (3 + itself), 4 × 4 + 1 = 17 (+3). At 6 tags, 25 (+11). Cyber seat, rounds 3-5 | credits → more Cyber cards → tags. Counted once, at play |
| cryptojacking_campaign | **new** | the tier-1 big card (TM's "cost 40, gain 10"): raise credit income by 8 for 22 credits and 2 insight | 22+5+2.5 = 29.5 vs 8 × 4 + 1 = 33 (+3.5). With Darknet + Hyperscale, 17+2.5+2.5 = 22 (+11). Early, any seat with the cash; best for Cyber | credits → none back |
| spoofed_credentials | kept | Credential Harvest: insight per Cyber tag, as an event (it leaves no tag) | 5+0+2.5 = 7.5 vs at 4 tags 10 (+2.5) | none |
| signal_hijack | kept | cheap troop event, fair | 3+2.5+2.5 = 8 vs 1.48 × 7 = 10.4 (+2.4) | none |
| security_ops_floor | changed | **fighting compute pays, the strong one**: "when you remove compute from an adversary project, gain 3 insight" (was 2 insight), 10 credits. Review: the proposal's "2 insight and 2 credits" box ran about 88 characters, past the building's 80; one resource at the same value fits (today's 2-insight box is 75) | 10+2.5+2.5 = 15 vs 7.5 a fire × 1 a round × 4 + 1 = 31 (+16). At the generic 0.4 a round, 13 (−2): a Cyber seat's card | insight → drain programs → insight. Capped by the cards in hand (each kept and played at ≥1 insight) and one fire per card / Action that removes |
| honeypot_relay | changed | **fighting ambassadors pays**: 6 credits a removal (was 3), at 10 credits | 10+2.5+2.5 = 15 vs 6 × 0.8 × 4 + 1 = 20.2 (+5.2). At 1 a round with Eviction Worm, +10. Mid-game, any seat that removes ambassadors, Cyber first | credits → removal cards → credits. Capped by cards in hand and one fire per removing card / action |
| darknet_exchange | changed | **Cyber credit discount**: 5 off Cyber cards (was 3), at 10 credits (was 3) | 10+2.5+2.5 = 15 vs 5 × 1 a round × 4 + 1 = 21 (+6). Floor 1: Threat Hunt and Signal Hijack go to 1. Cyber seat, early | discount → more plays → more Exploit Broker credits. Capped by cards in hand, the keep and insight per play (floor 1) |
| sentinel_mesh | changed | typed counter Action building, now cyber-typed like Garrison Command, Civic Watch and Market Watchdog. Jam 4 (they print 3) because jam 3 typed nets below 0 at the rates | 4+2.5+2.5 = 9 vs 4 × 0.7 × 4 × 0.7 + 1.5 = 9.3 (+0.3) | none (Action, once a round) |
| intrusion_lab | changed | **counts toward antimatter**: "when you remove compute from an adversary project, add 1 Agent here. At 3, gain 1 antimatter". Was energy → drain 4, which traded 4.5 for 4.4. Agents are Security's token, so the card carries Security as its second tag | 6+0+2.5 = 8.5 vs 7/3 × 1 a round × 4 + 1.5 = 10.8 (+2.3). About 2 antimatter a game for a Cyber seat: the gate for Singularity Exploit and the other ultimates | antimatter → Singularity Exploit → compute removed → a token. Capped by supply (Singularity is one card, one-off) and by `Content::check` (a threshold places no tokens) |
| exploit_broker | **new** | **tier-1 Cyber trigger engine**: "when you play a Cyber card, gain 5 credits", needs 2 Cyber, 9 credits (Citation Engine's shape) | 9+2.5+2.5 = 14 vs 5 × 1 a round × 4 + 5 (it fires on itself) + 1 = 26 (+12). Generic 0.45 a round: 15 (+1) | with Darknet, a Cyber card at 6 credits costs 1 and returns 5. Capped by cards in hand: each costs a keep (5 credits or 1 insight) and ≥1 insight to play |
| killswitch_directive | changed | the project discard was 8 of value for 17.5 paid. Now discard + drain-all 10, at 10 credits | 10+5+6.5 = 21.5 vs 8 + 16.5 = 24.5 (+3). An event leaves no tag. Late, anyone racing a project | none |
| root_access | kept | Cyber's liberate capstone, already fair | 40+2.5+6.5 = 49 vs 45 + 7 = 52 (+3) | one-off, tier 2 (player-cards.md) |
| singularity_exploit | changed | antimatter ultimate: remove a botnet + drain-all 12 (was + discard, which netted −6). Needs 3 Cyber (was 2) | 10+7+5+6.5 = 28.5 vs 12 + 19.8 + 1 = 32.8 (+4.3) | antimatter (Intrusion Lab / Auction) → this. One-off |
| zero_day_auction | **new** | the antimatter payoff: "gain 1 antimatter per 2 Cyber tags on your cards", needs 5 Cyber | 6+5+6.5 = 17.5 vs at 6 tags (5 + itself) 3 × 7 + 1 = 22 (+4.5). At 8 tags, +11.5. Late, Cyber seat only. Priced by hand (antimatter by tag is new to the ledger) | antimatter → ultimates. One-off, capped by the ultimates in hand (1 antimatter each) |
| hyperscale_botnet | changed | **insight discount**, over-X: built over a Cyber building, needs 3. "Each round, 2 insight. Effect: your cards cost 1 insight less." No chips; 6 credits (review: 8 netted +0.9, and the discount bites only for a seat with 0-1 Science tags, since `science_every` already takes 1 off at 2) | 6+2.5+6.5 = 15 vs 2 × 2.5 × 2.5 + 0.7 a round × 2.5 × 2.5 + 1 = 17.9 (+2.9), with no new slot. The floor (1) means it only bites on cards printed at 2+ insight, so Cyber's big cards print 2 | insight → plays. Capped by the floor (no card below 1 insight) |
| autonomous_defense_grid | changed | typed standing capstone. The one-off "raise credit income by 2" went (a building's one-off), and the price went 13 → 9 | 9+2.5+6.5 = 18 vs 5 × 1.5 × 2.5 + 1 = 19.75 (+1.75). Home-country Cyber threats | none |
| command_and_control_grid | **new** | **tier-2 Cyber trigger engine**: "when you play a card, gain 5 credits per tag on it" (`play_tag`, no tag), needs 3 Cyber, 18 credits | 18+5+6.5 = 29.5 vs 5 × 2.5 tags a round × 2.5 + 5 (itself) + 1 = 37.3 (+7.8). A three-tag card pays 15. Late, Cyber seat | credits → plays → credits. Capped by cards in hand (keep + ≥1 insight each) |
| blackout_script | **cut** | drain 5 for 4 was the trivial hack | Zero-Click Breach (drain 18) and Logic Bomb cover the single drain | |
| botnet_takedown | **cut** | botnet + drain 8 was a near-copy of Singularity Exploit | Singularity Exploit (botnet + drain-all 12) and Root Access (botnet with a liberation) | |
| black_ice_citadel | **cut** | standing drain 3 a project: 7 + chip + 2 insight for about 12 of value (−8) | Intrusion Lab and Ops Floor carry compute-fighting. Autonomous Defense Grid is the standing tier-2 capstone ("each tag ends in one") | |
| quantum_decryptor | **cut** | a Science-tag payoff (2 insight per Science tag) printed Cyber first. It belongs in Science's section, not Cyber's | Science's own triggers (Telemetry Archive, Citation Engine). See flag 2 | |

## Flags for the merge

1. **Buildings 9, not 6.** Six of the seven assigned ideas are buildings by rule: a discount, an insight discount, two
   fight payouts, a threshold and two trigger engines. Add the typed Action building and the typed capstone and that is 9.
   Programs are 12. Cutting to 6 would drop an assigned idea or a typed counter.
2. **Quantum Decryptor cut.** `docs/rules/card-grammar.md` cites it as the `play_tag` example; Science may want it back.
   Black Ice Citadel is cited in `adversary.md`, and Blackout Script there too: prose to fix in the rules commit.
3. **Autonomous Defense Grid** no longer matches the other three typed capstones (13 credits + "raise your income by 2").
   Their sections should move the same way, or the four drift apart.
4. **Sentinel Mesh is typed, jam 4.** Its family prints jam 3, which nets about −1.6 at the rates.
5. **Intrusion Lab holds Agents**, Security's token: Cyber has none. It carries the Security tag for that reason.
6. **The insight discount is weak by construction**: the floor of 1 leaves 1-insight cards alone. Hyperscale Botnet
   carries 2 insight income to be worth building, and Cyber's big cards print 2 insight so the discount has targets.
7. **Antimatter**: Intrusion Lab and Zero-Day Auction give a Cyber seat about 4-5 antimatter a game. Its rate (7) is
   known to be high (card-pricing.md); its real worth is the 1-a-card gate on ultimates.
8. **Review: Command-and-Control Grid is the only credits-per-tag (`play_tag`, no tag) trigger.** Science's
   Interdisciplinary Institute printed the same card (3 credits a tag); the reviewer moved it to 1 insight a tag, so
   each section's every-tag trigger pays its own currency (Diplomacy ambassadors, Power energy, Security Agents).
9. **Review: three flat "+8 credit income" programs.** Cryptojacking Campaign (22 + 2 insight), Science's University
   Spinout (24) and Security's Multiyear Defense Budget (24) print one effect. The merge should keep one or vary them.
10. **Review: one typed program.** Cyber's typed counters are Threat Hunt (program), Sentinel Mesh (Action building)
   and Autonomous Defense Grid; there is no second typed Cyber program (none today either: Security also has one).
   The brief's "two programs per threat type" is short here; not added, since a new card would push the section past 25.

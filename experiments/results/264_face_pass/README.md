# 264: card face pass for the fit exceptions

Rules revision 124, no bump: no card's effect changed (the golden fingerprint holds; only the content hash moved,
for threat and power wording). Base: main at b71e12574b (after 263).

## The fit

At the base, 15 faces printed below 7.5 pt, not the six 258 recorded: 262-263's faces and the threshold buildings
added nine. Each box's size is fixed by the painted frame (`layout.json`: the zones are art), so the levers were
words, icons and the fit ladder. Per-face before and after, text and pt: [faces.md](faces.md) (every face whose
text or size moved, 40 of them).

| Lever | Faces fitted | Meaning |
|---|---|---|
| House term **Rescue** (liberate it, then claim a facility next to it), `face.rs` folds the claim rider into the liberation's clause: icons the liberation's chip, then station / factory | Root Access 6.3, Peacekeeping Mission 6.3, The Nuclear Option 6.9 (also reworded at 7.5: Power to the People, Citywide Microgrid, Forensic Audit) | Same: the term is the rider's words; the Liberate action, which never claims, keeps "Liberate" |
| "within 2 of yours" for "within 2 of one of yours" (Annex's text too) | Survey Drone Fleet 6.9 | Same |
| No "Effect:" before a "When ..." trigger (Matt, message 12166; card-grammar.md) | Open Assembly 6.3, Agency Directorate 6.3, Conglomerate HQ 6.9 (with the icon step) | Same: the "When you" opens it |
| Fit ladder step `icons-1` before the text steps down, kept on the later steps: 4.5 mm icons (3.9 mm above the art; the track arrow stays narrow), tighter gaps, text at 7.5 pt (`card.js`, `card.css`) | Kinetic Interdiction 5.8, Annihilation Lance 5.8, Collateralized Grid 5.8 (two icon rows to one or less height) | Same icons, smaller |
| "when you remove compute" for "... adversary compute" (no player holds compute) | Intrusion Lab 6.3 (with the two above) | Same |

Tried and dropped: joining two removals into one adversary chip (6.3 pt alone; fit only with the sway icon glued on,
a one-card hack); widening the text column at the icon step (no change); dropping "in your color" from Particle
Accelerator (it reads as paying your own research cubes).

**Not done (4), each needing a play or grammar change** (trial fits in `out/264_face_pass/t4.json`, `t8.json`,
`t9.json`):

- **Popular Uprising** (5.4 pt, as before). "Rescue it" fits its words, but its one Action icon row (→ +1 Follower | OR
  −4 Followers → liberation chip, station / factory) is too wide. Without the claim icons it fit at 7.5, but then the
  chip alone would mean "liberate" on Hostile Buyout and "liberate and claim" here, so the icons stay (asked of Matt,
  default taken). Shortest play change that fits: its OR half liberates without the claim rider (7.5 in trial).

- **Retained Earnings** (6.3 pt; 88 characters, the top box holds about 78). Fits: "at 4, gain 12 credits" for "raise
  your credit income by 3" (a play change, Conglomerate's reward), or the shorthand "at 4, +3 credit income" (a
  grammar change: "Raise your X income by N" is the house phrase on about 30 faces).
- **Particle Accelerator** (6.3 pt; 93). Even "... to an open megaproject." without the OR half is 6.9. Shortest
  that should fit: "Action: pay 2 energy, gain 3 research cubes." (drops the megaproject half; untested).
- **Positron Storage Ring** (5.8 pt; 95, two ideas in a building's top box). Dropping the draw (r119's gain) is still
  6.9; shortest: drop the Effect, "Action: pay 2 energy, get 1 antimatter, draw 1 card." and reprice (untested).

`npm run cards:fit` now exits 1 on any box below 7.5 pt (amendment 12121), so it fails on these four until settled.

## D119 (261's deferred list): faces against the house terms

- **Hardening on nodes:** the Facility house term, rulebook §13 and card-grammar.md say "Harden N of your Nodes"
  (Stress Test Suite, Early Warning Drill: `shield_node`) puts the same token on your nodes' hexes. One token, one word.
- **Faraday Yard** and the Protector's **Bulwark**: "Target: a player's uncorrupted facility" (`board::hardenable`).
- **Threats** (in `drafts/adversary/`, loaded by `load_adversary.py`): a Botnet whose station holds one goes "on the
  nearest piece without one" (printed now); the three research threats put "N cubes onto the adversary's research
  track" (the projects' words); `pull_unity` prints "Remove N Unity from the Unity track" everywhere; Theft prints as
  every pay-or-discard threat ("... or discards 2 cards. The adversary gains 4 compute."): it is Requisition plus
  compute either way.
- **Offensive III:** "Each country rallies its Agent income now, to this round's Rally; then Corrupt cities ...",
  Grand Offensive's words for the out-of-turn build.

## D114: pricing notes

Hostile Buyout, Root Access, Peacekeeping Mission, Power to the People and Popular Uprising restated from
`effect_ledger.py` at r124 with the tier-2 keep (6.5 at the table) and the claim rider. Hostile Buyout reads 1.51 at the
table (pay 46.5, get 70), just over the 1.5 ceiling: a price call for a card item, not this one.

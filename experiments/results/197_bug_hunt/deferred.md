# 197 bug hunt: deferred (D88 on) and won't fix (W29 on)

- **D88** (Matt's call: a rules term) `take_site` drops guarded sites before it asks whether any are neutral
  (`team.rs` `take_site`). A country whose only neutral factory (or station) is firewalled or holds a no-Corrupt token
  falls through to "with none neutral", and the player's own site turns neutral. The rulebook tells players to
  firewall the target; doing so moves the hit onto their own site. Options: (a) a guarded neutral site counts as
  neutral, so nothing happens (a play change: bump, goldens); (b) print it: "with none neutral (a guarded one doesn't
  count)". Pick (a): it is what the face and the rulebook's advice say.
- **D89** Power counters are keyed by position (`ai.power.<board>.<k>`), and every reader resolves names from the
  current `content/ais.json`. Focus's cut shifted the Builder's Procure from 7 to 6, so a reader run on a pre-r86
  record calls Focus's uses Procure's (196's `cells.py` `focus` column already notes it). Options: (a) key the counter
  by the power's name (records change, play doesn't); (b) warn when a record's revision differs from HEAD's. Pick (a):
  any later cut or insert moves the indices again.

- **D90** (Matt's call: a rules term) The clock with the face-down event deck empty: Accelerated Timetable resolving in
  the second-to-last round discards nothing (F288 printed that). Options: (a) keep it, as printed now; (b) discard the
  face-up next card instead, so the next flip ends the game (a play change: bump). Pick (a): (b) ends a game with no
  round's warning, and the project rarely resolves that late.
- **D91** The sweep reports 12,739 `truncated` (with hover) a 12-game run, mostly project boxes and adversary card
  lines in the adversary strip (`div.project` 170x24, `div.atext`); their full text is on hover. A look, not a
  failure: worth a layout pass on the strip (taller project boxes at 1366x768) if Matt wants faces readable at a
  glance.

## Won't fix

- **W29** Botnet Conscripts turns a botnet into troops with a player ambassador on it. The face prints no exception,
  and the factory analogy in adversary.md and the rulebook is scoped to Wall; 195's review kept it.
- **W30** ai-powers.md's line on 196's Focus read ("with the track starting at 4") is accurate: it cites the tighter of
  the two tracks read.

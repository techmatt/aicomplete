# 169 bug hunt: fixed (F205 on)

r72 → r73. Each line: the fault as a user met it, then the change.

## Rules (r73)

- **F205** A Push (Checkpoint Network, its Action, the Protector's Patrol) could move an adversary troop onto the
  Protector's node, which Wall says no troop enters. `moves::may_arrive` refuses a troop onto a walled hex;
  ambassadors.md says so. Test `a_push_moves_no_troop_onto_a_protector_node`.
- **F206** Incursion I-II and Two Fronts I dealt every extra threat into country 0: `per_site` ran the untiered
  `deal_threats` once a site, and each call began again at country 0. One batch now (`effects::run`, `PerSite` over a
  lone untiered deal), one per country in turn as alignment.md says. Test `extra_threats_for_each_site_go_one_per_country`.
- **F207** Honeypot Relay's strike paid its "when you remove agents, gain 3 credits" only when the seat already had
  another fight trigger in play: the fight watch was skipped while the card was still in hand. `fighting_play`
  watches a card that brings its own fight trigger, on a play and on a free play (Rush Build, `play_free_building`).
  Test `a_building_s_own_play_fires_its_fight_trigger`, both paths.

## Words players read

- **F208** The house words (r70; amendment 7657): the adversary's pieces printed as "ambassadors" in the adversary
  cards' and projects' income lines (six boards, three projects), the viewer's labels ("adv. ambassador income",
  "adv. ambassador pool", "on board: N ambassadors", the country rows' "amb.", the agent token's name), the purge line
  ("ambassadors no longer count"), the rules prose (alignment, adversary, economy, README, threats, board,
  card-grammar, war, ambassadors, ai-boards: the cancel rule, "theirs"), the generated difficulty, Discord-sway, harms
  and game-end text, and the pricing notes; the Security token as "an Agents token" or "Agent" (player-cards,
  card-guidelines); "Matt to confirm" on Operatives. All now agents / Operatives.
- **F209** Liberate's printed cost said "from your supply": since r70 the ambassadors standing on the site pay first
  (yours, then others'), the supply the rest (`std_rule_text`, economy.md).
- **F210** ambassadors.md's new-player summary promised a stage-III liberation discount and "a liberated city takes an
  agent straight back"; both were gone (r67, `free_leaves` 0). card-pricing.md still priced the discount.
- **F211** alignment.md said a prohibition's tier III reaches "on or next to"; Rule of Law III and Demilitarized Zone
  II-III print and act "within 2".
- **F212** cards.md, alignment.md and `AlignmentCard`'s doc said the tips *before* this one pick a card's tier; the
  engine counts this tip.
- **F213** adversary.md said Decommission removes botnets by "a player's piece"; tiers I-II count ambassadors only.
- **F214** The game-end summary told new players the fear cards act "for each city holding an ambassador", a
  condition no card uses; it now names the real ones (sites a player owns, corrupted sites, pieces nearby).
- **F215** war.md called troop March "the game's one movement rule"; Gather and Push move troops too, at any
  `troop_steps`.
- **F216** player-cards.md: "the seven events also sway" (six events and a program).
- **F217** Supply Raid and Cryptojacking printed "discards 1 cards".
- **F218** ops.md, ambassadors.md and the house terms (player aid) said every Gather lets the player pick each piece
  and stop; a strike picks only the hex and takes troops first (the code's design, now the words; D76).

## Viewer

- **F219** A firewall (Firewall Array, Air Gap) was drawn as the default disc with no name (sweep: 91 `token-unknown`
  in agents' games). A brick-wall glyph and "firewall (no agent may arrive here)".
- **F220** A pending Unity prohibition showed only as raw vars under *Unlabeled tracks* ("prohibit.rally.where 3", a
  bit mask). The status strip's tracks (beside *Lose if*) show "no Rally (round) 6" with what it skips on hover; the
  raw vars are bookkeeping. Test `a_pending_prohibition_is_labeled_in_the_view`.
- **F221** "pick a hex (N)" counted answers, not hexes: a Gather or Push offers two batch sizes a hex, so it read 4
  where the prompt said 2 hexes.

## Read tools and probes

- **F222** `endgame.rs` (the round-cap probe) counted no Unity card that removes botnets or agents from r71 on
  (`remove_where` unread), so `endgame154.py` overstated the botnet shortfall; nor a player card's strike gather.
- **F223** `ladder.py` pooled every agent of a read into one win % a difficulty and judged the band on the blend (168's
  `s168-band`: strategist and greedy): a row per agent now.
- **F224** `ladder.py --screen` keyed games without their config, so a `read_specs --offs` screen's card-off configs
  overwrote base (the last config read as base). A config after the spec's first is a lever named by its file.
  Both in `test_read_tools.py`. The measures table labels by agent when either side has two.

## Checks added

- The sweep: r70-r72 moments (`COUNTER_EVENTS_R70`: gathers, pushes, cancels, strikes, cleanses, `remove_where`,
  standing liberations, prohibitions made and fired), two strategist cases at d1 (`c3-d1-strategist`,
  `c2-d1-strategist`), the firewall's glyph, and `stale-word` (a hard fail): an old house word on screen.
- `aic-sim docs --check` (the fast lane) refuses a stale house word in content, the rules, the generated faces, the
  pricing notes and `web/src` (`docs/spelling.rs`, `stale_failures`).

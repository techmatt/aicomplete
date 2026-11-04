# 101: where each Defaults-and-deviations reading went

`docs/rules.md`'s *Defaults and deviations* (57 numbered defaults, no deviations) was dissolved when the rules
split into `docs/rules/`. "Dropped" means the file that governs it already stated the same rule (the place is
named); "added" means the reading went into that file as an ordinary rule. Play is unchanged (no rules revision).

| # | Reading (short) | Disposition |
|---|---|---|
| 1 | Bases placed by the layout | dropped: board.md, *Country pieces* |
| 2 | One player per country, seat *s* in country *s* | dropped: board.md, *Setup* |
| 3 | Used player ambassadors gone; adversary ones to supply | dropped: ambassadors.md lead |
| 4 | Opposing ambassadors cancel on meeting | dropped: ambassadors.md, *Opposing ambassadors cancel* |
| 5 | `factory_units` per base; `unit_cap` | dropped: war.md, *Factories*, *Troops and war* |
| 6 | Spread range, priority, surplus to supply | dropped: adversary.md, *Each round* 2 |
| 7 | Clean city's energy with no node near is lost | dropped: economy.md, energy row |
| 8 | Destroyed node leaves the game; facility takes its cube | dropped: war.md |
| 9 | Troops target stations and factories, not nodes | dropped: war.md (`troop_node`) |
| 10 | Deck runs out = no face-up card (round cap) | dropped: adversary.md, *The deck* |
| 11 | The supply is a cap | added: README.md, *Terms* |
| 12 | Pay or discard with neither possible | dropped: threats.md |
| 13 | Team choices asked of seat 0 | dropped: README.md, *Players choose* |
| 14 | `adversary_discord` 0 | dropped: alignment.md, Sway table note |
| 15 | "Next to" / "adjacent" | dropped: README.md, *Terms* |
| 16 | When win and loss are checked | dropped: alignment.md, *Winning and losing* |
| 17 | Tip tier by count before the tip; antimatter per round | dropped: alignment.md (`antimatter_uses`) |
| 18 | Discord cards: extra threats, ties, Offensive | dropped: alignment.md, Discord card notes |
| 19 | Difficulty 1's country rotation; difficulty 4's look-ahead | added: adversary.md, *Difficulty* |
| 20 | Pooled counters, withdrawals, refunds | dropped: threats.md, *Countering* |
| 21 | Shared row dealt `per_country` per country | dropped: threats.md |
| 22 | Draft passes alternate direction | dropped: player-cards.md, draft |
| 23 | Open Market counts drafted keeps by anyone | dropped: alignment.md *Details*, player-cards.md |
| 24 | Building with no free slot built over one | dropped: player-cards.md, slots |
| 25 | Insight set each round; income in upkeep | dropped: economy.md, insight row and round table |
| 26 | Tags counted on every card in play | dropped: player-cards.md, tags |
| 27 | `ai.boards` repeats for more seats | dropped: ai-boards.md |
| 28 | Gift / transfer cards not offered alone; Fieldwork | added: card-grammar.md, *Help cards* |
| 29 | Node-track uncovers are per-round income | dropped: ai-boards.md, node track |
| 30 | The clock gates factories, troops, chips, war only | dropped: war.md, the clock |
| 31 | Setup factories start neutral | dropped: board.md, *Setup* |
| 32 | A corrupted city builds within its own country | dropped as obsolete: no current rule has a corrupted city build anywhere but itself (arrivals, adversary.md); see the report's findings |
| 33 | A node is one token per hex of any player; shares hexes | added: README.md, *Node* ("of any player"); sharing already stated |
| 34 | No botnet on a clean city or a piece with one | dropped: adversary.md, *Botnets* |
| 35 | Sabotage stops upkeep energy | dropped: threats.md table |
| 36 | Card-built factories are the player's | dropped: war.md, *Factories* |
| 37 | Project cost per player; compute spending | dropped: adversary.md, *The project deck* |
| 38 | Ongoing projects act in the build | dropped: adversary.md, *Each round* 1 and projects |
| 39 | Tip empties bucket first; no reshuffle | dropped: alignment.md |
| 40 | Alignment decks face down; tip resolves at once | dropped: alignment.md |
| 41 | The march order | dropped: war.md, the march |
| 42 | Undecided starts at `undecided_per_player` | dropped: alignment.md lead |
| 43 | Purge the Unaligned: what each player picks | added: war.md, node loss (the pick; the last-node rule was there) |
| 44 | Threat round flags last to the end of next round | dropped: threats.md table ("next round"); Shelter: card-grammar.md ("this round") |
| 45 | Round 1 is the opening, tier 1 | dropped: player-cards.md, *The opening* |
| 46 | Draft pick dealt face up; empty decks | dropped: player-cards.md, draft |
| 47 | Neutral city's energy sale question | dropped: economy.md, energy row |
| 48 | Corrupted city's cube diverted as Discord | dropped: alignment.md, Sway table |
| 49 | Power payment order | dropped: ai-boards.md, powers |
| 50 | Only buildings hold tokens | dropped: player-cards.md, *Tokens* |
| 51 | "Up to 2 hexes" is hex distance; spread moves each once | added: card-grammar.md (moves), adversary.md (spread) |
| 52 | A late threat executes next round | dropped: threats.md |
| 53 | Ambassador on an adversary factory sways and stays | dropped: war.md, *Factories* |
| 54 | Hardening only on clean player facilities | dropped: ai-boards.md, hardening |
| 55 | Fallback liberation timing, lower seats first | dropped: ambassadors.md |
| 56 | "On or next to" counts the city's hex | dropped: README.md, *Next to X* |
| 57 | Clear a city: which hex while more than N stand | added: card-grammar.md, *Clear a city* (all go when no more than N) |

The inline **Default** labels (about 25, in nine files) were dropped with their legend; each sentence they marked
stays. One, war.md's clock, read "**Default** (Matt confirmed)" and now reads "As Matt confirmed".

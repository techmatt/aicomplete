# 176: rules r75, the player side of the rulebook-friction rulings

Built on main 17d6c2b (r74, 175). `RULES_REVISION` 74 → 75, bumped from the constant (the prompt said 75 too).
Matt's rulings of 2026-11-10 on `docs/rulebook-friction.md`, as the prompt quoted them; amendments 7996 and 8041
(the later one wins where they differ: liberate cards stay as they are). Seeds: fresh 42133-42276 (next free 42277);
dev 36800-36815 for screens.

## What r75 changed (engine)

| Ruling | Engine | Where |
|---|---|---|
| 1. Contact, 1 for 1 | An arriving agent cancels one player ambassador; with several players' there, seat 0 chooses whose (`board::agent_cancels`, a loss prompt), for every arrival: Rally and creation (`adversary::place`), a Corrupt line's converge, Gather and Push. **Remove agents** spends `remove_ambassadors` (1), places nothing, and takes up to `remove_count` (3) agents one at a time from any hexes in reach (the player may stop). | `ambassadors.rs`, `adversary.rs`, `effects/moves.rs`, `standard.rs` |
| 2. Claim and Liberate | One bill (price + 1 an agent on the site), paid by the acting seat's own ambassadors on the site first, then its supply, never another's (`site_supply_cost`, `pay_site`; 166 spent anyone's). `claim_city` 4 → 5. No botnet clause in `can_claim` (the embargo stays). **Liberate puts the seat's cube on**, sways Unity as before, lands no agents. | `ambassadors.rs`, `standard.rs` |
| 3. Botnets | Standard action **Take down a botnet** (`botnet`): `botnet_credits` 5, a botnet on a hex holding your ambassador (it stays), `alignment.botnet` Unity. Move may now go onto a botnet's hex. | `standard.rs`, `board/mod.rs` |
| 4. Factory growth | Every city builds on a free zone next to it or nowhere; a corrupted city's on the adjacent zone by an agent, else the lowest-numbered; a neutral city's factory is neutral (`zone_owner` is gone). | `production.rs` |
| 5. Protection | `destroy_piece` (now `Res<bool>`): Shelter keeps the piece, nothing spent; otherwise the owner (seat 0 for a neutral piece) chooses among a corp troop at each adjacent base (troop strikes), the hardening token, their own ambassador, the Aegis Doctrine (their own pieces, once a round), or none. The site owner's ambassador no longer guards another player's node. War's node on a destroyed factory, and Scorched Earth's, go through the same choice. A declined hardening token goes with its facility. | `destroy.rs`, `troops.rs`, `new_ops.rs` |
| 6. Income | `round`: `board.produce` (factories, aligned cities, stations, megaprojects), `economy.upkeep`, then `board.bases` (growth: extra corp troops, factories). Insight stays at round start (amendment 7996). **Buy energy** (`buy_energy`): a neutral producing city in reach, once per city a round, `city_energy_credits` (4, below). | `production.rs`, `standard.rs`, `configs/baseline.json` |
| 7. Unity and Discord | The hold-off offers every amount (145's filter is gone); `carry` 1 → 2: both tracks keep their overflow. | `alignment.rs` |
| 8. Corp troops | At a Discord tip each ambassador on a neutral factory stops 1 of its troops (seat 0 picks the spared base only when it can't stop them all); a full base spends none. | `production.rs` |
| 9. Converters | Every converter `once: true`; use needs a city of the country in reach (`common::city_in_reach_in`). | `content/map.json`, `board/mod.rs` |

## Decisions

- **Buy energy at 4 credits, not the ruled 5.** Give 5 for 1 energy (4.5 at both the table and the anchor) is a loss,
  which CLAUDE.md refuses. At 4: give 4, get 4.5. The prompt's 5, screened beside it on the final binary (16 seeds a
  pair, [screens.md](screens.md)): energy bought 2.0 a game against 2.6-2.9, wins flat within SE at every level.
- **Liberate cards unchanged** (amendment 8041): they still leave the agents and put a cube on only with `align`. 178
  takes #29/M1. **Card claims of a station** likewise still pay an agent's extra from the supply only (flagged).
- **Shelter stays automatic** (it spends nothing, so there is nothing to choose); **Scorched Earth's** node is
  guarded as war's is (one protection rule); **corp-troop ambassadors** on a factory go in seat order, the players
  choose only which base goes without.
- **"At once"**: Unity of one action is placed as it resolves; tips are checked after, so the hold-off is "at once"
  for every case a tip can see.

## Give / get (at `configs/agents/values.json`, anchors in card-guidelines.md)

- Buy energy: give 4 credits (4), get 1 energy (4.5): +0.5. At the ruled 5: −0.5 (fails).
- Take down a botnet: give 5 credits (5), the ambassador stays; get a botnet removed (anchor 16) and 1 Unity (4): +15.
- Remove agents: give 1 ambassador (4); get up to 3 agents (5 each): +11 at 3.
- Claim a city: 5 ambassadors (20) for an aligned city's energy and Unity; never taken by the sim players at r74 or
  r75 (an ambassador costs 20 credits): flagged for 177. No player card changed: no `play_read` needed.

## The sim players' new decisions

Every new decision's agent code was reviewed; then a 17-agent review (4 finders by ruling, each finding verified by
a skeptic) found 15 faults, 12 fixed before the freeze:
- **Search-answered prompts** (whose ambassador an agent cancels, which protection to spend, Remove's extra hexes,
  Aegis): scored by the eval like every loss prompt. Agents decline a protection 0.4 times a game (2.4 spent); one
  decline in 8 recorded games, against 9 protections spent.
- **Buy energy rule** (`energy_rule`, on): the search scores a pass on the table before income, so it never bought
  (0.02 a game, r74's production prompt bought 2.5). The rule buys when the seat has no card to play and its credits
  plus income, less the price, reach the eval's credit cap: 2.5-2.9 a game, as r74.
- **Botnet reach** (`w.botnet_reach` 0.3, on): an ambassador on a botnet's hex is worth 0.3 of a botnet; take-downs
  0.3 → 1.3-1.5 a game.
- **Liberate** in the eval (the bill and the held count) and the gather heuristic count the seat's own ambassadors
  only; `spare()` keeps an ambassador standing on any site it would pay toward. The threat forecast's armor
  (`guide.rs`) no longer lets the site owner's ambassador guard another's node. The endgame probe counts Remove
  agents across hexes.

On and off, strategist and greedy, 16 seeds a pair (screens.md): both heuristics flat within SE (off − on: energy
−0.8 ± 2.9 greedy, −2.9 ± 3.3 strategist; botnet reach +3.8 ± 2.7, −1.7 ± 2.4). Kept on: each makes the sim players
use a ruled mechanic at all, which is what a ladder against r74 needs.

## The fresh ladder (r75 against r74, the same seeds)

[fresh-ladder.md](fresh-ladder.md): strategist 432 games a level, greedy 144, d0-d4, paired.

| d | strategist r74 → r75 | gap ± SE | greedy r74 → r75 | gap ± SE | r75 strategist losses: round cap / Discord deck / last node |
|---|---|---|---|---|---|
| 0 | 94.0 → 90.3 | −3.7 ± 1.8 | 94.4 → 91.7 | −2.8 ± 3.1 | 100 / 0 / 0 % |
| 1 | 69.4 → 77.3 | +7.9 ± 2.8 | 81.9 → 75.0 | −6.9 ± 4.4 | 70 / 30 / 0 % |
| 2 | 59.5 → 63.4 | +3.9 ± 2.6 | 59.7 → 70.1 | +10.4 ± 4.1 | 8 / 90 / 2 % |
| 3 | 41.7 → 50.0 | +8.3 ± 2.7 | 40.3 → 54.2 | +13.9 ± 4.6 | 28 / 68 / 4 % |
| 4 | 36.1 → 44.2 | +8.1 ± 2.5 | 46.5 → 51.4 | +4.9 ± 4.9 | 17 / 80 / 2 % |

Nothing was tuned toward the bands (177's job); by them the strategist meets d0-d3 and d4 now reads below d3 again.
Nine rulings moved at once, so no gap is attributed to one. What moved, a game: games end 0.5-0.8 rounds sooner;
losses shift from the round cap to the Discord deck (d3 30 → 68 %, d4 53 → 80 %), the shape `carry` 2 predicts;
adversary factories built 3.5 → 2.3, neutral 0.3 → 2.4 (growth next to the city); Remove 5.4 → 2.5; corruption node
losses −0.4 to −0.6; compute −4 to −13 %. Speed: 1.70 ms a decision against 1.69 (strategist), 1.80 against 1.75
(greedy), both sides at 6 threads on a quiet box.

## Checks

Fast lane and `cargo test-all` green; clippy clean; `aic-sim docs --check` current (`regen.py`); `npm run cards:fit`
passes; `loop_smoke.py --run`: no timeouts, no card at the play flag; `speed.py check`: no verdict across revisions
(the cost is above); no engine change after the frozen binary, so no `same_results` replay. The slow-lane
`pending_choices_replay_on_the_guess_three_countries` met two prompts answered on a guessed table: r75's tip prompt
comes before the tip's hidden Discord draw (the test now also stops comparing once a Unity or Discord card is drawn),
and, at d4 seed 15, a project's prompt in `adversary.reveal`, where 175's compute step follows the reveal's hidden
draw in the same phase, so a replay on a guess can resolve another project first (the test skips that phase). The
agents never guess a pending table (151), so play is untouched; splitting the compute step into its own phase would
restore the check (flagged). Specs: `specs176.py`,
`run176.sh`. Readers: `use176.py` (mechanic use), `onoff176.py` (paired on/off).

## For new players (r75's player side)

Every meeting of your ambassador and an agent removes one of each. Remove agents spends an ambassador to clear up to 3
agents in your reach. Claim takes a neutral site and Liberate a corrupted one, the same way: your ambassadors already
on it pay first, then your supply, 1 more for each agent there; either way the site is yours. Five credits and your
ambassador on a botnet take it down. A city builds factories only next to itself. When a piece of yours would be
destroyed, you choose which protection to spend, or none; only your own ambassador guards your pieces. Production is
one income step in printed order; buy a neutral city's energy as an action. Unity can go against Discord at once, and
both tracks keep their overflow. One ambassador stops one corp troop. Each converter is once a round, with a city of
its country in reach. **What resisted a short summary:** a liberate card still differs from the Liberate action
(agents stay, no cube without "align"), and at a Discord tip the players choose which base goes without when the
ambassadors can't stop every troop.

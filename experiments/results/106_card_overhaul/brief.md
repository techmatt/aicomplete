# 106 design brief (for the per-tag design helpers)

Repo: `C:\Code\aicomplete-worktrees\aicomplete\106_card_overhaul`. You design ONE tag's section of the player-card deck
for **r42, the card overhaul**. Write nothing in the repo except your two output files (below).

## Matt's direction (2026-11-04, locked)

A dramatic pass that turns the player cards into Terraforming Mars-style engines, built on 105's engine wording.
**Err toward strong engines that look "broken" and win big when played well.** Win rates are not a target. Every
card, price and number is a candidate.

- **Programs are TM's green cards (locked):** one-shot, no slot, stay in play (their tags count). Permanent income
  raises live here, including per-tag ones ("raise your credit income by 1 per Science tag on your cards").
- **Buildings are the blue cards (locked):** ongoing Actions, Effects and triggers, never one-off effects (nothing
  below the art except an extra cost). The exception is a very strong tier-2 building whose cost is having built the
  right buildings under it (`replaces`, "over X"): it may carry income and a one-off. Engine capstones may be
  buildings. Move what makes sense to programs: a building whose job is flat income ("Each round, 4 credits") becomes a
  program ("Raise your credit income by 4"); a building with a one-off below the art ("Raise your credit income by 2",
  "Add 2 Datasets here") loses it or becomes over-X tier 2.
- **Many TM-style triggers,** including tier-2 versions that count every tag on the played card.
- **Big, expensive cards in both tiers** (TM scale: median cost near 10; "cost 40, gain 10 income"). If credits pile
  up, the cards weren't big enough.
- **Insight is part of the engines and stays a constraint;** keeps stay in insight.

## Card rules (from `docs/card-guidelines.md`, which stays the standard: read it)

No `*`, one reading, no arithmetic beyond one simple "per X", no "per hex of a country". No trivial cards, no
0-credit cards (bar a rare reasoned one), no small riders that add no decision. Tier is power. No "up to N a round"
wording. A `needs` is N of one tag or N of any, never a mix. Science is not a sacrifice tag. A building holds income
or tokens, and one Action or Effect. A card beats the plain route it replaces (ruling 13). Text fits at 7.5 pt: a
building's box above the art about 80 characters on two lines; a program or event about 110. A token effect counts
only tokens on its own card; nothing gains tokens by itself each round (a trigger adding a token on an event, like
Telemetry Archive, is fine: 105). Only Science cards clear corruption markers, shield nodes or delay threats
(content check). Token types: Datasets (Science), Capital (Industry), Followers (Diplomacy), Agents (Security);
Cyber and Power have none.

## What you can print

Read `docs/ops.md` (every op), `docs/rules/card-grammar.md` (*The engine wording*: triggers, thresholds, discounts,
copy, reuse), `docs/card-pricing.md`, `configs/agents/values.json` (the rates), and the JSON shape of cards in
`content/cards.json` (fields: `id, name, tier, kind, cost, insight, energy, chips, antimatter, lowers, needs,
replaces, tags, keywords, tokens, effects, passives, ability{cost, tokens, effects, or}`; `crates/aic-core/src/content.rs`
`CardDef`, `Ability`, `check_card`). Triggers: `{"op":"when","on":{"event":"play","tag":"science"},"then":[...]}`;
events `play` (tag optional), `play_tag` (per tag on the played card), `team_play` (tag optional, `both`),
`play_cost` (`at`), `remove_troop`, `remove_compute`, `remove_ambassadors`, `place_tokens` (`kind`). `threshold`
`{at, then}`. `card_discount {tag?, resource: credits|insight, amount}`. `copy_income`, `reuse`.

**Three extensions being added in this item** (you may print them; the effect ledger can't price them yet, so price
them by hand at the rates):

- `{"op":"production_per_tag","resource":"credits","tag":"science","per":1}` (a program's one-off): "Raise your
  credit income by 1 per Science tag on your cards" (counted once, at play; the program counts itself).
- `income_per_tag` gains an optional `amount` (default 1): `{"op":"income_per_tag","resource":"credits","tag":"science","per":1,"amount":2}`
  is "Each round, 2 credits per Science tag on your cards" (a building; recounted each round).
- `play_tag` with no `tag`: `{"event":"play_tag"}` fires once per tag of any kind on the played card ("when you play a
  card, gain 1 credit per tag on it"): the tier-2 trigger form.

## Your section's targets

The deck goes from 159 to about 150 by **replacing, not adding**: cut trivial cards and weak cards that duplicate
another's role; rework the rest; add new engine cards in the freed places. Your tag (primary tag = first printed tag)
ends with **24-26 cards**: about **18 tier 1 and 7 tier 2**; kinds about **15 programs, 6 buildings, 4 events**
(the deck targets programs ~60%, buildings ~25%, events ~15%). Median printed credit cost near **10**, with some big
cards (20-45 credits) in both tiers. Keep the seven liberate capstones (a core mechanic: you may reprice or retune one)
and the typed counters (two programs and an Action building per threat type, Security/Diplomacy/Cyber/Industry) unless
one is trivial, in which case rework it rather than cut. Keep a card's `id` when you rework it (records key on it); a
new card gets a new snake_case id that names it (prefix nothing). A display rename keeps the id.

## The agreed ideas: your tag must carry the ones assigned to it (each on at least one card)

{IDEAS}

## Discipline for every new or changed card

- **Value check at creation:** what you pay (credits + insight to play × 2.5 + the keep, 2.5 tier 1 / 6.5 tier 2,
  + other costs at `values.json`) against what you get at `values.json`; when in an 8-10 round game it matters, and for
  which board (Builder: Industry, 5 slots; Protector: Security, 4 slots, energy income; Researcher: Science, 3 slots,
  carries insight). Income counts for its horizon (4 rounds tier 1, 2.5 tier 2; energy income × 0.8). **For
  engines, price at the fire rate a seat built around the tag would see** (say 1 card a round of its tag, 2-3 tags of
  it a round on a tag-heavy seat), not the table's generic rate (1.5 plays, 30% tagged). Err strong: a card that nets
  well above 0 for its seat is the goal; one that nets below 0 is a bug.
- **Loops:** trace every loop between resources and pieces a card adds and name its cap: once a round, slots, supply,
  or `Content::check` (a place-token trigger whose ops place tokens is refused). A cap is never "it's break-even".
- Run `python scripts/cards/effect_ledger.py --stdout --cards <your file>` if useful (it reads a list of cards; it
  will not understand the three extension ops: leave those cards out of a ledger run or price them by hand).

## Output (two files, in `experiments/results/106_card_overhaul/proposals/`)

1. `<tag>.json`: a JSON array of the full card objects of your final section (kept, changed and new; not the cut),
   in `content/cards.json`'s shape, `indent=1`, UTF-8. Tags, keywords (`tokens`, `hack`, `help` as the current cards
   use them) and `tokens` fields as the current cards do.
2. `<tag>.md`: one table row per card of the current section and per new card: `id | status (kept / changed / new /
   cut) | one line of reasoning | value check (pay vs get, when, for whom) | loop and cap (or none)`. For a cut card
   say which card covers its role.

Your current section is `out/106/current_<tag>.json` (the cards whose first tag is yours today). Write with the
Write tool (files with `encoding='utf-8', newline='\n'` if by script). Return a 5-line summary: counts by
tier/kind, the new engine cards, the cuts.

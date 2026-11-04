# Player cards (module `cards`, variant `draft`; [`content/cards.json`](../../content/cards.json))

*The opening, the draft, keeps, play, slots, tags, tokens and the liberate capstones. Terms and conventions are in [README.md](README.md).*

The cards config lists which cards are in the game (baseline: all of them).

- **Decks:** a tier-1 and a tier-2 deck of the listed cards (one copy of each), shuffled at setup; discards reshuffle
  into a deck when it runs out. When both decks and their discards are empty (every card is in a hand or
  in play), the draft hands are what has been dealt.
- **The opening (round 1).** Each player is dealt `opening_deal` tier-1 cards and keeps `opening_free` free; each
  further card kept costs `opening_keep_credits` credits. No draft in round 1 (as Terraforming Mars'
  first generation).
- **The draft (rounds 2+).** `cards.tier`: each player builds a hand of `deal` cards **card by card**, choosing for
  each card which deck it comes from; it is dealt face up into the hand, so the next choice sees it. `cards.draft`:
  take one card, pass the rest to the next seat, until the hands are gone: to seat *s*+1 in odd rounds, *s*−1 in even
  rounds (as in Terraforming Mars); nobody may pass in `cards.tier` or `cards.draft`. `cards.keep`: keep any drafted cards for insight, `keep_tier1` for a tier-1 card
  and `keep_tier2` for a tier-2 card (`keep_tier2_upgraded` with the board's keep upgrade); any insight short is paid
  at `insight_credits` credits an insight (keeps only). An Open Market free keep is used first. The rest are
  discarded when the action phase begins.
- **Playing** a card costs its credits (buildings minus your discounts: Industry tags, Master Plan), and any
  `insight`, `energy`, `chips`, `antimatter` and research cubes it prints, and lowers any income it prints under
  `lowers` ("lower your credit income 1": your production of that resource falls for the rest of the game; it can't
  be played if that income is lower). **Insight to play:** 0 for small programs and events, 1 by default (removal
  events included), 2 for big effects and most tier 2, 34 of 49 (the draft set: 27 / 96 / 36 cards at 0 / 1 / 2); it
  comes from the same insight as keeps, and the credit conversion is for keeps only. Its `needs` (tags on your cards
  in play, your board's printed tags and those its tracks revealed, [ai-boards.md](ai-boards.md)) are checked only when it is played. Each
  card's price is in [cards.md](../cards.md); the reasoning is in [card-pricing.md](../card-pricing.md).
- **Science owns a niche (candidate).** Among cards, only Science cards clear corruption markers, put a hardening
  token on a hex with your node (a shield, [ai-boards.md](ai-boards.md)) or make a threat wait a round (a delay: it skips this round's
  execution, stays up, and can be countered next round). The content check refuses any of the three on a card without
  the tag or on a power, save the Protector's Recovery, which clears a marker. Six tier-1 and two tier-2 cards do it.
  No card `needs` Science for it: a team without the tag still has the Clear standard action, its counters and the
  board.
- **Typed counters (candidate).** Twelve tier-1 cards fight one source type, and count for more than they cost toward
  threats of it that the player may counter: Security for military, Diplomacy for influence, Cyber for cyber, Industry
  for economic (Science's and Power's tools stay general). Each type has two programs and an Action building.
- **Tier-2 capstones (candidate).** Each tag ends in at least one standing tier-2 building: income per tag, a drain on
  every adversary project each upkeep, or an automatic counter. No capstone prints a shield or a delay. Reasoning and
  prices: [card-pricing.md](../card-pricing.md).
- **The liberate capstones (candidate; Matt).** Liberating a corrupted city is a core mechanic, and its main route is
  a family of tier-2 cards, one an engine, each paid mostly in its engine's resource and each liberating one corrupted
  city in your reach ([ambassadors.md](ambassadors.md)), some with an extra, a twist from its own engine (prices in [cards.md](../cards.md)):

  | Card | Engine | Kind | Extra |
  |---|---|---|---|
  | The Nuclear Option | antimatter | event | sway 2 toward Discord |
  | Power to the People | power + chips | building | income: 1 credit per Power tag on your cards |
  | Hostile Buyout | Diplomacy, credits | event | your cube goes on the city, and you gain 1 ambassador |
  | Forensic Audit | research cubes | event | none |
  | Root Access | Cyber (needs Cyber tags) | event | remove a botnet on or next to the city |
  | Peacekeeping Mission | Security (needs Security tags) | event | remove up to 3 adversary troops in the city's country |
  | Popular Uprising | Followers | building (arrives with 6 Followers) | Action: spend 6 Followers here: liberate, then discard it (and the stack under it) |

  Each is priced at its own value at the rates (`configs/agents/values.json`: a liberation about 45 credits, near what
  a corrupted city costs in play; the extra at its own row), about 25-47 credits, well under the fallback's ~100 and
  out of reach before mid-game. Great Liberation (Diplomacy, three cleanses in one country, cities included)
  liberates too. **Part of the draft is guessing who will have which engine**, and passing each liberate card to the
  seat that can use it. Loops: each card is one-shot and tier 2, and every route needs nodes next to the city, so
  liberation → clean city → energy → more liberation is capped; Popular Uprising liberates once too and leaves play when it
  does.
- **Tag discounts (candidate; the Terraforming Mars pattern).** Every `science_every` Science tags you have take 1
  insight off your tier-2 keeps and your card plays; every `industry_every` Industry tags take 1 credit off your
  buildings, as steel does. Neither goes below `discount_floor`, the cap on the loop "tags, cheaper costs, more tags";
  a cost printed under the floor stays. The floor holds for every building discount.
- **Tags:** Science, Industry, Cyber, Power, Diplomacy, Security; 1-3 a card. Your tags are those on every card you
  have in play: active buildings, the cards splayed under them, and programs. Events' tags don't stay.
- **Programs** resolve and stay in your play area (their tags count; powers can sacrifice them). **Events** resolve
  and are discarded. A program or event (or ability) is only offered when each of its targeted effects has a target (a
  Harden, a move or a gain per tag counts only on a card that does nothing else; an event paying per tag needs one of
  those tags on your cards). A removal's target is a hex with a piece the card names: "remove 1 adversary ambassador"
  can't name a hex holding only a troop. A building's when-built effect is not checked: with no target it does
  nothing.
- **Buildings** go into a slot: your AI board's `ai.slots` (a building printing `slots` adds them and sits in one
  itself; no card prints it). With a slot free you choose a new slot or **build over** one of your buildings; with
  none free you must build over one. The building underneath is **splayed down**: its passives and ability stop, its
  tags still count, and it is sacrificed with the stack. A building with `replaces` (most tier-2 buildings, over their
  own tag) must be built over one whose top card carries that tag.
- **Tokens** live on buildings that print a type (`tokens`): Datasets (Science), Followers (Diplomacy), Capital
  (Industry), Agents (Security). **A token card is progressive and counts only its own tokens (Matt's ruling 12).** No
  card gains tokens by itself. A token
  card's Action is "add N here, OR spend or count what is here"; a few tier-2 ones arrive loaded ("Add 2 Agents here"
  when built); a covered one holds its tokens and its Action stops; Actions may cost tokens from their own card;
  feeder cards (Dataset Scraper, Capital Injection, Vendor Financing, Manifesto Leak, Recruit Operatives) add tokens to
  one of your cards of a type; Sabotage and Purge Protocols make each player lose tokens. Sacrificing or discarding a
  card loses its tokens.
- **A one-shot Action (`once`):** an Action, or a half of one, may take its card out of play once it resolves, with
  the stack under it (Popular Uprising: spend its 6 Followers, liberate, then discard it).
- **Actions** (abilities: `ability`, buildings only) are used with `Use` once a round, pay their cost, and refresh at
  the start of a round. **An OR Action (`or`)** has two halves: one use a round, the player picks a half it can pay
  and that has a target (a half that counts tokens here needs one there). **Ongoing abilities live only on
  buildings:** income, Effect or Action on a program or event is refused when content loads.

Effects needing choices ask the acting player: hexes, countries, players, threats, amounts ("up to N" offers 0..N), or
"stop".

**Token cards, for new players.** A building that holds Agents, Datasets, Followers or Capital grows only when you
choose: its Action is "add 1 Agent here, OR get 1 credit per Agent here" (or pay some of them for an effect). Each
round you either grow the pile or cash in, and a few cards add tokens to one of your piles. A card only ever counts
the tokens on itself, so there is nothing to total across your cards.

(Variant `off`: no draft, [README.md](README.md#off-variants).)

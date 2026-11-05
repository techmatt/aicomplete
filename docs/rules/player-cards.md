# Player cards (module `cards`, variant `draft`; [`content/cards.json`](../../content/cards.json))

*The opening, the draft, keeps, play, slots, tags, tokens and the liberate capstones. Terms and conventions are in [README.md](README.md).*

The cards config lists which cards are in the game (baseline: all of them).

- **Decks:** a tier-1 and a tier-2 deck of the listed cards (one copy of each), shuffled at setup; discards reshuffle
  into a deck when the draft finds it empty, never for a draw (r44: a draw from an empty deck gets nothing). When both decks and their discards are empty (every card is in a hand or
  in play), the draft hands are what has been dealt.
- **The opening (round 1).** Each player is dealt `opening_deal` tier-1 cards and keeps `opening_free` free; each
  further tier-1 card kept costs `opening_keep_credits` credits. Each is also dealt `opening_deal_tier2` tier-2 cards
  (108, Matt), each kept for `opening_keep_tier2` insight from round 1's insight and taking none of the free keeps:
  a card to plan toward, played later once its `needs` and costs are met. No draft in round 1 (as Terraforming Mars'
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
  events included), 2 for big effects and most tier 2, 30 of 45 (the draft set: 18 / 105 / 31 cards at 0 / 1 / 2); it
  comes from the same insight as keeps, and the credit conversion is for keeps only. Its `needs` (tags on your cards
  in play, your board's printed tags and those its tracks revealed, [ai-boards.md](ai-boards.md)) are checked only when it is played. Each
  card's price is in [cards.md](../cards.md); the reasoning is in [card-pricing.md](../card-pricing.md).
- **Science owns a niche (candidate).** Among cards, only Science cards clear corruption markers, put a hardening
  token on a hex with your node (a shield, [ai-powers.md](ai-powers.md)) or make a threat wait a round (a delay: it skips this round's
  execution, stays up, and can be countered next round). The content check refuses any of the three on a card without
  the tag or on a power, save the Protector's Recovery, which clears a marker. Five tier-1 cards and one tier-2 card do it.
  No card `needs` Science for it: a team without the tag still has the Clear standard action, its counters and the
  board.
- **Typed counters (candidate).** Ten tier-1 cards fight one source type, and count for more than they cost toward
  threats of it that the player may counter: Security for military, Diplomacy for influence, Cyber for cyber, Industry
  for economic (Science's and Power's tools stay general). Diplomacy and Industry have two programs and an Action
  building, Cyber and Security one of each (106 made Security's third a troop removal).
- **Engines (candidate; 106, the Terraforming Mars pattern).** Programs are the green cards: one-shot, and where
  permanent income lives, flat ("raise your credit income by 8") or per tag ("by 1 per Science tag on your cards",
  one such program per tag, needs 2 of it). Buildings are the blue cards: triggers, Actions and Effects; a building's
  income is per tag, 3 a tag (Foundation Model Campus, Vertical Monopoly), or flat on a strong tier-2 one. **An engine
  pays up front (108, Matt):** a building with a trigger, threshold, discount or income also gives a one-time gain
  when built, worth about two of its fires (Terraforming Mars' production plus a matching stock): draw 2-3 cards (4 on
  Foundation Model Campus, Vertical Monopoly, Capital Projects Office and Orbital Embassy), 2 energy on the Power ones, or its first tokens here
  (Telemetry Archive, Retained Earnings, Open Assembly, Intrusion Lab, Conglomerate HQ), so its own play fills the
  first threshold. A committed seat earns an engine back within 2 rounds. Each tag has a tier-1 play
  trigger (when you play a card of the tag) and a tier-2 one that counts every tag on the played card, and its
  engines' payoffs: fight triggers, teammate triggers, discounts, rebates, thresholds, copy and reuse
  ([card-grammar.md](card-grammar.md), *The engine wording*). Cyber keeps a standing automatic counter
  (Autonomous Defense Grid); the others' were cut (Industry's at r44). No capstone prints a shield or a delay. Reasoning and
  prices: [card-pricing.md](../card-pricing.md).
- **The liberate capstones (candidate; Matt).** Liberating a corrupted city is a core mechanic, and its main route is
  a family of tier-2 cards, one an engine, each paid mostly in its engine's resource and each liberating one corrupted
  city in your reach ([ambassadors.md](ambassadors.md)), some with an extra, a twist from its own engine (prices in [cards.md](../cards.md)):

  | Card | Engine | Kind | Extra |
  |---|---|---|---|
  | The Nuclear Option | antimatter | event | sway 1 toward Discord |
  | Power to the People | energy | building (over Power, needs 2) | income: 1 credit per Power tag on your cards |
  | Hostile Buyout | Diplomacy, credits | event | your cube goes on the city, and you gain 3 ambassadors |
  | Forensic Audit | research cubes | event | draw 2 cards |
  | Root Access | Cyber (needs Cyber tags) | event | remove a botnet on or next to the city |
  | Peacekeeping Mission | Security (needs Security tags) | event | remove up to 2 adversary troops in the city's country |
  | Popular Uprising | Followers | building (over Diplomacy, arrives with 4 Followers) | Action: add 2 Followers here, or spend 6 to liberate, again and again |

  Each is priced at its own value at the rates (`configs/agents/values.json`: a liberation about 45 credits, near what
  a corrupted city costs in play; the extra at its own row), 18-42 credits, well under the fallback's ~100 and
  out of reach before mid-game. They read 1.0-1.35 at Matt's unit anchors, inside the margin like any fair card: the
  deck's late credit sinks (r45, pending a liberation anchor). Great Liberation (Diplomacy, three cleanses in one country, cities included)
  liberates too. **Part of the draft is guessing who will have which engine**, and passing each liberate card to the
  seat that can use it. Loops: each card is one-shot and tier 2, and every route needs nodes next to the city, so
  liberation → clean city → energy → more liberation is capped; Popular Uprising (106) liberates every third round at
  most, capped by its once-a-round Action, 6 Followers a liberation and corrupted cities in reach.
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
  own tag: Orbital Embassy, Agency Directorate, Power to the People, Popular Uprising, each a capstone with a big
  one-off) must be built over one whose top card carries that tag. 108 took it off the engines (Foundation Model
  Campus, Orbital Mass Driver, Capital Projects Office, Hyperscale Botnet, Positron Storage Ring): an engine goes in a free slot or over whichever building its owner chooses, so it never buries a
  better building of its tag.
- **Tokens** live on buildings that print a type (`tokens`): Datasets (Science), Followers (Diplomacy), Capital
  (Industry), Agents (Security). **A token card is progressive and counts only its own tokens (Matt's ruling 12).** No
  card gains tokens by itself. A token
  card's Action is "add N here, OR spend or count what is here"; a few tier-2 ones arrive loaded ("Add 2 Agents here"
  when built); a covered one holds its tokens and its Action stops; Actions may cost tokens from their own card;
  feeder cards (Dataset Scraper, Capital Injection, Vendor Financing, Payroll Automation, Manifesto Leak, Recruit Operatives) add tokens to
  one of your cards of a type; Sabotage and Purge Protocols make each player lose tokens. Sacrificing or discarding a
  card loses its tokens.
- **A one-shot Action (`once`):** an Action, or a half of one, may take its card out of play once it resolves, with
  the stack under it (Popular Uprising: spend its 6 Followers, liberate, then discard it and what it's built over).
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

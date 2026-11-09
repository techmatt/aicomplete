# Player cards (module `cards`, variant `draft`; [`content/cards.json`](../../content/cards.json))

*The opening, the draft, keeps, play, slots, tags, tokens and the liberate capstones. Terms and conventions are in [README.md](README.md).*

The cards config lists which cards are in the game (baseline: all of them).

- **Decks and zones** (r48, Matt; as in Terraforming Mars): a tier-1 and a tier-2 deck of the listed cards (one copy
  of each), shuffled at setup. The **discard** holds sacrificed cards, drafted cards not kept and cards discarded from a
  hand. A played event goes to its player's **played pile**, never the discard. If you must draw and can't, reshuffle
  the discard into the deck and draw; the draft does the same. When both decks and their discards are empty (every card
  is in a hand or in play), the draft hands are what has been dealt.
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
  events included), 2 for big effects and most tier 2 (counts below); it
  comes from the same insight as keeps, and the credit conversion is for keeps only. Its `needs` (your tags, *Tags* below) are checked only when it is played. Each
  card's price is in [cards.md](../cards.md); the reasoning is in [card-pricing.md](../card-pricing.md).

<!-- generated: draft_set -->
**The draft set in numbers.** 168 cards (113 tier 1, 55 tier 2). Insight to play: 25 / 112 / 31 cards at 0 / 1 / 2, 31 of the 55 tier-2 cards at 2. An energy cost: 14 cards, 8 of the 63 buildings.
<!-- /generated -->

- **Energy costs (candidate; Matt, r50).** Energy is a second cost on a card whose title reads power-hungry (most are buildings), its
  credits cut to match (about 4.5 an energy): mostly the Builder's heavy machinery, Industry and Power (Fabrication
  Megaplex 2, Lights-Out Assembly 1, Orbital Mass Driver 3, Positron Storage Ring 2, Power to the People 4), and AI
  compute (Foundation Model Campus 2). Energy costs stay uncommon (counts above), so a
  seat that makes little energy still has plenty to play. Seven late sinks (five 2-energy Actions, two tier-2
  buildings with a big energy cost; [cards.md](../cards.md)) serve a seat whose node tracks are full.
- **The end-game cards (r69, drafts; Matt, 2026-11-09).** Ten tier-2 cards for the last 3-4 rounds act where the
  adversary's ambassadors stand, in reach: **convert** them onto your node (Mass Defection, Double Agents, Open Source
  Uprising, Embassy of Exiles), **gather** them (and troops) onto a corrupted city and strike there (Kettle Operation, Lure
  Network), **firewall** a hex for a round (Firewall Array, Air Gap), or mass your own (Rally the Allies), and strike
  (Surgical Strike). The seven events also sway 1-2 Unity; three print a tag threshold ("with 3 Diplomacy tags,
  also …"). None frees a city, none is
  global, none makes a liberation cheaper the next time ([ambassadors.md](ambassadors.md)).
- **Science owns a niche (candidate).** Among cards, only Science cards clear corruption markers, put a hardening
  token on a hex with your node (a shield, [ai-powers.md](ai-powers.md)) or make a threat wait a round (a delay: it skips this round's
  execution, stays up, and can be countered next round). The content check refuses any of the three on a card without
  the tag or on a power, save the Protector's Recovery, which clears a marker. Four tier-1 cards and one tier-2 card do it.
  No card `needs` Science for it: a team without the tag still has the Clear standard action, its counters and the
  board.
- **Typed counters (candidate).** Nine tier-1 cards fight one threat tag, and count for more than they cost toward
  threats carrying it (or Universal) that the player is near: Security for military, Diplomacy for influence, Cyber for cyber, Industry
  for economic (Science's and Power's tools stay general). Industry has two programs and an Action building;
  Security, Diplomacy and Cyber one of each.
- **Engines (candidate; 106, the Terraforming Mars pattern).** Programs are the green cards: one-shot, and where
  permanent income lives, flat ("raise your credit income by 8") or per tag ("by 1 per 2 Science tags on your cards",
  one such program per tag, needs 2 of it). Buildings are the blue cards: triggers, Actions and Effects; a building's
  income is per tag, 3 a tag (Foundation Model Campus, Vertical Monopoly), or flat on a strong tier-2 one. **An engine
  pays up front (108, Matt):** a building with a trigger, threshold, discount or income also gives a one-time gain
  when built, worth about two of its fires (Terraforming Mars' production plus a matching stock): draw 2-3 cards (4 on
  Foundation Model Campus, Vertical Monopoly, Capital Projects Office and Orbital Embassy), 2 energy on three Power ones (not Positron Storage Ring), or its first tokens here
  (Telemetry Archive, Retained Earnings, Open Assembly, Conglomerate HQ, Agency Directorate), so its own play fills the
  first threshold; Intrusion Lab's 1 Agent fills its "at 2" on the first removal of compute,
  or a feeder's Agent (118: At N pays whatever put the tokens there). A committed seat earns an engine back within 2 rounds. Each tag has a tier-1 play
  trigger (when you play a card of the tag) and a tier-2 one that counts every tag on the played card, and its
  engines' payoffs: fight triggers, teammate triggers, discounts, rebates, thresholds, copy and reuse
  ([card-grammar.md](card-grammar.md), *The engine wording*). Cyber keeps a standing automatic counter
  (Autonomous Defense Grid: Cyber threats anywhere need 6 less to counter, r62); the others' were cut (Industry's at r44). No capstone prints a shield or a delay. Reasoning and
  prices: [card-pricing.md](../card-pricing.md).
- **The liberate capstones (candidate; Matt).** Liberating a corrupted city is a core mechanic, and its main route is
  a family of tier-2 cards, one an engine, each paid mostly in its engine's resource and each liberating one corrupted
  city in your reach ([ambassadors.md](ambassadors.md)), some with an extra, a twist from its own engine (prices in [cards.md](../cards.md)):

  | Card | Engine | Kind | Extra |
  |---|---|---|---|
  | The Nuclear Option | antimatter | event | sway 1 toward Discord |
  | Power to the People | energy | building (over Power, needs 2) | income: 1 credit per Power tag on your cards |
  | Citywide Microgrid | energy (8, a late sink) | building | Action: 2 Sway toward Unity |
  | Hostile Buyout | Diplomacy, credits | event | your cube goes on the city, and you gain 3 ambassadors |
  | Forensic Audit | research cubes | event | draw 2 cards |
  | Root Access | Cyber (needs Cyber tags) | event | remove a botnet on or next to the city |
  | Peacekeeping Mission | Security (needs Security tags) | event | remove up to 2 adversary troops in the city's country |
  | Popular Uprising | Followers | tier-1 building | Action: add 1 Follower here, OR pay 4 Followers to liberate, again and again |

  Each is priced at its own value at the rates (`configs/agents/values.json`: a liberation about 45 credits, near what
  a corrupted city costs in play; the extra at its own row), 4-42 credits, well under the Liberate action's 5 ambassadors (~100 credits; 3 in stage III, ~60) and
  out of reach before mid-game. They read 1.0-1.35 at Matt's unit anchors, inside the margin like any fair card: the
  deck's late credit sinks (r45, pending a liberation anchor). Great Liberation (Diplomacy, three cleanses in one country, cities included)
  liberates too. **Part of the draft is guessing who will have which engine**, and passing each liberate card to the
  seat that can use it. Loops: each card is one-shot and tier 2, and every route needs nodes next to the city, so
  liberation → clean city → energy → more liberation is capped; Popular Uprising repeats, capped by its once-a-round
  Action and 4 Followers a liberation (about one every fifth round; every 2-3 with Simulation Cluster's Reuse).
- **Tag discounts (candidate; the Terraforming Mars pattern).** Every `science_every` Science tags you have take 1
  insight off your tier-2 keeps and your card plays; every `industry_every` Industry tags take 1 credit off your
  buildings, as steel does. Neither goes below `discount_floor`, the cap on the loop "tags, cheaper costs, more tags";
  a cost printed under the floor stays. The floor holds for every building discount.
- **Tags:** Science, Industry, Cyber, Power, Diplomacy, Security; 1-3 a card. Your tags are those on every card you
  have in play (active buildings, the cards splayed under them, programs) and your board's, printed or revealed
  ([ai-boards.md](ai-boards.md)), which can't be sacrificed. Events' tags don't stay.
- **Programs** resolve and stay in your play area (their tags count; powers can sacrifice them). **Events** resolve
  and go to your played pile (their tags don't count). **Nothing to act on never blocks a card** (r48, Matt): a part
  of a card with nothing to act on (a removal with no target in reach, a move, a gain per tag with too few tags, a draw
  with no card left) does nothing, and the rest resolves. A program, event or ability is offered when at least one of
  its parts acts; one that would do nothing at all is not. A removal's target is a hex with a piece the card names:
  "remove 1 adversary ambassador" can't name a hex holding only a troop. A building's when-built effect is not checked.
- **Own tags:** a card's own tags never count toward its own `needs` (r48, Matt, as in Terraforming Mars): it is still
  in your hand. Once played, a program's tags count for its own gain per tag (it enters play before it resolves); an
  event's never do.
- **Buildings** go into a slot: your AI board's `ai.slots` (a building printing `slots` adds them and sits in one
  itself; no card prints it). With a slot free you choose a new slot or **build over** one of your buildings; with
  none free you must build over one. The building underneath is **splayed down**: its passives and ability stop, its
  tags still count, and it is sacrificed with the stack. A building with `replaces` (most tier-2 buildings, over their
  own tag: Orbital Embassy, Agency Directorate, Power to the People, each a capstone with a big
  one-off) must be built over one whose top card carries that tag. 108 took it off the engines (Foundation Model
  Campus, Orbital Mass Driver, Capital Projects Office, Hyperscale Botnet, Positron Storage Ring): an engine goes in a free slot or over whichever building its owner chooses, so it never buries a
  better building of its tag.
- **Tokens** live on buildings that print a type (`tokens`): Datasets (Science), Followers (Diplomacy), Capital
  (Industry), Agents (Security). **A token card is progressive and counts only its own tokens (Matt's ruling 12).** No
  card gains tokens except as it prints. Each arrives loaded ("Add 2 Agents here" when built), then grows one of
  two ways: an OR Action that adds N here or spends or counts what is here (Synthetic Data Forge, Safehouse Network,
  Popular Uprising), or an Effect that adds tokens when something happens, with an **At N** that takes N off and pays
  by itself (the other six in the deck: Telemetry Archive, Retained Earnings, Intrusion Lab, Open Assembly, Agency
  Directorate, Conglomerate Headquarters); a covered one holds its tokens and its Action stops; Actions may cost tokens from their own card;
  feeder cards (Dataset Scraper, Capital Injection, Payroll Automation, Manifesto Leak, Recruit Operatives, Deep State
  Ledger) add tokens to one of your cards of a type, and its At N pays if they reach it (118); Sabotage and Purge Protocols make each player lose tokens. Sacrificing or discarding a
  card loses its tokens.
- **Actions** (abilities: `ability`, buildings only) are used with `Use` once a round, pay their cost, and refresh at
  the start of a round. **An OR Action (`or`)** has two halves: one use a round, the player picks a half it can pay
  and that has a target (a half that counts tokens here needs one there). **Ongoing abilities live only on
  buildings:** income, Effect or Action on a program or event is refused when content loads.

Effects needing choices ask the acting player: hexes, countries, players, threats, amounts ("up to N" offers 0..N), or
"stop".

**Token cards, for new players.** A building that holds Agents, Datasets, Followers or Capital starts with a few.
Most then grow by themselves: each time you do what the card names (play a Science card, say), tokens go on it, and
when they reach its number it pays and empties by that many. Three have an Action instead: each round you either add
a token or spend or count them for an effect. A few cards add tokens to one of your piles. A card only ever counts
the tokens on itself, so there is nothing to total across your cards.

(Variant `off`: no draft, [README.md](README.md#off-variants).)

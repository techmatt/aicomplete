# Player cards (module `cards`, variant `draft`; [`content/cards.json`](../../content/cards.json))

*The opening, the draft, keeps, play, slots, tags, tokens and the liberate capstones. Terms and conventions are in [README.md](README.md).*

The cards config lists which cards are in the game (baseline: all of them).

- **Decks and zones** (Matt; as in Terraforming Mars): a tier-1 and a tier-2 deck of the listed cards (one copy
  of each), shuffled at setup. The **discard** holds sacrificed cards, drafted cards not kept and cards discarded from a
  hand. A played event goes to its player's **played pile**, never the discard. If you must draw and can't, reshuffle
  the discard into the deck and draw; the draft does the same. When both decks and their discards are empty (every card
  is in a hand or in play), the draft hands are what has been dealt.
- **The opening (round 1; r82, Matt).** Before round 1's draft, each player is dealt `opening_deal` tier-1 and
  `opening_deal_tier2` tier-2 cards straight into their hand, kept free. The tier-2s seed a build direction, played
  later once their `needs` and costs are met; one may go unused. Round 1 then drafts and keeps as every round does.
- **The draft (every round).** `cards.tier`: each player builds a hand of `deal` cards **card by card**, choosing for
  each card which deck it comes from; it is dealt face up into the hand, so the next choice sees it. `cards.draft`:
  take one card, pass the rest to the next seat, until the hands are gone: always to seat *s*+1, every round;
  nobody may pass in `cards.tier` or `cards.draft`. `cards.keep`: keep any drafted cards for insight, `keep_tier1` for a tier-1 card
  and `keep_tier2` for a tier-2 card (`keep_tier2_upgraded` with the board's keep upgrade), round 1
  included; credits never buy a keep (r82), so a keep you lack the insight for isn't made. An Open Market free keep is used first. The rest are
  discarded when the action phase begins.
- **Playing** a card costs its credits, less your discounts (Industry tags and Master Plan off a building; a tag discount
  such as Darknet Exchange's off any card with that tag), and any
  `insight`, `energy`, `chips`, `antimatter` and research cubes it prints, and lowers any income it prints under
  `lowers` ("lower your credit income 1": your production of that resource falls for the rest of the game; it can't
  be played if that income is lower). **Insight to play** is a price, spread over 0, 1 and 2 (counts below): most
  tier-1 cards play for 0; a card that draws costs at least 1; 2 for the strongest of each tier; it
  comes from the same insight as keeps. Its `needs` (your tags, *Tags* below) are checked only when it is played. Each
  card's price is in [cards.md](../cards.md); the reasoning is in [card-pricing.md](../card-pricing.md).

<!-- generated: draft_set -->
**The draft set in numbers.** 182 cards (118 tier 1, 64 tier 2). Insight to play: 95 / 57 / 30 cards at 0 / 1 / 2, 20 of the 64 tier-2 cards at 2. An energy cost: 14 cards, 8 of the 71 buildings.
<!-- /generated -->

- **Energy costs (candidate; Matt).** Energy is a second cost on a card whose title reads power-hungry (most are buildings), its
  credits cut to match (about 4.5 an energy): mostly the Builder's heavy machinery, Industry and Power, and AI
  compute. Energy costs stay uncommon (counts above), so a
  seat that makes little energy still has plenty to play. Seven late sinks (five 2-energy Actions, two tier-2
  buildings with a big energy cost; [cards.md](../cards.md)) serve a seat whose node tracks are full.
- **Tier 2 is the majors (r94, Matt, candidate).** Spirit Island's major powers: big payoffs, most paid in antimatter
  (15 tier-2 cards, and two dear tier-1 ones), giving lots to funnel into your own or another player's board actions:
  liberations, troop strikes, ambassadors and Unity, energy, chips, cubes. The draft price is their only acquisition
  cost; no tier-2 card clears many land corruption tokens (green cards do).
- **The end-game cards (drafts; Matt, 2026-11-09).** Ten tier-2 cards for the last 3-4 rounds act where the
  agents stand, in reach: **convert** them into your ambassadors where they stand (Mass Defection, Double
  Agents, Open Source Uprising, Embassy of Exiles), **strike** them (Kettle Operation: and troops) on or next to a hex in
  reach (Kettle Operation, Lure Network), **firewall** a hex until the end of the next adversary turn (Firewall Array, Air Gap), mass your own
  (Rally the Allies), or remove them, or troops, on one hex (Surgical Strike). The six events and Open Source Uprising also sway 1-2 Unity; three print a tag threshold ("with 3 Diplomacy tags,
  also …"). None frees a city, none is
  global, none makes a liberation cheaper the next time ([ambassadors.md](ambassadors.md)).
- **Tags are story, not rules (r93, Matt).** No tag owns an effect, as Magic's colors don't: each tag has its usual
  effects (the tag table in [ai-powers.md](ai-powers.md)), and a card whose effect fits none of its tags is a soft
  design warning, never a refusal. Shields on a node hex lean Security, land lifts Diplomacy or Power, delays (a threat
  skips this round's execution, stays up, and can be countered next round) Science. No board power prints a shield or
  a delay; the Protector's Recovery lifts land corruption. No card `needs` a tag for any of the three.
- **Typed counters (candidate).** Nine tier-1 cards fight one threat tag, and count for more than they cost toward
  threats carrying it (or Universal) that the player is near, a threat's tag being the card's own (Security,
  Diplomacy, Cyber, Industry; Science's and Power's tools stay general). Industry has two programs and an Action building;
  Security, Diplomacy and Cyber one of each.
- **Engines (candidate; the Terraforming Mars pattern).** Programs are the green cards: one-shot, and where
  permanent income lives, flat ("raise your credit income by 8") or per tag ("by 1 per 2 Science tags on your cards",
  one such program per tag, needs 2 of it). Buildings are the blue cards: triggers, Actions and Effects; a building's
  income is per tag, 3 a tag (Foundation Model Campus, Vertical Monopoly), counted once when it is played as a
  program's is (r98, `cards.tag_income_once`, Matt's candidate from 224 #6), or flat on a strong tier-2 one. **An engine
  pays up front (Matt):** a building with a trigger, threshold, discount or income also gives a one-time gain
  when built, worth about two of its fires: draw 2-4 cards, 2 energy on three Power ones (not Positron Storage Ring), or its first tokens here
  (the At N cards, *Tokens* below), so its own play fills the
  first threshold; Intrusion Lab's 1 Operative instead fills its "at 2" on the first removal of compute,
  or a feeder's Operative. A committed seat earns an engine back within 2 rounds. Each tag has a tier-1 play
  trigger and a tier-2 one that counts every tag on the played card, and its engines' payoffs
  ([card-grammar.md](card-grammar.md), *The engine wording*). Cyber and Industry each keep a standing
  automatic counter (Autonomous Defense Grid, Market Stabilization Board: Cyber or Industry threats anywhere need 6
  less to counter); no other tag has one. No capstone prints a hardening token or a delay.
- **The liberate capstones (candidate; Matt).** Liberating a corrupted city is a core mechanic, and its main route is
  a family of tier-2 cards, one an engine, each paid mostly in its engine's resource and each liberating one corrupted
  city in your reach as the Liberate action does, unpaid: its agents come off and your cube goes on
  ([ambassadors.md](ambassadors.md)); some add an extra, a twist from its own engine:

  | Card | Engine | Kind | Extra |
  |---|---|---|---|
  | The Nuclear Option | antimatter | event | sway 2 toward Discord (r90: collateral harm) |
  | Power to the People | energy | building (over Power, needs 2) | income: 1 credit per Power tag on your cards |
  | Citywide Microgrid | energy (8, a late sink) | building | Action: 2 Sway toward Unity |
  | Hostile Buyout | antimatter (r94), Diplomacy | event | gain 3 ambassadors |
  | Forensic Audit | research cubes | event | draw 2 cards |
  | Root Access | antimatter (r94), Cyber (needs Cyber tags) | event | remove a botnet on or next to the city |
  | Peacekeeping Mission | antimatter (r94), Security (needs Security tags) | event | remove up to 3 adversary troops in the city's country |
  | Popular Uprising | Followers | tier-1 building | Action: add 1 Follower here, OR pay 4 Followers to liberate, again and again |

  Each is priced at its own value at the rates, 4-42 credits (the three antimatter ones 18-22 and 1 antimatter, r94), well under the Liberate action's 5 ambassadors (~100
  credits) and out of reach before mid-game; they read 1.0-1.35 at Matt's unit anchors: the deck's late credit sinks
  ([card-pricing.md](../card-pricing.md), *The liberate capstones*). Great Liberation (Diplomacy, three cleanses in one country, cities included)
  liberates too. **Part of the draft is guessing who will have which engine**, and passing each liberate card to the
  seat that can use it. Loops: each card is one-shot and tier 2, and every route needs nodes next to the city, so
  liberation → clean city → energy → more liberation is capped; Popular Uprising repeats, capped by its once-a-round
  Action and 4 Followers a liberation (about one every fifth round; every 2-3 with Simulation Cluster's Reuse).
- **Tag discounts (candidate; the Terraforming Mars pattern).** Every `science_every` Science tags you have take 1
  insight off your tier-2 keeps and your card plays; every `industry_every` Industry tags take 1 credit off your
  buildings, as steel does. Neither goes below `discount_floor`, the cap on the loop "tags, cheaper costs, more tags";
  a cost printed under the floor stays.
- **Tags:** Science, Industry, Cyber, Power, Diplomacy, Security; 1-3 a card. Your tags are those on every card you
  have in play (active buildings, the cards splayed under them, programs) and your board's, printed or revealed
  ([ai-boards.md](ai-boards.md)), which can't be sacrificed.
- **Programs** resolve and stay in your play area (their tags count; powers can sacrifice them). **Events** resolve
  and go to your played pile (their tags don't count). **Nothing to act on never blocks a card** (Matt): a part
  of a card with nothing to act on (a removal with no target in reach, a move, a gain per tag with too few tags, a draw
  with no card left) does nothing, and the rest resolves. A program, event or ability is offered when at least one of
  its parts acts; one that would do nothing at all is not. A Discord sway is a price, never a part that acts (184). A removal's target is a hex with a piece the card names:
  "remove 1 agent" can't name a hex holding only a troop. A building's when-built effect is not checked.
- **Own tags:** a card's own tags never count toward its own `needs` (Matt, as in Terraforming Mars): it is still
  in your hand. Once played, a program's or building's tags count for its own gains per tag, which print "(including
  this)"; an event's never do ([card-grammar.md](card-grammar.md)).
- **Buildings** go into a slot: your AI board's `ai.slots` (a building printing `slots` adds them and sits in one
  itself; no card prints it). With a slot free you choose a new slot or **build over** one of your buildings; with
  none free you must build over one. The building underneath is **splayed down**: its passives and ability stop, its
  tags still count, and it is sacrificed with the stack. A building with `replaces` (three tier-2 buildings, each over
  its own tag: Orbital Embassy, Agency Directorate, Power to the People, each a capstone with a big
  one-off) must be built over one whose top card carries that tag. The engines (Foundation Model Campus, Orbital Mass
  Driver, Capital Projects Office, Hyperscale Botnet, Positron Storage Ring) have none: an engine goes in a free slot
  or over whichever building its owner chooses, so it never buries a better building of its tag.
- **Tokens** live on buildings that print a type (`tokens`): Datasets (Science), Followers (Diplomacy), Capital
  (Industry), Operatives (Security; r70, so agents are only the adversary's pieces). **A token card is progressive and counts only its own tokens (Matt's ruling 12).** No
  card gains tokens except as it prints. Each but Popular Uprising (empty) arrives loaded ("Add 2 Operatives here" when built), then grows one of
  two ways: an OR Action that adds N here or spends or counts what is here (Synthetic Data Forge, Safehouse Network,
  Popular Uprising), or an Effect that adds tokens when something happens, with an **At N** that takes N off and pays
  by itself (the other six in the deck: Telemetry Archive, Retained Earnings, Intrusion Lab, Open Assembly, Agency
  Directorate, Conglomerate Headquarters); a covered one holds its tokens and its Action stops; Actions may cost tokens from their own card;
  feeder cards (Dataset Scraper, Capital Injection, Payroll Automation, Manifesto Leak, Recruit Operatives, Deep State
  Ledger) add tokens to one of your cards of a type, and its At N pays if they reach it; Sabotage and Purge Protocols make each player lose tokens. Sacrificing or discarding a
  card loses its tokens.
- **Actions** (abilities: `ability`, buildings only) are used with `Use` once a round, pay their cost, and refresh at
  the start of a round. **An OR Action (`or`)** has two halves: one use a round, the player picks a half it can pay
  and that has a target (a half that counts tokens here needs one there). **Ongoing abilities live only on
  buildings:** income, Effect or Action on a program or event is refused when content loads.

Effects needing choices ask the acting player: hexes, countries, players, threats, amounts ("up to N" offers 0..N), or
"stop".

**Token cards, for new players.** A building that holds Operatives, Datasets, Followers or Capital starts with a few.
Most gain more each time you do what the card names (play a Science card, say), and at its number pay and empty by
that many. Three have an Action instead: each round, add a token or spend or count them. A few cards add tokens to
one of your piles. A card only ever counts the tokens on itself.

(Variant `off`: no draft, [README.md](README.md#off-variants).)

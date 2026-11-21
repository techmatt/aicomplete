# Player cards (module `cards`, variant `draft`; [`content/cards.json`](../../content/cards.json))

*Terms and conventions are in [README.md](README.md).*

The cards config lists the cards in the decks (`configs/baseline.json`; content holds more, "not in deck" in
[cards.md](../cards.md)); each tag is primary on 30 to 34 of them.

- **Decks and zones:** a tier-1 and a tier-2 deck of the listed cards (one copy
  of each), shuffled at setup. The **discard** holds sacrificed cards, drafted cards not kept and cards discarded from a
  hand. A played event goes to its player's **played pile**, never the discard. If you must draw and can't, reshuffle
  the discard into the deck and draw; the draft does the same. With both decks and discards empty, the draft hands are
  what has been dealt.
- **The opening (round 1).** Before round 1's draft, each player is dealt `opening_deal` tier-1 and
  `opening_deal_tier2` tier-2 cards into their hand, kept free. The tier-2s seed a build direction; one may go unused. Round 1 then drafts and keeps as every round does.
- **The draft (every round).** `cards.tier`: each player builds a hand of `deal` cards **card by card**, choosing each
  card's deck; it is dealt face up into the hand, so the next choice sees it. `cards.draft`: take one card, pass the
  rest to seat *s*+1, until the hands are gone; nobody may pass in `cards.tier` or `cards.draft`. `cards.keep`: keep
  any drafted cards for insight, `keep_tier1` a tier-1 card, `keep_tier2` a tier-2 (`keep_tier2_upgraded` with the
  board's keep upgrade), round 1 included; credits never buy a keep, so one you lack the insight for isn't made. An
  Open Market free keep is used first. The rest are discarded when the action phase begins.
- **Playing** a card costs its credits, less your discounts (Industry tags and Master Plan off a building; a tag discount
  such as Darknet Exchange's off any card with that tag), and any `insight`, `energy`, `chips`, `antimatter` and
  research cubes it prints, and lowers any income it prints under `lowers` ("lower your credit income 1": that
  production falls for the rest of the game; it can't be played if that income is lower). **Insight to play** is a
  price of 0, 1 or 2 (counts below): most tier-1 cards 0; a card that draws at least 1; 2 for the strongest of each tier; it comes from the same insight as keeps. Its `needs` (your tags, *Tags* below) are checked only when it is
  played. Prices: [cards.md](../cards.md); reasoning: [card-pricing.md](../card-pricing.md).
- **Card draw (candidate; draw is precious).** A draw takes the tier-1 deck's top card, no keep. There is no limit on
  plays: cards are scarce instead. On a card, draw 1 is a good benefit, 2 very strong, 3 exceptional (Recursive
  Self-Improvement, Zero-Day Auction); Industry, Power and Security keep a few one-time draw 1s. **Repeatable draw ends
  three tag chains:** Science, Preprint Server (needs 4 Science: when you play a Science card, draw 1); Cyber,
  Command-and-Control Grid (needs 3 Cyber: Action: draw 1; 2 a round with Simulation Cluster's Reuse); Diplomacy, Consulate Tower (needs 3 Diplomacy:
  when another player plays a Diplomacy card, draw 1). Board powers and converters draw at most 1
  ([ai-powers.md](ai-powers.md)); no megaproject pays a card at completion.

<!-- generated: draft_set -->
**The draft set in numbers.** 196 cards (124 tier 1, 72 tier 2). Insight to play: 108 / 60 / 28 cards at 0 / 1 / 2, 18 of the 72 tier-2 cards at 2. An energy cost: 13 cards, 7 of the 81 buildings.
<!-- /generated -->

- **Energy costs (candidate).** Energy is a second cost on a card whose title reads power-hungry (mostly buildings:
  heavy machinery, Industry and Power, AI compute), its credits cut to match ([card-pricing.md](../card-pricing.md),
  *Energy as a second cost*). They stay uncommon (counts above), so a seat that makes little energy still has plenty
  to play; seven 2-energy Actions, Fusion Torch Array's 5 and the 4-energy tier-2 buildings are late sinks.
- **Tier 2 is the majors (candidate).** Spirit Island's major powers: big payoffs, 21 of them spending antimatter, to play or by an Action (and
  two tier-1 cards, Accelerator Diplomacy and Containment Reactor), for your own or another player's board actions: liberations, troop
  strikes, ambassadors and Unity, energy, chips, cubes. **Antimatter sinks pay big (locked, Matt 287):** saving for one
  is clearly right, a strong route but not the only one (Kinetic Interdiction's country sweep, an Action, Orbital Mass
  Driver's income, Fusion Commons' Unity). The draft price is their only acquisition cost. Cards clear
  Fallout only as green cards (programs): a few at tier 1, and Open-Weights Release (4) at tier 2.
- **Finale buildings (candidate; Matt, 306).** Repeatable tier-2 buildings, hard to get (`needs` 2-3 of a tag or a
  big price), whose once-a-round Action spends a late-game resource; a plant beside one is the late engine:
  Siegebreaker Reactor (needs 3 Power; 1 antimatter: remove 2 troops on or next to a corrupted city, rescue it),
  Concord Reactor (12 credits, needs 2 Diplomacy; 1 antimatter: 5 ambassadors, 4 Unity), Singularity Exploit (needs
  3 Cyber; 1 antimatter: discard an adversary project, gain 2 insight), Fusion Torch Array (needs 3 Power; 5 energy: rescue
  a corrupted city).
- **The end-game cards (drafts).** Ten tier-2 cards for the last 3-4 rounds act where the agents stand, at the card's
  target: four **convert** them into your ambassadors where they stand, two **strike** them (one troops too) on or next
  to the target, two **firewall** a hex until the end of the next adversary turn, one masses your own, one removes
  them, or troops, on one hex ([cards.md](../cards.md)). None frees a city, none is global, none makes a
  liberation cheaper the next time ([ambassadors.md](ambassadors.md)).
- **Tags are story, not rules:** no tag owns an effect ([ai-powers.md](ai-powers.md), its tag table). Shields on a
  node hex lean Security, land lifts Diplomacy (three Power cards print one too), delays ([card-grammar.md](card-grammar.md), *Jam*)
  Science; no board power prints any of the three, and no card `needs` a tag for one.
- **Typed counters (candidate).** Nine tier-1 cards fight one threat tag, the card's own (none Science or Power), and count for more than they cost toward
  threats carrying it (or Universal) that the player is near.
- **Engines (candidate; the Terraforming Mars pattern).** Programs are the green cards: one-shot, and where permanent
  income lives, flat ("raise your credit income by 5") or per tag ("by 1 per 3 Science tags on your cards", one such
  program per tag, needs 2 of it). Buildings are the blue cards: triggers, Actions and Effects; a building's income is
  per tag at its tag count when played (2 a tag on the engines; Power to the People 1 per 2 Power tags), or flat (the tier-2 engines, Fabrication Megaplex,
  Cryo Sample Vault). **Some engines pay up front:** a one-time gain when built (a drawn card, 2 energy on two Power
  ones, or its first tokens here: the At N cards, *Tokens* below, so its own play, or for Intrusion Lab its first
  removal of compute or a feeder, fills the first threshold). A committed seat earns most engines back within 3
  rounds ([card-pricing.md](../card-pricing.md), *Engines*). Each tag has a tier-1 play trigger, a tier-2 one
  that counts that tag on the played card (no payout fires on any card), and its engines' payoffs ([card-grammar.md](card-grammar.md), *The engine
  wording*). Cyber and Industry each keep one standing automatic counter (a building: threats of its tag anywhere
  need 6 less of each resource); no other tag has one. No capstone prints a
  hardening token or a delay.
- **The liberate capstones (candidate).** Liberating a corrupted city is a core mechanic, its main route a family of
  tier-2 cards, one an engine, each paid mostly in its engine's resource and liberating one corrupted city in your
  reach as the Liberate action does, unpaid and with no ambassador on it ([ambassadors.md](ambassadors.md)). The city is left neutral (254), so
  all but Hostile Buyout then **claim a neutral station or factory next to it** (256, free then claim: no reach
  needed, nothing lost when none stands there); some add a twist from its engine:

  | Card | Engine | Kind | Extra |
  |---|---|---|---|
  | The Nuclear Option | antimatter | event | also frees each corrupted station or factory next to it; +3 energy income |
  | Power to the People | energy | building (over Power, needs 2) | income: 1 credit per 2 Power tags on your cards |
  | Citywide Microgrid | energy (4, a late sink) | building | Action: 3 Sway toward Unity |
  | Hostile Buyout | antimatter, Diplomacy | event | gain 5 ambassadors; any corrupted site, a station or factory becomes yours (254) |
  | Forensic Audit | research cubes | event | rescue the city |
  | Root Access | antimatter, Cyber (needs Cyber tags) | event | remove a botnet on or next to the city; +3 insight income |
  | Peacekeeping Mission | antimatter, Security (needs Security tags) | event | first removes up to 3 troops on or next to the city; rescues only if none is left; then places 3 ambassadors on it and sways 4 toward Unity |
  | Beamline Liberation | antimatter, Science | event | gain 7 research cubes |
  | Reactor Retooling | antimatter, Industry | event | +3 chip income |
  | Popular Uprising | Followers | tier-1 building | Action: add 1 Follower here, OR pay 4 Followers to liberate, again and again |
  | Siegebreaker Reactor | antimatter, Power (needs 3) | building | Action: first removes 2 troops on or next to the city; rescues only if no guard is left |
  | Fusion Torch Array | energy (5), Power (needs 3) | building | Action: rescue the city |

  They cost 0-11 credits, priced against the Liberate action's bill ([card-pricing.md](../card-pricing.md), *The
  liberate capstones*); the six antimatter events (0-3 credits and 1 antimatter) are sinks at 2.0-2.5×, the two
  finale buildings 1.7-1.8 a use (*Finale buildings*), the rest fair. Great Liberation (Diplomacy: its target and 2 more in the target's country, cities included)
  liberates too. **Part of the draft is guessing who will have which engine**, and passing each liberate card to the
  seat that can use it. Loops: each card is one-shot and tier 2, and every route needs nodes next to the city, so
  liberation → clean city → energy → more liberation is capped; Popular Uprising and the two finale buildings repeat,
  capped by their once-a-round Action and its fuel (4 Followers, 1 antimatter, 5 energy).
- **Tag discounts (candidate; the Terraforming Mars pattern).** Every `science_every` Science tags you have take 1
  insight off your tier-2 keeps and your card plays; every `industry_every` Industry tags take 1 credit off your
  buildings, as steel does. Neither goes below `discount_floor` (0: a discounted card may be free); the loop "tags,
  cheaper costs, more tags" is capped by the tags a card needs and the draw, not the floor.
- **Tags:** Science, Industry, Cyber, Power, Diplomacy, Security; 1-3 a card. Yours are those *on your cards*
  ([README.md](README.md#terms)); a board's can't be sacrificed.
- **Programs** resolve and stay in your play area (their tags count; powers can sacrifice them); a lasting raise they
  print stays even when sacrificed (a program's production, say). **Events** resolve
  and go to your played pile (their tags don't count). **Nothing to act on never blocks a card:** a part with nothing
  to act on (a map effect whose target condition no hex in reach meets, a move, a gain per tag with too few tags, a
  draw with no card left) does nothing, and the rest resolves. A program, event or ability is offered only when at
  least one of its parts acts. A Discord sway is a price, never a part that acts. A
  removal's target is a hex with a piece the card names: "remove 1 agent" can't name a hex holding only a troop. A
  building's when-built effect is not checked.
- **Own tags:** a card's own tags never count toward its own `needs`: it is still in your hand. Once played, a program's or building's tags count for its own gains per tag; an event's never do
  ([card-grammar.md](card-grammar.md), *Per tag*).
- **Buildings** go into a slot: your AI board's `ai.slots` (a building printing `slots`, which no card does, adds
  them and fills one). With a slot free you choose a new slot or **build over** one of your buildings; with
  none free you must build over one. The building underneath is **splayed down**: its passives and ability stop, its
  tags still count, and it is sacrificed with the stack. A building with `replaces` (Orbital Embassy, Agency
  Directorate, Power to the People: tier-2 capstones with a big one-off, each over its own tag) must be built over one
  whose top card carries that tag. The engines (Foundation Model Campus, Orbital Mass Driver, Capital Projects Office,
  Hyperscale Botnet, Positron Storage Ring) have none: an engine goes in a free slot or over whichever building its
  owner chooses, so it never buries a better building of its tag.
- **Tokens** live on buildings that print a type (`tokens`): Datasets (Science), Followers (Diplomacy), Capital
  (Industry), Operatives (Security). **A token card is progressive and counts
  only its own tokens (Matt's ruling 12).** No card gains tokens except as it prints. Each but Popular Uprising
  (empty) arrives loaded ("Add 2 Operatives here" when built), then grows one of two ways: an OR Action that adds N
  here or spends or counts what is here (three, Popular Uprising among them), or an Effect that
  adds tokens when something happens, with an **At N** that takes N off and pays by itself (the other six in the
  deck). A covered one holds its tokens and its Action stops; Actions may cost tokens from their own card. Feeder cards
  (five, [cards.md](../cards.md)) add tokens to one of your cards of a type, and its At N pays if they reach it;
  Sabotage and Purge Protocols make each player lose tokens. Sacrificing or discarding a card loses its tokens.
- **Actions** (`ability`, buildings only) are used (`Use`) once a round, paying their cost. **An OR Action (`or`)** has two halves: one use a round, the player picks a half it can pay and that has
  a target (a half that counts tokens here needs one there). **Ongoing abilities live only on buildings:** income,
  Effect or Action on a program or event is refused when content loads.

(Variant `off`: no draft, [README.md](README.md#off-variants).)

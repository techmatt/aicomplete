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
  price of 0, 1 or 2 (counts below): most tier-1 cards 0; a card that draws at least 1; 2 for the strongest of each
  tier; it comes from the same insight as keeps. Its `needs` (your tags, *Tags* below) are checked only when it is
  played. Prices: [cards.md](../cards.md); reasoning: [card-pricing.md](../card-pricing.md).

<!-- generated: draft_set -->
**The draft set in numbers.** 191 cards (124 tier 1, 67 tier 2). Insight to play: 104 / 57 / 30 cards at 0 / 1 / 2, 20 of the 67 tier-2 cards at 2. An energy cost: 14 cards, 8 of the 76 buildings.
<!-- /generated -->

- **Energy costs (candidate).** Energy is a second cost on a card whose title reads power-hungry (mostly buildings:
  heavy machinery, Industry and Power, AI compute), its credits cut to match ([card-pricing.md](../card-pricing.md),
  *Energy as a second cost*). They stay uncommon (counts above), so a seat that makes little energy still has plenty
  to play; seven 2-energy Actions and the energy-hungry tier-2 buildings (Citywide Microgrid and Power to the People
  at 4 energy, Orbital Mass Driver at 3) serve as late sinks for a seat whose node tracks are full.
- **Tier 2 is the majors (candidate).** Spirit Island's major powers: big payoffs, 15 of them paid in antimatter (and
  one tier-1 card, Accelerator Diplomacy), for your own or another player's board actions: liberations, troop
  strikes, ambassadors and Unity, energy, chips, cubes. The draft price is their only acquisition cost. Cards clear
  Fallout only as green cards (programs): a few at tier 1, and Open-Weights Release (4) at tier 2.
- **The end-game cards (drafts).** Ten tier-2 cards for the last 3-4 rounds act where the agents stand, at the card's
  target: four **convert** them into your ambassadors where they stand, two **strike** them (one troops too) on or next
  to the target, two **firewall** a hex until the end of the next adversary turn, one masses your own, one removes
  them, or troops, on one hex ([cards.md](../cards.md)). Mass Defection, Rally the Allies (2) and Open Source
  Uprising (1) also sway Unity; Kettle Operation and Surgical Strike sway 1 Discord. None frees a city, none is global, none makes a
  liberation cheaper the next time ([ambassadors.md](ambassadors.md)).
- **Tags are story, not rules:** no tag owns an effect ([ai-powers.md](ai-powers.md), its tag table). Shields on a
  node hex lean Security, land lifts Diplomacy or Power, delays ([card-grammar.md](card-grammar.md), *Jam*)
  Science; no board power prints any of the three, and no card `needs` a tag for one.
- **Typed counters (candidate).** Nine tier-1 cards fight one threat tag, the card's own (three Industry, two each
  Security, Diplomacy and Cyber; Science's and Power's tools stay general), and count for more than they cost toward
  threats carrying it (or Universal) that the player is near.
- **Engines (candidate; the Terraforming Mars pattern).** Programs are the green cards: one-shot, and where permanent
  income lives, flat ("raise your credit income by 8") or per tag ("by 1 per 2 Science tags on your cards", one such
  program per tag, needs 2 of it). Buildings are the blue cards: triggers, Actions and Effects; a building's income is
  per tag, 3 a tag at its tag count when played (as a program's), or flat (the tier-2 engines, Fabrication Megaplex,
  Cryo Sample Vault). **An engine pays up front:** a building with a trigger, threshold, discount or income (but the
  cheap Cryo Sample Vault) also gives a one-time gain when built, worth about
  two of its fires: draw 2-4 cards, 2 energy on two Power ones, or its first tokens here (the At N cards, *Tokens* below), so its own play (or,
  for Intrusion Lab, its first removal of compute or a feeder) fills the first threshold. A committed seat earns an engine back within 2 rounds. Each tag has a tier-1 play trigger, a tier-2 one
  that counts every tag on the played card, and its engines' payoffs ([card-grammar.md](card-grammar.md), *The engine
  wording*). Cyber and Industry each keep one standing automatic counter (a building: threats of its tag anywhere
  need 6 less of each resource); no other tag has one. No capstone prints a
  hardening token or a delay.
- **The liberate capstones (candidate).** Liberating a corrupted city is a core mechanic, its main route a family of
  tier-2 cards, one an engine, each paid mostly in its engine's resource and liberating one corrupted city in your
  reach as the Liberate action does, unpaid ([ambassadors.md](ambassadors.md)). The city is left neutral (254), so
  all but Hostile Buyout then **claim a neutral station or factory next to it** (256, free then claim: no reach
  needed, nothing lost when none stands there); some add a twist from its engine:

  | Card | Engine | Kind | Extra |
  |---|---|---|---|
  | The Nuclear Option | antimatter | event | sway 2 toward Discord (collateral harm) |
  | Power to the People | energy | building (over Power, needs 2) | income: 1 credit per Power tag on your cards |
  | Citywide Microgrid | energy (4, a late sink) | building | Action: 3 Sway toward Unity |
  | Hostile Buyout | antimatter, Diplomacy | event | gain 3 ambassadors; any corrupted site, a station or factory becomes yours (254) |
  | Forensic Audit | research cubes | event | draw 2 cards |
  | Root Access | antimatter, Cyber (needs Cyber tags) | event | remove a botnet on or next to the city |
  | Peacekeeping Mission | antimatter, Security (needs Security tags) | event | remove up to 3 adversary troops in the city's country |
  | Popular Uprising | Followers | tier-1 building | Action: add 1 Follower here, OR pay 4 Followers to liberate, again and again |

  They cost 6-26 credits (the four antimatter ones 6-22 and 1 antimatter), well under the Liberate action
  ([card-pricing.md](../card-pricing.md), *The liberate capstones*). Great Liberation (Diplomacy: its target and 2 more in the target's country, cities included)
  liberates too. **Part of the draft is guessing who will have which engine**, and passing each liberate card to the
  seat that can use it. Loops: each card is one-shot and tier 2, and every route needs nodes next to the city, so
  liberation → clean city → energy → more liberation is capped; Popular Uprising repeats, capped by its once-a-round
  Action and 4 Followers a liberation (about one every fifth round; every 2-3 with Simulation Cluster's Reuse).
- **Tag discounts (candidate; the Terraforming Mars pattern).** Every `science_every` Science tags you have take 1
  insight off your tier-2 keeps and your card plays; every `industry_every` Industry tags take 1 credit off your
  buildings, as steel does. Neither goes below `discount_floor`, the cap on the loop "tags, cheaper costs, more tags";
  a cost printed under the floor stays.
- **Tags:** Science, Industry, Cyber, Power, Diplomacy, Security; 1-3 a card. Yours are those *on your cards*
  ([README.md](README.md#terms)); a board's can't be sacrificed.
- **Programs** resolve and stay in your play area (their tags count; powers can sacrifice them); a lasting raise they
  print stays even when sacrificed (Twin City Compact's extra hosted city, 258, as production does). **Events** resolve
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

**Token cards, for new players.** A building that holds Operatives, Datasets, Followers or Capital starts with a few,
gains more each time you do what it names (play a Science card, say) or by its Action, and at its number pays and
loses that many. A card only ever counts its own tokens.

(Variant `off`: no draft, [README.md](README.md#off-variants).)

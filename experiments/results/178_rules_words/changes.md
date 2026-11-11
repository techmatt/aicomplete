# 178: what r77 changed (the spec the docs follow)

Rules revision 77 (bumped from the constant 76; the prompt said "r76", but 177 took 76). Matt's rulebook-friction
rulings of 2026-11-10 (#8, #17-19, #21, #22, #24, #26-30, #32), amendments 8044, 8112, 8118, 8122.

## Words (printed and in prose)

| Was | Now | Where |
|---|---|---|
| Deploy (each AI board's node power, 1 energy for 1 node) | **Expand** | `content/ais.json` power names |
| Deploy (the adversary's troop line on event cards, projects, the Protector landing, prohibitions "no Deploy") | **Muster** (verb: muster, mustered) | adversary cards, projects, Unity prohibitions, `Step::Muster` |
| a Unity or Discord card's "tier I-III" | **strength I-III** (the tip count picks the strength) | alignment text, docs; config `alignment.tier_at` keeps its name |
| "each board" on adversary cards (meaning a country) | **"each country"**; "a board" -> "a country" | adversary cards, projects; `Content::check` now refuses "each board" |
| Move (standard action), Movement (its allowance, printed 1-2 + track spaces "Movement +1") | **Dispatch** (the action), **Dispatches** (the allowance; track spaces "Dispatch +1"). Plays the same: one Dispatch puts an ambassador from your supply or anywhere on the map onto a piece, base, node or botnet's hex in reach | standard action words, view, AI tracks |
| corp troop(s) | **Drone(s)**, its own piece (179 draws the symbol) | all text; one name in code: `common::DRONE` |
| piece names on cards | capitalized on cards: **Ambassador, Agent, Node, Botnet, Troop, Drone** (amendment 8044). In the rulebook's and docs' prose: plain lowercase (ambassador, agent, node, botnet, troop, drone) | card faces, content text |

## Rules

1. **Tags (#21).** A card's own tags count for its own per-tag gains (as before, events excepted: an event's tags
   never count); those lines now print "(including this)" ("gain 1 credit per Science tag on your cards (including
   this)"). A card's own tags still never count toward its own requirement. Wording only, no play change.
2. **Effect triggers (#19).** Every trigger fires once per action (a card play, a building's Action, an AI power, a
   standard action), with its unit printed. Changed: "when you remove a Troop" (Security Contractor HQ) fired once
   per troop, now once per action that removed any; "when you place a token on your cards" (Data Licensing Deal)
   fired once per token, now once per placement. Unchanged: the per-tag play triggers ("+1 per Diplomacy tag on
   it") fire once and pay per tag; "when you remove agents" and "when you remove adversary compute" were already
   once per action.
3. **Wall, Vigil and the Protector's troops (#18).** The Protector's board now prints the landing: in each
   adversary turn's Muster, `adversary.wall_troops` (1) Troop (plus the level's Muster surcharge) lands in each
   country where the Protector has a node, on the lowest-numbered **adversary or neutral factory** there, never a
   player's; with none in the country, none lands (it used to take a player's factory, else a city). A Wall on that
   factory still keeps it in the supply. **Vigil:** "when a Troop ends its march next to your Node, gain 1
   insight": once per troop per march, any march (the threat's immediate march too); a troop used up striking has
   no end to its march. (It paid when the Wall stopped a troop that would have struck: a counterfactual.)
4. **Shelter (#24).** Prints "any country" (no reach needed). No play change.
5. **Draft (#26).** One fixed direction: always pass to the next player in seat order (it alternated by round).
6. **Removing around a hex (#27; reverses 169's D76, by Matt).** The strike cards (Kettle Operation, Lure Network's
   Action, Honeypot Relay) now read "remove up to N [agents/troops] on or next to a hex in reach, your choice": the
   hex itself counts, and the player chooses which pieces, batch by batch, and may stop. (It removed only from the
   hexes next to the hex, troops first, in hex order.) Nothing moves; the "gather" in their old text is gone.
7. **Land corruption (#28).** The Clear action (same price) may target a land corruption token in reach: it just
   comes off, no reward (a marker still pays its Unity and credits). Corrupted land forbids building: no factory
   (growth, cards, threats) and no research station on it, as no node goes there already.
8. **Liberate cards (#29, M1).** A liberate card liberates exactly as the Liberate action does, unpaid: the agents
   on the city come off, the adversary's cube comes off, Unity sways, and the player's cube goes on (always; the
   `align` field is gone). Great Liberation's cities likewise. No agent lands on a liberated city (`free_leaves` is
   gone from the config).
9. **Firewall and cordon (#30).** Both last "until the end of the next adversary turn" (printed on both): the
   firewall came off then already; the cordon (Hardpoint Lockdown) lasted to the end of the next round and now ends
   with the next adversary turn (at its last phase, the compute step). A firewalled site doesn't corrupt: neither a
   Corrupt X line nor natural corruption takes it (agents already on it stay; arrivals still can't land). A cordoned
   country's agent pool waits (it is kept, and lands at its next Rally); it used to be lost.
10. **Convert (#32).** Each converted agent becomes your ambassador where it stood (not on the node whose reach the
    card used; no node choice). The converted agents come off first, then the ambassadors go on, so two converted on
    one hex both stay; an agent the card left on that hex meets one of them, 1 for 1, as any contact does.
11. **Threats (#8).** Every threat card prints, above its text, "Payers: any player with a Node or Ambassador on or
    next to a token. Target: the player with a Node nearest a token." One location condition for every token type: a
    site of its type that is **not the adversary's** (cities, coastal cities, stations, factories; a base never is).
    The factory type's free-zone fallback is gone, so a threat whose only type has no such site in a country skips
    that country. A threat dealt after the action phase began gets a **waits** token (it executes next round), as
    before, now a printed token.
12. **Card claims of a station (amendment 8112).** A card that claims a neutral research station pays its extra
    (one ambassador per agent on it) as the Claim action pays: your own ambassadors on the station first, then your
    supply.
13. **The compute step's phase (amendment 8112).** The compute step is its own phase, `adversary.compute`, right
    after `adversary.reveal`; the firewall, cordon and prohibition ends move with it. No play change.

## The M-rows

Every M-row of docs/rulebook-friction.md is resolved. M1, M23, M27 and M39's Vigil half by the engine changes above
(a rules change); M16 by the doc (no rule puts an adversary cube on a megaproject; the printed payout never pays);
the rest by fixing the doc to the engine (items 178's helpers checked each against the code).

# 134: the drafting spec for the adversary decks (new model)

The brief every drafter worked from. The model itself is Matt's (2026-11-06, locked once built); this page fixes
the draft **format**, the **vocabulary** and the **value scale** so the decks agree. Drafts live in
[`drafts/adversary/`](../../../drafts/adversary/), outside what the engine loads (the engine reads `content/` only).
Nothing here changes r53.

## Files

| File | Holds |
|---|---|
| `drafts/adversary/threats_t1.json` | tier-1 threat deck (25: 5 per tag) |
| `drafts/adversary/threats_t2.json` | tier-2 threat deck (25 + 3 Universal) |
| `drafts/adversary/threats_t3.json` | tier-3 threat deck (11: 3 Universal, 3 Military, the rest 1-2 per tag) |
| `drafts/adversary/projects.json` | the universal project deck (20) |
| `drafts/adversary/militarist.json` | the Militarist's board and event deck (5 / 6 / 3 printed, 4 / 5 / 2 dealt) |

Each file is a JSON object `{"deck": ..., "cards": [...]}` (the Militarist file: `{"board": {...}, "events": [...]}`),
UTF-8, LF, one space indent like `content/`.

## Vocabulary

**Tags** (lowercase in data): `military`, `influence`, `cyber`, `economic`, `grid`, and `universal` (counts as every
tag, one of each; satisfies any pull). A threat's tags all sit at its deck's tier: a tier-2 card is "Military 2"
or "Military 2 + Economic 2". At most 2 tags a card (Universal alone).

**Locations** (a threat names 1 or 2; a token goes on the matching site in the pulling country when dealt):
`city`, `coastal_city`, `station` (research station), `factory` (a factory, else an industrial zone), `base`
(military base). **Placement when dealt (draft rule, after review):** a city or coastal-city token goes on an
uncorrupted city, a station token on an uncorrupted research station, a factory token on a factory that isn't the
adversary's, else a free industrial zone, a base token on a military base; among those, the one with the most
adversary pieces on or next to it (ambassadors, troops, botnets), ties the lower hex number. A printed game can do
this by eye. (The first rule, "most adversary pieces" alone, sent tokens to the adversary's own sites, where most
cards did nothing.)

## Table rules the drafts assume

These came out of the cross-deck review; a later rules item takes or changes them with the decks.

- **The pull**, printed on each tier deck's back: for each country in order, reveal threats from that tier's deck
  until one carries a listed tag (Universal fits any) and has a location type in that country; discard the rest;
  reshuffle the discard when the deck runs out. So a base-only threat never lands at 1 country (no bases).
- **Uncountered**, every threat also sways 1 toward Discord (today's `threat_discord`); cards don't print it. It is
  the floor that keeps a threat nobody is near from being free to ignore.
- **No threat or event takes a player's last node** (today: stage I only).
- **`nodes_near`**: the nodes removed leave the game as destroyed ones do.
- **Theft** hits the player near it with the most credits (ties the lowest seat), so the victim can always pay.
- **A threat that deals a threat** deals it now, into its own country, at a named tier; it executes next round.
- **War brought forward** prints as a second war step on the fronts next to the threat's base (`extra_war`):
  from round 6 the clock makes every round a war round, so "next round is a war round" would be blank.
- **Extra corp troops** (`corp_troops`) say when (`now` or `each_production`) and go past the base's cap: at the cap
  (almost always, after a production step) they would otherwise do nothing.

**Near** (the draft's one new term): a player is *near* a threat with a node or a player ambassador within 1 hex of
any of its location tokens. Any player near it may pay (pooled, as now). "Each player near it" is who an effect hits.
A placement the threat makes ("1 troop lands") lands **on its location** unless the text says otherwise.

**Counters are a menu**: `counters` is a list of options; the players pay **one** option in full (pooled). Items:
`{"resource": "credits"|"energy"|"insight"|"research_cubes"|"chips"|"ambassadors", "amount": N}`,
`{"sacrifice": "security"|"diplomacy"|"cyber"|"industry"|"power"|"science", "amount": N}` (sacrifice cards in play
carrying N tags of that kind, as a power's cost: [ai-powers.md](../../../docs/rules/ai-powers.md)),
`{"nodes_near": N}` (remove N of your nodes on or next to a location: Military's garrison counter).
Tag to sacrifice-tag map: Military → Security, Influence → Diplomacy, Cyber → Cyber, Economic → Industry, Grid → Power.

**Effects** use today's ops (`docs/ops.md`) with today's field names wherever one fits; scope is "near this threat"
(today's "here") unless a field says otherwise. An effect the engine lacks is written as the op you'd add, with
`"new": true` and a one-line `"why"`; list it again in the card's `flags`.

## Card fields

Threat: `id` (snake_case, unique across all drafts), `name`, `tier`, `tags` (list), `locations` (list of 1-2),
`text` (the uncountered harm as printed; plain words, American English, about 110 characters or fewer),
`counters` (menu), `effects`, `harms` (from the harm list below), `value` (one line: counter price, harm price per
player near it, the ratio), `check` (one line: when in an 8-10 round game it bites, who can realistically counter,
any loop and what caps it), `flags` (list; empty if none), `reuses` (today's id it reworks, or null).

Project: `id`, `name`, `tags`, `cost` (compute per player), `ongoing` (bool), `text`, `effects`, `harms`, `value`,
`check`, `flags`, `reuses`.

Militarist event: `id`, `name`, `stage` (1-3), `pulls` (list of `{"tier": 1-3, "tags": [...], "per": "country"}`;
`tags` is the set any of which satisfies, `["any"]` for a pull anything fits), `standing` (list of ops: compute,
pieces placed), `twists` (ops), `arrivals` (`coastal`, `industrial`, `stations`, `bases`, or null), `text` (the card
as printed, top to bottom), `harms`, `value`, `check`, `flags`, `reuses`.

Militarist board: `id`, `name`, `project_slots` (list of `{"tag": "military"|null}`), `rules` (1-2 persistent rules,
each `{name, text, effects}`), `build` (its numbers: `city_build`, `factory_build`, `arrivals`, `corrupt_at`,
`spread_range`, `wall_troops`, `escalation_ambassadors`, `botnet_compute`), `difficulty` (one line,
`{text, effects}`), `value`, `check`, `flags`.

## Harms (one or more per card)

Today's ledger rows: `node destroyed`, `credits or income lost`, `cards discarded`, `insight lost`,
`ambassadors lost`, `tokens stripped from cards`, `facilities destroyed or seized`, `production blacked out`,
`adversary pieces placed`, `projects advanced (compute)`, `extra threats`, `Discord sway`, `Unity pulled back`,
`actions or claims blocked`; new rows the model adds: `energy lost`, `war brought forward`, `troops made`
(corp troops: more war, more Discord). Rule kept: every harm from at least 2 kinds of source across the drafts; each
threat tier at least 5 kinds of harm.

## Value scale (the anchors, `docs/card-guidelines.md`)

credits 1; energy 4.5; insight 4; an ambassador (yours) 4; a research cube 6; a chip 3; a card discarded from hand 4;
a node destroyed 6; a sacrificed tag 5 (late-game liquidation; dearer early); a compute point 2; a sway 4; an
adversary ambassador placed 5 (what removing one costs); a troop landed 5 plus its strike; a botnet 16; a
corrupted city 45 (a liberation); a corrupted station 20; an adversary factory 15; a site claim blocked a round 6.

**Income**: a seat makes 14-18 credits a round from its board at the start, about 22 at mid-game and 30+ late.
**Calibration (`docs/rules/threats.md`)**: a counter is about a third to half a round's income (tier 1: 6-8 credits,
or 1-2 energy, 4-5 insight, 1-2 ambassadors, 1 research cube, 1 tag; tier 2: 9-12 credits, 2-3 energy, 6-8 insight,
2-3 ambassadors, 2 cubes, 2 tags; tier 3: 13-18 credits, 3-4 energy, 3 tags, or two resources); failing costs about
1.5-2× the counter **per player near it** (count about 1.5 players near a threat). No counter prints 1 credit;
credits counters are 5+.

**Tier rules**: tier-1 threats hit this round or next (credits, energy, insight, cards, a blackout, adversary
ambassadors) and destroy nothing permanent; permanent losses (nodes, facilities, corrupted sites, botnets) are
tier 2+. Prefer choice threats ("lose 4 credits or discard 2 cards": the player near it chooses). Tier 3 is a late
squeeze strong engines can still answer. Threats execute after production and war, at the round's end.

## Round order (timing words)

round start (refresh, AI boards sway) → event card → reveal the next card → draft and keep → actions (countering
included) → production from one snapshot (troops, energy, stations, income, upkeep) → war (its damage lands on next
round's production) → uncountered threats execute → cleanup.

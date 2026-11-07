# 137: replacing old jobs

Each rule of r55's ambassador site table, the production/war clock, bases, `factory_units` and the Free a station
action, and what takes it over at r56.

| r55 rule | r56 |
|---|---|
| Adversary ambassador on a city: stays, the city is dark | Kept: the site table's "one opposing ambassador blocks" (a dark city is the blocked neutral or player city) |
| Player ambassador on a dark city: Align cancels one adversary ambassador instead | The Claim action costs 1 more for each adversary ambassador on the site, and they all cancel |
| Corrupted city liberated by `liberate_at` standing player ambassadors (the fallback, checked after every action) | The Liberate action: `liberate_at` 5 ambassadors from one seat's supply (+1 a standing adversary ambassador), reach only |
| Adversary ambassador on a station: takes that round's cube for the adversary, used up | Blocks the station while it stands (`blocker_stays` 1; 0 is the r55 "used up" kept as a dial) |
| Player ambassador on a neutral station: claims it (1 ambassador) | The Claim action, `claim_station` 2; an arriving ambassador just stands |
| Player ambassador on a corrupted station: takes its cube, stays | Blocks its compute while it stands |
| 2 player ambassadors + 6 credits free a corrupted station (Free a station) | The Liberate action, `liberate_facility_at` 3, sways `free_facility` 2 Unity |
| Adversary ambassador on a factory: an adversary troop on its hex (locked), used up | Blocks a player's factory (chips) or a neutral one (its tip troop) while it stands; no troop |
| Player ambassador on a neutral factory: claims it | The Claim action, `claim_factory` 2 |
| Player ambassador on a factory: no corp troop; on an adversary factory, no troop; stays | Player factories make no corp troops (nothing to stop); on an adversary factory it blocks the troop while it stands |
| Adversary factory frees as a station does (Free a station, 135 ruling 3) | The Liberate action, as a station |
| Adversary ambassador on a base: `base_discord` Discord instead of a troop, used up | Cut (a base has no ambassador rules beyond the troop rule; no botnet forms on one) |
| Player ambassador on a base: no troop, stays | The troop rule: used up instead of the next corp troop made there |
| Each base makes 1 troop on a production step | Cut: corps make troops only at Discord tips (neutral factories) and from the extra-troop rules |
| Every factory makes `factory_units` corp troops at each of its corp's bases | Cut: a neutral factory makes 1 corp troop at each Discord tip, which its corp puts on each of its bases |
| The clock (`board.clock` 1,2,1,2,1,3): production rounds 1, 3, 5, 6+; war rounds 2, 4, 6+ | Production every round; war at the end of every round (after threats execute). `board.clock` kept at [3] as a restore dial |
| A war twist (`War`) adds a war step | Reads, but changes nothing while every round is a war round (no card prints it) |
| Casus Belli, General Mobilization: fight twice in next round's war step; once with no war step | Fight twice in next round's war (always a war now) |
| `destruction_discord` 0 | 1: each factory destroyed in war sways 1 Discord (`alignment.discord_from.war`) |
| "First base": troops at base (Factory Drive, War Drums), base arrivals (War Drums, Border War, Blitzkrieg), Escalation | Troops: a factory, else a city (the default landing). Arrivals and Escalation: each country's lowest-numbered factory (new arrival site `factory`), one a country as before |
| `base_sites_per_country` | Cut, with `Site::BaseSites` and `TroopAt::Base` |
| A player ambassador on any facility hex absorbs its destruction | Armor: only the owner's ambassador (a node's owner, or the site's owner, which also covers a node on its hex) |
| Adversary cubes on a megaproject and after completion (the corp's adversary payout and `after` line) | An adversary station makes compute instead; the payouts fire only if a card puts an adversary cube on |

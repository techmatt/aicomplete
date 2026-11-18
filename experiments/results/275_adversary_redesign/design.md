# 275: the card-driven adversary (design as built)

Matt's rulings (2026-11-18) are in the item prompt; every shape below the rulings is a **candidate**.

## The adversary's turn, three beats

1. **Card.** Next round's face-up event card becomes this round's (none left: a loss) and deals its threat pulls.
   Then, in the printed order (locked order, new lines):
   - **Infiltrate X** (one line; two at stage III): 1 agent (`infiltrate_agents`: 1 at d0-d3, 2 at d4) on each X
     in every country that is not yet the adversary's, whoever owns it. An arrival on a player ambassador cancels
     one (players choose whose); a firewall or a no-Infiltrate token keeps that site's agents in the supply; a
     cordoned country takes none.
   - **Muster N** (unchanged).
   - At most two other lines (compute income, a Discord sway, a site opening, Botnets spread, Blackout, Fallout...).
   - **Spill (level 2+): X into Y** (most stage II and III cards): each X not yet the adversary's holding 2 or more
     agents moves 1 onto the Y next to it holding the most agents (ties: the lowest-numbered; none: it stays). Below
     level 2 the line does nothing (`spill_at` 2; the card prints the level, like Botnets spread).
   - **Grow: X** (12 of the 16 cards): in each country, the X holding the most agents (ties: the lowest-numbered)
     that has a free hex next to it and fewer than `grow_cap` (2) facilities next to it builds one **neutral**
     facility there: the type it has fewer of next to it (ties:
     a factory; a factory only on a free industrial zone, a station on other open land, no base), else the other
     type. If that X holds agents or is the adversary's, the facility comes with 1 agent on it (an arrival).
2. **March** (unchanged).
3. **Income.** The top card goes face up as next round's (its Infiltrate and Grow sites are public a round ahead).
   **Each botnet** with no player ambassador on it **seeds** 1 agent (`botnet_seed`) onto the city, station or
   factory next to it, not yet the adversary's, holding the most agents (else onto its own site); then the compute
   step and the research check (unchanged); firewalls, cordons and prohibition tokens come off.

**Arrival corruption (the one spread rule).** Whenever the adversary puts agents on a hex (any placement from the
supply: Infiltrate, Surge, Grow's agent, a botnet's seed, a threat's or Discord card's creation, a threat's anchor;
or a Spill's move), that hex alone is checked: a city, station or factory not yet the adversary's holding its
corrupt number (city 5, 3 from d1; station 2; factory 2) **corrupts** (the aftermath is unchanged: the adversary's
cube, its agents to the supply, a node there destroyed, ambassadors there lost); else **2 agents on a player's
node** destroy the node. A firewall or a no-Corrupt token holds it. A player's Push or Gather is no arrival (it
checks nothing). Nothing else on the board is looked at.

**Uncountered threats** also land `threat_agents` (1) agent on their anchor (the first location token), an
arrival like any other.

**Projects** print **Surge N: X** (N agents onto each country's X holding the most agents) in place of Rally and
Corrupt lines.

## What each old job became

| Old | Its job | Now |
|---|---|---|
| Rally X (event lines) | put agents on a site type each round | **Infiltrate X**: 1 a site, no pool |
| Agent pools, agent income (`ambassador_income`), Garrisons' +1 | how many agents a round | the number of sites the card names; stage III names two types; d4 `infiltrate_agents` 2. Garrisons became **Botnet Recruiters** (botnets seed) |
| "+1 Agent a country a round" (11 event cards, Recruitment Drive) | the escalation by stage | gone; stage III's second Infiltrate type, and Spill from level 2 |
| Projects' "Rally 3: X" | a burst of agents | **Surge 3: X**, onto the most-infested X |
| Corrupt X lines and their adjacency move | concentrate agents and corrupt | arrival corruption; Spill and Surge concentrate |
| Natural corruption | corrupt whatever already holds its number | arrival corruption (a site can only reach its number by an arrival) |
| Corrupt open land (Blitzkrieg, Naval Blockade) | Fallout | a printed Fallout line (next to the lowest-numbered corrupted city) |
| Liberated marker's +1 (`retake_agents`) | protect a freshly liberated site | **cut**: liberation clears the site's agents, so it needs its full number of arrivals again; one marker fewer |
| Sleeper Cells (d1: city at 3) | the ladder's first rule | kept: "When an Agent lands on a city, it falls at 3, not 5" |
| 2 agents on a node | node loss | checked on arrival only |
| City growth and chips (`city_chip_cap`, `city_build_chips`) | new neutral facilities over the game | **Grow: X** on the card: one a country, chosen by the card's type and the most agents; chips gone |
| Discord cards' "then Corrupt cities" / "rallies its income now" | a burst | their creation lines +1 agent (an arrival may corrupt); "Surge 2: cities"; "this round's Infiltrate lines land again" |
| Unity's "no Rally" and "no Corrupt ... and no Agent moves onto it" | prohibitions | "no Infiltrate"; "nothing corrupts or flips" (Spill's move isn't blocked) |

## Every step and what it acts on

| Step | Acts on |
|---|---|
| Threat pulls | the card's pulls; a threat's location rule names its site |
| Infiltrate X | the sites of the type the card names |
| Muster N | each country's lowest-numbered factory (else city) |
| Other lines | what each line names |
| Spill X into Y | the named X holding 2+ (sites of one named type) and the Y next to each |
| Grow X | the named type's sites in each country (one grows) |
| An arrival | the hex the agents landed on |
| March | the troops (unchanged) |
| Botnets seed | each botnet and the sites next to it |
| Compute, research | projects and the research track (unchanged) |
| Projects' Surge | the named type's sites in each country (the most-infested) |
| Uncountered threat | its anchor |

No step looks at every site on the board.

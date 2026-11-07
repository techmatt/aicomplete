# 138: where each old job went

The spread step, its priority rules, botnet formation by spreading and arrivals by site type are gone. Each job they did,
and what does it at r57:

| Old rule or op | Its job | At r57 |
|---|---|---|
| Spread step (`adversary.spread`, `spread_range` 2) | moved ambassadors from crowded hexes onto pieces, the adversary's movement | **Corrupt X** lines (`adversary.corrupt`): a site gathers from itself and next door in its country; the pull is the only move |
| Spread priority (player-owned, then clean cities, then other pieces; nearest; players choose) | aimed the pressure at players | creation lines name their target (player sites, clean cities, the city already filling, corrupted cities); lowest-numbered breaks ties, no choice |
| Surplus returns to the supply | capped the spread | the supply cap, and every pulled ambassador returns when its site corrupts |
| `spread_shield` (Sanctuary capstone) | kept the spread off your nodes | `corruption_shield`: corruption never destroys your nodes |
| Spread reach twists (Long Patrols, Blitzkrieg) | stage-I / stage-III escalation | Long Patrols: Corrupt stations and a neutral station opens; Blitzkrieg: Corrupt cities and Corrupt land |
| Propaganda Ministry's double spread (`double_spread` flag) | more pressure in one country | create 1 on each of the 2 lowest clean cities in its country, then Corrupt cities there |
| Grand Offensive / Offensive "the adversary spreads" | an out-of-turn push | Corrupt cities (Offensive at 4+ tips builds first) |
| Botnet formation (a second spreading ambassador on a piece) | made botnets during play | **printed lines only**: Sleeper Cells, Full Spectrum Assault and Cascade Failure place one (as before), plus each board's setup botnet. Botnet hexes are a creation target (Signal Jamming, Botnet Swarm), so botnets still feed ambassadors |
| Corrupt step (`corrupt_at` on the city's own hex) | corrupted full cities | **natural corruption** (`adversary.natural`): any city, station or factory already holding its number corrupts; Corrupt X lines gather |
| `corruption_reach` 1 (nodes next to a corrupted city lost) | corruption's node harm | cut: only a node on the corrupted hex, and an allied ambassador there is armor (destroy_piece) |
| `CorruptSooner` twist (Fifth Column, Occupation) | cities corrupt at 1 fewer | a Corrupt line's "(1 fewer)" (`cut`) |
| Arrivals by site type (coastal, industrial, stations, each country's lowest factory; `arrivals` 1) | ambassadors every round on every matching site | each card's creation lines: "create 1 on each of the 2 lowest-numbered coastal cities" (and research stations, factories) |
| Escalation (stage II, `escalation_ambassadors` 1 on each country's lowest factory) | stage-II pressure | stage-II cards print "create 1 on the lowest-numbered factory" (Border War: 2) |
| `infiltrate` (each country's clean city with the most gains N) | focus on a filling city | "create N on the lowest-numbered city holding adversary ambassadors" (Long Patrols, War Drums, Fifth Column, Forced March, stage III, Blight, Firestorm) |
| `city_build` 2 (each corrupted city), `build_bonus` (d3) | corrupted cities grow | the board's creation line: create 2 on the lowest-numbered corrupted city; `build_bonus` adds to it at d3 |
| `factory_build` 0, `clean_factories` twist | factories near cities fill | creation lines on the lowest-numbered factories (Factory Drive, Supply Lines, Total Mobilization) |
| `station_build` twist (each corrupted station) | adversary stations grow | Station Raids: create 1 on the 2 lowest research stations, then Corrupt stations |
| `botnet_ambassadors` (Signal Jamming, Botnet Swarm) | botnets spawn ambassadors | create 1 (2) on each of the 2 lowest-numbered botnet hexes |
| `clean_city_ambassadors` (Recruitment Drive) | every clean city gains 1 a round | create 1 on each of the 2 lowest-numbered clean cities a round |
| `city_build` ongoing (Propaganda Network) | corrupted cities build 1 more | create 1 on the lowest-numbered corrupted city a round |
| Threat `ambassadors` placements (players choose among near pieces) | a threat's ambassadors | creation lines in the threat's country (Call-Up, Agitators, Insider Access, Turncoats, Overrun, Propaganda Ministry, Dark Cities, Garrison State) or "here" (Insurrection) |
| Overrun's corrupt-if-3 / Infiltration Drive's corrupt the most infiltrated city | direct corruption | Corrupt cities (1 fewer) |
| Unchanged | | Shadow Factories' fallback (2 ambassadors on a factory, players choose) is the one placement left outside the creation shape; Regime Change still corrupts its own city token |

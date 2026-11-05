# 121: scenes of their own for 117's four new cards

117's four new cards sat on scenes drawn for the cut cards whose slots they took. Each now has a scene
drawn new in batch 8 (`art-originals/scenes/b8/`, picker record `picks/scene_batch_8.json`). The edit is
[assign_art.py](assign_art.py), run once on the r46 `scenes.json` through `scenes_io`. No code, card text
or numbers changed.

| card | was showing | now | source | match |
|---|---|---|---|---|
| Fabrication Megaplex | foundry_furnace_hall (Foundry Core) | megaplex_fabrication_floor | new, b8 | a huge sawtooth-roofed fab hall from a gantry, robot cells, trays of chips on conveyors |
| Frontier Lab Charter | breakthrough_lab_flash (Breakthrough Lab) | frontier_lab_opening | new, b8 | a new glass lab opening on a hill campus, research cubes carried in, a founder holding up a blank charter |
| National Grid Buildout | corridor_through_forest (Transmission Corridor) | grid_buildout_crews | new, b8 | crews and cranes raising a pylon line across farmland, three substations going up |
| Decapitation Strike | strike_team_rooftop (Quick Reaction Force) | strike_on_the_rogue_hub | new, b8 | a coral-trimmed team pulling cables from a cold-blue machine core on a rooftop, dark-suited envoys led away |

## Subjects

Each is drawn after the locked preamble, emphasis, lighting and framing lines and before its tag's accent.

- **megaplex_fabrication_floor** (industry): A vast modern fabrication megaplex seen from a high gantry, rows of robotic assembly cells under a long sawtooth roof running to the far wall, conveyor lines carrying finished parts and trays of green circuit chips, small workers on the floor below; no flags or emblems.
- **frontier_lab_opening** (science): A brand-new frontier research lab of glass and white steel opening on a hilltop campus, scientists in lab coats carrying the first crates of glowing sky-blue research cubes through its tall doors while a founder holds up a blank rolled charter to a small cheering crowd; no flags or emblems.
- **grid_buildout_crews** (power): A national power grid being built across open country, crews and cranes raising a long line of new lattice pylons over farmland toward the horizon, cable drums on trucks, three fresh substations taking shape along the route, a power station far off; no flags or emblems.
- **strike_on_the_rogue_hub** (security): A security team in coral-trimmed tactical gear storming a hijacked rooftop server hub in daylight, one pulling the thick cables from a glowing cold-blue machine core while others lead a line of dark-suited adversary envoys away, figures seen from behind; no flags or emblems.

All four draws were kept first time: illustrative gouache, daylight, no text, flags, logos or emblems. The
lab opening shows its crowd at mid distance, no close-up faces. Each scene was drawn twice (a duplicate
launch); the second draw is `<id>.png`, the picker's, and the first stays as `<id>.try1.png` in `raw/`. The
four released scenes keep their `was` and are back in the unused pool. Every deck card names a scene no
other card names (`cards:check` all good).

# 152: scene candidates for Power Purchase Agreement and Microgrid Lab Compact

Batch 9 (`art-originals/scenes/b9/`), picker `art_d69` (`picks/art_d69.json`, previews in
`art-originals/picks/art_d69/`): one pick-one row per card, three candidates each, nothing selected.
The edit is [add_scenes.py](add_scenes.py), run on the r65 `scenes.json` through `scenes_io`, then
again with `--assign`. No code that plays, card text or numbers changed.

Both cards turn energy into a research station, so the two sets are split: the agreement's scenes
are a deal (a contract, a handshake, power sold to a station), the compact's a place (a lab campus
on its own grid). None uses solar panels, so none repeats `citywide_microgrid`'s shared rooftop
solar; the compact's subjects say "no solar panels" outright.

Both cards' first tag is power, so the six are filed under `power` and drawn with its amber accent
(Amendment #6670). Each carries `for: <card>`, a new key: written as a candidate for that card,
suggested for none. Matt's pick moves the winner to `card`.

## Subjects

Each is drawn after the locked preamble, emphasis, lighting and framing lines and before the
power accent. Each tile in the picker is the scene over both cards (event window, building window).

**Power Purchase Agreement** (a deal)

1. `ppa_handshake_at_the_truck` (picked, Matt 2026-11-08): A power-company engineer in an amber hard hat and a research-station director in a field jacket shaking hands over a signed blank contract laid on the hood of a utility truck, a crew behind them unspooling a thick new cable from a small substation across a meadow to a domed research station; no flags, logos or emblems.
2. `ppa_contract_at_the_wind_farm`: Two negotiators at a folding table set out in the grass beneath a row of wind turbines, one sliding a signed blank contract across to the other, a single amber cable running from the nearest turbine past the table to a domed research station on the next rise, crates of glowing sky-blue research cubes waiting at its door; no flags, logos or emblems.
3. `ppa_meter_handover`: At the gate of a domed research station, a utility clerk handing a large brass power meter and a rolled blank contract to a smiling scientist, a line of fresh wooden poles carrying new wire back over the hills to a distant power plant; no flags, logos or emblems.

**Microgrid Lab Compact** (a place)

1. `microgrid_lab_battery_yard` (picked, Matt 2026-11-08): A self-powered research campus in a green valley, low white laboratories set around a central yard of battery cabinets and a small control hut, two wind turbines on the slope and a little river-turbine house on the stream feeding it, an engineer checking a panel of amber lamps while scientists wheel crates of glowing sky-blue research cubes between the labs; no solar panels, no flags, logos or emblems.
2. `microgrid_lab_control_room`: Inside a bright research campus's own power control room, a tall window looking out over its battery yard, wind turbines and white lab buildings, an engineer pulling a large lever as a row of amber lamps lights up on a hand-drawn map of the campus grid, a scientist alongside holding a glowing sky-blue research cube; no solar panels, no flags, logos or emblems.
3. `microgrid_lab_island`: A small research lab campus on a rocky island running on its own grid, tidal turbines churning in the channel, a stone battery house and a few wind turbines on the cliffs, cables climbing to a domed research station on the headland where scientists unload crates of glowing sky-blue research cubes from a boat; no solar panels, no flags, logos or emblems.

## Draws

All six were kept on the first draw (42 to 53 s each, three at a time; Codex's `primary` usage reading 1 to 2 after them).
All six are illustrative gouache in daylight, with no text, flags, logos or emblems. The two contracts
show only a scribble where a signature would go. The two handshake scenes (agreement 1 and 3) are
half-length figures at mid distance, the closest framing in the set: the faces are small at card
size, but they are the nearest any of these come to the "no close-up faces" line. The control room
is the only interior and the only one that shows a map, drawn as a painted panel with no labels.

## Also

- **Market Stabilization Board**, back in the deck since 151, had no scene, so `cards:check` failed on it.
  Its own batch-5 scene, `balance_on_the_floor` (held as `was` since it left the deck), is suggested for it
  again through `scenes_io assign`.
- `npm run art:alts` (in `tools/card-art/`) is `scene-gen`, then `scene-picker --one`. `--one` writes a
  pick-one row per card the batch was written for, keyed by scene id, rather than batch 1-8's keep/reject row per scene.

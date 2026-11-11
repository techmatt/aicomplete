# 186: scene candidates for the ten tier-2 cards without a drawn scene

Batch 10 (`art-originals/scenes/b10/`), picker `art_d83` (`picks/art_d83.json`, previews in
`art-originals/picks/art_d83/`): one pick-one row per card, three candidates each, nothing selected.
The edit is [add_scenes.py](add_scenes.py), run on the r79 `scenes.json` through `scenes_io`, then
with `--copy`. No code that plays, card text or numbers changed.

## What each row holds

1. **The card's own 158 scene** (keeps `card`), its subject rewritten to the locked style (C+E,
   mid-key daylight, no flags, logos or emblems). 158's subjects asked for dawn, dusk, night, a dark
   data hall and "a wall of foreign flags"; each now says daylight and plain banners, and ends "no
   flags, logos, emblems or text." These are drawn, so each card has a served scene now and
   `npm run cards:check` in `web/` passes; they are provisional until Matt's pick.
2. **New candidates** (`for` the card), filed under the card's first tag.
3. **An unused drawn scene** (`was`, suggested for no card) that broadly fits, where one exists
   (Matt, 110): seven of the ten. Each keeps its `was` and gains `for`; its raw draw is copied
   from its old batch into `b10/raw/`. Mass Defection, Embassy of Exiles and Double Agents had no
   fitting unused scene and get two new ones instead.

| Card | Own 158 scene | New | Reused (drawn in) |
|---|---|---|---|
| Mass Defection | `defectors_crossing` | `defectors_leave_the_factory`, `defectors_on_the_footbridge` | none |
| Embassy of Exiles | `exiles_embassy` | `exiles_hotel_courtyard`, `exiles_at_the_gate` | none |
| Rally the Allies | `allies_rally` | `allies_at_the_airfield` | `joint_barracks_handover` (b3) |
| Double Agents | `double_agents_meet` | `double_agent_handoff`, `double_agents_cafe` | none |
| Kettle Operation | `kettle_cordon` | `kettle_trucks_close_in` | `sealed_hex_of_barricades` (b4) |
| Surgical Strike | `surgical_strike_team` | `surgical_strike_beam` | `strike_team_rooftop` (b1) |
| Lure Network | `lure_server_room` | `lure_decoy_racks` | `countermeasure_flares` (b3) |
| Firewall Array | `firewall_array_wall` | `firewall_emitter_ridge` | `aegis_dome` (b4) |
| Air Gap | `air_gap_cut` | `air_gap_island` | `dark_control_room` (b2) |
| Open Source Uprising | `open_source_hackathon` | `open_source_march` | `preprint_mailbag` (b2) |

The full subjects are in `tools/card-art/scenes.json` and in each row's note in the picker.
Each tile is the scene over its card and a partner of the other kind: the diplomacy events over
Embassy of Exiles, the security events over Agency Directorate, the cyber buildings over Air Gap,
Open Source Uprising over Interdisciplinary Institute.

## Draws

All 23 new scenes were kept on the first draw (37 to 67 s each, three at a time; Codex's
`primary` usage reading 7 after them). All are gouache-style illustration in daylight with no
text, flags, logos or emblems; the hackathon's screen shows only colored blocks, the café's
newspapers are blank. Watch at card size:

- `double_agents_meet` and `open_source_hackathon` have half-length figures in the foreground,
  the nearest these come to the "no close-up faces" line.
- `kettle_cordon` is seen from almost straight above and fills the frame; the building window
  crops into the ring.
- `joint_barracks_handover` (reused) has tall banners with plain chevron marks: not real flags,
  but emblem-like. `aegis_dome` and `countermeasure_flares` were drawn coral red for security;
  their cards are cyber (green).
- `exiles_hotel_courtyard` has solar panels on its roof.

## Next step

Matt's pick per row, then each winner moved to `card` through `scenes_io assign` (a `for` key
popped first, as 155's `pick_d69.py` did) and the losing own-158 scenes released.

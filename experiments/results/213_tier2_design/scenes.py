"""213: an unused drawn scene for each new card (tools/card-art/README.md: an unused scene that broadly fits a new
card's name goes there before a new draw). Run from the repo root: python experiments/results/213_tier2_design/scenes.py
"""
import os
import sys

sys.path.insert(0, os.path.join('tools', 'card-art'))
import scenes_io  # noqa: E402

PICKS = {
    'collider_consortium': 'breakthrough_lab_flash',   # a beam lancing out of a hilltop lab
    'catalyzed_fab_run': 'arc_furnace',                # a blinding contained arc in a furnace bowl
    'antimatter_accord': 'allies_at_the_airfield',     # delegations' planes side by side
    'cyclotron_lease': 'sample_repository_shelves',    # shelves of small luminous cubes: stored antimatter
    'accelerator_diplomacy': 'treaty_bridge',          # a bridge across a border carrying a shared line
}
s = scenes_io.load()
for card, scene in PICKS.items():
    scenes_io.assign(s, card, scene)
scenes_io.save(s)
print('assigned', len(PICKS))

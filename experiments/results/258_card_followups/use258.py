"""258's Use tables: 256's use256.py on this pass's cards (keeps, plays and Action uses; claims by route; nodes by
source; each resource's spend share).

    python experiments/results/258_card_followups/use258.py READ [--md OUT]

READ a merged read (merge.py): OUT.jsonl=OUT.spec.json.
"""
import os
import runpy
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
USE256 = os.path.join(HERE, '..', '256_item_c_cards', 'use256.py')
CARDS = ['mesh_repeater_network', 'research_reactor', 'survey_drone_fleet', 'orbital_relay_launcher',
         'antiproton_foundry', 'microgrid_lab_compact', 'field_assay_team', 'dyson_swarm_tranche',
         'exascale_training_cluster', 'citywide_microgrid', 'twin_city_compact', 'power_purchase_agreement',
         'relay_mast_rollout', 'positron_storage_ring', 'wafer_fab_line', 'parts_exchange', 'bug_bounty_board',
         'treaty_isotope_bank', 'synthetic_data_forge']

# use256.py reads its module-level CARDS when main runs: load it without running, swap the list, then run main.
mod = runpy.run_path(USE256, run_name='use256')
mod['CARDS'][:] = CARDS
sys.exit(mod['main']())

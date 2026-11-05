# 119: tests pinned off live card numbers

106 and 108's notes: core tests read a card's current numbers from content, so a card pass broke them by the dozen.
119 pinned what each asserts inside the test, at the r47 value; a reprice of the card no longer breaks it. The test's
meaning and assertions are unchanged.

## By card JSON written in the test (`pinned()`, tests/face.rs)

- `a_face_opens_with_its_boxes_word` (Synthetic Data Forge, Orbital Mass Driver), `a_bare_arrow_is_a_free_action`
  (Sentinel Mesh, Forge), `income_below_the_art_raises_or_lowers_the_track` (Endowed Research Chair),
  `each_mark_has_one_reading` (programs built from the ops it checks).

## By overlay (`table_with`, `lab_actions_with`, the new `threat_lab_with`)

- **engine.rs** (shared pins `TELEMETRY_PASSIVES`, `TELEMETRY_EFFECTS`; `FORGE_ABILITY` in tests/mod.rs):
  `income_per_tag_pays_its_amount_per_tag` (campus passives), `a_play_trigger_fires_for_its_tag_and_counts_itself`,
  `a_rebate_reads_the_printed_cost` (lights_out_assembly passives, two costs), `discounts_by_tag_take_credits_or_insight_off_down_to_the_floor`
  (planetary_referendum insight), `a_threshold_cashes_the_tokens_its_trigger_leaves`, `reuse_uses_another_action_again_but_never_a_reuse`,
  `placing_a_dataset_pays_data_licensing_deal`, `fight_triggers_fire_once_an_action_or_once_a_troop` (signal_hijack),
  `a_threshold_pays_when_a_feeder_fills_it` (intrusion_lab, recruit_operatives), `a_move_up_to_n_does_not_veto_a_card_that_also_gains`
  (summit_caucus), `a_program_counts_its_own_tag_toward_its_gain_per_tag` (consular_pouch),
  `production_per_tag_counts_once_at_play_and_counts_itself` (research_royalties).
- **cards.rs**: `playing_a_card_pays_its_insight_and_lowers_its_income` (lowers),
  `an_or_action_adds_here_then_offers_both_halves_and_counts_only_its_own_tokens` (forge),
  `a_draw_event_cannot_draw_itself_back` (breakthrough_leak).
- **boards.rs**: the three Rush Build tests (open_assembly, market_stabilization_board, capital_projects_office: energy,
  insight, needs).
- **freeing.rs**: `only_a_tier_2_ultimate_frees_a_city_outright` (great_liberation needs),
  `clearing_a_city_takes_the_ones_the_player_picks` (amnesty_decree, liberation_accords; both costs stay under the
  30 credits `fund` gives).
- **projects.rs**: `security_cards_drain_or_discard_a_project` (zero_knowledge_exploit, killswitch_directive),
  `botnet_takedown_removes_a_botnet_in_reach` (singularity_exploit).
- **researcher.rs**: `the_new_cards_can_be_played` (sample_repository, forensic_audit).
- **threats.rs**: `jamming_a_threat_down_to_what_is_paid_counters_it` (red_team_audit).
- **threat_sources.rs**: `an_auto_counter_works_its_home_country`, `a_threat_an_auto_counter_pays_in_full_is_countered`
  (shared `GRID` pin, auto_counter 5).

## Left as they are

- Tests whose expected values read the card's number from content at run time (citation_engine, the per-tag form,
  teammate triggers, copy income, a play-tag trigger with no tag): they follow a reprice by themselves.
- Board and map tests (insight powers, the converters, Vigil): the overlays don't reach them, and they name no player card.
- Tests that pick cards by a filter or tag (discount floors, `gain_per_tag`, the price walk, `content().check()`).
- endgame.rs: names no player cards. stats.rs's `economy_buildings_print_income` reads the global content, which no
  overlay reaches, and depends on which cards print an income (a rework), not on the amounts.

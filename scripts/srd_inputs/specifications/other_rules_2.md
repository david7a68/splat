---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:rules/rhythm_of_play:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/rhythm_of_play
        implementation: dnd2024:rule_rhythm_of_play
        parameters:
          resolution_precedence:
          - explicit_exception
          - general_rule
          scene_cycle:
          - gm_describes_scene
          - players_declare_actions
          - gm_resolves_and_narrates
          outside_combat:
            actor: gm
            choices:
            - activity_order
            - resolution_method
            all_characters_receive_opportunity: true
          combat_turns: true
    contract:
      id: dnd2024:rule_rhythm_of_play
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - resolution
      events: []
      capabilities:
      - rules
      - turns
    requires:
    - dnd2024:rule_rhythm_of_play
    citations:
    - SRD 5.2.1 pp. 5; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/rhythm_of_play.md
  dnd2024:rules/the_six_abilities_ability_modifiers:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/the_six_abilities_ability_modifiers
        implementation: dnd2024:rule_the_six_abilities_ability_modifiers
        parameters:
          ability_score_range:
          - 1
          - 30
          ability_modifier:
            op: floor
            args:
            - op: divide
              args:
              - op: subtract
                args:
                - variable: ability_score
                - 10
              - 2
          apply_to: d20_test_using_ability
          fractional_multiplication_and_division_rounding: floor
          explicit_round_up_overrides: true
    contract:
      id: dnd2024:rule_the_six_abilities_ability_modifiers
      inputs:
        actor: participant
        context: resolution
        ability_score: number
      outputs:
      - statistics
      events: []
      capabilities:
      - statistics
    requires:
    - dnd2024:rule_the_six_abilities_ability_modifiers
    citations:
    - SRD 5.2.1 pp. 5, 6; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/the_six_abilities_ability_modifiers.md
  dnd2024:rules/d20_tests_ability_modifier:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_ability_modifier
        implementation: dnd2024:rule_d20_tests_ability_modifier
        parameters:
          ability_selection:
            actor: gm
            criterion: most_relevant_ability
          ability_examples:
            strength:
            - lift
            - push
            - pull
            - break
            dexterity:
            - nimble_movement
            - quick_movement
            - quiet_movement
            constitution:
            - exceed_endurance
            intelligence:
            - reason
            - remember
            wisdom:
            - notice_environment
            - notice_behavior
            charisma:
            - influence
            - entertain
            - deceive
          check_modifier: selected_ability_modifier
    contract:
      id: dnd2024:rule_d20_tests_ability_modifier
      inputs:
        actor: participant
        context: resolution
      outputs:
      - check_modifiers
      events: []
      capabilities:
      - checks
    requires:
    - dnd2024:rule_d20_tests_ability_modifier
    citations:
    - SRD 5.2.1 pp. 6; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/d20_tests_ability_modifier.md
  dnd2024:rules/d20_tests_saving_throws:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_saving_throws
        implementation: dnd2024:rule_d20_tests_saving_throws
        parameters:
          trigger: threat_requires_save
          result_source: triggering_effect
          voluntary_failure:
            optional: true
            roll: false
    contract:
      id: dnd2024:rule_d20_tests_saving_throws
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - save_resolution
      events:
      - saving_throw.pending
      capabilities:
      - saving_throws
    requires:
    - dnd2024:rule_d20_tests_saving_throws
    citations:
    - SRD 5.2.1 pp. 7; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/d20_tests_saving_throws.md
  dnd2024:rules/d20_tests_difficulty_class_2:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_difficulty_class_2
        implementation: dnd2024:rule_d20_tests_difficulty_class_2
        parameters:
          save_dc_source_priority:
          - effect
          - gm_choice
          spell_dc:
            op: add
            args:
            - 8
            - variable: spellcasting_ability_modifier
            - variable: proficiency_bonus
          monster_ability_dc: stat_block_value
    contract:
      id: dnd2024:rule_d20_tests_difficulty_class_2
      inputs:
        actor: participant
        context: resolution
        proficiency_bonus: number
        spellcasting_ability_modifier: number
        selection:
          type: choice
          required: false
      outputs:
      - save_resolution
      events: []
      capabilities:
      - saving_throws
    requires:
    - dnd2024:rule_d20_tests_difficulty_class_2
    citations:
    - SRD 5.2.1 pp. 7; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/d20_tests_difficulty_class_2.md
  dnd2024:rules/d20_tests_proficiency_bonus_3:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_proficiency_bonus_3
        implementation: dnd2024:rule_d20_tests_proficiency_bonus_3
        parameters:
          add_proficiency_bonus_when:
            any:
            - proficient_with_used_weapon
            - spell_attack
    contract:
      id: dnd2024:rule_d20_tests_proficiency_bonus_3
      inputs:
        actor: participant
        context: resolution
      outputs:
      - attack_modifiers
      events: []
      capabilities:
      - attacks
    requires:
    - dnd2024:rule_d20_tests_proficiency_bonus_3
    citations:
    - SRD 5.2.1 pp. 7; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/d20_tests_proficiency_bonus_3.md
  dnd2024:rules/d20_tests_advantage_disadvantage:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_advantage_disadvantage
        implementation: dnd2024:rule_d20_tests_advantage_disadvantage
        parameters:
          apply_definitions:
          - key: dnd2024:glossary/advantage
          - key: dnd2024:glossary/disadvantage
          - key: dnd2024:glossary/heroic_inspiration
          circumstantial_advantage:
            actor: gm
            choices:
            - advantage
            - disadvantage
            - neither
          heroic_inspiration:
            maximum: 1
            reroll:
              die_count: 1
              timing: immediately_after_roll
              must_use_new_roll: true
              consume: true
            overflow:
              optional: true
              recipient: party_player_character_without_inspiration
            gm_award:
              optional: true
            human_daily_grant: true
    contract:
      id: dnd2024:rule_d20_tests_advantage_disadvantage
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - resolution
      events: []
      capabilities:
      - dice
      - heroic_inspiration
    requires:
    - dnd2024:rule_d20_tests_advantage_disadvantage
    citations:
    - SRD 5.2.1 pp. 7, 8; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/d20_tests_advantage_disadvantage.md
  dnd2024:rules/d20_tests_interactions_with_rerolls:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_interactions_with_rerolls
        implementation: dnd2024:rule_d20_tests_interactions_with_rerolls
        parameters:
          when:
          - advantage_or_disadvantage
          - reroll_or_replace_d20
          maximum_affected_dice: 1
          die_selection: roller_choice
    contract:
      id: dnd2024:rule_d20_tests_interactions_with_rerolls
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - dice_selection
      events: []
      capabilities:
      - dice
    requires:
    - dnd2024:rule_d20_tests_interactions_with_rerolls
    citations:
    - SRD 5.2.1 pp. 8; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/d20_tests_interactions_with_rerolls.md
  dnd2024:rules/proficiency_skill_proficiencies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/proficiency_skill_proficiencies
        implementation: dnd2024:rule_proficiency_skill_proficiencies
        parameters:
          skill_abilities:
            acrobatics: dexterity
            animal_handling: wisdom
            arcana: intelligence
            athletics: strength
            deception: charisma
            history: intelligence
            insight: wisdom
            intimidation: charisma
            investigation: intelligence
            medicine: wisdom
            nature: intelligence
            perception: wisdom
            performance: charisma
            persuasion: charisma
            religion: intelligence
            sleight_of_hand: dexterity
            stealth: dexterity
            survival: wisdom
          relevant_skill:
            actor: gm
            rules_suggest: true
          proficient:
            add: proficiency_bonus
          unproficient:
            check_permitted: true
            proficiency_bonus: 0
    contract:
      id: dnd2024:rule_proficiency_skill_proficiencies
      inputs:
        actor: participant
        context: resolution
      outputs:
      - check_modifiers
      events: []
      capabilities:
      - checks
      - proficiency
    requires:
    - dnd2024:rule_proficiency_skill_proficiencies
    citations:
    - SRD 5.2.1 pp. 8, 9; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/proficiency_skill_proficiencies.md
  dnd2024:rules/proficiency_saving_throw_proficiencies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/proficiency_saving_throw_proficiencies
        implementation: dnd2024:rule_proficiency_saving_throw_proficiencies
        parameters:
          saving_throw_bonus:
            base: ability_modifier
            proficient_add: proficiency_bonus
          starting_class_save_proficiencies_minimum: 2
          monster_proficiencies: stat_block
    contract:
      id: dnd2024:rule_proficiency_saving_throw_proficiencies
      inputs:
        actor: participant
        context: resolution
      outputs:
      - save_resolution
      events: []
      capabilities:
      - saving_throws
      - proficiency
    requires:
    - dnd2024:rule_proficiency_saving_throw_proficiencies
    citations:
    - SRD 5.2.1 pp. 9; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/proficiency_saving_throw_proficiencies.md
  dnd2024:rules/actions_one_thing_at_a_time:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/actions_one_thing_at_a_time
        implementation: dnd2024:rule_actions_one_thing_at_a_time
        parameters:
          maximum_simultaneous_actions: 1
          scope:
          - combat
          - social_interaction
          - exploration
    contract:
      id: dnd2024:rule_actions_one_thing_at_a_time
      inputs:
        actor: participant
        context: resolution
      outputs:
      - action_budget
      events: []
      capabilities:
      - actions
    requires:
    - dnd2024:rule_actions_one_thing_at_a_time
    citations:
    - SRD 5.2.1 pp. 10; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/actions_one_thing_at_a_time.md
  dnd2024:rules/social_interaction:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/social_interaction
        implementation: dnd2024:rule_social_interaction
        parameters:
          apply_definitions:
          - key: dnd2024:glossary/friendly_attitude
          - key: dnd2024:glossary/indifferent_attitude
          - key: dnd2024:glossary/hostile_attitude
          npc_controller: gm
          resolution_methods:
          - roleplay
          - ability_check
    contract:
      id: dnd2024:rule_social_interaction
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - social_interaction
    requires:
    - dnd2024:rule_social_interaction
    citations:
    - SRD 5.2.1 pp. 10; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/social_interaction.md
  dnd2024:rules/exploration:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 11; builtin/dnd2024-definitions/rules/exploration.md
  dnd2024:rules/exploration_obscured_areas:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_obscured_areas
        implementation: dnd2024:rule_exploration_obscured_areas
        parameters:
          lightly_obscured:
            sight_based_wisdom_perception: disadvantage
          heavily_obscured:
            opaque: true
            condition_when_attempting_to_see_inside: blinded
    contract:
      id: dnd2024:rule_exploration_obscured_areas
      inputs:
        actor: participant
        context: resolution
      outputs:
      - perception_modifiers
      events: []
      capabilities:
      - senses
    requires:
    - dnd2024:rule_exploration_obscured_areas
    citations:
    - SRD 5.2.1 pp. 11; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/exploration_obscured_areas.md
  dnd2024:rules/exploration_hiding:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_hiding
        implementation: dnd2024:rule_exploration_hiding
        parameters:
          apply_definitions:
          - key: dnd2024:glossary/hide_action
          circumstances_appropriate:
            actor: gm
            type: boolean
    contract:
      id: dnd2024:rule_exploration_hiding
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - stealth
    requires:
    - dnd2024:rule_exploration_hiding
    citations:
    - SRD 5.2.1 pp. 11; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/exploration_hiding.md
  dnd2024:rules/exploration_time_limited_object_interactions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_time_limited_object_interactions
        implementation: dnd2024:rule_exploration_time_limited_object_interactions
        parameters:
          free_object_interactions_per_turn: 1
          during:
          - movement
          - action
          additional_interaction_action: utilize
          when: time_limited
    contract:
      id: dnd2024:rule_exploration_time_limited_object_interactions
      inputs:
        actor: participant
        context: resolution
      outputs:
      - action_budget
      events: []
      capabilities:
      - actions
    requires:
    - dnd2024:rule_exploration_time_limited_object_interactions
    citations:
    - SRD 5.2.1 pp. 12; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/exploration_time_limited_object_interactions.md
  dnd2024:rules/exploration_breaking_objects:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_breaking_objects
        implementation: dnd2024:rule_exploration_breaking_objects
        parameters:
          apply_definitions:
          - key: dnd2024:glossary/breaking_objects
          fragile_nonmagical:
            action_cost: 1
            automatically_destroy: true
          marching_order:
            ordered_participants: true
            change_allowed: outside_combat
            determines:
            - trap_targets
            - hidden_enemy_detection
            - starting_distances
    contract:
      id: dnd2024:rule_exploration_breaking_objects
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - objects
      - travel
    requires:
    - dnd2024:rule_exploration_breaking_objects
    citations:
    - SRD 5.2.1 pp. 12; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/exploration_breaking_objects.md
  dnd2024:rules/exploration_travel_pace:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_travel_pace
        implementation: dnd2024:rule_exploration_travel_pace
        parameters:
          paces:
          - pace: fast
            feet_per_minute: 400
            miles_per_hour: 4
            miles_per_day: 30
            disadvantage:
            - wisdom_perception
            - wisdom_survival
            - dexterity_stealth
          - pace: normal
            feet_per_minute: 300
            miles_per_hour: 3
            miles_per_day: 24
            disadvantage:
            - dexterity_stealth
          - pace: slow
            feet_per_minute: 200
            miles_per_hour: 2
            miles_per_day: 18
            advantage:
            - wisdom_perception
            - wisdom_survival
          mounted:
            distance_multiplier: 2
            maximum_hours: 1
            recharge:
            - short_rest
            - long_rest
          pace_limit_source: terrain
    contract:
      id: dnd2024:rule_exploration_travel_pace
      inputs:
        actor: participant
        context: resolution
      outputs:
      - travel_rates
      events: []
      capabilities:
      - travel
    requires:
    - dnd2024:rule_exploration_travel_pace
    citations:
    - SRD 5.2.1 pp. 12; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/exploration_travel_pace.md
  dnd2024:rules/combat_the_order_of_combat:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_the_order_of_combat
        implementation: dnd2024:rule_combat_the_order_of_combat
        parameters:
          round_seconds: 6
          turns_per_round_per_participant: 1
          initial_order: initiative
          repeat_rounds_until: side_defeated
    contract:
      id: dnd2024:rule_combat_the_order_of_combat
      inputs:
        actor: participant
        context: resolution
      outputs:
      - turn_state
      events: []
      capabilities:
      - turns
    requires:
    - dnd2024:rule_combat_the_order_of_combat
    citations:
    - SRD 5.2.1 pp. 13; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/combat_the_order_of_combat.md
  dnd2024:rules/combat_your_turn:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_your_turn
        implementation: dnd2024:rule_combat_your_turn
        parameters:
          movement_maximum: speed
          actions: 1
          order: actor_choice
          may_forgo: true
          communication:
            brief_utterances_or_gestures_cost: 0
            extended_cost_actions: 1
            persuasion_action: influence
          object_interactions:
            free: 1
            during:
            - movement
            - action
            additional_action: utilize
            special_item_cost: description
            gm_require_utilize:
              when_any:
              - special_care
              - unusual_obstacle
          grid:
            square_feet: 5
            speed_squares:
              op: floor
              args:
              - op: divide
                args:
                - variable: speed
                - 5
            adjacent_entry_squares: 1
            diagonal_entry_squares: 1
            difficult_terrain_entry_squares: 2
            entry_requires_sufficient_movement: true
            diagonal_cross_solid_corner: false
            range: shortest_adjacent_square_path_excluding_origin_including_destination
            additional_costs_apply: true
    contract:
      id: dnd2024:rule_combat_your_turn
      inputs:
        actor: participant
        context: resolution
        speed: number
        selection:
          type: choice
          required: false
      outputs:
      - turn_state
      events: []
      capabilities:
      - turns
      - movement
      - actions
    requires:
    - dnd2024:rule_combat_your_turn
    citations:
    - SRD 5.2.1 pp. 13, 14; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/combat_your_turn.md
  dnd2024:rules/combat_difficult_terrain:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_difficult_terrain
        implementation: dnd2024:rule_combat_difficult_terrain
        parameters:
          movement_cost_extra_feet_per_foot: 1
          multiple_sources_stack: false
    contract:
      id: dnd2024:rule_combat_difficult_terrain
      inputs:
        actor: participant
        context: resolution
      outputs:
      - movement_budget
      events: []
      capabilities:
      - movement
    requires:
    - dnd2024:rule_combat_difficult_terrain
    citations:
    - SRD 5.2.1 pp. 14; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/combat_difficult_terrain.md
  dnd2024:rules/combat_creature_size:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_creature_size
        implementation: dnd2024:rule_combat_creature_size
        parameters:
          sizes:
          - size: tiny
            space_width_feet: 2.5
            occupied_squares: 0.25
          - size: small
            space_width_feet: 5
            occupied_squares: 1
          - size: medium
            space_width_feet: 5
            occupied_squares: 1
          - size: large
            space_width_feet: 10
            occupied_squares: 4
          - size: huge
            space_width_feet: 15
            occupied_squares: 9
          - size: gargantuan
            space_width_feet: 20
            occupied_squares: 16
          player_size_source: species
          monster_size_source: stat_block
    contract:
      id: dnd2024:rule_combat_creature_size
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spaces
      events: []
      capabilities:
      - geometry
    requires:
    - dnd2024:rule_combat_creature_size
    citations:
    - SRD 5.2.1 pp. 14; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/combat_creature_size.md
  dnd2024:rules/combat_cover:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_cover
        implementation: dnd2024:rule_combat_cover
        parameters:
          requires: effect_origin_opposite_side_of_cover
          stacking: highest_degree_only
          degrees:
          - degree: half
            minimum_covered_fraction: 0.5
            ac_bonus: 2
            dexterity_save_bonus: 2
            creature_can_supply: true
          - degree: three_quarters
            minimum_covered_fraction: 0.75
            ac_bonus: 5
            dexterity_save_bonus: 5
          - degree: total
            minimum_covered_fraction: 1
            direct_targeting: false
    contract:
      id: dnd2024:rule_combat_cover
      inputs:
        actor: participant
        context: resolution
      outputs:
      - cover_modifiers
      events: []
      capabilities:
      - cover
    requires:
    - dnd2024:rule_combat_cover
    citations:
    - SRD 5.2.1 pp. 15; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/combat_cover.md
  dnd2024:rules/combat_ranged_attacks_in_close_combat:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_ranged_attacks_in_close_combat
        implementation: dnd2024:rule_combat_ranged_attacks_in_close_combat
        parameters:
          attack_kind: ranged
          disadvantage_when:
            enemy_distance_feet_maximum: 5
            enemy_can_see_attacker: true
            enemy_incapacitated: false
          attack_sources:
          - weapon
          - spell
          - other
    contract:
      id: dnd2024:rule_combat_ranged_attacks_in_close_combat
      inputs:
        actor: participant
        context: resolution
      outputs:
      - attack_modifiers
      events: []
      capabilities:
      - attacks
    requires:
    - dnd2024:rule_combat_ranged_attacks_in_close_combat
    citations:
    - SRD 5.2.1 pp. 15; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/combat_ranged_attacks_in_close_combat.md
  dnd2024:rules/combat_opportunity_attacks:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_opportunity_attacks
        implementation: dnd2024:rule_combat_opportunity_attacks
        parameters:
          trigger:
            event: creature.leaves_reach
            target_visible: true
          cost: reaction
          attack_count: 1
          attack_options:
          - melee_weapon
          - unarmed_strike
          attack_timing: immediately_before_leaves_reach
          prevented_by:
          - disengage
          - teleportation
          - movement_without_target_movement_action_bonus_action_or_reaction
    contract:
      id: dnd2024:rule_combat_opportunity_attacks
      inputs:
        actor: participant
        context: resolution
      outputs:
      - attack_modifiers
      events:
      - creature.leaves_reach
      capabilities:
      - attacks
      - reactions
    requires:
    - dnd2024:rule_combat_opportunity_attacks
    citations:
    - SRD 5.2.1 pp. 15; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/combat_opportunity_attacks.md
  dnd2024:rules/combat_controlling_a_mount:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_controlling_a_mount
        implementation: dnd2024:rule_combat_controlling_a_mount
        parameters:
          controlled:
            requires: trained_to_accept_rider
            initiative: rider_initiative
            movement_turn: rider_turn
            controller: rider
            actions:
            - dash
            - disengage
            - dodge
            may_act_on_mounting_turn: true
          independent:
            initiative: unchanged
            controller: mount
    contract:
      id: dnd2024:rule_combat_controlling_a_mount
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - mounts
      - turns
    requires:
    - dnd2024:rule_combat_controlling_a_mount
    citations:
    - SRD 5.2.1 pp. 16; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/combat_controlling_a_mount.md
  dnd2024:rules/combat_impeded_weapons:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_impeded_weapons
        implementation: dnd2024:rule_combat_impeded_weapons
        parameters:
          environment: underwater
          melee_weapon_attack:
            disadvantage_when:
              swim_speed: false
              damage_type_not: piercing
          ranged_weapon_attack:
            beyond_normal_range: automatic_miss
            within_normal_range: disadvantage
    contract:
      id: dnd2024:rule_combat_impeded_weapons
      inputs:
        actor: participant
        context: resolution
      outputs:
      - attack_modifiers
      events: []
      capabilities:
      - attacks
    requires:
    - dnd2024:rule_combat_impeded_weapons
    citations:
    - SRD 5.2.1 pp. 16; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/combat_impeded_weapons.md
  dnd2024:rules/damage_and_healing_hit_points:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_hit_points
        implementation: dnd2024:rule_damage_and_healing_hit_points
        parameters:
          apply_definitions:
          - key: dnd2024:glossary/short_rest
          - key: dnd2024:glossary/long_rest
          hit_points:
            minimum: 0
            maximum: hit_point_maximum
            damage_operation: subtract
            loss_above_zero_impairs_capabilities: false
          bloodied:
            current_hit_points_lte_fraction_maximum: 0.5
            inherent_effect: false
          rest_duration_hours:
            short: 1
            long: 8
    contract:
      id: dnd2024:rule_damage_and_healing_hit_points
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - hit_points
      - rests
    requires:
    - dnd2024:rule_damage_and_healing_hit_points
    citations:
    - SRD 5.2.1 pp. 16; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/damage_and_healing_hit_points.md
  dnd2024:rules/damage_and_healing_saving_throws_and_damage:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 16; builtin/dnd2024-definitions/rules/damage_and_healing_saving_throws_and_damage.md
  dnd2024:rules/damage_and_healing_damage_types:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_damage_types
        implementation: dnd2024:rule_damage_and_healing_damage_types
        parameters:
          apply_definitions:
          - key: dnd2024:glossary/damage_types
          each_damage_instance_has_type: true
          inherent_type_rules: false
    contract:
      id: dnd2024:rule_damage_and_healing_damage_types
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - damage
    requires:
    - dnd2024:rule_damage_and_healing_damage_types
    citations:
    - SRD 5.2.1 pp. 16, 17; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/damage_and_healing_damage_types.md
  dnd2024:rules/damage_and_healing_order_of_application:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_order_of_application
        implementation: dnd2024:rule_damage_and_healing_order_of_application
        parameters:
          ordered_operations:
          - bonuses_penalties_multipliers
          - resistance
          - vulnerability
          resistance:
            divide: 2
            rounding: floor
          vulnerability:
            multiply: 2
          example:
            initial: 28
            flat_adjustment: -5
            after_adjustment: 23
            after_resistance: 11
            after_vulnerability: 22
    contract:
      id: dnd2024:rule_damage_and_healing_order_of_application
      inputs:
        actor: participant
        context: resolution
      outputs:
      - damage
      events: []
      capabilities:
      - damage
    requires:
    - dnd2024:rule_damage_and_healing_order_of_application
    citations:
    - SRD 5.2.1 pp. 17; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/damage_and_healing_order_of_application.md
  dnd2024:rules/damage_and_healing_dropping_to_0_hit_points:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_dropping_to_0_hit_points
        implementation: dnd2024:rule_damage_and_healing_dropping_to_0_hit_points
        parameters:
          apply_definitions:
          - key: dnd2024:glossary/dead
          - key: dnd2024:glossary/unconscious_condition
          trigger:
            hit_points: 0
          outcomes:
          - instant_death
          - unconscious
    contract:
      id: dnd2024:rule_damage_and_healing_dropping_to_0_hit_points
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - hit_points
      - death
    requires:
    - dnd2024:rule_damage_and_healing_dropping_to_0_hit_points
    citations:
    - SRD 5.2.1 pp. 17; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/damage_and_healing_dropping_to_0_hit_points.md
  dnd2024:rules/damage_and_healing_falling_unconscious:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_falling_unconscious
        implementation: dnd2024:rule_damage_and_healing_falling_unconscious
        parameters:
          apply_definitions:
          - key: dnd2024:glossary/unconscious_condition
          - key: dnd2024:glossary/death_saving_throw
          when:
            hit_points: 0
            instant_death: false
          unconscious_until: any_hit_points_regained
    contract:
      id: dnd2024:rule_damage_and_healing_falling_unconscious
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - conditions
      - death_saves
    requires:
    - dnd2024:rule_damage_and_healing_falling_unconscious
    citations:
    - SRD 5.2.1 pp. 17; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/damage_and_healing_falling_unconscious.md
  dnd2024:rules/damage_and_healing_temporary_hit_points:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_temporary_hit_points
        implementation: dnd2024:rule_damage_and_healing_temporary_hit_points
        parameters:
          apply_definitions:
          - key: dnd2024:glossary/temporary_hit_points
    contract:
      id: dnd2024:rule_damage_and_healing_temporary_hit_points
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - hit_points
    requires:
    - dnd2024:rule_damage_and_healing_temporary_hit_points
    citations:
    - SRD 5.2.1 pp. 18; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/damage_and_healing_temporary_hit_points.md
  dnd2024:rules/damage_and_healing_they_don_t_stack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_they_don_t_stack
        implementation: dnd2024:rule_damage_and_healing_they_don_t_stack
        parameters:
          temporary_hit_points_stacking: false
          new_grant_choice:
          - retain_existing
          - replace_with_new
    contract:
      id: dnd2024:rule_damage_and_healing_they_don_t_stack
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - hit_points
      events: []
      capabilities:
      - hit_points
    requires:
    - dnd2024:rule_damage_and_healing_they_don_t_stack
    citations:
    - SRD 5.2.1 pp. 18; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/damage_and_healing_they_don_t_stack.md
  dnd2024:rules/creation:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 19; builtin/dnd2024-definitions/rules/creation.md
  dnd2024:rules/create_your_character_step_1_choose_class:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_step_1_choose_class
        implementation: dnd2024:rule_create_your_character_step_1_choose_class
        parameters:
          choose:
            count: 1
            options:
            - key: dnd2024:classes/barbarian
            - key: dnd2024:classes/bard
            - key: dnd2024:classes/cleric
            - key: dnd2024:classes/druid
            - key: dnd2024:classes/fighter
            - key: dnd2024:classes/monk
            - key: dnd2024:classes/paladin
            - key: dnd2024:classes/ranger
            - key: dnd2024:classes/rogue
            - key: dnd2024:classes/sorcerer
            - key: dnd2024:classes/warlock
            - key: dnd2024:classes/wizard
          class_primary_abilities:
            barbarian:
            - strength
            bard:
            - charisma
            cleric:
            - wisdom
            druid:
            - wisdom
            fighter:
              any:
              - strength
              - dexterity
            monk:
            - dexterity
            - wisdom
            paladin:
            - strength
            - charisma
            ranger:
            - dexterity
            - wisdom
            rogue:
            - dexterity
            sorcerer:
            - charisma
            warlock:
            - charisma
            wizard:
            - intelligence
    contract:
      id: dnd2024:rule_create_your_character_step_1_choose_class
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rule_create_your_character_step_1_choose_class
    citations:
    - SRD 5.2.1 pp. 19; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/create_your_character_step_1_choose_class.md
  dnd2024:rules/create_your_character_step_2_character_origin:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_step_2_character_origin
        implementation: dnd2024:rule_create_your_character_step_2_character_origin
        parameters:
          choices:
            background: 1
            species: 1
            languages: 2
    contract:
      id: dnd2024:rule_create_your_character_step_2_character_origin
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rule_create_your_character_step_2_character_origin
    citations:
    - SRD 5.2.1 pp. 19; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/create_your_character_step_2_character_origin.md
  dnd2024:rules/create_your_character_choose_a_species:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_choose_a_species
        implementation: dnd2024:rule_create_your_character_choose_a_species
        parameters:
          choose:
            count: 1
            options:
            - key: dnd2024:species/dragonborn
            - key: dnd2024:species/dwarf
            - key: dnd2024:species/elf
            - key: dnd2024:species/gnome
            - key: dnd2024:species/goliath
            - key: dnd2024:species/halfling
            - key: dnd2024:species/human
            - key: dnd2024:species/orc
            - key: dnd2024:species/tiefling
          inherit:
          - traits
          - size
          - speed
    contract:
      id: dnd2024:rule_create_your_character_choose_a_species
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rule_create_your_character_choose_a_species
    citations:
    - SRD 5.2.1 pp. 20; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/create_your_character_choose_a_species.md
  dnd2024:rules/create_your_character_step_3_ability_scores:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_step_3_ability_scores
        implementation: dnd2024:rule_create_your_character_step_3_ability_scores
        parameters:
          generate_count: 6
          assign:
            abilities:
            - strength
            - dexterity
            - constitution
            - intelligence
            - wisdom
            - charisma
            each_number_once: true
    contract:
      id: dnd2024:rule_create_your_character_step_3_ability_scores
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rule_create_your_character_step_3_ability_scores
    citations:
    - SRD 5.2.1 pp. 20; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/create_your_character_step_3_ability_scores.md
  dnd2024:rules/create_your_character_adjust_ability_scores:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_adjust_ability_scores
        implementation: dnd2024:rule_create_your_character_adjust_ability_scores
        parameters:
          ability_choices: background_three_abilities
          options:
          - increases:
            - 2
            - 1
            distinct_abilities: true
          - increases:
            - 1
            - 1
            - 1
            distinct_abilities: true
          score_maximum: 20
          timing: after_base_assignment
    contract:
      id: dnd2024:rule_create_your_character_adjust_ability_scores
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rule_create_your_character_adjust_ability_scores
    citations:
    - SRD 5.2.1 pp. 21; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/create_your_character_adjust_ability_scores.md
  dnd2024:rules/create_your_character_the_nine_alignments:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_the_nine_alignments
        implementation: dnd2024:rule_create_your_character_the_nine_alignments
        parameters:
          alignment_axes:
            morality:
            - good
            - neutral
            - evil
            order:
            - lawful
            - neutral
            - chaotic
          options:
          - lawful_good
          - neutral_good
          - chaotic_good
          - lawful_neutral
          - neutral
          - chaotic_neutral
          - lawful_evil
          - neutral_evil
          - chaotic_evil
          rational_thought_absent_default: unaligned
          behavior_binding: false
    contract:
      id: dnd2024:rule_create_your_character_the_nine_alignments
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rule_create_your_character_the_nine_alignments
    citations:
    - SRD 5.2.1 pp. 21, 22; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/create_your_character_the_nine_alignments.md
  dnd2024:rules/create_your_character_fill_in_numbers:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_fill_in_numbers
        implementation: dnd2024:rule_create_your_character_fill_in_numbers
        parameters:
          saving_throw_and_skill_modifier:
            op: add
            args:
            - variable: ability_modifier
            - if: proficient
              then:
                variable: proficiency_bonus
              else: 0
          passive_perception:
            op: add
            args:
            - 10
            - variable: total_wisdom_perception_modifier
          level_one_hp:
            base_by_class:
              barbarian: 12
              fighter: 10
              paladin: 10
              ranger: 10
              bard: 8
              cleric: 8
              druid: 8
              monk: 8
              rogue: 8
              warlock: 8
              sorcerer: 6
              wizard: 6
            add: constitution_modifier
          level_one_hit_dice: 1
          hit_die_type_source: class
          initiative: dexterity_modifier
          unarmored_unshielded_ac:
            op: add
            args:
            - 10
            - variable: dexterity_modifier
          armor_and_feature_ac_override: true
          proficient_weapon_attack:
            melee:
              op: add
              args:
              - variable: strength_modifier
              - variable: proficiency_bonus
            ranged:
              op: add
              args:
              - variable: dexterity_modifier
              - variable: proficiency_bonus
            property_override: true
          weapon_damage_add: attack_ability_modifier
          spell_save_dc:
            op: add
            args:
            - 8
            - variable: spellcasting_ability_modifier
            - variable: proficiency_bonus
          spell_attack:
            op: add
            args:
            - variable: spellcasting_ability_modifier
            - variable: proficiency_bonus
          spellcasting_ability_source: granting_feature
          spell_slots_cantrips_prepared_limits: class_feature_table
    contract:
      id: dnd2024:rule_create_your_character_fill_in_numbers
      inputs:
        actor: participant
        context: resolution
        ability_modifier: number
        dexterity_modifier: number
        proficiency_bonus: number
        spellcasting_ability_modifier: number
        strength_modifier: number
        total_wisdom_perception_modifier: number
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
      - statistics
    requires:
    - dnd2024:rule_create_your_character_fill_in_numbers
    citations:
    - SRD 5.2.1 pp. 22, 23; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/create_your_character_fill_in_numbers.md
  dnd2024:rules/level_advancement_tiers_of_play:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 23, 24; builtin/dnd2024-definitions/rules/level_advancement_tiers_of_play.md
  dnd2024:rules/level_advancement_tier_3_levels_11_16:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 24; builtin/dnd2024-definitions/rules/level_advancement_tier_3_levels_11_16.md
  dnd2024:rules/starting_at_higher_levels_creating_your_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/starting_at_higher_levels_creating_your_character
        implementation: dnd2024:rule_starting_at_higher_levels_creating_your_character
        parameters:
          normal_character_creation_and_advancement: true
          starting_xp: minimum_for_starting_level
          normal_starting_equipment: true
          starting_equipment_table:
          - levels:
            - 2
            - 4
            base_gp: 0
            random_gp: 0
            magic_items:
              common: 1
          - levels:
            - 5
            - 10
            base_gp: 500
            random_gp:
              op: multiply
              args:
              - dice:
                  count: 1
                  sides: 10
              - 25
            magic_items:
              common: 1
              uncommon: 1
          - levels:
            - 11
            - 16
            base_gp: 5000
            random_gp:
              op: multiply
              args:
              - dice:
                  count: 1
                  sides: 10
              - 250
            magic_items:
              common: 2
              uncommon: 3
              rare: 1
          - levels:
            - 17
            - 20
            base_gp: 20000
            random_gp:
              op: multiply
              args:
              - dice:
                  count: 1
                  sides: 10
              - 250
            magic_items:
              common: 2
              uncommon: 4
              rare: 3
              very_rare: 1
          level_twenty_bonus_feats:
            gm_option: true
            xp_baseline: 355000
            xp_per_feat: 30000
            choice: any_qualified_feat
            level_for_prerequisites: 20
    contract:
      id: dnd2024:rule_starting_at_higher_levels_creating_your_character
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
      - advancement
    requires:
    - dnd2024:rule_starting_at_higher_levels_creating_your_character
    citations:
    - SRD 5.2.1 pp. 24; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/starting_at_higher_levels_creating_your_character.md
  dnd2024:rules/multiclassing_prerequisites:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/multiclassing_prerequisites
        implementation: dnd2024:rule_multiclassing_prerequisites
        parameters:
          minimum_primary_ability_score: 13
          classes_to_check: new_and_all_current
          primary_abilities:
            source:
              key: dnd2024:rules/create_your_character_step_1_choose_class
          interleaved_starting_equipment_table:
            source:
              key: dnd2024:rules/starting_at_higher_levels_creating_your_character
            field: starting_equipment_table
    contract:
      id: dnd2024:rule_multiclassing_prerequisites
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - multiclassing
    requires:
    - dnd2024:rule_multiclassing_prerequisites
    citations:
    - SRD 5.2.1 pp. 24, 25; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/multiclassing_prerequisites.md
  dnd2024:rules/multiclassing_proficiency_bonus:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/multiclassing_proficiency_bonus
        implementation: dnd2024:rule_multiclassing_proficiency_bonus
        parameters:
          proficiency_bonus_level_source: sum_all_class_levels
          proficiency_bonus:
            op: add
            args:
            - 2
            - op: floor
              args:
              - op: divide
                args:
                - op: subtract
                  args:
                  - variable: total_level
                  - 1
                - 4
    contract:
      id: dnd2024:rule_multiclassing_proficiency_bonus
      inputs:
        actor: participant
        context: resolution
        total_level: number
      outputs:
      - character_changes
      events: []
      capabilities:
      - multiclassing
      - statistics
    requires:
    - dnd2024:rule_multiclassing_proficiency_bonus
    citations:
    - SRD 5.2.1 pp. 25; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/multiclassing_proficiency_bonus.md
  dnd2024:rules/multiclassing_armor_class:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/multiclassing_armor_class
        implementation: dnd2024:rule_multiclassing_armor_class
        parameters:
          ac_calculations_active_maximum: 1
          selection: player_choice
    contract:
      id: dnd2024:rule_multiclassing_armor_class
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - statistics
      events: []
      capabilities:
      - statistics
    requires:
    - dnd2024:rule_multiclassing_armor_class
    citations:
    - SRD 5.2.1 pp. 25; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/multiclassing_armor_class.md
  dnd2024:rules/trinkets:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/trinkets
        implementation: dnd2024:rule_trinkets
        parameters:
          creation_roll_optional: true
          rolls_per_character_creation: 1
          roll:
            dice:
              count: 1
              sides: 100
          item_size: tiny
          results:
          - roll: 1
            appearance: A mummified goblin hand
          - roll: 2
            appearance: A crystal that faintly glows in moonlight
            properties:
              glow: faint
              trigger: moonlight
          - roll: 3
            appearance: A gold coin minted in an unknown land
          - roll: 4
            appearance: A diary written in a language you don’t know
            properties:
              language_known_to_owner: false
          - roll: 5
            appearance: A brass ring that never tarnishes
            properties:
              tarnishes: false
          - roll: 6
            appearance: An old chess piece made from glass
          - roll: 7
            appearance: A pair of knucklebone dice, each with a skull symbol on the side that would normally
              show six pips
            properties:
              count: 2
              die_six_face: skull
          - roll: 8
            appearance: A small idol depicting a nightmarish creature that gives you unsettling dreams
              when you sleep near it
            properties:
              sleep_nearby_dreams: unsettling
          - roll: 9
            appearance: A lock of someone’s hair
          - roll: 10
            appearance: The deed for a parcel of land in a realm unknown to you
          - roll: 11
            appearance: A 1-ounce block made from an unknown material
            properties:
              weight_ounces: 1
          - roll: 12
            appearance: A small cloth doll skewered with needles
          - roll: 13
            appearance: A tooth from an unknown beast
          - roll: 14
            appearance: An enormous scale, perhaps from a dragon
          - roll: 15
            appearance: A bright-green feather
          - roll: 16
            appearance: An old divination card bearing your likeness
          - roll: 17
            appearance: A glass orb filled with moving smoke
          - roll: 18
            appearance: A 1-pound egg with a bright-red shell
            properties:
              weight_pounds: 1
          - roll: 19
            appearance: A pipe that blows bubbles
            properties:
              emission: bubbles
          - roll: 20
            appearance: A glass jar containing a bit of flesh floating in pickling fluid
          - roll: 21
            appearance: A gnome-crafted music box that plays a song you dimly remember from your childhood
            properties:
              plays_music: true
          - roll: 22
            appearance: A wooden statuette of a smug halfling
          - roll: 23
            appearance: A brass orb etched with strange runes
          - roll: 24
            appearance: A multicolored stone disk
          - roll: 25
            appearance: A silver icon of a raven
          - roll: 26
            appearance: A bag containing forty-seven teeth, one of which is rotten
            properties:
              teeth_count: 47
              rotten_teeth: 1
          - roll: 27
            appearance: A shard of obsidian that always feels warm to the touch
            properties:
              touch_temperature: warm
          - roll: 28
            appearance: A dragon’s talon strung on a leather necklace
          - roll: 29
            appearance: A pair of old socks
            properties:
              count: 2
          - roll: 30
            appearance: A blank book whose pages refuse to hold ink, chalk, graphite, or any other marking
            properties:
              marking_retention: false
              media:
              - ink
              - chalk
              - graphite
              - other
          - roll: 31
            appearance: A silver badge that is a five-pointed star
            properties:
              star_points: 5
          - roll: 32
            appearance: A knife that belonged to a relative
          - roll: 33
            appearance: A glass vial filled with nail clippings
          - roll: 34
            appearance: A rectangular metal device with two tiny metal cups on one end that throws sparks
              when wet
            properties:
              metal_cups: 2
              wet_emits_sparks: true
          - roll: 35
            appearance: A white, sequined glove sized for a human
          - roll: 36
            appearance: A vest with one hundred tiny pockets
            properties:
              pockets: 100
          - roll: 37
            appearance: A weightless stone
            properties:
              weight_pounds: 0
          - roll: 38
            appearance: A sketch of a goblin
          - roll: 39
            appearance: An empty glass vial that smells of perfume
          - roll: 40
            appearance: A gemstone that looks like a lump of coal when examined by anyone but you
            properties:
              owner_appearance: gemstone
              others_appearance: coal
          - roll: 41
            appearance: A scrap of cloth from an old banner
          - roll: 42
            appearance: A rank insignia from a lost legionnaire
          - roll: 43
            appearance: A silver bell without a clapper
            properties:
              clapper: false
          - roll: 44
            appearance: A mechanical canary inside a lamp
          - roll: 45
            appearance: A miniature chest carved to look like it has numerous feet on the bottom
          - roll: 46
            appearance: A dead sprite inside a clear glass bottle
          - roll: 47
            appearance: A metal can that has no opening but sounds as if it is filled with liquid, sand,
              spiders, or broken glass (your choice)
            properties:
              opening: false
              sound_choice:
              - liquid
              - sand
              - spiders
              - broken_glass
          - roll: 48
            appearance: A glass orb filled with water, in which swims a clockwork goldfish
          - roll: 49
            appearance: A silver spoon with an M engraved on the handle
          - roll: 50
            appearance: A whistle made from gold-colored wood
          - roll: 51
            appearance: A dead scarab beetle the size of your hand
          - roll: 52
            appearance: Two toy soldiers, one missing a head
            properties:
              soldiers: 2
              missing_heads: 1
          - roll: 53
            appearance: A small box filled with different-sized buttons
          - roll: 54
            appearance: A candle that can’t be lit
            properties:
              can_be_lit: false
          - roll: 55
            appearance: A miniature cage with no door
            properties:
              door: false
          - roll: 56
            appearance: An old key
          - roll: 57
            appearance: An indecipherable treasure map
          - roll: 58
            appearance: A hilt from a broken sword
          - roll: 59
            appearance: A rabbit’s foot
          - roll: 60
            appearance: A glass eye
          - roll: 61
            appearance: A cameo of a hideous person
          - roll: 62
            appearance: A silver skull the size of a coin
          - roll: 63
            appearance: An alabaster mask
          - roll: 64
            appearance: A cone of sticky black incense that stinks
          - roll: 65
            appearance: A nightcap that gives you pleasant dreams when you wear it
            properties:
              while_worn_dreams: pleasant
          - roll: 66
            appearance: A single caltrop made from bone
          - roll: 67
            appearance: A gold monocle frame without the lens
            properties:
              lens: false
          - roll: 68
            appearance: A 1-inch cube, each side a different color
            properties:
              cube_edge_inches: 1
              side_colors_distinct: true
          - roll: 69
            appearance: A crystal doorknob
          - roll: 70
            appearance: A packet filled with pink dust
          - roll: 71
            appearance: A fragment of a beautiful song, written as musical notes on two pieces of parchment
            properties:
              parchment_pieces: 2
          - roll: 72
            appearance: A silver teardrop earring containing a real teardrop
          - roll: 73
            appearance: An eggshell painted with scenes of misery in disturbing detail
          - roll: 74
            appearance: A fan that, when unfolded, shows a sleepy cat
          - roll: 75
            appearance: A set of bone pipes
          - roll: 76
            appearance: A four-leaf clover pressed inside a book discussing manners and etiquette
            properties:
              clover_leaves: 4
          - roll: 77
            appearance: A sheet of parchment upon which is drawn a mechanical contraption
          - roll: 78
            appearance: An ornate scabbard that fits no blade you have found
          - roll: 79
            appearance: An invitation to a party where a murder happened
          - roll: 80
            appearance: A bronze pentacle with an etching of a rat’s head in its center
          - roll: 81
            appearance: A purple handkerchief embroidered with the name of an archmage
          - roll: 82
            appearance: Half a floor plan for a temple, a castle, or another structure
            properties:
              plan_fraction: 0.5
              structure_choice:
              - temple
              - castle
              - other_structure
          - roll: 83
            appearance: A bit of folded cloth that, when unfolded, turns into a stylish cap
            properties:
              unfolding_result: stylish_cap
          - roll: 84
            appearance: A receipt of deposit at a bank in a far-off city
          - roll: 85
            appearance: A diary with seven missing pages
            properties:
              missing_pages: 7
          - roll: 86
            appearance: An empty silver snuffbox bearing the inscription “dreams” on its lid
          - roll: 87
            appearance: An iron holy symbol devoted to an unknown god
          - roll: 88
            appearance: A book about a legendary hero’s rise and fall, with the last chapter missing
            properties:
              final_chapter_present: false
          - roll: 89
            appearance: A vial of dragon blood
          - roll: 90
            appearance: An ancient arrow of elven design
          - roll: 91
            appearance: A needle that never bends
            properties:
              bends: false
          - roll: 92
            appearance: An ornate brooch of dwarven design
          - roll: 93
            appearance: An empty wine bottle bearing a pretty label that says, “The Wizard of Wines Winery,
              Red Dragon Crush, 331422-W”
          - roll: 94
            appearance: A mosaic tile with a multicolored, glazed surface
          - roll: 95
            appearance: A petrified mouse
          - roll: 96
            appearance: A black pirate flag adorned with a dragon’s skull and crossbones
          - roll: 97
            appearance: A tiny mechanical crab or spider that moves about when it’s not being observed
            properties:
              shape_choice:
              - crab
              - spider
              moves_when: unobserved
          - roll: 98
            appearance: A glass jar containing lard with a label that reads, “Griffon Grease”
          - roll: 99
            appearance: A wooden box with a ceramic bottom that holds a living worm with a head on each
              end of its body
            properties:
              living_worm_heads: 2
          - roll: 100
            appearance: A metal urn containing the ashes of a hero
          gm_table_use:
          - dungeon_contents
          - creature_pockets
    contract:
      id: dnd2024:rule_trinkets
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
      - random_tables
    requires:
    - dnd2024:rule_trinkets
    citations:
    - SRD 5.2.1 pp. 26, 27; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/trinkets.md
  dnd2024:rules/barbarian_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/barbarian_as_a_level_1_character
        implementation: dnd2024:rule_barbarian_as_a_level_1_character
        parameters:
          class:
            key: dnd2024:classes/barbarian
          entry_mode: level_one_character
          grant_all_core_traits: true
          grant_features_at_class_level: 1
          embedded_feature_progression_source:
            key: dnd2024:tables/barbarian_progression
          level_one_features:
          - key: dnd2024:class_features/barbarian_level_1_rage
          - key: dnd2024:class_features/barbarian_level_1_unarmored_defense
          - key: dnd2024:class_features/barbarian_level_1_weapon_mastery
    contract:
      id: dnd2024:rule_barbarian_as_a_level_1_character
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
      - classes
    requires:
    - dnd2024:rule_barbarian_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 28; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/barbarian_as_a_level_1_character.md
  dnd2024:rules/barbarian_barbarian_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 30; builtin/dnd2024-definitions/rules/barbarian_barbarian_subclass.md
  dnd2024:rules/bard_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/bard_as_a_multiclass_character
        implementation: dnd2024:rule_bard_as_a_multiclass_character
        parameters:
          class:
            key: dnd2024:classes/bard
          entry_mode: multiclass
          hit_die_sides: 8
          armor_training:
          - light
          skill_choice:
            count: 1
            options: all
          musical_instrument_choice_count: 1
          weapon_proficiencies: []
          grant_features_at_class_level: 1
          spell_slots:
            key: dnd2024:rules/multiclassing_spellcasting
          level_one_features:
          - key: dnd2024:class_features/bard_level_1_bardic_inspiration
          - key: dnd2024:class_features/bard_level_1_spellcasting
    contract:
      id: dnd2024:rule_bard_as_a_multiclass_character
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - character_changes
      events: []
      capabilities:
      - multiclassing
      - classes
    requires:
    - dnd2024:rule_bard_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 31; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/bard_as_a_multiclass_character.md
  dnd2024:rules/bard_bard_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 35; builtin/dnd2024-definitions/rules/bard_bard_subclass.md
  dnd2024:rules/cleric_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/cleric_as_a_multiclass_character
        implementation: dnd2024:rule_cleric_as_a_multiclass_character
        parameters:
          class:
            key: dnd2024:classes/cleric
          entry_mode: multiclass
          hit_die_sides: 8
          armor_training:
          - light
          - medium
          - shield
          skill_choice:
            count: 0
            options: null
          musical_instrument_choice_count: 0
          weapon_proficiencies: []
          grant_features_at_class_level: 1
          spell_slots:
            key: dnd2024:rules/multiclassing_spellcasting
          level_one_features:
          - key: dnd2024:class_features/cleric_level_1_divine_order
          - key: dnd2024:class_features/cleric_level_1_spellcasting
    contract:
      id: dnd2024:rule_cleric_as_a_multiclass_character
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - character_changes
      events: []
      capabilities:
      - multiclassing
      - classes
    requires:
    - dnd2024:rule_cleric_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 36; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/cleric_as_a_multiclass_character.md
  dnd2024:rules/druid_becoming_a_druid:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 41; builtin/dnd2024-definitions/rules/druid_becoming_a_druid.md
  dnd2024:rules/druid_druid_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/druid_druid_class_features
        implementation: dnd2024:rule_druid_druid_class_features
        parameters:
          class:
            key: dnd2024:classes/druid
          feature_acquisition: on_reaching_feature_class_level
          level_source: class_level
          progression_source:
            key: dnd2024:tables/druid_progression
    contract:
      id: dnd2024:rule_druid_druid_class_features
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - classes
      - advancement
    requires:
    - dnd2024:rule_druid_druid_class_features
    citations:
    - SRD 5.2.1 pp. 41; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/druid_druid_class_features.md
  dnd2024:rules/fighter_becoming_a_fighter:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 47; builtin/dnd2024-definitions/rules/fighter_becoming_a_fighter.md
  dnd2024:rules/fighter_fighter_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/fighter_fighter_class_features
        implementation: dnd2024:rule_fighter_fighter_class_features
        parameters:
          class:
            key: dnd2024:classes/fighter
          feature_acquisition: on_reaching_feature_class_level
          level_source: class_level
          progression_source:
            key: dnd2024:tables/fighter_progression
    contract:
      id: dnd2024:rule_fighter_fighter_class_features
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - classes
      - advancement
    requires:
    - dnd2024:rule_fighter_fighter_class_features
    citations:
    - SRD 5.2.1 pp. 47; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/fighter_fighter_class_features.md
  dnd2024:rules/monk_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/monk_as_a_multiclass_character
        implementation: dnd2024:rule_monk_as_a_multiclass_character
        parameters:
          class:
            key: dnd2024:classes/monk
          entry_mode: multiclass
          hit_die_sides: 8
          armor_training: []
          skill_choice:
            count: 0
            options: null
          musical_instrument_choice_count: 0
          weapon_proficiencies: []
          grant_features_at_class_level: 1
          spell_slots: not_applicable
          level_one_features:
          - key: dnd2024:class_features/monk_level_1_martial_arts
          - key: dnd2024:class_features/monk_level_1_unarmored_defense
    contract:
      id: dnd2024:rule_monk_as_a_multiclass_character
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - character_changes
      events: []
      capabilities:
      - multiclassing
      - classes
    requires:
    - dnd2024:rule_monk_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 49; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/monk_as_a_multiclass_character.md
  dnd2024:rules/paladin_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/paladin_as_a_level_1_character
        implementation: dnd2024:rule_paladin_as_a_level_1_character
        parameters:
          class:
            key: dnd2024:classes/paladin
          entry_mode: level_one_character
          grant_all_core_traits: true
          grant_features_at_class_level: 1
          embedded_feature_progression_source:
            key: dnd2024:tables/paladin_progression
          level_one_features:
          - key: dnd2024:class_features/paladin_level_1_lay_on_hands
          - key: dnd2024:class_features/paladin_level_1_spellcasting
          - key: dnd2024:class_features/paladin_level_1_weapon_mastery
    contract:
      id: dnd2024:rule_paladin_as_a_level_1_character
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
      - classes
    requires:
    - dnd2024:rule_paladin_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 53; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/paladin_as_a_level_1_character.md
  dnd2024:rules/paladin_paladin_spell_list:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/paladin_paladin_spell_list
        implementation: dnd2024:rule_paladin_paladin_spell_list
        parameters:
          class:
            key: dnd2024:classes/paladin
          spell_list:
            1:
            - key: dnd2024:spells/bless
            - key: dnd2024:spells/command
            - key: dnd2024:spells/cure_wounds
            - key: dnd2024:spells/detect_evil_and_good
            - key: dnd2024:spells/detect_magic
            - key: dnd2024:spells/detect_poison_and_disease
            - key: dnd2024:spells/divine_favor
            - key: dnd2024:spells/divine_smite
            - key: dnd2024:spells/heroism
            - key: dnd2024:spells/protection_from_evil_and_good
            - key: dnd2024:spells/purify_food_and_drink
            - key: dnd2024:spells/searing_smite
            - key: dnd2024:spells/shield_of_faith
            2:
            - key: dnd2024:spells/aid
            - key: dnd2024:spells/find_steed
            - key: dnd2024:spells/gentle_repose
            - key: dnd2024:spells/lesser_restoration
            - key: dnd2024:spells/locate_object
            - key: dnd2024:spells/magic_weapon
            - key: dnd2024:spells/prayer_of_healing
            - key: dnd2024:spells/protection_from_poison
            - key: dnd2024:spells/shining_smite
            - key: dnd2024:spells/warding_bond
            - key: dnd2024:spells/zone_of_truth
            3:
            - key: dnd2024:spells/create_food_and_water
            - key: dnd2024:spells/daylight
            - key: dnd2024:spells/dispel_magic
            - key: dnd2024:spells/magic_circle
            - key: dnd2024:spells/remove_curse
            - key: dnd2024:spells/revivify
            4:
            - key: dnd2024:spells/aura_of_life
            - key: dnd2024:spells/banishment
            - key: dnd2024:spells/death_ward
            - key: dnd2024:spells/locate_creature
            5:
            - key: dnd2024:spells/dispel_evil_and_good
            - key: dnd2024:spells/geas
            - key: dnd2024:spells/greater_restoration
            - key: dnd2024:spells/raise_dead
          school_concentration_ritual_and_material_metadata: referenced_spell_definition
    contract:
      id: dnd2024:rule_paladin_paladin_spell_list
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_resolution
      events: []
      capabilities:
      - spellcasting
    requires:
    - dnd2024:rule_paladin_paladin_spell_list
    citations:
    - SRD 5.2.1 pp. 55, 56; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/paladin_paladin_spell_list.md
  dnd2024:rules/ranger_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/ranger_as_a_multiclass_character
        implementation: dnd2024:rule_ranger_as_a_multiclass_character
        parameters:
          class:
            key: dnd2024:classes/ranger
          entry_mode: multiclass
          hit_die_sides: 10
          armor_training:
          - light
          - medium
          - shield
          skill_choice:
            count: 1
            options:
            - animal_handling
            - athletics
            - insight
            - investigation
            - nature
            - perception
            - stealth
            - survival
          musical_instrument_choice_count: 0
          weapon_proficiencies:
          - martial
          grant_features_at_class_level: 1
          spell_slots:
            key: dnd2024:rules/multiclassing_spellcasting
          level_one_features:
          - key: dnd2024:class_features/ranger_level_1_favored_enemy
          - key: dnd2024:class_features/ranger_level_1_spellcasting
          - key: dnd2024:class_features/ranger_level_1_weapon_mastery
    contract:
      id: dnd2024:rule_ranger_as_a_multiclass_character
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - character_changes
      events: []
      capabilities:
      - multiclassing
      - classes
    requires:
    - dnd2024:rule_ranger_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 57; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/ranger_as_a_multiclass_character.md
  dnd2024:rules/rogue_becoming_a_rogue:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 61; builtin/dnd2024-definitions/rules/rogue_becoming_a_rogue.md
  dnd2024:rules/rogue_rogue_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/rogue_rogue_class_features
        implementation: dnd2024:rule_rogue_rogue_class_features
        parameters:
          class:
            key: dnd2024:classes/rogue
          feature_acquisition: on_reaching_feature_class_level
          level_source: class_level
          progression_source:
            key: dnd2024:tables/rogue_progression
    contract:
      id: dnd2024:rule_rogue_rogue_class_features
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - classes
      - advancement
    requires:
    - dnd2024:rule_rogue_rogue_class_features
    citations:
    - SRD 5.2.1 pp. 61; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/rogue_rogue_class_features.md
  dnd2024:rules/sorcerer_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_as_a_multiclass_character
        implementation: dnd2024:rule_sorcerer_as_a_multiclass_character
        parameters:
          class:
            key: dnd2024:classes/sorcerer
          entry_mode: multiclass
          hit_die_sides: 6
          armor_training: []
          skill_choice:
            count: 0
            options: null
          musical_instrument_choice_count: 0
          weapon_proficiencies: []
          grant_features_at_class_level: 1
          spell_slots:
            key: dnd2024:rules/multiclassing_spellcasting
          level_one_features:
          - key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
          - key: dnd2024:class_features/sorcerer_level_1_spellcasting
    contract:
      id: dnd2024:rule_sorcerer_as_a_multiclass_character
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - character_changes
      events: []
      capabilities:
      - multiclassing
      - classes
    requires:
    - dnd2024:rule_sorcerer_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 64; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/sorcerer_as_a_multiclass_character.md
  dnd2024:rules/sorcerer_careful_spell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_careful_spell
        implementation: dnd2024:rule_sorcerer_careful_spell
        parameters:
          cost_sorcery_points: 1
          trigger: casting_spell_forcing_other_creatures_to_save
          selected_creatures_maximum:
            op: max
            args:
            - 1
            - variable: charisma_modifier
          selected_creatures: spell_save_targets
          selected_effects:
            saving_throw: automatic_success
            half_damage_on_save_becomes: 0
    contract:
      id: dnd2024:rule_sorcerer_careful_spell
      inputs:
        actor: participant
        context: resolution
        charisma_modifier: number
      outputs:
      - spell_modifications
      events:
      - spell.cast_pending
      capabilities:
      - metamagic
      - saving_throws
    requires:
    - dnd2024:rule_sorcerer_careful_spell
    citations:
    - SRD 5.2.1 pp. 66, 67; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/sorcerer_careful_spell.md
  dnd2024:rules/sorcerer_extended_spell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_extended_spell
        implementation: dnd2024:rule_sorcerer_extended_spell
        parameters:
          cost_sorcery_points: 1
          minimum_spell_duration_minutes: 1
          duration_multiplier: 2
          duration_maximum_hours: 24
          concentration_saves_for_affected_spell: advantage
    contract:
      id: dnd2024:rule_sorcerer_extended_spell
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_modifications
      events:
      - spell.cast_pending
      capabilities:
      - metamagic
      - concentration
    requires:
    - dnd2024:rule_sorcerer_extended_spell
    citations:
    - SRD 5.2.1 pp. 67; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/sorcerer_extended_spell.md
  dnd2024:rules/sorcerer_seeking_spell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_seeking_spell
        implementation: dnd2024:rule_sorcerer_seeking_spell
        parameters:
          trigger: spell_attack_miss
          cost_sorcery_points: 1
          reroll_d20: 1
          must_use_new_roll: true
          compatible_with_other_metamagic: true
    contract:
      id: dnd2024:rule_sorcerer_seeking_spell
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_modifications
      events:
      - attack.missed
      capabilities:
      - metamagic
      - attacks
    requires:
    - dnd2024:rule_sorcerer_seeking_spell
    citations:
    - SRD 5.2.1 pp. 67; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/sorcerer_seeking_spell.md
  dnd2024:rules/sorcerer_twinned_spell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_twinned_spell
        implementation: dnd2024:rule_sorcerer_twinned_spell
        parameters:
          cost_sorcery_points: 1
          spell_eligibility:
            higher_slot_level_can_add_creature_target: true
          effective_spell_level_increase: 1
    contract:
      id: dnd2024:rule_sorcerer_twinned_spell
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_modifications
      events:
      - spell.cast_pending
      capabilities:
      - metamagic
      - spellcasting
    requires:
    - dnd2024:rule_sorcerer_twinned_spell
    citations:
    - SRD 5.2.1 pp. 67; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/sorcerer_twinned_spell.md
  dnd2024:rules/warlock_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_as_a_level_1_character
        implementation: dnd2024:rule_warlock_as_a_level_1_character
        parameters:
          class:
            key: dnd2024:classes/warlock
          entry_mode: level_one_character
          grant_all_core_traits: true
          grant_features_at_class_level: 1
          embedded_feature_progression_source:
            key: dnd2024:tables/warlock_progression
          level_one_features:
          - key: dnd2024:class_features/warlock_level_1_eldritch_invocations
          - key: dnd2024:class_features/warlock_level_1_pact_magic
    contract:
      id: dnd2024:rule_warlock_as_a_level_1_character
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
      - classes
    requires:
    - dnd2024:rule_warlock_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 70; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/warlock_as_a_level_1_character.md
  dnd2024:rules/warlock_eldritch_invocation_options:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 72; builtin/dnd2024-definitions/rules/warlock_eldritch_invocation_options.md
  dnd2024:rules/warlock_ascendant_step:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_ascendant_step
        implementation: dnd2024:rule_warlock_ascendant_step
        parameters:
          warlock_level_minimum: 5
          cast:
            spell:
              key: dnd2024:spells/levitate
            spell_slot_cost: 0
            target: self
            uses: unlimited
    contract:
      id: dnd2024:rule_warlock_ascendant_step
      inputs:
        actor: participant
        context: resolution
      outputs:
      - feature_effects
      events: []
      capabilities:
      - invocations
      - spellcasting
    requires:
    - dnd2024:rule_warlock_ascendant_step
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/warlock_ascendant_step.md
  dnd2024:rules/warlock_eldritch_mind:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_eldritch_mind
        implementation: dnd2024:rule_warlock_eldritch_mind
        parameters:
          advantage:
            save_ability: constitution
            purpose: maintain_concentration
    contract:
      id: dnd2024:rule_warlock_eldritch_mind
      inputs:
        actor: participant
        context: resolution
      outputs:
      - feature_effects
      events: []
      capabilities:
      - invocations
      - concentration
    requires:
    - dnd2024:rule_warlock_eldritch_mind
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/warlock_eldritch_mind.md
  dnd2024:rules/warlock_fiendish_vigor:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_fiendish_vigor
        implementation: dnd2024:rule_warlock_fiendish_vigor
        parameters:
          warlock_level_minimum: 2
          cast:
            spell:
              key: dnd2024:spells/false_life
            spell_slot_cost: 0
            target: self
            uses: unlimited
            temporary_hit_points_dice_result: maximum
    contract:
      id: dnd2024:rule_warlock_fiendish_vigor
      inputs:
        actor: participant
        context: resolution
      outputs:
      - feature_effects
      events: []
      capabilities:
      - invocations
      - spellcasting
    requires:
    - dnd2024:rule_warlock_fiendish_vigor
    citations:
    - SRD 5.2.1 pp. 73; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/warlock_fiendish_vigor.md
  dnd2024:rules/warlock_gift_of_the_protectors:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_gift_of_the_protectors
        implementation: dnd2024:rule_warlock_gift_of_the_protectors
        parameters:
          prerequisite:
            warlock_level_minimum: 9
            invocation: pact_of_the_tome
          book_page:
            created_when: book_of_shadows_conjured
            name_capacity:
              op: max
              args:
              - 1
              - variable: charisma_modifier
            add_name:
              action_cost: 1
              writer: named_creature
              warlock_permission: true
            erase_name:
              action: magic
              touch_page: true
          protect:
            trigger: named_creature_reduced_to_zero_hit_points
            exclude: killed_outright
            set_hit_points: 1
            shared_uses: 1
            recharge: warlock_long_rest
    contract:
      id: dnd2024:rule_warlock_gift_of_the_protectors
      inputs:
        actor: participant
        context: resolution
        charisma_modifier: number
      outputs:
      - feature_effects
      events:
      - hit_points.reduced_to_zero
      - rest.long_completed
      capabilities:
      - invocations
      - hit_points
      - resources
    requires:
    - dnd2024:rule_warlock_gift_of_the_protectors
    citations:
    - SRD 5.2.1 pp. 73; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/warlock_gift_of_the_protectors.md
  dnd2024:rules/warlock_lifedrinker:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_lifedrinker
        implementation: dnd2024:rule_warlock_lifedrinker
        parameters:
          prerequisite:
            warlock_level_minimum: 9
            invocation: pact_of_the_blade
          trigger: hit_creature_with_pact_weapon
          uses_per_turn: 1
          extra_damage:
            amount:
              dice:
                count: 1
                sides: 6
            type_choice:
            - necrotic
            - psychic
            - radiant
          optional_heal:
            spend_hit_dice: 1
            amount:
              op: max
              args:
              - 1
              - op: add
                args:
                - variable: spent_hit_die_roll
                - variable: constitution_modifier
    contract:
      id: dnd2024:rule_warlock_lifedrinker
      inputs:
        actor: participant
        context: resolution
        constitution_modifier: number
        spent_hit_die_roll: number
        selection:
          type: choice
          required: false
      outputs:
      - feature_effects
      events:
      - attack.hit
      capabilities:
      - invocations
      - damage
      - healing
    requires:
    - dnd2024:rule_warlock_lifedrinker
    citations:
    - SRD 5.2.1 pp. 73; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/warlock_lifedrinker.md
  dnd2024:rules/warlock_misty_visions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_misty_visions
        implementation: dnd2024:rule_warlock_misty_visions
        parameters:
          warlock_level_minimum: 2
          cast:
            spell:
              key: dnd2024:spells/silent_image
            spell_slot_cost: 0
            target: spell_definition
            uses: unlimited
    contract:
      id: dnd2024:rule_warlock_misty_visions
      inputs:
        actor: participant
        context: resolution
      outputs:
      - feature_effects
      events: []
      capabilities:
      - invocations
      - spellcasting
    requires:
    - dnd2024:rule_warlock_misty_visions
    citations:
    - SRD 5.2.1 pp. 74; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/warlock_misty_visions.md
  dnd2024:rules/warlock_pact_of_the_blade:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_pact_of_the_blade
        implementation: dnd2024:rule_warlock_pact_of_the_blade
        parameters:
          activation: bonus_action
          options:
            conjure:
              location: hand
              weapon_categories:
              - simple
              - martial
              weapon_kind: melee
              choice: warlock
            bond_existing:
              weapon_magic: true
              touch_required: true
              prohibited_if:
              - attuned_to_someone_else
              - bonded_to_another_warlock
          bond_benefits:
            weapon_proficiency: true
            spellcasting_focus: true
            optional_attack_and_damage_ability: charisma
            damage_type_choice:
            - normal
            - necrotic
            - psychic
            - radiant
          bond_ends_when:
          - activate_feature_again
          - warlock_dies
          - distance_feet_greater_than: 5
            continuous_minutes: 1
          conjured_weapon_disappears_when_bond_ends: true
    contract:
      id: dnd2024:rule_warlock_pact_of_the_blade
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - feature_effects
      events:
      - bonus_action.used
      - participant.died
      capabilities:
      - invocations
      - equipment
      - attacks
    requires:
    - dnd2024:rule_warlock_pact_of_the_blade
    citations:
    - SRD 5.2.1 pp. 74; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/warlock_pact_of_the_blade.md
  dnd2024:rules/warlock_repelling_blast:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_repelling_blast
        implementation: dnd2024:rule_warlock_repelling_blast
        parameters:
          prerequisite:
            warlock_level_minimum: 2
            known_warlock_cantrip_deals_damage_via_attack_roll: true
          cantrip_choice:
            known: true
            class: warlock
            requires_attack_roll: true
          hit_effect:
            target_maximum_size: large
            optional_push:
              feet_maximum: 10
              direction: straight_away_from_caster
          repeatable: true
          repeat_choice_must_differ: true
    contract:
      id: dnd2024:rule_warlock_repelling_blast
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - feature_effects
      events:
      - attack.hit
      capabilities:
      - invocations
      - movement
    requires:
    - dnd2024:rule_warlock_repelling_blast
    citations:
    - SRD 5.2.1 pp. 74; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/warlock_repelling_blast.md
  dnd2024:rules/warlock_whispers_of_the_grave:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_whispers_of_the_grave
        implementation: dnd2024:rule_warlock_whispers_of_the_grave
        parameters:
          warlock_level_minimum: 7
          cast:
            spell:
              key: dnd2024:spells/speak_with_dead
            spell_slot_cost: 0
            target: spell_definition
            uses: unlimited
    contract:
      id: dnd2024:rule_warlock_whispers_of_the_grave
      inputs:
        actor: participant
        context: resolution
      outputs:
      - feature_effects
      events: []
      capabilities:
      - invocations
      - spellcasting
    requires:
    - dnd2024:rule_warlock_whispers_of_the_grave
    citations:
    - SRD 5.2.1 pp. 74; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/warlock_whispers_of_the_grave.md
  dnd2024:rules/wizard_becoming_a_wizard:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 77; builtin/dnd2024-definitions/rules/wizard_becoming_a_wizard.md
  dnd2024:rules/wizard_wizard_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/wizard_wizard_class_features
        implementation: dnd2024:rule_wizard_wizard_class_features
        parameters:
          class:
            key: dnd2024:classes/wizard
          feature_acquisition: on_reaching_feature_class_level
          level_source: class_level
          progression_source:
            key: dnd2024:tables/wizard_progression
    contract:
      id: dnd2024:rule_wizard_wizard_class_features
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - classes
      - advancement
    requires:
    - dnd2024:rule_wizard_wizard_class_features
    citations:
    - SRD 5.2.1 pp. 77; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/wizard_wizard_class_features.md
  dnd2024:rules/character_backgrounds:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 83; builtin/dnd2024-definitions/rules/character_backgrounds.md
  dnd2024:rules/character_backgrounds_feat:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/character_backgrounds_feat
        implementation: dnd2024:rule_character_backgrounds_feat
        parameters:
          grant: background_specified_origin_feat
    contract:
      id: dnd2024:rule_character_backgrounds_feat
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
      - feats
    requires:
    - dnd2024:rule_character_backgrounds_feat
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/character_backgrounds_feat.md
  dnd2024:rules/character_backgrounds_equipment:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/character_backgrounds_equipment
        implementation: dnd2024:rule_character_backgrounds_equipment
        parameters:
          choose_one:
          - background_equipment_package
          - currency: gp
            amount: 50
    contract:
      id: dnd2024:rule_character_backgrounds_equipment
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
      - equipment
    requires:
    - dnd2024:rule_character_backgrounds_equipment
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/character_backgrounds_equipment.md
  dnd2024:rules/character_species_parts_of_a_species:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 83; builtin/dnd2024-definitions/rules/character_species_parts_of_a_species.md
  dnd2024:rules/character_species_speed:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/character_species_speed
        implementation: dnd2024:rule_character_species_speed
        parameters:
          speed_source: selected_species
    contract:
      id: dnd2024:rule_character_species_speed
      inputs:
        actor: participant
        context: resolution
      outputs:
      - character_changes
      events: []
      capabilities:
      - character_creation
      - movement
    requires:
    - dnd2024:rule_character_species_speed
    citations:
    - SRD 5.2.1 pp. 84; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/character_species_speed.md
  dnd2024:rules/feats:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 87; builtin/dnd2024-definitions/rules/feats.md
  dnd2024:rules/feat_descriptions_origin_feats:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 87; builtin/dnd2024-definitions/rules/feat_descriptions_origin_feats.md
  dnd2024:rules/feat_descriptions_epic_boon_feats:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 88; builtin/dnd2024-definitions/rules/feat_descriptions_epic_boon_feats.md
  dnd2024:rules/weapons:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons
        implementation: dnd2024:rule_weapons
        parameters:
          weapon_categories:
          - simple
          - martial
          weapon_kinds:
          - melee
          - ranged
          base_melee_reach_feet: 5
          damage_on_hit_source: weapon_damage_and_type
          properties_source: weapon_definition
          mastery:
            requires_feature_unlock: true
          selling:
            equipment_fraction_of_purchase_cost: 0.5
            trade_goods_and_valuables_fraction: 1
            magic_item_prices: magic_item_rules
    contract:
      id: dnd2024:rule_weapons
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_effects
      events: []
      capabilities:
      - equipment
    requires:
    - dnd2024:rule_weapons
    citations:
    - SRD 5.2.1 pp. 89; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/weapons.md
  dnd2024:rules/weapons_ammunition:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_ammunition
        implementation: dnd2024:rule_weapons_ammunition
        parameters:
          ranged_attack_requires: compatible_ammunition
          ammunition_consumed_per_attack: 1
          draw_ammunition_cost: part_of_attack
          load_one_handed_weapon_requires_free_hand: true
          recover_after_combat:
            time_minutes: 1
            quantity:
              op: floor
              args:
              - op: divide
                args:
                - variable: ammunition_used
                - 2
            remaining_ammunition: lost
    contract:
      id: dnd2024:rule_weapons_ammunition
      inputs:
        actor: participant
        context: resolution
        ammunition_used: number
      outputs:
      - item_effects
      events: []
      capabilities:
      - equipment
      - attacks
    requires:
    - dnd2024:rule_weapons_ammunition
    citations:
    - SRD 5.2.1 pp. 89; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/weapons_ammunition.md
  dnd2024:rules/weapons_light:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_light
        implementation: dnd2024:rule_weapons_light
        parameters:
          trigger:
            action: attack
            own_turn: true
            weapon_property: light
          extra_attack:
            count: 1
            cost: bonus_action
            timing: later_same_turn
            different_weapon: true
            weapon_property: light
            damage_ability_modifier:
              op: min
              args:
              - 0
              - variable: attack_ability_modifier
    contract:
      id: dnd2024:rule_weapons_light
      inputs:
        actor: participant
        context: resolution
        attack_ability_modifier: number
      outputs:
      - item_effects
      events:
      - attack.completed
      capabilities:
      - equipment
      - attacks
    requires:
    - dnd2024:rule_weapons_light
    citations:
    - SRD 5.2.1 pp. 89; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/weapons_light.md
  dnd2024:rules/weapons_reach:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_reach
        implementation: dnd2024:rule_weapons_reach
        parameters:
          reach_bonus_feet: 5
          scope:
          - attacks_with_weapon
          - opportunity_attack_reach_with_weapon
    contract:
      id: dnd2024:rule_weapons_reach
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_effects
      events: []
      capabilities:
      - equipment
      - attacks
    requires:
    - dnd2024:rule_weapons_reach
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/weapons_reach.md
  dnd2024:rules/weapons_versatile:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_versatile
        implementation: dnd2024:rule_weapons_versatile
        parameters:
          hands_options:
          - 1
          - 2
          two_handed_melee_damage: weapon_versatile_damage
          other_damage: weapon_base_damage
    contract:
      id: dnd2024:rule_weapons_versatile
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_effects
      events: []
      capabilities:
      - equipment
      - attacks
    requires:
    - dnd2024:rule_weapons_versatile
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/weapons_versatile.md
  dnd2024:rules/weapons_graze:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_graze
        implementation: dnd2024:rule_weapons_graze
        parameters:
          trigger: weapon_attack_misses_creature
          optional_damage:
            amount: attack_ability_modifier
            type: weapon_damage_type
            increase_sources_allowed:
            - ability_modifier_increase
    contract:
      id: dnd2024:rule_weapons_graze
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.missed
      capabilities:
      - weapon_mastery
      - damage
    requires:
    - dnd2024:rule_weapons_graze
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/weapons_graze.md
  dnd2024:rules/weapons_sap:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_sap
        implementation: dnd2024:rule_weapons_sap
        parameters:
          trigger: weapon_hits_creature
          effect:
            next_attack_roll: disadvantage
            expires: start_of_attacker_next_turn
    contract:
      id: dnd2024:rule_weapons_sap
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events:
      - attack.hit
      capabilities:
      - weapon_mastery
      - attacks
    requires:
    - dnd2024:rule_weapons_sap
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/weapons_sap.md
  dnd2024:rules/weapons_vex:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_vex
        implementation: dnd2024:rule_weapons_vex
        parameters:
          trigger: weapon_hits_and_damages_creature
          effect:
            attacker_next_attack_against_same_creature: advantage
            expires: end_of_attacker_next_turn
          interleaved_weapon_table:
            entries:
            - key: dnd2024:equipment/club
            - key: dnd2024:equipment/dagger
            - key: dnd2024:equipment/greatclub
            - key: dnd2024:equipment/handaxe
            - key: dnd2024:equipment/javelin
            - key: dnd2024:equipment/light_hammer
            - key: dnd2024:equipment/mace
            - key: dnd2024:equipment/quarterstaff
            - key: dnd2024:equipment/sickle
            - key: dnd2024:equipment/spear
            - key: dnd2024:equipment/dart
            - key: dnd2024:equipment/light_crossbow
            - key: dnd2024:equipment/shortbow
            - key: dnd2024:equipment/sling
            - key: dnd2024:equipment/battleaxe
            - key: dnd2024:equipment/flail
            - key: dnd2024:equipment/glaive
            - key: dnd2024:equipment/greataxe
            - key: dnd2024:equipment/greatsword
            - key: dnd2024:equipment/halberd
            - key: dnd2024:equipment/lance
            - key: dnd2024:equipment/longsword
            - key: dnd2024:equipment/maul
            - key: dnd2024:equipment/morningstar
            - key: dnd2024:equipment/pike
            - key: dnd2024:equipment/rapier
            - key: dnd2024:equipment/scimitar
            - key: dnd2024:equipment/shortsword
            - key: dnd2024:equipment/trident
            - key: dnd2024:equipment/warhammer
            - key: dnd2024:equipment/war_pick
            - key: dnd2024:equipment/whip
            - key: dnd2024:equipment/blowgun
            - key: dnd2024:equipment/hand_crossbow
            - key: dnd2024:equipment/heavy_crossbow
            - key: dnd2024:equipment/longbow
            - key: dnd2024:equipment/musket
            - key: dnd2024:equipment/pistol
            columns:
            - category
            - damage
            - damage_type
            - properties
            - mastery
            - weight
            - cost
    contract:
      id: dnd2024:rule_weapons_vex
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events:
      - attack.hit
      capabilities:
      - weapon_mastery
      - attacks
    requires:
    - dnd2024:rule_weapons_vex
    citations:
    - SRD 5.2.1 pp. 90, 91; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/weapons_vex.md
  dnd2024:rules/armor_light_medium_or_heavy_armor:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/armor_light_medium_or_heavy_armor
        implementation: dnd2024:rule_armor_light_medium_or_heavy_armor
        parameters:
          when:
            wearing_armor_category:
            - light
            - medium
            - heavy
            trained_in_worn_armor: false
          effects:
            d20_tests_using:
            - strength
            - dexterity
            roll_mode: disadvantage
            spellcasting_allowed: false
    contract:
      id: dnd2024:rule_armor_light_medium_or_heavy_armor
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_effects
      events: []
      capabilities:
      - equipment
      - proficiency
    requires:
    - dnd2024:rule_armor_light_medium_or_heavy_armor
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/armor_light_medium_or_heavy_armor.md
  dnd2024:rules/tools:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/tools
        implementation: dnd2024:rule_tools
        parameters:
          tool_check_ability: tool_definition
          utilize:
            options: tool_utilize_entries
            options_per_action: 1
            dc: selected_entry_dc
          craftable_items: tool_craft_entries
          each_variant_requires_separate_proficiency: true
    contract:
      id: dnd2024:rule_tools
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - tools
      - crafting
    requires:
    - dnd2024:rule_tools
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/tools.md
  dnd2024:rules/tools_other_tools:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 94; builtin/dnd2024-definitions/rules/tools_other_tools.md
  dnd2024:rules/mounts_and_vehicles_mounts_and_cargo:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/mounts_and_vehicles_mounts_and_cargo
        implementation: dnd2024:rule_mounts_and_vehicles_mounts_and_cargo
        parameters:
          vehicle_kinds:
          - carriage
          - cart
          - chariot
          - sled
          - wagon
          maximum_pulled_weight:
            op: multiply
            args:
            - 5
            - op: sum
              args:
              - variable: pulling_animals_base_carrying_capacities
          vehicle_weight_counts: true
    contract:
      id: dnd2024:rule_mounts_and_vehicles_mounts_and_cargo
      inputs:
        actor: participant
        context: resolution
        pulling_animals_base_carrying_capacities: resolution
      outputs:
      - vehicle_capacity
      events: []
      capabilities:
      - vehicles
      - encumbrance
    requires:
    - dnd2024:rule_mounts_and_vehicles_mounts_and_cargo
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/mounts_and_vehicles_mounts_and_cargo.md
  dnd2024:rules/mounts_and_vehicles_large_vehicles:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 100; builtin/dnd2024-definitions/rules/mounts_and_vehicles_large_vehicles.md
  dnd2024:rules/mounts_and_vehicles_passengers:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/mounts_and_vehicles_passengers
        implementation: dnd2024:rule_mounts_and_vehicles_passengers
        parameters:
          passenger_sizes:
          - small
          - medium
          hammock_capacity: vehicle_table_passengers
          private_capacity:
            op: floor
            args:
            - op: divide
              args:
              - variable: vehicle_table_passengers
              - 5
          typical_daily_prices:
            hammock:
              currency: sp
              amount: 5
            small_private_cabin:
              currency: gp
              amount: 2
          price_override:
            actor: gm
            scope: ship
    contract:
      id: dnd2024:rule_mounts_and_vehicles_passengers
      inputs:
        actor: participant
        context: resolution
        vehicle_table_passengers: number
      outputs:
      - vehicle_capacity
      events: []
      capabilities:
      - vehicles
      - economy
    requires:
    - dnd2024:rule_mounts_and_vehicles_passengers
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/mounts_and_vehicles_passengers.md
  dnd2024:rules/lifestyle_expenses:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/lifestyle_expenses
        implementation: dnd2024:rule_lifestyle_expenses
        parameters:
          payment_interval_choice:
            actor: gm
            options:
            - week
            - month
          selection_at: interval_start
          lifestyles:
          - wretched
          - squalid
          - poor
          - modest
          - comfortable
          - wealthy
          - aristocratic
          pay: daily_price_times_interval_days
          covers:
          - lodging
          - food
          - equipment_maintenance
          - necessities
          inherent_consequences: false
          gm_considerations:
          - risks
          - social_perception
    contract:
      id: dnd2024:rule_lifestyle_expenses
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - costs
      events: []
      capabilities:
      - economy
    requires:
    - dnd2024:rule_lifestyle_expenses
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/lifestyle_expenses.md
  dnd2024:rules/lifestyle_expenses_poor_2_sp_per_day:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/lifestyle_expenses_poor_2_sp_per_day
        implementation: dnd2024:rule_lifestyle_expenses_poor_2_sp_per_day
        parameters:
          lifestyle: poor
          cost_per_day:
            currency: sp
            amount: 2
    contract:
      id: dnd2024:rule_lifestyle_expenses_poor_2_sp_per_day
      inputs:
        actor: participant
        context: resolution
      outputs:
      - costs
      events: []
      capabilities:
      - economy
    requires:
    - dnd2024:rule_lifestyle_expenses_poor_2_sp_per_day
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/lifestyle_expenses_poor_2_sp_per_day.md
  dnd2024:rules/lifestyle_expenses_wealthy_4_gp_per_day:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/lifestyle_expenses_wealthy_4_gp_per_day
        implementation: dnd2024:rule_lifestyle_expenses_wealthy_4_gp_per_day
        parameters:
          lifestyle: wealthy
          cost_per_day:
            currency: gp
            amount: 4
    contract:
      id: dnd2024:rule_lifestyle_expenses_wealthy_4_gp_per_day
      inputs:
        actor: participant
        context: resolution
      outputs:
      - costs
      events: []
      capabilities:
      - economy
    requires:
    - dnd2024:rule_lifestyle_expenses_wealthy_4_gp_per_day
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/lifestyle_expenses_wealthy_4_gp_per_day.md
  dnd2024:rules/hirelings:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/hirelings
        implementation: dnd2024:rule_hirelings
        parameters:
          services:
          - service: skilled_hireling
            requires_any_proficiency:
            - weapon
            - tool
            - skill
            minimum_gp_per_day: 2
          - service: untrained_hireling
            requires_proficiency: false
            minimum_sp_per_day: 2
          - service: messenger
            cp_per_mile: 2
          expert_price_increase:
            actor: gm
            optional: true
    contract:
      id: dnd2024:rule_hirelings
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - costs
      events: []
      capabilities:
      - economy
    requires:
    - dnd2024:rule_hirelings
    citations:
    - SRD 5.2.1 pp. 102; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/hirelings.md
  dnd2024:rules/magic_items_identifying_a_magic_item:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_items_identifying_a_magic_item
        implementation: dnd2024:rule_magic_items_identifying_a_magic_item
        parameters:
          handling_reveals: extraordinary_nature
          automatic_properties_identification: false
          methods:
            identify_spell:
              spell:
                key: dnd2024:spells/identify
            short_rest:
              items_maximum: 1
              focus_required: true
              physical_contact_required: true
              reveal_at: rest_end
              reveals:
              - properties
              - usage
              reveals_curse: false
            potion_taste:
              quantity: small_taste
              reveals: potion_effect
          clues_and_experimentation:
            actor: gm
            outputs:
            - property_hints
    contract:
      id: dnd2024:rule_magic_items_identifying_a_magic_item
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
      - identification
    requires:
    - dnd2024:rule_magic_items_identifying_a_magic_item
    citations:
    - SRD 5.2.1 pp. 102; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/magic_items_identifying_a_magic_item.md
  dnd2024:rules/magic_items_no_more_than_three_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_items_no_more_than_three_items
        implementation: dnd2024:rule_magic_items_no_more_than_three_items
        parameters:
          attuned_items_maximum: 3
          fourth_attempt: fails
          same_definition_copies_attuned_maximum: 1
          must_end_existing_before_new_if_at_limit: true
    contract:
      id: dnd2024:rule_magic_items_no_more_than_three_items
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
      - attunement
    requires:
    - dnd2024:rule_magic_items_no_more_than_three_items
    citations:
    - SRD 5.2.1 pp. 102; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/magic_items_no_more_than_three_items.md
  dnd2024:rules/magic_items_multiple_items_of_the_same_kind:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_items_multiple_items_of_the_same_kind
        implementation: dnd2024:rule_magic_items_multiple_items_of_the_same_kind
        parameters:
          worn_category_limits:
            footwear_pair: 1
            gloves_or_gauntlets_pair: 1
            bracers_pair: 1
            armor_suit: 1
            headwear: 1
            cloak: 1
          exceptions:
            actor: gm
            scope: wearing_multiple_items
    contract:
      id: dnd2024:rule_magic_items_multiple_items_of_the_same_kind
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
      - equipment
    requires:
    - dnd2024:rule_magic_items_multiple_items_of_the_same_kind
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/magic_items_multiple_items_of_the_same_kind.md
  dnd2024:rules/crafting_nonmagical_items_tools:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/crafting_nonmagical_items_tools
        implementation: dnd2024:rule_crafting_nonmagical_items_tools
        parameters:
          required_tool_source: tool_craft_table
          unlisted_required_tool:
            actor: gm
            choice: tool
          must_use_required_tool: true
          proficiency_required_for:
          - crafter
          - every_assistant
    contract:
      id: dnd2024:rule_crafting_nonmagical_items_tools
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - crafted_items
      events: []
      capabilities:
      - crafting
    requires:
    - dnd2024:rule_crafting_nonmagical_items_tools
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/crafting_nonmagical_items_tools.md
  dnd2024:rules/brewing_potions_of_healing:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/brewing_potions_of_healing
        implementation: dnd2024:rule_brewing_potions_of_healing
        parameters:
          tool:
            key: dnd2024:equipment/other_tools_herbalism_kit_5_gp
          tool_proficiency_required: true
          tool_use_required: true
          output:
            key: dnd2024:equipment/potion_of_healing_50_gp
          quantity: 1
          raw_material_gp: 25
          days: 1
          work_hours: 8
    contract:
      id: dnd2024:rule_brewing_potions_of_healing
      inputs:
        actor: participant
        context: resolution
      outputs:
      - crafted_items
      events: []
      capabilities:
      - crafting
    requires:
    - dnd2024:rule_brewing_potions_of_healing
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/brewing_potions_of_healing.md
  dnd2024:rules/scribing_spell_scrolls_prerequisites_for_the_scribe:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/scribing_spell_scrolls_prerequisites_for_the_scribe
        implementation: dnd2024:rule_scribing_spell_scrolls_prerequisites_for_the_scribe
        parameters:
          proficiency_any:
          - arcana
          - calligraphers_supplies
          spell_prepared_each_work_day: true
          material_components_available: true
          consumed_components_consumption_timing: scroll_completion
          scroll_save_dc: scribe_spell_save_dc
          scroll_attack_bonus: scribe_spell_attack_bonus
    contract:
      id: dnd2024:rule_scribing_spell_scrolls_prerequisites_for_the_scribe
      inputs:
        actor: participant
        context: resolution
      outputs:
      - crafted_items
      events: []
      capabilities:
      - crafting
      - spellcasting
    requires:
    - dnd2024:rule_scribing_spell_scrolls_prerequisites_for_the_scribe
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/scribing_spell_scrolls_prerequisites_for_the_scribe.md
  dnd2024:rules/gaining_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/gaining_spells
        implementation: dnd2024:rule_gaining_spells
        parameters:
          cast_access_any:
          - prepared_spell
          - magic_item_spell_access
          feature_controls:
          - spell_access
          - always_prepared
          - prepared_list_changes
    contract:
      id: dnd2024:rule_gaining_spells
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_resolution
      events: []
      capabilities:
      - spellcasting
    requires:
    - dnd2024:rule_gaining_spells
    citations:
    - SRD 5.2.1 pp. 104; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/gaining_spells.md
  dnd2024:rules/casting_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells
        implementation: dnd2024:rule_casting_spells
        parameters:
          armor_worn_requires_training_to_cast: true
    contract:
      id: dnd2024:rule_casting_spells
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_resolution
      events: []
      capabilities:
      - spellcasting
      - proficiency
    requires:
    - dnd2024:rule_casting_spells
    citations:
    - SRD 5.2.1 pp. 104; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/casting_spells.md
  dnd2024:rules/casting_spells_casting_without_slots:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_casting_without_slots
        implementation: dnd2024:rule_casting_spells_casting_without_slots
        parameters:
          slot_free_methods:
            cantrip:
              level: 0
            ritual:
              requires_ritual_tag: true
              requires_prepared: true
              added_casting_minutes: 10
              other_casting_rules_apply: true
            special_ability:
              spells_and_usage_limits: feature_definition
            magic_item:
              spells_and_usage_limits: item_definition
          ritual_spell_may_use_normal_casting: true
    contract:
      id: dnd2024:rule_casting_spells_casting_without_slots
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_resolution
      events: []
      capabilities:
      - spellcasting
    requires:
    - dnd2024:rule_casting_spells_casting_without_slots
    citations:
    - SRD 5.2.1 pp. 104; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/casting_spells_casting_without_slots.md
  dnd2024:rules/casting_spells_class_spell_lists:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_class_spell_lists
        implementation: dnd2024:rule_casting_spells_class_spell_lists
        parameters:
          class_membership_source: spell_header_class_list
          feature_can_add_to_character_spell_list: true
          feature_addition_requires_listed_class_membership: false
    contract:
      id: dnd2024:rule_casting_spells_class_spell_lists
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_resolution
      events: []
      capabilities:
      - spellcasting
    requires:
    - dnd2024:rule_casting_spells_class_spell_lists
    citations:
    - SRD 5.2.1 pp. 105; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/casting_spells_class_spell_lists.md
  dnd2024:rules/casting_spells_reaction_and_bonus_action_triggers:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_reaction_and_bonus_action_triggers
        implementation: dnd2024:rule_casting_spells_reaction_and_bonus_action_triggers
        parameters:
          reaction_spell_requires_trigger: casting_time_entry
          bonus_action_spell_requires_trigger_if_defined: true
    contract:
      id: dnd2024:rule_casting_spells_reaction_and_bonus_action_triggers
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_resolution
      events: []
      capabilities:
      - spellcasting
      - reactions
    requires:
    - dnd2024:rule_casting_spells_reaction_and_bonus_action_triggers
    citations:
    - SRD 5.2.1 pp. 105; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/casting_spells_reaction_and_bonus_action_triggers.md
  dnd2024:rules/casting_spells_components:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_components
        implementation: dnd2024:rule_casting_spells_components
        parameters:
          component_types:
          - verbal
          - somatic
          - material
          required_components: spell_definition
          cast_allowed_only_if_all_required_components_available: true
    contract:
      id: dnd2024:rule_casting_spells_components
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_resolution
      events: []
      capabilities:
      - spellcasting
    requires:
    - dnd2024:rule_casting_spells_components
    citations:
    - SRD 5.2.1 pp. 105; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/casting_spells_components.md
  dnd2024:rules/casting_spells_material_m:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_material_m
        implementation: dnd2024:rule_casting_spells_material_m
        parameters:
          material_source: spell_components_entry
          consumed_only_if_specified: true
          free_hand_to_access: true
          same_hand_can_perform_somatic: true
          substitution:
            allowed_if:
              consumed: false
              specified_cost: false
            options:
              component_pouch:
                free_hand_required: true
              spellcasting_focus:
                granting_feature_required: true
                held_required_unless_item_exception: true
    contract:
      id: dnd2024:rule_casting_spells_material_m
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_resolution
      events: []
      capabilities:
      - spellcasting
      - equipment
    requires:
    - dnd2024:rule_casting_spells_material_m
    citations:
    - SRD 5.2.1 pp. 105, 106; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/casting_spells_material_m.md
  dnd2024:rules/casting_spells_targets:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_targets
        implementation: dnd2024:rule_casting_spells_targets
        parameters:
          target_kind_source: spell_description
          clear_path_required: true
          total_cover_blocks_targeting: true
          self_targeting:
            allowed_if_creature_of_choice: true
            excluded_if:
            - hostile_required
            - other_creature_required
          area_shapes:
          - cone
          - cube
          - cylinder
          - emanation
          - line
          - sphere
          area_targets: spell_area_geometry
          target_awareness:
            requires_perceptible_effect: true
            spell_exception_overrides: true
          invalid_target:
            effect: none
            spent_slot_refunded: false
            perception:
              if_successful_save_normally_no_effect: apparent_successful_save
              otherwise: no_effect_observed
            saving_throw_rolled: false
    contract:
      id: dnd2024:rule_casting_spells_targets
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - spell_resolution
      events: []
      capabilities:
      - spellcasting
      - targeting
    requires:
    - dnd2024:rule_casting_spells_targets
    citations:
    - SRD 5.2.1 pp. 106; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/casting_spells_targets.md
  dnd2024:rules/casting_spells_combining_spell_effects:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_combining_spell_effects
        implementation: dnd2024:rule_casting_spells_combining_spell_effects
        parameters:
          different_spell_effects: combine
          same_spell_effects:
            stack: false
            selection:
            - most_potent
            - most_recent_if_equal
            scope: overlapping_durations
            independent_durations_continue: true
          identify_ongoing_spell:
            requires:
            - noninstantaneous
            - ongoing_duration
            - observable_effects
            action: study
            ability: intelligence
            skill: arcana
            dc: 15
            success: identify_spell
    contract:
      id: dnd2024:rule_casting_spells_combining_spell_effects
      inputs:
        actor: participant
        context: resolution
      outputs:
      - spell_resolution
      events: []
      capabilities:
      - spellcasting
      - effects
      - checks
    requires:
    - dnd2024:rule_casting_spells_combining_spell_effects
    citations:
    - SRD 5.2.1 pp. 106; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/casting_spells_combining_spell_effects.md
  dnd2024:rules/travel_pace_good_roads:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/travel_pace_good_roads
        implementation: dnd2024:rule_travel_pace_good_roads
        parameters:
          good_road_maximum_pace_step_increase: 1
          pace_order:
          - slow
          - normal
          - fast
          cap: fast
    contract:
      id: dnd2024:rule_travel_pace_good_roads
      inputs:
        actor: participant
        context: resolution
      outputs:
      - travel_rates
      events: []
      capabilities:
      - travel
    requires:
    - dnd2024:rule_travel_pace_good_roads
    citations:
    - SRD 5.2.1 pp. 192; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/travel_pace_good_roads.md
  dnd2024:rules/travel_pace_special_movement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/travel_pace_special_movement
        implementation: dnd2024:rule_travel_pace_special_movement
        parameters:
          eligibility: sustained_special_movement_speed
          miles_per_hour:
            op: divide
            args:
            - variable: speed_feet
            - 10
          normal_miles_per_day:
            op: multiply
            args:
            - variable: miles_per_hour
            - variable: hours_traveled
          typical_hours_traveled: 8
          fast_miles_per_day:
            op: floor
            args:
            - op: multiply
              args:
              - variable: normal_miles_per_day
              - numerator: 4
                denominator: 3
          slow_miles_per_day:
            op: floor
            args:
            - op: multiply
              args:
              - variable: normal_miles_per_day
              - numerator: 2
                denominator: 3
          fast_pace_regardless_of_terrain_when_any:
          - flying
          - special_movement_ignores_difficult_terrain
          terrain_table:
          - terrain: arctic
            maximum_pace: fast
            encounter_distance_feet:
              op: multiply
              args:
              - dice:
                  count: 6
                  sides: 6
              - 10
            foraging_dc: 20
            navigation_dc: 10
            search_dc: 10
          - terrain: coastal
            maximum_pace: normal
            encounter_distance_feet:
              op: multiply
              args:
              - dice:
                  count: 2
                  sides: 10
              - 10
            foraging_dc: 10
            navigation_dc: 5
            search_dc: 15
          - terrain: desert
            maximum_pace: normal
            encounter_distance_feet:
              op: multiply
              args:
              - dice:
                  count: 6
                  sides: 6
              - 10
            foraging_dc: 20
            navigation_dc: 10
            search_dc: 10
          - terrain: forest
            maximum_pace: normal
            encounter_distance_feet:
              op: multiply
              args:
              - dice:
                  count: 2
                  sides: 8
              - 10
            foraging_dc: 10
            navigation_dc: 15
            search_dc: 15
          - terrain: grassland
            maximum_pace: fast
            encounter_distance_feet:
              op: multiply
              args:
              - dice:
                  count: 6
                  sides: 6
              - 10
            foraging_dc: 15
            navigation_dc: 5
            search_dc: 15
          - terrain: hill
            maximum_pace: normal
            encounter_distance_feet:
              op: multiply
              args:
              - dice:
                  count: 2
                  sides: 10
              - 10
            foraging_dc: 15
            navigation_dc: 10
            search_dc: 15
          - terrain: mountain
            maximum_pace: slow
            encounter_distance_feet:
              op: multiply
              args:
              - dice:
                  count: 4
                  sides: 10
              - 10
            foraging_dc: 20
            navigation_dc: 15
            search_dc: 20
          - terrain: swamp
            maximum_pace: slow
            encounter_distance_feet:
              op: multiply
              args:
              - dice:
                  count: 2
                  sides: 8
              - 10
            foraging_dc: 10
            navigation_dc: 15
            search_dc: 20
          - terrain: underdark
            maximum_pace: normal
            encounter_distance_feet:
              op: multiply
              args:
              - dice:
                  count: 2
                  sides: 6
              - 10
            foraging_dc: 20
            navigation_dc: 10
            search_dc: 20
          - terrain: urban
            maximum_pace: normal
            encounter_distance_feet:
              op: multiply
              args:
              - dice:
                  count: 2
                  sides: 6
              - 10
            foraging_dc: 20
            navigation_dc: 15
            search_dc: 15
          - terrain: waterborne
            maximum_pace: vehicle_speed
            encounter_distance_feet:
              op: multiply
              args:
              - dice:
                  count: 6
                  sides: 6
              - 10
            foraging_dc: 15
            navigation_dc: 10
            search_dc: 15
          arctic_fast_requires: appropriate_equipment
          waterborne_speed_source: vehicle
    contract:
      id: dnd2024:rule_travel_pace_special_movement
      inputs:
        actor: participant
        context: resolution
        hours_traveled: number
        miles_per_hour: number
        normal_miles_per_day: number
        speed_feet: number
      outputs:
      - travel_rates
      events: []
      capabilities:
      - travel
      - random_tables
    requires:
    - dnd2024:rule_travel_pace_special_movement
    citations:
    - SRD 5.2.1 pp. 192; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/travel_pace_special_movement.md
  dnd2024:rules/creating_a_background_1_choose_abilities:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/creating_a_background_1_choose_abilities
        implementation: dnd2024:rule_creating_a_background_1_choose_abilities
        parameters:
          background_ability_choice:
            count: 3
            distinct: true
            options:
            - strength
            - dexterity
            - constitution
            - intelligence
            - wisdom
            - charisma
            criterion: appropriate_to_background
          interleaved_terrain_dc_columns:
            source:
              key: dnd2024:rules/travel_pace_special_movement
            field: terrain_table
    contract:
      id: dnd2024:rule_creating_a_background_1_choose_abilities
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - background_definition
      events: []
      capabilities:
      - background_creation
    requires:
    - dnd2024:rule_creating_a_background_1_choose_abilities
    citations:
    - SRD 5.2.1 pp. 192, 193; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/creating_a_background_1_choose_abilities.md
  dnd2024:rules/creating_a_background_4_choose_a_tool_proficiency:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/creating_a_background_4_choose_a_tool_proficiency
        implementation: dnd2024:rule_creating_a_background_4_choose_a_tool_proficiency
        parameters:
          tool_proficiency_choice:
            count: 1
            criterion_any:
            - used_in_background_practice
            - associated_with_background
    contract:
      id: dnd2024:rule_creating_a_background_4_choose_a_tool_proficiency
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - background_definition
      events: []
      capabilities:
      - background_creation
    requires:
    - dnd2024:rule_creating_a_background_4_choose_a_tool_proficiency
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/creating_a_background_4_choose_a_tool_proficiency.md
  dnd2024:rules/magical_contagions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magical_contagions
        implementation: dnd2024:rule_magical_contagions
        parameters:
          curse_duration: specified_time_or_until_ended
          magical_contagion:
            magical_origin: true
            adverse_effect: true
            contagious: true
    contract:
      id: dnd2024:rule_magical_contagions
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events: []
      capabilities:
      - curses
      - contagions
    requires:
    - dnd2024:rule_magical_contagions
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/magical_contagions.md
  dnd2024:rules/magical_contagions_cursed_creatures:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magical_contagions_cursed_creatures
        implementation: dnd2024:rule_magical_contagions_cursed_creatures
        parameters:
          curse_origin_interaction:
            actor: gm
            scope: remove_curse_on_accursed_origin_creature
            choice: source_specific_curse_removal_effect
          optional_mummy_rule:
            enabled_by: gm
            permanent_destruction_requires:
              cast:
                key: dnd2024:spells/remove_curse
              target: corpse
    contract:
      id: dnd2024:rule_magical_contagions_cursed_creatures
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events: []
      capabilities:
      - curses
    requires:
    - dnd2024:rule_magical_contagions_cursed_creatures
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/magical_contagions_cursed_creatures.md
  dnd2024:rules/magical_contagions_environmental_curses:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magical_contagions_environmental_curses
        implementation: dnd2024:rule_magical_contagions_environmental_curses
        parameters:
          curse: demonic_possession
          exposure_contexts:
          - demonic_object_interaction
          - desecrated_location_lingering
          target_selection: gm
          initial_save:
            ability: charisma
            dc: 15
            failure: possessed_by_bodiless_demonic_entity
          while_possessed:
            trigger:
              d20_test_die_result: 1
            effect: entity_controls_creature_behavior
            regain_control_save:
              timing: end_of_each_later_turn
              ability: charisma
              dc: 15
              success: creature_regains_control
              curse_persists: true
          end_curse:
            long_rest_end_save:
              ability: charisma
              dc: 15
              success: curse_ends
            magic:
            - key: dnd2024:spells/dispel_evil_and_good
            - any_curse_removal_magic
    contract:
      id: dnd2024:rule_magical_contagions_environmental_curses
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events:
      - d20_test.rolled
      - turn.ended
      - rest.long_completed
      capabilities:
      - curses
      - possession
      - saving_throws
    requires:
    - dnd2024:rule_magical_contagions_environmental_curses
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/magical_contagions_environmental_curses.md
  dnd2024:rules/magical_contagions_example_contagions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 194; builtin/dnd2024-definitions/rules/magical_contagions_example_contagions.md
  dnd2024:rules/magical_contagions_sight_rot:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magical_contagions_sight_rot
        implementation: dnd2024:rule_magical_contagions_sight_rot
        parameters:
          contagion: sight_rot
          water_exposure:
            creature_types:
            - beast
            - humanoid
            trigger: drink_tainted_water
            save:
              ability: constitution
              dc: 15
              failure: infected
          infected_effect:
            condition: blinded
            until: contagion_ends
          immediate_cures:
          - key: dnd2024:spells/heal
          - key: dnd2024:spells/lesser_restoration
          ointment:
            craft_tool:
              key: dnd2024:equipment/other_tools_herbalism_kit_5_gp
            proficiency_required: true
            tool_use_required: true
            hours_per_dose: 1
            doses_per_craft: 1
            application_location: eyes
            suppression_hours_per_dose: 24
            total_suppression_hours_to_cure: 72
            doses_and_applications_to_cure: 3
          skin_contact_spread:
            recipient_type: humanoid
            source: infected_creature
            save:
              ability: constitution
              dc: 15
              failure: infected
              success:
                immunity_to_same_source_hours: 24
    contract:
      id: dnd2024:rule_magical_contagions_sight_rot
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events:
      - water.drunk
      - creature.skin_contact
      capabilities:
      - contagions
      - conditions
      - crafting
    requires:
    - dnd2024:rule_magical_contagions_sight_rot
    citations:
    - SRD 5.2.1 pp. 194; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/magical_contagions_sight_rot.md
  dnd2024:rules/environmental_effects_extreme_cold:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/environmental_effects_extreme_cold
        implementation: dnd2024:rule_environmental_effects_extreme_cold
        parameters:
          temperature_fahrenheit_maximum: 0
          exposure_save:
            timing: end_of_each_hour
            ability: constitution
            dc: 10
            failure:
              exhaustion_levels_add: 1
            automatic_success_if_any:
            - cold_resistance
            - cold_immunity
    contract:
      id: dnd2024:rule_environmental_effects_extreme_cold
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events:
      - hour.ended
      capabilities:
      - environment
      - saving_throws
    requires:
    - dnd2024:rule_environmental_effects_extreme_cold
    citations:
    - SRD 5.2.1 pp. 195; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/environmental_effects_extreme_cold.md
  dnd2024:rules/environmental_effects_heavy_precipitation:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/environmental_effects_heavy_precipitation
        implementation: dnd2024:rule_environmental_effects_heavy_precipitation
        parameters:
          weather:
          - heavy_rain
          - heavy_snow
          area_obscurity: lightly_obscured
          wisdom_perception_checks: disadvantage
          heavy_rain_extinguishes_open_flames: true
    contract:
      id: dnd2024:rule_environmental_effects_heavy_precipitation
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events: []
      capabilities:
      - environment
      - senses
    requires:
    - dnd2024:rule_environmental_effects_heavy_precipitation
    citations:
    - SRD 5.2.1 pp. 195; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/environmental_effects_heavy_precipitation.md
  dnd2024:rules/environmental_effects_strong_wind:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/environmental_effects_strong_wind
        implementation: dnd2024:rule_environmental_effects_strong_wind
        parameters:
          ranged_weapon_attacks: disadvantage
          extinguish_open_flames: true
          disperse_fog: true
          flying_creature_end_turn:
            must: land
            otherwise: fall
          desert_sandstorm:
            gm_option: true
            wisdom_perception: disadvantage
    contract:
      id: dnd2024:rule_environmental_effects_strong_wind
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - turn.ended
      capabilities:
      - environment
      - movement
    requires:
    - dnd2024:rule_environmental_effects_strong_wind
    citations:
    - SRD 5.2.1 pp. 195; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/environmental_effects_strong_wind.md
  dnd2024:rules/fear_and_mental_stress_fear_effects:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/fear_and_mental_stress_fear_effects
        implementation: dnd2024:rule_fear_and_mental_stress_fear_effects
        parameters:
          trigger: supernaturally_frightful_encounter
          default_save_ability: wisdom
          failed_save_condition: frightened
          dc_choice:
            actor: gm
            criterion: situation_terror
            examples:
            - situation: harmless_sarcophagus_apparition
              dc: 10
            - situation: magical_trap_private_worst_fear_illusion
              dc: 15
            - situation: abyss_portal_reveals_torment
              dc: 20
          default_repeat_save:
            timing: end_of_each_target_turn
            success: effect_ends
          optional_riders:
            actor: gm
            duration: while_frightened
            choices:
            - must_take_action: dash
              movement_direction: away_from_fear_source
            - attacks_against_target: advantage
            - per_turn_choose_exactly_one:
              - movement
              - action
              - bonus_action
    contract:
      id: dnd2024:rule_fear_and_mental_stress_fear_effects
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - turn.ended
      capabilities:
      - fear
      - saving_throws
    requires:
    - dnd2024:rule_fear_and_mental_stress_fear_effects
    citations:
    - SRD 5.2.1 pp. 196; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/fear_and_mental_stress_fear_effects.md
  dnd2024:rules/poison:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison
        implementation: dnd2024:rule_poison
        parameters:
          types:
            contact:
              application: smear_on_object
              potent_until_any:
              - exposed_skin_touch
              - washed_off
              exposure: exposed_skin_touch
            ingested:
              full_effect_requires: swallow_entire_dose
              delivery:
              - food
              - liquid
              partial_dose:
                actor: gm
                choices:
                - no_effect
                - saving_throw: advantage
                - failed_save_damage_multiplier: 0.5
            inhaled:
              delivery:
              - blown_powder
              - released_gas
              area:
                shape: cube
                edge_feet: 5
              cloud_duration: immediately_dissipates_after_exposure
              holding_breath_prevents: false
            injury:
              application_action: bonus_action
              apply_to:
              - weapon
              - ammunition_piece
              - similar_object
              potent_until_any:
              - delivered_through_wound
              - washed_off
              exposure_requires:
                coated_object_damage_type:
                - piercing
                - slashing
    contract:
      id: dnd2024:rule_poison
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events: []
      capabilities:
      - poisons
    requires:
    - dnd2024:rule_poison
    citations:
    - SRD 5.2.1 pp. 197; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/poison.md
  dnd2024:rules/poison_sample_poisons:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 197; builtin/dnd2024-definitions/rules/poison_sample_poisons.md
  dnd2024:rules/poison_crawler_mucus_200_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_crawler_mucus_200_gp
        implementation: dnd2024:rule_poison_crawler_mucus_200_gp
        parameters:
          price_gp_per_dose: 200
          poison_type: contact
          save:
            ability: constitution
            dc: 13
            failure:
              condition: poisoned
              duration_minutes: 1
          while_poisoned_by_this_effect:
            condition: paralyzed
          repeat_save:
            timing: end_of_each_target_turn
            success: effect_ends
    contract:
      id: dnd2024:rule_poison_crawler_mucus_200_gp
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events:
      - poison.exposure
      - turn.ended
      capabilities:
      - poisons
      - conditions
      - saving_throws
    requires:
    - dnd2024:rule_poison_crawler_mucus_200_gp
    citations:
    - SRD 5.2.1 pp. 197; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/poison_crawler_mucus_200_gp.md
  dnd2024:rules/poison_midnight_tears_1_500_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_midnight_tears_1_500_gp
        implementation: dnd2024:rule_poison_midnight_tears_1_500_gp
        parameters:
          price_gp_per_dose: 1500
          poison_type: ingested
          activation_timing: next_midnight
          before_activation_effect: none
          neutralized_by: any_effect_ending_poisoned_condition
          activation_save:
            ability: constitution
            dc: 17
            failure:
              damage:
                dice:
                  count: 9
                  sides: 6
              average: 31
              type: poison
            success:
              damage_fraction: 0.5
              rounding: floor
    contract:
      id: dnd2024:rule_poison_midnight_tears_1_500_gp
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events:
      - midnight
      capabilities:
      - poisons
      - damage
      - saving_throws
    requires:
    - dnd2024:rule_poison_midnight_tears_1_500_gp
    citations:
    - SRD 5.2.1 pp. 197, 198; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/poison_midnight_tears_1_500_gp.md
  dnd2024:rules/poison_purple_worm_poison_2_000_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_purple_worm_poison_2_000_gp
        implementation: dnd2024:rule_poison_purple_worm_poison_2_000_gp
        parameters:
          price_gp_per_dose: 2000
          poison_type: injury
          save:
            ability: constitution
            dc: 21
            failure:
              damage:
                dice:
                  count: 10
                  sides: 6
              average: 35
              type: poison
            success:
              damage_fraction: 0.5
              rounding: floor
    contract:
      id: dnd2024:rule_poison_purple_worm_poison_2_000_gp
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events:
      - poison.exposure
      capabilities:
      - poisons
      - damage
      - saving_throws
    requires:
    - dnd2024:rule_poison_purple_worm_poison_2_000_gp
    citations:
    - SRD 5.2.1 pp. 198; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/poison_purple_worm_poison_2_000_gp.md
  dnd2024:rules/poison_torpor_600_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_torpor_600_gp
        implementation: dnd2024:rule_poison_torpor_600_gp
        parameters:
          price_gp_per_dose: 600
          poison_type: ingested
          save:
            ability: constitution
            dc: 15
            failure:
              condition: poisoned
              duration:
                unit: hours
                amount:
                  dice:
                    count: 4
                    sides: 6
          while_poisoned_by_this_effect:
            speed_multiplier: 0.5
    contract:
      id: dnd2024:rule_poison_torpor_600_gp
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events:
      - poison.exposure
      capabilities:
      - poisons
      - conditions
      - saving_throws
    requires:
    - dnd2024:rule_poison_torpor_600_gp
    citations:
    - SRD 5.2.1 pp. 198; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/poison_torpor_600_gp.md
  dnd2024:rules/traps:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 199; builtin/dnd2024-definitions/rules/traps.md
  dnd2024:rules/traps_collapsing_roof:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/traps_collapsing_roof
        implementation: dnd2024:rule_traps_collapsing_roof
        parameters:
          severity: deadly
          default_levels:
          - 1
          - 4
          trigger: first_creature_crosses_trip_wire
          wire_height_inches: 3
          support_count: 2
          duration: instantaneous
          targets: creatures_beneath_unstable_ceiling
          save:
            ability: dexterity
            dc: 13
            failure:
              damage:
                dice:
                  count: 2
                  sides: 10
              average: 11
              type: bludgeoning
            success:
              damage_fraction: 0.5
              rounding: floor
          aftermath: area_becomes_difficult_terrain
          detect:
            action: search
            ability: wisdom
            skill: perception
            dc: 11
            success_reveals:
            - trip_wire
            - unstable_ceiling
          disarm_or_bypass:
            requires: detected
            options:
            - cut_wire
            - avoid_wire
            check_required: false
          higher_level_variants:
          - levels:
            - 5
            - 10
            damage:
              dice:
                count: 4
                sides: 10
            average: 22
            dc: 15
          - levels:
            - 11
            - 16
            damage:
              dice:
                count: 10
                sides: 10
            average: 55
            dc: 17
          - levels:
            - 17
            - 20
            damage:
              dice:
                count: 18
                sides: 10
            average: 99
            dc: 19
          geometry_assumption:
            unstable_ceiling_area: gm_supplied_location
    contract:
      id: dnd2024:rule_traps_collapsing_roof
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - creature.crosses_trip_wire
      capabilities:
      - traps
      - damage
      - terrain
    requires:
    - dnd2024:rule_traps_collapsing_roof
    citations:
    - SRD 5.2.1 pp. 199; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/traps_collapsing_roof.md
  dnd2024:rules/traps_hidden_pit:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/traps_hidden_pit
        implementation: dnd2024:rule_traps_hidden_pit
        parameters:
          severity: nuisance
          default_levels:
          - 1
          - 4
          trigger: creature_moves_onto_lid
          duration: instantaneous
          pit_depth_feet: 10
          trigger_effect:
            lid: opens_and_remains_open
            creature: falls_into_pit
            damage:
              dice:
                count: 1
                sides: 6
            average: 3
            damage_type: bludgeoning
          detect:
            action: study
            ability: intelligence
            skill: investigation
            dc: 15
            success: detect_pit
          disarm:
            requires: detected
            options:
            - wedge: iron_spike_or_similar
              result: lid_cannot_open
            - magic:
                key: dnd2024:spells/arcane_lock
              result: lid_held_shut
            crossing_after_disarm: safe
          escape:
            wall: smooth
            requires_any:
            - climb_speed
            - climbing_gear
            - spell:
                key: dnd2024:spells/spider_climb
            - similar_magic
            gm_easier_variant: cracks_for_handholds_and_footholds
          higher_level_variants:
          - levels:
            - 5
            - 10
            depth_feet: 30
            damage:
              dice:
                count: 3
                sides: 6
            average: 10
          - levels:
            - 11
            - 16
            depth_feet: 60
            damage:
              dice:
                count: 6
                sides: 6
            average: 21
          - levels:
            - 17
            - 20
            depth_feet: 120
            damage:
              dice:
                count: 12
                sides: 6
            average: 42
    contract:
      id: dnd2024:rule_traps_hidden_pit
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - creature.enters_lid
      capabilities:
      - traps
      - damage
      - movement
    requires:
    - dnd2024:rule_traps_hidden_pit
    citations:
    - SRD 5.2.1 pp. 200; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/traps_hidden_pit.md
  dnd2024:rules/traps_rolling_stone:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/traps_rolling_stone
        implementation: dnd2024:rule_traps_rolling_stone
        parameters:
          severity_by_level:
          - levels:
            - 11
            - 16
            severity: deadly
          - levels:
            - 17
            - 20
            severity: nuisance
          trigger: creature_enters_pressure_plate
          duration: until_stone_stops
          stone:
            shape: sphere
            radius_feet: 5
            size: large
            material: solid_stone
            ac: 17
            hit_points: 100
            damage_threshold: 10
            damage_immunities:
            - poison
            - psychic
            initiative_bonus: 8
            movement_feet_per_turn: 60
          initiative_participants: stone_and_nearby_creatures
          movement:
            direction: single_direction
            obstacle_can_redirect: true
            can_pass_creature_spaces: true
            creatures_can_pass_stone_space: true
            stone_space: difficult_terrain
            stops_at: wall_or_similar_barrier
            turns_corners: false
            curving_passages_allow_redirection: true
          collision:
            triggers:
            - stone_enters_creature_space_first_time_on_turn
            - creature_enters_rolling_stone_space
            save:
              ability: dexterity
              dc: 15
              failure:
                damage:
                  dice:
                    count: 10
                    sides: 10
                average: 55
                type: bludgeoning
                condition: prone
              success: no_effect
          detect:
            action: study
            ability: intelligence
            skill: investigation
            dc: 15
            success: deduce_pressure_plate_function
          disarm:
            wedge_under_plate: iron_spike_or_similar
            effect: prevents_trigger
          slow:
            action_cost: 1
            ability: strength
            skill: athletics
            dc: 20
            success:
              movement_feet_per_turn_subtract: 15
              at_zero: stone_stops
    contract:
      id: dnd2024:rule_traps_rolling_stone
      inputs:
        actor: participant
        context: resolution
      outputs:
      - effects
      events:
      - pressure_plate.entered
      - turn.started
      - stone.enters_creature_space
      - creature.enters_stone_space
      capabilities:
      - traps
      - objects
      - movement
      - damage
    requires:
    - dnd2024:rule_traps_rolling_stone
    citations:
    - SRD 5.2.1 pp. 201; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/traps_rolling_stone.md
  dnd2024:rules/combat_encounters_combat_encounter_difficulty:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 202; builtin/dnd2024-definitions/rules/combat_encounters_combat_encounter_difficulty.md
  dnd2024:rules/combat_encounters_step_3_spend_your_budget:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_encounters_step_3_spend_your_budget
        implementation: dnd2024:rule_combat_encounters_step_3_spend_your_budget
        parameters:
          budget_remaining:
            op: subtract
            args:
            - variable: xp_budget
            - op: sum
              args:
              - variable: selected_monster_xp
          constraint: selected_total_lte_budget
          unspent_xp_allowed: true
          examples:
          - party_size: 4
            level: 1
            difficulty: low
            xp_per_character: 50
            budget: 200
            choices:
            - - monster: bugbear_warrior
                count: 1
                xp_each: 200
            - - monster: giant_wasp
                count: 2
                xp_each: 100
            - - monster: giant_rat
                count: 6
                xp_each: 25
          - party_size: 5
            level: 3
            difficulty: moderate
            xp_per_character: 225
            budget: 1125
            choices:
            - - monster: druid
                count: 2
                xp_each: 450
              - monster: stirge
                count: 9
                xp_each: 25
            - - monster: wight
                count: 1
                xp_each: 700
              - monster: warhorse_skeleton
                count: 1
                xp_each: 100
              - monster: skeleton
                count: 6
                xp_each: 50
          - party_size: 6
            level: 15
            difficulty: high
            xp_per_character: 7800
            budget: 46800
            choices:
            - - monster: adult_red_dragon
                count: 2
                xp_each: 18000
              - monster: fire_giant
                count: 2
                xp_each: 5000
    contract:
      id: dnd2024:rule_combat_encounters_step_3_spend_your_budget
      inputs:
        actor: participant
        context: resolution
        selected_monster_xp: resolution
        xp_budget: number
        selection:
          type: choice
          required: false
      outputs:
      - resolution
      events: []
      capabilities:
      - encounter_building
    requires:
    - dnd2024:rule_combat_encounters_step_3_spend_your_budget
    citations:
    - SRD 5.2.1 pp. 202, 203; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/combat_encounters_step_3_spend_your_budget.md
  dnd2024:rules/combat_encounters_adjustments:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 203; builtin/dnd2024-definitions/rules/combat_encounters_adjustments.md
  dnd2024:rules/combat_encounters_powerful_creatures:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 203; builtin/dnd2024-definitions/rules/combat_encounters_powerful_creatures.md
  dnd2024:rules/magic_item_categories:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_categories
        implementation: dnd2024:rule_magic_item_categories
        parameters:
          magic_item_category_count: 1
          category_options:
          - armor
          - potion
          - ring
          - rod
          - scroll
          - staff
          - wand
          - weapon
          - wondrous_item
    contract:
      id: dnd2024:rule_magic_item_categories
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
    requires:
    - dnd2024:rule_magic_item_categories
    citations:
    - SRD 5.2.1 pp. 204; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/magic_item_categories.md
  dnd2024:rules/magic_item_categories_rings:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_categories_rings
        implementation: dnd2024:rule_magic_item_categories_rings
        parameters:
          category: ring
          magic_requires: worn_on_finger_or_similar_digit
          description_exception_overrides: true
    contract:
      id: dnd2024:rule_magic_item_categories_rings
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
      - equipment
    requires:
    - dnd2024:rule_magic_item_categories_rings
    citations:
    - SRD 5.2.1 pp. 204; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/magic_item_categories_rings.md
  dnd2024:rules/magic_item_categories_staffs:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_categories_staffs
        implementation: dnd2024:rule_magic_item_categories_staffs
        parameters:
          category: staff
          weight_pounds_range:
          - 2
          - 7
          default_uses:
          - weapon:
              key: dnd2024:equipment/quarterstaff
            magical_weapon: false
          - focus: arcane
          description_exception_overrides: true
    contract:
      id: dnd2024:rule_magic_item_categories_staffs
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
      - equipment
    requires:
    - dnd2024:rule_magic_item_categories_staffs
    citations:
    - SRD 5.2.1 pp. 205; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/magic_item_categories_staffs.md
  dnd2024:rules/magic_item_categories_wondrous_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_categories_wondrous_items
        implementation: dnd2024:rule_magic_item_categories_wondrous_items
        parameters:
          category: wondrous_item
          attunement_prerequisites:
            class: membership_in_required_class
            spellcaster:
              minimum_spells: 1
              allowed_sources:
              - traits
              - features
              excluded_sources:
              - magic_items
              - similar_external_sources
          wearable_size_adjustment:
            default: true
            gm_can_disable: true
            gm_fit_restrictions:
            - body_size
            - body_shape
          unusual_anatomy_fit:
            actor: gm
            scope: item_not_made_for_anatomy
            must_have_required_body_parts: true
          paired_items:
            default: both_worn
            gm_exception:
              scope: missing_body_part
              other_item_remains_on_person: true
    contract:
      id: dnd2024:rule_magic_item_categories_wondrous_items
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
      - attunement
      - equipment
    requires:
    - dnd2024:rule_magic_item_categories_wondrous_items
    citations:
    - SRD 5.2.1 pp. 205; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/magic_item_categories_wondrous_items.md
  dnd2024:rules/activating_a_magic_item:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/activating_a_magic_item
        implementation: dnd2024:rule_activating_a_magic_item
        parameters:
          default_activation_action: magic
          additional_requirements_source:
          - item_category
          - item_description
          item_description_overrides: true
    contract:
      id: dnd2024:rule_activating_a_magic_item
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
      - actions
    requires:
    - dnd2024:rule_activating_a_magic_item
    citations:
    - SRD 5.2.1 pp. 206; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/activating_a_magic_item.md
  dnd2024:rules/activating_a_magic_item_spells_cast_from_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
        implementation: dnd2024:rule_activating_a_magic_item_spells_cast_from_items
        parameters:
          spell_cast_defaults:
            spell_level: lowest_possible
            caster_level: lowest_possible
            user_spell_slots_expended: 0
            components_required: false
            casting_time: normal_spell
            range: normal_spell
            duration: normal_spell
            concentration: user_if_spell_requires
          item_specific_overrides: true
          confer_effect_without_casting:
            duration: normal_spell
            cast_event: false
          user_spellcasting_ability_if_required:
            multiple: user_choice
            none_modifier: 0
            add_user_proficiency_bonus: true
    contract:
      id: dnd2024:rule_activating_a_magic_item_spells_cast_from_items
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
      - spellcasting
    requires:
    - dnd2024:rule_activating_a_magic_item_spells_cast_from_items
    citations:
    - SRD 5.2.1 pp. 206; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/activating_a_magic_item_spells_cast_from_items.md
  dnd2024:rules/cursed_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/cursed_items
        implementation: dnd2024:rule_cursed_items
        parameters:
          curse_presence_source: item_description
          identification_reveals_curse_default: false
          identify_spell_reveals_curse: false
          voluntary_attunement_end_requires: curse_broken
          curse_removal_example:
            key: dnd2024:spells/remove_curse
    contract:
      id: dnd2024:rule_cursed_items
      inputs:
        actor: participant
        context: resolution
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
      - curses
      - attunement
    requires:
    - dnd2024:rule_cursed_items
    citations:
    - SRD 5.2.1 pp. 206; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/cursed_items.md
  dnd2024:rules/crafting_magic_items_arcana_proficiency:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/crafting_magic_items_arcana_proficiency
        implementation: dnd2024:rule_crafting_magic_items_arcana_proficiency
        parameters:
          arcana_proficiency_required_for:
          - crafter
          - all_assistants
    contract:
      id: dnd2024:rule_crafting_magic_items_arcana_proficiency
      inputs:
        actor: participant
        context: resolution
      outputs:
      - crafted_items
      events: []
      capabilities:
      - crafting
      - proficiency
    requires:
    - dnd2024:rule_crafting_magic_items_arcana_proficiency
    citations:
    - SRD 5.2.1 pp. 206; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/crafting_magic_items_arcana_proficiency.md
  dnd2024:rules/crafting_magic_items_time_and_cost:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/crafting_magic_items_time_and_cost
        implementation: dnd2024:rule_crafting_magic_items_time_and_cost
        parameters:
          rarity_table:
          - rarity: common
            days: 5
            raw_material_gp: 50
          - rarity: uncommon
            days: 10
            raw_material_gp: 200
          - rarity: rare
            days: 50
            raw_material_gp: 2000
          - rarity: very_rare
            days: 125
            raw_material_gp: 20000
          - rarity: legendary
            days: 250
            raw_material_gp: 100000
          work_hours_per_day: 8
          consecutive_days_required: false
          assistants:
            default_maximum: 1
            gm_can_allow_more: true
            time_divisor: total_workers
          consumables:
            time_multiplier: 0.5
            cost_multiplier: 0.5
            exclude: spell_scroll
            spell_scroll_rules: equipment_scribing_rules
          raw_material_availability:
            determined_by: gm
            city_chance_percent: 75
            other_settlement_chance_percent: 25
            failed_check_retry_minimum_days: 7
          incorporated_mundane_item:
            additional_requirement_choice:
            - pay_entire_purchase_cost
            - craft_under_nonmagical_rules
          plate_plus_one_example:
            buy_plate_and_enchant_gp: 3500
            craft_plate_and_enchant_gp: 2000
            plate_crafting_required_for_second_option: true
    contract:
      id: dnd2024:rule_crafting_magic_items_time_and_cost
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - crafted_items
      events: []
      capabilities:
      - crafting
      - economy
    requires:
    - dnd2024:rule_crafting_magic_items_time_and_cost
    citations:
    - SRD 5.2.1 pp. 207; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/crafting_magic_items_time_and_cost.md
  dnd2024:rules/sentient_magic_items_abilities:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sentient_magic_items_abilities
        implementation: dnd2024:rule_sentient_magic_items_abilities
        parameters:
          abilities:
          - intelligence
          - wisdom
          - charisma
          generation_choice:
          - gm_select
          - random
          random_each:
            dice_count: 4
            die_sides: 6
            drop_lowest: 1
            sum_remaining: true
    contract:
      id: dnd2024:rule_sentient_magic_items_abilities
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
      - random_tables
    requires:
    - dnd2024:rule_sentient_magic_items_abilities
    citations:
    - SRD 5.2.1 pp. 207; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/sentient_magic_items_abilities.md
  dnd2024:rules/sentient_magic_items_senses:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sentient_magic_items_senses
        implementation: dnd2024:rule_sentient_magic_items_senses
        parameters:
          generation_choice:
          - gm_select
          - random
          roll:
            dice:
              count: 1
              sides: 4
          results:
          - roll: 1
            hearing_feet: 30
            standard_vision_feet: 30
          - roll: 2
            hearing_feet: 60
            standard_vision_feet: 60
          - roll: 3
            hearing_feet: 120
            standard_vision_feet: 120
          - roll: 4
            hearing_feet: 120
            darkvision_feet: 120
    contract:
      id: dnd2024:rule_sentient_magic_items_senses
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - item_changes
      events: []
      capabilities:
      - magic_items
      - senses
      - random_tables
    requires:
    - dnd2024:rule_sentient_magic_items_senses
    citations:
    - SRD 5.2.1 pp. 207; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/sentient_magic_items_senses.md
    - SRD 5.2.1 p. 208, Sentient Item’s Senses table; builtin/srd-5.2.1/source.txt lines 13315–13321
  dnd2024:rules/monsters:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 254; builtin/dnd2024-definitions/rules/monsters.md
  dnd2024:rules/parts_of_a_stat_block_size:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_size
        implementation: dnd2024:rule_parts_of_a_stat_block_size
        parameters:
          size_options:
          - tiny
          - small
          - medium
          - large
          - huge
          - gargantuan
          multiple_stat_block_sizes:
            selector: gm
            constraint: listed_options
          size_mechanics:
            key: dnd2024:glossary/size
    contract:
      id: dnd2024:rule_parts_of_a_stat_block_size
      inputs:
        actor: participant
        context: resolution
        selection:
          type: choice
          required: false
      outputs:
      - monster_parameters
      events: []
      capabilities:
      - monsters
      - statistics
    requires:
    - dnd2024:rule_parts_of_a_stat_block_size
    citations:
    - SRD 5.2.1 pp. 254; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/parts_of_a_stat_block_size.md
  dnd2024:rules/parts_of_a_stat_block_alignment:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 254; builtin/dnd2024-definitions/rules/parts_of_a_stat_block_alignment.md
  dnd2024:rules/parts_of_a_stat_block_hit_points:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_hit_points
        implementation: dnd2024:rule_parts_of_a_stat_block_hit_points
        parameters:
          hit_point_generation_choice:
          - printed_number
          - roll_printed_expression
          combine_methods: false
          constitution_contribution:
            op: multiply
            args:
            - variable: constitution_modifier
            - variable: hit_dice_count
          typical_hit_die_by_size:
          - size: tiny
            die_sides: 4
            average_per_die: 2.5
          - size: small
            die_sides: 6
            average_per_die: 3.5
          - size: medium
            die_sides: 8
            average_per_die: 4.5
          - size: large
            die_sides: 10
            average_per_die: 5.5
          - size: huge
            die_sides: 12
            average_per_die: 6.5
          - size: gargantuan
            die_sides: 20
            average_per_die: 10.5
    contract:
      id: dnd2024:rule_parts_of_a_stat_block_hit_points
      inputs:
        actor: participant
        context: resolution
        constitution_modifier: number
        hit_dice_count: number
        selection:
          type: choice
          required: false
      outputs:
      - monster_parameters
      events: []
      capabilities:
      - monsters
      - hit_points
    requires:
    - dnd2024:rule_parts_of_a_stat_block_hit_points
    citations:
    - SRD 5.2.1 pp. 255; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/parts_of_a_stat_block_hit_points.md
  dnd2024:rules/parts_of_a_stat_block_skills:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_skills
        implementation: dnd2024:rule_parts_of_a_stat_block_skills
        parameters:
          listed_skills_are_proficiencies: true
          skill_bonus:
            op: add
            args:
            - variable: ability_modifier
            - variable: proficiency_bonus
            - variable: other_modifiers
    contract:
      id: dnd2024:rule_parts_of_a_stat_block_skills
      inputs:
        actor: participant
        context: resolution
        ability_modifier: number
        other_modifiers: number
        proficiency_bonus: number
      outputs:
      - monster_parameters
      events: []
      capabilities:
      - monsters
      - proficiency
    requires:
    - dnd2024:rule_parts_of_a_stat_block_skills
    citations:
    - SRD 5.2.1 pp. 255; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/parts_of_a_stat_block_skills.md
  dnd2024:rules/parts_of_a_stat_block_gear:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_gear
        implementation: dnd2024:rule_parts_of_a_stat_block_gear
        parameters:
          monster_proficient_with_equipment: true
          retrievable_items_source: gear_entry
          monster_special_item_rules_apply_only_to_original_monster: true
          other_user_uses: equipment_definition_rules
          gear_list_not_exhaustive_for_incidental_clothing: true
          equipment_outside_gear_entry:
            supernatural_or_specialized: true
            usable_after_monster_defeat: false
    contract:
      id: dnd2024:rule_parts_of_a_stat_block_gear
      inputs:
        actor: participant
        context: resolution
      outputs:
      - monster_parameters
      events: []
      capabilities:
      - monsters
      - equipment
    requires:
    - dnd2024:rule_parts_of_a_stat_block_gear
    citations:
    - SRD 5.2.1 pp. 255; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/parts_of_a_stat_block_gear.md
  dnd2024:rules/parts_of_a_stat_block_senses:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_senses
        implementation: dnd2024:rule_parts_of_a_stat_block_senses
        parameters:
          apply_definitions:
          - key: dnd2024:glossary/passive_perception
          - key: dnd2024:glossary/blindsight
          - key: dnd2024:glossary/darkvision
          - key: dnd2024:glossary/tremorsense
          - key: dnd2024:glossary/truesight
          value_source: stat_block_senses
          grant_only_listed_senses: true
    contract:
      id: dnd2024:rule_parts_of_a_stat_block_senses
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - monsters
      - senses
    requires:
    - dnd2024:rule_parts_of_a_stat_block_senses
    citations:
    - SRD 5.2.1 pp. 256; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/parts_of_a_stat_block_senses.md
  dnd2024:rules/parts_of_a_stat_block_challenge_rating:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_challenge_rating
        implementation: dnd2024:rule_parts_of_a_stat_block_challenge_rating
        parameters:
          apply_definitions:
          - key: dnd2024:glossary/challenge_rating
    contract:
      id: dnd2024:rule_parts_of_a_stat_block_challenge_rating
      inputs:
        actor: participant
        context: resolution
      outputs:
      - resolution
      events: []
      capabilities:
      - monsters
      - encounter_building
    requires:
    - dnd2024:rule_parts_of_a_stat_block_challenge_rating
    citations:
    - SRD 5.2.1 pp. 256; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/parts_of_a_stat_block_challenge_rating.md
  dnd2024:rules/parts_of_a_stat_block_traits:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_traits
        implementation: dnd2024:rule_parts_of_a_stat_block_traits
        parameters:
          trait_activation_options:
          - always
          - specified_situation
          condition_source: trait_definition
    contract:
      id: dnd2024:rule_parts_of_a_stat_block_traits
      inputs:
        actor: participant
        context: resolution
      outputs:
      - monster_parameters
      events: []
      capabilities:
      - monsters
      - effects
    requires:
    - dnd2024:rule_parts_of_a_stat_block_traits
    citations:
    - SRD 5.2.1 pp. 256; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/parts_of_a_stat_block_traits.md
  dnd2024:rules/parts_of_a_stat_block_saving_throw_effect_notation:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_saving_throw_effect_notation
        implementation: dnd2024:rule_parts_of_a_stat_block_saving_throw_effect_notation
        parameters:
          save_fields:
          - ability
          - dc
          - target_eligibility
          - failure_effect
          - success_effect
          half_damage_only:
            damage_fraction: 0.5
            rounding: floor
            other_effects_applied: false
    contract:
      id: dnd2024:rule_parts_of_a_stat_block_saving_throw_effect_notation
      inputs:
        actor: participant
        context: resolution
      outputs:
      - monster_parameters
      events: []
      capabilities:
      - monsters
      - saving_throws
      - damage
    requires:
    - dnd2024:rule_parts_of_a_stat_block_saving_throw_effect_notation
    citations:
    - SRD 5.2.1 pp. 256; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/parts_of_a_stat_block_saving_throw_effect_notation.md
  dnd2024:rules/parts_of_a_stat_block_spellcasting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_spellcasting
        implementation: dnd2024:rule_parts_of_a_stat_block_spellcasting
        parameters:
          spellcasting_parameters_source: monster_stat_block
          leveled_spell_default:
            cast_level: lowest_possible
            upcasting: false
          specific_restrictions_apply: true
          self_only_means: monster_only
          components:
            ignore_only_if_trait_says: true
            required_materials_present: true
            required_components_observable: true
          long_casting:
            minimum_minutes: 1
            magic_action_each_turn: true
            concentration_required: true
            listing_in_action_does_not_shorten_cast: true
            explicit_action_exception_overrides: true
    contract:
      id: dnd2024:rule_parts_of_a_stat_block_spellcasting
      inputs:
        actor: participant
        context: resolution
      outputs:
      - monster_parameters
      events: []
      capabilities:
      - monsters
      - spellcasting
      - concentration
    requires:
    - dnd2024:rule_parts_of_a_stat_block_spellcasting
    citations:
    - SRD 5.2.1 pp. 257; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/parts_of_a_stat_block_spellcasting.md
  dnd2024:rules/parts_of_a_stat_block_legendary_actions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_legendary_actions
        implementation: dnd2024:rule_parts_of_a_stat_block_legendary_actions
        parameters:
          timing: immediately_after_another_creature_turn_ends
          maximum_actions_per_trigger: 1
          prohibited_if_any:
          - incapacitated
          - unable_to_take_actions
          options_source: stat_block_legendary_actions
          uses_maximum_source: stat_block
          uses_spent_per_action: 1
          recharge:
            timing: start_of_monster_turn
            amount: all_expended_uses
    contract:
      id: dnd2024:rule_parts_of_a_stat_block_legendary_actions
      inputs:
        actor: participant
        context: resolution
      outputs:
      - monster_parameters
      events:
      - turn.ended
      - turn.started
      capabilities:
      - monsters
      - legendary_actions
      - resources
    requires:
    - dnd2024:rule_parts_of_a_stat_block_legendary_actions
    citations:
    - SRD 5.2.1 pp. 257; builtin/srd-5.2.1/source.txt; builtin/dnd2024-definitions/rules/parts_of_a_stat_block_legendary_actions.md
---

# Supplementary rules: source contract findings, group 2

This document accounts for all 174 assigned notes: 147 specified contracts and 27 informational notes. Every specified entry preserves the assignment source SHA-256. No engine implementation is included.

The official bundled SRD 5.2.1 is the sole rules source. Each frontmatter entry cites its source pages and local draft. Draft spans were read in full. The original two-column `builtin/srd-5.2.1/source.txt` was used to reconstruct the starting equipment table on page 24, travel terrain table on page 192, and sentient item senses table on page 208. The complete weapon table attached to Vex refers to all 38 existing equipment definitions. Class progression and level-one feature grants refer to existing tables and feature definitions. These are definition dependencies, not requests for prose interpretation.

The detached magic-item column inside Multiclassing Prerequisites belongs to Starting Equipment at Higher Levels. It refers to that reconstructed table. The detached foraging, navigation, and search DC columns inside Choose Abilities belong to the Travel Terrain table. They refer to that reconstructed table. Senses includes its four-row table from the next page. Embedded sidebars retain their rules: exception precedence, rounding, Heroic Inspiration, marching order, grid movement, resting, level-20 bonus feats, selling equipment, armor casting, magic item fit, and identification of ongoing spells.

Parameter expressions are typed trees: `op` names arithmetic operations, `args` are operands, `variable` names a typed contract input, and `dice` provides integer count and sides. A numerator/denominator pair is an exact rational. Source-field symbols read the named feature, item, participant, or event data from the resolution context. Selection inputs encode source-authorized player or GM decisions; each parameter limits the choice to its stated scope. Geometry-dependent contracts use supplied locations and movement events. They do not claim to calculate maps.

The Trinkets table has all 100 outcomes. Appearance strings retain descriptive identity; numerical quantities and unusual behavior have separate typed properties. Pure appearance carries no invented combat benefit. The phrase “half damage only” excludes all non-damage riders. Legendary Actions spend one use per action under the 2024 source rule. Twinned Spell increases effective spell level by one only for eligible spells.

Informational dispositions cover headings, introductory text without rules, tier descriptions, and encounter or roleplaying advice without a prescribed mechanical change. No source issue remains unresolved within this assignment.

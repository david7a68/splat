---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:rules/playing_the_game:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 5; builtin/srd-5.2.1/source.txt; section rules/playing_the_game.md
  dnd2024:rules/the_six_abilities_ability_scores:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/the_six_abilities_ability_scores
        implementation: dnd2024:rules_the_six_abilities_ability_scores
        parameters:
          normal_min: 1
          adventurer_max: 20
          absolute_max: 30
          feature_can_override_adventurer_max: true
          score_zero_consequence: reducing_effect_definition
          score_bands:
          - min: 1
            max: 1
            category: normal_minimum
          - min: 2
            max: 9
            category: weak
          - min: 10
            max: 11
            category: human_average
          - min: 12
            max: 19
            category: strong
          - min: 20
            max: 20
            category: adventurer_maximum
          - min: 21
            max: 29
            category: extraordinary
          - min: 30
            max: 30
            category: absolute_maximum
    contract:
      id: dnd2024:rules_the_six_abilities_ability_scores
      inputs:
        subject: participant
      outputs:
      - ability_score_constraints
      events: []
      capabilities:
      - character_statistics
    requires:
    - dnd2024:rules_the_six_abilities_ability_scores
    citations:
    - SRD 5.2.1 pp. 5; builtin/srd-5.2.1/source.txt; section rules/the_six_abilities_ability_scores.md
  dnd2024:rules/d20_tests_ability_checks:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_ability_checks
        implementation: dnd2024:rules_d20_tests_ability_checks
        parameters:
          check: ability_check
          eligibility:
            attack: false
            meaningful_failure_possible: true
            outcome_uncertain: true
            narrative_interest: true
          call_authority:
          - gm
          - applicable_rule
          ability_and_dc: gm_or_applicable_rule
    contract:
      id: dnd2024:rules_d20_tests_ability_checks
      inputs:
        subject: participant
        task: resolution
      outputs:
      - check_request
      events: []
      capabilities:
      - checks
    requires:
    - dnd2024:rules_d20_tests_ability_checks
    citations:
    - SRD 5.2.1 pp. 6; builtin/srd-5.2.1/source.txt; section rules/d20_tests_ability_checks.md
  dnd2024:rules/d20_tests_difficulty_class:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_difficulty_class
        implementation: dnd2024:rules_d20_tests_difficulty_class
        parameters:
          dc_authority: gm
          specific_dc_priority: applicable_rule_subject_to_gm
          suggested_dc:
            very_easy: 5
            easy: 10
            medium: 15
            hard: 20
            very_hard: 25
            nearly_impossible: 30
    contract:
      id: dnd2024:rules_d20_tests_difficulty_class
      inputs:
        task: resolution
        difficulty: choice
      outputs:
      - difficulty_class
      events: []
      capabilities:
      - checks
    requires:
    - dnd2024:rules_d20_tests_difficulty_class
    citations:
    - SRD 5.2.1 pp. 6; builtin/srd-5.2.1/source.txt; section rules/d20_tests_difficulty_class.md
  dnd2024:rules/d20_tests_proficiency_bonus_2:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_proficiency_bonus_2
        implementation: dnd2024:rules_d20_tests_proficiency_bonus_2
        parameters:
          test: saving_throw
          bonus:
            when: proficient_in_save_ability
            value:
              var: subject.proficiency_bonus
    contract:
      id: dnd2024:rules_d20_tests_proficiency_bonus_2
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - saving_throw.calculate
      capabilities:
      - checks
      - proficiency
    requires:
    - dnd2024:rules_d20_tests_proficiency_bonus_2
    citations:
    - SRD 5.2.1 pp. 7; builtin/srd-5.2.1/source.txt; section rules/d20_tests_proficiency_bonus_2.md
  dnd2024:rules/d20_tests_ability_modifier_3:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_ability_modifier_3
        implementation: dnd2024:rules_d20_tests_ability_modifier_3
        parameters:
          default_abilities:
            melee_weapon: strength
            unarmed_strike: strength
            ranged_weapon: dexterity
            spell_attack: spellcasting_feature_ability
          feature_override: true
          finesse:
            choice:
            - strength
            - dexterity
    contract:
      id: dnd2024:rules_d20_tests_ability_modifier_3
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - attack.calculate
      capabilities:
      - attacks
    requires:
    - dnd2024:rules_d20_tests_ability_modifier_3
    citations:
    - SRD 5.2.1 pp. 7; builtin/srd-5.2.1/source.txt; section rules/d20_tests_ability_modifier_3.md
  dnd2024:rules/d20_tests_rolling_20_or_1:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_rolling_20_or_1
        implementation: dnd2024:rules_d20_tests_rolling_20_or_1
        parameters:
          test: attack_roll
          natural_results:
            20:
              hit: true
              critical_hit: true
              ignore_ac_and_modifiers: true
            1:
              hit: false
              ignore_ac_and_modifiers: true
    contract:
      id: dnd2024:rules_d20_tests_rolling_20_or_1
      inputs:
        attack: resolution
      outputs:
      - attack_result
      events:
      - attack.roll_resolved
      capabilities:
      - attacks
    requires:
    - dnd2024:rules_d20_tests_rolling_20_or_1
    citations:
    - SRD 5.2.1 pp. 7; builtin/srd-5.2.1/source.txt; section rules/d20_tests_rolling_20_or_1.md
  dnd2024:rules/d20_tests_they_don_t_stack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_they_don_t_stack
        implementation: dnd2024:rules_d20_tests_they_don_t_stack
        parameters:
          roll: d20
          cases:
          - advantage_sources_min: 1
            disadvantage_sources: 0
            dice: 2
            select: highest
          - advantage_sources: 0
            disadvantage_sources_min: 1
            dice: 2
            select: lowest
          - advantage_sources_min: 1
            disadvantage_sources_min: 1
            dice: 1
            advantage: false
            disadvantage: false
          - advantage_sources: 0
            disadvantage_sources: 0
            dice: 1
    contract:
      id: dnd2024:rules_d20_tests_they_don_t_stack
      inputs:
        roll: resolution
      outputs:
      - roll_configuration
      events: []
      capabilities:
      - dice
    requires:
    - dnd2024:rules_d20_tests_they_don_t_stack
    citations:
    - SRD 5.2.1 pp. 8; builtin/srd-5.2.1/source.txt; section rules/d20_tests_they_don_t_stack.md
  dnd2024:rules/proficiency_the_bonus_doesn_t_stack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/proficiency_the_bonus_doesn_t_stack
        implementation: dnd2024:rules_proficiency_the_bonus_doesn_t_stack
        parameters:
          proficiency_additions_max: 1
          proficiency_multiplications_max: 1
          proficiency_divisions_max: 1
          alternative_skill_proficiencies: any_qualifying_skill
          multiple_qualifying_proficiencies: one_bonus
    contract:
      id: dnd2024:rules_proficiency_the_bonus_doesn_t_stack
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - check.calculate
      capabilities:
      - proficiency
    requires:
    - dnd2024:rules_proficiency_the_bonus_doesn_t_stack
    citations:
    - SRD 5.2.1 pp. 8; builtin/srd-5.2.1/source.txt; section rules/proficiency_the_bonus_doesn_t_stack.md
  dnd2024:rules/proficiency_determining_skills:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/proficiency_determining_skills
        implementation: dnd2024:rules_proficiency_determining_skills
        parameters:
          skill_proficiencies_source:
            player_character: character_creation_choices
            monster: stat_block
    contract:
      id: dnd2024:rules_proficiency_determining_skills
      inputs:
        subject: participant
      outputs:
      - skill_proficiencies
      events: []
      capabilities:
      - proficiency
    requires:
    - dnd2024:rules_proficiency_determining_skills
    citations:
    - SRD 5.2.1 pp. 9; builtin/srd-5.2.1/source.txt; section rules/proficiency_determining_skills.md
  dnd2024:rules/actions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/actions
        implementation: dnd2024:rules_actions
        parameters:
          main_actions:
          - key: dnd2024:glossary/attack_action
          - key: dnd2024:glossary/dash_action
          - key: dnd2024:glossary/disengage_action
          - key: dnd2024:glossary/dodge_action
          - key: dnd2024:glossary/help_action
          - key: dnd2024:glossary/hide_action
          - key: dnd2024:glossary/influence_action
          - key: dnd2024:glossary/magic_action
          - key: dnd2024:glossary/ready_action
          - key: dnd2024:glossary/search_action
          - key: dnd2024:glossary/study_action
          - key: dnd2024:glossary/utilize_action
          default_action_required_except:
          - movement
          - communication
          additional_actions_source:
          - class_features
          - other_abilities
          improvised_action:
            authority: gm
            choices:
              possible: boolean
              test: optional_d20_test
    contract:
      id: dnd2024:rules_actions
      inputs:
        subject: participant
        proposed_action: resolution
      outputs:
      - action_options
      - action_adjudication
      events: []
      capabilities:
      - actions
      - checks
    requires:
    - dnd2024:rules_actions
    citations:
    - SRD 5.2.1 pp. 9, 10; builtin/srd-5.2.1/source.txt; section rules/actions.md
  dnd2024:rules/actions_reactions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/actions_reactions
        implementation: dnd2024:rules_actions_reactions
        parameters:
          requires_grant_and_trigger: true
          allowed_turn: any
          uses:
            max: 1
            reset: subject_turn_start
          default_timing: immediately_after_trigger
          specific_timing_overrides_default: true
          interrupted_turn: resume_after_reaction
    contract:
      id: dnd2024:rules_actions_reactions
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - reaction.triggered
      - turn.started
      capabilities:
      - action_economy
    requires:
    - dnd2024:rules_actions_reactions
    citations:
    - SRD 5.2.1 pp. 10; builtin/srd-5.2.1/source.txt; section rules/actions_reactions.md
  dnd2024:rules/social_interaction_ability_checks:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/social_interaction_ability_checks
        implementation: dnd2024:rules_social_interaction_ability_checks
        parameters:
          roleplaying_attitude_change:
            authority: gm
            attitude: choice
          uncertain_response:
            authority: gm
            roll_optional: true
            action:
              key: dnd2024:glossary/influence_action
    contract:
      id: dnd2024:rules_social_interaction_ability_checks
      inputs:
        subject: participant
      outputs:
      - attitude
      - check_request
      events: []
      capabilities:
      - social_interaction
    requires:
    - dnd2024:rules_social_interaction_ability_checks
    citations:
    - SRD 5.2.1 pp. 11; builtin/srd-5.2.1/source.txt; section rules/social_interaction_ability_checks.md
  dnd2024:rules/exploration_vision_and_light:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 11; builtin/srd-5.2.1/source.txt; section rules/exploration_vision_and_light.md
  dnd2024:rules/exploration_special_senses:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_special_senses
        implementation: dnd2024:rules_exploration_special_senses
        parameters:
          senses:
          - key: dnd2024:glossary/blindsight
          - key: dnd2024:glossary/darkvision
          - key: dnd2024:glossary/tremorsense
          - key: dnd2024:glossary/truesight
    contract:
      id: dnd2024:rules_exploration_special_senses
      inputs:
        subject: participant
      outputs:
      - sense_definitions
      events: []
      capabilities:
      - senses
    requires:
    - dnd2024:rules_exploration_special_senses
    citations:
    - SRD 5.2.1 pp. 11; builtin/srd-5.2.1/source.txt; section rules/exploration_special_senses.md
  dnd2024:rules/exploration_what_is_an_object:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_what_is_an_object
        implementation: dnd2024:rules_exploration_what_is_an_object
        parameters:
          object:
            discrete: true
            inanimate: true
          composite_excluded:
          - building
          - vehicle
          composite_parts_may_be_objects: true
    contract:
      id: dnd2024:rules_exploration_what_is_an_object
      inputs:
        candidate: object
      outputs:
      - object_classification
      events: []
      capabilities:
      - objects
    requires:
    - dnd2024:rules_exploration_what_is_an_object
    citations:
    - SRD 5.2.1 pp. 12; builtin/srd-5.2.1/source.txt; section rules/exploration_what_is_an_object.md
  dnd2024:rules/exploration_carrying_objects:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_carrying_objects
        implementation: dnd2024:rules_exploration_carrying_objects
        parameters:
          ordinary_gear_weight_tracking: optional
          exceptional_load:
            triggers:
            - unusually_heavy_object
            - massive_object_count
            authority: gm
            enforce:
              key: dnd2024:glossary/carrying_capacity
    contract:
      id: dnd2024:rules_exploration_carrying_objects
      inputs:
        subject: participant
      outputs:
      - effects
      events: []
      capabilities:
      - inventory
      - carrying_capacity
    requires:
    - dnd2024:rules_exploration_carrying_objects
    citations:
    - SRD 5.2.1 pp. 12; builtin/srd-5.2.1/source.txt; section rules/exploration_carrying_objects.md
  dnd2024:rules/exploration_travel:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_travel
        implementation: dnd2024:rules_exploration_travel
        parameters:
          long_distance_resolution:
            authority: gm
            choices:
            - summarized
            - travel_pace
          time_critical_resolution: combat_movement
          travel_pace_rule:
            key: dnd2024:rules/exploration_travel_pace
    contract:
      id: dnd2024:rules_exploration_travel
      inputs:
        subject: participant
      outputs:
      - travel_mode
      events: []
      capabilities:
      - travel
    requires:
    - dnd2024:rules_exploration_travel
    citations:
    - SRD 5.2.1 pp. 12; builtin/srd-5.2.1/source.txt; section rules/exploration_travel.md
  dnd2024:rules/combat:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 13; builtin/srd-5.2.1/source.txt; section rules/combat.md
  dnd2024:rules/combat_initiative:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_initiative
        implementation: dnd2024:rules_combat_initiative
        parameters:
          check:
            ability: dexterity
            kind: ability_check
          roll_controller:
            player_character: player
            monster: gm
          identical_monster_group: one_shared_roll
          surprised:
            initiative_disadvantage: true
          order: descending_check_total
          order_persists_each_round: true
          ties:
            monsters_only: gm_choice
            characters_only: players_choice
            mixed: gm_choice
    contract:
      id: dnd2024:rules_combat_initiative
      inputs:
        combatants: participant_list
        surprised: participant_list
        ties:
          type: choice
          required: false
      outputs:
      - initiative_order
      events:
      - combat.started
      capabilities:
      - initiative
      - checks
    requires:
    - dnd2024:rules_combat_initiative
    citations:
    - SRD 5.2.1 pp. 13; builtin/srd-5.2.1/source.txt; section rules/combat_initiative.md
  dnd2024:rules/combat_movement_and_position:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_movement_and_position
        implementation: dnd2024:rules_combat_movement_and_position
        parameters:
          on_own_turn: true
          movement_budget:
            var: subject.speed
          minimum_distance: 0
          mixed_modes_allowed:
          - walk
          - climb
          - crawl
          - jump
          - swim
          spend_budget_per_segment: true
          speed_source:
            character: character_creation
            monster: stat_block
          special_speed_modes:
          - climb
          - fly
          - swim
          movement_mode_rules:
          - key: dnd2024:glossary/climbing
          - key: dnd2024:glossary/crawling
          - key: dnd2024:glossary/jumping
          - key: dnd2024:glossary/swimming
          - key: dnd2024:glossary/speed
    contract:
      id: dnd2024:rules_combat_movement_and_position
      inputs:
        subject: participant
        segments: resolution
      outputs:
      - movement_remaining
      - position
      events:
      - movement.requested
      capabilities:
      - movement
    requires:
    - dnd2024:rules_combat_movement_and_position
    citations:
    - SRD 5.2.1 pp. 14; builtin/srd-5.2.1/source.txt; section rules/combat_movement_and_position.md
  dnd2024:rules/combat_dropping_prone:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_dropping_prone
        implementation: dnd2024:rules_combat_dropping_prone
        parameters:
          timing: own_turn
          requires_speed_greater_than: 0
          condition: prone
          action_cost: 0
          movement_cost: 0
          optional: true
    contract:
      id: dnd2024:rules_combat_dropping_prone
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - condition.voluntarily_applied
      capabilities:
      - movement
      - conditions
    requires:
    - dnd2024:rules_combat_dropping_prone
    citations:
    - SRD 5.2.1 pp. 14; builtin/srd-5.2.1/source.txt; section rules/combat_dropping_prone.md
  dnd2024:rules/combat_making_an_attack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_making_an_attack
        implementation: dnd2024:rules_combat_making_an_attack
        parameters:
          target:
            kinds:
            - creature
            - object
            - location
            within_attack_range: true
          sequence:
          - choose_target
          - determine_modifiers
          - resolve_attack_roll
          - resolve_hit_or_miss
          modifiers:
          - cover
          - advantage
          - disadvantage
          - spell_bonuses
          - feature_bonuses
          - effect_penalties
          default_hit: roll_damage
          attack_specific_effects:
            may_replace_damage: true
            may_add_to_damage: true
          unseen_target:
            disadvantage: true
            wrong_target_location: automatic_miss
          unseen_attacker:
            advantage: true
          hidden_attacker:
            reveal_location: on_hit_or_miss
    contract:
      id: dnd2024:rules_combat_making_an_attack
      inputs:
        attacker: participant
        target: resolution
        attack: definition
      outputs:
      - attack_result
      - damage
      - effects
      - revealed_location
      events:
      - attack.requested
      capabilities:
      - attacks
      - senses
      - damage
    requires:
    - dnd2024:rules_combat_making_an_attack
    citations:
    - SRD 5.2.1 pp. 14, 15; builtin/srd-5.2.1/source.txt; section rules/combat_making_an_attack.md
  dnd2024:rules/combat_range:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_range
        implementation: dnd2024:rules_combat_range
        parameters:
          single_range:
            maximum: listed_range
          two_ranges:
            within_normal: normal_roll
            beyond_normal_through_long: disadvantage
            beyond_long: prohibited
    contract:
      id: dnd2024:rules_combat_range
      inputs:
        attack: resolution
        distance: number
      outputs:
      - attack_eligibility
      - roll_modifiers
      events: []
      capabilities:
      - attacks
      - geometry
    requires:
    - dnd2024:rules_combat_range
    citations:
    - SRD 5.2.1 pp. 15; builtin/srd-5.2.1/source.txt; section rules/combat_range.md
  dnd2024:rules/combat_reach:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_reach
        implementation: dnd2024:rules_combat_reach
        parameters:
          default_reach_ft: 5
          attack_requires_within_reach: true
          specific_attack_reach_overrides: true
    contract:
      id: dnd2024:rules_combat_reach
      inputs:
        subject: participant
      outputs:
      - melee_reach
      events: []
      capabilities:
      - attacks
      - geometry
    requires:
    - dnd2024:rules_combat_reach
    citations:
    - SRD 5.2.1 pp. 15; builtin/srd-5.2.1/source.txt; section rules/combat_reach.md
  dnd2024:rules/combat_mounting_and_dismounting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_mounting_and_dismounting
        implementation: dnd2024:rules_combat_mounting_and_dismounting
        parameters:
          timing: during_movement
          mount_distance_max_ft: 5
          choices:
          - mount
          - dismount
          movement_cost:
            op: floor
            args:
            - op: divide
              args:
              - var: subject.speed
              - 2
    contract:
      id: dnd2024:rules_combat_mounting_and_dismounting
      inputs:
        subject: participant
        mount: participant
        mode: choice
      outputs:
      - mounted_state
      - movement_remaining
      events: []
      capabilities:
      - mounts
      - movement
    requires:
    - dnd2024:rules_combat_mounting_and_dismounting
    citations:
    - SRD 5.2.1 pp. 15; builtin/srd-5.2.1/source.txt; section rules/combat_mounting_and_dismounting.md
  dnd2024:rules/combat_underwater_combat:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 16; builtin/srd-5.2.1/source.txt; section rules/combat_underwater_combat.md
  dnd2024:rules/damage_and_healing:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 16; builtin/srd-5.2.1/source.txt; section rules/damage_and_healing.md
  dnd2024:rules/damage_and_healing_critical_hits:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_critical_hits
        implementation: dnd2024:rules_damage_and_healing_critical_hits
        parameters:
          critical_hit:
            damage_dice_multiplier: 2
            include_additional_attack_dice: true
            flat_modifiers_multiplier: 1
    contract:
      id: dnd2024:rules_damage_and_healing_critical_hits
      inputs:
        attack: resolution
      outputs:
      - damage_roll
      events:
      - attack.critical_hit
      capabilities:
      - damage
    requires:
    - dnd2024:rules_damage_and_healing_critical_hits
    citations:
    - SRD 5.2.1 pp. 16; builtin/srd-5.2.1/source.txt; section rules/damage_and_healing_critical_hits.md
  dnd2024:rules/damage_and_healing_half_damage:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_half_damage
        implementation: dnd2024:rules_damage_and_healing_half_damage
        parameters:
          saving_throw_success_damage:
            op: floor
            args:
            - op: divide
              args:
              - var: failed_save_damage
              - 2
    contract:
      id: dnd2024:rules_damage_and_healing_half_damage
      inputs:
        failed_save_damage: integer
      outputs:
      - damage
      events: []
      capabilities:
      - damage
    requires:
    - dnd2024:rules_damage_and_healing_half_damage
    citations:
    - SRD 5.2.1 pp. 16; builtin/srd-5.2.1/source.txt; section rules/damage_and_healing_half_damage.md
  dnd2024:rules/damage_and_healing_no_stacking:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_no_stacking
        implementation: dnd2024:rules_damage_and_healing_no_stacking
        parameters:
          stacking_key: damage_type
          resistance_applications_max: 1
          vulnerability_applications_max: 1
          broad_and_specific_overlap: one_application
    contract:
      id: dnd2024:rules_damage_and_healing_no_stacking
      inputs:
        damage: resolution
      outputs:
      - damage
      events: []
      capabilities:
      - damage
    requires:
    - dnd2024:rules_damage_and_healing_no_stacking
    citations:
    - SRD 5.2.1 pp. 17; builtin/srd-5.2.1/source.txt; section rules/damage_and_healing_no_stacking.md
  dnd2024:rules/damage_and_healing_healing:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_healing
        implementation: dnd2024:rules_damage_and_healing_healing
        parameters:
          healing:
            new_hp:
              op: min
              args:
              - var: subject.hp_max
              - op: add
                args:
                - var: subject.hp
                - var: healing
            discard_excess: true
          knock_out:
            optional: true
            trigger:
              melee_attack: true
              would_reduce_target_hp_to: 0
            replace_hp_with: 1
            condition: unconscious
            start_short_rest: true
            ends:
            - short_rest_finished
            - any_hp_regained
            - successful_first_aid
            first_aid:
              action_cost: 1
              ability: wisdom
              skill: medicine
              dc: 10
    contract:
      id: dnd2024:rules_damage_and_healing_healing
      inputs:
        subject: participant
        healing:
          type: integer
          required: false
        attack:
          type: resolution
          required: false
      outputs:
      - hit_points
      - conditions
      - rest_state
      events:
      - healing.applied
      - attack.damage_pending
      capabilities:
      - healing
      - conditions
      - rests
    requires:
    - dnd2024:rules_damage_and_healing_healing
    citations:
    - SRD 5.2.1 pp. 17; builtin/srd-5.2.1/source.txt; section rules/damage_and_healing_healing.md
  dnd2024:rules/damage_and_healing_character_demise:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 17; builtin/srd-5.2.1/source.txt; section rules/damage_and_healing_character_demise.md
  dnd2024:rules/damage_and_healing_stabilizing_a_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_stabilizing_a_character
        implementation: dnd2024:rules_damage_and_healing_stabilizing_a_character
        parameters:
          action: help
          requires_target_hp: 0
          check:
            ability: wisdom
            skill: medicine
            dc: 10
          success:
            stable: true
            death_saves: false
            unconscious: true
            hp: 0
          stable_damage:
            stable: false
            resume_death_saves: true
          stable_without_healing:
            after:
              dice:
                count: 1
                sides: 4
              unit: hour
            regain_hp: 1
    contract:
      id: dnd2024:rules_damage_and_healing_stabilizing_a_character
      inputs:
        subject: participant
        target: participant
      outputs:
      - stability
      - hit_points
      - conditions
      events:
      - action.help
      - damage.applied
      - duration.expired
      capabilities:
      - death_saves
      - healing
      - conditions
    requires:
    - dnd2024:rules_damage_and_healing_stabilizing_a_character
    citations:
    - SRD 5.2.1 pp. 18; builtin/srd-5.2.1/source.txt; section rules/damage_and_healing_stabilizing_a_character.md
  dnd2024:rules/damage_and_healing_duration:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_duration
        implementation: dnd2024:rules_damage_and_healing_duration
        parameters:
          resource: temporary_hit_points
          expires:
          - depleted
          - long_rest_finished
    contract:
      id: dnd2024:rules_damage_and_healing_duration
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - rest.long_finished
      - temporary_hit_points.depleted
      capabilities:
      - temporary_hit_points
    requires:
    - dnd2024:rules_damage_and_healing_duration
    citations:
    - SRD 5.2.1 pp. 18; builtin/srd-5.2.1/source.txt; section rules/damage_and_healing_duration.md
  dnd2024:rules/character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 19; builtin/srd-5.2.1/source.txt; section rules/character.md
  dnd2024:rules/create_your_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character
        implementation: dnd2024:rules_create_your_character
        parameters:
          sequence:
          - choose_class
          - choose_background
          - choose_species
          - choose_languages
          - assign_six_ability_scores
          - choose_alignment
          - fill_character_details
          class_count_at_creation: 1
          origin_components:
          - background
          - species
    contract:
      id: dnd2024:rules_create_your_character
      inputs:
        subject: participant
      outputs:
      - creation_steps
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rules_create_your_character
    citations:
    - SRD 5.2.1 pp. 19; builtin/srd-5.2.1/source.txt; section rules/create_your_character.md
  dnd2024:rules/create_your_character_note_armor_training:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_note_armor_training
        implementation: dnd2024:rules_create_your_character_note_armor_training
        parameters:
          grant_source: class_definition
          armor_training: record_granted_categories
          training_enables_effective_armor_use: true
    contract:
      id: dnd2024:rules_create_your_character_note_armor_training
      inputs:
        subject: participant
      outputs:
      - armor_training
      events: []
      capabilities:
      - character_creation
      - equipment
    requires:
    - dnd2024:rules_create_your_character_note_armor_training
    citations:
    - SRD 5.2.1 pp. 19; builtin/srd-5.2.1/source.txt; section rules/create_your_character_note_armor_training.md
  dnd2024:rules/create_your_character_choose_starting_equipment:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_choose_starting_equipment
        implementation: dnd2024:rules_create_your_character_choose_starting_equipment
        parameters:
          equipment_sources:
          - class
          - background
          starting_coins:
            may_spend_immediately: true
            purchases: equipment_catalog
            deduct_cost: true
          record:
          - chosen_equipment
          - remaining_coins
    contract:
      id: dnd2024:rules_create_your_character_choose_starting_equipment
      inputs:
        subject: participant
      outputs:
      - inventory
      - coins
      events: []
      capabilities:
      - character_creation
      - inventory
    requires:
    - dnd2024:rules_create_your_character_choose_starting_equipment
    citations:
    - SRD 5.2.1 pp. 20; builtin/srd-5.2.1/source.txt; section rules/create_your_character_choose_starting_equipment.md
  dnd2024:rules/create_your_character_choose_languages:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_choose_languages
        implementation: dnd2024:rules_create_your_character_choose_languages
        parameters:
          granted_languages:
          - common
          additional_standard_languages:
            count: 2
            selection:
            - choose
            - roll
            dice:
              count: 1
              sides: 12
            table:
            - min: 1
              max: 1
              language: common_sign_language
            - min: 2
              max: 2
              language: draconic
            - min: 3
              max: 4
              language: dwarvish
            - min: 5
              max: 6
              language: elvish
            - min: 7
              max: 7
              language: giant
            - min: 8
              max: 8
              language: gnomish
            - min: 9
              max: 9
              language: goblin
            - min: 10
              max: 11
              language: halfling
            - min: 12
              max: 12
              language: orc
          knowledge:
          - communicate
          - read
          - write
          additional_grants_source: features
          rare_requires_feature: true
          rare_languages:
          - abyssal
          - celestial
          - deep_speech
          - druidic
          - infernal
          - primordial
          - sylvan
          - thieves_cant
          - undercommon
          primordial:
            dialects:
            - aquan
            - auran
            - ignan
            - terran
            mutually_intelligible: true
    contract:
      id: dnd2024:rules_create_your_character_choose_languages
      inputs:
        subject: participant
      outputs:
      - languages
      events: []
      capabilities:
      - character_creation
      - languages
    requires:
    - dnd2024:rules_create_your_character_choose_languages
    citations:
    - SRD 5.2.1 pp. 20; builtin/srd-5.2.1/source.txt; section rules/create_your_character_choose_languages.md
  dnd2024:rules/create_your_character_assign_ability_scores:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_assign_ability_scores
        implementation: dnd2024:rules_create_your_character_assign_ability_scores
        parameters:
          assign_generated_scores:
            bijection: true
            abilities:
            - strength
            - dexterity
            - constitution
            - intelligence
            - wisdom
            - charisma
          ability_modifier:
            op: floor
            args:
            - op: divide
              args:
              - op: subtract
                args:
                - var: ability_score
                - 10
              - 2
          suggested_standard_array_assignments:
            barbarian:
              strength: 15
              dexterity: 13
              constitution: 14
              intelligence: 10
              wisdom: 12
              charisma: 8
            bard:
              strength: 8
              dexterity: 14
              constitution: 12
              intelligence: 13
              wisdom: 10
              charisma: 15
            cleric:
              strength: 14
              dexterity: 8
              constitution: 13
              intelligence: 10
              wisdom: 15
              charisma: 12
            druid:
              strength: 8
              dexterity: 12
              constitution: 14
              intelligence: 13
              wisdom: 15
              charisma: 10
            fighter:
              strength: 15
              dexterity: 14
              constitution: 13
              intelligence: 8
              wisdom: 10
              charisma: 12
            monk:
              strength: 12
              dexterity: 15
              constitution: 13
              intelligence: 10
              wisdom: 14
              charisma: 8
            paladin:
              strength: 15
              dexterity: 10
              constitution: 13
              intelligence: 8
              wisdom: 12
              charisma: 14
            ranger:
              strength: 12
              dexterity: 15
              constitution: 13
              intelligence: 8
              wisdom: 14
              charisma: 10
            rogue:
              strength: 12
              dexterity: 15
              constitution: 13
              intelligence: 14
              wisdom: 10
              charisma: 8
            sorcerer:
              strength: 10
              dexterity: 13
              constitution: 14
              intelligence: 8
              wisdom: 12
              charisma: 15
            warlock:
              strength: 8
              dexterity: 14
              constitution: 13
              intelligence: 12
              wisdom: 10
              charisma: 15
            wizard:
              strength: 8
              dexterity: 12
              constitution: 13
              intelligence: 15
              wisdom: 14
              charisma: 10
    contract:
      id: dnd2024:rules_create_your_character_assign_ability_scores
      inputs:
        subject: participant
      outputs:
      - ability_scores
      - ability_modifiers
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rules_create_your_character_assign_ability_scores
    citations:
    - SRD 5.2.1 pp. 21; builtin/srd-5.2.1/source.txt; section rules/create_your_character_assign_ability_scores.md
  dnd2024:rules/create_your_character_step_4_alignment:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_step_4_alignment
        implementation: dnd2024:rules_create_your_character_step_4_alignment
        parameters:
          choose_count: 1
          choices:
          - lawful_good
          - neutral_good
          - chaotic_good
          - lawful_neutral
          - neutral
          - chaotic_neutral
          - lawful_evil
          - neutral_evil
          - chaotic_evil
          default_evil_allowed: false
          evil_exception_authority: gm
    contract:
      id: dnd2024:rules_create_your_character_step_4_alignment
      inputs:
        subject: participant
      outputs:
      - alignment
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rules_create_your_character_step_4_alignment
    citations:
    - SRD 5.2.1 pp. 21; builtin/srd-5.2.1/source.txt; section rules/create_your_character_step_4_alignment.md
  dnd2024:rules/create_your_character_record_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_record_class_features
        implementation: dnd2024:rules_create_your_character_record_class_features
        parameters:
          class_feature_level: 1
          resolve_all_feature_choices: true
          features_source: selected_class_feature_table
    contract:
      id: dnd2024:rules_create_your_character_record_class_features
      inputs:
        subject: participant
      outputs:
      - class_features
      - feature_choices
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rules_create_your_character_record_class_features
    citations:
    - SRD 5.2.1 pp. 22; builtin/srd-5.2.1/source.txt; section rules/create_your_character_record_class_features.md
  dnd2024:rules/level_advancement_gaining_a_level:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/level_advancement_gaining_a_level
        implementation: dnd2024:rules_level_advancement_gaining_a_level
        parameters:
          sequence:
          - choose_class
          - increase_hp_and_hit_dice
          - grant_class_features
          - update_proficiency
          - update_ability_modifiers
          class_choice:
          - existing
          - eligible_multiclass
          hit_die_gain: 1
          hp_gain:
            mode:
            - roll_class_hit_die
            - fixed_class_value
            constitution_modifier_added: true
            minimum: 1
            fixed:
              barbarian: 7
              bard: 5
              cleric: 5
              druid: 5
              fighter: 6
              monk: 5
              paladin: 6
              ranger: 6
              rogue: 5
              sorcerer: 4
              warlock: 5
              wizard: 4
          grant_features_at: new_class_level
          resolve_feature_choices: true
          proficiency_source: total_character_level
          propagate_stat_changes: true
          constitution_hp_adjustment:
            op: multiply
            args:
            - var: constitution_modifier_increase
            - var: total_character_level
    contract:
      id: dnd2024:rules_level_advancement_gaining_a_level
      inputs:
        subject: participant
      outputs:
      - character_statistics
      - features
      - hit_dice
      events:
      - character.level_gained
      capabilities:
      - level_advancement
    requires:
    - dnd2024:rules_level_advancement_gaining_a_level
    citations:
    - SRD 5.2.1 pp. 23; builtin/srd-5.2.1/source.txt; section rules/level_advancement_gaining_a_level.md
  dnd2024:rules/level_advancement_tier_2_levels_5_10:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 24; builtin/srd-5.2.1/source.txt; section rules/level_advancement_tier_2_levels_5_10.md
  dnd2024:rules/starting_at_higher_levels:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/starting_at_higher_levels
        implementation: dnd2024:rules_starting_at_higher_levels
        parameters:
          starting_level:
            authority: gm
            minimum: 1
          experienced_group_recommendation:
            level: 3
            binding: false
    contract:
      id: dnd2024:rules_starting_at_higher_levels
      inputs:
        campaign: campaign
        level: integer
      outputs:
      - starting_level
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rules_starting_at_higher_levels
    citations:
    - SRD 5.2.1 pp. 24; builtin/srd-5.2.1/source.txt; section rules/starting_at_higher_levels.md
  dnd2024:rules/multiclassing:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/multiclassing
        implementation: dnd2024:rules_multiclassing
        parameters:
          on_level_gain_choose:
          - advance_existing_class
          - gain_new_class
          multiple_classes_allowed: true
    contract:
      id: dnd2024:rules_multiclassing
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - character.level_gained
      capabilities:
      - multiclassing
    requires:
    - dnd2024:rules_multiclassing
    citations:
    - SRD 5.2.1 pp. 24; builtin/srd-5.2.1/source.txt; section rules/multiclassing.md
  dnd2024:rules/multiclassing_hit_points_and_hit_point_dice:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/multiclassing_hit_points_and_hit_point_dice
        implementation: dnd2024:rules_multiclassing_hit_points_and_hit_point_dice
        parameters:
          class_level_1_maximum_hit_die_hp_only_if:
            total_character_level: 1
          new_class_at_later_character_level: normal_level_gain_hp
          hit_dice:
            sum_all_classes: true
            pool_by_die_size: true
            track_different_die_sizes_separately: true
    contract:
      id: dnd2024:rules_multiclassing_hit_points_and_hit_point_dice
      inputs:
        subject: participant
      outputs:
      - hit_points
      - hit_dice
      events: []
      capabilities:
      - multiclassing
    requires:
    - dnd2024:rules_multiclassing_hit_points_and_hit_point_dice
    citations:
    - SRD 5.2.1 pp. 25; builtin/srd-5.2.1/source.txt; section rules/multiclassing_hit_points_and_hit_point_dice.md
  dnd2024:rules/multiclassing_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/multiclassing_class_features
        implementation: dnd2024:rules_multiclassing_class_features
        parameters:
          gain_features_for: new_class_level
          apply_class_multiclass_rules: true
          special_handling:
          - extra_attack
          - spellcasting
          - alternative_ac_calculations
          special_handling_rules:
          - key: dnd2024:rules/multiclassing_extra_attack
          - key: dnd2024:rules/multiclassing_spellcasting
          - key: dnd2024:rules/multiclassing_armor_class
    contract:
      id: dnd2024:rules_multiclassing_class_features
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - character.level_gained
      capabilities:
      - multiclassing
    requires:
    - dnd2024:rules_multiclassing_class_features
    citations:
    - SRD 5.2.1 pp. 25; builtin/srd-5.2.1/source.txt; section rules/multiclassing_class_features.md
  dnd2024:rules/multiclassing_spellcasting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/multiclassing_spellcasting
        implementation: dnd2024:rules_multiclassing_spellcasting
        parameters:
          multiclass_table_requires_spellcasting_classes_min: 2
          single_spellcasting_class: use_class_slots
          spell_preparation:
            separate_by_class: true
            use_only_class_level: true
            spell_associated_with_class: true
            casting_ability: associated_class_ability
          cantrip_scaling: total_character_level_unless_spell_overrides
          effective_caster_level:
            full_level_classes:
            - bard
            - cleric
            - druid
            - sorcerer
            - wizard
            half_level_classes:
            - paladin
            - ranger
            half_rounding: up
            round_each_class_before_sum: true
          slots_by_effective_level:
          - level: 1
            slots:
              1: 2
              2: 0
              3: 0
              4: 0
              5: 0
              6: 0
              7: 0
              8: 0
              9: 0
          - level: 2
            slots:
              1: 3
              2: 0
              3: 0
              4: 0
              5: 0
              6: 0
              7: 0
              8: 0
              9: 0
          - level: 3
            slots:
              1: 4
              2: 2
              3: 0
              4: 0
              5: 0
              6: 0
              7: 0
              8: 0
              9: 0
          - level: 4
            slots:
              1: 4
              2: 3
              3: 0
              4: 0
              5: 0
              6: 0
              7: 0
              8: 0
              9: 0
          - level: 5
            slots:
              1: 4
              2: 3
              3: 2
              4: 0
              5: 0
              6: 0
              7: 0
              8: 0
              9: 0
          - level: 6
            slots:
              1: 4
              2: 3
              3: 3
              4: 0
              5: 0
              6: 0
              7: 0
              8: 0
              9: 0
          - level: 7
            slots:
              1: 4
              2: 3
              3: 3
              4: 1
              5: 0
              6: 0
              7: 0
              8: 0
              9: 0
          - level: 8
            slots:
              1: 4
              2: 3
              3: 3
              4: 2
              5: 0
              6: 0
              7: 0
              8: 0
              9: 0
          - level: 9
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 1
              6: 0
              7: 0
              8: 0
              9: 0
          - level: 10
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 2
              6: 0
              7: 0
              8: 0
              9: 0
          - level: 11
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 2
              6: 1
              7: 0
              8: 0
              9: 0
          - level: 12
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 2
              6: 1
              7: 0
              8: 0
              9: 0
          - level: 13
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 2
              6: 1
              7: 1
              8: 0
              9: 0
          - level: 14
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 2
              6: 1
              7: 1
              8: 0
              9: 0
          - level: 15
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 2
              6: 1
              7: 1
              8: 1
              9: 0
          - level: 16
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 2
              6: 1
              7: 1
              8: 1
              9: 0
          - level: 17
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 2
              6: 1
              7: 1
              8: 1
              9: 1
          - level: 18
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 3
              6: 1
              7: 1
              8: 1
              9: 1
          - level: 19
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 3
              6: 2
              7: 1
              8: 1
              9: 1
          - level: 20
            slots:
              1: 4
              2: 3
              3: 3
              4: 3
              5: 3
              6: 2
              7: 2
              8: 1
              9: 1
          slots_allow_casting_any_prepared_spell_with_sufficient_level: true
          higher_slots_do_not_unlock_preparation: true
          upcast_effects: spell_definition
          pact_magic:
            pact_slots_cast_other_class_prepared_spells: true
            spellcasting_slots_cast_prepared_warlock_spells: true
    contract:
      id: dnd2024:rules_multiclassing_spellcasting
      inputs:
        subject: participant
      outputs:
      - spell_slots
      - prepared_spell_constraints
      - casting_ability
      - cantrip_level
      events: []
      capabilities:
      - multiclassing
      - spellcasting
    requires:
    - dnd2024:rules_multiclassing_spellcasting
    citations:
    - SRD 5.2.1 pp. 25, 26; builtin/srd-5.2.1/source.txt; section rules/multiclassing_spellcasting.md
  dnd2024:rules/barbarian_becoming_a_barbarian:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 28; builtin/srd-5.2.1/source.txt; section rules/barbarian_becoming_a_barbarian.md
  dnd2024:rules/barbarian_barbarian_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/barbarian_barbarian_class_features
        implementation: dnd2024:rules_barbarian_barbarian_class_features
        parameters:
          class:
            key: dnd2024:classes/barbarian
          grant_when_class_level_reached:
          - level: 11
            feature:
              key: dnd2024:class_features/barbarian_level_11_relentless_rage
          - level: 13
            feature:
              key: dnd2024:class_features/barbarian_level_13_improved_brutal_strike
          - level: 15
            feature:
              key: dnd2024:class_features/barbarian_level_15_persistent_rage
          - level: 17
            feature:
              key: dnd2024:class_features/barbarian_level_17_improved_brutal_strike
          - level: 18
            feature:
              key: dnd2024:class_features/barbarian_level_18_indomitable_might
          - level: 19
            feature:
              key: dnd2024:class_features/barbarian_level_19_epic_boon
          - level: 1
            feature:
              key: dnd2024:class_features/barbarian_level_1_rage
          - level: 1
            feature:
              key: dnd2024:class_features/barbarian_level_1_unarmored_defense
          - level: 1
            feature:
              key: dnd2024:class_features/barbarian_level_1_weapon_mastery
          - level: 20
            feature:
              key: dnd2024:class_features/barbarian_level_20_primal_champion
          - level: 2
            feature:
              key: dnd2024:class_features/barbarian_level_2_danger_sense
          - level: 2
            feature:
              key: dnd2024:class_features/barbarian_level_2_reckless_attack
          - level: 3
            feature:
              key: dnd2024:class_features/barbarian_level_3_barbarian_subclass
          - level: 3
            feature:
              key: dnd2024:class_features/barbarian_level_3_primal_knowledge
          - level: 4
            feature:
              key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
          - level: 5
            feature:
              key: dnd2024:class_features/barbarian_level_5_extra_attack
          - level: 5
            feature:
              key: dnd2024:class_features/barbarian_level_5_fast_movement
          - level: 7
            feature:
              key: dnd2024:class_features/barbarian_level_7_feral_instinct
          - level: 7
            feature:
              key: dnd2024:class_features/barbarian_level_7_instinctive_pounce
          - level: 9
            feature:
              key: dnd2024:class_features/barbarian_level_9_brutal_strike
    contract:
      id: dnd2024:rules_barbarian_barbarian_class_features
      inputs:
        subject: participant
      outputs:
      - class_features
      events: []
      capabilities:
      - character_features
      - level_advancement
    requires:
    - dnd2024:rules_barbarian_barbarian_class_features
    citations:
    - SRD 5.2.1 pp. 28; builtin/srd-5.2.1/source.txt; section rules/barbarian_barbarian_class_features.md
  dnd2024:rules/bard_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/bard_as_a_level_1_character
        implementation: dnd2024:rules_bard_as_a_level_1_character
        parameters:
          requires_total_character_level: 1
          core_traits:
            key: dnd2024:classes/bard
          grant_all_core_traits: true
          features:
          - level: 1
            feature:
              key: dnd2024:class_features/bard_level_1_bardic_inspiration
          - level: 1
            feature:
              key: dnd2024:class_features/bard_level_1_spellcasting
          progression:
          - level: 1
            proficiency_bonus: 2
            bardic_die_sides: 6
            features:
            - key: dnd2024:class_features/bard_level_1_bardic_inspiration
            - key: dnd2024:class_features/bard_level_1_spellcasting
          - level: 2
            proficiency_bonus: 2
            bardic_die_sides: 6
            features:
            - key: dnd2024:class_features/bard_level_2_expertise
            - key: dnd2024:class_features/bard_level_2_jack_of_all_trades
          - level: 3
            proficiency_bonus: 2
            bardic_die_sides: 6
            features:
            - key: dnd2024:class_features/bard_level_3_bard_subclass
          - level: 4
            proficiency_bonus: 2
            bardic_die_sides: 6
            features:
            - key: dnd2024:class_features/bard_level_4_ability_score_improvement
          - level: 5
            proficiency_bonus: 3
            bardic_die_sides: 8
            features:
            - key: dnd2024:class_features/bard_level_5_font_of_inspiration
          - level: 6
            proficiency_bonus: 3
            bardic_die_sides: 8
            features: []
          - level: 7
            proficiency_bonus: 3
            bardic_die_sides: 8
            features:
            - key: dnd2024:class_features/bard_level_7_countercharm
          - level: 8
            proficiency_bonus: 3
            bardic_die_sides: 8
            features: []
          - level: 9
            proficiency_bonus: 4
            bardic_die_sides: 8
            features: []
          - level: 10
            proficiency_bonus: 4
            bardic_die_sides: 10
            features:
            - key: dnd2024:class_features/bard_level_10_magical_secrets
          - level: 11
            proficiency_bonus: 4
            bardic_die_sides: 10
            features: []
          - level: 12
            proficiency_bonus: 4
            bardic_die_sides: 10
            features: []
          - level: 13
            proficiency_bonus: 5
            bardic_die_sides: 10
            features: []
          - level: 14
            proficiency_bonus: 5
            bardic_die_sides: 10
            features: []
          - level: 15
            proficiency_bonus: 5
            bardic_die_sides: 12
            features: []
          - level: 16
            proficiency_bonus: 5
            bardic_die_sides: 12
            features: []
          - level: 17
            proficiency_bonus: 6
            bardic_die_sides: 12
            features: []
          - level: 18
            proficiency_bonus: 6
            bardic_die_sides: 12
            features:
            - key: dnd2024:class_features/bard_level_18_superior_inspiration
          - level: 19
            proficiency_bonus: 6
            bardic_die_sides: 12
            features:
            - key: dnd2024:class_features/bard_level_19_epic_boon
          - level: 20
            proficiency_bonus: 6
            bardic_die_sides: 12
            features:
            - key: dnd2024:class_features/bard_level_20_words_of_creation
          subclass_feature_levels:
          - 3
          - 6
          - 14
    contract:
      id: dnd2024:rules_bard_as_a_level_1_character
      inputs:
        subject: participant
      outputs:
      - core_traits
      - class_features
      events: []
      capabilities:
      - character_creation
      - character_features
    requires:
    - dnd2024:rules_bard_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 31; builtin/srd-5.2.1/source.txt; section rules/bard_as_a_level_1_character.md
  dnd2024:rules/bard_bard_spell_list:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/bard_bard_spell_list
        implementation: dnd2024:rules_bard_bard_spell_list
        parameters:
          class:
            key: dnd2024:classes/bard
          spells:
          - spell:
              key: dnd2024:spells/dancing_lights
            level: 0
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/light
            level: 0
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/mage_hand
            level: 0
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/mending
            level: 0
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/message
            level: 0
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/minor_illusion
            level: 0
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/prestidigitation
            level: 0
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/starry_wisp
            level: 0
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/true_strike
            level: 0
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/vicious_mockery
            level: 0
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/animal_friendship
            level: 1
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/bane
            level: 1
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/charm_person
            level: 1
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/color_spray
            level: 1
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/command
            level: 1
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/comprehend_languages
            level: 1
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/cure_wounds
            level: 1
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/detect_magic
            level: 1
            school: divination
            concentration: true
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/disguise_self
            level: 1
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dissonant_whispers
            level: 1
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/faerie_fire
            level: 1
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/feather_fall
            level: 1
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/healing_word
            level: 1
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/heroism
            level: 1
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/hideous_laughter
            level: 1
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/identify
            level: 1
            school: divination
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/illusory_script
            level: 1
            school: illusion
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/longstrider
            level: 1
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/silent_image
            level: 1
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/sleep
            level: 1
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/speak_with_animals
            level: 1
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/thunderwave
            level: 1
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/unseen_servant
            level: 1
            school: conjuration
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/aid
            level: 2
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/animal_messenger
            level: 2
            school: enchantment
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/blindness_deafness
            level: 2
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/calm_emotions
            level: 2
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/detect_thoughts
            level: 2
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/enhance_ability
            level: 2
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/enlarge_reduce
            level: 2
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/enthrall
            level: 2
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/heat_metal
            level: 2
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/hold_person
            level: 2
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/invisibility
            level: 2
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/knock
            level: 2
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/lesser_restoration
            level: 2
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/locate_animals_or_plants
            level: 2
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/locate_object
            level: 2
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/magic_mouth
            level: 2
            school: illusion
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/mirror_image
            level: 2
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/see_invisibility
            level: 2
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/shatter
            level: 2
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/silence
            level: 2
            school: illusion
            concentration: true
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/suggestion
            level: 2
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/zone_of_truth
            level: 2
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/bestow_curse
            level: 3
            school: necromancy
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/clairvoyance
            level: 3
            school: divination
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/dispel_magic
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/fear
            level: 3
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/glyph_of_warding
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/hypnotic_pattern
            level: 3
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/major_image
            level: 3
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/mass_healing_word
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/nondetection
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/plant_growth
            level: 3
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/sending
            level: 3
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/slow
            level: 3
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/speak_with_dead
            level: 3
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/speak_with_plants
            level: 3
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/stinking_cloud
            level: 3
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/tiny_hut
            level: 3
            school: evocation
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/tongues
            level: 3
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/charm_monster
            level: 4
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/compulsion
            level: 4
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/confusion
            level: 4
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dimension_door
            level: 4
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/freedom_of_movement
            level: 4
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/greater_invisibility
            level: 4
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/hallucinatory_terrain
            level: 4
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/locate_creature
            level: 4
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/phantasmal_killer
            level: 4
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/polymorph
            level: 4
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/animate_objects
            level: 5
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/awaken
            level: 5
            school: transmutation
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/dominate_person
            level: 5
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dream
            level: 5
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/geas
            level: 5
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/greater_restoration
            level: 5
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/hold_monster
            level: 5
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/legend_lore
            level: 5
            school: divination
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/mass_cure_wounds
            level: 5
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/mislead
            level: 5
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/modify_memory
            level: 5
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/planar_binding
            level: 5
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/raise_dead
            level: 5
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/scrying
            level: 5
            school: divination
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/seeming
            level: 5
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/telepathic_bond
            level: 5
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/teleportation_circle
            level: 5
            school: conjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/eyebite
            level: 6
            school: necromancy
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/find_the_path
            level: 6
            school: divination
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/guards_and_wards
            level: 6
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/heroes_feast
            level: 6
            school: conjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/irresistible_dance
            level: 6
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/mass_suggestion
            level: 6
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/programmed_illusion
            level: 6
            school: illusion
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/true_seeing
            level: 6
            school: divination
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/arcane_sword
            level: 7
            school: evocation
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/etherealness
            level: 7
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/forcecage
            level: 7
            school: evocation
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/magnificent_mansion
            level: 7
            school: conjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/mirage_arcane
            level: 7
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/prismatic_spray
            level: 7
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/project_image
            level: 7
            school: illusion
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/regenerate
            level: 7
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/resurrection
            level: 7
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/symbol
            level: 7
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/teleport
            level: 7
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/antipathy_sympathy
            level: 8
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/befuddlement
            level: 8
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dominate_monster
            level: 8
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/glibness
            level: 8
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/mind_blank
            level: 8
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/power_word_stun
            level: 8
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/foresight
            level: 9
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/power_word_heal
            level: 9
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/power_word_kill
            level: 9
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/prismatic_wall
            level: 9
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/true_polymorph
            level: 9
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          sorting:
          - spell_level
          - name
    contract:
      id: dnd2024:rules_bard_bard_spell_list
      inputs:
        subject: participant
      outputs:
      - class_spell_list
      events: []
      capabilities:
      - spell_lists
    requires:
    - dnd2024:rules_bard_bard_spell_list
    citations:
    - SRD 5.2.1 pp. 33, 34, 35; builtin/srd-5.2.1/source.txt; section rules/bard_bard_spell_list.md
  dnd2024:rules/cleric_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/cleric_as_a_level_1_character
        implementation: dnd2024:rules_cleric_as_a_level_1_character
        parameters:
          requires_total_character_level: 1
          core_traits:
            key: dnd2024:classes/cleric
          grant_all_core_traits: true
          features:
          - level: 1
            feature:
              key: dnd2024:class_features/cleric_level_1_divine_order
          - level: 1
            feature:
              key: dnd2024:class_features/cleric_level_1_spellcasting
          progression:
          - level: 1
            proficiency_bonus: 2
            channel_divinity_uses: 0
            features:
            - key: dnd2024:class_features/cleric_level_1_divine_order
            - key: dnd2024:class_features/cleric_level_1_spellcasting
          - level: 2
            proficiency_bonus: 2
            channel_divinity_uses: 2
            features:
            - key: dnd2024:class_features/cleric_level_2_channel_divinity
          - level: 3
            proficiency_bonus: 2
            channel_divinity_uses: 2
            features:
            - key: dnd2024:class_features/cleric_level_3_cleric_subclass
          - level: 4
            proficiency_bonus: 2
            channel_divinity_uses: 2
            features:
            - key: dnd2024:class_features/cleric_level_4_ability_score_improvement
          - level: 5
            proficiency_bonus: 3
            channel_divinity_uses: 2
            features:
            - key: dnd2024:class_features/cleric_level_5_sear_undead
          - level: 6
            proficiency_bonus: 3
            channel_divinity_uses: 3
            features: []
          - level: 7
            proficiency_bonus: 3
            channel_divinity_uses: 3
            features:
            - key: dnd2024:class_features/cleric_level_7_blessed_strikes
          - level: 8
            proficiency_bonus: 3
            channel_divinity_uses: 3
            features: []
          - level: 9
            proficiency_bonus: 4
            channel_divinity_uses: 3
            features: []
          - level: 10
            proficiency_bonus: 4
            channel_divinity_uses: 3
            features:
            - key: dnd2024:class_features/cleric_level_10_divine_intervention
          - level: 11
            proficiency_bonus: 4
            channel_divinity_uses: 3
            features: []
          - level: 12
            proficiency_bonus: 4
            channel_divinity_uses: 3
            features: []
          - level: 13
            proficiency_bonus: 5
            channel_divinity_uses: 3
            features: []
          - level: 14
            proficiency_bonus: 5
            channel_divinity_uses: 3
            features:
            - key: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
          - level: 15
            proficiency_bonus: 5
            channel_divinity_uses: 3
            features: []
          - level: 16
            proficiency_bonus: 5
            channel_divinity_uses: 3
            features: []
          - level: 17
            proficiency_bonus: 6
            channel_divinity_uses: 3
            features: []
          - level: 18
            proficiency_bonus: 6
            channel_divinity_uses: 4
            features: []
          - level: 19
            proficiency_bonus: 6
            channel_divinity_uses: 4
            features:
            - key: dnd2024:class_features/cleric_level_19_epic_boon
          - level: 20
            proficiency_bonus: 6
            channel_divinity_uses: 4
            features:
            - key: dnd2024:class_features/cleric_level_20_greater_divine_intervention
          subclass_feature_levels:
          - 3
          - 6
          - 17
    contract:
      id: dnd2024:rules_cleric_as_a_level_1_character
      inputs:
        subject: participant
      outputs:
      - core_traits
      - class_features
      events: []
      capabilities:
      - character_creation
      - character_features
    requires:
    - dnd2024:rules_cleric_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 36; builtin/srd-5.2.1/source.txt; section rules/cleric_as_a_level_1_character.md
  dnd2024:rules/cleric_cleric_spell_list:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/cleric_cleric_spell_list
        implementation: dnd2024:rules_cleric_cleric_spell_list
        parameters:
          class:
            key: dnd2024:classes/cleric
          spells:
          - spell:
              key: dnd2024:spells/guidance
            level: 0
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/light
            level: 0
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/mending
            level: 0
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/resistance
            level: 0
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/sacred_flame
            level: 0
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/spare_the_dying
            level: 0
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/thaumaturgy
            level: 0
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/bane
            level: 1
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/bless
            level: 1
            school: enchantment
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/command
            level: 1
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/create_or_destroy_water
            level: 1
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/cure_wounds
            level: 1
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/detect_evil_and_good
            level: 1
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/detect_magic
            level: 1
            school: divination
            concentration: true
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/detect_poison_and_disease
            level: 1
            school: divination
            concentration: true
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/guiding_bolt
            level: 1
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/healing_word
            level: 1
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/inflict_wounds
            level: 1
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/protection_from_evil_and_good
            level: 1
            school: abjuration
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/purify_food_and_drink
            level: 1
            school: transmutation
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/sanctuary
            level: 1
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/shield_of_faith
            level: 1
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/aid
            level: 2
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/augury
            level: 2
            school: divination
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/blindness_deafness
            level: 2
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/calm_emotions
            level: 2
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/continual_flame
            level: 2
            school: evocation
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/enhance_ability
            level: 2
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/find_traps
            level: 2
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/gentle_repose
            level: 2
            school: necromancy
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/hold_person
            level: 2
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/lesser_restoration
            level: 2
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/locate_object
            level: 2
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/prayer_of_healing
            level: 2
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/protection_from_poison
            level: 2
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/silence
            level: 2
            school: illusion
            concentration: true
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/spiritual_weapon
            level: 2
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/warding_bond
            level: 2
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/zone_of_truth
            level: 2
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/animate_dead
            level: 3
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/beacon_of_hope
            level: 3
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/bestow_curse
            level: 3
            school: necromancy
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/clairvoyance
            level: 3
            school: divination
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/create_food_and_water
            level: 3
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/daylight
            level: 3
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dispel_magic
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/glyph_of_warding
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/magic_circle
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/mass_healing_word
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/meld_into_stone
            level: 3
            school: transmutation
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/protection_from_energy
            level: 3
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/remove_curse
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/revivify
            level: 3
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/sending
            level: 3
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/speak_with_dead
            level: 3
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/spirit_guardians
            level: 3
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/tongues
            level: 3
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/water_walk
            level: 3
            school: transmutation
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/aura_of_life
            level: 4
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/banishment
            level: 4
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/control_water
            level: 4
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/death_ward
            level: 4
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/divination
            level: 4
            school: divination
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/freedom_of_movement
            level: 4
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/guardian_of_faith
            level: 4
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/locate_creature
            level: 4
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/stone_shape
            level: 4
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/commune
            level: 5
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/contagion
            level: 5
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dispel_evil_and_good
            level: 5
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/flame_strike
            level: 5
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/geas
            level: 5
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/greater_restoration
            level: 5
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/hallow
            level: 5
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/insect_plague
            level: 5
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/legend_lore
            level: 5
            school: divination
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/mass_cure_wounds
            level: 5
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/planar_binding
            level: 5
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/raise_dead
            level: 5
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/scrying
            level: 5
            school: divination
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/blade_barrier
            level: 6
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/create_undead
            level: 6
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/find_the_path
            level: 6
            school: divination
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/forbiddance
            level: 6
            school: abjuration
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/harm
            level: 6
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/heal
            level: 6
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/heroes_feast
            level: 6
            school: conjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/planar_ally
            level: 6
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/sunbeam
            level: 6
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/true_seeing
            level: 6
            school: divination
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/word_of_recall
            level: 6
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/conjure_celestial
            level: 7
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/divine_word
            level: 7
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/etherealness
            level: 7
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/fire_storm
            level: 7
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/plane_shift
            level: 7
            school: conjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/regenerate
            level: 7
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/resurrection
            level: 7
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/symbol
            level: 7
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/antimagic_field
            level: 8
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/control_weather
            level: 8
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/earthquake
            level: 8
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/holy_aura
            level: 8
            school: abjuration
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/sunburst
            level: 8
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/astral_projection
            level: 9
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/gate
            level: 9
            school: conjuration
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/mass_heal
            level: 9
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/power_word_heal
            level: 9
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/true_resurrection
            level: 9
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          sorting:
          - spell_level
          - name
    contract:
      id: dnd2024:rules_cleric_cleric_spell_list
      inputs:
        subject: participant
      outputs:
      - class_spell_list
      events: []
      capabilities:
      - spell_lists
    requires:
    - dnd2024:rules_cleric_cleric_spell_list
    citations:
    - SRD 5.2.1 pp. 38, 39, 40; builtin/srd-5.2.1/source.txt; section rules/cleric_cleric_spell_list.md
  dnd2024:rules/druid_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/druid_as_a_multiclass_character
        implementation: dnd2024:rules_druid_as_a_multiclass_character
        parameters:
          class:
            key: dnd2024:classes/druid
          entry: multiclass
          hit_die_sides: 8
          armor_training:
          - light
          - shield
          features:
          - level: 1
            feature:
              key: dnd2024:class_features/druid_level_1_druidic
          - level: 1
            feature:
              key: dnd2024:class_features/druid_level_1_primal_order
          - level: 1
            feature:
              key: dnd2024:class_features/druid_level_1_spellcasting
          spell_slots_rule:
            key: dnd2024:rules/multiclassing_spellcasting
    contract:
      id: dnd2024:rules_druid_as_a_multiclass_character
      inputs:
        subject: participant
      outputs:
      - multiclass_grants
      events: []
      capabilities:
      - multiclassing
      - character_features
    requires:
    - dnd2024:rules_druid_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 41; builtin/srd-5.2.1/source.txt; section rules/druid_as_a_multiclass_character.md
  dnd2024:rules/druid_druid_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 46; builtin/srd-5.2.1/source.txt; section rules/druid_druid_subclass.md
  dnd2024:rules/fighter_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/fighter_as_a_multiclass_character
        implementation: dnd2024:rules_fighter_as_a_multiclass_character
        parameters:
          class:
            key: dnd2024:classes/fighter
          entry: multiclass
          hit_die_sides: 10
          armor_training:
          - light
          - medium
          - shield
          features:
          - level: 1
            feature:
              key: dnd2024:class_features/fighter_level_1_fighting_style
          - level: 1
            feature:
              key: dnd2024:class_features/fighter_level_1_second_wind
          - level: 1
            feature:
              key: dnd2024:class_features/fighter_level_1_weapon_mastery
          weapon_proficiencies:
          - martial
    contract:
      id: dnd2024:rules_fighter_as_a_multiclass_character
      inputs:
        subject: participant
      outputs:
      - multiclass_grants
      events: []
      capabilities:
      - multiclassing
      - character_features
    requires:
    - dnd2024:rules_fighter_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 47; builtin/srd-5.2.1/source.txt; section rules/fighter_as_a_multiclass_character.md
  dnd2024:rules/monk_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/monk_as_a_level_1_character
        implementation: dnd2024:rules_monk_as_a_level_1_character
        parameters:
          requires_total_character_level: 1
          core_traits:
            key: dnd2024:classes/monk
          grant_all_core_traits: true
          features:
          - level: 1
            feature:
              key: dnd2024:class_features/monk_level_1_martial_arts
          - level: 1
            feature:
              key: dnd2024:class_features/monk_level_1_unarmored_defense
    contract:
      id: dnd2024:rules_monk_as_a_level_1_character
      inputs:
        subject: participant
      outputs:
      - core_traits
      - class_features
      events: []
      capabilities:
      - character_creation
      - character_features
    requires:
    - dnd2024:rules_monk_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 49; builtin/srd-5.2.1/source.txt; section rules/monk_as_a_level_1_character.md
  dnd2024:rules/paladin_becoming_a_paladin:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 53; builtin/srd-5.2.1/source.txt; section rules/paladin_becoming_a_paladin.md
  dnd2024:rules/paladin_paladin_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/paladin_paladin_class_features
        implementation: dnd2024:rules_paladin_paladin_class_features
        parameters:
          class:
            key: dnd2024:classes/paladin
          grant_when_class_level_reached:
          - level: 10
            feature:
              key: dnd2024:class_features/paladin_level_10_aura_of_courage
          - level: 11
            feature:
              key: dnd2024:class_features/paladin_level_11_radiant_strikes
          - level: 14
            feature:
              key: dnd2024:class_features/paladin_level_14_restoring_touch
          - level: 18
            feature:
              key: dnd2024:class_features/paladin_level_18_aura_expansion
          - level: 19
            feature:
              key: dnd2024:class_features/paladin_level_19_epic_boon
          - level: 1
            feature:
              key: dnd2024:class_features/paladin_level_1_lay_on_hands
          - level: 1
            feature:
              key: dnd2024:class_features/paladin_level_1_spellcasting
          - level: 1
            feature:
              key: dnd2024:class_features/paladin_level_1_weapon_mastery
          - level: 2
            feature:
              key: dnd2024:class_features/paladin_level_2_fighting_style
          - level: 2
            feature:
              key: dnd2024:class_features/paladin_level_2_paladin_s_smite
          - level: 3
            feature:
              key: dnd2024:class_features/paladin_level_3_channel_divinity
          - level: 3
            feature:
              key: dnd2024:class_features/paladin_level_3_paladin_subclass
          - level: 4
            feature:
              key: dnd2024:class_features/paladin_level_4_ability_score_improvement
          - level: 5
            feature:
              key: dnd2024:class_features/paladin_level_5_extra_attack
          - level: 5
            feature:
              key: dnd2024:class_features/paladin_level_5_faithful_steed
          - level: 6
            feature:
              key: dnd2024:class_features/paladin_level_6_aura_of_protection
          - level: 9
            feature:
              key: dnd2024:class_features/paladin_level_9_abjure_foes
    contract:
      id: dnd2024:rules_paladin_paladin_class_features
      inputs:
        subject: participant
      outputs:
      - class_features
      events: []
      capabilities:
      - character_features
      - level_advancement
    requires:
    - dnd2024:rules_paladin_paladin_class_features
    citations:
    - SRD 5.2.1 pp. 53; builtin/srd-5.2.1/source.txt; section rules/paladin_paladin_class_features.md
  dnd2024:rules/ranger_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/ranger_as_a_level_1_character
        implementation: dnd2024:rules_ranger_as_a_level_1_character
        parameters:
          requires_total_character_level: 1
          core_traits:
            key: dnd2024:classes/ranger
          grant_all_core_traits: true
          features:
          - level: 1
            feature:
              key: dnd2024:class_features/ranger_level_1_favored_enemy
          - level: 1
            feature:
              key: dnd2024:class_features/ranger_level_1_spellcasting
          - level: 1
            feature:
              key: dnd2024:class_features/ranger_level_1_weapon_mastery
    contract:
      id: dnd2024:rules_ranger_as_a_level_1_character
      inputs:
        subject: participant
      outputs:
      - core_traits
      - class_features
      events: []
      capabilities:
      - character_creation
      - character_features
    requires:
    - dnd2024:rules_ranger_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 57; builtin/srd-5.2.1/source.txt; section rules/ranger_as_a_level_1_character.md
  dnd2024:rules/ranger_ranger_spell_list:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/ranger_ranger_spell_list
        implementation: dnd2024:rules_ranger_ranger_spell_list
        parameters:
          class:
            key: dnd2024:classes/ranger
          spells:
          - spell:
              key: dnd2024:spells/alarm
            level: 1
            school: abjuration
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/animal_friendship
            level: 1
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/cure_wounds
            level: 1
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/detect_magic
            level: 1
            school: divination
            concentration: true
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/detect_poison_and_disease
            level: 1
            school: divination
            concentration: true
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/ensnaring_strike
            level: 1
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/entangle
            level: 1
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/fog_cloud
            level: 1
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/goodberry
            level: 1
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/hunter_s_mark
            level: 1
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/jump
            level: 1
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/longstrider
            level: 1
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/speak_with_animals
            level: 1
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/aid
            level: 2
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/animal_messenger
            level: 2
            school: enchantment
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/barkskin
            level: 2
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/darkvision
            level: 2
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/enhance_ability
            level: 2
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/find_traps
            level: 2
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/gust_of_wind
            level: 2
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/lesser_restoration
            level: 2
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/locate_animals_or_plants
            level: 2
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/locate_object
            level: 2
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/magic_weapon
            level: 2
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/pass_without_trace
            level: 2
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/protection_from_poison
            level: 2
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/silence
            level: 2
            school: illusion
            concentration: true
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/spike_growth
            level: 2
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/conjure_animals
            level: 3
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/daylight
            level: 3
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dispel_magic
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/meld_into_stone
            level: 3
            school: transmutation
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/nondetection
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/plant_growth
            level: 3
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/protection_from_energy
            level: 3
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/revivify
            level: 3
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/speak_with_plants
            level: 3
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/water_breathing
            level: 3
            school: transmutation
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/water_walk
            level: 3
            school: transmutation
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/wind_wall
            level: 3
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/conjure_woodland_beings
            level: 4
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dominate_beast
            level: 4
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/freedom_of_movement
            level: 4
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/locate_creature
            level: 4
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/stoneskin
            level: 4
            school: transmutation
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/commune_with_nature
            level: 5
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/greater_restoration
            level: 5
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/tree_stride
            level: 5
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          sorting:
          - spell_level
          - name
    contract:
      id: dnd2024:rules_ranger_ranger_spell_list
      inputs:
        subject: participant
      outputs:
      - class_spell_list
      events: []
      capabilities:
      - spell_lists
    requires:
    - dnd2024:rules_ranger_ranger_spell_list
    citations:
    - SRD 5.2.1 pp. 60; builtin/srd-5.2.1/source.txt; section rules/ranger_ranger_spell_list.md
  dnd2024:rules/rogue_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/rogue_as_a_multiclass_character
        implementation: dnd2024:rules_rogue_as_a_multiclass_character
        parameters:
          class:
            key: dnd2024:classes/rogue
          entry: multiclass
          hit_die_sides: 8
          armor_training:
          - light
          features:
          - level: 1
            feature:
              key: dnd2024:class_features/rogue_level_1_expertise
          - level: 1
            feature:
              key: dnd2024:class_features/rogue_level_1_sneak_attack
          - level: 1
            feature:
              key: dnd2024:class_features/rogue_level_1_thieves_cant
          - level: 1
            feature:
              key: dnd2024:class_features/rogue_level_1_weapon_mastery
          tool_proficiencies:
          - thieves_tools
          skill_choice:
            count: 1
            options:
            - acrobatics
            - athletics
            - deception
            - insight
            - intimidation
            - investigation
            - perception
            - persuasion
            - sleight_of_hand
            - stealth
    contract:
      id: dnd2024:rules_rogue_as_a_multiclass_character
      inputs:
        subject: participant
      outputs:
      - multiclass_grants
      events: []
      capabilities:
      - multiclassing
      - character_features
    requires:
    - dnd2024:rules_rogue_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 61; builtin/srd-5.2.1/source.txt; section rules/rogue_as_a_multiclass_character.md
  dnd2024:rules/sorcerer_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_as_a_level_1_character
        implementation: dnd2024:rules_sorcerer_as_a_level_1_character
        parameters:
          requires_total_character_level: 1
          core_traits:
            key: dnd2024:classes/sorcerer
          grant_all_core_traits: true
          features:
          - level: 1
            feature:
              key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
          - level: 1
            feature:
              key: dnd2024:class_features/sorcerer_level_1_spellcasting
    contract:
      id: dnd2024:rules_sorcerer_as_a_level_1_character
      inputs:
        subject: participant
      outputs:
      - core_traits
      - class_features
      events: []
      capabilities:
      - character_creation
      - character_features
    requires:
    - dnd2024:rules_sorcerer_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 64; builtin/srd-5.2.1/source.txt; section rules/sorcerer_as_a_level_1_character.md
  dnd2024:rules/sorcerer_metamagic_options:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_metamagic_options
        implementation: dnd2024:rules_sorcerer_metamagic_options
        parameters:
          options:
          - key: dnd2024:rules/sorcerer_careful_spell
          - key: dnd2024:rules/sorcerer_distant_spell
          - key: dnd2024:rules/sorcerer_empowered_spell
          - key: dnd2024:rules/sorcerer_extended_spell
          - key: dnd2024:rules/sorcerer_heightened_spell
          - key: dnd2024:rules/sorcerer_quickened_spell
          - key: dnd2024:rules/sorcerer_seeking_spell
          - key: dnd2024:rules/sorcerer_subtle_spell
          - key: dnd2024:rules/sorcerer_transmuted_spell
          - key: dnd2024:rules/sorcerer_twinned_spell
    contract:
      id: dnd2024:rules_sorcerer_metamagic_options
      inputs:
        subject: participant
      outputs:
      - metamagic_options
      events: []
      capabilities:
      - character_options
    requires:
    - dnd2024:rules_sorcerer_metamagic_options
    citations:
    - SRD 5.2.1 pp. 66; builtin/srd-5.2.1/source.txt; section rules/sorcerer_metamagic_options.md
  dnd2024:rules/sorcerer_empowered_spell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_empowered_spell
        implementation: dnd2024:rules_sorcerer_empowered_spell
        parameters:
          trigger: spell_damage_roll
          cost:
            sorcery_points: 1
          reroll_dice:
            selection: caster_choice
            maximum:
              op: max
              args:
              - 1
              - var: subject.charisma_modifier
            use_new_results: true
          compatible_with_other_metamagic: true
    contract:
      id: dnd2024:rules_sorcerer_empowered_spell
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - spell.damage_rolled
      capabilities:
      - metamagic
      - dice
    requires:
    - dnd2024:rules_sorcerer_empowered_spell
    citations:
    - SRD 5.2.1 pp. 67; builtin/srd-5.2.1/source.txt; section rules/sorcerer_empowered_spell.md
  dnd2024:rules/sorcerer_quickened_spell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_quickened_spell
        implementation: dnd2024:rules_sorcerer_quickened_spell
        parameters:
          cost:
            sorcery_points: 2
          requires:
            spell_casting_time: action
            level_1_plus_spell_cast_this_turn: false
          casting_time_for_this_cast: bonus_action
          forbid_later_level_1_plus_spell_cast_this_turn: true
    contract:
      id: dnd2024:rules_sorcerer_quickened_spell
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - spell.cast_pending
      capabilities:
      - metamagic
      - action_economy
    requires:
    - dnd2024:rules_sorcerer_quickened_spell
    citations:
    - SRD 5.2.1 pp. 67; builtin/srd-5.2.1/source.txt; section rules/sorcerer_quickened_spell.md
  dnd2024:rules/sorcerer_transmuted_spell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_transmuted_spell
        implementation: dnd2024:rules_sorcerer_transmuted_spell
        parameters:
          cost:
            sorcery_points: 1
          damage_type_choice:
          - acid
          - cold
          - fire
          - lightning
          - poison
          - thunder
          requires_original_type_in_choice: true
          new_type_must_differ: true
          scope: current_spell_cast
    contract:
      id: dnd2024:rules_sorcerer_transmuted_spell
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - spell.cast_pending
      capabilities:
      - metamagic
      - damage
    requires:
    - dnd2024:rules_sorcerer_transmuted_spell
    citations:
    - SRD 5.2.1 pp. 67; builtin/srd-5.2.1/source.txt; section rules/sorcerer_transmuted_spell.md
  dnd2024:rules/warlock_becoming_a_warlock:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 70; builtin/srd-5.2.1/source.txt; section rules/warlock_becoming_a_warlock.md
  dnd2024:rules/warlock_warlock_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_warlock_class_features
        implementation: dnd2024:rules_warlock_warlock_class_features
        parameters:
          class:
            key: dnd2024:classes/warlock
          grant_when_class_level_reached:
          - level: 11
            feature:
              key: dnd2024:class_features/warlock_level_11_mystic_arcanum
          - level: 19
            feature:
              key: dnd2024:class_features/warlock_level_19_epic_boon
          - level: 1
            feature:
              key: dnd2024:class_features/warlock_level_1_eldritch_invocations
          - level: 1
            feature:
              key: dnd2024:class_features/warlock_level_1_pact_magic
          - level: 20
            feature:
              key: dnd2024:class_features/warlock_level_20_eldritch_master
          - level: 2
            feature:
              key: dnd2024:class_features/warlock_level_2_magical_cunning
          - level: 3
            feature:
              key: dnd2024:class_features/warlock_level_3_warlock_subclass
          - level: 4
            feature:
              key: dnd2024:class_features/warlock_level_4_ability_score_improvement
          - level: 9
            feature:
              key: dnd2024:class_features/warlock_level_9_contact_patron
    contract:
      id: dnd2024:rules_warlock_warlock_class_features
      inputs:
        subject: participant
      outputs:
      - class_features
      events: []
      capabilities:
      - character_features
      - level_advancement
    requires:
    - dnd2024:rules_warlock_warlock_class_features
    citations:
    - SRD 5.2.1 pp. 70; builtin/srd-5.2.1/source.txt; section rules/warlock_warlock_class_features.md
  dnd2024:rules/warlock_armor_of_shadows:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_armor_of_shadows
        implementation: dnd2024:rules_warlock_armor_of_shadows
        parameters:
          prerequisites:
            warlock_level_min: 0
          grant_cast:
            spell:
              key: dnd2024:spells/mage_armor
            spell_slot_cost: 0
            uses: unlimited
            target: self
    contract:
      id: dnd2024:rules_warlock_armor_of_shadows
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting
      - character_features
    requires:
    - dnd2024:rules_warlock_armor_of_shadows
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.txt; section rules/warlock_armor_of_shadows.md
  dnd2024:rules/warlock_devouring_blade:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_devouring_blade
        implementation: dnd2024:rules_warlock_devouring_blade
        parameters:
          prerequisites:
            warlock_level_min: 12
            invocation:
              key: dnd2024:rules/warlock_thirsting_blade
          thirsting_blade_extra_attacks: 2
    contract:
      id: dnd2024:rules_warlock_devouring_blade
      inputs:
        subject: participant
      outputs:
      - extra_attacks
      events: []
      capabilities:
      - character_features
      - attacks
    requires:
    - dnd2024:rules_warlock_devouring_blade
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.txt; section rules/warlock_devouring_blade.md
  dnd2024:rules/warlock_eldritch_spear:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_eldritch_spear
        implementation: dnd2024:rules_warlock_eldritch_spear
        parameters:
          prerequisites:
            warlock_level_min: 2
          selection:
            count: 1
            known: true
            class: warlock
            level: 0
            deals_damage: true
            range_min_ft: 10
          range_increase_ft:
            op: multiply
            args:
            - 30
            - var: subject.warlock_level
          repeatable: true
          repeat_selection: different_eligible_cantrip
    contract:
      id: dnd2024:rules_warlock_eldritch_spear
      inputs:
        subject: participant
      outputs:
      - spell_range_modifier
      events: []
      capabilities:
      - character_options
      - spellcasting
    requires:
    - dnd2024:rules_warlock_eldritch_spear
    citations:
    - SRD 5.2.1 pp. 73; builtin/srd-5.2.1/source.txt; section rules/warlock_eldritch_spear.md
  dnd2024:rules/warlock_gift_of_the_depths:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_gift_of_the_depths
        implementation: dnd2024:rules_warlock_gift_of_the_depths
        parameters:
          prerequisites:
            warlock_level_min: 5
          breathe_underwater: true
          swim_speed:
            var: subject.speed
          grant_cast:
            spell:
              key: dnd2024:spells/water_breathing
            spell_slot_cost: 0
            uses: 1
            reset: long_rest_finished
    contract:
      id: dnd2024:rules_warlock_gift_of_the_depths
      inputs:
        subject: participant
      outputs:
      - speeds
      - respiration
      - spell_grant
      events: []
      capabilities:
      - movement
      - spellcasting
    requires:
    - dnd2024:rules_warlock_gift_of_the_depths
    citations:
    - SRD 5.2.1 pp. 73; builtin/srd-5.2.1/source.txt; section rules/warlock_gift_of_the_depths.md
  dnd2024:rules/warlock_lessons_of_the_first_ones:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_lessons_of_the_first_ones
        implementation: dnd2024:rules_warlock_lessons_of_the_first_ones
        parameters:
          prerequisites:
            warlock_level_min: 2
          feat_choice:
            count: 1
            category: origin
          repeatable: true
          repeat_selection: different_origin_feat
    contract:
      id: dnd2024:rules_warlock_lessons_of_the_first_ones
      inputs:
        subject: participant
      outputs:
      - feat_grant
      events: []
      capabilities:
      - character_options
    requires:
    - dnd2024:rules_warlock_lessons_of_the_first_ones
    citations:
    - SRD 5.2.1 pp. 73; builtin/srd-5.2.1/source.txt; section rules/warlock_lessons_of_the_first_ones.md
  dnd2024:rules/warlock_master_of_myriad_forms:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_master_of_myriad_forms
        implementation: dnd2024:rules_warlock_master_of_myriad_forms
        parameters:
          prerequisites:
            warlock_level_min: 5
          grant_cast:
            spell:
              key: dnd2024:spells/alter_self
            spell_slot_cost: 0
            uses: unlimited
    contract:
      id: dnd2024:rules_warlock_master_of_myriad_forms
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting
      - character_features
    requires:
    - dnd2024:rules_warlock_master_of_myriad_forms
    citations:
    - SRD 5.2.1 pp. 73; builtin/srd-5.2.1/source.txt; section rules/warlock_master_of_myriad_forms.md
  dnd2024:rules/warlock_otherworldly_leap:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_otherworldly_leap
        implementation: dnd2024:rules_warlock_otherworldly_leap
        parameters:
          prerequisites:
            warlock_level_min: 2
          grant_cast:
            spell:
              key: dnd2024:spells/jump
            spell_slot_cost: 0
            uses: unlimited
            target: self
    contract:
      id: dnd2024:rules_warlock_otherworldly_leap
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting
      - character_features
    requires:
    - dnd2024:rules_warlock_otherworldly_leap
    citations:
    - SRD 5.2.1 pp. 74; builtin/srd-5.2.1/source.txt; section rules/warlock_otherworldly_leap.md
  dnd2024:rules/warlock_pact_of_the_tome:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_pact_of_the_tome
        implementation: dnd2024:rules_warlock_pact_of_the_tome
        parameters:
          conjure:
            timing:
            - short_rest_finished
            - long_rest_finished
            location: hand
            appearance: owner_choice
            exclusive_magic_access: owner
          book_disappears:
          - replacement_book_conjured
          - owner_dies
          on_conjure_choices:
          - count: 3
            spell_level: 0
            source: any_class_spell_list
            exclude_already_prepared: true
          - count: 2
            spell_level: 1
            ritual: true
            source: any_class_spell_list
            exclude_already_prepared: true
          while_book_on_person:
            chosen_spells_prepared: true
            chosen_spells_class: warlock
          spellcasting_focus: true
    contract:
      id: dnd2024:rules_warlock_pact_of_the_tome
      inputs:
        subject: participant
      outputs:
      - book
      - prepared_spells
      - spellcasting_focus
      events:
      - rest.short_finished
      - rest.long_finished
      - participant.died
      capabilities:
      - character_options
      - inventory
      - spellcasting
    requires:
    - dnd2024:rules_warlock_pact_of_the_tome
    citations:
    - SRD 5.2.1 pp. 74; builtin/srd-5.2.1/source.txt; section rules/warlock_pact_of_the_tome.md
  dnd2024:rules/warlock_visions_of_distant_realms:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_visions_of_distant_realms
        implementation: dnd2024:rules_warlock_visions_of_distant_realms
        parameters:
          prerequisites:
            warlock_level_min: 9
          grant_cast:
            spell:
              key: dnd2024:spells/arcane_eye
            spell_slot_cost: 0
            uses: unlimited
    contract:
      id: dnd2024:rules_warlock_visions_of_distant_realms
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting
      - character_features
    requires:
    - dnd2024:rules_warlock_visions_of_distant_realms
    citations:
    - SRD 5.2.1 pp. 74; builtin/srd-5.2.1/source.txt; section rules/warlock_visions_of_distant_realms.md
  dnd2024:rules/warlock_warlock_spell_list:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_warlock_spell_list
        implementation: dnd2024:rules_warlock_warlock_spell_list
        parameters:
          class:
            key: dnd2024:classes/warlock
          spells:
          - spell:
              key: dnd2024:spells/chill_touch
            level: 0
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/eldritch_blast
            level: 0
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/mage_hand
            level: 0
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/minor_illusion
            level: 0
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/poison_spray
            level: 0
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/prestidigitation
            level: 0
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/true_strike
            level: 0
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/bane
            level: 1
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/charm_person
            level: 1
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/comprehend_languages
            level: 1
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/detect_magic
            level: 1
            school: divination
            concentration: true
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/expeditious_retreat
            level: 1
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/hellish_rebuke
            level: 1
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/hex
            level: 1
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/hideous_laughter
            level: 1
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/illusory_script
            level: 1
            school: illusion
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/protection_from_evil_and_good
            level: 1
            school: abjuration
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/speak_with_animals
            level: 1
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/unseen_servant
            level: 1
            school: conjuration
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/darkness
            level: 2
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/enthrall
            level: 2
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/hold_person
            level: 2
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/invisibility
            level: 2
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/mind_spike
            level: 2
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/mirror_image
            level: 2
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/misty_step
            level: 2
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/ray_of_enfeeblement
            level: 2
            school: necromancy
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/spider_climb
            level: 2
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/suggestion
            level: 2
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/counterspell
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dispel_magic
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/fear
            level: 3
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/fly
            level: 3
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/gaseous_form
            level: 3
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/hypnotic_pattern
            level: 3
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/magic_circle
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/major_image
            level: 3
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/remove_curse
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/tongues
            level: 3
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/vampiric_touch
            level: 3
            school: necromancy
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/banishment
            level: 4
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/blight
            level: 4
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/charm_monster
            level: 4
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dimension_door
            level: 4
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/hallucinatory_terrain
            level: 4
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/contact_other_plane
            level: 5
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/dream
            level: 5
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/hold_monster
            level: 5
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/mislead
            level: 5
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/planar_binding
            level: 5
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/scrying
            level: 5
            school: divination
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/teleportation_circle
            level: 5
            school: conjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/circle_of_death
            level: 6
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/create_undead
            level: 6
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/eyebite
            level: 6
            school: necromancy
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/true_seeing
            level: 6
            school: divination
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/etherealness
            level: 7
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/finger_of_death
            level: 7
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/forcecage
            level: 7
            school: evocation
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/plane_shift
            level: 7
            school: conjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/befuddlement
            level: 8
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/demiplane
            level: 8
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dominate_monster
            level: 8
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/glibness
            level: 8
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/power_word_stun
            level: 8
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/astral_projection
            level: 9
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/foresight
            level: 9
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/gate
            level: 9
            school: conjuration
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/imprisonment
            level: 9
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/power_word_kill
            level: 9
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/true_polymorph
            level: 9
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/weird
            level: 9
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          sorting:
          - spell_level
          - name
    contract:
      id: dnd2024:rules_warlock_warlock_spell_list
      inputs:
        subject: participant
      outputs:
      - class_spell_list
      events: []
      capabilities:
      - spell_lists
    requires:
    - dnd2024:rules_warlock_warlock_spell_list
    citations:
    - SRD 5.2.1 pp. 74, 75, 76; builtin/srd-5.2.1/source.txt; section rules/warlock_warlock_spell_list.md
  dnd2024:rules/wizard_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/wizard_as_a_multiclass_character
        implementation: dnd2024:rules_wizard_as_a_multiclass_character
        parameters:
          class:
            key: dnd2024:classes/wizard
          entry: multiclass
          hit_die_sides: 6
          armor_training: []
          features:
          - level: 1
            feature:
              key: dnd2024:class_features/wizard_level_1_arcane_recovery
          - level: 1
            feature:
              key: dnd2024:class_features/wizard_level_1_ritual_adept
          - level: 1
            feature:
              key: dnd2024:class_features/wizard_level_1_spellcasting
          spell_slots_rule:
            key: dnd2024:rules/multiclassing_spellcasting
    contract:
      id: dnd2024:rules_wizard_as_a_multiclass_character
      inputs:
        subject: participant
      outputs:
      - multiclass_grants
      events: []
      capabilities:
      - multiclassing
      - character_features
    requires:
    - dnd2024:rules_wizard_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 77; builtin/srd-5.2.1/source.txt; section rules/wizard_as_a_multiclass_character.md
  dnd2024:rules/character_origins:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.txt; section rules/character_origins.md
  dnd2024:rules/character_backgrounds_ability_scores:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/character_backgrounds_ability_scores
        implementation: dnd2024:rules_character_backgrounds_ability_scores
        parameters:
          eligible_abilities: background_three_abilities
          allocation_choices:
          - - 2
            - 1
            - 0
          - - 1
            - 1
            - 1
          permutation_allowed: true
          ability_score_max_after_increase: 20
    contract:
      id: dnd2024:rules_character_backgrounds_ability_scores
      inputs:
        subject: participant
      outputs:
      - ability_score_increases
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rules_character_backgrounds_ability_scores
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.txt; section rules/character_backgrounds_ability_scores.md
  dnd2024:rules/character_backgrounds_tool_proficiency:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/character_backgrounds_tool_proficiency
        implementation: dnd2024:rules_character_backgrounds_tool_proficiency
        parameters:
          grant_count: 1
          source: background_definition
          selection:
          - fixed_tool
          - choose_artisans_tool
    contract:
      id: dnd2024:rules_character_backgrounds_tool_proficiency
      inputs:
        subject: participant
      outputs:
      - tool_proficiency
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rules_character_backgrounds_tool_proficiency
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.txt; section rules/character_backgrounds_tool_proficiency.md
  dnd2024:rules/character_species:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/character_species
        implementation: dnd2024:rules_character_species
        parameters:
          species_selection:
            count: 1
            grants: species_game_traits
          normal_lifespan_years_default: 80
          species_lifespan_overrides_default: true
          character_age:
            max: species_normal_lifespan
          physical_maturity_age_basis: shared_across_species
    contract:
      id: dnd2024:rules_character_species
      inputs:
        subject: participant
      outputs:
      - species_traits
      - age_constraints
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rules_character_species
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.txt; section rules/character_species.md
  dnd2024:rules/character_species_size:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/character_species_size
        implementation: dnd2024:rules_character_species_size
        parameters:
          size_source: species_definition
          if_species_allows_size_choice:
            choose_count: 1
            options:
            - small
            - medium
    contract:
      id: dnd2024:rules_character_species_size
      inputs:
        subject: participant
      outputs:
      - size
      events: []
      capabilities:
      - character_creation
    requires:
    - dnd2024:rules_character_species_size
    citations:
    - SRD 5.2.1 pp. 83, 84; builtin/srd-5.2.1/source.txt; section rules/character_species_size.md
  dnd2024:rules/character_species_species_descriptions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 84; builtin/srd-5.2.1/source.txt; section rules/character_species_species_descriptions.md
  dnd2024:rules/feat_descriptions_parts_of_a_feat:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/feat_descriptions_parts_of_a_feat
        implementation: dnd2024:rules_feat_descriptions_parts_of_a_feat
        parameters:
          category_constraint:
            if_specified: must_match
            otherwise: any_category
          prerequisites:
            must_satisfy_all: true
            feature_can_waive: true
            class_requirement_min_level: 1
          owned_feat_grants_benefits: true
          maximum_instances_default: 1
          repeatable_override: feat_repeatable_section
    contract:
      id: dnd2024:rules_feat_descriptions_parts_of_a_feat
      inputs:
        subject: participant
        feat: definition
        grant: resolution
      outputs:
      - feat_eligibility
      - feat_grants
      events: []
      capabilities:
      - character_options
    requires:
    - dnd2024:rules_feat_descriptions_parts_of_a_feat
    citations:
    - SRD 5.2.1 pp. 87; builtin/srd-5.2.1/source.txt; section rules/feat_descriptions_parts_of_a_feat.md
  dnd2024:rules/feat_descriptions_fighting_style_feats:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/feat_descriptions_fighting_style_feats
        implementation: dnd2024:rules_feat_descriptions_fighting_style_feats
        parameters:
          category: fighting_style
          members:
          - key: dnd2024:feats/fighting_style_feats_archery
          - key: dnd2024:feats/fighting_style_feats_defense
          - key: dnd2024:feats/fighting_style_feats_great_weapon_fighting
          - key: dnd2024:feats/fighting_style_feats_two_weapon_fighting
    contract:
      id: dnd2024:rules_feat_descriptions_fighting_style_feats
      inputs:
        subject: participant
      outputs:
      - feat_options
      events: []
      capabilities:
      - character_options
    requires:
    - dnd2024:rules_feat_descriptions_fighting_style_feats
    citations:
    - SRD 5.2.1 pp. 87; builtin/srd-5.2.1/source.txt; section rules/feat_descriptions_fighting_style_feats.md
  dnd2024:rules/coins:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/coins
        implementation: dnd2024:rules_coins
        parameters:
          gold_piece_value:
            copper: 0.01
            silver: 0.1
            electrum: 0.5
            gold: 1
            platinum: 10
          coins_per_pound: 50
          approximate_ounces_per_coin:
            numerator: 1
            denominator: 3
    contract:
      id: dnd2024:rules_coins
      inputs:
        coins: resolution
      outputs:
      - value_gp
      - weight_lb
      events: []
      capabilities:
      - currency
      - inventory
    requires:
    - dnd2024:rules_coins
    citations:
    - SRD 5.2.1 pp. 89; builtin/srd-5.2.1/source.txt; section rules/coins.md
  dnd2024:rules/weapons_properties:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 89; builtin/srd-5.2.1/source.txt; section rules/weapons_properties.md
  dnd2024:rules/weapons_heavy:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_heavy
        implementation: dnd2024:rules_weapons_heavy
        parameters:
          requires_property: heavy
          disadvantage_cases:
          - weapon_category: melee
            strength_below: 13
          - weapon_category: ranged
            dexterity_below: 13
    contract:
      id: dnd2024:rules_weapons_heavy
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - attack.calculate
      capabilities:
      - weapon_properties
    requires:
    - dnd2024:rules_weapons_heavy
    citations:
    - SRD 5.2.1 pp. 89; builtin/srd-5.2.1/source.txt; section rules/weapons_heavy.md
  dnd2024:rules/weapons_range:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_range
        implementation: dnd2024:rules_weapons_range
        parameters:
          range_unit: feet
          first_number: normal_range
          second_number: long_range
          range_follows_property:
          - ammunition
          - thrown
          beyond_normal: disadvantage
          beyond_long: prohibited
    contract:
      id: dnd2024:rules_weapons_range
      inputs:
        attack: resolution
      outputs:
      - eligibility
      - roll_modifiers
      events: []
      capabilities:
      - weapon_properties
    requires:
    - dnd2024:rules_weapons_range
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; section rules/weapons_range.md
  dnd2024:rules/weapons_two_handed:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_two_handed
        implementation: dnd2024:rules_weapons_two_handed
        parameters:
          requires_property: two_handed
          hands_required_when_attacking: 2
    contract:
      id: dnd2024:rules_weapons_two_handed
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - attack.requested
      capabilities:
      - weapon_properties
    requires:
    - dnd2024:rules_weapons_two_handed
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; section rules/weapons_two_handed.md
  dnd2024:rules/weapons_cleave:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_cleave
        implementation: dnd2024:rules_weapons_cleave
        parameters:
          trigger:
            melee_attack_hit: true
            target_kind: creature
            weapon: this_weapon
          optional: true
          extra_attack:
            kind: melee
            weapon: same_weapon
            target: different_creature
            target_distance_from_first_max_ft: 5
            target_within_attacker_reach: true
          extra_hit_damage:
            weapon_damage: true
            ability_modifier: only_if_negative
          uses:
            max: 1
            period: turn
    contract:
      id: dnd2024:rules_weapons_cleave
      inputs:
        subject: participant
        first_target: participant
        second_target: participant
        weapon: item_instance
      outputs:
      - effects
      events:
      - attack.hit
      capabilities:
      - weapon_mastery
      - attacks
    requires:
    - dnd2024:rules_weapons_cleave
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; section rules/weapons_cleave.md
  dnd2024:rules/weapons_push:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_push
        implementation: dnd2024:rules_weapons_push
        parameters:
          trigger:
            weapon_hit: true
            target_kind: creature
          optional: true
          target_size_max: large
          push:
            distance_ft_min: 0
            distance_ft_max: 10
            direction: straight_away_from_attacker
    contract:
      id: dnd2024:rules_weapons_push
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - attack.hit
      capabilities:
      - weapon_mastery
      - forced_movement
    requires:
    - dnd2024:rules_weapons_push
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; section rules/weapons_push.md
  dnd2024:rules/weapons_topple:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_topple
        implementation: dnd2024:rules_weapons_topple
        parameters:
          trigger: weapon_hit
          optional: true
          save:
            ability: constitution
            dc:
              op: add
              args:
              - 8
              - var: attack.ability_modifier
              - var: subject.proficiency_bonus
            failure:
              condition: prone
    contract:
      id: dnd2024:rules_weapons_topple
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - attack.hit
      capabilities:
      - weapon_mastery
      - saving_throws
      - conditions
    requires:
    - dnd2024:rules_weapons_topple
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; section rules/weapons_topple.md
  dnd2024:rules/armor_armor_training:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/armor_armor_training
        implementation: dnd2024:rules_armor_armor_training
        parameters:
          don_without_training: true
          training_source:
          - class
          - features
          monster_training: all_armor_in_stat_block
    contract:
      id: dnd2024:rules_armor_armor_training
      inputs:
        subject: participant
      outputs:
      - armor_training
      - equipment_eligibility
      events: []
      capabilities:
      - equipment
    requires:
    - dnd2024:rules_armor_armor_training
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt; section rules/armor_armor_training.md
  dnd2024:rules/armor_one_at_a_time:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/armor_one_at_a_time
        implementation: dnd2024:rules_armor_one_at_a_time
        parameters:
          worn_armor_suits_max: 1
          wielded_shields_max: 1
          armor_table:
          - armor: padded
            strength_min: null
            stealth_disadvantage: true
            weight_lb: 8
            cost_gp: 5
          - armor: leather
            strength_min: null
            stealth_disadvantage: false
            weight_lb: 10
            cost_gp: 10
          - armor: studded_leather
            strength_min: null
            stealth_disadvantage: false
            weight_lb: 13
            cost_gp: 45
          - armor: hide
            strength_min: null
            stealth_disadvantage: false
            weight_lb: 12
            cost_gp: 10
          - armor: chain_shirt
            strength_min: null
            stealth_disadvantage: false
            weight_lb: 20
            cost_gp: 50
          - armor: scale_mail
            strength_min: null
            stealth_disadvantage: true
            weight_lb: 45
            cost_gp: 50
          - armor: breastplate
            strength_min: null
            stealth_disadvantage: false
            weight_lb: 20
            cost_gp: 400
          - armor: half_plate
            strength_min: null
            stealth_disadvantage: true
            weight_lb: 40
            cost_gp: 750
          - armor: ring_mail
            strength_min: null
            stealth_disadvantage: true
            weight_lb: 40
            cost_gp: 30
          - armor: chain_mail
            strength_min: 13
            stealth_disadvantage: true
            weight_lb: 55
            cost_gp: 75
          - armor: splint
            strength_min: 15
            stealth_disadvantage: true
            weight_lb: 60
            cost_gp: 200
          - armor: plate
            strength_min: 15
            stealth_disadvantage: true
            weight_lb: 65
            cost_gp: 1500
          - armor: shield
            strength_min: null
            stealth_disadvantage: false
            weight_lb: 6
            cost_gp: 10
    contract:
      id: dnd2024:rules_armor_one_at_a_time
      inputs:
        subject: participant
        equipment: item_list
      outputs:
      - equipment_limits
      - armor_statistics
      events: []
      capabilities:
      - equipment
    requires:
    - dnd2024:rules_armor_one_at_a_time
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt; section rules/armor_one_at_a_time.md
  dnd2024:rules/tools_artisan_s_tools:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/tools_artisan_s_tools
        implementation: dnd2024:rules_tools_artisan_s_tools
        parameters:
          tool_category: artisans_tools
          separate_proficiency_per_tool: true
    contract:
      id: dnd2024:rules_tools_artisan_s_tools
      inputs:
        subject: participant
      outputs:
      - tool_proficiency_constraints
      events: []
      capabilities:
      - proficiency
    requires:
    - dnd2024:rules_tools_artisan_s_tools
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.txt; section rules/tools_artisan_s_tools.md
  dnd2024:rules/mounts_and_vehicles:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/mounts_and_vehicles
        implementation: dnd2024:rules_mounts_and_vehicles
        parameters:
          animal_statistics_source: monster_stat_block
          carrying_capacity_source: mounts_and_other_animals_table
          capacity_table_rule:
            key: dnd2024:rules/mounts_and_vehicles_saddles
    contract:
      id: dnd2024:rules_mounts_and_vehicles
      inputs:
        subject: participant
      outputs:
      - mount_statistics
      events: []
      capabilities:
      - mounts
    requires:
    - dnd2024:rules_mounts_and_vehicles
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.txt; section rules/mounts_and_vehicles.md
  dnd2024:rules/mounts_and_vehicles_saddles:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/mounts_and_vehicles_saddles
        implementation: dnd2024:rules_mounts_and_vehicles_saddles
        parameters:
          saddle_includes:
          - bit
          - bridle
          - reins
          - other_required_equipment
          military_saddle:
            remain_mounted_ability_checks: advantage
          exotic_saddle_required_for:
          - aquatic_mount
          - flying_mount
          mounts:
          - kind: camel
            capacity_lb: 450
            cost_gp: 50
          - kind: elephant
            capacity_lb: 1320
            cost_gp: 200
          - kind: draft_horse
            capacity_lb: 540
            cost_gp: 50
          - kind: riding_horse
            capacity_lb: 480
            cost_gp: 75
          - kind: mastiff
            capacity_lb: 195
            cost_gp: 25
          - kind: mule
            capacity_lb: 420
            cost_gp: 8
          - kind: pony
            capacity_lb: 225
            cost_gp: 30
          - kind: warhorse
            capacity_lb: 540
            cost_gp: 400
          tack_and_vehicles:
          - kind: carriage
            weight_lb: 600
            cost_gp: 100
          - kind: cart
            weight_lb: 200
            cost_gp: 15
          - kind: chariot
            weight_lb: 100
            cost_gp: 250
          - kind: feed_per_day
            weight_lb: 10
            cost_gp: 0.05
          - kind: exotic_saddle
            weight_lb: 40
            cost_gp: 60
          - kind: military_saddle
            weight_lb: 30
            cost_gp: 20
          - kind: riding_saddle
            weight_lb: 25
            cost_gp: 10
          - kind: sled
            weight_lb: 300
            cost_gp: 20
          - kind: stabling_per_day
            weight_lb: null
            cost_gp: 0.5
          - kind: wagon
            weight_lb: 400
            cost_gp: 35
    contract:
      id: dnd2024:rules_mounts_and_vehicles_saddles
      inputs:
        subject: participant
      outputs:
      - saddle_effects
      - mount_catalog
      - vehicle_catalog
      events: []
      capabilities:
      - mounts
      - equipment
    requires:
    - dnd2024:rules_mounts_and_vehicles_saddles
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.txt; section rules/mounts_and_vehicles_saddles.md
  dnd2024:rules/mounts_and_vehicles_crew:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/mounts_and_vehicles_crew
        implementation: dnd2024:rules_mounts_and_vehicles_crew
        parameters:
          minimum_crew:
            airship: 10
            galley: 80
            keelboat: 1
            longship: 40
            rowboat: 1
            sailing_ship: 20
            warship: 60
          skilled_hirelings_required_except:
          - keelboat
          - rowboat
    contract:
      id: dnd2024:rules_mounts_and_vehicles_crew
      inputs:
        vehicle: object
        crew: participant_list
      outputs:
      - operational
      events: []
      capabilities:
      - vehicles
    requires:
    - dnd2024:rules_mounts_and_vehicles_crew
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; section rules/mounts_and_vehicles_crew.md
  dnd2024:rules/mounts_and_vehicles_ship_repair:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/mounts_and_vehicles_ship_repair
        implementation: dnd2024:rules_mounts_and_vehicles_ship_repair
        parameters:
          requires_berthed: true
          per_hp:
            days: 1
            cost_gp: 20
          abundant_supplies_and_skilled_labor:
            time_multiplier: 0.5
            cost_multiplier: 0.5
    contract:
      id: dnd2024:rules_mounts_and_vehicles_ship_repair
      inputs:
        ship: object
        hp_to_repair: integer
        abundant_resources: boolean
      outputs:
      - repair_time
      - repair_cost
      - hit_points
      events: []
      capabilities:
      - crafting
      - vehicles
    requires:
    - dnd2024:rules_mounts_and_vehicles_ship_repair
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; section rules/mounts_and_vehicles_ship_repair.md
  dnd2024:rules/lifestyle_expenses_squalid_1_sp_per_day:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/lifestyle_expenses_squalid_1_sp_per_day
        implementation: dnd2024:rules_lifestyle_expenses_squalid_1_sp_per_day
        parameters:
          lifestyle: squalid
          cost_gp_per_day: 0.1
    contract:
      id: dnd2024:rules_lifestyle_expenses_squalid_1_sp_per_day
      inputs:
        subject: participant
      outputs:
      - lifestyle_cost
      events: []
      capabilities:
      - currency
      - downtime
    requires:
    - dnd2024:rules_lifestyle_expenses_squalid_1_sp_per_day
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; section rules/lifestyle_expenses_squalid_1_sp_per_day.md
  dnd2024:rules/lifestyle_expenses_comfortable_2_gp_per_day:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/lifestyle_expenses_comfortable_2_gp_per_day
        implementation: dnd2024:rules_lifestyle_expenses_comfortable_2_gp_per_day
        parameters:
          lifestyle: comfortable
          cost_gp_per_day: 2
    contract:
      id: dnd2024:rules_lifestyle_expenses_comfortable_2_gp_per_day
      inputs:
        subject: participant
      outputs:
      - lifestyle_cost
      events: []
      capabilities:
      - currency
      - downtime
    requires:
    - dnd2024:rules_lifestyle_expenses_comfortable_2_gp_per_day
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; section rules/lifestyle_expenses_comfortable_2_gp_per_day.md
  dnd2024:rules/food_drink_and_lodging:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/food_drink_and_lodging
        implementation: dnd2024:rules_food_drink_and_lodging
        parameters:
          daily_meals_and_lodging_included_in_lifestyle: true
          food_gp:
            ale_mug: 0.04
            bread_loaf: 0.02
            cheese_wedge: 0.1
          lodging_gp_per_day:
            squalid: 0.07
            poor: 0.1
            modest: 0.5
            comfortable: 0.8
            wealthy: 2
            aristocratic: 4
          meal_gp:
            squalid: 0.01
            poor: 0.02
            modest: 0.1
            comfortable: 0.2
            wealthy: 0.3
            aristocratic: 0.6
          wine_gp_per_bottle:
            common: 0.2
            fine: 10
    contract:
      id: dnd2024:rules_food_drink_and_lodging
      inputs:
        subject: participant
      outputs:
      - food_and_lodging_prices
      events: []
      capabilities:
      - currency
      - downtime
    requires:
    - dnd2024:rules_food_drink_and_lodging
    citations:
    - SRD 5.2.1 pp. 101, 102; builtin/srd-5.2.1/source.txt; section rules/food_drink_and_lodging.md
  dnd2024:rules/magic_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 102; builtin/srd-5.2.1/source.txt; section rules/magic_items.md
  dnd2024:rules/magic_items_attune_during_a_short_rest:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_items_attune_during_a_short_rest
        implementation: dnd2024:rules_magic_items_attune_during_a_short_rest
        parameters:
          activity: short_rest
          focus_items_max: 1
          requires_physical_contact: true
          cannot_simultaneously_identify_properties: true
          interrupted_rest: attunement_fails
          completed_rest:
            attuned: true
            full_magic_access: true
    contract:
      id: dnd2024:rules_magic_items_attune_during_a_short_rest
      inputs:
        subject: participant
        item: item_instance
        rest: resolution
      outputs:
      - attunement
      events:
      - rest.short_finished
      - rest.interrupted
      capabilities:
      - attunement
      - rests
    requires:
    - dnd2024:rules_magic_items_attune_during_a_short_rest
    citations:
    - SRD 5.2.1 pp. 102; builtin/srd-5.2.1/source.txt; section rules/magic_items_attune_during_a_short_rest.md
  dnd2024:rules/magic_items_wearing_and_wielding_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_items_wearing_and_wielding_items
        implementation: dnd2024:rules_magic_items_wearing_and_wielding_items
        parameters:
          required_equipment_positions:
            boots: feet
            gloves: hands
            hat: head
            helmet: head
            ring: finger
            armor: donned
            shield: strapped_to_arm
            cloak: shoulders
            weapon: held
          magic_garment_fit_default: any_size_or_build
          item_specific_overrides: true
    contract:
      id: dnd2024:rules_magic_items_wearing_and_wielding_items
      inputs:
        subject: participant
        item: item_instance
      outputs:
      - magic_property_access
      events: []
      capabilities:
      - equipment
      - magic_items
    requires:
    - dnd2024:rules_magic_items_wearing_and_wielding_items
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; section rules/magic_items_wearing_and_wielding_items.md
  dnd2024:rules/crafting_nonmagical_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/crafting_nonmagical_items
        implementation: dnd2024:rules_crafting_nonmagical_items
        parameters:
          requirements:
          - tools
          - raw_materials
          - time
          requirements_satisfied:
            create_item: true
            allow_use: true
            sale_price: normal_item_price
          requirement_rules:
          - key: dnd2024:rules/crafting_nonmagical_items_tools
          - key: dnd2024:rules/crafting_nonmagical_items_raw_materials
          - key: dnd2024:rules/crafting_nonmagical_items_time
    contract:
      id: dnd2024:rules_crafting_nonmagical_items
      inputs:
        subject: participant
        item: definition
        requirements: resolution
      outputs:
      - crafted_item
      events: []
      capabilities:
      - crafting
    requires:
    - dnd2024:rules_crafting_nonmagical_items
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; section rules/crafting_nonmagical_items.md
  dnd2024:rules/crafting_nonmagical_items_time:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/crafting_nonmagical_items_time
        implementation: dnd2024:rules_crafting_nonmagical_items_time
        parameters:
          work_hours_per_day: 8
          base_days:
            op: ceil
            args:
            - op: divide
              args:
              - var: item.purchase_cost_gp
              - 10
          days_must_be_consecutive: false
          assistance:
            divide_time_by: worker_count
            default_assistants_max: 1
            higher_assistant_limit_authority: gm
    contract:
      id: dnd2024:rules_crafting_nonmagical_items_time
      inputs:
        item: definition
        workers: participant_list
        assistant_limit: integer
      outputs:
      - crafting_days
      events: []
      capabilities:
      - crafting
    requires:
    - dnd2024:rules_crafting_nonmagical_items_time
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; section rules/crafting_nonmagical_items_time.md
  dnd2024:rules/scribing_spell_scrolls_time_and_cost:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/scribing_spell_scrolls_time_and_cost
        implementation: dnd2024:rules_scribing_spell_scrolls_time_and_cost
        parameters:
          work_hours_per_day: 8
          days_must_be_consecutive: false
          table:
          - spell_level: 0
            days: 1
            cost_gp: 15
          - spell_level: 1
            days: 1
            cost_gp: 25
          - spell_level: 2
            days: 3
            cost_gp: 100
          - spell_level: 3
            days: 5
            cost_gp: 150
          - spell_level: 4
            days: 10
            cost_gp: 1000
          - spell_level: 5
            days: 25
            cost_gp: 1500
          - spell_level: 6
            days: 40
            cost_gp: 10000
          - spell_level: 7
            days: 50
            cost_gp: 12500
          - spell_level: 8
            days: 60
            cost_gp: 15000
          - spell_level: 9
            days: 120
            cost_gp: 50000
    contract:
      id: dnd2024:rules_scribing_spell_scrolls_time_and_cost
      inputs:
        spell: definition
      outputs:
      - scribe_time
      - scribe_cost
      events: []
      capabilities:
      - crafting
      - spell_scrolls
    requires:
    - dnd2024:rules_scribing_spell_scrolls_time_and_cost
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; section rules/scribing_spell_scrolls_time_and_cost.md
  dnd2024:rules/spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 104; builtin/srd-5.2.1/source.txt; section rules/spells.md
  dnd2024:rules/gaining_spells_always_prepared_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/gaining_spells_always_prepared_spells
        implementation: dnd2024:rules_gaining_spells_always_prepared_spells
        parameters:
          always_prepared_spells_count_against_changeable_preparation_limit: false
    contract:
      id: dnd2024:rules_gaining_spells_always_prepared_spells
      inputs:
        subject: participant
      outputs:
      - prepared_spell_count
      events: []
      capabilities:
      - spell_preparation
    requires:
    - dnd2024:rules_gaining_spells_always_prepared_spells
    citations:
    - SRD 5.2.1 pp. 104; builtin/srd-5.2.1/source.txt; section rules/gaining_spells_always_prepared_spells.md
  dnd2024:rules/casting_spells_spell_slots:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_spell_slots
        implementation: dnd2024:rules_casting_spells_spell_slots
        parameters:
          spell_slot_required_default_for_spell_levels:
            min: 1
            max: 9
          slot_level_min: spell_level
          slots_expended_per_cast: 1
          slot_capacity_source: class_spellcasting_table
          restore_all_expended_slots: long_rest_finished
          specific_feature_exceptions: true
    contract:
      id: dnd2024:rules_casting_spells_spell_slots
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - spell.cast_pending
      - rest.long_finished
      capabilities:
      - spell_slots
    requires:
    - dnd2024:rules_casting_spells_spell_slots
    citations:
    - SRD 5.2.1 pp. 104; builtin/srd-5.2.1/source.txt; section rules/casting_spells_spell_slots.md
  dnd2024:rules/casting_spells_school_of_magic:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_school_of_magic
        implementation: dnd2024:rules_casting_spells_school_of_magic
        parameters:
          classification_only: true
          schools:
          - abjuration
          - conjuration
          - divination
          - enchantment
          - evocation
          - illusion
          - necromancy
          - transmutation
          standalone_school_effects: false
          other_rules_can_match_school: true
    contract:
      id: dnd2024:rules_casting_spells_school_of_magic
      inputs:
        spell: definition
      outputs:
      - school
      events: []
      capabilities:
      - spell_metadata
    requires:
    - dnd2024:rules_casting_spells_school_of_magic
    citations:
    - SRD 5.2.1 pp. 105; builtin/srd-5.2.1/source.txt; section rules/casting_spells_school_of_magic.md
  dnd2024:rules/casting_spells_one_spell_with_a_spell_slot_per_turn:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_one_spell_with_a_spell_slot_per_turn
        implementation: dnd2024:rules_casting_spells_one_spell_with_a_spell_slot_per_turn
        parameters:
          maximum_spell_slots_expended_to_cast_per_turn: 1
          scope: any_action_economy
    contract:
      id: dnd2024:rules_casting_spells_one_spell_with_a_spell_slot_per_turn
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - spell.cast_pending
      capabilities:
      - spell_slots
      - action_economy
    requires:
    - dnd2024:rules_casting_spells_one_spell_with_a_spell_slot_per_turn
    citations:
    - SRD 5.2.1 pp. 105; builtin/srd-5.2.1/source.txt; section rules/casting_spells_one_spell_with_a_spell_slot_per_turn.md
  dnd2024:rules/casting_spells_range:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_range
        implementation: dnd2024:rules_casting_spells_range
        parameters:
          range_modes:
            distance:
              unit: feet
              limits: spell_specified_effect_origin
            touch:
              requires_touch: true
              within_caster_reach: true
            self:
              origin: caster
              mode_from_spell:
              - on_caster
              - emanates_from_caster
          movable_effects_range_limited_after_cast_default: false
          specific_spell_overrides: true
    contract:
      id: dnd2024:rules_casting_spells_range
      inputs:
        caster: participant
        spell: definition
        target: resolution
      outputs:
      - range_eligibility
      events: []
      capabilities:
      - spellcasting
      - geometry
    requires:
    - dnd2024:rules_casting_spells_range
    citations:
    - SRD 5.2.1 pp. 105; builtin/srd-5.2.1/source.txt; section rules/casting_spells_range.md
  dnd2024:rules/casting_spells_somatic_s:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_somatic_s
        implementation: dnd2024:rules_casting_spells_somatic_s
        parameters:
          component: somatic
          hands_required_min: 1
          requires_perform_gestures: true
    contract:
      id: dnd2024:rules_casting_spells_somatic_s
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - spell.cast_pending
      capabilities:
      - spell_components
    requires:
    - dnd2024:rules_casting_spells_somatic_s
    citations:
    - SRD 5.2.1 pp. 105; builtin/srd-5.2.1/source.txt; section rules/casting_spells_somatic_s.md
  dnd2024:rules/casting_spells_effects:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_effects
        implementation: dnd2024:rules_casting_spells_effects
        parameters:
          effect_definition_authoritative: true
          mundane_physical_laws_constrain_effects: false
          unlisted_outcomes:
            authority: gm
            input_type: resolution
    contract:
      id: dnd2024:rules_casting_spells_effects
      inputs:
        spell: definition
        unlisted_outcome:
          type: resolution
          required: false
      outputs:
      - spell_effects
      events: []
      capabilities:
      - spellcasting
    requires:
    - dnd2024:rules_casting_spells_effects
    citations:
    - SRD 5.2.1 pp. 106; builtin/srd-5.2.1/source.txt; section rules/casting_spells_effects.md
  dnd2024:rules/casting_spells_attack_rolls:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_attack_rolls
        implementation: dnd2024:rules_casting_spells_attack_rolls
        parameters:
          spell_attack_modifier:
            op: add
            args:
            - var: caster.spellcasting_ability_modifier
            - var: caster.proficiency_bonus
    contract:
      id: dnd2024:rules_casting_spells_attack_rolls
      inputs:
        caster: participant
        spell: definition
      outputs:
      - attack_modifier
      events: []
      capabilities:
      - spell_attacks
    requires:
    - dnd2024:rules_casting_spells_attack_rolls
    citations:
    - SRD 5.2.1 pp. 106; builtin/srd-5.2.1/source.txt; section rules/casting_spells_attack_rolls.md
  dnd2024:rules/travel_pace:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/travel_pace
        implementation: dnd2024:rules_travel_pace
        parameters:
          paces:
          - normal
          - fast
          - slow
          maximum_determined_by: predominant_terrain_per_journey_stage
          terrain_table:
          - terrain: arctic
            maximum_pace: fast
            encounter_distance_ft:
              dice:
                count: 6
                sides: 6
              multiplier: 10
            foraging_dc: 20
            navigation_dc: 10
            search_dc: 10
          - terrain: coastal
            maximum_pace: normal
            encounter_distance_ft:
              dice:
                count: 2
                sides: 10
              multiplier: 10
            foraging_dc: 10
            navigation_dc: 5
            search_dc: 15
          - terrain: desert
            maximum_pace: normal
            encounter_distance_ft:
              dice:
                count: 6
                sides: 6
              multiplier: 10
            foraging_dc: 20
            navigation_dc: 10
            search_dc: 10
          - terrain: forest
            maximum_pace: normal
            encounter_distance_ft:
              dice:
                count: 2
                sides: 8
              multiplier: 10
            foraging_dc: 10
            navigation_dc: 15
            search_dc: 15
          - terrain: grassland
            maximum_pace: fast
            encounter_distance_ft:
              dice:
                count: 6
                sides: 6
              multiplier: 10
            foraging_dc: 15
            navigation_dc: 5
            search_dc: 15
          - terrain: hill
            maximum_pace: normal
            encounter_distance_ft:
              dice:
                count: 2
                sides: 10
              multiplier: 10
            foraging_dc: 15
            navigation_dc: 10
            search_dc: 15
          - terrain: mountain
            maximum_pace: slow
            encounter_distance_ft:
              dice:
                count: 4
                sides: 10
              multiplier: 10
            foraging_dc: 20
            navigation_dc: 15
            search_dc: 20
          - terrain: swamp
            maximum_pace: slow
            encounter_distance_ft:
              dice:
                count: 2
                sides: 8
              multiplier: 10
            foraging_dc: 10
            navigation_dc: 15
            search_dc: 20
          - terrain: underdark
            maximum_pace: normal
            encounter_distance_ft:
              dice:
                count: 2
                sides: 6
              multiplier: 10
            foraging_dc: 20
            navigation_dc: 10
            search_dc: 20
          - terrain: urban
            maximum_pace: normal
            encounter_distance_ft:
              dice:
                count: 2
                sides: 6
              multiplier: 10
            foraging_dc: 20
            navigation_dc: 15
            search_dc: 15
          - terrain: waterborne
            maximum_pace: vehicle_speed
            encounter_distance_ft:
              dice:
                count: 6
                sides: 6
              multiplier: 10
            foraging_dc: 15
            navigation_dc: 10
            search_dc: 15
          arctic_fast_pace_requires_appropriate_equipment: true
          waterborne_uses_vehicle_speed: true
    contract:
      id: dnd2024:rules_travel_pace
      inputs:
        party: participant_list
        terrain: choice
        pace: choice
      outputs:
      - pace_limit
      - terrain_checks
      - encounter_distance
      events: []
      capabilities:
      - travel
      - exploration
    requires:
    - dnd2024:rules_travel_pace
    citations:
    - SRD 5.2.1 pp. 192; builtin/srd-5.2.1/source.txt; section rules/travel_pace.md
  dnd2024:rules/travel_pace_extended_travel:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/travel_pace_extended_travel
        implementation: dnd2024:rules_travel_pace_extended_travel
        parameters:
          safe_daily_travel_hours: 8
          trigger: each_additional_hour_end
          save:
            ability: constitution
            dc:
              op: add
              args:
              - 10
              - op: subtract
                args:
                - var: hours_traveled_today
                - 8
            failure:
              exhaustion_gain: 1
    contract:
      id: dnd2024:rules_travel_pace_extended_travel
      inputs:
        subject: participant
        hours_traveled_today: integer
      outputs:
      - effects
      events:
      - travel.hour_finished
      capabilities:
      - travel
      - exhaustion
      - saving_throws
    requires:
    - dnd2024:rules_travel_pace_extended_travel
    citations:
    - SRD 5.2.1 pp. 192; builtin/srd-5.2.1/source.txt; section rules/travel_pace_extended_travel.md
  dnd2024:rules/creating_a_background:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 192; builtin/srd-5.2.1/source.txt; section rules/creating_a_background.md
  dnd2024:rules/creating_a_background_3_choose_skill_proficiencies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/creating_a_background_3_choose_skill_proficiencies
        implementation: dnd2024:rules_creating_a_background_3_choose_skill_proficiencies
        parameters:
          skill_proficiencies:
            count: 2
            selection: background_designer_choice
            appropriate_to_background: true
          require_related_background_ability: false
    contract:
      id: dnd2024:rules_creating_a_background_3_choose_skill_proficiencies
      inputs:
        background: definition
        skills: choice
      outputs:
      - skill_proficiencies
      events: []
      capabilities:
      - character_options
    requires:
    - dnd2024:rules_creating_a_background_3_choose_skill_proficiencies
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; section rules/creating_a_background_3_choose_skill_proficiencies.md
  dnd2024:rules/curses_and:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; section rules/curses_and.md
  dnd2024:rules/magical_contagions_bestow_curse:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magical_contagions_bestow_curse
        implementation: dnd2024:rules_magical_contagions_bestow_curse
        parameters:
          curse:
            key: dnd2024:spells/bestow_curse
          ended_by:
            key: dnd2024:spells/remove_curse
          potency_benchmarks:
          - duration:
              value: 1
              unit: minute
            spell_level: 3
          - duration: until_dispelled
            spell_level: 9
    contract:
      id: dnd2024:rules_magical_contagions_bestow_curse
      inputs:
        subject: participant
      outputs:
      - curse_effects
      - potency_benchmarks
      events: []
      capabilities:
      - curses
    requires:
    - dnd2024:rules_magical_contagions_bestow_curse
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; section rules/magical_contagions_bestow_curse.md
  dnd2024:rules/magical_contagions_narrative_curses:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magical_contagions_narrative_curses
        implementation: dnd2024:rules_magical_contagions_narrative_curses
        parameters:
          trigger:
            authority: gm
            kind: taboo_violation
            examples:
            - broken_vow
            - defiled_tomb
            - murdered_innocent
          curse_design:
            authority: gm
            effect_input_type: effect
            may_customize_other_curse: true
          victim_information:
            punishment_reason_known: true
            ending_method_discoverable: true
          remove_curse_response:
            authority: gm
            mode: choice
            allowed_modes:
            - end
            - suppress
            - custom_effect
            suppression_duration_input: duration
    contract:
      id: dnd2024:rules_magical_contagions_narrative_curses
      inputs:
        victim: participant
        violation: resolution
        curse_effect: effect
        ending_method: resolution
        remove_curse_response: choice
        suppression_duration:
          type: duration
          required: false
      outputs:
      - curse
      - curse_knowledge
      events: []
      capabilities:
      - curses
      - narrative_choices
    requires:
    - dnd2024:rules_magical_contagions_narrative_curses
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; section rules/magical_contagions_narrative_curses.md
  dnd2024:rules/magical_contagions_rest_and_recuperation:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magical_contagions_rest_and_recuperation
        implementation: dnd2024:rules_magical_contagions_rest_and_recuperation
        parameters:
          eligibility: infected_with_magical_contagion
          recuperation:
            days: 3
            forbid_long_rest_interrupting_activities: true
          end_save:
            ability: constitution
            dc: 15
          success:
            advantage_scope: saves_against_same_magical_contagion
            duration:
              value: 24
              unit: hour
    contract:
      id: dnd2024:rules_magical_contagions_rest_and_recuperation
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - recuperation.finished
      capabilities:
      - contagions
      - saving_throws
      - downtime
    requires:
    - dnd2024:rules_magical_contagions_rest_and_recuperation
    citations:
    - SRD 5.2.1 pp. 194; builtin/srd-5.2.1/source.txt; section rules/magical_contagions_rest_and_recuperation.md
  dnd2024:rules/magical_contagions_sewer_plague:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magical_contagions_sewer_plague
        implementation: dnd2024:rules_magical_contagions_sewer_plague
        parameters:
          kind: magical_contagion
          infection:
            target_type: humanoid
            exposure:
            - wounded_by_infected_carrier
            - contact_contaminated_filth_or_offal
            save:
              ability: constitution
              dc: 11
            on_failure: infected
            onset:
              dice:
                count: 1
                sides: 4
              unit: day
          onset_effect:
            exhaustion_gain: 1
          while_exhaustion_positive:
            hit_die_healing_multiplier: 0.5
            long_rest_restores_hp: false
            long_rest_reduces_exhaustion: false
          recovery:
            timing: daily_dawn
            save:
              ability: constitution
              dc: 11
            failure:
              exhaustion_gain: 1
            success:
              exhaustion_reduce: 1
              end_contagion_if_exhaustion_zero: true
    contract:
      id: dnd2024:rules_magical_contagions_sewer_plague
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - contagion.exposure
      - contagion.onset
      - dawn
      - rest.long_finished
      - hit_die.spent
      capabilities:
      - contagions
      - exhaustion
      - healing
      - saving_throws
    requires:
    - dnd2024:rules_magical_contagions_sewer_plague
    citations:
    - SRD 5.2.1 pp. 194; builtin/srd-5.2.1/source.txt; section rules/magical_contagions_sewer_plague.md
  dnd2024:rules/environmental_effects_deep_water:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/environmental_effects_deep_water
        implementation: dnd2024:rules_environmental_effects_deep_water
        parameters:
          water_depth_ft_greater_than: 100
          subject_lacks_swim_speed: true
          trigger: each_swimming_hour_end
          save:
            ability: constitution
            dc: 10
          failure:
            exhaustion_gain: 1
    contract:
      id: dnd2024:rules_environmental_effects_deep_water
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - swimming.hour_finished
      capabilities:
      - environment
      - saving_throws
      - exhaustion
    requires:
    - dnd2024:rules_environmental_effects_deep_water
    citations:
    - SRD 5.2.1 pp. 195; builtin/srd-5.2.1/source.txt; section rules/environmental_effects_deep_water.md
  dnd2024:rules/environmental_effects_frigid_water:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/environmental_effects_frigid_water
        implementation: dnd2024:rules_environmental_effects_frigid_water
        parameters:
          environment: frigid_water
          safe_immersion_minutes:
            var: subject.constitution_score
          trigger: each_additional_immersion_minute
          save:
            ability: constitution
            dc: 10
          automatic_success_any:
          - cold_resistance
          - cold_immunity
          - naturally_adapted_to_ice_cold_water
          failure:
            exhaustion_gain: 1
    contract:
      id: dnd2024:rules_environmental_effects_frigid_water
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - immersion.minute_finished
      capabilities:
      - environment
      - saving_throws
      - exhaustion
    requires:
    - dnd2024:rules_environmental_effects_frigid_water
    citations:
    - SRD 5.2.1 pp. 195; builtin/srd-5.2.1/source.txt; section rules/environmental_effects_frigid_water.md
  dnd2024:rules/environmental_effects_slippery_ice:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/environmental_effects_slippery_ice
        implementation: dnd2024:rules_environmental_effects_slippery_ice
        parameters:
          terrain: difficult
          triggers:
          - first_entry_on_turn
          - start_turn_inside
          save:
            ability: dexterity
            dc: 10
          failure:
            condition: prone
    contract:
      id: dnd2024:rules_environmental_effects_slippery_ice
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - movement.entered
      - turn.started
      capabilities:
      - environment
      - saving_throws
      - conditions
    requires:
    - dnd2024:rules_environmental_effects_slippery_ice
    citations:
    - SRD 5.2.1 pp. 195; builtin/srd-5.2.1/source.txt; section rules/environmental_effects_slippery_ice.md
  dnd2024:rules/fear_and_mental_stress:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 196; builtin/srd-5.2.1/source.txt; section rules/fear_and_mental_stress.md
  dnd2024:rules/fear_and_mental_stress_prolonged_effects:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/fear_and_mental_stress_prolonged_effects
        implementation: dnd2024:rules_fear_and_mental_stress_prolonged_effects
        parameters:
          application:
            authority: gm
            cause: mental_stress
            choice:
            - short_term
            - long_term
            - indefinite
          short_term:
            condition_choice:
            - frightened
            - incapacitated
            - stunned
            duration:
              dice:
                count: 1
                sides: 10
              unit: minute
            optional_narrative:
            - alarming_behavior
            - hallucinations
            suppressed_by:
              key: dnd2024:spells/calm_emotions
            removed_by:
              key: dnd2024:spells/lesser_restoration
          long_term:
            disadvantage: selected_ability_checks
            ability_scope:
              authority: gm
              choose_min: 1
              choose_max: 6
              options:
              - strength
              - dexterity
              - constitution
              - intelligence
              - wisdom
              - charisma
            duration:
              dice:
                count: 1
                sides: 10
              multiplier: 10
              unit: hour
            suppressed_by:
              key: dnd2024:spells/calm_emotions
            removed_by:
              key: dnd2024:spells/lesser_restoration
          indefinite:
            effect_template: long_term
            duration: until_removed
            suppressed_by:
              key: dnd2024:spells/calm_emotions
            removed_by:
              key: dnd2024:spells/greater_restoration
            lesser_restoration_removes: false
    contract:
      id: dnd2024:rules_fear_and_mental_stress_prolonged_effects
      inputs:
        subject: participant
        effect_mode: choice
        condition_choice:
          type: choice
          required: false
        affected_abilities:
          type: choice
          required: false
      outputs:
      - stress_effect
      events: []
      capabilities:
      - mental_stress
      - conditions
      - checks
    requires:
    - dnd2024:rules_fear_and_mental_stress_prolonged_effects
    citations:
    - SRD 5.2.1 pp. 196; builtin/srd-5.2.1/source.txt; section rules/fear_and_mental_stress_prolonged_effects.md
  dnd2024:rules/poison_harvesting_poison:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_harvesting_poison
        implementation: dnd2024:rules_poison_harvesting_poison
        parameters:
          target_requirements:
            venomous: true
            any_state:
            - dead
            - incapacitated
            already_harvested: false
          time:
            dice:
              count: 1
              sides: 6
            unit: minute
          required_tool: poisoners_kit
          check:
            ability: intelligence
            skill: nature
            dc: 20
          success:
            poison_doses: 1
            target_further_harvest_prohibited: true
          failure:
            poison_doses: 0
          failure_margin_at_least_5:
            subjected_to: target_poison
    contract:
      id: dnd2024:rules_poison_harvesting_poison
      inputs:
        subject: participant
        target: participant
        kit: item_instance
      outputs:
      - poison_doses
      - poison_exposure
      - harvest_state
      events: []
      capabilities:
      - poison
      - crafting
      - checks
    requires:
    - dnd2024:rules_poison_harvesting_poison
    citations:
    - SRD 5.2.1 pp. 197; builtin/srd-5.2.1/source.txt; section rules/poison_harvesting_poison.md
  dnd2024:rules/poison_burnt_othur_fumes_500_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_burnt_othur_fumes_500_gp
        implementation: dnd2024:rules_poison_burnt_othur_fumes_500_gp
        parameters:
          cost_gp: 500
          exposure_route: inhaled
          initial_save:
            ability: constitution
            dc: 13
          initial_failure:
            damage:
              dice:
                count: 3
                sides: 6
              average: 10
              type: poison
          repeat:
            timing: subject_turn_start
            ability: constitution
            dc: 13
            failure_damage:
              dice:
                count: 1
                sides: 6
              average: 3
              type: poison
            requires_initial_failure: false
          ends:
            successful_saves: 3
            consecutive_required: false
            count_initial_save_success: true
    contract:
      id: dnd2024:rules_poison_burnt_othur_fumes_500_gp
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - poison.exposure
      - turn.started
      capabilities:
      - poison
      - damage
      - saving_throws
    requires:
    - dnd2024:rules_poison_burnt_othur_fumes_500_gp
    citations:
    - SRD 5.2.1 pp. 197; builtin/srd-5.2.1/source.txt; section rules/poison_burnt_othur_fumes_500_gp.md
  dnd2024:rules/poison_malice_250_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_malice_250_gp
        implementation: dnd2024:rules_poison_malice_250_gp
        parameters:
          cost_gp: 250
          exposure_route: inhaled
          save:
            ability: constitution
            dc: 15
          failure:
            condition: poisoned
            duration:
              value: 1
              unit: hour
          while_poisoned_by_this:
            condition: blinded
    contract:
      id: dnd2024:rules_poison_malice_250_gp
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - poison.exposure
      capabilities:
      - poison
      - conditions
      - saving_throws
    requires:
    - dnd2024:rules_poison_malice_250_gp
    citations:
    - SRD 5.2.1 pp. 197; builtin/srd-5.2.1/source.txt; section rules/poison_malice_250_gp.md
  dnd2024:rules/poison_pale_tincture_250_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_pale_tincture_250_gp
        implementation: dnd2024:rules_poison_pale_tincture_250_gp
        parameters:
          cost_gp: 250
          exposure_route: ingested
          initial_save:
            ability: constitution
            dc: 16
          initial_failure:
            damage:
              dice:
                count: 1
                sides: 6
              average: 3
              type: poison
            condition: poisoned
          repeat_while_poisoned:
            interval:
              value: 24
              unit: hour
            ability: constitution
            dc: 16
            failure_damage:
              dice:
                count: 1
                sides: 6
              average: 3
              type: poison
          while_poisoned:
            healing_prohibited_for: damage_from_this_poison
            prohibition_scope: all_means
          ends:
            successful_saves: 7
            consecutive_required: false
    contract:
      id: dnd2024:rules_poison_pale_tincture_250_gp
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - poison.exposure
      - duration.expired
      - healing.pending
      capabilities:
      - poison
      - damage
      - healing
      - saving_throws
    requires:
    - dnd2024:rules_poison_pale_tincture_250_gp
    citations:
    - SRD 5.2.1 pp. 198; builtin/srd-5.2.1/source.txt; section rules/poison_pale_tincture_250_gp.md
  dnd2024:rules/poison_spider_s_sting_200_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_spider_s_sting_200_gp
        implementation: dnd2024:rules_poison_spider_s_sting_200_gp
        parameters:
          cost_gp: 200
          exposure_route: injury
          save:
            ability: constitution
            dc: 13
          failure:
            condition: poisoned
            duration:
              value: 1
              unit: hour
          failure_margin_at_least_5:
            condition: unconscious
            duration: while_poisoned_by_this
          unconscious_ends_early:
          - damage_taken
          - other_creature_action_shake_awake
    contract:
      id: dnd2024:rules_poison_spider_s_sting_200_gp
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - poison.exposure
      - damage.applied
      - action.shake_awake
      capabilities:
      - poison
      - conditions
      - saving_throws
    requires:
    - dnd2024:rules_poison_spider_s_sting_200_gp
    citations:
    - SRD 5.2.1 pp. 198; builtin/srd-5.2.1/source.txt; section rules/poison_spider_s_sting_200_gp.md
  dnd2024:rules/poison_wyvern_poison_1_200_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_wyvern_poison_1_200_gp
        implementation: dnd2024:rules_poison_wyvern_poison_1_200_gp
        parameters:
          cost_gp: 1200
          exposure_route: injury
          save:
            ability: constitution
            dc: 14
          damage:
            dice:
              count: 7
              sides: 6
            average: 24
            type: poison
          success_damage: half_round_down
    contract:
      id: dnd2024:rules_poison_wyvern_poison_1_200_gp
      inputs:
        subject: participant
      outputs:
      - effects
      events:
      - poison.exposure
      capabilities:
      - poison
      - damage
      - saving_throws
    requires:
    - dnd2024:rules_poison_wyvern_poison_1_200_gp
    citations:
    - SRD 5.2.1 pp. 198; builtin/srd-5.2.1/source.txt; section rules/poison_wyvern_poison_1_200_gp.md
  dnd2024:rules/traps_example_traps:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 199; builtin/srd-5.2.1/source.txt; section rules/traps_example_traps.md
  dnd2024:rules/traps_fire_casting_statue:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/traps_fire_casting_statue
        implementation: dnd2024:rules_traps_fire_casting_statue
        parameters:
          classification: deadly
          base_levels:
            min: 1
            max: 4
          triggers:
          - pressure_plate_first_entry_on_turn
          - turn_started_on_pressure_plate
          duration: instantaneous
          reset: next_turn_start
          area:
            shape: cone
            length_ft: 15
            origin: statue
          save:
            ability: dexterity
            dc: 15
          damage:
            dice:
              count: 2
              sides: 10
            average: 11
            type: fire
          success_damage: half_round_down
          detect_magic:
            aura: evocation
            location: statue
          examine_statue:
            action: search
            distance_max_ft: 5
            ability: wisdom
            skill: perception
            dc: 10
            success: reveal_glyph
          decipher_glyph:
            requires_glyph_found: true
            action: study
            ability: intelligence
            skill: arcana
            dc: 15
            success: identify_fire_symbol
          disarm_glyph:
            requires_glyph_found: true
            action_cost: 1
            tool: sharp
            operation: deface_glyph
            result: trap_disarmed
          examine_floor:
            action: search
            ability: wisdom
            skill: perception
            dc: 15
            success: reveal_pressure_plate
          block_pressure_plate:
            object_choice:
            - iron_spike
            - similar_wedge
            result: triggering_prevented
          optional_level_scaling:
          - levels:
              min: 5
              max: 10
            damage:
              dice:
                count: 4
                sides: 10
              average: 22
            cone_length_ft: 30
          - levels:
              min: 11
              max: 16
            damage:
              dice:
                count: 10
                sides: 10
              average: 55
            cone_length_ft: 60
          - levels:
              min: 17
              max: 20
            damage:
              dice:
                count: 18
                sides: 10
              average: 99
            cone_length_ft: 120
    contract:
      id: dnd2024:rules_traps_fire_casting_statue
      inputs:
        trap: object
        targets: participant_list
        interaction: resolution
        level_band: choice
      outputs:
      - damage
      - trap_state
      - discovery
      events:
      - movement.entered
      - turn.started
      - action.search
      - action.study
      - action.disarm
      capabilities:
      - traps
      - geometry
      - damage
      - checks
      - saving_throws
    requires:
    - dnd2024:rules_traps_fire_casting_statue
    citations:
    - SRD 5.2.1 pp. 199, 200; builtin/srd-5.2.1/source.txt; section rules/traps_fire_casting_statue.md
  dnd2024:rules/traps_poisoned_needle:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/traps_poisoned_needle
        implementation: dnd2024:rules_traps_poisoned_needle
        parameters:
          classification: nuisance
          base_levels:
            min: 1
            max: 4
          triggers:
          - lock_opened_with_object_other_than_correct_key
          - disarm_failure
          duration: instantaneous
          magical_opening_exemptions:
          - knock
          - similar_magic
          save:
            ability: constitution
            dc: 11
          failure:
            damage:
              dice:
                count: 1
                sides: 10
              average: 5
              type: poison
            condition: poisoned
            condition_duration:
              value: 1
              unit: hour
          success:
            damage: half_round_down
            condition: none
          detect:
            action: search
            ability: wisdom
            skill: perception
            dc: 15
            success: needle_detected
          disarm:
            requires_detected: true
            action_cost: 1
            ability: dexterity
            skill: sleight_of_hand
            dc: 15
            failure: trigger_trap
          optional_level_scaling:
          - levels:
              min: 5
              max: 10
            damage:
              dice:
                count: 2
                sides: 10
              average: 11
            save_dc: 13
          - levels:
              min: 11
              max: 16
            damage:
              dice:
                count: 4
                sides: 10
              average: 22
            save_dc: 15
          - levels:
              min: 17
              max: 20
            damage:
              dice:
                count: 10
                sides: 10
              average: 55
            save_dc: 17
    contract:
      id: dnd2024:rules_traps_poisoned_needle
      inputs:
        trap: object
        subject: participant
        interaction: resolution
        level_band: choice
      outputs:
      - damage
      - conditions
      - trap_state
      - discovery
      events:
      - lock.opened
      - action.search
      - action.disarm
      capabilities:
      - traps
      - damage
      - conditions
      - checks
      - saving_throws
    requires:
    - dnd2024:rules_traps_poisoned_needle
    citations:
    - SRD 5.2.1 pp. 200, 201; builtin/srd-5.2.1/source.txt; section rules/traps_poisoned_needle.md
  dnd2024:rules/combat_encounters:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 202; builtin/srd-5.2.1/source.txt; section rules/combat_encounters.md
  dnd2024:rules/combat_encounters_step_2_determine_your_xp_budget:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_encounters_step_2_determine_your_xp_budget
        implementation: dnd2024:rules_combat_encounters_step_2_determine_your_xp_budget
        parameters:
          per_character_xp:
          - level: 1
            low: 50
            moderate: 75
            high: 100
          - level: 2
            low: 100
            moderate: 150
            high: 200
          - level: 3
            low: 150
            moderate: 225
            high: 400
          - level: 4
            low: 250
            moderate: 375
            high: 500
          - level: 5
            low: 500
            moderate: 750
            high: 1100
          - level: 6
            low: 600
            moderate: 1000
            high: 1400
          - level: 7
            low: 750
            moderate: 1300
            high: 1700
          - level: 8
            low: 1000
            moderate: 1700
            high: 2100
          - level: 9
            low: 1300
            moderate: 2000
            high: 2600
          - level: 10
            low: 1600
            moderate: 2300
            high: 3100
          - level: 11
            low: 1900
            moderate: 2900
            high: 4100
          - level: 12
            low: 2200
            moderate: 3700
            high: 4700
          - level: 13
            low: 2600
            moderate: 4200
            high: 5400
          - level: 14
            low: 2900
            moderate: 4900
            high: 6200
          - level: 15
            low: 3300
            moderate: 5400
            high: 7800
          - level: 16
            low: 3800
            moderate: 6100
            high: 9800
          - level: 17
            low: 4500
            moderate: 7200
            high: 11700
          - level: 18
            low: 5000
            moderate: 8700
            high: 14200
          - level: 19
            low: 5500
            moderate: 10700
            high: 17200
          - level: 20
            low: 6400
            moderate: 13200
            high: 22000
          encounter_budget:
            op: multiply
            args:
            - var: per_character_budget
            - var: party_size
    contract:
      id: dnd2024:rules_combat_encounters_step_2_determine_your_xp_budget
      inputs:
        party_level: integer
        party_size: integer
        difficulty: choice
      outputs:
      - xp_budget
      events: []
      capabilities:
      - encounter_building
    requires:
    - dnd2024:rules_combat_encounters_step_2_determine_your_xp_budget
    citations:
    - SRD 5.2.1 pp. 202; builtin/srd-5.2.1/source.txt; section rules/combat_encounters_step_2_determine_your_xp_budget.md
  dnd2024:rules/combat_encounters_many_creatures:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_encounters_many_creatures
        implementation: dnd2024:rules_combat_encounters_many_creatures
        parameters:
          guideline: true
          if_creatures_per_character_greater_than: 2
          recommendation: include_quickly_defeated_fragile_creatures
          priority_character_levels:
          - 1
          - 2
          mandatory_combat_modifier: false
    contract:
      id: dnd2024:rules_combat_encounters_many_creatures
      inputs:
        party: participant_list
        enemies: participant_list
      outputs:
      - encounter_composition_guidance
      events: []
      capabilities:
      - encounter_building
    requires:
    - dnd2024:rules_combat_encounters_many_creatures
    citations:
    - SRD 5.2.1 pp. 203; builtin/srd-5.2.1/source.txt; section rules/combat_encounters_many_creatures.md
  dnd2024:rules/combat_encounters_number_of_stat_blocks:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 203; builtin/srd-5.2.1/source.txt; section rules/combat_encounters_number_of_stat_blocks.md
  dnd2024:rules/magic_items_2:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 204; builtin/srd-5.2.1/source.txt; section rules/magic_items_2.md
  dnd2024:rules/magic_item_categories_potions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_categories_potions
        implementation: dnd2024:rules_magic_item_categories_potions
        parameters:
          category: potion
          typical_volume_ounces: 1
          consumable: true
          activation:
            drink: bonus_action
            administer_to_other: bonus_action
            oil_application: item_specific_time
            effect_starts: immediately
            consumed_on_use: true
          mixing:
            triggers:
            - combine_in_container
            - drink_while_other_potion_effect_active
            initial_potions: 2
            extra_rolls: one_per_additional_potion
            combine_results: true
            reveal_hidden_results: when_evident
            die:
              count: 1
              sides: 100
            table:
            - min: 1
              max: 1
              effects:
                both_potions_lose_effects: true
                explosion:
                  magical: true
                  shape: sphere
                  radius_ft: 5
                  origin: mixture
                  damage:
                    dice:
                      count: 4
                      sides: 10
                    type: force
                  save: none
            - min: 2
              max: 8
              effects:
                both_potions_lose_effects: true
                replace_with_poison:
                  route: ingested
                  choice_authority: gm
            - min: 9
              max: 15
              effects:
                both_potions_lose_effects: true
            - min: 16
              max: 25
              effects:
                one_potion_loses_effect: true
                potion_selection: gm_choice
            - min: 26
              max: 35
              effects:
                both_work: true
                numerical_effect_multiplier: 0.5
                duration_multiplier: 0.5
                if_no_numerical_effect_and_no_duration: potion_loses_effect
            - min: 36
              max: 90
              effects:
                both_work_normally: true
            - min: 91
              max: 99
              effects:
                both_work: true
                one_potion_numerical_effect_multiplier: 2
                same_potion_duration_multiplier: 2
                potion_selection: gm_choice
                if_neither_has_doublable_property: both_work_normally
            - min: 100
              max: 100
              effects:
                working_potions: 1
                potion_selection: gm_choice
                selected_effect_duration: permanent
                effect_selection:
                  authority: gm
                  priorities:
                  - simplest
                  - most_fun
                examples:
                - potion: healing
                  permanent_effect: hp_max_increase
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 2
                - potion: invisibility
                  permanent_effect: invisible_condition
                dispel_magic_or_similar_may_end:
                  authority: gm
                  choice: boolean
    contract:
      id: dnd2024:rules_magic_item_categories_potions
      inputs:
        subject: participant
        potions: item_list
        mixing_choices:
          type: resolution
          required: false
      outputs:
      - potion_effects
      - consumed_items
      - miscibility_result
      events:
      - potion.used
      - potions.mixed
      capabilities:
      - potions
      - random_tables
      - damage
      - effects
    requires:
    - dnd2024:rules_magic_item_categories_potions
    citations:
    - SRD 5.2.1 pp. 204; builtin/srd-5.2.1/source.txt; section rules/magic_item_categories_potions.md
  dnd2024:rules/magic_item_categories_scrolls:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_categories_scrolls
        implementation: dnd2024:rules_magic_item_categories_scrolls
        parameters:
          category: scroll
          consumable: true
          activation_requires: read_scroll
          reader_eligibility: understands_any_written_language
          item_specific_eligibility_overrides: true
          on_magic_invoked:
            reuse_prohibited: true
            destruction_modes:
            - words_fade
            - crumble_to_dust
    contract:
      id: dnd2024:rules_magic_item_categories_scrolls
      inputs:
        subject: participant
        scroll: item_instance
      outputs:
      - activation_eligibility
      - consumed_item
      events:
      - scroll.activated
      capabilities:
      - scrolls
      - languages
    requires:
    - dnd2024:rules_magic_item_categories_scrolls
    citations:
    - SRD 5.2.1 pp. 205; builtin/srd-5.2.1/source.txt; section rules/magic_item_categories_scrolls.md
  dnd2024:rules/magic_item_categories_weapons:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_categories_weapons
        implementation: dnd2024:rules_magic_item_categories_weapons
        parameters:
          weapon_type:
            if_specified: item_definition
            otherwise: choose_or_randomize_equipment_weapon_type
          magic_weapon_with_ammunition_property:
            fired_ammunition_counts_as_magical_weapon: true
    contract:
      id: dnd2024:rules_magic_item_categories_weapons
      inputs:
        weapon: item_instance
        weapon_type:
          type: choice
          required: false
      outputs:
      - weapon_type
      - ammunition_magic_status
      events: []
      capabilities:
      - magic_items
      - weapons
    requires:
    - dnd2024:rules_magic_item_categories_weapons
    citations:
    - SRD 5.2.1 pp. 205; builtin/srd-5.2.1/source.txt; section rules/magic_item_categories_weapons.md
  dnd2024:rules/magic_item_rarity_magic_item_values_by_rarity:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_rarity_magic_item_values_by_rarity
        implementation: dnd2024:rules_magic_item_rarity_magic_item_values_by_rarity
        parameters:
          magic_trade_enabled_by: gm
          typical_market_locations:
            common:
            - town
            - city
            uncommon:
            - city
            rare:
            - city
            rarer:
            - wondrous_location
          rarity_base_value_gp:
            common: 100
            uncommon: 400
            rare: 4000
            very_rare: 40000
            legendary: 200000
            artifact: null
          artifact_valuation: priceless
          consumable_except_spell_scroll_multiplier: 0.5
          spell_scroll_value:
            op: multiply
            args:
            - 2
            - var: scroll.scribing_cost_gp
          incorporated_equipment_cost: add_full_purchase_cost
          seller_payment_choice:
          - coin
          - service
          seller_service_terms_authority: gm
    contract:
      id: dnd2024:rules_magic_item_rarity_magic_item_values_by_rarity
      inputs:
        item: item_instance
        campaign: campaign
      outputs:
      - item_value
      - payment_terms
      events: []
      capabilities:
      - magic_item_economy
    requires:
    - dnd2024:rules_magic_item_rarity_magic_item_values_by_rarity
    citations:
    - SRD 5.2.1 pp. 205, 206; builtin/srd-5.2.1/source.txt; section rules/magic_item_rarity_magic_item_values_by_rarity.md
  dnd2024:rules/activating_a_magic_item_consumable_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/activating_a_magic_item_consumable_items
        implementation: dnd2024:rules_activating_a_magic_item_consumable_items
        parameters:
          consumed_on_activation: true
          on_consumed:
            magic_lost: true
    contract:
      id: dnd2024:rules_activating_a_magic_item_consumable_items
      inputs:
        item: item_instance
      outputs:
      - consumed_item
      events:
      - item.activated
      capabilities:
      - magic_items
      - inventory
    requires:
    - dnd2024:rules_activating_a_magic_item_consumable_items
    citations:
    - SRD 5.2.1 pp. 206; builtin/srd-5.2.1/source.txt; section rules/activating_a_magic_item_consumable_items.md
  dnd2024:rules/the_next_dawn:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/the_next_dawn
        implementation: dnd2024:rules_the_next_dawn
        parameters:
          recharge_event: item_specified
          if_recharge_event_absent_on_world_or_plane:
            replacement_time_authority: gm
            replacement_time_input: duration
    contract:
      id: dnd2024:rules_the_next_dawn
      inputs:
        item: item_instance
        campaign: campaign
        replacement_time:
          type: duration
          required: false
      outputs:
      - recharge_schedule
      events: []
      capabilities:
      - item_recharge
      - calendar
    requires:
    - dnd2024:rules_the_next_dawn
    citations:
    - SRD 5.2.1 pp. 206; builtin/srd-5.2.1/source.txt; section rules/the_next_dawn.md
  dnd2024:rules/crafting_magic_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/crafting_magic_items
        implementation: dnd2024:rules_crafting_magic_items
        parameters:
          special_crafting_rule_exceptions:
            potion_of_healing:
              key: dnd2024:rules/brewing_potions_of_healing
            spell_scroll:
              key: dnd2024:rules/scribing_spell_scrolls
          other_items_rule_scope: magic_item_crafting
          requirement_rules:
          - key: dnd2024:rules/crafting_magic_items_arcana_proficiency
          - key: dnd2024:rules/crafting_magic_items_tools
          - key: dnd2024:rules/crafting_magic_items_spells
          - key: dnd2024:rules/crafting_magic_items_time_and_cost
    contract:
      id: dnd2024:rules_crafting_magic_items
      inputs:
        item: definition
      outputs:
      - crafting_rule_selection
      events: []
      capabilities:
      - crafting
    requires:
    - dnd2024:rules_crafting_magic_items
    citations:
    - SRD 5.2.1 pp. 206; builtin/srd-5.2.1/source.txt; section rules/crafting_magic_items.md
  dnd2024:rules/crafting_magic_items_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/crafting_magic_items_spells
        implementation: dnd2024:rules_crafting_magic_items_spells
        parameters:
          if_item_grants_spell_casting:
            required_prepared_spells: all_spells_castable_from_item
            timing: each_crafting_day
    contract:
      id: dnd2024:rules_crafting_magic_items_spells
      inputs:
        crafter: participant
        item: definition
      outputs:
      - crafting_eligibility
      events:
      - crafting.day_started
      capabilities:
      - crafting
      - spell_preparation
    requires:
    - dnd2024:rules_crafting_magic_items_spells
    citations:
    - SRD 5.2.1 pp. 207; builtin/srd-5.2.1/source.txt; section rules/crafting_magic_items_spells.md
  dnd2024:rules/sentient_magic_items_sentient_magic_item_traits:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 207; builtin/srd-5.2.1/source.txt; section rules/sentient_magic_items_sentient_magic_item_traits.md
  dnd2024:rules/sentient_magic_items_communication:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sentient_magic_items_communication
        implementation: dnd2024:rules_sentient_magic_items_communication
        parameters:
          selection:
          - gm_choice
          - random_roll
          die:
            count: 1
            sides: 10
          table:
          - min: 1
            max: 6
            modes:
            - transmit_emotions
            recipient: carrying_or_wielding_creature
          - min: 7
            max: 9
            modes:
            - speech
            language_count_min: 1
            languages: gm_choice
          - min: 10
            max: 10
            modes:
            - speech
            - telepathy
            language_count_min: 1
            languages: gm_choice
            telepathy_recipient: carrying_or_wielding_creature
    contract:
      id: dnd2024:rules_sentient_magic_items_communication
      inputs:
        item: item_instance
        communication_choice: choice
        languages:
          type: choice
          required: false
      outputs:
      - item_communication
      events: []
      capabilities:
      - sentient_items
      - random_tables
      - languages
    requires:
    - dnd2024:rules_sentient_magic_items_communication
    citations:
    - SRD 5.2.1 pp. 207, 208; builtin/srd-5.2.1/source.txt; section rules/sentient_magic_items_communication.md
  dnd2024:rules/sentient_magic_items_conflict:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sentient_magic_items_conflict
        implementation: dnd2024:rules_sentient_magic_items_conflict
        parameters:
          conflict_trigger:
            bearer_opposes:
            - item_alignment
            - item_purpose
            conflict_occurrence_authority: gm
          initial_save:
            ability: charisma
            dc:
              op: add
              args:
              - 12
              - var: item.charisma_modifier
          failure_demands:
            count_min: 1
            selection: item_controller
            options:
            - pursue_item_goals_exclusively
            - dispose_of_repugnant_possessions
            - transfer_item_to_another_bearer
            - carry_or_wear_item_at_all_times
          noncompliance_responses:
            choice: item_controller
            options:
            - prohibit_bearer_attunement
            - suppress_selected_activated_properties
            - attempt_bearer_control
          control:
            save:
              ability: charisma
              dc:
                op: add
                args:
                - 12
                - var: item.charisma_modifier
            failure:
              condition: charmed
              duration:
                dice:
                  count: 1
                  sides: 12
                unit: hour
              must_try_obey_item_commands: true
            repeat_save:
              trigger: bearer_damage_taken
              success: end_control
            attempt_recharge: next_dawn
            recharge_required_on_success_or_failure: true
    contract:
      id: dnd2024:rules_sentient_magic_items_conflict
      inputs:
        bearer: participant
        item: item_instance
        item_choices: resolution
      outputs:
      - demands
      - suppressed_properties
      - attunement_eligibility
      - control_effect
      events:
      - sentient_item.conflict
      - damage.applied
      - dawn
      capabilities:
      - sentient_items
      - conditions
      - saving_throws
      - attunement
    requires:
    - dnd2024:rules_sentient_magic_items_conflict
    citations:
    - SRD 5.2.1 pp. 208; builtin/srd-5.2.1/source.txt; section rules/sentient_magic_items_conflict.md
  dnd2024:rules/parts_of_a_stat_block:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 254; builtin/srd-5.2.1/source.txt; section rules/parts_of_a_stat_block.md
  dnd2024:rules/parts_of_a_stat_block_descriptive_tags:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 254; builtin/srd-5.2.1/source.txt; section rules/parts_of_a_stat_block_descriptive_tags.md
  dnd2024:rules/parts_of_a_stat_block_initiative:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_initiative
        implementation: dnd2024:rules_parts_of_a_stat_block_initiative
        parameters:
          initiative_entry:
            first: modifier
            parenthetical: score
          resolution_choice:
          - roll_with_modifier
          - use_fixed_score
          default_modifier: dexterity_modifier
          additional_modifiers: stat_block
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_initiative
      inputs:
        monster: participant
        mode: choice
      outputs:
      - initiative
      events: []
      capabilities:
      - initiative
    requires:
    - dnd2024:rules_parts_of_a_stat_block_initiative
    citations:
    - SRD 5.2.1 pp. 254; builtin/srd-5.2.1/source.txt; section rules/parts_of_a_stat_block_initiative.md
  dnd2024:rules/parts_of_a_stat_block_ability_scores:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_ability_scores
        implementation: dnd2024:rules_parts_of_a_stat_block_ability_scores
        parameters:
          abilities:
          - strength
          - dexterity
          - constitution
          - intelligence
          - wisdom
          - charisma
          each_has:
          - score
          - ability_modifier
          - saving_throw_modifier
          source: stat_block
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_ability_scores
      inputs:
        subject: participant
      outputs:
      - ability_statistics
      events: []
      capabilities:
      - monster_statistics
    requires:
    - dnd2024:rules_parts_of_a_stat_block_ability_scores
    citations:
    - SRD 5.2.1 pp. 255; builtin/srd-5.2.1/source.txt; section rules/parts_of_a_stat_block_ability_scores.md
  dnd2024:rules/parts_of_a_stat_block_immunities:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_immunities
        implementation: dnd2024:rules_parts_of_a_stat_block_immunities
        parameters:
          immunity_kinds:
          - damage_type
          - condition
          entry_order:
          - damage_types
          - conditions
          damage_immunity: prevent_matching_damage
          condition_immunity: prevent_matching_condition
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_immunities
      inputs:
        monster: participant
        effect: resolution
      outputs:
      - immunity_result
      events: []
      capabilities:
      - immunities
    requires:
    - dnd2024:rules_parts_of_a_stat_block_immunities
    citations:
    - SRD 5.2.1 pp. 255; builtin/srd-5.2.1/source.txt; section rules/parts_of_a_stat_block_immunities.md
  dnd2024:rules/parts_of_a_stat_block_equipping_a_monster_with_other_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_equipping_a_monster_with_other_items
        implementation: dnd2024:rules_parts_of_a_stat_block_equipping_a_monster_with_other_items
        parameters:
          additional_equipment:
            authority: gm
            selection: item_list
          after_monster_death:
            recoverable_equipment: gm_choice
            usable_equipment: gm_choice
          magic_item_attunement:
            key: dnd2024:glossary/attunement
          class_tag_qualifies_for_class_attunement: true
          combat_magic_items_may_change_challenge_rating: true
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_equipping_a_monster_with_other_items
      inputs:
        monster: participant
        equipment: item_list
        recovery_choices:
          type: resolution
          required: false
      outputs:
      - equipment
      - recoverable_loot
      - attunement_eligibility
      events: []
      capabilities:
      - monster_equipment
      - attunement
      - loot
    requires:
    - dnd2024:rules_parts_of_a_stat_block_equipping_a_monster_with_other_items
    citations:
    - SRD 5.2.1 pp. 255, 256; builtin/srd-5.2.1/source.txt; section rules/parts_of_a_stat_block_equipping_a_monster_with_other_items.md
  dnd2024:rules/parts_of_a_stat_block_telepathy:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_telepathy
        implementation: dnd2024:rules_parts_of_a_stat_block_telepathy
        parameters:
          rule:
            key: dnd2024:glossary/telepathy
          communication: mental
          magical: true
          range_source: stat_block
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_telepathy
      inputs:
        subject: participant
      outputs:
      - effects
      events: []
      capabilities:
      - telepathy
      - languages
    requires:
    - dnd2024:rules_parts_of_a_stat_block_telepathy
    citations:
    - SRD 5.2.1 pp. 256; builtin/srd-5.2.1/source.txt; section rules/parts_of_a_stat_block_telepathy.md
  dnd2024:rules/parts_of_a_stat_block_proficiency_bonus:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_proficiency_bonus
        implementation: dnd2024:rules_parts_of_a_stat_block_proficiency_bonus
        parameters:
          proficiency_bonus_by_cr:
          - cr_min: 0
            cr_max: 4
            bonus: 2
          - cr_min: 5
            cr_max: 8
            bonus: 3
          - cr_min: 9
            cr_max: 12
            bonus: 4
          - cr_min: 13
            cr_max: 16
            bonus: 5
          - cr_min: 17
            cr_max: 20
            bonus: 6
          - cr_min: 21
            cr_max: 24
            bonus: 7
          - cr_min: 25
            cr_max: 28
            bonus: 8
          - cr_min: 29
            cr_max: 30
            bonus: 9
          already_included_in_stat_block:
          - saving_throws
          - skills
          - other_proficient_statistics
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_proficiency_bonus
      inputs:
        monster: participant
      outputs:
      - proficiency_bonus
      events: []
      capabilities:
      - monster_statistics
      - proficiency
    requires:
    - dnd2024:rules_parts_of_a_stat_block_proficiency_bonus
    citations:
    - SRD 5.2.1 pp. 256; builtin/srd-5.2.1/source.txt; section rules/parts_of_a_stat_block_proficiency_bonus.md
  dnd2024:rules/parts_of_a_stat_block_attack_notation:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_attack_notation
        implementation: dnd2024:rules_parts_of_a_stat_block_attack_notation
        parameters:
          attack_fields:
          - melee_or_ranged
          - attack_bonus
          - reach_or_range
          - hit_effects
          default_targets: 1
          specific_target_count_overrides: true
          notation_branches:
            hit: only_on_hit
            miss: only_on_miss
            hit_or_miss: always_after_attack_roll
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_attack_notation
      inputs:
        attack: definition
        resolution: resolution
      outputs:
      - attack_effects
      events: []
      capabilities:
      - monster_attacks
    requires:
    - dnd2024:rules_parts_of_a_stat_block_attack_notation
    citations:
    - SRD 5.2.1 pp. 256; builtin/srd-5.2.1/source.txt; section rules/parts_of_a_stat_block_attack_notation.md
  dnd2024:rules/parts_of_a_stat_block_multiattack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_multiattack
        implementation: dnd2024:rules_parts_of_a_stat_block_multiattack
        parameters:
          action: attack
          attack_count_and_types: multiattack_entry
          additional_abilities_allowed: multiattack_entry
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_multiattack
      inputs:
        monster: participant
        multiattack: definition
      outputs:
      - attack_sequence
      - additional_abilities
      events: []
      capabilities:
      - monster_attacks
      - action_economy
    requires:
    - dnd2024:rules_parts_of_a_stat_block_multiattack
    citations:
    - SRD 5.2.1 pp. 257; builtin/srd-5.2.1/source.txt; section rules/parts_of_a_stat_block_multiattack.md
  dnd2024:rules/parts_of_a_stat_block_reactions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_reactions
        implementation: dnd2024:rules_parts_of_a_stat_block_reactions
        parameters:
          options_and_triggers_source: monster_stat_block_reactions
          reaction_rules:
            key: dnd2024:rules/actions_reactions
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_reactions
      inputs:
        monster: participant
        trigger: event
      outputs:
      - reaction_options
      events: []
      capabilities:
      - monster_reactions
      - action_economy
    requires:
    - dnd2024:rules_parts_of_a_stat_block_reactions
    citations:
    - SRD 5.2.1 pp. 257; builtin/srd-5.2.1/source.txt; section rules/parts_of_a_stat_block_reactions.md
---

# Supplementary SRD rule contracts: group 1

Accounted for all 174 assigned notes: 147 specified contracts and 27 informational headings or advice notes. All mechanics are data under each named contract; the engine remains out of scope.

The official bundled SRD 5.2.1 source owns these rules. Citations beside each contract identify its source pages. Supplementary notes preserve the source hash recorded in their conversion metadata. Tables displaced by the PDF column order were checked in `builtin/srd-5.2.1/source.txt`: armor on p. 92, mounts and tack on p. 100, ship crews on p. 101, food and lodging on pp. 101–102, scroll costs on p. 103, travel terrain on p. 192, potion mixing on p. 204, item values on pp. 205–206, and sentient communication on p. 208.

Canonical `key` records refer to existing definitions. Class entry contracts grant the core traits and named class features, so their source mechanics remain owned by those definitions. Spell list records include every printed spell, level, school, Concentration tag, Ritual tag, and specific Material tag. Class progression rows include the resource columns that overlap the entry section.

The one-at-a-time armor note includes the adjacent numeric armor table fragment; the contract restores its armor names. The making-an-attack and healing notes include the unseen-attacker and knock-out sidebars, respectively, and encode both. Numerical advice remains a nonbinding typed guideline. Pure headings and narrative advice have no executable claim.

Source-authorized GM choices are explicit inputs or bounded choice policies. Range and area checks require geometry supplied by the future engine. Burnt Othur Fumes repeats its saving throw at each turn start until three cumulative successes, including an initial success. Pale Tincture starts its repeat cycle after the initial failure and requires seven cumulative successes. No source issue prevents this accounting.

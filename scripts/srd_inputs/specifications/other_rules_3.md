---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:rules/the_six_abilities:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/the_six_abilities
        implementation: dnd2024:rules_the_six_abilities
        parameters:
          ability_ids:
          - strength
          - dexterity
          - constitution
          - intelligence
          - wisdom
          - charisma
          applies_to:
          - characters
          - monsters
    contract:
      id: dnd2024:rules_the_six_abilities
      inputs:
        definition: definition
      outputs:
      - typed_attributes
      events: []
      capabilities:
      - definition_catalogs
    requires:
    - dnd2024:rules_the_six_abilities
    citations:
    - SRD 5.2.1 pp. 5; builtin/srd-5.2.1/source.txt; source section The Six Abilities; draft rules/the_six_abilities.md
  dnd2024:rules/d20_tests:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests
        implementation: dnd2024:rules_d20_tests
        parameters:
          test_kinds:
          - ability_check
          - saving_throw
          - attack_roll
          die:
            count: 1
            sides: 20
            add: 0
          advantage:
            dice:
              count: 2
              sides: 20
              add: 0
            keep: highest
          disadvantage:
            dice:
              count: 2
              sides: 20
              add: 0
            keep: lowest
          total:
            op: sum
            terms:
            - selected_die
            - relevant_ability_modifier
            - applicable_proficiency_bonus
            - circumstantial_bonuses_and_penalties
          success:
            op: gte
            left: total
            right: target_number
          targets:
            ability_check: difficulty_class
            saving_throw: difficulty_class
            attack_roll: armor_class
          target_number_authority: gm
    contract:
      id: dnd2024:rules_d20_tests
      inputs:
        actor: participant
        test: resolution
      outputs:
      - test_modifiers
      - test_result
      events: []
      capabilities:
      - d20_tests
      - proficiency
    requires:
    - dnd2024:rules_d20_tests
    citations:
    - SRD 5.2.1 pp. 6; builtin/srd-5.2.1/source.txt; source section D20 Tests; draft rules/d20_tests.md
  dnd2024:rules/d20_tests_proficiency_bonus:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_proficiency_bonus
        implementation: dnd2024:rules_d20_tests_proficiency_bonus
        parameters:
          proficiency_bonus:
            applies_when:
              all:
              - gm_relevant_skill_or_tool
              - actor_has_proficiency
            any_alternative_skill_suffices: true
            times: 1
    contract:
      id: dnd2024:rules_d20_tests_proficiency_bonus
      inputs:
        actor: participant
        test: resolution
      outputs:
      - test_modifiers
      - test_result
      events: []
      capabilities:
      - d20_tests
      - proficiency
    requires:
    - dnd2024:rules_d20_tests_proficiency_bonus
    citations:
    - SRD 5.2.1 pp. 6; builtin/srd-5.2.1/source.txt; source section D20 Tests; draft rules/d20_tests_proficiency_bonus.md
  dnd2024:rules/d20_tests_ability_modifier_2:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_ability_modifier_2
        implementation: dnd2024:rules_d20_tests_ability_modifier_2
        parameters:
          saving_throw_modifier: named_ability_modifier
          example_abilities:
            direct_force: strength
            dodge: dexterity
            toxic_hazard: constitution
            illusion: intelligence
            mental_assault: wisdom
            identity: charisma
    contract:
      id: dnd2024:rules_d20_tests_ability_modifier_2
      inputs:
        definition: definition
      outputs:
      - typed_attributes
      events: []
      capabilities:
      - definition_catalogs
    requires:
    - dnd2024:rules_d20_tests_ability_modifier_2
    citations:
    - SRD 5.2.1 pp. 7; builtin/srd-5.2.1/source.txt; source section D20 Tests; draft rules/d20_tests_ability_modifier_2.md
  dnd2024:rules/d20_tests_attack_rolls:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_attack_rolls
        implementation: dnd2024:rules_d20_tests_attack_rolls
        parameters:
          hit:
            op: gte
            left: attack_roll_total
            right: target_armor_class
          combat_required: false
    contract:
      id: dnd2024:rules_d20_tests_attack_rolls
      inputs:
        actor: participant
        test: resolution
      outputs:
      - test_modifiers
      - test_result
      events: []
      capabilities:
      - d20_tests
      - proficiency
    requires:
    - dnd2024:rules_d20_tests_attack_rolls
    citations:
    - SRD 5.2.1 pp. 7; builtin/srd-5.2.1/source.txt; source section D20 Tests; draft rules/d20_tests_attack_rolls.md
  dnd2024:rules/d20_tests_armor_class:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_armor_class
        implementation: dnd2024:rules_d20_tests_armor_class
        parameters:
          default_base_ac:
            op: add
            left: 10
            right: dexterity_modifier
          base_formula_choice:
            count: 1
            from: available_ac_formulas
          maximum_simultaneous_base_formulas: 1
          modifiers_from:
          - armor
          - magic_items
          - spells
          - other_rules
    contract:
      id: dnd2024:rules_d20_tests_armor_class
      inputs:
        actor: participant
        item: item_instance
        choice: choice
      outputs:
      - item_attributes
      - actor_modifiers
      events: []
      capabilities:
      - equipment
      - armor_class
      - crafting
    requires:
    - dnd2024:rules_d20_tests_armor_class
    citations:
    - SRD 5.2.1 pp. 7; builtin/srd-5.2.1/source.txt; source section D20 Tests; draft rules/d20_tests_armor_class.md
  dnd2024:rules/d20_tests_roll_two_d20s:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/d20_tests_roll_two_d20s
        implementation: dnd2024:rules_d20_tests_roll_two_d20s
        parameters:
          roll:
            count: 2
            sides: 20
            add: 0
          keep:
            advantage: highest
            disadvantage: lowest
    contract:
      id: dnd2024:rules_d20_tests_roll_two_d20s
      inputs:
        actor: participant
        test: resolution
      outputs:
      - test_modifiers
      - test_result
      events: []
      capabilities:
      - d20_tests
      - proficiency
    requires:
    - dnd2024:rules_d20_tests_roll_two_d20s
    citations:
    - SRD 5.2.1 pp. 8; builtin/srd-5.2.1/source.txt; source section D20 Tests; draft rules/d20_tests_roll_two_d20s.md
  dnd2024:rules/proficiency:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/proficiency
        implementation: dnd2024:rules_proficiency
        parameters:
          basis:
            character: total_level
            monster: challenge_rating
          table:
          - minimum: 0
            maximum: 4
            bonus: 2
          - minimum: 5
            maximum: 8
            bonus: 3
          - minimum: 9
            maximum: 12
            bonus: 4
          - minimum: 13
            maximum: 16
            bonus: 5
          - minimum: 17
            maximum: 20
            bonus: 6
          - minimum: 21
            maximum: 24
            bonus: 7
          - minimum: 25
            maximum: 28
            bonus: 8
          - minimum: 29
            maximum: 30
            bonus: 9
          applications:
          - proficient_skill_test
          - proficient_save
          - proficient_item_test
          - spell_attack
          - spell_save_dc
    contract:
      id: dnd2024:rules_proficiency
      inputs:
        actor: participant
        test: resolution
      outputs:
      - test_modifiers
      - test_result
      events: []
      capabilities:
      - d20_tests
      - proficiency
    requires:
    - dnd2024:rules_proficiency
    citations:
    - SRD 5.2.1 pp. 8; builtin/srd-5.2.1/source.txt; source section Proficiency; draft rules/proficiency.md
  dnd2024:rules/proficiency_skill_list:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/proficiency_skill_list
        implementation: dnd2024:rules_proficiency_skill_list
        parameters:
          default_skill_abilities:
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
          ability_assignment: usual_not_exclusive
    contract:
      id: dnd2024:rules_proficiency_skill_list
      inputs:
        definition: definition
      outputs:
      - typed_attributes
      events: []
      capabilities:
      - definition_catalogs
    requires:
    - dnd2024:rules_proficiency_skill_list
    citations:
    - SRD 5.2.1 pp. 9; builtin/srd-5.2.1/source.txt; source section Proficiency; draft rules/proficiency_skill_list.md
  dnd2024:rules/proficiency_equipment_proficiencies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/proficiency_equipment_proficiencies
        implementation: dnd2024:rules_proficiency_equipment_proficiencies
        parameters:
          weapon_proficiency:
            required_to_wield: false
            attack_bonus: proficiency_bonus
          tool_proficiency:
            ability_check_bonus: proficiency_bonus
            matching_skill_proficiency: advantage
            proficiency_bonus_times: 1
    contract:
      id: dnd2024:rules_proficiency_equipment_proficiencies
      inputs:
        actor: participant
        test: resolution
      outputs:
      - test_modifiers
      - test_result
      events: []
      capabilities:
      - d20_tests
      - proficiency
    requires:
    - dnd2024:rules_proficiency_equipment_proficiencies
    citations:
    - SRD 5.2.1 pp. 9; builtin/srd-5.2.1/source.txt; source section Proficiency; draft rules/proficiency_equipment_proficiencies.md
  dnd2024:rules/actions_bonus_actions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/actions_bonus_actions
        implementation: dnd2024:rules_actions_bonus_actions
        parameters:
          per_turn_limit: 1
          requires_grant: true
          turn_scope: own
          timing: actor_choice_unless_specified
          blocked_by: cannot_take_actions
    contract:
      id: dnd2024:rules_actions_bonus_actions
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_actions_bonus_actions
    citations:
    - SRD 5.2.1 pp. 10; builtin/srd-5.2.1/source.txt; source section Actions; draft rules/actions_bonus_actions.md
  dnd2024:rules/social_interaction_roleplaying:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 10, 11; builtin/srd-5.2.1/source.txt; source section Social Interaction; draft rules/social_interaction_roleplaying.md
  dnd2024:rules/exploration_adventuring_equipment:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 11; builtin/srd-5.2.1/source.txt; source section Exploration; draft rules/exploration_adventuring_equipment.md
  dnd2024:rules/exploration_light:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_light
        implementation: dnd2024:rules_exploration_light
        parameters:
          illumination:
            bright_light: normal_vision
            dim_light: lightly_obscured
            darkness: heavily_obscured
    contract:
      id: dnd2024:rules_exploration_light
      inputs:
        area: location
        illumination: choice
      outputs:
      - visibility
      events: []
      capabilities:
      - lighting
      - perception
    requires:
    - dnd2024:rules_exploration_light
    citations:
    - SRD 5.2.1 pp. 11; builtin/srd-5.2.1/source.txt; source section Exploration; draft rules/exploration_light.md
  dnd2024:rules/exploration_interacting_with_objects:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 11, 12; builtin/srd-5.2.1/source.txt; source section Exploration; draft rules/exploration_interacting_with_objects.md
  dnd2024:rules/exploration_finding_hidden_objects:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_finding_hidden_objects
        implementation: dnd2024:rules_exploration_finding_hidden_objects
        parameters:
          search_location_required: object_vicinity
          check:
            ability: wisdom
            skill: perception
            dc_authority: gm
          success_options:
          - object
          - important_details
          - object_and_important_details
          outside_vicinity:
            reveal_object: false
            roll_irrelevant: true
    contract:
      id: dnd2024:rules_exploration_finding_hidden_objects
      inputs:
        actor: participant
        searched_area: location
        hidden_objects: object_list
        check: resolution
        gm_choice: choice
      outputs:
      - revealed_objects
      - revealed_details
      events: []
      capabilities:
      - exploration
      - perception
    requires:
    - dnd2024:rules_exploration_finding_hidden_objects
    citations:
    - SRD 5.2.1 pp. 12; builtin/srd-5.2.1/source.txt; source section Exploration; draft rules/exploration_finding_hidden_objects.md
  dnd2024:rules/exploration_hazards:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_hazards
        implementation: dnd2024:rules_exploration_hazards
        parameters:
          hazard_definitions:
          - key: dnd2024:glossary/burning_hazard
          - key: dnd2024:glossary/falling_hazard
          - key: dnd2024:glossary/suffocation_hazard
          - key: dnd2024:glossary/dehydration_hazard
          - key: dnd2024:glossary/malnutrition_hazard
    contract:
      id: dnd2024:rules_exploration_hazards
      inputs: {}
      outputs:
      - hazard_definitions
      events: []
      capabilities:
      - definition_catalogs
    requires:
    - dnd2024:rules_exploration_hazards
    citations:
    - SRD 5.2.1 pp. 12; builtin/srd-5.2.1/source.txt; source section Exploration; draft rules/exploration_hazards.md
  dnd2024:rules/exploration_vehicles:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/exploration_vehicles
        implementation: dnd2024:rules_exploration_vehicles
        parameters:
          land_vehicle:
            pace_choice: true
          water_vehicle:
            pace_choice: false
            speed_source: vehicle
            travel_hours_per_day_maximum: 24
            daily_hours_authority: gm
            daily_hours_factors:
            - vessel
            - crew_size
    contract:
      id: dnd2024:rules_exploration_vehicles
      inputs:
        travelers: participant_list
        vehicle:
          type: object
          required: false
        journey: resolution
      outputs:
      - travel_rate
      - travel_limits
      events: []
      capabilities:
      - travel
      - vehicles
    requires:
    - dnd2024:rules_exploration_vehicles
    citations:
    - SRD 5.2.1 pp. 12; builtin/srd-5.2.1/source.txt; source section Exploration; draft rules/exploration_vehicles.md
  dnd2024:rules/combat_combat_step_by_step:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_combat_step_by_step
        implementation: dnd2024:rules_combat_combat_step_by_step
        parameters:
          sequence:
          - operation: assign_positions
            authority: gm
            inputs:
            - marching_order
            - stated_positions
          - operation: roll_initiative
            participants: all
          - operation: take_turns
            order: initiative
            round_end: all_participants_acted
            repeat_until: combat_ended
    contract:
      id: dnd2024:rules_combat_combat_step_by_step
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_combat_combat_step_by_step
    citations:
    - SRD 5.2.1 pp. 13; builtin/srd-5.2.1/source.txt; source section Combat; draft rules/combat_combat_step_by_step.md
  dnd2024:rules/combat_ending_combat:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_ending_combat
        implementation: dnd2024:rules_combat_ending_combat
        parameters:
          end_conditions:
            any:
            - side_killed
            - side_unconscious
            - side_surrendered
            - side_fled
            - mutual_agreement
    contract:
      id: dnd2024:rules_combat_ending_combat
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_combat_ending_combat
    citations:
    - SRD 5.2.1 pp. 14; builtin/srd-5.2.1/source.txt; source section Combat; draft rules/combat_ending_combat.md
  dnd2024:rules/combat_breaking_up_your_move:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_breaking_up_your_move
        implementation: dnd2024:rules_combat_breaking_up_your_move
        parameters:
          movement_segments_allowed: true
          interleave_with:
          - action
          - bonus_action
          - reaction
          turn_scope: same_turn
          movement_budget: remaining_movement
    contract:
      id: dnd2024:rules_combat_breaking_up_your_move
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_combat_breaking_up_your_move
    citations:
    - SRD 5.2.1 pp. 14; builtin/srd-5.2.1/source.txt; source section Combat; draft rules/combat_breaking_up_your_move.md
  dnd2024:rules/combat_moving_around_other_creatures:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_moving_around_other_creatures
        implementation: dnd2024:rules_combat_moving_around_other_creatures
        parameters:
          pass_through:
            any:
            - ally
            - occupant_incapacitated
            - occupant_tiny
            - absolute_size_category_difference_minimum: 2
          difficult_terrain:
            unless_any:
            - occupant_tiny
            - ally
          willing_end_move_in_occupied_space: false
          occupied_space_turn_end:
            condition: prone
            unless_any:
            - mover_tiny
            - mover_larger_than_occupant
    contract:
      id: dnd2024:rules_combat_moving_around_other_creatures
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_combat_moving_around_other_creatures
    citations:
    - SRD 5.2.1 pp. 14; builtin/srd-5.2.1/source.txt; source section Combat; draft rules/combat_moving_around_other_creatures.md
  dnd2024:rules/combat_ranged_attacks:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 15; builtin/srd-5.2.1/source.txt; source section Combat; draft rules/combat_ranged_attacks.md
  dnd2024:rules/combat_melee_attacks:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_melee_attacks
        implementation: dnd2024:rules_combat_melee_attacks
        parameters:
          target_distance_maximum: attacker_reach
          attack_kind: melee
    contract:
      id: dnd2024:rules_combat_melee_attacks
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_combat_melee_attacks
    citations:
    - SRD 5.2.1 pp. 15; builtin/srd-5.2.1/source.txt; source section Combat; draft rules/combat_melee_attacks.md
  dnd2024:rules/combat_mounted_combat:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_mounted_combat
        implementation: dnd2024:rules_combat_mounted_combat
        parameters:
          mount_eligibility:
            willing: true
            size_categories_larger_minimum: 1
            anatomy_appropriate:
              authority: gm
              type: boolean
    contract:
      id: dnd2024:rules_combat_mounted_combat
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_combat_mounted_combat
    citations:
    - SRD 5.2.1 pp. 15; builtin/srd-5.2.1/source.txt; source section Combat; draft rules/combat_mounted_combat.md
  dnd2024:rules/combat_falling_off:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_falling_off
        implementation: dnd2024:rules_combat_falling_off
        parameters:
          triggers:
          - mount_forced_movement_pending
          - rider_knocked_prone
          - mount_knocked_prone
          save:
            ability: dexterity
            dc: 10
          failure:
            dismount: true
            condition: prone
            destination:
              unoccupied: true
              within_mount_feet: 5
    contract:
      id: dnd2024:rules_combat_falling_off
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events:
      - movement.forced_pending
      - condition.applied
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_combat_falling_off
    citations:
    - SRD 5.2.1 pp. 16; builtin/srd-5.2.1/source.txt; source section Combat; draft rules/combat_falling_off.md
  dnd2024:rules/combat_fire_resistance:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_fire_resistance
        implementation: dnd2024:rules_combat_fire_resistance
        parameters:
          predicate: underwater
          damage_resistance:
          - fire
          applies_to:
          - creatures
          - objects
    contract:
      id: dnd2024:rules_combat_fire_resistance
      inputs:
        target: participant
        damage:
          type: resolution
          required: false
        event: event
      outputs:
      - hit_point_updates
      - condition_updates
      - death_save_updates
      events: []
      capabilities:
      - damage
      - healing
      - death_saves
    requires:
    - dnd2024:rules_combat_fire_resistance
    citations:
    - SRD 5.2.1 pp. 16; builtin/srd-5.2.1/source.txt; source section Combat; draft rules/combat_fire_resistance.md
  dnd2024:rules/damage_and_healing_damage_rolls:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_damage_rolls
        implementation: dnd2024:rules_damage_and_healing_damage_rolls
        parameters:
          damage:
            op: max
            terms:
            - 0
            - op: sum
              terms:
              - damage_dice
              - modifiers
          weapon_roll_ability_modifier: same_as_attack
          spell_modifiers: specified_by_spell
          fixed_damage_ability_modifier_default: false
    contract:
      id: dnd2024:rules_damage_and_healing_damage_rolls
      inputs:
        target: participant
        damage:
          type: resolution
          required: false
        event: event
      outputs:
      - hit_point_updates
      - condition_updates
      - death_save_updates
      events: []
      capabilities:
      - damage
      - healing
      - death_saves
    requires:
    - dnd2024:rules_damage_and_healing_damage_rolls
    citations:
    - SRD 5.2.1 pp. 16; builtin/srd-5.2.1/source.txt; source section Damage and Healing; draft rules/damage_and_healing_damage_rolls.md
  dnd2024:rules/damage_and_healing_damage_against_multiple_targets:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_damage_against_multiple_targets
        implementation: dnd2024:rules_damage_and_healing_damage_against_multiple_targets
        parameters:
          shared_damage_roll:
            minimum_targets: 2
            requires_simultaneous_saving_throws: true
            roll_count: 1
    contract:
      id: dnd2024:rules_damage_and_healing_damage_against_multiple_targets
      inputs:
        target: participant
        damage:
          type: resolution
          required: false
        event: event
      outputs:
      - hit_point_updates
      - condition_updates
      - death_save_updates
      events: []
      capabilities:
      - damage
      - healing
      - death_saves
    requires:
    - dnd2024:rules_damage_and_healing_damage_against_multiple_targets
    citations:
    - SRD 5.2.1 pp. 16; builtin/srd-5.2.1/source.txt; source section Damage and Healing; draft rules/damage_and_healing_damage_against_multiple_targets.md
  dnd2024:rules/damage_and_healing_resistance_and_vulnerability:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_resistance_and_vulnerability
        implementation: dnd2024:rules_damage_and_healing_resistance_and_vulnerability
        parameters:
          resistance:
            multiplier: 0.5
            round: down
          vulnerability:
            multiplier: 2
          applies_to:
          - creature
          - object
    contract:
      id: dnd2024:rules_damage_and_healing_resistance_and_vulnerability
      inputs:
        target: participant
        damage:
          type: resolution
          required: false
        event: event
      outputs:
      - hit_point_updates
      - condition_updates
      - death_save_updates
      events: []
      capabilities:
      - damage
      - healing
      - death_saves
    requires:
    - dnd2024:rules_damage_and_healing_resistance_and_vulnerability
    citations:
    - SRD 5.2.1 pp. 17; builtin/srd-5.2.1/source.txt; source section Damage and Healing; draft rules/damage_and_healing_resistance_and_vulnerability.md
  dnd2024:rules/damage_and_healing_immunity:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_immunity
        implementation: dnd2024:rules_damage_and_healing_immunity
        parameters:
          damage_immunity:
            damage: 0
          condition_immunity:
            apply_condition: false
    contract:
      id: dnd2024:rules_damage_and_healing_immunity
      inputs:
        target: participant
        damage:
          type: resolution
          required: false
        event: event
      outputs:
      - hit_point_updates
      - condition_updates
      - death_save_updates
      events: []
      capabilities:
      - damage
      - healing
      - death_saves
    requires:
    - dnd2024:rules_damage_and_healing_immunity
    citations:
    - SRD 5.2.1 pp. 17; builtin/srd-5.2.1/source.txt; source section Damage and Healing; draft rules/damage_and_healing_immunity.md
  dnd2024:rules/damage_and_healing_instant_death:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_instant_death
        implementation: dnd2024:rules_damage_and_healing_instant_death
        parameters:
          death_conditions:
          - subject: monster
            hp: 0
            exception:
              authority: gm
              choice: use_character_death_rules
          - hp_maximum: 0
          - subject: character
            hp_reduced_to: 0
            remaining_damage_gte: hp_maximum
          death_rules:
            key: dnd2024:rules/damage_and_healing_death_saving_throws
    contract:
      id: dnd2024:rules_damage_and_healing_instant_death
      inputs:
        target: participant
        damage:
          type: resolution
          required: false
        event: event
      outputs:
      - hit_point_updates
      - condition_updates
      - death_save_updates
      events: []
      capabilities:
      - damage
      - healing
      - death_saves
    requires:
    - dnd2024:rules_damage_and_healing_instant_death
    citations:
    - SRD 5.2.1 pp. 17; builtin/srd-5.2.1/source.txt; source section Damage and Healing; draft rules/damage_and_healing_instant_death.md
  dnd2024:rules/damage_and_healing_death_saving_throws:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_death_saving_throws
        implementation: dnd2024:rules_damage_and_healing_death_saving_throws
        parameters:
          trigger:
            event: turn.started
            hp: 0
            stable: false
          save:
            die:
              count: 1
              sides: 20
              add: 0
            ability_modifier: null
            dc: 10
          success_threshold: 3
          failure_threshold: 3
          third_success: stable
          third_failure: dead
          consecutive_required: false
          reset_both_counters_on:
          - hp_regained
          - become_stable
          natural_die_overrides:
            '1':
              failures: 2
            '20':
              heal_hp: 1
          damage_at_zero:
            failure_increment: 1
            critical_hit_failure_increment: 2
            instant_death_when_damage_gte: hp_maximum
    contract:
      id: dnd2024:rules_damage_and_healing_death_saving_throws
      inputs:
        target: participant
        damage:
          type: resolution
          required: false
        event: event
      outputs:
      - hit_point_updates
      - condition_updates
      - death_save_updates
      events:
      - turn.started
      - damage.applied
      - healing.applied
      capabilities:
      - damage
      - healing
      - death_saves
    requires:
    - dnd2024:rules_damage_and_healing_death_saving_throws
    citations:
    - SRD 5.2.1 pp. 17, 18; builtin/srd-5.2.1/source.txt; source section Damage and Healing; draft rules/damage_and_healing_death_saving_throws.md
  dnd2024:rules/damage_and_healing_lose_temporary_hit_points_first:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_lose_temporary_hit_points_first
        implementation: dnd2024:rules_damage_and_healing_lose_temporary_hit_points_first
        parameters:
          damage_absorption_order:
          - temporary_hp
          - hp
          temporary_hp_absorbed:
            op: min
            terms:
            - damage
            - temporary_hp
          hp_damage:
            op: max
            terms:
            - 0
            - op: subtract
              left: damage
              right: temporary_hp
    contract:
      id: dnd2024:rules_damage_and_healing_lose_temporary_hit_points_first
      inputs:
        target: participant
        damage:
          type: resolution
          required: false
        event: event
      outputs:
      - hit_point_updates
      - condition_updates
      - death_save_updates
      events: []
      capabilities:
      - damage
      - healing
      - death_saves
    requires:
    - dnd2024:rules_damage_and_healing_lose_temporary_hit_points_first
    citations:
    - SRD 5.2.1 pp. 18; builtin/srd-5.2.1/source.txt; source section Damage and Healing; draft rules/damage_and_healing_lose_temporary_hit_points_first.md
  dnd2024:rules/damage_and_healing_they_re_not_hit_points_or_healing:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/damage_and_healing_they_re_not_hit_points_or_healing
        implementation: dnd2024:rules_damage_and_healing_they_re_not_hit_points_or_healing
        parameters:
          temporary_hp:
            add_to_hp: false
            restored_by_healing: false
            counts_as_healing: false
            allowed_at_full_hp: true
            restores_consciousness_at_zero_hp: false
    contract:
      id: dnd2024:rules_damage_and_healing_they_re_not_hit_points_or_healing
      inputs:
        target: participant
        damage:
          type: resolution
          required: false
        event: event
      outputs:
      - hit_point_updates
      - condition_updates
      - death_save_updates
      events: []
      capabilities:
      - damage
      - healing
      - death_saves
    requires:
    - dnd2024:rules_damage_and_healing_they_re_not_hit_points_or_healing
    citations:
    - SRD 5.2.1 pp. 18; builtin/srd-5.2.1/source.txt; source section Damage and Healing; draft rules/damage_and_healing_they_re_not_hit_points_or_healing.md
  dnd2024:rules/choose_a_character_sheet:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 19; builtin/srd-5.2.1/source.txt; source section Choose a Character Sheet; draft rules/choose_a_character_sheet.md
  dnd2024:rules/create_your_character_write_your_level:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_write_your_level
        implementation: dnd2024:rules_create_your_character_write_your_level
        parameters:
          default_starting_level: 1
          level_1_xp: 0
          higher_start_level_authority: gm
          subclass_required_at_start_level_minimum: 3
    contract:
      id: dnd2024:rules_create_your_character_write_your_level
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_create_your_character_write_your_level
    citations:
    - SRD 5.2.1 pp. 19; builtin/srd-5.2.1/source.txt; source section Create Your Character; draft rules/create_your_character_write_your_level.md
  dnd2024:rules/create_your_character_choose_a_background:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_choose_a_background
        implementation: dnd2024:rules_create_your_character_choose_a_background
        parameters:
          background_choice:
            count: 1
            options:
            - key: dnd2024:backgrounds/acolyte
            - key: dnd2024:backgrounds/soldier
            additional_options_authority: gm
          grants:
            feat: background_feat
            skill_proficiencies: 2
            tool_proficiencies: 1
          class_proficiencies_also_granted: true
          level_1_proficiency_bonus: 2
          ability_background_examples:
            strength: soldier
            dexterity: soldier
            constitution: soldier
            intelligence: acolyte
            wisdom: acolyte
            charisma: acolyte
    contract:
      id: dnd2024:rules_create_your_character_choose_a_background
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_create_your_character_choose_a_background
    citations:
    - SRD 5.2.1 pp. 19, 20; builtin/srd-5.2.1/source.txt; source section Create Your Character; draft rules/create_your_character_choose_a_background.md
  dnd2024:rules/create_your_character_imagine_your_past_and_present:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 20; builtin/srd-5.2.1/source.txt; source section Create Your Character; draft rules/create_your_character_imagine_your_past_and_present.md
  dnd2024:rules/create_your_character_generate_your_scores:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_generate_your_scores
        implementation: dnd2024:rules_create_your_character_generate_your_scores
        parameters:
          method_choice:
            authority: gm_or_player
            options:
            - standard_array
            - random_generation
            - point_cost
          standard_array:
          - 15
          - 14
          - 13
          - 12
          - 10
          - 8
          random_generation:
            repeat: 6
            roll:
              count: 4
              sides: 6
              add: 0
            keep_highest: 3
            aggregate: sum
          point_cost:
            budget: 27
            score_costs:
              8: 0
              9: 1
              10: 2
              11: 3
              12: 4
              13: 5
              14: 7
              15: 9
            score_count: 6
    contract:
      id: dnd2024:rules_create_your_character_generate_your_scores
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_create_your_character_generate_your_scores
    citations:
    - SRD 5.2.1 pp. 21; builtin/srd-5.2.1/source.txt; source section Create Your Character; draft rules/create_your_character_generate_your_scores.md
  dnd2024:rules/create_your_character_determine_ability_modifiers:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/create_your_character_determine_ability_modifiers
        implementation: dnd2024:rules_create_your_character_determine_ability_modifiers
        parameters:
          modifier:
            op: floor
            value:
              op: divide
              left:
                op: subtract
                left: ability_score
                right: 10
              right: 2
          source_table_score_range:
            minimum: 3
            maximum: 20
    contract:
      id: dnd2024:rules_create_your_character_determine_ability_modifiers
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_create_your_character_determine_ability_modifiers
    citations:
    - SRD 5.2.1 pp. 21; builtin/srd-5.2.1/source.txt; source section Create Your Character; draft rules/create_your_character_determine_ability_modifiers.md
  dnd2024:rules/create_your_character_step_5_character_creation_details:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 22; builtin/srd-5.2.1/source.txt; source section Create Your Character; draft rules/create_your_character_step_5_character_creation_details.md
  dnd2024:rules/level_advancement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/level_advancement
        implementation: dnd2024:rules_level_advancement
        parameters:
          advancement:
          - level: 1
            minimum_xp: 0
            proficiency_bonus: 2
          - level: 2
            minimum_xp: 300
            proficiency_bonus: 2
          - level: 3
            minimum_xp: 900
            proficiency_bonus: 2
          - level: 4
            minimum_xp: 2700
            proficiency_bonus: 2
          - level: 5
            minimum_xp: 6500
            proficiency_bonus: 3
          - level: 6
            minimum_xp: 14000
            proficiency_bonus: 3
          - level: 7
            minimum_xp: 23000
            proficiency_bonus: 3
          - level: 8
            minimum_xp: 34000
            proficiency_bonus: 3
          - level: 9
            minimum_xp: 48000
            proficiency_bonus: 4
          - level: 10
            minimum_xp: 64000
            proficiency_bonus: 4
          - level: 11
            minimum_xp: 85000
            proficiency_bonus: 4
          - level: 12
            minimum_xp: 100000
            proficiency_bonus: 4
          - level: 13
            minimum_xp: 120000
            proficiency_bonus: 5
          - level: 14
            minimum_xp: 140000
            proficiency_bonus: 5
          - level: 15
            minimum_xp: 165000
            proficiency_bonus: 5
          - level: 16
            minimum_xp: 195000
            proficiency_bonus: 5
          - level: 17
            minimum_xp: 225000
            proficiency_bonus: 6
          - level: 18
            minimum_xp: 265000
            proficiency_bonus: 6
          - level: 19
            minimum_xp: 305000
            proficiency_bonus: 6
          - level: 20
            minimum_xp: 355000
            proficiency_bonus: 6
          level_selection: highest_threshold_met
    contract:
      id: dnd2024:rules_level_advancement
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_level_advancement
    citations:
    - SRD 5.2.1 pp. 23; builtin/srd-5.2.1/source.txt; source section Level Advancement; draft rules/level_advancement.md
  dnd2024:rules/level_advancement_tier_1_levels_1_4:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 24; builtin/srd-5.2.1/source.txt; source section Level Advancement; draft rules/level_advancement_tier_1_levels_1_4.md
  dnd2024:rules/level_advancement_tier_4_levels_17_20:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 24; builtin/srd-5.2.1/source.txt; source section Level Advancement; draft rules/level_advancement_tier_4_levels_17_20.md
  dnd2024:rules/starting_at_higher_levels_starting_equipment:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/starting_at_higher_levels_starting_equipment
        implementation: dnd2024:rules_starting_at_higher_levels_starting_equipment
        parameters:
          authority: gm
          table_is_guidance: true
          normal_starting_equipment_included: true
          availability_authority: gm
          starting_level_equipment:
          - levels:
            - 2
            - 4
            additional_gp: 0
            magic_items:
              common: 1
          - levels:
            - 5
            - 10
            additional_gp:
              base: 500
              roll:
                count: 1
                sides: 10
                add: 0
              multiplier: 25
            magic_items:
              common: 1
              uncommon: 1
          - levels:
            - 11
            - 16
            additional_gp:
              base: 5000
              roll:
                count: 1
                sides: 10
                add: 0
              multiplier: 250
            magic_items:
              common: 2
              uncommon: 3
              rare: 1
          - levels:
            - 17
            - 20
            additional_gp:
              base: 20000
              roll:
                count: 1
                sides: 10
                add: 0
              multiplier: 250
            magic_items:
              common: 2
              uncommon: 4
              rare: 3
              very_rare: 1
    contract:
      id: dnd2024:rules_starting_at_higher_levels_starting_equipment
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_starting_at_higher_levels_starting_equipment
    citations:
    - SRD 5.2.1 pp. 24; builtin/srd-5.2.1/source.txt; source section Starting at Higher Levels; draft rules/starting_at_higher_levels_starting_equipment.md
  dnd2024:rules/multiclassing_experience_points:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/multiclassing_experience_points
        implementation: dnd2024:rules_multiclassing_experience_points
        parameters:
          xp_threshold_level_basis:
            op: sum
            terms: class_levels
          advancement:
            key: dnd2024:rules/level_advancement
    contract:
      id: dnd2024:rules_multiclassing_experience_points
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_multiclassing_experience_points
    citations:
    - SRD 5.2.1 pp. 25; builtin/srd-5.2.1/source.txt; source section Multiclassing; draft rules/multiclassing_experience_points.md
  dnd2024:rules/multiclassing_proficiencies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/multiclassing_proficiencies
        implementation: dnd2024:rules_multiclassing_proficiencies
        parameters:
          new_class_first_level:
            grant: class_multiclass_proficiencies_only
            exclude: other_starting_proficiencies
            source: chosen_class_definition
    contract:
      id: dnd2024:rules_multiclassing_proficiencies
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_multiclassing_proficiencies
    citations:
    - SRD 5.2.1 pp. 25; builtin/srd-5.2.1/source.txt; source section Multiclassing; draft rules/multiclassing_proficiencies.md
  dnd2024:rules/multiclassing_extra_attack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/multiclassing_extra_attack
        implementation: dnd2024:rules_multiclassing_extra_attack
        parameters:
          extra_attack:
            stacks_across_classes: false
            default_attacks: 2
            higher_count_requires_explicit_feature: true
          thirsting_blade:
            stacks_with_extra_attack: false
            weapon_scope: pact_weapon
    contract:
      id: dnd2024:rules_multiclassing_extra_attack
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_multiclassing_extra_attack
    citations:
    - SRD 5.2.1 pp. 25; builtin/srd-5.2.1/source.txt; source section Multiclassing; draft rules/multiclassing_extra_attack.md
  dnd2024:rules/classes:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 28; builtin/srd-5.2.1/source.txt; source section Classes; draft rules/classes.md
  dnd2024:rules/barbarian_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/barbarian_as_a_multiclass_character
        implementation: dnd2024:rules_barbarian_as_a_multiclass_character
        parameters:
          class_definition:
            key: dnd2024:classes/barbarian
          grant_core_fields:
          - hit_point_die
          weapon_proficiencies:
          - martial
          armor_training:
          - shield
          grant_features:
            class_level: 1
            from: class_definition
          spell_slots: not_applicable
    contract:
      id: dnd2024:rules_barbarian_as_a_multiclass_character
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_barbarian_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 28; builtin/srd-5.2.1/source.txt; source section Barbarian; draft rules/barbarian_as_a_multiclass_character.md
  dnd2024:rules/bard_becoming_a_bard:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 31; builtin/srd-5.2.1/source.txt; source section Bard; draft rules/bard_becoming_a_bard.md
  dnd2024:rules/bard_bard_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/bard_bard_class_features
        implementation: dnd2024:rules_bard_bard_class_features
        parameters:
          class_definition:
            key: dnd2024:classes/bard
          grant_features:
            level_basis: class_level
            unlock_when: class_level_gte_feature_level
            from: class_progression
          feature_unlocks:
          - class_level: 10
            feature:
              key: dnd2024:class_features/bard_level_10_magical_secrets
          - class_level: 18
            feature:
              key: dnd2024:class_features/bard_level_18_superior_inspiration
          - class_level: 19
            feature:
              key: dnd2024:class_features/bard_level_19_epic_boon
          - class_level: 1
            feature:
              key: dnd2024:class_features/bard_level_1_bardic_inspiration
          - class_level: 1
            feature:
              key: dnd2024:class_features/bard_level_1_spellcasting
          - class_level: 20
            feature:
              key: dnd2024:class_features/bard_level_20_words_of_creation
          - class_level: 2
            feature:
              key: dnd2024:class_features/bard_level_2_expertise
          - class_level: 2
            feature:
              key: dnd2024:class_features/bard_level_2_jack_of_all_trades
          - class_level: 3
            feature:
              key: dnd2024:class_features/bard_level_3_bard_subclass
          - class_level: 4
            feature:
              key: dnd2024:class_features/bard_level_4_ability_score_improvement
          - class_level: 5
            feature:
              key: dnd2024:class_features/bard_level_5_font_of_inspiration
          - class_level: 7
            feature:
              key: dnd2024:class_features/bard_level_7_countercharm
    contract:
      id: dnd2024:rules_bard_bard_class_features
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_bard_bard_class_features
    citations:
    - SRD 5.2.1 pp. 31; builtin/srd-5.2.1/source.txt; source section Bard; draft rules/bard_bard_class_features.md
  dnd2024:rules/cleric_becoming_a_cleric:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 36; builtin/srd-5.2.1/source.txt; source section Cleric; draft rules/cleric_becoming_a_cleric.md
  dnd2024:rules/cleric_cleric_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/cleric_cleric_class_features
        implementation: dnd2024:rules_cleric_cleric_class_features
        parameters:
          class_definition:
            key: dnd2024:classes/cleric
          grant_features:
            level_basis: class_level
            unlock_when: class_level_gte_feature_level
            from: class_progression
          feature_unlocks:
          - class_level: 10
            feature:
              key: dnd2024:class_features/cleric_level_10_divine_intervention
          - class_level: 14
            feature:
              key: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
          - class_level: 19
            feature:
              key: dnd2024:class_features/cleric_level_19_epic_boon
          - class_level: 1
            feature:
              key: dnd2024:class_features/cleric_level_1_divine_order
          - class_level: 1
            feature:
              key: dnd2024:class_features/cleric_level_1_spellcasting
          - class_level: 20
            feature:
              key: dnd2024:class_features/cleric_level_20_greater_divine_intervention
          - class_level: 2
            feature:
              key: dnd2024:class_features/cleric_level_2_channel_divinity
          - class_level: 3
            feature:
              key: dnd2024:class_features/cleric_level_3_cleric_subclass
          - class_level: 4
            feature:
              key: dnd2024:class_features/cleric_level_4_ability_score_improvement
          - class_level: 5
            feature:
              key: dnd2024:class_features/cleric_level_5_sear_undead
          - class_level: 7
            feature:
              key: dnd2024:class_features/cleric_level_7_blessed_strikes
    contract:
      id: dnd2024:rules_cleric_cleric_class_features
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_cleric_cleric_class_features
    citations:
    - SRD 5.2.1 pp. 36; builtin/srd-5.2.1/source.txt; source section Cleric; draft rules/cleric_cleric_class_features.md
  dnd2024:rules/druid_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/druid_as_a_level_1_character
        implementation: dnd2024:rules_druid_as_a_level_1_character
        parameters:
          class_definition:
            key: dnd2024:classes/druid
          grant_core_traits: all
          grant_features:
            class_level: 1
            from: class_definition
    contract:
      id: dnd2024:rules_druid_as_a_level_1_character
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_druid_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 41; builtin/srd-5.2.1/source.txt; source section Druid; draft rules/druid_as_a_level_1_character.md
  dnd2024:rules/druid_druid_spell_list:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/druid_druid_spell_list
        implementation: dnd2024:rules_druid_druid_spell_list
        parameters:
          class_definition:
            key: dnd2024:classes/druid
          spell_list:
          - spell:
              key: dnd2024:spells/druidcraft
            level: 0
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/elementalism
            level: 0
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/guidance
            level: 0
            school: divination
            concentration: true
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
              key: dnd2024:spells/poison_spray
            level: 0
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/produce_flame
            level: 0
            school: conjuration
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
              key: dnd2024:spells/shillelagh
            level: 0
            school: transmutation
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
              key: dnd2024:spells/starry_wisp
            level: 0
            school: evocation
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
              key: dnd2024:spells/charm_person
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
              key: dnd2024:spells/entangle
            level: 1
            school: conjuration
            concentration: true
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
              key: dnd2024:spells/healing_word
            level: 1
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/ice_knife
            level: 1
            school: conjuration
            concentration: false
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
              key: dnd2024:spells/augury
            level: 2
            school: divination
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/barkskin
            level: 2
            school: transmutation
            concentration: false
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
              key: dnd2024:spells/enlarge_reduce
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
              key: dnd2024:spells/flame_blade
            level: 2
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/flaming_sphere
            level: 2
            school: evocation
            concentration: true
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
              key: dnd2024:spells/moonbeam
            level: 2
            school: evocation
            concentration: true
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
              key: dnd2024:spells/spike_growth
            level: 2
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/call_lightning
            level: 3
            school: conjuration
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
              key: dnd2024:spells/sleet_storm
            level: 3
            school: conjuration
            concentration: true
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
              key: dnd2024:spells/confusion
            level: 4
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/conjure_minor_elementals
            level: 4
            school: conjuration
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
              key: dnd2024:spells/control_water
            level: 4
            school: transmutation
            concentration: true
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
              key: dnd2024:spells/dominate_beast
            level: 4
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/fire_shield
            level: 4
            school: evocation
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
              key: dnd2024:spells/giant_insect
            level: 4
            school: conjuration
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
              key: dnd2024:spells/ice_storm
            level: 4
            school: evocation
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
              key: dnd2024:spells/polymorph
            level: 4
            school: transmutation
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
              key: dnd2024:spells/stoneskin
            level: 4
            school: transmutation
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/wall_of_fire
            level: 4
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/antilife_shell
            level: 5
            school: abjuration
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
              key: dnd2024:spells/commune_with_nature
            level: 5
            school: divination
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/cone_of_cold
            level: 5
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/conjure_elemental
            level: 5
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/contagion
            level: 5
            school: necromancy
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
              key: dnd2024:spells/insect_plague
            level: 5
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
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
              key: dnd2024:spells/reincarnate
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
              key: dnd2024:spells/tree_stride
            level: 5
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/wall_of_stone
            level: 5
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/conjure_fey
            level: 6
            school: conjuration
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
              key: dnd2024:spells/flesh_to_stone
            level: 6
            school: transmutation
            concentration: true
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
              key: dnd2024:spells/move_earth
            level: 6
            school: transmutation
            concentration: true
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
              key: dnd2024:spells/transport_via_plants
            level: 6
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/wall_of_thorns
            level: 6
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/wind_walk
            level: 6
            school: transmutation
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
              key: dnd2024:spells/mirage_arcane
            level: 7
            school: illusion
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
              key: dnd2024:spells/reverse_gravity
            level: 7
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/symbol
            level: 7
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/animal_shapes
            level: 8
            school: transmutation
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
              key: dnd2024:spells/incendiary_cloud
            level: 8
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/sunburst
            level: 8
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/tsunami
            level: 8
            school: conjuration
            concentration: true
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
              key: dnd2024:spells/shapechange
            level: 9
            school: transmutation
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/storm_of_vengeance
            level: 9
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/true_resurrection
            level: 9
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
    contract:
      id: dnd2024:rules_druid_druid_spell_list
      inputs:
        class: definition
      outputs:
      - spell_options
      events: []
      capabilities:
      - spell_lists
    requires:
    - dnd2024:rules_druid_druid_spell_list
    citations:
    - SRD 5.2.1 pp. 44, 45; builtin/srd-5.2.1/source.txt; source section Druid; draft rules/druid_druid_spell_list.md
  dnd2024:rules/fighter_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/fighter_as_a_level_1_character
        implementation: dnd2024:rules_fighter_as_a_level_1_character
        parameters:
          class_definition:
            key: dnd2024:classes/fighter
          grant_core_traits: all
          grant_features:
            class_level: 1
            from: class_definition
    contract:
      id: dnd2024:rules_fighter_as_a_level_1_character
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_fighter_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 47; builtin/srd-5.2.1/source.txt; source section Fighter; draft rules/fighter_as_a_level_1_character.md
  dnd2024:rules/monk_becoming_a_monk:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 49; builtin/srd-5.2.1/source.txt; source section Monk; draft rules/monk_becoming_a_monk.md
  dnd2024:rules/monk_monk_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/monk_monk_class_features
        implementation: dnd2024:rules_monk_monk_class_features
        parameters:
          class_definition:
            key: dnd2024:classes/monk
          grant_features:
            level_basis: class_level
            unlock_when: class_level_gte_feature_level
            from: class_progression
          feature_unlocks:
          - class_level: 10
            feature:
              key: dnd2024:class_features/monk_level_10_heightened_focus
          - class_level: 10
            feature:
              key: dnd2024:class_features/monk_level_10_self_restoration
          - class_level: 13
            feature:
              key: dnd2024:class_features/monk_level_13_deflect_energy
          - class_level: 14
            feature:
              key: dnd2024:class_features/monk_level_14_disciplined_survivor
          - class_level: 15
            feature:
              key: dnd2024:class_features/monk_level_15_perfect_focus
          - class_level: 18
            feature:
              key: dnd2024:class_features/monk_level_18_superior_defense
          - class_level: 19
            feature:
              key: dnd2024:class_features/monk_level_19_epic_boon
          - class_level: 1
            feature:
              key: dnd2024:class_features/monk_level_1_martial_arts
          - class_level: 1
            feature:
              key: dnd2024:class_features/monk_level_1_unarmored_defense
          - class_level: 20
            feature:
              key: dnd2024:class_features/monk_level_20_body_and_mind
          - class_level: 2
            feature:
              key: dnd2024:class_features/monk_level_2_monk_s_focus
          - class_level: 2
            feature:
              key: dnd2024:class_features/monk_level_2_unarmored_movement
          - class_level: 2
            feature:
              key: dnd2024:class_features/monk_level_2_uncanny_metabolism
          - class_level: 3
            feature:
              key: dnd2024:class_features/monk_level_3_deflect_attacks
          - class_level: 3
            feature:
              key: dnd2024:class_features/monk_level_3_monk_subclass
          - class_level: 4
            feature:
              key: dnd2024:class_features/monk_level_4_ability_score_improvement
          - class_level: 4
            feature:
              key: dnd2024:class_features/monk_level_4_slow_fall
          - class_level: 5
            feature:
              key: dnd2024:class_features/monk_level_5_extra_attack
          - class_level: 5
            feature:
              key: dnd2024:class_features/monk_level_5_stunning_strike
          - class_level: 6
            feature:
              key: dnd2024:class_features/monk_level_6_empowered_strikes
          - class_level: 7
            feature:
              key: dnd2024:class_features/monk_level_7_evasion
          - class_level: 9
            feature:
              key: dnd2024:class_features/monk_level_9_acrobatic_movement
          recurring_feature_levels:
            ability_score_improvement:
            - 4
            - 8
            - 12
            - 16
            subclass:
            - 3
            - 6
            - 11
            - 17
    contract:
      id: dnd2024:rules_monk_monk_class_features
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_monk_monk_class_features
    citations:
    - SRD 5.2.1 pp. 49, 50; builtin/srd-5.2.1/source.txt; source section Monk; draft rules/monk_monk_class_features.md
  dnd2024:rules/paladin_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/paladin_as_a_multiclass_character
        implementation: dnd2024:rules_paladin_as_a_multiclass_character
        parameters:
          class_definition:
            key: dnd2024:classes/paladin
          grant_core_fields:
          - hit_point_die
          weapon_proficiencies:
          - martial
          armor_training:
          - light
          - medium
          - shield
          grant_features:
            class_level: 1
            from: class_definition
          spell_slots: multiclass_spellcasting_rules
    contract:
      id: dnd2024:rules_paladin_as_a_multiclass_character
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_paladin_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 53; builtin/srd-5.2.1/source.txt; source section Paladin; draft rules/paladin_as_a_multiclass_character.md
  dnd2024:rules/ranger_becoming_a_ranger:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 57; builtin/srd-5.2.1/source.txt; source section Ranger; draft rules/ranger_becoming_a_ranger.md
  dnd2024:rules/ranger_ranger_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/ranger_ranger_class_features
        implementation: dnd2024:rules_ranger_ranger_class_features
        parameters:
          class_definition:
            key: dnd2024:classes/ranger
          grant_features:
            level_basis: class_level
            unlock_when: class_level_gte_feature_level
            from: class_progression
          feature_unlocks:
          - class_level: 10
            feature:
              key: dnd2024:class_features/ranger_level_10_tireless
          - class_level: 13
            feature:
              key: dnd2024:class_features/ranger_level_13_relentless_hunter
          - class_level: 14
            feature:
              key: dnd2024:class_features/ranger_level_14_nature_s_veil
          - class_level: 17
            feature:
              key: dnd2024:class_features/ranger_level_17_precise_hunter
          - class_level: 18
            feature:
              key: dnd2024:class_features/ranger_level_18_feral_senses
          - class_level: 19
            feature:
              key: dnd2024:class_features/ranger_level_19_epic_boon
          - class_level: 1
            feature:
              key: dnd2024:class_features/ranger_level_1_favored_enemy
          - class_level: 1
            feature:
              key: dnd2024:class_features/ranger_level_1_spellcasting
          - class_level: 1
            feature:
              key: dnd2024:class_features/ranger_level_1_weapon_mastery
          - class_level: 20
            feature:
              key: dnd2024:class_features/ranger_level_20_foe_slayer
          - class_level: 2
            feature:
              key: dnd2024:class_features/ranger_level_2_deft_explorer
          - class_level: 2
            feature:
              key: dnd2024:class_features/ranger_level_2_fighting_style
          - class_level: 3
            feature:
              key: dnd2024:class_features/ranger_level_3_ranger_subclass
          - class_level: 4
            feature:
              key: dnd2024:class_features/ranger_level_4_ability_score_improvement
          - class_level: 5
            feature:
              key: dnd2024:class_features/ranger_level_5_extra_attack
          - class_level: 6
            feature:
              key: dnd2024:class_features/ranger_level_6_roving
          - class_level: 9
            feature:
              key: dnd2024:class_features/ranger_level_9_expertise
    contract:
      id: dnd2024:rules_ranger_ranger_class_features
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_ranger_ranger_class_features
    citations:
    - SRD 5.2.1 pp. 57; builtin/srd-5.2.1/source.txt; source section Ranger; draft rules/ranger_ranger_class_features.md
  dnd2024:rules/rogue_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/rogue_as_a_level_1_character
        implementation: dnd2024:rules_rogue_as_a_level_1_character
        parameters:
          class_definition:
            key: dnd2024:classes/rogue
          grant_core_traits: all
          grant_features:
            class_level: 1
            from: class_definition
    contract:
      id: dnd2024:rules_rogue_as_a_level_1_character
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_rogue_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 61; builtin/srd-5.2.1/source.txt; source section Rogue; draft rules/rogue_as_a_level_1_character.md
  dnd2024:rules/sorcerer_becoming_a_sorcerer:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 64; builtin/srd-5.2.1/source.txt; source section Sorcerer; draft rules/sorcerer_becoming_a_sorcerer.md
  dnd2024:rules/sorcerer_sorcerer_class_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_sorcerer_class_features
        implementation: dnd2024:rules_sorcerer_sorcerer_class_features
        parameters:
          class_definition:
            key: dnd2024:classes/sorcerer
          grant_features:
            level_basis: class_level
            unlock_when: class_level_gte_feature_level
            from: class_progression
          feature_unlocks:
          - class_level: 19
            feature:
              key: dnd2024:class_features/sorcerer_level_19_epic_boon
          - class_level: 1
            feature:
              key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
          - class_level: 1
            feature:
              key: dnd2024:class_features/sorcerer_level_1_spellcasting
          - class_level: 20
            feature:
              key: dnd2024:class_features/sorcerer_level_20_arcane_apotheosis
          - class_level: 2
            feature:
              key: dnd2024:class_features/sorcerer_level_2_font_of_magic
          - class_level: 2
            feature:
              key: dnd2024:class_features/sorcerer_level_2_metamagic
          - class_level: 3
            feature:
              key: dnd2024:class_features/sorcerer_level_3_sorcerer_subclass
          - class_level: 4
            feature:
              key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
          - class_level: 5
            feature:
              key: dnd2024:class_features/sorcerer_level_5_sorcerous_restoration
          - class_level: 7
            feature:
              key: dnd2024:class_features/sorcerer_level_7_sorcery_incarnate
    contract:
      id: dnd2024:rules_sorcerer_sorcerer_class_features
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_sorcerer_sorcerer_class_features
    citations:
    - SRD 5.2.1 pp. 64; builtin/srd-5.2.1/source.txt; source section Sorcerer; draft rules/sorcerer_sorcerer_class_features.md
  dnd2024:rules/sorcerer_distant_spell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_distant_spell
        implementation: dnd2024:rules_sorcerer_distant_spell
        parameters:
          cost:
            resource: sorcery_points
            amount: 1
          trigger: spell.cast_pending
          range_branches:
          - range_feet_minimum: 5
            range_multiplier: 2
          - range_kind: touch
            replacement_range_feet: 30
    contract:
      id: dnd2024:rules_sorcerer_distant_spell
      inputs:
        caster: participant
        spell: definition
        targets: participant_list
        cast_level: integer
        choice: choice
      outputs:
      - spell_modifications
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting
      - resources
    requires:
    - dnd2024:rules_sorcerer_distant_spell
    citations:
    - SRD 5.2.1 pp. 67; builtin/srd-5.2.1/source.txt; source section Sorcerer; draft rules/sorcerer_distant_spell.md
  dnd2024:rules/sorcerer_heightened_spell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_heightened_spell
        implementation: dnd2024:rules_sorcerer_heightened_spell
        parameters:
          cost:
            resource: sorcery_points
            amount: 2
          trigger: spell.cast_pending
          spell_requires_saving_throw: true
          target_choice:
            count: 1
            from: spell_targets
          save_disadvantage:
            scope: all_saves_against_this_casting
    contract:
      id: dnd2024:rules_sorcerer_heightened_spell
      inputs:
        caster: participant
        spell: definition
        targets: participant_list
        cast_level: integer
        choice: choice
      outputs:
      - spell_modifications
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting
      - resources
    requires:
    - dnd2024:rules_sorcerer_heightened_spell
    citations:
    - SRD 5.2.1 pp. 67; builtin/srd-5.2.1/source.txt; source section Sorcerer; draft rules/sorcerer_heightened_spell.md
  dnd2024:rules/sorcerer_subtle_spell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_subtle_spell
        implementation: dnd2024:rules_sorcerer_subtle_spell
        parameters:
          cost:
            resource: sorcery_points
            amount: 1
          trigger: spell.cast_pending
          omit_components:
          - verbal
          - somatic
          - material_without_cost_or_consumption
          retain_material_if:
            any:
            - consumed
            - specified_cost
    contract:
      id: dnd2024:rules_sorcerer_subtle_spell
      inputs:
        caster: participant
        spell: definition
        targets: participant_list
        cast_level: integer
        choice: choice
      outputs:
      - spell_modifications
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting
      - resources
    requires:
    - dnd2024:rules_sorcerer_subtle_spell
    citations:
    - SRD 5.2.1 pp. 67; builtin/srd-5.2.1/source.txt; source section Sorcerer; draft rules/sorcerer_subtle_spell.md
  dnd2024:rules/sorcerer_sorcerer_spell_list:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sorcerer_sorcerer_spell_list
        implementation: dnd2024:rules_sorcerer_sorcerer_spell_list
        parameters:
          class_definition:
            key: dnd2024:classes/sorcerer
          spell_list:
          - spell:
              key: dnd2024:spells/acid_splash
            level: 0
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/chill_touch
            level: 0
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dancing_lights
            level: 0
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/elementalism
            level: 0
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/fire_bolt
            level: 0
            school: evocation
            concentration: false
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
              key: dnd2024:spells/ray_of_frost
            level: 0
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/shocking_grasp
            level: 0
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/sorcerous_burst
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
              key: dnd2024:spells/burning_hands
            level: 1
            school: evocation
            concentration: false
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
              key: dnd2024:spells/chromatic_orb
            level: 1
            school: evocation
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/color_spray
            level: 1
            school: illusion
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
              key: dnd2024:spells/disguise_self
            level: 1
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/expeditious_retreat
            level: 1
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/false_life
            level: 1
            school: necromancy
            concentration: false
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
              key: dnd2024:spells/fog_cloud
            level: 1
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/grease
            level: 1
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/ice_knife
            level: 1
            school: conjuration
            concentration: false
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
              key: dnd2024:spells/mage_armor
            level: 1
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/magic_missile
            level: 1
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/ray_of_sickness
            level: 1
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/shield
            level: 1
            school: abjuration
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
              key: dnd2024:spells/thunderwave
            level: 1
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/alter_self
            level: 2
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/blindness_deafness
            level: 2
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/blur
            level: 2
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/darkness
            level: 2
            school: evocation
            concentration: true
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
              key: dnd2024:spells/detect_thoughts
            level: 2
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dragon_s_breath
            level: 2
            school: transmutation
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
              key: dnd2024:spells/flame_blade
            level: 2
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/flaming_sphere
            level: 2
            school: evocation
            concentration: true
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
              key: dnd2024:spells/levitate
            level: 2
            school: transmutation
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
              key: dnd2024:spells/scorching_ray
            level: 2
            school: evocation
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
              key: dnd2024:spells/web
            level: 2
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/blink
            level: 3
            school: transmutation
            concentration: false
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
              key: dnd2024:spells/counterspell
            level: 3
            school: abjuration
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
              key: dnd2024:spells/fear
            level: 3
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/fireball
            level: 3
            school: evocation
            concentration: false
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
              key: dnd2024:spells/haste
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
              key: dnd2024:spells/lightning_bolt
            level: 3
            school: evocation
            concentration: false
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
              key: dnd2024:spells/protection_from_energy
            level: 3
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/sleet_storm
            level: 3
            school: conjuration
            concentration: true
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
              key: dnd2024:spells/stinking_cloud
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
              key: dnd2024:spells/vampiric_touch
            level: 3
            school: necromancy
            concentration: true
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
              key: dnd2024:spells/dominate_beast
            level: 4
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/fire_shield
            level: 4
            school: evocation
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
              key: dnd2024:spells/ice_storm
            level: 4
            school: evocation
            concentration: false
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
              key: dnd2024:spells/stoneskin
            level: 4
            school: transmutation
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/vitriolic_sphere
            level: 4
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/wall_of_fire
            level: 4
            school: evocation
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
              key: dnd2024:spells/arcane_hand
            level: 5
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/cloudkill
            level: 5
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/cone_of_cold
            level: 5
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/creation
            level: 5
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dominate_person
            level: 5
            school: enchantment
            concentration: true
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
              key: dnd2024:spells/insect_plague
            level: 5
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/seeming
            level: 5
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/telekinesis
            level: 5
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/teleportation_circle
            level: 5
            school: conjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/wall_of_stone
            level: 5
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/chain_lightning
            level: 6
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/circle_of_death
            level: 6
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/disintegrate
            level: 6
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/eyebite
            level: 6
            school: necromancy
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/flesh_to_stone
            level: 6
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/freezing_sphere
            level: 6
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/globe_of_invulnerability
            level: 6
            school: abjuration
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
              key: dnd2024:spells/move_earth
            level: 6
            school: transmutation
            concentration: true
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
              key: dnd2024:spells/delayed_blast_fireball
            level: 7
            school: evocation
            concentration: true
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
              key: dnd2024:spells/finger_of_death
            level: 7
            school: necromancy
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
              key: dnd2024:spells/prismatic_spray
            level: 7
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/reverse_gravity
            level: 7
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/teleport
            level: 7
            school: conjuration
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
              key: dnd2024:spells/earthquake
            level: 8
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/incendiary_cloud
            level: 8
            school: conjuration
            concentration: true
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
              key: dnd2024:spells/sunburst
            level: 8
            school: evocation
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
              key: dnd2024:spells/meteor_swarm
            level: 9
            school: evocation
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
              key: dnd2024:spells/time_stop
            level: 9
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/wish
            level: 9
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
    contract:
      id: dnd2024:rules_sorcerer_sorcerer_spell_list
      inputs:
        class: definition
      outputs:
      - spell_options
      events: []
      capabilities:
      - spell_lists
    requires:
    - dnd2024:rules_sorcerer_sorcerer_spell_list
    citations:
    - SRD 5.2.1 pp. 67, 68, 69; builtin/srd-5.2.1/source.txt; source section Sorcerer; draft rules/sorcerer_sorcerer_spell_list.md
  dnd2024:rules/warlock_as_a_multiclass_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_as_a_multiclass_character
        implementation: dnd2024:rules_warlock_as_a_multiclass_character
        parameters:
          class_definition:
            key: dnd2024:classes/warlock
          grant_core_fields:
          - hit_point_die
          weapon_proficiencies: []
          armor_training:
          - light
          grant_features:
            class_level: 1
            from: class_definition
          spell_slots: multiclass_spellcasting_rules
    contract:
      id: dnd2024:rules_warlock_as_a_multiclass_character
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_warlock_as_a_multiclass_character
    citations:
    - SRD 5.2.1 pp. 70; builtin/srd-5.2.1/source.txt; source section Warlock; draft rules/warlock_as_a_multiclass_character.md
  dnd2024:rules/warlock_agonizing_blast:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_agonizing_blast
        implementation: dnd2024:rules_warlock_agonizing_blast
        parameters:
          prerequisites:
            warlock_level_minimum: 2
            known_warlock_damaging_cantrip: true
          choice:
            count: 1
            from: known_warlock_damaging_cantrips
          damage_roll_bonus: charisma_modifier
          repeatable: true
          repeat_choice_must_differ: true
    contract:
      id: dnd2024:rules_warlock_agonizing_blast
      inputs:
        actor: participant
        targets: participant_list
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - effects
      - resource_updates
      events: []
      capabilities:
      - class_features
      - effects
      - resources
    requires:
    - dnd2024:rules_warlock_agonizing_blast
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.txt; source section Warlock; draft rules/warlock_agonizing_blast.md
  dnd2024:rules/warlock_devil_s_sight:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_devil_s_sight
        implementation: dnd2024:rules_warlock_devil_s_sight
        parameters:
          prerequisites:
            warlock_level_minimum: 2
          vision:
            range_feet: 120
            normal_in:
            - dim_light
            - darkness
            magical: true
            nonmagical: true
    contract:
      id: dnd2024:rules_warlock_devil_s_sight
      inputs:
        actor: participant
        targets: participant_list
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - effects
      - resource_updates
      events: []
      capabilities:
      - class_features
      - effects
      - resources
    requires:
    - dnd2024:rules_warlock_devil_s_sight
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.txt; source section Warlock; draft rules/warlock_devil_s_sight.md
  dnd2024:rules/warlock_eldritch_smite:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_eldritch_smite
        implementation: dnd2024:rules_warlock_eldritch_smite
        parameters:
          prerequisites:
            warlock_level_minimum: 5
            invocation: pact_of_the_blade
          trigger: pact_weapon.hit_creature
          uses_per_turn: 1
          cost:
            resource: pact_magic_slot
            amount: 1
          extra_damage:
            type: force
            dice:
              count:
                op: add
                left: 1
                right: expended_slot_level
              sides: 8
          optional_prone:
            target_size_maximum: huge
    contract:
      id: dnd2024:rules_warlock_eldritch_smite
      inputs:
        actor: participant
        targets: participant_list
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - effects
      - resource_updates
      events:
      - attack.hit
      capabilities:
      - class_features
      - effects
      - resources
    requires:
    - dnd2024:rules_warlock_eldritch_smite
    citations:
    - SRD 5.2.1 pp. 72, 73; builtin/srd-5.2.1/source.txt; source section Warlock; draft rules/warlock_eldritch_smite.md
  dnd2024:rules/warlock_gaze_of_two_minds:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_gaze_of_two_minds
        implementation: dnd2024:rules_warlock_gaze_of_two_minds
        parameters:
          prerequisites:
            warlock_level_minimum: 5
          activation:
            economy: bonus_action
            range: touch
            target_willing: true
          duration:
            until: actor_next_turn_end
          maintenance:
            economy: bonus_action
            'on': subsequent_turn
            same_plane_required: true
            extend_until: actor_next_turn_end
            failure_to_maintain: end_connection
          perception:
            use_target_senses: true
            benefit_special_senses: true
          spell_origin_choice:
            options:
            - actor_space
            - target_space
            maximum_separation_feet: 60
    contract:
      id: dnd2024:rules_warlock_gaze_of_two_minds
      inputs:
        actor: participant
        targets: participant_list
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - effects
      - resource_updates
      events: []
      capabilities:
      - class_features
      - effects
      - resources
    requires:
    - dnd2024:rules_warlock_gaze_of_two_minds
    citations:
    - SRD 5.2.1 pp. 73; builtin/srd-5.2.1/source.txt; source section Warlock; draft rules/warlock_gaze_of_two_minds.md
  dnd2024:rules/warlock_investment_of_the_chain_master:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_investment_of_the_chain_master
        implementation: dnd2024:rules_warlock_investment_of_the_chain_master
        parameters:
          prerequisites:
            warlock_level_minimum: 5
            invocation: pact_of_the_chain
          trigger: find_familiar.cast
          familiar_benefits:
            movement_choice:
              options:
              - fly
              - swim
              speed_feet: 40
            command_attack:
              economy: bonus_action
              familiar_action: attack
            damage_type_choice:
              replace_types:
              - bludgeoning
              - piercing
              - slashing
              options:
              - necrotic
              - radiant
              optional: true
              timing: each_damage_instance
            saving_throw_dc: caster_spell_save_dc
            damage_resistance:
              trigger: familiar.damage_pending
              economy: reaction
              scope: triggering_damage
    contract:
      id: dnd2024:rules_warlock_investment_of_the_chain_master
      inputs:
        actor: participant
        targets: participant_list
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - effects
      - resource_updates
      events:
      - spell.cast_completed
      - damage.pending
      capabilities:
      - class_features
      - effects
      - resources
    requires:
    - dnd2024:rules_warlock_investment_of_the_chain_master
    citations:
    - SRD 5.2.1 pp. 73; builtin/srd-5.2.1/source.txt; source section Warlock; draft rules/warlock_investment_of_the_chain_master.md
  dnd2024:rules/warlock_mask_of_many_faces:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_mask_of_many_faces
        implementation: dnd2024:rules_warlock_mask_of_many_faces
        parameters:
          prerequisites:
            warlock_level_minimum: 2
          spell:
            key: dnd2024:spells/disguise_self
          spell_slot_cost: 0
    contract:
      id: dnd2024:rules_warlock_mask_of_many_faces
      inputs:
        actor: participant
        targets: participant_list
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - effects
      - resource_updates
      events: []
      capabilities:
      - class_features
      - effects
      - resources
    requires:
    - dnd2024:rules_warlock_mask_of_many_faces
    citations:
    - SRD 5.2.1 pp. 73; builtin/srd-5.2.1/source.txt; source section Warlock; draft rules/warlock_mask_of_many_faces.md
  dnd2024:rules/warlock_one_with_shadows:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_one_with_shadows
        implementation: dnd2024:rules_warlock_one_with_shadows
        parameters:
          prerequisites:
            warlock_level_minimum: 5
          activation_illumination:
          - dim_light
          - darkness
          spell:
            key: dnd2024:spells/invisibility
          target: self
          spell_slot_cost: 0
    contract:
      id: dnd2024:rules_warlock_one_with_shadows
      inputs:
        actor: participant
        targets: participant_list
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - effects
      - resource_updates
      events: []
      capabilities:
      - class_features
      - effects
      - resources
    requires:
    - dnd2024:rules_warlock_one_with_shadows
    citations:
    - SRD 5.2.1 pp. 74; builtin/srd-5.2.1/source.txt; source section Warlock; draft rules/warlock_one_with_shadows.md
  dnd2024:rules/warlock_pact_of_the_chain:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_pact_of_the_chain
        implementation: dnd2024:rules_warlock_pact_of_the_chain
        parameters:
          learn_spell:
            key: dnd2024:spells/find_familiar
          casting:
            economy: magic_action
            spell_slot_cost: 0
          familiar_forms:
            allow_normal: true
            additional:
            - key: dnd2024:creatures/imp
            - key: dnd2024:creatures/pseudodragon
            - key: dnd2024:creatures/quasit
            - key: dnd2024:creatures/skeleton
            - key: dnd2024:creatures/sphinx_of_wonder
            - key: dnd2024:creatures/sprite
            - key: dnd2024:creatures/venomous_snake
          attack_exchange:
            trigger: actor.attack_action
            forgo_attacks: 1
            familiar_attacks: 1
            familiar_economy: reaction
    contract:
      id: dnd2024:rules_warlock_pact_of_the_chain
      inputs:
        actor: participant
        targets: participant_list
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - effects
      - resource_updates
      events: []
      capabilities:
      - class_features
      - effects
      - resources
    requires:
    - dnd2024:rules_warlock_pact_of_the_chain
    citations:
    - SRD 5.2.1 pp. 74; builtin/srd-5.2.1/source.txt; source section Warlock; draft rules/warlock_pact_of_the_chain.md
  dnd2024:rules/warlock_thirsting_blade:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_thirsting_blade
        implementation: dnd2024:rules_warlock_thirsting_blade
        parameters:
          prerequisites:
            warlock_level_minimum: 5
            invocation: pact_of_the_blade
          extra_attack:
            weapon: pact_weapon
            attack_count: 2
            trigger: actor.attack_action
            own_turn_only: true
    contract:
      id: dnd2024:rules_warlock_thirsting_blade
      inputs:
        actor: participant
        targets: participant_list
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - effects
      - resource_updates
      events: []
      capabilities:
      - class_features
      - effects
      - resources
    requires:
    - dnd2024:rules_warlock_thirsting_blade
    citations:
    - SRD 5.2.1 pp. 74; builtin/srd-5.2.1/source.txt; source section Warlock; draft rules/warlock_thirsting_blade.md
  dnd2024:rules/warlock_witch_sight:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/warlock_witch_sight
        implementation: dnd2024:rules_warlock_witch_sight
        parameters:
          prerequisites:
            warlock_level_minimum: 15
          sense:
            type: truesight
            range_feet: 30
    contract:
      id: dnd2024:rules_warlock_witch_sight
      inputs:
        actor: participant
        targets: participant_list
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - effects
      - resource_updates
      events: []
      capabilities:
      - class_features
      - effects
      - resources
    requires:
    - dnd2024:rules_warlock_witch_sight
    citations:
    - SRD 5.2.1 pp. 74; builtin/srd-5.2.1/source.txt; source section Warlock; draft rules/warlock_witch_sight.md
  dnd2024:rules/wizard_as_a_level_1_character:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/wizard_as_a_level_1_character
        implementation: dnd2024:rules_wizard_as_a_level_1_character
        parameters:
          class_definition:
            key: dnd2024:classes/wizard
          grant_core_traits: all
          grant_features:
            class_level: 1
            from: class_definition
          feature_unlocks:
          - class_level: 18
            feature:
              key: dnd2024:class_features/wizard_level_18_spell_mastery
          - class_level: 19
            feature:
              key: dnd2024:class_features/wizard_level_19_epic_boon
          - class_level: 1
            feature:
              key: dnd2024:class_features/wizard_level_1_arcane_recovery
          - class_level: 1
            feature:
              key: dnd2024:class_features/wizard_level_1_ritual_adept
          - class_level: 1
            feature:
              key: dnd2024:class_features/wizard_level_1_spellcasting
          - class_level: 20
            feature:
              key: dnd2024:class_features/wizard_level_20_signature_spells
          - class_level: 2
            feature:
              key: dnd2024:class_features/wizard_level_2_scholar
          - class_level: 3
            feature:
              key: dnd2024:class_features/wizard_level_3_wizard_subclass
          - class_level: 4
            feature:
              key: dnd2024:class_features/wizard_level_4_ability_score_improvement
          - class_level: 5
            feature:
              key: dnd2024:class_features/wizard_level_5_memorize_spell
          recurring_feature_levels:
            ability_score_improvement:
            - 4
            - 8
            - 12
            - 16
            subclass:
            - 3
            - 6
            - 10
            - 14
    contract:
      id: dnd2024:rules_wizard_as_a_level_1_character
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_wizard_as_a_level_1_character
    citations:
    - SRD 5.2.1 pp. 77; builtin/srd-5.2.1/source.txt; source section Wizard; draft rules/wizard_as_a_level_1_character.md
  dnd2024:rules/wizard_wizard_spell_list:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/wizard_wizard_spell_list
        implementation: dnd2024:rules_wizard_wizard_spell_list
        parameters:
          class_definition:
            key: dnd2024:classes/wizard
          spell_list:
          - spell:
              key: dnd2024:spells/acid_splash
            level: 0
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/chill_touch
            level: 0
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dancing_lights
            level: 0
            school: illusion
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/elementalism
            level: 0
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/fire_bolt
            level: 0
            school: evocation
            concentration: false
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
              key: dnd2024:spells/ray_of_frost
            level: 0
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/shocking_grasp
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
              key: dnd2024:spells/alarm
            level: 1
            school: abjuration
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/burning_hands
            level: 1
            school: evocation
            concentration: false
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
              key: dnd2024:spells/chromatic_orb
            level: 1
            school: evocation
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/color_spray
            level: 1
            school: illusion
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
              key: dnd2024:spells/disguise_self
            level: 1
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/expeditious_retreat
            level: 1
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/false_life
            level: 1
            school: necromancy
            concentration: false
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
              key: dnd2024:spells/find_familiar
            level: 1
            school: conjuration
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/floating_disk
            level: 1
            school: conjuration
            concentration: false
            ritual: true
            specific_material: false
          - spell:
              key: dnd2024:spells/fog_cloud
            level: 1
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/grease
            level: 1
            school: conjuration
            concentration: false
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
              key: dnd2024:spells/ice_knife
            level: 1
            school: conjuration
            concentration: false
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
              key: dnd2024:spells/mage_armor
            level: 1
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/magic_missile
            level: 1
            school: evocation
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
              key: dnd2024:spells/ray_of_sickness
            level: 1
            school: necromancy
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/shield
            level: 1
            school: abjuration
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
              key: dnd2024:spells/acid_arrow
            level: 2
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/alter_self
            level: 2
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/arcane_lock
            level: 2
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/arcanist_s_magic_aura
            level: 2
            school: illusion
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
              key: dnd2024:spells/blur
            level: 2
            school: illusion
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
              key: dnd2024:spells/darkness
            level: 2
            school: evocation
            concentration: true
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
              key: dnd2024:spells/detect_thoughts
            level: 2
            school: divination
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/dragon_s_breath
            level: 2
            school: transmutation
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
              key: dnd2024:spells/flaming_sphere
            level: 2
            school: evocation
            concentration: true
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
              key: dnd2024:spells/gust_of_wind
            level: 2
            school: evocation
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
              key: dnd2024:spells/levitate
            level: 2
            school: transmutation
            concentration: true
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
              key: dnd2024:spells/magic_mouth
            level: 2
            school: illusion
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/magic_weapon
            level: 2
            school: transmutation
            concentration: false
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
              key: dnd2024:spells/rope_trick
            level: 2
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/scorching_ray
            level: 2
            school: evocation
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
              key: dnd2024:spells/web
            level: 2
            school: conjuration
            concentration: true
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
              key: dnd2024:spells/bestow_curse
            level: 3
            school: necromancy
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/blink
            level: 3
            school: transmutation
            concentration: false
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
              key: dnd2024:spells/fireball
            level: 3
            school: evocation
            concentration: false
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
              key: dnd2024:spells/glyph_of_warding
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/haste
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
              key: dnd2024:spells/lightning_bolt
            level: 3
            school: evocation
            concentration: false
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
              key: dnd2024:spells/nondetection
            level: 3
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/phantom_steed
            level: 3
            school: illusion
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
              key: dnd2024:spells/sending
            level: 3
            school: divination
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/sleet_storm
            level: 3
            school: conjuration
            concentration: true
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
              key: dnd2024:spells/vampiric_touch
            level: 3
            school: necromancy
            concentration: true
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
              key: dnd2024:spells/arcane_eye
            level: 4
            school: divination
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
              key: dnd2024:spells/black_tentacles
            level: 4
            school: conjuration
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
              key: dnd2024:spells/confusion
            level: 4
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/conjure_minor_elementals
            level: 4
            school: conjuration
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
              key: dnd2024:spells/dimension_door
            level: 4
            school: conjuration
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
              key: dnd2024:spells/fabricate
            level: 4
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/faithful_hound
            level: 4
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/fire_shield
            level: 4
            school: evocation
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
              key: dnd2024:spells/ice_storm
            level: 4
            school: evocation
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
              key: dnd2024:spells/private_sanctum
            level: 4
            school: abjuration
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/resilient_sphere
            level: 4
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/secret_chest
            level: 4
            school: conjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/stone_shape
            level: 4
            school: transmutation
            concentration: false
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
              key: dnd2024:spells/vitriolic_sphere
            level: 4
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/wall_of_fire
            level: 4
            school: evocation
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
              key: dnd2024:spells/arcane_hand
            level: 5
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/cloudkill
            level: 5
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/cone_of_cold
            level: 5
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/conjure_elemental
            level: 5
            school: conjuration
            concentration: true
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
              key: dnd2024:spells/creation
            level: 5
            school: illusion
            concentration: false
            ritual: false
            specific_material: false
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
              key: dnd2024:spells/passwall
            level: 5
            school: transmutation
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
              key: dnd2024:spells/summon_dragon
            level: 5
            school: conjuration
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/telekinesis
            level: 5
            school: transmutation
            concentration: true
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
              key: dnd2024:spells/wall_of_force
            level: 5
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/wall_of_stone
            level: 5
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/chain_lightning
            level: 6
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/circle_of_death
            level: 6
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/contingency
            level: 6
            school: abjuration
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
              key: dnd2024:spells/disintegrate
            level: 6
            school: transmutation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/eyebite
            level: 6
            school: necromancy
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/flesh_to_stone
            level: 6
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/freezing_sphere
            level: 6
            school: evocation
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/globe_of_invulnerability
            level: 6
            school: abjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/guards_and_wards
            level: 6
            school: abjuration
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/instant_summons
            level: 6
            school: conjuration
            concentration: false
            ritual: true
            specific_material: true
          - spell:
              key: dnd2024:spells/irresistible_dance
            level: 6
            school: enchantment
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/magic_jar
            level: 6
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/mass_suggestion
            level: 6
            school: enchantment
            concentration: false
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/move_earth
            level: 6
            school: transmutation
            concentration: true
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
              key: dnd2024:spells/wall_of_ice
            level: 6
            school: evocation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/arcane_sword
            level: 7
            school: evocation
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/delayed_blast_fireball
            level: 7
            school: evocation
            concentration: true
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
              key: dnd2024:spells/plane_shift
            level: 7
            school: conjuration
            concentration: false
            ritual: false
            specific_material: true
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
              key: dnd2024:spells/reverse_gravity
            level: 7
            school: transmutation
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/sequester
            level: 7
            school: transmutation
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/simulacrum
            level: 7
            school: illusion
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
              key: dnd2024:spells/antimagic_field
            level: 8
            school: abjuration
            concentration: true
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
              key: dnd2024:spells/clone
            level: 8
            school: necromancy
            concentration: false
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/control_weather
            level: 8
            school: transmutation
            concentration: true
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
              key: dnd2024:spells/incendiary_cloud
            level: 8
            school: conjuration
            concentration: true
            ritual: false
            specific_material: false
          - spell:
              key: dnd2024:spells/maze
            level: 8
            school: conjuration
            concentration: true
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
              key: dnd2024:spells/meteor_swarm
            level: 9
            school: evocation
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
              key: dnd2024:spells/shapechange
            level: 9
            school: transmutation
            concentration: true
            ritual: false
            specific_material: true
          - spell:
              key: dnd2024:spells/time_stop
            level: 9
            school: transmutation
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
          - spell:
              key: dnd2024:spells/wish
            level: 9
            school: conjuration
            concentration: false
            ritual: false
            specific_material: false
    contract:
      id: dnd2024:rules_wizard_wizard_spell_list
      inputs:
        class: definition
      outputs:
      - spell_options
      events: []
      capabilities:
      - spell_lists
    requires:
    - dnd2024:rules_wizard_wizard_spell_list
    citations:
    - SRD 5.2.1 pp. 79, 80, 81, 82; builtin/srd-5.2.1/source.txt; source section Wizard; draft rules/wizard_wizard_spell_list.md
  dnd2024:rules/character_backgrounds_parts_of_a_background:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.txt; source section Character Backgrounds; draft rules/character_backgrounds_parts_of_a_background.md
  dnd2024:rules/character_backgrounds_skill_proficiencies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/character_backgrounds_skill_proficiencies
        implementation: dnd2024:rules_character_backgrounds_skill_proficiencies
        parameters:
          background_skill_grants:
            count: 2
            from: background_specified_skills
    contract:
      id: dnd2024:rules_character_backgrounds_skill_proficiencies
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_character_backgrounds_skill_proficiencies
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.txt; source section Character Backgrounds; draft rules/character_backgrounds_skill_proficiencies.md
  dnd2024:rules/character_backgrounds_background_descriptions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.txt; source section Character Backgrounds; draft rules/character_backgrounds_background_descriptions.md
  dnd2024:rules/character_species_creature_type:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/character_species_creature_type
        implementation: dnd2024:rules_character_species_creature_type
        parameters:
          creature_type_source: species
          srd_species_creature_type: humanoid
    contract:
      id: dnd2024:rules_character_species_creature_type
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_character_species_creature_type
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.txt; source section Character Species; draft rules/character_species_creature_type.md
  dnd2024:rules/character_species_special_traits:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/character_species_special_traits
        implementation: dnd2024:rules_character_species_special_traits
        parameters:
          species_trait_grants:
            all: true
            from: chosen_species
            resolve_trait_choices: true
    contract:
      id: dnd2024:rules_character_species_special_traits
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_character_species_special_traits
    citations:
    - SRD 5.2.1 pp. 84; builtin/srd-5.2.1/source.txt; source section Character Species; draft rules/character_species_special_traits.md
  dnd2024:rules/feat_descriptions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 87; builtin/srd-5.2.1/source.txt; source section Feat Descriptions; draft rules/feat_descriptions.md
  dnd2024:rules/feat_descriptions_general_feats:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 87; builtin/srd-5.2.1/source.txt; source section Feat Descriptions; draft rules/feat_descriptions_general_feats.md
  dnd2024:rules/equipment:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 89; builtin/srd-5.2.1/source.txt; source section Equipment; draft rules/equipment.md
  dnd2024:rules/weapons_weapon_proficiency:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_weapon_proficiency
        implementation: dnd2024:rules_weapons_weapon_proficiency
        parameters:
          wield_requires_proficiency: false
          attack_proficiency_bonus:
            requires_weapon_proficiency: true
          monster_proficiency:
            all_stat_block_weapons: true
    contract:
      id: dnd2024:rules_weapons_weapon_proficiency
      inputs:
        actor: participant
        test: resolution
      outputs:
      - test_modifiers
      - test_result
      events: []
      capabilities:
      - d20_tests
      - proficiency
    requires:
    - dnd2024:rules_weapons_weapon_proficiency
    citations:
    - SRD 5.2.1 pp. 89; builtin/srd-5.2.1/source.txt; source section Weapons; draft rules/weapons_weapon_proficiency.md
  dnd2024:rules/weapons_finesse:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_finesse
        implementation: dnd2024:rules_weapons_finesse
        parameters:
          ability_choice:
          - strength
          - dexterity
          same_ability_for_attack_and_damage: true
    contract:
      id: dnd2024:rules_weapons_finesse
      inputs:
        actor: participant
        test: resolution
      outputs:
      - test_modifiers
      - test_result
      events: []
      capabilities:
      - d20_tests
      - proficiency
    requires:
    - dnd2024:rules_weapons_finesse
    citations:
    - SRD 5.2.1 pp. 89; builtin/srd-5.2.1/source.txt; source section Weapons; draft rules/weapons_finesse.md
  dnd2024:rules/weapons_loading:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_loading
        implementation: dnd2024:rules_weapons_loading
        parameters:
          ammunition_fired_limit: 1
          limit_scope: each_action_bonus_action_or_reaction
          extra_attacks_override_limit: false
    contract:
      id: dnd2024:rules_weapons_loading
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_weapons_loading
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; source section Weapons; draft rules/weapons_loading.md
  dnd2024:rules/weapons_thrown:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_thrown
        implementation: dnd2024:rules_weapons_thrown
        parameters:
          ranged_attack_allowed: true
          draw_as_part_of_attack: true
          melee_thrown_ability_modifier: same_as_melee_attack_for_attack_and_damage
    contract:
      id: dnd2024:rules_weapons_thrown
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_weapons_thrown
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; source section Weapons; draft rules/weapons_thrown.md
  dnd2024:rules/weapons_mastery_properties:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_mastery_properties
        implementation: dnd2024:rules_weapons_mastery_properties
        parameters:
          mastery_requires_unlocking_feature: true
          improvised_weapon_rules:
            key: dnd2024:glossary/improvised_weapons
          improvised_cases:
          - makeshift_object_weapon
          - weapon_used_unusually
    contract:
      id: dnd2024:rules_weapons_mastery_properties
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_weapons_mastery_properties
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; source section Weapons; draft rules/weapons_mastery_properties.md
  dnd2024:rules/weapons_nick:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_nick
        implementation: dnd2024:rules_weapons_nick
        parameters:
          extra_light_attack_economy_choice:
          - part_of_attack_action
          - bonus_action
          extra_light_attack_per_turn_limit: 1
    contract:
      id: dnd2024:rules_weapons_nick
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_weapons_nick
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; source section Weapons; draft rules/weapons_nick.md
  dnd2024:rules/weapons_slow:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/weapons_slow
        implementation: dnd2024:rules_weapons_slow
        parameters:
          trigger:
            hit_creature: true
            damage_dealt: true
          optional: true
          speed_reduction_feet: 10
          duration:
            until: attacker_next_turn_start
          maximum_combined_slow_reduction_feet: 10
    contract:
      id: dnd2024:rules_weapons_slow
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events:
      - attack.hit
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_weapons_slow
    citations:
    - SRD 5.2.1 pp. 90; builtin/srd-5.2.1/source.txt; source section Weapons; draft rules/weapons_slow.md
  dnd2024:rules/armor:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/armor
        implementation: dnd2024:rules_armor
        parameters:
          armor:
          - id: padded_armor
            category: light
            base_ac: 11
            dexterity_cap: null
            add_dexterity: true
            strength_minimum: null
            stealth_disadvantage: true
            weight_lb: 8
            cost_gp: 5
          - id: leather_armor
            category: light
            base_ac: 11
            dexterity_cap: null
            add_dexterity: true
            strength_minimum: null
            stealth_disadvantage: false
            weight_lb: 10
            cost_gp: 10
          - id: studded_leather_armor
            category: light
            base_ac: 12
            dexterity_cap: null
            add_dexterity: true
            strength_minimum: null
            stealth_disadvantage: false
            weight_lb: 13
            cost_gp: 45
          - id: hide_armor
            category: medium
            base_ac: 12
            dexterity_cap: 2
            add_dexterity: true
            strength_minimum: null
            stealth_disadvantage: false
            weight_lb: 12
            cost_gp: 10
          - id: chain_shirt
            category: medium
            base_ac: 13
            dexterity_cap: 2
            add_dexterity: true
            strength_minimum: null
            stealth_disadvantage: false
            weight_lb: 20
            cost_gp: 50
          - id: scale_mail
            category: medium
            base_ac: 14
            dexterity_cap: 2
            add_dexterity: true
            strength_minimum: null
            stealth_disadvantage: true
            weight_lb: 45
            cost_gp: 50
          - id: breastplate
            category: medium
            base_ac: 14
            dexterity_cap: 2
            add_dexterity: true
            strength_minimum: null
            stealth_disadvantage: false
            weight_lb: 20
            cost_gp: 400
          - id: half_plate_armor
            category: medium
            base_ac: 15
            dexterity_cap: 2
            add_dexterity: true
            strength_minimum: null
            stealth_disadvantage: true
            weight_lb: 40
            cost_gp: 750
          - id: ring_mail
            category: heavy
            base_ac: 14
            dexterity_cap: 0
            add_dexterity: false
            strength_minimum: null
            stealth_disadvantage: true
            weight_lb: 40
            cost_gp: 30
          - id: chain_mail
            category: heavy
            base_ac: 16
            dexterity_cap: 0
            add_dexterity: false
            strength_minimum: 13
            stealth_disadvantage: true
            weight_lb: 55
            cost_gp: 75
          - id: splint_armor
            category: heavy
            base_ac: 17
            dexterity_cap: 0
            add_dexterity: false
            strength_minimum: 15
            stealth_disadvantage: true
            weight_lb: 60
            cost_gp: 200
          - id: plate_armor
            category: heavy
            base_ac: 18
            dexterity_cap: 0
            add_dexterity: false
            strength_minimum: 15
            stealth_disadvantage: true
            weight_lb: 65
            cost_gp: 1500
          don_doff_minutes:
            light:
              don: 1
              doff: 1
            medium:
              don: 5
              doff: 1
            heavy:
              don: 10
              doff: 5
          insufficient_strength_speed_penalty_feet: 10
          shield:
            ac_bonus: 2
            don_action: utilize
            doff_action: utilize
            weight_lb: 6
            cost_gp: 10
    contract:
      id: dnd2024:rules_armor
      inputs:
        actor: participant
        item: item_instance
        choice: choice
      outputs:
      - item_attributes
      - actor_modifiers
      events: []
      capabilities:
      - equipment
      - armor_class
      - crafting
    requires:
    - dnd2024:rules_armor
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt; source section Armor; draft rules/armor.md
  dnd2024:rules/armor_shield:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/armor_shield
        implementation: dnd2024:rules_armor_shield
        parameters:
          shield_ac_benefit_requires_training: true
    contract:
      id: dnd2024:rules_armor_shield
      inputs:
        actor: participant
        item: item_instance
        choice: choice
      outputs:
      - item_attributes
      - actor_modifiers
      events: []
      capabilities:
      - equipment
      - armor_class
      - crafting
    requires:
    - dnd2024:rules_armor_shield
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt; source section Armor; draft rules/armor_shield.md
  dnd2024:rules/tools_tool_proficiency:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/tools_tool_proficiency
        implementation: dnd2024:rules_tools_tool_proficiency
        parameters:
          tool_check_proficiency_bonus: true
          also_proficient_in_used_skill: advantage
          monster_proficiency: all_stat_block_tools
    contract:
      id: dnd2024:rules_tools_tool_proficiency
      inputs:
        actor: participant
        test: resolution
      outputs:
      - test_modifiers
      - test_result
      events: []
      capabilities:
      - d20_tests
      - proficiency
    requires:
    - dnd2024:rules_tools_tool_proficiency
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.txt; source section Tools; draft rules/tools_tool_proficiency.md
  dnd2024:rules/adventuring_gear:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.txt; source section Adventuring Gear; draft rules/adventuring_gear.md
  dnd2024:rules/mounts_and_vehicles_barding:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/mounts_and_vehicles_barding
        implementation: dnd2024:rules_mounts_and_vehicles_barding
        parameters:
          barding:
            armor_options: all_armor_table_armor
            cost_multiplier: 4
            weight_multiplier: 2
    contract:
      id: dnd2024:rules_mounts_and_vehicles_barding
      inputs:
        actor: participant
        item: item_instance
        choice: choice
      outputs:
      - item_attributes
      - actor_modifiers
      events: []
      capabilities:
      - equipment
      - armor_class
      - crafting
    requires:
    - dnd2024:rules_mounts_and_vehicles_barding
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.txt; source section Mounts and Vehicles; draft rules/mounts_and_vehicles_barding.md
  dnd2024:rules/mounts_and_vehicles_speed:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/mounts_and_vehicles_speed
        implementation: dnd2024:rules_mounts_and_vehicles_speed
        parameters:
          vehicles: &id001
          - id: airship
            speed_mph: 8
            crew: 10
            passengers: 20
            cargo_tons: 1
            ac: 13
            hp: 300
            damage_threshold: null
            cost_gp: 40000
          - id: galley
            speed_mph: 4
            crew: 80
            passengers: 0
            cargo_tons: 150
            ac: 15
            hp: 500
            damage_threshold: 20
            cost_gp: 30000
          - id: keelboat
            speed_mph: 1
            crew: 1
            passengers: 6
            cargo_tons: 0.5
            ac: 15
            hp: 100
            damage_threshold: 10
            cost_gp: 3000
          - id: longship
            speed_mph: 3
            crew: 40
            passengers: 150
            cargo_tons: 10
            ac: 15
            hp: 300
            damage_threshold: 15
            cost_gp: 10000
          - id: rowboat
            speed_mph: 1.5
            crew: 1
            passengers: 3
            cargo_tons: 0
            ac: 11
            hp: 50
            damage_threshold: null
            cost_gp: 50
          - id: sailing_ship
            speed_mph: 2
            crew: 20
            passengers: 20
            cargo_tons: 100
            ac: 15
            hp: 300
            damage_threshold: 15
            cost_gp: 10000
          - id: warship
            speed_mph: 2.5
            crew: 60
            passengers: 60
            cargo_tons: 200
            ac: 15
            hp: 500
            damage_threshold: 20
            cost_gp: 25000
          strong_headwind_speed_multiplier: 0.5
          no_wind:
            sailing_movement: false
            rowing_required: true
          river_boats:
          - keelboat
          - rowboat
          downstream:
            add_current_speed: true
            typical_current_mph: 3
          significant_upstream_current:
            rowing_allowed: false
            shore_draft_animal_towing_allowed: true
          rowboat_weight_lb: 100
    contract:
      id: dnd2024:rules_mounts_and_vehicles_speed
      inputs:
        travelers: participant_list
        vehicle:
          type: object
          required: false
        journey: resolution
      outputs:
      - travel_rate
      - travel_limits
      events: []
      capabilities:
      - travel
      - vehicles
    requires:
    - dnd2024:rules_mounts_and_vehicles_speed
    citations:
    - SRD 5.2.1 pp. 100, 101; builtin/srd-5.2.1/source.txt; source section Mounts and Vehicles; draft rules/mounts_and_vehicles_speed.md
  dnd2024:rules/mounts_and_vehicles_damage_threshold:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/mounts_and_vehicles_damage_threshold
        implementation: dnd2024:rules_mounts_and_vehicles_damage_threshold
        parameters:
          threshold_source: vehicle_table
          threshold_rule:
            key: dnd2024:glossary/damage_threshold
          vehicle_thresholds:
            airship: null
            galley: 20
            keelboat: 10
            longship: 15
            rowboat: null
            sailing_ship: 15
            warship: 20
    contract:
      id: dnd2024:rules_mounts_and_vehicles_damage_threshold
      inputs:
        travelers: participant_list
        vehicle:
          type: object
          required: false
        journey: resolution
      outputs:
      - travel_rate
      - travel_limits
      events: []
      capabilities:
      - travel
      - vehicles
    requires:
    - dnd2024:rules_mounts_and_vehicles_damage_threshold
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; source section Mounts and Vehicles; draft rules/mounts_and_vehicles_damage_threshold.md
  dnd2024:rules/lifestyle_expenses_wretched_free:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/lifestyle_expenses_wretched_free
        implementation: dnd2024:rules_lifestyle_expenses_wretched_free
        parameters:
          lifestyle:
            id: wretched
            cost_gp_per_day: 0
          displaced_vehicle_table: *id001
    contract:
      id: dnd2024:rules_lifestyle_expenses_wretched_free
      inputs:
        actor: participant
        choice: choice
        days:
          type: integer
          required: false
      outputs:
      - cost
      - availability
      events: []
      capabilities:
      - economy
      - services
    requires:
    - dnd2024:rules_lifestyle_expenses_wretched_free
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; source section Lifestyle Expenses; draft rules/lifestyle_expenses_wretched_free.md
  dnd2024:rules/lifestyle_expenses_modest_1_gp_per_day:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/lifestyle_expenses_modest_1_gp_per_day
        implementation: dnd2024:rules_lifestyle_expenses_modest_1_gp_per_day
        parameters:
          lifestyle:
            id: modest
            cost_gp_per_day: 1
    contract:
      id: dnd2024:rules_lifestyle_expenses_modest_1_gp_per_day
      inputs:
        actor: participant
        choice: choice
        days:
          type: integer
          required: false
      outputs:
      - cost
      - availability
      events: []
      capabilities:
      - economy
      - services
    requires:
    - dnd2024:rules_lifestyle_expenses_modest_1_gp_per_day
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; source section Lifestyle Expenses; draft rules/lifestyle_expenses_modest_1_gp_per_day.md
  dnd2024:rules/lifestyle_expenses_aristocratic_10_gp_per_day:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/lifestyle_expenses_aristocratic_10_gp_per_day
        implementation: dnd2024:rules_lifestyle_expenses_aristocratic_10_gp_per_day
        parameters:
          lifestyle:
            id: aristocratic
            cost_gp_per_day: 10
    contract:
      id: dnd2024:rules_lifestyle_expenses_aristocratic_10_gp_per_day
      inputs:
        actor: participant
        choice: choice
        days:
          type: integer
          required: false
      outputs:
      - cost
      - availability
      events: []
      capabilities:
      - economy
      - services
    requires:
    - dnd2024:rules_lifestyle_expenses_aristocratic_10_gp_per_day
    citations:
    - SRD 5.2.1 pp. 101; builtin/srd-5.2.1/source.txt; source section Lifestyle Expenses; draft rules/lifestyle_expenses_aristocratic_10_gp_per_day.md
  dnd2024:rules/spellcasting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/spellcasting
        implementation: dnd2024:rules_spellcasting
        parameters:
          component_cost_added: true
          service_costs:
          - spell_levels:
            - 0
            - 0
            settlements:
            - village
            - town
            - city
            cost_gp: 30
          - spell_levels:
            - 1
            - 1
            settlements:
            - village
            - town
            - city
            cost_gp: 50
          - spell_levels:
            - 2
            - 2
            settlements:
            - village
            - town
            - city
            cost_gp: 200
          - spell_levels:
            - 3
            - 3
            settlements:
            - town
            - city
            cost_gp: 300
          - spell_levels:
            - 4
            - 5
            settlements:
            - town
            - city
            cost_gp: 2000
          - spell_levels:
            - 6
            - 8
            settlements:
            - city
            cost_gp: 20000
          - spell_levels:
            - 9
            - 9
            settlements:
            - city
            cost_gp: 100000
    contract:
      id: dnd2024:rules_spellcasting
      inputs:
        actor: participant
        choice: choice
        days:
          type: integer
          required: false
      outputs:
      - cost
      - availability
      events: []
      capabilities:
      - economy
      - services
    requires:
    - dnd2024:rules_spellcasting
    citations:
    - SRD 5.2.1 pp. 102; builtin/srd-5.2.1/source.txt; source section Spellcasting; draft rules/spellcasting.md
  dnd2024:rules/magic_items_attunement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_items_attunement
        implementation: dnd2024:rules_magic_items_attunement
        parameters:
          requires_attunement_item:
            magical_benefits_require_attunement: true
            nonmagical_benefits_require_attunement: false
            item_specific_exception_allowed: true
    contract:
      id: dnd2024:rules_magic_items_attunement
      inputs:
        actor: participant
        item: item_instance
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - item_updates
      - cost
      - effects
      events: []
      capabilities:
      - items
      - attunement
      - crafting
    requires:
    - dnd2024:rules_magic_items_attunement
    citations:
    - SRD 5.2.1 pp. 102; builtin/srd-5.2.1/source.txt; source section Magic Items; draft rules/magic_items_attunement.md
  dnd2024:rules/magic_items_ending_attunement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_items_ending_attunement
        implementation: dnd2024:rules_magic_items_ending_attunement
        parameters:
          attunement_ends_on:
          - prerequisite_lost
          - bearer_death
          - other_creature_attunes
          separation:
            distance_feet_strictly_greater_than: 100
            continuous_duration:
              value: 24
              unit: hours
          voluntary_end:
            focused_rest: short
            cursed_item_allowed: false
    contract:
      id: dnd2024:rules_magic_items_ending_attunement
      inputs:
        actor: participant
        item: item_instance
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - item_updates
      - cost
      - effects
      events:
      - attunement.completed
      - rest.short_completed
      - creature.died
      capabilities:
      - items
      - attunement
      - crafting
    requires:
    - dnd2024:rules_magic_items_ending_attunement
    citations:
    - SRD 5.2.1 pp. 102, 103; builtin/srd-5.2.1/source.txt; source section Magic Items; draft rules/magic_items_ending_attunement.md
  dnd2024:rules/magic_items_paired_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_items_paired_items
        implementation: dnd2024:rules_magic_items_paired_items
        parameters:
          paired_item_benefits_require_both_worn: true
          mixed_pairs_grant_benefits: false
    contract:
      id: dnd2024:rules_magic_items_paired_items
      inputs:
        actor: participant
        item: item_instance
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - item_updates
      - cost
      - effects
      events: []
      capabilities:
      - items
      - attunement
      - crafting
    requires:
    - dnd2024:rules_magic_items_paired_items
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; source section Magic Items; draft rules/magic_items_paired_items.md
  dnd2024:rules/crafting_nonmagical_items_raw_materials:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/crafting_nonmagical_items_raw_materials
        implementation: dnd2024:rules_crafting_nonmagical_items_raw_materials
        parameters:
          raw_material_cost:
            op: floor
            value:
              op: multiply
              left: purchase_cost
              right: 0.5
          appropriate_material_availability_authority: gm
    contract:
      id: dnd2024:rules_crafting_nonmagical_items_raw_materials
      inputs:
        actor: participant
        item: item_instance
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - item_updates
      - cost
      - effects
      events: []
      capabilities:
      - items
      - attunement
      - crafting
    requires:
    - dnd2024:rules_crafting_nonmagical_items_raw_materials
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; source section Crafting Nonmagical Items; draft rules/crafting_nonmagical_items_raw_materials.md
  dnd2024:rules/scribing_spell_scrolls:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/scribing_spell_scrolls
        implementation: dnd2024:rules_scribing_spell_scrolls
        parameters:
          creator_requires_spellcaster: true
          operation: scribe_spell_scroll
          detailed_rules_source: scribing_spell_scrolls_subsections
          detailed_rules:
          - key: dnd2024:rules/scribing_spell_scrolls_cantrips
          - key: dnd2024:rules/scribing_spell_scrolls_time_and_cost
          - key: dnd2024:rules/scribing_spell_scrolls_prerequisites_for_the_scribe
    contract:
      id: dnd2024:rules_scribing_spell_scrolls
      inputs:
        actor: participant
        item: item_instance
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - item_updates
      - cost
      - effects
      events: []
      capabilities:
      - items
      - attunement
      - crafting
    requires:
    - dnd2024:rules_scribing_spell_scrolls
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; source section Scribing Spell Scrolls; draft rules/scribing_spell_scrolls.md
  dnd2024:rules/scribing_spell_scrolls_cantrips:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/scribing_spell_scrolls_cantrips
        implementation: dnd2024:rules_scribing_spell_scrolls_cantrips
        parameters:
          cantrip_effective_caster_level: scribe_level
          scroll_costs:
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
      id: dnd2024:rules_scribing_spell_scrolls_cantrips
      inputs:
        actor: participant
        item: item_instance
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - item_updates
      - cost
      - effects
      events: []
      capabilities:
      - items
      - attunement
      - crafting
    requires:
    - dnd2024:rules_scribing_spell_scrolls_cantrips
    citations:
    - SRD 5.2.1 pp. 103; builtin/srd-5.2.1/source.txt; source section Scribing Spell Scrolls; draft rules/scribing_spell_scrolls_cantrips.md
  dnd2024:rules/gaining_spells_preparing_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/gaining_spells_preparing_spells
        implementation: dnd2024:rules_gaining_spells_preparing_spells
        parameters:
          prepared_spells_minimum_level: 1
          class_replacement_rules:
          - class: bard
            event: level_gained
            maximum_replacements: 1
          - class: cleric
            event: long_rest_finished
            maximum_replacements: any
          - class: druid
            event: long_rest_finished
            maximum_replacements: any
          - class: paladin
            event: long_rest_finished
            maximum_replacements: 1
          - class: ranger
            event: long_rest_finished
            maximum_replacements: 1
          - class: sorcerer
            event: level_gained
            maximum_replacements: 1
          - class: warlock
            event: level_gained
            maximum_replacements: 1
          - class: wizard
            event: long_rest_finished
            maximum_replacements: any
          monster_prepared_list_change:
            default: false
            gm_override: true
    contract:
      id: dnd2024:rules_gaining_spells_preparing_spells
      inputs:
        caster: participant
        spell: definition
        event: event
        choice: choice
      outputs:
      - spell_state
      - casting_requirements
      events: []
      capabilities:
      - spellcasting
      - concentration
    requires:
    - dnd2024:rules_gaining_spells_preparing_spells
    citations:
    - SRD 5.2.1 pp. 104; builtin/srd-5.2.1/source.txt; source section Gaining Spells; draft rules/gaining_spells_preparing_spells.md
  dnd2024:rules/casting_spells_spell_level:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_spell_level
        implementation: dnd2024:rules_casting_spells_spell_level
        parameters:
          spell_level_minimum: 0
          spell_level_maximum: 9
          cantrip_level: 0
          level_access_source: class_spellcasting_rules
    contract:
      id: dnd2024:rules_casting_spells_spell_level
      inputs:
        definition: definition
      outputs:
      - typed_attributes
      events: []
      capabilities:
      - definition_catalogs
    requires:
    - dnd2024:rules_casting_spells_spell_level
    citations:
    - SRD 5.2.1 pp. 104; builtin/srd-5.2.1/source.txt; source section Casting Spells; draft rules/casting_spells_spell_level.md
  dnd2024:rules/casting_spells_using_a_higher_level_spell_slot:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_using_a_higher_level_spell_slot
        implementation: dnd2024:rules_casting_spells_using_a_higher_level_spell_slot
        parameters:
          cast_level: expended_slot_level
          upcast_effects: spell_specified_only
    contract:
      id: dnd2024:rules_casting_spells_using_a_higher_level_spell_slot
      inputs:
        caster: participant
        spell: definition
        event: event
        choice: choice
      outputs:
      - spell_state
      - casting_requirements
      events: []
      capabilities:
      - spellcasting
      - concentration
    requires:
    - dnd2024:rules_casting_spells_using_a_higher_level_spell_slot
    citations:
    - SRD 5.2.1 pp. 104, 105; builtin/srd-5.2.1/source.txt; source section Casting Spells; draft rules/casting_spells_using_a_higher_level_spell_slot.md
  dnd2024:rules/casting_spells_casting_time:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_casting_time
        implementation: dnd2024:rules_casting_spells_casting_time
        parameters:
          casting_economy_source: spell_casting_time
          default_economy: magic_action
          other_casting_times:
          - bonus_action
          - reaction
          - one_minute_or_longer
    contract:
      id: dnd2024:rules_casting_spells_casting_time
      inputs:
        caster: participant
        spell: definition
        event: event
        choice: choice
      outputs:
      - spell_state
      - casting_requirements
      events: []
      capabilities:
      - spellcasting
      - concentration
    requires:
    - dnd2024:rules_casting_spells_casting_time
    citations:
    - SRD 5.2.1 pp. 105; builtin/srd-5.2.1/source.txt; source section Casting Spells; draft rules/casting_spells_casting_time.md
  dnd2024:rules/casting_spells_longer_casting_times:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_longer_casting_times
        implementation: dnd2024:rules_casting_spells_longer_casting_times
        parameters:
          casting_time_minimum_seconds: 60
          each_turn_action: magic
          requires_concentration_during_casting: true
          concentration_broken:
            spell_fails: true
            slot_expended: false
            restart_required: true
          includes_rituals: true
    contract:
      id: dnd2024:rules_casting_spells_longer_casting_times
      inputs:
        caster: participant
        spell: definition
        event: event
        choice: choice
      outputs:
      - spell_state
      - casting_requirements
      events:
      - turn.started
      - concentration.ended
      capabilities:
      - spellcasting
      - concentration
    requires:
    - dnd2024:rules_casting_spells_longer_casting_times
    citations:
    - SRD 5.2.1 pp. 105; builtin/srd-5.2.1/source.txt; source section Casting Spells; draft rules/casting_spells_longer_casting_times.md
  dnd2024:rules/casting_spells_verbal_v:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_verbal_v
        implementation: dnd2024:rules_casting_spells_verbal_v
        parameters:
          verbal:
            requires_normal_speaking_voice: true
            blocked_by:
            - gagged
            - magical_silence
    contract:
      id: dnd2024:rules_casting_spells_verbal_v
      inputs:
        caster: participant
        spell: definition
        event: event
        choice: choice
      outputs:
      - spell_state
      - casting_requirements
      events: []
      capabilities:
      - spellcasting
      - concentration
    requires:
    - dnd2024:rules_casting_spells_verbal_v
    citations:
    - SRD 5.2.1 pp. 105; builtin/srd-5.2.1/source.txt; source section Casting Spells; draft rules/casting_spells_verbal_v.md
  dnd2024:rules/casting_spells_duration:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_duration
        implementation: dnd2024:rules_casting_spells_duration
        parameters:
          duration_forms:
            concentration:
              key: dnd2024:glossary/concentration
            instantaneous:
              persists: false
            time_span:
              ends_at: elapsed_duration
              caster_dismissal:
                economy: none
                blocked_by: incapacitated
    contract:
      id: dnd2024:rules_casting_spells_duration
      inputs:
        caster: participant
        spell: definition
        event: event
        choice: choice
      outputs:
      - spell_state
      - casting_requirements
      events: []
      capabilities:
      - spellcasting
      - concentration
    requires:
    - dnd2024:rules_casting_spells_duration
    citations:
    - SRD 5.2.1 pp. 106; builtin/srd-5.2.1/source.txt; source section Casting Spells; draft rules/casting_spells_duration.md
  dnd2024:rules/casting_spells_saving_throws:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/casting_spells_saving_throws
        implementation: dnd2024:rules_casting_spells_saving_throws
        parameters:
          save_ability_and_outcome_source: spell
          dc:
            op: sum
            terms:
            - 8
            - spellcasting_ability_modifier
            - proficiency_bonus
    contract:
      id: dnd2024:rules_casting_spells_saving_throws
      inputs:
        actor: participant
        test: resolution
      outputs:
      - test_modifiers
      - test_result
      events: []
      capabilities:
      - d20_tests
      - proficiency
    requires:
    - dnd2024:rules_casting_spells_saving_throws
    citations:
    - SRD 5.2.1 pp. 106; builtin/srd-5.2.1/source.txt; source section Casting Spells; draft rules/casting_spells_saving_throws.md
  dnd2024:rules/gameplay_toolbox:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 192; builtin/srd-5.2.1/source.txt; source section Gameplay Toolbox; draft rules/gameplay_toolbox.md
  dnd2024:rules/travel_pace_slower_travelers:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/travel_pace_slower_travelers
        implementation: dnd2024:rules_travel_pace_slower_travelers
        parameters:
          group_pace: slow
          predicate:
            any_member_speed_ratio_lte: 0.5
    contract:
      id: dnd2024:rules_travel_pace_slower_travelers
      inputs:
        travelers: participant_list
        vehicle:
          type: object
          required: false
        journey: resolution
      outputs:
      - travel_rate
      - travel_limits
      events: []
      capabilities:
      - travel
      - vehicles
    requires:
    - dnd2024:rules_travel_pace_slower_travelers
    citations:
    - SRD 5.2.1 pp. 192; builtin/srd-5.2.1/source.txt; source section Travel Pace; draft rules/travel_pace_slower_travelers.md
  dnd2024:rules/travel_pace_vehicles:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/travel_pace_vehicles
        implementation: dnd2024:rules_travel_pace_vehicles
        parameters:
          travel_rate: vehicle_speed_mph
          travel_pace_choice: false
    contract:
      id: dnd2024:rules_travel_pace_vehicles
      inputs:
        travelers: participant_list
        vehicle:
          type: object
          required: false
        journey: resolution
      outputs:
      - travel_rate
      - travel_limits
      events: []
      capabilities:
      - travel
      - vehicles
    requires:
    - dnd2024:rules_travel_pace_vehicles
    citations:
    - SRD 5.2.1 pp. 192; builtin/srd-5.2.1/source.txt; source section Travel Pace; draft rules/travel_pace_vehicles.md
  dnd2024:rules/creating_a_background_2_choose_a_feat:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/creating_a_background_2_choose_a_feat
        implementation: dnd2024:rules_creating_a_background_2_choose_a_feat
        parameters:
          feat_choice:
            count: 1
            category: origin
    contract:
      id: dnd2024:rules_creating_a_background_2_choose_a_feat
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_creating_a_background_2_choose_a_feat
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; source section Creating a Background; draft rules/creating_a_background_2_choose_a_feat.md
  dnd2024:rules/creating_a_background_5_choose_equipment:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/creating_a_background_5_choose_equipment
        implementation: dnd2024:rules_creating_a_background_5_choose_equipment
        parameters:
          equipment_package:
            total_value_gp: 50
            unspent_gold_included: true
            excluded_categories:
            - martial_weapons
            - armor
    contract:
      id: dnd2024:rules_creating_a_background_5_choose_equipment
      inputs:
        actor: participant
        choice: choice
      outputs:
      - character_updates
      events: []
      capabilities:
      - character_creation
      - progression
    requires:
    - dnd2024:rules_creating_a_background_5_choose_equipment
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; source section Creating a Background; draft rules/creating_a_background_5_choose_equipment.md
  dnd2024:rules/magical_contagions_curses:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; source section Magical Contagions; draft rules/magical_contagions_curses.md
  dnd2024:rules/magical_contagions_cursed_magic_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 193; builtin/srd-5.2.1/source.txt; source section Magical Contagions; draft rules/magical_contagions_cursed_magic_items.md
  dnd2024:rules/magical_contagions_2:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 194; builtin/srd-5.2.1/source.txt; source section Magical Contagions; draft rules/magical_contagions_2.md
  dnd2024:rules/magical_contagions_cackle_fever:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magical_contagions_cackle_fever
        implementation: dnd2024:rules_magical_contagions_cackle_fever
        parameters:
          contagion: cackle_fever
          eligible_type: humanoid
          immune_species:
          - gnome
          onset_delay:
            dice:
              count: 1
              sides: 4
              add: 0
            unit: days
          onset_exhaustion:
            levels: 1
            duration: until_contagion_ends
          laughter:
            requires_exhaustion: true
            trigger: damage_taken
            excluded_damage_types:
            - psychic
            save:
              ability: constitution
              dc: 13
            failure:
              damage:
                dice:
                  count: 1
                  sides: 10
                  add: 0
                average: 5
                type: psychic
              condition: incapacitated
            repeat_save:
              event: affected_turn.ended
              success: end_laughter
            automatic_success_after:
              value: 1
              unit: minute
          recovery:
            event: long_rest.finished
            save:
              ability: constitution
              dc: 13
            success_count: 3
            consecutive_required: false
            success_threshold: end_contagion
            subsequent_immunity:
              value: 1
              unit: year
          spread:
            event: susceptible_turn.started
            area:
              shape: emanation
              radius_feet: 10
              origin: infected_creature
            save:
              ability: constitution
              dc: 10
            failure: infect
            success_immunity:
              scope: this_infected_source
              duration:
                value: 24
                unit: hours
    contract:
      id: dnd2024:rules_magical_contagions_cackle_fever
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events:
      - turn.started
      - turn.ended
      - damage.applied
      - rest.long_completed
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_magical_contagions_cackle_fever
    citations:
    - SRD 5.2.1 pp. 194; builtin/srd-5.2.1/source.txt; source section Magical Contagions; draft rules/magical_contagions_cackle_fever.md
  dnd2024:rules/environmental_effects:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 195; builtin/srd-5.2.1/source.txt; source section Environmental Effects; draft rules/environmental_effects.md
  dnd2024:rules/environmental_effects_extreme_heat:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/environmental_effects_extreme_heat
        implementation: dnd2024:rules_environmental_effects_extreme_heat
        parameters:
          temperature_f_minimum: 100
          requires_no_drinkable_water: true
          save:
            ability: constitution
            event: exposure_hour.ended
            dc:
              initial: 5
              increment_per_additional_hour: 1
            disadvantage_if_armor:
            - medium
            - heavy
            automatic_success_if_any:
            - fire_resistance
            - fire_immunity
          failure:
            exhaustion_levels: 1
    contract:
      id: dnd2024:rules_environmental_effects_extreme_heat
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events:
      - time.hour_elapsed
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_environmental_effects_extreme_heat
    citations:
    - SRD 5.2.1 pp. 195; builtin/srd-5.2.1/source.txt; source section Environmental Effects; draft rules/environmental_effects_extreme_heat.md
  dnd2024:rules/environmental_effects_high_altitude:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/environmental_effects_high_altitude
        implementation: dnd2024:rules_environmental_effects_high_altitude
        parameters:
          altitude_feet_minimum: 10000
          travel_hours_multiplier: 2
          acclimation:
            days_minimum: 30
            above_altitude_feet: 20000
            above_altitude_requires_native: true
            acclimated_exempt_from_multiplier: true
    contract:
      id: dnd2024:rules_environmental_effects_high_altitude
      inputs:
        travelers: participant_list
        vehicle:
          type: object
          required: false
        journey: resolution
      outputs:
      - travel_rate
      - travel_limits
      events: []
      capabilities:
      - travel
      - vehicles
    requires:
    - dnd2024:rules_environmental_effects_high_altitude
    citations:
    - SRD 5.2.1 pp. 195; builtin/srd-5.2.1/source.txt; source section Environmental Effects; draft rules/environmental_effects_high_altitude.md
  dnd2024:rules/environmental_effects_thin_ice:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/environmental_effects_thin_ice
        implementation: dnd2024:rules_environmental_effects_thin_ice
        parameters:
          area:
            shape: square
            side_feet: 10
          weight_tolerance_lb:
            roll:
              count: 3
              sides: 10
              add: 0
            multiplier: 10
          break_when_weight: greater_than_tolerance
          break_effect:
            creatures: all_on_broken_ice
            fall_into: frigid_water
            water_rules:
              key: dnd2024:rules/environmental_effects_frigid_water
    contract:
      id: dnd2024:rules_environmental_effects_thin_ice
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events:
      - object.load_changed
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_environmental_effects_thin_ice
    citations:
    - SRD 5.2.1 pp. 195; builtin/srd-5.2.1/source.txt; source section Environmental Effects; draft rules/environmental_effects_thin_ice.md
  dnd2024:rules/fear_and_mental_stress_mental_stress_effects:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/fear_and_mental_stress_mental_stress_effects
        implementation: dnd2024:rules_fear_and_mental_stress_mental_stress_effects
        parameters:
          damage_type: psychic
          save_ability_choice:
            default: wisdom
            alternatives:
            - intelligence
            - charisma
            authority: gm
          success_damage_choice:
            options:
            - none
            - half
            authority: gm
          example_effects:
          - id: hallucinogenic_substance
            save_dc: 10
            damage_dice:
              count: 1
              sides: 6
              add: 0
          - id: fiendish_idol
            save_dc: 15
            damage_dice:
              count: 3
              sides: 6
              add: 0
          - id: far_realm_trap
            save_dc: 20
            damage_dice:
              count: 9
              sides: 6
              add: 0
            planar_displacement:
              plane: far_realm
              until: target_next_turn_end
          examples_are_suggestions: true
    contract:
      id: dnd2024:rules_fear_and_mental_stress_mental_stress_effects
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events: []
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_fear_and_mental_stress_mental_stress_effects
    citations:
    - SRD 5.2.1 pp. 196; builtin/srd-5.2.1/source.txt; source section Fear and Mental Stress; draft rules/fear_and_mental_stress_mental_stress_effects.md
  dnd2024:rules/poison_purchasing_poison:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 197; builtin/srd-5.2.1/source.txt; source section Poison; draft rules/poison_purchasing_poison.md
  dnd2024:rules/poison_assassin_s_blood_150_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_assassin_s_blood_150_gp
        implementation: dnd2024:rules_poison_assassin_s_blood_150_gp
        parameters:
          poison:
            type: ingested
            cost_gp: 150
          save:
            ability: constitution
            dc: 10
          failure:
            damage:
              dice:
                count: 1
                sides: 12
                add: 0
              average: 6
              type: poison
            condition:
              id: poisoned
              duration:
                value: 24
                unit: hours
          success:
            damage_multiplier: 0.5
            round: down
            condition: null
    contract:
      id: dnd2024:rules_poison_assassin_s_blood_150_gp
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events: []
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_poison_assassin_s_blood_150_gp
    citations:
    - SRD 5.2.1 pp. 197; builtin/srd-5.2.1/source.txt; source section Poison; draft rules/poison_assassin_s_blood_150_gp.md
  dnd2024:rules/poison_essence_of_ether_300_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_essence_of_ether_300_gp
        implementation: dnd2024:rules_poison_essence_of_ether_300_gp
        parameters:
          poison:
            type: inhaled
            cost_gp: 300
          save:
            ability: constitution
            dc: 15
          failure:
            condition:
              id: poisoned
              duration:
                value: 8
                unit: hours
            unconscious_while_poisoned: true
          wake_triggers:
          - damage_taken
          - other_creature_action_shake_awake
          wake_ends_poisoned: false
    contract:
      id: dnd2024:rules_poison_essence_of_ether_300_gp
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events: []
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_poison_essence_of_ether_300_gp
    citations:
    - SRD 5.2.1 pp. 197; builtin/srd-5.2.1/source.txt; source section Poison; draft rules/poison_essence_of_ether_300_gp.md
  dnd2024:rules/poison_oil_of_taggit_400_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_oil_of_taggit_400_gp
        implementation: dnd2024:rules_poison_oil_of_taggit_400_gp
        parameters:
          poison:
            type: contact
            cost_gp: 400
          save:
            ability: constitution
            dc: 13
          failure:
            condition:
              id: poisoned
              duration:
                value: 24
                unit: hours
            unconscious_while_poisoned: true
          wake_triggers:
          - damage_taken
          wake_ends_poisoned: false
    contract:
      id: dnd2024:rules_poison_oil_of_taggit_400_gp
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events: []
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_poison_oil_of_taggit_400_gp
    citations:
    - SRD 5.2.1 pp. 198; builtin/srd-5.2.1/source.txt; source section Poison; draft rules/poison_oil_of_taggit_400_gp.md
  dnd2024:rules/poison_serpent_venom_200_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_serpent_venom_200_gp
        implementation: dnd2024:rules_poison_serpent_venom_200_gp
        parameters:
          poison:
            type: injury
            cost_gp: 200
          save:
            ability: constitution
            dc: 11
          failure:
            damage:
              dice:
                count: 3
                sides: 6
                add: 0
              average: 10
              type: poison
          success:
            damage_multiplier: 0.5
            round: down
    contract:
      id: dnd2024:rules_poison_serpent_venom_200_gp
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events: []
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_poison_serpent_venom_200_gp
    citations:
    - SRD 5.2.1 pp. 198; builtin/srd-5.2.1/source.txt; source section Poison; draft rules/poison_serpent_venom_200_gp.md
  dnd2024:rules/poison_truth_serum_150_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/poison_truth_serum_150_gp
        implementation: dnd2024:rules_poison_truth_serum_150_gp
        parameters:
          poison:
            type: ingested
            cost_gp: 150
          save:
            ability: constitution
            dc: 11
          failure:
            condition:
              id: poisoned
              duration:
                value: 1
                unit: hour
            while_poisoned:
              knowingly_communicate_lie: false
    contract:
      id: dnd2024:rules_poison_truth_serum_150_gp
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events: []
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_poison_truth_serum_150_gp
    citations:
    - SRD 5.2.1 pp. 198; builtin/srd-5.2.1/source.txt; source section Poison; draft rules/poison_truth_serum_150_gp.md
  dnd2024:rules/traps_parts_of_a_trap:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/traps_parts_of_a_trap
        implementation: dnd2024:rules_traps_parts_of_a_trap
        parameters:
          severity_options:
          - nuisance
          - deadly
          duration_forms:
          - rounds
          - minutes
          - hours
          - until_destroyed
          - until_dispelled
          - instantaneous
          after_activation_default: inert
          reset_requires_explicit_rule: true
    contract:
      id: dnd2024:rules_traps_parts_of_a_trap
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events: []
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_traps_parts_of_a_trap
    citations:
    - SRD 5.2.1 pp. 199; builtin/srd-5.2.1/source.txt; source section Traps; draft rules/traps_parts_of_a_trap.md
  dnd2024:rules/traps_falling_net:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/traps_falling_net
        implementation: dnd2024:rules_traps_falling_net
        parameters:
          trap:
            severity: nuisance
            levels:
            - 1
            - 4
            trigger: first_creature_crosses_tripwire
            duration: instantaneous
          geometry:
            net_square_side_feet: 10
            tripwire_height_inches: 3
            tripwire_between:
            - columns
            - trees
          save:
            ability: dexterity
            dc: 10
            automatic_success_size_minimum: huge
          failure:
            condition: restrained
            until: escape_or_net_destroyed
          escape:
            economy: action
            check:
              ability: strength
              dc: 10
              skill: athletics
            targets:
            - self
            - other_within_reach
          detection:
            economy: search
            check:
              ability: wisdom
              dc: 11
              skill: perception
            reveals:
            - tripwire
            - net
          disarm:
            after_detection:
            - cut_tripwire
            - avoid_tripwire
            check_required: false
          destroy:
            net_hp: 0
            frees_all: true
            statistics_source:
              key: dnd2024:equipment/net_1_gp
          set:
            requires:
            - thieves_tools
            - all_components
            - net
            check:
              ability: dexterity
              dc: 13
              skill: sleight_of_hand
            attempt_duration:
              value: 10
              unit: minutes
          scaling:
          - levels:
            - 1
            - 4
            save_and_escape_dc: 10
          - levels:
            - 5
            - 10
            save_and_escape_dc: 12
          - levels:
            - 11
            - 16
            save_and_escape_dc: 14
          - levels:
            - 17
            - 20
            save_and_escape_dc: 16
    contract:
      id: dnd2024:rules_traps_falling_net
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events:
      - movement.completed
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_traps_falling_net
    citations:
    - SRD 5.2.1 pp. 199; builtin/srd-5.2.1/source.txt; source section Traps; draft rules/traps_falling_net.md
  dnd2024:rules/traps_poisoned_darts:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/traps_poisoned_darts
        implementation: dnd2024:rules_traps_poisoned_darts
        parameters:
          trap:
            severity: deadly
            levels:
            - 1
            - 4
            trigger: creature_first_moves_onto_pressure_plate_on_turn
            duration: instantaneous
            maximum_activations: 3
            reset_event: next_turn.started
            reset_if_activations_less_than: 3
          targets: all_creatures_in_dart_paths
          save:
            ability: dexterity
            dc: 13
          failure:
            dart_count:
              count: 1
              sides: 3
              add: 0
            damage_per_dart:
              dice:
                count: 1
                sides: 6
                add: 0
              average: 3
              type: poison
          detection:
          - location: walls
            economy: search
            check:
              ability: wisdom
              dc: 15
              skill: perception
            reveals: dart_holes
          - location: floor_pressure_plate
            economy: search
            check:
              ability: wisdom
              dc: 15
              skill: perception
            reveals: pressure_plate
          disarm:
          - operation: plug_all_holes
            materials:
            - wax
            - cloth
            - detritus
            effect: prevents_firing
          - operation: wedge_pressure_plate
            materials:
            - iron_spike
            - similar_object
            effect: prevents_triggering
          scaling:
          - levels:
            - 1
            - 4
            poison_per_dart:
              dice:
                count: 1
                sides: 6
                add: 0
              average: 3
          - levels:
            - 5
            - 10
            poison_per_dart:
              dice:
                count: 2
                sides: 6
                add: 0
              average: 7
          - levels:
            - 11
            - 16
            poison_per_dart:
              dice:
                count: 4
                sides: 6
                add: 0
              average: 14
          - levels:
            - 17
            - 20
            poison_per_dart:
              dice:
                count: 7
                sides: 6
                add: 0
              average: 24
    contract:
      id: dnd2024:rules_traps_poisoned_darts
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events:
      - movement.completed
      - turn.started
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_traps_poisoned_darts
    citations:
    - SRD 5.2.1 pp. 200; builtin/srd-5.2.1/source.txt; source section Traps; draft rules/traps_poisoned_darts.md
  dnd2024:rules/traps_spiked_pit:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/traps_spiked_pit
        implementation: dnd2024:rules_traps_spiked_pit
        parameters:
          trap:
            severity: deadly
            levels:
            - 1
            - 4
            trigger: creature_moves_onto_lid
            duration: instantaneous
          lid:
            hinged: true
            hidden_as_floor: true
            remains_open_after_trigger: true
          trigger_effect: fall_to_pit_bottom
          depth_feet: 10
          damage:
          - type: bludgeoning
            dice:
              count: 1
              sides: 6
              add: 0
            average: 3
          - type: piercing
            dice:
              count: 2
              sides: 8
              add: 0
            average: 9
          detection:
            economy: study
            check:
              ability: intelligence
              dc: 15
              skill: investigation
            target: lid_floor_section
          disarm:
            after_detection:
            - wedge_lid_with_iron_spike_or_similar
            - hold_lid_closed_with_arcane_lock_or_similar_magic
          escape:
            requires_any:
            - climb_speed
            - climbing_gear
            - wall_climbing_magic
            gm_geometry_option: add_handholds_and_footholds
          scaling:
          - levels:
            - 1
            - 4
            depth_feet: 10
            damage:
            - type: bludgeoning
              dice:
                count: 1
                sides: 6
                add: 0
              average: 3
            - type: piercing
              dice:
                count: 2
                sides: 8
                add: 0
              average: 9
          - levels:
            - 5
            - 10
            depth_feet: 30
            damage:
            - type: bludgeoning
              dice:
                count: 3
                sides: 6
                add: 0
              average: 10
            - type: piercing
              dice:
                count: 3
                sides: 8
                add: 0
              average: 13
          - levels:
            - 11
            - 16
            depth_feet: 60
            damage:
            - type: bludgeoning
              dice:
                count: 6
                sides: 6
                add: 0
              average: 21
            - type: piercing
              dice:
                count: 8
                sides: 8
                add: 0
              average: 36
          - levels:
            - 17
            - 20
            depth_feet: 120
            damage:
            - type: bludgeoning
              dice:
                count: 12
                sides: 6
                add: 0
              average: 42
            - type: piercing
              dice:
                count: 13
                sides: 8
                add: 0
              average: 57
    contract:
      id: dnd2024:rules_traps_spiked_pit
      inputs:
        actor: participant
        targets: participant_list
        hazard: object
        event: event
        resolution: resolution
        choice: choice
      outputs:
      - effects
      - hazard_updates
      events:
      - movement.completed
      capabilities:
      - hazards
      - effects
      - saving_throws
    requires:
    - dnd2024:rules_traps_spiked_pit
    citations:
    - SRD 5.2.1 pp. 201; builtin/srd-5.2.1/source.txt; source section Traps; draft rules/traps_spiked_pit.md
  dnd2024:rules/combat_encounters_step_1_choose_a_difficulty:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/combat_encounters_step_1_choose_a_difficulty
        implementation: dnd2024:rules_combat_encounters_step_1_choose_a_difficulty
        parameters:
          difficulty_choice:
          - low
          - moderate
          - high
          low_difficulty_guideline:
            monster_count: 1
            party_size: 4
            monster_cr: character_level
            binding_rule: false
    contract:
      id: dnd2024:rules_combat_encounters_step_1_choose_a_difficulty
      inputs:
        party: participant_list
        choice: choice
      outputs:
      - encounter_difficulty
      events: []
      capabilities:
      - encounter_design
    requires:
    - dnd2024:rules_combat_encounters_step_1_choose_a_difficulty
    citations:
    - SRD 5.2.1 pp. 202; builtin/srd-5.2.1/source.txt; source section Combat Encounters; draft rules/combat_encounters_step_1_choose_a_difficulty.md
  dnd2024:rules/combat_encounters_troubleshooting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 203; builtin/srd-5.2.1/source.txt; source section Combat Encounters; draft rules/combat_encounters_troubleshooting.md
  dnd2024:rules/combat_encounters_cr_0_creatures:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 203; builtin/srd-5.2.1/source.txt; source section Combat Encounters; draft rules/combat_encounters_cr_0_creatures.md
  dnd2024:rules/combat_encounters_unusual_features:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: informational
    fields: {}
    citations:
    - SRD 5.2.1 pp. 203; builtin/srd-5.2.1/source.txt; source section Combat Encounters; draft rules/combat_encounters_unusual_features.md
  dnd2024:rules/magic_item_categories_armor:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_categories_armor
        implementation: dnd2024:rules_magic_item_categories_armor
        parameters:
          magic_armor:
            worn_required_for_magic: true
            description_can_override: true
            unspecified_armor_type:
              choice_authority: gm
              methods:
              - choose
              - random
              options: armor_table_types
    contract:
      id: dnd2024:rules_magic_item_categories_armor
      inputs:
        actor: participant
        item: item_instance
        choice: choice
      outputs:
      - item_attributes
      - actor_modifiers
      events: []
      capabilities:
      - equipment
      - armor_class
      - crafting
    requires:
    - dnd2024:rules_magic_item_categories_armor
    citations:
    - SRD 5.2.1 pp. 204; builtin/srd-5.2.1/source.txt; source section Magic Item Categories; draft rules/magic_item_categories_armor.md
  dnd2024:rules/magic_item_categories_rods:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_categories_rods
        implementation: dnd2024:rules_magic_item_categories_rods
        parameters:
          rod:
            typical_weight_lb:
              minimum: 2
              maximum: 5
            arcane_focus: true
            description_can_override: true
    contract:
      id: dnd2024:rules_magic_item_categories_rods
      inputs:
        actor: participant
        item: item_instance
        choice: choice
      outputs:
      - item_attributes
      - actor_modifiers
      events: []
      capabilities:
      - equipment
      - armor_class
      - crafting
    requires:
    - dnd2024:rules_magic_item_categories_rods
    citations:
    - SRD 5.2.1 pp. 204; builtin/srd-5.2.1/source.txt; source section Magic Item Categories; draft rules/magic_item_categories_rods.md
  dnd2024:rules/magic_item_categories_wands:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_categories_wands
        implementation: dnd2024:rules_magic_item_categories_wands
        parameters:
          wand:
            typical_length_inches:
              minimum: 12
              maximum: 15
            arcane_focus: true
            description_can_override: true
    contract:
      id: dnd2024:rules_magic_item_categories_wands
      inputs:
        actor: participant
        item: item_instance
        choice: choice
      outputs:
      - item_attributes
      - actor_modifiers
      events: []
      capabilities:
      - equipment
      - armor_class
      - crafting
    requires:
    - dnd2024:rules_magic_item_categories_wands
    citations:
    - SRD 5.2.1 pp. 205; builtin/srd-5.2.1/source.txt; source section Magic Item Categories; draft rules/magic_item_categories_wands.md
  dnd2024:rules/magic_item_rarity:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_rarity
        implementation: dnd2024:rules_magic_item_rarity
        parameters:
          rarity_values_gp:
            common: 100
            uncommon: 400
            rare: 4000
            very_rare: 40000
            legendary: 200000
            artifact: null
          artifact_value: priceless
          consumable_non_scroll_multiplier: 0.5
          spell_scroll_value:
            op: multiply
            left: scribe_cost
            right: 2
    contract:
      id: dnd2024:rules_magic_item_rarity
      inputs:
        actor: participant
        item: item_instance
        choice: choice
      outputs:
      - item_attributes
      - actor_modifiers
      events: []
      capabilities:
      - equipment
      - armor_class
      - crafting
    requires:
    - dnd2024:rules_magic_item_rarity
    citations:
    - SRD 5.2.1 pp. 205, 206; builtin/srd-5.2.1/source.txt; source section Magic Item Rarity; draft rules/magic_item_rarity.md
  dnd2024:rules/activating_a_magic_item_command_word:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/activating_a_magic_item_command_word
        implementation: dnd2024:rules_activating_a_magic_item_command_word
        parameters:
          command_word_modes:
          - spoken
          - signed
          spoken:
            audible_required: true
            sound_suppression_prevents_activation: true
    contract:
      id: dnd2024:rules_activating_a_magic_item_command_word
      inputs:
        actor: participant
        item: item_instance
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - item_updates
      - cost
      - effects
      events: []
      capabilities:
      - items
      - attunement
      - crafting
    requires:
    - dnd2024:rules_activating_a_magic_item_command_word
    citations:
    - SRD 5.2.1 pp. 206; builtin/srd-5.2.1/source.txt; source section Activating a Magic Item; draft rules/activating_a_magic_item_command_word.md
  dnd2024:rules/activating_a_magic_item_charges:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/activating_a_magic_item_charges
        implementation: dnd2024:rules_activating_a_magic_item_charges
        parameters:
          activation_expends_specified_charges: true
          identify_reveals: remaining_charges
          attuned_bearer_knows:
          - remaining_charges
          - charge_recovery
    contract:
      id: dnd2024:rules_activating_a_magic_item_charges
      inputs:
        actor: participant
        item: item_instance
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - item_updates
      - cost
      - effects
      events: []
      capabilities:
      - items
      - attunement
      - crafting
    requires:
    - dnd2024:rules_activating_a_magic_item_charges
    citations:
    - SRD 5.2.1 pp. 206; builtin/srd-5.2.1/source.txt; source section Activating a Magic Item; draft rules/activating_a_magic_item_charges.md
  dnd2024:rules/magic_item_resilience:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/magic_item_resilience
        implementation: dnd2024:rules_magic_item_resilience
        parameters:
          durability_minimum: nonmagical_equivalent
          default_resistance: all_damage
          default_resistance_exceptions:
          - potions
          - scrolls
          artifact:
            damage_immunity: true
            destruction_requires: artifact_specific_method
    contract:
      id: dnd2024:rules_magic_item_resilience
      inputs:
        actor: participant
        item: item_instance
        choice: choice
      outputs:
      - item_attributes
      - actor_modifiers
      events: []
      capabilities:
      - equipment
      - armor_class
      - crafting
    requires:
    - dnd2024:rules_magic_item_resilience
    citations:
    - SRD 5.2.1 pp. 206; builtin/srd-5.2.1/source.txt; source section Magic Item Resilience; draft rules/magic_item_resilience.md
  dnd2024:rules/crafting_magic_items_tools:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/crafting_magic_items_tools
        implementation: dnd2024:rules_crafting_magic_items_tools
        parameters:
          required_tool_use: true
          creator_proficiency_required: true
          assistant_proficiency_required: true
          category_tools:
            armor:
              options:
              - leatherworkers_tools
              - smiths_tools
              - weavers_tools
              selection: nonmagical_armor_tool_requirement
            potion:
              options:
              - alchemists_supplies
              - herbalism_kit
              selection: any
            ring:
              options:
              - jewelers_tools
            rod:
              options:
              - woodcarvers_tools
            scroll:
              options:
              - calligraphers_supplies
            staff:
              options:
              - woodcarvers_tools
            wand:
              options:
              - woodcarvers_tools
            weapon:
              options:
              - leatherworkers_tools
              - smiths_tools
              - woodcarvers_tools
              selection: nonmagical_weapon_tool_requirement
            wondrous_item:
              options:
              - tinkers_tools
              - nonmagical_base_item_required_tool
              selection: any
    contract:
      id: dnd2024:rules_crafting_magic_items_tools
      inputs:
        actor: participant
        item: item_instance
        choice: choice
      outputs:
      - item_attributes
      - actor_modifiers
      events: []
      capabilities:
      - equipment
      - armor_class
      - crafting
    requires:
    - dnd2024:rules_crafting_magic_items_tools
    citations:
    - SRD 5.2.1 pp. 206, 207; builtin/srd-5.2.1/source.txt; source section Crafting Magic Items; draft rules/crafting_magic_items_tools.md
  dnd2024:rules/sentient_magic_items:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sentient_magic_items
        implementation: dnd2024:rules_sentient_magic_items
        parameters:
          sentience:
            single_use_item_allowed: false
            controller: gm
            activated_properties_controller: gm
            good_relationship: bearer_access
            strained_relationship: possible_conflict
          conflict_rules:
            key: dnd2024:rules/sentient_magic_items_conflict
    contract:
      id: dnd2024:rules_sentient_magic_items
      inputs:
        actor: participant
        item: item_instance
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - item_updates
      - cost
      - effects
      events: []
      capabilities:
      - items
      - attunement
      - crafting
    requires:
    - dnd2024:rules_sentient_magic_items
    citations:
    - SRD 5.2.1 pp. 207; builtin/srd-5.2.1/source.txt; source section Sentient Magic Items; draft rules/sentient_magic_items.md
  dnd2024:rules/sentient_magic_items_alignment:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sentient_magic_items_alignment
        implementation: dnd2024:rules_sentient_magic_items_alignment
        parameters:
          alignment_required: true
          methods:
          - creator_or_nature
          - choose
          - roll
          roll:
            count: 1
            sides: 100
            add: 0
          table: &id002
          - roll_min: 1
            roll_max: 15
            alignment: lawful_good
          - roll_min: 16
            roll_max: 35
            alignment: neutral_good
          - roll_min: 36
            roll_max: 50
            alignment: chaotic_good
          - roll_min: 51
            roll_max: 63
            alignment: lawful_neutral
          - roll_min: 64
            roll_max: 73
            alignment: neutral
          - roll_min: 74
            roll_max: 85
            alignment: chaotic_neutral
          - roll_min: 86
            roll_max: 89
            alignment: lawful_evil
          - roll_min: 90
            roll_max: 96
            alignment: neutral_evil
          - roll_min: 97
            roll_max: 100
            alignment: chaotic_evil
    contract:
      id: dnd2024:rules_sentient_magic_items_alignment
      inputs:
        actor: participant
        item: item_instance
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - item_updates
      - cost
      - effects
      events: []
      capabilities:
      - items
      - attunement
      - crafting
    requires:
    - dnd2024:rules_sentient_magic_items_alignment
    citations:
    - SRD 5.2.1 pp. 207; builtin/srd-5.2.1/source.txt; source section Sentient Magic Items; draft rules/sentient_magic_items_alignment.md
  dnd2024:rules/sentient_magic_items_special_purpose:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/sentient_magic_items_special_purpose
        implementation: dnd2024:rules_sentient_magic_items_special_purpose
        parameters:
          purpose_optional: true
          purpose_methods:
          - choose
          - roll
          cooperative_when: wielder_use_aligns_with_purpose
          possible_conflict_when: wielder_deviates_from_purpose
          alignment_table:
            roll:
              count: 1
              sides: 100
              add: 0
            outcomes: *id002
          communication_table:
            roll:
              count: 1
              sides: 10
              add: 0
            outcomes:
            - roll_min: 1
              roll_max: 6
              mode: transmit_emotions
              recipient: carrier_or_wielder
            - roll_min: 7
              roll_max: 9
              mode: speech
              languages_minimum: 1
            - roll_min: 10
              roll_max: 10
              mode:
              - speech
              - telepathy
              languages_minimum: 1
              telepathy_recipient: carrier_or_wielder
          senses_table:
            roll:
              count: 1
              sides: 4
              add: 0
            outcomes:
            - roll: 1
              hearing_feet: 30
              vision_feet: 30
              vision_type: standard
            - roll: 2
              hearing_feet: 60
              vision_feet: 60
              vision_type: standard
            - roll: 3
              hearing_feet: 120
              vision_feet: 120
              vision_type: standard
            - roll: 4
              hearing_feet: 120
              vision_feet: 120
              vision_type: darkvision
          purpose_table:
            roll:
              count: 1
              sides: 10
              add: 0
            outcomes:
            - roll: 1
              id: aligned
              objective: defeat_or_destroy_opposed_alignment
              neutral_allowed: false
            - roll: 2
              id: bane
              objective: thwart_or_destroy
              target_choice: creature_type
            - roll: 3
              id: creator_seeker
              objectives:
              - find_creator
              - understand_creation
            - roll: 4
              id: destiny_seeker
              objective: fulfill_item_and_bearer_future_roles
            - roll: 5
              id: destroyer
              objectives:
              - destruction
              - arbitrary_fighting
            - roll: 6
              id: glory_seeker
              objectives:
              - greatest_item_renown
              - user_fame_or_notoriety
            - roll: 7
              id: lore_seeker
              objective_choices:
              - knowledge
              - solve_mystery
              - learn_secret
              - decipher_prophecy
            - roll: 8
              id: protector
              objective: defend
              target_choice: creature_kind
            - roll: 9
              id: soulmate_seeker
              objective: find_other_sentient_item
            - roll: 10
              id: templar
              objective: defend_servants_and_interests
              target_choice: deity
          conflict_rules:
            key: dnd2024:rules/sentient_magic_items_conflict
    contract:
      id: dnd2024:rules_sentient_magic_items_special_purpose
      inputs:
        actor: participant
        item: item_instance
        choice: choice
        event:
          type: event
          required: false
      outputs:
      - item_updates
      - cost
      - effects
      events: []
      capabilities:
      - items
      - attunement
      - crafting
    requires:
    - dnd2024:rules_sentient_magic_items_special_purpose
    citations:
    - SRD 5.2.1 pp. 208; builtin/srd-5.2.1/source.txt; source section Sentient Magic Items; draft rules/sentient_magic_items_special_purpose.md
  dnd2024:rules/stat_block_overview:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/stat_block_overview
        implementation: dnd2024:rules_stat_block_overview
        parameters:
          stat_block_sections:
          - name_size_type_tags_alignment
          - ac_hp_speed_initiative
          - ability_scores_modifiers_saves
          - senses_languages_cr_optional_details
          - traits
          - actions
          - bonus_actions
          - reactions
          - legendary_actions
          traits_active: always_or_stated_situations
          action_options: stat_block_plus_universal_actions
          optional_detail_omission_means_absent: true
    contract:
      id: dnd2024:rules_stat_block_overview
      inputs:
        definition: definition
      outputs:
      - typed_attributes
      events: []
      capabilities:
      - definition_catalogs
    requires:
    - dnd2024:rules_stat_block_overview
    citations:
    - SRD 5.2.1 pp. 254; builtin/srd-5.2.1/source.txt; source section Stat Block Overview; draft rules/stat_block_overview.md
  dnd2024:rules/parts_of_a_stat_block_creature_type:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_creature_type
        implementation: dnd2024:rules_parts_of_a_stat_block_creature_type
        parameters:
          creature_type_enum:
          - aberration
          - beast
          - celestial
          - construct
          - dragon
          - elemental
          - fey
          - fiend
          - giant
          - humanoid
          - monstrosity
          - ooze
          - plant
          - undead
          type_inherent_rules: false
          type_filters_other_effects: true
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_creature_type
      inputs:
        definition: definition
      outputs:
      - typed_attributes
      events: []
      capabilities:
      - definition_catalogs
    requires:
    - dnd2024:rules_parts_of_a_stat_block_creature_type
    citations:
    - SRD 5.2.1 pp. 254; builtin/srd-5.2.1/source.txt; source section Parts of a Stat Block; draft rules/parts_of_a_stat_block_creature_type.md
  dnd2024:rules/parts_of_a_stat_block_armor_class:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_armor_class
        implementation: dnd2024:rules_parts_of_a_stat_block_armor_class
        parameters:
          stat_block_ac_includes:
          - natural_armor
          - dexterity
          - gear
          - other_defenses
          do_not_readd_included_bonuses: true
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_armor_class
      inputs:
        monster: participant
      outputs:
      - armor_class
      events: []
      capabilities:
      - armor_class
    requires:
    - dnd2024:rules_parts_of_a_stat_block_armor_class
    citations:
    - SRD 5.2.1 pp. 254; builtin/srd-5.2.1/source.txt; source section Parts of a Stat Block; draft rules/parts_of_a_stat_block_armor_class.md
  dnd2024:rules/parts_of_a_stat_block_speed:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_speed
        implementation: dnd2024:rules_parts_of_a_stat_block_speed
        parameters:
          speed_fields:
          - walk
          - burrow
          - climb
          - fly
          - swim
          rules:
          - key: dnd2024:glossary/speed
          - key: dnd2024:glossary/burrow_speed
          - key: dnd2024:glossary/climb_speed
          - key: dnd2024:glossary/fly_speed
          - key: dnd2024:glossary/swim_speed
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_speed
      inputs:
        definition: definition
      outputs:
      - typed_attributes
      events: []
      capabilities:
      - definition_catalogs
    requires:
    - dnd2024:rules_parts_of_a_stat_block_speed
    citations:
    - SRD 5.2.1 pp. 255; builtin/srd-5.2.1/source.txt; source section Parts of a Stat Block; draft rules/parts_of_a_stat_block_speed.md
  dnd2024:rules/parts_of_a_stat_block_resistances_and_vulnerabilities:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_resistances_and_vulnerabilities
        implementation: dnd2024:rules_parts_of_a_stat_block_resistances_and_vulnerabilities
        parameters:
          damage_modifiers_source: stat_block_resistance_and_vulnerability_entries
          damage_rule:
            key: dnd2024:rules/damage_and_healing_resistance_and_vulnerability
          combat_tactics_guidance:
            special_limited_abilities: as_early_and_often_as_possible
            multiattack: unless_more_powerful_ability_used
            bonus_reaction_legendary_actions: as_often_as_possible
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_resistances_and_vulnerabilities
      inputs:
        monster: participant
        damage: resolution
      outputs:
      - damage_modifiers
      - tactics
      events: []
      capabilities:
      - damage
      - monster_actions
    requires:
    - dnd2024:rules_parts_of_a_stat_block_resistances_and_vulnerabilities
    citations:
    - SRD 5.2.1 pp. 255; builtin/srd-5.2.1/source.txt; source section Parts of a Stat Block; draft rules/parts_of_a_stat_block_resistances_and_vulnerabilities.md
  dnd2024:rules/parts_of_a_stat_block_ammunition_and_ranged_attacks:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_ammunition_and_ranged_attacks
        implementation: dnd2024:rules_parts_of_a_stat_block_ammunition_and_ranged_attacks
        parameters:
          stat_block_ranged_weapon_ammunition: necessary_ammunition_carried
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_ammunition_and_ranged_attacks
      inputs:
        monster: participant
        weapon: item_instance
      outputs:
      - ammunition
      events: []
      capabilities:
      - equipment
    requires:
    - dnd2024:rules_parts_of_a_stat_block_ammunition_and_ranged_attacks
    citations:
    - SRD 5.2.1 pp. 255; builtin/srd-5.2.1/source.txt; source section Parts of a Stat Block; draft rules/parts_of_a_stat_block_ammunition_and_ranged_attacks.md
  dnd2024:rules/parts_of_a_stat_block_languages:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_languages
        implementation: dnd2024:rules_parts_of_a_stat_block_languages
        parameters:
          language_entry_modes:
          - communicate
          - understand_only
          - none
          none:
            understands_any_language: false
          understand_only:
            can_communicate_in_language: false
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_languages
      inputs:
        definition: definition
      outputs:
      - typed_attributes
      events: []
      capabilities:
      - definition_catalogs
    requires:
    - dnd2024:rules_parts_of_a_stat_block_languages
    citations:
    - SRD 5.2.1 pp. 256; builtin/srd-5.2.1/source.txt; source section Parts of a Stat Block; draft rules/parts_of_a_stat_block_languages.md
  dnd2024:rules/parts_of_a_stat_block_experience_points:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_experience_points
        implementation: dnd2024:rules_parts_of_a_stat_block_experience_points
        parameters:
          xp_table:
          - cr: 0
            xp: null
          - cr: 0.125
            xp: 25
          - cr: 0.25
            xp: 50
          - cr: 0.5
            xp: 100
          - cr: 1
            xp: 200
          - cr: 2
            xp: 450
          - cr: 3
            xp: 700
          - cr: 4
            xp: 1100
          - cr: 5
            xp: 1800
          - cr: 6
            xp: 2300
          - cr: 7
            xp: 2900
          - cr: 8
            xp: 3900
          - cr: 9
            xp: 5000
          - cr: 10
            xp: 5900
          - cr: 11
            xp: 7200
          - cr: 12
            xp: 8400
          - cr: 13
            xp: 10000
          - cr: 14
            xp: 11500
          - cr: 15
            xp: 13000
          - cr: 16
            xp: 15000
          - cr: 17
            xp: 18000
          - cr: 18
            xp: 20000
          - cr: 19
            xp: 22000
          - cr: 20
            xp: 25000
          - cr: 21
            xp: 33000
          - cr: 22
            xp: 41000
          - cr: 23
            xp: 50000
          - cr: 24
            xp: 62000
          - cr: 25
            xp: 75000
          - cr: 26
            xp: 90000
          - cr: 27
            xp: 105000
          - cr: 28
            xp: 120000
          - cr: 29
            xp: 135000
          - cr: 30
            xp: 155000
          cr_zero_xp_options:
          - 0
          - 10
          award_events:
          - defeated_in_combat
          - otherwise_neutralized
          magically_summoned_xp: stat_block_unless_overridden
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_experience_points
      inputs:
        monster: participant
        event: event
      outputs:
      - xp_award
      events:
      - creature.defeated
      capabilities:
      - experience
    requires:
    - dnd2024:rules_parts_of_a_stat_block_experience_points
    citations:
    - SRD 5.2.1 pp. 256; builtin/srd-5.2.1/source.txt; source section Parts of a Stat Block; draft rules/parts_of_a_stat_block_experience_points.md
  dnd2024:rules/parts_of_a_stat_block_actions:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_actions
        implementation: dnd2024:rules_parts_of_a_stat_block_actions
        parameters:
          action_options:
            union:
            - stat_block_actions
            - universal_actions
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_actions
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_parts_of_a_stat_block_actions
    citations:
    - SRD 5.2.1 pp. 256; builtin/srd-5.2.1/source.txt; source section Parts of a Stat Block; draft rules/parts_of_a_stat_block_actions.md
  dnd2024:rules/parts_of_a_stat_block_damage_notation:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_damage_notation
        implementation: dnd2024:rules_parts_of_a_stat_block_damage_notation
        parameters:
          damage_mode_choice:
            options:
            - fixed_average
            - dice_expression
            count: 1
            authority: gm
          combine_modes: false
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_damage_notation
      inputs:
        damage: resolution
        choice: choice
      outputs:
      - damage_amount
      events: []
      capabilities:
      - damage
    requires:
    - dnd2024:rules_parts_of_a_stat_block_damage_notation
    citations:
    - SRD 5.2.1 pp. 257; builtin/srd-5.2.1/source.txt; source section Parts of a Stat Block; draft rules/parts_of_a_stat_block_damage_notation.md
  dnd2024:rules/parts_of_a_stat_block_bonus_action:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_bonus_action
        implementation: dnd2024:rules_parts_of_a_stat_block_bonus_action
        parameters:
          bonus_action_options: stat_block_bonus_actions
          rules:
            key: dnd2024:rules/actions_bonus_actions
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_bonus_action
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events: []
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_parts_of_a_stat_block_bonus_action
    citations:
    - SRD 5.2.1 pp. 257; builtin/srd-5.2.1/source.txt; source section Parts of a Stat Block; draft rules/parts_of_a_stat_block_bonus_action.md
  dnd2024:rules/parts_of_a_stat_block_limited_usage:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:rules/parts_of_a_stat_block_limited_usage
        implementation: dnd2024:rules_parts_of_a_stat_block_limited_usage
        parameters:
          usage_modes:
            x_per_day:
              maximum_uses: x
              recovery_event: long_rest.finished
              recovery: all
            recharge_x_y:
              maximum_uses: 1
              roll_event: actor_turn.started
              roll:
                count: 1
                sides: 6
                add: 0
              recharge_when:
                minimum: x
                maximum: y
              rest_recovery:
              - short_rest.finished
              - long_rest.finished
            recharge_after_rest:
              maximum_uses: 1
              recovery_events:
              - short_rest.finished
              - long_rest.finished
    contract:
      id: dnd2024:rules_parts_of_a_stat_block_limited_usage
      inputs:
        actor: participant
        targets: participant_list
        event: event
        choice: choice
      outputs:
      - action_options
      - effects
      - resource_updates
      events:
      - turn.started
      - rest.short_completed
      - rest.long_completed
      capabilities:
      - action_economy
      - movement
      - combat
    requires:
    - dnd2024:rules_parts_of_a_stat_block_limited_usage
    citations:
    - SRD 5.2.1 pp. 257; builtin/srd-5.2.1/source.txt; source section Parts of a Stat Block; draft rules/parts_of_a_stat_block_limited_usage.md
---

# Supplementary SRD rule contracts, group 3

All 174 assigned notes were checked against their official SRD 5.2.1 section text. The source SHA-256 values are the draft conversion metadata values for the bundled official PDF. Contracts describe definition behavior; they do not implement engines.

Tables split by PDF columns were reconstructed from `builtin/srd-5.2.1/source.txt`: Skills (p. 9), Starting Equipment at Higher Levels (p. 24), Armor (p. 92), Airborne and Waterborne Vehicles (p. 101), Magic Item Rarities and Values (p. 206), and sentient-item tables (p. 208). The vehicle table fragment attached to Wretched is fully typed there as well as with the vehicle rules. Sentient alignment, communication, senses, and purpose tables stay together in the assigned Special Purpose span. Source examples that are guidance retain an explicit nonbinding or GM-choice field.

Class start and progression sections link to existing class and feature definitions. They grant the named core records and features at the stated class levels. Spell lists carry every printed membership, level, school, and special flag. This preserves the distinction between a class list and rules that grant preparation or casting access.

The 2024 text for travel gives land vehicles normal pace selection on p. 12 and all vehicles their listed speed without pace selection on p. 192. Each note retains its own stated scope; a later policy decision must choose which section applies to a travel procedure. No contrary rule was silently removed.

Creature-type labels have no inherent behavior, but they are typed because other effects use these labels for eligibility. Flavor, organizational headings, and open-ended advice without a rule operation are informational. All explicitly numerical effects, costs, durations, random outcomes, and prerequisite choices are machine-readable.

Accounting: 144 specified, 30 informational, 174 total. No assigned note omitted.

Informational notes:

- `dnd2024:rules/social_interaction_roleplaying`: roleplaying_advice.
- `dnd2024:rules/exploration_adventuring_equipment`: equipment_use_examples.
- `dnd2024:rules/exploration_interacting_with_objects`: object_interaction_introduction.
- `dnd2024:rules/combat_ranged_attacks`: ranged_attack_introduction.
- `dnd2024:rules/choose_a_character_sheet`: record_format_advice.
- `dnd2024:rules/create_your_character_imagine_your_past_and_present`: character_history_prompts.
- `dnd2024:rules/create_your_character_step_5_character_creation_details`: character_sheet_completion_introduction.
- `dnd2024:rules/level_advancement_tier_1_levels_1_4`: tier_1_narrative.
- `dnd2024:rules/level_advancement_tier_4_levels_17_20`: tier_4_narrative.
- `dnd2024:rules/classes`: class_heading.
- `dnd2024:rules/bard_becoming_a_bard`: class_becoming_heading.
- `dnd2024:rules/cleric_becoming_a_cleric`: class_becoming_heading.
- `dnd2024:rules/monk_becoming_a_monk`: class_becoming_heading.
- `dnd2024:rules/ranger_becoming_a_ranger`: class_becoming_heading.
- `dnd2024:rules/sorcerer_becoming_a_sorcerer`: class_becoming_heading.
- `dnd2024:rules/character_backgrounds_parts_of_a_background`: background_parts_heading.
- `dnd2024:rules/character_backgrounds_background_descriptions`: background_descriptions_heading.
- `dnd2024:rules/feat_descriptions`: feat_organization.
- `dnd2024:rules/feat_descriptions_general_feats`: general_feat_heading.
- `dnd2024:rules/equipment`: equipment_heading.
- `dnd2024:rules/adventuring_gear`: adventuring_gear_introduction.
- `dnd2024:rules/gameplay_toolbox`: gameplay_toolbox_heading.
- `dnd2024:rules/magical_contagions_curses`: heading_or_narrative_introduction.
- `dnd2024:rules/magical_contagions_cursed_magic_items`: heading_or_narrative_introduction.
- `dnd2024:rules/magical_contagions_2`: heading_or_narrative_introduction.
- `dnd2024:rules/environmental_effects`: heading_or_narrative_introduction.
- `dnd2024:rules/poison_purchasing_poison`: poison_availability_advice.
- `dnd2024:rules/combat_encounters_troubleshooting`: encounter_design_advice.
- `dnd2024:rules/combat_encounters_cr_0_creatures`: encounter_design_advice.
- `dnd2024:rules/combat_encounters_unusual_features`: encounter_design_advice.

---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:glossary/ability_check:
    source_sha256: d454758d3928c846aad57bd648e6149a9e5afa9ce608742e9e3c443e81e24b01
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/ability_check
        implementation: dnd2024:glossary_ability_check
        parameters:
          test_kind: ability_check
          base_rule:
            key: dnd2024:glossary/d20_test
          abilities: &id001
          - strength
          - dexterity
          - constitution
          - intelligence
          - wisdom
          - charisma
          skill_optional: true
    contract:
      id: dnd2024:glossary_ability_check
      inputs:
        subject: participant
        ability: choice
        skill:
          type: definition
          required: false
      outputs:
      - check_definition
      events: []
      capabilities:
      - checks
    requires:
    - dnd2024:glossary_ability_check
    citations:
    - SRD 5.2.1 pp. 176; builtin/srd-5.2.1/glossary/ability_check.md
  dnd2024:glossary/ability_score_and_modifier:
    source_sha256: ccf63b312c8ff8dc27bdd53c166466e7c3cd7b7c91eab374975261ea3695b004
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/ability_score_and_modifier
        implementation: dnd2024:glossary_ability_score_and_modifier
        parameters:
          abilities: *id001
          d20_modifier:
            when: corresponding_ability_used
            add:
              var: ability_modifier
          other_modifier_application: when_rule_requires
    contract:
      id: dnd2024:glossary_ability_score_and_modifier
      inputs:
        subject: participant
        ability: choice
        ability_modifier: integer
      outputs:
      - modifier
      events: []
      capabilities:
      - checks
      - statistics
    requires:
    - dnd2024:glossary_ability_score_and_modifier
    citations:
    - SRD 5.2.1 pp. 176; builtin/srd-5.2.1/glossary/ability_score_and_modifier.md
  dnd2024:glossary/action:
    source_sha256: 372ed8e77309a2bc9f93c47358947ef317bc897a2f45aa3e3b61c6969adc55bc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/action
        implementation: dnd2024:glossary_action
        parameters:
          budget:
            per_turn: 1
            timing: own_turn
          options:
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
          additional_options: granted_feature_actions
    contract:
      id: dnd2024:glossary_action
      inputs:
        subject: participant
        selected_action: definition
      outputs:
      - action_budget
      - legal_actions
      events:
      - turn.started
      - action.requested
      capabilities:
      - action_economy
    requires:
    - dnd2024:glossary_action
    citations:
    - SRD 5.2.1 pp. 176; builtin/srd-5.2.1/glossary/action.md
  dnd2024:glossary/advantage:
    source_sha256: 896a872e141dce120ec01899bb26ef047b324c2b783525ceeba8a075d15c4c6a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/advantage
        implementation: dnd2024:glossary_advantage
        parameters:
          test_kinds:
          - ability_check
          - attack_roll
          - saving_throw
          dice:
            count: 2
            sides: 20
            select: highest
          same_kind_stacking: false
          opposite_kind_cancellation:
            any_advantage_and_any_disadvantage: true
            result:
              count: 1
              sides: 20
            ignore_source_counts: true
    contract:
      id: dnd2024:glossary_advantage
      inputs:
        subject: participant
        test: resolution
      outputs:
      - roll_mode
      events:
      - d20_test.pending
      capabilities:
      - checks
    requires:
    - dnd2024:glossary_advantage
    citations:
    - SRD 5.2.1 pp. 176; builtin/srd-5.2.1/glossary/advantage.md
  dnd2024:glossary/adventure:
    source_sha256: 9c64a573dff9069fda2bac1c0765cbe56016f792b7c4a3e0434b45871f3d1292
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 176; builtin/srd-5.2.1/glossary/adventure.md
  dnd2024:glossary/alignment:
    source_sha256: e779f409f9389f2bc699c372e30df4999feb99e4566162d0685be3a3c5730bf8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/alignment
        implementation: dnd2024:glossary_alignment
        parameters:
          morality:
          - good
          - evil
          - neutral
          order:
          - lawful
          - chaotic
          - neutral
          combination: cartesian_product
          combination_count: 9
    contract:
      id: dnd2024:glossary_alignment
      inputs:
        subject: participant
        morality: choice
        order: choice
      outputs:
      - alignment
      events: []
      capabilities:
      - statistics
    requires:
    - dnd2024:glossary_alignment
    citations:
    - SRD 5.2.1 pp. 176; builtin/srd-5.2.1/glossary/alignment.md
  dnd2024:glossary/ally:
    source_sha256: 80ff871ac60e811ae6c64a1725db00306b16ee76dbba5e3124c223bc4b1c9554
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/ally
        implementation: dnd2024:glossary_ally
        parameters:
          predicate:
            any:
            - same_adventuring_party
            - friend
            - same_combat_side
            - rule_designated_ally
            - gm_designated_ally
          gm_designation:
            chooser: gm
            options:
            - true
            - false
    contract:
      id: dnd2024:glossary_ally
      inputs:
        subject: participant
        other: participant
        relationship: choice
      outputs:
      - is_ally
      events: []
      capabilities:
      - relationships
    requires:
    - dnd2024:glossary_ally
    citations:
    - SRD 5.2.1 pp. 176; builtin/srd-5.2.1/glossary/ally.md
  dnd2024:glossary/area_of_effect:
    source_sha256: 9650a5fa856cf12161e499c28260a18fb4aa3312db555cac19b12f26ac2a7055
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/area_of_effect
        implementation: dnd2024:glossary_area_of_effect
        parameters:
          shapes:
          - cone
          - cube
          - cylinder
          - emanation
          - line
          - sphere
          origin: shape_defined
          location_excluded_when:
            all_straight_lines_from_origin_blocked: true
            blocking_obstruction_cover: total
          unseen_origin_behind_obstruction:
            replace_origin: near_side_of_obstruction
          geometry_assumptions:
            provider: external_geometry
            required_facts:
            - straight_line_blockage
            - total_cover
            - creator_visibility
            - nearest_obstruction_near_side
    contract:
      id: dnd2024:glossary_area_of_effect
      inputs:
        creator: participant
        origin: location
        area: effect
        geometry: resolution
      outputs:
      - effective_origin
      - included_locations
      events: []
      capabilities:
      - areas
      - geometry
      - visibility
    requires:
    - dnd2024:glossary_area_of_effect
    citations:
    - SRD 5.2.1 pp. 177; builtin/srd-5.2.1/glossary/area_of_effect.md
  dnd2024:glossary/armor_class:
    source_sha256: 80b442493f912d85b0fa4b3472d3bcfae4ace74771097a416a882782f6768616
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/armor_class
        implementation: dnd2024:glossary_armor_class
        parameters:
          attack_target_number: armor_class
          default_base:
            op: add
            args:
            - 10
            - var: dexterity_modifier
          alternative_base:
            chooser: subject
            options: available_base_ac_calculations
            select_count: 1
            stack: false
    contract:
      id: dnd2024:glossary_armor_class
      inputs:
        subject: participant
        selected_base: choice
      outputs:
      - armor_class
      events: []
      capabilities:
      - statistics
      - attacks
    requires:
    - dnd2024:glossary_armor_class
    citations:
    - SRD 5.2.1 pp. 177; builtin/srd-5.2.1/glossary/armor_class.md
  dnd2024:glossary/armor_training:
    source_sha256: 28995b4bb44238bba3e2e46176ddeede241e48e0255bd805f31515140f338feb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/armor_training
        implementation: dnd2024:glossary_armor_training
        parameters:
          categories:
          - light
          - medium
          - heavy
          - shield
          untrained_armor:
            categories:
            - light
            - medium
            - heavy
            d20_disadvantage_abilities:
            - strength
            - dexterity
            spellcasting_allowed: false
          untrained_shield:
            shield_ac_bonus: 0
    contract:
      id: dnd2024:glossary_armor_training
      inputs:
        subject: participant
        armor: item_instance
      outputs:
      - drawbacks
      events:
      - equipment.changed
      - d20_test.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - checks
      - spellcasting
    requires:
    - dnd2024:glossary_armor_training
    citations:
    - SRD 5.2.1 pp. 177; builtin/srd-5.2.1/glossary/armor_training.md
  dnd2024:glossary/attack_action:
    source_sha256: 3cbefc9142f5b358f0bb91862c9ba8e91ebf71ec559fff3f51858afee1b105c4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/attack_action
        implementation: dnd2024:glossary_attack_action
        parameters:
          cost: action
          default_attacks: 1
          attack_options:
          - weapon
          - unarmed_strike
          equipment_interaction:
            per_attack: 1
            operation:
              chooser: subject
              options:
              - equip
              - unequip
              - none
            timing:
              chooser: subject
              options:
              - before_attack
              - after_attack
            equipped_weapon_must_be_used: false
            equip_methods:
            - draw
            - pick_up
            unequip_methods:
            - sheathe
            - stow
            - drop
          multiple_attacks:
            requires: feature_granted_additional_attacks
            movement_between_attacks: true
            movement_spend: some_or_all_available_turn_movement
    contract:
      id: dnd2024:glossary_attack_action
      inputs:
        subject: participant
        targets: participant_list
        weapons:
          type: item_list
          required: false
        attack_plan: choice
      outputs:
      - attacks
      - equipment_changes
      - movement
      events:
      - action.requested
      - attack.pending
      - attack.completed
      capabilities:
      - action_economy
      - attacks
      - equipment
      - movement
    requires:
    - dnd2024:glossary_attack_action
    citations:
    - SRD 5.2.1 pp. 177; builtin/srd-5.2.1/glossary/attack_action.md
  dnd2024:glossary/attack_roll:
    source_sha256: 0bba4d2e73a76f80d0824ca5d40b5cd3ada7af94801e056297d6cc75d542affa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/attack_roll
        implementation: dnd2024:glossary_attack_roll
        parameters:
          test_kind: attack_roll
          base_rule:
            key: dnd2024:glossary/d20_test
          sources:
          - weapon
          - unarmed_strike
          - spell
    contract:
      id: dnd2024:glossary_attack_roll
      inputs:
        subject: participant
        attack: definition
      outputs:
      - check_definition
      events: []
      capabilities:
      - attacks
      - checks
    requires:
    - dnd2024:glossary_attack_roll
    citations:
    - SRD 5.2.1 pp. 177; builtin/srd-5.2.1/glossary/attack_roll.md
  dnd2024:glossary/attitude:
    source_sha256: b9b4fc8dd85e53a41303c6547fcb63a9030310372cae8d4ab2b7900d7411b9e7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/attitude
        implementation: dnd2024:glossary_attitude
        parameters:
          scope: monster_toward_player_character
          states:
          - friendly
          - hostile
          - indifferent
          default: indifferent
    contract:
      id: dnd2024:glossary_attitude
      inputs:
        subject: participant
        other: participant
        attitude: choice
      outputs:
      - attitude
      events: []
      capabilities:
      - relationships
    requires:
    - dnd2024:glossary_attitude
    citations:
    - SRD 5.2.1 pp. 177; builtin/srd-5.2.1/glossary/attitude.md
  dnd2024:glossary/attunement:
    source_sha256: 33488ee5ab707eec5b9bde46fc999c7f3f793d0218c2f1e0f9e7cc460ab6289b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/attunement
        implementation: dnd2024:glossary_attunement
        parameters:
          maximum_items: 3
          magical_properties_require_attunement_if_item_requires: true
    contract:
      id: dnd2024:glossary_attunement
      inputs:
        subject: participant
        item: item_instance
      outputs:
      - attunement_eligibility
      - property_access
      events:
      - attunement.requested
      - item.activation_pending
      capabilities:
      - equipment
      - attunement
    requires:
    - dnd2024:glossary_attunement
    citations:
    - SRD 5.2.1 pp. 177; builtin/srd-5.2.1/glossary/attunement.md
  dnd2024:glossary/blinded_condition:
    source_sha256: 17dcd9c9da1141f3e929f365a17310a2e0c050946421096e030a2629969e1f7a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/blinded_condition
        implementation: dnd2024:glossary_blinded_condition
        parameters:
          condition: blinded
          can_see: false
          automatic_failure:
            check_kind: ability_check
            requires_sight: true
          attack_rolls_against: advantage
          own_attack_rolls: disadvantage
    contract:
      id: dnd2024:glossary_blinded_condition
      inputs:
        subject: participant
      outputs:
      - condition_effects
      events:
      - condition.changed
      - ability_check.pending
      - attack.pending
      capabilities:
      - conditions
      - visibility
      - checks
      - attacks
    requires:
    - dnd2024:glossary_blinded_condition
    citations:
    - SRD 5.2.1 pp. 177; builtin/srd-5.2.1/glossary/blinded_condition.md
  dnd2024:glossary/blindsight:
    source_sha256: f645de72da7c10c73d06629ae005b1064de7bbbebf1eeb2a3d386fe0930d8695
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/blindsight
        implementation: dnd2024:glossary_blindsight
        parameters:
          range: feature_specified
          physical_sight_required: false
          sees_while:
          - blinded
          - darkness
          sees_invisible: true
          total_cover_blocks: true
    contract:
      id: dnd2024:glossary_blindsight
      inputs:
        subject: participant
        target: object
        range_feet: number
        geometry: resolution
      outputs:
      - can_see
      events: []
      capabilities:
      - senses
      - geometry
    requires:
    - dnd2024:glossary_blindsight
    citations:
    - SRD 5.2.1 pp. 177; builtin/srd-5.2.1/glossary/blindsight.md
  dnd2024:glossary/bloodied:
    source_sha256: f79d51ec3ad677a74d5e95857e67af09b1dbe8bf814f0a813a44514b55072500
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/bloodied
        implementation: dnd2024:glossary_bloodied
        parameters:
          predicate:
            op: less_or_equal
            args:
            - var: current_hit_points
            - op: divide
              args:
              - var: hit_point_maximum
              - 2
    contract:
      id: dnd2024:glossary_bloodied
      inputs:
        subject: participant
      outputs:
      - is_bloodied
      events:
      - hit_points.changed
      capabilities:
      - hit_points
    requires:
    - dnd2024:glossary_bloodied
    citations:
    - SRD 5.2.1 pp. 177; builtin/srd-5.2.1/glossary/bloodied.md
  dnd2024:glossary/bonus_action:
    source_sha256: d2121c27d8bdc04ce42404313090db2eb70b12bf52fb0a8279c288a5c316f1a9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/bonus_action
        implementation: dnd2024:glossary_bonus_action
        parameters:
          budget:
            per_turn: 1
            timing: own_turn
          explicit_rule_grant_required: true
          compatible_with_action: true
    contract:
      id: dnd2024:glossary_bonus_action
      inputs:
        subject: participant
        selected_bonus_action: definition
      outputs:
      - bonus_action_budget
      - eligibility
      events:
      - turn.started
      - bonus_action.requested
      capabilities:
      - action_economy
    requires:
    - dnd2024:glossary_bonus_action
    citations:
    - SRD 5.2.1 pp. 177; builtin/srd-5.2.1/glossary/bonus_action.md
  dnd2024:glossary/breaking_objects:
    source_sha256: b7537562ee32250148e27a8a3c531a7bbee523c11c70ab6a87766a19333ef3fb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/breaking_objects
        implementation: dnd2024:glossary_breaking_objects
        parameters:
          damage_sources:
          - attacks
          - spells_that_affect_objects
          fragile_automatic_break:
            eligibility: exceedingly_fragile
            approval:
              chooser: gm
              options:
              - true
              - false
            cost_options:
            - attack_action
            - utilize_action
          suggested_ac:
            cloth: 11
            paper: 11
            rope: 11
            crystal: 13
            glass: 13
            ice: 13
            wood: 15
            stone: 17
            iron: 19
            steel: 19
            mithral: 21
            adamantine: 23
          suggested_hit_points:
            tiny:
              fragile:
                fixed: 2
                dice: 1d4
              resilient:
                fixed: 5
                dice: 2d4
            small:
              fragile:
                fixed: 3
                dice: 1d6
              resilient:
                fixed: 10
                dice: 3d6
            medium:
              fragile:
                fixed: 4
                dice: 1d8
              resilient:
                fixed: 18
                dice: 4d8
            large:
              fragile:
                fixed: 5
                dice: 1d10
              resilient:
                fixed: 27
                dice: 5d10
          statistics_selection:
            chooser: gm
            suggested_values_binding: false
            ac_type: integer
            hit_points_type: integer
          destroyed_at_hit_points: 0
          huge_or_gargantuan:
            partition_into_sizes:
            - tiny
            - small
            - medium
            - large
            hit_points_per_section: true
            whole_object_collapse_on_section_destruction:
              chooser: gm
              options:
              - true
              - false
          damage_immunities:
          - poison
          - psychic
          damage_type_effectiveness:
            chooser: gm
            scope: object_and_damage_type
            options:
            - normal
            - immune
            - resistant
            - vulnerable
            example:
              materials:
              - paper
              - cloth
              damage_type: fire
              response: vulnerable
          damage_threshold:
            allowed: true
            typical: big_objects
            rule:
              key: dnd2024:glossary/damage_threshold
          ability_scores:
            default: absent
            exception: explicit_rule_assignment
            without_scores:
              ability_checks_allowed: false
              all_saving_throws: automatic_failure
    contract:
      id: dnd2024:glossary_breaking_objects
      inputs:
        object: object
        attack:
          type: resolution
          required: false
        gm_choices: choice
      outputs:
      - object_statistics
      - damage
      - destruction
      events:
      - damage.pending
      - hit_points.changed
      capabilities:
      - objects
      - checks
      - damage
      - gm_choices
    requires:
    - dnd2024:glossary_breaking_objects
    citations:
    - SRD 5.2.1 pp. 177, 178; builtin/srd-5.2.1/glossary/breaking_objects.md
  dnd2024:glossary/bright_light:
    source_sha256: 41f0281e4398a39857dd5ba5279f0eb24f080000621736ae411acd366ff1772c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/bright_light
        implementation: dnd2024:glossary_bright_light
        parameters:
          illumination: normal
    contract:
      id: dnd2024:glossary_bright_light
      inputs:
        location: location
      outputs:
      - illumination
      events: []
      capabilities:
      - visibility
    requires:
    - dnd2024:glossary_bright_light
    citations:
    - SRD 5.2.1 pp. 178; builtin/srd-5.2.1/glossary/bright_light.md
  dnd2024:glossary/burning_hazard:
    source_sha256: 6e6198c3cc210decca2f66bf4897c1a8418d8c2c4c895ad25072de9441c33f14
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/burning_hazard
        implementation: dnd2024:glossary_burning_hazard
        parameters:
          eligible_targets:
          - creature
          - object
          damage:
            dice: 1d4
            type: fire
            timing: subject_turn_start
          self_extinguish:
            cost: action
            apply_condition: prone
            required_activity: roll_on_ground
            ends_burning: true
          also_ends_when:
          - doused
          - submerged
          - suffocated
    contract:
      id: dnd2024:glossary_burning_hazard
      inputs:
        subject: object
        extinguish_method:
          type: choice
          required: false
      outputs:
      - damage
      - condition_changes
      - burning_ended
      events:
      - turn.started
      - action.requested
      - environment.changed
      capabilities:
      - hazards
      - damage
      - conditions
      - action_economy
    requires:
    - dnd2024:glossary_burning_hazard
    citations:
    - SRD 5.2.1 pp. 178; builtin/srd-5.2.1/glossary/burning_hazard.md
  dnd2024:glossary/burrow_speed:
    source_sha256: 2cc4e419138d24b46c09bcac6eaab03db104b266b385fbf9bd46da966c544c91
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/burrow_speed
        implementation: dnd2024:glossary_burrow_speed
        parameters:
          movement_mode: burrow
          allowed_materials:
          - sand
          - earth
          - mud
          - ice
          solid_rock:
            allowed: false
            exception: trait_allows_rock_burrowing
          distance_budget: burrow_speed
    contract:
      id: dnd2024:glossary_burrow_speed
      inputs:
        subject: participant
        route: location_list
      outputs:
      - movement_eligibility
      - movement_budget
      events: []
      capabilities:
      - movement
      - geometry
    requires:
    - dnd2024:glossary_burrow_speed
    citations:
    - SRD 5.2.1 pp. 178; builtin/srd-5.2.1/glossary/burrow_speed.md
  dnd2024:glossary/campaign:
    source_sha256: 9ddc2a8e9b421278a7fb7c624e3b1b95d89fdbeac534e27589ca675a4f4abf44
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 178; builtin/srd-5.2.1/glossary/campaign.md
  dnd2024:glossary/cantrip:
    source_sha256: 36bc27d308d64de7ae861bc730d031085557de293b2156061a2d67dbfcc2e358
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/cantrip
        implementation: dnd2024:glossary_cantrip
        parameters:
          spell_level: 0
          spell_slot_expended: false
    contract:
      id: dnd2024:glossary_cantrip
      inputs:
        caster: participant
        spell: definition
      outputs:
      - slot_cost
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting
    requires:
    - dnd2024:glossary_cantrip
    citations:
    - SRD 5.2.1 pp. 178; builtin/srd-5.2.1/glossary/cantrip.md
  dnd2024:glossary/carrying_capacity:
    source_sha256: 3d07c4e253841538e9c053487f48e1c564bea1e1efbe831af616656fb6ce0be1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/carrying_capacity
        implementation: dnd2024:glossary_carrying_capacity
        parameters:
          weight_unit: pound
          carry_strength_multipliers:
            tiny: 7.5
            small: 15
            medium: 15
            large: 30
            huge: 60
            gargantuan: 120
          drag_lift_push_strength_multipliers:
            tiny: 15
            small: 30
            medium: 30
            large: 60
            huge: 120
            gargantuan: 240
          over_carry_capacity:
            modes:
            - drag
            - lift
            - push
            speed_cap_feet: 5
    contract:
      id: dnd2024:glossary_carrying_capacity
      inputs:
        subject: participant
        weight_pounds: number
        mode: choice
      outputs:
      - maximum_weight
      - speed_cap
      events: []
      capabilities:
      - inventory
      - movement
    requires:
    - dnd2024:glossary_carrying_capacity
    citations:
    - SRD 5.2.1 pp. 178; builtin/srd-5.2.1/glossary/carrying_capacity.md
  dnd2024:glossary/challenge_rating:
    source_sha256: cc0c11ee7c0ec528ed1af21026054048a99ceb37480245fee04fe6104acac1b3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/challenge_rating
        implementation: dnd2024:glossary_challenge_rating
        parameters:
          reference_party_size: 4
          comparison_to_character_level:
            greater: likely_danger
            less: likely_little_threat
          estimate_modifiers:
          - circumstances
          - party_size
          estimate_is_binding: false
    contract:
      id: dnd2024:glossary_challenge_rating
      inputs:
        monster: participant
        character_level: integer
        party_size: integer
      outputs:
      - threat_estimate
      events: []
      capabilities:
      - encounter_planning
    requires:
    - dnd2024:glossary_challenge_rating
    citations:
    - SRD 5.2.1 pp. 178; builtin/srd-5.2.1/glossary/challenge_rating.md
  dnd2024:glossary/character_sheet:
    source_sha256: 80fd642fa5ace1dfea95f7fde1f17ea6bc4ef40dd6b15e9c26416e3ee50084fa
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 178; builtin/srd-5.2.1/glossary/character_sheet.md
  dnd2024:glossary/charmed_condition:
    source_sha256: cece6b27f7f0b6eca611083a408ff8f55b5d6e0e914f53d25a421f88f962035d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/charmed_condition
        implementation: dnd2024:glossary_charmed_condition
        parameters:
          condition: charmed
          forbidden_against_charmer:
          - attack
          - target_with_damaging_ability
          - target_with_damaging_magical_effect
          charmer_social_ability_checks: advantage
    contract:
      id: dnd2024:glossary_charmed_condition
      inputs:
        subject: participant
        charmer: participant
      outputs:
      - condition_effects
      events:
      - attack.pending
      - effect.targeting_pending
      - ability_check.pending
      capabilities:
      - conditions
      - relationships
      - targeting
      - checks
    requires:
    - dnd2024:glossary_charmed_condition
    citations:
    - SRD 5.2.1 pp. 178; builtin/srd-5.2.1/glossary/charmed_condition.md
  dnd2024:glossary/climb_speed:
    source_sha256: 0ec438a9fc566d36668ad7500216e00dc36f2502e075c4283af9cd0dc1742f5c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/climb_speed
        implementation: dnd2024:glossary_climb_speed
        parameters:
          movement_mode: climb
          replaces_speed: true
          ignores_mode_extra_movement_cost: true
    contract:
      id: dnd2024:glossary_climb_speed
      inputs:
        subject: participant
        distance_feet: number
      outputs:
      - movement_cost
      events: []
      capabilities:
      - movement
    requires:
    - dnd2024:glossary_climb_speed
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/climb_speed.md
  dnd2024:glossary/climbing:
    source_sha256: 437edfd3c8b48bac0133f06d33cc247e947cd640e906cb7f420a4aa6026ccb9d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/climbing
        implementation: dnd2024:glossary_climbing
        parameters:
          movement_mode: climb
          per_foot_cost:
            normal: 2
            difficult_terrain: 3
          mode_extra_cost_ignored_if:
            has_special_speed: climb
            uses_special_speed: true
          gm_optional_check:
            terrain_any:
            - slippery_surface
            - few_handholds
            required:
              chooser: gm
              options:
              - true
              - false
            ability: strength
            skill: athletics
            dc: 15
            success_required_to_move: true
    contract:
      id: dnd2024:glossary_climbing
      inputs:
        subject: participant
        distance_feet: number
        terrain: choice
        use_special_speed: boolean
        gm_requires_check: boolean
      outputs:
      - movement_cost
      - check_requirement
      events:
      - movement.pending
      capabilities:
      - movement
      - checks
      - gm_choices
    requires:
    - dnd2024:glossary_climbing
    citations:
    - SRD 5.2.1 pp. 178, 179; builtin/srd-5.2.1/glossary/climbing.md
  dnd2024:glossary/concentration:
    source_sha256: 46a6661ed1de803944f8e221f230962487e42dd1032267f03f65035915ee9048
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/concentration
        implementation: dnd2024:glossary_concentration
        parameters:
          maximum_simultaneous_effects: 1
          effect_ends_on_concentration_loss: true
          maximum_duration: effect_specified
          voluntary_end:
            timing: any_time
            cost: none
          breaks_when:
          - event: spell.cast_started
            requires_concentration: true
          - event: effect.activated
            requires_concentration: true
          - event: condition.applied
            condition: incapacitated
          - event: creature.died
          damage_save:
            ability: constitution
            dc:
              op: min
              args:
              - 30
              - op: max
                args:
                - 10
                - op: floor
                  args:
                  - op: divide
                    args:
                    - var: damage_taken
                    - 2
            failure: end_concentration
    contract:
      id: dnd2024:glossary_concentration
      inputs:
        subject: participant
        effect: effect
        damage_taken:
          type: integer
          required: false
      outputs:
      - concentration_state
      - ended_effects
      - save
      events:
      - spell.cast_started
      - effect.activated
      - damage.taken
      - condition.applied
      - creature.died
      - concentration.ended
      capabilities:
      - concentration
      - effects
      - checks
    requires:
    - dnd2024:glossary_concentration
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/concentration.md
  dnd2024:glossary/condition:
    source_sha256: bd5d47d4347befaba923ededb6e8bcadf99d8caaacd6015ca23d87e5be2ec0b0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/condition
        implementation: dnd2024:glossary_condition
        parameters:
          members:
          - key: dnd2024:glossary/blinded_condition
          - key: dnd2024:glossary/charmed_condition
          - key: dnd2024:glossary/deafened_condition
          - key: dnd2024:glossary/exhaustion_condition
          - key: dnd2024:glossary/frightened_condition
          - key: dnd2024:glossary/grappled_condition
          - key: dnd2024:glossary/incapacitated_condition
          - key: dnd2024:glossary/invisible_condition
          - key: dnd2024:glossary/paralyzed_condition
          - key: dnd2024:glossary/petrified_condition
          - key: dnd2024:glossary/poisoned_condition
          - key: dnd2024:glossary/prone_condition
          - key: dnd2024:glossary/restrained_condition
          - key: dnd2024:glossary/stunned_condition
          - key: dnd2024:glossary/unconscious_condition
          default_stacking: false
          stacking_exception: exhaustion
          duration_and_end: source_effect_defined
    contract:
      id: dnd2024:glossary_condition
      inputs:
        subject: participant
        condition: definition
      outputs:
      - condition_state
      events:
      - condition.applied
      - condition.ended
      capabilities:
      - conditions
    requires:
    - dnd2024:glossary_condition
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/condition.md
  dnd2024:glossary/cone_area_of_effect:
    source_sha256: 673748ab4c1f548f56b8bf737f15aa122766a7cb8a6de29ebf8e5557786d714b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/cone_area_of_effect
        implementation: dnd2024:glossary_cone_area_of_effect
        parameters:
          shape: cone
          propagation: straight_lines
          base_area_rule:
            key: dnd2024:glossary/area_of_effect
          origin: point
          direction:
            chooser: creator
          width_at_distance:
            var: distance_from_origin
          length: effect_specified
          origin_included:
            chooser: creator
            options:
            - false
            - true
            default: false
          geometry_provider: external_geometry
    contract:
      id: dnd2024:glossary_cone_area_of_effect
      inputs:
        creator: participant
        origin: location
        dimensions: choice
        geometry: resolution
        include_origin:
          type: boolean
          required: false
      outputs:
      - area
      - included_locations
      events: []
      capabilities:
      - areas
      - geometry
    requires:
    - dnd2024:glossary_cone_area_of_effect
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/cone_area_of_effect.md
  dnd2024:glossary/cover:
    source_sha256: c6cf5ade8bb0e10ecf9ad9be2ce27def18364a960262e0514fc5f682e7fc24f0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/cover
        implementation: dnd2024:glossary_cover
        parameters:
          degrees:
            half:
              ac_bonus: 2
              dexterity_save_bonus: 2
            three_quarters:
              ac_bonus: 5
              dexterity_save_bonus: 5
            total:
              direct_targeting_allowed: false
          combining_degrees: most_protective_only
    contract:
      id: dnd2024:glossary_cover
      inputs:
        subject: object
        cover: choice
      outputs:
      - cover_benefits
      events:
      - attack.pending
      - saving_throw.pending
      - effect.targeting_pending
      capabilities:
      - targeting
      - checks
      - geometry
    requires:
    - dnd2024:glossary_cover
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/cover.md
  dnd2024:glossary/crawling:
    source_sha256: e923817b8306ba40693412498f1894198abb6dd27fed3d223c3af4d3c92105fd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/crawling
        implementation: dnd2024:glossary_crawling
        parameters:
          movement_mode: crawl
          per_foot_cost:
            normal: 2
            difficult_terrain: 3
    contract:
      id: dnd2024:glossary_crawling
      inputs:
        subject: participant
        distance_feet: number
        difficult_terrain: boolean
      outputs:
      - movement_cost
      events: []
      capabilities:
      - movement
    requires:
    - dnd2024:glossary_crawling
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/crawling.md
  dnd2024:glossary/creature:
    source_sha256: 8b199d89cc2e86a8c377a2c99749200eb2552767b6f0057a600c7f4da538136d
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/creature.md
  dnd2024:glossary/creature_type:
    source_sha256: 0193fe8d8b3f97fa58f406d234775a40cf638dd025644a2edd5f962f70bc6053
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/creature_type
        implementation: dnd2024:glossary_creature_type
        parameters:
          types:
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
          intrinsic_type_rules: false
          all_creatures_have_type: true
          type_specific_effects: effect_defined
    contract:
      id: dnd2024:glossary_creature_type
      inputs:
        subject: participant
        type: choice
      outputs:
      - creature_type
      events: []
      capabilities:
      - statistics
      - targeting
    requires:
    - dnd2024:glossary_creature_type
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/creature_type.md
  dnd2024:glossary/critical_hit:
    source_sha256: 97b84bbd1080fc36fd1d5646daf45539568c02d32882c05178237a9dbc55c3c3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/critical_hit
        implementation: dnd2024:glossary_critical_hit
        parameters:
          natural_attack_die_trigger: 20
          automatic_hit: true
          ignore_ac_and_modifiers_for_hit: true
          damage_dice_multiplier: 2
          includes_all_attack_damage_dice: true
          damage_modifiers_multiplier: 1
    contract:
      id: dnd2024:glossary_critical_hit
      inputs:
        subject: participant
        attack: resolution
      outputs:
      - hit
      - critical
      - damage_expression
      events:
      - attack.rolled
      - damage.roll_pending
      capabilities:
      - attacks
      - damage
    requires:
    - dnd2024:glossary_critical_hit
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/critical_hit.md
  dnd2024:glossary/cube_area_of_effect:
    source_sha256: 8fc072ccd3db4179c9d9c3bd14fe48308053a817579c3a3165dd9d7e89c80b0e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/cube_area_of_effect
        implementation: dnd2024:glossary_cube_area_of_effect
        parameters:
          shape: cube
          propagation: straight_lines
          base_area_rule:
            key: dnd2024:glossary/area_of_effect
          origin: anywhere_on_face
          side_length: effect_specified
          origin_included:
            chooser: creator
            options:
            - false
            - true
            default: false
          geometry_provider: external_geometry
    contract:
      id: dnd2024:glossary_cube_area_of_effect
      inputs:
        creator: participant
        origin: location
        dimensions: choice
        geometry: resolution
        include_origin:
          type: boolean
          required: false
      outputs:
      - area
      - included_locations
      events: []
      capabilities:
      - areas
      - geometry
    requires:
    - dnd2024:glossary_cube_area_of_effect
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/cube_area_of_effect.md
  dnd2024:glossary/curses:
    source_sha256: 4766d9a8d6547adf0e167fd31ec21f9892fd64df452b9934eb53cb27a22d8128
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/curses
        implementation: dnd2024:glossary_curses
        parameters:
          target_types:
          - creature
          - object
          behavior: granting_effect_defined
          removal_sources:
          - key: dnd2024:spells/remove_curse
          - key: dnd2024:spells/greater_restoration
          other_removal_requires_explicit_curse_ending: true
    contract:
      id: dnd2024:glossary_curses
      inputs:
        subject: object
        curse: effect
        removal_effect:
          type: effect
          required: false
      outputs:
      - curse_state
      events:
      - effect.applied
      - effect.removal_pending
      capabilities:
      - effects
      - curses
    requires:
    - dnd2024:glossary_curses
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/curses.md
  dnd2024:glossary/cylinder_area_of_effect:
    source_sha256: 18ebdb799a3df97e4e0d064d15a3cdecabbf9c1bacf0a2015934648f948a3849
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/cylinder_area_of_effect
        implementation: dnd2024:glossary_cylinder_area_of_effect
        parameters:
          shape: cylinder
          propagation: straight_lines
          base_area_rule:
            key: dnd2024:glossary/area_of_effect
          origin: center_of_top_or_bottom
          origin_face:
            chooser: creator
            options:
            - top
            - bottom
          base_radius: effect_specified
          height: effect_specified
          origin_included: true
          geometry_provider: external_geometry
    contract:
      id: dnd2024:glossary_cylinder_area_of_effect
      inputs:
        creator: participant
        origin: location
        dimensions: choice
        geometry: resolution
        include_origin:
          type: boolean
          required: false
      outputs:
      - area
      - included_locations
      events: []
      capabilities:
      - areas
      - geometry
    requires:
    - dnd2024:glossary_cylinder_area_of_effect
    citations:
    - SRD 5.2.1 pp. 180; builtin/srd-5.2.1/glossary/cylinder_area_of_effect.md
  dnd2024:glossary/d20_test:
    source_sha256: 8dcb7b9d20cc1264cc689dee5368ef6cbdc5095ab19fa6213bb9f6aa90c5a342
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/d20_test
        implementation: dnd2024:glossary_d20_test
        parameters:
          test_kinds:
          - ability_check
          - attack_roll
          - saving_throw
          d20_test_modifiers_apply_to_all_kinds: true
          test_warranted:
            chooser: gm
            options:
            - true
            - false
    contract:
      id: dnd2024:glossary_d20_test
      inputs:
        subject: participant
        test_kind: choice
        gm_requires_test: boolean
      outputs:
      - test_requirement
      events: []
      capabilities:
      - checks
      - gm_choices
    requires:
    - dnd2024:glossary_d20_test
    citations:
    - SRD 5.2.1 pp. 180; builtin/srd-5.2.1/glossary/d20_test.md
  dnd2024:glossary/damage:
    source_sha256: c797fe3292e57fd53edbf7761a0efa0eeffe2845fc767cd2c87d745688fa0d62
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/damage
        implementation: dnd2024:glossary_damage
        parameters:
          targets:
          - creature
          - object
          hit_points_change:
            op: negate
            args:
            - var: damage_amount
    contract:
      id: dnd2024:glossary_damage
      inputs:
        subject: object
        damage_amount: integer
      outputs:
      - hit_points_delta
      events:
      - damage.applied
      capabilities:
      - damage
      - hit_points
    requires:
    - dnd2024:glossary_damage
    citations:
    - SRD 5.2.1 pp. 180; builtin/srd-5.2.1/glossary/damage.md
  dnd2024:glossary/damage_roll:
    source_sha256: 942e230142da3aefa6c5e53e18180bd0d64dd1cd5e4c34a619f48277c8d96a5a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/damage_roll
        implementation: dnd2024:glossary_damage_roll
        parameters:
          amount:
            op: add
            args:
            - var: rolled_total
            - var: applicable_modifiers
          apply_to: target
    contract:
      id: dnd2024:glossary_damage_roll
      inputs:
        target: object
        rolled_total: integer
        applicable_modifiers: integer
      outputs:
      - damage_amount
      events:
      - damage.rolled
      capabilities:
      - damage
    requires:
    - dnd2024:glossary_damage_roll
    citations:
    - SRD 5.2.1 pp. 180; builtin/srd-5.2.1/glossary/damage_roll.md
  dnd2024:glossary/damage_threshold:
    source_sha256: 68373d9e01b1c0074fae12cc9379f28955f564dc3d95796344fb7435e648dd13
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/damage_threshold
        implementation: dnd2024:glossary_damage_threshold
        parameters:
          scope: single_attack_or_effect_instance
          pass_predicate:
            op: greater_or_equal
            args:
            - var: damage_amount
            - var: threshold
          pass_damage:
            var: damage_amount
          fail_damage: 0
          fail_immunity: true
          threshold_is_not_subtracted: true
    contract:
      id: dnd2024:glossary_damage_threshold
      inputs:
        subject: object
        damage_amount: integer
        threshold: integer
      outputs:
      - damage_amount
      events:
      - damage.pending
      capabilities:
      - damage
    requires:
    - dnd2024:glossary_damage_threshold
    citations:
    - SRD 5.2.1 pp. 180; builtin/srd-5.2.1/glossary/damage_threshold.md
  dnd2024:glossary/damage_types:
    source_sha256: b38efaccf3ceee411478a7de9fbd51cdfadf8c15fe7873acf78a264c4aff2608
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/damage_types
        implementation: dnd2024:glossary_damage_types
        parameters:
          types: &id002
          - acid
          - bludgeoning
          - cold
          - fire
          - force
          - lightning
          - necrotic
          - piercing
          - poison
          - psychic
          - radiant
          - slashing
          - thunder
          intrinsic_type_rules: false
          gm_new_effect_type:
            chooser: gm
            options: *id002
          example_sources:
            acid:
            - corrosive_liquid
            - digestive_enzyme
            bludgeoning:
            - blunt_object
            - constriction
            - falling
            cold:
            - freezing_water
            - icy_blast
            fire:
            - flame
            - unbearable_heat
            force:
            - pure_magical_energy
            lightning:
            - electricity
            necrotic:
            - life_draining_energy
            piercing:
            - fang
            - puncturing_object
            poison:
            - toxic_gas
            - venom
            psychic:
            - mind_rending_energy
            radiant:
            - holy_energy
            - searing_radiation
            slashing:
            - claw
            - cutting_object
            thunder:
            - concussive_sound
    contract:
      id: dnd2024:glossary_damage_types
      inputs:
        effect: effect
        type: choice
      outputs:
      - damage_type
      events: []
      capabilities:
      - damage
      - gm_choices
    requires:
    - dnd2024:glossary_damage_types
    citations:
    - SRD 5.2.1 pp. 180; builtin/srd-5.2.1/glossary/damage_types.md
  dnd2024:glossary/darkness:
    source_sha256: 8c8eb17d985b4a844c138d78d1ad6051e6004ae415410aa6cdeaaf12de85c2f3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/darkness
        implementation: dnd2024:glossary_darkness
        parameters:
          illumination: darkness
          obscurity: heavily_obscured
    contract:
      id: dnd2024:glossary_darkness
      inputs:
        location: location
      outputs:
      - obscurity
      events: []
      capabilities:
      - visibility
    requires:
    - dnd2024:glossary_darkness
    citations:
    - SRD 5.2.1 pp. 180; builtin/srd-5.2.1/glossary/darkness.md
  dnd2024:glossary/darkvision:
    source_sha256: d2abb9c89da5449fb84b503ab7242484e562469d4b13af4610a6479587ac351a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/darkvision
        implementation: dnd2024:glossary_darkvision
        parameters:
          range: feature_specified
          within_range_light_mapping:
            dim: bright
            darkness: dim
          darkness_colors: grayscale
    contract:
      id: dnd2024:glossary_darkvision
      inputs:
        subject: participant
        range_feet: number
        target_location: location
      outputs:
      - perceived_illumination
      - color_vision
      events: []
      capabilities:
      - senses
      - visibility
    requires:
    - dnd2024:glossary_darkvision
    citations:
    - SRD 5.2.1 pp. 180; builtin/srd-5.2.1/glossary/darkvision.md
  dnd2024:glossary/dash_action:
    source_sha256: 5e06e0114f71f1b1346a43fce161c328f96dfe3fb398a0981520fbdf418f26d4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/dash_action
        implementation: dnd2024:glossary_dash_action
        parameters:
          cost: action
          duration: current_turn
          extra_movement:
            var: selected_speed_after_modifiers
          speed_selection:
            chooser: subject
            timing: each_dash
            options: all_available_speeds
    contract:
      id: dnd2024:glossary_dash_action
      inputs:
        subject: participant
        selected_speed: choice
      outputs:
      - extra_movement
      events:
      - action.requested
      capabilities:
      - action_economy
      - movement
    requires:
    - dnd2024:glossary_dash_action
    citations:
    - SRD 5.2.1 pp. 180; builtin/srd-5.2.1/glossary/dash_action.md
  dnd2024:glossary/dead:
    source_sha256: 969d1dd08379851ab0f8b25c7a017c8a319bb4c18864ffd748c347a0b510e6dc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/dead
        implementation: dnd2024:glossary_dead
        parameters:
          hit_points: 0
          healing_requires_prior_magical_revival: true
          revival:
            spirit_knows_caster: true
            spirit_can_refuse: true
            requires_spirit_return: true
            hit_points: revival_effect_defined
            ongoing_effects_return:
              kinds:
              - condition
              - magical_contagion
              - curse
              only_if_duration_ongoing: true
              unless_revival_states_otherwise: true
            exhaustion_levels:
              op: max
              args:
              - 0
              - op: subtract
                args:
                - var: death_exhaustion_levels
                - 1
          attunements_ended: true
    contract:
      id: dnd2024:glossary_dead
      inputs:
        subject: participant
        revival_effect:
          type: effect
          required: false
        spirit_accepts:
          type: boolean
          required: false
      outputs:
      - death_state
      - revival_result
      events:
      - creature.died
      - revival.pending
      capabilities:
      - death
      - hit_points
      - effects
      - attunement
    requires:
    - dnd2024:glossary_dead
    citations:
    - SRD 5.2.1 pp. 180; builtin/srd-5.2.1/glossary/dead.md
  dnd2024:glossary/deafened_condition:
    source_sha256: 760d98e765ef15032e77bd34b67f4352881ab625b61caae0fd1784d34ecab7a9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/deafened_condition
        implementation: dnd2024:glossary_deafened_condition
        parameters:
          condition: deafened
          can_hear: false
          automatic_failure:
            check_kind: ability_check
            requires_hearing: true
    contract:
      id: dnd2024:glossary_deafened_condition
      inputs:
        subject: participant
      outputs:
      - condition_effects
      events:
      - condition.changed
      - ability_check.pending
      capabilities:
      - conditions
      - senses
      - checks
    requires:
    - dnd2024:glossary_deafened_condition
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/deafened_condition.md
  dnd2024:glossary/death_saving_throw:
    source_sha256: 088b02d4fbee4f586ef5e0f7c57405263f90d6d5e0528cdc2b0f9735f5b608d7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/death_saving_throw
        implementation: dnd2024:glossary_death_saving_throw
        parameters:
          trigger: own_turn_start
          subject_kind: player_character
          hit_points: 0
          required_test: death_saving_throw
          exception_rule:
            key: dnd2024:glossary/stable
    contract:
      id: dnd2024:glossary_death_saving_throw
      inputs:
        subject: participant
      outputs:
      - death_save_required
      events:
      - turn.started
      capabilities:
      - death
      - checks
    requires:
    - dnd2024:glossary_death_saving_throw
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/death_saving_throw.md
  dnd2024:glossary/dehydration_hazard:
    source_sha256: 3a822cb6203feb7a6ba9c650dd871c65067c12d4ddacd8bc9df350841ae18cde
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/dehydration_hazard
        implementation: dnd2024:glossary_dehydration_hazard
        parameters:
          daily_water_gallons:
            tiny: 0.25
            small: 1
            medium: 1
            large: 4
            huge: 16
            gargantuan: 64
          exhaustion:
            trigger: day_end
            predicate:
              op: less_than
              args:
              - var: water_consumed
              - op: multiply
                args:
                - 0.5
                - var: daily_requirement
            levels: 1
            cause: dehydration
          cause_exhaustion_removal_locked_until:
            water_consumed_gallons_at_least: full_daily_requirement
    contract:
      id: dnd2024:glossary_dehydration_hazard
      inputs:
        subject: participant
        water_consumed: number
      outputs:
      - exhaustion_change
      - recovery_lock
      events:
      - day.ended
      - water.consumed
      - exhaustion.removal_pending
      capabilities:
      - hazards
      - survival
      - conditions
    requires:
    - dnd2024:glossary_dehydration_hazard
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/dehydration_hazard.md
  dnd2024:glossary/difficult_terrain:
    source_sha256: c262525ea7266a57eca463d81eed83e83004121c7cb74d4304c91f8250e6303e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/difficult_terrain
        implementation: dnd2024:glossary_difficult_terrain
        parameters:
          additional_cost_feet_per_foot: 1
          stacks: false
          qualifying_any:
          - creature_present: true
            creature_not_tiny: true
            creature_not_ally: true
          - furniture_size_at_least: traveler_size
          - terrain_any:
            - heavy_snow
            - ice
            - rubble
            - undergrowth
          - liquid_depth_min: shin
            liquid_depth_max: waist
          - narrow_opening_size_difference: -1
          - slope_degrees_min: 20
          similar_terrain:
            chooser: gm
            result_type: boolean
            similar_to_listed_terrain_required: true
    contract:
      id: dnd2024:glossary_difficult_terrain
      inputs:
        subject: participant
        location: location
        terrain_facts: resolution
      outputs:
      - is_difficult_terrain
      - movement_cost
      events: []
      capabilities:
      - movement
      - geometry
      - gm_choices
    requires:
    - dnd2024:glossary_difficult_terrain
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/difficult_terrain.md
  dnd2024:glossary/difficulty_class:
    source_sha256: fb7f616f45a09993db7bec7f7ba30081ccc73c324fcb5bafafac8f4252012df0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/difficulty_class
        implementation: dnd2024:glossary_difficulty_class
        parameters:
          target_number_for:
          - ability_check
          - saving_throw
    contract:
      id: dnd2024:glossary_difficulty_class
      inputs:
        dc: integer
        test_kind: choice
      outputs:
      - target_number
      events: []
      capabilities:
      - checks
    requires:
    - dnd2024:glossary_difficulty_class
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/difficulty_class.md
  dnd2024:glossary/dim_light:
    source_sha256: bbb9ba696f68098c7f89c426bf9886ae748e98e1f081f14344c6eb96dd9c1f57
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/dim_light
        implementation: dnd2024:glossary_dim_light
        parameters:
          illumination: dim
          obscurity: lightly_obscured
    contract:
      id: dnd2024:glossary_dim_light
      inputs:
        location: location
      outputs:
      - obscurity
      events: []
      capabilities:
      - visibility
    requires:
    - dnd2024:glossary_dim_light
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/dim_light.md
  dnd2024:glossary/disadvantage:
    source_sha256: b2a33c3781029d22f0f53bb0e59a9c02846a370d69b23aa0fcfc7dd251a03275
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/disadvantage
        implementation: dnd2024:glossary_disadvantage
        parameters:
          test_kinds:
          - ability_check
          - attack_roll
          - saving_throw
          dice:
            count: 2
            sides: 20
            select: lowest
          same_kind_stacking: false
          opposite_kind_cancellation:
            any_advantage_and_any_disadvantage: true
            result:
              count: 1
              sides: 20
            ignore_source_counts: true
    contract:
      id: dnd2024:glossary_disadvantage
      inputs:
        subject: participant
        test: resolution
      outputs:
      - roll_mode
      events:
      - d20_test.pending
      capabilities:
      - checks
    requires:
    - dnd2024:glossary_disadvantage
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/disadvantage.md
  dnd2024:glossary/disengage_action:
    source_sha256: a6f09110e29d3c16e7455cd2101a776b5aacb68fcf221119502480b608be8852
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/disengage_action
        implementation: dnd2024:glossary_disengage_action
        parameters:
          cost: action
          duration: remainder_of_current_turn
          movement_provokes_opportunity_attacks: false
    contract:
      id: dnd2024:glossary_disengage_action
      inputs:
        subject: participant
      outputs:
      - opportunity_attack_exemption
      events:
      - action.requested
      - movement.pending
      capabilities:
      - action_economy
      - movement
      - attacks
    requires:
    - dnd2024:glossary_disengage_action
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/disengage_action.md
  dnd2024:glossary/dodge_action:
    source_sha256: 2b65dec7fbd1bb8b7c5a48380b2f1fc435c41e59cd88806d51ef182c4adf9817
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/dodge_action
        implementation: dnd2024:glossary_dodge_action
        parameters:
          cost: action
          ends: own_next_turn_start
          incoming_attacks:
            disadvantage: true
            requires_subject_sees_attacker: true
          dexterity_saves: advantage
          lose_benefits_when_any:
          - incapacitated
          - speed_zero
    contract:
      id: dnd2024:glossary_dodge_action
      inputs:
        subject: participant
      outputs:
      - dodge_effect
      events:
      - action.requested
      - attack.pending
      - saving_throw.pending
      - condition.changed
      - speed.changed
      - turn.started
      capabilities:
      - action_economy
      - effects
      - checks
      - attacks
    requires:
    - dnd2024:glossary_dodge_action
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/dodge_action.md
  dnd2024:glossary/emanation_area_of_effect:
    source_sha256: 67d33eef635df651b879ac7681e6864f2c19e958391544f68be2dbceada861c8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/emanation_area_of_effect
        implementation: dnd2024:glossary_emanation_area_of_effect
        parameters:
          shape: emanation
          propagation: straight_lines
          base_area_rule:
            key: dnd2024:glossary/area_of_effect
          origin_types:
          - creature
          - object
          direction: all
          distance: effect_specified
          moves_with_origin: true
          movement_exceptions:
          - instantaneous
          - stationary
          origin_included:
            chooser: creator
            options:
            - false
            - true
            default: false
          geometry_provider: external_geometry
    contract:
      id: dnd2024:glossary_emanation_area_of_effect
      inputs:
        creator: participant
        origin: location
        dimensions: choice
        geometry: resolution
        include_origin:
          type: boolean
          required: false
      outputs:
      - area
      - included_locations
      events: []
      capabilities:
      - areas
      - geometry
    requires:
    - dnd2024:glossary_emanation_area_of_effect
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/emanation_area_of_effect.md
  dnd2024:glossary/encounter:
    source_sha256: 1b928f2c6a1b303eb1a6b9ce11e8a3d41a94018e9f0cc5db4e67c9f1595c5d0c
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/encounter.md
  dnd2024:glossary/enemy:
    source_sha256: 00d57319d25a63a00b52c1a64966307ca6bcd9d7945c4313c6721bf1430b0f3b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/enemy
        implementation: dnd2024:glossary_enemy
        parameters:
          predicate:
            any:
            - fights_against_subject
            - actively_works_to_harm_subject
            - rule_designated_enemy
            - gm_designated_enemy
          gm_designation:
            chooser: gm
            options:
            - true
            - false
    contract:
      id: dnd2024:glossary_enemy
      inputs:
        subject: participant
        other: participant
        relationship: choice
      outputs:
      - is_enemy
      events: []
      capabilities:
      - relationships
    requires:
    - dnd2024:glossary_enemy
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/enemy.md
  dnd2024:glossary/exhaustion_condition:
    source_sha256: a66493fa857fab2741a780008d3ba95e6abc07e9c634d44b19d0658a36565687
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/exhaustion_condition
        implementation: dnd2024:glossary_exhaustion_condition
        parameters:
          condition: exhaustion
          stacking: true
          levels_per_application: 1
          death_at_level: 6
          d20_modifier:
            op: multiply
            args:
            - -2
            - var: exhaustion_level
          speed_delta_feet:
            op: multiply
            args:
            - -5
            - var: exhaustion_level
          long_rest_levels_removed: 1
          ends_at_level: 0
    contract:
      id: dnd2024:glossary_exhaustion_condition
      inputs:
        subject: participant
      outputs:
      - exhaustion_level
      - modifiers
      - death
      events:
      - condition.applied
      - d20_test.pending
      - speed.calculated
      - long_rest.completed
      capabilities:
      - conditions
      - checks
      - movement
      - rests
      - death
    requires:
    - dnd2024:glossary_exhaustion_condition
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/exhaustion_condition.md
  dnd2024:glossary/experience_points:
    source_sha256: 7227ecc9415c9831a68de1538d4bb278bb6f65d7c5db098efdebd9c024361949
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/experience_points
        implementation: dnd2024:glossary_experience_points
        parameters:
          award:
            chooser: gm
            amount_type: integer
            triggers:
            - challenge_overcome
            - adventure_completed
          accumulation: add_to_total
          level_increase: level_advancement_threshold_reached
    contract:
      id: dnd2024:glossary_experience_points
      inputs:
        subject: participant
        xp_award: integer
        advancement_table: definition
      outputs:
      - xp_total
      - level
      events:
      - experience.awarded
      capabilities:
      - advancement
      - gm_choices
    requires:
    - dnd2024:glossary_experience_points
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/experience_points.md
  dnd2024:glossary/expertise:
    source_sha256: fdc8f060b724be4b69eb03ba3f870e50f201b465fb7556a1ce3bec09fe430f08
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/expertise
        implementation: dnd2024:glossary_expertise
        parameters:
          selection:
            skill_count: 1
            requires_skill_proficiency: true
            existing_expertise_disallowed: true
          proficiency_bonus_multiplier: 2
          scope: ability_check_with_selected_skill
          stacks_with_other_doubling: false
          same_skill_multiple_expertise: false
    contract:
      id: dnd2024:glossary_expertise
      inputs:
        subject: participant
        skill: definition
      outputs:
      - expertise_selection
      - check_modifier
      events:
      - expertise.granted
      - ability_check.pending
      capabilities:
      - proficiency
      - checks
    requires:
    - dnd2024:glossary_expertise
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/expertise.md
  dnd2024:glossary/falling_hazard:
    source_sha256: dff59fd575acc80843d188c4f7a4e0bfb8928feedbc9a1e40a79d384ea269bf7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/falling_hazard
        implementation: dnd2024:glossary_falling_hazard
        parameters:
          damage:
            timing: fall_end
            type: bludgeoning
            die_sides: 6
            dice_count:
              op: min
              args:
              - 20
              - op: floor
                args:
                - op: divide
                  args:
                  - var: fall_distance_feet
                  - 10
          landing_condition:
            condition: prone
            unless_no_fall_damage: true
          liquid_landing:
            eligible_liquids: any
            optional: true
            cost: reaction
            check:
              chooser: subject
              options:
              - ability: strength
                skill: athletics
              - ability: dexterity
                skill: acrobatics
              dc: 15
            success:
              entry_orientation:
              - head_first
              - feet_first
              fall_damage_multiplier: 0.5
    contract:
      id: dnd2024:glossary_falling_hazard
      inputs:
        subject: participant
        fall_distance_feet: number
        landing_in_liquid: boolean
        liquid_reaction:
          type: choice
          required: false
      outputs:
      - damage
      - condition_changes
      - reaction_check
      events:
      - fall.ended
      capabilities:
      - hazards
      - damage
      - checks
      - movement
    requires:
    - dnd2024:glossary_falling_hazard
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/falling_hazard.md
  dnd2024:glossary/fly_speed:
    source_sha256: 0e9fa8727aa2512b31247982ea9e68dffc5906289dc95d35fc4c3ab41aa0d9f9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/fly_speed
        implementation: dnd2024:glossary_fly_speed
        parameters:
          movement_mode: fly
          medium: air
          stay_aloft_until_any:
          - land
          - fall
          - die
    contract:
      id: dnd2024:glossary_fly_speed
      inputs:
        subject: participant
        distance_feet: number
      outputs:
      - flight_budget
      - aloft_state
      events: []
      capabilities:
      - movement
    requires:
    - dnd2024:glossary_fly_speed
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/fly_speed.md
  dnd2024:glossary/flying:
    source_sha256: 67c586c051c94a1a153f5b1a2442d6af474a4506b584157c639ae7ff76d27bdc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/flying
        implementation: dnd2024:glossary_flying
        parameters:
          fall_when_any:
          - incapacitated
          - prone
          - fly_speed_zero
          hover_prevents_these_falls: true
    contract:
      id: dnd2024:glossary_flying
      inputs:
        subject: participant
      outputs:
      - fall_required
      events:
      - condition.changed
      - speed.changed
      capabilities:
      - movement
      - conditions
    requires:
    - dnd2024:glossary_flying
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/flying.md
  dnd2024:glossary/friendly_attitude:
    source_sha256: 672358daf4a84ea5083cff49488335dc0015b7822a49c1c47b942f84d971d573
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/friendly_attitude
        implementation: dnd2024:glossary_friendly_attitude
        parameters:
          attitude: friendly
          influence_ability_check_mode: advantage
    contract:
      id: dnd2024:glossary_friendly_attitude
      inputs:
        subject: participant
        target: participant
      outputs:
      - roll_mode
      events:
      - ability_check.pending
      capabilities:
      - relationships
      - checks
    requires:
    - dnd2024:glossary_friendly_attitude
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/friendly_attitude.md
  dnd2024:glossary/frightened_condition:
    source_sha256: 45f0f76363c298d106cfa83589abe23d6d7cbe0c345fd0a094ff18b445d6ea5e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/frightened_condition
        implementation: dnd2024:glossary_frightened_condition
        parameters:
          condition: frightened
          source_in_line_of_sight:
            ability_checks: disadvantage
            attack_rolls: disadvantage
          willing_movement_closer_to_source_allowed: false
    contract:
      id: dnd2024:glossary_frightened_condition
      inputs:
        subject: participant
        fear_source: object
        geometry: resolution
      outputs:
      - condition_effects
      events:
      - ability_check.pending
      - attack.pending
      - movement.pending
      capabilities:
      - conditions
      - checks
      - movement
    requires:
    - dnd2024:glossary_frightened_condition
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/frightened_condition.md
  dnd2024:glossary/grappled_condition:
    source_sha256: 4928ca79335c2e6f4e4d5e4bb83f0ccf48e00263c6c94d0a132aa140c44772e6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/grappled_condition
        implementation: dnd2024:glossary_grappled_condition
        parameters:
          condition: grappled
          speed: 0
          speed_can_increase: false
          attack_disadvantage_unless_target_is_grappler: true
          grappler_moves_subject:
            allowed_modes:
            - drag
            - carry
            extra_cost_per_foot: 1
            extra_cost_exceptions:
            - target_size: tiny
            - target_size_at_least_categories_smaller: 2
    contract:
      id: dnd2024:glossary_grappled_condition
      inputs:
        subject: participant
        grappler: participant
      outputs:
      - condition_effects
      - grappler_movement_cost
      events:
      - attack.pending
      - movement.pending
      - speed.calculated
      capabilities:
      - conditions
      - movement
      - attacks
    requires:
    - dnd2024:glossary_grappled_condition
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/grappled_condition.md
  dnd2024:glossary/grappling:
    source_sha256: b69f0dbca73a8aeb4e8f50d33a5c732ca5564fc09f9ad3c6f862f023a1da19a3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/grappling
        implementation: dnd2024:glossary_grappling
        parameters:
          successful_grapple_condition: grappled
          body_part:
            default: free_hand
            alternatives_require_explicit_rule: true
            targets_per_part: 1
            cannot_target_other_creatures_while_grappling: true
          escape:
            cost: action
            check_choice:
              chooser: subject
              options:
              - ability: strength
                skill: athletics
              - ability: dexterity
                skill: acrobatics
            dc: grapple_escape_dc
            success: end_grappled
          also_ends_when_any:
          - grappler_incapacitated
          - distance_exceeds_grapple_range
          release:
            chooser: grappler
            timing: any_time
            cost: none
    contract:
      id: dnd2024:glossary_grappling
      inputs:
        subject: participant
        grappler: participant
        grapple: effect
        escape_choice:
          type: choice
          required: false
      outputs:
      - grapple_state
      - escape_check
      - body_part_reservation
      events:
      - grapple.started
      - action.requested
      - condition.changed
      - movement.completed
      - grapple.released
      capabilities:
      - grappling
      - conditions
      - checks
      - movement
    requires:
    - dnd2024:glossary_grappling
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/grappling.md
  dnd2024:glossary/hazard:
    source_sha256: 170808bb6ed915c4cd23dabb7c6fc6649a99a633f883be7ee8bdbd727bf77717
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/hazard.md
  dnd2024:glossary/healing:
    source_sha256: 207a2f3fb3a7f3d50869bc47729f52c13c4c274129b79f756b747ed38600479a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/healing
        implementation: dnd2024:glossary_healing
        parameters:
          hit_points_change:
            var: healing_amount
          base_bounds:
            key: dnd2024:glossary/hit_points
    contract:
      id: dnd2024:glossary_healing
      inputs:
        subject: participant
        healing_amount: integer
      outputs:
      - hit_points_delta
      events:
      - healing.applied
      capabilities:
      - hit_points
    requires:
    - dnd2024:glossary_healing
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/healing.md
  dnd2024:glossary/heavily_obscured:
    source_sha256: a072d2eaaa11f8cd2dee115d29fdaf434c93c60ed27cfe3444ce942bfeca6552
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/heavily_obscured
        implementation: dnd2024:glossary_heavily_obscured
        parameters:
          when: trying_to_see_something_in_heavily_obscured_space
          effective_condition: blinded
    contract:
      id: dnd2024:glossary_heavily_obscured
      inputs:
        subject: participant
        target_location: location
      outputs:
      - vision_effect
      events: []
      capabilities:
      - visibility
      - conditions
    requires:
    - dnd2024:glossary_heavily_obscured
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/heavily_obscured.md
  dnd2024:glossary/help_action:
    source_sha256: 362af06341f33db90099b53a7ec1938d88b76013d4155b0795a14126ea7287bd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/help_action
        implementation: dnd2024:glossary_help_action
        parameters:
          cost: action
          select_one:
            ability_check:
              choose_own_proficiency_kind:
              - skill
              - tool
              choose_one_ally: true
              proximity:
                verbal_or_physical_assistance_possible: true
                final_decision: gm
              advantage: next_ally_check_using_chosen_proficiency
              expires: own_next_turn_start
              consumed_after_matching_check: true
            attack_roll:
              enemy_distance_feet_max: 5
              advantage: next_attack_by_any_one_ally_against_enemy
              expires: own_next_turn_start
              consumed_after_matching_attack: true
    contract:
      id: dnd2024:glossary_help_action
      inputs:
        subject: participant
        mode: choice
        target: participant
        proficiency:
          type: definition
          required: false
        assistance_possible:
          type: boolean
          required: false
      outputs:
      - help_effect
      events:
      - action.requested
      - ability_check.pending
      - attack.pending
      - turn.started
      capabilities:
      - action_economy
      - checks
      - attacks
      - gm_choices
    requires:
    - dnd2024:glossary_help_action
    citations:
    - SRD 5.2.1 pp. 182, 183; builtin/srd-5.2.1/glossary/help_action.md
  dnd2024:glossary/heroic_inspiration:
    source_sha256: 1c3d78c6a87b44a5cda74b9a8fa23ffbc3077db9b0897fd06f072a69c909d6da
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/heroic_inspiration
        implementation: dnd2024:glossary_heroic_inspiration
        parameters:
          eligible: player_character
          capacity: 1
          expend:
            timing: immediately_after_rolling_any_die
            reroll_count: 1
            must_use_new_result: true
          gain_while_full:
            default: lost
            optional_transfer:
              recipient: player_character_without_heroic_inspiration
              chooser: subject
    contract:
      id: dnd2024:glossary_heroic_inspiration
      inputs:
        subject: participant
        die_result:
          type: resolution
          required: false
        transfer_target:
          type: participant
          required: false
      outputs:
      - reroll
      - inspiration_state
      events:
      - die.rolled
      - heroic_inspiration.gained
      capabilities:
      - dice
      - resources
    requires:
    - dnd2024:glossary_heroic_inspiration
    citations:
    - SRD 5.2.1 pp. 183; builtin/srd-5.2.1/glossary/heroic_inspiration.md
  dnd2024:glossary/hide_action:
    source_sha256: 5c7e389d4e0e37676a04637b372b885a18243c2653a94fdd37e6be26d45696a6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/hide_action
        implementation: dnd2024:glossary_hide_action
        parameters:
          cost: action
          prerequisites:
            any:
            - heavily_obscured
            - three_quarters_cover
            - total_cover
            outside_all_enemy_lines_of_sight: true
          visibility_information:
            can_see_creature: true
            can_discern_if_creature_sees_subject: true
          check:
            ability: dexterity
            skill: stealth
            dc: 15
          success:
            hidden: true
            condition: invisible
            find_check:
              ability: wisdom
              skill: perception
              dc:
                var: hide_check_total
          ends_immediately_after_any:
          - sound_louder_than_whisper
          - enemy_finds_subject
          - subject_makes_attack_roll
          - subject_casts_verbal_component_spell
    contract:
      id: dnd2024:glossary_hide_action
      inputs:
        subject: participant
        geometry: resolution
      outputs:
      - hide_check
      - hidden_effect
      - find_dc
      events:
      - action.requested
      - sound.made
      - hidden_creature.found
      - attack.rolled
      - spell.cast
      capabilities:
      - action_economy
      - stealth
      - visibility
      - checks
      - conditions
    requires:
    - dnd2024:glossary_hide_action
    citations:
    - SRD 5.2.1 pp. 183; builtin/srd-5.2.1/glossary/hide_action.md
  dnd2024:glossary/high_jump:
    source_sha256: 952787502ec9f5a42ddd1db8276eaf220710aa0a06e5643a5246d71e27e5b09d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/high_jump
        implementation: dnd2024:glossary_high_jump
        parameters:
          direction: vertical
          running_start:
            minimum_feet: 10
            on_foot: true
            immediately_before: true
          running_height_feet:
            op: max
            args:
            - 0
            - op: add
              args:
              - 3
              - var: strength_modifier
          standing_height_multiplier: 0.5
          movement_cost_per_foot: 1
          arms_extension_height_multiplier: 0.5
          maximum_reach:
            op: add
            args:
            - var: jump_height_feet
            - op: multiply
              args:
              - 1.5
              - var: creature_height_feet
    contract:
      id: dnd2024:glossary_high_jump
      inputs:
        subject: participant
        run_up_feet: number
        on_foot: boolean
        creature_height_feet: number
      outputs:
      - jump_height
      - movement_cost
      - maximum_reach
      events: []
      capabilities:
      - movement
    requires:
    - dnd2024:glossary_high_jump
    citations:
    - SRD 5.2.1 pp. 183; builtin/srd-5.2.1/glossary/high_jump.md
  dnd2024:glossary/hit_point_dice:
    source_sha256: f9ada18f1f823ebc575c3df88ef3e3b183f9a64ab9bd7c7416e1e5ebbd47bed9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/hit_point_dice
        implementation: dnd2024:glossary_hit_point_dice
        parameters:
          player_character_hp_maximum_uses_hit_dice: true
          short_rest_spending_allowed: true
          spending_rule:
            key: dnd2024:glossary/short_rest
    contract:
      id: dnd2024:glossary_hit_point_dice
      inputs:
        subject: participant
        hit_dice: choice
      outputs:
      - hit_dice_resource
      events: []
      capabilities:
      - hit_points
      - rests
    requires:
    - dnd2024:glossary_hit_point_dice
    citations:
    - SRD 5.2.1 pp. 183; builtin/srd-5.2.1/glossary/hit_point_dice.md
  dnd2024:glossary/hit_points:
    source_sha256: 9a8873da8583d1bbb47a3bb414f47450a093b2659c722c51b3ad96bc500ab8bc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/hit_points
        implementation: dnd2024:glossary_hit_points
        parameters:
          eligible:
          - creature
          - object
          minimum: 0
          maximum:
            var: hit_point_maximum
          damage_operation: subtract
          healing_operation: add
    contract:
      id: dnd2024:glossary_hit_points
      inputs:
        subject: object
        change: integer
      outputs:
      - clamped_hit_points
      events:
      - hit_points.changed
      capabilities:
      - hit_points
    requires:
    - dnd2024:glossary_hit_points
    citations:
    - SRD 5.2.1 pp. 183; builtin/srd-5.2.1/glossary/hit_points.md
  dnd2024:glossary/hostile_attitude:
    source_sha256: c72d4bd71b4b3b3c1c4fa1a1cb08c35d159661fa27db40870a1b1cd438588a8a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/hostile_attitude
        implementation: dnd2024:glossary_hostile_attitude
        parameters:
          attitude: hostile
          influence_ability_check_mode: disadvantage
    contract:
      id: dnd2024:glossary_hostile_attitude
      inputs:
        subject: participant
        target: participant
      outputs:
      - roll_mode
      events:
      - ability_check.pending
      capabilities:
      - relationships
      - checks
    requires:
    - dnd2024:glossary_hostile_attitude
    citations:
    - SRD 5.2.1 pp. 183; builtin/srd-5.2.1/glossary/hostile_attitude.md
  dnd2024:glossary/hover:
    source_sha256: e9ace69721b54b56a0bf560ea4fc2bdc5df6e1c8428f519b5a1173c38c89e2a3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/hover
        implementation: dnd2024:glossary_hover
        parameters:
          grant_sources:
          - stat_block
          - spell
          - other_effect
          while_flying_prevents_fall_from:
          - incapacitated
          - prone
          - fly_speed_zero
          base_rule:
            key: dnd2024:glossary/flying
    contract:
      id: dnd2024:glossary_hover
      inputs:
        subject: participant
      outputs:
      - fall_prevention
      events:
      - condition.changed
      - speed.changed
      capabilities:
      - movement
    requires:
    - dnd2024:glossary_hover
    citations:
    - SRD 5.2.1 pp. 183; builtin/srd-5.2.1/glossary/hover.md
  dnd2024:glossary/illusions:
    source_sha256: 84fb0fcd38aa9078c79d9644554a1b6b6bebaea39ae297f76064d6a884aaab16
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/illusions
        implementation: dnd2024:glossary_illusions
        parameters:
          deceived_senses_and_faculties: source_effect_defined
          spatial_manifestation:
            substantial: false
            weight: 0
            environment_response_appearance: as_if_real
            default_visual_responses:
            - shadows
            - reflections
            - wind
            default_audible_responses:
            - echo
            source_effect_can_override: true
    contract:
      id: dnd2024:glossary_illusions
      inputs:
        illusion: effect
        environment: resolution
      outputs:
      - illusion_properties
      events: []
      capabilities:
      - illusions
      - perception
    requires:
    - dnd2024:glossary_illusions
    citations:
    - SRD 5.2.1 pp. 183; builtin/srd-5.2.1/glossary/illusions.md
  dnd2024:glossary/immunity:
    source_sha256: f7de58ba41b40c1e119ca76b71a53dc8f2c6ba178a6decabc7c5bd4e8a37d1d9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/immunity
        implementation: dnd2024:glossary_immunity
        parameters:
          scopes:
          - damage_type
          - condition
          matching_damage: 0
          matching_condition_applies: false
          other_matching_effects: false
    contract:
      id: dnd2024:glossary_immunity
      inputs:
        subject: object
        incoming_effect: effect
        immunity: definition
      outputs:
      - immunity_result
      events:
      - damage.pending
      - condition.applied
      capabilities:
      - damage
      - conditions
    requires:
    - dnd2024:glossary_immunity
    citations:
    - SRD 5.2.1 pp. 183; builtin/srd-5.2.1/glossary/immunity.md
  dnd2024:glossary/improvised_weapons:
    source_sha256: 824e36a7a5bee48725c7ec50dddbb8c979594092feadf468ecd0494b0ff11613
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/improvised_weapons
        implementation: dnd2024:glossary_improvised_weapons
        parameters:
          qualifies_any:
          - object_used_as_makeshift_weapon
          - ranged_weapon_used_for_melee
          - melee_weapon_without_thrown_property_thrown
          - weapon_used_contrary_to_design
          proficiency_bonus_added: false
          damage:
            dice: 1d4
            type_choice:
              chooser: gm
              options: *id002
              constraint: appropriate_for_object
          thrown_range_feet:
            normal: 20
            long: 60
          weapon_equivalent:
            optional: true
            chooser: gm
            requires_resemblance: true
            allowed_categories:
            - simple
            - martial
            replacement_rules: selected_weapon_all_rules
    contract:
      id: dnd2024:glossary_improvised_weapons
      inputs:
        subject: participant
        weapon: item_instance
        use_mode: choice
        gm_damage_type: choice
        equivalent_weapon:
          type: definition
          required: false
      outputs:
      - weapon_attack_profile
      events:
      - attack.pending
      capabilities:
      - equipment
      - attacks
      - damage
      - gm_choices
    requires:
    - dnd2024:glossary_improvised_weapons
    citations:
    - SRD 5.2.1 pp. 183; builtin/srd-5.2.1/glossary/improvised_weapons.md
  dnd2024:glossary/incapacitated_condition:
    source_sha256: 090d59a1417b1cb5b5427d100837522e5a9d4a48d19e0e5248e2432e3a8877cf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/incapacitated_condition
        implementation: dnd2024:glossary_incapacitated_condition
        parameters:
          condition: incapacitated
          forbidden_economy:
          - action
          - bonus_action
          - reaction
          concentration_broken: true
          can_speak: false
          initiative_roll: disadvantage
    contract:
      id: dnd2024:glossary_incapacitated_condition
      inputs:
        subject: participant
      outputs:
      - condition_effects
      events:
      - condition.applied
      - action.requested
      - bonus_action.requested
      - reaction.requested
      - initiative.rolled
      capabilities:
      - conditions
      - action_economy
      - concentration
      - initiative
    requires:
    - dnd2024:glossary_incapacitated_condition
    citations:
    - SRD 5.2.1 pp. 184; builtin/srd-5.2.1/glossary/incapacitated_condition.md
  dnd2024:glossary/indifferent_attitude:
    source_sha256: a43f943a1be19dfbd5b76f608a9755c7153f09e9915fe5326a9757b000045082
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/indifferent_attitude
        implementation: dnd2024:glossary_indifferent_attitude
        parameters:
          attitude: indifferent
          monster_default: true
          desire_to_help: false
          desire_to_hinder: false
    contract:
      id: dnd2024:glossary_indifferent_attitude
      inputs:
        subject: participant
        other: participant
      outputs:
      - default_attitude
      events: []
      capabilities:
      - relationships
    requires:
    - dnd2024:glossary_indifferent_attitude
    citations:
    - SRD 5.2.1 pp. 184; builtin/srd-5.2.1/glossary/indifferent_attitude.md
  dnd2024:glossary/influence_action:
    source_sha256: 5dff7c156574c1c1834a14b028336d9757fd974419db7e1993a11ad5439ca02d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/influence_action
        implementation: dnd2024:glossary_influence_action
        parameters:
          cost: action
          target: monster
          interaction_modes:
          - deceive
          - intimidate
          - amuse
          - persuade
          - gently_coax
          receptiveness:
            chooser: gm
            options:
            - willing
            - unwilling
            - hesitant
            constraints:
              willing: urging_aligns_with_desires
              unwilling: urging_repugnant_or_counter_to_alignment
              hesitant: hesitant_to_comply
          branches:
            willing:
              check_required: false
              complies: true
              fulfillment_method_chooser: monster
            unwilling:
              check_required: false
              complies: false
            hesitant:
              check_required: true
              check_chooser: gm
              default_dc:
                op: max
                args:
                - 15
                - var: monster_intelligence_score
              attitude_modes:
                friendly: advantage
                hostile: disadvantage
                indifferent: normal
              success: complies
              failure:
                repeat_same_urging_cooldown:
                  default_hours: 24
                  override:
                    chooser: gm
                    type: duration
          suggested_checks:
          - mode: deceive
            ability: charisma
            skill: deception
            target_understands: true
          - mode: intimidate
            ability: charisma
            skill: intimidation
          - mode: amuse
            ability: charisma
            skill: performance
          - mode: persuade
            ability: charisma
            skill: persuasion
            target_understands: true
          - mode: gently_coax
            ability: wisdom
            skill: animal_handling
            target_types:
            - beast
            - monstrosity
    contract:
      id: dnd2024:glossary_influence_action
      inputs:
        subject: participant
        target: participant
        interaction: choice
        gm_receptiveness: choice
        gm_check: choice
        gm_dc:
          type: integer
          required: false
        gm_cooldown:
          type: duration
          required: false
      outputs:
      - compliance
      - check
      - repeat_cooldown
      events:
      - action.requested
      - ability_check.completed
      capabilities:
      - action_economy
      - relationships
      - checks
      - gm_choices
    requires:
    - dnd2024:glossary_influence_action
    citations:
    - SRD 5.2.1 pp. 184; builtin/srd-5.2.1/glossary/influence_action.md
  dnd2024:glossary/initiative:
    source_sha256: 0ccd759a6a863ab2c619d2f6491c165b2f4e5ca09cee0f5d59aa7e65112f271c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/initiative
        implementation: dnd2024:glossary_initiative
        parameters:
          purpose: combat_turn_order
          mode_choice:
            chooser: gm
            options:
            - roll
            - score
            default: roll
          score:
            op: add
            args:
            - 10
            - var: dexterity_modifier
          score_roll_mode_adjustment:
            advantage: 5
            disadvantage: -5
            normal: 0
    contract:
      id: dnd2024:glossary_initiative
      inputs:
        subject: participant
        mode: choice
        roll_mode: choice
      outputs:
      - initiative_score
      events:
      - initiative.requested
      capabilities:
      - initiative
      - checks
      - gm_choices
    requires:
    - dnd2024:glossary_initiative
    citations:
    - SRD 5.2.1 pp. 184; builtin/srd-5.2.1/glossary/initiative.md
  dnd2024:glossary/invisible_condition:
    source_sha256: b0bb513002633703f6ad780f9644ef3ae0a012ec5a7510eb4b4f0a5e57b56cca
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/invisible_condition
        implementation: dnd2024:glossary_invisible_condition
        parameters:
          condition: invisible
          initiative_roll: advantage
          concealment:
            unaffected_by_requires_seen_target_effects: true
            exception: effect_creator_can_see_subject
            includes_worn_and_carried_equipment: true
          attacks:
            incoming: disadvantage
            outgoing: advantage
            exception: other_creature_can_see_subject
    contract:
      id: dnd2024:glossary_invisible_condition
      inputs:
        subject: participant
        other:
          type: participant
          required: false
      outputs:
      - condition_effects
      events:
      - initiative.rolled
      - attack.pending
      - effect.targeting_pending
      capabilities:
      - conditions
      - visibility
      - attacks
      - initiative
    requires:
    - dnd2024:glossary_invisible_condition
    citations:
    - SRD 5.2.1 pp. 184; builtin/srd-5.2.1/glossary/invisible_condition.md
  dnd2024:glossary/jumping:
    source_sha256: aedd0a8899676f83d1cbed8fde25940cd4fe1834f3bba7957a63508560442f32
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/jumping
        implementation: dnd2024:glossary_jumping
        parameters:
          select_one:
          - direction: horizontal
            rule:
              key: dnd2024:glossary/long_jump
          - direction: vertical
            rule:
              key: dnd2024:glossary/high_jump
    contract:
      id: dnd2024:glossary_jumping
      inputs:
        subject: participant
        direction: choice
      outputs:
      - jump_rule
      events: []
      capabilities:
      - movement
    requires:
    - dnd2024:glossary_jumping
    citations:
    - SRD 5.2.1 pp. 184; builtin/srd-5.2.1/glossary/jumping.md
  dnd2024:glossary/knocking_out_a_creature:
    source_sha256: 668ac040834390a0dfba4bb40139b5e1180d43b5f1bd72c3a830c562ba75644b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/knocking_out_a_creature
        implementation: dnd2024:glossary_knocking_out_a_creature
        parameters:
          trigger:
            attack_type: melee
            would_reduce_target_to_hit_points: 0
          optional_attacker_choice: true
          replacement_hit_points: 1
          apply_condition: unconscious
          start_rest: short_rest
          ends_unconscious_when_any:
          - regains_any_hit_points
          - successful_first_aid
          first_aid:
            cost: action
            ability: wisdom
            skill: medicine
            dc: 10
    contract:
      id: dnd2024:glossary_knocking_out_a_creature
      inputs:
        attacker: participant
        target: participant
        knock_out: boolean
        first_aid_actor:
          type: participant
          required: false
      outputs:
      - hit_points
      - condition_changes
      - short_rest_started
      - first_aid_check
      events:
      - damage.pending
      - healing.applied
      - action.requested
      capabilities:
      - attacks
      - hit_points
      - conditions
      - rests
      - checks
    requires:
    - dnd2024:glossary_knocking_out_a_creature
    citations:
    - SRD 5.2.1 pp. 184; builtin/srd-5.2.1/glossary/knocking_out_a_creature.md
  dnd2024:glossary/lightly_obscured:
    source_sha256: 922e0bbbe6e66e4f38dce4ca8450ae703210b3cb77c370b771ec72039ed9515e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/lightly_obscured
        implementation: dnd2024:glossary_lightly_obscured
        parameters:
          check:
            ability: wisdom
            skill: perception
            requires_sight: true
          target_in_lightly_obscured_space: true
          roll_mode: disadvantage
    contract:
      id: dnd2024:glossary_lightly_obscured
      inputs:
        subject: participant
        target_location: location
      outputs:
      - roll_mode
      events:
      - ability_check.pending
      capabilities:
      - visibility
      - checks
    requires:
    - dnd2024:glossary_lightly_obscured
    citations:
    - SRD 5.2.1 pp. 184; builtin/srd-5.2.1/glossary/lightly_obscured.md
  dnd2024:glossary/line_area_of_effect:
    source_sha256: 24af98872a15ffb185d86742e81ba2c1e197c09d4e47d4ab6ad2bddec32381b8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/line_area_of_effect
        implementation: dnd2024:glossary_line_area_of_effect
        parameters:
          shape: line
          propagation: straight_lines
          base_area_rule:
            key: dnd2024:glossary/area_of_effect
          origin: point
          path: straight
          length: effect_specified
          width: effect_specified
          origin_included:
            chooser: creator
            options:
            - false
            - true
            default: false
          geometry_provider: external_geometry
    contract:
      id: dnd2024:glossary_line_area_of_effect
      inputs:
        creator: participant
        origin: location
        dimensions: choice
        geometry: resolution
        include_origin:
          type: boolean
          required: false
      outputs:
      - area
      - included_locations
      events: []
      capabilities:
      - areas
      - geometry
    requires:
    - dnd2024:glossary_line_area_of_effect
    citations:
    - SRD 5.2.1 pp. 184; builtin/srd-5.2.1/glossary/line_area_of_effect.md
  dnd2024:glossary/long_jump:
    source_sha256: 658733a50ff0e97a9df668efd3730bcfe16d87e5e9dff59f78c0e35d94c434be
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/long_jump
        implementation: dnd2024:glossary_long_jump
        parameters:
          direction: horizontal
          running_start:
            minimum_feet: 10
            immediately_before: true
          running_maximum_feet:
            var: strength_score
          standing_multiplier: 0.5
          movement_cost_per_foot: 1
          difficult_terrain_landing:
            ability: dexterity
            skill: acrobatics
            dc: 10
            failure_condition: prone
          low_obstacle:
            height_maximum_jump_distance_fraction: 0.25
            check_required:
              chooser: gm
              options:
              - true
              - false
            ability: strength
            skill: athletics
            dc: 10
            failure: hit_obstacle
          default_geometry_assumption: jump_height_irrelevant
    contract:
      id: dnd2024:glossary_long_jump
      inputs:
        subject: participant
        run_up_feet: number
        jump_distance_feet: number
        difficult_landing: boolean
        obstacle:
          type: object
          required: false
        gm_requires_obstacle_check:
          type: boolean
          required: false
      outputs:
      - jump_limit
      - movement_cost
      - checks
      - condition_changes
      events:
      - movement.pending
      - jump.landed
      capabilities:
      - movement
      - checks
      - geometry
      - gm_choices
    requires:
    - dnd2024:glossary_long_jump
    citations:
    - SRD 5.2.1 pp. 184, 185; builtin/srd-5.2.1/glossary/long_jump.md
  dnd2024:glossary/long_rest:
    source_sha256: 505a5fcb199b5c188b2f77ab1c54e1e92a2ef6cae51fa5aba2f066b055f9334e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/long_rest
        implementation: dnd2024:glossary_long_rest
        parameters:
          minimum_hours: 8
          minimum_sleep_hours: 6
          maximum_light_activity_hours: 2
          light_activities:
          - read
          - talk
          - eat
          - stand_watch
          sleep_condition: unconscious
          minimum_hours_after_previous_long_rest_finish_before_start: 16
          start_minimum_hit_points: 1
          completion:
            regain_lost_hit_points: all
            regain_spent_hit_dice: all
            restore_reduced_hit_point_maximum: true
            restore_reduced_ability_scores: true
            exhaustion_levels_removed: 1
            feature_recharge: each_feature_description
          interruptions:
          - event: initiative.rolled
          - event: spell.cast
            spell_level_min: 1
          - event: damage.taken
          - activity_any:
            - walking
            - physical_exertion
            hours: 1
          interruption_result:
            rest_stops: true
            short_rest_benefits_if_prior_rest_hours_at_least: 1
          resume:
            immediate_allowed: true
            additional_hours_per_interruption: 1
    contract:
      id: dnd2024:glossary_long_rest
      inputs:
        subject: participant
        rest_log: resolution
        resume_after_interruption:
          type: boolean
          required: false
      outputs:
      - rest_progress
      - rest_benefits
      - interruption_result
      events:
      - long_rest.started
      - long_rest.completed
      - initiative.rolled
      - spell.cast
      - damage.taken
      - activity.completed
      capabilities:
      - rests
      - hit_points
      - statistics
      - conditions
      - resources
    requires:
    - dnd2024:glossary_long_rest
    citations:
    - SRD 5.2.1 pp. 185; builtin/srd-5.2.1/glossary/long_rest.md
  dnd2024:glossary/magic_action:
    source_sha256: 24fef25fbb4317ec9ef751f066828493742251b517720c58095d05c3f64e55cd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/magic_action
        implementation: dnd2024:glossary_magic_action
        parameters:
          cost: action
          options:
          - cast_action_casting_time_spell
          - activate_magic_action_feature
          - activate_magic_action_item
          long_cast:
            casting_time_minutes_min: 1
            magic_action_each_casting_turn: true
            concentration_required: true
            concentration_loss:
              spell_fails: true
              slot_expended: false
    contract:
      id: dnd2024:glossary_magic_action
      inputs:
        subject: participant
        magic_source: definition
      outputs:
      - magic_activation
      - casting_progress
      - slot_cost
      events:
      - action.requested
      - spell.cast_started
      - concentration.ended
      capabilities:
      - action_economy
      - spellcasting
      - concentration
      - equipment
    requires:
    - dnd2024:glossary_magic_action
    citations:
    - SRD 5.2.1 pp. 185; builtin/srd-5.2.1/glossary/magic_action.md
  dnd2024:glossary/magical_effect:
    source_sha256: 9f306add578bedbb41581eaff3e2de69079bc0eac722cdc9753cdd4368686886
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/magical_effect
        implementation: dnd2024:glossary_magical_effect
        parameters:
          magical_when_any:
          - created_by_spell
          - created_by_magic_item
          - phenomenon_explicitly_labeled_magical
    contract:
      id: dnd2024:glossary_magical_effect
      inputs:
        effect: effect
      outputs:
      - is_magical
      events: []
      capabilities:
      - effects
    requires:
    - dnd2024:glossary_magical_effect
    citations:
    - SRD 5.2.1 pp. 185; builtin/srd-5.2.1/glossary/magical_effect.md
  dnd2024:glossary/malnutrition_hazard:
    source_sha256: e9336509b2a7b8097516388d635fc701e649bebc9d3c00853f56afbab61ab1bf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/malnutrition_hazard
        implementation: dnd2024:glossary_malnutrition_hazard
        parameters:
          daily_food_pounds:
            tiny: 0.25
            small: 1
            medium: 1
            large: 4
            huge: 16
            gargantuan: 64
          partial_food:
            predicate:
              greater_than: 0
              less_than_daily_requirement_fraction: 0.5
            day_end_save:
              ability: constitution
              dc: 10
              failure_exhaustion_levels: 1
          no_food:
            days_before_first_exhaustion: 5
            day_5_end_exhaustion_levels: 1
            each_subsequent_day_end_levels: 1
            save_allowed: false
          exhaustion_cause: malnutrition
          cause_exhaustion_removal_locked_until:
            food_consumed_pounds_at_least: full_daily_requirement
    contract:
      id: dnd2024:glossary_malnutrition_hazard
      inputs:
        subject: participant
        food_consumed_pounds: number
        consecutive_days_without_food: integer
      outputs:
      - exhaustion_change
      - save
      - recovery_lock
      events:
      - day.ended
      - food.consumed
      - exhaustion.removal_pending
      capabilities:
      - survival
      - hazards
      - conditions
      - checks
    requires:
    - dnd2024:glossary_malnutrition_hazard
    citations:
    - SRD 5.2.1 pp. 185; builtin/srd-5.2.1/glossary/malnutrition_hazard.md
  dnd2024:glossary/monster:
    source_sha256: 2d972f10b44694c5d994900bce582c1cc6a85c509fdb6fa1e0f6f7b0d0f6ce2f
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 185; builtin/srd-5.2.1/glossary/monster.md
  dnd2024:glossary/nonplayer_character:
    source_sha256: a8aadbfbc61f90ce64b17aa20cb5a5374b3eb8db8bf93c9157ddb3e4e2e7085f
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 185; builtin/srd-5.2.1/glossary/nonplayer_character.md
  dnd2024:glossary/object:
    source_sha256: 4a3981ec53d454c98e2ad5c18179d326b4f322f34d1e5bc711973ea97d7513d1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/object
        implementation: dnd2024:glossary_object
        parameters:
          living: false
          distinct: true
          composite_things: multiple_objects
    contract:
      id: dnd2024:glossary_object
      inputs:
        object: object
      outputs:
      - object_classification
      events: []
      capabilities:
      - objects
      - targeting
    requires:
    - dnd2024:glossary_object
    citations:
    - SRD 5.2.1 pp. 185; builtin/srd-5.2.1/glossary/object.md
  dnd2024:glossary/occupied_space:
    source_sha256: 59c4c90d9e765c5d38e1cb5f1252d161697915eaa4fd30596f26ff3b2b736796
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/occupied_space
        implementation: dnd2024:glossary_occupied_space
        parameters:
          predicate:
            any:
            - creature_in_space
            - space_completely_filled_by_objects
    contract:
      id: dnd2024:glossary_occupied_space
      inputs:
        location: location
        occupancy: resolution
      outputs:
      - occupied
      events: []
      capabilities:
      - geometry
    requires:
    - dnd2024:glossary_occupied_space
    citations:
    - SRD 5.2.1 pp. 185; builtin/srd-5.2.1/glossary/occupied_space.md
  dnd2024:glossary/opportunity_attacks:
    source_sha256: 72300e5f266afff5ae696bcca9a2a2d1c9eef54ba47fe6d1e675e549ab984fdc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/opportunity_attacks
        implementation: dnd2024:glossary_opportunity_attacks
        parameters:
          trigger:
            creature_leaves_reach: true
            subject_can_see_creature: true
            movement_uses_any:
            - action
            - bonus_action
            - reaction
            - speed
          cost: reaction
          attack:
            count: 1
            type: melee
            options:
            - weapon
            - unarmed_strike
            target: provoking_creature
            timing: immediately_before_leaving_reach
    contract:
      id: dnd2024:glossary_opportunity_attacks
      inputs:
        subject: participant
        provoker: participant
        movement: event
        attack_option: choice
      outputs:
      - reaction_attack
      events:
      - reach.exit_pending
      capabilities:
      - attacks
      - action_economy
      - movement
    requires:
    - dnd2024:glossary_opportunity_attacks
    citations:
    - SRD 5.2.1 pp. 185; builtin/srd-5.2.1/glossary/opportunity_attacks.md
  dnd2024:glossary/paralyzed_condition:
    source_sha256: d03ef4c524eb81bbdc5f6281bfd23075b30bd0436a2e1f83f971c346269b15b6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/paralyzed_condition
        implementation: dnd2024:glossary_paralyzed_condition
        parameters:
          condition: paralyzed
          grants_condition: incapacitated
          automatic_save_failure_abilities:
          - strength
          - dexterity
          incoming_attack_rolls: advantage
          automatic_critical_if_hit_attacker_within_feet: 5
          speed: 0
          speed_can_increase: false
    contract:
      id: dnd2024:glossary_paralyzed_condition
      inputs:
        subject: participant
      outputs:
      - condition_effects
      events:
      - condition.applied
      - condition.ended
      - attack.pending
      - saving_throw.pending
      - speed.calculated
      capabilities:
      - conditions
      - checks
      - attacks
      - movement
      - effects
    requires:
    - dnd2024:glossary_paralyzed_condition
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/paralyzed_condition.md
  dnd2024:glossary/passive_perception:
    source_sha256: b835a4588ddf2e5c7a33db9fc87fa8651033ecc80f3668ea9c072a64e2474590
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/passive_perception
        implementation: dnd2024:glossary_passive_perception
        parameters:
          use:
            chooser: gm
            scope: notice_without_conscious_perception_check
          score:
            op: add
            args:
            - 10
            - var: wisdom_perception_check_bonus
          roll_mode_adjustment:
            advantage: 5
            disadvantage: -5
            normal: 0
    contract:
      id: dnd2024:glossary_passive_perception
      inputs:
        subject: participant
        roll_mode: choice
        gm_uses_passive: boolean
      outputs:
      - passive_perception
      events: []
      capabilities:
      - checks
      - perception
      - gm_choices
    requires:
    - dnd2024:glossary_passive_perception
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/passive_perception.md
  dnd2024:glossary/per_day:
    source_sha256: 77a852950e9eab71032d6958106b1645b463406a79f72e6393fca5df8ac8365c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/per_day
        implementation: dnd2024:glossary_per_day
        parameters:
          usage_limit: feature_specified
          reset: long_rest_completed
          exhausted_uses_require_reset: true
    contract:
      id: dnd2024:glossary_per_day
      inputs:
        subject: participant
        feature: definition
        maximum_uses: integer
      outputs:
      - remaining_uses
      events:
      - feature.used
      - long_rest.completed
      capabilities:
      - resources
      - rests
    requires:
    - dnd2024:glossary_per_day
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/per_day.md
  dnd2024:glossary/petrified_condition:
    source_sha256: c2c3c4d40b947c0c45a7d82b27453c0b20ea056dda6a19af2df41aadc46557c1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/petrified_condition
        implementation: dnd2024:glossary_petrified_condition
        parameters:
          condition: petrified
          grants_condition: incapacitated
          automatic_save_failure_abilities:
          - strength
          - dexterity
          incoming_attack_rolls: advantage
          speed: 0
          speed_can_increase: false
          transform:
            targets:
            - self
            - worn_nonmagical_objects
            - carried_nonmagical_objects
            substance: solid_inanimate
            default_substance: stone
            weight_multiplier: 10
            aging_stops: true
          damage_resistance: all
          condition_immunity:
          - poisoned
    contract:
      id: dnd2024:glossary_petrified_condition
      inputs:
        subject: participant
      outputs:
      - condition_effects
      events:
      - condition.applied
      - condition.ended
      - attack.pending
      - saving_throw.pending
      - speed.calculated
      capabilities:
      - conditions
      - checks
      - attacks
      - movement
      - effects
    requires:
    - dnd2024:glossary_petrified_condition
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/petrified_condition.md
  dnd2024:glossary/player_character:
    source_sha256: 9a44183b3877d3a335155e5688cc157489b3eba0da76051e7600a66028829c73
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/player_character.md
  dnd2024:glossary/poisoned_condition:
    source_sha256: 3eac66c2e8d6180e9ff0f58c6e7f05e8c18ab1d077f677fcedc7f7a0b587f2f6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/poisoned_condition
        implementation: dnd2024:glossary_poisoned_condition
        parameters:
          condition: poisoned
          own_attack_rolls: disadvantage
          ability_checks: disadvantage
    contract:
      id: dnd2024:glossary_poisoned_condition
      inputs:
        subject: participant
      outputs:
      - condition_effects
      events:
      - attack.pending
      - ability_check.pending
      capabilities:
      - conditions
      - checks
      - attacks
    requires:
    - dnd2024:glossary_poisoned_condition
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/poisoned_condition.md
  dnd2024:glossary/possession:
    source_sha256: 739a9a972273b15e2911ab3398b69b2c2a16fde6169243842406d2a28815364f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/possession
        implementation: dnd2024:glossary_possession
        parameters:
          operation: possessing_effect_defined
          prevention_source:
            key: dnd2024:spells/protection_from_evil_and_good
          ending_source:
            key: dnd2024:spells/dispel_evil_and_good
          spell_specific_eligibility_still_applies: true
    contract:
      id: dnd2024:glossary_possession
      inputs:
        subject: participant
        possession: effect
        counter_effect:
          type: effect
          required: false
      outputs:
      - possession_state
      events:
      - possession.pending
      - effect.removal_pending
      capabilities:
      - effects
      - possession
    requires:
    - dnd2024:glossary_possession
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/possession.md
  dnd2024:glossary/proficiency:
    source_sha256: dd1fdeee715e9f62e06a61aae0a0e728a168f32de366f0f4db085cc46460a0c6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/proficiency
        implementation: dnd2024:glossary_proficiency
        parameters:
          categories:
          - skill
          - saving_throw
          - weapon
          - tool
          d20_test_bonus:
            requires: test_uses_proficiency
            add:
              var: proficiency_bonus
    contract:
      id: dnd2024:glossary_proficiency
      inputs:
        subject: participant
        proficiency: definition
        test: resolution
      outputs:
      - check_bonus
      events:
      - d20_test.pending
      capabilities:
      - proficiency
      - checks
    requires:
    - dnd2024:glossary_proficiency
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/proficiency.md
  dnd2024:glossary/prone_condition:
    source_sha256: f8f0eeb7c0e133dd553607a8124dca37052a2dd292c75cfbb99515e4b8c0b844
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/prone_condition
        implementation: dnd2024:glossary_prone_condition
        parameters:
          condition: prone
          movement_options:
          - crawl
          - right_self
          right_self:
            movement_cost:
              op: floor
              args:
              - op: divide
                args:
                - var: speed
                - 2
            requires_speed_greater_than: 0
            ends_condition: true
          own_attack_rolls: disadvantage
          incoming_attacks:
            attacker_distance_feet_max_5: advantage
            attacker_distance_feet_greater_5: disadvantage
    contract:
      id: dnd2024:glossary_prone_condition
      inputs:
        subject: participant
        attacker:
          type: participant
          required: false
        movement_option:
          type: choice
          required: false
      outputs:
      - condition_effects
      - stand_cost
      events:
      - movement.pending
      - attack.pending
      capabilities:
      - conditions
      - movement
      - attacks
    requires:
    - dnd2024:glossary_prone_condition
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/prone_condition.md
  dnd2024:glossary/reach:
    source_sha256: 9fc5368ebd593d260bf08d5cd83f3e62c57b7fe620a534e59d636e61f8ead00a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/reach
        implementation: dnd2024:glossary_reach
        parameters:
          default_feet: 5
          explicit_rule_override_allowed: true
    contract:
      id: dnd2024:glossary_reach
      inputs:
        subject: participant
      outputs:
      - reach_feet
      events: []
      capabilities:
      - attacks
      - geometry
    requires:
    - dnd2024:glossary_reach
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/reach.md
  dnd2024:glossary/reaction:
    source_sha256: b4c18be2701287dc0273d6495b2a698e921e3b5d30eec052f4b28ab154e9ab8e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/reaction
        implementation: dnd2024:glossary_reaction
        parameters:
          trigger: reaction_description_defined
          own_or_other_turn: true
          own_turn_compatible_with:
          - action
          - bonus_action
          - both
          budget:
            maximum: 1
            reset: own_next_turn_start
          available_to_all:
            key: dnd2024:glossary/opportunity_attacks
    contract:
      id: dnd2024:glossary_reaction
      inputs:
        subject: participant
        reaction: definition
        trigger: event
      outputs:
      - reaction_budget
      - eligibility
      events:
      - reaction.requested
      - turn.started
      capabilities:
      - action_economy
    requires:
    - dnd2024:glossary_reaction
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/reaction.md
  dnd2024:glossary/ready_action:
    source_sha256: 5dcdd49536e936cd63d00a718c807a865f2930ce96ae3f8b68cb191f2b871769
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/ready_action
        implementation: dnd2024:glossary_ready_action
        parameters:
          cost: action
          timing: own_turn
          expires: own_next_turn_start
          trigger:
            chooser: subject
            must_be_perceivable: true
          response:
            chooser: subject
            options:
            - chosen_action
            - move_up_to_speed
            cost: reaction
            timing: immediately_after_trigger_finishes
            may_ignore_trigger: true
          spell:
            casting_time_required: action
            cast_when_readied: true
            casting_resources_expended_when_readied: true
            energy_held_until_release: true
            release_cost: reaction
            concentration_required: true
            maximum_hold_until: own_next_turn_start
            concentration_loss:
              dissipates: true
              effect_occurs: false
    contract:
      id: dnd2024:glossary_ready_action
      inputs:
        subject: participant
        trigger: choice
        response: choice
        spell:
          type: definition
          required: false
      outputs:
      - readied_effect
      - resource_cost
      - reaction_response
      events:
      - action.requested
      - trigger.completed
      - reaction.requested
      - concentration.ended
      - turn.started
      capabilities:
      - action_economy
      - events
      - spellcasting
      - concentration
    requires:
    - dnd2024:glossary_ready_action
    citations:
    - SRD 5.2.1 pp. 186, 187; builtin/srd-5.2.1/glossary/ready_action.md
  dnd2024:glossary/resistance:
    source_sha256: 0c245ca31ddf7996031c5e9c0712b5448fa0dd3d32997e2cf9f68142ff662dbf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/resistance
        implementation: dnd2024:glossary_resistance
        parameters:
          matching_damage_multiplier: 0.5
          rounding: down
          applications_per_damage_instance_max: 1
    contract:
      id: dnd2024:glossary_resistance
      inputs:
        subject: object
        damage_type: choice
        damage_amount: integer
      outputs:
      - damage_amount
      events:
      - damage.pending
      capabilities:
      - damage
    requires:
    - dnd2024:glossary_resistance
    citations:
    - SRD 5.2.1 pp. 187; builtin/srd-5.2.1/glossary/resistance.md
  dnd2024:glossary/restrained_condition:
    source_sha256: 199e113a3500ff67773cc75e1bafb4f84d8af4ab55e7cd81ff800f71548a98b8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/restrained_condition
        implementation: dnd2024:glossary_restrained_condition
        parameters:
          condition: restrained
          speed: 0
          speed_can_increase: false
          incoming_attack_rolls: advantage
          own_attack_rolls: disadvantage
          dexterity_saves: disadvantage
    contract:
      id: dnd2024:glossary_restrained_condition
      inputs:
        subject: participant
      outputs:
      - condition_effects
      events:
      - attack.pending
      - saving_throw.pending
      - speed.calculated
      capabilities:
      - conditions
      - attacks
      - checks
      - movement
    requires:
    - dnd2024:glossary_restrained_condition
    citations:
    - SRD 5.2.1 pp. 187; builtin/srd-5.2.1/glossary/restrained_condition.md
  dnd2024:glossary/ritual:
    source_sha256: 5af6ae5176d45fa10027980eaae9351411482aa092c52cf295af31626c7e5ac1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/ritual
        implementation: dnd2024:glossary_ritual
        parameters:
          eligibility:
            spell_prepared: true
            ritual_tag: true
          casting_time_added_minutes: 10
          slot_expended: false
          upcast_allowed: false
    contract:
      id: dnd2024:glossary_ritual
      inputs:
        caster: participant
        spell: definition
      outputs:
      - eligibility
      - casting_time
      - slot_cost
      - cast_level
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting
    requires:
    - dnd2024:glossary_ritual
    citations:
    - SRD 5.2.1 pp. 187; builtin/srd-5.2.1/glossary/ritual.md
  dnd2024:glossary/round_down:
    source_sha256: 966120acb6d8996ef8f3c9501a68a0b0e10955abd122b2bf5ab32dc8a09a6c8e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/round_down
        implementation: dnd2024:glossary_round_down
        parameters:
          operations:
          - multiply
          - divide
          fractional_result_rounding: floor
          fraction_magnitude_irrelevant: true
          explicit_round_up_rule_overrides: true
    contract:
      id: dnd2024:glossary_round_down
      inputs:
        value: number
        rounding_override:
          type: choice
          required: false
      outputs:
      - rounded_result
      events: []
      capabilities:
      - arithmetic
    requires:
    - dnd2024:glossary_round_down
    citations:
    - SRD 5.2.1 pp. 187; builtin/srd-5.2.1/glossary/round_down.md
  dnd2024:glossary/save:
    source_sha256: d1119bdd743c54fec43510cb3744823b8e811881b87a51a9d3da3adb7ec4d6c5
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 187; builtin/srd-5.2.1/glossary/save.md
  dnd2024:glossary/saving_throw:
    source_sha256: f21ea60c2f069ef19d2cd700ee84ca8dc29ca9c5ad5c6cd0fa081d231186dc03
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/saving_throw
        implementation: dnd2024:glossary_saving_throw
        parameters:
          normally_requires_rule: true
          voluntary_failure:
            chooser: subject
            rolling_required: false
          missing_relevant_ability_score: automatic_failure
          result_effects: requiring_effect_defined
    contract:
      id: dnd2024:glossary_saving_throw
      inputs:
        subject: object
        effect: effect
        ability: choice
        voluntary_failure: boolean
      outputs:
      - save_requirement
      - save_result
      events:
      - saving_throw.pending
      capabilities:
      - checks
    requires:
    - dnd2024:glossary_saving_throw
    citations:
    - SRD 5.2.1 pp. 187; builtin/srd-5.2.1/glossary/saving_throw.md
  dnd2024:glossary/search_action:
    source_sha256: 766e9b2c8387dea9f364eac07904ed839df92d5f8bf3f869285c1beb2e06ecae
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/search_action
        implementation: dnd2024:glossary_search_action
        parameters:
          cost: action
          ability: wisdom
          purpose: discern_nonobvious_information
          suggested_skill_by_target:
            creature_state_of_mind: insight
            creature_ailment: medicine
            cause_of_death: medicine
            concealed_creature: perception
            concealed_object: perception
            tracks: survival
            food: survival
    contract:
      id: dnd2024:glossary_search_action
      inputs:
        subject: participant
        search_topic: choice
        dc: integer
      outputs:
      - check
      - discovered_information
      events:
      - action.requested
      capabilities:
      - action_economy
      - checks
      - perception
    requires:
    - dnd2024:glossary_search_action
    citations:
    - SRD 5.2.1 pp. 187; builtin/srd-5.2.1/glossary/search_action.md
  dnd2024:glossary/shape_shifting:
    source_sha256: 0df94767c534196578b99309f4d2b1c044296600c107b3701c35a6c7ef0bc61f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/shape_shifting
        implementation: dnd2024:glossary_shape_shifting
        parameters:
          form_properties: source_effect_defined
          ongoing_effects_carry_over_by_default: true
          ongoing_effect_kinds:
          - conditions
          - spells
          - curses
          - other_effects
          source_effect_override_allowed: true
          death_reverts_to_true_form: true
    contract:
      id: dnd2024:glossary_shape_shifting
      inputs:
        subject: participant
        transform_effect: effect
      outputs:
      - form
      - retained_effects
      events:
      - shape.changed
      - creature.died
      capabilities:
      - transformation
      - effects
    requires:
    - dnd2024:glossary_shape_shifting
    citations:
    - SRD 5.2.1 pp. 187; builtin/srd-5.2.1/glossary/shape_shifting.md
  dnd2024:glossary/short_rest:
    source_sha256: df68d1031b1ccc594679d06d4489ea26d046a6e57d1749537abe6373ce74c6a9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/short_rest
        implementation: dnd2024:glossary_short_rest
        parameters:
          duration_hours: 1
          maximum_exertion: light_activity
          allowed_examples:
          - read
          - talk
          - eat
          - stand_watch
          start_minimum_hit_points: 1
          completion:
            hit_dice:
              optional: true
              spend_available_only: true
              one_at_a_time: true
              healing_per_die:
                op: max
                args:
                - 1
                - op: add
                  args:
                  - var: hit_die_roll
                  - var: constitution_modifier
              decide_next_after_each_roll: true
            feature_recharge: feature_description_defined
          interruptions:
          - event: initiative.rolled
          - event: spell.cast
            spell_level_min: 1
          - event: damage.taken
          interruption_result:
            rest_stops: true
            benefits: false
    contract:
      id: dnd2024:glossary_short_rest
      inputs:
        subject: participant
        rest_log: resolution
        hit_dice_spending: choice
      outputs:
      - rest_progress
      - healing
      - hit_dice_spent
      - feature_recharge
      events:
      - short_rest.started
      - short_rest.completed
      - initiative.rolled
      - spell.cast
      - damage.taken
      capabilities:
      - rests
      - hit_points
      - resources
    requires:
    - dnd2024:glossary_short_rest
    citations:
    - SRD 5.2.1 pp. 187; builtin/srd-5.2.1/glossary/short_rest.md
  dnd2024:glossary/simultaneous_effects:
    source_sha256: 28a4ec8b94f35701167ee201353f4b00949e2df057eaf3f7291b6c3c04d871ef
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/simultaneous_effects
        implementation: dnd2024:glossary_simultaneous_effects
        parameters:
          when:
            same_time: true
            during_turn: true
            minimum_effects: 2
          order_choice:
            chooser: current_turn_creature_controller
            options: all_permutations_of_simultaneous_effects
    contract:
      id: dnd2024:glossary_simultaneous_effects
      inputs:
        turn_owner: participant
        effects: definition_list
        ordering: choice
      outputs:
      - ordered_effects
      events:
      - effects.simultaneous
      capabilities:
      - events
    requires:
    - dnd2024:glossary_simultaneous_effects
    citations:
    - SRD 5.2.1 pp. 187; builtin/srd-5.2.1/glossary/simultaneous_effects.md
  dnd2024:glossary/size:
    source_sha256: 2a54caef9eb964bca8544397b1b40236399c8650a032099b5fdf02c647be2b63
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/size
        implementation: dnd2024:glossary_size
        parameters:
          ordered_categories:
          - tiny
          - small
          - medium
          - large
          - huge
          - gargantuan
          applies_to:
          - creature
          - object
          creature_mechanical_use: combat_occupied_space
          object_mechanical_use: hit_points
    contract:
      id: dnd2024:glossary_size
      inputs:
        subject: object
        size: choice
      outputs:
      - size_category
      events: []
      capabilities:
      - statistics
      - geometry
    requires:
    - dnd2024:glossary_size
    citations:
    - SRD 5.2.1 pp. 188; builtin/srd-5.2.1/glossary/size.md
  dnd2024:glossary/skill:
    source_sha256: 6ea9444a312aeacc2885072d678b2a71796877194517ba11cbb90396f9b57d44
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/skill
        implementation: dnd2024:glossary_skill
        parameters:
          context: ability_check
          skill_proficiency_adds_proficiency_bonus: true
          requires_check_associated_with_skill: true
    contract:
      id: dnd2024:glossary_skill
      inputs:
        subject: participant
        skill: definition
        check: resolution
      outputs:
      - check_bonus
      events:
      - ability_check.pending
      capabilities:
      - proficiency
      - checks
    requires:
    - dnd2024:glossary_skill
    citations:
    - SRD 5.2.1 pp. 188; builtin/srd-5.2.1/glossary/skill.md
  dnd2024:glossary/speed:
    source_sha256: 03c50a8b9b769ee8ecef061b3ac5c962cd9f941e69c9d2e8a7c6c5c52479f854
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/speed
        implementation: dnd2024:glossary_speed
        parameters:
          unit: feet
          budget_period: own_turn
          special_modes:
          - burrow
          - climb
          - fly
          - swim
          mode_choice:
            chooser: subject
            switch_during_move: true
          on_mode_switch:
            remaining:
              op: subtract
              args:
              - var: new_speed
              - var: distance_already_moved
            usable_if_remaining_greater_than: 0
          temporary_speed_changes:
            also_apply_to_all_special_speeds: true
            same_duration: true
            additive_delta: same_amount
            set_zero: all_zero
            multiply: same_factor
    contract:
      id: dnd2024:glossary_speed
      inputs:
        subject: participant
        selected_mode: choice
        distance_already_moved: number
        speed_effect:
          type: effect
          required: false
      outputs:
      - remaining_movement
      - speeds
      events:
      - movement.pending
      - movement_mode.changed
      - speed.changed
      capabilities:
      - movement
    requires:
    - dnd2024:glossary_speed
    citations:
    - SRD 5.2.1 pp. 188; builtin/srd-5.2.1/glossary/speed.md
  dnd2024:glossary/spell:
    source_sha256: c78ef9d21f570407aa18bddd9334e663503bb982ad7d5f7cd281fd7ab7a7196d
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 188; builtin/srd-5.2.1/glossary/spell.md
  dnd2024:glossary/spell_attack:
    source_sha256: dc3f8508413d44f245fda3632db935b41e0e8394f820d987b80742c95fd56bec
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/spell_attack
        implementation: dnd2024:glossary_spell_attack
        parameters:
          test_kind: attack_roll
          source_any:
          - spell
          - other_magical_effect
    contract:
      id: dnd2024:glossary_spell_attack
      inputs:
        attack: resolution
      outputs:
      - is_spell_attack
      events: []
      capabilities:
      - attacks
      - spellcasting
    requires:
    - dnd2024:glossary_spell_attack
    citations:
    - SRD 5.2.1 pp. 188; builtin/srd-5.2.1/glossary/spell_attack.md
  dnd2024:glossary/spellcasting_focus:
    source_sha256: 5662c745a19c8a4f925084e5a4eb8d6ec5d43137b411bf0b2b6863a01420c8f9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/spellcasting_focus
        implementation: dnd2024:glossary_spellcasting_focus
        parameters:
          replaces: material_components
          eligibility:
            creature_can_use_focus_type: true
            material_consumed: false
            material_cost_specified: false
          class_focus_permissions: class_description_defined
    contract:
      id: dnd2024:glossary_spellcasting_focus
      inputs:
        caster: participant
        focus: item_instance
        spell: definition
      outputs:
      - material_component_substitution
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting
      - equipment
    requires:
    - dnd2024:glossary_spellcasting_focus
    citations:
    - SRD 5.2.1 pp. 188; builtin/srd-5.2.1/glossary/spellcasting_focus.md
  dnd2024:glossary/sphere_area_of_effect:
    source_sha256: 32a09751c71fb92abd08be37df0494efb86b71fb6ee9632fa352ceaf26d82b17
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/sphere_area_of_effect
        implementation: dnd2024:glossary_sphere_area_of_effect
        parameters:
          shape: sphere
          propagation: straight_lines
          base_area_rule:
            key: dnd2024:glossary/area_of_effect
          origin: center
          direction: all
          radius: effect_specified
          origin_included: true
          geometry_provider: external_geometry
    contract:
      id: dnd2024:glossary_sphere_area_of_effect
      inputs:
        creator: participant
        origin: location
        dimensions: choice
        geometry: resolution
        include_origin:
          type: boolean
          required: false
      outputs:
      - area
      - included_locations
      events: []
      capabilities:
      - areas
      - geometry
    requires:
    - dnd2024:glossary_sphere_area_of_effect
    citations:
    - SRD 5.2.1 pp. 188; builtin/srd-5.2.1/glossary/sphere_area_of_effect.md
  dnd2024:glossary/stable:
    source_sha256: ed0820bb4cc5c499fad98cd20de0a12df8405b71e83565ca402cb798ae8fa8aa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/stable
        implementation: dnd2024:glossary_stable
        parameters:
          hit_points: 0
          death_saving_throws_required: false
    contract:
      id: dnd2024:glossary_stable
      inputs:
        subject: participant
      outputs:
      - death_save_exemption
      events:
      - turn.started
      capabilities:
      - death
      - hit_points
    requires:
    - dnd2024:glossary_stable
    citations:
    - SRD 5.2.1 pp. 188; builtin/srd-5.2.1/glossary/stable.md
  dnd2024:glossary/stat_block:
    source_sha256: 55ac0aa288b85e2b18c6ed3fc5029f50eb11cc6c79f4557a4c62f59bc0c518e9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/stat_block
        implementation: dnd2024:glossary_stat_block
        parameters:
          schema:
            size: size_category
            creature_type: creature_type_and_tags
            alignment:
              default: suggested
              actual_chooser: gm
            armor_class: integer
            initiative:
              modifier: integer
              score: integer
            hit_points:
              static: integer
              dice: dice_expression
              constitution_contribution: integer
            speed: feet_by_mode
            abilities: scores_modifiers_and_save_modifiers
            skills: proficiencies
            resistances: damage_type_list
            vulnerabilities: damage_type_list
            immunities: damage_and_condition_lists
            gear: giveable_or_retrievable_equipment
            senses: special_senses_and_passive_perception
            languages: known_languages
            challenge_rating:
              rating: number
              experience_points: integer
              proficiency_bonus: integer
            traits: always_or_conditionally_active_features
            actions: additional_to_glossary_actions
            bonus_actions: available_bonus_actions
            reactions: available_special_reactions
          magically_created_creature_optional_fields:
          - hit_dice
          - initiative
          - challenge_rating
          attack:
            fields:
            - melee_or_ranged
            - attack_bonus
            - reach_or_range
            - on_hit
            default_target_count: 1
          save_effect:
            fields:
            - save_ability
            - dc
            - eligible_creatures
            - failure_result
            - success_result
          damage:
            mode_choice:
              chooser: gm
              options:
              - static
              - dice
            use_both: false
    contract:
      id: dnd2024:glossary_stat_block
      inputs:
        monster: participant
        stat_block: definition
        gm_alignment: choice
        gm_damage_mode: choice
      outputs:
      - monster_statistics
      - attack_defaults
      - damage_mode
      events: []
      capabilities:
      - statistics
      - attacks
      - damage
      - gm_choices
    requires:
    - dnd2024:glossary_stat_block
    citations:
    - SRD 5.2.1 pp. 188, 189; builtin/srd-5.2.1/glossary/stat_block.md
  dnd2024:glossary/study_action:
    source_sha256: 84cbb8e3ac008796b72398329051e39d9aac56ece0100ad968227eec41d871a6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/study_action
        implementation: dnd2024:glossary_study_action
        parameters:
          cost: action
          ability: intelligence
          sources:
          - memory
          - book
          - clue
          - other_knowledge_source
          result: recall_important_information
          suggested_skills:
            arcana:
              topics:
              - spells
              - magic_items
              - eldritch_symbols
              - magical_traditions
              - planes
              creature_types:
              - aberration
              - construct
              - elemental
              - fey
              - monstrosity
            history:
              topics:
              - historic_events
              - historic_people
              - ancient_civilizations
              - wars
              creature_types:
              - giant
              - humanoid
            investigation:
              topics:
              - traps
              - ciphers
              - riddles
              - gadgetry
            nature:
              topics:
              - terrain
              - flora
              - weather
              creature_types:
              - beast
              - dragon
              - ooze
              - plant
            religion:
              topics:
              - deities
              - religious_hierarchies
              - religious_rites
              - holy_symbols
              - cults
              creature_types:
              - celestial
              - fiend
              - undead
    contract:
      id: dnd2024:glossary_study_action
      inputs:
        subject: participant
        topic: choice
        knowledge_source: object
        dc: integer
      outputs:
      - check
      - recalled_information
      events:
      - action.requested
      capabilities:
      - action_economy
      - checks
      - knowledge
    requires:
    - dnd2024:glossary_study_action
    citations:
    - SRD 5.2.1 pp. 189; builtin/srd-5.2.1/glossary/study_action.md
  dnd2024:glossary/stunned_condition:
    source_sha256: 2db74dc8b940b36e810a17179cbc3a28bfcfcef799c99989bdedc13eab9afe08
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/stunned_condition
        implementation: dnd2024:glossary_stunned_condition
        parameters:
          condition: stunned
          grants_condition: incapacitated
          automatic_save_failure_abilities:
          - strength
          - dexterity
          incoming_attack_rolls: advantage
    contract:
      id: dnd2024:glossary_stunned_condition
      inputs:
        subject: participant
      outputs:
      - condition_effects
      events:
      - condition.applied
      - condition.ended
      - attack.pending
      - saving_throw.pending
      - speed.calculated
      capabilities:
      - conditions
      - checks
      - attacks
      - movement
      - effects
    requires:
    - dnd2024:glossary_stunned_condition
    citations:
    - SRD 5.2.1 pp. 189; builtin/srd-5.2.1/glossary/stunned_condition.md
  dnd2024:glossary/suffocation_hazard:
    source_sha256: 16f871ce51e58b42c0147532c7bbb505de5fc8be7d21e0e67b563ad1ebaa054e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/suffocation_hazard
        implementation: dnd2024:glossary_suffocation_hazard
        parameters:
          hold_breath_seconds:
            op: max
            args:
            - 30
            - op: multiply
              args:
              - 60
              - op: add
                args:
                - 1
                - var: constitution_modifier
          suffocating_when_any:
          - out_of_breath
          - choking
          exhaustion:
            timing: own_turn_end
            levels: 1
            cause: suffocation
          breathing_resumes:
            remove_exhaustion_levels: all_caused_by_suffocation
    contract:
      id: dnd2024:glossary_suffocation_hazard
      inputs:
        subject: participant
      outputs:
      - breath_duration
      - exhaustion_change
      events:
      - breath.held
      - turn.ended
      - breathing.resumed
      capabilities:
      - survival
      - hazards
      - conditions
    requires:
    - dnd2024:glossary_suffocation_hazard
    citations:
    - SRD 5.2.1 pp. 189; builtin/srd-5.2.1/glossary/suffocation_hazard.md
  dnd2024:glossary/surprise:
    source_sha256: db013e70c91d7363359ece0d58726ec41f5f39adfbcaf303112300d87bf7f14d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/surprise
        implementation: dnd2024:glossary_surprise
        parameters:
          predicate: caught_unawares_by_combat_start
          initiative_roll: disadvantage
    contract:
      id: dnd2024:glossary_surprise
      inputs:
        subject: participant
        caught_unawares: boolean
      outputs:
      - roll_mode
      events:
      - initiative.rolled
      capabilities:
      - initiative
    requires:
    - dnd2024:glossary_surprise
    citations:
    - SRD 5.2.1 pp. 189; builtin/srd-5.2.1/glossary/surprise.md
  dnd2024:glossary/swim_speed:
    source_sha256: c70b24cca5cef7679a8ea2bb88f149a08e86c1a38bd816f26932a5db87661941
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/swim_speed
        implementation: dnd2024:glossary_swim_speed
        parameters:
          movement_mode: swim
          replaces_speed: true
          ignores_mode_extra_movement_cost: true
    contract:
      id: dnd2024:glossary_swim_speed
      inputs:
        subject: participant
        distance_feet: number
      outputs:
      - movement_cost
      events: []
      capabilities:
      - movement
    requires:
    - dnd2024:glossary_swim_speed
    citations:
    - SRD 5.2.1 pp. 189; builtin/srd-5.2.1/glossary/swim_speed.md
  dnd2024:glossary/swimming:
    source_sha256: dbdf50c19be64bf4a670e10b9a59fb689d8679f733051c2cb35e7264cdd88d95
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/swimming
        implementation: dnd2024:glossary_swimming
        parameters:
          movement_mode: swim
          per_foot_cost:
            normal: 2
            difficult_terrain: 3
          mode_extra_cost_ignored_if:
            has_special_speed: swim
            uses_special_speed: true
          gm_optional_check:
            terrain_any:
            - rough_water
            required:
              chooser: gm
              options:
              - true
              - false
            ability: strength
            skill: athletics
            dc: 15
            success_required_to_move: true
    contract:
      id: dnd2024:glossary_swimming
      inputs:
        subject: participant
        distance_feet: number
        terrain: choice
        use_special_speed: boolean
        gm_requires_check: boolean
      outputs:
      - movement_cost
      - check_requirement
      events:
      - movement.pending
      capabilities:
      - movement
      - checks
      - gm_choices
    requires:
    - dnd2024:glossary_swimming
    citations:
    - SRD 5.2.1 pp. 189; builtin/srd-5.2.1/glossary/swimming.md
  dnd2024:glossary/target:
    source_sha256: d4f8c2124978936fc905d373204108082c61953d3291efe0a295e674f6bb9537
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/target
        implementation: dnd2024:glossary_target
        parameters:
          eligible_kinds:
          - creature
          - object
          targeted_when_any:
          - attack_roll_target
          - forced_save_target
          - selected_spell_recipient
          - selected_other_effect_recipient
    contract:
      id: dnd2024:glossary_target
      inputs:
        subject: object
        effect: effect
      outputs:
      - is_target
      events: []
      capabilities:
      - targeting
    requires:
    - dnd2024:glossary_target
    citations:
    - SRD 5.2.1 pp. 190; builtin/srd-5.2.1/glossary/target.md
  dnd2024:glossary/telepathy:
    source_sha256: 03c9bf01526f827ae96fed7406c5bd2f8ec2b289816d6520bf4c86c655b54461
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/telepathy
        implementation: dnd2024:glossary_telepathy
        parameters:
          magical: true
          range: feature_specified
          shared_language_required: false
          recipient_understanding_requires_any:
          - understands_at_least_one_language
          - has_telepathy
          source_rule_can_override_language_requirements: true
          sight_required: false
          initiate_or_end:
            chooser: telepath
            timing: any_time
            cost: none
          initiation_forbidden_if_any_participant_incapacitated: true
          contact_ends_when_any:
          - telepath_incapacitated
          - recipient_incapacitated
          - recipient_beyond_range
          - telepath_contacts_different_creature
          nontelepath:
            can_receive: true
            can_initiate: false
            can_reply_mentally_during_contact: true
    contract:
      id: dnd2024:glossary_telepathy
      inputs:
        telepath: participant
        recipient: participant
        range_feet: number
        operation: choice
      outputs:
      - contact_state
      - communication_eligibility
      events:
      - telepathy.contact_requested
      - condition.changed
      - movement.completed
      - telepathy.contact_ended
      capabilities:
      - communication
      - relationships
      - conditions
    requires:
    - dnd2024:glossary_telepathy
    citations:
    - SRD 5.2.1 pp. 190; builtin/srd-5.2.1/glossary/telepathy.md
  dnd2024:glossary/teleportation:
    source_sha256: 94719490aef522c4f00fe4da69205e77d85aa754379f4a72de851f6ae0114783
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/teleportation
        implementation: dnd2024:glossary_teleportation
        parameters:
          magical: true
          instantaneous: true
          traverses_intervening_space: false
          movement_cost_default: 0
          explicit_movement_cost_override_allowed: true
          provokes_opportunity_attacks: false
          accompanying_equipment: all_worn_and_carried
          touching_creatures_accompany_by_default: false
          effect_may_allow_accompanying_creatures: true
          blocked_destination:
            blocked_if_any:
            - another_creature_present
            - solid_obstacle
            replacement: nearest_unoccupied_space
            equal_nearest_choice: teleporting_creature
          destination_sight_required: source_effect_defined
          geometry_provider: external_geometry
    contract:
      id: dnd2024:glossary_teleportation
      inputs:
        subject: participant
        destination: location
        teleport_effect: effect
        geometry: resolution
        replacement_choice:
          type: location
          required: false
      outputs:
      - arrival_location
      - transported_equipment
      - movement_cost
      events:
      - teleportation.pending
      capabilities:
      - teleportation
      - geometry
      - equipment
    requires:
    - dnd2024:glossary_teleportation
    citations:
    - SRD 5.2.1 pp. 190; builtin/srd-5.2.1/glossary/teleportation.md
  dnd2024:glossary/temporary_hit_points:
    source_sha256: 08c706e3e0f0bf1a0b6fda88797af172900a8ca0cc89b592c976f84f60847f0a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/temporary_hit_points
        implementation: dnd2024:glossary_temporary_hit_points
        parameters:
          resource: temporary_hit_points
          damage_absorption:
            before: real_hit_points
            amount:
              op: min
              args:
              - var: temporary_hit_points
              - var: incoming_damage
          grant_source: effect_defined
    contract:
      id: dnd2024:glossary_temporary_hit_points
      inputs:
        subject: participant
        incoming_damage: integer
      outputs:
      - temporary_hit_points_spent
      - remaining_damage
      events:
      - damage.pending
      capabilities:
      - hit_points
      - damage
    requires:
    - dnd2024:glossary_temporary_hit_points
    citations:
    - SRD 5.2.1 pp. 190; builtin/srd-5.2.1/glossary/temporary_hit_points.md
  dnd2024:glossary/tremorsense:
    source_sha256: c7ca44ced04458b5277ff7e95901e6d54c0cf0cb355c9c96c86db37a69adf338
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/tremorsense
        implementation: dnd2024:glossary_tremorsense
        parameters:
          range: feature_specified
          detects:
          - creatures
          - moving_objects
          shared_contact_required_any:
          - same_surface
          - same_liquid
          surface_examples:
          - ground
          - wall
          - ceiling
          airborne_targets_detectable: false
          counts_as_sight: false
          result: pinpoint_location
    contract:
      id: dnd2024:glossary_tremorsense
      inputs:
        subject: participant
        target: object
        range_feet: number
        contact_facts: resolution
      outputs:
      - detected_location
      events: []
      capabilities:
      - senses
      - geometry
    requires:
    - dnd2024:glossary_tremorsense
    citations:
    - SRD 5.2.1 pp. 190; builtin/srd-5.2.1/glossary/tremorsense.md
  dnd2024:glossary/truesight:
    source_sha256: 03ee5f46dfb07b8c02bbda833a6701dc92d6aba0bec10d8675dfb7d1d1fd7e21
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/truesight
        implementation: dnd2024:glossary_truesight
        parameters:
          range: feature_specified
          see_in_darkness:
          - normal
          - magical
          see_invisible:
          - creatures
          - objects
          visual_illusions:
            appearance: transparent
            saves: automatic_success
          magical_transformations:
            discern_true_form: true
            requires_seeing_target: true
            target_types:
            - creature
            - object
          see_into_plane: ethereal
    contract:
      id: dnd2024:glossary_truesight
      inputs:
        subject: participant
        target: object
        range_feet: number
      outputs:
      - enhanced_vision
      - save_result
      events:
      - saving_throw.pending
      capabilities:
      - senses
      - illusions
      - checks
    requires:
    - dnd2024:glossary_truesight
    citations:
    - SRD 5.2.1 pp. 190; builtin/srd-5.2.1/glossary/truesight.md
  dnd2024:glossary/unarmed_strike:
    source_sha256: 52217f40f8c22688f3321850b0c5497e4419fd5307af115727dde87e4ef88929
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/unarmed_strike
        implementation: dnd2024:glossary_unarmed_strike
        parameters:
          attack_type: melee
          weapon_used: false
          body_strike_examples:
          - punch
          - kick
          - headbutt
          - similar_forceful_blow
          target_range_feet: 5
          effect_choice:
            chooser: subject
            options:
            - damage
            - grapple
            - shove
          damage:
            attack_bonus:
              op: add
              args:
              - var: strength_modifier
              - var: proficiency_bonus
            hit_damage:
              type: bludgeoning
              amount:
                op: add
                args:
                - 1
                - var: strength_modifier
          grapple:
            target_maximum_size_categories_larger: 1
            free_hand_required: true
            save_ability_choice:
              chooser: target
              options:
              - strength
              - dexterity
            save_dc:
              op: add
              args:
              - 8
              - var: strength_modifier
              - var: proficiency_bonus
            escape_dc: same_as_save_dc
            failure_condition: grappled
          shove:
            target_maximum_size_categories_larger: 1
            save_ability_choice:
              chooser: target
              options:
              - strength
              - dexterity
            save_dc:
              op: add
              args:
              - 8
              - var: strength_modifier
              - var: proficiency_bonus
            failure_choice:
              chooser: subject
              options:
              - push_feet: 5
                direction: away_from_subject
              - condition: prone
    contract:
      id: dnd2024:glossary_unarmed_strike
      inputs:
        subject: participant
        target: participant
        option: choice
        target_save_ability:
          type: choice
          required: false
        shove_result:
          type: choice
          required: false
      outputs:
      - attack_or_save
      - damage
      - condition_changes
      - forced_movement
      events:
      - attack.pending
      - saving_throw.pending
      capabilities:
      - attacks
      - checks
      - grappling
      - movement
      - damage
    requires:
    - dnd2024:glossary_unarmed_strike
    citations:
    - SRD 5.2.1 pp. 190; builtin/srd-5.2.1/glossary/unarmed_strike.md
  dnd2024:glossary/unconscious_condition:
    source_sha256: 6af1414a5f0f66bc9db217f32308ef3c01e3f71231a66ebcfc00e7b503e4897f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/unconscious_condition
        implementation: dnd2024:glossary_unconscious_condition
        parameters:
          condition: unconscious
          grants_condition: incapacitated
          automatic_save_failure_abilities:
          - strength
          - dexterity
          incoming_attack_rolls: advantage
          automatic_critical_if_hit_attacker_within_feet: 5
          speed: 0
          speed_can_increase: false
          also_apply_condition: prone
          drop_all_held_items: true
          prone_persists_after_end: true
          aware_of_surroundings: false
    contract:
      id: dnd2024:glossary_unconscious_condition
      inputs:
        subject: participant
      outputs:
      - condition_effects
      events:
      - condition.applied
      - condition.ended
      - attack.pending
      - saving_throw.pending
      - speed.calculated
      capabilities:
      - conditions
      - checks
      - attacks
      - movement
      - effects
    requires:
    - dnd2024:glossary_unconscious_condition
    citations:
    - SRD 5.2.1 pp. 191; builtin/srd-5.2.1/glossary/unconscious_condition.md
  dnd2024:glossary/unoccupied_space:
    source_sha256: 86168bd1bc9b45130049d47bdf7f94c73c6c8965f666d1970bb7cfe1395a1f39
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/unoccupied_space
        implementation: dnd2024:glossary_unoccupied_space
        parameters:
          predicate:
            all:
            - no_creatures_in_space
            - space_not_completely_filled_by_objects
    contract:
      id: dnd2024:glossary_unoccupied_space
      inputs:
        location: location
        occupancy: resolution
      outputs:
      - unoccupied
      events: []
      capabilities:
      - geometry
    requires:
    - dnd2024:glossary_unoccupied_space
    citations:
    - SRD 5.2.1 pp. 191; builtin/srd-5.2.1/glossary/unoccupied_space.md
  dnd2024:glossary/utilize_action:
    source_sha256: 00fcbec67d7b7392c51c50c7e11bbf91c5b704f445817caf6e615d6c811c4db0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/utilize_action
        implementation: dnd2024:glossary_utilize_action
        parameters:
          normal_object_interaction: alongside_other_activity
          object_use_requiring_action: utilize_action
          cost: action
    contract:
      id: dnd2024:glossary_utilize_action
      inputs:
        subject: participant
        object: object
        use: choice
      outputs:
      - object_use
      events:
      - action.requested
      capabilities:
      - action_economy
      - objects
    requires:
    - dnd2024:glossary_utilize_action
    citations:
    - SRD 5.2.1 pp. 191; builtin/srd-5.2.1/glossary/utilize_action.md
  dnd2024:glossary/vulnerability:
    source_sha256: 157154c8537b35e75afc3d4736ec5154258ffc83b07109f0598a7f64045cc83b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/vulnerability
        implementation: dnd2024:glossary_vulnerability
        parameters:
          matching_damage_multiplier: 2
          applications_per_damage_instance_max: 1
    contract:
      id: dnd2024:glossary_vulnerability
      inputs:
        subject: object
        damage_type: choice
        damage_amount: integer
      outputs:
      - damage_amount
      events:
      - damage.pending
      capabilities:
      - damage
    requires:
    - dnd2024:glossary_vulnerability
    citations:
    - SRD 5.2.1 pp. 191; builtin/srd-5.2.1/glossary/vulnerability.md
  dnd2024:glossary/weapon:
    source_sha256: bece61ae4a6b3751dba4acc62af7e562a785e9beb835cceaef28c1c3d57a4778
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/weapon
        implementation: dnd2024:glossary_weapon
        parameters:
          object_categories:
          - simple
          - martial
    contract:
      id: dnd2024:glossary_weapon
      inputs:
        object: item_instance
      outputs:
      - is_weapon
      events: []
      capabilities:
      - equipment
      - targeting
    requires:
    - dnd2024:glossary_weapon
    citations:
    - SRD 5.2.1 pp. 191; builtin/srd-5.2.1/glossary/weapon.md
  dnd2024:glossary/weapon_attack:
    source_sha256: b8a1f96e79c5d7a59cdb44c5d36b44a0713fdaf6b23834d91e7e738c9e1e31eb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:glossary/weapon_attack
        implementation: dnd2024:glossary_weapon_attack
        parameters:
          test_kind: attack_roll
          uses_weapon: true
    contract:
      id: dnd2024:glossary_weapon_attack
      inputs:
        attack: resolution
      outputs:
      - is_weapon_attack
      events: []
      capabilities:
      - attacks
    requires:
    - dnd2024:glossary_weapon_attack
    citations:
    - SRD 5.2.1 pp. 191; builtin/srd-5.2.1/glossary/weapon_attack.md
---

# SRD glossary conversion contracts

All 155 assigned glossary notes are accounted for: 144 specified contracts and 11 informational terms. Each entry cites its official bundled SRD 5.2.1 source and records the SHA-256 of that exact file. All source paragraphs and tables were read; no third-party rule text was used.

The informational terms are adventure, campaign, character sheet, creature, encounter, hazard, monster, nonplayer character, player character, spell, and save. These entries only define vocabulary or refer to another rule. Definitions that establish a relationship, eligibility predicate, GM decision, resource rule, or numerical effect have contracts even when brief.

The parameters are source-specific typed contracts. Symbols name rule states or semantic operations; expression nodes use `op` and `args`, with `var` for contextual numeric facts. Dice expressions are typed dice values. Each contract's closed input types follow the conversion brief. Optional inputs are explicitly marked. Runtime statistics, source-effect values, effect provenance, and referenced rule behavior are read from the supplied participant, object, definition, effect, or resolution. An empty event list denotes a query or classification rather than an invented timing trigger. No engine implementation is included.

GM and player discretion remains explicit: Influence receptiveness and checks, fragile object destruction and material effectiveness, climbing and swimming checks, low obstacle checks, passive notice, static Initiative, static monster damage, relationship designations, and area origin choices. Spatial contracts require an external geometry resolution with the stated facts; they do not claim that a map engine exists.

The six area shapes inherit the glossary area obstruction rule. Emanation origin exclusion refers to the originating creature or object, and its area moves only when the effect is neither instantaneous nor stationary. Cover selects the most protective degree. Difficult Terrain costs do not accumulate with other sources of Difficult Terrain; crawling, climbing, and swimming use the source's total costs of two feet per foot, or three in Difficult Terrain, before the special speed exception. Special speed effects propagate the same operation, including setting zero or multiplying, as the source examples require.

The 2024 distinctions are preserved: Concentration damage saves cap at DC 30; Exhaustion reduces D20 Tests by twice the level and Speed by five feet per level; Short Rest Hit Dice heal at least one HP per die; Long Rests restore all spent Hit Dice and require a further hour per interruption when resumed; a knockout leaves one HP and begins a Short Rest; Stunned does not set Speed to zero; Petrified grants Poisoned-condition immunity without inventing Poison-damage immunity; and Invisible attack benefits depend on whether the other creature can see the subject.

Glossary entries that refer to a broader rule retain the local clause and canonical glossary rule reference where available. Their contracts do not import unstated paragraphs from another chapter. Thus the Death Saving Throw note specifies the turn-start obligation, while the full death-save roll outcomes belong to the combat rule source. Stable supplies the stated exception. Likewise, the Temporary Hit Points glossary specifies the damage buffer; full resource lifecycle rules belong to their primary combat source. Source-specific overrides remain available wherever the glossary explicitly allows them.

No unresolvable source issues were found. Fixed examples in formula-based rules are represented by the general expression that yields those examples. Suggested tables remain suggestions with a constrained GM selection instead of becoming mandatory values.

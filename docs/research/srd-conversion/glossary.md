---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:glossary/ability_check:
    source_sha256: fa99356ed047671a24df3ade83d7b39fa39a46453c9b1c9466e48c0fb716c57b
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
    source_sha256: 955629953247ce620f501330b37ccafaa59ca9c692646d9c700c6c066272e9d8
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
    source_sha256: 4cd800fb70347d48d12b2907718d0606cb035ae6c8ed69c71abc449b6ba80041
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
    source_sha256: 7ed18e103f3853125922490fae743da064bf665c079e566fa4e8288f44d20d69
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
    source_sha256: ca4fe87bc18e46803a21419882c092d2a046674e9a8f1be3b611f922b4597a42
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 176; builtin/srd-5.2.1/glossary/adventure.md
  dnd2024:glossary/alignment:
    source_sha256: 96b2ddea5b15aec0cacbd44cc3a799ae2489148a1722c9855e3776cf5f32af45
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
    source_sha256: cbcfc2896436c19a3240ca3fe22be37f791c5c478335fa5fff33994bfbc5f27e
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
    source_sha256: 2cff4adf7ddbdca98021cea84e21bac18a67e9d7f0bae610a9bdee6ef9439c07
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
    source_sha256: cbea842053ef48fdc7e21f7e09f32af128b7bc98d2ec42fba3196fb32c0e760c
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
    source_sha256: f2614bc1429a01ed78f07352f5b44b66b3d2a3e1a50550460830c98dba488948
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
    source_sha256: dc2ec93416127de9e1cab11b4e7e93179b94cd8cc5f765e5c6187f67ab0e2541
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
    source_sha256: c64e4e1a9b83571949597eed7af3abdae97cdc916d878b25340bccb2f8c0cd88
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
    source_sha256: 999ad916454b76ecc0fe4ff59b5e966e2416304c831e79a975088977d19a2415
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
    source_sha256: ad35d382353e53a04ef21677d60bc88a74f88887f61f91707e2a0f1fb4a34a75
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
    source_sha256: 7e115e1b6e7d695d242d5f811a127743c527f1e073ac34da52f70ba83e742975
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
    source_sha256: decf8dad4f665ef326519425939e0b094a49183fdeebda44966a379b12fd0f5f
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
    source_sha256: 88bc142c7ad1e377f230e3ca3cec0f6d9f617d8400377fdf0a7e28c88523e9d7
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
    source_sha256: 0eb2d6370176ad02396148f3d9cf73ac0a26c7ef080652284037fa8cd77e20ef
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
    source_sha256: a40662a4ea00320220c627b44620044d77694b47c73fceba295e3a4a338a8eac
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
    source_sha256: f17af6d918f0a7ab83afb459739293328e91bef0750b0c87fc51b02bacb0b83a
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
    source_sha256: 7e2b93a344b32e9a73bafaab9a21bada84cdd56c1d1d7b1839dd23de0cd7ec73
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
    source_sha256: a5ed4a250e48ac7195b67a3357f340291c8e6a769c0aacce44ea0bcf4a4c0dce
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
    source_sha256: 1e34272dbc819b3642d3581794b51464cf3ef7fbe93f0b33552cf75436623a0d
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 178; builtin/srd-5.2.1/glossary/campaign.md
  dnd2024:glossary/cantrip:
    source_sha256: c5165558a7f426f5abbbca8a4bda63a1cce8749221367800e90750a8834c1c14
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
    source_sha256: f1fffebbf3565eb743ed8a81f47932427853582187da4e306a873578b241ab75
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
    source_sha256: 98809824128f1786c273d4180b33a91ee7087d676c8dafd59ca303cb59490619
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
    source_sha256: c690fc7673f580a47188a5f218a16ebce3ad218ae4904a2327cf9ee505cac816
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 178; builtin/srd-5.2.1/glossary/character_sheet.md
  dnd2024:glossary/charmed_condition:
    source_sha256: 6fbfd1c480c1b88e68861e63ca6c6a3c0f2f214f491bb801e1e2aa6fe2fbd31b
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
    source_sha256: a0847713257bb642e015e9e57167f493595c0049f654b0a34286d40ecaa2c809
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
    source_sha256: b5266117e9ba1d969c0f90aec315dad0332f4230d97921da4e7d0d480551e0a0
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
    source_sha256: 662623284a9edd7dc71f37091ba65978a67c59e4cc25b04f43d744feec61dfcd
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
    source_sha256: e47620e834d8a97c8ffd465239407526be258de3feba04a491002ab2d9adff8b
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
    source_sha256: 71b2172d036bfd1e4ce389b783a61383f452bfa4953ecbd0d407f0540b1f577b
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
    source_sha256: 7c3ab23e6b63d0449700ae6aa4c1f740c0049bb2f648708802c1963ca0df3a1d
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
    source_sha256: c1bcb47c54da41124e6bdaaa0eb75430a2e768965f2584aebe264970847b12c7
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
    source_sha256: cfac6ca232414ef81c09f72c0848571132c46f471c18ab384d2a271950121c7f
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 179; builtin/srd-5.2.1/glossary/creature.md
  dnd2024:glossary/creature_type:
    source_sha256: b567f8fadf2d32c32edd57959c931eded9ba55c622339aa59988af70acb708f2
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
    source_sha256: 2a3e8644d33f461812490abc45ac5421eb53c6f16b55caf46018228cf3bdbf3a
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
    source_sha256: 14cf3a8848c8bd7f7e30895579155e17561fd37922c121f17b251e005ca2a9bf
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
    source_sha256: 26e17afabe84448e6dff9b83338a57f9293cfdbbfddb9beb251814e54b3bddb2
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
    source_sha256: 8b4441427e2a992cfb9fd6f0fdfb8d513c425967c0842fc15f3590b6c9f60b8d
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
    source_sha256: 3b209e986a1277140e1a5fd84aa0969542970a51571bfc1cd6d4917878ab2ba7
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
    source_sha256: d2e93bd8c69d34b9a33d61da1fda1a2b2fc377d35e56fb74748ebe6960801979
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
    source_sha256: 51f66069039814331c6d13935c65de8f87c2fff171c745bf8e74b225da8cd03a
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
    source_sha256: 69e079b4d9e0b47a3e347bea3e653b160647679bc999e7760bf19de22447c3bf
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
    source_sha256: b1e45a59decf78e63f3060af1386cbde4b8acde446b49209d828b33d5407197c
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
    source_sha256: 36a6a553a20ddd686d4d8fc6357102e575aca5fe0a145f2345566ca43c0cbe5e
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
    source_sha256: fbeba4e4d69d31a37a4e0ff8779d4ad2746a88f3f92d8dacf03200d2a42247a0
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
    source_sha256: 462d6ef6638b908cdf5c203e662bac037c37519ee503d922403ded22483a6a7d
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
    source_sha256: 88f1ac9ec1b9c54d7947763cd68b5cc52c2417fced404b790c76f3bbc03ab2be
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
    source_sha256: 76ee58d789013c1355dde4bdc44fae380984abf3f1bcdd1cd731f963389f4e97
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
    source_sha256: 25abab64e3cc6bfcb0fae16431d42b403297d526bb8caf2b27179730f081317e
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
    source_sha256: 320316bcfc74a2880bdafa626e98d17f7e6f63e399a0452c76e2c4118cfa3ad2
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
    source_sha256: a51be1d3d8cb03c4b73dda390f1799ca23b81624286f0b19ef0b921b39bf7796
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
    source_sha256: a9fe720c7d5c5f5f233e5f430167399737fcf0a64f771ad478df906a4e98bae3
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
    source_sha256: b68185f19a186d672af652600989956bcb4a85ce9382705d6dbb74af42b0c57e
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
    source_sha256: 070e4c0d99c2c5087915ace2efb09b5772c516aa7a9dc256cdba3e12ce66f33d
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
    source_sha256: ce2f434e18a8e92e55fad7ecc305804d6ca8a1e259164a323e93d0c691eaf0d5
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
    source_sha256: 124bf2343033129fd7037fca69b382c78e26bc066611dc78a3754c14c1683ae8
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
    source_sha256: 5029804616f9dc81dbe159c475846de39e050bc09b1a0d17869a4e119a91a4cd
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
    source_sha256: 2c4273c42b77b7a007dd4f163b13977bb38699a923099cd8a4040021fd1d83a2
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 181; builtin/srd-5.2.1/glossary/encounter.md
  dnd2024:glossary/enemy:
    source_sha256: e5b989c2fbdefce4dfc1c9a30e46a36a55f07ae715828f556fcc074e16fd52b7
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
    source_sha256: aeaf9c0cf994826102f2571bab9b3945e5e3ab77971809ce59bf4810c2cfd61a
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
    source_sha256: d62530ad0ca64705c4630f2888ef8cde84ab605d6ea0519668dbf03449074b1c
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
    source_sha256: d05a1b73053fe97fbcd1b52c9d17634a72ea8f0432aad652d4ee363688a861bb
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
    source_sha256: 6ba067436f7139fad830ee3506ece7254125d6d3e59d48246b108d9cbfdfebed
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
    source_sha256: 270e6342c93c9543aa61e584e7e17059f02a40cae4d0f69360f900d7bf5d470d
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
    source_sha256: 3bbf5f5520f8687d23c832f34543534b5909441c75ab747b391ef8945e6f0bda
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
    source_sha256: 56aae3937f126bcf05793f5c51a984cfcf6a0642296cf445582f6c443310f8ac
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
    source_sha256: 4829309ccd3632e66c50199c77bd122dbd1e9b0d7a549ef064e9bfde705e6d83
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
    source_sha256: f688e0016ec9f5e765995692e1634aeddf75f5d7ce4b9d4bd5693a7f403660bc
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
    source_sha256: e842d8e862f56d587d4ddcd766ce32578ecd291dd7aa5a4d3e34930630c34064
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
    source_sha256: fedb58c7e11a6294860404740da5cb151246cb5a9befc696578cf53a5eaa1554
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 182; builtin/srd-5.2.1/glossary/hazard.md
  dnd2024:glossary/healing:
    source_sha256: b9fbd7e9b38060b4331277c0c243a765e1121418127da6934c9392b688e6e877
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
    source_sha256: bb89d7001ea12e7a99ecbbd98b5d08ee6a7cf9a6458a81037ceca295e86ec7f7
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
    source_sha256: 9eef828c9de0852f955fabba3a5ee5e554b19557aa68c400890a34fde766df7a
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
    source_sha256: 8d40f963b76e9bb7def25ca99f02f4e8b7e56dde1533497f55d176810aa542a0
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
    source_sha256: a45cb67e06677319ef7425984cd0d1baaacdd40d8afef7e72a4d28c1c5e8510c
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
    source_sha256: 9bc5f1673fe0068066b009455c85b1c8e61093edaa619e9cccd406e5a7255954
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
    source_sha256: 2c578d7324957c7653861389f4eb293e741ed83b5a8c961bd8bec5e7c271579c
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
    source_sha256: f37826e79d03ba8bdad9903ffb27f1177df82b45a6ed7304de92642616e0c607
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
    source_sha256: bbb3c99ccf138873d5677a066f226e8371045ea232ec7e75238f75f2009191cd
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
    source_sha256: c088906aff22ac59925111aabd0a444b75911c193121710e5190524969f93244
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
    source_sha256: fb75d84cff4622d895ba60115da8b834406ad654eb447aef6758d22f08b00513
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
    source_sha256: bbb56ad5ebfd04aaf830e699e80ed58afb61ebcaa53f5629bf961abc4d38d9ef
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
    source_sha256: ce7230d02ca8e81eca7314fe7bd5fc6c6fa1ff96c898b1afe66afa928a703919
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
    source_sha256: 4f8be409c29d3a8135ec0122b26b27b11b183645ab2b2bfba1f5f3a5e7e617ff
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
    source_sha256: 8ca75566f23730fe0a515c744724c43566503631667237474bbe07ad75b1eed1
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
    source_sha256: dd498caa2071ae377db562fb5a756f2b9f0632b0417c8a5ff8039b70f1be5d4f
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
    source_sha256: 90beec3c8c0f6c49fd70394f0546251ea865d1c5b798fe94b898d88cd266c897
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
    source_sha256: 966770ea7077c94c26d5d4258537a067d58a133f8f0edd8ac8f88ab759055f39
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
    source_sha256: 6fab9d0b7eaf340646ac06317362a58b4ddd5dcd4ab006c182088665965df77b
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
    source_sha256: f0ae3847351073d8804df5c618bebf13fb7eb63d79b951d4cf67af400f280f62
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
    source_sha256: 78d046a29a9d61c1d755d1bbc5976ba61d1fae8765a50759a02e4cd958d815ca
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
    source_sha256: e955b5cbe88415a20e7d5bf34747393bc42484df1ddcbf2769090def0380ddfe
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
    source_sha256: d304f5d62ffa72f598e97ff68655986733796b660d3952f945e78741aa673f85
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
    source_sha256: 4e4e0abc65c3f01876ff8eae6a5615b6ff336eee4c5b10eafa0bbceb31df057f
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
    source_sha256: 549cde5bcf69b36101b83e7b8a2bf04cc02202703b465887d7ae746e2d830fc0
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
    source_sha256: 687cc17af15ba22430db2b0ec060268817253490bcaf27e60c177a4ba53e7151
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
    source_sha256: 8c37143c4f54247a4ac6aa678c20b81bf9ef93ba8c01f755679e77881d11e8e4
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
    source_sha256: 858bc4e95402ccbd60987da7bcae09acf166475c18ea9fe2fc1cae72346ba3c6
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 185; builtin/srd-5.2.1/glossary/monster.md
  dnd2024:glossary/nonplayer_character:
    source_sha256: 79768b78fd5ec544e2c333bf5c108a8730fb4a9684ec7969a5ee8e66bd9112ca
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 185; builtin/srd-5.2.1/glossary/nonplayer_character.md
  dnd2024:glossary/object:
    source_sha256: 107504409b1f984a51a2cb3f4268fb6820f66306a304b7dd120c9f8b2b81af21
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
    source_sha256: 132f8a1e1138f7021be185e655ff572989dc80ba5299706fd19e567333692d54
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
    source_sha256: 34b8597db2c9099605e0c1e9687c18b052eda46adcdfba06870096ad1edf05ea
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
    source_sha256: 2d0ef77b7b49a42f0e3c206f772fee99ef2b16e5812ea228481ca0962226effc
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
    source_sha256: a826528d78f7f7660a657004949e7c998dd37b0274dde2e4af19892d0dd81647
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
    source_sha256: 41066a956aafce5c95a24a880dac1ce0717013f08ea0a7f50f8f548d2433e2b0
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
    source_sha256: 81235bdd13d9d7b5a766f72f12c8f05b377c2a60db2efbf637028b1591af9ff8
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
    source_sha256: 4667174d8cc265c61f0cb3ec6d7f9dc1b95082bd1f8b2d5189253f3e891a9c2a
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 186; builtin/srd-5.2.1/glossary/player_character.md
  dnd2024:glossary/poisoned_condition:
    source_sha256: b7bd4f0c0355022554c9f1098256154598de133a8f66a25398335d4db40f14d2
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
    source_sha256: 4327a8caa069369042bd3e7c53e4da5bfd2effe596d66aa0bebdb80d5ad95e47
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
    source_sha256: 1916a99d10a2a7544870e409672f337d097ff532fde15d799b7487e87d8b04a7
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
    source_sha256: 67efb408de5d976d08072ccfbb11f231143d854545028d507cdc13a8d902dc1e
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
    source_sha256: 8bd75e6db38f8a04e1a5cc99ce5e28eb0166a74526f7a80a1075bd2fe864e6b2
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
    source_sha256: 2913d47849424dbc5c2f7aa27792e38e2c5ba0037474670d15e22245d289d759
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
    source_sha256: dbd0ef5b8bd651769728f7b39b949c97d41914dcfbc1e67f9898f9f79809dbe2
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
    source_sha256: 4ee20788a39ea4345c92c113a80c5b096d47d4ab0bfe3b637ec51d0723f495ca
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
    source_sha256: 4b90918a764a41330f301350347358607849795b0e57d5dac0aa529ef8843fde
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
    source_sha256: 4308b07209e699af38ad243cc023cc72c972c5f6fbd034bf3e3b99d79ba91623
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
    source_sha256: 5b12abbcd5de6b0385124b1075ad05c761c65c56cbf704626438dc5eb588cdc9
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
    source_sha256: a89de607388b8c1598d5a2b137a9cc55f23c64bc2b216a74a838383350485ff9
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 187; builtin/srd-5.2.1/glossary/save.md
  dnd2024:glossary/saving_throw:
    source_sha256: 978ffcce87b7db85415c391320fa20166e6de4ed67e2faf10b812b8bd450026d
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
    source_sha256: a955135f094a95cdc513a828fcf0a8f0208a66d323ad36356f50bbe8c7d97b59
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
    source_sha256: c1c9eaea457d6f55d754c0f3df0d0e02865154eb3a5505e1bba59e4259b66aeb
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
    source_sha256: 840fa907fc376bd30d42c5c00f0501337290b70fbfc4b7da7a397ef67d7c6db0
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
    source_sha256: 3409536fa1a029576fbf77eefca1587112e9fb7541a8dc94cd1bcb8e274e6680
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
    source_sha256: 1173718e264f3abb700eeb380845672c17f3a50e2494b4909ad0dee3861b0adb
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
    source_sha256: ec2fff76987d62c3d07d2e7fe1cbcba8c19f1300e703053665c3bb983637606f
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
    source_sha256: 149dddf08db6b7346acd4a0de6c9d9e42301d0e821371a0357a74b0a8962199a
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
    source_sha256: 87b6fd4e7d2dde0da041b051f1d6f987475aa72bf8d3d763168f6bdfe72b19b7
    disposition: informational
    fields: {}
    requires: []
    citations:
    - SRD 5.2.1 pp. 188; builtin/srd-5.2.1/glossary/spell.md
  dnd2024:glossary/spell_attack:
    source_sha256: ce8e48e0351699f160f0bae3cc30bbd9947d5fa53eab7c2911243e5b665865b1
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
    source_sha256: a0a857233cd742566071dba320949c15b540ef7ebe567fcb34ea04dda2359d2c
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
    source_sha256: d7c7f60ecff3b5e5fc2eae1799b5c28b422bc856fe487023f0da1a0555bc921c
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
    source_sha256: eb5b462041bb30e429716381226e5cf40d0bd75b027ab8f5c3bdc39b95b2021d
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
    source_sha256: 76e6f94813d153963785505a9d6555179800bedf17f18133fff1a77ca2879ae7
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
    source_sha256: eef04398dda445a67d7853a507cc5f9c734875e81067c3d0f0cf1fd465dc9e9e
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
    source_sha256: a240045c9ae11accd32a6776700f40fe3e0e4609c87852930d90dd0dfc7f6741
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
    source_sha256: b8669eeece2ec5f36469baf763fa607be558e02d9390bb641b93f09e26e7e9c9
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
    source_sha256: 8904555c4d1973b50c3088d7ba8e2a647732bff28a1db11b8977688ff5159a6d
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
    source_sha256: 077371c509b78fa2573a96d49cd030931f09c3333014736b6cb3a035aa122b3c
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
    source_sha256: 8b6a9ca0c7e84e9e70978cb48121bf4460aa95e858b3d2a8904be5c38f360a2b
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
    source_sha256: 491c31c40ea5621b1e4ffdb2fd048408b93a3a183e6dd1d9983f894d8a795a28
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
    source_sha256: 0fffce87b8688f3c3922c6549a4cf1d802987193d2d2674d9fc0be3359ffec8f
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
    source_sha256: 075853fb258551ece6b4c0bc0670fe2d92678d1d1d9a7b0bc53d471961fef114
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
    source_sha256: 68d3cd101991ed31a511ed269c39368d1c919b81b7b56a99a5127bf12ac2570f
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
    source_sha256: 27a8e14dd5607b76c576c180737713d3cbe34cc730f96b2cec8848e29a66034d
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
    source_sha256: aeb8eac76b8d6945b91e0861c6407d23f10a809bfbeb892d2e9528ee2b75b23d
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
    source_sha256: 811ca5f9d97fe87ddced379af3ce02471245923e83c07440857ed50d74b5a7e9
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
    source_sha256: bd36c02e42af5a2c568edd96a0feff7256e0cf79c73b724c05fbc742e18de2a0
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
    source_sha256: 099150ba379c62032c45648f42b11621b63e86fd1e4a676f4722323b76d668e9
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
    source_sha256: f36cfb74ba13b09104f69e02011df61aec604ffe9b3da7e4fdac807a23423364
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
    source_sha256: f63ac1ea3771bee015b8e920b722ae473be46393147362c05c8d2ea8fa2d75be
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
    source_sha256: cf45a03735227bb29b87586f89f1f5615400a0b1c6fe7895b933c89b7dddb07a
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
    source_sha256: 7bfb1f2041ba8b27fb69b9e3b6db68ed27a9482b620244a480f8c77d02311729
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

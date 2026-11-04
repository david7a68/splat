---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:creatures/adult_blue_dragon:
    source_sha256: 21907431fb8a3951e8a925e8b327ca44290b61a7997a8c6d2e19bc54254f642c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/adult_blue_dragon
        implementation: dnd2024:creature_adult_blue_dragon
        parameters:
          skill_totals:
            perception: 12
            stealth: 5
          damage_immunities:
          - lightning
          condition_immunities: []
          senses:
          - sense: blindsight
            range_ft: 60
          - sense: darkvision
            range_ft: 120
          passive_perception: 22
          languages:
            speak: true
            known:
            - common
            - draconic
          features:
            legendary_resistance:
              economy: passive
              trigger: saving_throw.failed
              optional: true
              op: saving_throw.set_result
              result: success
              uses_in_lair: 4
              uses:
                count: 3
                recovery: long_rest
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 12
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 7
                  average: 16
                - type: lightning
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 0
                  average: 5
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/invisibility
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/mage_hand
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/shatter
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/scrying
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/sending
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 18
            lightning_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 19
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  origin: actor
                  size_ft: 90
                  width_ft: 5
              failure:
              - op: damage
                components:
                - type: lightning
                  amount:
                    dice:
                      count: 11
                      sides: 10
                    add: 0
                  average: 60
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            cloaked_flight:
              economy: legendary_action
              op: sequence
              steps:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/invisibility
                profile: spellcasting
                target: self
              - op: movement
                maximum:
                  multiply:
                  - 0.5
                  - actor_stat: fly_speed
                optional: true
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            sonic_boom:
              economy: legendary_action
              op: sequence
              steps:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/shatter
                profile: spellcasting
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            tail_swipe:
              economy: legendary_action
              op: feature.use
              feature: rend
              legendary_cost: 1
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 3
                feature: rend
              replace_attack:
                maximum: 1
                choices:
                - op: spell.cast
                  spell:
                    key: dnd2024:spells/shatter
                  profile: spellcasting
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            trigger: another_creature.turn_end_immediately_after
            cost_per_action: 1
            recover_all: actor.turn_start
            maximum_per_trigger: 1
            requires_actions_available: true
            excluded_conditions:
            - incapacitated
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_adult_blue_dragon
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - another_creature.turn_end_immediately_after
      - creature.feature_requested
      - saving_throw.failed
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_adult_blue_dragon
    citations:
    - SRD 5.2.1 pp. 266, 267; builtin/srd-5.2.1/creature/adult_blue_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/adult_copper_dragon:
    source_sha256: 0869be9a1a1c1a65f2bfeb679ecc574d51fd53223061a53a67d1277d57ed0c0a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/adult_copper_dragon
        implementation: dnd2024:creature_adult_copper_dragon
        parameters:
          skill_totals:
            deception: 9
            perception: 12
            stealth: 6
          damage_immunities:
          - acid
          condition_immunities: []
          senses:
          - sense: blindsight
            range_ft: 60
          - sense: darkvision
            range_ft: 120
          passive_perception: 22
          languages:
            speak: true
            known:
            - common
            - draconic
          features:
            legendary_resistance:
              economy: passive
              trigger: saving_throw.failed
              optional: true
              op: saving_throw.set_result
              result: success
              uses_in_lair: 4
              uses:
                count: 3
                recovery: long_rest
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 11
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 10
                    add: 6
                  average: 17
                - type: acid
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 0
                  average: 4
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/mind_spike
                  uses:
                    unlimited: true
                  cast_level: 4
                - spell:
                    key: dnd2024:spells/minor_illusion
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/shapechange
                  uses:
                    unlimited: true
                  overrides:
                    forms:
                    - beast
                    - humanoid
                    temporary_hp_granted: 0
                    maintain_requires_concentration: false
                    maintain_requires_temporary_hp: false
                - spell:
                    key: dnd2024:spells/greater_restoration
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/major_image
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 17
            acid_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 18
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  origin: actor
                  size_ft: 60
                  width_ft: 5
              failure:
              - op: damage
                components:
                - type: acid
                  amount:
                    dice:
                      count: 12
                      sides: 8
                    add: 0
                  average: 54
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            slowing_breath:
              economy: action
              op: saving_throw
              ability: constitution
              dc: 18
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 60
              failure:
              - op: effect.apply
                duration:
                  until: target.next_turn_end
                rules:
                  reactions_allowed: false
                  speed_multiplier: 0.5
                  turn_economy_choice:
                  - action
                  - bonus_action
                  both_allowed: false
              success: []
            giggling_magic:
              economy: legendary_action
              op: saving_throw
              ability: charisma
              dc: 17
              targets:
                kind: creature
                count: 1
                range_ft: 90
                visible_to_actor: true
              failure:
              - op: damage
                components:
                - type: psychic
                  amount:
                    dice:
                      count: 7
                      sides: 6
                    add: 0
                  average: 24
              - op: roll.add
                amount:
                  negate:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                rolls:
                - ability_check
                - attack
                roll_each_test: true
                duration:
                  until: target.next_turn_end
              success: []
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            mind_jolt:
              economy: legendary_action
              op: sequence
              steps:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/mind_spike
                profile: spellcasting
                cast_level: 4
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: movement
                maximum:
                  multiply:
                  - 0.5
                  - actor_stat: speed
              - op: feature.use
                feature: rend
              legendary_cost: 1
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 3
                feature: rend
              replace_attack:
                maximum: 1
                choices:
                - op: feature.use
                  feature: slowing_breath
                - op: spell.cast
                  spell:
                    key: dnd2024:spells/mind_spike
                  profile: spellcasting
                  cast_level: 4
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            trigger: another_creature.turn_end_immediately_after
            cost_per_action: 1
            recover_all: actor.turn_start
            maximum_per_trigger: 1
            requires_actions_available: true
            excluded_conditions:
            - incapacitated
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_adult_copper_dragon
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - another_creature.turn_end_immediately_after
      - creature.feature_requested
      - saving_throw.failed
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_adult_copper_dragon
    citations:
    - SRD 5.2.1 pp. 276, 277; builtin/srd-5.2.1/creature/adult_copper_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/adult_red_dragon:
    source_sha256: 4c5d749ef81ae07f7c0dd895bbc48cd70c9298fe345d655f914010d75102569f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/adult_red_dragon
        implementation: dnd2024:creature_adult_red_dragon
        parameters:
          skill_totals:
            perception: 13
            stealth: 6
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
          - sense: blindsight
            range_ft: 60
          - sense: darkvision
            range_ft: 120
          passive_perception: 23
          languages:
            speak: true
            known:
            - common
            - draconic
          features:
            legendary_resistance:
              economy: passive
              trigger: saving_throw.failed
              optional: true
              op: saving_throw.set_result
              result: success
              uses_in_lair: 4
              uses:
                count: 3
                recovery: long_rest
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 14
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 8
                  average: 13
                - type: fire
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 0
                  average: 5
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/command
                  uses:
                    unlimited: true
                  cast_level: 2
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/scorching_ray
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/fireball
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 20
                attack_bonus: 12
            fire_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 21
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 60
              failure:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 17
                      sides: 6
                    add: 0
                  average: 59
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            commanding_presence:
              economy: legendary_action
              op: sequence
              steps:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/command
                profile: spellcasting
                cast_level: 2
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            fiery_rays:
              economy: legendary_action
              op: sequence
              steps:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/scorching_ray
                profile: spellcasting
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: movement
                maximum:
                  multiply:
                  - 0.5
                  - actor_stat: speed
              - op: feature.use
                feature: rend
              legendary_cost: 1
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 3
                feature: rend
              replace_attack:
                maximum: 1
                choices:
                - op: spell.cast
                  spell:
                    key: dnd2024:spells/scorching_ray
                  profile: spellcasting
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            trigger: another_creature.turn_end_immediately_after
            cost_per_action: 1
            recover_all: actor.turn_start
            maximum_per_trigger: 1
            requires_actions_available: true
            excluded_conditions:
            - incapacitated
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_adult_red_dragon
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - another_creature.turn_end_immediately_after
      - creature.feature_requested
      - saving_throw.failed
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_adult_red_dragon
    citations:
    - SRD 5.2.1 pp. 318, 319; builtin/srd-5.2.1/creature/adult_red_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/air_elemental:
    source_sha256: aeb8415bb938277e67b76d122bf849c34ee0956d079ed793e52b9afd58a52266
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/air_elemental
        implementation: dnd2024:creature_air_elemental
        parameters:
          damage_resistances:
          - bludgeoning
          - lightning
          - piercing
          - slashing
          damage_immunities:
          - poison
          - thunder
          condition_immunities:
          - exhaustion
          - grappled
          - paralyzed
          - petrified
          - poisoned
          - prone
          - restrained
          - unconscious
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 10
          languages:
            speak: true
            known:
            - primordial
            primordial_dialects:
            - auran
          features:
            air_form:
              economy: passive
              op: movement.form
              minimum_opening_inches: 1
              extra_movement_cost: 0
              can_enter_and_end_in_creature_space: true
            thunderous_slam:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 8
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: thunder
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 5
                  average: 14
            whirlwind:
              economy: action
              op: saving_throw
              ability: strength
              dc: 13
              targets:
                kind: creature
                count: 1
                maximum_size: medium
                shares_actor_space: true
              failure:
              - op: damage
                components:
                - type: thunder
                  amount:
                    dice:
                      count: 4
                      sides: 10
                    add: 2
                  average: 24
              - op: push
                maximum_ft: 20
                direction: straight_away_from_actor
              - op: condition.apply
                condition: prone
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 4
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: thunderous_slam
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_air_elemental
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_air_elemental
    citations:
    - SRD 5.2.1 pp. 258, 259; builtin/srd-5.2.1/creature/air_elemental.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ancient_blue_dragon:
    source_sha256: 7bc51d31014234d780ce00f4ce88d26cf80752064cbbcff8747287df5f1a6350
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ancient_blue_dragon
        implementation: dnd2024:creature_ancient_blue_dragon
        parameters:
          skill_totals:
            perception: 17
            stealth: 7
          damage_immunities:
          - lightning
          condition_immunities: []
          senses:
          - sense: blindsight
            range_ft: 60
          - sense: darkvision
            range_ft: 120
          passive_perception: 27
          languages:
            speak: true
            known:
            - common
            - draconic
          features:
            legendary_resistance:
              economy: passive
              trigger: saving_throw.failed
              optional: true
              op: saving_throw.set_result
              result: success
              uses_in_lair: 5
              uses:
                count: 4
                recovery: long_rest
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 16
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 15
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 9
                  average: 18
                - type: lightning
                  amount:
                    dice:
                      count: 2
                      sides: 10
                    add: 0
                  average: 11
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/invisibility
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/mage_hand
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/shatter
                  uses:
                    unlimited: true
                  cast_level: 3
                - spell:
                    key: dnd2024:spells/scrying
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/sending
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 22
            lightning_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 23
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  origin: actor
                  size_ft: 120
                  width_ft: 10
              failure:
              - op: damage
                components:
                - type: lightning
                  amount:
                    dice:
                      count: 16
                      sides: 10
                    add: 0
                  average: 88
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            cloaked_flight:
              economy: legendary_action
              op: sequence
              steps:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/invisibility
                profile: spellcasting
                target: self
              - op: movement
                maximum:
                  multiply:
                  - 0.5
                  - actor_stat: fly_speed
                optional: true
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            sonic_boom:
              economy: legendary_action
              op: sequence
              steps:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/shatter
                profile: spellcasting
                cast_level: 3
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            tail_swipe:
              economy: legendary_action
              op: feature.use
              feature: rend
              legendary_cost: 1
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 3
                feature: rend
              replace_attack:
                maximum: 1
                choices:
                - op: spell.cast
                  spell:
                    key: dnd2024:spells/shatter
                  profile: spellcasting
                  cast_level: 3
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            trigger: another_creature.turn_end_immediately_after
            cost_per_action: 1
            recover_all: actor.turn_start
            maximum_per_trigger: 1
            requires_actions_available: true
            excluded_conditions:
            - incapacitated
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_ancient_blue_dragon
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - another_creature.turn_end_immediately_after
      - creature.feature_requested
      - saving_throw.failed
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_ancient_blue_dragon
    citations:
    - SRD 5.2.1 pp. 267; builtin/srd-5.2.1/creature/ancient_blue_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ancient_copper_dragon:
    source_sha256: 649ffc11f0b3bbf7105f1d0c24da170e931e4a7b9765107ea9853bf1204fe52e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ancient_copper_dragon
        implementation: dnd2024:creature_ancient_copper_dragon
        parameters:
          skill_totals:
            deception: 13
            perception: 17
            stealth: 8
          damage_immunities:
          - acid
          condition_immunities: []
          senses:
          - sense: blindsight
            range_ft: 60
          - sense: darkvision
            range_ft: 120
          passive_perception: 27
          languages:
            speak: true
            known:
            - common
            - draconic
          features:
            legendary_resistance:
              economy: passive
              trigger: saving_throw.failed
              optional: true
              op: saving_throw.set_result
              result: success
              uses_in_lair: 5
              uses:
                count: 4
                recovery: long_rest
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 15
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 15
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 10
                    add: 8
                  average: 19
                - type: acid
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 0
                  average: 9
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/mind_spike
                  uses:
                    unlimited: true
                  cast_level: 5
                - spell:
                    key: dnd2024:spells/minor_illusion
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/shapechange
                  uses:
                    unlimited: true
                  overrides:
                    forms:
                    - beast
                    - humanoid
                    temporary_hp_granted: 0
                    maintain_requires_concentration: false
                    maintain_requires_temporary_hp: false
                - spell:
                    key: dnd2024:spells/greater_restoration
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/major_image
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/project_image
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 21
            acid_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 22
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  origin: actor
                  size_ft: 90
                  width_ft: 10
              failure:
              - op: damage
                components:
                - type: acid
                  amount:
                    dice:
                      count: 14
                      sides: 8
                    add: 0
                  average: 63
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            slowing_breath:
              economy: action
              op: saving_throw
              ability: constitution
              dc: 22
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 90
              failure:
              - op: effect.apply
                duration:
                  until: target.next_turn_end
                rules:
                  reactions_allowed: false
                  speed_multiplier: 0.5
                  turn_economy_choice:
                  - action
                  - bonus_action
                  both_allowed: false
              success: []
            giggling_magic:
              economy: legendary_action
              op: saving_throw
              ability: charisma
              dc: 21
              targets:
                kind: creature
                count: 1
                range_ft: 120
                visible_to_actor: true
              failure:
              - op: damage
                components:
                - type: psychic
                  amount:
                    dice:
                      count: 9
                      sides: 6
                    add: 0
                  average: 31
              - op: roll.add
                amount:
                  negate:
                    dice:
                      count: 1
                      sides: 8
                    add: 0
                rolls:
                - ability_check
                - attack
                roll_each_test: true
                duration:
                  until: target.next_turn_end
              success: []
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            mind_jolt:
              economy: legendary_action
              op: sequence
              steps:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/mind_spike
                profile: spellcasting
                cast_level: 5
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: movement
                maximum:
                  multiply:
                  - 0.5
                  - actor_stat: speed
              - op: feature.use
                feature: rend
              legendary_cost: 1
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 3
                feature: rend
              replace_attack:
                maximum: 1
                choices:
                - op: feature.use
                  feature: slowing_breath
                - op: spell.cast
                  spell:
                    key: dnd2024:spells/mind_spike
                  profile: spellcasting
                  cast_level: 5
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            trigger: another_creature.turn_end_immediately_after
            cost_per_action: 1
            recover_all: actor.turn_start
            maximum_per_trigger: 1
            requires_actions_available: true
            excluded_conditions:
            - incapacitated
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_ancient_copper_dragon
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - another_creature.turn_end_immediately_after
      - creature.feature_requested
      - saving_throw.failed
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_ancient_copper_dragon
    citations:
    - SRD 5.2.1 pp. 277; builtin/srd-5.2.1/creature/ancient_copper_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ancient_red_dragon:
    source_sha256: 50551f8f8cf8e0a327188871dc66a29e33c3fbf0f60a8616c11f69432f24f76d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ancient_red_dragon
        implementation: dnd2024:creature_ancient_red_dragon
        parameters:
          skill_totals:
            perception: 16
            stealth: 7
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
          - sense: blindsight
            range_ft: 60
          - sense: darkvision
            range_ft: 120
          passive_perception: 26
          languages:
            speak: true
            known:
            - common
            - draconic
          features:
            legendary_resistance:
              economy: passive
              trigger: saving_throw.failed
              optional: true
              op: saving_throw.set_result
              result: success
              uses_in_lair: 5
              uses:
                count: 4
                recovery: long_rest
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 17
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 15
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 10
                  average: 19
                - type: fire
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 0
                  average: 10
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/command
                  uses:
                    unlimited: true
                  cast_level: 2
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/scorching_ray
                  uses:
                    unlimited: true
                  cast_level: 3
                - spell:
                    key: dnd2024:spells/fireball
                  uses:
                    count: 1
                    recovery: long_rest
                  cast_level: 6
                - spell:
                    key: dnd2024:spells/scrying
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 23
                attack_bonus: 15
            fire_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 24
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 90
              failure:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 26
                      sides: 6
                    add: 0
                  average: 91
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            commanding_presence:
              economy: legendary_action
              op: sequence
              steps:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/command
                profile: spellcasting
                cast_level: 2
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            fiery_rays:
              economy: legendary_action
              op: sequence
              steps:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/scorching_ray
                profile: spellcasting
                cast_level: 3
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: movement
                maximum:
                  multiply:
                  - 0.5
                  - actor_stat: speed
              - op: feature.use
                feature: rend
              legendary_cost: 1
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 3
                feature: rend
              replace_attack:
                maximum: 1
                choices:
                - op: spell.cast
                  spell:
                    key: dnd2024:spells/scorching_ray
                  profile: spellcasting
                  cast_level: 3
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            trigger: another_creature.turn_end_immediately_after
            cost_per_action: 1
            recover_all: actor.turn_start
            maximum_per_trigger: 1
            requires_actions_available: true
            excluded_conditions:
            - incapacitated
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_ancient_red_dragon
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - another_creature.turn_end_immediately_after
      - creature.feature_requested
      - saving_throw.failed
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_ancient_red_dragon
    citations:
    - SRD 5.2.1 pp. 319; builtin/srd-5.2.1/creature/ancient_red_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/animated_armor:
    source_sha256: 532392060f94b534fb672d33848d74feb11aafebc26a1a1bad3ba01cfd15958e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/animated_armor
        implementation: dnd2024:creature_animated_armor
        parameters:
          damage_immunities:
          - poison
          - psychic
          condition_immunities:
          - charmed
          - deafened
          - exhaustion
          - frightened
          - paralyzed
          - petrified
          - poisoned
          senses:
          - sense: blindsight
            range_ft: 60
          passive_perception: 6
          languages:
            speak: true
            known: []
          features:
            slam:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 2
                  average: 5
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: slam
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_animated_armor
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_animated_armor
    citations:
    - SRD 5.2.1 pp. 259; builtin/srd-5.2.1/creature/animated_armor.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/animated_rug_of_smothering:
    source_sha256: a2a67e9daceb4c06206968937cc8af610cbb2f653c6a076dd2984fcf9ed3e78f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/animated_rug_of_smothering
        implementation: dnd2024:creature_animated_rug_of_smothering
        parameters:
          damage_immunities:
          - poison
          - psychic
          condition_immunities:
          - charmed
          - deafened
          - exhaustion
          - frightened
          - paralyzed
          - petrified
          - poisoned
          senses:
          - sense: blindsight
            range_ft: 60
          passive_perception: 6
          languages:
            speak: true
            known: []
          features:
            smother:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: choice
                chooser: actor
                choices:
                - effects:
                  - op: damage
                    components:
                    - type: bludgeoning
                      amount:
                        dice:
                          count: 2
                          sides: 6
                        add: 3
                      average: 10
                - requires:
                    target_kind: creature
                    target_maximum_size: medium
                  effects:
                  - op: grapple
                    escape_dc: 13
                    maximum_targets: 1
                    while_grappled:
                    - op: condition.apply
                      condition: restrained
                    - op: condition.apply
                      condition: blinded
                    - op: suffocation.start
                    - trigger: target.turn_start
                      effects:
                      - op: damage
                        components:
                        - type: bludgeoning
                          amount:
                            dice:
                              count: 2
                              sides: 6
                            add: 3
                          average: 10
              unavailable_while_grappling: true
              while_grappling:
                incoming_damage_multiplier: 0.5
                round: down
                grappled_target_takes_same_damage: true
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_animated_rug_of_smothering
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      - target.turn_start
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_animated_rug_of_smothering
    citations:
    - SRD 5.2.1 pp. 259; builtin/srd-5.2.1/creature/animated_rug_of_smothering.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ape:
    source_sha256: 3a4cff30316e505ac5963bb445e520624884e76b315efdebf7715aeb6b7e6709
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ape
        implementation: dnd2024:creature_ape
        parameters:
          skill_totals:
            athletics: 5
            perception: 3
          senses: []
          passive_perception: 13
          languages:
            speak: true
            known: []
          features:
            fist:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 3
                  average: 5
            rock:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 25
                long: 50
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 3
                  average: 10
              recharge:
                die_sides: 6
                success_min: 6
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: fist
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_ape
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - spatial_constraints
    requires:
    - dnd2024:creature_ape
    citations:
    - SRD 5.2.1 pp. 344; builtin/srd-5.2.1/creature/ape.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/assassin:
    source_sha256: 88931de0357a848de118fff4202e62abc9566cafcc823a8c39024e90819006f0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/assassin
        implementation: dnd2024:creature_assassin
        parameters:
          skill_totals:
            acrobatics: 7
            perception: 6
            stealth: 10
          damage_resistances:
          - poison
          gear:
          - item: light_crossbow
            count: 1
          - item: shortsword
            count: 1
          - item: studded_leather_armor
            count: 1
          senses: []
          passive_perception: 16
          languages:
            speak: true
            known:
            - common
            - thieves_cant
          features:
            shortsword:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 4
                  average: 7
                - type: poison
                  amount:
                    dice:
                      count: 5
                      sides: 6
                    add: 0
                  average: 17
              - op: condition.apply
                condition: poisoned
                duration:
                  until: actor.next_turn_start
            light_crossbow:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 80
                long: 320
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 4
                  average: 8
                - type: poison
                  amount:
                    dice:
                      count: 6
                      sides: 6
                    add: 0
                  average: 21
            cunning_action:
              economy: bonus_action
              op: action.choose
              choices:
              - dash
              - disengage
              - hide
            evasion:
              economy: passive
              when:
                save_ability: dexterity
                success_damage_fraction: 0.5
                actor_excluded_conditions:
                - incapacitated
              op: damage_fraction.override
              success: 0
              failure: 0.5
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 3
                choices:
                - shortsword
                - light_crossbow
                combination: any
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_assassin
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_assassin
    citations:
    - SRD 5.2.1 pp. 260; builtin/srd-5.2.1/creature/assassin.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/awakened_tree:
    source_sha256: 7b0eeb88de45ff43c4f9ff93026db720f57d9efdc9a3d429330be061ff435b9e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/awakened_tree
        implementation: dnd2024:creature_awakened_tree
        parameters:
          damage_vulnerabilities:
          - fire
          damage_resistances:
          - bludgeoning
          - piercing
          senses: []
          passive_perception: 10
          languages:
            speak: true
            known:
            - common
            additional_choice_count: 1
          features:
            slam:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 4
                  average: 14
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_awakened_tree
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_awakened_tree
    citations:
    - SRD 5.2.1 pp. 260; builtin/srd-5.2.1/creature/awakened_tree.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/baboon:
    source_sha256: 9bace352b63fff6cd679213007aaef8efc856823e62d08da67aa86abd25d97f1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/baboon
        implementation: dnd2024:creature_baboon
        parameters:
          senses: []
          passive_perception: 11
          languages:
            speak: true
            known: []
          features:
            pack_tactics:
              economy: passive
              op: roll.advantage
              rolls:
              - attack
              target_kind: creature
              when:
                ally_within_target_ft: 5
                ally_excluded_conditions:
                - incapacitated
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 1
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: -1
                  average: 1
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_baboon
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_baboon
    citations:
    - SRD 5.2.1 pp. 345; builtin/srd-5.2.1/creature/baboon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/bandit:
    source_sha256: 502e6a02bd3f82ce61bce38394d5f0865e2f71f945a8cd98a1a12b4defbc32b5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/bandit
        implementation: dnd2024:creature_bandit
        parameters:
          gear:
          - item: leather_armor
            count: 1
          - item: light_crossbow
            count: 1
          - item: scimitar
            count: 1
          senses: []
          passive_perception: 10
          languages:
            speak: true
            known:
            - common
            - thieves_cant
          features:
            scimitar:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 3
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 1
                  average: 4
            light_crossbow:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 3
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 80
                long: 320
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 1
                  average: 5
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_bandit
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_bandit
    citations:
    - SRD 5.2.1 pp. 261; builtin/srd-5.2.1/creature/bandit.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/basilisk:
    source_sha256: a03334d7576342250dcca788d50e4ea8a01ac9dfbaefd4859c669bf27f26e759
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/basilisk
        implementation: dnd2024:creature_basilisk
        parameters:
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 9
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 3
                  average: 10
                - type: poison
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  average: 7
            petrifying_gaze:
              economy: bonus_action
              op: saving_throw
              ability: constitution
              dc: 12
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 30
              failure:
              - op: condition.apply
                condition: restrained
              - op: saving_throw.repeat
                event: target.next_turn_end
                only_if_condition: restrained
                ability: constitution
                dc: 12
                success:
                - op: effect.end
                failure:
                - op: condition.remove
                  condition: restrained
                - op: condition.apply
                  condition: petrified
              success: []
              include_actor_if_sees_own_reflection_in_area: true
              recharge:
                die_sides: 6
                success_min: 4
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_basilisk
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      - target.next_turn_end
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_basilisk
    citations:
    - SRD 5.2.1 pp. 262; builtin/srd-5.2.1/creature/basilisk.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/behir:
    source_sha256: 1a33b16ef5f16aa36219fa74e773ff9f6e43f28af77c8e1567980bc327590c21
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/behir
        implementation: dnd2024:creature_behir
        parameters:
          skill_totals:
            perception: 6
            stealth: 7
          damage_immunities:
          - lightning
          condition_immunities: []
          senses:
          - sense: darkvision
            range_ft: 90
          passive_perception: 16
          languages:
            speak: true
            known:
            - draconic
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 10
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 12
                    add: 6
                  average: 19
                - type: lightning
                  amount:
                    dice:
                      count: 2
                      sides: 10
                    add: 0
                  average: 11
            constrict:
              economy: action
              op: saving_throw
              ability: strength
              dc: 18
              targets:
                kind: creature
                count: 1
                range_ft: 5
                maximum_size: large
                visible_to_actor: true
              failure:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 5
                      sides: 8
                    add: 6
                  average: 28
              - op: grapple
                escape_dc: 16
                while_grappled:
                - op: condition.apply
                  condition: restrained
              success: []
            lightning_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 16
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  origin: actor
                  size_ft: 90
                  width_ft: 5
              failure:
              - op: damage
                components:
                - type: lightning
                  amount:
                    dice:
                      count: 12
                      sides: 10
                    add: 0
                  average: 66
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            swallow:
              economy: bonus_action
              op: saving_throw
              ability: dexterity
              dc: 18
              targets:
                kind: creature
                count: 1
                maximum_size: large
                grappled_by: actor
              failure:
              - op: swallow
                end_grapple: true
                conditions:
                - blinded
                - restrained
                total_cover_against: outside_actor
                periodic_damage:
                  event: actor.turn_start
                  damage:
                  - type: acid
                    amount:
                      dice:
                        count: 6
                        sides: 6
                      add: 0
                    average: 21
              success: []
              swallowed_capacity: 1
              regurgitation:
                trigger: turn.end
                ability: constitution
                dc: 14
                failure:
                  targets: swallowed_creature
                  exit_within_ft: 10
                  condition: prone
                when:
                  damage_to_actor_from_one_swallowed_creature_this_turn_at_least: 30
              actor_death:
                remove_condition: restrained
                escape_movement_cost_ft: 15
                exit_condition: prone
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - feature: bite
                count: 1
              - feature: constrict
                count: 1
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_behir
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      - that_turn.end
      - turn.end
      capabilities:
      - action_economy
      - attack_resolution
      - creature_lifecycle
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_behir
    citations:
    - SRD 5.2.1 pp. 263; builtin/srd-5.2.1/creature/behir.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/black_dragon_wyrmling:
    source_sha256: 2fe32ad79e40906f9008992a5631504604ead4c9bc0335a7ed26bc0a3d79fdb7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/black_dragon_wyrmling
        implementation: dnd2024:creature_black_dragon_wyrmling
        parameters:
          skill_totals:
            perception: 4
            stealth: 4
          damage_immunities:
          - acid
          condition_immunities: []
          senses:
          - sense: blindsight
            range_ft: 10
          - sense: darkvision
            range_ft: 60
          passive_perception: 14
          languages:
            speak: true
            known:
            - draconic
          features:
            amphibious:
              economy: passive
              op: breathing.allow
              media:
              - air
              - water
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 2
                  average: 5
                - type: acid
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 0
                  average: 2
            acid_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 11
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  origin: actor
                  size_ft: 15
                  width_ft: 5
              failure:
              - op: damage
                components:
                - type: acid
                  amount:
                    dice:
                      count: 5
                      sides: 8
                    add: 0
                  average: 22
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: rend
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_black_dragon_wyrmling
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_black_dragon_wyrmling
    citations:
    - SRD 5.2.1 pp. 263, 264; builtin/srd-5.2.1/creature/black_dragon_wyrmling.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/blood_hawk:
    source_sha256: 1f3395ec4cdceff30b555f53b82403e76fda42fdaf07e720cbaf5ed581801ebd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/blood_hawk
        implementation: dnd2024:creature_blood_hawk
        parameters:
          skill_totals:
            perception: 6
          senses: []
          passive_perception: 16
          languages:
            speak: true
            known: []
          features:
            pack_tactics:
              economy: passive
              op: roll.advantage
              rolls:
              - attack
              target_kind: creature
              when:
                ally_within_target_ft: 5
                ally_excluded_conditions:
                - incapacitated
            beak:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 2
                  average: 4
                replacement_when:
                  if:
                    target_bloodied: true
                  components:
                  - type: piercing
                    amount:
                      dice:
                        count: 1
                        sides: 8
                      add: 2
                    average: 6
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_blood_hawk
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_blood_hawk
    citations:
    - SRD 5.2.1 pp. 345, 346; builtin/srd-5.2.1/creature/blood_hawk.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/bone_devil:
    source_sha256: 0afa7b707927f68df2204fa6d5c96914dcefdeed3a92e3c747a8624e8904170f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/bone_devil
        implementation: dnd2024:creature_bone_devil
        parameters:
          skill_totals:
            deception: 7
            insight: 6
          damage_resistances:
          - cold
          damage_immunities:
          - fire
          - poison
          condition_immunities:
          - poisoned
          senses:
          - sense: darkvision
            range_ft: 120
          passive_perception: 12
          darkvision_ignores_magical_darkness: true
          languages:
            speak: true
            known:
            - infernal
          telepathy:
            range_ft: 120
            two_way: true
          features:
            diabolical_restoration:
              economy: passive
              trigger: actor.died
              when:
                plane_not: nine_hells
              body: disappears
              revive:
                new_body: true
                plane: nine_hells
                location_choice: gm
                hp: maximum
                delay:
                  amount: 0
                  unit: second
            magic_resistance:
              economy: passive
              op: roll.advantage
              rolls:
              - saving_throw
              against:
              - spell
              - magical_effect
            claw:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 8
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 4
                  average: 13
            infernal_sting:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 8
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 10
                    add: 4
                  average: 15
                - type: poison
                  amount:
                    dice:
                      count: 4
                      sides: 8
                    add: 0
                  average: 18
              - op: condition.apply
                condition: poisoned
                duration:
                  until: actor.next_turn_start
                while_active:
                - op: healing.prohibit
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: claw
              - count: 1
                feature: infernal_sting
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_bone_devil
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_bone_devil
    citations:
    - SRD 5.2.1 pp. 267, 268; builtin/srd-5.2.1/creature/bone_devil.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/brown_bear:
    source_sha256: 792387f94a4774e7a1be555270e6ccf0d1d5061c6ae0591098002fbd45ce057c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/brown_bear
        implementation: dnd2024:creature_brown_bear
        parameters:
          skill_totals:
            perception: 3
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 13
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 3
                  average: 7
            claw:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 3
                  average: 5
              - op: condition.apply
                condition: prone
                when:
                  target_kind: creature
                  target_maximum_size: large
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 1
                feature: bite
              - count: 1
                feature: claw
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_brown_bear
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_brown_bear
    citations:
    - SRD 5.2.1 pp. 346; builtin/srd-5.2.1/creature/brown_bear.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/bulette:
    source_sha256: b0be1f3d90c74246f3be883d25a5d6516c9a3bb6804935687959e8d2ff6e5bcb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/bulette
        implementation: dnd2024:creature_bulette
        parameters:
          skill_totals:
            perception: 6
          senses:
          - sense: darkvision
            range_ft: 60
          - sense: tremorsense
            range_ft: 120
          passive_perception: 16
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 12
                    add: 4
                  average: 17
            leap:
              economy: bonus_action
              op: movement.jump
              maximum_ft: 30
              movement_cost_ft: 10
            deadly_leap:
              economy: action
              op: sequence
              steps:
              - op: movement.jump
                movement_cost_ft: 5
                maximum_ft: 15
                destination_contains:
                  minimum_creatures: 1
                  maximum_size: large
              - op: saving_throw
                ability: dexterity
                dc: 15
                targets:
                  kind: creature
                  count: all
                  location: actor_destination_space
                failure:
                - op: damage
                  components:
                  - type: bludgeoning
                    amount:
                      dice:
                        count: 3
                        sides: 12
                      add: 0
                    average: 19
                - op: condition.apply
                  condition: prone
                success:
                - op: damage
                  from: failure_damage
                  multiplier: 0.5
                  round: down
                - op: push
                  distance_ft: 5
                  direction: straight_away_from_actor
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: bite
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_bulette
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_bulette
    citations:
    - SRD 5.2.1 pp. 272; builtin/srd-5.2.1/creature/bulette.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/centaur_trooper:
    source_sha256: d712333727edb65078ba536befa02a1c75820dd274b5f4fe587c4c07e0c44b19
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/centaur_trooper
        implementation: dnd2024:creature_centaur_trooper
        parameters:
          skill_totals:
            athletics: 6
            perception: 3
          gear:
          - item: breastplate
            count: 1
          - item: longbow
            count: 1
          - item: pike
            count: 1
          senses: []
          passive_perception: 13
          languages:
            speak: true
            known:
            - elvish
            - sylvan
          features:
            pike:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 4
                  average: 9
            longbow:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 150
                long: 600
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 2
                  average: 6
            trampling_charge:
              economy: bonus_action
              op: movement
              maximum:
                multiply:
                - 1
                - actor_stat: speed
              opportunity_attacks: false
              through_creature_maximum_size: medium
              on_enter_creature_space:
                maximum_per_creature: 1
                effect:
                  op: saving_throw
                  ability: strength
                  dc: 14
                  targets:
                    kind: entered_creature
                  failure:
                  - op: damage
                    components:
                    - type: bludgeoning
                      amount:
                        dice:
                          count: 1
                          sides: 6
                        add: 4
                      average: 7
                  - op: condition.apply
                    condition: prone
                  success: []
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 2
                choices:
                - pike
                - longbow
                combination: any
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_centaur_trooper
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_centaur_trooper
    citations:
    - SRD 5.2.1 pp. 272; builtin/srd-5.2.1/creature/centaur_trooper.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/chuul:
    source_sha256: b65c2deff5a407f37e23e74d82ff48136778fc05bd80064352e054805c96fd98
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/chuul
        implementation: dnd2024:creature_chuul
        parameters:
          skill_totals:
            perception: 4
          damage_immunities:
          - poison
          condition_immunities:
          - poisoned
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 14
          languages:
            speak: false
            known:
            - deep_speech
          features:
            amphibious:
              economy: passive
              op: breathing.allow
              media:
              - air
              - water
            sense_magic:
              economy: passive
              op: spell.effect
              spell:
                key: dnd2024:spells/detect_magic
              range_override_ft: 120
              magical: false
              continuous: true
            pincer:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 4
                  average: 9
              - op: grapple
                escape_dc: 14
                when:
                  target_kind: creature
                  target_maximum_size: large
                limb_slots: 2
                slots_per_target: 1
            paralyzing_tentacles:
              economy: action
              op: saving_throw
              ability: constitution
              dc: 13
              targets:
                kind: creature
                count: 1
                grappled_by: actor
              failure:
              - op: condition.apply
                condition: poisoned
                duration:
                  amount: 1
                  unit: minute
                while_active:
                - op: condition.apply
                  condition: paralyzed
                repeat_save:
                  event: target.turn_end
                  ability: constitution
                  dc: 13
                  success: effect.end
                  automatic_success_after:
                    amount: 1
                    unit: minute
              success: []
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - feature: pincer
                count: 2
              - feature: paralyzing_tentacles
                count: 1
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_chuul
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      - target.turn_end
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_chuul
    citations:
    - SRD 5.2.1 pp. 273, 274; builtin/srd-5.2.1/creature/chuul.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/cloud_giant:
    source_sha256: f584dedba830da1d06c3c51b090f6a759080872294d5b2ebfd2ccf71dbc90166
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/cloud_giant
        implementation: dnd2024:creature_cloud_giant
        parameters:
          skill_totals:
            insight: 7
            perception: 11
          senses: []
          passive_perception: 21
          languages:
            speak: true
            known:
            - common
            - giant
          features:
            thunderous_mace:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 12
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 3
                      sides: 8
                    add: 8
                  average: 21
                - type: thunder
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  average: 7
            thundercloud:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 12
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 240
                long: 240
              hit:
              - op: damage
                components:
                - type: thunder
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 8
                  average: 18
              - op: condition.apply
                condition: incapacitated
                duration:
                  until: target.next_turn_end
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/fog_cloud
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/light
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/control_weather
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/gaseous_form
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/telekinesis
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 15
            misty_step:
              economy: bonus_action
              op: spellcasting
              profile:
                ability: same_as_spellcasting
                waived_components: []
              choose_count: 1
              choices:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/misty_step
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 2
                choices:
                - thunderous_mace
                - thundercloud
                combination: any
              replace_attack:
                maximum: 1
                choices:
                - op: spell.cast
                  spell:
                    key: dnd2024:spells/fog_cloud
                  profile: spellcasting
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_cloud_giant
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_cloud_giant
    citations:
    - SRD 5.2.1 pp. 275; builtin/srd-5.2.1/creature/cloud_giant.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/constrictor_snake:
    source_sha256: edd1f27286bc07b5527ab3e45f1c96e0fe7cc0b5a2057b69485fa7dba8a12e34
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/constrictor_snake
        implementation: dnd2024:creature_constrictor_snake
        parameters:
          skill_totals:
            perception: 2
            stealth: 4
          senses:
          - sense: blindsight
            range_ft: 10
          passive_perception: 12
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 2
                  average: 6
            constrict:
              economy: action
              op: saving_throw
              ability: strength
              dc: 12
              targets:
                kind: creature
                count: 1
                range_ft: 5
                maximum_size: medium
                visible_to_actor: true
              failure:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 3
                      sides: 4
                    add: 0
                  average: 7
              - op: grapple
                escape_dc: 12
              success: []
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_constrictor_snake
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_constrictor_snake
    citations:
    - SRD 5.2.1 pp. 346, 347; builtin/srd-5.2.1/creature/constrictor_snake.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/crab:
    source_sha256: 0fdf96432f1a54d62058ba8253ab8d86c6ba07ff0adbf15fcf608a44b9d2bf2f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/crab
        implementation: dnd2024:creature_crab
        parameters:
          skill_totals:
            stealth: 2
          senses:
          - sense: blindsight
            range_ft: 30
          passive_perception: 9
          languages:
            speak: true
            known: []
          features:
            amphibious:
              economy: passive
              op: breathing.allow
              media:
              - air
              - water
            claw:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 2
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    constant: 1
                  average: 1
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_crab
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_crab
    citations:
    - SRD 5.2.1 pp. 347; builtin/srd-5.2.1/creature/crab.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/cultist_fanatic:
    source_sha256: 5a7e908bbffd453aa54142f518cdb6f74828b9c37200655a5db77bf938c45c6f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/cultist_fanatic
        implementation: dnd2024:creature_cultist_fanatic
        parameters:
          skill_totals:
            deception: 3
            persuasion: 3
            religion: 2
          gear:
          - item: holy_symbol
            count: 1
          - item: leather_armor
            count: 1
          senses: []
          passive_perception: 12
          languages:
            speak: true
            known:
            - common
          features:
            pact_blade:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 2
                  average: 6
                - type: necrotic
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  average: 7
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: wisdom
                waived_components: []
                spells:
                - spell:
                    key: dnd2024:spells/light
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/thaumaturgy
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/command
                  uses:
                    count: 2
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/hold_person
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 12
                attack_bonus: 4
            spiritual_weapon:
              economy: bonus_action
              op: spellcasting
              profile:
                ability: same_as_spellcasting
                waived_components: []
              choose_count: 1
              choices:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/spiritual_weapon
              uses:
                count: 2
                recovery: long_rest
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_cultist_fanatic
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_cultist_fanatic
    citations:
    - SRD 5.2.1 pp. 278; builtin/srd-5.2.1/creature/cultist_fanatic.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/deer:
    source_sha256: 1e252ca929fb9019885f43938b59d45c4a908548500a67d3d2582137766381c8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/deer
        implementation: dnd2024:creature_deer
        parameters:
          skill_totals:
            perception: 4
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 14
          languages:
            speak: true
            known: []
          features:
            agile:
              economy: passive
              op: opportunity_attack.suppress
              trigger: actor.leaves_enemy_reach
              movement_modes:
              - all
            ram:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 2
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 0
                  average: 2
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_deer
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.leaves_enemy_reach
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_deer
    citations:
    - SRD 5.2.1 pp. 347; builtin/srd-5.2.1/creature/deer.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/djinni:
    source_sha256: 4b35478cb6a506d432bd27ecb2b939354bbae398cf641aa9c00fc1ee0f9bbc6a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/djinni
        implementation: dnd2024:creature_djinni
        parameters:
          damage_immunities:
          - lightning
          - thunder
          condition_immunities: []
          senses:
          - sense: darkvision
            range_ft: 120
          passive_perception: 13
          languages:
            speak: true
            known:
            - primordial
            primordial_dialects:
            - auran
          features:
            elemental_restoration:
              economy: passive
              trigger: actor.died
              when:
                plane_not: elemental_plane_of_air
              body: disappears
              revive:
                new_body: true
                plane: elemental_plane_of_air
                location_choice: gm
                hp: maximum
                delay:
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 0
                  unit: day
            magic_resistance:
              economy: passive
              op: roll.advantage
              rolls:
              - saving_throw
              against:
              - spell
              - magical_effect
            wishes:
              economy: passive
              known_chance_percent: 30
              spell:
                key: dnd2024:spells/wish
              requester:
                excluded_subtype: genie
                wish_communicated_and_understood: true
              caster_stress: false
              uses: 3
              after_third_cast_lockout:
                amount: 365
                unit: day
              reset_uses_after_lockout: true
            storm_blade:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 9
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 5
                  average: 12
                - type: lightning
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  average: 7
            storm_bolt:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 9
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 120
                long: 120
              hit:
              - op: damage
                components:
                - type: thunder
                  amount:
                    dice:
                      count: 3
                      sides: 8
                    add: 0
                  average: 13
              - op: condition.apply
                condition: prone
                when:
                  target_kind: creature
                  target_maximum_size: large
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/detect_evil_and_good
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/create_food_and_water
                  uses:
                    count: 2
                    recovery: long_rest
                  overrides:
                    water_replacement_choice:
                    - water
                    - wine
                - spell:
                    key: dnd2024:spells/tongues
                  uses:
                    count: 2
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/wind_walk
                  uses:
                    count: 2
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/creation
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/gaseous_form
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/invisibility
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/major_image
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/plane_shift
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 17
            create_whirlwind:
              economy: action
              op: area.create
              target_location:
                range_ft: 120
                visible_to_actor: true
              area:
                shape: cylinder
                radius_ft: 20
                height_ft: 60
              duration:
                until: actor.concentration_end
              movement:
                event: actor.turn_start
                maximum_ft: 20
                optional: true
              entry_events:
              - area.enters_creature_space
              - creature.enters_area
              entry_save:
                op: saving_throw
                ability: strength
                dc: 17
                targets:
                  kind: creature
                  exclude: actor
                  maximum_checks_per_turn: 1
                failure:
                - op: condition.apply
                  condition: restrained
                  duration:
                    until: leaves_area_or_successful_repeat_save
                  moves_with_area: true
                  on_target_turn_start:
                  - op: damage
                    components:
                    - type: thunder
                      amount:
                        dice:
                          count: 6
                          sides: 6
                        add: 0
                      average: 21
                  repeat_save:
                    event: target.turn_end
                    ability: strength
                    dc: 17
                    success: effect.end
                success: []
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 3
                choices:
                - storm_blade
                - storm_bolt
                combination: any
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_djinni
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - actor.turn_start
      - creature.feature_requested
      - target.turn_end
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_djinni
    citations:
    - SRD 5.2.1 pp. 280; builtin/srd-5.2.1/creature/djinni.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/draft_horse:
    source_sha256: c759875abba5d68e76e607f7738d7016bb5f76aee69067342c3460e516c3d46c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/draft_horse
        implementation: dnd2024:creature_draft_horse
        parameters:
          senses: []
          passive_perception: 10
          languages:
            speak: true
            known: []
          features:
            hooves:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 4
                  average: 6
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_draft_horse
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_draft_horse
    citations:
    - SRD 5.2.1 pp. 347, 348; builtin/srd-5.2.1/creature/draft_horse.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/drider:
    source_sha256: e147491ca6b642b1dd5ff83fd49651b219263fb98e6494c9fa5b6e0e535470b9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/drider
        implementation: dnd2024:creature_drider
        parameters:
          skill_totals:
            perception: 6
            stealth: 10
          senses:
          - sense: darkvision
            range_ft: 120
          passive_perception: 16
          languages:
            speak: true
            known:
            - elvish
            - undercommon
          features:
            spider_climb:
              economy: passive
              op: movement.climb
              difficult_surfaces: true
              ceilings: true
              ability_check_required: false
            sunlight_sensitivity:
              economy: passive
              op: roll.disadvantage
              when:
                in_sunlight: true
              rolls:
              - ability_check
              - attack
            web_walker:
              economy: passive
              ignore_movement_restrictions_from:
              - web
              locate_creatures_in_same_web: true
            foreleg:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 4
                  average: 13
            poison_burst:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 120
                long: 120
              hit:
              - op: damage
                components:
                - type: poison
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 3
                  average: 13
            magic_of_the_spider_queen:
              economy: bonus_action
              op: spellcasting
              profile:
                ability: wisdom
                waived_components:
                - material
                save_dc: 14
              choose_count: 1
              choices:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/darkness
              - op: spell.cast
                spell:
                  key: dnd2024:spells/faerie_fire
              - op: spell.cast
                spell:
                  key: dnd2024:spells/web
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 3
                choices:
                - foreleg
                - poison_burst
                combination: any
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_drider
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_drider
    citations:
    - SRD 5.2.1 pp. 281; builtin/srd-5.2.1/creature/drider.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/dust_mephit:
    source_sha256: 12ca1a08de47611fcc3b919ee9505268d500f75814b9bee2fdd4c2b7d1aa92da
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/dust_mephit
        implementation: dnd2024:creature_dust_mephit
        parameters:
          skill_totals:
            perception: 2
            stealth: 4
          damage_vulnerabilities:
          - fire
          damage_immunities:
          - poison
          condition_immunities:
          - exhaustion
          - poisoned
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 12
          languages:
            speak: true
            known:
            - primordial
            primordial_dialects:
            - auran
            - terran
          features:
            claw:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 2
                  average: 4
            death_burst:
              economy: passive
              op: saving_throw
              ability: dexterity
              dc: 10
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  origin: actor
                  size_ft: 5
              failure:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 0
                  average: 5
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              trigger: actor.died
            blinding_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 10
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 15
              failure:
              - op: condition.apply
                condition: blinded
                duration:
                  until: actor.next_turn_end
              success: []
              recharge:
                die_sides: 6
                success_min: 6
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            sleep:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - verbal
                - somatic
                - material
                save_dc: 10
              choose_count: 1
              choices:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/sleep
              uses:
                count: 1
                recovery: long_rest
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_dust_mephit
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_dust_mephit
    citations:
    - SRD 5.2.1 pp. 307; builtin/srd-5.2.1/creature/dust_mephit.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/efreeti:
    source_sha256: 14b8c7e0e2e769c644473276dfebdfd2e353885ea3bcf7e9900f845da5b129cb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/efreeti
        implementation: dnd2024:creature_efreeti
        parameters:
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
          - sense: darkvision
            range_ft: 120
          passive_perception: 12
          languages:
            speak: true
            known:
            - primordial
            primordial_dialects:
            - ignan
          features:
            elemental_restoration:
              economy: passive
              trigger: actor.died
              when:
                plane_not: elemental_plane_of_fire
              body: disappears
              revive:
                new_body: true
                plane: elemental_plane_of_fire
                location_choice: gm
                hp: maximum
                delay:
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 0
                  unit: day
            magic_resistance:
              economy: passive
              op: roll.advantage
              rolls:
              - saving_throw
              against:
              - spell
              - magical_effect
            wishes:
              economy: passive
              known_chance_percent: 30
              spell:
                key: dnd2024:spells/wish
              requester:
                excluded_subtype: genie
                wish_communicated_and_understood: true
              caster_stress: false
              uses: 3
              after_third_cast_lockout:
                amount: 365
                unit: day
              reset_uses_after_lockout: true
            heated_blade:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 10
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 6
                  average: 13
                - type: fire
                  amount:
                    dice:
                      count: 2
                      sides: 12
                    add: 0
                  average: 13
            hurl_flame:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 8
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 120
                long: 120
              hit:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 7
                      sides: 6
                    add: 0
                  average: 24
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/elementalism
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/gaseous_form
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/invisibility
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/major_image
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/plane_shift
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/tongues
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/wall_of_fire
                  uses:
                    count: 1
                    recovery: long_rest
                  cast_level: 7
                save_dc: 16
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 3
                choices:
                - heated_blade
                - hurl_flame
                combination: any
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_efreeti
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_efreeti
    citations:
    - SRD 5.2.1 pp. 283; builtin/srd-5.2.1/creature/efreeti.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/erinyes:
    source_sha256: 9fdecdfa755e45352f44702828c6c86bdd4d422160f83ead97c513d7180defea
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/erinyes
        implementation: dnd2024:creature_erinyes
        parameters:
          skill_totals:
            perception: 6
            persuasion: 8
          damage_resistances:
          - cold
          damage_immunities:
          - fire
          - poison
          condition_immunities:
          - poisoned
          senses:
          - sense: truesight
            range_ft: 120
          passive_perception: 16
          languages:
            speak: true
            known:
            - infernal
          telepathy:
            range_ft: 120
            two_way: true
          features:
            diabolical_restoration:
              economy: passive
              trigger: actor.died
              when:
                plane_not: nine_hells
              body: disappears
              revive:
                new_body: true
                plane: nine_hells
                location_choice: gm
                hp: maximum
                delay:
                  amount: 0
                  unit: second
            magic_resistance:
              economy: passive
              op: roll.advantage
              rolls:
              - saving_throw
              against:
              - spell
              - magical_effect
            withering_sword:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 8
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 4
                  average: 13
                - type: necrotic
                  amount:
                    dice:
                      count: 2
                      sides: 10
                    add: 0
                  average: 11
            parry:
              economy: reaction
              trigger: melee_attack.hit_actor
              requires:
                holding_weapon: true
              op: ac.add
              amount: 4
              scope: triggering_attack
              recheck_hit: true
            magic_rope:
              economy: passive
              op: equipment.special
              item: magic_rope
              ac: 20
              hp: 90
              damage_immunities:
              - poison
              - psychic
              grants_feature: entangling_rope
              requires_bearing: true
              destroy_when:
              - hp_at_most: 0
              - distance_from_actor_ft_at_least: 5
                duration_at_least:
                  amount: 1
                  unit: hour
              - actor_dead: true
              restore_fully_after:
              - short_rest
              - long_rest
            entangling_rope:
              economy: action
              op: saving_throw
              ability: strength
              dc: 16
              targets:
                kind: creature
                count: 1
                range_ft: 120
                visible_to_actor: true
              failure:
              - op: damage
                components:
                - type: force
                  amount:
                    dice:
                      count: 4
                      sides: 6
                    add: 0
                  average: 14
              - op: condition.apply
                condition: restrained
                duration:
                  until_any:
                  - rope.destroyed
                  - actor.bonus_action_release
                  - actor.uses_entangling_rope_again
              success: []
              requires_item: magic_rope
              release_economy: bonus_action
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - feature: withering_sword
                count: 3
              - feature: entangling_rope
                count: 1
                optional: true
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_erinyes
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - creature.feature_requested
      - melee_attack.hit_actor
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - item_state
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_erinyes
    citations:
    - SRD 5.2.1 pp. 283; builtin/srd-5.2.1/creature/erinyes.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/fire_elemental:
    source_sha256: c3a544ba29d31426cf99659cd395ed7c23cfabb7f1b25a62b9e999266570599c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/fire_elemental
        implementation: dnd2024:creature_fire_elemental
        parameters:
          damage_resistances:
          - bludgeoning
          - piercing
          - slashing
          damage_immunities:
          - fire
          - poison
          condition_immunities:
          - exhaustion
          - grappled
          - paralyzed
          - petrified
          - poisoned
          - prone
          - restrained
          - unconscious
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 10
          languages:
            speak: true
            known:
            - primordial
            primordial_dialects:
            - ignan
          features:
            fire_form:
              economy: passive
              op: movement.form
              minimum_opening_inches: 1
              extra_movement_cost: 0
              can_enter_and_end_in_creature_space: true
              on_first_entry_per_creature_per_turn:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 0
                  average: 5
            illumination:
              economy: passive
              op: light.emit
              bright_radius_ft: 30
              dim_additional_ft: 30
            burn:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 3
                  average: 10
              - op: burning.start
                when_any:
                - target_kind: creature
                - target_kind: object
                  flammable: true
            fire_aura:
              economy: passive
              trigger: actor.turn_end
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  origin: actor
                  size_ft: 10
              effects:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 0
                  average: 5
              additional_targets:
                kind:
                - creature
                - flammable_object
                area:
                  shape: emanation
                  origin: actor
                  size_ft: 10
                effect:
                  op: burning.start
            water_susceptibility:
              economy: passive
              triggers:
              - event: actor.moves_in_water
                per_distance_ft: 5
              - event: water.splashed_on_actor
                per_volume_gallons: 1
              effects:
              - op: damage
                components:
                - type: cold
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  average: 3
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: burn
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_fire_elemental
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.moves_in_water
      - actor.turn_end
      - creature.feature_requested
      - water.splashed_on_actor
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_fire_elemental
    citations:
    - SRD 5.2.1 pp. 284, 285; builtin/srd-5.2.1/creature/fire_elemental.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/flying_snake:
    source_sha256: 35a515f7613562df409948c691603c8aea26eb21696f76ba1c4b25630db90079
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/flying_snake
        implementation: dnd2024:creature_flying_snake
        parameters:
          senses:
          - sense: blindsight
            range_ft: 10
          passive_perception: 11
          languages:
            speak: true
            known: []
          features:
            flyby:
              economy: passive
              op: opportunity_attack.suppress
              trigger: actor.leaves_enemy_reach
              movement_modes:
              - fly
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    constant: 1
                  average: 1
                - type: poison
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 0
                  average: 5
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_flying_snake
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.leaves_enemy_reach
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_flying_snake
    citations:
    - SRD 5.2.1 pp. 348; builtin/srd-5.2.1/creature/flying_snake.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/gargoyle:
    source_sha256: 5bf016b14b6fb1a32e8148fff449d5a2f2ffe08ab973868063bf914c4017914b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/gargoyle
        implementation: dnd2024:creature_gargoyle
        parameters:
          skill_totals:
            stealth: 4
          damage_immunities:
          - poison
          condition_immunities:
          - exhaustion
          - petrified
          - poisoned
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 10
          languages:
            speak: true
            known:
            - primordial
            primordial_dialects:
            - terran
          features:
            flyby:
              economy: passive
              op: opportunity_attack.suppress
              trigger: actor.leaves_enemy_reach
              movement_modes:
              - fly
            claw:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 2
                  average: 7
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: claw
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_gargoyle
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.leaves_enemy_reach
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_gargoyle
    citations:
    - SRD 5.2.1 pp. 286; builtin/srd-5.2.1/creature/gargoyle.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ghost:
    source_sha256: 21a23161f64aa2b9d21995657c7e31d135f77101d873c3a5a65cdbbc021f02f2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ghost
        implementation: dnd2024:creature_ghost
        parameters:
          damage_resistances:
          - acid
          - bludgeoning
          - cold
          - fire
          - lightning
          - piercing
          - slashing
          - thunder
          damage_immunities:
          - necrotic
          - poison
          condition_immunities:
          - charmed
          - exhaustion
          - frightened
          - grappled
          - paralyzed
          - petrified
          - poisoned
          - prone
          - restrained
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 11
          languages:
            speak: true
            known:
            - common
            additional_choice_count: 1
          features:
            ethereal_sight:
              economy: passive
              op: vision.cross_plane
              from: material
              to: ethereal
              range_ft: 60
            incorporeal_movement:
              economy: passive
              op: movement.pass_through
              entities:
              - creature
              - object
              cost: difficult_terrain
              end_turn_inside_object:
              - op: damage
                components:
                - type: force
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 0
                  average: 5
            withering_touch:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: necrotic
                  amount:
                    dice:
                      count: 3
                      sides: 10
                    add: 3
                  average: 19
            etherealness:
              economy: action
              op: spell.cast
              spell:
                key: dnd2024:spells/etherealness
              ability: charisma
              waived_components:
              - verbal
              - somatic
              - material
              overrides:
                visible_across_planes:
                - material
                - border_ethereal
                cross_plane_effects_allowed: false
            horrific_visage:
              economy: action
              op: saving_throw
              ability: wisdom
              dc: 13
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 60
                must_see_actor: true
                exclude_creature_types:
                - undead
              failure:
              - op: damage
                components:
                - type: psychic
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 3
                  average: 10
              - op: condition.apply
                condition: frightened
                duration:
                  until: actor.next_turn_start
              success:
              - op: feature.immunity
                source: actor
                feature: horrific_visage
                duration:
                  amount: 24
                  unit: hour
            possession:
              economy: action
              op: saving_throw
              ability: charisma
              dc: 13
              targets:
                kind: creature
                count: 1
                range_ft: 5
                visible_to_actor: true
                creature_types:
                - humanoid
              failure:
              - op: possession.start
                actor_disappears: true
                target_conditions:
                - incapacitated
                body_control: actor
                target_retains_awareness: true
                actor_targetable_only_by: effects_specifically_targeting_undead
                actor_stat_overrides_from_target:
                - speed
                - strength_modifier
                - dexterity_modifier
                - constitution_modifier
                end_when_any:
                - body_hp_zero
                - actor.bonus_action_leave
                end_effects:
                - op: actor.appear
                  within_target_ft: 5
                  unoccupied: true
                - op: feature.immunity
                  source: actor
                  feature: possession
                  duration:
                    amount: 24
                    unit: hour
              success:
              - op: feature.immunity
                source: actor
                feature: possession
                duration:
                  amount: 24
                  unit: hour
              leave_economy: bonus_action
              recharge:
                die_sides: 6
                success_min: 6
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: withering_touch
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_ghost
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_lifecycle
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_ghost
    citations:
    - SRD 5.2.1 pp. 287, 288; builtin/srd-5.2.1/creature/ghost.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_badger:
    source_sha256: 31a80cc7a898842a6c36324137fec7c604949f45c91e35e54ecacdd9e5dcb25c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_badger
        implementation: dnd2024:creature_giant_badger
        parameters:
          skill_totals:
            perception: 3
          damage_resistances:
          - poison
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 13
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 3
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 1
                  average: 6
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_giant_badger
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_giant_badger
    citations:
    - SRD 5.2.1 pp. 349; builtin/srd-5.2.1/creature/giant_badger.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_centipede:
    source_sha256: 7ffa55603c2a8c41319146c04342d14c40f77e623ea4aec235a070f6a9b75f69
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_centipede
        implementation: dnd2024:creature_giant_centipede
        parameters:
          senses:
          - sense: blindsight
            range_ft: 30
          passive_perception: 8
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 2
                  average: 4
              - op: condition.apply
                condition: poisoned
                duration:
                  until: actor.next_turn_start
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_giant_centipede
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_giant_centipede
    citations:
    - SRD 5.2.1 pp. 349, 350; builtin/srd-5.2.1/creature/giant_centipede.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_crocodile:
    source_sha256: 014d202b83d74a7d0326030b37cc234ec0862378b0cbf11af45509d5753451a3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_crocodile
        implementation: dnd2024:creature_giant_crocodile
        parameters:
          skill_totals:
            stealth: 5
          senses: []
          passive_perception: 10
          languages:
            speak: true
            known: []
          features:
            hold_breath:
              economy: passive
              op: breathing.hold
              duration:
                amount: 1
                unit: hour
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 8
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 3
                      sides: 10
                    add: 5
                  average: 21
              - op: grapple
                escape_dc: 15
                when:
                  target_kind: creature
                  target_maximum_size: large
                while_grappled:
                - op: condition.apply
                  condition: restrained
                - op: targeting.prohibit
                  feature: tail
                  source: actor
            tail:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 8
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 3
                      sides: 8
                    add: 5
                  average: 18
              - op: condition.apply
                condition: prone
                when:
                  target_kind: creature
                  target_maximum_size: large
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 1
                feature: bite
              - count: 1
                feature: tail
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_giant_crocodile
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_giant_crocodile
    citations:
    - SRD 5.2.1 pp. 350; builtin/srd-5.2.1/creature/giant_crocodile.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_fire_beetle:
    source_sha256: f7e0074a2d5f36f7672196850363ca7b60439a030d91becd51480803c1e257ea
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_fire_beetle
        implementation: dnd2024:creature_giant_fire_beetle
        parameters:
          damage_resistances:
          - fire
          senses:
          - sense: blindsight
            range_ft: 30
          passive_perception: 8
          languages:
            speak: true
            known: []
          features:
            illumination:
              economy: passive
              op: light.emit
              bright_radius_ft: 10
              dim_additional_ft: 10
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 1
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: fire
                  amount:
                    constant: 1
                  average: 1
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_giant_fire_beetle
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_giant_fire_beetle
    citations:
    - SRD 5.2.1 pp. 351; builtin/srd-5.2.1/creature/giant_fire_beetle.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_goat:
    source_sha256: b1ee30d085a6474a409693a7d29bd571228ad11fc39bcede8ac61682e7d050bc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_goat
        implementation: dnd2024:creature_giant_goat
        parameters:
          skill_totals:
            perception: 3
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 13
          languages:
            speak: true
            known: []
          features:
            ram:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 3
                  average: 6
              - op: conditional
                when:
                  target_maximum_size: large
                  actor_moved_straight_toward_target_ft_at_least: 20
                  immediately_before_hit: true
                  target_kind: creature
                effects:
                - op: damage
                  components:
                  - type: bludgeoning
                    amount:
                      dice:
                        count: 2
                        sides: 4
                      add: 0
                    average: 5
                - op: condition.apply
                  condition: prone
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_giant_goat
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_giant_goat
    citations:
    - SRD 5.2.1 pp. 351; builtin/srd-5.2.1/creature/giant_goat.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_lizard:
    source_sha256: 221c13e09d692bd268ae5f868ea322270d8a52d99a99b486be2b415830140069
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_lizard
        implementation: dnd2024:creature_giant_lizard
        parameters:
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 10
          languages:
            speak: true
            known: []
          features:
            spider_climb:
              economy: passive
              op: movement.climb
              difficult_surfaces: true
              ceilings: true
              ability_check_required: false
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 2
                  average: 6
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_giant_lizard
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_giant_lizard
    citations:
    - SRD 5.2.1 pp. 352; builtin/srd-5.2.1/creature/giant_lizard.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_rat:
    source_sha256: 4cb0bc5a39dc74e890cc11e325c1c4297decd1a84bc6130425fdf36a6c0c1927
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_rat
        implementation: dnd2024:creature_giant_rat
        parameters:
          skill_totals:
            perception: 2
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 12
          languages:
            speak: true
            known: []
          features:
            pack_tactics:
              economy: passive
              op: roll.advantage
              rolls:
              - attack
              target_kind: creature
              when:
                ally_within_target_ft: 5
                ally_excluded_conditions:
                - incapacitated
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 3
                  average: 5
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_giant_rat
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_giant_rat
    citations:
    - SRD 5.2.1 pp. 353; builtin/srd-5.2.1/creature/giant_rat.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_shark:
    source_sha256: c4a5578969f844758692b75826dde9995bb7945447541301659f3303922da9b6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_shark
        implementation: dnd2024:creature_giant_shark
        parameters:
          skill_totals:
            perception: 3
          senses:
          - sense: blindsight
            range_ft: 60
          passive_perception: 13
          languages:
            speak: true
            known: []
          features:
            water_breathing:
              economy: passive
              op: breathing.allow
              media:
              - water
              exclusive: true
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 9
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 3
                      sides: 10
                    add: 6
                  average: 22
              advantage_when:
                target_hp_below_maximum: true
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: bite
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_giant_shark
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_giant_shark
    citations:
    - SRD 5.2.1 pp. 353; builtin/srd-5.2.1/creature/giant_shark.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_venomous_snake:
    source_sha256: 81a139380958125e95383c7ca02514f5adcbbfa4ab7dc418f22efc777a9dd6a1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_venomous_snake
        implementation: dnd2024:creature_giant_venomous_snake
        parameters:
          skill_totals:
            perception: 2
          senses:
          - sense: blindsight
            range_ft: 10
          passive_perception: 12
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 4
                  average: 6
                - type: poison
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 0
                  average: 4
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_giant_venomous_snake
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_giant_venomous_snake
    citations:
    - SRD 5.2.1 pp. 354; builtin/srd-5.2.1/creature/giant_venomous_snake.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_weasel:
    source_sha256: 7781ed6b1fee3f50d5a66b263a09ccb01bff887716fa6ad080016f9e77c63ff3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_weasel
        implementation: dnd2024:creature_giant_weasel
        parameters:
          skill_totals:
            acrobatics: 5
            perception: 3
            stealth: 5
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 13
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 3
                  average: 5
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_giant_weasel
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_giant_weasel
    citations:
    - SRD 5.2.1 pp. 355; builtin/srd-5.2.1/creature/giant_weasel.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/glabrezu:
    source_sha256: 9fbd272e0a0df2541fc680ef9113a8936df4e6d44a9168cb8cf8fb4676f447e9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/glabrezu
        implementation: dnd2024:creature_glabrezu
        parameters:
          skill_totals:
            deception: 7
            perception: 7
          damage_resistances:
          - cold
          - fire
          - lightning
          damage_immunities:
          - poison
          condition_immunities:
          - poisoned
          senses:
          - sense: truesight
            range_ft: 120
          passive_perception: 17
          languages:
            speak: true
            known:
            - abyssal
          telepathy:
            range_ft: 120
            two_way: true
          features:
            demonic_restoration:
              economy: passive
              trigger: actor.died
              when:
                plane_not: abyss
              body: disappears
              revive:
                new_body: true
                plane: abyss
                location_choice: gm
                hp: maximum
                delay:
                  amount: 0
                  unit: second
            magic_resistance:
              economy: passive
              op: roll.advantage
              rolls:
              - saving_throw
              against:
              - spell
              - magical_effect
            pincer:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 9
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 10
                    add: 5
                  average: 16
              - op: grapple
                escape_dc: 15
                when:
                  target_kind: creature
                  target_maximum_size: medium
                limb_slots: 2
                slots_per_target: 1
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: intelligence
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/darkness
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/dispel_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/confusion
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/fly
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/power_word_stun
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 16
            pummel:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 17
              targets:
                kind: creature
                count: 1
                grappled_by: actor
              failure:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 5
                  average: 15
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - feature: pincer
                count: 2
              - choose:
                - pummel
                - spellcasting
                count: 1
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_glabrezu
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_glabrezu
    citations:
    - SRD 5.2.1 pp. 289; builtin/srd-5.2.1/creature/glabrezu.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/goat:
    source_sha256: ddb00c5b2eeb13d7b130504e5154de5948891f2df09ad576fcb8714959e2b64e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/goat
        implementation: dnd2024:creature_goat
        parameters:
          skill_totals:
            perception: 2
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 12
          languages:
            speak: true
            known: []
          features:
            ram:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 2
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    constant: 1
                  average: 1
                replacement_when:
                  if:
                    actor_moved_straight_toward_target_ft_at_least: 20
                    immediately_before_hit: true
                  components:
                  - type: bludgeoning
                    amount:
                      dice:
                        count: 1
                        sides: 4
                      add: 0
                    average: 2
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_goat
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_goat
    citations:
    - SRD 5.2.1 pp. 355; builtin/srd-5.2.1/creature/goat.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/goblin_warrior:
    source_sha256: fc9e8d8bff4f76a7aa086e545cac295d706a47a85e1f0d717c0fbae6351e2493
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/goblin_warrior
        implementation: dnd2024:creature_goblin_warrior
        parameters:
          skill_totals:
            stealth: 6
          gear:
          - item: leather_armor
            count: 1
          - item: scimitar
            count: 1
          - item: shield
            count: 1
          - item: shortbow
            count: 1
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 9
          languages:
            speak: true
            known:
            - common
            - goblin
          features:
            scimitar:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 2
                  average: 5
              - op: conditional
                when:
                  attack_had_advantage: true
                effects:
                - op: damage
                  components:
                  - type: slashing
                    amount:
                      dice:
                        count: 1
                        sides: 4
                      add: 0
                    average: 2
            shortbow:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 80
                long: 320
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 2
                  average: 5
              - op: conditional
                when:
                  attack_had_advantage: true
                effects:
                - op: damage
                  components:
                  - type: piercing
                    amount:
                      dice:
                        count: 1
                        sides: 4
                      add: 0
                    average: 2
            nimble_escape:
              economy: bonus_action
              op: action.choose
              choices:
              - disengage
              - hide
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_goblin_warrior
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_goblin_warrior
    citations:
    - SRD 5.2.1 pp. 290; builtin/srd-5.2.1/creature/goblin_warrior.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/gray_ooze:
    source_sha256: 150623a852419b3c5f7206a6e9f72c7af5d8ab41bbfc97559b01581094e35ce8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/gray_ooze
        implementation: dnd2024:creature_gray_ooze
        parameters:
          skill_totals:
            stealth: 2
          damage_resistances:
          - acid
          - cold
          - fire
          damage_immunities: []
          condition_immunities:
          - blinded
          - charmed
          - deafened
          - exhaustion
          - frightened
          - grappled
          - prone
          - restrained
          senses:
          - sense: blindsight
            range_ft: 60
          passive_perception: 8
          languages:
            speak: true
            known: []
          features:
            amorphous:
              economy: passive
              op: movement.form
              minimum_opening_inches: 1
              extra_movement_cost: 0
            pseudopod:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 3
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: acid
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 1
                  average: 10
              - op: item.ac_penalty
                target:
                  category: armor
                  worn_by: target
                  magical: false
                add: -1
                cumulative: true
                destroy_at_ac: 10
                remove_penalty_spell:
                  key: dnd2024:spells/mending
            corrosive_form:
              economy: passive
              ammunition:
                trigger: attack.resolved
                op: item.destroy
                when:
                  item_category: ammunition
                  item_magical: false
                  hit_actor: true
                  damage_dealt_to_actor_above: 0
              weapon:
                trigger: damage.applied
                op: item.attack_penalty
                add: -1
                cumulative: true
                destroy_at_penalty: -5
                remove_penalty_spell:
                  key: dnd2024:spells/mending
                when:
                  item_category: weapon
                  item_magical: false
                  damage_dealt_to_actor_above: 0
                  item_contacts_actor: true
              eat_material:
                materials:
                - nonmagical_metal
                - nonmagical_wood
                thickness_inches: 2
                duration:
                  amount: 1
                  unit: round
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_gray_ooze
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      - nonmagical_ammunition_hits_and_deals_damage
      - nonmagical_weapon_deals_damage_and_contacts_actor
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - item_state
      - spatial_constraints
    requires:
    - dnd2024:creature_gray_ooze
    citations:
    - SRD 5.2.1 pp. 293; builtin/srd-5.2.1/creature/gray_ooze.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/grick:
    source_sha256: 9704625452ca3d079afdc78a0789f1fb5be17d8132cb82734634edb4ad01dd48
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/grick
        implementation: dnd2024:creature_grick
        parameters:
          skill_totals:
            stealth: 4
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 12
          languages:
            speak: true
            known: []
          features:
            beak:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 2
                  average: 9
            tentacles:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 2
                  average: 7
              - op: grapple
                escape_dc: 12
                when:
                  target_kind: creature
                  target_maximum_size: medium
                limb_slots: 4
                slots_per_target: 4
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 1
                feature: beak
              - count: 1
                feature: tentacles
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_grick
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_grick
    citations:
    - SRD 5.2.1 pp. 295; builtin/srd-5.2.1/creature/grick.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/guard:
    source_sha256: 7fb48cd94ba847a9658730f32845be4b5e4e6e7369a6d1f277317e2588797881
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/guard
        implementation: dnd2024:creature_guard
        parameters:
          skill_totals:
            perception: 2
          gear:
          - item: chain_shirt
            count: 1
          - item: shield
            count: 1
          - item: spear
            count: 1
          senses: []
          passive_perception: 12
          languages:
            speak: true
            known:
            - common
          features:
            spear:
              economy: action
              op: attack
              modes:
              - melee
              - ranged
              attack_bonus: 3
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              range_ft:
                normal: 20
                long: 60
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 1
                  average: 4
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_guard
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_guard
    citations:
    - SRD 5.2.1 pp. 296; builtin/srd-5.2.1/creature/guard.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/half_dragon:
    source_sha256: 5714a0fd528f3b477e8ab5d625f7741a92db76356e83709a28c8aafa26d78a4f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/half_dragon
        implementation: dnd2024:creature_half_dragon
        parameters:
          skill_totals:
            athletics: 7
            perception: 5
            stealth: 5
          damage_resistances:
          - parameter: draconic_origin
          senses:
          - sense: blindsight
            range_ft: 10
          - sense: darkvision
            range_ft: 60
          passive_perception: 15
          languages:
            speak: true
            known:
            - common
            - draconic
          features:
            claw:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 4
                  average: 6
                - type:
                    parameter: draconic_origin
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  average: 7
            leap:
              economy: bonus_action
              op: movement.jump
              maximum_ft: 30
              movement_cost_ft: 10
            draconic_origin:
              economy: passive
              op: choice.bind
              parameter: draconic_origin
              chooser: gm
              choices:
              - acid
              - cold
              - fire
              - lightning
              - poison
              affects:
              - damage_resistance
              - claw_damage_type
              - dragons_breath_damage_type
            dragons_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 14
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 30
              failure:
              - op: damage
                components:
                - type:
                    parameter: draconic_origin
                  amount:
                    dice:
                      count: 8
                      sides: 6
                    add: 0
                  average: 28
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: claw
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_half_dragon
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_half_dragon
    citations:
    - SRD 5.2.1 pp. 297; builtin/srd-5.2.1/creature/half_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/hell_hound:
    source_sha256: d313781345d704698ba21ffbd5bb64db36dcff0f2954a545a241ace8e33a47dc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/hell_hound
        implementation: dnd2024:creature_hell_hound
        parameters:
          skill_totals:
            perception: 5
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 15
          languages:
            speak: false
            known:
            - infernal
          features:
            pack_tactics:
              economy: passive
              op: roll.advantage
              rolls:
              - attack
              target_kind: creature
              when:
                ally_within_target_ft: 5
                ally_excluded_conditions:
                - incapacitated
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 3
                  average: 7
                - type: fire
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  average: 3
            fire_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 12
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 15
              failure:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 5
                      sides: 6
                    add: 0
                  average: 17
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: bite
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_hell_hound
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_hell_hound
    citations:
    - SRD 5.2.1 pp. 297; builtin/srd-5.2.1/creature/hell_hound.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/hippogriff:
    source_sha256: 19e0ff4ac8c4c018a45e8412ec989615ee355b5506d51475224acaac7e9619ba
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/hippogriff
        implementation: dnd2024:creature_hippogriff
        parameters:
          skill_totals:
            perception: 5
          senses: []
          passive_perception: 15
          languages:
            speak: true
            known: []
          features:
            flyby:
              economy: passive
              op: opportunity_attack.suppress
              trigger: actor.leaves_enemy_reach
              movement_modes:
              - fly
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 3
                  average: 7
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: rend
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_hippogriff
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.leaves_enemy_reach
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_hippogriff
    citations:
    - SRD 5.2.1 pp. 298; builtin/srd-5.2.1/creature/hippogriff.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/hobgoblin_warrior:
    source_sha256: 822722b757dfb6125b7ffb3e2cbf83247b6e94cae3e6d567afe372367ff93f24
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/hobgoblin_warrior
        implementation: dnd2024:creature_hobgoblin_warrior
        parameters:
          gear:
          - item: half_plate_armor
            count: 1
          - item: longbow
            count: 1
          - item: longsword
            count: 1
          - item: shield
            count: 1
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 10
          languages:
            speak: true
            known:
            - common
            - goblin
          features:
            pack_tactics:
              economy: passive
              op: roll.advantage
              rolls:
              - attack
              target_kind: creature
              when:
                ally_within_target_ft: 5
                ally_excluded_conditions:
                - incapacitated
            longsword:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 3
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 10
                    add: 1
                  average: 12
            longbow:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 3
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 150
                long: 600
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 1
                  average: 5
                - type: poison
                  amount:
                    dice:
                      count: 3
                      sides: 4
                    add: 0
                  average: 7
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_hobgoblin_warrior
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_hobgoblin_warrior
    citations:
    - SRD 5.2.1 pp. 298, 299; builtin/srd-5.2.1/creature/hobgoblin_warrior.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/hunter_shark:
    source_sha256: 96811aafb606cbd3161f0bce0e6929b84781ebca369b038dff37317066455019
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/hunter_shark
        implementation: dnd2024:creature_hunter_shark
        parameters:
          skill_totals:
            perception: 2
          senses:
          - sense: blindsight
            range_ft: 60
          passive_perception: 12
          languages:
            speak: true
            known: []
          features:
            water_breathing:
              economy: passive
              op: breathing.allow
              media:
              - water
              exclusive: true
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 4
                  average: 14
              advantage_when:
                target_hp_below_maximum: true
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_hunter_shark
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_hunter_shark
    citations:
    - SRD 5.2.1 pp. 356; builtin/srd-5.2.1/creature/hunter_shark.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ice_devil:
    source_sha256: 6bea292edf0eadfbed9b4204dcdebdd66fdcb678b4f1ecf243d69b73fe93e3d1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ice_devil
        implementation: dnd2024:creature_ice_devil
        parameters:
          skill_totals:
            insight: 7
            perception: 7
            persuasion: 9
          damage_immunities:
          - cold
          - fire
          - poison
          condition_immunities:
          - poisoned
          senses:
          - sense: blindsight
            range_ft: 120
          passive_perception: 17
          languages:
            speak: true
            known:
            - infernal
          telepathy:
            range_ft: 120
            two_way: true
          features:
            diabolical_restoration:
              economy: passive
              trigger: actor.died
              when:
                plane_not: nine_hells
              body: disappears
              revive:
                new_body: true
                plane: nine_hells
                location_choice: gm
                hp: maximum
                delay:
                  amount: 0
                  unit: second
            magic_resistance:
              economy: passive
              op: roll.advantage
              rolls:
              - saving_throw
              against:
              - spell
              - magical_effect
            ice_spear:
              economy: action
              op: attack
              modes:
              - melee
              - ranged
              attack_bonus: 10
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              range_ft:
                normal: 30
                long: 120
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 5
                  average: 14
                - type: cold
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 0
                  average: 10
              - op: effect.apply
                duration:
                  until: target.next_turn_end
                rules:
                  bonus_actions_allowed: false
                  reactions_allowed: false
                  speed_add_ft: -10
                  turn_economy_choice:
                  - movement
                  - one_action
                  both_allowed: false
              after_attack:
                when: ranged_attack_hit_or_miss
                op: item.return
                destination: actor_hand
                magical: true
            tail:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 10
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 5
                  average: 15
                - type: cold
                  amount:
                    dice:
                      count: 4
                      sides: 8
                    add: 0
                  average: 18
            ice_wall:
              economy: action
              op: spellcasting
              profile:
                ability: intelligence
                waived_components:
                - verbal
                - somatic
                - material
                save_dc: 17
              choose_count: 1
              choices:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/wall_of_ice
                cast_level: 8
              recharge:
                die_sides: 6
                success_min: 6
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 3
                feature: ice_spear
              replace_attack:
                maximum: 1
                choices:
                - op: feature.use
                  feature: tail
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_ice_devil
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - item_state
      - resources
      - rest_recovery
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_ice_devil
    citations:
    - SRD 5.2.1 pp. 300; builtin/srd-5.2.1/creature/ice_devil.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/incubus:
    source_sha256: 8f0b2555f0c57cccf4de31b8db8f3599404993260c0f702d228a0fc7e981856c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/incubus
        implementation: dnd2024:creature_incubus
        parameters:
          skill_totals:
            deception: 9
            insight: 5
            perception: 5
            persuasion: 9
            stealth: 7
          damage_resistances:
          - cold
          - fire
          - poison
          - psychic
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 15
          languages:
            speak: true
            known:
            - common
            - infernal
            - abyssal
          telepathy:
            range_ft: 60
            two_way: true
          features:
            restless_touch:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: psychic
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 5
                  average: 15
              - op: curse.apply
                duration:
                  maximum:
                    amount: 24
                    unit: hour
                  until: actor.died
                rules:
                  short_rest_benefits: false
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/disguise_self
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/etherealness
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/dream
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/hypnotic_pattern
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 15
            succubus_form:
              economy: passive
              trigger: actor.long_rest_finished
              optional: true
              op: stat_block.replace
              definition:
                key: dnd2024:creatures/succubus
              equipment_transformed: false
            nightmare:
              economy: bonus_action
              op: saving_throw
              ability: wisdom
              dc: 15
              targets:
                kind: creature
                count: 1
                range_ft: 60
                visible_to_actor: true
              failure:
              - op: branch
                if:
                  target_hp_at_most: 20
                then:
                - op: condition.apply
                  condition: unconscious
                  duration:
                    maximum:
                      amount: 1
                      unit: hour
                    until_any:
                    - target.takes_damage
                    - creature_within_5_ft.uses_action_to_wake
                else:
                - op: damage
                  components:
                  - type: psychic
                    amount:
                      dice:
                        count: 4
                        sides: 8
                      add: 0
                    average: 18
              success: []
              recharge:
                die_sides: 6
                success_min: 6
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: restless_touch
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_incubus
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.long_rest_finished
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_lifecycle
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_incubus
    citations:
    - SRD 5.2.1 pp. 301; builtin/srd-5.2.1/creature/incubus.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/jackal:
    source_sha256: a29e1fae2e3ca328b33b9d0d61c5ea6e9d3c704d3dcfbbe67e07953fcf32ede1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/jackal
        implementation: dnd2024:creature_jackal
        parameters:
          skill_totals:
            perception: 5
            stealth: 4
          senses:
          - sense: darkvision
            range_ft: 90
          passive_perception: 15
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 1
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: -1
                  average: 1
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_jackal
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_jackal
    citations:
    - SRD 5.2.1 pp. 356; builtin/srd-5.2.1/creature/jackal.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/kobold_warrior:
    source_sha256: 8264d6626394c91192c45ca3d7def3d953a552e6090a16a5903fe2e1016957c7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/kobold_warrior
        implementation: dnd2024:creature_kobold_warrior
        parameters:
          gear:
          - item: daggers
            count: 3
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 8
          languages:
            speak: true
            known:
            - common
            - draconic
          features:
            pack_tactics:
              economy: passive
              op: roll.advantage
              rolls:
              - attack
              target_kind: creature
              when:
                ally_within_target_ft: 5
                ally_excluded_conditions:
                - incapacitated
            sunlight_sensitivity:
              economy: passive
              op: roll.disadvantage
              when:
                in_sunlight: true
              rolls:
              - ability_check
              - attack
            dagger:
              economy: action
              op: attack
              modes:
              - melee
              - ranged
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              range_ft:
                normal: 20
                long: 60
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 2
                  average: 4
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_kobold_warrior
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_kobold_warrior
    citations:
    - SRD 5.2.1 pp. 302; builtin/srd-5.2.1/creature/kobold_warrior.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/lemure:
    source_sha256: bd87468e3d6506e2a76f460564a863c8edfd684effba5efe963f47519b34ab4c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/lemure
        implementation: dnd2024:creature_lemure
        parameters:
          damage_resistances:
          - cold
          damage_immunities:
          - fire
          - poison
          condition_immunities:
          - charmed
          - frightened
          - poisoned
          senses:
          - sense: darkvision
            range_ft: 120
          passive_perception: 10
          darkvision_ignores_magical_darkness: true
          languages:
            speak: false
            known:
            - infernal
          features:
            vile_slime:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 2
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: poison
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 0
                  average: 2
            hellish_restoration:
              economy: passive
              trigger: actor.died
              when:
                plane: nine_hells
              revive:
                delay:
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 0
                  unit: day
                hp: maximum
              prevented_by:
              - killer_has_spell:
                  key: dnd2024:spells/bless
              - remains_sprinkled_with: holy_water
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_lemure
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_lemure
    citations:
    - SRD 5.2.1 pp. 304; builtin/srd-5.2.1/creature/lemure.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/lizard:
    source_sha256: ddeb0ce3e994552349e6237f35c191932e58be9833fb212eddafc1461d0897d9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/lizard
        implementation: dnd2024:creature_lizard
        parameters:
          senses:
          - sense: darkvision
            range_ft: 30
          passive_perception: 9
          languages:
            speak: true
            known: []
          features:
            spider_climb:
              economy: passive
              op: movement.climb
              difficult_surfaces: true
              ceilings: true
              ability_check_required: false
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 2
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    constant: 1
                  average: 1
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_lizard
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_lizard
    citations:
    - SRD 5.2.1 pp. 357; builtin/srd-5.2.1/creature/lizard.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/magmin:
    source_sha256: 5ab1364f5aeb6149f4ea2016c997df1055068a8034172ee06100c77dacf7ab38
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/magmin
        implementation: dnd2024:creature_magmin
        parameters:
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 10
          languages:
            speak: true
            known:
            - primordial
            primordial_dialects:
            - ignan
          features:
            touch:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 2
                  average: 7
              - op: burning.start
                when_any:
                - target_kind: creature
                - target_kind: object
                  flammable: true
                  worn_or_carried: false
            death_burst:
              economy: passive
              op: saving_throw
              ability: dexterity
              dc: 11
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  origin: actor
                  size_ft: 10
              failure:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  average: 7
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              trigger: actor.died
            ignited_illumination:
              economy: bonus_action
              op: state.choose
              parameter: ablaze
              choices:
              - true
              - false
              while_true:
                bright_radius_ft: 10
                dim_additional_ft: 10
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_magmin
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_magmin
    citations:
    - SRD 5.2.1 pp. 305, 306; builtin/srd-5.2.1/creature/magmin.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/marilith:
    source_sha256: 37307d7d7046ac98bf487126986faa89b00bdac6db145e55e00d23bef9e918d0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/marilith
        implementation: dnd2024:creature_marilith
        parameters:
          skill_totals:
            perception: 8
          damage_resistances:
          - cold
          - fire
          - lightning
          damage_immunities:
          - poison
          condition_immunities:
          - poisoned
          senses:
          - sense: truesight
            range_ft: 120
          passive_perception: 18
          languages:
            speak: true
            known:
            - abyssal
          telepathy:
            range_ft: 120
            two_way: true
          features:
            demonic_restoration:
              economy: passive
              trigger: actor.died
              when:
                plane_not: abyss
              body: disappears
              revive:
                new_body: true
                plane: abyss
                location_choice: gm
                hp: maximum
                delay:
                  amount: 0
                  unit: second
            magic_resistance:
              economy: passive
              op: roll.advantage
              rolls:
              - saving_throw
              against:
              - spell
              - magical_effect
            reactive:
              economy: passive
              op: economy.reaction_capacity
              count: 1
              refresh: each_combat_turn
            pact_blade:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 10
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 5
                  average: 10
                - type: necrotic
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  average: 7
            teleport:
              economy: bonus_action
              op: teleport
              maximum_ft: 120
              destination:
                unoccupied: true
                visible_to_actor: true
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            parry:
              economy: reaction
              trigger: melee_attack.hit_actor
              requires:
                holding_weapon: true
              op: ac.add
              amount: 5
              scope: triggering_attack
              recheck_hit: true
            constrict:
              economy: action
              op: saving_throw
              ability: strength
              dc: 17
              targets:
                kind: creature
                count: 1
                range_ft: 5
                maximum_size: medium
                visible_to_actor: true
              failure:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 2
                      sides: 10
                    add: 4
                  average: 15
              - op: grapple
                escape_dc: 14
                while_grappled:
                - op: condition.apply
                  condition: restrained
              success: []
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - feature: pact_blade
                count: 6
              - feature: constrict
                count: 1
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_marilith
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - actor.turn_start
      - creature.feature_requested
      - melee_attack.hit_actor
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_marilith
    citations:
    - SRD 5.2.1 pp. 306; builtin/srd-5.2.1/creature/marilith.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/merfolk_skirmisher:
    source_sha256: ba083663ec04b60a13ddfa2af5db925e6dbe943466c400de8b547c9290b45964
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/merfolk_skirmisher
        implementation: dnd2024:creature_merfolk_skirmisher
        parameters:
          senses: []
          passive_perception: 12
          languages:
            speak: true
            known:
            - common
            - primordial
            primordial_dialects:
            - aquan
          features:
            amphibious:
              economy: passive
              op: breathing.allow
              media:
              - air
              - water
            ocean_spear:
              economy: action
              op: attack
              modes:
              - melee
              - ranged
              attack_bonus: 2
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              range_ft:
                normal: 20
                long: 60
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  average: 3
                - type: cold
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 0
                  average: 2
              - op: speed.add
                amount_ft: -10
                duration:
                  until: target.next_turn_end
                when:
                  target_kind: creature
              after_attack:
                when: ranged_attack_hit_or_miss
                op: item.return
                destination: actor_hand
                magical: true
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_merfolk_skirmisher
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - item_state
      - spatial_constraints
    requires:
    - dnd2024:creature_merfolk_skirmisher
    citations:
    - SRD 5.2.1 pp. 308; builtin/srd-5.2.1/creature/merfolk_skirmisher.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/minotaur_of_baphomet:
    source_sha256: 363f6becc24e6a125af95fb2b6e4f1d992d030a39a488da4b11d2663cd79ef00
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/minotaur_of_baphomet
        implementation: dnd2024:creature_minotaur_of_baphomet
        parameters:
          skill_totals:
            perception: 7
            survival: 7
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 17
          languages:
            speak: true
            known:
            - abyssal
          features:
            abyssal_glaive:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 12
                    add: 4
                  average: 10
                - type: necrotic
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 0
                  average: 10
            gore:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 4
                      sides: 6
                    add: 4
                  average: 18
              - op: conditional
                when:
                  target_maximum_size: large
                  actor_moved_straight_toward_target_ft_at_least: 10
                  immediately_before_hit: true
                  target_kind: creature
                effects:
                - op: damage
                  components:
                  - type: piercing
                    amount:
                      dice:
                        count: 3
                        sides: 6
                      add: 0
                    average: 10
                - op: condition.apply
                  condition: prone
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_minotaur_of_baphomet
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - spatial_constraints
    requires:
    - dnd2024:creature_minotaur_of_baphomet
    citations:
    - SRD 5.2.1 pp. 309; builtin/srd-5.2.1/creature/minotaur_of_baphomet.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/mummy:
    source_sha256: 880ac13322f4bb9054cac2a77f5bb22f3bb256b15d6b2825a1cc73a160ecd901
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/mummy
        implementation: dnd2024:creature_mummy
        parameters:
          damage_vulnerabilities:
          - fire
          damage_immunities:
          - necrotic
          - poison
          condition_immunities:
          - charmed
          - exhaustion
          - frightened
          - paralyzed
          - poisoned
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 11
          languages:
            speak: true
            known:
            - common
            additional_choice_count: 2
          features:
            rotting_fist:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 3
                  average: 8
                - type: necrotic
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 0
                  average: 10
              - op: curse.apply
                when:
                  target_kind: creature
                rules:
                  healing_allowed: false
                  long_rest_restores_hp_maximum: false
                periodic:
                  interval:
                    amount: 24
                    unit: hour
                  effect:
                    op: hp_maximum.reduce
                    amount:
                      dice:
                        count: 3
                        sides: 6
                      add: 0
                    average: 10
              on_reduce_target_to_zero_hp:
                when:
                  target_kind: creature
                op: death
                corpse: dust
            dreadful_glare:
              economy: action
              op: saving_throw
              ability: wisdom
              dc: 11
              targets:
                kind: creature
                count: 1
                range_ft: 60
                visible_to_actor: true
              failure:
              - op: condition.apply
                condition: frightened
                duration:
                  until: actor.next_turn_end
              success:
              - op: feature.immunity
                source: actor
                feature: dreadful_glare
                duration:
                  amount: 24
                  unit: hour
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - feature: rotting_fist
                count: 2
              - feature: dreadful_glare
                count: 1
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_mummy
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_mummy
    citations:
    - SRD 5.2.1 pp. 309; builtin/srd-5.2.1/creature/mummy.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/night_hag:
    source_sha256: 83c049d30cc490f4ed90d4f2798f017c7efcbcaef6d874c8f9e4285c12462ab9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/night_hag
        implementation: dnd2024:creature_night_hag
        parameters:
          skill_totals:
            deception: 6
            insight: 5
            perception: 5
            stealth: 5
          damage_resistances:
          - cold
          - fire
          damage_immunities: []
          condition_immunities:
          - charmed
          senses:
          - sense: darkvision
            range_ft: 120
          passive_perception: 15
          languages:
            speak: true
            known:
            - common
            - infernal
            - abyssal
            - primordial
          features:
            magic_resistance:
              economy: passive
              op: roll.advantage
              rolls:
              - saving_throw
              against:
              - spell
              - magical_effect
            claw:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 4
                  average: 13
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: intelligence
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/etherealness
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/magic_missile
                  uses:
                    unlimited: true
                  cast_level: 4
                - spell:
                    key: dnd2024:spells/phantasmal_killer
                  uses:
                    count: 2
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/plane_shift
                  uses:
                    count: 2
                    recovery: long_rest
                  target_override: self
                save_dc: 14
            coven_magic:
              economy: passive
              op: spellcasting
              requires:
                hag_allies_within_ft: 30
                minimum_allies: 2
              ability: intelligence
              save_dc: 14
              waived_components:
              - material
              casting_time: spell_normal
              spells:
              - spell:
                  key: dnd2024:spells/augury
                uses:
                  count: 1
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/find_familiar
                uses:
                  count: 1
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/identify
                uses:
                  count: 1
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/locate_object
                uses:
                  count: 1
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/scrying
                uses:
                  count: 1
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/unseen_servant
                uses:
                  count: 1
                  recovery: long_rest
            soul_bag:
              economy: passive
              op: equipment.special
              item: soul_bag
              ac: 15
              hp: 20
              damage_resistances:
              - all
              grants_feature: nightmare_haunting
              requires_holding_or_carrying: true
              destroy_at_hp: 0
              on_destroy:
                release_all_souls: true
              create_replacement_after:
                amount: 7
                unit: day
            nightmare_haunting:
              economy: action
              op: spell.cast
              spell:
                key: dnd2024:spells/dream
              profile: spellcasting
              requires:
                actor_plane: ethereal
                holding_or_carrying_item: soul_bag
              target:
                kind: creature
                count: 1
                visible_to_actor: true
                plane: material
              messenger: actor_only
              fails_and_wasted_if_target:
              - spell:
                  key: dnd2024:spells/protection_from_evil_and_good
              - within_spell:
                  key: dnd2024:spells/magic_circle
              on_damage:
                op: hp_maximum.reduce
                amount:
                  event_field: dream_damage_taken
              on_kill:
                op: soul.trap
                container: soul_bag
                cannot_raise_until_soul_released: true
              uses:
                count: 1
                recovery: long_rest
            shape_shift:
              economy: bonus_action
              op: shapechange
              equipment_transformed: false
              statistics_preserved_except:
              - size
              choices:
              - form: humanoid
                sizes:
                - small
                - medium
              - form: true_form
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: claw
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_night_hag
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_lifecycle
      - creature_rules
      - damage
      - effects
      - item_state
      - resources
      - rest_recovery
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_night_hag
    citations:
    - SRD 5.2.1 pp. 311; builtin/srd-5.2.1/creature/night_hag.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ochre_jelly:
    source_sha256: 87132eecdc4f291566d5d4509dbd491f18420d2ed42b19fbaf029a0cd88c9c43
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ochre_jelly
        implementation: dnd2024:creature_ochre_jelly
        parameters:
          damage_resistances:
          - acid
          damage_immunities:
          - lightning
          - slashing
          condition_immunities:
          - charmed
          - deafened
          - exhaustion
          - frightened
          - grappled
          - prone
          - restrained
          senses:
          - sense: blindsight
            range_ft: 60
          passive_perception: 8
          languages:
            speak: true
            known: []
          features:
            amorphous:
              economy: passive
              op: movement.form
              minimum_opening_inches: 1
              extra_movement_cost: 0
            spider_climb:
              economy: passive
              op: movement.climb
              difficult_surfaces: true
              ceilings: true
              ability_check_required: false
            pseudopod:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: acid
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 2
                  average: 12
            split:
              economy: reaction
              trigger_any:
              - actor.becomes_bloodied
              - actor.subjected_to_lightning_damage
              - actor.subjected_to_slashing_damage
              requires:
                sizes:
                - large
                - medium
                hp_at_least: 10
              op: actor.replace_with_creatures
              definition:
                key: dnd2024:creatures/ochre_jelly
              count: 2
              each:
                size_steps: -1
                initiative: original_actor
                hp:
                  divide:
                  - actor_stat: hp
                  - 2
                  round: down
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_ochre_jelly
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_ochre_jelly
    citations:
    - SRD 5.2.1 pp. 312; builtin/srd-5.2.1/creature/ochre_jelly.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ogre_zombie:
    source_sha256: f051984329b625f2d36fe06b85c67b8b72313754ab046ff5ac9419636fc449cf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ogre_zombie
        implementation: dnd2024:creature_ogre_zombie
        parameters:
          damage_immunities:
          - poison
          condition_immunities:
          - exhaustion
          - poisoned
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 8
          languages:
            speak: false
            known:
            - common
            - giant
          features:
            undead_fortitude:
              economy: passive
              trigger: damage.reduces_actor_to_zero_hp
              exclude:
                damage_type: radiant
                critical_hit: true
              save:
                ability: constitution
                dc:
                  add:
                  - 5
                  - event_field: damage_taken
                success:
                  hp_instead: 1
            slam:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 4
                  average: 13
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_ogre_zombie
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      - damage.reduces_actor_to_zero_hp
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_ogre_zombie
    citations:
    - SRD 5.2.1 pp. 344; builtin/srd-5.2.1/creature/ogre_zombie.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/otyugh:
    source_sha256: c60cc53ec8eaea0fbc9532bc54c1c08abf191d2a6deea50ba4919c3c95601e81
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/otyugh
        implementation: dnd2024:creature_otyugh
        parameters:
          senses:
          - sense: darkvision
            range_ft: 120
          passive_perception: 11
          languages:
            speak: true
            known:
            - otyugh
          telepathy:
            range_ft: 120
            two_way: false
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 3
                  average: 12
              - op: condition.apply
                condition: poisoned
                on_target_long_rest_finished:
                  op: saving_throw
                  ability: constitution
                  dc: 15
                  targets:
                    kind: target
                  failure:
                  - op: hp_maximum.reduce
                    amount:
                      dice:
                        count: 1
                        sides: 10
                      add: 0
                    average: 5
                    recovery: this_poisoned_condition_ends
                  success:
                  - op: effect.end
            tentacle:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 3
                  average: 12
              - op: grapple
                escape_dc: 13
                when:
                  target_kind: creature
                  target_maximum_size: medium
                limb_slots: 2
                slots_per_target: 1
            tentacle_slam:
              economy: action
              op: saving_throw
              ability: constitution
              dc: 14
              targets:
                kind: creature
                count: all
                grappled_by: actor
              failure:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 3
                      sides: 8
                    add: 3
                  average: 16
              - op: condition.apply
                condition: stunned
                duration:
                  until: actor.next_turn_start
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 1
                feature: bite
              - count: 2
                feature: tentacle
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_otyugh
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_otyugh
    citations:
    - SRD 5.2.1 pp. 313; builtin/srd-5.2.1/creature/otyugh.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/panther:
    source_sha256: 62aeb99fa2a6cfd0bfaf53097e02537613f60547a2dc9b377cb7c662d7ea0312
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/panther
        implementation: dnd2024:creature_panther
        parameters:
          skill_totals:
            perception: 4
            stealth: 7
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 14
          languages:
            speak: true
            known: []
          features:
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 3
                  average: 6
            nimble_escape:
              economy: bonus_action
              op: action.choose
              choices:
              - disengage
              - hide
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_panther
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_panther
    citations:
    - SRD 5.2.1 pp. 358; builtin/srd-5.2.1/creature/panther.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/piranha:
    source_sha256: f825d1183c56b3b1868517c1b2270571c0723998b779c0b17dc2ce38d59ee699
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/piranha
        implementation: dnd2024:creature_piranha
        parameters:
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 8
          languages:
            speak: true
            known: []
          features:
            water_breathing:
              economy: passive
              op: breathing.allow
              media:
              - water
              exclusive: true
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    constant: 1
                  average: 1
              advantage_when:
                target_hp_below_maximum: true
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_piranha
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_piranha
    citations:
    - SRD 5.2.1 pp. 358; builtin/srd-5.2.1/creature/piranha.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/pit_fiend:
    source_sha256: fe5aaf426ec6abe5ef671fb5cdfd544adc88c7c632ab155f0c0b33b3c91ff3bc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/pit_fiend
        implementation: dnd2024:creature_pit_fiend
        parameters:
          skill_totals:
            perception: 10
            persuasion: 19
          damage_resistances:
          - cold
          damage_immunities:
          - fire
          - poison
          condition_immunities:
          - poisoned
          senses:
          - sense: truesight
            range_ft: 120
          passive_perception: 20
          languages:
            speak: true
            known:
            - infernal
          telepathy:
            range_ft: 120
            two_way: true
          features:
            diabolical_restoration:
              economy: passive
              trigger: actor.died
              when:
                plane_not: nine_hells
              body: disappears
              revive:
                new_body: true
                plane: nine_hells
                location_choice: gm
                hp: maximum
                delay:
                  amount: 0
                  unit: second
            legendary_resistance:
              economy: passive
              trigger: saving_throw.failed
              optional: true
              op: saving_throw.set_result
              result: success
              uses:
                count: 4
                recovery: long_rest
            magic_resistance:
              economy: passive
              op: roll.advantage
              rolls:
              - saving_throw
              against:
              - spell
              - magical_effect
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 14
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 8
                  average: 18
              - op: saving_throw
                ability: constitution
                dc: 21
                targets:
                  kind: target
                  requires_creature: true
                failure:
                - op: condition.apply
                  condition: poisoned
                  duration:
                    amount: 1
                    unit: minute
                  while_active:
                  - op: healing.prohibit
                  on_target_turn_start:
                  - op: damage
                    components:
                    - type: poison
                      amount:
                        dice:
                          count: 6
                          sides: 6
                        add: 0
                      average: 21
                  repeat_save:
                    event: target.turn_end
                    ability: constitution
                    dc: 21
                    success: effect.end
                    automatic_success_after:
                      amount: 1
                      unit: minute
                success: []
            devilish_claw:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 14
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: necrotic
                  amount:
                    dice:
                      count: 4
                      sides: 8
                    add: 8
                  average: 26
            fiery_mace:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 14
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: force
                  amount:
                    dice:
                      count: 4
                      sides: 6
                    add: 8
                  average: 22
                - type: fire
                  amount:
                    dice:
                      count: 6
                      sides: 6
                    add: 0
                  average: 21
            fear_aura:
              economy: passive
              trigger: enemy.turn_start
              requires:
                actor_excluded_conditions:
                - incapacitated
              effect:
                op: saving_throw
                ability: wisdom
                dc: 21
                targets:
                  kind: triggering_enemy
                  count: all
                  area:
                    shape: emanation
                    origin: actor
                    size_ft: 20
                failure:
                - op: condition.apply
                  condition: frightened
                  duration:
                    until: target.next_turn_start
                success:
                - op: feature.immunity
                  source: actor
                  feature: fear_aura
                  duration:
                    amount: 24
                    unit: hour
            hellfire_spellcasting:
              economy: action
              op: spell_sequence
              ability: charisma
              save_dc: 21
              waived_components:
              - material
              base:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/fireball
                cast_level: 5
                count: 2
              replacement:
                maximum: 1
                choices:
                - op: spell.cast
                  spell:
                    key: dnd2024:spells/hold_monster
                  cast_level: 7
                - op: spell.cast
                  spell:
                    key: dnd2024:spells/wall_of_fire
              recharge:
                die_sides: 6
                success_min: 4
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 1
                feature: bite
              - count: 2
                feature: devilish_claw
              - count: 1
                feature: fiery_mace
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_pit_fiend
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - actor.turn_start
      - creature.feature_requested
      - enemy.turn_start
      - saving_throw.failed
      - target.turn_end
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_pit_fiend
    citations:
    - SRD 5.2.1 pp. 314, 315; builtin/srd-5.2.1/creature/pit_fiend.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/polar_bear:
    source_sha256: 365968a3995f6546e19a967cc42f94f27e029012969959fb5fdc3fdd30b43a90
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/polar_bear
        implementation: dnd2024:creature_polar_bear
        parameters:
          skill_totals:
            perception: 5
            stealth: 4
          damage_resistances:
          - cold
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 15
          languages:
            speak: true
            known: []
          features:
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 5
                  average: 9
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: rend
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_polar_bear
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_polar_bear
    citations:
    - SRD 5.2.1 pp. 359; builtin/srd-5.2.1/creature/polar_bear.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/priest_acolyte:
    source_sha256: 38eff9d277d4f90c26149d92da1bacd2eff8f3c0e4c8beee9e715a8729cf287a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/priest_acolyte
        implementation: dnd2024:creature_priest_acolyte
        parameters:
          skill_totals:
            medicine: 4
            religion: 2
          gear:
          - item: chain_shirt
            count: 1
          - item: holy_symbol
            count: 1
          - item: mace
            count: 1
          senses: []
          passive_perception: 12
          languages:
            speak: true
            known:
            - common
          features:
            mace:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 2
                  average: 5
                - type: radiant
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 0
                  average: 2
            radiant_flame:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 60
                long: 60
              hit:
              - op: damage
                components:
                - type: radiant
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  average: 7
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: wisdom
                waived_components: []
                spells:
                - spell:
                    key: dnd2024:spells/light
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/thaumaturgy
                  uses:
                    unlimited: true
            divine_aid:
              economy: bonus_action
              op: spellcasting
              profile:
                ability: wisdom
                waived_components: []
                save_dc: 12
                ability_modifier: 2
                derived_dc:
                  base: 8
                  ability_modifier: 2
                  proficiency_bonus: 2
              choose_count: 1
              choices:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/bless
              - op: spell.cast
                spell:
                  key: dnd2024:spells/healing_word
              - op: spell.cast
                spell:
                  key: dnd2024:spells/sanctuary
              uses:
                count: 1
                recovery: long_rest
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_priest_acolyte
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_priest_acolyte
    citations:
    - SRD 5.2.1 pp. 316; builtin/srd-5.2.1/creature/priest_acolyte.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/purple_worm:
    source_sha256: 3d6c101a8f689bf89121b5b7947995d9e886eb897b4b300ead8417cbe55fba2a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/purple_worm
        implementation: dnd2024:creature_purple_worm
        parameters:
          senses:
          - sense: blindsight
            range_ft: 30
          - sense: tremorsense
            range_ft: 60
          passive_perception: 9
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 14
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 3
                      sides: 8
                    add: 9
                  average: 22
              - op: grapple
                escape_dc: 19
                when:
                  target_kind: creature
                  target_maximum_size: large
                while_grappled:
                - op: condition.apply
                  condition: restrained
            tail_stinger:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 14
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 9
                  average: 16
                - type: poison
                  amount:
                    dice:
                      count: 10
                      sides: 6
                    add: 0
                  average: 35
            tunneler:
              economy: passive
              op: burrow.solid_rock
              speed_multiplier: 0.5
              leaves_tunnel_diameter_ft: 10
            swallow:
              economy: bonus_action
              op: saving_throw
              ability: strength
              dc: 19
              targets:
                kind: creature
                count: 1
                maximum_size: large
                grappled_by: actor
              failure:
              - op: swallow
                end_grapple: true
                conditions:
                - blinded
                - restrained
                total_cover_against: outside_actor
                periodic_damage:
                  event: actor.turn_start
                  damage:
                  - type: acid
                    amount:
                      dice:
                        count: 5
                        sides: 6
                      add: 0
                    average: 17
              success: []
              swallowed_capacity: 3
              regurgitation:
                trigger: turn.end
                ability: constitution
                dc: 21
                failure:
                  targets: all_swallowed_creatures
                  exit_within_ft: 5
                  condition: prone
                when:
                  damage_to_actor_from_one_swallowed_creature_this_turn_at_least: 30
              actor_death:
                remove_condition: restrained
                escape_movement_cost_ft: 20
                exit_condition: prone
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 1
                feature: bite
              - count: 1
                feature: tail_stinger
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_purple_worm
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      - that_turn.end
      - turn.end
      capabilities:
      - action_economy
      - attack_resolution
      - creature_lifecycle
      - creature_rules
      - damage
      - effects
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_purple_worm
    citations:
    - SRD 5.2.1 pp. 316, 317; builtin/srd-5.2.1/creature/purple_worm.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/rat:
    source_sha256: bf795cdefa377e05021441923cc10e33a360b0f2dd603db819418f62b53eca24
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/rat
        implementation: dnd2024:creature_rat
        parameters:
          skill_totals:
            perception: 2
          senses:
          - sense: darkvision
            range_ft: 30
          passive_perception: 12
          languages:
            speak: true
            known: []
          features:
            agile:
              economy: passive
              op: opportunity_attack.suppress
              trigger: actor.leaves_enemy_reach
              movement_modes:
              - all
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 2
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    constant: 1
                  average: 1
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_rat
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.leaves_enemy_reach
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_rat
    citations:
    - SRD 5.2.1 pp. 359; builtin/srd-5.2.1/creature/rat.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/reef_shark:
    source_sha256: 4b1ca433b1db1e6cfcf7e087c7cee0588e0707e6ebaebaea7c977582ed921c94
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/reef_shark
        implementation: dnd2024:creature_reef_shark
        parameters:
          skill_totals:
            perception: 2
          senses:
          - sense: blindsight
            range_ft: 30
          passive_perception: 12
          languages:
            speak: true
            known: []
          features:
            pack_tactics:
              economy: passive
              op: roll.advantage
              rolls:
              - attack
              target_kind: creature
              when:
                ally_within_target_ft: 5
                ally_excluded_conditions:
                - incapacitated
            water_breathing:
              economy: passive
              op: breathing.allow
              media:
              - water
              exclusive: true
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 2
                  average: 7
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_reef_shark
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_reef_shark
    citations:
    - SRD 5.2.1 pp. 360; builtin/srd-5.2.1/creature/reef_shark.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/riding_horse:
    source_sha256: e50e5446f6d857357aa124bc64f8e5e6de42253a056c32c0f4fe1edb3038f99f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/riding_horse
        implementation: dnd2024:creature_riding_horse
        parameters:
          senses: []
          passive_perception: 10
          languages:
            speak: true
            known: []
          features:
            hooves:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 3
                  average: 7
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_riding_horse
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_riding_horse
    citations:
    - SRD 5.2.1 pp. 360; builtin/srd-5.2.1/creature/riding_horse.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/rust_monster:
    source_sha256: 0c1ec2b688ee25631d3d9abbeea4d97116180cd92f557ad67d5669ab9582342c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/rust_monster
        implementation: dnd2024:creature_rust_monster
        parameters:
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 11
          languages:
            speak: true
            known: []
          features:
            iron_scent:
              economy: passive
              op: location.pinpoint
              material: ferrous_metal
              range_ft: 30
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 3
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 1
                  average: 5
            antennae:
              economy: action
              op: item.target
              target:
                kind: object
                magical: false
                material: metal
                categories:
                - armor
                - weapon
                worn_or_carried_by_creature: true
                range_ft: 5
              bearer_save:
                op: saving_throw
                ability: dexterity
                dc: 11
                targets:
                  kind: item_bearer
                failure:
                - op: item.penalty
                  cumulative: true
                  amount: -1
                  armor:
                    stat: provided_ac
                    destroy_at_ac: 10
                  weapon:
                    stat: attack_roll
                    destroy_at_penalty: -5
                  remove_penalty_spell:
                    key: dnd2024:spells/mending
                success: []
            destroy_metal:
              economy: action
              op: object.destroy_volume
              range_ft: 5
              requires_touch: true
              target:
                magical: false
                material: metal
                worn_or_carried: false
              volume:
                shape: cube
                edge_ft: 1
            reflexive_antennae:
              economy: reaction
              trigger: attack.hits_actor
              effect:
                op: feature.use
                feature: antennae
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - feature: bite
                count: 1
              - feature: antennae
                count: 2
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_rust_monster
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - attack.hits_actor
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - item_state
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_rust_monster
    citations:
    - SRD 5.2.1 pp. 320, 321; builtin/srd-5.2.1/creature/rust_monster.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/salamander:
    source_sha256: 4f6b12aa756cd7299e7b658c1ef20bd237c2c27cc84939f013380616a36b5f7f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/salamander
        implementation: dnd2024:creature_salamander
        parameters:
          damage_vulnerabilities:
          - cold
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 10
          languages:
            speak: true
            known:
            - primordial
            primordial_dialects:
            - ignan
          features:
            flame_spear:
              economy: action
              op: attack
              modes:
              - melee
              - ranged
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              range_ft:
                normal: 20
                long: 60
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 4
                  average: 13
                - type: fire
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  average: 7
              after_attack:
                when: ranged_attack_hit_or_miss
                op: item.return
                destination: actor_hand
                magical: true
            fire_aura:
              economy: passive
              trigger: actor.turn_end
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  origin: actor
                  size_ft: 5
                actor_selects: true
              effects:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  average: 7
            constrict:
              economy: action
              op: saving_throw
              ability: strength
              dc: 15
              targets:
                kind: creature
                count: 1
                range_ft: 10
                maximum_size: large
                visible_to_actor: true
              failure:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 4
                  average: 11
                - type: fire
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  average: 7
              - op: grapple
                escape_dc: 14
                while_grappled:
                - op: condition.apply
                  condition: restrained
              success: []
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: flame_spear
              replace_attack:
                maximum: 1
                choices:
                - op: feature.use
                  feature: constrict
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_salamander
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_end
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - item_state
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_salamander
    citations:
    - SRD 5.2.1 pp. 321; builtin/srd-5.2.1/creature/salamander.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/scout:
    source_sha256: 900cef9dcdf665cf58143d407b0ee0345248cad9fe48100c670de9f487b42254
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/scout
        implementation: dnd2024:creature_scout
        parameters:
          skill_totals:
            nature: 4
            perception: 5
            stealth: 6
            survival: 5
          gear:
          - item: leather_armor
            count: 1
          - item: longbow
            count: 1
          - item: shortsword
            count: 1
          senses: []
          passive_perception: 15
          languages:
            speak: true
            known:
            - common
            additional_choice_count: 1
          features:
            shortsword:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 2
                  average: 5
            longbow:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 150
                long: 600
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 2
                  average: 6
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 2
                choices:
                - shortsword
                - longbow
                combination: any
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_scout
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_scout
    citations:
    - SRD 5.2.1 pp. 322; builtin/srd-5.2.1/creature/scout.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/shadow:
    source_sha256: c021169c2efec8a5f699b5cd7331596c37c0fff44b53dc1efcbb685bcbd2672b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/shadow
        implementation: dnd2024:creature_shadow
        parameters:
          skill_totals:
            stealth: 6
          damage_vulnerabilities:
          - radiant
          damage_resistances:
          - acid
          - cold
          - fire
          - lightning
          - thunder
          damage_immunities:
          - necrotic
          - poison
          condition_immunities:
          - exhaustion
          - frightened
          - grappled
          - paralyzed
          - petrified
          - poisoned
          - prone
          - restrained
          - unconscious
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 10
          languages:
            speak: true
            known: []
          features:
            amorphous:
              economy: passive
              op: movement.form
              minimum_opening_inches: 1
              extra_movement_cost: 0
            sunlight_weakness:
              economy: passive
              op: roll.disadvantage
              when:
                in_sunlight: true
              rolls:
              - all_d20_tests
            draining_swipe:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: necrotic
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 2
                  average: 5
              - op: ability_score.reduce
                ability: strength
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 0
                death_at_zero: true
              on_humanoid_slain:
                op: undead.create
                definition:
                  key: dnd2024:creatures/shadow
                delay:
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 0
                  unit: hour
            shadow_stealth:
              economy: bonus_action
              op: action.use
              action: hide
              when:
                lighting:
                - dim_light
                - darkness
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_shadow
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_lifecycle
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_shadow
    citations:
    - SRD 5.2.1 pp. 322, 323; builtin/srd-5.2.1/creature/shadow.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/shrieker_fungus:
    source_sha256: 559066b8dc2dd3feff7c571a9c829cef02e410ef8c10b2411fe73d5b2c3cda3a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/shrieker_fungus
        implementation: dnd2024:creature_shrieker_fungus
        parameters:
          damage_immunities: []
          condition_immunities:
          - blinded
          - charmed
          - deafened
          - frightened
          senses:
          - sense: blindsight
            range_ft: 30
          passive_perception: 6
          languages:
            speak: true
            known: []
          features:
            shriek:
              economy: reaction
              trigger: entity.moved
              op: sound.emit
              audible_radius_ft: 300
              duration:
                maximum:
                  amount: 1
                  unit: minute
                until: actor.died
              when:
                entity_kind_any:
                - creature
                - bright_light_source
                distance_to_actor_ft_at_most: 30
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_shrieker_fungus
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      - entity.moved
      capabilities:
      - action_economy
      - creature_rules
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_shrieker_fungus
    citations:
    - SRD 5.2.1 pp. 286; builtin/srd-5.2.1/creature/shrieker_fungus.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/solar:
    source_sha256: ec33a906a4ad6668b02bed421f483eb478c1f3d7b206bc0fb93291461a706d9f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/solar
        implementation: dnd2024:creature_solar
        parameters:
          skill_totals:
            perception: 14
          damage_immunities:
          - poison
          - radiant
          condition_immunities:
          - charmed
          - exhaustion
          - frightened
          - poisoned
          senses:
          - sense: truesight
            range_ft: 120
          passive_perception: 24
          languages:
            speak: true
            known: []
            all: true
          telepathy:
            range_ft: 120
            two_way: true
          features:
            divine_awareness:
              economy: passive
              op: lie.detect
              requires: hear_lie
              automatic: true
            exalted_restoration:
              economy: passive
              trigger: actor.died
              when:
                plane_not: mount_celestia
              body: disappears
              revive:
                new_body: true
                plane: mount_celestia
                location_choice: gm
                hp: maximum
                delay:
                  amount: 0
                  unit: second
            legendary_resistance:
              economy: passive
              trigger: saving_throw.failed
              optional: true
              op: saving_throw.set_result
              result: success
              uses:
                count: 4
                recovery: long_rest
            magic_resistance:
              economy: passive
              op: roll.advantage
              rolls:
              - saving_throw
              against:
              - spell
              - magical_effect
            flying_sword:
              economy: action
              op: attack
              modes:
              - melee
              - ranged
              attack_bonus: 15
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              range_ft:
                normal: 120
                long: 120
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 4
                      sides: 6
                    add: 8
                  average: 22
                - type: radiant
                  amount:
                    dice:
                      count: 8
                      sides: 8
                    add: 0
                  average: 36
              after_attack:
                when: ranged_attack_hit_or_miss
                op: item.return
                destination: actor_hand
                magical: true
                destination_choice:
                - actor_hand
                - hover_within_5_ft
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: charisma
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/detect_evil_and_good
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/commune
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/control_weather
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/dispel_evil_and_good
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/resurrection
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 25
            slaying_bow:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 21
              targets:
                kind: creature
                count: 1
                range_ft: 600
                visible_to_actor: true
              failure:
              - op: branch
                if:
                  target_hp_at_most: 100
                then:
                - op: death
                else:
                - op: damage
                  components:
                  - type: piercing
                    amount:
                      dice:
                        count: 4
                        sides: 8
                      add: 6
                    average: 24
                  - type: radiant
                    amount:
                      dice:
                        count: 8
                        sides: 8
                      add: 0
                    average: 36
              success: []
            divine_aid:
              economy: bonus_action
              op: spellcasting
              profile:
                ability: same_as_spellcasting
                waived_components: []
              choose_count: 1
              choices:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/cure_wounds
                cast_level: 2
              - op: spell.cast
                spell:
                  key: dnd2024:spells/lesser_restoration
              - op: spell.cast
                spell:
                  key: dnd2024:spells/remove_curse
              uses:
                count: 3
                recovery: long_rest
            blinding_gaze:
              economy: legendary_action
              op: saving_throw
              ability: constitution
              dc: 25
              targets:
                kind: creature
                count: 1
                range_ft: 120
                visible_to_actor: true
              failure:
              - op: condition.apply
                condition: blinded
                duration:
                  amount: 1
                  unit: minute
              success: []
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            radiant_teleport:
              economy: legendary_action
              op: sequence
              steps:
              - op: teleport
                maximum_ft: 60
                destination:
                  unoccupied: true
                  visible_to_actor: true
              - op: saving_throw
                ability: dexterity
                dc: 25
                targets:
                  kind: creature
                  count: all
                  area:
                    shape: emanation
                    origin: actor
                    size_ft: 10
                failure:
                - op: damage
                  components:
                  - type: radiant
                    amount:
                      dice:
                        count: 2
                        sides: 10
                      add: 0
                    average: 11
                success:
                - op: damage
                  from: failure_damage
                  multiplier: 0.5
                  round: down
              legendary_cost: 1
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: flying_sword
              replace_attack:
                maximum: 1
                choices:
                - op: feature.use
                  feature: slaying_bow
          legendary_actions:
            uses: 3
            uses_in_lair: 3
            trigger: another_creature.turn_end_immediately_after
            cost_per_action: 1
            recover_all: actor.turn_start
            maximum_per_trigger: 1
            requires_actions_available: true
            excluded_conditions:
            - incapacitated
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_solar
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - actor.turn_start
      - another_creature.turn_end_immediately_after
      - creature.feature_requested
      - saving_throw.failed
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - item_state
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_solar
    citations:
    - SRD 5.2.1 pp. 326, 327; builtin/srd-5.2.1/creature/solar.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/sphinx_of_valor:
    source_sha256: d5ccead7653d795397fc813619a34a0eafe649659f679e53e1618e42cbea1d82
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/sphinx_of_valor
        implementation: dnd2024:creature_sphinx_of_valor
        parameters:
          skill_totals:
            arcana: 9
            perception: 12
            religion: 15
          damage_resistances:
          - necrotic
          - radiant
          damage_immunities:
          - psychic
          condition_immunities:
          - charmed
          - frightened
          senses:
          - sense: truesight
            range_ft: 120
          passive_perception: 22
          languages:
            speak: true
            known:
            - common
            - celestial
          features:
            legendary_resistance:
              economy: passive
              trigger: saving_throw.failed
              optional: true
              op: saving_throw.set_result
              result: success
              uses_in_lair: 4
              uses:
                count: 3
                recovery: long_rest
            claw:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 12
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 4
                      sides: 6
                    add: 6
                  average: 20
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: wisdom
                waived_components:
                - material
                spells:
                - spell:
                    key: dnd2024:spells/detect_evil_and_good
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/thaumaturgy
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/dispel_magic
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/greater_restoration
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/heroes_feast
                  uses:
                    count: 1
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/zone_of_truth
                  uses:
                    count: 1
                    recovery: long_rest
                save_dc: 20
            inscrutable:
              economy: passive
              magic_permissions:
                remote_observation: requires_actor_consent
                thought_detection: requires_actor_consent
              disadvantage_on_checks:
                ability: wisdom
                skill: insight
                purpose:
                - intentions
                - sincerity
            roar:
              economy: action
              op: sequence_by_use
              magical: true
              sequence_reset: long_rest
              effects:
              - op: saving_throw
                ability: wisdom
                dc: 20
                targets:
                  kind: creature
                  count: all
                  area:
                    shape: emanation
                    origin: actor
                    size_ft: 500
                  allegiance: enemy
                failure:
                - op: condition.apply
                  condition: frightened
                  duration:
                    amount: 1
                    unit: minute
                success: []
              - op: saving_throw
                ability: wisdom
                dc: 20
                targets:
                  kind: creature
                  count: all
                  area:
                    shape: emanation
                    origin: actor
                    size_ft: 500
                  allegiance: enemy
                failure:
                - op: condition.apply
                  condition: paralyzed
                  duration:
                    amount: 1
                    unit: minute
                  repeat_save:
                    event: target.turn_end
                    ability: wisdom
                    dc: 20
                    success: effect.end
                    automatic_success_after:
                      amount: 1
                      unit: minute
                success: []
              - op: saving_throw
                ability: constitution
                dc: 20
                targets:
                  kind: creature
                  count: all
                  area:
                    shape: emanation
                    origin: actor
                    size_ft: 500
                  allegiance: enemy
                failure:
                - op: damage
                  components:
                  - type: thunder
                    amount:
                      dice:
                        count: 8
                        sides: 10
                      add: 0
                    average: 44
                - op: condition.apply
                  condition: prone
                success:
                - op: damage
                  from: failure_damage
                  multiplier: 0.5
                  round: down
              uses:
                count: 3
                recovery: long_rest
            arcane_prowl:
              economy: legendary_action
              op: sequence
              steps:
              - op: teleport
                optional: true
                maximum_ft: 30
                destination:
                  unoccupied: true
                  visible_to_actor: true
              - op: feature.use
                feature: claw
              legendary_cost: 1
            weight_of_years:
              economy: legendary_action
              op: saving_throw
              ability: constitution
              dc: 16
              targets:
                kind: creature
                count: 1
                range_ft: 120
                visible_to_actor: true
              failure:
              - op: exhaustion.add
                levels: 1
              - op: appearance.age
                years:
                  dice:
                    count: 3
                    sides: 10
                  add: 0
                duration:
                  until: target.has_no_exhaustion
              success: []
              lockout_until: actor.next_turn_start
              legendary_cost: 1
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - feature: claw
                count: 2
              - feature: roar
                count: 1
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            trigger: another_creature.turn_end_immediately_after
            cost_per_action: 1
            recover_all: actor.turn_start
            maximum_per_trigger: 1
            requires_actions_available: true
            excluded_conditions:
            - incapacitated
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_sphinx_of_valor
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - another_creature.turn_end_immediately_after
      - creature.feature_requested
      - saving_throw.failed
      - target.turn_end
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_sphinx_of_valor
    citations:
    - SRD 5.2.1 pp. 328; builtin/srd-5.2.1/creature/sphinx_of_valor.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/spirit_naga:
    source_sha256: c10ae266c7197f97bc314fe4fb1f74c89e8ff8c960f0d36465cac4575d4d22cc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/spirit_naga
        implementation: dnd2024:creature_spirit_naga
        parameters:
          damage_immunities:
          - poison
          condition_immunities:
          - charmed
          - poisoned
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 12
          languages:
            speak: true
            known:
            - common
            - abyssal
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 4
                  average: 7
                - type: poison
                  amount:
                    dice:
                      count: 4
                      sides: 6
                    add: 0
                  average: 14
            necrotic_ray:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 60
                long: 60
              hit:
              - op: damage
                components:
                - type: necrotic
                  amount:
                    dice:
                      count: 6
                      sides: 6
                    add: 0
                  average: 21
            spellcasting:
              economy: action
              op: spellcasting
              profile:
                ability: intelligence
                waived_components:
                - somatic
                - material
                spells:
                - spell:
                    key: dnd2024:spells/detect_magic
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/mage_hand
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/minor_illusion
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/water_breathing
                  uses:
                    unlimited: true
                - spell:
                    key: dnd2024:spells/detect_thoughts
                  uses:
                    count: 2
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/dimension_door
                  uses:
                    count: 2
                    recovery: long_rest
                - spell:
                    key: dnd2024:spells/hold_person
                  uses:
                    count: 2
                    recovery: long_rest
                  cast_level: 3
                - spell:
                    key: dnd2024:spells/lightning_bolt
                  uses:
                    count: 2
                    recovery: long_rest
                  cast_level: 4
                save_dc: 14
            fiendish_restoration:
              economy: passive
              trigger: actor.died
              op: revive
              hp: maximum
              delay:
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
                unit: day
              only_prevention_spell:
                key: dnd2024:spells/wish
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 3
                choices:
                - bite
                - necrotic_ray
                combination: any
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_spirit_naga
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_lifecycle
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_spirit_naga
    citations:
    - SRD 5.2.1 pp. 329; builtin/srd-5.2.1/creature/spirit_naga.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/steam_mephit:
    source_sha256: 7b810dec92c6de713210a4fbb647b9ac3ba0f4932ca238cce85fcd441c2b2df3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/steam_mephit
        implementation: dnd2024:creature_steam_mephit
        parameters:
          skill_totals:
            stealth: 2
          damage_immunities:
          - fire
          - poison
          condition_immunities:
          - exhaustion
          - poisoned
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 10
          languages:
            speak: true
            known:
            - primordial
            primordial_dialects:
            - aquan
            - ignan
          features:
            blurred_form:
              economy: passive
              op: roll.disadvantage
              rolls:
              - attack_against_actor
              unless:
                actor_condition: incapacitated
            claw:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 2
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 0
                  average: 2
                - type: fire
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 0
                  average: 2
            death_burst:
              economy: passive
              op: saving_throw
              ability: dexterity
              dc: 10
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  origin: actor
                  size_ft: 5
              failure:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 0
                  average: 5
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              trigger: actor.died
            steam_breath:
              economy: action
              op: saving_throw
              ability: constitution
              dc: 10
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 15
              failure:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 0
                  average: 5
              - op: speed.add
                amount_ft: -10
                duration:
                  until: actor.next_turn_end
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              ignore_resistance_sources:
              - underwater
              recharge:
                die_sides: 6
                success_min: 6
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_steam_mephit
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.died
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_steam_mephit
    citations:
    - SRD 5.2.1 pp. 308; builtin/srd-5.2.1/creature/steam_mephit.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/stone_golem:
    source_sha256: b50dcea5e27275eb1097a62368d56fe083aaceee480e9bf0062a242a5c9a48d2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/stone_golem
        implementation: dnd2024:creature_stone_golem
        parameters:
          damage_immunities:
          - poison
          - psychic
          condition_immunities:
          - charmed
          - exhaustion
          - frightened
          - paralyzed
          - petrified
          - poisoned
          senses:
          - sense: darkvision
            range_ft: 120
          passive_perception: 10
          languages:
            speak: false
            known:
            - common
            additional_choice_count: 2
          features:
            immutable_form:
              economy: passive
              op: shapechange.prohibit
            magic_resistance:
              economy: passive
              op: roll.advantage
              rolls:
              - saving_throw
              against:
              - spell
              - magical_effect
            slam:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 10
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 6
                  average: 15
                - type: force
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 0
                  average: 9
            force_bolt:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 9
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 120
                long: 120
              hit:
              - op: damage
                components:
                - type: force
                  amount:
                    dice:
                      count: 4
                      sides: 10
                    add: 0
                  average: 22
            slow:
              economy: bonus_action
              op: spellcasting
              profile:
                ability: constitution
                waived_components:
                - verbal
                - somatic
                - material
                save_dc: 17
              choose_count: 1
              choices:
              - op: spell.cast
                spell:
                  key: dnd2024:spells/slow
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 2
                choices:
                - slam
                - force_bolt
                combination: any
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spellcasting_defaults:
            cast_level: lowest_possible_unless_overridden
            upcast_allowed: only_explicit_cast_level_overrides
            required_material_components_available: true
            casting_time_at_least_1_minute:
              uses_normal_time: true
              magic_action_each_turn: true
              concentration_required: true
            other_components: spell_definition_unless_waived
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_stone_golem
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_lifecycle
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - spatial_constraints
      - spellcasting
    requires:
    - dnd2024:creature_stone_golem
    citations:
    - SRD 5.2.1 pp. 330; builtin/srd-5.2.1/creature/stone_golem.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/swarm_of_bats:
    source_sha256: 4f493610bb2a33210f05c068ad38c7fc9b9bab6da7ca0cf2f578857ab3dedb9c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/swarm_of_bats
        implementation: dnd2024:creature_swarm_of_bats
        parameters:
          damage_resistances:
          - bludgeoning
          - piercing
          - slashing
          damage_immunities: []
          condition_immunities:
          - charmed
          - frightened
          - grappled
          - paralyzed
          - petrified
          - prone
          - restrained
          - stunned
          senses:
          - sense: blindsight
            range_ft: 60
          passive_perception: 11
          languages:
            speak: true
            known: []
          features:
            swarm:
              economy: passive
              op: swarm.form
              shared_spaces_bidirectional: true
              minimum_opening:
                creature_size: tiny
                creature_shape: bat
              regain_hp: false
              gain_temporary_hp: false
            bites:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 0
                  average: 5
                replacement_when:
                  if:
                    actor_bloodied: true
                  components:
                  - type: piercing
                    amount:
                      dice:
                        count: 1
                        sides: 4
                      add: 0
                    average: 2
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_swarm_of_bats
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_swarm_of_bats
    citations:
    - SRD 5.2.1 pp. 361; builtin/srd-5.2.1/creature/swarm_of_bats.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/swarm_of_piranhas:
    source_sha256: 1a8f2279eb33d185ace9ec6daae7787077ac379df79ee3247a9318470fcebeb5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/swarm_of_piranhas
        implementation: dnd2024:creature_swarm_of_piranhas
        parameters:
          damage_resistances:
          - bludgeoning
          - piercing
          - slashing
          damage_immunities: []
          condition_immunities:
          - charmed
          - frightened
          - grappled
          - paralyzed
          - petrified
          - prone
          - restrained
          - stunned
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 8
          languages:
            speak: true
            known: []
          features:
            swarm:
              economy: passive
              op: swarm.form
              shared_spaces_bidirectional: true
              minimum_opening:
                creature_size: tiny
                creature_shape: piranha
              regain_hp: false
              gain_temporary_hp: false
            water_breathing:
              economy: passive
              op: breathing.allow
              media:
              - water
              exclusive: true
            bites:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 3
                  average: 8
                replacement_when:
                  if:
                    actor_bloodied: true
                  components:
                  - type: piercing
                    amount:
                      dice:
                        count: 1
                        sides: 4
                      add: 3
                    average: 5
              advantage_when:
                target_hp_below_maximum: true
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_swarm_of_piranhas
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_swarm_of_piranhas
    citations:
    - SRD 5.2.1 pp. 362; builtin/srd-5.2.1/creature/swarm_of_piranhas.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/swarm_of_venomous_snakes:
    source_sha256: 4b0649adb9f6d7d1efba3627b71493202f2408e04cacdb53d402ec2a28e1a031
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/swarm_of_venomous_snakes
        implementation: dnd2024:creature_swarm_of_venomous_snakes
        parameters:
          damage_resistances:
          - bludgeoning
          - piercing
          - slashing
          damage_immunities: []
          condition_immunities:
          - charmed
          - frightened
          - grappled
          - paralyzed
          - petrified
          - prone
          - restrained
          - stunned
          senses:
          - sense: blindsight
            range_ft: 10
          passive_perception: 10
          languages:
            speak: true
            known: []
          features:
            swarm:
              economy: passive
              op: swarm.form
              shared_spaces_bidirectional: true
              minimum_opening:
                creature_size: tiny
                creature_shape: snake
              regain_hp: false
              gain_temporary_hp: false
            bites:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 4
                  average: 8
                - type: poison
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 0
                  average: 10
                replacement_when:
                  if:
                    actor_bloodied: true
                  components:
                  - type: piercing
                    amount:
                      dice:
                        count: 1
                        sides: 4
                      add: 4
                    average: 6
                  - type: poison
                    amount:
                      dice:
                        count: 3
                        sides: 6
                      add: 0
                    average: 10
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_swarm_of_venomous_snakes
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_swarm_of_venomous_snakes
    citations:
    - SRD 5.2.1 pp. 362, 363; builtin/srd-5.2.1/creature/swarm_of_venomous_snakes.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/tough:
    source_sha256: 708b88aba78c3ce7bf1db2baed251412e415254e4d9ef8c59bc6c05b71d6738d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/tough
        implementation: dnd2024:creature_tough
        parameters:
          gear:
          - item: heavy_crossbow
            count: 1
          - item: leather_armor
            count: 1
          - item: mace
            count: 1
          senses: []
          passive_perception: 10
          languages:
            speak: true
            known:
            - common
          features:
            pack_tactics:
              economy: passive
              op: roll.advantage
              rolls:
              - attack
              target_kind: creature
              when:
                ally_within_target_ft: 5
                ally_excluded_conditions:
                - incapacitated
            mace:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 2
                  average: 5
            heavy_crossbow:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 3
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 100
                long: 400
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 1
                  average: 6
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_tough
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_tough
    citations:
    - SRD 5.2.1 pp. 332; builtin/srd-5.2.1/creature/tough.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/triceratops:
    source_sha256: 19a6938d098772788eac13306cfdd4d0151c384a1f243994f470b495111392ba
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/triceratops
        implementation: dnd2024:creature_triceratops
        parameters:
          senses: []
          passive_perception: 10
          languages:
            speak: true
            known: []
          features:
            gore:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 9
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 12
                    add: 6
                  average: 19
              - op: conditional
                when:
                  target_maximum_size: huge
                  actor_moved_straight_toward_target_ft_at_least: 20
                  immediately_before_hit: true
                effects:
                - op: damage
                  components:
                  - type: piercing
                    amount:
                      dice:
                        count: 2
                        sides: 8
                      add: 0
                    average: 9
                - op: condition.apply
                  condition: prone
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: gore
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_triceratops
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_triceratops
    citations:
    - SRD 5.2.1 pp. 363; builtin/srd-5.2.1/creature/triceratops.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/tyrannosaurus_rex:
    source_sha256: 384fc33402835d482e62e6269c71cab05e5350bb22e16b05bdb5641449fad59b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/tyrannosaurus_rex
        implementation: dnd2024:creature_tyrannosaurus_rex
        parameters:
          skill_totals:
            perception: 4
          senses: []
          passive_perception: 14
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 10
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 4
                      sides: 12
                    add: 7
                  average: 33
              - op: grapple
                escape_dc: 17
                when:
                  target_kind: creature
                  target_maximum_size: large
                while_grappled:
                - op: condition.apply
                  condition: restrained
                - op: targeting.prohibit
                  feature: tail
                  source: actor
            tail:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 10
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 15
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 4
                      sides: 8
                    add: 7
                  average: 25
              - op: condition.apply
                condition: prone
                when:
                  target_kind: creature
                  target_maximum_size: huge
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 1
                feature: bite
              - count: 1
                feature: tail
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_tyrannosaurus_rex
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_tyrannosaurus_rex
    citations:
    - SRD 5.2.1 pp. 363; builtin/srd-5.2.1/creature/tyrannosaurus_rex.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/vampire_familiar:
    source_sha256: 02e59457899f4fb93f6aa0ec084eda8150e689a426bfe8680e691d69cc1fa85a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/vampire_familiar
        implementation: dnd2024:creature_vampire_familiar
        parameters:
          skill_totals:
            perception: 4
            persuasion: 4
            stealth: 7
          damage_resistances:
          - necrotic
          damage_immunities: []
          condition_immunities:
          - condition: charmed
            except_source: vampire_master
          gear:
          - item: daggers
            count: 10
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 14
          languages:
            speak: true
            known:
            - common
            additional_choice_count: 1
          features:
            umbral_dagger:
              economy: action
              op: attack
              modes:
              - melee
              - ranged
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              range_ft:
                normal: 20
                long: 60
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 3
                  average: 5
                - type: necrotic
                  amount:
                    dice:
                      count: 3
                      sides: 4
                    add: 0
                  average: 7
              on_reduce_target_to_zero_hp:
                effects:
                - op: stable.set
                  value: true
                - op: condition.apply
                  condition: poisoned
                  duration:
                    amount: 1
                    unit: hour
                  while_active:
                  - op: condition.apply
                    condition: paralyzed
            deathless_agility:
              economy: bonus_action
              op: action.choose
              choices:
              - dash
              - disengage
            vampiric_connection:
              economy: passive
              when:
                same_plane_as: vampire_master
              master_can:
                telepathic_communication: true
                perceive_through_actor_senses: true
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: umbral_dagger
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_vampire_familiar
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_vampire_familiar
    citations:
    - SRD 5.2.1 pp. 334; builtin/srd-5.2.1/creature/vampire_familiar.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/violet_fungus:
    source_sha256: d80b13bbe7435df46b14f12ce900a0e5f528277c94f68fd309f3bc79142a532a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/violet_fungus
        implementation: dnd2024:creature_violet_fungus
        parameters:
          damage_immunities: []
          condition_immunities:
          - blinded
          - charmed
          - deafened
          - frightened
          senses:
          - sense: blindsight
            range_ft: 30
          passive_perception: 6
          languages:
            speak: true
            known: []
          features:
            rotting_touch:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 2
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: necrotic
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 0
                  average: 4
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 2
                feature: rotting_touch
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_violet_fungus
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_violet_fungus
    citations:
    - SRD 5.2.1 pp. 286; builtin/srd-5.2.1/creature/violet_fungus.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/warhorse:
    source_sha256: 63781390b968bd592c6a86c24363192552990d885c967c0fe42e2e9b4aecf74f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/warhorse
        implementation: dnd2024:creature_warhorse
        parameters:
          senses: []
          passive_perception: 11
          languages:
            speak: true
            known: []
          features:
            hooves:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 6
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 4
                  average: 9
              - op: conditional
                when:
                  target_maximum_size: large
                  actor_moved_straight_toward_target_ft_at_least: 20
                  immediately_before_hit: true
                  target_kind: creature
                effects:
                - op: damage
                  components:
                  - type: bludgeoning
                    amount:
                      dice:
                        count: 2
                        sides: 4
                      add: 0
                    average: 5
                - op: condition.apply
                  condition: prone
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_warhorse
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_warhorse
    citations:
    - SRD 5.2.1 pp. 364; builtin/srd-5.2.1/creature/warhorse.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/warrior_veteran:
    source_sha256: 05dfa7ad7cfeb17b6b3ac4fdb5914230c6a37d7e24a7e4e13ffa220a1765061a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/warrior_veteran
        implementation: dnd2024:creature_warrior_veteran
        parameters:
          skill_totals:
            athletics: 5
            perception: 2
          gear:
          - item: greatsword
            count: 1
          - item: heavy_crossbow
            count: 1
          - item: splint_armor
            count: 1
          senses: []
          passive_perception: 12
          languages:
            speak: true
            known:
            - common
            additional_choice_count: 1
          features:
            greatsword:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 3
                  average: 10
            heavy_crossbow:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 3
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 100
                long: 400
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 10
                    add: 1
                  average: 12
            parry:
              economy: reaction
              trigger: melee_attack.hit_actor
              requires:
                holding_weapon: true
              op: ac.add
              amount: 2
              scope: triggering_attack
              recheck_hit: true
            multiattack:
              economy: action
              op: multiattack
              choose_sequence:
              - feature: greatsword
                count: 2
              - feature: heavy_crossbow
                count: 2
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_warrior_veteran
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      - melee_attack.hit_actor
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_warrior_veteran
    citations:
    - SRD 5.2.1 pp. 337; builtin/srd-5.2.1/creature/warrior_veteran.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/werebear:
    source_sha256: 4345e17bf79a3c7ad2f0fec173a60b65d4c746201b1ecca4104b100011075ee6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/werebear
        implementation: dnd2024:creature_werebear
        parameters:
          skill_totals:
            perception: 7
          gear:
          - item: handaxes
            count: 4
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 17
          languages:
            speak: true
            known:
            - common
            speech_prohibited_forms:
            - bear
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 12
                    add: 4
                  average: 17
              - op: saving_throw
                ability: constitution
                dc: 14
                targets:
                  kind: target
                  creature_types:
                  - humanoid
                failure:
                - op: curse.apply
                  on_target_zero_hp:
                    instead:
                      op: stat_block.replace
                      definition:
                        key: dnd2024:creatures/werebear
                      controller: gm
                      hp: 10
                success:
                - op: feature.immunity
                  source: actor
                  feature: bite_curse
                  duration:
                    amount: 24
                    unit: hour
                effect_identity: bite_curse
              allowed_forms:
              - bear
              - hybrid
            handaxe:
              economy: action
              op: attack
              modes:
              - melee
              - ranged
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              range_ft:
                normal: 20
                long: 60
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 4
                  average: 14
              allowed_forms:
              - humanoid
              - hybrid
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 4
                  average: 13
              allowed_forms:
              - bear
              - hybrid
            shape_shift:
              economy: bonus_action
              op: shapechange
              equipment_transformed: false
              statistics_preserved_except:
              - size
              choices:
              - form: hybrid
                size: large
                animal: bear
              - form: bear
                size: large
              - form: true_humanoid
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 2
                choices:
                - handaxe
                - rend
                combination: any
              replace_attack:
                maximum: 1
                choices:
                - op: feature.use
                  feature: bite
          form_overrides:
            bear:
              speed_ft: 40
              climb_speed_ft: 30
              can_speak: false
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_werebear
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_lifecycle
      - creature_rules
      - damage
      - effects
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_werebear
    citations:
    - SRD 5.2.1 pp. 337, 338; builtin/srd-5.2.1/creature/werebear.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/weretiger:
    source_sha256: acf65a90ffdd570a2b0ea336194486cce264f26b8d07db630f0302ab4d3e18b5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/weretiger
        implementation: dnd2024:creature_weretiger
        parameters:
          skill_totals:
            perception: 5
            stealth: 4
          gear:
          - item: longbow
            count: 1
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 15
          languages:
            speak: true
            known:
            - common
            speech_prohibited_forms:
            - tiger
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 3
                  average: 12
              - op: saving_throw
                ability: constitution
                dc: 13
                targets:
                  kind: target
                  creature_types:
                  - humanoid
                failure:
                - op: curse.apply
                  on_target_zero_hp:
                    instead:
                      op: stat_block.replace
                      definition:
                        key: dnd2024:creatures/weretiger
                      controller: gm
                      hp: 10
                success:
                - op: feature.immunity
                  source: actor
                  feature: bite_curse
                  duration:
                    amount: 24
                    unit: hour
                effect_identity: bite_curse
              allowed_forms:
              - tiger
              - hybrid
            scratch:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 5
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 3
                  average: 10
            longbow:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 150
                long: 600
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 2
                  average: 11
              allowed_forms:
              - humanoid
              - hybrid
            prowl:
              economy: bonus_action
              allowed_forms:
              - tiger
              - hybrid
              op: sequence
              steps:
              - op: movement
                maximum:
                  multiply:
                  - 1
                  - actor_stat: speed
                opportunity_attacks: false
              - op: action.use
                action: hide
                optional: true
            shape_shift:
              economy: bonus_action
              op: shapechange
              equipment_transformed: false
              statistics_preserved_except:
              - size
              choices:
              - form: hybrid
                size: large
                animal: tiger
              - form: tiger
                size: large
              - form: true_humanoid
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 2
                choices:
                - scratch
                - longbow
                combination: any
              replace_attack:
                maximum: 1
                choices:
                - op: feature.use
                  feature: bite
          form_overrides:
            tiger:
              speed_ft: 40
              can_speak: false
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_weretiger
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_lifecycle
      - creature_rules
      - damage
      - effects
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_weretiger
    citations:
    - SRD 5.2.1 pp. 339; builtin/srd-5.2.1/creature/weretiger.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/wight:
    source_sha256: 51630a328918c53067da92182c38096c7a8607c299e485bdc566853bf765c104
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/wight
        implementation: dnd2024:creature_wight
        parameters:
          skill_totals:
            perception: 3
            stealth: 4
          damage_resistances:
          - necrotic
          damage_immunities:
          - poison
          condition_immunities:
          - exhaustion
          - poisoned
          gear:
          - item: studded_leather_armor
            count: 1
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 13
          languages:
            speak: true
            known:
            - common
            additional_choice_count: 1
          features:
            sunlight_sensitivity:
              economy: passive
              op: roll.disadvantage
              when:
                in_sunlight: true
              rolls:
              - ability_check
              - attack
            necrotic_sword:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 2
                  average: 6
                - type: necrotic
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 0
                  average: 4
            necrotic_bow:
              economy: action
              op: attack
              modes:
              - ranged
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              range_ft:
                normal: 150
                long: 600
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 2
                  average: 6
                - type: necrotic
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 0
                  average: 4
            life_drain:
              economy: action
              op: saving_throw
              ability: constitution
              dc: 13
              targets:
                kind: creature
                count: 1
                range_ft: 5
              failure:
              - op: damage
                components:
                - type: necrotic
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 2
                  average: 6
              - op: hp_maximum.reduce
                amount:
                  event_field: damage_taken
              success: []
              on_humanoid_slain:
                op: undead.create
                definition:
                  key: dnd2024:creatures/zombie
                delay:
                  amount: 24
                  unit: hour
                unless_any:
                - restored_to_life
                - body_destroyed
                controller: actor
                control_maximum: 12
            multiattack:
              economy: action
              op: multiattack
              attacks:
                count: 2
                choices:
                - necrotic_sword
                - necrotic_bow
                combination: any
              replace_attack:
                maximum: 1
                choices:
                - op: feature.use
                  feature: life_drain
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_wight
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_lifecycle
      - creature_rules
      - damage
      - effects
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_wight
    citations:
    - SRD 5.2.1 pp. 341; builtin/srd-5.2.1/creature/wight.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/wolf:
    source_sha256: f3bb6e79fd161a24842b46c853f09ef756dc7e5103715a7455a259f4ee68aeb7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/wolf
        implementation: dnd2024:creature_wolf
        parameters:
          skill_totals:
            perception: 5
            stealth: 4
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 15
          languages:
            speak: true
            known: []
          features:
            pack_tactics:
              economy: passive
              op: roll.advantage
              rolls:
              - attack
              target_kind: creature
              when:
                ally_within_target_ft: 5
                ally_excluded_conditions:
                - incapacitated
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 4
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 2
                  average: 5
              - op: condition.apply
                condition: prone
                when:
                  target_kind: creature
                  target_maximum_size: medium
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_wolf
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_wolf
    citations:
    - SRD 5.2.1 pp. 364; builtin/srd-5.2.1/creature/wolf.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/wyvern:
    source_sha256: e5b7c82f0ead7d5fb20ff3dabb22e0f2f0a164a4c10b37ba2dd85f37cfe75c0e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/wyvern
        implementation: dnd2024:creature_wyvern
        parameters:
          skill_totals:
            perception: 4
          senses:
          - sense: darkvision
            range_ft: 120
          passive_perception: 14
          languages:
            speak: true
            known: []
          features:
            bite:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 4
                  average: 13
            sting:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: piercing
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 4
                  average: 11
                - type: poison
                  amount:
                    dice:
                      count: 7
                      sides: 6
                    add: 0
                  average: 24
              - op: condition.apply
                condition: poisoned
                duration:
                  until: actor.next_turn_start
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 1
                feature: bite
              - count: 1
                feature: sting
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_wyvern
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_wyvern
    citations:
    - SRD 5.2.1 pp. 343; builtin/srd-5.2.1/creature/wyvern.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/young_blue_dragon:
    source_sha256: 15eb242af2b82e3c41aead0841e727311a29a5104c5f6ba304f997621b86cd67
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/young_blue_dragon
        implementation: dnd2024:creature_young_blue_dragon
        parameters:
          skill_totals:
            perception: 9
            stealth: 4
          damage_immunities:
          - lightning
          condition_immunities: []
          senses:
          - sense: blindsight
            range_ft: 30
          - sense: darkvision
            range_ft: 120
          passive_perception: 19
          languages:
            speak: true
            known:
            - common
            - draconic
          features:
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 9
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 5
                  average: 12
                - type: lightning
                  amount:
                    dice:
                      count: 1
                      sides: 10
                    add: 0
                  average: 5
            lightning_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 16
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  origin: actor
                  size_ft: 60
                  width_ft: 5
              failure:
              - op: damage
                components:
                - type: lightning
                  amount:
                    dice:
                      count: 10
                      sides: 10
                    add: 0
                  average: 55
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 3
                feature: rend
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_young_blue_dragon
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_young_blue_dragon
    citations:
    - SRD 5.2.1 pp. 266; builtin/srd-5.2.1/creature/young_blue_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/young_copper_dragon:
    source_sha256: 1fbcec12b3b22170710f2ee6760abac479048ef27d4d6ace44296b337cf0caf6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/young_copper_dragon
        implementation: dnd2024:creature_young_copper_dragon
        parameters:
          skill_totals:
            deception: 5
            perception: 7
            stealth: 4
          damage_immunities:
          - acid
          condition_immunities: []
          senses:
          - sense: blindsight
            range_ft: 30
          - sense: darkvision
            range_ft: 120
          passive_perception: 17
          languages:
            speak: true
            known:
            - common
            - draconic
          features:
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 7
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 10
                    add: 4
                  average: 15
            acid_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 14
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  origin: actor
                  size_ft: 40
                  width_ft: 5
              failure:
              - op: damage
                components:
                - type: acid
                  amount:
                    dice:
                      count: 9
                      sides: 8
                    add: 0
                  average: 40
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            slowing_breath:
              economy: action
              op: saving_throw
              ability: constitution
              dc: 14
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 30
              failure:
              - op: effect.apply
                duration:
                  until: target.next_turn_end
                rules:
                  reactions_allowed: false
                  speed_multiplier: 0.5
                  turn_economy_choice:
                  - action
                  - bonus_action
                  both_allowed: false
              success: []
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 3
                feature: rend
              replace_attack:
                maximum: 1
                choices:
                - op: feature.use
                  feature: slowing_breath
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_young_copper_dragon
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_young_copper_dragon
    citations:
    - SRD 5.2.1 pp. 276; builtin/srd-5.2.1/creature/young_copper_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/young_red_dragon:
    source_sha256: daa1069d4c65c5c3f1472efddcb7e9bd20f9f2987fac9be9c3d6535096bb137d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/young_red_dragon
        implementation: dnd2024:creature_young_red_dragon
        parameters:
          skill_totals:
            perception: 8
            stealth: 4
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
          - sense: blindsight
            range_ft: 30
          - sense: darkvision
            range_ft: 120
          passive_perception: 18
          languages:
            speak: true
            known:
            - common
            - draconic
          features:
            rend:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 10
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 10
              hit:
              - op: damage
                components:
                - type: slashing
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 6
                  average: 13
                - type: fire
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  average: 3
            fire_breath:
              economy: action
              op: saving_throw
              ability: dexterity
              dc: 17
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  origin: actor
                  size_ft: 30
              failure:
              - op: damage
                components:
                - type: fire
                  amount:
                    dice:
                      count: 16
                      sides: 6
                    add: 0
                  average: 56
              success:
              - op: damage
                from: failure_damage
                multiplier: 0.5
                round: down
              recharge:
                die_sides: 6
                success_min: 5
                success_max: 6
                check_event: actor.turn_start
                maximum_uses: 1
                initial_uses: 1
                also_recovers_after:
                - short_rest
                - long_rest
            multiattack:
              economy: action
              op: multiattack
              sequence:
              - count: 3
                feature: rend
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_young_red_dragon
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - actor.turn_start
      - creature.feature_requested
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - resources
      - rest_recovery
      - saving_throws
      - spatial_constraints
    requires:
    - dnd2024:creature_young_red_dragon
    citations:
    - SRD 5.2.1 pp. 318; builtin/srd-5.2.1/creature/young_red_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/zombie:
    source_sha256: ac3e190e987404ccd9c1cf28aca4f5518bc525e2c904e2cab1cea99f37fbdd15
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/zombie
        implementation: dnd2024:creature_zombie
        parameters:
          damage_immunities:
          - poison
          condition_immunities:
          - exhaustion
          - poisoned
          senses:
          - sense: darkvision
            range_ft: 60
          passive_perception: 8
          languages:
            speak: false
            known:
            - common
            additional_choice_count: 1
          features:
            undead_fortitude:
              economy: passive
              trigger: damage.reduces_actor_to_zero_hp
              exclude:
                damage_type: radiant
                critical_hit: true
              save:
                ability: constitution
                dc:
                  add:
                  - 5
                  - event_field: damage_taken
                success:
                  hp_instead: 1
            slam:
              economy: action
              op: attack
              modes:
              - melee
              attack_bonus: 3
              target:
                kind: creature_or_object
                count: 1
              reach_ft: 5
              hit:
              - op: damage
                components:
                - type: bludgeoning
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 1
                  average: 5
          damage_resolution:
            choice:
            - printed_average
            - roll_expression
            chooser: controller
            exclusive: true
          spatial_resolution:
            provider: caller
            required_facts:
            - distance
            - area_membership
            - visibility
            - occupancy
            - movement_path
            validate_against_typed_constraints: true
    contract:
      id: dnd2024:creature_zombie
      inputs:
        actor: participant
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        choices:
          type: choice
          required: false
        scene:
          type: resolution
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - roll_requests
      - state_changes
      events:
      - creature.feature_requested
      - damage.reduces_actor_to_zero_hp
      capabilities:
      - action_economy
      - attack_resolution
      - creature_rules
      - damage
      - effects
      - spatial_constraints
    requires:
    - dnd2024:creature_zombie
    citations:
    - SRD 5.2.1 pp. 343, 344; builtin/srd-5.2.1/creature/zombie.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
---

# Creature rule contracts: assignment creatures_3

All 112 assigned creature blocks are specified. The contracts contain 418 named feature records. The second and third sphinx roars are nested in the three-step Roar sequence. All 44 attack entries with added clauses or conditional damage have explicit typed branches. Seventy-nine distinct referenced definitions exist in the bundled definition tree.

Each definition cites its named official SRD source and carries the verified SHA-256 of that source file. These are rule contracts for conversion; they do not implement an engine. Ability scores, supplied save totals, AC, HP, initiative, CR, and base speeds remain the parent converter's source extraction. This file adds skills, defenses, senses, languages, gear, and all named feature rules. Form-specific speed and speech exceptions are explicit.

## Contract conventions

- `actor` is the creature that owns the block. `target` is the current affected creature; nested saves do not change ownership of the feature. `next_turn_start` and `next_turn_end` refer to the named actor or target. A duration without an expiry remains until the normal rules remove the effect. Printed damage averages and dice expressions are alternative resolution choices, as p. 257 specifies.
- Feature names are local symbols. `feature.use` invokes that feature's complete rules within the enclosing action; it does not add a second action cost. Limited uses, form restrictions, and target restrictions still apply. Multiattack replacement permits at most one replacement where the source says one. Spell casts reference the complete spell definition plus the explicit stat-block overrides.
- X/Day uses recover on a Long Rest. Recharge has capacity one, rolls at the owner's turn start, and also recovers after a Short or Long Rest. Legendary actions share their stated budget, occur immediately after another creature's turn, and cannot be used while Incapacitated or otherwise unable to act. These defaults come from SRD p. 257.
- Spellcasting uses the lowest possible spell level unless the block supplies an override. Required materials are available. Casts of 1 minute or longer keep the full casting process, including a Magic action each turn and Concentration; a spellcasting action does not shorten them. These defaults also come from p. 257.
- Spatial facts are supplied by the caller and checked against the stated range, shape, size, visibility, occupancy, and path constraints. Choices authorized to the GM, including the half-dragon's damage type and revival location, remain typed choices with explicit bounds.

## Source interpretations

The werebear and weretiger Shape-Shift paragraphs preserve statistics except size, but their headers give explicit animal-form speed and speech exceptions. The contracts retain those specific exceptions. See the cited creature sources, pp. 337–339.

Warrior Veteran says “two Greatsword or Heavy Crossbow attacks,” without the “in any combination” wording found in other blocks. Its contract preserves the two stated sequences as alternatives. Other blocks that explicitly allow any combination retain that freedom. See its cited creature source.

No 2014 clauses were added. In particular, Shadow's Strength reduction has no restoration clause in this source, Ghost possession uses the ghost's statistics except the named target modifiers and Speed, and the 2024 Mummy death-and-dust clause triggers when its attack reduces a creature to 0 Hit Points. Their contracts follow those printed clauses. Named source typos in printed damage averages are retained as printed alternatives, rather than silently recalculated.

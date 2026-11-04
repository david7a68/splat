---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:creatures/aboleth:
    source_sha256: a56f460e7eabcf7812cc78221c679f07414788c4d13999e9d4f8d58647b9ca2d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/aboleth
        implementation: dnd2024:creature_aboleth
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            eldritch_restoration:
              economy: trait
              operation: passive_rule
              trigger: self.destroyed
              new_body:
                delay:
                  roll:
                    count: 5
                    sides: 10
                    modifier: 0
                  unit: day
                hp: maximum
                location:
                  choice:
                  - far_realm
                  - gm_selected_location
                  chooser: gm
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 3
                recovery: long_rest
                lair_uses: 4
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            mucus_cloud:
              economy: trait
              operation: saving_throw
              ability: constitution
              dc: 14
              targets:
                kind: creature
                selection: each
                area:
                  shape: emanation
                  size_ft: 5
                  origin: self
              failure:
                curse:
                  skin: slimy
                  breathing:
                  - air
                  - water
                  can_regain_hp_if:
                    underwater: true
                  dry_damage:
                    interval_seconds: 600
                    while:
                      outside_body_of_water: true
                    unless:
                      skin_moistened_during_interval: true
                    damage:
                    - roll:
                        count: 1
                        sides: 12
                        modifier: 0
                      type: acid
              trigger: self.turn_end
              requires:
                self_underwater: true
              success:
                effects: []
            probing_telepathy:
              economy: trait
              operation: passive_rule
              trigger: telepathic_message_received
              requires:
                sender_visible: true
              learn:
              - sender.greatest_desires
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: tentacle
                count: 2
              - operation: choice
                options:
                - operation: use_feature
                  feature: consume_memories
                - operation: use_feature
                  feature: dominate_mind
                require_available: true
            tentacle:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 9
                reach_ft: 15
              hit:
                damage:
                - average: 12
                  roll:
                    count: 2
                    sides: 6
                    modifier: 5
                  type: bludgeoning
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 14
                  maximum_size: large
                  appendage_pool:
                    count: 4
                    type: tentacles
                    cost: 1
            consume_memories:
              economy: action
              operation: saving_throw
              ability: intelligence
              dc: 16
              targets:
                kind: creature
                count: 1
                range_ft: 30
                any_of:
                - charmed_by: self
                - grappled_by: self
              failure:
                damage:
                - average: 10
                  roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  type: psychic
              success:
                damage_multiplier: 0.5
                other_effects: false
              after_resolution:
                if:
                  target_type: humanoid
                  target_reduced_to_zero_hp_by_this_action: true
                learn: target.memories
            dominate_mind:
              economy: action
              resource:
                kind: daily
                uses: 2
                recovery: long_rest
              operation: saving_throw
              ability: wisdom
              dc: 16
              targets:
                kind: creature
                count: 1
                range_ft: 30
                visible: true
              failure:
                conditions:
                - condition: charmed
                ends_on:
                - self.dies
                - self_and_target_on_different_planes
                attitude: ally_of_self
                control:
                  controller: self
                  within_ft: 60
                telepathy:
                  with: self
                  range: unlimited
                repeat_save:
                  events:
                  - target.damage_taken
                  - event: elapsed_time
                    seconds: 86400
                    while_distance_at_least_miles: 1
                  success: end_effect
              success:
                effects: []
            lash:
              economy: legendary_action
              legendary_cost: 1
              operation: attack_sequence
              feature: tentacle
              count: 1
            psychic_drain:
              economy: legendary_action
              legendary_cost: 1
              operation: sequence
              requires:
                any_creature:
                  any_of:
                  - charmed_by: self
                  - grappled_by: self
              effects:
              - operation: use_feature
                feature: consume_memories
              - operation: heal
                target: self
                roll:
                  count: 1
                  sides: 10
                  modifier: 0
          skill_totals:
            history: 12
            perception: 10
          senses:
            darkvision: 120
            passive_perception: 20
          languages:
            speaks:
            - deep_speech
            understands: []
            telepathy_range_ft: 120
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            lair_uses: 4
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_aboleth
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - damage.resolved
      - creature.died
      - creature.destroyed
      - rest.completed
      - saving_throw.resolved
      - time.elapsed
      - telepathy.received
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - curses
      - legendary_actions
      - hit_points
    requires:
    - dnd2024:creature_aboleth
    citations:
    - SRD 5.2.1 pp. 258; builtin/srd-5.2.1/creature/aboleth.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/adult_brass_dragon:
    source_sha256: 756ad4ddda8bddaca91aec9e4d4727164e60bc29f4487f95fe6d34b5b3125f1d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/adult_brass_dragon
        implementation: dnd2024:creature_adult_brass_dragon
        parameters:
          features:
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 3
                recovery: long_rest
                lair_uses: 4
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: choice
                  options:
                  - operation: use_feature
                    feature: sleep_breath
                  - operation: cast_spell
                    spell:
                      key: dnd2024:spells/scorching_ray
                    cast_level: 2
                    spellcasting_profile: spellcasting
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 11
                reach_ft: 10
              hit:
                damage:
                - average: 17
                  roll:
                    count: 2
                    sides: 10
                    modifier: 6
                  type: slashing
                - average: 4
                  roll:
                    count: 1
                    sides: 8
                    modifier: 0
                  type: fire
            fire_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 18
              targets:
                kind: creature
                selection: each
                area:
                  shape: line
                  size_ft: 60
                  origin: self
                  width_ft: 5
              failure:
                damage:
                - average: 45
                  roll:
                    count: 10
                    sides: 8
                    modifier: 0
                  type: fire
              success:
                damage_multiplier: 0.5
                other_effects: false
            sleep_breath:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 18
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 60
                  origin: self
              failure:
                conditions:
                - condition: incapacitated
                  until: target.next_turn_end
                repeat_save:
                  event: target.next_turn_end
                  failure:
                    conditions:
                    - condition: unconscious
                      duration_seconds: 600
                    ends_on:
                    - event: target.damage_taken
                    - event: creature.action
                      operation: wake_target
                      within_ft: 5
                  success: end_effect
              success:
                effects: []
            spellcasting:
              economy: action
              operation: spellcasting
              ability: charisma
              components_waived:
              - material
              save_dc: 16
              options:
              - spell:
                  key: dnd2024:spells/detect_magic
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/minor_illusion
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/scorching_ray
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/shapechange
                usage:
                  kind: at_will
                overrides:
                  allowed_types:
                  - beast
                  - humanoid
                  gain_temporary_hp: false
                  maintain_requires_concentration: false
                  maintain_requires_temporary_hp: false
              - spell:
                  key: dnd2024:spells/speak_with_animals
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/detect_thoughts
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/control_weather
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            blazing_light:
              economy: legendary_action
              legendary_cost: 1
              operation: cast_spell
              spell:
                key: dnd2024:spells/scorching_ray
              cast_level: 2
              spellcasting_profile: spellcasting
            pounce:
              economy: legendary_action
              legendary_cost: 1
              operation: sequence
              effects:
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: speed
                  - 0.5
              - operation: attack_sequence
                feature: rend
                count: 1
            scorching_sands:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: saving_throw
              ability: dexterity
              dc: 16
              targets:
                kind: creature
                count: 1
                range_ft: 120
                visible: true
              failure:
                damage:
                - average: 27
                  roll:
                    count: 6
                    sides: 8
                    modifier: 0
                  type: fire
                speed_multiplier: 0.5
                until: target.next_turn_end
              success:
                effects: []
          skill_totals:
            history: 7
            perception: 11
            persuasion: 8
            stealth: 5
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
            blindsight: 60
            darkvision: 120
            passive_perception: 21
          languages:
            speaks:
            - common
            - draconic
            understands: []
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            lair_uses: 4
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_adult_brass_dragon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - damage.resolved
      - rest.completed
      - saving_throw.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
      - legendary_actions
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_adult_brass_dragon
    citations:
    - SRD 5.2.1 pp. 268, 269; builtin/srd-5.2.1/creature/adult_brass_dragon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/adult_gold_dragon:
    source_sha256: 499b65073d0bcd1cdec296399b0581c06e834307edcf77dc2a4782a36af31db6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/adult_gold_dragon
        implementation: dnd2024:creature_adult_gold_dragon
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 3
                recovery: long_rest
                lair_uses: 4
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: choice
                  options:
                  - operation: use_feature
                    feature: weakening_breath
                  - operation: cast_spell
                    spell:
                      key: dnd2024:spells/guiding_bolt
                    cast_level: 2
                    spellcasting_profile: spellcasting
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 14
                reach_ft: 10
              hit:
                damage:
                - average: 17
                  roll:
                    count: 2
                    sides: 8
                    modifier: 8
                  type: slashing
                - average: 4
                  roll:
                    count: 1
                    sides: 8
                    modifier: 0
                  type: fire
            fire_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 21
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 60
                  origin: self
              failure:
                damage:
                - average: 66
                  roll:
                    count: 12
                    sides: 10
                    modifier: 0
                  type: fire
              success:
                damage_multiplier: 0.5
                other_effects: false
            spellcasting:
              economy: action
              operation: spellcasting
              ability: charisma
              components_waived:
              - material
              save_dc: 21
              supplied_spell_attack_total: 13
              options:
              - spell:
                  key: dnd2024:spells/detect_magic
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/guiding_bolt
                usage:
                  kind: at_will
                cast_level: 2
              - spell:
                  key: dnd2024:spells/shapechange
                usage:
                  kind: at_will
                overrides:
                  allowed_types:
                  - beast
                  - humanoid
                  gain_temporary_hp: false
                  maintain_requires_concentration: false
                  maintain_requires_temporary_hp: false
              - spell:
                  key: dnd2024:spells/flame_strike
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/zone_of_truth
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            weakening_breath:
              economy: action
              operation: saving_throw
              ability: strength
              dc: 21
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 60
                  origin: self
                exclude_already_affected_by: this_breath
              failure:
                disadvantage_on:
                  roll: d20_test
                  ability: strength
                damage_roll_modifier:
                  subtract:
                    count: 1
                    sides: 6
                    modifier: 0
                repeat_save:
                  event: target.turn_end
                  success: end_effect
                automatic_success_after_seconds: 60
              success:
                effects: []
            banish:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: saving_throw
              ability: charisma
              dc: 21
              targets:
                kind: creature
                count: 1
                range_ft: 120
                visible: true
              failure:
                damage:
                - average: 10
                  roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  type: force
                conditions:
                - condition: incapacitated
                  until: self.next_turn_start
                transport:
                  plane: harmless_demiplane
                  until: self.next_turn_start
                  return:
                    unoccupied: true
                    within_self_ft: 120
                    chooser: self
              success:
                effects: []
            guiding_light:
              economy: legendary_action
              legendary_cost: 1
              operation: cast_spell
              spell:
                key: dnd2024:spells/guiding_bolt
              cast_level: 2
              spellcasting_profile: spellcasting
            pounce:
              economy: legendary_action
              legendary_cost: 1
              operation: sequence
              effects:
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: speed
                  - 0.5
              - operation: attack_sequence
                feature: rend
                count: 1
          skill_totals:
            insight: 8
            perception: 14
            persuasion: 13
            stealth: 8
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
            blindsight: 60
            darkvision: 120
            passive_perception: 24
          languages:
            speaks:
            - common
            - draconic
            understands: []
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            lair_uses: 4
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_adult_gold_dragon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - rest.completed
      - saving_throw.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
      - legendary_actions
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_adult_gold_dragon
    citations:
    - SRD 5.2.1 pp. 291, 292; builtin/srd-5.2.1/creature/adult_gold_dragon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/adult_silver_dragon:
    source_sha256: 6b67993844dc4def26415d44704e549dc407df6ad81da2c54b70797364c817f4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/adult_silver_dragon
        implementation: dnd2024:creature_adult_silver_dragon
        parameters:
          features:
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 3
                recovery: long_rest
                lair_uses: 4
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: choice
                  options:
                  - operation: use_feature
                    feature: paralyzing_breath
                  - operation: cast_spell
                    spell:
                      key: dnd2024:spells/ice_knife
                    cast_level: 1
                    spellcasting_profile: spellcasting
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 13
                reach_ft: 10
              hit:
                damage:
                - average: 17
                  roll:
                    count: 2
                    sides: 8
                    modifier: 8
                  type: slashing
                - average: 4
                  roll:
                    count: 1
                    sides: 8
                    modifier: 0
                  type: cold
            cold_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: constitution
              dc: 20
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 60
                  origin: self
              failure:
                damage:
                - average: 54
                  roll:
                    count: 12
                    sides: 8
                    modifier: 0
                  type: cold
              success:
                damage_multiplier: 0.5
                other_effects: false
            paralyzing_breath:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 20
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 60
                  origin: self
              failure:
                damage: []
                conditions:
                - condition: incapacitated
                  until: target.next_turn_end
                repeat_save:
                  event: target.next_turn_end
                  failure:
                    conditions:
                    - condition: paralyzed
                    repeat_save:
                      event: target.turn_end
                      success: end_effect
                    automatic_success_after_seconds: 60
                    automatic_success_clock_origin: initial_failed_save
                  success: end_effect
              success:
                effects: []
            spellcasting:
              economy: action
              operation: spellcasting
              ability: charisma
              components_waived:
              - material
              save_dc: 19
              supplied_spell_attack_total: 11
              options:
              - spell:
                  key: dnd2024:spells/detect_magic
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/hold_monster
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/ice_knife
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/shapechange
                usage:
                  kind: at_will
                overrides:
                  allowed_types:
                  - beast
                  - humanoid
                  gain_temporary_hp: false
                  maintain_requires_concentration: false
                  maintain_requires_temporary_hp: false
              - spell:
                  key: dnd2024:spells/ice_storm
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
                cast_level: 5
              - spell:
                  key: dnd2024:spells/zone_of_truth
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            chill:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: cast_spell
              spell:
                key: dnd2024:spells/hold_monster
              spellcasting_profile: spellcasting
            cold_gale:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: saving_throw
              ability: dexterity
              dc: 19
              targets:
                kind: creature
                selection: each
                area:
                  shape: line
                  size_ft: 60
                  origin: self
                  width_ft: 10
              failure:
                damage:
                - average: 14
                  roll:
                    count: 4
                    sides: 6
                    modifier: 0
                  type: cold
                push:
                  maximum_ft: 30
                  direction: straight_away_from_self
              success:
                damage_multiplier: 0.5
                other_effects: false
            pounce:
              economy: legendary_action
              legendary_cost: 1
              operation: sequence
              effects:
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: speed
                  - 0.5
              - operation: attack_sequence
                feature: rend
                count: 1
          skill_totals:
            history: 8
            perception: 11
            stealth: 5
          damage_immunities:
          - cold
          condition_immunities: []
          senses:
            blindsight: 60
            darkvision: 120
            passive_perception: 21
          languages:
            speaks:
            - common
            - draconic
            understands: []
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            lair_uses: 4
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_adult_silver_dragon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - rest.completed
      - saving_throw.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
      - legendary_actions
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_adult_silver_dragon
    citations:
    - SRD 5.2.1 pp. 324, 325; builtin/srd-5.2.1/creature/adult_silver_dragon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/allosaurus:
    source_sha256: fb818cd3b71a431ec96606a59f5fcfb5e1f5daea2e5f692454fd990291114c5d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/allosaurus
        implementation: dnd2024:creature_allosaurus
        parameters:
          features:
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 15
                  roll:
                    count: 2
                    sides: 10
                    modifier: 4
                  type: piercing
            claws:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 8
                  roll:
                    count: 1
                    sides: 8
                    modifier: 4
                  type: slashing
                prone:
                  condition: prone
                  maximum_size: large
                  requires:
                    immediately_preceding_straight_movement_toward_target_ft: 30
                additional_attack:
                  feature: bite
                  target: same
                  requires:
                    maximum_target_size: large
                    immediately_preceding_straight_movement_toward_target_ft: 30
                  optional: true
          skill_totals:
            perception: 5
          senses:
            passive_perception: 15
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_allosaurus
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_allosaurus
    citations:
    - SRD 5.2.1 pp. 344; builtin/srd-5.2.1/creature/allosaurus.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/ancient_brass_dragon:
    source_sha256: bec8c3e4ae9aee8daa88b2198ead56d2841334e0d0fbef072e02bed638c33409
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ancient_brass_dragon
        implementation: dnd2024:creature_ancient_brass_dragon
        parameters:
          features:
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 4
                recovery: long_rest
                lair_uses: 5
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: choice
                  options:
                  - operation: use_feature
                    feature: sleep_breath
                  - operation: cast_spell
                    spell:
                      key: dnd2024:spells/scorching_ray
                    cast_level: 3
                    spellcasting_profile: spellcasting
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 14
                reach_ft: 15
              hit:
                damage:
                - average: 19
                  roll:
                    count: 2
                    sides: 10
                    modifier: 8
                  type: slashing
                - average: 7
                  roll:
                    count: 2
                    sides: 6
                    modifier: 0
                  type: fire
            fire_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 21
              targets:
                kind: creature
                selection: each
                area:
                  shape: line
                  size_ft: 90
                  origin: self
                  width_ft: 5
              failure:
                damage:
                - average: 58
                  roll:
                    count: 13
                    sides: 8
                    modifier: 0
                  type: fire
              success:
                damage_multiplier: 0.5
                other_effects: false
            sleep_breath:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 21
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 90
                  origin: self
              failure:
                conditions:
                - condition: incapacitated
                  until: target.next_turn_end
                repeat_save:
                  event: target.next_turn_end
                  failure:
                    conditions:
                    - condition: unconscious
                      duration_seconds: 600
                    ends_on:
                    - event: target.damage_taken
                    - event: creature.action
                      operation: wake_target
                      within_ft: 5
                  success: end_effect
              success:
                effects: []
            spellcasting:
              economy: action
              operation: spellcasting
              ability: charisma
              components_waived:
              - material
              save_dc: 20
              options:
              - spell:
                  key: dnd2024:spells/detect_magic
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/minor_illusion
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/scorching_ray
                usage:
                  kind: at_will
                cast_level: 3
              - spell:
                  key: dnd2024:spells/shapechange
                usage:
                  kind: at_will
                overrides:
                  allowed_types:
                  - beast
                  - humanoid
                  gain_temporary_hp: false
                  maintain_requires_concentration: false
                  maintain_requires_temporary_hp: false
              - spell:
                  key: dnd2024:spells/speak_with_animals
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/control_weather
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/detect_thoughts
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            blazing_light:
              economy: legendary_action
              legendary_cost: 1
              operation: cast_spell
              spell:
                key: dnd2024:spells/scorching_ray
              cast_level: 3
              spellcasting_profile: spellcasting
            pounce:
              economy: legendary_action
              legendary_cost: 1
              operation: sequence
              effects:
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: speed
                  - 0.5
              - operation: attack_sequence
                feature: rend
                count: 1
            scorching_sands:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: saving_throw
              ability: dexterity
              dc: 20
              targets:
                kind: creature
                count: 1
                range_ft: 120
                visible: true
              failure:
                damage:
                - average: 36
                  roll:
                    count: 8
                    sides: 8
                    modifier: 0
                  type: fire
                speed_multiplier: 0.5
                until: target.next_turn_end
              success:
                effects: []
          skill_totals:
            history: 9
            perception: 14
            persuasion: 12
            stealth: 6
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
            blindsight: 60
            darkvision: 120
            passive_perception: 24
          languages:
            speaks:
            - common
            - draconic
            understands: []
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            lair_uses: 4
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_ancient_brass_dragon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - damage.resolved
      - rest.completed
      - saving_throw.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
      - legendary_actions
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_ancient_brass_dragon
    citations:
    - SRD 5.2.1 pp. 269; builtin/srd-5.2.1/creature/ancient_brass_dragon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/ancient_gold_dragon:
    source_sha256: 0a6d282ab34836a7c9b9ee7ae17d4b813f50ae1735f1e9e854a364760682d9ee
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ancient_gold_dragon
        implementation: dnd2024:creature_ancient_gold_dragon
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 4
                recovery: long_rest
                lair_uses: 5
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: choice
                  options:
                  - operation: use_feature
                    feature: weakening_breath
                  - operation: cast_spell
                    spell:
                      key: dnd2024:spells/guiding_bolt
                    cast_level: 4
                    spellcasting_profile: spellcasting
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 17
                reach_ft: 15
              hit:
                damage:
                - average: 19
                  roll:
                    count: 2
                    sides: 8
                    modifier: 10
                  type: slashing
                - average: 9
                  roll:
                    count: 2
                    sides: 8
                    modifier: 0
                  type: fire
            fire_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 24
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 90
                  origin: self
              failure:
                damage:
                - average: 71
                  roll:
                    count: 13
                    sides: 10
                    modifier: 0
                  type: fire
              success:
                damage_multiplier: 0.5
                other_effects: false
            spellcasting:
              economy: action
              operation: spellcasting
              ability: charisma
              components_waived:
              - material
              save_dc: 24
              supplied_spell_attack_total: 16
              options:
              - spell:
                  key: dnd2024:spells/detect_magic
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/guiding_bolt
                usage:
                  kind: at_will
                cast_level: 4
              - spell:
                  key: dnd2024:spells/shapechange
                usage:
                  kind: at_will
                overrides:
                  allowed_types:
                  - beast
                  - humanoid
                  gain_temporary_hp: false
                  maintain_requires_concentration: false
                  maintain_requires_temporary_hp: false
              - spell:
                  key: dnd2024:spells/flame_strike
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
                cast_level: 6
              - spell:
                  key: dnd2024:spells/word_of_recall
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/zone_of_truth
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            weakening_breath:
              economy: action
              operation: saving_throw
              ability: strength
              dc: 24
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 90
                  origin: self
                exclude_already_affected_by: this_breath
              failure:
                disadvantage_on:
                  roll: d20_test
                  ability: strength
                damage_roll_modifier:
                  subtract:
                    count: 1
                    sides: 10
                    modifier: 0
                repeat_save:
                  event: target.turn_end
                  success: end_effect
                automatic_success_after_seconds: 60
              success:
                effects: []
            banish:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: saving_throw
              ability: charisma
              dc: 24
              targets:
                kind: creature
                count: 1
                range_ft: 120
                visible: true
              failure:
                damage:
                - average: 24
                  roll:
                    count: 7
                    sides: 6
                    modifier: 0
                  type: force
                conditions:
                - condition: incapacitated
                  until: self.next_turn_start
                transport:
                  plane: harmless_demiplane
                  until: self.next_turn_start
                  return:
                    unoccupied: true
                    within_self_ft: 120
                    chooser: self
              success:
                effects: []
            guiding_light:
              economy: legendary_action
              legendary_cost: 1
              operation: cast_spell
              spell:
                key: dnd2024:spells/guiding_bolt
              cast_level: 4
              spellcasting_profile: spellcasting
            pounce:
              economy: legendary_action
              legendary_cost: 1
              operation: sequence
              effects:
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: speed
                  - 0.5
              - operation: attack_sequence
                feature: rend
                count: 1
          skill_totals:
            insight: 10
            perception: 17
            persuasion: 16
            stealth: 9
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
            blindsight: 60
            darkvision: 120
            passive_perception: 27
          languages:
            speaks:
            - common
            - draconic
            understands: []
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            lair_uses: 4
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_ancient_gold_dragon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - rest.completed
      - saving_throw.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
      - legendary_actions
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_ancient_gold_dragon
    citations:
    - SRD 5.2.1 pp. 292; builtin/srd-5.2.1/creature/ancient_gold_dragon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/ancient_silver_dragon:
    source_sha256: fa99b207ac0e663d05fc1b93d970d521d5c632496e20dd2bb391e9f14fe7d7f3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ancient_silver_dragon
        implementation: dnd2024:creature_ancient_silver_dragon
        parameters:
          features:
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 4
                recovery: long_rest
                lair_uses: 5
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: choice
                  options:
                  - operation: use_feature
                    feature: paralyzing_breath
                  - operation: cast_spell
                    spell:
                      key: dnd2024:spells/ice_knife
                    cast_level: 2
                    spellcasting_profile: spellcasting
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 17
                reach_ft: 15
              hit:
                damage:
                - average: 19
                  roll:
                    count: 2
                    sides: 8
                    modifier: 10
                  type: slashing
                - average: 9
                  roll:
                    count: 2
                    sides: 8
                    modifier: 0
                  type: cold
            cold_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: constitution
              dc: 24
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 90
                  origin: self
              failure:
                damage:
                - average: 67
                  roll:
                    count: 15
                    sides: 8
                    modifier: 0
                  type: cold
              success:
                damage_multiplier: 0.5
                other_effects: false
            paralyzing_breath:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 24
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 90
                  origin: self
              failure:
                damage: []
                conditions:
                - condition: incapacitated
                  until: target.next_turn_end
                repeat_save:
                  event: target.next_turn_end
                  failure:
                    conditions:
                    - condition: paralyzed
                    repeat_save:
                      event: target.turn_end
                      success: end_effect
                    automatic_success_after_seconds: 60
                    automatic_success_clock_origin: initial_failed_save
                  success: end_effect
              success:
                effects: []
            spellcasting:
              economy: action
              operation: spellcasting
              ability: charisma
              components_waived:
              - material
              save_dc: 23
              supplied_spell_attack_total: 15
              options:
              - spell:
                  key: dnd2024:spells/detect_magic
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/hold_monster
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/ice_knife
                usage:
                  kind: at_will
                cast_level: 2
              - spell:
                  key: dnd2024:spells/shapechange
                usage:
                  kind: at_will
                overrides:
                  allowed_types:
                  - beast
                  - humanoid
                  gain_temporary_hp: false
                  maintain_requires_concentration: false
                  maintain_requires_temporary_hp: false
              - spell:
                  key: dnd2024:spells/control_weather
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/ice_storm
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
                cast_level: 7
              - spell:
                  key: dnd2024:spells/teleport
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/zone_of_truth
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            chill:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: cast_spell
              spell:
                key: dnd2024:spells/hold_monster
              spellcasting_profile: spellcasting
            cold_gale:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: saving_throw
              ability: dexterity
              dc: 23
              targets:
                kind: creature
                selection: each
                area:
                  shape: line
                  size_ft: 60
                  origin: self
                  width_ft: 10
              failure:
                damage:
                - average: 14
                  roll:
                    count: 4
                    sides: 6
                    modifier: 0
                  type: cold
                push:
                  maximum_ft: 30
                  direction: straight_away_from_self
              success:
                damage_multiplier: 0.5
                other_effects: false
            pounce:
              economy: legendary_action
              legendary_cost: 1
              operation: sequence
              effects:
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: speed
                  - 0.5
              - operation: attack_sequence
                feature: rend
                count: 1
          skill_totals:
            history: 11
            perception: 16
            stealth: 7
          damage_immunities:
          - cold
          condition_immunities: []
          senses:
            blindsight: 60
            darkvision: 120
            passive_perception: 26
          languages:
            speaks:
            - common
            - draconic
            understands: []
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            lair_uses: 4
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_ancient_silver_dragon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - rest.completed
      - saving_throw.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - teleportation
      - spatial_queries
      - limited_resources
      - legendary_actions
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_ancient_silver_dragon
    citations:
    - SRD 5.2.1 pp. 325; builtin/srd-5.2.1/creature/ancient_silver_dragon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/animated_flying_sword:
    source_sha256: b547e6c583737c81ca2bbb89f337fd925cf648eb53f1ae5f4fe1302d076bf63b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/animated_flying_sword
        implementation: dnd2024:creature_animated_flying_sword
        parameters:
          features:
            slash:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 1
                    sides: 8
                    modifier: 2
                  type: slashing
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
            blindsight: 60
            passive_perception: 7
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_animated_flying_sword
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_animated_flying_sword
    citations:
    - SRD 5.2.1 pp. 259; builtin/srd-5.2.1/creature/animated_flying_sword.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/ankheg:
    source_sha256: b42d9c3d26ca37841bb2ce807de76a3658bc2832fa589b6edd7047e98f00e33d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ankheg
        implementation: dnd2024:creature_ankheg
        parameters:
          features:
            tunneler:
              economy: trait
              operation: passive_rule
              burrow_material: solid_rock
              speed_multiplier: 0.5
              tunnel_diameter_ft: 10
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
                advantage_if:
                  target_grappled_by: self
              hit:
                damage:
                - average: 10
                  roll:
                    count: 2
                    sides: 6
                    modifier: 3
                  type: slashing
                - average: 3
                  roll:
                    count: 1
                    sides: 6
                    modifier: 0
                  type: acid
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 13
                  maximum_size: large
            acid_spray:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 12
              targets:
                kind: creature
                selection: each
                area:
                  shape: line
                  size_ft: 30
                  origin: self
                  width_ft: 5
              failure:
                damage:
                - average: 14
                  roll:
                    count: 4
                    sides: 6
                    modifier: 0
                  type: acid
              success:
                damage_multiplier: 0.5
                other_effects: false
          senses:
            darkvision: 60
            tremorsense: 60
            passive_perception: 11
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_ankheg
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_ankheg
    citations:
    - SRD 5.2.1 pp. 259; builtin/srd-5.2.1/creature/ankheg.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/archelon:
    source_sha256: 1aae6f42a8829ae3b9e9b7504237fd9cd6c15b5822ee5f4c6124c0bf302054bb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/archelon
        implementation: dnd2024:creature_archelon
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: bite
                count: 2
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 14
                  roll:
                    count: 3
                    sides: 6
                    modifier: 4
                  type: piercing
          skill_totals:
            stealth: 5
          senses:
            passive_perception: 12
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_archelon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_archelon
    citations:
    - SRD 5.2.1 pp. 344, 345; builtin/srd-5.2.1/creature/archelon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/avatar_of_death:
    source_sha256: a791cea835f57490180fa68edfff3765f022979d76f0f9a50cd1c956a3077c8c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/avatar_of_death
        implementation: dnd2024:creature_avatar_of_death
        parameters:
          features:
            incorporeal_movement:
              economy: trait
              operation: passive_rule
              pass_through:
              - creatures
              - objects
              movement_terrain: difficult
              end_turn_inside_object_damage:
                roll:
                  count: 1
                  sides: 10
                  modifier: 0
                type: force
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: reaping_scythe
                count:
                  ceil:
                    divide:
                    - stat: summoner.proficiency_bonus
                    - 2
            reaping_scythe:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                automatic_hit: true
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 8
                    modifier: 3
                  type: slashing
                - average: 4
                  roll:
                    count: 1
                    sides: 8
                    modifier: 0
                  type: necrotic
          damage_immunities:
          - necrotic
          - poison
          condition_immunities:
          - charmed
          - exhaustion
          - frightened
          - paralyzed
          - petrified
          - poisoned
          - unconscious
          senses:
            truesight: 60
            passive_perception: 13
          languages:
            speaks:
              copy: summoner.languages
            understands: []
          derived_statistics:
            hp:
              divide:
              - stat: summoner.maximum_hp
              - 2
            proficiency_bonus:
              stat: summoner.proficiency_bonus
            languages:
              stat: summoner.languages
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_avatar_of_death
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_avatar_of_death
    citations:
    - SRD 5.2.1 pp. 232; builtin/srd-5.2.1/creature/avatar_of_death.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/axe_beak:
    source_sha256: b287d9023facf68d4fce73876feebcd3bf663d9b80bb33801249ddd2eb43c5ff
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/axe_beak
        implementation: dnd2024:creature_axe_beak
        parameters:
          features:
            beak:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 1
                    sides: 8
                    modifier: 2
                  type: slashing
          senses:
            passive_perception: 10
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_axe_beak
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_axe_beak
    citations:
    - SRD 5.2.1 pp. 260; builtin/srd-5.2.1/creature/axe_beak.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/badger:
    source_sha256: ca80d9bf5e875fa6e9d6ce5d443898e74c7138b2690618cda92930c1d8088a71
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/badger
        implementation: dnd2024:creature_badger
        parameters:
          features:
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 2
                reach_ft: 5
              hit:
                damage:
                - average: 1
                  roll:
                    constant: 1
                  type: piercing
          skill_totals:
            perception: 3
          damage_resistances:
          - poison
          senses:
            darkvision: 30
            passive_perception: 13
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_badger
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_badger
    citations:
    - SRD 5.2.1 pp. 345; builtin/srd-5.2.1/creature/badger.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/bandit_captain:
    source_sha256: ffb275060da0ecf9753d69b7b3ca7173b39bd7770b029a1828d97b03bd6a9288
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/bandit_captain
        implementation: dnd2024:creature_bandit_captain
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - scimitar
                - pistol
            scimitar:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 1
                    sides: 6
                    modifier: 3
                  type: slashing
            pistol:
              economy: action
              operation: attack
              attack:
                mode: ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                range_ft: 30
                long_range_ft: 90
              hit:
                damage:
                - average: 8
                  roll:
                    count: 1
                    sides: 10
                    modifier: 3
                  type: piercing
            parry:
              economy: reaction
              operation: modify_ac
              trigger: self.hit_by_melee_attack
              requires:
                holding_weapon: true
              bonus: 2
              scope: triggering_attack
              reevaluate_hit: true
          skill_totals:
            athletics: 4
            deception: 4
          gear:
          - item: pistol
            quantity: 1
          - item: scimitar
            quantity: 1
          - item: studded_leather_armor
            quantity: 1
          senses:
            passive_perception: 10
          languages:
            speaks:
            - common
            - thieves_cant
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_bandit_captain
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_bandit_captain
    citations:
    - SRD 5.2.1 pp. 261, 262; builtin/srd-5.2.1/creature/bandit_captain.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/bat:
    source_sha256: 16bd4b4564a792df6f09e5e9477ad21c2dc29646242684573e65b3d3fa3d1c4a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/bat
        implementation: dnd2024:creature_bat
        parameters:
          features:
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 1
                  roll:
                    constant: 1
                  type: piercing
          senses:
            blindsight: 60
            passive_perception: 11
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_bat
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_bat
    citations:
    - SRD 5.2.1 pp. 345; builtin/srd-5.2.1/creature/bat.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/berserker:
    source_sha256: 7b2a1b0476e489a919bd30334e50faaecfa03883187ceaaaecd70fe01f6a516c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/berserker
        implementation: dnd2024:creature_berserker
        parameters:
          features:
            bloodied_frenzy:
              economy: trait
              operation: passive_rule
              while:
                self_bloodied: true
              advantage_on:
              - attack_rolls
              - saving_throws
            greataxe:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 9
                  roll:
                    count: 1
                    sides: 12
                    modifier: 3
                  type: slashing
          gear:
          - item: greataxe
            quantity: 1
          - item: hide_armor
            quantity: 1
          senses:
            passive_perception: 10
          languages:
            speaks:
            - common
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_berserker
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - attack.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_berserker
    citations:
    - SRD 5.2.1 pp. 263; builtin/srd-5.2.1/creature/berserker.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/black_pudding:
    source_sha256: 0c26f65e1bb16b388da9872c9d1f32569de51945eeb54610b0db9b2625badd97
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/black_pudding
        implementation: dnd2024:creature_black_pudding
        parameters:
          features:
            amorphous:
              economy: trait
              operation: passive_rule
              minimum_gap_inches: 1
              extra_movement_cost: 0
            corrosive_form:
              economy: trait
              operation: passive_rule
              on_melee_attack_hit:
                attacker_damage:
                - roll:
                    count: 1
                    sides: 8
                    modifier: 0
                  type: acid
              on_ammunition_damage:
                requires_nonmagical: true
                destroy_ammunition: true
              on_weapon_damage_contact:
                requires_nonmagical: true
                cumulative_attack_penalty: -1
                destroy_at_penalty: -5
                remove_penalty_spell:
                  key: dnd2024:spells/mending
              consume_material:
                materials:
                - nonmagical_wood
                - nonmagical_metal
                depth_ft: 2
                seconds: 60
            spider_climb:
              economy: trait
              operation: passive_rule
              climb_difficult_surfaces: true
              climb_ceilings: true
              ability_check_required: false
            dissolving_pseudopod:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 10
              hit:
                damage:
                - average: 17
                  roll:
                    count: 4
                    sides: 6
                    modifier: 3
                  type: acid
                armor_corrosion:
                  worn: true
                  nonmagical: true
                  cumulative_ac_penalty: -1
                  destroy_at_ac: 10
                  remove_penalty_spell:
                    key: dnd2024:spells/mending
            split:
              economy: reaction
              operation: split_creature
              trigger:
                any_of:
                - self.becomes_bloodied
                - self.subjected_to_lightning_damage
                - self.subjected_to_slashing_damage
              requires:
                size:
                - medium
                - large
                hp_minimum: 10
              remove_original: true
              offspring:
                definition:
                  key: dnd2024:creatures/black_pudding
                count: 2
                size_steps: -1
                hp_each:
                  floor:
                    divide:
                    - stat: self.current_hp
                    - 2
                initiative: original
          damage_immunities:
          - acid
          - cold
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
            blindsight: 60
            passive_perception: 8
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_black_pudding
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_black_pudding
    citations:
    - SRD 5.2.1 pp. 265, 266; builtin/srd-5.2.1/creature/black_pudding.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/blue_dragon_wyrmling:
    source_sha256: 94329069a321e802072de92f7c95ee1eff7cfc1075c1abdf79a7ece97ee27902
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/blue_dragon_wyrmling
        implementation: dnd2024:creature_blue_dragon_wyrmling
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 2
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 8
                  roll:
                    count: 1
                    sides: 10
                    modifier: 3
                  type: slashing
                - average: 3
                  roll:
                    count: 1
                    sides: 6
                    modifier: 0
                  type: lightning
            lightning_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 12
              targets:
                kind: creature
                selection: each
                area:
                  shape: line
                  size_ft: 30
                  origin: self
                  width_ft: 5
              failure:
                damage:
                - average: 21
                  roll:
                    count: 6
                    sides: 6
                    modifier: 0
                  type: lightning
              success:
                damage_multiplier: 0.5
                other_effects: false
          skill_totals:
            perception: 4
            stealth: 2
          damage_immunities:
          - lightning
          condition_immunities: []
          senses:
            blindsight: 10
            darkvision: 60
            passive_perception: 14
          languages:
            speaks:
            - draconic
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_blue_dragon_wyrmling
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_blue_dragon_wyrmling
    citations:
    - SRD 5.2.1 pp. 266; builtin/srd-5.2.1/creature/blue_dragon_wyrmling.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/brass_dragon_wyrmling:
    source_sha256: 5823efd3c64f7c4584f8b079e8954ba64b360ba383af9a0ee1c4972159d16816
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/brass_dragon_wyrmling
        implementation: dnd2024:creature_brass_dragon_wyrmling
        parameters:
          features:
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 10
                    modifier: 2
                  type: slashing
            fire_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 11
              targets:
                kind: creature
                selection: each
                area:
                  shape: line
                  size_ft: 20
                  origin: self
                  width_ft: 5
              failure:
                damage:
                - average: 14
                  roll:
                    count: 4
                    sides: 6
                    modifier: 0
                  type: fire
              success:
                damage_multiplier: 0.5
                other_effects: false
            sleep_breath:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 11
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 15
                  origin: self
              failure:
                conditions:
                - condition: incapacitated
                  until: target.next_turn_end
                repeat_save:
                  event: target.next_turn_end
                  failure:
                    conditions:
                    - condition: unconscious
                      duration_seconds: 60
                    ends_on:
                    - event: target.damage_taken
                    - event: creature.action
                      operation: wake_target
                      within_ft: 5
                  success: end_effect
              success:
                effects: []
          skill_totals:
            perception: 4
            stealth: 2
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
            blindsight: 10
            darkvision: 60
            passive_perception: 14
          languages:
            speaks:
            - draconic
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_brass_dragon_wyrmling
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - damage.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_brass_dragon_wyrmling
    citations:
    - SRD 5.2.1 pp. 268; builtin/srd-5.2.1/creature/brass_dragon_wyrmling.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/bugbear_stalker:
    source_sha256: 559b44f530bb3f1de3267ff63b1008942109009f50adbc904bc8cf8fd20bfff0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/bugbear_stalker
        implementation: dnd2024:creature_bugbear_stalker
        parameters:
          features:
            abduct:
              economy: trait
              operation: passive_rule
              grappled_creature_movement_extra_cost: 0
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - javelin
                - morningstar
            javelin:
              economy: action
              operation: attack
              attack:
                mode: melee_or_ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 10
                range_ft: 30
                long_range_ft: 120
              hit:
                damage:
                - average: 13
                  roll:
                    count: 3
                    sides: 6
                    modifier: 3
                  type: piercing
            morningstar:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 10
                advantage_if:
                  target_grappled_by: self
              hit:
                damage:
                - average: 12
                  roll:
                    count: 2
                    sides: 8
                    modifier: 3
                  type: piercing
            quick_grapple:
              economy: bonus_action
              operation: saving_throw
              ability: dexterity
              dc: 13
              targets:
                kind: creature
                count: 1
                range_ft: 10
                visible: true
                maximum_size: medium
              failure:
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 13
              success:
                effects: []
          skill_totals:
            stealth: 6
            survival: 3
          gear:
          - item: chain_shirt
            quantity: 1
          - item: javelins
            quantity: 6
          - item: morningstar
            quantity: 1
          senses:
            darkvision: 60
            passive_perception: 11
          languages:
            speaks:
            - common
            - goblin
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_bugbear_stalker
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_bugbear_stalker
    citations:
    - SRD 5.2.1 pp. 271; builtin/srd-5.2.1/creature/bugbear_stalker.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/camel:
    source_sha256: 98447756b447b30f643c1ecfb44621071132c4bc085ae7e70bdd68a29c79fdd4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/camel
        implementation: dnd2024:creature_camel
        parameters:
          features:
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 4
                  roll:
                    count: 1
                    sides: 4
                    modifier: 2
                  type: bludgeoning
          senses:
            darkvision: 60
            passive_perception: 10
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_camel
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_camel
    citations:
    - SRD 5.2.1 pp. 346; builtin/srd-5.2.1/creature/camel.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/chain_devil:
    source_sha256: a0f64be27b6a412056cf4272bd1e5a748f86a04faabe14fd6cc4e554e2a868b0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/chain_devil
        implementation: dnd2024:creature_chain_devil
        parameters:
          features:
            diabolical_restoration:
              economy: trait
              operation: passive_rule
              trigger: self.dies
              unless:
                plane: nine_hells
              remove_old_body: true
              new_body:
                delay_seconds: 0
                plane: nine_hells
                hp: maximum
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: chain
                count: 2
              - operation: use_feature
                feature: conjure_infernal_chain
            chain:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 10
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 6
                    modifier: 4
                  type: slashing
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 14
                  maximum_size: large
                  appendage_pool:
                    count: 2
                    type: chains
                    cost: 1
                  while_grappled:
                    conditions:
                    - restrained
            conjure_infernal_chain:
              economy: action
              operation: saving_throw
              ability: dexterity
              dc: 15
              targets:
                kind: creature
                count: 1
                range_ft: 60
                visible: true
              failure:
                damage:
                - average: 9
                  roll:
                    count: 2
                    sides: 4
                    modifier: 4
                  type: fire
                conditions:
                - condition: restrained
                  until: self.next_turn_end
                pull:
                  maximum_size: large
                  maximum_ft: 30
                  direction: straight_toward_self
                conjured_chain:
                  disappears: self.next_turn_end
              success:
                conjured_chain_disappears: true
            unnerving_gaze:
              economy: reaction
              operation: saving_throw
              ability: wisdom
              dc: 15
              failure:
                conditions:
                - condition: frightened
                  until: target.turn_end
              trigger: target.turn_start
              targets:
                kind: creature
                count: 1
                range_ft: 30
                visible: true
                target_can_see_self: true
              success:
                immunity:
                  effect: this_unnerving_gaze
                  duration_seconds: 86400
          damage_resistances:
          - bludgeoning
          - cold
          - piercing
          - slashing
          damage_immunities:
          - fire
          - poison
          condition_immunities:
          - poisoned
          senses:
            darkvision: 120
            passive_perception: 11
            darkvision_ignores_magical_darkness: true
          languages:
            speaks:
            - infernal
            understands: []
            telepathy_range_ft: 120
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_chain_devil
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - creature.died
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_chain_devil
    citations:
    - SRD 5.2.1 pp. 272, 273; builtin/srd-5.2.1/creature/chain_devil.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/clay_golem:
    source_sha256: 4d761fe99222c84f9d52864e8cd0f3499b11d42ce81d4709fedd81d1225db444
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/clay_golem
        implementation: dnd2024:creature_clay_golem
        parameters:
          features:
            acid_absorption:
              economy: trait
              operation: passive_rule
              trigger: self.subjected_to_acid_damage
              damage_taken: 0
              heal:
                equal: acid_damage_dealt
            berserk:
              economy: trait
              operation: passive_rule
              trigger: self.turn_start
              requires:
                self_bloodied: true
              roll:
                die: 6
                success_faces:
                - 6
              on_success:
                state: berserk
              while_berserk:
                attack_priority:
                - target: nearest_visible_creature
                  if_reachable_for_move_and_attack: true
                - target: object
              ends_on:
              - self.destroyed
              - self.not_bloodied
            immutable_form:
              economy: trait
              operation: passive_rule
              can_shape_shift: false
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: slam
                count:
                  if:
                    used_this_turn: hasten
                  then: 3
                  else: 2
            slam:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 9
                reach_ft: 5
              hit:
                damage:
                - average: 10
                  roll:
                    count: 1
                    sides: 10
                    modifier: 5
                  type: bludgeoning
                - average: 6
                  roll:
                    count: 1
                    sides: 12
                    modifier: 0
                  type: acid
                maximum_hp_reduction:
                  equal: acid_damage_taken
            hasten:
              economy: bonus_action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: sequence
              effects:
              - operation: action
                action: dash
              - operation: action
                action: disengage
          damage_resistances:
          - bludgeoning
          - piercing
          - slashing
          damage_immunities:
          - acid
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
            darkvision: 60
            passive_perception: 9
          languages:
            speaks:
            - common
            understands: []
            additional_languages:
              count: 1
              chooser: creator
              can_speak: true
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_clay_golem
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - damage.resolved
      - creature.destroyed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - shape_shifting
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_clay_golem
    citations:
    - SRD 5.2.1 pp. 274; builtin/srd-5.2.1/creature/clay_golem.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/cockatrice:
    source_sha256: 64c791d4941f8fa95af37a9ca9b85b22c22ec20bfda47974ce3f8a46f481289f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/cockatrice
        implementation: dnd2024:creature_cockatrice
        parameters:
          features:
            petrifying_bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 3
                reach_ft: 5
              hit:
                damage:
                - average: 3
                  roll:
                    count: 1
                    sides: 4
                    modifier: 1
                  type: piercing
                save:
                  operation: saving_throw
                  ability: constitution
                  dc: 11
                  targets:
                    kind: creature
                  failure:
                    conditions:
                    - condition: restrained
                    repeat_save:
                      event: target.next_turn_end
                      if_still_restrained: true
                      success: end_effect
                      failure:
                        replace_condition:
                          from: restrained
                          to: petrified
                        duration_seconds: 86400
          damage_immunities: []
          condition_immunities:
          - petrified
          senses:
            darkvision: 60
            passive_perception: 11
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_cockatrice
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_cockatrice
    citations:
    - SRD 5.2.1 pp. 275; builtin/srd-5.2.1/creature/cockatrice.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/copper_dragon_wyrmling:
    source_sha256: 6472244b62dafa47e947ed7bac3165002a576aafdf27391f79018dd4fc5a218d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/copper_dragon_wyrmling
        implementation: dnd2024:creature_copper_dragon_wyrmling
        parameters:
          features:
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 10
                    modifier: 2
                  type: slashing
            acid_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 11
              targets:
                kind: creature
                selection: each
                area:
                  shape: line
                  size_ft: 20
                  origin: self
                  width_ft: 5
              failure:
                damage:
                - average: 18
                  roll:
                    count: 4
                    sides: 8
                    modifier: 0
                  type: acid
              success:
                damage_multiplier: 0.5
                other_effects: false
            slowing_breath:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 11
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 15
                  origin: self
              failure:
                can_take_reactions: false
                speed_multiplier: 0.5
                turn_economy:
                  choose_one:
                  - action
                  - bonus_action
                until: target.next_turn_end
              success:
                effects: []
          skill_totals:
            perception: 4
            stealth: 3
          damage_immunities:
          - acid
          condition_immunities: []
          senses:
            blindsight: 10
            darkvision: 60
            passive_perception: 14
          languages:
            speaks:
            - draconic
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_copper_dragon_wyrmling
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_copper_dragon_wyrmling
    citations:
    - SRD 5.2.1 pp. 275, 276; builtin/srd-5.2.1/creature/copper_dragon_wyrmling.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/crocodile:
    source_sha256: f0f74d492e74c10fca5b925e797f6bd7796be9951cd0772f690b4675143434fd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/crocodile
        implementation: dnd2024:creature_crocodile
        parameters:
          features:
            hold_breath:
              economy: trait
              operation: passive_rule
              hold_breath_seconds: 3600
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 1
                    sides: 8
                    modifier: 2
                  type: piercing
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 12
                  maximum_size: medium
                  while_grappled:
                    conditions:
                    - restrained
          skill_totals:
            stealth: 2
          senses:
            passive_perception: 10
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_crocodile
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_crocodile
    citations:
    - SRD 5.2.1 pp. 347; builtin/srd-5.2.1/creature/crocodile.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/darkmantle:
    source_sha256: 3ead20d34b82c02316daa0faf23a8a16a634201513732c1a786142b2c32e3960
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/darkmantle
        implementation: dnd2024:creature_darkmantle
        parameters:
          features:
            crush:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 1
                    sides: 6
                    modifier: 3
                  type: bludgeoning
                attach:
                  target: hit_target
                  covered_if:
                    maximum_size: medium
                    attack_had_advantage: true
                  while_covered:
                    conditions:
                    - blinded
                    suffocating: true
                  while_attached:
                    attack_targets:
                    - attached_target
                    attack_advantage: true
                    speed: 0
                    speed_bonuses: false
                    moves_with_target: true
                  detach_self:
                    economy: movement
                    cost_ft: 5
                    on_own_turn: true
                  detach_target:
                    economy: action
                    check: strength_athletics
                    dc: 13
                    actor: attached_target
            darkness_aura:
              economy: action
              resource:
                kind: daily
                uses: 1
                recovery: long_rest
              operation: create_area
              area:
                shape: emanation
                size_ft: 15
                origin: self
              magical_darkness: true
              concentration: true
              duration_seconds: 600
              darkvision_penetrates: false
              light_illuminates: false
          skill_totals:
            stealth: 3
          senses:
            blindsight: 60
            passive_perception: 10
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_darkmantle
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_darkmantle
    citations:
    - SRD 5.2.1 pp. 278, 279; builtin/srd-5.2.1/creature/darkmantle.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/deva:
    source_sha256: dbafe89b9a0b2841aa1c29178d57e4e71de140009e1b778c686722bb28ff8d57
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/deva
        implementation: dnd2024:creature_deva
        parameters:
          features:
            exalted_restoration:
              economy: trait
              operation: passive_rule
              trigger: self.dies
              unless:
                plane: mount_celestia
              remove_old_body: true
              new_body:
                delay_seconds: 0
                plane: mount_celestia
                hp: maximum
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: holy_mace
                count: 2
            holy_mace:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 8
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 6
                    modifier: 4
                  type: bludgeoning
                - average: 18
                  roll:
                    count: 4
                    sides: 8
                    modifier: 0
                  type: radiant
            spellcasting:
              economy: action
              operation: spellcasting
              ability: charisma
              components_waived:
              - material
              save_dc: 17
              options:
              - spell:
                  key: dnd2024:spells/detect_evil_and_good
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/shapechange
                usage:
                  kind: at_will
                overrides:
                  allowed_types:
                  - beast
                  - humanoid
                  gain_temporary_hp: false
                  maintain_requires_concentration: false
                  maintain_requires_temporary_hp: false
              - spell:
                  key: dnd2024:spells/commune
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/raise_dead
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            divine_aid:
              economy: bonus_action
              resource:
                kind: daily
                uses: 2
                recovery: long_rest
              operation: spell_choice
              options:
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/cure_wounds
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/lesser_restoration
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/remove_curse
              ability: charisma
          skill_totals:
            insight: 9
            perception: 9
          damage_resistances:
          - radiant
          damage_immunities: []
          condition_immunities:
          - charmed
          - exhaustion
          - frightened
          senses:
            darkvision: 120
            passive_perception: 19
          languages:
            speaks: all
            understands: []
            telepathy_range_ft: 120
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_deva
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - creature.died
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
      - curses
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_deva
    citations:
    - SRD 5.2.1 pp. 279; builtin/srd-5.2.1/creature/deva.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/doppelganger:
    source_sha256: 84c6b7ec4a3b4fa139a54de43faa34c6b294fba3a64fb8d9e162fc918969c1d0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/doppelganger
        implementation: dnd2024:creature_doppelganger
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: slam
                count: 2
              - operation: use_feature
                feature: unsettling_visage
                if_available: true
            slam:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
                advantage_if:
                  combat_round: 1
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 6
                    modifier: 4
                  type: bludgeoning
            read_thoughts:
              economy: action
              operation: cast_spell
              spell:
                key: dnd2024:spells/detect_thoughts
              ability: charisma
              save_dc: 12
              components_waived:
              - verbal
              - somatic
              - material
            unsettling_visage:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: wisdom
              dc: 12
              targets:
                kind: creature
                selection: each
                area:
                  shape: emanation
                  size_ft: 15
                  origin: self
                can_see_self: true
              failure:
                conditions:
                - condition: frightened
                repeat_save:
                  event: target.turn_end
                  success: end_effect
                automatic_success_after_seconds: 60
              success:
                effects: []
            shape_shift:
              economy: bonus_action
              operation: shape_shift
              forms:
              - type: humanoid
                size:
                - medium
                - small
              - form: true_form
              statistics_preserved_except:
              - size
              equipment_transformed: false
          skill_totals:
            deception: 6
            insight: 3
          damage_immunities: []
          condition_immunities:
          - charmed
          senses:
            darkvision: 60
            passive_perception: 11
          languages:
            speaks:
            - common
            understands: []
            additional_languages:
              count: 3
              chooser: creator
              can_speak: true
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_doppelganger
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - shape_shifting
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_doppelganger
    citations:
    - SRD 5.2.1 pp. 280; builtin/srd-5.2.1/creature/doppelganger.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/dragon_turtle:
    source_sha256: 73107fed11eba1d58212fa5d8293611be19cd230e2f6c71c59eee403a4b66478
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/dragon_turtle
        implementation: dnd2024:creature_dragon_turtle
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: bite
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: tail
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 13
                reach_ft: 15
              hit:
                damage:
                - average: 23
                  roll:
                    count: 3
                    sides: 10
                    modifier: 7
                  type: piercing
                - average: 7
                  roll:
                    count: 2
                    sides: 6
                    modifier: 0
                  type: fire
              underwater_fire_resistance_applies: false
            tail:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 13
                reach_ft: 15
              hit:
                damage:
                - average: 18
                  roll:
                    count: 2
                    sides: 10
                    modifier: 7
                  type: bludgeoning
                prone:
                  condition: prone
                  maximum_size: huge
            steam_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: constitution
              dc: 19
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 60
                  origin: self
              failure:
                damage:
                - average: 56
                  roll:
                    count: 16
                    sides: 6
                    modifier: 0
                  type: fire
              success:
                damage_multiplier: 0.5
                other_effects: false
              underwater_fire_resistance_applies: false
          damage_resistances:
          - fire
          senses:
            darkvision: 120
            passive_perception: 11
          languages:
            speaks:
            - draconic
            - language: primordial
              dialects:
              - aquan
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_dragon_turtle
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_dragon_turtle
    citations:
    - SRD 5.2.1 pp. 281; builtin/srd-5.2.1/creature/dragon_turtle.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/druid:
    source_sha256: 191e295c24f8d96330117728e55c9e8d982b319ff32a6c4177b13ba8bebb7e45
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/druid
        implementation: dnd2024:creature_druid
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - vine_staff
                - verdant_wisp
            vine_staff:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 8
                    modifier: 3
                  type: bludgeoning
                - average: 2
                  roll:
                    count: 1
                    sides: 4
                    modifier: 0
                  type: poison
            verdant_wisp:
              economy: action
              operation: attack
              attack:
                mode: ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                range_ft: 90
                long_range_ft: null
              hit:
                damage:
                - average: 10
                  roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  type: radiant
            spellcasting:
              economy: action
              operation: spellcasting
              ability: wisdom
              components_waived: []
              save_dc: 13
              options:
              - spell:
                  key: dnd2024:spells/druidcraft
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/speak_with_animals
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/entangle
                usage:
                  kind: daily
                  uses: 2
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/thunderwave
                usage:
                  kind: daily
                  uses: 2
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/animal_messenger
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/longstrider
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/moonbeam
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
          skill_totals:
            medicine: 5
            nature: 3
            perception: 5
          gear:
          - item: studded_leather_armor
            quantity: 1
          senses:
            passive_perception: 15
          languages:
            speaks:
            - common
            - druidic
            - sylvan
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_druid
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_druid
    citations:
    - SRD 5.2.1 pp. 282; builtin/srd-5.2.1/creature/druid.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/eagle:
    source_sha256: 16cfe2537117f25affaf13dc53f4b2f5c32b28fcd823d88b96dbbce7129e0ad1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/eagle
        implementation: dnd2024:creature_eagle
        parameters:
          features:
            talons:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 4
                  roll:
                    count: 1
                    sides: 4
                    modifier: 2
                  type: slashing
          skill_totals:
            perception: 6
          senses:
            passive_perception: 16
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_eagle
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_eagle
    citations:
    - SRD 5.2.1 pp. 348; builtin/srd-5.2.1/creature/eagle.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/elephant:
    source_sha256: d481dc9c59990a5980b3eed84a5a36d8067357dfc8ff77887a4cfc81ed0fbfa3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/elephant
        implementation: dnd2024:creature_elephant
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: gore
                count: 2
            gore:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 8
                reach_ft: 5
              hit:
                damage:
                - average: 15
                  roll:
                    count: 2
                    sides: 8
                    modifier: 6
                  type: piercing
                prone:
                  condition: prone
                  maximum_size: huge
                  requires:
                    immediately_preceding_straight_movement_toward_target_ft: 20
            trample:
              economy: bonus_action
              operation: saving_throw
              ability: dexterity
              dc: 16
              targets:
                kind: creature
                count: 1
                range_ft: 5
                required_conditions:
                - prone
              failure:
                damage:
                - average: 17
                  roll:
                    count: 2
                    sides: 10
                    modifier: 6
                  type: bludgeoning
              success:
                damage_multiplier: 0.5
                other_effects: false
          senses:
            passive_perception: 10
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_elephant
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_elephant
    citations:
    - SRD 5.2.1 pp. 348; builtin/srd-5.2.1/creature/elephant.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/ettercap:
    source_sha256: fb05f3326f19ba7fc9b0762db897539681dc398abe3a2675a25dcfc5a8a6f12b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ettercap
        implementation: dnd2024:creature_ettercap
        parameters:
          features:
            spider_climb:
              economy: trait
              operation: passive_rule
              climb_difficult_surfaces: true
              climb_ceilings: true
              ability_check_required: false
            web_walker:
              economy: trait
              operation: passive_rule
              ignore_web_movement_restrictions: true
              locate_creatures_on_same_web: true
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: bite
                count: 1
              - operation: attack_sequence
                feature: claw
                count: 1
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 6
                    modifier: 2
                  type: piercing
                - average: 2
                  roll:
                    count: 1
                    sides: 4
                    modifier: 0
                  type: poison
                conditions:
                - condition: poisoned
                  until: self.next_turn_start
            claw:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 2
                    sides: 4
                    modifier: 2
                  type: slashing
            web_strand:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 12
              targets:
                kind: creature
                count: 1
                range_ft: 30
                visible: true
                maximum_size: large
              failure:
                conditions:
                - condition: restrained
                  until: web.destroyed
                web_object:
                  ac: 10
                  hp: 5
                  damage_vulnerabilities:
                  - fire
                  damage_immunities:
                  - bludgeoning
                  - poison
                  - psychic
              success:
                effects: []
            reel:
              economy: bonus_action
              operation: pull
              targets:
                kind: creature
                count: 1
                range_ft: 30
                restrained_by: self.web_strand
              maximum_ft: 25
              direction: straight_toward_self
          skill_totals:
            perception: 3
            stealth: 4
            survival: 3
          senses:
            darkvision: 60
            passive_perception: 13
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_ettercap
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_ettercap
    citations:
    - SRD 5.2.1 pp. 284; builtin/srd-5.2.1/creature/ettercap.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/fire_giant:
    source_sha256: e931c6e5bd796ab8bd8c9f00ca4e7831d14fad77dd059bbaf4d0d4a65eb73e79
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/fire_giant
        implementation: dnd2024:creature_fire_giant
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - flame_sword
                - hammer_throw
            flame_sword:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 11
                reach_ft: 10
              hit:
                damage:
                - average: 21
                  roll:
                    count: 4
                    sides: 6
                    modifier: 7
                  type: slashing
                - average: 10
                  roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  type: fire
            hammer_throw:
              economy: action
              operation: attack
              attack:
                mode: ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 11
                range_ft: 60
                long_range_ft: 240
              hit:
                damage:
                - average: 23
                  roll:
                    count: 3
                    sides: 10
                    modifier: 7
                  type: bludgeoning
                - average: 4
                  roll:
                    count: 1
                    sides: 8
                    modifier: 0
                  type: fire
                push:
                  maximum_ft: 15
                  direction: straight_away_from_self
                next_attack_modifier:
                  disadvantage: true
                  expires: target.next_turn_end
                  uses: 1
          skill_totals:
            athletics: 11
            perception: 6
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
            passive_perception: 16
          languages:
            speaks:
            - giant
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_fire_giant
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_fire_giant
    citations:
    - SRD 5.2.1 pp. 285; builtin/srd-5.2.1/creature/fire_giant.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/frog:
    source_sha256: fe0428d81107f4f1268bbac3e7215a919575b4c51ad87896886de2f701f6d76c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/frog
        implementation: dnd2024:creature_frog
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            standing_leap:
              economy: trait
              operation: passive_rule
              long_jump_ft: 10
              high_jump_ft: 5
              running_start_required: false
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 3
                reach_ft: 5
              hit:
                damage:
                - average: 1
                  roll:
                    constant: 1
                  type: piercing
          skill_totals:
            perception: 1
            stealth: 3
          senses:
            darkvision: 30
            passive_perception: 11
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_frog
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_frog
    citations:
    - SRD 5.2.1 pp. 348, 349; builtin/srd-5.2.1/creature/frog.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/gelatinous_cube:
    source_sha256: a65ed1ab0167a7f7d2aa8972aa4d57e006a8423760b0b256ed059b4357e429b2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/gelatinous_cube
        implementation: dnd2024:creature_gelatinous_cube
        parameters:
          features:
            ooze_cube:
              economy: trait
              operation: passive_rule
              fills_space: true
              transparent: true
              other_creatures_can_enter: true
              on_creature_entry:
                use: engulf
                save_disadvantage: true
              inside_cover: total
              capacity:
                choose_one:
                - size: large
                  count: 1
                - sizes:
                  - medium
                  - small
                  count: 4
              pull_out:
                economy: action
                actor_within_ft: 5
                targets:
                - creature
                - object
                check: strength_athletics
                dc: 12
                actor_damage:
                - roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  type: acid
            transparent:
              economy: trait
              operation: passive_rule
              notice_check:
                ability: wisdom
                skill: perception
                dc: 15
                required_even_plain_sight: true
                unless: observer_has_seen_cube_move_or_act
            pseudopod:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 12
                  roll:
                    count: 3
                    sides: 6
                    modifier: 2
                  type: acid
            engulf:
              economy: action
              operation: saving_throw
              ability: dexterity
              dc: 12
              targets:
                kind: creature
                trigger: self.enters_target_space
                limit: first_time_during_this_move
              failure:
                damage:
                - roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  type: acid
                engulfed:
                  suffocating: true
                  can_cast_verbal_spells: false
                  conditions:
                  - restrained
                  damage_each_self_turn_start:
                  - roll:
                      count: 3
                      sides: 6
                      modifier: 0
                    type: acid
                  moves_with_self: true
                  escape:
                    economy: action
                    check: strength_athletics
                    dc: 12
                    destination: nearest_unoccupied_space
              success:
                damage_multiplier: 0.5
                move_target:
                  unoccupied: true
                  within_self_ft: 5
                if_no_unoccupied_space: failure_instead
              before_save:
                move:
                  operation: move
                  maximum_ft:
                    multiply:
                    - stat: speed
                    - 1
                  provoke_opportunity_attacks: false
                pass_through:
                  maximum_size: large
                  requires_capacity: true
          damage_immunities:
          - acid
          condition_immunities:
          - blinded
          - charmed
          - deafened
          - exhaustion
          - frightened
          - prone
          senses:
            blindsight: 60
            passive_perception: 8
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_gelatinous_cube
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_gelatinous_cube
    citations:
    - SRD 5.2.1 pp. 286, 287; builtin/srd-5.2.1/creature/gelatinous_cube.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/ghoul:
    source_sha256: 825b47ca5d9a9eb84490c5d03810d95bc159e34649b45149a45c513a0431059f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ghoul
        implementation: dnd2024:creature_ghoul
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: bite
                count: 2
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 6
                    modifier: 2
                  type: piercing
                - average: 3
                  roll:
                    count: 1
                    sides: 6
                    modifier: 0
                  type: necrotic
            claw:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 4
                  roll:
                    count: 1
                    sides: 4
                    modifier: 2
                  type: slashing
                save:
                  operation: saving_throw
                  ability: constitution
                  dc: 10
                  targets:
                    kind: creature
                    exclude:
                    - undead
                    - elf
                  failure:
                    conditions:
                    - condition: paralyzed
                      until: target.next_turn_end
          damage_immunities:
          - poison
          condition_immunities:
          - charmed
          - exhaustion
          - poisoned
          senses:
            darkvision: 60
            passive_perception: 10
          languages:
            speaks:
            - common
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_ghoul
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_ghoul
    citations:
    - SRD 5.2.1 pp. 288; builtin/srd-5.2.1/creature/ghoul.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/giant_bat:
    source_sha256: ccba443b618f2e19be76962fbec0aae6ee09adf65878eea53beaff7becf1d840
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_bat
        implementation: dnd2024:creature_giant_bat
        parameters:
          features:
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 1
                    sides: 6
                    modifier: 3
                  type: piercing
          senses:
            blindsight: 120
            passive_perception: 11
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_giant_bat
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_giant_bat
    citations:
    - SRD 5.2.1 pp. 349; builtin/srd-5.2.1/creature/giant_bat.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/giant_constrictor_snake:
    source_sha256: 8db77ee51abc801c6e32c606a5b62ee1eb11a5bb09236fa97cb68e851a5cb4fe
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_constrictor_snake
        implementation: dnd2024:creature_giant_constrictor_snake
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: bite
                count: 1
              - operation: use_feature
                feature: constrict
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 10
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 6
                    modifier: 4
                  type: piercing
            constrict:
              economy: action
              operation: saving_throw
              ability: strength
              dc: 14
              targets:
                kind: creature
                count: 1
                range_ft: 10
                visible: true
                maximum_size: large
              failure:
                damage:
                - average: 13
                  roll:
                    count: 2
                    sides: 8
                    modifier: 4
                  type: bludgeoning
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 14
              success:
                effects: []
          skill_totals:
            perception: 2
          senses:
            blindsight: 10
            passive_perception: 12
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_giant_constrictor_snake
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_giant_constrictor_snake
    citations:
    - SRD 5.2.1 pp. 350; builtin/srd-5.2.1/creature/giant_constrictor_snake.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/giant_eagle:
    source_sha256: 1c6033d7114564f6c000981cfbc5299dcdac44bb1d92c94c7c7314524e37fed5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_eagle
        implementation: dnd2024:creature_giant_eagle
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 2
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 4
                    modifier: 3
                  type: slashing
                - average: 3
                  roll:
                    count: 1
                    sides: 6
                    modifier: 0
                  type: radiant
          skill_totals:
            perception: 6
          damage_resistances:
          - necrotic
          - radiant
          senses:
            passive_perception: 16
          languages:
            speaks:
            - celestial
            understands:
            - common
            - language: primordial
              dialects:
              - auran
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_giant_eagle
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_giant_eagle
    citations:
    - SRD 5.2.1 pp. 350, 351; builtin/srd-5.2.1/creature/giant_eagle.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/giant_fly:
    source_sha256: d310e68423dd915bb8638424fd88f67215aff00d1faf53b58e49ac1d8f07f7b5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_fly
        implementation: dnd2024:creature_giant_fly
        parameters:
          features: {}
          senses:
            darkvision: 60
            passive_perception: 10
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_giant_fly
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.created
      capabilities:
      - creature_statistics
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_giant_fly
    citations:
    - SRD 5.2.1 pp. 222; builtin/srd-5.2.1/creature/giant_fly.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/giant_hyena:
    source_sha256: c8c677b1e612ef33c2520616a8d305950cc0350f620d12ee43d3898d6c74ebfa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_hyena
        implementation: dnd2024:creature_giant_hyena
        parameters:
          features:
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 10
                  roll:
                    count: 2
                    sides: 6
                    modifier: 3
                  type: piercing
            rampage:
              economy: bonus_action
              resource:
                kind: daily
                uses: 1
                recovery: long_rest
              operation: sequence
              trigger: self.dealt_damage
              requires:
                target_already_bloodied: true
              immediate: true
              effects:
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: speed
                  - 0.5
              - operation: attack_sequence
                feature: bite
                count: 1
          skill_totals:
            perception: 3
          senses:
            darkvision: 60
            passive_perception: 13
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_giant_hyena
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_giant_hyena
    citations:
    - SRD 5.2.1 pp. 352; builtin/srd-5.2.1/creature/giant_hyena.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/giant_octopus:
    source_sha256: c206d0f9dd134fd3394eed02302133a5c336f75a19992f81802fe46c5457d51b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_octopus
        implementation: dnd2024:creature_giant_octopus
        parameters:
          features:
            water_breathing:
              economy: trait
              operation: passive_rule
              breathing:
              - water
              hold_breath_outside_water_seconds: 3600
            tentacles:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 10
              hit:
                damage:
                - average: 10
                  roll:
                    count: 2
                    sides: 6
                    modifier: 3
                  type: bludgeoning
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 13
                  maximum_size: medium
                  appendage_pool:
                    count: 8
                    type: tentacles
                    cost: 8
                  while_grappled:
                    conditions:
                    - restrained
            ink_cloud:
              economy: reaction
              resource:
                kind: daily
                uses: 1
                recovery: long_rest
              operation: sequence
              trigger: self.damage_taken
              requires:
                self_underwater: true
              effects:
              - operation: create_area
                area:
                  shape: cube
                  size_ft: 10
                  center: self
                obscurement: heavy
                duration_seconds: 60
                ends_on:
                - strong_current
                - similar_dispersal_effect
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: swim_speed
                  - 1
          skill_totals:
            perception: 4
            stealth: 5
          senses:
            darkvision: 60
            passive_perception: 14
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_giant_octopus
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - damage.resolved
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_giant_octopus
    citations:
    - SRD 5.2.1 pp. 352; builtin/srd-5.2.1/creature/giant_octopus.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/giant_scorpion:
    source_sha256: d5eaacdd0cc215c5d210988aa3d831c28ac2b7a531b824a2b21818fcf673c59e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_scorpion
        implementation: dnd2024:creature_giant_scorpion
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: claw
                count: 2
              - operation: attack_sequence
                feature: sting
                count: 1
            claw:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 1
                    sides: 6
                    modifier: 3
                  type: bludgeoning
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 13
                  maximum_size: large
                  appendage_pool:
                    count: 2
                    type: claws
                    cost: 1
            sting:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 8
                    modifier: 3
                  type: piercing
                - average: 11
                  roll:
                    count: 2
                    sides: 10
                    modifier: 0
                  type: poison
          senses:
            blindsight: 60
            passive_perception: 9
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_giant_scorpion
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_giant_scorpion
    citations:
    - SRD 5.2.1 pp. 353; builtin/srd-5.2.1/creature/giant_scorpion.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/giant_spider:
    source_sha256: 2825bbfe2df6fc25af11bc17915b4f55a80d1ca24ae992792910c922859fd85f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_spider
        implementation: dnd2024:creature_giant_spider
        parameters:
          features:
            spider_climb:
              economy: trait
              operation: passive_rule
              climb_difficult_surfaces: true
              climb_ceilings: true
              ability_check_required: false
            web_walker:
              economy: trait
              operation: passive_rule
              ignore_web_movement_restrictions: true
              locate_creatures_on_same_web: true
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 8
                    modifier: 3
                  type: piercing
                - average: 7
                  roll:
                    count: 2
                    sides: 6
                    modifier: 0
                  type: poison
            web:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 13
              targets:
                kind: creature
                count: 1
                range_ft: 60
                visible: true
              failure:
                conditions:
                - condition: restrained
                  until: web.destroyed
                web_object:
                  ac: 10
                  hp: 5
                  damage_vulnerabilities:
                  - fire
                  damage_immunities:
                  - poison
                  - psychic
              success:
                effects: []
          skill_totals:
            perception: 4
            stealth: 7
          senses:
            darkvision: 60
            passive_perception: 14
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_giant_spider
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_giant_spider
    citations:
    - SRD 5.2.1 pp. 353, 354; builtin/srd-5.2.1/creature/giant_spider.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/giant_vulture:
    source_sha256: f6195e0ef6eb974bcedc7cd864bad72885c44d92723250253bf240ccc595101b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_vulture
        implementation: dnd2024:creature_giant_vulture
        parameters:
          features:
            pack_tactics:
              economy: trait
              operation: passive_rule
              attack_advantage:
                target_kind: creature
                ally_within_target_ft: 5
                ally_excludes_conditions:
                - incapacitated
            gouge:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 9
                  roll:
                    count: 2
                    sides: 6
                    modifier: 2
                  type: piercing
                conditions:
                - condition: poisoned
                  until: target.next_turn_end
          skill_totals:
            perception: 3
          damage_resistances:
          - necrotic
          senses:
            darkvision: 60
            passive_perception: 13
          languages:
            speaks: []
            understands:
            - common
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_giant_vulture
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_giant_vulture
    citations:
    - SRD 5.2.1 pp. 354; builtin/srd-5.2.1/creature/giant_vulture.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/giant_wolf_spider:
    source_sha256: 1408acb77ca6af0dbd6dc7c11ab0271d964fe28d724c4e71aee1a5c908cd0d5b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_wolf_spider
        implementation: dnd2024:creature_giant_wolf_spider
        parameters:
          features:
            spider_climb:
              economy: trait
              operation: passive_rule
              climb_difficult_surfaces: true
              climb_ceilings: true
              ability_check_required: false
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 4
                    modifier: 3
                  type: piercing
                - average: 5
                  roll:
                    count: 2
                    sides: 4
                    modifier: 0
                  type: poison
          skill_totals:
            perception: 3
            stealth: 7
          senses:
            blindsight: 10
            darkvision: 60
            passive_perception: 13
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_giant_wolf_spider
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_giant_wolf_spider
    citations:
    - SRD 5.2.1 pp. 355; builtin/srd-5.2.1/creature/giant_wolf_spider.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/gladiator:
    source_sha256: b10863e161e2fc8b60c6b833dd85ab4a7a0436bcef3fa86638fe221e6766b15a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/gladiator
        implementation: dnd2024:creature_gladiator
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: spear
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: shield_bash
            spear:
              economy: action
              operation: attack
              attack:
                mode: melee_or_ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 5
                range_ft: 20
                long_range_ft: 60
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 6
                    modifier: 4
                  type: piercing
            shield_bash:
              economy: action
              operation: saving_throw
              ability: strength
              dc: 15
              targets:
                kind: creature
                count: 1
                range_ft: 5
                visible: true
              failure:
                damage:
                - average: 9
                  roll:
                    count: 2
                    sides: 4
                    modifier: 4
                  type: bludgeoning
                prone:
                  maximum_size: medium
              success:
                effects: []
            parry:
              economy: reaction
              operation: modify_ac
              trigger: self.hit_by_melee_attack
              requires:
                holding_weapon: true
              bonus: 3
              scope: triggering_attack
              reevaluate_hit: true
          skill_totals:
            athletics: 10
            performance: 5
          gear:
          - item: shield
            quantity: 1
          - item: spears
            quantity: 3
          - item: studded_leather_armor
            quantity: 1
          senses:
            passive_perception: 11
          languages:
            speaks:
            - common
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_gladiator
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_gladiator
    citations:
    - SRD 5.2.1 pp. 289; builtin/srd-5.2.1/creature/gladiator.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/goblin_boss:
    source_sha256: 44a266077226b773888bb81b2ca6da7a46086654a90192076440e701759d661a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/goblin_boss
        implementation: dnd2024:creature_goblin_boss
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - scimitar
                - shortbow
            scimitar:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 6
                    modifier: 2
                  type: slashing
                conditional_damage:
                  if:
                    attack_had_advantage: true
                  damage:
                  - roll:
                      count: 1
                      sides: 4
                      modifier: 0
                    type: slashing
            shortbow:
              economy: action
              operation: attack
              attack:
                mode: ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                range_ft: 80
                long_range_ft: 320
              hit:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 6
                    modifier: 2
                  type: piercing
                conditional_damage:
                  if:
                    attack_had_advantage: true
                  damage:
                  - roll:
                      count: 1
                      sides: 4
                      modifier: 0
                    type: piercing
            nimble_escape:
              economy: bonus_action
              operation: choose_action
              options:
              - disengage
              - hide
              count: 1
            redirect_attack:
              economy: reaction
              operation: redirect_attack
              trigger: visible_creature.attack_roll_against_self
              ally:
                within_ft: 5
                size:
                - small
                - medium
                chooser: self
              swap_positions: true
              new_target: chosen_ally
          skill_totals:
            stealth: 6
          gear:
          - item: chain_shirt
            quantity: 1
          - item: scimitar
            quantity: 1
          - item: shield
            quantity: 1
          - item: shortbow
            quantity: 1
          senses:
            darkvision: 60
            passive_perception: 9
          languages:
            speaks:
            - common
            - goblin
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_goblin_boss
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - attack.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_goblin_boss
    citations:
    - SRD 5.2.1 pp. 290; builtin/srd-5.2.1/creature/goblin_boss.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/gold_dragon_wyrmling:
    source_sha256: 58d49938ac1fdfedf11777691a9d16846b0a4d3129481088dface3a3ad938aab
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/gold_dragon_wyrmling
        implementation: dnd2024:creature_gold_dragon_wyrmling
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 2
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 9
                  roll:
                    count: 1
                    sides: 10
                    modifier: 4
                  type: slashing
            fire_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 13
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 15
                  origin: self
              failure:
                damage:
                - average: 22
                  roll:
                    count: 4
                    sides: 10
                    modifier: 0
                  type: fire
              success:
                damage_multiplier: 0.5
                other_effects: false
            weakening_breath:
              economy: action
              operation: saving_throw
              ability: strength
              dc: 13
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 15
                  origin: self
                exclude_already_affected_by: this_breath
              failure:
                disadvantage_on:
                  roll: d20_test
                  ability: strength
                damage_roll_modifier:
                  subtract:
                    count: 1
                    sides: 4
                    modifier: 0
                repeat_save:
                  event: target.turn_end
                  success: end_effect
                automatic_success_after_seconds: 60
              success:
                effects: []
          skill_totals:
            perception: 4
            stealth: 4
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
            blindsight: 10
            darkvision: 60
            passive_perception: 14
          languages:
            speaks:
            - draconic
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_gold_dragon_wyrmling
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_gold_dragon_wyrmling
    citations:
    - SRD 5.2.1 pp. 290, 291; builtin/srd-5.2.1/creature/gold_dragon_wyrmling.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/green_dragon_wyrmling:
    source_sha256: 4c5159505ed7515d5a8e1edeed4aabe7656f82340d14fc27d06a14a3789d2b27
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/green_dragon_wyrmling
        implementation: dnd2024:creature_green_dragon_wyrmling
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 2
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 10
                    modifier: 2
                  type: slashing
                - average: 3
                  roll:
                    count: 1
                    sides: 6
                    modifier: 0
                  type: poison
            poison_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: constitution
              dc: 11
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 15
                  origin: self
              failure:
                damage:
                - average: 21
                  roll:
                    count: 6
                    sides: 6
                    modifier: 0
                  type: poison
              success:
                damage_multiplier: 0.5
                other_effects: false
          skill_totals:
            perception: 4
            stealth: 3
          damage_immunities:
          - poison
          condition_immunities:
          - poisoned
          senses:
            blindsight: 10
            darkvision: 60
            passive_perception: 14
          languages:
            speaks:
            - draconic
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_green_dragon_wyrmling
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_green_dragon_wyrmling
    citations:
    - SRD 5.2.1 pp. 293; builtin/srd-5.2.1/creature/green_dragon_wyrmling.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/griffon:
    source_sha256: 60492bba1351fc7b90cd885614687b014a365b9c2add557021c4ce9665965ace
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/griffon
        implementation: dnd2024:creature_griffon
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 2
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 8
                  roll:
                    count: 1
                    sides: 8
                    modifier: 4
                  type: piercing
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 14
                  maximum_size: medium
                  appendage_pool:
                    count: 2
                    type: claws
                    cost: 2
          skill_totals:
            perception: 5
          senses:
            darkvision: 60
            passive_perception: 15
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_griffon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_griffon
    citations:
    - SRD 5.2.1 pp. 295; builtin/srd-5.2.1/creature/griffon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/guard_captain:
    source_sha256: 1a8306fd558e93f66228380bd9fc69b6209e8cf5b6404a4a1519c2e5756ed835
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/guard_captain
        implementation: dnd2024:creature_guard_captain
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - javelin
                - longsword
            javelin:
              economy: action
              operation: attack
              attack:
                mode: melee_or_ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
                range_ft: 30
                long_range_ft: 120
              hit:
                damage:
                - average: 14
                  roll:
                    count: 3
                    sides: 6
                    modifier: 4
                  type: piercing
            longsword:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 15
                  roll:
                    count: 2
                    sides: 10
                    modifier: 4
                  type: slashing
          skill_totals:
            athletics: 6
            perception: 4
          gear:
          - item: breastplate
            quantity: 1
          - item: javelins
            quantity: 6
          - item: longsword
            quantity: 1
          - item: shield
            quantity: 1
          senses:
            passive_perception: 14
          languages:
            speaks:
            - common
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_guard_captain
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_guard_captain
    citations:
    - SRD 5.2.1 pp. 296; builtin/srd-5.2.1/creature/guard_captain.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/harpy:
    source_sha256: ec9d09d845cf173a1dbfe4c3c8c782c1f0ddd8c5c89a6d916de4ca9bcbf631ee
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/harpy
        implementation: dnd2024:creature_harpy
        parameters:
          features:
            claw:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 3
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 2
                    sides: 4
                    modifier: 1
                  type: slashing
            luring_song:
              economy: action
              operation: saving_throw
              ability: wisdom
              dc: 11
              targets:
                kind: creature
                types:
                - humanoid
                - giant
                selection: each
                area:
                  shape: emanation
                  size_ft: 300
                  origin: self
                trigger: song.starts
              failure:
                conditions:
                - condition: charmed
                  until: song.ends
                repeat_save:
                  events:
                  - target.turn_end
                  - before_target_enters_damaging_terrain
                  - event: target.damage_taken
                    source_excludes: self
                  success: end_effect_and_grant_immunity
                while_charmed:
                  conditions:
                  - incapacitated
                  ignore_other_harpy_luring_songs: true
                  movement:
                    event: target.turn
                    if_distance_greater_ft: 5
                    destination: within_5ft_of_self
                    route: most_direct
                    avoid_opportunity_attacks: false
              concentration: true
              magical: true
              success:
                immunity:
                  effect: this_harpy_luring_song
                  duration_seconds: 86400
          senses:
            passive_perception: 10
          languages:
            speaks:
            - common
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_harpy
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.ended
      - damage.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_harpy
    citations:
    - SRD 5.2.1 pp. 297; builtin/srd-5.2.1/creature/harpy.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/hezrou:
    source_sha256: 628ae233bf89865e9e425c371ebc88562b87a94dad96b2333476b8f414cdd4ef
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/hezrou
        implementation: dnd2024:creature_hezrou
        parameters:
          features:
            demonic_restoration:
              economy: trait
              operation: passive_rule
              trigger: self.dies
              unless:
                plane: abyss
              remove_old_body: true
              new_body:
                delay_seconds: 0
                plane: abyss
                hp: maximum
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            stench:
              economy: trait
              operation: saving_throw
              ability: constitution
              dc: 16
              targets:
                kind: creature
                selection: each
                area:
                  shape: emanation
                  size_ft: 10
                  origin: self
              failure:
                conditions:
                - condition: poisoned
                  until: target.next_turn_start
              trigger: target.turn_start
              success:
                effects: []
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 3
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 1
                    sides: 4
                    modifier: 4
                  type: slashing
                - average: 9
                  roll:
                    count: 2
                    sides: 8
                    modifier: 0
                  type: poison
            leap:
              economy: bonus_action
              operation: jump
              maximum_ft: 30
              movement_cost_ft: 10
          damage_resistances:
          - cold
          - fire
          - lightning
          damage_immunities:
          - poison
          condition_immunities:
          - poisoned
          senses:
            darkvision: 120
            passive_perception: 11
          languages:
            speaks:
            - abyssal
            understands: []
            telepathy_range_ft: 120
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_hezrou
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - creature.died
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_hezrou
    citations:
    - SRD 5.2.1 pp. 298; builtin/srd-5.2.1/creature/hezrou.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/hippopotamus:
    source_sha256: a20e33fea49796a6c450a6825e2aff87bb26ca4440f0fbe03ea0086815f5bf00
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/hippopotamus
        implementation: dnd2024:creature_hippopotamus
        parameters:
          features:
            hold_breath:
              economy: trait
              operation: passive_rule
              hold_breath_seconds: 600
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: bite
                count: 2
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 5
              hit:
                damage:
                - average: 16
                  roll:
                    count: 2
                    sides: 10
                    modifier: 5
                  type: piercing
          skill_totals:
            perception: 3
          senses:
            passive_perception: 13
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_hippopotamus
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_hippopotamus
    citations:
    - SRD 5.2.1 pp. 355, 356; builtin/srd-5.2.1/creature/hippopotamus.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/homunculus:
    source_sha256: 7b00c47f9b523dd882e332db28bf9af984b32bab60c8fe8bc86496bae48c6894
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/homunculus
        implementation: dnd2024:creature_homunculus
        parameters:
          features:
            telepathic_bond:
              economy: trait
              operation: passive_rule
              telepathy:
                with: master
                requires_same_plane: true
                range: unlimited
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 1
                  roll:
                    constant: 1
                  type: piercing
                save:
                  operation: saving_throw
                  ability: constitution
                  dc: 12
                  targets:
                    kind: target
                  failure:
                    conditions:
                    - condition: poisoned
                      until: self.next_turn_end
                  failure_by_5_or_more:
                    conditions:
                    - condition: poisoned
                      duration_seconds: 60
                    while_poisoned:
                      condition: unconscious
                      ends_on: target.damage_taken
          damage_immunities:
          - poison
          condition_immunities:
          - charmed
          - poisoned
          senses:
            darkvision: 60
            passive_perception: 10
          languages:
            speaks: []
            understands:
            - common
            additional_languages:
              count: 1
              chooser: creator
              can_speak: false
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_homunculus
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.ended
      - damage.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_homunculus
    citations:
    - SRD 5.2.1 pp. 299; builtin/srd-5.2.1/creature/homunculus.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/hydra:
    source_sha256: 543e8be3bf2b73fa89c57e03b5774a8463b24f7a1157d609fd76d677ffb0f45e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/hydra
        implementation: dnd2024:creature_hydra
        parameters:
          features:
            hold_breath:
              economy: trait
              operation: passive_rule
              hold_breath_seconds: 3600
            multiple_heads:
              economy: trait
              operation: passive_rule
              initial_heads: 5
              head_loss:
                trigger: damage_on_single_turn
                threshold: 25
                heads_lost: 1
              dies_if_heads: 0
              regrow:
                event: self.turn_end
                requires_living_head: true
                unless_fire_damage_since_last_turn: true
                heads_per_head_lost_since_last_turn: 2
                heal_per_regrowth_event: 20
            reactive_heads:
              economy: trait
              operation: passive_rule
              extra_reaction_capacity:
                max:
                - 0
                - subtract:
                  - stat: heads
                  - 1
              refresh_event: self.turn_start
              recompute_only_at_refresh: true
              allowed_uses:
              - opportunity_attack
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: bite
                count:
                  stat: heads
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 8
                reach_ft: 10
              hit:
                damage:
                - average: 10
                  roll:
                    count: 1
                    sides: 10
                    modifier: 5
                  type: piercing
          skill_totals:
            perception: 6
          damage_immunities: []
          condition_immunities:
          - blinded
          - charmed
          - deafened
          - frightened
          - stunned
          - unconscious
          senses:
            darkvision: 60
            passive_perception: 16
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_hydra
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - creature_transformation
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_hydra
    citations:
    - SRD 5.2.1 pp. 300; builtin/srd-5.2.1/creature/hydra.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/ice_mephit:
    source_sha256: 57d37a51b39d28edc57110f50dd3c91a26a4ed0bc0e29b569ceff74554072539
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ice_mephit
        implementation: dnd2024:creature_ice_mephit
        parameters:
          features:
            death_burst:
              economy: trait
              operation: saving_throw
              ability: constitution
              dc: 10
              targets:
                kind: creature
                selection: each
                area:
                  shape: emanation
                  size_ft: 5
                  origin: self
              failure:
                damage:
                - average: 5
                  roll:
                    count: 2
                    sides: 4
                    modifier: 0
                  type: cold
              success:
                damage_multiplier: 0.5
                other_effects: false
              trigger: self.dies
            claw:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 3
                reach_ft: 5
              hit:
                damage:
                - average: 3
                  roll:
                    count: 1
                    sides: 4
                    modifier: 1
                  type: slashing
                - average: 2
                  roll:
                    count: 1
                    sides: 4
                    modifier: 0
                  type: cold
            fog_cloud:
              economy: action
              resource:
                kind: daily
                uses: 1
                recovery: long_rest
              operation: cast_spell
              spell:
                key: dnd2024:spells/fog_cloud
              ability: charisma
              components_waived:
              - verbal
              - somatic
              - material
            frost_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: constitution
              dc: 10
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 15
                  origin: self
              failure:
                damage:
                - average: 7
                  roll:
                    count: 3
                    sides: 4
                    modifier: 0
                  type: cold
              success:
                damage_multiplier: 0.5
                other_effects: false
          skill_totals:
            perception: 2
            stealth: 3
          damage_vulnerabilities:
          - fire
          damage_immunities:
          - cold
          - poison
          condition_immunities:
          - exhaustion
          - poisoned
          senses:
            darkvision: 60
            passive_perception: 12
          languages:
            speaks:
            - language: primordial
              dialects:
              - aquan
              - auran
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_ice_mephit
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - creature.died
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_ice_mephit
    citations:
    - SRD 5.2.1 pp. 307; builtin/srd-5.2.1/creature/ice_mephit.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/invisible_stalker:
    source_sha256: a048c73038218c47a0b2e8fd7059aa04fc070e16414b07dc1250a3a99135fa9c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/invisible_stalker
        implementation: dnd2024:creature_invisible_stalker
        parameters:
          features:
            air_form:
              economy: trait
              operation: passive_rule
              occupy_enemy_space: true
              minimum_gap_inches: 1
              extra_movement_cost: 0
            invisibility:
              economy: trait
              operation: passive_rule
              conditions:
              - condition: invisible
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: wind_swipe
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: vortex
            wind_swipe:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 5
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 6
                    modifier: 4
                  type: force
            vortex:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 14
              targets:
                kind: creature
                count: 1
                maximum_size: large
                location: self.space
              failure:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 8
                    modifier: 3
                  type: thunder
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 13
                  while_grappled:
                    can_cast_verbal_spells: false
                    damage_each_self_turn_start:
                    - roll:
                        count: 2
                        sides: 6
                        modifier: 0
                      type: thunder
              success:
                effects: []
          skill_totals:
            perception: 8
            stealth: 10
          damage_resistances:
          - bludgeoning
          - piercing
          - slashing
          damage_immunities:
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
            darkvision: 60
            passive_perception: 18
          languages:
            speaks:
            - common
            - language: primordial
              dialects:
              - auran
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_invisible_stalker
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_invisible_stalker
    citations:
    - SRD 5.2.1 pp. 301, 302; builtin/srd-5.2.1/creature/invisible_stalker.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/killer_whale:
    source_sha256: 0c687f155449a989784f4f075cf0f85da9e667a1d6fbafdd0ab4ba4b64e99a65
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/killer_whale
        implementation: dnd2024:creature_killer_whale
        parameters:
          features:
            hold_breath:
              economy: trait
              operation: passive_rule
              hold_breath_seconds: 1800
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 21
                  roll:
                    count: 5
                    sides: 6
                    modifier: 4
                  type: piercing
          skill_totals:
            perception: 3
            stealth: 4
          senses:
            blindsight: 120
            passive_perception: 13
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_killer_whale
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_killer_whale
    citations:
    - SRD 5.2.1 pp. 356; builtin/srd-5.2.1/creature/killer_whale.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/kraken:
    source_sha256: 8d91fc232b71d40e74a1a014b99db9d9fe9fc7b81fe82586d5e08bd3d093f7e1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/kraken
        implementation: dnd2024:creature_kraken
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 4
                recovery: long_rest
                lair_uses: 5
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            siege_monster:
              economy: trait
              operation: passive_rule
              damage_multiplier: 2
              target_kinds:
              - object
              - structure
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: tentacle
                count: 2
              - operation: choice
                options:
                - operation: use_feature
                  feature: fling
                - operation: use_feature
                  feature: lightning_strike
                - operation: use_feature
                  feature: swallow
            tentacle:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 17
                reach_ft: 30
              hit:
                damage:
                - average: 24
                  roll:
                    count: 4
                    sides: 6
                    modifier: 10
                  type: bludgeoning
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 20
                  appendage_pool:
                    count: 10
                    type: tentacles
                    cost: 1
                  while_grappled:
                    conditions:
                    - restrained
            fling:
              economy: action
              operation: saving_throw
              ability: dexterity
              dc: 25
              targets:
                selection: thrown_creature_and_creatures_in_destination
              failure:
                damage:
                - roll:
                    count: 4
                    sides: 8
                    modifier: 0
                  type: bludgeoning
                conditions:
                - condition: prone
              success:
                damage_multiplier: 0.5
                other_effects: false
              before_save:
                throw:
                  target:
                    grappled_by: self
                    maximum_size: large
                  destination:
                    self_visible: true
                    within_self_ft: 60
                    in_air: false
            lightning_strike:
              economy: action
              operation: saving_throw
              ability: dexterity
              dc: 23
              targets:
                kind: creature
                count: 1
                range_ft: 120
                visible: true
              failure:
                damage:
                - average: 33
                  roll:
                    count: 6
                    sides: 10
                    modifier: 0
                  type: lightning
              success:
                damage_multiplier: 0.5
                other_effects: false
            swallow:
              economy: action
              operation: saving_throw
              ability: dexterity
              dc: 25
              failure:
                end_grapple: true
                swallow:
                  maximum_size: large
                  conditions:
                  - restrained
                  cover_against_outside: total
                  periodic_damage:
                    event: target.turn_start
                    damage:
                    - roll:
                        count: 7
                        sides: 6
                        modifier: 0
                      type: acid
                  regurgitate:
                    trigger: turn_end
                    if_damage_on_single_turn_from_one_inside_creature_at_least: 50
                    save:
                      ability: constitution
                      dc: 25
                      failure:
                        eject: all
                        within_ft: 10
                        conditions:
                        - prone
                  on_self_death:
                    remove_conditions:
                    - restrained
                    escape_movement_ft: 15
                    exit_condition: prone
                damage:
                - roll:
                    count: 3
                    sides: 8
                    modifier: 10
                  type: piercing
                end_grapple_requires_swallowed: true
              targets:
                kind: creature
                count: 1
                grappled_by: self
              capacity: 4
              success:
                effects: []
            storm_bolt:
              economy: legendary_action
              legendary_cost: 1
              operation: use_feature
              feature: lightning_strike
            toxic_ink:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: saving_throw
              ability: constitution
              dc: 23
              targets:
                kind: creature
                selection: each
                area:
                  shape: emanation
                  size_ft: 15
                  origin: self
              failure:
                conditions:
                - condition: blinded
                  until: self.next_turn_end
                - condition: poisoned
                  until: self.next_turn_end
                then_move:
                  operation: move
                  maximum_ft:
                    multiply:
                    - stat: speed
                    - 1
              requires:
                self_underwater: true
              success:
                effects: []
          skill_totals:
            history: 13
            perception: 11
          damage_immunities:
          - cold
          - lightning
          condition_immunities:
          - frightened
          - grappled
          - paralyzed
          - restrained
          senses:
            truesight: 120
            passive_perception: 21
          languages:
            speaks: []
            understands:
            - abyssal
            - celestial
            - infernal
            - primordial
            telepathy_range_ft: 120
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            lair_uses: 4
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_kraken
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - rest.completed
      - saving_throw.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - legendary_actions
      - defenses
    requires:
    - dnd2024:creature_kraken
    citations:
    - SRD 5.2.1 pp. 303; builtin/srd-5.2.1/creature/kraken.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/lich:
    source_sha256: 542a93cbd57597c12c20fd048ac2069e4ef9242fff9a2a69d6edc1d823b086cd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/lich
        implementation: dnd2024:creature_lich
        parameters:
          features:
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 4
                recovery: long_rest
                lair_uses: 5
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            spirit_jar:
              economy: trait
              operation: passive_rule
              trigger: self.destroyed
              requires:
                has_spirit_jar: true
              new_body:
                delay:
                  roll:
                    count: 1
                    sides: 10
                    modifier: 0
                  unit: day
                hp: maximum
                location:
                  unoccupied: true
                  in: self.lair
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 3
                options:
                - eldritch_burst
                - paralyzing_touch
            eldritch_burst:
              economy: action
              operation: attack
              attack:
                mode: melee_or_ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 12
                reach_ft: 5
                range_ft: 120
                long_range_ft: null
              hit:
                damage:
                - average: 31
                  roll:
                    count: 4
                    sides: 12
                    modifier: 5
                  type: force
            paralyzing_touch:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 12
                reach_ft: 5
              hit:
                damage:
                - average: 15
                  roll:
                    count: 3
                    sides: 6
                    modifier: 5
                  type: cold
                conditions:
                - condition: paralyzed
                  until: self.next_turn_start
            spellcasting:
              economy: action
              operation: spellcasting
              ability: intelligence
              components_waived: []
              save_dc: 20
              options:
              - spell:
                  key: dnd2024:spells/detect_magic
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/detect_thoughts
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/dispel_magic
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/fireball
                usage:
                  kind: at_will
                cast_level: 5
              - spell:
                  key: dnd2024:spells/invisibility
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/lightning_bolt
                usage:
                  kind: at_will
                cast_level: 5
              - spell:
                  key: dnd2024:spells/mage_hand
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/prestidigitation
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/animate_dead
                usage:
                  kind: daily
                  uses: 2
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/dimension_door
                usage:
                  kind: daily
                  uses: 2
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/plane_shift
                usage:
                  kind: daily
                  uses: 2
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/chain_lightning
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/finger_of_death
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/power_word_kill
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/scrying
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            protective_magic:
              economy: reaction
              operation: spell_choice
              options:
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/counterspell
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/shield
              trigger: chosen_spell.normal_trigger
              ability: intelligence
              save_dc: 20
            deathly_teleport:
              economy: legendary_action
              legendary_cost: 1
              operation: sequence
              effects:
              - operation: teleport
                maximum_ft: 60
                visible: true
                unoccupied: true
              - operation: damage
                targets:
                  kind: creature
                  selection: each
                  within_old_space_ft: 10
                damage:
                - roll:
                    count: 2
                    sides: 10
                    modifier: 0
                  type: necrotic
            disrupt_life:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: saving_throw
              ability: constitution
              dc: 20
              targets:
                kind: creature
                selection: each
                area:
                  shape: emanation
                  size_ft: 20
                  origin: self
                exclude_types:
                - undead
              failure:
                damage:
                - average: 31
                  roll:
                    count: 9
                    sides: 6
                    modifier: 0
                  type: necrotic
              success:
                damage_multiplier: 0.5
                other_effects: false
            frightening_gaze:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: cast_spell
              spell:
                key: dnd2024:spells/fear
              ability: intelligence
              save_dc: 20
          skill_totals:
            arcana: 19
            history: 12
            insight: 9
            perception: 9
          damage_resistances:
          - cold
          - lightning
          damage_immunities:
          - necrotic
          - poison
          condition_immunities:
          - charmed
          - exhaustion
          - frightened
          - paralyzed
          - poisoned
          gear:
          - item: component_pouch
            quantity: 1
          senses:
            truesight: 120
            passive_perception: 19
          languages:
            speaks: all
            understands: []
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            lair_uses: 4
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_lich
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - creature.destroyed
      - rest.completed
      - saving_throw.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - teleportation
      - spatial_queries
      - limited_resources
      - legendary_actions
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_lich
    citations:
    - SRD 5.2.1 pp. 304; builtin/srd-5.2.1/creature/lich.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/mage:
    source_sha256: f6710ee172931b67c6325be3cb5c693ab18629f3c6f5d487a0b0eddaf57516e2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/mage
        implementation: dnd2024:creature_mage
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: arcane_burst
                count: 3
            arcane_burst:
              economy: action
              operation: attack
              attack:
                mode: melee_or_ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
                range_ft: 120
                long_range_ft: null
              hit:
                damage:
                - average: 16
                  roll:
                    count: 3
                    sides: 8
                    modifier: 3
                  type: force
            spellcasting:
              economy: action
              operation: spellcasting
              ability: intelligence
              components_waived: []
              save_dc: 14
              options:
              - spell:
                  key: dnd2024:spells/detect_magic
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/light
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/mage_armor
                usage:
                  kind: at_will
                included_in_header_ac: true
              - spell:
                  key: dnd2024:spells/mage_hand
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/prestidigitation
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/fireball
                usage:
                  kind: daily
                  uses: 2
                  pool: per_spell
                  recovery: long_rest
                cast_level: 4
              - spell:
                  key: dnd2024:spells/invisibility
                usage:
                  kind: daily
                  uses: 2
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/cone_of_cold
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/fly
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            misty_step:
              economy: bonus_action
              resource:
                kind: daily
                uses: 3
                recovery: long_rest
              operation: cast_spell
              spell:
                key: dnd2024:spells/misty_step
              ability: intelligence
            protective_magic:
              economy: reaction
              resource:
                kind: daily
                uses: 3
                recovery: long_rest
              operation: spell_choice
              options:
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/counterspell
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/shield
              trigger: chosen_spell.normal_trigger
              ability: intelligence
              save_dc: 14
          skill_totals:
            arcana: 6
            history: 6
            perception: 4
          gear:
          - item: wand
            quantity: 1
          senses:
            passive_perception: 14
          languages:
            speaks:
            - common
            understands: []
            additional_languages:
              count: 3
              chooser: creator
              can_speak: true
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_mage
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_mage
    citations:
    - SRD 5.2.1 pp. 305; builtin/srd-5.2.1/creature/mage.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/mammoth:
    source_sha256: c0023dc4d0c2a03142232a56070f783fb67e7a85c36f442cc2f2e75868a82135
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/mammoth
        implementation: dnd2024:creature_mammoth
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: gore
                count: 2
            gore:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 10
                reach_ft: 10
              hit:
                damage:
                - average: 18
                  roll:
                    count: 2
                    sides: 10
                    modifier: 7
                  type: piercing
                prone:
                  condition: prone
                  maximum_size: huge
                  requires:
                    immediately_preceding_straight_movement_toward_target_ft: 20
            trample:
              economy: bonus_action
              operation: saving_throw
              ability: dexterity
              dc: 18
              targets:
                kind: creature
                count: 1
                range_ft: 5
                required_conditions:
                - prone
              failure:
                damage:
                - average: 29
                  roll:
                    count: 4
                    sides: 10
                    modifier: 7
                  type: bludgeoning
              success:
                damage_multiplier: 0.5
                other_effects: false
          senses:
            passive_perception: 10
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_mammoth
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_mammoth
    citations:
    - SRD 5.2.1 pp. 357; builtin/srd-5.2.1/creature/mammoth.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/mastiff:
    source_sha256: 451d21bd241630173defcd47549fac812b057dcd45cf125a7d638dc7f35768cc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/mastiff
        implementation: dnd2024:creature_mastiff
        parameters:
          features:
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 3
                reach_ft: 5
              hit:
                damage:
                - average: 4
                  roll:
                    count: 1
                    sides: 6
                    modifier: 1
                  type: piercing
                prone:
                  condition: prone
                  maximum_size: medium
          skill_totals:
            perception: 5
          senses:
            darkvision: 60
            passive_perception: 15
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_mastiff
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_mastiff
    citations:
    - SRD 5.2.1 pp. 357; builtin/srd-5.2.1/creature/mastiff.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/merrow:
    source_sha256: a1611a5cad10fb70d5f33c44d33e832c2f839c01cd6e490dabe2efeb53d61be2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/merrow
        implementation: dnd2024:creature_merrow
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - bite
                - claw
                - harpoon
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 1
                    sides: 4
                    modifier: 4
                  type: piercing
                conditions:
                - condition: poisoned
                  until: self.next_turn_end
            claw:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 9
                  roll:
                    count: 2
                    sides: 4
                    modifier: 4
                  type: slashing
            harpoon:
              economy: action
              operation: attack
              attack:
                mode: melee_or_ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
                range_ft: 20
                long_range_ft: 60
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 6
                    modifier: 4
                  type: piercing
                pull:
                  maximum_ft: 15
                  direction: straight_toward_self
                  maximum_size: large
          senses:
            darkvision: 60
            passive_perception: 10
          languages:
            speaks:
            - abyssal
            - language: primordial
              dialects:
              - aquan
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_merrow
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_merrow
    citations:
    - SRD 5.2.1 pp. 308; builtin/srd-5.2.1/creature/merrow.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/minotaur_skeleton:
    source_sha256: 76ce16ab443a9b3478029332725257535750fb7c6c7987e3be03320e9f246cc7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/minotaur_skeleton
        implementation: dnd2024:creature_minotaur_skeleton
        parameters:
          features:
            gore:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 6
                    modifier: 4
                  type: piercing
                prone:
                  condition: prone
                  maximum_size: large
                  requires:
                    immediately_preceding_straight_movement_toward_target_ft: 20
                conditional_damage:
                  requires:
                    maximum_target_size: large
                    immediately_preceding_straight_movement_toward_target_ft: 20
                  damage:
                  - roll:
                      count: 2
                      sides: 8
                      modifier: 0
                    type: piercing
            slam:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 15
                  roll:
                    count: 2
                    sides: 10
                    modifier: 4
                  type: bludgeoning
          damage_vulnerabilities:
          - bludgeoning
          damage_immunities:
          - poison
          condition_immunities:
          - exhaustion
          - poisoned
          senses:
            darkvision: 60
            passive_perception: 9
          languages:
            speaks: []
            understands:
            - abyssal
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_minotaur_skeleton
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_minotaur_skeleton
    citations:
    - SRD 5.2.1 pp. 326; builtin/srd-5.2.1/creature/minotaur_skeleton.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/mummy_lord:
    source_sha256: 458d27f5491ff71ef4872792635794193d8039e502044aac4a6a4731332b9aa4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/mummy_lord
        implementation: dnd2024:creature_mummy_lord
        parameters:
          features:
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 3
                recovery: long_rest
                lair_uses: 4
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            undead_restoration:
              economy: trait
              operation: passive_rule
              trigger: self.destroyed
              requires:
                heart_intact: true
              new_body:
                delay_seconds: 86400
                hp: maximum
                location:
                  in: self.lair
                  unoccupied: true
              heart_object:
                size: tiny
                ac: 17
                hp: 10
                damage_immunities:
                  all_except:
                  - fire
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 1
                options:
                - rotting_fist
                - channel_negative_energy
              - operation: use_feature
                feature: dreadful_glare
            rotting_fist:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 9
                reach_ft: 5
              hit:
                damage:
                - average: 15
                  roll:
                    count: 2
                    sides: 10
                    modifier: 4
                  type: bludgeoning
                - average: 10
                  roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  type: necrotic
                curse:
                  target_kind: creature
                  can_regain_hp: false
                  long_rest_benefits: false
                  maximum_hp_reduction:
                    interval_seconds: 86400
                    roll:
                      count: 3
                      sides: 6
                      modifier: 0
                on_reduced_to_zero_hp:
                  die: true
                  turn_to_dust: true
            channel_negative_energy:
              economy: action
              operation: attack
              attack:
                mode: ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 9
                range_ft: 60
                long_range_ft: null
              hit:
                damage:
                - average: 25
                  roll:
                    count: 6
                    sides: 6
                    modifier: 4
                  type: necrotic
            dreadful_glare:
              economy: action
              operation: saving_throw
              ability: wisdom
              dc: 17
              targets:
                kind: creature
                count: 1
                range_ft: 60
                visible: true
              failure:
                damage:
                - average: 25
                  roll:
                    count: 6
                    sides: 6
                    modifier: 4
                  type: psychic
                conditions:
                - condition: paralyzed
                  until: self.next_turn_end
              success:
                effects: []
            spellcasting:
              economy: action
              operation: spellcasting
              ability: wisdom
              components_waived:
              - material
              save_dc: 17
              supplied_spell_attack_total: 9
              options:
              - spell:
                  key: dnd2024:spells/dispel_magic
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/thaumaturgy
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/animate_dead
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/harm
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/insect_plague
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
                cast_level: 7
              material_components_available: true
            whirlwind_of_sand:
              economy: reaction
              operation: sequence
              trigger: self.hit_by_attack_roll
              effects:
              - operation: modify_ac
                bonus: 2
                scope: triggering_attack
                reevaluate_hit: true
              - operation: teleport
                maximum_ft: 60
                unoccupied: true
                visible: true
              - operation: apply_conditions
                targets:
                  kind: creature
                  selection: self_choice
                  visible: true
                  within_destination_ft: 5
                conditions:
                - condition: blinded
                  until: self.next_turn_end
            dread_command:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: cast_spell
              spell:
                key: dnd2024:spells/command
              cast_level: 2
              ability: wisdom
              save_dc: 17
            glare:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: use_feature
              feature: dreadful_glare
            necrotic_strike:
              economy: legendary_action
              legendary_cost: 1
              operation: attack_choice
              count: 1
              options:
              - rotting_fist
              - channel_negative_energy
          skill_totals:
            history: 5
            perception: 9
            religion: 5
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
            truesight: 60
            passive_perception: 19
          languages:
            speaks:
            - common
            understands: []
            additional_languages:
              count: 3
              chooser: creator
              can_speak: true
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            lair_uses: 4
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_mummy_lord
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - creature.destroyed
      - rest.completed
      - saving_throw.resolved
      - attack.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - teleportation
      - spatial_queries
      - limited_resources
      - curses
      - legendary_actions
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_mummy_lord
    citations:
    - SRD 5.2.1 pp. 309, 310; builtin/srd-5.2.1/creature/mummy_lord.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/nightmare:
    source_sha256: 33d5901d3be792cf50fdfc2a951f885366723aeecb46368fb5847548adbaea7c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/nightmare
        implementation: dnd2024:creature_nightmare
        parameters:
          features:
            confer_fire_resistance:
              economy: trait
              operation: passive_rule
              optional: true
              grant:
                damage_resistances:
                - fire
                target: rider
                while: mounted_on_self
            illumination:
              economy: trait
              operation: passive_rule
              bright_radius_ft: 10
              additional_dim_radius_ft: 10
            hooves:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 13
                  roll:
                    count: 2
                    sides: 8
                    modifier: 4
                  type: bludgeoning
                - average: 10
                  roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  type: fire
            ethereal_stride:
              economy: action
              operation: planar_teleport
              targets:
                self: true
                additional:
                  count_maximum: 3
                  willing: true
                  range_ft: 5
              plane_pairs:
              - - material
                - ethereal
              - - ethereal
                - material
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
            passive_perception: 11
          languages:
            speaks: []
            understands:
            - abyssal
            - common
            - infernal
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_nightmare
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - teleportation
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_nightmare
    citations:
    - SRD 5.2.1 pp. 311; builtin/srd-5.2.1/creature/nightmare.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/octopus:
    source_sha256: 22868868d9c682f5765fc7fe99e82886dc2bd55aae129707aba404ee139b152d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/octopus
        implementation: dnd2024:creature_octopus
        parameters:
          features:
            compression:
              economy: trait
              operation: passive_rule
              minimum_gap_inches: 1
              extra_movement_cost: 0
            water_breathing:
              economy: trait
              operation: passive_rule
              breathing:
              - water
            tentacles:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 1
                  roll:
                    constant: 1
                  type: bludgeoning
            ink_cloud:
              economy: reaction
              resource:
                kind: daily
                uses: 1
                recovery: long_rest
              operation: sequence
              trigger: creature.turn_end
              requires:
                self_underwater: true
                triggering_creature_within_ft: 5
              effects:
              - operation: create_area
                area:
                  shape: cube
                  size_ft: 5
                  center: self
                obscurement: heavy
                duration_seconds: 60
                ends_on:
                - strong_current
                - similar_dispersal_effect
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: swim_speed
                  - 1
          skill_totals:
            perception: 2
            stealth: 6
          senses:
            darkvision: 30
            passive_perception: 12
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_octopus
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.ended
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_octopus
    citations:
    - SRD 5.2.1 pp. 357, 358; builtin/srd-5.2.1/creature/octopus.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/oni:
    source_sha256: c77e132b8e2b62675c30c847624e7f2599ce741c36192c86e8c04ed292cbe327
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/oni
        implementation: dnd2024:creature_oni
        parameters:
          features:
            regeneration:
              economy: trait
              operation: passive_rule
              trigger: self.turn_start
              requires:
                minimum_hp: 1
              heal: 10
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - claw
                - nightmare_ray
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: spellcasting
            claw:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 10
              hit:
                damage:
                - average: 10
                  roll:
                    count: 1
                    sides: 12
                    modifier: 4
                  type: slashing
                - average: 9
                  roll:
                    count: 2
                    sides: 8
                    modifier: 0
                  type: necrotic
            nightmare_ray:
              economy: action
              operation: attack
              attack:
                mode: ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                range_ft: 60
                long_range_ft: null
              hit:
                damage:
                - average: 9
                  roll:
                    count: 2
                    sides: 6
                    modifier: 2
                  type: psychic
                conditions:
                - condition: frightened
                  until: self.next_turn_start
            shape_shift:
              economy: action
              operation: shape_shift
              forms:
              - type: humanoid
                size:
                - small
                - medium
              - type: giant
                size: large
              - form: true_form
              statistics_preserved_except:
              - size
              equipment_transformed: false
            spellcasting:
              economy: action
              operation: spellcasting
              ability: charisma
              components_waived:
              - material
              save_dc: 13
              options:
              - spell:
                  key: dnd2024:spells/charm_person
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
                cast_level: 2
              - spell:
                  key: dnd2024:spells/darkness
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/gaseous_form
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/sleep
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            invisibility:
              economy: bonus_action
              operation: cast_spell
              spell:
                key: dnd2024:spells/invisibility
              ability: charisma
              targets:
              - self
              components_waived:
              - verbal
              - somatic
              - material
          skill_totals:
            arcana: 5
            deception: 8
            perception: 4
          damage_resistances:
          - cold
          senses:
            darkvision: 60
            passive_perception: 14
          languages:
            speaks:
            - common
            - giant
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_oni
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spellcasting
      - shape_shifting
      - spatial_queries
      - limited_resources
      - hit_points
    requires:
    - dnd2024:creature_oni
    citations:
    - SRD 5.2.1 pp. 312, 313; builtin/srd-5.2.1/creature/oni.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/owl:
    source_sha256: 65a0d332791347f3406043909c9544efcf57629da95bf34fb0aaf501e5ef910b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/owl
        implementation: dnd2024:creature_owl
        parameters:
          features:
            flyby:
              economy: trait
              operation: passive_rule
              opportunity_attacks: false
              while:
                movement_mode: fly
            talons:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 3
                reach_ft: 5
              hit:
                damage:
                - average: 1
                  roll:
                    constant: 1
                  type: slashing
          skill_totals:
            perception: 5
            stealth: 5
          senses:
            darkvision: 120
            passive_perception: 15
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_owl
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_owl
    citations:
    - SRD 5.2.1 pp. 358; builtin/srd-5.2.1/creature/owl.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/pegasus:
    source_sha256: 9531d76717d959784fda16de169fa0e51b1a91b8f572be8b12588a6ca6134eeb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/pegasus
        implementation: dnd2024:creature_pegasus
        parameters:
          features:
            hooves:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 6
                    modifier: 4
                  type: bludgeoning
                - average: 5
                  roll:
                    count: 2
                    sides: 4
                    modifier: 0
                  type: radiant
          skill_totals:
            perception: 6
          senses:
            passive_perception: 16
          languages:
            speaks: []
            understands:
            - celestial
            - common
            - elvish
            - sylvan
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_pegasus
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_pegasus
    citations:
    - SRD 5.2.1 pp. 313; builtin/srd-5.2.1/creature/pegasus.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/pirate:
    source_sha256: c96360d9fbfd8bce521dd5d788b21ad705af121c77f0fb168e4898a794d3e2eb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/pirate
        implementation: dnd2024:creature_pirate
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: dagger
                count: 2
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: enthralling_panache
            dagger:
              economy: action
              operation: attack
              attack:
                mode: melee_or_ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
                range_ft: 20
                long_range_ft: 60
              hit:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 4
                    modifier: 3
                  type: piercing
            enthralling_panache:
              economy: action
              operation: saving_throw
              ability: wisdom
              dc: 12
              targets:
                kind: creature
                count: 1
                range_ft: 30
                visible: true
              failure:
                conditions:
                - condition: charmed
                  until: self.next_turn_start
              success:
                effects: []
          gear:
          - item: daggers
            quantity: 6
          - item: leather_armor
            quantity: 1
          senses:
            passive_perception: 11
          languages:
            speaks:
            - common
            understands: []
            additional_languages:
              count: 1
              chooser: creator
              can_speak: true
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_pirate
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_pirate
    citations:
    - SRD 5.2.1 pp. 314; builtin/srd-5.2.1/creature/pirate.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/planetar:
    source_sha256: 4046921338839496568b0a977af7301f8e8fca56f733a5423f8d23cb29a2343d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/planetar
        implementation: dnd2024:creature_planetar
        parameters:
          features:
            divine_awareness:
              economy: trait
              operation: passive_rule
              detect_lies:
                requires: heard
            exalted_restoration:
              economy: trait
              operation: passive_rule
              trigger: self.dies
              unless:
                plane: mount_celestia
              remove_old_body: true
              new_body:
                delay_seconds: 0
                plane: mount_celestia
                hp: maximum
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: choice
                options:
                - operation: attack_sequence
                  feature: radiant_sword
                  count: 3
                - operation: repeat
                  count: 2
                  effect:
                    operation: use_feature
                    feature: holy_burst
            radiant_sword:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 12
                reach_ft: 10
              hit:
                damage:
                - average: 14
                  roll:
                    count: 2
                    sides: 6
                    modifier: 7
                  type: slashing
                - average: 18
                  roll:
                    count: 4
                    sides: 8
                    modifier: 0
                  type: radiant
            holy_burst:
              economy: action
              operation: saving_throw
              ability: dexterity
              dc: 20
              targets:
                kind: enemy
                selection: each
                area:
                  shape: sphere
                  radius_ft: 20
                  center:
                    visible: true
                    range_ft: 120
              failure:
                damage:
                - average: 24
                  roll:
                    count: 7
                    sides: 6
                    modifier: 0
                  type: radiant
              success:
                damage_multiplier: 0.5
                other_effects: false
            spellcasting:
              economy: action
              operation: spellcasting
              ability: charisma
              components_waived:
              - material
              save_dc: 20
              options:
              - spell:
                  key: dnd2024:spells/detect_evil_and_good
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/commune
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/control_weather
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/dispel_evil_and_good
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/raise_dead
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            divine_aid:
              economy: bonus_action
              resource:
                kind: daily
                uses: 2
                recovery: long_rest
              operation: spell_choice
              options:
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/cure_wounds
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/invisibility
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/lesser_restoration
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/remove_curse
              ability: charisma
          skill_totals:
            perception: 11
          damage_resistances:
          - radiant
          damage_immunities: []
          condition_immunities:
          - charmed
          - exhaustion
          - frightened
          senses:
            truesight: 120
            passive_perception: 21
          languages:
            speaks: all
            understands: []
            telepathy_range_ft: 120
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_planetar
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - creature.died
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
      - curses
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_planetar
    citations:
    - SRD 5.2.1 pp. 315; builtin/srd-5.2.1/creature/planetar.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/pony:
    source_sha256: e1637c99fb2de4b3c29620e1022947db2fd5b750afa2ab55eceb01fac27a74b7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/pony
        implementation: dnd2024:creature_pony
        parameters:
          features:
            hooves:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 4
                  roll:
                    count: 1
                    sides: 4
                    modifier: 2
                  type: bludgeoning
          senses:
            passive_perception: 10
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_pony
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_pony
    citations:
    - SRD 5.2.1 pp. 359; builtin/srd-5.2.1/creature/pony.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/pseudodragon:
    source_sha256: 4f9e2968b7dc0491ecbb94785f3dd2c5e715b25cc6b9e936761e6ce33f3efa38
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/pseudodragon
        implementation: dnd2024:creature_pseudodragon
        parameters:
          features:
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: bite
                count: 2
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 4
                  roll:
                    count: 1
                    sides: 4
                    modifier: 2
                  type: piercing
            sting:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 12
              targets:
                kind: creature
                count: 1
                range_ft: 5
                visible: true
              failure:
                damage:
                - roll:
                    count: 2
                    sides: 4
                    modifier: 0
                  type: poison
                conditions:
                - condition: poisoned
                  duration_seconds: 3600
              failure_by_5_or_more:
                additional_while_poisoned:
                  condition: unconscious
                  ends_on:
                  - target.damage_taken
                  - event: creature.action
                    operation: wake_target
                    within_ft: 5
              success:
                effects: []
          skill_totals:
            perception: 5
            stealth: 4
          senses:
            blindsight: 10
            darkvision: 60
            passive_perception: 15
          languages:
            speaks: []
            understands:
            - common
            - draconic
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_pseudodragon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - damage.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_pseudodragon
    citations:
    - SRD 5.2.1 pp. 316; builtin/srd-5.2.1/creature/pseudodragon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/quasit:
    source_sha256: b27221103ad8f4095dc1389ab6efd89b9fb8069a031bd7b8941c979738794cd8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/quasit
        implementation: dnd2024:creature_quasit
        parameters:
          features:
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 4
                    modifier: 3
                  type: slashing
                conditions:
                - condition: poisoned
                  until: self.next_turn_start
            invisibility:
              economy: action
              operation: cast_spell
              spell:
                key: dnd2024:spells/invisibility
              ability: charisma
              targets:
              - self
              components_waived:
              - verbal
              - somatic
              - material
            scare:
              economy: action
              resource:
                kind: daily
                uses: 1
                recovery: long_rest
              operation: saving_throw
              ability: wisdom
              dc: 10
              targets:
                kind: creature
                count: 1
                range_ft: 20
                visible: false
              failure:
                conditions:
                - condition: frightened
                repeat_save:
                  event: target.turn_end
                  success: end_effect
                automatic_success_after_seconds: 60
              success:
                effects: []
            shape_shift:
              economy: action
              operation: shape_shift
              forms:
              - form: bat
                speed_ft: 10
                fly_speed_ft: 40
              - form: centipede
                speed_ft: 40
                climb_speed_ft: 40
              - form: toad
                speed_ft: 40
                swim_speed_ft: 40
              - form: true_form
              statistics_preserved_except:
              - speed
              equipment_transformed: false
          skill_totals:
            stealth: 5
          damage_resistances:
          - cold
          - fire
          - lightning
          damage_immunities:
          - poison
          condition_immunities:
          - poisoned
          senses:
            darkvision: 120
            passive_perception: 10
          languages:
            speaks:
            - abyssal
            - common
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_quasit
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - shape_shifting
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_quasit
    citations:
    - SRD 5.2.1 pp. 317; builtin/srd-5.2.1/creature/quasit.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/raven:
    source_sha256: 9bcfb09f1c3349f4c1304b7f96d555183be3a944006b44421aade3f57814038d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/raven
        implementation: dnd2024:creature_raven
        parameters:
          features:
            mimicry:
              economy: trait
              operation: passive_rule
              sounds:
                complexity: simple
                previously_heard: true
              detect_imitation:
                check: wisdom_insight
                dc: 10
            beak:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 1
                  roll:
                    constant: 1
                  type: piercing
          skill_totals:
            perception: 3
          senses:
            passive_perception: 13
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_raven
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_raven
    citations:
    - SRD 5.2.1 pp. 359; builtin/srd-5.2.1/creature/raven.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/remorhaz:
    source_sha256: f7ad51190f9e80faf909c9b3f4148536cd11756b310c1dcb84c82da4cac7e3c5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/remorhaz
        implementation: dnd2024:creature_remorhaz
        parameters:
          features:
            heat_aura:
              economy: trait
              operation: passive_rule
              trigger: self.turn_end
              targets:
                kind: creature
                selection: each
                area:
                  shape: emanation
                  size_ft: 5
                  origin: self
              damage:
              - roll:
                  count: 3
                  sides: 10
                  modifier: 0
                type: fire
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 11
                reach_ft: 10
              hit:
                damage:
                - average: 18
                  roll:
                    count: 2
                    sides: 10
                    modifier: 7
                  type: piercing
                - average: 14
                  roll:
                    count: 4
                    sides: 6
                    modifier: 0
                  type: fire
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 17
                  maximum_size: large
                  while_grappled:
                    conditions:
                    - restrained
            swallow:
              economy: bonus_action
              operation: saving_throw
              ability: strength
              dc: 19
              failure:
                end_grapple: true
                swallow:
                  maximum_size: large
                  conditions:
                  - restrained
                  - blinded
                  cover_against_outside: total
                  periodic_damage:
                    event: self.turn_start
                    damage:
                    - roll:
                        count: 3
                        sides: 6
                        modifier: 0
                      type: acid
                    - roll:
                        count: 3
                        sides: 6
                        modifier: 0
                      type: fire
                  regurgitate:
                    trigger: turn_end
                    if_damage_on_single_turn_from_one_inside_creature_at_least: 30
                    save:
                      ability: constitution
                      dc: 15
                      failure:
                        eject: all
                        within_ft: 5
                        conditions:
                        - prone
                  on_self_death:
                    remove_conditions:
                    - restrained
                    escape_movement_ft: 15
                    exit_condition: prone
              targets:
                kind: creature
                count: 1
                grappled_by: self
                maximum_size: large
              capacity: 2
              success:
                effects: []
          damage_immunities:
          - cold
          - fire
          condition_immunities: []
          senses:
            darkvision: 60
            tremorsense: 60
            passive_perception: 10
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_remorhaz
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_remorhaz
    citations:
    - SRD 5.2.1 pp. 319, 320; builtin/srd-5.2.1/creature/remorhaz.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/roc:
    source_sha256: 5cb4a09c07713f8a9bec3f03fcdb3210a70750a05c94f8a6648c4a747960997d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/roc
        implementation: dnd2024:creature_roc
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: beak
                count: 2
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: talons
            beak:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 13
                reach_ft: 10
              hit:
                damage:
                - average: 28
                  roll:
                    count: 3
                    sides: 12
                    modifier: 9
                  type: piercing
            talons:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 13
                reach_ft: 5
              hit:
                damage:
                - average: 23
                  roll:
                    count: 4
                    sides: 6
                    modifier: 9
                  type: slashing
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 19
                  maximum_size: huge
                  appendage_pool:
                    count: 2
                    type: talons
                    cost: 2
                  while_grappled:
                    conditions:
                    - restrained
            swoop:
              economy: bonus_action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: sequence
              requires:
                has_grappled_creature: true
              effects:
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: fly_speed
                  - 0.5
                provoke_opportunity_attacks: false
              - operation: drop_grappled_creature
          skill_totals:
            perception: 8
          senses:
            passive_perception: 18
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_roc
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_roc
    citations:
    - SRD 5.2.1 pp. 320; builtin/srd-5.2.1/creature/roc.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/saber_toothed_tiger:
    source_sha256: 5698a4f2e091a0d0a7cd6d217b1d4a22b8dadfe9e78dd9e375b8acb6ebd8ec3f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/saber_toothed_tiger
        implementation: dnd2024:creature_saber_toothed_tiger
        parameters:
          features:
            running_leap:
              economy: trait
              operation: passive_rule
              running_start_ft: 10
              long_jump_ft: 25
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 2
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 6
                    modifier: 4
                  type: slashing
            nimble_escape:
              economy: bonus_action
              operation: choose_action
              options:
              - disengage
              - hide
              count: 1
          skill_totals:
            perception: 5
            stealth: 7
          senses:
            darkvision: 60
            passive_perception: 15
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_saber_toothed_tiger
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_saber_toothed_tiger
    citations:
    - SRD 5.2.1 pp. 360; builtin/srd-5.2.1/creature/saber_toothed_tiger.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/satyr:
    source_sha256: 3eb592619d2d63437697aa070ca61cb83f91fd9c06904e46658c49f0de48c9f4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/satyr
        implementation: dnd2024:creature_satyr
        parameters:
          features:
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            hooves:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 4
                    modifier: 3
                  type: bludgeoning
                push:
                  maximum_ft: 10
                  direction: straight_away_from_self
                  maximum_size: medium
            mockery:
              economy: action
              operation: saving_throw
              ability: wisdom
              dc: 12
              targets:
                kind: creature
                count: 1
                range_ft: 90
                visible: true
              failure:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 6
                    modifier: 2
                  type: psychic
              success:
                effects: []
          skill_totals:
            perception: 2
            performance: 6
            stealth: 5
          senses:
            passive_perception: 12
          languages:
            speaks:
            - common
            - elvish
            - sylvan
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_satyr
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_satyr
    citations:
    - SRD 5.2.1 pp. 321, 322; builtin/srd-5.2.1/creature/satyr.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/sea_hag:
    source_sha256: 8094167d2c311998a0ad71180312acd7b9f150c96d28da189afdd6dfea64d59a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/sea_hag
        implementation: dnd2024:creature_sea_hag
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            coven_magic:
              economy: trait
              operation: spellcasting
              requires:
                hag_allies_within_ft: 30
                minimum_allies: 2
              ability: intelligence
              save_dc: 11
              components_waived:
              - material
              casting_time: spell_normal
              options:
              - spell:
                  key: dnd2024:spells/augury
                usage:
                  kind: rest
                  uses: 1
                  recovery: long_rest
                  pool: per_spell
              - spell:
                  key: dnd2024:spells/find_familiar
                usage:
                  kind: rest
                  uses: 1
                  recovery: long_rest
                  pool: per_spell
              - spell:
                  key: dnd2024:spells/identify
                usage:
                  kind: rest
                  uses: 1
                  recovery: long_rest
                  pool: per_spell
              - spell:
                  key: dnd2024:spells/locate_object
                usage:
                  kind: rest
                  uses: 1
                  recovery: long_rest
                  pool: per_spell
              - spell:
                  key: dnd2024:spells/scrying
                usage:
                  kind: rest
                  uses: 1
                  recovery: long_rest
                  pool: per_spell
              - spell:
                  key: dnd2024:spells/unseen_servant
                usage:
                  kind: rest
                  uses: 1
                  recovery: long_rest
                  pool: per_spell
              material_components_available: true
            vile_appearance:
              economy: trait
              operation: saving_throw
              ability: wisdom
              dc: 11
              targets:
                kind: creature
                count: 1
                range_ft: 30
                types:
                - beast
                - humanoid
                can_see_true_form: true
              failure:
                conditions:
                - condition: frightened
                  until: target.next_turn_start
              trigger: target.turn_start
              success:
                immunity:
                  effect: this_vile_appearance
                  duration_seconds: 86400
            claw:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 10
                  roll:
                    count: 2
                    sides: 6
                    modifier: 3
                  type: slashing
            death_glare:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: wisdom
              dc: 11
              targets:
                kind: creature
                count: 1
                range_ft: 30
                visible: true
                required_conditions:
                - frightened
              failure:
                if:
                  target_current_hp_at_most: 20
                then:
                  set_hp: 0
                else:
                  damage:
                  - roll:
                      count: 3
                      sides: 8
                      modifier: 0
                    type: psychic
              success:
                effects: []
            illusory_appearance:
              economy: action
              operation: cast_spell
              spell:
                key: dnd2024:spells/disguise_self
              ability: constitution
              save_dc: 13
              duration_seconds: 86400
          senses:
            darkvision: 60
            passive_perception: 11
          languages:
            speaks:
            - common
            - giant
            - language: primordial
              dialects:
              - aquan
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_sea_hag
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_sea_hag
    citations:
    - SRD 5.2.1 pp. 322; builtin/srd-5.2.1/creature/sea_hag.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/shambling_mound:
    source_sha256: 0d9a9fda80c032071eabbb837d6592b7ee6d136deab9f2c9c2f4db2e683bfe94
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/shambling_mound
        implementation: dnd2024:creature_shambling_mound
        parameters:
          features:
            lightning_absorption:
              economy: trait
              operation: passive_rule
              trigger: self.subjected_to_lightning_damage
              heal:
                equal: lightning_damage_dealt
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: charged_tendril
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: engulf
            charged_tendril:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 10
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 6
                    modifier: 4
                  type: bludgeoning
                - average: 5
                  roll:
                    count: 2
                    sides: 4
                    modifier: 0
                  type: lightning
                pull:
                  maximum_ft: 5
                  direction: straight_toward_self
                  maximum_size: medium
            engulf:
              economy: action
              operation: saving_throw
              ability: strength
              dc: 15
              targets:
                kind: creature
                count: 1
                range_ft: 5
                maximum_size: medium
              failure:
                move_target_to: self.space
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 14
                  while_grappled:
                    conditions:
                    - blinded
                    - restrained
                    damage_each_target_turn_start:
                    - roll:
                        count: 3
                        sides: 6
                        modifier: 0
                      type: lightning
                    moves_with_self: true
                    movement_extra_cost: 0
              capacity: 1
              success:
                effects: []
          skill_totals:
            stealth: 3
          damage_resistances:
          - cold
          - fire
          damage_immunities:
          - lightning
          condition_immunities:
          - deafened
          - exhaustion
          senses:
            blindsight: 60
            passive_perception: 10
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_shambling_mound
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_shambling_mound
    citations:
    - SRD 5.2.1 pp. 323; builtin/srd-5.2.1/creature/shambling_mound.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/silver_dragon_wyrmling:
    source_sha256: 652a268660b955fc2fa9888c39fc04605a0344b8d91aa8eeedf5e1aab077ae0e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/silver_dragon_wyrmling
        implementation: dnd2024:creature_silver_dragon_wyrmling
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 2
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 9
                  roll:
                    count: 1
                    sides: 10
                    modifier: 4
                  type: piercing
            cold_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: constitution
              dc: 13
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 15
                  origin: self
              failure:
                damage:
                - average: 18
                  roll:
                    count: 4
                    sides: 8
                    modifier: 0
                  type: cold
              success:
                damage_multiplier: 0.5
                other_effects: false
            paralyzing_breath:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 13
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 15
                  origin: self
              failure:
                damage: []
                conditions:
                - condition: incapacitated
                  until: target.next_turn_end
                repeat_save:
                  event: target.next_turn_end
                  failure:
                    conditions:
                    - condition: paralyzed
                    repeat_save:
                      event: target.turn_end
                      success: end_effect
                    automatic_success_after_seconds: 60
                    automatic_success_clock_origin: initial_failed_save
                  success: end_effect
              success:
                effects: []
          skill_totals:
            perception: 4
            stealth: 2
          damage_immunities:
          - cold
          condition_immunities: []
          senses:
            blindsight: 10
            darkvision: 60
            passive_perception: 14
          languages:
            speaks:
            - draconic
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_silver_dragon_wyrmling
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_silver_dragon_wyrmling
    citations:
    - SRD 5.2.1 pp. 324; builtin/srd-5.2.1/creature/silver_dragon_wyrmling.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/specter:
    source_sha256: 1dd786689cabd2c0bd4eab257dda9eab888372d5bb231ae1186d142e70211ef3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/specter
        implementation: dnd2024:creature_specter
        parameters:
          features:
            incorporeal_movement:
              economy: trait
              operation: passive_rule
              pass_through:
              - creatures
              - objects
              movement_terrain: difficult
              end_turn_inside_object_damage:
                roll:
                  count: 1
                  sides: 10
                  modifier: 0
                type: force
            sunlight_sensitivity:
              economy: trait
              operation: passive_rule
              while:
                illumination: sunlight
              disadvantage_on:
              - ability_checks
              - attack_rolls
            life_drain:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 2
                    sides: 6
                    modifier: 0
                  type: necrotic
                maximum_hp_reduction:
                  requires_target_kind: creature
                  equal: damage_taken
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
          - grappled
          - paralyzed
          - petrified
          - poisoned
          - prone
          - restrained
          - unconscious
          senses:
            darkvision: 60
            passive_perception: 10
          languages:
            speaks: []
            understands:
            - common
            additional_languages:
              count: 1
              chooser: creator
              can_speak: false
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_specter
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - damage.resolved
      - attack.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_specter
    citations:
    - SRD 5.2.1 pp. 327; builtin/srd-5.2.1/creature/specter.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/sphinx_of_wonder:
    source_sha256: 3573b2d84d441af30f367ed58dba9c351a91b27845bf18083ff43628b12dada8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/sphinx_of_wonder
        implementation: dnd2024:creature_sphinx_of_wonder
        parameters:
          features:
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 4
                    modifier: 3
                  type: slashing
                - average: 7
                  roll:
                    count: 2
                    sides: 6
                    modifier: 0
                  type: radiant
            burst_of_ingenuity:
              economy: reaction
              resource:
                kind: daily
                uses: 2
                recovery: long_rest
              operation: modify_roll
              trigger: ability_check_or_saving_throw
              targets:
                any_of:
                - self: true
                - within_ft: 30
              bonus: 2
          skill_totals:
            arcana: 4
            religion: 4
            stealth: 5
          damage_resistances:
          - necrotic
          - psychic
          - radiant
          senses:
            darkvision: 60
            passive_perception: 11
          languages:
            speaks:
            - celestial
            - common
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_sphinx_of_wonder
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_sphinx_of_wonder
    citations:
    - SRD 5.2.1 pp. 327; builtin/srd-5.2.1/creature/sphinx_of_wonder.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/sprite:
    source_sha256: bb8f0091d22a12c184658dd7caf99155db3c7f76dda1aa0ad69f8d8bf0d98b37
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/sprite
        implementation: dnd2024:creature_sprite
        parameters:
          features:
            needle_sword:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 1
                    sides: 4
                    modifier: 4
                  type: piercing
            enchanting_bow:
              economy: action
              operation: attack
              attack:
                mode: ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                range_ft: 40
                long_range_ft: 160
              hit:
                damage:
                - average: 1
                  roll:
                    constant: 1
                  type: piercing
                conditions:
                - condition: charmed
                  until: self.next_turn_start
            heart_sight:
              economy: action
              operation: saving_throw
              ability: charisma
              dc: 10
              targets:
                kind: creature
                count: 1
                range_ft: 5
                visible: true
              failure:
                learn:
                - target.emotions
                - target.alignment
              automatic_failure_types:
              - celestial
              - fiend
              - undead
              success:
                effects: []
            invisibility:
              economy: action
              operation: cast_spell
              spell:
                key: dnd2024:spells/invisibility
              ability: charisma
              targets:
              - self
              components_waived:
              - verbal
              - somatic
              - material
          skill_totals:
            perception: 3
            stealth: 8
          senses:
            passive_perception: 13
          languages:
            speaks:
            - common
            - elvish
            - sylvan
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_sprite
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_sprite
    citations:
    - SRD 5.2.1 pp. 329; builtin/srd-5.2.1/creature/sprite.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/stirge:
    source_sha256: 2f3a2f11e2d6b090ae84edf4b4aab4b76b9d09375acb8dd1214d68e8a37930b0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/stirge
        implementation: dnd2024:creature_stirge
        parameters:
          features:
            proboscis:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 6
                  roll:
                    count: 1
                    sides: 6
                    modifier: 3
                  type: piercing
                attach:
                  target: hit_target
                  while_attached:
                    can_use_proboscis: false
                    damage_each_self_turn_start:
                    - roll:
                        count: 2
                        sides: 4
                        modifier: 0
                      type: necrotic
                  detach_self:
                    movement_cost_ft: 5
                  detach_other:
                    economy: action
                    actor_any_of:
                    - target
                    - within_target_ft: 5
          senses:
            darkvision: 60
            passive_perception: 9
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_stirge
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_stirge
    citations:
    - SRD 5.2.1 pp. 329, 330; builtin/srd-5.2.1/creature/stirge.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/storm_giant:
    source_sha256: fd9a8fc80692d974addc411a265737b1111b0472f219fd6e95f4afd8e4560a24
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/storm_giant
        implementation: dnd2024:creature_storm_giant
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - storm_sword
                - thunderbolt
            storm_sword:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 14
                reach_ft: 10
              hit:
                damage:
                - average: 23
                  roll:
                    count: 4
                    sides: 6
                    modifier: 9
                  type: slashing
                - average: 13
                  roll:
                    count: 3
                    sides: 8
                    modifier: 0
                  type: lightning
            thunderbolt:
              economy: action
              operation: attack
              attack:
                mode: ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 14
                range_ft: 500
                long_range_ft: null
              hit:
                damage:
                - average: 22
                  roll:
                    count: 2
                    sides: 12
                    modifier: 9
                  type: lightning
                conditions:
                - condition: blinded
                  until: self.next_turn_start
                - condition: deafened
                  until: self.next_turn_start
            lightning_storm:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 18
              targets:
                kind: creature
                selection: each
                area:
                  shape: cylinder
                  radius_ft: 10
                  height_ft: 40
                  origin:
                    visible: true
                    range_ft: 500
              failure:
                damage:
                - average: 55
                  roll:
                    count: 10
                    sides: 10
                    modifier: 0
                  type: lightning
              success:
                damage_multiplier: 0.5
                other_effects: false
            spellcasting:
              economy: action
              operation: spellcasting
              ability: wisdom
              components_waived:
              - material
              save_dc: 18
              options:
              - spell:
                  key: dnd2024:spells/detect_magic
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/light
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/control_weather
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
          skill_totals:
            arcana: 8
            athletics: 14
            history: 8
            perception: 10
          damage_resistances:
          - cold
          damage_immunities:
          - lightning
          - thunder
          condition_immunities: []
          senses:
            darkvision: 120
            truesight: 30
            passive_perception: 20
          languages:
            speaks:
            - common
            - giant
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_storm_giant
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_storm_giant
    citations:
    - SRD 5.2.1 pp. 330, 331; builtin/srd-5.2.1/creature/storm_giant.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/swarm_of_crawling_claws:
    source_sha256: d13680aa681393847cc55669b68488d71445e2384606ce9ea0647c88bc079ca0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/swarm_of_crawling_claws
        implementation: dnd2024:creature_swarm_of_crawling_claws
        parameters:
          features:
            swarm:
              economy: trait
              operation: passive_rule
              share_creature_spaces: true
              minimum_opening_size: tiny
              can_regain_hp: false
              can_gain_temporary_hp: false
            swarm_of_grasping_hands:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                  if:
                    self_bloodied: true
                  then:
                  - average: 11
                    roll:
                      count: 2
                      sides: 8
                      modifier: 2
                    type: necrotic
                  else:
                  - average: 20
                    roll:
                      count: 4
                      sides: 8
                      modifier: 2
                    type: necrotic
                prone:
                  condition: prone
                  maximum_size: medium
          damage_resistances:
          - bludgeoning
          - piercing
          - slashing
          damage_immunities:
          - necrotic
          - poison
          condition_immunities:
          - charmed
          - exhaustion
          - frightened
          - grappled
          - incapacitated
          - paralyzed
          - petrified
          - poisoned
          - prone
          - restrained
          - stunned
          senses:
            blindsight: 30
            passive_perception: 10
          languages:
            speaks: []
            understands:
            - common
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_swarm_of_crawling_claws
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_swarm_of_crawling_claws
    citations:
    - SRD 5.2.1 pp. 278; builtin/srd-5.2.1/creature/swarm_of_crawling_claws.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/swarm_of_rats:
    source_sha256: c1985640373f89baaa9025e0b13f7d802709d1e7768a9d4acf748b65f84a1a88
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/swarm_of_rats
        implementation: dnd2024:creature_swarm_of_rats
        parameters:
          features:
            swarm:
              economy: trait
              operation: passive_rule
              share_creature_spaces: true
              minimum_opening_size: tiny
              can_regain_hp: false
              can_gain_temporary_hp: false
            bites:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 2
                reach_ft: 5
              hit:
                damage:
                  if:
                    self_bloodied: true
                  then:
                  - average: 2
                    roll:
                      count: 1
                      sides: 4
                      modifier: 0
                    type: piercing
                  else:
                  - average: 5
                    roll:
                      count: 2
                      sides: 4
                      modifier: 0
                    type: piercing
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
            darkvision: 30
            passive_perception: 10
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_swarm_of_rats
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_swarm_of_rats
    citations:
    - SRD 5.2.1 pp. 362; builtin/srd-5.2.1/creature/swarm_of_rats.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/tarrasque:
    source_sha256: 759cd55b623b7a2e5a94b331478cd5f0a874ca19196aebfd3c04d955399bbaf7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/tarrasque
        implementation: dnd2024:creature_tarrasque
        parameters:
          features:
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 6
                recovery: long_rest
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            reflective_carapace:
              economy: trait
              operation: passive_rule
              trigger: self.targeted_by_spell
              spell_eligibility:
                any_of:
                - spell:
                    key: dnd2024:spells/magic_missile
                - requires_ranged_attack_roll: true
              roll:
                die: 6
                results:
                - faces:
                  - 1
                  - 2
                  - 3
                  - 4
                  - 5
                  self_affected: false
                - faces:
                  - 6
                  self_affected: false
                  reflect_to: caster
            siege_monster:
              economy: trait
              operation: passive_rule
              damage_multiplier: 2
              target_kinds:
              - object
              - structure
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: bite
                count: 1
              - operation: attack_choice
                count: 3
                options:
                - claw
                - tail
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 19
                reach_ft: 15
              hit:
                damage:
                - average: 36
                  roll:
                    count: 4
                    sides: 12
                    modifier: 10
                  type: piercing
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 20
                  while_grappled:
                    conditions:
                    - restrained
                    can_teleport: false
            claw:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 19
                reach_ft: 15
              hit:
                damage:
                - average: 28
                  roll:
                    count: 4
                    sides: 8
                    modifier: 10
                  type: slashing
            tail:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 19
                reach_ft: 30
              hit:
                damage:
                - average: 23
                  roll:
                    count: 3
                    sides: 8
                    modifier: 10
                  type: bludgeoning
                prone:
                  condition: prone
                  maximum_size: huge
            thunderous_bellow:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: constitution
              dc: 27
              targets:
                kinds:
                - creature
                - object
                objects_exclude:
                - worn
                - carried
                selection: each
                area:
                  shape: cone
                  size_ft: 150
                  origin: self
              failure:
                damage:
                - average: 78
                  roll:
                    count: 12
                    sides: 12
                    modifier: 0
                  type: thunder
                conditions:
                - condition: deafened
                  until: target.next_turn_end
                - condition: frightened
                  until: target.next_turn_end
              success:
                damage_multiplier: 0.5
                other_effects: false
            swallow:
              economy: bonus_action
              operation: saving_throw
              ability: strength
              dc: 27
              failure:
                end_grapple: true
                swallow:
                  maximum_size: large
                  conditions:
                  - restrained
                  - blinded
                  cover_against_outside: total
                  periodic_damage:
                    event: self.turn_start
                    damage:
                    - roll:
                        count: 16
                        sides: 6
                        modifier: 0
                      type: acid
                  regurgitate:
                    trigger: turn_end
                    if_damage_on_single_turn_from_one_inside_creature_at_least: 60
                    save:
                      ability: constitution
                      dc: 20
                      failure:
                        eject: all
                        within_ft: 10
                        conditions:
                        - prone
                  on_self_death:
                    remove_conditions:
                    - restrained
                    escape_movement_ft: 20
                    exit_condition: prone
                  can_teleport: false
              targets:
                kind: creature
                count: 1
                grappled_by: self
                maximum_size: large
              capacity: 6
              success:
                effects: []
            onslaught:
              economy: legendary_action
              legendary_cost: 1
              operation: sequence
              effects:
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: speed
                  - 0.5
              - operation: attack_choice
                count: 1
                options:
                - claw
                - tail
            world_shaking_movement:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: sequence
              effects:
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: speed
                  - 1
              - operation: shock_wave
                area:
                  shape: emanation
                  size_ft: 60
                  origin: self
                duration: instantaneous
                targets: creatures
                end_concentration: true
                prone:
                  maximum_size: medium
          skill_totals:
            perception: 9
          damage_resistances:
          - bludgeoning
          - piercing
          - slashing
          damage_immunities:
          - fire
          - poison
          condition_immunities:
          - charmed
          - deafened
          - frightened
          - paralyzed
          - poisoned
          senses:
            blindsight: 120
            passive_perception: 19
          languages:
            speaks: []
            understands: []
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_tarrasque
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - rest.completed
      - saving_throw.resolved
      - attack.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - teleportation
      - spatial_queries
      - limited_resources
      - legendary_actions
      - defenses
    requires:
    - dnd2024:creature_tarrasque
    citations:
    - SRD 5.2.1 pp. 331, 332; builtin/srd-5.2.1/creature/tarrasque.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/tough_boss:
    source_sha256: 6113d391b859a97ce0d6c6bacd1dec411db39a2b23adcc49e259364944d4c12c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/tough_boss
        implementation: dnd2024:creature_tough_boss
        parameters:
          features:
            pack_tactics:
              economy: trait
              operation: passive_rule
              attack_advantage:
                target_kind: creature
                ally_within_target_ft: 5
                ally_excludes_conditions:
                - incapacitated
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - warhammer
                - heavy_crossbow
            warhammer:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 12
                  roll:
                    count: 2
                    sides: 8
                    modifier: 3
                  type: bludgeoning
                push:
                  maximum_ft: 10
                  direction: straight_away_from_self
                  maximum_size: large
            heavy_crossbow:
              economy: action
              operation: attack
              attack:
                mode: ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                range_ft: 100
                long_range_ft: 400
              hit:
                damage:
                - average: 13
                  roll:
                    count: 2
                    sides: 10
                    modifier: 2
                  type: piercing
          gear:
          - item: chain_mail
            quantity: 1
          - item: heavy_crossbow
            quantity: 1
          - item: warhammer
            quantity: 1
          senses:
            passive_perception: 10
          languages:
            speaks:
            - common
            understands: []
            additional_languages:
              count: 1
              chooser: creator
              can_speak: true
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_tough_boss
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_tough_boss
    citations:
    - SRD 5.2.1 pp. 332; builtin/srd-5.2.1/creature/tough_boss.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/troll:
    source_sha256: 54b83137757b3fab86e7d849fc92a7c5b04f6ec2efdc8848cda4a8094da8a617
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/troll
        implementation: dnd2024:creature_troll
        parameters:
          features:
            loathsome_limbs:
              economy: trait
              resource:
                kind: daily
                uses: 4
                recovery: long_rest
              operation: passive_rule
              trigger: any.turn_end
              requires:
                self_bloodied: true
                slashing_damage_this_turn_at_least: 15
              sever_limb:
                spawn:
                  key: dnd2024:creatures/troll_limb
                location: self.space
                turn: immediately_after_self
              exhaustion_per_missing_limb: 1
              regrow:
                event: self.hp_regained
                all_missing: true
            regeneration:
              economy: trait
              operation: passive_rule
              trigger: self.turn_start
              heal: 15
              suppressed_next_turn_by:
              - acid_damage
              - fire_damage
              death_rule:
                only_if:
                  turn_start_hp: 0
                  does_not_regenerate: true
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 3
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 10
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 6
                    modifier: 4
                  type: slashing
            charge:
              economy: bonus_action
              operation: move
              maximum_ft:
                multiply:
                - stat: speed
                - 0.5
              direction: straight_toward_enemy
              enemy_visible: true
          skill_totals:
            perception: 5
          senses:
            darkvision: 60
            passive_perception: 15
          languages:
            speaks:
            - giant
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_troll
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - creature_transformation
      - spatial_queries
      - limited_resources
      - hit_points
    requires:
    - dnd2024:creature_troll
    citations:
    - SRD 5.2.1 pp. 333; builtin/srd-5.2.1/creature/troll.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/unicorn:
    source_sha256: 658418bf0221107906b3b40bfb34abe0ad4ff1e43e2f84a3f975d86501c4feed
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/unicorn
        implementation: dnd2024:creature_unicorn
        parameters:
          features:
            legendary_resistance:
              economy: trait
              resource:
                kind: daily
                uses: 3
                recovery: long_rest
              operation: replace_save_result
              trigger: self.save_failed
              optional: true
              result: success
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: hooves
                count: 1
              - operation: attack_sequence
                feature: radiant_horn
                count: 1
            hooves:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 5
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 6
                    modifier: 4
                  type: bludgeoning
            radiant_horn:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 5
              hit:
                damage:
                - average: 9
                  roll:
                    count: 1
                    sides: 10
                    modifier: 4
                  type: radiant
            spellcasting:
              economy: action
              operation: spellcasting
              ability: charisma
              components_waived:
              - verbal
              - somatic
              - material
              save_dc: 14
              options:
              - spell:
                  key: dnd2024:spells/detect_evil_and_good
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/druidcraft
                usage:
                  kind: at_will
              - spell:
                  key: dnd2024:spells/calm_emotions
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/dispel_evil_and_good
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/entangle
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/pass_without_trace
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              - spell:
                  key: dnd2024:spells/word_of_recall
                usage:
                  kind: daily
                  uses: 1
                  pool: per_spell
                  recovery: long_rest
              material_components_available: true
            unicorn_s_blessing:
              economy: bonus_action
              resource:
                kind: daily
                uses: 3
                recovery: long_rest
              operation: spell_choice
              options:
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/cure_wounds
              - operation: cast_spell
                spell:
                  key: dnd2024:spells/lesser_restoration
              ability: charisma
              targets:
                another_creature: true
                touch_with_horn: true
            charging_horn:
              economy: legendary_action
              legendary_cost: 1
              operation: sequence
              effects:
              - operation: move
                maximum_ft:
                  multiply:
                  - stat: speed
                  - 0.5
                provoke_opportunity_attacks: false
              - operation: attack_sequence
                feature: radiant_horn
                count: 1
            shimmering_shield:
              economy: legendary_action
              legendary_cost: 1
              cooldown_until: self.next_turn_start
              operation: grant_defense
              targets:
                any_of:
                - self: true
                - kind: creature
                  count: 1
                  range_ft: 60
                  visible: true
              temporary_hp:
                roll:
                  count: 3
                  sides: 6
                  modifier: 0
                average: 10
              ac_bonus: 2
              until: self.next_turn_end
          damage_immunities:
          - poison
          condition_immunities:
          - charmed
          - paralyzed
          - poisoned
          senses:
            darkvision: 60
            passive_perception: 13
          languages:
            speaks:
            - celestial
            - elvish
            - sylvan
            understands: []
            telepathy_range_ft: 120
          legendary_actions:
            uses: 3
            recovery: self.turn_start
            trigger: other_creature.turn_end
            cost_per_action: 1
            requires:
              not_incapacitated: true
            maximum_actions_per_trigger: 1
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_unicorn
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - rest.completed
      - saving_throw.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spellcasting
      - spatial_queries
      - limited_resources
      - legendary_actions
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_unicorn
    citations:
    - SRD 5.2.1 pp. 334; builtin/srd-5.2.1/creature/unicorn.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/vampire_spawn:
    source_sha256: f9f400ac33325d560cd4618fd473cdf010dc7b73edeffe1d0bba17f366d46128
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/vampire_spawn
        implementation: dnd2024:creature_vampire_spawn
        parameters:
          features:
            spider_climb:
              economy: trait
              operation: passive_rule
              climb_difficult_surfaces: true
              climb_ceilings: true
              ability_check_required: false
            vampire_weakness:
              economy: trait
              operation: passive_rule
              weaknesses:
              - forbiddance
              - running_water
              - stake_to_the_heart
              - sunlight
            forbiddance:
              economy: trait
              operation: passive_rule
              residence_entry_requires: occupant_invitation
            running_water:
              economy: trait
              operation: passive_rule
              trigger: self.turn_end
              requires:
                in_running_water: true
              damage:
              - roll:
                  constant: 20
                type: acid
            stake_to_the_heart:
              economy: trait
              operation: passive_rule
              trigger: piercing_weapon_driven_into_heart
              requires:
                self_condition: incapacitated
              destroy_self: true
            sunlight:
              economy: trait
              operation: passive_rule
              trigger: self.turn_start
              requires:
                in_sunlight: true
              damage:
              - roll:
                  constant: 20
                type: radiant
              while_sunlight:
                disadvantage_on:
                - attack_rolls
                - ability_checks
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: claw
                count: 2
              - operation: use_feature
                feature: bite
            claw:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 8
                  roll:
                    count: 2
                    sides: 4
                    modifier: 3
                  type: slashing
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 13
                  maximum_size: medium
                  appendage_pool:
                    count: 2
                    type: claws
                    cost: 1
            bite:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 14
              targets:
                kind: creature
                count: 1
                range_ft: 5
                any_of:
                - willing: true
                - condition: grappled
                - condition: incapacitated
                - condition: restrained
              failure:
                damage:
                - average: 5
                  roll:
                    count: 1
                    sides: 4
                    modifier: 3
                  type: piercing
                - average: 10
                  roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  type: necrotic
                maximum_hp_reduction:
                  equal: necrotic_damage_taken
                heal_self:
                  equal: necrotic_damage_taken
              success:
                effects: []
            deathless_agility:
              economy: bonus_action
              operation: choose_action
              count: 1
              options:
              - dash
              - disengage
          skill_totals:
            perception: 3
            stealth: 6
          damage_resistances:
          - necrotic
          senses:
            darkvision: 60
            passive_perception: 13
          languages:
            speaks:
            - common
            understands: []
            additional_languages:
              count: 1
              chooser: creator
              can_speak: true
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_vampire_spawn
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - damage.resolved
      - attack.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - hit_points
    requires:
    - dnd2024:creature_vampire_spawn
    citations:
    - SRD 5.2.1 pp. 334, 335; builtin/srd-5.2.1/creature/vampire_spawn.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/vrock:
    source_sha256: ef91f29f0f92a67c29de69f6ecf8b8572ccc6e9478b95e8a1040f16576454f08
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/vrock
        implementation: dnd2024:creature_vrock
        parameters:
          features:
            demonic_restoration:
              economy: trait
              operation: passive_rule
              trigger: self.dies
              unless:
                plane: abyss
              remove_old_body: true
              new_body:
                delay_seconds: 0
                plane: abyss
                hp: maximum
            magic_resistance:
              economy: trait
              operation: passive_rule
              saving_throw_advantage_against:
              - spells
              - magical_effects
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: shred
                count: 2
            shred:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 10
                  roll:
                    count: 2
                    sides: 6
                    modifier: 3
                  type: piercing
                - average: 10
                  roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  type: poison
            spores:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: constitution
              dc: 15
              targets:
                kind: creature
                selection: each
                area:
                  shape: emanation
                  size_ft: 20
                  origin: self
              failure:
                conditions:
                - condition: poisoned
                repeat_save:
                  event: target.turn_end
                  success: end_effect
                while_poisoned:
                  damage_each_target_turn_start:
                  - roll:
                      count: 1
                      sides: 10
                      modifier: 0
                    type: poison
                ends_on:
                  operation: empty_holy_water_flask_on_target
              success:
                effects: []
            stunning_screech:
              economy: action
              resource:
                kind: daily
                uses: 1
                recovery: long_rest
              operation: saving_throw
              ability: constitution
              dc: 15
              targets:
                kind: creature
                selection: each
                area:
                  shape: emanation
                  size_ft: 20
                  origin: self
              failure:
                damage:
                - average: 10
                  roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  type: thunder
                conditions:
                - condition: stunned
                  until: self.next_turn_end
              automatic_success_tags:
              - demon
              success:
                effects: []
          damage_resistances:
          - cold
          - fire
          - lightning
          damage_immunities:
          - poison
          condition_immunities:
          - poisoned
          senses:
            darkvision: 120
            passive_perception: 11
          languages:
            speaks:
            - abyssal
            understands: []
            telepathy_range_ft: 120
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_vrock
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - creature.died
      - rest.completed
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_vrock
    citations:
    - SRD 5.2.1 pp. 336; builtin/srd-5.2.1/creature/vrock.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/warhorse_skeleton:
    source_sha256: acd63d1f7a665fcc16869590b72d3b6747f0bb5b59c38669da8be8ba237978be
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/warhorse_skeleton
        implementation: dnd2024:creature_warhorse_skeleton
        parameters:
          features:
            hooves:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 6
                    modifier: 4
                  type: bludgeoning
                prone:
                  condition: prone
                  maximum_size: large
                  requires:
                    immediately_preceding_straight_movement_toward_target_ft: 20
          damage_vulnerabilities:
          - bludgeoning
          damage_immunities:
          - poison
          condition_immunities:
          - exhaustion
          - poisoned
          senses:
            darkvision: 60
            passive_perception: 9
          languages:
            speaks: []
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_warhorse_skeleton
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_warhorse_skeleton
    citations:
    - SRD 5.2.1 pp. 326; builtin/srd-5.2.1/creature/warhorse_skeleton.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/water_elemental:
    source_sha256: 4a7c132b4030e881fae195abc0d696be735d614bc167aafbe3eaeb0f51c0929a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/water_elemental
        implementation: dnd2024:creature_water_elemental
        parameters:
          features:
            freeze:
              economy: trait
              operation: passive_rule
              trigger: self.cold_damage_taken
              speed_modifier_ft: -20
              until: self.next_turn_end
            water_form:
              economy: trait
              operation: passive_rule
              occupy_enemy_space: true
              minimum_gap_inches: 1
              extra_movement_cost: 0
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: slam
                count: 2
            slam:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 5
              hit:
                damage:
                - average: 13
                  roll:
                    count: 2
                    sides: 8
                    modifier: 4
                  type: bludgeoning
                prone:
                  condition: prone
                  maximum_size: medium
            whelm:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 4
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: strength
              dc: 15
              targets:
                kind: creature
                selection: each
                location: self.space
              failure:
                damage:
                - roll:
                    count: 4
                    sides: 8
                    modifier: 4
                  type: bludgeoning
                grapple:
                  condition: grappled
                  source: self
                  escape_dc: 14
                  maximum_size: large
                  while_grappled:
                    conditions:
                    - restrained
                    suffocating_unless_water_breathing: true
                    damage_each_self_turn_start:
                    - roll:
                        count: 2
                        sides: 8
                        modifier: 0
                      type: bludgeoning
              success:
                damage_multiplier: 0.5
                other_effects: false
              capacity:
                choose_one:
                - size: large
                  count: 1
                - maximum_size: medium
                  count: 2
              pull_out:
                economy: action
                actor_within_ft: 5
                check: strength_athletics
                dc: 14
          damage_resistances:
          - acid
          - fire
          damage_immunities:
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
            darkvision: 60
            passive_perception: 10
          languages:
            speaks:
            - language: primordial
              dialects:
              - aquan
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_water_elemental
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - damage.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_water_elemental
    citations:
    - SRD 5.2.1 pp. 337; builtin/srd-5.2.1/creature/water_elemental.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/wereboar:
    source_sha256: 9a736e0e8c550036ee9d30a83dbfa68fe4a1f91be4ec4dc9788b6fb0f3b05d1b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/wereboar
        implementation: dnd2024:creature_wereboar
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - javelin
                - tusk
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: gore
            gore:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 12
                  roll:
                    count: 2
                    sides: 8
                    modifier: 3
                  type: piercing
                save:
                  operation: saving_throw
                  ability: constitution
                  dc: 12
                  targets:
                    kind: humanoid
                  failure:
                    curse:
                      on_target_zero_hp:
                        replace:
                          definition:
                            key: dnd2024:creatures/wereboar
                          controller: gm
                          hp: 10
                  success:
                    immunity:
                      effect: this_creature_curse
                      duration_seconds: 86400
              allowed_forms:
              - boar
              - hybrid
            javelin:
              economy: action
              operation: attack
              attack:
                mode: melee_or_ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
                range_ft: 30
                long_range_ft: 120
              hit:
                damage:
                - average: 13
                  roll:
                    count: 3
                    sides: 6
                    modifier: 3
                  type: piercing
              allowed_forms:
              - humanoid
              - hybrid
            tusk:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 10
                  roll:
                    count: 2
                    sides: 6
                    modifier: 3
                  type: piercing
                prone:
                  condition: prone
                  maximum_size: medium
                  requires:
                    immediately_preceding_straight_movement_toward_target_ft: 20
                conditional_damage:
                  requires:
                    maximum_target_size: medium
                    immediately_preceding_straight_movement_toward_target_ft: 20
                  damage:
                  - roll:
                      count: 2
                      sides: 6
                      modifier: 0
                    type: piercing
              allowed_forms:
              - boar
              - hybrid
            shape_shift:
              economy: bonus_action
              operation: shape_shift
              forms:
              - form: hybrid
                size: medium
              - form: boar
                size: small
              - form: true_humanoid
                size: original
              statistics_preserved_except:
              - size
              - form_specific_speed
              equipment_transformed: false
          skill_totals:
            perception: 2
          gear:
          - item: javelins
            quantity: 6
          senses:
            passive_perception: 12
          languages:
            speaks:
            - common
            understands: []
            cannot_speak_forms:
            - boar
          form_rules:
            beast:
              speed_ft: 40
              can_speak: false
            true_humanoid:
              speed_ft: 30
            hybrid:
              speed_ft: 30
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_wereboar
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - shape_shifting
      - spatial_queries
      - limited_resources
      - curses
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_wereboar
    citations:
    - SRD 5.2.1 pp. 338; builtin/srd-5.2.1/creature/wereboar.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/werewolf:
    source_sha256: 268b4957114fa967643d7c40bd0970b638b61f87e00e555aebc4cacfc596ed27
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/werewolf
        implementation: dnd2024:creature_werewolf
        parameters:
          features:
            pack_tactics:
              economy: trait
              operation: passive_rule
              attack_advantage:
                target_kind: creature
                ally_within_target_ft: 5
                ally_excludes_conditions:
                - incapacitated
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_choice
                count: 2
                options:
                - scratch
                - longbow
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: bite
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 12
                  roll:
                    count: 2
                    sides: 8
                    modifier: 3
                  type: piercing
                save:
                  operation: saving_throw
                  ability: constitution
                  dc: 12
                  targets:
                    kind: humanoid
                  failure:
                    curse:
                      on_target_zero_hp:
                        replace:
                          definition:
                            key: dnd2024:creatures/werewolf
                          controller: gm
                          hp: 10
                  success:
                    immunity:
                      effect: this_creature_curse
                      duration_seconds: 86400
              allowed_forms:
              - wolf
              - hybrid
            scratch:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 10
                  roll:
                    count: 2
                    sides: 6
                    modifier: 3
                  type: slashing
            longbow:
              economy: action
              operation: attack
              attack:
                mode: ranged
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                range_ft: 150
                long_range_ft: 600
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 8
                    modifier: 2
                  type: piercing
              allowed_forms:
              - humanoid
              - hybrid
            shape_shift:
              economy: bonus_action
              operation: shape_shift
              forms:
              - form: hybrid
                size: large
              - form: wolf
                size: medium
              - form: true_humanoid
                size: original
              statistics_preserved_except:
              - size
              - form_specific_speed
              equipment_transformed: false
          skill_totals:
            perception: 4
            stealth: 4
          gear:
          - item: longbow
            quantity: 1
          senses:
            darkvision: 60
            passive_perception: 14
          languages:
            speaks:
            - common
            understands: []
            cannot_speak_forms:
            - wolf
          form_rules:
            beast:
              speed_ft: 40
              can_speak: false
            true_humanoid:
              speed_ft: 30
            hybrid:
              speed_ft: 30
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_werewolf
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - shape_shifting
      - spatial_queries
      - limited_resources
      - curses
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_werewolf
    citations:
    - SRD 5.2.1 pp. 339; builtin/srd-5.2.1/creature/werewolf.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/will_o_wisp:
    source_sha256: ede772b71c6d6d665f38212489323b5bda295b381ad78ba03355258a3682290b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/will_o_wisp
        implementation: dnd2024:creature_will_o_wisp
        parameters:
          features:
            ephemeral:
              economy: trait
              operation: passive_rule
              can_wear: false
              can_carry: false
            illumination:
              economy: trait
              operation: passive_rule
              bright_radius_ft: 20
              additional_dim_radius_ft: 20
            incorporeal_movement:
              economy: trait
              operation: passive_rule
              pass_through:
              - creatures
              - objects
              movement_terrain: difficult
              end_turn_inside_object_damage:
                roll:
                  count: 1
                  sides: 10
                  modifier: 0
                type: force
            shock:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 4
                reach_ft: 5
              hit:
                damage:
                - average: 11
                  roll:
                    count: 2
                    sides: 8
                    modifier: 2
                  type: lightning
            consume_life:
              economy: bonus_action
              operation: saving_throw
              ability: constitution
              dc: 10
              targets:
                kind: creature
                count: 1
                range_ft: 5
                visible: true
                living: true
                hp: 0
              failure:
                target_dies: true
                heal_self:
                  roll:
                    count: 3
                    sides: 6
                    modifier: 0
                  average: 10
              success:
                effects: []
            vanish:
              economy: bonus_action
              operation: apply_conditions
              targets:
              - self
              - self.light
              conditions:
              - invisible
              concentration: true
              ends_after:
              - self.attack_roll
              - self.use_consume_life
          damage_resistances:
          - acid
          - bludgeoning
          - cold
          - fire
          - necrotic
          - piercing
          - slashing
          damage_immunities:
          - lightning
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
            darkvision: 120
            passive_perception: 12
          languages:
            speaks:
            - common
            understands: []
            additional_languages:
              count: 1
              chooser: creator
              can_speak: true
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_will_o_wisp
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - attack.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - grappling
      - spatial_queries
      - limited_resources
      - hit_points
      - defenses
    requires:
    - dnd2024:creature_will_o_wisp
    citations:
    - SRD 5.2.1 pp. 341, 342; builtin/srd-5.2.1/creature/will_o_wisp.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/worg:
    source_sha256: 47cb374c17fb4c9f9aa1af44bb5c449b526fa78e9ded1a6dedc3de5c7b4671c0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/worg
        implementation: dnd2024:creature_worg
        parameters:
          features:
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 5
                reach_ft: 5
              hit:
                damage:
                - average: 7
                  roll:
                    count: 1
                    sides: 8
                    modifier: 3
                  type: piercing
                next_attack_against_target:
                  advantage: true
                  expires: self.next_turn_start
                  uses: 1
          skill_totals:
            perception: 4
          senses:
            darkvision: 60
            passive_perception: 14
          languages:
            speaks:
            - goblin
            - worg
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_worg
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - damage
      - spatial_queries
      - limited_resources
    requires:
    - dnd2024:creature_worg
    citations:
    - SRD 5.2.1 pp. 342; builtin/srd-5.2.1/creature/worg.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/xorn:
    source_sha256: ca95f710e2f0d370ef3fc5a4d0775ee376a7d9cba4e29e081607f1533fc8b68f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/xorn
        implementation: dnd2024:creature_xorn
        parameters:
          features:
            earth_glide:
              economy: trait
              operation: passive_rule
              burrow_materials:
              - earth
              - stone
              requires:
                material_nonmagical: true
                material_unworked: true
              disturb_material: false
            treasure_sense:
              economy: trait
              operation: passive_rule
              pinpoint:
              - precious_metals
              - precious_stones
              range_ft: 60
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: bite
                count: 1
              - operation: attack_sequence
                feature: claw
                count: 3
            bite:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 17
                  roll:
                    count: 4
                    sides: 6
                    modifier: 3
                  type: piercing
            claw:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 6
                reach_ft: 5
              hit:
                damage:
                - average: 8
                  roll:
                    count: 1
                    sides: 10
                    modifier: 3
                  type: slashing
            charge:
              economy: bonus_action
              operation: move
              maximum_ft:
                choose_stat:
                - speed
                - burrow_speed
              direction: straight_toward_enemy
              enemy_sensed: true
          skill_totals:
            perception: 6
            stealth: 6
          damage_immunities:
          - poison
          condition_immunities:
          - paralyzed
          - petrified
          - poisoned
          senses:
            darkvision: 60
            tremorsense: 60
            passive_perception: 16
          languages:
            speaks:
            - language: primordial
              dialects:
              - terran
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_xorn
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_xorn
    citations:
    - SRD 5.2.1 pp. 343; builtin/srd-5.2.1/creature/xorn.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/young_brass_dragon:
    source_sha256: 71baabe7079fe1f07d224dde5a4812af2fa2995f3ad9929d6c04c61af2c6a886
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/young_brass_dragon
        implementation: dnd2024:creature_young_brass_dragon
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 3
              replacements:
              - replace_count: 2
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: sleep_breath
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 7
                reach_ft: 10
              hit:
                damage:
                - average: 15
                  roll:
                    count: 2
                    sides: 10
                    modifier: 4
                  type: slashing
            fire_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 14
              targets:
                kind: creature
                selection: each
                area:
                  shape: line
                  size_ft: 40
                  origin: self
                  width_ft: 5
              failure:
                damage:
                - average: 38
                  roll:
                    count: 11
                    sides: 6
                    modifier: 0
                  type: fire
              success:
                damage_multiplier: 0.5
                other_effects: false
            sleep_breath:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 14
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 30
                  origin: self
              failure:
                conditions:
                - condition: incapacitated
                  until: target.next_turn_end
                repeat_save:
                  event: target.next_turn_end
                  failure:
                    conditions:
                    - condition: unconscious
                      duration_seconds: 60
                    ends_on:
                    - event: target.damage_taken
                    - event: creature.action
                      operation: wake_target
                      within_ft: 5
                  success: end_effect
              success:
                effects: []
          skill_totals:
            perception: 6
            persuasion: 5
            stealth: 3
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
            blindsight: 30
            darkvision: 120
            passive_perception: 16
          languages:
            speaks:
            - common
            - draconic
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_young_brass_dragon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      - damage.resolved
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_young_brass_dragon
    citations:
    - SRD 5.2.1 pp. 268; builtin/srd-5.2.1/creature/young_brass_dragon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/young_gold_dragon:
    source_sha256: bcf179046f97f79754d9bbde1701b3c80b1b829f6ab8f34d246e0d119b6bb6b3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/young_gold_dragon
        implementation: dnd2024:creature_young_gold_dragon
        parameters:
          features:
            amphibious:
              economy: trait
              operation: passive_rule
              breathing:
              - air
              - water
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: weakening_breath
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 10
                reach_ft: 10
              hit:
                damage:
                - average: 17
                  roll:
                    count: 2
                    sides: 10
                    modifier: 6
                  type: slashing
            fire_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: dexterity
              dc: 17
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 30
                  origin: self
              failure:
                damage:
                - average: 55
                  roll:
                    count: 10
                    sides: 10
                    modifier: 0
                  type: fire
              success:
                damage_multiplier: 0.5
                other_effects: false
            weakening_breath:
              economy: action
              operation: saving_throw
              ability: strength
              dc: 17
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 30
                  origin: self
                exclude_already_affected_by: this_breath
              failure:
                disadvantage_on:
                  roll: d20_test
                  ability: strength
                damage_roll_modifier:
                  subtract:
                    count: 1
                    sides: 6
                    modifier: 0
                repeat_save:
                  event: target.turn_end
                  success: end_effect
                automatic_success_after_seconds: 60
              success:
                effects: []
          skill_totals:
            insight: 5
            perception: 9
            persuasion: 9
            stealth: 6
          damage_immunities:
          - fire
          condition_immunities: []
          senses:
            blindsight: 30
            darkvision: 120
            passive_perception: 19
          languages:
            speaks:
            - common
            - draconic
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_young_gold_dragon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_young_gold_dragon
    citations:
    - SRD 5.2.1 pp. 291; builtin/srd-5.2.1/creature/young_gold_dragon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
  dnd2024:creatures/young_silver_dragon:
    source_sha256: f2a91b1434154d6971aa27901f49ef6f287e6b8f6118b92b36c10c9e9162aa08
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/young_silver_dragon
        implementation: dnd2024:creature_young_silver_dragon
        parameters:
          features:
            multiattack:
              economy: action
              operation: multiattack
              sequence:
              - operation: attack_sequence
                feature: rend
                count: 3
              replacements:
              - replace_count: 1
                maximum_uses: 1
                operation:
                  operation: use_feature
                  feature: paralyzing_breath
            rend:
              economy: action
              operation: attack
              attack:
                mode: melee
                targets:
                  count: 1
                  kind: creature_or_object
                supplied_attack_total: 10
                reach_ft: 10
              hit:
                damage:
                - average: 15
                  roll:
                    count: 2
                    sides: 8
                    modifier: 6
                  type: slashing
            cold_breath:
              economy: action
              resource:
                kind: recharge
                die: 6
                success_faces:
                - 5
                - 6
                check: self.turn_start
                rest_recovery:
                - short
                - long
                capacity: 1
                initial_uses: 1
              operation: saving_throw
              ability: constitution
              dc: 17
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 30
                  origin: self
              failure:
                damage:
                - average: 49
                  roll:
                    count: 11
                    sides: 8
                    modifier: 0
                  type: cold
              success:
                damage_multiplier: 0.5
                other_effects: false
            paralyzing_breath:
              economy: action
              operation: saving_throw
              ability: constitution
              dc: 17
              targets:
                kind: creature
                selection: each
                area:
                  shape: cone
                  size_ft: 30
                  origin: self
              failure:
                damage: []
                conditions:
                - condition: incapacitated
                  until: target.next_turn_end
                repeat_save:
                  event: target.next_turn_end
                  failure:
                    conditions:
                    - condition: paralyzed
                    repeat_save:
                      event: target.turn_end
                      success: end_effect
                    automatic_success_after_seconds: 60
                    automatic_success_clock_origin: initial_failed_save
                  success: end_effect
              success:
                effects: []
          skill_totals:
            history: 6
            perception: 8
            stealth: 4
          damage_immunities:
          - cold
          condition_immunities: []
          senses:
            blindsight: 30
            darkvision: 120
            passive_perception: 18
          languages:
            speaks:
            - common
            - draconic
            understands: []
          resolution_conventions:
            distance_unit: foot
            duration_unit: second
            damage_roll_mode:
              choice:
              - listed_average
              - roll_dice
              chooser: gm
            supplied_totals: before_additional_effect_modifiers
            multiattack_nested_economy_cost: 0
            nested_feature_resource_cost: normal
            spell_level_unless_specified: lowest_possible
            spell_upcast_without_permission: false
            spell_casting_time_at_least_one_minute: normal_casting_procedure
            geometry:
              input: adjudication
              assumptions:
              - range_and_visibility_verified
              - area_membership_verified
              - path_and_destination_legal
            unlisted_success_branch: no_effect
            unlisted_duration: until_removed_by_applicable_rule
    contract:
      id: dnd2024:creature_young_silver_dragon
      inputs:
        actor: participant
        feature:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        adjudication:
          type: resolution
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - creature.feature_requested
      - turn.started
      - turn.ended
      capabilities:
      - creature_features
      - action_economy
      - attacks
      - saving_throws
      - conditions
      - damage
      - spatial_queries
      - limited_resources
      - defenses
    requires:
    - dnd2024:creature_young_silver_dragon
    citations:
    - SRD 5.2.1 pp. 324; builtin/srd-5.2.1/creature/young_silver_dragon.md
    - SRD 5.2.1 pp. 255–257; builtin/srd-5.2.1/source.txt (stat block conventions, spellcasting, and limited usage)
---

# SRD creature conversion contracts: group 1

This document specifies all 112 assigned creature blocks and 471 named features. Ability scores, supplied saving throw totals, Armor Class, Hit Points, movement speeds, creature type, and challenge data remain in the parent converter's header extraction. This document supplies skill totals, defenses, senses, languages, gear, traits, action choices, attack damage and riders, save outcomes, spell lists, reactions, legendary actions, and resource rules. Every entry cites its original SRD block and binds to its SHA-256.

The parameters use typed records. Lowercase symbols are enums, property selectors, or event selectors. `self` is the feature owner; `target` is the affected creature. Numeric ranges use feet; durations use seconds unless the record names another unit. Dice have a count, die size, and modifier. A listed damage average is an alternative to rolling that damage, never extra damage. A missing success effect means that success has no effect. Persistent effects with no printed duration use the applicable condition, curse, healing, or effect removal rules. Shape and path checks are explicit spatial adjudication inputs. They do not imply a map engine.

A multiattack pays its own action cost once. Calls to nested attacks and features do not pay that feature's normal action or Bonus Action cost again, but still consume its limited resource. Spellcasting uses the printed ability and component exceptions. Unspecified spell levels use the lowest possible level. A spell with a casting time of at least 1 minute still uses its normal extended casting procedure. Per-day resources recover on a Long Rest, as required by the SRD stat block conventions on page 257.

Hydra reaction capacity is recomputed only at the start of its turn. A change in head count changes the next refresh; it does not grant new reactions immediately. Regrowth heals 20 Hit Points per event, not per head. This preserves the user's reviewed ruling and the existing `scripts/srd_inputs/contracts.yaml` Hydra contract.

The Giant Fly source file includes unrelated Figurines of Wondrous Power text after the creature's CR line. The actual Giant Fly block has no traits or actions. These item paragraphs are source extraction spillover and are excluded from the creature contract. They remain the responsibility of the named magic item conversion. This is a resolved source boundary issue, not an omitted creature rule.

No engine code is supplied. These contracts state all assigned mechanics and the capability families needed to implement them.

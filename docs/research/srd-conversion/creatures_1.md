---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:creatures/aboleth:
    source_sha256: 931e8347595110f3c1fde92d373ba751cf9b778c90aff7d67afcaca06e469a42
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
    source_sha256: 07da9bc301da4cff20837f29ff478214a10dc86b5611be4e660d3516c2c54ea2
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
    source_sha256: 8652f9338967e1c8cf799ef7231c06f5dd72c4c4a37649a3c1c3ca5e7d5e13d2
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
    source_sha256: 6d3605a58b837982cab11ce1ba28c9330697c1ec5b98f3a07d7c99b635b97617
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
    source_sha256: 565cc9f9e2672b653a7671f001fc0d9229f437598f03cab02d9a58f96df2075c
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
    source_sha256: d70ca58b068405f297974222149b3d360f5f9d650d1cafb0be6f66e9a1347f94
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
    source_sha256: 9a16b7cfabd12ee9bdbfd4d066587816dd9baeb8a24fce73b4d8b584d130aae2
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
    source_sha256: 61ef1199f1b09d32c0340e7e696864e9218b41bdd3603b2f379bd9c8e19febab
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
    source_sha256: cb0c086be52db472c302ec30c386c013a2efbde5b6dc6a903518f68925ed04ef
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
    source_sha256: 76901a11aec8324157f654f046d7016b4d889184b7c611eaa082b9f5a7913e77
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
    source_sha256: 5fd52aec4d8669d79405a3cff0c6379af6d961763e87b32a0e78c0c7d66a82c2
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
    source_sha256: b9824a671b8f6844a5529495ff7cfe91f6bce8d8b118655a9419d5c8e29a0a57
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
    source_sha256: 198b966748df74d0f68e24f675818efaf63e6aa678046cf31d98bec85403a224
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
    source_sha256: 57c56591f9c316c7100f6909d5a716fd6ed2df1f4a56a1c1c1f9721f7cb851c4
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
    source_sha256: de7aa6d61ed83c43441091eefd9addde5011d5a901b2768968c716d2b8122d96
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
    source_sha256: 58d3595190b9701cac37a011dde4a060e763e7427657b0db77c8d4d5b1bba3cf
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
    source_sha256: 3d05a30d67d1e757ad9d02f40fffc842e0db7b9387a3d4cb79d6fab771269ef7
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
    source_sha256: 31cf78d37dbc1001f005257a2eef0a380a5ee540a9cccc31dc547a18cace360e
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
    source_sha256: 279ff0eda24658193c8ff9065e7926d2b319c2f41a19102d9833b7f6aab548ea
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
    source_sha256: 6f460e9731282a768b358f5cb27a7af220d186d8e59b4037c91ea23eef89b2e8
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
    source_sha256: 6491f042035a25183da653f0f86c0d5ed70bc214c6d948a4ceb99a5c1f68c36f
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
    source_sha256: bd33ba5a8d2c526cfd809a2061bba34e9b99fb957016b709b7e3ddba4e575b74
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
    source_sha256: e93db4f35911e87cab267b3a62163dee9e8948fafdfad305862870c9e65e4cf0
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
    source_sha256: bfa11f299b1163986c2405c4ea96814b9c4d797c46ff926a50cda6724957bd7a
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
    source_sha256: 035bab0ca2117c05fe72d087f1789ba3df3f14ce3dc1746bf2b6932441c0900e
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
    source_sha256: d82271634f5ceed17ac060d459b445613b8c27831d6076c6c1a3bcf23218fd7b
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
    source_sha256: 07a84fddbd9c1acfd08b02b413043f56f248b173c94d444a62ab81ca03007251
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
    source_sha256: 4d15b26f726510789f718d5ccf44289e13ca662e1fc1a094ddb7f3053e5e6573
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
    source_sha256: 66194879026bcee4df4cf291296c9b4d9bced7239f70bc5395524ed12bc6b00e
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
    source_sha256: 68b020296767557835f4c0eba662251c75c2fc467e40125595b297a7381715c5
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
    source_sha256: 6ae1b3b80b44a89590fd47d7c58cc88245037a9055a98736935018569e1f8529
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
    source_sha256: f40f32e0ae6f0e7fba4d5110b3a2bb16475ea03db037d4c710c80ebe7e621979
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
    source_sha256: eb2c59c53f1b153b81fd4f7e15eb87000e43be36a033209557a770fb84ce9a7b
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
    source_sha256: de717ed3aac4cfda3736102e48cc48e4bbf3dc813317c9dc770659a5d8cd41c8
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
    source_sha256: 0de39724523076b6c3374d59e61682aac22233d5cdc6df9122456767c26dbc39
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
    source_sha256: 4a89f4d78f645dd229cfd417d0c1773ff84ba4bb9df7b5f0bed09856a081fc4d
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
    source_sha256: 41f19934c425432e7f9313581e5a3685bac4bcf68571f4d82479d273ba8880b6
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
    source_sha256: 40406eb21c1a8763583ea8d962fc395ed83da328fbaa5bfe9c7ae186c6f7e616
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
    source_sha256: 0ce93f9965303c04a2439712b8a355fd3a3dd768d2532cb712f445d0bcb54e7b
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
    source_sha256: 4fd18d500aac042f843076abbceba6410cdbbc7e054cd8e1a26a62ce99091c22
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
    source_sha256: 403c2db54da4650e697c8068e4d840d280ef9293432cae52aedb3a912bd10b4d
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
    source_sha256: db0bc1eb6c49ab3e386b735a614e737089949ba30c57ef468a85e0afc35bdd97
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
    source_sha256: 9fc5a044d6b48b0cfe14f16ab3eb563c5800e317ff1b2bb600560e1facd71c2a
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
    source_sha256: bbe3b8f783f42944deda9e6a14b99116f649d537e639d2f881c016a851ff6e3a
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
    source_sha256: cb6c95099b38a7b95a09f7f3cc69ccb59e90a5562f84bfdfa48fcd6a64e63286
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
    source_sha256: 3ef5c0ccf3d41ed6ccde135df335d818fcc53c20c9ff7aaec5d4ea5bbbd86b8c
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
    source_sha256: 881a33b69219fe3852e17a7a53b797005e75b30d24dc1dbc49a5c6d5b80835c0
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
    source_sha256: 114fa633e49fc5f03aa0229c5c3c67270d8276cb3647f9fc2a828a9f591296cf
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
    source_sha256: 1731ce5d294de52589b6d0441c02075fdf27973bf1bbe8305f70e1fc9c47cc02
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
    source_sha256: 4461366e806e97e3ca61c1c9503b32eda4cf4ab7dd130a283df9a45166fb98e8
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
    source_sha256: 87f3904b281dbbc4bfb868b945947c66dc50ebe285b8b5c3ed0824b075064a8c
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
    source_sha256: 11cfbfc37cb594e817c49f478ee3e524330f6ce164991f92a36b889d90646ac4
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
    source_sha256: ea033960af21a157599e39a66e2e4393ed44a123764cacf2e20a8c54d52f9d02
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
    source_sha256: 434bd1db2e99f412c2392cc07f7c2a60a5caf51289c2d25e3068d9e2987b648c
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
    source_sha256: e461fc621d2af27719a4d9a79cff5187ca5af22e2e6cb9983b1b5729b9e1c921
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
    source_sha256: e2041047b252477f1ccd0c813ce5b7895bce5ce59ef7492fb9610e4a7f3f61e6
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
    source_sha256: 2c39cddd66646a8192c091019472ed9aff7fddf7a303f7d0a9854ad0f086d25b
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
    source_sha256: ca08ea7d6826972f7336f6f1184d7398d6b29f73e9ead59bdd7cf50ba2a3434f
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
    source_sha256: 612a69de012f5de7e07f1d54ad175bdb362ea8fd9c04b8206bec17e8cae5df9c
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
    source_sha256: 429a5a604b7cc4f1f75e4669efbb7d30052e3ec08d28a83e10c8154b92c1e5c1
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
    source_sha256: 5fd1b318863c1c885dc0c1260f81b7751b0c17c1a6bfc94a683315217bec8b4c
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
    source_sha256: 720b9a15b3c62611d2222cd0f7257566fb5b883c1e6523f426edfd5ea10984c7
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
    source_sha256: f0bee5a86ec0ace427029a1642ca8216eae6f58df6a7dd8f40446c4fb4c88197
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
    source_sha256: 5d1913418fb4a17b87964ed1e684e58ad5e7576e23507c8953bd07a42edcf0f0
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
    source_sha256: a73261030e742704e20013e6e9292d264869e79efbcd67db0ec2bd09e10ac99e
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
    source_sha256: 30b051dadd204bea01ffeec10e08327b997bbb219dbe432eb93c927a07d0305e
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
    source_sha256: 0ab642db46371723783a1ff2db647720a322211f6d86fa0878ba996d1a8e1d0c
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
    source_sha256: 5fcc81354a307f0e487b13fcdefa1b92400dd471e246cf943115ff3a10392492
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
    source_sha256: 44d243ff9b68e20f818b5641a75adde4dded6ac9f34fd5dba8022f103b984f8b
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
    source_sha256: 7e113922844d8376ad72ea40dd3e5fb14aba52a0e8d5de44a28617a65aad9cb2
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
    source_sha256: c5c89fba27683e2b1de00bcb873ab44da204cad748008c8128cc1f8861dd7be7
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
    source_sha256: 5e833c32c60e2e766e1cfdb0c5c596ceb983a9c54df26c5a1a2295c9bf5097f7
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
    source_sha256: 283e4a06aa324c6267ce8844e366ee0f72a7fbeff89264a69fd12c935d9f7d3b
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
    source_sha256: e52c19e25be78f63fe95c3d6cf6628156c6ddcf36dff70782303b4bb6d13c904
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
    source_sha256: d66164c41d2c35be3ee6b81922c35549efea20296d32d9bcecdbdeba831784ef
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
    source_sha256: 5416caaba23b49d0cce5bb2ef97645c336c0ae3af1ffc54eb2834b1d93dc0713
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
    source_sha256: 6773cdaf0f43d0d7b2b7f746dcbfa1eb0e69de81750c751d24511d1a1eef4357
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
    source_sha256: f5b56971e0ae3af715670037f1b854f781fffc0c87d58a8a7fb4151cbd40f007
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
    source_sha256: 189044afe310363cdfad5b16c027dcf9c5b0452635847991c27ceb9a0c82c97d
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
    source_sha256: 043d49fce232734031d17779ba41af85cd7172e821a93dc6ab79426a5f1c5c39
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
    source_sha256: 66de419073ad8b049f79d30eee77c374c4ec23bd247ec9b40b7f93e20570e2b9
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
    source_sha256: 65fabba3ff35bbfaeb319de24944e614adfc82fe397581d1ba452d2b523bb81e
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
    source_sha256: d4d1c1c3c113faf3ba482fdde5f0a8e32e7f78eb8f0deb194d571bcbc493d853
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
    source_sha256: b56bc5e4a1c4dd3c043a7c56a95283729b7f041436a76d66a97a4b4286d7d932
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
    source_sha256: 04ad2318b260ede2a7e3836cc146aaedceaabef7dd909017c84690c04bbf1ba1
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
    source_sha256: 7eb21cd0802c874fdd512961584fc247491fba6004b78019cf7f1f5ceb17e4f8
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
    source_sha256: 8c7dc6fe4f16c5c1b4349983d30fbcd110b609e71e62069e4e677dd961039f7b
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
    source_sha256: fea608b31471daad7af9371b7ca1d0098ca0a4552f4f00afaa1489e9dfdc6f01
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
    source_sha256: acffa1a37328d8cd42bc8431005ff69e536ea77bf052add0354eca480917d156
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
    source_sha256: bda3ecf73a9f5bc80ffebcaa9301a3a8cb727a3a175174f7f6d5a67b2574abdb
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
    source_sha256: cc40f6e28225e458b3a085fd968aa02d79b15af3c909c1f2980d74ed7cf900a7
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
    source_sha256: 8eb8addfc864f364513788848621edf6bac36084c10bca643ec4c7275f1e2e7d
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
    source_sha256: 1d5e71127a9390f19b3652f6c63ad5bcdb1b615cd4b2329be2892d9aa26e7fe5
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
    source_sha256: 33555c566108bdbe98b7ed35baa181e875ae8fdd7ee170cf819aa6adad69d9bc
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
    source_sha256: bbea3bf7d0bdc377a2d101e8c26eb42960705104183f16700e0ad90996fc17bf
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
    source_sha256: b888950be131d8884b3dc5ae9383c76084723538c7890b2381051fc385d711fb
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
    source_sha256: 8460b6fb4c62fb3e0238699119a5e8619f1885aff0e27f639003a62a641c6ebe
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
    source_sha256: a7aab76bb43ebd3db536ea4f19f629e8569e8c9b5602e74dbabc138f663adc06
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
    source_sha256: f9df949c192663d5c69c362822d1faf0b7b1e74e4e249329b32ee511d50c1523
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
    source_sha256: 7ca86b069ca37b7aaec3ffe8a8c277a330a0ee54e1329af8a40d3ce513dce210
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
    source_sha256: 74d14dbdad9d79542d62442f331e6c90f1dadb66f12ae531c8ff117dbe73914f
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
    source_sha256: fb1a2d465cef4c0f72cedaaa1ae4f4a1b803d8144c0bbea3a34ee7502d4ccf45
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
    source_sha256: 4858f30df6b9fa3374aa4f74d13734a2b906f817a99875798e3f6bd1521c01a1
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
    source_sha256: b9d6121429c590f9a222bf000f5d5a20fe12c4eac2e484cd6ac025702ddb41fa
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
    source_sha256: b49ddc96959439540fe94f4a7c8388485a969a938df324339baf8b882cfe4fca
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
    source_sha256: 5ab498773c26f68de652a8bef612118d1a52282d2e647400e2f79c070bca68e4
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
    source_sha256: c40602a70d4ddab560a701a84435d70480124abbb8c75e3843d9ac3c3315d017
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
    source_sha256: dae5b686b4efbbeec76fc0366f40d88241601d57e671b25c3c30b4999b5b3846
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
    source_sha256: fe940e9e34b29140f1fa1b5d2bf069efb44ddf28dc10013fef355d0f11b051f8
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
    source_sha256: 199cedc2250331663225b21e64cac6ce99cb2191757e6462f87971d447d08a21
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
    source_sha256: e4a8390d0c75d8de29792e8b835035e277c9f976b410695b8174c2549a735025
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
    source_sha256: 6d0e5bef42492dafee936e23204607414b54002fda02ed7fa16175d924e1ac6f
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

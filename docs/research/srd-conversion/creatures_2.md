---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:creatures/adult_black_dragon:
    source_sha256: 6cfcf9d1c6c1199a6b856a471abc80d9505380a631d10859aba7925e7b28e4d2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/adult_black_dragon
        implementation: dnd2024:creature_adult_black_dragon
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            legendary_resistance:
              economy: passive
              resource:
                capacity: 3
                capacity_in_lair: 4
                recovery: long_rest
                cost: 1
              trigger: saving_throw.failed
              optional: true
              op: replace_save_outcome
              outcome: success
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 11
              targets:
                count: 1
              hit:
              - op: damage
                average: 13
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 6
                damage_type: slashing
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 0
                damage_type: acid
              reach_feet: 10
            acid_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 18
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  size_feet: 60
                  origin: self
                  width_feet: 5
              failure:
              - op: damage
                average: 54
                amount:
                  dice:
                    count: 12
                    sides: 8
                  add: 0
                damage_type: acid
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
              replacement:
                max_attacks: 1
                choices:
                - feature: spellcasting
                  spell:
                    key: dnd2024:spells/acid_arrow
                  cast_level: 3
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: charisma
              spells:
              - spell:
                  key: dnd2024:spells/acid_arrow
                frequency: at_will
                cast_level: 3
              - spell:
                  key: dnd2024:spells/detect_magic
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/fear
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/speak_with_dead
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/vitriolic_sphere
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - material
              save_dc: 17
              spell_attack_bonus: 9
            cloud_of_insects:
              economy: legendary_action
              op: save
              ability: dexterity
              dc: 17
              targets:
                kind: creature
                count: 1
                range_feet: 120
                visible_to: self
              failure:
              - op: damage
                average: 22
                amount:
                  dice:
                    count: 4
                    sides: 10
                  add: 0
                damage_type: poison
              - op: disadvantage
                roll: concentration_save
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
              success: []
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
            frightful_presence:
              economy: legendary_action
              op: cast_spell
              spell:
                key: dnd2024:spells/fear
              via: spellcasting
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
              cast_level: spell_minimum
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: move
                distance_max:
                  multiply:
                  - stat: speed
                  - numerator: 1
                    denominator: 2
              - op: use_feature
                feature: rend
                count: 1
          skill_totals:
            perception: 11
            stealth: 7
          defenses:
            immunities:
              damage_types:
              - acid
          senses:
            passive_perception: 21
            special:
            - sense: blindsight
              range_feet: 60
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            cost_per_action: 1
            trigger: another_creature.turn.end
            recovery:
              event: turn.start
              participant: self
              amount: all
            maximum_per_trigger: 1
            requires:
              self_incapacitated: false
              self_can_take_actions: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_adult_black_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - another_creature.turn.end
      - saving_throw.failed
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - legendary_actions
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_adult_black_dragon
    citations:
    - SRD 5.2.1 pp. 264; builtin/srd-5.2.1/creature/adult_black_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/adult_bronze_dragon:
    source_sha256: 3da6982105cc75bac35c592a4613da3797435fd3d53232fa9240a35440c03975
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/adult_bronze_dragon
        implementation: dnd2024:creature_adult_bronze_dragon
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            legendary_resistance:
              economy: passive
              resource:
                capacity: 3
                capacity_in_lair: 4
                recovery: long_rest
                cost: 1
              trigger: saving_throw.failed
              optional: true
              op: replace_save_outcome
              outcome: success
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 12
              targets:
                count: 1
              hit:
              - op: damage
                average: 16
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 7
                damage_type: slashing
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 0
                damage_type: lightning
              reach_feet: 10
            lightning_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 19
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  size_feet: 90
                  origin: self
                  width_feet: 5
              failure:
              - op: damage
                average: 55
                amount:
                  dice:
                    count: 10
                    sides: 10
                  add: 0
                damage_type: lightning
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
              replacement:
                max_attacks: 1
                choices:
                - feature: repulsion_breath
                - feature: spellcasting
                  spell:
                    key: dnd2024:spells/guiding_bolt
                  cast_level: 2
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: charisma
              spells:
              - spell:
                  key: dnd2024:spells/detect_magic
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/guiding_bolt
                frequency: at_will
                cast_level: 2
              - spell:
                  key: dnd2024:spells/shapechange
                frequency: at_will
                overrides:
                  form_types:
                  - beast
                  - humanoid
                  temporary_hit_points_gained: 0
                  concentration_required: false
                  temporary_hit_points_to_maintain_required: false
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/speak_with_animals
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/thaumaturgy
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/detect_thoughts
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/water_breathing
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - material
              save_dc: 17
              spell_attack_bonus: 10
            thunderclap:
              economy: legendary_action
              op: save
              ability: constitution
              dc: 17
              targets:
                kind: creature
                count: all
                area:
                  shape: sphere
                  size_feet: 20
                  origin: chosen_point
                  origin_visible_to: self
                  origin_range_feet: 90
              failure:
              - op: damage
                average: 10
                amount:
                  dice:
                    count: 3
                    sides: 6
                  add: 0
                damage_type: thunder
              - op: condition
                condition: deafened
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
              success: []
            repulsion_breath:
              economy: action
              op: save
              ability: strength
              dc: 19
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 30
                  origin: self
              failure:
              - op: push
                distance_max_feet: 60
                direction: straight_away_from_self
              - op: condition
                condition: prone
              success: []
            guiding_light:
              economy: legendary_action
              op: cast_spell
              spell:
                key: dnd2024:spells/guiding_bolt
              via: spellcasting
              cast_level: 2
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: move
                distance_max:
                  multiply:
                  - stat: speed
                  - numerator: 1
                    denominator: 2
              - op: use_feature
                feature: rend
                count: 1
          skill_totals:
            insight: 7
            perception: 12
            stealth: 5
          defenses:
            immunities:
              damage_types:
              - lightning
          senses:
            passive_perception: 22
            special:
            - sense: blindsight
              range_feet: 60
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            cost_per_action: 1
            trigger: another_creature.turn.end
            recovery:
              event: turn.start
              participant: self
              amount: all
            maximum_per_trigger: 1
            requires:
              self_incapacitated: false
              self_can_take_actions: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_adult_bronze_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - another_creature.turn.end
      - saving_throw.failed
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - legendary_actions
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_adult_bronze_dragon
    citations:
    - SRD 5.2.1 pp. 270, 271; builtin/srd-5.2.1/creature/adult_bronze_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/adult_green_dragon:
    source_sha256: 6633b3a677bf1ff70374d115894ec94002a20eccf5417069ec5b62687aed09ac
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/adult_green_dragon
        implementation: dnd2024:creature_adult_green_dragon
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            legendary_resistance:
              economy: passive
              resource:
                capacity: 3
                capacity_in_lair: 4
                recovery: long_rest
                cost: 1
              trigger: saving_throw.failed
              optional: true
              op: replace_save_outcome
              outcome: success
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 11
              targets:
                count: 1
              hit:
              - op: damage
                average: 15
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 6
                damage_type: slashing
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: poison
              reach_feet: 10
            poison_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: constitution
              dc: 18
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 60
                  origin: self
              failure:
              - op: damage
                average: 56
                amount:
                  dice:
                    count: 16
                    sides: 6
                  add: 0
                damage_type: poison
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
              replacement:
                max_attacks: 1
                choices:
                - feature: spellcasting
                  spell:
                    key: dnd2024:spells/mind_spike
                  cast_level: 3
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: charisma
              spells:
              - spell:
                  key: dnd2024:spells/detect_magic
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/mind_spike
                frequency: at_will
                cast_level: 3
              - spell:
                  key: dnd2024:spells/geas
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - material
              save_dc: 17
            noxious_miasma:
              economy: legendary_action
              op: save
              ability: constitution
              dc: 17
              targets:
                kind: creature
                count: all
                area:
                  shape: sphere
                  size_feet: 20
                  origin: chosen_point
                  origin_visible_to: self
                  origin_range_feet: 90
              failure:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: poison
              - op: modify_ac
                add: -2
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
              success: []
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
            mind_invasion:
              economy: legendary_action
              op: cast_spell
              spell:
                key: dnd2024:spells/mind_spike
              via: spellcasting
              cast_level: 3
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: move
                distance_max:
                  multiply:
                  - stat: speed
                  - numerator: 1
                    denominator: 2
              - op: use_feature
                feature: rend
                count: 1
          skill_totals:
            deception: 9
            perception: 12
            persuasion: 9
            stealth: 6
          defenses:
            immunities:
              damage_types:
              - poison
              conditions:
              - poisoned
          senses:
            passive_perception: 22
            special:
            - sense: blindsight
              range_feet: 60
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            cost_per_action: 1
            trigger: another_creature.turn.end
            recovery:
              event: turn.start
              participant: self
              amount: all
            maximum_per_trigger: 1
            requires:
              self_incapacitated: false
              self_can_take_actions: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_adult_green_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - another_creature.turn.end
      - saving_throw.failed
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - legendary_actions
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_adult_green_dragon
    citations:
    - SRD 5.2.1 pp. 294; builtin/srd-5.2.1/creature/adult_green_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/adult_white_dragon:
    source_sha256: 8141a18fbf7014e732c4fa1d0485fd05d73ddfa441f70fc2a609b1b2914aa4e1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/adult_white_dragon
        implementation: dnd2024:creature_adult_white_dragon
        parameters:
          features:
            ice_walk:
              economy: passive
              movement_checks_waived:
                surfaces:
                - ice
                modes:
                - walk
                - climb
              difficult_terrain_extra_cost_ignored:
              - ice
              - snow
            legendary_resistance:
              economy: passive
              resource:
                capacity: 3
                capacity_in_lair: 4
                recovery: long_rest
                cost: 1
              trigger: saving_throw.failed
              optional: true
              op: replace_save_outcome
              outcome: success
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 11
              targets:
                count: 1
              hit:
              - op: damage
                average: 13
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 6
                damage_type: slashing
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 0
                damage_type: cold
              reach_feet: 10
            cold_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: constitution
              dc: 19
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 60
                  origin: self
              failure:
              - op: damage
                average: 54
                amount:
                  dice:
                    count: 12
                    sides: 8
                  add: 0
                damage_type: cold
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
            freezing_burst:
              economy: legendary_action
              op: save
              ability: constitution
              dc: 14
              targets:
                kind: creature
                count: all
                area:
                  shape: sphere
                  size_feet: 30
                  origin: chosen_point
                  origin_visible_to: self
                  origin_range_feet: 120
              failure:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: cold
              - op: set_speed
                feet: 0
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
              success: []
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
            frightful_presence:
              economy: legendary_action
              op: cast_spell
              spell:
                key: dnd2024:spells/fear
              ability: charisma
              save_dc: 14
              components_waived:
              - material
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
              cast_level: spell_minimum
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: move
                distance_max:
                  multiply:
                  - stat: speed
                  - numerator: 1
                    denominator: 2
              - op: use_feature
                feature: rend
                count: 1
          skill_totals:
            perception: 11
            stealth: 5
          defenses:
            immunities:
              damage_types:
              - cold
          senses:
            passive_perception: 21
            special:
            - sense: blindsight
              range_feet: 60
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            cost_per_action: 1
            trigger: another_creature.turn.end
            recovery:
              event: turn.start
              participant: self
              amount: all
            maximum_per_trigger: 1
            requires:
              self_incapacitated: false
              self_can_take_actions: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_adult_white_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - another_creature.turn.end
      - saving_throw.failed
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - legendary_actions
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_adult_white_dragon
    citations:
    - SRD 5.2.1 pp. 340; builtin/srd-5.2.1/creature/adult_white_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ancient_black_dragon:
    source_sha256: 5cab99de300a22a164fd6e836a500cf879b665244c7b733ed312f1e28cbf2475
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ancient_black_dragon
        implementation: dnd2024:creature_ancient_black_dragon
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            legendary_resistance:
              economy: passive
              resource:
                capacity: 4
                capacity_in_lair: 5
                recovery: long_rest
                cost: 1
              trigger: saving_throw.failed
              optional: true
              op: replace_save_outcome
              outcome: success
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 15
              targets:
                count: 1
              hit:
              - op: damage
                average: 17
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 8
                damage_type: slashing
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 0
                damage_type: acid
              reach_feet: 15
            acid_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 22
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  size_feet: 90
                  origin: self
                  width_feet: 10
              failure:
              - op: damage
                average: 67
                amount:
                  dice:
                    count: 15
                    sides: 8
                  add: 0
                damage_type: acid
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
              replacement:
                max_attacks: 1
                choices:
                - feature: spellcasting
                  spell:
                    key: dnd2024:spells/acid_arrow
                  cast_level: 4
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: charisma
              spells:
              - spell:
                  key: dnd2024:spells/acid_arrow
                frequency: at_will
                cast_level: 4
              - spell:
                  key: dnd2024:spells/detect_magic
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/fear
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/create_undead
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/speak_with_dead
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/vitriolic_sphere
                frequency: daily
                uses: 1
                resource_scope: per_spell
                cast_level: 5
                recovery: long_rest
              components_waived:
              - material
              save_dc: 21
              spell_attack_bonus: 13
            cloud_of_insects:
              economy: legendary_action
              op: save
              ability: dexterity
              dc: 21
              targets:
                kind: creature
                count: 1
                range_feet: 120
                visible_to: self
              failure:
              - op: damage
                average: 33
                amount:
                  dice:
                    count: 6
                    sides: 10
                  add: 0
                damage_type: poison
              - op: disadvantage
                roll: concentration_save
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
              success: []
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
            frightful_presence:
              economy: legendary_action
              op: cast_spell
              spell:
                key: dnd2024:spells/fear
              via: spellcasting
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
              cast_level: spell_minimum
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: move
                distance_max:
                  multiply:
                  - stat: speed
                  - numerator: 1
                    denominator: 2
              - op: use_feature
                feature: rend
                count: 1
          skill_totals:
            perception: 16
            stealth: 9
          defenses:
            immunities:
              damage_types:
              - acid
          senses:
            passive_perception: 26
            special:
            - sense: blindsight
              range_feet: 60
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            cost_per_action: 1
            trigger: another_creature.turn.end
            recovery:
              event: turn.start
              participant: self
              amount: all
            maximum_per_trigger: 1
            requires:
              self_incapacitated: false
              self_can_take_actions: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_ancient_black_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - another_creature.turn.end
      - saving_throw.failed
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - legendary_actions
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_ancient_black_dragon
    citations:
    - SRD 5.2.1 pp. 265; builtin/srd-5.2.1/creature/ancient_black_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ancient_bronze_dragon:
    source_sha256: 90b430055606f00b760d7380790ff11d8de3718a4fa2ecbcee1f40d638cda7b0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ancient_bronze_dragon
        implementation: dnd2024:creature_ancient_bronze_dragon
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            legendary_resistance:
              economy: passive
              resource:
                capacity: 4
                capacity_in_lair: 5
                recovery: long_rest
                cost: 1
              trigger: saving_throw.failed
              optional: true
              op: replace_save_outcome
              outcome: success
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 16
              targets:
                count: 1
              hit:
              - op: damage
                average: 18
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 9
                damage_type: slashing
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 0
                damage_type: lightning
              reach_feet: 15
            lightning_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 23
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  size_feet: 120
                  origin: self
                  width_feet: 10
              failure:
              - op: damage
                average: 82
                amount:
                  dice:
                    count: 15
                    sides: 10
                  add: 0
                damage_type: lightning
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
              replacement:
                max_attacks: 1
                choices:
                - feature: repulsion_breath
                - feature: spellcasting
                  spell:
                    key: dnd2024:spells/guiding_bolt
                  cast_level: 2
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: charisma
              spells:
              - spell:
                  key: dnd2024:spells/detect_magic
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/guiding_bolt
                frequency: at_will
                cast_level: 2
              - spell:
                  key: dnd2024:spells/shapechange
                frequency: at_will
                overrides:
                  form_types:
                  - beast
                  - humanoid
                  temporary_hit_points_gained: 0
                  concentration_required: false
                  temporary_hit_points_to_maintain_required: false
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/speak_with_animals
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/thaumaturgy
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/detect_thoughts
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/control_water
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/scrying
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/water_breathing
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - material
              save_dc: 22
              spell_attack_bonus: 14
            thunderclap:
              economy: legendary_action
              op: save
              ability: constitution
              dc: 22
              targets:
                kind: creature
                count: all
                area:
                  shape: sphere
                  size_feet: 20
                  origin: chosen_point
                  origin_visible_to: self
                  origin_range_feet: 120
              failure:
              - op: damage
                average: 13
                amount:
                  dice:
                    count: 3
                    sides: 8
                  add: 0
                damage_type: thunder
              - op: condition
                condition: deafened
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
              success: []
            repulsion_breath:
              economy: action
              op: save
              ability: strength
              dc: 23
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 30
                  origin: self
              failure:
              - op: push
                distance_max_feet: 60
                direction: straight_away_from_self
              - op: condition
                condition: prone
              success: []
            guiding_light:
              economy: legendary_action
              op: cast_spell
              spell:
                key: dnd2024:spells/guiding_bolt
              via: spellcasting
              cast_level: 2
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: move
                distance_max:
                  multiply:
                  - stat: speed
                  - numerator: 1
                    denominator: 2
              - op: use_feature
                feature: rend
                count: 1
          skill_totals:
            insight: 10
            perception: 17
            stealth: 7
          defenses:
            immunities:
              damage_types:
              - lightning
          senses:
            passive_perception: 27
            special:
            - sense: blindsight
              range_feet: 60
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            cost_per_action: 1
            trigger: another_creature.turn.end
            recovery:
              event: turn.start
              participant: self
              amount: all
            maximum_per_trigger: 1
            requires:
              self_incapacitated: false
              self_can_take_actions: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_ancient_bronze_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - another_creature.turn.end
      - saving_throw.failed
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - legendary_actions
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_ancient_bronze_dragon
    citations:
    - SRD 5.2.1 pp. 271; builtin/srd-5.2.1/creature/ancient_bronze_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ancient_green_dragon:
    source_sha256: e70297ccf9b245ad4c16ed1dea826157764129a924377e63a36dfe7dd12775a7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ancient_green_dragon
        implementation: dnd2024:creature_ancient_green_dragon
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            legendary_resistance:
              economy: passive
              resource:
                capacity: 4
                capacity_in_lair: 5
                recovery: long_rest
                cost: 1
              trigger: saving_throw.failed
              optional: true
              op: replace_save_outcome
              outcome: success
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 15
              targets:
                count: 1
              hit:
              - op: damage
                average: 17
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 8
                damage_type: slashing
              - op: damage
                average: 10
                amount:
                  dice:
                    count: 3
                    sides: 6
                  add: 0
                damage_type: poison
              reach_feet: 15
            poison_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: constitution
              dc: 22
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 90
                  origin: self
              failure:
              - op: damage
                average: 77
                amount:
                  dice:
                    count: 22
                    sides: 6
                  add: 0
                damage_type: poison
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
              replacement:
                max_attacks: 1
                choices:
                - feature: spellcasting
                  spell:
                    key: dnd2024:spells/mind_spike
                  cast_level: 5
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: charisma
              spells:
              - spell:
                  key: dnd2024:spells/detect_magic
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/mind_spike
                frequency: at_will
                cast_level: 5
              - spell:
                  key: dnd2024:spells/geas
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/modify_memory
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - material
              save_dc: 21
            noxious_miasma:
              economy: legendary_action
              op: save
              ability: constitution
              dc: 21
              targets:
                kind: creature
                count: all
                area:
                  shape: sphere
                  size_feet: 30
                  origin: chosen_point
                  origin_visible_to: self
                  origin_range_feet: 90
              failure:
              - op: damage
                average: 17
                amount:
                  dice:
                    count: 5
                    sides: 6
                  add: 0
                damage_type: poison
              - op: modify_ac
                add: -2
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
              success: []
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
            mind_invasion:
              economy: legendary_action
              op: cast_spell
              spell:
                key: dnd2024:spells/mind_spike
              via: spellcasting
              cast_level: 5
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: move
                distance_max:
                  multiply:
                  - stat: speed
                  - numerator: 1
                    denominator: 2
              - op: use_feature
                feature: rend
                count: 1
          skill_totals:
            deception: 13
            perception: 17
            persuasion: 13
            stealth: 8
          defenses:
            immunities:
              damage_types:
              - poison
              conditions:
              - poisoned
          senses:
            passive_perception: 27
            special:
            - sense: blindsight
              range_feet: 60
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            cost_per_action: 1
            trigger: another_creature.turn.end
            recovery:
              event: turn.start
              participant: self
              amount: all
            maximum_per_trigger: 1
            requires:
              self_incapacitated: false
              self_can_take_actions: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_ancient_green_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - another_creature.turn.end
      - saving_throw.failed
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - legendary_actions
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_ancient_green_dragon
    citations:
    - SRD 5.2.1 pp. 294, 295; builtin/srd-5.2.1/creature/ancient_green_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ancient_white_dragon:
    source_sha256: ee58995365f7e50068e4c62e9a8c7ff6fa93f349bf9e928a88a0831208d5c490
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ancient_white_dragon
        implementation: dnd2024:creature_ancient_white_dragon
        parameters:
          features:
            ice_walk:
              economy: passive
              movement_checks_waived:
                surfaces:
                - ice
                modes:
                - walk
                - climb
              difficult_terrain_extra_cost_ignored:
              - ice
              - snow
            legendary_resistance:
              economy: passive
              resource:
                capacity: 4
                capacity_in_lair: 5
                recovery: long_rest
                cost: 1
              trigger: saving_throw.failed
              optional: true
              op: replace_save_outcome
              outcome: success
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 14
              targets:
                count: 1
              hit:
              - op: damage
                average: 17
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 8
                damage_type: slashing
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: cold
              reach_feet: 15
            cold_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: constitution
              dc: 22
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 90
                  origin: self
              failure:
              - op: damage
                average: 63
                amount:
                  dice:
                    count: 14
                    sides: 8
                  add: 0
                damage_type: cold
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
            freezing_burst:
              economy: legendary_action
              op: save
              ability: constitution
              dc: 20
              targets:
                kind: creature
                count: all
                area:
                  shape: sphere
                  size_feet: 30
                  origin: chosen_point
                  origin_visible_to: self
                  origin_range_feet: 120
              failure:
              - op: damage
                average: 14
                amount:
                  dice:
                    count: 4
                    sides: 6
                  add: 0
                damage_type: cold
              - op: set_speed
                feet: 0
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
              success: []
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
            frightful_presence:
              economy: legendary_action
              op: cast_spell
              spell:
                key: dnd2024:spells/fear
              ability: charisma
              save_dc: 18
              components_waived:
              - material
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
              cast_level: spell_minimum
            pounce:
              economy: legendary_action
              op: sequence
              steps:
              - op: move
                distance_max:
                  multiply:
                  - stat: speed
                  - numerator: 1
                    denominator: 2
              - op: use_feature
                feature: rend
                count: 1
          skill_totals:
            perception: 13
            stealth: 6
          defenses:
            immunities:
              damage_types:
              - cold
          senses:
            passive_perception: 23
            special:
            - sense: blindsight
              range_feet: 60
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            cost_per_action: 1
            trigger: another_creature.turn.end
            recovery:
              event: turn.start
              participant: self
              amount: all
            maximum_per_trigger: 1
            requires:
              self_incapacitated: false
              self_can_take_actions: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_ancient_white_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - another_creature.turn.end
      - saving_throw.failed
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - legendary_actions
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_ancient_white_dragon
    citations:
    - SRD 5.2.1 pp. 341; builtin/srd-5.2.1/creature/ancient_white_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/animated_object:
    source_sha256: d492cacec1732f44351bd6d50e6f7ea589e32b07158270ed466578f1d5fa769f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/animated_object
        implementation: dnd2024:creature_animated_object
        parameters:
          features:
            slam:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus:
                input: summoner_spell_attack_modifier
              reach_feet: 5
              targets:
                count: 1
              hit:
              - op: switch
                value:
                  stat: size
                cases:
                - values:
                  - tiny
                  - small
                  - medium
                  effects:
                  - op: damage
                    damage_type: force
                    amount:
                      add:
                      - dice:
                          count: 1
                          sides: 4
                        add: 3
                - values:
                  - large
                  effects:
                  - op: damage
                    damage_type: force
                    amount:
                      add:
                      - dice:
                          count: 2
                          sides: 6
                        add: 3
                      - input: summoner_spellcasting_ability_modifier
                - values:
                  - huge
                  effects:
                  - op: damage
                    damage_type: force
                    amount:
                      add:
                      - dice:
                          count: 2
                          sides: 12
                        add: 3
                      - input: summoner_spellcasting_ability_modifier
          defenses:
            immunities:
              damage_types:
              - poison
              - psychic
              conditions:
              - charmed
              - exhaustion
              - frightened
              - paralyzed
              - poisoned
          senses:
            passive_perception: 6
            special:
            - sense: blindsight
              range_feet: 30
          communication:
            speaks: []
            understands:
              languages_known_by: summoner
            can_speak: false
          proficiency_bonus:
            input: summoner_proficiency_bonus
          variant_stats:
            size_choice:
            - tiny
            - small
            - medium
            - large
            - huge
            hit_points_by_size:
              tiny: 10
              small: 10
              medium: 10
              large: 20
              huge: 40
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_animated_object
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
        summoner: participant
        spell_level: integer
        summoner_spell_attack_modifier: integer
        summoner_spell_save_dc: integer
        summoner_spellcasting_ability_modifier: integer
        summoner_proficiency_bonus: integer
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_animated_object
    citations:
    - SRD 5.2.1 pp. 109; builtin/srd-5.2.1/creature/animated_object.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ankylosaurus:
    source_sha256: 2319fc3479321972c83f48945ccd7531cea2b9fe99843739cff7d03c87fe351b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ankylosaurus
        implementation: dnd2024:creature_ankylosaurus
        parameters:
          features:
            tail:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 4
                damage_type: bludgeoning
              - op: if
                test:
                  target_kind: creature
                  target_size_max: huge
                then:
                - op: condition
                  condition: prone
              reach_feet: 10
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: tail
                count: 2
          defenses: {}
          senses:
            passive_perception: 11
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_ankylosaurus
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_ankylosaurus
    citations:
    - SRD 5.2.1 pp. 344; builtin/srd-5.2.1/creature/ankylosaurus.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/archmage:
    source_sha256: 2b8566ceb5db5b4d5c572ace6236beabf00829e21f7bf93bcf850f574f6159d8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/archmage
        implementation: dnd2024:creature_archmage
        parameters:
          features:
            magic_resistance:
              economy: passive
              advantage:
                roll: saving_throw
                against:
                - spell
                - magical_effect
            arcane_burst:
              economy: action
              op: attack
              attack_modes:
              - melee
              - ranged
              attack_bonus: 9
              targets:
                count: 1
              hit:
              - op: damage
                average: 27
                amount:
                  dice:
                    count: 4
                    sides: 10
                  add: 5
                damage_type: force
              reach_feet: 5
              range_feet: 150
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: arcane_burst
                count: 4
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: intelligence
              spells:
              - spell:
                  key: dnd2024:spells/detect_magic
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/detect_thoughts
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/disguise_self
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/invisibility
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/light
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/mage_armor
                frequency: at_will
                precast: true
                included_in_supplied_ac: true
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/mage_hand
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/prestidigitation
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/fly
                frequency: daily
                uses: 2
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/lightning_bolt
                frequency: daily
                uses: 2
                resource_scope: per_spell
                cast_level: 7
                recovery: long_rest
              - spell:
                  key: dnd2024:spells/cone_of_cold
                frequency: daily
                uses: 1
                resource_scope: per_spell
                cast_level: 9
                recovery: long_rest
              - spell:
                  key: dnd2024:spells/mind_blank
                frequency: daily
                uses: 1
                resource_scope: per_spell
                precast: true
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/scrying
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/teleport
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived: []
              save_dc: 17
              required_material_components_available: true
            misty_step:
              economy: bonus_action
              resource:
                capacity: 3
                recovery: long_rest
                cost: 1
              op: cast_spell
              spell:
                key: dnd2024:spells/misty_step
              via: spellcasting
              cast_level: spell_minimum
            protective_magic:
              economy: reaction
              resource:
                capacity: 3
                recovery: long_rest
                cost: 1
              op: cast_spell_choice
              choices:
              - spell:
                  key: dnd2024:spells/counterspell
              - spell:
                  key: dnd2024:spells/shield
              trigger: chosen_spell.trigger
              ability: intelligence
              save_dc: 17
          skill_totals:
            arcana: 13
            history: 9
            perception: 6
          defenses:
            immunities:
              conditional:
              - while_spell_active:
                  key: dnd2024:spells/mind_blank
                damage_types:
                - psychic
                conditions:
                - charmed
          senses:
            passive_perception: 16
            special: []
          gear:
          - item_name: Wand
            quantity: 1
          communication:
            speaks:
            - common
            understands:
            - common
            additional_languages:
              count: 5
              chooser: gm
              distinct_from_known: true
              can_speak: true
              can_understand: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_archmage
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - chosen_spell.trigger
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - movement
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_archmage
    citations:
    - SRD 5.2.1 pp. 305; builtin/srd-5.2.1/creature/archmage.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/awakened_shrub:
    source_sha256: 1c24be475b46a97d25d40441d8919827ae4b20be2b23c1aef6321801a5ada895
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/awakened_shrub
        implementation: dnd2024:creature_awakened_shrub
        parameters:
          features:
            rake:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 1
              targets:
                count: 1
              hit:
              - op: damage
                average: 1
                amount:
                  constant: 1
                damage_type: slashing
              reach_feet: 5
          defenses:
            vulnerabilities:
              damage_types:
              - fire
            resistances:
              damage_types:
              - piercing
          senses:
            passive_perception: 10
            special: []
          communication:
            speaks:
            - common
            understands:
            - common
            additional_languages:
              count: 1
              chooser: gm
              distinct_from_known: true
              can_speak: true
              can_understand: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_awakened_shrub
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_awakened_shrub
    citations:
    - SRD 5.2.1 pp. 260; builtin/srd-5.2.1/creature/awakened_shrub.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/azer_sentinel:
    source_sha256: 11ff563c312e647f94a3adcdbe98c5f5a2405e9a9e8527bb1fa73e6fe3479ad8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/azer_sentinel
        implementation: dnd2024:creature_azer_sentinel
        parameters:
          features:
            burning_hammer:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 3
                damage_type: bludgeoning
              - op: damage
                average: 3
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
                damage_type: fire
              reach_feet: 5
            fire_aura:
              economy: passive
              trigger: self.turn.end
              requires:
                self_incapacitated: false
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  size_feet: 5
                  origin: self
                selection: self_choice
              effects:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 0
                damage_type: fire
            illumination:
              economy: passive
              bright_radius_feet: 10
              dim_additional_feet: 10
          defenses:
            immunities:
              damage_types:
              - fire
              - poison
              conditions:
              - poisoned
          senses:
            passive_perception: 11
            special: []
          communication:
            speaks:
            - primordial
            understands:
            - primordial
            dialects:
            - ignan
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_azer_sentinel
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.turn.end
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_azer_sentinel
    citations:
    - SRD 5.2.1 pp. 261; builtin/srd-5.2.1/creature/azer_sentinel.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/balor:
    source_sha256: d925923482b65dd9c9d1a2a6afe2bb57bf5f46ec2d732c5ba850f4c61d333846
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/balor
        implementation: dnd2024:creature_balor
        parameters:
          features:
            legendary_resistance:
              economy: passive
              resource:
                capacity: 3
                recovery: long_rest
                cost: 1
              trigger: saving_throw.failed
              optional: true
              op: replace_save_outcome
              outcome: success
            magic_resistance:
              economy: passive
              advantage:
                roll: saving_throw
                against:
                - spell
                - magical_effect
            flame_whip:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 14
              targets:
                count: 1
              hit:
              - op: damage
                average: 18
                amount:
                  dice:
                    count: 3
                    sides: 6
                  add: 8
                damage_type: force
              - op: damage
                average: 17
                amount:
                  dice:
                    count: 5
                    sides: 6
                  add: 0
                damage_type: fire
              - op: if
                test:
                  target_kind: creature
                  target_size_max: huge
                then:
                - op: pull
                  distance_max_feet: 25
                  direction: straight_toward_self
                - op: condition
                  condition: prone
              reach_feet: 30
            lightning_blade:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 14
              targets:
                count: 1
              hit:
              - op: damage
                average: 21
                amount:
                  dice:
                    count: 3
                    sides: 8
                  add: 8
                damage_type: force
              - op: damage
                average: 22
                amount:
                  dice:
                    count: 4
                    sides: 10
                  add: 0
                damage_type: lightning
              - op: forbid_economy
                economy:
                - reaction
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
              reach_feet: 10
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: flame_whip
                count: 1
              - op: use_feature
                feature: lightning_blade
                count: 1
            death_throes:
              economy: passive
              trigger: self.died
              op: sequence
              steps:
              - op: save
                ability: dexterity
                dc: 20
                targets:
                  kind: creature
                  count: all
                  area:
                    shape: emanation
                    size_feet: 30
                    origin: self
                failure:
                - op: damage
                  average: 31
                  amount:
                    dice:
                      count: 9
                      sides: 6
                    add: 0
                  damage_type: fire
                - op: damage
                  average: 31
                  amount:
                    dice:
                      count: 9
                      sides: 6
                    add: 0
                  damage_type: force
                success:
                - op: damage_fraction
                  numerator: 1
                  denominator: 2
                  round: down
              - op: if
                test:
                  death_plane_not: abyss
                then:
                - op: revive
                  new_body: true
                  hit_points: maximum
                  delay:
                    seconds: 0
                  destination:
                    plane: abyss
                    location_choice: gm
            fire_aura:
              economy: passive
              trigger: self.turn.end
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  size_feet: 5
                  origin: self
              effects:
              - op: damage
                average: 13
                amount:
                  dice:
                    count: 3
                    sides: 8
                  add: 0
                damage_type: fire
            teleport:
              economy: bonus_action
              op: teleport
              targets:
                choice:
                - self
                - willing_demon
                target_range_feet: 10
              range_feet: 60
              destination:
                unoccupied: true
                visible_to: self
          skill_totals:
            perception: 9
          defenses:
            resistances:
              damage_types:
              - cold
              - lightning
            immunities:
              damage_types:
              - fire
              - poison
              conditions:
              - charmed
              - frightened
              - poisoned
          senses:
            passive_perception: 19
            special:
            - sense: truesight
              range_feet: 120
          communication:
            speaks:
            - abyssal
            understands:
            - abyssal
            telepathy:
              range_feet: 120
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_balor
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - saving_throw.failed
      - self.died
      - self.turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - legendary_actions
      - movement
      - resources
      - targeting
    requires:
    - dnd2024:creature_balor
    citations:
    - SRD 5.2.1 pp. 261; builtin/srd-5.2.1/creature/balor.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/barbed_devil:
    source_sha256: aef9548ae0bd2b0e4075ed94d8532880807935321d556f85a338f12c04ce8951
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/barbed_devil
        implementation: dnd2024:creature_barbed_devil
        parameters:
          features:
            magic_resistance:
              economy: passive
              advantage:
                roll: saving_throw
                against:
                - spell
                - magical_effect
            claws:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 10
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 3
                damage_type: piercing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                then:
                - op: condition
                  condition: grappled
                  escape_dc: 13
                  source: self
                  appendages:
                    kind: claw
                    available: 2
                    used: 2
              reach_feet: 5
            tail:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 14
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 3
                damage_type: slashing
              reach_feet: 10
            hurl_flame:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 17
                amount:
                  dice:
                    count: 5
                    sides: 6
                  add: 0
                damage_type: fire
              - op: if
                test:
                  target_kind: object
                  flammable: true
                  worn: false
                  carried: false
                then:
                - op: ignite
              range_feet: 150
            multiattack:
              economy: action
              op: choose_sequence
              choices:
              - - op: use_feature
                  feature: claws
                  count: 1
                - op: use_feature
                  feature: tail
                  count: 1
              - - op: use_feature
                  feature: hurl_flame
                  count: 2
            diabolical_restoration:
              economy: passive
              resource:
                capacity: 1
                recovery:
                - long_rest
                cost: 1
              trigger: self.died
              requires:
                death_plane_not: nine_hells
              op: revive
              new_body: true
              old_body: disappears
              delay:
                seconds: 0
              hit_points: maximum
              destination:
                plane: nine_hells
                location_choice: gm
            barbed_hide:
              economy: passive
              trigger: self.turn.start
              targets:
                kind: creature
                relation_any:
                - grappled_by_self
                - grappling_self
                count: all
              effects:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 0
                damage_type: piercing
          skill_totals:
            deception: 5
            insight: 5
            perception: 8
          defenses:
            resistances:
              damage_types:
              - cold
            immunities:
              damage_types:
              - fire
              - poison
              conditions:
              - poisoned
          senses:
            passive_perception: 18
            special:
            - sense: darkvision
              range_feet: 120
              magical_darkness_unimpeded: true
          communication:
            speaks:
            - infernal
            understands:
            - infernal
            telepathy:
              range_feet: 120
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_barbed_devil
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.died
      - self.turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - resources
      - targeting
    requires:
    - dnd2024:creature_barbed_devil
    citations:
    - SRD 5.2.1 pp. 262; builtin/srd-5.2.1/creature/barbed_devil.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/bearded_devil:
    source_sha256: 7f629fd01162dd92247948b9bff6c66409a0b33032d7402878349dcc6c1b7286
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/bearded_devil
        implementation: dnd2024:creature_bearded_devil
        parameters:
          features:
            magic_resistance:
              economy: passive
              advantage:
                roll: saving_throw
                against:
                - spell
                - magical_effect
            beard:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 3
                damage_type: piercing
              - op: condition
                condition: poisoned
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
                while_active:
                  regain_hit_points: false
              reach_feet: 5
            infernal_glaive:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 3
                damage_type: slashing
              - op: if
                test:
                  target_kind: creature
                  target_has_infernal_wound: false
                then:
                - op: save
                  ability: constitution
                  dc: 12
                  targets:
                    participant: target
                  failure:
                  - op: infernal_wound
                    stacking: one_per_target
                    periodic:
                      event: target.turn.start
                      effects:
                      - op: lose_hit_points
                        average: 5
                        amount:
                          dice:
                            count: 1
                            sides: 10
                          add: 0
                    ends_any:
                    - minutes: 1
                    - event: spell.restores_hit_points
                      target: affected
                    - economy: action
                      actor: target_or_creature_within_5_feet
                      check:
                        ability: wisdom
                        skill: medicine
                        dc: 12
                      outcome: success
                  success: []
              reach_feet: 10
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: beard
                count: 1
              - op: use_feature
                feature: infernal_glaive
                count: 1
          defenses:
            resistances:
              damage_types:
              - cold
            immunities:
              damage_types:
              - fire
              - poison
              conditions:
              - frightened
              - poisoned
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 120
              magical_darkness_unimpeded: true
          communication:
            speaks:
            - infernal
            understands:
            - infernal
            telepathy:
              range_feet: 120
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_bearded_devil
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - spell.restores_hit_points
      - target.turn.start
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_bearded_devil
    citations:
    - SRD 5.2.1 pp. 262, 263; builtin/srd-5.2.1/creature/bearded_devil.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/black_bear:
    source_sha256: b990c17c627e4d47a1714c1463d901f771eae33020bf8e2f1953f1fb83465af0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/black_bear
        implementation: dnd2024:creature_black_bear
        parameters:
          features:
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 2
                damage_type: slashing
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 2
          skill_totals:
            perception: 5
          defenses: {}
          senses:
            passive_perception: 15
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_black_bear
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_black_bear
    citations:
    - SRD 5.2.1 pp. 345; builtin/srd-5.2.1/creature/black_bear.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/blink_dog:
    source_sha256: b0c1dabab24f405b9f81e3fa3599ffcb93fc694567f2ddfa8d8ed571069ed84c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/blink_dog
        implementation: dnd2024:creature_blink_dog
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 3
                damage_type: piercing
              reach_feet: 5
            teleport:
              economy: bonus_action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 4
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: teleport
              targets: self
              range_feet: 40
              destination:
                unoccupied: true
                visible_to: self
          skill_totals:
            perception: 5
            stealth: 5
          defenses: {}
          senses:
            passive_perception: 15
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - blink_dog
            understands:
            - blink_dog
            - elvish
            - sylvan
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_blink_dog
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - movement
      - resources
      - targeting
    requires:
    - dnd2024:creature_blink_dog
    citations:
    - SRD 5.2.1 pp. 266; builtin/srd-5.2.1/creature/blink_dog.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/boar:
    source_sha256: b2f141b2c92f1b5701d343665bec1920da0ecfdf7b5a8cafd0f34c525976ac35
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/boar
        implementation: dnd2024:creature_boar
        parameters:
          features:
            gore:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 3
              targets:
                count: 1
              hit:
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 1
                damage_type: piercing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: medium
                  immediately_preceding_movement:
                    minimum_feet: 20
                    direction: straight_toward_target
                then:
                - op: damage
                  average: 3
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  damage_type: piercing
                - op: condition
                  condition: prone
              reach_feet: 5
            bloodied_fury:
              economy: passive
              advantage:
                roll: attack
                when: self_bloodied
          defenses: {}
          senses:
            passive_perception: 9
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_boar
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_boar
    citations:
    - SRD 5.2.1 pp. 346; builtin/srd-5.2.1/creature/boar.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/bronze_dragon_wyrmling:
    source_sha256: ac165f5886f9894b24d2ce033794256bacb81085e64460a5b0ee278af94dc668
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/bronze_dragon_wyrmling
        implementation: dnd2024:creature_bronze_dragon_wyrmling
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 3
                damage_type: slashing
              reach_feet: 5
            lightning_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 12
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  size_feet: 40
                  origin: self
                  width_feet: 5
              failure:
              - op: damage
                average: 16
                amount:
                  dice:
                    count: 3
                    sides: 10
                  add: 0
                damage_type: lightning
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 2
            repulsion_breath:
              economy: action
              op: save
              ability: strength
              dc: 12
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 30
                  origin: self
              failure:
              - op: push
                distance_max_feet: 30
                direction: straight_away_from_self
              - op: condition
                condition: prone
              success: []
          skill_totals:
            perception: 4
            stealth: 2
          defenses:
            immunities:
              damage_types:
              - lightning
          senses:
            passive_perception: 14
            special:
            - sense: blindsight
              range_feet: 10
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - draconic
            understands:
            - draconic
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_bronze_dragon_wyrmling
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_bronze_dragon_wyrmling
    citations:
    - SRD 5.2.1 pp. 269, 270; builtin/srd-5.2.1/creature/bronze_dragon_wyrmling.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/bugbear_warrior:
    source_sha256: 966a49949e63cac104748b8aab0180059b105741a7d3dadf982049b14f5d2478
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/bugbear_warrior
        implementation: dnd2024:creature_bugbear_warrior
        parameters:
          features:
            grab:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 2
                damage_type: bludgeoning
              - op: if
                test:
                  target_kind: creature
                  target_size_max: medium
                then:
                - op: condition
                  condition: grappled
                  escape_dc: 12
                  source: self
              reach_feet: 10
            light_hammer:
              economy: action
              op: attack
              attack_modes:
              - melee
              - ranged
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 3
                    sides: 4
                  add: 2
                damage_type: bludgeoning
              reach_feet: 10
              range_feet: 20
              long_range_feet: 60
              advantage_when:
                target_grappled_by: self
            abduct:
              economy: passive
              move_grappled_creature_extra_cost: 0
          skill_totals:
            stealth: 6
            survival: 2
          defenses: {}
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 60
          gear:
          - item_name: Hide Armor
            quantity: 1
          - item_name: Light Hammers
            quantity: 3
          communication:
            speaks:
            - common
            - goblin
            understands:
            - common
            - goblin
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_bugbear_warrior
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - resources
      - targeting
    requires:
    - dnd2024:creature_bugbear_warrior
    citations:
    - SRD 5.2.1 pp. 272; builtin/srd-5.2.1/creature/bugbear_warrior.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/cat:
    source_sha256: 9ee7c0088dbeb623b7b47e9243c3eb84de081cec46948563c17541354da799e6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/cat
        implementation: dnd2024:creature_cat
        parameters:
          features:
            scratch:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 1
                amount:
                  constant: 1
                damage_type: slashing
              reach_feet: 5
            jumper:
              economy: passive
              jump_distance_ability: dexterity
          skill_totals:
            perception: 3
            stealth: 4
          defenses: {}
          senses:
            passive_perception: 13
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_cat
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_cat
    citations:
    - SRD 5.2.1 pp. 346; builtin/srd-5.2.1/creature/cat.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/chimera:
    source_sha256: 4191504e3d35812003f595f402b8bc7e188833ba108a813f8433a7488c1ab560
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/chimera
        implementation: dnd2024:creature_chimera
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: choose_by_predicate
                test:
                  attack_had_advantage: true
                then:
                - op: damage
                  average: 18
                  amount:
                    dice:
                      count: 4
                      sides: 6
                    add: 4
                  damage_type: piercing
                else:
                - op: damage
                  average: 11
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 4
                  damage_type: piercing
              reach_feet: 5
            claw:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 4
                damage_type: slashing
              reach_feet: 5
            ram:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 10
                amount:
                  dice:
                    count: 1
                    sides: 12
                  add: 4
                damage_type: bludgeoning
              - op: if
                test:
                  target_kind: creature
                  target_size_max: medium
                then:
                - op: condition
                  condition: prone
              reach_feet: 5
            fire_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 15
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 15
                  origin: self
              failure:
              - op: damage
                average: 31
                amount:
                  dice:
                    count: 7
                    sides: 8
                  add: 0
                damage_type: fire
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: ram
                count: 1
              - op: use_feature
                feature: bite
                count: 1
              - op: choose
                choices:
                - op: use_feature
                  feature: claw
                  count: 1
                - op: use_feature
                  feature: fire_breath
                  count: 1
          skill_totals:
            perception: 8
          defenses: {}
          senses:
            passive_perception: 18
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands:
            - draconic
            can_speak: false
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_chimera
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_chimera
    citations:
    - SRD 5.2.1 pp. 273; builtin/srd-5.2.1/creature/chimera.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/cloaker:
    source_sha256: 0a1a3dcb136f1cf630a55d5a89cdbd3ea8ffeff017dde8f912ff46edaab85e52
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/cloaker
        implementation: dnd2024:creature_cloaker
        parameters:
          features:
            light_sensitivity:
              economy: passive
              disadvantage:
                rolls:
                - attack
                when: self_in_bright_light
            attach:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 13
                amount:
                  dice:
                    count: 3
                    sides: 6
                  add: 3
                damage_type: piercing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                then:
                - op: attach
                  effects_on_target:
                  - op: condition
                    condition: blinded
                  self_attack_restriction:
                    feature: attach
                    targets: attached_target_only
                  damage_taken:
                    self_fraction:
                      numerator: 1
                      denominator: 2
                      round: down
                    attached_target_damage: same_amount_as_self
                  detach_options:
                  - actor: self
                    movement_cost_feet: 5
                  - actor: target_or_creature_within_5_feet
                    economy: action
                    check:
                      ability: strength
                      skill: athletics
                      dc: 14
                    success: detach
              reach_feet: 5
            tail:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 3
                damage_type: slashing
              reach_feet: 10
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: attach
                count: 1
              - op: use_feature
                feature: tail
                count: 2
            phantasms:
              economy: bonus_action
              resource:
                capacity: 1
                recovery:
                - short_rest
                - long_rest
                cost: 1
              op: cast_spell
              spell:
                key: dnd2024:spells/mirror_image
              ability: wisdom
              components_waived:
              - verbal
              - somatic
              - material
              ends_early:
                events:
                - self.turn.start
                - self.turn.end
                when: self_in_bright_light
              cast_level: spell_minimum
            moan:
              economy: bonus_action
              op: save
              ability: wisdom
              dc: 13
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  size_feet: 60
                  origin: self
              failure:
              - op: condition
                condition: frightened
                ends:
                  event: turn.end
                  participant: self
                  occurrence: next
              success:
              - op: feature_immunity
                source: self
                feature: moan
                duration:
                  hours: 24
          skill_totals:
            stealth: 5
          defenses:
            immunities:
              conditions:
              - frightened
          senses:
            passive_perception: 12
            special:
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - deep_speech
            - undercommon
            understands:
            - deep_speech
            - undercommon
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_cloaker
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.end
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_cloaker
    citations:
    - SRD 5.2.1 pp. 274; builtin/srd-5.2.1/creature/cloaker.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/commoner:
    source_sha256: 464c18b41cf62ce6a8db2b72b401164409e66f17e7080d15c5fb490b573ac7b0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/commoner
        implementation: dnd2024:creature_commoner
        parameters:
          features:
            club:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 2
              targets:
                count: 1
              hit:
              - op: damage
                average: 2
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 0
                damage_type: bludgeoning
              reach_feet: 5
            training:
              economy: passive
              skill_choice:
                chooser: gm
                count: 1
                from: skills
              grant_skill_proficiency: true
              advantage:
                roll: ability_check
                skill: chosen_skill
          defenses: {}
          senses:
            passive_perception: 10
            special: []
          gear:
          - item_name: Club
            quantity: 1
          communication:
            speaks:
            - common
            understands:
            - common
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_commoner
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_commoner
    citations:
    - SRD 5.2.1 pp. 275; builtin/srd-5.2.1/creature/commoner.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/couatl:
    source_sha256: d1c1f7a41b828c5ccd36552a86a6d6d150211233e07c73802164fedad638203f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/couatl
        implementation: dnd2024:creature_couatl
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 11
                amount:
                  dice:
                    count: 1
                    sides: 12
                  add: 5
                damage_type: piercing
              - op: condition
                condition: poisoned
                ends:
                  event: turn.end
                  participant: self
                  occurrence: next
              reach_feet: 5
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: wisdom
              spells:
              - spell:
                  key: dnd2024:spells/detect_evil_and_good
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/detect_magic
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/detect_thoughts
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/shapechange
                frequency: at_will
                overrides:
                  form_types:
                  - beast
                  - humanoid
                  temporary_hit_points_gained: 0
                  concentration_required: false
                  temporary_hit_points_to_maintain_required: false
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/create_food_and_water
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/dream
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/greater_restoration
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/scrying
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/sleep
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - verbal
              - somatic
              - material
              save_dc: 15
            shielded_mind:
              economy: passive
              thought_reading_allowed: false
              incoming_telepathy_requires_self_permission: true
            divine_aid:
              economy: bonus_action
              resource:
                capacity: 2
                cost: 1
                recovery: long_rest
                scope: shared_choices
              op: cast_spell_choice
              choices:
              - spell:
                  key: dnd2024:spells/bless
              - spell:
                  key: dnd2024:spells/lesser_restoration
              - spell:
                  key: dnd2024:spells/sanctuary
              ability: wisdom
              save_dc: 15
              components_waived:
              - verbal
              - somatic
              - material
            constrict:
              economy: action
              op: save
              ability: strength
              dc: 15
              targets:
                kind: creature
                count: 1
                range_feet: 5
                visible_to: self
                size_max: medium
              failure:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 5
                damage_type: bludgeoning
              - op: condition
                condition: grappled
                escape_dc: 13
                source: self
              - op: condition
                condition: restrained
                ends:
                  event: grapple.end
                  source: self
              success: []
          defenses:
            resistances:
              damage_types:
              - bludgeoning
              - piercing
              - slashing
            immunities:
              damage_types:
              - psychic
              - radiant
          senses:
            passive_perception: 15
            special:
            - sense: truesight
              range_feet: 120
          communication:
            speaks: all
            understands: all
            telepathy:
              range_feet: 120
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_couatl
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - grapple.end
      - turn.end
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_couatl
    citations:
    - SRD 5.2.1 pp. 277, 278; builtin/srd-5.2.1/creature/couatl.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/cultist:
    source_sha256: e0adc85e4b44e9d25db449485f2a56a3cab690f67b5b35d1945f8a64d6784cc7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/cultist
        implementation: dnd2024:creature_cultist
        parameters:
          features:
            ritual_sickle:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 3
              targets:
                count: 1
              hit:
              - op: damage
                average: 3
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 1
                damage_type: slashing
              - op: damage
                average: 1
                amount:
                  constant: 1
                damage_type: necrotic
              reach_feet: 5
          skill_totals:
            deception: 2
            religion: 2
          defenses: {}
          senses:
            passive_perception: 10
            special: []
          gear:
          - item_name: Leather Armor
            quantity: 1
          - item_name: Sickle
            quantity: 1
          communication:
            speaks:
            - common
            understands:
            - common
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_cultist
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_cultist
    citations:
    - SRD 5.2.1 pp. 278; builtin/srd-5.2.1/creature/cultist.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/death_dog:
    source_sha256: c5d29335daf0cba8bef34ab95547b5dbf91ac4865e190b1349dec7c8d4b0eb4d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/death_dog
        implementation: dnd2024:creature_death_dog
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 2
                damage_type: piercing
              - op: if
                test:
                  target_kind: creature
                then:
                - op: save
                  ability: constitution
                  dc: 12
                  targets:
                    participant: target
                  failure:
                  - op: choose_by_predicate
                    test:
                      target_has_source_poison_effect: true
                    then:
                    - op: reduce_hit_point_maximum
                      average: 5
                      amount:
                        dice:
                          count: 1
                          sides: 10
                        add: 0
                    else:
                    - op: condition
                      condition: poisoned
                      ends:
                        event: repeated_save.success
                      while_active:
                        long_rest_restores_hp_maximum: false
                        repeat_save:
                          ability: constitution
                          dc: 12
                          interval:
                            hours: 24
                          success: end_effect
                          failure:
                          - op: reduce_hit_point_maximum
                            average: 5
                            amount:
                              dice:
                                count: 1
                                sides: 10
                              add: 0
                  success: []
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: bite
                count: 2
          skill_totals:
            perception: 5
            stealth: 4
          defenses:
            immunities:
              conditions:
              - blinded
              - charmed
              - deafened
              - frightened
              - stunned
              - unconscious
          senses:
            passive_perception: 15
            special:
            - sense: darkvision
              range_feet: 120
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_death_dog
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - repeated_save.success
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_death_dog
    citations:
    - SRD 5.2.1 pp. 279; builtin/srd-5.2.1/creature/death_dog.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/dire_wolf:
    source_sha256: 51e5b82bfef02c58198bbc7721e0c11603365a0b61b3c883830666c0444cabff
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/dire_wolf
        implementation: dnd2024:creature_dire_wolf
        parameters:
          features:
            pack_tactics:
              economy: passive
              advantage:
                roll: attack
                target_kind: creature
                requires:
                  ally_distance_to_target_max_feet: 5
                  ally_incapacitated: false
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 3
                damage_type: piercing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                then:
                - op: condition
                  condition: prone
              reach_feet: 5
          skill_totals:
            perception: 5
            stealth: 4
          defenses: {}
          senses:
            passive_perception: 15
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_dire_wolf
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_dire_wolf
    citations:
    - SRD 5.2.1 pp. 347; builtin/srd-5.2.1/creature/dire_wolf.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/draconic_spirit:
    source_sha256: a61cb5fc7440f99f19fcb986f66d7323726fce4750ade60da82f0f24a79c296a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/draconic_spirit
        implementation: dnd2024:creature_draconic_spirit
        parameters:
          features:
            shared_resistances:
              economy: passive
              trigger: summon.created
              choice:
                chooser: summoner
                options:
                - acid
                - cold
                - fire
                - lightning
                - poison
                count: 1
              grant_to: summoner
              resistance: chosen_type
              ends:
                event: summoning_spell.end
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus:
                input: summoner_spell_attack_modifier
              reach_feet: 10
              targets:
                count: 1
              hit:
              - op: damage
                damage_type: piercing
                amount:
                  add:
                  - dice:
                      count: 1
                      sides: 6
                    add: 4
                  - input: spell_level
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count:
                  floor_divide:
                  - input: spell_level
                  - 2
              - op: use_feature
                feature: breath_weapon
                count: 1
            breath_weapon:
              economy: action
              damage_type_choice:
                chooser: summoner
                timing: spell.cast
                options:
                - acid
                - cold
                - fire
                - lightning
                - poison
              op: save
              ability: dexterity
              dc:
                input: summoner_spell_save_dc
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 30
                  origin: self
              failure:
              - op: damage
                damage_type:
                  choice: breath_damage_type
                amount:
                  add:
                  - dice:
                      count: 2
                      sides: 6
                    add: 0
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
          defenses:
            resistances:
              damage_types:
              - acid
              - cold
              - fire
              - lightning
              - poison
            immunities:
              conditions:
              - charmed
              - frightened
              - poisoned
          senses:
            passive_perception: 12
            special:
            - sense: blindsight
              range_feet: 30
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - draconic
            understands:
              union:
              - - draconic
              - languages_known_by: summoner
          proficiency_bonus:
            input: summoner_proficiency_bonus
          scaling_stats:
            ac:
              add:
              - 14
              - input: spell_level
            hit_points:
              add:
              - 50
              - multiply:
                - 10
                - subtract:
                  - input: spell_level
                  - 5
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_draconic_spirit
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
        summoner: participant
        spell_level: integer
        summoner_spell_attack_modifier: integer
        summoner_spell_save_dc: integer
        summoner_spellcasting_ability_modifier: integer
        summoner_proficiency_bonus: integer
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - summon.created
      - summoning_spell.end
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_draconic_spirit
    citations:
    - SRD 5.2.1 pp. 166; builtin/srd-5.2.1/creature/draconic_spirit.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/dretch:
    source_sha256: 7f301e74d8aec1113177a94dc0784517a0f9a58a2c824d66a4ac924d4ffaeb14
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/dretch
        implementation: dnd2024:creature_dretch
        parameters:
          features:
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 3
              targets:
                count: 1
              hit:
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 1
                damage_type: slashing
              reach_feet: 5
            fetid_cloud:
              economy: action
              resource:
                capacity: 1
                recovery: long_rest
                cost: 1
              op: save
              ability: constitution
              dc: 11
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  size_feet: 10
                  origin: self
              failure:
              - op: condition
                condition: poisoned
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
                while_active:
                  turn_economy_exclusive_choice:
                  - action
                  - bonus_action
                  reactions_allowed: false
              success: []
          defenses:
            resistances:
              damage_types:
              - cold
              - fire
              - lightning
            immunities:
              damage_types:
              - poison
              conditions:
              - poisoned
          senses:
            passive_perception: 9
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - abyssal
            understands:
            - abyssal
            telepathy:
              range_feet: 60
              target_understands: abyssal
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_dretch
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.end
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_dretch
    citations:
    - SRD 5.2.1 pp. 281; builtin/srd-5.2.1/creature/dretch.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/dryad:
    source_sha256: e993b54b9236a4e039e839f371003e0b6748d2ce70293164096cc6a63d6884a0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/dryad
        implementation: dnd2024:creature_dryad
        parameters:
          features:
            magic_resistance:
              economy: passive
              advantage:
                roll: saving_throw
                against:
                - spell
                - magical_effect
            vine_lash:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 4
                damage_type: slashing
              reach_feet: 10
            thorn_burst:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 4
                damage_type: piercing
              range_feet: 60
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: choose
                choices:
                - op: use_feature
                  feature: vine_lash
                  count: 1
                - op: use_feature
                  feature: thorn_burst
                  count: 1
              - op: optional
                effect:
                  op: cast_spell
                  spell:
                    key: dnd2024:spells/charm_monster
                  via: spellcasting
                  cast_level: spell_minimum
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: charisma
              spells:
              - spell:
                  key: dnd2024:spells/animal_friendship
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/charm_monster
                frequency: at_will
                overrides:
                  duration:
                    hours: 24
                  ends_on: self_casts_same_spell_again
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/druidcraft
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/entangle
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/pass_without_trace
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - material
              save_dc: 14
            speak_with_beasts_and_plants:
              economy: passive
              communication:
                as_shared_language: true
                creature_types:
                - beast
                - plant
            tree_stride:
              economy: bonus_action
              op: teleport
              requires:
                tree_1_size_min: large
                tree_1_distance_max_feet: 5
              destination:
                unoccupied: true
                tree_2_size_min: large
                tree_2_distance_max_feet: 5
                tree_2_different_from_tree_1: true
                tree_2_distance_from_tree_1_max_feet: 60
          skill_totals:
            perception: 4
            stealth: 5
          defenses: {}
          senses:
            passive_perception: 14
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - elvish
            - sylvan
            understands:
            - elvish
            - sylvan
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_dryad
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - movement
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_dryad
    citations:
    - SRD 5.2.1 pp. 282; builtin/srd-5.2.1/creature/dryad.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/earth_elemental:
    source_sha256: b91ece571ec31d284385b109ec58929e5699bd467fd8daeed99682e0095fc4bd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/earth_elemental
        implementation: dnd2024:creature_earth_elemental
        parameters:
          features:
            siege_monster:
              economy: passive
              damage_multiplier: 2
              targets:
              - object
              - structure
            slam:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 8
              targets:
                count: 1
              hit:
              - op: damage
                average: 14
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 5
                damage_type: bludgeoning
              reach_feet: 10
            rock_launch:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 8
              targets:
                count: 1
              hit:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 5
                damage_type: bludgeoning
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                then:
                - op: condition
                  condition: prone
              range_feet: 60
            multiattack:
              economy: action
              op: repeat_choice
              count: 2
              choices:
              - feature: slam
              - feature: rock_launch
            earth_glide:
              economy: passive
              burrow_through:
                materials:
                - earth
                - stone
                magical: false
                worked: false
              disturbs_material: false
          defenses:
            vulnerabilities:
              damage_types:
              - thunder
            immunities:
              damage_types:
              - poison
              conditions:
              - exhaustion
              - paralyzed
              - petrified
              - poisoned
              - unconscious
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 60
            - sense: tremorsense
              range_feet: 60
          communication:
            speaks:
            - primordial
            understands:
            - primordial
            dialects:
            - terran
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_earth_elemental
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_earth_elemental
    citations:
    - SRD 5.2.1 pp. 282, 283; builtin/srd-5.2.1/creature/earth_elemental.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/elk:
    source_sha256: da6f9d7eb3f7f3468ac021e98c43057ae136498d9f4483221ee0537838119b98
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/elk
        implementation: dnd2024:creature_elk
        parameters:
          features:
            ram:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 3
                damage_type: bludgeoning
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                  immediately_preceding_movement:
                    minimum_feet: 20
                    direction: straight_toward_target
                then:
                - op: damage
                  average: 3
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  damage_type: bludgeoning
                - op: condition
                  condition: prone
              reach_feet: 5
          skill_totals:
            perception: 2
          defenses: {}
          senses:
            passive_perception: 12
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_elk
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_elk
    citations:
    - SRD 5.2.1 pp. 348; builtin/srd-5.2.1/creature/elk.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ettin:
    source_sha256: ffb46f6f22dbd73dc9688792eb759a3e808eefb139a899aea0fe1738e73e1f37
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ettin
        implementation: dnd2024:creature_ettin
        parameters:
          features:
            battleaxe:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 14
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 5
                damage_type: slashing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                then:
                - op: condition
                  condition: prone
              reach_feet: 5
            morningstar:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 14
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 5
                damage_type: piercing
              - op: disadvantage
                roll: attack
                count: 1
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: battleaxe
                count: 1
              - op: use_feature
                feature: morningstar
                count: 1
          skill_totals:
            perception: 4
          defenses:
            immunities:
              conditions:
              - blinded
              - charmed
              - deafened
              - frightened
              - stunned
              - unconscious
          senses:
            passive_perception: 14
            special:
            - sense: darkvision
              range_feet: 60
          gear:
          - item_name: Battleaxe
            quantity: 1
          - item_name: Morningstar
            quantity: 1
          communication:
            speaks:
            - giant
            understands:
            - giant
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_ettin
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.end
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_ettin
    citations:
    - SRD 5.2.1 pp. 284; builtin/srd-5.2.1/creature/ettin.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/flesh_golem:
    source_sha256: 6ee0d3cd00af05c264334b722d1600273a2f2b776d9a86f49e3bf668d13af163
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/flesh_golem
        implementation: dnd2024:creature_flesh_golem
        parameters:
          features:
            immutable_form:
              economy: passive
              shape_shift_allowed: false
            magic_resistance:
              economy: passive
              advantage:
                roll: saving_throw
                against:
                - spell
                - magical_effect
            slam:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 13
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 4
                damage_type: bludgeoning
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 0
                damage_type: lightning
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: slam
                count: 2
            lightning_absorption:
              economy: passive
              trigger: damage.subjected
              damage_type: lightning
              op: heal
              amount:
                event_value: incoming_damage_of_matching_type
            aversion_to_fire:
              economy: passive
              trigger: self.damage_taken
              requires:
                damage_type: fire
              disadvantage:
                rolls:
                - attack
                - ability_check
                ends:
                  event: turn.end
                  participant: self
                  occurrence: next
            berserk:
              economy: passive
              trigger: self.turn.start
              requires:
                self_bloodied: true
              activation_roll:
                roll:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
                success:
                - 6
              state: berserk
              while_active:
                turn_behavior:
                  attack_target: nearest_visible_creature
                  requires_reachable_with_move_and_attack: true
                  fallback_target_kind: object
              ends_any:
              - self_destroyed: true
              - self_bloodied: false
              suppress:
                actor: creator
                economy: action
                range_feet: 60
                self_can_hear_creator: true
                check:
                  ability: charisma
                  skill: persuasion
                  dc: 15
                on_success:
                  suppress_until:
                    event: turn.start
                    participant: self
                    occurrence: next
                  resume_activation_rolls_if_bloodied: true
          defenses:
            immunities:
              damage_types:
              - lightning
              - poison
              conditions:
              - charmed
              - exhaustion
              - frightened
              - paralyzed
              - petrified
              - poisoned
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands:
            - common
            additional_languages:
              count: 1
              chooser: gm
              distinct_from_known: true
              can_speak: false
              can_understand: true
            can_speak: false
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_flesh_golem
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - damage.subjected
      - self.damage_taken
      - self.turn.start
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - hit_points
      - resources
      - shapechanging
      - targeting
    requires:
    - dnd2024:creature_flesh_golem
    citations:
    - SRD 5.2.1 pp. 285; builtin/srd-5.2.1/creature/flesh_golem.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/frost_giant:
    source_sha256: 30691829df452658c3789f39a88b7f8ebf76a0f81ce926663e84294acb552ac0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/frost_giant
        implementation: dnd2024:creature_frost_giant
        parameters:
          features:
            frost_axe:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 9
              targets:
                count: 1
              hit:
              - op: damage
                average: 19
                amount:
                  dice:
                    count: 2
                    sides: 12
                  add: 6
                damage_type: slashing
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 0
                damage_type: cold
              reach_feet: 10
            great_bow:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 9
              targets:
                count: 1
              hit:
              - op: damage
                average: 17
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 6
                damage_type: piercing
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: cold
              - op: modify_speed
                add_feet: -10
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
              range_feet: 150
              long_range_feet: 600
            multiattack:
              economy: action
              op: repeat_choice
              count: 2
              choices:
              - feature: frost_axe
              - feature: great_bow
            war_cry:
              economy: bonus_action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              targets:
                choice:
                - self
                - one_creature_that_can_see_or_hear_self
              effects:
              - op: temporary_hit_points
                average: 16
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 5
              - op: advantage
                roll: attack
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
          skill_totals:
            athletics: 9
            perception: 3
          defenses:
            immunities:
              damage_types:
              - cold
          senses:
            passive_perception: 13
            special: []
          communication:
            speaks:
            - giant
            understands:
            - giant
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_frost_giant
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_frost_giant
    citations:
    - SRD 5.2.1 pp. 285, 286; builtin/srd-5.2.1/creature/frost_giant.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ghast:
    source_sha256: d67eaca551d360a1a23648c572076ac9623dad43a92ce239fe32e097f0354063
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ghast
        implementation: dnd2024:creature_ghast
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 3
                damage_type: piercing
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 0
                damage_type: necrotic
              reach_feet: 5
            claw:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 10
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 3
                damage_type: slashing
              - op: if
                test:
                  target_kind: creature
                  target_type_not: undead
                then:
                - op: save
                  ability: constitution
                  dc: 10
                  targets:
                    participant: target
                  failure:
                  - op: condition
                    condition: paralyzed
                    ends:
                      event: turn.end
                      participant: target
                      occurrence: next
                  success: []
              reach_feet: 5
            stench:
              economy: passive
              trigger: creature.turn.start
              op: save
              ability: constitution
              dc: 10
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  size_feet: 5
                  origin: self
              failure:
              - op: condition
                condition: poisoned
                ends:
                  event: turn.start
                  participant: target
                  occurrence: next
              success:
              - op: feature_immunity
                feature: stench
                source: self
                duration:
                  hours: 24
          defenses:
            resistances:
              damage_types:
              - necrotic
            immunities:
              damage_types:
              - poison
              conditions:
              - charmed
              - exhaustion
              - poisoned
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - common
            understands:
            - common
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_ghast
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - creature.turn.start
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_ghast
    citations:
    - SRD 5.2.1 pp. 287; builtin/srd-5.2.1/creature/ghast.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_ape:
    source_sha256: 8cc4775c27f2655390da8836cabd1fc2e64d07c91b6ff250aeec50af7467d2f7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_ape
        implementation: dnd2024:creature_giant_ape
        parameters:
          features:
            fist:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 9
              targets:
                count: 1
              hit:
              - op: damage
                average: 22
                amount:
                  dice:
                    count: 3
                    sides: 10
                  add: 6
                damage_type: bludgeoning
              reach_feet: 10
            leap:
              economy: bonus_action
              op: jump
              distance_max_feet: 30
              movement_cost_feet: 10
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: fist
                count: 2
            boulder_toss:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 17
              targets:
                kind: creature
                count: all
                area:
                  shape: sphere
                  size_feet: 5
                  origin: chosen_point
                  origin_visible_to: self
                  origin_range_feet: 90
              failure:
              - op: damage
                average: 24
                amount:
                  dice:
                    count: 7
                    sides: 6
                  add: 0
                damage_type: bludgeoning
              - op: if
                test:
                  target_size_max: large
                then:
                - op: condition
                  condition: prone
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
          skill_totals:
            athletics: 9
            perception: 4
            survival: 4
          defenses: {}
          senses:
            passive_perception: 14
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_giant_ape
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_giant_ape
    citations:
    - SRD 5.2.1 pp. 349; builtin/srd-5.2.1/creature/giant_ape.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_boar:
    source_sha256: 6113f0302e9b3703e5737d0b1584f7e109156f2154e0deb5ee4542c3ab6716b2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_boar
        implementation: dnd2024:creature_giant_boar
        parameters:
          features:
            gore:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 10
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 3
                damage_type: piercing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                  immediately_preceding_movement:
                    minimum_feet: 20
                    direction: straight_toward_target
                then:
                - op: damage
                  average: 7
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 0
                  damage_type: piercing
                - op: condition
                  condition: prone
              reach_feet: 5
            bloodied_fury:
              economy: passive
              advantage:
                roll: melee_attack
                when: self_bloodied
          defenses: {}
          senses:
            passive_perception: 8
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_giant_boar
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_giant_boar
    citations:
    - SRD 5.2.1 pp. 349; builtin/srd-5.2.1/creature/giant_boar.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_crab:
    source_sha256: 638f984fecd5afe43deea685e7b9f950a142456a659ca5c27b8668b1971cbfd4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_crab
        implementation: dnd2024:creature_giant_crab
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            claw:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 3
              targets:
                count: 1
              hit:
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 1
                damage_type: bludgeoning
              - op: if
                test:
                  target_kind: creature
                  target_size_max: medium
                then:
                - op: condition
                  condition: grappled
                  escape_dc: 11
                  source: self
                  appendages:
                    kind: claw
                    available: 2
                    used: 1
              reach_feet: 5
          skill_totals:
            stealth: 3
          defenses: {}
          senses:
            passive_perception: 9
            special:
            - sense: blindsight
              range_feet: 30
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_giant_crab
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - resources
      - targeting
    requires:
    - dnd2024:creature_giant_crab
    citations:
    - SRD 5.2.1 pp. 350; builtin/srd-5.2.1/creature/giant_crab.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_elk:
    source_sha256: 186b03521b38aec9f844ad837126f19c96ba34faeed2065976d6ac3911087327
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_elk
        implementation: dnd2024:creature_giant_elk
        parameters:
          features:
            ram:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 11
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 4
                damage_type: bludgeoning
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 2
                    sides: 4
                  add: 0
                damage_type: radiant
              - op: if
                test:
                  target_kind: creature
                  target_size_max: huge
                  immediately_preceding_movement:
                    minimum_feet: 20
                    direction: straight_toward_target
                then:
                - op: damage
                  average: 5
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 0
                  damage_type: bludgeoning
                - op: condition
                  condition: prone
              reach_feet: 10
          skill_totals:
            perception: 4
          defenses:
            resistances:
              damage_types:
              - necrotic
              - radiant
          senses:
            passive_perception: 14
            special:
            - sense: darkvision
              range_feet: 90
          communication:
            speaks:
            - celestial
            understands:
            - celestial
            - common
            - elvish
            - sylvan
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_giant_elk
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_giant_elk
    citations:
    - SRD 5.2.1 pp. 351; builtin/srd-5.2.1/creature/giant_elk.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_frog:
    source_sha256: d33e8d7d0fe0aecfb09d9ff82d1b2ff8bd09696f6e7cf30dc84e446cfa52e33d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_frog
        implementation: dnd2024:creature_giant_frog
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            standing_leap:
              economy: passive
              long_jump_max_feet: 20
              high_jump_max_feet: 10
              running_start_required: false
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 3
              targets:
                count: 1
              hit:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 2
                damage_type: piercing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: medium
                then:
                - op: condition
                  condition: grappled
                  escape_dc: 11
                  source: self
              reach_feet: 5
            swallow:
              economy: action
              op: swallow
              targets:
                count: 1
                grappled_by: self
                size_max: small
              on_swallow:
              - op: remove_condition
                condition: grappled
              - op: condition
                condition: blinded
              - op: condition
                condition: restrained
              while_swallowed:
                total_cover_against: external_attacks_and_effects
                self_cannot_use:
                - bite
              self_death:
                remove_conditions:
                - restrained
                escape:
                  movement_cost_feet: 5
                  exit_condition: prone
              digest:
                event: self.turn.end
                occurrence: next
                effects:
                - op: damage
                  average: 5
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 0
                  damage_type: acid
                after_damage:
                - op: if
                  test:
                    target_killed: false
                  then:
                  - op: disgorge
                    exit_condition: prone
          skill_totals:
            perception: 2
            stealth: 4
          defenses: {}
          senses:
            passive_perception: 12
            special:
            - sense: darkvision
              range_feet: 30
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_giant_frog
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.turn.end
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - resources
      - targeting
    requires:
    - dnd2024:creature_giant_frog
    citations:
    - SRD 5.2.1 pp. 351; builtin/srd-5.2.1/creature/giant_frog.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_insect:
    source_sha256: b30eb4fad0790b70ce41acf5af020703fc15f924fe54bdac602d9b8b97352d56
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_insect
        implementation: dnd2024:creature_giant_insect
        parameters:
          features:
            spider_climb:
              economy: passive
              climb_ability_check_required: false
              surfaces:
              - difficult
              - ceiling
            multiattack:
              economy: action
              op: repeat_choice
              count:
                floor_divide:
                - input: spell_level
                - 2
              choices:
              - feature: poison_jab
              - feature: web_bolt
                requires_form:
                - spider
            poison_jab:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus:
                input: summoner_spell_attack_modifier
              reach_feet: 10
              targets:
                count: 1
              hit:
              - op: damage
                damage_type: piercing
                amount:
                  add:
                  - dice:
                      count: 1
                      sides: 6
                    add: 3
                  - input: spell_level
              - op: damage
                damage_type: poison
                amount:
                  add:
                  - dice:
                      count: 1
                      sides: 4
                    add: 0
            web_bolt:
              economy: action
              requires_form:
              - spider
              op: attack
              attack_modes:
              - ranged
              attack_bonus:
                input: summoner_spell_attack_modifier
              range_feet: 60
              targets:
                count: 1
              hit:
              - op: damage
                damage_type: bludgeoning
                amount:
                  add:
                  - dice:
                      count: 1
                      sides: 10
                    add: 3
                  - input: spell_level
              - op: set_speed
                feet: 0
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
            venomous_spew:
              economy: bonus_action
              requires_form:
              - centipede
              op: save
              ability: constitution
              dc:
                input: summoner_spell_save_dc
              targets:
                kind: creature
                count: 1
                range_feet: 10
                visible_to: self
              failure:
              - op: condition
                condition: poisoned
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
              success: []
          defenses: {}
          senses:
            passive_perception: 12
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands:
              languages_known_by: summoner
            can_speak: false
          proficiency_bonus:
            input: summoner_proficiency_bonus
          scaling_stats:
            ac:
              add:
              - 11
              - input: spell_level
            hit_points:
              add:
              - 30
              - multiply:
                - 10
                - subtract:
                  - input: spell_level
                  - 4
          form_choice:
            chooser: summoner
            options:
            - wasp
            - spider
            - centipede
            fly_speed_feet:
              wasp: 40
              otherwise: 0
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_giant_insect
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
        summoner: participant
        spell_level: integer
        summoner_spell_attack_modifier: integer
        summoner_spell_save_dc: integer
        summoner_spellcasting_ability_modifier: integer
        summoner_proficiency_bonus: integer
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_giant_insect
    citations:
    - SRD 5.2.1 pp. 136; builtin/srd-5.2.1/creature/giant_insect.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_owl:
    source_sha256: 08267964d97ad41f90722ad4727cc100f0db11e81de3e7362ed15b7dfc429759
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_owl
        implementation: dnd2024:creature_giant_owl
        parameters:
          features:
            flyby:
              economy: passive
              opportunity_attack_immunity:
                trigger: fly_out_of_enemy_reach
            talons:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 2
                damage_type: slashing
              reach_feet: 5
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: wisdom
              spells:
              - spell:
                  key: dnd2024:spells/detect_evil_and_good
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/detect_magic
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/clairvoyance
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - verbal
              - somatic
              - material
          skill_totals:
            perception: 6
            stealth: 6
          defenses:
            resistances:
              damage_types:
              - necrotic
              - radiant
          senses:
            passive_perception: 16
            special:
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - celestial
            understands:
            - celestial
            - common
            - elvish
            - sylvan
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_giant_owl
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - fly_out_of_enemy_reach
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - movement
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_giant_owl
    citations:
    - SRD 5.2.1 pp. 352; builtin/srd-5.2.1/creature/giant_owl.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_seahorse:
    source_sha256: dd7fb2bc56e362913e821b2add6e810c63f4d5b149909a80319050fb6a045ea9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_seahorse
        implementation: dnd2024:creature_giant_seahorse
        parameters:
          features:
            water_breathing:
              economy: passive
              breathing_media:
              - water
            ram:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: choose_by_predicate
                test:
                  immediately_preceding_movement:
                    minimum_feet: 20
                    direction: straight_toward_target
                then:
                - op: damage
                  average: 11
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 2
                  damage_type: bludgeoning
                else:
                - op: damage
                  average: 9
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    add: 2
                  damage_type: bludgeoning
              reach_feet: 5
            bubble_dash:
              economy: bonus_action
              requires:
                underwater: true
              op: move
              distance_max:
                multiply:
                - stat: swim_speed
                - numerator: 1
                  denominator: 2
              provokes_opportunity_attacks: false
          defenses: {}
          senses:
            passive_perception: 11
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_giant_seahorse
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - movement
      - resources
      - targeting
    requires:
    - dnd2024:creature_giant_seahorse
    citations:
    - SRD 5.2.1 pp. 353; builtin/srd-5.2.1/creature/giant_seahorse.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_toad:
    source_sha256: f66ba266df60dad110cf5bab216dfe30732df482f414a3f8fb1d87c6cf492679
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_toad
        implementation: dnd2024:creature_giant_toad
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            standing_leap:
              economy: passive
              long_jump_max_feet: 20
              high_jump_max_feet: 10
              running_start_required: false
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 2
                damage_type: piercing
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 2
                    sides: 4
                  add: 0
                damage_type: poison
              - op: if
                test:
                  target_kind: creature
                  target_size_max: medium
                then:
                - op: condition
                  condition: grappled
                  escape_dc: 12
                  source: self
              reach_feet: 5
            swallow:
              economy: action
              op: swallow
              targets:
                count: 1
                grappled_by: self
                size_max: medium
              on_swallow:
              - op: remove_condition
                condition: grappled
              - op: condition
                condition: blinded
              - op: condition
                condition: restrained
              while_swallowed:
                total_cover_against: external_attacks_and_effects
                self_cannot_use:
                - bite
              self_death:
                remove_conditions:
                - restrained
                escape:
                  movement_cost_feet: 5
                  exit_condition: prone
              digest:
                event: self.turn.end
                occurrence: each
                effects:
                - op: damage
                  average: 10
                  amount:
                    dice:
                      count: 3
                      sides: 6
                    add: 0
                  damage_type: acid
              swallowed_capacity: 1
          defenses: {}
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_giant_toad
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.turn.end
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - resources
      - targeting
    requires:
    - dnd2024:creature_giant_toad
    citations:
    - SRD 5.2.1 pp. 354; builtin/srd-5.2.1/creature/giant_toad.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/giant_wasp:
    source_sha256: b2b9f9783aebe63e65c0ba9cf09c393268367d2003965ebb4dc544e53e8b8d2a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/giant_wasp
        implementation: dnd2024:creature_giant_wasp
        parameters:
          features:
            flyby:
              economy: passive
              opportunity_attack_immunity:
                trigger: fly_out_of_enemy_reach
            sting:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 2
                damage_type: piercing
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 2
                    sides: 4
                  add: 0
                damage_type: poison
              reach_feet: 5
          defenses: {}
          senses:
            passive_perception: 10
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_giant_wasp
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - fly_out_of_enemy_reach
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - movement
      - resources
      - targeting
    requires:
    - dnd2024:creature_giant_wasp
    citations:
    - SRD 5.2.1 pp. 354, 355; builtin/srd-5.2.1/creature/giant_wasp.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/gibbering_mouther:
    source_sha256: cba0c8b87d8e862cc717fdd8ce2e8411c7fb88f3373e9184dc858da6ade22699
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/gibbering_mouther
        implementation: dnd2024:creature_gibbering_mouther
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 2
              targets:
                count: 1
              hit:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: piercing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: medium
                then:
                - op: condition
                  condition: prone
              - op: if
                test:
                  attack_reduces_target_hp_to: 0
                then:
                - op: kill
                - op: absorb_body
                  equipment_remains: true
              reach_feet: 5
            blinding_spittle:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 10
              targets:
                kind: creature
                count: all
                area:
                  shape: sphere
                  size_feet: 10
                  origin: chosen_point
                  origin_range_feet: 30
              failure:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: radiant
              - op: condition
                condition: blinded
                ends:
                  event: turn.end
                  participant: self
                  occurrence: next
              success: []
            aberrant_ground:
              economy: passive
              terrain:
                area:
                  shape: emanation
                  radius_feet: 10
                  origin: self
                surface: ground
                difficult: true
            gibbering:
              economy: passive
              trigger: creature.turn.start
              requires:
                self_incapacitated: false
              op: save
              ability: wisdom
              dc: 10
              targets:
                kind: creature
                count: all
                range_feet: 20
              failure:
              - op: roll_behavior
                roll:
                  dice:
                    count: 1
                    sides: 8
                  add: 0
                duration: current_turn
                table:
                - roll_min: 1
                  roll_max: 4
                  behavior:
                    does_nothing: true
                - roll_min: 5
                  roll_max: 6
                  behavior:
                    action_allowed: false
                    bonus_action_allowed: false
                    movement: all
                    direction: random
                - roll_min: 7
                  roll_max: 8
                  behavior:
                    op: melee_attack
                    target: random_creature_within_reach
                    fallback: does_nothing
              success: []
          defenses:
            immunities:
              conditions:
              - prone
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_gibbering_mouther
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - creature.turn.start
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_gibbering_mouther
    citations:
    - SRD 5.2.1 pp. 288; builtin/srd-5.2.1/creature/gibbering_mouther.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/gnoll_warrior:
    source_sha256: 48a80dc69132a6c8a6a4b49ea3adc88c1b20b76e4b370ee93f4a1b4bb12349d3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/gnoll_warrior
        implementation: dnd2024:creature_gnoll_warrior
        parameters:
          features:
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 2
                damage_type: piercing
              reach_feet: 5
            bone_bow:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 3
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 1
                damage_type: piercing
              range_feet: 150
              long_range_feet: 600
            rampage:
              economy: bonus_action
              resource:
                capacity: 1
                recovery: long_rest
                cost: 1
              trigger: self.damage_dealt
              requires:
                target_kind: creature
                target_bloodied_before_damage: true
              op: sequence
              steps:
              - op: move
                distance_max:
                  multiply:
                  - stat: speed
                  - numerator: 1
                    denominator: 2
              - op: use_feature
                feature: rend
                count: 1
          defenses: {}
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - gnoll
            understands:
            - gnoll
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_gnoll_warrior
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.damage_dealt
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_gnoll_warrior
    citations:
    - SRD 5.2.1 pp. 289; builtin/srd-5.2.1/creature/gnoll_warrior.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/goblin_minion:
    source_sha256: fb742d01b95e73aab38a6a1944f5dd03972d42bec48a2af76c407dd73245b299
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/goblin_minion
        implementation: dnd2024:creature_goblin_minion
        parameters:
          features:
            dagger:
              economy: action
              op: attack
              attack_modes:
              - melee
              - ranged
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 2
                damage_type: piercing
              reach_feet: 5
              range_feet: 20
              long_range_feet: 60
            nimble_escape:
              economy: bonus_action
              op: take_action
              choice:
              - disengage
              - hide
          skill_totals:
            stealth: 6
          defenses: {}
          senses:
            passive_perception: 9
            special:
            - sense: darkvision
              range_feet: 60
          gear:
          - item_name: Daggers
            quantity: 3
          communication:
            speaks:
            - common
            - goblin
            understands:
            - common
            - goblin
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_goblin_minion
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_goblin_minion
    citations:
    - SRD 5.2.1 pp. 290; builtin/srd-5.2.1/creature/goblin_minion.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/gorgon:
    source_sha256: 7306094af5a5cc321c0e369bec3f947dedb1d128a052d8f97a960ba4176c809b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/gorgon
        implementation: dnd2024:creature_gorgon
        parameters:
          features:
            gore:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 8
              targets:
                count: 1
              hit:
              - op: damage
                average: 18
                amount:
                  dice:
                    count: 2
                    sides: 12
                  add: 5
                damage_type: piercing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                  immediately_preceding_movement:
                    minimum_feet: 20
                    direction: straight_toward_target
                then:
                - op: condition
                  condition: prone
              reach_feet: 5
            trample:
              economy: bonus_action
              op: save
              ability: dexterity
              dc: 16
              targets:
                kind: creature
                count: 1
                range_feet: 5
                condition: prone
              failure:
              - op: damage
                average: 16
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 5
                damage_type: bludgeoning
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            petrifying_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: constitution
              dc: 15
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 30
                  origin: self
              failure:
              - op: condition
                condition: restrained
                ends:
                  event: repeat_save.success
                repeat_save:
                  event: target.turn.end
                  occurrence: next
                  requires_condition: restrained
                  ability: constitution
                  dc: 15
                  success: end_effect
                  failure:
                  - op: remove_condition
                    condition: restrained
                  - op: condition
                    condition: petrified
              success: []
          skill_totals:
            perception: 7
          defenses:
            immunities:
              conditions:
              - exhaustion
              - petrified
          senses:
            passive_perception: 17
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_gorgon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - repeat_save.success
      - target.turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_gorgon
    citations:
    - SRD 5.2.1 pp. 292, 293; builtin/srd-5.2.1/creature/gorgon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/green_hag:
    source_sha256: 0824374c702ca9681fc24564296910602fffd0fa658f3fed083692ed328a8c9b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/green_hag
        implementation: dnd2024:creature_green_hag
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            claw:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 4
                damage_type: slashing
              - op: damage
                average: 3
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
                damage_type: poison
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: claw
                count: 2
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: wisdom
              spells:
              - spell:
                  key: dnd2024:spells/dancing_lights
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/disguise_self
                frequency: at_will
                overrides:
                  duration:
                    hours: 24
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/invisibility
                frequency: at_will
                overrides:
                  targets: self
                  while_invisible:
                    leaves_tracks: false
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/minor_illusion
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/ray_of_sickness
                frequency: at_will
                cast_level: 3
              components_waived:
              - material
              save_dc: 12
              spell_attack_bonus: 4
            mimicry:
              economy: passive
              imitations:
              - animal_sound
              - humanoid_voice
              discern:
                ability: wisdom
                skill: insight
                dc: 14
                requires_hearing: true
            coven_magic:
              economy: spell_normal_casting_time
              requires:
                hag_allies_within_feet: 30
                minimum_hag_allies: 2
              op: cast_spell_choice
              ability: intelligence
              save_dc: 11
              components_waived:
              - material
              choices:
              - spell:
                  key: dnd2024:spells/augury
                resource:
                  capacity: 1
                  cost: 1
                  recovery: long_rest
                  scope: per_spell
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/find_familiar
                resource:
                  capacity: 1
                  cost: 1
                  recovery: long_rest
                  scope: per_spell
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/identify
                resource:
                  capacity: 1
                  cost: 1
                  recovery: long_rest
                  scope: per_spell
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/locate_object
                resource:
                  capacity: 1
                  cost: 1
                  recovery: long_rest
                  scope: per_spell
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/scrying
                resource:
                  capacity: 1
                  cost: 1
                  recovery: long_rest
                  scope: per_spell
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/unseen_servant
                resource:
                  capacity: 1
                  cost: 1
                  recovery: long_rest
                  scope: per_spell
                cast_level: spell_minimum
          skill_totals:
            arcana: 5
            deception: 4
            perception: 4
            stealth: 3
          defenses: {}
          senses:
            passive_perception: 14
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - common
            - elvish
            - sylvan
            understands:
            - common
            - elvish
            - sylvan
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_green_hag
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_green_hag
    citations:
    - SRD 5.2.1 pp. 295; builtin/srd-5.2.1/creature/green_hag.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/grimlock:
    source_sha256: 1b7bf16fdbcbc0243d6a699a0cdb0dd4deeee32c61b1dbb8bc32e1739497b5ae
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/grimlock
        implementation: dnd2024:creature_grimlock
        parameters:
          features:
            bone_cudgel:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 3
                damage_type: bludgeoning
              - op: damage
                average: 2
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 0
                damage_type: psychic
              reach_feet: 5
          skill_totals:
            athletics: 5
            perception: 3
            stealth: 5
          defenses: {}
          senses:
            passive_perception: 13
            special:
            - sense: blindsight
              range_feet: 30
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_grimlock
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_grimlock
    citations:
    - SRD 5.2.1 pp. 296; builtin/srd-5.2.1/creature/grimlock.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/guardian_naga:
    source_sha256: 1ff0fcb66cca1e34327ad341991d2b970542854e2b25b6a18fa92c39bdd683ad
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/guardian_naga
        implementation: dnd2024:creature_guardian_naga
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 8
              targets:
                count: 1
              hit:
              - op: damage
                average: 17
                amount:
                  dice:
                    count: 2
                    sides: 12
                  add: 4
                damage_type: piercing
              - op: damage
                average: 22
                amount:
                  dice:
                    count: 4
                    sides: 10
                  add: 0
                damage_type: poison
              reach_feet: 10
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: bite
                count: 2
              replacement:
                max_attacks: all
                choices:
                - feature: poisonous_spittle
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: wisdom
              spells:
              - spell:
                  key: dnd2024:spells/thaumaturgy
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/clairvoyance
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/cure_wounds
                frequency: daily
                uses: 1
                resource_scope: per_spell
                cast_level: 6
                recovery: long_rest
              - spell:
                  key: dnd2024:spells/flame_strike
                frequency: daily
                uses: 1
                resource_scope: per_spell
                cast_level: 6
                recovery: long_rest
              - spell:
                  key: dnd2024:spells/geas
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/true_seeing
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - somatic
              - material
              save_dc: 16
            celestial_restoration:
              economy: passive
              resource:
                capacity: 1
                recovery:
                - long_rest
                cost: 1
              trigger: self.died
              op: schedule_revive
              delay:
                days:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
              hit_points: maximum
              canceled_by:
                spell:
                  key: dnd2024:spells/dispel_evil_and_good
                target: self_remains
            poisonous_spittle:
              economy: action
              op: save
              ability: constitution
              dc: 16
              targets:
                kind: creature
                count: 1
                range_feet: 60
                visible_to: self
              failure:
              - op: damage
                average: 31
                amount:
                  dice:
                    count: 7
                    sides: 8
                  add: 0
                damage_type: poison
              - op: condition
                condition: blinded
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
          skill_totals:
            arcana: 11
            history: 11
            religion: 11
          defenses:
            immunities:
              damage_types:
              - poison
              conditions:
              - charmed
              - paralyzed
              - poisoned
              - restrained
          senses:
            passive_perception: 14
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - celestial
            - common
            understands:
            - celestial
            - common
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_guardian_naga
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.died
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_guardian_naga
    citations:
    - SRD 5.2.1 pp. 296; builtin/srd-5.2.1/creature/guardian_naga.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/hawk:
    source_sha256: 4d24b78d90ef0b8091196fbe545826979be093373da83a16ab5744c133e21679
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/hawk
        implementation: dnd2024:creature_hawk
        parameters:
          features:
            talons:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 1
                amount:
                  constant: 1
                damage_type: slashing
              reach_feet: 5
          skill_totals:
            perception: 6
          defenses: {}
          senses:
            passive_perception: 16
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_hawk
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_hawk
    citations:
    - SRD 5.2.1 pp. 355; builtin/srd-5.2.1/creature/hawk.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/hill_giant:
    source_sha256: a96817d4fe3dee0418806c8ca9674a88c87a00cf7a1bc61965d540a801ea30b3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/hill_giant
        implementation: dnd2024:creature_hill_giant
        parameters:
          features:
            tree_club:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 8
              targets:
                count: 1
              hit:
              - op: damage
                average: 18
                amount:
                  dice:
                    count: 3
                    sides: 8
                  add: 5
                damage_type: bludgeoning
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                then:
                - op: condition
                  condition: prone
              reach_feet: 10
            trash_lob:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 8
              targets:
                count: 1
              hit:
              - op: damage
                average: 16
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 5
                damage_type: bludgeoning
              - op: condition
                condition: poisoned
                ends:
                  event: turn.end
                  participant: target
                  occurrence: next
              range_feet: 60
              long_range_feet: 240
            multiattack:
              economy: action
              op: repeat_choice
              count: 2
              choices:
              - feature: tree_club
              - feature: trash_lob
          skill_totals:
            perception: 2
          defenses: {}
          senses:
            passive_perception: 12
            special: []
          communication:
            speaks:
            - giant
            understands:
            - giant
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_hill_giant
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.end
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_hill_giant
    citations:
    - SRD 5.2.1 pp. 298; builtin/srd-5.2.1/creature/hill_giant.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/hobgoblin_captain:
    source_sha256: 4c84c02185886023bac8c95ed12ef83cc6730b86eca806b163c8ddb83a3cf876
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/hobgoblin_captain
        implementation: dnd2024:creature_hobgoblin_captain
        parameters:
          features:
            greatsword:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 2
                damage_type: slashing
              - op: damage
                average: 3
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
                damage_type: poison
              reach_feet: 5
            longbow:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 2
                damage_type: piercing
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 2
                    sides: 4
                  add: 0
                damage_type: poison
              range_feet: 150
              long_range_feet: 600
            multiattack:
              economy: action
              op: repeat_choice
              count: 2
              choices:
              - feature: greatsword
              - feature: longbow
            aura_of_authority:
              economy: passive
              requires:
                self_incapacitated: false
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  size_feet: 10
                  origin: self
                relation:
                - self
                - ally
              advantage:
                rolls:
                - attack
                - saving_throw
          defenses: {}
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 60
          gear:
          - item_name: Greatsword
            quantity: 1
          - item_name: Half Plate Armor
            quantity: 1
          - item_name: Longbow
            quantity: 1
          communication:
            speaks:
            - common
            - goblin
            understands:
            - common
            - goblin
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_hobgoblin_captain
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_hobgoblin_captain
    citations:
    - SRD 5.2.1 pp. 299; builtin/srd-5.2.1/creature/hobgoblin_captain.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/horned_devil:
    source_sha256: 00ad6ba8962050c3332b44054015dbd62079a2a395333e62b360630029d89331
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/horned_devil
        implementation: dnd2024:creature_horned_devil
        parameters:
          features:
            magic_resistance:
              economy: passive
              advantage:
                roll: saving_throw
                against:
                - spell
                - magical_effect
            searing_fork:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 10
              targets:
                count: 1
              hit:
              - op: damage
                average: 15
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 6
                damage_type: piercing
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 0
                damage_type: fire
              reach_feet: 10
            hurl_flame:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 8
              targets:
                count: 1
              hit:
              - op: damage
                average: 26
                amount:
                  dice:
                    count: 5
                    sides: 8
                  add: 4
                damage_type: fire
              - op: if
                test:
                  target_kind: object
                  flammable: true
                  worn: false
                  carried: false
                then:
                - op: ignite
              range_feet: 150
            multiattack:
              economy: action
              op: repeat_choice
              count: 3
              choices:
              - feature: searing_fork
              - feature: hurl_flame
              replacement:
                max_attacks: 1
                choices:
                - feature: infernal_tail
            diabolical_restoration:
              economy: passive
              resource:
                capacity: 1
                recovery:
                - long_rest
                cost: 1
              trigger: self.died
              requires:
                death_plane_not: nine_hells
              op: revive
              new_body: true
              old_body: disappears
              delay:
                seconds: 0
              hit_points: maximum
              destination:
                plane: nine_hells
                location_choice: gm
            infernal_tail:
              economy: action
              op: save
              ability: dexterity
              dc: 17
              targets:
                kind: creature
                count: 1
                range_feet: 10
                visible_to: self
              failure:
              - op: damage
                average: 10
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 6
                damage_type: necrotic
              - op: if
                test:
                  target_has_infernal_wound: false
                then:
                - op: infernal_wound
                  stacking: one_per_target
                  periodic:
                    event: target.turn.start
                    effects:
                    - op: lose_hit_points
                      average: 10
                      amount:
                        dice:
                          count: 3
                          sides: 6
                        add: 0
                  ends_any:
                  - minutes: 1
                  - event: spell.restores_hit_points
                    target: affected
                  - economy: action
                    actor: target_or_creature_within_5_feet
                    check:
                      ability: wisdom
                      skill: medicine
                      dc: 17
                    outcome: success
              success: []
          defenses:
            resistances:
              damage_types:
              - cold
            immunities:
              damage_types:
              - fire
              - poison
              conditions:
              - poisoned
          senses:
            passive_perception: 13
            special:
            - sense: darkvision
              range_feet: 150
              magical_darkness_unimpeded: true
          communication:
            speaks:
            - infernal
            understands:
            - infernal
            telepathy:
              range_feet: 120
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_horned_devil
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.died
      - spell.restores_hit_points
      - target.turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_horned_devil
    citations:
    - SRD 5.2.1 pp. 299, 300; builtin/srd-5.2.1/creature/horned_devil.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/hyena:
    source_sha256: c5a185921e856c774737a91dda6feb62fa787e2afea552820ce460ac45ab6fcf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/hyena
        implementation: dnd2024:creature_hyena
        parameters:
          features:
            pack_tactics:
              economy: passive
              advantage:
                roll: attack
                target_kind: creature
                requires:
                  ally_distance_to_target_max_feet: 5
                  ally_incapacitated: false
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 2
              targets:
                count: 1
              hit:
              - op: damage
                average: 3
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
                damage_type: piercing
              reach_feet: 5
          skill_totals:
            perception: 3
          defenses: {}
          senses:
            passive_perception: 13
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_hyena
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_hyena
    citations:
    - SRD 5.2.1 pp. 356; builtin/srd-5.2.1/creature/hyena.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/imp:
    source_sha256: d738fc9877d80688888c811b30791658b1f6d8e42e8d985eb89923d35bd5bef3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/imp
        implementation: dnd2024:creature_imp
        parameters:
          features:
            magic_resistance:
              economy: passive
              advantage:
                roll: saving_throw
                against:
                - spell
                - magical_effect
            sting:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 3
                damage_type: piercing
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: poison
              reach_feet: 5
            invisibility:
              economy: action
              op: cast_spell
              spell:
                key: dnd2024:spells/invisibility
              ability: charisma
              components_waived:
              - verbal
              - somatic
              - material
              targets: self
              cast_level: spell_minimum
            shape_shift:
              economy: action
              op: shape_shift
              forms:
              - form: rat
                speeds_feet:
                  walk: 20
              - form: raven
                speeds_feet:
                  walk: 20
                  fly: 60
              - form: spider
                speeds_feet:
                  walk: 20
                  climb: 20
              - form: 'true'
              retain_statistics_except:
              - speed
              equipment_transformed: false
          skill_totals:
            deception: 4
            insight: 3
            stealth: 5
          defenses:
            resistances:
              damage_types:
              - cold
            immunities:
              damage_types:
              - fire
              - poison
              conditions:
              - poisoned
          senses:
            passive_perception: 11
            special:
            - sense: darkvision
              range_feet: 120
              magical_darkness_unimpeded: true
          communication:
            speaks:
            - common
            - infernal
            understands:
            - common
            - infernal
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_imp
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - shapechanging
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_imp
    citations:
    - SRD 5.2.1 pp. 300, 301; builtin/srd-5.2.1/creature/imp.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/iron_golem:
    source_sha256: bd2352926f723397a1f598c1b4a4b30cd85aac3efde02e84ddcda9891d655f8d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/iron_golem
        implementation: dnd2024:creature_iron_golem
        parameters:
          features:
            immutable_form:
              economy: passive
              shape_shift_allowed: false
            magic_resistance:
              economy: passive
              advantage:
                roll: saving_throw
                against:
                - spell
                - magical_effect
            bladed_arm:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 12
              targets:
                count: 1
              hit:
              - op: damage
                average: 20
                amount:
                  dice:
                    count: 3
                    sides: 8
                  add: 7
                damage_type: slashing
              - op: damage
                average: 10
                amount:
                  dice:
                    count: 3
                    sides: 6
                  add: 0
                damage_type: fire
              reach_feet: 10
            fiery_bolt:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 10
              targets:
                count: 1
              hit:
              - op: damage
                average: 36
                amount:
                  dice:
                    count: 8
                    sides: 8
                  add: 0
                damage_type: fire
              range_feet: 120
            poison_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: constitution
              dc: 18
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 60
                  origin: self
              failure:
              - op: damage
                average: 55
                amount:
                  dice:
                    count: 10
                    sides: 10
                  add: 0
                damage_type: poison
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: repeat_choice
              count: 2
              choices:
              - feature: bladed_arm
              - feature: fiery_bolt
            fire_absorption:
              economy: passive
              trigger: damage.subjected
              damage_type: fire
              op: heal
              amount:
                event_value: incoming_damage_of_matching_type
          defenses:
            immunities:
              damage_types:
              - fire
              - poison
              - psychic
              conditions:
              - charmed
              - exhaustion
              - frightened
              - paralyzed
              - petrified
              - poisoned
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 120
          communication:
            speaks: []
            understands:
            - common
            additional_languages:
              count: 2
              chooser: gm
              distinct_from_known: true
              can_speak: false
              can_understand: true
            can_speak: false
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_iron_golem
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - damage.subjected
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - hit_points
      - resources
      - shapechanging
      - targeting
    requires:
    - dnd2024:creature_iron_golem
    citations:
    - SRD 5.2.1 pp. 302; builtin/srd-5.2.1/creature/iron_golem.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/knight:
    source_sha256: 5c28a2c8c47943247d1190c64bdbce7bedc8a8f4cd5fe6ac416c2723096294a9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/knight
        implementation: dnd2024:creature_knight
        parameters:
          features:
            greatsword:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 10
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 3
                damage_type: slashing
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 0
                damage_type: radiant
              reach_feet: 5
            heavy_crossbow:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 2
              targets:
                count: 1
              hit:
              - op: damage
                average: 11
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 0
                damage_type: piercing
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 0
                damage_type: radiant
              range_feet: 100
              long_range_feet: 400
            multiattack:
              economy: action
              op: repeat_choice
              count: 2
              choices:
              - feature: greatsword
              - feature: heavy_crossbow
            parry:
              economy: reaction
              trigger: melee_attack.hits_self
              requires:
                holding_weapon: true
              effects:
              - op: modify_ac
                add: 2
                scope: triggering_attack
                reevaluate_hit: true
          defenses:
            immunities:
              conditions:
              - frightened
          senses:
            passive_perception: 10
            special: []
          gear:
          - item_name: Greatsword
            quantity: 1
          - item_name: Heavy Crossbow
            quantity: 1
          - item_name: Plate Armor
            quantity: 1
          communication:
            speaks:
            - common
            understands:
            - common
            additional_languages:
              count: 1
              chooser: gm
              distinct_from_known: true
              can_speak: true
              can_understand: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_knight
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - melee_attack.hits_self
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_knight
    citations:
    - SRD 5.2.1 pp. 302; builtin/srd-5.2.1/creature/knight.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/lamia:
    source_sha256: 7a9f2ae953e12fe93435e83907f4d3c621720c4c82681a3fcff30eda4a9c268e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/lamia
        implementation: dnd2024:creature_lamia
        parameters:
          features:
            claw:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 3
                damage_type: slashing
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: psychic
              reach_feet: 5
            leap:
              economy: bonus_action
              op: jump
              distance_max_feet: 30
              movement_cost_feet: 10
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: claw
                count: 2
              replacement:
                max_attacks: 1
                choices:
                - feature: corrupting_touch
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: charisma
              spells:
              - spell:
                  key: dnd2024:spells/disguise_self
                frequency: at_will
                overrides:
                  appearance:
                    sizes:
                    - large
                    - medium
                    anatomy: biped
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/minor_illusion
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/geas
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/major_image
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/scrying
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - material
              save_dc: 13
            corrupting_touch:
              economy: action
              op: save
              ability: wisdom
              dc: 13
              targets:
                kind: creature
                count: 1
                range_feet: 5
                visible_to: self
              failure:
              - op: damage
                average: 13
                amount:
                  dice:
                    count: 3
                    sides: 8
                  add: 0
                damage_type: psychic
              - op: curse
                duration:
                  hours: 1
                effects:
                - op: condition
                  condition: charmed
                  source: self
                - op: condition
                  condition: poisoned
              success: []
          skill_totals:
            deception: 7
            insight: 4
            stealth: 5
          defenses: {}
          senses:
            passive_perception: 12
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - abyssal
            - common
            understands:
            - abyssal
            - common
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_lamia
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - curses
      - dice
      - effects
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_lamia
    citations:
    - SRD 5.2.1 pp. 303, 304; builtin/srd-5.2.1/creature/lamia.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/lion:
    source_sha256: 46dc4f98f124a42dc0bd68e9a71b2cdac11b785aabdb42d776346364402e5c88
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/lion
        implementation: dnd2024:creature_lion
        parameters:
          features:
            pack_tactics:
              economy: passive
              advantage:
                roll: attack
                target_kind: creature
                requires:
                  ally_distance_to_target_max_feet: 5
                  ally_incapacitated: false
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 3
                damage_type: slashing
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 2
              replacement:
                max_attacks: 1
                choices:
                - feature: roar
            running_leap:
              economy: passive
              long_jump_max_feet: 25
              running_start_min_feet: 10
            roar:
              economy: action
              op: save
              ability: wisdom
              dc: 11
              targets:
                kind: creature
                count: 1
                range_feet: 15
              failure:
              - op: condition
                condition: frightened
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
              success: []
          skill_totals:
            perception: 3
            stealth: 4
          defenses: {}
          senses:
            passive_perception: 13
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_lion
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_lion
    citations:
    - SRD 5.2.1 pp. 356, 357; builtin/srd-5.2.1/creature/lion.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/magma_mephit:
    source_sha256: e9db7fc66907783b6b2575aabc08c2956fd7a26710a74eaaa38812837368701d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/magma_mephit
        implementation: dnd2024:creature_magma_mephit
        parameters:
          features:
            claw:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 3
              targets:
                count: 1
              hit:
              - op: damage
                average: 3
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 1
                damage_type: slashing
              - op: damage
                average: 3
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
                damage_type: fire
              reach_feet: 5
            fire_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 11
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 15
                  origin: self
              failure:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: fire
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            death_burst:
              economy: passive
              trigger: self.died
              op: save
              ability: dexterity
              dc: 11
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  size_feet: 5
                  origin: self
              failure:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: fire
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
          skill_totals:
            stealth: 3
          defenses:
            vulnerabilities:
              damage_types:
              - cold
            immunities:
              damage_types:
              - fire
              - poison
              conditions:
              - exhaustion
              - poisoned
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - primordial
            understands:
            - primordial
            dialects:
            - ignan
            - terran
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_magma_mephit
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.died
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_magma_mephit
    citations:
    - SRD 5.2.1 pp. 307; builtin/srd-5.2.1/creature/magma_mephit.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/manticore:
    source_sha256: e15ff0415ad814add67ac16257219500c2ca238eb3123dfa1db9f2a2d9de1235
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/manticore
        implementation: dnd2024:creature_manticore
        parameters:
          features:
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 3
                damage_type: slashing
              reach_feet: 5
            tail_spike:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 3
                damage_type: piercing
              range_feet: 100
              long_range_feet: 200
            multiattack:
              economy: action
              op: repeat_choice
              count: 3
              choices:
              - feature: rend
              - feature: tail_spike
          defenses: {}
          senses:
            passive_perception: 11
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - common
            understands:
            - common
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_manticore
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_manticore
    citations:
    - SRD 5.2.1 pp. 306; builtin/srd-5.2.1/creature/manticore.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/medusa:
    source_sha256: 86c0e08b37d233d349e3a3353b50bf489ddc2acbfaf14e88c1b9d42c2de912cb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/medusa
        implementation: dnd2024:creature_medusa
        parameters:
          features:
            claw:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 10
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 3
                damage_type: slashing
              reach_feet: 5
            snake_hair:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 3
                damage_type: piercing
              - op: damage
                average: 14
                amount:
                  dice:
                    count: 4
                    sides: 6
                  add: 0
                damage_type: poison
              reach_feet: 5
            poison_ray:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 11
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 2
                damage_type: poison
              range_feet: 150
            multiattack:
              economy: action
              op: choose_sequence
              choices:
              - - op: use_feature
                  feature: claw
                  count: 2
                - op: use_feature
                  feature: snake_hair
                  count: 1
              - - op: use_feature
                  feature: poison_ray
                  count: 3
            petrifying_gaze:
              economy: bonus_action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: constitution
              dc: 13
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 30
                  origin: self
              failure:
              - op: condition
                condition: restrained
                ends:
                  event: repeat_save.success
                repeat_save:
                  event: target.turn.end
                  occurrence: next
                  requires_condition: restrained
                  ability: constitution
                  dc: 13
                  success: end_effect
                  failure:
                  - op: remove_condition
                    condition: restrained
                  - op: condition
                    condition: petrified
              success: []
              additional_target:
                participant: self
                when: self_sees_own_reflection_in_cone
          skill_totals:
            deception: 5
            perception: 4
            stealth: 6
          defenses: {}
          senses:
            passive_perception: 14
            special:
            - sense: darkvision
              range_feet: 150
          communication:
            speaks:
            - common
            understands:
            - common
            additional_languages:
              count: 1
              chooser: gm
              distinct_from_known: true
              can_speak: true
              can_understand: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_medusa
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - repeat_save.success
      - target.turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_medusa
    citations:
    - SRD 5.2.1 pp. 306, 307; builtin/srd-5.2.1/creature/medusa.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/mimic:
    source_sha256: bd0610458015d8bb5f8717bbf393d1c25880f5d8e5fe9e3c5d86c46da56e6c01
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/mimic
        implementation: dnd2024:creature_mimic
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: choose_by_predicate
                test:
                  target_grappled_by: self
                then:
                - op: damage
                  average: 12
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 3
                  damage_type: piercing
                else:
                - op: damage
                  average: 7
                  amount:
                    dice:
                      count: 1
                      sides: 8
                    add: 3
                  damage_type: piercing
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 0
                damage_type: acid
              reach_feet: 5
              advantage_when:
                target_grappled_by: self
            pseudopod:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 3
                damage_type: bludgeoning
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 0
                damage_type: acid
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                then:
                - op: condition
                  condition: grappled
                  escape_dc: 13
                  source: self
                  escape_check_disadvantage: true
              reach_feet: 5
            adhesive:
              economy: passive
              requires_form:
              - object
              trigger: self.touched
              op: adhere
              targets: touching_thing
              creature_effect:
                op: condition
                condition: grappled
                escape_dc: 13
                source: self
                size_max: huge
                escape_check_disadvantage: true
            shape_shift:
              economy: bonus_action
              op: shape_shift
              forms:
              - form: object
                size_choice:
                - medium
                - small
              - form: true_blob
              retain_statistics: true
              equipment_transformed: false
          skill_totals:
            stealth: 5
          defenses:
            immunities:
              damage_types:
              - acid
              conditions:
              - prone
          senses:
            passive_perception: 11
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_mimic
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.touched
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - resources
      - shapechanging
      - targeting
    requires:
    - dnd2024:creature_mimic
    citations:
    - SRD 5.2.1 pp. 308, 309; builtin/srd-5.2.1/creature/mimic.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/mule:
    source_sha256: 3aff7726ddaefe49580b744729045a49ef9990d0601c847031c83019f4d80db4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/mule
        implementation: dnd2024:creature_mule
        parameters:
          features:
            hooves:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 2
                damage_type: bludgeoning
              reach_feet: 5
            beast_of_burden:
              economy: passive
              carrying_capacity_size_increment: 1
          defenses: {}
          senses:
            passive_perception: 10
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_mule
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_mule
    citations:
    - SRD 5.2.1 pp. 357; builtin/srd-5.2.1/creature/mule.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/nalfeshnee:
    source_sha256: 5253fef54c6baab4d9cccb0fbf6deb0e18f403bd22175696e0cdce97ddc3b726
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/nalfeshnee
        implementation: dnd2024:creature_nalfeshnee
        parameters:
          features:
            magic_resistance:
              economy: passive
              advantage:
                roll: saving_throw
                against:
                - spell
                - magical_effect
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 10
              targets:
                count: 1
              hit:
              - op: damage
                average: 16
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 5
                damage_type: slashing
              - op: damage
                average: 11
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 0
                damage_type: force
              reach_feet: 10
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
            demonic_restoration:
              economy: passive
              resource:
                capacity: 1
                recovery:
                - long_rest
                cost: 1
              trigger: self.died
              requires:
                death_plane_not: abyss
              op: revive
              new_body: true
              old_body: disappears
              delay:
                seconds: 0
              hit_points: maximum
              destination:
                plane: abyss
                location_choice: gm
            teleport:
              economy: action
              op: teleport
              targets: self
              range_feet: 120
              destination:
                unoccupied: true
                visible_to: self
            horror_nimbus:
              economy: bonus_action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: wisdom
              dc: 15
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  size_feet: 15
                  origin: self
              failure:
              - op: damage
                average: 28
                amount:
                  dice:
                    count: 8
                    sides: 6
                  add: 0
                damage_type: psychic
              - op: condition
                condition: frightened
                ends:
                  any:
                  - minutes: 1
                  - event: target.damage_taken
                  - event: target.turn.end
                    self_out_of_target_line_of_sight: true
              success:
              - op: feature_immunity
                source: self
                feature: horror_nimbus
                duration:
                  hours: 24
            pursuit:
              economy: reaction
              trigger: another_creature.move.end
              requires:
                triggering_creature_visible: true
                triggering_creature_distance_max_feet: 120
              op: use_feature
              feature: teleport
              destination_constraints:
                distance_to_triggering_creature_max_feet: 10
          defenses:
            resistances:
              damage_types:
              - cold
              - fire
              - lightning
            immunities:
              damage_types:
              - poison
              conditions:
              - frightened
              - poisoned
          senses:
            passive_perception: 11
            special:
            - sense: truesight
              range_feet: 120
          communication:
            speaks:
            - abyssal
            understands:
            - abyssal
            telepathy:
              range_feet: 120
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_nalfeshnee
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - another_creature.move.end
      - self.died
      - target.damage_taken
      - target.turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - movement
      - resources
      - targeting
    requires:
    - dnd2024:creature_nalfeshnee
    citations:
    - SRD 5.2.1 pp. 310, 311; builtin/srd-5.2.1/creature/nalfeshnee.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/noble:
    source_sha256: 78810f6bdf62eaf24c401682e178ecd554ac8bffcbf345a7539169e2974fa4f8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/noble
        implementation: dnd2024:creature_noble
        parameters:
          features:
            rapier:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 3
              targets:
                count: 1
              hit:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 1
                damage_type: piercing
              reach_feet: 5
            parry:
              economy: reaction
              trigger: melee_attack.hits_self
              requires:
                holding_weapon: true
              effects:
              - op: modify_ac
                add: 2
                scope: triggering_attack
                reevaluate_hit: true
          skill_totals:
            deception: 5
            insight: 4
            persuasion: 5
          defenses: {}
          senses:
            passive_perception: 12
            special: []
          gear:
          - item_name: Breastplate
            quantity: 1
          - item_name: Rapier
            quantity: 1
          communication:
            speaks:
            - common
            understands:
            - common
            additional_languages:
              count: 2
              chooser: gm
              distinct_from_known: true
              can_speak: true
              can_understand: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_noble
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - melee_attack.hits_self
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_noble
    citations:
    - SRD 5.2.1 pp. 312; builtin/srd-5.2.1/creature/noble.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/ogre:
    source_sha256: 0fbc4d511eea24641f8cd4053cebda1ec96e2b5724fda3d566ef241aba86baa1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/ogre
        implementation: dnd2024:creature_ogre
        parameters:
          features:
            greatclub:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 13
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 4
                damage_type: bludgeoning
              reach_feet: 5
            javelin:
              economy: action
              op: attack
              attack_modes:
              - melee
              - ranged
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 11
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 4
                damage_type: piercing
              reach_feet: 5
              range_feet: 30
              long_range_feet: 120
          defenses: {}
          senses:
            passive_perception: 8
            special:
            - sense: darkvision
              range_feet: 60
          gear:
          - item_name: Greatclub
            quantity: 1
          - item_name: Javelins
            quantity: 3
          communication:
            speaks:
            - common
            - giant
            understands:
            - common
            - giant
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_ogre
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_ogre
    citations:
    - SRD 5.2.1 pp. 312; builtin/srd-5.2.1/creature/ogre.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/otherworldly_steed:
    source_sha256: a2e20478ed0f8773d534c5161270909cfd6fa761aa40fe6970681df1f41183c5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/otherworldly_steed
        implementation: dnd2024:creature_otherworldly_steed
        parameters:
          features:
            life_bond:
              economy: passive
              trigger: summoner.hit_points_regained
              requires:
                source_spell_level_min: 1
                summoner_distance_max_feet: 5
              op: heal
              amount:
                event_value: summoner_hp_regained
            otherworldly_slam:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus:
                input: summoner_spell_attack_modifier
              reach_feet: 5
              targets:
                count: 1
              hit:
              - op: damage
                damage_type:
                  by_self_type:
                    celestial: radiant
                    fey: psychic
                    fiend: necrotic
                amount:
                  add:
                  - dice:
                      count: 1
                      sides: 8
                    add: 0
                  - input: spell_level
            fell_glare:
              economy: bonus_action
              requires_type:
              - fiend
              resource:
                capacity: 1
                cost: 1
                recovery: long_rest
              op: save
              ability: wisdom
              dc:
                input: summoner_spell_save_dc
              targets:
                kind: creature
                count: 1
                range_feet: 60
                visible_to: self
              failure:
              - op: condition
                condition: frightened
                ends:
                  event: turn.end
                  participant: summoner
                  occurrence: next
              success: []
            fey_step:
              economy: bonus_action
              resource:
                capacity: 1
                cost: 1
                recovery: long_rest
              requires_type:
              - fey
              op: teleport
              targets:
              - self
              - rider
              range_feet: 60
              destination:
                unoccupied: true
                chooser: summoner
            healing_touch:
              economy: bonus_action
              resource:
                capacity: 1
                cost: 1
                recovery: long_rest
              requires_type:
              - celestial
              targets:
                kind: creature
                count: 1
                range_feet: 5
              op: heal
              amount:
                add:
                - dice:
                    count: 2
                    sides: 8
                  add: 0
                - input: spell_level
          defenses: {}
          senses:
            passive_perception: 11
            special: []
          communication:
            speaks: []
            understands: []
            telepathy:
              range_feet: 5280
              target: summoner
          proficiency_bonus:
            input: summoner_proficiency_bonus
          scaling_stats:
            ac:
              add:
              - 10
              - input: spell_level
            hit_points:
              add:
              - 5
              - multiply:
                - 10
                - input: spell_level
            hit_dice:
              count:
                input: spell_level
              sides: 10
            fly_speed_feet:
              if:
                spell_level_min: 4
              then: 60
              else: 0
          type_choice:
            chooser: summoner
            options:
            - celestial
            - fey
            - fiend
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_otherworldly_steed
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
        summoner: participant
        spell_level: integer
        summoner_spell_attack_modifier: integer
        summoner_spell_save_dc: integer
        summoner_spellcasting_ability_modifier: integer
        summoner_proficiency_bonus: integer
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - summoner.hit_points_regained
      - turn.end
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - hit_points
      - movement
      - resources
      - targeting
    requires:
    - dnd2024:creature_otherworldly_steed
    citations:
    - SRD 5.2.1 pp. 131; builtin/srd-5.2.1/creature/otherworldly_steed.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/owlbear:
    source_sha256: a7d6f16a35ddb4f7c33fb7b3e19d01b68473745cdeba1a6f483e004b81ceaa86
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/owlbear
        implementation: dnd2024:creature_owlbear
        parameters:
          features:
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 14
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 5
                damage_type: slashing
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 2
          skill_totals:
            perception: 5
          defenses: {}
          senses:
            passive_perception: 15
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_owlbear
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_owlbear
    citations:
    - SRD 5.2.1 pp. 313; builtin/srd-5.2.1/creature/owlbear.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/phase_spider:
    source_sha256: 1a7844564ce4f20139ab98b7aef77b7f8e0c12bc6ef538fb65ce0cb8624ce254
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/phase_spider
        implementation: dnd2024:creature_phase_spider
        parameters:
          features:
            spider_climb:
              economy: passive
              climb_ability_check_required: false
              surfaces:
              - difficult
              - ceiling
            web_walker:
              economy: passive
              ignore_movement_restrictions:
              - web
              knows_location:
                targets: creatures_touching_same_web
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 3
                damage_type: piercing
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 0
                damage_type: poison
              - op: if
                test:
                  attack_reduces_target_hp_to: 0
                then:
                - op: stabilize
                - op: condition
                  condition: poisoned
                  ends:
                    hours: 1
                  while_active:
                    conditions:
                    - paralyzed
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: bite
                count: 2
            ethereal_sight:
              economy: passive
              from_planes:
              - material
              - ethereal
              see_other_plane: true
              range_feet: 60
            ethereal_jaunt:
              economy: bonus_action
              op: teleport
              plane_pairs:
              - - material
                - ethereal
              - - ethereal
                - material
          skill_totals:
            stealth: 7
          defenses: {}
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_phase_spider
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - movement
      - resources
      - targeting
    requires:
    - dnd2024:creature_phase_spider
    citations:
    - SRD 5.2.1 pp. 313, 314; builtin/srd-5.2.1/creature/phase_spider.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/pirate_captain:
    source_sha256: 8ae83a748faa69f917e0dbb519c8536ba87ab2fa271f5b9cc2be8da492b0181c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/pirate_captain
        implementation: dnd2024:creature_pirate_captain
        parameters:
          features:
            rapier:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 13
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 4
                damage_type: piercing
              - op: advantage
                participant: self
                roll: attack
                count: 1
                ends:
                  event: turn.end
                  participant: self
                  occurrence: current
              reach_feet: 5
            pistol:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 15
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 4
                damage_type: piercing
              range_feet: 30
              long_range_feet: 90
            multiattack:
              economy: action
              op: repeat_choice
              count: 3
              choices:
              - feature: rapier
              - feature: pistol
            captain_s_charm:
              economy: bonus_action
              op: save
              ability: wisdom
              dc: 14
              targets:
                kind: creature
                count: 1
                range_feet: 30
                visible_to: self
              failure:
              - op: condition
                condition: charmed
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
                source: self
              success: []
            riposte:
              economy: reaction
              trigger: melee_attack.hits_self
              requires:
                holding_weapon: true
              effects:
              - op: modify_ac
                add: 3
                scope: triggering_attack
                reevaluate_hit: true
              - op: if
                test:
                  triggering_attack_misses_after_ac_bonus: true
                  attacker_within_rapier_range: true
                then:
                - op: use_feature
                  feature: rapier
                  count: 1
                  target: triggering_attacker
          skill_totals:
            acrobatics: 7
            perception: 5
          defenses: {}
          senses:
            passive_perception: 15
            special: []
          gear:
          - item_name: Pistol
            quantity: 1
          - item_name: Rapier
            quantity: 1
          communication:
            speaks:
            - common
            understands:
            - common
            additional_languages:
              count: 1
              chooser: gm
              distinct_from_known: true
              can_speak: true
              can_understand: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_pirate_captain
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - melee_attack.hits_self
      - turn.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_pirate_captain
    citations:
    - SRD 5.2.1 pp. 314; builtin/srd-5.2.1/creature/pirate_captain.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/plesiosaurus:
    source_sha256: d6645293220b2d6eca564048478d7394229d4e20396dd24fadc0cf080e414810
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/plesiosaurus
        implementation: dnd2024:creature_plesiosaurus
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 11
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 4
                damage_type: piercing
              reach_feet: 10
            hold_breath:
              economy: passive
              duration:
                hours: 1
          skill_totals:
            perception: 3
            stealth: 4
          defenses: {}
          senses:
            passive_perception: 13
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_plesiosaurus
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_plesiosaurus
    citations:
    - SRD 5.2.1 pp. 358, 359; builtin/srd-5.2.1/creature/plesiosaurus.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/priest:
    source_sha256: 3fb917e210dd4997b971b4bfe5faf3f27a6652702c0c498e4ea3939ed8a01c86
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/priest
        implementation: dnd2024:creature_priest
        parameters:
          features:
            mace:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 3
                damage_type: bludgeoning
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 2
                    sides: 4
                  add: 0
                damage_type: radiant
              reach_feet: 5
            radiant_flame:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 11
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 0
                damage_type: radiant
              range_feet: 60
            multiattack:
              economy: action
              op: repeat_choice
              count: 2
              choices:
              - feature: mace
              - feature: radiant_flame
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: wisdom
              spells:
              - spell:
                  key: dnd2024:spells/light
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/thaumaturgy
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/spirit_guardians
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived: []
              save_dc: 13
              required_material_components_available: true
            divine_aid:
              economy: bonus_action
              resource:
                capacity: 3
                cost: 1
                recovery: long_rest
                scope: shared_choices
              op: cast_spell_choice
              choices:
              - spell:
                  key: dnd2024:spells/bless
              - spell:
                  key: dnd2024:spells/dispel_magic
              - spell:
                  key: dnd2024:spells/healing_word
              - spell:
                  key: dnd2024:spells/lesser_restoration
              ability: wisdom
              save_dc: 13
              components_waived: []
          skill_totals:
            medicine: 7
            perception: 5
            religion: 5
          defenses: {}
          senses:
            passive_perception: 15
            special: []
          gear:
          - item_name: Chain Shirt
            quantity: 1
          - item_name: Holy Symbol
            quantity: 1
          - item_name: Mace
            quantity: 1
          communication:
            speaks:
            - common
            understands:
            - common
            additional_languages:
              count: 1
              chooser: gm
              distinct_from_known: true
              can_speak: true
              can_understand: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_priest
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - hit_points
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_priest
    citations:
    - SRD 5.2.1 pp. 316; builtin/srd-5.2.1/creature/priest.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/pteranodon:
    source_sha256: 27aba0cd6c565c4444605c25ba96799cf36f24507c1418c2e51219f1093b304f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/pteranodon
        implementation: dnd2024:creature_pteranodon
        parameters:
          features:
            flyby:
              economy: passive
              opportunity_attack_immunity:
                trigger: fly_out_of_enemy_reach
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 2
                damage_type: piercing
              reach_feet: 5
          skill_totals:
            perception: 1
          defenses: {}
          senses:
            passive_perception: 11
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_pteranodon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - fly_out_of_enemy_reach
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - movement
      - resources
      - targeting
    requires:
    - dnd2024:creature_pteranodon
    citations:
    - SRD 5.2.1 pp. 359; builtin/srd-5.2.1/creature/pteranodon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/rakshasa:
    source_sha256: d61c003627ba18126d5aaf573acff9cec6abed6874727ded2e5a52f3aed9abaf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/rakshasa
        implementation: dnd2024:creature_rakshasa
        parameters:
          features:
            cursed_touch:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 10
              targets:
                count: 1
              hit:
              - op: damage
                average: 12
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 5
                damage_type: slashing
              - op: damage
                average: 19
                amount:
                  dice:
                    count: 3
                    sides: 12
                  add: 0
                damage_type: necrotic
              - op: if
                test:
                  target_kind: creature
                then:
                - op: curse
                  effects:
                  - op: disable_rest_benefits
                    rests:
                    - short
                    - long
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: cursed_touch
                count: 3
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: charisma
              spells:
              - spell:
                  key: dnd2024:spells/detect_magic
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/detect_thoughts
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/disguise_self
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/mage_hand
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/minor_illusion
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/fly
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/invisibility
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/major_image
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/plane_shift
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - material
              save_dc: 18
            fiendish_restoration:
              economy: passive
              resource:
                capacity: 1
                recovery:
                - long_rest
                cost: 1
              trigger: self.died
              requires:
                death_plane_not: nine_hells
              op: revive
              new_body: true
              old_body: disappears
              delay:
                seconds: 0
              hit_points: maximum
              destination:
                plane: nine_hells
                location_choice: gm
            greater_magic_resistance:
              economy: passive
              auto_success:
                roll: saving_throw
                against:
                - spell
                - magical_effect
              auto_miss:
                incoming_roll: spell_attack
              permission_required_for:
                spell_detection:
                - remote_observation
                - thoughts
                - creature_type
                - alignment
            baleful_command:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: wisdom
              dc: 18
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  size_feet: 30
                  origin: self
                relation: enemy
              failure:
              - op: damage
                average: 28
                amount:
                  dice:
                    count: 8
                    sides: 6
                  add: 0
                damage_type: psychic
              - op: condition
                condition: frightened
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
              - op: condition
                condition: incapacitated
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
              success: []
          skill_totals:
            deception: 10
            insight: 8
            perception: 8
          defenses:
            vulnerabilities:
              conditional:
              - damage_type: piercing
                source_kind: weapon
                weapon_wielder_has_spell_effect:
                  key: dnd2024:spells/bless
            immunities:
              conditions:
              - charmed
              - frightened
          senses:
            passive_perception: 18
            special:
            - sense: truesight
              range_feet: 60
          communication:
            speaks:
            - common
            - infernal
            understands:
            - common
            - infernal
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_rakshasa
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.died
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - curses
      - dice
      - effects
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_rakshasa
    citations:
    - SRD 5.2.1 pp. 317, 318; builtin/srd-5.2.1/creature/rakshasa.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/red_dragon_wyrmling:
    source_sha256: f5fe57fed12b27657a2668e446696caed37ac94602084f78633bb7df20795454
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/red_dragon_wyrmling
        implementation: dnd2024:creature_red_dragon_wyrmling
        parameters:
          features:
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 4
                damage_type: slashing
              - op: damage
                average: 3
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
                damage_type: fire
              reach_feet: 5
            fire_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 13
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 15
                  origin: self
              failure:
              - op: damage
                average: 24
                amount:
                  dice:
                    count: 7
                    sides: 6
                  add: 0
                damage_type: fire
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 2
          skill_totals:
            perception: 4
            stealth: 2
          defenses:
            immunities:
              damage_types:
              - fire
          senses:
            passive_perception: 14
            special:
            - sense: blindsight
              range_feet: 10
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - draconic
            understands:
            - draconic
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_red_dragon_wyrmling
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_red_dragon_wyrmling
    citations:
    - SRD 5.2.1 pp. 318; builtin/srd-5.2.1/creature/red_dragon_wyrmling.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/rhinoceros:
    source_sha256: cd4b327f12b1d7813ffbfc55591772cdfe9933d1f69c5d0ca3f54f2899c078c3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/rhinoceros
        implementation: dnd2024:creature_rhinoceros
        parameters:
          features:
            gore:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 14
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 5
                damage_type: piercing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                  immediately_preceding_movement:
                    minimum_feet: 20
                    direction: straight_toward_target
                then:
                - op: damage
                  average: 9
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    add: 0
                  damage_type: piercing
                - op: condition
                  condition: prone
              reach_feet: 5
          defenses: {}
          senses:
            passive_perception: 11
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_rhinoceros
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_rhinoceros
    citations:
    - SRD 5.2.1 pp. 360; builtin/srd-5.2.1/creature/rhinoceros.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/roper:
    source_sha256: 622f10b5871f6b680bfa76c7e619074f3db866d6e8c29c995544c99d5d4f6d11
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/roper
        implementation: dnd2024:creature_roper
        parameters:
          features:
            spider_climb:
              economy: passive
              climb_ability_check_required: false
              surfaces:
              - difficult
              - ceiling
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 17
                amount:
                  dice:
                    count: 3
                    sides: 8
                  add: 4
                damage_type: piercing
              reach_feet: 5
            tentacle:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: condition
                condition: grappled
                escape_dc: 14
                source: self
                appendages:
                  kind: tentacle
                  available: 6
                  used: 1
              - op: condition
                condition: poisoned
                ends:
                  event: grapple.end
                  source: self
              reach_feet: 60
              appendage:
                ac: 20
                hp: 10
                damage_immunities:
                - poison
                - psychic
                on_destroyed:
                - free_grappled_creature
                damage_transferred_to_self: false
                regrow:
                  event: turn.start
                  participant: self
                  occurrence: next
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: tentacle
                count: 2
              - op: use_feature
                feature: reel
                count: 1
              - op: use_feature
                feature: bite
                count: 2
            reel:
              economy: action
              op: pull
              targets:
                count: all
                grappled_by: self
              distance_max_feet: 30
              direction: straight_toward_self
          skill_totals:
            perception: 6
            stealth: 5
          defenses: {}
          senses:
            passive_perception: 16
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_roper
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - grapple.end
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - resources
      - targeting
    requires:
    - dnd2024:creature_roper
    citations:
    - SRD 5.2.1 pp. 320; builtin/srd-5.2.1/creature/roper.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/sahuagin_warrior:
    source_sha256: b842b472f9565a7d1015439233cd25728fe97af61790b9021b92cb134bb77297
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/sahuagin_warrior
        implementation: dnd2024:creature_sahuagin_warrior
        parameters:
          features:
            claw:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 3
              targets:
                count: 1
              hit:
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 1
                damage_type: slashing
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: claw
                count: 2
            blood_frenzy:
              economy: passive
              advantage:
                roll: attack
                target_kind: creature
                when:
                  target_hp_less_than_maximum: true
            limited_amphibiousness:
              economy: passive
              breathing_media:
              - air
              - water
              submersion_required_every:
                hours: 4
              failure:
                outside_water: suffocating
            shark_telepathy:
              economy: passive
              magical: true
              op: control_creatures
              targets:
                kind: shark
                range_feet: 120
                count: all
              communication: telepathy
            aquatic_charge:
              economy: bonus_action
              op: move
              mode: swim
              distance_max:
                stat: swim_speed
              direction: straight_toward_visible_enemy
          skill_totals:
            perception: 5
          defenses:
            resistances:
              damage_types:
              - acid
              - cold
          senses:
            passive_perception: 15
            special:
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - sahuagin
            understands:
            - sahuagin
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_sahuagin_warrior
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_sahuagin_warrior
    citations:
    - SRD 5.2.1 pp. 321; builtin/srd-5.2.1/creature/sahuagin_warrior.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/scorpion:
    source_sha256: e6095b76047b1e345d264fd2d2adc3b60904112e5f16857810d9cbbf947c0c2a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/scorpion
        implementation: dnd2024:creature_scorpion
        parameters:
          features:
            sting:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 2
              targets:
                count: 1
              hit:
              - op: damage
                average: 1
                amount:
                  constant: 1
                damage_type: piercing
              - op: damage
                average: 3
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
                damage_type: poison
              reach_feet: 5
          defenses: {}
          senses:
            passive_perception: 9
            special:
            - sense: blindsight
              range_feet: 10
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_scorpion
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_scorpion
    citations:
    - SRD 5.2.1 pp. 360; builtin/srd-5.2.1/creature/scorpion.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/seahorse:
    source_sha256: e0a6b59a12d1b23bfbe5379c715bcbec8ea75733a9dcc96072b220a89074a27c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/seahorse
        implementation: dnd2024:creature_seahorse
        parameters:
          features:
            water_breathing:
              economy: passive
              breathing_media:
              - water
            bubble_dash:
              economy: action
              requires:
                underwater: true
              op: move
              distance_max:
                multiply:
                - stat: swim_speed
                - 1
              provokes_opportunity_attacks: false
          skill_totals:
            perception: 2
            stealth: 5
          defenses: {}
          senses:
            passive_perception: 12
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_seahorse
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - movement
      - resources
      - targeting
    requires:
    - dnd2024:creature_seahorse
    citations:
    - SRD 5.2.1 pp. 361; builtin/srd-5.2.1/creature/seahorse.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/shield_guardian:
    source_sha256: 608cfa8d600e05e340ff2feb560a8c93b72f19f5d2a42cf7b8e45fe8bec09824
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/shield_guardian
        implementation: dnd2024:creature_shield_guardian
        parameters:
          features:
            fist:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 11
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 4
                damage_type: bludgeoning
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: force
              reach_feet: 10
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: fist
                count: 2
            bound:
              economy: passive
              bond:
                item: amulet
                magical: true
              same_plane:
                wearer_can_telepathically_call_guardian: true
                guardian_knows_amulet:
                - distance
                - direction
              damage_transfer:
                requires_wearer_within_feet: 60
                from: amulet_wearer
                to: self
                fraction:
                  numerator: 1
                  denominator: 2
                  round: up
              telepathic_call_effect:
                op: travel_to
                destination: amulet_wearer
            regeneration:
              economy: passive
              trigger: self.turn.start
              requires:
                hit_points_min: 1
              op: heal
              amount:
                constant: 10
            spell_storing:
              economy: passive
              capacity: 1
              store:
                actor: amulet_wearer
                actor_kind: spellcaster
                cast_target: self
                caster_distance_max_feet: 5
                spell_level_max: 4
                spell_effect: suppressed
                previous_spell: lost
              release:
                op: cast_stored_spell
                parameters: set_by_original_caster
                ability: original_caster_spellcasting_ability
                components_waived:
                - verbal
                - somatic
                - material
                stored_spell_after_cast: lost
                economy: stored_spell_casting_time
            protection:
              economy: reaction
              trigger: attack_roll.hits_amulet_wearer
              requires:
                amulet_wearer_distance_max_feet: 5
              targets: amulet_wearer
              effects:
              - op: modify_ac
                add: 5
                includes_triggering_attack: true
                reevaluate_hit: true
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
          defenses:
            immunities:
              damage_types:
              - poison
              conditions:
              - charmed
              - exhaustion
              - frightened
              - paralyzed
              - petrified
              - poisoned
          senses:
            passive_perception: 10
            special:
            - sense: blindsight
              range_feet: 10
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: all
            commands_only: true
            can_speak: false
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_shield_guardian
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - attack_roll.hits_amulet_wearer
      - self.turn.start
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - hit_points
      - resources
      - targeting
    requires:
    - dnd2024:creature_shield_guardian
    citations:
    - SRD 5.2.1 pp. 323; builtin/srd-5.2.1/creature/shield_guardian.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/skeleton:
    source_sha256: 868f9d94d07d703ab6536e8da4319836cbc10fd1d9dfee83423bd636c81a0bb9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/skeleton
        implementation: dnd2024:creature_skeleton
        parameters:
          features:
            shortsword:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 3
                damage_type: piercing
              reach_feet: 5
            shortbow:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 3
                damage_type: piercing
              range_feet: 80
              long_range_feet: 320
          defenses:
            vulnerabilities:
              damage_types:
              - bludgeoning
            immunities:
              damage_types:
              - poison
              conditions:
              - exhaustion
              - poisoned
          senses:
            passive_perception: 9
            special:
            - sense: darkvision
              range_feet: 60
          gear:
          - item_name: Shortbow
            quantity: 1
          - item_name: Shortsword
            quantity: 1
          communication:
            speaks: []
            understands:
            - common
            additional_languages:
              count: 1
              chooser: gm
              distinct_from_known: true
              can_speak: false
              can_understand: true
            can_speak: false
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_skeleton
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_skeleton
    citations:
    - SRD 5.2.1 pp. 325, 326; builtin/srd-5.2.1/creature/skeleton.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/sphinx_of_lore:
    source_sha256: fd4ed085e1e7919456defd686e859048505280b909545495bb99b22045fb2411
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/sphinx_of_lore
        implementation: dnd2024:creature_sphinx_of_lore
        parameters:
          features:
            legendary_resistance:
              economy: passive
              resource:
                capacity: 3
                capacity_in_lair: 4
                recovery: long_rest
                cost: 1
              trigger: saving_throw.failed
              optional: true
              op: replace_save_outcome
              outcome: success
            claw:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 8
              targets:
                count: 1
              hit:
              - op: damage
                average: 14
                amount:
                  dice:
                    count: 3
                    sides: 6
                  add: 4
                damage_type: slashing
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: claw
                count: 3
            spellcasting:
              economy: action
              op: cast_spell_choice
              ability: intelligence
              spells:
              - spell:
                  key: dnd2024:spells/detect_magic
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/identify
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/mage_hand
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/minor_illusion
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/prestidigitation
                frequency: at_will
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/dispel_magic
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/legend_lore
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/locate_object
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/plane_shift
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/remove_curse
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              - spell:
                  key: dnd2024:spells/tongues
                frequency: daily
                uses: 1
                resource_scope: per_spell
                recovery: long_rest
                cast_level: spell_minimum
              components_waived:
              - material
              save_dc: 16
            inscrutable:
              economy: passive
              remote_magical_observation:
                requires_permission: true
              magical_thought_detection:
                requires_permission: true
              insight_disadvantage:
                purpose:
                - ascertain_intentions
                - ascertain_sincerity
            mind_rending_roar:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: wisdom
              dc: 16
              targets:
                kind: creature
                count: all
                area:
                  shape: emanation
                  size_feet: 300
                  origin: self
                relation: enemy
              failure:
              - op: damage
                average: 35
                amount:
                  dice:
                    count: 10
                    sides: 6
                  add: 0
                damage_type: psychic
              - op: condition
                condition: incapacitated
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
              success: []
            weight_of_years:
              economy: legendary_action
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
              op: save
              ability: constitution
              dc: 16
              targets:
                kind: creature
                count: 1
                range_feet: 120
                visible_to: self
              failure:
              - op: exhaustion
                levels: 1
              - op: apparent_age_increase
                years:
                  dice:
                    count: 3
                    sides: 10
                  add: 0
                ends:
                  target_exhaustion_levels: 0
              success: []
            arcane_prowl:
              economy: legendary_action
              op: sequence
              steps:
              - op: optional
                effect:
                  op: teleport
                  range_feet: 30
                  destination:
                    unoccupied: true
                    visible_to: self
              - op: use_feature
                feature: claw
                count: 1
          skill_totals:
            arcana: 12
            history: 12
            perception: 8
            religion: 12
          defenses:
            resistances:
              damage_types:
              - necrotic
              - radiant
            immunities:
              damage_types:
              - psychic
              conditions:
              - charmed
              - frightened
          senses:
            passive_perception: 18
            special:
            - sense: truesight
              range_feet: 120
          communication:
            speaks:
            - celestial
            - common
            understands:
            - celestial
            - common
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            cost_per_action: 1
            trigger: another_creature.turn.end
            recovery:
              event: turn.start
              participant: self
              amount: all
            maximum_per_trigger: 1
            requires:
              self_incapacitated: false
              self_can_take_actions: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_sphinx_of_lore
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - another_creature.turn.end
      - saving_throw.failed
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - curses
      - dice
      - effects
      - legendary_actions
      - movement
      - resources
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_sphinx_of_lore
    citations:
    - SRD 5.2.1 pp. 327, 328; builtin/srd-5.2.1/creature/sphinx_of_lore.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/spider:
    source_sha256: 0f59eac339b18f99e232026f1eac2a7724b5cf8494bdf8f658a478752cfd7150
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/spider
        implementation: dnd2024:creature_spider
        parameters:
          features:
            spider_climb:
              economy: passive
              climb_ability_check_required: false
              surfaces:
              - difficult
              - ceiling
            web_walker:
              economy: passive
              ignore_movement_restrictions:
              - web
              knows_location:
                targets: creatures_touching_same_web
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 1
                amount:
                  constant: 1
                damage_type: piercing
              - op: damage
                average: 2
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 0
                damage_type: poison
              reach_feet: 5
          skill_totals:
            stealth: 4
          defenses: {}
          senses:
            passive_perception: 10
            special:
            - sense: darkvision
              range_feet: 30
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_spider
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_spider
    citations:
    - SRD 5.2.1 pp. 361; builtin/srd-5.2.1/creature/spider.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/spy:
    source_sha256: b905d277fdf37ca5978a1564f807bbc7731b6a9ee9ea515771d4d3956ad27771
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/spy
        implementation: dnd2024:creature_spy
        parameters:
          features:
            shortsword:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 2
                damage_type: piercing
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: poison
              reach_feet: 5
            hand_crossbow:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 2
                damage_type: piercing
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: poison
              range_feet: 30
              long_range_feet: 120
            cunning_action:
              economy: bonus_action
              op: take_action
              choice:
              - dash
              - disengage
              - hide
          skill_totals:
            deception: 5
            insight: 4
            investigation: 5
            perception: 6
            sleight_of_hand: 4
            stealth: 6
          defenses: {}
          senses:
            passive_perception: 16
            special: []
          gear:
          - item_name: Hand Crossbow
            quantity: 1
          - item_name: Shortsword
            quantity: 1
          - item_name: Thieves’ Tools
            quantity: 1
          communication:
            speaks:
            - common
            understands:
            - common
            additional_languages:
              count: 1
              chooser: gm
              distinct_from_known: true
              can_speak: true
              can_understand: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_spy
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_spy
    citations:
    - SRD 5.2.1 pp. 329; builtin/srd-5.2.1/creature/spy.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/stone_giant:
    source_sha256: 2ee46878a975dff59d686d04d3f59a8b149e5976479b8d6d3bf01869afc12874
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/stone_giant
        implementation: dnd2024:creature_stone_giant
        parameters:
          features:
            stone_club:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 9
              targets:
                count: 1
              hit:
              - op: damage
                average: 22
                amount:
                  dice:
                    count: 3
                    sides: 10
                  add: 6
                damage_type: bludgeoning
              reach_feet: 15
            boulder:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 9
              targets:
                count: 1
              hit:
              - op: damage
                average: 15
                amount:
                  dice:
                    count: 2
                    sides: 8
                  add: 6
                damage_type: bludgeoning
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                then:
                - op: condition
                  condition: prone
              range_feet: 60
              long_range_feet: 240
            multiattack:
              economy: action
              op: repeat_choice
              count: 2
              choices:
              - feature: stone_club
              - feature: boulder
            deflect_missile:
              economy: reaction
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              trigger: ranged_attack.hits_self_and_deals_damage
              requires:
                damage_type_any:
                - bludgeoning
                - piercing
                - slashing
              effects:
              - op: reduce_damage
                average: 11
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 6
              - op: if
                test:
                  remaining_attack_damage: 0
                then:
                - op: optional
                  effect:
                    op: save
                    ability: dexterity
                    dc: 17
                    targets:
                      kind: creature
                      count: 1
                      range_feet: 60
                      visible_to: self
                    failure:
                    - op: damage
                      average: 11
                      amount:
                        dice:
                          count: 1
                          sides: 10
                        add: 6
                      damage_type: force
                    success: []
          skill_totals:
            athletics: 12
            perception: 4
            stealth: 5
          defenses: {}
          senses:
            passive_perception: 14
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - giant
            understands:
            - giant
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_stone_giant
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - ranged_attack.hits_self_and_deals_damage
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_stone_giant
    citations:
    - SRD 5.2.1 pp. 330; builtin/srd-5.2.1/creature/stone_giant.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/succubus:
    source_sha256: 653c9d99afaaec1fbe845cc5ea70691ff8a465bebd1a3cbb59c4ee708043bee8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/succubus
        implementation: dnd2024:creature_succubus
        parameters:
          features:
            fiendish_touch:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 16
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 5
                damage_type: psychic
              reach_feet: 5
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: fiendish_touch
                count: 1
              - op: choose
                choices:
                - op: use_feature
                  feature: charm
                  count: 1
                - op: use_feature
                  feature: draining_kiss
                  count: 1
            charm:
              economy: action
              op: cast_spell
              spell:
                key: dnd2024:spells/dominate_person
              ability: charisma
              save_dc: 15
              components_waived:
              - verbal
              - somatic
              - material
              cast_level: 8
            draining_kiss:
              economy: action
              op: save
              ability: constitution
              dc: 15
              targets:
                kind: creature
                count: 1
                range_feet: 5
                charmed_by: self
              failure:
              - op: damage
                average: 13
                amount:
                  dice:
                    count: 3
                    sides: 8
                  add: 0
                damage_type: psychic
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
              after_save:
              - op: reduce_hit_point_maximum
                amount:
                  event_value: damage_taken
            shape_shift:
              economy: bonus_action
              op: shape_shift
              forms:
              - form: humanoid
                size_choice:
                - medium
                - small
                fly_speed_available: false
              - form: 'true'
                fly_speed_available: true
              retain_statistics_except:
              - fly_speed
              equipment_transformed: false
            incubus_form:
              economy: passive
              trigger: self.long_rest.finished
              optional: true
              op: replace_stat_block
              definition:
                key: dnd2024:creatures/incubus
          skill_totals:
            deception: 9
            insight: 5
            perception: 5
            persuasion: 9
            stealth: 7
          defenses:
            resistances:
              damage_types:
              - cold
              - fire
              - poison
              - psychic
          senses:
            passive_perception: 15
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - abyssal
            - common
            - infernal
            understands:
            - abyssal
            - common
            - infernal
            telepathy:
              range_feet: 60
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_succubus
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.long_rest.finished
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - shapechanging
      - spellcasting
      - stat_block_replacement
      - targeting
    requires:
    - dnd2024:creature_succubus
    citations:
    - SRD 5.2.1 pp. 331; builtin/srd-5.2.1/creature/succubus.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/swarm_of_insects:
    source_sha256: 1b045da5bb774419108ac8e3218b987fb3437017465ddb4bd2d83b700473165a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/swarm_of_insects
        implementation: dnd2024:creature_swarm_of_insects
        parameters:
          features:
            spider_climb:
              economy: passive
              climb_ability_check_required: false
              surfaces:
              - difficult
              - ceiling
              requires:
                has_climb_speed: true
            swarm:
              economy: passive
              space_sharing: bidirectional
              minimum_passage_size: tiny
              regain_hit_points: false
              gain_temporary_hit_points: false
            bites:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 3
              targets:
                count: 1
              hit:
              - op: choose_by_predicate
                test:
                  self_bloodied: true
                then:
                - op: damage
                  average: 3
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 1
                  damage_type: poison
                else:
                - op: damage
                  average: 6
                  amount:
                    dice:
                      count: 2
                      sides: 4
                    add: 1
                  damage_type: poison
              reach_feet: 5
          defenses:
            resistances:
              damage_types:
              - bludgeoning
              - piercing
              - slashing
            immunities:
              conditions:
              - charmed
              - frightened
              - grappled
              - paralyzed
              - petrified
              - prone
              - restrained
              - stunned
          senses:
            passive_perception: 8
            special:
            - sense: blindsight
              range_feet: 30
          communication:
            speaks: []
            understands: []
          movement_variant:
            chooser: gm
            choose_one:
            - climb_speed_feet: 20
            - fly_speed_feet: 20
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_swarm_of_insects
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - resources
      - targeting
    requires:
    - dnd2024:creature_swarm_of_insects
    citations:
    - SRD 5.2.1 pp. 361; builtin/srd-5.2.1/creature/swarm_of_insects.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/swarm_of_ravens:
    source_sha256: 876efd6ae7700dd03cecde4b83df7e148d5e4967326341c410cbc4101ebbc736
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/swarm_of_ravens
        implementation: dnd2024:creature_swarm_of_ravens
        parameters:
          features:
            swarm:
              economy: passive
              space_sharing: bidirectional
              minimum_passage_size: tiny
              regain_hit_points: false
              gain_temporary_hit_points: false
            beaks:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: choose_by_predicate
                test:
                  self_bloodied: true
                then:
                - op: damage
                  average: 2
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    add: 0
                  damage_type: piercing
                else:
                - op: damage
                  average: 5
                  amount:
                    dice:
                      count: 1
                      sides: 6
                    add: 2
                  damage_type: piercing
              reach_feet: 5
            cacophony:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: wisdom
              dc: 10
              targets:
                kind: creature
                count: 1
                location: self_space
              failure:
              - op: condition
                condition: deafened
                ends:
                  event: turn.start
                  participant: self
                  occurrence: next
                while_active:
                  disadvantage:
                  - ability_check
                  - attack
              success: []
          skill_totals:
            perception: 5
          defenses:
            resistances:
              damage_types:
              - bludgeoning
              - piercing
              - slashing
            immunities:
              conditions:
              - charmed
              - frightened
              - grappled
              - paralyzed
              - petrified
              - prone
              - restrained
              - stunned
          senses:
            passive_perception: 15
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_swarm_of_ravens
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - resources
      - targeting
    requires:
    - dnd2024:creature_swarm_of_ravens
    citations:
    - SRD 5.2.1 pp. 362; builtin/srd-5.2.1/creature/swarm_of_ravens.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/tiger:
    source_sha256: 9392abc91d30b6a11be0fd2e35164c1e5f9b98a56b201c64989067ddde711bf3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/tiger
        implementation: dnd2024:creature_tiger
        parameters:
          features:
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 10
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 3
                damage_type: slashing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                then:
                - op: condition
                  condition: prone
              reach_feet: 5
            nimble_escape:
              economy: bonus_action
              op: take_action
              choice:
              - disengage
              - hide
          skill_totals:
            perception: 3
            stealth: 7
          defenses: {}
          senses:
            passive_perception: 13
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_tiger
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_tiger
    citations:
    - SRD 5.2.1 pp. 363; builtin/srd-5.2.1/creature/tiger.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/treant:
    source_sha256: f8e2c31d0fbc6ba9220ad3491806040153e5ef92c980a0309951fb5ece005fea
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/treant
        implementation: dnd2024:creature_treant
        parameters:
          features:
            siege_monster:
              economy: passive
              damage_multiplier: 2
              targets:
              - object
              - structure
            slam:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 10
              targets:
                count: 1
              hit:
              - op: damage
                average: 16
                amount:
                  dice:
                    count: 3
                    sides: 6
                  add: 6
                damage_type: bludgeoning
              reach_feet: 5
            hail_of_bark:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 10
              targets:
                count: 1
              hit:
              - op: damage
                average: 28
                amount:
                  dice:
                    count: 4
                    sides: 10
                  add: 6
                damage_type: piercing
              range_feet: 180
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: slam
                count: 2
            animate_trees:
              economy: action
              resource:
                capacity: 1
                recovery: long_rest
                cost: 1
              op: animate_objects
              targets:
                kind: tree
                count_max: 2
                range_feet: 60
                visible_to: self
              stat_block:
                key: dnd2024:creatures/treant
              overrides:
                ability_scores:
                  intelligence: 1
                  charisma: 1
                can_speak: false
                remove_features:
                - animate_trees
              initiative:
                same_count_as: self
                turn_order: immediately_after_self
              controller: self
              ends_any:
              - days: 1
              - animated_tree_dies: true
              - self_dies: true
              - distance_from_self_exceeds_feet: 120
              on_end:
                op: take_root
                if_possible: true
          defenses:
            vulnerabilities:
              damage_types:
              - fire
            resistances:
              damage_types:
              - bludgeoning
              - piercing
          senses:
            passive_perception: 13
            special: []
          communication:
            speaks:
            - common
            - druidic
            - elvish
            - sylvan
            understands:
            - common
            - druidic
            - elvish
            - sylvan
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_treant
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - summoning
      - targeting
    requires:
    - dnd2024:creature_treant
    citations:
    - SRD 5.2.1 pp. 333; builtin/srd-5.2.1/creature/treant.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/troll_limb:
    source_sha256: 17f33098ff2a9ef848fc826d3082ac45df727585dea12cb13881b4bc911fb1f9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/troll_limb
        implementation: dnd2024:creature_troll_limb
        parameters:
          features:
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 2
                    sides: 4
                  add: 4
                damage_type: slashing
              reach_feet: 5
            regeneration:
              economy: passive
              trigger: self.turn.start
              op: heal
              amount:
                constant: 5
              suppression:
                trigger: self.damage_taken
                damage_type_any:
                - acid
                - fire
                skip: next_turn
              death:
                trigger: self.turn.start
                requires:
                  hit_points: 0
                  regeneration_occurs: false
                exclusive: true
            troll_spawn:
              economy: passive
              senses_as:
                key: dnd2024:creatures/troll
              trigger: self.exists_for_24_hours
              requires:
                not_destroyed: true
              roll:
                dice:
                  count: 1
                  sides: 12
                add: 0
              table:
              - roll_min: 12
                roll_max: 12
                effect:
                  op: replace_stat_block
                  definition:
                    key: dnd2024:creatures/troll
              - roll_min: 1
                roll_max: 11
                effect:
                  op: wither_away
          defenses: {}
          senses:
            passive_perception: 9
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_troll_limb
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - self.damage_taken
      - self.exists_for_24_hours
      - self.turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - hit_points
      - resources
      - stat_block_replacement
      - targeting
    requires:
    - dnd2024:creature_troll_limb
    citations:
    - SRD 5.2.1 pp. 333; builtin/srd-5.2.1/creature/troll_limb.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/vampire:
    source_sha256: 6ecb7164f5e33949cbeaf2724952d6679d24dbf4378f334055be2e359f9c1648
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/vampire
        implementation: dnd2024:creature_vampire
        parameters:
          features:
            legendary_resistance:
              economy: passive
              resource:
                capacity: 3
                capacity_in_lair: 4
                recovery: long_rest
                cost: 1
              trigger: saving_throw.failed
              optional: true
              op: replace_save_outcome
              outcome: success
            spider_climb:
              economy: passive
              climb_ability_check_required: false
              surfaces:
              - difficult
              - ceiling
            grave_strike:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 9
              targets:
                count: 1
              hit:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 4
                damage_type: bludgeoning
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: necrotic
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                then:
                - op: condition
                  condition: grappled
                  escape_dc: 14
                  source: self
                  appendages:
                    kind: hand
                    available: 2
                    used: 1
              reach_feet: 5
              requires_form:
              - vampire
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: grave_strike
                count: 2
              - op: use_feature
                feature: bite
                count: 1
              requires_form:
              - vampire
            misty_escape:
              economy: none
              trigger: self.hit_points_drop_to_zero
              requires:
                outside_resting_place: true
              op: use_feature
              feature: shape_shift
              form: mist
              if_shape_shift_unavailable:
                op: destroy
              while_zero_hp_mist:
                return_to_vampire_form_allowed: false
                reach_resting_place_deadline:
                  hours: 2
                deadline_failure: destroy
              on_reach_resting_place:
                form: vampire
                conditions:
                - op: condition
                  condition: paralyzed
                  ends:
                    event: self.hit_points_regained
                after:
                  hours: 1
                  op: heal
                  amount:
                    constant: 1
            vampire_weakness:
              economy: passive
              op: feature_group
              members:
              - forbiddance
              - running_water
              - stake_to_the_heart
              - sunlight
            forbiddance:
              economy: passive
              enter_residence_requires:
                invitation_from: occupant
            running_water:
              economy: passive
              trigger: self.turn.end
              requires:
                in_running_water: true
              effects:
              - op: damage
                average: 20
                amount:
                  constant: 20
                damage_type: acid
            stake_to_the_heart:
              economy: passive
              trigger: piercing_weapon.driven_into_self_heart
              requires:
                self_incapacitated: true
                in_resting_place: true
              effects:
              - op: condition
                condition: paralyzed
                ends:
                  event: weapon.removed
            sunlight:
              economy: passive
              turn_start:
                requires:
                  in_sunlight: true
                effects:
                - op: damage
                  average: 20
                  amount:
                    constant: 20
                  damage_type: radiant
              while_in_sunlight:
                disadvantage:
                - attack
                - ability_check
            bite:
              economy: action
              requires_form:
              - bat
              - vampire
              op: save
              ability: constitution
              dc: 17
              targets:
                kind: creature
                count: 1
                range_feet: 5
                eligibility_any:
                - willing
                - grappled
                - incapacitated
                - restrained
              failure:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 4
                damage_type: piercing
              - op: damage
                average: 13
                amount:
                  dice:
                    count: 3
                    sides: 8
                  add: 0
                damage_type: necrotic
              - op: reduce_hit_point_maximum
                amount:
                  event_value: necrotic_damage_taken
              - op: heal
                participant: self
                amount:
                  event_value: necrotic_damage_taken
              - op: if
                test:
                  target_type: humanoid
                  attack_reduced_target_hp_to_zero: true
                then:
                - op: create_undead_on_burial
                  definition:
                    key: dnd2024:creatures/vampire_spawn
                  timing: following_sunset
                  controller: self
              success: []
            charm:
              economy: bonus_action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: cast_spell
              spell:
                key: dnd2024:spells/charm_person
              ability: charisma
              save_dc: 17
              components_waived:
              - verbal
              - somatic
              - material
              overrides:
                duration:
                  hours: 24
                while_charmed:
                  willing_for_self_bite: true
                  self_bite_damage_ends_spell: false
                on_end:
                  aware_of_charm: false
              cast_level: spell_minimum
            shape_shift:
              economy: bonus_action
              requires:
                in_sunlight: false
                in_running_water: false
              op: shape_shift
              worn_equipment_transformed: true
              forms:
              - form: bat
                size: tiny
                speeds_feet:
                  walk: 5
                  fly: 30
                can_speak: false
                retain_other_statistics: true
              - form: mist
                size: medium
                speeds_feet:
                  walk: 5
                  fly: 20
                hover: true
                actions_allowed: false
                can_speak: false
                manipulate_objects: false
                weight: 0
                enter_and_stop_in_enemy_space: true
                pass_through: spaces_air_can_pass
                pass_through_liquid: false
                damage_resistance:
                  all: true
                  except_sources:
                  - sunlight
              - form: vampire
              retain_statistics_except:
              - size
              - speed
              - listed_form_rules
            beguile:
              economy: legendary_action
              cooldown_ends:
                event: turn.start
                participant: self
                occurrence: next
              op: cast_spell
              spell:
                key: dnd2024:spells/command
              ability: charisma
              save_dc: 17
              components_waived:
              - verbal
              - somatic
              - material
              cast_level: spell_minimum
            deathless_strike:
              economy: legendary_action
              op: sequence
              steps:
              - op: move
                distance_max:
                  multiply:
                  - stat: speed
                  - numerator: 1
                    denominator: 2
              - op: use_feature
                feature: grave_strike
                count: 1
          skill_totals:
            perception: 7
            stealth: 9
          defenses:
            resistances:
              damage_types:
              - necrotic
          senses:
            passive_perception: 17
            special:
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            understands:
            - common
            additional_languages:
              count: 2
              chooser: gm
              distinct_from_known: true
              can_speak: true
              can_understand: true
          legendary_actions:
            uses: 3
            uses_in_lair: 4
            cost_per_action: 1
            trigger: another_creature.turn.end
            recovery:
              event: turn.start
              participant: self
              amount: all
            maximum_per_trigger: 1
            requires:
              self_incapacitated: false
              self_can_take_actions: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_vampire
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - another_creature.turn.end
      - piercing_weapon.driven_into_self_heart
      - saving_throw.failed
      - self.hit_points_drop_to_zero
      - self.hit_points_regained
      - self.turn.end
      - turn.start
      - weapon.removed
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - hit_points
      - legendary_actions
      - resources
      - shapechanging
      - spellcasting
      - targeting
    requires:
    - dnd2024:creature_vampire
    citations:
    - SRD 5.2.1 pp. 335, 336; builtin/srd-5.2.1/creature/vampire.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/venomous_snake:
    source_sha256: 9fc87b56cbc21fcebece1a8f24585d117ba382dc8fe3b8994de1d738beb29a63
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/venomous_snake
        implementation: dnd2024:creature_venomous_snake
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 2
                damage_type: piercing
              - op: damage
                average: 3
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
                damage_type: poison
              reach_feet: 5
          defenses: {}
          senses:
            passive_perception: 10
            special:
            - sense: blindsight
              range_feet: 10
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_venomous_snake
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_venomous_snake
    citations:
    - SRD 5.2.1 pp. 363; builtin/srd-5.2.1/creature/venomous_snake.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/vulture:
    source_sha256: 142e125ea27abe73c139a2c3d958b659eda75decc766ff9d2c787c79a5f21ff5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/vulture
        implementation: dnd2024:creature_vulture
        parameters:
          features:
            pack_tactics:
              economy: passive
              advantage:
                roll: attack
                target_kind: creature
                requires:
                  ally_distance_to_target_max_feet: 5
                  ally_incapacitated: false
            beak:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 2
              targets:
                count: 1
              hit:
              - op: damage
                average: 2
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 0
                damage_type: piercing
              reach_feet: 5
          skill_totals:
            perception: 3
          defenses: {}
          senses:
            passive_perception: 13
            special: []
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_vulture
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_vulture
    citations:
    - SRD 5.2.1 pp. 363, 364; builtin/srd-5.2.1/creature/vulture.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/warrior_infantry:
    source_sha256: 22d0d5f48109c0adc055db4c5198ac1d770b4c27549dbb29667ac78bbd182cc2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/warrior_infantry
        implementation: dnd2024:creature_warrior_infantry
        parameters:
          features:
            pack_tactics:
              economy: passive
              advantage:
                roll: attack
                target_kind: creature
                requires:
                  ally_distance_to_target_max_feet: 5
                  ally_incapacitated: false
            spear:
              economy: action
              op: attack
              attack_modes:
              - melee
              - ranged
              attack_bonus: 3
              targets:
                count: 1
              hit:
              - op: damage
                average: 4
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 1
                damage_type: piercing
              reach_feet: 5
              range_feet: 20
              long_range_feet: 60
          defenses: {}
          senses:
            passive_perception: 10
            special: []
          gear:
          - item_name: Chain Shirt
            quantity: 1
          - item_name: Spear
            quantity: 1
          communication:
            speaks:
            - common
            understands:
            - common
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_warrior_infantry
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_warrior_infantry
    citations:
    - SRD 5.2.1 pp. 336, 337; builtin/srd-5.2.1/creature/warrior_infantry.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/weasel:
    source_sha256: a3072e27131132c720045f8e387337cca67ce1c8c9cc0ff339960738807c6440
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/weasel
        implementation: dnd2024:creature_weasel
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 1
                amount:
                  constant: 1
                damage_type: piercing
              reach_feet: 5
          skill_totals:
            acrobatics: 5
            perception: 3
            stealth: 5
          defenses: {}
          senses:
            passive_perception: 13
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks: []
            understands: []
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_weasel
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_weasel
    citations:
    - SRD 5.2.1 pp. 364; builtin/srd-5.2.1/creature/weasel.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/wererat:
    source_sha256: cfcc7e0ae2dd6fb01c48c7d396d38bf40e35ccd4f30a6301e18675dcf1e81e79
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/wererat
        implementation: dnd2024:creature_wererat
        parameters:
          features:
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 8
                amount:
                  dice:
                    count: 2
                    sides: 4
                  add: 3
                damage_type: piercing
              - op: if
                test:
                  target_type: humanoid
                then:
                - op: save
                  ability: constitution
                  dc: 11
                  targets:
                    participant: target
                  failure:
                  - op: curse
                    trigger: target.hit_points_would_become_zero
                    replacement:
                      op: replace_stat_block
                      definition:
                        key: dnd2024:creatures/wererat
                      controller: gm
                      hit_points: 10
                  success:
                  - op: feature_immunity
                    source: self
                    feature: bite_curse
                    duration:
                      hours: 24
              reach_feet: 5
              requires_form:
              - rat
              - hybrid
            scratch:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 5
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 3
                damage_type: slashing
              reach_feet: 5
              range_feet: 30
              long_range_feet: 120
            multiattack:
              economy: action
              op: repeat_choice
              count: 2
              choices:
              - feature: scratch
              - feature: hand_crossbow
              replacement:
                max_attacks: 1
                choices:
                - feature: bite
            shape_shift:
              economy: bonus_action
              op: shape_shift
              forms:
              - form: hybrid
                size: medium
              - form: rat
                size: small
              - form: true_humanoid
              retain_statistics_except:
              - size
              equipment_transformed: false
            hand_crossbow:
              economy: action
              op: attack
              attack_modes:
              - ranged
              attack_bonus: 5
              range_feet: 30
              long_range_feet: 120
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 3
                damage_type: piercing
              requires_form:
              - humanoid
              - hybrid
          skill_totals:
            perception: 4
            stealth: 5
          defenses: {}
          senses:
            passive_perception: 14
            special:
            - sense: darkvision
              range_feet: 60
          gear:
          - item_name: Hand Crossbow
            quantity: 1
          communication:
            speaks:
            - common
            understands:
            - common
            cannot_speak_in_forms:
            - rat
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_wererat
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - target.hit_points_would_become_zero
      capabilities:
      - action_economy
      - creature_rules
      - curses
      - dice
      - effects
      - resources
      - shapechanging
      - stat_block_replacement
      - targeting
    requires:
    - dnd2024:creature_wererat
    citations:
    - SRD 5.2.1 pp. 338; builtin/srd-5.2.1/creature/wererat.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/white_dragon_wyrmling:
    source_sha256: 35d252968c56bfaa5d20ed58481a12a96fe0b97d695319acca83c7bdfc7f141b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/white_dragon_wyrmling
        implementation: dnd2024:creature_white_dragon_wyrmling
        parameters:
          features:
            ice_walk:
              economy: passive
              movement_checks_waived:
                surfaces:
                - ice
                modes:
                - walk
                - climb
              difficult_terrain_extra_cost_ignored:
              - ice
              - snow
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 4
              targets:
                count: 1
              hit:
              - op: damage
                average: 6
                amount:
                  dice:
                    count: 1
                    sides: 8
                  add: 2
                damage_type: slashing
              - op: damage
                average: 2
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 0
                damage_type: cold
              reach_feet: 5
            cold_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: constitution
              dc: 12
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 15
                  origin: self
              failure:
              - op: damage
                average: 22
                amount:
                  dice:
                    count: 5
                    sides: 8
                  add: 0
                damage_type: cold
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 2
          skill_totals:
            perception: 4
            stealth: 2
          defenses:
            immunities:
              damage_types:
              - cold
          senses:
            passive_perception: 14
            special:
            - sense: blindsight
              range_feet: 10
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - draconic
            understands:
            - draconic
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_white_dragon_wyrmling
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_white_dragon_wyrmling
    citations:
    - SRD 5.2.1 pp. 339, 340; builtin/srd-5.2.1/creature/white_dragon_wyrmling.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/winter_wolf:
    source_sha256: 502dc966ef92f5bb7d030dde49ead66d180348016171a9f5108b6ae137b1dd27
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/winter_wolf
        implementation: dnd2024:creature_winter_wolf
        parameters:
          features:
            pack_tactics:
              economy: passive
              advantage:
                roll: attack
                target_kind: creature
                requires:
                  ally_distance_to_target_max_feet: 5
                  ally_incapacitated: false
            bite:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 11
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 4
                damage_type: piercing
              - op: if
                test:
                  target_kind: creature
                  target_size_max: large
                then:
                - op: condition
                  condition: prone
              reach_feet: 5
            cold_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: constitution
              dc: 12
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 15
                  origin: self
              failure:
              - op: damage
                average: 18
                amount:
                  dice:
                    count: 4
                    sides: 8
                  add: 0
                damage_type: cold
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
          skill_totals:
            perception: 5
            stealth: 5
          defenses:
            immunities:
              damage_types:
              - cold
          senses:
            passive_perception: 15
            special: []
          communication:
            speaks:
            - common
            - giant
            understands:
            - common
            - giant
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_winter_wolf
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_winter_wolf
    citations:
    - SRD 5.2.1 pp. 342; builtin/srd-5.2.1/creature/winter_wolf.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/wraith:
    source_sha256: ad7252727091c06a9f6079f0f6e584ddd93d6c084c72595713126759aab34737
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/wraith
        implementation: dnd2024:creature_wraith
        parameters:
          features:
            sunlight_sensitivity:
              economy: passive
              disadvantage:
                rolls:
                - attack
                - ability_check
                when: self_in_sunlight
            life_drain:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 6
              targets:
                count: 1
              hit:
              - op: damage
                average: 21
                amount:
                  dice:
                    count: 4
                    sides: 8
                  add: 3
                damage_type: necrotic
              - op: if
                test:
                  target_kind: creature
                then:
                - op: reduce_hit_point_maximum
                  amount:
                    event_value: damage_taken
              reach_feet: 5
            incorporeal_movement:
              economy: passive
              move_through:
              - creatures
              - objects
              movement_cost: difficult_terrain
              turn_end_inside_object:
              - op: damage
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 0
                damage_type: force
            create_specter:
              economy: action
              op: create_creature
              targets:
                kind: corpse
                former_creature_type: humanoid
                range_feet: 10
                dead_for_max:
                  minutes: 1
              definition:
                key: dnd2024:creatures/specter
              destination_choice:
              - corpse_space
              - nearest_unoccupied_space
              controller: self
              controlled_capacity: 7
          defenses:
            resistances:
              damage_types:
              - acid
              - bludgeoning
              - cold
              - fire
              - piercing
              - slashing
            immunities:
              damage_types:
              - necrotic
              - poison
              conditions:
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
            passive_perception: 12
            special:
            - sense: darkvision
              range_feet: 60
          communication:
            speaks:
            - common
            understands:
            - common
            additional_languages:
              count: 2
              chooser: gm
              distinct_from_known: true
              can_speak: true
              can_understand: true
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_wraith
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events: []
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - grappling
      - resources
      - summoning
      - targeting
    requires:
    - dnd2024:creature_wraith
    citations:
    - SRD 5.2.1 pp. 342, 343; builtin/srd-5.2.1/creature/wraith.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/young_black_dragon:
    source_sha256: eb66b73440c58eb7ea1ab50102476166e55c0a0d2b678aed1cb56d2ccb7a8d18
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/young_black_dragon
        implementation: dnd2024:creature_young_black_dragon
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 2
                    sides: 4
                  add: 4
                damage_type: slashing
              - op: damage
                average: 3
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 0
                damage_type: acid
              reach_feet: 10
            acid_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 14
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  size_feet: 30
                  origin: self
                  width_feet: 5
              failure:
              - op: damage
                average: 49
                amount:
                  dice:
                    count: 14
                    sides: 6
                  add: 0
                damage_type: acid
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
          skill_totals:
            perception: 6
            stealth: 5
          defenses:
            immunities:
              damage_types:
              - acid
          senses:
            passive_perception: 16
            special:
            - sense: blindsight
              range_feet: 30
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_young_black_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_young_black_dragon
    citations:
    - SRD 5.2.1 pp. 264; builtin/srd-5.2.1/creature/young_black_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/young_bronze_dragon:
    source_sha256: 876975c61fcf9b2ce38426b0d7e875d4eaafb57839774b7e510797a1f241e45f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/young_bronze_dragon
        implementation: dnd2024:creature_young_bronze_dragon
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 8
              targets:
                count: 1
              hit:
              - op: damage
                average: 16
                amount:
                  dice:
                    count: 2
                    sides: 10
                  add: 5
                damage_type: slashing
              reach_feet: 10
            lightning_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: dexterity
              dc: 15
              targets:
                kind: creature
                count: all
                area:
                  shape: line
                  size_feet: 60
                  origin: self
                  width_feet: 5
              failure:
              - op: damage
                average: 49
                amount:
                  dice:
                    count: 9
                    sides: 10
                  add: 0
                damage_type: lightning
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
              replacement:
                max_attacks: 1
                choices:
                - feature: repulsion_breath
            repulsion_breath:
              economy: action
              op: save
              ability: strength
              dc: 15
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 30
                  origin: self
              failure:
              - op: push
                distance_max_feet: 40
                direction: straight_away_from_self
              - op: condition
                condition: prone
              success: []
          skill_totals:
            insight: 4
            perception: 7
            stealth: 3
          defenses:
            immunities:
              damage_types:
              - lightning
          senses:
            passive_perception: 17
            special:
            - sense: blindsight
              range_feet: 30
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_young_bronze_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_young_bronze_dragon
    citations:
    - SRD 5.2.1 pp. 270; builtin/srd-5.2.1/creature/young_bronze_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/young_green_dragon:
    source_sha256: e8c115b5c8fae3d3d80e547b2400100d48ca85575efcebfe7c07a0288ce9a3c4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/young_green_dragon
        implementation: dnd2024:creature_young_green_dragon
        parameters:
          features:
            amphibious:
              economy: passive
              breathing_media:
              - air
              - water
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 11
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 4
                damage_type: slashing
              - op: damage
                average: 7
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 0
                damage_type: poison
              reach_feet: 10
            poison_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: constitution
              dc: 14
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 30
                  origin: self
              failure:
              - op: damage
                average: 42
                amount:
                  dice:
                    count: 12
                    sides: 6
                  add: 0
                damage_type: poison
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
          skill_totals:
            deception: 5
            perception: 7
            stealth: 4
          defenses:
            immunities:
              damage_types:
              - poison
              conditions:
              - poisoned
          senses:
            passive_perception: 17
            special:
            - sense: blindsight
              range_feet: 30
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_young_green_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_young_green_dragon
    citations:
    - SRD 5.2.1 pp. 293, 294; builtin/srd-5.2.1/creature/young_green_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
  dnd2024:creatures/young_white_dragon:
    source_sha256: b33af9bdc67e8077789b9d6cffbab9642813970051fdd9de754fb1bda4228e29
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:creatures/young_white_dragon
        implementation: dnd2024:creature_young_white_dragon
        parameters:
          features:
            ice_walk:
              economy: passive
              movement_checks_waived:
                surfaces:
                - ice
                modes:
                - walk
                - climb
              difficult_terrain_extra_cost_ignored:
              - ice
              - snow
            rend:
              economy: action
              op: attack
              attack_modes:
              - melee
              attack_bonus: 7
              targets:
                count: 1
              hit:
              - op: damage
                average: 9
                amount:
                  dice:
                    count: 2
                    sides: 4
                  add: 4
                damage_type: slashing
              - op: damage
                average: 2
                amount:
                  dice:
                    count: 1
                    sides: 4
                  add: 0
                damage_type: cold
              reach_feet: 10
            cold_breath:
              economy: action
              resource:
                capacity: 1
                initial: 1
                recharge:
                  event: turn.start
                  participant: self
                  roll:
                    dice:
                      count: 1
                      sides: 6
                    add: 0
                  succeeds_on:
                  - 5
                  - 6
                cost: 1
                recovery_also:
                - short_rest
                - long_rest
              op: save
              ability: constitution
              dc: 15
              targets:
                kind: creature
                count: all
                area:
                  shape: cone
                  size_feet: 30
                  origin: self
              failure:
              - op: damage
                average: 40
                amount:
                  dice:
                    count: 9
                    sides: 8
                  add: 0
                damage_type: cold
              success:
              - op: damage_fraction
                numerator: 1
                denominator: 2
                round: down
            multiattack:
              economy: action
              op: sequence
              steps:
              - op: use_feature
                feature: rend
                count: 3
          skill_totals:
            perception: 6
            stealth: 3
          defenses:
            immunities:
              damage_types:
              - cold
          senses:
            passive_perception: 16
            special:
            - sense: blindsight
              range_feet: 30
            - sense: darkvision
              range_feet: 120
          communication:
            speaks:
            - common
            - draconic
            understands:
            - common
            - draconic
          stat_block_conventions:
            attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
            save_dc: supplied_total
            damage_mode_choice:
            - fixed_average
            - rolled_dice
            condition_source: self
            multiattack_child_economy_cost: 0
            multiattack_child_resource_costs_retained: true
            spellcasting:
              default_level: spell_minimum
              spell_slots_expended: false
              required_materials: available
              casting_time_at_least_one_minute:
                uses_normal_casting_duration: true
                magic_action_each_turn: true
                concentration_required: true
            geometry_assumptions:
              chooser: gm
              supplies:
              - range
              - visibility
              - line_of_sight
              - area_membership
              - reachable_locations
              - unoccupied_locations
              - terrain
              - plane
              - form_appearance
    contract:
      id: dnd2024:creature_young_white_dragon
      inputs:
        self: participant
        targets:
          type: participant_list
          required: false
        feature: choice
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
        resolution:
          type: resolution
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - resolved_feature
      - effects
      - resource_changes
      events:
      - turn.start
      capabilities:
      - action_economy
      - creature_rules
      - dice
      - effects
      - resources
      - targeting
    requires:
    - dnd2024:creature_young_white_dragon
    citations:
    - SRD 5.2.1 pp. 340; builtin/srd-5.2.1/creature/young_white_dragon.md
    - SRD 5.2.1 p. 257; builtin/srd-5.2.1/pages/257.md
---

# Creature conversion findings — group 2

All 112 assigned creature definitions are specified. Every named trait, action,
Bonus Action, Reaction, and Legendary Action is represented by typed parameters.
The definition citations above identify the original creature pages. The common
stat-block conventions use SRD page 257.

The source-specific contract vocabulary describes rule behavior; it does not
claim that the engine implements these operations. A feature identifier selects
one record in `features`. A `use_feature` step resolves that record with the same
source participant. Child uses within Multiattack spend their stated limited-use
resources but do not spend another action. Conditional branches run in sequence.
`damage_fraction` scales only the damage components of the matching failed-save
branch; riders do not carry over. A condition without a special end rule uses
its normal glossary duration. All source-relative conditions use the creature as
their source. `until` records identify a participant and its next stated turn
boundary. A duration after a condition describes that condition's lifetime.

Both fixed damage and dice are retained as alternatives. Supplied attack bonuses
and skill totals are complete totals before external modifiers. Creature header
ability scores, supplied saving throw totals, AC, HP, speeds, and classification
remain the parent converter's responsibility; variable summoned statistics and
source-specific movement choices are included here.

The Young White Dragon's unsigned Intelligence saving throw is corrected in the
transcription to −2, which equals its ability modifier. Ancient White Dragon's Freezing Burst DC 20 and Fear DC 18 are separate
source values. The Archmage's Mind Blank defenses depend on that precast spell.
The Vampire block has no Regeneration trait, and none is added.

Recharge on a die also restores after a Short or Long Rest. `X/Day` uses restore
after a Long Rest, as page 257 defines. Spell references retain all named level,
component, duration, target, and concentration exceptions. Normal long casting
times still apply to spells listed under Spellcasting.

No unresolvable source issue remains in this assignment. Geometry, free-form
appearances, language choices, and revival locations use bounded GM choices
where the source leaves those facts to the game state or GM.

Source coverage by definition (counts include explicit subfeatures of Vampire
Weakness, which are separately named in the source):

| Definition | Typed feature records | Source pages |
| --- | ---: | --- |
| adult_black_dragon | 9 | 264 |
| adult_bronze_dragon | 10 | 270, 271 |
| adult_green_dragon | 9 | 294 |
| adult_white_dragon | 8 | 340 |
| ancient_black_dragon | 9 | 265 |
| ancient_bronze_dragon | 10 | 271 |
| ancient_green_dragon | 9 | 294, 295 |
| ancient_white_dragon | 8 | 341 |
| animated_object | 1 | 109 |
| ankylosaurus | 2 | 344 |
| archmage | 6 | 305 |
| awakened_shrub | 1 | 260 |
| azer_sentinel | 3 | 261 |
| balor | 8 | 261 |
| barbed_devil | 7 | 262 |
| bearded_devil | 4 | 262, 263 |
| black_bear | 2 | 345 |
| blink_dog | 2 | 266 |
| boar | 2 | 346 |
| bronze_dragon_wyrmling | 5 | 269, 270 |
| bugbear_warrior | 3 | 272 |
| cat | 2 | 346 |
| chimera | 5 | 273 |
| cloaker | 6 | 274 |
| commoner | 2 | 275 |
| couatl | 5 | 277, 278 |
| cultist | 1 | 278 |
| death_dog | 2 | 279 |
| dire_wolf | 2 | 347 |
| draconic_spirit | 4 | 166 |
| dretch | 2 | 281 |
| dryad | 7 | 282 |
| earth_elemental | 5 | 282, 283 |
| elk | 1 | 348 |
| ettin | 3 | 284 |
| flesh_golem | 7 | 285 |
| frost_giant | 4 | 285, 286 |
| ghast | 3 | 287 |
| giant_ape | 4 | 349 |
| giant_boar | 2 | 349 |
| giant_crab | 2 | 350 |
| giant_elk | 1 | 351 |
| giant_frog | 4 | 351 |
| giant_insect | 5 | 136 |
| giant_owl | 3 | 352 |
| giant_seahorse | 3 | 353 |
| giant_toad | 4 | 354 |
| giant_wasp | 2 | 354, 355 |
| gibbering_mouther | 4 | 288 |
| gnoll_warrior | 3 | 289 |
| goblin_minion | 2 | 290 |
| gorgon | 3 | 292, 293 |
| green_hag | 6 | 295 |
| grimlock | 1 | 296 |
| guardian_naga | 5 | 296 |
| hawk | 1 | 355 |
| hill_giant | 3 | 298 |
| hobgoblin_captain | 4 | 299 |
| horned_devil | 6 | 299, 300 |
| hyena | 2 | 356 |
| imp | 4 | 300, 301 |
| iron_golem | 7 | 302 |
| knight | 4 | 302 |
| lamia | 5 | 303, 304 |
| lion | 5 | 356, 357 |
| magma_mephit | 3 | 307 |
| manticore | 3 | 306 |
| medusa | 5 | 306, 307 |
| mimic | 4 | 308, 309 |
| mule | 2 | 357 |
| nalfeshnee | 7 | 310, 311 |
| noble | 2 | 312 |
| ogre | 2 | 312 |
| otherworldly_steed | 5 | 131 |
| owlbear | 2 | 313 |
| phase_spider | 6 | 313, 314 |
| pirate_captain | 5 | 314 |
| plesiosaurus | 2 | 358, 359 |
| priest | 5 | 316 |
| pteranodon | 2 | 359 |
| rakshasa | 6 | 317, 318 |
| red_dragon_wyrmling | 3 | 318 |
| rhinoceros | 1 | 360 |
| roper | 5 | 320 |
| sahuagin_warrior | 6 | 321 |
| scorpion | 1 | 360 |
| seahorse | 2 | 361 |
| shield_guardian | 6 | 323 |
| skeleton | 2 | 325, 326 |
| sphinx_of_lore | 8 | 327, 328 |
| spider | 3 | 361 |
| spy | 3 | 329 |
| stone_giant | 4 | 330 |
| succubus | 6 | 331 |
| swarm_of_insects | 3 | 361 |
| swarm_of_ravens | 3 | 362 |
| tiger | 2 | 363 |
| treant | 5 | 333 |
| troll_limb | 3 | 333 |
| vampire | 15 | 335, 336 |
| venomous_snake | 1 | 363 |
| vulture | 2 | 363, 364 |
| warrior_infantry | 2 | 336, 337 |
| weasel | 1 | 364 |
| wererat | 5 | 338 |
| white_dragon_wyrmling | 4 | 339, 340 |
| winter_wolf | 3 | 342 |
| wraith | 4 | 342, 343 |
| young_black_dragon | 4 | 264 |
| young_bronze_dragon | 5 | 270 |
| young_green_dragon | 4 | 293, 294 |
| young_white_dragon | 4 | 340 |

Total: **112 definitions; 455 typed feature records**. No headings are classified as informational.

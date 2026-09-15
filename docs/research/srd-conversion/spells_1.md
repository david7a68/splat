---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:spells/acid_arrow:
    source_sha256: 55f5106415a010d6c50477acdbf7cdf6d76edb326bb4f226fcd831485a24be35
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/acid_arrow
        implementation: dnd2024:spell_acid_arrow
        parameters:
          target:
            kind: creature_or_object
            count: 1
            visible_to_caster: false
          attack:
            kind: ranged_spell
            hit:
              initial:
                amount:
                  dice:
                    count: 4
                    sides: 4
                    add: 0
                type: acid
              later:
                event: turn.end
                subject: target
                turn: next
                damage:
                  amount:
                    dice:
                      count: 2
                      sides: 4
                      add: 0
                  type: acid
            miss:
              damage:
                amount:
                  dice:
                    count: 4
                    sides: 4
                    add: 0
                type: acid
              scale:
                numerator: 1
                denominator: 2
              round: down
              later_damage: false
          scaling:
            per_slot_above: 2
            initial_dice: 1
            later_dice: 1
            sides: 4
    contract:
      id: dnd2024:spell_acid_arrow
      inputs:
        caster: participant
        cast_level: integer
        spell_attack_modifier: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - attack_result
      - damage
      - delayed_damage_effect
      events:
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
      - inventory
      - time
    requires:
    - dnd2024:spell_acid_arrow
    citations:
    - SRD 5.2.1 pp. 107; builtin/srd-5.2.1/spell/acid_arrow.md
  dnd2024:spells/alter_self:
    source_sha256: e1cfa74a0cc3e1134714029b787e31cf0d3fe4d2abf9f4e525187ab6b851e20e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/alter_self
        implementation: dnd2024:spell_alter_self
        parameters:
          target:
            kind: self
            count: 1
            visible_to_caster: false
          choice:
            chooser: caster
            count: 1
            options:
            - aquatic_adaptation
            - change_appearance
            - natural_weapons
            replace_action: magic
          options:
            aquatic_adaptation:
              underwater_breathing: true
              swim_speed:
                stat:
                  of: caster
                  key: speed
            change_appearance:
              choice_fields:
              - height
              - weight
              - face
              - voice
              - hair_length
              - color
              - distinguishing_features
              - apparent_species
              statistics_change: false
              same_size_category: true
              same_basic_shape: true
              change_action: magic
            natural_weapons:
              growth_choice:
                claws: slashing
                fangs: piercing
                horns: piercing
                hooves: bludgeoning
              attack: unarmed_strike
              damage_replaces_normal:
                dice:
                  count: 1
                  sides: 6
                  add:
                    binding: spellcasting_modifier
              attack_ability: spellcasting
              damage_ability: spellcasting
    contract:
      id: dnd2024:spell_alter_self
      inputs:
        caster: participant
        cast_level: integer
        spellcasting_modifier: integer
        spell_attack_modifier:
          type: integer
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - form_effect
      - granted_actions
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
      - inventory
      - choices
    requires:
    - dnd2024:spell_alter_self
    citations:
    - SRD 5.2.1 pp. 107; builtin/srd-5.2.1/spell/alter_self.md
  dnd2024:spells/animate_dead:
    source_sha256: ecdcd4ab8fc1785220e7c7441f513210767e992abd59ed25cfadc0b973701bb0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/animate_dead
        implementation: dnd2024:spell_animate_dead
        parameters:
          target:
            kind: bones_or_humanoid_corpse
            count: 1
            visible_to_caster: false
            size:
            - small
            - medium
            unique_remains_per_creature: true
          animation:
            bones:
              key: dnd2024:creatures/skeleton
            corpse:
              key: dnd2024:creatures/zombie
          control:
            hours: 24
            command:
              action: bonus_action
              event: turn.current
              range_feet: 60
              telepathic: true
              targets: any_created_creatures_in_range
              same_command_all: true
              options:
              - next_turn_action_and_movement
              - general_task
              persists_until: task_completed
            uncommanded:
              action: dodge
              movement: avoid_harm_only
            expiry: stop_obeying
          reassert:
            target: own_animated_creatures
            before_control_expires: true
            base_count: 4
            control_hours: 24
            replaces_animation: true
          scaling:
            per_slot_above: 3
            additional_animated: 2
            additional_reasserted: 2
    contract:
      id: dnd2024:spell_animate_dead
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - spawned_creatures
      - control_effect
      - commands
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - movement
      - summoning
      - time
      - relationships
    requires:
    - dnd2024:spell_animate_dead
    citations:
    - SRD 5.2.1 pp. 108; builtin/srd-5.2.1/spell/animate_dead.md
  dnd2024:spells/antipathy_sympathy:
    source_sha256: 6cba992a274908b04efa346ca9faa79a0c2164aaa1e131a8d375f7b855e56a6e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/antipathy_sympathy
        implementation: dnd2024:spell_antipathy_sympathy
        parameters:
          target:
            kind: creature_or_object
            count: 1
            visible_to_caster: false
            maximum_size: huge
          creature_kind_choice:
            chooser: caster
            selection: creature_kind
          aura:
            radius_feet: 120
            event: area.enter
            save:
              ability: wisdom
              dc:
                binding: save_dc
              failure:
                apply_chosen_mode: true
              success:
                effect: none
          modes:
            antipathy:
              condition: frightened
              movement:
                event: turn.current
                direction: away_from_target
                maximize_distance: true
                route: safest
            sympathy:
              condition: charmed
              movement:
                event: turn.current
                direction: toward_target
                maximize_distance: true
                route: safest
              within_feet: 5
              prohibit_willing_departure: true
              additional_save:
                event: damage.applied
                source: enchanted_target
                ability: wisdom
                success: end_effect
          repeat_save:
            event: turn.end
            distance_gt_feet: 120
            ability: wisdom
            success: end_effect
          successful_save_immunity:
            minutes: 1
            scope: this_effect
          mode_choice:
            chooser: caster
            options:
            - antipathy
            - sympathy
            at: cast
    contract:
      id: dnd2024:spell_antipathy_sympathy
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - aura_effect
      - saves
      - conditions
      - movement_constraints
      - immunity
      events:
      - spell.cast_completed
      - turn.end
      - damage.applied
      - area.enter
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - conditions
      - spatial_areas
      - movement
      - inventory
      - choices
      - time
      - relationships
    requires:
    - dnd2024:spell_antipathy_sympathy
    citations:
    - SRD 5.2.1 pp. 109, 110; builtin/srd-5.2.1/spell/antipathy_sympathy.md
  dnd2024:spells/arcane_sword:
    source_sha256: b8c92330a48bcd1f4d09132bbc3e28b84cdbde98cacf3ace4ed3b1da1476aebf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/arcane_sword
        implementation: dnd2024:spell_arcane_sword
        parameters:
          creation:
            kind: spectral_sword
            hover: true
            location: within_spell_range
          attack:
            event: spell.cast_completed
            kind: melee_spell
            target:
              kind: creature_or_object
              count: 1
              visible_to_caster: false
              distance_from_sword_feet: 5
            damage:
              amount:
                dice:
                  count: 4
                  sides: 12
                  add:
                    binding: spellcasting_modifier
              type: force
          repeat:
            action: bonus_action
            timing: later_caster_turns
            move_feet: 30
            destination_visible: true
            attack: same_or_different_target
    contract:
      id: dnd2024:spell_arcane_sword
      inputs:
        caster: participant
        cast_level: integer
        spellcasting_modifier: integer
        spell_attack_modifier: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - sword_effect
      - attacks
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
      - movement
      - inventory
      - time
    requires:
    - dnd2024:spell_arcane_sword
    citations:
    - SRD 5.2.1 pp. 110; builtin/srd-5.2.1/spell/arcane_sword.md
  dnd2024:spells/aura_of_life:
    source_sha256: 1559da6db8e15139feafe9f4d283255e951605f88fc571cd69a89d2e4f362769
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/aura_of_life
        implementation: dnd2024:spell_aura_of_life
        parameters:
          area:
            shape: emanation
            radius_feet: 30
            origin: caster
          beneficiaries:
          - caster
          - allies
          while_inside:
            resistance:
            - necrotic
            prevent_hp_maximum_reduction: true
          healing:
            event: turn.start
            subject: ally
            inside_aura: true
            hp_equals: 0
            amount: 1
    contract:
      id: dnd2024:spell_aura_of_life
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - aura_effect
      - resistances
      - hp_maximum_protection
      - healing
      events:
      - spell.cast_completed
      - turn.start
      capabilities:
      - spellcasting
      - effects
      - healing
      - spatial_areas
      - time
      - relationships
    requires:
    - dnd2024:spell_aura_of_life
    citations:
    - SRD 5.2.1 pp. 111; builtin/srd-5.2.1/spell/aura_of_life.md
  dnd2024:spells/barkskin:
    source_sha256: ad1506d11647507a532bab927b7e743c22340a286a37b8d551885e89046cb81c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/barkskin
        implementation: dnd2024:spell_barkskin
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
            willing: true
            touch: true
          armor_class:
            minimum: 17
          appearance: bark_skin
    contract:
      id: dnd2024:spell_barkskin
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - armor_class_effect
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_barkskin
    citations:
    - SRD 5.2.1 pp. 112; builtin/srd-5.2.1/spell/barkskin.md
  dnd2024:spells/black_tentacles:
    source_sha256: b7b1ddb4c5712a2cc84197f2627b7959b75206d5e08d62feb4b888a89c8fbe1f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/black_tentacles
        implementation: dnd2024:spell_black_tentacles
        parameters:
          area:
            shape: square
            side_feet: 20
            ground: true
            visible: true
            difficult_terrain: true
          triggers:
          - spell.cast_completed
          - area.enter
          - turn.end
          limit:
            saves_per_creature_per_turn: 1
          save:
            ability: strength
            dc:
              binding: save_dc
            failure:
              damage:
                amount:
                  dice:
                    count: 3
                    sides: 6
                    add: 0
                type: bludgeoning
              condition: restrained
              condition_duration: spell
            success:
              effect: none
          escape:
            action: action
            ability: strength
            skill: athletics
            dc:
              binding: save_dc
            success: end_restrained_on_self
    contract:
      id: dnd2024:spell_black_tentacles
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - area_effect
      - saves
      - damage
      - restrained
      - escape_check
      events:
      - spell.cast_completed
      - turn.end
      - area.enter
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - conditions
      - spatial_areas
      - time
      - checks
    requires:
    - dnd2024:spell_black_tentacles
    citations:
    - SRD 5.2.1 pp. 113; builtin/srd-5.2.1/spell/black_tentacles.md
  dnd2024:spells/blindness_deafness:
    source_sha256: eba3114003dad33eb690c9cbf6df63d8efc6c060d35d0683f856eb5cfb1c7e61
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/blindness_deafness
        implementation: dnd2024:spell_blindness_deafness
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: true
          condition_choice:
          - blinded
          - deafened
          save:
            ability: constitution
            dc:
              binding: save_dc
            failure:
              condition: chosen
              duration: spell
            success:
              effect: none
          repeat_save:
            event: turn.end
            ability: constitution
            success: end_spell_on_self
          scaling:
            per_slot_above: 2
            additional_targets: 1
    contract:
      id: dnd2024:spell_blindness_deafness
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - saves
      - condition_effect
      events:
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - conditions
      - choices
      - time
    requires:
    - dnd2024:spell_blindness_deafness
    citations:
    - SRD 5.2.1 pp. 113; builtin/srd-5.2.1/spell/blindness_deafness.md
  dnd2024:spells/call_lightning:
    source_sha256: b29b5e740e30daa7fe051974226d84d11deaefd0aa56bb8c68db0fce4862e11d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/call_lightning
        implementation: dnd2024:spell_call_lightning
        parameters:
          cloud:
            shape: cylinder
            height_feet: 10
            radius_feet: 60
            origin_visible: true
            above_caster: true
          bolt:
            target: visible_point_under_cloud
            creatures_within_feet: 5
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  amount:
                    dice:
                      count: 3
                      sides: 10
                      add: 0
                  type: lightning
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
          triggers:
          - spell.cast_completed
          repeat:
            action: magic
            same_or_new_point: true
          existing_storm:
            requires:
            - outdoors
            - storm_at_cast
            replace_cloud_creation_with_storm_control: true
            bonus_damage:
              dice:
                count: 1
                sides: 10
                add: 0
          scaling:
            per_slot_above: 3
            damage_dice: 1
            sides: 10
    contract:
      id: dnd2024:spell_call_lightning
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - cloud_effect
      - saves
      - damage
      - granted_action
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - spatial_areas
      - perception
      - relationships
    requires:
    - dnd2024:spell_call_lightning
    citations:
    - SRD 5.2.1 pp. 114; builtin/srd-5.2.1/spell/call_lightning.md
  dnd2024:spells/charm_person:
    source_sha256: 2f2fa564d6f32c6e721258183cc27a4ec6d4599cc854bb67d8ec5f010af1ec0a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/charm_person
        implementation: dnd2024:spell_charm_person
        parameters:
          target:
            kind: humanoid
            count: 1
            visible_to_caster: true
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              condition: charmed
              attitude_to_caster: friendly
            success:
              effect: none
          save_advantage_if: caster_or_allies_fighting_target
          end_on:
            event: damage.applied
            source:
            - caster
            - caster_allies
            subject: target
          on_end:
            target_knows_charmed_by_caster: true
          scaling:
            per_slot_above: 1
            additional_targets: 1
    contract:
      id: dnd2024:spell_charm_person
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - saves
      - charmed
      - attitude
      - knowledge
      events:
      - spell.cast_completed
      - damage.applied
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - conditions
      - knowledge
      - relationships
    requires:
    - dnd2024:spell_charm_person
    citations:
    - SRD 5.2.1 pp. 115; builtin/srd-5.2.1/spell/charm_person.md
  dnd2024:spells/clairvoyance:
    source_sha256: ffc9b1450d57eec003a7d09e45808ef2096fff9d8bf3f9a066c58af82617089b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/clairvoyance
        implementation: dnd2024:spell_clairvoyance
        parameters:
          sensor:
            location_choice:
            - visited_or_seen
            - unfamiliar_obvious_location
            invisible: true
            intangible: true
            invulnerable: true
            stationary: true
            visible_appearance: fist_sized_luminous_orb
          sense:
            choice:
            - seeing
            - hearing
            perception_origin: sensor
            switch_action: bonus_action
          focus:
            seeing: glass_eye
            hearing: jeweled_horn
            minimum_gp: 100
    contract:
      id: dnd2024:spell_clairvoyance
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - sensor
      - perception
      - granted_action
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - perception
      - choices
    requires:
    - dnd2024:spell_clairvoyance
    citations:
    - SRD 5.2.1 pp. 115; builtin/srd-5.2.1/spell/clairvoyance.md
  dnd2024:spells/command:
    source_sha256: 35fdb4cbb0e393cc1def0de68d4e1385220aaaeb5ad5e603085b68286b638ab6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/command
        implementation: dnd2024:spell_command
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: true
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              execute_command: next_target_turn
            success:
              effect: none
          choice:
            chooser: caster
            words: 1
            options:
            - approach
            - drop
            - flee
            - grovel
            - halt
          commands:
            approach:
              movement: toward_caster
              route: shortest_most_direct
              end_turn_within_feet: 5
            drop:
              drop: all_held_objects
              then: end_turn
            flee:
              whole_turn: move_away_from_caster
              means: fastest_available
            grovel:
              condition: prone
              then: end_turn
            halt:
              movement: false
              action: false
              bonus_action: false
          scaling:
            per_slot_above: 1
            additional_targets: 1
    contract:
      id: dnd2024:spell_command
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - saves
      - next_turn_command
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - conditions
      - movement
      - inventory
      - choices
      - time
      - relationships
    requires:
    - dnd2024:spell_command
    citations:
    - SRD 5.2.1 pp. 116; builtin/srd-5.2.1/spell/command.md
  dnd2024:spells/compulsion:
    source_sha256: 5c232aa23c88bac0fdb38dd4323512631a30cb39a9e4fc15ef137478f716941b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/compulsion
        implementation: dnd2024:spell_compulsion
        parameters:
          target:
            kind: creature
            count: chosen_all
            visible_to_caster: true
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              condition: charmed
            success:
              effect: none
          command:
            action: bonus_action
            direction: caster_chosen_horizontal
            targets: charmed_targets
            timing: next_target_turn
            movement: as_much_as_possible
            route: safest
            after_movement_save:
              ability: wisdom
              success: end_spell_on_self
    contract:
      id: dnd2024:spell_compulsion
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - saves
      - charmed
      - movement_command
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - conditions
      - movement
      - choices
      - time
      - relationships
    requires:
    - dnd2024:spell_compulsion
    citations:
    - SRD 5.2.1 pp. 117; builtin/srd-5.2.1/spell/compulsion.md
  dnd2024:spells/conjure_celestial:
    source_sha256: 05832e3846e252de3175fb165eb014eed4990a9bed1e073a75461308a544eb6d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/conjure_celestial
        implementation: dnd2024:spell_conjure_celestial
        parameters:
          area:
            shape: cylinder
            radius_feet: 10
            height_feet: 40
            light: bright
            origin: point_in_range
          target:
            kind: creature
            count: each_visible_in_area
            visible_to_caster: true
          choice_per_target:
            chooser: caster
            options:
            - healing_light
            - searing_light
            optional_on_later_triggers: true
          options:
            healing_light:
              heal:
                dice:
                  count: 4
                  sides: 12
                  add:
                    binding: spellcasting_modifier
            searing_light:
              save:
                ability: dexterity
                dc:
                  binding: save_dc
                failure:
                  damage:
                    amount:
                      dice:
                        count: 6
                        sides: 12
                        add: 0
                    type: radiant
                success:
                  damage_fraction:
                    numerator: 1
                    denominator: 2
          triggers:
          - spell.cast_completed
          - area.moves_into_creature
          - area.enter
          - turn.end
          movement:
            event: caster.moves_on_own_turn
            maximum_feet: 30
          limit:
            effects_per_creature_per_turn: 1
          scaling:
            per_slot_above: 7
            healing_dice: 1
            damage_dice: 1
            sides: 12
    contract:
      id: dnd2024:spell_conjure_celestial
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        spellcasting_modifier: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - area_effect
      - healing
      - saves
      - damage
      events:
      - spell.cast_completed
      - turn.end
      - area.enter
      - area.moved
      capabilities:
      - spellcasting
      - effects
      - damage
      - healing
      - saving_throws
      - spatial_areas
      - movement
      - perception
      - choices
      - time
    requires:
    - dnd2024:spell_conjure_celestial
    citations:
    - SRD 5.2.1 pp. 118; builtin/srd-5.2.1/spell/conjure_celestial.md
  dnd2024:spells/conjure_woodland_beings:
    source_sha256: e797d1e71ca8d6c7241af5a982c73ef859e8bbfcd62de85efd2ec909908b54a6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/conjure_woodland_beings
        implementation: dnd2024:spell_conjure_woodland_beings
        parameters:
          area:
            shape: emanation
            radius_feet: 10
            origin: caster
          triggers:
          - area.moves_into_creature
          - area.enter
          - turn.end
          target:
            kind: creature
            count: each_eligible
            visible_to_caster: true
          optional: true
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              damage:
                amount:
                  dice:
                    count: 5
                    sides: 8
                    add: 0
                type: force
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          limit:
            saves_per_creature_per_turn: 1
          grant:
            action: disengage
            cost: bonus_action
            duration: spell
          scaling:
            per_slot_above: 4
            damage_dice: 1
            sides: 8
    contract:
      id: dnd2024:spell_conjure_woodland_beings
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - aura_effect
      - saves
      - damage
      - bonus_disengage
      events:
      - spell.cast_completed
      - turn.end
      - area.enter
      - area.moved
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - spatial_areas
      - movement
      - time
    requires:
    - dnd2024:spell_conjure_woodland_beings
    citations:
    - SRD 5.2.1 pp. 118, 119; builtin/srd-5.2.1/spell/conjure_woodland_beings.md
  dnd2024:spells/continual_flame:
    source_sha256: 5aed7e8acec1a665e81803324d2809a5b6ec440e57ce0aa339f1c344e07f6cac
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/continual_flame
        implementation: dnd2024:spell_continual_flame
        parameters:
          target:
            kind: object
            count: 1
            visible_to_caster: false
            touch: true
          light:
            bright_radius_feet: 20
            additional_dim_feet: 20
          flame:
            heat: false
            fuel_consumption: false
            coverable: true
            hideable: true
            smotherable: false
            quenchable: false
    contract:
      id: dnd2024:spell_continual_flame
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - light_effect
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - perception
      - inventory
    requires:
    - dnd2024:spell_continual_flame
    citations:
    - SRD 5.2.1 pp. 119; builtin/srd-5.2.1/spell/continual_flame.md
  dnd2024:spells/create_food_and_water:
    source_sha256: 8a5480ad136a07cd5255804cedbb9ea745d058cd90b93c476a42b2d7ac2dbd5e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/create_food_and_water
        implementation: dnd2024:spell_create_food_and_water
        parameters:
          placement:
            choice:
            - ground
            - containers
            within_range: true
          food:
            pounds: 45
            nourishing: true
            taste: bland
            appearance_choice: food_kind
            uneaten_spoils_hours: 24
          water:
            gallons: 30
            fresh: true
            clean: true
    contract:
      id: dnd2024:spell_create_food_and_water
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - food_objects
      - water_quantity
      - food_expiry
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - choices
      - time
    requires:
    - dnd2024:spell_create_food_and_water
    citations:
    - SRD 5.2.1 pp. 120; builtin/srd-5.2.1/spell/create_food_and_water.md
  dnd2024:spells/cure_wounds:
    source_sha256: fa510b85cff9347ef844da3f8aa575308bd750368b6ce9a501185da8fd5e29b0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/cure_wounds
        implementation: dnd2024:spell_cure_wounds
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
            touch: true
          healing:
            dice:
              count: 2
              sides: 8
              add:
                binding: spellcasting_modifier
          scaling:
            per_slot_above: 1
            healing_dice: 2
            sides: 8
    contract:
      id: dnd2024:spell_cure_wounds
      inputs:
        caster: participant
        cast_level: integer
        spellcasting_modifier: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - healing
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - healing
    requires:
    - dnd2024:spell_cure_wounds
    citations:
    - SRD 5.2.1 pp. 121; builtin/srd-5.2.1/spell/cure_wounds.md
  dnd2024:spells/daylight:
    source_sha256: ffbba253b702b505b616f1b15955271d59966feacdeb9ae144652c36d42b2eb5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/daylight
        implementation: dnd2024:spell_daylight
        parameters:
          origin_choice:
            point:
              shape: sphere
            object:
              shape: emanation
              not_worn: true
              not_carried: true
              opaque_cover_blocks: true
          light:
            sunlight: true
            bright_radius_feet: 60
            additional_dim_feet: 60
          dispel:
            event: area.overlap
            target: darkness_spell
            maximum_spell_level: 3
    contract:
      id: dnd2024:spell_daylight
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - sunlight_effect
      - dispelled_effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - spatial_areas
      - perception
      - inventory
      - choices
    requires:
    - dnd2024:spell_daylight
    citations:
    - SRD 5.2.1 pp. 122; builtin/srd-5.2.1/spell/daylight.md
  dnd2024:spells/detect_evil_and_good:
    source_sha256: fff520b6a1c16dec20552ab26e1687bfe7026f1c233e3614e8dcb4cb1a85d496
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/detect_evil_and_good
        implementation: dnd2024:spell_detect_evil_and_good
        parameters:
          detection:
            radius_feet: 30
            origin: caster
            creature_types:
            - aberration
            - celestial
            - elemental
            - fey
            - fiend
            - undead
            outputs:
            - locations
            - hallow_presence
            - hallow_locations
          barriers:
          - materials:
            - stone
            - dirt
            - wood
            thickness_feet: 1
          - materials:
            - metal
            thickness_inches: 1
          - materials:
            - lead
            thickness: thin_sheet
    contract:
      id: dnd2024:spell_detect_evil_and_good
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - detected_locations
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_detect_evil_and_good
    citations:
    - SRD 5.2.1 pp. 123; builtin/srd-5.2.1/spell/detect_evil_and_good.md
  dnd2024:spells/dimension_door:
    source_sha256: f4f6f7440d22372cb485c7e965273bb5743975281d044d000f1af313ec2e5fff
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/dimension_door
        implementation: dnd2024:spell_dimension_door
        parameters:
          teleport:
            subject: caster
            destination_choice:
            - seen
            - visualized
            - distance_and_direction
            range_feet: 500
            exact_destination: true
          companion:
            maximum: 1
            willing: true
            initial_distance_feet: 5
            destination_distance_from_caster_feet: 5
          failure:
            if_any_destination:
            - creature_occupied
            - completely_object_filled
            teleport_fails_for_all: true
            damage_to_all_travelers:
              amount:
                dice:
                  count: 4
                  sides: 6
                  add: 0
              type: force
    contract:
      id: dnd2024:spell_dimension_door
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - teleport_result
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - movement
      - inventory
      - choices
    requires:
    - dnd2024:spell_dimension_door
    citations:
    - SRD 5.2.1 pp. 123, 124; builtin/srd-5.2.1/spell/dimension_door.md
  dnd2024:spells/dispel_magic:
    source_sha256: a7613ef509554f37c461a40cea762282fa9e0cde0d9f8b501986568e4844dafb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/dispel_magic
        implementation: dnd2024:spell_dispel_magic
        parameters:
          target:
            kind: creature_object_or_magical_effect
            count: 1
            visible_to_caster: false
          ongoing_spells:
            each: true
            automatic_end_max_level:
              binding: cast_level
            higher_level:
              check_ability: spellcasting
              dc:
                op: add
                args:
                - 10
                - binding: affected_spell_level
              success: end_spell
    contract:
      id: dnd2024:spell_dispel_magic
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
        affected_spell_level: integer
      outputs:
      - checks
      - ended_spells
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - inventory
      - checks
    requires:
    - dnd2024:spell_dispel_magic
    citations:
    - SRD 5.2.1 pp. 124; builtin/srd-5.2.1/spell/dispel_magic.md
  dnd2024:spells/divine_smite:
    source_sha256: 78ba62c95c6a0c53bf4ca3670f3c1972a93201621d976947126ec64d7ab629ad
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/divine_smite
        implementation: dnd2024:spell_divine_smite
        parameters:
          target:
            source: triggering_attack_target
          trigger:
            event: attack.hit
            weapon_kinds:
            - melee_weapon
            - unarmed_strike
            timing: immediately_after
          damage:
            amount:
              dice:
                count: 2
                sides: 8
                add: 0
            type: radiant
            extra_attack_damage: true
            bonus_if_type:
            - fiend
            - undead
            type_bonus:
              dice:
                count: 1
                sides: 8
                add: 0
          scaling:
            per_slot_above: 1
            damage_dice: 1
            sides: 8
    contract:
      id: dnd2024:spell_divine_smite
      inputs:
        caster: participant
        cast_level: integer
        spell_attack_modifier:
          type: integer
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - extra_attack_damage
      events:
      - spell.cast_completed
      - attack.hit
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
      - inventory
    requires:
    - dnd2024:spell_divine_smite
    citations:
    - SRD 5.2.1 pp. 125; builtin/srd-5.2.1/spell/divine_smite.md
  dnd2024:spells/dominate_person:
    source_sha256: 1a2d20d646bde1ba119ea02e78ed6c6d3ebc703b5f0a0a8b259720fa525dd699
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/dominate_person
        implementation: dnd2024:spell_dominate_person
        parameters:
          target:
            kind: humanoid
            count: 1
            visible_to_caster: true
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              condition: charmed
            success:
              effect: none
          initial_save_advantage_if: caster_or_allies_fighting_target
          repeat_save:
            event: damage.applied
            subject: target
            ability: wisdom
            success: end_spell_on_self
          link:
            telepathic: true
            same_plane_required: true
            command_on: caster_turn
            command_action_cost: none
            obey: best_effort_on_own_turn
            after_completed_without_new_order: free_actions_focused_on_self_protection
            reaction_command_cost: caster_reaction
          duration_by_slot:
            5:
              minutes: 1
            6:
              minutes: 10
            7:
              hours: 1
            8:
              hours: 8
            9:
              hours: 8
          concentration: true
    contract:
      id: dnd2024:spell_dominate_person
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - saves
      - charmed
      - telepathic_link
      - commands
      events:
      - spell.cast_completed
      - damage.applied
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - conditions
      - time
      - relationships
    requires:
    - dnd2024:spell_dominate_person
    citations:
    - SRD 5.2.1 pp. 126; builtin/srd-5.2.1/spell/dominate_person.md
  dnd2024:spells/earthquake:
    source_sha256: e326463a35dddacbee4bf78b8ce8cecffa98619a5a6fb86fdefb0dbf8b7c6c8c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/earthquake
        implementation: dnd2024:spell_earthquake
        parameters:
          area:
            shape: circle
            radius_feet: 100
            ground: true
            origin_visible: true
            difficult_terrain: true
          creature_effect:
            triggers:
            - spell.cast_completed
            - caster.turn.end
            target: creatures_on_ground_in_area
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                condition: prone
                break_concentration: true
              success:
                effect: none
          optional_effects:
            fissures:
              event: caster.turn.end
              turn: cast_turn
              count:
                dice:
                  count: 1
                  sides: 6
                  add: 0
              locations_chooser: caster
              under_structures: false
              depth_feet:
                op: multiply
                args:
                - dice:
                    count: 1
                    sides: 10
                    add: 0
                - 10
              width_feet: 10
              extent: area_edge_to_edge
              overlapping_creature_save:
                ability: dexterity
                dc:
                  binding: save_dc
                failure:
                  fall_into_fissure: true
                success:
                  move_with_fissure_edge: true
            structures:
              triggers:
              - spell.cast_completed
              - caster.turn.end
              eligible: ground_contact_in_area
              damage:
                amount: 50
                type: bludgeoning
              collapse_at_hp: 0
              collapse_creature_radius:
                structure_height_fraction:
                  numerator: 1
                  denominator: 2
              save:
                ability: dexterity
                dc:
                  binding: save_dc
                failure:
                  damage:
                    amount:
                      dice:
                        count: 12
                        sides: 6
                        add: 0
                    type: bludgeoning
                  condition: prone
                  buried: true
                success:
                  damage_fraction:
                    numerator: 1
                    denominator: 2
                  buried: false
                  prone: false
              escape:
                action: action
                ability: strength
                skill: athletics
                dc: 20
    contract:
      id: dnd2024:spell_earthquake
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - area_effect
      - saves
      - prone
      - concentration_ended
      - fissures
      - structure_damage
      - collapse
      - burial
      events:
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - conditions
      - spatial_areas
      - movement
      - choices
      - time
      - checks
    requires:
    - dnd2024:spell_earthquake
    citations:
    - SRD 5.2.1 pp. 127; builtin/srd-5.2.1/spell/earthquake.md
  dnd2024:spells/enlarge_reduce:
    source_sha256: 59a156b859471cae4625b0cc37f10303d2fb9f478a94db304c5fbbf3055da81d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/enlarge_reduce
        implementation: dnd2024:spell_enlarge_reduce
        parameters:
          target:
            kind: creature_or_object
            count: 1
            visible_to_caster: true
            object_not_worn_or_carried: true
          unwilling_creature:
            may_save: true
            ability: constitution
            success: no_effect
          equipment:
            resize_worn_and_carried: true
            drop: revert_immediately
            thrown_weapon_or_ammunition: revert_after_hit_or_miss
          choice:
          - enlarge
          - reduce
          effects:
            enlarge:
              size_category_delta: 1
              strength_checks: advantage
              strength_saves: advantage
              weapon_and_unarmed_hit_damage_bonus:
                dice:
                  count: 1
                  sides: 4
                  add: 0
            reduce:
              size_category_delta: -1
              strength_checks: disadvantage
              strength_saves: disadvantage
              weapon_and_unarmed_hit_damage_subtract:
                dice:
                  count: 1
                  sides: 4
                  add: 0
              minimum_damage: 1
    contract:
      id: dnd2024:spell_enlarge_reduce
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - save
      - size_effect
      - equipment_effect
      - attack_damage_modifier
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - forms
      - inventory
      - choices
      - checks
    requires:
    - dnd2024:spell_enlarge_reduce
    citations:
    - SRD 5.2.1 pp. 127, 128; builtin/srd-5.2.1/spell/enlarge_reduce.md
  dnd2024:spells/etherealness:
    source_sha256: af47e75e6ca8ed88d4b83b636baf630d2899d7e000289ada523577ce8dd3fa02
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/etherealness
        implementation: dnd2024:spell_etherealness
        parameters:
          target:
            kind: self
            count: 1
            visible_to_caster: false
          plane:
            destination: border_ethereal
            must_border_origin: true
            instant_end_if_origin:
            - ethereal
            - nonbordering_plane
          movement:
            any_direction: true
            vertical_additional_cost_per_foot: 1
          perception:
            origin_plane: true
            origin_plane_max_feet: 60
            origin_plane_color: gray
          interaction:
            can_affect_and_be_affected_by: ethereal_entities_only
            nonethereal_perception_and_interaction: requires_feature
          end:
            return: corresponding_origin_plane_space
            occupied: shunt_nearest_unoccupied
            force_damage_per_shunted_foot: 2
          upcast_targets:
            per_slot_above: 7
            maximum_per_level: 3
            includes_caster: true
            willing: true
            within_caster_feet: 10
    contract:
      id: dnd2024:spell_etherealness
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - planar_transition
      - movement_and_perception_effect
      - return_transition
      - shunt_damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - movement
      - perception
      - time
    requires:
    - dnd2024:spell_etherealness
    citations:
    - SRD 5.2.1 pp. 128; builtin/srd-5.2.1/spell/etherealness.md
  dnd2024:spells/faerie_fire:
    source_sha256: c1cf5a11ac10c982047c847d9f879ce33534bd8c643de7630ffa377051a772ad
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/faerie_fire
        implementation: dnd2024:spell_faerie_fire
        parameters:
          area:
            shape: cube
            side_feet: 20
          color_choice:
          - blue
          - green
          - violet
          objects:
            automatic: true
          creatures:
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                outlined: true
              success:
                effect: none
          outlined:
            dim_light_radius_feet: 10
            invisible_benefit: false
            attack_rolls_against: advantage_if_attacker_sees_target
    contract:
      id: dnd2024:spell_faerie_fire
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        spell_attack_modifier:
          type: integer
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - saves
      - outlined_effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - attacks
      - spatial_areas
      - perception
      - inventory
      - choices
    requires:
    - dnd2024:spell_faerie_fire
    citations:
    - SRD 5.2.1 pp. 129; builtin/srd-5.2.1/spell/faerie_fire.md
  dnd2024:spells/feather_fall:
    source_sha256: 71e3e7bba0dfba6b2a5f87abc6ca576907b5c295dc45a3b9a21cf5f1716f1b9b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/feather_fall
        implementation: dnd2024:spell_feather_fall
        parameters:
          target:
            kind: creature
            count:
              maximum: 5
            visible_to_caster: false
            falling: true
          descent_feet_per_round: 60
          landing:
            before_spell_end: true
            fall_damage: 0
            end_spell_on_landed_target: true
    contract:
      id: dnd2024:spell_feather_fall
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - descent_effect
      - prevented_fall_damage
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
    requires:
    - dnd2024:spell_feather_fall
    citations:
    - SRD 5.2.1 pp. 130; builtin/srd-5.2.1/spell/feather_fall.md
  dnd2024:spells/find_traps:
    source_sha256: d0d880180cbd402ccd2baf4db24b014233662bf487bfccc51629bff969b2c191
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/find_traps
        implementation: dnd2024:spell_find_traps
        parameters:
          detection:
            within_range: true
            line_of_sight: true
            eligible: object_or_mechanism_created_to_cause_damage_or_danger
            includes_spells:
            - alarm
            - glyph_of_warding
            excludes:
            - natural_floor_weakness
            - unstable_ceiling
            - hidden_sinkhole
            outputs:
            - presence
            - general_danger_nature
            reveal_location: false
    contract:
      id: dnd2024:spell_find_traps
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - trap_presence
      - danger_nature
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - inventory
    requires:
    - dnd2024:spell_find_traps
    citations:
    - SRD 5.2.1 pp. 131; builtin/srd-5.2.1/spell/find_traps.md
  dnd2024:spells/fire_storm:
    source_sha256: 786949484c9f504db1d6be917cd757db64cdb5f0839e3083eeead94fd6b6b53c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/fire_storm
        implementation: dnd2024:spell_fire_storm
        parameters:
          area:
            shape: cubes
            maximum_count: 10
            side_feet: 10
            each_contiguous_with_another: true
            arrangement_chooser: caster
          save:
            ability: dexterity
            dc:
              binding: save_dc
            failure:
              damage:
                amount:
                  dice:
                    count: 7
                    sides: 10
                    add: 0
                type: fire
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          objects:
            flammable: true
            not_worn_or_carried: true
            effect: burning
    contract:
      id: dnd2024:spell_fire_storm
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - saves
      - damage
      - burning_objects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - spatial_areas
      - inventory
      - choices
    requires:
    - dnd2024:spell_fire_storm
    citations:
    - SRD 5.2.1 pp. 132; builtin/srd-5.2.1/spell/fire_storm.md
  dnd2024:spells/flaming_sphere:
    source_sha256: ae450aee5c3bd8816a27346914220636ffa45b1b997ec30e4fb5fdec671e9c60
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/flaming_sphere
        implementation: dnd2024:spell_flaming_sphere
        parameters:
          sphere:
            diameter_feet: 5
            initial_space: unoccupied_ground
            bright_radius_feet: 20
            additional_dim_feet: 20
          damage_trigger:
            event: turn.end
            distance_feet: 5
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  amount:
                    dice:
                      count: 2
                      sides: 6
                      add: 0
                  type: fire
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
          move:
            action: bonus_action
            maximum_feet: 30
            along_ground: true
            barrier_max_height_feet: 5
            pit_max_width_feet: 10
            creature_space_entry:
              repeat_damage_save: true
              stop_movement_for_turn: true
          objects:
            on_touch: true
            flammable_unworn_uncarried: burning
          scaling:
            per_slot_above: 2
            damage_dice: 1
            sides: 6
    contract:
      id: dnd2024:spell_flaming_sphere
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - sphere_effect
      - saves
      - damage
      - burning_objects
      events:
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - spatial_areas
      - movement
      - inventory
      - time
    requires:
    - dnd2024:spell_flaming_sphere
    citations:
    - SRD 5.2.1 pp. 132; builtin/srd-5.2.1/spell/flaming_sphere.md
  dnd2024:spells/fog_cloud:
    source_sha256: dbd26817cd76a6deb6edbbd5ae6e9f6a9b4f29ea8d530263fda974d0ddfe934d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/fog_cloud
        implementation: dnd2024:spell_fog_cloud
        parameters:
          area:
            shape: sphere
            radius_feet: 20
            heavily_obscured: true
          end:
            strong_wind: disperse
          scaling:
            per_slot_above: 1
            radius_feet: 20
    contract:
      id: dnd2024:spell_fog_cloud
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - fog_area
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - spatial_areas
    requires:
    - dnd2024:spell_fog_cloud
    citations:
    - SRD 5.2.1 pp. 133; builtin/srd-5.2.1/spell/fog_cloud.md
  dnd2024:spells/freedom_of_movement:
    source_sha256: 16ae66dc402f4e46091360e3cf25fa2b1df076041314fdd75f23b0603693f2c0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/freedom_of_movement
        implementation: dnd2024:spell_freedom_of_movement
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
            willing: true
            touch: true
          effects:
            ignore_difficult_terrain: true
            magical_speed_reduction: false
            magically_imposed_conditions_prevented:
            - paralyzed
            - restrained
            swim_speed:
              stat:
                of: target
                key: speed
          escape:
            cost_movement_feet: 5
            automatic: true
            eligible:
            - nonmagical_restraints
            - nonmagical_grapple
          scaling:
            per_slot_above: 4
            additional_targets: 1
    contract:
      id: dnd2024:spell_freedom_of_movement
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - movement_effect
      - condition_protection
      - escape
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - conditions
      - movement
      - relationships
    requires:
    - dnd2024:spell_freedom_of_movement
    citations:
    - SRD 5.2.1 pp. 134; builtin/srd-5.2.1/spell/freedom_of_movement.md
  dnd2024:spells/geas:
    source_sha256: bbb69d9886f2df42e07167a14d0ea6047e285b15e58e8ea6792ed8d4358c635f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/geas
        implementation: dnd2024:spell_geas
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: true
          command:
            verbal: true
            choice:
            - perform_service
            - refrain_action_or_activity
            certain_death_command: end_spell
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              condition: charmed
            success:
              effect: none
          automatic_success_if: cannot_understand_command
          violation:
            event: target.acts_directly_counter_to_command
            damage:
              amount:
                dice:
                  count: 5
                  sides: 10
                  add: 0
              type: psychic
            maximum_per_day: 1
            requires: charmed_by_spell
          end_by_spells:
          - remove_curse
          - greater_restoration
          - wish
          duration_by_slot:
            5:
              days: 30
            6:
              days: 30
            7:
              days: 365
            8:
              days: 365
            9:
              until: named_ending_spell
    contract:
      id: dnd2024:spell_geas
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - save
      - charmed
      - command
      - violation_damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - conditions
      - movement
      - forms
      - choices
      - time
      - relationships
    requires:
    - dnd2024:spell_geas
    citations:
    - SRD 5.2.1 pp. 135; builtin/srd-5.2.1/spell/geas.md
  dnd2024:spells/globe_of_invulnerability:
    source_sha256: 6fda99b5826f0d215d5095c7ca8190531206ee35dfd0228a06fe76e7f6629095
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/globe_of_invulnerability
        implementation: dnd2024:spell_globe_of_invulnerability
        parameters:
          barrier:
            shape: emanation
            radius_feet: 10
            origin: caster_initial_location
            stationary: true
          blocked_spells:
            cast_from: outside_barrier
            maximum_level:
              op: subtract
              args:
              - binding: cast_level
              - 1
            targeting_inside_allowed: true
            affect_inside: false
            area_excludes_interior: true
    contract:
      id: dnd2024:spell_globe_of_invulnerability
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - spell_barrier
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - spatial_areas
    requires:
    - dnd2024:spell_globe_of_invulnerability
    citations:
    - SRD 5.2.1 pp. 136; builtin/srd-5.2.1/spell/globe_of_invulnerability.md
  dnd2024:spells/greater_invisibility:
    source_sha256: 3676e8f4953575cca13ea1b2191f1a045122da5b6dc6083ceb0d7736604818a8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/greater_invisibility
        implementation: dnd2024:spell_greater_invisibility
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
            touch: true
          condition:
            name: invisible
            duration: spell
    contract:
      id: dnd2024:spell_greater_invisibility
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - invisible_effect
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - conditions
      - perception
      - time
    requires:
    - dnd2024:spell_greater_invisibility
    citations:
    - SRD 5.2.1 pp. 137; builtin/srd-5.2.1/spell/greater_invisibility.md
  dnd2024:spells/guidance:
    source_sha256: ba37f5684bf59cd709200142e16948bc513803b48daa332f93d69186c0a55498
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/guidance
        implementation: dnd2024:spell_guidance
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
            willing: true
            touch: true
          skill_choice:
            chooser: caster
            selection: skill
          bonus:
            roll: ability_check
            requires_skill: chosen
            amount:
              dice:
                count: 1
                sides: 4
                add: 0
            all_matching_checks: true
    contract:
      id: dnd2024:spell_guidance
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - skill_bonus_effect
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - choices
      - checks
    requires:
    - dnd2024:spell_guidance
    citations:
    - SRD 5.2.1 pp. 138; builtin/srd-5.2.1/spell/guidance.md
  dnd2024:spells/hallucinatory_terrain:
    source_sha256: cf6fe9e0e38c2bf89502df65c333490b1e5d7db46406c74093b3fd0d170822ee
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/hallucinatory_terrain
        implementation: dnd2024:spell_hallucinatory_terrain
        parameters:
          area:
            shape: cube
            side_feet: 150
            target: natural_terrain
          illusion:
            replacement_choice: natural_terrain_kind
            senses:
            - sight
            - hearing
            - smell
            tactile_unchanged: true
            excluded:
            - manufactured_structures
            - equipment
            - creatures
          discern:
            touch_obvious_difference: automatic
            otherwise:
              action: study
              ability: intelligence
              skill: investigation
              dc:
                binding: save_dc
              success: disbelieve
            after_discern: vague_image_over_real_terrain
    contract:
      id: dnd2024:spell_hallucinatory_terrain
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - terrain_illusion
      - discernment_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - spatial_areas
      - perception
      - inventory
      - choices
      - checks
    requires:
    - dnd2024:spell_hallucinatory_terrain
    citations:
    - SRD 5.2.1 pp. 139; builtin/srd-5.2.1/spell/hallucinatory_terrain.md
  dnd2024:spells/healing_word:
    source_sha256: 31ccd8b23546e1746e582dd132c98a086e7e26e2442a7bf07afb4bb67d74faff
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/healing_word
        implementation: dnd2024:spell_healing_word
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: true
          healing:
            dice:
              count: 2
              sides: 4
              add:
                binding: spellcasting_modifier
          scaling:
            per_slot_above: 1
            healing_dice: 2
            sides: 4
    contract:
      id: dnd2024:spell_healing_word
      inputs:
        caster: participant
        cast_level: integer
        spellcasting_modifier: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - healing
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - healing
    requires:
    - dnd2024:spell_healing_word
    citations:
    - SRD 5.2.1 pp. 139; builtin/srd-5.2.1/spell/healing_word.md
  dnd2024:spells/heroism:
    source_sha256: 387b7a9380c580d74d0bf86d529201660cf6ec71781e6b61b42fa90f35fd465d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/heroism
        implementation: dnd2024:spell_heroism
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
            willing: true
            touch: true
          condition_immunity:
          - frightened
          temporary_hp:
            event: turn.start
            subject: target
            amount:
              binding: spellcasting_modifier
          scaling:
            per_slot_above: 1
            additional_targets: 1
    contract:
      id: dnd2024:spell_heroism
      inputs:
        caster: participant
        cast_level: integer
        spellcasting_modifier: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - condition_immunity
      - temporary_hp
      events:
      - spell.cast_completed
      - turn.start
      capabilities:
      - spellcasting
      - effects
      - healing
      - conditions
      - time
    requires:
    - dnd2024:spell_heroism
    citations:
    - SRD 5.2.1 pp. 140; builtin/srd-5.2.1/spell/heroism.md
  dnd2024:spells/hold_person:
    source_sha256: f3d0b49e8296e77f801eb3a53a6080b7971575b56235ec0106030ba780379652
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/hold_person
        implementation: dnd2024:spell_hold_person
        parameters:
          target:
            kind: humanoid
            count: 1
            visible_to_caster: true
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              condition: paralyzed
              duration: spell
            success:
              effect: none
          repeat_save:
            event: turn.end
            ability: wisdom
            success: end_spell_on_self
          scaling:
            per_slot_above: 2
            additional_targets: 1
    contract:
      id: dnd2024:spell_hold_person
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - saves
      - paralyzed
      events:
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - conditions
      - time
    requires:
    - dnd2024:spell_hold_person
    citations:
    - SRD 5.2.1 pp. 141; builtin/srd-5.2.1/spell/hold_person.md
  dnd2024:spells/ice_knife:
    source_sha256: 3823e1331dd8fa558a9e3b210590ce88ce8bdb946b3cc9ad0f0a057157ec0a2d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/ice_knife
        implementation: dnd2024:spell_ice_knife
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
          attack:
            kind: ranged_spell
            hit:
              damage:
                amount:
                  dice:
                    count: 1
                    sides: 10
                    add: 0
                type: piercing
          explosion:
            after:
            - hit
            - miss
            targets:
            - original_target
            - creatures_within_5_feet_of_target
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  amount:
                    dice:
                      count: 2
                      sides: 6
                      add: 0
                  type: cold
              success:
                effect: none
          scaling:
            per_slot_above: 1
            cold_damage_dice: 1
            sides: 6
    contract:
      id: dnd2024:spell_ice_knife
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        spell_attack_modifier: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - attack_result
      - saves
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - attacks
    requires:
    - dnd2024:spell_ice_knife
    citations:
    - SRD 5.2.1 pp. 141; builtin/srd-5.2.1/spell/ice_knife.md
  dnd2024:spells/imprisonment:
    source_sha256: 9e62770b152170e57c8254a2084d3f6a5db75248a8831b3ad8d3346aedd44ac7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/imprisonment
        implementation: dnd2024:spell_imprisonment
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: true
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              imprisoned: true
            success:
              unaffected: true
              immunity_to_spell_hours: 24
          imprisoned:
            breathing_required: false
            food_required: false
            drink_required: false
            aging: false
            divination_locate_or_perceive: false
            teleport: false
          mode_choice:
          - burial
          - chaining
          - hedged_prison
          - minimus_containment
          - slumber
          modes:
            burial:
              location: beneath_earth
              container: hollow_force_globe
              size: just_fits_target
              pass_in_or_out: false
            chaining:
              condition: restrained
              anchored: ground
              move_by_any_means: false
            hedged_prison:
              location: demiplane
              teleportation: false
              planar_travel: false
              appearance_choice: prison_form
            minimus_containment:
              height_inches: 1
              container: gemstone_or_similar_object
              container_indestructible: true
              light_passes: true
              anything_else_passes: false
            slumber:
              condition: unconscious
              can_awaken: false
          ending_trigger:
            chooser: caster
            decision_type: observable_action_predicate
            gm_approval_required: true
            gm_constraint:
              high_likelihood_within_years: 10
            chosen_at: cast
          dispel_magic:
            required_slot_level: 9
            eligible_targets:
            - prison
            - casting_component
    contract:
      id: dnd2024:spell_imprisonment
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - save
      - immunity
      - prison_effect
      - ending_trigger
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - conditions
      - movement
      - perception
      - forms
      - inventory
      - choices
      - time
    requires:
    - dnd2024:spell_imprisonment
    citations:
    - SRD 5.2.1 pp. 142; builtin/srd-5.2.1/spell/imprisonment.md
  dnd2024:spells/instant_summons:
    source_sha256: 06a7da6e6ad2349aa8bae24521abaac6ac10596defc772383d6f5d9d0e3a2b15
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/instant_summons
        implementation: dnd2024:spell_instant_summons
        parameters:
          target:
            kind: object
            count: 1
            visible_to_caster: false
            touch: true
            maximum_weight_pounds: 10
            maximum_dimension_feet: 6
          sapphire:
            touch: true
            different_each_cast: true
            invisible_inscription: object_name
          mark:
            invisible: true
            'on': object
          summon:
            action: magic
            requirements:
            - speak_object_name
            - crush_sapphire
            range: any_physical_or_planar_distance
            success:
              move_object_to: caster_hand
              end_spell: true
            held_or_carried_by_other_creature:
              transport: false
              reveal:
              - holder_identity
              - holder_current_location
    contract:
      id: dnd2024:spell_instant_summons
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - object_mark
      - sapphire_link
      - object_transport
      - holder_information
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - movement
      - perception
      - inventory
      - summoning
    requires:
    - dnd2024:spell_instant_summons
    citations:
    - SRD 5.2.1 pp. 143; builtin/srd-5.2.1/spell/instant_summons.md
  dnd2024:spells/knock:
    source_sha256: a1eb22cb343a0e435cd7802696b6c1ac0350eb60941bcc07025e5827278f7649
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/knock
        implementation: dnd2024:spell_knock
        parameters:
          target:
            kind: access_preventing_object
            count: 1
            visible_to_caster: true
          mundane:
            unlock_locks_maximum: 1
            unstick: true
            unbar: true
          arcane_lock:
            suppress_minutes: 10
            can_open_and_close: true
          sound:
            origin: target
            audible_feet: 300
    contract:
      id: dnd2024:spell_knock
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - access_state
      - suppressed_arcane_lock
      - sound
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - inventory
      - time
    requires:
    - dnd2024:spell_knock
    citations:
    - SRD 5.2.1 pp. 143, 144; builtin/srd-5.2.1/spell/knock.md
  dnd2024:spells/light:
    source_sha256: 4cebf1ca3ed3e5cb5c1df6320ddcdee0df9f1311424478a8c9608035d3fbd492
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/light
        implementation: dnd2024:spell_light
        parameters:
          target:
            kind: object
            count: 1
            visible_to_caster: false
            touch: true
            maximum_size: large
            not_worn_or_carried_by_other: true
          light:
            bright_radius_feet: 20
            additional_dim_feet: 20
            color_chooser: caster
            opaque_cover_blocks: true
          end:
            event: caster.recasts_same_spell
    contract:
      id: dnd2024:spell_light
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - light_effect
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - perception
      - inventory
      - choices
    requires:
    - dnd2024:spell_light
    citations:
    - SRD 5.2.1 pp. 144; builtin/srd-5.2.1/spell/light.md
  dnd2024:spells/locate_object:
    source_sha256: 58a754c9132d7b8a375292d85d71ab531d5c7ac9f27257864cab0ee6b4316743
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/locate_object
        implementation: dnd2024:spell_locate_object
        parameters:
          selection:
            specific_object:
              familiar: true
              describe_or_name: true
              seen_at_least_once_within_feet: 30
            object_kind:
              nearest: true
          detection:
            maximum_distance_feet: 1000
            outputs:
            - direction_to_object
            - movement_direction_if_moving
          blocked:
            material: lead
            thickness: any
            path: direct_caster_to_object
    contract:
      id: dnd2024:spell_locate_object
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - object_direction
      - movement_direction
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - movement
      - inventory
    requires:
    - dnd2024:spell_locate_object
    citations:
    - SRD 5.2.1 pp. 145; builtin/srd-5.2.1/spell/locate_object.md
  dnd2024:spells/magic_circle:
    source_sha256: 24631119786e7f77b3af30181db9c6845c5825d0c7eb259908bf25972ed552a3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/magic_circle
        implementation: dnd2024:spell_magic_circle
        parameters:
          area:
            shape: cylinder
            radius_feet: 10
            height_feet: 20
            origin: visible_ground_point
            glowing_runes_on_surface_intersections: true
          type_choice:
            minimum: 1
            options:
            - celestial
            - elemental
            - fey
            - fiend
            - undead
          mode_choice:
          - normal
          - reversed
          normal:
            nonmagical_willing_entry: false
            teleport_or_interplanar_entry:
              save_ability: charisma
              success_required: true
            attack_rolls_against_inside: disadvantage
            protect_inside_against:
            - possession
            - charmed
            - frightened
          reversed:
            restricted_direction: exit
            protected_targets: outside
          scaling:
            per_slot_above: 3
            duration_hours: 1
    contract:
      id: dnd2024:spell_magic_circle
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        spell_attack_modifier:
          type: integer
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - barrier_effect
      - saves
      - attack_modifier
      - condition_protection
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - attacks
      - conditions
      - spatial_areas
      - movement
      - choices
      - time
      - relationships
    requires:
    - dnd2024:spell_magic_circle
    citations:
    - SRD 5.2.1 pp. 145, 146; builtin/srd-5.2.1/spell/magic_circle.md
  dnd2024:spells/magic_weapon:
    source_sha256: e85f680b3d9f8a9805d74bd2e44cff44833aea5759c6dd079882cd5081aa6e21
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/magic_weapon
        implementation: dnd2024:spell_magic_weapon
        parameters:
          target:
            kind: weapon
            count: 1
            visible_to_caster: false
            touch: true
            magical: false
          magic_weapon: true
          attack_and_damage_bonus_by_slot:
            2: 1
            3: 2
            4: 2
            5: 2
            6: 3
            7: 3
            8: 3
            9: 3
          end:
            event: caster.recasts_same_spell
    contract:
      id: dnd2024:spell_magic_weapon
      inputs:
        caster: participant
        cast_level: integer
        spell_attack_modifier:
          type: integer
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - weapon_enchantment
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
      - inventory
    requires:
    - dnd2024:spell_magic_weapon
    citations:
    - SRD 5.2.1 pp. 146, 147; builtin/srd-5.2.1/spell/magic_weapon.md
  dnd2024:spells/mass_heal:
    source_sha256: ededeab0fd209d96768a26c98eef0559f7c0fc198f0c90268386f29102f7c3bf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mass_heal
        implementation: dnd2024:spell_mass_heal
        parameters:
          target:
            kind: creature
            count: any_number
            visible_to_caster: true
          healing:
            pool_maximum: 700
            allocation_chooser: caster
            integer_nonnegative_per_target: true
          healed_targets_remove_conditions:
          - blinded
          - deafened
          - poisoned
    contract:
      id: dnd2024:spell_mass_heal
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - healing_allocation
      - removed_conditions
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - healing
      - conditions
      - movement
      - choices
    requires:
    - dnd2024:spell_mass_heal
    citations:
    - SRD 5.2.1 pp. 147; builtin/srd-5.2.1/spell/mass_heal.md
  dnd2024:spells/meld_into_stone:
    source_sha256: fbaf60f68b2ff0296578a09f62e294d3222a6c26e49d6f831751383339ab7351
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/meld_into_stone
        implementation: dnd2024:spell_meld_into_stone
        parameters:
          stone:
            kind:
            - object
            - surface
            touch: true
            must_fully_contain_body: true
          merge:
            subject: caster_and_equipment
            nonmagical_detection: false
            outside_vision: false
            outside_hearing_perception_checks: disadvantage
            time_awareness: true
            cast_spells_on_self: true
            movement: false
          exit:
            movement_cost_feet: 5
            location: entry_point
            ends_spell: true
          stone_damage:
            minor: no_harm
            partial_destruction_or_shape_change_no_longer_fits:
              expel: true
              damage:
                amount:
                  dice:
                    count: 6
                    sides: 6
                    add: 0
                type: force
            complete_destruction_or_transmutation:
              expel: true
              damage:
                amount: 50
                type: force
          expulsion:
            destination: nearest_unoccupied_to_entry
            condition: prone
    contract:
      id: dnd2024:spell_meld_into_stone
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - merge_effect
      - expulsion
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
      - movement
      - perception
      - inventory
      - checks
    requires:
    - dnd2024:spell_meld_into_stone
    citations:
    - SRD 5.2.1 pp. 148; builtin/srd-5.2.1/spell/meld_into_stone.md
  dnd2024:spells/mind_blank:
    source_sha256: be81dc3e21ad9eaa8d2036ca4e920e3e0ba20223fa8354181a2413e579d1e18b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mind_blank
        implementation: dnd2024:spell_mind_blank
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
            willing: true
            touch: true
          immunities:
            damage:
            - psychic
            conditions:
            - charmed
            effects:
            - sense_emotions
            - sense_alignment
            - read_thoughts
            - magically_detect_location
            spells:
            - gather_information_about_target
            - remote_observation
            - mind_control
            includes_wish: true
    contract:
      id: dnd2024:spell_mind_blank
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - immunity_effect
      - information_protection
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
      - perception
      - forms
      - knowledge
      - relationships
    requires:
    - dnd2024:spell_mind_blank
    citations:
    - SRD 5.2.1 pp. 149; builtin/srd-5.2.1/spell/mind_blank.md
  dnd2024:spells/mirror_image:
    source_sha256: 595c87eb9779604567054b62607d85e986ada9de63976fb28aa6f37d11417a40
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mirror_image
        implementation: dnd2024:spell_mirror_image
        parameters:
          duplicates:
            initial_count: 3
            same_space: true
            follow_and_mimic: true
            indistinguishable: true
          trigger:
            event: attack.hit
            target: caster
            roll:
              dice_per_remaining_duplicate: 1
              sides: 6
            if_any_die_gte: 3
            replace_hit_target: one_duplicate
            destroy_hit_duplicate: true
          duplicates_ignore: all_other_damage_and_effects
          end:
            remaining_duplicates: 0
          attacker_bypass:
            any:
            - blinded
            - blindsight
            - truesight
    contract:
      id: dnd2024:spell_mirror_image
      inputs:
        caster: participant
        cast_level: integer
        spell_attack_modifier:
          type: integer
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - duplicate_pool
      - redirected_hit
      events:
      - spell.cast_completed
      - attack.hit
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
      - conditions
    requires:
    - dnd2024:spell_mirror_image
    citations:
    - SRD 5.2.1 pp. 150; builtin/srd-5.2.1/spell/mirror_image.md
  dnd2024:spells/moonbeam:
    source_sha256: bc6b7af378fcaaeeea500261b8247a0a5c8b5ea672674c27af9e0943835ebdc1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/moonbeam
        implementation: dnd2024:spell_moonbeam
        parameters:
          area:
            shape: cylinder
            radius_feet: 5
            height_feet: 40
            light: dim
          move:
            action: magic
            timing: later_caster_turns
            maximum_feet: 60
          triggers:
          - spell.cast_completed
          - area.moves_into_creature
          - area.enter
          - turn.end
          save:
            ability: constitution
            dc:
              binding: save_dc
            failure:
              damage:
                amount:
                  dice:
                    count: 2
                    sides: 10
                    add: 0
                type: radiant
              if_shape_shifted:
                revert_true_form: true
                prevent_shape_shift_until: leaves_cylinder
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
              revert: false
          limit:
            saves_per_creature_per_turn: 1
          scaling:
            per_slot_above: 2
            damage_dice: 1
            sides: 10
    contract:
      id: dnd2024:spell_moonbeam
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - area_effect
      - saves
      - damage
      - form_reversion
      - shape_shift_prohibition
      events:
      - spell.cast_completed
      - turn.end
      - area.enter
      - area.moved
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - spatial_areas
      - movement
      - perception
      - forms
      - time
    requires:
    - dnd2024:spell_moonbeam
    citations:
    - SRD 5.2.1 pp. 150, 151; builtin/srd-5.2.1/spell/moonbeam.md
  dnd2024:spells/passwall:
    source_sha256: 253e9cab18f7798f8b2c5ca4bc6bed5c9aa7b1040806d93e55d9d07e5c5e12d9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/passwall
        implementation: dnd2024:spell_passwall
        parameters:
          surface:
            materials:
            - wood
            - plaster
            - stone
            visible: true
          passage:
            dimensions_chooser: caster
            maximum_width_feet: 5
            maximum_height_feet: 8
            maximum_depth_feet: 20
            structural_instability: false
          end:
            subjects: creatures_and_objects_in_passage
            eject_safely: true
            destination: unoccupied_nearest_cast_surface
    contract:
      id: dnd2024:spell_passwall
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - passage
      - ejected_entities
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - inventory
      - choices
    requires:
    - dnd2024:spell_passwall
    citations:
    - SRD 5.2.1 pp. 151; builtin/srd-5.2.1/spell/passwall.md
  dnd2024:spells/planar_ally:
    source_sha256: ab7028e2c1da2d516a9042e54d2181c4d0c7a0aba3ca6f39598fe1e682c41b57
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/planar_ally
        implementation: dnd2024:spell_planar_ally
        parameters:
          patron:
            known_to_caster: true
            kind: cosmic_power_entity
          summon:
            creature_types:
            - celestial
            - elemental
            - fiend
            loyal_to_patron: true
            destination: unoccupied_in_range
            specific_name_request_optional: true
            final_creature_chooser: gm
          service:
            compelled: false
            communication_required_for_bargain: true
            task_choice: caster
            acceptance_choice: summoned_creature
            payment_forms:
            - gold
            - magic_items
            - sacrifice
            - treasure
            - quest
            price_rates:
            - unit: minute
              gp: 100
            - unit: hour
              gp: 1000
            - unit: day
              gp: 10000
              maximum_duration_days: 10
            gm_adjustment:
              decision: price_amount
              context: casting_circumstances
              ethos_aligned_options:
              - half
              - waived
              - unchanged
              nonhazardous_default_fraction:
                numerator: 1
                denominator: 2
              especially_dangerous: greater_gift
              suicidal_acceptance: rare
          return_home:
            events:
            - task_completed
            - agreed_duration_expired
            report_to_caster_first_if_possible: true
            destination: home_plane
            no_price_agreement: immediate_return
    contract:
      id: dnd2024:spell_planar_ally
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - summoned_creature
      - service_agreement
      - payment
      - return
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - forms
      - summoning
      - choices
      - time
      - relationships
    requires:
    - dnd2024:spell_planar_ally
    citations:
    - SRD 5.2.1 pp. 152; builtin/srd-5.2.1/spell/planar_ally.md
  dnd2024:spells/poison_spray:
    source_sha256: 0c37c5bd07298d1c87b428b8654dc9ab554f2ea8d983e58fdd2bb701f282eb68
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/poison_spray
        implementation: dnd2024:spell_poison_spray
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
          attack:
            kind: ranged_spell
            hit:
              damage:
                amount:
                  dice:
                    count: 1
                    sides: 12
                    add: 0
                type: poison
          cantrip_dice_by_character_level:
            1: 1
            5: 2
            11: 3
            17: 4
    contract:
      id: dnd2024:spell_poison_spray
      inputs:
        caster: participant
        cast_level: integer
        spell_attack_modifier: integer
        character_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - attack_result
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
    requires:
    - dnd2024:spell_poison_spray
    citations:
    - SRD 5.2.1 pp. 153; builtin/srd-5.2.1/spell/poison_spray.md
  dnd2024:spells/power_word_stun:
    source_sha256: dc714ce9646a96ebfa272623bb36c9156177b89d319fbc29090af85b8accb0eb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/power_word_stun
        implementation: dnd2024:spell_power_word_stun
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: true
          branches:
          - hp_lte: 150
            condition: stunned
            repeat_save:
              event: turn.end
              ability: constitution
              success: end_stunned
          - hp_gt: 150
            speed: 0
            until: caster.next_turn.start
    contract:
      id: dnd2024:spell_power_word_stun
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - stunned
      - speed_effect
      - saves
      events:
      - spell.cast_completed
      - turn.start
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - conditions
      - time
    requires:
    - dnd2024:spell_power_word_stun
    citations:
    - SRD 5.2.1 pp. 154; builtin/srd-5.2.1/spell/power_word_stun.md
  dnd2024:spells/prismatic_wall:
    source_sha256: 6a2938d3e6e4be83cbdab37cfd8df25b1b0456ae29b0ce3fc904cde2cedbd37e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/prismatic_wall
        implementation: dnd2024:spell_prismatic_wall
        parameters:
          geometry_choice:
            wall:
              vertical: true
              maximum_length_feet: 90
              maximum_height_feet: 30
              thickness_inches: 1
            globe:
              maximum_diameter_feet: 30
          geometry:
            center: point_in_range
            opaque: true
            creature_occupied_at_creation: instant_end_no_effect
          light:
            bright_radius_feet: 100
            additional_dim_feet: 100
          exempt:
            caster: true
            designated_creatures_at_cast: true
            near_and_pass_through_unharmed: true
          near_effect:
            triggers:
            - creature.moves_within_20_feet
            - turn.start
            can_see_wall_required: true
            save:
              ability: constitution
              dc:
                binding: save_dc
              failure:
                condition: blinded
                minutes: 1
              success:
                effect: none
          passage:
            triggers:
            - creature.reaches_into_wall
            - creature.passes_through_wall
            sequential_layers: true
            save_per_layer: dexterity
          destruction:
            ac: 10
            one_layer_at_time: true
            order:
            - red
            - orange
            - yellow
            - green
            - blue
            - indigo
            - violet
            destroyed_layer_absent_until_spell_end: true
            antimagic_field_effect: false
            dispel_magic_layers:
            - violet
          layers:
          - order: 1
            color: red
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  amount:
                    dice:
                      count: 12
                      sides: 6
                      add: 0
                  type: fire
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
            blocks: nonmagical_ranged_attacks
            destroy:
              damage_type: cold
              minimum_damage: 25
          - order: 2
            color: orange
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  amount:
                    dice:
                      count: 12
                      sides: 6
                      add: 0
                  type: acid
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
            blocks: magical_ranged_attacks
            destroy:
              effect: strong_wind
          - order: 3
            color: yellow
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  amount:
                    dice:
                      count: 12
                      sides: 6
                      add: 0
                  type: lightning
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
            destroy:
              damage_type: force
              minimum_damage: 60
          - order: 4
            color: green
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  amount:
                    dice:
                      count: 12
                      sides: 6
                      add: 0
                  type: poison
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
            destroy:
              spell: passwall
              or_spell:
                minimum_level: 5
                effect: open_portal_on_solid_surface
          - order: 5
            color: blue
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  amount:
                    dice:
                      count: 12
                      sides: 6
                      add: 0
                  type: cold
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
            destroy:
              damage_type: fire
              minimum_damage: 25
          - order: 6
            color: indigo
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                condition: restrained
                repeat_save:
                  event: turn.end
                  ability: constitution
                  count_success_and_failure_separately: true
                  consecutive_required: false
                  successes_to_end_restrained: 3
                  failures_to_petrify: 3
                  petrified_until: freed_by_restoring_effect
              success:
                effect: none
            blocks: spells_cast_through
            destroy:
              effect: bright_light
              spell: daylight
          - order: 7
            color: violet
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                condition: blinded
                repeat_save:
                  event: caster.next_turn.start
                  ability: wisdom
                  success:
                    end_blinded: true
                  failure:
                    end_blinded: true
                    teleport:
                      destination: other_plane
                      chooser: gm
              success:
                effect: none
            destroy:
              spell: dispel_magic
    contract:
      id: dnd2024:spell_prismatic_wall
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        spell_attack_modifier:
          type: integer
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - wall_layers
      - saves
      - damage
      - conditions
      - planar_transition
      - destroyed_layers
      events:
      - spell.cast_completed
      - turn.start
      - turn.end
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - attacks
      - conditions
      - spatial_areas
      - movement
      - perception
      - choices
      - time
    requires:
    - dnd2024:spell_prismatic_wall
    citations:
    - SRD 5.2.1 pp. 155; builtin/srd-5.2.1/spell/prismatic_wall.md
  dnd2024:spells/project_image:
    source_sha256: 521104e9fd711860d80cb2183623754f8615aa85c9aea4a20ac3086d32ccf7fb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/project_image
        implementation: dnd2024:spell_project_image
        parameters:
          image:
            appearance_and_voice: caster
            location: previously_seen_within_range
            obstacles_ignored: true
            intangible: true
            any_damage: disappear_and_end_spell
          senses:
            sight_origin: image
            hearing_origin: image
          control:
            action: magic
            move_feet: 60
            choices:
            - gesture
            - speech
            - behavior
            perfect_mannerism_mimicry: true
          discern:
            physical_interaction: automatic
            study:
              ability: intelligence
              skill: investigation
              dc:
                binding: save_dc
            after_discern:
              see_through: true
              sound: hollow
    contract:
      id: dnd2024:spell_project_image
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - image
      - perception
      - discernment_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - movement
      - perception
      - choices
      - relationships
      - checks
    requires:
    - dnd2024:spell_project_image
    citations:
    - SRD 5.2.1 pp. 156, 157; builtin/srd-5.2.1/spell/project_image.md
  dnd2024:spells/purify_food_and_drink:
    source_sha256: d04e05175b1c462431574dfbaedd3425ceb45f2ca28d76fb7d1401b6bacc66cc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/purify_food_and_drink
        implementation: dnd2024:spell_purify_food_and_drink
        parameters:
          area:
            shape: sphere
            radius_feet: 5
          eligible:
            kinds:
            - food
            - drink
            nonmagical: true
          remove:
          - poison
          - rot
    contract:
      id: dnd2024:spell_purify_food_and_drink
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - purified_food_and_drink
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - spatial_areas
      - movement
    requires:
    - dnd2024:spell_purify_food_and_drink
    citations:
    - SRD 5.2.1 pp. 157; builtin/srd-5.2.1/spell/purify_food_and_drink.md
  dnd2024:spells/ray_of_sickness:
    source_sha256: e3e5b3d23ded1bc9515ec48ad5a83f2eff02600f9537c009e604d613e9d76d65
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/ray_of_sickness
        implementation: dnd2024:spell_ray_of_sickness
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
          attack:
            kind: ranged_spell
            hit:
              damage:
                amount:
                  dice:
                    count: 2
                    sides: 8
                    add: 0
                type: poison
              condition: poisoned
              condition_until: caster.next_turn.end
          scaling:
            per_slot_above: 1
            damage_dice: 1
            sides: 8
    contract:
      id: dnd2024:spell_ray_of_sickness
      inputs:
        caster: participant
        cast_level: integer
        spell_attack_modifier: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - attack_result
      - damage
      - poisoned
      events:
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
      - conditions
      - time
    requires:
    - dnd2024:spell_ray_of_sickness
    citations:
    - SRD 5.2.1 pp. 158; builtin/srd-5.2.1/spell/ray_of_sickness.md
  dnd2024:spells/resilient_sphere:
    source_sha256: f3fbf310c891fd9d6ae3367e570cf1c59b641866eca58596a694e85dcce9c4fc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/resilient_sphere
        implementation: dnd2024:spell_resilient_sphere
        parameters:
          target:
            kind: creature_or_object
            count: 1
            visible_to_caster: false
            maximum_size: large
          unwilling_creature:
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                enclosed: true
              success:
                effect: none
          enclosure:
            blocks_in_and_out:
            - physical_objects
            - energy
            - spell_effects
            breathing_allowed: true
            sphere_damage_immunity: all
            outside_origin_damage_to_inside: false
            inside_creature_damage_to_outside: false
            weightless: true
            size: just_fits_target
          movement:
            enclosed_creature_action: action
            roll_maximum_speed_fraction:
              numerator: 1
              denominator: 2
            other_creatures_can_pick_up_and_move: true
          disintegrate:
            target_sphere: destroy_sphere
            harm_contents: false
          automatic_enclosure:
            eligible:
            - willing_creature
            - object
    contract:
      id: dnd2024:spell_resilient_sphere
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - save
      - sphere_effect
      - movement
      - destruction
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - spatial_areas
      - movement
      - inventory
    requires:
    - dnd2024:spell_resilient_sphere
    citations:
    - SRD 5.2.1 pp. 158; builtin/srd-5.2.1/spell/resilient_sphere.md
  dnd2024:spells/revivify:
    source_sha256: 5e4d2372d37c0675e0ec438f19ec47802c27d9011cf3930c5f2b4d4ca3c07479
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/revivify
        implementation: dnd2024:spell_revivify
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
            touch: true
            dead_within_minutes: 1
            death_by_old_age: false
          revive:
            hp: 1
            restore_missing_parts: false
    contract:
      id: dnd2024:spell_revivify
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - revived_creature
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - healing
      - time
    requires:
    - dnd2024:spell_revivify
    citations:
    - SRD 5.2.1 pp. 159; builtin/srd-5.2.1/spell/revivify.md
  dnd2024:spells/scorching_ray:
    source_sha256: db0da91f991d184b9de36b5a7d0fc02c8e1f0571f35c5c42ab75065c7ca71eac
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/scorching_ray
        implementation: dnd2024:spell_scorching_ray
        parameters:
          rays:
            base_count: 3
            target_assignment: same_or_different_targets_in_range
            attack_per_ray: ranged_spell
            hit:
              damage:
                amount:
                  dice:
                    count: 2
                    sides: 6
                    add: 0
                type: fire
          scaling:
            per_slot_above: 2
            additional_rays: 1
    contract:
      id: dnd2024:spell_scorching_ray
      inputs:
        caster: participant
        cast_level: integer
        spell_attack_modifier: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - attack_results
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
    requires:
    - dnd2024:spell_scorching_ray
    citations:
    - SRD 5.2.1 pp. 159; builtin/srd-5.2.1/spell/scorching_ray.md
  dnd2024:spells/see_invisibility:
    source_sha256: 6dad19f23499d21d328c924379dce94b10fc2492e8f1fdf626bd1f897aae2c93
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/see_invisibility
        implementation: dnd2024:spell_see_invisibility
        parameters:
          target:
            kind: self
            count: 1
            visible_to_caster: false
          vision:
            invisible_creatures_and_objects: as_visible
            ethereal_plane: true
            ethereal_appearance: ghostly
    contract:
      id: dnd2024:spell_see_invisibility
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - vision_effect
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - perception
      - inventory
    requires:
    - dnd2024:spell_see_invisibility
    citations:
    - SRD 5.2.1 pp. 160; builtin/srd-5.2.1/spell/see_invisibility.md
  dnd2024:spells/shapechange:
    source_sha256: 82bb0ea32a5b04acb1a2b24324a68856d1e82d55c574ccb4f83261ad7d701136
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/shapechange
        implementation: dnd2024:spell_shapechange
        parameters:
          target:
            kind: self
            count: 1
            visible_to_caster: false
          form:
            chooser: caster
            maximum_cr:
              source: caster
              stat_choice:
              - level
              - challenge_rating
            sort_previously_seen: true
            excluded_types:
            - construct
            - undead
            change_action: magic
          temporary_hp:
            on_initial_cast_only: true
            amount:
              stat:
                of: initial_form
                key: hit_points
            remaining_vanish_on_end: true
          statistics:
            replace_with: chosen_form_stat_block
            retain:
            - creature_type
            - alignment
            - personality
            - intelligence
            - wisdom
            - charisma
            - hit_points
            - hit_point_dice
            - proficiencies
            - communication_ability
            - spellcasting_feature_if_present
          equipment:
            choice_on_shape_shift:
            - drop_to_ground
            - resize_and_reshape_to_fit
            fit_duration: while_in_form
    contract:
      id: dnd2024:spell_shapechange
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - form_effect
      - temporary_hp
      - equipment_transitions
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - healing
      - forms
      - inventory
      - choices
      - time
    requires:
    - dnd2024:spell_shapechange
    citations:
    - SRD 5.2.1 pp. 161; builtin/srd-5.2.1/spell/shapechange.md
  dnd2024:spells/shillelagh:
    source_sha256: e45dcf4f260ef62f40a0609eae6b37327de9107018e9b8feb9cdb27fc3a5e56d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/shillelagh
        implementation: dnd2024:spell_shillelagh
        parameters:
          target:
            kind: held_weapon
            allowed:
            - club
            - quarterstaff
          melee_attacks:
            optional_ability_replacement: spellcasting_instead_of_strength
            applies_to:
            - attack_roll
            - damage_roll
            damage_dice_by_character_level:
              1:
                dice:
                  count: 1
                  sides: 8
                  add: 0
              5:
                dice:
                  count: 1
                  sides: 10
                  add: 0
              11:
                dice:
                  count: 1
                  sides: 12
                  add: 0
              17:
                dice:
                  count: 2
                  sides: 6
                  add: 0
            damage_type_choice:
            - force
            - weapon_normal
          end:
            events:
            - caster.recasts_same_spell
            - caster.lets_go_of_weapon
    contract:
      id: dnd2024:spell_shillelagh
      inputs:
        caster: participant
        cast_level: integer
        spell_attack_modifier:
          type: integer
          required: false
        character_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - weapon_effect
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
      - inventory
      - choices
    requires:
    - dnd2024:spell_shillelagh
    citations:
    - SRD 5.2.1 pp. 162; builtin/srd-5.2.1/spell/shillelagh.md
  dnd2024:spells/silent_image:
    source_sha256: b0b90eee24cbcfe29f58b625bdb5df3b45475724b1e529d0cb7efe921ef6b6d1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/silent_image
        implementation: dnd2024:spell_silent_image
        parameters:
          image:
            kind_choice:
            - object
            - creature
            - visible_phenomenon
            maximum_cube_feet: 15
            senses:
            - vision
            sound: false
            smell: false
            other_sensory_effects: false
          move:
            action: magic
            destination: any_point_in_range
            appearance_alteration: natural_movement
          discern:
            physical_interaction: automatic
            study:
              ability: intelligence
              skill: investigation
              dc:
                binding: save_dc
            after_discern: see_through
    contract:
      id: dnd2024:spell_silent_image
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - image
      - discernment_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - movement
      - perception
      - inventory
      - choices
      - checks
    requires:
    - dnd2024:spell_silent_image
    citations:
    - SRD 5.2.1 pp. 162; builtin/srd-5.2.1/spell/silent_image.md
  dnd2024:spells/slow:
    source_sha256: 250497db2f0dfbf1e61fb69af0605c082bea6012b8a8e2b1af5120bc9e7ad8ce
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/slow
        implementation: dnd2024:spell_slow
        parameters:
          area:
            shape: cube
            side_feet: 40
          target:
            kind: creature
            count:
              maximum: 6
            visible_to_caster: false
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              affected: true
            success:
              effect: none
          affected:
            speed_fraction:
              numerator: 1
              denominator: 2
            ac_delta: -2
            dexterity_save_delta: -2
            reactions: false
            on_turn_action_choice:
            - action
            - bonus_action
            both_action_and_bonus_action: false
            attack_action_maximum_attacks: 1
            somatic_spell_failure_percent: 25
          repeat_save:
            event: turn.end
            ability: wisdom
            success: end_spell_on_self
    contract:
      id: dnd2024:spell_slow
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        spell_attack_modifier:
          type: integer
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - saves
      - slow_effect
      - spell_failure_result
      events:
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - attacks
      - spatial_areas
      - choices
      - time
    requires:
    - dnd2024:spell_slow
    citations:
    - SRD 5.2.1 pp. 163; builtin/srd-5.2.1/spell/slow.md
  dnd2024:spells/speak_with_dead:
    source_sha256: 2c70cbe35be1e5bef34ba27840b9a47e5ff4047be7ba7bc9df3eff1d7891b1cd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/speak_with_dead
        implementation: dnd2024:spell_speak_with_dead
        parameters:
          target:
            kind: corpse
            count: 1
            visible_to_caster: false
            mouth_required: true
            undead_when_died: false
            no_same_spell_target_within_days: 10
          questions:
            maximum: 5
            knowledge: in_life_only
            languages: in_life_only
            answer_style:
            - brief
            - cryptic
            - repetitive
            truth_compulsion_if_caster_antagonistic_or_known_enemy: false
            learn_new_information: false
            comprehend_events_after_death: false
            speculate_future: false
          animation:
            returns_soul: false
            animating_spirit_only: true
    contract:
      id: dnd2024:spell_speak_with_dead
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - animated_corpse
      - questions_remaining
      - answers
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - forms
      - summoning
      - time
      - knowledge
    requires:
    - dnd2024:spell_speak_with_dead
    citations:
    - SRD 5.2.1 pp. 164; builtin/srd-5.2.1/spell/speak_with_dead.md
  dnd2024:spells/spirit_guardians:
    source_sha256: 73a03709235ce113e22af594454391c3bb87a8874f8544613b140a75c1c78042
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/spirit_guardians
        implementation: dnd2024:spell_spirit_guardians
        parameters:
          area:
            shape: emanation
            radius_feet: 15
            origin: caster
          appearance_by_alignment:
            good_or_neutral:
              choice:
              - angelic
              - fey
            evil: fiendish
          exempt:
            designated_creatures: at_cast
          affected:
            speed_fraction_while_inside:
              numerator: 1
              denominator: 2
          triggers:
          - area.moves_into_creature
          - area.enter
          - turn.end
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              damage:
                amount:
                  dice:
                    count: 3
                    sides: 8
                    add: 0
                type_by_caster_alignment:
                  good_or_neutral: radiant
                  evil: necrotic
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          limit:
            saves_per_creature_per_turn: 1
          scaling:
            per_slot_above: 3
            damage_dice: 1
            sides: 8
    contract:
      id: dnd2024:spell_spirit_guardians
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - aura_effect
      - speed_modifier
      - saves
      - damage
      events:
      - spell.cast_completed
      - turn.end
      - area.enter
      - area.moved
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - spatial_areas
      - movement
      - choices
      - time
    requires:
    - dnd2024:spell_spirit_guardians
    citations:
    - SRD 5.2.1 pp. 164, 165; builtin/srd-5.2.1/spell/spirit_guardians.md
  dnd2024:spells/stone_shape:
    source_sha256: cc531df89f4de2a5011a9bfb7fc9f0c4f28a0472f08919174a4b7645b34fec07
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/stone_shape
        implementation: dnd2024:spell_stone_shape
        parameters:
          target:
            touch: true
            choice:
            - kind: stone_object
              maximum_size: medium
            - kind: stone_section
              maximum_each_dimension_feet: 5
          shape:
            chooser: caster
            arbitrary_shape: true
            maximum_hinges: 2
            latch_allowed: true
            finer_mechanical_detail: false
    contract:
      id: dnd2024:spell_stone_shape
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - reshaped_stone
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - inventory
      - choices
    requires:
    - dnd2024:spell_stone_shape
    citations:
    - SRD 5.2.1 pp. 165; builtin/srd-5.2.1/spell/stone_shape.md
  dnd2024:spells/summon_dragon:
    source_sha256: a41e9e25e62e8f9b90805716154b3e983d586d4407aa94e68ef1e0061fd222be
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/summon_dragon
        implementation: dnd2024:spell_summon_dragon
        parameters:
          summon:
            definition:
              key: dnd2024:creatures/draconic_spirit
            initial_space: visible_unoccupied_in_range
            ally_to:
            - caster
            - caster_allies
            disappear_on:
            - hp_zero
            - spell_end
          control:
            initiative: caster_count
            turn: immediately_after_caster
            verbal_commands: true
            command_cost: none
            no_command:
              action: dodge
              movement: avoid_danger
          spirit:
            size: large
            type: dragon
            alignment: neutral
            ac:
              op: add
              args:
              - 14
              - binding: cast_level
            hp:
              op: add
              args:
              - 50
              - op: multiply
                args:
                - 10
                - op: subtract
                  args:
                  - binding: cast_level
                  - 5
            speeds_feet:
              walk: 30
              fly: 60
              swim: 30
            abilities:
              strength: 19
              dexterity: 14
              constitution: 17
              intelligence: 10
              wisdom: 14
              charisma: 14
            save_modifiers:
              strength: 4
              dexterity: 2
              constitution: 3
              intelligence: 0
              wisdom: 2
              charisma: 2
            resistances:
            - acid
            - cold
            - fire
            - lightning
            - poison
            condition_immunities:
            - charmed
            - frightened
            - poisoned
            senses_feet:
              blindsight: 30
              darkvision: 60
            passive_perception: 12
            languages:
            - draconic
            understands: caster_languages
            cr: none
            xp: 0
            proficiency_bonus:
              stat:
                of: caster
                key: proficiency_bonus
          shared_resistance:
            choice_at_summon: one_spirit_resistance
            beneficiary: caster
            duration: spell
          actions:
            multiattack:
              rend_count:
                op: floor
                args:
                - op: multiply
                  args:
                  - binding: cast_level
                  - numerator: 1
                    denominator: 2
              also_breath_weapon: true
            rend:
              kind: melee_attack
              attack_bonus:
                binding: spell_attack_modifier
              reach_feet: 10
              damage:
                amount:
                  dice:
                    count: 1
                    sides: 6
                    add:
                      op: add
                      args:
                      - 4
                      - binding: cast_level
                type: piercing
            breath_weapon:
              area:
                shape: cone
                length_feet: 30
              damage_type_choice_at_cast: one_spirit_resistance
              save:
                ability: dexterity
                dc:
                  binding: save_dc
                failure:
                  damage:
                    amount:
                      dice:
                        count: 2
                        sides: 6
                        add: 0
                    type: chosen
                success:
                  damage_fraction:
                    numerator: 1
                    denominator: 2
    contract:
      id: dnd2024:spell_summon_dragon
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        spell_attack_modifier: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - summoned_spirit
      - shared_resistance
      - commands
      - attacks
      - saves
      - damage
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - attacks
      - conditions
      - spatial_areas
      - movement
      - perception
      - inventory
      - summoning
      - choices
      - time
      - knowledge
      - relationships
    requires:
    - dnd2024:spell_summon_dragon
    citations:
    - SRD 5.2.1 pp. 166; builtin/srd-5.2.1/spell/summon_dragon.md
  dnd2024:spells/telekinesis:
    source_sha256: 47ead84643eccd82bce153f81eb8c0df9f3f04734bbae91c906a99f9c968138b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/telekinesis
        implementation: dnd2024:spell_telekinesis
        parameters:
          target:
            kind: creature_or_object
            count: 1
            visible_to_caster: true
            maximum_size: huge
          use:
            initial: spell.cast_completed
            repeat_action: magic
            repeat_timing: later_caster_turns
            switch_target: ends_prior_target_effect
          creature:
            save:
              ability: strength
              dc:
                binding: save_dc
              failure:
                move_maximum_feet: 30
                direction: any
                destination_within_spell_range: true
                condition: restrained
                until: caster.next_turn.end
                airborne: suspended
              success:
                effect: none
            fall_at: caster.next_turn.end
            renew_suspension_requires:
            - reuse_on_target
            - target_fails_save
          object:
            unworn_uncarried: automatic
            worn_or_carried:
              holder_save:
                ability: strength
                dc:
                  binding: save_dc
                failure:
                  pull_away: true
                  move: true
                success:
                  effect: none
            move_maximum_feet: 30
            direction: any
            destination_within_spell_range: true
            fine_control: true
            simple_tool_manipulation: true
    contract:
      id: dnd2024:spell_telekinesis
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - save
      - movement
      - restrained
      - object_manipulation
      events:
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - saving_throws
      - conditions
      - movement
      - inventory
      - time
      - relationships
    requires:
    - dnd2024:spell_telekinesis
    citations:
    - SRD 5.2.1 pp. 167, 168; builtin/srd-5.2.1/spell/telekinesis.md
    - SRD 5.2.1 p. 168; builtin/srd-5.2.1/source.pdf; builtin/srd-5.2.1/source.txt lines 10694–10703
  dnd2024:spells/thaumaturgy:
    source_sha256: c938381ef486e9790c827de73ada622e1418212dbabe7b79086cebecf482db6a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/thaumaturgy
        implementation: dnd2024:spell_thaumaturgy
        parameters:
          effect_choice:
            count: 1
            within_range: true
          maximum_simultaneous_minute_effects: 3
          effects:
            altered_eyes:
              appearance_choice: caster
              minutes: 1
            booming_voice:
              volume_multiplier_maximum: 3
              minutes: 1
              charisma_intimidation_checks: advantage
            fire_play:
              choice:
              - flicker
              - brighten
              - dim
              - change_color
              minutes: 1
            invisible_hand:
              target:
              - door
              - window
              unlocked_required: true
              choice:
              - open
              - slam_shut
              duration: instantaneous
            phantom_sound:
              sound_choice: caster
              origin: chosen_point_in_range
              duration: instantaneous
            tremors:
              target: ground
              harmless: true
              minutes: 1
    contract:
      id: dnd2024:spell_thaumaturgy
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - minor_effect
      - active_effect_count
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - perception
      - choices
      - time
      - checks
    requires:
    - dnd2024:spell_thaumaturgy
    citations:
    - SRD 5.2.1 pp. 169; builtin/srd-5.2.1/spell/thaumaturgy.md
  dnd2024:spells/tongues:
    source_sha256: 4c2cbca5e3b4543704d8a89b038d917e51e2f365e69d215a65a910c31441f8f4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/tongues
        implementation: dnd2024:spell_tongues
        parameters:
          target:
            kind: creature
            count: 1
            visible_to_caster: false
            touch: true
          understand:
            languages: any
            spoken_if: heard
            signed_if: seen
          be_understood:
            listener_knows_minimum_languages: 1
            speech_if: heard
            signing_if: seen
    contract:
      id: dnd2024:spell_tongues
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - language_comprehension_effect
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - knowledge
    requires:
    - dnd2024:spell_tongues
    citations:
    - SRD 5.2.1 pp. 169, 170; builtin/srd-5.2.1/spell/tongues.md
  dnd2024:spells/true_resurrection:
    source_sha256: 2626ee515044b0af367b52d166e5b784af49fb48c16051547201b82e797b2957
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/true_resurrection
        implementation: dnd2024:spell_true_resurrection
        parameters:
          target:
            dead_maximum_years: 200
            death_by_old_age: false
            touch_required_if_body_exists: true
          revive:
            hp: full
            close_all_wounds: true
            neutralize_poison: true
            cure_all_magical_contagions: true
            lift_curses_at_death: true
            replace_damaged_missing:
            - organs
            - limbs
            if_undead: restore_nonundead_form
          missing_body:
            create_new_body: true
            speak_name_required: true
            appearance_space: caster_chosen_unoccupied
            within_caster_feet: 10
    contract:
      id: dnd2024:spell_true_resurrection
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - revived_creature
      - restored_body
      - removed_afflictions
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - healing
      - forms
      - choices
    requires:
    - dnd2024:spell_true_resurrection
    citations:
    - SRD 5.2.1 pp. 170, 171; builtin/srd-5.2.1/spell/true_resurrection.md
  dnd2024:spells/unseen_servant:
    source_sha256: be9383f9c85c3f225340323a71b5a486ce7e351af68be33ded80331fa9f2daba
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/unseen_servant
        implementation: dnd2024:spell_unseen_servant
        parameters:
          force:
            size: medium
            invisible: true
            mindless: true
            shapeless: true
            initial_space: unoccupied_ground_in_range
            ac: 10
            hp: 1
            strength: 2
            can_attack: false
          end:
            hp_zero: true
            command_requires_move_beyond_caster_feet: 60
          command:
            cost: bonus_action
            maximum_per_caster_turn: 1
            mental: true
            move_maximum_feet: 15
            object_interaction: true
            task_scope: simple_human_tasks
            examples:
            - fetch
            - clean
            - mend
            - fold_clothes
            - light_fire
            - serve_food
            - pour_drink
            continue: best_ability_until_complete
            after_complete: wait_for_command
    contract:
      id: dnd2024:spell_unseen_servant
      inputs:
        caster: participant
        cast_level: integer
        spell_attack_modifier:
          type: integer
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - servant
      - commands
      - object_interactions
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - attacks
      - movement
      - perception
      - inventory
      - time
      - relationships
    requires:
    - dnd2024:spell_unseen_servant
    citations:
    - SRD 5.2.1 pp. 171; builtin/srd-5.2.1/spell/unseen_servant.md
  dnd2024:spells/wall_of_fire:
    source_sha256: 779875d416c463fb33a32ae379bc2667b11a1cc9d8a1ef18449e7380dbbfb143
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/wall_of_fire
        implementation: dnd2024:spell_wall_of_fire
        parameters:
          surface: solid
          opaque: true
          geometry_choice:
            wall:
              maximum_length_feet: 60
              maximum_height_feet: 20
              thickness_feet: 1
            ring:
              maximum_diameter_feet: 20
              maximum_height_feet: 20
              thickness_feet: 1
          appearance_effect:
            event: spell.cast_completed
            targets: creatures_in_wall
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  amount:
                    dice:
                      count: 5
                      sides: 8
                      add: 0
                  type: fire
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
          hot_side:
            chooser: caster
            chosen_at: cast
            damage:
              amount:
                dice:
                  count: 5
                  sides: 8
                  add: 0
              type: fire
            event: turn.end
            eligible:
            - within_10_feet_of_hot_side
            - inside_wall
          wall_damage:
            events:
            - area.first_entry_on_turn
            - turn.end
            inside_wall: true
            damage:
              amount:
                dice:
                  count: 5
                  sides: 8
                  add: 0
              type: fire
            save: false
            same_end_turn_occurrence_not_duplicated: true
          cold_side_damage: 0
          scaling:
            per_slot_above: 4
            damage_dice: 1
            sides: 8
    contract:
      id: dnd2024:spell_wall_of_fire
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - wall_effect
      - saves
      - damage
      events:
      - spell.cast_completed
      - turn.end
      - area.enter
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - spatial_areas
      - choices
      - time
    requires:
    - dnd2024:spell_wall_of_fire
    citations:
    - SRD 5.2.1 pp. 172; builtin/srd-5.2.1/spell/wall_of_fire.md
  dnd2024:spells/wall_of_thorns:
    source_sha256: 7e5a240c6e2b61e76e272d32e7f50de41e6e51b044dd7c6f97f6caf6c2f4af0c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/wall_of_thorns
        implementation: dnd2024:spell_wall_of_thorns
        parameters:
          surface: solid
          blocks_line_of_sight: true
          geometry_choice:
            wall:
              maximum_length_feet: 60
              maximum_height_feet: 10
              maximum_thickness_feet: 5
            circle:
              diameter_feet: 20
              maximum_height_feet: 20
              maximum_thickness_feet: 5
          appearance_effect:
            event: spell.cast_completed
            targets: creatures_in_wall
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  amount:
                    dice:
                      count: 7
                      sides: 8
                      add: 0
                  type: piercing
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
          movement:
            per_foot_cost_feet: 4
          later_effect:
            triggers:
            - area.first_entry_on_turn
            - turn.end
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  amount:
                    dice:
                      count: 7
                      sides: 8
                      add: 0
                  type: slashing
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
            maximum_saves_per_creature_per_turn: 1
          scaling:
            per_slot_above: 6
            piercing_dice: 1
            slashing_dice: 1
            sides: 8
    contract:
      id: dnd2024:spell_wall_of_thorns
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - wall_effect
      - saves
      - damage
      - movement_cost
      events:
      - spell.cast_completed
      - turn.end
      - area.enter
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - spatial_areas
      - movement
      - choices
      - time
    requires:
    - dnd2024:spell_wall_of_thorns
    citations:
    - SRD 5.2.1 pp. 173; builtin/srd-5.2.1/spell/wall_of_thorns.md
  dnd2024:spells/web:
    source_sha256: 0d38279be42915b7f7d797dd9654718058bca826e8f692031900047b8427e4b0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/web
        implementation: dnd2024:spell_web
        parameters:
          area:
            shape: cube
            side_feet: 20
            difficult_terrain: true
            lightly_obscured: true
          support:
            valid:
            - anchored_between_two_solid_masses
            - layered_floor_wall_or_ceiling
            unsupported:
              collapse: true
              end_at: caster.next_turn.start
            flat_surface_depth_feet: 5
          triggers:
          - area.first_entry_on_turn
          - turn.start
          save:
            ability: dexterity
            dc:
              binding: save_dc
            failure:
              condition: restrained
              until_any:
              - leaves_webs
              - breaks_free
            success:
              effect: none
          escape:
            action: action
            ability: strength
            skill: athletics
            dc:
              binding: save_dc
            success: end_restrained
          fire:
            cube_side_feet: 5
            burn_away_rounds: 1
            damage_event: turn.start
            target: creature_in_burning_webs
            damage:
              amount:
                dice:
                  count: 2
                  sides: 4
                  add: 0
              type: fire
    contract:
      id: dnd2024:spell_web
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
      - web_area
      - saves
      - restrained
      - escape_check
      - burning_webs
      - damage
      events:
      - spell.cast_completed
      - turn.start
      - area.enter
      capabilities:
      - spellcasting
      - effects
      - damage
      - saving_throws
      - conditions
      - spatial_areas
      - perception
      - time
      - checks
    requires:
    - dnd2024:spell_web
    citations:
    - SRD 5.2.1 pp. 174; builtin/srd-5.2.1/spell/web.md
  dnd2024:spells/wish:
    source_sha256: 7b25c16f59313843562eec92395aceb8e8e1704cdc03b9f278b9868039f450b6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/wish
        implementation: dnd2024:spell_wish
        parameters:
          choice:
            chooser: caster
            options:
            - duplicate_spell
            - object_creation
            - instant_health
            - resistance
            - spell_immunity
            - sudden_learning
            - roll_redo
            - reshape_reality
          effects:
            duplicate_spell:
              maximum_spell_level: 8
              other_spell_only: true
              casting_requirements_waived: all_including_costly_components
              spell_takes_effect: true
              stress: false
            object_creation:
              count: 1
              maximum_gp_value: 25000
              magic_item: false
              maximum_each_dimension_feet: 300
              destination: visible_unoccupied_ground
            instant_health:
              caster: true
              additional_visible_creatures_maximum: 20
              heal: full_hp
              end_all_effects:
              - exhaustion
              - charmed
              - petrified
              - curses
              - cursed_item_attunement
              - ability_score_reductions
              - hp_maximum_reductions
            resistance:
              maximum_visible_creatures: 10
              damage_type_choice_count: 1
              duration: permanent
            spell_immunity:
              maximum_visible_creatures: 10
              single_choice:
              - spell
              - magical_effect
              duration_hours: 8
            sudden_learning:
              replace_own_feat_count: 1
              new_feat_eligibility_required: true
              old_feat_cannot_be_prerequisite_for:
              - other_feat
              - feature
              remove_old_benefits: true
              grant_new_benefits: true
            roll_redo:
              roll_count: 1
              lookback: last_round_including_caster_last_turn
              reroll: true
              roll_mode_choice:
              - normal
              - advantage
              - disadvantage
              result_choice:
              - original
              - reroll
              reshape_prior_consequences: true
            reshape_reality:
              request: caster_precise_statement
              adjudicator: gm
              resolution_choice:
              - success
              - partial_success
              - failure
              - unforeseen_consequence
              risk_increases_with_wish_magnitude: true
              possible_consequences:
              - time_travel
              - transport_to_item_owner
              - powerful_foes_for_community_region_world_effects
              - divine_servants_intervention_for_god_effects
              undo_multiverse: automatic_failure
          stress:
            applies_to: all_modes_except_duplicate_spell
            spell_cast_damage:
              until: caster.long_rest.completed
              event: spell.cast_completed
              amount:
                dice:
                  count:
                    binding: cast_spell_level
                  sides: 10
                  add: 0
              type: necrotic
              reducible: false
              preventable: false
              cantrip_damage: 0
            strength:
              set: 3
              duration_days:
                dice:
                  count: 2
                  sides: 4
                  add: 0
              rest_light_activity_day_recovery_reduction_days: 2
            permanent_loss_of_wish_percent: 33
    contract:
      id: dnd2024:spell_wish
      inputs:
        caster: participant
        cast_level: integer
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
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
        cast_spell_level: integer
      outputs:
      - duplicated_spell
      - chosen_effect
      - revised_roll
      - feat_replacement
      - reality_resolution
      - stress_effect
      - wish_casting_loss
      events:
      - spell.cast_completed
      - rest.completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - healing
      - conditions
      - movement
      - perception
      - inventory
      - summoning
      - choices
      - time
      - relationships
    requires:
    - dnd2024:spell_wish
    citations:
    - SRD 5.2.1 pp. 175; builtin/srd-5.2.1/spell/wish.md
    - SRD 5.2.1 p. 137; builtin/srd-5.2.1/spell/greater_restoration.md
---

# SRD spell contracts: assignment spells_1

All 85 assigned spell definitions have source-specific typed parameters, contract boundaries, required capability families, and source citations. All 85 are specified; zero are informational. These are definition contracts. No engine implementation is included. The imported spell headers remain authoritative for level, class access, casting time, range, components, cost, consumption, base duration, and concentration. Parameters add every stated effect, target restriction, option, exception, and scaling rule.

Each contract is scoped to its named spell. Its enum values identify that spell's typed branches. Dice records contain count, sides, and add; scaling records add the stated amount for each slot above the given base. Half damage rounds down under the core damage rules. Character-level tables use the highest reached threshold. Area membership, visibility, paths, willingness, and other world facts must come from explicit resolution inputs; unavailable facts cannot silently pass. User and GM choices remain typed choices subject to the stated source constraints. Contract input fields marked optional apply only to the branches that use them. Standard event families identify the resolution boundaries; source-specific timing constraints are in the parameters.

The 2024 text matters: Mirror Image redirects a hit with d6 rolls; Barkskin sets an AC floor of 17; Guidance applies to all checks with one chosen skill; Command uses its five listed commands; Shapechange retains the caster's HP and grants initial-form temporary HP; several moving areas trigger on entry or a creature's turn end. Wish includes Sudden Learning and the published stress rules. No older-edition clauses are added.

Telekinesis has one source defect: its final example ends after “manipulating a simple tool,” in the named Markdown, source.txt, and official PDF page 168. The complete preceding authorization for fine control and the stated simple-tool example are specified. No missing example is inferred. This is an unresolved source truncation, not an omitted known rule.

Wall of Fire's two references to ending a turn inside the wall describe the same trigger and produce one damage application for that occurrence. Wish's Instant Health removes all active effects in the Greater Restoration categories, including Exhaustion; the linked spell supplies those categories. Summon Dragon includes the full embedded Draconic Spirit stat block even though it also has a canonical creature definition.

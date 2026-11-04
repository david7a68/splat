---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:spells/acid_splash:
    source_sha256: 75fefe4265eb9d866c24c8e170e60cfd5eee90495f8430295c70ff563d62d8ec
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/acid_splash
        implementation: dnd2024:spell_acid_splash
        parameters:
          targeting:
            kind: creatures_in_area
            origin: chosen_point_in_range
            shape: sphere
            radius_feet: 5
          save:
            ability: dexterity
            failure:
              damage:
                dice: 1d6
                type: acid
            success:
              damage: 0
          cantrip_scaling:
            character_level_dice:
              1: 1d6
              5: 2d6
              11: 3d6
              17: 4d6
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_acid_splash
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_acid_splash
    citations:
    - SRD 5.2.1 pp. 107; builtin/srd-5.2.1/spell/acid_splash.md
  dnd2024:spells/animal_friendship:
    source_sha256: 499b910ae342956b203528cce950eacb54a16da40f13dd8983f792048fb93f2b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/animal_friendship
        implementation: dnd2024:spell_animal_friendship
        parameters:
          targeting:
            kind: creature
            creature_types:
            - beast
            visible: true
            count:
              base: 1
              per_slot_above: 1
              slot_base: 1
          save:
            ability: wisdom
            failure:
              condition: charmed
              source: caster
              duration: spell_duration
          termination:
            event: damage.dealt
            dealer: caster_or_caster_ally
            recipient: target
            scope: spell
          spell_lifetime:
            concentration: false
            maximum:
              value: 24
              unit: hour
    contract:
      id: dnd2024:spell_animal_friendship
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - damage.applied
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
    requires:
    - dnd2024:spell_animal_friendship
    citations:
    - SRD 5.2.1 pp. 107, 108; builtin/srd-5.2.1/spell/animal_friendship.md
  dnd2024:spells/animate_objects:
    source_sha256: b33ecaa40ab7d114da226bf7b6b3aed9293b4542fef74ee88eaf88fea6d66c1a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/animate_objects
        implementation: dnd2024:spell_animate_objects
        parameters:
          targeting:
            kind: objects
            magical: false
            worn: false
            carried: false
            fixed_to_surface: false
            excluded_sizes:
            - gargantuan
            selection_budget:
              stat: spellcasting_ability_modifier
            costs:
              tiny: 1
              small: 1
              medium: 1
              large: 2
              huge: 3
          animation:
            creature_type: construct
            alignment: unaligned
            sprouts_legs: true
            ally_of:
            - caster
            - caster_allies
            duration:
              until_any:
              - spell_ends
              - zero_hit_points
            initiative:
              same_count_as: caster
              turn_order: immediately_after_caster
            control:
              economy: bonus_action
              channel: mental
              range_feet: 500
              targets: any_subset
              shared_command: true
              no_command:
                action: dodge
                movement: avoid_harm_only
            reversion_at_zero_hp:
              form: original_object
              excess_damage: carries_over
            stats:
              ac: 15
              hp_by_size:
                tiny: 10
                small: 10
                medium: 10
                large: 20
                huge: 40
              speed_feet: 30
              abilities:
                strength: 16
                dexterity: 10
                constitution: 10
                intelligence: 3
                wisdom: 3
                charisma: 1
              saving_throw_modifiers:
                strength: 3
                dexterity: 0
                constitution: 0
                intelligence: -4
                wisdom: -4
                charisma: -5
              damage_immunities:
              - poison
              - psychic
              condition_immunities:
              - charmed
              - exhaustion
              - frightened
              - paralyzed
              - poisoned
              blindsight_feet: 30
              passive_perception: 6
              understands_languages: caster_known
              challenge_rating: null
              xp: 0
              proficiency_bonus:
                stat: caster_proficiency_bonus
            slam:
              economy: action
              attack:
                kind: melee
                modifier: caster_spell_attack_modifier
                reach_feet: 5
              damage_type: force
              damage_by_size:
                medium_or_smaller:
                  dice: 1d4
                  add: 3
                  extra_dice_per_slot_above_5: 1d4
                large:
                  dice: 2d6
                  add:
                    op: add
                    args:
                    - 3
                    - stat: caster_spellcasting_ability_modifier
                  extra_dice_per_slot_above_5: 1d6
                huge:
                  dice: 2d12
                  add:
                    op: add
                    args:
                    - 3
                    - stat: caster_spellcasting_ability_modifier
                  extra_dice_per_slot_above_5: 1d12
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_animate_objects
      inputs:
        caster: participant
        cast_level: integer
        locations: location_list
        objects: object_list
        forms: definition_list
      outputs:
      - effects
      - damage_results
      - participants
      - locations
      events:
      - spell.cast_completed
      - health.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
      - conditions
      - spatial_effects
      - participant_lifecycle
      - forms
      - perception
      - controlled_participants
    requires:
    - dnd2024:spell_animate_objects
    citations:
    - SRD 5.2.1 pp. 108, 109; builtin/srd-5.2.1/spell/animate_objects.md
  dnd2024:spells/arcane_eye:
    source_sha256: 7e7852787d3485fe413e2e169bce1e3e5ec88c431e17cc85305f94062512d46e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/arcane_eye
        implementation: dnd2024:spell_arcane_eye
        parameters:
          targeting:
            kind: location
            within_range: true
          sensor:
            invisible: true
            invulnerable: true
            hovers: true
            perception: visual
            transmission: mental_to_caster
            directions: all
            darkvision_feet: 30
            duration: spell_duration
          move:
            economy: bonus_action
            max_feet: 30
            direction: any
            solid_barrier_blocks: true
            minimum_opening_diameter_inches: 1
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_arcane_eye
      inputs:
        caster: participant
        cast_level: integer
        locations: location_list
      outputs:
      - effects
      - locations
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - perception
    requires:
    - dnd2024:spell_arcane_eye
    citations:
    - SRD 5.2.1 pp. 110; builtin/srd-5.2.1/spell/arcane_eye.md
  dnd2024:spells/arcanist_s_magic_aura:
    source_sha256: 7b2ac3e17eed48010eca973a7a9cb43ee89813775951e996f2f37bc58157df18
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/arcanist_s_magic_aura
        implementation: dnd2024:spell_arcanist_s_magic_aura
        parameters:
          targeting:
            options:
            - kind: creature
              willing: true
              mode: mask
            - kind: object
              worn: false
              carried: false
              mode: false_aura
            touch: true
          mask:
            choice: creature_type
            excluded: target_actual_type
            affected_systems:
            - spells
            - magical_effects
            treated_type: chosen_type
          false_aura:
            affected_systems:
            - magic_aura_detection_spells
            - magic_aura_detection_effects
            choices:
            - nonmagical_appears_magical
            - magical_appears_nonmagical
            - chosen_school_aura
          persistence:
            same_target: true
            frequency: daily
            consecutive_days: 30
            resulting_duration: until_dispelled
          spell_lifetime:
            concentration: false
            maximum:
              value: 24
              unit: hour
    contract:
      id: dnd2024:spell_arcanist_s_magic_aura
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects: object_list
        choices: choice
      outputs:
      - effects
      events:
      - spell.cast_completed
      - time.elapsed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_arcanist_s_magic_aura
    citations:
    - SRD 5.2.1 pp. 110, 111; builtin/srd-5.2.1/spell/arcanist_s_magic_aura.md
  dnd2024:spells/awaken:
    source_sha256: 86824ea30bbdac181dd801c28c3ffd2b249ce2bada6389d7dbedf2a53d4514bf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/awaken
        implementation: dnd2024:spell_awaken
        parameters:
          targeting:
            touch: true
            options:
            - kind: creature
              types:
              - beast
              - plant
              intelligence_max: 3
            - kind: natural_plant
              creature: false
          grant:
            intelligence: 10
            speaks_language:
              choice: caster_known_language
              count: 1
          natural_plant:
            becomes_type: plant
            movable_parts:
            - limbs
            - roots
            - vines
            - creepers
            senses: human_equivalent
            statistics:
              chooser: gm
              type: definition
              constraint: appropriate_awakened_plant
          charm:
            condition: charmed
            source: caster
            days: 30
            ends_on_damage_by:
            - caster
            - caster_allies
            after_end_attitude:
              chooser: awakened_creature
              type: choice
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_awaken
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        forms: definition_list
        choices: choice
        campaign: campaign
      outputs:
      - effects
      - damage_results
      - participants
      events:
      - spell.cast_completed
      - damage.applied
      - time.elapsed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
      - participant_lifecycle
      - forms
      - campaign_state
      - perception
    requires:
    - dnd2024:spell_awaken
    citations:
    - SRD 5.2.1 pp. 111, 112; builtin/srd-5.2.1/spell/awaken.md
  dnd2024:spells/beacon_of_hope:
    source_sha256: 69ab2c9f92103a6b2be0495d819afa1f59f1e7ca384495b0a0f0d0acdded0739
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/beacon_of_hope
        implementation: dnd2024:spell_beacon_of_hope
        parameters:
          targeting:
            kind: creatures
            count: any
            within_range: true
          duration: spell_duration
          advantage_on:
          - wisdom_saving_throws
          - death_saving_throws
          healing:
            amount: maximum_possible
            sources: all
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_beacon_of_hope
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      - healing_results
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - healing
    requires:
    - dnd2024:spell_beacon_of_hope
    citations:
    - SRD 5.2.1 pp. 112; builtin/srd-5.2.1/spell/beacon_of_hope.md
  dnd2024:spells/blade_barrier:
    source_sha256: c54da2f2fb622b96e55eb7ee1bbbad60c0ca43a82995768fb7b8b9b105586d52
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/blade_barrier
        implementation: dnd2024:spell_blade_barrier
        parameters:
          targeting:
            kind: area
            within_range: true
          wall:
            choices:
            - shape: straight
              max_length_feet: 100
            - shape: ring
              max_diameter_feet: 60
            height_feet: 20
            thickness_feet: 5
            cover: three_quarters
            difficult_terrain: true
            duration: spell_duration
          damage_triggers:
            events:
            - spell.cast_completed
            - area.entered
            - turn.ended
            scope: creature_in_wall
            max_saves_per_turn: 1
          save:
            ability: dexterity
            failure:
              damage:
                dice: 6d10
                type: force
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          spell_lifetime:
            concentration: true
            maximum:
              value: 10
              unit: minute
    contract:
      id: dnd2024:spell_blade_barrier
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      - turn.ended
      - area.entered
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_blade_barrier
    citations:
    - SRD 5.2.1 pp. 113; builtin/srd-5.2.1/spell/blade_barrier.md
  dnd2024:spells/blink:
    source_sha256: bf9ff34a2a0635c676f0841e3cf1229ebc8979ef46b5386ed94e5dd8f85668bf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/blink
        implementation: dnd2024:spell_blink
        parameters:
          targeting:
            kind: self
          duration: spell_duration
          departure:
            event: caster.turn_ended
            roll: 1d6
            on_results:
            - 4
            - 5
            - 6
            destination_plane: ethereal
            already_ethereal: end_spell_immediately
          ethereal_state:
            perceive_original_plane_feet: 60
            original_plane_color: gray
            interaction_creatures: ethereal_only
            visible_from_original_plane: requires_ethereal_perception
          return:
            events:
            - caster.turn_started
            - spell.ended
            when: on_ethereal_plane
            destination_plane: original
            choose_visible_unoccupied_within_departure_feet: 10
            fallback: nearest_unoccupied_space
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_blink
      inputs:
        caster: participant
        cast_level: integer
      outputs:
      - effects
      events:
      - spell.cast_completed
      - turn.started
      - turn.ended
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - planar_travel
      - perception
    requires:
    - dnd2024:spell_blink
    citations:
    - SRD 5.2.1 pp. 113, 114; builtin/srd-5.2.1/spell/blink.md
  dnd2024:spells/calm_emotions:
    source_sha256: 6a5133f4b28e17e2ceacdd9b02228af6c33457f65a31038091fa9f9f57c430fe
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/calm_emotions
        implementation: dnd2024:spell_calm_emotions
        parameters:
          targeting:
            kind: creatures_in_area
            types:
            - humanoid
            origin: chosen_point_in_range
            shape: sphere
            radius_feet: 20
          save:
            ability: charisma
            failure_choice_per_target:
            - suppress_conditions
            - suppress_hostility
          suppress_conditions:
            immune:
            - charmed
            - frightened
            suppress_existing:
            - charmed
            - frightened
            duration: spell_duration
          suppress_hostility:
            attitude: indifferent
            toward: caster_chosen_previously_hostile_creatures
            ends_on:
            - target_takes_damage
            - target_witnesses_allies_take_damage
            - spell_ends
            restore_attitude: normal
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_calm_emotions
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      - damage.applied
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
      - spatial_effects
    requires:
    - dnd2024:spell_calm_emotions
    citations:
    - SRD 5.2.1 pp. 114; builtin/srd-5.2.1/spell/calm_emotions.md
  dnd2024:spells/chill_touch:
    source_sha256: 7d80daf3de39fb3f076553a90a2dba4b32f78a8e01844adb6b94237cf041fd12
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/chill_touch
        implementation: dnd2024:spell_chill_touch
        parameters:
          targeting:
            kind: target
            within_reach: true
          attack:
            kind: melee_spell
            hit:
              damage:
                dice: 1d10
                type: necrotic
              cannot_regain_hp:
                until: caster_next_turn_end
          cantrip_scaling:
            character_level_dice:
              1: 1d10
              5: 2d10
              11: 3d10
              17: 4d10
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_chill_touch
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
    requires:
    - dnd2024:spell_chill_touch
    citations:
    - SRD 5.2.1 pp. 115; builtin/srd-5.2.1/spell/chill_touch.md
  dnd2024:spells/clone:
    source_sha256: 01bc3841fed18870f7101c898289485fc6bcd345be1fd59547d206bccf9849a0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/clone
        implementation: dnd2024:spell_clone
        parameters:
          targeting:
            touch: true
            options:
            - kind: creature
            - kind: flesh_sample
              volume_min_cubic_inches: 1
          duplicate:
            location: component_vessel
            maturity_days: 120
            age_choice:
            - same_as_original
            - younger
            before_transfer: inert
            preservation:
              duration: indefinite
              requires: vessel_undisturbed
          soul_transfer:
            event: original.died
            requires:
            - clone_mature
            - soul_free
            - soul_willing
            destination: clone
            preserved:
            - physical_identity
            - personality
            - memories
            - abilities
            equipment: none
            original_remains:
              inert: true
              revival_possible: false
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_clone
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects: object_list
        choices: choice
        campaign: campaign
      outputs:
      - effects
      events:
      - spell.cast_completed
      - time.elapsed
      - health.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - campaign_state
    requires:
    - dnd2024:spell_clone
    citations:
    - SRD 5.2.1 pp. 115, 116; builtin/srd-5.2.1/spell/clone.md
  dnd2024:spells/commune:
    source_sha256: 1a8186710783253784a81369569217157738d23a20810f6a1271eb749a671150
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/commune
        implementation: dnd2024:spell_commune
        parameters:
          targeting:
            kind: self
            contact: deity_or_divine_proxy
          questions:
            max: 3
            deadline: spell_end
            form: yes_or_no
            answers:
              correct: true
              beyond_entity_knowledge: unclear
              gm_short_phrase_allowed_when:
              - one_word_misleading
              - one_word_contrary_to_deity_interests
          repeated_casting:
            reset: long_rest_finished
            no_answer_chance_percent:
              base: 0
              additional_per_cast_after_first: 25
              maximum: 100
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_commune
      inputs:
        caster: participant
        cast_level: integer
        choices: choice
        campaign: campaign
        questions: text
      outputs:
      - effects
      - information
      events:
      - spell.cast_completed
      - rest.completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - campaign_state
      - rest_lifecycle
    requires:
    - dnd2024:spell_commune
    citations:
    - SRD 5.2.1 pp. 116; builtin/srd-5.2.1/spell/commune.md
  dnd2024:spells/cone_of_cold:
    source_sha256: aec7a6c7fe774e7c2502d9f355654c549e38e7430c2107cb82a00cddacbca10a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/cone_of_cold
        implementation: dnd2024:spell_cone_of_cold
        parameters:
          targeting:
            kind: creatures_in_area
            shape: cone
            length_feet: 60
            origin: caster
          save:
            ability: constitution
            failure:
              damage:
                dice: 8d8
                type: cold
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          killed_target:
            form: frozen_statue
            until: thawed
          slot_scaling:
            above: 5
            damage_per_level: 1d8
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_cone_of_cold
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_cone_of_cold
    citations:
    - SRD 5.2.1 pp. 117; builtin/srd-5.2.1/spell/cone_of_cold.md
  dnd2024:spells/conjure_elemental:
    source_sha256: cc2cda5a4688b8d2642ad97c7f206b4e3eb4850bc6573d29064c86dd0e349b42
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/conjure_elemental
        implementation: dnd2024:spell_conjure_elemental
        parameters:
          targeting:
            kind: location
            within_range: true
            unoccupied: true
          spirit:
            size: large
            intangible: true
            duration: spell_duration
            element_damage_choice:
              air: lightning
              earth: thunder
              fire: fire
              water: cold
          capture:
            events:
            - creature.enters_spirit_space
            - creature.starts_turn_within_5_feet
            target_visible_to_caster: true
            optional: true
            requires: spirit_has_no_restrained_creature
            save:
              ability: dexterity
              failure:
                damage:
                  dice: 8d8
                  type: chosen_element_damage
                condition: restrained
                until: spell_ends_or_repeat_save_success
          restrained_repeat:
            event: target.turn_started
            save:
              ability: dexterity
              failure:
                damage:
                  dice: 4d8
                  type: chosen_element_damage
              success:
                remove: spirit_restrained
          slot_scaling:
            above: 5
            capture_damage_per_level: 1d8
            repeat_damage_per_level: 1d8
          spell_lifetime:
            concentration: true
            maximum:
              value: 10
              unit: minute
    contract:
      id: dnd2024:spell_conjure_elemental
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      - turn.started
      - area.entered
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
      - spatial_effects
    requires:
    - dnd2024:spell_conjure_elemental
    citations:
    - SRD 5.2.1 pp. 118; builtin/srd-5.2.1/spell/conjure_elemental.md
  dnd2024:spells/contact_other_plane:
    source_sha256: 71c4f76368d2aebe17713c86c3a9539241d5b9e10931233e21d9d0e75d8da5c1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/contact_other_plane
        implementation: dnd2024:spell_contact_other_plane
        parameters:
          targeting:
            kind: self
            contact: knowledgeable_extraplanar_entity
          save:
            ability: intelligence
            dc: 15
            success:
              questions_max: 5
              deadline: spell_end
              responder: gm
              answer_words: 1
              unknown_answer: unclear
              misleading_one_word: short_phrase_allowed
            failure:
              damage:
                dice: 6d6
                type: psychic
              condition: incapacitated
              ends_on:
              - long_rest_finished
              - greater_restoration_applied
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_contact_other_plane
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        choices: choice
        campaign: campaign
        questions: text
      outputs:
      - effects
      - damage_results
      - information
      events:
      - spell.cast_completed
      - rest.completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
      - campaign_state
      - rest_lifecycle
    requires:
    - dnd2024:spell_contact_other_plane
    citations:
    - SRD 5.2.1 pp. 119; builtin/srd-5.2.1/spell/contact_other_plane.md
  dnd2024:spells/control_water:
    source_sha256: 8069ed2446c079f375bf7415e2f7728a9cdfc46dafe970dbb134c3092baadeb7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/control_water
        implementation: dnd2024:spell_control_water
        parameters:
          targeting:
            kind: water_in_area
            shape: cube
            max_side_feet: 100
            within_range: true
          duration: spell_duration
          effect_choice:
          - flood
          - part_water
          - redirect_flow
          - whirlpool
          change_or_repeat:
            economy: magic_action
            timing: later_caster_turns
          flood:
            standing_water_rise_max_feet: 20
            large_body_alternative:
              wave_height_feet: 20
              travels: across_area
              vehicle_size_max: huge
              vehicles:
                carried_to: opposite_side
                capsize_chance_percent: 25
              repeat: caster_next_turn_start_while_flood
            ends_on:
            - spell_ends
            - different_effect_chosen
          part_water:
            trench: across_area
            walls: both_sides
            ends_on:
            - spell_ends
            - different_effect_chosen
            refill_duration_rounds: 1
          redirect_flow:
            direction: caster_choice
            allowed:
            - over_obstacles
            - up_walls
            - other_directions
            outside_area: terrain_normal_flow
            ends_on:
            - spell_ends
            - different_effect_chosen
          whirlpool:
            minimum_water_area_feet:
            - 50
            - 50
            minimum_water_depth_feet: 25
            geometry:
              base_width_feet: 5
              top_width_max_feet: 50
              height_feet: 25
              location: area_center
            pull:
              targets: creatures_in_water
              within_feet: 25
              distance_feet: 10
              toward: whirlpool
            damage:
              events:
              - first_entry_each_turn
              - target_turn_end_inside
              save_ability: strength
              failure:
                dice: 2d8
                type: bludgeoning
              success_fraction:
                numerator: 1
                denominator: 2
            swim_away:
              prerequisite:
                economy: action
                check:
                  ability: strength
                  skill: athletics
                  dc: spell_save_dc
                  success_required: true
            ends_on:
            - spell_ends
            - different_effect_chosen
          spell_lifetime:
            concentration: true
            maximum:
              value: 10
              unit: minute
    contract:
      id: dnd2024:spell_control_water
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        save_dc: integer
        choices: choice
        campaign: campaign
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      - turn.started
      - turn.ended
      - area.entered
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
      - campaign_state
    requires:
    - dnd2024:spell_control_water
    citations:
    - SRD 5.2.1 pp. 119, 120; builtin/srd-5.2.1/spell/control_water.md
  dnd2024:spells/create_or_destroy_water:
    source_sha256: ace59e867ae82c32c2f5096da98a7e73e01b9ed3de55e5d6e750994faec9ab94
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/create_or_destroy_water
        implementation: dnd2024:spell_create_or_destroy_water
        parameters:
          targeting:
            within_range: true
          choice:
          - create_container_water
          - rain
          - destroy_container_water
          - destroy_fog
          create_container_water:
            container: open
            clean: true
            max_gallons: 10
          rain:
            shape: cube
            side_feet: 30
            extinguishes: exposed_flames
          destroy_container_water:
            container: open
            max_gallons: 10
          destroy_fog:
            shape: cube
            side_feet: 30
          slot_scaling:
            above: 1
            gallons_per_level: 10
            cube_side_feet_per_level: 5
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_create_or_destroy_water
      inputs:
        caster: participant
        cast_level: integer
        locations: location_list
        objects: object_list
        choices: choice
      outputs:
      - effects
      - locations
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
    requires:
    - dnd2024:spell_create_or_destroy_water
    citations:
    - SRD 5.2.1 pp. 121; builtin/srd-5.2.1/spell/create_or_destroy_water.md
  dnd2024:spells/dancing_lights:
    source_sha256: 29a6f0e61580254b7157d9f97514f143c3196582aaab97b19821fa97ca9a512c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/dancing_lights
        implementation: dnd2024:spell_dancing_lights
        parameters:
          targeting:
            kind: locations
            within_range: true
          lights:
            max_count: 4
            size: torch
            appearance_choice:
            - torch
            - lantern
            - glowing_orb
            hover: true
            alternative_combined_form:
              count: 1
              size: medium
              shape: vaguely_humanlike
            illumination_each:
              kind: dim
              radius_feet: 10
            duration: spell_duration
          move:
            economy: bonus_action
            max_feet: 60
            destination_within_range: true
          constraints:
            max_distance_to_another_light_feet: 20
            beyond_spell_range: light_vanishes
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_dancing_lights
      inputs:
        caster: participant
        cast_level: integer
        locations: location_list
        choices: choice
      outputs:
      - effects
      - locations
      events:
      - spell.cast_completed
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
    requires:
    - dnd2024:spell_dancing_lights
    citations:
    - SRD 5.2.1 pp. 121, 122; builtin/srd-5.2.1/spell/dancing_lights.md
  dnd2024:spells/death_ward:
    source_sha256: 846694248f9219ea054404528d530d9aa97a6d3fd540b7658cdcec9541febc2b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/death_ward
        implementation: dnd2024:spell_death_ward
        parameters:
          targeting:
            kind: creature
            touch: true
            count: 1
          replacements:
          - trigger: first_hp_drop_to_zero
            set_hp: 1
            end_spell: true
          - trigger: instant_death_without_damage
            negate_for_target: true
            end_spell: true
          duration: spell_duration
          spell_lifetime:
            concentration: false
            maximum:
              value: 8
              unit: hour
    contract:
      id: dnd2024:spell_death_ward
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - damage.applied
      - health.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
    requires:
    - dnd2024:spell_death_ward
    citations:
    - SRD 5.2.1 pp. 122; builtin/srd-5.2.1/spell/death_ward.md
  dnd2024:spells/detect_magic:
    source_sha256: 71f9fc1b23a8210fb01e140a036e6f991f58d6e649dc110a29196d02ace3e9b1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/detect_magic
        implementation: dnd2024:spell_detect_magic
        parameters:
          targeting:
            kind: self
          sense:
            radius_feet: 30
            subject: magical_effect_presence
            duration: spell_duration
          reveal:
            requires: magic_sensed
            economy: magic_action
            aura_on: visible_creatures_and_objects_bearing_magic
            spell_created_effect: reveal_school
          blocked_by:
          - material: stone
            thickness_inches: 12
          - material: dirt
            thickness_inches: 12
          - material: wood
            thickness_inches: 12
          - material: metal
            thickness_inches: 1
          - material: lead
            thickness: thin_sheet
          spell_lifetime:
            concentration: true
            maximum:
              value: 10
              unit: minute
    contract:
      id: dnd2024:spell_detect_magic
      inputs:
        caster: participant
        cast_level: integer
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_detect_magic
    citations:
    - SRD 5.2.1 pp. 123; builtin/srd-5.2.1/spell/detect_magic.md
  dnd2024:spells/disguise_self:
    source_sha256: 91d088306dccbb962e7ad223c42ce7663a9178ce12df2fe702aedc4afad53484
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/disguise_self
        implementation: dnd2024:spell_disguise_self
        parameters:
          targeting:
            kind: self
            includes:
            - clothing
            - armor
            - weapons
            - carried_belongings
          illusion:
            duration: spell_duration
            height_change_max_feet: 1
            weight_appearance:
            - heavier
            - lighter
            limb_arrangement: same_basic
            remaining_appearance: caster_choice
            physical_inspection: fails
            physical_substance: false
          discern:
            economy: study_action
            check:
              ability: intelligence
              skill: investigation
              dc: spell_save_dc
            success: disguise_discerned
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_disguise_self
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        choices: choice
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - perception
    requires:
    - dnd2024:spell_disguise_self
    citations:
    - SRD 5.2.1 pp. 124; builtin/srd-5.2.1/spell/disguise_self.md
  dnd2024:spells/dissonant_whispers:
    source_sha256: 7e5a170842e31ad52bf6c89e7372965693c5181ba43d2fc396fc891b8cec8a42
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/dissonant_whispers
        implementation: dnd2024:spell_dissonant_whispers
        parameters:
          targeting:
            kind: creature
            count: 1
            visible: true
            within_range: true
          save:
            ability: wisdom
            failure:
              damage:
                dice: 3d6
                type: psychic
              movement:
                economy: reaction_if_available
                timing: immediate
                direction: away_from_caster
                distance: maximum_possible
                route: safest
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
              movement: false
          slot_scaling:
            above: 1
            damage_per_level: 1d6
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_dissonant_whispers
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
    requires:
    - dnd2024:spell_dissonant_whispers
    citations:
    - SRD 5.2.1 pp. 124; builtin/srd-5.2.1/spell/dissonant_whispers.md
  dnd2024:spells/divine_word:
    source_sha256: d466b8980e6c1505a5c71340ca711576dd68823c17e7f5e9eed90a3f13b54f1c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/divine_word
        implementation: dnd2024:spell_divine_word
        parameters:
          targeting:
            kind: creatures
            chosen: true
            within_range: true
          save:
            ability: charisma
            success: no_effect
          failure_hp_branches:
          - hp_min: 0
            hp_max: 20
            effect: death
          - hp_min: 21
            hp_max: 30
            conditions:
            - blinded
            - deafened
            - stunned
            duration_minutes: 60
          - hp_min: 31
            hp_max: 40
            conditions:
            - blinded
            - deafened
            duration_minutes: 10
          - hp_min: 41
            hp_max: 50
            conditions:
            - deafened
            duration_minutes: 1
          additional_failure:
            types:
            - celestial
            - elemental
            - fey
            - fiend
            hp_independent: true
            destination: origin_plane
            only_if_elsewhere: true
            return_to_departed_plane_forbidden_hours: 24
            bypass: wish
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_divine_word
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - conditions
      - planar_travel
    requires:
    - dnd2024:spell_divine_word
    citations:
    - SRD 5.2.1 pp. 125; builtin/srd-5.2.1/spell/divine_word.md
  dnd2024:spells/dragon_s_breath:
    source_sha256: aa009f9836bd6e354e9c95f8c9c5c88e8949abf034950d6c95bface69f7ab975
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/dragon_s_breath
        implementation: dnd2024:spell_dragon_s_breath
        parameters:
          targeting:
            kind: creature
            count: 1
            willing: true
            touch: true
          damage_choice:
          - acid
          - cold
          - fire
          - lightning
          - poison
          granted_action:
            duration: spell_duration
            economy: magic_action
            area:
              shape: cone
              length_feet: 15
              origin: target
            targets: all_creatures_in_area
            save:
              ability: dexterity
              dc: caster_spell_save_dc
              failure:
                damage:
                  dice: 3d6
                  type: chosen_type
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
          slot_scaling:
            above: 2
            damage_per_level: 1d6
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_dragon_s_breath
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_dragon_s_breath
    citations:
    - SRD 5.2.1 pp. 126; builtin/srd-5.2.1/spell/dragon_s_breath.md
  dnd2024:spells/eldritch_blast:
    source_sha256: 0e6b8d66d16eb5a3d1b2fad9924f98e691724d0e807bab567eb1a3514c08011c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/eldritch_blast
        implementation: dnd2024:spell_eldritch_blast
        parameters:
          targeting:
            kind: creature_or_object
            within_range: true
            per_beam: true
            same_or_different: true
          beams:
            character_level_count:
              1: 1
              5: 2
              11: 3
              17: 4
            independent_attacks: true
          attack:
            kind: ranged_spell
            hit:
              damage:
                dice: 1d10
                type: force
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_eldritch_blast
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
    requires:
    - dnd2024:spell_eldritch_blast
    citations:
    - SRD 5.2.1 pp. 127; builtin/srd-5.2.1/spell/eldritch_blast.md
  dnd2024:spells/ensnaring_strike:
    source_sha256: c4263996c41f342fbf37e2f720a7039455de1593709eb59efc5ca9a0113ebdf6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/ensnaring_strike
        implementation: dnd2024:spell_ensnaring_strike
        parameters:
          targeting:
            kind: creature_hit_by_triggering_weapon_attack
          save:
            ability: strength
            advantage_if_size_at_least: large
            failure:
              condition: restrained
              until: spell_ends
            success:
              end_spell: true
          restrained_damage:
            event: target.turn_started
            damage:
              dice: 1d6
              type: piercing
          escape:
            actor: target_or_creature_within_reach
            economy: action
            check:
              ability: strength
              skill: athletics
              dc: spell_save_dc
            success: end_spell
          slot_scaling:
            above: 1
            damage_per_level: 1d6
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_ensnaring_strike
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - turn.started
      - attack.hit
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
    requires:
    - dnd2024:spell_ensnaring_strike
    citations:
    - SRD 5.2.1 pp. 128; builtin/srd-5.2.1/spell/ensnaring_strike.md
  dnd2024:spells/expeditious_retreat:
    source_sha256: 9171c5413596fd56cf6a247b566a52c20bc889b63cd5c7d5500313ff19654847
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/expeditious_retreat
        implementation: dnd2024:spell_expeditious_retreat
        parameters:
          targeting:
            kind: self
          on_cast:
            action: dash
            included_in_casting_economy: true
          grant:
            action: dash
            economy: bonus_action
            duration: spell_duration
          spell_lifetime:
            concentration: true
            maximum:
              value: 10
              unit: minute
    contract:
      id: dnd2024:spell_expeditious_retreat
      inputs:
        caster: participant
        cast_level: integer
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_expeditious_retreat
    citations:
    - SRD 5.2.1 pp. 129; builtin/srd-5.2.1/spell/expeditious_retreat.md
  dnd2024:spells/faithful_hound:
    source_sha256: 1a14da1e4eaa38821fac92509b682239f16d6bf18b7010e9ca7dbfb0a026eaaf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/faithful_hound
        implementation: dnd2024:spell_faithful_hound
        parameters:
          targeting:
            kind: location
            unoccupied: true
            visible: true
            within_range: true
          hound:
            visible_to: caster_only
            intangible: true
            invulnerable: true
            truesight_feet: 30
            until_any:
            - spell_duration
            - caster_distance_exceeds_300_feet
          alarm:
            creature_size_min: small
            proximity_feet: 30
            exemption: previously_spoken_caster_specified_password
            effect: loud_barking
          bite:
            event: caster.turn_started
            target: one_enemy
            reach_feet: 5
            save:
              ability: dexterity
              failure:
                damage:
                  dice: 4d8
                  type: force
              success: no_effect
          move:
            economy: magic_action
            timing: later_caster_turns
            max_feet: 30
          spell_lifetime:
            concentration: false
            maximum:
              value: 8
              unit: hour
    contract:
      id: dnd2024:spell_faithful_hound
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        save_dc: integer
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      - turn.started
      - area.entered
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_faithful_hound
    citations:
    - SRD 5.2.1 pp. 129; builtin/srd-5.2.1/spell/faithful_hound.md
  dnd2024:spells/find_familiar:
    source_sha256: 23411e5a769dbdcb2ae857c6d6f6932c5a5c91d21edb3f46f6800a49063cab29
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/find_familiar
        implementation: dnd2024:spell_find_familiar
        parameters:
          targeting:
            kind: location
            unoccupied: true
            within_range: true
          form:
            chooser: caster
            eligible:
              type: beast
              challenge_rating: 0
            examples:
            - bat
            - cat
            - frog
            - hawk
            - lizard
            - octopus
            - owl
            - rat
            - raven
            - spider
            - weasel
            statistics: chosen_form
            replace_type_choice:
            - celestial
            - fey
            - fiend
          behavior:
            independent: true
            obeys_commands: true
            allies:
            - caster
            - caster_allies
            initiative: own_roll
            turn: own
            attack_allowed: false
            other_actions: normal
          connection:
            telepathy_range_feet: 100
            shared_senses:
              economy: bonus_action
              duration: until_caster_next_turn_start
              channels:
              - vision
              - hearing
              special_senses: included
          deliver_spell:
            spell_range: touch
            familiar_distance_to_caster_max_feet: 100
            familiar_economy: reaction
            timing: caster_casts_spell
            delivery: familiar_touch
          disappearance:
            zero_hp: disappears_until_recast
            dismissal:
              economy: magic_action
              options:
              - temporary_pocket_dimension
              - permanent
            return:
              requires: temporarily_dismissed
              economy: magic_action
              unoccupied: true
              distance_feet: 30
            on_zero_hp_or_pocket_dimension: leave_worn_and_carried_items_in_space
          limit:
            per_caster: 1
            recast_with_existing: adopt_new_eligible_form
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_find_familiar
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        forms: definition_list
        choices: choice
      outputs:
      - effects
      - participants
      - locations
      events:
      - spell.cast_completed
      - health.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - participant_lifecycle
      - forms
      - perception
      - controlled_participants
    requires:
    - dnd2024:spell_find_familiar
    citations:
    - SRD 5.2.1 pp. 130; builtin/srd-5.2.1/spell/find_familiar.md
  dnd2024:spells/finger_of_death:
    source_sha256: 6021cd8a26f8a38e0943d63ba4ccb6cf54bde45cb3c8c956677838c6c83e6c9a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/finger_of_death
        implementation: dnd2024:spell_finger_of_death
        parameters:
          targeting:
            kind: creature
            count: 1
            visible: true
            within_range: true
          save:
            ability: constitution
            failure:
              damage:
                dice: 7d8
                add: 30
                type: necrotic
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          killed_humanoid:
            event: caster.next_turn_started
            form: zombie
            obeys: caster_verbal_orders
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_finger_of_death
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        forms: definition_list
        save_dc: integer
      outputs:
      - effects
      - damage_results
      - participants
      events:
      - spell.cast_completed
      - turn.started
      - health.changed
      capabilities:
      - spellcasting
      - effects
      - damage
      - participant_lifecycle
      - forms
    requires:
    - dnd2024:spell_finger_of_death
    citations:
    - SRD 5.2.1 pp. 131; builtin/srd-5.2.1/spell/finger_of_death.md
  dnd2024:spells/fireball:
    source_sha256: 9f3d828bdc9e118a2228363a03198bef0c7d03da794dabb5ec8514c6aa37f202
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/fireball
        implementation: dnd2024:spell_fireball
        parameters:
          targeting:
            kind: creatures_in_area
            origin: chosen_point_in_range
            shape: sphere
            radius_feet: 20
          save:
            ability: dexterity
            failure:
              damage:
                dice: 8d6
                type: fire
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          objects:
            within_area: true
            flammable: true
            worn: false
            carried: false
            effect: burning
          slot_scaling:
            above: 3
            damage_per_level: 1d6
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_fireball
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        objects:
          type: object_list
          required: false
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_fireball
    citations:
    - SRD 5.2.1 pp. 131; builtin/srd-5.2.1/spell/fireball.md
  dnd2024:spells/flesh_to_stone:
    source_sha256: a486a027353b3e8ee99249df1c4106c9cf473c1027c5599e1378585bf60cfc74
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/flesh_to_stone
        implementation: dnd2024:spell_flesh_to_stone
        parameters:
          targeting:
            kind: creature
            count: 1
            visible: true
            within_range: true
          initial_save:
            ability: constitution
            automatic_success_types:
            - construct
            failure:
              condition: restrained
              duration: spell_duration
            success:
              speed: 0
              until: caster_next_turn_start
          progression:
            event: restrained_target.turn_ended
            save_ability: constitution
            counters:
              scope: saves_against_this_spell
              initial_failure_counts:
                type: choice
                chooser: table
                options:
                - true
                - false
                source_scope: saves_against_this_spell
              consecutive_required: false
              success_threshold: 3
              failure_threshold: 3
            on_success_threshold: end_spell
            on_failure_threshold:
              form: stone
              condition: petrified
              duration: spell_duration
          full_concentration_duration:
            condition: petrified
            duration: until_removed
            removal:
            - greater_restoration
            - similar_magic
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_flesh_to_stone
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      events:
      - spell.cast_completed
      - turn.ended
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - conditions
    requires:
    - dnd2024:spell_flesh_to_stone
    citations:
    - SRD 5.2.1 pp. 133; builtin/srd-5.2.1/spell/flesh_to_stone.md
  dnd2024:spells/forbiddance:
    source_sha256: c27c03f388dec22172077599fa11b693eac6d61d7e7d3de33ce22005b3b50a1b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/forbiddance
        implementation: dnd2024:spell_forbiddance
        parameters:
          targeting:
            kind: floor_area
            touch: true
            max_square_feet: 40000
            height_feet: 30
            cannot_overlap: other_forbiddance
          travel_block:
            entry_modes:
            - teleportation
            - portals
            - planar_travel
            includes:
            - astral
            - ethereal
            - feywild
            - shadowfell
            - plane_shift
            duration: spell_duration
          damage:
            types_choice:
              minimum: 1
              choices:
              - aberration
              - celestial
              - elemental
              - fey
              - fiend
              - undead
            events:
            - first_area_entry_each_turn
            - target_turn_end_inside
            dice: 5d10
            damage_type_choice:
            - radiant
            - necrotic
            exemption: password_spoken_on_entry
            password: caster_optional_at_cast
          persistence:
            same_location: true
            consecutive_days: 30
            frequency: daily
            resulting_duration: until_dispelled
            material_consumed: last_cast_only
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: day
    contract:
      id: dnd2024:spell_forbiddance
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        choices: choice
        campaign: campaign
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      - turn.ended
      - area.entered
      - time.elapsed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
      - campaign_state
      - planar_travel
    requires:
    - dnd2024:spell_forbiddance
    citations:
    - SRD 5.2.1 pp. 133; builtin/srd-5.2.1/spell/forbiddance.md
  dnd2024:spells/freezing_sphere:
    source_sha256: 0e565dd7f3d3337871a95a14e4a4440dc28fac785ada95baa0d7c3013877c3f9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/freezing_sphere
        implementation: dnd2024:spell_freezing_sphere
        parameters:
          targeting:
            kind: creatures_in_area
            origin: chosen_point_in_range
            shape: sphere
            radius_feet: 60
          save:
            ability: constitution
            failure:
              damage:
                dice: 10d6
                type: cold
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          water_impact:
            freeze_depth_inches: 6
            square_side_feet: 30
            duration_minutes: 1
            targets: surface_swimming_creatures
            condition: restrained
            escape:
              economy: action
              check:
                ability: strength
                skill: athletics
                dc: spell_save_dc
              success: break_free
          delayed_choice:
            spawn: caster_hand
            size: sling_bullet
            temperature: cool
            transfer_to_other_creature: true
            launch_timing: any_time
            launch_options:
            - kind: throw
              range_feet: 40
            - kind: sling
              range: sling_normal
            impact: same_spell_explosion
            set_down_without_shattering: true
            automatic_explosion_after_minutes: 1
          slot_scaling:
            above: 6
            damage_per_level: 1d6
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_freezing_sphere
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      - time.elapsed
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
      - spatial_effects
    requires:
    - dnd2024:spell_freezing_sphere
    citations:
    - SRD 5.2.1 pp. 134; builtin/srd-5.2.1/spell/freezing_sphere.md
  dnd2024:spells/gentle_repose:
    source_sha256: cb7add601498667319312bceefaa52fab0f9830eccb0a87b3ec9a5f97d7d7ea4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/gentle_repose
        implementation: dnd2024:spell_gentle_repose
        parameters:
          targeting:
            kind: corpse_or_remains
            touch: true
          duration: spell_duration
          protection:
            decay: prevented
            becoming_undead: prevented
          resurrection_deadline:
            exclude_days: days_under_this_effect
          spell_lifetime:
            concentration: false
            maximum:
              value: 10
              unit: day
    contract:
      id: dnd2024:spell_gentle_repose
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        campaign: campaign
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - campaign_state
    requires:
    - dnd2024:spell_gentle_repose
    citations:
    - SRD 5.2.1 pp. 135; builtin/srd-5.2.1/spell/gentle_repose.md
  dnd2024:spells/glyph_of_warding:
    source_sha256: cf360c79f5e75109af3f80ff5a1759cbb3d012ad5ad8ad769e7bd66ac1c1bba8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/glyph_of_warding
        implementation: dnd2024:spell_glyph_of_warding
        parameters:
          targeting:
            kind: surface_or_closable_object_interior
            touch: true
            diameter_max_feet: 10
          duration: until_dispelled_or_triggered
          movement_limit:
            distance_from_cast_location_feet: 10
            exceeded:
              break_glyph: true
              end_spell: true
              trigger: false
          detect:
            check:
              ability: wisdom
              skill: perception
              dc: spell_save_dc
            success_required: true
          trigger:
            chooser: caster_at_cast
            specification:
              type: choice
              predicate_record_required: true
              modes:
              - surface_touch
              - surface_step
              - covering_removed
              - proximity
              - object_opened
              - glyph_seen
              - custom_event_predicate
              optional_filters:
              - creature_types
              optional_exemptions:
              - spoken_password
              - custom_creature_predicate
              custom_predicates:
                type: choice
                must_provide:
                - event
                - subject
                - predicate_parameters
                unresolved_predicate: block_release
            resolution:
              release_effect: true
              end_glyph_spell: true
          mode_choice:
          - explosive_rune
          - spell_glyph
          explosive_rune:
            area:
              shape: sphere
              radius_feet: 20
              center: glyph
            targets: all_creatures_in_area
            save:
              ability: dexterity
              failure:
                damage:
                  dice: 5d8
                  type_choice_at_cast:
                  - acid
                  - cold
                  - fire
                  - lightning
                  - thunder
              success:
                damage_fraction:
                  numerator: 1
                  denominator: 2
            slot_scaling:
              above: 3
              damage_per_level: 1d8
          spell_glyph:
            stored_spell:
              prepared: true
              max_level: cast_slot_level
              allowed_targeting:
              - single_creature
              - area
              cast_during_glyph_creation: true
              immediate_effect: false
            release:
              single_target: triggering_creature
              area_center: triggering_creature
              hostile_summons_or_harmful_objects_or_traps:
                location: nearest_possible_to_intruder
                attack: intruder
              concentration_spell:
                duration: full_duration
                concentration_required: false
          spell_lifetime:
            concentration: false
            maximum:
              kind: until_dispelled_or_triggered
    contract:
      id: dnd2024:spell_glyph_of_warding
      inputs:
        caster: participant
        cast_level: integer
        locations: location_list
        objects: object_list
        save_dc: integer
        choices: choice
        campaign: campaign
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
      - campaign_state
      - perception
    requires:
    - dnd2024:spell_glyph_of_warding
    citations:
    - SRD 5.2.1 pp. 136, 137; builtin/srd-5.2.1/spell/glyph_of_warding.md
  dnd2024:spells/greater_restoration:
    source_sha256: c4bac956a32e020060c51189ea4a963d84436755c1b04e0f8c7a172dbf00c4d7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/greater_restoration
        implementation: dnd2024:spell_greater_restoration
        parameters:
          targeting:
            kind: creature
            touch: true
            count: 1
          remove_choice_count: 1
          choices:
          - kind: exhaustion
            levels: 1
          - kind: condition
            choices:
            - charmed
            - petrified
          - kind: curse
            include: cursed_item_attunement
          - kind: ability_score_reduction
            ability_count: 1
            reduction: any
          - kind: hp_maximum_reduction
            reduction: any
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_greater_restoration
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices: choice
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - conditions
      - health_maximum_modifiers
    requires:
    - dnd2024:spell_greater_restoration
    citations:
    - SRD 5.2.1 pp. 137; builtin/srd-5.2.1/spell/greater_restoration.md
  dnd2024:spells/guiding_bolt:
    source_sha256: fb535a11dab67c098965b4777ca793e8398c7b6218acb0e915854b4a129f6620
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/guiding_bolt
        implementation: dnd2024:spell_guiding_bolt
        parameters:
          targeting:
            kind: creature
            count: 1
            within_range: true
          attack:
            kind: ranged_spell
            hit:
              damage:
                dice: 4d6
                type: radiant
              rider:
                next_attack_against_target: advantage
                deadline: caster_next_turn_end
                consumes_on: next_attack_roll
          slot_scaling:
            above: 1
            damage_per_level: 1d6
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: round
    contract:
      id: dnd2024:spell_guiding_bolt
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - attack.hit
      - attack.roll_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
    requires:
    - dnd2024:spell_guiding_bolt
    citations:
    - SRD 5.2.1 pp. 138; builtin/srd-5.2.1/spell/guiding_bolt.md
  dnd2024:spells/harm:
    source_sha256: 890153265c624c096e47988b9ae4e47e98fe83a094d6689d22389c046e7b8cb9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/harm
        implementation: dnd2024:spell_harm
        parameters:
          targeting:
            kind: creature
            count: 1
            visible: true
            within_range: true
          save:
            ability: constitution
            failure:
              damage:
                dice: 14d6
                type: necrotic
              hp_maximum_reduction:
                amount: necrotic_damage_taken
                minimum_resulting_hp_maximum: 1
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
              hp_maximum_reduction: false
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_harm
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - health_maximum_modifiers
    requires:
    - dnd2024:spell_harm
    citations:
    - SRD 5.2.1 pp. 139; builtin/srd-5.2.1/spell/harm.md
  dnd2024:spells/heat_metal:
    source_sha256: 2892d0087a349ef9edc63d742ba0657081a1b161e928b1f1f7e67a3010e09fd3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/heat_metal
        implementation: dnd2024:spell_heat_metal
        parameters:
          targeting:
            kind: object
            manufactured: true
            material: metal
            visible: true
            within_range: true
          object_state:
            red_hot_glow: true
            duration: spell_duration
          damage:
            targets: creatures_physically_contacting_object
            dice: 2d8
            type: fire
            initial: on_cast
            repeat:
              economy: bonus_action
              timing: each_later_caster_turn
              requires: object_within_range
          holder_or_wearer_damaged:
            save:
              ability: constitution
              failure: drop_object_if_able
            if_not_dropped:
              disadvantage:
              - attack_rolls
              - ability_checks
              until: caster_next_turn_start
          slot_scaling:
            above: 2
            damage_per_level: 1d8
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_heat_metal
      inputs:
        caster: participant
        cast_level: integer
        objects: object_list
        save_dc: integer
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - turn.started
      - damage.applied
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_heat_metal
    citations:
    - SRD 5.2.1 pp. 139, 140; builtin/srd-5.2.1/spell/heat_metal.md
  dnd2024:spells/hex:
    source_sha256: c9767b5a741ec86f2997a380875172066ac3d46495441dd2a8448694c6cb2d66
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/hex
        implementation: dnd2024:spell_hex
        parameters:
          targeting:
            kind: creature
            count: 1
            visible: true
            within_range: true
          curse:
            duration: spell_duration
            additional_damage:
              event: caster_hits_target_with_attack_roll
              dice: 1d6
              type: necrotic
            disadvantage:
              rolls: ability_checks
              ability: caster_choice_at_cast
          transfer:
            requires: target_dropped_to_zero_hp_before_spell_ends
            economy: bonus_action
            timing: later_turn
            target: new_creature
          slot_duration_hours:
            1: 1
            2: 4
            3: 8
            4: 8
            5: 24
            6: 24
            7: 24
            8: 24
            9: 24
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_hex
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices: choice
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - damage.applied
      - attack.hit
      - health.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
    requires:
    - dnd2024:spell_hex
    citations:
    - SRD 5.2.1 pp. 140; builtin/srd-5.2.1/spell/hex.md
  dnd2024:spells/holy_aura:
    source_sha256: 72cf641680a1a1d810439b1276caa6da817d2bd3a5f3b917f072da197e8337b5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/holy_aura
        implementation: dnd2024:spell_holy_aura
        parameters:
          targeting:
            kind: chosen_creatures_in_aura
            shape: emanation
            radius_feet: 30
            origin: caster
          duration: spell_duration
          benefits_while_inside:
            saving_throws: advantage
            attacks_by_other_creatures_against_affected: disadvantage
          retaliation:
            event: melee_attack_roll_hits_affected_creature
            attacker_types:
            - fiend
            - undead
            save:
              ability: constitution
              failure:
                condition: blinded
                until: attacker_next_turn_end
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_holy_aura
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      events:
      - spell.cast_completed
      - area.entered
      - attack.hit
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - conditions
      - spatial_effects
    requires:
    - dnd2024:spell_holy_aura
    citations:
    - SRD 5.2.1 pp. 141; builtin/srd-5.2.1/spell/holy_aura.md
  dnd2024:spells/ice_storm:
    source_sha256: dc6e55430969f5daebf23fdc517fc0b852fbd48127ad2ac534056bf5b3f7debe
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/ice_storm
        implementation: dnd2024:spell_ice_storm
        parameters:
          targeting:
            kind: creatures_in_area
            shape: cylinder
            radius_feet: 20
            height_feet: 40
            origin: chosen_point_in_range
          save:
            ability: dexterity
            failure:
              damage:
              - dice: 2d10
                type: bludgeoning
              - dice: 4d6
                type: cold
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          terrain:
            area: ground_in_cylinder
            difficult_terrain: true
            until: caster_next_turn_end
          slot_scaling:
            above: 4
            bludgeoning_damage_per_level: 1d10
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_ice_storm
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_ice_storm
    citations:
    - SRD 5.2.1 pp. 141, 142; builtin/srd-5.2.1/spell/ice_storm.md
  dnd2024:spells/incendiary_cloud:
    source_sha256: c3441261e050dfe0b1004bc7cc5c5eda26e6be6daeadfe90eae66edac4e3e089
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/incendiary_cloud
        implementation: dnd2024:spell_incendiary_cloud
        parameters:
          targeting:
            kind: creatures_in_area
            shape: sphere
            radius_feet: 20
            origin: chosen_point_in_range
          cloud:
            heavily_obscured: true
            until_any:
            - spell_duration
            - strong_wind_dispersal
          damage_triggers:
            events:
            - cloud_appears
            - cloud_moves_into_creature_space
            - creature_enters_cloud
            - creature_ends_turn_in_cloud
            max_per_creature_per_turn: 1
          save:
            ability: dexterity
            failure:
              damage:
                dice: 10d8
                type: fire
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          movement:
            event: caster.turn_started
            distance_feet: 10
            direction: caster_choice_away_from_caster
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_incendiary_cloud
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      - turn.started
      - turn.ended
      - area.entered
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_incendiary_cloud
    citations:
    - SRD 5.2.1 pp. 142, 143; builtin/srd-5.2.1/spell/incendiary_cloud.md
  dnd2024:spells/invisibility:
    source_sha256: 1976385732b3ae54a16c1943176beb9b240d565b1d64cf0a686130d8b70e3fef
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/invisibility
        implementation: dnd2024:spell_invisibility
        parameters:
          targeting:
            kind: creatures
            touch: true
            count:
              base: 1
              per_slot_above: 1
              slot_base: 2
          condition:
            name: invisible
            duration: spell_duration
          early_end:
            timing: immediately_after
            target_events:
            - attack_roll_made
            - damage_dealt
            - spell_cast
            scope: spell
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_invisibility
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - damage.applied
      - attack.hit
      - attack.roll_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
    requires:
    - dnd2024:spell_invisibility
    citations:
    - SRD 5.2.1 pp. 143; builtin/srd-5.2.1/spell/invisibility.md
  dnd2024:spells/legend_lore:
    source_sha256: 928de22d067926446881076fd79cd2aff5025ef1d4386be1a603347eb8307cd8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/legend_lore
        implementation: dnd2024:spell_legend_lore
        parameters:
          targeting:
            kind: named_or_described_subject
            categories:
            - person
            - place
            - object
            requires: famous
          lore:
            recipient: caster_mind
            responder: gm
            content: significant_lore_summary
            eligible_information:
            - important_details
            - amusing_revelations
            - secret_lore
            accuracy: true
            precision_increases_with: prior_subject_knowledge
            language_style_choice:
            - literal
            - figurative
            - poetry
          not_famous:
            spell_fails: true
            auditory_feedback: sad_trombone_notes
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_legend_lore
      inputs:
        caster: participant
        cast_level: integer
        choices: choice
        campaign: campaign
        subject_description: text
      outputs:
      - effects
      - information
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - campaign_state
    requires:
    - dnd2024:spell_legend_lore
    citations:
    - SRD 5.2.1 pp. 144; builtin/srd-5.2.1/spell/legend_lore.md
  dnd2024:spells/lightning_bolt:
    source_sha256: 4ae68974f4f87fe2dcec6b07777997782de21a64ad1f8161a195a7a17b16e403
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/lightning_bolt
        implementation: dnd2024:spell_lightning_bolt
        parameters:
          targeting:
            kind: creatures_in_area
            shape: line
            length_feet: 100
            width_feet: 5
            origin: caster
            direction: caster_choice
          save:
            ability: dexterity
            failure:
              damage:
                dice: 8d6
                type: lightning
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          slot_scaling:
            above: 3
            damage_per_level: 1d6
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_lightning_bolt
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_lightning_bolt
    citations:
    - SRD 5.2.1 pp. 144; builtin/srd-5.2.1/spell/lightning_bolt.md
  dnd2024:spells/longstrider:
    source_sha256: 8dc18456d5aa226fdcfc1cac4c1db77af34a1060eb4faaeecacbf5f149b53b23
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/longstrider
        implementation: dnd2024:spell_longstrider
        parameters:
          targeting:
            kind: creatures
            touch: true
            count:
              base: 1
              per_slot_above: 1
              slot_base: 1
          effect:
            speed_bonus_feet: 10
            duration: spell_duration
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_longstrider
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_longstrider
    citations:
    - SRD 5.2.1 pp. 145; builtin/srd-5.2.1/spell/longstrider.md
  dnd2024:spells/magic_jar:
    source_sha256: 2f8e93bdd3473c477a5c609f3433bdd9fefd4d7de4e0a8f66bf0833d208d21ae
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/magic_jar
        implementation: dnd2024:spell_magic_jar
        parameters:
          targeting:
            kind: self
            soul_container: material_component
          initial:
            body_state: catatonic
            soul_location: container
          container_state:
            perception_origin: container_space
            movement: forbidden
            reactions: forbidden
            only_action: soul_projection
            projection_max_feet: 100
          projection_choices:
          - return_to_living_original_body_and_end_spell
          - possess_humanoid
          possession:
            targeting:
              types:
              - humanoid
              visible: true
              range_feet: 100
              excluded_protections:
              - protection_from_evil_and_good
              - magic_circle
            save:
              ability: charisma
              failure:
                caster_soul: target_body
                target_soul: container
              success:
                possession: false
                immunity_to_caster_possession_hours: 24
            host_control: caster
            replace_stats:
            - hit_points
            - hit_point_dice
            - strength
            - dexterity
            - constitution
            - speed
            - senses
            retain_stats: all_other_caster_statistics
            host_soul_in_container:
              perception: own_senses_from_container
              movement: forbidden
              condition: incapacitated
          leave_host:
            economy: magic_action
            container_range_max_feet: 100
            caster_soul: container
            host_soul: host_body
          host_death:
            host_dies: true
            caster_save:
              ability: charisma
              dc: own_spell_save_dc
            caster_survives_if:
            - save_success
            - container_within_100_feet
            survival_destination: container
            otherwise: caster_dies
          termination:
            events:
            - container_destroyed
            - spell_ends
            caster_soul_return: original_body
            caster_dies_if_any:
            - original_body_dead
            - original_body_more_than_100_feet_from_caster_soul
            trapped_host_soul:
              return_if:
              - host_body_alive
              - host_body_within_100_feet_of_container
              otherwise: host_dies
            container_destroyed_on_spell_end: true
          spell_lifetime:
            concentration: false
            maximum:
              kind: until_dispelled
    contract:
      id: dnd2024:spell_magic_jar
      inputs:
        caster: participant
        cast_level: integer
        save_dc: integer
        choices: choice
        campaign: campaign
      outputs:
      - effects
      events:
      - spell.cast_completed
      - time.elapsed
      - health.changed
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - conditions
      - spatial_effects
      - campaign_state
      - perception
    requires:
    - dnd2024:spell_magic_jar
    citations:
    - SRD 5.2.1 pp. 146; builtin/srd-5.2.1/spell/magic_jar.md
  dnd2024:spells/magnificent_mansion:
    source_sha256: 77dbe3dbe24a46056083b0e4be6b1981e6efd1acd373096c82ee5754854db31f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/magnificent_mansion
        implementation: dnd2024:spell_magnificent_mansion
        parameters:
          targeting:
            kind: doorway_location
            within_range: true
          doorway:
            width_feet: 5
            height_feet: 10
            duration: spell_duration
            entry_allowed: caster_and_creatures_designated_at_cast
            entry_requires: open
            open_close:
              economy: none
              caster_range_feet: 30
            closed: imperceptible
          dwelling:
            extradimensional: true
            air:
            - clean
            - fresh
            - warm
            floor_plan: caster_choice
            max_contiguous_cubes: 50
            cube_side_feet: 10
            furnishings: caster_choice
            banquet:
              courses: 9
              people: 100
            created_objects_removed: dissipate_into_smoke
          servants:
            count: 100
            appearance_and_attire: caster_choice
            near_transparent: true
            invulnerable: true
            obeys: caster_commands
            tasks: human_capable
            forbidden:
            - attack
            - directly_harm_creature
            - leave_dwelling
            serves: all_entrants
          end:
            expel: all_remaining_creatures_and_objects
            destination: nearest_unoccupied_spaces_to_entrance
          spell_lifetime:
            concentration: false
            maximum:
              value: 24
              unit: hour
    contract:
      id: dnd2024:spell_magnificent_mansion
      inputs:
        caster: participant
        cast_level: integer
        locations: location_list
        choices: choice
        campaign: campaign
      outputs:
      - effects
      - locations
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - campaign_state
      - planar_travel
      - controlled_participants
    requires:
    - dnd2024:spell_magnificent_mansion
    citations:
    - SRD 5.2.1 pp. 147; builtin/srd-5.2.1/spell/magnificent_mansion.md
  dnd2024:spells/mass_healing_word:
    source_sha256: 757e40f93027884c6656f6ee632dc7589e573aa5c5c9603f406bd530f3b1fede
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mass_healing_word
        implementation: dnd2024:spell_mass_healing_word
        parameters:
          targeting:
            kind: creatures
            max_count: 6
            chosen: true
            visible: true
            within_range: true
          healing:
            dice: 2d4
            add:
              stat: caster_spellcasting_ability_modifier
          slot_scaling:
            above: 3
            healing_per_level: 1d4
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_mass_healing_word
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices: choice
      outputs:
      - effects
      - healing_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - healing
    requires:
    - dnd2024:spell_mass_healing_word
    citations:
    - SRD 5.2.1 pp. 148; builtin/srd-5.2.1/spell/mass_healing_word.md
  dnd2024:spells/mending:
    source_sha256: dbf9c315e6607b495137381995960da56808d9f0b2569f7d4b997bbc98cc2af9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mending
        implementation: dnd2024:spell_mending
        parameters:
          targeting:
            kind: object
            touch: true
            break_or_tear_count: 1
            break_or_tear_max_dimension_feet: 1
          repair:
            physical: true
            damage_trace_remaining: false
            magic_item_eligible: true
            restores_magic: false
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_mending
      inputs:
        caster: participant
        cast_level: integer
        objects: object_list
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
    requires:
    - dnd2024:spell_mending
    citations:
    - SRD 5.2.1 pp. 148; builtin/srd-5.2.1/spell/mending.md
  dnd2024:spells/mind_spike:
    source_sha256: 1f544c4671f31120661f6472779d73da9404a67e1bcf6d4030903f40d76ac2ee
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mind_spike
        implementation: dnd2024:spell_mind_spike
        parameters:
          targeting:
            kind: creature
            count: 1
            visible: true
            within_range: true
          save:
            ability: wisdom
            failure:
              damage:
                dice: 3d8
                type: psychic
              tracking:
                duration: spell_duration
                requires: same_plane
                knows_location: always
                target_cannot_hide_from_caster: true
                invisible_benefits_against_caster: none
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          slot_scaling:
            above: 2
            damage_per_level: 1d8
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_mind_spike
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
      - planar_travel
    requires:
    - dnd2024:spell_mind_spike
    citations:
    - SRD 5.2.1 pp. 149; builtin/srd-5.2.1/spell/mind_spike.md
  dnd2024:spells/mislead:
    source_sha256: 6b269d29bb0702147f65117545fad3bab1caa05b35cdfd002b29cc110c2d0217
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mislead
        implementation: dnd2024:spell_mislead
        parameters:
          targeting:
            kind: self
          invisible:
            condition: invisible
            duration: spell_duration
            ends_immediately_after:
            - caster_attack_roll
            - caster_damage_dealt
            - caster_spell_cast
            double_ends_with_invisibility: false
          double:
            spawn_location: caster_space_at_cast
            duration: spell_duration
            intangible: true
            invulnerable: true
            control:
              economy: magic_action
              movement_max:
                op: multiply
                args:
                - 2
                - stat: caster_speed
              behaviors:
              - gesture
              - speak
              - caster_chosen_behavior
            shared_senses:
              channels:
              - vision
              - hearing
              origin: double_location
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_mislead
      inputs:
        caster: participant
        cast_level: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - damage.applied
      - attack.hit
      - attack.roll_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
      - perception
      - controlled_participants
    requires:
    - dnd2024:spell_mislead
    citations:
    - SRD 5.2.1 pp. 150; builtin/srd-5.2.1/spell/mislead.md
  dnd2024:spells/move_earth:
    source_sha256: 471caf771d7c2962f306e7f1f9132a36894172524271d525a1920090d5e786fa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/move_earth
        implementation: dnd2024:spell_move_earth
        parameters:
          targeting:
            kind: terrain_area
            max_side_feet: 40
            within_range: true
            materials:
            - dirt
            - sand
            - clay
          reshape:
            chooser: caster
            modes:
            - raise_elevation
            - lower_elevation
            - create_trench
            - fill_trench
            - erect_wall
            - flatten_wall
            - form_pillar
            - other_shape
            max_extent:
              fraction_of_largest_area_dimension:
                numerator: 1
                denominator: 2
            completion_minutes: 10
            ordinary_trapping_or_injury: false
            exceptional_trapping_or_injury:
              type: choice
              chooser: gm
              requires: exceptional_terrain_circumstances
              resolution_required: true
          new_area:
            event: each_10_minutes_of_concentration_completed
            optional: true
            within_range: true
          exclusions:
          - natural_stone_manipulation
          - stone_construction_manipulation
          - direct_plant_growth_effect
          carried_with_terrain:
          - rocks
          - structures
          - plants
          unstable_structure:
            collapse:
              chooser: gm
              type: boolean
              requires: reshaping_makes_structure_unstable
          spell_lifetime:
            concentration: true
            maximum:
              value: 2
              unit: hour
    contract:
      id: dnd2024:spell_move_earth
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        choices: choice
        campaign: campaign
      outputs:
      - effects
      - locations
      events:
      - spell.cast_completed
      - time.elapsed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - campaign_state
    requires:
    - dnd2024:spell_move_earth
    citations:
    - SRD 5.2.1 pp. 151; builtin/srd-5.2.1/spell/move_earth.md
  dnd2024:spells/phantasmal_force:
    source_sha256: 2bbf37d2eb6548695d2a442764168e762ab15f652f2b8d66488c2a23bcde9f4d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/phantasmal_force
        implementation: dnd2024:spell_phantasmal_force
        parameters:
          targeting:
            kind: creature
            count: 1
            visible: true
            within_range: true
          save:
            ability: intelligence
            failure: create_phantasm
            success: no_effect
          phantasm:
            choices:
            - object
            - creature
            - phenomenon
            max_cube_side_feet: 10
            perceived_by: target_only
            duration: spell_duration
            stimuli:
            - visual
            - sound
            - temperature
            - other_sensory
            target_believes_real: true
            illogical_interaction: rationalized
          discern:
            actor: target
            economy: study_action
            check:
              ability: intelligence
              skill: investigation
              dc: spell_save_dc
            success:
              realizes_illusion: true
              end_spell: true
          damage:
            requires: phantasm_depicts_dangerous_creature_or_hazard
            timing: each_caster_turn
            optional: true
            target_location: inside_or_within_5_feet_of_phantasm
            dice: 2d8
            type: psychic
            perceived_type: appropriate_to_illusion
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_phantasmal_force
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - perception
    requires:
    - dnd2024:spell_phantasmal_force
    citations:
    - SRD 5.2.1 pp. 151, 152; builtin/srd-5.2.1/spell/phantasmal_force.md
  dnd2024:spells/planar_binding:
    source_sha256: 09f64e68219b83a58d087920bbb605e86bf5db69ad5a61cf978f406f68727ddb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/planar_binding
        implementation: dnd2024:spell_planar_binding
        parameters:
          targeting:
            kind: creature
            count: 1
            types:
            - celestial
            - elemental
            - fey
            - fiend
            within_range_for_entire_cast: true
          save:
            timing: casting_completion
            ability: charisma
            failure:
              bound_service: spell_duration
              summoned_or_created_by_other_spell: extend_that_spell_to_this_duration
          commands:
            obedience: best_of_ability
            hostile_target: twists_commands_toward_own_objectives
          completed_commands_early:
            same_plane: travel_to_caster_and_report
            different_plane: return_to_binding_location_and_wait_until_spell_end
          slot_duration_days:
            5: 1
            6: 10
            7: 30
            8: 180
            9: 366
          spell_lifetime:
            concentration: false
            maximum:
              value: 24
              unit: hour
    contract:
      id: dnd2024:spell_planar_binding
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        campaign: campaign
      outputs:
      - effects
      events:
      - spell.cast_completed
      - time.elapsed
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - campaign_state
      - planar_travel
      - controlled_participants
    requires:
    - dnd2024:spell_planar_binding
    citations:
    - SRD 5.2.1 pp. 152, 153; builtin/srd-5.2.1/spell/planar_binding.md
  dnd2024:spells/polymorph:
    source_sha256: 01003aa0efaeaaf2cbe709b338ac559d3a21e8a6d781b59efbf6bd0eb2b7932d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/polymorph
        implementation: dnd2024:spell_polymorph
        parameters:
          targeting:
            kind: creature
            count: 1
            visible: true
            within_range: true
          save:
            ability: wisdom
            failure: transform
            success: no_effect
          form:
            chooser: caster
            type: beast
            maximum_cr: target_cr_or_level_if_no_cr
            replace_stats: chosen_beast_stat_block
            retain:
            - alignment
            - personality
            - creature_type
            - hit_points
            - hit_point_dice
            duration: spell_duration
          temporary_hp:
            amount: beast_form_hit_points
            remainder_vanishes_on_spell_end: true
            target_spell_ends_when: no_temporary_hp_remaining
          restrictions:
            actions: new_anatomy_limits
            speech: forbidden
            spellcasting: forbidden
          equipment:
            melds_into_form: true
            use: forbidden
            benefits: none
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_polymorph
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        forms: definition_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - participants
      events:
      - spell.cast_completed
      - health.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - participant_lifecycle
      - forms
    requires:
    - dnd2024:spell_polymorph
    citations:
    - SRD 5.2.1 pp. 153, 154; builtin/srd-5.2.1/spell/polymorph.md
  dnd2024:spells/prayer_of_healing:
    source_sha256: adac7326929f62df6196ae41834ea2a2745ec9c98de3315cfe3efac84396abfd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/prayer_of_healing
        implementation: dnd2024:spell_prayer_of_healing
        parameters:
          targeting:
            kind: creatures
            max_count: 5
            chosen: true
            within_range_for_entire_cast: true
            eligibility: no_benefit_since_last_long_rest
          benefits:
            short_rest: true
            healing:
              dice: 2d8
          repeat_limit:
            scope: each_target
            reset: target_long_rest_finished
          slot_scaling:
            above: 2
            healing_per_level: 1d8
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_prayer_of_healing
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices: choice
      outputs:
      - effects
      - healing_results
      events:
      - spell.cast_completed
      - rest.completed
      capabilities:
      - spellcasting
      - effects
      - rest_lifecycle
      - healing
    requires:
    - dnd2024:spell_prayer_of_healing
    citations:
    - SRD 5.2.1 pp. 154; builtin/srd-5.2.1/spell/prayer_of_healing.md
  dnd2024:spells/private_sanctum:
    source_sha256: 2369b5f5635c8788921bb83c767cf62aaa600d546ab184ba54fe4a3d712e4555
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/private_sanctum
        implementation: dnd2024:spell_private_sanctum
        parameters:
          targeting:
            kind: area
            shape: cube
            min_side_feet: 5
            max_side_feet: 100
            within_range: true
          security:
            chooser: caster_at_cast
            count: any
            choices:
            - block_sound_crossing_boundary
            - dark_foggy_boundary_blocks_vision_including_darkvision
            - block_divination_sensor_appearance_inside_and_perimeter_crossing
            - creatures_inside_untargetable_by_divination_spells
            - block_teleportation_in_and_out
            - block_planar_travel_within
          duration: spell_duration
          persistence:
            same_spot: true
            frequency: daily
            consecutive_days: 365
            resulting_duration: until_dispelled
          slot_scaling:
            above: 4
            max_cube_side_feet_per_level: 100
          spell_lifetime:
            concentration: false
            maximum:
              value: 24
              unit: hour
    contract:
      id: dnd2024:spell_private_sanctum
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        choices: choice
        campaign: campaign
      outputs:
      - effects
      - locations
      events:
      - spell.cast_completed
      - time.elapsed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - campaign_state
      - planar_travel
      - perception
    requires:
    - dnd2024:spell_private_sanctum
    citations:
    - SRD 5.2.1 pp. 156; builtin/srd-5.2.1/spell/private_sanctum.md
  dnd2024:spells/protection_from_energy:
    source_sha256: 5f22c520c2d8c7c19dc2c6cc947cb1017dfdfe5c4954ab42b209246c7a89e597
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/protection_from_energy
        implementation: dnd2024:spell_protection_from_energy
        parameters:
          targeting:
            kind: creature
            count: 1
            willing: true
            touch: true
          resistance:
            choose_count: 1
            damage_types:
            - acid
            - cold
            - fire
            - lightning
            - thunder
            duration: spell_duration
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_protection_from_energy
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
    requires:
    - dnd2024:spell_protection_from_energy
    citations:
    - SRD 5.2.1 pp. 157; builtin/srd-5.2.1/spell/protection_from_energy.md
  dnd2024:spells/raise_dead:
    source_sha256: cc336b0db147bfb114ec4a3c1109351d58a4076d7ed3967c1a57e656244a5ba6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/raise_dead
        implementation: dnd2024:spell_raise_dead
        parameters:
          targeting:
            kind: dead_creature
            touch: true
            dead_max_days: 10
            excluded_type_at_death: undead
          failure_if: missing_survival_essential_body_parts_or_organs
          revival:
            hit_points: 1
            neutralizes: poisons_present_at_death
            closes: all_mortal_wounds
            restores_missing_parts: false
          ordeal:
            rolls: d20_tests
            modifier: -4
            recovery:
              event: target.long_rest_finished
              reduction_in_penalty: 1
              stop_at: 0
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_raise_dead
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        campaign: campaign
      outputs:
      - effects
      events:
      - spell.cast_completed
      - rest.completed
      capabilities:
      - spellcasting
      - effects
      - campaign_state
      - rest_lifecycle
    requires:
    - dnd2024:spell_raise_dead
    citations:
    - SRD 5.2.1 pp. 157; builtin/srd-5.2.1/spell/raise_dead.md
  dnd2024:spells/regenerate:
    source_sha256: 9729fbcd9c67e2e84d517f086b3073f9e13dcc29219844eab33ecfac41dce9e6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/regenerate
        implementation: dnd2024:spell_regenerate
        parameters:
          targeting:
            kind: creature
            count: 1
            touch: true
          initial_healing:
            dice: 4d8
            add: 15
          ongoing:
            duration: spell_duration
            healing:
              event: target.turn_started
              amount: 1
            severed_part_regrowth_minutes: 2
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_regenerate
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      - healing_results
      events:
      - spell.cast_completed
      - turn.started
      - time.elapsed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - healing
    requires:
    - dnd2024:spell_regenerate
    citations:
    - SRD 5.2.1 pp. 158; builtin/srd-5.2.1/spell/regenerate.md
  dnd2024:spells/resistance:
    source_sha256: b0c7405fe0e3d24e511542de41b5854fe74ff86f7caa6a2a841c263e9a560d09
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/resistance
        implementation: dnd2024:spell_resistance
        parameters:
          targeting:
            kind: creature
            count: 1
            willing: true
            touch: true
          damage_type_choice:
          - acid
          - bludgeoning
          - cold
          - fire
          - lightning
          - necrotic
          - piercing
          - poison
          - radiant
          - slashing
          - thunder
          reduction:
            event: target.takes_chosen_damage_type
            total_damage_reduced_by: 1d4
            max_benefits_per_creature_per_turn: 1
            duration: spell_duration
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_resistance
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices: choice
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - damage.applied
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
    requires:
    - dnd2024:spell_resistance
    citations:
    - SRD 5.2.1 pp. 158; builtin/srd-5.2.1/spell/resistance.md
  dnd2024:spells/rope_trick:
    source_sha256: e9616ca471da6df3094633894ce00ed51a648a2dc04374b9eb6cca819dd27620
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/rope_trick
        implementation: dnd2024:spell_rope_trick
        parameters:
          targeting:
            kind: rope
            touch: true
          rope:
            upper_end_hovers_until: perpendicular_or_ceiling_reached
            can_pull_inside: true
            can_drop_outside: true
          portal:
            location: rope_upper_end
            invisible: true
            width_feet: 3
            height_feet: 5
            destination: extradimensional_space
            access: climb_rope
            duration: spell_duration
          space:
            capacity_creatures: 8
            maximum_creature_size: medium
            blocked_crossing:
            - attacks
            - spells
            - other_effects
            vision_from_inside_through_portal: true
            spell_end: contents_drop_out
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_rope_trick
      inputs:
        caster: participant
        cast_level: integer
        locations: location_list
        objects: object_list
      outputs:
      - effects
      - locations
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - planar_travel
    requires:
    - dnd2024:spell_rope_trick
    citations:
    - SRD 5.2.1 pp. 159; builtin/srd-5.2.1/spell/rope_trick.md
  dnd2024:spells/scrying:
    source_sha256: ad8b1cab7d5bed8335adc811732f31bc2422f2f321d342fff9088b16fcac971e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/scrying
        implementation: dnd2024:spell_scrying
        parameters:
          targeting:
            options:
            - kind: creature
              same_plane: true
            - kind: location
              previously_seen: true
          creature_save:
            ability: wisdom
            knowledge_modifier:
              secondhand: 5
              firsthand: 0
              extensive: -5
            connection_modifier:
              none: 0
              likeness: -2
              possession: -4
              body_part_hair_or_nail: -10
            combination: knowledge_plus_selected_connection
            target_awareness: unease_without_knowing_save_source
            success:
              no_effect: true
              cannot_retarget_hours: 24
            failure: create_sensor
            connection_selection:
              type: choice
              chooser: caster
              choices: available_physical_connections
              count: 1
          sensor:
            invisible: true
            intangible: true
            channels:
            - vision
            - hearing
            perspective: sensor_location
            creature_mode:
              distance_max_feet: 10
              moves_with_target: true
            location_mode:
              location: chosen_location
              moves: false
              save: none
            visible_appearance: fist_sized_luminous_orb
            duration: spell_duration
          spell_lifetime:
            concentration: true
            maximum:
              value: 10
              unit: minute
    contract:
      id: dnd2024:spell_scrying
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        save_dc: integer
        choices: choice
        campaign: campaign
      outputs:
      - effects
      - locations
      events:
      - spell.cast_completed
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - campaign_state
      - planar_travel
      - perception
    requires:
    - dnd2024:spell_scrying
    citations:
    - SRD 5.2.1 pp. 160; builtin/srd-5.2.1/spell/scrying.md
  dnd2024:spells/seeming:
    source_sha256: 00ac5fd241cb70bb7bbf3ea47442021e952ddda40a9108354c3fcada38127f74
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/seeming
        implementation: dnd2024:spell_seeming
        parameters:
          targeting:
            kind: creatures
            chosen: true
            visible: true
            within_range: true
          unwilling:
            save_optional: true
            ability: charisma
            success: unaffected
          illusion:
            appearance_per_target: same_or_different_caster_choice
            includes:
            - body
            - equipment
            height_change_max_feet: 1
            weight_appearance:
            - heavier
            - lighter
            limb_arrangement: same_basic
            remaining_appearance: caster_choice
            duration: spell_duration
            physical_inspection: fails
            physical_substance: false
          discern:
            economy: study_action
            check:
              ability: intelligence
              skill: investigation
              dc: spell_save_dc
            success: observer_knows_disguised
          spell_lifetime:
            concentration: false
            maximum:
              value: 8
              unit: hour
    contract:
      id: dnd2024:spell_seeming
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - perception
    requires:
    - dnd2024:spell_seeming
    citations:
    - SRD 5.2.1 pp. 160, 161; builtin/srd-5.2.1/spell/seeming.md
  dnd2024:spells/shatter:
    source_sha256: 0339ea00d56eed773cf8d195cfa78bcdef658515b09b90636a0da6cf38f40417
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/shatter
        implementation: dnd2024:spell_shatter
        parameters:
          targeting:
            kind: creatures_in_area
            shape: sphere
            radius_feet: 10
            origin: chosen_point_in_range
          save:
            ability: constitution
            disadvantage_types:
            - construct
            failure:
              damage:
                dice: 3d8
                type: thunder
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          object_damage:
            within_area: true
            magical: false
            worn: false
            carried: false
            damage:
              dice: 3d8
              type: thunder
            save: none
          sound: loud_noise
          slot_scaling:
            above: 2
            damage_per_level: 1d8
            applies_to:
            - creatures
            - objects
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_shatter
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        objects:
          type: object_list
          required: false
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_shatter
    citations:
    - SRD 5.2.1 pp. 161; builtin/srd-5.2.1/spell/shatter.md
  dnd2024:spells/shining_smite:
    source_sha256: c7c0b558fbe265bc97b4b73ddf987db5f796a3b976974535bd62c646db172774
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/shining_smite
        implementation: dnd2024:spell_shining_smite
        parameters:
          targeting:
            kind: creature_hit_by_triggering_melee_weapon_or_unarmed_strike
          extra_attack_damage:
            dice: 2d6
            type: radiant
          rider:
            duration: spell_duration
            bright_light_radius_feet: 5
            attacks_against_target: advantage
            invisible_condition_benefits: none
          slot_scaling:
            above: 2
            damage_per_level: 1d6
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_shining_smite
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - attack.hit
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
    requires:
    - dnd2024:spell_shining_smite
    citations:
    - SRD 5.2.1 pp. 162; builtin/srd-5.2.1/spell/shining_smite.md
  dnd2024:spells/simulacrum:
    source_sha256: 10032eadb6975221c7004547346d9937fdeb842ca9d83cec1f0c2c2ff91d3c1f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/simulacrum
        implementation: dnd2024:spell_simulacrum
        parameters:
          targeting:
            kind: creature
            count: 1
            types:
            - beast
            - humanoid
            within_caster_feet_for_entire_cast: 10
            final_touch:
            - target
            - same_size_ice_or_snow_pile
          creation:
            transforms: ice_or_snow_pile
            statistics: original_at_casting
            replace_type: construct
            hp_maximum_fraction:
              numerator: 1
              denominator: 2
            cannot_cast: simulacrum
            duration: spell_duration
          behavior:
            attitude: friendly
            friendly_toward: caster_and_designated_creatures
            obeys: caster_commands
            combat_turn: caster_turn
          restrictions:
            gain_levels: false
            short_rest: false
            long_rest: false
          repair:
            only_hp_restoration_method: true
            timing: caster_long_rest
            consumed_component_gp_per_hp: 100
            distance_to_caster_max_feet: 5
            throughout_repair: true
          termination:
            zero_hp: revert_to_snow_and_melt
            caster_recasts_spell: instantly_destroy_previous_simulacra
          spell_lifetime:
            concentration: false
            maximum:
              kind: until_dispelled
    contract:
      id: dnd2024:spell_simulacrum
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects: object_list
        forms: definition_list
      outputs:
      - effects
      - participants
      events:
      - spell.cast_completed
      - rest.completed
      - time.elapsed
      - health.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - participant_lifecycle
      - forms
      - rest_lifecycle
      - controlled_participants
    requires:
    - dnd2024:spell_simulacrum
    citations:
    - SRD 5.2.1 pp. 162, 163; builtin/srd-5.2.1/spell/simulacrum.md
  dnd2024:spells/sorcerous_burst:
    source_sha256: 0f2c0dfc5dbb658e02567d0f4014b0721796a037038ee7b683baacecc1fa380a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/sorcerous_burst
        implementation: dnd2024:spell_sorcerous_burst
        parameters:
          targeting:
            kind: creature_or_object
            count: 1
            within_range: true
          attack:
            kind: ranged_spell
            hit:
              damage:
                dice: 1d8
                type_choice:
                - acid
                - cold
                - fire
                - lightning
                - poison
                - psychic
                - thunder
          exploding_dice:
            trigger: any_spell_d8_result_is_8
            optional: true
            additional_die: 1d8
            add_to_damage: true
            added_dice_can_trigger: true
            total_added_dice_limit_per_cast:
              stat: caster_spellcasting_ability_modifier
          cantrip_scaling:
            character_level_dice:
              1: 1d8
              5: 2d8
              11: 3d8
              17: 4d8
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_sorcerous_burst
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        choices: choice
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
    requires:
    - dnd2024:spell_sorcerous_burst
    citations:
    - SRD 5.2.1 pp. 163; builtin/srd-5.2.1/spell/sorcerous_burst.md
  dnd2024:spells/speak_with_plants:
    source_sha256: 5a9147f5c96e230c6c32f17e90f78b49a54fe350d38b9fe0ba15a8fef67eea25
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/speak_with_plants
        implementation: dnd2024:spell_speak_with_plants
        parameters:
          targeting:
            kind: plants_in_area
            shape: emanation
            radius_feet: 30
            origin: caster_at_cast
            immobile: true
          animation:
            duration: spell_duration
            limited_sentience: true
            communicates_with: caster
            follows: simple_commands
            movable_parts:
            - branches
            - tendrils
            - stalks
            uproot_or_move_about: false
          questions:
            scope: events_in_spell_area
            history_days: 1
            topics:
            - passing_creatures
            - weather
            - other_circumstances
          terrain_choices:
          - from: plant_growth_difficult_terrain
            to: ordinary
          - from: ordinary_with_plants
            to: difficult_terrain
          terrain_duration: spell_duration
          plant_creatures:
            communication: shared_language_equivalent
          spell_lifetime:
            concentration: false
            maximum:
              value: 10
              unit: minute
    contract:
      id: dnd2024:spell_speak_with_plants
      inputs:
        caster: participant
        cast_level: integer
        choices: choice
        campaign: campaign
      outputs:
      - effects
      - information
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - campaign_state
      - controlled_participants
    requires:
    - dnd2024:spell_speak_with_plants
    citations:
    - SRD 5.2.1 pp. 164; builtin/srd-5.2.1/spell/speak_with_plants.md
  dnd2024:spells/spiritual_weapon:
    source_sha256: b3b925ce7e6ba05a4eebe6f69d07ccd212ab4e5fb6cbebb851b666cc4384edeb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/spiritual_weapon
        implementation: dnd2024:spell_spiritual_weapon
        parameters:
          targeting:
            kind: force_location
            within_range: true
            choice: caster
          force:
            floating: true
            spectral: true
            weapon_appearance: caster_choice
            duration: spell_duration
          attack:
            initial: optional_immediately_on_cast
            kind: melee_spell
            target: one_creature
            within_force_feet: 5
            hit:
              damage:
                dice: 1d8
                add:
                  stat: caster_spellcasting_ability_modifier
                type: force
          repeat:
            economy: bonus_action
            timing: later_caster_turns
            movement_max_feet: 20
            repeat_attack: true
          slot_scaling:
            above: 2
            damage_per_level: 1d8
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_spiritual_weapon
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        choices: choice
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
      - spatial_effects
    requires:
    - dnd2024:spell_spiritual_weapon
    citations:
    - SRD 5.2.1 pp. 165; builtin/srd-5.2.1/spell/spiritual_weapon.md
  dnd2024:spells/stoneskin:
    source_sha256: 71b619c86f8f7f6e32d421a5f753dea387dc506fbfc74446d4e22283965ffed9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/stoneskin
        implementation: dnd2024:spell_stoneskin
        parameters:
          targeting:
            kind: creature
            count: 1
            willing: true
            touch: true
          resistance:
            types:
            - bludgeoning
            - piercing
            - slashing
            duration: spell_duration
            magical_damage_included: true
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_stoneskin
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
    requires:
    - dnd2024:spell_stoneskin
    citations:
    - SRD 5.2.1 pp. 165; builtin/srd-5.2.1/spell/stoneskin.md
  dnd2024:spells/sunbeam:
    source_sha256: cb3e20730d36e43c85d939236268275bec3b84634e1aaa3822c45f81da42b34c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/sunbeam
        implementation: dnd2024:spell_sunbeam
        parameters:
          targeting:
            kind: creatures_in_area
            shape: line
            width_feet: 5
            length_feet: 60
            origin: caster
          save:
            ability: constitution
            failure:
              damage:
                dice: 6d8
                type: radiant
              condition: blinded
              until: caster_next_turn_start
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          repeat:
            economy: magic_action
            new_line: true
            duration: spell_duration
          mote:
            location: above_caster
            bright_radius_feet: 30
            additional_dim_feet: 30
            sunlight: true
            duration: spell_duration
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_sunbeam
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - turn.started
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
      - spatial_effects
    requires:
    - dnd2024:spell_sunbeam
    citations:
    - SRD 5.2.1 pp. 166; builtin/srd-5.2.1/spell/sunbeam.md
  dnd2024:spells/telepathic_bond:
    source_sha256: d9ba38c9ee1f5729e23b0ef2e15b018adfa79e969bea4a3c9238b73063b5719b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/telepathic_bond
        implementation: dnd2024:spell_telepathic_bond
        parameters:
          targeting:
            kind: creatures
            max_count: 8
            willing: true
            chosen: true
            within_range: true
            must_communicate_in_at_least_one_language: true
          bond:
            topology: each_to_all
            channel: telepathic
            shared_language_required: false
            distance_limit: none
            planar_crossing: false
            duration: spell_duration
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_telepathic_bond
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices: choice
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_telepathic_bond
    citations:
    - SRD 5.2.1 pp. 168; builtin/srd-5.2.1/spell/telepathic_bond.md
  dnd2024:spells/thunderwave:
    source_sha256: 537c4e2a2fa209f2551d9f218763e2733e165906d25aab2b4db903ea03d76789
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/thunderwave
        implementation: dnd2024:spell_thunderwave
        parameters:
          targeting:
            kind: creatures_in_area
            shape: cube
            side_feet: 15
            origin: caster
          save:
            ability: constitution
            failure:
              damage:
                dice: 2d8
                type: thunder
              push:
                distance_feet: 10
                direction: away_from_caster
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          objects:
            unsecured: true
            entirely_within_area: true
            push_feet: 10
            direction: away_from_caster
          sound:
            kind: thunderous_boom
            audible_radius_feet: 300
          slot_scaling:
            above: 1
            damage_per_level: 1d8
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_thunderwave
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        save_dc: integer
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_thunderwave
    citations:
    - SRD 5.2.1 pp. 169; builtin/srd-5.2.1/spell/thunderwave.md
  dnd2024:spells/transport_via_plants:
    source_sha256: c9cfc4bbf9c8fa5b71ccffcb2411ef9555a229b0bfa478456272225de2072ae5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/transport_via_plants
        implementation: dnd2024:spell_transport_via_plants
        parameters:
          targeting:
            origin:
              kind: inanimate_plant
              size_min: large
              within_range: true
            destination:
              kind: plant
              same_plane: true
              distance_limit: none
              previously_seen_or_touched: true
          link:
            duration: spell_duration
            eligible_traveler: any_creature
            direction: origin_to_destination
            movement_cost_feet: 5
            enter: origin_plant
            exit: destination_plant
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_transport_via_plants
      inputs:
        caster: participant
        cast_level: integer
        locations: location_list
        objects: object_list
        campaign: campaign
      outputs:
      - effects
      - locations
      events:
      - spell.cast_completed
      - area.entered
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - campaign_state
      - planar_travel
    requires:
    - dnd2024:spell_transport_via_plants
    citations:
    - SRD 5.2.1 pp. 170; builtin/srd-5.2.1/spell/transport_via_plants.md
  dnd2024:spells/true_seeing:
    source_sha256: fbb045c0f3863169cde41f9b49d9ba7afd501923286f9100e28dff81856ed06e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/true_seeing
        implementation: dnd2024:spell_true_seeing
        parameters:
          targeting:
            kind: creature
            count: 1
            willing: true
            touch: true
          sense:
            kind: truesight
            range_feet: 120
            duration: spell_duration
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_true_seeing
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_true_seeing
    citations:
    - SRD 5.2.1 pp. 171; builtin/srd-5.2.1/spell/true_seeing.md
  dnd2024:spells/vampiric_touch:
    source_sha256: 7cdacadbb6de6c3268b13e034412707be6d114b815d7859689aa565a85f61679
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/vampiric_touch
        implementation: dnd2024:spell_vampiric_touch
        parameters:
          targeting:
            kind: creature
            count: 1
            within_reach: true
          attack:
            kind: melee_spell
            hit:
              damage:
                dice: 3d6
                type: necrotic
              caster_healing:
                fraction_of_necrotic_damage_dealt:
                  numerator: 1
                  denominator: 2
                rounding: down
          repeat:
            economy: magic_action
            timing: each_caster_turn
            duration: spell_duration
            target: same_or_different
          slot_scaling:
            above: 3
            damage_per_level: 1d6
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_vampiric_touch
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      - damage_results
      - healing_results
      events:
      - spell.cast_completed
      - turn.started
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - attacks
      - healing
    requires:
    - dnd2024:spell_vampiric_touch
    citations:
    - SRD 5.2.1 pp. 171; builtin/srd-5.2.1/spell/vampiric_touch.md
  dnd2024:spells/wall_of_force:
    source_sha256: 96d84fd26363af5d0447ca3eb299dcf60a053363a9e5b1ad286d30506bc7566a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/wall_of_force
        implementation: dnd2024:spell_wall_of_force
        parameters:
          targeting:
            kind: chosen_point_in_range
          wall:
            invisible: true
            orientation: caster_choice_any
            support_choice:
            - free_floating
            - solid_surface
            shape_choices:
            - shape: hemispherical_dome
              radius_max_feet: 10
            - shape: globe
              radius_max_feet: 10
            - shape: flat_panels
              count: 10
              width_feet: 10
              height_feet: 10
              each_contiguous_with_another: true
            thickness_inches:
              numerator: 1
              denominator: 4
            duration: spell_duration
          initial_overlap:
            targets: creatures_cut_by_wall
            push_to_side: caster_choice
          barrier:
            physical_passage: forbidden
            damage_immunity: all
            dispel_magic: ineffective
            disintegrate: instant_destruction
            extends_into: ethereal_plane
            ethereal_passage: forbidden
          spell_lifetime:
            concentration: true
            maximum:
              value: 10
              unit: minute
    contract:
      id: dnd2024:spell_wall_of_force
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        choices: choice
      outputs:
      - effects
      - damage_results
      - locations
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
      - planar_travel
    requires:
    - dnd2024:spell_wall_of_force
    citations:
    - SRD 5.2.1 pp. 172; builtin/srd-5.2.1/spell/wall_of_force.md
  dnd2024:spells/warding_bond:
    source_sha256: 5588945c2a025622920e9ca991a5e3883acff3d4ae8e7e4766143350486be38e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/warding_bond
        implementation: dnd2024:spell_warding_bond
        parameters:
          targeting:
            kind: creature
            count: 1
            willing: true
            other_than_caster: true
            touch: true
          ongoing_requirement:
            caster_and_target_wear_component_rings: true
          benefits:
            within_caster_feet: 60
            ac_bonus: 1
            saving_throw_bonus: 1
            damage_resistance: all
            duration: spell_duration
          shared_damage:
            event: target.takes_damage
            recipient: caster
            amount: same_as_target_damage_taken
          ends_on_any:
          - caster_zero_hp
          - separation_exceeds_60_feet
          - warding_bond_cast_again_on_either_connected_creature
          spell_lifetime:
            concentration: false
            maximum:
              value: 1
              unit: hour
    contract:
      id: dnd2024:spell_warding_bond
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - damage.applied
      - health.changed
      - position.changed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - spatial_effects
    requires:
    - dnd2024:spell_warding_bond
    citations:
    - SRD 5.2.1 pp. 173; builtin/srd-5.2.1/spell/warding_bond.md
  dnd2024:spells/weird:
    source_sha256: de1a74456cd0b7fc1936c5825b8ca90709633c0e6c1615a9ac0af73e10886faa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/weird
        implementation: dnd2024:spell_weird
        parameters:
          targeting:
            kind: chosen_creatures_in_area
            shape: sphere
            radius_feet: 30
            origin: chosen_point_in_range
          initial_save:
            ability: wisdom
            failure:
              damage:
                dice: 10d10
                type: psychic
              condition: frightened
              duration: spell_duration
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
          repeat:
            event: frightened_target.turn_ended
            save:
              ability: wisdom
              failure:
                damage:
                  dice: 5d10
                  type: psychic
              success:
                end_spell_on_target: true
          spell_lifetime:
            concentration: true
            maximum:
              value: 1
              unit: minute
    contract:
      id: dnd2024:spell_weird
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices: choice
      outputs:
      - effects
      - damage_results
      events:
      - spell.cast_completed
      - turn.ended
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - damage
      - conditions
      - spatial_effects
    requires:
    - dnd2024:spell_weird
    citations:
    - SRD 5.2.1 pp. 174; builtin/srd-5.2.1/spell/weird.md
  dnd2024:spells/word_of_recall:
    source_sha256: 4a30e05f293e7da9cf9d3611bd7a716ba0c169ba7c5f60502db3ed3db339e3c4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/word_of_recall
        implementation: dnd2024:spell_word_of_recall
        parameters:
          sanctuary_preparation:
            method: cast_this_spell_at_chosen_location
            persists_as: designated_sanctuary
          targeting:
            caster: included
            other_creatures:
              max_count: 5
              willing: true
              within_caster_feet: 5
          teleport:
            timing: instantaneous
            requires: previously_prepared_sanctuary
            destination: nearest_unoccupied_spaces_to_designated_sanctuary_spot
            no_prepared_sanctuary: no_teleport_effect
          spell_lifetime:
            concentration: false
            maximum:
              kind: instantaneous
    contract:
      id: dnd2024:spell_word_of_recall
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations: location_list
        choices: choice
        campaign: campaign
      outputs:
      - effects
      - locations
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - spatial_effects
      - campaign_state
      - planar_travel
    requires:
    - dnd2024:spell_word_of_recall
    citations:
    - SRD 5.2.1 pp. 175; builtin/srd-5.2.1/spell/word_of_recall.md
---

# SRD spell conversion findings: group 2

All 85 assigned spell definitions have source-specific typed rule contracts. The complete bundled SRD 5.2.1 spell files, including tables and embedded stat blocks, were read. Each source hash was verified against the assignment. No engine implementation is included.

The retained `splat_spell` headers own casting time, range, components, material prices and ordinary material consumption. `spell_lifetime` gives the typed base duration. A `spell_duration` parameter resolves to that lifetime, including concentration and explicit source-specific extensions. Other effect clocks (for example, the end of a turn or 30-day charm) remain separate. Conditional material consumption for Forbiddance and the continued ring requirement for Warding Bond are included in their parameters.

All parameter strings describing mechanics are enum symbols, named stat selectors, event selectors, or named source-specific policies. Choices carry their source-authorized chooser and constraints. Scene locations and valid geometry are supplied as location inputs; a missing geometric or discretionary fact must remain unresolved. Capability names describe required engine families and do not claim implementation. Events name the documented event families and are observed only at the source-defined timing in the parameters.

Flesh to Stone (p. 133) says to count three successes or failures against the spell but does not expressly say whether the initial failed save belongs in that counter. `initial_failure_counts` therefore requires an explicit table choice between true and false. This is the sole source ambiguity found; neither outcome is silently imposed. The full-duration petrification clause is retained independently as written.

Scrying (p. 160) adds one knowledge modifier and one selected physical-connection modifier. Its location mode keeps the source's separate target rule: a previously seen location with a stationary sensor. The same-plane condition is explicit for its creature mode. Glyph of Warding (pp. 136–137) permits custom triggers; those require a typed event-and-predicate record before they can resolve.

Several 2024 differences are material: Chill Touch uses a melee attack and d10 damage; Polymorph retains the target's creature type, Hit Points and Hit Point Dice and uses Temporary Hit Points; Spiritual Weapon requires concentration and adds a d8 per higher slot level; Stoneskin applies to all listed physical damage; Resistance reduces chosen damage by d4 once per turn; Prayer of Healing grants Short Rest benefits and has a per-target Long Rest limit. These clauses are represented in the corresponding cited records.

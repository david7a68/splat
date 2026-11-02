---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:spells/alarm:
    source_sha256: 358cab506ade01aa0f53ba7ce1f879df269e41e81adcfa88b158f5cd04899f16
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/alarm
        implementation: dnd2024:spell_alarm
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 28800
          target:
            kinds:
            - door
            - window
            - area
            range_feet: 30
            maximum_cube_side_feet: 20
          exemptions:
            chooser: caster
            selection: creature_list
            timing: cast
          trigger:
            events:
            - creature.touched_area
            - creature.entered_area
            exclude_designated_creatures: true
            repeat: every_occurrence
          alarm_choice:
            chooser: caster
            timing: cast
            options:
              audible:
                sound: handbell
                seconds: 10
                audible_distance_feet: 60
                origin: warded_area
              mental:
                recipient: caster
                maximum_distance_miles: 1
                signal: mental_ping
                awakens_sleeping_recipient: true
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_alarm
      inputs:
        caster: participant
        cast_level: integer
        warded_location: location
        exempt_creatures: participant_list
        alarm_mode: choice
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - information
      - objects
      events:
      - creature.entered_area
      - creature.touched_area
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - information
      - world_objects
      - areas
    requires:
    - dnd2024:spell_alarm
    citations:
    - SRD 5.2.1 pp. 107; builtin/srd-5.2.1/spell/alarm.md
  dnd2024:spells/animal_shapes:
    source_sha256: 24948ab9b40082b027d13f11f9b20df6b36edc22e001f2ef70fddb38e95d4393
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/animal_shapes
        implementation: dnd2024:spell_animal_shapes
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 86400
          targets:
            kind: creature
            willing: true
            visible: true
            range_feet: 30
            count: any
          forms:
            chooser: caster
            choice_per_target: true
            type: beast
            maximum_size: large
            maximum_challenge_rating: 4
          transform_again:
            economy: magic_action
            timing: later_caster_turn
            targets: still_transformed_original_targets
            new_choice_per_target: true
            additional_temporary_hit_points: false
          statistics:
            replace: form_stat_block
            retain:
            - creature_type
            - hit_points
            - hit_point_dice
            - alignment
            - communication
            - intelligence
            - wisdom
            - charisma
          restrictions:
            actions_limited_by_form_anatomy: true
            cast_spells: false
            equipment: melded
            equipment_use: false
          temporary_hit_points:
            amount:
              stat:
                of: first_chosen_form
                key: hit_points
            grant_count_per_target: 1
            remove_remaining_on_spell_end: true
          end_target:
            economy: bonus_action
            chooser: target
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_animal_shapes
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        forms: definition_list
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - forms
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - forms
      - inventory
    requires:
    - dnd2024:spell_animal_shapes
    citations:
    - SRD 5.2.1 pp. 108; builtin/srd-5.2.1/spell/animal_shapes.md
  dnd2024:spells/antimagic_field:
    source_sha256: 38fe8605716b31d83686e6ad5d7ea472fa8ace3d02bc1dac89dd6f855a52fcbe
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/antimagic_field
        implementation: dnd2024:spell_antimagic_field
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 3600
          area:
            shape: emanation
            radius_feet: 10
            origin: caster
            moves_with_origin: true
          forbid_inside:
          - cast_spells
          - magic_actions
          - create_magical_effects
          - teleport_in
          - teleport_out
          - planar_travel
          prevent_affecting_inside:
          - spells
          - magic_actions
          - magical_effects
          - magic_item_properties
          suppress_inside:
            magic_item_properties: true
            portals: temporarily_closed
            spell_and_magic_areas: excluded
          ongoing_spells:
            operation: suppress
            exceptions_by_caster:
            - artifact
            - deity
            duration_elapses_while_suppressed: true
            suppress_does_not_end: true
          exceptions:
            dispel_magic_affects_aura: false
            other_antimagic_fields_nullify_aura: false
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_antimagic_field
      inputs:
        caster: participant
        cast_level: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - magic_suppression
      - concentration
    requires:
    - dnd2024:spell_antimagic_field
    citations:
    - SRD 5.2.1 pp. 109; builtin/srd-5.2.1/spell/antimagic_field.md
  dnd2024:spells/arcane_lock:
    source_sha256: 6eb3310550f1b763591abf7ccc9fc7adb856012177a23311229810be53460ba9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/arcane_lock
        implementation: dnd2024:spell_arcane_lock
        parameters:
          spell_lifetime:
            concentration: false
            until: dispelled_or_source_specific_end
          target:
            kinds:
            - door
            - window
            - gate
            - container
            - hatch
            closed: true
            touch: true
          lock:
            magical: true
            nonmagical_unlock: impossible
          authorized_open_close:
            caster: true
            designated_creatures:
              chooser: caster
              timing: cast
          password:
            optional: true
            chooser: caster
            spoken_within_feet: 5
            unlock_seconds: 60
    contract:
      id: dnd2024:spell_arcane_lock
      inputs:
        caster: participant
        cast_level: integer
        target: object
        authorized_creatures: participant_list
        password:
          type: text
          required: false
      outputs:
      - effects
      - objects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - world_objects
      - inventory
    requires:
    - dnd2024:spell_arcane_lock
    citations:
    - SRD 5.2.1 pp. 110; builtin/srd-5.2.1/spell/arcane_lock.md
  dnd2024:spells/augury:
    source_sha256: a3c92f71833956eda745e0f00a29d472ef6cfc3112b90ea7858b51ef16b80be6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/augury
        implementation: dnd2024:spell_augury
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          inquiry:
            kind: planned_course_of_action
            horizon_minutes: 30
          omen:
            chooser: gm
            mapping:
              good: weal
              bad: woe
              good_and_bad: weal_and_woe
              neither: indifference
            account_for_future_changed_circumstances: false
          repetition:
            counter: casts_since_long_rest
            reset_event: rest.long_completed
            first_cast_no_answer_percent: 0
            extra_cast_increment_percent: 25
            maximum_percent: 100
            failure_output: no_answer
    contract:
      id: dnd2024:spell_augury
      inputs:
        caster: participant
        cast_level: integer
        planned_activity: text
        omen_assessment: choice
        campaign: campaign
      outputs:
      - effects
      - information
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - information
      - campaign_time
    requires:
    - dnd2024:spell_augury
    citations:
    - SRD 5.2.1 pp. 111; builtin/srd-5.2.1/spell/augury.md
  dnd2024:spells/banishment:
    source_sha256: 344cf42f5e6e8fde1778ebf034ca40478c7a705bed2103bb039358bee7dea8f7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/banishment
        implementation: dnd2024:spell_banishment
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          targets:
            kind: creature
            visible: true
            range_feet: 30
            count:
              base: 1
              increment_per_slot_above: 1
              base_slot: 4
          save:
            ability: charisma
            dc: spell_save_dc
            success: unaffected
          failure:
            transport: harmless_demiplane
            condition_while_there: incapacitated
          spell_end:
            default:
              return: original_space
              occupied_fallback: nearest_unoccupied_space
            full_minute_exception:
              types:
              - aberration
              - celestial
              - elemental
              - fey
              - fiend
              minimum_elapsed_seconds: 60
              return_to_original_plane: false
              destination_plane:
                chooser: gm
                constraint: associated_with_target_creature_type
              destination_location: random
    contract:
      id: dnd2024:spell_banishment
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        destination_plane_choice:
          type: choice
          required: false
        save_dc: integer
      outputs:
      - effects
      - transports
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - planar_travel
      - conditions
      - tests
      - concentration
    requires:
    - dnd2024:spell_banishment
    citations:
    - SRD 5.2.1 pp. 112; builtin/srd-5.2.1/spell/banishment.md
  dnd2024:spells/bestow_curse:
    source_sha256: 9d6b86d8e3459350a6e463bb3506c8d4ac0aa0648591402acbe70bd63c7479f3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/bestow_curse
        implementation: dnd2024:spell_bestow_curse
        parameters:
          spell_lifetime:
            determined_by: slot_duration
          target:
            kind: creature
            touch: true
          save:
            ability: wisdom
            dc: spell_save_dc
            success: unaffected
          failure:
            apply: curse
            choose_effect: 1
            chooser: caster
          options:
            ability_impairment:
              ability_choice:
              - strength
              - dexterity
              - constitution
              - intelligence
              - wisdom
              - charisma
              disadvantage:
              - chosen_ability_checks
              - chosen_ability_saves
            attack_impairment:
              disadvantage: attack_rolls
              against: caster
            forced_dodge:
              in_combat_only: true
              event: turn.start
              subject: target
              save:
                ability: wisdom
                dc: spell_save_dc
              failure:
                mandatory_action: dodge
                duration: current_turn
            extra_damage:
              event: damage.applied
              source: caster
              target: cursed_target
              eligible_causes:
              - attack_roll
              - spell
              extra_damage:
                dice: 1d8
                type: necrotic
          slot_duration:
          - levels:
            - 3
            seconds: 60
            concentration: true
          - levels:
            - 4
            seconds: 600
            concentration: true
          - levels:
            - 5
            - 6
            seconds: 28800
            concentration: false
          - levels:
            - 7
            - 8
            seconds: 86400
            concentration: false
          - levels:
            - 9
            until: dispelled
            concentration: false
    contract:
      id: dnd2024:spell_bestow_curse
      inputs:
        caster: participant
        cast_level: integer
        target: participant
        curse_choice: choice
        save_dc: integer
      outputs:
      - effects
      - damage
      events:
      - concentration.ended
      - damage.applied
      - spell.cast_completed
      - turn.start
      capabilities:
      - spellcasting
      - effects
      - behavior
      - attacks
      - damage
      - tests
      - concentration
    requires:
    - dnd2024:spell_bestow_curse
    citations:
    - SRD 5.2.1 pp. 112, 113; builtin/srd-5.2.1/spell/bestow_curse.md
  dnd2024:spells/blight:
    source_sha256: db9517b8f9e2fae86dadc9444ffda628b8867f803b1a772b087bc7402bb48e75
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/blight
        implementation: dnd2024:spell_blight
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          modes:
            creature:
              target:
                kind: creature
                visible: true
                range_feet: 30
              save:
                ability: constitution
                dc: spell_save_dc
                automatic_failure_types:
                - plant
              damage:
                dice:
                  base_count: 8
                  sides: 8
                  extra_per_slot: 1
                  base_slot: 4
                type: necrotic
              success_damage_scale:
                numerator: 1
                denominator: 2
            plant:
              target:
                kind: plant_object
                magical: false
                creature: false
                range_feet: 30
              saving_throw: false
              effects:
              - wither
              - die
          target_binding:
            exactly_one:
            - target_creature
            - target_object
            must_match_selected_mode: true
    contract:
      id: dnd2024:spell_blight
      inputs:
        caster: participant
        cast_level: integer
        target_mode: choice
        target_creature:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        save_dc: integer
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
      - tests
    requires:
    - dnd2024:spell_blight
    citations:
    - SRD 5.2.1 pp. 113; builtin/srd-5.2.1/spell/blight.md
  dnd2024:spells/burning_hands:
    source_sha256: 4aac375121ed216206d12236c0c490300b0cb382d9af76df77c255e11387a58e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/burning_hands
        implementation: dnd2024:spell_burning_hands
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          area:
            shape: cone
            length_feet: 15
            origin: caster
          creatures:
            save:
              ability: dexterity
              dc: spell_save_dc
            damage:
              dice:
                base_count: 3
                sides: 6
                extra_per_slot: 1
                base_slot: 1
              type: fire
            success_damage_scale:
              numerator: 1
              denominator: 2
          objects:
            flammable: true
            worn: false
            carried: false
            effect: start_burning
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_burning_hands
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects: object_list
        origin: location
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - damage
      - tests
    requires:
    - dnd2024:spell_burning_hands
    citations:
    - SRD 5.2.1 pp. 114; builtin/srd-5.2.1/spell/burning_hands.md
  dnd2024:spells/charm_monster:
    source_sha256: d8b78682faf925a709857acd2a134fe2ea403f616b5b887c1ed6d47c5d9707de
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/charm_monster
        implementation: dnd2024:spell_charm_monster
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 3600
          targets:
            kind: creature
            visible: true
            range_feet: 30
            count:
              base: 1
              increment_per_slot_above: 1
              base_slot: 4
          save:
            ability: wisdom
            dc: spell_save_dc
            advantage_if: caster_or_allies_fighting_target
            success: unaffected
          failure:
            condition: charmed
            source: caster
            attitude_to_caster: friendly
          end_target:
            event: damage.applied
            source: caster_or_allies
            target: affected_target
          on_effect_end:
            target_knows: caster_charmed_target
    contract:
      id: dnd2024:spell_charm_monster
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
      outputs:
      - effects
      events:
      - damage.applied
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - behavior
      - conditions
      - tests
    requires:
    - dnd2024:spell_charm_monster
    citations:
    - SRD 5.2.1 pp. 114, 115; builtin/srd-5.2.1/spell/charm_monster.md
  dnd2024:spells/circle_of_death:
    source_sha256: a4ee6622eb0e572c6be2634c10379577adc90649b0f09942817c580dcb0bdb70
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/circle_of_death
        implementation: dnd2024:spell_circle_of_death
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          area:
            shape: sphere
            radius_feet: 60
            origin: chosen_point
            range_feet: 150
          targets:
            kind: creature
            selection: all_in_area
          save:
            ability: constitution
            dc: spell_save_dc
          damage:
            dice:
              base_count: 8
              sides: 8
              extra_per_slot: 2
              base_slot: 6
            type: necrotic
          success_damage_scale:
            numerator: 1
            denominator: 2
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_circle_of_death
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        origin: location
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - damage
      - tests
    requires:
    - dnd2024:spell_circle_of_death
    citations:
    - SRD 5.2.1 pp. 115; builtin/srd-5.2.1/spell/circle_of_death.md
  dnd2024:spells/color_spray:
    source_sha256: c003f45cb1a46af208853053a3d42a92901c703986a19adf6e35f337c8a2f5ef
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/color_spray
        implementation: dnd2024:spell_color_spray
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          area:
            shape: cone
            length_feet: 15
            origin: caster
          targets:
            kind: creature
            selection: all_in_area
          save:
            ability: constitution
            dc: spell_save_dc
            success: unaffected
          failure:
            condition: blinded
            until:
              event: turn.end
              subject: caster
              occurrence: next
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_color_spray
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        origin: location
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - areas
      - tests
    requires:
    - dnd2024:spell_color_spray
    citations:
    - SRD 5.2.1 pp. 116; builtin/srd-5.2.1/spell/color_spray.md
  dnd2024:spells/comprehend_languages:
    source_sha256: 99548299f7304a040bdd038709525d6343f2294484ccbdc0d56f249d85c89d06
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/comprehend_languages
        implementation: dnd2024:spell_comprehend_languages
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 3600
          target: caster
          comprehend:
            meaning: literal
            heard_languages: true
            seen_signed_languages: true
            written_language:
              visible: true
              touching_written_surface_required: true
              reading_seconds_per_page_approximate: 60
          decode:
            symbols: false
            secret_messages: false
    contract:
      id: dnd2024:spell_comprehend_languages
      inputs:
        caster: participant
        cast_level: integer
        language_content:
          type: text
          required: false
      outputs:
      - effects
      - information
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - information
    requires:
    - dnd2024:spell_comprehend_languages
    citations:
    - SRD 5.2.1 pp. 117; builtin/srd-5.2.1/spell/comprehend_languages.md
  dnd2024:spells/conjure_animals:
    source_sha256: 49a882803329cdbb8bf545644d97f840f208953710bad2eb3efd299571bd3d8a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/conjure_animals
        implementation: dnd2024:spell_conjure_animals
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 600
          pack:
            size: large
            spectral: true
            tangible: false
            initial_space:
              unoccupied: true
              visible: true
              range_feet: 60
            animal_appearance:
              chooser: caster
          caster_save_benefit:
            ability: strength
            advantage: true
            within_pack_feet: 5
          movement:
            event: participant.moved
            subject: caster
            caster_turn_only: true
            optional: true
            maximum_feet: 30
            destination:
              unoccupied: true
              visible_to_caster: true
          damage_trigger:
            events:
            - pack.moved_within_distance
            - creature.entered_distance
            - turn.end
            distance_feet: 10
            target_kind: creature
            target_visible_to_caster: true
            chooser: caster
            optional: true
            save_limit:
              count: 1
              per: turn
              scope: target
          save:
            ability: dexterity
            dc: spell_save_dc
            success: no_damage
          failure_damage:
            dice:
              base_count: 3
              sides: 10
              extra_per_slot: 1
              base_slot: 3
            type: slashing
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_conjure_animals
      inputs:
        caster: participant
        cast_level: integer
        pack_location: location
        appearance: choice
        targets:
          type: participant_list
          required: false
        movement_choice:
          type: location
          required: false
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - concentration.ended
      - creature.entered_distance
      - pack.moved_within_distance
      - participant.moved
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - movement
      - areas
      - damage
      - tests
      - concentration
    requires:
    - dnd2024:spell_conjure_animals
    citations:
    - SRD 5.2.1 pp. 117, 118; builtin/srd-5.2.1/spell/conjure_animals.md
  dnd2024:spells/conjure_minor_elementals:
    source_sha256: 9ca8cae225e9f477f4f78aacd619b91cbf93053a72a4194e5e8ae930775d36be
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/conjure_minor_elementals
        implementation: dnd2024:spell_conjure_minor_elementals
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 600
          area:
            shape: emanation
            radius_feet: 15
            origin: caster
            moves_with_origin: true
          damage_trigger:
            event: attack.hit
            attacker: caster
            target_kind: creature
            target_in_area: true
          extra_damage:
            dice:
              base_count: 2
              sides: 8
              extra_per_slot: 1
              base_slot: 4
            type:
              chooser: caster
              timing: attack_made
              options:
              - acid
              - cold
              - fire
              - lightning
          terrain:
            scope: ground_in_area
            affected: caster_enemies
            difficult: true
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_conjure_minor_elementals
      inputs:
        caster: participant
        cast_level: integer
        damage_type_choices: choice
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - attack.hit
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - attacks
      - damage
      - concentration
    requires:
    - dnd2024:spell_conjure_minor_elementals
    citations:
    - SRD 5.2.1 pp. 118; builtin/srd-5.2.1/spell/conjure_minor_elementals.md
  dnd2024:spells/contingency:
    source_sha256: ae143d6d1d68c3b11b36d7b29e22ea5e5e47194c975829874c988422a1400dab
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/contingency
        implementation: dnd2024:spell_contingency
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 864000
          contingent_spell:
            chooser: caster
            maximum_level: 5
            castable_by_caster: true
            casting_time: action
            can_target_caster: true
            cast_as_part_of_contingency: true
            pay_spell_slots:
            - contingency
            - contingent_spell
            initial_effect: delayed
          trigger:
            chooser: caster
            timing: cast
            kind: declared_circumstance
            resolution_input: trigger_occurrence
            activate: immediately_after_first_occurrence
            optional_at_activation: false
          activation:
            contingent_targets: caster_only
            ignore_normal_additional_targets: true
            end_contingency: true
          end_conditions:
          - duration_elapsed
          - caster_casts_contingency_again
          - material_component_not_on_caster_person
          maximum_active_per_caster: 1
    contract:
      id: dnd2024:spell_contingency
      inputs:
        caster: participant
        cast_level: integer
        contingent_spell: definition
        contingent_spell_level: integer
        declared_trigger: choice
        component: item_instance
      outputs:
      - effects
      events:
      - declared_trigger.occurred
      - item.holder_changed
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - magic_suppression
      - inventory
    requires:
    - dnd2024:spell_contingency
    citations:
    - SRD 5.2.1 pp. 119; builtin/srd-5.2.1/spell/contingency.md
  dnd2024:spells/counterspell:
    source_sha256: 3dcf6fba5eba512f4e3352846610f6615314c4d5b8d1da50c2174128aa7712bc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/counterspell
        implementation: dnd2024:spell_counterspell
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          trigger:
            event: spell.cast_pending
            creature_visible: true
            distance_feet_max: 60
            component_any:
            - verbal
            - somatic
            - material
          target: triggering_caster
          save:
            ability: constitution
            dc: spell_save_dc
            success: casting_continues
          failure:
            spell_effect: dissipate_without_effect
            casting_economy: wasted
            applicable_economy:
            - action
            - bonus_action
            - reaction
            slot_cast:
              spell_slot: not_expended
    contract:
      id: dnd2024:spell_counterspell
      inputs:
        caster: participant
        cast_level: integer
        target: participant
        triggering_cast: resolution
        save_dc: integer
      outputs:
      - save_result
      - casting_outcome
      - cost_disposition
      events:
      - save.completed
      - spell.cast_pending
      capabilities:
      - spellcasting
      - effects
      - magic_suppression
      - tests
    requires:
    - dnd2024:spell_counterspell
    citations:
    - SRD 5.2.1 pp. 120; builtin/srd-5.2.1/spell/counterspell.md
  dnd2024:spells/creation:
    source_sha256: 3f6ce2ab754a76e476ac02aaf37fff881619f4451be8f8a65901169c10c64c01
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/creation
        implementation: dnd2024:spell_creation
        parameters:
          spell_lifetime:
            concentration: false
            determined_by: material_duration_table
          object:
            count: 1
            range_feet: 30
            permitted_matter:
            - vegetable
            - mineral
            form_seen_before: true
            material_seen_before: true
            maximum_cube_side_feet:
              base: 5
              increment_per_slot: 5
              base_slot: 5
          duration_by_material_seconds:
            vegetable: 86400
            stone: 43200
            crystal: 43200
            precious_metal: 3600
            gems: 600
            adamantine: 60
            mithral: 60
          mixed_material_duration: minimum_component_duration
          unlisted_mineral_duration:
            kind: unresolved_source_case
            required_input: material_duration_resolution
            no_automatic_default: true
          used_as_other_spell_material:
            effect: other_spell_fails
    contract:
      id: dnd2024:spell_creation
      inputs:
        caster: participant
        cast_level: integer
        object_design: choice
        origin: location
        material_duration_resolution:
          type: duration
          required: false
      outputs:
      - effects
      - objects
      - unresolved_material_duration
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - world_objects
      - campaign_time
    requires:
    - dnd2024:spell_creation
    citations:
    - SRD 5.2.1 pp. 121; builtin/srd-5.2.1/spell/creation.md
  dnd2024:spells/darkvision:
    source_sha256: 2369137ebba9eb1f9e875cec4138db503c3ead80774d5df1715ed7949e526cef
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/darkvision
        implementation: dnd2024:spell_darkvision
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 28800
          target:
            kind: creature
            willing: true
            touch: true
          sense:
            kind: darkvision
            range_feet: 150
    contract:
      id: dnd2024:spell_darkvision
      inputs:
        caster: participant
        cast_level: integer
        target: participant
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_darkvision
    citations:
    - SRD 5.2.1 pp. 122; builtin/srd-5.2.1/spell/darkvision.md
  dnd2024:spells/demiplane:
    source_sha256: 00e3ab0b1b3f1108d2e83192c98e8fc898c0d8c3eec6ba4167553cb553a4ffd7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/demiplane
        implementation: dnd2024:spell_demiplane
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 3600
          doorway:
            size: medium
            range_feet: 60
            surface:
              flat: true
              solid: true
              visible: true
            can_open_close: true
          destination_choices:
            chooser: caster
            options:
            - new_demiplane
            - caster_previous_demiplane
            - other_caster_demiplane
            other_caster_requirement: know_nature_and_contents
          new_room:
            empty: true
            length_feet: 30
            width_feet: 30
            height_feet: 30
            material:
              chooser: caster
              options:
              - wood
              - stone
          end:
            door_vanishes: true
            objects_remain: true
            creatures:
              chooser: each_creature
              options:
              - remain
              - exit
              exit:
                destination: nearest_unoccupied_space_to_former_door
                condition: prone
    contract:
      id: dnd2024:spell_demiplane
      inputs:
        caster: participant
        cast_level: integer
        door_location: location
        destination: choice
        room_material:
          type: choice
          required: false
        exit_choices:
          type: choice
          required: false
      outputs:
      - effects
      - transports
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - planar_travel
    requires:
    - dnd2024:spell_demiplane
    citations:
    - SRD 5.2.1 pp. 122, 123; builtin/srd-5.2.1/spell/demiplane.md
  dnd2024:spells/detect_thoughts:
    source_sha256: 54c16518a459daabd03af62c62cc2b02af0b249783bdd80e03478e8d0c99ed15
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/detect_thoughts
        implementation: dnd2024:spell_detect_thoughts
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          activation:
            at_cast: choose_one_option
            later:
              economy: magic_action
              timing: caster_later_turn
              options:
              - sense_thoughts
              - read_thoughts
          sense_thoughts:
            range_feet: 30
            eligible:
              any:
              - knows_language
              - telepathic
            output: thinking_creature_presence
            read_content: false
          read_thoughts:
            targets: 1
            range_feet: 30
            eligible:
              any:
              - visible
              - detected_with_sense_thoughts
            output: foremost_current_thought
            no_language_and_not_telepathic_output: nothing
          probe:
            economy: magic_action
            timing: next_caster_turn_after_read
            target: read_target
            save:
              ability: wisdom
              dc: spell_save_dc
            failure_output:
            - reasoning
            - emotions
            - prominent_concern
            success: end_spell
            either_outcome: target_knows_probe
          resistance_after_probe:
            until: attention_shifts_to_other_target
            economy: action
            timing: target_turn
            check:
              ability: intelligence
              skill: arcana
              dc: spell_save_dc
            success: end_spell
          barriers:
          - material: stone
            thickness_inches: 12
          - material: dirt
            thickness_inches: 12
          - material: wood
            thickness_inches: 12
          - material: metal
            thickness_inches: 1
          - material: lead
            thickness: any_sheet
          barrier_scope: spell
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_detect_thoughts
      inputs:
        caster: participant
        cast_level: integer
        option: choice
        target:
          type: participant
          required: false
        thought_content:
          type: choice
          required: false
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - information
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - information
      - tests
      - concentration
    requires:
    - dnd2024:spell_detect_thoughts
    citations:
    - SRD 5.2.1 pp. 123; builtin/srd-5.2.1/spell/detect_thoughts.md
  dnd2024:spells/dispel_evil_and_good:
    source_sha256: 7a6a2bc6dd1186f2b62cb4b3aa7ac767fac54ce33ff08b142944539a07902af8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/dispel_evil_and_good
        implementation: dnd2024:spell_dispel_evil_and_good
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          creature_types:
          - celestial
          - elemental
          - fey
          - fiend
          - undead
          protection:
            disadvantage: attack_rolls
            attacker_types: configured_types
            target: caster
          special_functions:
            economy: magic_action
            end_spell_on_use: true
            choose_one:
            - break_enchantment
            - dismissal
          break_enchantment:
            target:
              kind: creature
              touch: true
            remove_effects_from_configured_types:
            - possession
            - charmed
            - frightened
          dismissal:
            target:
              kind: creature
              visible: true
              range_feet: 5
              types: configured_types
            save:
              ability: charisma
              dc: spell_save_dc
            failure:
              if_not_on_home_plane:
                transport_to: home_plane
                overrides:
                  undead: shadowfell
                  fey: feywild
              if_on_home_plane: unaffected
    contract:
      id: dnd2024:spell_dispel_evil_and_good
      inputs:
        caster: participant
        cast_level: integer
        option:
          type: choice
          required: false
        target:
          type: participant
          required: false
        save_dc: integer
      outputs:
      - effects
      - transports
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - planar_travel
      - conditions
      - tests
      - concentration
    requires:
    - dnd2024:spell_dispel_evil_and_good
    citations:
    - SRD 5.2.1 pp. 124; builtin/srd-5.2.1/spell/dispel_evil_and_good.md
  dnd2024:spells/divine_favor:
    source_sha256: a154f964c30a5cc20e4cbcf0bb39e3491591814bbba67c05c55ec84dec4a6949
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/divine_favor
        implementation: dnd2024:spell_divine_favor
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 60
          trigger:
            event: attack.hit
            attacker: caster
            weapon_required: true
          extra_damage:
            dice: 1d4
            type: radiant
    contract:
      id: dnd2024:spell_divine_favor
      inputs:
        caster: participant
        cast_level: integer
      outputs:
      - effects
      - damage
      events:
      - attack.hit
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - attacks
      - damage
    requires:
    - dnd2024:spell_divine_favor
    citations:
    - SRD 5.2.1 pp. 125; builtin/srd-5.2.1/spell/divine_favor.md
  dnd2024:spells/dominate_monster:
    source_sha256: 4f31c031107f32790f2c99650f9cd5aa924bdb30214adb56cb20384aaaaf7dee
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/dominate_monster
        implementation: dnd2024:spell_dominate_monster
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 3600
          target:
            kind: creature
            visible: true
            range_feet: 60
          initial_save:
            ability: wisdom
            dc: spell_save_dc
            advantage_if: caster_or_allies_fighting_target
            success: unaffected
          failure:
            condition: charmed
            source: caster
          repeat_save:
            event: damage.applied
            subject: target
            ability: wisdom
            dc: spell_save_dc
            success: end_on_target
            inherits_initial_save_advantage_rule: true
          telepathic_link:
            requires: same_plane
            while: charmed_by_spell
          commands:
            economy: none
            timing: caster_turn
            medium: telepathic_link
            compliance: best_effort_on_target_turn
            after_completion_without_new_order:
              autonomous: true
              focus: self_protection
          command_reaction:
            caster_economy: reaction
            target_economy: reaction
          slot_duration:
          - levels:
            - 8
            seconds: 3600
          - levels:
            - 9
            seconds: 28800
    contract:
      id: dnd2024:spell_dominate_monster
      inputs:
        caster: participant
        cast_level: integer
        target: participant
        commands:
          type: choice
          required: false
        save_dc: integer
      outputs:
      - effects
      events:
      - concentration.ended
      - damage.applied
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - behavior
      - conditions
      - tests
      - concentration
    requires:
    - dnd2024:spell_dominate_monster
    citations:
    - SRD 5.2.1 pp. 125, 126; builtin/srd-5.2.1/spell/dominate_monster.md
  dnd2024:spells/druidcraft:
    source_sha256: edb16769c81f1ff41c60bf7e6add71aa51345062a2cdee2bf923b5636de19988
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/druidcraft
        implementation: dnd2024:spell_druidcraft
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          choice:
            chooser: caster
            count: 1
            range_feet: 30
          options:
            weather_sensor:
              size: tiny
              harmless: true
              forecast_location: caster_location
              forecast_hours: 24
              duration_rounds: 1
              sensory_manifestation:
                chooser: caster
            bloom:
              instant: true
              target_choices:
              - flower
              - seed_pod
              - leaf_bud
              operations:
                flower: blossom
                seed_pod: open
                leaf_bud: bloom
            sensory_effect:
              harmless: true
              maximum_cube_side_feet: 5
              sensory_design:
                chooser: caster
            fire_play:
              operations:
              - light
              - snuff
              targets:
              - candle
              - torch
              - campfire
    contract:
      id: dnd2024:spell_druidcraft
      inputs:
        caster: participant
        cast_level: integer
        option: choice
        origin: location
        design: choice
      outputs:
      - effects
      - objects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - illusions
      - world_objects
    requires:
    - dnd2024:spell_druidcraft
    citations:
    - SRD 5.2.1 pp. 126; builtin/srd-5.2.1/spell/druidcraft.md
  dnd2024:spells/enhance_ability:
    source_sha256: 38999065e20035450e58afdd63322a311f31d2f2976f3db6af234b213659dcb1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/enhance_ability
        implementation: dnd2024:spell_enhance_ability
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 3600
          targets:
            kind: creature
            touch: true
            count:
              base: 1
              increment_per_slot_above: 1
              base_slot: 2
          ability:
            chooser: caster
            per_target: true
            options:
            - strength
            - dexterity
            - intelligence
            - wisdom
            - charisma
          modifier:
            selector: ability_checks_using_chosen_ability
            advantage: true
    contract:
      id: dnd2024:spell_enhance_ability
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        ability_choices: choice
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - concentration
    requires:
    - dnd2024:spell_enhance_ability
    citations:
    - SRD 5.2.1 pp. 127; builtin/srd-5.2.1/spell/enhance_ability.md
  dnd2024:spells/enthrall:
    source_sha256: c906942e4c1a78df22d77b140fadc724ee6662c9a5af3777eb52e8bf352f9999
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/enthrall
        implementation: dnd2024:spell_enthrall
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          targets:
            kind: creature
            visible: true
            range_feet: 60
            chooser: caster
            count: any
          save:
            ability: wisdom
            dc: spell_save_dc
            automatic_success_if: caster_or_companions_fighting_target
          failure:
            modifiers:
            - selector: wisdom_perception_checks
              add: -10
            - selector: passive_perception
              add: -10
    contract:
      id: dnd2024:spell_enthrall
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - tests
      - concentration
    requires:
    - dnd2024:spell_enthrall
    citations:
    - SRD 5.2.1 pp. 128; builtin/srd-5.2.1/spell/enthrall.md
  dnd2024:spells/fabricate:
    source_sha256: 5e142bad40db19200756c38e54020dfe778e3eb4f38279e24442619351257edb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/fabricate
        implementation: dnd2024:spell_fabricate
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          inputs:
            kind: raw_materials
            visible: true
            range_feet: 120
            sufficient_quantity_required: true
          output:
            kind: object
            material: same_as_inputs
            quality: based_on_input_quality
            maximum_size: large
            geometry_choice:
            - cube_side_feet: 10
              count: 1
            - cube_side_feet: 5
              count: 8
              connected: true
          mineral_exception:
            materials:
            - metal
            - stone
            - other_mineral
            maximum_size: medium
            maximum_cube_side_feet: 5
          forbidden_output:
          - creature
          - magic_item
          skilled_craft:
            examples:
            - weapon
            - armor
            requires_proficiency: corresponding_artisans_tools
          supplied_facts:
            material_sufficiency: boolean
            input_quality: choice
            requires_high_skill: boolean
            corresponding_tools: definition
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_fabricate
      inputs:
        caster: participant
        cast_level: integer
        materials: object_list
        product_design: choice
        material_facts: resolution
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - objects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - world_objects
      - inventory
    requires:
    - dnd2024:spell_fabricate
    citations:
    - SRD 5.2.1 pp. 129; builtin/srd-5.2.1/spell/fabricate.md
  dnd2024:spells/fear:
    source_sha256: 5eae32e41bb82a76b06d4adb41446721f680769761e6c210a384cf1494a65e3b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/fear
        implementation: dnd2024:spell_fear
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          area:
            shape: cone
            length_feet: 30
            origin: caster
          targets:
            kind: creature
            selection: all_in_area
          save:
            ability: wisdom
            dc: spell_save_dc
            success: unaffected
          failure:
            drop: all_held_items
            condition: frightened
            source: caster
          while_frightened:
            timing: target_turn
            action: dash
            movement: away_from_caster
            route: safest
            exception: nowhere_to_move
            route_assumption: supplied_spatial_paths
          repeat_save:
            event: turn.end
            subject: target
            requires: no_line_of_sight_to_caster
            ability: wisdom
            dc: spell_save_dc
            success: end_on_target
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_fear
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        origin: location
        safe_paths: resolution
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - movement
      - behavior
      - conditions
      - tests
      - concentration
    requires:
    - dnd2024:spell_fear
    citations:
    - SRD 5.2.1 pp. 130; builtin/srd-5.2.1/spell/fear.md
  dnd2024:spells/find_the_path:
    source_sha256: ab049d08cf1e908ea9a6f64ca7835bbfa0775e6c2952c98fe9ee5e4ac3735e05
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/find_the_path
        implementation: dnd2024:spell_find_the_path
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 86400
          destination:
            chooser: caster
            named: true
            familiar: true
            must_be_specific: true
            must_be_stationary: true
            same_plane_at_cast: true
            violation: spell_fails
          output: most_direct_physical_route
          ongoing_information:
            requires: same_plane_as_destination
            outputs:
            - distance
            - direction
            at_path_choice: most_direct_path
          supplied_world_facts:
          - physical_route_network
          - destination_familiarity
          - destination_motion
          - destination_plane
    contract:
      id: dnd2024:spell_find_the_path
      inputs:
        caster: participant
        cast_level: integer
        destination: location
        route_facts: resolution
      outputs:
      - effects
      - information
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - information
      - concentration
    requires:
    - dnd2024:spell_find_the_path
    citations:
    - SRD 5.2.1 pp. 131; builtin/srd-5.2.1/spell/find_the_path.md
  dnd2024:spells/fire_shield:
    source_sha256: 8340c09555d3495a3496abcc80501543b06d76e976cb613dd85d740085eeb317
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/fire_shield
        implementation: dnd2024:spell_fire_shield
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 600
          target: caster
          light:
            bright_radius_feet: 10
            additional_dim_feet: 10
          choice:
            chooser: caster
            options:
            - warm
            - chill
          warm:
            resistance: cold
            retaliatory_damage:
              dice: 2d8
              type: fire
          chill:
            resistance: fire
            retaliatory_damage:
              dice: 2d8
              type: cold
          retaliation:
            event: attack.hit
            attack_kind: melee_attack_roll
            target: caster
            attacker_kind: creature
            attacker_within_feet: 5
            damage_recipient: attacker
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_fire_shield
      inputs:
        caster: participant
        cast_level: integer
        shield_choice: choice
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - attack.hit
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - attacks
      - damage
    requires:
    - dnd2024:spell_fire_shield
    citations:
    - SRD 5.2.1 pp. 132; builtin/srd-5.2.1/spell/fire_shield.md
  dnd2024:spells/flame_strike:
    source_sha256: b4483ca79d6e40e3854c07d2f110bb4d82c2e677e24258de42a5e0d1bdfb768e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/flame_strike
        implementation: dnd2024:spell_flame_strike
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          area:
            shape: cylinder
            radius_feet: 10
            height_feet: 40
            origin: chosen_point
            range_feet: 60
          targets:
            kind: creature
            selection: all_in_area
          save:
            ability: dexterity
            dc: spell_save_dc
          damage:
          - dice:
              base_count: 5
              sides: 6
              extra_per_slot: 1
              base_slot: 5
            type: fire
          - dice:
              base_count: 5
              sides: 6
              extra_per_slot: 1
              base_slot: 5
            type: radiant
          success_damage_scale:
            numerator: 1
            denominator: 2
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_flame_strike
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        origin: location
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - damage
      - tests
    requires:
    - dnd2024:spell_flame_strike
    citations:
    - SRD 5.2.1 pp. 132; builtin/srd-5.2.1/spell/flame_strike.md
  dnd2024:spells/fly:
    source_sha256: 0494eb3e712ea08be00f66623b7ac64bb163ce781f38f56e2591d236614e86f3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/fly
        implementation: dnd2024:spell_fly
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 600
          targets:
            kind: creature
            willing: true
            touch: true
            count:
              base: 1
              increment_per_slot_above: 1
              base_slot: 3
          grant:
            fly_speed_feet: 60
            hover: true
          on_end:
            if: target_aloft
            effect: fall
            exception: target_can_stop_fall
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_fly
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - movement
      - concentration
    requires:
    - dnd2024:spell_fly
    citations:
    - SRD 5.2.1 pp. 133; builtin/srd-5.2.1/spell/fly.md
  dnd2024:spells/foresight:
    source_sha256: 029eb9b7bc4e018a834289911621724f60ae99dd03620092015bc37812ada286
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/foresight
        implementation: dnd2024:spell_foresight
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 28800
          target:
            kind: creature
            willing: true
            touch: true
          modifiers:
          - selector: target_d20_tests
            advantage: true
          - selector: other_creatures_attack_rolls_against_target
            disadvantage: true
          end_early:
            event: spell.cast_completed
            subject: caster
            spell:
              key: dnd2024:spells/foresight
    contract:
      id: dnd2024:spell_foresight
      inputs:
        caster: participant
        cast_level: integer
        target: participant
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_foresight
    citations:
    - SRD 5.2.1 pp. 134; builtin/srd-5.2.1/spell/foresight.md
  dnd2024:spells/gate:
    source_sha256: 512b60e01ba6247a30c1d501902682d4379d17f216765aab9cd6cbd959121bc6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/gate
        implementation: dnd2024:spell_gate
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          portal:
            origin:
              unoccupied: true
              visible: true
              range_feet: 60
            destination:
              precise_location: true
              different_plane: true
            shape: circle
            diameter_feet:
              minimum: 5
              maximum: 20
              chooser: caster
            orientation:
              chooser: caster
            destination_visible_through_portal: true
            faces:
            - front
            - back
          traversal:
            entry_face: front_only
            entity: anything
            timing: instantaneous
            destination: nearest_unoccupied_space_to_other_portal
          planar_ruler_veto:
            actors:
            - deity
            - planar_ruler
            locations:
            - in_presence
            - within_domain
            choice: prevent_portal_opening
          named_creature_option:
            timing: cast
            name_kind: actual_name
            forbidden_name_kinds:
            - pseudonym
            - title
            - nickname
            requires: target_on_different_plane
            portal_destination: next_to_named_creature
            forced_transport: nearest_unoccupied_space_on_caster_side
            control_granted: false
            subsequent_behavior:
              chooser: gm
              actor: summoned_creature
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_gate
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        destination: location
        orientation: choice
        diameter_feet: number
        creature_name:
          type: text
          required: false
        ruler_veto:
          type: choice
          required: false
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - transports
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - planar_travel
      - areas
      - concentration
    requires:
    - dnd2024:spell_gate
    citations:
    - SRD 5.2.1 pp. 135; builtin/srd-5.2.1/spell/gate.md
  dnd2024:spells/glibness:
    source_sha256: c231c516191a5fc39541c0875600c3d8a4937cc33f65517a04880d43cf2b08a4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/glibness
        implementation: dnd2024:spell_glibness
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 3600
          target: caster
          roll_replacement:
            event: check.rolled
            ability: charisma
            optional: true
            chooser: caster
            replace: die_result
            value: 15
          truth_detection:
            source_kind: magic
            actual_statement_truth: any
            reported_truth: truthful
    contract:
      id: dnd2024:spell_glibness
      inputs:
        caster: participant
        cast_level: integer
        replace_roll:
          type: boolean
          required: false
      outputs:
      - effects
      - information
      events:
      - check.rolled
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - information
    requires:
    - dnd2024:spell_glibness
    citations:
    - SRD 5.2.1 pp. 136; builtin/srd-5.2.1/spell/glibness.md
  dnd2024:spells/grease:
    source_sha256: 7e46012603ed75212a8bda624ee1fb9d97ff381846777fb92be674ae09267864
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/grease
        implementation: dnd2024:spell_grease
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 60
          area:
            shape: square
            side_feet: 10
            surface: ground
            origin: chosen_point
            range_feet: 60
          properties:
            flammable: false
            difficult_terrain: true
          saving_throw_triggers:
          - event: spell.cast_completed
            subjects: creatures_standing_in_area
          - event: creature.entered_area
          - event: turn.end
            subjects: creatures_in_area
          save:
            ability: dexterity
            dc: spell_save_dc
            failure_condition: prone
            success: unaffected
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_grease
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        targets: participant_list
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - creature.entered_area
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - movement
      - areas
      - tests
    requires:
    - dnd2024:spell_grease
    citations:
    - SRD 5.2.1 pp. 137; builtin/srd-5.2.1/spell/grease.md
  dnd2024:spells/guards_and_wards:
    source_sha256: 5dd640084502340836da37b062fd4b189a2267985d9152fe912bd34348dcc47e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/guards_and_wards
        implementation: dnd2024:spell_guards_and_wards
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 86400
          area:
            maximum_square_feet: 2500
            maximum_height_feet: 20
            shape_choices:
            - squares: 1
              side_feet: 50
            - squares: 100
              side_feet: 5
              contiguous: true
            - squares: 25
              side_feet: 10
              contiguous: true
          immunity:
            designated_individuals:
              chooser: caster
              timing: cast
            password:
              chooser: caster
              spoken_aloud: true
              within_area_feet: 5
              grant_to: speaker
          dispelling:
            whole_spell_targetable_by_dispel_magic: false
            independently_dispellable_effect_groups:
            - corridors
            - doors
            - stairs
            - other_spell_effect
            all_four_removed: end_spell
          permanence:
            repeat_casts: 365
            frequency: daily
            same_area: true
            result_duration: until_all_effect_groups_dispelled
          corridors:
            scope: all_warded_corridors
            fog_obscurement: heavy
            misdirection:
              at: intersection_or_branch_with_direction_choice
              exclude_caster: true
              probability_percent: 50
              perceived_direction: opposite_chosen_direction
          doors:
            scope: all_warded_doors
            lock_spell:
              key: dnd2024:spells/arcane_lock
            concealed_doors:
              chooser: caster
              maximum: 10
              appearance: plain_wall
          stairs:
            scope: all_warded_stairs_top_to_bottom
            effect_spell:
              key: dnd2024:spells/web
            destroyed_web_regrowth_seconds: 600
            regrowth_requires_spell_active: true
          other_spell_effect:
            choose_count: 1
            chooser: caster
            options:
              dancing_lights:
                spell:
                  key: dnd2024:spells/dancing_lights
                locations: 4
                location_kind: corridor
                behavior: repeated_simple_program
                duration: guards_and_wards
              magic_mouth:
                spell:
                  key: dnd2024:spells/magic_mouth
                locations: 2
              stinking_cloud:
                spell:
                  key: dnd2024:spells/stinking_cloud
                locations: 2
                dispersed_vapor_return_seconds_max: 600
                return_requires_spell_active: true
              gust_of_wind:
                spell:
                  key: dnd2024:spells/gust_of_wind
                locations: 1
                location_kinds:
                - corridor
                - room
                wind: continuous_for_duration
              suggestion:
                spell:
                  key: dnd2024:spells/suggestion
                square_side_feet: 5
                event: creature.entered_area
                delivery: mental
          nested_spell_invocation:
            additional_casting_actions: false
            additional_spell_slots: false
            concentration_owner: guards_and_wards_lifetime
            parent_exemptions_apply: true
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_guards_and_wards
      inputs:
        caster: participant
        cast_level: integer
        area: location_list
        exempt_creatures: participant_list
        password:
          type: text
          required: false
        spell_options: choice
        campaign: campaign
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - objects
      events:
      - creature.entered_area
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - illusions
      - world_objects
      - areas
      - campaign_time
    requires:
    - dnd2024:spell_guards_and_wards
    citations:
    - SRD 5.2.1 pp. 137, 138; builtin/srd-5.2.1/spell/guards_and_wards.md
  dnd2024:spells/hallow:
    source_sha256: f44a57426f1d0f4137ee38efe049d4f559a2fcd4bd55a3c2ca95d43038c3ab4d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/hallow
        implementation: dnd2024:spell_hallow
        parameters:
          spell_lifetime:
            concentration: false
            until: dispelled_or_source_specific_end
          area:
            origin: touched_point
            radius_feet_max: 60
            overlap_existing_hallow: spell_fails
          ward:
            chooser: caster
            type_selection: any_subset
            type_options:
            - aberration
            - celestial
            - elemental
            - fey
            - fiend
            - undead
            selected_types:
              willing_entry: forbidden
            occupants:
              suppress_effects_from_selected_types:
              - possession
              - charmed
              - frightened
              suppression_scope: while_in_area
          extra_effect:
            chooser: caster
            count: 1
          options:
            courage:
              creature_types: caster_choice
              while_in_area:
                gain_frightened: forbidden
            darkness:
              light_level: darkness
              normal_light_illuminates: false
              magical_light_illuminates:
                spell_level_less_than_cast_level: false
            daylight:
              light_level: bright
              magical_darkness_extinguishes:
                spell_level_less_than_cast_level: false
            peaceful_rest:
              subjects: dead_bodies_interred_in_area
              transform_into_undead: forbidden
            extradimensional_interference:
              creature_types: caster_choice
              forbidden_travel:
              - teleport_enter
              - teleport_exit
              - interplanar_enter
              - interplanar_exit
            fear:
              creature_types: caster_choice
              while_in_area:
                condition: frightened
            resistance:
              creature_types: caster_choice
              damage_type:
                chooser: caster
                count: 1
              while_in_area:
                resistance: chosen_damage_type
            silence:
              sound_from_inside: blocked
              sound_entering: blocked
            tongues:
              creature_types: caster_choice
              communication_with: any_creature_in_area
              shared_language_required: false
            vulnerability:
              creature_types: caster_choice
              damage_type:
                chooser: caster
                count: 1
              while_in_area:
                vulnerability: chosen_damage_type
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_hallow
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        radius_feet: number
        ward_types: choice
        extra_effect: choice
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - magic_suppression
      - conditions
    requires:
    - dnd2024:spell_hallow
    citations:
    - SRD 5.2.1 pp. 138, 139; builtin/srd-5.2.1/spell/hallow.md
  dnd2024:spells/heal:
    source_sha256: 906bd05641cdbe8f1391f264fd9041e5c80da7cc39f5433a73aa6e8b79bb5047
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/heal
        implementation: dnd2024:spell_heal
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          target:
            kind: creature
            visible: true
            range_feet: 60
          healing:
            base: 70
            increment_per_slot: 10
            base_slot: 6
          end_conditions:
          - blinded
          - deafened
          - poisoned
    contract:
      id: dnd2024:spell_heal
      inputs:
        caster: participant
        cast_level: integer
        target: participant
      outputs:
      - effects
      - healing
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - conditions
      - healing
    requires:
    - dnd2024:spell_heal
    citations:
    - SRD 5.2.1 pp. 139; builtin/srd-5.2.1/spell/heal.md
  dnd2024:spells/heroes_feast:
    source_sha256: 26fa71dec8abb987aaf54561b5a3cab3c2b2b4f017d4446769812a1c3c6bfc96
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/heroes_feast
        implementation: dnd2024:spell_heroes_feast
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          feast:
            appears_on: surface
            space:
              unoccupied: true
              cube_side_feet: 10
              adjacent_to_caster: true
            consume_seconds: 3600
            disappears_after_seconds: 3600
            maximum_participants: 12
          benefits_start: consumption_hour_completed
          benefits:
            duration_seconds: 86400
            damage_resistance: poison
            condition_immunities:
            - frightened
            - poisoned
            hit_point_maximum_increase:
              dice: 2d10
            hit_points_gained: same_as_maximum_increase
    contract:
      id: dnd2024:spell_heroes_feast
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        participants: participant_list
      outputs:
      - effects
      - healing
      - objects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - world_objects
      - conditions
      - healing
      - campaign_time
    requires:
    - dnd2024:spell_heroes_feast
    citations:
    - SRD 5.2.1 pp. 140; builtin/srd-5.2.1/spell/heroes_feast.md
  dnd2024:spells/hold_monster:
    source_sha256: e06eb2904e6bac7be6baa710a552233cc17d05121a72a7d5a4ab4ab28b74f318
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/hold_monster
        implementation: dnd2024:spell_hold_monster
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          targets:
            kind: creature
            visible: true
            range_feet: 90
            count:
              base: 1
              increment_per_slot_above: 1
              base_slot: 5
          save:
            ability: wisdom
            dc: spell_save_dc
            success: unaffected
            failure_condition: paralyzed
          repeat_save:
            event: turn.end
            subject: target
            ability: wisdom
            dc: spell_save_dc
            success: end_on_target
    contract:
      id: dnd2024:spell_hold_monster
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - conditions
      - tests
      - concentration
    requires:
    - dnd2024:spell_hold_monster
    citations:
    - SRD 5.2.1 pp. 140, 141; builtin/srd-5.2.1/spell/hold_monster.md
  dnd2024:spells/hypnotic_pattern:
    source_sha256: 189de898b0ff63acba041496f8500bc3faf877696c0e47e0e0b83133665bd1ed
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/hypnotic_pattern
        implementation: dnd2024:spell_hypnotic_pattern
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          area:
            shape: cube
            side_feet: 30
            range_feet: 120
          pattern:
            appears: momentarily
            then: vanishes
          targets:
            kind: creature
            in_area: true
            sees_pattern: true
          save:
            ability: wisdom
            dc: spell_save_dc
            success: unaffected
          failure:
            condition: charmed
            while_charmed:
              condition: incapacitated
              speed_feet: 0
          end_target_triggers:
          - event: damage.applied
            subject: target
          - event: action.completed
            actor: another_creature
            action: shake_target_awake
            economy: action
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_hypnotic_pattern
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        targets: participant_list
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - action.completed
      - concentration.ended
      - damage.applied
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - conditions
      - tests
      - concentration
    requires:
    - dnd2024:spell_hypnotic_pattern
    citations:
    - SRD 5.2.1 pp. 141; builtin/srd-5.2.1/spell/hypnotic_pattern.md
  dnd2024:spells/illusory_script:
    source_sha256: 3c62118f460c47001a4c27cbee6059f592d2ea0931f1f6e9f8771528d823f9f9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/illusory_script
        implementation: dnd2024:spell_illusory_script
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 864000
          target:
            kind: writing_surface
            suitable_for_writing: true
            touch: true
          message:
            real_meaning: caster_authored
            real_handwriting: caster
          authorized_readers:
            caster: true
            additional_creatures:
              chooser: caster
              timing: cast
          unauthorized_display_choice:
            chooser: caster
            options:
            - unintelligible_script
            - alternative_text
          unintelligible_script:
            appearance_choices:
            - unknown_script
            - magical_script
            readable: false
          alternative_text:
            meaning: caster_choice
            handwriting: caster_choice
            language:
              chooser: caster
              constraint: known_to_caster
          truesight:
            can_read_hidden_message: true
          on_dispel:
            remove:
            - original_script
            - illusion
    contract:
      id: dnd2024:spell_illusory_script
      inputs:
        caster: participant
        cast_level: integer
        surface: object
        authorized_readers: participant_list
        real_message: text
        illusory_display: choice
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - illusions
      - memory
      - inventory
    requires:
    - dnd2024:spell_illusory_script
    citations:
    - SRD 5.2.1 pp. 142; builtin/srd-5.2.1/spell/illusory_script.md
  dnd2024:spells/insect_plague:
    source_sha256: 26a053486889af342244c3755c3fbcfbc478ff5673cbd48334a626bf368bc65d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/insect_plague
        implementation: dnd2024:spell_insect_plague
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 600
          area:
            shape: sphere
            radius_feet: 20
            origin: chosen_point
            range_feet: 300
            stationary: true
          properties:
            obscurement: light
            difficult_terrain: true
          triggers:
          - event: spell.cast_completed
            subjects: creatures_in_area
          - event: creature.entered_area
            occurrence_per_turn: first
          - event: turn.end
            subjects: creatures_in_area
          save_limit:
            count: 1
            per: turn
            scope: target
          save:
            ability: constitution
            dc: spell_save_dc
          damage:
            dice:
              base_count: 4
              sides: 10
              extra_per_slot: 1
              base_slot: 5
            type: piercing
          success_damage_scale:
            numerator: 1
            denominator: 2
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_insect_plague
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        targets: participant_list
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - concentration.ended
      - creature.entered_area
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - areas
      - damage
      - tests
      - concentration
    requires:
    - dnd2024:spell_insect_plague
    citations:
    - SRD 5.2.1 pp. 143; builtin/srd-5.2.1/spell/insect_plague.md
  dnd2024:spells/jump:
    source_sha256: 37c60c354eee3464e8e432a580e05ff386f7939ec87b1425fbb517e10be7ce75
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/jump
        implementation: dnd2024:spell_jump
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 60
          targets:
            kind: creature
            willing: true
            touch: true
            count:
              base: 1
              increment_per_slot_above: 1
              base_slot: 1
          jump_grant:
            timing: target_turn
            count_per_turn: 1
            maximum_distance_feet: 30
            movement_cost_feet: 10
            optional: true
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_jump
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - movement
    requires:
    - dnd2024:spell_jump
    citations:
    - SRD 5.2.1 pp. 143; builtin/srd-5.2.1/spell/jump.md
  dnd2024:spells/levitate:
    source_sha256: b2f7a7c37c6d606e98c0c8da6146c137189d744cdff755bb5b1b68a278d53a95
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/levitate
        implementation: dnd2024:spell_levitate
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 600
          target:
            choices:
            - creature
            - loose_object
            visible: true
            range_feet: 60
            object_weight_pounds_max: 500
          resistance:
            only_if: unwilling_creature
            save:
              ability: constitution
              dc: spell_save_dc
            success: unaffected
          initial_motion:
            direction: up
            distance_feet_max: 20
            suspended_after: true
          target_movement:
            requires: fixed_object_or_surface_in_reach
            method: push_or_pull
            movement_rule: climbing
          caster_altitude_change:
            timing: caster_turn
            distance_feet_max: 20
            directions:
            - up
            - down
            self_target:
              economy: part_of_movement
            other_target:
              economy: magic_action
              target_must_remain_within_range_feet: 60
          spell_end_aloft:
            movement: float_gently_to_ground
          target_binding:
            exactly_one:
            - target_creature
            - target_object
            must_match_selected_mode: true
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_levitate
      inputs:
        caster: participant
        cast_level: integer
        movement_choice:
          type: choice
          required: false
        target_creature:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - movement
      - tests
      - concentration
    requires:
    - dnd2024:spell_levitate
    citations:
    - SRD 5.2.1 pp. 144; builtin/srd-5.2.1/spell/levitate.md
  dnd2024:spells/locate_creature:
    source_sha256: 71791c84c703600cfc2ec519cd7259ed418393aabeccda5a822dab5d794f3c92
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/locate_creature
        implementation: dnd2024:spell_locate_creature
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 3600
          target_choice:
            chooser: caster
            named_or_described: true
            familiar: true
            options:
            - known_specific_creature
            - nearest_of_specific_kind
            qualifying_prior_sighting:
              distance_feet_max: 30
              count_min: 1
          detection:
            distance_feet_max: 1000
            outputs:
            - direction_to_location
            moving_target_additional_output: movement_direction
          failure_conditions:
          - target_in_different_form
          - lead_blocks_direct_path
          lead_block:
            thickness: any
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_locate_creature
      inputs:
        caster: participant
        cast_level: integer
        creature_description: choice
        world_facts: resolution
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - information
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - information
      - concentration
    requires:
    - dnd2024:spell_locate_creature
    citations:
    - SRD 5.2.1 pp. 145; builtin/srd-5.2.1/spell/locate_creature.md
  dnd2024:spells/mage_hand:
    source_sha256: d1b4c55ba2ecbdab60c86eecad2fc7174e38f9d32c62ec46c2c710fcb0409645
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mage_hand
        implementation: dnd2024:spell_mage_hand
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 60
          hand:
            spectral: true
            floating: true
            initial_point: caster_choice
            range_feet: 30
          end_conditions:
          - distance_from_caster_greater_than_30_feet
          - caster_casts_mage_hand_again
          manipulation:
            at_cast: true
            later_economy: magic_action
            later_timing: caster_turn
            options:
            - manipulate_object
            - open_unlocked_door
            - open_unlocked_container
            - stow_in_open_container
            - retrieve_from_open_container
            - pour_vial_contents
          movement:
            as_part_of_later_control_action: true
            distance_feet_max: 30
          limits:
            attack: false
            activate_magic_item: false
            carried_weight_pounds_max: 10
    contract:
      id: dnd2024:spell_mage_hand
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        manipulation: choice
        object:
          type: object
          required: false
      outputs:
      - effects
      - objects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - world_objects
      - inventory
    requires:
    - dnd2024:spell_mage_hand
    citations:
    - SRD 5.2.1 pp. 145; builtin/srd-5.2.1/spell/mage_hand.md
  dnd2024:spells/magic_mouth:
    source_sha256: b42004816ffbf42654a12a2504c9946b429f683b5f9286ba6e2e253f5d4b8b68
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/magic_mouth
        implementation: dnd2024:spell_magic_mouth
        parameters:
          spell_lifetime:
            concentration: false
            until: dispelled_or_source_specific_end
          target:
            kind: object
            visible: true
            range_feet: 30
            worn_or_carried_by_another: false
          message:
            spoken_at_cast: true
            maximum_words: 25
            maximum_delivery_seconds: 600
            playback_voice: caster_voice_at_cast
            playback_volume: spoken_volume_at_cast
          trigger:
            chooser: caster
            timing: cast
            allowed_modalities:
            - visual
            - audible
            maximum_distance_from_object_feet: 30
            condition_resolution: supplied_trigger_predicate
          delivery:
            event: declared_trigger.occurred
            mouth_appears: true
            mouth_location: existing_mouth_or_mouthlike_feature_if_present
          recurrence_choice:
            chooser: caster
            timing: cast
            options:
            - end_after_delivery
            - repeat_on_each_trigger
    contract:
      id: dnd2024:spell_magic_mouth
      inputs:
        caster: participant
        cast_level: integer
        target: object
        message: text
        trigger: choice
        recurrence: choice
      outputs:
      - effects
      events:
      - declared_trigger.occurred
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - illusions
    requires:
    - dnd2024:spell_magic_mouth
    citations:
    - SRD 5.2.1 pp. 146; builtin/srd-5.2.1/spell/magic_mouth.md
  dnd2024:spells/mass_cure_wounds:
    source_sha256: ac734415522658a6792bbc35ad33112b6521507257a8a7ea51090d11f29632dd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mass_cure_wounds
        implementation: dnd2024:spell_mass_cure_wounds
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          area:
            shape: sphere
            radius_feet: 30
            origin: chosen_point
            point_visible: true
            range_feet: 60
          targets:
            kind: creature
            in_area: true
            chooser: caster
            maximum_count: 6
          healing:
            dice:
              base_count: 5
              sides: 8
              extra_per_slot: 1
              base_slot: 5
            add:
              binding: spellcasting_ability_modifier
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_mass_cure_wounds
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        origin: location
        spellcasting_ability_modifier: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - healing
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - healing
    requires:
    - dnd2024:spell_mass_cure_wounds
    citations:
    - SRD 5.2.1 pp. 147; builtin/srd-5.2.1/spell/mass_cure_wounds.md
  dnd2024:spells/maze:
    source_sha256: 83092df34a0033b0a3bd52d6e051d755e7c4887d3150ff2455d208e1f29c14d0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/maze
        implementation: dnd2024:spell_maze
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 600
          target:
            kind: creature
            visible: true
            range_feet: 60
          banish:
            destination: labyrinthine_demiplane
            initial_save: false
          escape:
            economy: study_action
            check:
              ability: intelligence
              skill: investigation
              dc: 20
            success:
              escape: true
              end_spell: true
          on_spell_end:
            return: original_space
            occupied_fallback: nearest_unoccupied_space
    contract:
      id: dnd2024:spell_maze
      inputs:
        caster: participant
        cast_level: integer
        target: participant
      outputs:
      - effects
      - transports
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - planar_travel
      - concentration
    requires:
    - dnd2024:spell_maze
    citations:
    - SRD 5.2.1 pp. 148; builtin/srd-5.2.1/spell/maze.md
  dnd2024:spells/meteor_swarm:
    source_sha256: a58126e8e5a34473e7491bff4839ecccc8242a2dae6be3fc51ed0086bd3d6801
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/meteor_swarm
        implementation: dnd2024:spell_meteor_swarm
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          areas:
            count: 4
            different_points: true
            point_visible: true
            range_miles: 1
            impact: ground
            shape: sphere
            radius_feet: 40
          targets:
            kind: creature
            in_any_area: true
            overlapping_areas_affect_count: 1
          save:
            ability: dexterity
            dc: spell_save_dc
          damage:
          - dice: 20d6
            type: fire
          - dice: 20d6
            type: bludgeoning
          success_damage_scale:
            numerator: 1
            denominator: 2
          objects:
            magical: false
            worn: false
            carried: false
            in_area: true
            take_damage: true
            flammable_effect: start_burning
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_meteor_swarm
      inputs:
        caster: participant
        cast_level: integer
        origins: location_list
        targets: participant_list
        objects: object_list
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - damage
      - tests
    requires:
    - dnd2024:spell_meteor_swarm
    citations:
    - SRD 5.2.1 pp. 149; builtin/srd-5.2.1/spell/meteor_swarm.md
  dnd2024:spells/mirage_arcane:
    source_sha256: 879e2a193b80de527ec4dd14c5ef6b3146a8ab7503b72f4b2d4e56ff3c1de45f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mirage_arcane
        implementation: dnd2024:spell_mirage_arcane
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 864000
          area:
            maximum_square_miles: 1
            range: sight
          terrain:
            appearance_design: caster_choice
            modalities:
            - visual
            - audible
            - tactile
            - olfactory
            movement_effect_choices:
            - create_difficult_terrain
            - remove_difficult_terrain
            - impede_movement
            terrain_geometry: supplied_design
          structures:
            alter_appearance: true
            add_illusory_structures: true
          creatures:
            disguise: false
            conceal: false
            add: false
          removed_illusory_piece:
            event: object.left_area
            effect: immediately_disappear
          truesight:
            see_true_terrain: true
            aware_of_illusion: true
            removes_other_modalities: false
            physical_interaction_remains_possible: true
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_mirage_arcane
      inputs:
        caster: participant
        cast_level: integer
        area: location
        terrain_design: choice
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - object.left_area
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - illusions
      - areas
    requires:
    - dnd2024:spell_mirage_arcane
    citations:
    - SRD 5.2.1 pp. 149, 150; builtin/srd-5.2.1/spell/mirage_arcane.md
  dnd2024:spells/modify_memory:
    source_sha256: b43b0559d5cd7ac8d674d93fe4ada835893d0ba5f4168f7a2f5663464c421aa3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/modify_memory
        implementation: dnd2024:spell_modify_memory
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          target:
            kind: creature
            other_than_caster: true
            visible: true
            range_feet: 30
          save:
            ability: wisdom
            dc: spell_save_dc
            advantage_if: caster_fighting_target
            success: unaffected
          failure:
            condition: charmed
            while_charmed:
              condition: incapacitated
              aware_of_surroundings: false
              can_hear_caster: true
          interrupted:
            events:
            - target.took_damage
            - target.targeted_by_another_spell
            end_spell: true
            modify_memory: false
          memory_event:
            experienced_by_target: true
            maximum_event_minutes: 10
            history_window_by_slot:
            - level: 5
              days: 1
            - level: 6
              days: 7
            - level: 7
              days: 30
            - level: 8
              days: 365
            - level: 9
              history: any_time
          modification_choice:
            chooser: caster
            options:
            - erase_event
            - perfect_recall
            - change_details
            - create_other_event_memory
            duration: permanent
          description:
            spoken_to_target: true
            target_understands_language: true
            complete_before_spell_ends: true
            fill_missing_details: target_mind
          commit:
            event: spell.effect_ended
            requires: completed_understood_description_and_not_interrupted
            otherwise: unchanged_memory
          behavior:
            compelled_by_memory: false
            gm_assessment:
              kind: choice
              options:
              - coherent
              - dismissed_as_bad_dream
              - too_nonsensical_to_affect
              consider:
              - natural_inclinations
              - alignment
              - beliefs
          restore_true_memory_spells:
          - key: dnd2024:spells/remove_curse
          - key: dnd2024:spells/greater_restoration
    contract:
      id: dnd2024:spell_modify_memory
      inputs:
        caster: participant
        cast_level: integer
        target: participant
        memory_event: choice
        description: text
        coherence_assessment: choice
        save_dc: integer
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      - spell.effect_ended
      - target.targeted_by_another_spell
      - target.took_damage
      capabilities:
      - spellcasting
      - effects
      - memory
      - behavior
      - conditions
      - tests
      - concentration
    requires:
    - dnd2024:spell_modify_memory
    citations:
    - SRD 5.2.1 pp. 150; builtin/srd-5.2.1/spell/modify_memory.md
  dnd2024:spells/pass_without_trace:
    source_sha256: 30eb9f9a0c1d550483096a3d7e486f4123748be4aaa150e13bb37200dc54d6b1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/pass_without_trace
        implementation: dnd2024:spell_pass_without_trace
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 3600
          area:
            shape: emanation
            radius_feet: 30
            origin: caster
            moves_with_origin: true
          beneficiaries:
            caster: true
            additional_creatures: caster_choice
            requires: in_aura
          benefits:
            dexterity_stealth_check_bonus: 10
            leave_tracks: false
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_pass_without_trace
      inputs:
        caster: participant
        cast_level: integer
        beneficiaries: participant_list
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - movement
      - areas
      - concentration
    requires:
    - dnd2024:spell_pass_without_trace
    citations:
    - SRD 5.2.1 pp. 151; builtin/srd-5.2.1/spell/pass_without_trace.md
  dnd2024:spells/phantom_steed:
    source_sha256: 1aa6dc3dbfa2f9065588cfb41cde836f5b57896b2e43755a7618c7ea1dd0c0e4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/phantom_steed
        implementation: dnd2024:spell_phantom_steed
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 3600
          steed:
            size: large
            creature_kind: quasi_real_horselike
            space:
              ground: true
              unoccupied: true
              range_feet: 30
            appearance: caster_choice
            stat_block:
              key: dnd2024:creatures/riding_horse
            speed_feet: 100
            travel_miles_per_hour: 13
          equipment:
            grants:
            - saddle
            - bit
            - bridle
            vanishes_if_distance_from_steed_greater_than_feet: 10
          rider:
            caster_or_chosen_creature: true
          end_early:
            event: damage.applied
            subject: steed
            minimum_damage: 1
          spell_end:
            fade_seconds: 60
            rider_dismount_seconds: 60
    contract:
      id: dnd2024:spell_phantom_steed
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        appearance: choice
        rider: participant
      outputs:
      - effects
      - objects
      events:
      - damage.applied
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - world_objects
      - inventory
    requires:
    - dnd2024:spell_phantom_steed
    citations:
    - SRD 5.2.1 pp. 152; builtin/srd-5.2.1/spell/phantom_steed.md
  dnd2024:spells/plant_growth:
    source_sha256: 6e0483e950ecce51643a4a46e01d15a4a5afa68c2b5859a1c8f72aa4b37f19ee
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/plant_growth
        implementation: dnd2024:spell_plant_growth
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          mode_choice:
            caster_choice: true
            determined_by: casting_time
          overgrowth:
            casting_economy: action
            origin:
              chosen_point: true
              range_feet: 150
            shape: sphere
            radius_feet: 100
            affected_plants: normal
            growth: thick_overgrown
            movement_feet_cost_per_foot: 4
            exclusions:
              chooser: caster
              count: any
              area_size: any
              within_spell_area: true
          enrichment:
            casting_seconds: 28800
            origin:
              chosen_point: true
              range_feet: 150
            radius_miles: 0.5
            affected_plants: all
            duration_days: 365
            harvest_food_multiplier: 2
            benefit_limit:
              count: 1
              per: year
              scope: plant
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_plant_growth
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        mode: choice
        excluded_areas:
          type: location_list
          required: false
        campaign: campaign
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - objects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - world_objects
      - areas
      - campaign_time
    requires:
    - dnd2024:spell_plant_growth
    citations:
    - SRD 5.2.1 pp. 153; builtin/srd-5.2.1/spell/plant_growth.md
  dnd2024:spells/power_word_kill:
    source_sha256: 96a61142eee046a239a55467d893b7409794f275bd7049577cd3783d35dadda9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/power_word_kill
        implementation: dnd2024:spell_power_word_kill
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          target:
            kind: creature
            visible: true
            range_feet: 60
          branches:
          - when:
              stat: current_hit_points
              comparison: lte
              value: 100
            effect: die
          - when:
              stat: current_hit_points
              comparison: gt
              value: 100
            damage:
              dice: 12d12
              type: psychic
    contract:
      id: dnd2024:spell_power_word_kill
      inputs:
        caster: participant
        cast_level: integer
        target: participant
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - damage
    requires:
    - dnd2024:spell_power_word_kill
    citations:
    - SRD 5.2.1 pp. 154; builtin/srd-5.2.1/spell/power_word_kill.md
  dnd2024:spells/prismatic_spray:
    source_sha256: 891d7aebcfc328a3c9dee97e5eb8cda403813732a848320b7473f70ba21a93c6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/prismatic_spray
        implementation: dnd2024:spell_prismatic_spray
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          area:
            shape: cone
            length_feet: 60
            origin: caster
          targets:
            kind: creature
            selection: all_in_area
          initial_save:
            ability: dexterity
            dc: spell_save_dc
            count_per_target: 1
          ray_roll:
            dice: 1d8
            scope: target
          rays:
          - roll: 1
            color: red
            failure_damage:
              dice: 12d6
              type: fire
            success_damage_scale:
              numerator: 1
              denominator: 2
          - roll: 2
            color: orange
            failure_damage:
              dice: 12d6
              type: acid
            success_damage_scale:
              numerator: 1
              denominator: 2
          - roll: 3
            color: yellow
            failure_damage:
              dice: 12d6
              type: lightning
            success_damage_scale:
              numerator: 1
              denominator: 2
          - roll: 4
            color: green
            failure_damage:
              dice: 12d6
              type: poison
            success_damage_scale:
              numerator: 1
              denominator: 2
          - roll: 5
            color: blue
            failure_damage:
              dice: 12d6
              type: cold
            success_damage_scale:
              numerator: 1
              denominator: 2
          - roll: 6
            color: indigo
            initial_success: unaffected
            initial_failure:
              condition: restrained
            subsequent_save:
              event: turn.end
              subject: target
              ability: constitution
              dc: spell_save_dc
              counters:
              - successes
              - failures
              counters_initial: 0
              consecutive_required: false
              successes_threshold:
                count: 3
                effect: end_restrained
              failures_threshold:
                count: 3
                effect: petrified
                duration: until_freed
              stop_saves_on_threshold: true
            freeing_example:
              key: dnd2024:spells/greater_restoration
          - roll: 7
            color: violet
            initial_success: unaffected
            initial_failure:
              condition: blinded
            subsequent_save:
              event: turn.start
              subject: caster
              occurrence: next
              ability: wisdom
              dc: spell_save_dc
              save_subject: target
              success:
                end_condition: blinded
              failure:
                end_condition: blinded
                teleport:
                  plane:
                    chooser: gm
                    constraint: different_plane
          - roll: 8
            color: special
            ray_count: 2
            roll_each: 1d8
            reroll_values:
            - 8
            apply_both: true
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_prismatic_spray
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        targets: participant_list
        plane_choice:
          type: choice
          required: false
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      - transports
      events:
      - spell.cast_completed
      - turn.end
      - turn.start
      capabilities:
      - spellcasting
      - effects
      - planar_travel
      - areas
      - conditions
      - damage
      - tests
    requires:
    - dnd2024:spell_prismatic_spray
    citations:
    - SRD 5.2.1 pp. 154, 155; builtin/srd-5.2.1/spell/prismatic_spray.md
  dnd2024:spells/programmed_illusion:
    source_sha256: 33f834e5208d7d8122a4f6ef958658ac44dd4b16c97bf474e66d61b49696172e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/programmed_illusion
        implementation: dnd2024:spell_programmed_illusion
        parameters:
          spell_lifetime:
            concentration: false
            until: dispelled_or_source_specific_end
          illusion:
            kinds:
            - object
            - creature
            - visible_phenomenon
            range_feet: 120
            maximum_cube_side_feet: 30
            initial_state: imperceptible
            visual_behavior: caster_script_at_cast
            sounds: caster_script_at_cast
            performance_seconds_max: 300
          trigger:
            chooser: caster
            timing: cast
            modalities:
            - visual
            - audible
            maximum_distance_from_area_feet: 30
            condition_resolution: supplied_trigger_predicate
          cycle:
            on_trigger: perform_script
            after_performance:
              disappear: true
              dormant_seconds: 600
            after_dormancy: triggerable_again
          physical_interaction:
            objects_pass_through: true
            reveals_illusion: true
          examination:
            economy: study_action
            check:
              ability: intelligence
              skill: investigation
              dc: spell_save_dc
            success: discern_illusion
          discerned:
            image: see_through
            noise: hollow
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_programmed_illusion
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        script: choice
        trigger: choice
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - illusions
      - areas
      - tests
    requires:
    - dnd2024:spell_programmed_illusion
    citations:
    - SRD 5.2.1 pp. 156; builtin/srd-5.2.1/spell/programmed_illusion.md
  dnd2024:spells/protection_from_poison:
    source_sha256: 26c177deaf584ff1ba684415ed2af4671736a0f596ccfe48dc8106a69a0b629c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/protection_from_poison
        implementation: dnd2024:spell_protection_from_poison
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 3600
          target:
            kind: creature
            touch: true
          immediate:
            end_condition: poisoned
          benefits:
            save_advantage:
              condition: poisoned
              purposes:
              - avoid
              - end
            damage_resistance: poison
    contract:
      id: dnd2024:spell_protection_from_poison
      inputs:
        caster: participant
        cast_level: integer
        target: participant
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - conditions
    requires:
    - dnd2024:spell_protection_from_poison
    citations:
    - SRD 5.2.1 pp. 157; builtin/srd-5.2.1/spell/protection_from_poison.md
  dnd2024:spells/ray_of_frost:
    source_sha256: aeb923eb41759bf7c327101d096240f0b77417965ca8953124a0a4fb9dc92a70
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/ray_of_frost
        implementation: dnd2024:spell_ray_of_frost
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          target:
            kind: creature
            range_feet: 60
          attack:
            kind: ranged_spell_attack
          hit:
            damage:
              type: cold
              dice:
                sides: 8
                counts_by_character_level:
                - minimum_level: 1
                  count: 1
                - minimum_level: 5
                  count: 2
                - minimum_level: 11
                  count: 3
                - minimum_level: 17
                  count: 4
            speed_modifier_feet: -10
            speed_modifier_until:
              event: turn.start
              subject: caster
              occurrence: next
          miss: no_effect
    contract:
      id: dnd2024:spell_ray_of_frost
      inputs:
        caster: participant
        cast_level: integer
        target: participant
        character_level: integer
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - turn.start
      capabilities:
      - spellcasting
      - effects
      - attacks
      - damage
    requires:
    - dnd2024:spell_ray_of_frost
    citations:
    - SRD 5.2.1 pp. 157; builtin/srd-5.2.1/spell/ray_of_frost.md
  dnd2024:spells/remove_curse:
    source_sha256: c26d242ec5d407bb6a3e285d56f3575f58d2a9c55cc0f759c1d651e4e6ec5364
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/remove_curse
        implementation: dnd2024:spell_remove_curse
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          target:
            kinds:
            - creature
            - object
            touch: true
            count: 1
          normal:
            end: all_curses_affecting_target
          cursed_magic_item_exception:
            item_curse: remains
            owner_attunement: broken
            item_removal_and_discard: permitted
          target_binding:
            exactly_one:
            - target_creature
            - target_object
            must_match_selected_mode: true
    contract:
      id: dnd2024:spell_remove_curse
      inputs:
        caster: participant
        cast_level: integer
        target_creature:
          type: participant
          required: false
        target_object:
          type: object
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - inventory
    requires:
    - dnd2024:spell_remove_curse
    citations:
    - SRD 5.2.1 pp. 158; builtin/srd-5.2.1/spell/remove_curse.md
  dnd2024:spells/reverse_gravity:
    source_sha256: 8949f97373bd8742909e54674a8396ccfc1f5287b8b9eb8c75b476f9a1506d24
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/reverse_gravity
        implementation: dnd2024:spell_reverse_gravity
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          area:
            shape: cylinder
            radius_feet: 50
            height_feet: 100
            origin: chosen_point
            range_feet: 100
          targets:
            kinds:
            - creature
            - object
            in_area: true
            anchored_to_ground: false
          avoid_fall:
            target_kind: creature
            requires: fixed_object_in_reach
            save:
              ability: dexterity
              dc: spell_save_dc
            success: grab_fixed_object_and_avoid_upward_fall
          upward_fall:
            destination: cylinder_top
            collision_with:
            - ceiling
            - anchored_object
            collision_rule: normal_downward_fall
            no_collision_at_top: hover_for_duration
          spell_end:
            affected_creatures_and_objects: fall_downward
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_reverse_gravity
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        targets: participant_list
        objects: object_list
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - movement
      - areas
      - tests
      - concentration
    requires:
    - dnd2024:spell_reverse_gravity
    citations:
    - SRD 5.2.1 pp. 159; builtin/srd-5.2.1/spell/reverse_gravity.md
  dnd2024:spells/sanctuary:
    source_sha256: 3a41061d0de762067b12199427078851889b7e48abc90bb300a67283474ce836
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/sanctuary
        implementation: dnd2024:spell_sanctuary
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 60
          target:
            kind: creature
            range_feet: 30
          protective_trigger:
            event: target.selected
            warded_creature_targeted_by:
            - attack_roll
            - damaging_spell
            attacker_kind: creature
            exclude: area_of_effect
          attacker_save:
            ability: wisdom
            dc: spell_save_dc
            success: original_target_allowed
            failure:
              chooser: attacker
              options:
              - choose_new_legal_target
              - lose_attack_or_spell
          end_early_events:
          - warded_creature.made_attack_roll
          - warded_creature.cast_spell
          - warded_creature.dealt_damage
    contract:
      id: dnd2024:spell_sanctuary
      inputs:
        caster: participant
        cast_level: integer
        target: participant
        retarget_choice:
          type: choice
          required: false
        save_dc: integer
      outputs:
      - effects
      events:
      - spell.cast_completed
      - target.selected
      - warded_creature.cast_spell
      - warded_creature.dealt_damage
      - warded_creature.made_attack_roll
      capabilities:
      - spellcasting
      - effects
      - attacks
      - tests
    requires:
    - dnd2024:spell_sanctuary
    citations:
    - SRD 5.2.1 pp. 159; builtin/srd-5.2.1/spell/sanctuary.md
  dnd2024:spells/secret_chest:
    source_sha256: 9f2f94e0e4ef3e81ddcb21364fd3ea42deb1317d0624a7129c5d6634ccd182b0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/secret_chest
        implementation: dnd2024:spell_secret_chest
        parameters:
          spell_lifetime:
            concentration: false
            until: dispelled_or_source_specific_end
          initial:
            touch_required:
            - chest
            - replica
            destination: ethereal_plane
            transported: chest_and_contents
            contents:
              nonliving: true
              maximum_cubic_feet: 12
            chest_dimensions_feet:
            - 3
            - 2
            - 2
          recall:
            economy: magic_action
            touch_required:
            - replica
            requires: chest_on_ethereal_plane
            destination:
              ground: true
              unoccupied: true
              within_caster_feet: 5
          send_back:
            economy: magic_action
            touch_required:
            - chest
            - replica
            destination: ethereal_plane
          expiration_check:
            starts_after_days: 60
            event: day.end
            probability_increment_percent_per_day: 5
            first_eligible_day_percent: 5
            maximum_percent: 100
            success: end_spell
          additional_end_conditions:
          - caster_casts_secret_chest_again
          - replica_destroyed
          end_while_chest_ethereal:
            chest_remains_ethereal: true
            recoverable_by_search: true
    contract:
      id: dnd2024:spell_secret_chest
      inputs:
        caster: participant
        cast_level: integer
        chest: object
        replica: object
        contents: object_list
        campaign: campaign
      outputs:
      - effects
      - transports
      - objects
      events:
      - day.end
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - planar_travel
      - world_objects
      - inventory
      - campaign_time
    requires:
    - dnd2024:spell_secret_chest
    citations:
    - SRD 5.2.1 pp. 160; builtin/srd-5.2.1/spell/secret_chest.md
  dnd2024:spells/sequester:
    source_sha256: cb3236308c5951bc69685d34c9fb86de2442c48191f3dbcdf6fd1057b21d1e1d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/sequester
        implementation: dnd2024:spell_sequester
        parameters:
          spell_lifetime:
            concentration: false
            until: dispelled_or_source_specific_end
          target:
            choices:
            - object
            - willing_creature
            touch: true
          protection:
            condition: invisible
            divination_spell_targeting: forbidden
            magical_detection: forbidden
            magical_remote_viewing: forbidden
          creature_extra:
            state: suspended_animation
            condition: unconscious
            aging: false
            food_required: false
            water_required: false
            air_required: false
          optional_end_trigger:
            chooser: caster
            requirement: occurs_or_visible_within_target_miles
            maximum_miles: 1
            supports_elapsed_time: true
            condition_resolution: supplied_trigger_predicate
          end_on_damage:
            event: damage.applied
            subject: target
          target_binding:
            exactly_one:
            - target_creature
            - target_object
            must_match_selected_mode: true
    contract:
      id: dnd2024:spell_sequester
      inputs:
        caster: participant
        cast_level: integer
        end_trigger:
          type: choice
          required: false
        target_creature:
          type: participant
          required: false
        target_object:
          type: object
          required: false
      outputs:
      - effects
      events:
      - damage.applied
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - magic_suppression
      - conditions
    requires:
    - dnd2024:spell_sequester
    citations:
    - SRD 5.2.1 pp. 161; builtin/srd-5.2.1/spell/sequester.md
  dnd2024:spells/shield_of_faith:
    source_sha256: 38a5c1039f87dc67f5f697dac4aae5e37f4fa39ae93bfa2f0eaf06139ee082f7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/shield_of_faith
        implementation: dnd2024:spell_shield_of_faith
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 600
          target:
            kind: creature
            chooser: caster
            range_feet: 60
          modifier:
            stat: armor_class
            add: 2
    contract:
      id: dnd2024:spell_shield_of_faith
      inputs:
        caster: participant
        cast_level: integer
        target: participant
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - concentration
    requires:
    - dnd2024:spell_shield_of_faith
    citations:
    - SRD 5.2.1 pp. 162; builtin/srd-5.2.1/spell/shield_of_faith.md
  dnd2024:spells/silence:
    source_sha256: 37a05864857c002f8d6105f6b85f16ba0aaf9b52e7e7c709dfa4a8e0fa7d63bf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/silence
        implementation: dnd2024:spell_silence
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 600
          area:
            shape: sphere
            radius_feet: 20
            origin: chosen_point
            range_feet: 120
          sound:
            creation_inside: forbidden
            passage_through: forbidden
          entirely_inside:
            creatures_and_objects:
              damage_immunity: thunder
            creatures:
              condition: deafened
          spellcasting_inside:
            verbal_component: forbidden
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_silence
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - concentration
    requires:
    - dnd2024:spell_silence
    citations:
    - SRD 5.2.1 pp. 162; builtin/srd-5.2.1/spell/silence.md
  dnd2024:spells/sleet_storm:
    source_sha256: 38591d30b30ca1abed7ff5278b49cbd1a4d948c5c0c8b0e1e43babc3418c3e08
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/sleet_storm
        implementation: dnd2024:spell_sleet_storm
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          area:
            shape: cylinder
            height_feet: 40
            radius_feet: 20
            origin: chosen_point
            range_feet: 150
          properties:
            obscurement: heavy
            exposed_flames: doused
            ground: difficult_terrain
          triggers:
          - event: creature.entered_area
            occurrence_per_turn: first
          - event: turn.start
            subjects: creatures_in_area
          save:
            ability: dexterity
            dc: spell_save_dc
            success: unaffected
            failure:
              condition: prone
              concentration: lost
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_sleet_storm
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        targets: participant_list
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - concentration.ended
      - creature.entered_area
      - spell.cast_completed
      - turn.start
      capabilities:
      - spellcasting
      - effects
      - areas
      - conditions
      - tests
      - concentration
    requires:
    - dnd2024:spell_sleet_storm
    citations:
    - SRD 5.2.1 pp. 163; builtin/srd-5.2.1/spell/sleet_storm.md
  dnd2024:spells/speak_with_animals:
    source_sha256: e60edb90b94f5cc5044f60fc7e66c2cede3b41f0f98287f878da515656c3f5c4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/speak_with_animals
        implementation: dnd2024:spell_speak_with_animals
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 600
          target: caster
          communication:
            with_creature_types:
            - beast
            comprehend: true
            verbal_communication: true
            influence_skill_options: all
          information:
            minimum_topics:
            - nearby_locations
            - nearby_monsters
            minimum_perception_history_hours: 24
            typical_topic_limits:
            - survival
            - companionship
            answers_depend_on: beast_knowledge
    contract:
      id: dnd2024:spell_speak_with_animals
      inputs:
        caster: participant
        cast_level: integer
        conversation:
          type: text
          required: false
      outputs:
      - effects
      - information
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - information
    requires:
    - dnd2024:spell_speak_with_animals
    citations:
    - SRD 5.2.1 pp. 164; builtin/srd-5.2.1/spell/speak_with_animals.md
  dnd2024:spells/spike_growth:
    source_sha256: 4b6c2f302b263b19bcb7a90d2356e046ab8d5b9ff41d3adc1b74ee5a3b028ed0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/spike_growth
        implementation: dnd2024:spell_spike_growth
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 600
          area:
            shape: sphere
            radius_feet: 20
            origin: chosen_point
            range_feet: 150
            affected_surface: ground
          properties:
            difficult_terrain: true
            camouflage: natural_terrain
          damage:
            event: participant.moved
            target_kind: creature
            movement_scopes:
            - into_area
            - within_area
            per_distance_feet: 5
            dice: 2d4
            type: piercing
            saving_throw: false
          hazard_recognition:
            applies_if: did_not_see_area_at_cast
            before_entering: true
            economy: search_action
            check:
              ability: wisdom
              skill_choice:
              - perception
              - survival
              dc: spell_save_dc
            success: recognize_hazard
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_spike_growth
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        targets: participant_list
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - concentration.ended
      - participant.moved
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - movement
      - areas
      - damage
      - tests
      - concentration
    requires:
    - dnd2024:spell_spike_growth
    citations:
    - SRD 5.2.1 pp. 164; builtin/srd-5.2.1/spell/spike_growth.md
  dnd2024:spells/stinking_cloud:
    source_sha256: 6a7f2d9c23db25ccbf7e8ea68e7d13b688cae72a0b308b1c74a2dc8ed38428c6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/stinking_cloud
        implementation: dnd2024:spell_stinking_cloud
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          area:
            shape: sphere
            radius_feet: 20
            origin: chosen_point
            range_feet: 90
          properties:
            obscurement: heavy
            medium: gas
          end_early:
            cause: strong_wind_dispersal
            example_spell:
              key: dnd2024:spells/gust_of_wind
          trigger:
            event: turn.start
            subjects: creatures_in_area
          save:
            ability: constitution
            dc: spell_save_dc
            success: unaffected
          failure:
            condition: poisoned
            until:
              event: turn.end
              subject: target
              occurrence: current
            while_poisoned:
              action: forbidden
              bonus_action: forbidden
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_stinking_cloud
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        targets: participant_list
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      - turn.end
      - turn.start
      capabilities:
      - spellcasting
      - effects
      - areas
      - conditions
      - tests
      - concentration
    requires:
    - dnd2024:spell_stinking_cloud
    citations:
    - SRD 5.2.1 pp. 165; builtin/srd-5.2.1/spell/stinking_cloud.md
  dnd2024:spells/suggestion:
    source_sha256: d69f79f28b931e4fe561ff5f71e2a3902efeaec1683b16a101527ce9ec51b8ad
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/suggestion
        implementation: dnd2024:spell_suggestion
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 28800
          target:
            kind: creature
            visible: true
            range_feet: 30
            hears_caster: true
            understands_caster: true
          instruction:
            maximum_words: 25
            kind: course_of_activity
            must_sound_achievable: true
            obvious_damage_to_target_or_allies: forbidden
            feasibility_resolution: supplied_assessment
          save:
            ability: wisdom
            dc: spell_save_dc
            success: unaffected
          failure:
            condition: charmed
            source: caster
            instruction_compliance: best_effort
          end_target:
            any:
            - caster_or_allies_deal_damage_to_target
            - suggested_activity_completed
            - spell_duration_ended
    contract:
      id: dnd2024:spell_suggestion
      inputs:
        caster: participant
        cast_level: integer
        target: participant
        suggested_activity: text
        feasibility_assessment: resolution
        save_dc: integer
      outputs:
      - effects
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - behavior
      - conditions
      - tests
      - concentration
    requires:
    - dnd2024:spell_suggestion
    citations:
    - SRD 5.2.1 pp. 166; builtin/srd-5.2.1/spell/suggestion.md
  dnd2024:spells/symbol:
    source_sha256: caee64894acd767654687623b646fb8a38457d89a88751687e461463271b14b2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/symbol
        implementation: dnd2024:spell_symbol
        parameters:
          spell_lifetime:
            concentration: false
            until: dispelled_or_source_specific_end
          glyph:
            placement_choices:
            - surface
            - inside_closable_object
            maximum_diameter_feet: 10
            object_movement_limit_feet_from_cast_point: 10
            movement_limit_exceeded:
              break_glyph: true
              end_spell: true
              trigger: false
          detection:
            check:
              ability: wisdom
              skill: perception
              dc: spell_save_dc
            success: notice_glyph
          trigger:
            chooser: caster
            timing: cast
            surface_examples:
            - touch
            - step_on
            - remove_covering_object
            - approach_distance
            object_examples:
            - open_object
            - see_glyph
            creature_type_filters: allowed
            exemption_conditions: allowed
            password_exemption: allowed
            condition_resolution: supplied_trigger_predicate
          activation:
            glow: true
            light: dim
            shape: sphere
            radius_feet: 60
            active_seconds: 600
            active_expiry: end_spell
          targets:
            kind: creature
            events:
            - glyph.activated
            - creature.entered_area
            - turn.end
            enter_occurrence_per_turn: first
            in_sphere: true
            limit:
              count: 1
              per: turn
              scope: target
          effect_choice:
            chooser: caster
            timing: inscription
            count: 1
          options:
            death:
              save:
                ability: constitution
                dc: spell_save_dc
              failure_damage:
                dice: 10d10
                type: necrotic
              success_damage_scale:
                numerator: 1
                denominator: 2
            discord:
              save:
                ability: wisdom
                dc: spell_save_dc
              failure:
                seconds: 60
                behavior: argue_with_other_creatures
                meaningful_communication: false
                disadvantage:
                - attack_rolls
                - ability_checks
            fear:
              save:
                ability: wisdom
                dc: spell_save_dc
              failure:
                condition: frightened
                seconds: 60
                source: glyph
                movement:
                  timing: each_target_turn
                  minimum_feet: 30
                  direction: away_from_glyph
                  only_if_able: true
            pain:
              save:
                ability: constitution
                dc: spell_save_dc
              failure:
                condition: incapacitated
                seconds: 60
            sleep:
              save:
                ability: wisdom
                dc: spell_save_dc
              failure:
                condition: unconscious
                seconds: 600
                awaken_if:
                - takes_damage
                - another_creature_uses_action_to_shake_awake
            stunning:
              save:
                ability: wisdom
                dc: spell_save_dc
              failure:
                condition: stunned
                seconds: 60
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_symbol
      inputs:
        caster: participant
        cast_level: integer
        glyph_location: location
        object:
          type: object
          required: false
        trigger: choice
        effect_choice: choice
        targets:
          type: participant_list
          required: false
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - creature.entered_area
      - glyph.activated
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - areas
      - behavior
      - conditions
      - damage
      - tests
    requires:
    - dnd2024:spell_symbol
    citations:
    - SRD 5.2.1 pp. 167; builtin/srd-5.2.1/spell/symbol.md
  dnd2024:spells/teleportation_circle:
    source_sha256: 1b54a94ec08b8f0514d75a19212c47ba4272d669b4ba827a102660e88792f71e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/teleportation_circle
        implementation: dnd2024:spell_teleportation_circle
        parameters:
          spell_lifetime:
            concentration: false
            maximum_rounds: 1
          origin:
            ground: true
            range_feet: 10
            circle_radius_feet: 5
            draw_sigils_during_cast: true
          destination:
            permanent_teleportation_circle: true
            sigil_sequence_known: true
            same_plane: true
            chooser: caster
          portal:
            until:
              event: turn.end
              subject: caster
              occurrence: next
            trigger: creature.entered_portal
            transport: instantaneous
            arrival_within_destination_feet: 5
            occupied_fallback: nearest_unoccupied_space
          knowledge:
            on_first_gain_casting_ability:
              learn_sequences: 2
              destination_plane: material
              chooser: gm
            additional_sequence_study_seconds: 60
            sigil_sequences: unique_per_circle
          permanence:
            same_location: true
            frequency: daily
            cast_days: 365
            result: permanent_teleportation_circle
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_teleportation_circle
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        destination: location
        campaign: campaign
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - transports
      events:
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - planar_travel
      - areas
      - memory
      - campaign_time
    requires:
    - dnd2024:spell_teleportation_circle
    citations:
    - SRD 5.2.1 pp. 168, 169; builtin/srd-5.2.1/spell/teleportation_circle.md
  dnd2024:spells/tiny_hut:
    source_sha256: 9586f9eaab9bf001890fce7e91b918939f33062996928ce85189d0e576200850
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/tiny_hut
        implementation: dnd2024:spell_tiny_hut
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 28800
          area:
            shape: emanation
            radius_feet: 10
            origin: caster_at_cast
            stationary: true
          cast_failure:
            if: any_creature_in_area_not_fully_encapsulated
          crossing:
            creatures_and_objects_present_at_cast: freely_allowed
            all_other_creatures_and_objects: barred
          spell_barrier:
            maximum_spell_level_blocked: 3
            cast_through: forbidden
            effects_extend_inside: forbidden
          interior:
            atmosphere: comfortable_and_dry
            weather_independent: true
            light_choice:
              chooser: caster
              economy: none
              options:
              - dim
              - darkness
          appearance:
            outside: opaque
            color: caster_choice
            inside: transparent
          early_end_conditions:
          - caster_leaves_area
          - caster_casts_tiny_hut_again
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_tiny_hut
      inputs:
        caster: participant
        cast_level: integer
        color: choice
        lighting_choice: choice
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - areas
      - magic_suppression
    requires:
    - dnd2024:spell_tiny_hut
    citations:
    - SRD 5.2.1 pp. 169; builtin/srd-5.2.1/spell/tiny_hut.md
  dnd2024:spells/true_polymorph:
    source_sha256: 21e16b78682ecc92d1e1c6524837cfc673c3e3215ccb452238de0cc354fc83ae
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/true_polymorph
        implementation: dnd2024:spell_true_polymorph
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 3600
          target:
            count: 1
            visible: true
            range_feet: 30
            kinds:
            - creature
            - nonmagical_object
            object_requirements:
              worn: false
              carried: false
          resistance:
            subject: unwilling_creature
            save:
              ability: wisdom
              dc: spell_save_dc
            success: unaffected
          lifetime:
            initial: concentration_up_to_3600_seconds
            ends_if_target:
            - dies
            - destroyed
            full_concentration_seconds: 3600
            full_concentration_result: until_dispelled
          choice:
            chooser: caster
            modes:
            - creature_into_creature
            - object_into_creature
            - creature_into_object
          creature_into_creature:
            form:
              kind: any_creature
              maximum_challenge_rating: target_challenge_rating_or_level
            statistics:
              replace: new_form_stat_block
              retain:
              - hit_points
              - hit_point_dice
              - alignment
              - personality
            temporary_hit_points:
              amount:
                stat:
                  of: new_form
                  key: hit_points
              remaining_vanish_on_spell_end: true
            restrictions:
              actions_limited_by_anatomy: true
              speak: false
              cast_spells: false
              gear: melded
              use_equipment: false
              benefit_from_equipment: false
          object_into_creature:
            form:
              kind: any_creature
              maximum_size: object_size
              maximum_challenge_rating: 9
            attitude:
              toward:
              - caster
              - caster_allies
              value: friendly
            combat:
              turn: immediately_after_caster
              obeys_caster_commands: true
            after_elapsed_greater_than_seconds:
              seconds: 3600
              caster_control: ends
              continued_friendly_attitude:
                chooser: gm
                consideration: caster_treatment
            statistics: chosen_creature_stat_block
          creature_into_object:
            form:
              kind: nonmagical_object
              maximum_size: target_creature_size
            transformation_includes:
            - creature
            - worn_items
            - carried_items
            statistics: object_statistics
            after_reversion:
              memory_of_time_in_object_form: none
          target_binding:
            exactly_one:
            - target_creature
            - target_object
            must_match_selected_mode: true
    contract:
      id: dnd2024:spell_true_polymorph
      inputs:
        caster: participant
        cast_level: integer
        mode: choice
        form: definition
        attitude_assessment:
          type: choice
          required: false
        target_creature:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        save_dc: integer
      outputs:
      - effects
      - forms
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - forms
      - inventory
      - tests
      - concentration
    requires:
    - dnd2024:spell_true_polymorph
    citations:
    - SRD 5.2.1 pp. 170; builtin/srd-5.2.1/spell/true_polymorph.md
  dnd2024:spells/tsunami:
    source_sha256: 6864fa5c464e788a4eaf255dcd52c23035a2daefeee2d99e658aec11e2a54c07
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/tsunami
        implementation: dnd2024:spell_tsunami
        parameters:
          spell_lifetime:
            concentration: true
            maximum_rounds: 6
          wall:
            origin: chosen_point
            range_miles: 1
            maximum_length_feet: 300
            maximum_height_feet: 300
            maximum_duration_rounds: 6
            maximum_thickness_feet: 50
          initial:
            targets: all_creatures_in_area
            save:
              ability: strength
              dc: spell_save_dc
            failure_damage:
              dice: 6d10
              type: bludgeoning
            success_damage_scale:
              numerator: 1
              denominator: 2
          movement:
            event: turn.start
            subject: caster
            only_turns_after_wall_appeared: true
            distance_feet: 50
            direction: away_from_caster
            carry: creatures_inside
          moving_wall_damage:
            targets:
              kinds: creature
              maximum_size: huge
              positions:
              - inside_wall
              - space_entered_by_wall
            save:
              ability: strength
              dc: spell_save_dc
              success: no_damage
            failure_damage:
              initial_dice: 5d10
              type: bludgeoning
              later_round_dice_decrement: 1
            damage_limit:
              count: 1
              per: round
              scope: creature
          decay:
            event: turn.end
            subject: caster
            sequence: after_wall_movement_turn
            height_decrease_feet: 50
            future_damage_decrease_d10: 1
            height_zero: end_spell
          trapped_movement:
            method: swimming
            check:
              ability: strength
              skill: athletics
              dc: spell_save_dc
            success: may_move
            failure: cannot_move
            exit_wall: fall_to_ground
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_tsunami
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        wall_dimensions: choice
        targets: participant_list
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - concentration.ended
      - spell.cast_completed
      - turn.end
      - turn.start
      capabilities:
      - spellcasting
      - effects
      - movement
      - areas
      - damage
      - tests
      - concentration
    requires:
    - dnd2024:spell_tsunami
    citations:
    - SRD 5.2.1 pp. 171; builtin/srd-5.2.1/spell/tsunami.md
  dnd2024:spells/vitriolic_sphere:
    source_sha256: adb9638d46828e51f3a8224fef9019260633bfeb1fe20c0313dc606e42018faf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/vitriolic_sphere
        implementation: dnd2024:spell_vitriolic_sphere
        parameters:
          spell_lifetime:
            concentration: false
            instantaneous: true
          area:
            shape: sphere
            radius_feet: 20
            origin: chosen_location
            range_feet: 150
            projectile_diameter_feet: 1
          targets:
            kind: creature
            selection: all_in_area
          save:
            ability: dexterity
            dc: spell_save_dc
          failure:
            initial_damage:
              dice:
                base_count: 10
                sides: 4
                extra_per_slot: 2
                base_slot: 4
              type: acid
            delayed_damage:
              event: turn.end
              subject: target
              occurrence: next
              dice: 5d4
              type: acid
          success:
            initial_damage_scale:
              numerator: 1
              denominator: 2
            delayed_damage: false
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_vitriolic_sphere
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        targets: participant_list
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - areas
      - damage
      - tests
    requires:
    - dnd2024:spell_vitriolic_sphere
    citations:
    - SRD 5.2.1 pp. 172; builtin/srd-5.2.1/spell/vitriolic_sphere.md
  dnd2024:spells/wall_of_stone:
    source_sha256: 431b4f9dc940e4ea90483c0e0aab8e5d43c109bab24771d79d1a4ef63bb65d9d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/wall_of_stone
        implementation: dnd2024:spell_wall_of_stone
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 600
          wall:
            material: stone
            magical: false
            origin: chosen_point
            range_feet: 120
            panel_count: 10
            panel_choice:
            - width_feet: 10
              height_feet: 10
              thickness_inches: 6
            - width_feet: 10
              height_feet: 20
              thickness_inches: 3
            every_panel_contiguous_with_another: true
          appearance_collision:
            creature_cut_by_wall:
              push_to_side: caster_choice
            enclosed_creature:
              save:
                ability: dexterity
                dc: spell_save_dc
              success:
                optional: true
                economy: reaction
                movement_max: target_speed
                destination: no_longer_enclosed
          geometry:
            shape: caster_choice
            overlap_creature_or_object: false
            vertical_required: false
            firm_foundation_required: false
            must_merge_with_existing_stone: true
            solid_existing_stone_support_required: true
            span_greater_than_feet:
              threshold: 20
              panel_size_scale:
                numerator: 1
                denominator: 2
              purpose: supports
            crude_features_allowed: true
          panels:
            armor_class: 15
            hit_points_per_thickness_inch: 30
            damage_immunity:
            - poison
            - psychic
            at_zero_hit_points: destroyed
            connected_panel_collapse:
              chooser: gm
              selection: connected_panels
          duration:
            full_concentration_seconds: 600
            full_concentration_result:
              permanent: true
              dispellable: false
            earlier_spell_end: wall_disappears
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_wall_of_stone
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        panel_design: choice
        collision_sides: choice
        collapse_choice:
          type: choice
          required: false
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - objects
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - world_objects
      - areas
      - tests
      - concentration
    requires:
    - dnd2024:spell_wall_of_stone
    citations:
    - SRD 5.2.1 pp. 173; builtin/srd-5.2.1/spell/wall_of_stone.md
  dnd2024:spells/water_walk:
    source_sha256: 957060b4de82469c148c0ca194217316ff85b59a7a8e56201960000a483e320a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/water_walk
        implementation: dnd2024:spell_water_walk
        parameters:
          spell_lifetime:
            concentration: false
            maximum_seconds: 3600
          targets:
            kind: creature
            willing: true
            chooser: caster
            maximum_count: 10
            range_feet: 30
          movement:
            surface_types:
            - any_liquid
            - water
            - acid
            - mud
            - snow
            - quicksand
            - lava
            treat_surface_as: harmless_solid_ground
            lava_heat_damage: still_possible
          cross_surface:
            directions:
            - surface_to_liquid
            - liquid_to_surface
            economy: bonus_action
          falling_into_liquid:
            passes_through_surface: true
            enters_liquid: true
            bonus_action_required: false
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_water_walk
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - movement
    requires:
    - dnd2024:spell_water_walk
    citations:
    - SRD 5.2.1 pp. 173, 174; builtin/srd-5.2.1/spell/water_walk.md
  dnd2024:spells/wind_wall:
    source_sha256: c81e71cf73d8c31481f53cc0a56cd726a8356b12efd6d3a9bbe3e0a0528d82a3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/wind_wall
        implementation: dnd2024:spell_wind_wall
        parameters:
          spell_lifetime:
            concentration: true
            maximum_seconds: 60
          wall:
            origin: chosen_point_on_ground
            range_feet: 120
            maximum_length_feet: 50
            maximum_height_feet: 15
            shape: caster_choice
            continuous_path_on_ground: true
            wind: strong
            maximum_thickness_feet: 1
          initial:
            targets: all_creatures_in_area
            save:
              ability: strength
              dc: spell_save_dc
            failure_damage:
              dice: 4d8
              type: bludgeoning
            success_damage_scale:
              numerator: 1
              denominator: 2
          blocked:
            gases:
            - fog
            - smoke
            - other_gases
            flying_creatures_and_objects_maximum_size: small
            gaseous_form_creatures: true
          loose_lightweight_material:
            on_enter: fly_upward
          projectiles:
            ordinary:
              examples:
              - arrows
              - bolts
              target_behind_wall:
                deflect: upward
                attack_result: automatic_miss
            exceptions_unaffected:
            - giant_hurled_boulders
            - siege_engine_projectiles
            - similar_projectiles
          spatial_resolution:
            source: supplied_world_geometry
            unknown_facts: require_resolution
            infer_unprovided_geometry: false
    contract:
      id: dnd2024:spell_wind_wall
      inputs:
        caster: participant
        cast_level: integer
        origin: location
        wall_design: choice
        targets: participant_list
        save_dc: integer
        spatial_facts:
          type: resolution
          required: false
      outputs:
      - effects
      - damage
      events:
      - concentration.ended
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - movement
      - areas
      - damage
      - tests
      - concentration
    requires:
    - dnd2024:spell_wind_wall
    citations:
    - SRD 5.2.1 pp. 174; builtin/srd-5.2.1/spell/wind_wall.md
---

# SRD spell conversion research: group 4

All 84 assigned spells have source-specific parameter contracts. No engine is
implemented. The existing spell headers retain casting time, components,
material costs and consumption, range, classes, and ritual availability. Typed
lifetime data is included here; spell-specific exceptions take precedence.
Each definition cites its bundled official SRD 5.2.1 source and includes the
verified SHA-256 of that exact source file.

The contracts cover target limits, saving throws, damage, healing, conditions,
repeat checks, choices, movement, duration, slot scaling, and source exceptions.
Choice inputs describe decisions allowed by the source. Geometry and hidden
world facts must be supplied when required; unknown facts cannot silently pass.
Optional target bindings for creature-or-object spells are mutually exclusive.
Parameters use enumerated symbols for operations and typed numbers and records
for limits. A contract name is an engine dependency, not an implemented engine.

Creation has one unresolved source case: its text allows mineral objects,
including metal, but the duration table has no row for ordinary base metals.
The five listed material categories have exact durations. An unlisted mineral
requires an explicit duration adjudication; no duration is invented here.
Source: SRD 5.2.1 p. 121, `builtin/srd-5.2.1/spell/creation.md`.

The 2024 source controls the less familiar clauses: Conjure Minor Elementals
adds 1d8 per slot above 4; Counterspell uses a Constitution save and does not
expend the interrupted slot; Tiny Hut blocks spells of level 3 or lower; and
True Polymorph retains the specified Hit Points and grants Temporary Hit Points.
Source citations are attached to those definitions above.

Validation: 84 of 84 assignment keys present; all source hashes match; all
canonical entry references resolve; all contract inputs use the closed type
vocabulary. No assigned entry is classified as informational.

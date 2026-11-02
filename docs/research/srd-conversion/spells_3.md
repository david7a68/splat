---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:spells/aid:
    source_sha256: 2b240adf75749bca2df230d77c2981a6bdfb2e03dfba825151a26732d548c133
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/aid
        implementation: dnd2024:spell_aid
        parameters:
          targets:
            kind: creature
            maximum: 3
            within_spell_range: true
          hit_points:
            current_increase:
              op: add
              args:
              - 5
              - op: multiply
                args:
                - 5
                - op: subtract
                  args:
                  - binding: cast_level
                  - 2
            maximum_increase:
              op: add
              args:
              - 5
              - op: multiply
                args:
                - 5
                - op: subtract
                  args:
                  - binding: cast_level
                  - 2
            duration: spell_duration
    contract:
      id: dnd2024:spell_aid
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - health
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - health
    requires:
    - dnd2024:spell_aid
    citations:
    - SRD 5.2.1 pp. 107; builtin/srd-5.2.1/spell/aid.md
  dnd2024:spells/animal_messenger:
    source_sha256: c26eb2f594e43a90b7b92b244da4a8cd612ca5b6f67105cbd3a2940bb7d8fcdb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/animal_messenger
        implementation: dnd2024:spell_animal_messenger
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            visible: true
            size: tiny
            creature_type: beast
          save:
            ability: charisma
            dc:
              binding: save_dc
            failure:
              effect: deliver_message
            success:
              effect: none
          automatic_save_success:
            challenge_rating_not_equal: 0
          message:
            word_limit: 25
            destination:
              visited_by_caster: true
            recipient:
              match: caster_general_description
            delivery: mimic_caster_communication
          travel:
            duration_hours:
              op: add
              args:
              - 24
              - op: multiply
                args:
                - 48
                - op: subtract
                  args:
                  - binding: cast_level
                  - 2
            miles_per_24_hours:
              flying: 50
              otherwise: 25
            rate_precision: approximate
            on_arrival: deliver_to_matching_creature
            on_duration_expired_before_arrival:
              message: lost
              beast_destination: casting_location
            on_arrival_duration_remaining: spell_duration_remaining
    contract:
      id: dnd2024:spell_animal_messenger
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        campaign:
          type: campaign
          required: false
        save_dc: integer
        message:
          type: text
          required: false
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
      outputs:
      - effects
      - knowledge
      - items
      events:
      - spell.cast_completed
      - save.before
      - save.after
      - time.elapsed
      capabilities:
      - spellcasting
      - effects
      - dice
      - saving_throws
      - knowledge
      - inventory
      - behavior_constraints
    requires:
    - dnd2024:spell_animal_messenger
    citations:
    - SRD 5.2.1 pp. 108; builtin/srd-5.2.1/spell/animal_messenger.md
  dnd2024:spells/antilife_shell:
    source_sha256: 1adb185d978c32ccd84bdbc3a70584ab9f2455bf53f7062d6966c83e943be91d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/antilife_shell
        implementation: dnd2024:spell_antilife_shell
        parameters:
          area:
            shape: emanation
            radius_feet: 10
            origin: caster
            duration: spell_duration
          barrier:
            excluded_types:
            - construct
            - undead
            blocks:
            - creature_passing
            - creature_reaching
            allows:
            - spell_casting_through
            - ranged_weapon_attacks_through
            - reach_weapon_attacks_through
          end_when:
            caster_moves_forcing_affected_creature_through: true
    contract:
      id: dnd2024:spell_antilife_shell
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
    requires:
    - dnd2024:spell_antilife_shell
    citations:
    - SRD 5.2.1 pp. 109; builtin/srd-5.2.1/spell/antilife_shell.md
  dnd2024:spells/arcane_hand:
    source_sha256: 3e16a773d8f0a54c758254ee2a6cb0c797ddad253cad6f09a8328a65c62e4e22
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/arcane_hand
        implementation: dnd2024:spell_arcane_hand
        parameters:
          creation:
            kind: object
            size: large
            space:
              unoccupied: true
              visible: true
              within_spell_range: true
            ac: 20
            hp:
              binding: caster_hp_maximum
            occupies_space: false
            end_spell_at_hp: 0
            duration: spell_duration
            appearance: hand
            movement_control: caster_hand_gestures
            hit_point_maximum:
              binding: caster_hp_maximum
          activation:
            initial: spell_cast
            subsequent: bonus_action_on_caster_turn
            move_feet_maximum: 60
            then_choose_effect: 1
          modes:
            clenched_fist:
              target:
                kind: creature_or_object
                distance_from_hand_feet: 5
              attack: melee_spell
              hit:
                dice:
                  count:
                    op: add
                    args:
                    - 5
                    - op: multiply
                      args:
                      - 2
                      - op: subtract
                        args:
                        - binding: cast_level
                        - 5
                  sides: 8
                  add: 0
                type: force
            forceful_hand:
              target:
                kind: creature
                maximum_size: huge
                distance_from_hand_feet: 5
              save:
                ability: strength
                dc:
                  binding: save_dc
                failure:
                  push_feet_maximum:
                    op: add
                    args:
                    - 5
                    - op: multiply
                      args:
                      - 5
                      - binding: spellcasting_modifier
                  hand_follows: true
                  hand_distance_from_target_maximum_feet: 5
                success:
                  effect: none
            grasping_hand:
              target:
                kind: creature
                maximum_size: huge
                distance_from_hand_feet: 5
              save:
                ability: dexterity
                dc:
                  binding: save_dc
                failure:
                  condition: grappled
                  escape_dc:
                    binding: save_dc
                success:
                  effect: none
              crush:
                cost: bonus_action
                requires: hand_grapples_target
                damage:
                  dice:
                    count:
                      op: add
                      args:
                      - 4
                      - op: multiply
                        args:
                        - 2
                        - op: subtract
                          args:
                          - binding: cast_level
                          - 5
                    sides: 6
                    add:
                      binding: spellcasting_modifier
                  type: bludgeoning
            interposing_hand:
              caster_cover: half
              cover_against:
              - originates_in_hand_space
              - passes_through_hand_space
              hand_space_difficult_terrain_for: caster_enemies
    contract:
      id: dnd2024:spell_arcane_hand
      inputs:
        caster: participant
        cast_level: integer
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        caster_hp_maximum: integer
        save_dc: integer
        spellcasting_modifier: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      - attacks
      - participants
      events:
      - spell.cast_completed
      - effect.ended
      - movement.completed
      - save.before
      - save.after
      - hp.changed
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - attacks
      - spatial_relationships
      - conditions
      - saving_throws
      - participant_lifecycle
    requires:
    - dnd2024:spell_arcane_hand
    citations:
    - SRD 5.2.1 pp. 110; builtin/srd-5.2.1/spell/arcane_hand.md
  dnd2024:spells/astral_projection:
    source_sha256: d23ba9b8f1252fa2fcc1a588291364cc8d0fcea2e0ffc18d3a595ed9c9bb0047
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/astral_projection
        implementation: dnd2024:spell_astral_projection
        parameters:
          targets:
            kind: creature
            maximum: 8
            within_spell_range: true
            willing: true
            additional_target: caster
          already_astral_plane:
            end_immediately: true
          projection:
            plane: astral
            copy:
            - game_statistics
            - possessions
            - body_appearance
            physical_body:
              condition: unconscious
              needs_food: false
              needs_air: false
              ages: false
            silver_cord:
              visible_length_feet: 1
              origin: between_shoulders
              cut_only_by_explicit_effect: true
              cut_effect: kill_body_and_astral_form
            damage_and_effects_shared_between_forms: false
            astral_plane_exit:
              physical_body_and_possessions: travel_along_cord
              reenter_body: new_plane
          end:
            per_target:
            - body_hp_zero
            - astral_form_hp_zero
            all_targets:
              caster_cost: magic_action
              choice: dismiss
            living_target_cleanup:
            - return_to_body
            - end_suspended_animation
    contract:
      id: dnd2024:spell_astral_projection
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      - participants
      - items
      events:
      - spell.cast_completed
      - damage.applied
      - effect.ended
      - movement.completed
      - hp.changed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - conditions
      - participant_lifecycle
      - inventory
      - forms
    requires:
    - dnd2024:spell_astral_projection
    citations:
    - SRD 5.2.1 pp. 111; builtin/srd-5.2.1/spell/astral_projection.md
  dnd2024:spells/bane:
    source_sha256: 3890a75e78bf9cc7253428770999ee00876e4279f65606aad08ed5c62000e89a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/bane
        implementation: dnd2024:spell_bane
        parameters:
          targets:
            kind: creature
            maximum:
              op: add
              args:
              - 3
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 1
            within_spell_range: true
            visible: true
          save:
            ability: charisma
            dc:
              binding: save_dc
            failure:
              roll_modifier:
                rolls:
                - attack
                - save
                amount:
                  dice:
                    count: 1
                    sides: 4
                    add: 0
                operation: subtract
                duration: spell_duration
            success:
              effect: none
    contract:
      id: dnd2024:spell_bane
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - attack.roll
      - attack.hit
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - saving_throws
      - roll_modifiers
    requires:
    - dnd2024:spell_bane
    citations:
    - SRD 5.2.1 pp. 112; builtin/srd-5.2.1/spell/bane.md
  dnd2024:spells/befuddlement:
    source_sha256: 92fa5da3bf05346cc01aa99bf0af7a5d9f34d35b4112acd0e1fb9976b21936d1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/befuddlement
        implementation: dnd2024:spell_befuddlement
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            visible: true
          save:
            ability: intelligence
            dc:
              binding: save_dc
            failure:
              damage:
                dice:
                  count: 10
                  sides: 12
                  add: 0
                type: psychic
              prohibit:
              - cast_spells
              - magic_action
              repeat_save:
                interval_days: 30
                success: end_effect
              end_by_spells:
              - key: dnd2024:spells/greater_restoration
              - key: dnd2024:spells/heal
              - key: dnd2024:spells/wish
            success:
              damage_fraction:
                numerator: 1
                denominator: 2
              damage:
                dice:
                  count: 10
                  sides: 12
                  add: 0
                type: psychic
    contract:
      id: dnd2024:spell_befuddlement
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - save.before
      - save.after
      - time.elapsed
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - saving_throws
    requires:
    - dnd2024:spell_befuddlement
    citations:
    - SRD 5.2.1 pp. 112; builtin/srd-5.2.1/spell/befuddlement.md
  dnd2024:spells/bless:
    source_sha256: d10906d086afa777b605b3fcf6b6f51dd49f7927c5e5116820acb6f973a4043f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/bless
        implementation: dnd2024:spell_bless
        parameters:
          targets:
            kind: creature
            maximum:
              op: add
              args:
              - 3
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 1
            within_spell_range: true
          roll_modifier:
            rolls:
            - attack
            - save
            amount:
              dice:
                count: 1
                sides: 4
                add: 0
            operation: add
            duration: spell_duration
    contract:
      id: dnd2024:spell_bless
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - attack.roll
      - attack.hit
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - saving_throws
      - roll_modifiers
    requires:
    - dnd2024:spell_bless
    citations:
    - SRD 5.2.1 pp. 113; builtin/srd-5.2.1/spell/bless.md
  dnd2024:spells/blur:
    source_sha256: a5b1db7dd1588ca7274033ad055bba36c723a43950705f3b3f4066ff9709fba5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/blur
        implementation: dnd2024:spell_blur
        parameters:
          targets:
            kind: self
          incoming_attacks:
            disadvantage: true
            duration: spell_duration
            immune_if_attacker_perceives_caster_using:
            - blindsight
            - truesight
    contract:
      id: dnd2024:spell_blur
      inputs:
        caster: participant
        cast_level: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - attack.roll
      - attack.hit
      capabilities:
      - spellcasting
      - effects
      - roll_modifiers
    requires:
    - dnd2024:spell_blur
    citations:
    - SRD 5.2.1 pp. 114; builtin/srd-5.2.1/spell/blur.md
  dnd2024:spells/chain_lightning:
    source_sha256: eb280ef3fe0c70b4d7a9e241abf2e6294b5b7344348e98c3dde2628642f0d642
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/chain_lightning
        implementation: dnd2024:spell_chain_lightning
        parameters:
          targets:
            primary:
              kind: creature_or_object
              visible: true
              within_spell_range: true
            secondary:
              kind: creature_or_object
              maximum:
                op: add
                args:
                - 3
                - op: multiply
                  args:
                  - 1
                  - op: subtract
                    args:
                    - binding: cast_level
                    - 6
              distance_from_primary_maximum_feet: 30
              chooser: caster
            maximum_bolts_per_target: 1
          save:
            ability: dexterity
            dc:
              binding: save_dc
            failure:
              damage:
                dice:
                  count: 10
                  sides: 8
                  add: 0
                type: lightning
            success:
              damage:
                dice:
                  count: 10
                  sides: 8
                  add: 0
                type: lightning
              damage_fraction:
                numerator: 1
                denominator: 2
    contract:
      id: dnd2024:spell_chain_lightning
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - spatial_relationships
      - saving_throws
    requires:
    - dnd2024:spell_chain_lightning
    citations:
    - SRD 5.2.1 pp. 114; builtin/srd-5.2.1/spell/chain_lightning.md
  dnd2024:spells/chromatic_orb:
    source_sha256: 9b2f6235ac7c0a5c86b51b799bb37111300953d6fe2abf6c94eba1d89bbd2a7c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/chromatic_orb
        implementation: dnd2024:spell_chromatic_orb
        parameters:
          targets:
            kind: creature_or_object
            within_spell_range: true
          damage_type_choice:
          - acid
          - cold
          - fire
          - lightning
          - poison
          - thunder
          attack:
            kind: ranged_spell
            hit:
              dice:
                count:
                  op: add
                  args:
                  - 3
                  - op: multiply
                    args:
                    - 1
                    - op: subtract
                      args:
                      - binding: cast_level
                      - 1
                sides: 8
                add: 0
              type: chosen
          leap:
            trigger:
              damage_roll_d8_matching_faces_minimum: 2
            distance_from_previous_target_maximum_feet: 30
            target: different_target_caster_choice
            repeat_attack_and_damage_roll: true
            maximum_leaps:
              binding: cast_level
            maximum_targets_per_creature_per_cast: 1
    contract:
      id: dnd2024:spell_chromatic_orb
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - attacks
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - dice
      - attacks
      - spatial_relationships
    requires:
    - dnd2024:spell_chromatic_orb
    citations:
    - SRD 5.2.1 pp. 115; builtin/srd-5.2.1/spell/chromatic_orb.md
  dnd2024:spells/cloudkill:
    source_sha256: 8747c2f8bbf623b3f881e94c26a11ba7d27664b7ec871f185bd3cb6d0e69d954
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/cloudkill
        implementation: dnd2024:spell_cloudkill
        parameters:
          area:
            shape: sphere
            radius_feet: 20
            origin: chosen_point_within_range
            obscurement: heavy
            duration: spell_duration
          save:
            ability: constitution
            dc:
              binding: save_dc
            failure:
              damage:
                dice:
                  count:
                    op: add
                    args:
                    - 5
                    - op: multiply
                      args:
                      - 1
                      - op: subtract
                        args:
                        - binding: cast_level
                        - 5
                  sides: 8
                  add: 0
                type: poison
            success:
              damage:
                dice:
                  count:
                    op: add
                    args:
                    - 5
                    - op: multiply
                      args:
                      - 1
                      - op: subtract
                        args:
                        - binding: cast_level
                        - 5
                  sides: 8
                  add: 0
                type: poison
              damage_fraction:
                numerator: 1
                denominator: 2
          save_triggers:
          - initial_in_area
          - sphere_moves_into_creature_space
          - creature_enters_area
          - creature_turn_ends_in_area
          save_limit:
            per_creature_per_turn: 1
          movement:
            event: caster_turn_start
            feet: 10
            direction: away_from_caster
          end_when:
            strong_wind_disperses_fog: true
    contract:
      id: dnd2024:spell_cloudkill
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - turn.start
      - effect.ended
      - movement.completed
      - turn.end
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - spatial_relationships
      - saving_throws
    requires:
    - dnd2024:spell_cloudkill
    citations:
    - SRD 5.2.1 pp. 116; builtin/srd-5.2.1/spell/cloudkill.md
  dnd2024:spells/commune_with_nature:
    source_sha256: 2d4ce2f022b84e11ef5d8084920450bbed639b58a35dfced653a1657497769bd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/commune_with_nature
        implementation: dnd2024:spell_commune_with_nature
        parameters:
          area:
            outdoors_radius_miles: 3
            natural_underground_radius_feet: 300
            fails_where_nature_replaced_by_construction: true
          knowledge:
            choose_facts_count: 3
            choices:
              settlements:
                output: locations
              planar_portals:
                output: locations
              powerful_creature:
                minimum_challenge_rating: 10
                creature_types:
                - celestial
                - elemental
                - fey
                - fiend
                - undead
                chooser: gm
                count: 1
                output: location
              most_prevalent_kind:
                caster_category_choice:
                - plant
                - mineral
                - beast
                output: kind
              bodies_of_water:
                output: locations
    contract:
      id: dnd2024:spell_commune_with_nature
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - knowledge
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - knowledge
    requires:
    - dnd2024:spell_commune_with_nature
    citations:
    - SRD 5.2.1 pp. 116, 117; builtin/srd-5.2.1/spell/commune_with_nature.md
  dnd2024:spells/confusion:
    source_sha256: e718bd12d87df87b3302cca3d64d639afb1ca3d3f53fa903535e93b7f139c6bb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/confusion
        implementation: dnd2024:spell_confusion
        parameters:
          area:
            shape: sphere
            radius_feet:
              op: add
              args:
              - 10
              - op: multiply
                args:
                - 5
                - op: subtract
                  args:
                  - binding: cast_level
                  - 4
            origin: chosen_point_within_range
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              prohibit:
              - bonus_action
              - reaction
              duration: spell_duration
              turn_start_roll:
                dice:
                  dice:
                    count: 1
                    sides: 10
                    add: 0
                table:
                - range:
                  - 1
                  - 1
                  action: none
                  movement: all_available
                  direction_roll:
                    dice:
                      dice:
                        count: 1
                        sides: 4
                        add: 0
                    faces:
                      '1': north
                      '2': east
                      '3': south
                      '4': west
                - range:
                  - 2
                  - 6
                  action: none
                  movement: none
                - range:
                  - 7
                  - 8
                  movement: none
                  action: attack
                  melee_attacks: 1
                  target: random_creature_within_reach
                  without_target_action: none
                - range:
                  - 9
                  - 10
                  behavior: target_choice
              repeat_save:
                event: target_turn_end
                success: end_effect_on_target
            success:
              effect: none
    contract:
      id: dnd2024:spell_confusion
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - turn.start
      - turn.end
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - spatial_relationships
      - saving_throws
      - behavior_constraints
    requires:
    - dnd2024:spell_confusion
    citations:
    - SRD 5.2.1 pp. 117; builtin/srd-5.2.1/spell/confusion.md
  dnd2024:spells/conjure_fey:
    source_sha256: 274232b54b4777fb0309953b6b09ad4a06059e711286996d392101f717c33c95
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/conjure_fey
        implementation: dnd2024:spell_conjure_fey
        parameters:
          creation:
            kind: spirit
            size: medium
            origin_plane: feywild
            appearance_choice: fey_creature
            space:
              unoccupied: true
              visible: true
              within_spell_range: true
            duration: spell_duration
          attack:
            optional: true
            target:
              kind: creature
              distance_from_spirit_maximum_feet: 5
            kind: melee_spell
            hit:
              damage:
                dice:
                  count:
                    op: add
                    args:
                    - 3
                    - op: multiply
                      args:
                      - 1
                      - op: subtract
                        args:
                        - binding: cast_level
                        - 6
                  sides: 12
                  add:
                    binding: spellcasting_modifier
                type: psychic
              condition: frightened
              fear_sources:
              - caster
              - spirit
              until: caster_next_turn_start
          activation:
            initial: spirit_appears
            later:
              cost: bonus_action
              'on': caster_turn
              teleport_before_attack:
                maximum_feet: 30
                destination_unoccupied: true
                destination_visible_to_caster: true
    contract:
      id: dnd2024:spell_conjure_fey
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        spellcasting_modifier: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      - attacks
      - participants
      events:
      - spell.cast_completed
      - movement.completed
      - turn.start
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - attacks
      - spatial_relationships
      - conditions
      - participant_lifecycle
    requires:
    - dnd2024:spell_conjure_fey
    citations:
    - SRD 5.2.1 pp. 118; builtin/srd-5.2.1/spell/conjure_fey.md
  dnd2024:spells/contagion:
    source_sha256: 466507530a6342b0ffaa63f4d3ffc1ca20d07a2bb45f4b01692a88532f47ada2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/contagion
        implementation: dnd2024:spell_contagion
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
          ability_choice:
          - strength
          - dexterity
          - constitution
          - intelligence
          - wisdom
          - charisma
          save:
            ability: constitution
            dc:
              binding: save_dc
            failure:
              damage:
                dice:
                  count: 11
                  sides: 8
                  add: 0
                type: necrotic
              condition: poisoned
              while_poisoned:
                saving_throw_disadvantage_ability: chosen
              repeat_save:
                event: target_turn_end
                successes_to_end: 3
                failures_to_stop_repeating_and_persist: 3
                count_initial_save: false
                duration_days: 7
              poison_removal_attempt:
                save:
                  ability: constitution
                  dc:
                    binding: save_dc
                  failure:
                    prevent_condition_removal: true
                  success:
                    allow_condition_removal: true
            success:
              effect: none
    contract:
      id: dnd2024:spell_contagion
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - turn.end
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - conditions
      - saving_throws
      - roll_modifiers
    requires:
    - dnd2024:spell_contagion
    citations:
    - SRD 5.2.1 pp. 119; builtin/srd-5.2.1/spell/contagion.md
  dnd2024:spells/control_weather:
    source_sha256: 268edcc932c452655b876e8ddc0981bc9c9b2b4d2290720b437e5640faafba1a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/control_weather
        implementation: dnd2024:spell_control_weather
        parameters:
          requires:
            caster_outdoors: true
          area:
            radius_miles: 5
            origin: caster
            duration: spell_duration
          end_when:
            caster_goes_indoors: true
          weather:
            initial_conditions_chooser: gm
            change_each_stage_by_maximum: 1
            change_directions:
            - up
            - down
            change_delay_minutes:
              roll:
                dice:
                  count: 1
                  sides: 4
                  add: 0
              multiply: 10
            next_change_requires: previous_change_took_effect
            wind_direction_change_allowed: true
            on_spell_end:
              return_to_normal: gradual
              transition_duration:
                chooser: gm
                source_unspecified: true
            precipitation_stages:
            - - clear
            - - light_clouds
            - - overcast
              - ground_fog
            - - rain
              - hail
              - snow
            - - torrential_rain
              - driving_hail
              - blizzard
            temperature_stages:
            - heat_wave
            - hot
            - warm
            - cool
            - cold
            - freezing
            wind_stages:
            - calm
            - moderate
            - strong
            - gale
            - storm
    contract:
      id: dnd2024:spell_control_weather
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - movement.completed
      - time.elapsed
      capabilities:
      - spellcasting
      - effects
      - dice
      - spatial_relationships
      - conditions
      - weather
    requires:
    - dnd2024:spell_control_weather
    citations:
    - SRD 5.2.1 pp. 120; builtin/srd-5.2.1/spell/control_weather.md
  dnd2024:spells/create_undead:
    source_sha256: 2169a8177417aa52f0d7d27e854740be7b7dac16ee53e31e006be5d55c0ddc3b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/create_undead
        implementation: dnd2024:spell_create_undead
        parameters:
          requires:
            time: night
          modes:
          - animate
          - reassert_control
          targets:
            animate:
              kind: corpse
              former_type: humanoid
              sizes:
              - medium
              - small
              within_spell_range: true
            reassert_control:
              kind: creature_animated_by_this_spell
              control_not_expired: true
              within_spell_range: true
          slot_options:
            '6':
            - kind: ghoul
              maximum: 3
            '7':
            - kind: ghoul
              maximum: 4
            '8':
            - kind: ghoul
              maximum: 5
            - kinds:
              - ghast
              - wight
              maximum: 2
            '9':
            - kind: ghoul
              maximum: 6
            - kinds:
              - ghast
              - wight
              maximum: 3
            - kind: mummy
              maximum: 2
          control:
            duration_hours: 24
            renew_duration_hours: 24
            expiry: stop_obeying_orders
            command:
              cost: bonus_action
              event: caster_turn
              range_feet: 120
              communication: mental
              multiple_targets_same_command: true
              choices:
              - next_turn_action_and_movement
              - general_task
              persists_until: task_complete
            no_command:
              action: dodge
              movement: avoid_harm_only
          stat_blocks:
            ghoul:
              key: dnd2024:creatures/ghoul
            ghast:
              key: dnd2024:creatures/ghast
            wight:
              key: dnd2024:creatures/wight
            mummy:
              key: dnd2024:creatures/mummy
    contract:
      id: dnd2024:spell_create_undead
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
      outputs:
      - effects
      - participants
      - items
      events:
      - spell.cast_completed
      - rest.completed
      - movement.completed
      - time.elapsed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - participant_lifecycle
      - inventory
    requires:
    - dnd2024:spell_create_undead
    citations:
    - SRD 5.2.1 pp. 121; builtin/srd-5.2.1/spell/create_undead.md
  dnd2024:spells/darkness:
    source_sha256: 7a177cabbba22ff1e01b06f2aa003e6a7007fc7f28da31c805c6fd2da64aac93
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/darkness
        implementation: dnd2024:spell_darkness
        parameters:
          area_modes:
            point:
              shape: sphere
              radius_feet: 15
              within_spell_range: true
            object:
              shape: emanation
              radius_feet: 15
              within_spell_range: true
              object_worn: false
              object_carried: false
              blocked_while_object_covered_by_opaque_cover: true
          area:
            illumination: magical_darkness
            duration: spell_duration
            darkvision_sees_through: false
            nonmagical_light_illuminates: false
          overlap:
            light_levels:
            - bright
            - dim
            created_by_spell_maximum_level: 2
            effect: dispel_other_spell
    contract:
      id: dnd2024:spell_darkness
      inputs:
        caster: participant
        cast_level: integer
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
      outputs:
      - effects
      - items
      events:
      - spell.cast_completed
      - effect.ended
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - inventory
    requires:
    - dnd2024:spell_darkness
    citations:
    - SRD 5.2.1 pp. 122; builtin/srd-5.2.1/spell/darkness.md
  dnd2024:spells/delayed_blast_fireball:
    source_sha256: 633a81a770c49206e99ccf3981ebe0868962fc7d8b8f2df530280ef7b26aeee3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/delayed_blast_fireball
        implementation: dnd2024:spell_delayed_blast_fireball
        parameters:
          bead:
            initial_location: chosen_point_within_range
            duration: spell_duration
            base_damage:
              dice:
                count:
                  op: add
                  args:
                  - 12
                  - op: multiply
                    args:
                    - 1
                    - op: subtract
                      args:
                      - binding: cast_level
                      - 7
                sides: 6
                add: 0
              type: fire
            accumulate:
              event: caster_turn_end
              requires: spell_not_ended
              additional_dice:
                dice:
                  count: 1
                  sides: 6
                  add: 0
            touch:
              save:
                ability: dexterity
                dc:
                  binding: save_dc
                failure:
                  end_spell: true
                success:
                  throw_optional: true
                  throw_maximum_feet: 40
            thrown_collision_end_spell:
            - creature_space
            - solid_object
          explosion:
            trigger: spell_ends
            origin: current_bead_location
            shape: sphere
            radius_feet: 20
            targets: each_creature_in_area
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage: accumulated
              success:
                damage: accumulated
                damage_fraction:
                  numerator: 1
                  denominator: 2
            ignite:
              kind: flammable_object
              worn: false
              carried: false
              in_area: true
    contract:
      id: dnd2024:spell_delayed_blast_fireball
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - turn.end
      - effect.ended
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - spatial_relationships
      - saving_throws
    requires:
    - dnd2024:spell_delayed_blast_fireball
    citations:
    - SRD 5.2.1 pp. 122; builtin/srd-5.2.1/spell/delayed_blast_fireball.md
  dnd2024:spells/detect_poison_and_disease:
    source_sha256: ddf9833bd35574275c913ddbd272d03e5155e429bbc19a05876370af061d2eb1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/detect_poison_and_disease
        implementation: dnd2024:spell_detect_poison_and_disease
        parameters:
          sense:
            radius_feet: 30
            origin: caster
            duration: spell_duration
            categories:
            - poison
            - poisonous_creature
            - venomous_creature
            - magical_contagion
            outputs:
            - location
            - kind
            barrier_blocks:
            - materials:
              - stone
              - dirt
              - wood
              minimum_thickness_feet: 1
            - material: metal
              minimum_thickness_inches: 1
            - material: lead
              any_thin_sheet: true
    contract:
      id: dnd2024:spell_detect_poison_and_disease
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - knowledge
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - knowledge
    requires:
    - dnd2024:spell_detect_poison_and_disease
    citations:
    - SRD 5.2.1 pp. 123; builtin/srd-5.2.1/spell/detect_poison_and_disease.md
  dnd2024:spells/disintegrate:
    source_sha256: 0ba2a2cc17aa48a12d21a2375fc79ade07c707c7491e482edf518c8ce52df9d5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/disintegrate
        implementation: dnd2024:spell_disintegrate
        parameters:
          targets:
            kind_choices:
            - creature
            - nonmagical_object
            - magical_force_creation
            visible: true
            within_spell_range: true
          creature:
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  dice:
                    count:
                      op: add
                      args:
                      - 10
                      - op: multiply
                        args:
                        - 3
                        - op: subtract
                          args:
                          - binding: cast_level
                          - 6
                    sides: 6
                    add: 40
                  type: force
                if_damage_reduces_to_zero_hp:
                  transform_to: gray_dust
                  include_worn_carried_nonmagical_items: true
                  revival_spells_only:
                  - key: dnd2024:spells/true_resurrection
                  - key: dnd2024:spells/wish
              success:
                effect: none
          object_or_force:
            automatic_disintegration: true
            maximum_size_for_whole_target: large
            huge_or_larger:
              portion_shape: cube
              side_feet: 10
    contract:
      id: dnd2024:spell_disintegrate
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
      outputs:
      - effects
      - damage
      - items
      events:
      - spell.cast_completed
      - save.before
      - save.after
      - hp.changed
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - saving_throws
      - inventory
    requires:
    - dnd2024:spell_disintegrate
    citations:
    - SRD 5.2.1 pp. 124; builtin/srd-5.2.1/spell/disintegrate.md
  dnd2024:spells/divination:
    source_sha256: 2c590fd93993103148985dfd41d3458ff3507cc06e29cd468bd285e80460c23e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/divination
        implementation: dnd2024:spell_divination
        parameters:
          question:
            maximum: 1
            subjects:
            - specific_goal
            - event
            - activity
            future_limit_days: 7
          reply:
            provider: gm
            truthful: true
            form_choices:
            - short_phrase
            - cryptic_rhyme
            accounts_for_later_changed_circumstances: false
          repeat_cast:
            reset: caster_long_rest_completed
            no_answer_chance_percent_per_cast_after_first: 25
            cumulative: true
    contract:
      id: dnd2024:spell_divination
      inputs:
        caster: participant
        cast_level: integer
        campaign:
          type: campaign
          required: false
        message:
          type: text
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - knowledge
      events:
      - spell.cast_completed
      - rest.completed
      capabilities:
      - spellcasting
      - effects
      - knowledge
    requires:
    - dnd2024:spell_divination
    citations:
    - SRD 5.2.1 pp. 125; builtin/srd-5.2.1/spell/divination.md
  dnd2024:spells/dominate_beast:
    source_sha256: dc3eb37f86506c62613d5b8ac3604bd324f162ce1810cae4aed265955a415ce2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/dominate_beast
        implementation: dnd2024:spell_dominate_beast
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            visible: true
            creature_type: beast
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              condition: charmed
              duration: spell_duration
            success:
              effect: none
          save_advantage_if:
            caster_or_allies_fighting_target: true
          repeat_save:
            event: target_takes_damage
            ability: wisdom
            success: end_spell_on_target
          control:
            requires: target_charmed_by_spell
            telepathy:
              requires_same_plane: true
            command:
              event: caster_turn
              cost: none
              obey: best_effort_on_target_turn
            completed_without_new_order:
              behavior: target_choice
              priority: self_protection
            reaction_command:
              cost: caster_reaction
              target_cost: reaction
          duration_by_slot:
            '4':
              minutes: 1
            '5':
              minutes: 10
            '6':
              hours: 1
            7_plus:
              hours: 8
          concentration: true
    contract:
      id: dnd2024:spell_dominate_beast
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - damage.applied
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - conditions
      - saving_throws
      - roll_modifiers
      - behavior_constraints
    requires:
    - dnd2024:spell_dominate_beast
    citations:
    - SRD 5.2.1 pp. 125; builtin/srd-5.2.1/spell/dominate_beast.md
  dnd2024:spells/dream:
    source_sha256: ffbffa386a82b708f5f0c1746f75267856a8ae33ebf169c5f0bf2a72cdbcb52c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/dream
        implementation: dnd2024:spell_dream
        parameters:
          targets:
            dream_target:
              kind: creature
              known_to_caster: true
              same_plane_as_caster: true
            messenger:
              choices:
              - caster
              - willing_touched_creature
          trance:
            condition: incapacitated
            speed: 0
            end_voluntarily_any_time: true
            on_end: end_spell
          dream:
            requires: target_asleep
            conversation_until:
            - target_wakes
            - spell_ends
            messenger_can_shape:
            - landscapes
            - objects
            - images
            target_recall: perfect
            target_initially_awake:
              messenger_knows: true
              choices:
              - end_trance
              - wait_for_sleep
          terrifying_option:
            message_word_limit: 10
            save_after_message:
              ability: wisdom
              dc:
                binding: save_dc
              failure:
                rest_benefits: false
                damage_when_target_wakes:
                  dice:
                    count: 3
                    sides: 6
                    add: 0
                  type: psychic
              success:
                effect: none
    contract:
      id: dnd2024:spell_dream
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        message:
          type: text
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - knowledge
      events:
      - spell.cast_completed
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - spatial_relationships
      - conditions
      - saving_throws
      - knowledge
    requires:
    - dnd2024:spell_dream
    citations:
    - SRD 5.2.1 pp. 126; builtin/srd-5.2.1/spell/dream.md
  dnd2024:spells/elementalism:
    source_sha256: ef526021b2e4b5ed125882693f8e04e548e0a32b5d662428c964d6e7512fb5ca
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/elementalism
        implementation: dnd2024:spell_elementalism
        parameters:
          selection:
            effect_count: 1
            within_spell_range: true
          modes:
            beckon_air:
              shape: cube
              side_feet: 5
              effects:
              - ripple_cloth
              - stir_dust
              - rustle_leaves
              - close_doors
              - close_shutters
              doors_shutters_held_open_immune: true
            beckon_earth:
              choices:
              - cover: thin_dust_or_sand
                shape: square
                side_feet: 5
              - write_words_maximum: 1
                medium:
                - dirt
                - sand
                handwriting: caster
            beckon_fire:
              shape: cube
              side_feet: 5
              embers_harmless: true
              smoke_color_choice: true
              smoke_scent_choice: true
              scent_duration_minutes: 1
              can_light:
              - candles
              - torches
              - lamps
            beckon_water:
              choices:
              - shape: cube
                side_feet: 5
                effect: lightly_dampen_creatures_and_objects
                temperature: cool
              - clean_water_cups: 1
                destination:
                - open_container
                - surface
                evaporates_after_minutes: 1
            sculpt_element:
              materials:
              - dirt
              - sand
              - fire
              - smoke
              - mist
              - water
              maximum_cube_side_feet: 1
              shape_choice: crude
              duration_hours: 1
    contract:
      id: dnd2024:spell_elementalism
      inputs:
        caster: participant
        cast_level: integer
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
      outputs:
      - effects
      - items
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - inventory
    requires:
    - dnd2024:spell_elementalism
    citations:
    - SRD 5.2.1 pp. 127; builtin/srd-5.2.1/spell/elementalism.md
  dnd2024:spells/entangle:
    source_sha256: fd0854f53d3e0430733f951f99997b1b8a2f534c673e13707e4b0308e22c65e7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/entangle
        implementation: dnd2024:spell_entangle
        parameters:
          area:
            shape: square
            side_feet: 20
            origin: ground_within_spell_range
            terrain: difficult
            duration: spell_duration
            plants_disappear_on_end: true
          targets:
            kind: creature
            in_area_at_cast: true
            exclude_caster: true
          save:
            ability: strength
            dc:
              binding: save_dc
            failure:
              condition: restrained
              until: spell_ends
            success:
              effect: none
          escape:
            cost: action
            check:
              ability: strength
              skill: athletics
              dc:
                binding: save_dc
            success: end_restrained_from_spell
    contract:
      id: dnd2024:spell_entangle
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - turn.end
      - effect.ended
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - spatial_relationships
      - conditions
      - saving_throws
    requires:
    - dnd2024:spell_entangle
    citations:
    - SRD 5.2.1 pp. 128; builtin/srd-5.2.1/spell/entangle.md
  dnd2024:spells/eyebite:
    source_sha256: 38c2b688b287dc02f9df6d517a49409b8ae0f7cfc1d508ce6ad4ee6f2c9e49d6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/eyebite
        implementation: dnd2024:spell_eyebite
        parameters:
          targets:
            kind: creature
            visible: true
            maximum_range_feet: 60
            count_per_activation: 1
            exclude_previous_successful_savers_this_cast: true
          activation:
            initial: spell_cast
            subsequent:
              cost: magic_action
              'on': caster_turn
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              chosen_effect_duration: spell_duration
            success:
              effect: none
          modes:
            asleep:
              condition: unconscious
              end_if:
              - target_takes_damage
              - another_creature_uses_action_to_shake_awake
            panicked:
              condition: frightened
              fear_source: caster
              each_target_turn:
                action: dash
                movement: away_from_caster
                route:
                - safest
                - shortest
              end_if:
                distance_from_caster_feet_minimum: 60
                cannot_see_caster: true
            sickened:
              condition: poisoned
    contract:
      id: dnd2024:spell_eyebite
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - damage.applied
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - spatial_relationships
      - conditions
      - saving_throws
    requires:
    - dnd2024:spell_eyebite
    citations:
    - SRD 5.2.1 pp. 129; builtin/srd-5.2.1/spell/eyebite.md
  dnd2024:spells/false_life:
    source_sha256: a2eb9a40132c7252454877da273cb2585467c0dde92f5b4eb8178f79afefd631
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/false_life
        implementation: dnd2024:spell_false_life
        parameters:
          targets:
            kind: self
          temporary_hp:
            dice:
              count: 2
              sides: 4
              add:
                op: add
                args:
                - 4
                - op: multiply
                  args:
                  - 5
                  - op: subtract
                    args:
                    - binding: cast_level
                    - 1
    contract:
      id: dnd2024:spell_false_life
      inputs:
        caster: participant
        cast_level: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - health
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - dice
      - health
    requires:
    - dnd2024:spell_false_life
    citations:
    - SRD 5.2.1 pp. 129; builtin/srd-5.2.1/spell/false_life.md
  dnd2024:spells/find_steed:
    source_sha256: 176717cb54d1b7bd48bbb321498b0a532d80052c02eba34cc6ce555c27465eb9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/find_steed
        implementation: dnd2024:spell_find_steed
        parameters:
          summon:
            space:
              unoccupied: true
              within_spell_range: true
              chooser: caster
            replace_existing_steed_from_spell: true
            appearance:
              size: large
              form: rideable_animal
              chooser: caster
            creature_type_choice:
            - celestial
            - fey
            - fiend
            alignment: neutral
            ally_to:
            - caster
            - caster_allies
            returning_steed_identity_choice:
            - previous
            - different
          combat:
            initiative: caster_count
            while_ridden: controlled_mount
            while_caster_incapacitated:
              turn: immediately_after_caster
              control: independent
              priority: protect_caster
          disappear:
            triggers:
            - steed_hp_zero
            - caster_death
            leave_worn_and_carried_items: true
          statistics:
            ac:
              op: add
              args:
              - 12
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 2
            hp:
              op: add
              args:
              - 25
              - op: multiply
                args:
                - 10
                - op: subtract
                  args:
                  - binding: cast_level
                  - 2
            hit_dice:
              dice:
                count:
                  binding: cast_level
                sides: 10
                add: 0
            speed_feet: 60
            fly:
              speed_feet: 60
              minimum_cast_level: 4
            abilities:
              strength: 18
              dexterity: 12
              constitution: 14
              intelligence: 6
              wisdom: 12
              charisma: 8
            save_modifiers:
              strength: 4
              dexterity: 1
              constitution: 2
              intelligence: -2
              wisdom: 1
              charisma: -1
            passive_perception: 11
            telepathy:
              miles: 1
              only_with: caster
            challenge_rating: null
            xp: 0
            proficiency_bonus:
              binding: caster_proficiency_bonus
          life_bond:
            event: caster_regains_hp_from_level_1_plus_spell
            caster_distance_maximum_feet: 5
            steed_healing: same_hp_amount
          actions:
            otherworldly_slam:
              attack: melee
              attack_bonus:
                binding: spell_attack_modifier
              reach_feet: 5
              damage:
                dice:
                  count: 1
                  sides: 8
                  add:
                    binding: cast_level
                type: creature_type_dependent
              damage_type_by_creature_type:
                celestial: radiant
                fey: psychic
                fiend: necrotic
          bonus_actions:
            fell_glare:
              creature_type: fiend
              uses: 1
              recharge: long_rest
              target:
                count: 1
                visible_to_steed: true
                range_feet: 60
              save:
                ability: wisdom
                dc:
                  binding: save_dc
                failure:
                  condition: frightened
                  until: caster_next_turn_end
                success:
                  effect: none
            fey_step:
              creature_type: fey
              uses: 1
              recharge: long_rest
              teleport:
                subjects:
                - steed
                - rider
                range_feet: 60
                destination_unoccupied: true
                chooser: caster
            healing_touch:
              creature_type: celestial
              uses: 1
              recharge: long_rest
              target:
                count: 1
                range_feet: 5
              healing:
                dice:
                  count: 2
                  sides: 8
                  add:
                    binding: cast_level
    contract:
      id: dnd2024:spell_find_steed
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        caster_proficiency_bonus: integer
        save_dc: integer
        spell_attack_modifier: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      - attacks
      - participants
      events:
      - spell.cast_completed
      - rest.completed
      - effect.ended
      - movement.completed
      - turn.end
      - save.before
      - save.after
      - hp.changed
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - attacks
      - spatial_relationships
      - conditions
      - saving_throws
      - participant_lifecycle
    requires:
    - dnd2024:spell_find_steed
    citations:
    - SRD 5.2.1 pp. 130, 131; builtin/srd-5.2.1/spell/find_steed.md
  dnd2024:spells/fire_bolt:
    source_sha256: 9ed755dd2c91cd5c5b50577b27b71242ffc4e97a25facba9286c28927f4fa6ee
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/fire_bolt
        implementation: dnd2024:spell_fire_bolt
        parameters:
          targets:
            kind: creature_or_object
            within_spell_range: true
          attack:
            kind: ranged_spell
            hit:
              damage:
                dice:
                  count:
                    by_caster_level:
                    - minimum: 1
                      count: 1
                    - minimum: 5
                      count: 2
                    - minimum: 11
                      count: 3
                    - minimum: 17
                      count: 4
                  sides: 10
                  add: 0
                type: fire
              ignite_if:
                target_flammable: true
                target_worn: false
                target_carried: false
    contract:
      id: dnd2024:spell_fire_bolt
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        caster_level: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      - attacks
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - attacks
    requires:
    - dnd2024:spell_fire_bolt
    citations:
    - SRD 5.2.1 pp. 132; builtin/srd-5.2.1/spell/fire_bolt.md
  dnd2024:spells/flame_blade:
    source_sha256: 84b27bde20bded9dc8edb72b5e3e052ccc60b1e561c193a7edf4ad9cf9fa45ef
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/flame_blade
        implementation: dnd2024:spell_flame_blade
        parameters:
          creation:
            requires_free_hand: true
            form: scimitar_sized_shaped_blade
            duration: spell_duration
            release: disappear
            reevoke_cost: bonus_action
            light:
              bright_radius_feet: 10
              additional_dim_feet: 10
          attack:
            cost: magic_action
            kind: melee_spell
            hit:
              dice:
                count:
                  op: add
                  args:
                  - 3
                  - op: multiply
                    args:
                    - 1
                    - op: subtract
                      args:
                      - binding: cast_level
                      - 2
                sides: 6
                add:
                  binding: spellcasting_modifier
              type: fire
    contract:
      id: dnd2024:spell_flame_blade
      inputs:
        caster: participant
        cast_level: integer
        spellcasting_modifier: integer
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
      outputs:
      - effects
      - attacks
      - items
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - dice
      - attacks
      - inventory
    requires:
    - dnd2024:spell_flame_blade
    citations:
    - SRD 5.2.1 pp. 132; builtin/srd-5.2.1/spell/flame_blade.md
  dnd2024:spells/floating_disk:
    source_sha256: a85a3ea0ba3f2255fabb7ab75a767f00163748103988799295108c9dba80f130
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/floating_disk
        implementation: dnd2024:spell_floating_disk
        parameters:
          creation:
            shape: horizontal_circular_plane
            material: force
            diameter_feet: 3
            thickness_inches: 1
            height_above_ground_feet: 3
            space:
              unoccupied: true
              visible: true
              within_spell_range: true
            capacity_pounds: 500
            duration: spell_duration
          movement:
            immobile_if_caster_within_feet: 20
            follows_if_caster_farther_than_feet: 20
            follow_distance_maximum_feet: 20
            allowed_terrain:
            - uneven
            - stairs
            - slopes
            cannot_cross_elevation_change_feet_minimum: 10
          end_if:
            load_pounds_greater_than: 500
            caster_distance_feet_greater_than: 100
          end_cleanup:
            contents: fall_to_ground
    contract:
      id: dnd2024:spell_floating_disk
      inputs:
        caster: participant
        cast_level: integer
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
      outputs:
      - effects
      - items
      events:
      - spell.cast_completed
      - effect.ended
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - inventory
    requires:
    - dnd2024:spell_floating_disk
    citations:
    - SRD 5.2.1 pp. 133; builtin/srd-5.2.1/spell/floating_disk.md
  dnd2024:spells/forcecage:
    source_sha256: 23d5b243134b91fc6cb5dcea69d7436eada454a3e47f675e91a99f3ef815c298
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/forcecage
        implementation: dnd2024:spell_forcecage
        parameters:
          creation:
            shape: cube
            material: magical_force
            immobile: true
            invisible: true
            within_spell_range: true
            duration: spell_duration
            modes:
              cage:
                side_feet_maximum: 20
                bar_diameter_inches:
                  numerator: 1
                  denominator: 2
                bar_spacing_inches:
                  numerator: 1
                  denominator: 2
              box:
                side_feet_maximum: 10
                blocks:
                - all_matter
                - spells_cast_into_area
                - spells_cast_out_of_area
          initial_creatures:
            fully_inside: trapped
            partially_inside_or_too_large:
              push_direction: away_from_center
              until: fully_outside
          escape:
            nonmagical: prohibited
            teleport_or_interplanar:
              prior_save:
                ability: charisma
                dc:
                  binding: save_dc
                failure:
                  exit: false
                  spell_or_effect: wasted
                success:
                  exit_allowed: true
            ethereal: blocked
          dispel_magic_can_dispel: false
    contract:
      id: dnd2024:spell_forcecage
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - saving_throws
    requires:
    - dnd2024:spell_forcecage
    citations:
    - SRD 5.2.1 pp. 134; builtin/srd-5.2.1/spell/forcecage.md
  dnd2024:spells/gaseous_form:
    source_sha256: 032853848f71a822d43e18fb118315ed88a52023808bed980f04375183112034
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/gaseous_form
        implementation: dnd2024:spell_gaseous_form
        parameters:
          targets:
            kind: creature
            maximum:
              op: add
              args:
              - 1
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 3
            within_spell_range: true
            willing: true
            touched: true
          transformation:
            form: misty_cloud
            include_worn_carried: true
            duration: spell_duration
            only_movement:
              fly_speed_feet: 10
              hover: true
            may_enter_and_occupy_other_creature_space: true
            damage_resistance:
            - bludgeoning
            - piercing
            - slashing
            condition_immunity:
            - prone
            save_advantage:
            - strength
            - dexterity
            - constitution
            pass_narrow_openings: true
            liquids_treated_as: solid_surfaces
            prohibit:
            - talk
            - manipulate_objects
            - drop_carried_or_held_objects
            - use_carried_or_held_objects
            - interact_with_carried_or_held_objects
            - attack
            - cast_spells
          end_on_target:
            hp_zero: true
            voluntary_cost: magic_action
    contract:
      id: dnd2024:spell_gaseous_form
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      - movement.completed
      - save.before
      - save.after
      - hp.changed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - conditions
      - saving_throws
      - roll_modifiers
      - forms
    requires:
    - dnd2024:spell_gaseous_form
    citations:
    - SRD 5.2.1 pp. 134, 135; builtin/srd-5.2.1/spell/gaseous_form.md
  dnd2024:spells/giant_insect:
    source_sha256: 409319b5676333998f2844b930b125c2dce55b7241675fcf7249f898b1a2a8aa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/giant_insect
        implementation: dnd2024:spell_giant_insect
        parameters:
          summon:
            form_choice:
            - centipede
            - spider
            - wasp
            space:
              unoccupied: true
              visible: true
              within_spell_range: true
            ally_to:
            - caster
            - caster_allies
            disappear:
            - hp_zero
            - spell_ends
          combat:
            initiative: caster_count
            turn: immediately_after_caster
            verbal_commands_cost: none
            without_command:
              action: dodge
              movement: avoid_danger
          statistics:
            size: large
            type: beast
            alignment: unaligned
            ac:
              op: add
              args:
              - 11
              - binding: cast_level
            hp:
              op: add
              args:
              - 30
              - op: multiply
                args:
                - 10
                - op: subtract
                  args:
                  - binding: cast_level
                  - 4
            speed_feet: 40
            climb_speed_feet: 40
            fly:
              form: wasp
              speed_feet: 40
            abilities:
              strength: 17
              dexterity: 13
              constitution: 15
              intelligence: 4
              wisdom: 14
              charisma: 3
            save_modifiers:
              strength: 3
              dexterity: 1
              constitution: 2
              intelligence: -3
              wisdom: 2
              charisma: -4
            darkvision_feet: 60
            passive_perception: 12
            understood_languages: caster_known_languages
            challenge_rating: null
            xp: 0
            proficiency_bonus:
              binding: caster_proficiency_bonus
          traits:
            spider_climb:
              climb_difficult_surfaces: true
              ceilings: true
              requires_ability_check: false
          actions:
            multiattack:
              attack_count:
                op: floor
                args:
                - op: multiply
                  args:
                  - binding: cast_level
                  - numerator: 1
                    denominator: 2
              choices:
              - poison_jab
              - web_bolt_if_spider
            poison_jab:
              attack: melee
              bonus:
                binding: spell_attack_modifier
              reach_feet: 10
              damage:
              - dice:
                  count: 1
                  sides: 6
                  add:
                    op: add
                    args:
                    - 3
                    - binding: cast_level
                type: piercing
              - dice:
                  count: 1
                  sides: 4
                  add: 0
                type: poison
            web_bolt:
              form: spider
              attack: ranged
              bonus:
                binding: spell_attack_modifier
              range_feet: 60
              damage:
                dice:
                  count: 1
                  sides: 10
                  add:
                    op: add
                    args:
                    - 3
                    - binding: cast_level
                type: bludgeoning
              hit_effect:
                speed: 0
                until: insect_next_turn_start
          bonus_actions:
            venomous_spew:
              form: centipede
              target:
                kind: creature
                count: 1
                visible_to_insect: true
                range_feet: 10
              save:
                ability: constitution
                dc:
                  binding: save_dc
                failure:
                  condition: poisoned
                  until: insect_next_turn_start
                success:
                  effect: none
    contract:
      id: dnd2024:spell_giant_insect
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        caster_proficiency_bonus: integer
        save_dc: integer
        spell_attack_modifier: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      - attacks
      - participants
      events:
      - spell.cast_completed
      - effect.ended
      - movement.completed
      - turn.start
      - save.before
      - save.after
      - hp.changed
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - attacks
      - spatial_relationships
      - conditions
      - saving_throws
      - participant_lifecycle
    requires:
    - dnd2024:spell_giant_insect
    citations:
    - SRD 5.2.1 pp. 135, 136; builtin/srd-5.2.1/spell/giant_insect.md
  dnd2024:spells/goodberry:
    source_sha256: c762733cdadc1f10570da64cdf792c79fb517b77da64a803666288b108fe049b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/goodberry
        implementation: dnd2024:spell_goodberry
        parameters:
          creation:
            kind: berry
            count: 10
            location: caster_hand
            duration: spell_duration
            uneaten_disappear_on_end: true
          eat:
            cost: bonus_action
            berries_consumed: 1
            healing_hp: 1
            nourishment_creature_days: 1
    contract:
      id: dnd2024:spell_goodberry
      inputs:
        caster: participant
        cast_level: integer
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
      outputs:
      - effects
      - items
      - health
      events:
      - spell.cast_completed
      - effect.ended
      capabilities:
      - spellcasting
      - effects
      - inventory
      - health
    requires:
    - dnd2024:spell_goodberry
    citations:
    - SRD 5.2.1 pp. 137; builtin/srd-5.2.1/spell/goodberry.md
  dnd2024:spells/guardian_of_faith:
    source_sha256: 09789baf77cbecb9cd9ec4f5ee26693bced12d648bc285a31ca150af6fda6b5b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/guardian_of_faith
        implementation: dnd2024:spell_guardian_of_faith
        parameters:
          creation:
            size: large
            kind: spectral_guardian
            space:
              unoccupied: true
              visible: true
              within_spell_range: true
            hover: true
            occupies_space: true
            invulnerable: true
            duration: spell_duration
            appearance_constraint: appropriate_to_caster_deity_or_pantheon
          targets:
            relation: enemy
            distance_from_guardian_maximum_feet: 10
          triggers:
          - moves_into_distance_first_time_on_turn
          - starts_turn_in_distance
          save:
            ability: dexterity
            dc:
              binding: save_dc
            failure:
              damage:
                amount: 20
                type: radiant
            success:
              damage:
                amount: 10
                type: radiant
          vanish_when:
            total_damage_dealt_minimum: 60
    contract:
      id: dnd2024:spell_guardian_of_faith
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      - participants
      events:
      - spell.cast_completed
      - turn.start
      - effect.ended
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - spatial_relationships
      - saving_throws
      - participant_lifecycle
    requires:
    - dnd2024:spell_guardian_of_faith
    citations:
    - SRD 5.2.1 pp. 137; builtin/srd-5.2.1/spell/guardian_of_faith.md
  dnd2024:spells/gust_of_wind:
    source_sha256: a330944a713c25a0060f2b8c7b4e7b174cb218f62e515f1aefbe2b7fa30840bb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/gust_of_wind
        implementation: dnd2024:spell_gust_of_wind
        parameters:
          area:
            shape: line
            length_feet: 60
            width_feet: 10
            origin: caster
            direction_choice: caster
            duration: spell_duration
            wind_strength: strong
          save:
            ability: strength
            dc:
              binding: save_dc
            failure:
              push_feet: 15
              direction: away_from_caster_along_line
            success:
              effect: none
          save_triggers:
          - initial_creature_in_line
          - creature_turn_ends_in_line
          movement:
            moving_closer_to_caster_in_line:
              cost_feet_per_foot: 2
          environment:
            disperse:
            - gas
            - vapor
            unprotected_flames: extinguish
            protected_flames:
              flicker_wildly: true
              extinguish_chance_percent: 50
          redirect:
            cost: bonus_action
            timing: caster_later_turns
    contract:
      id: dnd2024:spell_gust_of_wind
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - movement.completed
      - turn.end
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - spatial_relationships
      - saving_throws
    requires:
    - dnd2024:spell_gust_of_wind
    citations:
    - SRD 5.2.1 pp. 138; builtin/srd-5.2.1/spell/gust_of_wind.md
  dnd2024:spells/haste:
    source_sha256: 26a4cd5bc3eca2a31be06e26c85bd668e65d3a6fbd4f528c114ee6aa21322b72
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/haste
        implementation: dnd2024:spell_haste
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            visible: true
            willing: true
          benefits:
            duration: spell_duration
            speed_multiplier: 2
            ac_bonus: 2
            saving_throw_advantage:
            - dexterity
            additional_action_each_target_turn:
              count: 1
              allowed:
              - attack
              - dash
              - disengage
              - hide
              - utilize
              attack_count_maximum: 1
          on_end:
            condition: incapacitated
            speed: 0
            until: target_next_turn_end
    contract:
      id: dnd2024:spell_haste
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - attack.roll
      - attack.hit
      - effect.ended
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - conditions
      - roll_modifiers
    requires:
    - dnd2024:spell_haste
    citations:
    - SRD 5.2.1 pp. 139; builtin/srd-5.2.1/spell/haste.md
  dnd2024:spells/hellish_rebuke:
    source_sha256: 616ee5c8292a2a8b87016551fef731a8c3e08caf6f7c5deeafff5e194c1b066e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/hellish_rebuke
        implementation: dnd2024:spell_hellish_rebuke
        parameters:
          targets:
            kind: creature
            identity: triggering_damage_source
            visible: true
            maximum_distance_feet: 60
          save:
            ability: dexterity
            dc:
              binding: save_dc
            failure:
              damage:
                dice:
                  count: &id001
                    op: add
                    args:
                    - 2
                    - op: multiply
                      args:
                      - 1
                      - op: subtract
                        args:
                        - binding: cast_level
                        - 1
                  sides: 10
                  add: 0
                type: fire
            success:
              damage:
                dice:
                  count: *id001
                  sides: 10
                  add: 0
                type: fire
              damage_fraction:
                numerator: 1
                denominator: 2
    contract:
      id: dnd2024:spell_hellish_rebuke
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - damage.applied
      - save.before
      - save.after
      - spell.cast_pending
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - saving_throws
    requires:
    - dnd2024:spell_hellish_rebuke
    citations:
    - SRD 5.2.1 pp. 140; builtin/srd-5.2.1/spell/hellish_rebuke.md
  dnd2024:spells/hideous_laughter:
    source_sha256: 56c6993ec995f47f47c7c73ef1d3b39a58f1f3f65f4b2379932a6f609d2d5192
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/hideous_laughter
        implementation: dnd2024:spell_hideous_laughter
        parameters:
          targets:
            kind: creature
            maximum:
              op: add
              args:
              - 1
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 1
            within_spell_range: true
            visible: true
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              conditions:
              - prone
              - incapacitated
              duration: spell_duration
              cannot_end_own_prone: true
              laugh_uncontrollably_if_capable: true
            success:
              effect: none
          repeat_save:
            ability: wisdom
            events:
            - target_turn_end
            - target_takes_damage
            advantage_if: triggered_by_damage
            success: end_spell
    contract:
      id: dnd2024:spell_hideous_laughter
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - turn.end
      - damage.applied
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - conditions
      - saving_throws
      - roll_modifiers
    requires:
    - dnd2024:spell_hideous_laughter
    citations:
    - SRD 5.2.1 pp. 140; builtin/srd-5.2.1/spell/hideous_laughter.md
  dnd2024:spells/hunter_s_mark:
    source_sha256: bf18880c35e033a7190063d63f5aa884b4f370288b6907a621d4a292ba29278e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/hunter_s_mark
        implementation: dnd2024:spell_hunter_s_mark
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            visible: true
          mark:
            duration: spell_duration
            extra_damage:
              trigger: caster_hits_target_with_attack_roll
              damage:
                dice:
                  count: 1
                  sides: 6
                  add: 0
                type: force
            advantage:
              checks_ability: wisdom
              skills:
              - perception
              - survival
              purpose: find_marked_target
          transfer:
            requires: marked_target_dropped_to_zero_hp_before_spell_end
            cost: bonus_action
            new_target:
              kind: creature
              maximum: 1
              within_spell_range: true
              visible: true
          duration_by_slot:
            '1_2':
              hours: 1
            '3_4':
              hours: 8
            5_plus:
              hours: 24
          concentration: true
    contract:
      id: dnd2024:spell_hunter_s_mark
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - attack.roll
      - attack.hit
      - movement.completed
      - hp.changed
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - spatial_relationships
      - roll_modifiers
    requires:
    - dnd2024:spell_hunter_s_mark
    citations:
    - SRD 5.2.1 pp. 141; builtin/srd-5.2.1/spell/hunter_s_mark.md
  dnd2024:spells/identify:
    source_sha256: e243c679cfa4c9bcb8144bd7543cdaa5a56eb25bf1a4c24e41a65956dbcf007f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/identify
        implementation: dnd2024:spell_identify
        parameters:
          targets:
            count: 1
            kind_choices:
            - object
            - creature
            touch_throughout_casting: true
          knowledge:
            magical_object:
            - properties
            - use_instructions
            - attunement_required
            - charges_if_any
            object:
            - ongoing_spells
            - creating_spell_name_if_created_by_spell
            creature:
            - ongoing_spells
    contract:
      id: dnd2024:spell_identify
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
      outputs:
      - effects
      - knowledge
      - items
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - knowledge
      - inventory
    requires:
    - dnd2024:spell_identify
    citations:
    - SRD 5.2.1 pp. 142; builtin/srd-5.2.1/spell/identify.md
  dnd2024:spells/inflict_wounds:
    source_sha256: 47d056a464236fbbc57cdd4d088530ec45123db82b9b0508f9d918ed17bc4e29
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/inflict_wounds
        implementation: dnd2024:spell_inflict_wounds
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            touched: true
          save:
            ability: constitution
            dc:
              binding: save_dc
            failure:
              damage:
                dice:
                  count:
                    op: add
                    args:
                    - 2
                    - op: multiply
                      args:
                      - 1
                      - op: subtract
                        args:
                        - binding: cast_level
                        - 1
                  sides: 10
                  add: 0
                type: necrotic
            success:
              damage:
                dice:
                  count:
                    op: add
                    args:
                    - 2
                    - op: multiply
                      args:
                      - 1
                      - op: subtract
                        args:
                        - binding: cast_level
                        - 1
                  sides: 10
                  add: 0
                type: necrotic
              damage_fraction:
                numerator: 1
                denominator: 2
    contract:
      id: dnd2024:spell_inflict_wounds
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - saving_throws
    requires:
    - dnd2024:spell_inflict_wounds
    citations:
    - SRD 5.2.1 pp. 143; builtin/srd-5.2.1/spell/inflict_wounds.md
  dnd2024:spells/irresistible_dance:
    source_sha256: ab661562fba1b97ed26ce01dc12f29fd7fd0b341cb276c78554cffbe0147bf72
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/irresistible_dance
        implementation: dnd2024:spell_irresistible_dance
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            visible: true
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              condition: charmed
              duration: spell_duration
              while_charmed:
                dance: true
                all_movement: dance_in_place
                disadvantage:
                - dexterity_saves
                - attack_rolls
                other_creatures_attack_advantage_against_target: true
              repeat_save:
                cost: action
                timing: each_target_turn
                success: end_spell_on_target
            success:
              dance: true
              all_movement: dance_in_place
              until: target_next_turn_end
    contract:
      id: dnd2024:spell_irresistible_dance
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - turn.end
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - conditions
      - saving_throws
      - roll_modifiers
    requires:
    - dnd2024:spell_irresistible_dance
    citations:
    - SRD 5.2.1 pp. 143; builtin/srd-5.2.1/spell/irresistible_dance.md
  dnd2024:spells/lesser_restoration:
    source_sha256: f5c46d89e4456e8742e3ab233251b52824d4bbaca80641f631a44e256974df0e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/lesser_restoration
        implementation: dnd2024:spell_lesser_restoration
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            touched: true
          remove_condition:
            count: 1
            choices:
            - blinded
            - deafened
            - paralyzed
            - poisoned
    contract:
      id: dnd2024:spell_lesser_restoration
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - conditions
    requires:
    - dnd2024:spell_lesser_restoration
    citations:
    - SRD 5.2.1 pp. 144; builtin/srd-5.2.1/spell/lesser_restoration.md
  dnd2024:spells/locate_animals_or_plants:
    source_sha256: 00f22b299754428f29d70e233414aacd7f495c0189ebf6dc2e1a23d840c3eb50
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/locate_animals_or_plants
        implementation: dnd2024:spell_locate_animals_or_plants
        parameters:
          query:
            selection: named_or_described_kind
            categories:
            - beast
            - plant_creature
            - nonmagical_plant
            radius_miles: 5
            match: closest
            output:
            - direction
            - distance
            requires_match_present: true
    contract:
      id: dnd2024:spell_locate_animals_or_plants
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - knowledge
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - knowledge
    requires:
    - dnd2024:spell_locate_animals_or_plants
    citations:
    - SRD 5.2.1 pp. 144; builtin/srd-5.2.1/spell/locate_animals_or_plants.md
  dnd2024:spells/mage_armor:
    source_sha256: ba5265aa82af20d5f0fb69b9abb0667ac584b41a24af2e4658758764d917b207
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mage_armor
        implementation: dnd2024:spell_mage_armor
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            touched: true
            willing: true
            wearing_armor: false
          base_ac:
            base: 13
            add: target_dexterity_modifier
            duration: spell_duration
          end_when:
            target_dons_armor: true
    contract:
      id: dnd2024:spell_mage_armor
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_mage_armor
    citations:
    - SRD 5.2.1 pp. 145; builtin/srd-5.2.1/spell/mage_armor.md
  dnd2024:spells/magic_missile:
    source_sha256: 34651f6abdcbca3c127bd4662cb8ed00c648542f7a98fa22793d4cbac510a6db
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/magic_missile
        implementation: dnd2024:spell_magic_missile
        parameters:
          darts:
            count:
              op: add
              args:
              - 3
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 1
            target_per_dart:
              kind: creature
              maximum: 1
              within_spell_range: true
              visible: true
            allow_same_target_multiple_darts: true
            automatic_strike: true
            simultaneous: true
            damage_per_dart:
              dice:
                count: 1
                sides: 4
                add: 1
              type: force
    contract:
      id: dnd2024:spell_magic_missile
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
    requires:
    - dnd2024:spell_magic_missile
    citations:
    - SRD 5.2.1 pp. 146; builtin/srd-5.2.1/spell/magic_missile.md
  dnd2024:spells/major_image:
    source_sha256: ef51aee269219b5c6deaebe6651d5119bad94d2f71448a1f4adb4cc54387b3f9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/major_image
        implementation: dnd2024:spell_major_image
        parameters:
          illusion:
            depiction_choices:
            - object
            - creature
            - visible_phenomenon
            maximum_cube_side_feet: 20
            location:
              visible: true
              within_spell_range: true
            senses:
            - visual
            - sound
            - smell
            - temperature
            sensory_qualities: appropriate_to_depiction
            can_deal_damage: false
            can_cause_conditions: false
          manipulation:
            requires_caster_within_spell_range_of_illusion: true
            cost: magic_action
            move_to: spot_within_spell_range
            alter_appearance_for_natural_movement: true
            vary_sound_over_time: true
            conversation: true
          discern:
            physical_interaction:
              automatic: true
              objects_pass_through: true
            study:
              cost: study_action
              ability: intelligence
              skill: investigation
              dc:
                binding: save_dc
            on_discern:
              see_through_image: true
              other_sensory_qualities: faint
          duration_by_slot:
            '3':
              minutes: 10
              concentration: true
            4_plus:
              until: dispelled
              concentration: false
    contract:
      id: dnd2024:spell_major_image
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - conditions
      - saving_throws
    requires:
    - dnd2024:spell_major_image
    citations:
    - SRD 5.2.1 pp. 147; builtin/srd-5.2.1/spell/major_image.md
  dnd2024:spells/mass_suggestion:
    source_sha256: 5b5fa7d36d4ba55868414c4661742648a1014af6cea184a88d95b7f45d387688
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/mass_suggestion
        implementation: dnd2024:spell_mass_suggestion
        parameters:
          targets:
            kind: creature
            maximum: 12
            within_spell_range: true
            visible: true
            can_hear_caster: true
            understands_caster: true
          suggestion:
            word_limit: 25
            must_sound_achievable: true
            must_not_obviously_damage:
            - targets
            - target_allies
            content_validation:
              chooser: gm
              input_type: choice
              constraints:
              - achievable_appearance
              - no_obvious_damage
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              condition: charmed
              duration: spell_duration
              behavior: pursue_suggestion_best_ability
              end_on_target:
              - caster_or_caster_ally_deals_damage_to_target
              - suggested_activity_complete
            success:
              effect: none
          duration_by_slot:
            '6':
              hours: 24
            '7':
              days: 10
            '8':
              days: 30
            '9':
              days: 366
    contract:
      id: dnd2024:spell_mass_suggestion
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        message:
          type: text
          required: false
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - damage.applied
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - conditions
      - saving_throws
      - behavior_constraints
    requires:
    - dnd2024:spell_mass_suggestion
    citations:
    - SRD 5.2.1 pp. 148; builtin/srd-5.2.1/spell/mass_suggestion.md
  dnd2024:spells/message:
    source_sha256: 36796dc20974441935860478dcac88df30442569ab1d6578713569ec29813697
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/message
        implementation: dnd2024:spell_message
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
          communication:
            point_toward_target: true
            caster_whispers: true
            message_heard_only_by: target
            reply_optional: true
            reply_whispered: true
            reply_heard_only_by: caster
          through_solid_objects:
            requires:
            - caster_familiar_with_target
            - caster_knows_target_beyond_barrier
            blocked_by:
            - kind: magical_silence
            - materials:
              - stone
              - metal
              - wood
              minimum_thickness_feet: 1
            - material: lead
              any_thin_sheet: true
    contract:
      id: dnd2024:spell_message
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        message:
          type: text
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - knowledge
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - knowledge
    requires:
    - dnd2024:spell_message
    citations:
    - SRD 5.2.1 pp. 148, 149; builtin/srd-5.2.1/spell/message.md
  dnd2024:spells/minor_illusion:
    source_sha256: ba92e52398d6119d21d6c3c0fb0d08b0360df1c266b0ebcb092e9b8d713223fa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/minor_illusion
        implementation: dnd2024:spell_minor_illusion
        parameters:
          illusion:
            within_spell_range: true
            duration: spell_duration
            end_on_recast: true
            choose_modes: 1
          modes:
            sound:
              volume_minimum: whisper
              volume_maximum: scream
              sound_choice: any
              timing_choices:
              - continuous
              - discrete_over_duration
            image:
              depiction: object
              maximum_cube_side_feet: 5
              prohibit_sensory_effects:
              - sound
              - light
              - smell
              - all_nonvisual
              physical_interaction:
                reveals_illusion: true
                things_pass_through: true
          discern:
            cost: study_action
            check:
              ability: intelligence
              skill: investigation
              dc:
                binding: save_dc
            success: recognize_illusion
            recognized_illusion_appearance: faint
    contract:
      id: dnd2024:spell_minor_illusion
      inputs:
        caster: participant
        cast_level: integer
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      - movement.completed
      - save.before
      - save.after
      - spell.cast_pending
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - saving_throws
    requires:
    - dnd2024:spell_minor_illusion
    citations:
    - SRD 5.2.1 pp. 149; builtin/srd-5.2.1/spell/minor_illusion.md
  dnd2024:spells/misty_step:
    source_sha256: fe57eb9912ccc0a4eb075663077de72a554e5433a6c155d41e7f4b37e4efd81f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/misty_step
        implementation: dnd2024:spell_misty_step
        parameters:
          teleport:
            subject: caster
            maximum_feet: 30
            destination_unoccupied: true
            destination_visible: true
    contract:
      id: dnd2024:spell_misty_step
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
    requires:
    - dnd2024:spell_misty_step
    citations:
    - SRD 5.2.1 pp. 150; builtin/srd-5.2.1/spell/misty_step.md
  dnd2024:spells/nondetection:
    source_sha256: 0983a5661bfe5b3c4c51e9450958135734c6891d8413f011f61699b6df936a50
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/nondetection
        implementation: dnd2024:spell_nondetection
        parameters:
          targets:
            touched: true
            choices:
            - kind: creature
              willing: true
            - kind: place
              maximum_dimension_feet: 10
            - kind: object
              maximum_dimension_feet: 10
          protection:
            duration: spell_duration
            cannot_be_targeted_by_spell_school: divination
            cannot_be_perceived_through: magical_scrying_sensors
    contract:
      id: dnd2024:spell_nondetection
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - movement.completed
      - spell.cast_pending
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
    requires:
    - dnd2024:spell_nondetection
    citations:
    - SRD 5.2.1 pp. 151; builtin/srd-5.2.1/spell/nondetection.md
  dnd2024:spells/phantasmal_killer:
    source_sha256: 2cd81d1944c7a15538eb46b7bd30cf83380a9bb68d4b9cbb52d1f510ebeaac48
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/phantasmal_killer
        implementation: dnd2024:spell_phantasmal_killer
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            visible: true
          illusion:
            depiction: target_deepest_fears
            visible_only_to: target
          initial_save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              damage:
                dice:
                  count:
                    op: add
                    args:
                    - 4
                    - op: multiply
                      args:
                      - 1
                      - op: subtract
                        args:
                        - binding: cast_level
                        - 4
                  sides: 10
                  add: 0
                type: psychic
              disadvantage:
              - ability_checks
              - attack_rolls
              duration: spell_duration
            success:
              damage:
                dice:
                  count:
                    op: add
                    args:
                    - 4
                    - op: multiply
                      args:
                      - 1
                      - op: subtract
                        args:
                        - binding: cast_level
                        - 4
                  sides: 10
                  add: 0
                type: psychic
              damage_fraction:
                numerator: 1
                denominator: 2
              end_spell: true
          repeat_save:
            event: target_turn_end
            save:
              ability: wisdom
              dc:
                binding: save_dc
              failure:
                damage:
                  dice:
                    count:
                      op: add
                      args:
                      - 4
                      - op: multiply
                        args:
                        - 1
                        - op: subtract
                          args:
                          - binding: cast_level
                          - 4
                    sides: 10
                    add: 0
                  type: psychic
              success:
                end_spell: true
    contract:
      id: dnd2024:spell_phantasmal_killer
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - turn.end
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - saving_throws
      - roll_modifiers
    requires:
    - dnd2024:spell_phantasmal_killer
    citations:
    - SRD 5.2.1 pp. 152; builtin/srd-5.2.1/spell/phantasmal_killer.md
  dnd2024:spells/plane_shift:
    source_sha256: 3a5802cd32fbd8a75cf3ccec9e5583c1267a765b1e661b96294728b4012d8459
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/plane_shift
        implementation: dnd2024:spell_plane_shift
        parameters:
          targets:
            caster: true
            additional_creatures_maximum: 8
            willing: true
            link_hands_in_circle: true
          transport:
            different_plane: true
            plane_matches_attuned_material: true
            destination_modes:
              general:
                caster_specifies: general_terms
                arrival_chooser: gm
                arrival_constraint: in_or_near_destination
              teleportation_circle:
                requires_known_sigil_sequence: true
                destination: circle_on_other_plane
                overflow: closest_unoccupied_spaces_next_to_circle
    contract:
      id: dnd2024:spell_plane_shift
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
    requires:
    - dnd2024:spell_plane_shift
    citations:
    - SRD 5.2.1 pp. 153; builtin/srd-5.2.1/spell/plane_shift.md
  dnd2024:spells/power_word_heal:
    source_sha256: 10ccd108b55ea895c923d7dd7d0d85cdb294469a3b71a6fe9c3b22015985d3b3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/power_word_heal
        implementation: dnd2024:spell_power_word_heal
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            visible: true
          healing:
            restore: all_hit_points
          end_conditions:
          - charmed
          - frightened
          - paralyzed
          - poisoned
          - stunned
          prone:
            stand_optional: true
            cost: target_reaction
    contract:
      id: dnd2024:spell_power_word_heal
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - health
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - conditions
      - health
    requires:
    - dnd2024:spell_power_word_heal
    citations:
    - SRD 5.2.1 pp. 154; builtin/srd-5.2.1/spell/power_word_heal.md
  dnd2024:spells/prestidigitation:
    source_sha256: 4d450e76e7714a965c1310eb2b1ba3ed6b18a7d6fdcb4cb930d450c34816f413
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/prestidigitation
        implementation: dnd2024:spell_prestidigitation
        parameters:
          selection:
            within_spell_range: true
            effect_count: 1
            noninstantaneous_active_maximum_across_castings: 3
          modes:
            sensory_effect:
              duration: instantaneous
              harmless: true
              sensory_effect_choice: true
            fire_play:
              duration: instantaneous
              choices:
              - light
              - snuff
              target_kinds:
              - candle
              - torch
              - small_campfire
            clean_or_soil:
              duration: instantaneous
              choices:
              - clean
              - soil
              object_maximum_cubic_feet: 1
            minor_sensation:
              choices:
              - chill
              - warm
              - flavor
              nonliving_material_maximum_cubic_feet: 1
              duration_hours: 1
            magic_mark:
              choices:
              - color
              - small_mark
              - symbol
              target_kinds:
              - object
              - surface
              duration_hours: 1
            minor_creation:
              choices:
              - nonmagical_trinket
              - illusory_image
              must_fit_in_caster_hand: true
              until: caster_next_turn_end
              trinket:
                damage_possible: false
                monetary_worth: 0
    contract:
      id: dnd2024:spell_prestidigitation
      inputs:
        caster: participant
        cast_level: integer
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
      outputs:
      - effects
      - items
      events:
      - spell.cast_completed
      - movement.completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - inventory
    requires:
    - dnd2024:spell_prestidigitation
    citations:
    - SRD 5.2.1 pp. 154; builtin/srd-5.2.1/spell/prestidigitation.md
  dnd2024:spells/produce_flame:
    source_sha256: 024e00953378ef1f813bba102f80c63803b5f58ad6ceceab1aeb0a99f1a1d816
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/produce_flame
        implementation: dnd2024:spell_produce_flame
        parameters:
          flame:
            location: caster_hand
            duration: spell_duration
            emits_heat: false
            ignites_objects: false
            bright_radius_feet: 20
            additional_dim_feet: 20
            end_on_recast: true
          hurl:
            cost: magic_action
            available_until_spell_ends: true
            target:
              kind: creature_or_object
              maximum_distance_feet: 60
            attack: ranged_spell
            hit:
              dice:
                count:
                  by_caster_level:
                  - minimum: 1
                    count: 1
                  - minimum: 5
                    count: 2
                  - minimum: 11
                    count: 3
                  - minimum: 17
                    count: 4
                sides: 8
                add: 0
              type: fire
            ends_spell: false
    contract:
      id: dnd2024:spell_produce_flame
      inputs:
        caster: participant
        cast_level: integer
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        caster_level: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - attacks
      events:
      - spell.cast_completed
      - effect.ended
      - movement.completed
      - spell.cast_pending
      capabilities:
      - spellcasting
      - effects
      - dice
      - attacks
      - spatial_relationships
    requires:
    - dnd2024:spell_produce_flame
    citations:
    - SRD 5.2.1 pp. 156; builtin/srd-5.2.1/spell/produce_flame.md
  dnd2024:spells/protection_from_evil_and_good:
    source_sha256: f335abadcb3f4219d2456747cf35f709a191093474f125d2c76761a03d161376
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/protection_from_evil_and_good
        implementation: dnd2024:spell_protection_from_evil_and_good
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            willing: true
            touched: true
          protection:
            duration: spell_duration
            against_types:
            - aberration
            - celestial
            - elemental
            - fey
            - fiend
            - undead
            attacks_against_target: disadvantage
            prevent:
            - possession
            - charmed_condition
            - frightened_condition
            already_affected:
              advantage_on_new_saves_against:
              - possession
              - charmed
              - frightened
              ends_existing_effect: false
    contract:
      id: dnd2024:spell_protection_from_evil_and_good
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - attack.roll
      - attack.hit
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - conditions
      - saving_throws
      - roll_modifiers
    requires:
    - dnd2024:spell_protection_from_evil_and_good
    citations:
    - SRD 5.2.1 pp. 157; builtin/srd-5.2.1/spell/protection_from_evil_and_good.md
  dnd2024:spells/ray_of_enfeeblement:
    source_sha256: 69a238fecc71a6230a62d4efab60cc4573a1bf321aa4982c82c7ec33e5200b7d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/ray_of_enfeeblement
        implementation: dnd2024:spell_ray_of_enfeeblement
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
          save:
            ability: constitution
            dc:
              binding: save_dc
            failure:
              duration: spell_duration
              disadvantage:
                rolls: d20_tests
                ability: strength
              damage_roll_modifier:
                all_damage_rolls: true
                operation: subtract
                amount:
                  dice:
                    count: 1
                    sides: 8
                    add: 0
              repeat_save:
                event: target_turn_end
                success: end_spell
            success:
              disadvantage_next_attack: true
              until: caster_next_turn_start
    contract:
      id: dnd2024:spell_ray_of_enfeeblement
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - turn.end
      - attack.roll
      - attack.hit
      - turn.start
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - saving_throws
      - roll_modifiers
    requires:
    - dnd2024:spell_ray_of_enfeeblement
    citations:
    - SRD 5.2.1 pp. 157; builtin/srd-5.2.1/spell/ray_of_enfeeblement.md
  dnd2024:spells/reincarnate:
    source_sha256: 6955ced29de402475862e98c07b1f771d9b67731ac8435e6fe2b37784631e354
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/reincarnate
        implementation: dnd2024:spell_reincarnate
        parameters:
          targets:
            kind: dead_humanoid_or_piece
            touched: true
            dead_days_maximum: 10
          revive:
            create_new_body: true
            call_soul_into_body: true
            species_selection:
              methods:
              - roll
              - gm_choose_another_playable_species
              roll:
                dice:
                  count: 1
                  sides: 10
                  add: 0
              faces:
                '1': reroll
                '2': dragonborn
                '3': dwarf
                '4': elf
                '5': gnome
                '6': goliath
                '7': halfling
                '8': human
                '9': orc
                '10': tiefling
            species_options_chooser: reincarnated_creature
            retains:
            - former_life_memories
            - original_capabilities_except_species_traits
            remove: previous_species_traits
            gain: new_species_traits
    contract:
      id: dnd2024:spell_reincarnate
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - health
      events:
      - spell.cast_completed
      - time.elapsed
      capabilities:
      - spellcasting
      - effects
      - dice
      - health
      - forms
    requires:
    - dnd2024:spell_reincarnate
    citations:
    - SRD 5.2.1 pp. 158; builtin/srd-5.2.1/spell/reincarnate.md
  dnd2024:spells/resurrection:
    source_sha256: 612ff7fe0c1466230e1353f6d1da30bc5a8593479cc93957285dee9402bf1aac
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/resurrection
        implementation: dnd2024:spell_resurrection
        parameters:
          targets:
            kind: dead_creature
            touched: true
            dead_years_maximum: 100
            death_from_old_age: false
            undead_when_died: false
          revive:
            hp: all
            neutralize: poisons_at_time_of_death
            close: all_mortal_wounds
            restore: missing_body_parts
          target_penalty:
            rolls: d20_tests
            initial: -4
            recovery:
              event: target_long_rest_completed
              increase: 1
              maximum: 0
          caster_costly_revival:
            requires_dead_days_minimum: 365
            prohibit: cast_spells
            disadvantage: d20_tests
            until: caster_long_rest_completed
    contract:
      id: dnd2024:spell_resurrection
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - health
      events:
      - spell.cast_completed
      - rest.completed
      - time.elapsed
      capabilities:
      - spellcasting
      - effects
      - roll_modifiers
      - health
    requires:
    - dnd2024:spell_resurrection
    citations:
    - SRD 5.2.1 pp. 158, 159; builtin/srd-5.2.1/spell/resurrection.md
  dnd2024:spells/sacred_flame:
    source_sha256: 4148c351b0778052b9d3c6d79b357f8e71d07f2eef486c3974aadc2c4aca2332
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/sacred_flame
        implementation: dnd2024:spell_sacred_flame
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            visible: true
          save:
            ability: dexterity
            dc:
              binding: save_dc
            failure:
              damage:
                dice:
                  count:
                    by_caster_level:
                    - minimum: 1
                      count: 1
                    - minimum: 5
                      count: 2
                    - minimum: 11
                      count: 3
                    - minimum: 17
                      count: 4
                  sides: 8
                  add: 0
                type: radiant
            success:
              effect: none
          ignore_cover_for_save:
          - half
          - three_quarters
    contract:
      id: dnd2024:spell_sacred_flame
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        caster_level: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - saving_throws
    requires:
    - dnd2024:spell_sacred_flame
    citations:
    - SRD 5.2.1 pp. 159; builtin/srd-5.2.1/spell/sacred_flame.md
  dnd2024:spells/searing_smite:
    source_sha256: 36889d4635f82b0d87b070634516f9a0f85f2ac783e3981c87d97c2e343b9703
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/searing_smite
        implementation: dnd2024:spell_searing_smite
        parameters:
          targets:
            identity: target_hit_by_triggering_attack
          extra_attack_damage:
            dice:
              count:
                op: add
                args:
                - 1
                - op: multiply
                  args:
                  - 1
                  - op: subtract
                    args:
                    - binding: cast_level
                    - 1
              sides: 6
              add: 0
            type: fire
          ongoing:
            duration: spell_duration
            event: target_turn_start
            steps:
            - damage:
                dice:
                  count:
                    op: add
                    args:
                    - 1
                    - op: multiply
                      args:
                      - 1
                      - op: subtract
                        args:
                        - binding: cast_level
                        - 1
                  sides: 6
                  add: 0
                type: fire
            - save:
                ability: constitution
                dc:
                  binding: save_dc
                failure:
                  continue_spell: true
                success:
                  end_spell: true
    contract:
      id: dnd2024:spell_searing_smite
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - turn.start
      - attack.roll
      - attack.hit
      - save.before
      - save.after
      - spell.cast_pending
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - saving_throws
    requires:
    - dnd2024:spell_searing_smite
    citations:
    - SRD 5.2.1 pp. 160; builtin/srd-5.2.1/spell/searing_smite.md
  dnd2024:spells/sending:
    source_sha256: b0b6271a8ef2b0a02ca9a9313443a879b3190be92d69e2c066ffa31200fd07cf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/sending
        implementation: dnd2024:spell_sending
        parameters:
          targets:
            kind: creature
            eligibility_any:
            - caster_has_met
            - described_by_someone_who_has_met
            distance: unlimited
            same_plane_required: false
          message:
            word_limit: 25
            channel: mental
            recognize_sender_if_target_knows_caster: true
            understands_meaning: true
            reply:
              optional: true
              timing: immediate
              same_manner: true
              word_limit: 25
          cross_plane:
            delivery_failure_chance_percent: 5
            caster_knows_failure: true
          block:
            choice: recipient_after_receiving_message
            duration_hours: 8
            scope: same_caster_sending
            attempt_while_blocked:
              spell_fails: true
              caster_learns_blocked: true
    contract:
      id: dnd2024:spell_sending
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        campaign:
          type: campaign
          required: false
        message:
          type: text
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - knowledge
      events:
      - spell.cast_completed
      - time.elapsed
      capabilities:
      - spellcasting
      - effects
      - knowledge
    requires:
    - dnd2024:spell_sending
    citations:
    - SRD 5.2.1 pp. 161; builtin/srd-5.2.1/spell/sending.md
  dnd2024:spells/shield:
    source_sha256: e8e680f8d1e68e6c9a34c1dac97f683e6f1b2bd438609f3b77bc2656790ff7b8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/shield
        implementation: dnd2024:spell_shield
        parameters:
          targets:
            kind: caster
          protection:
            until: caster_next_turn_start
            ac_bonus: 5
            includes_triggering_attack: true
            no_damage_from_spell:
              key: dnd2024:spells/magic_missile
    contract:
      id: dnd2024:spell_shield
      inputs:
        caster: participant
        cast_level: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - attack.roll
      - attack.hit
      - turn.start
      - spell.cast_pending
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_shield
    citations:
    - SRD 5.2.1 pp. 161, 162; builtin/srd-5.2.1/spell/shield.md
  dnd2024:spells/shocking_grasp:
    source_sha256: 67a5a31de4e82621d3976ec1e14d99fd7c490bf85b2e86fcd1e7e2e9ea3261d3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/shocking_grasp
        implementation: dnd2024:spell_shocking_grasp
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
          attack:
            kind: melee_spell
            hit:
              damage:
                dice:
                  count:
                    by_caster_level:
                    - minimum: 1
                      count: 1
                    - minimum: 5
                      count: 2
                    - minimum: 11
                      count: 3
                    - minimum: 17
                      count: 4
                  sides: 8
                  add: 0
                type: lightning
              prohibit: opportunity_attacks
              until: target_next_turn_start
    contract:
      id: dnd2024:spell_shocking_grasp
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        caster_level: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      - damage
      - attacks
      events:
      - spell.cast_completed
      - attack.roll
      - attack.hit
      - turn.start
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - attacks
    requires:
    - dnd2024:spell_shocking_grasp
    citations:
    - SRD 5.2.1 pp. 162; builtin/srd-5.2.1/spell/shocking_grasp.md
  dnd2024:spells/sleep:
    source_sha256: 8b88394a57cae1059e9744e67c7f2d8daeafc54a783e449476d26c745d5fe009
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/sleep
        implementation: dnd2024:spell_sleep
        parameters:
          area:
            shape: sphere
            radius_feet: 5
            origin: chosen_point_within_range
          targets:
            kind: creature
            in_area: true
            chooser: caster
          automatic_save_success_if_any:
          - does_not_sleep
          - immune_to_exhaustion
          initial_save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              condition: incapacitated
              until: target_next_turn_end
              then_repeat_save:
                ability: wisdom
                dc:
                  binding: save_dc
                failure:
                  condition: unconscious
                  duration: spell_duration
                success:
                  effect: none
            success:
              effect: none
          end_on_target:
          - takes_damage
          - another_creature:
              within_feet: 5
              cost: action
              activity: shake_awake
    contract:
      id: dnd2024:spell_sleep
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - turn.end
      - damage.applied
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - spatial_relationships
      - conditions
      - saving_throws
    requires:
    - dnd2024:spell_sleep
    citations:
    - SRD 5.2.1 pp. 163; builtin/srd-5.2.1/spell/sleep.md
  dnd2024:spells/spare_the_dying:
    source_sha256: f3f7ee4fd00b1019f9d81a24178b3ef74bf92bd1a29de13cc9010c6788619c43
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/spare_the_dying
        implementation: dnd2024:spell_spare_the_dying
        parameters:
          targets:
            kind: creature
            count: 1
            hp: 0
            dead: false
            maximum_range_feet:
              by_caster_level:
              - minimum: 1
                value: 15
              - minimum: 5
                value: 30
              - minimum: 11
                value: 60
              - minimum: 17
                value: 120
          health_transition: stabilize
    contract:
      id: dnd2024:spell_spare_the_dying
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        caster_level: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - health
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - health
    requires:
    - dnd2024:spell_spare_the_dying
    citations:
    - SRD 5.2.1 pp. 163; builtin/srd-5.2.1/spell/spare_the_dying.md
  dnd2024:spells/spider_climb:
    source_sha256: a8280dc1bdc6d65722f79227e25da59ac64f89fd6708b9852c5abd82a3095573
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/spider_climb
        implementation: dnd2024:spell_spider_climb
        parameters:
          targets:
            kind: creature
            maximum:
              op: add
              args:
              - 1
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 2
            within_spell_range: true
            willing: true
            touched: true
          movement:
            duration: spell_duration
            surfaces:
            - vertical_up
            - vertical_down
            - vertical_across
            - ceilings
            hands_remain_free: true
            climb_speed: target_speed
    contract:
      id: dnd2024:spell_spider_climb
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
    requires:
    - dnd2024:spell_spider_climb
    citations:
    - SRD 5.2.1 pp. 164; builtin/srd-5.2.1/spell/spider_climb.md
  dnd2024:spells/starry_wisp:
    source_sha256: 3d0b5515268795855bb47c90a6a1caa7a0f99cba075472291c0462fdb4693bd6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/starry_wisp
        implementation: dnd2024:spell_starry_wisp
        parameters:
          targets:
            kind: creature_or_object
            count: 1
            within_spell_range: true
          attack:
            kind: ranged_spell
            hit:
              damage:
                dice:
                  count:
                    by_caster_level:
                    - minimum: 1
                      count: 1
                    - minimum: 5
                      count: 2
                    - minimum: 11
                      count: 3
                    - minimum: 17
                      count: 4
                  sides: 8
                  add: 0
                type: radiant
              light:
                level: dim
                radius_feet: 10
              cannot_benefit_from_condition: invisible
              until: caster_next_turn_end
    contract:
      id: dnd2024:spell_starry_wisp
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        caster_level: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      - damage
      - attacks
      events:
      - spell.cast_completed
      - attack.roll
      - attack.hit
      - movement.completed
      - turn.end
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - attacks
      - spatial_relationships
      - conditions
    requires:
    - dnd2024:spell_starry_wisp
    citations:
    - SRD 5.2.1 pp. 165; builtin/srd-5.2.1/spell/starry_wisp.md
  dnd2024:spells/storm_of_vengeance:
    source_sha256: 5a89df7a22acddcbc2cac39e398037b9c1e5840640246c3ddad28a12a810caa4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/storm_of_vengeance
        implementation: dnd2024:spell_storm_of_vengeance
        parameters:
          cloud:
            radius_feet: 300
            center: chosen_point_within_range
            duration: spell_duration
          initial:
            targets: creatures_under_cloud_at_appearance
            save:
              ability: constitution
              dc:
                binding: save_dc
              failure:
                damage:
                  dice:
                    count: 2
                    sides: 6
                    add: 0
                  type: thunder
                condition: deafened
                duration: spell_duration
              success:
                effect: none
          later_effects:
            event: caster_turn_start
            turns:
            - number: 2
              targets: each_creature_and_object_under_cloud
              damage:
                dice:
                  count: 4
                  sides: 6
                  add: 0
                type: acid
            - number: 3
              targets:
                count: 6
                different: true
                kinds:
                - creature
                - object
                under_cloud: true
                chooser: caster
              save:
                ability: dexterity
                dc:
                  binding: save_dc
                failure:
                  damage:
                    dice:
                      count: 10
                      sides: 6
                      add: 0
                    type: lightning
                success:
                  damage:
                    dice:
                      count: 10
                      sides: 6
                      add: 0
                    type: lightning
                  damage_fraction:
                    numerator: 1
                    denominator: 2
            - number: 4
              targets: each_creature_under_cloud
              damage:
                dice:
                  count: 2
                  sides: 6
                  add: 0
                type: bludgeoning
            - numbers:
              - 5
              - 6
              - 7
              - 8
              - 9
              - 10
              targets: each_creature_under_cloud
              damage:
                dice:
                  count: 1
                  sides: 6
                  add: 0
                type: cold
              area_until_spell_ends:
                terrain: difficult
                obscurement: heavy
                ranged_weapon_attacks: impossible
                wind: strong
    contract:
      id: dnd2024:spell_storm_of_vengeance
      inputs:
        caster: participant
        cast_level: integer
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - turn.start
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - spatial_relationships
      - conditions
      - saving_throws
    requires:
    - dnd2024:spell_storm_of_vengeance
    citations:
    - SRD 5.2.1 pp. 165, 166; builtin/srd-5.2.1/spell/storm_of_vengeance.md
  dnd2024:spells/sunburst:
    source_sha256: a912e905a3ec7ba385c14dfd916364968fb85b7e7bb688be7b2cc069a57e7c6e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/sunburst
        implementation: dnd2024:spell_sunburst
        parameters:
          area:
            shape: sphere
            radius_feet: 60
            origin: chosen_point_within_range
            illumination: sunlight
          targets:
            kind: creature
            each_in_area: true
          save:
            ability: constitution
            dc:
              binding: save_dc
            failure:
              damage:
                dice:
                  count: 12
                  sides: 6
                  add: 0
                type: radiant
              condition: blinded
              duration_minutes: 1
            success:
              damage:
                dice:
                  count: 12
                  sides: 6
                  add: 0
                type: radiant
              damage_fraction:
                numerator: 1
                denominator: 2
          repeat_save:
            requires: blinded_by_spell
            event: target_turn_end
            ability: constitution
            success: end_effect_on_target
          dispel:
            in_area: true
            effect: darkness
            created_by: any_spell
    contract:
      id: dnd2024:spell_sunburst
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - turn.end
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - spatial_relationships
      - conditions
      - saving_throws
    requires:
    - dnd2024:spell_sunburst
    citations:
    - SRD 5.2.1 pp. 167; builtin/srd-5.2.1/spell/sunburst.md
  dnd2024:spells/teleport:
    source_sha256: d7a941d039a3a91f2d055079d5fe8bdda9cdbe1282e737b852eb1ad87e73056c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/teleport
        implementation: dnd2024:spell_teleport
        parameters:
          targets:
            modes:
              creatures:
                caster: true
                additional_maximum: 8
                willing: true
                visible: true
                within_spell_range: true
              object:
                count: 1
                visible: true
                within_spell_range: true
                maximum_size: large
                held_or_carried_by_unwilling_creature: false
                caster_transported: false
          destination:
            known_to_caster: true
            same_plane_as_caster: true
          familiarity:
            permanent_circle:
              known_sigil_sequence: true
            linked_object:
              caster_possesses_object_from_destination: true
              taken_within_months: 6
            very_familiar:
              any:
              - visited_often
              - carefully_studied
              - visible_at_cast
            seen_casually:
              seen_more_than_once: true
              very_familiar: false
            viewed_once_or_described:
              any:
              - seen_once
              - seen_once_magically
              - known_from_description
              - known_from_map
            false_destination:
              exists: false
          outcome_roll:
            roller: gm
            dice:
              dice:
                count: 1
                sides: 100
                add: 0
            tables:
              permanent_circle:
                on_target:
                - 1
                - 100
              linked_object:
                on_target:
                - 1
                - 100
              very_familiar:
                mishap:
                - 1
                - 5
                similar_area:
                - 6
                - 13
                off_target:
                - 14
                - 24
                on_target:
                - 25
                - 100
              seen_casually:
                mishap:
                - 1
                - 33
                similar_area:
                - 34
                - 43
                off_target:
                - 44
                - 53
                on_target:
                - 54
                - 100
              viewed_once_or_described:
                mishap:
                - 1
                - 43
                similar_area:
                - 44
                - 53
                off_target:
                - 54
                - 73
                on_target:
                - 74
                - 100
              false_destination:
                mishap:
                - 1
                - 50
                similar_area:
                - 51
                - 100
          outcomes:
            mishap:
              damage_to_each_teleporting_target:
                dice:
                  count: 3
                  sides: 10
                  add: 0
                type: force
              reroll_same_table: true
              repeated_mishaps_each_deal_damage: true
            similar_area:
              arrival: closest_visually_or_thematically_similar_area
              chooser: gm
              constraints:
              - closest
              - visually_or_thematically_similar
            off_target:
              distance_miles:
                dice:
                  count: 2
                  sides: 12
                  add: 0
              direction:
                dice:
                  dice:
                    count: 1
                    sides: 8
                    add: 0
                faces:
                  '1': east
                  '2': southeast
                  '3': south
                  '4': southwest
                  '5': west
                  '6': northwest
                  '7': north
                  '8': northeast
            on_target:
              arrival: intended_destination
    contract:
      id: dnd2024:spell_teleport
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - spatial_relationships
    requires:
    - dnd2024:spell_teleport
    citations:
    - SRD 5.2.1 pp. 168; builtin/srd-5.2.1/spell/teleport.md
  dnd2024:spells/time_stop:
    source_sha256: f6164760aa3cade63b2ad2de5bc8416d1790b8da367165c466ff4badd3db6bb1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/time_stop
        implementation: dnd2024:spell_time_stop
        parameters:
          time:
            other_creatures_elapsed_time: 0
            caster_consecutive_turns:
              dice:
                count: 1
                sides: 4
                add: 1
            caster_actions_and_movement: normal
          end_when:
            caster_action_or_created_effect_affects_any:
            - other_creature
            - object_worn_or_carried_by_other_creature
            caster_distance_from_cast_location_feet_greater_than: 1000
    contract:
      id: dnd2024:spell_time_stop
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - dice
      - spatial_relationships
      - turn_scheduling
    requires:
    - dnd2024:spell_time_stop
    citations:
    - SRD 5.2.1 pp. 169; builtin/srd-5.2.1/spell/time_stop.md
  dnd2024:spells/tree_stride:
    source_sha256: f58003c2d0ea8b9ab772e9a4106159b6c2def1f1825e101f10ec312a93d7a69e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/tree_stride
        implementation: dnd2024:spell_tree_stride
        parameters:
          transport:
            duration: spell_duration
            source_and_destination_trees:
              living: true
              minimum_size: caster_size
              same_kind: true
            maximum_tree_distance_feet: 500
            entry_movement_cost_feet: 5
            on_entry_knowledge:
              locations: all_same_kind_trees
              radius_feet: 500
            as_part_of_entry_choices:
            - pass_to_eligible_tree
            - step_out_of_current_tree
            exit:
              destination_space_chooser: caster
              within_destination_tree_feet: 5
              movement_cost_feet: 5
            no_movement_remaining:
              appear_within_entered_tree_feet: 5
            uses_per_caster_turn: 1
            must_end_turn_outside_tree: true
    contract:
      id: dnd2024:spell_tree_stride
      inputs:
        caster: participant
        cast_level: integer
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - knowledge
      events:
      - spell.cast_completed
      - movement.completed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - knowledge
    requires:
    - dnd2024:spell_tree_stride
    citations:
    - SRD 5.2.1 pp. 170; builtin/srd-5.2.1/spell/tree_stride.md
  dnd2024:spells/true_strike:
    source_sha256: ec4bd7e2ec18ebf1a31600d0cfe3304b637d793e4ef2aef2a71c405591afe634
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/true_strike
        implementation: dnd2024:spell_true_strike
        parameters:
          attack:
            count: 1
            weapon: casting_material_weapon
            attack_ability: spellcasting
            damage_ability: spellcasting
            replace_abilities:
            - strength
            - dexterity
            damage_type_choice:
            - radiant
            - weapon_normal
            extra_damage:
              type: radiant
              dice:
                count:
                  by_caster_level:
                  - minimum: 1
                    count: 0
                  - minimum: 5
                    count: 1
                  - minimum: 11
                    count: 2
                  - minimum: 17
                    count: 3
                sides: 6
                add: 0
              applies_regardless_of_chosen_damage_type: true
    contract:
      id: dnd2024:spell_true_strike
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        objects:
          type: object_list
          required: false
        caster_level: integer
        choices:
          type: choice
          required: false
        items:
          type: item_list
          required: false
      outputs:
      - effects
      - attacks
      - items
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
      - dice
      - attacks
      - inventory
    requires:
    - dnd2024:spell_true_strike
    citations:
    - SRD 5.2.1 pp. 171; builtin/srd-5.2.1/spell/true_strike.md
  dnd2024:spells/vicious_mockery:
    source_sha256: 93f0866ea19605320c5821c9f95df9ec2d1cf55a0c62d76f9dd021a2cbc25325
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/vicious_mockery
        implementation: dnd2024:spell_vicious_mockery
        parameters:
          targets:
            kind: creature
            maximum: 1
            within_spell_range: true
            perception_any:
            - caster_sees_target
            - caster_hears_target
          save:
            ability: wisdom
            dc:
              binding: save_dc
            failure:
              damage:
                dice:
                  count:
                    by_caster_level:
                    - minimum: 1
                      count: 1
                    - minimum: 5
                      count: 2
                    - minimum: 11
                      count: 3
                    - minimum: 17
                      count: 4
                  sides: 6
                  add: 0
                type: psychic
              disadvantage_next_attack: true
              until: target_next_turn_end
            success:
              effect: none
    contract:
      id: dnd2024:spell_vicious_mockery
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        save_dc: integer
        caster_level: integer
        choices:
          type: choice
          required: false
        trigger:
          type: event
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - attack.roll
      - attack.hit
      - turn.end
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - saving_throws
      - roll_modifiers
    requires:
    - dnd2024:spell_vicious_mockery
    citations:
    - SRD 5.2.1 pp. 171, 172; builtin/srd-5.2.1/spell/vicious_mockery.md
  dnd2024:spells/wall_of_ice:
    source_sha256: 065b35a375e6d9a58442ca765086fd94c9c2e3509974b07aa00d7307ecdaa949
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/wall_of_ice
        implementation: dnd2024:spell_wall_of_ice
        parameters:
          creation:
            kind: object
            surface: solid
            within_spell_range: true
            thickness_feet: 1
            duration: spell_duration
            shape_choice:
              hemispherical_dome:
                radius_feet_maximum: 10
              globe:
                radius_feet_maximum: 10
              flat_surface:
                panels: 10
                panel_side_feet: 10
                each_panel_contiguous_with_another: true
            ac: 12
            hp_per_10_foot_section: 30
            damage_immunities:
            - cold
            - poison
            - psychic
            damage_vulnerabilities:
            - fire
          initial_intersection:
            target: creature_whose_space_wall_cuts
            push_to_side:
              chooser: caster
            save:
              ability: dexterity
              dc:
                binding: save_dc
              failure:
                damage:
                  dice:
                    count: &id002
                      op: add
                      args:
                      - 10
                      - op: multiply
                        args:
                        - 2
                        - op: subtract
                          args:
                          - binding: cast_level
                          - 6
                    sides: 6
                    add: 0
                  type: cold
              success:
                damage:
                  dice:
                    count: *id002
                    sides: 6
                    add: 0
                  type: cold
                damage_fraction:
                  numerator: 1
                  denominator: 2
          section_destroyed:
            hp: 0
            replace_with: sheet_of_frigid_air
            location: former_section_space
          frigid_air:
            trigger: creature_moves_through_first_time_per_turn
            save:
              ability: constitution
              dc:
                binding: save_dc
              failure:
                damage:
                  dice:
                    count:
                      op: add
                      args:
                      - 5
                      - op: multiply
                        args:
                        - 1
                        - op: subtract
                          args:
                          - binding: cast_level
                          - 6
                    sides: 6
                    add: 0
                  type: cold
              success:
                damage:
                  dice:
                    count:
                      op: add
                      args:
                      - 5
                      - op: multiply
                        args:
                        - 1
                        - op: subtract
                          args:
                          - binding: cast_level
                          - 6
                    sides: 6
                    add: 0
                  type: cold
                damage_fraction:
                  numerator: 1
                  denominator: 2
    contract:
      id: dnd2024:spell_wall_of_ice
      inputs:
        caster: participant
        cast_level: integer
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - damage
      events:
      - spell.cast_completed
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - damage
      - spatial_relationships
      - saving_throws
    requires:
    - dnd2024:spell_wall_of_ice
    citations:
    - SRD 5.2.1 pp. 172, 173; builtin/srd-5.2.1/spell/wall_of_ice.md
  dnd2024:spells/water_breathing:
    source_sha256: 2153690ac42da5f5098eb9b60df374839ae12275642abcf7dd84fdf327a7e122
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/water_breathing
        implementation: dnd2024:spell_water_breathing
        parameters:
          targets:
            kind: creature
            maximum: 10
            within_spell_range: true
            willing: true
          respiration:
            underwater: true
            retain_normal_respiration: true
            duration: spell_duration
    contract:
      id: dnd2024:spell_water_breathing
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      capabilities:
      - spellcasting
      - effects
    requires:
    - dnd2024:spell_water_breathing
    citations:
    - SRD 5.2.1 pp. 173; builtin/srd-5.2.1/spell/water_breathing.md
  dnd2024:spells/wind_walk:
    source_sha256: 0fef3e0ec745d3b4da506bad3689c022f842b97ec15580d85e0b15dfb74878ae
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/wind_walk
        implementation: dnd2024:spell_wind_walk
        parameters:
          targets:
            kind: creature
            maximum: 10
            within_spell_range: true
            willing: true
            additional_target: caster
          cloud_form:
            duration: spell_duration
            appearance: cloud_wisp
            fly_speed_feet: 300
            hover: true
            condition_immunity:
            - prone
            damage_resistance:
            - bludgeoning
            - piercing
            - slashing
            only_actions:
            - dash
            - magic_action_begin_revert
          transformations:
            to_normal:
              cost: magic_action
              duration_minutes: 1
              during_condition: stunned
            to_cloud:
              requires: spell_not_ended
              cost: magic_action
              duration_minutes: 1
          end_while_flying_in_cloud:
            descent_feet_per_round: 60
            maximum_minutes: 1
            landing: safe
            if_not_landed_after_minutes:
              minutes: 1
              effect: fall_remaining_distance
    contract:
      id: dnd2024:spell_wind_walk
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_completed
      - effect.ended
      - movement.completed
      - time.elapsed
      capabilities:
      - spellcasting
      - effects
      - spatial_relationships
      - conditions
      - forms
    requires:
    - dnd2024:spell_wind_walk
    citations:
    - SRD 5.2.1 pp. 174; builtin/srd-5.2.1/spell/wind_walk.md
  dnd2024:spells/zone_of_truth:
    source_sha256: c3e70246b557d2fb01caf1e7089f8b013c38d8ee32602a688fa2e3bfe843f26c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:spells/zone_of_truth
        implementation: dnd2024:spell_zone_of_truth
        parameters:
          area:
            shape: sphere
            radius_feet: 15
            origin: chosen_point_within_range
            duration: spell_duration
          save_triggers:
          - creature_enters_area_first_time_on_turn
          - creature_starts_turn_in_area
          save:
            ability: charisma
            dc:
              binding: save_dc
            failure:
              cannot_speak_deliberate_lie: true
              restriction_only_in_radius: true
              aware_of_spell: true
              may_refuse_answer: true
              may_evade_truthfully: true
            success:
              effect: none
          caster_knows_each_save_outcome: true
    contract:
      id: dnd2024:spell_zone_of_truth
      inputs:
        caster: participant
        cast_level: integer
        targets: participant_list
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
        save_dc: integer
        choices:
          type: choice
          required: false
      outputs:
      - effects
      - knowledge
      events:
      - spell.cast_completed
      - turn.start
      - movement.completed
      - save.before
      - save.after
      capabilities:
      - spellcasting
      - effects
      - dice
      - spatial_relationships
      - saving_throws
      - knowledge
    requires:
    - dnd2024:spell_zone_of_truth
    citations:
    - SRD 5.2.1 pp. 175; builtin/srd-5.2.1/spell/zone_of_truth.md
---

# SRD spell contracts, assignment 3

All 85 assigned spell definitions are specified from the bundled SRD 5.2.1 sources. Each definition has its own source hash and page/file citation above. The existing extracted spell header remains authoritative for casting time, range, components, material cost and consumption, base duration, concentration, classes, school, and level. The parameters specify every mechanical body clause, including higher slot and cantrip scaling. No engine is implemented.

The parameter maps are source-specific typed contracts. Snake-case strings denote closed semantic choices or named operations, not prose to parse. `binding` and `op` use the mechanics-schema expression convention. A `by_caster_level` table chooses the row with the greatest minimum at or below the caster level; a zero extra-dice count means no extra damage roll. Slot tables select the stated slot or slot range. `duration: spell_duration` uses the extracted header or the explicit higher-slot duration override. Dice are random rolls, percentages are chances, and distances retain their stated units. Success damage fractions apply to the damage roll in the paired failure branch; no second roll is implied.

Spatial inputs supply actual positions, visibility, obstacles, and eligible objects. Campaign facts supply known places, creature identities, weather, and the other stated world facts. Source-authorized GM choices have explicit selection constraints: for example, Commune with Nature chooses one eligible CR 10+ creature, Plane Shift chooses an arrival in or near the named destination, and Teleport chooses the closest similar area. Control Weather specifies gradual recovery but gives no numerical recovery time; the contract exposes that timing as a constrained GM choice instead of inventing a rate.

The Otherworldly Steed and Giant Insect stat blocks are fully included in their spell contracts, including actions, traits, form options, and scaling. Produce Flame retains its fire-hurl action for the duration. Major Image uses the SRD 5.2.1 level 4+ permanent form. Forcecage requires concentration and consumes its component under the extracted header. Contagion counts the repeated saves separately from the initial save. The source does not provide a precise weather normalization schedule; no other unresolved source issue was found.

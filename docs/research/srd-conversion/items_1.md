---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:items/adamantine_armor:
    source_sha256: b330832896b95830c905e782a2858ca7a1d4cbb03cf4f9ef90122a385ecef85e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/adamantine_armor
        implementation: dnd2024:item_adamantine_armor
        parameters:
          equipment_state: worn
          critical_hits_received:
            replace: normal_hit
          attunement:
            required: false
    contract:
      id: dnd2024:item_adamantine_armor
      inputs:
        user: participant
        item: item_instance
      outputs:
      - modifiers
      events:
      - equipment.state_changed
      capabilities:
      - equipment
      - derived_statistics
    requires:
    - dnd2024:item_adamantine_armor
    citations:
    - SRD 5.2.1 pp. 209; builtin/srd-5.2.1/magic_item/adamantine_armor.md
  dnd2024:items/amulet_of_health:
    source_sha256: 29744e6455369640aef6fa5fe83af149204be80dcb9be3541e19bff23123876c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/amulet_of_health
        implementation: dnd2024:item_amulet_of_health
        parameters:
          equipment_state: worn
          ability:
            name: constitution
            set: 19
            only_if_unmodified_score_below: 19
          attunement:
            required: true
    contract:
      id: dnd2024:item_amulet_of_health
      inputs:
        user: participant
        item: item_instance
      outputs:
      - modifiers
      events:
      - equipment.state_changed
      capabilities:
      - equipment
      - derived_statistics
    requires:
    - dnd2024:item_amulet_of_health
    citations:
    - SRD 5.2.1 pp. 209; builtin/srd-5.2.1/magic_item/amulet_of_health.md
  dnd2024:items/animated_shield:
    source_sha256: 09924148527543d802db8af6635d75cd0d7a46ae824282e254b99b1544f65536
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/animated_shield
        implementation: dnd2024:item_animated_shield
        parameters:
          activation:
            action: bonus_action
            requires: holding
            duration:
              value: 1
              unit: minute
            shield_state: animated
            location: wielder_space
            protection: as_wielded
            hands_required: 0
            end_on:
            - duration
            - bonus_action_end
            - wielder_death
            - wielder_incapacitated
            on_end:
              if_free_hand: return_to_hand
              otherwise: fall_to_ground
          attunement:
            required: true
    contract:
      id: dnd2024:item_animated_shield
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_animated_shield
    citations:
    - SRD 5.2.1 pp. 209; builtin/srd-5.2.1/magic_item/animated_shield.md
  dnd2024:items/armor_of_invulnerability:
    source_sha256: 108a42f04e72e4000a240085aa28cae306101e6c540f3779ae5d308943825f6d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/armor_of_invulnerability
        implementation: dnd2024:item_armor_of_invulnerability
        parameters:
          equipment_state: worn
          resistance:
          - bludgeoning
          - piercing
          - slashing
          metal_shell:
            action: magic
            resource:
              uses: 1
              recovery: dawn
            duration:
              value: 10
              unit: minute
            immunity:
            - bludgeoning
            - piercing
            - slashing
            end_on:
            - duration
            - armor_removed
          attunement:
            required: true
    contract:
      id: dnd2024:item_armor_of_invulnerability
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_armor_of_invulnerability
    citations:
    - SRD 5.2.1 pp. 210; builtin/srd-5.2.1/magic_item/armor_of_invulnerability.md
  dnd2024:items/arrow_catching_shield:
    source_sha256: c6c15ca9b5bed878ea37eec024193bb5419e45cb1baf7044f4b8cb53c1bf35e3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/arrow_catching_shield
        implementation: dnd2024:item_arrow_catching_shield
        parameters:
          equipment_state: wielded
          armor_class:
            bonus: 2
            against: ranged_attack_roll
            stacks_with_normal_shield: true
          redirect:
            action: reaction
            trigger: ranged_attack_roll_against_target_within_5_feet
            target_distance_ft: 5
            replace_target: self
          attunement:
            required: true
    contract:
      id: dnd2024:item_arrow_catching_shield
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
    requires:
    - dnd2024:item_arrow_catching_shield
    citations:
    - SRD 5.2.1 pp. 211; builtin/srd-5.2.1/magic_item/arrow_catching_shield.md
  dnd2024:items/bag_of_holding:
    source_sha256: 25ef09fc35897277d100419c0d05a7fb2732b35b211e2eed5c3a9be23ec65b35
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/bag_of_holding
        implementation: dnd2024:item_bag_of_holding
        parameters:
          container:
            extradimensional: true
            interior_approximate_ft:
            - 2
            - 2
            - 4
            capacity_weight_lb: 500
            capacity_volume_cubic_ft: 64
            weight_lb: 5
            weight_independent_of_contents: true
            retrieve_action: utilize
            destruction_triggers:
            - overloaded
            - pierced
            - torn
            destruction_contents_destination:
              plane: astral
              distribution: scattered
            inverted:
              contents: spill_unharmed
              usable: false
              restore: turn_right_side_out
            air_supply:
              minutes: 10
              divide_by: breathing_creature_count
            interior_dimensions_are_approximate: true
          nested_extradimensional:
            trigger: placed_inside_item_created_extradimensional_space
            examples:
            - key: dnd2024:items/handy_haversack
            - key: dnd2024:items/portable_hole
            includes_similar_items: true
            destroy: both_items
            gate:
              origin: nesting_location
              shape: sphere
              radius_ft: 10
              targets: all_creatures_in_area
              destination:
                plane: astral
                location: random
              one_way: true
              closes_immediately_after_transport: true
              reopenable: false
          attunement:
            required: false
    contract:
      id: dnd2024:item_bag_of_holding
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
      outputs:
      - item_state
      - locations
      - effects
      events:
      - item.used
      - item.damaged
      - item.placed
      capabilities:
      - inventory
      - extradimensional_spaces
      - planar_travel
    requires:
    - dnd2024:item_bag_of_holding
    citations:
    - SRD 5.2.1 pp. 212; builtin/srd-5.2.1/magic_item/bag_of_holding.md
  dnd2024:items/bead_of_nourishment:
    source_sha256: 3de0e8889c51c70e88a0657f7256fac1bc461cb31f0679ff9dfe99ebf78bd02c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/bead_of_nourishment
        implementation: dnd2024:item_bead_of_nourishment
        parameters:
          consumption:
            trigger: dissolve_on_tongue
            consume_item: true
            nourishment_days_of_rations: 1
          attunement:
            required: false
    contract:
      id: dnd2024:item_bead_of_nourishment
      inputs:
        user: participant
        item: item_instance
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - consumables
      - effects
    requires:
    - dnd2024:item_bead_of_nourishment
    citations:
    - SRD 5.2.1 pp. 213; builtin/srd-5.2.1/magic_item/bead_of_nourishment.md
  dnd2024:items/berserker_axe:
    source_sha256: b9d6e06c20b9a426e6b5d71bd4d1bd914fe0f8e5ffe6b63ad4a3124b54598214
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/berserker_axe
        implementation: dnd2024:item_berserker_axe
        parameters:
          weapon_bonus:
            attack: 1
            damage: 1
          attuned_passive:
            hit_point_maximum_bonus:
              value_from: character_level
          curse:
            starts: attunement
            unwilling_to_part: true
            must_keep: within_reach
            other_weapon_attacks: disadvantage
            berserk_trigger:
              event: damage_received
              source: other_creature
              requires: item_in_possession
              save:
                ability: wisdom
                dc: 15
              failure: berserk
            berserk:
              end_check: self_turn_start
              end_if:
                no_creatures_seen_or_heard_within_ft: 60
              enemy:
                select: nearest_seen_or_heard_creature
                tie: random
              turn:
                movement: as_close_to_enemy_as_possible
                action: attack
                attack_target: enemy
                weapon: this_item
                if_unreachable:
                  spend: all_available_movement
                  then: end_turn
              retarget_on:
              - target_death
              - target_neither_seen_nor_heard
          attunement:
            required: true
    contract:
      id: dnd2024:item_berserker_axe
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - curse_state
      - effects
      events:
      - equipment.state_changed
      - attunement.completed
      - damage.received
      capabilities:
      - curses
      - equipment
      - effects
    requires:
    - dnd2024:item_berserker_axe
    citations:
    - SRD 5.2.1 pp. 213; builtin/srd-5.2.1/magic_item/berserker_axe.md
  dnd2024:items/boots_of_speed:
    source_sha256: 5086a5ae52e90b9b434d77ae412a99bcdf8f660580164355dad41a3ffc246aa1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/boots_of_speed
        implementation: dnd2024:item_boots_of_speed
        parameters:
          equipment_state: worn
          activation:
            action: bonus_action
            gesture: click_heels
            speed_multiplier: 2
            opportunity_attacks_against: disadvantage
            end_gesture: click_heels_again
          resource:
            active_duration:
              value: 10
              unit: minute
            scope: wearer
            accumulates_across_activations: true
            on_exhaustion: deactivate
            recovery: wearer_long_rest
          attunement:
            required: true
    contract:
      id: dnd2024:item_boots_of_speed
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_boots_of_speed
    citations:
    - SRD 5.2.1 pp. 214; builtin/srd-5.2.1/magic_item/boots_of_speed.md
  dnd2024:items/bowl_of_commanding_water_elementals:
    source_sha256: e53f17bcf719bac8d5304d07742aaf27eee5632f4874db7e004d6462211d1c00
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/bowl_of_commanding_water_elementals
        implementation: dnd2024:item_bowl_of_commanding_water_elementals
        parameters:
          activation:
            action: magic
            distance_ft: 5
            requires: bowl_filled_with_water
            resource:
              uses: 1
              recovery: dawn
            summon:
              creature:
                key: dnd2024:creatures/water_elemental
              placement:
                unoccupied: true
                selection: nearest_item
              understands: summoner_languages
              commands: obey
              initiative: immediately_after_summoner_same_count
              duration:
                value: 1
                unit: hour
              end_on:
              - death
              - summoner_bonus_action_dismissal
          container:
            diameter_ft: 1
            depth_ft: 0.5
            capacity_gallons: 3
          attunement:
            required: false
    contract:
      id: dnd2024:item_bowl_of_commanding_water_elementals
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        locations:
          type: location_list
          required: false
      outputs:
      - summoned_creatures
      - item_state
      events:
      - item.used
      - turn.started
      - creature.died
      capabilities:
      - summoning
      - initiative
      - item_resources
    requires:
    - dnd2024:item_bowl_of_commanding_water_elementals
    citations:
    - SRD 5.2.1 pp. 214; builtin/srd-5.2.1/magic_item/bowl_of_commanding_water_elementals.md
  dnd2024:items/brazier_of_commanding_fire_elementals:
    source_sha256: 4588cc8e62d158feb968cfe67fcf45c0af44f5dd7ba94b95f0d78e732beb338b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/brazier_of_commanding_fire_elementals
        implementation: dnd2024:item_brazier_of_commanding_fire_elementals
        parameters:
          activation:
            action: magic
            distance_ft: 5
            resource:
              uses: 1
              recovery: dawn
            summon:
              creature:
                key: dnd2024:creatures/fire_elemental
              placement:
                unoccupied: true
                selection: nearest_item
              understands: summoner_languages
              commands: obey
              initiative: immediately_after_summoner_same_count
              duration:
                value: 1
                unit: hour
              end_on:
              - death
              - summoner_bonus_action_dismissal
          attunement:
            required: false
    contract:
      id: dnd2024:item_brazier_of_commanding_fire_elementals
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        locations:
          type: location_list
          required: false
      outputs:
      - summoned_creatures
      - item_state
      events:
      - item.used
      - turn.started
      - creature.died
      capabilities:
      - summoning
      - initiative
      - item_resources
    requires:
    - dnd2024:item_brazier_of_commanding_fire_elementals
    citations:
    - SRD 5.2.1 pp. 214; builtin/srd-5.2.1/magic_item/brazier_of_commanding_fire_elementals.md
  dnd2024:items/candle_of_invocation:
    source_sha256: 799444d5669d2137e9576d91f670850a5dc320ea0f91c7e55b01a06e35ca6049
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/candle_of_invocation
        implementation: dnd2024:item_candle_of_invocation
        parameters:
          activation:
            action: magic
            operation: light
          burn:
            total:
              value: 4
              unit: hour
            accounting_increment:
              value: 1
              unit: minute
            snuff_and_resume: true
            on_exhaustion: destroy_item
          lit_effects:
            light:
              kind: dim
              radius_ft: 30
            attuned_user_in_light:
              d20_tests: advantage
            cleric_or_druid_in_light:
              prepared_spells_level: 1
              spell_slot_cost: 0
          first_lighting_alternative:
            cast:
              spell:
                key: dnd2024:spells/gate
            destroys_item: true
            portal_destination:
              plane_selection:
              - gm_choice
              - d100
              table:
              - min: 1
                max: 5
                plane: abyss
              - min: 6
                max: 10
                plane: acheron
              - min: 11
                max: 17
                plane: arborea
              - min: 18
                max: 25
                plane: arcadia
              - min: 26
                max: 33
                plane: beastlands
              - min: 34
                max: 41
                plane: bytopia
              - min: 42
                max: 46
                plane: carceri
              - min: 47
                max: 54
                plane: elysium
              - min: 55
                max: 59
                plane: gehenna
              - min: 60
                max: 64
                plane: hades
              - min: 65
                max: 69
                plane: limbo
              - min: 70
                max: 77
                plane: mechanus
              - min: 78
                max: 85
                plane: mount_celestia
              - min: 86
                max: 90
                plane: nine_hells
              - min: 91
                max: 95
                plane: pandemonium
              - min: 96
                max: 100
                plane: ysgard
          attunement:
            required: true
    contract:
      id: dnd2024:item_candle_of_invocation
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
      - spatial_queries
    requires:
    - dnd2024:item_candle_of_invocation
    citations:
    - SRD 5.2.1 pp. 214, 215; builtin/srd-5.2.1/magic_item/candle_of_invocation.md
  dnd2024:items/censer_of_controlling_air_elementals:
    source_sha256: dda18667eed5e96161abd1482263a3f0068d329fe6b54dc5a5934954f055a92d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/censer_of_controlling_air_elementals
        implementation: dnd2024:item_censer_of_controlling_air_elementals
        parameters:
          activation:
            action: magic
            requires: gently_swinging_censer
            resource:
              uses: 1
              recovery: dawn
            summon:
              creature:
                key: dnd2024:creatures/air_elemental
              placement:
                unoccupied: true
                selection: nearest_item
              understands: summoner_languages
              commands: obey
              initiative: immediately_after_summoner_same_count
              duration:
                value: 1
                unit: hour
              end_on:
              - death
              - summoner_bonus_action_dismissal
          attunement:
            required: false
    contract:
      id: dnd2024:item_censer_of_controlling_air_elementals
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        locations:
          type: location_list
          required: false
      outputs:
      - summoned_creatures
      - item_state
      events:
      - item.used
      - turn.started
      - creature.died
      capabilities:
      - summoning
      - initiative
      - item_resources
    requires:
    - dnd2024:item_censer_of_controlling_air_elementals
    citations:
    - SRD 5.2.1 pp. 215; builtin/srd-5.2.1/magic_item/censer_of_controlling_air_elementals.md
  dnd2024:items/cloak_of_arachnida:
    source_sha256: 85f90f0be78d6406c1743d6a1ba57f43c43b74cf999789b72fd18b42c0dc05f4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/cloak_of_arachnida
        implementation: dnd2024:item_cloak_of_arachnida
        parameters:
          equipment_state: worn
          resistance:
          - poison
          climb:
            speed: walking_speed
            vertical_surfaces: true
            ceilings: true
            hands_free: true
          webs:
            cannot_be_caught: true
            movement: difficult_terrain
          spells:
          - spell:
              key: dnd2024:spells/web
            save_dc: 13
            area_multiplier: 2
            resource:
              uses: 1
              recovery: dawn
            area_multiplier_scope: web_area
          attunement:
            required: true
    contract:
      id: dnd2024:item_cloak_of_arachnida
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_cloak_of_arachnida
    citations:
    - SRD 5.2.1 pp. 215; builtin/srd-5.2.1/magic_item/cloak_of_arachnida.md
  dnd2024:items/cloak_of_invisibility:
    source_sha256: 7db51db3da33c12885fdf522454664aef4017273f564a330ff193f7eee79283a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/cloak_of_invisibility
        implementation: dnd2024:item_cloak_of_invisibility
        parameters:
          equipment_state: worn
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 3
                bonus: 0
          activation:
            action: magic
            gesture: pull_hood_up
            cost: 1
            condition: invisible
            duration:
              value: 1
              unit: hour
            end_on:
            - hood_lowered
            - cloak_removed
            lower_hood_action: none
          attunement:
            required: true
    contract:
      id: dnd2024:item_cloak_of_invisibility
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_cloak_of_invisibility
    citations:
    - SRD 5.2.1 pp. 215, 216; builtin/srd-5.2.1/magic_item/cloak_of_invisibility.md
  dnd2024:items/cloak_of_the_manta_ray:
    source_sha256: c58e9757177bb251508a5e4454d583327ee80f72e9b48a957c78905864aae755
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/cloak_of_the_manta_ray
        implementation: dnd2024:item_cloak_of_the_manta_ray
        parameters:
          equipment_state: worn
          breathing:
          - underwater
          swim_speed_ft: 60
          attunement:
            required: true
    contract:
      id: dnd2024:item_cloak_of_the_manta_ray
      inputs:
        user: participant
        item: item_instance
      outputs:
      - modifiers
      events:
      - equipment.state_changed
      capabilities:
      - equipment
      - derived_statistics
    requires:
    - dnd2024:item_cloak_of_the_manta_ray
    citations:
    - SRD 5.2.1 pp. 216; builtin/srd-5.2.1/magic_item/cloak_of_the_manta_ray.md
  dnd2024:items/crystal_ball_of_telepathy:
    source_sha256: dd1e1936078bddeee4c946cd331dce1f0395ad35fa92aee9e79509f0df710831
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/crystal_ball_of_telepathy
        implementation: dnd2024:item_crystal_ball_of_telepathy
        parameters:
          equipment_state: touching
          spells:
          - spell:
              key: dnd2024:spells/scrying
            save_dc: 17
          - spell:
              key: dnd2024:spells/suggestion
            save_dc: 17
            resource:
              uses: 1
              recovery: dawn
            target:
              seen: true
              within_sensor_ft: 30
            origin: scrying_sensor
            concentration: false
            end_on:
            - scrying_ends
          telepathy:
            targets: seen_creatures
            within_sensor_ft: 30
            requires: scrying_active
          attunement:
            required: true
    contract:
      id: dnd2024:item_crystal_ball_of_telepathy
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_crystal_ball_of_telepathy
    citations:
    - SRD 5.2.1 pp. 216; builtin/srd-5.2.1/magic_item/crystal_ball_of_telepathy.md
  dnd2024:items/cubic_gate:
    source_sha256: 80e593db86253cb966237e963d39366ae738c410463659b1f26c4260c1a583b6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/cubic_gate
        implementation: dnd2024:item_cubic_gate
        parameters:
          dimensions:
            side_inches: 3
          planes:
            side_count: 6
            all_distinct: true
            fixed:
            - material
            gm_choices: 5
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 3
                bonus: 0
          activation:
            action: magic
            cost: 1
            options:
            - spell:
                key: dnd2024:spells/gate
              presses: 1
              destination: selected_side_plane
            - spell:
                key: dnd2024:spells/plane_shift
              presses: 2
              destination: selected_side_plane
          attunement:
            required: false
    contract:
      id: dnd2024:item_cubic_gate
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
      - spatial_queries
    requires:
    - dnd2024:item_cubic_gate
    citations:
    - SRD 5.2.1 pp. 216; builtin/srd-5.2.1/magic_item/cubic_gate.md
  dnd2024:items/decanter_of_endless_water:
    source_sha256: 3734f765d508dfb9e1bb3aa033ac7415991af9934af024fd942219fc7b61610e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/decanter_of_endless_water
        implementation: dnd2024:item_decanter_of_endless_water
        parameters:
          weight_lb: 2
          activation:
            action: magic
            remove_stopper: true
            command_word: true
            water_type_choice:
            - fresh
            - salt
            end: self_next_turn_start
            options:
              splash:
                gallons: 1
              fountain:
                gallons: 5
              geyser:
                gallons: 30
                shape: line
                length_ft: 30
                width_ft: 1
                aim:
                  requires: holding
                  action: none
                  directions: 1
                target:
                  choice:
                  - one_creature_in_line
                  - one_object_in_line
                creature:
                  save:
                    ability: strength
                    dc: 13
                  failure:
                    damage:
                      roll:
                        dice: 1
                        sides: 4
                        bonus: 0
                      type: bludgeoning
                    condition: prone
                object:
                  worn_or_carried: false
                  maximum_weight_lb: 200
                  effect: knocked_over
          attunement:
            required: false
    contract:
      id: dnd2024:item_decanter_of_endless_water
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
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
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
      - spatial_queries
    requires:
    - dnd2024:item_decanter_of_endless_water
    citations:
    - SRD 5.2.1 pp. 217; builtin/srd-5.2.1/magic_item/decanter_of_endless_water.md
  dnd2024:items/demon_armor:
    source_sha256: afc9179851b25fb5869ef98f490857220eff8c2dc66261e237b783b46b26c221
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/demon_armor
        implementation: dnd2024:item_demon_armor
        parameters:
          equipment_state: worn
          armor_class_bonus: 1
          languages:
          - abyssal
          unarmed_strike:
            damage_dice:
              dice: 1
              sides: 8
              bonus: 0
            damage_type: slashing
            replaces: normal_bludgeoning_damage
            attack_bonus: 1
            damage_bonus: 1
          curse:
            starts: don_armor
            cannot_doff: true
            unlock:
            - spell:
                key: dnd2024:spells/remove_curse
            - magic: similar_curse_removal
            attack_disadvantage_against: demons
            save_disadvantage_against:
              source: demons
              effect:
              - spells
              - special_abilities
            attunement_required: false
          attunement:
            required: true
    contract:
      id: dnd2024:item_demon_armor
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - curse_state
      - effects
      events:
      - equipment.state_changed
      - attunement.completed
      - damage.received
      - spell.cast_completed
      capabilities:
      - curses
      - equipment
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_demon_armor
    citations:
    - SRD 5.2.1 pp. 218; builtin/srd-5.2.1/magic_item/demon_armor.md
  dnd2024:items/dragon_scale_mail:
    source_sha256: 0e5a5805078c85b318ef59e45d9f336e4154afb36269b0e2f57df21843e30ddd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/dragon_scale_mail
        implementation: dnd2024:item_dragon_scale_mail
        parameters:
          equipment_state: worn
          armor_class_bonus: 1
          saving_throws:
            advantage_against: dragon_breath_weapons
          variants:
            black:
              resistance: acid
            blue:
              resistance: lightning
            brass:
              resistance: fire
            bronze:
              resistance: lightning
            copper:
              resistance: acid
            gold:
              resistance: fire
            green:
              resistance: poison
            red:
              resistance: fire
            silver:
              resistance: cold
            white:
              resistance: cold
          detection:
            action: magic
            resource:
              uses: 1
              recovery: dawn
            target: closest_dragon_matching_armor_type
            range_miles: 30
            outputs:
            - distance
            - direction
          attunement:
            required: true
    contract:
      id: dnd2024:item_dragon_scale_mail
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
      - spatial_queries
    requires:
    - dnd2024:item_dragon_scale_mail
    citations:
    - SRD 5.2.1 pp. 218, 219; builtin/srd-5.2.1/magic_item/dragon_scale_mail.md
  dnd2024:items/dust_of_dryness:
    source_sha256: 04b7f08056bfd71d829b86bdeb3cc388c29696beb4d5976ebdf86760ca805826
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/dust_of_dryness
        implementation: dnd2024:item_dust_of_dryness
        parameters:
          pinches:
            initial:
              dice: 1
              sides: 6
              bonus: 4
            recovery: null
          water_absorption:
            action: utilize
            cost_pinches: 1
            maximum_cube_side_ft: 15
            output:
              kind: pellet
              size: marble
              weight: negligible
              placement: near_sprinkled_dust
              floats_or_rests: true
              stores: absorbed_water
            release:
              action: utilize
              requires: smash_against_hard_surface
              water: all_stored
              destroy_pellet: true
              end_magic: true
          elemental_damage:
            action: utilize
            cost_pinches: 1
            range_ft: 5
            target:
              type: elemental
              composition: mostly_water
            save:
              ability: constitution
              dc: 13
            damage:
              roll:
                dice: 10
                sides: 6
                bonus: 0
              type: necrotic
              success_multiplier: 0.5
          attunement:
            required: false
    contract:
      id: dnd2024:item_dust_of_dryness
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - consumables
      - effects
      - item_resources
    requires:
    - dnd2024:item_dust_of_dryness
    citations:
    - SRD 5.2.1 pp. 219; builtin/srd-5.2.1/magic_item/dust_of_dryness.md
  dnd2024:items/dwarven_thrower:
    source_sha256: 2acb43caa61c5d310b99c931e440bec787c753995e90198081571370a0344da5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/dwarven_thrower
        implementation: dnd2024:item_dwarven_thrower
        parameters:
          weapon_bonus:
            attack: 3
            damage: 3
          attunement_eligibility:
            any_of:
            - species: dwarf
            - attuned_item:
                key: dnd2024:items/belt_of_dwarvenkind
          thrown:
            normal_range_ft: 20
            long_range_ft: 60
          ranged_hit_extra_damage:
            type: force
            normal:
              dice: 1
              sides: 8
              bonus: 0
            giant:
              dice: 2
              sides: 8
              bonus: 0
          return_to_hand:
            event: ranged_attack_hit_or_miss
            timing: immediate
          attunement:
            required: true
            eligibility:
              any_of:
              - species: dwarf
              - attuned_item:
                  key: dnd2024:items/belt_of_dwarvenkind
    contract:
      id: dnd2024:item_dwarven_thrower
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_dwarven_thrower
    citations:
    - SRD 5.2.1 pp. 219; builtin/srd-5.2.1/magic_item/dwarven_thrower.md
  dnd2024:items/elemental_gem:
    source_sha256: 9b5e89bf853d0e85419e95fd244bc9d8f48ef6a264445c4889ffda0882e74e6a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/elemental_gem
        implementation: dnd2024:item_elemental_gem
        parameters:
          activation:
            action: utilize
            operation: break_gem
            item_magic_ends: true
            summon:
              creature:
                value_from: variant.creature
              placement:
                unoccupied: true
                selection: nearest_item
              understands: summoner_languages
              commands: obey
              initiative: immediately_after_summoner_same_count
              duration:
                value: 1
                unit: hour
              end_on:
              - death
              - summoner_bonus_action_dismissal
          variants:
            blue_sapphire:
              creature:
                key: dnd2024:creatures/air_elemental
            emerald:
              creature:
                key: dnd2024:creatures/water_elemental
            red_corundum:
              creature:
                key: dnd2024:creatures/fire_elemental
            yellow_diamond:
              creature:
                key: dnd2024:creatures/earth_elemental
          attunement:
            required: false
    contract:
      id: dnd2024:item_elemental_gem
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        locations:
          type: location_list
          required: false
      outputs:
      - summoned_creatures
      - item_state
      events:
      - item.used
      - turn.started
      - creature.died
      capabilities:
      - summoning
      - initiative
      - item_resources
    requires:
    - dnd2024:item_elemental_gem
    citations:
    - SRD 5.2.1 pp. 220; builtin/srd-5.2.1/magic_item/elemental_gem.md
  dnd2024:items/energy_bow:
    source_sha256: 82335e7feb50093e7dbf35ed8b4f7dbbde08c3957965adaf9683657086e9f9b8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/energy_bow
        implementation: dnd2024:item_energy_bow
        parameters:
          weapon_bonus:
            attack: 1
            damage: 1
          ammunition:
            requires_string: false
            generation_trigger: firing_motion
            kind: energy_arrow
            damage_type: force
            replaces_damage_type: piercing
            disappears_on:
            - hit
            - miss
            light:
              bright_radius_ft: 20
              additional_dim_ft: 20
          arrow_of_restraint:
            trigger: ranged_attack_against_creature
            replace_damage: true
            on_hit:
              save:
                ability: strength
                dc: 15
              failure:
                condition: restrained
                duration:
                  value: 1
                  unit: minute
            escape:
              actor: restrained_target
              action: action
              check:
                ability: strength
                skill: athletics
                dc: 20
              success: end_restraint
          arrow_of_transport:
            action: magic
            range_ft: 60
            target_seen: true
            eligible:
              any_of:
              - kind: creature
                willing: true
                maximum_size: medium
              - kind: object
                worn_or_carried: false
                fits_cube_ft: 5
            destination:
              range_from_user_ft: 10
              seen: true
              unoccupied: true
            effect: teleport
            attack_roll_required: false
          energy_ladder:
            action: magic
            wall_range_ft: 60
            maximum_length_ft: 60
            duration:
              value: 1
              unit: minute
            rungs: glowing_energy_arrows
          attunement:
            required: true
    contract:
      id: dnd2024:item_energy_bow
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
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
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
      - spatial_queries
    requires:
    - dnd2024:item_energy_bow
    citations:
    - SRD 5.2.1 pp. 220; builtin/srd-5.2.1/magic_item/energy_bow.md
  dnd2024:items/eyes_of_minute_seeing:
    source_sha256: 7872836b1b3d25c099d6a50e03b54f43cb7537f96c5f0368248daa773839564e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/eyes_of_minute_seeing
        implementation: dnd2024:item_eyes_of_minute_seeing
        parameters:
          equipment_state: worn
          darkvision_ft: 1
          investigation:
            advantage: true
            target_distance_ft: 1
          attunement:
            required: false
    contract:
      id: dnd2024:item_eyes_of_minute_seeing
      inputs:
        user: participant
        item: item_instance
      outputs:
      - modifiers
      events:
      - equipment.state_changed
      capabilities:
      - equipment
      - derived_statistics
    requires:
    - dnd2024:item_eyes_of_minute_seeing
    citations:
    - SRD 5.2.1 pp. 221; builtin/srd-5.2.1/magic_item/eyes_of_minute_seeing.md
  dnd2024:items/figurine_of_wondrous_power:
    source_sha256: c1577372bcbbf0608167d6564296e81e09c19b27bdeeac158d179f275384e4c8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/figurine_of_wondrous_power
        implementation: dnd2024:item_figurine_of_wondrous_power
        parameters:
          activation:
            action: magic
            operation: throw_to_ground
            range_ft: 60
            requires:
              space_unoccupied_by:
              - creatures
              - objects
              sufficient_space_for_creature: true
            invalid_space: no_transformation
          creature_behavior:
            attitude_to_user_and_allies: friendly
            understands: user_languages
            commands: obey
            initiative: immediately_after_user_same_count
            without_commands: defend_self_only
          reversion:
            'on':
            - duration_expires
            - creature_hp_zero
            - user_magic_action_while_touching_creature
            starts: variant_recovery
          variants:
            bronze_griffon:
              creature:
                key: dnd2024:creatures/griffon
              duration:
                value: 6
                unit: hour
              recovery:
                value: 5
                unit: day
              rarity: rare
            ebony_fly:
              creature:
                key: dnd2024:creatures/giant_fly
              duration:
                value: 12
                unit: hour
              recovery:
                value: 2
                unit: day
              rarity: rare
              mount: true
              creature_definition_owns_embedded_stat_block: true
            golden_lions:
              creature:
                key: dnd2024:creatures/lion
              duration:
                value: 1
                unit: hour
              recovery:
                value: 7
                unit: day
              rarity: rare
              set_count: 2
              activation_count_choices:
              - 1
              - 2
              resource_scope: each_figurine
            ivory_goats:
              rarity: rare
              set_count: 3
              members:
                terror:
                  creature:
                    key: dnd2024:creatures/giant_goat
                  duration:
                    value: 3
                    unit: hour
                  recovery:
                    value: 15
                    unit: day
                  can_attack: false
                  horns:
                    remove_action: magic
                    harmless_removal: true
                    weapons:
                    - base:
                        key: dnd2024:equipment/lance
                      bonus: 1
                    - base:
                        key: dnd2024:equipment/longsword
                      bonus: 2
                    on_revert:
                      weapons: disappear
                      horns: return
                  fear:
                    requires: user_riding_goat
                    shape: emanation
                    radius_ft: 30
                    origin: goat
                    trigger: hostile_creature_turn_start
                    save:
                      ability: wisdom
                      dc: 15
                    failure:
                      condition: frightened
                      duration:
                        value: 1
                        unit: minute
                      end_on:
                      - rider_dismounts
                      - goat_reverts
                    repeat_save: target_turn_end
                    success:
                      end_effect: true
                      immunity:
                        value: 24
                        unit: hour
                traveling:
                  creature:
                    key: dnd2024:creatures/riding_horse
                  appearance: goat
                  size: large
                  charges:
                    maximum: 24
                    initial: 24
                    cost:
                      per:
                        value: 1
                        unit: hour
                      partial_interval_rounding: up
                    multiple_activations: true
                    on_zero:
                      revert: true
                      unavailable_for:
                        value: 7
                        unit: day
                      then_recover: all_charges
                travail:
                  creature:
                    key: dnd2024:creatures/giant_goat
                  duration:
                    value: 3
                    unit: hour
                  recovery:
                    value: 30
                    unit: day
            marble_elephant:
              creature:
                key: dnd2024:creatures/elephant
              duration:
                value: 24
                unit: hour
              recovery:
                value: 7
                unit: day
              rarity: rare
            obsidian_steed:
              creature:
                key: dnd2024:creatures/nightmare
              duration:
                value: 24
                unit: hour
              recovery:
                value: 5
                unit: day
              rarity: very_rare
              combat: self_defense_only
              disobedience:
                probability_per_activation: 0.1
                ignores:
                - orders
                - revert_command
                if_mounted:
                  transport:
                  - rider
                  - nightmare
                  destination:
                    plane: hades
                    location: random
                  then: revert
            onyx_dog:
              creature:
                key: dnd2024:creatures/mastiff
              duration:
                value: 6
                unit: hour
              recovery:
                value: 7
                unit: day
              rarity: rare
              overrides:
                intelligence: 8
                languages:
                - common
                blindsight_ft: 60
            serpentine_owl:
              creature:
                key: dnd2024:creatures/giant_owl
              duration:
                value: 8
                unit: hour
              recovery:
                value: 2
                unit: day
              rarity: rare
              telepathy:
                with: user
                range: unlimited
                same_plane_required: true
            silver_raven:
              creature:
                key: dnd2024:creatures/raven
              duration:
                value: 12
                unit: hour
              recovery:
                value: 2
                unit: day
              rarity: uncommon
              spells:
              - spell:
                  key: dnd2024:spells/animal_messenger
                target: this_creature
                requires: raven_form
          attunement:
            required: false
    contract:
      id: dnd2024:item_figurine_of_wondrous_power
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - summoned_creatures
      - item_state
      events:
      - item.used
      - turn.started
      - creature.died
      - spell.cast_completed
      capabilities:
      - summoning
      - initiative
      - item_resources
      - item_spellcasting
      - spatial_queries
    requires:
    - dnd2024:item_figurine_of_wondrous_power
    citations:
    - SRD 5.2.1 pp. 221, 222; builtin/srd-5.2.1/magic_item/figurine_of_wondrous_power.md
  dnd2024:items/frost_brand:
    source_sha256: 2af8f5a8d0f38f6b8823367ea476d300f1b4cf82c7e48347567522d9bb600e7f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/frost_brand
        implementation: dnd2024:item_frost_brand
        parameters:
          on_hit:
            extra_damage:
              roll:
                dice: 1
                sides: 6
                bonus: 0
              type: cold
          held:
            resistance:
            - fire
          freezing_temperature:
            light:
              bright_radius_ft: 10
              additional_dim_ft: 10
          draw_weapon:
            optional: true
            extinguish:
              radius_ft: 30
              targets: all_nonmagical_flames
            recovery:
              value: 1
              unit: hour
          attunement:
            required: true
    contract:
      id: dnd2024:item_frost_brand
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
      - item_resources
    requires:
    - dnd2024:item_frost_brand
    citations:
    - SRD 5.2.1 pp. 223; builtin/srd-5.2.1/magic_item/frost_brand.md
  dnd2024:items/gem_of_seeing:
    source_sha256: c38893313aa57132b4707ab59d470bb9334191df41c0d33ca961095fed4da25b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/gem_of_seeing
        implementation: dnd2024:item_gem_of_seeing
        parameters:
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 3
                bonus: 0
          activation:
            action: magic
            cost: 1
            duration:
              value: 10
              unit: minute
            truesight_ft: 120
            requires_during_effect: peer_through_gem
          attunement:
            required: true
    contract:
      id: dnd2024:item_gem_of_seeing
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_gem_of_seeing
    citations:
    - SRD 5.2.1 pp. 223; builtin/srd-5.2.1/magic_item/gem_of_seeing.md
  dnd2024:items/gloves_of_missile_snaring:
    source_sha256: 28a29a57334447b0e374fa68a7ce5c4b2290f1236928dae4cf1c52f270268abd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/gloves_of_missile_snaring
        implementation: dnd2024:item_gloves_of_missile_snaring
        parameters:
          equipment_state: worn
          reaction:
            trigger:
              hit_by: attack_roll
              weapon_category:
              - ranged
              - thrown
            requires: free_hand
            damage_reduction:
              sum:
              - dice: 1
                sides: 10
                bonus: 0
              - value_from: dexterity_modifier
            if_damage_reduced_to_zero:
              may_catch:
              - ammunition
              - weapon
              requires: fits_free_hand
          attunement:
            required: true
    contract:
      id: dnd2024:item_gloves_of_missile_snaring
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
    requires:
    - dnd2024:item_gloves_of_missile_snaring
    citations:
    - SRD 5.2.1 pp. 224; builtin/srd-5.2.1/magic_item/gloves_of_missile_snaring.md
  dnd2024:items/goggles_of_night:
    source_sha256: 8c2e4c4532c6c7f667541dc18c95e283489e34eb8c6ec4b10d9170f40b44081a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/goggles_of_night
        implementation: dnd2024:item_goggles_of_night
        parameters:
          equipment_state: worn
          darkvision:
            without_existing_ft: 60
            existing_range_bonus_ft: 60
          attunement:
            required: false
    contract:
      id: dnd2024:item_goggles_of_night
      inputs:
        user: participant
        item: item_instance
      outputs:
      - modifiers
      events:
      - equipment.state_changed
      capabilities:
      - equipment
      - derived_statistics
    requires:
    - dnd2024:item_goggles_of_night
    citations:
    - SRD 5.2.1 pp. 224; builtin/srd-5.2.1/magic_item/goggles_of_night.md
  dnd2024:items/hat_of_disguise:
    source_sha256: e39f052772ff14235a15c475a09d9bb3b977d21c5b62dc8dee9afdb69ceb83e8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/hat_of_disguise
        implementation: dnd2024:item_hat_of_disguise
        parameters:
          equipment_state: worn
          spells:
          - spell:
              key: dnd2024:spells/disguise_self
            end_on:
            - hat_removed
          attunement:
            required: true
    contract:
      id: dnd2024:item_hat_of_disguise
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_hat_of_disguise
    citations:
    - SRD 5.2.1 pp. 224; builtin/srd-5.2.1/magic_item/hat_of_disguise.md
  dnd2024:items/helm_of_brilliance:
    source_sha256: 8beeed50c15db9bf764269a402b484884eb360dcca09017578abd2a2c011c440
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/helm_of_brilliance
        implementation: dnd2024:item_helm_of_brilliance
        parameters:
          equipment_state: worn
          gems:
            initial:
              diamond:
                dice: 1
                sides: 10
                bonus: 0
              ruby:
                dice: 2
                sides: 10
                bonus: 0
              fire_opal:
                dice: 3
                sides: 10
                bonus: 0
              opal:
                dice: 4
                sides: 10
                bonus: 0
            pried_out: destroy_gem
            none_remaining: item_loses_magic
          diamond_light:
            requires_gem: diamond
            shape: emanation
            radius_ft: 30
            light_if_undead_present: dim
            undead_turn_start_damage:
              roll:
                dice: 1
                sides: 6
                bonus: 0
              type: radiant
          fire_opal_flames:
            requires_gem: fire_opal
            action: magic
            target: one_held_weapon
            light:
              bright_radius_ft: 10
              additional_dim_ft: 10
            harmless_to:
            - user
            - weapon
            on_hit_extra_damage:
              roll:
                dice: 1
                sides: 6
                bonus: 0
              type: fire
            end_on:
            - bonus_action_extinguish
            - weapon_dropped
            - weapon_stowed
          ruby_resistance:
            requires_gem: ruby
            resistance: fire
          spells:
          - spell:
              key: dnd2024:spells/daylight
            save_dc: 18
            component:
              gem: opal
              quantity: 1
              consumed: true
              remove_from_helm: true
          - spell:
              key: dnd2024:spells/fireball
            save_dc: 18
            component:
              gem: fire_opal
              quantity: 1
              consumed: true
              remove_from_helm: true
          - spell:
              key: dnd2024:spells/prismatic_spray
            save_dc: 18
            component:
              gem: diamond
              quantity: 1
              consumed: true
              remove_from_helm: true
          - spell:
              key: dnd2024:spells/wall_of_fire
            save_dc: 18
            component:
              gem: ruby
              quantity: 1
              consumed: true
              remove_from_helm: true
          fire_damage_mishap:
            trigger:
              event: fire_damage_received
              cause: failed_save_against_spell
            roll:
              dice: 1
              sides: 20
              bonus: 0
            on_value: 1
            effect:
              shape: emanation
              radius_ft: 60
              targets: all_creatures
              save:
                ability: dexterity
                dc: 17
              failure_damage:
                value_from: remaining_gem_count
                type: radiant
              success_damage: 0
              then: destroy_helm
          attunement:
            required: true
    contract:
      id: dnd2024:item_helm_of_brilliance
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_helm_of_brilliance
    citations:
    - SRD 5.2.1 pp. 225, 226; builtin/srd-5.2.1/magic_item/helm_of_brilliance.md
  dnd2024:items/helm_of_teleportation:
    source_sha256: 4681e343694f62bcbd8b8c25cf8bac27248bb624f57334f2824cdedb21216ec8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/helm_of_teleportation
        implementation: dnd2024:item_helm_of_teleportation
        parameters:
          equipment_state: worn
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 3
                bonus: 0
          spells:
          - spell:
              key: dnd2024:spells/teleport
            cost: 1
          attunement:
            required: true
    contract:
      id: dnd2024:item_helm_of_teleportation
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        locations:
          type: location_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_helm_of_teleportation
    citations:
    - SRD 5.2.1 pp. 226; builtin/srd-5.2.1/magic_item/helm_of_teleportation.md
  dnd2024:items/horn_of_valhalla:
    source_sha256: c009f04a1bde5026add7874095f4e00fcf1923e73482e65b9c5f3ba60d833e69
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/horn_of_valhalla
        implementation: dnd2024:item_horn_of_valhalla
        parameters:
          activation:
            action: magic
            operation: blow_horn
            recovery:
              value: 7
              unit: day
            summon:
              creature:
                key: dnd2024:creatures/berserker
              count_from: variant.spirits
              placement:
                unoccupied: true
                range_ft: 60
              origin_plane: ysgard
              duration:
                value: 1
                unit: hour
              return_on:
              - duration
              - hp_zero
              condition_immunities:
              - charmed
              - frightened
              requirements_met:
                friendly_to:
                - user
                - allies
                commands: obey
              requirements_not_met:
                attack: user
          variant_selection:
          - gm_choice
          - d100
          variants:
          - min: 1
            max: 40
            metal: silver
            rarity: rare
            spirits: 2
            requirement: null
          - min: 41
            max: 75
            metal: brass
            rarity: rare
            spirits: 3
            requirement:
              proficiency: all_simple_weapons
          - min: 76
            max: 90
            metal: bronze
            rarity: very_rare
            spirits: 4
            requirement:
              training: all_medium_armor
          - min: 91
            max: 100
            metal: iron
            rarity: legendary
            spirits: 5
            requirement:
              proficiency: all_martial_weapons
          attunement:
            required: false
    contract:
      id: dnd2024:item_horn_of_valhalla
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - summoned_creatures
      - item_state
      events:
      - item.used
      - turn.started
      - creature.died
      capabilities:
      - summoning
      - initiative
      - item_resources
    requires:
    - dnd2024:item_horn_of_valhalla
    citations:
    - SRD 5.2.1 pp. 226; builtin/srd-5.2.1/magic_item/horn_of_valhalla.md
  dnd2024:items/immovable_rod:
    source_sha256: 360beb55a8bd0489319eb4c8542634c01755fbea0cdf27574a60fdd488f866d5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/immovable_rod
        implementation: dnd2024:item_immovable_rod
        parameters:
          toggle:
            action: utilize
            actor: any_creature
            operation: press_button
            states:
            - fixed
            - unfixed
          fixed:
            gravity_ignored: true
            maximum_load_lb: 8000
            overload:
            - deactivate
            - fall
            move:
              action: utilize
              check:
                ability: strength
                skill: athletics
                dc: 30
              success_distance_max_ft: 10
          attunement:
            required: false
    contract:
      id: dnd2024:item_immovable_rod
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
      - spatial_queries
    requires:
    - dnd2024:item_immovable_rod
    citations:
    - SRD 5.2.1 pp. 227; builtin/srd-5.2.1/magic_item/immovable_rod.md
  dnd2024:items/iron_bands:
    source_sha256: c541ae75671f29e338cbcf3bb07f40bdc3e9c76a9b0107d03f3ba352cd795b30
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/iron_bands
        implementation: dnd2024:item_iron_bands
        parameters:
          dimensions:
            diameter_inches: 3
            weight_lb: 1
          activation:
            action: magic
            operation: throw
            target:
              maximum_size: huge
              seen: true
              range_ft: 60
            attack:
              kind: ranged
              bonus:
                sum:
                - value_from: dexterity_modifier
                - value_from: proficiency_bonus
            on_hit:
              condition: restrained
              duration: until_released_or_broken
            on_miss: revert_to_sphere
            resource:
              uses: 1
              recovery: dawn
          release:
            actor: user
            action: bonus_action
            command: true
            effects:
            - end_restraint
            - revert_to_sphere
          break_attempt:
            actor: any_creature_touching_bands
            action: action
            check:
              ability: strength
              skill: athletics
              dc: 20
            success:
            - destroy_item
            - free_creature
            failure:
              actor_further_attempts: automatic_failure
              duration:
                value: 24
                unit: hour
          attunement:
            required: false
    contract:
      id: dnd2024:item_iron_bands
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        locations:
          type: location_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
      - spatial_queries
    requires:
    - dnd2024:item_iron_bands
    citations:
    - SRD 5.2.1 pp. 228; builtin/srd-5.2.1/magic_item/iron_bands.md
  dnd2024:items/lantern_of_revealing:
    source_sha256: 3dab090a0c932d276822dbfca89ba9b7883abfda7debc2a51ad780ca510727bf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/lantern_of_revealing
        implementation: dnd2024:item_lantern_of_revealing
        parameters:
          fuel:
            oil_pints: 1
            burn_duration:
              value: 6
              unit: hour
          lit:
            bright_radius_ft: 30
            additional_dim_ft: 30
            reveal_invisible:
              targets:
              - creatures
              - objects
              within: bright_light
          lower_hood:
            action: utilize
            replace_light:
              dim_radius_ft: 5
              bright_radius_ft: 0
          attunement:
            required: false
    contract:
      id: dnd2024:item_lantern_of_revealing
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_lantern_of_revealing
    citations:
    - SRD 5.2.1 pp. 228, 229; builtin/srd-5.2.1/magic_item/lantern_of_revealing.md
  dnd2024:items/mace_of_smiting:
    source_sha256: 05b9ab9c1c215f0d16042925860ab93449f40fc3a19697bd74c0e5f79c6ba49c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/mace_of_smiting
        implementation: dnd2024:item_mace_of_smiting
        parameters:
          weapon_bonus:
            default: 1
            against_construct: 3
            applies:
            - attack
            - damage
          natural_twenty:
            extra_damage:
              type: bludgeoning
              default: 7
              construct: 14
            after_damage:
              target_type: construct
              hp_maximum: 25
              effect: destroy
          attunement:
            required: false
    contract:
      id: dnd2024:item_mace_of_smiting
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
    requires:
    - dnd2024:item_mace_of_smiting
    citations:
    - SRD 5.2.1 pp. 229; builtin/srd-5.2.1/magic_item/mace_of_smiting.md
  dnd2024:items/manual_of_bodily_health:
    source_sha256: 499a32104724cbb0506facc0a29a6bef2dd58bec08a73295b7c53e51cec52930
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/manual_of_bodily_health
        implementation: dnd2024:item_manual_of_bodily_health
        parameters:
          study:
            hours_required: 48
            maximum_elapsed_days: 6
            practice_required: true
            ability: constitution
            increase: 2
            maximum_score: 30
            after_completion: item_loses_magic
            magic_recovery:
              value: 100
              unit: year
          attunement:
            required: false
    contract:
      id: dnd2024:item_manual_of_bodily_health
      inputs:
        user: participant
        item: item_instance
        study_duration: duration
        elapsed_duration: duration
      outputs:
      - ability_scores
      - item_state
      events:
      - study.completed
      capabilities:
      - downtime
      - ability_score_changes
      - item_resources
    requires:
    - dnd2024:item_manual_of_bodily_health
    citations:
    - SRD 5.2.1 pp. 229; builtin/srd-5.2.1/magic_item/manual_of_bodily_health.md
  dnd2024:items/manual_of_quickness_of_action:
    source_sha256: 7a233d246ff6ed9035afa08f1402761d69a9074462a67619386fca3b21d663cc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/manual_of_quickness_of_action
        implementation: dnd2024:item_manual_of_quickness_of_action
        parameters:
          study:
            hours_required: 48
            maximum_elapsed_days: 6
            practice_required: true
            ability: dexterity
            increase: 2
            maximum_score: 30
            after_completion: item_loses_magic
            magic_recovery:
              value: 100
              unit: year
          attunement:
            required: false
    contract:
      id: dnd2024:item_manual_of_quickness_of_action
      inputs:
        user: participant
        item: item_instance
        study_duration: duration
        elapsed_duration: duration
      outputs:
      - ability_scores
      - item_state
      events:
      - study.completed
      capabilities:
      - downtime
      - ability_score_changes
      - item_resources
    requires:
    - dnd2024:item_manual_of_quickness_of_action
    citations:
    - SRD 5.2.1 pp. 230; builtin/srd-5.2.1/magic_item/manual_of_quickness_of_action.md
  dnd2024:items/mirror_of_life_trapping:
    source_sha256: 5a17e1a9a059a9026dd9eee112263280d39cee858a5c4012d21a0ba6eec4433f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/mirror_of_life_trapping
        implementation: dnd2024:item_mirror_of_life_trapping
        parameters:
          object:
            height_ft: 4
            width_ft: 2
            weight_lb: 50
            armor_class: 11
            hit_points: 10
            immunity:
            - poison
            - psychic
            vulnerability:
            - bludgeoning
            at_zero_hp:
            - shatter
            - destroy
          activation:
            action: magic
            command_word: activation_word
            range_ft: 5
            requires: hanging_on_vertical_surface
            duration: until_deactivated
            deactivation:
              action: magic
              command_word: activation_word
          capture:
            trigger: creature_sees_own_reflection_while_mirror_active
            exclude: activator
            range_ft: 30
            save:
              ability: charisma
              dc: 15
              advantage_if: knows_mirror_nature
              automatic_success_types:
              - construct
            failure:
              move_to: empty_cell
              include: worn_and_carried_possessions
            if_all_cells_occupied:
              release: random_prisoner
              then: capture_new_prisoner
            trigger_scope: each_reflection_exposure
          cells:
            count: 12
            capacity_each: 1
            space: infinite
            fog_visibility_ft: 10
            aging: false
            needs_food: false
            needs_water: false
            needs_sleep: false
            escape_methods:
            - planar_travel_magic
            - release
          release_placement:
            random_replacement:
              unoccupied: true
              within_sight_of: mirror
              facing: away_from_mirror
            shattered:
              release: all_prisoners
              unoccupied: true
              near: mirror
            command:
              unoccupied: true
              selection: nearest_mirror
              facing: away_from_mirror
              include: possessions
          communication:
            action: magic
            range_ft: 5
            selector:
            - creature_name
            - cell_number
            display: prisoner_image
            two_way_communication: true
          release_command:
            action: magic
            range_ft: 5
            word: second_command_word
            count: 1
          nested_extradimensional:
            trigger: mirror_placed_inside_item_created_extradimensional_space
            examples:
            - key: dnd2024:items/bag_of_holding
            - key: dnd2024:items/portable_hole
            includes_similar_items: true
            destroy: both_items
            gate:
              origin: nesting_location
              radius_ft: 10
              exclude_total_cover: true
              targets: creatures
              destination:
                plane: astral
                location: random
              one_way: true
              closes_immediately_after_transport: true
              reopenable: false
          attunement:
            required: false
    contract:
      id: dnd2024:item_mirror_of_life_trapping
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
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
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - locations
      - effects
      events:
      - item.used
      - item.damaged
      - item.placed
      capabilities:
      - inventory
      - extradimensional_spaces
      - planar_travel
      - spatial_queries
    requires:
    - dnd2024:item_mirror_of_life_trapping
    citations:
    - SRD 5.2.1 pp. 230; builtin/srd-5.2.1/magic_item/mirror_of_life_trapping.md
  dnd2024:items/necklace_of_adaptation:
    source_sha256: c012f4c0a2f6915155ea522cf55700099967158a3891f9b1866ec5b25d3694f6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/necklace_of_adaptation
        implementation: dnd2024:item_necklace_of_adaptation
        parameters:
          equipment_state: worn
          breathing: normal_in_any_environment
          saving_throws:
            advantage: true
            purpose:
            - avoid_poisoned
            - end_poisoned
          attunement:
            required: true
    contract:
      id: dnd2024:item_necklace_of_adaptation
      inputs:
        user: participant
        item: item_instance
      outputs:
      - modifiers
      events:
      - equipment.state_changed
      capabilities:
      - equipment
      - derived_statistics
    requires:
    - dnd2024:item_necklace_of_adaptation
    citations:
    - SRD 5.2.1 pp. 232; builtin/srd-5.2.1/magic_item/necklace_of_adaptation.md
  dnd2024:items/nine_lives_stealer:
    source_sha256: 6c3ac0d128f97e638523770d8a027ed54444aad5a3c8a88019ff3dc4e15fd998
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/nine_lives_stealer
        implementation: dnd2024:item_nine_lives_stealer
        parameters:
          weapon_bonus:
            attack: 2
            damage: 2
          charges:
            initial:
              dice: 1
              sides: 8
              bonus: 1
            recovery: null
            on_zero: lose_life_stealing
          life_stealing:
            requires: charges_positive
            trigger:
              event: attack_roll
              natural_d20: 20
              target_kind: creature
              target_current_hp_less_than: 100
            save:
              ability: constitution
              dc: 15
              automatic_success_types:
              - construct
              - undead
            failure:
              effect: instant_death
              charge_cost: 1
            success_charge_cost: 0
          attunement:
            required: true
    contract:
      id: dnd2024:item_nine_lives_stealer
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
      - item_resources
    requires:
    - dnd2024:item_nine_lives_stealer
    citations:
    - SRD 5.2.1 pp. 233; builtin/srd-5.2.1/magic_item/nine_lives_stealer.md
  dnd2024:items/oil_of_sharpness:
    source_sha256: 5760e32f03feee5974e1d2a3bdbc3440f8c7768b06f2660673206826f1309b77
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/oil_of_sharpness
        implementation: dnd2024:item_oil_of_sharpness
        parameters:
          application:
            duration:
              value: 1
              unit: minute
            consume_vial: true
            target_choice:
            - kind: melee_weapon
              count: 1
            - kind: ammunition
              count: 20
            eligibility:
              nonmagical: true
              damage_types_any:
              - slashing
              - piercing
            result:
              melee_weapon:
                definition:
                  key: dnd2024:items/weapon_1_2_or_3
                bonus: 3
              ammunition:
                definition:
                  key: dnd2024:items/ammunition_1_2_or_3
                bonus: 3
            duration_of_conversion: permanent
          attunement:
            required: false
    contract:
      id: dnd2024:item_oil_of_sharpness
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        objects:
          type: object_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - consumables
      - effects
    requires:
    - dnd2024:item_oil_of_sharpness
    citations:
    - SRD 5.2.1 pp. 233; builtin/srd-5.2.1/magic_item/oil_of_sharpness.md
  dnd2024:items/periapt_of_health:
    source_sha256: 4b1a3dcac66f10cd11d98b50f0666b7380fe33ddf5ae9a6cca612c8c7137f092
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/periapt_of_health
        implementation: dnd2024:item_periapt_of_health
        parameters:
          equipment_state: worn
          healing:
            action: magic
            resource:
              uses: 1
              recovery: dawn
            hit_points:
              dice: 2
              sides: 4
              bonus: 2
          saving_throws:
            advantage: true
            purpose:
            - avoid_poisoned
            - end_poisoned
          attunement:
            required: true
    contract:
      id: dnd2024:item_periapt_of_health
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_periapt_of_health
    citations:
    - SRD 5.2.1 pp. 234; builtin/srd-5.2.1/magic_item/periapt_of_health.md
  dnd2024:items/philter_of_love:
    source_sha256: 171d46c5b4aa70dc680bc1781fccac66220987daae86778d1f5802eab3813bee
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/philter_of_love
        implementation: dnd2024:item_philter_of_love
        parameters:
          consumption:
            trigger: drink
            consume_item: true
          delayed_effect:
            trigger: next_creature_seen
            window:
              value: 10
              unit: minute
            condition: charmed
            source: seen_creature
            duration:
              value: 1
              unit: hour
          attunement:
            required: false
    contract:
      id: dnd2024:item_philter_of_love
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - consumables
      - effects
    requires:
    - dnd2024:item_philter_of_love
    citations:
    - SRD 5.2.1 pp. 234; builtin/srd-5.2.1/magic_item/philter_of_love.md
  dnd2024:items/plate_armor_of_etherealness:
    source_sha256: 86b0eeb6fa06dafaa36e9de5050c7f31e98382cb052f089aff1e220e60188d4c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/plate_armor_of_etherealness
        implementation: dnd2024:item_plate_armor_of_etherealness
        parameters:
          equipment_state: worn
          activation:
            action: magic
            command_word: true
            resource:
              uses: 1
              recovery: dawn
            gain_spell_effect:
              spell:
                key: dnd2024:spells/etherealness
            end_on:
            - armor_removed
            - magic_action_repeat_command_word
          attunement:
            required: true
    contract:
      id: dnd2024:item_plate_armor_of_etherealness
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        locations:
          type: location_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_plate_armor_of_etherealness
    citations:
    - SRD 5.2.1 pp. 234; builtin/srd-5.2.1/magic_item/plate_armor_of_etherealness.md
  dnd2024:items/potion_of_clairvoyance:
    source_sha256: b86f8ae09fe9e30cb2d156ca6902d01d2a789bac833a196a3a61765244fb74d8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_clairvoyance
        implementation: dnd2024:item_potion_of_clairvoyance
        parameters:
          consumption:
            trigger: drink
            consume_item: true
            gain_spell_effect:
              spell:
                key: dnd2024:spells/clairvoyance
              concentration: false
          attunement:
            required: false
    contract:
      id: dnd2024:item_potion_of_clairvoyance
      inputs:
        user: participant
        item: item_instance
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - consumables
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_potion_of_clairvoyance
    citations:
    - SRD 5.2.1 pp. 235; builtin/srd-5.2.1/magic_item/potion_of_clairvoyance.md
  dnd2024:items/potion_of_flying:
    source_sha256: c3d47852cac55ca9205384607974ee8d3b0b9b80f8c621840bc9dc34f41ce14e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_flying
        implementation: dnd2024:item_potion_of_flying
        parameters:
          consumption:
            trigger: drink
            consume_item: true
          effect:
            duration:
              value: 1
              unit: hour
            fly_speed: walking_speed
            hover: true
            expiry_airborne: fall_unless_other_means_of_staying_aloft
          attunement:
            required: false
    contract:
      id: dnd2024:item_potion_of_flying
      inputs:
        user: participant
        item: item_instance
        locations:
          type: location_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - consumables
      - effects
    requires:
    - dnd2024:item_potion_of_flying
    citations:
    - SRD 5.2.1 pp. 235; builtin/srd-5.2.1/magic_item/potion_of_flying.md
  dnd2024:items/potion_of_growth:
    source_sha256: 8cd6130cc036736cf00f93c0bd7bc2c473c525022fa5f762c4e841360f3c9b4c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_growth
        implementation: dnd2024:item_potion_of_growth
        parameters:
          consumption:
            trigger: drink
            consume_item: true
            gain_spell_effect:
              spell:
                key: dnd2024:spells/enlarge_reduce
              mode: enlarge
              duration:
                value: 10
                unit: minute
              concentration: false
          attunement:
            required: false
    contract:
      id: dnd2024:item_potion_of_growth
      inputs:
        user: participant
        item: item_instance
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - consumables
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_potion_of_growth
    citations:
    - SRD 5.2.1 pp. 235; builtin/srd-5.2.1/magic_item/potion_of_growth.md
  dnd2024:items/potion_of_invulnerability:
    source_sha256: e89b86da89f3244343eebadb045439c89b2d3440c253e66335e089054c7eec22
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_invulnerability
        implementation: dnd2024:item_potion_of_invulnerability
        parameters:
          consumption:
            trigger: drink
            consume_item: true
          effect:
            duration:
              value: 1
              unit: minute
            resistance: all_damage
          attunement:
            required: false
    contract:
      id: dnd2024:item_potion_of_invulnerability
      inputs:
        user: participant
        item: item_instance
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - consumables
      - effects
    requires:
    - dnd2024:item_potion_of_invulnerability
    citations:
    - SRD 5.2.1 pp. 236; builtin/srd-5.2.1/magic_item/potion_of_invulnerability.md
  dnd2024:items/potion_of_poison:
    source_sha256: 77d9569518154c6abdec90126950e37b75b90a9758bf50b03eaaed736f8e51df
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_poison
        implementation: dnd2024:item_potion_of_poison
        parameters:
          illusion:
            senses:
            - appearance
            - smell
            - taste
            disguise: beneficial_potion
            identified_by:
              key: dnd2024:spells/identify
          consumption:
            trigger: drink
            consume_item: true
            damage:
              roll:
                dice: 4
                sides: 6
                bonus: 0
              type: poison
            save:
              ability: constitution
              dc: 13
            failure:
              condition: poisoned
              duration:
                value: 1
                unit: hour
          attunement:
            required: false
    contract:
      id: dnd2024:item_potion_of_poison
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - consumables
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_potion_of_poison
    citations:
    - SRD 5.2.1 pp. 236; builtin/srd-5.2.1/magic_item/potion_of_poison.md
  dnd2024:items/potion_of_vitality:
    source_sha256: 52c3484f0ef8afa49243b2bc16809347c6704fa0bc56e9ae6f482b92dd3d4b5b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_vitality
        implementation: dnd2024:item_potion_of_vitality
        parameters:
          consumption:
            trigger: drink
            consume_item: true
            remove_exhaustion: all_levels
            end_conditions:
            - poisoned
          effect:
            duration:
              value: 24
              unit: hour
            hit_point_dice_spent: maximum_healing
          attunement:
            required: false
    contract:
      id: dnd2024:item_potion_of_vitality
      inputs:
        user: participant
        item: item_instance
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - consumables
      - effects
    requires:
    - dnd2024:item_potion_of_vitality
    citations:
    - SRD 5.2.1 pp. 236; builtin/srd-5.2.1/magic_item/potion_of_vitality.md
  dnd2024:items/quarterstaff_of_the_acrobat:
    source_sha256: 262dc18a9db8e245cb2583fd33daea2c772a22f70403d174d32408b06a0e7f9e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/quarterstaff_of_the_acrobat
        implementation: dnd2024:item_quarterstaff_of_the_acrobat
        parameters:
          weapon_bonus:
            attack: 2
            damage: 2
          equipment_state: held
          light:
            kind: dim
            color: green
            radius_ft: 10
            enable:
            - bonus_action
            - after_initiative_roll
            disable: bonus_action
          change_form:
            action: bonus_action
            options:
              rod:
                length_inches: 6
              pole:
                length_ft: 10
              quarterstaff: {}
            elongation_limited_by: surrounding_space
          form_properties:
            quarterstaff:
              acrobatics: advantage
              thrown:
                normal_range_ft: 30
                long_range_ft: 120
                return_to_hand: immediately_after_ranged_attack
              deflection:
                action: reaction
                trigger: hit_by_attack
                armor_class_bonus: 5
                scope: triggering_attack
                can_turn_hit_to_miss: true
                uses: 1
                recovery:
                - short_rest
                - long_rest
            pole:
              acrobatics: advantage
          attunement:
            required: true
    contract:
      id: dnd2024:item_quarterstaff_of_the_acrobat
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
      - item_resources
    requires:
    - dnd2024:item_quarterstaff_of_the_acrobat
    citations:
    - SRD 5.2.1 pp. 237; builtin/srd-5.2.1/magic_item/quarterstaff_of_the_acrobat.md
  dnd2024:items/ring_of_elemental_command:
    source_sha256: ec8f341404fb463bdbb3740b31c1c14921315a78402e32910fb79c85b1c80f31
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_elemental_command
        implementation: dnd2024:item_ring_of_elemental_command
        parameters:
          equipment_state: worn
          variant_selection:
            method:
            - gm_choice
            - random
            options:
            - air
            - earth
            - fire
            - water
            random_distribution: uniform_four_planes
          elemental_bane:
            attacks_against_elementals: advantage
            elemental_attacks_against_user: disadvantage
          elemental_compulsion:
            action: magic
            target:
              type: elemental
              seen: true
              range_ft: 60
            save:
              ability: wisdom
              dc: 18
            failure:
              condition: charmed
              duration: until_user_next_turn_start
              user_controls_target_next_turn:
              - move
              - action
          charges:
            maximum: 5
            initial: 5
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 4
                bonus: 1
          variants:
            air:
              focus:
                language: auran
                resistance:
                - lightning
                fly_speed: walking_speed
                hover: true
              spells:
              - spell:
                  key: dnd2024:spells/chain_lightning
                cost: 3
                save_dc: 18
              - spell:
                  key: dnd2024:spells/feather_fall
                cost: 0
                save_dc: 18
              - spell:
                  key: dnd2024:spells/gust_of_wind
                cost: 2
                save_dc: 18
              - spell:
                  key: dnd2024:spells/wind_wall
                cost: 1
                save_dc: 18
            earth:
              focus:
                language: terran
                resistance:
                - acid
                ignore_difficult_terrain_materials:
                - rubble
                - rocks
                - dirt
                earth_glide:
                  materials:
                  - solid_earth
                  - rock
                  movement_cost: difficult_terrain
                  disturb_material: false
                  if_end_turn_inside: shunt_to_nearest_unoccupied_space_last_occupied
              spells:
              - spell:
                  key: dnd2024:spells/earthquake
                cost: 5
                save_dc: 18
              - spell:
                  key: dnd2024:spells/stone_shape
                cost: 2
                save_dc: 18
              - spell:
                  key: dnd2024:spells/stoneskin
                cost: 3
                save_dc: 18
              - spell:
                  key: dnd2024:spells/wall_of_stone
                cost: 3
                save_dc: 18
            fire:
              focus:
                language: ignan
                immunity:
                - fire
              spells:
              - spell:
                  key: dnd2024:spells/burning_hands
                cost: 1
                save_dc: 18
              - spell:
                  key: dnd2024:spells/fireball
                cost: 2
                save_dc: 18
              - spell:
                  key: dnd2024:spells/fire_storm
                cost: 4
                save_dc: 18
              - spell:
                  key: dnd2024:spells/wall_of_fire
                cost: 3
                save_dc: 18
            water:
              focus:
                language: aquan
                swim_speed_ft: 60
                underwater_breathing: true
              spells:
              - spell:
                  key: dnd2024:spells/create_or_destroy_water
                cost: 1
                save_dc: 18
              - spell:
                  key: dnd2024:spells/ice_storm
                cost: 2
                save_dc: 18
              - spell:
                  key: dnd2024:spells/tsunami
                cost: 5
                save_dc: 18
              - spell:
                  key: dnd2024:spells/wall_of_ice
                cost: 3
                save_dc: 18
              - spell:
                  key: dnd2024:spells/water_walk
                cost: 2
                save_dc: 18
          attunement:
            required: true
    contract:
      id: dnd2024:item_ring_of_elemental_command
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
      - spatial_queries
    requires:
    - dnd2024:item_ring_of_elemental_command
    citations:
    - SRD 5.2.1 pp. 237, 238; builtin/srd-5.2.1/magic_item/ring_of_elemental_command.md
  dnd2024:items/ring_of_free_action:
    source_sha256: 9d9c2adff7989b590568f6be289f66f99eb2eaa27c9852a0408162e0cf0fd6e5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_free_action
        implementation: dnd2024:item_ring_of_free_action
        parameters:
          equipment_state: worn
          difficult_terrain_extra_cost: 0
          magic_prevention:
            speed_reductions: all_speeds
            conditions:
            - paralyzed
            - restrained
          attunement:
            required: true
    contract:
      id: dnd2024:item_ring_of_free_action
      inputs:
        user: participant
        item: item_instance
      outputs:
      - modifiers
      events:
      - equipment.state_changed
      capabilities:
      - equipment
      - derived_statistics
    requires:
    - dnd2024:item_ring_of_free_action
    citations:
    - SRD 5.2.1 pp. 238; builtin/srd-5.2.1/magic_item/ring_of_free_action.md
  dnd2024:items/ring_of_mind_shielding:
    source_sha256: 29b23fda058a009351f18c71f620d34939b5dc17a2e265556df7cdae329e3301
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_mind_shielding
        implementation: dnd2024:item_ring_of_mind_shielding
        parameters:
          equipment_state: worn
          magic_immunities:
          - read_thoughts
          - detect_lie
          - know_alignment
          - know_creature_type
          incoming_telepathy:
            requires_consent: true
            exception: resident_ring_soul
          imperceptibility:
            enable_action: magic
            end_on:
            - magic_action_make_perceptible
            - ring_removed
            - wearer_death
          soul:
            trigger: wearer_dies
            requires: ring_has_no_soul
            effect: soul_enters_ring
            choice:
            - remain
            - depart_afterlife
            resident_telepathy:
              target: current_wearer
              wearer_cannot_block: true
            capacity: 1
          attunement:
            required: true
    contract:
      id: dnd2024:item_ring_of_mind_shielding
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_ring_of_mind_shielding
    citations:
    - SRD 5.2.1 pp. 238; builtin/srd-5.2.1/magic_item/ring_of_mind_shielding.md
  dnd2024:items/ring_of_resistance:
    source_sha256: 5106382ee61171745ae8ed21b425bb32e42a91c50d0311c26324f0318aa723bf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_resistance
        implementation: dnd2024:item_ring_of_resistance
        parameters:
          equipment_state: worn
          variant_selection:
          - gm_choice
          - d10
          variants:
          - roll: 1
            resistance: acid
            gemstone: pearl
          - roll: 2
            resistance: cold
            gemstone: tourmaline
          - roll: 3
            resistance: fire
            gemstone: garnet
          - roll: 4
            resistance: force
            gemstone: sapphire
          - roll: 5
            resistance: lightning
            gemstone: citrine
          - roll: 6
            resistance: necrotic
            gemstone: jet
          - roll: 7
            resistance: poison
            gemstone: amethyst
          - roll: 8
            resistance: psychic
            gemstone: jade
          - roll: 9
            resistance: radiant
            gemstone: topaz
          - roll: 10
            resistance: thunder
            gemstone: spinel
          attunement:
            required: false
    contract:
      id: dnd2024:item_ring_of_resistance
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - modifiers
      events:
      - equipment.state_changed
      capabilities:
      - equipment
      - derived_statistics
    requires:
    - dnd2024:item_ring_of_resistance
    citations:
    - SRD 5.2.1 pp. 238; builtin/srd-5.2.1/magic_item/ring_of_resistance.md
  dnd2024:items/ring_of_spell_turning:
    source_sha256: 334f59559e7177349c690bf921ec1072a6679a68effec7f57b76b1f12dac2d38
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_spell_turning
        implementation: dnd2024:item_ring_of_spell_turning
        parameters:
          equipment_state: worn
          saving_throws_against_spells: advantage
          successful_save:
            maximum_spell_level: 7
            spell_effect_on_user: none
            reflection:
              action: reaction
              requires:
                only_target: user
                area_of_effect: false
              target: original_caster
              save_dc: original_caster_spell_save_dc
              save_against: same_spell
          attunement:
            required: true
    contract:
      id: dnd2024:item_ring_of_spell_turning
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
    requires:
    - dnd2024:item_ring_of_spell_turning
    citations:
    - SRD 5.2.1 pp. 239; builtin/srd-5.2.1/magic_item/ring_of_spell_turning.md
  dnd2024:items/ring_of_the_ram:
    source_sha256: db60f8b3564b7f0fbcebb52ca5ca75e60020d846bf8b25b1df9bd7337bdf08ec
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_the_ram
        implementation: dnd2024:item_ring_of_the_ram
        parameters:
          equipment_state: worn
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 3
                bonus: 0
          activation:
            action: magic
            charge_choice:
              minimum: 1
              maximum: 3
            options:
              creature_attack:
                target:
                  kind: creature
                  seen: true
                  range_ft: 60
                attack:
                  kind: ranged_spell
                  bonus: 7
                on_hit:
                  damage:
                    dice_per_charge: 2
                    sides: 10
                    type: force
                  push:
                    ft_per_charge: 5
                    direction: away_from_user
              object_break:
                target:
                  kind: object
                  nonmagical: true
                  worn_or_carried: false
                  seen: true
                  range_ft: 60
                check:
                  ability: strength
                  bonus_per_charge: 5
                  dc_from: object_break_dc
          attunement:
            required: true
    contract:
      id: dnd2024:item_ring_of_the_ram
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
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
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
      - spatial_queries
    requires:
    - dnd2024:item_ring_of_the_ram
    citations:
    - SRD 5.2.1 pp. 239; builtin/srd-5.2.1/magic_item/ring_of_the_ram.md
  dnd2024:items/ring_of_water_walking:
    source_sha256: 8bb55aa47592e7f8d93c6f0cd9f1ee4626df5d5d4b7d10a3fc0c76d6d8b9283d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_water_walking
        implementation: dnd2024:item_ring_of_water_walking
        parameters:
          equipment_state: worn
          spells:
          - spell:
              key: dnd2024:spells/water_walk
            targets: self_only
          attunement:
            required: false
    contract:
      id: dnd2024:item_ring_of_water_walking
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_ring_of_water_walking
    citations:
    - SRD 5.2.1 pp. 239; builtin/srd-5.2.1/magic_item/ring_of_water_walking.md
  dnd2024:items/robe_of_scintillating_colors:
    source_sha256: c64459a2a7a52affcddae263fcff19c6e4f999208f9618f01535a502ad27fdd7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/robe_of_scintillating_colors
        implementation: dnd2024:item_robe_of_scintillating_colors
        parameters:
          equipment_state: worn
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 3
                bonus: 0
          activation:
            action: magic
            cost: 1
            duration: until_user_next_turn_end
            light:
              bright_radius_ft: 30
              additional_dim_ft: 30
            attacks_against_user:
              disadvantage_if: attacker_can_see_user
            activation_save:
              targets: creatures_in_bright_light_that_see_user
              ability: wisdom
              dc: 15
              failure:
                condition: stunned
                duration: until_effect_ends
          attunement:
            required: true
    contract:
      id: dnd2024:item_robe_of_scintillating_colors
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_robe_of_scintillating_colors
    citations:
    - SRD 5.2.1 pp. 240; builtin/srd-5.2.1/magic_item/robe_of_scintillating_colors.md
  dnd2024:items/robe_of_useful_items:
    source_sha256: fbe0a1313d4be5b81d53113033fb84045720bc4f53b62397851c38c0c4f9383f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/robe_of_useful_items
        implementation: dnd2024:item_robe_of_useful_items
        parameters:
          equipment_state: worn
          activation:
            action: magic
            operation: detach_patch
            count: 1
            result: represented_object_or_creature
            consume_patch: true
            on_last_patch: robe_becomes_nonmagical
          initial_patches:
            fixed:
            - definition:
                key: dnd2024:equipment/lantern_bullseye_10_gp
              count: 2
              filled: true
              lit: true
            - definition:
                key: dnd2024:equipment/dagger
              count: 2
            - definition:
                key: dnd2024:equipment/mirror_5_gp
              count: 2
            - definition:
                key: dnd2024:equipment/pole_5_cp
              count: 2
            - definition:
                key: dnd2024:equipment/rope_1_gp
              count: 2
              coiled: true
            - definition:
                key: dnd2024:equipment/sack_1_cp
              count: 2
            additional_count:
              dice: 4
              sides: 4
              bonus: 0
            additional_selection:
            - gm_choice
            - d100_per_patch
            table:
            - min: 1
              max: 8
              patch:
                kind: bag
                contents:
                  gp: 100
            - min: 9
              max: 15
              patch:
                kind: silver_coffer
                length_ft: 1
                width_inches: 6
                depth_inches: 6
                value_gp: 500
            - min: 16
              max: 22
              patch:
                kind: iron_door
                maximum_width_ft: 10
                maximum_height_ft: 10
                barred_side: user_choice
                placement: reachable_opening
                conform_to_opening: true
                attach_and_hinge: true
            - min: 23
              max: 30
              patch:
                kind: gems
                count: 10
                value_each_gp: 100
            - min: 31
              max: 44
              patch:
                kind: wooden_ladder
                length_ft: 24
            - min: 45
              max: 51
              patch:
                creature:
                  key: dnd2024:creatures/riding_horse
                equipment:
                  kind: riding_saddle
            - min: 52
              max: 59
              patch:
                kind: open_pit
                cube_side_ft: 10
                placement: ground
                range_ft: 10
            - min: 60
              max: 68
              patch:
                definition:
                  key: dnd2024:equipment/potion_of_healing_50_gp
                count: 4
            - min: 69
              max: 75
              patch:
                kind: rowboat
                length_ft: 12
            - min: 76
              max: 83
              patch:
                definition:
                  key: dnd2024:items/spell_scroll
                spell_choice:
                  chooser: user
                  allowed_levels:
                  - 1
                  - 2
                  - 3
            - min: 84
              max: 90
              patch:
                creature:
                  key: dnd2024:creatures/mastiff
                count: 2
            - min: 91
              max: 96
              patch:
                kind: window
                dimensions_ft:
                - 2
                - 4
                maximum_depth_ft: 2
                placement: reachable_vertical_surface
            - min: 97
              max: 100
              patch:
                kind: portable_ram
            additional_table_rolls_independent: true
          attunement:
            required: false
    contract:
      id: dnd2024:item_robe_of_useful_items
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
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
        campaign:
          type: campaign
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
      - spatial_queries
    requires:
    - dnd2024:item_robe_of_useful_items
    citations:
    - SRD 5.2.1 pp. 240, 241; builtin/srd-5.2.1/magic_item/robe_of_useful_items.md
  dnd2024:items/rod_of_lordly_might:
    source_sha256: b316b99f11668135e8aa1970d4378d734601ae0d390ba0ed236721400c8f8d60
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/rod_of_lordly_might
        implementation: dnd2024:item_rod_of_lordly_might
        parameters:
          normal_form:
            weapon:
              key: dnd2024:equipment/mace
            attack_bonus: 3
            damage_bonus: 3
            magical: true
          buttons:
            action: bonus_action
            duration: until_same_or_different_button_pressed
            repeat_same_button: normal_form
            options:
              1:
                weapon_choice:
                - key: dnd2024:equipment/longsword
                - key: dnd2024:equipment/shortsword
                magical: true
                extra_hit_damage:
                  roll:
                    dice: 2
                    sides: 6
                    bonus: 0
                  type: fire
                light:
                  bright_radius_ft: 40
                  additional_dim_ft: 40
                attack_bonus: 0
                damage_bonus: 0
              2:
                weapon:
                  key: dnd2024:equipment/battleaxe
                attack_bonus: 3
                damage_bonus: 3
                magical: true
              3:
                weapon:
                  key: dnd2024:equipment/spear
                attack_bonus: 3
                damage_bonus: 3
                magical: true
                length_ft: 6
              4:
                kind: climbing_pole
                length:
                  maximum_ft: 50
                  chooser: user
                buttons_remain_reachable: true
                anchors:
                  bottom_spikes: 1
                  top_hooks: 3
                  maximum_surface_hardness: granite
                rungs:
                  length_inches: 3
                  spacing_ft: 1
                maximum_load_lb: 4000
                revert_if:
                - overloaded
                - not_solidly_anchored
              5:
                kind: handheld_battering_ram
                checks:
                  ability: strength
                  skill: athletics
                  bonus: 10
                  purpose: break_doors_barricades_or_barriers
              6:
                form: normal
                information:
                - magnetic_north
                - approximate_depth_or_height_relative_to_ground
                north_requires: magnetic_north_exists
          drain_life:
            trigger: melee_hit_with_rod_against_creature
            optional: true
            resource:
              uses: 1
              recovery: dawn
            save:
              ability: constitution
              dc: 17
            failure:
              damage:
                roll:
                  dice: 4
                  sides: 6
                  bonus: 0
                type: necrotic
              user_healing:
                fraction_of_necrotic_damage: 0.5
          paralyze:
            trigger: melee_hit_with_rod_against_creature
            optional: true
            resource:
              uses: 1
              recovery: dawn
            save:
              ability: constitution
              dc: 17
            failure:
              condition: paralyzed
              duration:
                value: 1
                unit: minute
            repeat_save: target_turn_end
            success: end_condition
          terrify:
            requires: held
            action: magic
            resource:
              uses: 1
              recovery: dawn
            targets:
              kind: creatures
              seen: true
              range_ft: 30
              count: all
            save:
              ability: wisdom
              dc: 17
            failure:
              condition: frightened
              duration:
                value: 1
                unit: minute
            repeat_save: target_turn_end
            success: end_condition
          attunement:
            required: true
    contract:
      id: dnd2024:item_rod_of_lordly_might
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        locations:
          type: location_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
      - spatial_queries
    requires:
    - dnd2024:item_rod_of_lordly_might
    citations:
    - SRD 5.2.1 pp. 241, 242; builtin/srd-5.2.1/magic_item/rod_of_lordly_might.md
  dnd2024:items/rod_of_security:
    source_sha256: 444b59b2d813c203ced67900edbfbf10241a031635ce3dbc531db1040fd1daf9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/rod_of_security
        implementation: dnd2024:item_rod_of_security
        parameters:
          equipment_state: held
          activation:
            action: magic
            recovery:
              value: 10
              unit: day
            transport:
              self: true
              other_targets:
                maximum: 199
                willing: true
                seen: true
              destination: created_demiplane
          demiplane:
            form_choice:
              chooser: user
              kind: imagined_environment
              constraints:
              - sufficient_food_and_water
              - environment_cannot_harm_occupants
              - native_objects_cannot_leave
            food_and_water: sufficient_for_all_visitors
            environment_cannot_harm_occupants: true
            native_interactable_objects:
              exist_only_inside: true
              on_removal: disappear
            hourly_healing:
              per_visitor: as_one_hit_point_die_spent
              hit_point_die_cost: 0
            aging: false
            time_passage: normal
            maximum_stay_days:
              operation: floor_divide
              numerator: 200
              denominator: creatures_present
          return_transport:
            triggers:
            - duration_expires
            - user_magic_action_end
            targets: all_visitors
            destination: original_activation_location_per_visitor
            if_occupied: nearest_unoccupied_space
          attunement:
            required: false
    contract:
      id: dnd2024:item_rod_of_security
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        locations:
          type: location_list
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
      - spatial_queries
    requires:
    - dnd2024:item_rod_of_security
    citations:
    - SRD 5.2.1 pp. 242; builtin/srd-5.2.1/magic_item/rod_of_security.md
  dnd2024:items/scarab_of_protection:
    source_sha256: 969c3fec90af8eb751b9ce5192d821660c61573773e6d01954179fa4a9246450
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/scarab_of_protection
        implementation: dnd2024:item_scarab_of_protection
        parameters:
          equipment_state: on_person
          armor_class_bonus: 1
          saving_throws_against_spells: advantage
          charges:
            initial: 12
            maximum: 12
            recovery: null
            on_zero: destroy_item
          preservation:
            action: reaction
            trigger: failed_saving_throw
            eligible_effect:
              any_of:
              - spell_school: necromancy
              - origin_type: undead
                harmful: true
            cost: 1
            replace_result: success
          attunement:
            required: true
    contract:
      id: dnd2024:item_scarab_of_protection
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
      - item_resources
    requires:
    - dnd2024:item_scarab_of_protection
    citations:
    - SRD 5.2.1 pp. 243; builtin/srd-5.2.1/magic_item/scarab_of_protection.md
  dnd2024:items/sentinel_shield:
    source_sha256: 84c7eda2c77a760b989922fce74f5dbb0c20a8f6cb7c9a5f88ea34433f9f8e5c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/sentinel_shield
        implementation: dnd2024:item_sentinel_shield
        parameters:
          equipment_state: held
          advantage:
          - initiative
          - wisdom_perception
          attunement:
            required: false
    contract:
      id: dnd2024:item_sentinel_shield
      inputs:
        user: participant
        item: item_instance
      outputs:
      - modifiers
      events:
      - equipment.state_changed
      capabilities:
      - equipment
      - derived_statistics
    requires:
    - dnd2024:item_sentinel_shield
    citations:
    - SRD 5.2.1 pp. 243; builtin/srd-5.2.1/magic_item/sentinel_shield.md
  dnd2024:items/shield_of_the_cavalier:
    source_sha256: a1b95819f3a59cfb1eb0833ea6de5c53d1e81803551fc78bb0c9f5b8f793a934
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/shield_of_the_cavalier
        implementation: dnd2024:item_shield_of_the_cavalier
        parameters:
          equipment_state: held
          armor_class:
            bonus: 2
            stacks_with_normal_shield: true
          forceful_bash:
            trigger: attack_action
            replaces: one_attack_roll
            range_ft: 5
            attack_bonus:
              sum:
              - value_from: proficiency_bonus
              - value_from: strength_modifier
            on_hit:
              damage:
                roll:
                  dice: 2
                  sides: 6
                  bonus: 2
                add: strength_modifier
                type: force
              if_creature:
                optional_push:
                  maximum_ft: 10
                  direction: directly_away_from_user
                optional_prone:
                  maximum_size: user_size
          protective_field:
            action: reaction
            resource:
              uses: 1
              recovery: dawn
            trigger:
              target:
                any_of:
                - self
                - ally: true
                  seen: true
                  range_ft: 5
              event:
              - targeted_by_attack
              - saving_throw_against_area_effect
            area:
              shape: emanation
              radius_ft: 5
              origin: user_activation_location
              immobile: true
            duration:
              value: 1
              unit: minute
            concentration: true
            appearance:
              partially_contained_creatures_and_objects: push_to_nearest_unoccupied_space_outside
            triggering_effect: no_effect_on_contents
            boundary:
              passage_in: false
              passage_out: false
              outside_damage_to_inside: false
              inside_creature_damage_to_outside: false
          attunement:
            required: true
    contract:
      id: dnd2024:item_shield_of_the_cavalier
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
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
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
      - item_resources
      - spatial_queries
    requires:
    - dnd2024:item_shield_of_the_cavalier
    citations:
    - SRD 5.2.1 pp. 243; builtin/srd-5.2.1/magic_item/shield_of_the_cavalier.md
  dnd2024:items/spell_scroll:
    source_sha256: b2ce656f45c832f75f22e25f62cd04618fa35059413e3546b57deb33a9506cb8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/spell_scroll
        implementation: dnd2024:item_spell_scroll
        parameters:
          stored_spell:
            count: 1
            definition_type: spell
          read_and_cast:
            requires: spell_on_user_spell_list
            otherwise: unintelligible
            casting_time: normal_spell_casting_time
            material_components_required: false
            on_cast_completed: destroy_scroll
            on_cast_interrupted: preserve_scroll
            higher_than_normally_castable:
              check:
                ability: user_spellcasting_ability
                dc:
                  sum:
                  - 10
                  - value_from: spell_level
              failure:
              - erase_spell
              - no_other_effect
          level_table:
          - spell_level: 0
            rarity: common
            save_dc: 13
            attack_bonus: 5
          - spell_level: 1
            rarity: common
            save_dc: 13
            attack_bonus: 5
          - spell_level: 2
            rarity: uncommon
            save_dc: 13
            attack_bonus: 5
          - spell_level: 3
            rarity: uncommon
            save_dc: 15
            attack_bonus: 7
          - spell_level: 4
            rarity: rare
            save_dc: 15
            attack_bonus: 7
          - spell_level: 5
            rarity: rare
            save_dc: 17
            attack_bonus: 9
          - spell_level: 6
            rarity: very_rare
            save_dc: 17
            attack_bonus: 9
          - spell_level: 7
            rarity: very_rare
            save_dc: 18
            attack_bonus: 10
          - spell_level: 8
            rarity: very_rare
            save_dc: 18
            attack_bonus: 10
          - spell_level: 9
            rarity: legendary
            save_dc: 19
            attack_bonus: 11
          copy_to_spellbook:
            requires: wizard_spell
            check:
              ability: intelligence
              skill: arcana
              dc:
                sum:
                - 10
                - value_from: spell_level
            success: copy_spell
            regardless_of_result: destroy_scroll
          attunement:
            required: false
    contract:
      id: dnd2024:item_spell_scroll
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_spell_scroll
    citations:
    - SRD 5.2.1 pp. 244; builtin/srd-5.2.1/magic_item/spell_scroll.md
  dnd2024:items/staff_of_charming:
    source_sha256: 6177c693a6a2c6241e6055d700a1aad95e23b8b6e84ed02e5c1e98753d1b0bc8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_charming
        implementation: dnd2024:item_staff_of_charming
        parameters:
          equipment_state: held
          charges:
            maximum: 10
            initial: 10
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 8
                bonus: 2
          spells:
          - spell:
              key: dnd2024:spells/charm_person
            cost: 1
            save_dc: user_spell_save_dc
          - spell:
              key: dnd2024:spells/command
            cost: 1
            save_dc: user_spell_save_dc
          - spell:
              key: dnd2024:spells/comprehend_languages
            cost: 1
            save_dc: user_spell_save_dc
          reflect:
            action: reaction
            trigger: successful_save_against_enchantment_spell_targeting_only_user
            cost: 1
            target: original_caster
            cast_as: user
          resist:
            trigger: failed_save_against_enchantment_spell_targeting_only_user
            resource:
              uses: 1
              recovery: dawn
            replace_result: success
            action: none
          last_charge:
            roll:
              dice: 1
              sides: 20
              bonus: 0
            on_value: 1
            effect: destroy_item
          attunement:
            required: true
            eligibility:
              class_any:
              - bard
              - cleric
              - druid
              - sorcerer
              - warlock
              - wizard
    contract:
      id: dnd2024:item_staff_of_charming
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_staff_of_charming
    citations:
    - SRD 5.2.1 pp. 245; builtin/srd-5.2.1/magic_item/staff_of_charming.md
  dnd2024:items/staff_of_healing:
    source_sha256: 62b89b99e1b0adfa9237a456ca0df0bddf11d0db1f1372c542099a856846aa1b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_healing
        implementation: dnd2024:item_staff_of_healing
        parameters:
          equipment_state: held
          charges:
            maximum: 10
            initial: 10
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 6
                bonus: 4
          spells:
          - spell:
              key: dnd2024:spells/cure_wounds
            cost_per_spell_level: 1
            minimum_spell_level: 1
            maximum_spell_level: 4
            ability_modifier: user_spellcasting_ability_modifier
          - spell:
              key: dnd2024:spells/lesser_restoration
            cost: 2
            ability_modifier: user_spellcasting_ability_modifier
          - spell:
              key: dnd2024:spells/mass_cure_wounds
            cost: 5
            ability_modifier: user_spellcasting_ability_modifier
          last_charge:
            roll:
              dice: 1
              sides: 20
              bonus: 0
            on_value: 1
            effect: vanish_permanently
          attunement:
            required: true
            eligibility:
              class_any:
              - bard
              - cleric
              - druid
    contract:
      id: dnd2024:item_staff_of_healing
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_staff_of_healing
    citations:
    - SRD 5.2.1 pp. 245; builtin/srd-5.2.1/magic_item/staff_of_healing.md
  dnd2024:items/staff_of_swarming_insects:
    source_sha256: 1f8a2d1f20b875e048e25a3748f72f2189091a69e33205890cff50a9f4edf993
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_swarming_insects
        implementation: dnd2024:item_staff_of_swarming_insects
        parameters:
          equipment_state: held
          charges:
            maximum: 10
            initial: 10
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 6
                bonus: 4
          insect_cloud:
            action: magic
            cost: 1
            shape: emanation
            radius_ft: 30
            origin: user
            duration:
              value: 10
              unit: minute
            insects_harmless: true
            area: heavily_obscured
            vision_exception: user
            end_on: strong_wind
          spells:
          - spell:
              key: dnd2024:spells/giant_insect
            cost: 4
            save_dc: user_spell_save_dc
            attack_modifier: user_spell_attack_modifier
          - spell:
              key: dnd2024:spells/insect_plague
            cost: 5
            save_dc: user_spell_save_dc
            attack_modifier: user_spell_attack_modifier
          last_charge:
            roll:
              dice: 1
              sides: 20
              bonus: 0
            on_value: 1
            effect: destroy_item
          attunement:
            required: true
            eligibility:
              class_any:
              - bard
              - cleric
              - druid
              - sorcerer
              - warlock
              - wizard
    contract:
      id: dnd2024:item_staff_of_swarming_insects
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_staff_of_swarming_insects
    citations:
    - SRD 5.2.1 pp. 246; builtin/srd-5.2.1/magic_item/staff_of_swarming_insects.md
  dnd2024:items/staff_of_the_woodlands:
    source_sha256: b44ef1f214b6abc24872fc72654988da0a3109d15f28c81367caadc1b10b35f5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_the_woodlands
        implementation: dnd2024:item_staff_of_the_woodlands
        parameters:
          charges:
            maximum: 6
            initial: 6
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 6
                bonus: 0
          weapon:
            base:
              key: dnd2024:equipment/quarterstaff
            magical: true
            attack_bonus: 2
            damage_bonus: 2
          held:
            spell_attack_bonus: 2
          spells:
          - spell:
              key: dnd2024:spells/animal_friendship
            cost: 1
            save_dc: user_spell_save_dc
            requires: held
          - spell:
              key: dnd2024:spells/awaken
            cost: 5
            save_dc: user_spell_save_dc
            requires: held
          - spell:
              key: dnd2024:spells/barkskin
            cost: 2
            save_dc: user_spell_save_dc
            requires: held
          - spell:
              key: dnd2024:spells/locate_animals_or_plants
            cost: 2
            save_dc: user_spell_save_dc
            requires: held
          - spell:
              key: dnd2024:spells/pass_without_trace
            cost: 2
            save_dc: user_spell_save_dc
            requires: held
          - spell:
              key: dnd2024:spells/speak_with_animals
            cost: 1
            save_dc: user_spell_save_dc
            requires: held
          - spell:
              key: dnd2024:spells/speak_with_plants
            cost: 3
            save_dc: user_spell_save_dc
            requires: held
          - spell:
              key: dnd2024:spells/wall_of_thorns
            cost: 6
            save_dc: user_spell_save_dc
            requires: held
          tree_form:
            action: magic
            requires:
              plant_end_in: earth
              space: unoccupied
            cost: 1
            height_ft: 60
            trunk_diameter_ft: 5
            top_branch_radius_ft: 20
            appearance: ordinary_healthy_tree
            detect_magic:
              school: transmutation
              intensity: faint
            revert:
              action: magic
              requires: touch_tree
              occupants: fall
          last_charge:
            roll:
              dice: 1
              sides: 20
              bonus: 0
            on_value: 1
            effect: nonmagical_quarterstaff
          attunement:
            required: true
            eligibility:
              class_any:
              - druid
    contract:
      id: dnd2024:item_staff_of_the_woodlands
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_staff_of_the_woodlands
    citations:
    - SRD 5.2.1 pp. 247; builtin/srd-5.2.1/magic_item/staff_of_the_woodlands.md
  dnd2024:items/stone_of_controlling_earth_elementals:
    source_sha256: 4a4c38986d53257f61a90b308a7873482d098b400341fafb4fff9a5823bd88e3
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/stone_of_controlling_earth_elementals
        implementation: dnd2024:item_stone_of_controlling_earth_elementals
        parameters:
          weight_lb: 5
          activation:
            action: magic
            requires: stone_touching_ground
            resource:
              uses: 1
              recovery: dawn
            summon:
              creature:
                key: dnd2024:creatures/earth_elemental
              placement:
                unoccupied: true
                chooser: user
                range_ft: 30
              commands: obey
              initiative: immediately_after_user_same_count
              duration:
                value: 1
                unit: hour
              end_on:
              - death
              - user_bonus_action_dismissal
          attunement:
            required: false
    contract:
      id: dnd2024:item_stone_of_controlling_earth_elementals
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        locations:
          type: location_list
          required: false
      outputs:
      - summoned_creatures
      - item_state
      events:
      - item.used
      - turn.started
      - creature.died
      capabilities:
      - summoning
      - initiative
      - item_resources
    requires:
    - dnd2024:item_stone_of_controlling_earth_elementals
    citations:
    - SRD 5.2.1 pp. 248; builtin/srd-5.2.1/magic_item/stone_of_controlling_earth_elementals.md
  dnd2024:items/sword_of_life_stealing:
    source_sha256: 61b3f9c2dee57d4b47ba4a2430f59d245efb31d7f9147ff226eab876e4ea776a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/sword_of_life_stealing
        implementation: dnd2024:item_sword_of_life_stealing
        parameters:
          trigger:
            event: attack_roll
            natural_d20: 20
            target:
              kind: creature
              excluded_types:
              - construct
              - undead
          extra_damage:
            amount: 15
            type: necrotic
          temporary_hit_points:
            equal_to: necrotic_damage_taken_by_target
          attunement:
            required: true
    contract:
      id: dnd2024:item_sword_of_life_stealing
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
    requires:
    - dnd2024:item_sword_of_life_stealing
    citations:
    - SRD 5.2.1 pp. 248; builtin/srd-5.2.1/magic_item/sword_of_life_stealing.md
  dnd2024:items/talisman_of_pure_good:
    source_sha256: c6c811cf89e1c0bddc42b56200dfd1336f4f1ea9900cabfc8ef8cc7b5f7a556a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/talisman_of_pure_good
        implementation: dnd2024:item_talisman_of_pure_good
        parameters:
          touch_hazard:
            attunement_required: false
            target_types:
            - fiend
            - undead
            triggers:
            - touch_item
            - end_turn_holding_or_carrying_item
            damage:
              roll:
                dice: 8
                sides: 6
                bonus: 0
              type: radiant
          holy_symbol: true
          held_or_worn:
            spell_attack_bonus: 2
          charges:
            initial: 7
            maximum: 7
            recovery: null
            on_zero: destroy_item
          pure_rebuke:
            requires: held_or_worn
            action: magic
            cost: 1
            target:
              kind: creature
              count: 1
              seen: true
              on_ground: true
              range_ft: 120
            save:
              ability: dexterity
              dc: 20
              disadvantage_types:
              - fiend
              - undead
            failure:
              fall_into_fissure: true
              destroy_target: true
              remains: false
            success:
              damage:
                roll:
                  dice: 4
                  sides: 6
                  bonus: 0
                type: psychic
              fall_into_fissure: false
            after_either_result:
              fissure: closes_without_trace
          attunement:
            required: true
            eligibility:
              class_any:
              - cleric
              - paladin
    contract:
      id: dnd2024:item_talisman_of_pure_good
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_talisman_of_pure_good
    citations:
    - SRD 5.2.1 pp. 248, 249; builtin/srd-5.2.1/magic_item/talisman_of_pure_good.md
  dnd2024:items/thunderous_greatclub:
    source_sha256: 92023a10d3552770b01d8275a134873418b9e2c724cda1dfe2d2f5b5304a512e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/thunderous_greatclub
        implementation: dnd2024:item_thunderous_greatclub
        parameters:
          attuned:
            strength:
              set: 20
              only_if_unmodified_score_below: 20
          on_hit:
            creature_extra_damage:
              roll:
                dice: 1
                sides: 8
                bonus: 0
              type: thunder
            unworn_uncarried_object_extra_damage:
              roll:
                dice: 3
                sides: 8
                bonus: 0
              type: thunder
          clap_of_thunder:
            action: magic
            requires: strike_hard_surface
            audible_range_ft: 300
            shape: cone
            length_ft: 30
            creatures:
              save:
                ability: strength
                dc: 15
              failure_condition: prone
            objects:
              nonmagical: true
              worn_or_carried: false
              damage:
                roll:
                  dice: 3
                  sides: 8
                  bonus: 0
                type: thunder
          earthquake:
            action: magic
            requires: strike_ground
            resource:
              uses: 1
              recovery: dawn
            shape: circle
            radius_ft: 50
            origin: impact_point
            ground_contact_structures_damage:
              amount: 50
              type: bludgeoning
            ground_creatures:
              save:
                ability: dexterity
                dc: 20
              failure_condition: prone
              if_concentrating:
                save:
                  ability: constitution
                  dc: 20
                failure: concentration_ends
            optional_fissure:
              depth_ft: 30
              width_ft: 10
              placement: user_choice_within_area
              creatures_at_opening:
                save:
                  ability: dexterity
                  dc: 20
                failure: fall_into_fissure
                success: move_with_edge
              structures_at_opening: collapse_into_fissure
              length:
                source_specified: false
                input: chosen_fissure_location
                constraint: within_earthquake_area
          attunement:
            required: true
    contract:
      id: dnd2024:item_thunderous_greatclub
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
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
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
      - item_resources
      - spatial_queries
    requires:
    - dnd2024:item_thunderous_greatclub
    citations:
    - SRD 5.2.1 pp. 249; builtin/srd-5.2.1/magic_item/thunderous_greatclub.md
  dnd2024:items/tome_of_understanding:
    source_sha256: 138dd0c63c21c96514a0c4a2e497f634c4b6a318b6d823ce52e3071224615932
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/tome_of_understanding
        implementation: dnd2024:item_tome_of_understanding
        parameters:
          study:
            hours_required: 48
            maximum_elapsed_days: 6
            practice_required: true
            ability: wisdom
            increase: 2
            maximum_score: 30
            after_completion: item_loses_magic
            magic_recovery:
              value: 100
              unit: year
          attunement:
            required: false
    contract:
      id: dnd2024:item_tome_of_understanding
      inputs:
        user: participant
        item: item_instance
        study_duration: duration
        elapsed_duration: duration
      outputs:
      - ability_scores
      - item_state
      events:
      - study.completed
      capabilities:
      - downtime
      - ability_score_changes
      - item_resources
    requires:
    - dnd2024:item_tome_of_understanding
    citations:
    - SRD 5.2.1 pp. 250; builtin/srd-5.2.1/magic_item/tome_of_understanding.md
  dnd2024:items/vicious_weapon:
    source_sha256: 45affdb205154ab3131face2f4f075935be357b1f4bf47a66a323f045cdbb332
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/vicious_weapon
        implementation: dnd2024:item_vicious_weapon
        parameters:
          on_hit:
            target_kind: creature
            extra_damage:
              roll:
                dice: 2
                sides: 6
                bonus: 0
              type_from: weapon_normal_damage_type
          attunement:
            required: false
    contract:
      id: dnd2024:item_vicious_weapon
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
        event:
          type: event
          required: false
      outputs:
      - modifiers
      - effects
      - damage
      - item_state
      events:
      - equipment.state_changed
      - attack.resolved
      - saving_throw.resolved
      capabilities:
      - equipment
      - combat_reactions
      - effects
      - damage
    requires:
    - dnd2024:item_vicious_weapon
    citations:
    - SRD 5.2.1 pp. 250; builtin/srd-5.2.1/magic_item/vicious_weapon.md
  dnd2024:items/wand_of_enemy_detection:
    source_sha256: 21a736c57ff07966ba0bd556c5ffe6c2dc22b34e59968d97e157fedfdb072883
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_enemy_detection
        implementation: dnd2024:item_wand_of_enemy_detection
        parameters:
          equipment_state: held
          charges:
            maximum: 7
            initial: 7
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 6
                bonus: 1
          activation:
            action: magic
            cost: 1
            duration:
              value: 1
              unit: minute
            end_on: stop_holding_wand
            detection:
              select: nearest_hostile_creature
              range_ft: 60
              output:
              - direction
              distance_revealed: false
              detect_despite:
              - invisible
              - ethereal
              - disguised
              - hidden
          last_charge:
            roll:
              dice: 1
              sides: 20
              bonus: 0
            on_value: 1
            effect: destroy_item
          attunement:
            required: true
    contract:
      id: dnd2024:item_wand_of_enemy_detection
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_wand_of_enemy_detection
    citations:
    - SRD 5.2.1 pp. 250; builtin/srd-5.2.1/magic_item/wand_of_enemy_detection.md
  dnd2024:items/wand_of_lightning_bolts:
    source_sha256: 3e6fb587d156d6e70b8af766323e5e350cfd567afea6227a89f7aee55c11726c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_lightning_bolts
        implementation: dnd2024:item_wand_of_lightning_bolts
        parameters:
          equipment_state: held
          charges:
            maximum: 7
            initial: 7
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 6
                bonus: 1
          spells:
          - spell:
              key: dnd2024:spells/lightning_bolt
            save_dc: 15
            charge_choice:
              minimum: 1
              maximum: 3
            spell_level:
              sum:
              - 2
              - value_from: charges_spent
          last_charge:
            roll:
              dice: 1
              sides: 20
              bonus: 0
            on_value: 1
            effect: destroy_item
          attunement:
            required: true
            eligibility:
              spellcaster: true
    contract:
      id: dnd2024:item_wand_of_lightning_bolts
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_wand_of_lightning_bolts
    citations:
    - SRD 5.2.1 pp. 251; builtin/srd-5.2.1/magic_item/wand_of_lightning_bolts.md
  dnd2024:items/wand_of_paralysis:
    source_sha256: 3ee70540f052f533c3fa74ead9cf9e4c2285ce23fc232559fd4fbeff5e962f22
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_paralysis
        implementation: dnd2024:item_wand_of_paralysis
        parameters:
          equipment_state: held
          charges:
            maximum: 7
            initial: 7
            recovery:
              event: dawn
              amount:
                dice: 1
                sides: 6
                bonus: 1
          activation:
            action: magic
            cost: 1
            target:
              kind: creature
              seen: true
              range_ft: 60
            save:
              ability: constitution
              dc: 15
            failure:
              condition: paralyzed
              duration:
                value: 1
                unit: minute
            repeat_save: target_turn_end
            success: end_condition
          last_charge:
            roll:
              dice: 1
              sides: 20
              bonus: 0
            on_value: 1
            effect: destroy_item
          attunement:
            required: true
            eligibility:
              spellcaster: true
    contract:
      id: dnd2024:item_wand_of_paralysis
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      capabilities:
      - equipment
      - item_resources
      - effects
    requires:
    - dnd2024:item_wand_of_paralysis
    citations:
    - SRD 5.2.1 pp. 251; builtin/srd-5.2.1/magic_item/wand_of_paralysis.md
  dnd2024:items/wand_of_the_war_mage_1_2_or_3:
    source_sha256: 3eb0bf4a8786d7eb3d6b0f35daa0ae2b1c44b673230969da6c447d15e065c065
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_the_war_mage_1_2_or_3
        implementation: dnd2024:item_wand_of_the_war_mage_1_2_or_3
        parameters:
          equipment_state: held
          variants:
          - rarity: uncommon
            spell_attack_bonus: 1
          - rarity: rare
            spell_attack_bonus: 2
          - rarity: very_rare
            spell_attack_bonus: 3
          spell_attacks:
            ignore_cover:
            - half
          attunement:
            required: true
            eligibility:
              spellcaster: true
    contract:
      id: dnd2024:item_wand_of_the_war_mage_1_2_or_3
      inputs:
        user: participant
        item: item_instance
      outputs:
      - modifiers
      events:
      - equipment.state_changed
      capabilities:
      - equipment
      - derived_statistics
    requires:
    - dnd2024:item_wand_of_the_war_mage_1_2_or_3
    citations:
    - SRD 5.2.1 pp. 251; builtin/srd-5.2.1/magic_item/wand_of_the_war_mage_1_2_or_3.md
  dnd2024:items/weapon_1_2_or_3:
    source_sha256: a01a85ec56e773bb506f7956eb30ccdd491d36c4ee665f7cf8cc86e85aa72427
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/weapon_1_2_or_3
        implementation: dnd2024:item_weapon_1_2_or_3
        parameters:
          variants:
          - rarity: uncommon
            attack_bonus: 1
            damage_bonus: 1
          - rarity: rare
            attack_bonus: 2
            damage_bonus: 2
          - rarity: very_rare
            attack_bonus: 3
            damage_bonus: 3
          bonus_scope: attacks_with_this_weapon
          attunement:
            required: false
    contract:
      id: dnd2024:item_weapon_1_2_or_3
      inputs:
        user: participant
        item: item_instance
      outputs:
      - modifiers
      events:
      - equipment.state_changed
      capabilities:
      - equipment
      - derived_statistics
    requires:
    - dnd2024:item_weapon_1_2_or_3
    citations:
    - SRD 5.2.1 pp. 253; builtin/srd-5.2.1/magic_item/weapon_1_2_or_3.md
  dnd2024:items/wind_fan:
    source_sha256: 03cdbd6e09547d27e6f183816619f38cf06c324a336461012e40848ae5c7baeb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wind_fan
        implementation: dnd2024:item_wind_fan
        parameters:
          equipment_state: held
          spells:
          - spell:
              key: dnd2024:spells/gust_of_wind
            save_dc: 13
          failure:
            usage_count_reset: dawn
            first_use_failure_chance: 0
            subsequent_cumulative_increment: 0.2
            probability:
              minimum: 0
              maximum: 1
              operation: multiply
              operands:
              - value_from: previous_uses_since_dawn
              - 0.2
            on_failure:
            - no_spell
            - destroy_magic
            - tear_item_unusable
            roll_before_cast: true
          attunement:
            required: false
    contract:
      id: dnd2024:item_wind_fan
      inputs:
        user: participant
        item: item_instance
        choice:
          type: choice
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - effects
      - item_state
      events:
      - item.used
      - spell.cast_completed
      capabilities:
      - equipment
      - item_resources
      - effects
      - item_spellcasting
    requires:
    - dnd2024:item_wind_fan
    citations:
    - SRD 5.2.1 pp. 253; builtin/srd-5.2.1/magic_item/wind_fan.md
---

# Magic item contracts: assignment items_1

All 86 assigned entries have typed rule contracts. None is informational. All named source files were read in full and their SHA-256 hashes were verified. Each definition cites the official SRD pages and its bundled source file. Canonical entry references were checked against the existing definition keys.

The contracts describe source behavior; they do not implement an engine. Item headers remain the parent importer's responsibility. The typed `attunement` values repeat the source's eligibility gates. Operations on spells delegate to the referenced spell contract with normal spell rules unless an explicit item override applies. An item that grants a spell effect uses `gain_spell_effect`, which is distinct from casting a spell. Optional contract inputs are needed only for the selected operation. Symbolic selectors and conditions are named contract vocabulary; each is scoped to that item's structured parameters.

Geometry values are in feet unless a field names another unit. Geometry queries require supplied scene locations. DM choices and random outcomes are explicit: table ranges include both endpoints; percentile 00 is represented as 100. Dice records contain count, sides and a constant bonus. Resources cannot exceed their maximum on recovery. The general rules supply rounding down for fractions unless the source states otherwise. Durations use the campaign calendar for days and years.

The Bag of Holding has a 64-cubic-foot capacity, even though the source also gives approximate interior dimensions of 2 by 2 by 4 feet. Both source values are retained. The bag's nesting hazard states a Sphere, while the mirror's hazard explicitly excludes Total Cover. The two hazards therefore retain separate area constraints (SRD pp. 212, 230).

The Figurine of Wondrous Power includes all nine named variants and all three ivory goats. Golden Lions have separate recovery timers and can activate together. The Giant Fly uses its existing canonical creature entry, which owns the embedded stat block. The Obsidian Steed's disobedience includes refusal of reversion commands (SRD pp. 221–222).

Oil of Sharpness converts eligible gear without an expiry stated by the source. This contract therefore records a permanent conversion to the existing +3 weapon or ammunition definition. It does not add the old one-hour limit (SRD p. 233). The Ring of Resistance has no attunement requirement in this source (SRD p. 238). The Horn of Valhalla uses fixed spirit counts of 2, 3, 4 and 5 (SRD p. 226).

The Ring of Water Walking's printed wording is “you cast Water Walk”; its contract exposes that spell with self as its only target and otherwise uses normal casting rules. No passive water-surface movement rule is substituted (SRD p. 239). The Rod of Lordly Might's first button specifies a flaming sword and does not state a +3 bonus; the mace, battleaxe and spear forms explicitly do (SRD pp. 241–242).

The Thunderous Greatclub's concentration save applies to concentrating creatures on the ground in the earthquake area, independently of their Dexterity save result. The optional fissure has the printed width and depth; its unstated length is a supplied location choice and is not assigned a number (SRD p. 249). The Rod of Security's healing imitates spending a Hit Point Die without requiring one to be spent; its stay limit uses the count of creatures present (SRD p. 242).

No source entry is skipped. Remaining engine work belongs to the capability families in each contract and is outside this conversion research task.

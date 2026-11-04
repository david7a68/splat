---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:items/ammunition_of_slaying:
    source_sha256: bf57a161012c8143c21284e7e19caa7365e230aae89aab75ad1f30bdbb58556e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ammunition_of_slaying
        implementation: dnd2024:item_ammunition_of_slaying
        parameters:
          trigger: item.damage_dealt
          target_type:
            selection: gm_choice_or_roll
            die: 100
            rows:
            - - 1
              - 10
              - aberration
            - - 11
              - 15
              - beast
            - - 16
              - 20
              - celestial
            - - 21
              - 25
              - construct
            - - 26
              - 35
              - dragon
            - - 36
              - 45
              - elemental
            - - 46
              - 50
              - humanoid
            - - 51
              - 60
              - fey
            - - 61
              - 70
              - fiend
            - - 71
              - 75
              - giant
            - - 76
              - 80
              - monstrosity
            - - 81
              - 85
              - ooze
            - - 86
              - 90
              - plant
            - - 91
              - 100
              - undead
            columns:
            - minimum_roll
            - maximum_roll
            - creature_type
          if_target_matches: true
          save:
            ability: constitution
            dc: 17
            failure:
              damage:
                amount:
                  roll:
                    count: 6
                    sides: 10
                type: force
            success:
              damage_fraction:
              - 1
              - 2
          after_extra_damage:
            item_magic: false
          attunement_required: false
    contract:
      id: dnd2024:item_ammunition_of_slaying
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - damage.resolved
      capabilities:
      - item_effects
      - dice
      - damage
    requires:
    - dnd2024:item_ammunition_of_slaying
    citations:
    - SRD 5.2.1 pp. 209; builtin/srd-5.2.1/magic_item/ammunition_of_slaying.md
  dnd2024:items/amulet_of_the_planes:
    source_sha256: 0b41c1df782b15560681eb1ff191edc0481f78ed84098e2dbd394635ebfa518c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/amulet_of_the_planes
        implementation: dnd2024:item_amulet_of_the_planes
        parameters:
          equipped: worn
          action: magic
          destination:
            type: location
            familiar: true
            plane: other_than_current
          check:
            ability: intelligence
            skill: arcana
            dc: 15
            success:
              spell:
                key: dnd2024:spells/plane_shift
            failure:
              transport:
              - actor
              - creatures
              - objects
              radius_ft: 15
              destination_roll:
                die: 100
                rows:
                - range:
                  - 1
                  - 60
                  plane: named_plane
                  location: random
                - range:
                  - 61
                  - 70
                  plane_roll:
                    die: 6
                    values:
                    - air
                    - earth
                    - fire
                    - water
                    - feywild
                    - shadowfell
                  location: random
                - range:
                  - 71
                  - 80
                  plane_roll:
                    die: 8
                    values:
                    - arborea
                    - arcadia
                    - beastlands
                    - bytopia
                    - elysium
                    - mechanus
                    - mount_celestia
                    - ysgard
                  location: random
                - range:
                  - 81
                  - 90
                  plane_roll:
                    die: 8
                    values:
                    - abyss
                    - acheron
                    - carceri
                    - gehenna
                    - hades
                    - limbo
                    - nine_hells
                    - pandemonium
                  location: random
                - range:
                  - 91
                  - 100
                  plane: astral
                  location: random
          attunement_required: true
    contract:
      id: dnd2024:item_amulet_of_the_planes
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - planar_travel
    requires:
    - dnd2024:item_amulet_of_the_planes
    citations:
    - SRD 5.2.1 pp. 209; builtin/srd-5.2.1/magic_item/amulet_of_the_planes.md
  dnd2024:items/armor_1_2_or_3:
    source_sha256: f3a9c74a2b43728dc4e6e3f7759366dcdd8e86212d4330ede000b561e03f0dea
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/armor_1_2_or_3
        implementation: dnd2024:item_armor_1_2_or_3
        parameters:
          equipped: worn
          variants:
          - rarity: rare
            ac_bonus: 1
          - rarity: very_rare
            ac_bonus: 2
          - rarity: legendary
            ac_bonus: 3
          base_armor_categories:
          - light
          - medium
          - heavy
          attunement_required: false
    contract:
      id: dnd2024:item_armor_1_2_or_3
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.equipment_changed
      capabilities:
      - item_effects
      - item_variants
    requires:
    - dnd2024:item_armor_1_2_or_3
    citations:
    - SRD 5.2.1 pp. 210; builtin/srd-5.2.1/magic_item/armor_1_2_or_3.md
  dnd2024:items/armor_of_vulnerability:
    source_sha256: d28bf432565ca44ac76f42e88811281e38919b15e9a2f15a1676c8169d31481a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/armor_of_vulnerability
        implementation: dnd2024:item_armor_of_vulnerability
        parameters:
          equipped: worn
          resistance:
            selection: gm_choice_or_random
            options:
            - bludgeoning
            - piercing
            - slashing
          curse:
            reveal_on:
            - identify_cast_on_item
            - attunement_completed
            apply_on: attunement_completed
            persists_after_doff: true
            vulnerability:
              set_difference:
              - - bludgeoning
                - piercing
                - slashing
              - selected_resistance
            ends_on:
            - spell:
                key: dnd2024:spells/remove_curse
              target: wearer
            - magic_equivalent: remove_curse
              adjudicator: gm
          attunement_required: true
    contract:
      id: dnd2024:item_armor_of_vulnerability
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.attunement_completed
      - spell.cast_completed
      - item.equipment_changed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
      - curses
    requires:
    - dnd2024:item_armor_of_vulnerability
    citations:
    - SRD 5.2.1 pp. 211; builtin/srd-5.2.1/magic_item/armor_of_vulnerability.md
  dnd2024:items/bag_of_devouring:
    source_sha256: e12eacc904c6b3303727ef7d1409aa0b446f4310bb69396ef023fdce271ab4b9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/bag_of_devouring
        implementation: dnd2024:item_bag_of_devouring
        parameters:
          appearance:
            key: dnd2024:items/bag_of_holding
          orifice:
            inside_out: closed
            senses_contents: true
          organic_contents:
            types:
            - animal_matter
            - vegetable_matter
            wholly_inserted: devoured_permanently
          partial_living_insertion:
            pull_inside_probability:
            - 1
            - 2
          escape:
            action: action
            check:
              ability: strength
              skill: athletics
              dc: 15
          rescue:
            action: action
            reach_inside_triggers_pull: true
            check_only_if_rescuer_not_pulled: true
            check:
              ability: strength
              skill: athletics
              dc: 20
          turn_start_inside:
            devoured: true
            body_destroyed: true
          storage:
            inanimate_volume_cubic_ft: 1
            daily_ejection:
              frequency_per_day: 1
              time_choice: gm
              plane_choice: gm
              contents: all_inanimate_objects
          on_damage_kind:
            pierced_or_torn:
              destroy_item: true
              transport_contents:
                plane: astral
                location: random
          attunement_required: false
    contract:
      id: dnd2024:item_bag_of_devouring
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.contents_changed
      - item.turned_inside_out
      - turn.started
      - item.damaged
      - time.day_changed
      - item.activation_requested
      capabilities:
      - item_effects
      - planar_travel
      - inventory_containers
      - damage
    requires:
    - dnd2024:item_bag_of_devouring
    citations:
    - SRD 5.2.1 pp. 212; builtin/srd-5.2.1/magic_item/bag_of_devouring.md
  dnd2024:items/bead_of_force:
    source_sha256: 27259a7f1d5b356e47ac0d617dd1a2ecd9e6ecec6ea186c03892182f2ef3145a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/bead_of_force
        implementation: dnd2024:item_bead_of_force
        parameters:
          diameter_inches: 0.75
          weight_oz: 1
          found_quantity:
            roll:
              count: 1
              sides: 4
              add: 4
          action: magic
          throw_range_ft: 60
          consumed_on: impact
          area:
            shape: sphere
            radius_ft: 10
          save:
            ability: dexterity
            dc: 15
            failure:
              damage:
                amount:
                  roll:
                    count: 5
                    sides: 4
                type: force
            success:
              damage: 0
          barrier:
            duration:
              amount: 1
              unit: minutes
            transparent: true
            trapped:
              save: failure
              wholly_inside: true
            push_out:
              or:
              - save: success
              - partially_inside: true
              direction: away_from_center
              until: outside
            passable:
            - breathable_air
            blocks:
            - attacks
            - other_effects
            - other_matter
            movement:
              actor: enclosed_creature
              action: utilize
              maximum_distance:
                multiply:
                - actor.speed
                - 0.5
            can_be_picked_up: true
            weight_lb: 1
            weight_includes_contents: true
          attunement_required: false
    contract:
      id: dnd2024:item_bead_of_force
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - item.impact
      - time.elapsed
      capabilities:
      - item_effects
      - dice
      - effect_duration
      - spatial_targeting
      - damage
    requires:
    - dnd2024:item_bead_of_force
    citations:
    - SRD 5.2.1 pp. 212, 213; builtin/srd-5.2.1/magic_item/bead_of_force.md
  dnd2024:items/belt_of_giant_strength:
    source_sha256: 0dfb33eb5bcb0e6876cacdec493c3b4bb92d7ddc5f90efdc887c077915ac211a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/belt_of_giant_strength
        implementation: dnd2024:item_belt_of_giant_strength
        parameters:
          equipped: worn
          ability_floor:
            ability: strength
            when: unmodified_score_below_variant
          variants:
          - giant: hill
            score: 21
            rarity: rare
          - giant: frost
            score: 23
            rarity: very_rare
          - giant: stone
            score: 23
            rarity: very_rare
          - giant: fire
            score: 25
            rarity: very_rare
          - giant: cloud
            score: 27
            rarity: legendary
          - giant: storm
            score: 29
            rarity: legendary
          attunement_required: true
    contract:
      id: dnd2024:item_belt_of_giant_strength
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.equipment_changed
      - ability_score.changed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_variants
    requires:
    - dnd2024:item_belt_of_giant_strength
    citations:
    - SRD 5.2.1 pp. 213; builtin/srd-5.2.1/magic_item/belt_of_giant_strength.md
  dnd2024:items/boots_of_levitation:
    source_sha256: 3fe1d0764da498bb0d7ea3f4c6a3771a5274e50230e79f38bc75568befb93d4e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/boots_of_levitation
        implementation: dnd2024:item_boots_of_levitation
        parameters:
          equipped: worn
          spells:
          - spell:
              key: dnd2024:spells/levitate
            target: self
            usage: at_will
          attunement_required: true
    contract:
      id: dnd2024:item_boots_of_levitation
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
    requires:
    - dnd2024:item_boots_of_levitation
    citations:
    - SRD 5.2.1 pp. 213; builtin/srd-5.2.1/magic_item/boots_of_levitation.md
  dnd2024:items/boots_of_the_winterlands:
    source_sha256: d1f7e32c5938ebb45d86221a9d4e94d3f0f4f62a6173d65833dce592fd2b0b94
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/boots_of_the_winterlands
        implementation: dnd2024:item_boots_of_the_winterlands
        parameters:
          equipped: worn
          resistance:
          - cold
          temperature_protection:
            fahrenheit_maximum: 0
            lower_limit: null
            additional_protection_required: false
          ignore_difficult_terrain:
          - ice
          - snow
          attunement_required: true
    contract:
      id: dnd2024:item_boots_of_the_winterlands
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.equipment_changed
      - damage.pending
      - movement.cost_requested
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
    requires:
    - dnd2024:item_boots_of_the_winterlands
    citations:
    - SRD 5.2.1 pp. 214; builtin/srd-5.2.1/magic_item/boots_of_the_winterlands.md
  dnd2024:items/bracers_of_defense:
    source_sha256: aca63428f54b7323c3c7478adc7cff81ba0db87770108d6bc3b40b6a76b8f5b4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/bracers_of_defense
        implementation: dnd2024:item_bracers_of_defense
        parameters:
          equipped: worn
          requires_equipment_state:
            armor_worn: false
            shield_used: false
          ac_bonus: 2
          attunement_required: true
    contract:
      id: dnd2024:item_bracers_of_defense
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.equipment_changed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
    requires:
    - dnd2024:item_bracers_of_defense
    citations:
    - SRD 5.2.1 pp. 214; builtin/srd-5.2.1/magic_item/bracers_of_defense.md
  dnd2024:items/broom_of_flying:
    source_sha256: f7f6d9c39d1ca7a8e4d5a88206ef7346674f36ba888b16557795c882afca2047
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/broom_of_flying
        implementation: dnd2024:item_broom_of_flying
        parameters:
          default_mode: mundane_broom
          activate:
            action: magic
            must_stand_astride: true
            mode: hovering_ridable
          fly_speed_ft: 50
          capacity_lb: 400
          encumbered:
            over_lb: 200
            fly_speed_ft: 30
          hover_ends_on:
          - landing
          - dismounted
          send:
            action: magic
            destination:
              familiar: true
              named: true
              maximum_distance_miles: 1
            travels_alone: true
          recall:
            action: magic
            command_word: true
            maximum_distance_miles: 1
            destination: actor
          attunement_required: true
    contract:
      id: dnd2024:item_broom_of_flying
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - movement.completed
      - item.rider_changed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
    requires:
    - dnd2024:item_broom_of_flying
    citations:
    - SRD 5.2.1 pp. 214; builtin/srd-5.2.1/magic_item/broom_of_flying.md
  dnd2024:items/carpet_of_flying:
    source_sha256: 4609012840e9eca41c33bb9de58cb25cf091ca8f1e6d59865f7f80cb1213e7cf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/carpet_of_flying
        implementation: dnd2024:item_carpet_of_flying
        parameters:
          activate:
            action: magic
            command_word: true
            mode: hovering_flying
          direction_control_range_ft: 30
          variant_selection: gm_choice_or_roll
          variants:
          - roll:
            - 1
            - 20
            dimensions_ft:
            - 3
            - 5
            normal_capacity_lb: 200
            fly_speed_ft: 80
          - roll:
            - 21
            - 55
            dimensions_ft:
            - 4
            - 6
            normal_capacity_lb: 400
            fly_speed_ft: 60
          - roll:
            - 56
            - 80
            dimensions_ft:
            - 5
            - 7
            normal_capacity_lb: 600
            fly_speed_ft: 40
          - roll:
            - 81
            - 100
            dimensions_ft:
            - 6
            - 9
            normal_capacity_lb: 800
            fly_speed_ft: 30
          variant_die: 100
          maximum_capacity_multiplier: 2
          over_normal_capacity_speed_multiplier: 0.5
          attunement_required: false
    contract:
      id: dnd2024:item_carpet_of_flying
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - item.load_changed
      capabilities:
      - item_effects
      - dice
      - spatial_targeting
      - item_variants
    requires:
    - dnd2024:item_carpet_of_flying
    citations:
    - SRD 5.2.1 pp. 215; builtin/srd-5.2.1/magic_item/carpet_of_flying.md
  dnd2024:items/circlet_of_blasting:
    source_sha256: 5a1b823854e5c4af7decd3f3a308df0ffd0285dd7050139b7c5ee8b7f6730ec6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/circlet_of_blasting
        implementation: dnd2024:item_circlet_of_blasting
        parameters:
          equipped: worn
          spells:
          - spell:
              key: dnd2024:spells/scorching_ray
            attack_bonus: 5
            uses:
              maximum: 1
              recovery_event: time.dawn
          attunement_required: false
    contract:
      id: dnd2024:item_circlet_of_blasting
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - time.dawn
      capabilities:
      - item_effects
      - item_spellcasting
    requires:
    - dnd2024:item_circlet_of_blasting
    citations:
    - SRD 5.2.1 pp. 215; builtin/srd-5.2.1/magic_item/circlet_of_blasting.md
  dnd2024:items/cloak_of_elvenkind:
    source_sha256: 7d4bee7f31a59599ec9f3494ebb068b4e6b22202bca9172ef30896c93b58fa52
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/cloak_of_elvenkind
        implementation: dnd2024:item_cloak_of_elvenkind
        parameters:
          equipped: worn
          checks:
          - ability: wisdom
            skill: perception
            actor: other
            target: wearer
            mode: disadvantage
          - ability: dexterity
            skill: stealth
            actor: wearer
            mode: advantage
          attunement_required: true
    contract:
      id: dnd2024:item_cloak_of_elvenkind
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - ability_check.pending
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
    requires:
    - dnd2024:item_cloak_of_elvenkind
    citations:
    - SRD 5.2.1 pp. 215; builtin/srd-5.2.1/magic_item/cloak_of_elvenkind.md
  dnd2024:items/cloak_of_the_bat:
    source_sha256: 41250cb6912fa9c2ba38a378b7e82c59ebf095e8aa66517ae659b0e7bd06aa26
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/cloak_of_the_bat
        implementation: dnd2024:item_cloak_of_the_bat
        parameters:
          equipped: worn
          checks:
          - ability: dexterity
            skill: stealth
            mode: advantage
          flight:
            speed_ft: 40
            requires_all:
            - light_level_in:
              - dim_light
              - darkness
            - gripping_cloak_edges: true
            ends_immediately_if_requirement_lost: true
          spells:
          - spell:
              key: dnd2024:spells/polymorph
            target: self
            form:
              key: dnd2024:creatures/bat
            retain_ability_scores:
            - intelligence
            - wisdom
            - charisma
            activation_requires_light:
            - dim_light
            - darkness
            uses:
              maximum: 1
              recovery_event: time.dawn
          attunement_required: true
    contract:
      id: dnd2024:item_cloak_of_the_bat
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - ability_check.pending
      - lighting.changed
      - item.grip_changed
      - spell.cast_pending
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
    requires:
    - dnd2024:item_cloak_of_the_bat
    citations:
    - SRD 5.2.1 pp. 216; builtin/srd-5.2.1/magic_item/cloak_of_the_bat.md
  dnd2024:items/crystal_ball_of_mind_reading:
    source_sha256: 914e4d2da2d9a0b915221fd14838fecd46821d3d64a470ecc62936a95c106b03
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/crystal_ball_of_mind_reading
        implementation: dnd2024:item_crystal_ball_of_mind_reading
        parameters:
          equipped: touching
          spells:
          - spell:
              key: dnd2024:spells/scrying
            save_dc: 17
            usage: at_will
          - spell:
              key: dnd2024:spells/detect_thoughts
            save_dc: 17
            targets:
              visible_to_actor: true
              maximum_distance_from_scrying_sensor_ft: 30
            concentration: false
            ends_on: linked_scrying_ends
            usage: at_will
          attunement_required: true
    contract:
      id: dnd2024:item_crystal_ball_of_mind_reading
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - spell.ended
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
    requires:
    - dnd2024:item_crystal_ball_of_mind_reading
    citations:
    - SRD 5.2.1 pp. 216; builtin/srd-5.2.1/magic_item/crystal_ball_of_mind_reading.md
  dnd2024:items/cube_of_force:
    source_sha256: 873f1aa00faf5c7707948ee41a95651a6c441397744685d9b9d73a99d78b4e13
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/cube_of_force
        implementation: dnd2024:item_cube_of_force
        parameters:
          dimensions_inches:
          - 1
          - 1
          - 1
          activation: press_spell_face
          charges:
            initial: 10
            maximum: 10
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 6
              cap_at_maximum: true
          spells:
          - spell:
              key: dnd2024:spells/mage_armor
            charge_cost: 1
            save_dc: 17
          - spell:
              key: dnd2024:spells/shield
            charge_cost: 1
            save_dc: 17
          - spell:
              key: dnd2024:spells/tiny_hut
            charge_cost: 3
            save_dc: 17
          - spell:
              key: dnd2024:spells/private_sanctum
            charge_cost: 4
            save_dc: 17
          - spell:
              key: dnd2024:spells/resilient_sphere
            charge_cost: 4
            save_dc: 17
          - spell:
              key: dnd2024:spells/wall_of_force
            charge_cost: 5
            save_dc: 17
          attunement_required: true
    contract:
      id: dnd2024:item_cube_of_force
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - item_charges
    requires:
    - dnd2024:item_cube_of_force
    citations:
    - SRD 5.2.1 pp. 216; builtin/srd-5.2.1/magic_item/cube_of_force.md
  dnd2024:items/dancing_sword:
    source_sha256: 9fbbe578213877109d2f5f60a5f1d8cbaefd49b4958053c9b6525e9d20a87c18
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/dancing_sword
        implementation: dnd2024:item_dancing_sword
        parameters:
          activate:
            action: bonus_action
            toss: true
            fly_ft: 30
            attack:
              target_count: 1
              target_range_from_weapon_ft: 5
              attack_roll: actor.weapon_attack
              damage_modifier: actor.weapon_ability_modifier
          hover_command:
            action: bonus_action
            move_up_to_ft: 30
            destination_range_from_actor_ft: 30
            optional_attack:
              target_count: 1
              target_range_from_weapon_ft: 5
              attack_roll: actor.weapon_attack
              damage_modifier: actor.weapon_ability_modifier
          after_attack_count: 4
          return_behavior:
            to: actor_hand
            no_free_hand: drop_in_actor_space
            obstructed_path: move_closest_then_drop
          hover_ends_on:
          - grasped
          - distance_from_actor_over_30_ft
          attunement_required: true
    contract:
      id: dnd2024:item_dancing_sword
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - attack.resolved
      - item.grasped
      - movement.completed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - damage
    requires:
    - dnd2024:item_dancing_sword
    citations:
    - SRD 5.2.1 pp. 217; builtin/srd-5.2.1/magic_item/dancing_sword.md
  dnd2024:items/defender:
    source_sha256: c44975c64cdb293692d64e3e60e5c06c83b6a66659ebb7ee4231663902664db4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/defender
        implementation: dnd2024:item_defender
        parameters:
          attack_bonus: 3
          damage_bonus: 3
          allocation:
            event: first_attack_with_item_on_own_turn
            choice:
              integer_min: 0
              integer_max: 3
            ac_bonus: chosen_amount
            attack_and_damage_bonus:
              subtract:
              - 3
              - chosen_amount
            duration_end: actor.next_turn_start
            ac_requires_holding: true
          attunement_required: true
    contract:
      id: dnd2024:item_defender
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - attack.pending
      - turn.started
      - item.equipment_changed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - effect_duration
      - damage
    requires:
    - dnd2024:item_defender
    citations:
    - SRD 5.2.1 pp. 218; builtin/srd-5.2.1/magic_item/defender.md
  dnd2024:items/dragon_orb:
    source_sha256: e1222886cff2e9658ae7b2c167a5290422086d45425d4a852852aa853b040bdf
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/dragon_orb
        implementation: dnd2024:item_dragon_orb
        parameters:
          diameter_inches:
            idle: 10
            used: 20
          control:
            action: magic
            peer_into_depths: true
            save:
              ability: charisma
              dc: 15
              failure:
                condition: charmed
                source: orb
                duration_end: attunement_ends
                voluntary_end_attunement: false
                orb_spells:
                - spell:
                    key: dnd2024:spells/suggestion
                  save_dc: 18
                  usage: at_will
                  purpose:
                    choice: gm
                    category: evil_dragon_essence_objective
              success:
                control: true
                duration_end: attunement_ends
          charges:
            initial: 7
            maximum: 7
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 4
                  add: 3
              cap_at_maximum: true
          spells_require_control: true
          spells:
          - spell:
              key: dnd2024:spells/cure_wounds
            level: 9
            charge_cost: 4
          - spell:
              key: dnd2024:spells/daylight
            charge_cost: 1
          - spell:
              key: dnd2024:spells/death_ward
            charge_cost: 2
          - spell:
              key: dnd2024:spells/detect_magic
            charge_cost: 0
          - spell:
              key: dnd2024:spells/scrying
            save_dc: 18
            charge_cost: 3
          call_dragons:
            action: magic
            requires_control: true
            telepathic_range_miles: 40
            targets: chromatic_dragons
            excluded: dragon_deities
            movement: as_soon_as_possible_most_direct_route
            attitude:
              choice: gm
              hostile_permitted: true
            cooldown:
              amount: 1
              unit: hours
          object_defense:
            ac: 20
            damage_destroys_from:
            - weapon_magic_bonus: 3
            - spell:
                key: dnd2024:spells/disintegrate
            immune_to_all_other_harm: true
          attunement_required: true
    contract:
      id: dnd2024:item_dragon_orb
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - item.attunement_ended
      - spell.cast_pending
      - time.dawn
      - time.elapsed
      - item.damage_pending
      - item.attunement_completed
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - item_charges
      - effect_duration
      - item_recharge
      - damage
      - conditions
    requires:
    - dnd2024:item_dragon_orb
    citations:
    - SRD 5.2.1 pp. 218; builtin/srd-5.2.1/magic_item/dragon_orb.md
  dnd2024:items/dust_of_disappearance:
    source_sha256: 26d0cc53999e55d5fabb58606c5de346ec28856d17d5ab756842be8b7ef9dcba
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/dust_of_disappearance
        implementation: dnd2024:item_dust_of_disappearance
        parameters:
          action: utilize
          consumed_on: effect_start
          area:
            shape: emanation
            radius_ft: 10
            origin: actor
            targets:
            - actor
            - creatures
            - objects
          effect:
            condition: invisible
            duration:
              amount:
                roll:
                  count: 2
                  sides: 4
              unit: minutes
            duration_roll_shared: true
            ends_for_each_creature_immediately_after:
            - attack_roll
            - damage_dealt
            - spell_cast
          attunement_required: false
    contract:
      id: dnd2024:item_dust_of_disappearance
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - attack.rolled
      - damage.resolved
      - spell.cast_completed
      - time.elapsed
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - effect_duration
      - spatial_targeting
      - damage
      - conditions
    requires:
    - dnd2024:item_dust_of_disappearance
    citations:
    - SRD 5.2.1 pp. 219; builtin/srd-5.2.1/magic_item/dust_of_disappearance.md
  dnd2024:items/dwarven_plate:
    source_sha256: 312be87a3b1c16f78db5c5fcfa42713f30b6de09e9e12bf74fcc95722db12d36
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/dwarven_plate
        implementation: dnd2024:item_dwarven_plate
        parameters:
          equipped: worn
          ac_bonus: 2
          reaction:
            trigger: unwilling_movement_along_ground
            distance_reduction_ft:
              choice_min: 0
              choice_max: 10
              not_below_zero: true
          attunement_required: false
    contract:
      id: dnd2024:item_dwarven_plate
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - movement.forced_pending
      capabilities:
      - item_effects
    requires:
    - dnd2024:item_dwarven_plate
    citations:
    - SRD 5.2.1 pp. 219; builtin/srd-5.2.1/magic_item/dwarven_plate.md
  dnd2024:items/efreeti_bottle:
    source_sha256: f77937a26a5d275ec270c1b2c14d459a85896b08b4c9f8d215706259e26b4058
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/efreeti_bottle
        implementation: dnd2024:item_efreeti_bottle
        parameters:
          action: magic
          activation: remove_stopper
          summon:
            at: actor.turn_end
            creature:
              key: dnd2024:creatures/efreeti
            space:
              unoccupied: true
              range_ft: 30
          first_use_roll:
            die: 10
            persistent_result: true
            rows:
            - range:
              - 1
              - 1
              behavior: attack_user
              duration:
                amount: 5
                unit: rounds
              end:
              - disappear
              - bottle_nonmagical
            - range:
              - 2
              - 9
              understands: user_languages
              behavior: obey_commands
              duration:
                amount: 1
                unit: hours
              end:
              - return_to_bottle
              - new_stopper
              stopper_locked:
                amount: 24
                unit: hours
              repeat_on_openings:
              - 2
              - 3
              opening_4:
              - escape
              - disappear
              - bottle_nonmagical
            - range:
              - 10
              - 10
              understands: user_languages
              spell:
                spell:
                  key: dnd2024:spells/wish
                uses: 1
                beneficiary: user
              ends_on_any:
              - wish_granted
              - one_hour_elapsed
              end:
              - disappear
              - bottle_nonmagical
          attunement_required: false
    contract:
      id: dnd2024:item_efreeti_bottle
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - turn.ended
      - time.elapsed
      - spell.cast_completed
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - effect_duration
      - spatial_targeting
      - summoning
    requires:
    - dnd2024:item_efreeti_bottle
    citations:
    - SRD 5.2.1 pp. 220; builtin/srd-5.2.1/magic_item/efreeti_bottle.md
  dnd2024:items/elven_chain:
    source_sha256: 01536c99f71beb546a063a66c41ce6fa5e853c2d6c2d8818f79b1bd5f0ffeaeb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/elven_chain
        implementation: dnd2024:item_elven_chain
        parameters:
          equipped: worn
          ac_bonus: 1
          armor_training:
            applies_to: this_item
            granted: true
            requires_prior_training: false
          attunement_required: false
    contract:
      id: dnd2024:item_elven_chain
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.equipment_changed
      capabilities:
      - item_effects
    requires:
    - dnd2024:item_elven_chain
    citations:
    - SRD 5.2.1 pp. 220; builtin/srd-5.2.1/magic_item/elven_chain.md
  dnd2024:items/eyes_of_charming:
    source_sha256: 7dec762c66a07296f4a0d5879df41a824fdd65e8c44910253f8c054758c013b8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/eyes_of_charming
        implementation: dnd2024:item_eyes_of_charming
        parameters:
          equipped: worn
          charges:
            initial: 3
            maximum: 3
            recovery:
              event: time.dawn
              amount: all
              cap_at_maximum: true
          spells:
          - spell:
              key: dnd2024:spells/charm_person
            save_dc: 13
            charge_choice:
              min: 1
              max: available_charges
            cast_level: charges_spent
          attunement_required: true
    contract:
      id: dnd2024:item_eyes_of_charming
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
      - item_charges
    requires:
    - dnd2024:item_eyes_of_charming
    citations:
    - SRD 5.2.1 pp. 221; builtin/srd-5.2.1/magic_item/eyes_of_charming.md
  dnd2024:items/feather_token:
    source_sha256: b82b5ab5f5a5f44d77b6f1d2a205f8e398e28a82d604953dc46a8f2764fb9126
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/feather_token
        implementation: dnd2024:item_feather_token
        parameters:
          single_use: true
          selection: gm_choice_or_roll
          variant_die: 100
          variants:
          - name: anchor
            roll:
            - 1
            - 20
            rarity: uncommon
            action: magic
            target: boat_or_ship
            range: touch
            effect:
              immovable_by_any_means: true
              duration:
                amount: 24
                unit: hours
              early_end: token_touches_vessel_again
            consumed_on: effect_end
          - name: bird
            roll:
            - 21
            - 35
            rarity: rare
            action: magic
            toss_height_ft: 5
            consumed_on: activation
            summon:
              key: dnd2024:creatures/roc
            overrides:
              can_attack: false
              obeys: simple_commands
              flight:
                normal_capacity_lb: 500
                normal_speed_mph: 16
                maximum_distance_per_day_miles: 144
                rest_hours: 1
                rest_after_flight_hours: 3
                maximum_capacity_lb: 1000
                over_normal_capacity_speed_multiplier: 0.5
            ends_on:
            - daily_maximum_distance_flown
            - zero_hp
            - magic_action_dismissal
          - name: fan
            roll:
            - 36
            - 50
            rarity: uncommon
            action: magic
            requires: aboard_boat_or_ship
            toss_height_max_ft: 10
            consumed_on: activation
            floating: true
            wind: strong
            ship_count: 1
            sailing_speed_bonus_mph: 5
            duration:
              amount: 8
              unit: hours
            dismiss_action: magic
          - name: swan_boat
            roll:
            - 51
            - 65
            rarity: rare
            action: magic
            target:
              body_of_water_minimum_diameter_ft: 60
            range: touch
            consumed_on: activation
            boat:
              length_ft: 50
              width_ft: 20
              self_propelled: true
              water_speed_mph: 6
            command:
              action: magic
              requires: aboard
              options:
              - move
              - turn
              maximum_turn_degrees: 90
            duration:
              amount: 24
              unit: hours
            dismiss_action: magic
          - name: tree
            roll:
            - 66
            - 90
            rarity: uncommon
            action: magic
            requires: outdoors
            target:
              unoccupied: true
              ground: true
            range: touch
            consumed_on: activation
            create:
              type: oak_tree
              magical: false
              height_ft: 60
              trunk_diameter_ft: 5
              top_branch_radius_ft: 20
          - name: whip
            roll:
            - 91
            - 100
            rarity: rare
            action: magic
            throw_range_ft: 10
            consumed_on: activation
            floating: true
            attack:
              action: bonus_action
              type: melee_spell
              bonus: 9
              target_count: 1
              target_range_from_whip_ft: 10
              damage:
                amount:
                  roll:
                    count: 1
                    sides: 6
                    add: 5
                type: force
              movement_max_ft: 20
            duration:
              amount: 1
              unit: hours
            ends_on:
            - magic_action_dismissal
            - user_death
            - user_incapacitated
          attunement_required: false
    contract:
      id: dnd2024:item_feather_token
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - time.elapsed
      - participant.died
      - condition.changed
      - movement.completed
      - damage.resolved
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - effect_duration
      - spatial_targeting
      - summoning
      - item_variants
      - damage
    requires:
    - dnd2024:item_feather_token
    citations:
    - SRD 5.2.1 pp. 221; builtin/srd-5.2.1/magic_item/feather_token.md
  dnd2024:items/folding_boat:
    source_sha256: 973c39edf2c785181afa40f4b448865ae02a2e96b2810eb1c7eab1ae30c4aefb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/folding_boat
        implementation: dnd2024:item_folding_boat
        parameters:
          box:
            dimensions_inches:
            - 12
            - 6
            - 6
            weight_lb: 4
            floats: true
            openable_storage: true
          transformation:
            action: magic
            command_words:
              first: rowboat
              second: keelboat
              third: box
            box_requires_no_creatures_aboard: true
            objects_on_folding:
              fits_box: inside_box
              does_not_fit_box: outside_box
            objects_on_unfolding: remain_inside_vessel
            vessel_weight: normal_vessel_weight
            vessel_statistics:
              rowboat:
                ac: 11
                hp: 50
                speed_mph: 1.5
                crew: 1
                passengers: 3
                cargo_tons: 0
                damage_threshold: 0
                weight_lb: 100
              keelboat:
                ac: 15
                hp: 100
                speed_mph: 1
                crew: 1
                passengers: 6
                cargo_tons: 0.5
                damage_threshold: 10
            zero_vessel_hp: item_destroyed
            vehicle_rules:
            - key: dnd2024:rules/mounts_and_vehicles_large_vehicles
            - key: dnd2024:rules/mounts_and_vehicles_speed
            - key: dnd2024:rules/mounts_and_vehicles_crew
          attunement_required: false
    contract:
      id: dnd2024:item_folding_boat
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - item.hp_changed
      capabilities:
      - item_effects
      - inventory_containers
      - damage
    requires:
    - dnd2024:item_folding_boat
    citations:
    - SRD 5.2.1 pp. 223; builtin/srd-5.2.1/magic_item/folding_boat.md
    - SRD 5.2.1 p. 101; builtin/srd-5.2.1/source.txt, Airborne and Waterborne Vehicles table
  dnd2024:items/gem_of_brightness:
    source_sha256: 8731d1cc5b12a700c97429847e9031b9b00751692e7a946e4c6c06ef300fb4ee
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/gem_of_brightness
        implementation: dnd2024:item_gem_of_brightness
        parameters:
          equipped: held
          charges:
            initial: 50
            maximum: 50
            recovery: null
          action: magic
          command_words:
          - word: first
            charge_cost: 0
            light:
              bright_radius_ft: 30
              additional_dim_ft: 30
            ends_on:
            - bonus_action_repeat_first_word
            - another_gem_function
          - word: second
            charge_cost: 1
            target:
              visible: true
              range_ft: 60
              creature_count: 1
            save:
              ability: constitution
              dc: 15
              failure:
                condition: blinded
                duration:
                  amount: 1
                  unit: minutes
              repeat:
                event: target.turn_end
                success: end_effect
          - word: third
            charge_cost: 5
            area:
              shape: cone
              size_ft: 30
            save:
              ability: constitution
              dc: 15
              failure:
                condition: blinded
                duration:
                  amount: 1
                  unit: minutes
              repeat:
                event: target.turn_end
                success: end_effect
          at_zero_charges:
            item_magic: false
            jewel_value_gp: 50
          attunement_required: false
    contract:
      id: dnd2024:item_gem_of_brightness
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - turn.ended
      - time.elapsed
      capabilities:
      - item_effects
      - item_charges
      - effect_duration
      - spatial_targeting
      - conditions
    requires:
    - dnd2024:item_gem_of_brightness
    citations:
    - SRD 5.2.1 pp. 223; builtin/srd-5.2.1/magic_item/gem_of_brightness.md
  dnd2024:items/glamoured_studded_leather:
    source_sha256: 2d716a0a4f1d39653f88875bdfb4bd49cf33c501a2aadb43183f67130356c4c7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/glamoured_studded_leather
        implementation: dnd2024:item_glamoured_studded_leather
        parameters:
          equipped: worn
          ac_bonus: 1
          appearance_change:
            action: bonus_action
            illusion: true
            appearance:
              choice: wearer
              kinds:
              - normal_clothing
              - armor
              attributes:
              - color
              - style
              - accessories
            retains:
            - bulk
            - weight
            ends_on:
            - next_appearance_change
            - armor_doffed
          attunement_required: false
    contract:
      id: dnd2024:item_glamoured_studded_leather
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - item.equipment_changed
      capabilities:
      - item_effects
    requires:
    - dnd2024:item_glamoured_studded_leather
    citations:
    - SRD 5.2.1 pp. 223, 224; builtin/srd-5.2.1/magic_item/glamoured_studded_leather.md
  dnd2024:items/gloves_of_thievery:
    source_sha256: 45e6e2107a62dab41012d839bcf9bc27d08f1374ac2e52347637e9a119403635
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/gloves_of_thievery
        implementation: dnd2024:item_gloves_of_thievery
        parameters:
          equipped: worn
          item_imperceptible: true
          checks:
          - ability: dexterity
            skill: sleight_of_hand
            bonus: 5
          attunement_required: false
    contract:
      id: dnd2024:item_gloves_of_thievery
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.equipment_changed
      - ability_check.pending
      capabilities:
      - item_effects
    requires:
    - dnd2024:item_gloves_of_thievery
    citations:
    - SRD 5.2.1 pp. 224; builtin/srd-5.2.1/magic_item/gloves_of_thievery.md
  dnd2024:items/handy_haversack:
    source_sha256: bbe2aa5bcb2c28c5293eb015b09eb996fe945900dfe1f762d97e39390a2ba4ac
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/handy_haversack
        implementation: dnd2024:item_handy_haversack
        parameters:
          weight_lb: 5
          weight_independent_of_contents: true
          pouches:
          - name: side
            count: 2
            capacity_lb: 200
            capacity_cubic_ft: 25
          - name: central
            count: 1
            capacity_lb: 500
            capacity_cubic_ft: 64
          extradimensional: true
          retrieve:
            action_choice:
            - utilize
            - bonus_action
            requested_item_position: top
          destroy_on:
          - pouch_overload
          - pouch_pierced
          - pouch_torn
          destroyed_contents:
            non_artifacts: lost_forever
            artifacts: reappear_elsewhere
          inside_out:
            contents: spill_unharmed
            usable: false
            restore: turn_right_side_out
          breath_per_pouch:
            minutes:
              divide:
              - 10
              - number_of_breathing_creatures
          extradimensional_nesting:
            trigger: placed_inside_extradimensional_item_space
            examples:
            - key: dnd2024:items/bag_of_holding
            - key: dnd2024:items/portable_hole
            destroy:
            - this_item
            - containing_item
            gate:
              origin: insertion_location
              targets: creatures
              radius_ft: 10
              total_cover_blocks: true
              destination:
                plane: astral
                location: random
              one_way: true
              close_after_transport: true
              can_reopen: false
          attunement_required: false
    contract:
      id: dnd2024:item_handy_haversack
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.contents_changed
      - item.damaged
      - item.destroyed
      - item.turned_inside_out
      - item.activation_requested
      - time.elapsed
      capabilities:
      - item_effects
      - planar_travel
    requires:
    - dnd2024:item_handy_haversack
    citations:
    - SRD 5.2.1 pp. 224; builtin/srd-5.2.1/magic_item/handy_haversack.md
  dnd2024:items/headband_of_intellect:
    source_sha256: 60debd97a2a3e0ada6a932db3e67c1a71a445c39ac86a5950431ce4e022133d9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/headband_of_intellect
        implementation: dnd2024:item_headband_of_intellect
        parameters:
          equipped: worn
          ability_floor:
            ability: intelligence
            score: 19
            requires_unmodified_score_below: 19
          attunement_required: true
    contract:
      id: dnd2024:item_headband_of_intellect
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.equipment_changed
      - ability_score.changed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
    requires:
    - dnd2024:item_headband_of_intellect
    citations:
    - SRD 5.2.1 pp. 225; builtin/srd-5.2.1/magic_item/headband_of_intellect.md
  dnd2024:items/helm_of_telepathy:
    source_sha256: 7a369f9448d4f9700d8563d389dfe57c304244bbea1321b31f7552b6bae1facd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/helm_of_telepathy
        implementation: dnd2024:item_helm_of_telepathy
        parameters:
          equipped: worn
          telepathy_range_ft: 30
          spells:
          - spell:
              key: dnd2024:spells/detect_thoughts
            save_dc: 13
            uses:
              maximum: 1
              recovery_event: time.dawn
            usage_pool: independent_per_spell
          - spell:
              key: dnd2024:spells/suggestion
            save_dc: 13
            uses:
              maximum: 1
              recovery_event: time.dawn
            usage_pool: independent_per_spell
          attunement_required: true
    contract:
      id: dnd2024:item_helm_of_telepathy
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
      - spatial_targeting
    requires:
    - dnd2024:item_helm_of_telepathy
    citations:
    - SRD 5.2.1 pp. 226; builtin/srd-5.2.1/magic_item/helm_of_telepathy.md
  dnd2024:items/horn_of_blasting:
    source_sha256: 48ab2b864caa95d19d99b93fbbfad60d1bb1cd2afe2d5cf4db8b96f48f873820
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/horn_of_blasting
        implementation: dnd2024:item_horn_of_blasting
        parameters:
          action: magic
          activation: blow_horn
          audible_range_ft: 600
          area:
            shape: cone
            size_ft: 30
          save:
            ability: constitution
            dc: 15
            failure:
              damage:
                amount:
                  roll:
                    count: 5
                    sides: 8
                type: thunder
              condition: deafened
              condition_duration:
                amount: 1
                unit: minutes
            success:
              damage_fraction:
              - 1
              - 2
              condition: null
          objects:
            materials:
            - glass
            - crystal
            worn_or_carried: false
            damage:
              amount:
                roll:
                  count: 10
                  sides: 8
              type: thunder
          each_use_explosion:
            probability:
            - 1
            - 5
            target: user
            damage:
              amount:
                roll:
                  count: 10
                  sides: 6
              type: force
            destroy_item: true
          attunement_required: false
    contract:
      id: dnd2024:item_horn_of_blasting
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - time.elapsed
      capabilities:
      - item_effects
      - dice
      - effect_duration
      - spatial_targeting
      - damage
      - conditions
    requires:
    - dnd2024:item_horn_of_blasting
    citations:
    - SRD 5.2.1 pp. 226; builtin/srd-5.2.1/magic_item/horn_of_blasting.md
  dnd2024:items/horseshoes_of_speed:
    source_sha256: 18def4dbce651a1e8ae0a03f23ac116a4072552a43c35622880468b74d145af1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/horseshoes_of_speed
        implementation: dnd2024:item_horseshoes_of_speed
        parameters:
          set_quantity: 4
          attach:
            action: magic
            target: horse_or_similar_creature_hoof
            range: touch
            count_per_action: 1
          remove:
            action: magic
            count_per_action: 1
          benefit:
            requires_all_four_same_creature: true
            speed_bonus_ft: 30
          attunement_required: false
    contract:
      id: dnd2024:item_horseshoes_of_speed
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - item.equipment_changed
      capabilities:
      - item_effects
    requires:
    - dnd2024:item_horseshoes_of_speed
    citations:
    - SRD 5.2.1 pp. 227; builtin/srd-5.2.1/magic_item/horseshoes_of_speed.md
  dnd2024:items/ioun_stone:
    source_sha256: 6c24bd083696d4ad55e05504c66bd4ea8a725b48cb5f33b70bbc0fc78a9487df
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ioun_stone
        implementation: dnd2024:item_ioun_stone
        parameters:
          activate:
            action: magic
            toss_in_air: true
            orbit_distance_ft:
              roll:
                count: 1
                sides: 3
            maximum_simultaneously_orbiting: 3
          benefits_require_orbit: true
          orbit:
            counts_as: worn
            avoids_collisions: true
            other_creatures_cannot:
            - attack
            - snatch
          stow:
            action: utilize
            quantity: any_number_orbiting
          attunement_end:
            orbiting: fall_as_dropped
          variants:
          - name: absorption
            rarity: very_rare
            reaction:
              trigger: visible_creature_casts_spell
              max_spell_level: 4
              effect: cancel_spell
              resources_wasted: true
              capacity_spell_levels: 20
              cumulative_levels_at_capacity:
              - burn_out
              - nonmagical
              color_after: dull_gray
          - name: greater_absorption
            rarity: legendary
            reaction:
              trigger: visible_creature_casts_spell
              max_spell_level: 8
              effect: cancel_spell
              resources_wasted: true
              capacity_spell_levels: 20
              cumulative_levels_at_capacity:
              - burn_out
              - nonmagical
              color_after: dull_gray
          - name: agility
            rarity: very_rare
            ability: dexterity
            increase: 2
            maximum: 20
          - name: fortitude
            rarity: very_rare
            ability: constitution
            increase: 2
            maximum: 20
          - name: insight
            rarity: very_rare
            ability: wisdom
            increase: 2
            maximum: 20
          - name: intellect
            rarity: very_rare
            ability: intelligence
            increase: 2
            maximum: 20
          - name: leadership
            rarity: very_rare
            ability: charisma
            increase: 2
            maximum: 20
          - name: strength
            rarity: very_rare
            ability: strength
            increase: 2
            maximum: 20
          - name: awareness
            rarity: rare
            advantage:
            - initiative
            - wisdom_perception
          - name: mastery
            rarity: legendary
            proficiency_bonus_increase: 1
          - name: protection
            rarity: rare
            ac_bonus: 1
          - name: regeneration
            rarity: legendary
            healing:
              hp: 15
              interval:
                amount: 1
                unit: hours
              requires_hp_at_least: 1
          - name: reserve
            rarity: rare
            spell_storage:
              capacity_spell_levels: 4
              initial_spell_levels:
                roll:
                  count: 1
                  sides: 4
              initial_spells_choice: gm
              store:
                caster: any_creature
                touch_required_during_cast: true
                spell_level_min: 1
                spell_level_max: 4
                occupancy: cast_slot_level
                normal_spell_effect: false
                insufficient_capacity: expend_without_effect
              release:
                cast_as: user
                copy_original_caster:
                - slot_level
                - save_dc
                - spell_attack_bonus
                - spellcasting_ability
                remove_spell_from_storage: true
                free_capacity: true
          - name: sustenance
            rarity: rare
            needs_food: false
            needs_drink: false
          absorption_remaining_budget_policy:
            adjudicator: gm
            scope: spell_level_exceeds_remaining_lifetime_budget
            options:
            - reject_absorption
            - absorb_and_burn_out
            source_unspecified: true
          absorption_burnout_threshold:
            comparison: greater_than_or_equal
            levels: 20
          attunement_required: true
    contract:
      id: dnd2024:item_ioun_stone
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - item.attunement_ended
      - spell.cast_pending
      - time.elapsed
      - item.attunement_completed
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - inventory_containers
      - item_variants
      - healing
    requires:
    - dnd2024:item_ioun_stone
    citations:
    - SRD 5.2.1 pp. 227, 228; builtin/srd-5.2.1/magic_item/ioun_stone.md
  dnd2024:items/javelin_of_lightning:
    source_sha256: 37f98a12d3f87255ef089121bebb3736bca3716bb3c9d285dbaf6d6b6d002894
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/javelin_of_lightning
        implementation: dnd2024:item_javelin_of_lightning
        parameters:
          on_hit_damage_type_choice:
          - piercing
          - lightning
          lightning_bolt:
            trigger: throw_at_target
            max_target_range_ft: 120
            replaces: ranged_attack_roll
            area:
              shape: line
              width_ft: 5
              from: user
              to: target
            targets:
            - target
            - all_other_creatures_in_line
            exclude:
            - user
            save:
              ability: dexterity
              dc: 13
              failure:
                damage:
                  amount:
                    roll:
                      count: 4
                      sides: 6
                  type: lightning
              success:
                damage_fraction:
                - 1
                - 2
            after_damage: return_weapon_to_hand
            uses:
              maximum: 1
              recovery_event: time.dawn
          attunement_required: false
    contract:
      id: dnd2024:item_javelin_of_lightning
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - attack.hit
      - attack.pending
      - time.dawn
      capabilities:
      - item_effects
      - dice
      - spatial_targeting
      - damage
    requires:
    - dnd2024:item_javelin_of_lightning
    citations:
    - SRD 5.2.1 pp. 228; builtin/srd-5.2.1/magic_item/javelin_of_lightning.md
  dnd2024:items/mace_of_disruption:
    source_sha256: 236d530e21d219a2cef21be82ff7c57f6dff43a6308777c4e30355ca3b6cfdf6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/mace_of_disruption
        implementation: dnd2024:item_mace_of_disruption
        parameters:
          on_hit:
            target_types:
            - fiend
            - undead
            damage:
              amount:
                roll:
                  count: 2
                  sides: 6
              type: radiant
            after_damage_hp_at_most: 25
            save:
              ability: wisdom
              dc: 15
              failure:
                destroy_target: true
              success:
                condition: frightened
                duration_end: user.next_turn_end
          while_held:
            light:
              bright_radius_ft: 20
              additional_dim_ft: 20
          attunement_required: true
    contract:
      id: dnd2024:item_mace_of_disruption
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - attack.hit
      - item.equipment_changed
      - turn.ended
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - effect_duration
      - damage
      - conditions
    requires:
    - dnd2024:item_mace_of_disruption
    citations:
    - SRD 5.2.1 pp. 229; builtin/srd-5.2.1/magic_item/mace_of_disruption.md
  dnd2024:items/mantle_of_spell_resistance:
    source_sha256: 8e57044dacd4349cd6ded42af9bc2567f6bfcb6382db2393ed1f9690d1e55b1d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/mantle_of_spell_resistance
        implementation: dnd2024:item_mantle_of_spell_resistance
        parameters:
          equipped: worn
          saving_throw_advantage:
            against: spells
          attunement_required: true
    contract:
      id: dnd2024:item_mantle_of_spell_resistance
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - saving_throw.pending
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
    requires:
    - dnd2024:item_mantle_of_spell_resistance
    citations:
    - SRD 5.2.1 pp. 229; builtin/srd-5.2.1/magic_item/mantle_of_spell_resistance.md
  dnd2024:items/manual_of_golems:
    source_sha256: c61b572ec2bf14df18551b729b6da2370bee0a4236194b6d0babbe5e0facf391
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/manual_of_golems
        implementation: dnd2024:item_manual_of_golems
        parameters:
          eligibility:
            spellcaster: true
            level_5_spell_slots_at_least: 2
          ineligible_read_attempt:
            damage:
              amount:
                roll:
                  count: 6
                  sides: 6
              type: psychic
          type_selection: gm_choice_or_roll
          type_die: 20
          variants:
          - roll:
            - 1
            - 5
            creature:
              key: dnd2024:creatures/clay_golem
            work_days: 30
            cost_gp: 65000
          - roll:
            - 6
            - 17
            creature:
              key: dnd2024:creatures/flesh_golem
            work_days: 60
            cost_gp: 50000
          - roll:
            - 18
            - 18
            creature:
              key: dnd2024:creatures/iron_golem
            work_days: 120
            cost_gp: 100000
          - roll:
            - 19
            - 20
            creature:
              key: dnd2024:creatures/stone_golem
            work_days: 90
            cost_gp: 80000
          crafting:
            manual_at_hand: true
            uninterrupted: true
            maximum_daily_rest_hours: 8
            supplies_cost_required: true
            completion: consume_book_in_flames
            animate: sprinkle_book_ashes_on_golem
            control: creator
            understands_and_obeys_commands: true
          attunement_required: false
    contract:
      id: dnd2024:item_manual_of_golems
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.read_requested
      - crafting.progressed
      - item.activation_requested
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - item_variants
      - damage
    requires:
    - dnd2024:item_manual_of_golems
    citations:
    - SRD 5.2.1 pp. 229, 230; builtin/srd-5.2.1/magic_item/manual_of_golems.md
  dnd2024:items/medallion_of_thoughts:
    source_sha256: 6f0989535453a4aa4bad6f926c2144274b1a8e6028d36570bf3abccec20635cb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/medallion_of_thoughts
        implementation: dnd2024:item_medallion_of_thoughts
        parameters:
          equipped: worn
          charges:
            initial: 5
            maximum: 5
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 4
              cap_at_maximum: true
          spells:
          - spell:
              key: dnd2024:spells/detect_thoughts
            save_dc: 13
            charge_cost: 1
          attunement_required: true
    contract:
      id: dnd2024:item_medallion_of_thoughts
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - item_charges
    requires:
    - dnd2024:item_medallion_of_thoughts
    citations:
    - SRD 5.2.1 pp. 230; builtin/srd-5.2.1/magic_item/medallion_of_thoughts.md
  dnd2024:items/mysterious_deck:
    source_sha256: 609eda2f7d2cc18f4fdb23148f3bebc7d71990902f3a9980cce422c67eaad067
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/mysterious_deck
        implementation: dnd2024:item_mysterious_deck
        parameters:
          deck_size_roll:
            die: 100
            rows:
            - range:
              - 1
              - 75
              size: 13
            - range:
              - 76
              - 100
              size: 22
          draw_procedure:
            declare_count_before_draw: true
            draw_randomly: true
            excess_declared_draws: no_effect
            effect_timing: immediate_on_draw
            maximum_interval:
              amount: 1
              unit: hours
            missed_remaining_draws: fly_out_and_all_resolve_simultaneously
            drawn_card_disappears: true
            return_to_deck_except:
            - fool
            - jester
            removed_card_rolled_again: reroll
          tables:
            die: 100
            deck_22:
            - range:
              - 1
              - 5
              card: balance
            - range:
              - 6
              - 10
              card: comet
            - range:
              - 11
              - 14
              card: donjon
            - range:
              - 15
              - 18
              card: euryale
            - range:
              - 19
              - 23
              card: fates
            - range:
              - 24
              - 27
              card: flames
            - range:
              - 28
              - 31
              card: fool
            - range:
              - 32
              - 36
              card: gem
            - range:
              - 37
              - 41
              card: jester
            - range:
              - 42
              - 46
              card: key
            - range:
              - 47
              - 51
              card: knight
            - range:
              - 52
              - 56
              card: moon
            - range:
              - 57
              - 60
              card: puzzle
            - range:
              - 61
              - 64
              card: rogue
            - range:
              - 65
              - 68
              card: ruin
            - range:
              - 69
              - 73
              card: sage
            - range:
              - 74
              - 77
              card: skull
            - range:
              - 78
              - 82
              card: star
            - range:
              - 83
              - 87
              card: sun
            - range:
              - 88
              - 91
              card: talons
            - range:
              - 92
              - 96
              card: throne
            - range:
              - 97
              - 100
              card: void
            deck_13:
            - range:
              - 1
              - 8
              card: euryale
            - range:
              - 9
              - 16
              card: flames
            - range:
              - 17
              - 24
              card: jester
            - range:
              - 25
              - 32
              card: key
            - range:
              - 33
              - 40
              card: knight
            - range:
              - 41
              - 48
              card: moon
            - range:
              - 49
              - 56
              card: rogue
            - range:
              - 57
              - 64
              card: ruin
            - range:
              - 65
              - 72
              card: skull
            - range:
              - 73
              - 80
              card: star
            - range:
              - 81
              - 88
              card: sun
            - range:
              - 89
              - 96
              card: throne
            - range:
              - 97
              - 100
              card: void
          cards:
            balance:
              optional: true
              increase:
                ability_choice: true
                amount: 2
                maximum: 22
              required_decrease:
                different_ability: true
                amount: 2
                pre_score_minimum: 6
            comet:
              trigger: next_combat_with_hostile_creatures
              choose_foe_at: initiative_roll
              foe_choice: one_hostile_combatant
              success_trigger: user_reduces_chosen_foe_to_zero_hp_in_that_combat
              success:
                advantage:
                - death_saving_throws
                duration:
                  amount: 1
                  unit: years
              no_effect_if:
              - foe_not_chosen
              - another_creature_reduces_foe_to_zero_hp
            donjon:
              teleport: extradimensional_sphere
              state: suspended_animation
              inventory: accompanies_user_except_artifacts
              artifacts: remain_original_space
              end: found_and_removed_from_sphere
              cannot_be_located_by: divination_magic
              location_reveal_exception:
                spell:
                  key: dnd2024:spells/wish
              stop_drawing: true
            euryale:
              curse: true
              saving_throw_bonus: -2
              ends_only_on:
              - god_intervention
              - fates_card
            fates:
              resource_uses: 1
              expiry: user_death
              activation: any_time_including_draw
              effect:
                event_choice: user
                operation: avoid_or_erase_as_never_happened
            flames:
              enemy:
                type: powerful_devil
                choice: gm
              behavior:
              - seek_user_ruin
              - torment_user
              - eventually_attempt_slay_user
              ends_on:
              - user_death
              - devil_death
            fool:
              disadvantage:
              - d20_tests
              duration:
                amount: 72
                unit: hours
              additional_draw:
                count: 1
                mandatory: true
                counts_against_declared: false
            gem:
              create_at: user_feet
              selection:
                options:
                - kind: jewelry
                  count: 25
                  value_each_gp: 2000
                - kind: gem
                  count: 50
                  value_each_gp: 1000
            jester:
              choice:
              - advantage:
                - d20_tests
                duration:
                  amount: 72
                  unit: hours
              - additional_draw:
                  count: 2
                  counts_against_declared: false
            key:
              create_at: user_person
              item_choice: gm
              constraints:
                magic_weapon: true
                rarity_minimum: rare
                user_proficient: true
            knight:
              summon:
                key: dnd2024:creatures/knight
              stat_block_override:
                choice: gm
                type: creature_definition
              space:
                unoccupied: true
                range_ft: 30
                choice: user
              alignment: user_alignment
              loyal_service_until: death
              name_backstory_choice:
              - user
              - gm
            moon:
              spell_grant:
                spell:
                  key: dnd2024:spells/wish
                uses:
                  roll:
                    count: 1
                    sides: 3
            puzzle:
              ability_choice:
              - intelligence
              - wisdom
              permanent_decrease:
                roll:
                  count: 1
                  sides: 4
                  add: 1
              minimum_score: 1
              additional_draw:
                count: 1
                optional: true
                counts_against_declared: false
            rogue:
              npc_choice: gm
              attitude: hostile
              identity_secret_until_revealed: true
              ends_only_on:
              - spell:
                  key: dnd2024:spells/wish
              - divine_intervention
            ruin:
              wealth_scope: all_carried_or_owned
              exclude:
              - magic_items
              portable_property: vanish
              business_buildings_land: ownership_lost
              reality_change_constraint: minimum_necessary
              ownership_documents: vanish
            sage:
              uses: 1
              expiry:
                amount: 1
                unit: years
              activation: meditate
              question_choice: user
              answer:
                delivery: mental
                truthful: true
            skull:
              summon:
                key: dnd2024:creatures/avatar_of_death
              placement: nearest_unoccupied_space_to_user
              attack_targets: summoner_only
              despawn_on:
              - avatar_zero_hp
              - summoner_death
              ally_damage_trigger:
                summon_another_avatar: true
                summoner: damaging_ally
                placement: nearest_unoccupied_space_to_damaging_ally
                attack_target: damaging_ally
              maximum_per_creature_per_draw: 1
              slain_by_avatar:
                can_restore_to_life: false
            star:
              ability_choice: true
              increase: 2
              maximum: 24
            sun:
              create_magic_item:
                choice: gm
                location: user_person
              temporary_hp:
                amount: 10
                event: time.dawn
                ends: user_death
            talons:
              items_scope: worn_or_carried_magic_items
              non_artifacts: disintegrate
              artifacts: vanish
            throne:
              skill_choice:
              - history
              - insight
              - intimidation
              - persuasion
              grant:
              - proficiency
              - expertise
              small_keep:
                rightful_ownership: true
                location: gm_choice
                inhabited_by:
                  monster_count_min: 1
                  choice: gm
                claim_requires: clear_monsters
            void:
              soul:
                contained_in: object
                location_choice: gm
                guardians:
                  powerful: true
                  minimum_count: 1
              body:
                inert: true
                ages: false
                requires:
                  food: false
                  air: false
                  water: false
              wish:
                return_soul: false
                reveal_object_location: true
              stop_drawing: true
          avatar_dynamic_statistics:
            definition:
              key: dnd2024:creatures/avatar_of_death
            hp_maximum:
              floor:
                multiply:
                - summoner.hp_maximum
                - 0.5
            proficiency_bonus: summoner.proficiency_bonus
            languages: summoner.languages
            multiattack_count:
              ceil:
                multiply:
                - summoner.proficiency_bonus
                - 0.5
            size: medium
            creature_type: undead
            alignment: neutral_evil
            ac: 20
            initiative_bonus: 3
            initiative_score: 13
            speed_ft: 60
            fly_speed_ft: 60
            hover: true
            abilities:
              strength: 16
              dexterity: 16
              constitution: 16
              intelligence: 16
              wisdom: 16
              charisma: 16
            saving_throw_bonuses:
              strength: 3
              dexterity: 3
              constitution: 3
              intelligence: 3
              wisdom: 3
              charisma: 3
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
            truesight_ft: 60
            passive_perception: 13
            challenge_rating: null
            xp: 0
            incorporeal_movement:
              through:
              - creatures
              - objects
              cost: difficult_terrain
              end_turn_inside_object:
                damage:
                  amount:
                    roll:
                      count: 1
                      sides: 10
                  type: force
                  average: 5
            reaping_scythe:
              type: melee
              automatic_hit: true
              reach_ft: 5
              damage:
              - type: slashing
                amount:
                  roll:
                    count: 1
                    sides: 8
                    add: 3
                average: 7
              - type: necrotic
                amount:
                  roll:
                    count: 1
                    sides: 8
                average: 4
          attunement_required: false
    contract:
      id: dnd2024:item_mysterious_deck
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.card_drawn
      - time.elapsed
      - initiative.rolled
      - hp.changed
      - participant.died
      - damage.resolved
      - time.dawn
      - item.activation_requested
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - effect_duration
      - curses
      - spatial_targeting
      - summoning
      - damage
      - conditions
    requires:
    - dnd2024:item_mysterious_deck
    citations:
    - SRD 5.2.1 pp. 231, 232; builtin/srd-5.2.1/magic_item/mysterious_deck.md
  dnd2024:items/necklace_of_prayer_beads:
    source_sha256: 249f980b234b5fd525099484b9268b371846ca48e695a16e8be41b9143d30e21
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/necklace_of_prayer_beads
        implementation: dnd2024:item_necklace_of_prayer_beads
        parameters:
          equipped: worn
          attunement_classes:
          - cleric
          - druid
          - paladin
          magic_bead_count:
            roll:
              count: 1
              sides: 4
              add: 2
          removed_magic_bead: nonmagical
          bead_type_selection: gm_choice_or_roll
          duplicate_types_allowed: true
          bead_die: 20
          beads:
          - roll:
            - 1
            - 6
            name: blessing
            cast:
              spell:
                key: dnd2024:spells/bless
              action: bonus_action
              save_dc: user.spell_save_dc
              level: 1
            uses:
              maximum: 1
              recovery_event: time.dawn
            usage_pool: per_bead
          - roll:
            - 7
            - 12
            name: curing
            cast:
              spell:
                key: dnd2024:spells/cure_wounds
              action: bonus_action
              save_dc: user.spell_save_dc
              level: 2
            uses:
              maximum: 1
              recovery_event: time.dawn
            usage_pool: per_bead
          - roll:
            - 13
            - 16
            name: favor
            cast:
              spell:
                key: dnd2024:spells/greater_restoration
              action: bonus_action
              save_dc: user.spell_save_dc
              level: 5
            uses:
              maximum: 1
              recovery_event: time.dawn
            usage_pool: per_bead
          - roll:
            - 17
            - 18
            name: smiting
            cast:
              spell:
                key: dnd2024:spells/shining_smite
              action: bonus_action
              save_dc: user.spell_save_dc
              level: 2
            uses:
              maximum: 1
              recovery_event: time.dawn
            usage_pool: per_bead
          - roll:
            - 19
            - 19
            name: summons
            cast:
              spell:
                key: dnd2024:spells/guardian_of_faith
              action: bonus_action
              save_dc: user.spell_save_dc
              level: 4
            uses:
              maximum: 1
              recovery_event: time.dawn
            usage_pool: per_bead
          - roll:
            - 20
            - 20
            name: wind_walking
            cast:
              spell:
                key: dnd2024:spells/wind_walk
              action: bonus_action
              save_dc: user.spell_save_dc
              level: 6
            uses:
              maximum: 1
              recovery_event: time.dawn
            usage_pool: per_bead
          attunement_required: true
    contract:
      id: dnd2024:item_necklace_of_prayer_beads
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - item.contents_changed
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - summoning
    requires:
    - dnd2024:item_necklace_of_prayer_beads
    citations:
    - SRD 5.2.1 pp. 233; builtin/srd-5.2.1/magic_item/necklace_of_prayer_beads.md
  dnd2024:items/oil_of_etherealness:
    source_sha256: 08f4947b9a6e9ee3b3f41c973acfbdab9e9047f65a1c289685f81884974c0a02
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/oil_of_etherealness
        implementation: dnd2024:item_oil_of_etherealness
        parameters:
          application_duration:
            amount: 10
            unit: minutes
          vials_by_target_size:
            tiny: 1
            small: 1
            medium: 1
            large: 2
            huge: 3
            gargantuan: 4
          covers:
          - creature
          - worn_equipment
          - carried_equipment
          consume_applied_vials: true
          effect:
            spell:
              key: dnd2024:spells/etherealness
            duration:
              amount: 1
              unit: hours
          attunement_required: false
    contract:
      id: dnd2024:item_oil_of_etherealness
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.application_completed
      capabilities:
      - item_effects
      - item_spellcasting
      - effect_duration
    requires:
    - dnd2024:item_oil_of_etherealness
    citations:
    - SRD 5.2.1 pp. 233; builtin/srd-5.2.1/magic_item/oil_of_etherealness.md
  dnd2024:items/pearl_of_power:
    source_sha256: e43f83708f1882568447f1fe3499867e83469dda922303e7e494007eacdf4e9f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/pearl_of_power
        implementation: dnd2024:item_pearl_of_power
        parameters:
          equipped: on_person
          attunement_requires: spellcaster
          action: magic
          restore_slot:
            count: 1
            expended_only: true
            level_min: 1
            level_max: 3
            choice: user
          uses:
            maximum: 1
            recovery_event: time.dawn
          attunement_required: true
    contract:
      id: dnd2024:item_pearl_of_power
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
    requires:
    - dnd2024:item_pearl_of_power
    citations:
    - SRD 5.2.1 pp. 234; builtin/srd-5.2.1/magic_item/pearl_of_power.md
  dnd2024:items/periapt_of_wound_closure:
    source_sha256: 791e208bf0c5e2930d823065417ac1d22a72f90af74203ecf7849ea88143342d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/periapt_of_wound_closure
        implementation: dnd2024:item_periapt_of_wound_closure
        parameters:
          equipped: worn
          death_save:
            optional: true
            roll_max: 9
            replace_roll: 10
            result: success
          hit_point_die_healing:
            multiplier: 2
            scope: hit_points_restored_by_die
          attunement_required: true
    contract:
      id: dnd2024:item_periapt_of_wound_closure
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - death_save.rolled
      - hit_point_die.healing_resolved
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - healing
    requires:
    - dnd2024:item_periapt_of_wound_closure
    citations:
    - SRD 5.2.1 pp. 234; builtin/srd-5.2.1/magic_item/periapt_of_wound_closure.md
  dnd2024:items/pipes_of_the_sewers:
    source_sha256: c2c39ed16525f69270287fe74bc5a7399c6e9ccc92ef15d93a43560add441a20
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/pipes_of_the_sewers
        implementation: dnd2024:item_pipes_of_the_sewers
        parameters:
          equipped: on_person
          rats:
            types:
            - ordinary_rat
            - giant_rat
            attitude: indifferent
            attacks_user_only_if:
            - user_threatens
            - user_harms
          charges:
            initial: 3
            maximum: 3
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 3
              cap_at_maximum: true
          play:
            action: magic
            maintain_each_round: true
          call:
            action: bonus_action
            requires: playing_pipes
            charges_min: 1
            charges_max: 3
            swarm_per_charge: 1
            creature:
              key: dnd2024:creatures/swarm_of_rats
            available_rats_radius_miles: 0.5
            availability_choice: gm
            insufficient_rats: charge_wasted
            movement: shortest_route_to_music
            automatically_controlled: false
          control:
            trigger: swarm_enters_30_ft_while_playing
            ineligible: controlled_by_another
            save:
              ability: wisdom
              dc: 15
              failure:
                attitude: friendly_to_user_and_allies
                obeys_commands: true
                no_command_behavior: defend_self_otherwise_no_actions
                requires_continued_magic_action_each_round: true
              success:
                behavior: normal
                immunity_to_pipes:
                  amount: 24
                  unit: hours
            turn_start_distance_over_ft: 30
            out_of_range_result:
              end_control: true
              behavior: normal
              immunity_to_pipes:
                amount: 24
                unit: hours
          attunement_required: true
    contract:
      id: dnd2024:item_pipes_of_the_sewers
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - movement.completed
      - turn.started
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_charges
    requires:
    - dnd2024:item_pipes_of_the_sewers
    citations:
    - SRD 5.2.1 pp. 234; builtin/srd-5.2.1/magic_item/pipes_of_the_sewers.md
  dnd2024:items/potion_of_animal_friendship:
    source_sha256: 850e5c1a813e98062beedd1a779b41f02a30acc3778c86231dc0a82eba853daa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_animal_friendship
        implementation: dnd2024:item_potion_of_animal_friendship
        parameters:
          trigger: potion_drunk
          consumed: true
          cast:
            spell:
              key: dnd2024:spells/animal_friendship
            level: 3
            save_dc: 13
          cast_timing: on_drinking
          optional_cast: true
          attunement_required: false
    contract:
      id: dnd2024:item_potion_of_animal_friendship
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.consumed
      capabilities:
      - item_effects
      - item_spellcasting
    requires:
    - dnd2024:item_potion_of_animal_friendship
    citations:
    - SRD 5.2.1 pp. 235; builtin/srd-5.2.1/magic_item/potion_of_animal_friendship.md
  dnd2024:items/potion_of_diminution:
    source_sha256: 3596a55f56776f351731ffe1a19622be29cd472913e5b73b1833764702ecf068
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_diminution
        implementation: dnd2024:item_potion_of_diminution
        parameters:
          trigger: potion_drunk
          consumed: true
          effect:
            spell:
              key: dnd2024:spells/enlarge_reduce
            mode: reduce
            duration:
              amount:
                roll:
                  count: 1
                  sides: 4
              unit: hours
            concentration: false
          attunement_required: false
    contract:
      id: dnd2024:item_potion_of_diminution
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.consumed
      - time.elapsed
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - effect_duration
    requires:
    - dnd2024:item_potion_of_diminution
    citations:
    - SRD 5.2.1 pp. 235; builtin/srd-5.2.1/magic_item/potion_of_diminution.md
  dnd2024:items/potion_of_giant_strength:
    source_sha256: b918702f4b248a110034a0b3f8f908498b819e161103951fb68ecdc40955c3cb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_giant_strength
        implementation: dnd2024:item_potion_of_giant_strength
        parameters:
          trigger: potion_drunk
          consumed: true
          duration:
            amount: 1
            unit: hours
          ability_floor:
            ability: strength
            when: current_score_below_variant
          variants:
          - giant: hill
            score: 21
            rarity: uncommon
          - giant: frost
            score: 23
            rarity: rare
          - giant: stone
            score: 23
            rarity: rare
          - giant: fire
            score: 25
            rarity: rare
          - giant: cloud
            score: 27
            rarity: very_rare
          - giant: storm
            score: 29
            rarity: legendary
          attunement_required: false
    contract:
      id: dnd2024:item_potion_of_giant_strength
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.consumed
      - time.elapsed
      capabilities:
      - item_effects
      - effect_duration
      - item_variants
    requires:
    - dnd2024:item_potion_of_giant_strength
    citations:
    - SRD 5.2.1 pp. 235; builtin/srd-5.2.1/magic_item/potion_of_giant_strength.md
  dnd2024:items/potion_of_invisibility:
    source_sha256: faa8aaaef03e1b8b945a75bb49281dc4b8ffc0002b79d6dcf078460e7972f518
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_invisibility
        implementation: dnd2024:item_potion_of_invisibility
        parameters:
          trigger: potion_drunk
          consumed: true
          effect:
            condition: invisible
            duration:
              amount: 1
              unit: hours
            ends_on:
            - user.attack_roll
            - user.damage_dealt
            - user.spell_cast
          attunement_required: false
    contract:
      id: dnd2024:item_potion_of_invisibility
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.consumed
      - attack.rolled
      - damage.resolved
      - spell.cast_completed
      - time.elapsed
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - effect_duration
      - damage
      - conditions
    requires:
    - dnd2024:item_potion_of_invisibility
    citations:
    - SRD 5.2.1 pp. 236; builtin/srd-5.2.1/magic_item/potion_of_invisibility.md
  dnd2024:items/potion_of_mind_reading:
    source_sha256: 7a45709ae64f0d40e97dd036b5e888bf6c5649941d9e97c6da5bc5b7cfc649c8
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_mind_reading
        implementation: dnd2024:item_potion_of_mind_reading
        parameters:
          trigger: potion_drunk
          consumed: true
          effect:
            spell:
              key: dnd2024:spells/detect_thoughts
            save_dc: 13
            duration:
              amount: 10
              unit: minutes
            concentration: false
          attunement_required: false
    contract:
      id: dnd2024:item_potion_of_mind_reading
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.consumed
      - time.elapsed
      capabilities:
      - item_effects
      - item_spellcasting
      - effect_duration
    requires:
    - dnd2024:item_potion_of_mind_reading
    citations:
    - SRD 5.2.1 pp. 236; builtin/srd-5.2.1/magic_item/potion_of_mind_reading.md
  dnd2024:items/potion_of_speed:
    source_sha256: e0dd202e57154657424922cf208da71b2a64e9ff6a2d47cba91a3a078a39d2ad
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_speed
        implementation: dnd2024:item_potion_of_speed
        parameters:
          trigger: potion_drunk
          consumed: true
          effect:
            spell:
              key: dnd2024:spells/haste
            duration:
              amount: 1
              unit: minutes
            concentration: false
            end_lethargy: false
          attunement_required: false
    contract:
      id: dnd2024:item_potion_of_speed
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.consumed
      - time.elapsed
      capabilities:
      - item_effects
      - item_spellcasting
      - effect_duration
    requires:
    - dnd2024:item_potion_of_speed
    citations:
    - SRD 5.2.1 pp. 236; builtin/srd-5.2.1/magic_item/potion_of_speed.md
  dnd2024:items/potions_of_healing:
    source_sha256: 7854f672f9523af53ba0fe9fee00bba8edcd5867813da9c96cd80ddc991571ac
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potions_of_healing
        implementation: dnd2024:item_potions_of_healing
        parameters:
          trigger: potion_drunk
          consumed: true
          variants:
          - name: healing
            rarity: common
            heal_hp:
              roll:
                count: 2
                sides: 4
                add: 2
          - name: greater
            rarity: uncommon
            heal_hp:
              roll:
                count: 4
                sides: 4
                add: 4
          - name: superior
            rarity: rare
            heal_hp:
              roll:
                count: 8
                sides: 4
                add: 8
          - name: supreme
            rarity: very_rare
            heal_hp:
              roll:
                count: 10
                sides: 4
                add: 20
          attunement_required: false
    contract:
      id: dnd2024:item_potions_of_healing
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.consumed
      capabilities:
      - item_effects
      - dice
      - item_variants
      - healing
    requires:
    - dnd2024:item_potions_of_healing
    citations:
    - SRD 5.2.1 pp. 236; builtin/srd-5.2.1/magic_item/potions_of_healing.md
  dnd2024:items/ring_of_djinni_summoning:
    source_sha256: f961a07567b9bb4994262672d48a9ad406cbee54568ecbf5417d98ebcbffed49
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_djinni_summoning
        implementation: dnd2024:item_ring_of_djinni_summoning
        parameters:
          equipped: worn
          action: magic
          summon:
            creature:
              key: dnd2024:creatures/djinni
            identity: same_particular_djinni
            origin_plane: elemental_air
            space:
              unoccupied: true
              choice: user
              range_ft: 120
            duration:
              amount: 1
              unit: hours
            concentration: true
            ends_on:
            - concentration_ends
            - zero_hp
            attitude: friendly_to_user_and_allies
            obeys_commands: true
            no_command_behavior: defend_self_otherwise_no_actions
          cooldown:
            from: djinni_departure
            duration:
              amount: 24
              unit: hours
          djinni_dies:
            item_magic: false
          attunement_required: true
    contract:
      id: dnd2024:item_ring_of_djinni_summoning
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - concentration.ended
      - hp.changed
      - participant.died
      - time.elapsed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - effect_duration
      - item_recharge
      - spatial_targeting
      - summoning
      - planar_travel
    requires:
    - dnd2024:item_ring_of_djinni_summoning
    citations:
    - SRD 5.2.1 pp. 237; builtin/srd-5.2.1/magic_item/ring_of_djinni_summoning.md
  dnd2024:items/ring_of_feather_falling:
    source_sha256: 347a02ba1f355cf6d587dd4d3cb2271c6f1b384fa6c026ce782de53020bf1487
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_feather_falling
        implementation: dnd2024:item_ring_of_feather_falling
        parameters:
          equipped: worn
          trigger: falling
          descent_ft_per_round: 60
          fall_damage: 0
          attunement_required: true
    contract:
      id: dnd2024:item_ring_of_feather_falling
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - fall.started
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - damage
    requires:
    - dnd2024:item_ring_of_feather_falling
    citations:
    - SRD 5.2.1 pp. 238; builtin/srd-5.2.1/magic_item/ring_of_feather_falling.md
  dnd2024:items/ring_of_jumping:
    source_sha256: 87139f21b1c28418a24ecbba08261380f7fa7dd36fd53bd319f637346b9ecf73
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_jumping
        implementation: dnd2024:item_ring_of_jumping
        parameters:
          equipped: worn
          spells:
          - spell:
              key: dnd2024:spells/jump
            target: self
            usage: at_will
          attunement_required: true
    contract:
      id: dnd2024:item_ring_of_jumping
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
    requires:
    - dnd2024:item_ring_of_jumping
    citations:
    - SRD 5.2.1 pp. 238; builtin/srd-5.2.1/magic_item/ring_of_jumping.md
  dnd2024:items/ring_of_regeneration:
    source_sha256: 1b0a97599a127a104790956add77bc36586a37527119a1f987c94dabc3c697ad
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_regeneration
        implementation: dnd2024:item_ring_of_regeneration
        parameters:
          equipped: worn
          healing:
            amount:
              roll:
                count: 1
                sides: 6
            interval:
              amount: 10
              unit: minutes
            requires_hp_minimum: 1
          regrow_part:
            trigger: body_part_lost
            duration:
              amount:
                roll:
                  count: 1
                  sides: 6
                  add: 1
              unit: days
            requires_hp_minimum_throughout: 1
            full_functionality: true
          attunement_required: true
    contract:
      id: dnd2024:item_ring_of_regeneration
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - time.elapsed
      - body_part.lost
      - hp.changed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - effect_duration
      - healing
    requires:
    - dnd2024:item_ring_of_regeneration
    citations:
    - SRD 5.2.1 pp. 238; builtin/srd-5.2.1/magic_item/ring_of_regeneration.md
  dnd2024:items/ring_of_spell_storing:
    source_sha256: abfe52242f392769fb91f1cc3cabed1bddc64c45c632812a131029550c179a89
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_spell_storing
        implementation: dnd2024:item_ring_of_spell_storing
        parameters:
          equipped: worn
          spell_storage:
            capacity_spell_levels: 5
            initial_spell_levels:
              roll:
                count: 1
                sides: 6
                add: -1
            initial_spells_choice: gm
            store:
              caster: any_creature
              touch_required_during_cast: true
              spell_level_min: 1
              spell_level_max: 5
              occupancy: cast_slot_level
              normal_spell_effect: false
              insufficient_capacity: expend_without_effect
            release:
              cast_as: user
              copy_original_caster:
              - slot_level
              - save_dc
              - spell_attack_bonus
              - spellcasting_ability
              remove_spell_from_storage: true
              free_capacity: true
          attunement_required: true
    contract:
      id: dnd2024:item_ring_of_spell_storing
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - inventory_containers
    requires:
    - dnd2024:item_ring_of_spell_storing
    citations:
    - SRD 5.2.1 pp. 239; builtin/srd-5.2.1/magic_item/ring_of_spell_storing.md
  dnd2024:items/ring_of_telekinesis:
    source_sha256: 364ebc0eec540126bd89c2a054728a1648de43bbc7cc4609ddcad0aa744b9a78
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_telekinesis
        implementation: dnd2024:item_ring_of_telekinesis
        parameters:
          equipped: worn
          spells:
          - spell:
              key: dnd2024:spells/telekinesis
            usage: at_will
          attunement_required: true
    contract:
      id: dnd2024:item_ring_of_telekinesis
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
    requires:
    - dnd2024:item_ring_of_telekinesis
    citations:
    - SRD 5.2.1 pp. 239; builtin/srd-5.2.1/magic_item/ring_of_telekinesis.md
  dnd2024:items/ring_of_warmth:
    source_sha256: eccba3a58f8a02c349323605f628d7f41ca737025ef140fcc35856c99fdc968b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_warmth
        implementation: dnd2024:item_ring_of_warmth
        parameters:
          equipped: worn
          damage_reduction:
            type: cold
            amount:
              roll:
                count: 2
                sides: 8
            minimum_final_damage: 0
          temperature_protection:
            fahrenheit_maximum: 0
            lower_limit: null
            targets:
            - wearer
            - worn_items
            - carried_items
          attunement_required: true
    contract:
      id: dnd2024:item_ring_of_warmth
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - damage.pending
      - temperature.exposure
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - damage
    requires:
    - dnd2024:item_ring_of_warmth
    citations:
    - SRD 5.2.1 pp. 239; builtin/srd-5.2.1/magic_item/ring_of_warmth.md
  dnd2024:items/robe_of_eyes:
    source_sha256: a63dcfd7e535713e598f70bce4ca46a805dcebc7f75acddee72c54ef0a6d55b4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/robe_of_eyes
        implementation: dnd2024:item_robe_of_eyes
        parameters:
          equipped: worn
          checks:
          - ability: wisdom
            skill: perception
            requires: sight
            mode: advantage
          senses:
            darkvision_ft: 120
            truesight_ft: 120
          drawbacks:
          - trigger:
              spell:
                key: dnd2024:spells/light
              target: robe
            effect:
              condition: blinded
              duration:
                amount: 1
                unit: minutes
            repeat_save:
              ability: constitution
              dc: 11
              failure:
                continue: true
              success:
                end_effect: true
              repeat: target.turn_end
          - trigger:
              spell:
                key: dnd2024:spells/daylight
              distance_from_robe_ft_max: 5
            effect:
              condition: blinded
              duration:
                amount: 1
                unit: minutes
            repeat_save:
              ability: constitution
              dc: 15
              failure:
                continue: true
              success:
                end_effect: true
              repeat: target.turn_end
          attunement_required: true
    contract:
      id: dnd2024:item_robe_of_eyes
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_completed
      - turn.ended
      - ability_check.pending
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
      - effect_duration
      - conditions
    requires:
    - dnd2024:item_robe_of_eyes
    citations:
    - SRD 5.2.1 pp. 240; builtin/srd-5.2.1/magic_item/robe_of_eyes.md
  dnd2024:items/robe_of_the_archmagi:
    source_sha256: bb6bb9d773588e504b7a0dd523e5db26de8cf83125a87e4ac718823800937ebe
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/robe_of_the_archmagi
        implementation: dnd2024:item_robe_of_the_archmagi
        parameters:
          equipped: worn
          attunement_classes:
          - sorcerer
          - warlock
          - wizard
          armor_class:
            requires_no_armor: true
            base:
              add:
              - 15
              - dexterity_modifier
          saving_throw_advantage:
            against:
            - spells
            - other_magical_effects
          spell_save_dc_bonus: 2
          spell_attack_bonus: 2
          attunement_required: true
    contract:
      id: dnd2024:item_robe_of_the_archmagi
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.equipment_changed
      - saving_throw.pending
      - spell.cast_pending
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
    requires:
    - dnd2024:item_robe_of_the_archmagi
    citations:
    - SRD 5.2.1 pp. 240; builtin/srd-5.2.1/magic_item/robe_of_the_archmagi.md
  dnd2024:items/rod_of_alertness:
    source_sha256: 663d1fbb2d06ff8f5dd3a32cd457c393f09d8f929dd409214233890347f22c5c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/rod_of_alertness
        implementation: dnd2024:item_rod_of_alertness
        parameters:
          while_held:
            advantage:
            - wisdom_perception
            - initiative
            spells:
            - spell:
                key: dnd2024:spells/detect_evil_and_good
              usage: at_will
            - spell:
                key: dnd2024:spells/detect_magic
              usage: at_will
            - spell:
                key: dnd2024:spells/detect_poison_and_disease
              usage: at_will
            - spell:
                key: dnd2024:spells/see_invisibility
              usage: at_will
          protective_aura:
            action: magic
            plant_haft_in_ground: true
            light:
              bright_radius_ft: 60
              additional_dim_ft: 60
            beneficiaries:
              relation:
              - self
              - ally
              within: rod_bright_light
            ac_bonus: 1
            save_bonus: 1
            sense_invisible_creature_location:
              creature_within: rod_bright_light
            duration:
              amount: 10
              unit: minutes
            ends_on:
              action: magic
              actor: any_creature
              operation: pull_rod_from_ground
            uses:
              maximum: 1
              recovery_event: time.dawn
          attunement_required: true
    contract:
      id: dnd2024:item_rod_of_alertness
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - spell.cast_pending
      - time.dawn
      - time.elapsed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
      - effect_duration
    requires:
    - dnd2024:item_rod_of_alertness
    citations:
    - SRD 5.2.1 pp. 241; builtin/srd-5.2.1/magic_item/rod_of_alertness.md
  dnd2024:items/rod_of_rulership:
    source_sha256: 6b4ee308fab03fad7c66c7294394d6a9496ea872bf10262485f562480c7b964f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/rod_of_rulership
        implementation: dnd2024:item_rod_of_rulership
        parameters:
          action: magic
          activation: present_rod_and_command_obedience
          targets:
            choice: user
            visible: true
            range_ft: 120
          save:
            ability: wisdom
            dc: 15
            failure:
              condition: charmed
              duration:
                amount: 8
                unit: hours
              regards_user_as: trusted_leader
              ends_on:
              - harmed_by_user
              - harmed_by_user_ally
              - command_contrary_to_nature
          uses:
            maximum: 1
            recovery_event: time.dawn
          attunement_required: true
    contract:
      id: dnd2024:item_rod_of_rulership
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - participant.harmed
      - command.issued
      - time.dawn
      - time.elapsed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - effect_duration
      - spatial_targeting
      - conditions
    requires:
    - dnd2024:item_rod_of_rulership
    citations:
    - SRD 5.2.1 pp. 242; builtin/srd-5.2.1/magic_item/rod_of_rulership.md
  dnd2024:items/rope_of_entanglement:
    source_sha256: 167b8eba18eb95f91f2d1ddd65095108a391374af9cece6b9d5131a2d4f2ad75
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/rope_of_entanglement
        implementation: dnd2024:item_rope_of_entanglement
        parameters:
          length_ft: 30
          equipped: holding_one_end
          activate:
            action: magic
            command: true
            target:
              visible: true
              range_ft: 20
              creature_count: 1
            save:
              ability: dexterity
              dc: 15
              failure:
                condition: restrained
          release_options:
          - trigger: user_lets_go
            rope_location: target_space
            rope_state: coiled
          - action: bonus_action
            repeat_command: true
            rope_location: user_hand
            rope_state: coiled
          escape:
            action: action
            check_choice:
            - ability: strength
              skill: athletics
              dc: 15
            - ability: dexterity
              skill: acrobatics
              dc: 15
            success: end_restrained
            user_reaction:
              requires: holding_rope
              command: coil_into_user_hand
            otherwise_rope_location: target_space
          object_statistics:
            ac: 20
            hp: 20
            damage_immunities:
            - poison
            - psychic
            healing:
              hp: 1
              interval:
                amount: 5
                unit: minutes
              requires_hp_minimum: 1
            zero_hp: destroyed
          restrained_ends_on:
          - rope_destroyed
          - release
          - successful_escape_check
          attunement_required: false
    contract:
      id: dnd2024:item_rope_of_entanglement
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - item.grip_changed
      - item.hp_changed
      - time.elapsed
      capabilities:
      - item_effects
      - spatial_targeting
      - damage
      - healing
      - conditions
    requires:
    - dnd2024:item_rope_of_entanglement
    citations:
    - SRD 5.2.1 pp. 242, 243; builtin/srd-5.2.1/magic_item/rope_of_entanglement.md
  dnd2024:items/sending_stones:
    source_sha256: eeb2d3fdb2b21f74a991564fe41b543b0f8b77bdd884f1312329fc63aefe7250
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/sending_stones
        implementation: dnd2024:item_sending_stones
        parameters:
          pair_size: 2
          equipped: touching
          spells:
          - spell:
              key: dnd2024:spells/sending
            target: other_stone_bearer
            uses:
              maximum: 1
              recovery_event: time.dawn
            usage_pool: shared_pair
          no_other_bearer:
            inform_user: true
            cast_spell: false
            expend_use: false
          one_stone_destroyed:
            other_stone_magic: false
          attunement_required: false
    contract:
      id: dnd2024:item_sending_stones
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - time.dawn
      - item.destroyed
      capabilities:
      - item_effects
      - item_spellcasting
    requires:
    - dnd2024:item_sending_stones
    citations:
    - SRD 5.2.1 pp. 243; builtin/srd-5.2.1/magic_item/sending_stones.md
  dnd2024:items/shield_of_missile_attraction:
    source_sha256: 4725125d72543dbf4bf824ef3c4ec2e5f947d517b654600e3cb8667b1578f788
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/shield_of_missile_attraction
        implementation: dnd2024:item_shield_of_missile_attraction
        parameters:
          while_held:
            damage_resistance:
              source: attacks_with_ranged_weapons
          curse:
            apply_on: attunement_completed
            persists_after_doff: true
            ends_on:
            - spell:
                key: dnd2024:spells/remove_curse
              target: user
            - magic_equivalent: remove_curse
              adjudicator: gm
            redirect_attack:
              weapon_category: ranged
              original_target_distance_from_user_ft_max: 10
              new_target: user
          attunement_required: true
    contract:
      id: dnd2024:item_shield_of_missile_attraction
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - attack.target_selected
      - damage.pending
      - item.attunement_completed
      - spell.cast_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - item_spellcasting
      - curses
      - damage
    requires:
    - dnd2024:item_shield_of_missile_attraction
    citations:
    - SRD 5.2.1 pp. 243; builtin/srd-5.2.1/magic_item/shield_of_missile_attraction.md
  dnd2024:items/sovereign_glue:
    source_sha256: ffebf00253066fcf2c92919b0c83b7c8e6a12fee9d643755fde54fe4927a9c29
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/sovereign_glue
        implementation: dnd2024:item_sovereign_glue
        parameters:
          initial_ounces:
            roll:
              count: 1
              sides: 6
              add: 1
          storage:
            container_types:
            - jar
            - flask
            interior_coating:
              key: dnd2024:items/oil_of_slipperiness
          apply:
            action: utilize
            ounces_per_action: 1
            square_ft_per_ounce: 1
            bond_targets: two_objects
            set_time:
              amount: 1
              unit: minutes
            consumes_applied_ounces: true
          bond:
            permanent: true
            broken_only_by:
            - key: dnd2024:items/universal_solvent
            - key: dnd2024:items/oil_of_etherealness
            - key: dnd2024:spells/wish
          attunement_required: false
    contract:
      id: dnd2024:item_sovereign_glue
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - time.elapsed
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - inventory_containers
    requires:
    - dnd2024:item_sovereign_glue
    citations:
    - SRD 5.2.1 pp. 244; builtin/srd-5.2.1/magic_item/sovereign_glue.md
  dnd2024:items/sphere_of_annihilation:
    source_sha256: 0e249ca017432ebdc743338a33a9265fd1d158a3f5e712f7c95e72936e3dcc93
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/sphere_of_annihilation
        implementation: dnd2024:item_sphere_of_annihilation
        parameters:
          diameter_ft: 2
          hovering: true
          default_motion: stationary
          contact:
            wholly_engulfed: matter_obliterated
            partly_touching:
              damage:
                amount:
                  roll:
                    count: 8
                    sides: 10
                type: force
            artifact_exception: unharmed_unless_artifact_explicitly_susceptible
          control:
            action: magic
            maximum_range_ft: 60
            check:
              ability: intelligence
              skill: arcana
              dc: 25
              success:
                control_until: user.next_turn_start
                previous_controller_loses_control: true
              failure:
                move_ft: 10
                direction: straight_toward_user
          controlled_movement:
            action: bonus_action
            direction_choice: one_direction
            maximum_distance_ft:
              max:
              - 5
              - multiply:
                - 5
                - user.intelligence_modifier
          interaction:
            trigger:
            - planar_portal_contact
            - extradimensional_space_contact
            die: 100
            rows:
            - range:
              - 1
              - 50
              destroy_sphere: true
            - range:
              - 51
              - 85
              sphere: passes_through_or_enters
            - range:
              - 86
              - 100
              transport:
              - sphere
              - creatures
              - objects
              radius_ft: 180
              destination_plane: random
          on_any_sphere_movement_into_creature_space:
            save:
              ability: dexterity
              dc: 19
              failure:
                damage:
                  amount:
                    roll:
                      count: 8
                      sides: 10
                  type: force
            zero_hp:
              obliterate_body: true
              leave_possessions: true
              physical_remains: false
          attunement_required: false
    contract:
      id: dnd2024:item_sphere_of_annihilation
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - matter.contact
      - movement.completed
      - turn.started
      - portal.contact
      - extradimensional_space.contact
      capabilities:
      - item_effects
      - dice
      - spatial_targeting
      - planar_travel
      - damage
    requires:
    - dnd2024:item_sphere_of_annihilation
    citations:
    - SRD 5.2.1 pp. 244, 245; builtin/srd-5.2.1/magic_item/sphere_of_annihilation.md
  dnd2024:items/staff_of_frost:
    source_sha256: 6bae851f757a80d8e16f408516eb83f16229b1a4d81423d6b51f83e9aa2fa569
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_frost
        implementation: dnd2024:item_staff_of_frost
        parameters:
          equipped: held
          attunement_classes:
          - druid
          - sorcerer
          - warlock
          - wizard
          resistance:
          - cold
          charges:
            initial: 10
            maximum: 10
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 6
                  add: 4
              cap_at_maximum: true
            last_charge:
              roll:
                roll:
                  count: 1
                  sides: 20
              'on': 1
              result: turn_to_water_and_destroy
          spells:
          - spell:
              key: dnd2024:spells/cone_of_cold
            charge_cost: 5
            save_dc: user.spell_save_dc
          - spell:
              key: dnd2024:spells/ice_storm
            charge_cost: 4
            save_dc: user.spell_save_dc
          - spell:
              key: dnd2024:spells/fog_cloud
            charge_cost: 1
            save_dc: user.spell_save_dc
          - spell:
              key: dnd2024:spells/wall_of_ice
            charge_cost: 4
            save_dc: user.spell_save_dc
          attunement_required: true
    contract:
      id: dnd2024:item_staff_of_frost
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - time.dawn
      - damage.pending
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - item_charges
    requires:
    - dnd2024:item_staff_of_frost
    citations:
    - SRD 5.2.1 pp. 245; builtin/srd-5.2.1/magic_item/staff_of_frost.md
  dnd2024:items/staff_of_striking:
    source_sha256: caee2781bd71e7fd849ebae4b45a2a7aada58678b8120f7ae84f50a9f7d1c8fa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_striking
        implementation: dnd2024:item_staff_of_striking
        parameters:
          weapon:
            key: dnd2024:equipment/quarterstaff
          magical_weapon: true
          attack_bonus: 3
          damage_bonus: 3
          charges:
            initial: 10
            maximum: 10
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 6
                  add: 4
              cap_at_maximum: true
            last_charge:
              roll:
                roll:
                  count: 1
                  sides: 20
              'on': 1
              result: nonmagical_quarterstaff
          on_melee_hit:
            optional_charge_spend:
              minimum: 1
              maximum: 3
            extra_damage:
              type: force
              amount:
                roll:
                  count: charges_spent
                  sides: 6
          attunement_required: true
    contract:
      id: dnd2024:item_staff_of_striking
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - attack.hit
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_charges
      - damage
    requires:
    - dnd2024:item_staff_of_striking
    citations:
    - SRD 5.2.1 pp. 246; builtin/srd-5.2.1/magic_item/staff_of_striking.md
  dnd2024:items/staff_of_the_python:
    source_sha256: 4037206518335f4d9625619c028106fd6c41109d90752bba3f614c8a9c3902f2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_the_python
        implementation: dnd2024:item_staff_of_the_python
        parameters:
          activate:
            action: magic
            throw_range_ft: 10
            unoccupied_space: true
            transform:
              key: dnd2024:creatures/giant_constrictor_snake
          snake:
            controller: user
            initiative: user_initiative
            turn: immediately_after_user
            command:
              action: none
              'on': user_turn
              mental: true
              range_ft: 60
              requires_user_not_incapacitated: true
              choices:
              - action_and_movement
              - general_command
            no_commands: defend_self
          revert:
            action: bonus_action
            command: true
            location: snake_current_space
            heal_snake_to_full: true
            cooldown:
              amount: 1
              unit: hours
          snake_zero_hp:
          - snake_dies
          - revert_to_staff
          - staff_shatters_and_destroyed
          attunement_required: true
    contract:
      id: dnd2024:item_staff_of_the_python
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - turn.started
      - hp.changed
      - time.elapsed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_recharge
      - spatial_targeting
      - healing
    requires:
    - dnd2024:item_staff_of_the_python
    citations:
    - SRD 5.2.1 pp. 247; builtin/srd-5.2.1/magic_item/staff_of_the_python.md
  dnd2024:items/staff_of_withering:
    source_sha256: ed6b176985c35ba1dfe3c40326c7b82bc0bb72eea442c3bc4d1b139b5a39f09e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_withering
        implementation: dnd2024:item_staff_of_withering
        parameters:
          weapon:
            key: dnd2024:equipment/quarterstaff
          magical_weapon: true
          base_damage: normal_quarterstaff
          charges:
            initial: 3
            maximum: 3
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 3
              cap_at_maximum: true
          on_hit:
            optional_charge_cost: 1
            damage:
              amount:
                roll:
                  count: 2
                  sides: 10
              type: necrotic
            save:
              ability: constitution
              dc: 15
              failure:
                disadvantage:
                  test_kinds:
                  - ability_check
                  - saving_throw
                  abilities:
                  - strength
                  - constitution
                duration:
                  amount: 1
                  unit: hours
          attunement_required: true
    contract:
      id: dnd2024:item_staff_of_withering
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - attack.hit
      - time.dawn
      - time.elapsed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_charges
      - effect_duration
      - damage
    requires:
    - dnd2024:item_staff_of_withering
    citations:
    - SRD 5.2.1 pp. 248; builtin/srd-5.2.1/magic_item/staff_of_withering.md
  dnd2024:items/sun_blade:
    source_sha256: be217217a65dc7a5ad2aa247e17bbb866f35f5cac5e662655be4f04c892e257a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/sun_blade
        implementation: dnd2024:item_sun_blade
        parameters:
          default_form: hilt
          blade_toggle:
            action: bonus_action
            requires: grasping_hilt
          while_blade_exists:
            weapon:
              key: dnd2024:equipment/longsword
            properties_add:
            - finesse
            proficient_if_any:
            - longsword_proficiency
            - shortsword_proficiency
            attack_bonus: 2
            damage_bonus: 2
            replace_damage_type:
              from: slashing
              to: radiant
            on_hit:
              target_type: undead
              damage:
                amount:
                  roll:
                    count: 1
                    sides: 8
                type: radiant
            light:
              bright_radius_ft: 15
              additional_dim_ft: 15
              sunlight: true
            adjust_light:
              action: magic
              increment_ft: 5
              minimum_bright_and_additional_dim_ft: 10
              maximum_bright_and_additional_dim_ft: 30
              adjust_both_together: true
          attunement_required: true
    contract:
      id: dnd2024:item_sun_blade
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - attack.hit
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - damage
    requires:
    - dnd2024:item_sun_blade
    citations:
    - SRD 5.2.1 pp. 248; builtin/srd-5.2.1/magic_item/sun_blade.md
  dnd2024:items/sword_of_wounding:
    source_sha256: 5a1e63f13f321c9aeaa0c31c2d43ec5ea27e15e9f95c354a876f53f1735bfd6f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/sword_of_wounding
        implementation: dnd2024:item_sword_of_wounding
        parameters:
          on_creature_hit:
            damage:
              amount:
                roll:
                  count: 2
                  sides: 6
              type: necrotic
            save:
              ability: constitution
              dc: 15
              failure:
                can_regain_hp: false
                duration:
                  amount: 1
                  unit: hours
              repeat:
                event: target.turn_end
                success: end_effect
          attunement_required: true
    contract:
      id: dnd2024:item_sword_of_wounding
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - attack.hit
      - turn.ended
      - time.elapsed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - effect_duration
      - damage
    requires:
    - dnd2024:item_sword_of_wounding
    citations:
    - SRD 5.2.1 pp. 248; builtin/srd-5.2.1/magic_item/sword_of_wounding.md
  dnd2024:items/talisman_of_ultimate_evil:
    source_sha256: 3f87ee80d016701ad27a963ac4bd94e30a0d73495d7c0ce940f18c4002b78261
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/talisman_of_ultimate_evil
        implementation: dnd2024:item_talisman_of_ultimate_evil
        parameters:
          touch_harm:
            target_type_not_in:
            - fiend
            - undead
            triggers:
            - touch
            - turn_end_holding_or_carrying
            damage:
              amount:
                roll:
                  count: 8
                  sides: 6
              type: necrotic
            requires_attunement: false
          holy_symbol: true
          while_worn_or_held:
            spell_attack_bonus: 2
          charges:
            initial: 6
            maximum: 6
            recovery: null
          ultimate_end:
            equipped_any:
            - worn
            - held
            action: magic
            charge_cost: 1
            target:
              visible: true
              on_ground: true
              range_ft: 120
              creature_count: 1
            save:
              ability: dexterity
              dc: 20
              failure:
                destroy_target: true
                remains: false
              success:
                damage:
                  amount:
                    roll:
                      count: 4
                      sides: 6
                  type: psychic
            save_disadvantage_types:
            - celestial
            fissure: closes_without_trace
          at_zero_charges:
            destroy_item: true
            residue: slime
          attunement_required: true
    contract:
      id: dnd2024:item_talisman_of_ultimate_evil
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.touched
      - turn.ended
      - item.activation_requested
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - item_charges
      - spatial_targeting
      - damage
    requires:
    - dnd2024:item_talisman_of_ultimate_evil
    citations:
    - SRD 5.2.1 pp. 249; builtin/srd-5.2.1/magic_item/talisman_of_ultimate_evil.md
  dnd2024:items/tome_of_leadership_and_influence:
    source_sha256: f0bc697bcfd508acef2217cecd4e2377bb990c02bcb53241591debbcf230f577
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/tome_of_leadership_and_influence
        implementation: dnd2024:item_tome_of_leadership_and_influence
        parameters:
          study:
            total_hours: 48
            within_days: 6
            practice_required: true
          completion:
            ability: charisma
            increase: 2
            maximum: 30
            item_magic: false
          magic_recovery:
            amount: 100
            unit: years
          attunement_required: false
    contract:
      id: dnd2024:item_tome_of_leadership_and_influence
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - study.progressed
      - time.elapsed
      capabilities:
      - item_effects
    requires:
    - dnd2024:item_tome_of_leadership_and_influence
    citations:
    - SRD 5.2.1 pp. 249, 250; builtin/srd-5.2.1/magic_item/tome_of_leadership_and_influence.md
  dnd2024:items/universal_solvent:
    source_sha256: 05ab90f5f1c667c04917e6c15a75c3f9b38e3be383c29ef55dcec9ee4999a9ae
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/universal_solvent
        implementation: dnd2024:item_universal_solvent
        parameters:
          initial_ounces:
            roll:
              count: 1
              sides: 6
              add: 1
          apply:
            action: utilize
            ounces_minimum: 1
            surface: within_reach
            square_ft_adhesive_per_ounce: 1
            effect_timing: instantaneous
            operation: dissolve_adhesive
            includes:
              key: dnd2024:items/sovereign_glue
            consumes_applied_ounces: true
          attunement_required: false
    contract:
      id: dnd2024:item_universal_solvent
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      capabilities:
      - item_effects
      - dice
    requires:
    - dnd2024:item_universal_solvent
    citations:
    - SRD 5.2.1 pp. 250; builtin/srd-5.2.1/magic_item/universal_solvent.md
  dnd2024:items/wand_of_binding:
    source_sha256: 366a72c32566ee80d01e20b84f804d8c55086e764bebae814ab0b399d8fe0d3e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_binding
        implementation: dnd2024:item_wand_of_binding
        parameters:
          equipped: held
          charges:
            initial: 7
            maximum: 7
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 6
                  add: 1
              cap_at_maximum: true
            last_charge:
              roll:
                roll:
                  count: 1
                  sides: 20
              'on': 1
              result: destroy_to_ashes
          spells:
          - spell:
              key: dnd2024:spells/hold_monster
            charge_cost: 5
            save_dc: 17
          - spell:
              key: dnd2024:spells/hold_person
            charge_cost: 2
            save_dc: 17
          attunement_required: true
    contract:
      id: dnd2024:item_wand_of_binding
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - item_charges
    requires:
    - dnd2024:item_wand_of_binding
    citations:
    - SRD 5.2.1 pp. 250; builtin/srd-5.2.1/magic_item/wand_of_binding.md
  dnd2024:items/wand_of_fireballs:
    source_sha256: 5bc41e2daea844f07a71ac8b08ea7171579e7872354a96a30b6f83c705bc5a16
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_fireballs
        implementation: dnd2024:item_wand_of_fireballs
        parameters:
          equipped: held
          attunement_requires: spellcaster
          charges:
            initial: 7
            maximum: 7
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 6
                  add: 1
              cap_at_maximum: true
            last_charge:
              roll:
                roll:
                  count: 1
                  sides: 20
              'on': 1
              result: destroy_to_ashes
          spells:
          - spell:
              key: dnd2024:spells/fireball
            save_dc: 15
            charge_choice:
              min: 1
              max: 3
            cast_level:
              add:
              - 2
              - charges_spent
          attunement_required: true
    contract:
      id: dnd2024:item_wand_of_fireballs
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - item_charges
    requires:
    - dnd2024:item_wand_of_fireballs
    citations:
    - SRD 5.2.1 pp. 250, 251; builtin/srd-5.2.1/magic_item/wand_of_fireballs.md
  dnd2024:items/wand_of_magic_missiles:
    source_sha256: 161d59ced6a793a2083580bc237df6c9187f764a9716656669e2997b1a78c74c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_magic_missiles
        implementation: dnd2024:item_wand_of_magic_missiles
        parameters:
          equipped: held
          charges:
            initial: 7
            maximum: 7
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 6
                  add: 1
              cap_at_maximum: true
            last_charge:
              roll:
                roll:
                  count: 1
                  sides: 20
              'on': 1
              result: destroy_to_ashes
          spells:
          - spell:
              key: dnd2024:spells/magic_missile
            charge_choice:
              min: 1
              max: 3
            cast_level: charges_spent
          attunement_required: false
    contract:
      id: dnd2024:item_wand_of_magic_missiles
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - spell.cast_pending
      - time.dawn
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - item_charges
    requires:
    - dnd2024:item_wand_of_magic_missiles
    citations:
    - SRD 5.2.1 pp. 251; builtin/srd-5.2.1/magic_item/wand_of_magic_missiles.md
  dnd2024:items/wand_of_secrets:
    source_sha256: 5ba34ef62923a89918b43f84e96afc440ef31debb227bfc69c1bc423f380d3ba
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_secrets
        implementation: dnd2024:item_wand_of_secrets
        parameters:
          equipped: held
          charges:
            initial: 3
            maximum: 3
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 3
              cap_at_maximum: true
          action: magic
          charge_cost: 1
          detection:
            types:
            - secret_door
            - trap
            range_ft: 60
            target_selection: nearest
            indication:
            - pulse
            - point_to_target
            none_in_range: no_indication
          attunement_required: false
    contract:
      id: dnd2024:item_wand_of_secrets
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - time.dawn
      capabilities:
      - item_effects
      - dice
      - item_charges
      - spatial_targeting
    requires:
    - dnd2024:item_wand_of_secrets
    citations:
    - SRD 5.2.1 pp. 251; builtin/srd-5.2.1/magic_item/wand_of_secrets.md
  dnd2024:items/wand_of_wonder:
    source_sha256: c231c5a2e83f48c9ff9462d925aa497e9e853a660ccfce7ebe4412f36a3fc909
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_wonder
        implementation: dnd2024:item_wand_of_wonder
        parameters:
          equipped: held
          action: magic
          charge_cost: 1
          charges:
            initial: 7
            maximum: 7
            recovery:
              event: time.dawn
              amount:
                roll:
                  count: 1
                  sides: 6
                  add: 1
              cap_at_maximum: true
            last_charge:
              roll:
                roll:
                  count: 1
                  sides: 20
              'on': 1
              result: destroy_to_dust
          chosen_origin:
            type: point
            distance_from_user_ft_max: 120
          spell_overrides:
            save_dc: 15
            range_ft_minimum: 120
          multiple_possible_subjects:
            selection: random
            adjudicator: gm
          effect_die: 100
          effects:
          - range:
            - 1
            - 20
            effect:
              cast_at_origin: true
              spell_roll:
                die: 10
                rows:
                - range:
                  - 1
                  - 2
                  spell:
                    key: dnd2024:spells/darkness
                - range:
                  - 3
                  - 4
                  spell:
                    key: dnd2024:spells/faerie_fire
                - range:
                  - 5
                  - 6
                  spell:
                    key: dnd2024:spells/fireball
                - range:
                  - 7
                  - 8
                  spell:
                    key: dnd2024:spells/slow
                - range:
                  - 9
                  - 10
                  spell:
                    key: dnd2024:spells/stinking_cloud
          - range:
            - 21
            - 25
            effect:
              target: user
              condition: stunned
              ends: user.next_turn_start
              chosen_origin_effect: false
          - range:
            - 26
            - 30
            effect:
              spell:
                key: dnd2024:spells/gust_of_wind
              line:
                from: user
                to: chosen_origin
          - range:
            - 31
            - 35
            effect:
              target: user
              damage:
                amount:
                  roll:
                    count: 1
                    sides: 6
                type: psychic
              chosen_origin_effect: false
          - range:
            - 36
            - 40
            effect:
              weather: heavy_rain
              area:
                shape: cylinder
                origin: chosen_origin
                height_ft: 120
                radius_ft: 60
              duration:
                amount: 1
                unit: minutes
              obscuration: lightly_obscured
          - range:
            - 41
            - 45
            effect:
              create: oversized_butterflies
              count: 600
              area:
                shape: cylinder
                origin: chosen_origin
                height_ft: 60
                radius_ft: 30
              duration:
                amount: 10
                unit: minutes
              obscuration: heavily_obscured
          - range:
            - 46
            - 50
            effect:
              spell:
                key: dnd2024:spells/lightning_bolt
              line:
                from: user
                to: chosen_origin
          - range:
            - 51
            - 55
            effect:
              spell:
                key: dnd2024:spells/enlarge_reduce
              mode: enlarge
              target: closest_creature_to_chosen_origin
              ineligible_other_target_fallback: user
          - range:
            - 56
            - 60
            effect:
              summon_roll:
                die: 4
                rows:
                - range:
                  - 1
                  - 1
                  creature:
                    key: dnd2024:creatures/rhinoceros
                - range:
                  - 2
                  - 2
                  creature:
                    key: dnd2024:creatures/elephant
                - range:
                  - 3
                  - 4
                  creature:
                    key: dnd2024:creatures/rat
              placement: nearest_unoccupied_space_to_chosen_origin
              magically_formed: true
              controlled_by_user: false
              behavior: normal
              duration:
                amount: 1
                unit: hours
              ends_on: zero_hp
          - range:
            - 61
            - 64
            effect:
              create: grass
              area:
                shape: circle
                radius_ft: 60
                ground: true
                center: nearest_to_chosen_origin
              existing_grass:
                size_multiplier: 10
                overgrowth_duration:
                  amount: 1
                  unit: minutes
          - range:
            - 65
            - 68
            effect:
              target:
                kind: object
                choice: gm
                worn_or_carried: false
                distance_from_origin_ft_max: 120
                maximum_each_dimension_ft: 10
              transport_plane: ethereal
              no_eligible_target: no_effect
          - range:
            - 69
            - 72
            effect:
              spell:
                key: dnd2024:spells/enlarge_reduce
              mode: reduce
              target: self
              duration:
                amount: 1
                unit: minutes
              chosen_origin_effect: false
          - range:
            - 73
            - 77
            effect:
              target: closest_creature_to_chosen_origin
              grow: leaves
              ends_on:
              - leaves_picked_off
              - 24_hours_elapsed
          - range:
            - 78
            - 82
            effect:
              chosen_origin_effect: false
              area:
                shape: emanation
                origin: user
                radius_ft: 30
              save:
                ability: constitution
                dc: 15
                failure:
                  condition: blinded
                  duration:
                    amount: 1
                    unit: minutes
                repeat:
                  event: target.turn_end
                  success: end_effect
          - range:
            - 83
            - 87
            effect:
              spell:
                key: dnd2024:spells/invisibility
              target: self
              chosen_origin_effect: false
          - range:
            - 88
            - 92
            effect:
              chosen_origin_effect: false
              gem_count:
                multiply:
                - roll:
                    count: 1
                    sides: 4
                - 10
              gem_value_gp: 1
              area:
                shape: line
                origin: wand_tip
                toward: chosen_origin
                length_ft: 30
                width_ft: 5
              damage:
                type: bludgeoning
                amount_per_gem: 1
                total: gem_count
                allocation: divide_equally_among_all_creatures_in_line
          - range:
            - 93
            - 97
            effect:
              spell:
                key: dnd2024:spells/polymorph
              target: closest_creature_to_chosen_origin
              form_roll:
                die: 4
                rows:
                - range:
                  - 1
                  - 1
                  creature:
                    key: dnd2024:creatures/black_bear
                - range:
                  - 2
                  - 2
                  creature:
                    key: dnd2024:creatures/giant_wasp
                - range:
                  - 3
                  - 4
                  creature:
                    key: dnd2024:creatures/frog
          - range:
            - 98
            - 100
            effect:
              target: closest_creature_to_chosen_origin
              save:
                ability: constitution
                dc: 15
                failure:
                  condition: restrained
                  repeat_at: target.next_turn_end
                  repeat_save:
                    requires:
                      condition: restrained
                      source: this_effect
                      active: true
                    ability: constitution
                    dc: 15
                    failure:
                      replace_condition: petrified
                      ends_on:
                      - spell:
                          key: dnd2024:spells/greater_restoration
                      - magic_equivalent: greater_restoration
                        adjudicator: gm
                    success:
                      end_effect: true
          gem_damage_rounding:
            scope: gem_total_divided_by_creature_count
            policy: system_round_down
            reason: general_round_down_rule
          attunement_required: true
    contract:
      id: dnd2024:item_wand_of_wonder
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - turn.ended
      - time.elapsed
      - time.dawn
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - item_spellcasting
      - item_charges
      - effect_duration
      - spatial_targeting
      - summoning
      - planar_travel
      - damage
      - conditions
    requires:
    - dnd2024:item_wand_of_wonder
    citations:
    - SRD 5.2.1 pp. 251, 252; builtin/srd-5.2.1/magic_item/wand_of_wonder.md
  dnd2024:items/well_of_many_worlds:
    source_sha256: 18f0a244d9e9f8683ca551952bf7f08fb3f16e241faac19ad0a61b73a963d264
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/well_of_many_worlds
        implementation: dnd2024:item_well_of_many_worlds
        parameters:
          folded_size: handkerchief
          unfolded:
            shape: circle
            diameter_ft: 6
          open:
            action: magic
            unfold_and_place_on: solid_surface
            portal:
              two_way: true
              diameter_ft: 6
              shape: circle
              destination:
                choice: gm_each_opening
                options:
                - another_world
                - another_plane
          close:
            action: magic
            actor: any_creature
            maximum_range_ft: 5
            take_edges_and_fold: true
          cooldown:
            starts: portal_opened
            duration:
              amount:
                roll:
                  count: 1
                  sides: 8
              unit: hours
          attunement_required: false
    contract:
      id: dnd2024:item_well_of_many_worlds
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - time.elapsed
      capabilities:
      - item_effects
      - dice
      - effect_duration
      - item_recharge
      - spatial_targeting
      - planar_travel
    requires:
    - dnd2024:item_well_of_many_worlds
    citations:
    - SRD 5.2.1 pp. 253; builtin/srd-5.2.1/magic_item/well_of_many_worlds.md
  dnd2024:items/wings_of_flying:
    source_sha256: 6a443fd72a7c12ebc9b51bc3eb0e237424f5f494a6260d1fda4b3740d10b69d5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wings_of_flying
        implementation: dnd2024:item_wings_of_flying
        parameters:
          equipped: worn
          activate:
            action: magic
            transform: cloak_to_wings
          effect:
            fly_speed_ft: 60
            duration:
              amount: 1
              unit: hours
            early_end_action: magic
            aloft_on_end: fall
          cooldown:
            starts: wings_disappear
            duration:
              amount:
                roll:
                  count: 1
                  sides: 12
              unit: hours
          attunement_required: true
    contract:
      id: dnd2024:item_wings_of_flying
      inputs:
        actor: participant
        item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        destination:
          type: location
          required: false
        selection:
          type: choice
          required: false
        context:
          type: resolution
          required: false
      outputs:
      - effects
      - item_state_changes
      events:
      - item.activation_requested
      - time.elapsed
      - item.attunement_completed
      - item.attunement_ended
      capabilities:
      - item_effects
      - dice
      - effect_duration
      - item_recharge
    requires:
    - dnd2024:item_wings_of_flying
    citations:
    - SRD 5.2.1 pp. 253; builtin/srd-5.2.1/magic_item/wings_of_flying.md
---

# SRD magic item contracts: group 3

All 86 assigned entries are specified. Each entry cites its complete official bundled source and records its SHA-256. These are definition contracts; engine implementation is outside this research.

Spell references inherit the complete canonical spell rules and general magic-item spellcasting rules, with only explicit item overrides in these contracts. Charges cannot exceed their maximum or be spent below zero. Equipment and attunement gates apply to beneficial item properties; explicitly stated contact hazards, curses, storage admission, and physical object behavior remain independent of those gates. A missing charge recovery rule means that the source gives no recovery.

The contracts retain all 22 Mysterious Deck cards, both complete draw tables, and the embedded Avatar of Death statistics. Both Ioun Stone absorption variants have a 20-level lifetime budget in this edition. The reserve stone stores four levels; the ring stores five. The source does not state whether a final absorption may exceed the remaining budget, so the implementation must retain that narrow boundary as a GM choice rather than infer an older edition rule.

Wand of Wonder includes all 18 percentile branches and their nested dice tables. Its spell range floor is 120 feet; the point chosen for a branch does not silently replace a spell-specific origin described by that branch. The source does not define gem damage rounding when equal division is fractional; the contract applies the general system rule to round down.

Potion of Animal Friendship grants the stated level 3 cast on drinking. The source states no hour-long repeated casting benefit. Ring of Warmth reduces each Cold damage instance by 2d8. Cube of Force has the six spell faces listed in this edition. No earlier-edition effects are added.

GM choices are bounded to the source subjects: destination locations, variant selection, available rats, summoned allies, campaign property changes, hostile NPCs, event erasure, and object eligibility. Narrative choices do not stand in for numerical mechanics. Ship statistics used by Folding Boat are from the official Equipment table on page 101.

---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:items/ammunition_1_2_or_3:
    source_sha256: dd40054c96171b9a4a50632716192199cc7b00ca06308999123fd034f3d0add2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ammunition_1_2_or_3
        implementation: dnd2024:item_ammunition_1_2_or_3
        parameters:
          variants:
          - rarity: uncommon
            attack_roll_bonus: 1
            damage_roll_bonus: 1
            scope: attacks_with_source_item
          - rarity: rare
            attack_roll_bonus: 2
            damage_roll_bonus: 2
            scope: attacks_with_source_item
          - rarity: very_rare
            attack_roll_bonus: 3
            damage_roll_bonus: 3
            scope: attacks_with_source_item
          on_hit:
            magical: false
          typical_lot_sizes:
          - 10
          - 20
          valuation:
            pieces: 10
            equivalent_category: potion
            rarity: same
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_ammunition_1_2_or_3
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - attack.roll_pending
      - attack.hit
      capabilities:
      - inventory
      - effects
      - roll_resolution
    requires:
    - dnd2024:item_ammunition_1_2_or_3
    citations:
    - SRD 5.2.1 pp. 209; builtin/srd-5.2.1/magic_item/ammunition_1_2_or_3.md
  dnd2024:items/amulet_of_proof_against_detection_and_location:
    source_sha256: 59782d874bdf79e9e72d1d4275a1c0044aa75e03ceb783eaebfe2a39e27a1f24
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/amulet_of_proof_against_detection_and_location
        implementation: dnd2024:item_amulet_of_proof_against_detection_and_location
        parameters:
          equipment_state: worn
          prevent:
            spell_targeting:
              school: divination
            perception_by: magical_scrying_sensors
          exception:
            chooser: wearer
            allow: true
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_amulet_of_proof_against_detection_and_location
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.target_pending
      - scrying.perception_pending
      - equipment.changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
    requires:
    - dnd2024:item_amulet_of_proof_against_detection_and_location
    citations:
    - SRD 5.2.1 pp. 209; builtin/srd-5.2.1/magic_item/amulet_of_proof_against_detection_and_location.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/apparatus_of_the_crab:
    source_sha256: 42e0537b27b58db95668469da3d4bd7eb74c0f993fd7981ed98c318f38dffd00
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/apparatus_of_the_crab
        implementation: dnd2024:item_apparatus_of_the_crab
        parameters:
          weight_lb: 500
          size: large
          ac: 20
          hp: 200
          damage_immunities:
          - poison
          - psychic
          hidden_catch:
            check:
              ability: intelligence
              skill: investigation
              dc: 20
            success: find_catch
            release_catch: unlock_entry_hatch
          occupants:
            maximum: 2
            sizes:
            - tiny
            - small
            - medium
          pilot_required: 1
          hatch_closed:
            airtight: true
            watertight: true
            breathing_hours:
              op: divide
              args:
              - 10
              - binding: breathing_occupants
            zero_breathing_occupants_air_depletion: false
          floats: true
          safe_depth_ft: 900
          below_safe_depth:
            interval:
              amount: 1
              unit: minute
            damage:
              roll:
                expression: 2d6
                scope: use
              type: bludgeoning
          legs_extended:
            speed_ft: 30
            swim_speed_ft: 30
          legs_retracted:
            speed_ft: 0
            swim_speed_ft: 0
            speed_bonuses_allowed: false
          operate:
            action: utilize
            actor_location: inside_compartment
            maximum_levers: 2
            positions:
            - up
            - down
            reset_after_each_use: neutral
          levers:
          - number: 1
            up:
              legs: extended
            down:
              legs: retracted
          - number: 2
            up:
              forward_window_shutter: open
            down:
              forward_window_shutter: closed
          - number: 3
            up:
              side_window_shutters: open
              count_per_side: 2
            down:
              side_window_shutters: closed
              count_per_side: 2
          - number: 4
            up:
              claws: extended
              count: 2
            down:
              claws: retracted
          - number: 5
            scope: each_extended_claw
            up:
              attack:
                kind: melee
                bonus: 8
                reach_ft: 5
                hit:
                  damage:
                    average: 7
                    roll:
                      expression: 2d6
                      scope: attack
                    type: bludgeoning
            down:
              attack:
                kind: melee
                bonus: 8
                reach_ft: 5
                hit:
                  condition: grappled
                  escape_dc: 15
          - number: 6
            requires: legs_extended
            up:
              movement: forward
              modes:
              - walk
              - swim
            down:
              movement: backward
              modes:
              - walk
              - swim
          - number: 7
            requires: legs_extended
            up:
              turn_degrees: 90
              direction: counterclockwise
            down:
              turn_degrees: 90
              direction: clockwise
          - number: 8
            up:
              light:
                bright_radius_ft: 30
                dim_additional_ft: 30
            down:
              light: 'off'
          - number: 9
            requires: in_liquid
            up:
              vertical_movement: sink
              maximum_ft: 20
            down:
              vertical_movement: rise
              maximum_ft: 20
          - number: 10
            up:
              rear_hatch: open_unsealed
            down:
              rear_hatch: closed_sealed
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_apparatus_of_the_crab
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
        breathing_occupants:
          type: integer
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.utilize_pending
      - time.minute
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - spatial_state
    requires:
    - dnd2024:item_apparatus_of_the_crab
    citations:
    - SRD 5.2.1 pp. 210; builtin/srd-5.2.1/magic_item/apparatus_of_the_crab.md
    - SRD 5.2.1 p. 210; builtin/srd-5.2.1/pages/210.md (complete lever table)
    - SRD 5.2.1 p. 210; builtin/srd-5.2.1/magic_item/armor_of_resistance.md (misplaced Down column)
  dnd2024:items/armor_of_resistance:
    source_sha256: ac7d4eb8d98d9ce69fb842c5275a673aa97783dd088f5f284671f740d3dcdece
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/armor_of_resistance
        implementation: dnd2024:item_armor_of_resistance
        parameters:
          equipment_state: worn
          resistance:
            chooser: gm
            method:
            - choose
            - roll
            table:
              roll:
                expression: 1d10
                scope: use
              rows:
              - min: 1
                max: 1
                result:
                  damage_type: acid
              - min: 2
                max: 2
                result:
                  damage_type: cold
              - min: 3
                max: 3
                result:
                  damage_type: fire
              - min: 4
                max: 4
                result:
                  damage_type: force
              - min: 5
                max: 5
                result:
                  damage_type: lightning
              - min: 6
                max: 6
                result:
                  damage_type: necrotic
              - min: 7
                max: 7
                result:
                  damage_type: poison
              - min: 8
                max: 8
                result:
                  damage_type: psychic
              - min: 9
                max: 9
                result:
                  damage_type: radiant
              - min: 10
                max: 10
                result:
                  damage_type: thunder
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_armor
    contract:
      id: dnd2024:item_armor_of_resistance
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - roll_resolution
    requires:
    - dnd2024:item_armor_of_resistance
    citations:
    - SRD 5.2.1 pp. 210; builtin/srd-5.2.1/magic_item/armor_of_resistance.md
  dnd2024:items/bag_of_beans:
    source_sha256: 8c3b8f32ca74a56bffdf1f20d1ddd35992f5d6876a1a141e098238ee286f3c90
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/bag_of_beans
        implementation: dnd2024:item_bag_of_beans
        parameters:
          beans:
            initial:
              expression: 3d4
              scope: use
            on_empty:
              magical: false
          weight_lb: 0.5
          weight_independent_of_contents: true
          dump:
            minimum_beans: 1
            destroy: all_dumped_beans
            area:
              shape: sphere
              radius_ft: 10
              origin: beans
              include_actor: true
            save:
              ability: dexterity
              dc: 15
              failure:
                damage:
                  roll:
                    expression: 5d4
                    scope: use
                  type: force
              success:
                damage_scale:
                  numerator: 1
                  denominator: 2
          plant:
            cost_beans: 1
            substrates:
            - dirt
            - sand
            water_required: true
            consume_bean: true
            delay:
              amount: 1
              unit: minute
            origin: planted_location
            chooser: gm
            method:
            - choose
            - roll
            table:
              roll:
                expression: 1d100
                scope: use
              rows:
              - min: 1
                max: 1
                result:
                  spawn: toadstools
                  count:
                    expression: 5d4
                    scope: use
                  on_eat:
                    roll_parity:
                      odd:
                        save:
                          ability: constitution
                          dc: 15
                          failure:
                            damage:
                              roll:
                                expression: 5d6
                                scope: use
                              type: poison
                            condition: poisoned
                            duration:
                              amount: 1
                              unit: hour
                          success:
                            effect: none
                      even:
                        temporary_hp:
                          expression: 5d6
                          scope: use
                        duration:
                          amount: 1
                          unit: hour
              - min: 2
                max: 10
                result:
                  spawn: geyser
                  height_ft: 30
                  duration:
                    roll:
                      expression: 1d4
                      scope: use
                    unit: minute
                  fluid:
                    chooser: gm
                    options:
                    - water
                    - beer
                    - mayonnaise
                    - tea
                    - vinegar
                    - wine
                    - oil
              - min: 11
                max: 20
                result:
                  spawn:
                    key: dnd2024:creatures/treant
                  alignment_by_roll_parity:
                    odd: chaotic_evil
                    even: chaotic_good
              - min: 21
                max: 30
                result:
                  spawn: stone_statue
                  appearance: planter
                  animate: true
                  mobile: false
                  behavior:
                  - threaten_planter
                  - if_planter_absent_describe_planter_as_villain
                  - direct_visitors_to_find_and_attack_planter
                  track_planter:
                    same_plane_required: true
                    exact_location_known: true
                  becomes_inanimate_after:
                    amount: 24
                    unit: hour
              - min: 31
                max: 40
                result:
                  spawn: green_campfire
                  duration:
                    amount: 24
                    unit: hour
                  ends_on: extinguished
              - min: 41
                max: 50
                result:
                  spawn:
                    key: dnd2024:creatures/shrieker_fungus
                  count: 3
              - min: 51
                max: 60
                result:
                  spawn: pink_toads
                  count:
                    expression: 1d4+4
                    scope: use
                  on_touch:
                    transform:
                      chooser: gm
                      definition_category: creature
                      maximum_size: large
                    behavior: alignment_and_nature
                    disappear_after:
                      amount: 1
                      unit: minute
              - min: 61
                max: 70
                result:
                  spawn:
                    key: dnd2024:creatures/bulette
                  entry: burrow_up
                  behavior: attack
              - min: 71
                max: 80
                result:
                  spawn: fruit_tree
                  fruit_count:
                    expression: 1d10+20
                    scope: use
                  magic_fruit_count:
                    expression: 1d8
                    scope: use
                  magic_fruit:
                    selection: random
                    definition_category: potion
                    effect: selected_potion
                  tree_vanishes_after:
                    amount: 1
                    unit: hour
                  picked_fruit_remains: true
                  picked_magic_expires_after:
                    amount: 30
                    unit: day
              - min: 81
                max: 90
                result:
                  spawn: rainbow_eggs
                  count:
                    expression: 1d4+3
                    scope: use
                  on_eat:
                    save:
                      ability: constitution
                      dc: 20
                      failure:
                        damage:
                          roll:
                            expression: 10d6
                            scope: use
                          type: force
                      success:
                        ability_score_increase: 1
                        ability_selector: lowest
                        tie_breaker: random
                        duration: permanent
              - min: 91
                max: 95
                result:
                  spawn: pyramid
                  base_square_ft: 60
                  interior: burial_chamber
                  occupant:
                    chooser: gm
                    options:
                    - key: dnd2024:creatures/mummy
                    - key: dnd2024:creatures/mummy_lord
                    additional_allowed_creature_type: undead
                  sarcophagus_treasure:
                    chooser: gm
                    type: item_list
              - min: 96
                max: 100
                result:
                  spawn: giant_beanstalk
                  height_ft:
                    chooser: gm
                    type: number
                  top_destination:
                    chooser: gm
                    type: location
                    allows_other_planes: true
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_bag_of_beans
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - item.contents_dumped
      - item.bean_planted
      - time.minute
      - item.consumed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_bag_of_beans
    citations:
    - SRD 5.2.1 pp. 211; builtin/srd-5.2.1/magic_item/bag_of_beans.md
  dnd2024:items/bag_of_tricks:
    source_sha256: ace4021b6af19dee11e3214152de4eb70f682a0bf6ff7a9e79814ef40321b48c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/bag_of_tricks
        implementation: dnd2024:item_bag_of_tricks
        parameters:
          uses:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                expression: '3'
                scope: use
              cap: maximum
          activate:
            action: magic
            throw_maximum_ft: 20
            draw_and_throw: true
          variants:
            gray:
              roll:
                expression: 1d8
                scope: use
              rows:
              - min: 1
                max: 1
                result:
                  creature:
                    key: dnd2024:creatures/weasel
              - min: 2
                max: 2
                result:
                  creature:
                    key: dnd2024:creatures/giant_rat
              - min: 3
                max: 3
                result:
                  creature:
                    key: dnd2024:creatures/badger
              - min: 4
                max: 4
                result:
                  creature:
                    key: dnd2024:creatures/boar
              - min: 5
                max: 5
                result:
                  creature:
                    key: dnd2024:creatures/panther
              - min: 6
                max: 6
                result:
                  creature:
                    key: dnd2024:creatures/giant_badger
              - min: 7
                max: 7
                result:
                  creature:
                    key: dnd2024:creatures/dire_wolf
              - min: 8
                max: 8
                result:
                  creature:
                    key: dnd2024:creatures/giant_elk
            rust:
              roll:
                expression: 1d8
                scope: use
              rows:
              - min: 1
                max: 1
                result:
                  creature:
                    key: dnd2024:creatures/rat
              - min: 2
                max: 2
                result:
                  creature:
                    key: dnd2024:creatures/owl
              - min: 3
                max: 3
                result:
                  creature:
                    key: dnd2024:creatures/mastiff
              - min: 4
                max: 4
                result:
                  creature:
                    key: dnd2024:creatures/goat
              - min: 5
                max: 5
                result:
                  creature:
                    key: dnd2024:creatures/giant_goat
              - min: 6
                max: 6
                result:
                  creature:
                    key: dnd2024:creatures/giant_boar
              - min: 7
                max: 7
                result:
                  creature:
                    key: dnd2024:creatures/lion
              - min: 8
                max: 8
                result:
                  creature:
                    key: dnd2024:creatures/brown_bear
            tan:
              roll:
                expression: 1d8
                scope: use
              rows:
              - min: 1
                max: 1
                result:
                  creature:
                    key: dnd2024:creatures/jackal
              - min: 2
                max: 2
                result:
                  creature:
                    key: dnd2024:creatures/ape
              - min: 3
                max: 3
                result:
                  creature:
                    key: dnd2024:creatures/baboon
              - min: 4
                max: 4
                result:
                  creature:
                    key: dnd2024:creatures/axe_beak
              - min: 5
                max: 5
                result:
                  creature:
                    key: dnd2024:creatures/black_bear
              - min: 6
                max: 6
                result:
                  creature:
                    key: dnd2024:creatures/giant_weasel
              - min: 7
                max: 7
                result:
                  creature:
                    key: dnd2024:creatures/giant_hyena
              - min: 8
                max: 8
                result:
                  creature:
                    key: dnd2024:creatures/tiger
          summon:
            attitude: friendly_to_user_and_allies
            initiative: immediately_after_user_same_count
            end:
            - next_dawn
            - zero_hit_points
            command:
              action: bonus_action
              controls:
              - next_turn_movement
              - next_turn_action
            without_command: nature_appropriate_behavior
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_bag_of_tricks
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - action.bonus_pending
      - time.dawn
      - participant.hp_changed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_bag_of_tricks
    citations:
    - SRD 5.2.1 pp. 212; builtin/srd-5.2.1/magic_item/bag_of_tricks.md
  dnd2024:items/belt_of_dwarvenkind:
    source_sha256: 956c90432c01b23fe5c0d1f1a32d8a00ad1213fcb4f3aa4a99a62f5edb978281
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/belt_of_dwarvenkind
        implementation: dnd2024:item_belt_of_dwarvenkind
        parameters:
          equipment_state: worn
          language: dwarvish
          checks:
          - ability: charisma
            skill: persuasion
            target_ancestries:
            - dwarf
            - duergar
            advantage: true
          ability:
            constitution:
              increase: 2
              maximum: 20
          beard:
            requires: attuned
            event: dawn
            chance_percent: 50
            requires_can_grow_beard: true
            result_by_existing_beard:
              absent: full
              present: thicker
          non_dwarf_non_duergar:
            darkvision_ft: 60
            damage_resistance: poison
            save_advantage:
              avoid_or_end_condition: poisoned
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_belt_of_dwarvenkind
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - attunement.changed
      - time.dawn
      - check.pending
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_belt_of_dwarvenkind
    citations:
    - SRD 5.2.1 pp. 213; builtin/srd-5.2.1/magic_item/belt_of_dwarvenkind.md
  dnd2024:items/boots_of_elvenkind:
    source_sha256: 8e670af0298ccf71c55c4ac8730fdf74a9f0c5368cd2ae04ac31ac009aa9be08
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/boots_of_elvenkind
        implementation: dnd2024:item_boots_of_elvenkind
        parameters:
          equipment_state: worn
          steps_silent:
            all_surfaces: true
          check_advantage:
          - ability: dexterity
            skill: stealth
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_boots_of_elvenkind
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - movement.pending
      - check.pending
      capabilities:
      - inventory
      - effects
      - roll_resolution
    requires:
    - dnd2024:item_boots_of_elvenkind
    citations:
    - SRD 5.2.1 pp. 213; builtin/srd-5.2.1/magic_item/boots_of_elvenkind.md
  dnd2024:items/boots_of_striding_and_springing:
    source_sha256: 44bdbd83f364810f5e9691b14ef6eea24fc9f596eeff63fa5390cbd82bcd8319
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/boots_of_striding_and_springing
        implementation: dnd2024:item_boots_of_striding_and_springing
        parameters:
          equipment_state: worn
          speed_minimum_ft: 30
          ignore_speed_reductions_from:
          - excess_carried_weight
          - heavy_armor
          jump:
            uses_per_own_turn: 1
            maximum_ft: 30
            movement_cost_ft: 10
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_boots_of_striding_and_springing
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - movement.jump_pending
      - turn.started
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - spatial_state
    requires:
    - dnd2024:item_boots_of_striding_and_springing
    citations:
    - SRD 5.2.1 pp. 214; builtin/srd-5.2.1/magic_item/boots_of_striding_and_springing.md
  dnd2024:items/bracers_of_archery:
    source_sha256: 7f8af9134d53ea9b4c56d348145d616438334140f00860a6a3b6c5e0f45cd0c5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/bracers_of_archery
        implementation: dnd2024:item_bracers_of_archery
        parameters:
          equipment_state: worn
          weapon_proficiencies:
          - key: dnd2024:equipment/longbow
          - key: dnd2024:equipment/shortbow
          damage_bonus:
            weapons:
            - key: dnd2024:equipment/longbow
            - key: dnd2024:equipment/shortbow
            amount: 2
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_bracers_of_archery
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - attack.damage_pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - roll_resolution
    requires:
    - dnd2024:item_bracers_of_archery
    citations:
    - SRD 5.2.1 pp. 214; builtin/srd-5.2.1/magic_item/bracers_of_archery.md
  dnd2024:items/brooch_of_shielding:
    source_sha256: 62fcca1f28990e7d409a158c461c6d07810eb913fe8b3ef840180627917b9880
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/brooch_of_shielding
        implementation: dnd2024:item_brooch_of_shielding
        parameters:
          equipment_state: worn
          damage_resistance: force
          damage_immunity_from_spells:
          - key: dnd2024:spells/magic_missile
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_brooch_of_shielding
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - damage.pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - roll_resolution
    requires:
    - dnd2024:item_brooch_of_shielding
    citations:
    - SRD 5.2.1 pp. 214; builtin/srd-5.2.1/magic_item/brooch_of_shielding.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/cape_of_the_mountebank:
    source_sha256: 33dc08bb50b28f0e70bf12961bf42d236d18ce08380c1cc91040363e87c13f1d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/cape_of_the_mountebank
        implementation: dnd2024:item_cape_of_the_mountebank
        parameters:
          equipment_state: worn
          uses:
            maximum: 1
            initial: 1
            recovery:
              event: dawn
              amount:
                expression: '1'
                scope: use
              cap: maximum
          spell_grants:
          - spell:
              key: dnd2024:spells/dimension_door
            charge_cost: 1
            casting_time: magic
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          on_teleport:
            origin_space:
              obscurity: lightly_obscured
              duration:
                until: end_of_user_next_turn
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_cape_of_the_mountebank
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - spell.teleport_completed
      - turn.ended
      - time.dawn
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - spatial_state
    requires:
    - dnd2024:item_cape_of_the_mountebank
    citations:
    - SRD 5.2.1 pp. 215; builtin/srd-5.2.1/magic_item/cape_of_the_mountebank.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/chime_of_opening:
    source_sha256: ebe94a06c6064608bbf30f05a221ef2257cdca6e7f373d893bd5c277089c8ae7
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/chime_of_opening
        implementation: dnd2024:item_chime_of_opening
        parameters:
          length_ft: 1
          weight_lb: 1
          uses:
            maximum: 10
            initial: 10
          spell_grants:
          - spell:
              key: dnd2024:spells/knock
            charge_cost: 1
            casting_time: magic
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
            activation: strike_chime
            sound_override:
              kind: ringing_tone
              audible_ft: 300
          on_last_use:
            state: cracked_useless
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_chime_of_opening
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
    requires:
    - dnd2024:item_chime_of_opening
    citations:
    - SRD 5.2.1 pp. 215; builtin/srd-5.2.1/magic_item/chime_of_opening.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/cloak_of_displacement:
    source_sha256: 49353844c0c111ef374c23b02be090ee0da5d9912e44748dc9f55f267fd29700
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/cloak_of_displacement
        implementation: dnd2024:item_cloak_of_displacement
        parameters:
          equipment_state: worn
          attacks_against_wearer: disadvantage
          damage_taken:
            suppress_until: start_of_wearer_next_turn
          suppress_when:
            speed_ft: 0
          effect_type: illusion
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_cloak_of_displacement
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - damage.applied
      - turn.started
      - speed.changed
      - attack.roll_pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - roll_resolution
    requires:
    - dnd2024:item_cloak_of_displacement
    citations:
    - SRD 5.2.1 pp. 215; builtin/srd-5.2.1/magic_item/cloak_of_displacement.md
  dnd2024:items/cloak_of_protection:
    source_sha256: 5aac07589770f414a2d0767eeab5272583499e0be49eb42fa364f6f5aca1cf7b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/cloak_of_protection
        implementation: dnd2024:item_cloak_of_protection
        parameters:
          equipment_state: worn
          ac_bonus: 1
          saving_throw_bonus: 1
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_cloak_of_protection
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - saving_throw.pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
    requires:
    - dnd2024:item_cloak_of_protection
    citations:
    - SRD 5.2.1 pp. 216; builtin/srd-5.2.1/magic_item/cloak_of_protection.md
  dnd2024:items/crystal_ball:
    source_sha256: 8ffcbcd09dd7ed9e8ce0da59815312eace3f8db9960252407b7d05a1819ffdb5
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/crystal_ball
        implementation: dnd2024:item_crystal_ball
        parameters:
          equipment_state: touching
          spell_grants:
          - spell:
              key: dnd2024:spells/scrying
            charge_cost: 0
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
            save_dc: 17
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_crystal_ball
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_crystal_ball
    citations:
    - SRD 5.2.1 pp. 216; builtin/srd-5.2.1/magic_item/crystal_ball.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/crystal_ball_of_true_seeing:
    source_sha256: 8f214f31b1b251093608c8ecf661226fc1b328c6eee6e8f9d29ead36c761a6e0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/crystal_ball_of_true_seeing
        implementation: dnd2024:item_crystal_ball_of_true_seeing
        parameters:
          equipment_state: touching
          spell_grants:
          - spell:
              key: dnd2024:spells/scrying
            charge_cost: 0
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
            save_dc: 17
          while_scrying:
            truesight_ft: 120
            origin: spell_sensor
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_crystal_ball_of_true_seeing
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - perception.pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_crystal_ball_of_true_seeing
    citations:
    - SRD 5.2.1 pp. 216; builtin/srd-5.2.1/magic_item/crystal_ball_of_true_seeing.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/dagger_of_venom:
    source_sha256: 628e2ed7d7e6b96b2d34903205d0670c43559dc2c69be79584dd914022032007
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/dagger_of_venom
        implementation: dnd2024:item_dagger_of_venom
        parameters:
          attack_roll_bonus: 1
          damage_roll_bonus: 1
          scope: attacks_with_source_item
          poison_coating:
            action: bonus_action
            uses:
              maximum: 1
              initial: 1
              recovery:
                event: dawn
                amount:
                  expression: '1'
                  scope: use
                cap: maximum
            duration:
              amount: 1
              unit: minute
            ends_on: first_creature_hit
            on_first_creature_hit:
              save:
                ability: constitution
                dc: 15
                failure:
                  damage:
                    roll:
                      expression: 2d10
                      scope: attack
                    type: poison
                  condition: poisoned
                  duration:
                    amount: 1
                    unit: minute
                success:
                  effect: none
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_dagger_of_venom
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.bonus_pending
      - time.dawn
      - time.minute
      - attack.hit
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - participant_state
    requires:
    - dnd2024:item_dagger_of_venom
    citations:
    - SRD 5.2.1 pp. 216; builtin/srd-5.2.1/magic_item/dagger_of_venom.md
  dnd2024:items/deck_of_illusions:
    source_sha256: a6c33e8fd4b81bf975a9c422f82c473e5fa7b1b5813ef4a021a19caf108c8cb4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/deck_of_illusions
        implementation: dnd2024:item_deck_of_illusions
        parameters:
          full_deck:
            cards: 34
            specific_creatures: 32
            mirrored_cards: 2
          found_deck_missing:
            expression: 1d20-1
            scope: use
          activate:
            action: magic
            draw: random_only
            throw_range_ft: 30
            illusion:
              roll:
                expression: 1d100
                scope: use
              rows:
              - min: 1
                max: 3
                result:
                  appearance:
                    key: dnd2024:creatures/adult_red_dragon
              - min: 4
                max: 6
                result:
                  appearance:
                    key: dnd2024:creatures/archmage
              - min: 7
                max: 9
                result:
                  appearance:
                    key: dnd2024:creatures/assassin
              - min: 10
                max: 12
                result:
                  appearance:
                    key: dnd2024:creatures/bandit_captain
              - min: 13
                max: 15
                result:
                  appearance:
                    key: dnd2024:creatures/basilisk
              - min: 16
                max: 18
                result:
                  appearance:
                    key: dnd2024:creatures/berserker
              - min: 19
                max: 21
                result:
                  appearance:
                    key: dnd2024:creatures/bugbear_warrior
              - min: 22
                max: 24
                result:
                  appearance:
                    key: dnd2024:creatures/cloud_giant
              - min: 25
                max: 27
                result:
                  appearance:
                    key: dnd2024:creatures/druid
              - min: 28
                max: 30
                result:
                  appearance:
                    key: dnd2024:creatures/erinyes
              - min: 31
                max: 33
                result:
                  appearance:
                    key: dnd2024:creatures/ettin
              - min: 34
                max: 36
                result:
                  appearance:
                    key: dnd2024:creatures/fire_giant
              - min: 37
                max: 39
                result:
                  appearance:
                    key: dnd2024:creatures/frost_giant
              - min: 40
                max: 42
                result:
                  appearance:
                    key: dnd2024:creatures/gnoll_warrior
              - min: 43
                max: 45
                result:
                  appearance:
                    key: dnd2024:creatures/goblin_warrior
              - min: 46
                max: 48
                result:
                  appearance:
                    key: dnd2024:creatures/guardian_naga
              - min: 49
                max: 51
                result:
                  appearance:
                    key: dnd2024:creatures/hill_giant
              - min: 52
                max: 54
                result:
                  appearance:
                    key: dnd2024:creatures/hobgoblin_warrior
              - min: 55
                max: 57
                result:
                  appearance:
                    key: dnd2024:creatures/incubus
              - min: 58
                max: 60
                result:
                  appearance:
                    key: dnd2024:creatures/iron_golem
              - min: 61
                max: 63
                result:
                  appearance:
                    key: dnd2024:creatures/knight
              - min: 64
                max: 66
                result:
                  appearance:
                    key: dnd2024:creatures/kobold_warrior
              - min: 67
                max: 69
                result:
                  appearance:
                    key: dnd2024:creatures/lich
              - min: 70
                max: 72
                result:
                  appearance:
                    key: dnd2024:creatures/medusa
              - min: 73
                max: 75
                result:
                  appearance:
                    key: dnd2024:creatures/night_hag
              - min: 76
                max: 78
                result:
                  appearance:
                    key: dnd2024:creatures/ogre
              - min: 79
                max: 81
                result:
                  appearance:
                    key: dnd2024:creatures/oni
              - min: 82
                max: 84
                result:
                  appearance:
                    key: dnd2024:creatures/priest
              - min: 85
                max: 87
                result:
                  appearance:
                    key: dnd2024:creatures/succubus
              - min: 88
                max: 90
                result:
                  appearance:
                    key: dnd2024:creatures/troll
              - min: 91
                max: 93
                result:
                  appearance:
                    key: dnd2024:creatures/warrior_veteran
              - min: 94
                max: 96
                result:
                  appearance:
                    key: dnd2024:creatures/wyvern
              - min: 97
                max: 100
                result:
                  appearance: card_drawer
          illusion:
            behavior: imitate_creature
            can_harm: false
            move:
              action: magic
              user_maximum_distance_ft: 120
              user_must_see: true
              destination_maximum_distance_from_card_ft: 30
            physical_interaction:
              objects_pass_through: true
              reveals_false: true
            inspect:
              action: study
              check:
                ability: intelligence
                skill: investigation
                dc: 15
              success: identify_illusion
            ends_on:
            - card_moved
            - dispelled
            dispel_examples:
            - key: dnd2024:spells/dispel_magic
            on_end:
              card_image: disappears
              card_reusable: false
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_deck_of_illusions
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - action.study_pending
      - object.interacted
      - item.moved
      - effect.dispelled
      capabilities:
      - inventory
      - effects
      - spellcasting
      - roll_resolution
      - participant_state
    requires:
    - dnd2024:item_deck_of_illusions
    citations:
    - SRD 5.2.1 pp. 217, 218; builtin/srd-5.2.1/magic_item/deck_of_illusions.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/dimensional_shackles:
    source_sha256: 9ef2ea79ab6ef02970dd075cac38c6cdfa086138cf3b32b36c245280a7c35172
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/dimensional_shackles
        implementation: dnd2024:item_dimensional_shackles
        parameters:
          apply:
            action: utilize
            target_condition: incapacitated
            sizes:
            - small
            - medium
            - large
            designate_removers: true
          bound:
            prevent_movement:
            - teleportation
            - planar_travel
            - other_extradimensional
            allow: passing_through_interdimensional_portal
          remove:
            action: utilize
            allowed_actors:
            - applier
            - designated_creatures
          break_free:
            interval:
              amount: 30
              unit: day
            actor: bound_creature
            check:
              ability: strength
              skill: athletics
              dc: 30
            success:
              freed: true
              item_destroyed: true
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_dimensional_shackles
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.utilize_pending
      - check.pending
      - time.day
      - movement.planar_pending
      capabilities:
      - inventory
      - effects
      - roll_resolution
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_dimensional_shackles
    citations:
    - SRD 5.2.1 pp. 218; builtin/srd-5.2.1/magic_item/dimensional_shackles.md
  dnd2024:items/dragon_slayer:
    source_sha256: c99f71a19005098278dc16bafafbd895e1d3e0a3c25f8779a803b674220d41a1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/dragon_slayer
        implementation: dnd2024:item_dragon_slayer
        parameters:
          attack_roll_bonus: 1
          damage_roll_bonus: 1
          scope: attacks_with_source_item
          on_hit:
            target_creature_type: dragon
            extra_damage:
              roll:
                expression: 3d6
                scope: attack
              type: weapon_damage_type
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_dragon_slayer
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - attack.roll_pending
      - attack.hit
      capabilities:
      - inventory
      - effects
      - roll_resolution
      - participant_state
    requires:
    - dnd2024:item_dragon_slayer
    citations:
    - SRD 5.2.1 pp. 219; builtin/srd-5.2.1/magic_item/dragon_slayer.md
  dnd2024:items/dust_of_sneezing_and_choking:
    source_sha256: b583ce91cba7b95eec727b1dbb6757260a74a39c3e7f91af551462e1a36b7fd4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/dust_of_sneezing_and_choking
        implementation: dnd2024:item_dust_of_sneezing_and_choking
        parameters:
          uses: 1
          identify_false_result:
            key: dnd2024:items/dust_of_disappearance
          appearance_matches:
            key: dnd2024:items/dust_of_disappearance
          activation:
            action: utilize
            consume: true
            area:
              shape: emanation
              radius_ft: 30
              origin: user
              include_user: true
            save:
              ability: constitution
              dc: 15
              failure:
                condition: incapacitated
                suffocating: true
              success:
                effect: none
              automatic_success_types:
              - construct
              - elemental
              - ooze
              - plant
              - undead
          repeat_save:
            event: target_turn_end
            ability: constitution
            dc: 15
            success: end_effect
          end_on_spell_targeting:
          - key: dnd2024:spells/lesser_restoration
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_dust_of_sneezing_and_choking
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.utilize_pending
      - turn.ended
      - spell.cast_completed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
      - spatial_state
    requires:
    - dnd2024:item_dust_of_sneezing_and_choking
    citations:
    - SRD 5.2.1 pp. 219; builtin/srd-5.2.1/magic_item/dust_of_sneezing_and_choking.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/efficient_quiver:
    source_sha256: 9dffda7633e6da94c85bd3d8fa3c2309c7516e4f73e6ee1e76b3d46716a007b4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/efficient_quiver
        implementation: dnd2024:item_efficient_quiver
        parameters:
          extradimensional_compartments:
          - capacity: 60
            item_shapes:
            - arrow
            - bolt
            - similar
          - capacity: 18
            item_shapes:
            - javelin
            - similar
          - capacity: 6
            item_shapes:
            - bow
            - quarterstaff
            - spear
            - similar_long_object
          maximum_total_weight_lb: 2
          retrieve_economy: normal_quiver_or_scabbard
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_efficient_quiver
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - inventory.insert_pending
      - inventory.retrieve_pending
      capabilities:
      - inventory
      - effects
    requires:
    - dnd2024:item_efficient_quiver
    citations:
    - SRD 5.2.1 pp. 219; builtin/srd-5.2.1/magic_item/efficient_quiver.md
  dnd2024:items/elixir_of_health:
    source_sha256: e42f2de0d4cfe00f60be1081618bf167e16d0037ca86717ab1e539bc31628d0b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/elixir_of_health
        implementation: dnd2024:item_elixir_of_health
        parameters:
          consume: drink
          cure: all_magical_contagions
          end_conditions:
          - blinded
          - deafened
          - paralyzed
          - poisoned
          consume_action: bonus_action
          administer_action: bonus_action
          effect_target: drinker
          consumed_quantity: 1
          effect_starts: immediately_after_consumption
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_potions
    contract:
      id: dnd2024:item_elixir_of_health
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - item.consumed
      capabilities:
      - inventory
      - effects
    requires:
    - dnd2024:item_elixir_of_health
    citations:
    - SRD 5.2.1 pp. 220; builtin/srd-5.2.1/magic_item/elixir_of_health.md
  dnd2024:items/eversmoking_bottle:
    source_sha256: 028a188dbf9b8c9d30ebd1acd5a86d7f4a707c43cbd048e6087f851583d0a1a6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/eversmoking_bottle
        implementation: dnd2024:item_eversmoking_bottle
        parameters:
          toggle_action: magic
          open:
            cloud:
              shape: emanation
              origin: bottle
              radius_ft: 60
              obscurity: heavily_obscured
              growth_ft_per_minute: 10
              maximum_radius_ft: 120
          closed:
            cloud_location: fixed_at_closing
            disperse_after:
              amount: 10
              unit: minute
          strong_wind:
            disperse_after:
              amount: 1
              unit: minute
            examples:
            - key: dnd2024:spells/gust_of_wind
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_eversmoking_bottle
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - time.minute
      - wind.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - spatial_state
    requires:
    - dnd2024:item_eversmoking_bottle
    citations:
    - SRD 5.2.1 pp. 220, 221; builtin/srd-5.2.1/magic_item/eversmoking_bottle.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/eyes_of_the_eagle:
    source_sha256: 6ef1c0aae7314dd064edefe637c638035d38f803f905207545954f8fc8cbbdff
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/eyes_of_the_eagle
        implementation: dnd2024:item_eyes_of_the_eagle
        parameters:
          equipment_state: worn_over_eyes
          check_advantage:
            ability: wisdom
            skill: perception
            sense: sight
          clear_visibility:
            minimum_detail_object_width_ft: 2
            distance_limit: extremely_distant
            distance_adjudication:
              chooser: gm
              type: number
              scope: visibility_distance
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_eyes_of_the_eagle
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - check.pending
      - perception.pending
      capabilities:
      - inventory
      - effects
      - roll_resolution
    requires:
    - dnd2024:item_eyes_of_the_eagle
    citations:
    - SRD 5.2.1 pp. 221; builtin/srd-5.2.1/magic_item/eyes_of_the_eagle.md
  dnd2024:items/flame_tongue:
    source_sha256: a0a86a4a12352ad4ce0ef671eeae07189b6b76819c524baa01ef029d495ff5a1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/flame_tongue
        implementation: dnd2024:item_flame_tongue
        parameters:
          equipment_state: held
          ignite:
            action: bonus_action
            command_word: true
          ablaze:
            bright_radius_ft: 40
            dim_additional_ft: 40
            on_hit:
              damage:
                roll:
                  expression: 2d6
                  scope: attack
                type: fire
          extinguish:
            action: bonus_action
            command_word: true
          ends_on:
          - drop
          - stow
          - sheathe
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_flame_tongue
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.bonus_pending
      - equipment.changed
      - attack.hit
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - roll_resolution
      - spatial_state
    requires:
    - dnd2024:item_flame_tongue
    citations:
    - SRD 5.2.1 pp. 223; builtin/srd-5.2.1/magic_item/flame_tongue.md
  dnd2024:items/gauntlets_of_ogre_power:
    source_sha256: 5d4abb8e8c07ef33e14519830927078fdcee8823ae3b89a163e37db48682792e
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/gauntlets_of_ogre_power
        implementation: dnd2024:item_gauntlets_of_ogre_power
        parameters:
          equipment_state: worn
          strength:
            set: 19
            only_if_unmodified_less_than: 19
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_gauntlets_of_ogre_power
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - ability.changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
    requires:
    - dnd2024:item_gauntlets_of_ogre_power
    citations:
    - SRD 5.2.1 pp. 223; builtin/srd-5.2.1/magic_item/gauntlets_of_ogre_power.md
  dnd2024:items/giant_slayer:
    source_sha256: 5a4ddf4926b05f6bfa20ae8a17261837c897596ca557e6f27ed410ea3bc2611a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/giant_slayer
        implementation: dnd2024:item_giant_slayer
        parameters:
          attack_roll_bonus: 1
          damage_roll_bonus: 1
          scope: attacks_with_source_item
          on_hit:
            target_creature_type: giant
            extra_damage:
              roll:
                expression: 2d6
                scope: attack
              type: weapon_damage_type
            save:
              ability: strength
              dc: 15
              failure:
                condition: prone
              success:
                effect: none
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_giant_slayer
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - attack.roll_pending
      - attack.hit
      capabilities:
      - inventory
      - effects
      - roll_resolution
      - participant_state
    requires:
    - dnd2024:item_giant_slayer
    citations:
    - SRD 5.2.1 pp. 223; builtin/srd-5.2.1/magic_item/giant_slayer.md
  dnd2024:items/gloves_of_swimming_and_climbing:
    source_sha256: da869b547cb5e3d794adcf44c711fadbbb41438d6299877be59d78baffc97b36
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/gloves_of_swimming_and_climbing
        implementation: dnd2024:item_gloves_of_swimming_and_climbing
        parameters:
          equipment_state: worn
          climb_speed: speed
          swim_speed: speed
          check_bonus:
            ability: strength
            skill: athletics
            tasks:
            - climb
            - swim
            amount: 5
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_gloves_of_swimming_and_climbing
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - speed.changed
      - check.pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - roll_resolution
    requires:
    - dnd2024:item_gloves_of_swimming_and_climbing
    citations:
    - SRD 5.2.1 pp. 224; builtin/srd-5.2.1/magic_item/gloves_of_swimming_and_climbing.md
  dnd2024:items/hammer_of_thunderbolts:
    source_sha256: 0caee1b0d4be9bbc318035835a737e47cecc1239d32ad7b2c001ccfd8b1b20d2
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/hammer_of_thunderbolts
        implementation: dnd2024:item_hammer_of_thunderbolts
        parameters:
          attack_roll_bonus: 1
          damage_roll_bonus: 1
          scope: attacks_with_source_item
          charges:
            maximum: 5
            initial: 5
            recovery:
              event: dawn
              amount:
                expression: 1d4+1
                scope: use
              cap: maximum
          throw:
            charge_cost: 1
            economy: ranged_weapon_attack
            property: thrown
            normal_range_ft: 20
            long_range_ft: 60
            on_hit:
              audible_ft: 300
              targets:
                origin: hit_target
                radius_ft: 30
                include_hit_target: true
                exclude_user: true
              save:
                ability: constitution
                dc: 17
                failure:
                  condition: stunned
                  duration:
                    until: end_of_user_next_turn
                success:
                  effect: none
            after_hit_or_miss: returns_to_hand
          giants_bane:
            requires:
              weapon_attuned: true
              worn_and_attuned_any:
              - key: dnd2024:items/belt_of_giant_strength
              - key: dnd2024:items/gauntlets_of_ogre_power
            on_natural_20:
              target_type: giant
              save:
                ability: constitution
                dc: 17
                failure:
                  dead: true
                success:
                  effect: none
            other_item_strength_grant:
              increase: 4
              maximum: 30
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_hammer_of_thunderbolts
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - attack.roll_pending
      - attack.hit
      - attack.miss
      - time.dawn
      - equipment.changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - spatial_state
    requires:
    - dnd2024:item_hammer_of_thunderbolts
    citations:
    - SRD 5.2.1 pp. 224; builtin/srd-5.2.1/magic_item/hammer_of_thunderbolts.md
  dnd2024:items/hat_of_many_spells:
    source_sha256: e6e54a015ebaf3f766c1eb6614dcc95794b536e61f4efa21a07780d377c453b9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/hat_of_many_spells
        implementation: dnd2024:item_hat_of_many_spells
        parameters:
          equipment_state: held
          focus:
            spell_class: wizard
            added_somatic_component: reach_into_hat_and_pull_spell_out
          unknown_spell:
            selection:
              known: false
              spell_list: wizard
              minimum_level: 1
              level_must_be_castable: true
              maximum_material_component_cost_gp: 1000
            slot_cost: selected_spell_level
            cost_timing: before_arcana_check
            check:
              ability: intelligence
              skill: arcana
              dc:
                op: add
                args:
                - 10
                - binding: selected_spell_level
            success:
              cast_selected_spell: true
              casting_time: spell_default
              lockout_until:
              - short_rest_completed
              - long_rest_completed
              slot_cost: already_paid_selected_spell_level
              components: []
            failure:
              cast_selected_spell: false
              lockout: false
              table:
                roll:
                  expression: 1d100
                  scope: use
                rows:
                - min: 1
                  max: 50
                  result:
                    cast_random_spell:
                      roll:
                        expression: 1d10
                        scope: use
                      rows:
                      - min: 1
                        max: 1
                        result:
                          spell:
                            key: dnd2024:spells/enlarge_reduce
                          charge_cost: 0
                          casting_time: spell_default
                          method: item
                          slot_cost: 0
                          components: []
                          level: lowest_possible
                          range: spell_default
                          duration: spell_default
                          concentration: spell_default
                          mode: enlarge
                      - min: 2
                        max: 2
                        result:
                          spell:
                            key: dnd2024:spells/enlarge_reduce
                          charge_cost: 0
                          casting_time: spell_default
                          method: item
                          slot_cost: 0
                          components: []
                          level: lowest_possible
                          range: spell_default
                          duration: spell_default
                          concentration: spell_default
                          mode: reduce
                      - min: 3
                        max: 3
                        result:
                          spell:
                            key: dnd2024:spells/faerie_fire
                          charge_cost: 0
                          casting_time: spell_default
                          method: item
                          slot_cost: 0
                          components: []
                          level: lowest_possible
                          range: spell_default
                          duration: spell_default
                          concentration: spell_default
                      - min: 4
                        max: 4
                        result:
                          spell:
                            key: dnd2024:spells/fireball
                          charge_cost: 0
                          casting_time: spell_default
                          method: item
                          slot_cost: 0
                          components: []
                          level: lowest_possible
                          range: spell_default
                          duration: spell_default
                          concentration: spell_default
                      - min: 5
                        max: 5
                        result:
                          spell:
                            key: dnd2024:spells/gust_of_wind
                          charge_cost: 0
                          casting_time: spell_default
                          method: item
                          slot_cost: 0
                          components: []
                          level: lowest_possible
                          range: spell_default
                          duration: spell_default
                          concentration: spell_default
                      - min: 6
                        max: 6
                        result:
                          spell:
                            key: dnd2024:spells/invisibility
                          charge_cost: 0
                          casting_time: spell_default
                          method: item
                          slot_cost: 0
                          components: []
                          level: lowest_possible
                          range: spell_default
                          duration: spell_default
                          concentration: spell_default
                          target: self
                      - min: 7
                        max: 7
                        result:
                          spell:
                            key: dnd2024:spells/lightning_bolt
                          charge_cost: 0
                          casting_time: spell_default
                          method: item
                          slot_cost: 0
                          components: []
                          level: lowest_possible
                          range: spell_default
                          duration: spell_default
                          concentration: spell_default
                      - min: 8
                        max: 8
                        result:
                          spell:
                            key: dnd2024:spells/phantasmal_force
                          charge_cost: 0
                          casting_time: spell_default
                          method: item
                          slot_cost: 0
                          components: []
                          level: lowest_possible
                          range: spell_default
                          duration: spell_default
                          concentration: spell_default
                      - min: 9
                        max: 9
                        result:
                          spell:
                            key: dnd2024:spells/polymorph
                          charge_cost: 0
                          casting_time: spell_default
                          method: item
                          slot_cost: 0
                          components: []
                          level: lowest_possible
                          range: spell_default
                          duration: spell_default
                          concentration: spell_default
                      - min: 10
                        max: 10
                        result:
                          spell:
                            key: dnd2024:spells/stinking_cloud
                          charge_cost: 0
                          casting_time: spell_default
                          method: item
                          slot_cost: 0
                          components: []
                          level: lowest_possible
                          range: spell_default
                          duration: spell_default
                          concentration: spell_default
                - min: 51
                  max: 55
                  result:
                    condition: stunned
                    duration:
                      until: end_of_user_next_turn
                - min: 56
                  max: 60
                  result:
                    spawn: harmless_butterfly_swarm
                    shape: cube
                    size_ft: 10
                    range_ft: 30
                    duration:
                      amount: 1
                      unit: minute
                - min: 61
                  max: 65
                  result:
                    nonmagical_item:
                      roll:
                        expression: 1d4
                        scope: use
                      rows:
                      - min: 1
                        max: 1
                        result:
                          item:
                            key: dnd2024:equipment/acid_25_gp
                      - min: 2
                        max: 2
                        result:
                          item:
                            key: dnd2024:equipment/alchemist_s_fire_50_gp
                      - min: 3
                        max: 3
                        result:
                          item:
                            key: dnd2024:equipment/crowbar_2_gp
                      - min: 4
                        max: 4
                        result:
                          item:
                            key: dnd2024:equipment/torch_1_cp
                          lit: true
                - min: 66
                  max: 70
                  result:
                    condition: poisoned
                    duration:
                      amount: 1
                      unit: hour
                - min: 71
                  max: 75
                  result:
                    condition: petrified
                    duration:
                      until: end_of_user_next_turn
                - min: 76
                  max: 80
                  result:
                    nonmagical_item:
                      roll:
                        expression: 1d4
                        scope: use
                      rows:
                      - min: 1
                        max: 1
                        result:
                          item:
                            key: dnd2024:equipment/dagger
                      - min: 2
                        max: 2
                        result:
                          items:
                          - key: dnd2024:equipment/rope_1_gp
                          - key: dnd2024:equipment/grappling_hook_2_gp
                          attachment: hook_tied_to_rope_end
                      - min: 3
                        max: 3
                        result:
                          item:
                            key: dnd2024:equipment/caltrops_1_gp
                      - min: 4
                        max: 4
                        result:
                          item_kind: gem
                          value_gp: 50
                - min: 81
                  max: 85
                  result:
                    spawn:
                      roll:
                        expression: 1d4
                        scope: use
                      rows:
                      - min: 1
                        max: 1
                        result:
                          creature:
                            key: dnd2024:creatures/camel
                      - min: 2
                        max: 2
                        result:
                          creature:
                            key: dnd2024:creatures/constrictor_snake
                      - min: 3
                        max: 3
                        result:
                          creature:
                            key: dnd2024:creatures/elephant
                      - min: 4
                        max: 4
                        result:
                          creature:
                            key: dnd2024:creatures/mule
                    location: closest_unoccupied_space
                    controlled: false
                    behavior: normal
                    duration:
                      amount: 1
                      unit: hour
                    ends:
                    - duration_expired
                    - zero_hit_points
                - min: 86
                  max: 90
                  result:
                    spawn:
                      key: dnd2024:creatures/swarm_of_bats
                    location: user_space
                    attitude: hostile
                    behavior: attack_user
                - min: 91
                  max: 95
                  result:
                    spawn: portal
                    orientation: vertical
                    diameter_ft: 10
                    two_way: true
                    location:
                      unoccupied: true
                      range_ft: 30
                    duration:
                      until: end_of_user_next_turn
                    destination:
                      chooser: gm
                      type: location
                      different_plane: true
                - min: 96
                  max: 100
                  result:
                    spawn_magic_item:
                      chooser: gm
                      rarity:
                        roll:
                          expression: 1d6
                          scope: use
                        rows:
                        - min: 1
                          max: 3
                          result:
                            rarity: common
                        - min: 4
                          max: 5
                          result:
                            rarity: uncommon
                        - min: 6
                          max: 6
                          result:
                            rarity: rare
                    disappear_after:
                      amount: 1
                      unit: hour
                    end_early:
                    - consumed
                    - destroyed
          all_hat_spells:
            save_dc: user_spell_save_dc
            attack_bonus: user_spell_attack_bonus
            added_somatic_component: reach_into_hat_and_pull_spell_out
          attunement:
            required: true
            eligible_classes:
            - wizard
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_hat_of_many_spells
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
        selected_spell_level:
          type: integer
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - check.completed
      - rest.completed
      - turn.ended
      - time.elapsed
      - participant.hp_changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_hat_of_many_spells
    citations:
    - SRD 5.2.1 pp. 224, 225; builtin/srd-5.2.1/magic_item/hat_of_many_spells.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/helm_of_comprehending_languages:
    source_sha256: dcb8a27edc63f7d2caac41fc643dee9f7d70aec042d8556babeda4edd63d8b87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/helm_of_comprehending_languages
        implementation: dnd2024:item_helm_of_comprehending_languages
        parameters:
          equipment_state: worn
          spell_grants:
          - spell:
              key: dnd2024:spells/comprehend_languages
            charge_cost: 0
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_helm_of_comprehending_languages
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
    requires:
    - dnd2024:item_helm_of_comprehending_languages
    citations:
    - SRD 5.2.1 pp. 226; builtin/srd-5.2.1/magic_item/helm_of_comprehending_languages.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/holy_avenger:
    source_sha256: fbdb0234a33666af019d1011299a46ce6b83ce3c5873217382f569a0b1c04783
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/holy_avenger
        implementation: dnd2024:item_holy_avenger
        parameters:
          attack_roll_bonus: 3
          damage_roll_bonus: 3
          scope: attacks_with_source_item
          on_hit:
            target_types:
            - fiend
            - undead
            damage:
              roll:
                expression: 2d10
                scope: attack
              type: radiant
          aura:
            equipment_state: held_drawn
            shape: emanation
            origin: user
            radius_ft: 10
            radius_override:
              paladin_level_minimum: 17
              radius_ft: 30
            targets:
            - self
            - friendly_creatures
            save_advantage_against:
            - spells
            - other_magical_effects
          attunement:
            required: true
            eligible_classes:
            - paladin
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_holy_avenger
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - attack.roll_pending
      - attack.hit
      - equipment.changed
      - position.changed
      - saving_throw.pending
      - class.level_changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - roll_resolution
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_holy_avenger
    citations:
    - SRD 5.2.1 pp. 226; builtin/srd-5.2.1/magic_item/holy_avenger.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/horseshoes_of_a_zephyr:
    source_sha256: d41a54ecb6c89f751af6caa82f80c6569d6bbfdbcf9834ce66cc5fa487690a19
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/horseshoes_of_a_zephyr
        implementation: dnd2024:item_horseshoes_of_a_zephyr
        parameters:
          set_size: 4
          affix:
            action: magic
            target: hoof
            eligible_creatures:
            - horse
            - similar_creature
          remove:
            action: magic
          while_all_four_affixed:
            float_height_inches: 4
            normal_movement: true
            surface_support_required: false
            surface_examples:
            - water
            - lava
            leaves_tracks: false
            ignore_difficult_terrain: true
            daily_travel_without_extended_travel_exhaustion_hours: 12
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_horseshoes_of_a_zephyr
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - movement.pending
      - travel.elapsed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_horseshoes_of_a_zephyr
    citations:
    - SRD 5.2.1 pp. 226; builtin/srd-5.2.1/magic_item/horseshoes_of_a_zephyr.md
  dnd2024:items/instant_fortress:
    source_sha256: 4c151894d16e00cc90c69d9e4c73027c7cdf99ae2c0cd9f249f6ee036c16723a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/instant_fortress
        implementation: dnd2024:item_instant_fortress
        parameters:
          statuette_size_inches: 1
          material: adamantine
          expand:
            action: magic
            place_on_ground: true
            command_word: true
            displace:
              creatures: unoccupied_adjacent_outside_space
              objects: outside_tower
              exclude_worn_carried_objects: true
          shrink:
            command_word: true
            requires: tower_empty
            preserve_damage: true
          tower:
            width_ft: 20
            length_ft: 20
            height_ft: 30
            arrow_slits: all_sides
            battlement: true
            floors: 2
            connector:
              chooser: user
              options:
              - ladder
              - staircase
              - ramp
            roof_access: trapdoor
            ground_door:
              faces: user_at_creation
              open:
                actor: attuned_user
                action: bonus_action
                command: true
              immune_to:
              - key: dnd2024:spells/knock
              immune_to_similar_opening_magic: true
            cannot_tip_over: true
            component_statistics:
              components:
              - roof
              - door
              - walls
              ac: 20
              hp: 100
              immunity:
              - bludgeoning
              - piercing
              - slashing
              immunity_exception: siege_equipment
              resistance: all_other_damage
          repair:
            only_spell:
              key: dnd2024:spells/wish
            counts_as: replicate_spell_level_8_or_lower
            hp_restored: all
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_instant_fortress
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - action.bonus_pending
      - command.issued
      - spell.cast_completed
      - damage.pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - roll_resolution
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_instant_fortress
    citations:
    - SRD 5.2.1 pp. 227; builtin/srd-5.2.1/magic_item/instant_fortress.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/iron_flask:
    source_sha256: c4054db29d066342cbe4bcf9df40b5101286da0e3ccaca46f3fb83e7f77c8096
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/iron_flask
        implementation: dnd2024:item_iron_flask
        parameters:
          equipment_state: held
          capture:
            action: magic
            range_ft: 60
            must_see: true
            requires_flask_empty: true
            target_native_plane_must_differ_from_current: true
            save:
              ability: wisdom
              dc: 17
              failure:
                trapped_in_flask: true
              success:
                effect: none
            save_advantage_if: previously_trapped_by_same_flask
          capacity_creatures: 1
          trapped:
            duration: until_released
            age: false
            needs:
            - no_breathing
            - no_food
            - no_drink
          release:
            action: magic
            remove_stopper: true
            obeys_user_commands_duration:
              amount: 1
              unit: hour
            understands_commands_any_language: true
            no_command_or_likely_death_or_imprisonment:
              self_defense: true
              other_actions: false
            after_duration: normal_disposition_and_alignment
          identify:
            reveals_occupied: true
            reveals_creature_type: false
          determine_type: open_flask
          initial_occupant:
            chooser: gm
            optional: true
            type: definition
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_iron_flask
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - time.elapsed
      - spell.cast_completed
      - command.issued
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_iron_flask
    citations:
    - SRD 5.2.1 pp. 228; builtin/srd-5.2.1/magic_item/iron_flask.md
  dnd2024:items/luck_blade:
    source_sha256: 93acd0a27bdc2bf1fdaa536ea3f01bc223f5c3ebed21b797250d8fb329415332
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/luck_blade
        implementation: dnd2024:item_luck_blade
        parameters:
          attack_roll_bonus: 1
          damage_roll_bonus: 1
          scope: attacks_with_source_item
          on_person:
            saving_throw_bonus: 1
            luck:
              action: none
              requires_not_condition: incapacitated
              trigger: failed_d20_test
              reroll: true
              must_use_second: true
              uses:
                maximum: 1
                initial: 1
                recovery:
                  event: dawn
                  amount:
                    expression: '1'
                    scope: use
                  cap: maximum
          wish:
            equipment_state: held
            charges:
              initial:
                expression: 1d3
                scope: use
              recovery: null
            spell_grant:
              spell:
                key: dnd2024:spells/wish
              charge_cost: 1
              casting_time: spell_default
              method: item
              slot_cost: 0
              components: []
              level: lowest_possible
              range: spell_default
              duration: spell_default
              concentration: spell_default
            uses_per_dawn: 1
            on_zero_charges: lose_wish_property
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_luck_blade
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - attack.roll_pending
      - saving_throw.pending
      - d20_test.failed
      - spell.cast_pending
      - time.dawn
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_luck_blade
    citations:
    - SRD 5.2.1 pp. 229; builtin/srd-5.2.1/magic_item/luck_blade.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/mace_of_terror:
    source_sha256: e167eb9232e40273d85193b762bf709fb9e72338464a1aa925a09c8270e09f9b
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/mace_of_terror
        implementation: dnd2024:item_mace_of_terror
        parameters:
          equipment_state: held
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                expression: 1d3
                scope: use
              cap: maximum
          terror:
            action: magic
            charge_cost: 1
            targets:
              chooser: user
              range_ft: 30
            save:
              ability: wisdom
              dc: 15
              failure:
                condition: frightened
                duration:
                  amount: 1
                  unit: minute
              success:
                effect: none
            while_frightened:
              movement: as_far_away_from_user_as_possible
              opportunity_attacks: false
              actions_allowed:
              - dash
              - escape_movement_preventing_effect
              if_no_movement_possible: dodge_allowed
            repeat_save:
              event: target_turn_end
              ability: wisdom
              dc: 15
              success: end_effect
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_mace_of_terror
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - turn.started
      - turn.ended
      - time.dawn
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - spatial_state
    requires:
    - dnd2024:item_mace_of_terror
    citations:
    - SRD 5.2.1 pp. 229; builtin/srd-5.2.1/magic_item/mace_of_terror.md
  dnd2024:items/manual_of_gainful_exercise:
    source_sha256: 23212510d2e01098da87d2c92935b972063b1ee8ca8f31f84ba6997e01539092
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/manual_of_gainful_exercise
        implementation: dnd2024:item_manual_of_gainful_exercise
        parameters:
          study:
            hours: 48
            within_days: 6
            practice_required: true
          completion:
            ability: strength
            increase: 2
            maximum: 30
            duration: permanent
            item_magical: false
          recovery:
            after:
              amount: 100
              unit: year
            item_magical: true
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_manual_of_gainful_exercise
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - study.completed
      - time.elapsed
      capabilities:
      - inventory
      - effects
      - resources_and_time
    requires:
    - dnd2024:item_manual_of_gainful_exercise
    citations:
    - SRD 5.2.1 pp. 229; builtin/srd-5.2.1/magic_item/manual_of_gainful_exercise.md
  dnd2024:items/marvelous_pigments:
    source_sha256: 470fa0df80ed449942959d11207ef599647963d150e84fc1c5a0e6795c24a196
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/marvelous_pigments
        implementation: dnd2024:item_marvelous_pigments
        parameters:
          box:
            initial_pots:
              expression: 1d4
              scope: use
            brush: true
            total_weight_lb: 1
          paint:
            cost_pots: 1
            brush_required: true
            duration:
              amount: 10
              unit: minute
            concentration: true
            user_must_remain_in_area: true
            area:
              shape: cube
              size_ft: 20
            elements:
              kinds:
              - three_dimensional_object
              - terrain_feature
              count: any
              all_extents_inside_area: true
            interrupted:
              painted_elements_vanish: true
              pot_wasted: true
            completion:
              elements_become_real: true
              door_connects_to_existing_beyond: true
              pit_depth_within_cube: true
            value_limits:
              per_object_gp: 25
              total_gp: 500
              exceeding:
                appearance: authentic
                material: worthless
                close_inspection_reveals: true
            painted_energy:
              dissipates_on_completion: true
              damage: 0
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_marvelous_pigments
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - painting.started
      - concentration.ended
      - position.changed
      - time.elapsed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - spatial_state
    requires:
    - dnd2024:item_marvelous_pigments
    citations:
    - SRD 5.2.1 pp. 230; builtin/srd-5.2.1/magic_item/marvelous_pigments.md
  dnd2024:items/mithral_armor:
    source_sha256: 409e915b374d88ac813b9ed8348f74b8cd534a3b53d6e48c10cdb13198dbc4ea
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/mithral_armor
        implementation: dnd2024:item_mithral_armor
        parameters:
          wear_under_normal_clothes: true
          remove_stealth_disadvantage: true
          remove_strength_requirement: true
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_armor
    contract:
      id: dnd2024:item_mithral_armor
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - check.pending
      capabilities:
      - inventory
      - effects
    requires:
    - dnd2024:item_mithral_armor
    citations:
    - SRD 5.2.1 pp. 231; builtin/srd-5.2.1/magic_item/mithral_armor.md
  dnd2024:items/necklace_of_fireballs:
    source_sha256: ea26bce0d7f5de65776d7382ad00d8ebde5d10f77fc0e77bab1e4315f7fda743
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/necklace_of_fireballs
        implementation: dnd2024:item_necklace_of_fireballs
        parameters:
          beads:
            initial:
              expression: 1d6+3
              scope: use
          activate:
            action: magic
            throw_maximum_ft: 60
            beads_expended:
              minimum: 1
              maximum: remaining
              whole_necklace_allowed: true
            detonate_at: trajectory_end
            spell:
              key: dnd2024:spells/fireball
            spell_level: 3
            save_dc: 15
            damage_override:
              dice:
                count:
                  op: min
                  args:
                  - 12
                  - op: add
                    args:
                    - 8
                    - op: subtract
                      args:
                      - binding: beads_expended
                      - 1
                sides: 6
                add: 0
              type: fire
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_necklace_of_fireballs
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
        beads_expended:
          type: integer
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      capabilities:
      - inventory
      - effects
      - spellcasting
      - roll_resolution
    requires:
    - dnd2024:item_necklace_of_fireballs
    citations:
    - SRD 5.2.1 pp. 233; builtin/srd-5.2.1/magic_item/necklace_of_fireballs.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/oathbow:
    source_sha256: 4b03e05b26ee2678e1ebe59e7551b9de689557ef1b9c2083a24518b7be943dd0
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/oathbow
        implementation: dnd2024:item_oathbow
        parameters:
          declare_enemy:
            trigger: ranged_attack_with_bow
            command:
              modes:
              - utter
              - sign
            target: attack_target
            maximum_active: 1
            duration:
              until_any:
              - enemy_death
              - dawn_seven_days_later
            if_enemy_dies_next_selection_after: next_dawn
          against_sworn_enemy:
            attack_advantage: true
            ignore_cover:
            - half
            - three_quarters
            ignore_long_range_disadvantage: true
            on_hit:
              damage:
                roll:
                  expression: 3d6
                  scope: attack
                type: piercing
          while_sworn_enemy_alive:
            other_weapons_attack_disadvantage: true
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_oathbow
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - attack.roll_pending
      - attack.hit
      - participant.died
      - time.dawn
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_oathbow
    citations:
    - SRD 5.2.1 pp. 233; builtin/srd-5.2.1/magic_item/oathbow.md
  dnd2024:items/oil_of_slipperiness:
    source_sha256: 79f5868efa53c5d33c54d40650b18e1b819f0c5118ba9c52456d5a6d9bf42823
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/oil_of_slipperiness
        implementation: dnd2024:item_oil_of_slipperiness
        parameters:
          consume: apply_or_pour
          apply:
            duration:
              amount: 10
              unit: minute
            target: creature_and_worn_carried_equipment
            vials_by_size:
              tiny: 1
              small: 1
              medium: 1
              large: 2
              huge: 3
              gargantuan: 4
            spell_effect:
              key: dnd2024:spells/freedom_of_movement
            duration_effect:
              amount: 8
              unit: hour
          pour:
            action: magic
            area:
              shape: square
              side_ft: 10
            spell_effect:
              key: dnd2024:spells/grease
            duration_effect:
              amount: 8
              unit: hour
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_potions
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_oil_of_slipperiness
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - item.applied
      - action.magic_pending
      - time.elapsed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_oil_of_slipperiness
    citations:
    - SRD 5.2.1 pp. 233, 234; builtin/srd-5.2.1/magic_item/oil_of_slipperiness.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/periapt_of_proof_against_poison:
    source_sha256: 29f214375779318a83080328c12bfb81906ef4eebf1b3ddf35da6d2d46f3e314
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/periapt_of_proof_against_poison
        implementation: dnd2024:item_periapt_of_proof_against_poison
        parameters:
          equipment_state: worn
          condition_immunities:
          - poisoned
          damage_immunities:
          - poison
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_periapt_of_proof_against_poison
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - condition.pending
      - damage.pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - roll_resolution
    requires:
    - dnd2024:item_periapt_of_proof_against_poison
    citations:
    - SRD 5.2.1 pp. 234; builtin/srd-5.2.1/magic_item/periapt_of_proof_against_poison.md
  dnd2024:items/pipes_of_haunting:
    source_sha256: f9201b920b52ded6b5118b0afae4831920b4ccc200ee6f88ef8a9e734351a6fc
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/pipes_of_haunting
        implementation: dnd2024:item_pipes_of_haunting
        parameters:
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                expression: 1d3
                scope: use
              cap: maximum
          activate:
            action: magic
            play_pipes: true
            charge_cost: 1
            targets:
              chooser: user
              range_ft: 30
            save:
              ability: wisdom
              dc: 15
              failure:
                condition: frightened
                duration:
                  amount: 1
                  unit: minute
              success:
                immune_to_same_pipes_effect:
                  amount: 24
                  unit: hour
          repeat_save:
            event: target_turn_end
            ability: wisdom
            dc: 15
            success:
              end_effect: true
              immune_to_same_pipes_effect:
                amount: 24
                unit: hour
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_pipes_of_haunting
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - turn.ended
      - time.dawn
      - time.elapsed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_pipes_of_haunting
    citations:
    - SRD 5.2.1 pp. 234; builtin/srd-5.2.1/magic_item/pipes_of_haunting.md
  dnd2024:items/portable_hole:
    source_sha256: 5cc8bab46e348a2f237c7e9d593df23326e12f67820aa97ccb2981bff7c5d49d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/portable_hole
        implementation: dnd2024:item_portable_hole
        parameters:
          unfolded_diameter_ft: 6
          open:
            action: magic
            place_on_or_against: solid_surface
            space:
              shape: cylinder
              diameter_ft: 6
              depth_ft: 10
              plane: extradimensional
            creates_through_passage: false
            exit: climb_out
          close:
            action: magic
            grasp_edges_and_fold: true
            retain_contents: true
          weight: negligible_regardless_of_contents
          closed:
            escape:
              action: action
              check:
                ability: strength
                skill: athletics
                dc: 10
              success:
                exit_distance_ft: 5
            breathing_hours:
              op: divide
              args:
              - 1
              - binding: breathing_occupants
            zero_breathing_occupants_air_depletion: false
          nested_extradimensional:
            trigger: placed_inside_extradimensional_item
            examples:
            - key: dnd2024:items/bag_of_holding
            - key: dnd2024:items/handy_haversack
            include_similar_items: true
            destroy_both_items: true
            gate:
              origin: insertion_location
              destination_plane: astral
              destination_location: random
              creatures_radius_ft: 10
              exclude_total_cover: true
              forced_transport: true
              one_way: true
              close_after_transport: true
              reopenable: false
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_portable_hole
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
        breathing_occupants:
          type: integer
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - action.pending
      - inventory.insert_pending
      - time.elapsed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_portable_hole
    citations:
    - SRD 5.2.1 pp. 234, 235; builtin/srd-5.2.1/magic_item/portable_hole.md
  dnd2024:items/potion_of_climbing:
    source_sha256: 09e1354d02f26eb7c1923e27e7763e905285d63c6ba08a0f674e91330387a67a
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_climbing
        implementation: dnd2024:item_potion_of_climbing
        parameters:
          consume: drink
          duration:
            amount: 1
            unit: hour
          climb_speed: speed
          check_advantage:
            ability: strength
            skill: athletics
            task: climb
          consume_action: bonus_action
          administer_action: bonus_action
          effect_target: drinker
          consumed_quantity: 1
          effect_starts: immediately_after_consumption
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_potions
    contract:
      id: dnd2024:item_potion_of_climbing
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - item.consumed
      - time.elapsed
      - check.pending
      - speed.changed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_potion_of_climbing
    citations:
    - SRD 5.2.1 pp. 235; builtin/srd-5.2.1/magic_item/potion_of_climbing.md
  dnd2024:items/potion_of_gaseous_form:
    source_sha256: 2dd6fd89c05a7973c27f6d185d6b9f9e981e6ee5ca83c844a5bddeaa13caeeaa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_gaseous_form
        implementation: dnd2024:item_potion_of_gaseous_form
        parameters:
          consume: drink
          spell_effect:
            key: dnd2024:spells/gaseous_form
          duration:
            amount: 1
            unit: hour
          concentration: false
          end_early_action: bonus_action
          consume_action: bonus_action
          administer_action: bonus_action
          effect_target: drinker
          consumed_quantity: 1
          effect_starts: immediately_after_consumption
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_potions
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_potion_of_gaseous_form
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - item.consumed
      - action.bonus_pending
      - time.elapsed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
    requires:
    - dnd2024:item_potion_of_gaseous_form
    citations:
    - SRD 5.2.1 pp. 235; builtin/srd-5.2.1/magic_item/potion_of_gaseous_form.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/potion_of_heroism:
    source_sha256: b43a88f4fbd53533a123cf1b254a369d8ea097c5a1d2a282c6a91ed612700fb9
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_heroism
        implementation: dnd2024:item_potion_of_heroism
        parameters:
          consume: drink
          temporary_hp: 10
          duration:
            amount: 1
            unit: hour
          spell_effect:
            key: dnd2024:spells/bless
          concentration: false
          consume_action: bonus_action
          administer_action: bonus_action
          effect_target: drinker
          consumed_quantity: 1
          effect_starts: immediately_after_consumption
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_potions
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_potion_of_heroism
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - item.consumed
      - time.elapsed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
    requires:
    - dnd2024:item_potion_of_heroism
    citations:
    - SRD 5.2.1 pp. 236; builtin/srd-5.2.1/magic_item/potion_of_heroism.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/potion_of_longevity:
    source_sha256: 35a58323149cea572d4217a275729a46884163eb57970db2f56cc93eb0750ad1
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_longevity
        implementation: dnd2024:item_potion_of_longevity
        parameters:
          consume: drink
          physical_age_change_years:
            expression: 1d6+6
            scope: use
          default_direction: decrease
          minimum_age_years: 13
          subsequent_use:
            track: lifetime_prior_potions_of_longevity_consumed
            aging_probability_percent:
              op: min
              args:
              - 100
              - op: multiply
                args:
                - 10
                - binding: prior_consumptions
            on_probability_success: increase_age_instead
          consume_action: bonus_action
          administer_action: bonus_action
          effect_target: drinker
          consumed_quantity: 1
          effect_starts: immediately_after_consumption
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_potions
    contract:
      id: dnd2024:item_potion_of_longevity
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
        prior_consumptions:
          type: integer
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - item.consumed
      capabilities:
      - inventory
      - effects
    requires:
    - dnd2024:item_potion_of_longevity
    citations:
    - SRD 5.2.1 pp. 236; builtin/srd-5.2.1/magic_item/potion_of_longevity.md
  dnd2024:items/potion_of_resistance:
    source_sha256: 32c4d9f772976977b63bd576669f43d80a88e352de1e4861301b206816ac6850
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_resistance
        implementation: dnd2024:item_potion_of_resistance
        parameters:
          consume: drink
          duration:
            amount: 1
            unit: hour
          resistance:
            chooser: gm
            method:
            - choose
            - roll
            table:
              roll:
                expression: 1d10
                scope: use
              rows:
              - min: 1
                max: 1
                result:
                  damage_type: acid
              - min: 2
                max: 2
                result:
                  damage_type: cold
              - min: 3
                max: 3
                result:
                  damage_type: fire
              - min: 4
                max: 4
                result:
                  damage_type: force
              - min: 5
                max: 5
                result:
                  damage_type: lightning
              - min: 6
                max: 6
                result:
                  damage_type: necrotic
              - min: 7
                max: 7
                result:
                  damage_type: poison
              - min: 8
                max: 8
                result:
                  damage_type: psychic
              - min: 9
                max: 9
                result:
                  damage_type: radiant
              - min: 10
                max: 10
                result:
                  damage_type: thunder
          consume_action: bonus_action
          administer_action: bonus_action
          effect_target: drinker
          consumed_quantity: 1
          effect_starts: immediately_after_consumption
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_potions
    contract:
      id: dnd2024:item_potion_of_resistance
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - item.consumed
      - time.elapsed
      - damage.pending
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_potion_of_resistance
    citations:
    - SRD 5.2.1 pp. 236; builtin/srd-5.2.1/magic_item/potion_of_resistance.md
  dnd2024:items/potion_of_water_breathing:
    source_sha256: 747ca5eeed0cc2dcb56460bc4227273049565806a7974bdcd340cd644acfb794
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/potion_of_water_breathing
        implementation: dnd2024:item_potion_of_water_breathing
        parameters:
          consume: drink
          duration:
            amount: 24
            unit: hour
          breathe_underwater: true
          consume_action: bonus_action
          administer_action: bonus_action
          effect_target: drinker
          consumed_quantity: 1
          effect_starts: immediately_after_consumption
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_potions
    contract:
      id: dnd2024:item_potion_of_water_breathing
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - item.consumed
      - time.elapsed
      capabilities:
      - inventory
      - effects
      - resources_and_time
    requires:
    - dnd2024:item_potion_of_water_breathing
    citations:
    - SRD 5.2.1 pp. 236, 237; builtin/srd-5.2.1/magic_item/potion_of_water_breathing.md
  dnd2024:items/ring_of_animal_influence:
    source_sha256: 48b4116c70edf48d12acd0b1d3b8d917c5ec7b5fdbc1d7e6e0ebc634c765ea39
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_animal_influence
        implementation: dnd2024:item_ring_of_animal_influence
        parameters:
          equipment_state: worn_on_finger_or_similar_digit
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                expression: 1d3
                scope: use
              cap: maximum
          spell_grants:
          - spell:
              key: dnd2024:spells/animal_friendship
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
            save_dc: 13
          - spell:
              key: dnd2024:spells/fear
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
            save_dc: 13
            target_creature_types:
            - beast
          - spell:
              key: dnd2024:spells/speak_with_animals
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
            save_dc: 13
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_rings
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_ring_of_animal_influence
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - time.dawn
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
      - participant_state
    requires:
    - dnd2024:item_ring_of_animal_influence
    citations:
    - SRD 5.2.1 pp. 237; builtin/srd-5.2.1/magic_item/ring_of_animal_influence.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/ring_of_evasion:
    source_sha256: 8da3092bff4ab54fa99ef20b25a8da73890a6c88e9cb824297640597e006a1fa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_evasion
        implementation: dnd2024:item_ring_of_evasion
        parameters:
          equipment_state: worn_on_finger_or_similar_digit
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                expression: 1d3
                scope: use
              cap: maximum
          reaction:
            trigger: failed_dexterity_saving_throw
            charge_cost: 1
            replace_outcome: success
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_rings
    contract:
      id: dnd2024:item_ring_of_evasion
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - saving_throw.failed
      - time.dawn
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - resources_and_time
    requires:
    - dnd2024:item_ring_of_evasion
    citations:
    - SRD 5.2.1 pp. 238; builtin/srd-5.2.1/magic_item/ring_of_evasion.md
  dnd2024:items/ring_of_invisibility:
    source_sha256: 75eb06cf791f96c6fe506b5efa82756105fe23ea17595b885e754cdde4b77c57
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_invisibility
        implementation: dnd2024:item_ring_of_invisibility
        parameters:
          equipment_state: worn_on_finger_or_similar_digit
          activate:
            action: magic
            condition: invisible
            target: self
          ends_on:
          - ring_removed
          - user_bonus_action_end
          attack_or_cast_ends_effect: false
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_rings
    contract:
      id: dnd2024:item_ring_of_invisibility
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - action.bonus_pending
      - equipment.changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - roll_resolution
    requires:
    - dnd2024:item_ring_of_invisibility
    citations:
    - SRD 5.2.1 pp. 238; builtin/srd-5.2.1/magic_item/ring_of_invisibility.md
  dnd2024:items/ring_of_protection:
    source_sha256: 489aedbdc089f8361674c704442060ae6adeb633e4bba2e21f95310bfda0d723
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_protection
        implementation: dnd2024:item_ring_of_protection
        parameters:
          equipment_state: worn_on_finger_or_similar_digit
          ac_bonus: 1
          saving_throw_bonus: 1
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_rings
    contract:
      id: dnd2024:item_ring_of_protection
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - saving_throw.pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
    requires:
    - dnd2024:item_ring_of_protection
    citations:
    - SRD 5.2.1 pp. 238; builtin/srd-5.2.1/magic_item/ring_of_protection.md
  dnd2024:items/ring_of_shooting_stars:
    source_sha256: 5c30aea24656f03707f3ab24b5ccd21b9350e701bc541ffea032e129c2591bf6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_shooting_stars
        implementation: dnd2024:item_ring_of_shooting_stars
        parameters:
          spell_grants:
          - spell:
              key: dnd2024:spells/dancing_lights
            charge_cost: 0
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/light
            charge_cost: 0
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/faerie_fire
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          charges:
            maximum: 6
            initial: 6
            recovery:
              event: dawn
              amount:
                expression: 1d6
                scope: use
              cap: maximum
          lightning_spheres:
            action: magic
            charge_cost: 2
            count:
              minimum: 1
              maximum: 4
              chooser: user
            sphere_diameter_ft: 3
            initial_location:
              unoccupied: true
              visible_to_user: true
              range_ft: 120
            duration:
              amount: 1
              unit: minute
            concentration: true
            dim_light_radius_ft: 30
            move:
              action: bonus_action
              each_sphere_maximum_ft: 30
              maximum_distance_from_user_ft: 120
            discharge:
              trigger: first_within_five_feet_of_eligible_creature
              distance_ft: 5
              exclude_user: true
              exclude_total_cover: true
              destroy_sphere: true
              save:
                ability: dexterity
                dc: 15
                failure:
                  damage:
                    type: lightning
                    roll_by_initial_sphere_count:
                      1:
                        expression: 4d12
                        scope: use
                      2:
                        expression: 5d4
                        scope: use
                      3:
                        expression: 2d6
                        scope: use
                      4:
                        expression: 2d4
                        scope: use
                success:
                  damage_scale:
                    numerator: 1
                    denominator: 2
          shooting_stars:
            action: magic
            charge_cost:
              minimum: 1
              maximum: 3
            motes_per_charge: 1
            each_mote:
              destination:
                visible_to_user: true
                range_ft: 60
              area:
                shape: cube
                size_ft: 15
                origin: destination
              save:
                ability: dexterity
                dc: 15
                failure:
                  damage:
                    roll:
                      expression: 5d4
                      scope: use
                    type: radiant
                success:
                  damage_scale:
                    numerator: 1
                    denominator: 2
          equipment_state: worn_on_finger_or_similar_digit
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_rings
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_ring_of_shooting_stars
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - action.magic_pending
      - action.bonus_pending
      - concentration.ended
      - position.changed
      - time.dawn
      - time.elapsed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_ring_of_shooting_stars
    citations:
    - SRD 5.2.1 pp. 238, 239; builtin/srd-5.2.1/magic_item/ring_of_shooting_stars.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/ring_of_swimming:
    source_sha256: e1601c56e21e4caf78fc638d0c1ae67e9a7aeeda33a909aebd7667843e488117
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_swimming
        implementation: dnd2024:item_ring_of_swimming
        parameters:
          equipment_state: worn_on_finger_or_similar_digit
          swim_speed_ft: 40
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_rings
    contract:
      id: dnd2024:item_ring_of_swimming
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      capabilities:
      - inventory
      - effects
    requires:
    - dnd2024:item_ring_of_swimming
    citations:
    - SRD 5.2.1 pp. 239; builtin/srd-5.2.1/magic_item/ring_of_swimming.md
  dnd2024:items/ring_of_three_wishes:
    source_sha256: 9b12868342ba553efd56786cfa3d6413c63376636dcc4a4ebcea566f36f2ec51
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_three_wishes
        implementation: dnd2024:item_ring_of_three_wishes
        parameters:
          equipment_state: worn_on_finger_or_similar_digit
          charges:
            maximum: 3
            initial: 3
          spell_grants:
          - spell:
              key: dnd2024:spells/wish
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          last_charge:
            magical: false
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_rings
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_ring_of_three_wishes
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
    requires:
    - dnd2024:item_ring_of_three_wishes
    citations:
    - SRD 5.2.1 pp. 239; builtin/srd-5.2.1/magic_item/ring_of_three_wishes.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/ring_of_x_ray_vision:
    source_sha256: 3316052224ac433db472c731ed2da768baeb91ade963f53902d28bcd8e6824b6
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/ring_of_x_ray_vision
        implementation: dnd2024:item_ring_of_x_ray_vision
        parameters:
          equipment_state: worn_on_finger_or_similar_digit
          activate:
            action: magic
            duration:
              amount: 1
              unit: minute
            range_ft: 30
            objects_appear_transparent: true
            light_passes_through_for_user_vision: true
            penetration_limits:
            - material: stone
              thickness_inches: 12
            - material: common_metal
              thickness_inches: 1
            - material: wood
              thickness_inches: 36
            - material: dirt
              thickness_inches: 36
            blocked_by:
            - thicker_substances
            - any_sheet_of_lead
          additional_uses_before_long_rest:
            save:
              ability: constitution
              dc: 15
              failure:
                exhaustion_increase: 1
              success:
                effect: none
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_rings
    contract:
      id: dnd2024:item_ring_of_x_ray_vision
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - time.elapsed
      - rest.completed
      - perception.pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_ring_of_x_ray_vision
    citations:
    - SRD 5.2.1 pp. 239, 240; builtin/srd-5.2.1/magic_item/ring_of_x_ray_vision.md
  dnd2024:items/robe_of_stars:
    source_sha256: 64e16b05d77c8eeb60099e689c9667cefad9bedd143e57bff58003157f00e1be
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/robe_of_stars
        implementation: dnd2024:item_robe_of_stars
        parameters:
          equipment_state: worn
          saving_throw_bonus: 1
          stars:
            maximum: 6
            initial: 6
            recovery:
              event: dusk
              amount:
                expression: 1d6
                scope: use
              cap: maximum
          spell_grants:
          - spell:
              key: dnd2024:spells/magic_missile
            charge_cost: 1
            casting_time: magic
            method: item
            slot_cost: 0
            components: []
            level: 5
            range: spell_default
            duration: spell_default
            concentration: spell_default
            activation: remove_and_expend_star
          astral_travel:
            depart_action: magic
            include: all_worn_and_carried
            destination_plane: astral
            duration: until_return_action
            return_action: magic
            return_plane: prior_plane
            return_location: last_occupied_space
            if_occupied: nearest_unoccupied_space
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_robe_of_stars
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - time.dusk
      - saving_throw.pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - spatial_state
    requires:
    - dnd2024:item_robe_of_stars
    citations:
    - SRD 5.2.1 pp. 240; builtin/srd-5.2.1/magic_item/robe_of_stars.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/rod_of_absorption:
    source_sha256: b839f1ffa17b0555fc67056e330b6f083686ed853e341b00ffdf8a0bdc854ed4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/rod_of_absorption
        implementation: dnd2024:item_rod_of_absorption
        parameters:
          equipment_state: held
          absorb:
            action: reaction
            targeting: only_user
            area_of_effect_allowed: false
            cancel_spell_effect: true
            casting_resources_wasted: true
            energy_gained: cast_spell_level
            lifetime_energy_limit: 50
            overflow: spell_unaffected
            store_spell_identity: false
          attunement_reveals:
          - lifetime_absorbed_levels
          - current_stored_levels
          spend_energy:
            requires_spellcaster: true
            spell_must_be: prepared_or_known
            slot_level_maximum: 5
            cannot_exceed_own_slot_levels: true
            energy_cost: slot_level
            purpose: cast_spell_in_place_of_own_slot
            other_casting_rules: normal
          initial_energy:
            expression: 1d10
            scope: use
          initial_energy_counts_toward_lifetime_absorption: true
          initial_lifetime_absorbed_levels:
            source: item_history
            minimum: initial_stored_energy
            maximum: 50
          becomes_nonmagical_when:
            lifetime_absorbed_levels: 50
            current_stored_levels: 0
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_rods
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_rod_of_absorption
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
    requires:
    - dnd2024:item_rod_of_absorption
    citations:
    - SRD 5.2.1 pp. 241; builtin/srd-5.2.1/magic_item/rod_of_absorption.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/rod_of_resurrection:
    source_sha256: e113380825cc4240e01fb42cfd24a64ec5e60987c73960edf9bb041a4f9bdb83
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/rod_of_resurrection
        implementation: dnd2024:item_rod_of_resurrection
        parameters:
          equipment_state: held
          charges:
            maximum: 5
            initial: 5
            recovery:
              event: dawn
              amount:
                expression: '1'
                scope: use
              cap: maximum
          spell_grants:
          - spell:
              key: dnd2024:spells/heal
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/resurrection
            charge_cost: 5
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          last_charge:
            roll:
              expression: 1d20
              scope: use
            'on': 1
            effect: item_disappears_harmlessly
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_rods
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_rod_of_resurrection
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - time.dawn
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_rod_of_resurrection
    citations:
    - SRD 5.2.1 pp. 242; builtin/srd-5.2.1/magic_item/rod_of_resurrection.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/rope_of_climbing:
    source_sha256: 9af3115d10fa959314a0b7206251e8651e40eb7ffdfbccd2d5fbc40986029244
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/rope_of_climbing
        implementation: dnd2024:item_rope_of_climbing
        parameters:
          length_ft: 60
          weight_capacity_lb: 3000
          ac: 20
          hp: 20
          damage_immunities:
          - poison
          - psychic
          regeneration:
            amount: 1
            interval:
              amount: 5
              unit: minute
            minimum_current_hp: 1
          zero_hp: destroyed
          animate:
            equipment_state: one_end_held
            action: magic
            destination_maximum_distance: rope_length
            movement_ft_on_activation: 10
            movement_ft_at_user_turn_start: 10
            ends_on:
            - destination_reached
            - stop_command
          commands:
          - fasten_to_object
          - unfasten
          - knot
          - unknot
          - coil_for_carrying
          knotted:
            knot_interval_ft: 1
            length_ft: 50
            ability_check_advantage: climb_using_rope
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_rope_of_climbing
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - command.issued
      - turn.started
      - time.elapsed
      - participant.hp_changed
      - check.pending
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - spatial_state
    requires:
    - dnd2024:item_rope_of_climbing
    citations:
    - SRD 5.2.1 pp. 242; builtin/srd-5.2.1/magic_item/rope_of_climbing.md
  dnd2024:items/scimitar_of_speed:
    source_sha256: 84eb27dc14143c53e12a60de3ef8f05586419dcff81ca618b6ca91ccd12a4a6d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/scimitar_of_speed
        implementation: dnd2024:item_scimitar_of_speed
        parameters:
          attack_roll_bonus: 2
          damage_roll_bonus: 2
          scope: attacks_with_source_item
          bonus_action_attack:
            count: 1
            weapon: source_item
            on_own_turn: true
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_scimitar_of_speed
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - attack.roll_pending
      - action.bonus_pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - roll_resolution
    requires:
    - dnd2024:item_scimitar_of_speed
    citations:
    - SRD 5.2.1 pp. 243; builtin/srd-5.2.1/magic_item/scimitar_of_speed.md
  dnd2024:items/shield_1_2_or_3:
    source_sha256: 6e883adc286144355d6e11d768a5c7b23c0ecf70cc4239c3e0c8be7972e0c237
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/shield_1_2_or_3
        implementation: dnd2024:item_shield_1_2_or_3
        parameters:
          equipment_state: held
          variants:
          - rarity: uncommon
            ac_bonus: 1
            additional_to_normal_shield_bonus: true
          - rarity: rare
            ac_bonus: 2
            additional_to_normal_shield_bonus: true
          - rarity: very_rare
            ac_bonus: 3
            additional_to_normal_shield_bonus: true
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_armor
    contract:
      id: dnd2024:item_shield_1_2_or_3
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      capabilities:
      - inventory
      - effects
    requires:
    - dnd2024:item_shield_1_2_or_3
    citations:
    - SRD 5.2.1 pp. 243; builtin/srd-5.2.1/magic_item/shield_1_2_or_3.md
  dnd2024:items/slippers_of_spider_climbing:
    source_sha256: f0c78544788d9b740395ee3daf1d44ad5d87520efaa76eefecccbf73d6467dd4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/slippers_of_spider_climbing
        implementation: dnd2024:item_slippers_of_spider_climbing
        parameters:
          equipment_state: worn
          climb_speed: speed
          spider_movement:
            surfaces:
            - vertical
            - ceiling
            hands_free: true
            disallowed_surface: slippery
            examples:
            - ice
            - oil
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_slippers_of_spider_climbing
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - movement.pending
      - speed.changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spatial_state
    requires:
    - dnd2024:item_slippers_of_spider_climbing
    citations:
    - SRD 5.2.1 pp. 244; builtin/srd-5.2.1/magic_item/slippers_of_spider_climbing.md
  dnd2024:items/spellguard_shield:
    source_sha256: 81431841a2727156148f87b1016e636cab1c39a96a1acf4343c2546ce212d0eb
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/spellguard_shield
        implementation: dnd2024:item_spellguard_shield
        parameters:
          equipment_state: held
          save_advantage_against:
          - spells
          - other_magical_effects
          attacks_against_user_disadvantage:
            attack_category: spell
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_armor
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_spellguard_shield
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - saving_throw.pending
      - attack.roll_pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - roll_resolution
    requires:
    - dnd2024:item_spellguard_shield
    citations:
    - SRD 5.2.1 pp. 244; builtin/srd-5.2.1/magic_item/spellguard_shield.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/staff_of_fire:
    source_sha256: c6dc616db78fb5bf393a2b931ee331fb7942213cbfdcf109a464610a551234ac
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_fire
        implementation: dnd2024:item_staff_of_fire
        parameters:
          equipment_state: held
          damage_resistance: fire
          charges:
            maximum: 10
            initial: 10
            recovery:
              event: dawn
              amount:
                expression: 1d6+4
                scope: use
              cap: maximum
          spell_save_dc: user_spell_save_dc
          spell_grants:
          - spell:
              key: dnd2024:spells/burning_hands
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/wall_of_fire
            charge_cost: 4
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/fireball
            charge_cost: 3
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          last_charge:
            roll:
              expression: 1d20
              scope: use
            'on': 1
            effect: destroyed
          attunement:
            required: true
            eligible_classes:
            - druid
            - sorcerer
            - warlock
            - wizard
          common_rules:
          - key: dnd2024:rules/magic_item_categories_staffs
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_staff_of_fire
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - time.dawn
      - equipment.changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_staff_of_fire
    citations:
    - SRD 5.2.1 pp. 245; builtin/srd-5.2.1/magic_item/staff_of_fire.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/staff_of_power:
    source_sha256: 4aacf855a46723943e2370bc1f84c0a003c5ab824b4264198adff5a4912c4232
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_power
        implementation: dnd2024:item_staff_of_power
        parameters:
          weapon:
            key: dnd2024:equipment/quarterstaff
          attack_roll_bonus: 2
          damage_roll_bonus: 2
          scope: attacks_with_source_item
          while_held:
            ac_bonus: 2
            saving_throw_bonus: 2
            spell_attack_bonus: 2
          charges:
            maximum: 20
            initial: 20
            recovery:
              event: dawn
              amount:
                expression: 2d8+4
                scope: use
              cap: maximum
          spell_grants_require: held
          spell_save_dc: user_spell_save_dc
          spell_grants:
          - spell:
              key: dnd2024:spells/cone_of_cold
            charge_cost: 5
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/fireball
            charge_cost: 5
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: 5
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/globe_of_invulnerability
            charge_cost: 6
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/hold_monster
            charge_cost: 5
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/levitate
            charge_cost: 2
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/lightning_bolt
            charge_cost: 5
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: 5
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/magic_missile
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/ray_of_enfeeblement
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/wall_of_force
            charge_cost: 5
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          last_charge:
            table:
              roll:
                expression: 1d20
                scope: use
              rows:
              - min: 1
                max: 1
                result:
                  retain_only:
                  - weapon_attack_bonus
                  - weapon_damage_bonus
              - min: 2
                max: 19
                result:
                  effect: none
              - min: 20
                max: 20
                result:
                  restore_charges:
                    expression: 1d8+2
                    scope: use
          retributive_strike:
            action: magic
            method:
            - break_over_knee
            - break_against_solid_surface
            destroy_item: true
            area:
              shape: emanation
              radius_ft: 30
              origin: staff
            user:
              escape_chance_percent: 50
              on_escape:
                transport: random_plane
                damage: 0
              on_no_escape:
                damage:
                  type: force
                  amount:
                    op: multiply
                    args:
                    - 16
                    - binding: charges_before_destruction
            other_creatures:
              save:
                ability: dexterity
                dc: 17
                failure:
                  damage:
                    type: force
                    amount:
                      op: multiply
                      args:
                      - 4
                      - binding: charges_before_destruction
                success:
                  damage_scale:
                    numerator: 1
                    denominator: 2
          attunement:
            required: true
            eligible_classes:
            - sorcerer
            - warlock
            - wizard
          common_rules:
          - key: dnd2024:rules/magic_item_categories_staffs
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_staff_of_power
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
        charges_before_destruction:
          type: integer
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - action.magic_pending
      - time.dawn
      - equipment.changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_staff_of_power
    citations:
    - SRD 5.2.1 pp. 245, 246; builtin/srd-5.2.1/magic_item/staff_of_power.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/staff_of_the_magi:
    source_sha256: 5161acd7250f9d4fdc73f444998d32244e36712920b21695a7145a869ed75f65
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_the_magi
        implementation: dnd2024:item_staff_of_the_magi
        parameters:
          weapon:
            key: dnd2024:equipment/quarterstaff
          attack_roll_bonus: 2
          damage_roll_bonus: 2
          scope: attacks_with_source_item
          while_held:
            spell_attack_bonus: 2
            save_advantage_against:
            - spells
          charges:
            maximum: 50
            initial: 50
            recovery:
              event: dawn
              amount:
                expression: 4d6+2
                scope: use
              cap: maximum
          spell_grants_require: held
          spell_save_dc: user_spell_save_dc
          spell_grants:
          - spell:
              key: dnd2024:spells/arcane_lock
            charge_cost: 0
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/conjure_elemental
            charge_cost: 7
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/detect_magic
            charge_cost: 0
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/dispel_magic
            charge_cost: 3
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/enlarge_reduce
            charge_cost: 0
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/fireball
            charge_cost: 7
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: 7
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/flaming_sphere
            charge_cost: 2
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/ice_storm
            charge_cost: 4
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/invisibility
            charge_cost: 2
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/knock
            charge_cost: 2
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/light
            charge_cost: 0
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/lightning_bolt
            charge_cost: 7
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: 7
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/mage_hand
            charge_cost: 0
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/passwall
            charge_cost: 5
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/plane_shift
            charge_cost: 7
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/protection_from_evil_and_good
            charge_cost: 0
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/telekinesis
            charge_cost: 5
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/wall_of_fire
            charge_cost: 4
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          - spell:
              key: dnd2024:spells/web
            charge_cost: 2
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          absorption:
            equipment_state: held
            action: reaction
            trigger: another_creature_casts_spell_targeting_only_user
            cancel_effect: true
            charge_gain: spell_level
            if_result_above_50:
              effect: retributive_strike
              trigger: automatic
              charge_basis: total_after_absorption
          last_charge:
            roll:
              expression: 1d20
              scope: use
            'on': 20
            restore_charges:
              expression: 1d12+1
              scope: use
          retributive_strike:
            action: magic
            method:
            - break_over_knee
            - break_against_solid_surface
            destroy_item: true
            area:
              shape: emanation
              radius_ft: 30
              origin: staff
            user:
              escape_chance_percent: 50
              on_escape:
                transport: random_plane
                damage: 0
              on_no_escape:
                damage:
                  type: force
                  amount:
                    op: multiply
                    args:
                    - 16
                    - binding: charges_before_destruction
            other_creatures:
              save:
                ability: dexterity
                dc: 17
                failure:
                  damage:
                    type: force
                    amount:
                      op: multiply
                      args:
                      - 6
                      - binding: charges_before_destruction
                success:
                  damage_scale:
                    numerator: 1
                    denominator: 2
          attunement:
            required: true
            eligible_classes:
            - sorcerer
            - warlock
            - wizard
          common_rules:
          - key: dnd2024:rules/magic_item_categories_staffs
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_staff_of_the_magi
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
        charges_before_destruction:
          type: integer
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - action.magic_pending
      - time.dawn
      - equipment.changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
      - spatial_state
      - participant_state
    requires:
    - dnd2024:item_staff_of_the_magi
    citations:
    - SRD 5.2.1 pp. 246, 247; builtin/srd-5.2.1/magic_item/staff_of_the_magi.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/staff_of_thunder_and_lightning:
    source_sha256: eafb294cbec1512709218b32b48760d6e36e16394854562fdc7543a7c2017591
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/staff_of_thunder_and_lightning
        implementation: dnd2024:item_staff_of_thunder_and_lightning
        parameters:
          weapon:
            key: dnd2024:equipment/quarterstaff
          attack_roll_bonus: 2
          damage_roll_bonus: 2
          scope: attacks_with_source_item
          properties_each_independent_uses:
            maximum: 1
            initial: 1
            recovery:
              event: dawn
              amount:
                expression: '1'
                scope: use
              cap: maximum
          lightning:
            trigger: melee_hit_with_staff
            action: none
            damage:
              roll:
                expression: 2d6
                scope: attack
              type: lightning
          thunder:
            trigger: melee_hit_with_staff
            action: none
            audible_ft: 300
            save:
              ability: constitution
              dc: 17
              failure:
                condition: stunned
                duration:
                  until: end_of_user_next_turn
              success:
                effect: none
          thunder_and_lightning:
            trigger: immediately_after_melee_hit_with_staff
            action: bonus_action
            apply_properties:
            - lightning
            - thunder
            consume_daily_uses:
            - thunder_and_lightning
            consume_other_daily_uses: false
          lightning_strike:
            action: magic
            area:
              shape: line
              width_ft: 5
              length_ft: 120
            save:
              ability: dexterity
              dc: 17
              failure:
                damage:
                  roll:
                    expression: 9d6
                    scope: use
                  type: lightning
              success:
                damage_scale:
                  numerator: 1
                  denominator: 2
          thunderclap:
            action: magic
            audible_ft: 600
            area:
              shape: emanation
              radius_ft: 60
              origin: user
            save:
              ability: constitution
              dc: 17
              failure:
                damage:
                  roll:
                    expression: 2d6
                    scope: use
                  type: thunder
                condition: deafened
                duration:
                  amount: 1
                  unit: minute
              success:
                damage_scale:
                  numerator: 1
                  denominator: 2
                condition: null
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_staffs
    contract:
      id: dnd2024:item_staff_of_thunder_and_lightning
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - attack.hit
      - action.magic_pending
      - action.bonus_pending
      - time.dawn
      - turn.ended
      - time.elapsed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - spatial_state
    requires:
    - dnd2024:item_staff_of_thunder_and_lightning
    citations:
    - SRD 5.2.1 pp. 247, 248; builtin/srd-5.2.1/magic_item/staff_of_thunder_and_lightning.md
  dnd2024:items/stone_of_good_luck_luckstone:
    source_sha256: d63474cf4f061f82fd17dde3609cdc0e043120c5076fe232c29255d3447f4c0c
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/stone_of_good_luck_luckstone
        implementation: dnd2024:item_stone_of_good_luck_luckstone
        parameters:
          equipment_state: on_person
          ability_check_bonus: 1
          saving_throw_bonus: 1
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_stone_of_good_luck_luckstone
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - equipment.changed
      - check.pending
      - saving_throw.pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - roll_resolution
    requires:
    - dnd2024:item_stone_of_good_luck_luckstone
    citations:
    - SRD 5.2.1 pp. 248; builtin/srd-5.2.1/magic_item/stone_of_good_luck_luckstone.md
  dnd2024:items/sword_of_sharpness:
    source_sha256: 22530872659e7dcb36428325aa2e110493964f570372e7e2b2a9fc3184f62b71
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/sword_of_sharpness
        implementation: dnd2024:item_sword_of_sharpness
        parameters:
          on_hit_object:
            maximize: weapon_damage_dice
          on_natural_20_against_creature:
            extra_damage:
              amount: 14
              type: slashing
            exhaustion_increase: 1
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_sword_of_sharpness
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - attack.hit
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - roll_resolution
      - participant_state
    requires:
    - dnd2024:item_sword_of_sharpness
    citations:
    - SRD 5.2.1 pp. 248; builtin/srd-5.2.1/magic_item/sword_of_sharpness.md
  dnd2024:items/talisman_of_the_sphere:
    source_sha256: e6d1464b518b77dd393972beda0743667f2ce9fa74415eed2322a0a3afc91f28
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/talisman_of_the_sphere
        implementation: dnd2024:item_talisman_of_the_sphere
        parameters:
          equipment_state_any:
          - held
          - worn
          check_advantage:
            ability: intelligence
            skill: arcana
            task: control_sphere_of_annihilation
            item:
              key: dnd2024:items/sphere_of_annihilation
          move_sphere:
            requires: controlling_sphere_at_turn_start
            action: magic
            distance_ft:
              op: add
              args:
              - 10
              - op: multiply
                args:
                - 10
                - stat:
                    of: actor
                    key: intelligence_modifier
            straight_line_required: false
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_talisman_of_the_sphere
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - check.pending
      - turn.started
      - action.magic_pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - roll_resolution
      - spatial_state
    requires:
    - dnd2024:item_talisman_of_the_sphere
    citations:
    - SRD 5.2.1 pp. 249; builtin/srd-5.2.1/magic_item/talisman_of_the_sphere.md
  dnd2024:items/tome_of_clear_thought:
    source_sha256: 4fbf0ff2355d4dface2890f293d3d0d70dc8cbb8e13b386c6026da906c0ccce4
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/tome_of_clear_thought
        implementation: dnd2024:item_tome_of_clear_thought
        parameters:
          study:
            hours: 48
            within_days: 6
            practice_required: true
          completion:
            ability: intelligence
            increase: 2
            maximum: 30
            duration: permanent
            item_magical: false
          recovery:
            after:
              amount: 100
              unit: year
            item_magical: true
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_tome_of_clear_thought
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - study.completed
      - time.elapsed
      capabilities:
      - inventory
      - effects
      - resources_and_time
    requires:
    - dnd2024:item_tome_of_clear_thought
    citations:
    - SRD 5.2.1 pp. 249; builtin/srd-5.2.1/magic_item/tome_of_clear_thought.md
  dnd2024:items/trident_of_fish_command:
    source_sha256: 72d25f8b5035fc8ebacf0ef172ec802ccadcf920b0c80548670c8d91f87835ec
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/trident_of_fish_command
        implementation: dnd2024:item_trident_of_fish_command
        parameters:
          equipment_state: carried
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                expression: 1d3
                scope: use
              cap: maximum
          spell_grants:
          - spell:
              key: dnd2024:spells/dominate_beast
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
            save_dc: 15
            target_creature_types:
            - beast
            target_requires_swim_speed: true
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_trident_of_fish_command
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - time.dawn
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
      - participant_state
    requires:
    - dnd2024:item_trident_of_fish_command
    citations:
    - SRD 5.2.1 pp. 250; builtin/srd-5.2.1/magic_item/trident_of_fish_command.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/vorpal_sword:
    source_sha256: 876f6945eed42baa75c979f407da2409e5288030a9ac163a4a03f58c8e0744cd
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/vorpal_sword
        implementation: dnd2024:item_vorpal_sword
        parameters:
          attack_roll_bonus: 3
          damage_roll_bonus: 3
          scope: attacks_with_source_item
          ignore_damage_resistance:
          - slashing
          on_natural_20:
            target: creature
            decapitation:
              heads_removed: 1
              requires_at_least_one_head: true
              dies_if_cannot_survive_head_loss: true
              immune_if:
              - slashing_damage_immunity
              - no_head
              - head_not_needed
              gm_choice:
                type: boolean
                scope: head_too_large_to_sever_with_weapon
              legendary_resistance:
                optional_use: true
                daily_uses_cost: 1
                effect: replace_decapitation_with_extra_damage
            instead_if_decapitation_immune_or_resisted:
              extra_damage:
                amount: 30
                type: slashing
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_vorpal_sword
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - attack.hit
      - damage.pending
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - resources_and_time
      - roll_resolution
      - participant_state
    requires:
    - dnd2024:item_vorpal_sword
    citations:
    - SRD 5.2.1 pp. 250; builtin/srd-5.2.1/magic_item/vorpal_sword.md
  dnd2024:items/wand_of_fear:
    source_sha256: fd237e9f2c3d5e80ba9107c25ca1dbfc9111938bad361072b8380787843f4c21
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_fear
        implementation: dnd2024:item_wand_of_fear
        parameters:
          equipment_state: held
          charges:
            maximum: 7
            initial: 7
            recovery:
              event: dawn
              amount:
                expression: 1d6+1
                scope: use
              cap: maximum
          spell_grants:
          - spell:
              key: dnd2024:spells/command
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
            save_dc: 15
            allowed_commands:
            - flee
            - grovel
          - spell:
              key: dnd2024:spells/fear
            charge_cost: 3
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
            save_dc: 15
            area_override:
              shape: cone
              length_ft: 60
          last_charge:
            roll:
              expression: 1d20
              scope: use
            'on': 1
            effect: destroyed
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wands
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_wand_of_fear
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - time.dawn
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_wand_of_fear
    citations:
    - SRD 5.2.1 pp. 250; builtin/srd-5.2.1/magic_item/wand_of_fear.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/wand_of_magic_detection:
    source_sha256: 1e2d634360ca9a854987532993233d7af263ba4e1c36252333755f16a8d4325d
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_magic_detection
        implementation: dnd2024:item_wand_of_magic_detection
        parameters:
          equipment_state: held
          charges:
            maximum: 3
            initial: 3
            recovery:
              event: dawn
              amount:
                expression: 1d3
                scope: use
              cap: maximum
          spell_grants:
          - spell:
              key: dnd2024:spells/detect_magic
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
          attunement:
            required: false
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wands
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_wand_of_magic_detection
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - time.dawn
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
    requires:
    - dnd2024:item_wand_of_magic_detection
    citations:
    - SRD 5.2.1 pp. 251; builtin/srd-5.2.1/magic_item/wand_of_magic_detection.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/wand_of_polymorph:
    source_sha256: 1b8e78d2789471459749d0f5b77e8ef61d06dc69a8a633bf0270ff049707e2ad
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_polymorph
        implementation: dnd2024:item_wand_of_polymorph
        parameters:
          equipment_state: held
          charges:
            maximum: 7
            initial: 7
            recovery:
              event: dawn
              amount:
                expression: 1d6+1
                scope: use
              cap: maximum
          spell_grants:
          - spell:
              key: dnd2024:spells/polymorph
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
            save_dc: 15
          last_charge:
            roll:
              expression: 1d20
              scope: use
            'on': 1
            effect: destroyed
          attunement:
            required: true
            requires_spellcaster: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wands
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_wand_of_polymorph
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - time.dawn
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_wand_of_polymorph
    citations:
    - SRD 5.2.1 pp. 251; builtin/srd-5.2.1/magic_item/wand_of_polymorph.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/wand_of_web:
    source_sha256: cedb94a6e80fe14963929eb38f53e1557983b00d0044d98b6e2ba29115e4dffa
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/wand_of_web
        implementation: dnd2024:item_wand_of_web
        parameters:
          equipment_state: held
          charges:
            maximum: 7
            initial: 7
            recovery:
              event: dawn
              amount:
                expression: 1d6+1
                scope: use
              cap: maximum
          spell_grants:
          - spell:
              key: dnd2024:spells/web
            charge_cost: 1
            casting_time: spell_default
            method: item
            slot_cost: 0
            components: []
            level: lowest_possible
            range: spell_default
            duration: spell_default
            concentration: spell_default
            save_dc: 13
          last_charge:
            roll:
              expression: 1d20
              scope: use
            'on': 1
            effect: destroyed
          attunement:
            required: true
            requires_spellcaster: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wands
          - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
    contract:
      id: dnd2024:item_wand_of_web
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - spell.cast_pending
      - time.dawn
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spellcasting
      - resources_and_time
      - roll_resolution
    requires:
    - dnd2024:item_wand_of_web
    citations:
    - SRD 5.2.1 pp. 251; builtin/srd-5.2.1/magic_item/wand_of_web.md
    - SRD 5.2.1 p. 206; builtin/srd-5.2.1/pages/206.md (Spells Cast from Items)
  dnd2024:items/weapon_of_warning:
    source_sha256: 91def4ea92a227c1082e878898fbc80a4b771dfe33781a260e2d1b6539fa9a3f
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/weapon_of_warning
        implementation: dnd2024:item_weapon_of_warning
        parameters:
          equipment_state: within_user_reach
          aura:
            radius_ft: 30
            origin: user
            targets:
            - self
            - allies
            combat_start:
              wake_natural_sleep: true
              wake_magical_sleep: false
            initiative_advantage: true
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_weapons
    contract:
      id: dnd2024:item_weapon_of_warning
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - combat.started
      - initiative.pending
      - position.changed
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - spatial_state
    requires:
    - dnd2024:item_weapon_of_warning
    citations:
    - SRD 5.2.1 pp. 253; builtin/srd-5.2.1/magic_item/weapon_of_warning.md
  dnd2024:items/winged_boots:
    source_sha256: b28db5bc168c5f66d417749c43b37d925dd84e4b4978298e18ccc768c9e43433
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:items/winged_boots
        implementation: dnd2024:item_winged_boots
        parameters:
          equipment_state: worn
          charges:
            maximum: 4
            initial: 4
            recovery:
              event: dawn
              amount:
                expression: 1d4
                scope: use
              cap: maximum
          activate:
            action: magic
            charge_cost: 1
            fly_speed_ft: 30
            duration:
              amount: 1
              unit: hour
          expires_while_flying:
            descent_ft_per_round: 30
            until: landed
          attunement:
            required: true
          common_rules:
          - key: dnd2024:rules/magic_item_categories_wondrous_items
    contract:
      id: dnd2024:item_winged_boots
      inputs:
        actor: participant
        source_item: item_instance
        targets:
          type: participant_list
          required: false
        objects:
          type: object_list
          required: false
        locations:
          type: location_list
          required: false
        choice:
          type: choice
          required: false
        event:
          type: event
          required: false
        campaign:
          type: campaign
          required: false
      outputs:
      - item_state
      - effects
      - resolutions
      events:
      - action.magic_pending
      - time.dawn
      - time.elapsed
      - round.ended
      - attunement.changed
      capabilities:
      - inventory
      - effects
      - resources_and_time
    requires:
    - dnd2024:item_winged_boots
    citations:
    - SRD 5.2.1 pp. 253; builtin/srd-5.2.1/magic_item/winged_boots.md
---

# SRD magic items: group 2

All 86 assigned items are specified. Each definition cites its full named SRD source; SHA-256 values identify those source files. Spell grants inherit the canonical spell definition and the common SRD item-casting rules, with the item exceptions shown in the contract. The parent importer retains full item headers and base equipment restrictions.

The Apparatus of the Crab source extraction splits its lever table. Its ten Down entries occur at the end of the Armor of Resistance note. The complete page 210 layout confirms their assignment; this contract restores them to the apparatus. Armor of Resistance includes only the armor rules.

Tables use inclusive numeric bounds and treat 00 on percentile dice as 100. Dice records express independent source-authorized draws. GM decisions are restricted typed choices, including summoned forms, treasure, planes, geography, and oversized heads. Exact geometry and visibility facts are supplied through location and choice inputs; the contracts do not implement a map. Item instance state tracks consumables, cumulative age-potion risk, used cards, sworn enemies, lifetime absorption, stored energy, air, equipment state, and recovery.

For Staff of the Magi overflow, charges after absorption are the explosion basis: the source says absorption gains the spell level, then checks for a total above 50. For the rod, initial stored energy counts toward lifetime absorption. The normal item-casting rules supply components, concentration, and casting time unless the item explicitly overrides them. No engine implementation is included.

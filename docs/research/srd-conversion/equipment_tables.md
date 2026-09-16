---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:equipment/artisan_s_tools_alchemist_s_supplies_50_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 50
          unit: gp
        mass:
          value: 8
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_alchemist_s_supplies_50_gp
        implementation: dnd2024:equipment_artisan_s_tools_alchemist_s_supplies_50_gp
        parameters:
          tool:
            ability: intelligence
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: identify_substance
              check:
                ability: intelligence
                dc: 15
            - action: utilize
              operation: ignite_fire
              check:
                ability: intelligence
                dc: 15
            craftable:
            - key: dnd2024:equipment/acid_25_gp
            - key: dnd2024:equipment/alchemist_s_fire_50_gp
            - key: dnd2024:equipment/component_pouch_25_gp
            - key: dnd2024:equipment/oil_1_sp
            - key: dnd2024:equipment/paper_2_sp
            - key: dnd2024:equipment/perfume_5_gp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 8
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_alchemist_s_supplies_50_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_alchemist_s_supplies_50_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_brewer_s_supplies_20_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 20
          unit: gp
        mass:
          value: 9
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_brewer_s_supplies_20_gp
        implementation: dnd2024:equipment_artisan_s_tools_brewer_s_supplies_20_gp
        parameters:
          tool:
            ability: intelligence
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: detect_poisoned_drink
              check:
                ability: intelligence
                dc: 15
            - action: utilize
              operation: identify_alcohol
              check:
                ability: intelligence
                dc: 10
            craftable:
            - key: dnd2024:equipment/antitoxin_50_gp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 9
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_brewer_s_supplies_20_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_brewer_s_supplies_20_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_calligrapher_s_supplies_10_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 10
          unit: gp
        mass:
          value: 5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_calligrapher_s_supplies_10_gp
        implementation: dnd2024:equipment_artisan_s_tools_calligrapher_s_supplies_10_gp
        parameters:
          tool:
            ability: dexterity
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: write_forgery_resistant_flourishes
              check:
                ability: dexterity
                dc: 15
            craftable:
            - key: dnd2024:equipment/ink_10_gp
            - key: dnd2024:equipment/spell_scroll_cantrip_30_gp_level_1_50_gp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 5
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_calligrapher_s_supplies_10_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_calligrapher_s_supplies_10_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_carpenter_s_tools_8_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 8
          unit: gp
        mass:
          value: 6
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_carpenter_s_tools_8_gp
        implementation: dnd2024:equipment_artisan_s_tools_carpenter_s_tools_8_gp
        parameters:
          tool:
            ability: strength
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: seal_door_or_container
              check:
                ability: strength
                dc: 20
            - action: utilize
              operation: pry_open_door_or_container
              check:
                ability: strength
                dc: 20
            craftable:
            - key: dnd2024:equipment/club
            - key: dnd2024:equipment/greatclub
            - key: dnd2024:equipment/quarterstaff
            - key: dnd2024:equipment/barrel_2_gp
            - key: dnd2024:equipment/chest_5_gp
            - key: dnd2024:equipment/ladder_1_sp
            - key: dnd2024:equipment/pole_5_cp
            - key: dnd2024:equipment/ram_portable_4_gp
            - key: dnd2024:equipment/torch_1_cp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 6
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_carpenter_s_tools_8_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_carpenter_s_tools_8_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_cartographer_s_tools_15_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 15
          unit: gp
        mass:
          value: 6
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_cartographer_s_tools_15_gp
        implementation: dnd2024:equipment_artisan_s_tools_cartographer_s_tools_15_gp
        parameters:
          tool:
            ability: wisdom
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: draft_map
              check:
                ability: wisdom
                dc: 15
              area_size: small
            craftable:
            - key: dnd2024:equipment/map_1_gp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 6
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_cartographer_s_tools_15_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_cartographer_s_tools_15_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_cobbler_s_tools_5_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: gp
        mass:
          value: 5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_cobbler_s_tools_5_gp
        implementation: dnd2024:equipment_artisan_s_tools_cobbler_s_tools_5_gp
        parameters:
          tool:
            ability: dexterity
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: modify_footwear
              check:
                ability: dexterity
                dc: 10
              effect:
                roll: ability_check
                mode: advantage
                ability: dexterity
                skill: acrobatics
                target: wearer
                uses: 1
            craftable:
            - key: dnd2024:equipment/climber_s_kit_25_gp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 5
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_cobbler_s_tools_5_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_cobbler_s_tools_5_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_cook_s_utensils_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 8
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_cook_s_utensils_1_gp
        implementation: dnd2024:equipment_artisan_s_tools_cook_s_utensils_1_gp
        parameters:
          tool:
            ability: wisdom
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: improve_food_flavor
              check:
                ability: wisdom
                dc: 10
            - action: utilize
              operation: detect_spoiled_or_poisoned_food
              check:
                ability: wisdom
                dc: 15
            craftable:
            - key: dnd2024:equipment/rations_5_sp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 8
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_cook_s_utensils_1_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_cook_s_utensils_1_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_glassblower_s_tools_30_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 30
          unit: gp
        mass:
          value: 5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_glassblower_s_tools_30_gp
        implementation: dnd2024:equipment_artisan_s_tools_glassblower_s_tools_30_gp
        parameters:
          tool:
            ability: intelligence
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: identify_previous_contents
              check:
                ability: intelligence
                dc: 15
              material: glass
              lookback:
                value: 24
                unit: hour
            craftable:
            - key: dnd2024:equipment/bottle_glass_2_gp
            - key: dnd2024:equipment/magnifying_glass_100_gp
            - key: dnd2024:equipment/spyglass_1_000_gp
            - key: dnd2024:equipment/vial_1_gp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 5
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_glassblower_s_tools_30_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_glassblower_s_tools_30_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_jeweler_s_tools_25_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 25
          unit: gp
        mass:
          value: 2
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_jeweler_s_tools_25_gp
        implementation: dnd2024:equipment_artisan_s_tools_jeweler_s_tools_25_gp
        parameters:
          tool:
            ability: intelligence
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: appraise_gem
              check:
                ability: intelligence
                dc: 15
            craftable:
            - key: dnd2024:equipment/arcane_focus_varies
            - key: dnd2024:equipment/holy_symbol_varies
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 2
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_jeweler_s_tools_25_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_jeweler_s_tools_25_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_leatherworker_s_tools_5_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: gp
        mass:
          value: 5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_leatherworker_s_tools_5_gp
        implementation: dnd2024:equipment_artisan_s_tools_leatherworker_s_tools_5_gp
        parameters:
          tool:
            ability: dexterity
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: decorate_leather
              check:
                ability: dexterity
                dc: 10
            craftable:
            - key: dnd2024:equipment/sling
            - key: dnd2024:equipment/whip
            - key: dnd2024:equipment/hide_armor
            - key: dnd2024:equipment/leather_armor
            - key: dnd2024:equipment/studded_leather_armor
            - key: dnd2024:equipment/backpack_2_gp
            - key: dnd2024:equipment/case_crossbow_bolt_1_gp
            - key: dnd2024:equipment/case_map_or_scroll_1_gp
            - key: dnd2024:equipment/parchment_1_sp
            - key: dnd2024:equipment/pouch_5_sp
            - key: dnd2024:equipment/quiver_1_gp
            - key: dnd2024:equipment/waterskin_2_sp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 5
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_leatherworker_s_tools_5_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_leatherworker_s_tools_5_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_mason_s_tools_10_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 10
          unit: gp
        mass:
          value: 8
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_mason_s_tools_10_gp
        implementation: dnd2024:equipment_artisan_s_tools_mason_s_tools_10_gp
        parameters:
          tool:
            ability: strength
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: chisel_symbol_in_stone
              check:
                ability: strength
                dc: 10
            - action: utilize
              operation: chisel_hole_in_stone
              check:
                ability: strength
                dc: 10
            craftable:
            - key: dnd2024:equipment/block_and_tackle_1_gp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 8
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_mason_s_tools_10_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_mason_s_tools_10_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_painter_s_supplies_10_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 10
          unit: gp
        mass:
          value: 5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_painter_s_supplies_10_gp
        implementation: dnd2024:equipment_artisan_s_tools_painter_s_supplies_10_gp
        parameters:
          tool:
            ability: wisdom
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: paint_recognizable_image
              check:
                ability: wisdom
                dc: 10
              requires: subject_previously_seen
            craftable:
            - key: dnd2024:equipment/druidic_focus_varies
            - key: dnd2024:equipment/holy_symbol_varies
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 5
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_painter_s_supplies_10_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_painter_s_supplies_10_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_potter_s_tools_10_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 10
          unit: gp
        mass:
          value: 3
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_potter_s_tools_10_gp
        implementation: dnd2024:equipment_artisan_s_tools_potter_s_tools_10_gp
        parameters:
          tool:
            ability: intelligence
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: identify_previous_contents
              check:
                ability: intelligence
                dc: 15
              material: ceramic
              lookback:
                value: 24
                unit: hour
            craftable:
            - key: dnd2024:equipment/jug_2_cp
            - key: dnd2024:equipment/lamp_5_sp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 3
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_potter_s_tools_10_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_potter_s_tools_10_gp
    citations:
    - SRD 5.2.1 pp. 93; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_smith_s_tools_20_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 20
          unit: gp
        mass:
          value: 8
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_smith_s_tools_20_gp
        implementation: dnd2024:equipment_artisan_s_tools_smith_s_tools_20_gp
        parameters:
          tool:
            ability: strength
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: pry_open_door_or_container
              check:
                ability: strength
                dc: 20
            craftable:
            - key: dnd2024:equipment/ball_bearings_1_gp
            - key: dnd2024:equipment/bucket_5_cp
            - key: dnd2024:equipment/caltrops_1_gp
            - key: dnd2024:equipment/chain_5_gp
            - key: dnd2024:equipment/crowbar_2_gp
            - key: dnd2024:equipment/grappling_hook_2_gp
            - key: dnd2024:equipment/pot_iron_2_gp
            - key: dnd2024:equipment/spikes_iron_1_gp
            - key: dnd2024:equipment/dagger
            - key: dnd2024:equipment/handaxe
            - key: dnd2024:equipment/javelin
            - key: dnd2024:equipment/light_hammer
            - key: dnd2024:equipment/mace
            - key: dnd2024:equipment/sickle
            - key: dnd2024:equipment/spear
            - key: dnd2024:equipment/battleaxe
            - key: dnd2024:equipment/flail
            - key: dnd2024:equipment/glaive
            - key: dnd2024:equipment/greataxe
            - key: dnd2024:equipment/greatsword
            - key: dnd2024:equipment/halberd
            - key: dnd2024:equipment/lance
            - key: dnd2024:equipment/longsword
            - key: dnd2024:equipment/maul
            - key: dnd2024:equipment/morningstar
            - key: dnd2024:equipment/pike
            - key: dnd2024:equipment/rapier
            - key: dnd2024:equipment/scimitar
            - key: dnd2024:equipment/shortsword
            - key: dnd2024:equipment/trident
            - key: dnd2024:equipment/warhammer
            - key: dnd2024:equipment/war_pick
            - key: dnd2024:equipment/chain_shirt
            - key: dnd2024:equipment/scale_mail
            - key: dnd2024:equipment/breastplate
            - key: dnd2024:equipment/half_plate_armor
            - key: dnd2024:equipment/ring_mail
            - key: dnd2024:equipment/chain_mail
            - key: dnd2024:equipment/splint_armor
            - key: dnd2024:equipment/plate_armor
            - key: dnd2024:equipment/ammunition_varies
              variant: firearm_bullets
            - key: dnd2024:equipment/ammunition_varies
              variant: sling_bullets
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 8
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_smith_s_tools_20_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_smith_s_tools_20_gp
    citations:
    - SRD 5.2.1 pp. 93, 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_tinker_s_tools_50_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 50
          unit: gp
        mass:
          value: 10
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_tinker_s_tools_50_gp
        implementation: dnd2024:equipment_artisan_s_tools_tinker_s_tools_50_gp
        parameters:
          tool:
            ability: dexterity
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: assemble_scrap_item
              check:
                ability: dexterity
                dc: 20
              size: tiny
              expires:
                value: 1
                unit: minute
              expiry: falls_apart
            craftable:
            - key: dnd2024:equipment/musket
            - key: dnd2024:equipment/pistol
            - key: dnd2024:equipment/bell_1_gp
            - key: dnd2024:equipment/lantern_bullseye_10_gp
            - key: dnd2024:equipment/flask_2_cp
            - key: dnd2024:equipment/lantern_hooded_5_gp
            - key: dnd2024:equipment/hunting_trap_5_gp
            - key: dnd2024:equipment/lock_10_gp
            - key: dnd2024:equipment/manacles_2_gp
            - key: dnd2024:equipment/mirror_5_gp
            - key: dnd2024:equipment/shovel_2_gp
            - key: dnd2024:equipment/signal_whistle_5_cp
            - key: dnd2024:equipment/tinderbox_5_sp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 10
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_tinker_s_tools_50_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_tinker_s_tools_50_gp
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_weaver_s_tools_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_weaver_s_tools_1_gp
        implementation: dnd2024:equipment_artisan_s_tools_weaver_s_tools_1_gp
        parameters:
          tool:
            ability: dexterity
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: mend_clothing_tear
              check:
                ability: dexterity
                dc: 10
            - action: utilize
              operation: sew_design
              check:
                ability: dexterity
                dc: 10
              size: tiny
            craftable:
            - key: dnd2024:equipment/padded_armor
            - key: dnd2024:equipment/basket_4_sp
            - key: dnd2024:equipment/bedroll_1_gp
            - key: dnd2024:equipment/blanket_5_sp
            - key: dnd2024:equipment/clothes_fine_15_gp
            - key: dnd2024:equipment/net_1_gp
            - key: dnd2024:equipment/robe_1_gp
            - key: dnd2024:equipment/rope_1_gp
            - key: dnd2024:equipment/sack_1_cp
            - key: dnd2024:equipment/string_1_sp
            - key: dnd2024:equipment/tent_2_gp
            - key: dnd2024:equipment/clothes_traveler_s_2_gp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 5
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_weaver_s_tools_1_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_weaver_s_tools_1_gp
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/artisan_s_tools_woodcarver_s_tools_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/artisan_s_tools_woodcarver_s_tools_1_gp
        implementation: dnd2024:equipment_artisan_s_tools_woodcarver_s_tools_1_gp
        parameters:
          tool:
            ability: dexterity
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: carve_wood_pattern
              check:
                ability: dexterity
                dc: 10
            craftable:
            - key: dnd2024:equipment/club
            - key: dnd2024:equipment/greatclub
            - key: dnd2024:equipment/quarterstaff
            - key: dnd2024:equipment/arcane_focus_varies
            - key: dnd2024:equipment/druidic_focus_varies
            - key: dnd2024:equipment/ink_pen_2_cp
            - key: dnd2024:equipment/dart
            - key: dnd2024:equipment/light_crossbow
            - key: dnd2024:equipment/shortbow
            - key: dnd2024:equipment/blowgun
            - key: dnd2024:equipment/hand_crossbow
            - key: dnd2024:equipment/heavy_crossbow
            - key: dnd2024:equipment/longbow
            - key: dnd2024:equipment/ammunition_varies
              variant: arrows
            - key: dnd2024:equipment/ammunition_varies
              variant: bolts
            - key: dnd2024:equipment/ammunition_varies
              variant: needles
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 5
            unit: lb
    contract:
      id: dnd2024:equipment_artisan_s_tools_woodcarver_s_tools_1_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_artisan_s_tools_woodcarver_s_tools_1_gp
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/other_tools_disguise_kit_25_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 25
          unit: gp
        mass:
          value: 3
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/other_tools_disguise_kit_25_gp
        implementation: dnd2024:equipment_other_tools_disguise_kit_25_gp
        parameters:
          tool:
            ability: charisma
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: apply_makeup
              check:
                ability: charisma
                dc: 10
            craftable:
            - key: dnd2024:equipment/costume_5_gp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 3
            unit: lb
    contract:
      id: dnd2024:equipment_other_tools_disguise_kit_25_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_other_tools_disguise_kit_25_gp
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/other_tools_forgery_kit_15_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 15
          unit: gp
        mass:
          value: 5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/other_tools_forgery_kit_15_gp
        implementation: dnd2024:equipment_other_tools_forgery_kit_15_gp
        parameters:
          tool:
            ability: dexterity
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: mimic_handwriting
              check:
                ability: dexterity
                dc: 15
              maximum_words: 10
            - action: utilize
              operation: duplicate_wax_seal
              check:
                ability: dexterity
                dc: 20
            craftable: []
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 5
            unit: lb
    contract:
      id: dnd2024:equipment_other_tools_forgery_kit_15_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_other_tools_forgery_kit_15_gp
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/other_tools_gaming_set_varies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
      splat_rule:
        mechanic: dnd2024:equipment/other_tools_gaming_set_varies
        implementation: dnd2024:equipment_other_tools_gaming_set_varies
        parameters:
          tool:
            ability: wisdom
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: detect_cheating
              check:
                ability: wisdom
                dc: 10
            - action: utilize
              operation: win_game
              check:
                ability: wisdom
                dc: 20
            craftable: []
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          variants:
          - id: dice
            price:
              value: 1
              unit: sp
            mass:
              value: 0
              unit: lb
              negligible: true
          - id: dragonchess
            price:
              value: 1
              unit: gp
            mass:
              value: 0
              unit: lb
              negligible: true
          - id: playing_cards
            price:
              value: 5
              unit: sp
            mass:
              value: 0
              unit: lb
              negligible: true
          - id: three_dragon_ante
            price:
              value: 1
              unit: gp
            mass:
              value: 0
              unit: lb
              negligible: true
    contract:
      id: dnd2024:equipment_other_tools_gaming_set_varies
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_other_tools_gaming_set_varies
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/other_tools_herbalism_kit_5_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: gp
        mass:
          value: 3
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/other_tools_herbalism_kit_5_gp
        implementation: dnd2024:equipment_other_tools_herbalism_kit_5_gp
        parameters:
          tool:
            ability: intelligence
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: identify_plant
              check:
                ability: intelligence
                dc: 10
            craftable:
            - key: dnd2024:equipment/antitoxin_50_gp
            - key: dnd2024:equipment/candle_1_cp
            - key: dnd2024:equipment/healer_s_kit_5_gp
            - key: dnd2024:equipment/potion_of_healing_50_gp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 3
            unit: lb
    contract:
      id: dnd2024:equipment_other_tools_herbalism_kit_5_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_other_tools_herbalism_kit_5_gp
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/other_tools_musical_instrument_varies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
      splat_rule:
        mechanic: dnd2024:equipment/other_tools_musical_instrument_varies
        implementation: dnd2024:equipment_other_tools_musical_instrument_varies
        parameters:
          tool:
            ability: charisma
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: play_known_tune
              check:
                ability: charisma
                dc: 10
            - action: utilize
              operation: improvise_song
              check:
                ability: charisma
                dc: 15
            craftable: []
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          variants:
          - id: bagpipes
            price:
              value: 30
              unit: gp
            mass:
              value: 6
              unit: lb
          - id: drum
            price:
              value: 6
              unit: gp
            mass:
              value: 3
              unit: lb
          - id: dulcimer
            price:
              value: 25
              unit: gp
            mass:
              value: 10
              unit: lb
          - id: flute
            price:
              value: 2
              unit: gp
            mass:
              value: 1
              unit: lb
          - id: horn
            price:
              value: 3
              unit: gp
            mass:
              value: 2
              unit: lb
          - id: lute
            price:
              value: 35
              unit: gp
            mass:
              value: 2
              unit: lb
          - id: lyre
            price:
              value: 30
              unit: gp
            mass:
              value: 2
              unit: lb
          - id: pan_flute
            price:
              value: 12
              unit: gp
            mass:
              value: 2
              unit: lb
          - id: shawm
            price:
              value: 2
              unit: gp
            mass:
              value: 1
              unit: lb
          - id: viol
            price:
              value: 30
              unit: gp
            mass:
              value: 1
              unit: lb
    contract:
      id: dnd2024:equipment_other_tools_musical_instrument_varies
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_other_tools_musical_instrument_varies
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/other_tools_navigator_s_tools_25_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 25
          unit: gp
        mass:
          value: 2
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/other_tools_navigator_s_tools_25_gp
        implementation: dnd2024:equipment_other_tools_navigator_s_tools_25_gp
        parameters:
          tool:
            ability: wisdom
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: plot_course
              check:
                ability: wisdom
                dc: 10
            - action: utilize
              operation: determine_position_by_stargazing
              check:
                ability: wisdom
                dc: 15
            craftable: []
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 2
            unit: lb
    contract:
      id: dnd2024:equipment_other_tools_navigator_s_tools_25_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_other_tools_navigator_s_tools_25_gp
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/other_tools_poisoner_s_kit_50_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 50
          unit: gp
        mass:
          value: 2
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/other_tools_poisoner_s_kit_50_gp
        implementation: dnd2024:equipment_other_tools_poisoner_s_kit_50_gp
        parameters:
          tool:
            ability: intelligence
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: detect_poisoned_object
              check:
                ability: intelligence
                dc: 10
            craftable:
            - key: dnd2024:equipment/poison_basic_100_gp
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 2
            unit: lb
    contract:
      id: dnd2024:equipment_other_tools_poisoner_s_kit_50_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_other_tools_poisoner_s_kit_50_gp
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/other_tools_thieves_tools_25_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 25
          unit: gp
        mass:
          value: 1
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/other_tools_thieves_tools_25_gp
        implementation: dnd2024:equipment_other_tools_thieves_tools_25_gp
        parameters:
          tool:
            ability: dexterity
            proficiency_bonus_if_proficient: true
            skill_and_tool_proficiency_grants_advantage: true
            utilize:
            - action: utilize
              operation: pick_lock
              check:
                ability: dexterity
                dc: 15
            - action: utilize
              operation: disarm_trap
              check:
                ability: dexterity
                dc: 15
            craftable: []
            monster_proficient_if_tool_in_stat_block: true
            proficiency_rule:
              key: dnd2024:rules/tools_tool_proficiency
            crafting_rule:
              key: dnd2024:rules/crafting_nonmagical_items_tools
          mass:
            value: 1
            unit: lb
    contract:
      id: dnd2024:equipment_other_tools_thieves_tools_25_gp
      inputs:
        actor: participant
        item: item_instance
        operation: choice
        target:
          type: object
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - check_requests
      - tool_effects
      - crafting_options
      events:
      - action.utilize
      capabilities:
      - tool_proficiency
      - ability_checks
      - crafting
      - character_choices
    requires:
    - dnd2024:equipment_other_tools_thieves_tools_25_gp
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/acid_25_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 25
          unit: gp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/acid_25_gp
        implementation: dnd2024:equipment_acid_25_gp
        parameters:
          use:
            economy: replace_one_attack
            action: attack
            consume: 1
            target:
              count: 1
              kinds:
              - creature
              - object
              visible: true
              range_ft: 20
            save:
              ability: dexterity
              dc:
                op: add
                operands:
                - 8
                - ability_modifier: dexterity
                - stat: proficiency_bonus
              success: no_effect
              failure:
                damage:
                  amount:
                    dice:
                      count: 2
                      sides: 6
                    bonus: 0
                  type: acid
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_acid_25_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - attack.replacement
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_acid_25_gp
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/alchemist_s_fire_50_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 50
          unit: gp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/alchemist_s_fire_50_gp
        implementation: dnd2024:equipment_alchemist_s_fire_50_gp
        parameters:
          use:
            economy: replace_one_attack
            action: attack
            consume: 1
            target:
              count: 1
              kinds:
              - creature
              - object
              visible: true
              range_ft: 20
            save:
              ability: dexterity
              dc:
                op: add
                operands:
                - 8
                - ability_modifier: dexterity
                - stat: proficiency_bonus
              success: no_effect
              failure:
                damage:
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    bonus: 0
                  type: fire
                apply_rule:
                  key: dnd2024:glossary/burning_hazard
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_alchemist_s_fire_50_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - attack.replacement
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_alchemist_s_fire_50_gp
    citations:
    - SRD 5.2.1 pp. 94; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/ammunition_varies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
      splat_rule:
        mechanic: dnd2024:equipment/ammunition_varies
        implementation: dnd2024:equipment_ammunition_varies
        parameters:
          required_by_property: ammunition
          weapon_selects_type: true
          storage_included: false
          variants:
          - id: arrows
            purchase_quantity: 20
            storage:
              key: dnd2024:equipment/quiver_1_gp
            mass:
              value: 1
              unit: lb
            price:
              value: 1
              unit: gp
          - id: bolts
            purchase_quantity: 20
            storage:
              key: dnd2024:equipment/case_crossbow_bolt_1_gp
            mass:
              value: 1.5
              unit: lb
            price:
              value: 1
              unit: gp
          - id: firearm_bullets
            purchase_quantity: 10
            storage:
              key: dnd2024:equipment/pouch_5_sp
            mass:
              value: 2
              unit: lb
            price:
              value: 3
              unit: gp
          - id: sling_bullets
            purchase_quantity: 20
            storage:
              key: dnd2024:equipment/pouch_5_sp
            mass:
              value: 1.5
              unit: lb
            price:
              value: 4
              unit: cp
          - id: needles
            purchase_quantity: 50
            storage:
              key: dnd2024:equipment/pouch_5_sp
            mass:
              value: 1
              unit: lb
            price:
              value: 1
              unit: gp
    contract:
      id: dnd2024:equipment_ammunition_varies
      inputs:
        item: item_instance
        variant: choice
        weapon:
          type: item_instance
          required: false
      outputs:
      - variant_properties
      - ammunition_compatibility
      - purchase_quantities
      events: []
      capabilities:
      - ammunition
      - item_variants
      - inventory
    requires:
    - dnd2024:equipment_ammunition_varies
    citations:
    - SRD 5.2.1 pp. 94, 95, 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/antitoxin_50_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 50
          unit: gp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/antitoxin_50_gp
        implementation: dnd2024:equipment_antitoxin_50_gp
        parameters:
          use:
            economy: bonus_action
            consume: 1
            target: self
            operation: drink
            effect:
              roll: saving_throw
              mode: advantage
              scope: avoid_or_end_poisoned
              duration:
                value: 1
                unit: hour
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_antitoxin_50_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_antitoxin_50_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/arcane_focus_varies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
      splat_rule:
        mechanic: dnd2024:equipment/arcane_focus_varies
        implementation: dnd2024:equipment_arcane_focus_varies
        parameters:
          spellcasting_focus:
            eligible_classes:
            - key: dnd2024:classes/sorcerer
            - key: dnd2024:classes/warlock
            - key: dnd2024:classes/wizard
            rule:
              key: dnd2024:glossary/spellcasting_focus
          variants:
          - id: crystal
            mass:
              value: 1
              unit: lb
            price:
              value: 10
              unit: gp
            valid_use_states:
            - held
          - id: orb
            mass:
              value: 3
              unit: lb
            price:
              value: 20
              unit: gp
            valid_use_states:
            - held
          - id: rod
            mass:
              value: 2
              unit: lb
            price:
              value: 10
              unit: gp
            valid_use_states:
            - held
          - id: staff
            mass:
              value: 4
              unit: lb
            price:
              value: 5
              unit: gp
            also_weapon:
              key: dnd2024:equipment/quarterstaff
            valid_use_states:
            - held
          - id: wand
            mass:
              value: 1
              unit: lb
            price:
              value: 10
              unit: gp
            valid_use_states:
            - held
    contract:
      id: dnd2024:equipment_arcane_focus_varies
      inputs:
        actor: participant
        item: item_instance
        variant: choice
        spell:
          type: definition
          required: false
      outputs:
      - focus_eligibility
      - variant_properties
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting_focus
      - item_variants
    requires:
    - dnd2024:equipment_arcane_focus_varies
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/backpack_2_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: gp
        mass:
          value: 5.0
          unit: lb
        containment:
          capacity:
            mass: {value: 30, unit: lb}
            volume: {value: 1, unit: ft3}
      splat_rule:
        mechanic: dnd2024:equipment/backpack_2_gp
        implementation: dnd2024:equipment_backpack_2_gp
        parameters:
          capacity:
            mass_lb: 30
            volume_ft3: 1
          also_usable_as: saddlebag
          mass:
            value: 5.0
            unit: lb
    contract:
      id: dnd2024:equipment_backpack_2_gp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_backpack_2_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/ball_bearings_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 2.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/ball_bearings_1_gp
        implementation: dnd2024:equipment_ball_bearings_1_gp
        parameters:
          deploy:
            action: utilize
            area:
              shape: square
              side_ft: 10
              range_ft: 10
              level_ground: true
            trigger: creature_enters_first_time_on_turn
            save:
              ability: dexterity
              dc: 10
              failure:
                condition: prone
              success: no_effect
          recover:
            duration:
              value: 10
              unit: minute
          mass:
            value: 2.0
            unit: lb
    contract:
      id: dnd2024:equipment_ball_bearings_1_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_ball_bearings_1_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/barrel_2_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: gp
        mass:
          value: 70.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/barrel_2_gp
        implementation: dnd2024:equipment_barrel_2_gp
        parameters:
          capacity:
            alternative_capacities:
            - liquid_gallons: 40
            - dry_volume_ft3: 4
          mass:
            value: 70.0
            unit: lb
    contract:
      id: dnd2024:equipment_barrel_2_gp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_barrel_2_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/basket_4_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 4
          unit: sp
        mass:
          value: 2.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/basket_4_sp
        implementation: dnd2024:equipment_basket_4_sp
        parameters:
          capacity:
            mass_lb: 40
            volume_ft3: 2
          mass:
            value: 2.0
            unit: lb
    contract:
      id: dnd2024:equipment_basket_4_sp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_basket_4_sp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/bedroll_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 7.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/bedroll_1_gp
        implementation: dnd2024:equipment_bedroll_1_gp
        parameters:
          sleep_capacity:
            count: 1
            sizes:
            - small
            - medium
          while_inside:
            saving_throw_scope: extreme_cold
            result: automatic_success
          mass:
            value: 7.0
            unit: lb
    contract:
      id: dnd2024:equipment_bedroll_1_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_bedroll_1_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/bell_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/bell_1_gp
        implementation: dnd2024:equipment_bell_1_gp
        parameters:
          use:
            action: utilize
            effect:
              sound_audible_range_ft: 60
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_bell_1_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_bell_1_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/blanket_5_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: sp
        mass:
          value: 3.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/blanket_5_sp
        implementation: dnd2024:equipment_blanket_5_sp
        parameters:
          while_wrapped:
            roll: saving_throw
            mode: advantage
            scope: extreme_cold
          mass:
            value: 3.0
            unit: lb
    contract:
      id: dnd2024:equipment_blanket_5_sp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_blanket_5_sp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/block_and_tackle_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 5.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/block_and_tackle_1_gp
        implementation: dnd2024:equipment_block_and_tackle_1_gp
        parameters:
          hoist_capacity:
            op: multiply
            operands:
            - 4
            - stat: normal_lift_capacity
          mass:
            value: 5.0
            unit: lb
    contract:
      id: dnd2024:equipment_block_and_tackle_1_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_block_and_tackle_1_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/book_25_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 25
          unit: gp
        mass:
          value: 5.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/book_25_gp
        implementation: dnd2024:equipment_book_25_gp
        parameters:
          contents_kind:
            choice:
            - fiction
            - nonfiction
          consult:
            requirements:
              kind: nonfiction
              accurate: true
              topic_matches_check: true
            bonus: 5
            roll: ability_check
            ability: intelligence
            skills:
            - arcana
            - history
            - nature
            - religion
          mass:
            value: 5.0
            unit: lb
    contract:
      id: dnd2024:equipment_book_25_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_book_25_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/bottle_glass_2_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: gp
        mass:
          value: 2.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/bottle_glass_2_gp
        implementation: dnd2024:equipment_bottle_glass_2_gp
        parameters:
          capacity:
            liquid_pints: 1.5
          mass:
            value: 2.0
            unit: lb
    contract:
      id: dnd2024:equipment_bottle_glass_2_gp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_bottle_glass_2_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/bucket_5_cp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: cp
        mass:
          value: 2.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/bucket_5_cp
        implementation: dnd2024:equipment_bucket_5_cp
        parameters:
          capacity:
            volume_ft3: 0.5
          mass:
            value: 2.0
            unit: lb
    contract:
      id: dnd2024:equipment_bucket_5_cp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_bucket_5_cp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/burglar_s_pack_16_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 16
          unit: gp
        mass:
          value: 42.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/burglar_s_pack_16_gp
        implementation: dnd2024:equipment_burglar_s_pack_16_gp
        parameters:
          contents:
          - item:
              key: dnd2024:equipment/backpack_2_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/ball_bearings_1_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/bell_1_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/candle_1_cp
            quantity: 10
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/crowbar_2_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/lantern_hooded_5_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/oil_1_sp
            quantity: 7
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/rations_5_sp
            quantity: 5
            unit: day
          - item:
              key: dnd2024:equipment/rope_1_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/tinderbox_5_sp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/waterskin_2_sp
            quantity: 1
            unit: purchase_unit
          grant_contents_on_acquisition: true
          mass:
            value: 42.0
            unit: lb
          grant_idempotency: once_per_acquired_pack_instance
    contract:
      id: dnd2024:equipment_burglar_s_pack_16_gp
      inputs:
        actor: participant
        item: item_instance
      outputs:
      - inventory_grants
      events:
      - item.acquired
      capabilities:
      - inventory
      - item_bundles
    requires:
    - dnd2024:equipment_burglar_s_pack_16_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/caltrops_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 2.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/caltrops_1_gp
        implementation: dnd2024:equipment_caltrops_1_gp
        parameters:
          deploy:
            action: utilize
            area:
              shape: square
              side_ft: 5
              range_ft: 5
            trigger: creature_enters_first_time_on_turn
            save:
              ability: dexterity
              dc: 15
              failure:
                damage:
                  amount: 1
                  type: piercing
                speed_set: 0
                speed_expiry: target_next_turn_start
              success: no_effect
          recover:
            duration:
              value: 10
              unit: minute
          mass:
            value: 2.0
            unit: lb
    contract:
      id: dnd2024:equipment_caltrops_1_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_caltrops_1_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/candle_1_cp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: cp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/candle_1_cp
        implementation: dnd2024:equipment_candle_1_cp
        parameters:
          light:
            requires_lit: true
            shape: radius
            bright_ft: 5
            additional_dim_ft: 5
          burn_duration:
            value: 1
            unit: hour
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_candle_1_cp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_candle_1_cp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/case_crossbow_bolt_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/case_crossbow_bolt_1_gp
        implementation: dnd2024:equipment_case_crossbow_bolt_1_gp
        parameters:
          capacity:
            ammunition_variant: bolts
            count: 20
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_case_crossbow_bolt_1_gp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_case_crossbow_bolt_1_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/case_map_or_scroll_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/case_map_or_scroll_1_gp
        implementation: dnd2024:equipment_case_map_or_scroll_1_gp
        parameters:
          capacity:
            alternative_capacities:
            - item:
                key: dnd2024:equipment/paper_2_sp
              count: 10
            - item:
                key: dnd2024:equipment/parchment_1_sp
              count: 5
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_case_map_or_scroll_1_gp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_case_map_or_scroll_1_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/chain_5_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: gp
        mass:
          value: 10.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/chain_5_gp
        implementation: dnd2024:equipment_chain_5_gp
        parameters:
          bind:
            action: utilize
            target:
              range_ft: 5
              unwilling: true
              has_any_condition:
              - grappled
              - incapacitated
              - restrained
            check:
              ability: strength
              skill: athletics
              dc: 13
            bound_legs_condition: restrained
            until: escape
          escape:
            economy: action
            check:
              ability: dexterity
              skill: acrobatics
              dc: 18
          burst:
            economy: action
            check:
              ability: strength
              skill: athletics
              dc: 20
          mass:
            value: 10.0
            unit: lb
    contract:
      id: dnd2024:equipment_chain_5_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_chain_5_gp
    citations:
    - SRD 5.2.1 pp. 96; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/chest_5_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: gp
        mass:
          value: 25.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/chest_5_gp
        implementation: dnd2024:equipment_chest_5_gp
        parameters:
          capacity:
            volume_ft3: 12
          mass:
            value: 25.0
            unit: lb
    contract:
      id: dnd2024:equipment_chest_5_gp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_chest_5_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/climber_s_kit_25_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 25
          unit: gp
        mass:
          value: 12.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/climber_s_kit_25_gp
        implementation: dnd2024:equipment_climber_s_kit_25_gp
        parameters:
          components:
          - boot_tips
          - gloves
          - pitons
          - harness
          anchor:
            action: utilize
            target: self
            maximum_fall_ft: 25
            maximum_distance_from_anchor_ft: 25
          unanchor:
            economy: bonus_action
          mass:
            value: 12.0
            unit: lb
    contract:
      id: dnd2024:equipment_climber_s_kit_25_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_climber_s_kit_25_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/clothes_fine_15_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 15
          unit: gp
        mass:
          value: 6.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/clothes_fine_15_gp
        implementation: dnd2024:equipment_clothes_fine_15_gp
        parameters:
          access_requirement:
            authority: dm
            scope:
            - event
            - location
            eligible_if_wearing: true
          mass:
            value: 6.0
            unit: lb
    contract:
      id: dnd2024:equipment_clothes_fine_15_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_clothes_fine_15_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/clothes_traveler_s_2_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: gp
        mass:
          value: 4.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/clothes_traveler_s_2_gp
        implementation: dnd2024:equipment_clothes_traveler_s_2_gp
        parameters:
          garment:
            purpose: travel
            resilient: true
          mass:
            value: 4.0
            unit: lb
    contract:
      id: dnd2024:equipment_clothes_traveler_s_2_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_clothes_traveler_s_2_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/component_pouch_25_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 25
          unit: gp
        mass:
          value: 2.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/component_pouch_25_gp
        implementation: dnd2024:equipment_component_pouch_25_gp
        parameters:
          watertight: true
          material_components:
            available: all_free_components_of_owner_spells
            priced_components_included: false
            rule:
              key: dnd2024:rules/spellcasting
          mass:
            value: 2.0
            unit: lb
    contract:
      id: dnd2024:equipment_component_pouch_25_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_component_pouch_25_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/costume_5_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: gp
        mass:
          value: 4.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/costume_5_gp
        implementation: dnd2024:equipment_costume_5_gp
        parameters:
          while_worn:
            roll: ability_check
            mode: advantage
            scope: impersonate_represented_person_or_person_type
          mass:
            value: 4.0
            unit: lb
    contract:
      id: dnd2024:equipment_costume_5_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_costume_5_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/crowbar_2_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: gp
        mass:
          value: 5.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/crowbar_2_gp
        implementation: dnd2024:equipment_crowbar_2_gp
        parameters:
          using:
            roll: ability_check
            mode: advantage
            ability: strength
            requires: leverage_applicable
          mass:
            value: 5.0
            unit: lb
    contract:
      id: dnd2024:equipment_crowbar_2_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_crowbar_2_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/diplomat_s_pack_39_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 39
          unit: gp
        mass:
          value: 39.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/diplomat_s_pack_39_gp
        implementation: dnd2024:equipment_diplomat_s_pack_39_gp
        parameters:
          contents:
          - item:
              key: dnd2024:equipment/chest_5_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/clothes_fine_15_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/ink_10_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/ink_pen_2_cp
            quantity: 5
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/lamp_5_sp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/case_map_or_scroll_1_gp
            quantity: 2
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/oil_1_sp
            quantity: 4
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/paper_2_sp
            quantity: 5
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/parchment_1_sp
            quantity: 5
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/perfume_5_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/tinderbox_5_sp
            quantity: 1
            unit: purchase_unit
          grant_contents_on_acquisition: true
          mass:
            value: 39.0
            unit: lb
          grant_idempotency: once_per_acquired_pack_instance
    contract:
      id: dnd2024:equipment_diplomat_s_pack_39_gp
      inputs:
        actor: participant
        item: item_instance
      outputs:
      - inventory_grants
      events:
      - item.acquired
      capabilities:
      - inventory
      - item_bundles
    requires:
    - dnd2024:equipment_diplomat_s_pack_39_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/druidic_focus_varies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
      splat_rule:
        mechanic: dnd2024:equipment/druidic_focus_varies
        implementation: dnd2024:equipment_druidic_focus_varies
        parameters:
          spellcasting_focus:
            eligible_classes:
            - key: dnd2024:classes/druid
            - key: dnd2024:classes/ranger
            rule:
              key: dnd2024:glossary/spellcasting_focus
          variants:
          - id: sprig_of_mistletoe
            mass:
              value: 0
              unit: lb
            price:
              value: 1
              unit: gp
            valid_use_states:
            - held
          - id: wooden_staff
            mass:
              value: 4
              unit: lb
            price:
              value: 5
              unit: gp
            also_weapon:
              key: dnd2024:equipment/quarterstaff
            valid_use_states:
            - held
          - id: yew_wand
            mass:
              value: 1
              unit: lb
            price:
              value: 10
              unit: gp
            valid_use_states:
            - held
    contract:
      id: dnd2024:equipment_druidic_focus_varies
      inputs:
        actor: participant
        item: item_instance
        variant: choice
        spell:
          type: definition
          required: false
      outputs:
      - focus_eligibility
      - variant_properties
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting_focus
      - item_variants
    requires:
    - dnd2024:equipment_druidic_focus_varies
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/dungeoneer_s_pack_12_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 12
          unit: gp
        mass:
          value: 55.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/dungeoneer_s_pack_12_gp
        implementation: dnd2024:equipment_dungeoneer_s_pack_12_gp
        parameters:
          contents:
          - item:
              key: dnd2024:equipment/backpack_2_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/caltrops_1_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/crowbar_2_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/oil_1_sp
            quantity: 2
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/rations_5_sp
            quantity: 10
            unit: day
          - item:
              key: dnd2024:equipment/rope_1_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/tinderbox_5_sp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/torch_1_cp
            quantity: 10
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/waterskin_2_sp
            quantity: 1
            unit: purchase_unit
          grant_contents_on_acquisition: true
          mass:
            value: 55.0
            unit: lb
          grant_idempotency: once_per_acquired_pack_instance
    contract:
      id: dnd2024:equipment_dungeoneer_s_pack_12_gp
      inputs:
        actor: participant
        item: item_instance
      outputs:
      - inventory_grants
      events:
      - item.acquired
      capabilities:
      - inventory
      - item_bundles
    requires:
    - dnd2024:equipment_dungeoneer_s_pack_12_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/entertainer_s_pack_40_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 40
          unit: gp
        mass:
          value: 58.5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/entertainer_s_pack_40_gp
        implementation: dnd2024:equipment_entertainer_s_pack_40_gp
        parameters:
          contents:
          - item:
              key: dnd2024:equipment/backpack_2_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/bedroll_1_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/bell_1_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/lantern_bullseye_10_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/costume_5_gp
            quantity: 3
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/mirror_5_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/oil_1_sp
            quantity: 8
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/rations_5_sp
            quantity: 9
            unit: day
          - item:
              key: dnd2024:equipment/tinderbox_5_sp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/waterskin_2_sp
            quantity: 1
            unit: purchase_unit
          grant_contents_on_acquisition: true
          mass:
            value: 58.5
            unit: lb
          grant_idempotency: once_per_acquired_pack_instance
    contract:
      id: dnd2024:equipment_entertainer_s_pack_40_gp
      inputs:
        actor: participant
        item: item_instance
      outputs:
      - inventory_grants
      events:
      - item.acquired
      capabilities:
      - inventory
      - item_bundles
    requires:
    - dnd2024:equipment_entertainer_s_pack_40_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/explorer_s_pack_10_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 10
          unit: gp
        mass:
          value: 55.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/explorer_s_pack_10_gp
        implementation: dnd2024:equipment_explorer_s_pack_10_gp
        parameters:
          contents:
          - item:
              key: dnd2024:equipment/backpack_2_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/bedroll_1_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/oil_1_sp
            quantity: 2
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/rations_5_sp
            quantity: 10
            unit: day
          - item:
              key: dnd2024:equipment/rope_1_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/tinderbox_5_sp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/torch_1_cp
            quantity: 10
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/waterskin_2_sp
            quantity: 1
            unit: purchase_unit
          grant_contents_on_acquisition: true
          mass:
            value: 55.0
            unit: lb
          grant_idempotency: once_per_acquired_pack_instance
    contract:
      id: dnd2024:equipment_explorer_s_pack_10_gp
      inputs:
        actor: participant
        item: item_instance
      outputs:
      - inventory_grants
      events:
      - item.acquired
      capabilities:
      - inventory
      - item_bundles
    requires:
    - dnd2024:equipment_explorer_s_pack_10_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/flask_2_cp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: cp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/flask_2_cp
        implementation: dnd2024:equipment_flask_2_cp
        parameters:
          capacity:
            liquid_pints: 1
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_flask_2_cp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_flask_2_cp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/grappling_hook_2_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: gp
        mass:
          value: 4.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/grappling_hook_2_gp
        implementation: dnd2024:equipment_grappling_hook_2_gp
        parameters:
          throw:
            action: utilize
            range_ft: 50
            target_kinds:
            - railing
            - ledge
            - other_catch
            check:
              ability: dexterity
              skill: acrobatics
              dc: 13
            success: hook_catches
          climb:
            requires: rope_tied_to_caught_hook
            operation: climb_rope
          mass:
            value: 4.0
            unit: lb
    contract:
      id: dnd2024:equipment_grappling_hook_2_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - attack.replacement
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_grappling_hook_2_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/healer_s_kit_5_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: gp
        mass:
          value: 3.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/healer_s_kit_5_gp
        implementation: dnd2024:equipment_healer_s_kit_5_gp
        parameters:
          uses: 10
          use:
            action: utilize
            expend_uses: 1
            target:
              condition: unconscious
              hit_points: 0
            effect: stabilize
            medicine_check_required: false
          mass:
            value: 3.0
            unit: lb
    contract:
      id: dnd2024:equipment_healer_s_kit_5_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_healer_s_kit_5_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/holy_symbol_varies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
      splat_rule:
        mechanic: dnd2024:equipment/holy_symbol_varies
        implementation: dnd2024:equipment_holy_symbol_varies
        parameters:
          spellcasting_focus:
            eligible_classes:
            - key: dnd2024:classes/cleric
            - key: dnd2024:classes/paladin
            rule:
              key: dnd2024:glossary/spellcasting_focus
          variants:
          - id: amulet
            mass:
              value: 1
              unit: lb
            price:
              value: 5
              unit: gp
            valid_use_states:
            - worn
            - held
          - id: emblem
            mass:
              value: 0
              unit: lb
            price:
              value: 5
              unit: gp
            valid_use_states:
            - borne_on_fabric
            - borne_on_shield
          - id: reliquary
            mass:
              value: 2
              unit: lb
            price:
              value: 5
              unit: gp
            valid_use_states:
            - held
    contract:
      id: dnd2024:equipment_holy_symbol_varies
      inputs:
        actor: participant
        item: item_instance
        variant: choice
        spell:
          type: definition
          required: false
      outputs:
      - focus_eligibility
      - variant_properties
      events:
      - spell.cast_pending
      capabilities:
      - spellcasting_focus
      - item_variants
    requires:
    - dnd2024:equipment_holy_symbol_varies
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/holy_water_25_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 25
          unit: gp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/holy_water_25_gp
        implementation: dnd2024:equipment_holy_water_25_gp
        parameters:
          use:
            economy: replace_one_attack
            action: attack
            consume: 1
            target:
              count: 1
              kinds:
              - creature
              visible: true
              range_ft: 20
            save:
              ability: dexterity
              dc:
                op: add
                operands:
                - 8
                - ability_modifier: dexterity
                - stat: proficiency_bonus
              success: no_effect
              failure:
                damage:
                  amount:
                    dice:
                      count: 2
                      sides: 8
                    bonus: 0
                  type: radiant
                only_if_creature_type_in:
                - fiend
                - undead
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_holy_water_25_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - attack.replacement
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_holy_water_25_gp
    citations:
    - SRD 5.2.1 pp. 97; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/hunting_trap_5_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: gp
        mass:
          value: 25.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/hunting_trap_5_gp
        implementation: dnd2024:equipment_hunting_trap_5_gp
        parameters:
          set:
            action: utilize
            anchor: immobile_object
            trigger: creature_steps_on_pressure_plate
            save:
              ability: dexterity
              dc: 13
              success: no_effect
              failure:
                damage:
                  amount:
                    dice:
                      count: 1
                      sides: 4
                    bonus: 0
                  type: piercing
                speed_set: 0
                speed_expiry: target_next_turn_start
                afterward: movement_limited_to_chain_length
            chain_length_ft:
              default: 3
              type: number
              authority: item_instance
          free:
            economy: action
            target: self_or_trapped_creature_in_reach
            check:
              ability: strength
              skill: athletics
              dc: 13
            success: free_target
            failure:
              damage:
                target: trapped_creature
                amount: 1
                type: piercing
          mass:
            value: 25.0
            unit: lb
    contract:
      id: dnd2024:equipment_hunting_trap_5_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_hunting_trap_5_gp
    citations:
    - SRD 5.2.1 pp. 97, 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/ink_10_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 10
          unit: gp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/ink_10_gp
        implementation: dnd2024:equipment_ink_10_gp
        parameters:
          container: bottle
          volume_ounces: 1
          writing_capacity:
            pages: 500
            approximate: true
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_ink_10_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_ink_10_gp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/ink_pen_2_cp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: cp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/ink_pen_2_cp
        implementation: dnd2024:equipment_ink_pen_2_cp
        parameters:
          operations:
          - write
          - draw
          requires_item:
            key: dnd2024:equipment/ink_10_gp
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_ink_pen_2_cp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_ink_pen_2_cp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/jug_2_cp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: cp
        mass:
          value: 4.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/jug_2_cp
        implementation: dnd2024:equipment_jug_2_cp
        parameters:
          capacity:
            liquid_gallons: 1
          mass:
            value: 4.0
            unit: lb
    contract:
      id: dnd2024:equipment_jug_2_cp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_jug_2_cp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/ladder_1_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: sp
        mass:
          value: 25.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/ladder_1_sp
        implementation: dnd2024:equipment_ladder_1_sp
        parameters:
          height_ft: 10
          traversal: climb
          mass:
            value: 25.0
            unit: lb
    contract:
      id: dnd2024:equipment_ladder_1_sp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_ladder_1_sp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/lamp_5_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: sp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/lamp_5_sp
        implementation: dnd2024:equipment_lamp_5_sp
        parameters:
          light:
            requires_lit: true
            shape: radius
            bright_ft: 15
            additional_dim_ft: 30
          fuel:
            key: dnd2024:equipment/oil_1_sp
          hours_per_flask: 6
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_lamp_5_sp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_lamp_5_sp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/lantern_bullseye_10_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 10
          unit: gp
        mass:
          value: 2.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/lantern_bullseye_10_gp
        implementation: dnd2024:equipment_lantern_bullseye_10_gp
        parameters:
          light:
            requires_lit: true
            shape: cone
            bright_ft: 60
            additional_dim_ft: 60
          fuel:
            key: dnd2024:equipment/oil_1_sp
          hours_per_flask: 6
          mass:
            value: 2.0
            unit: lb
    contract:
      id: dnd2024:equipment_lantern_bullseye_10_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_lantern_bullseye_10_gp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/lantern_hooded_5_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: gp
        mass:
          value: 2.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/lantern_hooded_5_gp
        implementation: dnd2024:equipment_lantern_hooded_5_gp
        parameters:
          light:
            requires_lit: true
            shape: radius
            bright_ft: 30
            additional_dim_ft: 30
          fuel:
            key: dnd2024:equipment/oil_1_sp
          hours_per_flask: 6
          hood:
            economy: bonus_action
            operations:
            - lower
            - raise
            lowered_light:
              shape: radius
              bright_ft: 0
              dim_ft: 5
          mass:
            value: 2.0
            unit: lb
    contract:
      id: dnd2024:equipment_lantern_hooded_5_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_lantern_hooded_5_gp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/lock_10_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 10
          unit: gp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/lock_10_gp
        implementation: dnd2024:equipment_lock_10_gp
        parameters:
          includes_key: true
          pick:
            requires_item:
              key: dnd2024:equipment/other_tools_thieves_tools_25_gp
            check:
              ability: dexterity
              skill: sleight_of_hand
              dc: 15
            success: unlock
          key_use:
            matching_key_required: true
            operation: unlock
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_lock_10_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_lock_10_gp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/magnifying_glass_100_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 100
          unit: gp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/magnifying_glass_100_gp
        implementation: dnd2024:equipment_magnifying_glass_100_gp
        parameters:
          using:
            roll: ability_check
            mode: advantage
            scope: appraise_or_inspect_highly_detailed_item
          ignite:
            minimum_light: sunlight_brightness
            requires_tinder: true
            duration:
              value: 5
              unit: minute
            approximate: true
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_magnifying_glass_100_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_magnifying_glass_100_gp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/manacles_2_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: gp
        mass:
          value: 6.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/manacles_2_gp
        implementation: dnd2024:equipment_manacles_2_gp
        parameters:
          bind:
            action: utilize
            target:
              range_ft: 5
              sizes:
              - small
              - medium
              unwilling: true
              has_any_condition:
              - grappled
              - incapacitated
              - restrained
            check:
              ability: dexterity
              skill: sleight_of_hand
              dc: 13
            while_bound:
              attack_roll_mode: disadvantage
              restrained_if: attached_to_fixed_chain_or_hook
          escape:
            economy: action
            check:
              ability: dexterity
              skill: sleight_of_hand
              dc: 20
          burst:
            economy: action
            check:
              ability: strength
              skill: athletics
              dc: 25
          includes_key: true
          pick:
            requires_item:
              key: dnd2024:equipment/other_tools_thieves_tools_25_gp
            check:
              ability: dexterity
              skill: sleight_of_hand
              dc: 15
            success: unlock
          mass:
            value: 6.0
            unit: lb
    contract:
      id: dnd2024:equipment_manacles_2_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_manacles_2_gp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/map_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/map_1_gp
        implementation: dnd2024:equipment_map_1_gp
        parameters:
          consult:
            accurate: true
            scope: find_way_in_represented_place
            check_bonus: 5
            ability: wisdom
            skill: survival
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_map_1_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_map_1_gp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/mirror_5_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: gp
        mass:
          value: 0.5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/mirror_5_gp
        implementation: dnd2024:equipment_mirror_5_gp
        parameters:
          material: steel
          held: true
          operations:
          - personal_cosmetics
          - peek_around_corner
          - reflect_light_signal
          mass:
            value: 0.5
            unit: lb
    contract:
      id: dnd2024:equipment_mirror_5_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_mirror_5_gp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/net_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 3.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/net_1_gp
        implementation: dnd2024:equipment_net_1_gp
        parameters:
          throw:
            economy: replace_one_attack
            action: attack
            target:
              kind: creature
              visible: true
              range_ft: 15
            save:
              ability: dexterity
              dc:
                op: add
                operands:
                - 8
                - ability_modifier: dexterity
                - stat: proficiency_bonus
              automatic_success_sizes:
              - huge
              - gargantuan
              success: no_effect
              failure:
                condition: restrained
                until: escape_or_net_destroyed
          escape:
            economy: action
            actor: target_or_creature_within_5_ft
            check:
              ability: strength
              skill: athletics
              dc: 10
            success: free_target
          object_stats:
            armor_class: 10
            hit_points: 5
            damage_immunities:
            - bludgeoning
            - poison
            - psychic
          on_destroy: free_target
          mass:
            value: 3.0
            unit: lb
    contract:
      id: dnd2024:equipment_net_1_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - attack.replacement
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_net_1_gp
    citations:
    - SRD 5.2.1 pp. 98; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/oil_1_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: sp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/oil_1_sp
        implementation: dnd2024:equipment_oil_1_sp
        parameters:
          douse_target:
            economy: replace_one_attack
            action: attack
            consume: 1
            target:
              count: 1
              kinds:
              - creature
              - object
              range_ft: 20
            save:
              ability: dexterity
              dc:
                op: add
                operands:
                - 8
                - ability_modifier: dexterity
                - stat: proficiency_bonus
              success: no_effect
              failure:
                coating_duration:
                  value: 1
                  unit: minute
                trigger: target_takes_fire_damage
                extra_damage:
                  amount: 5
                  type: fire
          douse_area:
            action: utilize
            consume: 1
            area:
              shape: square
              side_ft: 5
              range_ft: 5
              level_ground: true
            requires_ignition: true
            duration:
              rounds: 2
              expiry: turn_end
              outside_combat_seconds: 12
            triggers:
            - creature_enters
            - creature_turn_end_in_area
            damage:
              amount: 5
              type: fire
            maximum_applications_per_creature_per_turn: 1
          fuel:
            compatible:
            - key: dnd2024:equipment/lamp_5_sp
            - key: dnd2024:equipment/lantern_bullseye_10_gp
            - key: dnd2024:equipment/lantern_hooded_5_gp
            burn_time_per_flask:
              value: 6
              unit: hour
            time_is_cumulative: true
            extinguish_action: utilize
            rekindling_allowed: true
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_oil_1_sp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - attack.replacement
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_oil_1_sp
    citations:
    - SRD 5.2.1 pp. 98, 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/paper_2_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: sp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/paper_2_sp
        implementation: dnd2024:equipment_paper_2_sp
        parameters:
          quantity: 1
          unit: sheet
          writing_capacity:
            words: 250
            approximate: true
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_paper_2_sp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_paper_2_sp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/parchment_1_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: sp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/parchment_1_sp
        implementation: dnd2024:equipment_parchment_1_sp
        parameters:
          quantity: 1
          unit: sheet
          writing_capacity:
            words: 250
            approximate: true
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_parchment_1_sp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_parchment_1_sp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/perfume_5_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: gp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/perfume_5_gp
        implementation: dnd2024:equipment_perfume_5_gp
        parameters:
          container: vial
          volume_ounces: 4
          apply:
            target: self
            duration:
              value: 1
              unit: hour
            effect:
              roll: ability_check
              mode: advantage
              ability: charisma
              skill: persuasion
              target_type: humanoid
              target_attitude: indifferent
              target_max_distance_ft: 5
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_perfume_5_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_perfume_5_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/poison_basic_100_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 100
          unit: gp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/poison_basic_100_gp
        implementation: dnd2024:equipment_poison_basic_100_gp
        parameters:
          apply:
            economy: bonus_action
            consume: 1
            target_choices:
            - kind: weapon
              count: 1
            - kind: ammunition
              maximum_count: 3
            duration:
              value: 1
              unit: minute
            early_expiry: poison_damage_dealt
            trigger:
              target_kind: creature
              weapon_or_ammunition_damage_type_in:
              - piercing
              - slashing
            extra_damage:
              amount:
                dice:
                  count: 1
                  sides: 4
                bonus: 0
              type: poison
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_poison_basic_100_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_poison_basic_100_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/pole_5_cp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: cp
        mass:
          value: 7.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/pole_5_cp
        implementation: dnd2024:equipment_pole_5_cp
        parameters:
          length_ft: 10
          touch_range_ft: 10
          vault:
            roll: ability_check
            mode: advantage
            ability: strength
            skill: athletics
            scope: required_high_or_long_jump_check
          mass:
            value: 7.0
            unit: lb
    contract:
      id: dnd2024:equipment_pole_5_cp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_pole_5_cp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/pot_iron_2_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: gp
        mass:
          value: 10.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/pot_iron_2_gp
        implementation: dnd2024:equipment_pot_iron_2_gp
        parameters:
          capacity:
            liquid_gallons: 1
          mass:
            value: 10.0
            unit: lb
    contract:
      id: dnd2024:equipment_pot_iron_2_gp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_pot_iron_2_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/potion_of_healing_50_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 50
          unit: gp
        mass:
          value: 0.5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/potion_of_healing_50_gp
        implementation: dnd2024:equipment_potion_of_healing_50_gp
        parameters:
          magical: true
          use:
            economy: bonus_action
            consume: 1
            target: self_or_creature_within_5_ft
            operation: drink_or_administer
            healing:
              dice:
                count: 2
                sides: 4
              bonus: 2
          mass:
            value: 0.5
            unit: lb
    contract:
      id: dnd2024:equipment_potion_of_healing_50_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_potion_of_healing_50_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/pouch_5_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: sp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/pouch_5_sp
        implementation: dnd2024:equipment_pouch_5_sp
        parameters:
          capacity:
            mass_lb: 6
            volume_ft3: 0.2
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_pouch_5_sp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_pouch_5_sp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/priest_s_pack_33_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 33
          unit: gp
        mass:
          value: 29.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/priest_s_pack_33_gp
        implementation: dnd2024:equipment_priest_s_pack_33_gp
        parameters:
          contents:
          - item:
              key: dnd2024:equipment/backpack_2_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/blanket_5_sp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/holy_water_25_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/lamp_5_sp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/rations_5_sp
            quantity: 7
            unit: day
          - item:
              key: dnd2024:equipment/robe_1_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/tinderbox_5_sp
            quantity: 1
            unit: purchase_unit
          grant_contents_on_acquisition: true
          mass:
            value: 29.0
            unit: lb
          grant_idempotency: once_per_acquired_pack_instance
    contract:
      id: dnd2024:equipment_priest_s_pack_33_gp
      inputs:
        actor: participant
        item: item_instance
      outputs:
      - inventory_grants
      events:
      - item.acquired
      capabilities:
      - inventory
      - item_bundles
    requires:
    - dnd2024:equipment_priest_s_pack_33_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/quiver_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/quiver_1_gp
        implementation: dnd2024:equipment_quiver_1_gp
        parameters:
          capacity:
            ammunition_variant: arrows
            count: 20
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_quiver_1_gp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_quiver_1_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/ram_portable_4_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 4
          unit: gp
        mass:
          value: 35.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/ram_portable_4_gp
        implementation: dnd2024:equipment_ram_portable_4_gp
        parameters:
          break_door:
            ability: strength
            check_bonus: 4
            helper_maximum: 1
            with_helper_mode: advantage
          mass:
            value: 35.0
            unit: lb
    contract:
      id: dnd2024:equipment_ram_portable_4_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_ram_portable_4_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/rations_5_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: sp
        mass:
          value: 2.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/rations_5_sp
        implementation: dnd2024:equipment_rations_5_sp
        parameters:
          food:
            travel_ready: true
            components:
            - jerky
            - dried_fruit
            - hardtack
            - nuts
            mass_lb: 2
          nutrition_rule:
            key: dnd2024:glossary/malnutrition_hazard
          mass:
            value: 2.0
            unit: lb
    contract:
      id: dnd2024:equipment_rations_5_sp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_rations_5_sp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/robe_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 4.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/robe_1_gp
        implementation: dnd2024:equipment_robe_1_gp
        parameters:
          access_requirement:
            authority: dm
            scope:
            - event
            - location
            eligible_if_wearing: true
            required_colors:
              type: choice
              authority: dm
            required_symbols:
              type: choice
              authority: dm
          mass:
            value: 4.0
            unit: lb
    contract:
      id: dnd2024:equipment_robe_1_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_robe_1_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/rope_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 5.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/rope_1_gp
        implementation: dnd2024:equipment_rope_1_gp
        parameters:
          tie_knot:
            action: utilize
            check:
              ability: dexterity
              skill: sleight_of_hand
              dc: 10
          bind:
            unwilling_target_requires_any_condition:
            - grappled
            - incapacitated
            - restrained
            bound_legs_condition: restrained
            until: escape
          burst:
            check:
              ability: strength
              skill: athletics
              dc: 20
          escape:
            economy: action
            check:
              ability: dexterity
              skill: acrobatics
              dc: 15
          mass:
            value: 5.0
            unit: lb
    contract:
      id: dnd2024:equipment_rope_1_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_rope_1_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/sack_1_cp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: cp
        mass:
          value: 0.5
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/sack_1_cp
        implementation: dnd2024:equipment_sack_1_cp
        parameters:
          capacity:
            mass_lb: 30
            volume_ft3: 1
          mass:
            value: 0.5
            unit: lb
    contract:
      id: dnd2024:equipment_sack_1_cp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_sack_1_cp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/scholar_s_pack_40_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 40
          unit: gp
        mass:
          value: 22.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/scholar_s_pack_40_gp
        implementation: dnd2024:equipment_scholar_s_pack_40_gp
        parameters:
          contents:
          - item:
              key: dnd2024:equipment/backpack_2_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/book_25_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/ink_10_gp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/ink_pen_2_cp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/lamp_5_sp
            quantity: 1
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/oil_1_sp
            quantity: 10
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/parchment_1_sp
            quantity: 10
            unit: purchase_unit
          - item:
              key: dnd2024:equipment/tinderbox_5_sp
            quantity: 1
            unit: purchase_unit
          grant_contents_on_acquisition: true
          mass:
            value: 22.0
            unit: lb
          grant_idempotency: once_per_acquired_pack_instance
    contract:
      id: dnd2024:equipment_scholar_s_pack_40_gp
      inputs:
        actor: participant
        item: item_instance
      outputs:
      - inventory_grants
      events:
      - item.acquired
      capabilities:
      - inventory
      - item_bundles
    requires:
    - dnd2024:equipment_scholar_s_pack_40_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/shovel_2_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: gp
        mass:
          value: 5.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/shovel_2_gp
        implementation: dnd2024:equipment_shovel_2_gp
        parameters:
          dig:
            duration:
              value: 1
              unit: hour
            shape: cube
            side_ft: 5
            material_choices:
            - soil
            - similar_material
            similar_material_authority: dm
          mass:
            value: 5.0
            unit: lb
    contract:
      id: dnd2024:equipment_shovel_2_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_shovel_2_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/signal_whistle_5_cp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: cp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/signal_whistle_5_cp
        implementation: dnd2024:equipment_signal_whistle_5_cp
        parameters:
          use:
            action: utilize
            effect:
              sound_audible_range_ft: 600
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_signal_whistle_5_cp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_signal_whistle_5_cp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/spell_scroll_cantrip_30_gp_level_1_50_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/spell_scroll_cantrip_30_gp_level_1_50_gp
        implementation: dnd2024:equipment_spell_scroll_cantrip_30_gp_level_1_50_gp
        parameters:
          magical: true
          variants:
          - id: cantrip
            spell_level: 0
            price:
              value: 30
              unit: gp
          - id: level_1
            spell_level: 1
            price:
              value: 50
              unit: gp
          spell_selection:
            authority: creator
            level: variant_spell_level
          cast:
            requires_spell_on_class_list: true
            requires_reading: true
            casting_time: spell_normal
            material_components_required: false
            save_dc: 13
            attack_bonus: 5
            on_completed: scroll_disintegrates
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_spell_scroll_cantrip_30_gp_level_1_50_gp
      inputs:
        actor: participant
        item: item_instance
        spell: definition
      outputs:
      - cast_eligibility
      - spell_cast
      - item_consumption
      events:
      - spell.cast_pending
      - spell.cast_completed
      capabilities:
      - item_spellcasting
      - spellcasting
      - consumables
    requires:
    - dnd2024:equipment_spell_scroll_cantrip_30_gp_level_1_50_gp
    citations:
    - SRD 5.2.1 pp. 99; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/spikes_iron_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 5.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/spikes_iron_1_gp
        implementation: dnd2024:equipment_spikes_iron_1_gp
        parameters:
          purchase_quantity: 10
          hammer:
            action: utilize
            requires: blunt_object
            materials:
            - wood
            - earth
            - similar_material
            similar_material_authority: dm
            operations:
            - jam_door_shut
            - provide_rope_anchor
            - provide_chain_anchor
          mass:
            value: 5.0
            unit: lb
    contract:
      id: dnd2024:equipment_spikes_iron_1_gp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_spikes_iron_1_gp
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/spyglass_1_000_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1000
          unit: gp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/spyglass_1_000_gp
        implementation: dnd2024:equipment_spyglass_1_000_gp
        parameters:
          view:
            apparent_size_multiplier: 2
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_spyglass_1_000_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_spyglass_1_000_gp
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/string_1_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: sp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/string_1_sp
        implementation: dnd2024:equipment_string_1_sp
        parameters:
          length_ft: 10
          tie_knot:
            action: utilize
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_string_1_sp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_string_1_sp
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/tent_2_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: gp
        mass:
          value: 20.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/tent_2_gp
        implementation: dnd2024:equipment_tent_2_gp
        parameters:
          sleep_capacity:
            count: 2
            sizes:
            - small
            - medium
          mass:
            value: 20.0
            unit: lb
    contract:
      id: dnd2024:equipment_tent_2_gp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_tent_2_gp
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/tinderbox_5_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 5
          unit: sp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/tinderbox_5_sp
        implementation: dnd2024:equipment_tinderbox_5_sp
        parameters:
          components:
          - flint
          - fire_steel
          - tinder
          ignite:
            exposed_fuel:
              economy: bonus_action
            other_fire:
              duration:
                value: 1
                unit: minute
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_tinderbox_5_sp
      inputs:
        actor: participant
        item: item_instance
        operation:
          type: choice
          required: false
        target:
          type: participant
          required: false
        target_object:
          type: object
          required: false
        location:
          type: location
          required: false
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - action_options
      - check_requests
      - item_effects
      - item_state
      events:
      - item.used
      capabilities:
      - item_actions
      - effects
      - ability_checks
      - consumables
    requires:
    - dnd2024:equipment_tinderbox_5_sp
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/torch_1_cp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: cp
        mass:
          value: 1.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/torch_1_cp
        implementation: dnd2024:equipment_torch_1_cp
        parameters:
          light:
            requires_lit: true
            shape: radius
            bright_ft: 20
            additional_dim_ft: 20
          burn_duration:
            value: 1
            unit: hour
          attack:
            action: attack
            weapon_category: simple_melee
            damage:
              amount: 1
              type: fire
          mass:
            value: 1.0
            unit: lb
    contract:
      id: dnd2024:equipment_torch_1_cp
      inputs:
        actor: participant
        item: item_instance
        event:
          type: event
          required: false
        choices:
          type: choice
          required: false
      outputs:
      - item_properties
      - conditional_modifiers
      events: []
      capabilities:
      - equipment
      - conditional_modifiers
    requires:
    - dnd2024:equipment_torch_1_cp
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/vial_1_gp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 1
          unit: gp
        mass:
          value: 0.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/vial_1_gp
        implementation: dnd2024:equipment_vial_1_gp
        parameters:
          capacity:
            liquid_ounces: 4
          mass:
            value: 0.0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_vial_1_gp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_vial_1_gp
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/waterskin_2_sp:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_item:
        category: adventuring_gear
        equipment:
          attunement: none
        price:
          value: 2
          unit: sp
        mass:
          value: 5.0
          unit: lb
      splat_rule:
        mechanic: dnd2024:equipment/waterskin_2_sp
        implementation: dnd2024:equipment_waterskin_2_sp
        parameters:
          capacity:
            liquid_pints: 4
          nutrition_rule:
            key: dnd2024:glossary/dehydration_hazard
          mass:
            value: 5.0
            unit: lb
            when: full
    contract:
      id: dnd2024:equipment_waterskin_2_sp
      inputs:
        item: item_instance
        contents:
          type: item_list
          required: false
      outputs:
      - capacity_constraints
      events: []
      capabilities:
      - containers
      - inventory
    requires:
    - dnd2024:equipment_waterskin_2_sp
    citations:
    - SRD 5.2.1 pp. 100; builtin/srd-5.2.1/source.pdf
    - SRD 5.2.1 pp. 93–100, especially Adventuring Gear weights p. 95; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/club:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d4
          type: bludgeoning
        properties_text: Light
        mastery: slow
        weapon_category: simple melee
        price:
          value: 1
          unit: sp
        mass:
          value: 2.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/club
        implementation: dnd2024:equipment_club
        parameters:
          weapon_category: simple_melee
          damage:
            expression: 1d4
            type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 4
              bonus: 0
          properties:
          - id: light
            rule:
              key: dnd2024:rules/weapons_light
            trigger: attack_with_light_weapon_during_own_attack_action
            extra_attack:
              count: 1
              economy: bonus_action
              timing: later_same_turn
              weapon: different_light_weapon
              damage_ability_modifier: negative_only
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: slow
            rule:
              key: dnd2024:rules/weapons_slow
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              optional: true
              speed_reduction_ft: 10
              expiry: attacker_next_turn_start
              maximum_stacked_reduction_ft: 10
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_club
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_club
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/dagger:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d4
          type: piercing
        properties_text: Finesse, Light, Thrown (Range 20/60)
        mastery: nick
        weapon_category: simple melee
        price:
          value: 2
          unit: gp
        mass:
          value: 1.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/dagger
        implementation: dnd2024:equipment_dagger
        parameters:
          weapon_category: simple_melee
          damage:
            expression: 1d4
            type: piercing
            amount:
              dice:
                count: 1
                sides: 4
              bonus: 0
          properties:
          - id: finesse
            rule:
              key: dnd2024:rules/weapons_finesse
            attack_and_damage_ability_choice:
            - strength
            - dexterity
            same_ability_for_both: true
          - id: light
            rule:
              key: dnd2024:rules/weapons_light
            trigger: attack_with_light_weapon_during_own_attack_action
            extra_attack:
              count: 1
              economy: bonus_action
              timing: later_same_turn
              weapon: different_light_weapon
              damage_ability_modifier: negative_only
          - id: thrown
            rule:
              key: dnd2024:rules/weapons_thrown
            range:
              normal_ft: 20
              long_ft: 60
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            draw_as_part_of_attack: true
            attack_type: ranged
            melee_weapon_uses_melee_ability: true
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: nick
            rule:
              key: dnd2024:rules/weapons_nick
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: light_property_extra_attack
              replace_economy: bonus_action
              economy: part_of_attack_action
              maximum_per_turn: 1
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_dagger
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_dagger
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/greatclub:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d8
          type: bludgeoning
        properties_text: Two-Handed
        mastery: push
        weapon_category: simple melee
        price:
          value: 2
          unit: sp
        mass:
          value: 10.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/greatclub
        implementation: dnd2024:equipment_greatclub
        parameters:
          weapon_category: simple_melee
          damage:
            expression: 1d8
            type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 8
              bonus: 0
          properties:
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: push
            rule:
              key: dnd2024:rules/weapons_push
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              optional: true
              maximum_target_size: large
              push_maximum_ft: 10
              direction: straight_away_from_attacker
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_greatclub
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_greatclub
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/handaxe:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d6
          type: slashing
        properties_text: Light, Thrown (Range 20/60)
        mastery: vex
        weapon_category: simple melee
        price:
          value: 5
          unit: gp
        mass:
          value: 2.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/handaxe
        implementation: dnd2024:equipment_handaxe
        parameters:
          weapon_category: simple_melee
          damage:
            expression: 1d6
            type: slashing
            amount:
              dice:
                count: 1
                sides: 6
              bonus: 0
          properties:
          - id: light
            rule:
              key: dnd2024:rules/weapons_light
            trigger: attack_with_light_weapon_during_own_attack_action
            extra_attack:
              count: 1
              economy: bonus_action
              timing: later_same_turn
              weapon: different_light_weapon
              damage_ability_modifier: negative_only
          - id: thrown
            rule:
              key: dnd2024:rules/weapons_thrown
            range:
              normal_ft: 20
              long_ft: 60
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            draw_as_part_of_attack: true
            attack_type: ranged
            melee_weapon_uses_melee_ability: true
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: vex
            rule:
              key: dnd2024:rules/weapons_vex
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              effect:
                attacker_attack_roll_mode: advantage
                against: same_target
                count: 1
                expiry: attacker_next_turn_end
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_handaxe
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_handaxe
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/javelin:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d6
          type: piercing
        properties_text: Thrown (Range 30/120)
        mastery: slow
        weapon_category: simple melee
        price:
          value: 5
          unit: sp
        mass:
          value: 2.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/javelin
        implementation: dnd2024:equipment_javelin
        parameters:
          weapon_category: simple_melee
          damage:
            expression: 1d6
            type: piercing
            amount:
              dice:
                count: 1
                sides: 6
              bonus: 0
          properties:
          - id: thrown
            rule:
              key: dnd2024:rules/weapons_thrown
            range:
              normal_ft: 30
              long_ft: 120
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            draw_as_part_of_attack: true
            attack_type: ranged
            melee_weapon_uses_melee_ability: true
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: slow
            rule:
              key: dnd2024:rules/weapons_slow
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              optional: true
              speed_reduction_ft: 10
              expiry: attacker_next_turn_start
              maximum_stacked_reduction_ft: 10
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_javelin
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_javelin
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/light_hammer:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d4
          type: bludgeoning
        properties_text: Light, Thrown (Range 20/60)
        mastery: nick
        weapon_category: simple melee
        price:
          value: 2
          unit: gp
        mass:
          value: 2.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/light_hammer
        implementation: dnd2024:equipment_light_hammer
        parameters:
          weapon_category: simple_melee
          damage:
            expression: 1d4
            type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 4
              bonus: 0
          properties:
          - id: light
            rule:
              key: dnd2024:rules/weapons_light
            trigger: attack_with_light_weapon_during_own_attack_action
            extra_attack:
              count: 1
              economy: bonus_action
              timing: later_same_turn
              weapon: different_light_weapon
              damage_ability_modifier: negative_only
          - id: thrown
            rule:
              key: dnd2024:rules/weapons_thrown
            range:
              normal_ft: 20
              long_ft: 60
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            draw_as_part_of_attack: true
            attack_type: ranged
            melee_weapon_uses_melee_ability: true
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: nick
            rule:
              key: dnd2024:rules/weapons_nick
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: light_property_extra_attack
              replace_economy: bonus_action
              economy: part_of_attack_action
              maximum_per_turn: 1
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_light_hammer
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_light_hammer
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/mace:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d6
          type: bludgeoning
        properties_text: —
        mastery: sap
        weapon_category: simple melee
        price:
          value: 5
          unit: gp
        mass:
          value: 4.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/mace
        implementation: dnd2024:equipment_mace
        parameters:
          weapon_category: simple_melee
          damage:
            expression: 1d6
            type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 6
              bonus: 0
          properties: []
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: sap
            rule:
              key: dnd2024:rules/weapons_sap
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              effect:
                attack_roll_mode: disadvantage
                count: 1
                expiry: attacker_next_turn_start
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_mace
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_mace
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/quarterstaff:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d6
          type: bludgeoning
        properties_text: Versatile (1d8)
        mastery: topple
        weapon_category: simple melee
        price:
          value: 2
          unit: sp
        mass:
          value: 4.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/quarterstaff
        implementation: dnd2024:equipment_quarterstaff
        parameters:
          weapon_category: simple_melee
          damage:
            expression: 1d6
            type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 6
              bonus: 0
          properties:
          - id: versatile
            rule:
              key: dnd2024:rules/weapons_versatile
            two_hand_melee_damage:
              expression: 1d8
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: topple
            rule:
              key: dnd2024:rules/weapons_topple
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              optional: true
              save:
                ability: constitution
                dc:
                  op: add
                  operands:
                  - 8
                  - stat: attack_ability_modifier
                  - stat: proficiency_bonus
                failure:
                  condition: prone
                success: no_effect
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_quarterstaff
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_quarterstaff
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/sickle:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d4
          type: slashing
        properties_text: Light
        mastery: nick
        weapon_category: simple melee
        price:
          value: 1
          unit: gp
        mass:
          value: 2.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/sickle
        implementation: dnd2024:equipment_sickle
        parameters:
          weapon_category: simple_melee
          damage:
            expression: 1d4
            type: slashing
            amount:
              dice:
                count: 1
                sides: 4
              bonus: 0
          properties:
          - id: light
            rule:
              key: dnd2024:rules/weapons_light
            trigger: attack_with_light_weapon_during_own_attack_action
            extra_attack:
              count: 1
              economy: bonus_action
              timing: later_same_turn
              weapon: different_light_weapon
              damage_ability_modifier: negative_only
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: nick
            rule:
              key: dnd2024:rules/weapons_nick
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: light_property_extra_attack
              replace_economy: bonus_action
              economy: part_of_attack_action
              maximum_per_turn: 1
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_sickle
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_sickle
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/spear:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d6
          type: piercing
        properties_text: Thrown (Range 20/60), Versatile (1d8)
        mastery: sap
        weapon_category: simple melee
        price:
          value: 1
          unit: gp
        mass:
          value: 3.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/spear
        implementation: dnd2024:equipment_spear
        parameters:
          weapon_category: simple_melee
          damage:
            expression: 1d6
            type: piercing
            amount:
              dice:
                count: 1
                sides: 6
              bonus: 0
          properties:
          - id: thrown
            rule:
              key: dnd2024:rules/weapons_thrown
            range:
              normal_ft: 20
              long_ft: 60
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            draw_as_part_of_attack: true
            attack_type: ranged
            melee_weapon_uses_melee_ability: true
          - id: versatile
            rule:
              key: dnd2024:rules/weapons_versatile
            two_hand_melee_damage:
              expression: 1d8
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: sap
            rule:
              key: dnd2024:rules/weapons_sap
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              effect:
                attack_roll_mode: disadvantage
                count: 1
                expiry: attacker_next_turn_start
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_spear
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_spear
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/dart:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d4
          type: piercing
        properties_text: Finesse, Thrown (Range 20/60)
        mastery: vex
        weapon_category: simple ranged
        price:
          value: 5
          unit: cp
        mass:
          value: 0.25
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/dart
        implementation: dnd2024:equipment_dart
        parameters:
          weapon_category: simple_ranged
          damage:
            expression: 1d4
            type: piercing
            amount:
              dice:
                count: 1
                sides: 4
              bonus: 0
          properties:
          - id: finesse
            rule:
              key: dnd2024:rules/weapons_finesse
            attack_and_damage_ability_choice:
            - strength
            - dexterity
            same_ability_for_both: true
          - id: thrown
            rule:
              key: dnd2024:rules/weapons_thrown
            range:
              normal_ft: 20
              long_ft: 60
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            draw_as_part_of_attack: true
            attack_type: ranged
            melee_weapon_uses_melee_ability: true
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: dexterity
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: vex
            rule:
              key: dnd2024:rules/weapons_vex
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              effect:
                attacker_attack_roll_mode: advantage
                against: same_target
                count: 1
                expiry: attacker_next_turn_end
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_dart
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_dart
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/light_crossbow:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d8
          type: piercing
        properties_text: Ammunition (Range 80/320; Bolt), Loading, Two-Handed
        mastery: slow
        weapon_category: simple ranged
        price:
          value: 25
          unit: gp
        mass:
          value: 5.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/light_crossbow
        implementation: dnd2024:equipment_light_crossbow
        parameters:
          weapon_category: simple_ranged
          damage:
            expression: 1d8
            type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              bonus: 0
          properties:
          - id: ammunition
            rule:
              key: dnd2024:rules/weapons_ammunition
            range:
              normal_ft: 80
              long_ft: 320
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            ammunition:
              key: dnd2024:equipment/ammunition_varies
              variant: bolts
            expend_per_attack: 1
            draw_as_part_of_attack: true
            free_hand_to_load_if_one_handed: true
            recovery:
              duration:
                value: 1
                unit: minute
              timing: after_fight
              fraction: 0.5
              rounding: down
              unrecovered: lost
          - id: loading
            rule:
              key: dnd2024:rules/weapons_loading
            ammunition_limit: 1
            per:
            - action
            - bonus_action
            - reaction
            ignores_extra_attacks: true
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: dexterity
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: slow
            rule:
              key: dnd2024:rules/weapons_slow
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              optional: true
              speed_reduction_ft: 10
              expiry: attacker_next_turn_start
              maximum_stacked_reduction_ft: 10
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_light_crossbow
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_light_crossbow
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/shortbow:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d6
          type: piercing
        properties_text: Ammunition (Range 80/320; Arrow), Two-Handed
        mastery: vex
        weapon_category: simple ranged
        price:
          value: 25
          unit: gp
        mass:
          value: 2.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/shortbow
        implementation: dnd2024:equipment_shortbow
        parameters:
          weapon_category: simple_ranged
          damage:
            expression: 1d6
            type: piercing
            amount:
              dice:
                count: 1
                sides: 6
              bonus: 0
          properties:
          - id: ammunition
            rule:
              key: dnd2024:rules/weapons_ammunition
            range:
              normal_ft: 80
              long_ft: 320
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            ammunition:
              key: dnd2024:equipment/ammunition_varies
              variant: arrows
            expend_per_attack: 1
            draw_as_part_of_attack: true
            free_hand_to_load_if_one_handed: true
            recovery:
              duration:
                value: 1
                unit: minute
              timing: after_fight
              fraction: 0.5
              rounding: down
              unrecovered: lost
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: dexterity
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: vex
            rule:
              key: dnd2024:rules/weapons_vex
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              effect:
                attacker_attack_roll_mode: advantage
                against: same_target
                count: 1
                expiry: attacker_next_turn_end
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_shortbow
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_shortbow
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/sling:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d4
          type: bludgeoning
        properties_text: Ammunition (Range 30/120; Bullet)
        mastery: slow
        weapon_category: simple ranged
        price:
          value: 1
          unit: sp
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/sling
        implementation: dnd2024:equipment_sling
        parameters:
          weapon_category: simple_ranged
          damage:
            expression: 1d4
            type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 4
              bonus: 0
          properties:
          - id: ammunition
            rule:
              key: dnd2024:rules/weapons_ammunition
            range:
              normal_ft: 30
              long_ft: 120
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            ammunition:
              key: dnd2024:equipment/ammunition_varies
              variant: sling_bullets
            expend_per_attack: 1
            draw_as_part_of_attack: true
            free_hand_to_load_if_one_handed: true
            recovery:
              duration:
                value: 1
                unit: minute
              timing: after_fight
              fraction: 0.5
              rounding: down
              unrecovered: lost
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: dexterity
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: slow
            rule:
              key: dnd2024:rules/weapons_slow
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              optional: true
              speed_reduction_ft: 10
              expiry: attacker_next_turn_start
              maximum_stacked_reduction_ft: 10
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
          mass:
            value: 0
            unit: lb
            negligible: true
    contract:
      id: dnd2024:equipment_sling
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_sling
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/battleaxe:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d8
          type: slashing
        properties_text: Versatile (1d10)
        mastery: topple
        weapon_category: martial melee
        price:
          value: 10
          unit: gp
        mass:
          value: 4.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/battleaxe
        implementation: dnd2024:equipment_battleaxe
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d8
            type: slashing
            amount:
              dice:
                count: 1
                sides: 8
              bonus: 0
          properties:
          - id: versatile
            rule:
              key: dnd2024:rules/weapons_versatile
            two_hand_melee_damage:
              expression: 1d10
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: topple
            rule:
              key: dnd2024:rules/weapons_topple
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              optional: true
              save:
                ability: constitution
                dc:
                  op: add
                  operands:
                  - 8
                  - stat: attack_ability_modifier
                  - stat: proficiency_bonus
                failure:
                  condition: prone
                success: no_effect
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_battleaxe
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_battleaxe
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/flail:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d8
          type: bludgeoning
        properties_text: —
        mastery: sap
        weapon_category: martial melee
        price:
          value: 10
          unit: gp
        mass:
          value: 2.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/flail
        implementation: dnd2024:equipment_flail
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d8
            type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 8
              bonus: 0
          properties: []
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: sap
            rule:
              key: dnd2024:rules/weapons_sap
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              effect:
                attack_roll_mode: disadvantage
                count: 1
                expiry: attacker_next_turn_start
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_flail
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_flail
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/glaive:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d10
          type: slashing
        properties_text: Heavy, Reach, Two-Handed
        mastery: graze
        weapon_category: martial melee
        price:
          value: 20
          unit: gp
        mass:
          value: 6.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/glaive
        implementation: dnd2024:equipment_glaive
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d10
            type: slashing
            amount:
              dice:
                count: 1
                sides: 10
              bonus: 0
          properties:
          - id: heavy
            rule:
              key: dnd2024:rules/weapons_heavy
            required_ability: strength
            minimum_score: 13
            below_minimum_attack_mode: disadvantage
          - id: reach
            rule:
              key: dnd2024:rules/weapons_reach
            reach_bonus_ft: 5
            applies_to:
            - attacks
            - opportunity_attack_reach
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: graze
            rule:
              key: dnd2024:rules/weapons_graze
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: attack_misses_creature
              optional: true
              damage:
                amount:
                  stat: attack_ability_modifier
                type: weapon_damage_type
                increases_only_from: attack_ability_modifier
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_glaive
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_glaive
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/greataxe:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d12
          type: slashing
        properties_text: Heavy, Two-Handed
        mastery: cleave
        weapon_category: martial melee
        price:
          value: 30
          unit: gp
        mass:
          value: 7.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/greataxe
        implementation: dnd2024:equipment_greataxe
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d12
            type: slashing
            amount:
              dice:
                count: 1
                sides: 12
              bonus: 0
          properties:
          - id: heavy
            rule:
              key: dnd2024:rules/weapons_heavy
            required_ability: strength
            minimum_score: 13
            below_minimum_attack_mode: disadvantage
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: cleave
            rule:
              key: dnd2024:rules/weapons_cleave
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: melee_attack_hits_creature
              optional: true
              extra_melee_attack:
                target: second_creature
                within_first_target_ft: 5
                within_weapon_reach: true
                damage_ability_modifier: negative_only
              maximum_per_turn: 1
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_greataxe
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_greataxe
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/greatsword:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 2d6
          type: slashing
        properties_text: Heavy, Two-Handed
        mastery: graze
        weapon_category: martial melee
        price:
          value: 50
          unit: gp
        mass:
          value: 6.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/greatsword
        implementation: dnd2024:equipment_greatsword
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 2d6
            type: slashing
            amount:
              dice:
                count: 2
                sides: 6
              bonus: 0
          properties:
          - id: heavy
            rule:
              key: dnd2024:rules/weapons_heavy
            required_ability: strength
            minimum_score: 13
            below_minimum_attack_mode: disadvantage
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: graze
            rule:
              key: dnd2024:rules/weapons_graze
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: attack_misses_creature
              optional: true
              damage:
                amount:
                  stat: attack_ability_modifier
                type: weapon_damage_type
                increases_only_from: attack_ability_modifier
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_greatsword
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_greatsword
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/halberd:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d10
          type: slashing
        properties_text: Heavy, Reach, Two-Handed
        mastery: cleave
        weapon_category: martial melee
        price:
          value: 20
          unit: gp
        mass:
          value: 6.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/halberd
        implementation: dnd2024:equipment_halberd
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d10
            type: slashing
            amount:
              dice:
                count: 1
                sides: 10
              bonus: 0
          properties:
          - id: heavy
            rule:
              key: dnd2024:rules/weapons_heavy
            required_ability: strength
            minimum_score: 13
            below_minimum_attack_mode: disadvantage
          - id: reach
            rule:
              key: dnd2024:rules/weapons_reach
            reach_bonus_ft: 5
            applies_to:
            - attacks
            - opportunity_attack_reach
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: cleave
            rule:
              key: dnd2024:rules/weapons_cleave
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: melee_attack_hits_creature
              optional: true
              extra_melee_attack:
                target: second_creature
                within_first_target_ft: 5
                within_weapon_reach: true
                damage_ability_modifier: negative_only
              maximum_per_turn: 1
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_halberd
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_halberd
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/lance:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d10
          type: piercing
        properties_text: Heavy, Reach, Two-Handed (unless mounted)
        mastery: topple
        weapon_category: martial melee
        price:
          value: 10
          unit: gp
        mass:
          value: 6.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/lance
        implementation: dnd2024:equipment_lance
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d10
            type: piercing
            amount:
              dice:
                count: 1
                sides: 10
              bonus: 0
          properties:
          - id: heavy
            rule:
              key: dnd2024:rules/weapons_heavy
            required_ability: strength
            minimum_score: 13
            below_minimum_attack_mode: disadvantage
          - id: reach
            rule:
              key: dnd2024:rules/weapons_reach
            reach_bonus_ft: 5
            applies_to:
            - attacks
            - opportunity_attack_reach
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
            exception:
              if: mounted
              hands_to_attack: 1
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: topple
            rule:
              key: dnd2024:rules/weapons_topple
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              optional: true
              save:
                ability: constitution
                dc:
                  op: add
                  operands:
                  - 8
                  - stat: attack_ability_modifier
                  - stat: proficiency_bonus
                failure:
                  condition: prone
                success: no_effect
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_lance
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_lance
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/longsword:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d8
          type: slashing
        properties_text: Versatile (1d10)
        mastery: sap
        weapon_category: martial melee
        price:
          value: 15
          unit: gp
        mass:
          value: 3.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/longsword
        implementation: dnd2024:equipment_longsword
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d8
            type: slashing
            amount:
              dice:
                count: 1
                sides: 8
              bonus: 0
          properties:
          - id: versatile
            rule:
              key: dnd2024:rules/weapons_versatile
            two_hand_melee_damage:
              expression: 1d10
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: sap
            rule:
              key: dnd2024:rules/weapons_sap
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              effect:
                attack_roll_mode: disadvantage
                count: 1
                expiry: attacker_next_turn_start
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_longsword
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_longsword
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/maul:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 2d6
          type: bludgeoning
        properties_text: Heavy, Two-Handed
        mastery: topple
        weapon_category: martial melee
        price:
          value: 10
          unit: gp
        mass:
          value: 10.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/maul
        implementation: dnd2024:equipment_maul
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 2d6
            type: bludgeoning
            amount:
              dice:
                count: 2
                sides: 6
              bonus: 0
          properties:
          - id: heavy
            rule:
              key: dnd2024:rules/weapons_heavy
            required_ability: strength
            minimum_score: 13
            below_minimum_attack_mode: disadvantage
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: topple
            rule:
              key: dnd2024:rules/weapons_topple
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              optional: true
              save:
                ability: constitution
                dc:
                  op: add
                  operands:
                  - 8
                  - stat: attack_ability_modifier
                  - stat: proficiency_bonus
                failure:
                  condition: prone
                success: no_effect
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_maul
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_maul
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/morningstar:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d8
          type: piercing
        properties_text: —
        mastery: sap
        weapon_category: martial melee
        price:
          value: 15
          unit: gp
        mass:
          value: 4.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/morningstar
        implementation: dnd2024:equipment_morningstar
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d8
            type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              bonus: 0
          properties: []
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: sap
            rule:
              key: dnd2024:rules/weapons_sap
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              effect:
                attack_roll_mode: disadvantage
                count: 1
                expiry: attacker_next_turn_start
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_morningstar
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_morningstar
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/pike:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d10
          type: piercing
        properties_text: Heavy, Reach, Two-Handed
        mastery: push
        weapon_category: martial melee
        price:
          value: 5
          unit: gp
        mass:
          value: 18.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/pike
        implementation: dnd2024:equipment_pike
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d10
            type: piercing
            amount:
              dice:
                count: 1
                sides: 10
              bonus: 0
          properties:
          - id: heavy
            rule:
              key: dnd2024:rules/weapons_heavy
            required_ability: strength
            minimum_score: 13
            below_minimum_attack_mode: disadvantage
          - id: reach
            rule:
              key: dnd2024:rules/weapons_reach
            reach_bonus_ft: 5
            applies_to:
            - attacks
            - opportunity_attack_reach
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: push
            rule:
              key: dnd2024:rules/weapons_push
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              optional: true
              maximum_target_size: large
              push_maximum_ft: 10
              direction: straight_away_from_attacker
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_pike
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_pike
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/rapier:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d8
          type: piercing
        properties_text: Finesse
        mastery: vex
        weapon_category: martial melee
        price:
          value: 25
          unit: gp
        mass:
          value: 2.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/rapier
        implementation: dnd2024:equipment_rapier
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d8
            type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              bonus: 0
          properties:
          - id: finesse
            rule:
              key: dnd2024:rules/weapons_finesse
            attack_and_damage_ability_choice:
            - strength
            - dexterity
            same_ability_for_both: true
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: vex
            rule:
              key: dnd2024:rules/weapons_vex
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              effect:
                attacker_attack_roll_mode: advantage
                against: same_target
                count: 1
                expiry: attacker_next_turn_end
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_rapier
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_rapier
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/scimitar:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d6
          type: slashing
        properties_text: Finesse, Light
        mastery: nick
        weapon_category: martial melee
        price:
          value: 25
          unit: gp
        mass:
          value: 3.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/scimitar
        implementation: dnd2024:equipment_scimitar
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d6
            type: slashing
            amount:
              dice:
                count: 1
                sides: 6
              bonus: 0
          properties:
          - id: finesse
            rule:
              key: dnd2024:rules/weapons_finesse
            attack_and_damage_ability_choice:
            - strength
            - dexterity
            same_ability_for_both: true
          - id: light
            rule:
              key: dnd2024:rules/weapons_light
            trigger: attack_with_light_weapon_during_own_attack_action
            extra_attack:
              count: 1
              economy: bonus_action
              timing: later_same_turn
              weapon: different_light_weapon
              damage_ability_modifier: negative_only
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: nick
            rule:
              key: dnd2024:rules/weapons_nick
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: light_property_extra_attack
              replace_economy: bonus_action
              economy: part_of_attack_action
              maximum_per_turn: 1
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_scimitar
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_scimitar
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/shortsword:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d6
          type: piercing
        properties_text: Finesse, Light
        mastery: vex
        weapon_category: martial melee
        price:
          value: 10
          unit: gp
        mass:
          value: 2.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/shortsword
        implementation: dnd2024:equipment_shortsword
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d6
            type: piercing
            amount:
              dice:
                count: 1
                sides: 6
              bonus: 0
          properties:
          - id: finesse
            rule:
              key: dnd2024:rules/weapons_finesse
            attack_and_damage_ability_choice:
            - strength
            - dexterity
            same_ability_for_both: true
          - id: light
            rule:
              key: dnd2024:rules/weapons_light
            trigger: attack_with_light_weapon_during_own_attack_action
            extra_attack:
              count: 1
              economy: bonus_action
              timing: later_same_turn
              weapon: different_light_weapon
              damage_ability_modifier: negative_only
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: vex
            rule:
              key: dnd2024:rules/weapons_vex
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              effect:
                attacker_attack_roll_mode: advantage
                against: same_target
                count: 1
                expiry: attacker_next_turn_end
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_shortsword
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_shortsword
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/trident:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d8
          type: piercing
        properties_text: Thrown (Range 20/60), Versatile (1d10)
        mastery: topple
        weapon_category: martial melee
        price:
          value: 5
          unit: gp
        mass:
          value: 4.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/trident
        implementation: dnd2024:equipment_trident
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d8
            type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              bonus: 0
          properties:
          - id: thrown
            rule:
              key: dnd2024:rules/weapons_thrown
            range:
              normal_ft: 20
              long_ft: 60
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            draw_as_part_of_attack: true
            attack_type: ranged
            melee_weapon_uses_melee_ability: true
          - id: versatile
            rule:
              key: dnd2024:rules/weapons_versatile
            two_hand_melee_damage:
              expression: 1d10
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: topple
            rule:
              key: dnd2024:rules/weapons_topple
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              optional: true
              save:
                ability: constitution
                dc:
                  op: add
                  operands:
                  - 8
                  - stat: attack_ability_modifier
                  - stat: proficiency_bonus
                failure:
                  condition: prone
                success: no_effect
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_trident
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_trident
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/warhammer:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d8
          type: bludgeoning
        properties_text: Versatile (1d10)
        mastery: push
        weapon_category: martial melee
        price:
          value: 15
          unit: gp
        mass:
          value: 5.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/warhammer
        implementation: dnd2024:equipment_warhammer
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d8
            type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 8
              bonus: 0
          properties:
          - id: versatile
            rule:
              key: dnd2024:rules/weapons_versatile
            two_hand_melee_damage:
              expression: 1d10
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: push
            rule:
              key: dnd2024:rules/weapons_push
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              optional: true
              maximum_target_size: large
              push_maximum_ft: 10
              direction: straight_away_from_attacker
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_warhammer
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_warhammer
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/war_pick:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d8
          type: piercing
        properties_text: Versatile (1d10)
        mastery: sap
        weapon_category: martial melee
        price:
          value: 5
          unit: gp
        mass:
          value: 2.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/war_pick
        implementation: dnd2024:equipment_war_pick
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d8
            type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              bonus: 0
          properties:
          - id: versatile
            rule:
              key: dnd2024:rules/weapons_versatile
            two_hand_melee_damage:
              expression: 1d10
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: sap
            rule:
              key: dnd2024:rules/weapons_sap
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              effect:
                attack_roll_mode: disadvantage
                count: 1
                expiry: attacker_next_turn_start
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_war_pick
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_war_pick
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/whip:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d4
          type: slashing
        properties_text: Finesse, Reach
        mastery: slow
        weapon_category: martial melee
        price:
          value: 2
          unit: gp
        mass:
          value: 3.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/whip
        implementation: dnd2024:equipment_whip
        parameters:
          weapon_category: martial_melee
          damage:
            expression: 1d4
            type: slashing
            amount:
              dice:
                count: 1
                sides: 4
              bonus: 0
          properties:
          - id: finesse
            rule:
              key: dnd2024:rules/weapons_finesse
            attack_and_damage_ability_choice:
            - strength
            - dexterity
            same_ability_for_both: true
          - id: reach
            rule:
              key: dnd2024:rules/weapons_reach
            reach_bonus_ft: 5
            applies_to:
            - attacks
            - opportunity_attack_reach
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: strength
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: slow
            rule:
              key: dnd2024:rules/weapons_slow
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              optional: true
              speed_reduction_ft: 10
              expiry: attacker_next_turn_start
              maximum_stacked_reduction_ft: 10
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_whip
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_whip
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/blowgun:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: '1'
          type: piercing
        properties_text: Ammunition (Range 25/100; Needle), Loading
        mastery: vex
        weapon_category: martial ranged
        price:
          value: 10
          unit: gp
        mass:
          value: 1.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/blowgun
        implementation: dnd2024:equipment_blowgun
        parameters:
          weapon_category: martial_ranged
          damage:
            expression: '1'
            type: piercing
            amount: 1
          properties:
          - id: ammunition
            rule:
              key: dnd2024:rules/weapons_ammunition
            range:
              normal_ft: 25
              long_ft: 100
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            ammunition:
              key: dnd2024:equipment/ammunition_varies
              variant: needles
            expend_per_attack: 1
            draw_as_part_of_attack: true
            free_hand_to_load_if_one_handed: true
            recovery:
              duration:
                value: 1
                unit: minute
              timing: after_fight
              fraction: 0.5
              rounding: down
              unrecovered: lost
          - id: loading
            rule:
              key: dnd2024:rules/weapons_loading
            ammunition_limit: 1
            per:
            - action
            - bonus_action
            - reaction
            ignores_extra_attacks: true
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: dexterity
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: vex
            rule:
              key: dnd2024:rules/weapons_vex
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              effect:
                attacker_attack_roll_mode: advantage
                against: same_target
                count: 1
                expiry: attacker_next_turn_end
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_blowgun
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_blowgun
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/hand_crossbow:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d6
          type: piercing
        properties_text: Ammunition (Range 30/120; Bolt), Light, Loading
        mastery: vex
        weapon_category: martial ranged
        price:
          value: 75
          unit: gp
        mass:
          value: 3.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/hand_crossbow
        implementation: dnd2024:equipment_hand_crossbow
        parameters:
          weapon_category: martial_ranged
          damage:
            expression: 1d6
            type: piercing
            amount:
              dice:
                count: 1
                sides: 6
              bonus: 0
          properties:
          - id: ammunition
            rule:
              key: dnd2024:rules/weapons_ammunition
            range:
              normal_ft: 30
              long_ft: 120
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            ammunition:
              key: dnd2024:equipment/ammunition_varies
              variant: bolts
            expend_per_attack: 1
            draw_as_part_of_attack: true
            free_hand_to_load_if_one_handed: true
            recovery:
              duration:
                value: 1
                unit: minute
              timing: after_fight
              fraction: 0.5
              rounding: down
              unrecovered: lost
          - id: light
            rule:
              key: dnd2024:rules/weapons_light
            trigger: attack_with_light_weapon_during_own_attack_action
            extra_attack:
              count: 1
              economy: bonus_action
              timing: later_same_turn
              weapon: different_light_weapon
              damage_ability_modifier: negative_only
          - id: loading
            rule:
              key: dnd2024:rules/weapons_loading
            ammunition_limit: 1
            per:
            - action
            - bonus_action
            - reaction
            ignores_extra_attacks: true
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: dexterity
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: vex
            rule:
              key: dnd2024:rules/weapons_vex
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              effect:
                attacker_attack_roll_mode: advantage
                against: same_target
                count: 1
                expiry: attacker_next_turn_end
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_hand_crossbow
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_hand_crossbow
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/heavy_crossbow:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d10
          type: piercing
        properties_text: Ammunition (Range 100/400; Bolt), Heavy, Loading, Two-Handed
        mastery: push
        weapon_category: martial ranged
        price:
          value: 50
          unit: gp
        mass:
          value: 18.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/heavy_crossbow
        implementation: dnd2024:equipment_heavy_crossbow
        parameters:
          weapon_category: martial_ranged
          damage:
            expression: 1d10
            type: piercing
            amount:
              dice:
                count: 1
                sides: 10
              bonus: 0
          properties:
          - id: ammunition
            rule:
              key: dnd2024:rules/weapons_ammunition
            range:
              normal_ft: 100
              long_ft: 400
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            ammunition:
              key: dnd2024:equipment/ammunition_varies
              variant: bolts
            expend_per_attack: 1
            draw_as_part_of_attack: true
            free_hand_to_load_if_one_handed: true
            recovery:
              duration:
                value: 1
                unit: minute
              timing: after_fight
              fraction: 0.5
              rounding: down
              unrecovered: lost
          - id: heavy
            rule:
              key: dnd2024:rules/weapons_heavy
            required_ability: dexterity
            minimum_score: 13
            below_minimum_attack_mode: disadvantage
          - id: loading
            rule:
              key: dnd2024:rules/weapons_loading
            ammunition_limit: 1
            per:
            - action
            - bonus_action
            - reaction
            ignores_extra_attacks: true
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: dexterity
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: push
            rule:
              key: dnd2024:rules/weapons_push
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_creature
              optional: true
              maximum_target_size: large
              push_maximum_ft: 10
              direction: straight_away_from_attacker
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_heavy_crossbow
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_heavy_crossbow
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/longbow:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d8
          type: piercing
        properties_text: Ammunition (Range 150/600; Arrow), Heavy, Two-Handed
        mastery: slow
        weapon_category: martial ranged
        price:
          value: 50
          unit: gp
        mass:
          value: 2.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/longbow
        implementation: dnd2024:equipment_longbow
        parameters:
          weapon_category: martial_ranged
          damage:
            expression: 1d8
            type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              bonus: 0
          properties:
          - id: ammunition
            rule:
              key: dnd2024:rules/weapons_ammunition
            range:
              normal_ft: 150
              long_ft: 600
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            ammunition:
              key: dnd2024:equipment/ammunition_varies
              variant: arrows
            expend_per_attack: 1
            draw_as_part_of_attack: true
            free_hand_to_load_if_one_handed: true
            recovery:
              duration:
                value: 1
                unit: minute
              timing: after_fight
              fraction: 0.5
              rounding: down
              unrecovered: lost
          - id: heavy
            rule:
              key: dnd2024:rules/weapons_heavy
            required_ability: dexterity
            minimum_score: 13
            below_minimum_attack_mode: disadvantage
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: dexterity
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: slow
            rule:
              key: dnd2024:rules/weapons_slow
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              optional: true
              speed_reduction_ft: 10
              expiry: attacker_next_turn_start
              maximum_stacked_reduction_ft: 10
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_longbow
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_longbow
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/musket:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d12
          type: piercing
        properties_text: Ammunition (Range 40/120; Bullet), Loading, Two-Handed
        mastery: slow
        weapon_category: martial ranged
        price:
          value: 500
          unit: gp
        mass:
          value: 10.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/musket
        implementation: dnd2024:equipment_musket
        parameters:
          weapon_category: martial_ranged
          damage:
            expression: 1d12
            type: piercing
            amount:
              dice:
                count: 1
                sides: 12
              bonus: 0
          properties:
          - id: ammunition
            rule:
              key: dnd2024:rules/weapons_ammunition
            range:
              normal_ft: 40
              long_ft: 120
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            ammunition:
              key: dnd2024:equipment/ammunition_varies
              variant: firearm_bullets
            expend_per_attack: 1
            draw_as_part_of_attack: true
            free_hand_to_load_if_one_handed: true
            recovery:
              duration:
                value: 1
                unit: minute
              timing: after_fight
              fraction: 0.5
              rounding: down
              unrecovered: lost
          - id: loading
            rule:
              key: dnd2024:rules/weapons_loading
            ammunition_limit: 1
            per:
            - action
            - bonus_action
            - reaction
            ignores_extra_attacks: true
          - id: two_handed
            rule:
              key: dnd2024:rules/weapons_two_handed
            hands_to_attack: 2
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: dexterity
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: slow
            rule:
              key: dnd2024:rules/weapons_slow
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              optional: true
              speed_reduction_ft: 10
              expiry: attacker_next_turn_start
              maximum_stacked_reduction_ft: 10
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_musket
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_musket
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/pistol:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        damage:
          expression: 1d10
          type: piercing
        properties_text: Ammunition (Range 30/90; Bullet), Loading
        mastery: vex
        weapon_category: martial ranged
        price:
          value: 250
          unit: gp
        mass:
          value: 3.0
          unit: lb
        category: weapon
      splat_rule:
        mechanic: dnd2024:equipment/pistol
        implementation: dnd2024:equipment_pistol
        parameters:
          weapon_category: martial_ranged
          damage:
            expression: 1d10
            type: piercing
            amount:
              dice:
                count: 1
                sides: 10
              bonus: 0
          properties:
          - id: ammunition
            rule:
              key: dnd2024:rules/weapons_ammunition
            range:
              normal_ft: 30
              long_ft: 90
              normal_range_mode: normal
              long_range_mode: disadvantage
              beyond_long_range: cannot_attack
              rule:
                key: dnd2024:rules/weapons_range
            ammunition:
              key: dnd2024:equipment/ammunition_varies
              variant: firearm_bullets
            expend_per_attack: 1
            draw_as_part_of_attack: true
            free_hand_to_load_if_one_handed: true
            recovery:
              duration:
                value: 1
                unit: minute
              timing: after_fight
              fraction: 0.5
              rounding: down
              unrecovered: lost
          - id: loading
            rule:
              key: dnd2024:rules/weapons_loading
            ammunition_limit: 1
            per:
            - action
            - bonus_action
            - reaction
            ignores_extra_attacks: true
          attack:
            proficiency_bonus_if_proficient: true
            base_ability: dexterity
            normal_hit_damage_ability_modifier: attack_ability_modifier
          mastery:
            property: vex
            rule:
              key: dnd2024:rules/weapons_vex
            requires_weapon_mastery_feature_for_this_weapon: true
            automatic_from_weapon_proficiency: false
            parameters:
              trigger: hit_and_damage_creature
              effect:
                attacker_attack_roll_mode: advantage
                against: same_target
                count: 1
                expiry: attacker_next_turn_end
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_pistol
      inputs:
        actor: participant
        item: item_instance
        target:
          type: participant
          required: false
        event: event
        choices:
          type: choice
          required: false
      outputs:
      - attack_options
      - attack_modifiers
      - damage
      - mastery_effects
      - ammunition_updates
      events:
      - attack.pending
      - attack.hit
      - attack.miss
      - damage.resolved
      - combat.ended
      capabilities:
      - weapon_properties
      - weapon_mastery
      - attacks
      - effects
      - ammunition
    requires:
    - dnd2024:equipment_pistol
    citations:
    - SRD 5.2.1 pp. 91; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 pp. 89–91, weapon properties and mastery; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/padded_armor:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: light armor
          stealth_disadvantage: true
          base_ac: 11
          add_dexterity_modifier: true
        price:
          value: 5
          unit: gp
        mass:
          value: 8.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/padded_armor
        implementation: dnd2024:equipment_padded_armor
        parameters:
          armor:
            category: light armor
            stealth_disadvantage: true
            base_ac: 11
            add_dexterity_modifier: true
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 1
              unit: minute
            doff:
              value: 1
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          stealth:
            ability: dexterity
            skill: stealth
            mode: disadvantage
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_padded_armor
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_padded_armor
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/leather_armor:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: light armor
          stealth_disadvantage: false
          base_ac: 11
          add_dexterity_modifier: true
        price:
          value: 10
          unit: gp
        mass:
          value: 10.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/leather_armor
        implementation: dnd2024:equipment_leather_armor
        parameters:
          armor:
            category: light armor
            stealth_disadvantage: false
            base_ac: 11
            add_dexterity_modifier: true
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 1
              unit: minute
            doff:
              value: 1
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_leather_armor
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_leather_armor
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/studded_leather_armor:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: light armor
          stealth_disadvantage: false
          base_ac: 12
          add_dexterity_modifier: true
        price:
          value: 45
          unit: gp
        mass:
          value: 13.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/studded_leather_armor
        implementation: dnd2024:equipment_studded_leather_armor
        parameters:
          armor:
            category: light armor
            stealth_disadvantage: false
            base_ac: 12
            add_dexterity_modifier: true
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 1
              unit: minute
            doff:
              value: 1
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_studded_leather_armor
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_studded_leather_armor
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/hide_armor:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: medium armor
          stealth_disadvantage: false
          base_ac: 12
          add_dexterity_modifier: true
          dexterity_modifier_max: 2
        price:
          value: 10
          unit: gp
        mass:
          value: 12.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/hide_armor
        implementation: dnd2024:equipment_hide_armor
        parameters:
          armor:
            category: medium armor
            stealth_disadvantage: false
            base_ac: 12
            add_dexterity_modifier: true
            dexterity_modifier_max: 2
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 5
              unit: minute
            doff:
              value: 1
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_hide_armor
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_hide_armor
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/chain_shirt:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: medium armor
          stealth_disadvantage: false
          base_ac: 13
          add_dexterity_modifier: true
          dexterity_modifier_max: 2
        price:
          value: 50
          unit: gp
        mass:
          value: 20.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/chain_shirt
        implementation: dnd2024:equipment_chain_shirt
        parameters:
          armor:
            category: medium armor
            stealth_disadvantage: false
            base_ac: 13
            add_dexterity_modifier: true
            dexterity_modifier_max: 2
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 5
              unit: minute
            doff:
              value: 1
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_chain_shirt
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_chain_shirt
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/scale_mail:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: medium armor
          stealth_disadvantage: true
          base_ac: 14
          add_dexterity_modifier: true
          dexterity_modifier_max: 2
        price:
          value: 50
          unit: gp
        mass:
          value: 45.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/scale_mail
        implementation: dnd2024:equipment_scale_mail
        parameters:
          armor:
            category: medium armor
            stealth_disadvantage: true
            base_ac: 14
            add_dexterity_modifier: true
            dexterity_modifier_max: 2
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 5
              unit: minute
            doff:
              value: 1
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          stealth:
            ability: dexterity
            skill: stealth
            mode: disadvantage
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_scale_mail
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_scale_mail
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/breastplate:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: medium armor
          stealth_disadvantage: false
          base_ac: 14
          add_dexterity_modifier: true
          dexterity_modifier_max: 2
        price:
          value: 400
          unit: gp
        mass:
          value: 20.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/breastplate
        implementation: dnd2024:equipment_breastplate
        parameters:
          armor:
            category: medium armor
            stealth_disadvantage: false
            base_ac: 14
            add_dexterity_modifier: true
            dexterity_modifier_max: 2
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 5
              unit: minute
            doff:
              value: 1
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_breastplate
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_breastplate
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/half_plate_armor:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: medium armor
          stealth_disadvantage: true
          base_ac: 15
          add_dexterity_modifier: true
          dexterity_modifier_max: 2
        price:
          value: 750
          unit: gp
        mass:
          value: 40.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/half_plate_armor
        implementation: dnd2024:equipment_half_plate_armor
        parameters:
          armor:
            category: medium armor
            stealth_disadvantage: true
            base_ac: 15
            add_dexterity_modifier: true
            dexterity_modifier_max: 2
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 5
              unit: minute
            doff:
              value: 1
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          stealth:
            ability: dexterity
            skill: stealth
            mode: disadvantage
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_half_plate_armor
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_half_plate_armor
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/ring_mail:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: heavy armor
          stealth_disadvantage: true
          base_ac: 14
          add_dexterity_modifier: false
        price:
          value: 30
          unit: gp
        mass:
          value: 40.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/ring_mail
        implementation: dnd2024:equipment_ring_mail
        parameters:
          armor:
            category: heavy armor
            stealth_disadvantage: true
            base_ac: 14
            add_dexterity_modifier: false
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 10
              unit: minute
            doff:
              value: 5
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          stealth:
            ability: dexterity
            skill: stealth
            mode: disadvantage
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_ring_mail
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_ring_mail
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/chain_mail:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: heavy armor
          stealth_disadvantage: true
          base_ac: 16
          add_dexterity_modifier: false
          strength_minimum: 13
        price:
          value: 75
          unit: gp
        mass:
          value: 55.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/chain_mail
        implementation: dnd2024:equipment_chain_mail
        parameters:
          armor:
            category: heavy armor
            stealth_disadvantage: true
            base_ac: 16
            add_dexterity_modifier: false
            strength_minimum: 13
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 10
              unit: minute
            doff:
              value: 5
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          stealth:
            ability: dexterity
            skill: stealth
            mode: disadvantage
          strength_penalty:
            if_strength_below: 13
            speed_reduction_ft: 10
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_chain_mail
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_chain_mail
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/splint_armor:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: heavy armor
          stealth_disadvantage: true
          base_ac: 17
          add_dexterity_modifier: false
          strength_minimum: 15
        price:
          value: 200
          unit: gp
        mass:
          value: 60.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/splint_armor
        implementation: dnd2024:equipment_splint_armor
        parameters:
          armor:
            category: heavy armor
            stealth_disadvantage: true
            base_ac: 17
            add_dexterity_modifier: false
            strength_minimum: 15
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 10
              unit: minute
            doff:
              value: 5
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          stealth:
            ability: dexterity
            skill: stealth
            mode: disadvantage
          strength_penalty:
            if_strength_below: 15
            speed_reduction_ft: 10
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_splint_armor
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_splint_armor
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/plate_armor:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: heavy armor
          stealth_disadvantage: true
          base_ac: 18
          add_dexterity_modifier: false
          strength_minimum: 15
        price:
          value: 1500
          unit: gp
        mass:
          value: 65.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/plate_armor
        implementation: dnd2024:equipment_plate_armor
        parameters:
          armor:
            category: heavy armor
            stealth_disadvantage: true
            base_ac: 18
            add_dexterity_modifier: false
            strength_minimum: 15
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don:
              value: 10
              unit: minute
            doff:
              value: 5
              unit: minute
          training:
            without_training:
              d20_test_disadvantage_abilities:
              - strength
              - dexterity
              can_cast_spells: false
          stacking_group: body_armor
          stealth:
            ability: dexterity
            skill: stealth
            mode: disadvantage
          strength_penalty:
            if_strength_below: 15
            speed_reduction_ft: 10
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_plate_armor
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_plate_armor
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:equipment/shield:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_item:
        armor:
          category: shield
          stealth_disadvantage: false
          ac_bonus: 2
        price:
          value: 10
          unit: gp
        mass:
          value: 6.0
          unit: lb
        category: armor
      splat_rule:
        mechanic: dnd2024:equipment/shield
        implementation: dnd2024:equipment_shield
        parameters:
          armor:
            category: shield
            stealth_disadvantage: false
            ac_bonus: 2
          while_equipped: true
          maximum_simultaneous_same_category: 1
          equip:
            don: utilize
            doff: utilize
            requires_wielding: true
          training:
            ac_bonus_requires_training: true
          stacking_group: shield
          application:
            actor: equipper
            item: equipped_instance
            inactive_if_not_equipped: true
    contract:
      id: dnd2024:equipment_shield
      inputs:
        actor: participant
        item: item_instance
        event: event
      outputs:
      - armor_class
      - equipment_constraints
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
      events:
      - item.equipped
      - item.unequipped
      - ability_check.pending
      - spell.cast_pending
      capabilities:
      - equipment
      - armor_class
      - roll_modifiers
      - speed_modifiers
      - spellcasting_eligibility
    requires:
    - dnd2024:equipment_shield
    citations:
    - SRD 5.2.1 pp. 92; builtin/srd-5.2.1/source.txt
    - SRD 5.2.1 p. 92, armor training and don/doff times; builtin/srd-5.2.1/source.txt
  dnd2024:tables/barbarian_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/barbarian_progression
        implementation: dnd2024:class_progression_barbarian_progression
        parameters:
          class: barbarian
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Rage, Unarmored Defense, Weapon Mastery
            rages: 2
            rage_damage: 2
            weapon_mastery: 2
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_1_rage
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_1_unarmored_defense
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_1_weapon_mastery
              at_class_level: 1
          - level: 2
            proficiency_bonus: 2
            features_text: Danger Sense, Reckless Attack
            rages: 2
            rage_damage: 2
            weapon_mastery: 2
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_2_danger_sense
              at_class_level: 2
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_2_reckless_attack
              at_class_level: 2
          - level: 3
            proficiency_bonus: 2
            features_text: Barbarian Subclass, Primal Knowledge
            rages: 3
            rage_damage: 2
            weapon_mastery: 2
            grants:
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/barbarian_level_3_barbarian_subclass
              at_class_level: 3
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_3_primal_knowledge
              at_class_level: 3
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement
            rages: 3
            rage_damage: 2
            weapon_mastery: 3
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
              at_class_level: 4
          - level: 5
            proficiency_bonus: 3
            features_text: Extra Attack, Fast Movement
            rages: 3
            rage_damage: 2
            weapon_mastery: 3
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_5_extra_attack
              at_class_level: 5
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_5_fast_movement
              at_class_level: 5
          - level: 6
            proficiency_bonus: 3
            features_text: Subclass feature
            rages: 4
            rage_damage: 2
            weapon_mastery: 3
            grants:
            - operation: grant_selected_subclass_features
              class_level: 6
          - level: 7
            proficiency_bonus: 3
            features_text: Feral Instinct, Instinctive Pounce
            rages: 4
            rage_damage: 2
            weapon_mastery: 3
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_7_feral_instinct
              at_class_level: 7
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_7_instinctive_pounce
              at_class_level: 7
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            rages: 4
            rage_damage: 2
            weapon_mastery: 3
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
              at_class_level: 8
          - level: 9
            proficiency_bonus: 4
            features_text: Brutal Strike
            rages: 4
            rage_damage: 3
            weapon_mastery: 3
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_9_brutal_strike
              at_class_level: 9
          - level: 10
            proficiency_bonus: 4
            features_text: Subclass feature
            rages: 4
            rage_damage: 3
            weapon_mastery: 4
            grants:
            - operation: grant_selected_subclass_features
              class_level: 10
          - level: 11
            proficiency_bonus: 4
            features_text: Relentless Rage
            rages: 4
            rage_damage: 3
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_11_relentless_rage
              at_class_level: 11
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            rages: 5
            rage_damage: 3
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
              at_class_level: 12
          - level: 13
            proficiency_bonus: 5
            features_text: Improved Brutal Strike
            rages: 5
            rage_damage: 3
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_13_improved_brutal_strike
              at_class_level: 13
          - level: 14
            proficiency_bonus: 5
            features_text: Subclass feature
            rages: 5
            rage_damage: 3
            weapon_mastery: 4
            grants:
            - operation: grant_selected_subclass_features
              class_level: 14
          - level: 15
            proficiency_bonus: 5
            features_text: Persistent Rage
            rages: 5
            rage_damage: 3
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_15_persistent_rage
              at_class_level: 15
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            rages: 5
            rage_damage: 4
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
              at_class_level: 16
          - level: 17
            proficiency_bonus: 6
            features_text: Improved Brutal Strike
            rages: 6
            rage_damage: 4
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_17_improved_brutal_strike
              at_class_level: 17
          - level: 18
            proficiency_bonus: 6
            features_text: Indomitable Might
            rages: 6
            rage_damage: 4
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_18_indomitable_might
              at_class_level: 18
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            rages: 6
            rage_damage: 4
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_19_epic_boon
              at_class_level: 19
          - level: 20
            proficiency_bonus: 6
            features_text: Primal Champion
            rages: 6
            rage_damage: 4
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/barbarian_level_20_primal_champion
              at_class_level: 20
          class_definition:
            key: dnd2024:classes/barbarian
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
            rage_damage:
              kind: damage_bonus
              requires: rage_feature_eligibility
            rages:
              kind: resource_capacity
              resource: rages
              recovery: owning_feature_contract
              level_change_refills: false
            weapon_mastery:
              kind: selection_capacity
              selection: weapon_mastery
              eligibility: owning_feature_contract
    contract:
      id: dnd2024:class_progression_barbarian_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_barbarian_progression
    citations:
    - SRD 5.2.1 pp. 28; builtin/srd-5.2.1/source.txt
  dnd2024:tables/bard_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/bard_progression
        implementation: dnd2024:class_progression_bard_progression
        parameters:
          class: bard
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Bardic Inspiration, Spellcasting
            bardic_die: d6
            cantrips: 2
            prepared_spells: 4
            spell_slots_1: 2
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_1_bardic_inspiration
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_1_spellcasting
              at_class_level: 1
            bardic_die_dice:
              dice:
                count: 1
                sides: 6
              bonus: 0
          - level: 2
            proficiency_bonus: 2
            features_text: Expertise, Jack of All Trades
            bardic_die: d6
            cantrips: 2
            prepared_spells: 5
            spell_slots_1: 3
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_2_expertise
              at_class_level: 2
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_2_jack_of_all_trades
              at_class_level: 2
            bardic_die_dice:
              dice:
                count: 1
                sides: 6
              bonus: 0
          - level: 3
            proficiency_bonus: 2
            features_text: Bard Subclass
            bardic_die: d6
            cantrips: 2
            prepared_spells: 6
            spell_slots_1: 4
            spell_slots_2: 2
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/bard_level_3_bard_subclass
              at_class_level: 3
            bardic_die_dice:
              dice:
                count: 1
                sides: 6
              bonus: 0
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement
            bardic_die: d6
            cantrips: 3
            prepared_spells: 7
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_4_ability_score_improvement
              at_class_level: 4
            bardic_die_dice:
              dice:
                count: 1
                sides: 6
              bonus: 0
          - level: 5
            proficiency_bonus: 3
            features_text: Font of Inspiration
            bardic_die: d8
            cantrips: 3
            prepared_spells: 9
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 2
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_5_font_of_inspiration
              at_class_level: 5
            bardic_die_dice:
              dice:
                count: 1
                sides: 8
              bonus: 0
          - level: 6
            proficiency_bonus: 3
            features_text: Subclass feature
            bardic_die: d8
            cantrips: 3
            prepared_spells: 10
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 6
            bardic_die_dice:
              dice:
                count: 1
                sides: 8
              bonus: 0
          - level: 7
            proficiency_bonus: 3
            features_text: Countercharm
            bardic_die: d8
            cantrips: 3
            prepared_spells: 11
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 1
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_7_countercharm
              at_class_level: 7
            bardic_die_dice:
              dice:
                count: 1
                sides: 8
              bonus: 0
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            bardic_die: d8
            cantrips: 3
            prepared_spells: 12
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 2
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_4_ability_score_improvement
              at_class_level: 8
            bardic_die_dice:
              dice:
                count: 1
                sides: 8
              bonus: 0
          - level: 9
            proficiency_bonus: 4
            features_text: Expertise
            bardic_die: d8
            cantrips: 3
            prepared_spells: 14
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 1
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_2_expertise
              at_class_level: 9
            bardic_die_dice:
              dice:
                count: 1
                sides: 8
              bonus: 0
          - level: 10
            proficiency_bonus: 4
            features_text: Magical Secrets
            bardic_die: d10
            cantrips: 4
            prepared_spells: 15
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_10_magical_secrets
              at_class_level: 10
            bardic_die_dice:
              dice:
                count: 1
                sides: 10
              bonus: 0
          - level: 11
            proficiency_bonus: 4
            features_text: —
            bardic_die: d10
            cantrips: 4
            prepared_spells: 16
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
            bardic_die_dice:
              dice:
                count: 1
                sides: 10
              bonus: 0
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            bardic_die: d10
            cantrips: 4
            prepared_spells: 16
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_4_ability_score_improvement
              at_class_level: 12
            bardic_die_dice:
              dice:
                count: 1
                sides: 10
              bonus: 0
          - level: 13
            proficiency_bonus: 5
            features_text: —
            bardic_die: d10
            cantrips: 4
            prepared_spells: 17
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
            bardic_die_dice:
              dice:
                count: 1
                sides: 10
              bonus: 0
          - level: 14
            proficiency_bonus: 5
            features_text: Subclass feature
            bardic_die: d10
            cantrips: 4
            prepared_spells: 17
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 14
            bardic_die_dice:
              dice:
                count: 1
                sides: 10
              bonus: 0
          - level: 15
            proficiency_bonus: 5
            features_text: —
            bardic_die: d12
            cantrips: 4
            prepared_spells: 18
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 0
            grants: []
            bardic_die_dice:
              dice:
                count: 1
                sides: 12
              bonus: 0
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            bardic_die: d12
            cantrips: 4
            prepared_spells: 18
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_4_ability_score_improvement
              at_class_level: 16
            bardic_die_dice:
              dice:
                count: 1
                sides: 12
              bonus: 0
          - level: 17
            proficiency_bonus: 6
            features_text: —
            bardic_die: d12
            cantrips: 4
            prepared_spells: 19
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants: []
            bardic_die_dice:
              dice:
                count: 1
                sides: 12
              bonus: 0
          - level: 18
            proficiency_bonus: 6
            features_text: Superior Inspiration
            bardic_die: d12
            cantrips: 4
            prepared_spells: 20
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_18_superior_inspiration
              at_class_level: 18
            bardic_die_dice:
              dice:
                count: 1
                sides: 12
              bonus: 0
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            bardic_die: d12
            cantrips: 4
            prepared_spells: 21
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 2
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_19_epic_boon
              at_class_level: 19
            bardic_die_dice:
              dice:
                count: 1
                sides: 12
              bonus: 0
          - level: 20
            proficiency_bonus: 6
            features_text: Words of Creation
            bardic_die: d12
            cantrips: 4
            prepared_spells: 22
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 2
            spell_slots_7: 2
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/bard_level_20_words_of_creation
              at_class_level: 20
            bardic_die_dice:
              dice:
                count: 1
                sides: 12
              bonus: 0
          class_definition:
            key: dnd2024:classes/bard
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            bardic_die:
              kind: dice_expression
              applies_to: bardic_die
            cantrips:
              kind: selection_capacity
              selection: cantrips
              eligibility: owning_feature_contract
            prepared_spells:
              kind: selection_capacity
              selection: prepared_spells
              eligibility: owning_feature_contract
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
            spell_slots_1:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 1
              recovery: long_rest
            spell_slots_2:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 2
              recovery: long_rest
            spell_slots_3:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 3
              recovery: long_rest
            spell_slots_4:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 4
              recovery: long_rest
            spell_slots_5:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 5
              recovery: long_rest
            spell_slots_6:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 6
              recovery: long_rest
            spell_slots_7:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 7
              recovery: long_rest
            spell_slots_8:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 8
              recovery: long_rest
            spell_slots_9:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 9
              recovery: long_rest
    contract:
      id: dnd2024:class_progression_bard_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_bard_progression
    citations:
    - SRD 5.2.1 pp. 31; builtin/srd-5.2.1/source.txt
  dnd2024:tables/cleric_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/cleric_progression
        implementation: dnd2024:class_progression_cleric_progression
        parameters:
          class: cleric
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Spellcasting, Divine Order
            channel_divinity: 0
            cantrips: 3
            prepared_spells: 4
            spell_slots_1: 2
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_1_spellcasting
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_1_divine_order
              at_class_level: 1
          - level: 2
            proficiency_bonus: 2
            features_text: Channel Divinity
            channel_divinity: 2
            cantrips: 3
            prepared_spells: 5
            spell_slots_1: 3
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_2_channel_divinity
              at_class_level: 2
          - level: 3
            proficiency_bonus: 2
            features_text: Cleric Subclass
            channel_divinity: 2
            cantrips: 3
            prepared_spells: 6
            spell_slots_1: 4
            spell_slots_2: 2
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/cleric_level_3_cleric_subclass
              at_class_level: 3
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement
            channel_divinity: 2
            cantrips: 4
            prepared_spells: 7
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_4_ability_score_improvement
              at_class_level: 4
          - level: 5
            proficiency_bonus: 3
            features_text: Sear Undead
            channel_divinity: 2
            cantrips: 4
            prepared_spells: 9
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 2
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_5_sear_undead
              at_class_level: 5
          - level: 6
            proficiency_bonus: 3
            features_text: Subclass feature
            channel_divinity: 3
            cantrips: 4
            prepared_spells: 10
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 6
          - level: 7
            proficiency_bonus: 3
            features_text: Blessed Strikes
            channel_divinity: 3
            cantrips: 4
            prepared_spells: 11
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 1
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_7_blessed_strikes
              at_class_level: 7
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            channel_divinity: 3
            cantrips: 4
            prepared_spells: 12
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 2
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_4_ability_score_improvement
              at_class_level: 8
          - level: 9
            proficiency_bonus: 4
            features_text: —
            channel_divinity: 3
            cantrips: 4
            prepared_spells: 14
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 1
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 10
            proficiency_bonus: 4
            features_text: Divine Intervention
            channel_divinity: 3
            cantrips: 5
            prepared_spells: 15
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_10_divine_intervention
              at_class_level: 10
          - level: 11
            proficiency_bonus: 4
            features_text: —
            channel_divinity: 3
            cantrips: 5
            prepared_spells: 16
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            channel_divinity: 3
            cantrips: 5
            prepared_spells: 16
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_4_ability_score_improvement
              at_class_level: 12
          - level: 13
            proficiency_bonus: 5
            features_text: —
            channel_divinity: 3
            cantrips: 5
            prepared_spells: 17
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 14
            proficiency_bonus: 5
            features_text: Improved Blessed Strikes
            channel_divinity: 3
            cantrips: 5
            prepared_spells: 17
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
              at_class_level: 14
          - level: 15
            proficiency_bonus: 5
            features_text: —
            channel_divinity: 3
            cantrips: 5
            prepared_spells: 18
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 0
            grants: []
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            channel_divinity: 3
            cantrips: 5
            prepared_spells: 18
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_4_ability_score_improvement
              at_class_level: 16
          - level: 17
            proficiency_bonus: 6
            features_text: Subclass feature
            channel_divinity: 3
            cantrips: 5
            prepared_spells: 19
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_selected_subclass_features
              class_level: 17
          - level: 18
            proficiency_bonus: 6
            features_text: —
            channel_divinity: 4
            cantrips: 5
            prepared_spells: 20
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants: []
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            channel_divinity: 4
            cantrips: 5
            prepared_spells: 21
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 2
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_19_epic_boon
              at_class_level: 19
          - level: 20
            proficiency_bonus: 6
            features_text: Greater Divine Intervention
            channel_divinity: 4
            cantrips: 5
            prepared_spells: 22
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 2
            spell_slots_7: 2
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/cleric_level_20_greater_divine_intervention
              at_class_level: 20
          class_definition:
            key: dnd2024:classes/cleric
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            cantrips:
              kind: selection_capacity
              selection: cantrips
              eligibility: owning_feature_contract
            channel_divinity:
              kind: resource_capacity
              resource: channel_divinity
              recovery: owning_feature_contract
              level_change_refills: false
            prepared_spells:
              kind: selection_capacity
              selection: prepared_spells
              eligibility: owning_feature_contract
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
            spell_slots_1:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 1
              recovery: long_rest
            spell_slots_2:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 2
              recovery: long_rest
            spell_slots_3:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 3
              recovery: long_rest
            spell_slots_4:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 4
              recovery: long_rest
            spell_slots_5:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 5
              recovery: long_rest
            spell_slots_6:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 6
              recovery: long_rest
            spell_slots_7:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 7
              recovery: long_rest
            spell_slots_8:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 8
              recovery: long_rest
            spell_slots_9:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 9
              recovery: long_rest
    contract:
      id: dnd2024:class_progression_cleric_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_cleric_progression
    citations:
    - SRD 5.2.1 pp. 36; builtin/srd-5.2.1/source.txt
  dnd2024:tables/druid_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/druid_progression
        implementation: dnd2024:class_progression_druid_progression
        parameters:
          class: druid
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Spellcasting, Druidic, Primal Order
            wild_shape: 0
            cantrips: 2
            prepared_spells: 4
            spell_slots_1: 2
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_1_spellcasting
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_1_druidic
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_1_primal_order
              at_class_level: 1
          - level: 2
            proficiency_bonus: 2
            features_text: Wild Shape, Wild Companion
            wild_shape: 2
            cantrips: 2
            prepared_spells: 5
            spell_slots_1: 3
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_2_wild_shape
              at_class_level: 2
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_2_wild_companion
              at_class_level: 2
          - level: 3
            proficiency_bonus: 2
            features_text: Druid Subclass
            wild_shape: 2
            cantrips: 2
            prepared_spells: 6
            spell_slots_1: 4
            spell_slots_2: 2
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/druid_level_3_druid_subclass
              at_class_level: 3
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement
            wild_shape: 2
            cantrips: 3
            prepared_spells: 7
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_4_ability_score_improvement
              at_class_level: 4
          - level: 5
            proficiency_bonus: 3
            features_text: Wild Resurgence
            wild_shape: 2
            cantrips: 3
            prepared_spells: 9
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 2
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_5_wild_resurgence
              at_class_level: 5
          - level: 6
            proficiency_bonus: 3
            features_text: Subclass feature
            wild_shape: 3
            cantrips: 3
            prepared_spells: 10
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 6
          - level: 7
            proficiency_bonus: 3
            features_text: Elemental Fury
            wild_shape: 3
            cantrips: 3
            prepared_spells: 11
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 1
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_7_elemental_fury
              at_class_level: 7
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            wild_shape: 3
            cantrips: 3
            prepared_spells: 12
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 2
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_4_ability_score_improvement
              at_class_level: 8
          - level: 9
            proficiency_bonus: 4
            features_text: —
            wild_shape: 3
            cantrips: 3
            prepared_spells: 14
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 1
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 10
            proficiency_bonus: 4
            features_text: Subclass feature
            wild_shape: 3
            cantrips: 4
            prepared_spells: 15
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 10
          - level: 11
            proficiency_bonus: 4
            features_text: —
            wild_shape: 3
            cantrips: 4
            prepared_spells: 16
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            wild_shape: 3
            cantrips: 4
            prepared_spells: 16
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_4_ability_score_improvement
              at_class_level: 12
          - level: 13
            proficiency_bonus: 5
            features_text: —
            wild_shape: 3
            cantrips: 4
            prepared_spells: 17
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 14
            proficiency_bonus: 5
            features_text: Subclass feature
            wild_shape: 3
            cantrips: 4
            prepared_spells: 17
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 14
          - level: 15
            proficiency_bonus: 5
            features_text: Improved Elemental Fury
            wild_shape: 3
            cantrips: 4
            prepared_spells: 18
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_15_improved_elemental_fury
              at_class_level: 15
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            wild_shape: 3
            cantrips: 4
            prepared_spells: 18
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_4_ability_score_improvement
              at_class_level: 16
          - level: 17
            proficiency_bonus: 6
            features_text: —
            wild_shape: 4
            cantrips: 4
            prepared_spells: 19
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants: []
          - level: 18
            proficiency_bonus: 6
            features_text: Beast Spells
            wild_shape: 4
            cantrips: 4
            prepared_spells: 20
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_18_beast_spells
              at_class_level: 18
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            wild_shape: 4
            cantrips: 4
            prepared_spells: 21
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 2
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_19_epic_boon
              at_class_level: 19
          - level: 20
            proficiency_bonus: 6
            features_text: Archdruid
            wild_shape: 4
            cantrips: 4
            prepared_spells: 22
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 2
            spell_slots_7: 2
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/druid_level_20_archdruid
              at_class_level: 20
          class_definition:
            key: dnd2024:classes/druid
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            cantrips:
              kind: selection_capacity
              selection: cantrips
              eligibility: owning_feature_contract
            prepared_spells:
              kind: selection_capacity
              selection: prepared_spells
              eligibility: owning_feature_contract
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
            spell_slots_1:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 1
              recovery: long_rest
            spell_slots_2:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 2
              recovery: long_rest
            spell_slots_3:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 3
              recovery: long_rest
            spell_slots_4:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 4
              recovery: long_rest
            spell_slots_5:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 5
              recovery: long_rest
            spell_slots_6:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 6
              recovery: long_rest
            spell_slots_7:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 7
              recovery: long_rest
            spell_slots_8:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 8
              recovery: long_rest
            spell_slots_9:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 9
              recovery: long_rest
            wild_shape:
              kind: resource_capacity
              resource: wild_shape
              recovery: owning_feature_contract
              level_change_refills: false
    contract:
      id: dnd2024:class_progression_druid_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_druid_progression
    citations:
    - SRD 5.2.1 pp. 41; builtin/srd-5.2.1/source.txt
  dnd2024:tables/fighter_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/fighter_progression
        implementation: dnd2024:class_progression_fighter_progression
        parameters:
          class: fighter
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Fighting Style, Second Wind, Weapon Mastery
            second_wind: 2
            weapon_mastery: 3
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_1_fighting_style
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_1_second_wind
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_1_weapon_mastery
              at_class_level: 1
          - level: 2
            proficiency_bonus: 2
            features_text: Action Surge (one use), Tactical Mind
            second_wind: 2
            weapon_mastery: 3
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_2_action_surge
              at_class_level: 2
              maximum_uses: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_2_tactical_mind
              at_class_level: 2
          - level: 3
            proficiency_bonus: 2
            features_text: Fighter Subclass
            second_wind: 2
            weapon_mastery: 3
            grants:
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/fighter_level_3_fighter_subclass
              at_class_level: 3
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement
            second_wind: 3
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_4_ability_score_improvement
              at_class_level: 4
          - level: 5
            proficiency_bonus: 3
            features_text: Extra Attack, Tactical Shift
            second_wind: 3
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_5_extra_attack
              at_class_level: 5
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_5_tactical_shift
              at_class_level: 5
          - level: 6
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            second_wind: 3
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_4_ability_score_improvement
              at_class_level: 6
          - level: 7
            proficiency_bonus: 3
            features_text: Subclass feature
            second_wind: 3
            weapon_mastery: 4
            grants:
            - operation: grant_selected_subclass_features
              class_level: 7
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            second_wind: 3
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_4_ability_score_improvement
              at_class_level: 8
          - level: 9
            proficiency_bonus: 4
            features_text: Indomitable (one use), Tactical Master
            second_wind: 3
            weapon_mastery: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_9_indomitable
              at_class_level: 9
              maximum_uses: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_9_tactical_master
              at_class_level: 9
          - level: 10
            proficiency_bonus: 4
            features_text: Subclass feature
            second_wind: 4
            weapon_mastery: 5
            grants:
            - operation: grant_selected_subclass_features
              class_level: 10
          - level: 11
            proficiency_bonus: 4
            features_text: Two Extra Attacks
            second_wind: 4
            weapon_mastery: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_11_two_extra_attacks
              at_class_level: 11
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            second_wind: 4
            weapon_mastery: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_4_ability_score_improvement
              at_class_level: 12
          - level: 13
            proficiency_bonus: 5
            features_text: Indomitable (two uses), Studied Attacks
            second_wind: 4
            weapon_mastery: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_9_indomitable
              at_class_level: 13
              maximum_uses: 2
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_13_studied_attacks
              at_class_level: 13
          - level: 14
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            second_wind: 4
            weapon_mastery: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_4_ability_score_improvement
              at_class_level: 14
          - level: 15
            proficiency_bonus: 5
            features_text: Subclass feature
            second_wind: 4
            weapon_mastery: 5
            grants:
            - operation: grant_selected_subclass_features
              class_level: 15
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            second_wind: 4
            weapon_mastery: 6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_4_ability_score_improvement
              at_class_level: 16
          - level: 17
            proficiency_bonus: 6
            features_text: Action Surge (two uses), Indomitable (three uses)
            second_wind: 4
            weapon_mastery: 6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_2_action_surge
              at_class_level: 17
              maximum_uses: 2
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_9_indomitable
              at_class_level: 17
              maximum_uses: 3
          - level: 18
            proficiency_bonus: 6
            features_text: Subclass feature
            second_wind: 4
            weapon_mastery: 6
            grants:
            - operation: grant_selected_subclass_features
              class_level: 18
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            second_wind: 4
            weapon_mastery: 6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_19_epic_boon
              at_class_level: 19
          - level: 20
            proficiency_bonus: 6
            features_text: Three Extra Attacks
            second_wind: 4
            weapon_mastery: 6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/fighter_level_20_three_extra_attacks
              at_class_level: 20
          class_definition:
            key: dnd2024:classes/fighter
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
            second_wind:
              kind: resource_capacity
              resource: second_wind
              recovery: owning_feature_contract
              level_change_refills: false
            weapon_mastery:
              kind: selection_capacity
              selection: weapon_mastery
              eligibility: owning_feature_contract
    contract:
      id: dnd2024:class_progression_fighter_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_fighter_progression
    citations:
    - SRD 5.2.1 pp. 47; builtin/srd-5.2.1/source.txt
  dnd2024:tables/monk_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/monk_progression
        implementation: dnd2024:class_progression_monk_progression
        parameters:
          class: monk
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Martial Arts, Unarmored Defense
            martial_arts: 1d6
            focus_points: 0
            unarmored_movement: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_1_martial_arts
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_1_unarmored_defense
              at_class_level: 1
            martial_arts_dice:
              dice:
                count: 1
                sides: 6
              bonus: 0
          - level: 2
            proficiency_bonus: 2
            features_text: Monk’s Focus, Unarmored Movement, Uncanny Metabolism
            martial_arts: 1d6
            focus_points: 2
            unarmored_movement: 10
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_2_monk_s_focus
              at_class_level: 2
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_2_unarmored_movement
              at_class_level: 2
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_2_uncanny_metabolism
              at_class_level: 2
            martial_arts_dice:
              dice:
                count: 1
                sides: 6
              bonus: 0
          - level: 3
            proficiency_bonus: 2
            features_text: Deflect Attacks, Monk Subclass
            martial_arts: 1d6
            focus_points: 3
            unarmored_movement: 10
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_3_deflect_attacks
              at_class_level: 3
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/monk_level_3_monk_subclass
              at_class_level: 3
            martial_arts_dice:
              dice:
                count: 1
                sides: 6
              bonus: 0
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement, Slow Fall
            martial_arts: 1d6
            focus_points: 4
            unarmored_movement: 10
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_4_ability_score_improvement
              at_class_level: 4
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_4_slow_fall
              at_class_level: 4
            martial_arts_dice:
              dice:
                count: 1
                sides: 6
              bonus: 0
          - level: 5
            proficiency_bonus: 3
            features_text: Extra Attack, Stunning Strike
            martial_arts: 1d8
            focus_points: 5
            unarmored_movement: 10
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_5_extra_attack
              at_class_level: 5
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_5_stunning_strike
              at_class_level: 5
            martial_arts_dice:
              dice:
                count: 1
                sides: 8
              bonus: 0
          - level: 6
            proficiency_bonus: 3
            features_text: Empowered Strikes, Subclass feature
            martial_arts: 1d8
            focus_points: 6
            unarmored_movement: 15
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_6_empowered_strikes
              at_class_level: 6
            - operation: grant_selected_subclass_features
              class_level: 6
            martial_arts_dice:
              dice:
                count: 1
                sides: 8
              bonus: 0
          - level: 7
            proficiency_bonus: 3
            features_text: Evasion
            martial_arts: 1d8
            focus_points: 7
            unarmored_movement: 15
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_7_evasion
              at_class_level: 7
            martial_arts_dice:
              dice:
                count: 1
                sides: 8
              bonus: 0
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            martial_arts: 1d8
            focus_points: 8
            unarmored_movement: 15
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_4_ability_score_improvement
              at_class_level: 8
            martial_arts_dice:
              dice:
                count: 1
                sides: 8
              bonus: 0
          - level: 9
            proficiency_bonus: 4
            features_text: Acrobatic Movement
            martial_arts: 1d8
            focus_points: 9
            unarmored_movement: 15
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_9_acrobatic_movement
              at_class_level: 9
            martial_arts_dice:
              dice:
                count: 1
                sides: 8
              bonus: 0
          - level: 10
            proficiency_bonus: 4
            features_text: Heightened Focus, Self-Restoration
            martial_arts: 1d8
            focus_points: 10
            unarmored_movement: 20
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_10_heightened_focus
              at_class_level: 10
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_10_self_restoration
              at_class_level: 10
            martial_arts_dice:
              dice:
                count: 1
                sides: 8
              bonus: 0
          - level: 11
            proficiency_bonus: 4
            features_text: Subclass feature
            martial_arts: 1d10
            focus_points: 11
            unarmored_movement: 20
            grants:
            - operation: grant_selected_subclass_features
              class_level: 11
            martial_arts_dice:
              dice:
                count: 1
                sides: 10
              bonus: 0
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            martial_arts: 1d10
            focus_points: 12
            unarmored_movement: 20
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_4_ability_score_improvement
              at_class_level: 12
            martial_arts_dice:
              dice:
                count: 1
                sides: 10
              bonus: 0
          - level: 13
            proficiency_bonus: 5
            features_text: Deflect Energy
            martial_arts: 1d10
            focus_points: 13
            unarmored_movement: 20
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_13_deflect_energy
              at_class_level: 13
            martial_arts_dice:
              dice:
                count: 1
                sides: 10
              bonus: 0
          - level: 14
            proficiency_bonus: 5
            features_text: Disciplined Survivor
            martial_arts: 1d10
            focus_points: 14
            unarmored_movement: 25
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_14_disciplined_survivor
              at_class_level: 14
            martial_arts_dice:
              dice:
                count: 1
                sides: 10
              bonus: 0
          - level: 15
            proficiency_bonus: 5
            features_text: Perfect Focus
            martial_arts: 1d10
            focus_points: 15
            unarmored_movement: 25
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_15_perfect_focus
              at_class_level: 15
            martial_arts_dice:
              dice:
                count: 1
                sides: 10
              bonus: 0
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            martial_arts: 1d10
            focus_points: 16
            unarmored_movement: 25
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_4_ability_score_improvement
              at_class_level: 16
            martial_arts_dice:
              dice:
                count: 1
                sides: 10
              bonus: 0
          - level: 17
            proficiency_bonus: 6
            features_text: Subclass feature
            martial_arts: 1d12
            focus_points: 17
            unarmored_movement: 25
            grants:
            - operation: grant_selected_subclass_features
              class_level: 17
            martial_arts_dice:
              dice:
                count: 1
                sides: 12
              bonus: 0
          - level: 18
            proficiency_bonus: 6
            features_text: Superior Defense
            martial_arts: 1d12
            focus_points: 18
            unarmored_movement: 30
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_18_superior_defense
              at_class_level: 18
            martial_arts_dice:
              dice:
                count: 1
                sides: 12
              bonus: 0
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            martial_arts: 1d12
            focus_points: 19
            unarmored_movement: 30
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_19_epic_boon
              at_class_level: 19
            martial_arts_dice:
              dice:
                count: 1
                sides: 12
              bonus: 0
          - level: 20
            proficiency_bonus: 6
            features_text: Body and Mind
            martial_arts: 1d12
            focus_points: 20
            unarmored_movement: 30
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/monk_level_20_body_and_mind
              at_class_level: 20
            martial_arts_dice:
              dice:
                count: 1
                sides: 12
              bonus: 0
          class_definition:
            key: dnd2024:classes/monk
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            focus_points:
              kind: resource_capacity
              resource: focus_points
              recovery: owning_feature_contract
              level_change_refills: false
            martial_arts:
              kind: dice_expression
              applies_to: martial_arts
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
            unarmored_movement:
              kind: speed_bonus
              unit: ft
              requires: unarmored_movement_feature_eligibility
    contract:
      id: dnd2024:class_progression_monk_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_monk_progression
    citations:
    - SRD 5.2.1 pp. 50; builtin/srd-5.2.1/source.txt
  dnd2024:tables/paladin_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/paladin_progression
        implementation: dnd2024:class_progression_paladin_progression
        parameters:
          class: paladin
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Lay On Hands, Spellcasting, Weapon Mastery
            channel_divinity: 0
            prepared_spells: 2
            spell_slots_1: 2
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_1_lay_on_hands
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_1_spellcasting
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_1_weapon_mastery
              at_class_level: 1
          - level: 2
            proficiency_bonus: 2
            features_text: Fighting Style, Paladin’s Smite
            channel_divinity: 0
            prepared_spells: 3
            spell_slots_1: 2
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_2_fighting_style
              at_class_level: 2
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_2_paladin_s_smite
              at_class_level: 2
          - level: 3
            proficiency_bonus: 2
            features_text: Channel Divinity, Paladin Subclass
            channel_divinity: 2
            prepared_spells: 4
            spell_slots_1: 3
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_3_channel_divinity
              at_class_level: 3
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/paladin_level_3_paladin_subclass
              at_class_level: 3
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement
            channel_divinity: 2
            prepared_spells: 5
            spell_slots_1: 3
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_4_ability_score_improvement
              at_class_level: 4
          - level: 5
            proficiency_bonus: 3
            features_text: Extra Attack, Faithful Steed
            channel_divinity: 2
            prepared_spells: 6
            spell_slots_1: 4
            spell_slots_2: 2
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_5_extra_attack
              at_class_level: 5
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_5_faithful_steed
              at_class_level: 5
          - level: 6
            proficiency_bonus: 3
            features_text: Aura of Protection
            channel_divinity: 2
            prepared_spells: 6
            spell_slots_1: 4
            spell_slots_2: 2
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_6_aura_of_protection
              at_class_level: 6
          - level: 7
            proficiency_bonus: 3
            features_text: Subclass feature
            channel_divinity: 2
            prepared_spells: 7
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 7
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            channel_divinity: 2
            prepared_spells: 7
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_4_ability_score_improvement
              at_class_level: 8
          - level: 9
            proficiency_bonus: 4
            features_text: Abjure Foes
            channel_divinity: 2
            prepared_spells: 9
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 2
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_9_abjure_foes
              at_class_level: 9
          - level: 10
            proficiency_bonus: 4
            features_text: Aura of Courage
            channel_divinity: 2
            prepared_spells: 9
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 2
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_10_aura_of_courage
              at_class_level: 10
          - level: 11
            proficiency_bonus: 4
            features_text: Radiant Strikes
            channel_divinity: 3
            prepared_spells: 10
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_11_radiant_strikes
              at_class_level: 11
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            channel_divinity: 3
            prepared_spells: 10
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_4_ability_score_improvement
              at_class_level: 12
          - level: 13
            proficiency_bonus: 5
            features_text: —
            channel_divinity: 3
            prepared_spells: 11
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 1
            spell_slots_5: 0
            grants: []
          - level: 14
            proficiency_bonus: 5
            features_text: Restoring Touch
            channel_divinity: 3
            prepared_spells: 11
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 1
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_14_restoring_touch
              at_class_level: 14
          - level: 15
            proficiency_bonus: 5
            features_text: Subclass feature
            channel_divinity: 3
            prepared_spells: 12
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 2
            spell_slots_5: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 15
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            channel_divinity: 3
            prepared_spells: 12
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 2
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_4_ability_score_improvement
              at_class_level: 16
          - level: 17
            proficiency_bonus: 6
            features_text: —
            channel_divinity: 3
            prepared_spells: 14
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 1
            grants: []
          - level: 18
            proficiency_bonus: 6
            features_text: Aura Expansion
            channel_divinity: 3
            prepared_spells: 14
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_18_aura_expansion
              at_class_level: 18
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            channel_divinity: 3
            prepared_spells: 15
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/paladin_level_19_epic_boon
              at_class_level: 19
          - level: 20
            proficiency_bonus: 6
            features_text: Subclass feature
            channel_divinity: 3
            prepared_spells: 15
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            grants:
            - operation: grant_selected_subclass_features
              class_level: 20
          class_definition:
            key: dnd2024:classes/paladin
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            channel_divinity:
              kind: resource_capacity
              resource: channel_divinity
              recovery: owning_feature_contract
              level_change_refills: false
            prepared_spells:
              kind: selection_capacity
              selection: prepared_spells
              eligibility: owning_feature_contract
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
            spell_slots_1:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 1
              recovery: long_rest
            spell_slots_2:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 2
              recovery: long_rest
            spell_slots_3:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 3
              recovery: long_rest
            spell_slots_4:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 4
              recovery: long_rest
            spell_slots_5:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 5
              recovery: long_rest
    contract:
      id: dnd2024:class_progression_paladin_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_paladin_progression
    citations:
    - SRD 5.2.1 pp. 53; builtin/srd-5.2.1/source.txt
  dnd2024:tables/ranger_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/ranger_progression
        implementation: dnd2024:class_progression_ranger_progression
        parameters:
          class: ranger
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Spellcasting, Favored Enemy, Weapon Mastery
            favored_enemy: 2
            prepared_spells: 2
            spell_slots_1: 2
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_1_spellcasting
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_1_favored_enemy
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_1_weapon_mastery
              at_class_level: 1
          - level: 2
            proficiency_bonus: 2
            features_text: Deft Explorer, Fighting Style
            favored_enemy: 2
            prepared_spells: 3
            spell_slots_1: 2
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_2_deft_explorer
              at_class_level: 2
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_2_fighting_style
              at_class_level: 2
          - level: 3
            proficiency_bonus: 2
            features_text: Ranger Subclass
            favored_enemy: 2
            prepared_spells: 4
            spell_slots_1: 3
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/ranger_level_3_ranger_subclass
              at_class_level: 3
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement
            favored_enemy: 2
            prepared_spells: 5
            spell_slots_1: 3
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_4_ability_score_improvement
              at_class_level: 4
          - level: 5
            proficiency_bonus: 3
            features_text: Extra Attack
            favored_enemy: 3
            prepared_spells: 6
            spell_slots_1: 4
            spell_slots_2: 2
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_5_extra_attack
              at_class_level: 5
          - level: 6
            proficiency_bonus: 3
            features_text: Roving
            favored_enemy: 3
            prepared_spells: 6
            spell_slots_1: 4
            spell_slots_2: 2
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_6_roving
              at_class_level: 6
          - level: 7
            proficiency_bonus: 3
            features_text: Subclass feature
            favored_enemy: 3
            prepared_spells: 7
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 7
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            favored_enemy: 3
            prepared_spells: 7
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_4_ability_score_improvement
              at_class_level: 8
          - level: 9
            proficiency_bonus: 4
            features_text: Expertise
            favored_enemy: 4
            prepared_spells: 9
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 2
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_9_expertise
              at_class_level: 9
          - level: 10
            proficiency_bonus: 4
            features_text: Tireless
            favored_enemy: 4
            prepared_spells: 9
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 2
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_10_tireless
              at_class_level: 10
          - level: 11
            proficiency_bonus: 4
            features_text: Subclass feature
            favored_enemy: 4
            prepared_spells: 10
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 11
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            favored_enemy: 4
            prepared_spells: 10
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 0
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_4_ability_score_improvement
              at_class_level: 12
          - level: 13
            proficiency_bonus: 5
            features_text: Relentless Hunter
            favored_enemy: 5
            prepared_spells: 11
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 1
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_13_relentless_hunter
              at_class_level: 13
          - level: 14
            proficiency_bonus: 5
            features_text: Nature’s Veil
            favored_enemy: 5
            prepared_spells: 11
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 1
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_14_nature_s_veil
              at_class_level: 14
          - level: 15
            proficiency_bonus: 5
            features_text: Subclass feature
            favored_enemy: 5
            prepared_spells: 12
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 2
            spell_slots_5: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 15
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            favored_enemy: 5
            prepared_spells: 12
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 2
            spell_slots_5: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_4_ability_score_improvement
              at_class_level: 16
          - level: 17
            proficiency_bonus: 6
            features_text: Precise Hunter
            favored_enemy: 6
            prepared_spells: 14
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_17_precise_hunter
              at_class_level: 17
          - level: 18
            proficiency_bonus: 6
            features_text: Feral Senses
            favored_enemy: 6
            prepared_spells: 14
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_18_feral_senses
              at_class_level: 18
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            favored_enemy: 6
            prepared_spells: 15
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_19_epic_boon
              at_class_level: 19
          - level: 20
            proficiency_bonus: 6
            features_text: Foe Slayer
            favored_enemy: 6
            prepared_spells: 15
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/ranger_level_20_foe_slayer
              at_class_level: 20
          class_definition:
            key: dnd2024:classes/ranger
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            favored_enemy:
              kind: resource_capacity
              resource: favored_enemy
              recovery: owning_feature_contract
              level_change_refills: false
            prepared_spells:
              kind: selection_capacity
              selection: prepared_spells
              eligibility: owning_feature_contract
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
            spell_slots_1:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 1
              recovery: long_rest
            spell_slots_2:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 2
              recovery: long_rest
            spell_slots_3:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 3
              recovery: long_rest
            spell_slots_4:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 4
              recovery: long_rest
            spell_slots_5:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 5
              recovery: long_rest
    contract:
      id: dnd2024:class_progression_ranger_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_ranger_progression
    citations:
    - SRD 5.2.1 pp. 58; builtin/srd-5.2.1/source.txt
  dnd2024:tables/rogue_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/rogue_progression
        implementation: dnd2024:class_progression_rogue_progression
        parameters:
          class: rogue
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Expertise, Sneak Attack, Thieves’ Cant, Weapon Mastery
            sneak_attack: 1d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_1_expertise
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_1_sneak_attack
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_1_thieves_cant
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_1_weapon_mastery
              at_class_level: 1
            sneak_attack_dice:
              dice:
                count: 1
                sides: 6
              bonus: 0
          - level: 2
            proficiency_bonus: 2
            features_text: Cunning Action
            sneak_attack: 1d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_2_cunning_action
              at_class_level: 2
            sneak_attack_dice:
              dice:
                count: 1
                sides: 6
              bonus: 0
          - level: 3
            proficiency_bonus: 2
            features_text: Rogue Subclass, Steady Aim
            sneak_attack: 2d6
            grants:
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/rogue_level_3_rogue_subclass
              at_class_level: 3
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_3_steady_aim
              at_class_level: 3
            sneak_attack_dice:
              dice:
                count: 2
                sides: 6
              bonus: 0
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement
            sneak_attack: 2d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_4_ability_score_improvement
              at_class_level: 4
            sneak_attack_dice:
              dice:
                count: 2
                sides: 6
              bonus: 0
          - level: 5
            proficiency_bonus: 3
            features_text: Cunning Strike, Uncanny Dodge
            sneak_attack: 3d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_5_cunning_strike
              at_class_level: 5
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_5_uncanny_dodge
              at_class_level: 5
            sneak_attack_dice:
              dice:
                count: 3
                sides: 6
              bonus: 0
          - level: 6
            proficiency_bonus: 3
            features_text: Expertise
            sneak_attack: 3d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_1_expertise
              at_class_level: 6
            sneak_attack_dice:
              dice:
                count: 3
                sides: 6
              bonus: 0
          - level: 7
            proficiency_bonus: 3
            features_text: Evasion, Reliable Talent
            sneak_attack: 4d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_7_evasion
              at_class_level: 7
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_7_reliable_talent
              at_class_level: 7
            sneak_attack_dice:
              dice:
                count: 4
                sides: 6
              bonus: 0
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            sneak_attack: 4d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_4_ability_score_improvement
              at_class_level: 8
            sneak_attack_dice:
              dice:
                count: 4
                sides: 6
              bonus: 0
          - level: 9
            proficiency_bonus: 4
            features_text: Subclass feature
            sneak_attack: 5d6
            grants:
            - operation: grant_selected_subclass_features
              class_level: 9
            sneak_attack_dice:
              dice:
                count: 5
                sides: 6
              bonus: 0
          - level: 10
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            sneak_attack: 5d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_4_ability_score_improvement
              at_class_level: 10
            sneak_attack_dice:
              dice:
                count: 5
                sides: 6
              bonus: 0
          - level: 11
            proficiency_bonus: 4
            features_text: Improved Cunning Strike
            sneak_attack: 6d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_11_improved_cunning_strike
              at_class_level: 11
            sneak_attack_dice:
              dice:
                count: 6
                sides: 6
              bonus: 0
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            sneak_attack: 6d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_4_ability_score_improvement
              at_class_level: 12
            sneak_attack_dice:
              dice:
                count: 6
                sides: 6
              bonus: 0
          - level: 13
            proficiency_bonus: 5
            features_text: Subclass feature
            sneak_attack: 7d6
            grants:
            - operation: grant_selected_subclass_features
              class_level: 13
            sneak_attack_dice:
              dice:
                count: 7
                sides: 6
              bonus: 0
          - level: 14
            proficiency_bonus: 5
            features_text: Devious Strikes
            sneak_attack: 7d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_14_devious_strikes
              at_class_level: 14
            sneak_attack_dice:
              dice:
                count: 7
                sides: 6
              bonus: 0
          - level: 15
            proficiency_bonus: 5
            features_text: Slippery Mind
            sneak_attack: 8d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_15_slippery_mind
              at_class_level: 15
            sneak_attack_dice:
              dice:
                count: 8
                sides: 6
              bonus: 0
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            sneak_attack: 8d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_4_ability_score_improvement
              at_class_level: 16
            sneak_attack_dice:
              dice:
                count: 8
                sides: 6
              bonus: 0
          - level: 17
            proficiency_bonus: 6
            features_text: Subclass feature
            sneak_attack: 9d6
            grants:
            - operation: grant_selected_subclass_features
              class_level: 17
            sneak_attack_dice:
              dice:
                count: 9
                sides: 6
              bonus: 0
          - level: 18
            proficiency_bonus: 6
            features_text: Elusive
            sneak_attack: 9d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_18_elusive
              at_class_level: 18
            sneak_attack_dice:
              dice:
                count: 9
                sides: 6
              bonus: 0
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            sneak_attack: 10d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_19_epic_boon
              at_class_level: 19
            sneak_attack_dice:
              dice:
                count: 10
                sides: 6
              bonus: 0
          - level: 20
            proficiency_bonus: 6
            features_text: Stroke of Luck
            sneak_attack: 10d6
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/rogue_level_20_stroke_of_luck
              at_class_level: 20
            sneak_attack_dice:
              dice:
                count: 10
                sides: 6
              bonus: 0
          class_definition:
            key: dnd2024:classes/rogue
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
            sneak_attack:
              kind: dice_expression
              applies_to: sneak_attack
    contract:
      id: dnd2024:class_progression_rogue_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_rogue_progression
    citations:
    - SRD 5.2.1 pp. 62; builtin/srd-5.2.1/source.txt
  dnd2024:tables/sorcerer_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/sorcerer_progression
        implementation: dnd2024:class_progression_sorcerer_progression
        parameters:
          class: sorcerer
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Spellcasting, Innate Sorcery
            sorcery_points: 0
            cantrips: 4
            prepared_spells: 2
            spell_slots_1: 2
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_1_spellcasting
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
              at_class_level: 1
          - level: 2
            proficiency_bonus: 2
            features_text: Font of Magic, Metamagic
            sorcery_points: 2
            cantrips: 4
            prepared_spells: 4
            spell_slots_1: 3
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_2_font_of_magic
              at_class_level: 2
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_2_metamagic
              at_class_level: 2
          - level: 3
            proficiency_bonus: 2
            features_text: Sorcerer Subclass
            sorcery_points: 3
            cantrips: 4
            prepared_spells: 6
            spell_slots_1: 4
            spell_slots_2: 2
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/sorcerer_level_3_sorcerer_subclass
              at_class_level: 3
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement
            sorcery_points: 4
            cantrips: 5
            prepared_spells: 7
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
              at_class_level: 4
          - level: 5
            proficiency_bonus: 3
            features_text: Sorcerous Restoration
            sorcery_points: 5
            cantrips: 5
            prepared_spells: 9
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 2
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_5_sorcerous_restoration
              at_class_level: 5
          - level: 6
            proficiency_bonus: 3
            features_text: Subclass feature
            sorcery_points: 6
            cantrips: 5
            prepared_spells: 10
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 6
          - level: 7
            proficiency_bonus: 3
            features_text: Sorcery Incarnate
            sorcery_points: 7
            cantrips: 5
            prepared_spells: 11
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 1
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_7_sorcery_incarnate
              at_class_level: 7
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            sorcery_points: 8
            cantrips: 5
            prepared_spells: 12
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 2
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
              at_class_level: 8
          - level: 9
            proficiency_bonus: 4
            features_text: —
            sorcery_points: 9
            cantrips: 5
            prepared_spells: 14
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 1
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 10
            proficiency_bonus: 4
            features_text: Metamagic
            sorcery_points: 10
            cantrips: 6
            prepared_spells: 15
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_2_metamagic
              at_class_level: 10
          - level: 11
            proficiency_bonus: 4
            features_text: —
            sorcery_points: 11
            cantrips: 6
            prepared_spells: 16
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            sorcery_points: 12
            cantrips: 6
            prepared_spells: 16
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
              at_class_level: 12
          - level: 13
            proficiency_bonus: 5
            features_text: —
            sorcery_points: 13
            cantrips: 6
            prepared_spells: 17
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 14
            proficiency_bonus: 5
            features_text: Subclass feature
            sorcery_points: 14
            cantrips: 6
            prepared_spells: 17
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 14
          - level: 15
            proficiency_bonus: 5
            features_text: —
            sorcery_points: 15
            cantrips: 6
            prepared_spells: 18
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 0
            grants: []
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            sorcery_points: 16
            cantrips: 6
            prepared_spells: 18
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
              at_class_level: 16
          - level: 17
            proficiency_bonus: 6
            features_text: Metamagic
            sorcery_points: 17
            cantrips: 6
            prepared_spells: 19
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_2_metamagic
              at_class_level: 17
          - level: 18
            proficiency_bonus: 6
            features_text: Subclass feature
            sorcery_points: 18
            cantrips: 6
            prepared_spells: 20
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_selected_subclass_features
              class_level: 18
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            sorcery_points: 19
            cantrips: 6
            prepared_spells: 21
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 2
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_19_epic_boon
              at_class_level: 19
          - level: 20
            proficiency_bonus: 6
            features_text: Arcane Apotheosis
            sorcery_points: 20
            cantrips: 6
            prepared_spells: 22
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 2
            spell_slots_7: 2
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/sorcerer_level_20_arcane_apotheosis
              at_class_level: 20
          class_definition:
            key: dnd2024:classes/sorcerer
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            cantrips:
              kind: selection_capacity
              selection: cantrips
              eligibility: owning_feature_contract
            prepared_spells:
              kind: selection_capacity
              selection: prepared_spells
              eligibility: owning_feature_contract
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
            sorcery_points:
              kind: resource_capacity
              resource: sorcery_points
              recovery: owning_feature_contract
              level_change_refills: false
            spell_slots_1:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 1
              recovery: long_rest
            spell_slots_2:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 2
              recovery: long_rest
            spell_slots_3:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 3
              recovery: long_rest
            spell_slots_4:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 4
              recovery: long_rest
            spell_slots_5:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 5
              recovery: long_rest
            spell_slots_6:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 6
              recovery: long_rest
            spell_slots_7:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 7
              recovery: long_rest
            spell_slots_8:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 8
              recovery: long_rest
            spell_slots_9:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 9
              recovery: long_rest
    contract:
      id: dnd2024:class_progression_sorcerer_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_sorcerer_progression
    citations:
    - SRD 5.2.1 pp. 65; builtin/srd-5.2.1/source.txt
  dnd2024:tables/warlock_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/warlock_progression
        implementation: dnd2024:class_progression_warlock_progression
        parameters:
          class: warlock
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Eldritch Invocations, Pact Magic
            eldritch_invocations: 1
            cantrips: 2
            prepared_spells: 2
            pact_slots: 1
            pact_slot_level: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_1_eldritch_invocations
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_1_pact_magic
              at_class_level: 1
          - level: 2
            proficiency_bonus: 2
            features_text: Magical Cunning
            eldritch_invocations: 3
            cantrips: 2
            prepared_spells: 3
            pact_slots: 2
            pact_slot_level: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_2_magical_cunning
              at_class_level: 2
          - level: 3
            proficiency_bonus: 2
            features_text: Warlock Subclass
            eldritch_invocations: 3
            cantrips: 2
            prepared_spells: 4
            pact_slots: 2
            pact_slot_level: 2
            grants:
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/warlock_level_3_warlock_subclass
              at_class_level: 3
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement
            eldritch_invocations: 3
            cantrips: 3
            prepared_spells: 5
            pact_slots: 2
            pact_slot_level: 2
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_4_ability_score_improvement
              at_class_level: 4
          - level: 5
            proficiency_bonus: 3
            features_text: —
            eldritch_invocations: 5
            cantrips: 3
            prepared_spells: 6
            pact_slots: 2
            pact_slot_level: 3
            grants: []
          - level: 6
            proficiency_bonus: 3
            features_text: Subclass feature
            eldritch_invocations: 5
            cantrips: 3
            prepared_spells: 7
            pact_slots: 2
            pact_slot_level: 3
            grants:
            - operation: grant_selected_subclass_features
              class_level: 6
          - level: 7
            proficiency_bonus: 3
            features_text: —
            eldritch_invocations: 6
            cantrips: 3
            prepared_spells: 8
            pact_slots: 2
            pact_slot_level: 4
            grants: []
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            eldritch_invocations: 6
            cantrips: 3
            prepared_spells: 9
            pact_slots: 2
            pact_slot_level: 4
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_4_ability_score_improvement
              at_class_level: 8
          - level: 9
            proficiency_bonus: 4
            features_text: Contact Patron
            eldritch_invocations: 7
            cantrips: 3
            prepared_spells: 10
            pact_slots: 2
            pact_slot_level: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_9_contact_patron
              at_class_level: 9
          - level: 10
            proficiency_bonus: 4
            features_text: Subclass feature
            eldritch_invocations: 7
            cantrips: 4
            prepared_spells: 10
            pact_slots: 2
            pact_slot_level: 5
            grants:
            - operation: grant_selected_subclass_features
              class_level: 10
          - level: 11
            proficiency_bonus: 4
            features_text: Mystic Arcanum (level 6 spell)
            eldritch_invocations: 7
            cantrips: 4
            prepared_spells: 11
            pact_slots: 3
            pact_slot_level: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_11_mystic_arcanum
              at_class_level: 11
              arcanum_spell_level: 6
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            eldritch_invocations: 8
            cantrips: 4
            prepared_spells: 11
            pact_slots: 3
            pact_slot_level: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_4_ability_score_improvement
              at_class_level: 12
          - level: 13
            proficiency_bonus: 5
            features_text: Mystic Arcanum (level 7 spell)
            eldritch_invocations: 8
            cantrips: 4
            prepared_spells: 12
            pact_slots: 3
            pact_slot_level: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_11_mystic_arcanum
              at_class_level: 13
              arcanum_spell_level: 7
          - level: 14
            proficiency_bonus: 5
            features_text: Subclass feature
            eldritch_invocations: 8
            cantrips: 4
            prepared_spells: 12
            pact_slots: 3
            pact_slot_level: 5
            grants:
            - operation: grant_selected_subclass_features
              class_level: 14
          - level: 15
            proficiency_bonus: 5
            features_text: Mystic Arcanum (level 8 spell)
            eldritch_invocations: 9
            cantrips: 4
            prepared_spells: 13
            pact_slots: 3
            pact_slot_level: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_11_mystic_arcanum
              at_class_level: 15
              arcanum_spell_level: 8
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            eldritch_invocations: 9
            cantrips: 4
            prepared_spells: 13
            pact_slots: 3
            pact_slot_level: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_4_ability_score_improvement
              at_class_level: 16
          - level: 17
            proficiency_bonus: 6
            features_text: Mystic Arcanum (level 9 spell)
            eldritch_invocations: 9
            cantrips: 4
            prepared_spells: 14
            pact_slots: 4
            pact_slot_level: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_11_mystic_arcanum
              at_class_level: 17
              arcanum_spell_level: 9
          - level: 18
            proficiency_bonus: 6
            features_text: —
            eldritch_invocations: 10
            cantrips: 4
            prepared_spells: 14
            pact_slots: 4
            pact_slot_level: 5
            grants: []
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            eldritch_invocations: 10
            cantrips: 4
            prepared_spells: 15
            pact_slots: 4
            pact_slot_level: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_19_epic_boon
              at_class_level: 19
          - level: 20
            proficiency_bonus: 6
            features_text: Eldritch Master
            eldritch_invocations: 10
            cantrips: 4
            prepared_spells: 15
            pact_slots: 4
            pact_slot_level: 5
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/warlock_level_20_eldritch_master
              at_class_level: 20
          class_definition:
            key: dnd2024:classes/warlock
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            cantrips:
              kind: selection_capacity
              selection: cantrips
              eligibility: owning_feature_contract
            eldritch_invocations:
              kind: selection_capacity
              selection: eldritch_invocations
              eligibility: owning_feature_contract
            pact_slot_level:
              kind: spell_level
              applies_to: pact_slots
            pact_slots:
              kind: resource_capacity
              resource: pact_slots
              recovery: owning_feature_contract
              level_change_refills: false
            prepared_spells:
              kind: selection_capacity
              selection: prepared_spells
              eligibility: owning_feature_contract
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
    contract:
      id: dnd2024:class_progression_warlock_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_warlock_progression
    citations:
    - SRD 5.2.1 pp. 71; builtin/srd-5.2.1/source.txt
  dnd2024:tables/wizard_progression:
    source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:tables/wizard_progression
        implementation: dnd2024:class_progression_wizard_progression
        parameters:
          class: wizard
          rows:
          - level: 1
            proficiency_bonus: 2
            features_text: Spellcasting, Ritual Adept, Arcane Recovery
            cantrips: 3
            prepared_spells: 4
            spell_slots_1: 2
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_1_spellcasting
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_1_ritual_adept
              at_class_level: 1
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_1_arcane_recovery
              at_class_level: 1
          - level: 2
            proficiency_bonus: 2
            features_text: Scholar
            cantrips: 3
            prepared_spells: 5
            spell_slots_1: 3
            spell_slots_2: 0
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_2_scholar
              at_class_level: 2
          - level: 3
            proficiency_bonus: 2
            features_text: Wizard Subclass
            cantrips: 3
            prepared_spells: 6
            spell_slots_1: 4
            spell_slots_2: 2
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: choose_subclass
              feature:
                key: dnd2024:class_features/wizard_level_3_wizard_subclass
              at_class_level: 3
          - level: 4
            proficiency_bonus: 2
            features_text: Ability Score Improvement
            cantrips: 4
            prepared_spells: 7
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 0
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_4_ability_score_improvement
              at_class_level: 4
          - level: 5
            proficiency_bonus: 3
            features_text: Memorize Spell
            cantrips: 4
            prepared_spells: 9
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 2
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_5_memorize_spell
              at_class_level: 5
          - level: 6
            proficiency_bonus: 3
            features_text: Subclass feature
            cantrips: 4
            prepared_spells: 10
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 0
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 6
          - level: 7
            proficiency_bonus: 3
            features_text: —
            cantrips: 4
            prepared_spells: 11
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 1
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 8
            proficiency_bonus: 3
            features_text: Ability Score Improvement
            cantrips: 4
            prepared_spells: 12
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 2
            spell_slots_5: 0
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_4_ability_score_improvement
              at_class_level: 8
          - level: 9
            proficiency_bonus: 4
            features_text: —
            cantrips: 4
            prepared_spells: 14
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 1
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 10
            proficiency_bonus: 4
            features_text: Subclass feature
            cantrips: 5
            prepared_spells: 15
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 0
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 10
          - level: 11
            proficiency_bonus: 4
            features_text: —
            cantrips: 5
            prepared_spells: 16
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 12
            proficiency_bonus: 4
            features_text: Ability Score Improvement
            cantrips: 5
            prepared_spells: 16
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 0
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_4_ability_score_improvement
              at_class_level: 12
          - level: 13
            proficiency_bonus: 5
            features_text: —
            cantrips: 5
            prepared_spells: 17
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 0
            spell_slots_9: 0
            grants: []
          - level: 14
            proficiency_bonus: 5
            features_text: Subclass feature
            cantrips: 5
            prepared_spells: 18
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 0
            spell_slots_9: 0
            grants:
            - operation: grant_selected_subclass_features
              class_level: 14
          - level: 15
            proficiency_bonus: 5
            features_text: —
            cantrips: 5
            prepared_spells: 19
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 0
            grants: []
          - level: 16
            proficiency_bonus: 5
            features_text: Ability Score Improvement
            cantrips: 5
            prepared_spells: 21
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 0
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_4_ability_score_improvement
              at_class_level: 16
          - level: 17
            proficiency_bonus: 6
            features_text: —
            cantrips: 5
            prepared_spells: 22
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 2
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants: []
          - level: 18
            proficiency_bonus: 6
            features_text: Spell Mastery
            cantrips: 5
            prepared_spells: 23
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 1
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_18_spell_mastery
              at_class_level: 18
          - level: 19
            proficiency_bonus: 6
            features_text: Epic Boon
            cantrips: 5
            prepared_spells: 24
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 2
            spell_slots_7: 1
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_19_epic_boon
              at_class_level: 19
          - level: 20
            proficiency_bonus: 6
            features_text: Signature Spells
            cantrips: 5
            prepared_spells: 25
            spell_slots_1: 4
            spell_slots_2: 3
            spell_slots_3: 3
            spell_slots_4: 3
            spell_slots_5: 3
            spell_slots_6: 2
            spell_slots_7: 2
            spell_slots_8: 1
            spell_slots_9: 1
            grants:
            - operation: grant_feature
              feature:
                key: dnd2024:class_features/wizard_level_20_signature_spells
              at_class_level: 20
          class_definition:
            key: dnd2024:classes/wizard
          lookup:
            input: class_level
            minimum: 1
            maximum: 20
            match: exact
            numeric_columns: current_capacity_or_value
            feature_grants: all_crossed_levels
            repeat_application: idempotent_per_class_level_and_feature
          multiclass:
            proficiency_bonus_source: total_character_level
            spell_slot_override_rule:
              key: dnd2024:rules/multiclassing_spellcasting
            pact_magic_pool_separate: true
          column_semantics:
            cantrips:
              kind: selection_capacity
              selection: cantrips
              eligibility: owning_feature_contract
            prepared_spells:
              kind: selection_capacity
              selection: prepared_spells
              eligibility: owning_feature_contract
            proficiency_bonus:
              kind: bonus
              applies_to: proficiency
              use_class_row_only_if_single_class: true
            spell_slots_1:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 1
              recovery: long_rest
            spell_slots_2:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 2
              recovery: long_rest
            spell_slots_3:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 3
              recovery: long_rest
            spell_slots_4:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 4
              recovery: long_rest
            spell_slots_5:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 5
              recovery: long_rest
            spell_slots_6:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 6
              recovery: long_rest
            spell_slots_7:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 7
              recovery: long_rest
            spell_slots_8:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 8
              recovery: long_rest
            spell_slots_9:
              kind: resource_capacity
              resource: spell_slot
              spell_level: 9
              recovery: long_rest
    contract:
      id: dnd2024:class_progression_wizard_progression
      inputs:
        actor: participant
        class_level: integer
        previous_class_level:
          type: integer
          required: false
        selected_subclass:
          type: definition
          required: false
        choices:
          type: definition_list
          required: false
      outputs:
      - feature_grants
      - selection_capacities
      - resource_capacities
      - class_values
      events:
      - character.class_level_changed
      capabilities:
      - class_progression
      - feature_grants
      - resource_capacities
      - character_choices
    requires:
    - dnd2024:class_progression_wizard_progression
    citations:
    - SRD 5.2.1 pp. 77; builtin/srd-5.2.1/source.txt
---

# Equipment and class progression contracts

This file specifies all **169 assigned definitions**: 25 tools, 81 other gear definitions, 38 weapons, 13 armor definitions, and 12 class progression tables. There are no informational dispositions. The 240 progression rows and all 51 decoded weapon/armor records are retained. Human-readable feature and property labels remain for display; typed grants and property records control their meaning.

The primary source is the bundled [SRD 5.2.1](../../../builtin/srd-5.2.1/source.txt). Gear source hashes intentionally match the original PDF hash recorded by the importer. Weapons and progression tables use the text source hash. Each definition cites its source pages.

The seven pack lists contain all source items and quantities. Ration quantities in these lists are days; an ordinary purchase of Rations has the catalog weight of 2 pounds. Oil quantities mean filled flasks, rather than separately granted empty Flask items. Packs grant their contents once when acquired. Catalog pack weights are preserved even when the source's rounded totals differ from the sum of individual contents. The Waterskin weight is for a full container.

The six equipment variant groups contain 30 variants: 4 gaming sets, 10 instruments, 5 ammunition types, 5 arcane focuses, 3 druidic focuses, and 3 holy symbols. The scroll entry adds its 2 spell-level variants. Variants use a canonical parent key and a local variant ID; they do not invent nonexistent item definitions. The ammunition excerpt also contains the complete Adventuring Gear weight table; its weights have been distributed to the corresponding gear definitions.

Tool operations keep each source DC, governing ability, and craft list. A successful tool check produces the named operation output. Subject identity, a map's extent, resemblance, the applicability of leverage, garment access rules, and comparable materials require bounded choices from the actor or GM. These choices do not replace printed checks or numbers. Tools do not by themselves waive the separate crafting rules.

A container's simultaneous mass and volume limits both apply. Alternative capacities are explicit choices. Sound, lighting, reach, anchor distance, and area geometry are declared quantities; a caller supplies location and eligibility facts because map implementation is outside this research task. An omitted action cost means the source supplies no special action cost. In particular, this file does not invent extra action costs for applying Perfume, bursting Rope, or ordinary observational use.

Weapon mastery requires a feature that unlocks mastery for that weapon. Weapon proficiency alone does not unlock it. Ranged ammunition uses local type selectors, including distinct firearm and sling bullets. Lance's mounted exception changes its two-hand requirement only. Armor applies while equipped, with separate shield and body-armor stacking groups; the body-armor group spans all three weight categories.

Class table row values are current values or capacities, not additive grants at every level. Capacity lookup does not replenish expended resources. Feature rows apply at each crossed class level. Repeated Ability Score Improvement, Expertise, Metamagic, Action Surge, Indomitable, and Mystic Arcanum entries point to the existing canonical feature definition, with the acquisition level retained. Subclass rows grant the selected subclass's features at that class level. A multiclass character gets Proficiency Bonus from total character level and combines spell slots under the separate multiclass rule; Pact Magic remains separate. Feature eligibility, recovery, and choices remain the responsibility of the referenced feature contracts.

Some imported progression Markdown bodies include adjacent two-column feature prose after the table. Those paragraphs are already represented by named class feature definitions; the table contract covers the printed progression table and points to those canonical features. No adjacent feature prose is promoted to an unstructured executable wrapper here.

The source does not specify a default Rope length, a number of Ball Bearings or Caltrops per purchase, or a number of Perfume applications per vial. None is invented. Oil coating follows the printed extra-damage trigger without an added first-trigger-only restriction. Basic Poison expires when its damage is dealt or after one minute; the application records the chosen weapon or up to three pieces of ammunition. Additional adjudication of multiple coated ammunition pieces should use that printed expiry, rather than import a different edition's poison rule.

Validation checked exact assigned-key coverage, all canonical entry references, source metadata hashes, 240 preserved rows, and 51 preserved equipment records. Engine implementation and engine tests are outside this file's scope.

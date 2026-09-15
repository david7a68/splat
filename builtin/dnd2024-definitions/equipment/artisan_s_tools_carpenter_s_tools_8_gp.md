---
splat_schema: 1
splat_id: 73bffff6-cb8d-5944-afd7-117612ef8d4b
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_carpenter_s_tools_8_gp
splat_name: Carpenter’s Tools (8 GP)
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
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
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_carpenter_s_tools_8_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    operation:
      type: choice
      required: true
    target:
      required: false
      type: object
    choices:
      required: false
      type: choice
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
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 93
  engine_gaps: &id001
  - dnd2024:equipment_artisan_s_tools_carpenter_s_tools_8_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 9b841b4c866df1d88c69626b90477683e2778162da651e468a4a2c4d2219ca9d
  parent_name: Artisan’s Tools
  line_span:
    first: 8025
    count: 5
splat_clauses:
- key: source_rules
  text: '# Carpenter’s Tools (8 GP) Ability: Strength Weight: 6 lb. Utilize: Seal or pry open a door or
    container (DC 20) Craft: Club, Greatclub, Quarterstaff, Barrel, Chest, Ladder, Pole, Portable Ram,
    Torch'
  sha256: 41589c059428ad64c1b30d2bf58339aa895bf56e2e2631889de7138532253053
  status: contract_specified
splat_requires_mechanics: *id001
---

# Carpenter’s Tools (8 GP)

<!-- SRD page 93 -->

Ability: Strength   Weight: 6 lb.\
Utilize: Seal or pry open a door or container (DC 20)\
Craft: Club, Greatclub, Quarterstaff, Barrel, Chest, Lad-\
der, Pole, Portable Ram, Torch\

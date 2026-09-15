---
splat_schema: 1
splat_id: 5086c6de-d84a-5b21-a5e5-cfad2cedeeae
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_mason_s_tools_10_gp
splat_name: Mason’s Tools (10 GP)
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
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_mason_s_tools_10_gp
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
  - dnd2024:equipment_artisan_s_tools_mason_s_tools_10_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: a7a2654bd4ca3413b549e03d7387963305f74c076f6a50f586661f5f803b21a8
  parent_name: Artisan’s Tools
  line_span:
    first: 8059
    count: 4
splat_clauses:
- key: source_rules
  text: '# Mason’s Tools (10 GP) Ability: Strength Weight: 8 lb. Utilize: Chisel a symbol or hole in stone
    (DC 10) Craft: Block and Tackle'
  sha256: 5979544d289e0a3a92cfd5b56ff85d8d153280c02003207b4535d93df9bf2ef4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mason’s Tools (10 GP)

<!-- SRD page 93 -->

Ability: Strength   Weight: 8 lb.\
Utilize: Chisel a symbol or hole in stone (DC 10)\
Craft: Block and Tackle\

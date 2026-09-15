---
splat_schema: 1
splat_id: 0be75d37-c22a-5cda-a7cf-bdae9ce4f67d
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_cartographer_s_tools_15_gp
splat_name: Cartographer’s Tools (15 GP)
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
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_cartographer_s_tools_15_gp
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
  - dnd2024:equipment_artisan_s_tools_cartographer_s_tools_15_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 7726eebe994fdb1dcebefd8ac761074b48913be511cf213f00d591de71bbc7df
  parent_name: Artisan’s Tools
  line_span:
    first: 8030
    count: 4
splat_clauses:
- key: source_rules
  text: '# Cartographer’s Tools (15 GP) Ability: Wisdom Weight: 6 lb. Utilize: Draft a map of a small
    area (DC 15) Craft: Map'
  sha256: 4ec030c896bb810e8442a1c6eb989b1fcd372338eed322040c0b867110ec27ce
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cartographer’s Tools (15 GP)

<!-- SRD page 93 -->

Ability: Wisdom   Weight: 6 lb.\
Utilize: Draft a map of a small area (DC 15)\
Craft: Map\

---
splat_schema: 1
splat_id: ad393109-a0dd-5a4e-b750-83ea5e55abc6
splat_kind: item
splat_key: dnd2024:equipment/other_tools_navigator_s_tools_25_gp
splat_name: Navigator’s Tools (25 GP)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
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
splat_contracts:
- id: dnd2024:equipment_other_tools_navigator_s_tools_25_gp
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
  - 94
  engine_gaps: &id001
  - dnd2024:equipment_other_tools_navigator_s_tools_25_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: acb6dbf3f5e8690fc6161170fdafe587b852589ce241b4df03e835bfcab40167
  parent_name: Other Tools
  line_span:
    first: 8128
    count: 4
splat_clauses:
- key: source_rules
  text: '# Navigator’s Tools (25 GP) Ability: Wisdom Weight: 2 lb. Utilize: Plot a course (DC 10), or
    determine position by stargazing (DC 15)'
  sha256: fb386e746a2e0e76fe54805d23c57ac3b32e545cf6549b530e8d3eff41c3f334
  status: contract_specified
splat_requires_mechanics: *id001
---

# Navigator’s Tools (25 GP)

<!-- SRD page 94 -->

Ability: Wisdom   Weight: 2 lb.\
Utilize: Plot a course (DC 10), or determine position by\
stargazing (DC 15)\

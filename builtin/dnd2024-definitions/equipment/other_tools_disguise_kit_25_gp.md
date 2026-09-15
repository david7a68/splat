---
splat_schema: 1
splat_id: ae870cda-c505-58dc-a490-0803e3e55015
splat_kind: item
splat_key: dnd2024:equipment/other_tools_disguise_kit_25_gp
splat_name: Disguise Kit (25 GP)
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
splat_contracts:
- id: dnd2024:equipment_other_tools_disguise_kit_25_gp
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
  - dnd2024:equipment_other_tools_disguise_kit_25_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 6d3025383e7a6aa2da1f647da9add4cbc229fd2af7cab7b2276336a00f1be8b1
  parent_name: Other Tools
  line_span:
    first: 8102
    count: 4
splat_clauses:
- key: source_rules
  text: '# Disguise Kit (25 GP) Ability: Charisma Weight: 3 lb. Utilize: Apply makeup (DC 10) Craft: Costume'
  sha256: 4174d3b99105e5ae9d3cb863d05324260c52a5a11df45060352adc1473c2d9f6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Disguise Kit (25 GP)

<!-- SRD page 94 -->

Ability:  Charisma  Weight: 3 lb.\
Utilize: Apply makeup (DC 10)\
Craft: Costume\

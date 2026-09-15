---
splat_schema: 1
splat_id: 1ae96ba8-215a-5d00-96e2-94883ecaf10b
splat_kind: item
splat_key: dnd2024:equipment/other_tools_thieves_tools_25_gp
splat_name: Thieves’ Tools (25 GP)
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
splat_contracts:
- id: dnd2024:equipment_other_tools_thieves_tools_25_gp
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
  - dnd2024:equipment_other_tools_thieves_tools_25_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 3a741c80b767eaab25991b440514415e53475455b090ad08268b30e8d01b5872
  parent_name: Other Tools
  line_span:
    first: 8136
    count: 3
splat_clauses:
- key: source_rules
  text: '# Thieves’ Tools (25 GP) Ability: Dexterity Weight: 1 lb. Utilize: Pick a lock (DC 15), or disarm
    a trap (DC 15)'
  sha256: 621085da033e53bd85deb161ef68e6d0b81491f41ea610600505323a46e7d0bd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Thieves’ Tools (25 GP)

<!-- SRD page 94 -->

Ability:  Dexterity  Weight: 1 lb.\
Utilize: Pick a lock (DC 15), or disarm a trap (DC 15)\

---
splat_schema: 1
splat_id: 9f2b6828-f8ba-5d78-a57c-4ca5b1bbe0a9
splat_kind: item
splat_key: dnd2024:equipment/other_tools_forgery_kit_15_gp
splat_name: Forgery Kit (15 GP)
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
splat_contracts:
- id: dnd2024:equipment_other_tools_forgery_kit_15_gp
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
  - dnd2024:equipment_other_tools_forgery_kit_15_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 1033be59592361561af217fcf501f766762e352451dbca9668b9b55480711553
  parent_name: Other Tools
  line_span:
    first: 8106
    count: 4
splat_clauses:
- key: source_rules
  text: '# Forgery Kit (15 GP) Ability: Dexterity Weight: 5 lb. Utilize: Mimic 10 or fewer words of someone
    else’s handwriting (DC 15), or duplicate a wax seal (DC 20)'
  sha256: 321320aa415643284bfeb14831f926acb01616cceb33e485fb9d5d34efac5674
  status: contract_specified
splat_requires_mechanics: *id001
---

# Forgery Kit (15 GP)

<!-- SRD page 94 -->

Ability:  Dexterity  Weight: 5 lb.\
Utilize: Mimic 10 or fewer words of someone else’s\
handwriting (DC 15), or duplicate a wax seal (DC 20)\

---
splat_schema: 1
splat_id: 2077d049-21be-5fc9-9fe8-c1f7788dc692
splat_kind: item
splat_key: dnd2024:equipment/other_tools_poisoner_s_kit_50_gp
splat_name: Poisoner’s Kit (50 GP)
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
splat_contracts:
- id: dnd2024:equipment_other_tools_poisoner_s_kit_50_gp
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
  - dnd2024:equipment_other_tools_poisoner_s_kit_50_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 4d6788a8e372cdd67118db139b3e6488da5fc53361daed69e900822c21c8d082
  parent_name: Other Tools
  line_span:
    first: 8132
    count: 4
splat_clauses:
- key: source_rules
  text: '# Poisoner’s Kit (50 GP) Ability: Intelligence Weight: 2 lb. Utilize: Detect a poisoned object
    (DC 10) Craft: Basic Poison'
  sha256: 14e3ef8a2895a5c9eb9e402f08be3546182e5b3b5d47545bee953df2ce9de910
  status: contract_specified
splat_requires_mechanics: *id001
---

# Poisoner’s Kit (50 GP)

<!-- SRD page 94 -->

Ability: Intelligence Weight: 2 lb.\
Utilize: Detect a poisoned object (DC 10)\
Craft: Basic Poison\

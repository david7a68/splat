---
splat_schema: 1
splat_id: 60fb432f-51b6-5ec4-aaeb-410e3bfc7dcb
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_potter_s_tools_10_gp
splat_name: Potter’s Tools (10 GP)
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
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_potter_s_tools_10_gp
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
  - dnd2024:equipment_artisan_s_tools_potter_s_tools_10_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 9033b62f9adf1bc854028c5ec70bd9ea80b39ecc33a1b81e3fc5f27896db9a2d
  parent_name: Artisan’s Tools
  line_span:
    first: 8068
    count: 5
splat_clauses:
- key: source_rules
  text: '# Potter’s Tools (10 GP) Ability: Intelligence Weight: 3 lb. Utilize: Discern what a ceramic
    object held in the past 24 hours (DC 15) Craft: Jug, Lamp'
  sha256: 1cf987b80c06362ba2029fb7c94e2ca8faacd697813382d4703a0b59434c4b4d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potter’s Tools (10 GP)

<!-- SRD page 93 -->

Ability: Intelligence Weight: 3 lb.\
Utilize: Discern what a ceramic object held in the past\
24 hours (DC 15)\
Craft: Jug, Lamp\

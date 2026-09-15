---
splat_schema: 1
splat_id: 521d7211-427e-54e2-a572-884bc3010e49
splat_kind: item
splat_key: dnd2024:equipment/other_tools_herbalism_kit_5_gp
splat_name: Herbalism Kit (5 GP)
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
splat_contracts:
- id: dnd2024:equipment_other_tools_herbalism_kit_5_gp
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
  - dnd2024:equipment_other_tools_herbalism_kit_5_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: c52f3f25def20c7d7c002f5e80c8f0efe75bc63bf26938565c317365a013476d
  parent_name: Other Tools
  line_span:
    first: 8116
    count: 4
splat_clauses:
- key: source_rules
  text: '# Herbalism Kit (5 GP) Ability: Intelligence Weight: 3 lb. Utilize: Identify a plant (DC 10)
    Craft: Antitoxin, Candle, Healer’s Kit, Potion of Healing'
  sha256: eebd6bf9cb9515ba4094075d9eeeb46068bb55ca6ac7a2765119aece2bf2efa1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Herbalism Kit (5 GP)

<!-- SRD page 94 -->

Ability: Intelligence Weight: 3 lb.\
Utilize: Identify a plant (DC 10)\
Craft: Antitoxin, Candle, Healer’s Kit, Potion of Healing\

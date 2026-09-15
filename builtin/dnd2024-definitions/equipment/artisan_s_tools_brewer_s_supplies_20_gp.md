---
splat_schema: 1
splat_id: 06649811-baac-5864-9a11-e2a7d49c340c
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_brewer_s_supplies_20_gp
splat_name: Brewer’s Supplies (20 GP)
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
    value: 20
    unit: gp
  mass:
    value: 9
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/artisan_s_tools_brewer_s_supplies_20_gp
  implementation: dnd2024:equipment_artisan_s_tools_brewer_s_supplies_20_gp
  parameters:
    tool:
      ability: intelligence
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: detect_poisoned_drink
        check:
          ability: intelligence
          dc: 15
      - action: utilize
        operation: identify_alcohol
        check:
          ability: intelligence
          dc: 10
      craftable:
      - key: dnd2024:equipment/antitoxin_50_gp
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    mass:
      value: 9
      unit: lb
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_brewer_s_supplies_20_gp
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
  - dnd2024:equipment_artisan_s_tools_brewer_s_supplies_20_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 331acf732180620ff13f13359aacc9cbfc0ace0f6dce40b2ff0159c616764b99
  parent_name: Artisan’s Tools
  line_span:
    first: 8015
    count: 5
splat_clauses:
- key: source_rules
  text: '# Brewer’s Supplies (20 GP) Ability: Intelligence Weight: 9 lb. Utilize: Detect poisoned drink
    (DC 15), or identify alcohol (DC 10) Craft: Antitoxin'
  sha256: 30b066cab9bb4903b23b2010ec76664b06c397190b72286fdd77ea1ccb679db5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Brewer’s Supplies (20 GP)

<!-- SRD page 93 -->

Ability: Intelligence Weight: 9 lb.\
Utilize: Detect poisoned drink (DC 15), or identify alco-\
hol (DC 10)\
Craft: Antitoxin\

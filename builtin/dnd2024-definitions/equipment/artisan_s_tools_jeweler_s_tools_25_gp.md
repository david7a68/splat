---
splat_schema: 1
splat_id: 59d7c58e-2545-588d-bfdf-13318952f1a4
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_jeweler_s_tools_25_gp
splat_name: Jeweler’s Tools (25 GP)
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
    value: 25
    unit: gp
  mass:
    value: 2
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/artisan_s_tools_jeweler_s_tools_25_gp
  implementation: dnd2024:equipment_artisan_s_tools_jeweler_s_tools_25_gp
  parameters:
    tool:
      ability: intelligence
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: appraise_gem
        check:
          ability: intelligence
          dc: 15
      craftable:
      - key: dnd2024:equipment/arcane_focus_varies
      - key: dnd2024:equipment/holy_symbol_varies
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    mass:
      value: 2
      unit: lb
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_jeweler_s_tools_25_gp
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
  - dnd2024:equipment_artisan_s_tools_jeweler_s_tools_25_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: ec4256e4ce5156c103c826f203920600ac95054b50f808e734fce888fd86df99
  parent_name: Artisan’s Tools
  line_span:
    first: 8049
    count: 4
splat_clauses:
- key: source_rules
  text: '# Jeweler’s Tools (25 GP) Ability: Intelligence Weight: 2 lb. Utilize: Discern a gem’s value
    (DC 15) Craft: Arcane Focus, Holy Symbol'
  sha256: da620a433a4a4387192b44fa48e4eab741a2b005c74748d8fbdbb02ff9fa7434
  status: contract_specified
splat_requires_mechanics: *id001
---

# Jeweler’s Tools (25 GP)

<!-- SRD page 93 -->

Ability: Intelligence Weight: 2 lb.\
Utilize: Discern a gem’s value (DC 15)\
Craft: Arcane Focus, Holy Symbol\

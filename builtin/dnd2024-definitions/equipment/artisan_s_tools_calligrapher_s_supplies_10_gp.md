---
splat_schema: 1
splat_id: 21f16989-a8e1-5ad8-b1f0-0e98edd9cb8b
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_calligrapher_s_supplies_10_gp
splat_name: Calligrapher’s Supplies (10 GP)
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
    value: 5
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/artisan_s_tools_calligrapher_s_supplies_10_gp
  implementation: dnd2024:equipment_artisan_s_tools_calligrapher_s_supplies_10_gp
  parameters:
    tool:
      ability: dexterity
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: write_forgery_resistant_flourishes
        check:
          ability: dexterity
          dc: 15
      craftable:
      - key: dnd2024:equipment/ink_10_gp
      - key: dnd2024:equipment/spell_scroll_cantrip_30_gp_level_1_50_gp
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    mass:
      value: 5
      unit: lb
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_calligrapher_s_supplies_10_gp
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
  - dnd2024:equipment_artisan_s_tools_calligrapher_s_supplies_10_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 3777b26801858cf04a3ea9f61657d1374348a930940c696a318ada705f13ac96
  parent_name: Artisan’s Tools
  line_span:
    first: 8020
    count: 5
splat_clauses:
- key: source_rules
  text: '# Calligrapher’s Supplies (10 GP) Ability: Dexterity Weight: 5 lb. Utilize: Write text with impressive
    flourishes that guard against forgery (DC 15) Craft: Ink, Spell Scroll'
  sha256: 45c99e0693adad89363a9565da5d6193a1be6d394a5e70a088d58449f0879f19
  status: contract_specified
splat_requires_mechanics: *id001
---

# Calligrapher’s Supplies (10 GP)

<!-- SRD page 93 -->

Ability:  Dexterity  Weight: 5 lb.\
Utilize: Write text with impressive flourishes that guard\
against forgery (DC 15)\
Craft: Ink, Spell Scroll\

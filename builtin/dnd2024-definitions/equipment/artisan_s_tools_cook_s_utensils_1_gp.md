---
splat_schema: 1
splat_id: ec93020c-d470-5c52-8666-c2fb31533727
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_cook_s_utensils_1_gp
splat_name: Cook’s Utensils (1 GP)
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
    value: 1
    unit: gp
  mass:
    value: 8
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/artisan_s_tools_cook_s_utensils_1_gp
  implementation: dnd2024:equipment_artisan_s_tools_cook_s_utensils_1_gp
  parameters:
    tool:
      ability: wisdom
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: improve_food_flavor
        check:
          ability: wisdom
          dc: 10
      - action: utilize
        operation: detect_spoiled_or_poisoned_food
        check:
          ability: wisdom
          dc: 15
      craftable:
      - key: dnd2024:equipment/rations_5_sp
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    mass:
      value: 8
      unit: lb
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_cook_s_utensils_1_gp
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
  - dnd2024:equipment_artisan_s_tools_cook_s_utensils_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: d16f78f5a6f18226cbd438607849853840b0b43d9c99f2b0b87ac65f4e516b3d
  parent_name: Artisan’s Tools
  line_span:
    first: 8039
    count: 5
splat_clauses:
- key: source_rules
  text: '# Cook’s Utensils (1 GP) Ability: Wisdom Weight: 8 lb. Utilize: Improve food’s flavor (DC 10),
    or detect spoiled or poisoned food (DC 15) Craft: Rations'
  sha256: 9e0dd325acc08ccfe64a992034c0720759a7998de5ead0745f4064df67c3f2b4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cook’s Utensils (1 GP)

<!-- SRD page 93 -->

Ability: Wisdom   Weight: 8 lb.\
Utilize: Improve food’s flavor (DC 10), or detect spoiled\
or poisoned food (DC 15)\
Craft: Rations\

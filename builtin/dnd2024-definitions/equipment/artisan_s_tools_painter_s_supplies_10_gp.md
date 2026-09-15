---
splat_schema: 1
splat_id: f9673160-cb1c-5e46-9c4e-eefcc9259355
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_painter_s_supplies_10_gp
splat_name: Painter’s Supplies (10 GP)
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
  mechanic: dnd2024:equipment/artisan_s_tools_painter_s_supplies_10_gp
  implementation: dnd2024:equipment_artisan_s_tools_painter_s_supplies_10_gp
  parameters:
    tool:
      ability: wisdom
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: paint_recognizable_image
        check:
          ability: wisdom
          dc: 10
        requires: subject_previously_seen
      craftable:
      - key: dnd2024:equipment/druidic_focus_varies
      - key: dnd2024:equipment/holy_symbol_varies
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    mass:
      value: 5
      unit: lb
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_painter_s_supplies_10_gp
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
  - dnd2024:equipment_artisan_s_tools_painter_s_supplies_10_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 8cce4d3adeac9570620bc42e21a88c1aed0a763666c84380cbb64ff5624ac103
  parent_name: Artisan’s Tools
  line_span:
    first: 8063
    count: 5
splat_clauses:
- key: source_rules
  text: '# Painter’s Supplies (10 GP) Ability: Wisdom Weight: 5 lb. Utilize: Paint a recognizable image
    of something you’ve seen (DC 10) Craft: Druidic Focus, Holy Symbol'
  sha256: 6bed097b919bb68e26edb634a6a10dc93607951746bb29b55a2c45cfbb906c3d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Painter’s Supplies (10 GP)

<!-- SRD page 93 -->

Ability: Wisdom   Weight: 5 lb.\
Utilize: Paint a recognizable image of something you’ve\
seen (DC 10)\
Craft: Druidic Focus, Holy Symbol\

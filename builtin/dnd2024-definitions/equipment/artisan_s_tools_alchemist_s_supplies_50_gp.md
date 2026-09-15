---
splat_schema: 1
splat_id: 0c6ccdcd-ebfb-580d-b367-214af7def135
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_alchemist_s_supplies_50_gp
splat_name: Alchemist’s Supplies (50 GP)
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
    value: 50
    unit: gp
  mass:
    value: 8
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/artisan_s_tools_alchemist_s_supplies_50_gp
  implementation: dnd2024:equipment_artisan_s_tools_alchemist_s_supplies_50_gp
  parameters:
    tool:
      ability: intelligence
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: identify_substance
        check:
          ability: intelligence
          dc: 15
      - action: utilize
        operation: ignite_fire
        check:
          ability: intelligence
          dc: 15
      craftable:
      - key: dnd2024:equipment/acid_25_gp
      - key: dnd2024:equipment/alchemist_s_fire_50_gp
      - key: dnd2024:equipment/component_pouch_25_gp
      - key: dnd2024:equipment/oil_1_sp
      - key: dnd2024:equipment/paper_2_sp
      - key: dnd2024:equipment/perfume_5_gp
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    mass:
      value: 8
      unit: lb
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_alchemist_s_supplies_50_gp
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
  - dnd2024:equipment_artisan_s_tools_alchemist_s_supplies_50_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 18c098cecc1fcc540be27c54a4584a5c2e2964e9f970bcb34447dde68d10b23d
  parent_name: Artisan’s Tools
  line_span:
    first: 8009
    count: 6
splat_clauses:
- key: source_rules
  text: '# Alchemist’s Supplies (50 GP) Ability: Intelligence Weight: 8 lb. Utilize: Identify a substance
    (DC 15), or start a fire (DC 15) Craft: Acid, Alchemist’s Fire, Component Pouch, Oil, Paper, Perfume'
  sha256: 3686beda15d0700b7aeeda229dad4bf963f0024c3265955eec54df12c1ef221b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Alchemist’s Supplies (50 GP)

<!-- SRD page 93 -->

Ability: Intelligence Weight: 8 lb.\
Utilize: Identify a substance (DC 15), or start a fire\
(DC 15)\
Craft: Acid, Alchemist’s Fire, Component Pouch, Oil,\
Paper, Perfume\

---
splat_schema: 1
splat_id: e1d15a2f-7172-557d-b647-188b224a322e
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_weaver_s_tools_1_gp
splat_name: Weaver’s Tools (1 GP)
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
    value: 1
    unit: gp
  mass:
    value: 5
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/artisan_s_tools_weaver_s_tools_1_gp
  implementation: dnd2024:equipment_artisan_s_tools_weaver_s_tools_1_gp
  parameters:
    tool:
      ability: dexterity
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: mend_clothing_tear
        check:
          ability: dexterity
          dc: 10
      - action: utilize
        operation: sew_design
        check:
          ability: dexterity
          dc: 10
        size: tiny
      craftable:
      - key: dnd2024:equipment/padded_armor
      - key: dnd2024:equipment/basket_4_sp
      - key: dnd2024:equipment/bedroll_1_gp
      - key: dnd2024:equipment/blanket_5_sp
      - key: dnd2024:equipment/clothes_fine_15_gp
      - key: dnd2024:equipment/net_1_gp
      - key: dnd2024:equipment/robe_1_gp
      - key: dnd2024:equipment/rope_1_gp
      - key: dnd2024:equipment/sack_1_cp
      - key: dnd2024:equipment/string_1_sp
      - key: dnd2024:equipment/tent_2_gp
      - key: dnd2024:equipment/clothes_traveler_s_2_gp
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    mass:
      value: 5
      unit: lb
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_weaver_s_tools_1_gp
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
  - dnd2024:equipment_artisan_s_tools_weaver_s_tools_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: f9cd77b5ccd95408dea8156940e26184daa9168a645280bafd442b16b7bf0117
  parent_name: Artisan’s Tools
  line_span:
    first: 8088
    count: 7
splat_clauses:
- key: source_rules
  text: '# Weaver’s Tools (1 GP) Ability: Dexterity Weight: 5 lb. Utilize: Mend a tear in clothing (DC
    10), or sew a Tiny design (DC 10) Craft: Padded Armor, Basket, Bedroll, Blanket, Fine Clothes, Net,
    Robe, Rope, Sack, String, Tent, Traveler’s Clothes'
  sha256: a9e5c73a9520b6f5b440810664bf1e4cf21d6b67499e318306dd887b1d30305a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Weaver’s Tools (1 GP)

<!-- SRD page 94 -->

Ability:  Dexterity  Weight: 5 lb.\
Utilize: Mend a tear in clothing (DC 10), or sew a Tiny\
design (DC 10)\
Craft: Padded Armor, Basket, Bedroll, Blanket, Fine\
Clothes, Net, Robe, Rope, Sack, String, Tent, Traveler’s\
Clothes\

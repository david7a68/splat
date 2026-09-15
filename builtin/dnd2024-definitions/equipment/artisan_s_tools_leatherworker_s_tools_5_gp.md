---
splat_schema: 1
splat_id: 675c7a41-552f-588c-8c8b-132b716a8f85
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_leatherworker_s_tools_5_gp
splat_name: Leatherworker’s Tools (5 GP)
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
    value: 5
    unit: gp
  mass:
    value: 5
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/artisan_s_tools_leatherworker_s_tools_5_gp
  implementation: dnd2024:equipment_artisan_s_tools_leatherworker_s_tools_5_gp
  parameters:
    tool:
      ability: dexterity
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: decorate_leather
        check:
          ability: dexterity
          dc: 10
      craftable:
      - key: dnd2024:equipment/sling
      - key: dnd2024:equipment/whip
      - key: dnd2024:equipment/hide_armor
      - key: dnd2024:equipment/leather_armor
      - key: dnd2024:equipment/studded_leather_armor
      - key: dnd2024:equipment/backpack_2_gp
      - key: dnd2024:equipment/case_crossbow_bolt_1_gp
      - key: dnd2024:equipment/case_map_or_scroll_1_gp
      - key: dnd2024:equipment/parchment_1_sp
      - key: dnd2024:equipment/pouch_5_sp
      - key: dnd2024:equipment/quiver_1_gp
      - key: dnd2024:equipment/waterskin_2_sp
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    mass:
      value: 5
      unit: lb
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_leatherworker_s_tools_5_gp
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
  - dnd2024:equipment_artisan_s_tools_leatherworker_s_tools_5_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: abb9c4d0c192b1bae32a2857a52520399bab34677a1fe2c60c1bfb5e3f865acf
  parent_name: Artisan’s Tools
  line_span:
    first: 8053
    count: 6
splat_clauses:
- key: source_rules
  text: '# Leatherworker’s Tools (5 GP) Ability: Dexterity Weight: 5 lb. Utilize: Add a design to a leather
    item (DC 10) Craft: Sling, Whip, Hide Armor, Leather Armor, Studded Leather Armor, Backpack, Crossbow
    Bolt Case, Map or Scroll Case, Parchment, Pouch, Quiver, Waterskin'
  sha256: 26e2450e5c3a0516e5756d87de6f50335eeea802cf817f52957a5d62dfbdf703
  status: contract_specified
splat_requires_mechanics: *id001
---

# Leatherworker’s Tools (5 GP)

<!-- SRD page 93 -->

Ability:  Dexterity  Weight: 5 lb.\
Utilize: Add a design to a leather item (DC 10)\
Craft: Sling, Whip, Hide Armor, Leather Armor, Studded\
Leather Armor, Backpack, Crossbow Bolt Case, Map\
or Scroll Case, Parchment, Pouch, Quiver, Waterskin\

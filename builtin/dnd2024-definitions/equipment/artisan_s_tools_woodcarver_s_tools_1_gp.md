---
splat_schema: 1
splat_id: c924f49f-7e03-52f9-bd4a-4e08a5e8cc0f
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_woodcarver_s_tools_1_gp
splat_name: Woodcarver’s Tools (1 GP)
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
  mechanic: dnd2024:equipment/artisan_s_tools_woodcarver_s_tools_1_gp
  implementation: dnd2024:equipment_artisan_s_tools_woodcarver_s_tools_1_gp
  parameters:
    tool:
      ability: dexterity
      proficiency_bonus_if_proficient: true
      skill_and_tool_proficiency_grants_advantage: true
      utilize:
      - action: utilize
        operation: carve_wood_pattern
        check:
          ability: dexterity
          dc: 10
      craftable:
      - key: dnd2024:equipment/club
      - key: dnd2024:equipment/greatclub
      - key: dnd2024:equipment/quarterstaff
      - key: dnd2024:equipment/arcane_focus_varies
      - key: dnd2024:equipment/druidic_focus_varies
      - key: dnd2024:equipment/ink_pen_2_cp
      - key: dnd2024:equipment/dart
      - key: dnd2024:equipment/light_crossbow
      - key: dnd2024:equipment/shortbow
      - key: dnd2024:equipment/blowgun
      - key: dnd2024:equipment/hand_crossbow
      - key: dnd2024:equipment/heavy_crossbow
      - key: dnd2024:equipment/longbow
      - key: dnd2024:equipment/ammunition_varies
        variant: arrows
      - key: dnd2024:equipment/ammunition_varies
        variant: bolts
      - key: dnd2024:equipment/ammunition_varies
        variant: needles
      monster_proficient_if_tool_in_stat_block: true
      proficiency_rule:
        key: dnd2024:rules/tools_tool_proficiency
      crafting_rule:
        key: dnd2024:rules/crafting_nonmagical_items_tools
    mass:
      value: 5
      unit: lb
splat_contracts:
- id: dnd2024:equipment_artisan_s_tools_woodcarver_s_tools_1_gp
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
  - dnd2024:equipment_artisan_s_tools_woodcarver_s_tools_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: de9aaedcaba664488e0b02d06c713e589cd375ec5e66bd8ae6f61f2c9a8edc89
  parent_name: Artisan’s Tools
  line_span:
    first: 8095
    count: 6
splat_clauses:
- key: source_rules
  text: '# Woodcarver’s Tools (1 GP) Ability: Dexterity Weight: 5 lb. Utilize: Carve a pattern in wood
    (DC 10) Craft: Club, Greatclub, Quarterstaff, Ranged weapons (except Pistol, Musket, and Sling), Arcane
    Focus, Arrows, Bolts, Druidic Focus, Ink Pen, Needles'
  sha256: 7bb3d070560ed3830685e29e85a9ac5546de7be91a73ac22030f0a3c77d81b03
  status: contract_specified
splat_requires_mechanics: *id001
---

# Woodcarver’s Tools (1 GP)

<!-- SRD page 94 -->

Ability:  Dexterity  Weight: 5 lb.\
Utilize: Carve a pattern in wood (DC 10)\
Craft: Club, Greatclub, Quarterstaff, Ranged weapons\
(except Pistol, Musket, and Sling), Arcane Focus, Ar-\
rows, Bolts, Druidic Focus, Ink Pen, Needles\

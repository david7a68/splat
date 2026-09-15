---
splat_schema: 1
splat_id: 5a50b803-e3c0-5ced-af76-c9d017c48c68
splat_kind: rule
splat_key: dnd2024:rules/crafting_magic_items_tools
splat_name: Tools
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206, 207
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/crafting_magic_items_tools
  parameters:
    required_tool_use: true
    creator_proficiency_required: true
    assistant_proficiency_required: true
    category_tools:
      armor:
        options:
        - leatherworkers_tools
        - smiths_tools
        - weavers_tools
        selection: nonmagical_armor_tool_requirement
      potion:
        options:
        - alchemists_supplies
        - herbalism_kit
        selection: any
      ring:
        options:
        - jewelers_tools
      rod:
        options:
        - woodcarvers_tools
      scroll:
        options:
        - calligraphers_supplies
      staff:
        options:
        - woodcarvers_tools
      wand:
        options:
        - woodcarvers_tools
      weapon:
        options:
        - leatherworkers_tools
        - smiths_tools
        - woodcarvers_tools
        selection: nonmagical_weapon_tool_requirement
      wondrous_item:
        options:
        - tinkers_tools
        - nonmagical_base_item_required_tool
        selection: any
  implementation: dnd2024:rules_crafting_magic_items_tools
splat_contracts:
- id: dnd2024:rules_crafting_magic_items_tools
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - item_attributes
  - actor_modifiers
  events: []
  capabilities:
  - equipment
  - armor_class
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 206
  - 207
  engine_gaps: &id001
  - dnd2024:rules_crafting_magic_items_tools
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 99f2bff2b2b3809c8ff7440514a01aa7ef41634d3f3c8449a13563ea4e023019
  parent_name: Crafting Magic Items
  line_span:
    first: 10667
    count: 27
splat_clauses:
- key: source_rules
  text: '# Tools The Magic Item Tools table lists which tool is required to make a magic item of each
    category. You must use the required tool to make an item and have proficiency with that tool. Any
    assistants must also have proficiency with it. For more information on the tools, see “Equipment.”
    Magic Item Tools Item Category Required Tool Armor Leatherworker’s Tools, Smith’s Tools, or Weaver’s
    Tools depending on the kind of armor as noted in the tools’ descriptions Potion Alchemist’s Supplies
    or Herbalism Kit Ring Jeweler’s Tools Rod Woodcarver’s Tools Scroll Calligrapher’s Supplies Staff
    Woodcarver’s Tools Wand Woodcarver’s Tools Weapon Leatherworker’s Tools, Smith’s Tools, or Woodcarver’s
    Tools depending on the kind of weapon as noted in the tools’ descriptions Wondrous Item Tinker’s Tools
    or the tool required to make the nonmagical item on which the magic item is based'
  sha256: fcfaca0e07e7cd46e4616ab00e60ae08b22287c760e0ea78d079bb4affaa8aba
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tools

<!-- SRD page 206 -->

The Magic Item Tools table lists which tool is re-\
quired to make a magic item of each category. You\
must use the required tool to make an item and\
have proficiency with that tool. Any assistants must\
also have proficiency with it. For more information\
on the tools, see “Equipment.”\

<!-- SRD page 207 -->

Magic Item Tools\
Item Category Required Tool\
Armor Leatherworker’s Tools, Smith’s\
Tools, or Weaver’s Tools depend-\
ing on the kind of armor as noted\
in the tools’ descriptions\
Potion Alchemist’s Supplies or Herbalism\
Kit\
Ring Jeweler’s Tools\
Rod Woodcarver’s Tools\
Scroll Calligrapher’s Supplies\
Staff Woodcarver’s Tools\
Wand Woodcarver’s Tools\
Weapon Leatherworker’s Tools, Smith’s\
Tools, or Woodcarver’s Tools de-\
pending on the kind of weapon as\
noted in the tools’ descriptions\
Wondrous Item Tinker’s Tools or the tool required\
to make the nonmagical item on\
which the magic item is based\

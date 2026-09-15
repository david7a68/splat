---
splat_schema: 1
splat_id: cda5ae0b-7540-5c91-ac01-a4a7d32ec809
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories
splat_name: Magic Item Categories
splat_origin:
  key: dnd2024:reference/pages/204
splat_sources:
- title: SRD 5.2.1
  locator: pp. 204
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_item_categories
  parameters:
    magic_item_category_count: 1
    category_options:
    - armor
    - potion
    - ring
    - rod
    - scroll
    - staff
    - wand
    - weapon
    - wondrous_item
  implementation: dnd2024:rule_magic_item_categories
splat_contracts:
- id: dnd2024:rule_magic_item_categories
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - item_changes
  events: []
  capabilities:
  - magic_items
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 204
  engine_gaps: &id001
  - dnd2024:rule_magic_item_categories
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: fd0a5990841a793b0395cd103073df022a102660f92ae9ca620feccadd36d698
  parent_name: Magic Item Categories
  line_span:
    first: 10396
    count: 16
splat_clauses:
- key: source_rules
  text: '# Magic Item Categories Every magic item belongs to a category. The Magic Item Categories table
    lists the nine categories and provides examples. Rules for the categories appear after the table.
    Magic Item Categories Category Examples Armor +1 Leather Armor, +1 Shield Potions Potion of Healing
    Rings Ring of Invisibility Rods Immovable Rod Scrolls Spell Scroll Staffs Staff of Striking Wands
    Wand of Fireballs Weapons +1 Ammunition, +1 Longsword Wondrous Items Bag of Holding, Boots of Elvenkind'
  sha256: 85b368a6fef74bfd150960b4ff75a2b513d2acd269c901e1ba5cd07b09066f54
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magic Item Categories

<!-- SRD page 204 -->

Every magic item belongs to a category. The Magic\
Item Categories table lists the nine categories and\
provides examples. Rules for the categories appear\
after the table.\
Magic Item Categories\
Category Examples\
Armor +1 Leather Armor, +1 Shield\
Potions Potion of Healing\
Rings Ring of Invisibility\
Rods Immovable Rod\
Scrolls Spell Scroll\
Staffs Staff of Striking\
Wands Wand of Fireballs\
Weapons +1 Ammunition, +1 Longsword\
Wondrous Items Bag of Holding, Boots of Elvenkind\

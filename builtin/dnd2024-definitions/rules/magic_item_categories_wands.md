---
splat_schema: 1
splat_id: f2c0f678-f96e-57f5-be17-644b3b43e49d
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_wands
splat_name: Wands
splat_origin:
  key: dnd2024:reference/pages/205
splat_sources:
- title: SRD 5.2.1
  locator: pp. 205
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_item_categories_wands
  parameters:
    wand:
      typical_length_inches:
        minimum: 12
        maximum: 15
      arcane_focus: true
      description_can_override: true
  implementation: dnd2024:rules_magic_item_categories_wands
splat_contracts:
- id: dnd2024:rules_magic_item_categories_wands
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
  - 205
  engine_gaps: &id001
  - dnd2024:rules_magic_item_categories_wands
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: ccfa1439d77c97c7f1461a2ba2cec89b188d1da9ffb23850e8825c9515571cbf
  parent_name: Magic Item Categories
  line_span:
    first: 10507
    count: 7
splat_clauses:
- key: source_rules
  text: '# Wands An item in the Wand category is typically 12 to 15 inches long and crafted of metal,
    bone, or wood. It is tipped with metal, crystal, stone, or some other material. Unless its description
    notes otherwise, a wand can be used as an Arcane Focus.'
  sha256: 3bf6a0d8f7073cfad8d69d451a496413d7e6393eaa97b727631f6d24f30a81da
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wands

<!-- SRD page 205 -->

An item in the Wand category is typically 12 to 15\
inches long and crafted of metal, bone, or wood. It\
is tipped with metal, crystal, stone, or some other\
material.\
Unless its description notes otherwise, a wand\
can be used as an Arcane Focus.\

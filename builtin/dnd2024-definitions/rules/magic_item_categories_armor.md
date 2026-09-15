---
splat_schema: 1
splat_id: a7024fb9-92e0-5b68-87a8-2ff015f95749
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_armor
splat_name: Armor
splat_origin:
  key: dnd2024:reference/pages/204
splat_sources:
- title: SRD 5.2.1
  locator: pp. 204
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_item_categories_armor
  parameters:
    magic_armor:
      worn_required_for_magic: true
      description_can_override: true
      unspecified_armor_type:
        choice_authority: gm
        methods:
        - choose
        - random
        options: armor_table_types
  implementation: dnd2024:rules_magic_item_categories_armor
splat_contracts:
- id: dnd2024:rules_magic_item_categories_armor
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
  - 204
  engine_gaps: &id001
  - dnd2024:rules_magic_item_categories_armor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: c515822313c5000c71b60f452edf5267fff935c28bb0d69f223819753ea70611
  parent_name: Magic Item Categories
  line_span:
    first: 10412
    count: 9
splat_clauses:
- key: source_rules
  text: '# Armor An item in the Armor category is typically a magical version of armor from “Equipment.”
    Unless an armor’s description notes otherwise, the armor must be worn for its magic to function. Some
    suits of magic armor specify the type of armor they are, such as Chain Mail or Plate Armor. If no
    type is specified, choose the type or determine it randomly.'
  sha256: 284f5bdc6f28eb878708ade5ba72b87f31203651747b7e61cf8a019fdd332db5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Armor

<!-- SRD page 204 -->

An item in the Armor category is typically a magical\
version of armor from “Equipment.” Unless an ar-\
mor’s description notes otherwise, the armor must\
be worn for its magic to function.\
Some suits of magic armor specify the type of ar-\
mor they are, such as Chain Mail or Plate Armor. If\
no type is specified, choose the type or determine it\
randomly.\

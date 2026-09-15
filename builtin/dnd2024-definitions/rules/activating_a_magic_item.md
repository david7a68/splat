---
splat_schema: 1
splat_id: 38fb70ea-cb99-5c05-b5a9-2d497b879ba8
splat_kind: rule
splat_key: dnd2024:rules/activating_a_magic_item
splat_name: Activating a Magic Item
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/activating_a_magic_item
  parameters:
    default_activation_action: magic
    additional_requirements_source:
    - item_category
    - item_description
    item_description_overrides: true
  implementation: dnd2024:rule_activating_a_magic_item
splat_contracts:
- id: dnd2024:rule_activating_a_magic_item
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
  - actions
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 206
  engine_gaps: &id001
  - dnd2024:rule_activating_a_magic_item
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 1be829642c2b7fabd2dcb8dc94bc56e9a6be6012747b2b79cdd3ffcdfa7f0576
  parent_name: Activating a Magic Item
  line_span:
    first: 10591
    count: 7
splat_clauses:
- key: source_rules
  text: '# Activating a Magic Item It usually takes a Magic action to activate a magic item. The item’s
    user might also need to do something special. The description of each item category or individual
    item details how an item is activated. Certain items use the following rules for their activation.'
  sha256: 2d689d3a700704483eb981dd99d8990aed7856693677aa9c64d0850852744396
  status: contract_specified
splat_requires_mechanics: *id001
---

# Activating a Magic Item

<!-- SRD page 206 -->

It usually takes a Magic action to activate a magic\
item. The item’s user might also need to do some-\
thing special. The description of each item category\
or individual item details how an item is activated.\
Certain items use the following rules for their\
activation.\

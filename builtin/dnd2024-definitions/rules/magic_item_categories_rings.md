---
splat_schema: 1
splat_id: 8995e3f7-0cd2-5e85-8859-810fbc49be86
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_rings
splat_name: Rings
splat_origin:
  key: dnd2024:reference/pages/204
splat_sources:
- title: SRD 5.2.1
  locator: pp. 204
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_item_categories_rings
  parameters:
    category: ring
    magic_requires: worn_on_finger_or_similar_digit
    description_exception_overrides: true
  implementation: dnd2024:rule_magic_item_categories_rings
splat_contracts:
- id: dnd2024:rule_magic_item_categories_rings
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
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 204
  engine_gaps: &id001
  - dnd2024:rule_magic_item_categories_rings
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: ab0d449dc07d3373c14e2cbb8215dda972d45a8cb8115d900527e928f2eb35f5
  parent_name: Magic Item Categories
  line_span:
    first: 10473
    count: 4
splat_clauses:
- key: source_rules
  text: '# Rings For its magic to function, an item in the Ring category must be worn on a finger or a
    similar digit unless its description notes otherwise.'
  sha256: ab777982de96f423aa2c8848f500546ae3a750180147b5929a0669df505ff807
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rings

<!-- SRD page 204 -->

For its magic to function, an item in the Ring cate-\
gory must be worn on a finger or a similar digit un-\
less its description notes otherwise.\

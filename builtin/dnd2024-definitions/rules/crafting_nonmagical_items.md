---
splat_schema: 1
splat_id: 10605318-c041-5207-be01-b99126d74e7c
splat_kind: rule
splat_key: dnd2024:rules/crafting_nonmagical_items
splat_name: Crafting Nonmagical Items
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/crafting_nonmagical_items
  parameters:
    requirements:
    - tools
    - raw_materials
    - time
    requirements_satisfied:
      create_item: true
      allow_use: true
      sale_price: normal_item_price
    requirement_rules:
    - key: dnd2024:rules/crafting_nonmagical_items_tools
    - key: dnd2024:rules/crafting_nonmagical_items_raw_materials
    - key: dnd2024:rules/crafting_nonmagical_items_time
  implementation: dnd2024:rules_crafting_nonmagical_items
splat_contracts:
- id: dnd2024:rules_crafting_nonmagical_items
  inputs:
    subject:
      type: participant
      required: true
    item:
      type: definition
      required: true
    requirements:
      type: resolution
      required: true
  outputs:
  - crafted_item
  events: []
  capabilities:
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 103
  engine_gaps: &id001
  - dnd2024:rules_crafting_nonmagical_items
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: f12b901f1ddb7a1791d3d2656e996c64edce60b45aaa959d6efe10145de55698
  parent_name: Crafting Nonmagical Items
  line_span:
    first: 8960
    count: 5
splat_clauses:
- key: source_rules
  text: '# Crafting Nonmagical Items To craft a nonmagical item, you need tools, raw materials, and time,
    each of which is detailed below. If you meet the requirements, you make the item, and you can use
    it or sell it at its normal price.'
  sha256: 222b159f42fd972a6049a51325344f9d70ca2b934799f62d4befe52b6f16ec12
  status: contract_specified
splat_requires_mechanics: *id001
---

# Crafting Nonmagical Items

<!-- SRD page 103 -->

To craft a nonmagical item, you need tools, raw ma-\
terials, and time, each of which is detailed below. If\
you meet the requirements, you make the item, and\
you can use it or sell it at its normal price.\

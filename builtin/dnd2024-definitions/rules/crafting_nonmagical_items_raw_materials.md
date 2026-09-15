---
splat_schema: 1
splat_id: e04d38f4-48df-5dcf-85b2-3f43dec3a84f
splat_kind: rule
splat_key: dnd2024:rules/crafting_nonmagical_items_raw_materials
splat_name: Raw Materials
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/crafting_nonmagical_items_raw_materials
  parameters:
    raw_material_cost:
      op: floor
      value:
        op: multiply
        left: purchase_cost
        right: 0.5
    appropriate_material_availability_authority: gm
  implementation: dnd2024:rules_crafting_nonmagical_items_raw_materials
splat_contracts:
- id: dnd2024:rules_crafting_nonmagical_items_raw_materials
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
    event:
      required: false
      type: event
  outputs:
  - item_updates
  - cost
  - effects
  events: []
  capabilities:
  - items
  - attunement
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 103
  engine_gaps: &id001
  - dnd2024:rules_crafting_nonmagical_items_raw_materials
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: b4531accbcfda2a0755eda378c006f3aaf089ff691ca7b22d982490b98bac8f5
  parent_name: Crafting Nonmagical Items
  line_span:
    first: 8972
    count: 6
splat_clauses:
- key: source_rules
  text: '# Raw Materials To make an item, you need raw materials worth half its purchase cost (round down).
    For example, you need 750 GP of raw materials to make Plate Armor, which sells for 1,500 GP. The GM
    determines whether appropriate raw materials are available.'
  sha256: 2b29c1cd9d19d0b58ca167922436f6f3a958d8e42dd74a8a57f16fd4ac989fa4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Raw Materials

<!-- SRD page 103 -->

To make an item, you need raw materials worth\
half its purchase cost (round down). For example,\
you need 750 GP of raw materials to make Plate\
Armor, which sells for 1,500 GP. The GM determines\
whether appropriate raw materials are available.\

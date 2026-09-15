---
splat_schema: 1
splat_id: 5840ec52-b305-531a-a1f5-7b1b4aa2c773
splat_kind: rule
splat_key: dnd2024:rules/armor_shield
splat_name: Shield
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/armor_shield
  parameters:
    shield_ac_benefit_requires_training: true
  implementation: dnd2024:rules_armor_shield
splat_contracts:
- id: dnd2024:rules_armor_shield
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
  - 92
  engine_gaps: &id001
  - dnd2024:rules_armor_shield
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 697ec4decf31abce5ab9375e006a5a35368a91bf41f304a056fc77a24eee98d3
  parent_name: Armor
  line_span:
    first: 7957
    count: 3
splat_clauses:
- key: source_rules
  text: '# Shield You gain the Armor Class benefit of a Shield only if you have training with it.'
  sha256: 37deffd2fd7771ee9f8a246c906c58fbd2bc9eb7f668c216628c507b135c6af4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shield

<!-- SRD page 92 -->

You gain the Armor Class benefit of a Shield only if\
you have training with it.\

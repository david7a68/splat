---
splat_schema: 1
splat_id: 72da2e49-1dec-519e-b482-c87ebc9473c0
splat_kind: rule
splat_key: dnd2024:rules/magic_items_paired_items
splat_name: Paired Items
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_items_paired_items
  parameters:
    paired_item_benefits_require_both_worn: true
    mixed_pairs_grant_benefits: false
  implementation: dnd2024:rules_magic_items_paired_items
splat_contracts:
- id: dnd2024:rules_magic_items_paired_items
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
  - dnd2024:rules_magic_items_paired_items
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 47bed7a8535c4634b7b0be69ffd03d3fe0a4fd52f54d6036a9fc1c35031e8d63
  parent_name: Magic Items
  line_span:
    first: 8953
    count: 7
splat_clauses:
- key: source_rules
  text: '# Paired Items Items that come in pairs—such as boots, bracers, gauntlets, and gloves—impart
    their benefits only if both items of the pair are worn. For example, a character wearing a Boot of
    Striding and Springing on one foot and a Boot of Elvenkind on the other foot gains no benefit from
    either.'
  sha256: ade1c803404cac77254ae805260d2084610b1b24ace61abe876a752bd5914112
  status: contract_specified
splat_requires_mechanics: *id001
---

# Paired Items

<!-- SRD page 103 -->

Items that come in pairs—such as boots, bracers,\
gauntlets, and gloves—impart their benefits only\
if both items of the pair are worn. For example, a\
character wearing a Boot of Striding and Springing\
on one foot and a Boot of Elvenkind on the other foot\
gains no benefit from either.\

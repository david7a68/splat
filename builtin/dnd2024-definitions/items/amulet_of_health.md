---
splat_schema: 1
splat_id: f7747af0-db80-53aa-9416-85779ee7244c
splat_kind: item
splat_key: dnd2024:items/amulet_of_health
splat_name: Amulet of Health
splat_origin:
  key: dnd2024:reference/magic_item/amulet_of_health
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/amulet_of_health
  implementation: dnd2024:item_amulet_of_health
  parameters:
    equipment_state: worn
    ability:
      name: constitution
      set: 19
      only_if_unmodified_score_below: 19
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_amulet_of_health
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
  outputs:
  - modifiers
  events:
  - equipment.state_changed
  capabilities:
  - equipment
  - derived_statistics
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 853f56a2b80b2a9ba590a2a812758cdd88736d5ce9028ea4d24d103f581d1a82
  source_path: builtin/srd-5.2.1/magic_item/amulet_of_health.md
  pages:
  - 209
  engine_gaps: &id001
  - dnd2024:item_amulet_of_health
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 2a633ed418205161eb4a65df6a182c063ad9faf040be939ad0d6fe85cfac1899
splat_clauses:
- key: source_rules
  text: '# Amulet of Health Wondrous Item, Rare (Requires Attunement) Your Constitution is 19 while you
    wear this amulet. It has no effect on you if your Constitution is 19 or higher without it.'
  sha256: d7bdb0cca1b2a9a2ad8054f851722bdf222ef79210e6339ee64a5624b537974b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Amulet of Health

<!-- SRD page 209 -->

Wondrous Item, Rare (Requires Attunement)\
Your Constitution is 19 while you wear this amulet.\
It has no effect on you if your Constitution is 19 or\
higher without it.\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].

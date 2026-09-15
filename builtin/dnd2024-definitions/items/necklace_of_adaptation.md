---
splat_schema: 1
splat_id: bc54a8b3-ca8f-5c4d-bc4f-a54ec1222ff2
splat_kind: item
splat_key: dnd2024:items/necklace_of_adaptation
splat_name: Necklace of Adaptation
splat_origin:
  key: dnd2024:reference/magic_item/necklace_of_adaptation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 232
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/necklace_of_adaptation
  implementation: dnd2024:item_necklace_of_adaptation
  parameters:
    equipment_state: worn
    breathing: normal_in_any_environment
    saving_throws:
      advantage: true
      purpose:
      - avoid_poisoned
      - end_poisoned
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_necklace_of_adaptation
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
  source_sha256: 3918ccb8259c467b053fcc58c4888f9fca823f36858a2ecd6bcfb05cb188546e
  source_path: builtin/srd-5.2.1/magic_item/necklace_of_adaptation.md
  pages:
  - 232
  engine_gaps: &id001
  - dnd2024:item_necklace_of_adaptation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: dfd1c7022c339343e8a4f66d73b2d9827d8710bc87e6992e83f621e1ff6bea32
splat_clauses:
- key: source_rules
  text: '# Necklace of Adaptation Wondrous Item, Uncommon (Requires Attunement) While wearing this necklace,
    you can breathe normally in any environment, and you have Advantage on saving throws made to avoid
    or end the Poisoned condition.'
  sha256: d5acf34b6fedc857a2947c44e8bfedb1447d7db92cf1c8748f64155cdb62c4f5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Necklace of Adaptation

<!-- SRD page 232 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing this necklace, you can breathe nor-\
mally in any environment, and you have Advantage\
on saving throws made to avoid or end the Poisoned\
condition.\

Original layout: [[../../srd-5.2.1/pages/232|page 232]].

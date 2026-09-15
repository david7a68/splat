---
splat_schema: 1
splat_id: 8934f3f1-6941-500a-ac0d-1149bb97959b
splat_kind: item
splat_key: dnd2024:items/gem_of_seeing
splat_name: Gem of Seeing
splat_origin:
  key: dnd2024:reference/magic_item/gem_of_seeing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/gem_of_seeing
  implementation: dnd2024:item_gem_of_seeing
  parameters:
    charges:
      maximum: 3
      initial: 3
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 3
          bonus: 0
    activation:
      action: magic
      cost: 1
      duration:
        value: 10
        unit: minute
      truesight_ft: 120
      requires_during_effect: peer_through_gem
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_gem_of_seeing
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 026cb10ed90a7c8b536ea886011122cbb66bc868c0238e482fc95d0185f67126
  source_path: builtin/srd-5.2.1/magic_item/gem_of_seeing.md
  pages:
  - 223
  engine_gaps: &id001
  - dnd2024:item_gem_of_seeing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: ee207906007411f0d53f24380e2b9afab2f7a36bd0eaf8c515f3fd64a575f143
splat_clauses:
- key: source_rules
  text: '# Gem of Seeing Wondrous Item, Rare (Requires Attunement) This gem has 3 charges. As a Magic
    action, you can expend 1 charge. For the next 10 minutes, you have Truesight out to 120 feet when
    you peer through the gem. The gem regains 1d3 expended charges daily at dawn.'
  sha256: 9099ca44bf108e9541d222beb681e7506e0aa38b96b95d908bc170e88bd405f0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gem of Seeing

<!-- SRD page 223 -->

Wondrous Item, Rare (Requires Attunement)\
This gem has 3 charges. As a Magic action, you can\
expend 1 charge. For the next 10 minutes, you have\
Truesight out to 120 feet when you peer through the\
gem.\
The gem regains 1d3 expended charges daily at\
dawn.\

Original layout: [[../../srd-5.2.1/pages/223|page 223]].

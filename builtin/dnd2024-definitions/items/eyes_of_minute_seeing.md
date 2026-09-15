---
splat_schema: 1
splat_id: cb02a936-1e4b-53d6-9cb1-a396c3205902
splat_kind: item
splat_key: dnd2024:items/eyes_of_minute_seeing
splat_name: Eyes of Minute Seeing
splat_origin:
  key: dnd2024:reference/magic_item/eyes_of_minute_seeing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 221
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/eyes_of_minute_seeing
  implementation: dnd2024:item_eyes_of_minute_seeing
  parameters:
    equipment_state: worn
    darkvision_ft: 1
    investigation:
      advantage: true
      target_distance_ft: 1
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_eyes_of_minute_seeing
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
  source_sha256: ed96fa653f55d278636561eec7730a1939ae0bf952addfa00af40fd9e8887ccc
  source_path: builtin/srd-5.2.1/magic_item/eyes_of_minute_seeing.md
  pages:
  - 221
  engine_gaps: &id001
  - dnd2024:item_eyes_of_minute_seeing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: d738b914a9d513c60f5d1b0c71c87dba592fa6bb36d2fae5423542fce3a588a9
splat_clauses:
- key: source_rules
  text: '# Eyes of Minute Seeing Wondrous Item, Uncommon These crystal lenses fit over the eyes. While
    wearing them, your vision improves significantly out to a range of 1 foot, granting you Darkvision
    within that range and Advantage on Intelligence (Investigation) checks made to examine something within
    that range.'
  sha256: 5a487f153ca6149eeddbcdada88066465efee6ab0156d2b59053603bad31c8e0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Eyes of Minute Seeing

<!-- SRD page 221 -->

Wondrous Item, Uncommon\
These crystal lenses fit over the eyes. While wear-\
ing them, your vision improves significantly out to a\
range of 1 foot, granting you Darkvision within that\
range and Advantage on Intelligence (Investigation)\
checks made to examine something within that\
range.\

Original layout: [[../../srd-5.2.1/pages/221|page 221]].

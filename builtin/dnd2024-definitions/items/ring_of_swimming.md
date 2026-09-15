---
splat_schema: 1
splat_id: 0b509d42-d42f-5046-9d89-7acb9cc4312b
splat_kind: item
splat_key: dnd2024:items/ring_of_swimming
splat_name: Ring of Swimming
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_swimming
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/ring_of_swimming
  implementation: dnd2024:item_ring_of_swimming
  parameters:
    equipment_state: worn_on_finger_or_similar_digit
    swim_speed_ft: 40
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_rings
splat_contracts:
- id: dnd2024:item_ring_of_swimming
  inputs:
    actor:
      type: participant
      required: true
    source_item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choice:
      required: false
      type: choice
    event:
      required: false
      type: event
    campaign:
      required: false
      type: campaign
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - equipment.changed
  capabilities:
  - inventory
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 30071db96f2cc3088431df00b9a3c2eed4ddd0535b60e53226afb702cb147909
  source_path: builtin/srd-5.2.1/magic_item/ring_of_swimming.md
  pages:
  - 239
  engine_gaps: &id001
  - dnd2024:item_ring_of_swimming
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 8f9c1bb57fec539f31b28bde89b0feb1ccbb5cc1bff8e96ccb4696dd574e44ed
splat_clauses:
- key: source_rules
  text: '# Ring of Swimming Ring, Uncommon You have a Swim Speed of 40 feet while wearing this ring.'
  sha256: 3b8e410aa3a9858ca2d34fc334b4ed09780f136539d6169f3f67a362599d14f3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Swimming

<!-- SRD page 239 -->

Ring, Uncommon\
You have a Swim Speed of 40 feet while wearing this\
ring.\

Original layout: [[../../srd-5.2.1/pages/239|page 239]].

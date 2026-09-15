---
splat_schema: 1
splat_id: 1def14bc-dd94-5aed-aa96-1603e2e9b7d5
splat_kind: item
splat_key: dnd2024:items/stone_of_good_luck_luckstone
splat_name: Stone of Good Luck (Luckstone)
splat_origin:
  key: dnd2024:reference/magic_item/stone_of_good_luck_luckstone
splat_sources:
- title: SRD 5.2.1
  locator: pp. 248
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/stone_of_good_luck_luckstone
  implementation: dnd2024:item_stone_of_good_luck_luckstone
  parameters:
    equipment_state: on_person
    ability_check_bonus: 1
    saving_throw_bonus: 1
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_stone_of_good_luck_luckstone
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
  - check.pending
  - saving_throw.pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 497adb2a060e1a4b3f7bf5ee030a74f9e161e80f4309f8d0f1dfb63c331862ae
  source_path: builtin/srd-5.2.1/magic_item/stone_of_good_luck_luckstone.md
  pages:
  - 248
  engine_gaps: &id001
  - dnd2024:item_stone_of_good_luck_luckstone
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 46dbed55cfa09926f4d5b6aa76aa7bff75ad0d94240a52ae5248a5dcfc3bf9ac
splat_clauses:
- key: source_rules
  text: '# Stone of Good Luck (Luckstone) Wondrous Item, Uncommon (Requires Attunement) While this polished
    agate is on your person, you gain a +1 bonus to ability checks and saving throws.'
  sha256: 19967ad92ae45459cf97fa41dd385651df4c562be8dd309f2e27c3e7b6da81c6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Stone of Good Luck (Luckstone)

<!-- SRD page 248 -->

Wondrous Item, Uncommon (Requires Attunement)\
While this polished agate is on your person, you\
gain a +1 bonus to ability checks and saving throws.\

Original layout: [[../../srd-5.2.1/pages/248|page 248]].

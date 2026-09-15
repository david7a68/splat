---
splat_schema: 1
splat_id: 0a152959-c402-533a-ab75-eabcaf59b4fa
splat_kind: item
splat_key: dnd2024:items/boots_of_striding_and_springing
splat_name: Boots of Striding and Springing
splat_origin:
  key: dnd2024:reference/magic_item/boots_of_striding_and_springing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/boots_of_striding_and_springing
  implementation: dnd2024:item_boots_of_striding_and_springing
  parameters:
    equipment_state: worn
    speed_minimum_ft: 30
    ignore_speed_reductions_from:
    - excess_carried_weight
    - heavy_armor
    jump:
      uses_per_own_turn: 1
      maximum_ft: 30
      movement_cost_ft: 10
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_boots_of_striding_and_springing
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
  - movement.jump_pending
  - turn.started
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 2e25759f596af19d3b6f92168b2654f1a371303e128345839053382414c7f4a8
  source_path: builtin/srd-5.2.1/magic_item/boots_of_striding_and_springing.md
  pages:
  - 214
  engine_gaps: &id001
  - dnd2024:item_boots_of_striding_and_springing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 579b81c6ee9958cf3bc97aa1a5a56549a3227447ccc3219c18054e4ddaf2622c
splat_clauses:
- key: source_rules
  text: '# Boots of Striding and Springing Wondrous Item, Uncommon (Requires Attunement) While you wear
    these boots, your Speed becomes 30 feet unless your Speed is higher, and your Speed isn’t reduced
    by you carrying weight in excess of your carrying capacity or wearing Heavy Armor. Once on each of
    your turns, you can jump up to 30 feet by spending only 10 feet of movement.'
  sha256: 1f7f2eaabf0a06c36c1e6c08ad9c9480a443c2b25b17da890f887d2fd0f04955
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boots of Striding and Springing

<!-- SRD page 214 -->

Wondrous Item, Uncommon (Requires Attunement)\
While you wear these boots, your Speed becomes\
30 feet unless your Speed is higher, and your Speed\
isn’t reduced by you carrying weight in excess of\
your carrying capacity or wearing Heavy Armor.\
Once on each of your turns, you can jump up to 30\
feet by spending only 10 feet of movement.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].

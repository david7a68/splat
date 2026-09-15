---
splat_schema: 1
splat_id: 284a950f-5282-5b29-9200-ab6307b8ccf3
splat_kind: item
splat_key: dnd2024:items/gauntlets_of_ogre_power
splat_name: Gauntlets of Ogre Power
splat_origin:
  key: dnd2024:reference/magic_item/gauntlets_of_ogre_power
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/gauntlets_of_ogre_power
  implementation: dnd2024:item_gauntlets_of_ogre_power
  parameters:
    equipment_state: worn
    strength:
      set: 19
      only_if_unmodified_less_than: 19
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_gauntlets_of_ogre_power
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
  - ability.changed
  - attunement.changed
  capabilities:
  - inventory
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: afc0a7e2e341c8dc9a69925e7e80b7800abed338d0ec0c8166251641cd311065
  source_path: builtin/srd-5.2.1/magic_item/gauntlets_of_ogre_power.md
  pages:
  - 223
  engine_gaps: &id001
  - dnd2024:item_gauntlets_of_ogre_power
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: dc990a5dc7e1669f489b4191c59fc3036ca3ec2b7c8ed044e4cce314a76fbe34
splat_clauses:
- key: source_rules
  text: '# Gauntlets of Ogre Power Wondrous Item, Uncommon (Requires Attunement) Your Strength is 19 while
    you wear these gauntlets. They have no effect on you if your Strength is 19 or higher without them.'
  sha256: b49908ecc651ee2853cc972d1d4ea23dc882ff71f4817d777df24d5890febc99
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gauntlets of Ogre Power

<!-- SRD page 223 -->

Wondrous Item, Uncommon (Requires Attunement)\
Your Strength is 19 while you wear these gauntlets.\
They have no effect on you if your Strength is 19 or\
higher without them.\

Original layout: [[../../srd-5.2.1/pages/223|page 223]].

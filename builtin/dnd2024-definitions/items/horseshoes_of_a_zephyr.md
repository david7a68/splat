---
splat_schema: 1
splat_id: 1080cfe9-ff7f-51a3-8f7b-128ae97ccd80
splat_kind: item
splat_key: dnd2024:items/horseshoes_of_a_zephyr
splat_name: Horseshoes of a Zephyr
splat_origin:
  key: dnd2024:reference/magic_item/horseshoes_of_a_zephyr
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/horseshoes_of_a_zephyr
  implementation: dnd2024:item_horseshoes_of_a_zephyr
  parameters:
    set_size: 4
    affix:
      action: magic
      target: hoof
      eligible_creatures:
      - horse
      - similar_creature
    remove:
      action: magic
    while_all_four_affixed:
      float_height_inches: 4
      normal_movement: true
      surface_support_required: false
      surface_examples:
      - water
      - lava
      leaves_tracks: false
      ignore_difficult_terrain: true
      daily_travel_without_extended_travel_exhaustion_hours: 12
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_horseshoes_of_a_zephyr
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
  - action.magic_pending
  - movement.pending
  - travel.elapsed
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - spatial_state
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 2c71446f0bbd1b47466459319d9398f0439963748ea016a7342927f919d32980
  source_path: builtin/srd-5.2.1/magic_item/horseshoes_of_a_zephyr.md
  pages:
  - 226
  engine_gaps: &id001
  - dnd2024:item_horseshoes_of_a_zephyr
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 849e2adf5ea1f26ca14197372c5674a988197706f778d6b38423c4b6df2a3e86
splat_clauses:
- key: source_rules
  text: '# Horseshoes of a Zephyr Wondrous Item, Very Rare These horseshoes come in a set of four. As
    a Magic action, you can touch one of the horseshoes to the hoof of a horse or similar creature, whereupon
    the horseshoe affixes itself to the hoof. Removing a horseshoe also takes a Magic action. While all
    four shoes are affixed to the hooves of a horse or similar creature, they allow the creature to move
    normally while floating 4 inches above a surface. This effect means the creature can cross or stand
    above nonsolid or unstable surfaces, such as water or lava. The creature leaves no tracks and ignores
    Difficult Terrain. In addition, the creature can travel for up to 12 hours a day without gaining Exhaustion
    levels from extended travel.'
  sha256: 08b5b5f981d79ad7f1a11bb92ee384e6a48a6a661bd1db24ad729b3285e5d421
  status: contract_specified
splat_requires_mechanics: *id001
---

# Horseshoes of a Zephyr

<!-- SRD page 226 -->

Wondrous Item, Very Rare\
These horseshoes come in a set of four. As a Magic\
action, you can touch one of the horseshoes to the\
hoof of a horse or similar creature, whereupon the\
horseshoe affixes itself to the hoof. Removing a\
horseshoe also takes a Magic action.\
While all four shoes are affixed to the hooves of a\
horse or similar creature, they allow the creature\
to move normally while floating 4 inches above a\
surface. This effect means the creature can cross\
or stand above nonsolid or unstable surfaces, such\
as water or lava. The creature leaves no tracks and\
ignores Difficult Terrain. In addition, the creature\
can travel for up to 12 hours a day without gaining\
Exhaustion levels from extended travel.\

Original layout: [[../../srd-5.2.1/pages/226|page 226]].

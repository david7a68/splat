---
splat_schema: 1
splat_id: 5c5a485f-c32d-5032-b88d-2e293d44ebcf
splat_kind: item
splat_key: dnd2024:items/ring_of_x_ray_vision
splat_name: Ring of X-ray Vision
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_x_ray_vision
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239, 240
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_x_ray_vision
  implementation: dnd2024:item_ring_of_x_ray_vision
  parameters:
    equipment_state: worn_on_finger_or_similar_digit
    activate:
      action: magic
      duration:
        amount: 1
        unit: minute
      range_ft: 30
      objects_appear_transparent: true
      light_passes_through_for_user_vision: true
      penetration_limits:
      - material: stone
        thickness_inches: 12
      - material: common_metal
        thickness_inches: 1
      - material: wood
        thickness_inches: 36
      - material: dirt
        thickness_inches: 36
      blocked_by:
      - thicker_substances
      - any_sheet_of_lead
    additional_uses_before_long_rest:
      save:
        ability: constitution
        dc: 15
        failure:
          exhaustion_increase: 1
        success:
          effect: none
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_rings
splat_contracts:
- id: dnd2024:item_ring_of_x_ray_vision
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
  - time.elapsed
  - rest.completed
  - perception.pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 917a499aa6a2d023dbe19246304f20a5b27ea88fbc7fc17e2328c103ab880525
  source_path: builtin/srd-5.2.1/magic_item/ring_of_x_ray_vision.md
  pages:
  - 239
  - 240
  engine_gaps: &id001
  - dnd2024:item_ring_of_x_ray_vision
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 40f42d4eea1e312f966b35e3d54286af8919304139d9ce2ea19edd0494fd795c
splat_clauses:
- key: source_rules
  text: '# Ring of X-ray Vision Ring, Rare (Requires Attunement) While wearing this ring, you can take
    a Magic action to gain X-ray vision with a range of 30 feet for 1 minute. To you, solid objects within
    that radius appear transparent and don’t prevent light from passing through them. The vision can penetrate
    1 foot of stone, 1 inch of common metal, or up to 3 feet of wood or dirt. Thicker substances or a
    thin sheet of lead block the vision. Whenever you use the ring again before taking a Long Rest, you
    must succeed on a DC 15 Constitution saving throw or gain 1 Exhaustion level.'
  sha256: ec6c31b54038af372f2e890a951248be098018576c8f3ccc1e6af60a01b8064f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of X-ray Vision

<!-- SRD page 239 -->

Ring, Rare (Requires Attunement)\
While wearing this ring, you can take a Magic ac-\
tion to gain X-ray vision with a range of 30 feet for\
1 minute. To you, solid objects within that radius\
appear transparent and don’t prevent light from\
passing through them. The vision can penetrate 1\

<!-- SRD page 240 -->

foot of stone, 1 inch of common metal, or up to 3 feet\
of wood or dirt. Thicker substances or a thin sheet\
of lead block the vision.\
Whenever you use the ring again before taking a\
Long Rest, you must succeed on a DC 15 Constitu-\
tion saving throw or gain 1 Exhaustion level.\

Original layout: [[../../srd-5.2.1/pages/239|page 239]], [[../../srd-5.2.1/pages/240|page 240]].

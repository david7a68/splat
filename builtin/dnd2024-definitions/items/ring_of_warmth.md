---
splat_schema: 1
splat_id: 93e59450-81e0-5b89-b4e4-50ba170b8500
splat_kind: item
splat_key: dnd2024:items/ring_of_warmth
splat_name: Ring of Warmth
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_warmth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_warmth
  implementation: dnd2024:item_ring_of_warmth
  parameters:
    equipped: worn
    damage_reduction:
      type: cold
      amount:
        roll:
          count: 2
          sides: 8
      minimum_final_damage: 0
    temperature_protection:
      fahrenheit_maximum: 0
      lower_limit: null
      targets:
      - wearer
      - worn_items
      - carried_items
    attunement_required: true
splat_contracts:
- id: dnd2024:item_ring_of_warmth
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - damage.pending
  - temperature.exposure
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 301df3d3ae100052fabf20f66e80167586823655d253fd967b40386d95bf6da0
  source_path: builtin/srd-5.2.1/magic_item/ring_of_warmth.md
  pages:
  - 239
  engine_gaps: &id001
  - dnd2024:item_ring_of_warmth
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 1eb7646eb69e4cd80dd035380ca574927692a3586b748ba6b2468d752b9b0d26
splat_clauses:
- key: source_rules
  text: '# Ring of Warmth Ring, Uncommon (Requires Attunement) If you take Cold damage while wearing this
    ring, the ring reduces the damage you take by 2d8. In addition, while wearing this ring, you and everything
    you wear and carry are unharmed by temperatures of 0 degrees Fahrenheit or lower.'
  sha256: 117d52efcf9da25a0a26866c070e941802cde0d0709bcc6e102aa7b906fbf177
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Warmth

<!-- SRD page 239 -->

Ring, Uncommon (Requires Attunement)\
If you take Cold damage while wearing this ring, the\
ring reduces the damage you take by 2d8.\
In addition, while wearing this ring, you and\
everything you wear and carry are unharmed by\
temperatures of 0 degrees Fahrenheit or lower.\

Original layout: [[../../srd-5.2.1/pages/239|page 239]].

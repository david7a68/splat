---
splat_schema: 1
splat_id: 836b656b-3b3d-56d0-88af-f9c9a42b32ed
splat_kind: item
splat_key: dnd2024:items/ring_of_invisibility
splat_name: Ring of Invisibility
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_invisibility
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_invisibility
  implementation: dnd2024:item_ring_of_invisibility
  parameters:
    equipment_state: worn_on_finger_or_similar_digit
    activate:
      action: magic
      condition: invisible
      target: self
    ends_on:
    - ring_removed
    - user_bonus_action_end
    attack_or_cast_ends_effect: false
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_rings
splat_contracts:
- id: dnd2024:item_ring_of_invisibility
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
  - action.bonus_pending
  - equipment.changed
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 157930c6672513077794765efc8e2b192f791c88063768b5d00c508fd4b60b01
  source_path: builtin/srd-5.2.1/magic_item/ring_of_invisibility.md
  pages:
  - 238
  engine_gaps: &id001
  - dnd2024:item_ring_of_invisibility
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 8f9e8a185920560fc1ca164c46dc3e4800e3c6569335c31e2a3ce524df52ad02
splat_clauses:
- key: source_rules
  text: '# Ring of Invisibility Ring, Legendary (Requires Attunement) While wearing this ring, you can
    take a Magic action to give yourself the Invisible condition. You remain Invisible until the ring
    is removed or until you take a Bonus Action to become visible again.'
  sha256: f3ed31f0763f47bdc1e01f7a1cd0ff232851e31a544417e4c1c043a00e11db0b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Invisibility

<!-- SRD page 238 -->

Ring, Legendary (Requires Attunement)\
While wearing this ring, you can take a Magic action\
to give yourself the Invisible condition. You remain\
Invisible until the ring is removed or until you take\
a Bonus Action to become visible again.\

Original layout: [[../../srd-5.2.1/pages/238|page 238]].

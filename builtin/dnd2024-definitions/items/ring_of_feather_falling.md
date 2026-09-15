---
splat_schema: 1
splat_id: 53b73aa5-d0ca-5d4d-a52f-4c0f65fe3cb7
splat_kind: item
splat_key: dnd2024:items/ring_of_feather_falling
splat_name: Ring of Feather Falling
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_feather_falling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_feather_falling
  implementation: dnd2024:item_ring_of_feather_falling
  parameters:
    equipped: worn
    trigger: falling
    descent_ft_per_round: 60
    fall_damage: 0
    attunement_required: true
splat_contracts:
- id: dnd2024:item_ring_of_feather_falling
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
  - fall.started
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: e5f8efdb8db6c04c06f953c4be2a75d85e69579653aa16311b92d78d611bb33b
  source_path: builtin/srd-5.2.1/magic_item/ring_of_feather_falling.md
  pages:
  - 238
  engine_gaps: &id001
  - dnd2024:item_ring_of_feather_falling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 5b2b30c269a61ea1e7a1f92739a62edb4620cca17ee6e6f3e3c8368d7ba86097
splat_clauses:
- key: source_rules
  text: '# Ring of Feather Falling Ring, Rare (Requires Attunement) When you fall while wearing this ring,
    you descend 60 feet per round and take no damage from falling.'
  sha256: 44657f7eb729ad26ee28caa21213d711712ed5616c259886dc59296d510fd424
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Feather Falling

<!-- SRD page 238 -->

Ring, Rare (Requires Attunement)\
When you fall while wearing this ring, you descend\
60 feet per round and take no damage from falling.\

Original layout: [[../../srd-5.2.1/pages/238|page 238]].

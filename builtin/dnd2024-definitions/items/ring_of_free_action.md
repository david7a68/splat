---
splat_schema: 1
splat_id: 8780760b-3ed7-5057-8cc1-336f263c3fe4
splat_kind: item
splat_key: dnd2024:items/ring_of_free_action
splat_name: Ring of Free Action
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_free_action
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
  mechanic: dnd2024:items/ring_of_free_action
  implementation: dnd2024:item_ring_of_free_action
  parameters:
    equipment_state: worn
    difficult_terrain_extra_cost: 0
    magic_prevention:
      speed_reductions: all_speeds
      conditions:
      - paralyzed
      - restrained
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_ring_of_free_action
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
  source_sha256: 64e1620f9d44aaeae8a99b703393c3f80b5b46d32580265086334039fa585aff
  source_path: builtin/srd-5.2.1/magic_item/ring_of_free_action.md
  pages:
  - 238
  engine_gaps: &id001
  - dnd2024:item_ring_of_free_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: bd9e4b68a767e95c40785b94e0d705271ea36363e9e537f4be76af697ca3ef97
splat_clauses:
- key: source_rules
  text: '# Ring of Free Action Ring, Rare (Requires Attunement) While you wear this ring, Difficult Terrain
    doesn’t cost you extra movement. In addition, magic can neither reduce any of your Speeds nor cause
    you to have the Paralyzed or Restrained condition.'
  sha256: 305e90c2e1390b0221c2613242f67dc879982a71c4b23a38ab67e4e1a38bf388
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Free Action

<!-- SRD page 238 -->

Ring, Rare (Requires Attunement)\
While you wear this ring, Difficult Terrain doesn’t\
cost you extra movement. In addition, magic can\
neither reduce any of your Speeds nor cause you to\
have the Paralyzed or Restrained condition.\

Original layout: [[../../srd-5.2.1/pages/238|page 238]].

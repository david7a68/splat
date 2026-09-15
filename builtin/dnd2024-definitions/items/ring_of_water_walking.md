---
splat_schema: 1
splat_id: a74684b9-44a9-5787-93cb-114328f7a365
splat_kind: item
splat_key: dnd2024:items/ring_of_water_walking
splat_name: Ring of Water Walking
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_water_walking
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
  mechanic: dnd2024:items/ring_of_water_walking
  implementation: dnd2024:item_ring_of_water_walking
  parameters:
    equipment_state: worn
    spells:
    - spell:
        key: dnd2024:spells/water_walk
      targets: self_only
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_ring_of_water_walking
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
  outputs:
  - effects
  - item_state
  events:
  - item.used
  - spell.cast_completed
  capabilities:
  - equipment
  - item_resources
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 79b1fecbcafcf59bd80b52127eeb6184345319ff4718fefe8996e8f9d97c18ae
  source_path: builtin/srd-5.2.1/magic_item/ring_of_water_walking.md
  pages:
  - 239
  engine_gaps: &id001
  - dnd2024:item_ring_of_water_walking
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 2459752b990809cc87722e3f845ba5107a6e1e3b827c632acf17df055fc02e96
splat_clauses:
- key: source_rules
  text: '# Ring of Water Walking Ring, Uncommon While wearing this ring, you cast Water Walk from it,
    targeting only yourself.'
  sha256: d9bcf2d9867e7839b38cc13841fcd2b0066415372544228691e7f52d816fb37a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Water Walking

<!-- SRD page 239 -->

Ring, Uncommon\
While wearing this ring, you cast Water Walk from\
it, targeting only yourself.\

Original layout: [[../../srd-5.2.1/pages/239|page 239]].

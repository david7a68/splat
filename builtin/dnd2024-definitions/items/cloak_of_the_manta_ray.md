---
splat_schema: 1
splat_id: c02fe88c-e434-5de9-aa45-c00a1c9b376d
splat_kind: item
splat_key: dnd2024:items/cloak_of_the_manta_ray
splat_name: Cloak of the Manta Ray
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_the_manta_ray
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/cloak_of_the_manta_ray
  implementation: dnd2024:item_cloak_of_the_manta_ray
  parameters:
    equipment_state: worn
    breathing:
    - underwater
    swim_speed_ft: 60
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_cloak_of_the_manta_ray
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
  source_sha256: 5f483e19b00d619e19d7d3b1f86ab64d90dfeb4c6974f0f222e52c513b1ffb2a
  source_path: builtin/srd-5.2.1/magic_item/cloak_of_the_manta_ray.md
  pages:
  - 216
  engine_gaps: &id001
  - dnd2024:item_cloak_of_the_manta_ray
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: eb9b3df538652a1292366759491ce630d2d69cb21df0461ffb99b91215738926
splat_clauses:
- key: source_rules
  text: '# Cloak of the Manta Ray Wondrous Item, Uncommon (Requires Attunement) While wearing this cloak,
    you can breathe underwater, and you have a Swim Speed of 60 feet.'
  sha256: 55843b12221d4c4f8ceb4d91e7fcb56fb69e780c3bdad0fdeda9fd1772fd91c1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cloak of the Manta Ray

<!-- SRD page 216 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing this cloak, you can breathe underwa-\
ter, and you have a Swim Speed of 60 feet.\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].

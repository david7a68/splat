---
splat_schema: 1
splat_id: f05f68f5-9772-5bed-b198-67bbbb36b06d
splat_kind: item
splat_key: dnd2024:equipment/mirror_5_gp
splat_name: Mirror (5 GP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 5
    unit: gp
  mass:
    value: 0.5
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/mirror_5_gp
  implementation: dnd2024:equipment_mirror_5_gp
  parameters:
    material: steel
    held: true
    operations:
    - personal_cosmetics
    - peek_around_corner
    - reflect_light_signal
    mass:
      value: 0.5
      unit: lb
splat_contracts:
- id: dnd2024:equipment_mirror_5_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
  outputs:
  - item_properties
  - conditional_modifiers
  events: []
  capabilities:
  - equipment
  - conditional_modifiers
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 98
  engine_gaps: &id001
  - dnd2024:equipment_mirror_5_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 2b57634215776c4b42ca3c26ec4113ca759e884f25069a093a5be77294e28303
  line_span:
    first: 8512
    count: 4
splat_clauses:
- key: source_rules
  text: '# Mirror (5 GP) A handheld steel Mirror is useful for personal cosmetics but also for peeking
    around corners and reflecting light as a signal.'
  sha256: f48d1bb6b613bbc3a03149ea1b09b9557e0f1f1232d39383a34e82fea7ffb674
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mirror (5 GP)

<!-- SRD page 98 -->

A handheld steel Mirror is useful for personal cos-\
metics but also for peeking around corners and re-\
flecting light as a signal.\

---
splat_schema: 1
splat_id: 8896c1c7-da50-51bf-968b-824a1f2c935e
splat_kind: item
splat_key: dnd2024:equipment/chest_5_gp
splat_name: Chest (5 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 5
    unit: gp
  mass:
    value: 25.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/chest_5_gp
  implementation: dnd2024:equipment_chest_5_gp
  parameters:
    capacity:
      volume_ft3: 12
    mass:
      value: 25.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_chest_5_gp
  inputs:
    item:
      type: item_instance
      required: true
    contents:
      required: false
      type: item_list
  outputs:
  - capacity_constraints
  events: []
  capabilities:
  - containers
  - inventory
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 97
  engine_gaps: &id001
  - dnd2024:equipment_chest_5_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: f783767dd257ec1f26a995027f8a59120e09f93cfca1faf8a24cc6208f727195
  line_span:
    first: 8349
    count: 2
splat_clauses:
- key: source_rules
  text: '# Chest (5 GP) A Chest holds up to 12 cubic feet of contents.'
  sha256: e0758d889e776288f3cb4f3b313d3070b8d856f34cf12e84026a59da1741c2e8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Chest (5 GP)

<!-- SRD page 97 -->

A Chest holds up to 12 cubic feet of contents.\

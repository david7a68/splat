---
splat_schema: 1
splat_id: 52cf9346-aa60-5ea2-9832-fc53e1b52533
splat_kind: item
splat_key: dnd2024:equipment/jug_2_cp
splat_name: Jug (2 CP)
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
    value: 2
    unit: cp
  mass:
    value: 4.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/jug_2_cp
  implementation: dnd2024:equipment_jug_2_cp
  parameters:
    capacity:
      liquid_gallons: 1
    mass:
      value: 4.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_jug_2_cp
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
  - 98
  engine_gaps: &id001
  - dnd2024:equipment_jug_2_cp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 0747ef6a8b1343e61274527b8095eef2d4e32312c35fb78ed3ef09f94e848ff7
  line_span:
    first: 8463
    count: 2
splat_clauses:
- key: source_rules
  text: '# Jug (2 CP) A Jug holds up to 1 gallon.'
  sha256: 93c0297425c780b15c03ebc97316489961553ec547cb5faa6a0b2e02c5f93565
  status: contract_specified
splat_requires_mechanics: *id001
---

# Jug (2 CP)

<!-- SRD page 98 -->

A Jug holds up to 1 gallon.\

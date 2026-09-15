---
splat_schema: 1
splat_id: 8d9f28f9-1de3-5ee8-976b-9f522c93ecee
splat_kind: item
splat_key: dnd2024:equipment/flask_2_cp
splat_name: Flask (2 CP)
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
    value: 2
    unit: cp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/flask_2_cp
  implementation: dnd2024:equipment_flask_2_cp
  parameters:
    capacity:
      liquid_pints: 1
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_flask_2_cp
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
  - dnd2024:equipment_flask_2_cp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 608ae9a9ee0761dc770e23f66c2b88db9b6c9e7f81a7825f04a36b6f90cc0e3f
  line_span:
    first: 8407
    count: 2
splat_clauses:
- key: source_rules
  text: '# Flask (2 CP) A Flask holds up to 1 pint.'
  sha256: 95214ef78bdad072b92e001dc0e0eca4a97076b9ac5b2053b6b45f2454376bbe
  status: contract_specified
splat_requires_mechanics: *id001
---

# Flask (2 CP)

<!-- SRD page 97 -->

A Flask holds up to 1 pint.\

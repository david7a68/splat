---
splat_schema: 1
splat_id: 07c76721-91c9-5787-98bf-edaaa94a351a
splat_kind: item
splat_key: dnd2024:equipment/bucket_5_cp
splat_name: Bucket (5 CP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 5
    unit: cp
  mass:
    value: 2.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/bucket_5_cp
  implementation: dnd2024:equipment_bucket_5_cp
  parameters:
    capacity:
      volume_ft3: 0.5
    mass:
      value: 2.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_bucket_5_cp
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
  - 96
  engine_gaps: &id001
  - dnd2024:equipment_bucket_5_cp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: e080a2b14480578ec89fcc16f46e39773e02bcdaca5d61bcaed954d0738d5c4d
  line_span:
    first: 8315
    count: 2
splat_clauses:
- key: source_rules
  text: '# Bucket (5 CP) A Bucket holds up to half a cubic foot of contents.'
  sha256: e980a48d8ed6b2a4b322b41721c8ce4f79af578c97bc90b4fe83127d700b8076
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bucket (5 CP)

<!-- SRD page 96 -->

A Bucket holds up to half a cubic foot of contents.\

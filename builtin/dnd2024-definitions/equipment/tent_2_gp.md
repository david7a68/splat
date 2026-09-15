---
splat_schema: 1
splat_id: a86bb862-32fd-5f90-8086-40ae456a5d04
splat_kind: item
splat_key: dnd2024:equipment/tent_2_gp
splat_name: Tent (2 GP)
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 2
    unit: gp
  mass:
    value: 20.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/tent_2_gp
  implementation: dnd2024:equipment_tent_2_gp
  parameters:
    sleep_capacity:
      count: 2
      sizes:
      - small
      - medium
    mass:
      value: 20.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_tent_2_gp
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
  - 100
  engine_gaps: &id001
  - dnd2024:equipment_tent_2_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: a932c6338442863d7cfdadaeece3dab9d393616c09b0f5531d0856b6a1d37e21
  line_span:
    first: 8662
    count: 2
splat_clauses:
- key: source_rules
  text: '# Tent (2 GP) A Tent sleeps up to two Small or Medium creatures.'
  sha256: 0ef5e5b6353c03474970113c62f11d4f2186d285db8ed116dc06d351b8f3835f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tent (2 GP)

<!-- SRD page 100 -->

A Tent sleeps up to two Small or Medium creatures.\

---
splat_schema: 1
splat_id: c8767c21-bf3b-55b2-99f2-35a0b190a8d2
splat_kind: item
splat_key: dnd2024:equipment/quiver_1_gp
splat_name: Quiver (1 GP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 1
    unit: gp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/quiver_1_gp
  implementation: dnd2024:equipment_quiver_1_gp
  parameters:
    capacity:
      ammunition_variant: arrows
      count: 20
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_quiver_1_gp
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
  - 99
  engine_gaps: &id001
  - dnd2024:equipment_quiver_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 190101e65e4a39bf3a9b9e8b0ebe162e4367b4380ec7649a10bf996edb71b5c4
  line_span:
    first: 8597
    count: 2
splat_clauses:
- key: source_rules
  text: '# Quiver (1 GP) A Quiver holds up to 20 Arrows.'
  sha256: 5ada848dc3cf88d75c6bd33f7f10bb69f3217ae690be8b1c93d57c4b6f429460
  status: contract_specified
splat_requires_mechanics: *id001
---

# Quiver (1 GP)

<!-- SRD page 99 -->

A Quiver holds up to 20 Arrows.\

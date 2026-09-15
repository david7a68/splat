---
splat_schema: 1
splat_id: 57e326bc-416a-562d-ad97-44af4f06202e
splat_kind: item
splat_key: dnd2024:equipment/pot_iron_2_gp
splat_name: Pot, Iron (2 GP)
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
    value: 2
    unit: gp
  mass:
    value: 10.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/pot_iron_2_gp
  implementation: dnd2024:equipment_pot_iron_2_gp
  parameters:
    capacity:
      liquid_gallons: 1
    mass:
      value: 10.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_pot_iron_2_gp
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
  - dnd2024:equipment_pot_iron_2_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 2197e35f9aee0774239d96785d2ae8cdfa3e3aa969b3c4765a0f3346d450d432
  line_span:
    first: 8582
    count: 2
splat_clauses:
- key: source_rules
  text: '# Pot, Iron (2 GP) An Iron Pot holds up to 1 gallon.'
  sha256: 62680956babd8bd1f96c3aadc5429c3fa81373ef1829768597c50910f7920a20
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pot, Iron (2 GP)

<!-- SRD page 99 -->

An Iron Pot holds up to 1 gallon.\

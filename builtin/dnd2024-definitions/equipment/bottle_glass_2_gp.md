---
splat_schema: 1
splat_id: d4189637-d8d4-525f-ae00-4155c9137439
splat_kind: item
splat_key: dnd2024:equipment/bottle_glass_2_gp
splat_name: Bottle, Glass (2 GP)
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
    value: 2
    unit: gp
  mass:
    value: 2.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/bottle_glass_2_gp
  implementation: dnd2024:equipment_bottle_glass_2_gp
  parameters:
    capacity:
      liquid_pints: 1.5
    mass:
      value: 2.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_bottle_glass_2_gp
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
  - dnd2024:equipment_bottle_glass_2_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: aeabcaa9b6cc8d5aa228cfdc0ca237dd0642f7fc155a17b5db43efbf7866024d
  line_span:
    first: 8313
    count: 2
splat_clauses:
- key: source_rules
  text: '# Bottle, Glass (2 GP) A Glass Bottle holds up to 11/2 pints.'
  sha256: 3f4d30f328530b190eb5cdfb57a1cc7524ec7525bc2fb0cd209fd70f71a96fbb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bottle, Glass (2 GP)

<!-- SRD page 96 -->

A Glass Bottle holds up to 11/2 pints.\

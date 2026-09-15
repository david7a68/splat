---
splat_schema: 1
splat_id: a102cd30-ca0e-5c7e-8276-e374c171bf82
splat_kind: item
splat_key: dnd2024:equipment/barrel_2_gp
splat_name: Barrel (2 GP)
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
    value: 70.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/barrel_2_gp
  implementation: dnd2024:equipment_barrel_2_gp
  parameters:
    capacity:
      alternative_capacities:
      - liquid_gallons: 40
      - dry_volume_ft3: 4
    mass:
      value: 70.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_barrel_2_gp
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
  - dnd2024:equipment_barrel_2_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 18d723dff95217cab56dc40626ebc774fd8b1dea78a9dafe5555e9737ee7e1d7
  line_span:
    first: 8288
    count: 3
splat_clauses:
- key: source_rules
  text: '# Barrel (2 GP) A Barrel holds up to 40 gallons of liquid or up to 4 cubic feet of dry goods.'
  sha256: 7a13d734ecaa7ec777c0cc04169d8821a16b33faf20df238fd578fa55bd42312
  status: contract_specified
splat_requires_mechanics: *id001
---

# Barrel (2 GP)

<!-- SRD page 96 -->

A Barrel holds up to 40 gallons of liquid or up to 4\
cubic feet of dry goods.\

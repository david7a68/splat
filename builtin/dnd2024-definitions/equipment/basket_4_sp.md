---
splat_schema: 1
splat_id: 4b1a2e62-71ea-57a5-8c4f-a9ba24482c67
splat_kind: item
splat_key: dnd2024:equipment/basket_4_sp
splat_name: Basket (4 SP)
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
    value: 4
    unit: sp
  mass:
    value: 2.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/basket_4_sp
  implementation: dnd2024:equipment_basket_4_sp
  parameters:
    capacity:
      mass_lb: 40
      volume_ft3: 2
    mass:
      value: 2.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_basket_4_sp
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
  - dnd2024:equipment_basket_4_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: cc007b2d37f23d7457f8e445a2d7edce3d8d1b83469b1041fd679eb14836047b
  line_span:
    first: 8291
    count: 2
splat_clauses:
- key: source_rules
  text: '# Basket (4 SP) A Basket holds up to 40 pounds within 2 cubic feet.'
  sha256: 2bbc743c4a629f18181a0d77d6c2e8367740d5949d79bb40f3ab3cbd7891dd54
  status: contract_specified
splat_requires_mechanics: *id001
---

# Basket (4 SP)

<!-- SRD page 96 -->

A Basket holds up to 40 pounds within 2 cubic feet.\

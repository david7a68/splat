---
splat_schema: 1
splat_id: 816f1aad-46fb-5329-a7ee-140b08dd8ba9
splat_kind: item
splat_key: dnd2024:equipment/case_crossbow_bolt_1_gp
splat_name: Case, Crossbow Bolt (1 GP)
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
    value: 1
    unit: gp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/case_crossbow_bolt_1_gp
  implementation: dnd2024:equipment_case_crossbow_bolt_1_gp
  parameters:
    capacity:
      ammunition_variant: bolts
      count: 20
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_case_crossbow_bolt_1_gp
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
  - dnd2024:equipment_case_crossbow_bolt_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 28f0a883b8573154cf56bd14f51d4989faf95cc96dfbcf7a38a0d98f3f2b4b94
  line_span:
    first: 8333
    count: 2
splat_clauses:
- key: source_rules
  text: '# Case, Crossbow Bolt (1 GP) A Crossbow Bolt Case holds up to 20 Bolts.'
  sha256: 44b6224547d71df2ccd5363ecdfd4a7a5e7decc65780e2f6b1ebccf5095ec927
  status: contract_specified
splat_requires_mechanics: *id001
---

# Case, Crossbow Bolt (1 GP)

<!-- SRD page 96 -->

A Crossbow Bolt Case holds up to 20 Bolts.\

---
splat_schema: 1
splat_id: 2f495087-e0fb-552c-9f6f-d587b25e9b14
splat_kind: item
splat_key: dnd2024:equipment/sack_1_cp
splat_name: Sack (1 CP)
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
    unit: cp
  mass:
    value: 0.5
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/sack_1_cp
  implementation: dnd2024:equipment_sack_1_cp
  parameters:
    capacity:
      mass_lb: 30
      volume_ft3: 1
    mass:
      value: 0.5
      unit: lb
splat_contracts:
- id: dnd2024:equipment_sack_1_cp
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
  - dnd2024:equipment_sack_1_cp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: ae06f9505ce13db8f158a0da4f4e9d7cac0e20f071834d5543ccc88e4492fbe8
  line_span:
    first: 8624
    count: 2
splat_clauses:
- key: source_rules
  text: '# Sack (1 CP) A Sack holds up to 30 pounds within 1 cubic foot.'
  sha256: 6896817e01b28d795b33f50bb2f69628fa2f0f835eb0177664c173aef5d0896f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sack (1 CP)

<!-- SRD page 99 -->

A Sack holds up to 30 pounds within 1 cubic foot.\

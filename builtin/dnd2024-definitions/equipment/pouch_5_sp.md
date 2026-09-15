---
splat_schema: 1
splat_id: b1b73d18-7615-589b-9425-3e74a3ec54c8
splat_kind: item
splat_key: dnd2024:equipment/pouch_5_sp
splat_name: Pouch (5 SP)
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
    value: 5
    unit: sp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/pouch_5_sp
  implementation: dnd2024:equipment_pouch_5_sp
  parameters:
    capacity:
      mass_lb: 6
      volume_ft3: 0.2
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_pouch_5_sp
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
  - dnd2024:equipment_pouch_5_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 3e53a58989e92ecaa193fe6655d274f7b8d029ce76a0ba1d52d0cb712b54702b
  line_span:
    first: 8590
    count: 3
splat_clauses:
- key: source_rules
  text: '# Pouch (5 SP) A Pouch holds up to 6 pounds within one-fifth of a cubic foot.'
  sha256: 94af527d10238b4a232346d3ffdb292576bc84db79ee1f0a47c47e4043d04442
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pouch (5 SP)

<!-- SRD page 99 -->

A Pouch holds up to 6 pounds within one-fifth of a\
cubic foot.\

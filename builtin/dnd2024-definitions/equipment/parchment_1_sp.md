---
splat_schema: 1
splat_id: d850e5db-ed29-5640-9e75-b7dabbb6c54d
splat_kind: item
splat_key: dnd2024:equipment/parchment_1_sp
splat_name: Parchment (1 SP)
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
    unit: sp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/parchment_1_sp
  implementation: dnd2024:equipment_parchment_1_sp
  parameters:
    quantity: 1
    unit: sheet
    writing_capacity:
      words: 250
      approximate: true
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_parchment_1_sp
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
  - 99
  engine_gaps: &id001
  - dnd2024:equipment_parchment_1_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: f45ffcf44185240aa61aab444f34386b091f2e3dcc6a715dcb72156cec19fa4c
  line_span:
    first: 8560
    count: 3
splat_clauses:
- key: source_rules
  text: '# Parchment (1 SP) One sheet of Parchment can hold about 250 handwritten words.'
  sha256: da906c3bbc7c39ff25f5d5ac4a70d7c53dea8a432652d58c1a5fb904e505f6ff
  status: contract_specified
splat_requires_mechanics: *id001
---

# Parchment (1 SP)

<!-- SRD page 99 -->

One sheet of Parchment can hold about 250 hand-\
written words.\

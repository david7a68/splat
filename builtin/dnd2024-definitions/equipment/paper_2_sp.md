---
splat_schema: 1
splat_id: de6aa2c5-dec2-585f-aa2f-6c1f41bd242e
splat_kind: item
splat_key: dnd2024:equipment/paper_2_sp
splat_name: Paper (2 SP)
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
    unit: sp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/paper_2_sp
  implementation: dnd2024:equipment_paper_2_sp
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
- id: dnd2024:equipment_paper_2_sp
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
  - dnd2024:equipment_paper_2_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 1987b2fa2ad7c738e84b9046c7a247989ffd947c342ab4612053c8529d3e351c
  line_span:
    first: 8557
    count: 3
splat_clauses:
- key: source_rules
  text: '# Paper (2 SP) One sheet of Paper can hold about 250 handwritten words.'
  sha256: bbfc7a7fbb2c5dd3a7aef185977c40c330a4ff584e2cbdfed4eaef764bedbfb6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Paper (2 SP)

<!-- SRD page 99 -->

One sheet of Paper can hold about 250 handwritten\
words.\

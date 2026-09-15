---
splat_schema: 1
splat_id: 725d861d-fce4-55f8-b9c5-b5416518d9a2
splat_kind: item
splat_key: dnd2024:equipment/ink_pen_2_cp
splat_name: Ink Pen (2 CP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 2
    unit: cp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/ink_pen_2_cp
  implementation: dnd2024:equipment_ink_pen_2_cp
  parameters:
    operations:
    - write
    - draw
    requires_item:
      key: dnd2024:equipment/ink_10_gp
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_ink_pen_2_cp
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
  - 98
  engine_gaps: &id001
  - dnd2024:equipment_ink_pen_2_cp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 86c056737558d1c13ea65f674917de01c839cac03c36086f2e8bf009820273a5
  line_span:
    first: 8461
    count: 2
splat_clauses:
- key: source_rules
  text: '# Ink Pen (2 CP) Using Ink, an Ink Pen is used to write or draw.'
  sha256: 0a2e232e84cb5cd0cc107c28528ffc72d6d5b9d05242e92d5a3f87e8eb052dd2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ink Pen (2 CP)

<!-- SRD page 98 -->

Using Ink, an Ink Pen is used to write or draw.\

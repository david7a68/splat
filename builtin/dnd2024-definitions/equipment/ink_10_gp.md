---
splat_schema: 1
splat_id: b7741792-13aa-5dc6-98c9-614ed3bd581c
splat_kind: item
splat_key: dnd2024:equipment/ink_10_gp
splat_name: Ink (10 GP)
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
    value: 10
    unit: gp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/ink_10_gp
  implementation: dnd2024:equipment_ink_10_gp
  parameters:
    container: bottle
    volume_ounces: 1
    writing_capacity:
      pages: 500
      approximate: true
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_ink_10_gp
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
  - dnd2024:equipment_ink_10_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 952e38f9f66784c121553818f802e771a40aceb1753f234f11750b92989dfbf3
  line_span:
    first: 8458
    count: 3
splat_clauses:
- key: source_rules
  text: '# Ink (10 GP) Ink comes in a 1-ounce bottle, which provides enough ink to write about 500 pages.'
  sha256: 47e2a1340d1f766d768a25edf04626befe1554e89ff8cb83dcea8b3afe3def74
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ink (10 GP)

<!-- SRD page 98 -->

Ink comes in a 1-ounce bottle, which provides\
enough ink to write about 500 pages.\

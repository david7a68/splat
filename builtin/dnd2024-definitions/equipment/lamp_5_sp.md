---
splat_schema: 1
splat_id: 0887b5a4-499c-5a15-85a4-c2c34f6e7b8d
splat_kind: item
splat_key: dnd2024:equipment/lamp_5_sp
splat_name: Lamp (5 SP)
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
    value: 5
    unit: sp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/lamp_5_sp
  implementation: dnd2024:equipment_lamp_5_sp
  parameters:
    light:
      requires_lit: true
      shape: radius
      bright_ft: 15
      additional_dim_ft: 30
    fuel:
      key: dnd2024:equipment/oil_1_sp
    hours_per_flask: 6
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_lamp_5_sp
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
  - dnd2024:equipment_lamp_5_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 8a7e7c99382e5540780e8e08c48c8dcdaed547c72d06361b471fa6275bd7ead3
  line_span:
    first: 8468
    count: 3
splat_clauses:
- key: source_rules
  text: '# Lamp (5 SP) A Lamp burns Oil as fuel to cast Bright Light in a 15- foot radius and Dim Light
    for an additional 30 feet.'
  sha256: 3c3470c04d5c9d7ef142cdca0c27c69813b09182f20110913945824355cea2fc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lamp (5 SP)

<!-- SRD page 98 -->

A Lamp burns Oil as fuel to cast Bright Light in a 15-\
foot radius and Dim Light for an additional 30 feet.\

---
splat_schema: 1
splat_id: 04fbeb09-cf10-5fcc-9fcd-fa6d29cbfa10
splat_kind: item
splat_key: dnd2024:equipment/ladder_1_sp
splat_name: Ladder (1 SP)
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
    value: 1
    unit: sp
  mass:
    value: 25.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/ladder_1_sp
  implementation: dnd2024:equipment_ladder_1_sp
  parameters:
    height_ft: 10
    traversal: climb
    mass:
      value: 25.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_ladder_1_sp
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
  - dnd2024:equipment_ladder_1_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 5ac1dbba2540c3b72e9d39ee7ad9012414776ce5959120250342af9b1faac9d0
  line_span:
    first: 8465
    count: 3
splat_clauses:
- key: source_rules
  text: '# Ladder (1 SP) A Ladder is 10 feet tall. You must climb to move up or down it.'
  sha256: d9e1274f3481527a5ae1b19a2bba5dfb505339a85415d1ab8a9c27526b3008bb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ladder (1 SP)

<!-- SRD page 98 -->

A Ladder is 10 feet tall. You must climb to move up\
or down it.\

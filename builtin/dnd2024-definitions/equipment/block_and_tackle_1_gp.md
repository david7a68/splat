---
splat_schema: 1
splat_id: a8e95bdd-9833-5cd5-8a85-1656c48edf56
splat_kind: item
splat_key: dnd2024:equipment/block_and_tackle_1_gp
splat_name: Block and Tackle (1 GP)
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
    value: 5.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/block_and_tackle_1_gp
  implementation: dnd2024:equipment_block_and_tackle_1_gp
  parameters:
    hoist_capacity:
      op: multiply
      operands:
      - 4
      - stat: normal_lift_capacity
    mass:
      value: 5.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_block_and_tackle_1_gp
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
  - 96
  engine_gaps: &id001
  - dnd2024:equipment_block_and_tackle_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 08ace7434a33b7e1ec89dc194c3fc0af8b1bf95200bfeda5a46fa35145b49e7e
  line_span:
    first: 8305
    count: 3
splat_clauses:
- key: source_rules
  text: '# Block and Tackle (1 GP) A Block and Tackle allows you to hoist up to four times the weight
    you can normally lift.'
  sha256: bc1c989cfc32380f908bc52b756ac7c01677e887401aff89a8bab5787ee4397a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Block and Tackle (1 GP)

<!-- SRD page 96 -->

A Block and Tackle allows you to hoist up to four\
times the weight you can normally lift.\

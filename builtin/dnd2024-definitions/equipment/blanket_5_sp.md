---
splat_schema: 1
splat_id: a7433f8d-e2a1-5936-81d8-5bb4a482b18c
splat_kind: item
splat_key: dnd2024:equipment/blanket_5_sp
splat_name: Blanket (5 SP)
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
    value: 5
    unit: sp
  mass:
    value: 3.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/blanket_5_sp
  implementation: dnd2024:equipment_blanket_5_sp
  parameters:
    while_wrapped:
      roll: saving_throw
      mode: advantage
      scope: extreme_cold
    mass:
      value: 3.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_blanket_5_sp
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
  - dnd2024:equipment_blanket_5_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 26f67b5e63c1eec607ddd7bf88ae715023c658eb17872b4defe9091a9bf6f6ea
  line_span:
    first: 8301
    count: 4
splat_clauses:
- key: source_rules
  text: '# Blanket (5 SP) While wrapped in a blanket, you have Advantage on saving throws against extreme
    cold (see “Gameplay Toolbox”).'
  sha256: 24500093df575a3e381dc41a5468b9be3759dba1e1d87bbaa8b7874a21e9f7a3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blanket (5 SP)

<!-- SRD page 96 -->

While wrapped in a blanket, you have Advantage on\
saving throws against extreme cold (see “Gameplay\
Toolbox”).\

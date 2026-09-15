---
splat_schema: 1
splat_id: b27f6b06-d9d0-55c8-ac07-deb3ea70bf57
splat_kind: item
splat_key: dnd2024:equipment/bedroll_1_gp
splat_name: Bedroll (1 GP)
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
    value: 7.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/bedroll_1_gp
  implementation: dnd2024:equipment_bedroll_1_gp
  parameters:
    sleep_capacity:
      count: 1
      sizes:
      - small
      - medium
    while_inside:
      saving_throw_scope: extreme_cold
      result: automatic_success
    mass:
      value: 7.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_bedroll_1_gp
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
  - dnd2024:equipment_bedroll_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 373566650985e27745a3d76fd7fc3a5c52425999920271e30d22f1a0fe70d498
  line_span:
    first: 8293
    count: 5
splat_clauses:
- key: source_rules
  text: '# Bedroll (1 GP) A Bedroll sleeps one Small or Medium creature. While in a Bedroll, you automatically
    succeed on saving throws against extreme cold (see “Gameplay Toolbox”).'
  sha256: 096e855196e88f3852db4d6cbe898ec6b670423886dda6c9c64a60e49c933c74
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bedroll (1 GP)

<!-- SRD page 96 -->

A Bedroll sleeps one Small or Medium creature.\
While in a Bedroll, you automatically succeed on\
saving throws against extreme cold (see “Gameplay\
Toolbox”).\

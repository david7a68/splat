---
splat_schema: 1
splat_id: 0d82864d-775d-549a-aece-1e16c16a7233
splat_kind: item
splat_key: dnd2024:equipment/pole_5_cp
splat_name: Pole (5 CP)
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
    unit: cp
  mass:
    value: 7.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/pole_5_cp
  implementation: dnd2024:equipment_pole_5_cp
  parameters:
    length_ft: 10
    touch_range_ft: 10
    vault:
      roll: ability_check
      mode: advantage
      ability: strength
      skill: athletics
      scope: required_high_or_long_jump_check
    mass:
      value: 7.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_pole_5_cp
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
  - dnd2024:equipment_pole_5_cp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 1db170079abe713a68cfbab721399d8823cc366b8b2a4cd66f0830ee2cb25baa
  line_span:
    first: 8576
    count: 6
splat_clauses:
- key: source_rules
  text: '# Pole (5 CP) A Pole is 10 feet long. You can use it to touch something up to 10 feet away. If
    you must make a Strength (Athletics) check as part of a High or Long Jump, you can use the Pole to
    vault, giving yourself Advantage on the check.'
  sha256: e7981eab1c8b5b6434de573ad818f7e8652d479a044b9e9dec956902485904f0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pole (5 CP)

<!-- SRD page 99 -->

A Pole is 10 feet long. You can use it to touch\
something up to 10 feet away. If you must make a\
Strength (Athletics) check as part of a High or Long\
Jump, you can use the Pole to vault, giving yourself\
Advantage on the check.\

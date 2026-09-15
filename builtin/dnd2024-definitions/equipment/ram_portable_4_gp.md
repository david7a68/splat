---
splat_schema: 1
splat_id: 3b2cb6cd-1d2a-5659-ada8-195557ffd613
splat_kind: item
splat_key: dnd2024:equipment/ram_portable_4_gp
splat_name: Ram, Portable (4 GP)
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
    value: 4
    unit: gp
  mass:
    value: 35.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/ram_portable_4_gp
  implementation: dnd2024:equipment_ram_portable_4_gp
  parameters:
    break_door:
      ability: strength
      check_bonus: 4
      helper_maximum: 1
      with_helper_mode: advantage
    mass:
      value: 35.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_ram_portable_4_gp
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
  - dnd2024:equipment_ram_portable_4_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 23e5a4ec93309ddcb57199039464e7a1251108499e67e9fc5574e0879c1de59d
  line_span:
    first: 8599
    count: 5
splat_clauses:
- key: source_rules
  text: '# Ram, Portable (4 GP) You can use a Portable Ram to break down doors. When doing so, you gain
    a +4 bonus to the Strength check. One other character can help you use the ram, giving you Advantage
    on this check.'
  sha256: b49531dfb199b6c3102e9ae2cf96103705ec3d7688e2d5766f24b275dd3dbc5f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ram, Portable (4 GP)

<!-- SRD page 99 -->

You can use a Portable Ram to break down doors.\
When doing so, you gain a +4 bonus to the Strength\
check. One other character can help you use the\
ram, giving you Advantage on this check.\

---
splat_schema: 1
splat_id: 891bccc3-1366-53c0-b1a5-ce5494bda5e5
splat_kind: item
splat_key: dnd2024:equipment/robe_1_gp
splat_name: Robe (1 GP)
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
    value: 1
    unit: gp
  mass:
    value: 4.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/robe_1_gp
  implementation: dnd2024:equipment_robe_1_gp
  parameters:
    access_requirement:
      authority: dm
      scope:
      - event
      - location
      eligible_if_wearing: true
      required_colors:
        type: choice
        authority: dm
      required_symbols:
        type: choice
        authority: dm
    mass:
      value: 4.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_robe_1_gp
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
  - dnd2024:equipment_robe_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: e74552382e8c5403e5e6eb48c582539dc1c71c52f1073e3ca2fdbbe386568020
  line_span:
    first: 8608
    count: 4
splat_clauses:
- key: source_rules
  text: '# Robe (1 GP) A Robe has vocational or ceremonial significance. Some events and locations admit
    only people wearing a Robe bearing certain colors or symbols.'
  sha256: 1d078c8dfc65a2054e8173ed1086aad586c01e2074c766c1c0dcdd91abc4c13b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Robe (1 GP)

<!-- SRD page 99 -->

A Robe has vocational or ceremonial significance.\
Some events and locations admit only people wear-\
ing a Robe bearing certain colors or symbols.\

---
splat_schema: 1
splat_id: ac8e87e8-da02-5b25-82a4-23f07e2140a1
splat_kind: item
splat_key: dnd2024:equipment/lantern_bullseye_10_gp
splat_name: Lantern, Bullseye (10 GP)
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
    value: 2.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/lantern_bullseye_10_gp
  implementation: dnd2024:equipment_lantern_bullseye_10_gp
  parameters:
    light:
      requires_lit: true
      shape: cone
      bright_ft: 60
      additional_dim_ft: 60
    fuel:
      key: dnd2024:equipment/oil_1_sp
    hours_per_flask: 6
    mass:
      value: 2.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_lantern_bullseye_10_gp
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
  - dnd2024:equipment_lantern_bullseye_10_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 235fb9f298eb1595b7871483c2def0bd92340e76bc751d86e8250ab91cd14f0a
  line_span:
    first: 8471
    count: 4
splat_clauses:
- key: source_rules
  text: '# Lantern, Bullseye (10 GP) A Bullseye Lantern burns Oil as fuel to cast Bright Light in a 60-foot
    Cone and Dim Light for an additional 60 feet.'
  sha256: 16c7bdba5047449926b0d6aa3865b88089046790946a9c5206987670a73316ff
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lantern, Bullseye (10 GP)

<!-- SRD page 98 -->

A Bullseye Lantern burns Oil as fuel to cast Bright\
Light in a 60-foot Cone and Dim Light for an addi-\
tional 60 feet.\

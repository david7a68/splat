---
splat_schema: 1
splat_id: ac8b78df-1670-5be3-a34d-104654c811ab
splat_kind: item
splat_key: dnd2024:equipment/lantern_hooded_5_gp
splat_name: Lantern, Hooded (5 GP)
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
    unit: gp
  mass:
    value: 2.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/lantern_hooded_5_gp
  implementation: dnd2024:equipment_lantern_hooded_5_gp
  parameters:
    light:
      requires_lit: true
      shape: radius
      bright_ft: 30
      additional_dim_ft: 30
    fuel:
      key: dnd2024:equipment/oil_1_sp
    hours_per_flask: 6
    hood:
      economy: bonus_action
      operations:
      - lower
      - raise
      lowered_light:
        shape: radius
        bright_ft: 0
        dim_ft: 5
    mass:
      value: 2.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_lantern_hooded_5_gp
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
  - dnd2024:equipment_lantern_hooded_5_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 24b212a51910e415dea4b0e86b36c70890ac2b2217e120d9785553103ebf2667
  line_span:
    first: 8475
    count: 6
splat_clauses:
- key: source_rules
  text: '# Lantern, Hooded (5 GP) A Hooded Lantern burns Oil as fuel to cast Bright Light in a 30-foot
    radius and Dim Light for an additional 30 feet. As a Bonus Action, you can lower the hood, reducing
    the light to Dim Light in a 5-foot radius, or raise it again.'
  sha256: 6641f400a56498061fe36ee6088a2fb4a21212a4ddd87ae90c98b9ad79f99d99
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lantern, Hooded (5 GP)

<!-- SRD page 98 -->

A Hooded Lantern burns Oil as fuel to cast Bright\
Light in a 30-foot radius and Dim Light for an ad-\
ditional 30 feet. As a Bonus Action, you can lower\
the hood, reducing the light to Dim Light in a 5-foot\
radius, or raise it again.\

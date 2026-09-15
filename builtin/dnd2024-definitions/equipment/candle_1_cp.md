---
splat_schema: 1
splat_id: 316c92fa-a021-535b-aff8-ac6e28364841
splat_kind: item
splat_key: dnd2024:equipment/candle_1_cp
splat_name: Candle (1 CP)
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
    unit: cp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/candle_1_cp
  implementation: dnd2024:equipment_candle_1_cp
  parameters:
    light:
      requires_lit: true
      shape: radius
      bright_ft: 5
      additional_dim_ft: 5
    burn_duration:
      value: 1
      unit: hour
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_candle_1_cp
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
  - dnd2024:equipment_candle_1_cp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: b094439d9787f9e0a9e7ce9b80c6f09c59444b73fea6fa20df88570d8dc4af96
  line_span:
    first: 8330
    count: 3
splat_clauses:
- key: source_rules
  text: '# Candle (1 CP) For 1 hour, a lit Candle sheds Bright Light in a 5-foot radius and Dim Light
    for an additional 5 feet.'
  sha256: 4c910d2ad01ef5b0057418b9aec353fcf29007123013d183193c39d383492a89
  status: contract_specified
splat_requires_mechanics: *id001
---

# Candle (1 CP)

<!-- SRD page 96 -->

For 1 hour, a lit Candle sheds Bright Light in a 5-foot\
radius and Dim Light for an additional 5 feet.\

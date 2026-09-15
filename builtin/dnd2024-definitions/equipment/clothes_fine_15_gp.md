---
splat_schema: 1
splat_id: 64766bd6-c8ff-50e0-8e32-cd9b03056446
splat_kind: item
splat_key: dnd2024:equipment/clothes_fine_15_gp
splat_name: Clothes, Fine (15 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 15
    unit: gp
  mass:
    value: 6.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/clothes_fine_15_gp
  implementation: dnd2024:equipment_clothes_fine_15_gp
  parameters:
    access_requirement:
      authority: dm
      scope:
      - event
      - location
      eligible_if_wearing: true
    mass:
      value: 6.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_clothes_fine_15_gp
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
  - 97
  engine_gaps: &id001
  - dnd2024:equipment_clothes_fine_15_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 7820f3ce9a9b1d9d0f13346308348f646d7b90243743564a901f79335438194a
  line_span:
    first: 8358
    count: 5
splat_clauses:
- key: source_rules
  text: '# Clothes, Fine (15 GP) Fine Clothes are made of expensive fabrics and adorned with expertly
    crafted details. Some events and locations admit only people wearing these clothes.'
  sha256: e9ebf1e1afe0570c4a8d546463d7d03e8763960649614f073057765e702f535a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Clothes, Fine (15 GP)

<!-- SRD page 97 -->

Fine Clothes are made of expensive fabrics and\
adorned with expertly crafted details. Some events\
and locations admit only people wearing these\
clothes.\

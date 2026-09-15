---
splat_schema: 1
splat_id: 828ea576-67f4-550b-801c-7f8cf78ed75a
splat_kind: item
splat_key: dnd2024:equipment/clothes_traveler_s_2_gp
splat_name: Clothes, Traveler’s (2 GP)
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
    value: 2
    unit: gp
  mass:
    value: 4.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/clothes_traveler_s_2_gp
  implementation: dnd2024:equipment_clothes_traveler_s_2_gp
  parameters:
    garment:
      purpose: travel
      resilient: true
    mass:
      value: 4.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_clothes_traveler_s_2_gp
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
  - dnd2024:equipment_clothes_traveler_s_2_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 16b3f2f1d4194de50562f32cca1498d9ef197415274d8c18c1ca03f46abf3348
  line_span:
    first: 8363
    count: 3
splat_clauses:
- key: source_rules
  text: '# Clothes, Traveler’s (2 GP) Traveler’s Clothes are resilient garments designed for travel in
    various environments.'
  sha256: bb6147c43992fca262002c5148a7bb91acc78b43f443e5d497d52b9763cb55af
  status: contract_specified
splat_requires_mechanics: *id001
---

# Clothes, Traveler’s (2 GP)

<!-- SRD page 97 -->

Traveler’s Clothes are resilient garments designed\
for travel in various environments.\

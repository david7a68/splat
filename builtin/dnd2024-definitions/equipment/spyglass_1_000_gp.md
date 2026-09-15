---
splat_schema: 1
splat_id: 6be059f3-a0fb-5182-8e46-0157bfd8b8e1
splat_kind: item
splat_key: dnd2024:equipment/spyglass_1_000_gp
splat_name: Spyglass (1,000 GP)
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 1000
    unit: gp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/spyglass_1_000_gp
  implementation: dnd2024:equipment_spyglass_1_000_gp
  parameters:
    view:
      apparent_size_multiplier: 2
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_spyglass_1_000_gp
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
  - 100
  engine_gaps: &id001
  - dnd2024:equipment_spyglass_1_000_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: fdc57f3399f28fdfc472109a25ac56458faffb3d322a44bde2a641c3d033e3b5
  line_span:
    first: 8656
    count: 3
splat_clauses:
- key: source_rules
  text: '# Spyglass (1,000 GP) Objects viewed through a Spyglass are magnified to twice their size.'
  sha256: 34ee6830712b558577e3fb6ac98f1ddea6c4d5084132be2127ef8a1ccf08a21b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spyglass (1,000 GP)

<!-- SRD page 100 -->

Objects viewed through a Spyglass are magnified to\
twice their size.\

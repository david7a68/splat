---
splat_schema: 1
splat_id: f4efbbe1-2fb0-5d26-9b2b-cd250764ceac
splat_kind: item
splat_key: dnd2024:equipment/rations_5_sp
splat_name: Rations (5 SP)
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
    unit: sp
  mass:
    value: 2.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/rations_5_sp
  implementation: dnd2024:equipment_rations_5_sp
  parameters:
    food:
      travel_ready: true
      components:
      - jerky
      - dried_fruit
      - hardtack
      - nuts
      mass_lb: 2
    nutrition_rule:
      key: dnd2024:glossary/malnutrition_hazard
    mass:
      value: 2.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_rations_5_sp
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
  - dnd2024:equipment_rations_5_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 549ea5fa4f644f5e639df7545f691cf193edc158d38965517552ee5d79651779
  line_span:
    first: 8604
    count: 4
splat_clauses:
- key: source_rules
  text: '# Rations (5 SP) Rations consist of travel-ready food, including jerky, dried fruit, hardtack,
    and nuts. See “Malnutrition” in “Rules Glossary” for the risks of not eating.'
  sha256: b855f5469ee4859e0b01c7555fe9139c005eb0c9d3b54c3f741a99a95d643409
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rations (5 SP)

<!-- SRD page 99 -->

Rations consist of travel-ready food, including jerky,\
dried fruit, hardtack, and nuts. See “Malnutrition”\
in “Rules Glossary” for the risks of not eating.\

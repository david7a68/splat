---
splat_schema: 1
splat_id: 30968556-9039-54c6-b86f-d4fb6baadd28
splat_kind: item
splat_key: dnd2024:equipment/magnifying_glass_100_gp
splat_name: Magnifying Glass (100 GP)
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
    value: 100
    unit: gp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/magnifying_glass_100_gp
  implementation: dnd2024:equipment_magnifying_glass_100_gp
  parameters:
    using:
      roll: ability_check
      mode: advantage
      scope: appraise_or_inspect_highly_detailed_item
    ignite:
      minimum_light: sunlight_brightness
      requires_tinder: true
      duration:
        value: 5
        unit: minute
      approximate: true
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_magnifying_glass_100_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    operation:
      required: false
      type: choice
    target:
      required: false
      type: participant
    target_object:
      required: false
      type: object
    location:
      required: false
      type: location
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
  outputs:
  - action_options
  - check_requests
  - item_effects
  - item_state
  events:
  - item.used
  capabilities:
  - item_actions
  - effects
  - ability_checks
  - consumables
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 98
  engine_gaps: &id001
  - dnd2024:equipment_magnifying_glass_100_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 2ce050005dc970403f886fe37ffb644afb1bdc0663c4e659a33a4b3bdc81baba
  line_span:
    first: 8485
    count: 6
splat_clauses:
- key: source_rules
  text: '# Magnifying Glass (100 GP) A Magnifying Glass grants Advantage on any ability check made to
    appraise or inspect a highly detailed item. Lighting a fire with a Magnifying Glass requires light
    as bright as sunlight to focus, tinder to ignite, and about 5 minutes for the fire to ignite.'
  sha256: 7351fb8e73702fe2b32ab38a6b2502e74fa1a4ef9a021069486c082393bff732
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magnifying Glass (100 GP)

<!-- SRD page 98 -->

A Magnifying Glass grants Advantage on any ability\
check made to appraise or inspect a highly detailed\
item. Lighting a fire with a Magnifying Glass re-\
quires light as bright as sunlight to focus, tinder to\
ignite, and about 5 minutes for the fire to ignite.\

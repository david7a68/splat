---
splat_schema: 1
splat_id: 08eddade-32e9-5fb4-b96b-b9c141eb5b8a
splat_kind: item
splat_key: dnd2024:equipment/tinderbox_5_sp
splat_name: Tinderbox (5 SP)
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
    value: 5
    unit: sp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/tinderbox_5_sp
  implementation: dnd2024:equipment_tinderbox_5_sp
  parameters:
    components:
    - flint
    - fire_steel
    - tinder
    ignite:
      exposed_fuel:
        economy: bonus_action
      other_fire:
        duration:
          value: 1
          unit: minute
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_tinderbox_5_sp
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
  - 100
  engine_gaps: &id001
  - dnd2024:equipment_tinderbox_5_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 2e345768921c27ed5c7b211eae1ee919072bac7c81c1e1d4146abf9c545f9a10
  line_span:
    first: 8664
    count: 7
splat_clauses:
- key: source_rules
  text: '# Tinderbox (5 SP) A Tinderbox is a small container holding flint, fire steel, and tinder (usually
    dry cloth soaked in light oil) used to kindle a fire. Using it to light a Candle, Lamp, Lantern, or
    Torch—or anything else with exposed fuel—takes a Bonus Action. Lighting any other fire takes 1 minute.'
  sha256: a37a23d80df652a444818cbbb1f9d6a019a7da5acc0fd12551c753d0ce735110
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tinderbox (5 SP)

<!-- SRD page 100 -->

A Tinderbox is a small container holding flint, fire\
steel, and tinder (usually dry cloth soaked in light\
oil) used to kindle a fire. Using it to light a Candle,\
Lamp, Lantern, or Torch—or anything else with\
exposed fuel—takes a Bonus Action. Lighting any\
other fire takes 1 minute.\

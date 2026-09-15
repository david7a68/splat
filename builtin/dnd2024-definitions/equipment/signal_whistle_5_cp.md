---
splat_schema: 1
splat_id: c1e03a27-67a9-5276-bfba-615f7de246f6
splat_kind: item
splat_key: dnd2024:equipment/signal_whistle_5_cp
splat_name: Signal Whistle (5 CP)
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
    unit: cp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/signal_whistle_5_cp
  implementation: dnd2024:equipment_signal_whistle_5_cp
  parameters:
    use:
      action: utilize
      effect:
        sound_audible_range_ft: 600
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_signal_whistle_5_cp
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
  - 99
  engine_gaps: &id001
  - dnd2024:equipment_signal_whistle_5_cp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 4f7c354735099f505ff7964c92facc013238f1c52c7568a062723ab04f8f6351
  line_span:
    first: 8634
    count: 4
splat_clauses:
- key: source_rules
  text: '# Signal Whistle (5 CP) When blown as a Utilize action, a Signal Whistle produces a sound that
    can be heard up to 600 feet away.'
  sha256: 2e370e3e40e4c50944f382cf38e153e3064a0b7ac2c79496c4964ac16e2ae1a0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Signal Whistle (5 CP)

<!-- SRD page 99 -->

When blown as a Utilize action, a Signal Whistle\
produces a sound that can be heard up to 600 feet\
away.\

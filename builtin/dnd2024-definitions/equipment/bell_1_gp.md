---
splat_schema: 1
splat_id: e874d0e0-ba14-5ebe-a718-b0a315c60115
splat_kind: item
splat_key: dnd2024:equipment/bell_1_gp
splat_name: Bell (1 GP)
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
    unit: gp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/bell_1_gp
  implementation: dnd2024:equipment_bell_1_gp
  parameters:
    use:
      action: utilize
      effect:
        sound_audible_range_ft: 60
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_bell_1_gp
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
  - 96
  engine_gaps: &id001
  - dnd2024:equipment_bell_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 5ac5e04d017fa495906f3e0c0ab6c5cc5436b0e15e17b490b26586788ae698e0
  line_span:
    first: 8298
    count: 3
splat_clauses:
- key: source_rules
  text: '# Bell (1 GP) When rung as a Utilize action, a Bell produces a sound that can be heard up to
    60 feet away.'
  sha256: 82ec20fbac0c610a0d34e663b8754d1e97779843d48a6405798b4ecf972b99b5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bell (1 GP)

<!-- SRD page 96 -->

When rung as a Utilize action, a Bell produces a\
sound that can be heard up to 60 feet away.\

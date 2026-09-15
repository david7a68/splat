---
splat_schema: 1
splat_id: 712db94b-28bd-5dce-b8d7-ad4165453795
splat_kind: item
splat_key: dnd2024:equipment/shovel_2_gp
splat_name: Shovel (2 GP)
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
    value: 2
    unit: gp
  mass:
    value: 5.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/shovel_2_gp
  implementation: dnd2024:equipment_shovel_2_gp
  parameters:
    dig:
      duration:
        value: 1
        unit: hour
      shape: cube
      side_ft: 5
      material_choices:
      - soil
      - similar_material
      similar_material_authority: dm
    mass:
      value: 5.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_shovel_2_gp
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
  - dnd2024:equipment_shovel_2_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 4feedf86e6b3cb43214df6fe7cef87d5eb9248834f98fc54e62bdb046d2bb04b
  line_span:
    first: 8630
    count: 4
splat_clauses:
- key: source_rules
  text: '# Shovel (2 GP) Working for 1 hour, you can use a Shovel to dig a hole that is 5 feet on each
    side in soil or similar material.'
  sha256: 06bb1565df51ae77797a57fe6ff9d1af10973e9893818e9fdac983708eb4fb51
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shovel (2 GP)

<!-- SRD page 99 -->

Working for 1 hour, you can use a Shovel to dig a\
hole that is 5 feet on each side in soil or similar\
material.\

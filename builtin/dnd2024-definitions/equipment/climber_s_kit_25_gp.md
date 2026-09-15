---
splat_schema: 1
splat_id: 0096b952-f968-54f0-966c-338f69a01e15
splat_kind: item
splat_key: dnd2024:equipment/climber_s_kit_25_gp
splat_name: Climber’s Kit (25 GP)
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
    value: 25
    unit: gp
  mass:
    value: 12.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/climber_s_kit_25_gp
  implementation: dnd2024:equipment_climber_s_kit_25_gp
  parameters:
    components:
    - boot_tips
    - gloves
    - pitons
    - harness
    anchor:
      action: utilize
      target: self
      maximum_fall_ft: 25
      maximum_distance_from_anchor_ft: 25
    unanchor:
      economy: bonus_action
    mass:
      value: 12.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_climber_s_kit_25_gp
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
  - 97
  engine_gaps: &id001
  - dnd2024:equipment_climber_s_kit_25_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 9b5eb0c68e43469dfaa4afc3317c8578ad9845648824498bd940ce88b9d65d4f
  line_span:
    first: 8351
    count: 7
splat_clauses:
- key: source_rules
  text: '# Climber’s Kit (25 GP) A Climber’s Kit includes boot tips, gloves, pitons, and a harness. As
    a Utilize action, you can use the Climber’s Kit to anchor yourself; when you do, you can’t fall more
    than 25 feet from the anchor point, and you can’t move more than 25 feet from there without undoing
    the anchor as a Bonus Action.'
  sha256: e580aea90d70edc7fef9c938d3f3aafa2c69b496f56569a933ee6997d3b0b4e0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Climber’s Kit (25 GP)

<!-- SRD page 97 -->

A Climber’s Kit includes boot tips, gloves, pitons,\
and a harness. As a Utilize action, you can use the\
Climber’s Kit to anchor yourself; when you do, you\
can’t fall more than 25 feet from the anchor point,\
and you can’t move more than 25 feet from there\
without undoing the anchor as a Bonus Action.\

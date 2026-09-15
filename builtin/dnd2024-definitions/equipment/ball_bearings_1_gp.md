---
splat_schema: 1
splat_id: 55664569-b7c6-5c29-a6be-a0f8a1be023f
splat_kind: item
splat_key: dnd2024:equipment/ball_bearings_1_gp
splat_name: Ball Bearings (1 GP)
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
    value: 2.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/ball_bearings_1_gp
  implementation: dnd2024:equipment_ball_bearings_1_gp
  parameters:
    deploy:
      action: utilize
      area:
        shape: square
        side_ft: 10
        range_ft: 10
        level_ground: true
      trigger: creature_enters_first_time_on_turn
      save:
        ability: dexterity
        dc: 10
        failure:
          condition: prone
        success: no_effect
    recover:
      duration:
        value: 10
        unit: minute
    mass:
      value: 2.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_ball_bearings_1_gp
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
  - dnd2024:equipment_ball_bearings_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: b33c53eff469b9db0e388ad087631527b029e8e68c1e87ef9eaa09e822ef1571
  line_span:
    first: 8280
    count: 8
splat_clauses:
- key: source_rules
  text: '# Ball Bearings (1 GP) As a Utilize action, you can spill Ball Bearings from their pouch. They
    spread to cover a level, 10-footsquare area within 10 feet of yourself. A creature that enters this
    area for the first time on a turn must succeed on a DC 10 Dexterity saving throw or have the Prone
    condition. It takes 10 minutes to recover the Ball Bearings.'
  sha256: e3b7673ecf0ff89481d433702afe88f96653e942a36b2bc1f769ab59e3722986
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ball Bearings (1 GP)

<!-- SRD page 96 -->

As a Utilize action, you can spill Ball Bearings from\
their pouch. They spread to cover a level, 10-foot-\
square area within 10 feet of yourself. A creature\
that enters this area for the first time on a turn\
must succeed on a DC 10 Dexterity saving throw or\
have the Prone condition. It takes 10 minutes to re-\
cover the Ball Bearings.\

---
splat_schema: 1
splat_id: a4cb4197-cac1-5c91-874d-53516761b2f1
splat_kind: item
splat_key: dnd2024:equipment/caltrops_1_gp
splat_name: Caltrops (1 GP)
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
  mechanic: dnd2024:equipment/caltrops_1_gp
  implementation: dnd2024:equipment_caltrops_1_gp
  parameters:
    deploy:
      action: utilize
      area:
        shape: square
        side_ft: 5
        range_ft: 5
      trigger: creature_enters_first_time_on_turn
      save:
        ability: dexterity
        dc: 15
        failure:
          damage:
            amount: 1
            type: piercing
          speed_set: 0
          speed_expiry: target_next_turn_start
        success: no_effect
    recover:
      duration:
        value: 10
        unit: minute
    mass:
      value: 2.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_caltrops_1_gp
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
  - dnd2024:equipment_caltrops_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 5f463de1515f862503592fea270b06fd6fcac8044e8564878bc72e0eaf4dec7f
  line_span:
    first: 8322
    count: 8
splat_clauses:
- key: source_rules
  text: '# Caltrops (1 GP) As a Utilize action, you can spread Caltrops from their bag to cover a 5-foot-square
    area within 5 feet of yourself. A creature that enters this area for the first time on a turn must
    succeed on a DC 15 Dexterity saving throw or take 1 Piercing damage and have its Speed reduced to
    0 until the start of its next turn. It takes 10 minutes to recover the Caltrops.'
  sha256: e6a8e9938c5883d9faf23a98d4e990ea5dcd6b41aeec19dd94700c2dc08b732d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Caltrops (1 GP)

<!-- SRD page 96 -->

As a Utilize action, you can spread Caltrops from\
their bag to cover a 5-foot-square area within 5 feet\
of yourself. A creature that enters this area for the\
first time on a turn must succeed on a DC 15 Dex-\
terity saving throw or take 1 Piercing damage and\
have its Speed reduced to 0 until the start of its next\
turn. It takes 10 minutes to recover the Caltrops.\

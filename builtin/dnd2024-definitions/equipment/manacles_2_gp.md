---
splat_schema: 1
splat_id: 20bc9590-5fc2-5f8e-ab11-5993354b875f
splat_kind: item
splat_key: dnd2024:equipment/manacles_2_gp
splat_name: Manacles (2 GP)
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
    value: 2
    unit: gp
  mass:
    value: 6.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/manacles_2_gp
  implementation: dnd2024:equipment_manacles_2_gp
  parameters:
    bind:
      action: utilize
      target:
        range_ft: 5
        sizes:
        - small
        - medium
        unwilling: true
        has_any_condition:
        - grappled
        - incapacitated
        - restrained
      check:
        ability: dexterity
        skill: sleight_of_hand
        dc: 13
      while_bound:
        attack_roll_mode: disadvantage
        restrained_if: attached_to_fixed_chain_or_hook
    escape:
      economy: action
      check:
        ability: dexterity
        skill: sleight_of_hand
        dc: 20
    burst:
      economy: action
      check:
        ability: strength
        skill: athletics
        dc: 25
    includes_key: true
    pick:
      requires_item:
        key: dnd2024:equipment/other_tools_thieves_tools_25_gp
      check:
        ability: dexterity
        skill: sleight_of_hand
        dc: 15
      success: unlock
    mass:
      value: 6.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_manacles_2_gp
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
  - dnd2024:equipment_manacles_2_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: a0127099395e85909ad70afcf8a2bd4d0c3b85cce0065ae7d3a80af4d653180e
  line_span:
    first: 8491
    count: 17
splat_clauses:
- key: source_rules
  text: '# Manacles (2 GP) As a Utilize action, you can use Manacles to bind an unwilling Small or Medium
    creature within 5 feet of yourself that has the Grappled, Incapacitated, or Restrained condition if
    you succeed on a DC 13 Dexterity (Sleight of Hand) check. While bound, a creature has Disadvantage
    on attack rolls, and the creature is Restrained if the Manacles are attached to a chain or hook that
    is fixed in place. Escaping the Manacles requires a successful DC 20 Dexterity (Sleight of Hand) check
    as an action. Bursting them requires a successful DC 25 Strength (Athletics) check as an action. Each
    set of Manacles comes with a key. Without the key, a creature can use Thieves’ Tools to pick the Manacles’
    lock with a successful DC 15 Dexterity (Sleight of Hand) check.'
  sha256: 5b3a5867a74348b784bbe2a8568c6d97023de1a1b99d482adc10ee18aea8a00c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Manacles (2 GP)

<!-- SRD page 98 -->

As a Utilize action, you can use Manacles to bind an\
unwilling Small or Medium creature within 5 feet\
of yourself that has the Grappled, Incapacitated,\
or Restrained condition if you succeed on a DC 13\
Dexterity (Sleight of Hand) check. While bound, a\
creature has Disadvantage on attack rolls, and the\
creature is Restrained if the Manacles are attached\
to a chain or hook that is fixed in place. Escaping\
the Manacles requires a successful DC 20 Dexterity\
(Sleight of Hand) check as an action. Bursting them\
requires a successful DC 25 Strength (Athletics)\
check as an action.\
Each set of Manacles comes with a key. Without\
the key, a creature can use Thieves’ Tools to pick the\
Manacles’ lock with a successful DC 15 Dexterity\
(Sleight of Hand) check.\

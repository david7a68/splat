---
splat_schema: 1
splat_id: 16ecaf16-855f-5f92-956e-891647177473
splat_kind: item
splat_key: dnd2024:equipment/chain_5_gp
splat_name: Chain (5 GP)
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
    value: 5
    unit: gp
  mass:
    value: 10.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/chain_5_gp
  implementation: dnd2024:equipment_chain_5_gp
  parameters:
    bind:
      action: utilize
      target:
        range_ft: 5
        unwilling: true
        has_any_condition:
        - grappled
        - incapacitated
        - restrained
      check:
        ability: strength
        skill: athletics
        dc: 13
      bound_legs_condition: restrained
      until: escape
    escape:
      economy: action
      check:
        ability: dexterity
        skill: acrobatics
        dc: 18
    burst:
      economy: action
      check:
        ability: strength
        skill: athletics
        dc: 20
    mass:
      value: 10.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_chain_5_gp
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
  - dnd2024:equipment_chain_5_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 6e9dd578e9508a65ecb1cd527eb4cc3eb8f7c9db6342cc71daa6fa09e81d8cf5
  line_span:
    first: 8338
    count: 11
splat_clauses:
- key: source_rules
  text: '# Chain (5 GP) As a Utilize action, you can wrap a Chain around an unwilling creature within
    5 feet of yourself that has the Grappled, Incapacitated, or Restrained condition if you succeed on
    a DC 13 Strength (Athletics) check. If the creature’s legs are bound, the creature has the Restrained
    condition until it escapes. Escaping the Chain requires the creature to make a successful DC 18 Dexterity
    (Acrobatics) check as an action. Bursting the Chain requires a successful DC 20 Strength (Athletics)
    check as an action.'
  sha256: 27a048d68d469f4ca6fddb655c0996d9736dd4a97abac6d609e68fced27d5893
  status: contract_specified
splat_requires_mechanics: *id001
---

# Chain (5 GP)

<!-- SRD page 96 -->

As a Utilize action, you can wrap a Chain around an\
unwilling creature within 5 feet of yourself that has\
the Grappled, Incapacitated, or Restrained condi-\
tion if you succeed on a DC 13 Strength (Athletics)\
check. If the creature’s legs are bound, the creature\
has the Restrained condition until it escapes. Es-\
caping the Chain requires the creature to make a\
successful DC 18 Dexterity (Acrobatics) check as an\
action. Bursting the Chain requires a successful DC\
20 Strength (Athletics) check as an action.\

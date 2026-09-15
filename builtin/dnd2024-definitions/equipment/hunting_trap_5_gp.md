---
splat_schema: 1
splat_id: 37ab425c-b924-5442-a4d6-ec9a30294fb7
splat_kind: item
splat_key: dnd2024:equipment/hunting_trap_5_gp
splat_name: Hunting Trap (5 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97, 98
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 5
    unit: gp
  mass:
    value: 25.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/hunting_trap_5_gp
  implementation: dnd2024:equipment_hunting_trap_5_gp
  parameters:
    set:
      action: utilize
      anchor: immobile_object
      trigger: creature_steps_on_pressure_plate
      save:
        ability: dexterity
        dc: 13
        success: no_effect
        failure:
          damage:
            amount:
              dice:
                count: 1
                sides: 4
              bonus: 0
            type: piercing
          speed_set: 0
          speed_expiry: target_next_turn_start
          afterward: movement_limited_to_chain_length
      chain_length_ft:
        default: 3
        type: number
        authority: item_instance
    free:
      economy: action
      target: self_or_trapped_creature_in_reach
      check:
        ability: strength
        skill: athletics
        dc: 13
      success: free_target
      failure:
        damage:
          target: trapped_creature
          amount: 1
          type: piercing
    mass:
      value: 25.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_hunting_trap_5_gp
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
  - 98
  engine_gaps: &id001
  - dnd2024:equipment_hunting_trap_5_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 2cffcf40584bea808a3c121f91b4fc4de1429c94a46b381aef39f0d3c53fbb4b
  line_span:
    first: 8441
    count: 17
splat_clauses:
- key: source_rules
  text: '# Hunting Trap (5 GP) As a Utilize action, you can set a Hunting Trap, which is a sawtooth steel
    ring that snaps shut when a creature steps on a pressure plate in the center. The trap is affixed
    by a heavy chain to an immobile object, such as a tree or a spike driven into the ground. A creature
    that steps on the plate must succeed on a DC 13 Dexterity saving throw or take 1d4 Piercing damage
    and have its Speed reduced to 0 until the start of its next turn. Thereafter, until the creature breaks
    free of the trap, its movement is limited by the length of the chain (typically 3 feet). A creature
    can use its action to make a DC 13 Strength (Athletics) check, freeing itself or another creature
    within its reach on a success. Each failed check deals 1 Piercing damage to the trapped creature.'
  sha256: 194f0d472e30d3518971a50339692ebc4e56648488f09271af3bfcc0c23cbdf0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hunting Trap (5 GP)

<!-- SRD page 97 -->

As a Utilize action, you can set a Hunting Trap,\
which is a sawtooth steel ring that snaps shut when\
a creature steps on a pressure plate in the center.\

<!-- SRD page 98 -->

The trap is affixed by a heavy chain to an immo-\
bile object, such as a tree or a spike driven into the\
ground. A creature that steps on the plate must\
succeed on a DC 13 Dexterity saving throw or take\
1d4 Piercing damage and have its Speed reduced to\
0 until the start of its next turn. Thereafter, until\
the creature breaks free of the trap, its movement\
is limited by the length of the chain (typically 3\
feet). A creature can use its action to make a DC\
13 Strength (Athletics) check, freeing itself or an-\
other creature within its reach on a success. Each\
failed check deals 1 Piercing damage to the trapped\
creature.\

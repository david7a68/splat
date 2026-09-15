---
splat_schema: 1
splat_id: 1146923b-6200-5244-a2d0-54a361b76f2d
splat_kind: item
splat_key: dnd2024:equipment/net_1_gp
splat_name: Net (1 GP)
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
    value: 1
    unit: gp
  mass:
    value: 3.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/net_1_gp
  implementation: dnd2024:equipment_net_1_gp
  parameters:
    throw:
      economy: replace_one_attack
      action: attack
      target:
        kind: creature
        visible: true
        range_ft: 15
      save:
        ability: dexterity
        dc:
          op: add
          operands:
          - 8
          - ability_modifier: dexterity
          - stat: proficiency_bonus
        automatic_success_sizes:
        - huge
        - gargantuan
        success: no_effect
        failure:
          condition: restrained
          until: escape_or_net_destroyed
    escape:
      economy: action
      actor: target_or_creature_within_5_ft
      check:
        ability: strength
        skill: athletics
        dc: 10
      success: free_target
    object_stats:
      armor_class: 10
      hit_points: 5
      damage_immunities:
      - bludgeoning
      - poison
      - psychic
    on_destroy: free_target
    mass:
      value: 3.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_net_1_gp
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
  - attack.replacement
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
  - dnd2024:equipment_net_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: dc75c16966f4fff57a05d717a5a0cf5a903b73ff0115619dcc1574de2be5279c
  line_span:
    first: 8516
    count: 15
splat_clauses:
- key: source_rules
  text: '# Net (1 GP) When you take the Attack action, you can replace one of your attacks with throwing
    a Net. Target a creature you can see within 15 feet of yourself. The target must succeed on a Dexterity
    saving throw (DC 8 plus your Dexterity modifier and Proficiency Bonus) or have the Restrained condition
    until it escapes. The target succeeds automatically if it is Huge or larger. To escape, the target
    or a creature within 5 feet of it must take an action to make a DC 10 Strength (Athletics) check,
    freeing the Restrained creature on a success. Destroying the Net (AC 10; 5 HP; Immunity to Bludgeoning,
    Poison, and Psychic damage) also frees the target, ending the effect.'
  sha256: 2e72400e52aa4d9679ff06af74551ff08a850845e887bd993bcd66c79875cd81
  status: contract_specified
splat_requires_mechanics: *id001
---

# Net (1 GP)

<!-- SRD page 98 -->

When you take the Attack action, you can replace\
one of your attacks with throwing a Net. Target a\
creature you can see within 15 feet of yourself. The\
target must succeed on a Dexterity saving throw\
(DC 8 plus your Dexterity modifier and Proficiency\
Bonus) or have the Restrained condition until it\
escapes. The target succeeds automatically if it is\
Huge or larger.\
To escape, the target or a creature within 5 feet\
of it must take an action to make a DC 10 Strength\
(Athletics) check, freeing the Restrained creature\
on a success. Destroying the Net (AC 10; 5 HP; Im-\
munity to Bludgeoning, Poison, and Psychic dam-\
age) also frees the target, ending the effect.\

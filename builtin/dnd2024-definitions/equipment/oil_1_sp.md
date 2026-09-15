---
splat_schema: 1
splat_id: b547b909-2f9d-594a-9dec-54430a9a77e9
splat_kind: item
splat_key: dnd2024:equipment/oil_1_sp
splat_name: Oil (1 SP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98, 99
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 1
    unit: sp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/oil_1_sp
  implementation: dnd2024:equipment_oil_1_sp
  parameters:
    douse_target:
      economy: replace_one_attack
      action: attack
      consume: 1
      target:
        count: 1
        kinds:
        - creature
        - object
        range_ft: 20
      save:
        ability: dexterity
        dc:
          op: add
          operands:
          - 8
          - ability_modifier: dexterity
          - stat: proficiency_bonus
        success: no_effect
        failure:
          coating_duration:
            value: 1
            unit: minute
          trigger: target_takes_fire_damage
          extra_damage:
            amount: 5
            type: fire
    douse_area:
      action: utilize
      consume: 1
      area:
        shape: square
        side_ft: 5
        range_ft: 5
        level_ground: true
      requires_ignition: true
      duration:
        rounds: 2
        expiry: turn_end
        outside_combat_seconds: 12
      triggers:
      - creature_enters
      - creature_turn_end_in_area
      damage:
        amount: 5
        type: fire
      maximum_applications_per_creature_per_turn: 1
    fuel:
      compatible:
      - key: dnd2024:equipment/lamp_5_sp
      - key: dnd2024:equipment/lantern_bullseye_10_gp
      - key: dnd2024:equipment/lantern_hooded_5_gp
      burn_time_per_flask:
        value: 6
        unit: hour
      time_is_cumulative: true
      extinguish_action: utilize
      rekindling_allowed: true
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_oil_1_sp
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
  - 99
  engine_gaps: &id001
  - dnd2024:equipment_oil_1_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: c470245c49b895de616c006fbe395cd3f4ac7aa56df5c45d921050ba61c08fd3
  line_span:
    first: 8531
    count: 26
splat_clauses:
- key: source_rules
  text: '# Oil (1 SP) You can douse a creature, object, or space with Oil or use it as fuel, as detailed
    below. Dousing a Creature or an Object. When you take the Attack action, you can replace one of your
    attacks with throwing an Oil flask. Target one creature or object within 20 feet of yourself. The
    target must succeed on a Dexterity saving throw (DC 8 plus your Dexterity modifier and Proficiency
    Bonus) or be covered in oil. If the target takes Fire damage before the oil dries (after 1 minute),
    the target takes an extra 5 Fire damage from burning oil. Dousing a Space. You can take the Utilize
    action to pour an Oil flask on level ground to cover a 5-foot-square area within 5 feet of yourself.
    If lit, the oil burns until the end of the turn 2 rounds from when the oil was lit (or 12 seconds)
    and deals 5 Fire damage to any creature that enters the area or ends its turn there. A creature can
    take this damage only once per turn. Fuel. Oil serves as fuel for Lamps and Lanterns. Once lit, a
    flask of Oil burns for 6 hours in a Lamp or Lantern. That duration doesn’t need to be consecutive;
    you can extinguish the burning Oil (as a Utilize action) and rekindle it again until it has burned
    for a total of 6 hours.'
  sha256: ca2a1732b535fb56e6d27e06bfa8b5986ed7517632f3b505f2645ae81a5ba49d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Oil (1 SP)

<!-- SRD page 98 -->

You can douse a creature, object, or space with Oil\
or use it as fuel, as detailed below.\
Dousing a Creature or an Object. When you\
take the Attack action, you can replace one of your\
attacks with throwing an Oil flask. Target one crea-\
ture or object within 20 feet of yourself. The target\
must succeed on a Dexterity saving throw (DC 8\
plus your Dexterity modifier and Proficiency Bonus)\
or be covered in oil. If the target takes Fire damage\
before the oil dries (after 1 minute), the target takes\
an extra 5 Fire damage from burning oil.\
Dousing a Space. You can take the Utilize ac-\
tion to pour an Oil flask on level ground to cover a\
5-foot-square area within 5 feet of yourself. If lit,\
the oil burns until the end of the turn 2 rounds from\
when the oil was lit (or 12 seconds) and deals 5 Fire\
damage to any creature that enters the area or ends\
its turn there. A creature can take this damage only\
once per turn.\

<!-- SRD page 99 -->

Fuel. Oil serves as fuel for Lamps and Lanterns.\
Once lit, a flask of Oil burns for 6 hours in a Lamp or\
Lantern. That duration doesn’t need to be consecu-\
tive; you can extinguish the burning Oil (as a Utilize\
action) and rekindle it again until it has burned for a\
total of 6 hours.\

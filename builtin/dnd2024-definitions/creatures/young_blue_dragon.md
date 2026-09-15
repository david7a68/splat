---
splat_schema: 1
splat_id: 3c69da88-f30f-5f64-b083-7cc44ef33822
splat_kind: creature
splat_key: dnd2024:creatures/young_blue_dragon
splat_name: Young Blue Dragon
splat_origin:
  key: dnd2024:reference/creature/young_blue_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 266
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Lawful Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Burrow 20 ft., Fly 80 ft.
  skills_text: Perception +9, Stealth +4
  immunities_text: Lightning
  senses_text: Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 19
  languages_text: Common, Draconic
  cr_text: 9 (XP 5,000; PB +4)
  hit_points_text: 152 (16d10 + 64)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 5
  dexterity:
    value: 10
  save_dexterity:
    value: 4
  constitution:
    value: 19
  save_constitution:
    value: 4
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 13
  save_wisdom:
    value: 5
  charisma:
    value: 17
  save_charisma:
    value: 3
  armor_class:
    value: 18
  max_hp:
    value: 152
  initiative:
    value: 4
  speed:
    value: 40
    unit: ft
  burrow_speed:
    value: 20
    unit: ft
  fly_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/young_blue_dragon
  implementation: dnd2024:creature_young_blue_dragon
  parameters:
    skill_totals:
      perception: 9
      stealth: 4
    damage_immunities:
    - lightning
    condition_immunities: []
    senses:
    - sense: blindsight
      range_ft: 30
    - sense: darkvision
      range_ft: 120
    passive_perception: 19
    languages:
      speak: true
      known:
      - common
      - draconic
    features:
      rend:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 9
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 2
                sides: 6
              add: 5
            average: 12
          - type: lightning
            amount:
              dice:
                count: 1
                sides: 10
              add: 0
            average: 5
      lightning_breath:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 16
        targets:
          kind: creature
          count: all
          area:
            shape: line
            origin: actor
            size_ft: 60
            width_ft: 5
        failure:
        - op: damage
          components:
          - type: lightning
            amount:
              dice:
                count: 10
                sides: 10
              add: 0
            average: 55
        success:
        - op: damage
          from: failure_damage
          multiplier: 0.5
          round: down
        recharge:
          die_sides: 6
          success_min: 5
          success_max: 6
          check_event: actor.turn_start
          maximum_uses: 1
          initial_uses: 1
          also_recovers_after:
          - short_rest
          - long_rest
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 3
          feature: rend
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spatial_resolution:
      provider: caller
      required_facts:
      - distance
      - area_membership
      - visibility
      - occupancy
      - movement_path
      validate_against_typed_constraints: true
splat_contracts:
- id: dnd2024:creature_young_blue_dragon
  inputs:
    actor:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    choices:
      required: false
      type: choice
    scene:
      required: false
      type: resolution
    event:
      required: false
      type: event
  outputs:
  - effects
  - roll_requests
  - state_changes
  events:
  - actor.turn_start
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 8965660efdb2512da0b9e52d3c48b3ff2a446c4967bf77ff26777880c4a4b9e7
  source_path: builtin/srd-5.2.1/creature/young_blue_dragon.md
  pages:
  - 266
  engine_gaps: &id001
  - dnd2024:creature_young_blue_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 754cb19930245feedacdc6e5f9113aa72fe4e13b36332262c538f96373a63e35
splat_clauses:
- key: source_rules
  text: '# Young Blue Dragon Large Dragon (Chromatic), Lawful Evil AC 18 Initiative +4 (14) HP 152 (16d10
    + 64) Speed 40 ft., Burrow 20 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 21 +5 +5 Dex 10 +0 +4
    Con 19 +4 +4 Int 14 +2 +2 Wis 13 +1 +5 Cha 17 +3 +3 Skills Perception +9, Stealth +4 Immunities Lightning
    Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 19 Languages Common, Draconic CR
    9 (XP 5,000; PB +4) ### Actions Multiattack. The dragon makes three Rend attacks. Rend. Melee Attack
    Roll: +9, reach 10 ft. Hit: 12 (2d6 + 5) Slashing damage plus 5 (1d10) Lightning damage. Lightning
    Breath (Recharge 5–6). Dexterity Saving Throw: DC 16, each creature in a 60-foot-long, 5-footwide
    Line. Failure: 55 (10d10) Lightning damage. Success: Half damage.'
  sha256: b2933fc5e3a83bbb38860931e6500dd752c8ae2c0f99d886485651d361982098
  status: contract_specified
splat_requires_mechanics: *id001
---

# Young Blue Dragon

<!-- SRD page 266 -->

Large Dragon (Chromatic), Lawful Evil\
AC 18 Initiative +4 (14)\
HP 152 (16d10 + 64)\
Speed 40 ft., Burrow 20 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 10 +0 +4 Con 19 +4 +4\
Int 14 +2 +2 Wis 13 +1 +5 Cha 17 +3 +3\
Skills Perception +9, Stealth +4\
Immunities Lightning\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 19\
Languages Common, Draconic\
CR 9 (XP 5,000; PB +4)\

### Actions

Multiattack. The dragon makes three Rend attacks.\
Rend. Melee Attack Roll: +9, reach 10 ft. Hit: 12 (2d6\
+ 5) Slashing damage plus 5 (1d10) Lightning damage.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 16, each creature in a 60-foot-long, 5-foot-\
wide Line. Failure: 55 (10d10) Lightning damage. Suc-\
cess: Half damage.\

Original layout: [[../../srd-5.2.1/pages/266|page 266]].

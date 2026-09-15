---
splat_schema: 1
splat_id: c764ea50-08dd-54ef-a072-39043928808f
splat_kind: creature
splat_key: dnd2024:creatures/ape
splat_name: Ape
splat_origin:
  key: dnd2024:reference/creature/ape
splat_sources:
- title: SRD 5.2.1
  locator: pp. 344
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Athletics +5, Perception +3
  senses_text: Passive Perception 13
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 19 (3d8 + 6)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 19
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ape
  implementation: dnd2024:creature_ape
  parameters:
    skill_totals:
      athletics: 5
      perception: 3
    senses: []
    passive_perception: 13
    languages:
      speak: true
      known: []
    features:
      fist:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 5
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 4
              add: 3
            average: 5
      rock:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 5
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 25
          long: 50
        hit:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 2
                sides: 6
              add: 3
            average: 10
        recharge:
          die_sides: 6
          success_min: 6
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
        - count: 2
          feature: fist
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
- id: dnd2024:creature_ape
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
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: c68fa64e146eb8134c7b8e45ae8c5bf540f5eecd9a9bda459cfae97961008b22
  source_path: builtin/srd-5.2.1/creature/ape.md
  pages:
  - 344
  engine_gaps: &id001
  - dnd2024:creature_ape
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 3f60ea2b329c499421943f2294ad1f948582c79e25455ed3aef4f251458e6321
splat_clauses:
- key: source_rules
  text: '# Ape Medium Beast, Unaligned AC 12 Initiative +2 (12) HP 19 (3d8 + 6) Speed 30 ft., Climb 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 14 +2 +2 Con 14 +2 +2 Int 6 -2 -2 Wis 12 +1 +1 Cha
    7 -2 -2 Skills Athletics +5, Perception +3 Senses Passive Perception 13 Languages None CR 1/2 (XP
    100; PB +2) ### Actions Multiattack. The ape makes two Fist attacks. Fist. Melee Attack Roll: +5,
    reach 5 ft. Hit: 5 (1d4 + 3) Bludgeoning damage. Rock (Recharge 6). Ranged Attack Roll: +5, range
    25/50 ft. Hit: 10 (2d6 + 3) Bludgeoning damage.'
  sha256: 21dbd209833a81cb6db776db19e18fda431674b8d8a78c248e25e0f3aefd07a4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ape

<!-- SRD page 344 -->

Medium Beast, Unaligned\
AC 12 Initiative +2 (12)\
HP 19 (3d8 + 6)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 14 +2 +2 Con 14 +2 +2\
Int 6 −2 −2 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Athletics +5, Perception +3\
Senses Passive Perception 13\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Actions

Multiattack. The ape makes two Fist attacks.\
Fist. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3)\
Bludgeoning damage.\
Rock (Recharge 6). Ranged Attack Roll: +5, range\
25/50 ft. Hit: 10 (2d6 + 3) Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/344|page 344]].

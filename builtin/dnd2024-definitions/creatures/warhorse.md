---
splat_schema: 1
splat_id: 9bb302df-2d68-5311-9182-d160654ad979
splat_kind: creature
splat_key: dnd2024:creatures/warhorse
splat_name: Warhorse
splat_origin:
  key: dnd2024:reference/creature/warhorse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 364
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 60 ft.
  senses_text: Passive Perception 11
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 19 (3d10 + 3)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 3
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 11
  max_hp:
    value: 19
  initiative:
    value: 1
  speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/warhorse
  implementation: dnd2024:creature_warhorse
  parameters:
    senses: []
    passive_perception: 11
    languages:
      speak: true
      known: []
    features:
      hooves:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 6
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
                count: 2
                sides: 4
              add: 4
            average: 9
        - op: conditional
          when:
            target_maximum_size: large
            actor_moved_straight_toward_target_ft_at_least: 20
            immediately_before_hit: true
            target_kind: creature
          effects:
          - op: damage
            components:
            - type: bludgeoning
              amount:
                dice:
                  count: 2
                  sides: 4
                add: 0
              average: 5
          - op: condition.apply
            condition: prone
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
- id: dnd2024:creature_warhorse
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
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: f1534724359fab4fce61595bdbf0c114feddb274b84c4fcf5945d9ccd627bc01
  source_path: builtin/srd-5.2.1/creature/warhorse.md
  pages:
  - 364
  engine_gaps: &id001
  - dnd2024:creature_warhorse
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: c1a1eb8367dfa760db2b1c1bc82e0d769480c4b5f1b3cad85b3b9eab24d9a8b7
splat_clauses:
- key: source_rules
  text: '# Warhorse Large Beast, Unaligned AC 11 Initiative +1 (11) HP 19 (3d10 + 3) Speed 60 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 12 +1 +1 Con13 +1 +1 Int 2 -4 -4 Wis 12 +1 +3 Cha 7 -2 -2
    Senses Passive Perception 11 Languages None CR 1/2 (XP 100; PB +2) ### Actions Hooves. Melee Attack
    Roll: +6, reach 5 ft. Hit: 9 (2d4 + 4) Bludgeoning damage. If the target is a Large or smaller creature
    and the horse moved 20+ feet straight toward it immediately before the hit, the target takes an extra
    5 (2d4) Bludgeoning damage and has the Prone condition.'
  sha256: a785a94aad4e962239bbf00be97ff70bc8b9ad5bd48edb27f29789422a2410ce
  status: contract_specified
splat_requires_mechanics: *id001
---

# Warhorse

<!-- SRD page 364 -->

Large Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 19 (3d10 + 3)\
Speed 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 12 +1 +1 Con13 +1 +1\
Int 2 −4 −4 Wis 12 +1 +3 Cha 7 −2 −2\
Senses Passive Perception 11\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (2d4 +\
4) Bludgeoning damage. If the target is a Large or smaller\
creature and the horse moved 20+ feet straight toward\
it immediately before the hit, the target takes an extra 5\
(2d4) Bludgeoning damage and has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/364|page 364]].

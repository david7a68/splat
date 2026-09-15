---
splat_schema: 1
splat_id: 22aec2de-4165-572c-a211-b153c046c532
splat_kind: creature
splat_key: dnd2024:creatures/brown_bear
splat_name: Brown Bear
splat_origin:
  key: dnd2024:reference/creature/brown_bear
splat_sources:
- title: SRD 5.2.1
  locator: pp. 346
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft., Climb 30 ft.
  skills_text: Perception +3
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 22 (3d10 + 6)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 11
  max_hp:
    value: 22
  initiative:
    value: 1
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/brown_bear
  implementation: dnd2024:creature_brown_bear
  parameters:
    skill_totals:
      perception: 3
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 13
    languages:
      speak: true
      known: []
    features:
      bite:
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
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              add: 3
            average: 7
      claw:
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
          - type: slashing
            amount:
              dice:
                count: 1
                sides: 4
              add: 3
            average: 5
        - op: condition.apply
          condition: prone
          when:
            target_kind: creature
            target_maximum_size: large
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 1
          feature: bite
        - count: 1
          feature: claw
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
- id: dnd2024:creature_brown_bear
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
  source_sha256: f0158ecdda4268e412403a085d02090de34d6440169eddfee31fb62526fd5c86
  source_path: builtin/srd-5.2.1/creature/brown_bear.md
  pages:
  - 346
  engine_gaps: &id001
  - dnd2024:creature_brown_bear
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: dccc0dfb9326e75fcc804cd178ed6afa5b55f75711747a19ad5388e9ee08fb83
splat_clauses:
- key: source_rules
  text: '# Brown Bear Large Beast, Unaligned AC 11 Initiative +1 (11) HP 22 (3d10 + 6) Speed 40 ft., Climb
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 12 +1 +1 Con 15 +2 +2 Int 2 -4 -4 Wis 13 +1 +1
    Cha 7 -2 -2 Skills Perception +3 Senses Darkvision 60 ft.; Passive Perception 13 Languages None CR
    1 (XP 200; PB +2) ### Actions Multiattack. The bear makes one Bite attack and one Claw attack. Bite.
    Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Piercing damage. Claw. Melee Attack Roll: +5,
    reach 5 ft. Hit: 5 (1d4 + 3) Slashing damage. If the target is a Large or smaller creature, it has
    the Prone condition.'
  sha256: dcd32653ac4843a7458d14b6c189f2dc18cbca2043faf9b88bf1822c06da3a36
  status: contract_specified
splat_requires_mechanics: *id001
---

# Brown Bear

<!-- SRD page 346 -->

Large Beast, Unaligned\
AC 11 Initiative +1 (11)\
HP 22 (3d10 + 6)\
Speed 40 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 12 +1 +1 Con 15 +2 +2\
Int 2 −4 −4 Wis 13 +1 +1 Cha 7 −2 −2\
Skills Perception +3\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1 (XP 200; PB +2)\

### Actions

Multiattack. The bear makes one Bite attack and one\
Claw attack.\
Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Piercing damage.\
Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3)\
Slashing damage. If the target is a Large or smaller crea-\
ture, it has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/346|page 346]].

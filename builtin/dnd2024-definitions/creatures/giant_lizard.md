---
splat_schema: 1
splat_id: f0a62d29-44ed-555c-b85f-e48889fa370a
splat_kind: creature
splat_key: dnd2024:creatures/giant_lizard
splat_name: Giant Lizard
splat_origin:
  key: dnd2024:reference/creature/giant_lizard
splat_sources:
- title: SRD 5.2.1
  locator: pp. 352
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft., Climb 40 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 19 (3d10 + 3)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 12
  save_dexterity:
    value: 3
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 12
  max_hp:
    value: 19
  initiative:
    value: 1
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_lizard
  implementation: dnd2024:creature_giant_lizard
  parameters:
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 10
    languages:
      speak: true
      known: []
    features:
      spider_climb:
        economy: passive
        op: movement.climb
        difficult_surfaces: true
        ceilings: true
        ability_check_required: false
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 4
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
              add: 2
            average: 6
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
- id: dnd2024:creature_giant_lizard
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
  source_sha256: bb7a4e4df7ad76654101fd09d7bce008385aa3a2964ac9d3eaff2cd7c0206c89
  source_path: builtin/srd-5.2.1/creature/giant_lizard.md
  pages:
  - 352
  engine_gaps: &id001
  - dnd2024:creature_giant_lizard
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 24cec4a2c3eb9db797f58816415935b50053daba12ebeb61e311b2d7a63b8b7f
splat_clauses:
- key: source_rules
  text: '# Giant Lizard Large Beast, Unaligned AC 12 Initiative +1 (11) HP 19 (3d10 + 3) Speed 40 ft.,
    Climb 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 12 +1 +3 Con 13 +1 +1 Int 2 -4 -4 Wis 10
    +0 +0 Cha 5 -3 -3 Senses Darkvision 60 ft.; Passive Perception 10 Languages None CR 1/4 (XP 50; PB
    +2) ### Traits Spider Climb. The lizard can climb difficult surfaces, including along ceilings, without
    needing to make an ability check. ### Actions Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8
    + 2) Piercing damage.'
  sha256: 09037c6e1e575348332ffd5376b73fd2981cca0c11f101a68487836c5fc764d0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Lizard

<!-- SRD page 352 -->

Large Beast, Unaligned\
AC 12  Initiative +1 (11)\
HP 19 (3d10 + 3)\
Speed 40 ft., Climb 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 12 +1 +3 Con 13 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Traits

Spider Climb. The lizard can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/352|page 352]].

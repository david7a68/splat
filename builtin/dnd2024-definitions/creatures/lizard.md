---
splat_schema: 1
splat_id: 2f497705-dfff-5dc6-a052-42a1327dcf87
splat_kind: creature
splat_key: dnd2024:creatures/lizard
splat_name: Lizard
splat_origin:
  key: dnd2024:reference/creature/lizard
splat_sources:
- title: SRD 5.2.1
  locator: pp. 357
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 20 ft., Climb 20 ft.
  senses_text: Darkvision 30 ft.; Passive Perception 9
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 2 (1d4)
splat_stats:
  strength:
    value: 2
  save_strength:
    value: -4
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 10
  max_hp:
    value: 2
  initiative:
    value: 0
  speed:
    value: 20
    unit: ft
  climb_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/lizard
  implementation: dnd2024:creature_lizard
  parameters:
    senses:
    - sense: darkvision
      range_ft: 30
    passive_perception: 9
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
        attack_bonus: 2
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              constant: 1
            average: 1
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
- id: dnd2024:creature_lizard
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
  source_sha256: 132b44f8c43e107e952d7029a281c1e2d620a6454518c8b04e56d5a43cbbcbba
  source_path: builtin/srd-5.2.1/creature/lizard.md
  pages:
  - 357
  engine_gaps: &id001
  - dnd2024:creature_lizard
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 515eee6087b0ca56484c8ef7a2727f03d327db8dd7e5ff26518fdedbae1d5c79
splat_clauses:
- key: source_rules
  text: '# Lizard Tiny Beast, Unaligned AC 10 Initiative +0 (10) HP 2 (1d4) Speed 20 ft., Climb 20 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 2 -4 -4 Dex 11 +0 +0 Con 10 +0 +0 Int 1 -5 -5 Wis 8 -1 -1 Cha 3 -4
    -4 Senses Darkvision 30 ft.; Passive Perception 9 Languages None CR 0 (XP 10; PB +2) ### Traits Spider
    Climb. The lizard can climb difficult surfaces, including along ceilings, without needing to make
    an ability check. ### Actions Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage.'
  sha256: 144656236e09bac53252a53e996d0c4e7c75c5fe21402ee25db30c709113d6e7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lizard

<!-- SRD page 357 -->

Tiny Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 2 (1d4)\
Speed 20 ft., Climb 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 11 +0 +0 Con 10 +0 +0\
Int 1 −5 −5 Wis 8 −1 −1 Cha 3 −4 −4\
Senses Darkvision 30 ft.; Passive Perception 9\
Languages None\
CR 0 (XP 10; PB +2)\

### Traits

Spider Climb. The lizard can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing\
damage.\

Original layout: [[../../srd-5.2.1/pages/357|page 357]].

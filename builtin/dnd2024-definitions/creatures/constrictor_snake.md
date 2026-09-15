---
splat_schema: 1
splat_id: 54f0ed25-a085-5b50-b211-31b98aca035c
splat_kind: creature
splat_key: dnd2024:creatures/constrictor_snake
splat_name: Constrictor Snake
splat_origin:
  key: dnd2024:reference/creature/constrictor_snake
splat_sources:
- title: SRD 5.2.1
  locator: pp. 346, 347
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Swim 30 ft.
  skills_text: Perception +2, Stealth +4
  senses_text: Blindsight 10 ft.; Passive Perception 12
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 13 (2d10 + 2)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 13
  max_hp:
    value: 13
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/constrictor_snake
  implementation: dnd2024:creature_constrictor_snake
  parameters:
    skill_totals:
      perception: 2
      stealth: 4
    senses:
    - sense: blindsight
      range_ft: 10
    passive_perception: 12
    languages:
      speak: true
      known: []
    features:
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
      constrict:
        economy: action
        op: saving_throw
        ability: strength
        dc: 12
        targets:
          kind: creature
          count: 1
          range_ft: 5
          maximum_size: medium
          visible_to_actor: true
        failure:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 3
                sides: 4
              add: 0
            average: 7
        - op: grapple
          escape_dc: 12
        success: []
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
- id: dnd2024:creature_constrictor_snake
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
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: eb446a11cb234fd25d2a2c404732ad98dac3d60394a6c513516cba6d7997b557
  source_path: builtin/srd-5.2.1/creature/constrictor_snake.md
  pages:
  - 346
  - 347
  engine_gaps: &id001
  - dnd2024:creature_constrictor_snake
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: de219ebbdc060abbd91138bcdb5fdc50801251df037f58d3309be77cdc6b80e2
splat_clauses:
- key: source_rules
  text: '# Constrictor Snake Large Beast, Unaligned AC 13 Initiative +2 (12) HP 13 (2d10 + 2) Speed 30
    ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 14 +2 +2 Con 12 +1 +1 Int 1 -5 -5 Wis
    10 +0 +0 Cha 3 -4 -4 Skills Perception +2, Stealth +4 Senses Blindsight 10 ft.; Passive Perception
    12 Languages None CR 1/4 (XP 50; PB +2) ### Actions Bite. Melee Attack Roll: +4, reach 5 ft. Hit:
    6 (1d8 + 2) Piercing damage. Constrict. Strength Saving Throw: DC 12, one Medium or smaller creature
    the snake can see within 5 feet. Failure: 7 (3d4) Bludgeoning damage, and the target has the Grappled
    condition (escape DC 12).'
  sha256: 7c1bad749a4ec1ce5c473fb0f7f970caf6e9351973e32bda4318738565d90f1f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Constrictor Snake

<!-- SRD page 346 -->

Large Beast, Unaligned\
AC 13  Initiative +2 (12)\
HP 13 (2d10 + 2)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 14 +2 +2 Con 12 +1 +1\
Int 1 −5 −5 Wis 10 +0 +0 Cha 3 −4 −4\
Skills Perception +2, Stealth +4\
Senses Blindsight 10 ft.; Passive Perception 12\
Languages None\
CR 1/4 (XP 50; PB +2)\

<!-- SRD page 347 -->


### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2)\
Piercing damage.\
Constrict. Strength Saving Throw: DC 12, one Medium\
or smaller creature the snake can see within 5 feet.\
Failure: 7 (3d4) Bludgeoning damage, and the target has\
the Grappled condition (escape DC 12).\

Original layout: [[../../srd-5.2.1/pages/346|page 346]], [[../../srd-5.2.1/pages/347|page 347]].

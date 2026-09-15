---
splat_schema: 1
splat_id: 3bc35e96-47b1-52f6-8872-2790bffce358
splat_kind: creature
splat_key: dnd2024:creatures/giant_badger
splat_name: Giant Badger
splat_origin:
  key: dnd2024:reference/creature/giant_badger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 349
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Burrow 10 ft.
  skills_text: Perception +3
  resistances_text: Poison
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 15 (2d8 + 6)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 13
  max_hp:
    value: 15
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
  burrow_speed:
    value: 10
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_badger
  implementation: dnd2024:creature_giant_badger
  parameters:
    skill_totals:
      perception: 3
    damage_resistances:
    - poison
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
        attack_bonus: 3
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
                count: 2
                sides: 4
              add: 1
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
- id: dnd2024:creature_giant_badger
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
  source_sha256: 199fb22925b2205cb35d72de9640a7e602e066f85c9c7798b21e89a27738fae1
  source_path: builtin/srd-5.2.1/creature/giant_badger.md
  pages:
  - 349
  engine_gaps: &id001
  - dnd2024:creature_giant_badger
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/giant_badger
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 2a17ebfd0231f47f775cf1439ab425e859a5ff2f610bbd577c9215a2c7624bc4
splat_clauses:
- key: source_rules
  text: '# Giant Badger Medium Beast, Unaligned AC 13 Initiative +0 (10) HP 15 (2d8 + 6) Speed 30 ft.,
    Burrow 10 ft. MOD SAVE MOD SAVE MOD SAVE Str 13 +1 +1 Dex 10 +0 +0 Con 17 +3 +3 Int 2 -4 -4 Wis 12
    +1 +1 Cha 5 -3 -3 Skills Perception +3 Resistances Poison Senses Darkvision 60 ft.; Passive Perception
    13 Languages None CR 1/4 (XP 50; PB +2) ### Actions Bite. Melee Attack Roll: +3, reach 5 ft. Hit:
    6 (2d4 + 1) Piercing damage.'
  sha256: ecb567856c27ca39b2ddd05c7d95a45086d29cfa0e38a8c2e2d43769d65103af
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Badger

<!-- SRD page 349 -->

Medium Beast, Unaligned\
AC 13  Initiative +0 (10)\
HP 15 (2d8 + 6)\
Speed 30 ft., Burrow 10 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 10 +0 +0 Con 17 +3 +3\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Perception +3\
Resistances Poison\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 6 (2d4 + 1)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/349|page 349]].

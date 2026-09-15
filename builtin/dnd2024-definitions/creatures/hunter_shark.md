---
splat_schema: 1
splat_id: 7beff13b-3807-593a-8d30-b4cb938a787b
splat_kind: creature
splat_key: dnd2024:creatures/hunter_shark
splat_name: Hunter Shark
splat_origin:
  key: dnd2024:reference/creature/hunter_shark
splat_sources:
- title: SRD 5.2.1
  locator: pp. 356
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 5 ft., Swim 40 ft.
  skills_text: Perception +2
  senses_text: Blindsight 60 ft.; Passive Perception 12
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 45 (6d10 + 12)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 12
  max_hp:
    value: 45
  initiative:
    value: 2
  speed:
    value: 5
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/hunter_shark
  implementation: dnd2024:creature_hunter_shark
  parameters:
    skill_totals:
      perception: 2
    senses:
    - sense: blindsight
      range_ft: 60
    passive_perception: 12
    languages:
      speak: true
      known: []
    features:
      water_breathing:
        economy: passive
        op: breathing.allow
        media:
        - water
        exclusive: true
      bite:
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
          - type: piercing
            amount:
              dice:
                count: 3
                sides: 6
              add: 4
            average: 14
        advantage_when:
          target_hp_below_maximum: true
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
- id: dnd2024:creature_hunter_shark
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
  source_sha256: 2cf30579465cfbbe4447edfe3eed87c58c8617d0713dfc04449ed1dec711a908
  source_path: builtin/srd-5.2.1/creature/hunter_shark.md
  pages:
  - 356
  engine_gaps: &id001
  - dnd2024:creature_hunter_shark
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: a75d93012852f18afd2e2c3e43cdb342eed703bec06818a4dfbc49db4fa64477
splat_clauses:
- key: source_rules
  text: '# Hunter Shark Large Beast, Unaligned AC 12 Initiative +2 (12) HP 45 (6d10 + 12) Speed 5 ft.,
    Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 14 +2 +2 Con 15 +2 +2 Int 1 -5 -5 Wis 10 +0
    +0 Cha 4 -3 -3 Skills Perception +2 Senses Blindsight 60 ft.; Passive Perception 12 Languages None
    CR 2 (XP 450; PB +2) ### Traits Water Breathing. The shark can breathe only underwater. ### Actions
    Bite. Melee Attack Roll: +6 (with Advantage if the target doesn’t have all its Hit Points), reach
    5 ft. Hit: 14 (3d6 + 4) Piercing damage.'
  sha256: 55ef094c4427e60acea343170f7ae28d4da192dbd2177ae189e68e071ca872d5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hunter Shark

<!-- SRD page 356 -->

Large Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 45 (6d10 + 12)\
Speed 5 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 14 +2 +2 Con 15 +2 +2\
Int 1 −5 −5 Wis 10 +0 +0 Cha 4 −3 −3\
Skills Perception +2\
Senses Blindsight 60 ft.; Passive Perception 12\
Languages None\
CR 2 (XP 450; PB +2)\

### Traits

Water Breathing. The shark can breathe only underwater.\

### Actions

Bite. Melee Attack Roll: +6 (with Advantage if the tar-\
get doesn’t have all its Hit Points), reach 5 ft. Hit: 14\
(3d6 + 4) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/356|page 356]].

---
splat_schema: 1
splat_id: 4bab219d-c9cc-5920-8bcc-1ba356ce8b64
splat_kind: creature
splat_key: dnd2024:creatures/giant_shark
splat_name: Giant Shark
splat_origin:
  key: dnd2024:reference/creature/giant_shark
splat_sources:
- title: SRD 5.2.1
  locator: pp. 353
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: beast
  alignment: Unaligned
  movement_text: 5 ft., Swim 60 ft.
  skills_text: Perception +3
  senses_text: Blindsight 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 92 (8d12 + 40)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 13
  max_hp:
    value: 92
  initiative:
    value: 3
  speed:
    value: 5
    unit: ft
  swim_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_shark
  implementation: dnd2024:creature_giant_shark
  parameters:
    skill_totals:
      perception: 3
    senses:
    - sense: blindsight
      range_ft: 60
    passive_perception: 13
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
        attack_bonus: 9
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
                sides: 10
              add: 6
            average: 22
        advantage_when:
          target_hp_below_maximum: true
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: bite
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
- id: dnd2024:creature_giant_shark
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
  source_sha256: 0fba0b4c0434b88e0f3f20a1d0e6406d7585a11cdaf57c7b5c1804ca729ddca8
  source_path: builtin/srd-5.2.1/creature/giant_shark.md
  pages:
  - 353
  engine_gaps: &id001
  - dnd2024:creature_giant_shark
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 958b1924bb2afc8fcb9fa6ecb10ed4c1695b95bb7e846cccf36a55c613b48b3f
splat_clauses:
- key: source_rules
  text: '# Giant Shark Huge Beast, Unaligned AC 13 Initiative +3 (13) HP 92 (8d12 + 40) Speed 5 ft., Swim
    60 ft. MOD SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 11 +0 +0 Con 21 +5 +5 Int 1 -5 -5 Wis 10 +0 +0
    Cha 5 -3 -3 Skills Perception +3 Senses Blindsight 60 ft.; Passive Perception 13 Languages None CR
    5 (XP 1,800; PB +3) ### Traits Water Breathing. The shark can breathe only underwater. ### Actions
    Multiattack. The shark makes two Bite attacks. Bite. Melee Attack Roll: +9 (with Advantage if the
    target doesn’t have all its Hit Points), reach 5 ft. Hit: 22 (3d10 + 6) Piercing damage.'
  sha256: 71b1932aea2380e215fe1e2bfe50e031c990f1b37ad82bce0d75d9f0c68a54e4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Shark

<!-- SRD page 353 -->

Huge Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 92 (8d12 + 40)\
Speed 5 ft., Swim 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 11 +0 +0 Con 21 +5 +5\
Int 1 −5 −5 Wis 10 +0 +0 Cha 5 −3 −3\
Skills Perception +3\
Senses Blindsight 60 ft.; Passive Perception 13\
Languages None\
CR 5 (XP 1,800; PB +3)\

### Traits

Water Breathing. The shark can breathe only underwater.\

### Actions

Multiattack. The shark makes two Bite attacks.\
Bite. Melee Attack Roll: +9 (with Advantage if the target\
doesn’t have all its Hit Points), reach 5 ft. Hit: 22 (3d10\
+ 6) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/353|page 353]].

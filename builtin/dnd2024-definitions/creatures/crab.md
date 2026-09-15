---
splat_schema: 1
splat_id: eaba7510-634f-5ec3-ab74-10343020a7ba
splat_kind: creature
splat_key: dnd2024:creatures/crab
splat_name: Crab
splat_origin:
  key: dnd2024:reference/creature/crab
splat_sources:
- title: SRD 5.2.1
  locator: pp. 347
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 20 ft., Swim 20 ft.
  skills_text: Stealth +2
  senses_text: Blindsight 30 ft.; Passive Perception 9
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 3 (1d4 + 1)
splat_stats:
  strength:
    value: 6
  save_strength:
    value: -2
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 2
  save_charisma:
    value: -4
  armor_class:
    value: 11
  max_hp:
    value: 3
  initiative:
    value: 0
  speed:
    value: 20
    unit: ft
  swim_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/crab
  implementation: dnd2024:creature_crab
  parameters:
    skill_totals:
      stealth: 2
    senses:
    - sense: blindsight
      range_ft: 30
    passive_perception: 9
    languages:
      speak: true
      known: []
    features:
      amphibious:
        economy: passive
        op: breathing.allow
        media:
        - air
        - water
      claw:
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
          - type: bludgeoning
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
- id: dnd2024:creature_crab
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
  source_sha256: acac0ddcdc39ac1cd89652c5ee7abb30af16c433e8cb2c2f7768d062bc5493c8
  source_path: builtin/srd-5.2.1/creature/crab.md
  pages:
  - 347
  engine_gaps: &id001
  - dnd2024:creature_crab
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 1244a8eb30659889db35812e5c7467f09eadde64c2fc1161b96d2f41bec99246
splat_clauses:
- key: source_rules
  text: '# Crab Tiny Beast, Unaligned AC 11 Initiative +0 (10) HP 3 (1d4 + 1) Speed 20 ft., Swim 20 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 6 -2 -2 Dex 11 +0 +0 Con 12 +1 +1 Int 1 -5 -5 Wis 8 -1 -1 Cha 2 -4
    -4 Skills Stealth +2 Senses Blindsight 30 ft.; Passive Perception 9 Languages None CR 0 (XP 10; PB
    +2) ### Traits Amphibious. The crab can breathe air and water. ### Actions Claw. Melee Attack Roll:
    +2, reach 5 ft. Hit: 1 Bludgeoning damage.'
  sha256: ca7eeb877ad81ef99a0802ed6f13ed28e4083896e75082206c63880cf42100b6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Crab

<!-- SRD page 347 -->

Tiny Beast, Unaligned\
AC 11  Initiative +0 (10)\
HP 3 (1d4 + 1)\
Speed 20 ft., Swim 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 11 +0 +0 Con 12 +1 +1\
Int 1 −5 −5 Wis 8 −1 −1 Cha 2 −4 −4\
Skills Stealth +2\
Senses Blindsight 30 ft.; Passive Perception 9\
Languages None\
CR 0 (XP 10; PB +2)\

### Traits

Amphibious. The crab can breathe air and water.\

### Actions

Claw. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Blud-\
geoning damage.\

Original layout: [[../../srd-5.2.1/pages/347|page 347]].

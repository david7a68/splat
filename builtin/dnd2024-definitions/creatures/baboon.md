---
splat_schema: 1
splat_id: e05c3bfb-3cc6-5042-a194-c0ff7ddc6f96
splat_kind: creature
splat_key: dnd2024:creatures/baboon
splat_name: Baboon
splat_origin:
  key: dnd2024:reference/creature/baboon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 345
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft.
  senses_text: Passive Perception 11
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 3 (1d6)
splat_stats:
  strength:
    value: 8
  save_strength:
    value: -1
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 4
  save_intelligence:
    value: -3
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 3
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
  mechanic: dnd2024:creatures/baboon
  implementation: dnd2024:creature_baboon
  parameters:
    senses: []
    passive_perception: 11
    languages:
      speak: true
      known: []
    features:
      pack_tactics:
        economy: passive
        op: roll.advantage
        rolls:
        - attack
        target_kind: creature
        when:
          ally_within_target_ft: 5
          ally_excluded_conditions:
          - incapacitated
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 1
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
                sides: 4
              add: -1
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
- id: dnd2024:creature_baboon
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
  source_sha256: 09da717970cd6e3c14a28c014336c2b5571033a63a4c15820e0eaa6da206438b
  source_path: builtin/srd-5.2.1/creature/baboon.md
  pages:
  - 345
  engine_gaps: &id001
  - dnd2024:creature_baboon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 4a9a346c971752bc66d7c67f6faa0538300cd94df6ddb09105a272b623afe8b5
splat_clauses:
- key: source_rules
  text: '# Baboon Small Beast, Unaligned AC 12 Initiative +2 (12) HP 3 (1d6) Speed 30 ft., Climb 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 8 -1 -1 Dex 14 +2 +2 Con 11 +0 +0 Int 4 -3 -3 Wis 12 +1 +1 Cha 6 -2
    -2 Senses Passive Perception 11 Languages None CR 0 (XP 10; PB +2) ### Traits Pack Tactics. The baboon
    has Advantage on an attack roll against a creature if at least one of the baboon’s allies is within
    5 feet of the creature and the ally doesn’t have the Incapacitated condition. ### Actions Bite. Melee
    Attack Roll: +1, reach 5 ft. Hit: 1 (1d4 - 1) Piercing damage.'
  sha256: be8eb0f7f81f0d3a2819695026a2a1a483a62ad734a4b78984717d19a04ffc86
  status: contract_specified
splat_requires_mechanics: *id001
---

# Baboon

<!-- SRD page 345 -->

Small Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 3 (1d6)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 14 +2 +2 Con 11 +0 +0\
Int 4 −3 −3 Wis 12 +1 +1 Cha 6 −2 −2\
Senses Passive Perception 11\
Languages None\
CR 0 (XP 10; PB +2)\

### Traits

Pack Tactics. The baboon has Advantage on an attack\
roll against a creature if at least one of the baboon’s\
allies is within 5 feet of the creature and the ally doesn’t\
have the Incapacitated condition.\

### Actions

Bite. Melee Attack Roll: +1, reach 5 ft. Hit: 1 (1d4 − 1)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/345|page 345]].

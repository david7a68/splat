---
splat_schema: 1
splat_id: 703a8a36-2e37-5b7f-bc16-0ae31fb12cc7
splat_kind: creature
splat_key: dnd2024:creatures/bandit
splat_name: Bandit
splat_origin:
  key: dnd2024:reference/creature/bandit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 261
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  gear_text: Leather Armor, Light Crossbow, Scimitar
  senses_text: Passive Perception 10
  languages_text: Common, Thieves’ Cant
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 11 (2d8 + 2)
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 12
  max_hp:
    value: 11
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/bandit
  implementation: dnd2024:creature_bandit
  parameters:
    gear:
    - item: leather_armor
      count: 1
    - item: light_crossbow
      count: 1
    - item: scimitar
      count: 1
    senses: []
    passive_perception: 10
    languages:
      speak: true
      known:
      - common
      - thieves_cant
    features:
      scimitar:
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
          - type: slashing
            amount:
              dice:
                count: 1
                sides: 6
              add: 1
            average: 4
      light_crossbow:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 3
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 80
          long: 320
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              add: 1
            average: 5
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
- id: dnd2024:creature_bandit
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
  source_sha256: 5ffe4bcfd67a8131df0ecb3c918da46933f0298280a52f7ae4ded50ba294f2b7
  source_path: builtin/srd-5.2.1/creature/bandit.md
  pages:
  - 261
  engine_gaps: &id001
  - dnd2024:creature_bandit
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/bandit
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: fa9e0b72af7fc42328ef417d2dc97c29e6d084dadf22fe50bcb3a77c463559bd
splat_clauses:
- key: source_rules
  text: '# Bandit Medium or Small Humanoid, Neutral AC 12 Initiative +1 (11) HP 11 (2d8 + 2) Speed 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 11 +0 +0 Dex 12 +1 +1 Con 12 +1 +1 Int 10 +0 +0 Wis 10 +0 +0 Cha
    10 +0 +0 Gear Leather Armor, Light Crossbow, Scimitar Senses Passive Perception 10 Languages Common,
    Thieves’ Cant CR 1/8 (XP 25; PB +2) ### Actions Scimitar. Melee Attack Roll: +3, reach 5 ft. Hit:
    4 (1d6 + 1) Slashing damage. Light Crossbow. Ranged Attack Roll: +3, range 80/320 ft. Hit: 5 (1d8
    + 1) Piercing damage.'
  sha256: 88a531e15a7550a47efd47de872755a86b47372c5f7a2cfe68e9be2b8dc75456
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bandit

<!-- SRD page 261 -->

Medium or Small Humanoid, Neutral\
AC 12 Initiative +1 (11)\
HP 11 (2d8 + 2)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 12 +1 +1 Con 12 +1 +1\
Int 10 +0 +0 Wis 10 +0 +0 Cha 10 +0 +0\
Gear Leather Armor, Light Crossbow, Scimitar\
Senses Passive Perception 10\
Languages Common, Thieves’ Cant\
CR 1/8 (XP 25; PB +2)\

### Actions

Scimitar. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6\
+ 1) Slashing damage.\
Light Crossbow. Ranged Attack Roll: +3, range 80/320\
ft. Hit: 5 (1d8 + 1) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/261|page 261]].

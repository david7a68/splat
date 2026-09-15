---
splat_schema: 1
splat_id: 4ad4d8a0-ac33-5953-a26d-9f48e8e902f7
splat_kind: creature
splat_key: dnd2024:creatures/guard
splat_name: Guard
splat_origin:
  key: dnd2024:reference/creature/guard
splat_sources:
- title: SRD 5.2.1
  locator: pp. 296
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  skills_text: Perception +2
  gear_text: Chain Shirt, Shield, Spear
  senses_text: Passive Perception 12
  languages_text: Common
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 11 (2d8 + 2)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
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
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 16
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
  mechanic: dnd2024:creatures/guard
  implementation: dnd2024:creature_guard
  parameters:
    skill_totals:
      perception: 2
    gear:
    - item: chain_shirt
      count: 1
    - item: shield
      count: 1
    - item: spear
      count: 1
    senses: []
    passive_perception: 12
    languages:
      speak: true
      known:
      - common
    features:
      spear:
        economy: action
        op: attack
        modes:
        - melee
        - ranged
        attack_bonus: 3
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        range_ft:
          normal: 20
          long: 60
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 6
              add: 1
            average: 4
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
- id: dnd2024:creature_guard
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
  source_sha256: 73d7d66f55fa1edb54a49a9a5cea28990f204bc5127939c8531785f1ebe80496
  source_path: builtin/srd-5.2.1/creature/guard.md
  pages:
  - 296
  engine_gaps: &id001
  - dnd2024:creature_guard
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/guard
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 8123ed7126451fc65c16be146c0905b5531f6a4d0cc33a2f10489caa8283c97d
splat_clauses:
- key: source_rules
  text: '# Guard Medium or Small Humanoid, Neutral AC 16 Initiative +1 (11) HP 11 (2d8 + 2) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 13 +1 +1 Dex 12 +1 +1 Con 12 +1 +1 Int 10 +0 +0 Wis 11 +0 +0 Cha 10
    +0 +0 Skills Perception +2 Gear Chain Shirt, Shield, Spear Senses Passive Perception 12 Languages
    Common CR 1/8 (XP 25; PB +2) ### Actions Spear. Melee or Ranged Attack Roll: +3, reach 5 ft. or range
    20/60 ft. Hit: 4 (1d6 + 1) Piercing damage.'
  sha256: cd43171ce39c5473c000ab573795bbfd3662e765515684a464960cee9f033212
  status: contract_specified
splat_requires_mechanics: *id001
---

# Guard

<!-- SRD page 296 -->

Medium or Small Humanoid, Neutral\
AC 16 Initiative +1 (11)\
HP 11 (2d8 + 2)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 12 +1 +1 Con 12 +1 +1\
Int 10 +0 +0 Wis 11 +0 +0 Cha 10 +0 +0\
Skills Perception +2\
Gear Chain Shirt, Shield, Spear\
Senses Passive Perception 12\
Languages Common\
CR 1/8 (XP 25; PB +2)\

### Actions

Spear. Melee or Ranged Attack Roll: +3, reach 5 ft. or\
range 20/60 ft. Hit: 4 (1d6 + 1) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/296|page 296]].

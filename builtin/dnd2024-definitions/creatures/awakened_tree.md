---
splat_schema: 1
splat_id: 8a76ac78-98c3-5ce4-b9d8-7bf1312ce86f
splat_kind: creature
splat_key: dnd2024:creatures/awakened_tree
splat_name: Awakened Tree
splat_origin:
  key: dnd2024:reference/creature/awakened_tree
splat_sources:
- title: SRD 5.2.1
  locator: pp. 260
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: plant
  alignment: Neutral
  movement_text: 20 ft.
  resistances_text: Bludgeoning, Piercing
  vulnerabilities_text: Fire
  senses_text: Passive Perception 10
  languages_text: Common plus one other language
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 59 (7d12 + 14)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 6
  save_dexterity:
    value: -2
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 59
  initiative:
    value: -2
  speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/awakened_tree
  implementation: dnd2024:creature_awakened_tree
  parameters:
    damage_vulnerabilities:
    - fire
    damage_resistances:
    - bludgeoning
    - piercing
    senses: []
    passive_perception: 10
    languages:
      speak: true
      known:
      - common
      additional_choice_count: 1
    features:
      slam:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 6
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 3
                sides: 6
              add: 4
            average: 14
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
- id: dnd2024:creature_awakened_tree
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
  source_sha256: 61dfb36c36f575f423fd57e33e0bb48dfcc2d9a2c1b62ec3b23be986ef3b175c
  source_path: builtin/srd-5.2.1/creature/awakened_tree.md
  pages:
  - 260
  engine_gaps: &id001
  - dnd2024:creature_awakened_tree
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/awakened_tree
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 0930c999f5a6668205e84ddb8ea2eb4c77388119a9e924516eb1b384364ec435
splat_clauses:
- key: source_rules
  text: '# Awakened Tree Huge Plant, Neutral AC 13 Initiative -2 (8) HP 59 (7d12 + 14) Speed 20 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 6 -2 -2 Con 15 +2 +2 Int 10 +0 +0 Wis 10 +0 +0 Cha 7 -2 -2
    Vulnerabilities Fire Resistances Bludgeoning, Piercing Senses Passive Perception 10 Languages Common
    plus one other language CR 2 (XP 450; PB +2) ### Actions Slam. Melee Attack Roll: +6, reach 10 ft.
    Hit: 14 (3d6 + 4) Bludgeoning damage.'
  sha256: 4be2c8ab4403c5d5735cb46cbddce2920349e04198b1b41b000930ff28195857
  status: contract_specified
splat_requires_mechanics: *id001
---

# Awakened Tree

<!-- SRD page 260 -->

Huge Plant, Neutral\
AC 13 Initiative −2 (8)\
HP 59 (7d12 + 14)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 6 −2 −2 Con 15 +2 +2\
Int 10 +0 +0 Wis 10 +0 +0 Cha 7 −2 −2\
Vulnerabilities Fire\
Resistances Bludgeoning, Piercing\
Senses Passive Perception 10\
Languages Common plus one other language\
CR 2 (XP 450; PB +2)\

### Actions

Slam. Melee Attack Roll: +6, reach 10 ft. Hit: 14 (3d6\
+ 4) Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/260|page 260]].

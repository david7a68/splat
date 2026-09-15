---
splat_schema: 1
splat_id: 9eb70a3b-655f-5d61-9db6-9da7198c1f94
splat_kind: creature
splat_key: dnd2024:creatures/guard_captain
splat_name: Guard Captain
splat_origin:
  key: dnd2024:reference/creature/guard_captain
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
  skills_text: Athletics +6, Perception +4
  gear_text: Breastplate, Javelins (6), Longsword, Shield
  senses_text: Passive Perception 14
  languages_text: Common
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 75 (10d8 + 30)
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
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 13
  save_charisma:
    value: 1
  armor_class:
    value: 18
  max_hp:
    value: 75
  initiative:
    value: 4
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/guard_captain
  implementation: dnd2024:creature_guard_captain
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_choice
          count: 2
          options:
          - javelin
          - longsword
      javelin:
        economy: action
        operation: attack
        attack:
          mode: melee_or_ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 6
          reach_ft: 5
          range_ft: 30
          long_range_ft: 120
        hit:
          damage:
          - average: 14
            roll:
              count: 3
              sides: 6
              modifier: 4
            type: piercing
      longsword:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 6
          reach_ft: 5
        hit:
          damage:
          - average: 15
            roll:
              count: 2
              sides: 10
              modifier: 4
            type: slashing
    skill_totals:
      athletics: 6
      perception: 4
    gear:
    - item: breastplate
      quantity: 1
    - item: javelins
      quantity: 6
    - item: longsword
      quantity: 1
    - item: shield
      quantity: 1
    senses:
      passive_perception: 14
    languages:
      speaks:
      - common
      understands: []
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_guard_captain
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 528993a4389df709065c87d1c0220809f669e2787245021f2c783a99de79d2ac
  source_path: builtin/srd-5.2.1/creature/guard_captain.md
  pages:
  - 296
  engine_gaps: &id001
  - dnd2024:creature_guard_captain
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 3c01fc9095ae944c2c3696dcd8d2d2c32d772e5cdcd3a04db104a8a3d02c1de0
splat_clauses:
- key: source_rules
  text: '# Guard Captain Medium or Small Humanoid, Neutral AC 18 Initiative +4 (14) HP 75 (10d8 + 30)
    Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 14 +2 +2 Con 16 +3 +3 Int 12 +1 +1 Wis 14
    +2 +2 Cha 13 +1 +1 Skills Athletics +6, Perception +4 Gear Breastplate, Javelins (6), Longsword, Shield
    Senses Passive Perception 14 Languages Common CR 4 (XP 1,100; PB +2) ### Actions Multiattack. The
    guard makes two attacks, using Javelin or Longsword in any combination. Javelin. Melee or Ranged Attack
    Roll: +6, reach 5 ft. or range 30/120 ft. Hit: 14 (3d6 + 4) Piercing damage. Longsword. Melee Attack
    Roll: +6, reach 5 ft. Hit: 15 (2d10 + 4) Slashing damage.'
  sha256: b767a876f90a007b630c84ceafe3c11aab75d1235d7565dbf9e92269be081e63
  status: contract_specified
splat_requires_mechanics: *id001
---

# Guard Captain

<!-- SRD page 296 -->

Medium or Small Humanoid, Neutral\
AC 18  Initiative +4 (14)\
HP 75 (10d8 + 30)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 14 +2 +2 Con 16 +3 +3\
Int 12 +1 +1 Wis 14 +2 +2 Cha 13 +1 +1\
Skills Athletics +6, Perception +4\
Gear Breastplate, Javelins (6), Longsword, Shield\
Senses Passive Perception 14\
Languages Common\
CR 4 (XP 1,100; PB +2)\

### Actions

Multiattack. The guard makes two attacks, using Javelin\
or Longsword in any combination.\
Javelin. Melee or Ranged Attack Roll: +6, reach 5 ft. or\
range 30/120 ft. Hit: 14 (3d6 + 4) Piercing damage.\
Longsword. Melee Attack Roll: +6, reach 5 ft. Hit: 15\
(2d10 + 4) Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/296|page 296]].

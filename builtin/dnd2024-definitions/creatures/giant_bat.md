---
splat_schema: 1
splat_id: 3e305169-1950-5fa9-b74f-21fa4f6bc77c
splat_kind: creature
splat_key: dnd2024:creatures/giant_bat
splat_name: Giant Bat
splat_origin:
  key: dnd2024:reference/creature/giant_bat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 349
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 10 ft., Fly 60 ft.
  senses_text: Blindsight 120 ft.; Passive Perception 11
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 22 (4d10)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 22
  initiative:
    value: 3
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_bat
  implementation: dnd2024:creature_giant_bat
  parameters:
    features:
      bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
        hit:
          damage:
          - average: 6
            roll:
              count: 1
              sides: 6
              modifier: 3
            type: piercing
    senses:
      blindsight: 120
      passive_perception: 11
    languages:
      speaks: []
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
- id: dnd2024:creature_giant_bat
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
  source_sha256: 3ff30696dc3431212318216ac799f8dc3ac731820c6e0a0504cf4a464bfcc52f
  source_path: builtin/srd-5.2.1/creature/giant_bat.md
  pages:
  - 349
  engine_gaps: &id001
  - dnd2024:creature_giant_bat
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/giant_bat
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: ad3f993fc488f99e7dbfef10aabf3e539b3e74be8ba7b7194a81ac2368ed9b72
splat_clauses:
- key: source_rules
  text: '# Giant Bat Large Beast, Unaligned AC 13 Initiative +3 (13) HP 22 (4d10) Speed 10 ft., Fly 60
    ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 16 +3 +3 Con 11 +0 +0 Int 2 -4 -4 Wis 12 +1 +1 Cha
    6 -2 -2 Senses Blindsight 120 ft.; Passive Perception 11 Languages None CR 1/4 (XP 50; PB +2) ###
    Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Piercing damage.'
  sha256: 5628a78166a57a3d98a2a1a251d686258715da5af51f253edb06c80f358e30f1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Bat

<!-- SRD page 349 -->

Large Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 22 (4d10)\
Speed 10 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 16 +3 +3 Con 11 +0 +0\
Int 2 −4 −4 Wis 12 +1 +1 Cha 6 −2 −2\
Senses Blindsight 120 ft.; Passive Perception 11\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/349|page 349]].

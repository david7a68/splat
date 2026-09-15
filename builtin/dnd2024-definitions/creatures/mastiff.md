---
splat_schema: 1
splat_id: af2e9ac8-fac6-5292-b5fe-0c96b1b9bb51
splat_kind: creature
splat_key: dnd2024:creatures/mastiff
splat_name: Mastiff
splat_origin:
  key: dnd2024:reference/creature/mastiff
splat_sources:
- title: SRD 5.2.1
  locator: pp. 357
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  skills_text: Perception +5
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: None
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 5 (1d8 + 1)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 3
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 5
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/mastiff
  implementation: dnd2024:creature_mastiff
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
          supplied_attack_total: 3
          reach_ft: 5
        hit:
          damage:
          - average: 4
            roll:
              count: 1
              sides: 6
              modifier: 1
            type: piercing
          prone:
            condition: prone
            maximum_size: medium
    skill_totals:
      perception: 5
    senses:
      darkvision: 60
      passive_perception: 15
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
- id: dnd2024:creature_mastiff
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
  - conditions
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 56ad9b8d5d6482ff73638fea2f1fa2205a882a2c51398a0fc803ad4ff74f3d6f
  source_path: builtin/srd-5.2.1/creature/mastiff.md
  pages:
  - 357
  engine_gaps: &id001
  - dnd2024:creature_mastiff
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 48e8a07258b225d7e512bf7c449df94ef6936939fcf5da88e1c6f2352d3ddb55
splat_clauses:
- key: source_rules
  text: '# Mastiff Medium Beast, Unaligned AC 12 Initiative +2 (12) HP 5 (1d8 + 1) Speed 40 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 13 +1 +1 Dex 14 +2 +2 Con 12 +1 +1 Int 3 -4 -4 Wis 12 +1 +3 Cha 7 -2 -2 Skills
    Perception +5 Senses Darkvision 60 ft.; Passive Perception 15 Languages None CR 1/8 (XP 25; PB +2)
    ### Actions Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1) Piercing damage. If the target
    is a Medium or smaller creature, it has the Prone condition.'
  sha256: 1e08dd03deddb4e81bf4fdb408c9e894ece87ae7de917e106806fbe3a67cbf11
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mastiff

<!-- SRD page 357 -->

Medium Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 5 (1d8 + 1)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 14 +2 +2 Con 12 +1 +1\
Int 3 −4 −4 Wis 12 +1 +3 Cha 7 −2 −2\
Skills Perception +5\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 1/8 (XP 25; PB +2)\

### Actions

Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1)\
Piercing damage. If the target is a Medium or smaller\
creature, it has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/357|page 357]].

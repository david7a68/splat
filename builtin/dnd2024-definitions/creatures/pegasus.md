---
splat_schema: 1
splat_id: 9e1ea0e7-23c3-5691-a57c-b9970b82c0bb
splat_kind: creature
splat_key: dnd2024:creatures/pegasus
splat_name: Pegasus
splat_origin:
  key: dnd2024:reference/creature/pegasus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 313
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: celestial
  alignment: Chaotic Good
  movement_text: 60 ft., Fly 90 ft.
  skills_text: Perception +6
  senses_text: Passive Perception 16
  languages_text: Understands Celestial, Common, Elvish, and Sylvan but can’t speak
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 59 (7d10 + 21)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 15
  save_dexterity:
    value: 4
  constitution:
    value: 16
  save_constitution:
    value: 5
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 15
  save_wisdom:
    value: 4
  charisma:
    value: 13
  save_charisma:
    value: 3
  armor_class:
    value: 12
  max_hp:
    value: 59
  initiative:
    value: 2
  speed:
    value: 60
    unit: ft
  fly_speed:
    value: 90
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/pegasus
  implementation: dnd2024:creature_pegasus
  parameters:
    features:
      hooves:
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
          - average: 7
            roll:
              count: 1
              sides: 6
              modifier: 4
            type: bludgeoning
          - average: 5
            roll:
              count: 2
              sides: 4
              modifier: 0
            type: radiant
    skill_totals:
      perception: 6
    senses:
      passive_perception: 16
    languages:
      speaks: []
      understands:
      - celestial
      - common
      - elvish
      - sylvan
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
- id: dnd2024:creature_pegasus
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
  source_sha256: 6bd48faf0a624bbc888fdcdfdf99fca419e2939c3c60df1c766c6b4a7419eade
  source_path: builtin/srd-5.2.1/creature/pegasus.md
  pages:
  - 313
  engine_gaps: &id001
  - dnd2024:creature_pegasus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 80d9c103bda6d58f5f2b2d586794887e8c639f959a8983a853531053c395a351
splat_clauses:
- key: source_rules
  text: '# Pegasus Large Celestial, Chaotic Good AC 12 Initiative +2 (12) HP 59 (7d10 + 21) Speed 60 ft.,
    Fly 90 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 15 +2 +4 Con 16 +3 +5 Int 10 +0 +0 Wis 15 +2
    +4 Cha 13 +1 +3 Skills Perception +6 Senses Passive Perception 16 Languages Understands Celestial,
    Common, Elvish, and Sylvan but can’t speak CR 2 (XP 450; PB +2) ### Actions Hooves. Melee Attack Roll:
    +6, reach 5 ft. Hit: 7 (1d6 + 4) Bludgeoning damage plus 5 (2d4) Radiant damage.'
  sha256: 3776822d53369b820f2fb2148e694305cf06e9e7bbf0fa38d9ef428ec25fefe6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pegasus

<!-- SRD page 313 -->

Large Celestial, Chaotic Good\
AC 12 Initiative +2 (12)\
HP 59 (7d10 + 21)\
Speed 60 ft., Fly 90 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +4 Con 16 +3 +5\
Int 10 +0 +0 Wis 15 +2 +4 Cha 13 +1 +3\
Skills Perception +6\
Senses Passive Perception 16\
Languages Understands Celestial, Common, Elvish, and\
Sylvan but can’t speak\
CR 2 (XP 450; PB +2)\

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 7 (1d6\
+ 4) Bludgeoning damage plus 5 (2d4) Radiant damage.\

Original layout: [[../../srd-5.2.1/pages/313|page 313]].

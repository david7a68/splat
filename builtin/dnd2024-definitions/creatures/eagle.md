---
splat_schema: 1
splat_id: 21e8a0b6-2688-5df4-947d-b78117ff85ed
splat_kind: creature
splat_key: dnd2024:creatures/eagle
splat_name: Eagle
splat_origin:
  key: dnd2024:reference/creature/eagle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 348
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: beast
  alignment: Unaligned
  movement_text: 10 ft., Fly 60 ft.
  skills_text: Perception +6
  senses_text: Passive Perception 16
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 4 (1d6 + 1)
splat_stats:
  strength:
    value: 6
  save_strength:
    value: -2
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 4
  initiative:
    value: 2
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/eagle
  implementation: dnd2024:creature_eagle
  parameters:
    features:
      talons:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          reach_ft: 5
        hit:
          damage:
          - average: 4
            roll:
              count: 1
              sides: 4
              modifier: 2
            type: slashing
    skill_totals:
      perception: 6
    senses:
      passive_perception: 16
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
- id: dnd2024:creature_eagle
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
  source_sha256: 9f3d76bd9c559bb9f16d68651e65d4f0bf55a246c5c83cc80824788b87c91be4
  source_path: builtin/srd-5.2.1/creature/eagle.md
  pages:
  - 348
  engine_gaps: &id001
  - dnd2024:creature_eagle
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 433746c9368f2db17f28ed1a67addd4479762b2e7effb8f10eaccb850a2a8b59
splat_clauses:
- key: source_rules
  text: '# Eagle Small Beast, Unaligned AC 12 Initiative +2 (12) HP 4 (1d6 + 1) Speed 10 ft., Fly 60 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 6 -2 -2 Dex 15 +2 +2 Con 12 +1 +1 Int 2 -4 -4 Wis 14 +2 +2 Cha 7 -2
    -2 Skills Perception +6 Senses Passive Perception 16 Languages None CR 0 (XP 10; PB +2) ### Actions
    Talons. Melee Attack Roll: +4, reach 5 feet. Hit: 4 (1d4 + 2) Slashing damage.'
  sha256: 43e022e5754c7eb2a73ef85316ff8c57e6d775b39b8c6da26e00a77d85156545
  status: contract_specified
splat_requires_mechanics: *id001
---

# Eagle

<!-- SRD page 348 -->

Small Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 4 (1d6 + 1)\
Speed 10 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 15 +2 +2 Con 12 +1 +1\
Int 2 −4 −4 Wis 14 +2 +2 Cha 7 −2 −2\
Skills Perception +6\
Senses Passive Perception 16\
Languages None\
CR 0 (XP 10; PB +2)\

### Actions

Talons. Melee Attack Roll: +4, reach 5 feet. Hit: 4 (1d4\
+ 2) Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/348|page 348]].

---
splat_schema: 1
splat_id: e5d54ebb-04c3-5399-8918-5d36b7621f4b
splat_kind: creature
splat_key: dnd2024:creatures/camel
splat_name: Camel
splat_origin:
  key: dnd2024:reference/creature/camel
splat_sources:
- title: SRD 5.2.1
  locator: pp. 346
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 50 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: None
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 17 (2d10 + 6)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 17
  save_constitution:
    value: 5
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 10
  max_hp:
    value: 17
  initiative:
    value: -1
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/camel
  implementation: dnd2024:creature_camel
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
          supplied_attack_total: 4
          reach_ft: 5
        hit:
          damage:
          - average: 4
            roll:
              count: 1
              sides: 4
              modifier: 2
            type: bludgeoning
    senses:
      darkvision: 60
      passive_perception: 10
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
- id: dnd2024:creature_camel
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
  source_sha256: 0f462cb9a869d48dc27d978ba7bc6a4647622cc519d68df52ca4f0954f579b2a
  source_path: builtin/srd-5.2.1/creature/camel.md
  pages:
  - 346
  engine_gaps: &id001
  - dnd2024:creature_camel
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/camel
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: a9d64d2992a963928d7de15f7c1c68a2d7786ff5fa8d29aebd0df83760a042c7
splat_clauses:
- key: source_rules
  text: '# Camel Large Beast, Unaligned AC 10 Initiative -1 (9) HP 17 (2d10 + 6) Speed 50 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 15 +2 +2 Dex 8 -1 -1 Con 17 +3 +5 Int 2 -4 -4 Wis 11 +0 +0 Cha 5 -3 -3 Senses
    Darkvision 60 ft.; Passive Perception 10 Languages None CR 1/8 (XP 25; PB +2) ### Actions Bite. Melee
    Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Bludgeoning damage.'
  sha256: 3ceecca83fadc8751cdac62068f774a0cbcab9b233d326d025a0dc7f70f57c74
  status: contract_specified
splat_requires_mechanics: *id001
---

# Camel

<!-- SRD page 346 -->

Large Beast, Unaligned\
AC 10   Initiative −1 (9)\
HP 17 (2d10 + 6)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 8 −1 −1 Con 17 +3 +5\
Int 2 −4 −4 Wis 11 +0 +0 Cha 5 −3 −3\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages None\
CR 1/8 (XP 25; PB +2)\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2)\
Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/346|page 346]].

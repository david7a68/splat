---
splat_schema: 1
splat_id: 6fad22d3-75bf-5c1d-95a8-e2e367dd4121
splat_kind: creature
splat_key: dnd2024:creatures/pony
splat_name: Pony
splat_origin:
  key: dnd2024:reference/creature/pony
splat_sources:
- title: SRD 5.2.1
  locator: pp. 359
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  senses_text: Passive Perception 10
  languages_text: None
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 11 (2d8 + 2)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 4
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 10
  max_hp:
    value: 11
  initiative:
    value: 0
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/pony
  implementation: dnd2024:creature_pony
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
- id: dnd2024:creature_pony
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
  source_sha256: 3e9cd32df4d7a45004af6a10c1a5b34b1851fcea5f44b95d0f5fd37d30311a5d
  source_path: builtin/srd-5.2.1/creature/pony.md
  pages:
  - 359
  engine_gaps: &id001
  - dnd2024:creature_pony
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/pony
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 360563bca59b6cc1a316895389fd5c81e7cab47451040310e0074412f4c44c26
splat_clauses:
- key: source_rules
  text: '# Pony Medium Beast, Unaligned AC 10 Initiative +0 (10) HP 11 (2d8 + 2) Speed 40 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 15 +2 +4 Dex 10 +0 +0 Con 13 +1 +1 Int 2 -4 -4 Wis 11 +0 +0 Cha 7 -2 -2 Senses
    Passive Perception 10 Languages None CR 1/8 (XP 25; PB +2) ### Actions Hooves. Melee Attack Roll:
    +4, reach 5 ft. Hit: 4 (1d4 + 2) Bludgeoning damage.'
  sha256: ef4ae432754b6bdd15e9febd4deb00cb59f5350a67f25a84092951346f9c38dc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pony

<!-- SRD page 359 -->

Medium Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 11 (2d8 + 2)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  15 +2 +4 Dex 10 +0 +0 Con 13 +1 +1\
Int 2 −4 −4 Wis 11 +0 +0 Cha 7 −2 −2\
Senses Passive Perception 10\
Languages None\
CR 1/8 (XP 25; PB +2)\

### Actions

Hooves. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4\
+ 2) Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/359|page 359]].

---
splat_schema: 1
splat_id: 87b77368-acf9-5128-b017-ac51db4b16e8
splat_kind: creature
splat_key: dnd2024:creatures/giant_wolf_spider
splat_name: Giant Wolf Spider
splat_origin:
  key: dnd2024:reference/creature/giant_wolf_spider
splat_sources:
- title: SRD 5.2.1
  locator: pp. 355
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 40 ft., Climb 40 ft.
  skills_text: Perception +3, Stealth +7
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 11 (2d8 + 2)
splat_stats:
  strength:
    value: 12
  save_strength:
    value: 1
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 13
  max_hp:
    value: 11
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_wolf_spider
  implementation: dnd2024:creature_giant_wolf_spider
  parameters:
    features:
      spider_climb:
        economy: trait
        operation: passive_rule
        climb_difficult_surfaces: true
        climb_ceilings: true
        ability_check_required: false
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
          - average: 5
            roll:
              count: 1
              sides: 4
              modifier: 3
            type: piercing
          - average: 5
            roll:
              count: 2
              sides: 4
              modifier: 0
            type: poison
    skill_totals:
      perception: 3
      stealth: 7
    senses:
      blindsight: 10
      darkvision: 60
      passive_perception: 13
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
- id: dnd2024:creature_giant_wolf_spider
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
  source_sha256: 439412db81b28fb658ff0db148f569af4d759354dcfb9c281a39a55ab7c531c1
  source_path: builtin/srd-5.2.1/creature/giant_wolf_spider.md
  pages:
  - 355
  engine_gaps: &id001
  - dnd2024:creature_giant_wolf_spider
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 62092cd6cfaf56ebf3c48705924438229eacef30a08fc89029acad5ec0521dcd
splat_clauses:
- key: source_rules
  text: '# Giant Wolf Spider Medium Beast, Unaligned AC 13 Initiative +3 (13) HP 11 (2d8 + 2) Speed 40
    ft., Climb 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 12 +1 +1 Dex 16 +3 +3 Con 13 +1 +1 Int 3 -4 -4 Wis
    12 +1 +1 Cha 4 -3 -3 Skills Perception +3, Stealth +7 Senses Blindsight 10 ft., Darkvision 60 ft.;
    Passive Perception 13 Languages None CR 1/4 (XP 50; PB +2) ### Traits Spider Climb. The spider can
    climb difficult surfaces, including along ceilings, without needing to make an ability check. ###
    Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Piercing damage plus 5 (2d4) Poison
    damage.'
  sha256: 3985cf7c707e60edb9471f3e25fedafdd1931d2441645d9f9b4a3e620a0afd95
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Wolf Spider

<!-- SRD page 355 -->

Medium Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 11 (2d8 + 2)\
Speed 40 ft., Climb 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 16 +3 +3 Con 13 +1 +1\
Int 3 −4 −4 Wis 12 +1 +1 Cha 4 −3 −3\
Skills Perception +3, Stealth +7\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 13\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Traits

Spider Climb. The spider can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3)\
Piercing damage plus 5 (2d4) Poison damage.\

Original layout: [[../../srd-5.2.1/pages/355|page 355]].

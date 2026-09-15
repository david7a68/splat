---
splat_schema: 1
splat_id: 883a37d4-0528-5c57-a94c-cf5f8e56eb9d
splat_kind: creature
splat_key: dnd2024:creatures/badger
splat_name: Badger
splat_origin:
  key: dnd2024:reference/creature/badger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 345
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 20 ft., Burrow 5 ft.
  skills_text: Perception +3
  resistances_text: Poison
  senses_text: Darkvision 30 ft.; Passive Perception 13
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 5 (1d4 + 3)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 11
  max_hp:
    value: 5
  initiative:
    value: 0
  speed:
    value: 20
    unit: ft
  burrow_speed:
    value: 5
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/badger
  implementation: dnd2024:creature_badger
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
          supplied_attack_total: 2
          reach_ft: 5
        hit:
          damage:
          - average: 1
            roll:
              constant: 1
            type: piercing
    skill_totals:
      perception: 3
    damage_resistances:
    - poison
    senses:
      darkvision: 30
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
- id: dnd2024:creature_badger
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
  source_sha256: 4b48e316675a1c0e4fe5f9fe3842396abd0b9354aaf042dfdbee53f3ab385c5a
  source_path: builtin/srd-5.2.1/creature/badger.md
  pages:
  - 345
  engine_gaps: &id001
  - dnd2024:creature_badger
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/badger
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 3a2a2248b8975d4e35e2d394a4c5375bb88a83842a45af0a204bf30e0d122795
splat_clauses:
- key: source_rules
  text: '# Badger Tiny Beast, Unaligned AC 11 Initiative +0 (10) HP 5 (1d4 + 3) Speed 20 ft., Burrow 5
    ft. MOD SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 11 +0 +0 Con 16 +3 +3 Int 2 -4 -4 Wis 12 +1 +1 Cha
    5 -3 -3 Skills Perception +3 Resistances Poison Senses Darkvision 30 ft.; Passive Perception 13 Languages
    None CR 0 (XP 10; PB +2) ### Actions Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage.'
  sha256: 9e3dd8d4cc4a16d0d4168a675ff5110e4b4867c039af7de3ccbfa03ffb912fa6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Badger

<!-- SRD page 345 -->

Tiny Beast, Unaligned\
AC 11  Initiative +0 (10)\
HP 5 (1d4 + 3)\
Speed 20 ft., Burrow 5 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 11 +0 +0 Con 16 +3 +3\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Perception +3\
Resistances Poison\
Senses Darkvision 30 ft.; Passive Perception 13\
Languages None\
CR 0 (XP 10; PB +2)\

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing\
damage.\

Original layout: [[../../srd-5.2.1/pages/345|page 345]].

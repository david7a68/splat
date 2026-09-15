---
splat_schema: 1
splat_id: 7cf98ec7-cf46-57a1-95ea-89be0fe74537
splat_kind: creature
splat_key: dnd2024:creatures/berserker
splat_name: Berserker
splat_origin:
  key: dnd2024:reference/creature/berserker
splat_sources:
- title: SRD 5.2.1
  locator: pp. 263
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  gear_text: Greataxe, Hide Armor
  senses_text: Passive Perception 10
  languages_text: Common
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 67 (9d8 + 27)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 9
  save_intelligence:
    value: -1
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 9
  save_charisma:
    value: -1
  armor_class:
    value: 13
  max_hp:
    value: 67
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/berserker
  implementation: dnd2024:creature_berserker
  parameters:
    features:
      bloodied_frenzy:
        economy: trait
        operation: passive_rule
        while:
          self_bloodied: true
        advantage_on:
        - attack_rolls
        - saving_throws
      greataxe:
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
          - average: 9
            roll:
              count: 1
              sides: 12
              modifier: 3
            type: slashing
    gear:
    - item: greataxe
      quantity: 1
    - item: hide_armor
      quantity: 1
    senses:
      passive_perception: 10
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
- id: dnd2024:creature_berserker
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
  - attack.resolved
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: d51571eac276f5570eb722ff42d0b75f9a1b52c95dd2932f7e4e4d234e96930f
  source_path: builtin/srd-5.2.1/creature/berserker.md
  pages:
  - 263
  engine_gaps: &id001
  - dnd2024:creature_berserker
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 14064144058ea1af86135557134ca3d2f67e8c908268cf14fcf3058500dc99b3
splat_clauses:
- key: source_rules
  text: '# Berserker Medium or Small Humanoid, Neutral AC 13 Initiative +1 (11) HP 67 (9d8 + 27) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 12 +1 +1 Con 17 +3 +3 Int 9 -1 -1 Wis 11 +0 +0
    Cha 9 -1 -1 Gear Greataxe, Hide Armor Senses Passive Perception 10 Languages Common CR 2 (XP 450;
    PB +2) ### Traits Bloodied Frenzy. While Bloodied, the berserker has Advantage on attack rolls and
    saving throws. ### Actions Greataxe. Melee Attack Roll: +5, reach 5 ft. Hit: 9 (1d12 + 3) Slashing
    damage.'
  sha256: 2080bb7afd088f93eef74c3bdcf0a248d118e58b1cde4100450e0f1ccff4a773
  status: contract_specified
splat_requires_mechanics: *id001
---

# Berserker

<!-- SRD page 263 -->

Medium or Small Humanoid, Neutral\
AC 13 Initiative +1 (11)\
HP 67 (9d8 + 27)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 12 +1 +1 Con 17 +3 +3\
Int 9 −1 −1 Wis 11 +0 +0 Cha 9 −1 −1\
Gear Greataxe, Hide Armor\
Senses Passive Perception 10\
Languages Common\
CR 2 (XP 450; PB +2)\

### Traits

Bloodied Frenzy. While Bloodied, the berserker has\
Advantage on attack rolls and saving throws.\

### Actions

Greataxe. Melee Attack Roll: +5, reach 5 ft. Hit: 9\
(1d12 + 3) Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/263|page 263]].

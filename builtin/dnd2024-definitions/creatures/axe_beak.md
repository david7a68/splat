---
splat_schema: 1
splat_id: 3fbef6af-bb6e-51d8-8bf2-9bd632abf451
splat_kind: creature
splat_key: dnd2024:creatures/axe_beak
splat_name: Axe Beak
splat_origin:
  key: dnd2024:reference/creature/axe_beak
splat_sources:
- title: SRD 5.2.1
  locator: pp. 260
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Unaligned
  movement_text: 50 ft.
  senses_text: Passive Perception 10
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 19 (3d10 + 3)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 11
  max_hp:
    value: 19
  initiative:
    value: 1
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/axe_beak
  implementation: dnd2024:creature_axe_beak
  parameters:
    features:
      beak:
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
          - average: 6
            roll:
              count: 1
              sides: 8
              modifier: 2
            type: slashing
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
- id: dnd2024:creature_axe_beak
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
  source_sha256: 06a259e37ed82ca87bf4b27ac601c16eb46e1728f489136054a939509c5ff711
  source_path: builtin/srd-5.2.1/creature/axe_beak.md
  pages:
  - 260
  engine_gaps: &id001
  - dnd2024:creature_axe_beak
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/axe_beak
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 0b3807c979f681eb544ba571b0b12c437ab0763ac767be7bb97273f54eb89cb7
splat_clauses:
- key: source_rules
  text: '# Axe Beak Large Monstrosity, Unaligned AC 11 Initiative +1 (11) HP 19 (3d10 + 3) Speed 50 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 12 +1 +1 Con 12 +1 +1 Int 2 -4 -4 Wis 10 +0 +0 Cha 5 -3
    -3 Senses Passive Perception 10 Languages None CR 1/4 (XP 50; PB +2) ### Actions Beak. Melee Attack
    Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage.'
  sha256: ffbe84f5b8f0c115af29e900c538501cb6b92fff906bc4bfb249f8dcba6f0c2d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Axe Beak

<!-- SRD page 260 -->

Large Monstrosity, Unaligned\
AC 11 Initiative +1 (11)\
HP 19 (3d10 + 3)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 12 +1 +1 Con 12 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
Senses Passive Perception 10\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Beak. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2)\
Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/260|page 260]].

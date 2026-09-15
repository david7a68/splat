---
splat_schema: 1
splat_id: 91576b90-7b8c-5204-a710-e0f7a3d88545
splat_kind: creature
splat_key: dnd2024:creatures/bat
splat_name: Bat
splat_origin:
  key: dnd2024:reference/creature/bat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 345
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 5 ft., Fly 30 ft.
  senses_text: Blindsight 60 ft.; Passive Perception 11
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 1 (1d4 - 1)
splat_stats:
  strength:
    value: 2
  save_strength:
    value: -4
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 8
  save_constitution:
    value: -1
  intelligence:
    value: 2
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
    value: 12
  max_hp:
    value: 1
  initiative:
    value: 2
  speed:
    value: 5
    unit: ft
  fly_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/bat
  implementation: dnd2024:creature_bat
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
          - average: 1
            roll:
              constant: 1
            type: piercing
    senses:
      blindsight: 60
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
- id: dnd2024:creature_bat
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
  source_sha256: 529aff6cda25189c674583b51c8c9b19081c16da5110263662ba6afe956268fb
  source_path: builtin/srd-5.2.1/creature/bat.md
  pages:
  - 345
  engine_gaps: &id001
  - dnd2024:creature_bat
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/bat
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: d18ef519572b1a35b7410b3f6c8331f044f619499b39d33fff5848230d0cefea
splat_clauses:
- key: source_rules
  text: '# Bat Tiny Beast, Unaligned AC 12 Initiative +2 (12) HP 1 (1d4 - 1) Speed 5 ft., Fly 30 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 2 -4 -4 Dex 15 +2 +2 Con 8 -1 -1 Int 2 -4 -4 Wis 12 +1 +1 Cha 4 -3 -3 Senses
    Blindsight 60 ft.; Passive Perception 11 Languages None CR 0 (XP 10; PB +2) ### Actions Bite. Melee
    Attack Roll: +4, reach 5 ft. Hit: 1 Piercing damage.'
  sha256: c9bb69f7a349d642b495ecb0a65da81df6d7fd09191cd23452635f3313a34fb7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bat

<!-- SRD page 345 -->

Tiny Beast, Unaligned\
AC 12 Initiative +2 (12)\
HP 1 (1d4 − 1)\
Speed 5 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 15 +2 +2 Con 8 −1 −1\
Int 2 −4 −4 Wis 12 +1 +1 Cha 4 −3 −3\
Senses Blindsight 60 ft.; Passive Perception 11\
Languages None\
CR 0 (XP 10; PB +2)\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing\
damage.\

Original layout: [[../../srd-5.2.1/pages/345|page 345]].

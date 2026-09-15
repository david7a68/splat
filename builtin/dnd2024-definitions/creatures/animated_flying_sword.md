---
splat_schema: 1
splat_id: 7be357ec-e71b-54e7-974a-3d8acad27110
splat_kind: creature
splat_key: dnd2024:creatures/animated_flying_sword
splat_name: Animated Flying Sword
splat_origin:
  key: dnd2024:reference/creature/animated_flying_sword
splat_sources:
- title: SRD 5.2.1
  locator: pp. 259
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: construct
  alignment: Unaligned
  movement_text: 5 ft., Fly 50 ft. (hover)
  immunities_text: Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned
  senses_text: Blindsight 60 ft.; Passive Perception 7
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 14 (4d6)
splat_stats:
  strength:
    value: 12
  save_strength:
    value: 1
  dexterity:
    value: 15
  save_dexterity:
    value: 4
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 5
  save_wisdom:
    value: -3
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 17
  max_hp:
    value: 14
  initiative:
    value: 4
  speed:
    value: 5
    unit: ft
  fly_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/animated_flying_sword
  implementation: dnd2024:creature_animated_flying_sword
  parameters:
    features:
      slash:
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
    damage_immunities:
    - poison
    - psychic
    condition_immunities:
    - charmed
    - deafened
    - exhaustion
    - frightened
    - paralyzed
    - petrified
    - poisoned
    senses:
      blindsight: 60
      passive_perception: 7
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
- id: dnd2024:creature_animated_flying_sword
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
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: bc43d56a59994fe9a17564ae85a001a4e8b33a1eb3a9fe0101d54b96c89f002b
  source_path: builtin/srd-5.2.1/creature/animated_flying_sword.md
  pages:
  - 259
  engine_gaps: &id001
  - dnd2024:creature_animated_flying_sword
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/animated_flying_sword
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 89b942efcd36d9df2e8781db4ffad738bb4f87912e0da503d762684de0680481
splat_clauses:
- key: source_rules
  text: '# Animated Flying Sword Small Construct, Unaligned AC 17 Initiative +4 (14) HP 14 (4d6) Speed
    5 ft., Fly 50 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 12 +1 +1 Dex 15 +2 +4 Con 11 +0 +0 Int 1
    -5 -5 Wis 5 -3 -3 Cha 1 -5 -5 Immunities Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened,
    Paralyzed, Petrified, Poisoned Senses Blindsight 60 ft.; Passive Perception 7 Languages None CR 1/4
    (XP 50; PB +2) ### Actions Slash. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage.'
  sha256: 27ddd5c71a3d041b7f0c8f8088f809ace67dfd4b972e24790e8326187c27d59e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Animated Flying Sword

<!-- SRD page 259 -->

Small Construct, Unaligned\
AC 17 Initiative +4 (14)\
HP 14 (4d6)\
Speed 5 ft., Fly 50 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 15 +2 +4 Con 11 +0 +0\
Int 1 −5 −5 Wis 5 −3 −3 Cha 1 −5 −5\
Immunities Poison, Psychic; Charmed, Deafened,\
Exhaustion, Frightened, Paralyzed, Petrified, Poisoned\
Senses Blindsight 60 ft.; Passive Perception 7\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Slash. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2)\
Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/259|page 259]].
